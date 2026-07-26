## Read a saved event query

**get** `/accounts/{account_id}/cloudforce-one/events/queries/{query_id}`

Retrieve a saved event query by its ID

### Path Parameters

- `account_id: string`

  Account ID.

- `query_id: number`

  Event query ID

### Returns

- `id: number`

  Unique identifier for the saved query

- `account_id: number`

  Account ID

- `alert_enabled: boolean`

  Whether alerts are enabled

- `alert_rollup_enabled: boolean`

  Whether alert rollup is enabled

- `created_at: string`

  Creation timestamp

- `name: string`

  Name of the saved query

- `query_json: string`

  JSON string containing the query parameters

- `rule_enabled: boolean`

  Whether rule is enabled

- `updated_at: string`

  Last update timestamp

- `user_email: string`

  Email of the user who created the query

- `custom_threat_feed_id: optional number`

  Intel Indicator Feed ID (numeric)

- `rule_list_id: optional string`

  WAF rules list ID for blocking

- `rule_scope: optional string`

  Scope for the rule

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries/$QUERY_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": 0,
  "account_id": 0,
  "alert_enabled": true,
  "alert_rollup_enabled": true,
  "created_at": "created_at",
  "name": "name",
  "query_json": "query_json",
  "rule_enabled": true,
  "updated_at": "updated_at",
  "user_email": "user_email",
  "custom_threat_feed_id": 0,
  "rule_list_id": "rule_list_id",
  "rule_scope": "rule_scope"
}
```
