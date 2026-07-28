use reqwest::StatusCode;
use reqwest::header::{
    ETAG, HeaderMap, HeaderName, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED, LOCATION,
    RETRY_AFTER,
};
use reqwest::redirect::Policy;
use std::str;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

use crate::manifest::Validator;
use crate::url_map::{AllowedOrigins, CanonicalUrl};

/// Waits before each retry; its length is the retry count, so a URL costs at
/// most `RETRY_BACKOFF.len() + 1` requests. Kept short and few: a docs host that
/// still errors after ~2 s of spacing is down or broken, not busy, and a long
/// ladder multiplied by every failing page would stall the whole run.
const RETRY_BACKOFF: [Duration; 2] = [Duration::from_millis(500), Duration::from_millis(1500)];

/// Requests a single URL can cost at most: the first attempt plus every retry.
pub const MAX_ATTEMPTS: usize = RETRY_BACKOFF.len() + 1;

/// Ceiling on an honoured `Retry-After`. A server asking for longer wants more
/// than one sync run should hold a download slot idle, so that request is
/// ignored and the local backoff applies instead.
const MAX_RETRY_AFTER: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum FetchOutcome {
    Document {
        final_url: Url,
        body: String,
        validator: Validator,
    },
    NotModified {
        final_url: Url,
    },
    Missing,
    IgnoredRedirect,
    /// The body exceeds the caller's `max_bytes` cap and was dropped without
    /// being kept. An oversized single Markdown file is almost never real
    /// documentation, so it is excluded rather than mirrored.
    Oversize {
        final_url: Url,
    },
}

/// A failed attempt. `retry_after` carries the server's own pacing request when
/// it sent one; `retryable` marks the failures worth another attempt — transport
/// errors, `429`, and `5xx`. A definitive answer (a `4xx` that is not `429`, a
/// malformed redirect, a non-UTF-8 body) is not retried: repeating it only
/// wastes the slot.
struct FetchError {
    message: String,
    retryable: bool,
    retry_after: Option<Duration>,
}

impl FetchError {
    fn permanent(message: String) -> Self {
        Self {
            message,
            retryable: false,
            retry_after: None,
        }
    }

    fn transient(message: String, retry_after: Option<Duration>) -> Self {
        Self {
            message,
            retryable: true,
            retry_after,
        }
    }

    /// How long to wait before attempt `attempt + 1`, or `None` when this error
    /// must not be retried or the attempts are spent.
    fn delay_before_retry(&self, attempt: usize) -> Option<Duration> {
        if !self.retryable {
            return None;
        }
        let backoff = *RETRY_BACKOFF.get(attempt)?;
        Some(
            self.retry_after
                .filter(|requested| *requested <= MAX_RETRY_AFTER)
                .unwrap_or(backoff),
        )
    }
}

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    allowed: AllowedOrigins,
}

impl HttpClient {
    pub fn new(allowed: &AllowedOrigins, timeout: Duration) -> Result<Self, String> {
        let redirect_allowed = allowed.clone();
        let redirect = Policy::custom(move |attempt| {
            if attempt.previous().len() >= 10 {
                attempt.error("too many redirects")
            } else if redirect_allowed.contains(attempt.url()) {
                attempt.follow()
            } else {
                attempt.stop()
            }
        });
        let client = reqwest::Client::builder()
            .user_agent(concat!("jj-llms-txt-wiki/", env!("CARGO_PKG_VERSION")))
            .timeout(timeout)
            .redirect(redirect)
            .build()
            .map_err(|error| format!("build HTTP client: {error}"))?;
        Ok(Self {
            client,
            allowed: allowed.clone(),
        })
    }

    /// `max_bytes` caps the accepted body size; `None` disables the cap (used for
    /// entry documents and `llms-full.txt`, which are legitimately large). A body
    /// past the cap yields [`FetchOutcome::Oversize`] instead of `Document`.
    ///
    /// Retries transient failures (see [`FetchError`]) on the caller's slot, so a
    /// blip on one page never costs the crawl its snapshot. The returned `Err` is
    /// the last attempt's message — by then the URL has been given every attempt
    /// this client makes, which is what lets the crawler treat it as a settled
    /// verdict rather than a maybe.
    pub async fn fetch(
        &self,
        url: &CanonicalUrl,
        validator: Option<&Validator>,
        max_bytes: Option<usize>,
    ) -> Result<FetchOutcome, String> {
        let mut attempt = 0;
        loop {
            match self.try_fetch(url, validator, max_bytes).await {
                Ok(outcome) => return Ok(outcome),
                Err(error) => {
                    let Some(delay) = error.delay_before_retry(attempt) else {
                        return Err(error.message);
                    };
                    sleep(delay).await;
                    attempt += 1;
                }
            }
        }
    }

