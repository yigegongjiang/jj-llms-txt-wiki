# Datasets

## Lists all datasets in an account

**get** `/accounts/{account_id}/cloudforce-one/events/dataset`

Lists all threat event datasets configured in Cloudforce One.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `includeDeleted: optional boolean`

  When true, include soft-deleted datasets in the response. Each item includes a `deletedAt` field (ISO 8601 or null). Default: false.

### Returns

- `indicatorWriteMode: "read_only" or "create_only" or "full"`

  Effective indicator mutation capability after account/dataset authorization and dataset storage capability are applied. API Gateway method permissions are separate and must also allow the requested operation.

  - `"read_only"`

  - `"create_only"`

  - `"full"`

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

- `deletedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "indicatorWriteMode": "full",
    "isAnalytics": true,
    "isPublic": true,
    "name": "friendly dataset name",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "deletedAt": "deletedAt"
  }
]
```

## Reads a dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}`

Retrieves details for a specific threat event dataset.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

### Returns

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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

## Updates an existing dataset

**patch** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}`

Partially updates a threat event dataset in Cloudforce One, modifying specific fields without replacing the entire dataset configuration.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID \
    -X PATCH \
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

## Reads raw data for an event by UUID

**get** `/accounts/{account_id}/cloudforce-one/events/raw/{dataset_id}/{event_id}`

Retrieves the raw data associated with an event. Searches across all shards in the dataset.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

- `event_id: string`

  Event ID.

### Returns

- `id: number`

- `accountId: number`

- `created: string`

- `data: string`

- `source: string`

- `tlp: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/raw/$DATASET_ID/$EVENT_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": 1,
  "accountId": 1234,
  "created": "1970-01-01T00:00:00.000Z",
  "data": "{\"foo\": \"bar\"}",
  "source": "https://example.com",
  "tlp": "amber"
}
```

## Domain Types

### Dataset List Response

- `DatasetListResponse = array of object { indicatorWriteMode, isAnalytics, isPublic, 3 more }`

  - `indicatorWriteMode: "read_only" or "create_only" or "full"`

    Effective indicator mutation capability after account/dataset authorization and dataset storage capability are applied. API Gateway method permissions are separate and must also allow the requested operation.

    - `"read_only"`

    - `"create_only"`

    - `"full"`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

  - `deletedAt: optional string`

### Dataset Get Response

- `DatasetGetResponse object { isAnalytics, isPublic, name, uuid }`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

### Dataset Create Response

- `DatasetCreateResponse object { isAnalytics, isPublic, name, uuid }`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

### Dataset Edit Response

- `DatasetEditResponse object { isAnalytics, isPublic, name, uuid }`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

### Dataset Delete Response

- `DatasetDeleteResponse object { name, uuid }`

  - `name: string`

  - `uuid: string`

### Dataset Raw Response

- `DatasetRawResponse object { id, accountId, created, 3 more }`

  - `id: number`

  - `accountId: number`

  - `created: string`

  - `data: string`

  - `source: string`

  - `tlp: string`

# Health

# Events

## Reads an event

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/events/{event_id}`

Retrieves a specific event by its UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

- `event_id: string`

  Event UUID.

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/events/$EVENT_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "attacker": "Flying Yeti",
  "attackerCountry": "CN",
  "attackerCountryAlpha3": "CHN",
  "category": "Domain Resolution",
  "datasetId": "dataset-example-id",
  "date": "2022-04-01T00:00:00Z",
  "event": "An attacker registered the domain domain.com",
  "hasChildren": true,
  "indicator": "domain.com",
  "indicatorType": "domain",
  "indicatorTypeId": 5,
  "killChain": 0,
  "mitreAttack": [
    " "
  ],
  "mitreCapec": [
    " "
  ],
  "numReferenced": 0,
  "numReferences": 0,
  "rawId": "453gw34w3",
  "referenced": [
    " "
  ],
  "referencedIds": [
    0
  ],
  "references": [
    " "
  ],
  "referencesIds": [
    0
  ],
  "tags": [
    "malware"
  ],
  "targetCountry": "US",
  "targetCountryAlpha3": "USA",
  "targetIndustry": "Agriculture",
  "tlp": "amber",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "insight": "insight",
  "releasabilityId": "releasabilityId"
}
```

## Domain Types

### Event Get Response

- `EventGetResponse object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`
