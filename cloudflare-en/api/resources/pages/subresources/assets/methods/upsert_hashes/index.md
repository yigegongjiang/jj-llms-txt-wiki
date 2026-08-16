## Upsert asset hashes

**post** `/pages/assets/upsert-hashes`

Register the provided file hashes as recently uploaded to the Pages
asset store. Used as part of the Pages Direct Upload workflow so future
deployments can avoid re-uploading files that are already present.

Authenticate with the JWT obtained from the upload-token endpoint:
GET /accounts/{account_id}/pages/projects/{project_name}/upload-token

### Body Parameters

- `hashes: array of string`

  List of file content hashes to register in the asset store.

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
curl https://api.cloudflare.com/client/v4/pages/assets/upsert-hashes \
    -H 'Content-Type: application/json' \
    -d '{
          "hashes": [
            "a948904f2f0f479b8f936b8a0c5d9882",
            "b026324c6904b2a9cb4b88d6d61c81d1"
          ]
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
  "success": true
}
```
