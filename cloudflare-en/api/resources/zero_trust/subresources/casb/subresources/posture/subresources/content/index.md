# Content

## List DLP content findings

**get** `/accounts/{account_id}/data-security/posture/content`

List DLP content findings

### Path Parameters

- `account_id: string`

### Query Parameters

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `dlp_profile_id: optional string`

  Filter by an DLP profile ID

- `integration_id: optional string`

  Filter by an integration ID

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `order: optional "asset_name" or "dlp_profile_count" or "integration_name" or "latest_affliction_date"`

  Which field to use when ordering content assets.

  - `"asset_name"`

  - `"dlp_profile_count"`

  - `"integration_name"`

  - `"latest_affliction_date"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `search: optional string`

  A search term.

- `vendor: optional "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter by vendor

  - `"ANTHROPIC"`

  - `"AWS"`

  - `"BITBUCKET"`

  - `"BOX"`

  - `"CONFLUENCE"`

  - `"DROPBOX"`

  - `"GITHUB"`

  - `"GOOGLE_CLOUD_PLATFORM"`

  - `"GOOGLE_WORKSPACE"`

  - `"JIRA"`

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `result_info: object { count, cursor, next, 4 more }`

  Pagination and result information.

  - `count: optional number`

    Total number of results for the requested service.

  - `cursor: optional string`

    Cursor for cursor-based pagination.

  - `next: optional string`

    URL to the next page of results.

  - `page: optional number`

    Current page within paginated list of results.

  - `per_page: optional number`

    Number of results per page of results.

  - `previous: optional string`

    URL to the previous page of results.

  - `total_count: optional number`

    Total results available without any search parameters.

- `success: boolean`

  Whether the API call was successful.

- `result: optional array of object { asset_id, asset_name, dlp_contexts, 4 more }`

  Array of content asset objects.

  - `asset_id: string`

    Unique identifier for the asset.

  - `asset_name: string`

    Name of the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information for this asset.

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

  - `dlp_profile_count: number`

    Number of DLP profiles that flagged this asset.

  - `dlp_profile_ids: array of string`

    IDs of DLP profiles that flagged this asset.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Most recent date this asset was flagged.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/content \
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
  "result_info": {
    "count": 1,
    "cursor": "eyJpZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCIsImFmZmxpY3Rpb25fZGF0ZSI6IjE5NzAtMDEtMDFUMDA6MDA6MDAuMDAwMDAwWiJ9",
    "next": "https://example.com",
    "page": 1,
    "per_page": 20,
    "previous": "https://example.com",
    "total_count": 2000
  },
  "success": true,
  "result": [
    {
      "asset_id": "e6910838-4b91-45e9-b2b4-91bb23cb9762",
      "asset_name": "Test Asset Name",
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
      "dlp_profile_count": 2,
      "dlp_profile_ids": [
        "c12f2059-8df4-43f8-9eb9-d27112d92b63",
        "822c051b-0bb4-4747-8929-471a1d506eef"
      ],
      "integration": {
        "created": "2021-08-10T20:16:11.851451Z",
        "last_hydrated": "2025-03-18T17:25:38.697894Z",
        "name": "Example integration",
        "permissions": [
          "GroupMember.Read.All",
          "Group.Read.All"
        ],
        "policy": {
          "id": "d647642e-09ac-4b34-8acc-ac30f57adc2c",
          "client_id": "client_id",
          "compliance_level": "standard",
          "dlp_enabled": true,
          "link": "https://example.com",
          "name": "Google Workspace Standard Policy",
          "permissions": [
            "https://www.googleapis.com/auth/admin.directory.domain.readonly",
            "https://www.googleapis.com/auth/admin.directory.user.readonly"
          ]
        },
        "status": "Healthy",
        "updated": "2021-08-10T20:16:11.851451Z",
        "upgradable": false,
        "vendor": {
          "id": "R09PR0xFX1dPUktTUEFDRQ==",
          "description": "Identify important security issues across your Google Workspace account ranging from shadow IT, misconfigurations, user access, and more.",
          "display_name": "Google Workspace",
          "logo": "https://cdn.vectrix-infra.com/DetectionPack_Logos/GoogleWorkspace/g.png",
          "name": "GOOGLE_WORKSPACE",
          "static_logo": "https://onprem.cloudflare.come/DetectionPack_Logos/GoogleWorkspace/g.png",
          "zt_enrollments": [
            "casb"
          ],
          "policies": [
            {
              "foo": "bar"
            }
          ]
        },
        "zt_enrollments": [
          {
            "id": "casb",
            "description": "example",
            "display_name": "Cloud Access Security Broker",
            "enabled": true
          }
        ],
        "id": "c416bc38-75dc-425f-ae25-c37b5df5c37f",
        "credential_health_status": "Healthy",
        "credentials_expiry": "2025-03-18T17:25:38.697902Z",
        "is_paused": false,
        "upgrade_dismissed": false
      },
      "latest_affliction_date": "2024-10-18T19:53:57.626659Z"
    }
  ]
}
```

## Create a content export

**post** `/accounts/{account_id}/data-security/posture/content/export`

Creates a CSV export for content and accepts optional filters in the payload.

### Path Parameters

- `account_id: string`

### Body Parameters

- `dlp_profile_information: array of object { id, entries, name }`

  DLP profile metadata for the export.

  - `id: string`

    Unique identifier for the DLP profile.

  - `entries: array of object { id, name, profile_id }`

    Entries contained within this DLP profile.

    - `id: string`

      Unique identifier for the DLP profile entry.

    - `name: string`

      Name of the DLP profile entry.

    - `profile_id: string`

      ID of the parent DLP profile.

  - `name: string`

    Name of the DLP profile.

- `dlp_profile_id: optional array of string`

  Filter by DLP profile IDs.

- `integration_id: optional array of string`

  Filter by integration IDs.

- `max_affliction_date: optional string`

  Filter to view content flagged on or before this date.

- `min_affliction_date: optional string`

  Filter to view content flagged on or after this date.

- `orders: optional array of object { direction, name }`

  Ordering specifications for the export.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "asset_name" or "dlp_profile_count" or "integration_name" or "latest_affliction_date"`

    Content-specific field names for ordering.

    - `"asset_name"`

    - `"dlp_profile_count"`

    - `"integration_name"`

    - `"latest_affliction_date"`

