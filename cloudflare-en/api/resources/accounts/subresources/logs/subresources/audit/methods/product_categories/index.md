## List account audit log product categories (Version 2)

**get** `/accounts/{account_id}/logs/audit/product_categories`

Lists the available audit log product categories and the resource products each one expands to. Use these values with the product_category filter on the account audit logs endpoint.

### Path Parameters

- `account_id: string`

  The unique id that identifies the account.

### Returns

- `errors: optional array of object { message }`

  - `message: string`

- `result: optional array of object { label, products, value }`

  - `label: optional string`

    A human-readable label for the product category.

  - `products: optional array of object { label, value }`

    The resource products that the product category expands to.

    - `label: optional string`

      A human-readable label for the product.

    - `value: optional string`

      The resource_product value that the product category expands to.

  - `value: optional string`

    The product category identifier used with the product_category filter.

- `success: optional true`

  Indicates whether the API call was successful

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/logs/audit/product_categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "message": "message"
    }
  ],
  "result": [
    {
      "label": "Zero Trust",
      "products": [
        {
          "label": "Gateway",
          "value": "gateway"
        }
      ],
      "value": "zerotrust"
    }
  ],
  "success": true
}
```
