# Credentials

## Update Token Configuration credentials

**put** `/zones/{zone_id}/token_validation/config/{config_id}/credentials`

Update Token Configuration credentials with full replacement semantics. Key identities (`{alg,kid}`) must be unique within the request. Symmetric keys (`kty: "oct"`) require `k`; `k: null` is invalid.

### Path Parameters

- `zone_id: string`

  Identifier.

- `config_id: string`

  UUID.

### Body Parameters

- `keys: array of object { alg, e, kid, 2 more }  or object { alg, crv, kid, 3 more }  or object { alg, crv, kid, 3 more }  or object { alg, k, kid, kty }`

  - `APIShieldCredentialsJWTKeyRSA object { alg, e, kid, 2 more }`

    JSON representation of an RSA key.

    - `alg: "RS256" or "RS384" or "RS512" or 3 more`

      Algorithm

      - `"RS256"`

      - `"RS384"`

      - `"RS512"`

      - `"PS256"`

      - `"PS384"`

      - `"PS512"`

    - `e: string`

      RSA exponent

    - `kid: string`

      Key ID

    - `kty: "RSA"`

      Key Type

      - `"RSA"`

    - `n: string`

      RSA modulus

  - `APIShieldCredentialsJWTKeyEcEs256 object { alg, crv, kid, 3 more }`

    JSON representation of an ES256 key

    - `alg: "ES256"`

      Algorithm

      - `"ES256"`

    - `crv: "P-256"`

      Curve

      - `"P-256"`

    - `kid: string`

      Key ID

    - `kty: "EC"`

      Key Type

      - `"EC"`

    - `x: string`

      X EC coordinate

    - `y: string`

      Y EC coordinate

  - `APIShieldCredentialsJWTKeyEcEs384 object { alg, crv, kid, 3 more }`

    JSON representation of an ES384 key

    - `alg: "ES384"`

      Algorithm

      - `"ES384"`

    - `crv: "P-384"`

      Curve

      - `"P-384"`

    - `kid: string`

      Key ID

    - `kty: "EC"`

      Key Type

      - `"EC"`

    - `x: string`

      X EC coordinate

    - `y: string`

      Y EC coordinate

  - `APIShieldCredentialsJWTKeyOctRequest object { alg, k, kid, kty }`

    JSON representation of a symmetric key for create/PUT requests.

    - `alg: "HS256" or "HS384" or "HS512"`

      Algorithm

      - `"HS256"`

      - `"HS384"`

      - `"HS512"`

    - `k: string`

      Symmetric key material. Required for create and PUT update requests.

    - `kid: string`

      Key ID

    - `kty: "oct"`

      Key Type

      - `"oct"`

### Returns

- `errors: Message`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `keys: array of object { alg, e, kid, 2 more }  or object { alg, crv, kid, 3 more }  or object { alg, crv, kid, 3 more }  or object { alg, kid, kty }`

  - `APIShieldCredentialsJWTKeyRSA object { alg, e, kid, 2 more }`

    JSON representation of an RSA key.

    - `alg: "RS256" or "RS384" or "RS512" or 3 more`

      Algorithm

      - `"RS256"`

      - `"RS384"`

      - `"RS512"`

      - `"PS256"`

      - `"PS384"`

      - `"PS512"`

    - `e: string`

      RSA exponent

    - `kid: string`

      Key ID

    - `kty: "RSA"`

      Key Type

      - `"RSA"`

    - `n: string`

      RSA modulus

  - `APIShieldCredentialsJWTKeyEcEs256 object { alg, crv, kid, 3 more }`

    JSON representation of an ES256 key

    - `alg: "ES256"`

      Algorithm

      - `"ES256"`

    - `crv: "P-256"`

      Curve

      - `"P-256"`

    - `kid: string`

      Key ID

    - `kty: "EC"`

      Key Type

      - `"EC"`

    - `x: string`

      X EC coordinate

    - `y: string`

      Y EC coordinate

  - `APIShieldCredentialsJWTKeyEcEs384 object { alg, crv, kid, 3 more }`

    JSON representation of an ES384 key

    - `alg: "ES384"`

      Algorithm

      - `"ES384"`

    - `crv: "P-384"`

      Curve

      - `"P-384"`

    - `kid: string`

      Key ID

    - `kty: "EC"`

      Key Type

      - `"EC"`

    - `x: string`

      X EC coordinate

    - `y: string`

      Y EC coordinate

  - `APIShieldCredentialsJWTKeyOctResponse object { alg, kid, kty }`

    JSON representation of a symmetric verification key in API responses (secret material is redacted).

    - `alg: "HS256" or "HS384" or "HS512"`

      Algorithm

      - `"HS256"`

      - `"HS384"`

      - `"HS512"`

    - `kid: string`

      Key ID

    - `kty: "oct"`

      Key Type

      - `"oct"`

