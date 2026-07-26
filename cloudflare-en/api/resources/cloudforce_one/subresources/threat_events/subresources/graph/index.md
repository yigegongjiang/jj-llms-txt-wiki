# Graph

## Query graph neighborhood from R2 Data Catalog

**get** `/accounts/{account_id}/cloudforce-one/events/graph`

Expands the single-level relationship neighborhood of one or more seed nodes (event, indicator, or tag) from R2 Data Catalog. Seeds use compact id format (type:uuid), e.g. "event:550e8400-...". Multi-seed requests merge and deduplicate results server-side. Hydrates neighbor entities with summary data from Durable Objects. Supports filtering by relationship type and dataset scope.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cursor: optional string`

  Opaque pagination token. Only valid when seeds has exactly 1 entry; 400 otherwise.

- `datasetIds: optional array of string`

  Comma-separated dataset UUIDs to restrict neighbor scope. Intersected with ACL grants.

- `direction: optional string`

  Edge direction relative to each seed: out (seed→neighbors), in (neighbors→seed), both (default).

- `expand: optional array of string`

  Comma-separated list of response sections to expand (hydrate). Allowed: `nodes`. Omitting `expand` returns identifier-only nodes.

- `hydration: optional string`

  Hydration strategy for neighbor nodes when expand=nodes is set. r2_join (default): use R2 JOIN query + DO fallback. do_only: use plain R2 query + hydrate all neighbors via Durable Objects.

- `limit: optional number`

  Max neighbors per seed (default: 100, max: 1000). Values above 1000 return 400.

- `max_nodes: optional number`

  Total accumulated node cap across all seeds (default: 500, max: 1000). Values above 1000 return 400.

- `relationshipTypes: optional array of string`

  Comma-separated relationship types to filter by. Allowed: tagged_with, appears_in, related_to, caused_by, attributed_to.

- `seeds: optional array of string`

  Comma-separated compact seed ids (type:uuid). Example: seeds=event:550e8400-...,indicator:661fa920-... Provide 1–50 entries; omitting seeds returns 400.

### Returns

- `errors: array of map[unknown]`

- `messages: array of map[unknown]`

- `result: object { edges, node, nodes }`

  - `edges: array of object { id, relationshipType, source, 5 more }`

    - `id: string`

      Deterministic composite edge id (source→target:relationshipType)

    - `relationshipType: string`

    - `source: string`

      Compact id of the source node (type:uuid)

    - `sourceId: string`

    - `sourceType: string`

    - `target: string`

      Compact id of the target node (type:uuid)

    - `targetId: string`

    - `targetType: string`

  - `node: map[unknown]`

    Focal node object (legacy single-seed). Null when unavailable.

  - `nodes: array of map[unknown]`

- `success: boolean`

- `result_info: optional object { count, edge_count, query_time_ms, 6 more }`

  - `count: number`

    Number of nodes in result.nodes (seeds + neighbors)

  - `edge_count: number`

    Number of edges in result.edges

  - `query_time_ms: number`

    Total query time in milliseconds

  - `total_count: number`

    Total count of nodes (same as count for this endpoint)

  - `cursor: optional string`

    Opaque pagination cursor for the next page; null when exhausted or for multi-seed requests (single-seed only)

  - `depth_reached: optional number`

    Traversal depth reached (always 1 for single-level)

  - `has_more: optional boolean`

    True when a cursor is available for the next page (single-seed only)

  - `seeds: optional array of string`

    Composite ids of the seed node(s) (type:uuid). Always an array, even for one seed.

  - `truncated: optional boolean`

    True when results were capped (per-seed limit or max_nodes)

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/graph \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "edges": [
      {
        "id": "event:550e8400-e29b-41d4-a716-446655440000→indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04:appears_in",
        "relationshipType": "appears_in",
        "source": "event:550e8400-e29b-41d4-a716-446655440000",
        "sourceId": "550e8400-e29b-41d4-a716-446655440000",
        "sourceType": "event",
        "target": "indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "targetId": "661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "targetType": "indicator"
      }
    ],
    "node": null,
    "nodes": [
      {
        "attacker": "APT28",
        "category": "intrusion",
        "datasetId": "a1b2c3d4-0001-4000-8000-000000000001",
        "date": "2026-06-01",
        "event": "Attacker registered domain evil.example.com",
        "id": "event:550e8400-e29b-41d4-a716-446655440000",
        "role": "focal",
        "type": "event",
        "uuid": "550e8400-e29b-41d4-a716-446655440000"
      },
      {
        "datasetId": "a1b2c3d4-0001-4000-8000-000000000001",
        "id": "indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "indicatorType": "domain",
        "role": "focal",
        "type": "indicator",
        "uuid": "661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "value": "evil.example.com"
      },
      {
        "categoryId": "threat-actor",
        "id": "tag:772af1c8-dc4a-4a29-b3e6-4f8c9d2a6e71",
        "type": "tag",
        "uuid": "772af1c8-dc4a-4a29-b3e6-4f8c9d2a6e71",
        "value": "APT28"
      }
    ]
  },
  "result_info": {
    "count": 3,
    "cursor": null,
    "depth_reached": 1,
    "edge_count": 1,
    "has_more": false,
    "query_time_ms": 890,
    "seeds": [
      "event:550e8400-e29b-41d4-a716-446655440000",
      "indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04"
    ],
    "total_count": 3,
    "truncated": false
  },
  "success": true
}
```

## Domain Types

### Graph List Response

- `GraphListResponse object { edges, node, nodes }`

  - `edges: array of object { id, relationshipType, source, 5 more }`

    - `id: string`

      Deterministic composite edge id (source→target:relationshipType)

    - `relationshipType: string`

    - `source: string`

      Compact id of the source node (type:uuid)

    - `sourceId: string`

    - `sourceType: string`

    - `target: string`

      Compact id of the target node (type:uuid)

    - `targetId: string`

    - `targetType: string`

  - `node: map[unknown]`

    Focal node object (legacy single-seed). Null when unavailable.

  - `nodes: array of map[unknown]`