- `search: optional string`

  Search term to filter content.

- `vendors: optional array of "ANTHROPIC" or "AWS" or "BITBUCKET" or 13 more`

  Filter by vendor types.

  - `"ANTHROPIC"`

  - `"AWS"`

  - `"BITBUCKET"`

  - `"BOX"`

  - `"CONFLUENCE"`

  - `"DROPBOX"`

  - `"GITHUB"`

  - `"GOOGLE_CLOUD_PLATFORM"`

  - `"GOOGLE_WORKSPACE"`

  - `"JIRA"`

  - `"MICROSOFT"`

  - `"MICROSOFT_INTERNAL"`

  - `"OPENAI"`

  - `"SALESFORCE"`

  - `"SERVICENOW"`

  - `"SLACK"`

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

- `result: optional object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/content/export \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "dlp_profile_information": [
            {
              "id": "e91a2360-da51-4fdf-9711-bcdecd462614",
              "entries": [
                {
                  "id": "55ba2c6c-8ef4-4b2e-9148-e75e8b6ccac1",
                  "name": "Credit Card Numbers",
                  "profile_id": "e91a2360-da51-4fdf-9711-bcdecd462614"
                }
              ],
              "name": "Financial Information"
            }
          ],
          "dlp_profile_id": [
            "e91a2360-da51-4fdf-9711-bcdecd462614"
          ],
          "integration_id": [
            "c416bc38-75dc-425f-ae25-c37b5df5c37f"
          ],
          "max_affliction_date": "2024-01-01T00:00:00Z",
          "min_affliction_date": "2023-01-01T00:00:00Z",
          "search": "sensitive",
          "vendors": [
            "GOOGLE_WORKSPACE"
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
  "success": true,
  "result": {
    "id": "45ce02c2-e797-4a71-98cb-937244352fd4",
    "status": "Success",
    "type": "finding",
    "user_id": "e7712d506b1ee4c5ede0802815f55a75",
    "download_url": "https://example.com/45ce02c2-e797-4a71-98cb-937244352fd4",
    "errors": null,
    "file_name": "findings_export_2024-02-27.csv",
    "file_path": "/exports/finding-instances/2024/02/27/Finding_Instances_2024-02-27T04:05:26Z.csv"
  }
}
```

