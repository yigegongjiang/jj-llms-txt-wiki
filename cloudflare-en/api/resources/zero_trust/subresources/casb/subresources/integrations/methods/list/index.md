## List integrations

**get** `/accounts/{account_id}/one/integrations`

Returns a paginated list of integrations for the account.

### Path Parameters

- `account_id: string`

### Query Parameters

- `application: optional string`

  Filter by application/vendor (e.g., GOOGLE_WORKSPACE, MICROSOFT_INTERNAL).

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `dlp_enabled: optional boolean`

  Filter by DLP enabled status (true/false).

- `order: optional "application" or "created" or "name" or "status"`

  Field to order results by.

  - `"application"`

  - `"created"`

  - `"name"`

  - `"status"`

- `page: optional number`

  Page number within the paginated result set.

- `page_size: optional number`

  Number of results per page.

- `search: optional string`

  Search integrations by name or application.

- `status: optional "Healthy" or "Initializing" or "Offline" or "Unhealthy"`

  Filter by integration status.

  - `"Healthy"`

  - `"Initializing"`

  - `"Offline"`

  - `"Unhealthy"`

- `use_cases: optional string`

  Filter by enabled use cases (e.g., casb, ces). Matches integrations enrolled in any of the specified values. Can be specified multiple times.

### Returns

- `errors: array of unknown`

  List of errors.

- `messages: array of string`

  List of messages.

- `result: array of object { id, application, created, 4 more }`

  List of items.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `created: string`

    When the integration was created.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

- `result_info: object { count, next, page, 3 more }`

  Pagination metadata.

  - `count: optional number`

    Number of items in current page.

  - `next: optional string`

    URL for next page.

  - `page: optional number`

    Current page number.

  - `per_page: optional number`

    Number of items per page.

  - `previous: optional string`

    URL for previous page.

  - `total_count: optional number`

    Total number of items.

- `success: boolean`

  Whether the request succeeded.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": [
    {
      "application": {
        "category": "Productivity",
        "display_name": "Google Workspace",
        "logo": "https://dash.cloudflare.com/v2/static/google_workspace.png"
      },
      "created": "2025-01-15T10:00:00Z",
      "id": "019d2e6a-d995-7185-afbd-4feead9e42ec",
      "is_paused": false,
      "name": "My Google Workspace",
      "status": "Healthy",
      "updated": "2025-04-10T08:30:00Z"
    }
  ],
  "result_info": {
    "count": 1,
    "next": null,
    "page": 1,
    "per_page": 10,
    "previous": null,
    "total_count": 1
  },
  "success": true
}
```
