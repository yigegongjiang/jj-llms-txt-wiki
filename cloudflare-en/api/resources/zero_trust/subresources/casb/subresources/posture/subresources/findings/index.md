# Findings

## List posture findings

**get** `/accounts/{account_id}/data-security/posture/findings`

List all security findings that have been identified as being problematic.
This will return a list of findings regardless if they have been ignored or not.

### Path Parameters

- `account_id: string`

### Query Parameters

- `cursor: optional string`

  A cursor for pagination. Obtained from the `result_info.cursor` field of a previous response.

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `finding_type_ids: optional string`

  A comma separated list of UUIDs identifying the finding type(s).

- `ignored: optional boolean`

  Filter for only the ignored findings. Set to false to only see "active" items

- `integration_id: optional string`

  Filter by an integration ID

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `observation: optional "Activity" or "Insight" or "Issue"`

  Filter by observation type of the finding

  - `"Activity"`

  - `"Insight"`

  - `"Issue"`

- `order: optional "finding.name" or "instance_count" or "integration.name" or 2 more`

  Which field to use when ordering the findings.

  - `"finding.name"`

  - `"instance_count"`

  - `"integration.name"`

  - `"latest_affliction_date"`

  - `"severity"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `product: optional "Cloud" or "Saas"`

  Filter by product category of the finding

  - `"Cloud"`

  - `"Saas"`

- `search: optional string`

  A search term.

- `severity: optional "Critical" or "High" or "Medium" or "Low"`

  Filter by severity

  - `"Critical"`

  - `"High"`

  - `"Medium"`

  - `"Low"`

- `type: optional "Content" or "Posture"`

  Filter by type of the finding

  - `"Content"`

  - `"Posture"`

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

- `result: optional array of object { id, active_count, archived_count, 6 more }`

  Array of finding objects.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings \
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
      "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
      "active_count": 5,
      "archived_count": 2,
      "finding": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "category": {
          "observation": "Issue",
          "product": "SaaS",
          "type": "Posture"
        },
        "name": "Slack File Publicly Accessible",
        "severity": "High",
        "vendor": "Google Workspace",
        "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
        "remediation": {
          "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
          "frameworks": [
            "SOC2",
            "ISO27001"
          ],
          "guide": "To fix this issue, update the file permissions to remove public access.",
          "impact": "Publicly accessible files may expose sensitive information.",
          "locale": "en-US",
          "threat": "Data exposure and potential compliance violations."
        }
      },
      "ignored": false,
      "instance_count": 7,
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
      "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
      "severity_override": {
        "created_by": "1234",
        "severity": "Critical"
      }
    }
  ]
}
```

## Get a finding type

**get** `/accounts/{account_id}/data-security/posture/findings/{finding_id}`

Gets a security Finding that has been identified as being problematic.

### Path Parameters

- `account_id: string`

- `finding_id: string`

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

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID \
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
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
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
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Create new findings export request

**post** `/accounts/{account_id}/data-security/posture/findings/export`

Creates a CSV export for findings and accepts optional filters in the payload.

### Path Parameters

- `account_id: string`

### Body Parameters

- `ignored: optional boolean`

  Filter for only the ignored findings. Set to false to only see active items.

- `integration_id: optional array of string`

  Filter by multiple integration IDs.

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `orders: optional array of object { direction, name }`

  Which fields to use when ordering the findings.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "instance_count" or "finding.name" or "integration.name" or 2 more`

    Which field to use when ordering the findings.

    - `"instance_count"`

    - `"finding.name"`

    - `"integration.name"`

    - `"latest_affliction_date"`

    - `"severity"`

- `product: optional "SaaS" or "Cloud"`

  Filter by finding's category product.

  - `"SaaS"`

  - `"Cloud"`

- `search: optional string`

  A search term.

- `severities: optional array of "CRITICAL" or "HIGH" or "MEDIUM" or "LOW"`

  Filter by severity levels.

  - `"CRITICAL"`

  - `"HIGH"`

  - `"MEDIUM"`

  - `"LOW"`

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/export \
    -X POST \
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

## Mark a finding as ignored

**post** `/accounts/{account_id}/data-security/posture/findings/ignore`

Given a list of findings, mark as ignored. Does nothing if Finding is already ignored.

### Path Parameters

- `account_id: string`

### Body Parameters

- `checks: array of string`

  A list of finding IDs to pass along.

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

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/ignore \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "checks": [
            "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo="
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
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
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
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Remove ignore marker from a finding

**post** `/accounts/{account_id}/data-security/posture/findings/unignore`

Ability to un-ignore a Finding if it's previously been ignored. Does nothing if the Finding is not ignored.

### Path Parameters

- `account_id: string`

### Body Parameters

- `checks: array of string`

  A list of finding IDs to pass along.

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

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/unignore \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "checks": [
            "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo="
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
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
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
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Update the severity for a finding

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/tune_finding_severity`

