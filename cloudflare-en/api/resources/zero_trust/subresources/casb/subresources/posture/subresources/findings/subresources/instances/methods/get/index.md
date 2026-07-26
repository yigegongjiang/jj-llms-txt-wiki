## Get a finding instance using an instance ID

**get** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances/{instance_id}`

Gets a security Finding instance by id.

### Path Parameters

- `account_id: string`

- `finding_id: string`

- `instance_id: string`

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

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { affliction_date, asset, dlp_contexts, 4 more }`

  A specific instance of a security finding. In the API interface, we refer to the 'finding' table in our DB as finding instances, optimized for the p99 use case.

  - `affliction_date: string`

    When this specific instance was identified.

  - `asset: object { category, external_id, fields, 3 more }`

    Asset information including metadata and categorization.

    - `category: object { service, type, vendor, id }`

      Category information for an asset.

      - `service: string`

        The specific service within the vendor the asset is part of (often none). Example - AWS is the vendor, S3 is the service.

      - `type: string`

        The type of asset.

      - `vendor: string`

        The vendor the asset is part of.

      - `id: optional string`

        Unique identifier for the asset category.

    - `external_id: string`

      External identifier from the source system.

    - `fields: array of object { name, value, link }`

      The fields associated with the asset.

      - `name: string`

        The name of the field.

      - `value: string`

        The value of the field.

      - `link: optional string`

        Optional link associated with the field.

    - `name: string`

      Human-readable name of the asset.

    - `id: optional string`

      Unique identifier for the asset.

    - `link: optional string`

      Direct link to the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information if this is a content finding.

    - `created: string`

      When the DLP context was created.

    - `entry_ids: array of string`

      DLP Entry IDs.

    - `profile_id: string`

      DLP Profile ID.

    - `updated: string`

      When the DLP context was last updated.

    - `id: optional string`

      Unique identifier for the DLP context.

    - `deleted: optional string`

      When the DLP context was deleted.

    - `match_context_max_extent: optional number`

      DLP Right Boundary of match context.

    - `match_context_min_extent: optional number`

      DLP Left Boundary of match context.

    - `match_context_payload: optional map[unknown]`

      DLP Match context payload that matched the profile in question.

  - `remediations: array of object { id, created_at, stale, status }`

    A list of the 10 most recent remediation jobs for this finding instance, ordered by creation time (most recent first). The 'stale' field indicates whether the remediation job was created before the finding instance's affliction_date (true) or after it (false). If there has never been a remediation job for this finding instance, this field will be an empty array.

    - `id: string`

      Unique identifier for the remediation job.

    - `created_at: string`

      When the remediation job was created.

    - `stale: boolean`

      Whether this remediation job is stale (created before the finding instance's affliction_date).

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

  - `webhooks: array of object { latest_job, webhook_id, webhook_label }`

    The most recent webhook job invocation for each webhook configuration associated with this finding instance. Each entry represents the latest job (any status) per webhook config. The 'stale' field indicates whether the job was invoked before the finding instance's current affliction_date. If no webhook jobs have been created, this field will be an empty array.

    - `latest_job: object { id, created_at, stale, status }`

      The most recent webhook job for this webhook configuration.

      - `id: string`

        Unique identifier for the webhook job.

      - `created_at: string`

        When the webhook job was created.

      - `stale: boolean`

        Whether this webhook job is stale (created before the finding instance's current affliction_date).

      - `status: "pending" or "processing" or "completed"`

        Current status of the webhook job.

        - `"pending"`

        - `"processing"`

        - `"completed"`

    - `webhook_id: string`

      Unique identifier for the webhook configuration.

    - `webhook_label: string`

      Account-specified display label for the webhook configuration.

  - `id: optional string`

    Unique identifier for the finding instance.

  - `is_archived: optional boolean`

    Whether this finding instance has been archived.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances/$INSTANCE_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
  "success": true,
  "result": {
    "affliction_date": "2025-03-18T17:25:38.700541Z",
    "asset": {
      "category": {
        "service": null,
        "type": "file",
        "vendor": "Slack",
        "id": "1a78cbf3-b98f-4289-b1f2-22db64130f4f"
      },
      "external_id": "external-file-id-123",
      "fields": [
        {
          "name": "Credential name",
          "value": "Test asset 2",
          "link": "https://example.com"
        }
      ],
      "name": "Public.svg",
      "id": "8a043daf-def4-403e-9d28-da2e93d9b824",
      "link": "https://slack-files.com/TYJH37DCK-E0238GG6B8-92fd5y5674"
    },
    "dlp_contexts": [
      {
        "created": "2025-03-18T17:25:38.695977Z",
        "entry_ids": [
          "21befc68-a297-4090-ac10-17a051b901cd",
          "d6dd1e16-f78c-401a-b564-45c4e44aa467"
        ],
        "profile_id": "ab20a60b-21f2-4b13-ac98-24dcee27ac0e",
        "updated": "2025-03-18T17:25:38.695977Z",
        "id": "7653ff3a-d25e-4c10-8034-3460937c045b",
        "deleted": "2025-03-18T17:25:38.695977Z",
        "match_context_max_extent": 512,
        "match_context_min_extent": 1,
        "match_context_payload": {}
      }
    ],
    "remediations": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "created_at": "2025-03-18T18:30:15.123456Z",
        "stale": false,
        "status": "pending"
      }
    ],
    "webhooks": [
      {
        "latest_job": {
          "id": "123e4567-e89b-12d3-a456-426614174000",
          "created_at": "2025-03-18T18:30:15.123456Z",
          "stale": false,
          "status": "pending"
        },
        "webhook_id": "550e8400-e29b-41d4-a716-446655440000",
        "webhook_label": "Send to Gmail"
      }
    ],
    "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
    "is_archived": false
  }
}
```
