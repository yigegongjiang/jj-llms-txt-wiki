## List applications

**get** `/accounts/{account_id}/one/applications`

Returns a list of available applications with use cases and permissions.

### Path Parameters

- `account_id: string`

### Query Parameters

- `environment: optional string`

  Filter by supported environment (standard, fedramp).

- `page: optional number`

  A page number within the paginated result set.

- `page_size: optional number`

  Number of results to return per page.

### Returns

- `errors: array of unknown`

  List of errors.

- `messages: array of string`

  List of messages.

- `result: array of object { id, auth_methods, category, 7 more }`

  List of items.

  - `id: "ANTHROPIC" or "AWS" or "BITBUCKET" or 12 more`

    Vendor identifier (e.g. microsoft_internal, google_workspace).

    * `ANTHROPIC` - ANTHROPIC
    * `AWS` - AWS
    * `BITBUCKET` - BITBUCKET
    * `BOX` - BOX
    * `CONFLUENCE` - CONFLUENCE
    * `DROPBOX` - DROPBOX
    * `GITHUB` - GITHUB
    * `GOOGLE_CLOUD_PLATFORM` - GOOGLE_CLOUD_PLATFORM
    * `GOOGLE_WORKSPACE` - GOOGLE_WORKSPACE
    * `JIRA` - JIRA
    * `MICROSOFT_INTERNAL` - MICROSOFT_INTERNAL
    * `OPENAI` - OPENAI
    * `SALESFORCE` - SALESFORCE
    * `SERVICENOW` - SERVICENOW
    * `SLACK` - SLACK

    - `"ANTHROPIC"`

    - `"AWS"`

    - `"BITBUCKET"`

    - `"BOX"`

    - `"CONFLUENCE"`

    - `"DROPBOX"`

    - `"GITHUB"`

    - `"GOOGLE_CLOUD_PLATFORM"`

    - `"GOOGLE_WORKSPACE"`

    - `"JIRA"`

    - `"MICROSOFT_INTERNAL"`

    - `"OPENAI"`

    - `"SALESFORCE"`

    - `"SERVICENOW"`

    - `"SLACK"`

  - `auth_methods: array of object { id, display_name }`

    Available auth methods.

    - `id: string`

      Auth method identifier.

    - `display_name: string`

      Human-readable auth method name.

  - `category: string`

    Vendor category (e.g. Productivity, AI).

  - `description: string`

    Brief description of the integration.

  - `display_name: string`

    Human-readable vendor name.

  - `dlp_enabled: boolean`

    Whether DLP scanning is supported.

  - `logo: string`

    Logo path.

  - `permissions: array of object { display_name, scope, severity }`

    All permissions with severity.

    - `display_name: string`

      Human-readable permission name.

    - `scope: string`

      Vendor-native scope identifier.

    - `severity: "low" or "medium" or "high" or "critical"`

      Permission sensitivity level.

      * `low` - low
      * `medium` - medium
      * `high` - high
      * `critical` - critical

      - `"low"`

      - `"medium"`

      - `"high"`

      - `"critical"`

  - `supported_environments: array of string`

    Environments this vendor supports (standard, fedramp).

  - `use_cases: array of object { id, display_name }`

    Supported use cases.

    - `id: string`

      Use case identifier (e.g. casb, ces).

    - `display_name: string`

      Human-readable use case name.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/applications \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": [
    {
      "auth_methods": [
        {
          "display_name": "OAuth 2.0 Admin Consent",
          "id": "oauth2_standard"
        }
      ],
      "category": "Productivity",
      "description": "Monitor OneDrive, SharePoint, Teams, and Outlook.",
      "display_name": "Microsoft",
      "dlp_enabled": true,
      "id": "MICROSOFT_INTERNAL",
      "logo": "https://dash.cloudflare.com/v2/static/microsoft_internal.svg",
      "permissions": [
        {
          "display_name": "Read all users' full profiles",
          "scope": "User.Read.All",
          "severity": "high"
        },
        {
          "display_name": "Read all files",
          "scope": "Files.Read.All",
          "severity": "high"
        },
        {
          "display_name": "Read and write mail",
          "scope": "Mail.ReadWrite",
          "severity": "critical"
        }
      ],
      "supported_environments": [
        "standard",
        "fedramp"
      ],
      "use_cases": [
        {
          "display_name": "Cloud Access Security Broker",
          "id": "casb"
        },
        {
          "display_name": "Cloud Email Security",
          "id": "ces"
        }
      ]
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
