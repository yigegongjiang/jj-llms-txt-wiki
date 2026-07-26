# By Dataset

## Lists all target industries for a specific dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/targetIndustries`

List all target industries referenced in events for a specific dataset.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/targetIndustries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### By Dataset List Response

- `ByDatasetListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`
