---
title: Email Routing
---

[Skip to content](#_top)

[API Reference](https://developers.cloudflare.com/api)

Copy Markdown

Open in **Claude**Open in **ChatGPT**Open in **Cursor**

---

**Copy Markdown****View as Markdown**

# Email Routing

##### [Get Email Routing settings](https://developers.cloudflare.com/api/resources/email_routing/methods/get)

GET/zones/{zone\_id}/email/routing

##### [Update Email Routing settings](https://developers.cloudflare.com/api/resources/email_routing/methods/edit)

PATCH/zones/{zone\_id}/email/routing

##### [Apply Email Routing settings](https://developers.cloudflare.com/api/resources/email_routing/methods/update)

PUT/zones/{zone\_id}/email/routing

##### [Disable Email Routing](https://developers.cloudflare.com/api/resources/email_routing/methods/disable)

Deprecated

POST/zones/{zone\_id}/email/routing/disable

##### [Enable Email Routing](https://developers.cloudflare.com/api/resources/email_routing/methods/enable)

Deprecated

POST/zones/{zone\_id}/email/routing/enable

##### [Unlock Email Routing](https://developers.cloudflare.com/api/resources/email_routing/methods/unlock)

Deprecated

POST/zones/{zone\_id}/email/routing/unlock

##### ModelsExpand Collapse

<details>

<summary>

Settings object {id, enabled, name, 6 more }

</summary>

id: string

Email Routing settings identifier.

maxLength32

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

enabled: trueor false

State of the zone settings for Email Routing.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

name: string

Domain of your zone.

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

created: optional string

The date and time the settings have been created.

formatdate-time

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: optional string

The date and time the settings have been modified.

formatdate-time

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

<details>

<summary>

skip\_wizard: optional trueor false

Flag to check if the user skipped the configuration wizard.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20skip_wizard%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20skip_wizard%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20skip_wizard">Link to this property</a>

<details>

<summary>

status: optional "ready"or "unconfigured"or "misconfigured"or 2 more

Show the state of your account, and the type or configuration error.

</summary>

One of the following:

"ready"

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"unconfigured"

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"misconfigured"

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

"misconfigured/locked"

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%203">Link to this property</a>

"unlocked"

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%204">Link to this property</a>

</details>

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

<details>

<summary>

support\_subaddress: optional trueor false

Whether subaddressing (plus-addressing) is honored when matching incoming mail against routing rules.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20support_subaddress%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20support_subaddress%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20support_subaddress">Link to this property</a>

Deprecatedtag: optional string

Email Routing settings tag. (Deprecated, replaced by Email Routing settings identifier)

maxLength32

<a href="#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)%20%3E%20(property)%20tag">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing%20%3E%20(model)%20settings%20%3E%20(schema)>)

#### Email RoutingDNS

##### [Email Routing - DNS settings](https://developers.cloudflare.com/api/resources/email_routing/subresources/dns/methods/get)

GET/zones/{zone\_id}/email/routing/dns

##### [Enable Email Routing](https://developers.cloudflare.com/api/resources/email_routing/subresources/dns/methods/create)

POST/zones/{zone\_id}/email/routing/dns

##### [Unlock Email Routing DNS records](https://developers.cloudflare.com/api/resources/email_routing/subresources/dns/methods/edit)

PATCH/zones/{zone\_id}/email/routing/dns

##### [Disable Email Routing](https://developers.cloudflare.com/api/resources/email_routing/subresources/dns/methods/delete)

DELETE/zones/{zone\_id}/email/routing/dns

##### ModelsExpand Collapse

<details>

<summary>

DNSRecord object {content, name, priority, 2 more }

List of records needed to enable an Email Routing zone.

</summary>

content: optional string

DNS record content.

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20content">Link to this property</a>

name: optional string

DNS record name (or @ for the zone apex).

maxLength255

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

priority: optional number

Required for MX, SRV and URI records. Unused by other record types. Records with lower priorities are preferred.

maximum65535

minimum0

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20priority">Link to this property</a>

<details>

<summary>

ttl: optional numberor 1

Time to live, in seconds, of the DNS record. Must be between 60 and 86400, or 1 for ‘automatic’.

</summary>

One of the following:

number

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20ttl%20%3E%20(variant)%200">Link to this property</a>

1

Time to live, in seconds, of the DNS record. Must be between 60 and 86400, or 1 for ‘automatic’.

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20ttl%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20ttl">Link to this property</a>

<details>

<summary>

type: optional "A"or "AAAA"or "CNAME"or 15 more

DNS record type.

</summary>

One of the following:

"A"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"AAAA"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"CNAME"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

"HTTPS"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%203">Link to this property</a>

"TXT"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%204">Link to this property</a>

"SRV"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%205">Link to this property</a>

"LOC"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%206">Link to this property</a>

"MX"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%207">Link to this property</a>

"NS"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%208">Link to this property</a>

"CERT"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%209">Link to this property</a>

"DNSKEY"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2010">Link to this property</a>

"DS"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2011">Link to this property</a>

"NAPTR"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2012">Link to this property</a>

"SMIMEA"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2013">Link to this property</a>

"SSHFP"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2014">Link to this property</a>

"SVCB"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2015">Link to this property</a>

"TLSA"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2016">Link to this property</a>

"URI"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2017">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)>)

<details>

<summary>

DNSGetResponse = array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)">DNSRecord</a> { content, name, priority, 2 more }

