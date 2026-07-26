## Create flag

**post** `/accounts/{account_id}/flagship/apps/{app_id}/flags`

Creates a flag. Returns 409 if the key already exists. `type` is inferred from variation values and may be omitted.

### Path Parameters

- `account_id: string`

  Cloudflare account ID.

- `app_id: string`

  App identifier.

### Body Parameters

- `default_variation: string`

  Variation the API serves when the flag is off, or when it's on but no rule matches the context. Must be a key in `variations`.

- `enabled: boolean`

  When false, the flag bypasses all rules and always serves `default_variation`.

- `key: string`

  Unique identifier for the flag within an app. Used in all evaluation and SDK calls.

- `rules: array of object { conditions, priority, serve_variation, rollout }`

  Targeting rules evaluated in ascending `priority`; the first matching rule wins. An empty array means the flag always serves `default_variation`.

  - `conditions: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

    Conditions the context must satisfy for this rule to match. An empty array matches all contexts.

    - `object { attribute, operator, value }`

      - `attribute: string`

      - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

        - `"equals"`

        - `"not_equals"`

        - `"greater_than"`

        - `"less_than"`

        - `"greater_than_or_equals"`

        - `"less_than_or_equals"`

        - `"contains"`

        - `"starts_with"`

        - `"ends_with"`

        - `"in"`

        - `"not_in"`

      - `value: string or number or boolean or 2 more`

        - `string`

        - `number`

        - `boolean`

        - `map[unknown]`

        - `array of unknown`

    - `object { clauses, logical_operator }`

      - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

        - `object { attribute, operator, value }`

          - `attribute: string`

          - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

            - `"equals"`

            - `"not_equals"`

            - `"greater_than"`

            - `"less_than"`

            - `"greater_than_or_equals"`

            - `"less_than_or_equals"`

            - `"contains"`

            - `"starts_with"`

            - `"ends_with"`

            - `"in"`

            - `"not_in"`

          - `value: string or number or boolean or 2 more`

            - `string`

            - `number`

            - `boolean`

            - `map[unknown]`

            - `array of unknown`

        - `object { clauses, logical_operator }`

          - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

            - `object { attribute, operator, value }`

              - `attribute: string`

              - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                - `"equals"`

                - `"not_equals"`

                - `"greater_than"`

                - `"less_than"`

                - `"greater_than_or_equals"`

                - `"less_than_or_equals"`

                - `"contains"`

                - `"starts_with"`

                - `"ends_with"`

                - `"in"`

                - `"not_in"`

              - `value: string or number or boolean or 2 more`

                - `string`

                - `number`

                - `boolean`

                - `map[unknown]`

                - `array of unknown`

            - `object { clauses, logical_operator }`

              - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

                - `object { attribute, operator, value }`

                  - `attribute: string`

                  - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                    - `"equals"`

                    - `"not_equals"`

                    - `"greater_than"`

                    - `"less_than"`

                    - `"greater_than_or_equals"`

                    - `"less_than_or_equals"`

                    - `"contains"`

                    - `"starts_with"`

                    - `"ends_with"`

                    - `"in"`

                    - `"not_in"`

                  - `value: string or number or boolean or 2 more`

                    - `string`

                    - `number`

                    - `boolean`

                    - `map[unknown]`

                    - `array of unknown`

                - `object { clauses, logical_operator }`

                  - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

                    - `object { attribute, operator, value }`

                      - `attribute: string`

                      - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                        - `"equals"`

                        - `"not_equals"`

                        - `"greater_than"`

                        - `"less_than"`

                        - `"greater_than_or_equals"`

                        - `"less_than_or_equals"`

                        - `"contains"`

                        - `"starts_with"`

                        - `"ends_with"`

                        - `"in"`

                        - `"not_in"`

                      - `value: string or number or boolean or 2 more`

                        - `string`

                        - `number`

                        - `boolean`

                        - `map[unknown]`

                        - `array of unknown`

                    - `object { clauses, logical_operator }`

                      - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

                        - `object { attribute, operator, value }`

                          - `attribute: string`

                          - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                            - `"equals"`

                            - `"not_equals"`

                            - `"greater_than"`

                            - `"less_than"`

                            - `"greater_than_or_equals"`

                            - `"less_than_or_equals"`

                            - `"contains"`

                            - `"starts_with"`

                            - `"ends_with"`

                            - `"in"`

                            - `"not_in"`

                          - `value: string or number or boolean or 2 more`

                            - `string`

                            - `number`

                            - `boolean`

                            - `map[unknown]`

                            - `array of unknown`

                        - `object { clauses, logical_operator }`

                          - `clauses: array of string or number or boolean or 2 more`

                            - `string`

                            - `number`

                            - `boolean`

                            - `map[unknown]`

                            - `array of unknown`

                          - `logical_operator: "AND" or "OR"`

                            - `"AND"`

                            - `"OR"`

                      - `logical_operator: "AND" or "OR"`

                        - `"AND"`

                        - `"OR"`

                  - `logical_operator: "AND" or "OR"`

                    - `"AND"`

                    - `"OR"`

              - `logical_operator: "AND" or "OR"`

                - `"AND"`

                - `"OR"`

          - `logical_operator: "AND" or "OR"`

            - `"AND"`

            - `"OR"`

      - `logical_operator: "AND" or "OR"`

        - `"AND"`

        - `"OR"`

  - `priority: number`

    Evaluation order: the API evaluates rules with lower numbers first. Must be unique across the flag's rules.

  - `serve_variation: string`

    Variation the API serves when this rule matches. Must be a key in `variations`.

  - `rollout: optional object { percentage, attribute }`

    - `percentage: number`

      Percentage of matching traffic (0–100) served this variation. For multi-way splits, use cumulative upper bounds across rules (e.g. 30, 70, 100).

    - `attribute: optional string`

      Context attribute used for sticky bucketing. Defaults to `targetingKey`. If absent at evaluation time, bucketing is random per request.

- `variations: map[string or number or boolean or 2 more]`

  Map of variation name to value. All values share the same type (boolean, string, number, or JSON object/array), and each serialized value stays within 10KB.

  - `string`

  - `number`

  - `boolean`

  - `map[unknown]`

  - `array of unknown`

- `description: optional string`

- `type: optional "boolean" or "string" or "number" or "json"`

  Value type of the flag's variations. The API infers this from the variation values on write, so you can omit it in requests.

  - `"boolean"`

  - `"string"`

  - `"number"`

  - `"json"`

### Returns

- `errors: array of object { message }`

  - `message: string`

- `messages: array of object { message }`

  - `message: string`

- `result: object { default_variation, enabled, key, 6 more }`

  - `default_variation: string`

    Variation the API serves when the flag is off, or when it's on but no rule matches the context. Must be a key in `variations`.

  - `enabled: boolean`

    When false, the flag bypasses all rules and always serves `default_variation`.

  - `key: string`

    Unique identifier for the flag within an app. Used in all evaluation and SDK calls.

  - `rules: array of object { conditions, priority, serve_variation, rollout }`

    Targeting rules evaluated in ascending `priority`; the first matching rule wins. An empty array means the flag always serves `default_variation`.

    - `conditions: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

      Conditions the context must satisfy for this rule to match. An empty array matches all contexts.

      - `object { attribute, operator, value }`

        - `attribute: string`

        - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

          - `"equals"`

          - `"not_equals"`

          - `"greater_than"`

          - `"less_than"`

          - `"greater_than_or_equals"`

          - `"less_than_or_equals"`

          - `"contains"`

          - `"starts_with"`

          - `"ends_with"`

          - `"in"`

          - `"not_in"`

        - `value: string or number or boolean or 2 more`

          - `string`

          - `number`

          - `boolean`

          - `map[unknown]`

          - `array of unknown`

      - `object { clauses, logical_operator }`

        - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

          - `object { attribute, operator, value }`

            - `attribute: string`

            - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

              - `"equals"`

              - `"not_equals"`

              - `"greater_than"`

              - `"less_than"`

              - `"greater_than_or_equals"`

              - `"less_than_or_equals"`

              - `"contains"`

              - `"starts_with"`

              - `"ends_with"`

              - `"in"`

              - `"not_in"`

            - `value: string or number or boolean or 2 more`

              - `string`

              - `number`

              - `boolean`

              - `map[unknown]`

              - `array of unknown`

          - `object { clauses, logical_operator }`

            - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

              - `object { attribute, operator, value }`

                - `attribute: string`

                - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                  - `"equals"`

                  - `"not_equals"`

                  - `"greater_than"`

                  - `"less_than"`

                  - `"greater_than_or_equals"`

                  - `"less_than_or_equals"`

                  - `"contains"`

                  - `"starts_with"`

                  - `"ends_with"`

                  - `"in"`

                  - `"not_in"`

                - `value: string or number or boolean or 2 more`

                  - `string`

                  - `number`

                  - `boolean`

                  - `map[unknown]`

                  - `array of unknown`

              - `object { clauses, logical_operator }`

                - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

                  - `object { attribute, operator, value }`

                    - `attribute: string`

                    - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                      - `"equals"`

                      - `"not_equals"`

                      - `"greater_than"`

                      - `"less_than"`

                      - `"greater_than_or_equals"`

                      - `"less_than_or_equals"`

                      - `"contains"`

                      - `"starts_with"`

                      - `"ends_with"`

                      - `"in"`

                      - `"not_in"`

                    - `value: string or number or boolean or 2 more`

                      - `string`

                      - `number`

                      - `boolean`

                      - `map[unknown]`

                      - `array of unknown`

                  - `object { clauses, logical_operator }`

                    - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

                      - `object { attribute, operator, value }`

                        - `attribute: string`

                        - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                          - `"equals"`

                          - `"not_equals"`

                          - `"greater_than"`

                          - `"less_than"`

                          - `"greater_than_or_equals"`

                          - `"less_than_or_equals"`

                          - `"contains"`

                          - `"starts_with"`

                          - `"ends_with"`

                          - `"in"`

                          - `"not_in"`

                        - `value: string or number or boolean or 2 more`

                          - `string`

                          - `number`

                          - `boolean`

                          - `map[unknown]`

                          - `array of unknown`

                      - `object { clauses, logical_operator }`

                        - `clauses: array of object { attribute, operator, value }  or object { clauses, logical_operator }`

                          - `object { attribute, operator, value }`

                            - `attribute: string`

                            - `operator: "equals" or "not_equals" or "greater_than" or 8 more`

                              - `"equals"`

                              - `"not_equals"`

                              - `"greater_than"`

                              - `"less_than"`

                              - `"greater_than_or_equals"`

                              - `"less_than_or_equals"`

                              - `"contains"`

                              - `"starts_with"`

                              - `"ends_with"`

                              - `"in"`

                              - `"not_in"`

                            - `value: string or number or boolean or 2 more`

                              - `string`

                              - `number`

                              - `boolean`

                              - `map[unknown]`

                              - `array of unknown`

                          - `object { clauses, logical_operator }`

                            - `clauses: array of string or number or boolean or 2 more`

                              - `string`

                              - `number`

                              - `boolean`

                              - `map[unknown]`

                              - `array of unknown`

                            - `logical_operator: "AND" or "OR"`

                              - `"AND"`

                              - `"OR"`

                        - `logical_operator: "AND" or "OR"`

                          - `"AND"`

                          - `"OR"`

                    - `logical_operator: "AND" or "OR"`

                      - `"AND"`

                      - `"OR"`

                - `logical_operator: "AND" or "OR"`

                  - `"AND"`

                  - `"OR"`

            - `logical_operator: "AND" or "OR"`

              - `"AND"`

              - `"OR"`

        - `logical_operator: "AND" or "OR"`

          - `"AND"`

          - `"OR"`

    - `priority: number`

      Evaluation order: the API evaluates rules with lower numbers first. Must be unique across the flag's rules.

    - `serve_variation: string`

      Variation the API serves when this rule matches. Must be a key in `variations`.

    - `rollout: optional object { percentage, attribute }`

      - `percentage: number`

        Percentage of matching traffic (0–100) served this variation. For multi-way splits, use cumulative upper bounds across rules (e.g. 30, 70, 100).

      - `attribute: optional string`

        Context attribute used for sticky bucketing. Defaults to `targetingKey`. If absent at evaluation time, bucketing is random per request.

  - `variations: map[string or number or boolean or 2 more]`

    Map of variation name to value. All values share the same type (boolean, string, number, or JSON object/array), and each serialized value stays within 10KB.

    - `string`

    - `number`

    - `boolean`

    - `map[unknown]`

    - `array of unknown`

  - `description: optional string`

  - `type: optional "boolean" or "string" or "number" or "json"`

    Value type of the flag's variations. The API infers this from the variation values on write, so you can omit it in requests.

    - `"boolean"`

    - `"string"`

    - `"number"`

    - `"json"`

  - `updated_at: optional string`

  - `updated_by: optional string`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/flagship/apps/$APP_ID/flags \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "default_variation": "x",
          "enabled": true,
          "key": "x",
          "rules": [
            {
              "conditions": [
                {
                  "attribute": "x",
                  "operator": "equals",
                  "value": "string"
                }
              ],
              "priority": 1,
              "serve_variation": "x"
            }
          ],
          "variations": {
            "foo": "string"
          }
        }'
```

#### Response

```json
{
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
  "result": {
    "default_variation": "x",
    "enabled": true,
    "key": "x",
    "rules": [
      {
        "conditions": [
          {
            "attribute": "x",
            "operator": "equals",
            "value": "string"
          }
        ],
        "priority": 1,
        "serve_variation": "x",
        "rollout": {
          "percentage": 0,
          "attribute": "x"
        }
      }
    ],
    "variations": {
      "foo": "string"
    },
    "description": "description",
    "type": "boolean",
    "updated_at": "updated_at",
    "updated_by": "updated_by"
  },
  "success": true
}
```
