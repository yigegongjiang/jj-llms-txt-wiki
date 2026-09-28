---
title: Email Security
---

[Skip to content](#_top)

[API Reference](https://developers.cloudflare.com/api)

Copy Markdown

Open in **Claude**Open in **ChatGPT**Open in **Cursor**

---

**Copy Markdown****View as Markdown**

# Email Security

#### Email SecurityInvestigate

##### [Search email messages](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/methods/list)

GET/accounts/{account\_id}/email-security/investigate

##### [Get message details](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/methods/get)

GET/accounts/{account\_id}/email-security/investigate/{investigate\_id}

##### ModelsExpand Collapse

<details>

<summary>

InvestigateListResponse object {id, action\_log, client\_recipients, 32 more }

</summary>

id: string

Unique identifier for a message retrieved from investigation.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

Deprecatedaction\_log: array of object {completed\_at, operation, completed\_timestamp, 2 more }

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

</summary>

completed\_at: string

Timestamp when action completed.

formatdate-time

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20completed_at">Link to this property</a>

<details>

<summary>

operation: "MOVE"or "RELEASE"or "RECLASSIFY"or 3 more

Type of action performed.

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%201">Link to this property</a>

"RECLASSIFY"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%202">Link to this property</a>

"SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%203">Link to this property</a>

"QUARANTINE\_RELEASE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%204">Link to this property</a>

"PREVIEW"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation">Link to this property</a>

Deprecatedcompleted\_timestamp: optional string

Use <code>completed_at</code> instead.

Deprecated, use <code>completed_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20completed_timestamp">Link to this property</a>

<details>

<summary>

properties: optional object {folder, requested\_by }

Additional properties for the action.

</summary>

folder: optional string

Target folder for move operations.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties%20%3E%20(property)%20folder">Link to this property</a>

requested\_by: optional string

User who requested the action.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties%20%3E%20(property)%20requested_by">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties">Link to this property</a>

status: optional string

Status of the action.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20status">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20action_log">Link to this property</a>

client\_recipients: array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20client_recipients">Link to this property</a>

detection\_reasons: array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20detection_reasons">Link to this property</a>

is\_phish\_submission: boolean

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20is_phish_submission">Link to this property</a>

is\_quarantined: boolean

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20is_quarantined">Link to this property</a>

postfix\_id: string

The identifier of the message.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20postfix_id">Link to this property</a>

<details>

<summary>

properties: object {allowlisted\_pattern, allowlisted\_pattern\_type, blocklisted\_message, 2 more }

Message processing properties.

</summary>

allowlisted\_pattern: optional string

Pattern that allowlisted this message.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern">Link to this property</a>

<details>

<summary>

allowlisted\_pattern\_type: optional "quarantine\_release"or "acceptable\_sender"or "allowed\_sender"or 5 more

Type of allowlist pattern.

</summary>

One of the following:

"quarantine\_release"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%200">Link to this property</a>

"acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%201">Link to this property</a>

"allowed\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%202">Link to this property</a>

"allowed\_recipient"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%203">Link to this property</a>

"domain\_similarity"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%204">Link to this property</a>

"domain\_recency"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%205">Link to this property</a>

"managed\_acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%206">Link to this property</a>

"outbound\_ndr"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type">Link to this property</a>

blocklisted\_message: optional boolean

Whether message was blocklisted.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20blocklisted_message">Link to this property</a>

blocklisted\_pattern: optional string

Pattern that blocklisted this message.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20blocklisted_pattern">Link to this property</a>

<details>

<summary>

whitelisted\_pattern\_type: optional "quarantine\_release"or "acceptable\_sender"or "allowed\_sender"or 5 more

Legacy field for allowlist pattern type.

</summary>

One of the following:

"quarantine\_release"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%200">Link to this property</a>

"acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%201">Link to this property</a>

"allowed\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%202">Link to this property</a>

"allowed\_recipient"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%203">Link to this property</a>

"domain\_similarity"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%204">Link to this property</a>

"domain\_recency"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%205">Link to this property</a>

"managed\_acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%206">Link to this property</a>

"outbound\_ndr"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20properties">Link to this property</a>

Deprecatedts: string

Use <code>scanned_at</code> instead.

Deprecated, use <code>scanned_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20ts">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_mode: optional "DIRECT"or "BCC"or "JOURNAL"or 8 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%202">Link to this property</a>

"REVIEW\_SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%203">Link to this property</a>

"DMARC\_UNVERIFIED"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%204">Link to this property</a>

"DMARC\_FAILURE\_REPORT"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%205">Link to this property</a>

"DMARC\_AGGREGATE\_REPORT"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%206">Link to this property</a>

"THREAT\_INTEL\_SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%207">Link to this property</a>

"SIMULATION\_SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%208">Link to this property</a>

"API"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%209">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%2010">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode">Link to this property</a>

<details>

<summary>

delivery\_status: optional array of "delivered"or "moved"or "quarantined"or 5 more

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status">Link to this property</a>

edf\_hash: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20edf_hash">Link to this property</a>

envelope\_from: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20envelope_from">Link to this property</a>

envelope\_to: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20envelope_to">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

Deprecatedfindings: optional array of object {attachment, detail, detection, 6 more }

Use the <code>findings</code> field from GET /investigate/{investigate\_id}/detections instead.

Deprecated, use the <code>findings</code> field from <code>GET /investigate/{investigate_id}/detections</code> instead. End of life: November 1, 2026. Detection findings for this message.

</summary>

attachment: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20attachment">Link to this property</a>

detail: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detail">Link to this property</a>

<details>

<summary>

detection: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection">Link to this property</a>

field: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20field">Link to this property</a>

name: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

portion: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20portion">Link to this property</a>

reason: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20reason">Link to this property</a>

score: optional number

formatdouble

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20score">Link to this property</a>

value: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20findings">Link to this property</a>

from: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20from">Link to this property</a>

from\_name: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20from_name">Link to this property</a>

htmltext\_structure\_hash: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20htmltext_structure_hash">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20message_id">Link to this property</a>

<details>

<summary>

post\_delivery\_operations: optional array of "PREVIEW"or "QUARANTINE\_RELEASE"or "SUBMISSION"or "MOVE"

Post-delivery operations performed on this message.

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"MOVE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations">Link to this property</a>

postfix\_id\_outbound: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20postfix_id_outbound">Link to this property</a>

replyto: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20replyto">Link to this property</a>

scanned\_at: optional string

When the message was scanned (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20scanned_at">Link to this property</a>

sent\_at: optional string

When the message was sent (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20sent_at">Link to this property</a>

sent\_date: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20sent_date">Link to this property</a>

smtp\_helo\_server\_ip: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20smtp_helo_server_ip">Link to this property</a>

smtp\_previous\_hop\_ip: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20smtp_previous_hop_ip">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20subject">Link to this property</a>

threat\_categories: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20threat_categories">Link to this property</a>

to: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20to">Link to this property</a>

to\_name: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20to_name">Link to this property</a>

<details>

<summary>

validation: optional object {comment, dkim, dmarc, spf }

</summary>

comment: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

dkim: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim">Link to this property</a>

<details>

<summary>

dmarc: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc">Link to this property</a>

<details>

<summary>

spf: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20validation">Link to this property</a>

x\_originating\_ip: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)%20%3E%20(property)%20x_originating_ip">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_list_response%20%3E%20(schema)>)

<details>

<summary>

InvestigateGetResponse object {id, action\_log, client\_recipients, 32 more }

</summary>

id: string

Unique identifier for a message retrieved from investigation.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

Deprecatedaction\_log: array of object {completed\_at, operation, completed\_timestamp, 2 more }

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

</summary>

completed\_at: string

Timestamp when action completed.

formatdate-time

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20completed_at">Link to this property</a>

<details>

<summary>

operation: "MOVE"or "RELEASE"or "RECLASSIFY"or 3 more

Type of action performed.

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%201">Link to this property</a>

"RECLASSIFY"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%202">Link to this property</a>

"SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%203">Link to this property</a>

"QUARANTINE\_RELEASE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%204">Link to this property</a>

"PREVIEW"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation">Link to this property</a>

Deprecatedcompleted\_timestamp: optional string

Use <code>completed_at</code> instead.

Deprecated, use <code>completed_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20completed_timestamp">Link to this property</a>

<details>

<summary>

properties: optional object {folder, requested\_by }

Additional properties for the action.

</summary>

folder: optional string

Target folder for move operations.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties%20%3E%20(property)%20folder">Link to this property</a>

requested\_by: optional string

User who requested the action.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties%20%3E%20(property)%20requested_by">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties">Link to this property</a>

status: optional string

Status of the action.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20status">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20action_log">Link to this property</a>

client\_recipients: array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20client_recipients">Link to this property</a>

detection\_reasons: array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20detection_reasons">Link to this property</a>

is\_phish\_submission: boolean

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20is_phish_submission">Link to this property</a>

is\_quarantined: boolean

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20is_quarantined">Link to this property</a>

postfix\_id: string

The identifier of the message.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20postfix_id">Link to this property</a>

<details>

<summary>

properties: object {allowlisted\_pattern, allowlisted\_pattern\_type, blocklisted\_message, 2 more }

Message processing properties.

</summary>

allowlisted\_pattern: optional string

Pattern that allowlisted this message.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern">Link to this property</a>

<details>

<summary>

allowlisted\_pattern\_type: optional "quarantine\_release"or "acceptable\_sender"or "allowed\_sender"or 5 more

Type of allowlist pattern.

</summary>

One of the following:

"quarantine\_release"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%200">Link to this property</a>

"acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%201">Link to this property</a>

"allowed\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%202">Link to this property</a>

"allowed\_recipient"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%203">Link to this property</a>

"domain\_similarity"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%204">Link to this property</a>

"domain\_recency"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%205">Link to this property</a>

"managed\_acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%206">Link to this property</a>

"outbound\_ndr"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type">Link to this property</a>

blocklisted\_message: optional boolean

Whether message was blocklisted.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20blocklisted_message">Link to this property</a>

blocklisted\_pattern: optional string

Pattern that blocklisted this message.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20blocklisted_pattern">Link to this property</a>

<details>

<summary>

whitelisted\_pattern\_type: optional "quarantine\_release"or "acceptable\_sender"or "allowed\_sender"or 5 more

Legacy field for allowlist pattern type.

</summary>

One of the following:

"quarantine\_release"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%200">Link to this property</a>

"acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%201">Link to this property</a>

"allowed\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%202">Link to this property</a>

"allowed\_recipient"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%203">Link to this property</a>

"domain\_similarity"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%204">Link to this property</a>

"domain\_recency"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%205">Link to this property</a>

"managed\_acceptable\_sender"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%206">Link to this property</a>

"outbound\_ndr"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20properties">Link to this property</a>

Deprecatedts: string

Use <code>scanned_at</code> instead.

Deprecated, use <code>scanned_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20ts">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_mode: optional "DIRECT"or "BCC"or "JOURNAL"or 8 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%202">Link to this property</a>

"REVIEW\_SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%203">Link to this property</a>

"DMARC\_UNVERIFIED"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%204">Link to this property</a>

"DMARC\_FAILURE\_REPORT"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%205">Link to this property</a>

"DMARC\_AGGREGATE\_REPORT"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%206">Link to this property</a>

"THREAT\_INTEL\_SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%207">Link to this property</a>

"SIMULATION\_SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%208">Link to this property</a>

"API"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%209">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode%20%3E%20(member)%2010">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_mode">Link to this property</a>

<details>

<summary>

delivery\_status: optional array of "delivered"or "moved"or "quarantined"or 5 more

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20delivery_status">Link to this property</a>

edf\_hash: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20edf_hash">Link to this property</a>

envelope\_from: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20envelope_from">Link to this property</a>

envelope\_to: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20envelope_to">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

Deprecatedfindings: optional array of object {attachment, detail, detection, 6 more }

Use the <code>findings</code> field from GET /investigate/{investigate\_id}/detections instead.

Deprecated, use the <code>findings</code> field from <code>GET /investigate/{investigate_id}/detections</code> instead. End of life: November 1, 2026. Detection findings for this message.

</summary>

attachment: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20attachment">Link to this property</a>

detail: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detail">Link to this property</a>

<details>

<summary>

detection: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection">Link to this property</a>

field: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20field">Link to this property</a>

name: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

portion: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20portion">Link to this property</a>

reason: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20reason">Link to this property</a>

score: optional number

formatdouble

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20score">Link to this property</a>

value: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20findings">Link to this property</a>

from: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20from">Link to this property</a>

from\_name: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20from_name">Link to this property</a>

htmltext\_structure\_hash: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20htmltext_structure_hash">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20message_id">Link to this property</a>

<details>

<summary>

post\_delivery\_operations: optional array of "PREVIEW"or "QUARANTINE\_RELEASE"or "SUBMISSION"or "MOVE"

Post-delivery operations performed on this message.

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUBMISSION"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"MOVE"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20post_delivery_operations">Link to this property</a>

postfix\_id\_outbound: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20postfix_id_outbound">Link to this property</a>

replyto: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20replyto">Link to this property</a>

scanned\_at: optional string

When the message was scanned (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20scanned_at">Link to this property</a>

sent\_at: optional string

When the message was sent (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20sent_at">Link to this property</a>

sent\_date: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20sent_date">Link to this property</a>

smtp\_helo\_server\_ip: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20smtp_helo_server_ip">Link to this property</a>

smtp\_previous\_hop\_ip: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20smtp_previous_hop_ip">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20subject">Link to this property</a>

threat\_categories: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20threat_categories">Link to this property</a>

to: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20to">Link to this property</a>

to\_name: optional array of string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20to_name">Link to this property</a>

<details>

<summary>

validation: optional object {comment, dkim, dmarc, spf }

</summary>

comment: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

dkim: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim">Link to this property</a>

<details>

<summary>

dmarc: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc">Link to this property</a>

<details>

<summary>

spf: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20validation">Link to this property</a>

x\_originating\_ip: optional string

<a href="#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)%20%3E%20(property)%20x_originating_ip">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate%20%3E%20(model)%20investigate_get_response%20%3E%20(schema)>)

#### Email SecurityInvestigateDetections

##### [Get message detection details](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/detections/methods/get)

GET/accounts/{account\_id}/email-security/investigate/{investigate\_id}/detections

##### ModelsExpand Collapse

<details>

<summary>

DetectionGetResponse object {action, attachments, findings, 6 more }

</summary>

action: string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20action">Link to this property</a>

<details>

<summary>

attachments: array of object {size, content\_type, detection, 6 more }

</summary>

size: number

Size of the attachment in bytes.

minimum0

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20size">Link to this property</a>

content\_type: optional string

MIME type of the attachment.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20content_type">Link to this property</a>

<details>

<summary>

detection: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

Detection result for this attachment.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20detection">Link to this property</a>

encrypted: optional boolean

Whether the attachment is encrypted.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20encrypted">Link to this property</a>

filename: optional string

Name of the attached file.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20filename">Link to this property</a>

md5: optional string

MD5 hash of the attachment.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20md5">Link to this property</a>

name: optional string

Attachment name (alternative to filename).

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

sha1: optional string

SHA1 hash of the attachment.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20sha1">Link to this property</a>

sha256: optional string

SHA256 hash of the attachment.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments%20%3E%20(items)%20%3E%20(property)%20sha256">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20attachments">Link to this property</a>

<details>

<summary>

findings: array of object {attachment, detail, detection, 6 more }

</summary>

attachment: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20attachment">Link to this property</a>

detail: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detail">Link to this property</a>

<details>

<summary>

detection: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

Detection result associated with this finding.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection">Link to this property</a>

field: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20field">Link to this property</a>

name: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

portion: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20portion">Link to this property</a>

reason: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20reason">Link to this property</a>

score: optional number

formatdouble

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20score">Link to this property</a>

value: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20findings">Link to this property</a>

<details>

<summary>

headers: array of object {name, value }

</summary>

name: string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20headers%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

value: string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20headers%20%3E%20(items)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20headers">Link to this property</a>

<details>

<summary>

links: array of object {href, text }

</summary>

href: string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20links%20%3E%20(items)%20%3E%20(property)%20href">Link to this property</a>

text: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20links%20%3E%20(items)%20%3E%20(property)%20text">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20links">Link to this property</a>

<details>

<summary>

sender\_info: object {as\_name, as\_number, geo, 2 more }

</summary>

as\_name: optional string

The name of the autonomous system.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20sender_info%20%3E%20(property)%20as_name">Link to this property</a>

as\_number: optional number

The number of the autonomous system.

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20sender_info%20%3E%20(property)%20as_number">Link to this property</a>

geo: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20sender_info%20%3E%20(property)%20geo">Link to this property</a>

ip: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20sender_info%20%3E%20(property)%20ip">Link to this property</a>

pld: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20sender_info%20%3E%20(property)%20pld">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20sender_info">Link to this property</a>

<details>

<summary>

threat\_categories: array of object {id, description, name }

</summary>

id: optional number

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20threat_categories%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

description: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20threat_categories%20%3E%20(items)%20%3E%20(property)%20description">Link to this property</a>

name: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20threat_categories%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20threat_categories">Link to this property</a>

<details>

<summary>

validation: object {comment, dkim, dmarc, spf }

</summary>

comment: optional string

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

dkim: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dkim">Link to this property</a>

<details>

<summary>

dmarc: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc">Link to this property</a>

<details>

<summary>

spf: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation%20%3E%20(property)%20spf">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20validation">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)%20%3E%20(property)%20final_disposition">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.detections%20%3E%20(model)%20detection_get_response%20%3E%20(schema)>)

#### Email SecurityInvestigatePreview

##### [Get preview for a detection](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/preview/methods/get)

GET/accounts/{account\_id}/email-security/investigate/{investigate\_id}/preview

##### [Generate preview for a non-detection message](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/preview/methods/create)

POST/accounts/{account\_id}/email-security/investigate/preview

##### ModelsExpand Collapse

<details>

<summary>

PreviewGetResponse object {screenshot }

</summary>

screenshot: string

A base64 encoded PNG image of the email.

<a href="#(resource)%20email_security.investigate.preview%20%3E%20(model)%20preview_get_response%20%3E%20(schema)%20%3E%20(property)%20screenshot">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.preview%20%3E%20(model)%20preview_get_response%20%3E%20(schema)>)

<details>

<summary>

PreviewCreateResponse object {screenshot }

</summary>

screenshot: string

A base64 encoded PNG image of the email.

<a href="#(resource)%20email_security.investigate.preview%20%3E%20(model)%20preview_create_response%20%3E%20(schema)%20%3E%20(property)%20screenshot">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.preview%20%3E%20(model)%20preview_create_response%20%3E%20(schema)>)

#### Email SecurityInvestigateRaw

##### [Get raw email content](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/raw/methods/get)

GET/accounts/{account\_id}/email-security/investigate/{investigate\_id}/raw

##### ModelsExpand Collapse

<details>

<summary>

RawGetResponse object {raw }

</summary>

raw: string

A UTF-8 encoded eml file of the email.

<a href="#(resource)%20email_security.investigate.raw%20%3E%20(model)%20raw_get_response%20%3E%20(schema)%20%3E%20(property)%20raw">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.raw%20%3E%20(model)%20raw_get_response%20%3E%20(schema)>)