</summary>

content: optional string

DNS record content.

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20content">Link to this property</a>

name: optional string

DNS record name (or @ for the zone apex).

maxLength255

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

priority: optional number

Required for MX, SRV and URI records. Unused by other record types. Records with lower priorities are preferred.

maximum65535

minimum0

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20priority">Link to this property</a>

<details>

<summary>

ttl: optional numberor 1

Time to live, in seconds, of the DNS record. Must be between 60 and 86400, or 1 for ‘automatic’.

</summary>

One of the following:

number

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20ttl%20%3E%20(variant)%200">Link to this property</a>

1

Time to live, in seconds, of the DNS record. Must be between 60 and 86400, or 1 for ‘automatic’.

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20ttl%20%3E%20(variant)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20ttl">Link to this property</a>

<details>

<summary>

type: optional "A"or "AAAA"or "CNAME"or 15 more

DNS record type.

</summary>

One of the following:

"A"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"AAAA"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"CNAME"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

"HTTPS"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%203">Link to this property</a>

"TXT"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%204">Link to this property</a>

"SRV"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%205">Link to this property</a>

"LOC"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%206">Link to this property</a>

"MX"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%207">Link to this property</a>

"NS"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%208">Link to this property</a>

"CERT"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%209">Link to this property</a>

"DNSKEY"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2010">Link to this property</a>

"DS"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2011">Link to this property</a>

"NAPTR"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2012">Link to this property</a>

"SMIMEA"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2013">Link to this property</a>

"SSHFP"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2014">Link to this property</a>

"SVCB"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2015">Link to this property</a>

"TLSA"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2016">Link to this property</a>

"URI"

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%2017">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.dns%20%3E%20(model)%20dns_record%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.dns%20%3E%20(model)%20dns_get_response%20%3E%20(schema)>)

#### Email RoutingRules

##### [List account or zone routing rules](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/methods/list)

GET/{accounts\_or\_zones}/{account\_or\_zone\_id}/email/routing/rules

##### [Get routing rule](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/methods/get)

GET/zones/{zone\_id}/email/routing/rules/{rule\_identifier}

##### [Create routing rule](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/methods/create)

POST/zones/{zone\_id}/email/routing/rules

##### [Update routing rule](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/methods/update)

PUT/zones/{zone\_id}/email/routing/rules/{rule\_identifier}

