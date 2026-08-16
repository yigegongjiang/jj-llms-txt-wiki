# Casb

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

# Integrations

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

## Get integration details

**get** `/accounts/{account_id}/one/integrations/{id}`

Returns full integration details including use cases and permissions.

### Path Parameters

- `account_id: string`

- `id: string`

### Returns

- `result: object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

- `success: boolean`

  Whether the request succeeded.

- `errors: optional array of map[unknown]`

  List of errors.

- `messages: optional array of string`

  List of messages.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations/$ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "application": {
      "category": "Productivity",
      "display_name": "Google Workspace",
      "logo": "https://dash.cloudflare.com/v2/static/google_workspace.png"
    },
    "auth_method": {
      "display_name": "OAuth 2.0",
      "id": "oauth"
    },
    "authorization_link": {
      "components": {
        "client_id": "abc",
        "instance_name": "example"
      },
      "link": "https://example.cloudflare.com/authorize"
    },
    "created": "2025-01-01T00:00:00Z",
    "credentials_expiry": "2026-01-01T00:00:00Z",
    "dlp_profiles": [
      "e91a2360-da51-4fdf-9711-bcdecd462614"
    ],
    "health_details": [],
    "id": "019d2e6a-d995-7185-afbd-4feead9e42ec",
    "is_paused": false,
    "last_hydrated": "2025-04-10T08:30:00Z",
    "name": "My Google Workspace",
    "status": "Healthy",
    "updated": "2025-04-10T08:30:00Z",
    "use_cases": [
      {
        "description": "Discover and secure SaaS applications",
        "features": [
          {
            "description": "Automatically remediate security issues (requires write permissions)",
            "id": "auto_remediation",
            "is_enabled": true,
            "name": "Auto Remediation",
            "permissions": [
              {
                "display_name": "Manage users",
                "scope": "https://www.googleapis.com/auth/admin.directory.user",
                "status": "granted"
              }
            ]
          }
        ],
        "id": "casb",
        "is_enabled": true,
        "name": "Cloud Access Security Broker",
        "permissions": [
          {
            "display_name": "Drive (Read Only)",
            "scope": "https://www.googleapis.com/auth/drive.readonly",
            "status": "granted"
          },
          {
            "display_name": "Gmail (Read Only)",
            "scope": "https://www.googleapis.com/auth/gmail.readonly",
            "status": "missing"
          }
        ]
      },
      {
        "description": "Protect against email-based threats",
        "features": [],
        "id": "ces",
        "is_enabled": false,
        "name": "Cloud Email Security",
        "permissions": []
      }
    ]
  },
  "success": true
}
```

## Create integration

**post** `/accounts/{account_id}/one/integrations`

Creates a new integration for the specified application. Integration creation with OAuth is not supported by API at the moment. For other auth methods, use `GET /v2/applications/{application_id}/credential-guide` to see the required credential structure and example payloads for each vendor.

### Path Parameters

- `account_id: string`

### Body Parameters

- `application: "ANTHROPIC" or "AWS" or "BITBUCKET" or 12 more`

  Vendor/application slug (e.g., GOOGLE_WORKSPACE).

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

- `credentials: map[unknown]`

  Credentials for the integration.

- `name: string`

  Name of the integration.

- `auth_method: optional string`

  Authentication method slug (uses default if omitted).

- `dlp_profiles: optional array of string`

  List of DLP profile IDs to associate.

- `permissions: optional array of string`

  List of permission scopes (uses policy defaults if empty).

- `use_cases: optional array of "casb" or "ces" or "auto_remediation"`

  List of use case or feature slugs to enroll (e.g., ['casb', 'ces', 'auto_remediation']).

  - `"casb"`

  - `"ces"`

  - `"auto_remediation"`

### Returns

- `result: object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

- `success: boolean`

  Whether the request succeeded.

- `errors: optional array of map[unknown]`

  List of errors.

- `messages: optional array of string`

  List of messages.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "application": "GOOGLE_WORKSPACE",
          "credentials": {
            "admin_email": "bar"
          },
          "name": "My Google Workspace"
        }'
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "application": {
      "category": "Productivity",
      "display_name": "Google Workspace",
      "logo": "https://dash.cloudflare.com/v2/static/google_workspace.png"
    },
    "auth_method": {
      "display_name": "OAuth 2.0",
      "id": "oauth"
    },
    "authorization_link": {
      "components": {
        "client_id": "abc",
        "instance_name": "example"
      },
      "link": "https://example.cloudflare.com/authorize"
    },
    "created": "2025-01-01T00:00:00Z",
    "credentials_expiry": "2026-01-01T00:00:00Z",
    "dlp_profiles": [
      "e91a2360-da51-4fdf-9711-bcdecd462614"
    ],
    "health_details": [],
    "id": "019d2e6a-d995-7185-afbd-4feead9e42ec",
    "is_paused": false,
    "last_hydrated": "2025-04-10T08:30:00Z",
    "name": "My Google Workspace",
    "status": "Healthy",
    "updated": "2025-04-10T08:30:00Z",
    "use_cases": [
      {
        "description": "Discover and secure SaaS applications",
        "features": [
          {
            "description": "Automatically remediate security issues (requires write permissions)",
            "id": "auto_remediation",
            "is_enabled": true,
            "name": "Auto Remediation",
            "permissions": [
              {
                "display_name": "Manage users",
                "scope": "https://www.googleapis.com/auth/admin.directory.user",
                "status": "granted"
              }
            ]
          }
        ],
        "id": "casb",
        "is_enabled": true,
        "name": "Cloud Access Security Broker",
        "permissions": [
          {
            "display_name": "Drive (Read Only)",
            "scope": "https://www.googleapis.com/auth/drive.readonly",
            "status": "granted"
          },
          {
            "display_name": "Gmail (Read Only)",
            "scope": "https://www.googleapis.com/auth/gmail.readonly",
            "status": "missing"
          }
        ]
      },
      {
        "description": "Protect against email-based threats",
        "features": [],
        "id": "ces",
        "is_enabled": false,
        "name": "Cloud Email Security",
        "permissions": []
      }
    ]
  },
  "success": true
}
```

## Update integration

**patch** `/accounts/{account_id}/one/integrations/{id}`

Updates an integration's name, permissions, DLP profiles, use cases, or credentials.

### Path Parameters

- `account_id: string`

- `id: string`

### Body Parameters

- `credentials: optional map[unknown]`

  Partial credential fields to merge with existing.

- `dlp_profiles: optional array of string`

  List of DLP profile IDs to associate with the integration.

- `name: optional string`

  Name of the integration.

- `permissions: optional array of string`

  List of permission scopes granted to the integration.

- `use_cases: optional array of "casb" or "ces" or "auto_remediation"`

  List of use case or feature slugs to enroll (e.g., ['casb', 'ces', 'auto_remediation']).

  - `"casb"`

  - `"ces"`

  - `"auto_remediation"`

### Returns

- `result: object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

- `success: boolean`

  Whether the request succeeded.

- `errors: optional array of map[unknown]`

  List of errors.

- `messages: optional array of string`

  List of messages.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations/$ID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "application": {
      "category": "Productivity",
      "display_name": "Google Workspace",
      "logo": "https://dash.cloudflare.com/v2/static/google_workspace.png"
    },
    "auth_method": {
      "display_name": "OAuth 2.0",
      "id": "oauth"
    },
    "authorization_link": {
      "components": {
        "client_id": "abc",
        "instance_name": "example"
      },
      "link": "https://example.cloudflare.com/authorize"
    },
    "created": "2025-01-01T00:00:00Z",
    "credentials_expiry": "2026-01-01T00:00:00Z",
    "dlp_profiles": [
      "e91a2360-da51-4fdf-9711-bcdecd462614"
    ],
    "health_details": [],
    "id": "019d2e6a-d995-7185-afbd-4feead9e42ec",
    "is_paused": false,
    "last_hydrated": "2025-04-10T08:30:00Z",
    "name": "My Google Workspace",
    "status": "Healthy",
    "updated": "2025-04-10T08:30:00Z",
    "use_cases": [
      {
        "description": "Discover and secure SaaS applications",
        "features": [
          {
            "description": "Automatically remediate security issues (requires write permissions)",
            "id": "auto_remediation",
            "is_enabled": true,
            "name": "Auto Remediation",
            "permissions": [
              {
                "display_name": "Manage users",
                "scope": "https://www.googleapis.com/auth/admin.directory.user",
                "status": "granted"
              }
            ]
          }
        ],
        "id": "casb",
        "is_enabled": true,
        "name": "Cloud Access Security Broker",
        "permissions": [
          {
            "display_name": "Drive (Read Only)",
            "scope": "https://www.googleapis.com/auth/drive.readonly",
            "status": "granted"
          },
          {
            "display_name": "Gmail (Read Only)",
            "scope": "https://www.googleapis.com/auth/gmail.readonly",
            "status": "missing"
          }
        ]
      },
      {
        "description": "Protect against email-based threats",
        "features": [],
        "id": "ces",
        "is_enabled": false,
        "name": "Cloud Email Security",
        "permissions": []
      }
    ]
  },
  "success": true
}
```

