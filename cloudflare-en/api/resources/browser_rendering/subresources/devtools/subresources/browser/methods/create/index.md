## Get a browser session ID.

**post** `/accounts/{account_id}/browser-rendering/devtools/browser`

Acquires a browser and returns its session ID and websocket URL. Optionally accepts a JSON body with session guardrails to restrict outbound HTTP/S traffic.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `keep_alive: optional number`

  Keep-alive time in milliseconds.

- `lab: optional boolean`

  Use experimental browser.

- `liveViewUrlExpiresInMs: optional number`

  How long the live view URL remains valid, in milliseconds (max 60 minutes). Only used when targets is true.

- `recording: optional boolean`

- `targets: optional boolean`

  Include browser targets in response.

### Body Parameters

- `guardrails: optional object { allowedDomains, allowedDomainSets }`

  - `allowedDomains: optional array of string`

    Hostname patterns, max 50. Supports exact hosts (example.com) or a single * wildcard anywhere. Prefer *.example.com (subdomain wildcard) over *example.com (prefix wildcard) to avoid matching overbroad lookalikes like evilexample.com.

  - `allowedDomainSets: optional array of string`

    Max 4 entries: curated preset names (common-cdns) and/or https URLs of newline-separated hostname lists.

### Returns

- `sessionId: string`

  Browser session ID.

- `webSocketDebuggerUrl: optional string`

  WebSocket URL for the session.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/browser-rendering/devtools/browser \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "sessionId": "sessionId",
  "webSocketDebuggerUrl": "webSocketDebuggerUrl"
}
```
