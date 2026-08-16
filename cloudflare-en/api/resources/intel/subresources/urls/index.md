# URLs

## Get URL Intelligence

**get** `/accounts/{account_id}/intel/url`

Gets security information about a URL, including content categories and risk types. The URL must be provided as a query parameter.

### Path Parameters

- `account_id: string`

  Identifier.

### Query Parameters

- `url: string`

  The URL to look up.

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

- `result: optional URL`

  - `content_categories: array of object { id, name, source_id, super_category_id }`

    Content categories associated with this URL.

    - `id: optional number`

    - `name: optional string`

    - `source_id: optional number`

    - `super_category_id: optional number`

  - `full_url: string`

    The full URL that was looked up.

  - `hostname: string`

    The hostname of the URL.

  - `risk_type: array of object { id, name, source_id, super_category_id }`

    Security risk types associated with this URL.

    - `id: optional number`

    - `name: optional string`

    - `source_id: optional number`

    - `super_category_id: optional number`

  - `url_path: string`

    The path component of the URL.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/url \
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
  "result": {
    "content_categories": [
      {
        "id": 155,
        "name": "Technology",
        "source_id": 1,
        "super_category_id": 26
      }
    ],
    "full_url": "https://example.com/path",
    "hostname": "example.com",
    "risk_type": [
      {
        "id": 0,
        "name": "name",
        "source_id": 0,
        "super_category_id": 0
      }
    ],
    "url_path": "/path"
  }
}
```

## Domain Types

### URL

- `URL object { content_categories, full_url, hostname, 2 more }`

  - `content_categories: array of object { id, name, source_id, super_category_id }`

    Content categories associated with this URL.

    - `id: optional number`

    - `name: optional string`

    - `source_id: optional number`

    - `super_category_id: optional number`

  - `full_url: string`

    The full URL that was looked up.

  - `hostname: string`

    The hostname of the URL.

  - `risk_type: array of object { id, name, source_id, super_category_id }`

    Security risk types associated with this URL.

    - `id: optional number`

    - `name: optional string`

    - `source_id: optional number`

    - `super_category_id: optional number`

  - `url_path: string`

    The path component of the URL.
