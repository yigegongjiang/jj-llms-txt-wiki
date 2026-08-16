# Transformations

## List Image Resizing configurations for account

**get** `/accounts/{account_id}/settings/transformations`

Returns a list of Image Resizing configurations across all zones for the account.
This endpoint is useful for retrieving the transformations (image_resizing) state
for all zones belonging to an account.

### Path Parameters

- `account_id: string`

  Identifier.

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

- `result: optional array of TransformationsConfig`

  - `id: optional string`

    Feature identifier.

  - `cf_zone_tag: optional string`

    Zone tag identifier.

  - `editable: optional boolean`

    Whether this setting can be modified.

  - `modified_on: optional string`

    When this setting was last modified.

  - `value: optional string`

    Current value of the feature setting.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/settings/transformations \
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
  "success": true,
  "result": [
    {
      "id": "image_resizing",
      "cf_zone_tag": "023e105f4ecef8ad9ca31a8372d0c353",
      "editable": true,
      "modified_on": "2024-01-15T10:30:00Z",
      "value": "on"
    }
  ]
}
```

## Domain Types

### Transformations Config

- `TransformationsConfig object { id, cf_zone_tag, editable, 2 more }`

  A configuration item for a specific zone and feature.

  - `id: optional string`

    Feature identifier.

  - `cf_zone_tag: optional string`

    Zone tag identifier.

  - `editable: optional boolean`

    Whether this setting can be modified.

  - `modified_on: optional string`

    When this setting was last modified.

  - `value: optional string`

    Current value of the feature setting.
