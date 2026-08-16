## Get the current settings for the active account

**get** `/accounts/{account_id}/cni/settings`

Retrieves current settings configuration for the specified resource or service.

### Path Parameters

- `account_id: string`

### Returns

- `default_asn: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/settings \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "default_asn": 0
}
```
