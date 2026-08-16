## Upload asset

**post** `/pages/assets/upload`

Upload one or more files to the Pages asset store. Each file is
identified by its content hash and is uploaded using the same JSON shape
as the Cloudflare KV bulk write API. Used as part of the Pages Direct
Upload workflow.

Authenticate with the JWT obtained from the upload-token endpoint:
GET /accounts/{account_id}/pages/projects/{project_name}/upload-token

### Body Parameters

- `body: array of object { base64, key, metadata, value }`

  - `base64: boolean`

    Whether value is base64 encoded.

  - `key: string`

    File content hash used as the object key in the Pages asset store.

  - `metadata: object { contentType }`

    - `contentType: string`

      MIME type for the uploaded file.

  - `value: string`

    File content. When base64 is true, this value is base64 encoded.

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

### Example

```http
curl https://api.cloudflare.com/client/v4/pages/assets/upload \
    -H 'Content-Type: application/json' \
    -d '[
          {
            "base64": true,
            "key": "b026324c6904b2a9cb4b88d6d61c81d1",
            "metadata": {
              "contentType": "text/plain"
            },
            "value": "SGVsbG8sIFdvcmxkIQ=="
          }
        ]'
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
  "success": true
}
```
