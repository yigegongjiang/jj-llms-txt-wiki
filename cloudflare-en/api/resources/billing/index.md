# Billing

# Profiles

## Billing Profile Details

**get** `/accounts/{account_id}/billing/profile`

Gets the current billing profile for the account.

### Path Parameters

- `account_id: string`

  Identifier

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

- `result: object { id, account_type, address, 36 more }`

  - `id: optional string`

    Billing item identifier tag.

  - `account_type: optional string`

  - `address: optional string`

  - `address2: optional string`

  - `balance: optional string`

  - `card_expiry_month: optional number`

  - `card_expiry_year: optional number`

  - `card_number: optional string`

  - `city: optional string`

  - `company: optional string`

  - `country: optional string`

  - `created_on: optional string`

  - `device_data: optional string`

  - `edited_on: optional string`

  - `enterprise_billing_email: optional string`

  - `enterprise_primary_email: optional string`

  - `first_name: optional string`

  - `is_partner: optional boolean`

  - `last_name: optional string`

  - `next_bill_date: optional string`

  - `payment_address: optional string`

  - `payment_address2: optional string`

  - `payment_city: optional string`

  - `payment_country: optional string`

  - `payment_email: optional string`

  - `payment_first_name: optional string`

  - `payment_gateway: optional string`

  - `payment_last_name: optional string`

  - `payment_nonce: optional string`

  - `payment_state: optional string`

  - `payment_zipcode: optional string`

  - `primary_email: optional string`

  - `state: optional string`

  - `tax_id_type: optional string`

  - `telephone: optional string`

  - `use_legacy: optional boolean`

  - `validation_code: optional string`

  - `vat: optional string`

  - `zipcode: optional string`

- `success: true`

  Whether the API call was successful

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/billing/profile \
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
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "result": {
    "id": "b69a9f3492637782896352daae219e7d",
    "account_type": "type",
    "address": "123 Main Street",
    "address2": "Apt 1",
    "balance": "0",
    "card_expiry_month": 12,
    "card_expiry_year": 2099,
    "card_number": "4242424242424242",
    "city": "Anytown",
    "company": "Company",
    "country": "Anycountry",
    "created_on": "2014-03-01T12:21:59.3456Z",
    "device_data": "sample_data",
    "edited_on": "2014-03-01T12:21:59.3456Z",
    "enterprise_billing_email": "johndoe@gmail.com",
    "enterprise_primary_email": "johndoe@gmail.com",
    "first_name": "John",
    "is_partner": false,
    "last_name": "Doe",
    "next_bill_date": "2014-03-01T12:21:59.3456Z",
    "payment_address": "123 Main Street",
    "payment_address2": "Apt 1",
    "payment_city": "Anytown",
    "payment_country": "Anycountry",
    "payment_email": "johndoe@gmail.com",
    "payment_first_name": "John",
    "payment_gateway": "gateway",
    "payment_last_name": "Doe",
    "payment_nonce": "abc123",
    "payment_state": "state",
    "payment_zipcode": "12345",
    "primary_email": "johndoe@gmail.com",
    "state": "AnyState",
    "tax_id_type": "type",
    "telephone": "1234567899",
    "use_legacy": false,
    "validation_code": "1111",
    "vat": "GB123456789",
    "zipcode": "12345"
  },
  "success": true
}
```

## Domain Types

### Profile Get Response

- `ProfileGetResponse object { id, account_type, address, 36 more }`

  - `id: optional string`

    Billing item identifier tag.

  - `account_type: optional string`

  - `address: optional string`

  - `address2: optional string`

  - `balance: optional string`

  - `card_expiry_month: optional number`

  - `card_expiry_year: optional number`

  - `card_number: optional string`

  - `city: optional string`

  - `company: optional string`

  - `country: optional string`

  - `created_on: optional string`

  - `device_data: optional string`

  - `edited_on: optional string`

  - `enterprise_billing_email: optional string`

  - `enterprise_primary_email: optional string`

  - `first_name: optional string`

  - `is_partner: optional boolean`

  - `last_name: optional string`

  - `next_bill_date: optional string`

  - `payment_address: optional string`

  - `payment_address2: optional string`

  - `payment_city: optional string`

  - `payment_country: optional string`

  - `payment_email: optional string`

  - `payment_first_name: optional string`

  - `payment_gateway: optional string`

  - `payment_last_name: optional string`

  - `payment_nonce: optional string`

  - `payment_state: optional string`

  - `payment_zipcode: optional string`

  - `primary_email: optional string`

  - `state: optional string`

  - `tax_id_type: optional string`

  - `telephone: optional string`

  - `use_legacy: optional boolean`

  - `validation_code: optional string`

  - `vat: optional string`

  - `zipcode: optional string`

# Usage

## Get Account Billable Usage Info (Version 1, Alpha)

**get** `/accounts/{account_id}/billable-usage/info`

Returns high-level usage information for the account, including coverage,
and subscription metadata.

### Path Parameters

- `account_id: string`

  Represents a Cloudflare resource identifier tag.

### Returns

- `errors: array of object { message, code }`

  Contains error details if the request failed.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `messages: array of object { message, code }`

  Contains any informational messages from the API.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `result: object { covered, subscriptions }`

  Contains the usage info.

  - `covered: boolean`

    Indicates whether the account is covered.

  - `subscriptions: array of object { id, billing_cycle_anchor_timestamp, start_timestamp, end_timestamp }`

    List of subscriptions for the account.

    - `id: string`

      The identifier for the Cloudflare subscription.

    - `billing_cycle_anchor_timestamp: string`

      The subscription billing cycle anchor timestamp.

    - `start_timestamp: string`

      The subscription start timestamp.

    - `end_timestamp: optional string`

      The subscription end timestamp. Omitted for active subscriptions; present only when the subscription has been cancelled.

- `success: true`

  Indicates whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/billable-usage/info \
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
  "result": {
    "covered": true,
    "subscriptions": [
      {
        "id": "3F3CD4CQ6N7FXO7IK6NVFJBOYA",
        "billing_cycle_anchor_timestamp": "2023-01-01T00:00:00Z",
        "start_timestamp": "2023-01-01T00:00:00Z",
        "end_timestamp": "2023-12-31T23:59:59Z"
      }
    ]
  },
  "success": true
}
```

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