#### Email SecurityInvestigateTrace

##### [Get email trace](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/trace/methods/get)

GET/accounts/{account\_id}/email-security/investigate/{investigate\_id}/trace

##### ModelsExpand Collapse

<details>

<summary>

TraceGetResponse object {inbound, outbound }

</summary>

<details>

<summary>

inbound: object {lines, pending }

</summary>

<details>

<summary>

lines: optional array of object {lineno, logged\_at, message, ts }

</summary>

lineno: optional number

Line number in the trace log.

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20lineno">Link to this property</a>

logged\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20logged_at">Link to this property</a>

message: optional string

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20message">Link to this property</a>

Deprecatedts: optional string

Use <code>logged_at</code> instead.

Deprecated, use <code>logged_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20ts">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound%20%3E%20(property)%20lines">Link to this property</a>

pending: optional boolean

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound%20%3E%20(property)%20pending">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20inbound">Link to this property</a>

<details>

<summary>

outbound: object {lines, pending }

</summary>

<details>

<summary>

lines: optional array of object {lineno, logged\_at, message, ts }

</summary>

lineno: optional number

Line number in the trace log.

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20lineno">Link to this property</a>

logged\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20logged_at">Link to this property</a>

message: optional string

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20message">Link to this property</a>

Deprecatedts: optional string

Use <code>logged_at</code> instead.

Deprecated, use <code>logged_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound%20%3E%20(property)%20lines%20%3E%20(items)%20%3E%20(property)%20ts">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound%20%3E%20(property)%20lines">Link to this property</a>

pending: optional boolean

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound%20%3E%20(property)%20pending">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)%20%3E%20(property)%20outbound">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.trace%20%3E%20(model)%20trace_get_response%20%3E%20(schema)>)

#### Email SecurityInvestigateMove

##### [Move a message](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/move/methods/create)

POST/accounts/{account\_id}/email-security/investigate/{investigate\_id}/move

##### [Move messages](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/move/methods/bulk)

POST/accounts/{account\_id}/email-security/investigate/move

##### ModelsExpand Collapse

<details>

<summary>

MoveCreateResponse object {success, completed\_at, completed\_timestamp, 6 more }

</summary>

success: boolean

Whether the operation succeeded.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20success">Link to this property</a>

completed\_at: optional string

