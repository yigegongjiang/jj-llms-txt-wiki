# Zone Tags

## Get tags for a zone-level resource

**get** `/zones/{zone_id}/tags`

Retrieves tags for a specific zone-level resource.

### Path Parameters

- `zone_id: string`

  Zone ID is required only for zone-level resources

### Query Parameters

- `resource_id: string`

  The ID of the resource to retrieve tags for.

- `resource_type: "access_application_policy" or "api_gateway_operation" or "custom_certificate" or 8 more`

  The type of the resource.

  - `"access_application_policy"`

  - `"api_gateway_operation"`

  - `"custom_certificate"`

  - `"custom_hostname"`

  - `"dns_record"`

  - `"healthcheck"`

  - `"load_balancer"`

  - `"managed_client_certificate"`

  - `"worker_route"`

  - `"zone"`

  - `"zone_ruleset"`

- `access_application_id: optional string`

  Access application ID identifier. Required for access_application_policy resources.

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

- `result: optional object { id, etag, name, 2 more }  or object { id, access_application_id, etag, 4 more }  or object { id, etag, name, 2 more }  or 34 more`

  Response for access_application resources

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

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/tags \
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
  "result": {
    "id": "023e105f4ecef8ad9ca31a8372d0c353",
    "etag": "v1:RBNvo1WzZ4oRRq0W9-hkng",
    "name": "my-worker-script",
    "tags": {
      "environment": "production",
      "team": "engineering"
    },
    "type": "access_application"
  }
}
```

## Set tags for a zone-level resource

**put** `/zones/{zone_id}/tags`

Creates or updates tags for a specific zone-level resource. Replaces all existing tags for the resource.

### Path Parameters

- `zone_id: string`

  Zone ID is required only for zone-level resources

### Header Parameters

- `"If-Match": optional string`

### Body Parameters

- `body: object { resource_id, resource_type, tags }  or object { access_application_id, resource_id, resource_type, tags }`

  Request body schema for setting tags on zone-level resources.

  - `ResourceTaggingSetTagsRequestZoneLevelBase object { resource_id, resource_type, tags }`

    Request body schema for deleting tags from zone-level resources. Zone ID comes from URL path.

    - `resource_id: string`

      Identifies the unique resource.

    - `resource_type: "api_gateway_operation" or "custom_certificate" or "custom_hostname" or 7 more`

      Enum for base zone-level resource types (those with no extra required fields).

      - `"api_gateway_operation"`

      - `"custom_certificate"`

      - `"custom_hostname"`

      - `"dns_record"`

      - `"healthcheck"`

      - `"load_balancer"`

      - `"managed_client_certificate"`

      - `"worker_route"`

      - `"zone"`

      - `"zone_ruleset"`

    - `tags: optional map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

  - `ResourceTaggingSetTagsRequestZoneLevelAccessApplicationPolicy object { access_application_id, resource_id, resource_type, tags }`

    Request body schema for deleting tags from zone-level resources. Zone ID comes from URL path.

    - `access_application_id: string`

      Access application ID is required only for access_application_policy resources

    - `resource_id: string`

      Identifies the unique resource.

    - `resource_type: "api_gateway_operation" or "custom_certificate" or "custom_hostname" or 8 more`

      Enum for base zone-level resource types (those with no extra required fields).

      - `"api_gateway_operation"`

      - `"custom_certificate"`

      - `"custom_hostname"`

      - `"dns_record"`

      - `"healthcheck"`

      - `"load_balancer"`

      - `"managed_client_certificate"`

      - `"worker_route"`

      - `"zone"`

      - `"zone_ruleset"`

      - `"access_application_policy"`

    - `tags: optional map[string]`

      Contains key-value pairs of tags. Keys may contain at most 256 characters. Values may contain at most 1024 characters and may be empty for key-only tags.

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

- `result: optional object { id, etag, name, 2 more }  or object { id, access_application_id, etag, 4 more }  or object { id, etag, name, 2 more }  or 34 more`

  Response for access_application resources

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

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/tags \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "resource_id": "023e105f4ecef8ad9ca31a8372d0c353",
          "resource_type": "zone",
          "tags": {
            "environment": "production",
            "team": "engineering"
          }
        }'
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
    "id": "023e105f4ecef8ad9ca31a8372d0c353",
    "etag": "v1:RBNvo1WzZ4oRRq0W9-hkng",
    "name": "my-worker-script",
    "tags": {
      "environment": "production",
      "team": "engineering"
    },
    "type": "access_application"
  }
}
```

## Delete tags from a zone-level resource

**delete** `/zones/{zone_id}/tags`

Removes all tags from a specific zone-level resource.

### Path Parameters

- `zone_id: string`

  Zone ID is required only for zone-level resources

### Header Parameters

- `"If-Match": optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/tags \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

## Domain Types

### Zone Tag Get Response

- `ZoneTagGetResponse = object { id, etag, name, 2 more }  or object { id, access_application_id, etag, 4 more }  or object { id, etag, name, 2 more }  or 34 more`

  Response for access_application resources

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

### Zone Tag Update Response

- `ZoneTagUpdateResponse = object { id, etag, name, 2 more }  or object { id, access_application_id, etag, 4 more }  or object { id, etag, name, 2 more }  or 34 more`

  Response for access_application resources

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
