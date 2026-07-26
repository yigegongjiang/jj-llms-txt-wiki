## Lists all tag categories (SoT)

**get** `/accounts/{account_id}/cloudforce-one/events/tags/categories`

Returns all Source-of-Truth tag categories for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `search: optional string`

### Returns

- `categories: array of object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "categories": [
    {
      "name": "Actor",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "createdAt": "createdAt",
      "description": "description",
      "updatedAt": "updatedAt"
    }
  ]
}
```
