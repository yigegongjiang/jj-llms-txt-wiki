## List tagged resources

**get** `/accounts/{account_id}/tags/resources`

Lists all tagged resources for an account.

### Path Parameters

- `account_id: string`

  Identifier.

### Query Parameters

- `id: optional array of string`

  Filter by resource ID. Can be repeated up to 50 times to filter by multiple IDs. Example: ?id=abc&id=def

- `cursor: optional string`

  Cursor for pagination.

- `name: optional string`

  Filter by resource name. Performs a case-insensitive substring match. Example: ?name=my-zone

- `tag: optional array of string`

  Filter resources by tag criteria. This parameter can be repeated multiple times, with AND logic between parameters.

  Supported syntax:

  - **Key-only**: `tag=<key>` - Resource must have the tag key (e.g., `tag=production`)
  - **Key-value**: `tag=<key>=<value>` - Resource must have the tag with specific value (e.g., `tag=env=prod`)
  - **Multiple values (OR)**: `tag=<key>=<v1>,<v2>` - Resource must have tag with any of the values (e.g., `tag=env=prod,staging`)
  - **Negate key-only**: `tag=!<key>` - Resource must not have the tag key (e.g., `tag=!archived`)
  - **Negate key-value**: `tag=<key>!=<value>` - Resource must not have the tag with specific value (e.g., `tag=region!=us-west-1`)

  Multiple tag parameters are combined with AND logic.

- `type: optional array of "access_application" or "access_application_policy" or "access_group" or 34 more`

  Filter by resource type. Can be repeated to filter by multiple types (OR logic). Example: ?type=zone&type=worker

  - `"access_application"`

  - `"access_application_policy"`

  - `"access_group"`

  - `"account"`

  - `"account_ruleset"`

  - `"ai_gateway"`

  - `"alerting_policy"`

  - `"alerting_webhook"`

  - `"api_gateway_operation"`

  - `"cloudflared_tunnel"`

  - `"custom_certificate"`

  - `"custom_hostname"`

  - `"d1_database"`

  - `"dns_record"`

  - `"durable_object_namespace"`

  - `"gateway_list"`

  - `"gateway_rule"`

  - `"healthcheck"`

  - `"image"`

  - `"infrastructure_target"`

  - `"kv_namespace"`

  - `"load_balancer"`

  - `"load_balancer_monitor"`

  - `"load_balancer_pool"`

  - `"managed_client_certificate"`

  - `"pages_project"`

  - `"queue"`

  - `"r2_bucket"`

  - `"resource_share"`

  - `"stream_live_input"`

  - `"stream_video"`

  - `"vectorize_index"`

  - `"worker"`

  - `"worker_route"`

  - `"worker_version"`

  - `"zone"`

  - `"zone_ruleset"`

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

  Whether the API call was successful.

  - `true`

