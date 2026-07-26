# Graphql

## GraphQL endpoint for event aggregation

**post** `/accounts/{account_id}/cloudforce-one/events/graphql`

Execute GraphQL aggregations over threat events. Supports multi-dimensional group-bys, optional date range filtering, and multi-dataset aggregation.

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `data: optional unknown`

- `errors: optional array of unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/graphql \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "data": {},
  "errors": [
    {}
  ]
}
```

## Domain Types

### Graphql Create Response

- `GraphqlCreateResponse object { data, errors }`

  - `data: optional unknown`

  - `errors: optional array of unknown`