When the move operation completed (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20completed_at">Link to this property</a>

Deprecatedcompleted\_timestamp: optional string

Use <code>completed_at</code> instead.

Deprecated, use <code>completed_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20completed_timestamp">Link to this property</a>

destination: optional string

Destination folder for the message.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20destination">Link to this property</a>

Deprecateditem\_count: optional number

This field is deprecated.

Number of items moved. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20item_count">Link to this property</a>

message\_id: optional string

Message identifier.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20message_id">Link to this property</a>

operation: optional string

Type of operation performed.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20operation">Link to this property</a>

recipient: optional string

Recipient email address.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20recipient">Link to this property</a>

status: optional string

Operation status.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_create_response%20%3E%20(schema)>)

<details>

<summary>

MoveBulkResponse object {success, completed\_at, completed\_timestamp, 6 more }

</summary>

success: boolean

Whether the operation succeeded.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20success">Link to this property</a>

completed\_at: optional string

When the move operation completed (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20completed_at">Link to this property</a>

Deprecatedcompleted\_timestamp: optional string

Use <code>completed_at</code> instead.

Deprecated, use <code>completed_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20completed_timestamp">Link to this property</a>

destination: optional string

Destination folder for the message.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20destination">Link to this property</a>

Deprecateditem\_count: optional number

This field is deprecated.

Number of items moved. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20item_count">Link to this property</a>

message\_id: optional string

Message identifier.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20message_id">Link to this property</a>

operation: optional string

Type of operation performed.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20operation">Link to this property</a>

recipient: optional string

Recipient email address.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20recipient">Link to this property</a>

status: optional string

Operation status.

<a href="#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.move%20%3E%20(model)%20move_bulk_response%20%3E%20(schema)>)

#### Email SecurityInvestigateReclassify

##### [Change email classification](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/reclassify/methods/create)

Deprecated

POST/accounts/{account\_id}/email-security/investigate/{investigate\_id}/reclassify

##### ModelsExpand Collapse

ReclassifyCreateResponse = unknown

[Link to this property](<#(resource)%20email_security.investigate.reclassify%20%3E%20(model)%20reclassify_create_response%20%3E%20(schema)>)

#### Email SecurityInvestigateRelease

##### [Release messages from quarantine](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/release/methods/bulk)

POST/accounts/{account\_id}/email-security/investigate/release

##### ModelsExpand Collapse

<details>

<summary>

ReleaseBulkResponse object {id, delivered, failed, 2 more }

</summary>

id: string

Unique identifier for a message retrieved from investigation.

<a href="#(resource)%20email_security.investigate.release%20%3E%20(model)%20release_bulk_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

delivered: optional array of string

<a href="#(resource)%20email_security.investigate.release%20%3E%20(model)%20release_bulk_response%20%3E%20(schema)%20%3E%20(property)%20delivered">Link to this property</a>

failed: optional array of string

<a href="#(resource)%20email_security.investigate.release%20%3E%20(model)%20release_bulk_response%20%3E%20(schema)%20%3E%20(property)%20failed">Link to this property</a>

Deprecatedpostfix\_id: optional string

Use <code>id</code> instead.

Deprecated, use <code>id</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.release%20%3E%20(model)%20release_bulk_response%20%3E%20(schema)%20%3E%20(property)%20postfix_id">Link to this property</a>

undelivered: optional array of string

<a href="#(resource)%20email_security.investigate.release%20%3E%20(model)%20release_bulk_response%20%3E%20(schema)%20%3E%20(property)%20undelivered">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.release%20%3E%20(model)%20release_bulk_response%20%3E%20(schema)>)

#### Email SecurityInvestigateBulk

##### [List bulk action jobs](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/bulk/methods/list)

GET/accounts/{account\_id}/email-security/investigate/bulk

##### [Create a bulk action job](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/bulk/methods/create)

POST/accounts/{account\_id}/email-security/investigate/bulk

##### [Get bulk action job details](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/bulk/methods/get)

GET/accounts/{account\_id}/email-security/investigate/bulk/{job\_id}

##### [Delete a bulk action job](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/bulk/methods/delete)

DELETE/accounts/{account\_id}/email-security/investigate/bulk/{job\_id}

##### ModelsExpand Collapse

<details>

<summary>

BulkListResponse object {action\_params, action\_type, created\_at, 13 more }

</summary>

<details>

<summary>

action\_params: object {destination, type, expected\_disposition } or object {type }

</summary>

One of the following:

<details>

<summary>

Move object {destination, type, expected\_disposition }

</summary>

<details>

<summary>

destination: "Inbox"or "JunkEmail"or "DeletedItems"or 2 more

</summary>

One of the following:

"Inbox"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%200">Link to this property</a>

"JunkEmail"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%201">Link to this property</a>

"DeletedItems"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%202">Link to this property</a>

"RecoverableItemsDeletions"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%203">Link to this property</a>

"RecoverableItemsPurges"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination">Link to this property</a>

type: "MOVE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20type">Link to this property</a>

<details>

<summary>

Deprecatedexpected\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

This field is nonfunctional.

Nonfunctional field. End of life: December 1, 2026.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

Release object {type }

</summary>

type: "RELEASE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params">Link to this property</a>

<details>

<summary>

action\_type: "MOVE"or "RELEASE"

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20action_type">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

job\_id: string

formatuuid

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20job_id">Link to this property</a>

messages\_cancelled: number

Messages that were cancelled: rows cancelled via the API before being claimed, and rows whose in-flight attempt ended when the job reached a terminal state. Together the counters satisfy total\_messages\_discovered = messages\_pending + messages\_successful + messages\_failed + messages\_skipped + messages\_cancelled.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20messages_cancelled">Link to this property</a>

messages\_failed: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20messages_failed">Link to this property</a>

messages\_pending: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20messages_pending">Link to this property</a>

messages\_skipped: number

Messages that discovery skipped (for example, phish submissions, which the job cannot action).

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20messages_skipped">Link to this property</a>

messages\_successful: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20messages_successful">Link to this property</a>

<details>

<summary>

search\_params: object {action\_log, alert\_id, delivery\_status, 15 more }

</summary>

Deprecatedaction\_log: optional boolean

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20action_log">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_status: optional "delivered"or "moved"or "quarantined"or 5 more

Delivery status of the message.

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status">Link to this property</a>

detections\_only: optional boolean

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20detections_only">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20domain">Link to this property</a>

end: optional string

End of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20end">Link to this property</a>

exact\_subject: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20exact_subject">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

message\_action: optional "PREVIEW"or "QUARANTINE\_RELEASED"or "MOVED"

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%201">Link to this property</a>

"MOVED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_id">Link to this property</a>

metric: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20metric">Link to this property</a>

query: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20query">Link to this property</a>

recipient: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20recipient">Link to this property</a>

sender: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20sender">Link to this property</a>

smtp\_helo\_ip: optional string

Matches messages whose SMTP HELO server IP address equals this value.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20smtp_helo_ip">Link to this property</a>

start: optional string

Beginning of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20start">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20subject">Link to this property</a>

submissions: optional boolean

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20submissions">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20search_params">Link to this property</a>

<details>

<summary>

status: "PENDING"or "DISCOVERING"or "PROCESSING"or 3 more

Status of a bulk action job.

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"DISCOVERING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"PROCESSING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"COMPLETED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%204">Link to this property</a>

"CANCELLED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

total\_messages\_discovered: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20total_messages_discovered">Link to this property</a>

comment: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

completed\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20completed_at">Link to this property</a>

started\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20started_at">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)%20%3E%20(property)%20status_message">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_list_response%20%3E%20(schema)>)

<details>

<summary>

BulkCreateResponse object {action\_params, action\_type, created\_at, 13 more }

</summary>

<details>

<summary>

action\_params: object {destination, type, expected\_disposition } or object {type }

</summary>

One of the following:

<details>

<summary>

Move object {destination, type, expected\_disposition }

</summary>

<details>

<summary>

destination: "Inbox"or "JunkEmail"or "DeletedItems"or 2 more

</summary>

One of the following:

"Inbox"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%200">Link to this property</a>

"JunkEmail"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%201">Link to this property</a>

"DeletedItems"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%202">Link to this property</a>

"RecoverableItemsDeletions"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%203">Link to this property</a>

"RecoverableItemsPurges"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination">Link to this property</a>

type: "MOVE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20type">Link to this property</a>

<details>

<summary>

Deprecatedexpected\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

This field is nonfunctional.

Nonfunctional field. End of life: December 1, 2026.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

Release object {type }

</summary>

type: "RELEASE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params">Link to this property</a>

<details>

<summary>

action\_type: "MOVE"or "RELEASE"

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20action_type">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

job\_id: string

formatuuid

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20job_id">Link to this property</a>

messages\_cancelled: number

Messages that were cancelled: rows cancelled via the API before being claimed, and rows whose in-flight attempt ended when the job reached a terminal state. Together the counters satisfy total\_messages\_discovered = messages\_pending + messages\_successful + messages\_failed + messages\_skipped + messages\_cancelled.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_cancelled">Link to this property</a>

messages\_failed: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_failed">Link to this property</a>

messages\_pending: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_pending">Link to this property</a>

messages\_skipped: number

Messages that discovery skipped (for example, phish submissions, which the job cannot action).

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_skipped">Link to this property</a>

messages\_successful: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_successful">Link to this property</a>

<details>

<summary>

search\_params: object {action\_log, alert\_id, delivery\_status, 15 more }

</summary>

Deprecatedaction\_log: optional boolean

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20action_log">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_status: optional "delivered"or "moved"or "quarantined"or 5 more

Delivery status of the message.

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status">Link to this property</a>

detections\_only: optional boolean

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20detections_only">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20domain">Link to this property</a>

end: optional string

End of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20end">Link to this property</a>

exact\_subject: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20exact_subject">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

message\_action: optional "PREVIEW"or "QUARANTINE\_RELEASED"or "MOVED"

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%201">Link to this property</a>

"MOVED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_id">Link to this property</a>

metric: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20metric">Link to this property</a>

query: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20query">Link to this property</a>

recipient: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20recipient">Link to this property</a>

sender: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20sender">Link to this property</a>

smtp\_helo\_ip: optional string

Matches messages whose SMTP HELO server IP address equals this value.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20smtp_helo_ip">Link to this property</a>

start: optional string

Beginning of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20start">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20subject">Link to this property</a>

submissions: optional boolean

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20submissions">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params">Link to this property</a>

<details>

<summary>

status: "PENDING"or "DISCOVERING"or "PROCESSING"or 3 more

Status of a bulk action job.

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"DISCOVERING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"PROCESSING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"COMPLETED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%204">Link to this property</a>

"CANCELLED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

total\_messages\_discovered: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20total_messages_discovered">Link to this property</a>

comment: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

completed\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20completed_at">Link to this property</a>

started\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20started_at">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)%20%3E%20(property)%20status_message">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_create_response%20%3E%20(schema)>)

<details>

<summary>

BulkGetResponse object {action\_params, action\_type, created\_at, 13 more }

</summary>

<details>

<summary>