- `result: optional array of object { id, etag, name, 2 more }  or object { id, access_application_id, etag, 4 more }  or object { id, etag, name, 2 more }  or 34 more`

  - `AccessApplication object { id, etag, name, 2 more }`

    Response for access_application resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "access_application"`

      - `"access_application"`

  - `AccessApplicationPolicy object { id, access_application_id, etag, 4 more }`

    Response for access_application_policy resources

    - `id: string`

      Identifies the unique resource.

    - `access_application_id: string`

      Access application ID is required only for access_application_policy resources

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "access_application_policy"`

      - `"access_application_policy"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `AccessGroup object { id, etag, name, 2 more }`

    Response for access_group resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "access_group"`

      - `"access_group"`

  - `Account object { id, etag, name, 2 more }`

    Response for account resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "account"`

      - `"account"`

  - `AccountRuleset object { id, etag, name, 2 more }`

    Response for account_ruleset resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "account_ruleset"`

      - `"account_ruleset"`

  - `AIGateway object { id, etag, name, 2 more }`

    Response for ai_gateway resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "ai_gateway"`

      - `"ai_gateway"`

  - `AlertingPolicy object { id, etag, name, 2 more }`

    Response for alerting_policy resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "alerting_policy"`

      - `"alerting_policy"`

  - `AlertingWebhook object { id, etag, name, 2 more }`

    Response for alerting_webhook resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "alerting_webhook"`

      - `"alerting_webhook"`

  - `APIGatewayOperation object { id, etag, name, 3 more }`

    Response for api_gateway_operation resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "api_gateway_operation"`

      - `"api_gateway_operation"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `CloudflaredTunnel object { id, etag, name, 2 more }`

    Response for cloudflared_tunnel resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "cloudflared_tunnel"`

      - `"cloudflared_tunnel"`

  - `CustomCertificate object { id, etag, name, 3 more }`

    Response for custom_certificate resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "custom_certificate"`

      - `"custom_certificate"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `CustomHostname object { id, etag, name, 3 more }`

    Response for custom_hostname resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "custom_hostname"`

      - `"custom_hostname"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `D1Database object { id, etag, name, 2 more }`

    Response for d1_database resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "d1_database"`

      - `"d1_database"`

  - `DNSRecord object { id, etag, name, 3 more }`

    Response for dns_record resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "dns_record"`

      - `"dns_record"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `DurableObjectNamespace object { id, etag, name, 2 more }`

    Response for durable_object_namespace resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "durable_object_namespace"`

      - `"durable_object_namespace"`

  - `GatewayList object { id, etag, name, 2 more }`

    Response for gateway_list resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "gateway_list"`

      - `"gateway_list"`

  - `GatewayRule object { id, etag, name, 2 more }`

    Response for gateway_rule resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "gateway_rule"`

      - `"gateway_rule"`

  - `Healthcheck object { id, etag, name, 3 more }`

    Response for healthcheck resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "healthcheck"`

      - `"healthcheck"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `Image object { id, etag, name, 2 more }`

    Response for image resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "image"`

      - `"image"`

  - `InfrastructureTarget object { id, etag, name, 2 more }`

    Response for infrastructure_target resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "infrastructure_target"`

      - `"infrastructure_target"`

  - `KVNamespace object { id, etag, name, 2 more }`

    Response for kv_namespace resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "kv_namespace"`

      - `"kv_namespace"`

  - `LoadBalancer object { id, etag, name, 3 more }`

    Response for load_balancer resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "load_balancer"`

      - `"load_balancer"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `LoadBalancerMonitor object { id, etag, name, 2 more }`

    Response for load_balancer_monitor resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "load_balancer_monitor"`

      - `"load_balancer_monitor"`

  - `LoadBalancerPool object { id, etag, name, 2 more }`

    Response for load_balancer_pool resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "load_balancer_pool"`

      - `"load_balancer_pool"`

  - `ManagedClientCertificate object { id, etag, name, 3 more }`

    Response for managed_client_certificate resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "managed_client_certificate"`

      - `"managed_client_certificate"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `PagesProject object { id, etag, name, 2 more }`

    Response for pages_project resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "pages_project"`

      - `"pages_project"`

  - `Queue object { id, etag, name, 2 more }`

    Response for queue resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "queue"`

      - `"queue"`

  - `R2Bucket object { id, etag, name, 2 more }`

    Response for r2_bucket resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "r2_bucket"`

      - `"r2_bucket"`

  - `ResourceShare object { id, etag, name, 2 more }`

    Response for resource_share resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "resource_share"`

      - `"resource_share"`

  - `StreamLiveInput object { id, etag, name, 2 more }`

    Response for stream_live_input resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "stream_live_input"`

      - `"stream_live_input"`

  - `StreamVideo object { id, etag, name, 2 more }`

    Response for stream_video resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "stream_video"`

      - `"stream_video"`

  - `VectorizeIndex object { id, etag, name, 2 more }`

    Response for vectorize_index resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "vectorize_index"`

      - `"vectorize_index"`

  - `Worker object { id, etag, name, 2 more }`

    Response for worker resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "worker"`

      - `"worker"`

  - `WorkerRoute object { id, etag, name, 3 more }`

    Response for worker_route resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "worker_route"`

      - `"worker_route"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `WorkerVersion object { id, etag, name, 3 more }`

    Response for worker_version resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "worker_version"`

      - `"worker_version"`

    - `worker_id: string`

      Worker ID is required only for worker_version resources

  - `Zone object { id, etag, name, 3 more }`

    Response for zone resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "zone"`

      - `"zone"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

  - `ZoneRuleset object { id, etag, name, 3 more }`

    Response for zone_ruleset resources

    - `id: string`

      Identifies the unique resource.

    - `etag: string`

      ETag identifier for optimistic concurrency control. Formatted as "v1:<hash>" where
      the hash is the base64url-encoded SHA-256 (truncated to 128 bits) of the tags map
      canonicalized using RFC 8785 (JSON Canonicalization Scheme). Clients should treat
      ETags as opaque strings and pass them back via the If-Match header on write operations.

    - `name: string`

      Human-readable name of the resource.

    - `tags: map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

    - `type: "zone_ruleset"`

      - `"zone_ruleset"`

    - `zone_id: string`

      Zone ID is required only for zone-level resources

- `result_info: optional object { count, cursor }`

  - `count: optional number`

    Indicates the number of results returned in the current page.

  - `cursor: optional string`

    Provides a cursor for the next page of results. Include this value in the next request to continue pagination.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/tags/resources \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "result": [
    {
      "id": "023e105f4ecef8ad9ca31a8372d0c353",
      "etag": "v1:RBNvo1WzZ4oRRq0W9-hkng",
      "name": "my-worker-script",
      "tags": {
        "environment": "production",
        "team": "engineering"
      },
      "type": "access_application"
    }
  ],
  "result_info": {
    "count": 20,
    "cursor": "eyJhY2NvdW50X2lkIjoxMjM0NTY3ODkwfQ"
  }
}
```
