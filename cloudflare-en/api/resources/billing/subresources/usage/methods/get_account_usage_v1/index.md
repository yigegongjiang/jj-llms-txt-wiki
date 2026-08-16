## Get Account Billable Usage (Version 1, Alpha)

**get** `/accounts/{account_id}/billable-usage`

Returns billable usage data for the account.
When no query parameters are provided, returns usage for the current
billing period.

### Path Parameters

- `account_id: string`

  Represents a Cloudflare resource identifier tag.

### Query Parameters

- `from: optional string`

  Start date for the usage query (ISO 8601). The provided time range must include the subscription billing cycle anchor day, otherwise no usage data is returned. Use the info endpoint to retrieve the subscription anchor day.

- `to: optional string`

  End date for the usage query (ISO 8601).

### Returns

- `errors: array of object { message, code }`

  Contains error details if the request failed.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `messages: array of object { message, code }`

  Contains informational notices about the response.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `result: array of object { BilledCost, BillingAccountId, BillingAccountName, 24 more }`

  Contains the array of billable usage records.

  - `BilledCost: number`

    The amount invoiced for this charge. PayGo is billed directly by Cloudflare, so this equals ContractedCost.

  - `BillingAccountId: string`

    The identifier of the account the charge is billed to (account tag).

  - `BillingAccountName: string`

    The display name of the billing account. Null when the name could not be resolved.

  - `BillingCurrency: string`

    Specifies the billing currency code (ISO 4217).

  - `BillingPeriodStart: string`

    Indicates the start of the billing period. There is no `BillingPeriodEnd` counterpart; see the known gaps described on this schema.

  - `ChargeCategory: "Usage"`

    Describes the nature of the charge. Always "Usage" for this endpoint, which only returns metered usage.

    - `"Usage"`

  - `ChargeClass: string`

    Indicates whether the row corrects a previously invoiced billing period. Always null for this endpoint, which does not return corrections.

  - `ChargeDescription: string`

    A human-readable summary of the charge.

  - `ChargePeriodEnd: string`

    Indicates the end of the charge period.

  - `ChargePeriodStart: string`

    Indicates the start of the charge period.

  - `ConsumedQuantity: number`

    Specifies the quantity consumed during this charge period.

  - `ConsumedUnit: string`

    A display name for the unit of measurement used for the product (for example, "GB-months", "GB-seconds"). May be empty when the unit is implicit in the service name.

  - `ContractedCost: number`

    Specifies the cost for this charge period in the billing currency.

  - `CumulatedContractedCost: number`

    Specifies the cumulated cost for the billing period in the billing currency.

  - `CumulatedPricingQuantity: number`

    Specifies the cumulated pricing quantity for the billing period.

  - `EffectiveCost: number`

    The amortized cost of the charge. PayGo has no upfront commitments, so this equals ContractedCost.

  - `HostProviderName: string`

    The provider that hosts the infrastructure or platform the service runs on.

  - `InvoiceIssuerName: string`

    The entity that issues the invoice for this charge.

  - `ListCost: number`

    The cost at published list prices, before any discount. PayGo has no commitment discounts, so this equals ContractedCost.

  - `PricingQuantity: number`

    Specifies the pricing quantity for this charge period.

  - `PricingUnit: string`

    The unit that PricingQuantity is expressed in. Unlike ConsumedUnit this is never empty; it falls back to "Count" when the service has no explicit unit.

  - `ServiceName: string`

    Identifies the Cloudflare service.

  - `ServiceProviderName: string`

    The provider of the purchased service.

  - `ServiceFamilyName: optional string`

    Identifies the product family for the Cloudflare service.

  - `SubscriptionId: optional string`

    The identifier for the Cloudflare subscription.

  - `ZoneId: optional string`

    The identifier for the Cloudflare zone (zone tag).

  - `ZoneName: optional string`

    The display name of the Cloudflare zone.

- `success: true`

  Indicates whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/billable-usage \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "message": "message",
      "code": 0
    }
  ],
  "messages": [
    {
      "message": "message",
      "code": 0
    }
  ],
  "result": [
    {
      "BilledCost": 0.75,
      "BillingAccountId": "023e105f4ecef8ad9ca31a8372d0c353",
      "BillingAccountName": "Example Account",
      "BillingCurrency": "USD",
      "BillingPeriodStart": "2025-02-01T00:00:00Z",
      "ChargeCategory": "Usage",
      "ChargeClass": "ChargeClass",
      "ChargeDescription": "Workers Standard usage measured in Count",
      "ChargePeriodEnd": "2025-02-02T00:00:00Z",
      "ChargePeriodStart": "2025-02-01T00:00:00Z",
      "ConsumedQuantity": 150000,
      "ConsumedUnit": "GB-months",
      "ContractedCost": 0.75,
      "CumulatedContractedCost": 2.25,
      "CumulatedPricingQuantity": 4500000,
      "EffectiveCost": 0.75,
      "HostProviderName": "Cloudflare, Inc.",
      "InvoiceIssuerName": "Cloudflare, Inc.",
      "ListCost": 0.75,
      "PricingQuantity": 150000,
      "PricingUnit": "Count",
      "ServiceName": "Workers Standard",
      "ServiceProviderName": "Cloudflare, Inc.",
      "ServiceFamilyName": "Workers",
      "SubscriptionId": "3F3CD4CQ6N7FXO7IK6NVFJBOYA",
      "ZoneId": "023e105f4ecef8ad9ca31a8372d0c353",
      "ZoneName": "example.com"
    }
  ],
  "success": true
}
```
