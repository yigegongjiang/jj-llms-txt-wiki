## Create blocked email sender

**post** `/accounts/{account_id}/email-security/settings/block_senders`

Creates a new blocked sender pattern. Emails matching this pattern will be blocked from delivery. Patterns can be email addresses, domains, or IP addresses, and support regular expressions.

### Path Parameters

- `account_id: string`

  Identifier.

### Body Parameters

- `is_regex: boolean`

- `pattern: string`

  The pattern value to match. The format depends on `pattern_type`: a valid email address for EMAIL (e.g. `user@example.com`), a valid domain name for DOMAIN (e.g. `example.com`), or a plain IPv4 address or IPv4 CIDR block for IP (e.g. `1.2.3.4` or `1.2.3.0/24`); the API accepts only globally reachable IP addresses and rejects private, loopback, link-local, and unspecified addresses.

- `pattern_type: "EMAIL" or "DOMAIN" or "IP" or "UNKNOWN"`

  Type of pattern matching.

  - EMAIL: matches a full email address (e.g. `user@example.com`)
  - DOMAIN: matches a domain name (e.g. `example.com`)
  - IP: matches a plain IPv4 address (e.g. `1.2.3.4`) or an IPv4 CIDR block (e.g. `1.2.3.0/24`). The API accepts only globally reachable addresses.
  - UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

  - `"EMAIL"`

  - `"DOMAIN"`

  - `"IP"`

  - `"UNKNOWN"`

- `comments: optional string`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { id, comments, created_at, 5 more }`

  A blocked sender pattern.

  - `id: optional string`

    Blocked sender pattern identifier.

  - `comments: optional string`

  - `created_at: optional string`

  - `is_regex: optional boolean`

  - `last_modified: optional string`

    Deprecated, use `modified_at` instead. End of life: November 1, 2026.

  - `modified_at: optional string`

  - `pattern: optional string`

    The pattern value to match. The format depends on `pattern_type`: a valid email address for EMAIL (e.g. `user@example.com`), a valid domain name for DOMAIN (e.g. `example.com`), or a plain IPv4 address or IPv4 CIDR block for IP (e.g. `1.2.3.4` or `1.2.3.0/24`); the API accepts only globally reachable IP addresses and rejects private, loopback, link-local, and unspecified addresses.

  - `pattern_type: optional "EMAIL" or "DOMAIN" or "IP" or "UNKNOWN"`

    Type of pattern matching.

    - EMAIL: matches a full email address (e.g. `user@example.com`)
    - DOMAIN: matches a domain name (e.g. `example.com`)
    - IP: matches a plain IPv4 address (e.g. `1.2.3.4`) or an IPv4 CIDR block (e.g. `1.2.3.0/24`). The API accepts only globally reachable addresses.
    - UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

    - `"EMAIL"`

    - `"DOMAIN"`

    - `"IP"`

    - `"UNKNOWN"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/email-security/settings/block_senders \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "is_regex": false,
          "pattern": "test@example.com",
          "pattern_type": "EMAIL",
          "comments": "Block sender with email test@example.com"
        }'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": {
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "comments": "Block sender with email test@example.com",
    "created_at": "2014-01-01T05:20:00.12345Z",
    "is_regex": false,
    "last_modified": "2014-01-01T05:20:00.12345Z",
    "modified_at": "2014-01-01T05:20:00.12345Z",
    "pattern": "test@example.com",
    "pattern_type": "EMAIL"
  }
}
```