## Delete integration

**delete** `/accounts/{account_id}/one/integrations/{id}`

Delete an integration by soft-deleting it.

### Path Parameters

- `account_id: string`

- `id: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations/$ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

## Pause integration

**post** `/accounts/{account_id}/one/integrations/{id}/pause`

Pauses an integration, stopping all crawlers.

### Path Parameters

- `account_id: string`

- `id: string`

### Returns

- `result: object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

- `success: boolean`

  Whether the request succeeded.

- `errors: optional array of map[unknown]`

  List of errors.

- `messages: optional array of string`

  List of messages.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations/$ID/pause \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "application": {
      "category": "Productivity",
      "display_name": "Google Workspace",
      "logo": "https://dash.cloudflare.com/v2/static/google_workspace.png"
    },
    "auth_method": {
      "display_name": "OAuth 2.0",
      "id": "oauth"
    },
    "authorization_link": {
      "components": {
        "client_id": "abc",
        "instance_name": "example"
      },
      "link": "https://example.cloudflare.com/authorize"
    },
    "created": "2025-01-01T00:00:00Z",
    "credentials_expiry": "2026-01-01T00:00:00Z",
    "dlp_profiles": [
      "e91a2360-da51-4fdf-9711-bcdecd462614"
    ],
    "health_details": [],
    "id": "019d2e6a-d995-7185-afbd-4feead9e42ec",
    "is_paused": true,
    "last_hydrated": "2025-04-10T08:30:00Z",
    "name": "My Google Workspace",
    "status": "Healthy",
    "updated": "2025-04-10T08:30:00Z",
    "use_cases": [
      {
        "description": "Discover and secure SaaS applications",
        "features": [
          {
            "description": "Automatically remediate security issues (requires write permissions)",
            "id": "auto_remediation",
            "is_enabled": true,
            "name": "Auto Remediation",
            "permissions": [
              {
                "display_name": "Manage users",
                "scope": "https://www.googleapis.com/auth/admin.directory.user",
                "status": "granted"
              }
            ]
          }
        ],
        "id": "casb",
        "is_enabled": true,
        "name": "Cloud Access Security Broker",
        "permissions": [
          {
            "display_name": "Drive (Read Only)",
            "scope": "https://www.googleapis.com/auth/drive.readonly",
            "status": "granted"
          },
          {
            "display_name": "Gmail (Read Only)",
            "scope": "https://www.googleapis.com/auth/gmail.readonly",
            "status": "missing"
          }
        ]
      },
      {
        "description": "Protect against email-based threats",
        "features": [],
        "id": "ces",
        "is_enabled": false,
        "name": "Cloud Email Security",
        "permissions": []
      }
    ]
  },
  "success": true
}
```

## Resume integration

**post** `/accounts/{account_id}/one/integrations/{id}/resume`

Resumes a paused integration, restarting crawlers.

### Path Parameters

- `account_id: string`

- `id: string`

### Returns

- `result: object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

- `success: boolean`

  Whether the request succeeded.

- `errors: optional array of map[unknown]`

  List of errors.

