## Acquire and connect to browser session.

**get** `/accounts/{account_id}/browser-rendering/devtools/browser`

Acquires and establishes a WebSocket connection to a browser session. Session guardrails may be supplied in the `cf-brapi-guardrails` header as base64url-encoded JSON of the same `guardrails` object the POST body accepts (for example `{"allowedDomains":["*.example.com"]}`).

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `keep_alive: optional number`

  Keep-alive time in ms (only valid when acquiring new session).

- `lab: optional boolean`

  Use experimental browser.

- `recording: optional boolean`

### Header Parameters

- `"cf-brapi-guardrails": optional string`

  Optional base64url-encoded JSON session guardrails (allowedDomains and allowedDomainSets)

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/browser-rendering/devtools/browser \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```
