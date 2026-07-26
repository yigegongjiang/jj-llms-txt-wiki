# Portals

## List MCP Portals

**get** `/accounts/{account_id}/access/ai-controls/mcp/portals`

Lists all MCP portals configured for the account.

### Path Parameters

- `account_id: string`

### Query Parameters

- `page: optional number`

- `per_page: optional number`

- `search: optional string`

  Search by id, name, hostname

### Returns

- `result: array of object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": [
    {
      "id": "my-mcp-portal",
      "hostname": "exmaple.com",
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
          "created_at": "2019-12-27T18:11:19.117Z",
          "created_by": "created_by",
          "default_disabled": true,
          "description": "This is one remote mcp server",
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
      "created_at": "2019-12-27T18:11:19.117Z",
      "created_by": "created_by",
      "description": "This is my custom MCP Portal",
      "modified_at": "2019-12-27T18:11:19.117Z",
      "modified_by": "modified_by",
      "secure_web_gateway": false
    }
  ],
  "success": true
}
```

## Create a new MCP Portal

**post** `/accounts/{account_id}/access/ai-controls/mcp/portals`

Creates a new MCP portal for managing AI tool access through Cloudflare Access.

### Path Parameters

- `account_id: string`

### Body Parameters

- `id: string`

  portal id

- `hostname: string`

- `name: string`

- `allow_code_mode: optional boolean`

  Allow remote code execution in Dynamic Workers (beta)

- `description: optional string`

- `secure_web_gateway: optional boolean`

  Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `servers: optional array of object { server_id, default_disabled, on_behalf, 2 more }`

  - `server_id: string`

    server id

  - `default_disabled: optional boolean`

  - `on_behalf: optional boolean`

  - `updated_prompts: optional array of object { name, alias, description, enabled }`

    - `name: string`

    - `alias: optional string`

    - `description: optional string`

    - `enabled: optional boolean`

  - `updated_tools: optional array of object { name, alias, description, enabled }`

    - `name: string`

    - `alias: optional string`

    - `description: optional string`

    - `enabled: optional boolean`

### Returns

- `result: object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "id": "my-mcp-portal",
          "hostname": "exmaple.com",
          "name": "My MCP Portal",
          "allow_code_mode": true,
          "description": "This is my custom MCP Portal"
        }'
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-portal",
    "hostname": "exmaple.com",
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
        "created_at": "2019-12-27T18:11:19.117Z",
        "created_by": "created_by",
        "default_disabled": true,
        "description": "This is one remote mcp server",
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

## Read details of an MCP Portal

**get** `/accounts/{account_id}/access/ai-controls/mcp/portals/{id}`

Read the details of a single MCP Portal, including its configured servers.

### Path Parameters

- `account_id: string`

- `id: string`

  portal id

### Returns

- `result: object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals/$ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-portal",
    "hostname": "exmaple.com",
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
        "created_at": "2019-12-27T18:11:19.117Z",
        "created_by": "created_by",
        "default_disabled": true,
        "description": "This is one remote mcp server",
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

## Update a MCP Portal

**put** `/accounts/{account_id}/access/ai-controls/mcp/portals/{id}`

Updates an MCP portal configuration.

### Path Parameters

- `account_id: string`

- `id: string`

  portal id

### Body Parameters

- `allow_code_mode: optional boolean`

  Allow remote code execution in Dynamic Workers (beta)

- `description: optional string`

- `hostname: optional string`

- `name: optional string`

- `secure_web_gateway: optional boolean`

  Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `servers: optional array of object { server_id, default_disabled, on_behalf, 2 more }`

  - `server_id: string`

    server id

  - `default_disabled: optional boolean`

  - `on_behalf: optional boolean`

  - `updated_prompts: optional array of object { name, alias, description, enabled }`

    - `name: string`

    - `alias: optional string`

    - `description: optional string`

    - `enabled: optional boolean`

  - `updated_tools: optional array of object { name, alias, description, enabled }`

    - `name: string`

    - `alias: optional string`

    - `description: optional string`

    - `enabled: optional boolean`

### Returns

- `result: object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals/$ID \
    -X PUT \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-portal",
    "hostname": "exmaple.com",
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
        "created_at": "2019-12-27T18:11:19.117Z",
        "created_by": "created_by",
        "default_disabled": true,
        "description": "This is one remote mcp server",
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

## Delete a MCP Portal

**delete** `/accounts/{account_id}/access/ai-controls/mcp/portals/{id}`

Deletes an MCP portal from the account.

### Path Parameters

- `account_id: string`

- `id: string`

  portal id

### Returns

- `result: object { id, hostname, name, 7 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `allow_code_mode: optional boolean`

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals/$ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-portal",
    "hostname": "exmaple.com",
    "name": "My MCP Portal",
    "allow_code_mode": true,
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

## Domain Types

### Portal List Response

- `PortalListResponse object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

### Portal Create Response

- `PortalCreateResponse object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

### Portal Read Response

- `PortalReadResponse object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

### Portal Update Response

- `PortalUpdateResponse object { id, hostname, name, 8 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `servers: array of object { id, auth_type, hostname, 21 more }`

    - `id: string`

      server id

    - `auth_type: "oauth" or "bearer" or "unauthenticated"`

      - `"oauth"`

      - `"bearer"`

      - `"unauthenticated"`

    - `hostname: string`

    - `name: string`

    - `prompts: array of map[unknown]`

    - `server_id: string`

      server id

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

    - `created_at: optional string`

    - `created_by: optional string`

    - `default_disabled: optional boolean`

    - `description: optional string`

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

      When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

    - `last_successful_sync: optional string`

    - `last_synced: optional string`

    - `modified_at: optional string`

    - `modified_by: optional string`

    - `on_behalf: optional boolean`

    - `secure_web_gateway: optional boolean`

      Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway

### Portal Delete Response

- `PortalDeleteResponse object { id, hostname, name, 7 more }`

  - `id: string`

    portal id

  - `hostname: string`

  - `name: string`

  - `allow_code_mode: optional boolean`

    Allow remote code execution in Dynamic Workers (beta)

  - `created_at: optional string`

  - `created_by: optional string`

  - `description: optional string`

  - `modified_at: optional string`

  - `modified_by: optional string`

  - `secure_web_gateway: optional boolean`

    Route outbound MCP traffic through Zero Trust Secure Web Gateway
