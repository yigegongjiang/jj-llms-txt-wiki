> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuración de la puerta de enlace de aplicaciones Claude

> Referencia para cada opción de gateway.yaml: listener y TLS, OIDC, sesión, almacén Postgres, upstream de Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud y Microsoft Foundry, enrutamiento de modelos, políticas administradas y telemetría.

Una implementación de la puerta de enlace de aplicaciones Claude se configura mediante un archivo YAML, convencionalmente `gateway.yaml`. El archivo define todo lo que hace la puerta de enlace: dónde escucha, cómo inician sesión los desarrolladores, dónde va la inferencia y qué políticas y telemetría se aplican. Esta página es la referencia para cada opción en ese archivo.

Para escribir el primero, comience desde el [inicio rápido](/docs/es/claude-apps-gateway#quickstart), que construye una configuración mínima funcional y la ejecuta. Una vez que tenga una configuración con la que esté satisfecho, la [guía de implementación](/docs/es/claude-apps-gateway-deploy) cubre la containerización y el alojamiento en Kubernetes, Cloud Run o su propia plataforma.

La puerta de enlace lee el archivo una vez, al iniciar, con `claude gateway --config /path/to/gateway.yaml`. Cada opción se valida contra un esquema al arrancar, por lo que una configuración mal formada falla al iniciar con un error a nivel de campo en lugar de en el primer uso.

El [ejemplo completo](#complete-example) al final de esta página ejercita cada sección.

<h2 id="file-structure">
  Estructura del archivo
</h2>

Cinco secciones son [requeridas](#required-sections). Todas las demás secciones son [opcionales](#optional-sections), y una sección omitida toma sus valores predeterminados. Las claves desconocidas fallan al arrancar, por lo que un error tipográfico aparece como un error nombrado en lugar de una configuración silenciosamente ignorada.

**Secciones requeridas:**

* [`listen`](#listen): dirección de enlace, URL pública, terminación TLS
* [`oidc`](#oidc): su proveedor de identidad (IdP), incluido emisor, cliente, mapeo de reclamaciones y quién puede iniciar sesión
* [`session`](#session): los tokens portadores que emite la puerta de enlace, con secreto y duración
* [`store`](#store): PostgreSQL, para concesiones de dispositivos y contadores de límite de velocidad
* [`upstreams`](#upstreams): dónde va la inferencia, ya sea Anthropic, Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud o Microsoft Foundry

**Secciones opcionales:**

* [`admin`](#admin): autenticación de API de administración y retención de límites de gasto
* [`enforcement`](#enforcement): comportamiento de límite de gasto de fallo abierto o fallo cerrado
* [`pricing`](#pricing): tasas contratadas y un multiplicador de descuento para el medidor de gasto y para las cifras de costo que ven los desarrolladores
* [`models`](#models) y `auto_include_builtin_models`: lista de modelos curada por administrador e IDs por upstream
* [`managed`](#managed): políticas de configuración administradas por grupo de IdP
* [`telemetry`](#telemetry): reenvío OTLP a su pila de observabilidad
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): permitir/denegar IP, límites de tamaño de solicitud, tiempo hasta el primer byte del upstream y límites de inicio de sesión por IP
* [`load_test_mode`](#load_test_mode): prueba de carga de la puerta de enlace sin llamar a un proveedor de modelos

<h2 id="secret-expansion">
  Expansión de secretos
</h2>

No escriba secretos como `client_secret`, `jwt_secret` o `postgres_url` directamente en `gateway.yaml`. Haga referencia a ellos con uno de los formularios a continuación, y la puerta de enlace resuelve el valor al arrancar desde una variable de entorno o un archivo:

| Formulario      | Se resuelve a                                                                                                                                                                                                                                                                                          | Usar para                                                                         |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| `${VAR}`        | La variable de entorno `VAR`. El arranque falla si no está definida.                                                                                                                                                                                                                                   | Variables de entorno de contenedor, AWS Secrets Manager mediante inyección de env |
| `${file:/path}` | Contenido del archivo en esa ruta absoluta, recortado. La referencia debe ser el valor completo del campo: a diferencia de `${VAR}`, no se expande dentro de una cadena más larga, así que para una contraseña de base de datos establezca `store.password` en lugar de incrustarla en `postgres_url`. | Montajes de volumen de secreto de Kubernetes, Vault Agent, SOPS                   |

<h2 id="required-sections">
  Secciones requeridas
</h2>

<h3 id="listen">
  `listen`
</h3>

El bloque `listen` controla dónde sirve la puerta de enlace: la dirección de enlace y puerto, el origen visible externamente y la terminación TLS opcional.

| Campo                  | Requerido                       | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | No                              | Dirección de enlace. Predeterminado `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `port`                 | No                              | Puerto de enlace. Predeterminado `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `public_url`           | A menos que `host` sea loopback | El origen `https://` visible externamente, utilizado para construir el `redirect_uri` de IdP y metadatos de descubrimiento. Requerido siempre que `host` no sea una dirección loopback, ya sea que TLS termine en un proxy como ALB, Ingress o Cloud Run o en la puerta de enlace misma a través de `tls`, porque la puerta de enlace nunca deriva su propio origen de encabezados `X-Forwarded-*`; son suplantables por el cliente. El arranque falla sin él. `trusted_proxies` a continuación rige solo la resolución de IP del cliente. También es necesario para habilitar [telemetría](#telemetry), porque la puerta de enlace construye el punto final OTLP que envía a los clientes desde esta URL. |
| `tls.cert` / `tls.key` | No                              | Rutas PEM si la puerta de enlace termina TLS por sí misma                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `trusted_proxies`      | No                              | CIDR o IPs de equilibradores de carga frente a la puerta de enlace. Cuando se establece, la puerta de enlace confía en `X-Forwarded-For` solo desde estos pares y registra la IP del cliente real para límites de velocidad por IP y auditoría. Equivalente a nginx `set_real_ip_from`. Las entradas `X-Forwarded-For` escritas como `ipv4:port` o `[ipv6]:port`, como algunos equilibradores de carga hacen, se leen con el puerto descartado. Una dirección IPv6 con un puerto añadido y sin corchetes puede leerse como una dirección diferente o no leerse en absoluto, así que desactive la opción de puerto en cualquier proxy que escriba esa forma.                                                |

<h3 id="oidc">
  `oidc`
</h3>

El bloque `oidc` conecta la puerta de enlace a su proveedor de identidad y decide quién puede iniciar sesión. Nombra el emisor y cliente OAuth, mapea las reclamaciones que llevan correo electrónico y grupos, y restringe el inicio de sesión por dominio de correo electrónico o grupo.

OpenID Connect (OIDC) es el protocolo SSO que la puerta de enlace utiliza con su proveedor de identidad; consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para saber qué registrar en el lado de IdP.

| Campo                           | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `issuer`                        | Sí        | Base de descubrimiento OIDC. Debe servir descubrimiento en `/.well-known/openid-configuration`. Use HTTPS en producción; la puerta de enlace acepta un emisor `http://`. Un emisor de bucle local como `http://localhost:8081` es rechazado por la [protección SSRF](/docs/es/claude-apps-gateway-deploy#threat-model-summary) a menos que `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` esté establecido en el entorno de la puerta de enlace.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `client_id` / `client_secret`   | Sí        | De su registro de cliente OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `allowed_email_domains`         | No        | Rechace id\_tokens cuya reclamación `email` no esté en uno de estos dominios, sin distinción de mayúsculas y minúsculas. Defensa en profundidad contra configuración errónea de IdP multiinquilino. Independientemente de esta configuración, un id\_token cuya reclamación `email_verified` es explícitamente `false` siempre se rechaza.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowed_groups`                | No        | Restrinja el inicio de sesión a miembros de estos grupos de IdP, comparados contra `groups_claim`. Un usuario en un dominio de correo electrónico permitido pero en ninguno de estos grupos es rechazado. Requiere que IdP emita la reclamación de grupos. La coincidencia es una comparación de cadena exacta y sensible a mayúsculas y minúsculas contra los valores en esa reclamación, y la puerta de enlace no expande grupos anidados: para admitir miembros de un subgrupo, enumere el subgrupo aquí o configure IdP para emitir membresía aplanada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `groups_claim`                  | No        | Qué reclamación de id\_token lleva la membresía del grupo. Predeterminado `groups`. Microsoft Entra emite roles de aplicación bajo `roles`. Acepta una clave plana o un puntero JSON RFC 6901 como `/resource_access/gateway/roles` para reclamaciones anidadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `google_groups`                 | No        | Busque los grupos del usuario que inició sesión a través de la API del Directorio del SDK de administración de Google Workspace, porque el id\_token de Google no lleva reclamación de grupos. Establezca `service_account_json_path` en un archivo de clave de cuenta de servicio con delegación en todo el dominio en el alcance `https://www.googleapis.com/auth/admin.directory.group.readonly`, y `admin_email` en un administrador de Workspace que la cuenta de servicio suplanta; la API del Directorio requiere un asunto administrador real. Las direcciones de correo electrónico del grupo de cada usuario se convierten en su reclamación de grupos, por lo que `allowed_groups` y `managed.policies.match.groups` coinciden en correos electrónicos de grupo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `email_claim`                   | No        | Qué reclamación de id\_token lleva el correo electrónico del usuario. Predeterminado `email`. Algunos IdP, como ADFS y Entra B2C, emiten `upn` o `preferred_username` en su lugar. Acepta una clave plana, un puntero JSON o una lista de claves de respaldo donde se utiliza la primera clave presente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `scopes`                        | No        | Anulación completa de los alcances OIDC que solicita la puerta de enlace. Predeterminado `[openid, profile, email, offline_access]`. Establezca cuando su IdP rechace alcances que no reconoce, o requiera un alcance personalizado para emitir grupos o correo electrónico. Debe incluir `openid`. Soltar `offline_access` desactiva los tokens de actualización, por lo que los desarrolladores vuelven a ejecutar el inicio de sesión del navegador cada `session.ttl_hours`. Consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para recetas de alcance por IdP como el flujo de token de actualización de Google.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `scope_on_refresh`              | No        | También envíe `scope`, con la misma lista que la solicitud de inicio de sesión, cuando la puerta de enlace intercambia un token de actualización. Predeterminado `false`: la solicitud de actualización omite `scope`. La mayoría de IdP devuelven un id\_token en cada actualización y no necesitan esto. Establezca `true` cuando su IdP devuelve un id\_token en la actualización solo si se solicita `openid` nuevamente, que Okta documenta para su concesión de actualización. Sin un id\_token, cada actualización depende del punto final userinfo de IdP que acepte el token de acceso actualizado. Si controla el inicio de sesión o coincide con políticas en grupos y el id\_token de su IdP en tiempo de actualización los omite, también establezca `userinfo_fallback: true` para que la puerta de enlace los complete desde el punto final userinfo. Un IdP que otorgó menos alcances de los solicitados puede rechazar la actualización con `invalid_scope`, incluso para sesiones existentes si agrega entradas a `scopes` mientras esto está activado. Desestablezca la clave si las actualizaciones comienzan a fallar en `token_endpoint` después de establecerla. Requiere Claude Code v2.1.260 o posterior en el servidor de la puerta de enlace. |
| `extra_auth_params`             | No        | Parámetros de consulta adicionales añadidos a la solicitud de autorización de IdP, textualmente. Este es el mecanismo de anulación para comportamiento específico de IdP, como `access_type: offline` para tokens de actualización de Google, `domain_hint` para algunos inquilinos de Entra, o `acr_values` para flujos de escalada. No puede anular los parámetros de protocolo administrados por la puerta de enlace: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` y `client_id`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `userinfo_fallback`             | No        | Cuando el id\_token omite correo electrónico o grupos, búsquelos en `/userinfo`. Necesario para tokens de acceso ligeros de Keycloak, el servidor org de Okta y tokens mínimos de ADFS. El id\_token sigue siendo autoritario; userinfo solo llena vacíos. Predeterminado `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `use_pkce`                      | No        | Envíe un desafío PKCE (S256) en la solicitud de autorización. Predeterminado `true`. Establezca `false` solo si su IdP rechaza PKCE para este cliente confidencial.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `clock_skew_seconds`            | No        | Tolere la desviación del reloj al validar reclamaciones de tiempo de id\_token. Predeterminado `0`, que es estricto. Aumente si ve errores "token expirado / aún no válido" justo después del inicio de sesión debido a desviación del reloj de host/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `token_endpoint_auth_method`    | No        | Anule el método de autenticación del punto final del token. Acepta `client_secret_basic` o `client_secret_post`. Negociado automáticamente de forma predeterminada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `id_token_signed_response_alg`  | No        | Algoritmo de firma de id\_token esperado. Predeterminado `RS256`. Establezca para IdP que firman con ES256, PS256 o EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `additional_authorized_parties` | No        | Valores `azp` adicionales para aceptar más allá de `client_id`, para flujos de intermediario de Keycloak e intercambio de tokens                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `discovery_url`                 | No        | Busque el documento de descubrimiento desde esta URL en lugar de derivarlo de `issuer`, para IdP detrás de un proxy que reescribe el host del emisor. La ruta debe contener `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `use_proxy`                     | No        | Envíe las propias solicitudes de IdP de la puerta de enlace a través del proxy directo en `HTTPS_PROXY` o `HTTP_PROXY`, honrando `NO_PROXY`. `false` mantiene esas solicitudes directas. Requiere v2.1.227 o posterior; consulte [Solicitudes de IdP a través de un proxy directo](#idp-requests-through-a-forward-proxy) a continuación.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `form_action_origins`           | No        | Orígenes adicionales para la directiva `Content-Security-Policy: form-action` de la página `/device`. La puerta de enlace ya permite `'self'` y el origen `authorization_endpoint` descubierto, pero Chrome aplica `form-action` contra toda la cadena de redirección. Si su IdP redirige a través de un segundo host, como Azure AD federado a ADFS, Okta de concentrador y radio, o un interceptor SSO corporativo, enumere cada origen por el que la solicitud de autorización puede redirigir.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `ca_cert_pem`                   | No        | El certificado CA codificado en PEM en sí, no una ruta a un archivo. Reemplaza el almacén de confianza del sistema solo para solicitudes de IdP. Para cargar un archivo montado, escriba `${file:/etc/gateway/idp-ca.pem}`. Úselo para Keycloak o Dex detrás de PKI corporativa.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

<h4 id="idp-requests-through-a-forward-proxy">
  Solicitudes de IdP a través de un proxy directo
</h4>

Los upstreams de inferencia honran `HTTPS_PROXY` e `HTTP_PROXY` en cada versión. Las propias solicitudes de la puerta de enlace al IdP, descubrimiento, JWKS, token y userinfo, van directas a menos que establezca `oidc.use_proxy: true`, que requiere v2.1.227 o posterior. Cuando se establece una variable de proxy, `use_proxy` no se establece, y el emisor no está cubierto por `NO_PROXY`, la puerta de enlace mantiene esas solicitudes directas y registra un aviso al arrancar pidiéndole que elija; `use_proxy: false` las mantiene directas y silencia el aviso.

Con `use_proxy: true`, la vaina resuelve el nombre de host de cada punto final de IdP por sí misma y pide al proxy que `CONNECT` a la dirección IP resuelta, por lo que el proxy debe aceptar `CONNECT` a la dirección IP de cada host que el documento de descubrimiento nombra, no solo el emisor. Use una URL de proxy `http://`. `ca_cert_pem` y la [protección SSRF](/docs/es/claude-apps-gateway-deploy#threat-model-summary) se aplican en la ruta proxificada también.

[Egreso solo de proxy](#proxy-only-egress) cambia ambos: mientras está activo, las solicitudes de IdP siguen el proxy a menos que establezca `use_proxy: false`, y la puerta de enlace entrega al proxy cada nombre de host de IdP sin resolverlo primero.

<h4 id="proxy-only-egress">
  Egreso solo de proxy
</h4>

Establezca `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1` en el entorno de la puerta de enlace, junto a `HTTPS_PROXY`, cuando la vaina alcanza otros hosts solo a través de ese proxy directo y no puede resolver nombres DNS públicos por sí misma, o cuando el proxy rechaza `CONNECT` a una dirección IP. Requiere v2.1.277 o posterior. Es una variable de entorno en lugar de una clave `gateway.yaml` para que nada en el archivo de configuración pueda relajar la verificación de dirección de la puerta de enlace.

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

La puerta de enlace registra una línea `network:` al arrancar mientras el egreso solo de proxy está activo.

Cada fila a continuación es una clase de solicitud saliente en una puerta de enlace con `HTTPS_PROXY` establecido, de forma predeterminada y mientras el egreso solo de proxy está activo.

| Solicitud saliente                                                                                                                     | Predeterminado                                                                                                                                                                            | Egreso solo de proxy activo                                                                            |
| -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Upstreams `provider: anthropic`, intercambio de tokens de Workload Identity Federation, exportaciones `telemetry.forward_to`           | Resuelto y verificado localmente, luego `CONNECT` a la dirección IP verificada a través del proxy. Un recopilador de telemetría listado en `NO_PROXY` se alcanza directamente en su lugar | Nombre de host entregado al proxy                                                                      |
| Descubrimiento de IdP, JWKS, token y userinfo                                                                                          | Directo a menos que [`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy), luego `CONNECT` a la dirección IP verificada                                                         | Nombre de host entregado al proxy, a menos que `oidc.use_proxy: false` mantenga un IdP interno directo |
| Upstreams de Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud y Microsoft Foundry; búsquedas de grupos de Google | Nombre de host entregado al proxy                                                                                                                                                         | Sin cambios                                                                                            |

El egreso solo de proxy se mantiene desactivado a menos que el entorno de la puerta de enlace cumpla con las tres condiciones siguientes:

* `HTTPS_PROXY` o `HTTP_PROXY` está establecido.
* `NO_PROXY` y `no_proxy` están vacíos. Si su plataforma inyecta cualquiera en vainas, establezca ambos en un valor vacío en el contenedor de la puerta de enlace. Listar un recopilador de telemetría en `NO_PROXY` mantiene el egreso solo de proxy desactivado.
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK` no está activado. Un recopilador o IdP en el propio loopback de la vaina no se puede combinar con egreso solo de proxy, porque una dirección loopback entregada al proxy sería la del propio host del proxy, así que dé a esos servicios una dirección que el proxy pueda alcanzar en su lugar. Por la misma razón, la puerta de enlace rechaza nombres de estilo `localhost` directamente mientras el egreso solo de proxy está activo.

Cuando una de esas condiciones no se cumple, la puerta de enlace registra una advertencia al arrancar nombrando la variable que lo detuvo y mantiene el comportamiento predeterminado.

Una vez que el egreso solo de proxy está activo, permita cada destino en el proxy, incluyendo un recopilador interno y cualquier host configurado por dirección IP. Aún puede mantener un IdP interno directo con [`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy).

<Warning>
  Active esto solo cuando la lista de permitidos del proxy sea al menos tan estricta como la verificación propia de la puerta de enlace. El proxy debe rechazar puntos finales de metadatos en la nube como `169.254.169.254` y `metadata.google.internal`, direcciones de enlace local, y el propio loopback del host del proxy, y debe rechazarlos por la dirección a la que se resuelve un nombre, no solo por nombre, porque la puerta de enlace ya no detecta un nombre de host que se resuelve a uno de ellos. Un proxy que se conecta a cualquier lugar que se le pida elimina la [protección SSRF](/docs/es/claude-apps-gateway-deploy#threat-model-summary) de la puerta de enlace para estas solicitudes.
</Warning>

<h3 id="session">
  `session`
</h3>

El bloque `session` forma los tokens portadores que emite la puerta de enlace después del inicio de sesión: el secreto que los firma y cuánto tiempo viven.

| Campo        | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------ | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Sí        | Al menos 32 bytes de entropía, por ejemplo de `openssl rand -base64 32`. Firma los tokens portadores HS256 de la puerta de enlace. Acepta una cadena única o una matriz para rotación: el índice 0 firma y todas las entradas verifican. Para rotar, anteponga un nuevo secreto, espere `ttl_hours`, luego suelte el antiguo.                                                                                                                                                                                                |
| `ttl_hours`  | No        | Duración del token portador de la puerta de enlace. Predeterminado `1`. El CLI se actualiza silenciosamente antes de la expiración cuando IdP emite tokens de actualización. Una duración más corta desactiva más rápido; una más larga hace menos viajes de IdP. Si su IdP no puede emitir tokens de actualización porque `offline_access` no está disponible, no hay actualización silenciosa, así que aumente esto a `8` o `12` para evitar enviar desarrolladores de vuelta al inicio de sesión del navegador cada hora. |

<h3 id="store">
  `store`
</h3>

El bloque `store` apunta la puerta de enlace a su base de datos PostgreSQL, que contiene concesiones de dispositivos y contadores de límite de velocidad.

| Campo                     | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `postgres_url`            | Sí        | URL `postgres://` o `postgresql://`. Requerido: el encuentro de concesión de dispositivo, donde la devolución del navegador escribe y el CLI de sondeo lee, necesita estado entre réplicas. La puerta de enlace ejecuta sus propias migraciones de esquema al arrancar y al actualizar, por lo que el rol necesita derechos para crear y alterar tablas en el esquema de destino. Consulte [Actualizaciones](/docs/es/claude-apps-gateway-deploy#upgrades) y [Postgres](/docs/es/claude-apps-gateway-deploy#postgres). |
| `username`                | No        | Anula el usuario en `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `password`                | No        | Credencial de base de datos. Establézcalo aquí en lugar de en `postgres_url` para que la credencial se mantenga fuera de la URL. Acepta cualquier carácter y tiene prioridad sobre las credenciales de URL.                                                                                                                                                                                                                                                                                                  |
| `max_connections`         | No        | Tamaño del grupo de conexiones de Postgres por réplica. Predeterminado `5`, que es conservador y amigable con bases de datos compartidas. Con [límites de gasto](#admin) habilitados, la ruta activa realiza algunas operaciones por solicitud de inferencia, así que aumente para una base de datos dedicada bajo carga, y mantenga réplicas × esto por debajo de `max_connections` de la base de datos.                                                                                                    |
| `connect_timeout_seconds` | No        | Segundos que la puerta de enlace espera cuando abre una conexión de Postgres. Un número entero de `1` a `60`, predeterminado `5`. Aumente si los intentos de conexión agotan el tiempo de espera cuando comienza una nueva instancia de puerta de enlace. Requiere Claude Code v2.1.274 o posterior en el servidor de la puerta de enlace. Las versiones anteriores se niegan a iniciar cuando se establece la clave.                                                                                        |

Para desarrollo local, apunte `postgres_url` a un contenedor Postgres desechable, por ejemplo `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` es una lista ordenada. La puerta de enlace reenvía la inferencia al primer upstream que resuelve el modelo solicitado.

En `5xx`, `429`, `401`, `403`, `404`, o tiempo de espera, la puerta de enlace conmuta por error al siguiente upstream; otros `4xx` no, porque esos errores son atribuibles a la solicitud en lugar del upstream. Un `401` o `403` significa que la credencial propia de la puerta de enlace falló contra ese upstream. Un `404` significa que ese upstream no sirve el modelo solicitado, por lo que un upstream posterior en la lista aún puede.

Si establece `forward_user_identity: true` en un upstream, un `429` que devuelve a una solicitud que llevaba el correo electrónico del desarrollador no conmuta por error. Consulte [cómo una denegación de límite por usuario llega al desarrollador](#per-user-identity-headers-for-a-proxy-you-run).

La conmutación por error en `404` requiere gateway v2.1.198 o posterior. Las versiones anteriores devolvieron el primer `404` al cliente incluso cuando un upstream posterior en la lista sirvió el modelo.

Múltiples upstreams del mismo proveedor deben establecer un `name:` distinto.

Los clientes de Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud y Microsoft Foundry se construyen una vez al iniciar, y sus SDK actualizan credenciales internamente, por lo que rotar credenciales en la nube no requiere un reinicio. Las claves API estáticas de Anthropic y los portadores se leen al iniciar; consulte [API de Anthropic](#anthropic-api).

<h4 id="upstream-error-messages">
  Mensajes de error de upstream
</h4>

La puerta de enlace devuelve la respuesta de error de un upstream, o su propio `502`, dependiendo de cómo respondieron los upstreams:

* **Un upstream devolvió un estado en el que la puerta de enlace no [conmuta por error](#multiple-upstreams)**: esa respuesta del upstream. La puerta de enlace no intenta más upstreams.
* **Cada upstream que la puerta de enlace intentó falló de una manera en la que [conmuta por error](#multiple-upstreams)**: el último `429`. Cuando ninguno devolvió un `429`, la puerta de enlace prefiere, en orden, el último `401` o `403`, el último `404`, y el último `501`. Cuando ninguno devolvió ninguno de esos, el propio `502` de la puerta de enlace, `all upstreams failed (N attempted)`, donde N cuenta cada entrada en [`upstreams`](#upstreams), incluyendo entradas que la puerta de enlace omitió porque no sirven el modelo solicitado.

Cuando la puerta de enlace devuelve la respuesta de un upstream, mantiene el código de estado del upstream. Si mantiene el mensaje del upstream depende del proveedor. El cuerpo de error de un upstream de API de Anthropic llega al desarrollador sin cambios.

Los upstreams de Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud y Microsoft Foundry pueden nombrar sus IDs de cuenta, ARN de rol e IDs de proyecto en su texto de error. La puerta de enlace registra ese texto completo en el [registro operacional](/docs/es/claude-apps-gateway-deploy#logs). Lo que el desarrollador ve de esos upstreams depende del rechazo:

* `400` o `413` en el sobre de error estándar de Anthropic: el mensaje del upstream, como `prompt is too long`. Claude Platform en AWS, Agent Platform y Microsoft Foundry devuelven este sobre para rechazos de API de modelo.
* `400` o `413` en la forma propia del proveedor: un token `capability_rejected:`. Cuando la puerta de enlace no puede clasificar el rechazo, `upstream rejected the request` en un `400` o `request too large for this upstream` en un `413`.
* Cualquier otro estado: copia genérica por estado, como `upstream rate limit exceeded` en un `429`.

Por ejemplo, la puerta de enlace reemplaza `Input is too long for requested model.` de Amazon Bedrock con `capability_rejected: prompt_too_long`. Claude Code [compacta automáticamente](/docs/es/errors#prompt-is-too-long) en ese token, como lo hace en `prompt is too long`.

Mantener el mensaje `400` o `413` de un upstream en la nube, o reemplazarlo con un token `capability_rejected:`, requiere gateway v2.1.233 o posterior.

<h4 id="anthropic-api">
  API de Anthropic
</h4>

El upstream mínimo de Anthropic es una clave API de la [Consola Claude](https://platform.claude.com):

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # O un portador OAuth (p. ej., un token intercambiado por Workload-Identity-Federation):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # predeterminado; anule para un proxy directo
```

Las dos formas de credencial difieren en el encabezado que envían:

* **`api_key`**: envía `x-api-key`. Rótelo en la Consola Claude y actualice la variable env.
* **`oauth_token`**: envía `Authorization: Bearer`. Use la forma de portador cuando su organización emita tokens de corta duración en lugar de claves API de larga duración. El portador se lee una vez al iniciar, así que actualice remontando el secreto e reiniciando.

En lugar de una clave estática o portador, puede usar Workload Identity Federation. Cree una regla de federación siguiendo la [guía de Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), luego monte el JWT de OIDC de su carga de trabajo como un archivo, como un token de cuenta de servicio proyectado de Kubernetes o un id-token de plataforma de CI. La puerta de enlace intercambia el JWT por un portador de corta duración y lo actualiza automáticamente. El archivo de token se relee en cada intercambio, por lo que los tokens proyectados rotados se recogen sin un reinicio.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # requerido si la regla cubre >1 espacio de trabajo
      # service_account_id: svac_...   # verificación de destino esperado opcional
```

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  Encabezados de identidad por usuario para un proxy que usted ejecuta
</h5>

Puede apuntar el `base_url` de un upstream `provider: anthropic` a un proxy que usted ejecuta en lugar de a la API de Anthropic. Para decirle a ese proxy qué desarrollador envió cada solicitud, establezca `forward_user_identity: true` en ese upstream. El proxy puede entonces atribuir gasto por desarrollador. Requiere una puerta de enlace ejecutando Claude Code v2.1.233 o posterior.

Por ejemplo, para un proxy en `upstream-gateway.internal.example.com`:

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # predeterminado false
```

La puerta de enlace añade estos encabezados a cada solicitud que reenvía a ese upstream.

| Encabezado                    | Valor                                                                  |
| ----------------------------- | ---------------------------------------------------------------------- |
| `x-litellm-end-user-id`       | El correo electrónico del desarrollador, cuando IdP lo proporcionó.    |
| `x-claude-gateway-user-id`    | El asunto de IdP del desarrollador, de la reclamación `sub` del token. |
| `x-claude-gateway-user-email` | El correo electrónico del desarrollador, cuando IdP lo proporcionó.    |

Cuando el token de IdP no lleva correo electrónico, la puerta de enlace envía solo `x-claude-gateway-user-id` y omite los dos encabezados de correo electrónico. Si su IdP pone el correo electrónico en una reclamación diferente, establezca [`oidc.email_claim`](#oidc) en esa reclamación.

Cuando su proxy responde `429` a una solicitud que llevaba el correo electrónico del desarrollador, la puerta de enlace devuelve esa respuesta al desarrollador tal como está en lugar de conmutar por error al siguiente upstream, por lo que el presupuesto por usuario o límite de velocidad de su proxy se mantiene. Las otras respuestas del proxy siguen las [reglas de conmutación por error](#upstreams) ordinarias. Si el token de IdP de un desarrollador no lleva correo electrónico, la puerta de enlace reenvía sus solicitudes sin los encabezados de correo electrónico, por lo que un `429` a una de esas solicitudes cuenta como capacidad de upstream y conmuta por error. Antes de v2.1.267 en el servidor de la puerta de enlace, cada `429` conmutaba por error.

Establezca `forward_user_identity` solo en un upstream cuyo `base_url` sea un proxy que usted opera. La puerta de enlace envía correos electrónicos de desarrollador a cualquier servidor que ese `base_url` nombre. Si el `base_url` es la API de Anthropic, que es el predeterminado, la puerta de enlace se niega a iniciar.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Para la implementación de Bedrock del lado del cliente que la puerta de enlace reemplaza o enfrenta, consulte [Claude Code en Amazon Bedrock](/docs/es/amazon-bedrock). El upstream del lado de la puerta de enlace:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferido: cadena de credenciales predeterminada de AWS
    # O credenciales explícitas:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # O un token portador de API de Bedrock:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Anule el punto final de bedrock-runtime para implementaciones FIPS o de punto final de VPC:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Un bloque `auth` vacío utiliza la cadena de credenciales predeterminada del SDK de AWS: variables env, `~/.aws/credentials`, rol de tarea de ECS, metadatos de instancia de EC2 o IRSA en EKS. En producción, otorgue a la vaina de la puerta de enlace un rol de IAM en lugar de incrustar claves estáticas en una imagen de contenedor.

Las credenciales explícitas deben ser completas: la puerta de enlace falla al arrancar cuando `aws_access_key_id` y `aws_secret_access_key` no se establecen juntos, o cuando `aws_session_token` se establece sin ellos. Antes de v2.1.207, un bloque `auth:` parcial pasó la validación.

| Configuración           | Cómo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permisos de IAM         | Otorgue al principal de la puerta de enlace `bedrock:InvokeModel` y `bedrock:InvokeModelWithResponseStream` tanto en los ARN de perfil de inferencia como en los ARN de modelo de fundación subyacentes. Para el catálogo integrado en regiones de EE.UU.: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` y `arn:aws:bedrock:*::foundation-model/anthropic.*`. También otorgue `bedrock:CountTokens` en los ARN de modelo de fundación. La puerta de enlace lo utiliza, sin cargo, para contar los tokens de entrada de una solicitud que el cliente abandonó, por lo que [límites de gasto](#admin) se mantienen precisos. Sin él, la puerta de enlace vuelve a una solicitud de Bedrock de un token para ese conteo. |
| Acceso a modelos        | Amazon Bedrock habilita el acceso a modelos de forma predeterminada en regiones comerciales. La puerta de enlace de nivel de cuenta restante es la de Anthropic: si nadie en su cuenta de AWS la ha enviado, abra la consola de Amazon Bedrock, seleccione un modelo de Anthropic del catálogo de modelos y complete el formulario. Consulte [Enviar detalles de caso de uso](/docs/es/amazon-bedrock#1-submit-use-case-details) para el formulario de AWS Organizations y los permisos que el remitente necesita.                                                                                                                                                                                                                                 |
| EKS (IRSA)              | Cree un rol de IAM con la política anterior y una política de confianza para el proveedor OIDC de su clúster limitado a la cuenta de servicio de la puerta de enlace. Anote la cuenta de servicio con `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` la recoge.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ECS / EC2               | Adjunte el rol de IAM a la definición de tarea o perfil de instancia. `auth: {}` la recoge.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| En cualquier otro lugar | Pase credenciales a través de las variables env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` y `AWS_SESSION_TOKEN`, o establézcalas explícitamente en `auth:` con expansión `${VAR}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Región                  | `region:` es la región del punto final de API. Los perfiles de inferencia entre regiones enrutan a través de la geografía (EE.UU., UE, APAC) independientemente de cuál elija. Para regiones no estadounidenses o ARN de rendimiento aprovisionado, agregue un bloque [`models:`](#models) con los IDs correctos por upstream.                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="claude-platform-on-aws">
  Claude Platform en AWS
</h4>

Claude Platform en AWS sirve la API de Anthropic de primera parte en infraestructura de AWS en `aws-external-anthropic.<region>.api.aws`. Utiliza IDs de modelo de primera parte, honra encabezados `anthropic-beta` tal como se envían, y sirve `count_tokens`, por lo que ninguna de la traducción específica de Bedrock se aplica. El proveedor `anthropicAws` requiere Claude Code v2.1.198 o posterior; las versiones anteriores de gateway lo rechazan al arrancar.

Para la implementación del lado del cliente de la misma plataforma, consulte [Claude Code en Claude Platform en AWS](/docs/es/claude-platform-on-aws). El upstream del lado de la puerta de enlace:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # enviado como x-api-key
    # O SigV4 a través de la cadena de credenciales predeterminada de AWS:
    # auth: {}
    # O credenciales SigV4 explícitas:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Anule el punto final derivado:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

La plataforma se ejecuta en una cuenta de AWS separada de Amazon Bedrock y firma solicitudes SigV4 para su propio nombre de servicio, `aws-external-anthropic`, por lo que un rol de IAM limitado a Bedrock no lo autoriza. Una clave API en `auth.api_key` tiene prioridad cuando también se establecen credenciales SigV4. Un bloque `auth` vacío utiliza la cadena de credenciales predeterminada del SDK de AWS, la misma cadena que usa el upstream [Amazon Bedrock](#amazon-bedrock).

| Campo                                                   | Requerido | Descripción                                                                                                                                                  |
| ------------------------------------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `region`                                                | Sí        | Región de AWS, letras minúsculas, dígitos e guiones. La puerta de enlace deriva el punto final de él como `https://aws-external-anthropic.<region>.api.aws`. |
| `workspace_id`                                          | Sí        | Enviado como encabezado en cada solicitud; la plataforma lo requiere                                                                                         |
| `auth.api_key`                                          | No        | Clave API para la plataforma, enviada como `x-api-key`. No es un token portador: los dos modos de autenticación son una clave API o SigV4.                   |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | No        | Credenciales SigV4 explícitas. Establecer uno sin el otro falla al arrancar. `auth.aws_session_token` se acepta junto a ellos.                               |
| `base_url`                                              | No        | Anule el punto final derivado                                                                                                                                |

Debido a que la plataforma resuelve IDs de modelo de primera parte, el catálogo integrado enruta a ella sin un bloque [`models:`](#models). Cuando cura una lista `models:`, clave la entrada `anthropicAws:` con el ID de primera parte.

<h4 id="google-cloud-agent-platform">
  Plataforma de agentes de Google Cloud
</h4>

Para la configuración equivalente del lado del cliente, consulte [Claude Code en Google Cloud](/docs/es/google-vertex-ai). El upstream del lado de la puerta de enlace:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # preferido: Credenciales predeterminadas de aplicación
    # O un archivo de clave de cuenta de servicio:
    # auth: { service_account_json: /secrets/sa.json }
    # Anule el punto final de aiplatform para Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Un bloque `auth` vacío utiliza Credenciales predeterminadas de aplicación: `GOOGLE_APPLICATION_CREDENTIALS`, metadatos de GCE o Workload Identity de GKE. Los archivos de clave JSON de cuenta de servicio son compatibles pero desaconsejados; use Workload Identity o adjunte una cuenta de servicio a la instancia de GCE o Cloud Run.

Establezca `region: global` para usar el [punto final global de Agent Platform](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) en lugar de uno regional. Google luego enruta cada solicitud a una región disponible, por lo que no rastrea la disponibilidad de modelos por región. Establecer una región específica fija cada solicitud a ella.

| Configuración           | Cómo                                                                                                                                                                                                                              |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permisos de IAM         | Otorgue a la cuenta de servicio de la puerta de enlace `roles/aiplatform.user` en el proyecto, o un rol personalizado con `aiplatform.endpoints.predict`. Habilite la API de Agent Platform (`aiplatform.googleapis.com`).        |
| Acceso a modelos        | En Model Garden, habilite los modelos Claude para su proyecto. Se publican en regiones específicas; consulte la tarjeta del modelo para regiones compatibles.                                                                     |
| GKE (Workload Identity) | Vincule una cuenta de servicio de GCP a la cuenta de servicio de Kubernetes de la puerta de enlace y anote la KSA con `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` la recoge.      |
| Cloud Run / GCE         | Establezca la cuenta de servicio del servicio en una con `roles/aiplatform.user`. `auth: {}` la recoge.                                                                                                                           |
| En cualquier otro lugar | `auth: { service_account_json: /secrets/sa.json }`, la ruta a un archivo de clave JSON montado como secreto. El campo toma una ruta de archivo, no el contenido de la clave, por lo que no hay expansión `${file:…}` involucrada. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Para la implementación de Foundry del lado del cliente, consulte [Claude Code en Microsoft Foundry](/docs/es/microsoft-foundry). El upstream del lado de la puerta de enlace:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferido: DefaultAzureCredential / Managed Identity
    # O una clave API:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` se resuelve a través de `DefaultAzureCredential`: Managed Identity en AKS, ACI o App Service; la CLI de Azure; o credenciales de entorno. Las claves API funcionan pero son amplias del proyecto y no se rotan automáticamente. El punto final de Foundry se deriva de `resource:`; establezca el `base_url` opcional para anularlo para nubes soberanas como Azure Government.

| Configuración           | Cómo                                                                                                                                                                                                                    |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Otorgue a la identidad de la puerta de enlace `Azure AI User` o `Cognitive Services User` en el recurso de Foundry                                                                                                      |
| Implementaciones        | Foundry utiliza nombres de implementación elegidos por administrador, no IDs de modelo canónicos. Agregue un bloque [`models:`](#models) que asigne cada ID canónico a su nombre de implementación.                     |
| AKS (workload identity) | Federe una Managed Identity asignada por el usuario con el emisor OIDC del clúster y vincúlela a la cuenta de servicio de la puerta de enlace. `use_azure_ad: true` la recoge a través de `WorkloadIdentityCredential`. |
| ACI / App Service       | Habilite la identidad administrada asignada por el sistema o por el usuario en el recurso. `use_azure_ad: true` la recoge.                                                                                              |
| En cualquier otro lugar | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Entrecomille `${…}` dentro de `{ }`.                                                                                                                                         |

<h4 id="static-headers-on-upstream-requests">
  Encabezados estáticos en solicitudes de upstream
</h4>

Para agregar encabezados fijos a las solicitudes que la puerta de enlace envía a un upstream, establezca `headers:` en ese upstream. Úselo cuando un proxy que ejecuta frente al proveedor enruta o atribuye tráfico por un encabezado.

`headers:` requiere Claude Code v2.1.277 o posterior en el servidor de la puerta de enlace. Una puerta de enlace anterior se niega a iniciar cuando encuentra la clave. Actualice cada réplica antes de agregar la clave, y elimine la clave antes de revertir a una versión anterior.

Los encabezados van al servidor que `base_url` nombra, o al punto final propio del proveedor cuando `base_url` no se establece. El proveedor también los recibe a menos que su proxy los elimine.

Este ejemplo alcanza un upstream `provider: vertex` a través de un proxy en `upstream-proxy.internal.example.com`. Establece el encabezado `x-source` que el proxy lee, y envía un token de la variable de entorno `PROXY_TOKEN` como `x-proxy-token`:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

Los valores son texto ASCII imprimible sin espacio en ninguno de los extremos. Entrecomille un número, `true`, o `false` para que YAML lo lea como texto.

Para mantener un secreto fuera del archivo de configuración, use [expansión de secretos](#secret-expansion) para cargar el valor de una variable de entorno con `${VAR}` o de un archivo con `${file:/path}`. Un `${VAR}` que se resuelve a un valor vacío detiene la puerta de enlace de iniciar.

`headers:` funciona en cada proveedor, y cada upstream envía solo el suyo.

No cada solicitud que la puerta de enlace envía a un upstream los lleva:

| Solicitud que la puerta de enlace envía a este upstream                            | Lleva `headers:`                     |
| ---------------------------------------------------------------------------------- | ------------------------------------ |
| `/v1/messages`, streaming o no, y `/v1/messages/count_tokens`                      | Sí                                   |
| Una solicitud que conmutó por error desde otro upstream                            | Sí, solo `headers:` de este upstream |
| Llamada `CountTokens` de Amazon Bedrock para una solicitud que el cliente abandonó | No                                   |
| El intercambio de tokens de Workload Identity Federation                           | No                                   |

En un upstream de Amazon Bedrock o Claude Platform en AWS que firma solicitudes con AWS SigV4, estos encabezados son parte de la firma, por lo que su proxy debe pasarlos sin cambios.

Si utiliza un nombre que la puerta de enlace reserva, se niega a iniciar, y el error de inicio nombra el encabezado. Los nombres reservados incluyen:

* `authorization` y `x-api-key`
* `host`, `content-type` y `user-agent`
* Cualquier nombre que comience con `anthropic-`, `x-goog-`, `x-amz-` o `x-amzn-`

<h4 id="multiple-upstreams">
  Múltiples upstreams
</h4>

El mismo proveedor puede aparecer más de una vez con un `name:` distinto. Esto cubre diferentes regiones, diferentes cuentas a través de diferentes cadenas de credenciales, rendimiento aprovisionado versus bajo demanda, y conmutación por error entre proveedores.

La puerta de enlace intenta upstreams en orden. `5xx`, `429`, `401`, `403`, `404`, tiempos de espera y punto final faltante (`501`) conmutan por error; otros `4xx` no.

`429` es capacidad por upstream, por lo que el agotamiento de rendimiento aprovisionado (PT) conmuta por error a bajo demanda. Si establece [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) en un upstream, un `429` a una solicitud que llevaba el correo electrónico del desarrollador es una denegación por usuario en su lugar y no conmuta por error.

Cada solicitud comienza en el primer upstream. Una solicitud alcanza un upstream posterior solo cuando cada upstream anterior a él ha fallado o no sirve el modelo solicitado.

La puerta de enlace no mantiene registro de upstreams fallidos, por lo que mientras un upstream está inactivo, cada solicitud que lo alcanza aún lo intenta y espera a que falle antes de pasar al siguiente.

Para un upstream de API de Anthropic, [`timeouts.upstream_ttfb_ms`](#http-tuning) limita la espera en un upstream inactivo. Esa configuración no se aplica a los otros proveedores, donde la puerta de enlace espera hasta una hora a que un upstream comience a responder.

`404` es disponibilidad de modelo por upstream, por lo que un upstream que no ha habilitado un modelo no bloquea un upstream posterior que lo sirve. Un upstream que no puede resolver el modelo solicitado se omite sin un viaje de red.

Este ejemplo enruta una asignación de rendimiento aprovisionado de Bedrock primero, desborda a bajo demanda y una segunda cuenta, y vuelve a la API de Anthropic al final:

```yaml theme={null}
upstreams:
  # Primario: rendimiento aprovisionado en su región de inicio.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Desbordamiento: bajo demanda entre regiones.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Cuenta diferente: una asignación de Bedrock separada a través de credenciales de rol asumido.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Último recurso: API de Anthropic directo.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Los IDs de modelo por upstream se clave en el `name:` del upstream.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Palanca                        | Cómo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Diferentes regiones            | Un upstream de Bedrock por región, cada uno con su propio `region:`. Con [`auto_include_builtin_models: true`](#models) los perfiles de inferencia entre regiones enrutan automáticamente; para implementaciones fijas de región use un bloque `models:`.                                                                                                                                                                                                                                                                                |
| Diferentes cuentas             | Un upstream de Bedrock por cuenta, cada uno con sus propias credenciales en `auth:`. La cadena predeterminada (`auth: {}`) utiliza la identidad de la vaina; para una segunda cuenta, establezca credenciales explícitas o un token portador.                                                                                                                                                                                                                                                                                            |
| Rendimiento aprovisionado      | Asigne el modelo al ARN de rendimiento aprovisionado en `models:` para el nombre de ese upstream. Otros upstreams mantienen el ID bajo demanda, por lo que la capacidad de PT se agota antes de conmutar por error.                                                                                                                                                                                                                                                                                                                      |
| Puntos finales de VPC / FIPS   | Establezca `base_url:` en el upstream a su URL de punto final de VPC o FIPS                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Enrutamiento limitado a modelo | Solo un modelo `id` personalizado, uno que no sea un modelo Claude integrado, omite los upstreams ausentes de su mapa `upstream_model:`. La puerta de enlace intenta modelos integrados en cada upstream en orden y utiliza el ID predeterminado del proveedor donde el mapa no tiene entrada, por lo que para modelos integrados el mapa cambia qué ID recibe un upstream en lugar de si se intenta; un upstream que rechaza el ID sigue las mismas [reglas de conmutación por error](#upstreams) que cualquier otro error de upstream. |

La conmutación por error entre proveedores en la nube, o a la API de Anthropic directo, cambia qué acuerdo, geografía y otros términos rigen la solicitud.

El CLI aplica el mismo control de características a puertas de enlace independientemente de cuál upstream sirva una solicitud dada, por lo que la conmutación por error no envía un campo de cuerpo que un upstream rechazaría.

<h2 id="optional-sections">
  Secciones opcionales
</h2>

<h3 id="admin">
  `admin`
</h3>

Opcional. Habilita `/v1/organizations/spend_limits`, que refleja la API pública de administración de Anthropic, y la aplicación de gastos por desarrollador en `/v1/messages`. Consulte [Límites de gastos](/docs/es/claude-apps-gateway-spend-limits) para saber cómo se establecen y aplican los límites; esta sección cubre las claves de `gateway.yaml` que activan la función y la ajustan.

```yaml theme={null}
admin:
  # Claves API estáticas nombradas para los puntos finales de administración, enviadas como x-api-key.
  # El id aparece en el registro de auditoría como admin-key:<id> para que cada clave sea
  # atribuible. Array para rotación: agregue la nueva clave, actualice los clientes,
  # elimine la antigua.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # Grupos de IdP con acceso administrativo completo a través del JWT de gateway normal (sin clave API).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Campo                     | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | No        | Array de `{id, key}`. Una `x-api-key` que coincida con una de estas puede listar, establecer y eliminar límites de gastos. Los valores de clave deben tener al menos 32 caracteres; los `id` deben ser únicos en `read_keys` y `write_keys`.                                                                                                                                                                                                            |
| `read_keys`               | No        | Array de `{id, key}`. Solo lectura: cada punto final `GET`, incluida la enumeración de límites, la obtención de uno por ID y la lectura de [`/effective`](/docs/es/claude-apps-gateway-spend-limits#%2Feffective) y [`/audit`](/docs/es/claude-apps-gateway-spend-limits#%2Faudit).                                                                                                                                                                               |
| `admin_groups`            | No        | Nombres de grupos de IdP. Un JWT de gateway cuya reclamación `groups` incluya uno de estos tiene acceso administrativo completo, lectura y escritura, y auditorías como `oidc:<sub>`. Utilice esto para administradores humanos; utilice claves API para máquinas. Una entrada vacía en esta lista detiene el gateway al iniciar. Consulte [Valores de coincidencia que detienen el gateway al iniciar](#matcher-values-that-stop-the-gateway-at-boot). |
| `blocked_message`         | No        | Se añade textualmente al `429 billing_error` que ve un desarrollador bloqueado. Escriba la instrucción completa, como una URL o un canal de Slack. Cuando no está establecido, el gateway envía solo el mensaje predeterminado. Consulte [Cómo funciona la aplicación](/docs/es/claude-apps-gateway-spend-limits#how-enforcement-works).                                                                                                                     |
| `audit_retention_days`    | No        | Predeterminado `365`. Las filas `admin_audit` más antiguas se eliminan.                                                                                                                                                                                                                                                                                                                                                                                 |
| `spend_retention_months`  | No        | Predeterminado `13`. Las filas del contador `spend` más antiguas que esto se eliminan. El valor predeterminado mantiene un año completo más el mes parcial actual para informes año a año.                                                                                                                                                                                                                                                              |
| `identity_retention_days` | No        | Predeterminado `90`. TTL de última visualización para filas `principal_emails`, que contienen el correo electrónico, nombre para mostrar y grupos de cada desarrollador (PII). Deliberadamente más corto que la retención de gastos para que una identidad desaprovisionada caduque mientras sus contadores de gastos anónimos permanecen.                                                                                                              |
| `group_limit_mode`        | No        | `min` (predeterminado) o `max`. Cuando un desarrollador está en varios grupos con límites, `min` aplica el más restrictivo y `max` el menos restrictivo. Utilizado tanto por la aplicación como por `/effective`.                                                                                                                                                                                                                                       |

<h3 id="enforcement">
  `enforcement`
</h3>

El bloque `enforcement` controla cómo se comportan las comprobaciones de límites de gastos cuando el almacén no está disponible.

| Campo                  | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ---------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | No        | Predeterminado `false`. La aplicación de límites de gastos falla abierta en una interrupción de Postgres, por lo que la inferencia se mantiene activa. Establezca `true` para fallar cerrado: los desarrolladores que superan el límite se bloquean, pero también todos los demás si el almacén es inaccesible. Requiere un bloque [`admin:`](#admin): la aplicación de límites de gastos solo se ejecuta cuando `admin` está configurado, y el gateway se niega a iniciar si establece esto en `true` sin uno. |

<h3 id="pricing">
  `pricing`
</h3>

El bloque `pricing` le dice al medidor de gastos qué cobrar en lugar del precio de lista en USD, para que los límites y [`/effective`](/docs/es/claude-apps-gateway-spend-limits#%2Feffective) reflejen sus tasas contratadas. Los montos permanecen en USD y siguen siendo una estimación, no una factura. Dos requisitos previos:

* Claude Code v2.1.227 o posterior en el servidor de gateway. Las versiones anteriores rechazan la clave desconocida al iniciar.
* Un bloque [`admin:`](#admin) o, en v2.1.268 o posterior, un bloque [`managed:`](#managed) con al menos una política. El gateway se niega a iniciar con `pricing` establecido y ninguno de los dos bloques, porque nada lo leería.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| Campo        | Requerido | Descripción                                                                                                                                                                                                                                                  |
| ------------ | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `multiplier` | No        | Predeterminado `1`. El medidor multiplica cada cantidad medida por esto, ya sea con precio de lista o anulada, por lo que `0.85` factura el 85% del precio. Debe ser mayor que 0 y como máximo 10, y un valor superior a 1 es un [marcado](#mark-prices-up). |
| `overrides`  | No        | Filas de `{upstream, model, input, output, cache_read, cache_write}` en USD por millón de tokens. Las cuatro tasas son obligatorias. Cada una debe ser mayor que 0 y como máximo 10000.                                                                      |

Cómo el medidor coincide con una fila de anulación:

* Una fila reemplaza el precio de lista para solicitudes que `upstream`, un [`upstreams[].name`](#upstreams), sirve para `model`. Esto incluye la tasa más alta de [modo rápido](/docs/es/fast-mode#understand-the-cost-tradeoff), por lo que las solicitudes de modo rápido y estándar se miden con las mismas cuatro tasas.
* Un ID integrado como `claude-sonnet-4-6`, coincidido como [`models[].id`](#models), cubre cada forma fechada, forma regional de Amazon Bedrock, o forma de Google Cloud's Agent Platform que el medidor precifica como ese modelo. Cualquier otra cadena, como un alias o un ARN de perfil de inferencia, coincide con el ID que el cliente envió o la cadena enviada al upstream, sin distinción de mayúsculas y minúsculas.
* Donde las filas se superponen, el medidor elige la fila más específica en lugar de la primera fila: una fila cuyo `model` es la cadena de modelo exacta enviada al upstream, luego una fila que coincide con el ID exacto que el cliente envió, luego una fila que nombra el modelo integrado.
* Un nombre de upstream desconocido falla al iniciar, al igual que dos filas para un upstream que nombran el mismo modelo, incluidas dos ortografías de un modelo integrado. El gateway advierte al iniciar sobre una fila que ningún modelo solicitable puede usar.
* Las solicitudes de búsqueda web permanecen al precio de lista de \$0.01; el multiplicador aún se aplica a ellas.

Para tasas por región, asigne a cada región su propio upstream nombrado y una fila por upstream.

<h4 id="mark-prices-up">
  Marcar precios hacia arriba
</h4>

Con v2.1.271 o posterior en el servidor de gateway, puede establecer `multiplier` por encima de 1, hasta 10, para medir más de lo que cobra el proveedor, por ejemplo una tasa de reembolso interno. Este ejemplo mide cada solicitud al 120% del precio:

```yaml theme={null}
pricing:
  multiplier: 1.2
```

Con un bloque [`admin:`](#admin), el marcado también se aplica a los límites de gastos. El medidor cuenta el 120% del precio, por lo que los desarrolladores alcanzan sus límites más rápido. El gateway registra una advertencia al iniciar que dice así.

El multiplicador no cambia lo que cobra el proveedor upstream por las solicitudes.

Si el gateway también [envía las tasas a clientes conectados](#send-the-rates-to-signed-in-clients), los desarrolladores necesitan Claude Code v2.1.271 o posterior para ver el marcado. Los clientes anteriores ignoran un `multiplier` superior a 1 y muestran costos sin él.

Un servidor de gateway anterior a v2.1.271 se niega a iniciar si establece un `multiplier` superior a 1.

<h4 id="send-the-rates-to-signed-in-clients">
  Enviar las tasas a clientes conectados
</h4>

Con v2.1.268 o posterior en el servidor de gateway, el gateway también coloca las tasas de `pricing` en las políticas [`managed`](#managed) que sirve, como la configuración administrada [`modelPricing`](/docs/es/settings-reference#modelpricing). Los desarrolladores coincididos por una política ven las tasas de `pricing` para el primer upstream que sirve cada ID de modelo en `/usage`, la línea de estado y OpenTelemetry. Un desarrollador que no coincide con ninguna política no recibe configuraciones administradas, por lo que sus cifras permanecen al precio de lista. Los clientes aplican la configuración en Claude Code v2.1.242 o posterior.

* Lo que agrega el gateway: a menos que el bloque `cli` de una política ya establezca `modelPricing`, el gateway agrega el `multiplier` y, para cada ID de modelo que un cliente pueda solicitar, la fila de anulación del primer upstream que sirve ese ID. Una tasa que solo un upstream de conmutación por error cobra permanece en el gateway.
* Optar una política: establezca `modelPricing` en `{}` en el bloque `cli` de esa política, y sus desarrolladores permanecen al precio de lista.
* Mantener las tasas propias de una política: una política cuyo bloque `cli` establece `modelPricing` con su propio `multiplier` u `overrides` mantiene ese `modelPricing` completo, y el gateway no agrega tasas propias a él.

<h3 id="models">
  `models`
</h3>

El bloque `models` es una lista de modelos opcional curada por administrador, servida en `/v1/models` y utilizada para traducir IDs de modelo por upstream. Es obligatorio para regiones de Amazon Bedrock que no sean EE.UU., ARN de rendimiento aprovisionado de Amazon Bedrock y nombres de implementación de Microsoft Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

Cada clave bajo `upstream_model` debe coincidir con el `name` de un upstream configurado, que por defecto es el nombre del proveedor. Una clave que no coincida con ningún upstream falla al iniciar, por lo que omita las líneas para proveedores que no utiliza.

<h3 id="managed">
  `managed`
</h3>

El bloque `managed` define políticas de acceso basadas en roles con clave en grupos de IdP o dominio de correo electrónico. Las políticas se evalúan en orden; se selecciona la primera coincidencia, luego se fusiona en la base de captura general `match: {}`. Se sirven por usuario en `GET /managed/settings` con almacenamiento en caché de ETag/304.

```yaml theme={null}
managed:
  policies:
    # Grupos específicos primero.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Captura general predeterminada al final: coincide con todos los que se autenticaron.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Una captura general `match: {}`, convencionalmente enumerada al final, se trata como una capa base. Cada otra política hereda cualquier clave que no establezca de la captura general, por lo que las entradas por rol solo necesitan enumerar lo que difiere del valor predeterminado de la organización. Las reglas de fusión dependen del tipo de clave:

* **Listas de permitidos**: `availableModels` y `permissions.allow`. La lista de una política específica reemplaza completamente la de la base.
* **Listas de denegados y arrays de hooks**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` y cada array de tipo de evento `hooks`. Estos toman la unión de base y política, por lo que un hook de denegación o auditoría en toda la organización no puede ser eliminado accidentalmente por una anulación por rol.
* **Claves de tipo registro**: `env`, `modelOverrides` y `skillOverrides`. Estas se fusionan superficialmente, por lo que un bloque `env` por rol anula las claves que establece y hereda el resto de la base.

`availableModels` también se aplica en el lado del servidor en `/v1/messages`, por lo que un modelo denegado devuelve `400` independientemente de lo que envíe el cliente.

El gateway valida el valor `model` en sí antes de retransmitir una solicitud, por lo que un valor mal formado nunca llega a un upstream. Rechaza la solicitud con un `400` en dos casos:

* Cuando el valor falta o está vacío, el gateway rechaza la solicitud con el mensaje `model is required`. Esa comprobación requiere un gateway que ejecute Claude Code v2.1.228 o posterior.
* Cuando el valor está presente pero no es una cadena, el gateway rechaza la solicitud con el mensaje `model must be a string`. Requiere un gateway que ejecute Claude Code v2.1.221 o posterior.

| Coincidencia                                        | Comportamiento                                                                                                                                                                                             |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Coincide con cada usuario autenticado. Comience con uno de estos y agregue políticas con alcance de grupo por encima más tarde.                                                                            |
| `match: { groups: [a, b] }`                         | Coincide si la reclamación `groups` del JWT contiene alguno de los grupos enumerados. Sensible a mayúsculas y minúsculas: los grupos deben coincidir con el uso exacto de mayúsculas y minúsculas del IdP. |
| `match: { email_domain: example.com }`              | Coincide con la parte después de la última `@` en la reclamación `email` del JWT, sin distinción de mayúsculas y minúsculas. Acepta un dominio por política.                                               |
| `match: { groups: [a], email_domain: example.com }` | Ambas condiciones deben coincidir                                                                                                                                                                          |

Un usuario autenticado que no coincida con ninguna política obtiene los valores predeterminados del gateway, lo que significa cada modelo en el catálogo y sin configuraciones administradas. Agregue una captura general `match: {}` al final si desea una política predeterminada garantizada.

<Note>
  El gateway no mantiene su propio directorio de usuarios. Autoriza cada solicitud desde el token de IdP del usuario, leyendo la pertenencia al grupo de la reclamación `groups` del token y evaluando políticas contra ella. No hay un registro para enumerar y no hay cuentas para crear previamente, y por lo tanto no hay punto final SCIM, porque no hay nada para que SCIM sincronice.

  Ejecute la gestión del ciclo de vida del usuario y el grupo en la fuente de verdad, que es el aprovisionamiento SCIM nativo de su IdP o una plataforma dedicada de gobernanza de identidades. La pertenencia y desaprovisionamiento gobernados allí fluyen hacia el gateway automáticamente a través del token. Si desea el aprovisionamiento SCIM de las propias cuentas de Claude, esa es una capacidad de [Claude for Enterprise](/docs/es/admin-setup).

  Se aplican dos relojes de propagación:

  * **Contenidos de política**: editar una política y reimplementar llega a clientes conectados en su próxima encuesta de configuraciones administradas, dentro de una hora, aparte de los [cambios que se aplican solo en el próximo lanzamiento](/docs/es/server-managed-settings#fetch-and-caching-behavior)
  * **Pertenencia al grupo**: cambiar la pertenencia al grupo de un usuario cambia qué política los coincide. Esto entra en vigor en el próximo acuñamiento de sesión, lo que significa el próximo refresco silencioso, limitado por `session.ttl_hours`.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  Valores de coincidencia que detienen el gateway al iniciar
</h4>

Al iniciar, el gateway comprueba el bloque `match` de cada política y la lista [`admin_groups`](#admin). Cualquiera de estos valores detiene el gateway con un error que nombra el campo:

* Una lista `groups` vacía
* Una entrada vacía en `groups` o en `admin_groups`
* Un `email_domain` vacío
* Un `email_domain` que contiene `@`, espacios en blanco o una coma. El gateway recorta el valor y elimina una `@` inicial antes de esta comprobación. Escriba un dominio desnudo, como `example.com`.

Antes de v2.1.232, el gateway se iniciaba con estos valores. Cada valor tenía este efecto:

* Un `email_domain` vacío: el gateway omitía la comprobación de dominio, por lo que una política con un `email_domain` vacío y sin lista `groups` coincidía con cada usuario autenticado
* Una lista `groups` vacía: la política no coincidía con nadie
* Un `email_domain` que contiene `@`, espacios en blanco o una coma: la política no coincidía con nadie
* Una entrada vacía en `groups` o en `admin_groups`: la entrada coincidía con un usuario solo cuando la reclamación `groups` del IdP de ese usuario también contenía una entrada vacía. En `admin_groups`, esa coincidencia otorgaba acceso administrativo. Si su lista `admin_groups` nunca contenía una entrada vacía, nadie obtenía acceso administrativo de esta manera.

<h4 id="what-goes-in-cli">
  Qué va en `cli`
</h4>

Cada valor `cli` es un documento completo de `managed-settings.json` de Claude Code, el mismo esquema que implementaría a través de MDM o `/etc/claude-code/managed-settings.json`, expresado aquí como YAML. El CLI aplica el documento entregado en el nivel administrado, por encima de la configuración de usuario y proyecto, en lugar de la configuración administrada por servidor. Por lo tanto, ignora la configuración [restringida a fuentes de política a nivel de SO](/docs/es/server-managed-settings#current-limitations), como `policyHelper` y `wslInheritsWindowsSettings`.

El gateway valida cada documento contra el esquema de configuración del CLI al iniciar, por lo que una clave de nivel superior no reconocida falla al iniciar con un error que nombra cada clave ofensiva. Las partes deliberadamente abiertas del esquema aún aceptan valores arbitrarios, porque clientes más nuevos pueden reconocer entradas que el esquema del gateway no. Estas claves abiertas incluyen `env`, `pluginConfigs` y claves anidadas bajo `permissions`.

Debido a que la validación utiliza el esquema incluido con la versión instalada del gateway, poner una clave de configuración de nivel superior introducida por una versión más nueva de Claude Code en la configuración administrada requiere actualizar primero el gateway. Pruebe una nueva política en un cliente antes de implementarla.

La referencia de clave completa está en [Configuración de Claude Code](/docs/es/settings-reference#all-settings). Las claves que los operadores buscan primero:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Acceso a modelos (también aplicado en el lado del servidor en /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Política de permisos
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Entorno insertado en el proceso CLI. DISABLE_UPDATES bloquea
        # actualizaciones de fondo y manuales; DISABLE_AUTOUPDATER detiene solo
        # actualizaciones de fondo.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Hooks en toda la organización. Los comandos de hooks se ejecutan en máquinas de desarrolladores, no en el
        # gateway, por lo que la ruta debe existir en cada SO cliente en la política.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Clave                                      | Aplicada por  | Efecto                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------------ | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `availableModels`                          | Gateway + CLI | Lista de permitidos de modelos. También se comprueba en `/v1/messages`, por lo que un cliente parcheado no puede omitirlo.                                                                                                                                                                                                                                                                                   |
| `permissions.allow` / `.deny`              | CLI           | Reglas de herramientas y comandos. Consulte [Permisos](/docs/es/permissions).                                                                                                                                                                                                                                                                                                                                     |
| `permissions.disableBypassPermissionsMode` | CLI           | Establezca en `disable` para bloquear [`bypassPermissions`](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode), el modo que omite indicadores de permisos, y la bandera `--dangerously-skip-permissions`                                                                                                                                                                                      |
| `allowManagedPermissionRulesOnly`          | CLI           | Cuando es `true`, la configuración administrada se convierte en la única fuente de configuración de reglas de permisos. La entrada [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly) enumera cada fuente que Claude Code luego ignora.                                                                                                                             |
| `env`                                      | CLI           | Variables de entorno fusionadas en el proceso CLI. Utilice para telemetría, actualización automática y anulaciones de nombres de modelos.                                                                                                                                                                                                                                                                    |
| `hooks`                                    | CLI           | Hooks en toda la organización [hooks](/docs/es/hooks)                                                                                                                                                                                                                                                                                                                                                             |
| `managedMcpServers`                        | CLI           | Servidores MCP remotos [proporcionados a cada desarrollador coincidente](/docs/es/managed-mcp#provide-servers-through-managed-settings) junto con los servidores que agregan ellos mismos, solo `http` y `sse`. Consulte [Servidores MCP en una política](#mcp-servers-in-a-policy). Requiere Claude Code v2.1.259 o posterior en el servidor de gateway y en clientes. Los clientes anteriores ignoran la clave. |

Debido a que estas configuraciones llegan a través de la red, el CLI muestra a cada desarrollador un diálogo de aprobación de seguridad antes de aplicar la configuración enumerada a continuación:

* `hooks`
* Variables `env` que requieren la aprobación del desarrollador, como variables de proxy y URL base
* configuraciones de ejecución de shell como `apiKeyHelper` y `statusLine`
* la configuración de binario de sandbox `sandbox.bwrapPath`, `sandbox.socatPath` y `sandbox.ripgrep`
* Configuraciones de Sandbox que interceptan tráfico, inyectan credenciales o debilitan el aislamiento, como `sandbox.network.tlsTerminate` y la configuración del puerto proxy. [Diálogos de aprobación de seguridad](/docs/es/server-managed-settings#security-approval-dialogs) los enumera todos.

[Memoria de aprobación](/docs/es/server-managed-settings#approval-memory) cubre cuánto tiempo dura una aprobación y cuándo aparece el diálogo nuevamente.

Claude Code aplica algunas variables `env` entregadas sin mostrar al desarrollador el diálogo de aprobación, como configuraciones de selección de modelos y límites numéricos. Otras variables entregadas pueden requerir la aprobación del desarrollador antes de que surtan efecto; un valor de proxy, URL base u `OTEL_EXPORTER_OTLP_ENDPOINT` no vacío siempre lo hace. Cuando una variable entregada necesita aprobación, el diálogo la nombra.

[Variables de entorno y el diálogo de aprobación](/docs/es/server-managed-settings#environment-variables-and-the-approval-dialog) tiene los detalles, incluidos cuatro conmutadores de privacidad cuyo valor entregado decide si necesitan aprobación. Antes de v2.1.218, Claude Code aplicaba menos variables sin preguntar al desarrollador, por lo que más variables entregadas activaban el diálogo.

La configuración de [telemetría](#telemetry) del gateway inserta `OTEL_EXPORTER_OTLP_ENDPOINT`, por lo que establecer `telemetry.forward_to` activa el diálogo en cada cliente interactivo. El diálogo protege la máquina del desarrollador de un gateway comprometido u hostil, no la organización del desarrollador.

Una ejecución no interactiva con la bandera `-p` no puede mostrar el diálogo. Aplica la configuración insertada para esa ejecución solo y no la registra como aprobada, por lo que la próxima sesión interactiva del desarrollador aún muestra el diálogo. Antes de v2.1.207, una ejecución no interactiva guardaba la configuración como aprobada y ninguna sesión interactiva posterior mostraba el diálogo para ellas.

Si un desarrollador rechaza, Claude Code sale de esa sesión en lugar de aplicar la política. Cuando inserta un nuevo hook, o cualquier variable env que active el diálogo, en una política amplia, Claude Code por lo tanto muestra el diálogo a cada desarrollador coincidente. Muestra el diálogo en una sesión en ejecución en la próxima encuesta cada hora, y de lo contrario en el próximo inicio del desarrollador.

La clave `cli` se llamaba `settings` en versiones anteriores. Esa ortografía aún se acepta como un alias, pero las nuevas implementaciones deben usar `cli`.

<h4 id="mcp-servers-in-a-policy">
  Servidores MCP en una política
</h4>

Para proporcionar servidores MCP a los clientes de Claude Code que coincida una política, establezca [`managedMcpServers`](/docs/es/managed-mcp#provide-servers-through-managed-settings) en el bloque `cli` de esa política. Necesita Claude Code v2.1.259 o posterior en el servidor de gateway y en clientes.

El gateway comprueba cada entrada al iniciar con [las mismas reglas que Claude Code aplica en el cliente](/docs/es/managed-mcp#what-an-entry-can-contain), y si una entrada falla una comprobación, el gateway se niega a iniciar y nombra la entrada.

Si escribe una referencia `${VAR}` en `gateway.yaml`, el gateway la resuelve desde su entorno al iniciar a través de [expansión de secretos](#secret-expansion) antes de ejecutar las comprobaciones de entrada, por lo que cada cliente coincidente recibe el valor literal y puede leerlo. La [orientación de encabezado para servidores proporcionados](/docs/es/managed-mcp#provide-servers-through-managed-settings) se aplica al valor expandido.

El gateway rechaza la ortografía `.mcp.json` `mcpServers` en un bloque `cli`, y su error de inicio nombra `managedMcpServers` como la clave a usar. Antes de v2.1.259, el gateway rechazaba cualquier definición de servidor MCP en un bloque `cli`.

<h4 id="claude-desktop-overlay">
  Superposición de Claude Desktop
</h4>

Si su organización también implementa [Claude Desktop](/docs/es/desktop), el mismo gateway sirve a ambos clientes. Apunte `bootstrapUrl`, en la [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop, a `<listen.public_url>/user/bootstrap`. Claude Desktop deriva el emisor de OAuth de esa URL, ejecuta el mismo inicio de sesión de código de dispositivo contra este gateway y obtiene su configuración de la respuesta.

<Note>
  Requiere Claude Code v2.1.203 o posterior en el servidor de gateway, y una opción explícita: `/user/bootstrap` devuelve 404 a menos que la política que coincida con el usuario lleve una clave `desktop`. Un `desktop: {}` vacío opta una política, y una clave `desktop` en la capa base `match: {}` opta en cada política que la hereda. El registro de auditoría registra cada solicitud como `desktop_bootstrap.serve` o `desktop_bootstrap.denied`.
</Note>

El gateway deriva gran parte de la respuesta del bloque `cli` de la política coincidente y de la configuración del gateway de nivel superior:

* La lista de modelos, de `availableModels`
* Herramientas deshabilitadas, de entradas `permissions.deny` de nombre de herramienta desnudo. Si establece `disabledBuiltinTools` en el bloque `desktop` de la política, el gateway sirve la unión de su valor y la lista derivada, por lo que puede deshabilitar más herramientas de esta manera pero no puede volver a habilitar una que deshabilitó a través de `permissions.deny`
* La lista de permitidos de salida, de `sandbox.network.allowedDomains`. Si establece `coworkEgressAllowedHosts` en el bloque `desktop` de la política, el gateway usa ese valor en lugar de la lista derivada
* Un punto final OTLP que apunta al gateway mismo, y los atributos de identidad del usuario conectado. El gateway retransmite las exportaciones que recibe en ese punto final a sus destinos `forward_to`. Incluye el punto final y los atributos cuando establece tanto [`telemetry.forward_to`](#telemetry) como `listen.public_url`.

  Claude Desktop exporta cada señal con una codificación: `http/protobuf`, o `http/json` cuando establece `OTEL_EXPORTER_OTLP_PROTOCOL` o uno de sus variantes por señal en `http/json` en el `env` de la política. Antes de Claude Code v2.1.261 en el servidor de gateway, la respuesta establecía `http/json` independientemente, por lo que un recopilador que acepta solo protobuf rechazaba las exportaciones de Claude Desktop

Para establecer `disabledBuiltinTools`, `coworkEgressAllowedHosts` o la configuración `managedMcpServers` propia de Claude Desktop en el bloque `desktop` de una política, necesita Claude Code v2.1.232 o posterior en el servidor de gateway. El `managedMcpServers` de Claude Desktop toma un valor de array en lugar de un objeto.

El gateway omite claves sin equivalente de Claude Desktop, como `hooks` y reglas de permisos con alcance como `Bash(npm *)`, de la respuesta de bootstrap.

Agregue el bloque `desktop` opcional junto a `cli` para establecer la configuración de Claude Desktop directamente. Escriba la configuración de la [referencia de configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop como nombres de clave planos. Deje fuera las claves que Claude Desktop lee solo de MDM o archivos locales, como `bootstrapUrl`; el gateway las rechaza al iniciar. Antes de v2.1.232, el gateway aceptaba una lista fija de 11 claves de puerta de características, como `chatTabEnabled` y `disableAutoUpdates`, y rechazaba todas las demás claves al iniciar. Antes de v2.1.227, el gateway también rechazaba `chatTabEnabled` y `chatAdvancedFileAnalysisEnabled` al iniciar.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

Cada clave es opcional; Claude Desktop aplica su propio valor predeterminado para cualquier clave que omita. El gateway valida cada bloque `desktop` al iniciar contra el esquema de configuración que el propio Claude Desktop usa, por lo que un error aparece al inicio del gateway como un error que nombra la clave en lugar de llegar a cada desktop conectado. El gateway falla al iniciar cuando un bloque contiene:

* Una clave desconocida
* Una clave reconocida cuyo valor Claude Desktop rechazaría o dejaría caer silenciosamente, como un valor vacío o una subclave mal escrita dentro de una entrada anidada. Antes de v2.1.260, el gateway dejaba caer silenciosamente un campo mal escrito dentro de un objeto anidado de una entrada `managedMcpServers` u `orgPluginSettings` en lugar de fallar al iniciar.
* Una clave que el gateway calcula a sí mismo: la conexión de inferencia, la lista de modelos y el relé OTLP. Configure esos a través de [`upstreams`](#upstreams), [`models`](#models) y la sección [`telemetry`](#telemetry) de `forward_to`.
* Un alias heredado de una clave actual. En el error de inicio, el gateway nombra la clave canónica a escribir.

Si utiliza un valor o forma de entrada obsoleta, como una entrada `managedMcpServers` sin `transport`, el gateway se inicia y registra una advertencia que nombra el reemplazo.

El gateway valida un bloque `desktop` contra el esquema incluido con su versión instalada, como lo hace con el bloque `cli`. Para entregar una configuración introducida por una versión más nueva de Claude Desktop, actualice primero el gateway. Por ejemplo, `userPluginMarketplacesEnabled` y `userPluginUploadsEnabled` necesitan Claude Code v2.1.260 o posterior en el servidor de gateway y Claude Desktop 1.37937.0 o posterior en las máquinas de los miembros.

Si establece `orgPluginSettings` en el bloque `desktop` de una política, el gateway lo sirve en la forma de array que Claude Desktop 1.15200.0 y posterior lee. Los desktops más antiguos ignoran el array y no aplican ninguna política de herramientas de plugins, por lo que actualice a los miembros a 1.15200.0 o posterior antes de confiar en ello.

El gateway rellena las claves que el bloque `desktop` de una política no establece desde el bloque `desktop` de la captura general `match: {}`, de la misma manera que rellena el bloque `cli` de una política desde la base. Si establece `disabledBuiltinTools` o `builtinToolPolicy` en la base y una política de rol, el gateway mantiene la restricción de la base:

* `disabledBuiltinTools`: el gateway usa la unión de la lista de la base y la lista de la política
* `builtinToolPolicy`: si establece una herramienta en un valor distinto de `allow` en la base, el gateway mantiene ese valor incluso si establece `allow` para la misma herramienta en una política de rol

Para todas las demás claves, si las establece en la política de rol, el gateway usa el valor de la política de rol. El gateway reemplaza un array o un objeto anidado como `banner` completo, por lo que si establece `banner.text` en una política de rol, el gateway descarta el `banner.backgroundColor` de la base.

Si no implementa Claude Desktop, deje `desktop` completamente fuera de sus políticas; el gateway luego devuelve 404 desde `/user/bootstrap` para cada usuario.

<h4 id="precedence-with-other-managed-sources">
  Precedencia con otras fuentes administradas
</h4>

Si un dispositivo también tiene una política entregada por MDM o un `managed-settings.json` local, la configuración entregada por gateway ocupa el primer lugar. [Precedencia dentro del nivel administrado](/docs/es/managed-settings#precedence-within-the-managed-tier) en la página de configuraciones administradas dice cuándo se aplican las fuentes locales, y tiene las [claves que Claude Code lee de cada fuente de administrador](/docs/es/managed-settings#keys-read-from-every-admin-source) independientemente de qué fuente seleccionó, como las claves de bloqueo de sandbox, `forceRemoteSettingsRefresh` y el `env` por variable. Un [`policyHelper`](/docs/es/settings-reference#policyhelper) configurado en un perfil MDM o el archivo de configuraciones administradas se ejecuta solo cuando el gateway no entrega configuraciones; la entrada dice qué reemplaza su salida.

Los hosts de incrustación como [Claude Desktop](/docs/es/desktop) pueden suministrar política a través de la opción SDK `managedSettings`. [Configuraciones principales de hosts de incrustación](/docs/es/managed-settings#parent-settings-from-embedding-hosts) dice cuándo Claude Code la aplica, y [Restringir configuraciones principales](/docs/es/claude-apps-gateway#restrict-parent-settings) enumera qué configuraciones de dirección de permitidos aún se aplican sin los bloqueos `allowManaged*Only`.

Las políticas de gateway se aplican a cada invocación de Claude Code en la máquina, incluidas ejecuciones no interactivas `claude -p` y sesiones generadas por el SDK de Agent. Si el gateway es inaccesible al iniciar, las sesiones conectadas salen con un error en lugar de ejecutarse sin su política.

<h3 id="telemetry">
  `telemetry`
</h3>

El CLI envía métricas, registros y, cuando está habilitado, trazas al gateway, que las retransmite textualmente a cada destino configurado. Las exportaciones utilizan OpenTelemetry Protocol (OTLP) sobre HTTP. Para omitir el relé y hacer que las sesiones exporten directamente a su recopilador, [nombre el recopilador en una política](#export-directly-to-your-collector). Consulte [Monitoreo de uso](/docs/es/monitoring-usage) para las métricas y eventos que emite el CLI.

El CLI marca cada exportación con la identidad del usuario autenticado, leída del JWT emitido por el gateway: los atributos `user.id`, `user.email` y `user.groups`. La atribución de costo y uso por desarrollador funciona sin configuración en el lado del desarrollador.

[Claude Desktop](#claude-desktop-overlay) y las sesiones de Cowork conectadas a través del gateway marcan su telemetría con `user.email` y `user.groups` junto a `enduser.id`, por lo que puede cubrir el uso de terminal, Desktop y Cowork con una consulta en `user.email` o `user.groups`. `user.groups` es la lista de grupos de IdP separada por comas.

La telemetría de Desktop y Cowork también lleva `enduser.sub`, la reclamación `sub` que su proveedor de identidades emite para el usuario, que permanece igual cuando cambia el correo electrónico de un usuario. Las sesiones de terminal marcan el mismo valor bajo `user.id`, por lo que una consulta que coincida con `enduser.sub` contra `user.id` de terminal cubre el uso de terminal, Desktop y Cowork de un usuario junto. En las exportaciones de Desktop y Cowork, `user.id` es un identificador anónimo, no el sujeto.

Como todos los datos de OpenTelemetry de Claude Code, estos atributos van solo a destinos que su organización configura, nunca a Anthropic.

Si la lista de grupos de un usuario es más larga que 255 caracteres una vez codificada en porcentaje, o un nombre de grupo contiene una coma o un signo igual, el gateway deja `user.groups` fuera de la telemetría de Desktop y Cowork de ese usuario en lugar de truncarla. Las sesiones de terminal de ese usuario aún llevan la lista completa.

El gateway deja `enduser.sub` cuando el sujeto es más largo que 255 caracteres una vez codificado en porcentaje, o contiene un espacio, un carácter fuera de ASCII imprimible, o uno de `,` `;` `=` `\` `"` `%`. La telemetría de Desktop y Cowork de ese usuario mantiene sus otros atributos.

Necesita Claude Code v2.1.265 o posterior en el servidor de gateway para `user.email` y `user.groups` en la telemetría de Desktop y Cowork, y Claude Desktop 1.24012 o posterior en la máquina de cada desarrollador para `user.groups`.

Necesita Claude Code v2.1.274 o posterior en el servidor de gateway para `enduser.sub`.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Opción por señal. Predeterminado: solo métricas.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Cada destino opta en `metrics`, `logs` y `traces` independientemente, y el valor predeterminado es solo métricas. Las señales difieren en sensibilidad:

  * **Métricas**: contadores agregados como conteos de tokens, conteos de solicitudes y latencia
  * **Registros y trazas**: pueden llevar comandos Bash completos, entradas de herramientas y rutas de archivos, cubriendo cualquier cosa que Claude Code haga en la máquina de un desarrollador

  Habilite registros y trazas solo en destinos con los controles de acceso y la política de retención que esos datos justifican.
</Warning>

Cada URL `forward_to` debe usar `https://`, con una excepción para un recopilador en la interfaz de loopback del gateway:

* `http://localhost:<port>` pasa la validación de configuración, pero la [guardia SSRF](/docs/es/claude-apps-gateway-deploy#threat-model-summary) bloquea cada exportación con `ECONNREFUSED_SSRF` a menos que establezca `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` en el entorno del gateway
* `http://127.0.0.1:<port>` o `http://[::1]:<port>` falla al iniciar a menos que esa variable esté establecida

Para un recopilador en el clúster, expóngalo sobre HTTPS en su propia dirección interna, o ejecútelo como un sidecar con la variable establecida.

Cuando `HTTPS_PROXY` está establecido, el gateway envía exportaciones a través de ese proxy.

Para llegar a un recopilador interno directamente, agréguelo a `NO_PROXY` por nombre de host o por un dominio con un punto inicial como `.internal.example.com`, que requiere Claude Code v2.1.277 o posterior en el servidor de gateway. Asegúrese de que el gateway pueda llegar al recopilador sin el proxy. Una entrada sin un punto inicial coincide solo con ese nombre exacto, no con nombres bajo él. Los rangos CIDR no coinciden.

Con [salida solo proxy](#proxy-only-egress) activada, permita el recopilador en el proxy en su lugar, ya que cualquier entrada `NO_PROXY` mantiene la salida solo proxy desactivada.

La telemetría está desactivada en el CLI de forma predeterminada. Cuando establece tanto `telemetry.forward_to` como `listen.public_url`, el gateway la activa para clientes conectados insertando seis variables de entorno a través de `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` y `OTEL_TRACES_EXPORTER`, cada uno establecido en `otlp` si al menos un destino `forward_to` habilita esa señal y en `none` de lo contrario
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

Antes de Claude Code v2.1.265 en el servidor de gateway, el gateway insertaba los tres selectores de exportador como `otlp`, incluido para señales que ningún destino optó.

El punto final insertado se construye a partir de la URL pública, por lo que las métricas y registros no necesitan configuración OTEL de desarrolladores o políticas.

Los desarrolladores conectados a través de `/login` no pueden redirigir exportaciones con su propia configuración OTEL:

* **Variables establecidas localmente**: Claude Code aplica las variables insertadas en el nivel administrado, por lo que cada una anula el valor que un desarrollador establece localmente.
* **Puntos finales configurados localmente**: con la exportación OTLP/HTTP habilitada, el CLI ignora cualquier punto final configurado localmente, independientemente de si el gateway insertó las variables de telemetría. Sus exportaciones van al gateway a menos que una política [nombre su recopilador como punto final](#export-directly-to-your-collector).

Sin un destino `forward_to` para una señal, el gateway la acepta y la descarta. Si los desarrolladores ya exportan telemetría de Claude Code a uno de sus recopiladores, agréguelo como destino `forward_to`, con registros o trazas habilitadas si exportan esos, para que continúe recibiendo sus datos después de que se conecten. Para omitir el relé en su lugar, [nombre el recopilador en una política](#export-directly-to-your-collector).

[Trazas](/docs/es/monitoring-usage#traces-beta) también requieren `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` en cada cliente. Establézcalo en el bloque `env` de una política administrada, ya que el gateway no lo inserta. Los desarrolladores lo aprueban en el mismo [diálogo de aprobación de seguridad](#managed) que el punto final insertado ya activa.

Establézcalo en `1` solo en las políticas cuyos grupos desea rastrear. Una política que no lo establece hereda el valor de su política de captura general `match: {}` si esa política establece uno, según las [reglas de fusión](#managed). Para evitar que los clientes de un grupo envíen trazas incluso cuando un desarrollador establece la variable localmente, establézcala en `0` en la política de ese grupo.

Tanto las codificaciones OTLP de protobuf como JSON se retransmiten, y cualquier backend compatible con OpenTelemetry funciona como destino.

<h4 id="export-directly-to-your-collector">
  Exportar directamente a su recopilador
</h4>

Para hacer que las sesiones conectadas a través de `/login` envíen telemetría directamente a su recopilador en lugar de a través del relé, establezca `OTEL_EXPORTER_OTLP_ENDPOINT` en la URL base `https://` del recopilador en el bloque `env` de una [política administrada](#managed). Claude Code añade `/v1/metrics`, `/v1/logs` o `/v1/traces` a la URL que establece, como `https://otel-collector.example.com:4318`, y exporta cada señal allí sobre OTLP/HTTP. Requiere Claude Code v2.1.265 o posterior en la máquina de cada desarrollador. Los clientes anteriores exportan a través del relé.

Para autenticarse en el recopilador, establezca `OTEL_EXPORTER_OTLP_HEADERS` en el mismo bloque `env`. Las sesiones nunca envían el token de sesión de gateway del desarrollador a un recopilador nombrado de esta manera.

Cuando agrega o cambia este punto final en una política, Claude Code pide a cada desarrollador que lo apruebe en el [diálogo de aprobación de seguridad](#managed) antes de aplicarlo en una sesión interactiva.

Claude Code comprueba el punto final antes de exportar una señal directamente, y mantiene esa señal en el relé cuando una comprobación falla. Las comprobaciones incluyen:

* El punto final proviene del gateway mismo. Si establece la misma variable en un perfil MDM o un `managed-settings.json` local, las exportaciones permanecen en el relé.
* La URL usa `https://`, o `http://` a una dirección de loopback
* La URL se resuelve en una ruta que termina en `/v1/<signal>`, sin consulta o fragmento. Claude Code construye esa ruta a sí mismo desde la variable genérica. Utiliza una variable por señal como `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` tal como está escrita, por lo que incluya la ruta completa allí.
* La URL no es el host del gateway. Un punto final dirigido al gateway mantiene la ruta de relé y su token de sesión.
* Ni usted ni el desarrollador han configurado [`otelHeadersHelper`](/docs/es/settings-reference#otelheadershelper) en ninguna fuente de configuración. Con un ayudante configurado, cada señal permanece en el relé.

El punto final que nombra cambia solo dónde van las exportaciones. Aún elige qué señales exportan en absoluto con los selectores `OTEL_*_EXPORTER`.

El punto final solo no activa la exportación, por lo que también establezca las variables que lo hacen, a menos que el gateway ya las inserte:

* Si el gateway ya [inserta las variables de telemetría](#telemetry), cubren habilitación, selectores y protocolo, y su punto final explícito anula el valor `<public_url>` insertado. Establezca un selector `OTEL_*_EXPORTER` en `otlp` usted mismo solo para una señal que ningún destino `forward_to` habilita.
* Si no, también establezca `CLAUDE_CODE_ENABLE_TELEMETRY=1`, los selectores `OTEL_*_EXPORTER` y `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`.

Cuando el desarrollador se desconecta, o se conecta a un gateway diferente, las exportaciones al recopilador se detienen y Claude Code descarta cada lote restante en lugar de enviarlo.

<h4 id="when-a-destination-fails">
  Cuando un destino falla
</h4>

El gateway no almacena en búfer, reintenta ni almacena telemetría, por lo que descarta una exportación que no llega a un destino en lugar de entregarla tarde. Cada destino tiene éxito o falla por su cuenta, y el cliente exportador recibe una respuesta de éxito de cualquier manera, por lo que una entrega fallida aparece solo en el registro del gateway.

Después de cinco entregas consecutivas fallidas a un destino, el gateway pausa el reenvío a él en tramos de 30 segundos, registrando cada pausa, hasta que una entrega tiene éxito. Cualquier respuesta de error, tiempo de espera o error de conexión cuenta como una entrega fallida, excepto `400`, `413`, `415`, `422` y `431`, que significan que el recopilador rechazó la carga útil de esa exportación como mal formada o demasiado grande.

Una carga útil rechazada ni avanza ni reinicia el contador de fallos: el gateway continúa reenviando al destino y registra una advertencia que lo nombra y el estado, en el primer rechazo del destino y cada centésimo después.

<h3 id="http-tuning">
  Ajuste HTTP
</h3>

Cuatro bloques opcionales de nivel superior, `access_control`, `limits`, `timeouts` y `rate_limits`, ajustan la superficie HTTP. Los valores predeterminados se adaptan a la mayoría de implementaciones.

| Bloque           | Clave                                          | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------------- | ---------------------------------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vacío          | Permitir/denegar IP de entrada por dirección de cliente, después de la resolución de `trusted_proxies`. `deny_cidrs` se comprueba primero; un cliente que coincida se rechaza incluso si `allow_cidrs` también coincide. Si `allow_cidrs` no está vacío, el gateway es denegación predeterminada. `/healthz` y `/readyz` están exentos de `allow_cidrs`. Cuando un proxy de confianza envía una entrada `X-Forwarded-For` que no es una dirección IP, el cliente real es desconocido y el gateway registra una advertencia una vez nombrando qué verificar. Donde se aplica cualquiera de las listas a la solicitud, la rechaza con `403` y razón de auditoría `xff_unparseable`. Donde ninguno lo hace, sirve la solicitud y usa la dirección del proxy como dirección IP del cliente para límites de velocidad por IP y auditoría. |
| `limits`         | `max_request_bytes`                            | 32 MiB         | Cuerpo de solicitud de entrada máximo; las solicitudes de tamaño excesivo obtienen `413` antes de que el cuerpo se almacene en búfer. Aumente para solicitudes de archivo o imagen grandes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `limits`         | `max_request_header_bytes`                     | sin establecer | Cuando se establece, los encabezados de tamaño excesivo devuelven `431`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `limits`         | `max_url_length`                               | sin establecer | Cuando se establece, una URL demasiado larga devuelve `414`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000         | Espera máxima para los encabezados de respuesta del upstream (tiempo hasta el primer byte). El cuerpo de respuesta luego se transmite sin límite de reloj de pared. Se aplica a la ruta de upstream de Anthropic directo; en todos los demás proveedores, el gateway espera hasta una hora a que comience la respuesta.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600       | Límite de velocidad por IP en el punto final de autorización de dispositivo no autenticado. Aumente para una organización grande detrás de una dirección IP de salida compartida o NAT. [Implementaciones grandes](/docs/es/claude-apps-gateway-deploy#large-rollouts) muestra cómo dimensionarlo. Estos límites se aplican solo al flujo de inicio de sesión de concesión de dispositivo, no a la inferencia `/v1/messages`. Consulte [Resistencia de fuerza bruta de código de usuario](/docs/es/claude-apps-gateway-deploy#user-code-brute-force-resistance).                                                                                                                                                                                                                                                                               |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600       | Límite de velocidad por IP en envíos de `user_code` en `/device`. Es lo que detiene a alguien de adivinar el código de otro desarrollador. [Implementaciones grandes](/docs/es/claude-apps-gateway-deploy#large-rollouts) muestra cuánto aumentarlo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

Si deja ambas listas `access_control` vacías, que es el valor predeterminado, el gateway sirve cualquier dirección de cliente, por lo que solo su red restringe quién puede alcanzarlo. Eso importa porque un gateway puede insertar [configuraciones administradas](#managed) que ejecutan comandos en máquinas de desarrolladores.

Mientras `allow_cidrs` esté vacío, el gateway advierte en dos lugares, sin cambiar cómo responde a ninguna solicitud:

* **Al iniciar**: una advertencia en el registro operacional recomienda permitir solo los rangos privados `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128` y `fc00::/7`, más cualquier otro rango interno desde el que se conectan sus desarrolladores. Si vincula el gateway a una dirección de loopback y no establece ni `trusted_proxies` ni `public_url`, como en desarrollo local, la advertencia no aparece.
* **En tiempo de ejecución**: la primera vez que llega una solicitud desde una dirección fuera de esos rangos privados, el gateway registra una advertencia y emite un evento de auditoría [`access.public_client`](/docs/es/claude-apps-gateway-deploy#logs) que lleva la dirección IP del cliente. Ambos se disparan una vez por proceso. Las direcciones de enlace local, `169.254.0.0/16` y `fe80::/10`, no cuentan como públicas. El gateway responde `/healthz` y `/readyz` antes de que se ejecute esta comprobación, por lo que los sondeos de salud desde rangos públicos no la activan.

Ambas señales utilizan la dirección del cliente tal como la resuelve el gateway. Si un equilibrador de carga, reenvío de puerto o túnel retransmite tráfico y no está enumerado en `listen.trusted_proxies`, el gateway ve la dirección del relé, que generalmente es privada, por lo que ni la advertencia en tiempo de ejecución ni una lista de permitidos privada lo detecta.

Detrás de tal front end, establezca primero [`listen.trusted_proxies`](#listen) para que el gateway vea direcciones de cliente reales, y mantenga el gateway y todo lo que está frente a él inaccesible desde la internet pública independientemente.

<h3 id="load_test_mode">
  `load_test_mode`
</h3>

El bloque `load_test_mode` le permite hacer pruebas de carga en un gateway sin llamar a un proveedor de modelos. Mientras está activado, el gateway construye y firma cada solicitud de proveedor como de costumbre, la descarta en lugar de enviarla, y transmite una respuesta enlatada a través de su ruta de respuesta normal. La respuesta es texto de relleno que comienza con una oración que dice que es enlatada.

Requiere v2.1.283 o posterior. Las versiones anteriores se niegan a iniciar cuando la clave está establecida, por lo que actualice cada réplica antes de agregar el bloque y elimínelo antes de revertir.

El ejemplo a continuación activa el modo con los valores predeterminados, una respuesta de aproximadamente 750 tokens de salida transmitida durante aproximadamente 10 segundos:

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| Campo           | Requerido | Descripción                                                                                                                                                                                   |
| --------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`       | Sí        | `true` activa el modo. `false` mantiene sus números en el archivo con el modo desactivado. El gateway se niega a iniciar si el bloque está presente sin él.                                   |
| `reply_tokens`  | No        | Predeterminado `750`. Aproximadamente cuántos tokens de texto lleva cada respuesta enlatada, un número entero de 1 a 100000.                                                                  |
| `reply_seconds` | No        | Predeterminado `9.5`. Cuánto tiempo tarda una respuesta transmitida, de 0 a 600. `0` envía toda la respuesta a la vez. Una respuesta a una solicitud sin transmisión siempre vuelve a la vez. |

Una prueba de carga en este modo cubre el gateway, su Postgres y todo lo que está frente al gateway. No cubre los límites, velocidad o ruta de red del proveedor.

Mientras el modo está activado, una solicitud puede llevar un encabezado `x-load-test-user` que contenga un número entero de hasta siete dígitos, y el gateway cuenta cada número como un desarrollador separado con el correo electrónico y grupos del desarrollador cuyo token vino con la solicitud. Asigne a la implementación de prueba de carga su propia base de datos vacía, porque el gateway se niega a iniciar con el modo activado contra una base de datos en la que algún desarrollador ya ha gastado algo.

<Warning>
  Nunca active esto para un gateway que los desarrolladores usan. Cada solicitud obtiene la respuesta enlatada y ningún modelo se llama. El gateway registra una advertencia `load_test_mode is on` al iniciar y marca cada evento de auditoría [`inference`](/docs/es/claude-apps-gateway-deploy#logs) con `load_test: true` mientras el modo está activado.
</Warning>

<h2 id="complete-example">
  Ejemplo completo
</h2>

Esta configuración de referencia completa ejercita cada sección central; los [bloques de ajuste HTTP](#http-tuning) mantienen sus valores predeterminados. Cópiela, elimine lo que no necesite y complete sus valores. La configuración en el [Inicio rápido](/docs/es/claude-apps-gateway#quickstart) es una versión mínima de esta.

```yaml gateway.yaml theme={null}
# Ejecutar con:
#   claude gateway --config gateway.yaml
#
# La verbosidad del registro operativo se controla mediante la variable de entorno
# CLAUDE_GATEWAY_LOG_LEVEL (debug | info | warn | error; predeterminado info). debug
# también registra los nombres de reclamaciones en cada id_token, para diagnóstico de groups_claim.
# No afecta los eventos de auditoría, que siempre se emiten.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omita el bloque tls cuando se ejecute detrás de una entrada que termina TLS.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Requerido cuando el emisor es el servidor de la organización Okta, cuyos id_tokens
  # pueden omitir correo electrónico y grupos; la puerta de enlace los completa desde /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emite grupos solo cuando se solicita el alcance `groups` y el
  # filtro de reclamación de grupos de la aplicación los permite. La política de contratistas a continuación
  # coincide con grupos, por lo que el alcance se solicita aquí.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Roles de aplicación de Entra: use `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5
  # connect_timeout_seconds: 5

# Habilita /v1/organizations/spend_limits (refleja la API de administrador de Anthropic)
# y cumplimiento de gasto por desarrollador en /v1/messages. Omita para deshabilitar.
# Los límites en sí se establecen a través de la API de administrador, no aquí.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

# Prueba de carga de esta implementación sin llamar a un proveedor de modelo. Nunca en una
# puerta de enlace que usen desarrolladores: cada solicitud obtiene una respuesta enlatada.
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# Medir a tasas contratadas en lugar del precio de lista en USD. Requiere admin: o una
# política managed:. Con managed:, las mismas tasas también van a clientes que han iniciado sesión.
# Las tasas a continuación son marcadores de posición, no precios de contrato reales.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Restrinja la opción del selector predeterminado a availableModels en lugar de
        # el predeterminado de nivel, para que los contratistas no obtengan un 400 en el predeterminado.
        enforceAvailableModels: true
        # allow aprueba automáticamente estas herramientas; no bloquea el resto.
        # Agregue reglas de negación para restringir herramientas.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  Configuración administrada del lado del cliente
</h2>

Todo lo anterior configura el servidor de puerta de enlace. Apuntar máquinas de desarrollador a la puerta de enlace se configura por separado, en cada dispositivo, a través de la [configuración administrada](/docs/es/managed-settings) de Claude Code. La puerta de enlace no puede empujar las claves de inicio de sesión por sí misma, porque son lo que le dice al cliente dónde está la puerta de enlace.

Para el CLI, establezca estas claves en el `managed-settings.json` por SO. Las dos claves de inicio de sesión enrutan el `/login` de cada desarrollador a su puerta de enlace:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` mantiene el funcionamiento de la entrega de Claude Desktop de la lista de permitidos de salida a sus sesiones de Claude Code integradas; [Entregar política a sesiones de Claude Desktop](/docs/es/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) explica el mecanismo y dónde debe estar la aceptación.

Implemente el archivo `managed-settings.json` en cada dispositivo, típicamente a través de su plataforma MDM. La ruta del archivo difiere por plataforma. Consulte [dónde almacena cada mecanismo la política](/docs/es/managed-settings#where-each-mechanism-stores-the-policy).

De forma predeterminada, una política de registro en Windows o una plist de preferencias administradas en macOS reemplaza el archivo `managed-settings.json` en lugar de fusionarse con él, aparte de las [claves de excepción y verificaciones entre fuentes anteriores](#precedence-with-other-managed-sources). Las tres claves en este fragmento siguen la regla de fuente de prioridad más alta, por lo que las flotas que entregan política a través de Política de grupo o perfiles de configuración deben poner las tres en ese mecanismo en su lugar.

Para Claude Desktop, establezca la clave `bootstrapUrl` en la propia [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop en `<listen.public_url>/user/bootstrap`. El flujo de inicio de sesión y la política por grupo coinciden entonces con los del CLI una vez que una política se acepta del lado del servidor con una clave `desktop`; sin la aceptación, `/user/bootstrap` devuelve 404. Consulte [Superposición de Claude Desktop](#claude-desktop-overlay) para la mitad del lado del servidor.

Claude Code honra [`forceLoginGatewayUrl`](/docs/es/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/es/settings-reference#gatewayinternalnetworks), y el valor `"gateway"` de [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod) solo desde una fuente administrada en la máquina: `managed-settings.json`, la plist de macOS o el registro HKLM de Windows, o un asistente de política. Un desarrollador que los establezca en su propio `~/.claude/settings.json` no tiene efecto, y tampoco lo hace establecerlos en la carga útil de la puerta de enlace.

<h2 id="related">
  Relacionado
</h2>

* [Descripción general de la puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway): inicio rápido y conexión de desarrollador
* [Guía de implementación](/docs/es/claude-apps-gateway-deploy): configuración de IdP, imagen de contenedor, Kubernetes y Cloud Run, y operaciones
* [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits): límites por desarrollador y la API de administración
