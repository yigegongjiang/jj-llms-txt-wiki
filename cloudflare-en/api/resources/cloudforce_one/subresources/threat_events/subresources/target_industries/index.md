# Target Industries

## Lists target industries across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/targetIndustries`

List target industries referenced in events across one or more datasets.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query target industries from. If not provided, uses the default dataset.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/targetIndustries \
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

### Target Industry List Response

- `TargetIndustryListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

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

# Catalog

## Lists all target industries from industry map catalog

**get** `/accounts/{account_id}/cloudforce-one/events/targetIndustries/catalog`

List all predefined target industries from the industry map catalog.

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/targetIndustries/catalog \
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

### Catalog List Response

- `CatalogListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`
