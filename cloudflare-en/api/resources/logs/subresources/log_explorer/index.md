# Log Explorer

# Query

## Run a log query

**post** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/query/sql`

Run a SQL query against account or zone-level datasets.

Timestamp fields are RFC3339 strings. Filter with:
WHERE {timestamp_field} >= now() - INTERVAL '30' DAY
WHERE {timestamp_field} >= '2026-04-01T00:00:00Z'
WHERE {timestamp_field} BETWEEN '2026-04-01T00:00:00Z' AND '2026-04-30T23:59:59Z'

List configured account or zone datasets to see enabled account or zone-level datasets.
Zone-level datasets will not appear here.
List available account or zone datasets to inspect their schemas and timestamp fields.

For more information about the datasets, and the meaning of each field, check out https://developers.cloudflare.com/logs/logpush/logpush-job/datasets/

### Path Parameters

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

- `success: boolean`

- `result: optional array of map[unknown]`

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/query/sql \
    -H 'Content-Type: text/plain' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -F 'body=@/path/to/body'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": [
    {
      "foo": "bar"
    }
  ]
}
```

## Domain Types

### Query Sql Response

- `QuerySqlResponse = map[unknown]`

# Datasets

## List account or zone datasets

**get** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/datasets`

Returns all Log Explorer datasets configured for the account or zone.

Pass `include_zones=true` to also include zone-level datasets that
belong to this account or zone. List responses omit the `fields` property;
use the single-dataset endpoint to retrieve field configuration.

### Path Parameters

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Query Parameters

- `include_zones: optional boolean`

  Set to true to include zone-scoped datasets belonging to this account.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

- `success: boolean`

- `result: optional array of DatasetSummary`

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/datasets \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": [
    {
      "created_at": "2019-12-27T18:11:19.117Z",
      "dataset": "dataset",
      "dataset_id": "dataset_id",
      "deletion_protection": true,
      "enabled": true,
      "object_id": "object_id",
      "object_type": "account",
      "updated_at": "2019-12-27T18:11:19.117Z"
    }
  ]
}
```

## Get an account or zone dataset

**get** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/datasets/{dataset_id}`

Retrieve a single Log Explorer dataset by ID for the account or zone.

### Path Parameters

- `dataset_id: string`

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

- `success: boolean`

- `result: optional Dataset`

  A Log Explorer dataset summary. List endpoints return this type and omit
  field configuration; use the single-dataset endpoint to retrieve it.

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `fields: array of object { enabled, name }`

    The field configuration for this dataset.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

  - `filter: optional string`

    The Logpush filter predicate applied to this dataset. Omitted
    when no filter is set.

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/datasets/$DATASET_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": {
    "created_at": "2019-12-27T18:11:19.117Z",
    "dataset": "dataset",
    "dataset_id": "dataset_id",
    "deletion_protection": true,
    "enabled": true,
    "fields": [
      {
        "enabled": true,
        "name": "name"
      }
    ],
    "object_id": "object_id",
    "object_type": "account",
    "updated_at": "2019-12-27T18:11:19.117Z",
    "filter": "filter"
  }
}
```

## Create an account or zone dataset

**post** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/datasets`

Create a new Log Explorer dataset for the account or zone.

List available account or zone datasets to see the dataset types and fields you
can use.

The `fields` property is optional. If not specified, all available fields
will be enabled.

For dataset field definitions, see: https://developers.cloudflare.com/logs/logpush/logpush-job/datasets/

### Path Parameters

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Body Parameters

- `dataset: string`

  Dataset type name to create (e.g. `http_requests`).

- `fields: optional array of object { enabled, name }`

  Controls which fields the API ingests. Defaults to all available
  fields when absent.

  - `enabled: boolean`

    Whether the API includes this field in log ingest.

  - `name: string`

    Field name in lowercase.

- `filter: optional string`

  Optional Logpush filter predicate to restrict which events are ingested.
  If provided, replaces the dataset's default filter entirely.
  See [Logpush filters](https://developers.cloudflare.com/logs/reference/filters/)
  for syntax and examples.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

- `success: boolean`

- `result: optional Dataset`

  A Log Explorer dataset summary. List endpoints return this type and omit
  field configuration; use the single-dataset endpoint to retrieve it.

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `fields: array of object { enabled, name }`

    The field configuration for this dataset.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

  - `filter: optional string`

    The Logpush filter predicate applied to this dataset. Omitted
    when no filter is set.

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/datasets \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "dataset": "dataset"
        }'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": {
    "created_at": "2019-12-27T18:11:19.117Z",
    "dataset": "dataset",
    "dataset_id": "dataset_id",
    "deletion_protection": true,
    "enabled": true,
    "fields": [
      {
        "enabled": true,
        "name": "name"
      }
    ],
    "object_id": "object_id",
    "object_type": "account",
    "updated_at": "2019-12-27T18:11:19.117Z",
    "filter": "filter"
  }
}
```

