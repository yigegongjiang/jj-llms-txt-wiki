## Delete an interconnect object

**delete** `/accounts/{account_id}/cni/interconnects/{icon}`

Permanently removes a network interconnect configuration. The physical or virtual connection
will be terminated.

### Path Parameters

- `account_id: string`

  Customer account tag

- `icon: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects/$ICON \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```
