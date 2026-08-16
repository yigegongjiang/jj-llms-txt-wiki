## List custom pages

**get** `/accounts/{account_id}/access/custom_pages`

List custom pages

### Path Parameters

- `account_id: string`

  Identifier.

### Query Parameters

- `page: optional number`

  Page number of results.

- `per_page: optional number`

  Number of results per page.

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

- `result: optional array of CustomPageWithoutHTML`

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

- `result_info: optional object { count, page, per_page, 2 more }`

  - `count: optional number`

    Total number of results for the requested service.

  - `page: optional number`

    Current page within paginated list of results.

  - `per_page: optional number`

    Number of results per page of results.

  - `total_count: optional number`

    Total results available without any search parameters.

  - `total_pages: optional number`

    The number of total pages in the entire result set.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/custom_pages \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
  "result": [
    {
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
  ],
  "result_info": {
    "count": 1,
    "page": 1,
    "per_page": 20,
    "total_count": 2000,
    "total_pages": 100
  }
}
```
