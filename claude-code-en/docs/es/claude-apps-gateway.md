> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Puerta de enlace de aplicaciones Claude para Amazon Bedrock, Claude Platform en AWS, Google Cloud y Microsoft Foundry

> Ejecute Claude Code a través de Amazon Bedrock, Claude Platform en AWS, Google Cloud o Microsoft Foundry detrás de una puerta de enlace autohospedada con inicio de sesión SSO, acceso a modelos por grupo y telemetría OTLP.

<Note>
  La puerta de enlace de aplicaciones Claude está diseñada para organizaciones que deben —o prefieren— enrutar la inferencia a través de su propio proveedor de nube, por ejemplo para cumplir con los requisitos de [residencia de datos](/docs/es/claude-apps-gateway-deploy#compliance-posture). Si no tiene este requisito y desea acceso a otras características como aprovisionamiento SCIM o Claude Code en web y dispositivos móviles, Claude Enterprise puede ser una mejor opción. Consulte la página de [disponibilidad de características](/docs/es/feature-availability) para una comparación completa de todos los métodos de implementación.
</Note>

Claude apps gateway es un servicio autohospedado que se sitúa entre los clientes de Claude Code de sus desarrolladores y su proveedor de modelos. Los desarrolladores inician sesión con su proveedor de identidad corporativo (IdP) en lugar de mantener claves API o credenciales de nube. La puerta de enlace mantiene la credencial ascendente, aplica el acceso a modelos y [configuraciones administradas](/docs/es/managed-settings) por grupo de IdP, y retransmite la telemetría de uso a su propia pila de observabilidad.

Se incluye en el binario `claude`, por lo que el mismo ejecutable que ejecuta Claude Code en una computadora portátil ejecuta el servidor de puerta de enlace con `claude gateway --config gateway.yaml`.

Esta página cubre:

* [Por qué Claude apps gateway](#why-claude-apps-gateway), qué agrega sobre ejecutar el suyo propio, y cuándo algo más se ajusta mejor
* Un [inicio rápido](#quickstart) con [requisitos previos](#prerequisites) que lleva una puerta de enlace de cero a un desarrollador que ha iniciado sesión
* [Conectar desarrolladores](#connect-developers), incluida la configuración de la URL de la puerta de enlace a través de configuraciones administradas
* [Disponibilidad y limitaciones](#availability-and-limitations) que cubre qué características de Claude Code funcionan a través de la puerta de enlace y qué soporta el servidor

Las páginas complementarias profundizan más. La [referencia de configuración](/docs/es/claude-apps-gateway-config) cubre todas las opciones en el archivo YAML que escribe el inicio rápido, y la [guía de implementación](/docs/es/claude-apps-gateway-deploy) cubre la configuración por IdP, la implementación en Kubernetes y Cloud Run, y las operaciones.

<h2 id="why-claude-apps-gateway">
  Por qué puerta de enlace de aplicaciones Claude
</h2>

La [descripción general de la puerta de enlace](/docs/es/gateways) cubre qué hace una puerta de enlace y por qué ejecutaría una. La puerta de enlace de aplicaciones Claude es la propia puerta de enlace de Anthropic, integrada en el binario `claude` y probada junto con cada lanzamiento de Claude Code, por lo que reenvía los encabezados y campos de solicitud que Claude Code envía sin que los operadores mantengan una lista de permitidos separada. Una vez implementada, le proporciona:

* **Credenciales**: la clave API ascendente o la credencial de nube vive solo en su infraestructura. Los desarrolladores se autentican con SSO corporativo y reciben tokens portadores de corta duración, por lo que la desvinculación ocurre en su IdP. Desaprovisione un usuario y su acceso a la puerta de enlace expira dentro de la duración de la sesión, una hora por defecto.
* **Control de acceso**: sus grupos de IdP se asignan a listas de permitidos de modelos y políticas de [configuración administrada](/docs/es/managed-settings). La puerta de enlace aplica el acceso a modelos del lado del servidor, rechazando solicitudes de modelos no otorgados, y selecciona la política de configuración administrada de cada grupo, que la CLI aplica en el [nivel de configuración administrada](/docs/es/settings#settings-precedence). Diferentes equipos obtienen diferentes modelos, herramientas y permisos, y un desarrollador no puede anular lo que su política bloquea.
* **Entrega de configuración**: la puerta de enlace entrega la configuración administrada a los clientes conectados por sí misma, reemplazando la [configuración administrada por servidor](/docs/es/server-managed-settings) de la consola de administrador de claude.ai.
* **Telemetría**: cada destino configurado recibe [métricas del Protocolo OpenTelemetry (OTLP)](/docs/es/monitoring-usage) con recuentos de tokens, modelo, identidad del usuario y latencia por defecto, con registros y trazas como activaciones opcionales por destino.
* **Enrutamiento ascendente**: los clientes hablan la API de Mensajes de Anthropic a la puerta de enlace, y la puerta de enlace traduce para cada ascendente, ya sea Amazon Bedrock, [Plataforma Claude en AWS](/docs/es/claude-platform-on-aws), Plataforma de Agentes de Google Cloud, Microsoft Foundry, o la API de Anthropic, con conmutación por error entre ellos. Puede cambiar regiones, proveedores u orden de conmutación por error sin que los desarrolladores lo noten o reconfiguren.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagrama que muestra clientes de Claude Code y las pestañas Chat, Cowork y Code de Claude Desktop conectándose sobre HTTPS con tokens portadores a una puerta de enlace de aplicaciones Claude autohospedada dentro de su infraestructura, que inicia sesión de usuarios contra su IdP, almacena estado de autenticación en PostgreSQL, retransmite telemetría a su recopilador OTLP, y reenvía inferencia a Amazon Bedrock, Plataforma Claude en AWS, Google Cloud, Microsoft Foundry o la API de Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  El plano de datos de la puerta de enlace no envía nada a la infraestructura de Anthropic a menos que la API de Anthropic sea un ascendente configurado. Usted controla dónde van la telemetría, los registros de auditoría, la configuración administrada y la identidad de IdP de sus desarrolladores, y la puerta de enlace no envía ninguno de ellos a Anthropic. Para el tráfico restante que el proceso de CLI puede enviar y cómo cerrarlo, consulte [Postura de cumplimiento](/docs/es/claude-apps-gateway-deploy#compliance-posture).
</Note>

Para ver qué características de Claude Code funcionan a través de la puerta de enlace y qué soporta el servidor en sí, consulte [Disponibilidad y limitaciones](#availability-and-limitations) a continuación. Para decisiones como costo, derivación, ejecutar múltiples puertas de enlace y plataformas sin servidor, consulte la [guía de implementación](/docs/es/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Otras implementaciones de puerta de enlace
</h3>

Si ya ejecuta una puerta de enlace LLM o puerta de enlace API que cumple con sus necesidades, continúe usándola; [Otras puertas de enlace LLM](/docs/es/llm-gateway) cubre la configuración de Claude Code contra ella.

La [guía de compatibilidad de protocolo de puerta de enlace](/docs/es/llm-gateway-protocol) documenta qué espera Claude Code de cualquier puerta de enlace: los puntos finales que llama, los encabezados y campos de cuerpo a reenviar, y qué deja de funcionar cuando se eliminan. Una puerta de enlace de aplicaciones Claude en ejecución también sirve su propia referencia de protocolo en `GET /protocol`, que describe los puntos finales que expone a clientes de Claude Code: inicio de sesión SSO, inferencia, entrega de configuración administrada, descubrimiento de modelos y telemetría. Obténgalo con `curl https://claude-gateway.internal.example.com/protocol` desde cualquier puerta de enlace implementada, como la que produce el [inicio rápido](#quickstart) a continuación.

Los cambios importantes en el protocolo se anuncian con anticipación, pero no se garantiza compatibilidad hacia atrás indefinida.

<h2 id="quickstart">
  Inicio rápido
</h2>

Este inicio rápido recorre la ruta mínima: registre un cliente OAuth en su IdP, escriba un `gateway.yaml`, ejecute la puerta de enlace junto con Postgres con Docker Compose, y verifique el inicio de sesión de extremo a extremo. Utiliza un ascendente de Amazon Bedrock; la Plataforma de Claude en AWS, la Plataforma de Agentes de Google Cloud, Microsoft Foundry, y la API de Anthropic son igualmente compatibles intercambiando el bloque `upstreams` como se muestra en la [referencia de configuración](/docs/es/claude-apps-gateway-config#upstreams). Al final tiene una puerta de enlace a la que un desarrollador puede `/login`.

<Note>
  **Implemente en su red privada.** Claude Code solo se conecta a una puerta de enlace cuya dirección es privada. Esta es una protección de seguridad, porque una puerta de enlace confiable puede insertar configuración que ejecute comandos en máquinas de desarrolladores. Coloque la puerta de enlace detrás de un equilibrador de carga interno o VPN y asígnele un nombre de host que se resuelva solo a direcciones IP privadas. Si su red interna está numerada desde espacio IPv4 público que su organización posee, consulte [Permitir una puerta de enlace en espacio de dirección público que usted posee](#allow-a-gateway-on-public-address-space-you-own).
</Note>

<h3 id="prerequisites">
  Requisitos previos
</h3>

Tenga estos en lugar antes de comenzar:

| Lo que necesita                              | Detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 o posterior             | El subcomando `claude gateway` y el flujo de inicio de sesión de la puerta de enlace se envían en v2.1.195. Las compilaciones públicas anteriores no las incluyen. Tanto la máquina que ejecuta el servidor de puerta de enlace como la máquina de cada desarrollador deben estar en v2.1.195 o posterior; ejecute `claude update` para obtener la última versión. La [Plataforma de Claude en AWS ascendente](/docs/es/claude-apps-gateway-config#claude-platform-on-aws) requiere Claude Code v2.1.198 o posterior en el servidor de puerta de enlace.                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Proveedor de identidad OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, o Dex, u otro IdP compatible con OIDC como PingFederate. La puerta de enlace ejecuta el descubrimiento OIDC estándar y el flujo de código de autorización contra ella. SAML y LDAP no son compatibles.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| PostgreSQL 14 o posterior                    | Respalda el flujo de inicio de sesión del dispositivo, donde la devolución de llamada del navegador escribe y la CLI de sondeo lee, más contadores de límite de velocidad. Cualquier Postgres administrado funciona, incluido el nivel más pequeño. Sin límites de gasto configurados, la puerta de enlace almacena algunos KB de estado de autenticación de corta duración; con [límites de gasto](/docs/es/claude-apps-gateway-spend-limits), también mantiene tablas de gasto, auditoría e identidad duraderas que deben respaldarse. TLS a través de `?sslmode=require` se recomienda.                                                                                                                                                                                                                                                                                                                                                                                               |
| Ascendente de modelo                         | Credenciales de Amazon Bedrock, credenciales de Plataforma de Claude en AWS, credenciales de Google Cloud, un recurso de Microsoft Foundry, o una clave API de Anthropic. Se admiten múltiples ascendentes con conmutación por error.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| HTTPS                                        | La puerta de enlace debe ser accesible sobre `https://` desde computadoras portátiles de desarrolladores y desde cualquier navegador utilizado para el inicio de sesión; la puerta de enlace sirve la página de verificación del dispositivo en el mismo oyente. Proporcione un certificado TLS a través de `listen.tls` o ejecute detrás de una entrada que termina TLS, y establezca `listen.public_url` en el origen externo en ambos casos. Un origen `http://` simple se acepta solo cuando el host de la puerta de enlace es loopback: `localhost`, `127.0.0.1`, o `::1`.                                                                                                                                                                                                                                                                                                                                                                                                     |
| Dirección de red privada                     | En `/login`, Claude Code requiere que el nombre de host o la dirección IP de la puerta de enlace se resuelvan solo a direcciones privadas: RFC 1918, link-local, CGNAT `100.64.0.0/10`, ULA IPv6 `fc00::/7`, o loopback. Para una puerta de enlace que usted aloja, cualquier dirección pública fuera de un bloque que usted declare se rechaza; consulte el [modelo de amenaza](/docs/es/claude-apps-gateway-deploy#threat-model-summary) en la guía de implementación. Si las máquinas de desarrolladores enrutan HTTPS a través de un proxy corporativo, el inicio de sesión también requiere que el host del proxy se resuelva a direcciones privadas; si no es así, agregue el host de la puerta de enlace a `NO_PROXY` para que la CLI se conecte directamente. Si su red interna está numerada desde espacio IPv4 público que su organización posee, [declare esos bloques](#allow-a-gateway-on-public-address-space-you-own) para que `/login` acepte una puerta de enlace allí. |
| Tiempo de ejecución de Linux                 | El servidor de puerta de enlace solo se ejecuta en el binario nativo de Linux. macOS funciona para desarrollo local. Windows no es compatible como plataforma de servidor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h3 id="steps">
  Pasos
</h3>

<Steps>
  <Step title="Registre un cliente OAuth en su IdP">
    Decida primero el nombre de host de la puerta de enlace, porque el URI de redirección debe coincidir con él. Cree una nueva aplicación web OIDC y establezca el URI de redirección en `https://claude-gateway.<your-domain>/oauth/callback`, donde el host es el mismo valor que establece como [`listen.public_url`](/docs/es/claude-apps-gateway-config#listen) en el paso 3. Anote el `client_id` y `client_secret`. Las instrucciones por IdP están en [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Aprovisione una base de datos PostgreSQL">
    Cualquier Postgres 14 o posterior funciona, incluido el nivel administrado más pequeño. La puerta de enlace ejecuta sus propias migraciones de esquema al arrancar, por lo que el rol de la base de datos necesita derechos para crear y alterar tablas; consulte [`store`](/docs/es/claude-apps-gateway-config#store).
  </Step>

  <Step title="Escriba gateway.yaml">
    Los secretos se leen a través de la expansión `${ENV_VAR}` para que el archivo en sí pueda vivir en control de versiones. Use un nombre de host `public_url` que se resuelva a una IP privada en su red, porque `/login` rechaza direcciones públicas. La configuración mínima tiene cinco secciones, y todos los demás campos tienen un valor predeterminado:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Requerido a menos que el host sea una dirección loopback. Utilizado para el IdP
      # redirect_uri y el documento de descubrimiento.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # debe servir /.well-known/openid-configuration
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # rechazar id_tokens fuera de su organización
      userinfo_fallback: true                  # para IdPs cuyo id_token omite email/groups; inofensivo de otra manera

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # también limita la latencia de revocación en desaprovisionamiento de IdP

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # agregue ?sslmode=require para Postgres administrado

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # vacío: cadena de credenciales predeterminada de AWS
    # (IRSA, rol de tarea EC2/ECS, variables de entorno, ~/.aws)

    # Los modelos se traducen por ascendente automáticamente. El catálogo integrado
    # asigna claude-opus-4-8 a us.anthropic.claude-opus-4-8 y así sucesivamente para cada
    # modelo Claude compatible con Bedrock. Establezca false y agregue una lista `models:` para
    # exponer solo modelos específicos.
    auto_include_builtin_models: true
    ```

    Esta configuración es suficiente para un bucle de inicio de sesión funcional con el catálogo de modelos predeterminado de Amazon Bedrock. Una vez que se ejecute, agregue RBAC por grupo y configuración administrada a través de [`managed.policies`](/docs/es/claude-apps-gateway-config#managed), distribución de telemetría a través de [`telemetry`](/docs/es/claude-apps-gateway-config#telemetry), y conmutación por error de múltiples ascendentes, ARNs de rendimiento aprovisionado, o regiones no estadounidenses a través de [`models`](/docs/es/claude-apps-gateway-config#models).

    <Note>
      El ascendente de Amazon Bedrock necesita un principal de AWS con `bedrock:InvokeModel` y `bedrock:InvokeModelWithResponseStream` en los ARNs `inference-profile/us.anthropic.*` y los ARNs `foundation-model/anthropic.*` subyacentes. También necesita el formulario de caso de uso único de Anthropic enviado para la cuenta desde el catálogo de modelos de la consola de Bedrock.

      Suministre la credencial con IRSA en EKS, un rol de tarea de ECS, o un perfil de instancia de EC2 en lugar de claves estáticas. La [referencia `upstreams`](/docs/es/claude-apps-gateway-config#upstreams) tiene los detalles completos de IAM, la matriz de credenciales entre nubes, y los bloques `auth` para los otros proveedores.
    </Note>
  </Step>

  <Step title="Ejecútelo">
    Construya una imagen de contenedor alrededor del binario `claude` que cumpla con los [requisitos de imagen](/docs/es/claude-apps-gateway-deploy#container-image), luego ejecútela junto con Postgres. El archivo Compose hace referencia a la imagen como `registry.example.com/claude-gateway:2.1.198`; sustituya su propio registro y etiqueta de imagen:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # Credenciales de AWS: en producción, omita estas y use un rol de instancia.
          # Para pruebas locales de Compose, pase las suyas propias:
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    La puerta de enlace es un único binario de Linux que lee la configuración, se conecta a Postgres y aplica sus migraciones de esquema, ejecuta el descubrimiento OIDC contra su IdP, construye clientes ascendentes, e inicia la escucha. El arranque es de cierre fallido para la configuración, la conexión de Postgres, el descubrimiento OIDC, y la construcción del cliente ascendente. Si alguno de esos es inaccesible o está mal configurado, la puerta de enlace sale con un error en lugar de servir tráfico en un estado degradado.

    Un arranque exitoso no valida la ruta de inferencia, porque las credenciales de instancia de Amazon Bedrock y la Plataforma de Agentes de Google Cloud se resuelven en la primera solicitud, no al arrancar.

    Observe stderr para la secuencia de arranque. Las líneas de registro utilizan el formato `[gateway] <timestamp> <level> <message>`, los eventos de auditoría son JSON de una sola línea con un campo `evt`, y un banner de inicio, omitido a continuación, se imprime entre las líneas de migración y escucha. Una base de datos nueva imprime una línea `migration N applied` por migración de esquema; una base de datos ya migrada no imprime ninguna. Debería ver, en orden:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    La puerta de enlace también registra una advertencia de que `access_control.allow_cidrs` está vacío. Eso es lo esperado aquí, porque nada limita qué direcciones de cliente sirve la puerta de enlace hasta que establezca una lista de permitidos. La [referencia `access_control`](/docs/es/claude-apps-gateway-config#http-tuning) tiene los rangos recomendados.

    Si el arranque sale antes de la línea `claude gateway listening on`, la última línea de stderr nombra el problema:

    * un Postgres inaccesible
    * un rol de Postgres sin permiso DDL
    * un documento de descubrimiento OIDC inaccesible o inválido
    * una violación del esquema de configuración con la ruta del campo ofensivo

    Corrija y reinicie.

    Si ya tiene una entrada que termina TLS, omita Compose y ejecute el binario directamente con `claude gateway --config gateway.yaml`. Establezca `public_url` en el origen de la entrada y vincule `listen` a una dirección de loopback o interna del clúster.
  </Step>

  <Step title="Verifique la superficie de autenticación">
    Tres verificaciones confirman que la puerta de enlace puede autenticar a un usuario real antes de entregársela a un desarrollador.

    Los ejemplos utilizan la URL pública de la puerta de enlace; para la configuración local de Compose sin una entrada, sustituya `http://localhost:8080` en las dos primeras verificaciones. La tercera verificación abre `verification_uri_complete`, que se construye a partir de `public_url`, por lo que para Compose local establezca `public_url: http://localhost:8080` en `gateway.yaml`, y agregue `http://localhost:8080/oauth/callback` como un segundo URI de redirección en el cliente OAuth del paso 1, porque la puerta de enlace construye el `redirect_uri` de IdP a partir de `public_url`. El enlace de verificación luego se abre en su navegador local.

    En Windows PowerShell, ejecute `curl.exe`; el `curl` simple es un alias para `Invoke-WebRequest` y rechaza estas banderas.

    Primero, obtenga el documento de descubrimiento, que confirma que la puerta de enlace está activa, la configuración es válida, y todas las verificaciones de arranque pasaron:

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    La respuesta incluye campos adicionales, como `response_types_supported` y `scopes_supported`.

    Segundo, solicite una autorización de dispositivo, que confirma que el flujo de inicio de sesión del dispositivo funciona y Postgres es accesible y escribible:

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    Tercero, pruebe la rama del navegador abriendo `verification_uri_complete` en un navegador y confirmando el código. Debería ser redirigido a la página de inicio de sesión de su IdP, y después de iniciar sesión, aterrizar de nuevo en la puerta de enlace con una confirmación de inicio de sesión.

    Use la primera verificación fallida para localizar el problema:

    * **Primera verificación falla**: el arranque no se completó; verifique stderr
    * **Segunda verificación falla**: Postgres no es accesible desde la puerta de enlace o el rol no puede escribir; verifique la cadena de conexión y los permisos
    * **Tercera verificación no llega al IdP**: verifique que el URI de redirección de IdP coincida exactamente con `https://<gateway>/oauth/callback`
    * **Tercera verificación llega al IdP pero rebota con un error**: lea el registro de auditoría de la puerta de enlace, que registra cada rechazo de autenticación con la razón, como `email domain not allowed`
  </Step>

  <Step title="Inicie sesión de un desarrollador">
    Este último paso ocurre en una máquina de desarrollador, no en el servidor. Establezca `forceLoginMethod` en `"gateway"` y `forceLoginGatewayUrl` en la `public_url` de su puerta de enlace en el [archivo de configuración administrada](/docs/es/managed-settings#delivery-mechanisms) de esa máquina, luego ejecute `/login`, presione Intro en la pantalla **Cloud gateway**, y complete el inicio de sesión del navegador. [Establezca la URL de la puerta de enlace](#set-the-gateway-url) a continuación cubre la distribución de ambas claves a cada máquina de desarrollador.
  </Step>
</Steps>

<h2 id="connect-developers">
  Conectar desarrolladores
</h2>

Los desarrolladores se conectan desde sus propias computadoras portátiles con un inicio de sesión de navegador, usando su cuenta de trabajo corporativa. No necesitan una cuenta de claude.ai, una clave API, o una suscripción, porque las solicitudes al modelo van a través de la puerta de enlace usando la credencial ascendente de la organización. La conexión es impulsada por la [configuración administrada del lado del cliente](/docs/es/claude-apps-gateway-config#client-side-managed-settings) que inserta a través de MDM, por lo que no hay configuración manual en el lado del desarrollador; esta sección cubre lo que configura el administrador.

La CLI toma la huella digital del certificado TLS de hoja de la puerta de enlace en la primera conexión y la fija por nombre de host. Verifica esa fijación nuevamente durante el inicio de sesión, en actualizaciones silenciosas de sesión, y en obtenciones de configuración administrada, mientras que las solicitudes de inferencia usan validación TLS estándar sin la fijación. Las solicitudes enrutadas a través de un proxy HTTPS omiten la verificación de fijación, por lo que agregue el host de la puerta de enlace a `NO_PROXY` para mantenerlas directas.

Publique la huella digital SHA-256 esperada junto con la URL de la puerta de enlace para que los desarrolladores tengan algo con lo que comparar. El indicador `/login` muestra los primeros 16 caracteres de la huella digital como hexadecimal minúscula sin dos puntos. Para imprimir la huella digital completa en esa forma desde el archivo de certificado, ejecute:

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

Cuando el certificado rota, cada desarrollador ve el indicador de confianza nuevamente, por lo que trate las rotaciones como un evento planificado y republique la huella digital. Si su política de puerta de enlace incluye [configuración que necesita aprobación](/docs/es/server-managed-settings#security-approval-dialogs), el desarrollador también ve ese diálogo de aprobación nuevamente después de aceptar el nuevo certificado, porque Claude Code vincula [memoria de aprobación](/docs/es/server-managed-settings#approval-memory) al certificado fijado.

Una puerta de enlace puede devolver el campo opcional `email` en su respuesta de token para nombrar la cuenta que usó un inicio de sesión. Cuando lo hace, el desarrollador confirma la cuenta antes de que Claude Code guarde la credencial. Después de un inicio de sesión confirmado, `/status` muestra la cuenta.

La confirmación requiere Claude Code v2.1.275 o posterior en la máquina del desarrollador; un cliente por debajo de esa versión ignora el campo. El servidor de la puerta de enlace en el binario `claude` no devuelve el campo, por lo que sus inicios de sesión se completan sin la confirmación.

Una vez que el desarrollador inicia sesión, el [selector de modelos](/docs/es/model-config) muestra los modelos en su lista de permitidos `availableModels`. La configuración administrada se aplica al inicio y se actualiza cada hora, y la telemetría se enruta a su recopilador.

Las sesiones se actualizan silenciosamente antes de la expiración de `ttl_hours`. Cuando una actualización falla después del desaprovisionamiento de IdP, Claude Code solicita al desarrollador que inicie sesión nuevamente.

<h3 id="set-the-gateway-url">
  Establecer la URL de la puerta de enlace
</h3>

Tres claves van en el archivo de [configuración administrada](/docs/es/managed-settings#delivery-mechanisms) por sistema operativo que implementa a través de MDM o directamente en el disco. `forceLoginMethod` y `forceLoginGatewayUrl` abren `/login` directamente en la pantalla **Cloud gateway** con la URL rellenada, y `parentSettingsBehavior: "merge"` permite que Claude Desktop entregue la lista de permitidos de salida de la puerta de enlace a las sesiones de Claude Code que lanza, explicado en [Entregar política a sesiones de Claude Desktop](#deliver-policy-to-claude-desktop-sessions):

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

El desarrollador presiona Intro para conectarse. El [indicador de huella digital TLS de primera conexión](#connect-developers) aún aparece. Una vez que el archivo está en una máquina, un desarrollador que no ha completado el inicio de sesión de la puerta de enlace ve uno de los mensajes descritos en [La política del administrador requiere un inicio de sesión de Cloud gateway](/docs/es/errors#administrator-policy-requires-a-cloud-gateway-sign-in). Los desarrolladores que seleccionan un proveedor de nube a través de una variable de entorno como `CLAUDE_CODE_USE_BEDROCK` no necesitan el inicio de sesión de la puerta de enlace.

Un desarrollador no puede configurar esto manualmente. El selector de inicio de sesión no tiene opción de puerta de enlace, y `forceLoginGatewayUrl` se ignora en los archivos de configuración propios de un desarrollador. `forceLoginMethod` solo, sin una URL, deja al desarrollador en un mensaje "Contacte a su administrador de TI". Las claves de inicio de sesión pertenecen al archivo que inserta en máquinas, no en el bloque `managed.policies[].cli` de la puerta de enlace, que solo llega a clientes que ya están conectados.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  Permitir una puerta de enlace en espacio de direcciones público que usted posee
</h3>

Algunas organizaciones numeran su red interna desde un bloque IPv4 público que poseen, como el espacio de direcciones propio de un operador o un `/8` heredado, por lo que su puerta de enlace no puede tener una dirección privada. Liste esos bloques en la configuración administrada `gatewayInternalNetworks`. `/login` entonces acepta una puerta de enlace dentro de un bloque listado cuando la máquina del desarrollador se conecta a ella desde una dirección dentro del mismo bloque. Esto requiere Claude Code v2.1.268 o posterior en la máquina del desarrollador; las versiones anteriores ignoran la clave y aplican la regla de dirección privada.

<Warning>
  `gatewayInternalNetworks` es para redes internas que resultan estar numeradas desde espacio de direcciones público. No hace que sea seguro exponer una puerta de enlace a internet: una puerta de enlace confiable puede insertar configuración que ejecute comandos en máquinas de desarrolladores.

  Mantenga la puerta de enlace inaccesible desde fuera de su red con sus reglas de firewall o balanceador de carga. Establezca el [`access_control.allow_cidrs`](/docs/es/claude-apps-gateway-config#http-tuning) de la puerta de enlace a los mismos bloques que declara aquí, para que la puerta de enlace en sí rechace clientes desde cualquier otro lugar. Detrás de un balanceador de carga o ingress, establezca también `listen.trusted_proxies` a ese front end, porque la puerta de enlace de otro modo coincide con `allow_cidrs` contra la dirección del front end en sí en lugar de la del desarrollador.
</Warning>

Agregue la clave a la misma fuente de configuración administrada que las claves de inicio de sesión: el archivo de configuración administrada, perfil MDM, o política de registro. Claude Code la ignora en configuración administrada por el usuario, proyecto, y servidor.

Este ejemplo declara un bloque. Reemplace `203.0.113.0/24` con su propio bloque. Es un rango de documentación, y Claude Code rechaza esos.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code valida la lista en `/login` antes de que contacte a cualquier puerta de enlace:

* Cada entrada es un bloque IPv4 escrito como su primera dirección y un prefijo de `/8` a `/32`.
* La lista contiene como máximo cuatro bloques, y ninguno dos se superponen.
* Ningún bloque se superpone con espacio de direcciones privadas: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16`, y `100.64.0.0/10`. `/login` ya acepta una puerta de enlace allí sin esta clave.
* Ningún bloque se superpone con espacio que nunca es la red de una organización: `198.18.0.0/15` y `192.0.0.0/24`, que clientes VPN y NAT64 mantienen como direcciones locales; los rangos de documentación `192.0.2.0/24`, `198.51.100.0/24`, y `203.0.113.0/24`; y los rangos reservados `0.0.0.0/8`, `192.88.99.0/24`, y multidifusión `224.0.0.0/4`. Puede declarar bloques dentro de `240.0.0.0/4`, que algunas redes grandes usan como espacio de unidifusión interno.

Los bloques de `managed-settings.json` y sus archivos drop-in `managed-settings.d/` se combinan en una lista, y estos límites se aplican a la lista combinada. Para estrechar un bloque, reemplace su entrada en lugar de agregar una segunda, superpuesta en un drop-in; `/login` rechaza la superposición.

Si una entrada rompe una regla, o el valor no es una lista de cadenas, Claude Code rechaza cada nuevo inicio de sesión de puerta de enlace en esa máquina y nombra el problema en el mensaje. El inicio de sesión a una puerta de enlace en una dirección privada también falla, y los inicios de sesión existentes siguen funcionando. Intente el valor en una máquina antes de implementarlo. Claude Code también lista un valor tipado incorrectamente entre la [configuración administrada inválida que reporta](/docs/es/managed-settings#keys-that-fail-closed).

Con una lista válida, `/login` aplica tres verificaciones a una puerta de enlace cuya dirección está dentro de un bloque listado:

* Cada dirección a la que se resuelve el nombre de host de la puerta de enlace está dentro de ese bloque. Claude Code rechaza un nombre que también tiene registros fuera de él, direcciones privadas e IPv6 incluidas.
* La máquina del desarrollador se conecta desde dentro del mismo bloque. Claude Code rechaza una máquina detrás de NAT, dentro de un contenedor o WSL2, o en una VPN cuyo grupo de direcciones se encuentra fuera del bloque, y nombra la dirección desde la que se conectó la máquina.
* La conexión es directa. Si `HTTPS_PROXY` se aplica al host de la puerta de enlace, `/login` rechaza y nombra la entrada `NO_PROXY` a agregar.

Cuando los tres pasan, el [indicador de confianza](#connect-developers) agrega una línea nombrando la dirección de la máquina, la dirección de la puerta de enlace, y el bloque declarado que contiene ambas.

La clave no cambia nada para otras puertas de enlace: el inicio de sesión a una en una dirección privada funciona como antes, y el inicio de sesión a una en una dirección pública fuera de cada bloque listado se rechaza como antes.

Un bloque declarado estrecha quién puede iniciar sesión pero no prueba dónde está una máquina, por lo que declare solo espacio de direcciones que su organización controla. Un bloque compartido con otros inquilinos, como un rango público de un proveedor de nube, permite que cualquiera en él pase la misma verificación.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Entregar política a sesiones de Claude Desktop
</h3>

Claude Desktop ejecuta sus pestañas Cowork y Code, más la pestaña Chat cuando la habilita, en sesiones de Claude Code incrustadas y envía sus solicitudes de modelo a través de la puerta de enlace. Pasa política a cada una de esas sesiones, construida a partir de la configuración que la puerta de enlace le sirve en `/user/bootstrap`: la lista de permitidos de modelos, herramientas deshabilitadas, y lista de permitidos de salida derivada del bloque `cli` de la política coincidente, más la [superposición `desktop`](/docs/es/claude-apps-gateway-config#claude-desktop-overlay).

Otras claves `cli`, como hooks, `env`, y reglas de permiso con alcance como `Bash(npm *)`, llegan solo a clientes que inician sesión a través de `/login`. Claude Desktop lee la URL de la puerta de enlace de su propia configuración administrada e inicia sesión con su propio flujo, separado de las claves `forceLoginMethod` y `forceLoginGatewayUrl` en [Establecer la URL de la puerta de enlace](#set-the-gateway-url).

La configuración pasada por un proceso de lanzamiento son configuración principal. Claude Code ignora la configuración principal en cualquier máquina que tenga una fuente administrada implementada por el administrador, a menos que la [fuente que entrega la política](/docs/es/managed-settings#which-managed-source-claude-code-uses) establezca `parentSettingsBehavior: "merge"`.

<h4 id="which-machines-need-the-opt-in">
  Qué máquinas necesitan la opción de participación
</h4>

Las máquinas que solo ejecutan Claude Desktop la necesitan. Claude Desktop aplica la lista de modelos y la lista de herramientas deshabilitadas a sesiones incrustadas en sí mismo, pero la lista de permitidos de salida las alcanza solo como configuración principal, en forma de reglas de dominio `WebFetch` y reglas de red de sandbox. Sin la opción de participación, esas sesiones se ejecutan sin la restricción de salida, y nada le advierte. La puerta de enlace aún rechaza solicitudes de inferencia para modelos que la política no otorga.

Las máquinas donde los desarrolladores inician sesión a través de `/login` no la necesitan; cada sesión de Claude Code obtiene su política de la puerta de enlace.

Las flotas cuyo [`policyHelper`](/docs/es/settings-reference#policyhelper) suministra configuración administrada no pueden usarla: Claude Code nunca fusiona configuración principal en esas flotas, porque lee configuración administrada solo de la salida del asistente.

<h4 id="set-the-opt-in">
  Establecer la opción de participación
</h4>

Implemente el fragmento de configuración administrada de [Establecer la URL de la puerta de enlace](#set-the-gateway-url), espéjelo a cualquier fuente del lado del cliente que supere al archivo, luego verifique.

<Steps>
  <Step title="Implemente la opción de participación en el archivo de configuración administrada">
    El [fragmento anterior](#set-the-gateway-url) ya incluye `parentSettingsBehavior: "merge"`, por lo que el archivo que inserta en máquinas lo lleva.
  </Step>

  <Step title="Espeje el fragmento a cualquier fuente que supere al archivo">
    Claude Code lee `parentSettingsBehavior` solo de la [fuente seleccionada](/docs/es/managed-settings#which-managed-source-claude-code-uses). Agregar cualquier clave de política a una fuente puede hacer que esa fuente sea la seleccionada, por lo que en una fuente del lado del cliente, espeje el fragmento completo en lugar de solo `parentSettingsBehavior`. [Configuración administrada del lado del cliente](/docs/es/claude-apps-gateway-config#client-side-managed-settings) cubre flotas que entregan política a través de Group Policy o perfiles de configuración. Un plist de preferencias administradas en macOS o una política HKLM en Windows supera al archivo `managed-settings.json`, y la configuración administrada remota de la puerta de enlace supera a ambas, por lo que en máquinas que inician sesión en la puerta de enlace, también establezca `parentSettingsBehavior` en el bloque [`cli`](/docs/es/claude-apps-gateway-config#managed) de la política de la puerta de enlace.
  </Step>

  <Step title="Verifique qué fuente está seleccionada">
    En una máquina que solo ejecuta Claude Desktop, llame al [`resolveSettings()`](/docs/es/agent-sdk/typescript#resolvesettings) del SDK del Agente y lea `policyOrigin` en la entrada `managed` en su lista `sources`. El valor nombra la fuente del lado del cliente seleccionada, `plist`, `hklm`, o `file`, que es la fuente que debe llevar el fragmento. Las sesiones incrustadas de Claude Desktop no obtienen la política de la puerta de enlace, por lo que el bloque `cli` de la puerta de enlace nunca cuenta como la fuente seleccionada para ellas.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  Restringir configuración principal
</h3>

Una vez que implemente `parentSettingsBehavior: "merge"`, cualquier proceso de host que lance Claude Code puede suministrar configuración principal, no solo Claude Desktop sino también una aplicación del SDK del Agente o una extensión de IDE.

Claude Code filtra la configuración principal contra una lista de permitidos de claves restrictivas, pero algunas claves permitidas pueden otorgar acceso en lugar de restringirlo. A menos que establezca los bloqueos `allowManaged*Only`, las reglas de permiso de permitir y las listas de permitidos de sandbox suministradas por el host aún se aplican. Las reglas de negación y pregunta de su política permanecen en vigor de cualquier manera; [se evalúan antes de cualquier regla de permitir](/docs/es/permissions#manage-permissions).

Claude Code reenvía entradas [`sandbox.credentials`](/docs/es/settings-reference#sandbox-credentials) suministradas por el principal en forma despojada:

* **Entradas `deny`**: reenviadas solo con su `path` o `name` y el modo.
* **Entradas de archivo con [`mode: mask`](/docs/es/sandboxing#mask-credential-files)**: reenviadas solo centinela, como una máscara de archivo completo cuyo `injectHosts` es la lista vacía, por lo que el proxy nunca sustituye el valor real por una entrada suministrada por el principal en ninguna plataforma. Todos los campos de enmascaramiento estructurado también se descartan, por lo que un patrón de extracción suministrado por el principal no puede desplazar una máscara más estricta que otra fuente establece para la misma ruta.
* **Entradas `envVars` con `mode: mask`**: no reenviadas. `deny` es la única restricción que el canal principal puede expresar a través de entradas `envVars`.
* **[`awsPairs` y `sigv4`](/docs/es/sandboxing#re-sign-aws-requests)**: reenviadas solo restricción. De `sigv4`, solo se mantienen valores `deny`, y un principal que define un bloque `sigv4` en absoluto fija las tres formas de solicitud, `streaming`, `presigned`, y `sigv4a`, a `deny`. Un par `awsPairs` nunca se reenvía en una forma que pueda volver a firmar; un par que nombra una de las variables AWS convencionales se reemplaza por una entrada inerte que mantiene el emparejamiento automático de `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, y `AWS_SESSION_TOKEN` suprimido.

<h4 id="deploy-the-locks">
  Implemente los bloqueos
</h4>

Para mantener la configuración principal lo más cercana a solo restricción como el filtro admite, agregue los cinco bloqueos `allowManaged*Only`, y las listas de permitidos que rigen, a las mismas fuentes que la opción de participación de fusión:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

Una política del sistema operativo, como una política de registro HKLM o un plist de preferencias administradas, supera a este archivo, por lo que entregue el fragmento completo a través de él en lugar del archivo. La configuración administrada remota de la puerta de enlace supera a las fuentes de política del sistema operativo y archivo pero llega solo a clientes conectados. Espeje los bloqueos, las listas de permitidos, y la opción de participación de fusión en el bloque [`cli`](/docs/es/claude-apps-gateway-config#managed) de la política y mantenga este archivo implementado, porque las máquinas que nunca se conectan, incluidas las que solo ejecutan Claude Desktop, obtienen su política solo del archivo.

<h4 id="lock-behavior-across-sources">
  Comportamiento de bloqueo entre fuentes
</h4>

Establecer un bloqueo no restringe los otros; cada clave está documentada en la [referencia de configuración](/docs/es/settings-reference#all-settings).

De una fuente de administrador por debajo del ganador, los dos bloqueos de sandbox aún se aplican, y `allowManagedPermissionRulesOnly` aún bloquea reglas de permitir suministradas por el principal y `additionalDirectories`. En Claude Code v2.1.273 o posterior, el bloqueo de servidor MCP también se aplica desde una fuente por debajo del ganador, y mientras esté activado, la lista administrada `allowedMcpServers` proviene de la fuente de administrador de mayor prioridad que establece una.

El bloqueo de hooks y el efecto de `allowManagedPermissionRulesOnly` en las reglas propias del desarrollador necesitan la fuente ganadora por defecto; bajo la opción de participación de fusión `managedSourcesBehavior` en [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources), Claude Code aplica el valor más estricto que cualquier fuente establece para cada bloqueo. En flotas [`policyHelper`](/docs/es/settings-reference#policyhelper), Claude Code lee los bloqueos solo de la salida del asistente.

Cada bloqueo hace que Claude Code ignore las entradas propias del desarrollador para esa configuración, por lo que incluya las listas de permitidos de su organización junto a los bloqueos:

* **Dominios de red**: bloquear con una lista de dominios administrados vacía bloquea todo el tráfico saliente en sandbox.
* **Servidores MCP**: bloquear sin `allowedMcpServers` en ninguna fuente de administrador o en la configuración suministrada por el principal carga cada servidor que `deniedMcpServers` no bloquea.
* **Rutas de lectura**: las entradas `allowRead` solo vuelven a permitir rutas dentro de regiones `denyRead`, por lo que emparéjelas con un `denyRead` administrado.

<h4 id="settings-the-locks-don’t-cover">
  Configuración que los bloqueos no cubren
</h4>

Seis configuraciones suministradas por el principal pasan el filtro incluso con los cinco bloqueos establecidos. Bajo la configuración de primer ganador predeterminada, el valor de administrador que bloquea el del principal es el de la fuente de administrador de mayor prioridad, excepto para `allowedMcpServers` mientras el [bloqueo de servidor MCP](#lock-behavior-across-sources) esté activado. Bajo la opción de participación de fusión `managedSourcesBehavior`, [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) dice qué valor de fuente se aplica en su lugar.

* **`forceLoginOrgUUID`**: Claude Code honra un valor suministrado por el principal cuando la fuente de administrador de mayor prioridad no establece un UUID de organización. El inicio de sesión de la puerta de enlace no verifica esta clave, por lo que importa solo para flotas que también usan inicios de sesión de Anthropic de primera parte. Un UUID de organización en la fuente de administrador de mayor prioridad bloquea el valor del principal y es el que Claude Code aplica, por lo que establezca `forceLoginOrgUUID` allí.
* **`allowedMcpServers`**: Claude Code honra una lista de permitidos suministrada por el principal cuando ninguna lista de administrador está en vigor. `allowManagedMcpServersOnly` no la bloquea, porque el bloqueo aplica cualquier lista que gane como el valor administrado, incluida una lista suministrada por el principal cuando ninguna fuente de administrador suministra una. Una lista en la fuente de administrador de mayor prioridad bloquea la del principal y es la lista que Claude Code aplica, por lo que establezca `allowedMcpServers` allí, junto al bloqueo. Antes de v2.1.223, un valor para cualquiera de las claves en cualquier fuente de administrador bloqueaba la del principal.
* **`availableModels`**: Claude Code honra una lista de modelos suministrada por el principal cuando la fuente administrada ganadora no establece una. Si su flota restringe modelos, establezca `availableModels` en la fuente ganadora.
* **`strictKnownMarketplaces`**: Claude Code honra una lista de permitidos de mercado de plugins suministrada por el principal cuando la fuente administrada ganadora no establece una. Si su flota restringe mercados, establezca `strictKnownMarketplaces` en la fuente ganadora. Requiere Claude Code v2.1.282 o posterior.
* **`blockedMarketplaces`**: una lista de bloqueo de mercado suministrada por el principal pasa y se suma a cualquier lista de bloqueo que una fuente administrada establece, ya que una lista de bloqueo solo puede restringir más. Requiere Claude Code v2.1.282 o posterior.
* **`strictPluginOnlyCustomization`**: esta clave pasa el filtro independientemente de cualquier bloqueo, y hace que Claude Code ignore la personalización propia del desarrollador, incluidos hooks protectores. Ningún bloqueo la bloquea.

<h3 id="connect-claude-desktop">
  Conectar Claude Desktop
</h3>

[Claude Desktop](/docs/es/desktop) se conecta a la misma puerta de enlace a través de una clave MDM diferente: establezca `bootstrapUrl` en la [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop a `<listen.public_url>/user/bootstrap`, y opte por la política del usuario con una clave `desktop`. [Superposición de Claude Desktop](/docs/es/claude-apps-gateway-config#claude-desktop-overlay) cubre ambas mitades. Requiere Claude Code v2.1.203 o posterior en el servidor de la puerta de enlace.

Claude Desktop firma al desarrollador a través del proveedor de identidad de la puerta de enlace con el mismo paso de SSO del navegador, luego obtiene su configuración de la puerta de enlace en lugar de Anthropic. El acceso a modelos y la política siguen las mismas reglas por grupo que la CLI. Un desarrollador que usa tanto la CLI como Claude Desktop inicia sesión en cada uno por separado; la sesión de la puerta de enlace no se comparte entre ellas.

Una vez conectado, Claude Desktop envía solicitudes de modelo de cada pestaña habilitada a través de la puerta de enlace. Muestra las pestañas Cowork y Code por defecto. Para activar también la pestaña Chat, establezca `chatTabEnabled` a `true` en la [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration) de Claude Desktop, o en el bloque [`desktop`](/docs/es/claude-apps-gateway-config#claude-desktop-overlay) de la política en una puerta de enlace que ejecuta Claude Code v2.1.227 o posterior.

<h3 id="ci-pipelines-and-remote-machines">
  Canalizaciones de CI y máquinas remotas
</h3>

No hay flujo de token de servicio para canalizaciones desatendidas. El inicio de sesión de la puerta de enlace siempre ejecuta el flujo de dispositivo del navegador, por lo que un trabajo de CI sin un desarrollador para aprobar el inicio de sesión no puede autenticarse; configure esos contra su proveedor directamente.

Una vez que un desarrollador ha iniciado sesión, cada sesión de Claude Code en esa máquina usa la sesión de la puerta de enlace, incluidas las ejecuciones no interactivas de `claude -p` y sesiones iniciadas por el SDK del Agente. Claude Code aplica la [política de la puerta de enlace](/docs/es/claude-apps-gateway-config#managed) a cada una de ellas.

El flujo de dispositivo separa la CLI de sondeo de la aprobación del navegador, por lo que una caja de desarrollo remota sin pantalla aún funciona: el desarrollador ejecuta `/login` sobre SSH en la máquina remota y abre el enlace de verificación en el navegador en su computadora portátil.

<h3 id="whats-enforced-on-developers">
  Qué se aplica en los desarrolladores
</h3>

Estas garantías se aplican a cada sesión conectada a través de `/login`. Las sesiones incrustadas que Claude Desktop lanza obtienen su política como se describe en [Entregar política a sesiones de Claude Desktop](#deliver-policy-to-claude-desktop-sessions), y la viñeta de telemetría dice dónde van sus exportaciones.

* **Acceso a modelos**: las solicitudes de modelos que la política no otorga devuelven 400, y el selector `/model` se filtra a la lista de permitidos `availableModels` de la política. Establezca [`enforceAvailableModels: true`](/docs/es/model-config#default-model-behavior) en la política para que la opción Predeterminado se resuelva a un modelo dentro de `availableModels` en lugar de al predeterminado integrado de Claude Code; sin él, Predeterminado permanece seleccionable y se rechaza en el tiempo de solicitud si ese modelo no se otorga.
* **Destino de telemetría**: en sesiones conectadas a través de `/login`, la CLI envía sus exportaciones OTLP/HTTP a la puerta de enlace en lugar de a un `OTEL_EXPORTER_OTLP_ENDPOINT` establecido localmente, a menos que una política [nombre su recopilador como el punto final](/docs/es/claude-apps-gateway-config#export-directly-to-your-collector). La puerta de enlace retransmite las exportaciones que recibe a los destinos en [`telemetry.forward_to`](/docs/es/claude-apps-gateway-config#telemetry).
  * En las sesiones incrustadas que [Claude Desktop lanza](#connect-claude-desktop), la CLI envía sus exportaciones al `OTEL_EXPORTER_OTLP_ENDPOINT` configurado. La CLI adjunta el token de sesión de la puerta de enlace a esas exportaciones solo cuando ese punto final apunta a la puerta de enlace en sí.
  * Sin destino configurado para una señal, la puerta de enlace la acepta y descarta.
  * Si ya recopila telemetría de Claude Code directamente, agregue su recopilador como destino `forward_to`, o nombre su recopilador en una política para omitir el relé.
* **Credenciales**: el token de la puerta de enlace es la única credencial de la sesión. [Perfiles de Anthropic](/docs/es/authentication#anthropic-profiles-and-federation-credentials) y cualquier inicio de sesión anterior de claude.ai se ignoran mientras se conecta, por lo que los desarrolladores no necesitan cerrar sesión de claude.ai primero. Para una credencial `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper` configurada, vea [La política del administrador requiere un inicio de sesión de Cloud gateway](/docs/es/errors#administrator-policy-requires-a-cloud-gateway-sign-in).
* **Configuración administrada**: las claves bloqueadas no se pueden anular localmente. La CLI aplica la política al inicio y aplica cambios en cada sondeo cada hora, aparte de los [cambios que se aplican solo en el siguiente lanzamiento](/docs/es/server-managed-settings#fetch-and-caching-behavior).
* **Inicio con la puerta de enlace inaccesible**: las sesiones conectadas salen al inicio con un error después de aproximadamente 10 segundos en lugar de iniciarse sin su configuración.
* **Inicio después de que la puerta de enlace termina la sesión**: vea [Aplicar inicio cerrado por fallo](/docs/es/server-managed-settings#enforce-fail-closed-startup) para los lanzamientos que se abren desconectados de la puerta de enlace y los que salen cuando la puerta de enlace responde con un `401`.
* **Desaprovisionamiento**: una sesión cuyo usuario está deshabilitado en el IdP expira dentro de `ttl_hours` cuando la siguiente actualización falla.
* **Cierre de sesión**: `/logout` elimina la credencial de la puerta de enlace de la máquina del desarrollador.
  * Cuando el documento de descubrimiento de la puerta de enlace anuncia un `revocation_endpoint` en el esquema, host y puerto propios de la URL de la puerta de enlace, `/logout` también envía los tokens almacenados a ese punto final para que la puerta de enlace pueda terminar la sesión en su lado. La solicitud es de mejor esfuerzo, por lo que el cierre de sesión se completa en la máquina del desarrollador independientemente de si el punto final responde. La revocación requiere Claude Code v2.1.275 o posterior en la máquina del desarrollador.
  * El servidor de la puerta de enlace en el binario `claude` no anuncia ninguno, por lo que un cierre de sesión desde él termina la sesión en la máquina del desarrollador solamente. Para forzar sesiones fuera del lado del servidor, vea [Rotación de secreto JWT](/docs/es/claude-apps-gateway-deploy#jwt-secret-rotation).

<h3 id="what-the-organization-can-see">
  Qué puede ver la organización
</h3>

La telemetría de uso lleva la identidad del desarrollador, recuentos de tokens, modelo y latencia al recopilador de la organización. La puerta de enlace no registra ni almacena contenido de indicación o finalización. Si se recopila telemetría más rica como registros y trazas, que puede incluir comandos y rutas de archivo, es la [opción por destino](/docs/es/claude-apps-gateway-config#telemetry) de la organización.

<h2 id="availability-and-limitations">
  Disponibilidad y limitaciones
</h2>

La tabla cubre qué características de Claude Code funcionan cuando los desarrolladores se conectan a través de la puerta de enlace, y qué soporta el servidor de puerta de enlace en sí. Donde algo no es compatible, la columna Notas proporciona la alternativa.

La puerta de enlace entrega los valores [`anthropic-beta`](https://platform.claude.com/docs/es/api/beta-headers) que la CLI envía a cada ascendente, por lo que los operadores no mantienen una lista de permitidos de beta. Para Amazon Bedrock, que ignora el encabezado, la puerta de enlace mueve los valores al campo `anthropic_beta` del cuerpo de la solicitud; los otros ascendentes reciben el encabezado como se envía.

| Característica                                                                                                               | Estado                                | Notas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reenvío de inferencia (Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud, Microsoft Foundry, Anthropic) | Disponible                            | Con traducción de modelos por ascendente y conmutación por error. El ascendente de Amazon Bedrock utiliza el punto final `bedrock-runtime` y la cadena de credenciales predeterminada de AWS; el [punto final Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint) de Amazon Bedrock no es un ascendente compatible. El [ascendente Claude Platform en AWS](/docs/es/claude-apps-gateway-config#claude-platform-on-aws) requiere Claude Code v2.1.198 o posterior en el servidor de puerta de enlace.                                             |
| Acceso a modelos y configuración administrada por grupo de IdP                                                               | Disponible                            | El acceso a modelos se aplica del lado del servidor; la configuración administrada se entrega por grupo de IdP y se aplica por la CLI en el [nivel de configuración administrada](/docs/es/settings#settings-precedence)                                                                                                                                                                                                                                                                                                                         |
| Claude Desktop                                                                                                               | Disponible con participación opcional | La puerta de enlace sirve la configuración de Claude Desktop en `/user/bootstrap` una vez que una política [participa con una clave `desktop`](/docs/es/claude-apps-gateway-config#claude-desktop-overlay), y Claude Desktop envía solicitudes de modelo desde sus pestañas Cowork y Code, y desde la pestaña Chat cuando la habilita, a través de la puerta de enlace. Para activar la pestaña Chat, consulte [Conectar Claude Desktop](#connect-claude-desktop). Requiere Claude Code v2.1.203 o posterior en el servidor de puerta de enlace. |
| Distribución de telemetría (OTLP/HTTP)                                                                                       | Disponible                            | Identidad marcada por exportación; ambas codificaciones protobuf y JSON                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Proveedores de identidad OIDC                                                                                                | Disponible                            | Cualquier IdP compatible con OIDC; la puerta de enlace ejecuta el descubrimiento OIDC estándar y el flujo de código de autorización. Consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para la configuración por IdP                                                                                                                                                                                                                                                              |
| Límites de gasto por usuario y por grupo                                                                                     | Disponible                            | Consulte [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Búsqueda web del lado del servidor                                                                                           | No disponible                         | La CLI no puede ver qué proveedor ascendente enruta la puerta de enlace, por lo que no puede verificar la compatibilidad de búsqueda web y deshabilita WebSearch en sesiones de puerta de enlace                                                                                                                                                                                                                                                                                                                                            |
| [Remote Control](/docs/es/remote-control)                                                                                         | No disponible                         | La CLI muestra [un error que nombra la puerta de enlace](/docs/es/errors#remote-control-requires-the-anthropic-api)                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`/design-sync`](/docs/es/commands#all-commands) y `/design-login`                                                                | No disponible                         | Ambos necesitan claude.ai, que la CLI no contacta en sesiones de puerta de enlace, por lo que ninguno de los dos comandos aparece allí                                                                                                                                                                                                                                                                                                                                                                                                      |
| Características que necesitan obtención de indicadores de características, como `/import` e `claude import`                  | No disponible                         | La CLI omite la obtención de indicadores en sesiones de puerta de enlace. [Características que necesitan obtención de indicadores de características](/docs/es/env-vars#features-that-need-feature-flag-fetching) enumera lo que eso desactiva                                                                                                                                                                                                                                                                                                   |
| Almacenamiento en caché de indicación estándar                                                                               | Disponible                            | La puerta de enlace reenvía los puntos de interrupción `cache_control` a cada ascendente. [Dónde vive el caché](/docs/es/prompt-caching#where-the-cache-lives) cubre qué bloques marca la CLI, incluido el contexto del sistema que añade a mitad de la conversación                                                                                                                                                                                                                                                                             |
| TTL de caché de 1 hora                                                                                                       | No disponible                         | La CLI omite la beta de ttl de caché extendido en sesiones de puerta de enlace, porque no todos los ascendentes a los que la puerta de enlace puede enrutar soportan el TTL de 1 hora, por lo que el almacenamiento en caché de indicación a través de la puerta de enlace utiliza el TTL de 5 minutos; consulte la nota de encabezado de beta anterior                                                                                                                                                                                     |
| Modo automático                                                                                                              | Disponible                            | Sigue las [reglas del proveedor de terceros](/docs/es/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): solo los modelos elegibles en proveedores de terceros pueden usarlo. Antes de v2.1.207, el modo automático en sesiones de puerta de enlace requería establecer `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, entregable a través del bloque `env` de política administrada                                                                                                                                                    |
| Optimizaciones solo de primera parte como alcance de caché global y herramientas eficientes en tokens                        | No disponible                         | La CLI no las habilita en sesiones de puerta de enlace; consulte la nota de encabezado de beta anterior                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| OTLP/gRPC                                                                                                                    | No compatible                         | OTLP sobre HTTP solo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| SAML, LDAP y otra autenticación no OIDC                                                                                      | No compatible                         | Solo OIDC. Frente con un puente OIDC si es necesario                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Multi-inquilino (múltiples emisores OIDC)                                                                                    | No compatible                         | Un emisor por puerta de enlace. Ejecute instancias separadas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Servidor Windows                                                                                                             | No compatible                         | Implemente en Linux. macOS solo para desarrollo local                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Gráfico Helm                                                                                                                 | No disponible                         | La puerta de enlace se ejecuta como un Deployment sin estado estándar; consulte la [guía de implementación](/docs/es/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                                                                      |
| Interfaz de usuario de administrador                                                                                         | No disponible                         | La configuración es el archivo YAML; reimplemente para cambiarlo                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

<h2 id="next-steps">
  Próximos pasos
</h2>

El inicio rápido lo deja con una configuración mínima ejecutándose bajo Docker Compose. Para llevarlo más lejos:

* Expanda `gateway.yaml` más allá de la configuración mínima, por ejemplo para agregar RBAC por grupo, conmutación por error de múltiples ascendentes, o destinos de telemetría. La [referencia de configuración](/docs/es/claude-apps-gateway-config) cubre cada opción.
* Pase de Compose a una implementación de producción en Kubernetes o Cloud Run, configure su IdP correctamente, y revise el modelo de seguridad. La [guía de implementación y operaciones](/docs/es/claude-apps-gateway-deploy) cubre la configuración por IdP, los requisitos de imagen de contenedor, sondeos de salud y solución de problemas.
* Coloque límites de gasto en desarrolladores individuales o grupos para que una carga de trabajo descontrolada no pueda consumir todo su compromiso. [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits) cubre la API de administrador y cómo funciona la aplicación.
* Para un ejemplo completo trabajado en AWS, con ECS Fargate o EKS, Amazon RDS y Secrets Manager, consulte [Implementar en AWS](/docs/es/claude-apps-gateway-on-aws).
* Para un ejemplo completo trabajado en Google Cloud, con Cloud Run, Cloud SQL y Secret Manager, consulte [Implementar en Google Cloud](/docs/es/claude-apps-gateway-on-gcp).
