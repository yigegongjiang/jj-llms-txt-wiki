## Delete a WAF override

**delete** `/zones/{zone_id}/firewall/waf/overrides/{overrides_id}`

**This endpoint has been deprecated and returns 410 Gone. Please use the [Rulesets API](https://developers.cloudflare.com/ruleset-engine/) instead.**

Previously deleted an existing URI-based WAF override.

### Path Parameters

- `zone_id: string`

  Defines an identifier.

- `overrides_id: string`

  The unique identifier of the WAF override.

### Returns

- `result: optional object { id }`

  - `id: optional string`

    The unique identifier of the WAF override.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/firewall/waf/overrides/$OVERRIDES_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "id": "de677e5818985db1285d0e80225f06e5"
  }
}
```
