## Get extension

**get** `/accounts/{account_id}/registrar-sandbox/extensions/{extension}`

Returns metadata and JSON Schema documents describing the expected input
structure for registration operations on a specific
extension (TLD).

Supports HTTP conditional GET via `ETag`. Include the `ETag` value
from a previous response in an `If-None-Match` header to receive a
`304 Not Modified` when the data has not changed.

### Path Parameters

- `account_id: string`

  Identifier

- `extension: string`

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

- `result: object { metadata, registration_schema }`

  Extension entry with metadata and JSON Schema documents for the registration operation.

  - `metadata: object { name, tld }`

    Extension metadata

    - `name: string`

      The full name of the extension. For example, "co.uk", or "uk"

    - `tld: string`

      The tld of the extension. For example, for "co.uk", it's "uk". For "uk", it's "uk"

  - `registration_schema: unknown`

    JSON Schema describing the expected input structure for registration operations on this extension.

- `success: true`

  Whether the API call was successful

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/registrar-sandbox/extensions/$EXTENSION \
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
  "result": {
    "metadata": {
      "name": "name",
      "tld": "tld"
    },
    "registration_schema": {}
  },
  "success": true
}
```
