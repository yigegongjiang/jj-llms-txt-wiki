# Sinkholes

## List sinkholes owned by this account

**get** `/accounts/{account_id}/intel/sinkholes`

Lists sinkholes owned by the account for redirecting malicious traffic.

### Path Parameters

- `account_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of Sinkhole`

  - `id: optional string`

    The unique identifier for the sinkhole.

  - `account_tag: optional string`

    The account tag that owns this sinkhole.

  - `created_on: optional string`

    The date and time when the sinkhole was created.

  - `modified_on: optional string`

    The date and time when the sinkhole was last modified.

  - `name: optional string`

    The name of the sinkhole.

  - `r2_bucket: optional string`

    The name of the R2 bucket to store results.

  - `r2_id: optional string`

    The id of the R2 instance.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "success": true,
  "result": [
    {
      "id": "93defa6e909e464e8c89a85859f36d3c",
      "account_tag": "233f45e61fd1f7e21e1e154ede4q2859",
      "created_on": "2023-05-12T12:21:56.777653Z",
      "modified_on": "2023-06-18T03:13:34.123321Z",
      "name": "my_sinkhole",
      "r2_bucket": "my_bucket",
      "r2_id": "example_r2_id"
    }
  ]
}
```

## Get a sinkhole

**get** `/accounts/{account_id}/intel/sinkholes/{sinkhole_id}`

Get the specified sinkhole by its unique identifier.

### Path Parameters

- `account_id: string`

  Identifier.

- `sinkhole_id: string`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Sinkhole`

  - `id: optional string`

    The unique identifier for the sinkhole.

  - `account_tag: optional string`

    The account tag that owns this sinkhole.

  - `created_on: optional string`

    The date and time when the sinkhole was created.

  - `modified_on: optional string`

    The date and time when the sinkhole was last modified.

  - `name: optional string`

    The name of the sinkhole.

  - `r2_bucket: optional string`

    The name of the R2 bucket to store results.

  - `r2_id: optional string`

    The id of the R2 instance.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes/$SINKHOLE_ID \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "success": true,
  "result": {
    "id": "93defa6e909e464e8c89a85859f36d3c",
    "account_tag": "233f45e61fd1f7e21e1e154ede4q2859",
    "created_on": "2023-05-12T12:21:56.777653Z",
    "modified_on": "2023-06-18T03:13:34.123321Z",
    "name": "my_sinkhole",
    "r2_bucket": "my_bucket",
    "r2_id": "example_r2_id"
  }
}
```

## Create a new sinkhole for your account

**post** `/accounts/{account_id}/intel/sinkholes`

Create a new sinkhole. Logs of large request bodies will be truncated, but the full request body can be recorded in R2. If you wish to record large request bodies in R2, include the R2 key ID, key secret, and bucket name in the request body.

### Path Parameters

- `account_id: string`

  Identifier.

### Body Parameters

- `name: string`

  The name of the sinkhole.

- `r2_bucket: optional string`

  The name of the R2 bucket to store results. Required if you want to store large request bodies in R2.

- `r2_id: optional string`

  The id of the R2 instance. Required if you want to store large request bodies in R2.

- `r2_secret: optional string`

  The secret key for the R2 API token. Required if you want to store large request bodies in R2.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Sinkhole`

  - `id: optional string`

    The unique identifier for the sinkhole.

  - `account_tag: optional string`

    The account tag that owns this sinkhole.

  - `created_on: optional string`

    The date and time when the sinkhole was created.

  - `modified_on: optional string`

    The date and time when the sinkhole was last modified.

  - `name: optional string`

    The name of the sinkhole.

  - `r2_bucket: optional string`

    The name of the R2 bucket to store results.

  - `r2_id: optional string`

    The id of the R2 instance.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "name": "name"
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
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": {
    "id": "93defa6e909e464e8c89a85859f36d3c",
    "account_tag": "233f45e61fd1f7e21e1e154ede4q2859",
    "created_on": "2023-05-12T12:21:56.777653Z",
    "modified_on": "2023-06-18T03:13:34.123321Z",
    "name": "my_sinkhole",
    "r2_bucket": "my_bucket",
    "r2_id": "example_r2_id"
  }
}
```

## Update a sinkhole

**put** `/accounts/{account_id}/intel/sinkholes/{sinkhole_id}`

Update the name or R2 configuration of the specified sinkhole.

### Path Parameters

- `account_id: string`

  Identifier.

- `sinkhole_id: string`

### Body Parameters

- `name: string`

  The name of the sinkhole.

- `r2_bucket: optional string`

  The name of the R2 bucket to store results. Required if you want to store large request bodies in R2.

- `r2_id: optional string`

  The id of the R2 instance. Required if you want to store large request bodies in R2.

- `r2_secret: optional string`

  The secret key for the R2 API token. Required if you want to store large request bodies in R2.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes/$SINKHOLE_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "name": "name"
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
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": {}
}
```

