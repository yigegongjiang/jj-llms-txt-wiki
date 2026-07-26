## Creates remediation jobs

**post** `/accounts/{account_id}/data-security/posture/remediations/jobs`

Create one or more remediation jobs tied to a specific Cloudflare Account.

### Path Parameters

- `account_id: string`

### Body Parameters

- `finding_instance_ids: array of string`

  UUIDs identifying Finding Instances.

- `remediation_type_id: string`

  A UUID identifying this Remediation Type.

### Returns

- `errors: array of unknown`

  Array of error messages.

- `messages: array of unknown`

  Array of informational messages.

- `result: object { created, failed }`

  - `created: array of object { id, asset, created_at, 11 more }`

    Successfully created remediation jobs.

    - `id: string`

      Unique identifier for the remediation job.

    - `asset: object { id, category, external_id, 3 more }`

      Asset information for a remediation job.

      - `id: string`

        Unique identifier for the asset.

      - `category: object { service, type, vendor }`

        Category information for a remediation job asset.

        - `service: string`

          Specific service within the vendor.

        - `type: string`

          Asset type.

        - `vendor: "AWS" or "Anthropic" or "Bitbucket" or 16 more`

          Display names for vendor types.

          - `"AWS"`

          - `"Anthropic"`

          - `"Bitbucket"`

          - `"Box"`

          - `"Confluence"`

          - `"Dropbox"`

          - `"GitHub"`

          - `"Google Cloud Platform"`

          - `"Google Workspace"`

          - `"Jira"`

          - `"Microsoft"`

          - `"Microsoft Internal"`

          - `"Okta"`

          - `"OpenAI"`

          - `"Slack"`

          - `"Salesforce"`

          - `"ServiceNow"`

          - `"Workday"`

          - `"Zoom"`

      - `external_id: string`

        External identifier from the source system.

      - `fields: array of object { name, value, link }`

        Additional fields associated with the asset.

        - `name: string`

          Field name.

        - `value: string or number or boolean`

          Field value (can be string, number, or boolean).

          - `string`

          - `number`

          - `boolean`

        - `link: optional string`

          Optional link associated with the field.

      - `name: string`

        Human-readable name of the asset.

      - `link: optional string`

        Direct link to the asset.

    - `created_at: string`

      When the remediation job was created.

    - `finding_id: string`

      Encoded finding ID.

    - `finding_instance_id: string`

      ID of the finding instance being remediated.

    - `finding_type_id: string`

      ID of the finding type.

    - `finding_type_name: string`

      Name of the finding type.

    - `integration_name: string`

      Name of the integration.

    - `last_updated: string`

      When the remediation job was last updated.

    - `remediation_type: string`

      Type of remediation being performed.

    - `status: "pending" or "processing" or "completed" or 2 more`

      Status of a remediation job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

      - `"validating"`

    - `triggered_by_user: string`

      Email of the user who triggered the remediation. For account-token actors this is the literal "Account API Token"; for policy actors this is empty.

    - `triggered_by_actor: optional "user" or "account_token"`

      Type of actor that triggered the remediation job. Null on legacy rows created before this column was populated.

      - `"user"`

      - `"account_token"`

    - `triggered_by_id: optional string`

      ID of the actor that triggered the job. Meaning depends on triggered_by_actor. Null on legacy rows.

  - `failed: array of object { error, finding_instance_id }`

    Failed remediation job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a remediation job.

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/remediations/jobs \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "finding_instance_ids": [
            "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e"
          ],
          "remediation_type_id": "5a7d9e2f-1b3c-4d5e-8f6a-7b8c9d0e1f2a"
        }'
```

#### Response

```json
{
  "errors": [
    {}
  ],
  "messages": [
    {}
  ],
  "result": {
    "created": [
      {
        "id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
        "asset": {
          "id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
          "category": {
            "service": "OneDrive",
            "type": "SaaS",
            "vendor": "Google Workspace"
          },
          "external_id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
          "fields": [
            {
              "name": "File Name",
              "value": "sensitive-document.xlsx",
              "link": "https://dashboard.microsoft.com/files/details"
            }
          ],
          "name": "Microsoft File Publicly Accessible",
          "link": "https://dashboard.microsoft.com/files/details"
        },
        "created_at": "2025-07-07T18:39:13.123456Z",
        "finding_id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
        "finding_instance_id": "3f7b8c9d-6e5a-4f3b-9c2d-1e0a8b7c6d5e",
        "finding_type_id": "775c5f38-efcf-4b2b-93db-8428979eb6a2",
        "finding_type_name": "Microsoft: File publicly accessible with edit access",
        "integration_name": "Microsoft",
        "last_updated": "2025-07-07T18:39:13.123456Z",
        "remediation_type": "Remove publicly accessible edit url",
        "status": "pending",
        "triggered_by_user": "user@example.com",
        "triggered_by_actor": "user",
        "triggered_by_id": "0123456789abcdef0123456789abcdef"
      }
    ],
    "failed": [
      {
        "error": "Failed to create remediation job",
        "finding_instance_id": "2e6b4c8a-9d1f-4e3b-8c7a-5f9e2d1a6b4c"
      }
    ]
  },
  "success": true
}
```
