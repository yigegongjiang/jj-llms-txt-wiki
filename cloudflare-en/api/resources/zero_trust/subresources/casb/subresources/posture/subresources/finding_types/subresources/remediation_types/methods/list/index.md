## List remediation types for a finding type

**get** `/accounts/{account_id}/data-security/posture/finding_types/{finding_type_id}/remediation_types`

List all remediation types for a given finding type.
This endpoint supports both cursor and offset pagination.
Note that `cursor` and `page` are mutually exclusive.

### Path Parameters

- `account_id: string`

- `finding_type_id: string`

### Query Parameters

- `cursor: optional string`

  A cursor for pagination.

- `integration_id: optional string`

  Filter by an integration ID

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

    Error or message code.

  - `message: string`

    Human-readable message.

  - `documentation_url: optional string`

    Link to relevant documentation.

  - `source: optional object { pointer }`

    - `pointer: optional string`

      JSON pointer to the source of the error.

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

    Error or message code.

  - `message: string`

    Human-readable message.

  - `documentation_url: optional string`

    Link to relevant documentation.

  - `source: optional object { pointer }`

    - `pointer: optional string`

      JSON pointer to the source of the error.

- `result_info: object { count, cursor, next, 4 more }`

  Pagination and result information.

  - `count: optional number`

    Total number of results for the requested service.

  - `cursor: optional string`

    Cursor for cursor-based pagination.

  - `next: optional string`

    URL to the next page of results.

  - `page: optional number`

    Current page within paginated list of results.

  - `per_page: optional number`

    Number of results per page of results.

  - `previous: optional string`

    URL to the previous page of results.

  - `total_count: optional number`

    Total results available without any search parameters.

- `success: boolean`

  Whether the API call was successful.

- `result: optional array of object { id, description, display_name, 2 more }`

  Array of remediation type objects.

  - `id: string`

    The identifier for the remediation type.

  - `description: string`

    A description of the action(s) taken by the remediation type.

  - `display_name: string`

    The name of the remediation type as displayed in the cloudflare dashboard.

  - `finding_type_id: string`

    The identifier of the finding_type which this remediation type should remediate.

  - `remediation_type: string`

    The name of the remediation type.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/finding_types/$FINDING_TYPE_ID/remediation_types \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "Request processed successfully",
      "documentation_url": "https://developers.cloudflare.com/api/operations/list-findings",
      "source": {
        "pointer": "/data/attributes/name"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "Request processed successfully",
      "documentation_url": "https://developers.cloudflare.com/api/operations/list-findings",
      "source": {
        "pointer": "/data/attributes/name"
      }
    }
  ],
  "result_info": {
    "count": 1,
    "cursor": "eyJpZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCIsImFmZmxpY3Rpb25fZGF0ZSI6IjE5NzAtMDEtMDFUMDA6MDA6MDAuMDAwMDAwWiJ9",
    "next": "https://example.com",
    "page": 1,
    "per_page": 20,
    "previous": "https://example.com",
    "total_count": 2000
  },
  "success": true,
  "result": [
    {
      "id": "7d736ac5-ed3b-46d5-9375-7025175ba1d9",
      "description": "Remove publicly accessible URL granting edit access",
      "display_name": "Remove Publicly Accessible URL - Edit Access",
      "finding_type_id": "6a790513-bbb5-4933-8971-76a744ec5448",
      "remediation_type": "Microsoft: Remove Publicly Accessible URL - Edit Access"
    }
  ]
}
```
