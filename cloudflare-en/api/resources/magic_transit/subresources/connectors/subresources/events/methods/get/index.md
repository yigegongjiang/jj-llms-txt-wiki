## Get Event

**get** `/accounts/{account_id}/magic/connectors/{connector_id}/telemetry/events/{event_t}.{event_n}`

Gets Magic WAN Connector Telemetry Event

### Path Parameters

- `account_id: string`

  Account identifier

- `connector_id: string`

- `event_t: number`

- `event_n: number`

### Returns

- `result: object { e, n, t, v }`

  Recorded Event

  - `e: object { k }`

    Event kind plus event-specific payload fields.

    Event kinds:

    - `Init`: Initialized process
    - `Leave`: Stopped process
    - `StartAttestation`: Started attestation
    - `FinishAttestationSuccess`: Finished attestation
    - `FinishAttestationFailure`: Failed attestation
    - `StartRotateCryptKey`: Started crypt key rotation
    - `FinishRotateCryptKeySuccess`: Finished crypt key rotation
    - `FinishRotateCryptKeyFailure`: Failed crypt key rotation
    - `StartRotatePki`: Started PKI rotation
    - `FinishRotatePkiSuccess`: Finished PKI rotation
    - `FinishRotatePkiFailure`: Failed PKI rotation
    - `StartUpgrade`: Started upgrade
    - `FinishUpgradeSuccess`: Finished upgrade
    - `FinishUpgradeFailure`: Failed upgrade
    - `BlessSlotSuccess`: Blessed boot entry slot
    - `BlessSlotPending`: Boot entry slot is not yet blessed
    - `BlessSlotFailure`: Failed to bless boot entry slot
    - `Reconcile`: Reconciled
    - `ConfigureCloudflaredTunnel`: Configured Cloudflared tunnel
    - `RekeyInstallBoth`: Installed initial inbound and outbound keys
    - `RekeyStart`: Installed new inbound key, kept old outbound
    - `RekeyRestart`: Restarted in-progress rekey with newer key material
    - `RekeyAdvance`: Confirmed traffic on new inbound key, swapped outbound to new
    - `RekeyComplete`: Deleted old keys
    - `RekeyReset`: Deleted all keys after receiving an unexpected key
    - `HaTransition`: Completed HA state transition
    - `HaError`: Received unexpected HA error
    - `HaInit`: Initialized HA subsystem
    - `HaLeave`: Stopped HA subsystem

    - `k: "Init" or "Leave" or "StartAttestation" or 26 more`

      Event kind

      - `"Init"`

      - `"Leave"`

      - `"StartAttestation"`

      - `"FinishAttestationSuccess"`

      - `"FinishAttestationFailure"`

      - `"StartRotateCryptKey"`

      - `"FinishRotateCryptKeySuccess"`

      - `"FinishRotateCryptKeyFailure"`

      - `"StartRotatePki"`

      - `"FinishRotatePkiSuccess"`

      - `"FinishRotatePkiFailure"`

      - `"StartUpgrade"`

      - `"FinishUpgradeSuccess"`

      - `"FinishUpgradeFailure"`

      - `"BlessSlotSuccess"`

      - `"BlessSlotPending"`

      - `"BlessSlotFailure"`

      - `"Reconcile"`

      - `"ConfigureCloudflaredTunnel"`

      - `"RekeyInstallBoth"`

      - `"RekeyStart"`

      - `"RekeyRestart"`

      - `"RekeyAdvance"`

      - `"RekeyComplete"`

      - `"RekeyReset"`

      - `"HaTransition"`

      - `"HaError"`

      - `"HaInit"`

      - `"HaLeave"`

  - `n: number`

    Sequence number, used to order events with the same timestamp

  - `t: number`

    Time the Event was recorded (seconds since the Unix epoch)

  - `v: optional string`

    Version

- `success: boolean`

- `errors: optional array of object { code, message }`

  - `code: number`

  - `message: string`

- `messages: optional array of object { code, message }`

  - `code: number`

  - `message: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/magic/connectors/$CONNECTOR_ID/telemetry/events/$EVENT_T.$EVENT_N \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "e": {
      "k": "Init"
    },
    "n": 0,
    "t": 0,
    "v": "v"
  },
  "success": true,
  "errors": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "messages": [
    {
      "code": 0,
      "message": "message"
    }
  ]
}
```