##### [Delete routing rule](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/methods/delete)

DELETE/zones/{zone\_id}/email/routing/rules/{rule\_identifier}

##### ModelsExpand Collapse

<details>

<summary>

Action object {type, value }

Actions pattern.

</summary>

<details>

<summary>

type: "drop"or "forward"or "worker"

Type of supported action.

</summary>

One of the following:

"drop"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"forward"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"worker"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

value: optional array of string

List of values for the action. Currently limited to a single value.

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)>)

<details>

<summary>

EmailRoutingRule object {id, actions, enabled, 5 more }

</summary>

id: optional string

Routing rule identifier.

maxLength32

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

actions: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)">Action</a> { type, value }

List actions patterns.

</summary>

<details>

<summary>

type: "drop"or "forward"or "worker"

Type of supported action.

</summary>

One of the following:

"drop"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"forward"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"worker"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

value: optional array of string

List of values for the action. Currently limited to a single value.

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20actions">Link to this property</a>

<details>

<summary>

enabled: optional trueor false

Routing rule status.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

matchers: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)">Matcher</a> { type, field, value }

Matching patterns to forward to your actions.

</summary>

<details>

<summary>

type: "all"or "literal"

Type of matcher.

</summary>

One of the following:

"all"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"literal"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

field: optional "to"

Field for type matcher.

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20field">Link to this property</a>

value: optional string

Value for matcher.

maxLength90

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20matchers">Link to this property</a>

name: optional string

Routing rule name.

maxLength256

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

priority: optional number

Priority of the routing rule.

minimum0

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20priority">Link to this property</a>

<details>

<summary>

source: optional "api"or "wrangler"

Who manages the rule. <code>api</code> covers dashboard, generic API, and Terraform; <code>wrangler</code> means the rule is managed by a Worker’s wrangler.jsonc. Defaults to <code>api</code> when omitted on write.

</summary>

One of the following:

"api"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%200">Link to this property</a>

"wrangler"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20source">Link to this property</a>

Deprecatedtag: optional string

Routing rule tag. (Deprecated, replaced by routing rule identifier)

maxLength32

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)%20%3E%20(property)%20tag">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules%20%3E%20(model)%20email_routing_rule%20%3E%20(schema)>)

<details>

<summary>

Matcher object {type, field, value }

Matching pattern to forward your actions.

</summary>

<details>

<summary>

type: "all"or "literal"

Type of matcher.

</summary>

One of the following:

"all"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"literal"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

field: optional "to"

Field for type matcher.

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20field">Link to this property</a>

value: optional string

Value for matcher.

maxLength90

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)>)

#### Email RoutingRulesCatch Alls

##### [Get catch-all rule](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/subresources/catch_alls/methods/get)

GET/zones/{zone\_id}/email/routing/rules/catch\_all

##### [Update catch-all rule](https://developers.cloudflare.com/api/resources/email_routing/subresources/rules/subresources/catch_alls/methods/update)

PUT/zones/{zone\_id}/email/routing/rules/catch\_all

##### ModelsExpand Collapse

<details>

<summary>

CatchAllAction object {type, value }

Action for the catch-all routing rule.

</summary>

<details>

<summary>

type: "drop"or "forward"or "worker"

Type of action for catch-all rule.

</summary>

One of the following:

"drop"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"forward"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"worker"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

value: optional array of string

List of values for the action. Currently limited to a single value.

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)>)

<details>

<summary>

CatchAllMatcher object {type }

Matcher for catch-all routing rule.

</summary>

type: "all"

Type of matcher. Default is ‘all’.

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_matcher%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_matcher%20%3E%20(schema)>)

<details>

<summary>

CatchAllGetResponse object {id, actions, enabled, 4 more }

</summary>

id: optional string

Routing rule identifier.

maxLength32

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

actions: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)">CatchAllAction</a> { type, value }

List actions for the catch-all routing rule.