- `messages: optional array of string`

  List of messages.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/integrations/$ID/resume \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "application": {
      "category": "Productivity",
      "display_name": "Google Workspace",
      "logo": "https://dash.cloudflare.com/v2/static/google_workspace.png"
    },
    "auth_method": {
      "display_name": "OAuth 2.0",
      "id": "oauth"
    },
    "authorization_link": {
      "components": {
        "client_id": "abc",
        "instance_name": "example"
      },
      "link": "https://example.cloudflare.com/authorize"
    },
    "created": "2025-01-01T00:00:00Z",
    "credentials_expiry": "2026-01-01T00:00:00Z",
    "dlp_profiles": [
      "e91a2360-da51-4fdf-9711-bcdecd462614"
    ],
    "health_details": [],
    "id": "019d2e6a-d995-7185-afbd-4feead9e42ec",
    "is_paused": false,
    "last_hydrated": "2025-04-10T08:30:00Z",
    "name": "My Google Workspace",
    "status": "Healthy",
    "updated": "2025-04-10T08:30:00Z",
    "use_cases": [
      {
        "description": "Discover and secure SaaS applications",
        "features": [
          {
            "description": "Automatically remediate security issues (requires write permissions)",
            "id": "auto_remediation",
            "is_enabled": true,
            "name": "Auto Remediation",
            "permissions": [
              {
                "display_name": "Manage users",
                "scope": "https://www.googleapis.com/auth/admin.directory.user",
                "status": "granted"
              }
            ]
          }
        ],
        "id": "casb",
        "is_enabled": true,
        "name": "Cloud Access Security Broker",
        "permissions": [
          {
            "display_name": "Drive (Read Only)",
            "scope": "https://www.googleapis.com/auth/drive.readonly",
            "status": "granted"
          },
          {
            "display_name": "Gmail (Read Only)",
            "scope": "https://www.googleapis.com/auth/gmail.readonly",
            "status": "missing"
          }
        ]
      },
      {
        "description": "Protect against email-based threats",
        "features": [],
        "id": "ces",
        "is_enabled": false,
        "name": "Cloud Email Security",
        "permissions": []
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Integration List Response

- `IntegrationListResponse object { id, application, created, 4 more }`

  Serializer for v2 integration list responses.

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

### Integration Get Response

- `IntegrationGetResponse object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

### Integration Create Response

- `IntegrationCreateResponse object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

### Integration Update Response

- `IntegrationUpdateResponse object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

### Integration Pause Response

- `IntegrationPauseResponse object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

### Integration Resume Response

- `IntegrationResumeResponse object { id, application, auth_method, 11 more }`

  The requested item.

  - `id: string`

    Integration ID.

  - `application: map[string]`

  - `auth_method: map[string]`

    The integration's authentication method.

  - `authorization_link: object { components, link }`

    Authorization link for the integration.

    - `components: map[unknown]`

    - `link: string`

  - `created: string`

    When the integration was created.

  - `credentials_expiry: string`

    Credentials expiry time.

  - `dlp_profiles: array of string`

    DLP Profiles enabled for the integration.

  - `health_details: array of map[unknown]`

    Health details with remediation hints.

  - `is_paused: boolean`

    Whether the user paused the integration.

  - `last_hydrated: string`

    Last time the integration was hydrated.

  - `name: string`

    Name of the integration.

  - `status: string`

    Integration status.

  - `updated: string`

    When the integration was last updated.

  - `use_cases: array of map[unknown]`

    Use cases enabled for the integration.

# Posture

# Findings

## List posture findings

**get** `/accounts/{account_id}/data-security/posture/findings`

List all security findings that have been identified as being problematic.
This will return a list of findings regardless if they have been ignored or not.

### Path Parameters

- `account_id: string`

### Query Parameters

- `cursor: optional string`

  A cursor for pagination. Obtained from the `result_info.cursor` field of a previous response.

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `finding_type_ids: optional string`

  A comma separated list of UUIDs identifying the finding type(s).

- `ignored: optional boolean`

  Filter for only the ignored findings. Set to false to only see "active" items

- `integration_id: optional string`

  Filter by an integration ID

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `observation: optional "Activity" or "Insight" or "Issue"`

  Filter by observation type of the finding

  - `"Activity"`

  - `"Insight"`

  - `"Issue"`

- `order: optional "finding.name" or "instance_count" or "integration.name" or 2 more`

  Which field to use when ordering the findings.

  - `"finding.name"`

  - `"instance_count"`

  - `"integration.name"`

  - `"latest_affliction_date"`

  - `"severity"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `product: optional "Cloud" or "Saas"`

  Filter by product category of the finding

  - `"Cloud"`

  - `"Saas"`

- `search: optional string`

  A search term.

- `severity: optional "Critical" or "High" or "Medium" or "Low"`

  Filter by severity

  - `"Critical"`

  - `"High"`

  - `"Medium"`

  - `"Low"`

- `type: optional "Content" or "Posture"`

  Filter by type of the finding

  - `"Content"`

  - `"Posture"`

- `vendor: optional "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter by vendor

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

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `result: optional array of object { id, active_count, archived_count, 6 more }`

  Array of finding objects.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings \
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
      "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
      "active_count": 5,
      "archived_count": 2,
      "finding": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "category": {
          "observation": "Issue",
          "product": "SaaS",
          "type": "Posture"
        },
        "name": "Slack File Publicly Accessible",
        "severity": "High",
        "vendor": "Google Workspace",
        "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
        "remediation": {
          "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
          "frameworks": [
            "SOC2",
            "ISO27001"
          ],
          "guide": "To fix this issue, update the file permissions to remove public access.",
          "impact": "Publicly accessible files may expose sensitive information.",
          "locale": "en-US",
          "threat": "Data exposure and potential compliance violations."
        }
      },
      "ignored": false,
      "instance_count": 7,
      "integration": {
        "created": "2021-08-10T20:16:11.851451Z",
        "last_hydrated": "2025-03-18T17:25:38.697894Z",
        "name": "Example integration",
        "permissions": [
          "GroupMember.Read.All",
          "Group.Read.All"
        ],
        "policy": {
          "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
          "client_id": "client_id",
          "compliance_level": "standard",
          "dlp_enabled": true,
          "link": "https://example.com",
          "name": "Google Workspace Standard Policy",
          "permissions": [
            "https://www.googleapis.com/auth/admin.directory.domain.readonly",
            "https://www.googleapis.com/auth/admin.directory.user.readonly"
          ]
        },
        "status": "Healthy",
        "updated": "2021-08-10T20:16:11.851451Z",
        "upgradable": false,
        "vendor": {
          "id": "R09PR0xFX1dPUktTUEFDRQ==",
          "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
          "display_name": "Google Workspace",
          "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
          "name": "GOOGLE_WORKSPACE",
          "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
          "zt_enrollments": [
            "casb"
          ],
          "policies": [
            {
              "foo": "bar"
            }
          ]
        },
        "zt_enrollments": [
          {
            "id": "casb",
            "description": "example",
            "display_name": "Cloud Access Security Broker",
            "enabled": true
          }
        ],
        "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
        "credential_health_status": "Healthy",
        "credentials_expiry": "2025-03-18T17:25:38.697902Z",
        "is_paused": false,
        "upgrade_dismissed": false
      },
      "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
      "severity_override": {
        "created_by": "1234",
        "severity": "Critical"
      }
    }
  ]
}
```

## Get a finding type

**get** `/accounts/{account_id}/data-security/posture/findings/{finding_id}`

Gets a security Finding that has been identified as being problematic.

### Path Parameters

- `account_id: string`

- `finding_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID \
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
  "success": true,
  "result": {
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
    "integration": {
      "created": "2021-08-10T20:16:11.851451Z",
      "last_hydrated": "2025-03-18T17:25:38.697894Z",
      "name": "Example integration",
      "permissions": [
        "GroupMember.Read.All",
        "Group.Read.All"
      ],
      "policy": {
        "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
        "client_id": "client_id",
        "compliance_level": "standard",
        "dlp_enabled": true,
        "link": "https://example.com",
        "name": "Google Workspace Standard Policy",
        "permissions": [
          "https://www.googleapis.com/auth/admin.directory.domain.readonly",
          "https://www.googleapis.com/auth/admin.directory.user.readonly"
        ]
      },
      "status": "Healthy",
      "updated": "2021-08-10T20:16:11.851451Z",
      "upgradable": false,
      "vendor": {
        "id": "R09PR0xFX1dPUktTUEFDRQ==",
        "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
        "display_name": "Google Workspace",
        "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
        "name": "GOOGLE_WORKSPACE",
        "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
        "zt_enrollments": [
          "casb"
        ],
        "policies": [
          {
            "foo": "bar"
          }
        ]
      },
      "zt_enrollments": [
        {
          "id": "casb",
          "description": "example",
          "display_name": "Cloud Access Security Broker",
          "enabled": true
        }
      ],
      "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
      "credential_health_status": "Healthy",
      "credentials_expiry": "2025-03-18T17:25:38.697902Z",
      "is_paused": false,
      "upgrade_dismissed": false
    },
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Create new findings export request

**post** `/accounts/{account_id}/data-security/posture/findings/export`

Creates a CSV export for findings and accepts optional filters in the payload.

### Path Parameters

- `account_id: string`

### Body Parameters

- `ignored: optional boolean`

  Filter for only the ignored findings. Set to false to only see active items.

- `integration_id: optional array of string`

  Filter by multiple integration IDs.

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `orders: optional array of object { direction, name }`

  Which fields to use when ordering the findings.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "instance_count" or "finding.name" or "integration.name" or 2 more`

    Which field to use when ordering the findings.

    - `"instance_count"`

    - `"finding.name"`

    - `"integration.name"`

    - `"latest_affliction_date"`

    - `"severity"`

- `product: optional "SaaS" or "Cloud"`

  Filter by finding's category product.

  - `"SaaS"`

  - `"Cloud"`

- `search: optional string`

  A search term.

- `severities: optional array of "CRITICAL" or "HIGH" or "MEDIUM" or "LOW"`

  Filter by severity levels.

  - `"CRITICAL"`

  - `"HIGH"`

  - `"MEDIUM"`

  - `"LOW"`

- `vendors: optional array of "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter by vendor types.

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

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/export \
    -X POST \
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
  "success": true,
  "result": {
    "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
    "status": "Success",
    "type": "finding",
    "user_id": "e7712d506b1ee4c5ede0802815f55a75",
    "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
    "errors": null,
    "file_name": "findings_export_2024-02-27.csv",
    "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
  }
}
```

## Mark a finding as ignored

**post** `/accounts/{account_id}/data-security/posture/findings/ignore`

Given a list of findings, mark as ignored. Does nothing if Finding is already ignored.

### Path Parameters

- `account_id: string`

### Body Parameters

- `checks: array of string`

  A list of finding IDs to pass along.

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/ignore \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "checks": [
            "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo="
          ]
        }'
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
  "success": true,
  "result": {
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
    "integration": {
      "created": "2021-08-10T20:16:11.851451Z",
      "last_hydrated": "2025-03-18T17:25:38.697894Z",
      "name": "Example integration",
      "permissions": [
        "GroupMember.Read.All",
        "Group.Read.All"
      ],
      "policy": {
        "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
        "client_id": "client_id",
        "compliance_level": "standard",
        "dlp_enabled": true,
        "link": "https://example.com",
        "name": "Google Workspace Standard Policy",
        "permissions": [
          "https://www.googleapis.com/auth/admin.directory.domain.readonly",
          "https://www.googleapis.com/auth/admin.directory.user.readonly"
        ]
      },
      "status": "Healthy",
      "updated": "2021-08-10T20:16:11.851451Z",
      "upgradable": false,
      "vendor": {
        "id": "R09PR0xFX1dPUktTUEFDRQ==",
        "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
        "display_name": "Google Workspace",
        "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
        "name": "GOOGLE_WORKSPACE",
        "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
        "zt_enrollments": [
          "casb"
        ],
        "policies": [
          {
            "foo": "bar"
          }
        ]
      },
      "zt_enrollments": [
        {
          "id": "casb",
          "description": "example",
          "display_name": "Cloud Access Security Broker",
          "enabled": true
        }
      ],
      "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
      "credential_health_status": "Healthy",
      "credentials_expiry": "2025-03-18T17:25:38.697902Z",
      "is_paused": false,
      "upgrade_dismissed": false
    },
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Remove ignore marker from a finding

**post** `/accounts/{account_id}/data-security/posture/findings/unignore`

Ability to un-ignore a Finding if it's previously been ignored. Does nothing if the Finding is not ignored.

### Path Parameters

- `account_id: string`

### Body Parameters

- `checks: array of string`

  A list of finding IDs to pass along.

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/unignore \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "checks": [
            "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo="
          ]
        }'
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
  "success": true,
  "result": {
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
    "integration": {
      "created": "2021-08-10T20:16:11.851451Z",
      "last_hydrated": "2025-03-18T17:25:38.697894Z",
      "name": "Example integration",
      "permissions": [
        "GroupMember.Read.All",
        "Group.Read.All"
      ],
      "policy": {
        "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
        "client_id": "client_id",
        "compliance_level": "standard",
        "dlp_enabled": true,
        "link": "https://example.com",
        "name": "Google Workspace Standard Policy",
        "permissions": [
          "https://www.googleapis.com/auth/admin.directory.domain.readonly",
          "https://www.googleapis.com/auth/admin.directory.user.readonly"
        ]
      },
      "status": "Healthy",
      "updated": "2021-08-10T20:16:11.851451Z",
      "upgradable": false,
      "vendor": {
        "id": "R09PR0xFX1dPUktTUEFDRQ==",
        "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
        "display_name": "Google Workspace",
        "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
        "name": "GOOGLE_WORKSPACE",
        "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
        "zt_enrollments": [
          "casb"
        ],
        "policies": [
          {
            "foo": "bar"
          }
        ]
      },
      "zt_enrollments": [
        {
          "id": "casb",
          "description": "example",
          "display_name": "Cloud Access Security Broker",
          "enabled": true
        }
      ],
      "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
      "credential_health_status": "Healthy",
      "credentials_expiry": "2025-03-18T17:25:38.697902Z",
      "is_paused": false,
      "upgrade_dismissed": false
    },
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Update the severity for a finding

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/tune_finding_severity`

