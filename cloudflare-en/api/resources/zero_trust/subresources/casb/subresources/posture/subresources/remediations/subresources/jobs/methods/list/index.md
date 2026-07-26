## List remediation jobs

**get** `/accounts/{account_id}/data-security/posture/remediations/jobs`

List all remediation jobs tied to a specific Cloudflare Account. Note that `cursor` and `page` are mutually exclusive.

### Path Parameters

- `account_id: string`

### Query Parameters

- `cursor: optional string`

  A cursor for pagination.

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `integration_id: optional string`

  Filter by an integration ID

- `max_updated_at: optional string`

  Filter to view remediations updated on or before the max updated datetime. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_updated_at: optional string`

  Filter to view remediations updated on or after the min updated datetime. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `order: optional "created_at" or "affliction_date" or "integration_name" or 4 more`

  An optional param to sort the results by the given field.

  - `"created_at"`

  - `"affliction_date"`

  - `"integration_name"`

  - `"status"`

  - `"last_updated_at"`

  - `"asset_name"`

  - `"finding_type_name"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `search: optional string`

  A search term.

- `status: optional "pending" or "processing" or "completed" or 2 more`

  Filter to view remediations with the given status.

  - `"pending"`

  - `"processing"`

  - `"completed"`

  - `"failed"`

  - `"validating"`

- `triggered_by_actor: optional array of "user" or "account_token"`

  Filter remediations by what kind of actor triggered them. Supports multiple comma-separated values.

  - `"user"`

  - `"account_token"`

### Returns

- `errors: array of unknown`

  Array of error messages.

- `messages: array of unknown`

  Array of informational messages.

- `result: array of object { id, asset, created_at, 11 more }`

  Array of remediation job objects.

  - `id: string`

    Unique identifier for the remediation job.

  - `asset: object { id, category, external_id, 3 more }`

    Asset information for a remediation job.

    - `id: string`

      Unique identifier for the asset.

    - `category: object { service, type, vendor }`

      Category information for a remediation job asset.

      - `service: string`

        Specific service within the vendor.

      - `type: string`

        Asset type.

      - `vendor: "AWS" or "Anthropic" or "Bitbucket" or 16 more`

        Display names for vendor types.

        - `"AWS"`

        - `"Anthropic"`

        - `"Bitbucket"`

        - `"Box"`

        - `"Confluence"`

        - `"Dropbox"`

        - `"GitHub"`

        - `"Google Cloud Platform"`

        - `"Google Workspace"`

        - `"Jira"`

        - `"Microsoft"`

        - `"Microsoft Internal"`

        - `"Okta"`

        - `"OpenAI"`

        - `"Slack"`

        - `"Salesforce"`

        - `"ServiceNow"`

        - `"Workday"`

        - `"Zoom"`

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      Additional fields associated with the asset.

      - `name: string`

        Field name.

      - `value: string or number or boolean`

        Field value (can be string, number, or boolean).

        - `string`

        - `number`

        - `boolean`

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `link: optional string`

      Direct link to the asset.

  - `created_at: string`

    When the remediation job was created.

  - `finding_id: string`

    Encoded finding ID.

  - `finding_instance_id: string`

    ID of the finding instance being remediated.

  - `finding_type_id: string`

    ID of the finding type.

  - `finding_type_name: string`

    Name of the finding type.

  - `integration_name: string`

    Name of the integration.

  - `last_updated: string`

    When the remediation job was last updated.

  - `remediation_type: string`

    Type of remediation being performed.

  - `status: "pending" or "processing" or "completed" or 2 more`

    Status of a remediation job.

    - `"pending"`

    - `"processing"`

    - `"completed"`

    - `"failed"`

    - `"validating"`

  - `triggered_by_user: string`

    Email of the user who triggered the remediation. For account-token actors this is the literal "Account API Token"; for policy actors this is empty.

  - `triggered_by_actor: optional "user" or "account_token"`

    Type of actor that triggered the remediation job. Null on legacy rows created before this column was populated.

    - `"user"`

    - `"account_token"`

  - `triggered_by_id: optional string`

    ID of the actor that triggered the job. Meaning depends on triggered_by_actor. Null on legacy rows.

- `result_info: object { count, cursor, page, 2 more }`

  - `count: optional number`

    Number of results on current page.

  - `cursor: optional string`

    Cursor for pagination.

  - `page: optional number`

    Current page number.

  - `per_page: optional number`

    Number of results per page.

  - `total_count: optional number`

    Total number of results.

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/remediations/jobs \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {}
  ],
  "messages": [
    {}
  ],
  "result": [
    {
      "id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
      "asset": {
        "id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "category": {
          "service": "OneDrive",
          "type": "SaaS",
          "vendor": "Google Workspace"
        },
        "external_id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
        "fields": [
          {
            "name": "File Name",
            "value": "sensitive-document.xlsx",
            "link": "https://dashboard.microsoft.com/files/details"
          }
        ],
        "name": "Microsoft File Publicly Accessible",
        "link": "https://dashboard.microsoft.com/files/details"
      },
      "created_at": "2025-07-07T18:39:13.123456Z",
      "finding_id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
      "finding_instance_id": "3f7b8c9d-6e5a-4f3b-9c2d-1e0a8b7c6d5e",
      "finding_type_id": "775c5f38-efcf-4b2b-93db-8428979eb6a2",
      "finding_type_name": "Microsoft: File publicly accessible with edit access",
      "integration_name": "Microsoft",
      "last_updated": "2025-07-07T18:39:13.123456Z",
      "remediation_type": "Remove publicly accessible edit url",
      "status": "pending",
      "triggered_by_user": "user@example.com",
      "triggered_by_actor": "user",
      "triggered_by_id": "0123456789abcdef0123456789abcdef"
    }
  ],
  "result_info": {
    "count": 2,
    "cursor": "next_cursor_value",
    "page": 1,
    "per_page": 10,
    "total_count": 2
  },
  "success": true
}
```
