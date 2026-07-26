## Creates a new tag category (SoT)

**post** `/accounts/{account_id}/cloudforce-one/events/tags/categories/create`

Creates a new Source-of-Truth tag category for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `name: string`

- `description: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "name": "Actor"
        }'
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