## Delete a sinkhole

**delete** `/accounts/{account_id}/intel/sinkholes/{sinkhole_id}`

Delete the specified sinkhole. The sinkhole must not have any active ingress rules defined. A 409 response code indicates that this condition is not met.

### Path Parameters

- `account_id: string`

  Identifier.

- `sinkhole_id: string`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes/$SINKHOLE_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "success": true,
  "result": {}
}
```

## Domain Types

### Sinkhole

- `Sinkhole object { id, account_tag, created_on, 4 more }`

  - `id: optional string`

    The unique identifier for the sinkhole.

  - `account_tag: optional string`

    The account tag that owns this sinkhole.

  - `created_on: optional string`

    The date and time when the sinkhole was created.

  - `modified_on: optional string`

    The date and time when the sinkhole was last modified.

  - `name: optional string`

    The name of the sinkhole.

  - `r2_bucket: optional string`

    The name of the R2 bucket to store results.

  - `r2_id: optional string`

    The id of the R2 instance.

### Sinkhole Update Response

- `SinkholeUpdateResponse = unknown`

### Sinkhole Delete Response

- `SinkholeDeleteResponse = unknown`

# Ingresses

## Create an ingress rule

**post** `/zones/{zone_id}/intel/sinkholes/{sinkhole_id}/ingresses`

Create a new ingress rule for the specified sinkhole. The CIDR block must be a Cloudflare BYOIP associated with your account. The zone_id must be a zone with the ability to create Spectrum Apps. The sinkhole must belong to the same account as the zone.

### Path Parameters

- `zone_id: string`

  Identifier.

- `sinkhole_id: string`

### Body Parameters

- `cidr: string`

  The CIDR block for the ingress rule in IPv4 or IPv6 notation (e.g., 192.0.2.0/24). Must be a Cloudflare BYOIP associated with your account.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { id, cidr, created_on, 3 more }`

  - `id: optional string`

    The unique identifier for the ingress rule.

  - `cidr: optional string`

    The CIDR block for the ingress rule.

  - `created_on: optional string`

    The date and time when the ingress rule was created.

  - `modified_on: optional string`

    The date and time when the ingress rule was last modified.

  - `sinkhole_id: optional string`

    The sinkhole this ingress rule belongs to.

  - `zone_tag: optional string`

    The zone tag associated with this ingress rule.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/intel/sinkholes/$SINKHOLE_ID/ingresses \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "cidr": "cidr"
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
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": {
    "id": "de32ae5203724ed08dcc26e971a4d22f",
    "cidr": "192.0.2.0/24",
    "created_on": "2023-06-01T10:00:00Z",
    "modified_on": "2023-06-15T14:30:00Z",
    "sinkhole_id": "93defa6e909e464e8c89a85859f36d3c",
    "zone_tag": "4c961e9d94f40aa922775483b9ee18cf"
  }
}
```

## Get an ingress rule

**get** `/zones/{zone_id}/intel/sinkholes/{sinkhole_id}/ingresses/{ingress_id}`

Get the specified ingress rule associated with a sinkhole. The sinkhole must belong to the same account as the zone.

### Path Parameters

- `zone_id: string`

  Identifier.

- `sinkhole_id: string`

- `ingress_id: string`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { id, cidr, created_on, 3 more }`

  - `id: optional string`

    The unique identifier for the ingress rule.

  - `cidr: optional string`

    The CIDR block for the ingress rule.

  - `created_on: optional string`

    The date and time when the ingress rule was created.

  - `modified_on: optional string`

    The date and time when the ingress rule was last modified.

  - `sinkhole_id: optional string`

    The sinkhole this ingress rule belongs to.

  - `zone_tag: optional string`

    The zone tag associated with this ingress rule.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/intel/sinkholes/$SINKHOLE_ID/ingresses/$INGRESS_ID \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "success": true,
  "result": {
    "id": "de32ae5203724ed08dcc26e971a4d22f",
    "cidr": "192.0.2.0/24",
    "created_on": "2023-06-01T10:00:00Z",
    "modified_on": "2023-06-15T14:30:00Z",
    "sinkhole_id": "93defa6e909e464e8c89a85859f36d3c",
    "zone_tag": "4c961e9d94f40aa922775483b9ee18cf"
  }
}
```

## Update an ingress rule

**put** `/zones/{zone_id}/intel/sinkholes/{sinkhole_id}/ingresses/{ingress_id}`

Update the specified ingress rule. The sinkhole must belong to the same account as the zone.

### Path Parameters

- `zone_id: string`

  Identifier.

- `sinkhole_id: string`

- `ingress_id: string`

### Body Parameters

- `cidr: string`

  The CIDR block for the ingress rule in IPv4 or IPv6 notation (e.g., 192.0.2.0/24). Must be a Cloudflare BYOIP associated with your account.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/intel/sinkholes/$SINKHOLE_ID/ingresses/$INGRESS_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "cidr": "cidr"
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
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": {}
}
```

