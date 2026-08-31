---
title: Email Sending
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

# Email Sending

##### [Send an email](https://developers.cloudflare.com/api/resources/email%5Fsending/methods/send)

POST/accounts/{account\_id}/email/sending/send

##### [Send a raw MIME email](https://developers.cloudflare.com/api/resources/email%5Fsending/methods/send%5Fraw)

POST/accounts/{account\_id}/email/sending/send\_raw

##### ModelsExpand Collapse 

EmailSendingSendResponse object { delivered, message\_id, permanent\_bounces, queued } 

delivered: array of string

Email addresses to which the message was delivered immediately.

message\_id: string

Message ID of the sent email.

permanent\_bounces: array of string

Email addresses that permanently bounced.

queued: array of string

Email addresses for which delivery was queued for later.

EmailSendingSendRawResponse object { delivered, message\_id, permanent\_bounces, queued } 

delivered: array of string

Email addresses to which the message was delivered immediately.

message\_id: string

Message ID of the sent email.

permanent\_bounces: array of string

Email addresses that permanently bounced.

queued: array of string

Email addresses for which delivery was queued for later.

#### Email SendingSuppressions

##### [List account Email Sending suppressions](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/suppressions/methods/list)

GET/accounts/{account\_id}/email/sending/suppressions

##### [Get account Email Sending suppression](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/suppressions/methods/get)

GET/accounts/{account\_id}/email/sending/suppressions/{suppression\_id}

##### [Create account Email Sending suppression](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/suppressions/methods/create)

POST/accounts/{account\_id}/email/sending/suppressions

##### [Update account Email Sending suppression](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/suppressions/methods/edit)

PATCH/accounts/{account\_id}/email/sending/suppressions/{suppression\_id}

##### [Delete account Email Sending suppression](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/suppressions/methods/delete)

DELETE/accounts/{account\_id}/email/sending/suppressions/{suppression\_id}

##### [Bulk import account Email Sending suppressions](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/suppressions/methods/import)

POST/accounts/{account\_id}/email/sending/suppressions/bulk

##### ModelsExpand Collapse 

SuppressionListResponse object { id, created\_at, email, 4 more } 

id: string

formatuuid

created\_at: string

formatdate-time

email: string

formatemail

expires\_at: string

formatdate-time

read\_only: boolean

Whether clients may mutate this suppression. This is determined by the server and must not be inferred from `reason`.

reason: string

note: optional string

SuppressionGetResponse object { id, created\_at, email, 4 more } 

id: string

formatuuid

created\_at: string

formatdate-time

email: string

formatemail

expires\_at: string

formatdate-time

read\_only: boolean

Whether clients may mutate this suppression. This is determined by the server and must not be inferred from `reason`.

reason: string

note: optional string

SuppressionCreateResponse object { id } 

id: string

formatuuid

SuppressionEditResponse object { id, created\_at, email, 4 more } 

id: string

formatuuid

created\_at: string

formatdate-time

email: string

formatemail

expires\_at: string

formatdate-time

read\_only: boolean

Whether clients may mutate this suppression. This is determined by the server and must not be inferred from `reason`.

reason: string

note: optional string

SuppressionDeleteResponse object { id } 

id: string

formatuuid

SuppressionImportResponse object { deduplicated, errors, invalid, 4 more } 

deduplicated: number

errors: number

invalid: number

items: array of object { index, status, id, 2 more } 

index: number

status: "processed" or "invalid" or "error" or "skipped"

One of the following:

"processed"

"invalid"

"error"

"skipped"

id: optional string

formatuuid

email: optional string

formatemail

error: optional string

processed: number

skipped: number

total: number

#### Email SendingSubdomains

##### [List sending subdomains](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/subdomains/methods/list)

GET/zones/{zone\_id}/email/sending/subdomains

##### [Get a sending subdomain](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/subdomains/methods/get)

GET/zones/{zone\_id}/email/sending/subdomains/{subdomain\_id}

##### [Create a sending subdomain](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/subdomains/methods/create)

POST/zones/{zone\_id}/email/sending/subdomains

##### [Update a sending subdomain](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/subdomains/methods/edit)

PATCH/zones/{zone\_id}/email/sending/subdomains/{subdomain\_id}

##### [Delete a sending subdomain](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/subdomains/methods/delete)

DELETE/zones/{zone\_id}/email/sending/subdomains/{subdomain\_id}

