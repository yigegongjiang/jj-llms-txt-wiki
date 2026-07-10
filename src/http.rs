use reqwest::StatusCode;
use reqwest::header::{
    ETAG, HeaderMap, HeaderName, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED, LOCATION,
};
use reqwest::redirect::Policy;
use std::str;
use std::time::Duration;
use url::Url;

use crate::manifest::Validator;
use crate::url_map::{CanonicalUrl, same_origin};

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
}

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    entry: Url,
}

impl HttpClient {
    pub fn new(entry: &Url, timeout: Duration) -> Result<Self, String> {
        let redirect_origin = entry.clone();
        let redirect = Policy::custom(move |attempt| {
            if attempt.previous().len() >= 10 {
                attempt.error("too many redirects")
            } else if same_origin(&redirect_origin, attempt.url()) {
                attempt.follow()
            } else {
                attempt.stop()
            }
        });
        let client = reqwest::Client::builder()
            .user_agent(concat!("llms-wiki/", env!("CARGO_PKG_VERSION")))
            .timeout(timeout)
            .redirect(redirect)
            .build()
            .map_err(|error| format!("build HTTP client: {error}"))?;
        Ok(Self {
            client,
            entry: entry.clone(),
        })
    }

    pub async fn fetch(
        &self,
        url: &CanonicalUrl,
        validator: Option<&Validator>,
    ) -> Result<FetchOutcome, String> {
        let mut request = self.client.get(url.as_url().clone());
        if let Some(validator) = validator {
            if let Some(etag) = &validator.etag {
                request = request.header(IF_NONE_MATCH, etag);
            }
            if let Some(last_modified) = &validator.last_modified {
                request = request.header(IF_MODIFIED_SINCE, last_modified);
            }
        }
        let response = request
            .send()
            .await
            .map_err(|error| format!("GET {url}: {error}"))?;
        let status = response.status();

        if status == StatusCode::NOT_MODIFIED {
            return Ok(FetchOutcome::NotModified {
                final_url: response.url().clone(),
            });
        }

        if status.is_success() {
            let final_url = response.url().clone();
            let validator = extract_validator(response.headers());
            let bytes = response
                .bytes()
                .await
                .map_err(|error| format!("read {url}: {error}"))?;
            let body = str::from_utf8(&bytes)
                .map_err(|error| format!("response is not UTF-8 for {url}: {error}"))?
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
                .ok_or_else(|| format!("GET {url}: {status} without Location"))?
                .to_str()
                .map_err(|error| format!("GET {url}: invalid Location: {error}"))?;
            let target = response
                .url()
                .join(location)
                .map_err(|error| format!("GET {url}: invalid redirect target: {error}"))?;
            if !same_origin(&self.entry, &target) {
                return Ok(FetchOutcome::IgnoredRedirect);
            }
        }

        Err(format!("GET {url}: HTTP {status}"))
    }
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
