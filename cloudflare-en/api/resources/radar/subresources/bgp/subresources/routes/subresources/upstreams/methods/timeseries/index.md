## Get upstream composition time series for an AS

**get** `/radar/bgp/routes/upstreams/{asn}/timeseries`

Retrieves the share of an AS’s observed paths carried by each direct upstream over time, derived from RouteViews RIB snapshots across all collectors (the combined product). Each upstream ASN is returned as its own series of shares (0–1); the least-significant upstreams beyond the requested limit are grouped into an "OTHER" series. Series share a common set of timestamps.

### Path Parameters

- `asn: number`

  Single Autonomous System Number (ASN) as integer.

### Query Parameters

- `dateEnd: optional string`

  End of the date range (inclusive). Alternative to `dateRange`; provide together with `dateStart`.

- `dateStart: optional string`

  Start of the date range (inclusive). Alternative to `dateRange`; provide together with `dateEnd`.

- `format: optional "JSON" or "CSV"`

  Format in which results will be returned.

  - `"JSON"`

  - `"CSV"`

- `ipVersion: optional "IPv4" or "IPv6"`

  Address family of the observed paths. Defaults to IPv4.

  - `"IPv4"`

  - `"IPv6"`

- `limit: optional number`

  Number of upstream ASNs to return as separate series, ranked by the first bucket. Remaining upstreams are grouped into an "OTHER" series. Defaults to 5.

### Returns

- `result: object { meta, serie_0 }`

  - `meta: object { dataTime, effectiveCollector, queryTime, stale }`

    - `dataTime: string`

      Timestamp of the underlying RIB data.

    - `effectiveCollector: string`

    - `queryTime: string`

      Timestamp when the query was executed.

    - `stale: boolean`

  - `serie_0: object { timestamps }`

    - `timestamps: array of string`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/radar/bgp/routes/upstreams/$ASN/timeseries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "meta": {
      "dataTime": "2019-12-27T18:11:19.117Z",
      "effectiveCollector": "effectiveCollector",
      "queryTime": "2019-12-27T18:11:19.117Z",
      "stale": true
    },
    "serie_0": {
      "timestamps": [
        "2023-08-08T10:15:00Z"
      ]
    }
  },
  "success": true
}
```
