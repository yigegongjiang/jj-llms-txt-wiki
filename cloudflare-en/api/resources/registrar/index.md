---
title: Registrar
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# Registrar

Registrar API for searching, checking, registering, and managing domains through Cloudflare Registrar.

## Prerequisites

Before using this API, ensure:

1. **Cloudflare account** — the caller must have a valid Cloudflare account.
2. **Billing profile** — the account must have a billing profile with a valid, current default payment method (credit card or other accepted method). This cannot be set up via API — the account owner must configure billing at `https://dash.cloudflare.com/{account_id}/billing/payment-info` before calling `POST /registrations`.
3. **API authentication** — use an API token or API key with the appropriate Registrar permissions for the operations you are calling.

## Terminology: domain extension

Throughout this API, “extension” refers to the domain extension part of a fully qualified domain name — the portion after the registrable label. For example, in `example.co.uk`, the extension is `co.uk` (not just `uk`). This covers both top-level domains like `com` and multi-level extensions like `co.uk`. This is distinct from other uses of the word “extension” (e.g., EPP extensions).

## Supported extensions

This API supports programmatic registration for all extensions supported by the dashboard experience, with the following exceptions:

`giving`, `mom`, `inc`, `lol`, `sh`, `link`, `cc`, `new`

Cloudflare Registrar supports 400+ extensions in the dashboard. Extensions listed above can be registered at `https://dash.cloudflare.com/{account_id}/domains/registrations`.

## Typical workflow

1. **Search** — call `GET /domain-search?q={keyword}` to discover available domains.
2. **Check** — call `POST /domain-check` with candidate domains to verify real-time availability and pricing.
3. **Review the response** — if `registrable: false`, inspect `reason` to understand whether the domain is unavailable, the extension is not supported by this API, the extension is not supported by Cloudflare Registrar at all, or the extension’s registry has frozen new registrations.
4. **Handle premium domains** — if `tier: premium`, premium registration is not currently supported by this API. Surface the premium pricing to the user, but do not proceed to `POST /registrations` for that domain.
5. **Observe the registration schema** — call `GET /extensions/:extension_name`to discover the required values for registering this extension.
6. **Register** — call `POST /registrations` with the chosen domain name for supported non-premium registrations.
7. **Confirm completion** — if the response is `201 Created`, registration completed within the default timeout and no polling is needed.
8. **Poll when needed** — if the response is `202 Accepted`, poll `links.self` from the workflow response.
9. **Stop for user action** — if `state: action_required`, stop polling and surface `context.action` to the user. The workflow will not resolve on its own.
10. **Continue when blocked** — if `state: blocked`, continue polling and inform the user that a third party, such as the extension registry or losing registrar, is delaying progress.
11. **Review failures before retrying** — if `state: failed`, review `error.code` and `error.message`, then decide whether user action or a new Check call is needed.

**All successful domain registrations are non-refundable.** Once the registration workflow completes with `state: succeeded`, the charge cannot be reversed. Confirm pricing and domain choice with the user before calling `POST /registrations`.

## Default behavior for mutating operations

By default, mutating operations such as create and update hold the connection for a bounded, server-defined amount of time while the operation completes. In most cases, the response contains a completed workflow status and no polling is required.

* **Completed within the synchronous wait window:** Returns `201` (create) or `200` (update) with a `workflow_status` where `state: succeeded` and `completed: true`.
* **Still processing after the synchronous wait window:** Returns `202 Accepted` with a `workflow_status` where `completed: false`. Use the `links.self` URL to poll for completion.

## Non-blocking mode

To receive an immediate `202 Accepted` response without waiting, send the `Prefer: respond-async` request header (RFC 7240). The server will acknowledge it with a `Preference-Applied: respond-async` response header.

## Polling

When the response is `202`, poll the workflow status endpoint indicated by `links.self` in the response body until the workflow reaches a terminal state or requires user action.

##### [Search for available domains](https://developers.cloudflare.com/api/resources/registrar/methods/search)

