---
title: List gateways
---

[Skip to content](#_top)

[API Reference](https://developers.cloudflare.com/api)

[AI Gateway](https://developers.cloudflare.com/api/resources/ai_gateway)

Copy Markdown

Open in **Claude**Open in **ChatGPT**Open in **Cursor**

---

**Copy Markdown****View as Markdown**

# List gateways

GET/accounts/{account\_id}/ai-gateway/gateways

Lists the AI Gateways in the account. Use `search` to filter by gateway ID.

##### Security

<details>

<summary>API Token</summary>



The preferred authorization scheme for interacting with the Cloudflare API. <a href="https://developers.cloudflare.com/fundamentals/api/get-started/create-token/">Create a token</a>.

**Example:**<code>Authorization: Bearer Sn3lZJTBX6kkg7OdcBUAxOO963GEIyGQqnFTOFYY</code>

</details>

<details>

<summary>API Email + API Key</summary>



The previous authorization scheme for interacting with the Cloudflare API, used in conjunction with a Global API key.

**Example:**<code>X-Auth-Email: user@example.com</code>

The previous authorization scheme for interacting with the Cloudflare API. When possible, use API tokens instead of Global API keys.

**Example:**<code>X-Auth-Key: 144c9defac04969c7bfad8efaa8ea194</code>

</details>

##### Accepted Permissions (at least one required)

`AI Gateway Write``AI Gateway Read`

##### P ath ParametersExpand Collapse

account\_id: string

[Link to this property](<#(resource)%20ai_gateway%20%3E%20(method)%20list%20%3E%20(params)%20default%20%3E%20(param)%20account_id%20%3E%20(schema)>)

##### Q uery ParametersExpand Collapse

page: optional number

minimum1

[Link to this property](<#(resource)%20ai_gateway%20%3E%20(method)%20list%20%3E%20(params)%20default%20%3E%20(param)%20page%20%3E%20(schema)>)

per\_page: optional number

maximum100

minimum1

[Link to this property](<#(resource)%20ai_gateway%20%3E%20(method)%20list%20%3E%20(params)%20default%20%3E%20(param)%20per_page%20%3E%20(schema)>)

search: optional string

Search by id

[Link to this property](<#(resource)%20ai_gateway%20%3E%20(method)%20list%20%3E%20(params)%20default%20%3E%20(param)%20search%20%3E%20(schema)>)

##### ReturnsExpand Collapse

<details>

<summary>

result: array of object {id, cache\_invalidate\_on\_update, cache\_ttl, 25 more }

</summary>

id: string

Unique identifier of the AI Gateway within the account.

maxLength64

minLength1

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

cache\_invalidate\_on\_update: boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20cache_invalidate_on_update">Link to this property</a>

cache\_ttl: number

minimum0

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20cache_ttl">Link to this property</a>

collect\_logs: boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20collect_logs">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

modified\_at: string

formatdate-time

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

rate\_limiting\_interval: number

minimum0

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20rate_limiting_interval">Link to this property</a>

rate\_limiting\_limit: number

minimum0

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20rate_limiting_limit">Link to this property</a>

authentication: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20authentication">Link to this property</a>

byok\_only: optional boolean

Requires customer-provided provider credentials and prevents fallback to Unified Billing.

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20byok_only">Link to this property</a>

<details>

<summary>

dlp: optional object {action, enabled, profiles } or object {enabled, policies }

</summary>

One of the following:

<details>

<summary>

object {action, enabled, profiles }

</summary>

<details>

<summary>

action: "BLOCK"or "FLAG"

</summary>

One of the following:

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%200%20%3E%20(property)%20action%20%3E%20(member)%200">Link to this property</a>

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%200%20%3E%20(property)%20action%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%200%20%3E%20(property)%20action">Link to this property</a>

enabled: boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%200%20%3E%20(property)%20enabled">Link to this property</a>

profiles: array of string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%200%20%3E%20(property)%20profiles">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

object {enabled, policies }

</summary>

enabled: boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

policies: array of object {id, action, check, 2 more }

</summary>

id: string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

action: "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20action%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20action%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20action">Link to this property</a>

<details>

<summary>

check: array of "REQUEST"or "RESPONSE"

</summary>

One of the following:

"REQUEST"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20check%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"RESPONSE"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20check%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20check">Link to this property</a>

enabled: boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20enabled">Link to this property</a>

profiles: array of string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies%20%3E%20(items)%20%3E%20(property)%20profiles">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201%20%3E%20(property)%20policies">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20dlp">Link to this property</a>

<details>

<summary>

guardrails: optional object {prompt, response }

</summary>

<details>

<summary>

prompt: object {P1, S1, S10, 11 more }

</summary>

<details>

<summary>

P1: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20P1%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20P1%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20P1">Link to this property</a>

<details>

<summary>

S1: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S1%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S1%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S1">Link to this property</a>

<details>

<summary>

S10: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S10%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S10%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S10">Link to this property</a>

<details>

<summary>

S11: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S11%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S11%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S11">Link to this property</a>

<details>

<summary>

S12: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S12%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S12%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S12">Link to this property</a>

<details>

<summary>

S13: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S13%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S13%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S13">Link to this property</a>

<details>

<summary>

S2: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S2%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S2%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S2">Link to this property</a>

<details>

<summary>

S3: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S3%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S3%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S3">Link to this property</a>

<details>

<summary>

S4: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S4%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S4%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S4">Link to this property</a>

<details>

<summary>

S5: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S5%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S5%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S5">Link to this property</a>

<details>

<summary>

S6: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S6%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S6%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S6">Link to this property</a>

<details>

<summary>

S7: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S7%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S7%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S7">Link to this property</a>

<details>

<summary>

S8: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S8%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S8%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S8">Link to this property</a>

<details>

<summary>

S9: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S9%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S9%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt%20%3E%20(property)%20S9">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20prompt">Link to this property</a>

<details>

<summary>

response: object {P1, S1, S10, 11 more }

</summary>

<details>

<summary>

P1: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20P1%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20P1%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20P1">Link to this property</a>

<details>

<summary>

S1: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S1%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S1%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S1">Link to this property</a>

<details>

<summary>

S10: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S10%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S10%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S10">Link to this property</a>

<details>

<summary>

S11: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S11%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S11%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S11">Link to this property</a>

<details>

<summary>

S12: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S12%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S12%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S12">Link to this property</a>

<details>

<summary>

S13: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S13%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S13%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S13">Link to this property</a>

<details>

<summary>

S2: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S2%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S2%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S2">Link to this property</a>

<details>

<summary>

S3: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S3%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S3%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S3">Link to this property</a>

<details>

<summary>

S4: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S4%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S4%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S4">Link to this property</a>

<details>

<summary>

S5: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S5%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S5%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S5">Link to this property</a>

<details>

<summary>

S6: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S6%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S6%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S6">Link to this property</a>

<details>

<summary>

S7: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S7%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S7%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S7">Link to this property</a>

<details>

<summary>

S8: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S8%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S8%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S8">Link to this property</a>

<details>

<summary>

S9: optional "FLAG"or "BLOCK"

</summary>

One of the following:

"FLAG"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S9%20%3E%20(member)%200">Link to this property</a>

"BLOCK"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S9%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response%20%3E%20(property)%20S9">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails%20%3E%20(property)%20response">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20guardrails">Link to this property</a>

is\_default: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20is_default">Link to this property</a>

log\_classification: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20log_classification">Link to this property</a>

log\_management: optional number

maximum10000000

minimum10000

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20log_management">Link to this property</a>

<details>

<summary>

log\_management\_strategy: optional "STOP\_INSERTING"or "DELETE\_OLDEST"

</summary>

One of the following:

"STOP\_INSERTING"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20log_management_strategy%20%3E%20(member)%200">Link to this property</a>

"DELETE\_OLDEST"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20log_management_strategy%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20log_management_strategy">Link to this property</a>

logpush: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20logpush">Link to this property</a>

logpush\_public\_key: optional string

maxLength1024

minLength16

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20logpush_public_key">Link to this property</a>

<details>

<summary>

otel: optional array of object {headers, url, authorization, content\_type }

</summary>

headers: map\[string]

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel%20%3E%20(items)%20%3E%20(property)%20headers">Link to this property</a>

url: string

formaturi

maxLength2048

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel%20%3E%20(items)%20%3E%20(property)%20url">Link to this property</a>

authorization: optional string

maxLength256

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel%20%3E%20(items)%20%3E%20(property)%20authorization">Link to this property</a>

<details>

<summary>

content\_type: optional "json"or "protobuf"

</summary>

One of the following:

"json"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel%20%3E%20(items)%20%3E%20(property)%20content_type%20%3E%20(member)%200">Link to this property</a>

"protobuf"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel%20%3E%20(items)%20%3E%20(property)%20content_type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel%20%3E%20(items)%20%3E%20(property)%20content_type">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20otel">Link to this property</a>

<details>

<summary>

rate\_limiting\_technique: optional "fixed"or "sliding"

</summary>

One of the following:

"fixed"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20rate_limiting_technique%20%3E%20(member)%200">Link to this property</a>

"sliding"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20rate_limiting_technique%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20rate_limiting_technique">Link to this property</a>

<details>

<summary>

retry\_backoff: optional "constant"or "linear"or "exponential"

Backoff strategy for retry delays

</summary>

One of the following:

"constant"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_backoff%20%3E%20(member)%200">Link to this property</a>

"linear"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_backoff%20%3E%20(member)%201">Link to this property</a>

"exponential"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_backoff%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_backoff">Link to this property</a>

retry\_delay: optional number

Delay between retry attempts in milliseconds (0-60000)

maximum60000

minimum0

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_delay">Link to this property</a>

retry\_max\_attempts: optional number

Maximum number of retry attempts for failed requests (1-5)

maximum5

minimum1

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_max_attempts">Link to this property</a>

<details>

<summary>

spend\_limits: optional object {enabled, rules }

</summary>

enabled: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

rules: optional array of object {limit, limitType, window, 6 more }

</summary>

limit: number

exclusiveMinimum

minimum0

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20limit">Link to this property</a>

limitType: "cost"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20limitType">Link to this property</a>

window: number

exclusiveMinimum

minimum0

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20window">Link to this property</a>

id: optional string

minLength1

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

metadata: optional map\[object {mode } or object {mode, values } ]

</summary>

One of the following:

<details>

<summary>

Mode object {mode }

</summary>

mode: "partition"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20metadata%20%3E%20(items)%20%3E%20(variant)%200%20%3E%20(property)%20mode">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20metadata%20%3E%20(items)%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

object {mode, values }

</summary>

mode: "filter"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20metadata%20%3E%20(items)%20%3E%20(variant)%201%20%3E%20(property)%20mode">Link to this property</a>

values: array of string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20metadata%20%3E%20(items)%20%3E%20(variant)%201%20%3E%20(property)%20values">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20metadata%20%3E%20(items)%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20metadata">Link to this property</a>

<details>

<summary>

model: optional object {mode, values }

</summary>

mode: "filter"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20model%20%3E%20(property)%20mode">Link to this property</a>

values: array of string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20model%20%3E%20(property)%20values">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20model">Link to this property</a>

<details>

<summary>

provider: optional object {mode, values }

</summary>

mode: "filter"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20provider%20%3E%20(property)%20mode">Link to this property</a>

values: array of string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20provider%20%3E%20(property)%20values">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20provider">Link to this property</a>

<details>

<summary>

technique: optional "fixed"or "sliding"

</summary>

One of the following:

"fixed"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20technique%20%3E%20(member)%200">Link to this property</a>

"sliding"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20technique%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules%20%3E%20(items)%20%3E%20(property)%20technique">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits%20%3E%20(property)%20rules">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20spend_limits">Link to this property</a>

store\_id: optional string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20store_id">Link to this property</a>

<details>

<summary>

stripe: optional object {authorization, usage\_events }

</summary>

authorization: string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20stripe%20%3E%20(property)%20authorization">Link to this property</a>

<details>

<summary>

usage\_events: array of object {payload }

</summary>

payload: string

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20stripe%20%3E%20(property)%20usage_events%20%3E%20(items)%20%3E%20(property)%20payload">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20stripe%20%3E%20(property)%20usage_events">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20stripe">Link to this property</a>

<details>

<summary>

workers\_ai\_billing\_mode: optional "postpaid"or "unified"

Controls how Workers AI inference calls routed through this gateway are billed. ‘postpaid’ bills the account directly through Workers AI; ‘unified’ deducts credits via AI Gateway using neuron-based pricing and delegates billing to AI Gateway.

</summary>

One of the following:

"postpaid"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20workers_ai_billing_mode%20%3E%20(member)%200">Link to this property</a>

"unified"

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20workers_ai_billing_mode%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20workers_ai_billing_mode">Link to this property</a>

zdr: optional boolean

<a href="#(resource)%20ai_gateway%20%3E%20(model)%20ai_gateway_list_response%20%3E%20(schema)%20%3E%20(property)%20zdr">Link to this property</a>

</details>

[Link to this property](<#(resource)%20ai_gateway%20%3E%20(method)%20list%20%3E%20(network%20schema)%20%3E%20(property)%20result>)

success: boolean

[Link to this property](<#(resource)%20ai_gateway%20%3E%20(method)%20list%20%3E%20(network%20schema)%20%3E%20(property)%20success>)

### List gateways

HTTP

HTTPTypeScriptPythonGoTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/ai-gateway/gateways \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

200 example

```
{
  "result": [
    {
      "id": "my-gateway",
      "cache_invalidate_on_update": true,
      "cache_ttl": 0,
      "collect_logs": true,
      "created_at": "2019-12-27T18:11:19.117Z",
      "modified_at": "2019-12-27T18:11:19.117Z",
      "rate_limiting_interval": 0,
      "rate_limiting_limit": 0,
      "authentication": true,
      "byok_only": true,
      "dlp": {
        "action": "BLOCK",
        "enabled": true,
        "profiles": [
          "string"
        ]
      },
      "guardrails": {
        "prompt": {
          "P1": "FLAG",
          "S1": "FLAG",
          "S10": "FLAG",
          "S11": "FLAG",
          "S12": "FLAG",
          "S13": "FLAG",
          "S2": "FLAG",
          "S3": "FLAG",
          "S4": "FLAG",
          "S5": "FLAG",
          "S6": "FLAG",
          "S7": "FLAG",
          "S8": "FLAG",
          "S9": "FLAG"
        },
        "response": {
          "P1": "FLAG",
          "S1": "FLAG",
          "S10": "FLAG",
          "S11": "FLAG",
          "S12": "FLAG",
          "S13": "FLAG",
          "S2": "FLAG",
          "S3": "FLAG",
          "S4": "FLAG",
          "S5": "FLAG",
          "S6": "FLAG",
          "S7": "FLAG",
          "S8": "FLAG",
          "S9": "FLAG"
        }
      },
      "is_default": true,
      "log_classification": true,
      "log_management": 10000,
      "log_management_strategy": "STOP_INSERTING",
      "logpush": true,
      "logpush_public_key": "xxxxxxxxxxxxxxxx",
      "otel": [
        {
          "headers": {
            "foo": "string"
          },
          "url": "https://example.com",
          "authorization": "authorization",
          "content_type": "json"
        }
      ],
      "rate_limiting_technique": "fixed",
      "retry_backoff": "constant",
      "retry_delay": 0,
      "retry_max_attempts": 1,
      "spend_limits": {
        "enabled": true,
        "rules": [
          {
            "limit": 1,
            "limitType": "cost",
            "window": 1,
            "id": "x",
            "enabled": true,
            "metadata": {
              "foo": {
                "mode": "partition"
              }
            },
            "model": {
              "mode": "filter",
              "values": [
                "string"
              ]
            },
            "provider": {
              "mode": "filter",
              "values": [
                "string"
              ]
            },
            "technique": "fixed"
          }
        ]
      },
      "store_id": "store_id",
      "stripe": {
        "authorization": "authorization",
        "usage_events": [
          {
            "payload": "payload"
          }
        ]
      },
      "workers_ai_billing_mode": "postpaid",
      "zdr": true
    }
  ],
  "success": true
}
```

##### Returns Examples

200 example

```
{
  "result": [
    {
      "id": "my-gateway",
      "cache_invalidate_on_update": true,
      "cache_ttl": 0,
      "collect_logs": true,
      "created_at": "2019-12-27T18:11:19.117Z",
      "modified_at": "2019-12-27T18:11:19.117Z",
      "rate_limiting_interval": 0,
      "rate_limiting_limit": 0,
      "authentication": true,
      "byok_only": true,
      "dlp": {
        "action": "BLOCK",
        "enabled": true,
        "profiles": [
          "string"
        ]
      },
      "guardrails": {
        "prompt": {
          "P1": "FLAG",
          "S1": "FLAG",
          "S10": "FLAG",
          "S11": "FLAG",
          "S12": "FLAG",
          "S13": "FLAG",
          "S2": "FLAG",
          "S3": "FLAG",
          "S4": "FLAG",
          "S5": "FLAG",
          "S6": "FLAG",
          "S7": "FLAG",
          "S8": "FLAG",
          "S9": "FLAG"
        },
        "response": {
          "P1": "FLAG",
          "S1": "FLAG",
          "S10": "FLAG",
          "S11": "FLAG",
          "S12": "FLAG",
          "S13": "FLAG",
          "S2": "FLAG",
          "S3": "FLAG",
          "S4": "FLAG",
          "S5": "FLAG",
          "S6": "FLAG",
          "S7": "FLAG",
          "S8": "FLAG",
          "S9": "FLAG"
        }
      },
      "is_default": true,
      "log_classification": true,
      "log_management": 10000,
      "log_management_strategy": "STOP_INSERTING",
      "logpush": true,
      "logpush_public_key": "xxxxxxxxxxxxxxxx",
      "otel": [
        {
          "headers": {
            "foo": "string"
          },
          "url": "https://example.com",
          "authorization": "authorization",
          "content_type": "json"
        }
      ],
      "rate_limiting_technique": "fixed",
      "retry_backoff": "constant",
      "retry_delay": 0,
      "retry_max_attempts": 1,
      "spend_limits": {
        "enabled": true,
        "rules": [
          {
            "limit": 1,
            "limitType": "cost",
            "window": 1,
            "id": "x",
            "enabled": true,
            "metadata": {
              "foo": {
                "mode": "partition"
              }
            },
            "model": {
              "mode": "filter",
              "values": [
                "string"
              ]
            },
            "provider": {
              "mode": "filter",
              "values": [
                "string"
              ]
            },
            "technique": "fixed"
          }
        ]
      },
      "store_id": "store_id",
      "stripe": {
        "authorization": "authorization",
        "usage_events": [
          {
            "payload": "payload"
          }
        ]
      },
      "workers_ai_billing_mode": "postpaid",
      "zdr": true
    }
  ],
  "success": true
}
```