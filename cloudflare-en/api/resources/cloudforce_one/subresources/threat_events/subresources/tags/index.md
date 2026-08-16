# Tags

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

## Creates a new tag

**post** `/accounts/{account_id}/cloudforce-one/events/tags/create`

Creates a new tag to be used accross threat events.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `value: string`

- `activeDuration: optional string`

- `actorCategory: optional string`

  Actor variety. Allowed values: Activist, Competitor, Customer, Crime Syndicate, Former Employee, Nation State, Organized Crime, Nation State Affiliated, Terrorist, Unaffiliated.

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

- `categoryUuid: optional string`

- `dateOfDiscovery: optional string`

  Date the actor was discovered (ISO YYYY-MM-DD).

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

  Actor motive. Allowed values: Convenience, Fear, Fun, Financial, Grudge, Ideology, Espionage.

- `motiveConfidence: optional number`

  Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

- `opsecLevel: optional string`

- `originCountryConfidence: optional number`

  Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

- `originCountryISO: optional string`

- `originCountryTlp: optional "red" or "amber" or "green" or "white"`

  TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `"red"`

  - `"amber"`

  - `"green"`

  - `"white"`

- `priority: optional number`

- `sophisticationLevel: optional string`

### Returns

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "value": "APT28",
          "actorCategory": "Nation State",
          "actorCategoryConfidence": 7,
          "attributionConfidenceScore": 7,
          "categoryUuid": "12345678-1234-1234-1234-1234567890ab",
          "dateOfDiscovery": "2024-01-15",
          "motive": "Espionage",
          "motiveConfidence": 7,
          "originCountryConfidence": 7,
          "originCountryTlp": "amber"
        }'
```

#### Response

```json
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
```

## Updates a tag (SoT)

**patch** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}`

Updates a Source-of-Truth tag by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Body Parameters

- `activeDuration: optional string`

- `actorCategory: optional string`

  Actor variety. Allowed values: Activist, Competitor, Customer, Crime Syndicate, Former Employee, Nation State, Organized Crime, Nation State Affiliated, Terrorist, Unaffiliated.

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

- `categoryUuid: optional string`

- `dateOfDiscovery: optional string`

  Date the actor was discovered (ISO YYYY-MM-DD).

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

  Actor motive. Allowed values: Convenience, Fear, Fun, Financial, Grudge, Ideology, Espionage.

- `motiveConfidence: optional number`

  Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

- `opsecLevel: optional string`

- `originCountryConfidence: optional number`

  Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

- `originCountryISO: optional string`

- `originCountryTlp: optional "red" or "amber" or "green" or "white"`

  TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `"red"`

  - `"amber"`

  - `"green"`

  - `"white"`

- `priority: optional number`

- `sophisticationLevel: optional string`

- `value: optional string`

### Returns

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
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
```

## Deletes a tag (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}`

Deletes a Source-of-Truth tag by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Domain Types

### Tag List Response

- `TagListResponse object { pagination, tags }`

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

### Tag Create Response

- `TagCreateResponse object { uuid, value, activeDuration, 25 more }`

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

### Tag Edit Response

- `TagEditResponse object { uuid, value, activeDuration, 25 more }`

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

### Tag Delete Response

- `TagDeleteResponse object { uuid }`

  - `uuid: string`

# Categories

## Lists all tag categories (SoT)

**get** `/accounts/{account_id}/cloudforce-one/events/tags/categories`

Returns all Source-of-Truth tag categories for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `search: optional string`

### Returns

- `categories: array of object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "categories": [
    {
      "name": "Actor",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "createdAt": "createdAt",
      "description": "description",
      "updatedAt": "updatedAt"
    }
  ]
}
```

## Creates a new tag category (SoT)

**post** `/accounts/{account_id}/cloudforce-one/events/tags/categories/create`

Creates a new Source-of-Truth tag category for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `name: string`

- `description: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "name": "Actor"
        }'
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```

## Updates a tag category (SoT)

**patch** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Updates a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Body Parameters

- `description: optional string`

- `name: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```

## Deletes a tag category (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Deletes a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Domain Types

### Category List Response

- `CategoryListResponse object { categories }`

  - `categories: array of object { name, uuid, createdAt, 2 more }`

    - `name: string`

    - `uuid: string`

    - `createdAt: optional string`

    - `description: optional string`

    - `updatedAt: optional string`

### Category Create Response

