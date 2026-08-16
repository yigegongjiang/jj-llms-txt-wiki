# Interconnects

## List existing interconnects

**get** `/accounts/{account_id}/cni/interconnects`

Lists all network interconnects configured for the account, including physical and virtual
connections.

### Path Parameters

- `account_id: string`

  Customer account tag

### Query Parameters

- `cursor: optional number`

- `limit: optional number`

- `site: optional string`

  If specified, only show interconnects located at the given site

- `type: optional string`

  If specified, only show interconnects of the given type

### Returns

- `items: array of object { account, facility, name, 7 more }  or object { account, name, region, 4 more }`

  - `NscInterconnectPhysicalBody object { account, facility, name, 7 more }`

    - `account: string`

    - `facility: object { address, name }`

      - `address: array of string`

      - `name: string`

    - `name: string`

    - `site: string`

      A Cloudflare site name.

    - `slot_id: string`

    - `speed: string`

    - `type: string`

    - `virtual_port_reservation_id: string`

    - `ccr_device_name: optional string`

    - `owner: optional string`

  - `NscInterconnectGcpPartnerBody object { account, name, region, 4 more }`

    - `account: string`

    - `name: string`

    - `region: string`

    - `type: string`

    - `virtual_port_reservation_id: string`

    - `owner: optional string`

    - `speed: optional "50M" or "100M" or "200M" or 9 more`

      Bandwidth structure as visible through the customer-facing API.

      - `"50M"`

      - `"100M"`

      - `"200M"`

      - `"300M"`

      - `"400M"`

      - `"500M"`

      - `"1G"`

      - `"2G"`

      - `"5G"`

      - `"10G"`

      - `"20G"`

      - `"50G"`

- `next: optional number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": [
    {
      "account": "account",
      "facility": {
        "address": [
          "string"
        ],
        "name": "name"
      },
      "name": "name",
      "site": "site",
      "slot_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
      "speed": "speed",
      "type": "type",
      "virtual_port_reservation_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
      "ccr_device_name": "ccr_device_name",
      "owner": "owner"
    }
  ],
  "next": 0
}
```

## Get information about an interconnect object

**get** `/accounts/{account_id}/cni/interconnects/{icon}`

Retrieves configuration and status details for a specific network interconnect.

### Path Parameters

- `account_id: string`

  Customer account tag

- `icon: string`

### Returns

- `NscInterconnectPhysicalBody object { account, facility, name, 7 more }`

  - `account: string`

  - `facility: object { address, name }`

    - `address: array of string`

    - `name: string`

  - `name: string`

  - `site: string`

    A Cloudflare site name.

  - `slot_id: string`

  - `speed: string`

  - `type: string`

  - `virtual_port_reservation_id: string`

  - `ccr_device_name: optional string`

  - `owner: optional string`

- `NscInterconnectGcpPartnerBody object { account, name, region, 4 more }`

  - `account: string`

  - `name: string`

  - `region: string`

  - `type: string`

  - `virtual_port_reservation_id: string`

  - `owner: optional string`

  - `speed: optional "50M" or "100M" or "200M" or 9 more`

    Bandwidth structure as visible through the customer-facing API.

    - `"50M"`

    - `"100M"`

    - `"200M"`

    - `"300M"`

    - `"400M"`

    - `"500M"`

    - `"1G"`

    - `"2G"`

    - `"5G"`

    - `"10G"`

    - `"20G"`

    - `"50G"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects/$ICON \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "account": "account",
  "facility": {
    "address": [
      "string"
    ],
    "name": "name"
  },
  "name": "name",
  "site": "site",
  "slot_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
  "speed": "speed",
  "type": "type",
  "virtual_port_reservation_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
  "ccr_device_name": "ccr_device_name",
  "owner": "owner"
}
```

## Create a new interconnect

**post** `/accounts/{account_id}/cni/interconnects`

Creates a new network interconnect for connecting Cloudflare's network to external networks.
Interconnects provide dedicated bandwidth and reduced latency for traffic exchange.

### Path Parameters

- `account_id: string`

  Customer account tag

### Body Parameters