## Domain Types

### Content List Response

- `ContentListResponse object { asset_id, asset_name, dlp_contexts, 4 more }`

  Content asset with DLP information.

  - `asset_id: string`

    Unique identifier for the asset.

  - `asset_name: string`

    Name of the asset.

  - `dlp_contexts: array of object { created, entry_ids, profile_id, 6 more }`

    DLP context information for this asset.

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

  - `dlp_profile_count: number`

    Number of DLP profiles that flagged this asset.

  - `dlp_profile_ids: array of string`

    IDs of DLP profiles that flagged this asset.

  - `integration: object { created, last_hydrated, name, 12 more }`

    Summary information about an integration.

    - `created: string`

      When entity was created.

    - `last_hydrated: string`

      When were the integration credentials last updated.

    - `name: string`

      Name of the integration.

    - `permissions: array of string`

      The vendor-specific permissions associated with the integration.

    - `policy: object { id, client_id, compliance_level, 4 more }`

      Policy configuration for an integration.

      - `id: optional string`

        Policy identifier.

      - `client_id: optional string`

        OAuth client ID for the policy.

      - `compliance_level: optional string`

        Compliance level for the policy.

      - `dlp_enabled: optional boolean`

        Whether DLP is enabled for this policy.

      - `link: optional string`

        Link to policy documentation.

      - `name: optional string`

        Policy name.

      - `permissions: optional array of string`

        List of permissions included in the policy.

    - `status: string`

      Current status of the integration.

    - `updated: string`

      Last entity was updated.

    - `upgradable: boolean`

      Whether the integrations permissions can be updated.

    - `vendor: object { id, description, display_name, 5 more }`

      Information about a vendor/service provider.

      - `id: string`

        The id of the vendor.

      - `description: string`

        Detailed information about what kinds of issues are detected for this vendor.

      - `display_name: string`

        The display name of the vendor.

      - `logo: string`

        Logo URL for the vendor.

      - `name: string`

        The name of the vendor.

      - `static_logo: string`

        Static logo URL for the vendor.

      - `zt_enrollments: array of string`

        The vendor's compatible Zero Trust products.

      - `policies: optional array of map[unknown]`

        The policies related to the vendor.

    - `zt_enrollments: array of object { id, description, display_name, enabled }`

      Zero Trust products associated with this integration.

      - `id: optional string`

        The internal identifier of the Zero Trust Product.

      - `description: optional string`

        Brief description of the Zero Trust Product.

      - `display_name: optional string`

        The verbose name of the Zero Trust Product.

      - `enabled: optional boolean`

        Flag to enable/disable access to the listed integration from the corresponding Cloudflare product.

    - `id: optional string`

      Integration ID.

    - `credential_health_status: optional "Initializing" or "Healthy" or "Unhealthy"`

      Health status of integration credentials.

      - `"Initializing"`

      - `"Healthy"`

      - `"Unhealthy"`

    - `credentials_expiry: optional string`

      The date and time when the integration credentials will expire.

    - `is_paused: optional boolean`

      Whether the given integration is paused by the user.

    - `upgrade_dismissed: optional boolean`

      UI State as to whether a potential permissions upgrade has been dismissed.

  - `latest_affliction_date: string`

    Most recent date this asset was flagged.

### Content Export Response

- `ContentExportResponse object { id, status, type, 5 more }`

  Information about an export job.

  - `id: string`

    Unique identifier for the export job.

  - `status: "Pending" or "Success" or "Failure" or 2 more`

    Status of an export job.

    - `"Pending"`

    - `"Success"`

    - `"Failure"`

    - `"Rescheduled"`

    - `"In-Progress"`

  - `type: "finding" or "findingInstance" or "content" or "remediationJob"`

    Type of export job.

    - `"finding"`

    - `"findingInstance"`

    - `"content"`

    - `"remediationJob"`

  - `user_id: string`

    ID of the export-requesting user.

  - `download_url: optional string`

    The URL by which the successfully created export can be downloaded by the end users.

  - `errors: optional string`

    Contains information on errors which may have occurred during export creation.

  - `file_name: optional string`

    The base name of the file that is/was generated by the export job.

  - `file_path: optional string`

    The full path of the file that is stored within external storage (currently R2).