- `CategoryCreateResponse object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Category Edit Response

- `CategoryEditResponse object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Category Delete Response

- `CategoryDeleteResponse object { uuid }`

  - `uuid: string`

# Indicators

## List indicators related to a tag

**get** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}/indicators`

Returns indicators associated with the provided tag UUID, with pagination. By default fans out across every indicator dataset the account can read; pass datasetIds to scope to specific datasets.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Query Parameters

- `datasetIds: optional array of string`

  Dataset UUIDs to scope to (repeat the param for multiple), or 'all' / '*' for every readable indicator dataset. Omit to search all readable datasets.

- `indicatorType: optional string`

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType. Multiple conditions are AND'd together. Max 10 conditions per request.

  - `field: "value" or "indicatorType" or "uuid"`

    The indicator field to search on. Allowed: value, indicatorType, uuid.

    - `"value"`

    - `"indicatorType"`

    - `"uuid"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'value', op:'in', value:['evil.com','bad.org']}.

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

  - `value: string or array of string`

    Search value. String for most operators. Array of strings for 'in' operator (max 100 items).

    - `string`

    - `array of string`

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

    - `datasetId: string`

    - `eventId: string`

    - `eventDate: optional string`

      ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "indicators": [
    {
      "createdAt": "2022-04-01T00:00:00Z",
      "indicatorType": "domain",
      "updatedAt": "2022-04-01T00:00:00Z",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "malicious-domain.com",
      "datasetId": "dataset-uuid-123",
      "relatedEvents": [
        {
          "datasetId": "dataset-uuid-123",
          "eventId": "event-uuid-456",
          "eventDate": "2024-06-15T00:00:00Z"
        }
      ],
      "tags": [
        {
          "categoryName": "categoryName",
          "uuid": "uuid",
          "value": "value"
        }
      ]
    }
  ],
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  }
}
```

## Domain Types

### Indicator List Response

- `IndicatorListResponse object { indicators, pagination }`

  - `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

    - `createdAt: string`

    - `indicatorType: string`

    - `updatedAt: string`

    - `uuid: string`

    - `value: string`

    - `datasetId: optional string`

      The dataset ID this indicator belongs to. Included in list responses.

    - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

      - `datasetId: string`

      - `eventId: string`

      - `eventDate: optional string`

        ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`

# By Dataset

## List indicators related to a tag within a dataset (deprecated)

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/tags/{tag_uuid}/indicators`

This endpoint is deprecated. Use GET /:account_id/events/tags/:tag_uuid/indicators with the optional datasetIds query parameter instead. Returns indicators associated with the provided tag UUID within a single dataset's indicator shards, with pagination.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

- `tag_uuid: string`

  Tag UUID.

### Query Parameters

- `indicatorType: optional string`

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType. Multiple conditions are AND'd together. Max 10 conditions per request.

  - `field: "value" or "indicatorType" or "uuid"`

    The indicator field to search on. Allowed: value, indicatorType, uuid.

    - `"value"`

    - `"indicatorType"`

    - `"uuid"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'value', op:'in', value:['evil.com','bad.org']}.

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

  - `value: string or array of string`

    Search value. String for most operators. Array of strings for 'in' operator (max 100 items).

    - `string`

    - `array of string`

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

    - `datasetId: string`

    - `eventId: string`

    - `eventDate: optional string`

      ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/tags/$TAG_UUID/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "indicators": [
    {
      "createdAt": "2022-04-01T00:00:00Z",
      "indicatorType": "domain",
      "updatedAt": "2022-04-01T00:00:00Z",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "malicious-domain.com",
      "datasetId": "dataset-uuid-123",
      "relatedEvents": [
        {
          "datasetId": "dataset-uuid-123",
          "eventId": "event-uuid-456",
          "eventDate": "2024-06-15T00:00:00Z"
        }
      ],
      "tags": [
        {
          "categoryName": "categoryName",
          "uuid": "uuid",
          "value": "value"
        }
      ]
    }
  ],
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  }
}
```

## Domain Types

### By Dataset List Response

- `ByDatasetListResponse object { indicators, pagination }`

  - `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

    - `createdAt: string`

    - `indicatorType: string`

    - `updatedAt: string`

    - `uuid: string`

    - `value: string`

    - `datasetId: optional string`

      The dataset ID this indicator belongs to. Included in list responses.

    - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

      - `datasetId: string`

      - `eventId: string`

      - `eventDate: optional string`

        ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`