- `body: object { account, slot_id, type, speed }  or object { account, bandwidth, pairing_key, type }`

  - `NscInterconnectCreatePhysicalBody object { account, slot_id, type, speed }`

    - `account: string`

    - `slot_id: string`

    - `type: string`

    - `speed: optional string`

  - `NscInterconnectCreateGcpPartnerBody object { account, bandwidth, pairing_key, type }`

    - `account: string`

    - `bandwidth: "50M" or "100M" or "200M" or 9 more`

      Bandwidth structure as visible through the customer-facing API.

      - `"50M"`

      - `"100M"`

      - `"200M"`

      - `"300M"`

      - `"400M"`

      - `"500M"`

      - `"1G"`

      - `"2G"`

      - `"5G"`

      - `"10G"`

      - `"20G"`

      - `"50G"`

    - `pairing_key: string`

      Pairing key provided by GCP

    - `type: string`

### Returns

- `NscInterconnectPhysicalBody object { account, facility, name, 7 more }`

  - `account: string`

  - `facility: object { address, name }`

    - `address: array of string`

    - `name: string`

  - `name: string`

  - `site: string`

    A Cloudflare site name.

  - `slot_id: string`

  - `speed: string`

  - `type: string`

  - `virtual_port_reservation_id: string`

  - `ccr_device_name: optional string`

  - `owner: optional string`

- `NscInterconnectGcpPartnerBody object { account, name, region, 4 more }`

  - `account: string`

  - `name: string`

  - `region: string`

  - `type: string`

  - `virtual_port_reservation_id: string`

  - `owner: optional string`

  - `speed: optional "50M" or "100M" or "200M" or 9 more`

    Bandwidth structure as visible through the customer-facing API.

    - `"50M"`

    - `"100M"`

    - `"200M"`

    - `"300M"`

    - `"400M"`

    - `"500M"`

    - `"1G"`

    - `"2G"`

    - `"5G"`

    - `"10G"`

    - `"20G"`

    - `"50G"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "account": "account",
          "slot_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
          "type": "type"
        }'
```

#### Response

```json
{
  "account": "account",
  "facility": {
    "address": [
      "string"
    ],
    "name": "name"
  },
  "name": "name",
  "site": "site",
  "slot_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
  "speed": "speed",
  "type": "type",
  "virtual_port_reservation_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
  "ccr_device_name": "ccr_device_name",
  "owner": "owner"
}
```

## Delete an interconnect object

**delete** `/accounts/{account_id}/cni/interconnects/{icon}`

Permanently removes a network interconnect configuration. The physical or virtual connection
will be terminated.

### Path Parameters

- `account_id: string`

  Customer account tag

