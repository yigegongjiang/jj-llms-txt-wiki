## Get finding by ID

**get** `/accounts/{account_id}/data-security/posture/finding_types/{finding_type_id}`

Retrieve a specific finding type by its unique identifier.

### Path Parameters

- `account_id: string`

- `finding_type_id: string`

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

- `result: optional object { id, category, name, 2 more }`

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

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/finding_types/$FINDING_TYPE_ID \
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
    "id": "a20895dd-9c3b-43bd-a608-71c98c6c2d94",
    "category": {
      "observation": "Issue",
      "product": "SaaS",
      "type": "Posture"
    },
    "name": "Slack File Publicly Accessible",
    "severity": "High",
    "vendor": "Google Workspace"
  }
}
```
