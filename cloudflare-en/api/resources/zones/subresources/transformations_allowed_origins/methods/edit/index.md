## Change Image Transformations Allowed Origins setting

**patch** `/zones/{zone_id}/settings/transformations_allowed_origins`

Media Transformations Allowed Origins restricts transformations for images and video served through
Cloudflare's network to requests originating from specified domains. Refer to the
Image Transformations and Video Transformations documentation for more information.

### Path Parameters

- `zone_id: string`

  Identifier.

### Body Parameters

- `value: string`

  Comma-separated list of allowed origin domains for image and video transformations.
  Use "*" to allow all origins (default).

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional TransformationsAllowedOrigins`

  Controls which origins are allowed to request image and video transformations.

  - `id: optional "image_resizing_allowed_origins"`

    ID of the zone setting.

    - `"image_resizing_allowed_origins"`

  - `editable: optional true or false`

    Whether or not this setting can be modified for this zone (based on your Cloudflare plan level).

    - `true`

    - `false`

  - `modified_on: optional string`

    last time this setting was modified.

  - `value: optional "on" or "off"`

    Comma-separated list of allowed origin domains for image and video transformations.
    Use "*" to allow all origins (default).

    - `"on"`

    - `"off"`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/settings/transformations_allowed_origins \
    -X PATCH \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "value": "example.com,cdn.example.com"
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
    "id": "image_resizing_allowed_origins",
    "editable": true,
    "modified_on": "2014-01-01T05:20:00.12345Z",
    "value": "on"
  }
}
```
