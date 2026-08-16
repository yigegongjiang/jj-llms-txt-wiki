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