Update the severity of a Finding.
This will update the `severity_override` field on the Finding payload with the new severity value.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Body Parameters

- `new_severity: 1 or 2 or 3 or 4`

  The numeric severity value to apply to the finding.

  - `1`

  - `2`

  - `3`

  - `4`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/tune_finding_severity \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "new_severity": 1
        }'
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
  "success": true,
  "result": {
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
    "integration": {
      "created": "2021-08-10T20:16:11.851451Z",
      "last_hydrated": "2025-03-18T17:25:38.697894Z",
      "name": "Example integration",
      "permissions": [
        "GroupMember.Read.All",
        "Group.Read.All"
      ],
      "policy": {
        "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
        "client_id": "client_id",
        "compliance_level": "standard",
        "dlp_enabled": true,
        "link": "https://example.com",
        "name": "Google Workspace Standard Policy",
        "permissions": [
          "https://www.googleapis.com/auth/admin.directory.domain.readonly",
          "https://www.googleapis.com/auth/admin.directory.user.readonly"
        ]
      },
      "status": "Healthy",
      "updated": "2021-08-10T20:16:11.851451Z",
      "upgradable": false,
      "vendor": {
        "id": "R09PR0xFX1dPUktTUEFDRQ==",
        "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
        "display_name": "Google Workspace",
        "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
        "name": "GOOGLE_WORKSPACE",
        "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
        "zt_enrollments": [
          "casb"
        ],
        "policies": [
          {
            "foo": "bar"
          }
        ]
      },
      "zt_enrollments": [
        {
          "id": "casb",
          "description": "example",
          "display_name": "Cloud Access Security Broker",
          "enabled": true
        }
      ],
      "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
      "credential_health_status": "Healthy",
      "credentials_expiry": "2025-03-18T17:25:38.697902Z",
      "is_paused": false,
      "upgrade_dismissed": false
    },
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Reset severity for a finding back to the default

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/reset_finding_severity`

If a Finding's severity has been changed, reset it back to default value.
Does nothing if no override exists.

### Path Parameters

- `account_id: string`

- `finding_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/reset_finding_severity \
    -X POST \
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
  "success": true,
  "result": {
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
    "integration": {
      "created": "2021-08-10T20:16:11.851451Z",
      "last_hydrated": "2025-03-18T17:25:38.697894Z",
      "name": "Example integration",
      "permissions": [
        "GroupMember.Read.All",
        "Group.Read.All"
      ],
      "policy": {
        "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
        "client_id": "client_id",
        "compliance_level": "standard",
        "dlp_enabled": true,
        "link": "https://example.com",
        "name": "Google Workspace Standard Policy",
        "permissions": [
          "https://www.googleapis.com/auth/admin.directory.domain.readonly",
          "https://www.googleapis.com/auth/admin.directory.user.readonly"
        ]
      },
      "status": "Healthy",
      "updated": "2021-08-10T20:16:11.851451Z",
      "upgradable": false,
      "vendor": {
        "id": "R09PR0xFX1dPUktTUEFDRQ==",
        "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
        "display_name": "Google Workspace",
        "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
        "name": "GOOGLE_WORKSPACE",
        "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
        "zt_enrollments": [
          "casb"
        ],
        "policies": [
          {
            "foo": "bar"
          }
        ]
      },
      "zt_enrollments": [
        {
          "id": "casb",
          "description": "example",
          "display_name": "Cloud Access Security Broker",
          "enabled": true
        }
      ],
      "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
      "credential_health_status": "Healthy",
      "credentials_expiry": "2025-03-18T17:25:38.697902Z",
      "is_paused": false,
      "upgrade_dismissed": false
    },
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Domain Types

### Finding List Response

- `FindingListResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Get Response

- `FindingGetResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Export Response

- `FindingExportResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Finding Ignore Response

- `FindingIgnoreResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Unignore Response

- `FindingUnignoreResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Tune Severity Response

- `FindingTuneSeverityResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Reset Severity Response

- `FindingResetSeverityResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

# Instances

## List instances of a finding

**get** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances`

Lists all security finding instances for a given security finding.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Query Parameters

- `archived: optional boolean`

  Archived

- `asset_ids: optional array of string`

  Filter finding instances by an array of asset IDs. Supports multiple comma-separated values.

- `cursor: optional string`

  A cursor for pagination. Obtained from the `result_info.cursor` field of a previous response.

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `finding_instance_ids: optional array of string`

  Filter finding instances by an array of finding instance IDs. Supports multiple comma-separated values.

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `order: optional "affliction_date" or "asset.name" or "remediation.status"`

  Which field to use when ordering the Finding's instances.
  When ordering by 'remediation.status', only the most recent non-stale remediation job is considered. Stale jobs (created before the instance's affliction_date) are treated as having no status for ordering purposes.

  - `"affliction_date"`

  - `"asset.name"`

  - `"remediation.status"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `remediation_statuses: optional array of "none" or "pending" or "processing" or 3 more`

  Filter finding instances by most recent remediation job status. Supports multiple comma-separated values.
  Use 'none' to filter instances with no remediation jobs or instances where the most recent job is stale.
  Note: Stale jobs (created before the instance's affliction_date) are ignored for filtering purposes, but are still included in the 'remediations' array with stale=true.

  - `"none"`

  - `"pending"`

  - `"processing"`

  - `"validating"`

  - `"completed"`

  - `"failed"`

- `search: optional string`

  A search term.

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

- `result: optional array of object { affliction_date, asset, dlp_contexts, 4 more }`

  Array of finding instance objects.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances \
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
      "affliction_date": "2025-03-18T17:25:38.700541Z",
      "asset": {
        "category": {
          "service": null,
          "type": "file",
          "vendor": "Slack",
          "id": "1a78cbf3-b98f-4289-b1f2-22db64130f4f"
        },
        "external_id": "external-file-id-123",
        "fields": [
          {
            "name": "Credential name",
            "value": "Test asset 2",
            "link": "https://example.com"
          }
        ],
        "name": "Public.svg",
        "id": "8a043daf-def4-403e-9d28-da2e93d9b824",
        "link": "https://slack-files.com/TYJH37DCK-E0238GG6B8-92fd5y5674"
      },
      "dlp_contexts": [
        {
          "created": "2025-03-18T17:25:38.695977Z",
          "entry_ids": [
            "21befc68-a297-4090-ac10-17a051b901cd",
            "d6dd1e16-f78c-401a-b564-45c4e44aa467"
          ],
          "profile_id": "ab20a60b-21f2-4b13-ac98-24dcee27ac0e",
          "updated": "2025-03-18T17:25:38.695977Z",
          "id": "7653ff3a-d25e-4c10-8034-3460937c045b",
          "deleted": "2025-03-18T17:25:38.695977Z",
          "match_context_max_extent": 512,
          "match_context_min_extent": 1,
          "match_context_payload": {}
        }
      ],
      "remediations": [
        {
          "id": "123e4567-e89b-12d3-a456-426614174000",
          "created_at": "2025-03-18T18:30:15.123456Z",
          "stale": false,
          "status": "pending"
        }
      ],
      "webhooks": [
        {
          "latest_job": {
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "created_at": "2025-03-18T18:30:15.123456Z",
            "stale": false,
            "status": "pending"
          },
          "webhook_id": "550e8400-e29b-41d4-a716-446655440000",
          "webhook_label": "Send to Gmail"
        }
      ],
      "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
      "is_archived": false
    }
  ]
}
```

## Get a finding instance using an instance ID

**get** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances/{instance_id}`

Gets a security Finding instance by id.

### Path Parameters

- `account_id: string`

- `finding_id: string`

- `instance_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances/$INSTANCE_ID \
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
  "success": true,
  "result": {
    "affliction_date": "2025-03-18T17:25:38.700541Z",
    "asset": {
      "category": {
        "service": null,
        "type": "file",
        "vendor": "Slack",
        "id": "1a78cbf3-b98f-4289-b1f2-22db64130f4f"
      },
      "external_id": "external-file-id-123",
      "fields": [
        {
          "name": "Credential name",
          "value": "Test asset 2",
          "link": "https://example.com"
        }
      ],
      "name": "Public.svg",
      "id": "8a043daf-def4-403e-9d28-da2e93d9b824",
      "link": "https://slack-files.com/TYJH37DCK-E0238GG6B8-92fd5y5674"
    },
    "dlp_contexts": [
      {
        "created": "2025-03-18T17:25:38.695977Z",
        "entry_ids": [
          "21befc68-a297-4090-ac10-17a051b901cd",
          "d6dd1e16-f78c-401a-b564-45c4e44aa467"
        ],
        "profile_id": "ab20a60b-21f2-4b13-ac98-24dcee27ac0e",
        "updated": "2025-03-18T17:25:38.695977Z",
        "id": "7653ff3a-d25e-4c10-8034-3460937c045b",
        "deleted": "2025-03-18T17:25:38.695977Z",
        "match_context_max_extent": 512,
        "match_context_min_extent": 1,
        "match_context_payload": {}
      }
    ],
    "remediations": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "created_at": "2025-03-18T18:30:15.123456Z",
        "stale": false,
        "status": "pending"
      }
    ],
    "webhooks": [
      {
        "latest_job": {
          "id": "123e4567-e89b-12d3-a456-426614174000",
          "created_at": "2025-03-18T18:30:15.123456Z",
          "stale": false,
          "status": "pending"
        },
        "webhook_id": "550e8400-e29b-41d4-a716-446655440000",
        "webhook_label": "Send to Gmail"
      }
    ],
    "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
    "is_archived": false
  }
}
```

## Create a finding instances export

**post** `/accounts/{account_id}/data-security/posture/findings/{storage_namespace_id}/instances/export`

Creates a CSV export for Finding instances and accepts optional filters in the payload.

The `storage_namespace_id` path parameter is derived from the finding ID by base64-decoding it
(which yields `integration_id:finding_type_id`) and replacing the colon with a hyphen.

### Path Parameters

- `account_id: string`

- `storage_namespace_id: string`

### Body Parameters

- `archived: optional boolean`

  Filter for archived status.

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `orders: optional array of object { direction, name }`

  Ordering specifications for the export.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "asset.name" or "affliction_date"`

    Which field to use when ordering the finding instances.

    - `"asset.name"`

    - `"affliction_date"`

