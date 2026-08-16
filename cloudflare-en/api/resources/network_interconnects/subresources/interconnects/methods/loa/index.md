## Generate the Letter of Authorization (LOA) for a given interconnect

**get** `/accounts/{account_id}/cni/interconnects/{icon}/loa`

Downloads the Letter of Authorization (LOA) for a network interconnect, required for
physical cross-connect provisioning.

### Path Parameters

- `account_id: string`

  Customer account tag

- `icon: string`

### Query Parameters

- `name: optional string`

  Custom name to use in the LOA instead of the account name (200 Character limit)

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects/$ICON/loa \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```
