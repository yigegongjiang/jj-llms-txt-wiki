## Update multiple interconnects

**put** `/accounts/{account_id}/magic/cf_interconnects`

Updates multiple interconnects associated with an account. Use `?validate_only=true` as an optional query parameter to only run validation without persisting changes.

### Path Parameters

- `account_id: string`

  Identifier

### Header Parameters

- `"x-magic-new-hc-target": optional boolean`

### Body Parameters

- `body: unknown`

### Returns

- `errors: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

- `result: object { modified, modified_interconnects }`

  - `modified: optional boolean`

  - `modified_interconnects: optional array of object { id, automatic_return_routing, colo_name, 11 more }`

    - `id: optional string`

      Identifier

    - `automatic_return_routing: optional boolean`

      True if automatic stateful return routing should be enabled for a tunnel, false otherwise. Requires the `coupler_integration` account flag to be enabled; requests setting this to `true` without that flag will be rejected.

    - `colo_name: optional string`

      The name of the interconnect. The name cannot share a name with other tunnels.

    - `created_on: optional string`

      The date and time the tunnel was created.

    - `description: optional string`

      An optional description of the interconnect.

    - `gre: optional object { cloudflare_endpoint }`

      The configuration specific to GRE interconnects.

      - `cloudflare_endpoint: optional string`

        The IP address assigned to the Cloudflare side of the GRE tunnel created as part of the Interconnect.

    - `health_check: optional HealthCheck`

      - `enabled: optional boolean`

        Determines whether to run healthchecks for a tunnel.

      - `rate: optional HealthCheckRate`

        How frequent the health check is run. The default value is `mid`.

        - `"low"`

        - `"mid"`

        - `"high"`

      - `target: optional object { effective, saved }  or string`

        The destination address in a request type health check. After the healthcheck is decapsulated at the customer end of the tunnel, the ICMP echo will be forwarded to this address. This field defaults to `customer_gre_endpoint address`. This field is ignored for bidirectional healthchecks as the interface_address (not assigned to the Cloudflare side of the tunnel) is used as the target. Must be in object form if the x-magic-new-hc-target header is set to true and string form if x-magic-new-hc-target is absent or set to false.

        - `MagicHealthCheckTarget object { effective, saved }`

          The destination address in a request type health check. After the healthcheck is decapsulated at the customer end of the tunnel, the ICMP echo will be forwarded to this address. This field defaults to `customer_gre_endpoint address`. This field is ignored for bidirectional healthchecks as the interface_address (not assigned to the Cloudflare side of the tunnel) is used as the target.

          - `effective: optional string`

            The effective health check target. If 'saved' is empty, then this field will be populated with the calculated default value on GET requests. Ignored in POST, PUT, and PATCH requests.

          - `saved: optional string`

            The saved health check target. Setting the value to the empty string indicates that the calculated default value will be used.

        - `string`

      - `type: optional HealthCheckType`

        The type of healthcheck to run, reply or request. The default value is `reply`.

        - `"reply"`

        - `"request"`

    - `interface_address: optional string`

      The IPv4 interface address for the interconnect. For MPLS Interconnects,
      use a /30 or /31 prefix. For GRE Interconnects, a /29, /30, or /31 prefix
      may be used. A /29 prefix is only allowed for v1.5 interconnects,
      and the address must be the .3 host of the subnet (the fourth address
      overall; the network address is not usable). Select the subnet from RFC 1918
      or the approved link-local ranges.

    - `interface_address6: optional string`

      A 127 bit IPV6 prefix from within the virtual_subnet6 prefix space with the address being the first IP of the subnet and not same as the address of virtual_subnet6. Eg if virtual_subnet6 is 2606:54c1:7:0:a9fe:12d2::/127 , interface_address6 could be 2606:54c1:7:0:a9fe:12d2:1:200/127

    - `modified_on: optional string`

      The date and time the tunnel was last modified.

    - `mtu: optional number`

      The Maximum Transmission Unit (MTU) in bytes for the interconnect. The minimum value is 576.

    - `name: optional string`

      The name of the interconnect. The name cannot share a name with other tunnels.

    - `version: optional string`

      Immutable interconnect version configured at creation time. One of:

      - "1"
      - "1.5"
      - "2"

    - `virtual_port_reservation_id: optional string`

      An identifier that correlates this interconnect with the corresponding V2 CNI interconnect resource.

- `success: true`

  Whether the API call was successful

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/magic/cf_interconnects \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{}'
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
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "result": {
    "modified": true,
    "modified_interconnects": [
      {
        "id": "c4a7362d577a6c3019a474fd6f485821",
        "automatic_return_routing": true,
        "colo_name": "pni_ord",
        "created_on": "2017-06-14T00:00:00Z",
        "description": "Tunnel for Interconnect to ORD",
        "gre": {
          "cloudflare_endpoint": "203.0.113.1"
        },
        "health_check": {
          "enabled": true,
          "rate": "low",
          "target": {
            "effective": "203.0.113.1",
            "saved": "203.0.113.1"
          },
          "type": "request"
        },
        "interface_address": "192.0.2.3/29",
        "interface_address6": "2606:54c1:7:0:a9fe:12d2:1:200/127",
        "modified_on": "2017-06-14T05:20:00Z",
        "mtu": 0,
        "name": "pni_ord",
        "version": "1.5",
        "virtual_port_reservation_id": "c4a7362d577a6c3019a474fd6f485821"
      }
    ]
  },
  "success": true
}
```