- `search: optional string`

  A search term.

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$STORAGE_NAMESPACE_ID/instances/export \
    -X POST \
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
  "success": true,
  "result": {
    "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
    "status": "Success",
    "type": "finding",
    "user_id": "e7712d506b1ee4c5ede0802815f55a75",
    "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
    "errors": null,
    "file_name": "findings_export_2024-02-27.csv",
    "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
  }
}
```

## Archive a finding

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances/archive`

Archive one or more finding instances.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Body Parameters

- `check_instances: array of string`

  A list of finding instance IDs to pass along.

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances/archive \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "check_instances": [
            "497f6eca-6276-4993-bfeb-53cbbbba6f08"
          ]
        }'
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
  "success": true,
  "result": {
    "affliction_date": "2025-03-18T17:25:38.700541Z",
    "asset": {
      "category": {
        "service": null,
        "type": "file",
        "vendor": "Slack",
        "id": "1a78cbf3-b98f-4289-b1f2-22db64130f4f"
      },
      "external_id": "external-file-id-123",
      "fields": [
        {
          "name": "Credential name",
          "value": "Test asset 2",
          "link": "https://example.com"
        }
      ],
      "name": "Public.svg",
      "id": "8a043daf-def4-403e-9d28-da2e93d9b824",
      "link": "https://slack-files.com/TYJH37DCK-E0238GG6B8-92fd5y5674"
    },
    "dlp_contexts": [
      {
        "created": "2025-03-18T17:25:38.695977Z",
        "entry_ids": [
          "21befc68-a297-4090-ac10-17a051b901cd",
          "d6dd1e16-f78c-401a-b564-45c4e44aa467"
        ],
        "profile_id": "ab20a60b-21f2-4b13-ac98-24dcee27ac0e",
        "updated": "2025-03-18T17:25:38.695977Z",
        "id": "7653ff3a-d25e-4c10-8034-3460937c045b",
        "deleted": "2025-03-18T17:25:38.695977Z",
        "match_context_max_extent": 512,
        "match_context_min_extent": 1,
        "match_context_payload": {}
      }
    ],
    "remediations": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "created_at": "2025-03-18T18:30:15.123456Z",
        "stale": false,
        "status": "pending"
      }
    ],
    "webhooks": [
      {
        "latest_job": {
          "id": "123e4567-e89b-12d3-a456-426614174000",
          "created_at": "2025-03-18T18:30:15.123456Z",
          "stale": false,
          "status": "pending"
        },
        "webhook_id": "550e8400-e29b-41d4-a716-446655440000",
        "webhook_label": "Send to Gmail"
      }
    ],
    "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
    "is_archived": false
  }
}
```

## Remove the archive marking from a finding instance

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances/unarchive`

Remove the archive marking from one or more finding instances.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Body Parameters

- `check_instances: array of string`

  A list of finding instance IDs to pass along.

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances/unarchive \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "check_instances": [
            "497f6eca-6276-4993-bfeb-53cbbbba6f08"
          ]
        }'
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
  "success": true,
  "result": {
    "affliction_date": "2025-03-18T17:25:38.700541Z",
    "asset": {
      "category": {
        "service": null,
        "type": "file",
        "vendor": "Slack",
        "id": "1a78cbf3-b98f-4289-b1f2-22db64130f4f"
      },
      "external_id": "external-file-id-123",
      "fields": [
        {
          "name": "Credential name",
          "value": "Test asset 2",
          "link": "https://example.com"
        }
      ],
      "name": "Public.svg",
      "id": "8a043daf-def4-403e-9d28-da2e93d9b824",
      "link": "https://slack-files.com/TYJH37DCK-E0238GG6B8-92fd5y5674"
    },
    "dlp_contexts": [
      {
        "created": "2025-03-18T17:25:38.695977Z",
        "entry_ids": [
          "21befc68-a297-4090-ac10-17a051b901cd",
          "d6dd1e16-f78c-401a-b564-45c4e44aa467"
        ],
        "profile_id": "ab20a60b-21f2-4b13-ac98-24dcee27ac0e",
        "updated": "2025-03-18T17:25:38.695977Z",
        "id": "7653ff3a-d25e-4c10-8034-3460937c045b",
        "deleted": "2025-03-18T17:25:38.695977Z",
        "match_context_max_extent": 512,
        "match_context_min_extent": 1,
        "match_context_payload": {}
      }
    ],
    "remediations": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "created_at": "2025-03-18T18:30:15.123456Z",
        "stale": false,
        "status": "pending"
      }
    ],
    "webhooks": [
      {
        "latest_job": {
          "id": "123e4567-e89b-12d3-a456-426614174000",
          "created_at": "2025-03-18T18:30:15.123456Z",
          "stale": false,
          "status": "pending"
        },
        "webhook_id": "550e8400-e29b-41d4-a716-446655440000",
        "webhook_label": "Send to Gmail"
      }
    ],
    "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
    "is_archived": false
  }
}
```

## Domain Types

### Instance List Response

- `InstanceListResponse object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Instance Get Response

- `InstanceGetResponse object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Instance Export Response

- `InstanceExportResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Instance Archive Response

- `InstanceArchiveResponse object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Instance Unarchive Response

- `InstanceUnarchiveResponse object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

# Exports

## List all export jobs

**get** `/accounts/{account_id}/data-security/posture/exports`

List all export jobs for a given requestor's organization

### Path Parameters

- `account_id: string`

### Query Parameters

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `status: optional "Pending" or "Success" or "Failure" or 2 more`

  Filter on export job's status

  - `"Pending"`

  - `"Success"`

  - `"Failure"`

  - `"Rescheduled"`

  - `"In-Progress"`

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

- `result: optional array of object { id, status, type, 5 more }`

  Array of export job objects.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/exports \
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
      "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
      "status": "Success",
      "type": "finding",
      "user_id": "e7712d506b1ee4c5ede0802815f55a75",
      "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
      "errors": null,
      "file_name": "findings_export_2024-02-27.csv",
      "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
    }
  ]
}
```

## Get a single export job

**get** `/accounts/{account_id}/data-security/posture/exports/{id}`

Retrieves a single export job by its unique identifier

### Path Parameters

- `account_id: string`

- `id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/exports/$ID \
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
  "success": true,
  "result": {
    "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
    "status": "Success",
    "type": "finding",
    "user_id": "e7712d506b1ee4c5ede0802815f55a75",
    "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
    "errors": null,
    "file_name": "findings_export_2024-02-27.csv",
    "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
  }
}
```

## Domain Types

### Export List Response

- `ExportListResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Export Get Response

