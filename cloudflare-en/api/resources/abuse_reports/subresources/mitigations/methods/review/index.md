## Request review on mitigations

**post** `/accounts/{account_id}/abuse-reports/{report_id}/mitigations/appeal`

Request a review for mitigations on an account.

### Path Parameters

- `account_id: string`

- `report_id: string`

### Body Parameters

- `appeals: optional array of object { id, reason }`

  List of mitigations to appeal.

  - `id: string`

    ID of the mitigation to appeal.

  - `reason: "removed" or "misclassified"`

    Reason why the customer is appealing.

    - `"removed"`

    - `"misclassified"`

- `data: optional object { city, country, email, 11 more }`

  Counter-notice details supporting an appeal.

  - `city: string`

  - `country: string`

  - `email: string`

  - `full_name: string`

  - `jurisdiction_consent: boolean`

  - `perjury_attestation: boolean`

  - `phone_number: string`

  - `signature: string`

  - `state: string`

  - `street_address: string`

  - `urls: array of string`

  - `zip_code: string`

  - `company: optional string`

  - `counter_notice_response: optional string`

- `type: optional "counter_notice" or "content_removed"`

  The type of appeal being submitted.

  - `"counter_notice"`

  - `"content_removed"`

### Returns

- `result_info: object { count, page, per_page, 2 more }`

  - `count: number`

  - `page: number`

  - `per_page: number`

  - `total_count: number`

  - `total_pages: number`

- `success: boolean`

- `errors: optional array of object { message }`

  - `message: string`

- `messages: optional array of object { message }`

  - `message: string`

- `result: optional array of object { id, effective_date, entity_id, 3 more }`

  - `id: string`

    ID of remediation.

  - `effective_date: string`

    Date when the mitigation will become active. Time in RFC 3339 format (https://www.rfc-editor.org/rfc/rfc3339.html)

  - `entity_id: string`

  - `entity_type: "url_pattern" or "account" or "zone"`

    The type of entity targeted by a mitigation.

    - `"url_pattern"`

    - `"account"`

    - `"zone"`

  - `status: "pending" or "active" or "in_review" or 2 more`

    The status of a mitigation

    - `"pending"`

    - `"active"`

    - `"in_review"`

    - `"cancelled"`

    - `"removed"`

  - `type: "account_suspend" or "copyright_interstitial" or "geo_block" or 17 more`

    The type of mitigation applied to a reported entity.

    - `"account_suspend"`

    - `"copyright_interstitial"`

    - `"geo_block"`

    - `"legal_block"`

    - `"malware_interstitial"`

    - `"misleading_interstitial"`

    - `"network_block"`

    - `"phishing_interstitial"`

    - `"playfairite_enforce"`

    - `"r2_takedown_account"`

    - `"r2_takedown_bucket"`

    - `"r2_takedown_object"`

    - `"rate_limit_cache"`

    - `"redirect_video_stream"`

    - `"zone_fint"`

    - `"registrar_freeze"`

    - `"registrar_parking"`

    - `"stream_block_account"`

    - `"user_suspend"`

    - `"workers_takedown_by_zone_id"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/abuse-reports/$REPORT_ID/mitigations/appeal \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "type": "counter_notice"
        }'
```

#### Response

```json
{
  "result_info": {
    "count": 0,
    "page": 0,
    "per_page": 0,
    "total_count": 0,
    "total_pages": 0
  },
  "success": true,
  "errors": [
    {
      "message": "message"
    }
  ],
  "messages": [
    {
      "message": "message"
    }
  ],
  "result": [
    {
      "id": "id",
      "effective_date": "2009-11-10T23:00:00Z",
      "entity_id": "entity_id",
      "entity_type": "url_pattern",
      "status": "pending",
      "type": "account_suspend"
    }
  ]
}
```