</summary>

<details>

<summary>

type: "drop"or "forward"or "worker"

Type of action for catch-all rule.

</summary>

One of the following:

"drop"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"forward"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"worker"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

value: optional array of string

List of values for the action. Currently limited to a single value.

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20actions">Link to this property</a>

<details>

<summary>

enabled: optional trueor false

Routing rule status.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

matchers: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_matcher%20%3E%20(schema)">CatchAllMatcher</a> { type }

List of matchers for the catch-all routing rule.

</summary>

type: "all"

Type of matcher. Default is ‘all’.

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_matcher%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20matchers">Link to this property</a>

name: optional string

Routing rule name.

maxLength256

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

source: optional "api"or "wrangler"

Who manages the rule. <code>api</code> covers dashboard, generic API, and Terraform; <code>wrangler</code> means the rule is managed by a Worker’s wrangler.jsonc. Defaults to <code>api</code> when omitted on write.

</summary>

One of the following:

"api"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%200">Link to this property</a>

"wrangler"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20source">Link to this property</a>

Deprecatedtag: optional string

Routing rule tag. (Deprecated, replaced by routing rule identifier)

maxLength32

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)%20%3E%20(property)%20tag">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_get_response%20%3E%20(schema)>)

<details>

<summary>

CatchAllUpdateResponse object {id, actions, enabled, 4 more }

</summary>

id: optional string

Routing rule identifier.

maxLength32

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

actions: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)">CatchAllAction</a> { type, value }

List actions for the catch-all routing rule.

</summary>

<details>

<summary>

type: "drop"or "forward"or "worker"

Type of action for catch-all rule.

</summary>

One of the following:

"drop"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"forward"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"worker"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

value: optional array of string

List of values for the action. Currently limited to a single value.

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_action%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20actions">Link to this property</a>

<details>

<summary>

enabled: optional trueor false

Routing rule status.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

matchers: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_matcher%20%3E%20(schema)">CatchAllMatcher</a> { type }

List of matchers for the catch-all routing rule.

</summary>

type: "all"

Type of matcher. Default is ‘all’.

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_matcher%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20matchers">Link to this property</a>

name: optional string

Routing rule name.

maxLength256

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

source: optional "api"or "wrangler"

Who manages the rule. <code>api</code> covers dashboard, generic API, and Terraform; <code>wrangler</code> means the rule is managed by a Worker’s wrangler.jsonc. Defaults to <code>api</code> when omitted on write.

</summary>

One of the following:

"api"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%200">Link to this property</a>

"wrangler"

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20source">Link to this property</a>

Deprecatedtag: optional string

Routing rule tag. (Deprecated, replaced by routing rule identifier)

maxLength32

<a href="#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)%20%3E%20(property)%20tag">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.rules.catch_alls%20%3E%20(model)%20catch_all_update_response%20%3E%20(schema)>)

#### Email RoutingAccount Rules

##### [List account or zone routing rules](https://developers.cloudflare.com/api/resources/email_routing/subresources/account_rules/methods/list)

GET/{accounts\_or\_zones}/{account\_or\_zone\_id}/email/routing/rules

##### ModelsExpand Collapse

<details>

<summary>

AccountRule object {id, actions, enabled, 6 more }

</summary>

id: optional string

Routing rule identifier.

maxLength32

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

<details>

<summary>

actions: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)">Action</a> { type, value }

List actions patterns.

</summary>

<details>

<summary>

type: "drop"or "forward"or "worker"

Type of supported action.

</summary>

One of the following:

"drop"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"forward"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

"worker"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

value: optional array of string

List of values for the action. Currently limited to a single value.

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20action%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20actions">Link to this property</a>

<details>

<summary>

enabled: optional trueor false

Routing rule status.

</summary>

One of the following:

true

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%200">Link to this property</a>

false

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20enabled%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20enabled">Link to this property</a>

<details>

<summary>