action\_params: object {destination, type, expected\_disposition } or object {type }

</summary>

One of the following:

<details>

<summary>

Move object {destination, type, expected\_disposition }

</summary>

<details>

<summary>

destination: "Inbox"or "JunkEmail"or "DeletedItems"or 2 more

</summary>

One of the following:

"Inbox"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%200">Link to this property</a>

"JunkEmail"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%201">Link to this property</a>

"DeletedItems"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%202">Link to this property</a>

"RecoverableItemsDeletions"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%203">Link to this property</a>

"RecoverableItemsPurges"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination">Link to this property</a>

type: "MOVE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20type">Link to this property</a>

<details>

<summary>

Deprecatedexpected\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

This field is nonfunctional.

Nonfunctional field. End of life: December 1, 2026.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

Release object {type }

</summary>

type: "RELEASE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_params">Link to this property</a>

<details>

<summary>

action\_type: "MOVE"or "RELEASE"

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20action_type">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

job\_id: string

formatuuid

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20job_id">Link to this property</a>

messages\_cancelled: number

Messages that were cancelled: rows cancelled via the API before being claimed, and rows whose in-flight attempt ended when the job reached a terminal state. Together the counters satisfy total\_messages\_discovered = messages\_pending + messages\_successful + messages\_failed + messages\_skipped + messages\_cancelled.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20messages_cancelled">Link to this property</a>

messages\_failed: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20messages_failed">Link to this property</a>

messages\_pending: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20messages_pending">Link to this property</a>

messages\_skipped: number

Messages that discovery skipped (for example, phish submissions, which the job cannot action).

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20messages_skipped">Link to this property</a>

messages\_successful: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20messages_successful">Link to this property</a>

<details>

<summary>

search\_params: object {action\_log, alert\_id, delivery\_status, 15 more }

</summary>

Deprecatedaction\_log: optional boolean

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20action_log">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_status: optional "delivered"or "moved"or "quarantined"or 5 more

Delivery status of the message.

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status">Link to this property</a>

detections\_only: optional boolean

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20detections_only">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20domain">Link to this property</a>

end: optional string

End of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20end">Link to this property</a>

exact\_subject: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20exact_subject">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

message\_action: optional "PREVIEW"or "QUARANTINE\_RELEASED"or "MOVED"

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%201">Link to this property</a>

"MOVED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_id">Link to this property</a>

metric: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20metric">Link to this property</a>

query: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20query">Link to this property</a>

recipient: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20recipient">Link to this property</a>

sender: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20sender">Link to this property</a>

smtp\_helo\_ip: optional string

Matches messages whose SMTP HELO server IP address equals this value.

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20smtp_helo_ip">Link to this property</a>

start: optional string

Beginning of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20start">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20subject">Link to this property</a>

submissions: optional boolean

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20submissions">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20search_params">Link to this property</a>

<details>

<summary>

status: "PENDING"or "DISCOVERING"or "PROCESSING"or 3 more

Status of a bulk action job.

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"DISCOVERING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"PROCESSING"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"COMPLETED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%204">Link to this property</a>

"CANCELLED"

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

total\_messages\_discovered: number

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20total_messages_discovered">Link to this property</a>

comment: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

completed\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20completed_at">Link to this property</a>

started\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20started_at">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)%20%3E%20(property)%20status_message">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_get_response%20%3E%20(schema)>)

<details>

<summary>

BulkDeleteResponse object {id }

</summary>

id: string

formatuuid

<a href="#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.bulk%20%3E%20(model)%20bulk_delete_response%20%3E%20(schema)>)

#### Email SecurityInvestigateBulkCancel

##### [Cancel a bulk action job](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/bulk/subresources/cancel/methods/create)

POST/accounts/{account\_id}/email-security/investigate/bulk/{job\_id}/cancel

##### ModelsExpand Collapse

<details>

<summary>

CancelCreateResponse object {action\_params, action\_type, created\_at, 13 more }

</summary>

<details>

<summary>

action\_params: object {destination, type, expected\_disposition } or object {type }

</summary>

One of the following:

<details>

<summary>

Move object {destination, type, expected\_disposition }

</summary>

<details>

<summary>

destination: "Inbox"or "JunkEmail"or "DeletedItems"or 2 more

</summary>

One of the following:

"Inbox"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%200">Link to this property</a>

"JunkEmail"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%201">Link to this property</a>

"DeletedItems"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%202">Link to this property</a>

"RecoverableItemsDeletions"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%203">Link to this property</a>

"RecoverableItemsPurges"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination">Link to this property</a>

type: "MOVE"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20type">Link to this property</a>

<details>

<summary>

Deprecatedexpected\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

This field is nonfunctional.

Nonfunctional field. End of life: December 1, 2026.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

Release object {type }

</summary>

type: "RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_params">Link to this property</a>

<details>

<summary>

action\_type: "MOVE"or "RELEASE"

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20action_type">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

job\_id: string

formatuuid

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20job_id">Link to this property</a>

messages\_cancelled: number

Messages that were cancelled: rows cancelled via the API before being claimed, and rows whose in-flight attempt ended when the job reached a terminal state. Together the counters satisfy total\_messages\_discovered = messages\_pending + messages\_successful + messages\_failed + messages\_skipped + messages\_cancelled.

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_cancelled">Link to this property</a>

messages\_failed: number

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_failed">Link to this property</a>

messages\_pending: number

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_pending">Link to this property</a>

messages\_skipped: number

Messages that discovery skipped (for example, phish submissions, which the job cannot action).

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_skipped">Link to this property</a>

messages\_successful: number

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20messages_successful">Link to this property</a>

<details>

<summary>

search\_params: object {action\_log, alert\_id, delivery\_status, 15 more }

</summary>

Deprecatedaction\_log: optional boolean

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20action_log">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_status: optional "delivered"or "moved"or "quarantined"or 5 more

Delivery status of the message.

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20delivery_status">Link to this property</a>

detections\_only: optional boolean

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20detections_only">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20domain">Link to this property</a>

end: optional string

End of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20end">Link to this property</a>

exact\_subject: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20exact_subject">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

message\_action: optional "PREVIEW"or "QUARANTINE\_RELEASED"or "MOVED"

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%201">Link to this property</a>

"MOVED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_action">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20message_id">Link to this property</a>

metric: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20metric">Link to this property</a>

query: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20query">Link to this property</a>

recipient: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20recipient">Link to this property</a>

sender: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20sender">Link to this property</a>

smtp\_helo\_ip: optional string

Matches messages whose SMTP HELO server IP address equals this value.

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20smtp_helo_ip">Link to this property</a>

start: optional string

Beginning of search date range.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20start">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20subject">Link to this property</a>

submissions: optional boolean

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params%20%3E%20(property)%20submissions">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20search_params">Link to this property</a>

<details>

<summary>

status: "PENDING"or "DISCOVERING"or "PROCESSING"or 3 more

Status of a bulk action job.

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"DISCOVERING"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"PROCESSING"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"COMPLETED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%204">Link to this property</a>

"CANCELLED"

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

total\_messages\_discovered: number

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20total_messages_discovered">Link to this property</a>

comment: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

completed\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20completed_at">Link to this property</a>

started\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20started_at">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)%20%3E%20(property)%20status_message">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.bulk.cancel%20%3E%20(model)%20cancel_create_response%20%3E%20(schema)>)

#### Email SecurityInvestigateBulkMessages

##### [List messages for a bulk action job](https://developers.cloudflare.com/api/resources/email_security/subresources/investigate/subresources/bulk/subresources/messages/methods/list)

GET/accounts/{account\_id}/email-security/investigate/bulk/{job\_id}/messages

##### ModelsExpand Collapse

<details>

<summary>

MessageListResponse object {action\_params, action\_type, created\_at, 10 more }

</summary>

<details>

<summary>

action\_params: object {client\_recipient, destination, type, expected\_disposition } or object {client\_recipient, type }

</summary>

One of the following:

<details>

<summary>

Move object {client\_recipient, destination, type, expected\_disposition }

</summary>

client\_recipient: string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20client_recipient">Link to this property</a>

<details>

<summary>

destination: "Inbox"or "JunkEmail"or "DeletedItems"or 2 more

</summary>

One of the following:

"Inbox"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%200">Link to this property</a>

"JunkEmail"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%201">Link to this property</a>

"DeletedItems"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%202">Link to this property</a>

"RecoverableItemsDeletions"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%203">Link to this property</a>

"RecoverableItemsPurges"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20destination">Link to this property</a>

type: "MOVE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20type">Link to this property</a>

<details>

<summary>

Deprecatedexpected\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

This field is nonfunctional.

Nonfunctional field. End of life: December 1, 2026.

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200%20%3E%20(property)%20expected_disposition">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%200">Link to this property</a>

<details>

<summary>

Release object {client\_recipient, type }

</summary>

client\_recipient: string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201%20%3E%20(property)%20client_recipient">Link to this property</a>

type: "RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_params">Link to this property</a>

<details>

<summary>

action\_type: "MOVE"or "RELEASE"

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20action_type">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

message\_id: string

formatuuid

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message_id">Link to this property</a>

postfix\_id: string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20postfix_id">Link to this property</a>

retry\_count: number

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_count">Link to this property</a>

<details>

<summary>

status: "PENDING"or "PROCESSING"or "COMPLETED"or 3 more

Status of a message within a bulk action job.

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"PROCESSING"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"COMPLETED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

"CANCELLED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%204">Link to this property</a>

"SKIPPED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20alert_id">Link to this property</a>

email\_message\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20email_message_id">Link to this property</a>

<details>

<summary>

message: optional object {id, action\_log, client\_recipients, 32 more }

</summary>

id: string

Unique identifier for a message retrieved from investigation.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

Deprecatedaction\_log: array of object {completed\_at, operation, completed\_timestamp, 2 more }

Use GET /investigate/{investigate\_id}/action\_log instead.

Deprecated, use <code>GET /investigate/{investigate_id}/action_log</code> instead. End of life: November 1, 2026.

</summary>

completed\_at: string

Timestamp when action completed.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20completed_at">Link to this property</a>

<details>

<summary>

operation: "MOVE"or "RELEASE"or "RECLASSIFY"or 3 more

Type of action performed.

</summary>

One of the following:

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%200">Link to this property</a>

"RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%201">Link to this property</a>

"RECLASSIFY"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%202">Link to this property</a>

"SUBMISSION"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%203">Link to this property</a>

"QUARANTINE\_RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%204">Link to this property</a>

"PREVIEW"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20operation">Link to this property</a>

Deprecatedcompleted\_timestamp: optional string

Use <code>completed_at</code> instead.

Deprecated, use <code>completed_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20completed_timestamp">Link to this property</a>

<details>

<summary>

properties: optional object {folder, requested\_by }

Additional properties for the action.

</summary>

folder: optional string

Target folder for move operations.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties%20%3E%20(property)%20folder">Link to this property</a>

requested\_by: optional string

