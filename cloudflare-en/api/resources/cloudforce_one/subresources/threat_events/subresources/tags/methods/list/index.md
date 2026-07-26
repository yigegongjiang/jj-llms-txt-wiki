## Lists all tags (SoT)

**get** `/accounts/{account_id}/cloudforce-one/events/tags`

Returns all Source-of-Truth tags for an account. Supports legacy free-text `search` on tag value and `categoryUuid` exact match, plus a structured `filters` JSON array for filtering by metadata fields (originCountryISO, actorCategory, motive, priority, etc.). Country values may be passed as alpha-2, alpha-3, name, or common alias.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cache: optional "from-graph"`

  Cache strategy. 'from-graph' serves results from the graph-node KV cache when all requested UUIDs are cached; falls back to normal path on partial/zero hit.

  - `"from-graph"`

- `categoryUuid: optional string`

- `filters: optional array of object { field, op, value }`

  Structured filters as a JSON array of {field, op, value} objects. Searchable fields: uuid, value, actorCategory, actorCategoryConfidence, aliasGroupNames, attributionConfidence, attributionConfidenceScore, attributionOrganization, categoryName, motive, motiveConfidence, opsecLevel, originCountryISO, originCountryConfidence, sophisticationLevel, priority, analyticPriority. Operators: equals, not, contains, startsWith, endsWith, gt, lt, gte, lte, like, in, find. Use 'in' for bulk OR within a single field, e.g. filters=[{"field":"originCountryISO","op":"in","value":["IR","CN"]}]. Multiple entries are AND-joined. Max 10 entries per request, max 100 values per 'in'. Per-field notes: `uuid` accepts only 'equals' and 'in' (other operators throw ValidationError) — matched against the canonical lowercase storage but callers may pass either case (the server lowercases before comparison); index-backed by the column's UNIQUE constraint and intended for batched UUID → tag resolution. `originCountryISO` uses its B-tree index for equals/not/in. `priority` uses its B-tree index for numeric comparisons. Other string columns (`actorCategory`, `motive`, etc.) are case-insensitive and unindexed; current catalog size makes this a non-issue. `endsWith` and `aliasGroupNames` contains/like are leading-wildcard scans and slow on large result sets. `aliasGroupNames` matches on the JSON-encoded text, so substrings can cross alias boundaries (a search for "apt28" will also match "apt280" if both appear in the same tag's alias list).

  - `field: "uuid" or "value" or "actorCategory" or 14 more`

    Tag field to search on. Allowed: uuid, value, actorCategory, actorCategoryConfidence, aliasGroupNames, attributionConfidence, attributionConfidenceScore, attributionOrganization, categoryName, motive, motiveConfidence, opsecLevel, originCountryISO, originCountryConfidence, sophisticationLevel, priority, analyticPriority.

    - `"uuid"`

    - `"value"`

    - `"actorCategory"`

    - `"actorCategoryConfidence"`

    - `"aliasGroupNames"`

    - `"attributionConfidence"`

    - `"attributionConfidenceScore"`

    - `"attributionOrganization"`

    - `"categoryName"`

    - `"motive"`

    - `"motiveConfidence"`

    - `"opsecLevel"`

    - `"originCountryISO"`

    - `"originCountryConfidence"`

    - `"sophisticationLevel"`

    - `"priority"`

    - `"analyticPriority"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk OR within a single field, e.g. {field:"originCountryISO", op:"in", value:["IR","CN"]}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: optional string or number or array of string or number`

    Search value. String or number for most operators. Array for 'in' (max 100 items). Country values may be passed as alpha-2, alpha-3, name, or common alias (e.g. 'iran', 'IR', 'IRN') and are normalized to alpha-2 server-side.

    - `string`

    - `number`

    - `array of string or number`

      - `string`

      - `number`

- `page: optional number`

- `pageSize: optional number`

- `search: optional string`

  Legacy free-text substring match on tag value.

### Returns

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

- `tags: array of object { uuid, value, activeDuration, 25 more }`

  - `uuid: string`

  - `value: string`

  - `activeDuration: optional string`

  - `actorCategory: optional string`

  - `actorCategoryConfidence: optional number`

    Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

  - `aliases: optional array of object { value, confidence, tlp }`

    Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `aliasGroupNames: optional array of string`

  - `aliasGroupNamesInternal: optional array of string`

  - `analyticPriority: optional number`

  - `attributionConfidence: optional string`

  - `attributionConfidenceScore: optional number`

  - `attributionOrganization: optional string`

  - `categoryName: optional string`

  - `categoryUuid: optional string`

  - `dateOfDiscovery: optional string`

  - `externalReferenceLinks: optional array of string`

  - `externalReferences: optional array of object { url, description }`

    Structured external references ({ url, description }). Public: returned to all accounts.

    - `url: string`

    - `description: optional string`

  - `internalAliases: optional array of object { value, confidence, tlp }`

    Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `internalDescription: optional string`

  - `motive: optional string`

  - `motiveConfidence: optional number`

    Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

  - `opsecLevel: optional string`

  - `originCountryConfidence: optional number`

    Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `originCountryISO: optional string`

  - `originCountryISOAlpha3: optional string`

  - `originCountryTlp: optional "red" or "amber" or "green" or "white"`

    TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

  - `priority: optional number`

  - `sophisticationLevel: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  },
  "tags": [
    {
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "APT28",
      "activeDuration": "activeDuration",
      "actorCategory": "actorCategory",
      "actorCategoryConfidence": 7,
      "aliases": [
        {
          "value": "Fancy Bear",
          "confidence": 8,
          "tlp": "amber"
        }
      ],
      "aliasGroupNames": [
        "string"
      ],
      "aliasGroupNamesInternal": [
        "string"
      ],
      "analyticPriority": 0,
      "attributionConfidence": "attributionConfidence",
      "attributionConfidenceScore": 7,
      "attributionOrganization": "attributionOrganization",
      "categoryName": "Nation State",
      "categoryUuid": "12345678-1234-1234-1234-1234567890ab",
      "dateOfDiscovery": "2024-01-15",
      "externalReferenceLinks": [
        "string"
      ],
      "externalReferences": [
        {
          "url": "https://example.com/report",
          "description": "Vendor threat report"
        }
      ],
      "internalAliases": [
        {
          "value": "Fancy Bear",
          "confidence": 8,
          "tlp": "amber"
        }
      ],
      "internalDescription": "internalDescription",
      "motive": "motive",
      "motiveConfidence": 7,
      "opsecLevel": "opsecLevel",
      "originCountryConfidence": 7,
      "originCountryISO": "originCountryISO",
      "originCountryISOAlpha3": "IRN",
      "originCountryTlp": "amber",
      "priority": 0,
      "sophisticationLevel": "sophisticationLevel"
    }
  ]
}
```
