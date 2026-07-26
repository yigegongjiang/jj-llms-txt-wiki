# Categories

## Lists categories across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/categories`

List categories across one or more datasets for the account.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query categories from. If not provided, uses the default dataset.

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "killChain": 0,
    "name": "name",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "mitreAttack": [
      "T1234"
    ],
    "mitreCapec": [
      "123"
    ],
    "shortname": "shortname"
  }
]
```

## Reads a category

**get** `/accounts/{account_id}/cloudforce-one/events/categories/{category_id}`

Retrieve a single category by its identifier.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_id: string`

  Category UUID.

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/$CATEGORY_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "killChain": 0,
  "name": "name",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "mitreAttack": [
    "T1234"
  ],
  "mitreCapec": [
    "123"
  ],
  "shortname": "shortname"
}
```

## Creates a new category

**post** `/accounts/{account_id}/cloudforce-one/events/categories/create`

Create a new event category for the account.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `killChain: number`

- `name: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "killChain": 0,
          "name": "name",
          "shortname": "shortname"
        }'
```

#### Response

```json
{
  "killChain": 0,
  "name": "name",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "mitreAttack": [
    "T1234"
  ],
  "mitreCapec": [
    "123"
  ],
  "shortname": "shortname"
}
```

## Updates a category

**patch** `/accounts/{account_id}/cloudforce-one/events/categories/{category_id}`

Update an existing category by its identifier.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_id: string`

  Category UUID.

### Body Parameters

- `killChain: optional number`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `name: optional string`

- `shortname: optional string`

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/$CATEGORY_ID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "killChain": 0,
  "name": "name",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "mitreAttack": [
    "T1234"
  ],
  "mitreCapec": [
    "123"
  ],
  "shortname": "shortname"
}
```

## Deletes a category

**delete** `/accounts/{account_id}/cloudforce-one/events/categories/{category_id}`

Delete a category by its identifier.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_id: string`

  Category UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/$CATEGORY_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Domain Types

### Category List Response

- `CategoryListResponse = array of object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Get Response

- `CategoryGetResponse object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Create Response

- `CategoryCreateResponse object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Edit Response

- `CategoryEditResponse object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Delete Response

- `CategoryDeleteResponse object { uuid }`

  - `uuid: string`

# Catalog

## Lists categories

**get** `/accounts/{account_id}/cloudforce-one/events/categories/catalog`

List all categories stored in the account catalog.

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/catalog \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "killChain": 0,
    "name": "name",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "mitreAttack": [
      "T1234"
    ],
    "mitreCapec": [
      "123"
    ],
    "shortname": "shortname"
  }
]
```

## Domain Types

### Catalog List Response

- `CatalogListResponse = array of object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`
