## List DNSSEC ZSKs

**get** `/zones/{zone_id}/dnssec/zsk`

List the Zone Signing Keys (ZSKs) that DNSSEC uses for the zone.

### Path Parameters

- `zone_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of object { DNSKEY, Location, Name, 2 more }`

  - `DNSKEY: optional object { Algorithm, Flags, Hdr, 2 more }`

    - `Algorithm: optional number`

    - `Flags: optional number`

    - `Hdr: optional object { Class, Name, Rdlength, 2 more }`

      - `Class: optional number`

      - `Name: optional string`

      - `Rdlength: optional number`

      - `Rrtype: optional number`

      - `Ttl: optional number`

    - `Protocol: optional number`

    - `PublicKey: optional string`

  - `Location: optional "database" or "vault"`

    Storage backend where the DNSSEC key material is stored.

    - `"database"`

    - `"vault"`

  - `Name: optional string`

    Internal key name for the ZSK.

  - `SigningKey: optional object { kek, privkey, pubkey }`

    - `kek: optional string`

      Key encryption key name used to encrypt the private key.

    - `privkey: optional string`

      Encrypted private key material for the signing key.

    - `pubkey: optional string`

      Public key content associated with the signing key.

  - `Tag: optional "active" or "publish" or "external" or 3 more`

    Lifecycle state tag attached to the DNSSEC key.

    - `"active"`

    - `"publish"`

    - `"external"`

    - `"retired"`

    - `"revoked"`

    - `"removed"`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/dnssec/zsk \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
  "success": true,
  "result": [
    {
      "DNSKEY": {
        "Algorithm": 13,
        "Flags": 256,
        "Hdr": {
          "Class": 1,
          "Name": "example.com.",
          "Rdlength": 0,
          "Rrtype": 48,
          "Ttl": 3600
        },
        "Protocol": 3,
        "PublicKey": "oXiGYrSTO+LSCJ3mohc8EP+CzF9KxBj8/ydXJ22pKuZP3VAC3/Md/k7xZfz470CoRyZJ6gV6vml07IC3d8xqhA=="
      },
      "Location": "database",
      "Name": "zsk_default",
      "SigningKey": {
        "kek": "edge_kek_default",
        "privkey": "U3ZlbidzIHZlcnkgc2VjcmV0IGtleQ==",
        "pubkey": "256 3 13 oXiGYrSTO+LSCJ3mohc8EP+CzF9KxBj8/ydXJ22pKuZP3VAC3/Md/k7xZfz470CoRyZJ6gV6vml07IC3d8xqhA=="
      },
      "Tag": "active"
    }
  ]
}
```
