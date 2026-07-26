## Deletes a tag category (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Deletes a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```
