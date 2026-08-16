## Change Image Transformations C2PA setting

**patch** `/zones/{zone_id}/settings/transformations_c2pa`

C2PA (Coalition for Content Provenance and Authenticity) signing adds cryptographic metadata
to images processed through Cloudflare Image Transformations, enabling verification of image
authenticity and provenance.

### Path Parameters

- `zone_id: string`

  Identifier.

### Body Parameters

- `value: "off" or "on"`

  Whether C2PA signing is enabled for image transformations.

  - `"off"`

  - `"on"`

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

- `result: optional TransformationsC2pa`

  Controls C2PA signing for images processed through Cloudflare Image Transformations.

  - `id: optional "image_resizing_c2pa"`

    ID of the zone setting.

    - `"image_resizing_c2pa"`

  - `editable: optional true or false`

    Whether or not this setting can be modified for this zone (based on your Cloudflare plan level).

    - `true`

    - `false`

  - `modified_on: optional string`

    last time this setting was modified.

  - `value: optional "on" or "off"`

    Current value of the zone setting.

    - `"on"`

    - `"off"`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/settings/transformations_c2pa \
    -X PATCH \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "value": "off"
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
    "id": "image_resizing_c2pa",
    "editable": true,
    "modified_on": "2014-01-01T05:20:00.12345Z",
    "value": "on"
  }
}
```
