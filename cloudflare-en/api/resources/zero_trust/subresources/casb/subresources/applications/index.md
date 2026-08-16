# Applications

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

## Get application details

**get** `/accounts/{account_id}/one/applications/{application_id}`

Returns full application details including auth methods, use cases, and permissions.

### Path Parameters

- `account_id: string`

- `application_id: "ANTHROPIC" or "AWS" or "BITBUCKET" or 12 more`

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

### Returns

- `result: object { id, auth_methods, category, 6 more }`

  The requested item.

  - `id: "ANTHROPIC" or "AWS" or "BITBUCKET" or 12 more`

    Vendor identifier.

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

  - `auth_methods: array of object { id, display_name, is_default, supported_environments }`

    Available authentication methods.

    - `id: string`

      Auth method identifier.

    - `display_name: string`

      Human-readable auth method name.

    - `is_default: boolean`

      Whether this is the default auth method.

    - `supported_environments: array of string`

      Environments this auth method supports.

  - `category: string`

    Vendor category.

  - `description: string`

    Brief description.

  - `display_name: string`

    Human-readable vendor name.

  - `dlp_enabled: boolean`

    Whether DLP scanning is supported.

  - `instructions: string`

    Setup instructions for the user.

  - `logo: string`

    Logo path.

  - `use_cases: array of object { id, base_scopes, description, 2 more }`

    Use cases with full scope details.

    - `id: string`

      Use case identifier.

    - `base_scopes: array of object { display_name, scope, severity }`

      Scopes always required for this use case.

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

    - `description: string`

      Use case description.

    - `display_name: string`

      Human-readable use case name.

    - `features: array of object { id, description, display_name, scopes }`

      Optional features with extra scopes.

      - `id: string`

        Feature identifier.

      - `description: string`

        Feature description.

      - `display_name: string`

        Human-readable feature name.

      - `scopes: array of object { display_name, scope, severity }`

        Additional scopes when feature is enabled.

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

- `success: boolean`

  Whether the request succeeded.

- `errors: optional array of map[unknown]`

  List of errors.