- `ExportGetResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

# Finding Types

## List all finding types

**get** `/accounts/{account_id}/data-security/posture/finding_types`

List all available finding types with pagination support.

### Path Parameters

- `account_id: string`

### Query Parameters

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `vendors: optional array of "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter finding types by vendor. Supports multiple comma-separated values.

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

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `result: optional array of object { id, category, name, 2 more }`

  Array of finding type objects.

  - `id: string`

    The unique identifier of the finding.

  - `category: object { observation, product, type }`

    Category information for a finding.

    - `observation: "Issue" or "Insight" or "Activity"`

      The type of the observation.

      - `"Issue"`

      - `"Insight"`

      - `"Activity"`

    - `product: "SaaS" or "Cloud"`

      The product category.

      - `"SaaS"`

      - `"Cloud"`

    - `type: "Content" or "Posture"`

      The type of the finding category.

      - `"Content"`

      - `"Posture"`

  - `name: string`

    The name of the finding.

  - `severity: "Critical" or "High" or "Medium" or "Low"`

    The severity level of a finding.

    - `"Critical"`

    - `"High"`

    - `"Medium"`

    - `"Low"`

  - `vendor: string`

    The SaaS/Cloud vendor of the platform with which the finding is associated.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/finding_types \
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
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace"
    }
  ]
}
```

## Get finding by ID

**get** `/accounts/{account_id}/data-security/posture/finding_types/{finding_type_id}`

Retrieve a specific finding type by its unique identifier.

### Path Parameters

- `account_id: string`

- `finding_type_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, category, name, 2 more }`

  Basic finding type information.

  - `id: string`

    The unique identifier of the finding.

  - `category: object { observation, product, type }`

    Category information for a finding.

    - `observation: "Issue" or "Insight" or "Activity"`

      The type of the observation.

      - `"Issue"`

      - `"Insight"`

      - `"Activity"`

    - `product: "SaaS" or "Cloud"`

      The product category.

      - `"SaaS"`

      - `"Cloud"`

    - `type: "Content" or "Posture"`

      The type of the finding category.

      - `"Content"`

      - `"Posture"`

  - `name: string`

    The name of the finding.

  - `severity: "Critical" or "High" or "Medium" or "Low"`

    The severity level of a finding.

    - `"Critical"`

    - `"High"`

    - `"Medium"`

    - `"Low"`

  - `vendor: string`

    The SaaS/Cloud vendor of the platform with which the finding is associated.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/finding_types/$FINDING_TYPE_ID \
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
  "success": true,
  "result": {
    "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
    "category": {
      "observation": "Issue",
      "product": "SaaS",
      "type": "Posture"
    },
    "name": "Slack File Publicly Accessible",
    "severity": "High",
    "vendor": "Google Workspace"
  }
}
```

## Domain Types

### Finding Type List Response

- `FindingTypeListResponse object { id, category, name, 2 more }`

  Basic finding type information.

  - `id: string`

    The unique identifier of the finding.

  - `category: object { observation, product, type }`

    Category information for a finding.

    - `observation: "Issue" or "Insight" or "Activity"`

      The type of the observation.

      - `"Issue"`

      - `"Insight"`

      - `"Activity"`

    - `product: "SaaS" or "Cloud"`

      The product category.

      - `"SaaS"`

      - `"Cloud"`

    - `type: "Content" or "Posture"`

      The type of the finding category.

      - `"Content"`

      - `"Posture"`

  - `name: string`

    The name of the finding.

  - `severity: "Critical" or "High" or "Medium" or "Low"`

    The severity level of a finding.

    - `"Critical"`

    - `"High"`

    - `"Medium"`

    - `"Low"`

  - `vendor: string`

    The SaaS/Cloud vendor of the platform with which the finding is associated.

### Finding Type Get Response

- `FindingTypeGetResponse object { id, category, name, 2 more }`

  Basic finding type information.

  - `id: string`

    The unique identifier of the finding.

  - `category: object { observation, product, type }`

    Category information for a finding.

    - `observation: "Issue" or "Insight" or "Activity"`

      The type of the observation.

      - `"Issue"`

      - `"Insight"`

      - `"Activity"`

    - `product: "SaaS" or "Cloud"`

      The product category.

      - `"SaaS"`

      - `"Cloud"`

    - `type: "Content" or "Posture"`

      The type of the finding category.

      - `"Content"`

      - `"Posture"`

  - `name: string`

    The name of the finding.

  - `severity: "Critical" or "High" or "Medium" or "Low"`

    The severity level of a finding.

    - `"Critical"`

    - `"High"`

    - `"Medium"`

    - `"Low"`

  - `vendor: string`

    The SaaS/Cloud vendor of the platform with which the finding is associated.

# Remediation Types

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

## Domain Types

### Remediation Type List Response

- `RemediationTypeListResponse object { id, description, display_name, 2 more }`

  Information about a remediation type.

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

# Content

## List DLP content findings

**get** `/accounts/{account_id}/data-security/posture/content`

List DLP content findings

### Path Parameters

- `account_id: string`

### Query Parameters

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `dlp_profile_id: optional string`

  Filter by an DLP profile ID

- `integration_id: optional string`

  Filter by an integration ID

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `order: optional "asset_name" or "dlp_profile_count" or "integration_name" or "latest_affliction_date"`

  Which field to use when ordering content assets.

  - `"asset_name"`

  - `"dlp_profile_count"`

  - `"integration_name"`

  - `"latest_affliction_date"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `search: optional string`

  A search term.

- `vendor: optional "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter by vendor

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

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `result: optional array of object { asset_id, asset_name, dlp_contexts, 4 more }`

  Array of content asset objects.

  - `asset_id: string`

    Unique identifier for the asset.

  - `asset_name: string`

    Name of the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information for this asset.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `dlp_profile_count: number`

    Number of DLP profiles that flagged this asset.

  - `dlp_profile_ids: array of string`

    IDs of DLP profiles that flagged this asset.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Most recent date this asset was flagged.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/content \
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
      "asset_id": "e6910838-4b91-45e9-b2b4-91bb23cb9762",
      "asset_name": "Test Asset Name",
      "dlp_contexts": [
        {
          "created": "2025-03-18T17:25:38.695977Z",
          "entry_ids": [
            "21befc68-a297-4090-ac10-17a051b901cd",
            "d6dd1e16-f78c-401a-b564-45c4e44aa467"
          ],
          "profile_id": "ab20a60b-21f2-4b13-ac98-24dcee27ac0e",
          "updated": "2025-03-18T17:25:38.695977Z",
          "id": "7653ff3a-d25e-4c10-8034-3460937c045b",
          "deleted": "2025-03-18T17:25:38.695977Z",
          "match_context_max_extent": 512,
          "match_context_min_extent": 1,
          "match_context_payload": {}
        }
      ],
      "dlp_profile_count": 2,
      "dlp_profile_ids": [
        "c12f2059-8df4-43f8-9eb9-d27112d92b63",
        "822c051b-0bb4-4747-8929-471a1d506eef"
      ],
      "integration": {
        "created": "2021-08-10T20:16:11.851451Z",
        "last_hydrated": "2025-03-18T17:25:38.697894Z",
        "name": "Example integration",
        "permissions": [
          "GroupMember.Read.All",
          "Group.Read.All"
        ],
        "policy": {
          "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
          "client_id": "client_id",
          "compliance_level": "standard",
          "dlp_enabled": true,
          "link": "https://example.com",
          "name": "Google Workspace Standard Policy",
          "permissions": [
            "https://www.googleapis.com/auth/admin.directory.domain.readonly",
            "https://www.googleapis.com/auth/admin.directory.user.readonly"
          ]
        },
        "status": "Healthy",
        "updated": "2021-08-10T20:16:11.851451Z",
        "upgradable": false,
        "vendor": {
          "id": "R09PR0xFX1dPUktTUEFDRQ==",
          "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
          "display_name": "Google Workspace",
          "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
          "name": "GOOGLE_WORKSPACE",
          "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
          "zt_enrollments": [
            "casb"
          ],
          "policies": [
            {
              "foo": "bar"
            }
          ]
        },
        "zt_enrollments": [
          {
            "id": "casb",
            "description": "example",
            "display_name": "Cloud Access Security Broker",
            "enabled": true
          }
        ],
        "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
        "credential_health_status": "Healthy",
        "credentials_expiry": "2025-03-18T17:25:38.697902Z",
        "is_paused": false,
        "upgrade_dismissed": false
      },
      "latest_affliction_date": "2024-10-18T19:53:57.626659Z"
    }
  ]
}
```

## Create a content export

**post** `/accounts/{account_id}/data-security/posture/content/export`

Creates a CSV export for content and accepts optional filters in the payload.

### Path Parameters

- `account_id: string`

### Body Parameters

- `dlp_profile_information: array of object { id, entries, name }`

  DLP profile metadata for the export.

  - `id: string`

    Unique identifier for the DLP profile.

  - `entries: array of object { id, name, profile_id }`

    Entries contained within this DLP profile.

    - `id: string`

      Unique identifier for the DLP profile entry.

    - `name: string`

      Name of the DLP profile entry.

    - `profile_id: string`

      ID of the parent DLP profile.

  - `name: string`

    Name of the DLP profile.

- `dlp_profile_id: optional array of string`

  Filter by DLP profile IDs.

- `integration_id: optional array of string`

  Filter by integration IDs.

- `max_affliction_date: optional string`

  Filter to view content flagged on or before this date.

- `min_affliction_date: optional string`

  Filter to view content flagged on or after this date.

- `orders: optional array of object { direction, name }`

  Ordering specifications for the export.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "asset_name" or "dlp_profile_count" or "integration_name" or "latest_affliction_date"`

    Content-specific field names for ordering.

    - `"asset_name"`

    - `"dlp_profile_count"`

    - `"integration_name"`

    - `"latest_affliction_date"`