## Get Account Usage (Version 2, Alpha, Restricted)

**get** `/accounts/{account_id}/billable/usage`

Returns cost and usage data for a single Cloudflare account, aligned
with the [FinOps FOCUS v1.3](https://focus.finops.org/focus-specification/v1-3/)
Cost and Usage dataset specification.

Each record represents one billable metric for one account on one day.
This includes all metered usage, including usage that falls within
free-tier allowances and may result in zero cost.

**Note:** Cost and pricing fields are not yet populated and
will be absent from responses until billing integration is complete.

When `from` and `to` are omitted, defaults to the start of the current
month through today. The maximum date range is 31 days.

### Path Parameters

- `account_id: string`

  Represents a Cloudflare resource identifier tag.

### Query Parameters

- `from: optional string`

  Start date for the usage query (ISO 8601). Required if `to` is set. When omitted along with `to`, defaults to the start of the current month. Filters by charge period (when consumption happened), not billing period. The maximum date range is 31 days.

- `to: optional string`

  End date for the usage query (ISO 8601). Required if `from` is set. When omitted along with `from`, defaults to today. Filters by charge period (when consumption happened), not billing period. The maximum date range is 31 days.

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

- `result: array of object { BillingAccountId, BillingAccountName, ChargeCategory, 32 more }`

  Contains the array of cost and usage records.

  - `BillingAccountId: string`

    Public identifier of the Cloudflare account (account tag).

  - `BillingAccountName: string`

    Display name of the Cloudflare account.

  - `ChargeCategory: "Usage"`

    Highest-level classification of a charge based on the nature of how it gets billed. Currently only "Usage" is supported.

    - `"Usage"`

  - `ChargeDescription: string`

    Self-contained summary of the charge's purpose and price.

  - `ChargeFrequency: "Usage-Based"`

    Indicates how often a charge occurs. Currently only "Usage-Based" is supported.

    - `"Usage-Based"`

  - `ChargePeriodEnd: string`

    Exclusive end of the time interval during which the usage was consumed.

  - `ChargePeriodStart: string`

    Inclusive start of the time interval during which the usage was consumed.

  - `ConsumedQuantity: number`

    Measured usage amount within the charge period. Reflects raw metered consumption before pricing transformations.

  - `ConsumedUnit: string`

    Unit of measure for the consumed quantity (e.g., "GB", "Requests", "vCPU-Hours").

  - `HostProviderName: string`

    Name of the entity providing the underlying infrastructure or platform.

  - `InvoiceIssuerName: string`

    Name of the entity responsible for invoicing for the services consumed.

  - `ServiceProviderName: string`

    Name of the entity that made the services available for purchase.

  - `x_BillableMetricId: string`

    The unique identifier for the billable metric in the Cloudflare catalog. Cloudflare extension; replaces FOCUS SkuId.

  - `x_BillableMetricName: string`

    The display name of the billable metric. Cloudflare extension; replaces FOCUS SkuMeter.

  - `BilledCost: optional number`

    A charge serving as the basis for invoicing, inclusive of all reduced rates and discounts while excluding the amortization of upfront charges (one-time or recurring).

  - `BillingCurrency: optional string`

    Currency that a charge was billed in (ISO 4217).

  - `BillingPeriodEnd: optional string`

    Exclusive end of the billing cycle that contains this usage record.

  - `BillingPeriodStart: optional string`

    Inclusive start of the billing cycle that contains this usage record.

  - `ChargeClass: optional "Correction"`

    Indicates whether the row represents a correction to one or more charges invoiced in a previous billing period.

    - `"Correction"`

  - `ContractedCost: optional number`

    Cost calculated by multiplying ContractedUnitPrice and the corresponding PricingQuantity.

  - `ContractedUnitPrice: optional number`

    The agreed-upon unit price for a single PricingUnit of the associated billable metric, inclusive of negotiated discounts, if present, while excluding any other discounts.

  - `EffectiveCost: optional number`

    The amortized cost of the charge after applying all reduced rates, discounts, and the applicable portion of relevant, prepaid purchases (one-time or recurring) that covered the charge.

  - `ListCost: optional number`

    Cost calculated by multiplying ListUnitPrice and the corresponding PricingQuantity.

  - `ListUnitPrice: optional number`

    Suggested provider-published unit price for a single PricingUnit of the associated billable metric, exclusive of any discounts.

  - `PricingQuantity: optional number`

    Volume of a given service used or purchased, based on the PricingUnit.

  - `PricingUnit: optional string`

    Provider-specified measurement unit for determining unit prices, indicating how the provider rates measured usage after applying pricing rules like block pricing.

  - `RegionId: optional string`

    Provider-assigned identifier for an isolated geographic area where a service is provided.

  - `RegionName: optional string`

    Name of an isolated geographic area where a service is provided.

  - `SubAccountId: optional string`

    Unique identifier assigned to a grouping of services. For Cloudflare, this is the subscription or contract ID.

  - `SubAccountName: optional string`

    Name assigned to a grouping of services. For Cloudflare, this is the subscription or contract display name.

  - `x_ProductCategoryName: optional string`

    The product category the charge belongs to (e.g., "Developer", "Cloudflare One"). Cloudflare extension; replaces FOCUS ServiceCategory.

  - `x_ProductFamilyId: optional string`

    The unique identifier for the product family in the Cloudflare catalog. Cloudflare extension; replaces FOCUS ServiceId.

  - `x_ProductFamilyName: optional string`

    The product family the charge belongs to (e.g., "R2", "Workers"). Cloudflare extension; replaces FOCUS ServiceName.

  - `x_ZoneId: optional string`

    The identifier for the Cloudflare zone (zone tag). Cloudflare extension.

  - `x_ZoneName: optional string`

    The display name of the Cloudflare zone. Cloudflare extension.

- `success: true`

  Indicates whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/billable/usage \
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
      "BillingAccountId": "023e105f4ecef8ad9ca31a8372d0c353",
      "BillingAccountName": "My Account",
      "ChargeCategory": "Usage",
      "ChargeDescription": "Workers Standard Requests — daily usage",
      "ChargeFrequency": "Usage-Based",
      "ChargePeriodEnd": "2025-05-02T00:00:00Z",
      "ChargePeriodStart": "2025-05-01T00:00:00Z",
      "ConsumedQuantity": 150000,
      "ConsumedUnit": "Requests",
      "HostProviderName": "Cloudflare",
      "InvoiceIssuerName": "Cloudflare",
      "ServiceProviderName": "Cloudflare",
      "x_BillableMetricId": "workers_standard_requests",
      "x_BillableMetricName": "Workers Standard Requests",
      "BilledCost": 0,
      "BillingCurrency": "USD",
      "BillingPeriodEnd": "2025-06-01T00:00:00Z",
      "BillingPeriodStart": "2025-05-01T00:00:00Z",
      "ChargeClass": "Correction",
      "ContractedCost": 0.75,
      "ContractedUnitPrice": 0.000005,
      "EffectiveCost": 0,
      "ListCost": 0.75,
      "ListUnitPrice": 0.000005,
      "PricingQuantity": 150000,
      "PricingUnit": "Requests",
      "RegionId": "EEUR",
      "RegionName": "Eastern Europe",
      "SubAccountId": "c9bd752d-9ca8-411d-b804-be44a758057f",
      "SubAccountName": "My Subscription",
      "x_ProductCategoryName": "Developer",
      "x_ProductFamilyId": "workers",
      "x_ProductFamilyName": "Workers",
      "x_ZoneId": "023e105f4ecef8ad9ca31a8372d0c353",
      "x_ZoneName": "example.com"
    }
  ],
  "success": true
}
```

## Get Account Billable Usage Info (Version 1, Alpha)

**get** `/accounts/{account_id}/billable-usage/info`

Returns high-level usage information for the account, including coverage,
and subscription metadata.

### Path Parameters

- `account_id: string`

  Represents a Cloudflare resource identifier tag.

### Returns

- `errors: array of object { message, code }`

  Contains error details if the request failed.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `messages: array of object { message, code }`

  Contains any informational messages from the API.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `result: object { covered, subscriptions }`

  Contains the usage info.

  - `covered: boolean`

    Indicates whether the account is covered.

  - `subscriptions: array of object { id, billing_cycle_anchor_timestamp, start_timestamp, end_timestamp }`

    List of subscriptions for the account.

    - `id: string`

      The identifier for the Cloudflare subscription.

    - `billing_cycle_anchor_timestamp: string`

      The subscription billing cycle anchor timestamp.

    - `start_timestamp: string`

      The subscription start timestamp.

    - `end_timestamp: optional string`

      The subscription end timestamp. Omitted for active subscriptions; present only when the subscription has been cancelled.

- `success: true`

  Indicates whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/billable-usage/info \
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
  "result": {
    "covered": true,
    "subscriptions": [
      {
        "id": "3F3CD4CQ6N7FXO7IK6NVFJBOYA",
        "billing_cycle_anchor_timestamp": "2023-01-01T00:00:00Z",
        "start_timestamp": "2023-01-01T00:00:00Z",
        "end_timestamp": "2023-12-31T23:59:59Z"
      }
    ]
  },
  "success": true
}
```

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

