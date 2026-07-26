## Deletes a tag (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}`

Deletes a Source-of-Truth tag by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```