## Delete an ingress rule

**delete** `/zones/{zone_id}/intel/sinkholes/{sinkhole_id}/ingresses/{ingress_id}`

Delete the specified ingress rule. The sinkhole must belong to the same account as the zone.

### Path Parameters

- `zone_id: string`

  Identifier.

- `sinkhole_id: string`

- `ingress_id: string`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/intel/sinkholes/$SINKHOLE_ID/ingresses/$INGRESS_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "success": true,
  "result": {}
}
```

## Domain Types

### Ingress Create Response

- `IngressCreateResponse object { id, cidr, created_on, 3 more }`

  - `id: optional string`

    The unique identifier for the ingress rule.

  - `cidr: optional string`

    The CIDR block for the ingress rule.

  - `created_on: optional string`

    The date and time when the ingress rule was created.

  - `modified_on: optional string`

    The date and time when the ingress rule was last modified.

  - `sinkhole_id: optional string`

    The sinkhole this ingress rule belongs to.

  - `zone_tag: optional string`

    The zone tag associated with this ingress rule.

### Ingress Get Response

- `IngressGetResponse object { id, cidr, created_on, 3 more }`

  - `id: optional string`

    The unique identifier for the ingress rule.

  - `cidr: optional string`

    The CIDR block for the ingress rule.

  - `created_on: optional string`

    The date and time when the ingress rule was created.

  - `modified_on: optional string`

    The date and time when the ingress rule was last modified.

  - `sinkhole_id: optional string`

    The sinkhole this ingress rule belongs to.

  - `zone_tag: optional string`

    The zone tag associated with this ingress rule.

### Ingress Update Response

- `IngressUpdateResponse = unknown`

### Ingress Delete Response

- `IngressDeleteResponse = unknown`
