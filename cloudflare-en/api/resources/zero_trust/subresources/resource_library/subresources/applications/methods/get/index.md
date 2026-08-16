## Get application

**get** `/accounts/{account_id}/resource-library/applications/{id}`

Get application by ID.

### Path Parameters

- `account_id: string`

- `id: number`

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

  Indicates whether the API call was successful.

  - `true`

- `result: optional object { id, application_confidence_score, application_source, 15 more }`

  - `id: number`

    Returns the application ID.

  - `application_confidence_score: number`

    Confidence score for the application. Returns -1 when no score is available.

  - `application_source: string`

    Returns the application source.

  - `application_type: string`

    Returns the application type.

  - `application_type_description: string`

    Returns the application type description.

  - `category_id: number`

    Returns the category ID.

  - `created_at: string`

    Returns the application creation time.

  - `gen_ai_score: number`

    GenAI score for the application. Returns -1 when no score is available.

  - `hostnames: array of string`

    Hostnames matched by the application.

  - `human_id: string`

    Returns the human readable ID.

  - `ip_subnets: array of string`

    IP subnets matched by the application.

  - `name: string`

    Returns the application name.

  - `port_protocols: array of string`

    Port and protocol pairs matched by the application.

  - `support_domains: array of string`

    Support domains matched by the application.

  - `supported: array of "GATEWAY" or "ACCESS" or "CASB"`

    Cloudflare products that support this application.

    - `"GATEWAY"`

    - `"ACCESS"`

    - `"CASB"`

  - `updated_at: string`

    Returns the application update time.

  - `version: string`

    Returns the application version.

  - `application_score_composition: optional unknown`

    Returns the score composition breakdown for the application.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/resource-library/applications/$ID \
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
  "result": {
    "id": 498,
    "application_confidence_score": 0.92,
    "application_source": "cloudflare",
    "application_type": "Human Resources",
    "application_type_description": "Applications used to manage employees and workforce tools.",
    "category_id": 12,
    "created_at": "2025-01-01T00:00:00Z",
    "gen_ai_score": 1.5,
    "hostnames": [
      "example.com",
      "foo.com"
    ],
    "human_id": "HR",
    "ip_subnets": [
      "192.168.1.0/24",
      "10.0.0.0/8"
    ],
    "name": "HR",
    "port_protocols": [
      "tcp/80",
      "tcp/443"
    ],
    "support_domains": [
      "example.com",
      "foo.com"
    ],
    "supported": [
      "GATEWAY",
      "ACCESS"
    ],
    "updated_at": "2025-01-01T00:00:00Z",
    "version": "2025-01-01T00:00:00Z",
    "application_score_composition": {
      "categories": [
        {
          "confidence": 0.95,
          "name": "Security"
        }
      ],
      "plan": "free"
    }
  }
}
```
