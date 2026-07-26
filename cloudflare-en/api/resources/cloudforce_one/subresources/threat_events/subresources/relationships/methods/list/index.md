## Filter and list events related to specific event

**get** `/accounts/{account_id}/cloudforce-one/events/{event_id}/relationships`

The `event_id` must be defined (to list existing events (and their IDs), use the [`Filter and List Events`](https://developers.cloudflare.com/api/resources/cloudforce_one/subresources/threat_events/methods/list/) endpoint). Also, must provide query parameters.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Query Parameters

- `datasetId: string`

  The dataset ID to search within.

- `direction: optional "ancestors" or "descendants" or "both"`

  The direction to traverse the graph. Defaults to 'both' to search all.

  - `"ancestors"`

  - `"descendants"`

  - `"both"`

- `includeParent: optional boolean`

  Whether to include the starting event in the results. Defaults to true.

- `indicatorTypeIds: optional array of string`

  An optional array of indicator type IDs to filter the results by.

- `maxDepth: optional number`

  The maximum depth to traverse. Defaults to 5.

- `page: optional number`

- `pageSize: optional number`

- `relationshipTypes: optional string or array of string`

  An optional array of relationship types to filter by.

  - `string`

  - `array of string`

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/$EVENT_ID/relationships \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "attacker": "Flying Yeti",
    "attackerCountry": "CN",
    "attackerCountryAlpha3": "CHN",
    "category": "Domain Resolution",
    "datasetId": "dataset-example-id",
    "date": "2022-04-01T00:00:00Z",
    "event": "An attacker registered the domain domain.com",
    "hasChildren": true,
    "indicator": "domain.com",
    "indicatorType": "domain",
    "indicatorTypeId": 5,
    "killChain": 0,
    "mitreAttack": [
      " "
    ],
    "mitreCapec": [
      " "
    ],
    "numReferenced": 0,
    "numReferences": 0,
    "rawId": "453gw34w3",
    "referenced": [
      " "
    ],
    "referencedIds": [
      0
    ],
    "references": [
      " "
    ],
    "referencesIds": [
      0
    ],
    "tags": [
      "malware"
    ],
    "targetCountry": "US",
    "targetCountryAlpha3": "USA",
    "targetIndustry": "Agriculture",
    "tlp": "amber",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "insight": "insight",
    "releasabilityId": "releasabilityId"
  }
]
```
