## Create deployment tail

**post** `/accounts/{account_id}/pages/projects/{project_name}/deployments/{deployment_id}/tails`

Start a tail that receives logs and exception data.

### Path Parameters

- `account_id: string`

  Identifier.

- `project_name: string`

  Name of the project.

- `deployment_id: string`

  Identifier.

### Body Parameters

- `filters: optional array of map[unknown]`

  Filters to apply to the tail session.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `result: object { id, url }`

  A tail session for streaming logs from a Pages deployment.

  - `id: string`

    Identifier of the tail session.

  - `url: optional string`

    Optional WebSocket URL to connect to for receiving tail events, when returned by the tail service.

- `success: true`

  Whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/pages/projects/$PROJECT_NAME/deployments/$DEPLOYMENT_ID/tails \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{}'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "result": {
    "id": "49a4dcf81a3940fab8453b2be3fb86ef",
    "url": "wss://tail.developers.workers.dev/49a4dcf81a3940fab8453b2be3fb86ef"
  },
  "success": true
}
```