matchers: optional array of <a href="https://developers.cloudflare.com/api/resources/email_routing#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)">Matcher</a> { type, field, value }

Matching patterns to forward to your actions.

</summary>

<details>

<summary>

type: "all"or "literal"

Type of matcher.

</summary>

One of the following:

"all"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%200">Link to this property</a>

"literal"

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20type">Link to this property</a>

field: optional "to"

Field for type matcher.

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20field">Link to this property</a>

value: optional string

Value for matcher.

maxLength90

<a href="#(resource)%20email_routing.rules%20%3E%20(model)%20matcher%20%3E%20(schema)%20%3E%20(property)%20value">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20matchers">Link to this property</a>

name: optional string

Routing rule name.

maxLength256

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

priority: optional number

Priority of the routing rule.

minimum0

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20priority">Link to this property</a>

<details>

<summary>

source: optional "api"or "wrangler"

Who manages the rule. <code>api</code> covers dashboard, generic API, and Terraform; <code>wrangler</code> means the rule is managed by a Worker’s wrangler.jsonc. Defaults to <code>api</code> when omitted on write.

</summary>

One of the following:

"api"

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%200">Link to this property</a>

"wrangler"

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20source%20%3E%20(member)%201">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20source">Link to this property</a>

Deprecatedtag: optional string

Routing rule tag. (Deprecated, replaced by routing rule identifier)

maxLength32

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20tag">Link to this property</a>

<details>

<summary>

zone: optional object {name, tag }

Zone information for the routing rule.

</summary>

name: optional string

Zone name.

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20zone%20%3E%20(property)%20name">Link to this property</a>

tag: optional string

Zone tag.

maxLength32

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20zone%20%3E%20(property)%20tag">Link to this property</a>

</details>

<a href="#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)%20%3E%20(property)%20zone">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.account_rules%20%3E%20(model)%20account_rule%20%3E%20(schema)>)

#### Email RoutingAddresses

##### [List destination addresses](https://developers.cloudflare.com/api/resources/email_routing/subresources/addresses/methods/list)

GET/accounts/{account\_id}/email/routing/addresses

##### [Get a destination address](https://developers.cloudflare.com/api/resources/email_routing/subresources/addresses/methods/get)

GET/accounts/{account\_id}/email/routing/addresses/{destination\_address\_identifier}

##### [Create a destination address](https://developers.cloudflare.com/api/resources/email_routing/subresources/addresses/methods/create)

POST/accounts/{account\_id}/email/routing/addresses

##### [Update destination address](https://developers.cloudflare.com/api/resources/email_routing/subresources/addresses/methods/edit)

PATCH/accounts/{account\_id}/email/routing/addresses/{destination\_address\_identifier}

##### [Delete destination address](https://developers.cloudflare.com/api/resources/email_routing/subresources/addresses/methods/delete)

DELETE/accounts/{account\_id}/email/routing/addresses/{destination\_address\_identifier}

##### ModelsExpand Collapse

<details>

<summary>

Address object {id, created, email, 3 more }

</summary>

id: optional string

Destination address identifier.

maxLength32

<a href="#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: optional string

The date and time the destination address has been created.

formatdate-time

<a href="#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

email: optional string

The contact email address of the user.

maxLength90

<a href="#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)%20%3E%20(property)%20email">Link to this property</a>

modified: optional string

The date and time the destination address was last modified.

formatdate-time

<a href="#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

Deprecatedtag: optional string

Destination address tag. (Deprecated, replaced by destination address identifier)

maxLength32

<a href="#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)%20%3E%20(property)%20tag">Link to this property</a>

verified: optional string

The date and time the destination address has been verified. Null means not verified yet.

formatdate-time

<a href="#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)%20%3E%20(property)%20verified">Link to this property</a>

</details>

[Link to this property](<#(resource)%20email_routing.addresses%20%3E%20(model)%20address%20%3E%20(schema)>)