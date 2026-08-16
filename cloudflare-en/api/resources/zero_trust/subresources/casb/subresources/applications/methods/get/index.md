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
