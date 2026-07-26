## Get the number of outages by location

**get** `/radar/annotations/outages/locations`

Retrieves the number of outages by location.

### Query Parameters

- `dateEnd: optional string`

  End of the date range (inclusive). Alternative to `dateRange`; provide together with `dateStart`.

- `dateRange: optional string`

  Filters results by a relative date range ending at the current time. Use `<n>d` for days (up to `364d`) or `<n>w` for weeks (up to `52w`), e.g. `7d`. Append `control` to request the equivalent previous period for comparison: the comparison window is shifted back by the current window's length rounded up to a whole number of weeks, so it keeps the same weekday alignment and does not overlap the current window (e.g. `3dcontrol` covers days -10 to -7, `7dcontrol` covers days -14 to -7, `28dcontrol` covers days -56 to -28, and `10dcontrol` covers days -24 to -14). Mutually exclusive with `dateStart`/`dateEnd`.

- `dateStart: optional string`

  Start of the date range (inclusive). Alternative to `dateRange`; provide together with `dateEnd`.

- `format: optional "JSON" or "CSV"`

  Format in which results will be returned.

  - `"JSON"`

  - `"CSV"`

- `limit: optional number`

  Limits the number of objects returned in the response.

### Returns

- `result: object { annotations }`

  - `annotations: array of object { clientCountryAlpha2, clientCountryName, value }`

    - `clientCountryAlpha2: string`

    - `clientCountryName: string`

    - `value: string`

      A numeric string.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/radar/annotations/outages/locations \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "annotations": [
      {
        "clientCountryAlpha2": "PT",
        "clientCountryName": "Portugal",
        "value": "10"
      }
    ]
  },
  "success": true
}
```
