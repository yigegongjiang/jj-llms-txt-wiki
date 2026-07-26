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