Update the severity of a Finding.
This will update the `severity_override` field on the Finding payload with the new severity value.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Body Parameters

- `new_severity: 1 or 2 or 3 or 4`

  The numeric severity value to apply to the finding.

  - `1`

  - `2`

  - `3`

  - `4`

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

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/tune_finding_severity \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "new_severity": 1
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
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
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
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Reset severity for a finding back to the default

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/reset_finding_severity`

If a Finding's severity has been changed, reset it back to default value.
Does nothing if no override exists.

### Path Parameters

- `account_id: string`

- `finding_id: string`

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

- `result: optional object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/reset_finding_severity \
    -X POST \
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
    "id": "MDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxOjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMgo=",
    "active_count": 5,
    "archived_count": 2,
    "finding": {
      "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
      "category": {
        "observation": "Issue",
        "product": "SaaS",
        "type": "Posture"
      },
      "name": "Slack File Publicly Accessible",
      "severity": "High",
      "vendor": "Google Workspace",
      "description": "This finding indicates that a file in your Slack workspace is publicly accessible.",
      "remediation": {
        "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
        "frameworks": [
          "SOC2",
          "ISO27001"
        ],
        "guide": "To fix this issue, update the file permissions to remove public access.",
        "impact": "Publicly accessible files may expose sensitive information.",
        "locale": "en-US",
        "threat": "Data exposure and potential compliance violations."
      }
    },
    "ignored": false,
    "instance_count": 7,
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
    "latest_affliction_date": "2025-03-18T17:25:38.700131Z",
    "severity_override": {
      "created_by": "1234",
      "severity": "Critical"
    }
  }
}
```

## Domain Types

### Finding List Response

- `FindingListResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Get Response

- `FindingGetResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Export Response

- `FindingExportResponse object { id, status, type, 5 more }`

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

### Finding Ignore Response

- `FindingIgnoreResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Unignore Response

- `FindingUnignoreResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Tune Severity Response

- `FindingTuneSeverityResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

### Finding Reset Severity Response

- `FindingResetSeverityResponse object { id, active_count, archived_count, 6 more }`

  Aggregated finding information with counts and metadata. This is optimized for list API queries and represents a finding along with its instance statistics.

  - `id: string`

    Base64 encoded identifier of the security finding.

  - `active_count: number`

    Number of active problematic instances identified in the security finding.

  - `archived_count: number`

    Number of archived instances identified in the security finding.

  - `finding: object { id, category, name, 4 more }`

    Basic finding type information.

    - `id: string`

      The unique identifier of the finding.

    - `category: object { observation, product, type }`

      Category information for a finding.

      - `observation: "Issue" or "Insight" or "Activity"`

        The type of the observation.

        - `"Issue"`

        - `"Insight"`

        - `"Activity"`

      - `product: "SaaS" or "Cloud"`

        The product category.

        - `"SaaS"`

        - `"Cloud"`

      - `type: "Content" or "Posture"`

        The type of the finding category.

        - `"Content"`

        - `"Posture"`

    - `name: string`

      The name of the finding.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

    - `vendor: string`

      The SaaS/Cloud vendor of the platform with which the finding is associated.

    - `description: optional string`

      Detailed description of the finding.

    - `remediation: optional object { id, frameworks, guide, 3 more }`

      Remediation guide information for a finding.

      - `id: string`

        Remediation Id.

      - `frameworks: array of string`

        Relevant Compliance Frameworks.

      - `guide: string`

        Remediation guide text.

      - `impact: string`

        Description of the potential impact.

      - `locale: string`

        I18N Locale.

      - `threat: string`

        Description of the threat.

  - `ignored: boolean`

    Determines if finding is currently ignored.

  - `instance_count: number`

    Number of total (Active or archived) problematic instances identified in the security finding.

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

    Timestamp of the latest affliction date of an active finding.

  - `severity_override: optional object { created_by, severity }`

    Override information for finding severity.

    - `created_by: string`

      User ID who created the override.

    - `severity: "Critical" or "High" or "Medium" or "Low"`

      The severity level of a finding.

      - `"Critical"`

      - `"High"`

      - `"Medium"`

      - `"Low"`

