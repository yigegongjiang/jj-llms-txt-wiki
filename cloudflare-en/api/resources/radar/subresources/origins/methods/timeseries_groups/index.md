## Get origin metrics time series grouped by dimension

**get** `/radar/origins/timeseries_groups/{dimension}`

Retrieves the distribution of origin metrics grouped by the specified dimension over time.

### Path Parameters

- `dimension: "ORIGIN" or "REGION" or "SUCCESS_RATE" or "PERCENTILE"`

  Specifies the origin attribute by which to group the results. `ORIGIN` groups across all providers and does not accept an `origin` or `region`. `REGION` requires an `origin`. `SUCCESS_RATE` and `PERCENTILE` require both an `origin` and a `region` and constrain the `metric` (`SUCCESS_RATE` supports only `REQUESTS`; `PERCENTILE` supports any metric except `REQUESTS`). `limitPerGroup` is only supported on the `REGION` dimension.

  - `"ORIGIN"`

  - `"REGION"`

  - `"SUCCESS_RATE"`

  - `"PERCENTILE"`

### Query Parameters

- `metric: "CONNECTION_FAILURES" or "REQUESTS" or "RESPONSE_HEADER_RECEIVE_DURATION" or 3 more`

  Specifies the metric to retrieve. Allowed metrics depend on the selected dimension (see the `dimension` path parameter).

  - `"CONNECTION_FAILURES"`

  - `"REQUESTS"`

  - `"RESPONSE_HEADER_RECEIVE_DURATION"`

  - `"TCP_HANDSHAKE_DURATION"`

  - `"TCP_RTT"`

  - `"TLS_HANDSHAKE_DURATION"`

