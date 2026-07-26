# Servers

## List MCP Servers

**get** `/accounts/{account_id}/access/ai-controls/mcp/servers`

Lists all MCP portals configured for the account.

### Path Parameters

- `account_id: string`

### Query Parameters

- `page: optional number`

- `per_page: optional number`

- `search: optional string`

  Search by id, name

### Returns

- `result: array of object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/servers \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": [
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
    }
  ],
  "success": true
}
```

## Create a new MCP Server

**post** `/accounts/{account_id}/access/ai-controls/mcp/servers`

Creates a new MCP portal for managing AI tool access through Cloudflare Access.

### Path Parameters

- `account_id: string`

### Body Parameters

- `id: string`

  server id

- `auth_type: "oauth" or "bearer" or "unauthenticated"`

  - `"oauth"`

  - `"bearer"`

  - `"unauthenticated"`

- `hostname: string`

- `name: string`

- `auth_credentials: optional string`

- `client_secret: optional string`

  Pre-registered OAuth client_secret. Write-only - accepted on create/update when auth_credentials.auth_mode is 'manual'. Stored AES-GCM-encrypted in server_oauth_secrets; never returned by read endpoints.

- `description: optional string`

- `is_shared_oauth_callback_enabled: optional boolean`

  When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

- `secure_web_gateway: optional boolean`

  Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

- `result: object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/servers \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "id": "my-mcp-server",
          "auth_type": "unauthenticated",
          "hostname": "https://example.com/mcp",
          "name": "My MCP Server",
          "description": "This is one remote mcp server"
        }'
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
    "created_at": "2019-12-27T18:11:19.117Z",
    "created_by": "created_by",
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

## Read the details of a MCP Server

**get** `/accounts/{account_id}/access/ai-controls/mcp/servers/{id}`

Retrieves gateway configuration for MCP portals.

### Path Parameters

- `account_id: string`

- `id: string`

  server id

### Returns

- `result: object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/servers/$ID \
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
    "created_at": "2019-12-27T18:11:19.117Z",
    "created_by": "created_by",
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

## Update a MCP Server

**put** `/accounts/{account_id}/access/ai-controls/mcp/servers/{id}`

Updates an MCP portal configuration.

### Path Parameters

- `account_id: string`

- `id: string`

  server id

### Body Parameters

- `auth_credentials: optional string`

- `client_secret: optional string`

  Pre-registered OAuth client_secret. Write-only - accepted on create/update when auth_credentials.auth_mode is 'manual'. Stored AES-GCM-encrypted in server_oauth_secrets; never returned by read endpoints.

- `description: optional string`

- `is_shared_oauth_callback_enabled: optional boolean`

  When true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true. Effective behavior is gated by the gateway worker's per-env rollout mode KV key.

- `name: optional string`

- `secure_web_gateway: optional boolean`

  Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

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

- `result: object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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
    "created_at": "2019-12-27T18:11:19.117Z",
    "created_by": "created_by",
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

## Delete a MCP Server

**delete** `/accounts/{account_id}/access/ai-controls/mcp/servers/{id}`

Deletes an MCP portal from the account.

### Path Parameters

- `account_id: string`

- `id: string`

  server id

### Returns

- `result: object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/servers/$ID \
    -X DELETE \
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
    "created_at": "2019-12-27T18:11:19.117Z",
    "created_by": "created_by",
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

## Sync MCP Server Capabilities

**post** `/accounts/{account_id}/access/ai-controls/mcp/servers/{id}/sync`

Syncs an MCP server's capabilities and returns the updated server state, including any connection errors.

### Path Parameters

- `account_id: string`

- `id: string`

  portal id

### Returns

- `result: object { error, error_details, status }`

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

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/servers/$ID/sync \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "error": "error",
    "error_details": {
      "cause": "cause",
      "is_upstream": true,
      "mcp_code": 0,
      "retryable": true,
      "status_code": 0
    },
    "status": "waiting"
  },
  "success": true
}
```

## Domain Types

### Server List Response

- `ServerListResponse object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

### Server Create Response

- `ServerCreateResponse object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

### Server Read Response

- `ServerReadResponse object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

### Server Update Response

- `ServerUpdateResponse object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

### Server Delete Response

- `ServerDeleteResponse object { id, auth_type, hostname, 18 more }`

  - `id: string`

    server id

  - `auth_type: "oauth" or "bearer" or "unauthenticated"`

    - `"oauth"`

    - `"bearer"`

    - `"unauthenticated"`

  - `hostname: string`

  - `name: string`

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

  - `created_at: optional string`

  - `created_by: optional string`

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

  - `secure_web_gateway: optional boolean`

    Route outbound traffic to this MCP server through Zero Trust Secure Web Gateway

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    Current sync state of the server

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`

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

### Server Sync Response

- `ServerSyncResponse object { error, error_details, status }`

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

  - `status: optional "waiting" or "ready" or "stale" or "error"`

    - `"waiting"`

    - `"ready"`

    - `"stale"`

    - `"error"`
