## List mirrored tags for an indicator dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators/tags`

Returns all mirrored tags from the indicator dataset (DO mirror table). No pagination.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators/tags \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {}
]
```
