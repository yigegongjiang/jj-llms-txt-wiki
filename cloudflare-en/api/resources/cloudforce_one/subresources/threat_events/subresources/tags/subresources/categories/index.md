# Categories

## Lists all tag categories (SoT)

**get** `/accounts/{account_id}/cloudforce-one/events/tags/categories`

Returns all Source-of-Truth tag categories for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `search: optional string`

### Returns

- `categories: array of object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "categories": [
    {
      "name": "Actor",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "createdAt": "createdAt",
      "description": "description",
      "updatedAt": "updatedAt"
    }
  ]
}
```

## Creates a new tag category (SoT)

**post** `/accounts/{account_id}/cloudforce-one/events/tags/categories/create`

Creates a new Source-of-Truth tag category for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `name: string`

- `description: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "name": "Actor"
        }'
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```

## Updates a tag category (SoT)

**patch** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Updates a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Body Parameters

- `description: optional string`

- `name: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```

## Deletes a tag category (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Deletes a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
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

- `CategoryListResponse object { categories }`

  - `categories: array of object { name, uuid, createdAt, 2 more }`

    - `name: string`

    - `uuid: string`

    - `createdAt: optional string`

    - `description: optional string`

    - `updatedAt: optional string`

### Category Create Response

- `CategoryCreateResponse object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Category Edit Response

- `CategoryEditResponse object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Category Delete Response

- `CategoryDeleteResponse object { uuid }`

  - `uuid: string`