- `search: optional string`

  Search term to filter content.

- `vendors: optional array of "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter by vendor types.

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

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/content/export \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "dlp_profile_information": [
            {
              "id": "e91a2360-da51-4fdf-9711-bcdecd462614",
              "entries": [
                {
                  "id": "55ba2c6c-8ef4-4b2e-9148-e75e8b6ccac1",
                  "name": "Credit Card Numbers",
                  "profile_id": "e91a2360-da51-4fdf-9711-bcdecd462614"
                }
              ],
              "name": "Financial Information"
            }
          ],
          "dlp_profile_id": [
            "e91a2360-da51-4fdf-9711-bcdecd462614"
          ],
          "integration_id": [
            "c416bc38-75dc-425f-ae25-c37b5df5c37f"
          ],
          "max_affliction_date": "2024-01-01T00:00:00Z",
          "min_affliction_date": "2023-01-01T00:00:00Z",
          "search": "sensitive",
          "vendors": [
            "GOOGLE_WORKSPACE"
          ]
        }'
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
  "success": true,
  "result": {
    "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
    "status": "Success",
    "type": "finding",
    "user_id": "e7712d506b1ee4c5ede0802815f55a75",
    "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
    "errors": null,
    "file_name": "findings_export_2024-02-27.csv",
    "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
  }
}
```

## Domain Types

### Content List Response

- `ContentListResponse object { asset_id, asset_name, dlp_contexts, 4 more }`

  Content asset with DLP information.

  - `asset_id: string`

    Unique identifier for the asset.

  - `asset_name: string`

    Name of the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information for this asset.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `dlp_profile_count: number`

    Number of DLP profiles that flagged this asset.

  - `dlp_profile_ids: array of string`

    IDs of DLP profiles that flagged this asset.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Most recent date this asset was flagged.

### Content Export Response

- `ContentExportResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

# Remediations

# Jobs

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

## Creates remediation jobs

**post** `/accounts/{account_id}/data-security/posture/remediations/jobs`

Create one or more remediation jobs tied to a specific Cloudflare Account.

### Path Parameters

- `account_id: string`

### Body Parameters

- `finding_instance_ids: array of string`

  UUIDs identifying Finding Instances.

- `remediation_type_id: string`

  A UUID identifying this Remediation Type.

### Returns

- `errors: array of unknown`

  Array of error messages.

- `messages: array of unknown`

  Array of informational messages.

- `result: object { created, failed }`

  - `created: array of object { id, asset, created_at, 11 more }`

    Successfully created remediation jobs.

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

  - `failed: array of object { error, finding_instance_id }`

    Failed remediation job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a remediation job.

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/remediations/jobs \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "finding_instance_ids": [
            "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e"
          ],
          "remediation_type_id": "5a7d9e2f-1b3c-4d5e-8f6a-7b8c9d0e1f2a"
        }'
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
  "result": {
    "created": [
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
    "failed": [
      {
        "error": "Failed to create remediation job",
        "finding_instance_id": "2e6b4c8a-9d1f-4e3b-8c7a-5f9e2d1a6b4c"
      }
    ]
  },
  "success": true
}
```

## Create a remediation jobs export

**post** `/accounts/{account_id}/data-security/posture/remediations/jobs/export`

Creates a CSV export for remediation jobs and accepts optional filters in the payload.

### Path Parameters

- `account_id: string`

### Body Parameters

- `integration_id: optional array of string`

  Filter by multiple integration IDs.

- `max_updated_at: optional string`

  Filter to view remediation jobs updated on or before this datetime. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_updated_at: optional string`

  Filter to view remediation jobs updated on or after this datetime. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `orders: optional array of object { direction, name }`

  Ordering specifications for the export.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "asset_name" or "finding_type_name" or "integration_name" or 3 more`

    Which field to use when ordering the remediation jobs.

    - `"asset_name"`

    - `"finding_type_name"`

    - `"integration_name"`

    - `"status"`

    - `"last_updated_at"`

    - `"affliction_date"`

- `search: optional string`

  A search term.

- `status: optional array of "pending" or "processing" or "completed" or 2 more`

  Filter by remediation job status.

  - `"pending"`

  - `"processing"`

  - `"completed"`

  - `"failed"`

  - `"validating"`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/remediations/jobs/export \
    -X POST \
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
  "success": true,
  "result": {
    "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
    "status": "Success",
    "type": "finding",
    "user_id": "e7712d506b1ee4c5ede0802815f55a75",
    "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
    "errors": null,
    "file_name": "findings_export_2024-02-27.csv",
    "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
  }
}
```

## Domain Types

### Job List Response

- `JobListResponse object { id, asset, created_at, 11 more }`

  Information about a remediation job.

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

### Job Create Response

- `JobCreateResponse object { created, failed }`

  - `created: array of object { id, asset, created_at, 11 more }`

    Successfully created remediation jobs.

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

  - `failed: array of object { error, finding_instance_id }`

    Failed remediation job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a remediation job.

### Job Export Response

- `JobExportResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

# Webhooks

## List webhook configurations

**get** `/accounts/{account_id}/data-security/posture/webhooks`

Retrieves all webhook configurations for the authenticated account.
Returns an array of webhook configurations that can be used to send finding notifications.

### Path Parameters

- `account_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional array of object { id, authentication_type, created_at, 6 more }`

  List of webhook configurations.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks \
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
  "success": true,
  "result": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "authentication_type": "Bearer Auth",
      "created_at": "2024-01-15T10:30:00Z",
      "destination_url": "https://example.com/webhook",
      "label": "Send to Gmail",
      "status": "enabled",
      "updated_at": "2024-01-20T14:45:00Z",
      "version": 1,
      "headers": [
        {
          "key": "authorization"
        }
      ]
    }
  ]
}
```

## Create a new webhook configuration

**post** `/accounts/{account_id}/data-security/posture/webhooks`

Creates a new webhook configuration for sending finding notifications to external endpoints.

### Path Parameters

- `account_id: string`

### Body Parameters

- `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

  Type of authentication used for the webhook.

  - `"Basic Auth"`

  - `"None"`

  - `"Bearer Auth"`

  - `"Static Headers"`

  - `"HMAC-Signing"`

- `destination_url: string`

  Target URL for the webhook configuration. Where resulting data will be sent.

- `label: string`

  Account-specified display label for the webhook configuration.

- `headers: optional array of object { key, value }`

  List of custom headers to include in webhook requests.

  - `key: string`

    Header key name.

  - `value: optional string`

    Header value. Required on Create and Evaluate. On Update, omit or set to null to keep existing value.

- `signing_secret: optional string`

  Secret key used for HMAC signing when authentication_type is "HMAC-Signing".

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "authentication_type": "Bearer Auth",
          "destination_url": "https://example.com/webhook",
          "label": "Send to Slack",
          "headers": [
            {
              "key": "Authorization",
              "value": "Bearer token123"
            },
            {
              "key": "X-Custom-Header",
              "value": "value"
            }
          ],
          "signing_secret": "my-secret-key"
        }'
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
  "success": true,
  "result": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "authentication_type": "Bearer Auth",
    "created_at": "2024-01-15T10:30:00Z",
    "destination_url": "https://example.com/webhook",
    "label": "Send to Gmail",
    "status": "enabled",
    "updated_at": "2024-01-20T14:45:00Z",
    "version": 1,
    "headers": [
      {
        "key": "authorization"
      }
    ]
  }
}
```

## Get webhook configuration by ID

**get** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}`

Retrieves a specific webhook configuration by its unique identifier.

### Path Parameters

- `account_id: string`

- `webhook_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID \
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
  "success": true,
  "result": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "authentication_type": "Bearer Auth",
    "created_at": "2024-01-15T10:30:00Z",
    "destination_url": "https://example.com/webhook",
    "label": "Send to Gmail",
    "status": "enabled",
    "updated_at": "2024-01-20T14:45:00Z",
    "version": 1,
    "headers": [
      {
        "key": "authorization"
      }
    ]
  }
}
```

## Update an existing webhook configuration

**put** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}`

Updates an existing webhook configuration with new settings.

### Path Parameters

- `account_id: string`

- `webhook_id: string`

### Body Parameters

- `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

  Type of authentication used for the webhook.

  - `"Basic Auth"`

  - `"None"`

  - `"Bearer Auth"`

  - `"Static Headers"`

  - `"HMAC-Signing"`

- `destination_url: string`

  Target URL for the webhook configuration. Where resulting data will be sent.

- `label: string`

  Account-specified display label for the webhook configuration.

- `status: "enabled" or "disabled"`

  Status of the webhook configuration.

  - `"enabled"`

  - `"disabled"`

