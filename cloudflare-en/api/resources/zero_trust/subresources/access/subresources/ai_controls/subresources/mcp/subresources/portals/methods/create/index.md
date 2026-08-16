## Create a new MCP Portal

**post** `/accounts/{account_id}/access/ai-controls/mcp/portals`

Creates a new MCP portal for managing AI tool access through Cloudflare Access.

### Path Parameters

- `account_id: string`

### Body Parameters

- `id: string`

  Unique identifier for the MCP portal.

- `hostname: string`

  Hostname where the MCP portal is available.

- `name: string`

  Display name for the MCP portal.

- `allow_code_mode: optional boolean`

  Deprecated: use `code_mode` for new integrations. `true` maps to any non-off Code Mode policy; `false` maps to `code_mode: off`. If both fields are sent, they must be consistent or the request returns a 400.

- `code_mode: optional "off" or "opt_in" or "default_on" or "enforced"`

  Code Mode policy for this portal. `off`: Code Mode is unavailable; query parameters are ignored. `opt_in`: Code Mode is off by default; clients turn it on with `?codemode=search_and_execute`. `default_on`: Code Mode is on by default; clients can opt out with `?codemode=off`. `enforced`: Code Mode is always on; query parameters are ignored. Defaults to `opt_in` when omitted on create. If both `code_mode` and `allow_code_mode` are sent, they must be consistent or the request returns a 400.

  - `"off"`

  - `"opt_in"`

  - `"default_on"`

  - `"enforced"`

- `description: optional string`

  Optional description of the MCP portal.

- `secure_web_gateway: optional boolean`

  Route outbound MCP traffic through Zero Trust Secure Web Gateway.

- `servers: optional array of object { server_id, default_disabled, on_behalf, 2 more }`

  MCP servers attached to the portal and their portal-specific settings.

  - `server_id: string`

    Unique identifier for the MCP server.

  - `default_disabled: optional boolean`

    Disable this server by default for clients connecting through the portal.

  - `on_behalf: optional boolean`

    Use end-user OAuth credentials when connecting this server to the portal.

  - `updated_prompts: optional array of object { name, alias, description, enabled }`

    Portal-specific prompt overrides.

    - `name: string`

      Name of the tool or prompt capability to override.

    - `alias: optional string`

      Custom name exposed for the capability.

    - `description: optional string`

      Custom description exposed for the capability.

    - `enabled: optional boolean`

      Whether the capability is available through the MCP server.

  - `updated_tools: optional array of object { name, alias, description, enabled }`

    Portal-specific tool overrides.

    - `name: string`

      Name of the tool or prompt capability to override.

    - `alias: optional string`

      Custom name exposed for the capability.

    - `description: optional string`

      Custom description exposed for the capability.

    - `enabled: optional boolean`

      Whether the capability is available through the MCP server.

### Returns

- `result: object { id, hostname, name, 9 more }`

  - `id: string`

    Unique identifier for the MCP portal.

  - `hostname: string`

    Hostname where the MCP portal is available.

  - `name: string`

    Display name for the MCP portal.

  - `servers: array of object { id, auth_type, hostname, 22 more }`

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

    - `server_id: string`

      Unique identifier for the MCP server.

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

    - `default_disabled: optional boolean`

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

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway.

    - `status: optional "waiting" or "ready" or "stale" or "error"`

      Current sync state of the server

      - `"waiting"`

      - `"ready"`

      - `"stale"`

      - `"error"`

    - `updated_prompts: optional array of object { name, enabled, portal_alias, 3 more }`

      - `name: string`

      - `enabled: optional boolean`

      - `portal_alias: optional string`

      - `portal_description: optional string`

      - `server_alias: optional string`

      - `server_description: optional string`

    - `updated_tools: optional array of object { name, enabled, portal_alias, 3 more }`

      - `name: string`

      - `enabled: optional boolean`

      - `portal_alias: optional string`

      - `portal_description: optional string`

      - `server_alias: optional string`

      - `server_description: optional string`

  - `allow_code_mode: optional boolean`

    Deprecated: use `code_mode` for new integrations. `true` maps to any non-off Code Mode policy; `false` maps to `code_mode: off`. If both fields are sent, they must be consistent or the request returns a 400.

  - `code_mode: optional "off" or "opt_in" or "default_on" or "enforced"`

    Code Mode policy for this portal. `off`: Code Mode is unavailable; query parameters are ignored. `opt_in`: Code Mode is off by default; clients turn it on with `?codemode=search_and_execute`. `default_on`: Code Mode is on by default; clients can opt out with `?codemode=off`. `enforced`: Code Mode is always on; query parameters are ignored. Defaults to `opt_in` when omitted on create. If both `code_mode` and `allow_code_mode` are sent, they must be consistent or the request returns a 400.

    - `"off"`

    - `"opt_in"`

    - `"default_on"`

    - `"enforced"`

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

    Optional description of the MCP portal.

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "id": "my-mcp-portal",
          "hostname": "example.com",
          "name": "My MCP Portal",
          "allow_code_mode": true,
          "code_mode": "opt_in",
          "description": "This is my custom MCP Portal"
        }'
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-portal",
    "hostname": "example.com",
    "name": "My MCP Portal",
    "servers": [
      {
        "id": "my-mcp-server",
        "auth_type": "unauthenticated",
        "hostname": "https://example.com/mcp",
        "name": "My MCP Server",
        "prompts": [
          {
            "foo": "bar"
          }
        ],
        "server_id": "my-mcp-server",
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
        "default_disabled": true,
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
        "on_behalf": true,
        "secure_web_gateway": false,
        "status": "ready",
        "updated_prompts": [
          {
            "name": "name",
            "enabled": true,
            "portal_alias": "portal-tool-alias",
            "portal_description": "portal-level description",
            "server_alias": "server-tool-alias",
            "server_description": "server-level description"
          }
        ],
        "updated_tools": [
          {
            "name": "name",
            "enabled": true,
            "portal_alias": "portal-tool-alias",
            "portal_description": "portal-level description",
            "server_alias": "server-tool-alias",
            "server_description": "server-level description"
          }
        ]
      }
    ],
    "allow_code_mode": true,
    "code_mode": "opt_in",
    "created_at": "2019-12-27T18:11:19.117Z",
    "created_by": "created_by",
    "description": "This is my custom MCP Portal",
    "modified_at": "2019-12-27T18:11:19.117Z",
    "modified_by": "modified_by",
    "secure_web_gateway": false
  },
  "success": true
}
```
