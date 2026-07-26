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