- `headers: optional array of object { key, value }`

  List of custom headers to include in webhook requests.

  - `key: string`

    Header key name.

  - `value: optional string`

    Header value. Required on Create and Evaluate. On Update, omit or set to null to keep existing value.

- `signing_secret: optional string`

  Secret key used for HMAC signing when authentication_type is "HMAC-Signing".

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "authentication_type": "Bearer Auth",
          "destination_url": "https://example.com/webhook",
          "label": "Send to Slack",
          "status": "enabled",
          "headers": [
            {
              "key": "Authorization",
              "value": "Bearer token123"
            },
            {
              "key": "X-Custom-Header",
              "value": "value"
            }
          ],
          "signing_secret": "my-secret-key"
        }'
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
  "success": true,
  "result": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "authentication_type": "Bearer Auth",
    "created_at": "2024-01-15T10:30:00Z",
    "destination_url": "https://example.com/webhook",
    "label": "Send to Gmail",
    "status": "enabled",
    "updated_at": "2024-01-20T14:45:00Z",
    "version": 1,
    "headers": [
      {
        "key": "authorization"
      }
    ]
  }
}
```

## Delete a webhook configuration

**delete** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}`

Soft deletes a webhook configuration by its unique identifier.
The webhook will be marked as deleted and will no longer be available for use.

### Path Parameters

- `account_id: string`

- `webhook_id: string`

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

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID \
    -X DELETE \
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
  "success": true
}
```

## Test a webhook configuration before creating it

**post** `/accounts/{account_id}/data-security/posture/webhooks/evaluate`

Sends a test webhook event to the specified destination URL to verify the webhook endpoint
is reachable and properly configured. This allows customers to validate their webhook
configuration before creating the actual webhook resource.

The test payload includes:

- event_type: "webhook.test"
- timestamp: Current UTC timestamp
- message: Test message indicating this is from Cloudflare CASB
- data: Object with test: true

### Path Parameters

- `account_id: string`

### Body Parameters

- `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

  Type of authentication to use for the test webhook request.

  - `"Basic Auth"`

  - `"None"`

  - `"Bearer Auth"`

  - `"Static Headers"`

  - `"HMAC-Signing"`

- `destination_url: string`

  Target URL to send the test webhook event to.

- `headers: optional array of object { key, value }`

  List of custom headers to include in the test webhook request.

  - `key: string`

    Header key name.

  - `value: optional string`

    Header value. Required on Create and Evaluate. On Update, omit or set to null to keep existing value.

- `signing_secret: optional string`

  Secret key used for HMAC signing when authentication_type is "HMAC-Signing".

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/evaluate \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "authentication_type": "Bearer Auth",
          "destination_url": "https://example.com/webhook",
          "headers": [
            {
              "key": "Authorization",
              "value": "Bearer token123"
            },
            {
              "key": "X-Custom-Header",
              "value": "value"
            }
          ],
          "signing_secret": "my-secret-key"
        }'
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
  "success": true,
  "result": {
    "message": "Webhook test successful",
    "status_code": 200,
    "success": true
  }
}
```

## Test an existing webhook configuration

**post** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}/evaluate`

Sends a test webhook event using an existing webhook configuration.
This allows customers to verify their webhook endpoint is still reachable and properly
configured after creating the webhook resource.

The test payload includes:

- event_type: "webhook.test"
- timestamp: Current UTC timestamp
- message: Test message indicating this is from Cloudflare CASB
- data: Object with test: true

### Path Parameters

- `account_id: string`

- `webhook_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID/evaluate \
    -X POST \
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
  "success": true,
  "result": {
    "message": "Webhook test successful",
    "status_code": 200,
    "success": true
  }
}
```

## Domain Types

### Webhook List Response

- `WebhookListResponse object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Webhook Create Response

- `WebhookCreateResponse object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Webhook Get Response

- `WebhookGetResponse object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Webhook Update Response

- `WebhookUpdateResponse object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Webhook Delete Response

- `WebhookDeleteResponse object { errors, messages, success }`

  Common response structure for all API endpoints.

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

  - `success: boolean`

    Whether the API call was successful.

### Webhook Evaluate Response

- `WebhookEvaluateResponse object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

### Webhook Evaluate Existing Response

- `WebhookEvaluateExistingResponse object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

# Jobs

## Create webhook jobs

**post** `/accounts/{account_id}/data-security/posture/webhooks/jobs`

Creates webhook jobs to send a finding instance to one or more configured webhooks.

### Path Parameters

- `account_id: string`

### Body Parameters

- `finding_instance_ids: array of string`

  Array of finding instance IDs to send to the webhooks.

- `webhook_ids: array of string`

  Array of webhook IDs to trigger jobs for.

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

- `result: object { created, failed }`

  - `created: array of object { id, asset_data, created_at, 9 more }`

    Successfully created webhook jobs.

    - `id: string`

      Unique identifier for the webhook job.

    - `asset_data: map[unknown]`

      Asset data associated with this webhook job.

    - `created_at: string`

      When the webhook job was created.

    - `integration_id: string`

      ID of the integration.

    - `last_updated_at: string`

      When the webhook job was last updated.

    - `parameters: object { finding_instance_id }`

      Parameters for a webhook job.

      - `finding_instance_id: string`

        ID of the finding instance.

    - `status: "pending" or "processing" or "completed" or "failed"`

      Status of a webhook job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

    - `triggered_by_actor: "user" or "account_token"`

      Type of actor that triggered the webhook job.

      - `"user"`

      - `"account_token"`

    - `triggered_by_id: string`

      ID of the actor that triggered the job.

    - `webhook_id: string`

      ID of the webhook configuration.

    - `failure_details: optional map[unknown]`

      Additional details about the failure.

    - `failure_reason: optional "Permission Denied" or "Integration Unavailable" or "Service Temporarily Unavailable" or "System Error"`

      Reason for webhook job failure.

      - `"Permission Denied"`

      - `"Integration Unavailable"`

      - `"Service Temporarily Unavailable"`

      - `"System Error"`

  - `failed: array of object { error, finding_instance_id, webhook_id }`

    Failed webhook job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a webhook job.

    - `webhook_id: string`

      ID of the webhook configuration.

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/jobs \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "finding_instance_ids": [
            "770e8400-e29b-41d4-a716-446655440002",
            "660e8400-e29b-41d4-a716-446655440001"
          ],
          "webhook_ids": [
            "550e8400-e29b-41d4-a716-446655440000",
            "660e8400-e29b-41d4-a716-446655440001"
          ]
        }'
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
  "result": {
    "created": [
      {
        "id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
        "asset_data": {
          "foo": "bar"
        },
        "created_at": "2025-07-07T18:39:13.123456Z",
        "integration_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "last_updated_at": "2025-07-07T18:39:13.123456Z",
        "parameters": {
          "finding_instance_id": "3f7b8c9d-6e5a-4f3b-9c2d-1e0a8b7c6d5e"
        },
        "status": "pending",
        "triggered_by_actor": "user",
        "triggered_by_id": "user@example.com",
        "webhook_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "failure_details": {
          "foo": "bar"
        },
        "failure_reason": "Permission Denied"
      }
    ],
    "failed": [
      {
        "error": "Failed to create webhook job",
        "finding_instance_id": "2e6b4c8a-9d1f-4e3b-8c7a-5f9e2d1a6b4c",
        "webhook_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e"
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Job Create Response

- `JobCreateResponse object { created, failed }`

  - `created: array of object { id, asset_data, created_at, 9 more }`

    Successfully created webhook jobs.

    - `id: string`

      Unique identifier for the webhook job.

    - `asset_data: map[unknown]`

      Asset data associated with this webhook job.

    - `created_at: string`

      When the webhook job was created.

    - `integration_id: string`

      ID of the integration.

    - `last_updated_at: string`

      When the webhook job was last updated.

    - `parameters: object { finding_instance_id }`

      Parameters for a webhook job.

      - `finding_instance_id: string`

        ID of the finding instance.

    - `status: "pending" or "processing" or "completed" or "failed"`

      Status of a webhook job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

    - `triggered_by_actor: "user" or "account_token"`

      Type of actor that triggered the webhook job.

      - `"user"`

      - `"account_token"`

    - `triggered_by_id: string`

      ID of the actor that triggered the job.

    - `webhook_id: string`

      ID of the webhook configuration.

    - `failure_details: optional map[unknown]`

      Additional details about the failure.

    - `failure_reason: optional "Permission Denied" or "Integration Unavailable" or "Service Temporarily Unavailable" or "System Error"`

      Reason for webhook job failure.

      - `"Permission Denied"`

      - `"Integration Unavailable"`

      - `"Service Temporarily Unavailable"`

      - `"System Error"`

  - `failed: array of object { error, finding_instance_id, webhook_id }`

    Failed webhook job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a webhook job.

    - `webhook_id: string`

      ID of the webhook configuration.
