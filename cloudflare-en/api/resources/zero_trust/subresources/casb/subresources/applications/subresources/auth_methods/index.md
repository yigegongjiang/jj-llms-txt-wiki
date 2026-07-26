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
