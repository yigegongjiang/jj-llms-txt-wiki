## Creates a dataset

**post** `/accounts/{account_id}/cloudforce-one/events/dataset/create`

Creates a new threat event dataset in Cloudforce One for organizing related threat events.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `isPublic: boolean`

  If true, then anyone can search the dataset. If false, then its limited to the account.

- `name: string`

  Used to describe the dataset within the account context.

### Returns

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "isPublic": true,
          "name": "x"
        }'
```

#### Response

```json
{
  "isAnalytics": true,
  "isPublic": true,
  "name": "friendly dataset name",
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```
