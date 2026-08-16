## Delete a share recipient

**delete** `/accounts/{account_id}/shares/{share_id}/recipients/{recipient_id}`

Performs a **soft delete**: sets the recipient's
`desired_association_status` to `disassociated`, which signals the
background reconciliation workflow (Temporal) to remove the shared
resources from the recipient account. The recipient record remains in
the database for audit purposes and is still returned by
`GET /accounts/{account_id}/shares/{share_id}/recipients` with its
updated status.

Resource access is not fully removed until the workflow completes and
`current_association_status` transitions to `disassociated`. The
recipient record itself is never physically deleted.

### Path Parameters

- `account_id: string`

  Account identifier.

- `share_id: string`

  Share identifier tag.

- `recipient_id: string`

  Share Recipient identifier tag.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/shares/$SHARE_ID/recipients/$RECIPIENT_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
