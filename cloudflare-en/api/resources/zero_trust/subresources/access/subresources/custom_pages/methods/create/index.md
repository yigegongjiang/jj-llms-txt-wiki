## Create a custom page

**post** `/accounts/{account_id}/access/custom_pages`

Create a custom page

### Path Parameters

- `account_id: string`

  Identifier.

### Body Parameters

- `custom_html: string`

  Custom page HTML.

- `name: string`

  Custom page name.

- `type: "identity_denied" or "forbidden" or "login" or "interstitial"`

  Custom page type.

  - `"identity_denied"`

  - `"forbidden"`

  - `"login"`

  - `"interstitial"`

- `contract_version: optional number`

  Contract version of the page's Liquid template. Present (>= 1) marks a sanitized template; absent or 0 marks a legacy page served verbatim.

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

- `result: optional CustomPageWithoutHTML`

  - `name: string`

    Custom page name.

  - `type: "identity_denied" or "forbidden" or "login" or "interstitial"`

    Custom page type.

    - `"identity_denied"`

    - `"forbidden"`

    - `"login"`

    - `"interstitial"`

  - `contract_version: optional number`

    Contract version of the page's Liquid template. Present (>= 1) marks a sanitized template; absent or 0 marks a legacy page served verbatim.

  - `uid: optional string`

    UUID.

  - `warnings: optional array of object { message, tier, ref }`

    Advisory validation findings returned when creating or updating a template. Omitted when empty.

    - `message: string`

      Human-readable description of the finding.

    - `tier: string`

      The validation tier that produced the finding (e.g. html, liquid).

    - `ref: optional string`

      Optional pointer to the part of the template the finding refers to.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/custom_pages \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "custom_html": "<html><body><h1>Access Denied</h1></body></html>",
          "name": "name",
          "type": "identity_denied"
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
    "name": "name",
    "type": "identity_denied",
    "app_count": 0,
    "contract_version": 0,
    "created_at": "2014-01-01T05:20:00.12345Z",
    "uid": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "updated_at": "2014-01-01T05:20:00.12345Z",
    "warnings": [
      {
        "message": "message",
        "tier": "tier",
        "ref": "ref"
      }
    ]
  }
}
```
