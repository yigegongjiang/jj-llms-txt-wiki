# Robots

## Get robots.txt rules

**get** `/zones/{zone_id}/ai-audit/robots`

Fetches and parses the robots.txt file for a zone or a specific subdomain within the zone. Returns parsed user-agent rules, content signals, and sitemaps.

### Path Parameters

- `zone_id: string`

### Query Parameters

- `subdomain: optional string`

  Optional subdomain to fetch robots.txt for. If omitted, fetches robots.txt for the zone apex domain.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

- `success: boolean`

- `result: optional object { userAgents, sitemaps, status }`

  Parsed robots.txt rules for a single domain.

  - `userAgents: map[object { allow, disallow, contentSignals, crawlDelay } ]`

    Map of user-agent string to its parsed rules.

    - `allow: array of string`

      List of allowed path patterns.

    - `disallow: array of string`

      List of disallowed path patterns.

    - `contentSignals: optional object { "ai-input", "ai-train", search }`

      Content signal directives from robots.txt.

      - `"ai-input": optional "yes" or "no"`

        Whether AI input usage is permitted.

        - `"yes"`

        - `"no"`

      - `"ai-train": optional "yes" or "no"`

        Whether AI training is permitted.

        - `"yes"`

        - `"no"`

      - `search: optional "yes" or "no"`

        Whether search indexing is permitted.

        - `"yes"`

        - `"no"`

    - `crawlDelay: optional number`

      Crawl delay in seconds.

  - `sitemaps: optional array of string`

    List of sitemap URLs found in robots.txt.

  - `status: optional number`

    HTTP status code from fetching the robots.txt file.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/ai-audit/robots \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
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
    "userAgents": {
      "foo": {
        "allow": [
          "string"
        ],
        "disallow": [
          "string"
        ],
        "contentSignals": {
          "ai-input": "yes",
          "ai-train": "yes",
          "search": "yes"
        },
        "crawlDelay": 0
      }
    },
    "sitemaps": [
      "string"
    ],
    "status": 0
  }
}
```

## Bulk get robots.txt rules

**post** `/zones/{zone_id}/ai-audit/robots/bulk`

Fetches and parses robots.txt files for multiple domains within a zone in a single request. Each domain must belong to the specified zone. Results are keyed by hostname.

### Path Parameters

- `zone_id: string`

### Body Parameters

- `body: array of string`

  Array of domain hostnames to fetch robots.txt for. Each domain must end with the zone name. Maximum 25 domains per request.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

- `success: boolean`

- `result: optional map[object { userAgents, sitemaps, status } ]`

  Map of hostname to parsed robots.txt rules.

  - `userAgents: map[object { allow, disallow, contentSignals, crawlDelay } ]`

    Map of user-agent string to its parsed rules.

    - `allow: array of string`

      List of allowed path patterns.

    - `disallow: array of string`

      List of disallowed path patterns.

    - `contentSignals: optional object { "ai-input", "ai-train", search }`

      Content signal directives from robots.txt.

      - `"ai-input": optional "yes" or "no"`

        Whether AI input usage is permitted.

        - `"yes"`

        - `"no"`

      - `"ai-train": optional "yes" or "no"`

        Whether AI training is permitted.

        - `"yes"`

        - `"no"`

      - `search: optional "yes" or "no"`

        Whether search indexing is permitted.

        - `"yes"`

        - `"no"`

    - `crawlDelay: optional number`

      Crawl delay in seconds.

  - `sitemaps: optional array of string`

    List of sitemap URLs found in robots.txt.

  - `status: optional number`

    HTTP status code from fetching the robots.txt file.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/ai-audit/robots/bulk \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '[
          "example.com",
          "blog.example.com"
        ]'
```

#### Response

```json
{
  "errors": [],
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
    "foo": {
      "userAgents": {
        "foo": {
          "allow": [
            "string"
          ],
          "disallow": [
            "string"
          ],
          "contentSignals": {
            "ai-input": "yes",
            "ai-train": "yes",
            "search": "yes"
          },
          "crawlDelay": 0
        }
      },
      "sitemaps": [
        "string"
      ],
      "status": 0
    }
  }
}
```

## Domain Types

### Robot Get Response

- `RobotGetResponse object { userAgents, sitemaps, status }`

  Parsed robots.txt rules for a single domain.

  - `userAgents: map[object { allow, disallow, contentSignals, crawlDelay } ]`

    Map of user-agent string to its parsed rules.

    - `allow: array of string`

      List of allowed path patterns.

    - `disallow: array of string`

      List of disallowed path patterns.

    - `contentSignals: optional object { "ai-input", "ai-train", search }`

      Content signal directives from robots.txt.

      - `"ai-input": optional "yes" or "no"`

        Whether AI input usage is permitted.

        - `"yes"`

        - `"no"`

      - `"ai-train": optional "yes" or "no"`

        Whether AI training is permitted.

        - `"yes"`

        - `"no"`

      - `search: optional "yes" or "no"`

        Whether search indexing is permitted.

        - `"yes"`

        - `"no"`

    - `crawlDelay: optional number`

      Crawl delay in seconds.

  - `sitemaps: optional array of string`

    List of sitemap URLs found in robots.txt.

  - `status: optional number`

    HTTP status code from fetching the robots.txt file.

### Robot Bulk Get Response

- `RobotBulkGetResponse = map[object { userAgents, sitemaps, status } ]`

  Map of hostname to parsed robots.txt rules.

  - `userAgents: map[object { allow, disallow, contentSignals, crawlDelay } ]`

    Map of user-agent string to its parsed rules.

    - `allow: array of string`

      List of allowed path patterns.

    - `disallow: array of string`

      List of disallowed path patterns.

    - `contentSignals: optional object { "ai-input", "ai-train", search }`

      Content signal directives from robots.txt.

      - `"ai-input": optional "yes" or "no"`

        Whether AI input usage is permitted.

        - `"yes"`

        - `"no"`

      - `"ai-train": optional "yes" or "no"`

        Whether AI training is permitted.

        - `"yes"`

        - `"no"`

      - `search: optional "yes" or "no"`

        Whether search indexing is permitted.

        - `"yes"`

        - `"no"`

    - `crawlDelay: optional number`

      Crawl delay in seconds.

  - `sitemaps: optional array of string`

    List of sitemap URLs found in robots.txt.

  - `status: optional number`

    HTTP status code from fetching the robots.txt file.