    /// One attempt. Classifies the outcome; [`Self::fetch`] owns the retry loop.
    async fn try_fetch(
        &self,
        url: &CanonicalUrl,
        validator: Option<&Validator>,
        max_bytes: Option<usize>,
    ) -> Result<FetchOutcome, FetchError> {
        let mut request = self.client.get(url.as_url().clone());
        if let Some(validator) = validator {
            if let Some(etag) = &validator.etag {
                request = request.header(IF_NONE_MATCH, etag);
            }
            if let Some(last_modified) = &validator.last_modified {
                request = request.header(IF_MODIFIED_SINCE, last_modified);
            }
        }
        // A transport error (DNS, connect, TLS, timeout, reset mid-stream) says
        // nothing about the resource, only about this attempt — always worth
        // retrying.
        let response = request
            .send()
            .await
            .map_err(|error| FetchError::transient(format!("GET {url}: {error}"), None))?;
        let status = response.status();

        if status == StatusCode::NOT_MODIFIED {
            return Ok(FetchOutcome::NotModified {
                final_url: response.url().clone(),
            });
        }

        if status.is_success() {
            let final_url = response.url().clone();
            // Reject before buffering when the server advertises a Content-Length
            // past the cap, so a giant document never lands in memory. No
            // compression feature is enabled, so Content-Length is the exact body
            // size; the post-buffer check below still catches chunked responses
            // that omit the header.
            if let Some(max) = max_bytes
                && response
                    .content_length()
                    .is_some_and(|length| length > max as u64)
            {
                return Ok(FetchOutcome::Oversize { final_url });
            }
            let validator = extract_validator(response.headers());
            let bytes = response
                .bytes()
                .await
                .map_err(|error| FetchError::transient(format!("read {url}: {error}"), None))?;
            if let Some(max) = max_bytes
                && bytes.len() > max
            {
                return Ok(FetchOutcome::Oversize { final_url });
            }
            let body = str::from_utf8(&bytes)
                .map_err(|error| {
                    FetchError::permanent(format!("response is not UTF-8 for {url}: {error}"))
                })?
                .to_owned();
            return Ok(FetchOutcome::Document {
                final_url,
                body,
                validator,
            });
        }

        if matches!(status.as_u16(), 404 | 410) {
            return Ok(FetchOutcome::Missing);
        }

        if status.is_redirection() {
            let location = response
                .headers()
                .get(LOCATION)
                .ok_or_else(|| {
                    FetchError::permanent(format!("GET {url}: {status} without Location"))
                })?
                .to_str()
                .map_err(|error| {
                    FetchError::permanent(format!("GET {url}: invalid Location: {error}"))
                })?;
            let target = response.url().join(location).map_err(|error| {
                FetchError::permanent(format!("GET {url}: invalid redirect target: {error}"))
            })?;
            if !self.allowed.contains(&target) {
                return Ok(FetchOutcome::IgnoredRedirect);
            }
        }

        let message = format!("GET {url}: HTTP {status}");
        // `429` and `5xx` are the server saying "not now"; everything else here
        // is a settled answer that a repeat request would only repeat.
        if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
            return Err(FetchError::transient(
                message,
                retry_after(response.headers()),
            ));
        }
        Err(FetchError::permanent(message))
    }
}

/// Parse `Retry-After` as delta-seconds. The HTTP-date form is accepted by the
/// spec but effectively unused by docs CDNs, and guessing at clock skew to honour
/// it would be worse than falling back to the local backoff.
fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    let seconds: u64 = headers
        .get(RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim()
        .parse()
        .ok()?;
    Some(Duration::from_secs(seconds))
}

fn extract_validator(headers: &HeaderMap) -> Validator {
    Validator {
        etag: header_value(headers, ETAG),
        last_modified: header_value(headers, LAST_MODIFIED),
    }
}

fn header_value(headers: &HeaderMap, name: HeaderName) -> Option<String> {
    headers
        .get(name)?
        .to_str()
        .ok()
        .map(|value| value.to_owned())
}