- `messages: Message`

- `success: true`

  Whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/token_validation/config/$CONFIG_ID/credentials \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "keys": [
            {
              "alg": "RS256",
              "e": "e",
              "kid": "kid",
              "kty": "RSA",
              "n": "n"
            }
          ]
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
  "keys": [
    {
      "alg": "ES256",
      "crv": "P-256",
      "kid": "38013f13-c266-4eec-a72a-92ec92779f21",
      "kty": "EC",
      "x": "KN53JRwN3wCjm2o39bvZUX2VdrsHzS8pxOAGjm8m7EQ",
      "y": "lnkkzIxaveggz-HFhcMWW15nxvOj0Z_uQsXbpK0GFcY"
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
  "success": true
}
```

## Domain Types

### Credential Update Response

- `CredentialUpdateResponse object { errors, keys, messages, success }`

  - `errors: Message`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `keys: array of object { alg, e, kid, 2 more }  or object { alg, crv, kid, 3 more }  or object { alg, crv, kid, 3 more }  or object { alg, kid, kty }`

    - `APIShieldCredentialsJWTKeyRSA object { alg, e, kid, 2 more }`

      JSON representation of an RSA key.

      - `alg: "RS256" or "RS384" or "RS512" or 3 more`

        Algorithm

        - `"RS256"`

        - `"RS384"`

        - `"RS512"`

        - `"PS256"`

        - `"PS384"`

        - `"PS512"`

      - `e: string`

        RSA exponent

      - `kid: string`

        Key ID

      - `kty: "RSA"`

        Key Type

        - `"RSA"`

      - `n: string`

        RSA modulus

    - `APIShieldCredentialsJWTKeyEcEs256 object { alg, crv, kid, 3 more }`

      JSON representation of an ES256 key

      - `alg: "ES256"`

        Algorithm

        - `"ES256"`

      - `crv: "P-256"`

        Curve

        - `"P-256"`

      - `kid: string`

        Key ID

      - `kty: "EC"`

        Key Type

        - `"EC"`

      - `x: string`

        X EC coordinate

      - `y: string`

        Y EC coordinate

    - `APIShieldCredentialsJWTKeyEcEs384 object { alg, crv, kid, 3 more }`

      JSON representation of an ES384 key

      - `alg: "ES384"`

        Algorithm

        - `"ES384"`

      - `crv: "P-384"`

        Curve

        - `"P-384"`

      - `kid: string`

        Key ID

      - `kty: "EC"`

        Key Type

        - `"EC"`

      - `x: string`

        X EC coordinate

      - `y: string`

        Y EC coordinate

    - `APIShieldCredentialsJWTKeyOctResponse object { alg, kid, kty }`

      JSON representation of a symmetric verification key in API responses (secret material is redacted).

      - `alg: "HS256" or "HS384" or "HS512"`

        Algorithm

        - `"HS256"`

        - `"HS384"`

        - `"HS512"`

      - `kid: string`

        Key ID

      - `kty: "oct"`

        Key Type

        - `"oct"`

  - `messages: Message`

  - `success: true`

    Whether the API call was successful.

    - `true`