- `aggInterval: optional "15m" or "1h" or "1d" or "1w"`

  Aggregation interval of the results (e.g., in 15 minutes or 1 hour intervals). Refer to [Aggregation intervals](https://developers.cloudflare.com/radar/concepts/aggregation-intervals/). When omitted, the interval is auto-selected from the requested date range; finer intervals are only available for shorter ranges. If the requested interval is too granular for the date range, the request is rejected.

  - `"15m"`

  - `"1h"`

  - `"1d"`

  - `"1w"`

- `dateEnd: optional array of string`

  End of the date range (inclusive). Alternative to `dateRange`; provide together with `dateStart`. When requesting comparison series, every series must resolve to the same duration as the main series. Each `dateStart`/`dateEnd` is floored to the nearest 15 minutes before evaluation, so windows whose durations match only before alignment may be rejected.

- `dateRange: optional array of string`

  Filters results by relative date range ending at the current time, with each value producing a separate series. Use `<n>d` for days (up to `364d`) or `<n>w` for weeks (up to `52w`). Append `control` to request the equivalent previous period for comparison: the comparison window is shifted back by the current window's length rounded up to a whole number of weeks, so it keeps the same weekday alignment and does not overlap the current window (e.g. `7dcontrol` covers days -14 to -7, `10dcontrol` covers days -24 to -14). For example, pass `7d` and `7dcontrol` to compare this week with the previous week. All series must resolve to the same duration as the main series; relative ranges (including `control`) satisfy this automatically. Use this parameter or set specific start and end dates (`dateStart` and `dateEnd` parameters).

- `dateStart: optional array of string`

  Start of the date range. Alternative to `dateRange`; provide together with `dateEnd`. When requesting comparison series, every series must resolve to the same duration as the main series. Each `dateStart`/`dateEnd` is floored to the nearest 15 minutes before evaluation, so windows whose durations match only before alignment may be rejected.

- `format: optional "JSON" or "CSV"`

  Format in which results will be returned.

  - `"JSON"`

  - `"CSV"`

- `limitPerGroup: optional number`

  Limits the number of objects per group to the top items within the specified time range. When item count exceeds the limit, extra items appear grouped under an "other" category. Only supported on high-cardinality dimensions; otherwise the request is rejected. Minimum value is 2.

- `name: optional array of string`

  Array of names used to label the series in the response.

- `normalization: optional "PERCENTAGE" or "MIN0_MAX"`

  Normalization method applied to the results. Refer to [Normalization methods](https://developers.cloudflare.com/radar/concepts/normalization/).

  - `"PERCENTAGE"`

  - `"MIN0_MAX"`

- `origin: optional array of "AMAZON" or "GOOGLE" or "MICROSOFT" or "ORACLE"`

  Filters results by origin. Required for every dimension except `ORIGIN`; must not be set on the `ORIGIN` dimension, which groups across all providers.

  - `"AMAZON"`

  - `"GOOGLE"`

  - `"MICROSOFT"`

  - `"ORACLE"`

- `region: optional array of string`

  Filters results by origin region. Requires `origin` to be set and is validated against it.

### Returns

- `result: object { meta, serie_0 }`

  - `meta: object { aggInterval, confidenceInfo, dateRange, 3 more }`

    Metadata for the results.

    - `aggInterval: "FIFTEEN_MINUTES" or "ONE_HOUR" or "ONE_DAY" or 2 more`

      Aggregation interval of the results (e.g., in 15 minutes or 1 hour intervals). Refer to [Aggregation intervals](https://developers.cloudflare.com/radar/concepts/aggregation-intervals/).

      - `"FIFTEEN_MINUTES"`

      - `"ONE_HOUR"`

      - `"ONE_DAY"`

      - `"ONE_WEEK"`

      - `"ONE_MONTH"`

    - `confidenceInfo: object { annotations, level }`

      - `annotations: array of object { dataSource, description, endDate, 5 more }`

        - `dataSource: "ALL" or "AI_BOTS" or "AI_GATEWAY" or 22 more`

          Data source for annotations.

          - `"ALL"`

          - `"AI_BOTS"`

          - `"AI_GATEWAY"`

          - `"BGP"`

          - `"BOTS"`

          - `"CONNECTION_ANOMALY"`

          - `"CT"`

          - `"DNS"`

          - `"DNS_MAGNITUDE"`

          - `"DNS_AS112"`

          - `"DOS"`

          - `"EMAIL_ROUTING"`

          - `"EMAIL_SECURITY"`

          - `"FW"`

          - `"FW_PG"`

          - `"HTTP"`

          - `"HTTP_CONTROL"`

          - `"HTTP_CRAWLER_REFERER"`

          - `"HTTP_ORIGINS"`

          - `"IQI"`

          - `"LEAKED_CREDENTIALS"`

          - `"NET"`

          - `"ROBOTS_TXT"`

          - `"SPEED"`

          - `"WORKERS_AI"`

        - `description: string`

        - `endDate: string`

        - `eventType: "EVENT" or "GENERAL" or "OUTAGE" or 3 more`

          Event type for annotations.

          - `"EVENT"`

          - `"GENERAL"`

          - `"OUTAGE"`

          - `"PARTIAL_PROJECTION"`

          - `"PIPELINE"`

          - `"TRAFFIC_ANOMALY"`

        - `isInstantaneous: boolean`

          Whether event is a single point in time or a time range.

        - `linkedUrl: string`

        - `startDate: string`

        - `tags: optional array of string`

      - `level: number`

        Provides an indication of how much confidence Cloudflare has in the data.

    - `dateRange: array of object { endTime, startTime }`

      - `endTime: string`

        Adjusted end of date range.

      - `startTime: string`

        Adjusted start of date range.

    - `lastUpdated: string`

      Timestamp of the last dataset update.

    - `normalization: "PERCENTAGE" or "MIN0_MAX" or "MIN_MAX" or 5 more`

      Normalization method applied to the results. Refer to [Normalization methods](https://developers.cloudflare.com/radar/concepts/normalization/).

      - `"PERCENTAGE"`

      - `"MIN0_MAX"`

      - `"MIN_MAX"`

      - `"RAW_VALUES"`

      - `"PERCENTAGE_CHANGE"`

      - `"ROLLING_AVERAGE"`

      - `"OVERLAPPED_PERCENTAGE"`

      - `"RATIO"`

    - `units: array of object { name, value }`

      Measurement units for the results.

      - `name: string`

      - `value: string`

  - `serie_0: object { timestamps }`

    - `timestamps: array of string`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/radar/origins/timeseries_groups/$DIMENSION \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "meta": {
      "aggInterval": "FIFTEEN_MINUTES",
      "confidenceInfo": {
        "annotations": [
          {
            "dataSource": "ALL",
            "description": "Cable cut in Tonga",
            "endDate": "2019-12-27T18:11:19.117Z",
            "eventType": "EVENT",
            "isInstantaneous": true,
            "linkedUrl": "https://example.com",
            "startDate": "2019-12-27T18:11:19.117Z",
            "tags": [
              "BOT_CLASS"
            ]
          }
        ],
        "level": 0
      },
      "dateRange": [
        {
          "endTime": "2022-09-17T10:22:57.555Z",
          "startTime": "2022-09-16T10:22:57.555Z"
        }
      ],
      "lastUpdated": "2019-12-27T18:11:19.117Z",
      "normalization": "PERCENTAGE",
      "units": [
        {
          "name": "*",
          "value": "requests"
        }
      ]
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
