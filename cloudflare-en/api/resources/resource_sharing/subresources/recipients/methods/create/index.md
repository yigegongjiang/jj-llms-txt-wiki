## Create a new share recipient

**post** `/accounts/{account_id}/shares/{share_id}/recipients`

Adds a single recipient to an account-targeted resource share, granting
them access to the shared resources. The recipient account must belong
to the same organization as the share owner.

To replace the entire recipient list in one call, use
`PUT /accounts/{account_id}/shares/{share_id}/recipients` instead.

### Path Parameters

- `account_id: string`

  Account identifier.

- `share_id: string`

  Share identifier tag.

### Body Parameters

- `account_id: optional string`

  Deprecated alias for `recipient_account_id`. Use `recipient_account_id` instead.
  The body field collided with the URL path parameter of the same name, which prevented SDK generators from distinguishing the source account (in the URL) from the recipient account (in the body). Both names will continue to be accepted until 2027-05-26 (see `x-sunset`).

- `organization_id: optional string`

  Organization identifier.

- `recipient_account_id: optional string`

  The account that will receive the share.

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, account_id, association_status, 3 more }`

  A recipient of a share. The `association_status` field tracks the
  lifecycle of the shared resources in the recipient account. All
  recipients are returned by the list endpoint regardless of status;
  filter client-side if only active recipients are needed.

  - `id: string`

    Share Recipient identifier tag.

  - `account_id: string`

    Account identifier.

  - `association_status: "associating" or "associated" or "disassociating" or "disassociated"`

    The current state of the recipient relative to the share. The
    `desired_association_status` (not exposed in the response) tracks the
    target state set by the API; the background reconciliation workflow
    drives `current_association_status` toward it.

    - `associating` — The recipient was recently added; the workflow is
      pushing shared resources into the recipient account.
    - `associated` — Shared resources have been successfully applied to
      the recipient account.
    - `disassociating` — The recipient was removed (via DELETE or PUT
      replacement); the workflow is removing shared resources from the
      recipient account.
    - `disassociated` — Shared resources have been removed from the
      recipient account. The recipient record remains in the database.

    - `"associating"`

    - `"associated"`

    - `"disassociating"`

    - `"disassociated"`

  - `created: string`

    When the share was created.

  - `modified: string`

    When the share was modified.

  - `resources: optional array of object { error, resource_id, resource_version, terminal }`

    - `error: string`

      Share Recipient error message.

    - `resource_id: string`

      Share Resource identifier.

    - `resource_version: number`

      Resource Version.

    - `terminal: boolean`

      Whether the error is terminal or will be continually retried.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/shares/$SHARE_ID/recipients \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "account_id": "023e105f4ecef8ad9ca31a8372d0c353",
          "organization_id": "023e105f4ecef8ad9ca31a8372d0c353",
          "recipient_account_id": "023e105f4ecef8ad9ca31a8372d0c353"
        }'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": {
    "id": "3fd85f74b32742f1bff64a85009dda07",
    "account_id": "023e105f4ecef8ad9ca31a8372d0c353",
    "association_status": "associating",
    "created": "2023-09-21T18:56:32.624632Z",
    "modified": "2023-09-21T18:56:32.624632Z",
    "resources": [
      {
        "error": "Recipient is missing necessary entitlement",
        "resource_id": "023e105f4ecef8ad9ca31a8372d0c353",
        "resource_version": 0,
        "terminal": true
      }
    ]
  }
}
```
