> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Puerta de enlace de aplicaciones Claude para Amazon Bedrock, Claude Platform en AWS, Google Cloud y Microsoft Foundry

> Ejecute Claude Code a través de Amazon Bedrock, Claude Platform en AWS, Google Cloud o Microsoft Foundry detrás de una puerta de enlace autohospedada con inicio de sesión SSO, acceso a modelos por grupo y telemetría OTLP.

<Note>
  La puerta de enlace de aplicaciones Claude está diseñada para organizaciones que deben —o prefieren— enrutar la inferencia a través de su propio proveedor de nube, por ejemplo para cumplir con los requisitos de [residencia de datos](/docs/es/claude-apps-gateway-deploy#compliance-posture). Si no tiene este requisito y desea acceso a otras características como aprovisionamiento SCIM o Claude Code en web y dispositivos móviles, Claude Enterprise puede ser una mejor opción. Consulte la página de [disponibilidad de características](/docs/es/feature-availability) para una comparación completa de todos los métodos de implementación.
</Note>

Claude apps gateway es un servicio autohospedado que se sitúa entre los clientes de Claude Code de sus desarrolladores y su proveedor de modelos. Los desarrolladores inician sesión con su proveedor de identidad corporativo (IdP) en lugar de mantener claves API o credenciales de nube. La puerta de enlace mantiene la credencial ascendente, aplica el acceso a modelos y [configuraciones administradas](/docs/es/permissions#managed-settings) por grupo de IdP, y retransmite la telemetría de uso a su propia pila de observabilidad.

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
* **Control de acceso**: sus grupos de IdP se asignan a listas de permitidos de modelos y políticas de [configuración administrada](/docs/es/permissions#managed-settings). La puerta de enlace aplica el acceso a modelos del lado del servidor, rechazando solicitudes de modelos no otorgados, y selecciona la política de configuración administrada de cada grupo, que la CLI aplica en el [nivel de configuración administrada](/docs/es/settings#settings-precedence). Diferentes equipos obtienen diferentes modelos, herramientas y permisos, y un desarrollador no puede anular lo que su política bloquea.
* **Entrega de configuración**: la puerta de enlace entrega la configuración administrada a los clientes conectados por sí misma, reemplazando la [configuración administrada por servidor](/docs/es/server-managed-settings) de la consola de administrador de claude.ai.
* **Telemetría**: cada destino configurado, como Datadog, Splunk o ClickHouse, recibe [métricas del Protocolo OpenTelemetry (OTLP)](/docs/es/monitoring-usage) con recuentos de tokens, modelo, identidad del usuario y latencia por defecto, con registros y trazas como activaciones opcionales por destino.
* **Enrutamiento ascendente**: los clientes hablan la API de Mensajes de Anthropic a la puerta de enlace, y la puerta de enlace traduce para cada ascendente, ya sea Amazon Bedrock, [Plataforma Claude en AWS](/docs/es/claude-platform-on-aws), Plataforma de Agentes de Google Cloud, Microsoft Foundry, o la API de Anthropic, con conmutación por error entre ellos. Puede cambiar regiones, proveedores u orden de conmutación por error sin que los desarrolladores lo noten o reconfiguren.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Diagrama que muestra clientes de Claude Code conectándose sobre HTTPS con tokens portadores a una puerta de enlace de aplicaciones Claude autohospedada dentro de su infraestructura, que inicia sesión de usuarios contra su IdP, almacena estado de autenticación en PostgreSQL, retransmite telemetría a su recopilador OTLP, y reenvía inferencia a Amazon Bedrock, Plataforma Claude en AWS, Google Cloud, Microsoft Foundry o la API de Anthropic" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  El plano de datos de la puerta de enlace no envía nada a la infraestructura de Anthropic a menos que la API de Anthropic sea un ascendente configurado. Usted controla dónde van la telemetría, los registros de auditoría, la configuración administrada y la identidad de IdP de sus desarrolladores, y la puerta de enlace no envía ninguno de ellos a Anthropic. Para el tráfico restante que el proceso de CLI puede enviar y cómo cerrarlo, consulte [Postura de cumplimiento](/docs/es/claude-apps-gateway-deploy#compliance-posture).
</Note>

Para ver qué características de Claude Code funcionan a través de la puerta de enlace y qué soporta el servidor en sí, consulte [Disponibilidad y limitaciones](#availability-and-limitations) a continuación. Para decisiones como costo, derivación, ejecutar múltiples puertas de enlace y plataformas sin servidor, consulte la [guía de implementación](/docs/es/claude-apps-gateway-deploy#deployment).

<h3 id="other-gateway-implementations">
  Otras implementaciones de puerta de enlace
</h3>

Si ya ejecuta una puerta de enlace LLM o puerta de enlace API que cumple con sus necesidades, continúe usándola; [Otras puertas de enlace LLM](/docs/es/llm-gateway) cubre la configuración de Claude Code contra ella.

La [referencia del protocolo de puerta de enlace](/docs/es/llm-gateway-protocol) documenta el contrato que Claude Code espera de cualquier puerta de enlace: los puntos finales que llama, los encabezados y campos de cuerpo a reenviar, y qué deja de funcionar cuando se eliminan. Una puerta de enlace de aplicaciones Claude en ejecución sirve un superconjunto de ese contrato en `GET /protocol`, agregando los puntos finales específicos de la puerta de enlace de aplicaciones Claude para inicio de sesión SSO, entrega de configuración administrada y telemetría. Obténgalo con `curl https://claude-gateway.internal.example.com/protocol` desde cualquier puerta de enlace implementada, como la que produce el [inicio rápido](#quickstart) a continuación.

Los cambios importantes en el protocolo se anuncian con anticipación, pero no se garantiza compatibilidad hacia atrás indefinida.

<h2 id="quickstart">
  Inicio rápido
</h2>

Este inicio rápido recorre la ruta mínima: registre un cliente OAuth en su IdP, escriba un `gateway.yaml`, ejecute la puerta de enlace junto con Postgres con Docker Compose, y verifique el inicio de sesión de extremo a extremo. Utiliza un ascendente de Amazon Bedrock; la Plataforma de Claude en AWS, la Plataforma de Agentes de Google Cloud, Microsoft Foundry, y la API de Anthropic son igualmente compatibles intercambiando el bloque `upstreams` como se muestra en la [referencia de configuración](/docs/es/claude-apps-gateway-config#upstreams). Al final tiene una puerta de enlace a la que un desarrollador puede `/login`.

<Note>
  **Implemente en su red privada.** Claude Code solo se conecta a una puerta de enlace cuya dirección es privada. Esta es una protección de seguridad, porque una puerta de enlace confiable puede insertar configuración que ejecute comandos en máquinas de desarrolladores. Coloque la puerta de enlace detrás de un equilibrador de carga interno o VPN y asígnele un nombre de host que se resuelva solo a direcciones IP privadas.

  Los puntos finales de puerta de enlace pública operados por Anthropic son la excepción: `/login` los acepta sobre `https://`. Estos son un conjunto pequeño y fijo de puertas de enlace que el propio Anthropic opera; no son una opción de implementación que pueda seleccionar o configurar. La lista se compila en Claude Code, por lo que ninguna configuración puede agregar un nombre de host a ella y ninguna puerta de enlace que aloje califica para la exención. Antes de v2.1.206, `/login` rechazaba esos puntos finales como cualquier otra dirección pública.
</Note>

<h3 id="prerequisites">
  Requisitos previos
</h3>

Tenga estos en lugar antes de comenzar:

| Lo que necesita                              | Detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 o posterior             | El subcomando `claude gateway` y el flujo de inicio de sesión de la puerta de enlace se envían en v2.1.195. Las compilaciones públicas anteriores no las incluyen. Tanto la máquina que ejecuta el servidor de puerta de enlace como la máquina de cada desarrollador deben estar en v2.1.195 o posterior; ejecute `claude update` para obtener la última versión. La [Plataforma de Claude en AWS ascendente](/docs/es/claude-apps-gateway-config#claude-platform-on-aws) requiere Claude Code v2.1.198 o posterior en el servidor de puerta de enlace.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Proveedor de identidad OpenID Connect (OIDC) | Okta, Microsoft Entra ID, Google Workspace, Keycloak, o Dex, u otro IdP compatible con OIDC como PingFederate. La puerta de enlace ejecuta el descubrimiento OIDC estándar y el flujo de código de autorización contra ella. SAML y LDAP no son compatibles.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| PostgreSQL 14 o posterior                    | Respalda el flujo de inicio de sesión del dispositivo, donde la devolución de llamada del navegador escribe y la CLI de sondeo lee, más contadores de límite de velocidad. Cualquier Postgres administrado funciona, incluido el nivel más pequeño. Sin límites de gasto configurados, la puerta de enlace almacena algunos KB de estado de autenticación de corta duración; con [límites de gasto](/docs/es/claude-apps-gateway-spend-limits), también mantiene tablas de gasto, auditoría e identidad duraderas que deben respaldarse. TLS a través de `?sslmode=require` se recomienda.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Ascendente de modelo                         | Credenciales de Amazon Bedrock, credenciales de Plataforma de Claude en AWS, credenciales de Google Cloud, un recurso de Microsoft Foundry, o una clave API de Anthropic. Se admiten múltiples ascendentes con conmutación por error.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| HTTPS                                        | La puerta de enlace debe ser accesible sobre `https://` desde computadoras portátiles de desarrolladores y desde cualquier navegador utilizado para el inicio de sesión; la puerta de enlace sirve la página de verificación del dispositivo en el mismo oyente. Proporcione un certificado TLS a través de `listen.tls`, o ejecute detrás de una entrada que termina TLS y establezca `listen.public_url`. Un origen `http://` simple se acepta solo en loopback, para desarrollo local.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Dirección de red privada                     | En `/login`, Claude Code requiere que el nombre de host o la dirección IP de la puerta de enlace se resuelvan solo a direcciones privadas: RFC 1918, CGNAT `100.64.0.0/10`, ULA IPv6 `fc00::/7`, o loopback para desarrollo local. La verificación se ejecuta en cada IP resuelta, por lo que si alguna dirección a la que se resuelve el nombre es pública, `/login` rechaza la URL. Si las máquinas de desarrolladores enrutan HTTPS a través de un proxy corporativo, el inicio de sesión también requiere que el host del proxy se resuelva a direcciones privadas; si no es así, agregue el host de la puerta de enlace a `NO_PROXY` para que la CLI se conecte directamente. Los puntos finales de puerta de enlace operados por Anthropic son exentos de las verificaciones de dirección privada y proxy: `/login` los acepta sobre `https://` por coincidencia exacta de nombre de host, por lo que el requisito de red privada se aplica solo a una puerta de enlace que aloje usted mismo. Antes de v2.1.206, `/login` rechazaba un punto final operado por Anthropic como cualquier otra dirección pública. |
| Tiempo de ejecución de Linux                 | El servidor de puerta de enlace solo se ejecuta en el binario nativo de Linux. macOS funciona para desarrollo local. Windows no es compatible como plataforma de servidor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

El servidor de puerta de enlace requiere el binario `claude` nativo; descargue una versión fija como se describe en [Instalar Claude Code](/docs/es/setup). El servidor utiliza características de tiempo de ejecución que no están disponibles cuando Claude Code se ejecuta bajo Node. Si ve `requires the native binary` al arrancar, cambie a uno de los métodos de instalación independientes.

<h3 id="steps">
  Pasos
</h3>

<Steps>
  <Step title="Registre un cliente OAuth en su IdP">
    Decida primero el nombre de host de la puerta de enlace, porque el URI de redirección debe coincidir con él. Cree una nueva aplicación web OIDC y establezca el URI de redirección en `https://claude-gateway.<your-domain>/oauth/callback`, donde el host es el mismo valor que establece como [`listen.public_url`](/docs/es/claude-apps-gateway-config#listen) en el paso 3. Anote el `client_id` y `client_secret`. Las instrucciones por IdP están en [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup).
  </Step>

  <Step title="Aprovisione una base de datos PostgreSQL">
    Cualquier Postgres 14 o posterior funciona, incluido el nivel administrado más pequeño. La puerta de enlace ejecuta sus propias migraciones de esquema al arrancar, por lo que el usuario de la base de datos necesita permiso `CREATE TABLE`. Si su política de seguridad prohíbe DDL de roles de aplicación, pre-cree el esquema en su lugar; consulte [`store`](/docs/es/claude-apps-gateway-config#store).
  </Step>

  <Step title="Escriba gateway.yaml">
    Los secretos se leen a través de la expansión `${ENV_VAR}` para que el archivo en sí pueda vivir en control de versiones. Use un nombre de host `public_url` que se resuelva a una IP privada en su red, porque `/login` rechaza direcciones públicas. La configuración mínima tiene cinco secciones, y todos los demás campos tienen un valor predeterminado:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # Requerido detrás de cualquier proxy que termina TLS. Utilizado para el IdP
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

    Esta configuración es suficiente para un bucle de inicio de sesión funcional con el catálogo de modelos predeterminado de Bedrock. Una vez que se ejecute, agregue RBAC por grupo y configuración administrada a través de [`managed.policies`](/docs/es/claude-apps-gateway-config#managed), distribución de telemetría a través de [`telemetry`](/docs/es/claude-apps-gateway-config#telemetry), y conmutación por error de múltiples ascendentes, ARNs de rendimiento aprovisionado, o regiones no estadounidenses a través de [`models`](/docs/es/claude-apps-gateway-config#models).

    <Note>
      El ascendente de Bedrock necesita un principal de AWS con `bedrock:InvokeModel` y `bedrock:InvokeModelWithResponseStream` en los ARNs `inference-profile/us.anthropic.*` y los ARNs `foundation-model/anthropic.*` subyacentes, y acceso a modelos habilitado en la consola de Bedrock para los modelos Claude que desea. Suministre la credencial con IRSA en EKS, un rol de tarea de ECS, o un perfil de instancia de EC2 en lugar de claves estáticas. La [referencia `upstreams`](/docs/es/claude-apps-gateway-config#upstreams) tiene los detalles completos de IAM, la matriz de credenciales entre nubes, y los bloques `auth` para los otros proveedores.
    </Note>
  </Step>

  <Step title="Ejecútelo">
    Construya una imagen de contenedor alrededor del binario `claude` que cumpla con los [requisitos de imagen](/docs/es/claude-apps-gateway-deploy#container-image), luego ejecútela junto con Postgres:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <your-registry>/claude-gateway:<version>
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

    La puerta de enlace es un único binario de Linux que lee la configuración, ejecuta el descubrimiento OIDC contra su IdP, aplica sus migraciones de esquema de Postgres, construye clientes ascendentes, e inicia la escucha. El arranque es de cierre fallido para la configuración, la conexión de Postgres con un tiempo de espera de 5 segundos, el descubrimiento OIDC, y la construcción del cliente ascendente. Si alguno de esos es inaccesible o está mal configurado, la puerta de enlace sale con un error en lugar de servir tráfico en un estado degradado.

    Un arranque exitoso no valida la ruta de inferencia, porque las credenciales de instancia de Bedrock y Agent Platform se resuelven en la primera solicitud, no al arrancar.

    Observe stderr para la secuencia de arranque. Las líneas de registro utilizan el formato `[gateway] <timestamp> <level> <message>`, los eventos de auditoría son JSON de una sola línea con un campo `evt`, y un banner de inicio, omitido a continuación, se imprime entre las líneas de migración y escucha. Debería ver, en orden:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

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
    Este último paso ocurre en una máquina de desarrollador, no en el servidor. Establezca `forceLoginMethod` en `"gateway"` y `forceLoginGatewayUrl` en la `public_url` de su puerta de enlace en el [archivo de configuración administrada](/docs/es/settings#settings-files) de esa máquina, luego ejecute `/login`, presione Intro en la pantalla **Cloud gateway**, y complete el inicio de sesión del navegador. [Establezca la URL de la puerta de enlace](#set-the-gateway-url) a continuación cubre la distribución de ambas claves a escala.
  </Step>
</Steps>

<h2 id="connect-developers">
  Conectar desarrolladores
</h2>

Los desarrolladores se conectan desde sus propias computadoras portátiles con un inicio de sesión de navegador, usando su cuenta de trabajo corporativa. No necesitan una cuenta de claude.ai, una clave API, o una suscripción, porque las solicitudes al modelo van a través de la puerta de enlace usando la credencial ascendente de la organización. La conexión es impulsada por la [configuración administrada del lado del cliente](/docs/es/claude-apps-gateway-config#client-side-managed-settings) que inserta a través de MDM, por lo que no hay configuración manual en el lado del desarrollador; esta sección cubre lo que configura el administrador.

La CLI toma la huella digital del certificado TLS de hoja de la puerta de enlace en la primera conexión y la fija por nombre de host. Publique la huella digital SHA-256 esperada junto con la URL de la puerta de enlace para que los desarrolladores tengan algo con lo que comparar. Obtenga la huella digital del archivo de certificado con `openssl x509 -noout -fingerprint -sha256 -in cert.pem`; el indicador `/login` muestra los primeros 16 caracteres del resumen como hexadecimal minúscula sin separadores.

Cuando el certificado rota, cada desarrollador ve el indicador de confianza nuevamente, por lo que trate las rotaciones como un evento planificado y republique la huella digital.

Una vez conectado, el [selector de modelos](/docs/es/model-config) muestra los modelos en la lista de permitidos `availableModels` del desarrollador, la configuración administrada se aplica al inicio y se actualiza cada hora, y la telemetría se enruta a su recopilador. Las sesiones se actualizan silenciosamente antes de la expiración de `ttl_hours`, y una actualización fallida después del desaprovisionamiento de IdP solicita un nuevo inicio de sesión.

<h3 id="set-the-gateway-url">
  Establecer la URL de la puerta de enlace
</h3>

Establezca ambas claves en el archivo de [configuración administrada](/docs/es/settings#settings-files) por sistema operativo que implementa a través de MDM o directamente en el disco, y `/login` abre directamente en la pantalla **Cloud gateway** con la URL rellenada:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

El desarrollador presiona Intro para conectarse. El indicador de huella digital TLS de primera conexión aún aparece.

No hay opción de puerta de enlace en el selector de inicio de sesión para que un desarrollador seleccione manualmente, y `forceLoginGatewayUrl` se ignora en los archivos de configuración propios de un desarrollador. `forceLoginMethod` solo, sin una URL, deja al desarrollador en un mensaje "Contacte a su administrador de TI". Ambas claves pertenecen al archivo que inserta en máquinas, no en el bloque `managed.policies[].cli` de la puerta de enlace, que solo llega a clientes que ya están conectados.

<h3 id="ci-pipelines-and-remote-machines">
  Canalizaciones de CI y máquinas remotas
</h3>

No hay flujo de token de servicio para canalizaciones desatendidas. El inicio de sesión de la puerta de enlace siempre ejecuta el flujo de dispositivo del navegador, por lo que un trabajo de CI sin un desarrollador para aprobar el inicio de sesión no puede autenticarse; configure esos contra su proveedor directamente.

Una vez que un desarrollador ha iniciado sesión, cada invocación de Claude Code en esa máquina usa la sesión de la puerta de enlace, incluidas las ejecuciones no interactivas de `claude -p` y sesiones iniciadas por el SDK del Agente, y la [política de la puerta de enlace se aplica a todas ellas](/docs/es/claude-apps-gateway-config#managed).

El flujo de dispositivo separa la CLI de sondeo de la aprobación del navegador, por lo que una caja de desarrollo remota sin pantalla aún funciona: el desarrollador ejecuta `/login` sobre SSH en la máquina remota y abre el enlace de verificación en el navegador en su computadora portátil.

<h3 id="what’s-enforced-on-developers">
  Qué se aplica en los desarrolladores
</h3>

Estas garantías se aplican a cada sesión de puerta de enlace conectada.

* **Acceso a modelos**: las solicitudes de modelos que la política no otorga devuelven 400, y el selector `/model` se filtra a la lista de permitidos `availableModels` de la política. Establezca [`enforceAvailableModels: true`](/docs/es/model-config#default-model-behavior) en la política para que la opción Predeterminado se resuelva a un modelo dentro de `availableModels` en lugar de al predeterminado integrado de Claude Code; sin él, Predeterminado permanece seleccionable y se rechaza en el tiempo de solicitud si ese modelo no se otorga.
* **Destino de telemetría**: cuando se configura [reenvío de telemetría](/docs/es/claude-apps-gateway-config#telemetry), el punto final de exportación OTLP se fija a la puerta de enlace, y la configuración insertada por la puerta de enlace anula las variables `OTEL_*` configuradas localmente.
* **Credenciales**: el token de la puerta de enlace es la única credencial de la sesión. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper`, y cualquier inicio de sesión anterior de claude.ai se ignoran mientras se conecta, por lo que los desarrolladores no necesitan cerrar sesión de claude.ai primero.
* **Configuración administrada**: las claves bloqueadas no se pueden anular localmente. La CLI aplica la política al inicio y en cada sondeo cada hora.
* **Inicio**: las sesiones conectadas salen al inicio con un error después de aproximadamente 10 segundos cuando la puerta de enlace es inaccesible, en lugar de iniciarse sin su configuración.
* **Desaprovisionamiento**: una sesión cuyo usuario está deshabilitado en el IdP expira dentro de `ttl_hours` cuando la siguiente actualización falla.

<h3 id="what-the-organization-can-see">
  Qué puede ver la organización
</h3>

La telemetría de uso lleva la identidad del desarrollador, recuentos de tokens, modelo y latencia al recopilador de la organización. La puerta de enlace no registra ni almacena contenido de indicación o finalización. Si se recopila telemetría más rica como registros y trazas, que puede incluir comandos y rutas de archivo, es la [opción por destino](/docs/es/claude-apps-gateway-config#telemetry) de la organización.

<h2 id="availability-and-limitations">
  Disponibilidad y limitaciones
</h2>

La tabla cubre qué características de Claude Code funcionan cuando los desarrolladores se conectan a través de la puerta de enlace, y qué soporta el servidor de puerta de enlace en sí. Donde algo no es compatible, la columna Notas proporciona la alternativa.

La puerta de enlace entrega los valores [`anthropic-beta`](https://platform.claude.com/docs/es/api/beta-headers) que la CLI envía a cada ascendente, por lo que los operadores no mantienen una lista de permitidos de beta. Para Amazon Bedrock, que ignora el encabezado, la puerta de enlace mueve los valores al campo `anthropic_beta` del cuerpo de la solicitud; los otros ascendentes reciben el encabezado como se envía.

El conjunto de beta de sesión de puerta de enlace de la CLI omite betas solo de primera parte y la beta de ttl de caché extendido, por lo que esas filas a continuación muestran como no disponibles.

| Característica                                                                                                               | Estado        | Notas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------------------------------------------------------------------------------------------------------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reenvío de inferencia (Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud, Microsoft Foundry, Anthropic) | Disponible    | Con traducción de modelos por ascendente y conmutación por error. El ascendente de Amazon Bedrock utiliza el punto final `bedrock-runtime` y la cadena de credenciales predeterminada de AWS; el [punto final Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint) de Amazon Bedrock no es un ascendente compatible. El [ascendente Claude Platform en AWS](/docs/es/claude-apps-gateway-config#claude-platform-on-aws) requiere Claude Code v2.1.198 o posterior en el servidor de puerta de enlace. |
| Acceso a modelos y configuración administrada por grupo de IdP                                                               | Disponible    | El acceso a modelos se aplica del lado del servidor; la configuración administrada se entrega por grupo de IdP y se aplica por la CLI en el [nivel de configuración administrada](/docs/es/settings#settings-precedence)                                                                                                                                                                                                                                                                             |
| Distribución de telemetría (OTLP/HTTP)                                                                                       | Disponible    | Identidad marcada por exportación; ambas codificaciones protobuf y JSON                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Proveedores de identidad OIDC                                                                                                | Disponible    | Cualquier IdP compatible con OIDC; la puerta de enlace ejecuta el descubrimiento OIDC estándar y el flujo de código de autorización. Consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para la configuración por IdP                                                                                                                                                                                                                  |
| Límites de gasto por usuario y por grupo                                                                                     | Disponible    | Consulte [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Búsqueda web del lado del servidor                                                                                           | No disponible | La CLI no puede ver qué proveedor ascendente enruta la puerta de enlace, por lo que no puede verificar la compatibilidad de búsqueda web y deshabilita WebSearch en sesiones de puerta de enlace                                                                                                                                                                                                                                                                                                |
| Almacenamiento en caché de indicación estándar                                                                               | Disponible    | Los puntos de interrupción `cache_control` se reenvían a cada ascendente                                                                                                                                                                                                                                                                                                                                                                                                                        |
| TTL de caché de 1 hora                                                                                                       | No disponible | La CLI omite la beta de ttl de caché extendido en sesiones de puerta de enlace, porque no todos los ascendentes a los que la puerta de enlace puede enrutar soportan el TTL de 1 hora, por lo que el almacenamiento en caché de indicación a través de la puerta de enlace utiliza el TTL de 5 minutos; consulte la nota de encabezado de beta anterior                                                                                                                                         |
| Modo automático                                                                                                              | Disponible    | Sigue las [reglas del proveedor de terceros](/docs/es/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry): solo los modelos elegibles en proveedores de terceros pueden usarlo. Antes de v2.1.207, el modo automático en sesiones de puerta de enlace requería establecer `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, entregable a través del bloque `env` de política administrada                                                                                                        |
| Optimizaciones solo de primera parte como alcance de caché global y herramientas eficientes en tokens                        | No disponible | La CLI no las habilita en sesiones de puerta de enlace; consulte la nota de encabezado de beta anterior                                                                                                                                                                                                                                                                                                                                                                                         |
| OTLP/gRPC                                                                                                                    | No compatible | OTLP sobre HTTP solo                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| SAML, LDAP y otra autenticación no OIDC                                                                                      | No compatible | Solo OIDC. Frente con un puente OIDC si es necesario                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Multi-inquilino (múltiples emisores OIDC)                                                                                    | No compatible | Un emisor por puerta de enlace. Ejecute instancias separadas                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Servidor Windows                                                                                                             | No compatible | Implemente en Linux. macOS solo para desarrollo local                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Gráfico Helm                                                                                                                 | No disponible | La puerta de enlace se ejecuta como un Deployment sin estado estándar; consulte la [guía de implementación](/docs/es/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                                                                                                                                                          |
| Interfaz de usuario de administrador                                                                                         | No disponible | La configuración es el archivo YAML; reimplemente para cambiarlo                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h2 id="next-steps">
  Próximos pasos
</h2>

El inicio rápido lo deja con una configuración mínima ejecutándose bajo Docker Compose. Para llevarlo más lejos:

* Expanda `gateway.yaml` más allá de la configuración mínima, por ejemplo para agregar RBAC por grupo, conmutación por error de múltiples ascendentes, o destinos de telemetría. La [referencia de configuración](/docs/es/claude-apps-gateway-config) cubre cada opción.
* Pase de Compose a una implementación de producción en Kubernetes o Cloud Run, configure su IdP correctamente, y revise el modelo de seguridad. La [guía de implementación y operaciones](/docs/es/claude-apps-gateway-deploy) cubre la configuración por IdP, los requisitos de imagen de contenedor, sondeos de salud y solución de problemas.
* Coloque límites de gasto en desarrolladores individuales o grupos para que una carga de trabajo descontrolada no pueda consumir todo su compromiso. [Límites de gasto](/docs/es/claude-apps-gateway-spend-limits) cubre la API de administrador y cómo funciona la aplicación.
* Para un ejemplo completo trabajado en Google Cloud, con Cloud Run, Cloud SQL y Secret Manager, consulte [Implementar en Google Cloud](/docs/es/claude-apps-gateway-on-gcp).