## Update an account or zone dataset

**put** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/datasets/{dataset_id}`

Updates the enabled state and/or field configuration of an account or zone dataset.

### Path Parameters

- `dataset_id: string`

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Body Parameters

- `enabled: boolean`

  Whether to enable or disable log ingest for this dataset.

- `deletion_protection: optional boolean`

  Set to `false` to allow deletion of this dataset.

- `fields: optional array of object { enabled, name }`

  Controls which fields the API ingests after the update. Defaults
  to all available fields when absent.

  - `enabled: boolean`

    Whether the API includes this field in log ingest.

  - `name: string`

    Field name in lowercase.

- `filter: optional string`

  Optional Logpush filter predicate to restrict which events are
  ingested. If omitted, the existing filter is left unchanged. Set
  to an empty string (`""`) to clear the filter. Otherwise,
  replaces the dataset's filter entirely.
  See [Logpush filters](https://developers.cloudflare.com/logs/reference/filters/)
  for syntax and examples.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

- `success: boolean`

- `result: optional Dataset`

  A Log Explorer dataset summary. List endpoints return this type and omit
  field configuration; use the single-dataset endpoint to retrieve it.

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `fields: array of object { enabled, name }`

    The field configuration for this dataset.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

  - `filter: optional string`

    The Logpush filter predicate applied to this dataset. Omitted
    when no filter is set.

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/datasets/$DATASET_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "enabled": true
        }'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": {
    "created_at": "2019-12-27T18:11:19.117Z",
    "dataset": "dataset",
    "dataset_id": "dataset_id",
    "deletion_protection": true,
    "enabled": true,
    "fields": [
      {
        "enabled": true,
        "name": "name"
      }
    ],
    "object_id": "object_id",
    "object_type": "account",
    "updated_at": "2019-12-27T18:11:19.117Z",
    "filter": "filter"
  }
}
```

## Delete an account or zone dataset

**delete** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/datasets/{dataset_id}`

Deletes a Log Explorer dataset for the account or zone. Dataset deletion must not
be protected.

### Path Parameters

- `dataset_id: string`

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

- `success: boolean`

- `result: optional Dataset`

  A Log Explorer dataset summary. List endpoints return this type and omit
  field configuration; use the single-dataset endpoint to retrieve it.

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `fields: array of object { enabled, name }`

    The field configuration for this dataset.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

  - `filter: optional string`

    The Logpush filter predicate applied to this dataset. Omitted
    when no filter is set.

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/datasets/$DATASET_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": {
    "created_at": "2019-12-27T18:11:19.117Z",
    "dataset": "dataset",
    "dataset_id": "dataset_id",
    "deletion_protection": true,
    "enabled": true,
    "fields": [
      {
        "enabled": true,
        "name": "name"
      }
    ],
    "object_id": "object_id",
    "object_type": "account",
    "updated_at": "2019-12-27T18:11:19.117Z",
    "filter": "filter"
  }
}
```

## Domain Types

### Create Request

