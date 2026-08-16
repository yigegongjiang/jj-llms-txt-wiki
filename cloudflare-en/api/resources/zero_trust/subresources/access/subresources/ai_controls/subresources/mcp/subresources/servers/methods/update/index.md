## Update an MCP Server

**put** `/accounts/{account_id}/access/ai-controls/mcp/servers/{id}`

Updates an MCP server's configuration and credentials.

### Path Parameters

- `account_id: string`

- `id: string`

  Unique identifier for the MCP server.

### Body Parameters

- `auth_credentials: optional string`

  Static credential for the upstream MCP server. For auth_type "bearer", either a raw token string (e.g. "sk-abc123"), which is wrapped server-side as `Authorization: Bearer <token>`, or a JSON-encoded object of the form `{"headers":{"Header-Name":"value",...}}` for custom or multiple static headers (e.g. Cloudflare Access service tokens: `{"headers":{"cf-access-client-id":"...","cf-access-client-secret":"..."}}`).

- `client_secret: optional string`

  Pre-registered OAuth client_secret. Write-only - accepted on create/update when auth_credentials.auth_mode is 'manual'. Stored AES-GCM-encrypted in server_oauth_secrets; never returned by read endpoints.

- `description: optional string`

  Optional description of the MCP server.

- `is_shared_oauth_callback_enabled: optional boolean`

  When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true.

- `name: optional string`

  Display name for the MCP server.

- `secure_web_gateway: optional boolean`

  Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway.

- `updated_prompts: optional array of object { name, alias, description, enabled }`

  Server-wide prompt capability overrides.

  - `name: string`

    Name of the tool or prompt capability to override.

  - `alias: optional string`

    Custom name exposed for the capability.

  - `description: optional string`

    Custom description exposed for the capability.

  - `enabled: optional boolean`

    Whether the capability is available through the MCP server.

- `updated_tools: optional array of object { name, alias, description, enabled }`

  Server-wide tool capability overrides.

  - `name: string`

    Name of the tool or prompt capability to override.

  - `alias: optional string`

    Custom name exposed for the capability.

  - `description: optional string`

    Custom description exposed for the capability.

  - `enabled: optional boolean`

    Whether the capability is available through the MCP server.

### Returns

- `result: object { id, auth_type, hostname, 19 more }`

  - `id: string`

    Unique identifier for the MCP server.

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    Authentication method used to connect to the upstream MCP server.

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

    URL of the upstream MCP endpoint.

  - `name: string`

    Display name for the MCP server.

  - `prompts: array of map[unknown]`

  - `tools: array of map[unknown]`

  - `auth_config_summary: optional object { auth_mode, client_secret_version, config, 2 more }`

    Safe subset of auth_credentials surfaced to the dashboard. Includes auth_mode (dcr|manual), has_client_secret, client_secret_version, and the OAuth endpoints + client_id for manual servers. Never includes the secret value.

    - `auth_mode: optional "dcr" or "manual"`

      - `"dcr"`

      - `"manual"`

    - `client_secret_version: optional number`

    - `config: optional object { authorization_endpoint, issuer, resource, 2 more }`

      - `authorization_endpoint: optional string`

      - `issuer: optional string`

      - `resource: optional string`

      - `revocation_endpoint: optional string`

      - `token_endpoint: optional string`

    - `has_client_secret: optional boolean`

    - `registration_info: optional object { client_id, redirect_uris, scope, token_endpoint_auth_method }`

      - `client_id: optional string`

      - `redirect_uris: optional array of string`

      - `scope: optional string`

      - `token_endpoint_auth_method: optional string`

  - `authentication_status: optional "not_required" or "required" or "connected" or 2 more`

    Whether administrative authentication is required before capabilities can be synced. Manual OAuth is user-managed and has no administrative authentication flow.

    - `"not_required"`

    - `"required"`

    - `"connected"`

    - `"stale"`

    - `"manual"`

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

    Optional description of the MCP server.

  - `error: optional string`

  - `error_details: optional object { cause, is_upstream, mcp_code, 2 more }`

    - `cause: optional string`

      Underlying error message

    - `is_upstream: optional boolean`

      True = MCP server returned an error. False = couldn't reach the server

    - `mcp_code: optional number`

      MCP protocol error code

    - `retryable: optional boolean`

      Whether the error is transient and worth retrying

    - `status_code: optional number`

      HTTP status code from the server

  - `is_shared_oauth_callback_enabled: optional boolean`

    When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true.

  - `last_successful_sync: optional string`

  - `last_synced: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway.

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

  - `updated_prompts: optional array of object { name, alias, description, enabled }`

    Server-wide prompt capability overrides.

    - `name: string`

      Name of the tool or prompt capability to override.

    - `alias: optional string`

      Custom name exposed for the capability.

    - `description: optional string`

      Custom description exposed for the capability.

    - `enabled: optional boolean`

      Whether the capability is available through the MCP server.

  - `updated_tools: optional array of object { name, alias, description, enabled }`

    Server-wide tool capability overrides.

    - `name: string`

      Name of the tool or prompt capability to override.

    - `alias: optional string`

      Custom name exposed for the capability.

    - `description: optional string`

      Custom description exposed for the capability.

    - `enabled: optional boolean`

      Whether the capability is available through the MCP server.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/servers/$ID \
    -X PUT \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-server",
    "auth_type": "unauthenticated",
    "hostname": "https://example.com/mcp",
    "name": "My MCP Server",
    "prompts": [
      {
        "foo": "bar"
      }
    ],
    "tools": [
      {
        "foo": "bar"
      }
    ],
    "auth_config_summary": {
      "auth_mode": "dcr",
      "client_secret_version": 0,
      "config": {
        "authorization_endpoint": "authorization_endpoint",
        "issuer": "issuer",
        "resource": "resource",
        "revocation_endpoint": "revocation_endpoint",
        "token_endpoint": "token_endpoint"
      },
      "has_client_secret": true,
      "registration_info": {
        "client_id": "client_id",
        "redirect_uris": [
          "string"
        ],
        "scope": "scope",
        "token_endpoint_auth_method": "token_endpoint_auth_method"
      }
    },
    "authentication_status": "not_required",
    "created_at": "2019-12-27T18:11:19.117Z",
    "created_by": "created_by",
    "description": "This is one remote MCP server",
    "error": "error",
    "error_details": {
      "cause": "cause",
      "is_upstream": true,
      "mcp_code": 0,
      "retryable": true,
      "status_code": 0
    },
    "is_shared_oauth_callback_enabled": true,
    "last_successful_sync": "2019-12-27T18:11:19.117Z",
    "last_synced": "2019-12-27T18:11:19.117Z",
    "modified_at": "2019-12-27T18:11:19.117Z",
    "modified_by": "modified_by",
    "secure_web_gateway": false,
    "status": "ready",
    "updated_prompts": [
      {
        "name": "name",
        "alias": "my-custom-alias",
        "description": "description",
        "enabled": true
      }
    ],
    "updated_tools": [
      {
        "name": "name",
        "alias": "my-custom-alias",
        "description": "description",
        "enabled": true
      }
    ]
  },
  "success": true
}
```
