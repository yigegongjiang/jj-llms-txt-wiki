## Delete a saved event query

**delete** `/accounts/{account_id}/cloudforce-one/events/queries/{query_id}`

Delete a saved event query by its ID

### Path Parameters

- `account_id: string`

  Account ID.

- `query_id: number`

  Event query ID

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries/$QUERY_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```
