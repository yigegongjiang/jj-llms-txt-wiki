> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verificar la identidad de la sesión en entornos autohospedados

> Verifique el JWT CLAUDE_CODE_SESSION_ACCESS_TOKEN para que los servicios en su red puedan confiar en las solicitudes de sesiones en su entorno autohospedado.

<Note>
  Los entornos autohospedados están en versión beta pública en planes Team y Enterprise; un [Propietario](/docs/es/cloud-environments#organization-shared-environments) los habilita activando **Allow self-hosted environments** en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Esta página cubre la verificación de identidad de sesión; consulte el [inicio rápido](/docs/es/self-hosted-environments-quickstart) para la configuración y [Implementar en producción](/docs/es/self-hosted-environments-deploy) para las recetas de flota.
</Note>

Un [entorno autohospedado](/docs/es/self-hosted-environments) permite que las sesiones de [Claude Code en la web](/docs/es/claude-code-on-the-web) se ejecuten en la infraestructura que usted opera en lugar de en la de Anthropic. Debido a que la sesión se ejecuta dentro de su red, Claude puede llamar directamente a sus servicios internos. Esos servicios necesitan una forma de confirmar que una solicitud proviene de una sesión de Claude Code en su entorno e identificar la identidad del usuario o servicio que creó esa sesión.

Cada sesión en un entorno autohospedado recibe un JSON Web Token (JWT) firmado en la variable de entorno `CLAUDE_CODE_SESSION_ACCESS_TOKEN`. Una sesión presenta el token como cualquier credencial de portador; por ejemplo, un script que Claude ejecuta puede llamar a su servicio con `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"`. Anthropic firma el token y publica las claves de verificación en un punto final JWKS público. Sus servicios obtienen esas claves, verifican la firma y leen las reclamaciones para decidir qué acceso otorgar.

<h2 id="the-session-token">
  El token de sesión
</h2>

Antes de escribir código de verificación, sepa qué establece el token y la forma que verá su biblioteca JWT.

<h3 id="what-the-token-proves">
  Qué prueba el token
</h3>

Un token válido establece algunos hechos y deliberadamente no otros:

* **Prueba**: Anthropic emitió el token para una sesión específica en un entorno específico, y cómo se creó la sesión: por un usuario en su organización, o por la identidad de servicio de su organización, que es cómo comienzan las [sesiones del canal Claude Tag](https://claude.com/docs/claude-tag/concepts/agent-identity)
* **No prueba**: qué proceso en el host del ejecutor lo presenta. El token se encuentra en una variable de entorno dentro de la sesión, por lo que cualquier código que Claude ejecute y cualquier herramienta o servidor MCP que inicie la sesión puede leerlo y presentarlo.

Dos consecuencias para sus servicios:

* Verifique la reclamación `aud` contra su ID de entorno, el valor `ccpool_...` que se muestra con su entorno en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), para rechazar tokens emitidos a cualquier entorno de otra organización.
* Limite las credenciales que derive del token a lo que una única sesión de codificación debería poder hacer, no a todo lo que el creador de la sesión puede hacer. Consulte [Limitar credenciales derivadas](#scope-derived-credentials).

<h3 id="token-format">
  Formato del token
</h3>

El valor de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` tiene un prefijo `sk-ant-cc-` seguido de un JWT estándar de tres partes:

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

Elimine el prefijo antes de pasar el valor a una biblioteca JWT. Los tokens emitidos a sesiones en la nube alojadas por Anthropic llevan un prefijo `sk-ant-si-` en su lugar y están firmados por un conjunto de claves diferente, por lo que rechace cualquier valor que no comience con `sk-ant-cc-`.

El algoritmo de firma es `ES256`, que es ECDSA en la curva P-256 con SHA-256. El encabezado del token lleva un `kid` que identifica qué clave en el JWKS lo firmó.

<h2 id="verify-the-token">
  Verificar el token
</h2>

La verificación se ejecuta en uno de dos lugares. Los servicios en su red verifican el token criptográficamente contra las claves publicadas por Anthropic, y los scripts contenedores dentro de la sesión pueden usar el decodificador integrado del binario del ejecutor en su lugar.

<h3 id="verify-the-token-from-your-service">
  Verificar el token desde su servicio
</h3>

Anthropic publica las claves de verificación en un punto final público y sin autenticación:

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

La respuesta es un [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517) estándar. Anthropic rota las claves de firma periódicamente, y las claves anteriores a una rotación permanecen en el conjunto el tiempo suficiente para que los tokens que firmaron continúen verificándose, por lo que no fije una única clave. El punto final establece `Cache-Control: public, max-age=300`, por lo que almacenar en caché el conjunto de claves y volver a obtenerlo cada cinco minutos es seguro.

Verifique cada token entrante contra estas comprobaciones:

<Steps>
  <Step title="Verificar el prefijo">
    Rechace el valor si no comienza con `sk-ant-cc-`, luego elimine ese prefijo. El resto es un JWT compacto estándar.
  </Step>

  <Step title="Verificar la firma">
    Obtenga el JWKS, seleccione la clave cuyo `kid` coincida con el encabezado del token y verifique la firma `ES256`. Rechace los tokens cuyo encabezado `alg` no sea `ES256`. Si un token llega con un `kid` que no está en su conjunto de claves almacenado en caché, vuelva a obtener el JWKS una vez antes de rechazarlo: después de una rotación, los nuevos tokens se firman con una clave que su conjunto almacenado en caché aún no tiene.
  </Step>

  <Step title="Verificar el emisor">
    Rechace el token si `iss` no es exactamente `ccr`.
  </Step>

  <Step title="Verificar la audiencia contra su entorno">
    La reclamación `aud` es una matriz. Rechace el token a menos que contenga su ID de entorno, que tiene la forma `ccpool_...`. El ID de entorno se muestra en el cuadro de diálogo de detalles de su entorno en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), y aparece como la reclamación `ccr:pool_id` en cualquiera de los tokens de sesión del entorno. Esta comprobación es lo que limita el token a su entorno y rechaza los tokens emitidos a otras organizaciones.
  </Step>

  <Step title="Verificar el rol">
    Rechace el token si `ccr:role` no es exactamente `session_worker`. Otros tokens emitidos para entornos autohospedados, como secretos de entorno, tokens de ejecutor y órdenes de trabajo, se firman con el mismo conjunto de claves pero llevan roles diferentes.
  </Step>

  <Step title="Verificar la expiración">
    Rechace el token si `exp` está en el pasado. Anthropic emite tokens de sesión con una vida útil de cuatro horas por defecto y un máximo de ocho horas. El ejecutor actualiza el token antes de la expiración e inserta el nuevo valor en la sesión, por lo que los subprocesos que Claude inicia después de una actualización lo heredan. Una sesión puede, por lo tanto, presentar varios tokens válidos distintos a su servicio durante su vida útil.
  </Step>

  <Step title="Leer la identidad">
    La identidad del usuario creador está en la reclamación `act`: `act.sub` es su ID de usuario de Anthropic en la forma con prefijo `user:<id>`, y `act.email`, cuando la superficie creadora registró uno, es su dirección de correo electrónico. Las sesiones que crea la identidad de servicio de su organización, incluidas las sesiones del canal Claude Tag, llevan un asunto `agent:` en su lugar, por lo que trate una sesión como creada por el usuario solo cuando `act.sub` lleva el prefijo `user:`, en lugar de probar si las reclamaciones de identidad están ausentes. Consulte la [referencia de reclamaciones](#claims-reference) para la estructura completa y las reclamaciones duplicadas planas.
  </Step>
</Steps>

Las comprobaciones se asignan directamente a bibliotecas JWT estándar. Los ejemplos a continuación implementan la secuencia completa en Node.js con [`jose`](https://www.npmjs.com/package/jose), que maneja la obtención de JWKS, almacenamiento en caché y selección de `kid`, y en Python con [`PyJWT`](https://pyjwt.readthedocs.io/) y su cliente JWKS integrado.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  Verificar el token dentro de la sesión
</h3>

Los [scripts contenedores](/docs/es/self-hosted-environments-configuration#wrapper-scripts) se ejecutan dentro de la sesión, antes de que Claude comience. En lugar de llamar a una biblioteca JWT, pueden ejecutar el subcomando `self-hosted-runner decode-token` del binario del ejecutor. El subcomando lee el token de un argumento posicional, de `CLAUDE_CODE_SESSION_ACCESS_TOKEN`, o de stdin canalizado, en ese orden, luego elimina el prefijo, verifica la firma contra el punto final de JWKS, comprueba la expiración e imprime las reclamaciones como JSON. El subcomando realiza solo las comprobaciones de firma y expiración; no comprueba `iss`, `aud` o `ccr:role`. Cuando la decisión de autenticación de su contenedor depende de esas reclamaciones, léalas del JSON impreso y compárelas explícitamente.

Este comando extrae la identidad del creador, prefiriendo el asunto del proveedor de SSO, luego la dirección de correo electrónico, luego el asunto `act.sub` del creador, `user:<id>` o `agent:<id>`:

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

Los contenedores reciben la ruta absoluta al binario del ejecutor en `CLAUDE_RUNNER_CLAUDE_BIN`; use esa ruta en lugar de un `claude` resuelto por PATH para que la decodificación se ejecute en el mismo binario que usa el ejecutor.

Use `jq -re` en lugar de `jq -r` para que una reclamación faltante cause una salida distinta de cero. Con solo `-r`, una reclamación faltante imprime la cadena literal `null` y sale con cero, lo que silenciosamente pasa un valor incorrecto aguas abajo. Pase `--no-verify` a `decode-token` solo para inspección sin conexión donde el punto final de JWKS es inaccesible.

<h2 id="claims-reference">
  Referencia de reclamaciones
</h2>

La tabla a continuación enumera las reclamaciones de token de sesión relevantes para la verificación. Lea la identidad del espacio de nombres `ccr:*` y la cadena `act`; las reclamaciones planas `account_email`, `organization_uuid` y `account_uuid` son duplicados de compatibilidad hacia atrás que pueden eliminarse. Las sesiones que crea la identidad de servicio de su organización, incluidas las sesiones del canal Claude Tag, llevan un asunto `agent:` en `act.sub` y omiten `act.email`, `ccr:account_id`, `account_email` y `account_uuid`. Las dos reclamaciones de correo electrónico también son opcionales para sesiones creadas por el usuario: Anthropic las registra en la creación de la sesión solo cuando las credenciales de la solicitud creadora llevan un correo electrónico, y una sesión enviada desde la CLI puede carecer de ambas, por lo que base la identidad en `act.sub` o `ccr:account_id` en lugar de en el correo electrónico. Los tokens también pueden llevar reclamaciones adicionales más allá de esta tabla; ignore las reclamaciones que no reconozca.

| Reclamación         | Tipo              | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------ | :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | cadena            | Siempre `ccr`.                                                                                                                                                                                                                                                                                                                                                                                                               |
| `sub`               | cadena            | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `aud`               | matriz de cadenas | Siempre contiene `anthropic-api`. Para sesiones en entornos autohospedados, la matriz también contiene su ID de entorno, como `ccpool_...`. Verifique el ID de entorno, no `anthropic-api`.                                                                                                                                                                                                                                  |
| `exp`               | número            | Expiración como marca de tiempo de Unix. Vida útil predeterminada de cuatro horas, máximo de ocho horas.                                                                                                                                                                                                                                                                                                                     |
| `iat`               | número            | Emitido en como marca de tiempo de Unix.                                                                                                                                                                                                                                                                                                                                                                                     |
| `jti`               | cadena            | Identificador de token único.                                                                                                                                                                                                                                                                                                                                                                                                |
| `ccr:role`          | cadena            | Siempre `session_worker` para tokens de sesión.                                                                                                                                                                                                                                                                                                                                                                              |
| `ccr:session_id`    | cadena            | El ID de sesión. Mismo valor que el sufijo de `sub`.                                                                                                                                                                                                                                                                                                                                                                         |
| `ccr:pool_id`       | cadena            | Su ID de entorno. Mismo valor que aparece en `aud`.                                                                                                                                                                                                                                                                                                                                                                          |
| `ccr:org_id`        | cadena            | Su ID de organización de Anthropic.                                                                                                                                                                                                                                                                                                                                                                                          |
| `ccr:account_id`    | cadena            | El ID de cuenta de Anthropic del usuario creador: el valor de `act.sub` sin el prefijo `user:`, un ID etiquetado `user_...`. El mismo valor que el `CLAUDE_RUNNER_ACCOUNT_ID` del [hook spawn-runner](/docs/es/self-hosted-environments-configuration#the-spawn-runner-hook) lleva y [`--lock-to-account`](/docs/es/self-hosted-environments-reference#runner-cli-flags) acepta, por lo que los tres se comparan como cadenas iguales. |
| `account_email`     | cadena            | Duplicado de `act.email`; ausente siempre que `act.email` lo esté.                                                                                                                                                                                                                                                                                                                                                           |
| `organization_uuid` | cadena            | Su UUID de organización de Anthropic.                                                                                                                                                                                                                                                                                                                                                                                        |
| `account_uuid`      | cadena            | El UUID de cuenta de Anthropic del usuario creador.                                                                                                                                                                                                                                                                                                                                                                          |
| `act`               | objeto            | Cadena de delegación [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693). Consulte [La cadena `act`](#the-act-chain).                                                                                                                                                                                                                                                                                                         |

<h3 id="the-act-chain">
  La cadena `act`
</h3>

La reclamación `act` registra la ruta de delegación completa desde la identidad del usuario o servicio que creó la sesión hasta el [entorno](/docs/es/self-hosted-environments#key-concepts) cuyo secreto admitió al ejecutor, y la identidad que creó ese secreto. El creador es el actor más externo, por lo que `act.sub` los identifica directamente.

| Ruta              | Descripción                                                                                                                                                                                                                                                                       |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | El ID de usuario de Anthropic del usuario creador, en la forma `user:<id>`, o `agent:<id>` cuando la identidad de servicio de su organización creó la sesión, como lo hace para las sesiones del canal Claude Tag.                                                                |
| `act.email`       | La dirección de correo electrónico del usuario creador, cuando se registró una en la creación de la sesión. No la requiera; base en `act.sub`.                                                                                                                                    |
| `act.attested_by` | La atestación del proveedor de identidad ascendente para el usuario creador, cuando está disponible. `act.attested_by.sub` es el asunto que su proveedor de SSO, como Google u Okta, emitió. Prefiera esto sobre `act.email` cuando asigne a identidades en sus propios sistemas. |
| `act.act`         | El ejecutor que generó la sesión. `act.act.sub` es `ccr:runner:<runner_id>`.                                                                                                                                                                                                      |
| `act.act.act`     | El entorno. `act.act.act.sub` es `ccr:pool:<pool_id>`.                                                                                                                                                                                                                            |
| `act.act.act.act` | La identidad que creó el secreto del entorno con el que se registró el ejecutor. La cadena termina aquí.                                                                                                                                                                          |

<h2 id="scope-derived-credentials">
  Limitar credenciales derivadas
</h2>

El token de sesión identifica la identidad del usuario o servicio que creó la sesión, pero no lo trate como equivalente a ese creador iniciando sesión directamente. El token se encuentra en una variable de entorno dentro de la sesión, por lo que cualquier código que Claude ejecute y cualquier herramienta o servidor MCP que inicie la sesión puede leerlo y presentarlo.

La verificación también es sin conexión: un token que se verifica contra el JWKS permanece válido hasta su `exp`, sin importar lo que haya sucedido con la sesión desde entonces, y Anthropic no publica una fuente de revocación para tokens de sesión. Limite cualquier cosa que derive del token en consecuencia.

Cuando su servicio intercambia el token por credenciales internas, emita credenciales limitadas a lo que una sesión de codificación debería alcanzar:

* **Limitar capacidades**: otorgue acceso de lectura y escritura a los recursos que la sesión necesita para tareas de codificación, no a las capacidades administrativas que el creador tiene en otros lugares.
* **Limitar vida útil**: limite las credenciales derivadas a `exp` del token, o menos.
* **Auditar como la sesión**: registre `ccr:session_id` y `jti` junto con la identidad del creador para que pueda rastrear acciones hasta una sesión específica.

<h2 id="related-environment-variables">
  Variables de entorno relacionadas
</h2>

La identidad del creador también aparece en variables de entorno simples en dos superficies que nunca verifican el token:

* **El [hook `spawn-runner`](/docs/es/self-hosted-environments-configuration#the-spawn-runner-hook), en el orquestador**: el hook se ejecuta antes de que exista cualquier ejecutor para una sesión en cola y recibe la identidad del creador en variables como `CLAUDE_RUNNER_ACCOUNT_EMAIL` y `CLAUDE_RUNNER_ACCOUNT_ID`. El orquestador las lee de la orden de trabajo, el token de un solo uso firmado que autoriza generar un ejecutor, sin verificar la firma de la orden de trabajo en sí; las reclamaciones se confían porque la orden de trabajo llega sobre la conexión del orquestador a Anthropic, que el secreto del entorno autentica.
* **[Scripts contenedores](/docs/es/self-hosted-environments-configuration#wrapper-scripts), dentro de la sesión**: los contenedores reciben `CCR_SESSION_ACCOUNT_EMAIL`, el correo electrónico del creador preextraído del token sin verificación de firma. La variable es adecuada para etiquetado, como remolques de confirmación, no para decisiones de autenticación.

Use las variables simples para decisiones del lado del orquestador, como seleccionar una imagen de máquina. Use `CLAUDE_CODE_SESSION_ACCESS_TOKEN` cuando un servicio aguas abajo necesite prueba criptográfica independiente en lugar de confiar en el entorno del ejecutor.

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Entornos autohospedados](/docs/es/self-hosted-environments): el modelo de entorno, ejecutor y sesión; el [inicio rápido](/docs/es/self-hosted-environments-quickstart) y [Implementar en producción](/docs/es/self-hosted-environments-deploy) contienen configuración y operaciones
* [Personalizar sesiones](/docs/es/self-hosted-environments-configuration): scripts contenedores que consumen el token y el hook `spawn-runner`
* [Referencia](/docs/es/self-hosted-environments-reference): banderas CLI, variables de entorno y métricas