- `messages: optional array of string`

  List of messages.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/applications/$APPLICATION_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "auth_methods": [
      {
        "display_name": "OAuth 2.0 Admin Consent",
        "id": "oauth2",
        "is_default": true,
        "supported_environments": [
          "standard",
          "fedramp"
        ]
      }
    ],
    "category": "Productivity",
    "description": "Monitor OneDrive, SharePoint, Teams, and Outlook.",
    "display_name": "Microsoft",
    "dlp_enabled": true,
    "id": "MICROSOFT_INTERNAL",
    "instructions": "You'll need a Microsoft 365 admin account with Global Admin or Application Admin role.",
    "logo": "https://dash.cloudflare.com/v2/static/microsoft_internal.svg",
    "use_cases": [
      {
        "base_scopes": [
          {
            "display_name": "Read all users' full profiles",
            "scope": "User.Read.All",
            "severity": "high"
          },
          {
            "display_name": "Read all files",
            "scope": "Files.Read.All",
            "severity": "high"
          }
        ],
        "description": "Discover and secure SaaS applications",
        "display_name": "Cloud Access Security Broker",
        "features": [
          {
            "description": "Automatically remediate security issues",
            "display_name": "Auto Remediation",
            "id": "auto_remediation",
            "scopes": [
              {
                "display_name": "Read and write all files",
                "scope": "Files.ReadWrite.All",
                "severity": "critical"
              }
            ]
          }
        ],
        "id": "casb"
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Application List Response

- `ApplicationListResponse object { id, auth_methods, category, 7 more }`

  Application item in list response.

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

### Application Get Response

- `ApplicationGetResponse object { id, auth_methods, category, 6 more }`

  The requested item.

  - `id: "ANTHROPIC" or "AWS" or "BITBUCKET" or 12 more`

    Vendor identifier.

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

  - `auth_methods: array of object { id, display_name, is_default, supported_environments }`

    Available authentication methods.

    - `id: string`

      Auth method identifier.

    - `display_name: string`

      Human-readable auth method name.

    - `is_default: boolean`

      Whether this is the default auth method.

    - `supported_environments: array of string`

      Environments this auth method supports.

  - `category: string`

    Vendor category.

  - `description: string`

    Brief description.

  - `display_name: string`

    Human-readable vendor name.

  - `dlp_enabled: boolean`

    Whether DLP scanning is supported.

  - `instructions: string`

    Setup instructions for the user.

  - `logo: string`

    Logo path.

  - `use_cases: array of object { id, base_scopes, description, 2 more }`

    Use cases with full scope details.

    - `id: string`

      Use case identifier.

    - `base_scopes: array of object { display_name, scope, severity }`

      Scopes always required for this use case.

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

    - `description: string`

      Use case description.

    - `display_name: string`

      Human-readable use case name.

    - `features: array of object { id, description, display_name, scopes }`

      Optional features with extra scopes.

      - `id: string`

        Feature identifier.

      - `description: string`

        Feature description.

      - `display_name: string`

        Human-readable feature name.

      - `scopes: array of object { display_name, scope, severity }`

        Additional scopes when feature is enabled.

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

# Auth Methods

## Get auth methods

**get** `/accounts/{account_id}/one/applications/{application_id}/auth-methods`

Returns available auth methods for the specified vendor, including credential schema, instructions, and example payloads. Use this to understand what credentials are required before calling POST /v2/integrations.

### Path Parameters

- `account_id: string`

- `application_id: "ANTHROPIC" or "AWS" or "BITBUCKET" or 12 more`

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

### Query Parameters

- `page: optional number`

  A page number within the paginated result set.

- `page_size: optional number`

  Number of results to return per page.

### Returns

- `errors: array of unknown`

  List of errors.

- `messages: array of string`

  List of messages.

- `result: array of object { id, display_name, human_interaction_required, 4 more }`

  List of items.

  - `id: string`

    Auth method identifier.

  - `display_name: string`

    Human-readable auth method name.

  - `human_interaction_required: boolean`

    Whether setup requires human interaction or integration can be created purely using API (e.g., For OAuth can not be created without user interaction).

  - `instructions: object { markdown }`

    Step-by-step instructions for obtaining credentials.

    - `markdown: string`

      Detailed instructions in markdown format.

  - `payload_example: map[unknown]`

    Example credentials payload with placeholder values.

  - `payload_schema: map[unknown]`

    JSON Schema for the credentials object in POST /v2/integrations request.

  - `redirect_url: string`

    OAuth redirect URL for vendors requiring human interaction.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/applications/$APPLICATION_ID/auth-methods \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": [
    {
      "display_name": "API Key",
      "human_interaction_required": false,
      "id": "api_key",
      "instructions": {
        "markdown": "## Getting your API Key\n\n1. Log in to your admin console\n2. Navigate to Settings > API\n3. Generate a new API key"
      },
      "payload_example": {
        "api_key": "sk-xxxxxxxxxxxxxxxxxxxx"
      },
      "payload_schema": {
        "properties": {
          "api_key": {
            "description": "Your API key",
            "type": "string"
          }
        },
        "required": [
          "api_key"
        ],
        "type": "object"
      },
      "redirect_url": null
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

## Domain Types

### Auth Method List Response

- `AuthMethodListResponse object { id, display_name, human_interaction_required, 4 more }`

  Detailed auth method info including credentials schema and instructions.

  - `id: string`

    Auth method identifier.

  - `display_name: string`

    Human-readable auth method name.

  - `human_interaction_required: boolean`

    Whether setup requires human interaction or integration can be created purely using API (e.g., For OAuth can not be created without user interaction).

  - `instructions: object { markdown }`

    Step-by-step instructions for obtaining credentials.

    - `markdown: string`

      Detailed instructions in markdown format.

  - `payload_example: map[unknown]`

    Example credentials payload with placeholder values.

  - `payload_schema: map[unknown]`

    JSON Schema for the credentials object in POST /v2/integrations request.

  - `redirect_url: string`

    OAuth redirect URL for vendors requiring human interaction.
