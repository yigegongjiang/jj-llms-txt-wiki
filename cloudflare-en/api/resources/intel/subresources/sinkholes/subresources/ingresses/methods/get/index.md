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
