## Update a namespace

**put** `/accounts/{account_id}/ai-search/namespaces/{name}`

Update the description and/or the public endpoint configuration of an existing namespace. The default namespace's description cannot be modified, but its public endpoint can.

### Path Parameters

- `account_id: string`

- `name: string`

### Body Parameters

- `description: optional string`

  Optional description for the namespace. Max 256 characters.

- `public_endpoint_params: optional object { authorized_hosts, chat_completions_endpoint, custom_domains, 6 more }`

  - `authorized_hosts: optional array of string`

  - `chat_completions_endpoint: optional object { disabled }`

    - `disabled: optional boolean`

      Disable chat completions endpoint for this public endpoint

  - `custom_domains: optional array of string`

    Custom domain hostnames that alias this public endpoint. GET and create responses return the current set; on update (PUT) this field is only echoed back when supplied in the request body, otherwise it is null (omit it to leave domains unchanged).

  - `default_domain_enabled: optional boolean`

    When false, the instance is reachable only via a registered custom domain and the default <public_endpoint_id>.search.ai.cloudflare.com host returns 404. Requires at least one custom domain. Defaults to true. public_endpoint_params is replaced wholesale on update, so resend default_domain_enabled on every update to keep the default host off — omitting it resets to true.

  - `enabled: optional boolean`

  - `instances_allowed: optional array of string`

    Instance IDs exposed through the namespace public endpoint. Empty means nothing is searchable. Every ID must be an existing instance in this namespace, and the list cannot exceed the account's multi-instance search limit.

  - `mcp: optional object { description, disabled }`

    - `description: optional string`

    - `disabled: optional boolean`

      Disable MCP endpoint for this public endpoint

  - `rate_limit: optional object { period_ms, requests, technique }`

    - `period_ms: optional number`

    - `requests: optional number`

    - `technique: optional "fixed" or "sliding"`

      - `"fixed"`

      - `"sliding"`

  - `search_endpoint: optional object { disabled }`

    - `disabled: optional boolean`

      Disable search endpoint for this public endpoint

### Returns

- `result: object { created_at, name, description, 2 more }`

  - `created_at: string`

  - `name: string`

  - `description: optional string`

    Optional description for the namespace. Max 256 characters.

  - `public_endpoint_id: optional string`

  - `public_endpoint_params: optional object { authorized_hosts, chat_completions_endpoint, custom_domains, 6 more }`

    - `authorized_hosts: optional array of string`

    - `chat_completions_endpoint: optional object { disabled }`

      - `disabled: optional boolean`

        Disable chat completions endpoint for this public endpoint

    - `custom_domains: optional array of string`

      Custom domain hostnames that alias this public endpoint. GET and create responses return the current set; on update (PUT) this field is only echoed back when supplied in the request body, otherwise it is null (omit it to leave domains unchanged).

    - `default_domain_enabled: optional boolean`

      When false, the instance is reachable only via a registered custom domain and the default <public_endpoint_id>.search.ai.cloudflare.com host returns 404. Requires at least one custom domain. Defaults to true. public_endpoint_params is replaced wholesale on update, so resend default_domain_enabled on every update to keep the default host off — omitting it resets to true.

    - `enabled: optional boolean`

    - `instances_allowed: optional array of string`

      Instance IDs exposed through the namespace public endpoint. Empty means nothing is searchable. Every ID must be an existing instance in this namespace, and the list cannot exceed the account's multi-instance search limit.

    - `mcp: optional object { description, disabled }`

      - `description: optional string`

      - `disabled: optional boolean`

        Disable MCP endpoint for this public endpoint

    - `rate_limit: optional object { period_ms, requests, technique }`

      - `period_ms: optional number`

      - `requests: optional number`

      - `technique: optional "fixed" or "sliding"`

        - `"fixed"`

        - `"sliding"`

    - `search_endpoint: optional object { disabled }`

      - `disabled: optional boolean`

        Disable search endpoint for this public endpoint

- `success: true`

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/ai-search/namespaces/$NAME \
    -X PUT \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "created_at": "2019-12-27T18:11:19.117Z",
    "name": "production",
    "description": "Production environment",
    "public_endpoint_id": "public_endpoint_id",
    "public_endpoint_params": {
      "authorized_hosts": [
        "string"
      ],
      "chat_completions_endpoint": {
        "disabled": true
      },
      "custom_domains": [
        "search.example.com"
      ],
      "default_domain_enabled": true,
      "enabled": true,
      "instances_allowed": [
        "docs",
        "blog"
      ],
      "mcp": {
        "description": "description",
        "disabled": true
      },
      "rate_limit": {
        "period_ms": 60000,
        "requests": 1,
        "technique": "fixed"
      },
      "search_endpoint": {
        "disabled": true
      }
    }
  },
  "success": true
}
```
