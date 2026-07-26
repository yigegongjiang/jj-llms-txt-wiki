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
