# Paths

## Get tier-1 path segments for an AS

**get** `/radar/bgp/routes/paths/{asn}`

Retrieves the paths an AS uses to reach the tier-1 clique, derived from RouteViews RIB snapshots. Each entry is an ordered AS-path segment (from the queried AS toward a tier-1) with the number of observed paths and peers, and the collectors that observed it. By default segments are merged across all active collectors; pass "collector" to scope to one. The response also includes an "asnInfo" map (keyed by ASN) with the name and country for every ASN in the returned segments plus the queried ASN (best-effort; null when unavailable).

### Path Parameters

- `asn: number`

  Single Autonomous System Number (ASN) as integer.

### Query Parameters

- `collector: optional string`

  Scope to a single RouteViews collector (e.g. "route-views3"). Omit to merge across all active collectors (identical path segments are deduplicated, observation counts summed, and every contributing collector listed).

- `format: optional "JSON" or "CSV"`

  Format in which results will be returned.

  - `"JSON"`

  - `"CSV"`

- `ipVersion: optional "IPv4" or "IPv6"`

  Address family of the observed paths. Defaults to IPv4.

  - `"IPv4"`

  - `"IPv6"`

### Returns

- `result: object { asnInfo, collectors, meta, paths }`

  - `asnInfo: map[object { asn, country, name } ]`

    - `asn: number`

      ASN number.

    - `country: string`

      Alpha-2 country code.

    - `name: string`

      AS name.

  - `collectors: array of string`

  - `meta: object { dataTime, effectiveCollector, queryTime, stale }`

    - `dataTime: string`

      Timestamp of the underlying RIB data.

    - `effectiveCollector: string`

    - `queryTime: string`

      Timestamp when the query was executed.

    - `stale: boolean`

  - `paths: array of object { collectors, pathsCount, peersCount, segment }`

    - `collectors: array of string`

    - `pathsCount: number`

    - `peersCount: number`

    - `segment: array of number`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/radar/bgp/routes/paths/$ASN \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "asnInfo": {
      "13335": {
        "asn": 13335,
        "country": "US",
        "name": "Cloudflare"
      }
    },
    "collectors": [
      "string"
    ],
    "meta": {
      "dataTime": "2019-12-27T18:11:19.117Z",
      "effectiveCollector": "effectiveCollector",
      "queryTime": "2019-12-27T18:11:19.117Z",
      "stale": true
    },
    "paths": [
      {
        "collectors": [
          "string"
        ],
        "pathsCount": 0,
        "peersCount": 0,
        "segment": [
          0
        ]
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Path List Response

- `PathListResponse object { asnInfo, collectors, meta, paths }`

  - `asnInfo: map[object { asn, country, name } ]`

    - `asn: number`

      ASN number.

    - `country: string`

      Alpha-2 country code.

    - `name: string`

      AS name.

  - `collectors: array of string`

  - `meta: object { dataTime, effectiveCollector, queryTime, stale }`

    - `dataTime: string`

      Timestamp of the underlying RIB data.

    - `effectiveCollector: string`

    - `queryTime: string`

      Timestamp when the query was executed.

    - `stale: boolean`

  - `paths: array of object { collectors, pathsCount, peersCount, segment }`

    - `collectors: array of string`

    - `pathsCount: number`

    - `peersCount: number`

    - `segment: array of number`
