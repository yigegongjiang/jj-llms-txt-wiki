## Create organization

**post** `/organizations`

Create a new organization for a user. (Currently in Public Beta - see https://developers.cloudflare.com/fundamentals/organizations/)

### Body Parameters

- `name: string`

- `parent: optional object { id, name }`

  - `id: string`

  - `name: string`

- `profile: optional AccountProfile`

  - `business_address: string`

  - `business_email: string`

  - `business_name: string`

  - `business_phone: string`

  - `external_metadata: string`

### Returns

- `errors: array of unknown`

- `messages: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `result: Organization`

  References an Organization in the Cloudflare data model.

  - `id: string`

  - `create_time: string`

  - `meta: object { hierarchy_tags, managed_by, tenant_flags }`

    - `hierarchy_tags: optional array of string`

      Ordered chain of organization tags from the root organization down to
      (and including) this organization itself. Root organizations return a
      single-element array containing their own tag; sub-organizations return
      `[rootTag, ...intermediateTags, parentTag, selfTag]`. Useful for
      constructing authorization scopes that need to cover every ancestor
      in the hierarchy.

    - `managed_by: optional string`

    - `tenant_flags: optional object { account_creation, account_deletion, account_migration, 4 more }`

      Enable features for Organizations.

      - `account_creation: string`

      - `account_deletion: string`

      - `account_migration: string`

      - `account_mobility: string`

      - `enterprise_capability: string`

      - `member_management: string`

      - `sub_org_creation: string`

  - `name: string`

  - `parent: optional object { id, name }`

    - `id: string`

    - `name: string`

  - `profile: optional AccountProfile`

    - `business_address: string`

    - `business_email: string`

    - `business_name: string`

    - `business_phone: string`

    - `external_metadata: string`

- `success: true`

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/organizations \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "name": "name"
        }'
```

#### Response

```json
{
  "errors": [],
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
    "id": "a7b9c3d2e8f4g1h5i6j0k9l2m3n7o4p8",
    "create_time": "2019-12-27T18:11:19.117Z",
    "meta": {
      "hierarchy_tags": [
        "string"
      ],
      "managed_by": "managed_by",
      "tenant_flags": {
        "account_creation": "account_creation",
        "account_deletion": "account_deletion",
        "account_migration": "account_migration",
        "account_mobility": "account_mobility",
        "enterprise_capability": "enterprise_capability",
        "member_management": "member_management",
        "sub_org_creation": "sub_org_creation"
      }
    },
    "name": "name",
    "parent": {
      "id": "a7b9c3d2e8f4g1h5i6j0k9l2m3n7o4p8",
      "name": "name"
    },
    "profile": {
      "business_address": "business_address",
      "business_email": "business_email",
      "business_name": "business_name",
      "business_phone": "business_phone",
      "external_metadata": "external_metadata"
    }
  },
  "success": true
}
```
