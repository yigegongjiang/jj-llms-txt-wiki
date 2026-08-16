## Delete Workers VPC connectivity service

**delete** `/accounts/{account_id}/connectivity/directory/services/{service_id}`

Removes a single Workers VPC connectivity service by its ID.

### Path Parameters

- `account_id: string`

- `service_id: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/connectivity/directory/services/$SERVICE_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```