GET/accounts/{account\_id}/registrar/domain-search

##### [Check domain availability](https://developers.cloudflare.com/api/resources/registrar/methods/check)

POST/accounts/{account\_id}/registrar/domain-check

##### ModelsExpand Collapse 

Registration object { auto\_renew, created\_at, domain\_name, 4 more } 

A domain registration resource representing the current state of a registered domain.

auto\_renew: boolean

Whether the domain will be automatically renewed before expiration.

created\_at: string

When the domain was registered. Present when the registration resource exists.

formatdate-time

domain\_name: string

Fully qualified domain name (FQDN) including the extension (e.g., `example.com`, `mybrand.app`). The domain name uniquely identifies a registration — the same domain cannot be registered twice, making it a natural idempotency key for registration requests.

expires\_at: string

When the domain registration expires. Present when the registration is ready; may be null only while `status` is `registration_pending`.

formatdate-time

locked: boolean

Whether the domain is locked for transfer.

privacy\_mode: "off" or "redaction"

Current WHOIS privacy mode for the registration.

One of the following:

"off"

"redaction"

status: "active" or "registration\_pending" or "expired" or 3 more

Current registration status.

* `active`: Domain is registered and operational
* `registration_pending`: Registration is in progress
* `expired`: Domain has expired
* `suspended`: Domain is suspended by the registry
* `redemption_period`: Domain is in the redemption grace period
* `pending_delete`: Domain is pending deletion by the registry

One of the following:

"active"

"registration\_pending"

"expired"

"suspended"

"redemption\_period"

"pending\_delete"

WorkflowStatus object { completed, created\_at, links, 4 more } 

Status of an async registration workflow.

completed: boolean

Whether the workflow has reached a terminal state. `true` when `state` is `succeeded` or `failed`. `false` for `pending`, `in_progress`, `action_required`, and `blocked`.

created\_at: string

formatdate-time

links: object { self, resource } 

self: string

URL to this status resource.

resource: optional string

URL to the domain resource.

state: "pending" or "in\_progress" or "action\_required" or 3 more

Workflow lifecycle state.

* `pending`: Workflow has been created but not yet started processing.
* `in_progress`: Actively processing. Continue polling `links.self`. The workflow has an internal deadline and will not remain in this state indefinitely.
* `action_required`: Paused — requires action by the user (not the system). See `context.action` for what is needed. An automated polling loop must break on this state; it will not resolve on its own without user intervention.
* `blocked`: The workflow cannot make progress due to a third party such as the domain extension’s registry or a losing registrar. No user action will help. Continue polling — the block may resolve when the third party responds.
* `succeeded`: Terminal. The operation completed successfully. `completed` will be `true`. For registrations, `context.registration`contains the resulting registration resource.
* `failed`: Terminal. The operation failed. `completed` will be `true`. See `error.code` and `error.message` for the reason. Do not auto-retry without user review.

One of the following:

"pending"

"in\_progress"

"action\_required"

"blocked"

"succeeded"

"failed"

updated\_at: string

formatdate-time

context: optional map\[unknown\]

Workflow-specific data for this workflow.

The workflow subject is identified by `context.domain_name` for domain-centric workflows.

error: optional object { code, message } 

Error details when a workflow reaches the `failed` state. The specific error codes and messages depend on the workflow type (registration, update, etc.) and the underlying registry response. These workflow error codes are separate from immediate HTTP error `errors[].code`values returned by non-2xx responses. Surface `error.message` to the user for context.

code: string

Machine-readable error code identifying the failure reason.

message: string

Human-readable explanation of the failure. May include registry-specific details.

RegistrarSearchResponse object { domains } 

Contains the search results.

domains: array of object { name, registrable, pricing, 2 more } 

Array of domain suggestions sorted by relevance. May be empty if no domains match the search criteria.

name: string

The fully qualified domain name (FQDN) in punycode format for internationalized domain names (IDNs).

registrable: boolean

