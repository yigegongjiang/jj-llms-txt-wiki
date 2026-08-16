## List AI Search instances.

**get** `/accounts/{account_id}/ai-search/instances`

List all AI Search instances in the account.

### Path Parameters

- `account_id: string`

### Query Parameters

- `namespace: optional string`

  Filter by namespace.

- `order_by: optional "created_at"`

  Field to order results by.

  - `"created_at"`

- `order_by_direction: optional "asc" or "desc"`

  Order direction.

  - `"asc"`

  - `"desc"`

- `page: optional number`

  Page number (1-indexed).

- `per_page: optional number`

  Number of results per page.

- `search: optional string`

  Filter instances whose id contains this string (case-insensitive).

### Returns

- `result: array of object { id, ai_gateway_id, ai_search_model, 42 more }`

  - `id: string`

  - `ai_gateway_id: string`

  - `ai_search_model: string`

  - `cache: boolean`

  - `cache_threshold: "super_strict_match" or "close_enough" or "flexible_friend" or "anything_goes"`

    - `"super_strict_match"`

    - `"close_enough"`

    - `"flexible_friend"`

    - `"anything_goes"`

  - `cache_ttl: 600 or 1800 or 3600 or 7 more`

    - `600`

    - `1800`

    - `3600`

    - `7200`

    - `21600`

    - `43200`

    - `86400`

    - `172800`

    - `259200`

    - `518400`

  - `chunk: boolean`

  - `chunk_overlap: number`

  - `chunk_size: number`

  - `created_at: string`

  - `created_by: string`

  - `custom_metadata: array of object { data_type, field_name }`

    - `data_type: "text" or "number" or "boolean" or "datetime"`

      - `"text"`

      - `"number"`

      - `"boolean"`

      - `"datetime"`

    - `field_name: string`

  - `embedding_model: string`

  - `enable: boolean`

  - `engine_version: number`

  - `fusion_method: "max" or "rrf"`

    - `"max"`

    - `"rrf"`

  - `hybrid_search_enabled: boolean`

  - `index_method: object { keyword, vector }`

    - `keyword: boolean`

    - `vector: boolean`

  - `indexing_options: object { keyword_tokenizer }`

    - `keyword_tokenizer: optional "porter" or "trigram"`

      - `"porter"`

      - `"trigram"`

  - `last_activity: string`

  - `max_num_results: number`

  - `metadata: object { created_from_aisearch_wizard, worker_domain }`

    - `created_from_aisearch_wizard: optional boolean`

    - `worker_domain: optional string`

  - `modified_at: string`

  - `modified_by: string`

  - `namespace: string`

  - `paused: boolean`

  - `public_endpoint_id: string`

  - `public_endpoint_params: object { authorized_hosts, chat_completions_endpoint, custom_domains, 5 more }`

    - `authorized_hosts: optional array of string`

    - `chat_completions_endpoint: optional object { disabled }`

      - `disabled: optional boolean`

    - `custom_domains: optional array of string`

    - `default_domain_enabled: optional boolean`

    - `enabled: optional boolean`

    - `mcp: optional object { description, disabled }`

      - `description: optional string`

      - `disabled: optional boolean`

    - `rate_limit: optional object { period_ms, requests, technique }`

      - `period_ms: optional number`

      - `requests: optional number`

      - `technique: optional "fixed" or "sliding"`

        - `"fixed"`

        - `"sliding"`

    - `search_endpoint: optional object { disabled }`

      - `disabled: optional boolean`

  - `reranking: boolean`

  - `reranking_model: string`

  - `retrieval_options: object { boost_by, keyword_match_mode }`

    - `boost_by: optional array of object { field, dataType, direction }`

      - `field: string`

      - `dataType: optional "number" or "datetime" or "text" or "boolean"`

        - `"number"`

        - `"datetime"`

        - `"text"`

        - `"boolean"`

      - `direction: optional "asc" or "desc" or "exists" or "not_exists"`

        - `"asc"`

        - `"desc"`

        - `"exists"`

        - `"not_exists"`

    - `keyword_match_mode: optional "and" or "or"`

      - `"and"`

      - `"or"`

  - `rewrite_model: string`

  - `rewrite_query: boolean`

  - `score_threshold: number`

  - `source: string`

  - `source_params: object { exclude_items, include_items, prefix, 2 more }`

    - `exclude_items: optional array of string`

    - `include_items: optional array of string`

    - `prefix: optional string`

    - `r2_jurisdiction: optional string`

    - `web_crawler: optional object { discover_options, parse_options, parse_type }`

      - `discover_options: optional object { depth, include_external_links, include_subdomains, 3 more }`

        - `depth: optional number`

        - `include_external_links: optional boolean`

        - `include_subdomains: optional boolean`

        - `limit: optional number`

          Maximum number of pages to crawl. New values are capped at 100000; instances configured before that cap may report a higher stored value, which the crawler clamps at run time.

        - `max_age: optional number`

        - `source: optional "all" or "sitemaps" or "links"`

          - `"all"`

          - `"sitemaps"`

          - `"links"`

      - `parse_options: optional object { content_selector, include_headers, include_images, 2 more }`

        - `content_selector: optional array of object { path, selector }`

          - `path: string`

          - `selector: string`

        - `include_headers: optional map[string]`

        - `include_images: optional boolean`

        - `specific_sitemaps: optional array of string`

        - `use_browser_rendering: optional boolean`

      - `parse_type: optional "sitemap" or "discover"`

        - `"sitemap"`

        - `"discover"`

  - `status: string`

  - `summarization: boolean`

  - `summarization_model: string`

  - `sync_interval: 900 or 1800 or 3600 or 5 more`

    - `900`

    - `1800`

    - `3600`

    - `7200`

    - `14400`

    - `21600`

    - `43200`

    - `86400`

  - `system_prompt_ai_search: string`

  - `system_prompt_index_summarization: string`

  - `system_prompt_rewrite_query: string`

  - `token_id: string`

  - `type: "r2" or "web-crawler"`

    - `"r2"`

    - `"web-crawler"`

