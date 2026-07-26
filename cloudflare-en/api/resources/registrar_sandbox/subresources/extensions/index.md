# Extensions

## List extensions

**get** `/accounts/{account_id}/registrar-sandbox/extensions`

Returns metadata and JSON Schema documents describing the expected input
structure for registration operations on each supported
extension (TLD).

This endpoint uses cursor-based pagination. Results are ordered by
extension name by default. To fetch the next page, pass the `cursor`
value from the `result_info` object in the response as the `cursor`
query parameter in your next request. An empty `cursor` string
indicates there are no more pages.

Supports HTTP conditional GET via `ETag`. Include the `ETag` value
from a previous response in an `If-None-Match` header to receive a
`304 Not Modified` when the data has not changed.

### Path Parameters

- `account_id: string`

  Identifier

### Query Parameters

- `cursor: optional string`

  Opaque token from a previous response's `result_info.cursor`.
  Pass this value to fetch the next page of results. Omit (or
  pass an empty string) for the first page.

- `direction: optional "asc" or "desc"`

  Sort direction for results. Defaults to ascending order.

  - `"asc"`

  - `"desc"`

- `name: optional string`

  Filter extensions by exact name match.
  For example, `name=com` returns only the `com` extension.

- `per_page: optional number`

  Number of items to return per page.

- `sort_by: optional "name" or "created_at" or "updated_at"`

  Column to sort results by. Defaults to `name` when omitted.

  - `"name"`

  - `"created_at"`

  - `"updated_at"`

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

- `result: array of object { metadata, registration_schema }`

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

- `result_info: optional object { count, cursor, per_page }`

  Cursor-based pagination metadata.

  - `count: optional number`

    Number of items in the current result set.

  - `cursor: optional string`

    Opaque token for the next page. Empty string when no more pages.

  - `per_page: optional number`

    Number of items per page.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/registrar-sandbox/extensions \
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
  "result": [
    {
      "metadata": {
        "name": "name",
        "tld": "tld"
      },
      "registration_schema": {}
    }
  ],
  "success": true,
  "result_info": {
    "count": 0,
    "cursor": "cursor",
    "per_page": 0
  }
}
```

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

## Domain Types

### Extension List Response

- `ExtensionListResponse object { metadata, registration_schema }`

  Extension entry with metadata and JSON Schema documents for the registration operation.

  - `metadata: object { name, tld }`

    Extension metadata

    - `name: string`

      The full name of the extension. For example, "co.uk", or "uk"

    - `tld: string`

      The tld of the extension. For example, for "co.uk", it's "uk". For "uk", it's "uk"

  - `registration_schema: unknown`

    JSON Schema describing the expected input structure for registration operations on this extension.

### Extension Get Response

- `ExtensionGetResponse object { metadata, registration_schema }`

  Extension entry with metadata and JSON Schema documents for the registration operation.

  - `metadata: object { name, tld }`

    Extension metadata

    - `name: string`

      The full name of the extension. For example, "co.uk", or "uk"

    - `tld: string`

      The tld of the extension. For example, for "co.uk", it's "uk". For "uk", it's "uk"

  - `registration_schema: unknown`

    JSON Schema describing the expected input structure for registration operations on this extension.