##### ModelsExpand Collapse 

SubdomainListResponse object { enabled, name, tag, 6 more } 

enabled: boolean

Whether Email Sending is enabled on this subdomain.

name: string

The exact domain name or a leftmost wildcard such as `*.example.com`.

tag: string

Sending subdomain identifier.

maxLength32

created: optional string

The date and time the destination address has been created.

formatdate-time

dkim\_selector: optional string

The DKIM selector used for email signing. Wildcard rows publish the selector and sign with `d=<base>`.

drop\_suppressed\_recipients: optional boolean

Whether a send request that includes a recipient suppressed on this subdomain drops that recipient and still delivers to the rest, instead of failing the entire request.

modified: optional string

The date and time the destination address was last modified.

formatdate-time

preview\_enabled: optional boolean

Whether sent messages from this subdomain can be previewed in the activity log.

return\_path\_domain: optional string

The return-path domain used for bounce handling. Wildcard rows use `cf-bounce.<base>`.

SubdomainGetResponse object { enabled, name, tag, 6 more } 

enabled: boolean

Whether Email Sending is enabled on this subdomain.

name: string

The exact domain name or a leftmost wildcard such as `*.example.com`.

tag: string

Sending subdomain identifier.

maxLength32

created: optional string

The date and time the destination address has been created.

formatdate-time

dkim\_selector: optional string

The DKIM selector used for email signing. Wildcard rows publish the selector and sign with `d=<base>`.

drop\_suppressed\_recipients: optional boolean

Whether a send request that includes a recipient suppressed on this subdomain drops that recipient and still delivers to the rest, instead of failing the entire request.

modified: optional string

The date and time the destination address was last modified.

formatdate-time

preview\_enabled: optional boolean

Whether sent messages from this subdomain can be previewed in the activity log.

return\_path\_domain: optional string

The return-path domain used for bounce handling. Wildcard rows use `cf-bounce.<base>`.

SubdomainCreateResponse object { enabled, name, tag, 6 more } 

enabled: boolean

Whether Email Sending is enabled on this subdomain.

name: string

The exact domain name or a leftmost wildcard such as `*.example.com`.

tag: string

Sending subdomain identifier.

maxLength32

created: optional string

The date and time the destination address has been created.

formatdate-time

dkim\_selector: optional string

The DKIM selector used for email signing. Wildcard rows publish the selector and sign with `d=<base>`.

drop\_suppressed\_recipients: optional boolean

Whether a send request that includes a recipient suppressed on this subdomain drops that recipient and still delivers to the rest, instead of failing the entire request.

modified: optional string

The date and time the destination address was last modified.

formatdate-time

preview\_enabled: optional boolean

Whether sent messages from this subdomain can be previewed in the activity log.

return\_path\_domain: optional string

The return-path domain used for bounce handling. Wildcard rows use `cf-bounce.<base>`.

SubdomainEditResponse object { enabled, name, tag, 6 more } 

enabled: boolean

Whether Email Sending is enabled on this subdomain.

name: string

The exact domain name or a leftmost wildcard such as `*.example.com`.

tag: string

Sending subdomain identifier.

maxLength32

created: optional string

The date and time the destination address has been created.

formatdate-time

dkim\_selector: optional string

The DKIM selector used for email signing. Wildcard rows publish the selector and sign with `d=<base>`.

drop\_suppressed\_recipients: optional boolean

Whether a send request that includes a recipient suppressed on this subdomain drops that recipient and still delivers to the rest, instead of failing the entire request.

modified: optional string

The date and time the destination address was last modified.

formatdate-time

preview\_enabled: optional boolean

Whether sent messages from this subdomain can be previewed in the activity log.

return\_path\_domain: optional string

The return-path domain used for bounce handling. Wildcard rows use `cf-bounce.<base>`.

SubdomainDeleteResponse object { errors, messages, success } 

errors: array of object { code, message, documentation\_url, source } 

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

messages: array of object { code, message, documentation\_url, source } 

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

success: true

Whether the API call was successful.

#### Email SendingSubdomainsDNS

##### [Get sending subdomain DNS records](https://developers.cloudflare.com/api/resources/email%5Fsending/subresources/subdomains/subresources/dns/methods/get)

GET/zones/{zone\_id}/email/sending/subdomains/{subdomain\_id}/dns