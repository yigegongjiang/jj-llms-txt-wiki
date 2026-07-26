## Updates a tag category (SoT)

**patch** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Updates a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Body Parameters

- `description: optional string`

- `name: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```