- `CreateRequest object { dataset, fields, filter }`

  - `dataset: string`

    Dataset type name to create (e.g. `http_requests`).

  - `fields: optional array of object { enabled, name }`

    Controls which fields the API ingests. Defaults to all available
    fields when absent.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `filter: optional string`

    Optional Logpush filter predicate to restrict which events are ingested.
    If provided, replaces the dataset's default filter entirely.
    See [Logpush filters](https://developers.cloudflare.com/logs/reference/filters/)
    for syntax and examples.

### Dataset

- `Dataset object { created_at, dataset, dataset_id, 7 more }`

  A Log Explorer dataset summary. List endpoints return this type and omit
  field configuration; use the single-dataset endpoint to retrieve it.

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `fields: array of object { enabled, name }`

    The field configuration for this dataset.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

  - `filter: optional string`

    The Logpush filter predicate applied to this dataset. Omitted
    when no filter is set.

### Dataset Summary

- `DatasetSummary object { created_at, dataset, dataset_id, 5 more }`

  A Log Explorer dataset summary. List endpoints return this type and omit
  field configuration; use the single-dataset endpoint to retrieve it.

  - `created_at: string`

    RFC3339 timestamp recording when the API created this dataset.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `dataset_id: string`

    Unique dataset ID.

  - `deletion_protection: boolean`

    Whether deletion is blocked. Set to `false` before deleting the dataset.

  - `enabled: boolean`

    Whether log ingest is currently active for this dataset.

  - `object_id: string`

    Public ID of the account or zone that owns this dataset.

  - `object_type: "account" or "zone"`

    Whether this dataset belongs to an account or a zone.

    - `"account"`

    - `"zone"`

  - `updated_at: string`

    RFC3339 timestamp recording when the API last updated this dataset.

### Update Request

- `UpdateRequest object { enabled, deletion_protection, fields, filter }`

  - `enabled: boolean`

    Whether to enable or disable log ingest for this dataset.

  - `deletion_protection: optional boolean`

    Set to `false` to allow deletion of this dataset.

  - `fields: optional array of object { enabled, name }`

    Controls which fields the API ingests after the update. Defaults
    to all available fields when absent.

    - `enabled: boolean`

      Whether the API includes this field in log ingest.

    - `name: string`

      Field name in lowercase.

  - `filter: optional string`

    Optional Logpush filter predicate to restrict which events are
    ingested. If omitted, the existing filter is left unchanged. Set
    to an empty string (`""`) to clear the filter. Otherwise,
    replaces the dataset's filter entirely.
    See [Logpush filters](https://developers.cloudflare.com/logs/reference/filters/)
    for syntax and examples.

# Available

## List available account or zone datasets

**get** `/{accounts_or_zones}/{account_or_zone_id}/logs/explorer/datasets/available`

Returns all dataset types that this account or zone can create. Each entry
includes the dataset schema and timestamp field.

The schema shows all possible fields for a dataset. However, not all
fields may be available for your account or zone. When creating or updating a
dataset, only fields available to your account or zone can be enabled. If you
request a field that is not available, you will receive an error.

### Path Parameters

- `account_id: optional string`

  The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.

- `zone_id: optional string`

  The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.

### Returns

- `AvailableList object { errors, messages, success, result }`

  - `errors: array of ResponseInfo`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `messages: array of string`

  - `success: boolean`

  - `result: optional array of AvailableDataset`

    - `dataset: string`

      Dataset type name (e.g. `http_requests`).

    - `object_type: "account" or "zone"`

      Whether this dataset type is account-scoped or zone-scoped.

      - `"account"`

      - `"zone"`

    - `schema: object { properties, required, type }`

      JSON Schema that describes the fields this dataset exposes.

      - `properties: optional map[unknown]`

      - `required: optional array of string`

      - `type: optional "object"`

        - `"object"`

    - `timestamp_field: string`

      The primary timestamp field name for this dataset.

### Example

```http
curl https://api.cloudflare.com/client/v4/$ACCOUNTS_OR_ZONES/$ACCOUNT_OR_ZONE_ID/logs/explorer/datasets/available \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "success": true,
  "result": [
    {
      "dataset": "dataset",
      "object_type": "account",
      "schema": {
        "properties": {
          "foo": "bar"
        },
        "required": [
          "string"
        ],
        "type": "object"
      },
      "timestamp_field": "timestamp_field"
    }
  ]
}
```

## Domain Types

### Available Dataset

- `AvailableDataset object { dataset, object_type, schema, timestamp_field }`

  A dataset type that the account or zone can create.

  - `dataset: string`

    Dataset type name (e.g. `http_requests`).

  - `object_type: "account" or "zone"`

    Whether this dataset type is account-scoped or zone-scoped.

    - `"account"`

    - `"zone"`

  - `schema: object { properties, required, type }`

    JSON Schema that describes the fields this dataset exposes.

    - `properties: optional map[unknown]`

    - `required: optional array of string`

    - `type: optional "object"`

      - `"object"`

  - `timestamp_field: string`

    The primary timestamp field name for this dataset.

### Available List

- `AvailableList object { errors, messages, success, result }`

  - `errors: array of ResponseInfo`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `messages: array of string`

  - `success: boolean`

  - `result: optional array of AvailableDataset`

    - `dataset: string`

      Dataset type name (e.g. `http_requests`).

    - `object_type: "account" or "zone"`

      Whether this dataset type is account-scoped or zone-scoped.

      - `"account"`

      - `"zone"`

    - `schema: object { properties, required, type }`

      JSON Schema that describes the fields this dataset exposes.

      - `properties: optional map[unknown]`

      - `required: optional array of string`

      - `type: optional "object"`

        - `"object"`

    - `timestamp_field: string`

      The primary timestamp field name for this dataset.
