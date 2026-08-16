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