## Get Account Usage (Version 2, Alpha, Restricted)

**get** `/accounts/{account_id}/billable/usage`

Returns cost and usage data for a single Cloudflare account, aligned
with the [FinOps FOCUS v1.3](https://focus.finops.org/focus-specification/v1-3/)
Cost and Usage dataset specification.

Each record represents one billable metric for one account on one day.
This includes all metered usage, including usage that falls within
free-tier allowances and may result in zero cost.

**Note:** Cost and pricing fields are not yet populated and
will be absent from responses until billing integration is complete.

When `from` and `to` are omitted, defaults to the start of the current
month through today. The maximum date range is 31 days.

### Path Parameters

- `account_id: string`

  Represents a Cloudflare resource identifier tag.

### Query Parameters

- `from: optional string`

  Start date for the usage query (ISO 8601). Required if `to` is set. When omitted along with `to`, defaults to the start of the current month. Filters by charge period (when consumption happened), not billing period. The maximum date range is 31 days.

- `to: optional string`

  End date for the usage query (ISO 8601). Required if `from` is set. When omitted along with `from`, defaults to today. Filters by charge period (when consumption happened), not billing period. The maximum date range is 31 days.

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

- `result: array of object { BillingAccountId, BillingAccountName, ChargeCategory, 32 more }`

  Contains the array of cost and usage records.

  - `BillingAccountId: string`

    Public identifier of the Cloudflare account (account tag).

  - `BillingAccountName: string`

    Display name of the Cloudflare account.

  - `ChargeCategory: "Usage"`

    Highest-level classification of a charge based on the nature of how it gets billed. Currently only "Usage" is supported.

    - `"Usage"`

  - `ChargeDescription: string`

    Self-contained summary of the charge's purpose and price.

  - `ChargeFrequency: "Usage-Based"`

    Indicates how often a charge occurs. Currently only "Usage-Based" is supported.

    - `"Usage-Based"`

  - `ChargePeriodEnd: string`

    Exclusive end of the time interval during which the usage was consumed.

  - `ChargePeriodStart: string`

    Inclusive start of the time interval during which the usage was consumed.

  - `ConsumedQuantity: number`

    Measured usage amount within the charge period. Reflects raw metered consumption before pricing transformations.

  - `ConsumedUnit: string`

    Unit of measure for the consumed quantity (e.g., "GB", "Requests", "vCPU-Hours").

  - `HostProviderName: string`

    Name of the entity providing the underlying infrastructure or platform.

  - `InvoiceIssuerName: string`

    Name of the entity responsible for invoicing for the services consumed.

  - `ServiceProviderName: string`

    Name of the entity that made the services available for purchase.

  - `x_BillableMetricId: string`

    The unique identifier for the billable metric in the Cloudflare catalog. Cloudflare extension; replaces FOCUS SkuId.

  - `x_BillableMetricName: string`

    The display name of the billable metric. Cloudflare extension; replaces FOCUS SkuMeter.

  - `BilledCost: optional number`

    A charge serving as the basis for invoicing, inclusive of all reduced rates and discounts while excluding the amortization of upfront charges (one-time or recurring).

  - `BillingCurrency: optional string`

    Currency that a charge was billed in (ISO 4217).

  - `BillingPeriodEnd: optional string`

    Exclusive end of the billing cycle that contains this usage record.

  - `BillingPeriodStart: optional string`

    Inclusive start of the billing cycle that contains this usage record.

  - `ChargeClass: optional "Correction"`

    Indicates whether the row represents a correction to one or more charges invoiced in a previous billing period.

    - `"Correction"`

  - `ContractedCost: optional number`

    Cost calculated by multiplying ContractedUnitPrice and the corresponding PricingQuantity.

  - `ContractedUnitPrice: optional number`

    The agreed-upon unit price for a single PricingUnit of the associated billable metric, inclusive of negotiated discounts, if present, while excluding any other discounts.

  - `EffectiveCost: optional number`

    The amortized cost of the charge after applying all reduced rates, discounts, and the applicable portion of relevant, prepaid purchases (one-time or recurring) that covered the charge.

  - `ListCost: optional number`

    Cost calculated by multiplying ListUnitPrice and the corresponding PricingQuantity.

  - `ListUnitPrice: optional number`

    Suggested provider-published unit price for a single PricingUnit of the associated billable metric, exclusive of any discounts.

  - `PricingQuantity: optional number`

    Volume of a given service used or purchased, based on the PricingUnit.

  - `PricingUnit: optional string`

    Provider-specified measurement unit for determining unit prices, indicating how the provider rates measured usage after applying pricing rules like block pricing.

  - `RegionId: optional string`

    Provider-assigned identifier for an isolated geographic area where a service is provided.

  - `RegionName: optional string`

    Name of an isolated geographic area where a service is provided.

  - `SubAccountId: optional string`

    Unique identifier assigned to a grouping of services. For Cloudflare, this is the subscription or contract ID.

  - `SubAccountName: optional string`

    Name assigned to a grouping of services. For Cloudflare, this is the subscription or contract display name.

  - `x_ProductCategoryName: optional string`

    The product category the charge belongs to (e.g., "Developer", "Cloudflare One"). Cloudflare extension; replaces FOCUS ServiceCategory.

  - `x_ProductFamilyId: optional string`

    The unique identifier for the product family in the Cloudflare catalog. Cloudflare extension; replaces FOCUS ServiceId.

  - `x_ProductFamilyName: optional string`

    The product family the charge belongs to (e.g., "R2", "Workers"). Cloudflare extension; replaces FOCUS ServiceName.

  - `x_ZoneId: optional string`

    The identifier for the Cloudflare zone (zone tag). Cloudflare extension.

  - `x_ZoneName: optional string`

    The display name of the Cloudflare zone. Cloudflare extension.

- `success: true`

  Indicates whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/billable/usage \
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
      "BillingAccountId": "023e105f4ecef8ad9ca31a8372d0c353",
      "BillingAccountName": "My Account",
      "ChargeCategory": "Usage",
      "ChargeDescription": "Workers Standard Requests — daily usage",
      "ChargeFrequency": "Usage-Based",
      "ChargePeriodEnd": "2025-05-02T00:00:00Z",
      "ChargePeriodStart": "2025-05-01T00:00:00Z",
      "ConsumedQuantity": 150000,
      "ConsumedUnit": "Requests",
      "HostProviderName": "Cloudflare",
      "InvoiceIssuerName": "Cloudflare",
      "ServiceProviderName": "Cloudflare",
      "x_BillableMetricId": "workers_standard_requests",
      "x_BillableMetricName": "Workers Standard Requests",
      "BilledCost": 0,
      "BillingCurrency": "USD",
      "BillingPeriodEnd": "2025-06-01T00:00:00Z",
      "BillingPeriodStart": "2025-05-01T00:00:00Z",
      "ChargeClass": "Correction",
      "ContractedCost": 0.75,
      "ContractedUnitPrice": 0.000005,
      "EffectiveCost": 0,
      "ListCost": 0.75,
      "ListUnitPrice": 0.000005,
      "PricingQuantity": 150000,
      "PricingUnit": "Requests",
      "RegionId": "EEUR",
      "RegionName": "Eastern Europe",
      "SubAccountId": "c9bd752d-9ca8-411d-b804-be44a758057f",
      "SubAccountName": "My Subscription",
      "x_ProductCategoryName": "Developer",
      "x_ProductFamilyId": "workers",
      "x_ProductFamilyName": "Workers",
      "x_ZoneId": "023e105f4ecef8ad9ca31a8372d0c353",
      "x_ZoneName": "example.com"
    }
  ],
  "success": true
}
```

## Domain Types

### Usage Paygo Info Response

- `UsagePaygoInfoResponse object { covered, subscriptions }`

  Contains the usage info.

  - `covered: boolean`

    Indicates whether the account is covered.

  - `subscriptions: array of object { id, billing_cycle_anchor_timestamp, start_timestamp, end_timestamp }`

    List of subscriptions for the account.

    - `id: string`

      The identifier for the Cloudflare subscription.

    - `billing_cycle_anchor_timestamp: string`

      The subscription billing cycle anchor timestamp.

    - `start_timestamp: string`

      The subscription start timestamp.

    - `end_timestamp: optional string`

      The subscription end timestamp. Omitted for active subscriptions; present only when the subscription has been cancelled.

### Usage Paygo Response

- `UsagePaygoResponse = array of object { BilledCost, BillingAccountId, BillingAccountName, 24 more }`

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

### Usage Get Response

- `UsageGetResponse = array of object { BillingAccountId, BillingAccountName, ChargeCategory, 32 more }`

  Contains the array of cost and usage records.

  - `BillingAccountId: string`

    Public identifier of the Cloudflare account (account tag).

  - `BillingAccountName: string`

    Display name of the Cloudflare account.

  - `ChargeCategory: "Usage"`

    Highest-level classification of a charge based on the nature of how it gets billed. Currently only "Usage" is supported.

    - `"Usage"`

  - `ChargeDescription: string`

    Self-contained summary of the charge's purpose and price.

  - `ChargeFrequency: "Usage-Based"`

    Indicates how often a charge occurs. Currently only "Usage-Based" is supported.

    - `"Usage-Based"`

  - `ChargePeriodEnd: string`

    Exclusive end of the time interval during which the usage was consumed.

  - `ChargePeriodStart: string`

    Inclusive start of the time interval during which the usage was consumed.

  - `ConsumedQuantity: number`

    Measured usage amount within the charge period. Reflects raw metered consumption before pricing transformations.

  - `ConsumedUnit: string`

    Unit of measure for the consumed quantity (e.g., "GB", "Requests", "vCPU-Hours").

  - `HostProviderName: string`

    Name of the entity providing the underlying infrastructure or platform.

  - `InvoiceIssuerName: string`

    Name of the entity responsible for invoicing for the services consumed.

  - `ServiceProviderName: string`

    Name of the entity that made the services available for purchase.

  - `x_BillableMetricId: string`

    The unique identifier for the billable metric in the Cloudflare catalog. Cloudflare extension; replaces FOCUS SkuId.

  - `x_BillableMetricName: string`

    The display name of the billable metric. Cloudflare extension; replaces FOCUS SkuMeter.

  - `BilledCost: optional number`

    A charge serving as the basis for invoicing, inclusive of all reduced rates and discounts while excluding the amortization of upfront charges (one-time or recurring).

  - `BillingCurrency: optional string`

    Currency that a charge was billed in (ISO 4217).

  - `BillingPeriodEnd: optional string`

    Exclusive end of the billing cycle that contains this usage record.

  - `BillingPeriodStart: optional string`

    Inclusive start of the billing cycle that contains this usage record.

  - `ChargeClass: optional "Correction"`

    Indicates whether the row represents a correction to one or more charges invoiced in a previous billing period.

    - `"Correction"`

  - `ContractedCost: optional number`

    Cost calculated by multiplying ContractedUnitPrice and the corresponding PricingQuantity.

  - `ContractedUnitPrice: optional number`

    The agreed-upon unit price for a single PricingUnit of the associated billable metric, inclusive of negotiated discounts, if present, while excluding any other discounts.

  - `EffectiveCost: optional number`

    The amortized cost of the charge after applying all reduced rates, discounts, and the applicable portion of relevant, prepaid purchases (one-time or recurring) that covered the charge.

  - `ListCost: optional number`

    Cost calculated by multiplying ListUnitPrice and the corresponding PricingQuantity.

  - `ListUnitPrice: optional number`

    Suggested provider-published unit price for a single PricingUnit of the associated billable metric, exclusive of any discounts.

  - `PricingQuantity: optional number`

    Volume of a given service used or purchased, based on the PricingUnit.

  - `PricingUnit: optional string`

    Provider-specified measurement unit for determining unit prices, indicating how the provider rates measured usage after applying pricing rules like block pricing.

  - `RegionId: optional string`

    Provider-assigned identifier for an isolated geographic area where a service is provided.

  - `RegionName: optional string`

    Name of an isolated geographic area where a service is provided.

  - `SubAccountId: optional string`

    Unique identifier assigned to a grouping of services. For Cloudflare, this is the subscription or contract ID.

  - `SubAccountName: optional string`

    Name assigned to a grouping of services. For Cloudflare, this is the subscription or contract display name.

  - `x_ProductCategoryName: optional string`

    The product category the charge belongs to (e.g., "Developer", "Cloudflare One"). Cloudflare extension; replaces FOCUS ServiceCategory.

  - `x_ProductFamilyId: optional string`

    The unique identifier for the product family in the Cloudflare catalog. Cloudflare extension; replaces FOCUS ServiceId.

  - `x_ProductFamilyName: optional string`

    The product family the charge belongs to (e.g., "R2", "Workers"). Cloudflare extension; replaces FOCUS ServiceName.

  - `x_ZoneId: optional string`

    The identifier for the Cloudflare zone (zone tag). Cloudflare extension.

  - `x_ZoneName: optional string`

    The display name of the Cloudflare zone. Cloudflare extension.

### Usage Get Account Usage Info V1 Response

- `UsageGetAccountUsageInfoV1Response object { covered, subscriptions }`

  Contains the usage info.

  - `covered: boolean`

    Indicates whether the account is covered.

  - `subscriptions: array of object { id, billing_cycle_anchor_timestamp, start_timestamp, end_timestamp }`

    List of subscriptions for the account.

    - `id: string`

      The identifier for the Cloudflare subscription.

    - `billing_cycle_anchor_timestamp: string`

      The subscription billing cycle anchor timestamp.

    - `start_timestamp: string`

      The subscription start timestamp.

    - `end_timestamp: optional string`

      The subscription end timestamp. Omitted for active subscriptions; present only when the subscription has been cancelled.

### Usage Get Account Usage V1 Response

- `UsageGetAccountUsageV1Response = array of object { BilledCost, BillingAccountId, BillingAccountName, 24 more }`

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

### Usage Get Account Usage V2 Response

- `UsageGetAccountUsageV2Response = array of object { BillingAccountId, BillingAccountName, ChargeCategory, 32 more }`

  Contains the array of cost and usage records.

  - `BillingAccountId: string`

    Public identifier of the Cloudflare account (account tag).

  - `BillingAccountName: string`

    Display name of the Cloudflare account.

  - `ChargeCategory: "Usage"`

    Highest-level classification of a charge based on the nature of how it gets billed. Currently only "Usage" is supported.

    - `"Usage"`

  - `ChargeDescription: string`

    Self-contained summary of the charge's purpose and price.

  - `ChargeFrequency: "Usage-Based"`

    Indicates how often a charge occurs. Currently only "Usage-Based" is supported.

    - `"Usage-Based"`

  - `ChargePeriodEnd: string`

    Exclusive end of the time interval during which the usage was consumed.

  - `ChargePeriodStart: string`

    Inclusive start of the time interval during which the usage was consumed.

  - `ConsumedQuantity: number`

    Measured usage amount within the charge period. Reflects raw metered consumption before pricing transformations.

  - `ConsumedUnit: string`

    Unit of measure for the consumed quantity (e.g., "GB", "Requests", "vCPU-Hours").

  - `HostProviderName: string`

    Name of the entity providing the underlying infrastructure or platform.

  - `InvoiceIssuerName: string`

    Name of the entity responsible for invoicing for the services consumed.

  - `ServiceProviderName: string`

    Name of the entity that made the services available for purchase.

  - `x_BillableMetricId: string`

    The unique identifier for the billable metric in the Cloudflare catalog. Cloudflare extension; replaces FOCUS SkuId.

  - `x_BillableMetricName: string`

    The display name of the billable metric. Cloudflare extension; replaces FOCUS SkuMeter.

  - `BilledCost: optional number`

    A charge serving as the basis for invoicing, inclusive of all reduced rates and discounts while excluding the amortization of upfront charges (one-time or recurring).

  - `BillingCurrency: optional string`

    Currency that a charge was billed in (ISO 4217).

  - `BillingPeriodEnd: optional string`

    Exclusive end of the billing cycle that contains this usage record.

  - `BillingPeriodStart: optional string`

    Inclusive start of the billing cycle that contains this usage record.

  - `ChargeClass: optional "Correction"`

    Indicates whether the row represents a correction to one or more charges invoiced in a previous billing period.

    - `"Correction"`

  - `ContractedCost: optional number`

    Cost calculated by multiplying ContractedUnitPrice and the corresponding PricingQuantity.

  - `ContractedUnitPrice: optional number`

    The agreed-upon unit price for a single PricingUnit of the associated billable metric, inclusive of negotiated discounts, if present, while excluding any other discounts.

  - `EffectiveCost: optional number`

    The amortized cost of the charge after applying all reduced rates, discounts, and the applicable portion of relevant, prepaid purchases (one-time or recurring) that covered the charge.

  - `ListCost: optional number`

    Cost calculated by multiplying ListUnitPrice and the corresponding PricingQuantity.

  - `ListUnitPrice: optional number`

    Suggested provider-published unit price for a single PricingUnit of the associated billable metric, exclusive of any discounts.

  - `PricingQuantity: optional number`

    Volume of a given service used or purchased, based on the PricingUnit.

  - `PricingUnit: optional string`

    Provider-specified measurement unit for determining unit prices, indicating how the provider rates measured usage after applying pricing rules like block pricing.

  - `RegionId: optional string`

    Provider-assigned identifier for an isolated geographic area where a service is provided.

  - `RegionName: optional string`

    Name of an isolated geographic area where a service is provided.

  - `SubAccountId: optional string`

    Unique identifier assigned to a grouping of services. For Cloudflare, this is the subscription or contract ID.

  - `SubAccountName: optional string`

    Name assigned to a grouping of services. For Cloudflare, this is the subscription or contract display name.

  - `x_ProductCategoryName: optional string`

    The product category the charge belongs to (e.g., "Developer", "Cloudflare One"). Cloudflare extension; replaces FOCUS ServiceCategory.

  - `x_ProductFamilyId: optional string`

    The unique identifier for the product family in the Cloudflare catalog. Cloudflare extension; replaces FOCUS ServiceId.

  - `x_ProductFamilyName: optional string`

    The product family the charge belongs to (e.g., "R2", "Workers"). Cloudflare extension; replaces FOCUS ServiceName.

  - `x_ZoneId: optional string`

    The identifier for the Cloudflare zone (zone tag). Cloudflare extension.

  - `x_ZoneName: optional string`

    The display name of the Cloudflare zone. Cloudflare extension.