Indicates whether this domain appears available based on search data. Search results are non-authoritative and may be stale. - `true`: The domain appears available. Use POST /domain-check to confirm before registration.

* `false`: The domain does not appear available in search results.

pricing: optional object { currency, registration\_cost, renewal\_cost } 

Annual pricing information for a registrable domain. This object is only present when `registrable` is `true`. All prices are per year and returned as strings to preserve decimal precision.

`registration_cost` and `renewal_cost` are frequently the same value, but may differ — especially for premium domains where registries set different rates for initial registration vs. renewal. For a multi-year registration (e.g., 4 years), the first year is charged at `registration_cost` and each subsequent year at `renewal_cost`. Registry pricing may change over time; the values returned here reflect the current registry rate. Premium pricing may be surfaced by Search and Check, but premium registration is not currently supported by this API.

currency: string

ISO-4217 currency code for the prices (e.g., “USD”, “EUR”, “GBP”).

registration\_cost: string

The first-year cost to register this domain. For premium domains (`tier: premium`), this price is set by the registry and may be significantly higher than standard pricing. For multi-year registrations, this cost applies to the first year only; subsequent years are charged at `renewal_cost`.

renewal\_cost: string

Per-year renewal cost for this domain. Applied to each year beyond the first year of a multi-year registration, and to each annual auto-renewal thereafter. May differ from `registration_cost`, especially for premium domains where initial registration often costs more than renewals.

reason: optional "extension\_not\_supported\_via\_api" or "extension\_not\_supported" or "extension\_disallows\_registration" or 2 more

Present only when `registrable` is `false` on search results. Explains why the domain does not appear registrable through this API. These values are advisory; use POST /domain-check for authoritative status.

* `extension_not_supported_via_api`: Cloudflare Registrar supports this extension in the dashboard but it is not yet available for programmatic registration via this API.
* `extension_not_supported`: This extension is not supported by Cloudflare Registrar at all.
* `extension_disallows_registration`: The extension’s registry has temporarily or permanently frozen new registrations.
* `domain_premium`: The domain is premium priced. Premium registration is not currently supported by this API.
* `domain_unavailable`: The domain appears unavailable.

One of the following:

"extension\_not\_supported\_via\_api"

"extension\_not\_supported"

"extension\_disallows\_registration"

"domain\_premium"

"domain\_unavailable"

tier: optional "standard" or "premium"

The pricing tier for this domain. Always present when `registrable` is `true`; defaults to `standard` for most domains. May be absent when `registrable`is `false`.

* `standard`: Standard registry pricing
* `premium`: Premium domain with higher pricing set by the registry

One of the following:

"standard"

"premium"

RegistrarCheckResponse object { domains } 

Contains the availability check results.

domains: array of object { name, registrable, pricing, 2 more } 

Array of domain availability results. Domains on unsupported extensions are included with `registrable: false` and a `reason`field. Malformed domain names may be omitted.

name: string

The fully qualified domain name (FQDN) in punycode format for internationalized domain names (IDNs).

registrable: boolean

Indicates whether this domain can be registered programmatically through this API based on a real-time registry check.

* `true`: Domain is available for registration. The `pricing` object will be included.
* `false`: Domain is not available. See the `reason` field for why. `tier` may still be present on some non-registrable results, such as premium domains.

pricing: optional object { currency, registration\_cost, renewal\_cost } 

Annual pricing information for a registrable domain. This object is only present when `registrable` is `true`. All prices are per year and returned as strings to preserve decimal precision.

`registration_cost` and `renewal_cost` are frequently the same value, but may differ — especially for premium domains where registries set different rates for initial registration vs. renewal. For a multi-year registration (e.g., 4 years), the first year is charged at `registration_cost` and each subsequent year at `renewal_cost`. Registry pricing may change over time; the values returned here reflect the current registry rate. Premium pricing may be surfaced by Search and Check, but premium registration is not currently supported by this API.

currency: string

ISO-4217 currency code for the prices (e.g., “USD”, “EUR”, “GBP”).