User who requested the action.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties%20%3E%20(property)%20requested_by">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20properties">Link to this property</a>

status: optional string

Status of the action.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log%20%3E%20(items)%20%3E%20(property)%20status">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20action_log">Link to this property</a>

client\_recipients: array of string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20client_recipients">Link to this property</a>

detection\_reasons: array of string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20detection_reasons">Link to this property</a>

is\_phish\_submission: boolean

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20is_phish_submission">Link to this property</a>

is\_quarantined: boolean

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20is_quarantined">Link to this property</a>

postfix\_id: string

The identifier of the message.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20postfix_id">Link to this property</a>

<details>

<summary>

properties: object {allowlisted\_pattern, allowlisted\_pattern\_type, blocklisted\_message, 2 more }

Message processing properties.

</summary>

allowlisted\_pattern: optional string

Pattern that allowlisted this message.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern">Link to this property</a>

<details>

<summary>

allowlisted\_pattern\_type: optional "quarantine\_release"or "acceptable\_sender"or "allowed\_sender"or 5 more

Type of allowlist pattern.

</summary>

One of the following:

"quarantine\_release"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%200">Link to this property</a>

"acceptable\_sender"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%201">Link to this property</a>

"allowed\_sender"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%202">Link to this property</a>

"allowed\_recipient"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%203">Link to this property</a>

"domain\_similarity"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%204">Link to this property</a>

"domain\_recency"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%205">Link to this property</a>

"managed\_acceptable\_sender"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%206">Link to this property</a>

"outbound\_ndr"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20allowlisted_pattern_type">Link to this property</a>

blocklisted\_message: optional boolean

Whether message was blocklisted.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20blocklisted_message">Link to this property</a>

blocklisted\_pattern: optional string

Pattern that blocklisted this message.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20blocklisted_pattern">Link to this property</a>

<details>

<summary>

whitelisted\_pattern\_type: optional "quarantine\_release"or "acceptable\_sender"or "allowed\_sender"or 5 more

Legacy field for allowlist pattern type.

</summary>

One of the following:

"quarantine\_release"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%200">Link to this property</a>

"acceptable\_sender"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%201">Link to this property</a>

"allowed\_sender"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%202">Link to this property</a>

"allowed\_recipient"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%203">Link to this property</a>

"domain\_similarity"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%204">Link to this property</a>

"domain\_recency"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%205">Link to this property</a>

"managed\_acceptable\_sender"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%206">Link to this property</a>

"outbound\_ndr"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties%20%3E%20(property)%20whitelisted_pattern_type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20properties">Link to this property</a>

Deprecatedts: string

Use <code>scanned_at</code> instead.

Deprecated, use <code>scanned_at</code> instead. End of life: November 1, 2026.

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20ts">Link to this property</a>

alert\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20alert_id">Link to this property</a>

<details>

<summary>

delivery\_mode: optional "DIRECT"or "BCC"or "JOURNAL"or 8 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%202">Link to this property</a>

"REVIEW\_SUBMISSION"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%203">Link to this property</a>

"DMARC\_UNVERIFIED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%204">Link to this property</a>

"DMARC\_FAILURE\_REPORT"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%205">Link to this property</a>

"DMARC\_AGGREGATE\_REPORT"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%206">Link to this property</a>

"THREAT\_INTEL\_SUBMISSION"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%207">Link to this property</a>

"SIMULATION\_SUBMISSION"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%208">Link to this property</a>

"API"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%209">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode%20%3E%20(member)%2010">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_mode">Link to this property</a>

<details>

<summary>

delivery\_status: optional array of "delivered"or "moved"or "quarantined"or 5 more

</summary>

One of the following:

"delivered"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"moved"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"quarantined"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"rejected"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"deferred"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"bounced"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"queued"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"move\_failed"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20delivery_status">Link to this property</a>

edf\_hash: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20edf_hash">Link to this property</a>

envelope\_from: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20envelope_from">Link to this property</a>

envelope\_to: optional array of string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20envelope_to">Link to this property</a>

<details>

<summary>

final\_disposition: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20final_disposition">Link to this property</a>

<details>

<summary>

Deprecatedfindings: optional array of object {attachment, detail, detection, 6 more }

Use the <code>findings</code> field from GET /investigate/{investigate\_id}/detections instead.

Deprecated, use the <code>findings</code> field from <code>GET /investigate/{investigate_id}/detections</code> instead. End of life: November 1, 2026. Detection findings for this message.

</summary>

attachment: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20attachment">Link to this property</a>

detail: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detail">Link to this property</a>

<details>

<summary>

detection: optional "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20detection">Link to this property</a>

field: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20field">Link to this property</a>

name: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

portion: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20portion">Link to this property</a>

reason: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20reason">Link to this property</a>

score: optional number

formatdouble

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20score">Link to this property</a>

value: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings%20%3E%20(items)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20findings">Link to this property</a>

from: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20from">Link to this property</a>

from\_name: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20from_name">Link to this property</a>

htmltext\_structure\_hash: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20htmltext_structure_hash">Link to this property</a>

message\_id: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20message_id">Link to this property</a>

<details>

<summary>

post\_delivery\_operations: optional array of "PREVIEW"or "QUARANTINE\_RELEASE"or "SUBMISSION"or "MOVE"

Post-delivery operations performed on this message.

</summary>

One of the following:

"PREVIEW"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"QUARANTINE\_RELEASE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUBMISSION"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"MOVE"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20post_delivery_operations%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20post_delivery_operations">Link to this property</a>

postfix\_id\_outbound: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20postfix_id_outbound">Link to this property</a>

replyto: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20replyto">Link to this property</a>

scanned\_at: optional string

When the message was scanned (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20scanned_at">Link to this property</a>

sent\_at: optional string

When the message was sent (UTC).

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20sent_at">Link to this property</a>

sent\_date: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20sent_date">Link to this property</a>

smtp\_helo\_server\_ip: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20smtp_helo_server_ip">Link to this property</a>

smtp\_previous\_hop\_ip: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20smtp_previous_hop_ip">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20subject">Link to this property</a>

threat\_categories: optional array of string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20threat_categories">Link to this property</a>

to: optional array of string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20to">Link to this property</a>

to\_name: optional array of string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20to_name">Link to this property</a>

<details>

<summary>

validation: optional object {comment, dkim, dmarc, spf }

</summary>

comment: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

dkim: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dkim%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dkim">Link to this property</a>

<details>

<summary>

dmarc: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20dmarc">Link to this property</a>

<details>

<summary>

spf: optional "pass"or "neutral"or "fail"or 2 more

</summary>

One of the following:

"pass"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%200">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%201">Link to this property</a>

"fail"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%202">Link to this property</a>

"error"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%203">Link to this property</a>

"none"

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20spf%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation%20%3E%20(property)%20spf">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20validation">Link to this property</a>

x\_originating\_ip: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message%20%3E%20(property)%20x_originating_ip">Link to this property</a>

</details>

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20message">Link to this property</a>

processed\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20processed_at">Link to this property</a>

retry\_after: optional string

When to retry the action if it failed.

formatdate-time

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20retry_after">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)%20%3E%20(property)%20status_message">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.investigate.bulk.messages%20%3E%20(model)%20message_list_response%20%3E%20(schema)>)

#### Email SecurityPhishguard

#### Email SecurityPhishguardReports

##### [List PhishGuard reports](https://developers.cloudflare.com/api/resources/email_security/subresources/phishguard/subresources/reports/methods/list)

GET/accounts/{account\_id}/email-security/phishguard/reports

##### ModelsExpand Collapse

<details>

<summary>

ReportListResponse object {id, content, disposition, 7 more }

</summary>

id: number

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

content: string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20content">Link to this property</a>

<details>

<summary>

disposition: "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20disposition">Link to this property</a>

<details>

<summary>

fields: object {to, from, occurred\_at, 2 more }

</summary>

to: array of string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20fields%20%3E%20(property)%20to">Link to this property</a>

from: optional string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20fields%20%3E%20(property)%20from">Link to this property</a>

occurred\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20fields%20%3E%20(property)%20occurred_at">Link to this property</a>

postfix\_id: optional string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20fields%20%3E%20(property)%20postfix_id">Link to this property</a>

Deprecatedts: optional string

Use <code>occurred_at</code> instead.

Deprecated, use <code>occurred_at</code> instead.

formatdate-time

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20fields%20%3E%20(property)%20ts">Link to this property</a>

</details>

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20fields">Link to this property</a>

priority: string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20priority">Link to this property</a>

title: string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20title">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

tags: optional array of object {category, value }

</summary>

category: string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20tags%20%3E%20(items)%20%3E%20(property)%20category">Link to this property</a>

value: string

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20tags%20%3E%20(items)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20tags">Link to this property</a>

Deprecatedts: optional string

Use <code>created_at</code> instead.

Deprecated, use <code>created_at</code> instead.

formatdate-time

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20ts">Link to this property</a>

updated\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)%20%3E%20(property)%20updated_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.phishguard.reports%20%3E%20(model)%20report_list_response%20%3E%20(schema)>)

#### Email SecuritySettings

#### Email SecuritySettingsAllow Policies

##### [List email allow policies](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/allow_policies/methods/list)

GET/accounts/{account\_id}/email-security/settings/allow\_policies

##### [Get an email allow policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/allow_policies/methods/get)

GET/accounts/{account\_id}/email-security/settings/allow\_policies/{policy\_id}

##### [Create email allow policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/allow_policies/methods/create)

POST/accounts/{account\_id}/email-security/settings/allow\_policies

##### [Update an email allow policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/allow_policies/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/allow\_policies/{policy\_id}

##### [Delete an email allow policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/allow_policies/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/allow\_policies/{policy\_id}

##### [Batch allow policy operations](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/allow_policies/methods/batch)

POST/accounts/{account\_id}/email-security/settings/allow\_policies/batch

##### ModelsExpand Collapse

<details>

<summary>

AllowPolicyListResponse object {id, created\_at, last\_modified, 12 more }

An email allow policy.

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_list_response%20%3E%20(schema)>)

<details>

<summary>

AllowPolicyGetResponse object {id, created\_at, last\_modified, 12 more }

An email allow policy.

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_get_response%20%3E%20(schema)>)

<details>

<summary>

AllowPolicyCreateResponse object {id, created\_at, last\_modified, 12 more }

An email allow policy.

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_create_response%20%3E%20(schema)>)

<details>

<summary>

AllowPolicyEditResponse object {id, created\_at, last\_modified, 12 more }

An email allow policy.

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_edit_response%20%3E%20(schema)>)

<details>

<summary>

AllowPolicyDeleteResponse object {id }

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_delete_response%20%3E%20(schema)>)

<details>

<summary>

AllowPolicyBatchResponse object {deletes, patches, posts, puts }

</summary>

<details>

<summary>

deletes: optional array of object {id }

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes">Link to this property</a>

<details>

<summary>

