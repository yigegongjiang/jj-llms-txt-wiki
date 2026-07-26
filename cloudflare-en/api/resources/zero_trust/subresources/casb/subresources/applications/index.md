# Applications

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

## Get application details

**get** `/accounts/{account_id}/one/applications/{application_id}`

Returns full application details including auth methods, use cases, and permissions.

### Path Parameters

- `account_id: string`

- `application_id: "ANTHROPIC" or "BITBUCKET" or "BOX" or 10 more`

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

### Returns

- `id: "ANTHROPIC" or "BITBUCKET" or "BOX" or 10 more`

  Vendor identifier.

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

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/applications/$APPLICATION_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": "ANTHROPIC",
  "auth_methods": [
    {
      "id": "id",
      "display_name": "display_name",
      "is_default": true,
      "supported_environments": [
        "string"
      ]
    }
  ],
  "category": "category",
  "description": "description",
  "display_name": "display_name",
  "dlp_enabled": true,
  "instructions": "instructions",
  "logo": "logo",
  "use_cases": [
    {
      "id": "id",
      "base_scopes": [
        {
          "display_name": "display_name",
          "scope": "scope",
          "severity": "low"
        }
      ],
      "description": "description",
      "display_name": "display_name",
      "features": [
        {
          "id": "id",
          "description": "description",
          "display_name": "display_name",
          "scopes": [
            {
              "display_name": "display_name",
              "scope": "scope",
              "severity": "low"
            }
          ]
        }
      ]
    }
  ]
}
```

## Domain Types

### Application List Response

- `ApplicationListResponse = array of object { id, auth_methods, category, 7 more }`

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

### Application Get Response

- `ApplicationGetResponse object { id, auth_methods, category, 6 more }`

  Full application detail for onboarding UI.

  - `id: "ANTHROPIC" or "BITBUCKET" or "BOX" or 10 more`

    Vendor identifier.

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

- `application_id: "ANTHROPIC" or "BITBUCKET" or "BOX" or 10 more`

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

### Returns

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

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/one/applications/$APPLICATION_ID/auth-methods \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "id": "id",
    "display_name": "display_name",
    "human_interaction_required": true,
    "instructions": {
      "markdown": "markdown"
    },
    "payload_example": {
      "foo": "bar"
    },
    "payload_schema": {
      "foo": "bar"
    },
    "redirect_url": "redirect_url"
  }
]
```

## Domain Types

### Auth Method List Response

- `AuthMethodListResponse = array of object { id, display_name, human_interaction_required, 4 more }`

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
