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
* [`models`](#models) y `auto_include_builtin_models`: lista de modelos curada por administrador e IDs por upstream
* [`managed`](#managed): políticas de configuración administradas por grupo de IdP
* [`telemetry`](#telemetry): reenvío OTLP a su pila de observabilidad
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): permitir/denegar IP, límites de tamaño de solicitud, tiempo hasta el primer byte del upstream y límites de inicio de sesión por IP

<h2 id="secret-expansion">
  Expansión de secretos
</h2>

No escriba secretos como `client_secret`, `jwt_secret` o `postgres_url` directamente en `gateway.yaml`. Haga referencia a ellos con uno de los formularios a continuación, y la puerta de enlace resuelve el valor al arrancar desde una variable de entorno o un archivo:

| Formulario      | Se resuelve a                                                        | Usar para                                                                         |
| --------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `${VAR}`        | La variable de entorno `VAR`. El arranque falla si no está definida. | Variables de entorno de contenedor, AWS Secrets Manager mediante inyección de env |
| `${file:/path}` | Contenido del archivo, recortado                                     | Montajes de volumen de secreto de Kubernetes, Vault Agent, SOPS                   |

<h2 id="required-sections">
  Secciones requeridas
</h2>

<h3 id="listen">
  `listen`
</h3>

El bloque `listen` controla dónde sirve la puerta de enlace: la dirección de enlace y puerto, el origen visible externamente y la terminación TLS opcional.

| Campo                  | Requerido          | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------- | ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | No                 | Dirección de enlace. Predeterminado `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `port`                 | No                 | Puerto de enlace. Predeterminado `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `public_url`           | Detrás de un proxy | El origen `https://` visible externamente, utilizado para construir el `redirect_uri` de IdP y metadatos de descubrimiento. Requerido detrás de cualquier proxy que termine TLS como ALB, Ingress o Cloud Run, porque la puerta de enlace no confía en los encabezados `X-Forwarded-*` al construir su propio origen; son suplantables por el cliente. `trusted_proxies` a continuación rige solo la resolución de IP del cliente. También es necesario para habilitar [telemetría](#telemetry), porque la puerta de enlace construye el punto final OTLP que envía a los clientes desde esta URL. |
| `tls.cert` / `tls.key` | No                 | Rutas PEM si la puerta de enlace termina TLS por sí misma                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `trusted_proxies`      | No                 | CIDR o IPs de equilibradores de carga frente a la puerta de enlace. Cuando se establece, la puerta de enlace confía en `X-Forwarded-For` solo desde estos pares y registra la IP del cliente real para límites de velocidad por IP y auditoría. Equivalente a nginx `set_real_ip_from`.                                                                                                                                                                                                                                                                                                            |

<h3 id="oidc">
  `oidc`
</h3>

El bloque `oidc` conecta la puerta de enlace a su proveedor de identidad y decide quién puede iniciar sesión. Nombra el emisor y cliente OAuth, mapea las reclamaciones que llevan correo electrónico y grupos, y restringe el inicio de sesión por dominio de correo electrónico o grupo.

OpenID Connect (OIDC) es el protocolo SSO que la puerta de enlace utiliza con su proveedor de identidad; consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para saber qué registrar en el lado de IdP.

| Campo                           | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Sí        | Base de descubrimiento OIDC. Debe servir descubrimiento en `/.well-known/openid-configuration`. Use HTTPS en producción; la puerta de enlace acepta un emisor `http://`. Un emisor de bucle local como `http://localhost:8081` es rechazado por la [protección SSRF](/docs/es/claude-apps-gateway-deploy#threat-model-summary) a menos que `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` esté establecido en el entorno de la puerta de enlace.                                                                                                                                                                                                                                                                                                                                              |
| `client_id` / `client_secret`   | Sí        | De su registro de cliente OAuth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowed_email_domains`         | No        | Rechace id\_tokens cuya reclamación `email` no esté en uno de estos dominios, sin distinción de mayúsculas y minúsculas. Defensa en profundidad contra configuración errónea de IdP multiinquilino. Independientemente de esta configuración, un id\_token cuya reclamación `email_verified` es explícitamente `false` siempre se rechaza.                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowed_groups`                | No        | Restrinja el inicio de sesión a miembros de estos grupos de IdP, comparados contra `groups_claim`. Un usuario en un dominio de correo electrónico permitido pero en ninguno de estos grupos es rechazado. Requiere que IdP emita la reclamación de grupos.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `groups_claim`                  | No        | Qué reclamación de id\_token lleva la membresía del grupo. Predeterminado `groups`. Microsoft Entra emite roles de aplicación bajo `roles`. Acepta una clave plana o un puntero JSON RFC 6901 como `/resource_access/gateway/roles` para reclamaciones anidadas.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `google_groups`                 | No        | Busque los grupos del usuario que inició sesión a través de la API del Directorio del SDK de administración de Google Workspace, porque el id\_token de Google no lleva reclamación de grupos. Establezca `service_account_json_path` en un archivo de clave de cuenta de servicio con delegación en todo el dominio en el alcance `https://www.googleapis.com/auth/admin.directory.group.readonly`, y `admin_email` en un administrador de Workspace que la cuenta de servicio suplanta; la API del Directorio requiere un asunto administrador real. Las direcciones de correo electrónico del grupo de cada usuario se convierten en su reclamación de grupos, por lo que `allowed_groups` y `managed.policies.match.groups` coinciden en correos electrónicos de grupo. |
| `email_claim`                   | No        | Qué reclamación de id\_token lleva el correo electrónico del usuario. Predeterminado `email`. Algunos IdP, como ADFS y Entra B2C, emiten `upn` o `preferred_username` en su lugar. Acepta una clave plana, un puntero JSON o una lista de claves de respaldo donde se utiliza la primera clave presente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `scopes`                        | No        | Anulación completa de los alcances OIDC que solicita la puerta de enlace. Predeterminado `[openid, profile, email, offline_access]`. Establezca cuando su IdP rechace alcances que no reconoce, o requiera un alcance personalizado para emitir grupos o correo electrónico. Debe incluir `openid`. Soltar `offline_access` desactiva los tokens de actualización, por lo que los desarrolladores vuelven a ejecutar el inicio de sesión del navegador cada `session.ttl_hours`. Consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para recetas de alcance por IdP como el flujo de token de actualización de Google.                                                                                             |
| `extra_auth_params`             | No        | Parámetros de consulta adicionales añadidos a la solicitud de autorización de IdP, textualmente. Este es el mecanismo de anulación para comportamiento específico de IdP, como `access_type: offline` para tokens de actualización de Google, `domain_hint` para algunos inquilinos de Entra, o `acr_values` para flujos de escalada. No puede anular los parámetros de protocolo administrados por la puerta de enlace: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` y `client_id`.                                                                                                                                                                                                                                                   |
| `userinfo_fallback`             | No        | Cuando el id\_token omite correo electrónico o grupos, búsquelos en `/userinfo`. Necesario para tokens de acceso ligeros de Keycloak, el servidor org de Okta y tokens mínimos de ADFS. El id\_token sigue siendo autoritario; userinfo solo llena vacíos. Predeterminado `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `use_pkce`                      | No        | Envíe un desafío PKCE (S256) en la solicitud de autorización. Predeterminado `true`. Establezca `false` solo si su IdP rechaza PKCE para este cliente confidencial.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `clock_skew_seconds`            | No        | Tolere la desviación del reloj al validar reclamaciones de tiempo de id\_token. Predeterminado `0`, que es estricto. Aumente si ve errores "token expirado / aún no válido" justo después del inicio de sesión debido a desviación del reloj de host/IdP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `token_endpoint_auth_method`    | No        | Anule el método de autenticación del punto final del token. Acepta `client_secret_basic` o `client_secret_post`. Negociado automáticamente de forma predeterminada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `id_token_signed_response_alg`  | No        | Algoritmo de firma de id\_token esperado. Predeterminado `RS256`. Establezca para IdP que firman con ES256, PS256 o EdDSA.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `additional_authorized_parties` | No        | Valores `azp` adicionales para aceptar más allá de `client_id`, para flujos de intermediario de Keycloak e intercambio de tokens                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `discovery_url`                 | No        | Busque el documento de descubrimiento desde esta URL en lugar de derivarlo de `issuer`, para IdP detrás de un proxy que reescribe el host del emisor. La ruta debe contener `/.well-known/`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `form_action_origins`           | No        | Orígenes adicionales para la directiva `Content-Security-Policy: form-action` de la página `/device`. La puerta de enlace ya permite `'self'` y el origen `authorization_endpoint` descubierto, pero Chrome aplica `form-action` contra toda la cadena de redirección. Si su IdP redirige a través de un segundo host, como Azure AD federado a ADFS, Okta de concentrador y radio, o un interceptor SSO corporativo, enumere cada origen por el que la solicitud de autorización puede redirigir.                                                                                                                                                                                                                                                                          |
| `ca_cert_pem`                   | No        | Certificado CA PEM que reemplaza el almacén de confianza del sistema solo para solicitudes de IdP. Úselo para Keycloak o Dex detrás de PKI corporativa.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

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

| Campo             | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | Sí        | URL `postgres://` o `postgresql://`. Requerido: el encuentro de concesión de dispositivo, donde la devolución del navegador escribe y el CLI de sondeo lee, necesita estado entre réplicas. La puerta de enlace ejecuta sus propias migraciones de esquema al arrancar, por lo que el rol necesita `CREATE TABLE` en el esquema de destino. Si su política de seguridad prohíbe DDL desde el rol de aplicación, ejecute las migraciones con un rol de administrador, inicialmente y nuevamente cada vez que una nueva versión envíe migraciones, y otorgue al rol de aplicación `SELECT, INSERT, UPDATE, DELETE` en las tablas de la puerta de enlace. Consulte [Actualizaciones](/docs/es/claude-apps-gateway-deploy#upgrades) y [Postgres](/docs/es/claude-apps-gateway-deploy#postgres). |
| `username`        | No        | Anula el usuario en `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `password`        | No        | Credencial de base de datos. Establézcalo aquí en lugar de en `postgres_url` para que la credencial se mantenga fuera de la URL. Acepta cualquier carácter y tiene prioridad sobre las credenciales de URL.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_connections` | No        | Tamaño del grupo de conexiones de Postgres por réplica. Predeterminado `5`, que es conservador y amigable con bases de datos compartidas. Con [límites de gasto](#admin) habilitados, la ruta activa realiza algunas operaciones por solicitud de inferencia, así que aumente para una base de datos dedicada bajo carga, y mantenga réplicas × esto por debajo de `max_connections` de la base de datos.                                                                                                                                                                                                                                                                                                                                                                         |

Para desarrollo local, apunte `postgres_url` a un contenedor Postgres desechable, por ejemplo `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` es una lista ordenada. La puerta de enlace reenvía la inferencia al primer upstream que resuelve el modelo solicitado. En `5xx`, `429`, `401`, `403`, `404`, o tiempo de espera, falla al siguiente; otros `4xx` no, porque esos errores son atribuibles a la solicitud en lugar del upstream. Un `401` o `403` significa que la credencial propia de la puerta de enlace falló contra ese upstream, y un `404` significa que ese upstream no sirve el modelo solicitado, por lo que un upstream posterior en la lista aún puede.

La conmutación por error en `404` requiere gateway v2.1.198 o posterior. Las versiones anteriores devolvieron el primer `404` al cliente incluso cuando un upstream posterior en la lista sirvió el modelo.

Múltiples upstreams del mismo proveedor deben establecer un `name:` distinto.

Los clientes de Bedrock, Claude Platform en AWS, Agent Platform y Foundry se construyen una vez al iniciar, y sus SDK actualizan credenciales internamente, por lo que rotar credenciales en la nube no requiere un reinicio. Las claves API estáticas de Anthropic y los portadores se leen al iniciar; consulte [API de Anthropic](#anthropic-api).

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

| Configuración           | Cómo                                                                                                                                                                                                                                                                                                                                                                                  |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Permisos de IAM         | Otorgue al principal de la puerta de enlace `bedrock:InvokeModel` y `bedrock:InvokeModelWithResponseStream` tanto en los ARN de perfil de inferencia como en los ARN de modelo de fundación subyacentes. Para el catálogo integrado en regiones de EE.UU.: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` y `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| Acceso a modelos        | En la consola de Bedrock, por región, solicite y habilite el acceso a modelos para los modelos Claude que desee. Los perfiles de inferencia entre regiones (`us.anthropic.*`) requieren acceso a modelos en cada región que abarca el perfil.                                                                                                                                         |
| EKS (IRSA)              | Cree un rol de IAM con la política anterior y una política de confianza para el proveedor OIDC de su clúster limitado a la cuenta de servicio de la puerta de enlace. Anote la cuenta de servicio con `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` la recoge.                                                                                    |
| ECS / EC2               | Adjunte el rol de IAM a la definición de tarea o perfil de instancia. `auth: {}` la recoge.                                                                                                                                                                                                                                                                                           |
| En cualquier otro lugar | Pase credenciales a través de las variables env `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` y `AWS_SESSION_TOKEN`, o establézcalas explícitamente en `auth:` con expansión `${VAR}`                                                                                                                                                                                                  |
| Región                  | `region:` es la región del punto final de API. Los perfiles de inferencia entre regiones enrutan a través de la geografía (EE.UU., UE, APAC) independientemente de cuál elija. Para regiones no estadounidenses o ARN de rendimiento aprovisionado, agregue un bloque [`models:`](#models) con los IDs correctos por upstream.                                                        |

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

<h4 id="multiple-upstreams">
  Múltiples upstreams
</h4>

El mismo proveedor puede aparecer más de una vez con un `name:` distinto. Esto cubre diferentes regiones, diferentes cuentas a través de diferentes cadenas de credenciales, rendimiento aprovisionado versus bajo demanda, y conmutación por error entre proveedores.

La puerta de enlace intenta upstreams en orden. `5xx`, `429`, `401`, `403`, `404`, tiempos de espera y punto final faltante (`501`) conmutan por error; otros `4xx` no.

`429` es capacidad por upstream, por lo que el agotamiento de rendimiento aprovisionado (PT) conmuta por error a bajo demanda. `404` es disponibilidad de modelo por upstream, por lo que un upstream que no ha habilitado un modelo no bloquea un upstream posterior que lo sirve. Un upstream que no puede resolver el modelo solicitado se omite sin un viaje de red.

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

# Los IDs de modelo por upstream se clave en el `name:` del upstream; un upstream
# sin un `name:` predeterminado a su cadena de proveedor (p. ej., `bedrock`). Cualquier
# upstream no listado para un modelo se omite, que es cómo enruta un modelo
# a rendimiento aprovisionado mientras todo lo demás permanece bajo demanda.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Palanca                        | Cómo                                                                                                                                                                                                                                                      |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Diferentes regiones            | Un upstream de Bedrock por región, cada uno con su propio `region:`. Con [`auto_include_builtin_models: true`](#models) los perfiles de inferencia entre regiones enrutan automáticamente; para implementaciones fijas de región use un bloque `models:`. |
| Diferentes cuentas             | Un upstream de Bedrock por cuenta, cada uno con sus propias credenciales en `auth:`. La cadena predeterminada (`auth: {}`) utiliza la identidad de la vaina; para una segunda cuenta, establezca credenciales explícitas o un token portador.             |
| Rendimiento aprovisionado      | Asigne el modelo al ARN de rendimiento aprovisionado en `models:` para el nombre de ese upstream. Otros upstreams mantienen el ID bajo demanda, por lo que la capacidad de PT se agota antes de conmutar por error.                                       |
| Puntos finales de VPC / FIPS   | Establezca `base_url:` en el upstream a su URL de punto final de VPC o FIPS                                                                                                                                                                               |
| Enrutamiento limitado a modelo | Omita un upstream del mapa `upstream_model:` de un modelo y ese upstream se omite para ese modelo. Por ejemplo, enrute Opus a rendimiento aprovisionado y Sonnet y Haiku a bajo demanda.                                                                  |

La conmutación por error entre proveedores en la nube, o a la API de Anthropic directo, cambia qué acuerdo, geografía y otros términos rigen la solicitud.

El CLI aplica el mismo control de características a puertas de enlace independientemente de cuál upstream sirva una solicitud dada, por lo que la conmutación por error no envía un campo de cuerpo que un upstream rechazaría.

<h2 id="optional-sections">
  Secciones opcionales
</h2>

<h3 id="admin">
  `admin`
</h3>

Opcional. Habilita `/v1/organizations/spend_limits`, que refleja la API de administración pública de Anthropic, y cumplimiento de gasto por desarrollador en `/v1/messages`. Consulte [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits) para saber cómo se establecen y aplican los límites; esta sección cubre las claves `gateway.yaml` que activan la función y la ajustan.

```yaml theme={null}
admin:
  # Claves API estáticas nombradas para los puntos finales de administración, enviadas como x-api-key.
  # El id aparece en el registro de auditoría como admin-key:<id> para que cada clave sea
  # atribuible. Matriz para rotación: agregue la nueva clave, despliegue clientes,
  # elimine la antigua.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # Grupos de IdP otorgados acceso de administrador completo a través del JWT de puerta de enlace normal (sin clave API).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Campo                     | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                |
| ------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `write_keys`              | No        | Matriz de `{id, key}`. Un `x-api-key` que coincida con uno de estos puede listar, establecer y eliminar límites de gasto. Los valores de clave deben tener al menos 32 caracteres; los `id`s deben ser únicos en `read_keys` y `write_keys`.                                                                                               |
| `read_keys`               | No        | Matriz de `{id, key}`. Solo lectura: cada punto final `GET`, incluida la enumeración de límites, la obtención de uno por ID y la lectura de [`/effective`](/docs/es/claude-apps-gateway-spend-limits#%2Feffective) y [`/audit`](/docs/es/claude-apps-gateway-spend-limits#%2Faudit).                                                                 |
| `admin_groups`            | No        | Nombres de grupos de IdP. Un JWT de puerta de enlace cuya reclamación `groups` incluye uno de estos tiene acceso de administrador completo, lectura y escritura, y audita como `oidc:<sub>`. Úselo para administradores humanos; use claves API para máquinas.                                                                             |
| `blocked_message`         | No        | Añadido textualmente al `429 billing_error` que ve un desarrollador bloqueado. Escriba la instrucción completa, como una URL o un canal de Slack. Sin establecer, el error es `spend limit reached`.                                                                                                                                       |
| `audit_retention_days`    | No        | Predeterminado `365`. Las filas `admin_audit` más antiguas se barren.                                                                                                                                                                                                                                                                      |
| `spend_retention_months`  | No        | Predeterminado `13`. Las filas del contador `spend` más antiguas que esto se barren. El predeterminado mantiene un año completo más el mes parcial actual para informes año a año.                                                                                                                                                         |
| `identity_retention_days` | No        | Predeterminado `90`. TTL de última visualización para filas `principal_emails`, que contienen el correo electrónico, nombre para mostrar y grupos de cada desarrollador (PII). Deliberadamente más corto que la retención de gasto para que una identidad desaprovisionada envejezca mientras sus contadores de gasto anónimos permanecen. |
| `group_limit_mode`        | No        | `min` (predeterminado) o `max`. Cuando un desarrollador está en varios grupos con límites, `min` aplica el más restrictivo y `max` el menos. Utilizado tanto por cumplimiento como por `/effective`.                                                                                                                                       |

<h3 id="enforcement">
  `enforcement`
</h3>

El bloque `enforcement` controla cómo se comportan las verificaciones de límite de gasto cuando el almacén no está disponible.

| Campo                  | Requerido | Descripción                                                                                                                                                                                                                                                                                                                           |
| ---------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | No        | Predeterminado `false`. El cumplimiento de gasto falla abierto en una interrupción de Postgres, por lo que la inferencia permanece activa. Establezca `true` para fallar cerrado: los desarrolladores sobre el límite se bloquean, pero también todos si el almacén es inaccesible. No tiene efecto sin un bloque [`admin:`](#admin). |

<h3 id="models">
  `models`
</h3>

El bloque `models` es una lista de modelos curada por administrador opcional, servida en `/v1/models` y utilizada para traducir IDs de modelo por upstream. Es necesario para regiones de Bedrock no estadounidenses, ARN de rendimiento aprovisionado de Bedrock y nombres de implementación de Foundry.

```yaml theme={null}
auto_include_builtin_models: true   # false: exponga solo la lista a continuación
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: texto opcional mostrado en clientes que lo muestren
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # o un ARN de perfil de inferencia
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

El bloque `managed` define políticas de acceso basadas en roles clave en grupos de IdP o dominio de correo electrónico. Las políticas se evalúan en orden; se selecciona la primera coincidencia, luego se fusiona en la base de captura `match: {}` descrita a continuación. Se sirven por usuario en `GET /managed/settings` con almacenamiento en caché de ETag/304.

```yaml theme={null}
managed:
  policies:
    # Grupos específicos primero.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Captura predeterminada al final: coincide con todos los que se autenticaron.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Una captura `match: {}`, convencionalmente listada al final, se trata como una capa base. Cada otra política hereda cualquier clave que no establezca de la captura, por lo que las entradas por rol solo necesitan listar lo que difiere del predeterminado de la organización. Las reglas de fusión dependen del tipo de clave:

* **Listas de permitidos**: `availableModels` y `permissions.allow`. La lista de una política específica reemplaza completamente la de la base.
* **Listas de denegación y matrices de hooks**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` y cada matriz de tipo de evento `hooks`. Estos toman la unión de base y política, por lo que un gancho de denegación o auditoría en toda la organización no puede ser accidentalmente eliminado por una anulación por rol.
* **Claves de tipo registro**: `env`, `modelOverrides` y `skillOverrides`. Estos se fusionan superficialmente, por lo que un bloque `env` por rol anula las claves que establece y hereda el resto de la base.

`availableModels` también se aplica del lado del servidor en `/v1/messages`, por lo que un modelo denegado devuelve `400` independientemente de lo que envíe el cliente.

| Coincidencia                                        | Comportamiento                                                                                                                                                                                              |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Coincide con cada usuario autenticado. Comience con uno de estos y agregue políticas limitadas a grupo más tarde.                                                                                           |
| `match: { groups: [a, b] }`                         | Coincide si la reclamación `groups` del JWT contiene cualquiera de los grupos listados. Sensible a mayúsculas y minúsculas: los grupos deben coincidir con el uso exacto de mayúsculas y minúsculas de IdP. |
| `match: { email_domain: example.com }`              | Coincide con la parte después de la última `@` en la reclamación `email` del JWT, sin distinción de mayúsculas y minúsculas. Acepta un dominio por política.                                                |
| `match: { groups: [a], email_domain: example.com }` | Ambas condiciones deben coincidir                                                                                                                                                                           |

Un usuario autenticado que no coincida con ninguna política obtiene los valores predeterminados de la puerta de enlace, lo que significa cada modelo en el catálogo y sin configuración administrada. Agregue una captura `match: {}` al final si desea una política predeterminada garantizada.

<Note>
  La puerta de enlace no mantiene su propio directorio de usuarios. Autoriza cada solicitud desde el token de IdP del usuario, leyendo la membresía del grupo de la reclamación `groups` del token y evaluando políticas contra ella. No hay un registro para enumerar y no hay cuentas para crear previamente, y por lo tanto no hay punto final SCIM, porque no hay nada para que SCIM sincronice.

  Ejecute la gestión del ciclo de vida del usuario y grupo en la fuente de verdad, que es el aprovisionamiento SCIM nativo de su IdP o una plataforma de gobernanza de identidad dedicada. La membresía y desaprovisionamiento gobernados allí fluyen a la puerta de enlace automáticamente a través del token. Si desea el aprovisionamiento SCIM de las propias cuentas de Claude, esa es una capacidad de [Claude para empresas](/docs/es/admin-setup).

  Se aplican dos relojes de propagación:

  * **Contenido de política**: editar una política e implementar nuevamente llega a clientes conectados en su próxima encuesta de configuración administrada, dentro de una hora
  * **Membresía de grupo**: cambiar la membresía de grupo de un usuario cambia qué política los coincide. Esto entra en vigor en la siguiente acuñación de sesión, lo que significa la siguiente actualización silenciosa, limitada por `session.ttl_hours`.
</Note>

<h4 id="what-goes-in-cli">
  Qué va en `cli`
</h4>

Cada valor `cli` es un documento completo de `managed-settings.json` de Claude Code, el mismo esquema que implementaría a través de MDM o `/etc/claude-code/managed-settings.json`, expresado aquí como YAML. El CLI aplica el documento entregado en el nivel administrado, por encima de la configuración de usuario y proyecto.

La puerta de enlace valida cada documento contra el esquema de configuración del CLI al arrancar, por lo que una clave de nivel superior no reconocida o una clave reconocida con un valor mal formado falla al arrancar con un error que nombra cada clave ofensiva. Las partes deliberadamente abiertas del esquema aún aceptan valores arbitrarios, porque clientes más nuevos pueden reconocer entradas que el esquema de la puerta de enlace no. Estas claves abiertas son `env`, `pluginConfigs` y claves anidadas bajo `permissions`.

Debido a que la validación utiliza el esquema incluido con la versión instalada de la puerta de enlace, poner una clave de configuración de nivel superior introducida por una versión más nueva de Claude Code en la configuración administrada requiere actualizar primero la puerta de enlace. Pruebe una nueva política en un cliente antes de implementarla ampliamente.

La referencia de clave completa está en [Configuración de Claude Code](/docs/es/settings#available-settings). Las claves que los operadores alcanzan primero:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Acceso a modelos (también aplicado del lado del servidor en /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Política de permisos
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # bloquea --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignora reglas de permisos de usuario/proyecto

        # Entorno empujado al proceso CLI. DISABLE_UPDATES bloquea
        # actualizaciones de fondo y manuales; DISABLE_AUTOUPDATER detiene solo
        # actualizaciones de fondo.
        env:
          DISABLE_UPDATES: "1"                    # versiones de pin a través de su propia distribución

        # Hooks en toda la organización. Los comandos de gancho se ejecutan en máquinas de desarrollador, no en la
        # puerta de enlace, por lo que la ruta debe existir en cada SO de cliente en la política.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Clave                                      | Aplicada por           | Efecto                                                                                                                                                                                                                                        |
| ------------------------------------------ | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Puerta de enlace + CLI | Lista de permitidos de modelos. También se verifica en `/v1/messages`, por lo que un cliente parcheado no puede omitirlo.                                                                                                                     |
| `permissions.allow` / `.deny`              | CLI                    | Reglas de herramientas y comandos. Consulte [Permisos](/docs/es/permissions).                                                                                                                                                                      |
| `permissions.disableBypassPermissionsMode` | CLI                    | Establezca en `disable` para bloquear [`bypassPermissions`](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode), el modo que aprueba automáticamente cada llamada de herramienta, y la bandera `--dangerously-skip-permissions` |
| `allowManagedPermissionRulesOnly`          | CLI                    | Cuando es `true`, se ignoran las reglas de permisos de usuario y proyecto; solo se aplican las reglas de este documento                                                                                                                       |
| `env`                                      | CLI                    | Variables de entorno fusionadas en el proceso CLI. Úselo para telemetría, actualización automática y anulaciones de nombres de modelos.                                                                                                       |
| `hooks`                                    | CLI                    | [Hooks](/docs/es/hooks) en toda la organización                                                                                                                                                                                                    |

Debido a que estas configuraciones llegan a través de la red, el CLI muestra a cada desarrollador un diálogo de aprobación de seguridad de una sola vez antes de aplicar cualquier cosa que pueda ejecutar un comando de shell o alterar dónde va el tráfico. El diálogo cubre:

* `hooks`
* Variables `env` que no están en la lista segura integrada del CLI
* Configuración de ejecución de shell como `apiKeyHelper` y `statusLine`
* Contenido de CLAUDE.md administrado

La lista segura determina qué variables `env` se aplican sin aprobación:

* **En la lista segura**: variables de actualización automática y nombre de modelo
* **No en la lista segura**: variables de proxy, variables de URL base y `OTEL_EXPORTER_OTLP_ENDPOINT`

La configuración de [telemetría](#telemetry) de la puerta de enlace empuja `OTEL_EXPORTER_OTLP_ENDPOINT`, por lo que establecer `telemetry.forward_to` desencadena el diálogo en cada cliente interactivo. Las ejecuciones no interactivas con la bandera `-p` omiten el diálogo y aplican configuración sin aprobación. El diálogo protege la máquina del desarrollador de una puerta de enlace comprometida u hostil, no la organización del desarrollador, por lo que el salto `-p` es intencional en lugar de una brecha.

Si un desarrollador rechaza, Claude Code sale en lugar de aplicar la política. Empujar un nuevo gancho o variable env no segura a una política amplia significa un mensaje de aprobación en el próximo inicio de cada desarrollador coincidente.

La clave `cli` se nombró `settings` en versiones anteriores. Ese deletreo aún se acepta como alias, pero las nuevas implementaciones deben usar `cli`.

<h4 id="precedence-with-other-managed-sources">
  Precedencia con otras fuentes administradas
</h4>

Si un dispositivo también tiene un `managed-settings.json` local o una política entregada por MDM, las fuentes administradas no se fusionan. La fuente de mayor prioridad proporciona toda la configuración de política, clasificada en este orden con mayor prioridad primero:

1. El [asistente de política](/docs/es/settings#compute-managed-settings-with-a-policy-helper)
2. Configuración entregada por puerta de enlace
3. MDM, a través del registro HKLM en Windows o un plist en macOS
4. El archivo `managed-settings.json`
5. El registro HKCU, solo en Windows

Los hosts de incrustación pueden suministrar política a través de la opción `managedSettings` del SDK. Se ignora de forma predeterminada y se aplica solo cuando una fuente administrada se adhiere con [`parentSettingsBehavior: "merge"`](/docs/es/settings#available-settings), filtrada para que pueda endurecer la política pero no aflojarla.

La excepción es un pequeño conjunto de claves entre fuentes, honradas cuando cualquier fuente de administrador las establece; el nivel HKCU escribible por el usuario se excluye:

* `sandbox.network.allowManagedDomainsOnly` y `sandbox.filesystem.allowManagedReadPathsOnly`: cuando se bloquean, las listas de permitidos correspondientes se unen en todas las fuentes
* [`allowAllClaudeAiMcps`](/docs/es/settings#available-settings): anulación de solo permitir para la lista de permitidos del servidor MCP de claude.ai
* `sandbox.bwrapPath` y `sandbox.socatPath`: rutas del sistema de archivos a los binarios auxiliares de [sandbox](/docs/es/sandboxing)

`allowManagedPermissionRulesOnly` y `disableBypassPermissionsMode` no son entre fuentes, por lo que solo se aplica el valor de la fuente ganadora.

Las políticas de puerta de enlace se aplican a cada invocación de Claude Code en la máquina, incluidas ejecuciones no interactivas `claude -p` y sesiones generadas por el SDK del agente. Si la puerta de enlace es inaccesible al iniciar, las sesiones firmadas salen con un error en lugar de ejecutarse sin su política.

<Warning>
  `mcpServers` dentro de un bloque `cli` de política se rechaza al arrancar la puerta de enlace. La distribución de MCP por grupo no está disponible; implemente servidores MCP a través del `managed-mcp.json` basado en archivos en cada dispositivo o permita que los desarrolladores los agreguen localmente.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

El CLI envía métricas, registros y, cuando está habilitado, trazas del Protocolo OpenTelemetry (OTLP) sobre HTTP a la puerta de enlace, que las retransmite textualmente a cada destino configurado. Consulte [Monitoreo de uso](/docs/es/monitoring-usage) para las métricas y eventos que emite el CLI.

El CLI marca cada exportación con la identidad del usuario autenticado, leída del JWT emitido por la puerta de enlace: los atributos `user.id`, `user.email` y `user.groups`. La atribución de costo y uso por desarrollador funciona sin configuración del lado del desarrollador.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Opt-in por señal. Predeterminado: solo métricas.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Cada destino se adhiere a `metrics`, `logs` y `traces` independientemente, y el predeterminado es solo métricas. Las señales difieren en sensibilidad:

  * **Métricas**: contadores agregados como conteos de tokens, conteos de solicitudes y latencia
  * **Registros y trazas**: pueden llevar comandos bash completos, entradas de herramientas y rutas de archivos, cubriendo cualquier cosa que Claude Code haga en la máquina de un desarrollador

  Habilite registros y trazas solo en destinos con los controles de acceso y la política de retención que los datos justifiquen.
</Warning>

La telemetría está desactivada en el CLI de forma predeterminada. Configurar `telemetry.forward_to` junto con `listen.public_url` la activa. La puerta de enlace empuja cinco variables env a cada cliente conectado a través de `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

El punto final empujado se construye a partir de la URL pública, por lo que las métricas y registros no necesitan configuración OTEL de desarrolladores o políticas. La configuración empujada se aplica en el nivel administrado, anulando variables `OTEL_*` que un desarrollador establece localmente.

[Las trazas](/docs/es/monitoring-usage#traces-beta) además requieren `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` en cada cliente. La puerta de enlace no empuja esa variable, así que establézcala a través del bloque `env` de una política administrada. No está en la lista segura del CLI, por lo que entregarla a través de una política está cubierta por el mismo [diálogo de aprobación de seguridad](#managed) que el punto final OTLP empujado ya desencadena.

Tanto las codificaciones OTLP de protobuf como JSON se retransmiten, y cualquier backend compatible con OpenTelemetry funciona como destino.

<h3 id="http-tuning">
  Ajuste de HTTP
</h3>

Cuatro bloques opcionales de nivel superior, `access_control`, `limits`, `timeouts` y `rate_limits`, ajustan la superficie HTTP. Los valores predeterminados se adaptan a la mayoría de implementaciones.

| Bloque           | Clave                                          | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ---------------- | ---------------------------------------------- | -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | vacío          | Permitir/denegar IP de entrada por dirección de cliente, después de la resolución de `trusted_proxies`. `deny_cidrs` se verifica primero; un cliente que coincida se rechaza incluso si `allow_cidrs` también coincide. Si `allow_cidrs` no está vacío, la puerta de enlace es predeterminada denegada. `/healthz` y `/readyz` están exentos de `allow_cidrs`.                                                                         |
| `limits`         | `max_request_bytes`                            | 32 MiB         | Cuerpo de solicitud de entrada máximo; las solicitudes de tamaño excesivo obtienen `413` antes de que el cuerpo se almacene en búfer. Aumente para solicitudes de archivo o imagen grandes.                                                                                                                                                                                                                                            |
| `limits`         | `max_request_header_bytes`                     | sin establecer | Cuando se establece, los encabezados de tamaño excesivo devuelven `431`                                                                                                                                                                                                                                                                                                                                                                |
| `limits`         | `max_url_length`                               | sin establecer | Cuando se establece, una URL demasiado larga devuelve `414`                                                                                                                                                                                                                                                                                                                                                                            |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000         | Espera máxima para los encabezados de respuesta del upstream (tiempo hasta el primer byte). El cuerpo de respuesta luego se transmite sin límite de reloj de pared. Se aplica a la ruta de upstream de Anthropic directo; cada otro proveedor está limitado por el tiempo de espera propio del SDK del proveedor.                                                                                                                      |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600       | Límite de velocidad por IP en el punto final de autorización de dispositivo no autenticado. Aumente para una organización grande detrás de una IP de salida compartida o NAT. Estos límites se aplican solo al flujo de inicio de sesión de concesión de dispositivo, no a la inferencia `/v1/messages`. Consulte [Resistencia de fuerza bruta de código de usuario](/docs/es/claude-apps-gateway-deploy#user-code-brute-force-resistance). |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600       | Límite de velocidad por IP en envíos de `user_code` en `/device`                                                                                                                                                                                                                                                                                                                                                                       |

<h2 id="complete-example">
  Ejemplo completo
</h2>

Esta configuración de referencia completa ejercita cada sección principal; los bloques [ajuste de HTTP](#http-tuning) mantienen sus valores predeterminados. Cópiela, elimine lo que no necesite y complete sus valores. La configuración en el [Inicio rápido](/docs/es/claude-apps-gateway#quickstart) es una versión mínima de esta.

```yaml gateway.yaml theme={null}
# Ejecutar con:
#   claude gateway --config gateway.yaml
#
# La verbosidad del registro operativo se controla mediante la variable de entorno
# CLAUDE_GATEWAY_LOG_LEVEL (info | warn | error; predeterminado info). No
# afecta eventos de auditoría, que siempre se emiten.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omita el bloque tls cuando se ejecute detrás de una entrada que termine TLS.
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
  # Requerido cuando el emisor es el servidor org de Okta, cuyos id_tokens
  # pueden omitir correo electrónico y grupos; la puerta de enlace los completa desde /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emite grupos solo cuando se solicita el alcance `groups` y el
  # filtro de reclamación de grupos de la aplicación los permite. La política de contratistas a continuación
  # coincide en grupos, por lo que el alcance se solicita aquí.
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

# Habilita /v1/organizations/spend_limits (refleja la API de administración de Anthropic)
# y cumplimiento de gasto por desarrollador en /v1/messages. Omita para desactivar.
# Los límites en sí se establecen a través de la API de administración, no aquí.
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
        # Agregue reglas de denegación para restringir herramientas.
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

Todo lo anterior configura el servidor de puerta de enlace. Apuntar máquinas de desarrollador a él se configura por separado, en cada dispositivo, a través de la [configuración administrada](/docs/es/settings#settings-files) de Claude Code. La puerta de enlace no puede empujar estas claves por sí misma, porque son lo que le dice al cliente dónde está la puerta de enlace.

Para el CLI, establezca ambas claves en el `managed-settings.json` por SO:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Implemente ese archivo en cada dispositivo, típicamente a través de su plataforma MDM. La ruta del archivo difiere por plataforma:

| Plataforma  | Ruta                                                                                                                                   |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-settings.json`, o el dominio de preferencias administradas `com.anthropic.claudecode` |
| Linux y WSL | `/etc/claude-code/managed-settings.json`                                                                                               |
| Windows     | `C:\Program Files\ClaudeCode\managed-settings.json`, o Política de grupo a través del registro HKLM                                    |

`forceLoginGatewayUrl` y el valor `"gateway"` de `forceLoginMethod` se honran solo desde el nivel administrado controlado por administrador. Un desarrollador que los establezca en su propio `~/.claude/settings.json` no tiene efecto.

<h2 id="related">
  Relacionado
</h2>

* [Descripción general de la puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway): inicio rápido y conexión de desarrollador
* [Guía de implementación](/docs/es/claude-apps-gateway-deploy): configuración de IdP, imagen de contenedor, Kubernetes y Cloud Run, y operaciones
* [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits): límites por desarrollador y la API de administración