# Instances

## List instances of a finding

**get** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances`

Lists all security finding instances for a given security finding.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Query Parameters

- `archived: optional boolean`

  Archived

- `asset_ids: optional array of string`

  Filter finding instances by an array of asset IDs. Supports multiple comma-separated values.

- `cursor: optional string`

  A cursor for pagination. Obtained from the `result_info.cursor` field of a previous response.

- `direction: optional "asc" or "desc"`

  Direction to order results.

  - `"asc"`

  - `"desc"`

- `finding_instance_ids: optional array of string`

  Filter finding instances by an array of finding instance IDs. Supports multiple comma-separated values.

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `order: optional "affliction_date" or "asset.name" or "remediation.status"`

  Which field to use when ordering the Finding's instances.
  When ordering by 'remediation.status', only the most recent non-stale remediation job is considered. Stale jobs (created before the instance's affliction_date) are treated as having no status for ordering purposes.

  - `"affliction_date"`

  - `"asset.name"`

  - `"remediation.status"`

- `page: optional number`

  A page number within the paginated result set.

- `per_page: optional number`

  Number of results to return per page.

- `remediation_statuses: optional array of "none" or "pending" or "processing" or 3 more`

  Filter finding instances by most recent remediation job status. Supports multiple comma-separated values.
  Use 'none' to filter instances with no remediation jobs or instances where the most recent job is stale.
  Note: Stale jobs (created before the instance's affliction_date) are ignored for filtering purposes, but are still included in the 'remediations' array with stale=true.

  - `"none"`

  - `"pending"`

  - `"processing"`

  - `"validating"`

  - `"completed"`

  - `"failed"`

- `search: optional string`

  A search term.

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

- `result: optional array of object { affliction_date, asset, dlp_contexts, 4 more }`

  Array of finding instance objects.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances \
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
  ]
}
```

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

## Create a finding instances export

**post** `/accounts/{account_id}/data-security/posture/findings/{storage_namespace_id}/instances/export`

Creates a CSV export for Finding instances and accepts optional filters in the payload.

The `storage_namespace_id` path parameter is derived from the finding ID by base64-decoding it
(which yields `integration_id:finding_type_id`) and replacing the colon with a hyphen.

### Path Parameters

- `account_id: string`

- `storage_namespace_id: string`

### Body Parameters

- `archived: optional boolean`

  Filter for archived status.

- `max_affliction_date: optional string`

  Filter to view findings that occurred on or before the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `min_affliction_date: optional string`

  Filter to view findings that occurred on or after the affliction date. Can be a date-time in ISO 8601 format or an epoch timestamp.

- `orders: optional array of object { direction, name }`

  Ordering specifications for the export.

  - `direction: "asc" or "desc"`

    Sort direction.

    - `"asc"`

    - `"desc"`

  - `name: "asset.name" or "affliction_date"`

    Which field to use when ordering the finding instances.

    - `"asset.name"`

    - `"affliction_date"`

- `search: optional string`

  A search term.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$STORAGE_NAMESPACE_ID/instances/export \
    -X POST \
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

## Archive a finding

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances/archive`

Archive one or more finding instances.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Body Parameters

- `check_instances: array of string`

  A list of finding instance IDs to pass along.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances/archive \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "check_instances": [
            "497f6eca-6276-4993-bfeb-53cbbbba6f08"
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

## Remove the archive marking from a finding instance

**post** `/accounts/{account_id}/data-security/posture/findings/{finding_id}/instances/unarchive`

Remove the archive marking from one or more finding instances.

### Path Parameters

- `account_id: string`

- `finding_id: string`

### Body Parameters

- `check_instances: array of string`

  A list of finding instance IDs to pass along.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/findings/$FINDING_ID/instances/unarchive \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "check_instances": [
            "497f6eca-6276-4993-bfeb-53cbbbba6f08"
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

## Domain Types

### Instance List Response

- `InstanceListResponse object { affliction_date, asset, dlp_contexts, 4 more }`

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

### Instance Get Response

- `InstanceGetResponse object { affliction_date, asset, dlp_contexts, 4 more }`

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

### Instance Export Response

- `InstanceExportResponse object { id, status, type, 5 more }`

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

### Instance Archive Response

- `InstanceArchiveResponse object { affliction_date, asset, dlp_contexts, 4 more }`

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

### Instance Unarchive Response

- `InstanceUnarchiveResponse object { affliction_date, asset, dlp_contexts, 4 more }`

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