- `icon: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects/$ICON \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

## Generate the Letter of Authorization (LOA) for a given interconnect

**get** `/accounts/{account_id}/cni/interconnects/{icon}/loa`

Downloads the Letter of Authorization (LOA) for a network interconnect, required for
physical cross-connect provisioning.

### Path Parameters

- `account_id: string`

  Customer account tag

- `icon: string`

### Query Parameters

- `name: optional string`

  Custom name to use in the LOA instead of the account name (200 Character limit)

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects/$ICON/loa \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

## Get the current status of an interconnect object

**get** `/accounts/{account_id}/cni/interconnects/{icon}/status`

Gets the current operational status of a network interconnect, including link state and
traffic metrics.

### Path Parameters

- `account_id: string`

  Customer account tag

- `icon: string`

### Returns

- `Pending object { state }`

  - `state: "Pending"`

    - `"Pending"`

- `Down object { state, reason }`

  - `state: "Down"`

    - `"Down"`

  - `reason: optional string`

    Diagnostic information, if available

- `Unhealthy object { state, reason }`

  - `state: "Unhealthy"`

    - `"Unhealthy"`

  - `reason: optional string`

    Diagnostic information, if available

- `Healthy object { state }`

  - `state: "Healthy"`

    - `"Healthy"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/interconnects/$ICON/status \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "state": "Pending"
}
```

## Domain Types

### Interconnect List Response

- `InterconnectListResponse object { items, next }`

  - `items: array of object { account, facility, name, 7 more }  or object { account, name, region, 4 more }`

    - `NscInterconnectPhysicalBody object { account, facility, name, 7 more }`

      - `account: string`

      - `facility: object { address, name }`

        - `address: array of string`

        - `name: string`

      - `name: string`

      - `site: string`

        A Cloudflare site name.

      - `slot_id: string`

      - `speed: string`

      - `type: string`

      - `virtual_port_reservation_id: string`

      - `ccr_device_name: optional string`

      - `owner: optional string`

    - `NscInterconnectGcpPartnerBody object { account, name, region, 4 more }`

      - `account: string`

      - `name: string`

      - `region: string`

      - `type: string`

      - `virtual_port_reservation_id: string`

      - `owner: optional string`

      - `speed: optional "50M" or "100M" or "200M" or 9 more`

        Bandwidth structure as visible through the customer-facing API.

        - `"50M"`

        - `"100M"`

        - `"200M"`

        - `"300M"`

        - `"400M"`

        - `"500M"`

        - `"1G"`

        - `"2G"`

        - `"5G"`

        - `"10G"`

        - `"20G"`

        - `"50G"`

  - `next: optional number`

### Interconnect Get Response

- `InterconnectGetResponse = object { account, facility, name, 7 more }  or object { account, name, region, 4 more }`

  - `NscInterconnectPhysicalBody object { account, facility, name, 7 more }`

    - `account: string`

    - `facility: object { address, name }`

      - `address: array of string`

      - `name: string`

    - `name: string`

    - `site: string`

      A Cloudflare site name.

    - `slot_id: string`

    - `speed: string`

    - `type: string`

    - `virtual_port_reservation_id: string`

    - `ccr_device_name: optional string`

    - `owner: optional string`

  - `NscInterconnectGcpPartnerBody object { account, name, region, 4 more }`

    - `account: string`

    - `name: string`

    - `region: string`

    - `type: string`

    - `virtual_port_reservation_id: string`

    - `owner: optional string`

    - `speed: optional "50M" or "100M" or "200M" or 9 more`

      Bandwidth structure as visible through the customer-facing API.

      - `"50M"`

      - `"100M"`

      - `"200M"`

      - `"300M"`

      - `"400M"`

      - `"500M"`

      - `"1G"`

      - `"2G"`

      - `"5G"`

      - `"10G"`

      - `"20G"`

      - `"50G"`

### Interconnect Create Response

- `InterconnectCreateResponse = object { account, facility, name, 7 more }  or object { account, name, region, 4 more }`

  - `NscInterconnectPhysicalBody object { account, facility, name, 7 more }`

    - `account: string`

    - `facility: object { address, name }`

      - `address: array of string`

      - `name: string`

    - `name: string`

    - `site: string`

      A Cloudflare site name.

    - `slot_id: string`

    - `speed: string`

    - `type: string`

    - `virtual_port_reservation_id: string`

    - `ccr_device_name: optional string`

    - `owner: optional string`

  - `NscInterconnectGcpPartnerBody object { account, name, region, 4 more }`

    - `account: string`

    - `name: string`

    - `region: string`

    - `type: string`

    - `virtual_port_reservation_id: string`

    - `owner: optional string`

    - `speed: optional "50M" or "100M" or "200M" or 9 more`

      Bandwidth structure as visible through the customer-facing API.

      - `"50M"`

      - `"100M"`

      - `"200M"`

      - `"300M"`

      - `"400M"`

      - `"500M"`

      - `"1G"`

      - `"2G"`

      - `"5G"`

      - `"10G"`

      - `"20G"`

      - `"50G"`

### Interconnect Status Response

- `InterconnectStatusResponse = object { state }  or object { state, reason }  or object { state, reason }  or object { state }`

  - `Pending object { state }`

    - `state: "Pending"`

      - `"Pending"`

  - `Down object { state, reason }`

    - `state: "Down"`

      - `"Down"`

    - `reason: optional string`

      Diagnostic information, if available

  - `Unhealthy object { state, reason }`

    - `state: "Unhealthy"`

      - `"Unhealthy"`

    - `reason: optional string`

      Diagnostic information, if available

  - `Healthy object { state }`

    - `state: "Healthy"`

      - `"Healthy"`
