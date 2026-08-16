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
