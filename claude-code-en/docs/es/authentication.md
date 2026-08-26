> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autenticación

> Inicie sesión en Claude Code y configure la autenticación para individuos, equipos y organizaciones.

Claude Code admite múltiples métodos de autenticación según su configuración. Los usuarios individuales pueden iniciar sesión con una cuenta de Claude.ai, mientras que los equipos pueden usar Claude for Teams o Enterprise, la Claude Console, o un proveedor de nube como Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Inicie sesión en Claude Code
</h2>

Después de [instalar Claude Code](/docs/es/setup#install-claude-code), ejecute `claude` en su terminal. En el primer lanzamiento, Claude Code abre una ventana del navegador para que inicie sesión.

Si el navegador no se abre automáticamente, presione `c` para copiar la URL de inicio de sesión al portapapeles y luego péguelo en su navegador.

Si su navegador muestra un código de inicio de sesión en lugar de redirigirse después de que inicie sesión, péguelo en el terminal en el símbolo del sistema `Paste code here if prompted`. Esto sucede cuando el navegador no puede alcanzar el servidor de devolución de llamada local de Claude Code, lo cual es común en WSL2, sesiones SSH y contenedores.

Cuando el inicio de sesión se completa, el terminal muestra `Login successful` y le solicita que presione `Enter` para continuar.

Puede autenticarse con cualquiera de estos tipos de cuenta:

* **Suscripción Claude Pro o Max**: inicie sesión con su cuenta de Claude.ai. Suscríbase en [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams o Enterprise**: inicie sesión con la cuenta de Claude.ai que su administrador de equipo le invitó a usar.
* **Claude Console**: inicie sesión con sus credenciales de Console. Su administrador debe haberle [invitado](#claude-console-authentication) primero.
* **Proveedores de nube**: si su organización usa [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry), establezca las variables de entorno requeridas antes de ejecutar `claude`, o seleccione **plataforma de terceros** en el símbolo del sistema de inicio de sesión, que inicia un asistente de configuración interactivo para Bedrock y Vertex AI. No se necesita inicio de sesión en el navegador.
* **Puerta de enlace en la nube**: si su organización ejecuta una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada, inicie sesión con SSO corporativo a través de `/login`. El token emitido por la puerta de enlace es la única credencial de la sesión.

Los administradores pueden restringir el inicio de sesión interactivo con la configuración administrada [`forceLoginMethod` y `forceLoginOrgUUID`](/docs/es/settings#available-settings). Cuando se establece cualquiera de ellas, las sesiones autenticadas por `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` se bloquean al inicio; las sesiones de proveedores de nube no se ven afectadas.

Para cerrar sesión y volver a autenticarse, escriba `/logout` en el símbolo del sistema de Claude Code. Cerrar sesión también restablece su estado de configuración de primer lanzamiento, por lo que la próxima vez que ejecute `claude` le guiará a través del inicio de sesión y la configuración nuevamente.

Si tiene problemas para iniciar sesión, consulte [solución de problemas de autenticación](/docs/es/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Configure la autenticación del equipo
</h2>

Para equipos y organizaciones, puede configurar el acceso a Claude Code de una de estas formas:

* [Claude for Teams o Enterprise](#claude-for-teams-or-enterprise), recomendado para la mayoría de los equipos
* [Claude Console](#claude-console-authentication)
* [Puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), una puerta de enlace autohospedada que inicia sesión a los desarrolladores con su IdP y enruta la inferencia al proveedor de nube que configure
* [Amazon Bedrock](/docs/es/amazon-bedrock)
* [Plataforma de agentes de Google Cloud](/docs/es/google-vertex-ai)
* [Microsoft Foundry](/docs/es/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams o Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) y [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) proporcionan la mejor experiencia para organizaciones que usan Claude Code. Los miembros del equipo obtienen acceso tanto a Claude Code como a Claude en la web con facturación centralizada y gestión de equipos.

* **Claude for Teams**: plan de autoservicio con características de colaboración, herramientas de administración y gestión de facturación. Mejor para equipos más pequeños.
* **Claude for Enterprise**: añade SSO, captura de dominio, permisos basados en roles, API de cumplimiento y configuración de políticas administradas para configuraciones de Claude Code en toda la organización. Mejor para organizaciones más grandes con requisitos de seguridad y cumplimiento.

<Steps>
  <Step title="Suscribirse">
    Suscríbase a [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) o póngase en contacto con ventas para [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Invitar a miembros del equipo">
    Invite a miembros del equipo desde el panel de administración.
  </Step>

  <Step title="Instalar e iniciar sesión">
    Los miembros del equipo instalan Claude Code e inician sesión con sus cuentas de Claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Autenticación de Claude Console
</h3>

Para organizaciones que prefieren facturación basada en API, puede configurar el acceso a través de Claude Console.

<Steps>
  <Step title="Crear o usar una cuenta de Console">
    Use su cuenta de Claude Console existente o cree una nueva.
  </Step>

  <Step title="Agregar usuarios">
    Puede agregar usuarios mediante cualquiera de estos métodos:

    * Invitar usuarios en masa desde dentro de Console: Settings -> Members -> Invite
    * [Configurar SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Asignar roles">
    Al invitar usuarios, asigne uno de:

    * **Rol Claude Code**: los usuarios solo pueden crear claves API de Claude Code
    * **Rol Developer**: los usuarios pueden crear cualquier tipo de clave API
  </Step>

  <Step title="Los usuarios completan la configuración">
    Cada usuario invitado necesita:

    * Aceptar la invitación de Console
    * [Verificar requisitos del sistema](/docs/es/setup#system-requirements)
    * [Instalar Claude Code](/docs/es/setup#install-claude-code)
    * Iniciar sesión con credenciales de cuenta de Console
  </Step>
</Steps>

<h3 id="cloud-provider-authentication">
  Autenticación del proveedor de nube
</h3>

Para equipos que usan Amazon Bedrock, Plataforma de agentes de Google Cloud o Microsoft Foundry:

<Steps>
  <Step title="Seguir la configuración del proveedor">
    Siga la [documentación de Amazon Bedrock](/docs/es/amazon-bedrock), [documentación de Plataforma de agentes de Google Cloud](/docs/es/google-vertex-ai) o [documentación de Microsoft Foundry](/docs/es/microsoft-foundry).
  </Step>

  <Step title="Distribuir configuración">
    Distribuya las variables de entorno e instrucciones para generar credenciales de nube a sus usuarios. Lea más sobre cómo [administrar la configuración aquí](/docs/es/settings).
  </Step>

  <Step title="Instalar Claude Code">
    Los usuarios pueden [instalar Claude Code](/docs/es/setup#install-claude-code).
  </Step>
</Steps>

<h2 id="credential-management">
  Gestión de credenciales
</h2>

Claude Code administra de forma segura sus credenciales de autenticación:

* **Ubicación de almacenamiento**:
  * En macOS, las credenciales se almacenan en el Keychain de macOS cifrado.
  * En Linux, las credenciales se almacenan en `~/.claude/.credentials.json` con modo de archivo `0600`.
  * En Windows, las credenciales se almacenan en `%USERPROFILE%\.claude\.credentials.json` y heredan los controles de acceso del directorio de su perfil de usuario, lo que restringe el archivo a su cuenta de usuario de forma predeterminada.
  * Si ha establecido la variable de entorno `CLAUDE_CONFIG_DIR` en Linux o Windows, el archivo `.credentials.json` se encuentra en ese directorio en su lugar.
  * Claude Code administra `.credentials.json` a través de `/login` y `/logout`. Para enrutar solicitudes a través de un punto final de API personalizado, establezca la variable de entorno [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) en su lugar.
* **Tipos de autenticación admitidos**: credenciales de Claude.ai, credenciales de API de Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth y tokens de sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway).
* **Scripts de credenciales personalizados**: la configuración [`apiKeyHelper`](/docs/es/settings#available-settings) se puede configurar para ejecutar un script de shell que devuelva una clave API.
* **Intervalos de actualización**: por defecto, `apiKeyHelper` se llama después de 5 minutos o en respuesta HTTP 401. Establezca la variable de entorno `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` para intervalos de actualización personalizados.
* **Aviso de helper lento**: si `apiKeyHelper` tarda más de 10 segundos en devolver una clave, Claude Code muestra un aviso de advertencia en la barra de símbolo del sistema mostrando el tiempo transcurrido. Si ve este aviso regularmente, verifique si su script de credenciales se puede optimizar.
* **Fallos del helper**: cuando el script sale con un error, agota el tiempo de espera o no imprime nada, las solicitudes fallan con [`Your apiKeyHelper script is failing`](/docs/es/errors#your-apikeyhelper-script-is-failing) dentro de tres intentos. Antes de v2.1.208, los fallos del helper aparecían como un 401 genérico después de aproximadamente diez reintentos silenciosos.

`apiKeyHelper`, `ANTHROPIC_API_KEY` y `ANTHROPIC_AUTH_TOKEN` se aplican a la CLI y a las superficies que la envuelven, incluida la extensión de VS Code, el Agent SDK y GitHub Actions. Claude Desktop y las sesiones en la nube no llaman a `apiKeyHelper` ni leen estas variables de entorno: utilizan OAuth, excepto las sesiones de escritorio que ejecutan una [configuración de inferencia de terceros](/docs/es/llm-gateway-connect#desktop-app), que se autentican con la credencial de esa configuración.

<h3 id="renew-an-expiring-login">
  Renovar un inicio de sesión que está por expirar
</h3>

Cuando el inicio de sesión que creó con `/login` está a menos de cinco días de expirar, Claude Code muestra una advertencia al inicio: `Your login expires in 3 days · run /login to renew`. Requiere Claude Code v2.1.203 o posterior.

Ejecute `/login` para renovar. La advertencia es informativa y nunca bloquea una solicitud: la autenticación sigue funcionando hasta que el inicio de sesión realmente expire. La duración del inicio de sesión en sí no cambia; la advertencia anticipada es lo que v2.1.203 añade.

Una vez que el inicio de sesión almacenado expira y no se puede actualizar, cada solicitud falla con [`Login expired · Please run /login`](/docs/es/errors#login-expired) hasta que inicie sesión nuevamente. Antes de v2.1.206, un inicio de sesión expirado aparecía como un error de modelo en su lugar.

La advertencia aparece solo cuando un inicio de sesión de claude.ai o Claude Console es la credencial activa, y no cuando un proveedor de nube, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` proporciona la credencial.

Renovar anticipadamente es más importante para sesiones que se ejecutan sin supervisión. Una [sesión en segundo plano en vista de agente](/docs/es/agent-view) o una sesión de [Remote Control](/docs/es/remote-control) que sobrevive al inicio de sesión deja de hacer progreso una vez que la credencial expira y no puede recuperarse hasta que inicie sesión nuevamente.

<h3 id="authentication-precedence">
  Precedencia de autenticación
</h3>

Cuando hay múltiples credenciales presentes, Claude Code elige una en este orden:

1. Credenciales del proveedor de nube, cuando `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` o `CLAUDE_CODE_USE_FOUNDRY` está establecido. Consulte [integraciones de terceros](/docs/es/third-party-integrations) para la configuración.
2. Variable de entorno `ANTHROPIC_AUTH_TOKEN`. Se envía como encabezado `Authorization: Bearer`. Use esto cuando enrute a través de una [puerta de enlace LLM o proxy](/docs/es/llm-gateway) que se autentica con tokens de portador en lugar de claves API de Anthropic.
3. Variable de entorno `ANTHROPIC_API_KEY`. Se envía como encabezado `X-Api-Key`. Use esto para acceso directo a la API de Anthropic con una clave de [Claude Console](https://platform.claude.com). En modo interactivo, se le solicita una vez que apruebe o rechace la clave, y su elección se recuerda. Para cambiarla más tarde, use el botón de alternancia "Use custom API key" en `/config`. El botón de alternancia solo aparece mientras `ANTHROPIC_API_KEY` está establecido en su entorno. En modo no interactivo (`-p`), la clave siempre se usa cuando está presente.
4. Salida del script [`apiKeyHelper`](/docs/es/settings#available-settings). Use esto para credenciales dinámicas o rotativas, como tokens de corta duración obtenidos de un almacén.
5. Variable de entorno `CLAUDE_CODE_OAUTH_TOKEN`. Un token OAuth de larga duración generado por [`claude setup-token`](#generate-a-long-lived-token). Use esto para canalizaciones de CI y scripts donde el inicio de sesión del navegador no está disponible.
6. Credenciales OAuth de suscripción de `/login`. Este es el predeterminado para usuarios de Claude Pro, Max, Team y Enterprise.

Una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada se encuentra fuera de esta lista: es una selección de proveedor como Amazon Bedrock o la Plataforma de Agentes de Google Cloud, y los supera. Cuando existe una sesión de puerta de enlace, la CLI se autentica con el token de puerta de enlace incluso si `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` o `CLAUDE_CODE_USE_FOUNDRY` está establecido, y las entradas de token de portador, clave API y `apiKeyHelper` anteriores no se utilizan.

Si tiene una suscripción activa de Claude pero también tiene `ANTHROPIC_API_KEY` establecido en su entorno, la clave API tiene precedencia una vez aprobada. Esto puede causar fallos de autenticación si la clave pertenece a una organización deshabilitada o expirada. Ejecute `unset ANTHROPIC_API_KEY` para volver a su suscripción y verifique `/status` para confirmar qué método está activo. La fila `Login method` muestra su cuenta de suscripción, y aparece una fila `API key` cuando se está utilizando una clave API.

[Claude Code en la Web](/docs/es/claude-code-on-the-web) siempre usa sus credenciales de suscripción. Si establece `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` en el entorno de sandbox, no anula sus credenciales de suscripción.

<h3 id="generate-a-long-lived-token">
  Generar un token de larga duración
</h3>

Para canalizaciones de CI, scripts u otros entornos donde el inicio de sesión interactivo del navegador no está disponible, genere un token OAuth de un año con `claude setup-token`:

```bash theme={null}
claude setup-token
```

El comando lo guía a través de la autorización OAuth e imprime un token en el terminal. No guarda el token en ningún lugar; cópielo y establézcalo como la variable de entorno `CLAUDE_CODE_OAUTH_TOKEN` donde desee autenticarse:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Este token se autentica con su suscripción de Claude y requiere un plan Pro, Max, Team o Enterprise. Se limita solo a inferencia y no puede establecer sesiones de [Remote Control](/docs/es/remote-control).

[Bare mode](/docs/es/headless#start-faster-with-bare-mode) no lee `CLAUDE_CODE_OAUTH_TOKEN`. Si su script pasa `--bare`, autentíquese con `ANTHROPIC_API_KEY` o un `apiKeyHelper` en su lugar.