patches: optional array of object {id, created\_at, last\_modified, 12 more }

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches">Link to this property</a>

<details>

<summary>

posts: optional array of object {id, created\_at, last\_modified, 12 more }

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts">Link to this property</a>

<details>

<summary>

puts: optional array of object {id, created\_at, last\_modified, 12 more }

</summary>

id: string

Allow policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

Deprecatedlast\_modified: string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

is\_acceptable\_sender: optional boolean

Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_acceptable_sender">Link to this property</a>

is\_exempt\_recipient: optional boolean

Bypasses all detections for messages to this recipient.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_exempt_recipient">Link to this property</a>

Deprecatedis\_recipient: optional boolean

Use <code>is_exempt_recipient</code> instead.

Deprecated as of July 1, 2025. Use <code>is_exempt_recipient</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_recipient">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedis\_sender: optional boolean

Use <code>is_trusted_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_trusted_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_sender">Link to this property</a>

Deprecatedis\_spoof: optional boolean

Use <code>is_acceptable_sender</code> instead.

Deprecated as of July 1, 2025. Use <code>is_acceptable_sender</code> instead. End of life: July 1, 2026.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_spoof">Link to this property</a>

is\_trusted\_sender: optional boolean

Bypasses all detections and link following for messages from this sender.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_trusted_sender">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type">Link to this property</a>

verify\_sender: optional boolean

Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication.

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20verify_sender">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.allow_policies%20%3E%20(model)%20allow_policy_batch_response%20%3E%20(schema)>)

#### Email SecuritySettingsBlock Senders

##### [List blocked email senders](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/block_senders/methods/list)

GET/accounts/{account\_id}/email-security/settings/block\_senders

##### [Get a blocked email sender](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/block_senders/methods/get)

GET/accounts/{account\_id}/email-security/settings/block\_senders/{pattern\_id}

##### [Create blocked email sender](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/block_senders/methods/create)

POST/accounts/{account\_id}/email-security/settings/block\_senders

##### [Update a blocked email sender](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/block_senders/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/block\_senders/{pattern\_id}

##### [Delete a blocked email sender](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/block_senders/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/block\_senders/{pattern\_id}

##### [Batch blocked sender operations](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/block_senders/methods/batch)

POST/accounts/{account\_id}/email-security/settings/block\_senders/batch

##### ModelsExpand Collapse

<details>

<summary>

BlockSenderListResponse object {id, comments, created\_at, 5 more }

A blocked sender pattern.

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_list_response%20%3E%20(schema)>)

<details>

<summary>

BlockSenderGetResponse object {id, comments, created\_at, 5 more }

A blocked sender pattern.

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_get_response%20%3E%20(schema)>)

<details>

<summary>

BlockSenderCreateResponse object {id, comments, created\_at, 5 more }

A blocked sender pattern.

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_create_response%20%3E%20(schema)>)

<details>

<summary>

BlockSenderEditResponse object {id, comments, created\_at, 5 more }

A blocked sender pattern.

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_edit_response%20%3E%20(schema)>)

<details>

<summary>

BlockSenderDeleteResponse object {id }

</summary>

id: string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_delete_response%20%3E%20(schema)>)

<details>

<summary>

BlockSenderBatchResponse object {deletes, patches, posts, puts }

</summary>

<details>

<summary>

deletes: optional array of object {id }

</summary>

id: string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes">Link to this property</a>

<details>

<summary>

patches: optional array of object {id, comments, created\_at, 5 more }

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches">Link to this property</a>

<details>

<summary>

posts: optional array of object {id, comments, created\_at, 5 more }

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts">Link to this property</a>

<details>

<summary>

puts: optional array of object {id, comments, created\_at, 5 more }

</summary>

id: optional string

Blocked sender pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

The pattern value to match. The format depends on <code>pattern_type</code>: a valid email address for EMAIL (e.g. <code>user@example.com</code>), a valid domain name for DOMAIN (e.g. <code>example.com</code>), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. <code>1.2.3.4</code>, <code>1.2.3.0/24</code>, <code>2606:4700:4700::1111</code>, or <code>2606:4700:4700::/48</code>); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

pattern\_type: optional "EMAIL"or "DOMAIN"or "IP"or "UNKNOWN"

Type of pattern matching.

- EMAIL: matches a full email address (e.g. <code>user@example.com</code>)
- DOMAIN: matches a domain name (e.g. <code>example.com</code>)
- IP: matches a plain IPv4 or IPv6 address (e.g. <code>1.2.3.4</code> or <code>2606:4700:4700::1111</code>) or CIDR block (e.g. <code>1.2.3.0/24</code> or <code>2606:4700:4700::/48</code>). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.
- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.

</summary>

One of the following:

"EMAIL"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%200">Link to this property</a>

"DOMAIN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%201">Link to this property</a>

"IP"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%202">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern_type">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.block_senders%20%3E%20(model)%20block_sender_batch_response%20%3E%20(schema)>)

#### Email SecuritySettingsContent Policies

##### [List content policies](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/content_policies/methods/list)

GET/accounts/{account\_id}/email-security/settings/content\_policies

##### [Get a content policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/content_policies/methods/get)

GET/accounts/{account\_id}/email-security/settings/content\_policies/{policy\_id}

##### [Create a content policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/content_policies/methods/create)

POST/accounts/{account\_id}/email-security/settings/content\_policies

##### [Update a content policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/content_policies/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/content\_policies/{policy\_id}

##### [Delete a content policy](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/content_policies/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/content\_policies/{policy\_id}

##### [Batch content policy operations](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/content_policies/methods/batch)

POST/accounts/{account\_id}/email-security/settings/content\_policies/batch

##### ModelsExpand Collapse

<details>

<summary>

ContentPolicyListResponse object {id, created\_at, enabled, 5 more }

A content policy pattern that matches against the subject or body of an email.

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)%20%3E%20(property)%20targets">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_list_response%20%3E%20(schema)>)

<details>

<summary>

ContentPolicyGetResponse object {id, created\_at, enabled, 5 more }

A content policy pattern that matches against the subject or body of an email.

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)%20%3E%20(property)%20targets">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_get_response%20%3E%20(schema)>)

<details>

<summary>

ContentPolicyCreateResponse object {id, created\_at, enabled, 5 more }

A content policy pattern that matches against the subject or body of an email.

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)%20%3E%20(property)%20targets">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_create_response%20%3E%20(schema)>)

<details>

<summary>

ContentPolicyEditResponse object {id, created\_at, enabled, 5 more }

A content policy pattern that matches against the subject or body of an email.

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)%20%3E%20(property)%20targets">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_edit_response%20%3E%20(schema)>)

<details>

<summary>

ContentPolicyDeleteResponse object {id }

</summary>

id: string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_delete_response%20%3E%20(schema)>)

<details>

<summary>

ContentPolicyBatchResponse object {deletes, patches, posts, puts }

</summary>

<details>

<summary>

deletes: optional array of object {id }

</summary>

id: string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes">Link to this property</a>

<details>

<summary>

patches: optional array of object {id, created\_at, enabled, 5 more }

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20targets">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches">Link to this property</a>

<details>

<summary>

posts: optional array of object {id, created\_at, enabled, 5 more }

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20targets">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts">Link to this property</a>

<details>

<summary>

puts: optional array of object {id, created\_at, enabled, 5 more }

</summary>

id: optional string

Content policy identifier.

formatuuid

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

enabled: optional boolean

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20enabled">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength256

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20name">Link to this property</a>

notes: optional string

maxLength4096

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20notes">Link to this property</a>

pattern: optional string

maxLength2048

minLength1

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

<details>

<summary>

targets: optional array of "SUBJECT"or "BODY"

</summary>

One of the following:

"SUBJECT"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BODY"

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20targets%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20targets">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.content_policies%20%3E%20(model)%20content_policy_batch_response%20%3E%20(schema)>)

#### Email SecuritySettingsDomains

##### [List protected email domains](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/list)

GET/accounts/{account\_id}/email-security/settings/domains

##### [Get an email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/get)

GET/accounts/{account\_id}/email-security/settings/domains/{domain\_id}

##### [Replace an email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/update)

PUT/accounts/{account\_id}/email-security/settings/domains/{domain\_id}

##### [Update an email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/domains/{domain\_id}

##### [Add a new email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/create)

POST/accounts/{account\_id}/email-security/settings/domains

##### [Unprotect an email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/domains/{domain\_id}

##### [Batch domain operations](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/batch)

POST/accounts/{account\_id}/email-security/settings/domains/batch

##### [Unprotect multiple email domains](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/domains/methods/bulk_delete)

Deprecated

DELETE/accounts/{account\_id}/email-security/settings/domains

##### ModelsExpand Collapse

<details>

<summary>

DomainListResponse object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)%20%3E%20(property)%20transport">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_list_response%20%3E%20(schema)>)

<details>

<summary>

DomainGetResponse object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)%20%3E%20(property)%20transport">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_get_response%20%3E%20(schema)>)

<details>

<summary>

DomainUpdateResponse object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)%20%3E%20(property)%20transport">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_update_response%20%3E%20(schema)>)

<details>

<summary>

DomainEditResponse object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20transport">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_edit_response%20%3E%20(schema)>)

<details>

<summary>

DomainCreateResponse object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)%20%3E%20(property)%20transport">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_create_response%20%3E%20(schema)>)

<details>

<summary>

DomainDeleteResponse object {id }

</summary>

id: string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_delete_response%20%3E%20(schema)>)

<details>

<summary>

DomainBatchResponse object {deletes, patches, posts, puts }

</summary>

<details>

<summary>

deletes: array of object {id }

</summary>

id: string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes">Link to this property</a>

<details>

<summary>

patches: array of object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20transport">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches">Link to this property</a>

<details>

<summary>

posts: array of object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20transport">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts">Link to this property</a>

<details>

<summary>

puts: array of object {id, allowed\_delivery\_modes, authorization, 19 more }

</summary>

id: optional string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

allowed\_delivery\_modes: optional array of "DIRECT"or "BCC"or "JOURNAL"or 2 more

</summary>

One of the following:

"DIRECT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"BCC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"JOURNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"API"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"RETRO\_SCAN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20allowed_delivery_modes">Link to this property</a>

<details>

<summary>

authorization: optional object {authorized, timestamp, status\_message }

</summary>

authorized: boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20authorized">Link to this property</a>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20timestamp">Link to this property</a>

status\_message: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20authorization%20%3E%20(property)%20status_message">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20authorization">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

<details>

<summary>

dmarc\_status: optional "none"or "good"or "invalid"

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%201">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20dmarc_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20dmarc_status">Link to this property</a>

domain: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20domain">Link to this property</a>

<details>

<summary>

drop\_dispositions: optional array of "MALICIOUS"or "MALICIOUS-BEC"or "SUSPICIOUS"or 7 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"MALICIOUS-BEC"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

"ENCRYPTED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%206">Link to this property</a>

"EXTERNAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%207">Link to this property</a>

