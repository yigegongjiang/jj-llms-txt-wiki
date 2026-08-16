# Quota

## Get Custom Hostname Quota

**get** `/zones/{zone_id}/custom_hostnames/quota`

Returns custom hostname quota usage for a zone. The allocated quota is a soft limit; creating custom hostnames after usage exceeds this limit can still succeed until the hard cap is reached. Use the exceeded and hard_cap fields to track when usage is above the soft limit and when new custom hostname creation will be rejected.

### Path Parameters

- `zone_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of string`

  Informational messages returned by the custom hostname API.

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { allocated, exceeded, hard_cap, used }`

  - `allocated: number`

    The allocated custom hostname quota.

  - `exceeded: boolean`

    Whether the current usage has exceeded the allocated quota.

  - `hard_cap: number`

    The maximum number of custom hostnames allowed before create requests are rejected.

  - `used: number`

    The number of custom hostnames currently in use.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/custom_hostnames/quota \
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
    "allocated": 100,
    "exceeded": false,
    "hard_cap": 200,
    "used": 50
  }
}
```

## Domain Types

### Quota Get Response

- `QuotaGetResponse object { allocated, exceeded, hard_cap, used }`

  - `allocated: number`

    The allocated custom hostname quota.

  - `exceeded: boolean`

    Whether the current usage has exceeded the allocated quota.

  - `hard_cap: number`

    The maximum number of custom hostnames allowed before create requests are rejected.

  - `used: number`

    The number of custom hostnames currently in use.
