## List applications

**get** `/accounts/{account_id}/one/applications`

Returns a list of available applications with use cases and permissions.

### Path Parameters

- `account_id: string`

### Query Parameters

- `environment: optional string`

  Filter by supported environment (standard, fedramp).

### Returns

- `id: "ANTHROPIC" or "BITBUCKET" or "BOX" or 10 more`

  Vendor identifier (e.g. microsoft_internal, google_workspace).

  * `ANTHROPIC` - ANTHROPIC
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
  * `SLACK` - SLACK

  - `"ANTHROPIC"`

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

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/applications \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "id": "ANTHROPIC",
    "auth_methods": [
      {
        "id": "id",
        "display_name": "display_name"
      }
    ],
    "category": "category",
    "description": "description",
    "display_name": "display_name",
    "dlp_enabled": true,
    "logo": "logo",
    "permissions": [
      {
        "display_name": "display_name",
        "scope": "scope",
        "severity": "low"
      }
    ],
    "supported_environments": [
      "string"
    ],
    "use_cases": [
      {
        "id": "id",
        "display_name": "display_name"
      }
    ]
  }
]
```