"UNKNOWN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%208">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions%20%3E%20(items)%20%3E%20(member)%209">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20drop_dispositions">Link to this property</a>

<details>

<summary>

emails\_processed: optional object {timestamp, total\_emails\_processed, total\_emails\_processed\_previous }

</summary>

timestamp: string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20timestamp">Link to this property</a>

total\_emails\_processed: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed">Link to this property</a>

total\_emails\_processed\_previous: number

minimum0

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20emails_processed%20%3E%20(property)%20total_emails_processed_previous">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20emails_processed">Link to this property</a>

<details>

<summary>

folder: optional "AllItems"or "Inbox"

</summary>

One of the following:

"AllItems"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20folder%20%3E%20(member)%200">Link to this property</a>

"Inbox"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20folder%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20folder">Link to this property</a>

<details>

<summary>

inbox\_provider: optional "Microsoft"or "Google"

</summary>

One of the following:

"Microsoft"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%200">Link to this property</a>

"Google"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20inbox_provider%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20inbox_provider">Link to this property</a>

integration\_id: optional string

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20integration_id">Link to this property</a>

ip\_restrictions: optional array of string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20ip_restrictions">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

lookback\_hops: optional number

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20lookback_hops">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

o365\_tenant\_id: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20o365_tenant_id">Link to this property</a>

<details>

<summary>

regions: optional array of "GLOBAL"or "AU"or "DE"or 2 more

</summary>

One of the following:

"GLOBAL"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"AU"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"DE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"IN"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"US"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20regions%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20regions">Link to this property</a>

require\_tls\_inbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20require_tls_inbound">Link to this property</a>

require\_tls\_outbound: optional boolean

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20require_tls_outbound">Link to this property</a>

<details>

<summary>

spf\_status: optional "none"or "good"or "neutral"or 2 more

</summary>

One of the following:

"none"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%200">Link to this property</a>

"good"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%201">Link to this property</a>

"neutral"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%202">Link to this property</a>

"open"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%203">Link to this property</a>

"invalid"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20spf_status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20spf_status">Link to this property</a>

<details>

<summary>

status: optional "PENDING"or "ACTIVE"or "FAILED"or "TIMEOUT"

</summary>

One of the following:

"PENDING"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"ACTIVE"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"FAILED"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"TIMEOUT"

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20status">Link to this property</a>

transport: optional string

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20transport">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_batch_response%20%3E%20(schema)>)

<details>

<summary>

DomainBulkDeleteResponse object {id }

</summary>

id: string

Domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_bulk_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.domains%20%3E%20(model)%20domain_bulk_delete_response%20%3E%20(schema)>)

#### Email SecuritySettingsImpersonation Registry

##### [List impersonation registry entries](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/impersonation_registry/methods/list)

GET/accounts/{account\_id}/email-security/settings/impersonation\_registry

##### [Get an impersonation registry entry](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/impersonation_registry/methods/get)

GET/accounts/{account\_id}/email-security/settings/impersonation\_registry/{impersonation\_registry\_id}

##### [Create impersonation registry entry](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/impersonation_registry/methods/create)

POST/accounts/{account\_id}/email-security/settings/impersonation\_registry

##### [Update an impersonation registry entry](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/impersonation_registry/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/impersonation\_registry/{impersonation\_registry\_id}

##### [Delete an impersonation registry entry](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/impersonation_registry/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/impersonation\_registry/{impersonation\_registry\_id}

##### ModelsExpand Collapse

<details>

<summary>

ImpersonationRegistryListResponse object {id, comments, created\_at, 9 more }

An impersonation registry entry.

</summary>

id: optional string

Impersonation registry entry identifier.

formatuuid

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

directory\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20directory_id">Link to this property</a>

directory\_node\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20directory_node_id">Link to this property</a>

email: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20email">Link to this property</a>

Deprecatedexternal\_directory\_node\_id: optional string

This field is deprecated.

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20external_directory_node_id">Link to this property</a>

is\_email\_regex: optional boolean

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20is_email_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

provenance: optional "A1S\_INTERNAL"or "SNOOPY-CASB\_OFFICE\_365"or "SNOOPY-OFFICE\_365"or "SNOOPY-GOOGLE\_DIRECTORY"

</summary>

One of the following:

"A1S\_INTERNAL"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%200">Link to this property</a>

"SNOOPY-CASB\_OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%201">Link to this property</a>

"SNOOPY-OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%202">Link to this property</a>

"SNOOPY-GOOGLE\_DIRECTORY"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)%20%3E%20(property)%20provenance">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_list_response%20%3E%20(schema)>)

<details>

<summary>

ImpersonationRegistryGetResponse object {id, comments, created\_at, 9 more }

An impersonation registry entry.

</summary>

id: optional string

Impersonation registry entry identifier.

formatuuid

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

directory\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20directory_id">Link to this property</a>

directory\_node\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20directory_node_id">Link to this property</a>

email: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20email">Link to this property</a>

Deprecatedexternal\_directory\_node\_id: optional string

This field is deprecated.

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20external_directory_node_id">Link to this property</a>

is\_email\_regex: optional boolean

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20is_email_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

provenance: optional "A1S\_INTERNAL"or "SNOOPY-CASB\_OFFICE\_365"or "SNOOPY-OFFICE\_365"or "SNOOPY-GOOGLE\_DIRECTORY"

</summary>

One of the following:

"A1S\_INTERNAL"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%200">Link to this property</a>

"SNOOPY-CASB\_OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%201">Link to this property</a>

"SNOOPY-OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%202">Link to this property</a>

"SNOOPY-GOOGLE\_DIRECTORY"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)%20%3E%20(property)%20provenance">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_get_response%20%3E%20(schema)>)

<details>

<summary>

ImpersonationRegistryCreateResponse object {id, comments, created\_at, 9 more }

An impersonation registry entry.

</summary>

id: optional string

Impersonation registry entry identifier.

formatuuid

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

directory\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20directory_id">Link to this property</a>

directory\_node\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20directory_node_id">Link to this property</a>

email: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20email">Link to this property</a>

Deprecatedexternal\_directory\_node\_id: optional string

This field is deprecated.

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20external_directory_node_id">Link to this property</a>

is\_email\_regex: optional boolean

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20is_email_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

provenance: optional "A1S\_INTERNAL"or "SNOOPY-CASB\_OFFICE\_365"or "SNOOPY-OFFICE\_365"or "SNOOPY-GOOGLE\_DIRECTORY"

</summary>

One of the following:

"A1S\_INTERNAL"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%200">Link to this property</a>

"SNOOPY-CASB\_OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%201">Link to this property</a>

"SNOOPY-OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%202">Link to this property</a>

"SNOOPY-GOOGLE\_DIRECTORY"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)%20%3E%20(property)%20provenance">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_create_response%20%3E%20(schema)>)

<details>

<summary>

ImpersonationRegistryEditResponse object {id, comments, created\_at, 9 more }

An impersonation registry entry.

</summary>

id: optional string

Impersonation registry entry identifier.

formatuuid

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

directory\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20directory_id">Link to this property</a>

directory\_node\_id: optional number

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20directory_node_id">Link to this property</a>

email: optional string

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20email">Link to this property</a>

Deprecatedexternal\_directory\_node\_id: optional string

This field is deprecated.

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20external_directory_node_id">Link to this property</a>

is\_email\_regex: optional boolean

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_email_regex">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

name: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

provenance: optional "A1S\_INTERNAL"or "SNOOPY-CASB\_OFFICE\_365"or "SNOOPY-OFFICE\_365"or "SNOOPY-GOOGLE\_DIRECTORY"

</summary>

One of the following:

"A1S\_INTERNAL"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%200">Link to this property</a>

"SNOOPY-CASB\_OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%201">Link to this property</a>

"SNOOPY-OFFICE\_365"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%202">Link to this property</a>

"SNOOPY-GOOGLE\_DIRECTORY"

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20provenance%20%3E%20(member)%203">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)%20%3E%20(property)%20provenance">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_edit_response%20%3E%20(schema)>)

<details>

<summary>

ImpersonationRegistryDeleteResponse object {id }

</summary>

id: string

Impersonation registry entry identifier.

formatuuid

<a href="#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.impersonation_registry%20%3E%20(model)%20impersonation_registry_delete_response%20%3E%20(schema)>)

#### Email SecuritySettingsSending Domain Restrictions

##### [List sending domain restrictions](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/sending_domain_restrictions/methods/list)

GET/accounts/{account\_id}/email-security/settings/sending\_domain\_restrictions

##### [Get a sending domain restriction](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/sending_domain_restrictions/methods/get)

GET/accounts/{account\_id}/email-security/settings/sending\_domain\_restrictions/{sending\_domain\_restriction\_id}

##### [Create a sending domain restriction](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/sending_domain_restrictions/methods/create)

POST/accounts/{account\_id}/email-security/settings/sending\_domain\_restrictions

##### [Update a sending domain restriction](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/sending_domain_restrictions/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/sending\_domain\_restrictions/{sending\_domain\_restriction\_id}

##### [Delete a sending domain restriction](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/sending_domain_restrictions/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/sending\_domain\_restrictions/{sending\_domain\_restriction\_id}

##### ModelsExpand Collapse

<details>

<summary>

SendingDomainRestrictionListResponse object {id, comments, created\_at, 4 more }

A sending domain restriction that enforces TLS (Transport Layer Security) requirements for emails from specific domains. If TLS is required, the system drops mail without TLS from the specified domain.

</summary>

id: optional string

Sending domain restriction identifier.

formatuuid

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

domain: optional string

Domain that requires TLS enforcement.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

exclude: optional array of string

Subdomains to exempt from TLS requirements.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20exclude">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_list_response%20%3E%20(schema)>)

<details>

<summary>

SendingDomainRestrictionGetResponse object {id, comments, created\_at, 4 more }

A sending domain restriction that enforces TLS (Transport Layer Security) requirements for emails from specific domains. If TLS is required, the system drops mail without TLS from the specified domain.

</summary>

id: optional string

Sending domain restriction identifier.

formatuuid

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

domain: optional string

Domain that requires TLS enforcement.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

exclude: optional array of string

Subdomains to exempt from TLS requirements.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20exclude">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_get_response%20%3E%20(schema)>)

<details>

<summary>

SendingDomainRestrictionCreateResponse object {id, comments, created\_at, 4 more }

A sending domain restriction that enforces TLS (Transport Layer Security) requirements for emails from specific domains. If TLS is required, the system drops mail without TLS from the specified domain.

</summary>

id: optional string

Sending domain restriction identifier.

formatuuid

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

domain: optional string

Domain that requires TLS enforcement.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

exclude: optional array of string

Subdomains to exempt from TLS requirements.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20exclude">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_create_response%20%3E%20(schema)>)

<details>

<summary>

SendingDomainRestrictionEditResponse object {id, comments, created\_at, 4 more }