registration\_cost: string

The first-year cost to register this domain. For premium domains (`tier: premium`), this price is set by the registry and may be significantly higher than standard pricing. For multi-year registrations, this cost applies to the first year only; subsequent years are charged at `renewal_cost`.

renewal\_cost: string

Per-year renewal cost for this domain. Applied to each year beyond the first year of a multi-year registration, and to each annual auto-renewal thereafter. May differ from `registration_cost`, especially for premium domains where initial registration often costs more than renewals.

reason: optional "extension\_not\_supported\_via\_api" or "extension\_not\_supported" or "extension\_disallows\_registration" or 2 more

Present only when `registrable` is `false`. Explains why the domain cannot be registered via this API.

* `extension_not_supported_via_api`: Cloudflare Registrar supports this extension in the dashboard but it is not yet available for programmatic registration via this API. The user can register via `https://dash.cloudflare.com/{account_id}/domains/registrations`.
* `extension_not_supported`: This extension is not supported by Cloudflare Registrar at all.
* `extension_disallows_registration`: The extension’s registry has temporarily or permanently frozen new registrations. No registrar can register domains on this extension at this time.
* `domain_premium`: The domain is premium priced. Premium registration is not currently supported by this API.
* `domain_unavailable`: The domain is already registered, reserved, or otherwise not available on a supported extension.

One of the following:

"extension\_not\_supported\_via\_api"

"extension\_not\_supported"

"extension\_disallows\_registration"

"domain\_premium"

"domain\_unavailable"

tier: optional "standard" or "premium"

The pricing tier for this domain. Always present when `registrable` is `true`; defaults to `standard` for most domains. May be absent when `registrable` is `false`.

* `standard`: Standard registry pricing
* `premium`: Premium domain with higher pricing set by the registry

One of the following:

"standard"

"premium"

#### RegistrarDomains

##### [List domains](https://developers.cloudflare.com/api/resources/registrar/subresources/domains/methods/list)

Deprecated

GET/accounts/{account\_id}/registrar/domains

##### [Get domain](https://developers.cloudflare.com/api/resources/registrar/subresources/domains/methods/get)

Deprecated

GET/accounts/{account\_id}/registrar/domains/{domain\_name}

##### [Update domain](https://developers.cloudflare.com/api/resources/registrar/subresources/domains/methods/update)

Deprecated

PUT/accounts/{account\_id}/registrar/domains/{domain\_name}

##### ModelsExpand Collapse 

Domain object { id, available, can\_register, 9 more } 

id: optional string

Domain identifier.

maxLength32

available: optional boolean

Shows if a domain is available for transferring into Cloudflare Registrar.

can\_register: optional boolean

Indicates if the domain can be registered as a new domain.

created\_at: optional string

Shows time of creation.

formatdate-time

current\_registrar: optional string

Shows name of current registrar.

expires\_at: optional string

Shows when domain name registration expires.

formatdate-time

locked: optional boolean

Shows whether a registrar lock is in place for a domain.

registrant\_contact: optional object { address, city, country, 10 more } 

Shows contact information for domain registrant.

address: string

Address.

city: string

City.

country: string

The country in which the user lives.

maxLength30

first\_name: string

User’s first name

maxLength60

last\_name: string

User’s last name

maxLength60

organization: string

Name of organization.

phone: string

User’s telephone number

maxLength20

state: string

State.

zip: string

The zipcode or postal code where the user lives.

maxLength20

id: optional string

Contact Identifier.

maxLength32

address2: optional string

Optional address line for unit, floor, suite, etc.

email: optional string

The contact email address of the user.

maxLength90

fax: optional string

Contact fax number.

registry\_statuses: optional string

