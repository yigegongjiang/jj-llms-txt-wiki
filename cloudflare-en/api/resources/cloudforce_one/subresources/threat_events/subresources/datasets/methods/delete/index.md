## Delete a dataset

**delete** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}`

Soft-deletes a dataset given a datasetId.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID to delete

### Returns

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "name": "friendly dataset name",
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```