- `result_info: object { count, page, per_page, total_count }`

  - `count: number`

  - `page: number`

  - `per_page: number`

  - `total_count: number`

- `success: true`

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/ai-search/instances \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": [
    {
      "id": "id",
      "ai_gateway_id": "ai_gateway_id",
      "ai_search_model": "ai_search_model",
      "cache": true,
      "cache_threshold": "super_strict_match",
      "cache_ttl": 600,
      "chunk": true,
      "chunk_overlap": 0,
      "chunk_size": 0,
      "created_at": "2019-12-27T18:11:19.117Z",
      "created_by": "created_by",
      "custom_metadata": [
        {
          "data_type": "text",
          "field_name": "field_name"
        }
      ],
      "embedding_model": "embedding_model",
      "enable": true,
      "engine_version": 0,
      "fusion_method": "max",
      "hybrid_search_enabled": true,
      "index_method": {
        "keyword": true,
        "vector": true
      },
      "indexing_options": {
        "keyword_tokenizer": "porter"
      },
      "last_activity": "2019-12-27T18:11:19.117Z",
      "max_num_results": 0,
      "metadata": {
        "created_from_aisearch_wizard": true,
        "worker_domain": "worker_domain"
      },
      "modified_at": "2019-12-27T18:11:19.117Z",
      "modified_by": "modified_by",
      "namespace": "namespace",
      "paused": true,
      "public_endpoint_id": "public_endpoint_id",
      "public_endpoint_params": {
        "authorized_hosts": [
          "string"
        ],
        "chat_completions_endpoint": {
          "disabled": true
        },
        "custom_domains": [
          "x"
        ],
        "default_domain_enabled": true,
        "enabled": true,
        "mcp": {
          "description": "description",
          "disabled": true
        },
        "rate_limit": {
          "period_ms": 60000,
          "requests": 1,
          "technique": "fixed"
        },
        "search_endpoint": {
          "disabled": true
        }
      },
      "reranking": true,
      "reranking_model": "reranking_model",
      "retrieval_options": {
        "boost_by": [
          {
            "field": "x",
            "dataType": "number",
            "direction": "asc"
          }
        ],
        "keyword_match_mode": "and"
      },
      "rewrite_model": "rewrite_model",
      "rewrite_query": true,
      "score_threshold": 0,
      "source": "source",
      "source_params": {
        "exclude_items": [
          "string"
        ],
        "include_items": [
          "string"
        ],
        "prefix": "prefix",
        "r2_jurisdiction": "r2_jurisdiction",
        "web_crawler": {
          "discover_options": {
            "depth": 1,
            "include_external_links": true,
            "include_subdomains": true,
            "limit": 1,
            "max_age": 0,
            "source": "all"
          },
          "parse_options": {
            "content_selector": [
              {
                "path": "x",
                "selector": "x"
              }
            ],
            "include_headers": {
              "foo": "string"
            },
            "include_images": true,
            "specific_sitemaps": [
              "https://example.com"
            ],
            "use_browser_rendering": true
          },
          "parse_type": "sitemap"
        }
      },
      "status": "status",
      "summarization": true,
      "summarization_model": "summarization_model",
      "sync_interval": 900,
      "system_prompt_ai_search": "system_prompt_ai_search",
      "system_prompt_index_summarization": "system_prompt_index_summarization",
      "system_prompt_rewrite_query": "system_prompt_rewrite_query",
      "token_id": "token_id",
      "type": "r2"
    }
  ],
  "result_info": {
    "count": 0,
    "page": 0,
    "per_page": 0,
    "total_count": 0
  },
  "success": true
}
```
