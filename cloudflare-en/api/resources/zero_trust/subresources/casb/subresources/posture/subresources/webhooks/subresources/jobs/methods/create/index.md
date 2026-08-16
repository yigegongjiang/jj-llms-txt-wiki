## Create webhook jobs

**post** `/accounts/{account_id}/data-security/posture/webhooks/jobs`

Creates webhook jobs to send a finding instance to one or more configured webhooks.

### Path Parameters

- `account_id: string`

### Body Parameters

- `finding_instance_ids: array of string`

  Array of finding instance IDs to send to the webhooks.

- `webhook_ids: array of string`

  Array of webhook IDs to trigger jobs for.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

    Error or message code.

  - `message: string`

    Human-readable message.

  - `documentation_url: optional string`

    Link to relevant documentation.

  - `source: optional object { pointer }`

    - `pointer: optional string`

      JSON pointer to the source of the error.

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

    Error or message code.

  - `message: string`

    Human-readable message.

  - `documentation_url: optional string`

    Link to relevant documentation.

  - `source: optional object { pointer }`

    - `pointer: optional string`

      JSON pointer to the source of the error.

- `result: object { created, failed }`

  - `created: array of object { id, asset_data, created_at, 9 more }`

    Successfully created webhook jobs.

    - `id: string`

      Unique identifier for the webhook job.

    - `asset_data: map[unknown]`

      Asset data associated with this webhook job.

    - `created_at: string`

      When the webhook job was created.

    - `integration_id: string`

      ID of the integration.

    - `last_updated_at: string`

      When the webhook job was last updated.

    - `parameters: object { finding_instance_id }`

      Parameters for a webhook job.

      - `finding_instance_id: string`

        ID of the finding instance.

    - `status: "pending" or "processing" or "completed" or "failed"`

      Status of a webhook job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

    - `triggered_by_actor: "user" or "account_token"`

      Type of actor that triggered the webhook job.

      - `"user"`

      - `"account_token"`

    - `triggered_by_id: string`

      ID of the actor that triggered the job.

    - `webhook_id: string`

      ID of the webhook configuration.

    - `failure_details: optional map[unknown]`

      Additional details about the failure.

    - `failure_reason: optional "Permission Denied" or "Integration Unavailable" or "Service Temporarily Unavailable" or "System Error"`

      Reason for webhook job failure.

      - `"Permission Denied"`

      - `"Integration Unavailable"`

      - `"Service Temporarily Unavailable"`

      - `"System Error"`

  - `failed: array of object { error, finding_instance_id, webhook_id }`

    Failed webhook job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a webhook job.

    - `webhook_id: string`

      ID of the webhook configuration.

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/jobs \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "finding_instance_ids": [
            "770e8400-e29b-41d4-a716-446655440002",
            "660e8400-e29b-41d4-a716-446655440001"
          ],
          "webhook_ids": [
            "550e8400-e29b-41d4-a716-446655440000",
            "660e8400-e29b-41d4-a716-446655440001"
          ]
        }'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "Request processed successfully",
      "documentation_url": "https://developers.cloudflare.com/api/operations/list-findings",
      "source": {
        "pointer": "/data/attributes/name"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "Request processed successfully",
      "documentation_url": "https://developers.cloudflare.com/api/operations/list-findings",
      "source": {
        "pointer": "/data/attributes/name"
      }
    }
  ],
  "result": {
    "created": [
      {
        "id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
        "asset_data": {
          "foo": "bar"
        },
        "created_at": "2025-07-07T18:39:13.123456Z",
        "integration_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "last_updated_at": "2025-07-07T18:39:13.123456Z",
        "parameters": {
          "finding_instance_id": "3f7b8c9d-6e5a-4f3b-9c2d-1e0a8b7c6d5e"
        },
        "status": "pending",
        "triggered_by_actor": "user",
        "triggered_by_id": "user@example.com",
        "webhook_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "failure_details": {
          "foo": "bar"
        },
        "failure_reason": "Permission Denied"
      }
    ],
    "failed": [
      {
        "error": "Failed to create webhook job",
        "finding_instance_id": "2e6b4c8a-9d1f-4e3b-8c7a-5f9e2d1a6b4c",
        "webhook_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e"
      }
    ]
  },
  "success": true
}
```
