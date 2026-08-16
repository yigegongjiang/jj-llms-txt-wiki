## Delete an MCP Portal

**delete** `/accounts/{account_id}/access/ai-controls/mcp/portals/{id}`

Deletes an MCP portal from the account.

### Path Parameters

- `account_id: string`

- `id: string`

  Unique identifier for the MCP portal.

### Returns

- `result: object { id, hostname, name, 8 more }`

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/access/ai-controls/mcp/portals/$ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "id": "my-mcp-portal",
    "hostname": "example.com",
    "name": "My MCP Portal",
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