A comma-separated list of registry status codes. A full list of status codes can be found at [EPP Status Codes](https://www.icann.org/resources/pages/epp-status-codes-2014-06-16-en).

supported\_tld: optional boolean

Whether a particular TLD is currently supported by Cloudflare Registrar. Refer to [TLD Policies](https://www.cloudflare.com/tld-policies/) for a list of supported TLDs.

transfer\_in: optional object { accept\_foa, approve\_transfer, can\_cancel\_transfer, 3 more } 

Statuses for domain transfers into Cloudflare Registrar.

accept\_foa: optional "needed" or "ok"

Form of authorization has been accepted by the registrant.

One of the following:

"needed"

"ok"

approve\_transfer: optional "needed" or "ok" or "pending" or 3 more

Shows transfer status with the registry.

One of the following:

"needed"

"ok"

"pending"

"trying"

"rejected"

"unknown"

can\_cancel\_transfer: optional boolean

Indicates if cancellation is still possible.

disable\_privacy: optional "needed" or "ok" or "unknown"

Privacy guards are disabled at the foreign registrar.

One of the following:

"needed"

"ok"

"unknown"

enter\_auth\_code: optional "needed" or "ok" or "pending" or 2 more

Auth code has been entered and verified.

One of the following:

"needed"

"ok"

"pending"

"trying"

"rejected"

unlock\_domain: optional "needed" or "ok" or "pending" or 2 more

Domain is unlocked at the foreign registrar.

One of the following:

"needed"

"ok"

"pending"

"trying"

"unknown"

updated\_at: optional string

Last updated.

formatdate-time

DomainGetResponse \= unknown

DomainUpdateResponse \= unknown

#### RegistrarRegistrations

##### [Create Registration](https://developers.cloudflare.com/api/resources/registrar/subresources/registrations/methods/create)

POST/accounts/{account\_id}/registrar/registrations

##### [List Registrations](https://developers.cloudflare.com/api/resources/registrar/subresources/registrations/methods/list)

GET/accounts/{account\_id}/registrar/registrations

##### [Get Registration](https://developers.cloudflare.com/api/resources/registrar/subresources/registrations/methods/get)

GET/accounts/{account\_id}/registrar/registrations/{domain\_name}

##### [Update Registration](https://developers.cloudflare.com/api/resources/registrar/subresources/registrations/methods/edit)

PATCH/accounts/{account\_id}/registrar/registrations/{domain\_name}

#### RegistrarRegistration Status

##### [Get Registration Status](https://developers.cloudflare.com/api/resources/registrar/subresources/registration%5Fstatus/methods/get)

GET/accounts/{account\_id}/registrar/registrations/{domain\_name}/registration-status

#### RegistrarUpdate Status

##### [Get Update Status](https://developers.cloudflare.com/api/resources/registrar/subresources/update%5Fstatus/methods/get)

GET/accounts/{account\_id}/registrar/registrations/{domain\_name}/update-status

#### RegistrarExtensions

##### [List extensions](https://developers.cloudflare.com/api/resources/registrar/subresources/extensions/methods/list)

GET/accounts/{account\_id}/registrar/extensions

##### [Get extension](https://developers.cloudflare.com/api/resources/registrar/subresources/extensions/methods/get)

GET/accounts/{account\_id}/registrar/extensions/{extension}

##### ModelsExpand Collapse 

ExtensionListResponse object { metadata, registration\_schema } 

Extension entry with metadata and JSON Schema documents for the registration operation.

metadata: object { name, tld } 

Extension metadata

name: string

The full name of the extension. For example, “co.uk”, or “uk”

tld: string

The tld of the extension. For example, for “co.uk”, it’s “uk”. For “uk”, it’s “uk”

registration\_schema: unknown

JSON Schema describing the expected input structure for registration operations on this extension.

ExtensionGetResponse object { metadata, registration\_schema } 

Extension entry with metadata and JSON Schema documents for the registration operation.

metadata: object { name, tld } 

Extension metadata

name: string

The full name of the extension. For example, “co.uk”, or “uk”

tld: string

The tld of the extension. For example, for “co.uk”, it’s “uk”. For “uk”, it’s “uk”

registration\_schema: unknown

JSON Schema describing the expected input structure for registration operations on this extension.