A sending domain restriction that enforces TLS (Transport Layer Security) requirements for emails from specific domains. If TLS is required, the system drops mail without TLS from the specified domain.

</summary>

id: optional string

Sending domain restriction identifier.

formatuuid

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

domain: optional string

Domain that requires TLS enforcement.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20domain">Link to this property</a>

exclude: optional array of string

Subdomains to exempt from TLS requirements.

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20exclude">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_edit_response%20%3E%20(schema)>)

<details>

<summary>

SendingDomainRestrictionDeleteResponse object {id }

</summary>

id: string

Sending domain restriction identifier.

formatuuid

<a href="#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.sending_domain_restrictions%20%3E%20(model)%20sending_domain_restriction_delete_response%20%3E%20(schema)>)

#### Email SecuritySettingsTrusted Domains

##### [List trusted email domains](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/trusted_domains/methods/list)

GET/accounts/{account\_id}/email-security/settings/trusted\_domains

##### [Get a trusted email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/trusted_domains/methods/get)

GET/accounts/{account\_id}/email-security/settings/trusted\_domains/{trusted\_domain\_id}

##### [Create trusted email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/trusted_domains/methods/create)

POST/accounts/{account\_id}/email-security/settings/trusted\_domains

##### [Update a trusted email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/trusted_domains/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/trusted\_domains/{trusted\_domain\_id}

##### [Delete a trusted email domain](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/trusted_domains/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/trusted\_domains/{trusted\_domain\_id}

##### [Batch trusted domain operations](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/trusted_domains/methods/batch)

POST/accounts/{account\_id}/email-security/settings/trusted\_domains/batch

##### ModelsExpand Collapse

<details>

<summary>

TrustedDomainListResponse object {id, comments, created\_at, 6 more }

A trusted email domain.

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_list_response%20%3E%20(schema)>)

<details>

<summary>

TrustedDomainGetResponse object {id, comments, created\_at, 6 more }

A trusted email domain.

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_get_response%20%3E%20(schema)>)

<details>

<summary>

TrustedDomainCreateResponse object {id, comments, created\_at, 6 more }

A trusted email domain.

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_create_response%20%3E%20(schema)>)

<details>

<summary>

TrustedDomainEditResponse object {id, comments, created\_at, 6 more }

A trusted email domain.

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_edit_response%20%3E%20(schema)>)

<details>

<summary>

TrustedDomainDeleteResponse object {id }

</summary>

id: string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_delete_response%20%3E%20(schema)>)

<details>

<summary>

TrustedDomainBatchResponse object {deletes, patches, posts, puts }

</summary>

<details>

<summary>

deletes: optional array of object {id }

</summary>

id: string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20deletes">Link to this property</a>

<details>

<summary>

patches: optional array of object {id, comments, created\_at, 6 more }

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20patches">Link to this property</a>

<details>

<summary>

posts: optional array of object {id, comments, created\_at, 6 more }

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20posts">Link to this property</a>

<details>

<summary>

puts: optional array of object {id, comments, created\_at, 6 more }

</summary>

id: optional string

Trusted domain identifier.

formatuuid

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20id">Link to this property</a>

comments: optional string

maxLength1024

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20comments">Link to this property</a>

created\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20created_at">Link to this property</a>

is\_recent: optional boolean

Select to prevent recently registered domains from triggering a Suspicious or Malicious disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_recent">Link to this property</a>

is\_regex: optional boolean

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_regex">Link to this property</a>

is\_similarity: optional boolean

Select for partner or other approved domains that have similar spelling to your connected domains. Prevents listed domains from triggering a Spoof disposition.

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20is_similarity">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20modified_at">Link to this property</a>

pattern: optional string

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts%20%3E%20(items)%20%3E%20(property)%20pattern">Link to this property</a>

</details>

<a href="#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)%20%3E%20(property)%20puts">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.trusted_domains%20%3E%20(model)%20trusted_domain_batch_response%20%3E%20(schema)>)

#### Email SecuritySettingsURL Ignore Patterns

##### [List URL ignore patterns](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/url_ignore_patterns/methods/list)

GET/accounts/{account\_id}/email-security/settings/url\_ignore\_patterns

##### [Get a URL ignore pattern](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/url_ignore_patterns/methods/get)

GET/accounts/{account\_id}/email-security/settings/url\_ignore\_patterns/{pattern\_id}

##### [Create a URL ignore pattern](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/url_ignore_patterns/methods/create)

POST/accounts/{account\_id}/email-security/settings/url\_ignore\_patterns

##### [Update a URL ignore pattern](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/url_ignore_patterns/methods/edit)

PATCH/accounts/{account\_id}/email-security/settings/url\_ignore\_patterns/{pattern\_id}

##### [Delete a URL ignore pattern](https://developers.cloudflare.com/api/resources/email_security/subresources/settings/subresources/url_ignore_patterns/methods/delete)

DELETE/accounts/{account\_id}/email-security/settings/url\_ignore\_patterns/{pattern\_id}

##### ModelsExpand Collapse

<details>

<summary>

URLIgnorePatternListResponse object {id, created\_at, pattern, 3 more }

A URL ignore pattern that exempts matching URLs from Email Security’s URL rewriting.

</summary>

id: string

URL ignore pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

pattern: string

Regular expression identifying URLs to exempt from rewriting.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

comments: optional string

Optional note describing the reason for the ignore pattern.

maxLength1024

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_list_response%20%3E%20(schema)>)

<details>

<summary>

URLIgnorePatternGetResponse object {id, created\_at, pattern, 3 more }

A URL ignore pattern that exempts matching URLs from Email Security’s URL rewriting.

</summary>

id: string

URL ignore pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

pattern: string

Regular expression identifying URLs to exempt from rewriting.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

comments: optional string

Optional note describing the reason for the ignore pattern.

maxLength1024

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_get_response%20%3E%20(schema)>)

<details>

<summary>

URLIgnorePatternCreateResponse object {id, created\_at, pattern, 3 more }

A URL ignore pattern that exempts matching URLs from Email Security’s URL rewriting.

</summary>

id: string

URL ignore pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

pattern: string

Regular expression identifying URLs to exempt from rewriting.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

comments: optional string

Optional note describing the reason for the ignore pattern.

maxLength1024

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_create_response%20%3E%20(schema)>)

<details>

<summary>

URLIgnorePatternEditResponse object {id, created\_at, pattern, 3 more }

A URL ignore pattern that exempts matching URLs from Email Security’s URL rewriting.

</summary>

id: string

URL ignore pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created\_at: string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)%20%3E%20(property)%20created_at">Link to this property</a>

pattern: string

Regular expression identifying URLs to exempt from rewriting.

maxLength1024

minLength1

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)%20%3E%20(property)%20pattern">Link to this property</a>

comments: optional string

Optional note describing the reason for the ignore pattern.

maxLength1024

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)%20%3E%20(property)%20comments">Link to this property</a>

Deprecatedlast\_modified: optional string

Use <code>modified_at</code> instead.

Deprecated, use <code>modified_at</code> instead. End of life: November 1, 2026.

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)%20%3E%20(property)%20last_modified">Link to this property</a>

modified\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified_at">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_edit_response%20%3E%20(schema)>)

<details>

<summary>

URLIgnorePatternDeleteResponse object {id }

</summary>

id: string

URL ignore pattern identifier.

formatuuid

<a href="#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_delete_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.settings.url_ignore_patterns%20%3E%20(model)%20url_ignore_pattern_delete_response%20%3E%20(schema)>)

#### Email SecuritySubmissions

##### [List reclassify submissions](https://developers.cloudflare.com/api/resources/email_security/subresources/submissions/methods/list)

GET/accounts/{account\_id}/email-security/submissions

##### ModelsExpand Collapse

<details>

<summary>

SubmissionListResponse object {requested\_at, submission\_id, customer\_status, 15 more }

</summary>

requested\_at: string

When the submission was requested (UTC).

formatdate-time

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_at">Link to this property</a>

submission\_id: string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20submission_id">Link to this property</a>

<details>

<summary>

customer\_status: optional "escalated"or "reviewed"or "unreviewed"

</summary>

One of the following:

"escalated"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20customer_status%20%3E%20(member)%200">Link to this property</a>

"reviewed"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20customer_status%20%3E%20(member)%201">Link to this property</a>

"unreviewed"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20customer_status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20customer_status">Link to this property</a>

<details>

<summary>

escalated\_as: optional "MALICIOUS"or "SUSPICIOUS"or "SPOOF"or 3 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as%20%3E%20(member)%200">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as%20%3E%20(member)%201">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as%20%3E%20(member)%202">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as%20%3E%20(member)%203">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as%20%3E%20(member)%204">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_as">Link to this property</a>

escalated\_at: optional string

formatdate-time

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_at">Link to this property</a>

escalated\_by: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_by">Link to this property</a>

escalated\_submission\_id: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20escalated_submission_id">Link to this property</a>

<details>

<summary>

original\_disposition: optional "MALICIOUS"or "SUSPICIOUS"or "SPOOF"or 3 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition%20%3E%20(member)%200">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition%20%3E%20(member)%201">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition%20%3E%20(member)%202">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition%20%3E%20(member)%203">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition%20%3E%20(member)%204">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_disposition">Link to this property</a>

original\_edf\_hash: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_edf_hash">Link to this property</a>

original\_postfix\_id: optional string

The postfix ID of the original message that was submitted.

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20original_postfix_id">Link to this property</a>

outcome: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome">Link to this property</a>

<details>

<summary>

outcome\_disposition: optional "MALICIOUS"or "SUSPICIOUS"or "SPOOF"or 3 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition%20%3E%20(member)%200">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition%20%3E%20(member)%201">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition%20%3E%20(member)%202">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition%20%3E%20(member)%203">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition%20%3E%20(member)%204">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20outcome_disposition">Link to this property</a>

requested\_by: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_by">Link to this property</a>

<details>

<summary>

requested\_disposition: optional "MALICIOUS"or "SUSPICIOUS"or "SPOOF"or 3 more

</summary>

One of the following:

"MALICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition%20%3E%20(member)%200">Link to this property</a>

"SUSPICIOUS"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition%20%3E%20(member)%201">Link to this property</a>

"SPOOF"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition%20%3E%20(member)%202">Link to this property</a>

"SPAM"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition%20%3E%20(member)%203">Link to this property</a>

"BULK"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition%20%3E%20(member)%204">Link to this property</a>

"NONE"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_disposition">Link to this property</a>

Deprecatedrequested\_ts: optional string

Use <code>requested_at</code> instead.

Deprecated, use <code>requested_at</code> instead.

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20requested_ts">Link to this property</a>

status: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

subject: optional string

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20subject">Link to this property</a>

<details>

<summary>

type: optional "Team"or "User"

Indicates whether a team member or an end user created the submission.

</summary>

One of the following:

"Team"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"User"

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_security.submissions%20%3E%20(model)%20submission_list_response%20%3E%20(schema)>)