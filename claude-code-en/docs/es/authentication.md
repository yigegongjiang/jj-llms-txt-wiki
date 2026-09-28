> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autenticación

> Inicie sesión en Claude Code y configure la autenticación para individuos, equipos y organizaciones.

Claude Code admite múltiples métodos de autenticación según su configuración. Los usuarios individuales pueden iniciar sesión con una cuenta de Claude.ai, mientras que los equipos pueden usar Claude for Teams o Enterprise, la Claude Console, o un proveedor de nube como Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Inicie sesión en Claude Code
</h2>

Después de [instalar Claude Code](/docs/es/setup#install-claude-code), ejecute `claude` en su terminal. En el primer lanzamiento, Claude Code abre una ventana del navegador para que inicie sesión. Si ha establecido la variable de entorno `ANTHROPIC_API_KEY`, Claude Code omite el símbolo del sistema de inicio de sesión y le pide que apruebe la clave en su lugar.

Si el navegador no se abre automáticamente, presione `c` para copiar la URL de inicio de sesión al portapapeles y luego péguelo en su navegador.

Si su navegador muestra un código de inicio de sesión en lugar de redirigirse después de que inicie sesión, péguelo en el terminal en el símbolo del sistema `Paste code here if prompted`. Esto sucede cuando el navegador no puede alcanzar el servidor de devolución de llamada local de Claude Code, lo cual es común en WSL2, sesiones SSH y contenedores.

Cuando el inicio de sesión se completa, el terminal muestra `Login successful` y le solicita que presione `Enter` para continuar.

Puede autenticarse con cualquiera de estos tipos de cuenta:

* **Suscripción Claude Pro o Max**: inicie sesión con su cuenta de claude.ai. Suscríbase en [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams o Enterprise**: inicie sesión con la cuenta de claude.ai que su administrador de equipo le invitó a usar.
* **Claude Console**: inicie sesión con sus credenciales de Console. Su administrador debe haberle [invitado](#claude-console-authentication) primero. Puede iniciar sesión con o sin [crear una clave de API](#sign-in-without-an-api-key).
* **Proveedores de nube**: si su organización usa [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry), establezca las variables de entorno requeridas antes de ejecutar `claude`, o seleccione **plataforma de terceros** en el símbolo del sistema de inicio de sesión, que inicia un asistente de configuración interactivo para Bedrock y Vertex AI. No se necesita inicio de sesión en el navegador.
* **Puerta de enlace en la nube**: si su organización ejecuta una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada, inicie sesión con SSO corporativo a través de `/login`. El token emitido por la puerta de enlace es la única credencial de la sesión.

Los administradores pueden dirigir qué método de inicio de sesión utilizan los desarrolladores y requerir que los inicios de sesión de claude.ai pertenezcan a una organización específica; consulte [Restringir el inicio de sesión a su organización](#restrict-login-to-your-organization).

Para cerrar sesión y volver a autenticarse, escriba `/logout` en el símbolo del sistema de Claude Code. Cerrar sesión también restablece su estado de configuración de primer lanzamiento, por lo que la próxima vez que ejecute `claude` le guiará a través del inicio de sesión y la configuración nuevamente.

Si tiene problemas para iniciar sesión, consulte [solución de problemas de autenticación](/docs/es/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Configurar la autenticación del equipo
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

* **Claude for Teams**: plan de autoservicio con características de colaboración, herramientas de administración, SSO, gestión de facturación y [configuración administrada por servidor](/docs/es/server-managed-settings) para configuración de Claude Code en toda la organización. Mejor para equipos más pequeños.
* **Claude for Enterprise**: añade captura de dominio, permisos basados en roles y API de cumplimiento. Mejor para organizaciones más grandes con requisitos de seguridad y cumplimiento.

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

<h4 id="sign-in-without-an-api-key">
  Iniciar sesión sin una clave API
</h4>

Puede iniciar sesión en su cuenta de Console sin crear una clave API, incluso cuando su organización no permite que los desarrolladores las creen. Elija la cuenta de Anthropic Console en el indicador `/login` y Claude Code le pregunta cómo desea iniciar sesión. Requiere Claude Code v2.1.242 o posterior. Ambas rutas lo inician sesión en Console en el navegador y difieren en lo que Claude Code almacena después:

* **Iniciar sesión con su cuenta de Console**, etiquetado como `(recomendado)`: Claude Code mantiene el token OAuth de ese inicio de sesión y lo almacena como un [perfil de Anthropic](#anthropic-profiles-and-federation-credentials). No crea ninguna clave API
* **Crear una clave API**, etiquetado como `(heredado)`: Claude Code crea una clave API de Console para usted y la almacena con sus otras credenciales

En la práctica, el perfil almacena un inicio de sesión OAuth mientras que una clave API es una credencial estática: Claude Code actualiza automáticamente el inicio de sesión del perfil, y cuando la actualización falla, las solicitudes fallan con [Inicio de sesión de perfil de Anthropic expirado](/docs/es/errors#anthropic-profile-login-expired) hasta que inicie sesión nuevamente.

No obtiene la opción en cada máquina. Claude Code crea una clave API sin preguntar en estos casos:

* Ejecuta contra un proveedor de nube, como [Amazon Bedrock, Plataforma de agentes de Google Cloud o Microsoft Foundry](/docs/es/third-party-integrations) o [Claude Platform en AWS](/docs/es/claude-platform-on-aws)
* Cualquier archivo de configuración establece [`forceLoginOrgUUID`](#restrict-login-to-your-organization), o establece `forceLoginMethod` en `"claudeai"` o `"console"`
* Existe una fuente de configuración administrada en su máquina, como el archivo de configuración administrada, un perfil MDM o la configuración administrada por servidor en caché, pero Claude Code [no puede leerla](/docs/es/managed-settings#invalid-entries-in-managed-settings) y ninguna otra fuente administrada proporciona una política

Desestablezca `ANTHROPIC_API_KEY` antes de iniciar sesión sin una clave. Un perfil escrito por el propio inicio de sesión de Console de Claude Code, o por el `ant auth login` de la CLI de Claude Platform, es el mismo tipo de credencial, por lo que iniciar sesión nuevamente lo reemplaza.

Después de iniciar sesión sin una clave, tiene un perfil en lugar de una clave API almacenada:

* **Qué perfil escribe**: Claude Code escribe el perfil nombrado por `ANTHROPIC_PROFILE`, o su perfil activo, o `default`. Si ese perfil es un perfil de federación, Claude Code rechaza el inicio de sesión en lugar de sobrescribirlo
* **De qué cierra sesión**: Claude Code cierra sesión de cualquier inicio de sesión de claude.ai almacenado en la máquina
* **Cómo deshacerlo**: ejecute `/logout`, que elimina y revoca la credencial que escribió este inicio de sesión

Si su organización usa [configuración administrada por servidor](/docs/es/server-managed-settings), se aplican a este inicio de sesión en Claude Code v2.1.257 o posterior.

Todo lo demás sobre perfiles se aplica a este inicio de sesión, incluido dónde se clasifica contra sus otras credenciales, la fila `Profile` que obtiene en `/status` y las características que necesitan un inicio de sesión de claude.ai. Consulte [Perfiles de Anthropic y credenciales de federación](#anthropic-profiles-and-federation-credentials).

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

<h3 id="restrict-login-to-your-organization">
  Restringir el inicio de sesión a su organización
</h3>

Para requerir que los inicios de sesión de claude.ai de los desarrolladores pertenezcan a una organización específica de Anthropic, establezca [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod) y [`forceLoginOrgUUID`](/docs/es/settings-reference#forceloginorguuid) en [configuración administrada](/docs/es/managed-settings). Establezca `forceLoginOrgUUID` en su ID de organización, que se muestra en [configuración de administrador de claude.ai](https://claude.ai/admin-settings/organization) para organizaciones de Claude for Teams o Enterprise. Claude Code reporta un error para un inicio de sesión de claude.ai en cualquier otra organización y sale al inicio si la credencial de claude.ai en uso pertenece a una organización que no está en la lista.

Para inicios de sesión de Claude Console, Claude Code usa `forceLoginOrgUUID` para preseleccionar la organización en la página de inicio de sesión de Console cuando lo establece en un único ID de organización de Console, que se muestra en [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization). No verifica a qué organización pertenece la credencial de Console resultante, en el inicio de sesión o al inicio, y un desarrollador que inició sesión con una cuenta de Console antes de que implementara las claves permanece conectado.

Si establece `forceLoginOrgUUID` en cualquier archivo de configuración, Claude Code deja de ofrecer el [inicio de sesión de Console sin claves](#sign-in-without-an-api-key) en las sesiones a las que se aplica ese archivo y crea una clave API en su lugar. Para dirigir a los desarrolladores al inicio de sesión de claude.ai en su lugar, establezca `forceLoginMethod` en `"claudeai"`.

Los desarrolladores pueden iniciar sesión desde varias rutas: el flujo de terminal `/login`, la [extensión de VS Code](/docs/es/vs-code), el Agent SDK, `claude setup-token`, `/install-github-app` e [inicio de sesión de puerta de enlace](/docs/es/claude-apps-gateway) para organizaciones que enrutan a través de una puerta de enlace de nube. En Claude Code v2.1.212 o posterior, cada ruta aplica `forceLoginMethod`; antes de v2.1.212, solo los inicios de sesión de terminal aplicaban cualquiera de las claves. En la pantalla de inicio de sesión interactivo del terminal, a la que se accede mediante `/login` u onboarding de primera ejecución, Claude Code preselecciona un método `claudeai` o `console` sin aplicarlo, por lo que incluso con `forceLoginMethod` establecido en `"claudeai"`, un desarrollador aún puede completar un inicio de sesión de Console allí. Las rutas difieren en `forceLoginOrgUUID`:

* **Terminal, extensión de VS Code e inicios de sesión de Agent SDK**: verifican `forceLoginOrgUUID` para inicios de sesión de cuenta de claude.ai
* **`claude setup-token` e `/install-github-app`**: aplican solo `forceLoginMethod`, por lo que pueden acuñar un token en una organización diferente
* **Inicio de sesión de [puerta de enlace](/docs/es/claude-apps-gateway)**: seleccionado por `forceLoginMethod: "gateway"` en lugar de restringido por él, y no se autentica contra una organización de Anthropic, por lo que `forceLoginOrgUUID` no se aplica; use su proveedor de identidad de puerta de enlace para restringir el acceso

Implemente las claves a través de su herramienta de administración de dispositivos. [La configuración administrada por servidor](/docs/es/server-managed-settings) solo llega a cuentas que ya están autenticadas en su organización, por lo que no pueden redirigir el primer inicio de sesión de un desarrollador. Si su organización también distribuye configuración administrada por servidor, establezca las claves en ambos lugares: las fuentes de [configuración administrada no se fusionan](/docs/es/server-managed-settings#settings-precedence), y la configuración administrada por servidor en caché reemplaza el archivo administrado por dispositivo, aparte de algunos [excepciones por clave](/docs/es/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` y los valores `"claudeai"` y `"console"` de `forceLoginMethod` no están entre esas excepciones, así que manténgalos en ambos lugares.

Las claves también deciden si una sesión que no usa una credencial de inicio de sesión puede iniciarse. Consulte [`forceLoginOrgUUID`](/docs/es/settings-reference#forceloginorguuid) en la referencia de configuración para el comportamiento completo.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper`**: bloqueados al inicio, ya que la pertenencia a la organización no se puede verificar para una credencial de entorno
* **Sesiones de proveedor de nube como Amazon Bedrock**: no bloqueadas, porque se autentican contra su proveedor de nube. Restrinja esas a través de sus políticas de IAM en la nube
* **[Perfil de Anthropic o credenciales de federación](#anthropic-profiles-and-federation-credentials)**: no bloqueadas, y las claves no verifican a qué organización pertenece el perfil

<h2 id="credential-management">
  Gestión de credenciales
</h2>

Claude Code administra de forma segura sus credenciales de autenticación:

* **Ubicación de almacenamiento**:
  * En macOS, las credenciales se almacenan en el Keychain de macOS cifrado. Cuando el Keychain rechaza la escritura, como cuando está bloqueado en una sesión SSH, Claude Code almacena su inicio de sesión en `~/.claude/.credentials.json` con modo de archivo `0600` en su lugar, el mismo almacenamiento que utiliza en Linux. Un inicio de sesión de Console que crea una clave API falla hasta que el Keychain sea escribible. Para mover su inicio de sesión de vuelta al Keychain, siga [los pasos de recuperación](/docs/es/troubleshoot-install#not-logged-in-or-token-expired).
  * En Linux, las credenciales se almacenan en `~/.claude/.credentials.json` con modo de archivo `0600`.
  * En Windows, las credenciales se almacenan en `%USERPROFILE%\.claude\.credentials.json` y heredan los controles de acceso del directorio de su perfil de usuario, lo que restringe el archivo a su cuenta de usuario de forma predeterminada.
  * Si ha establecido la variable de entorno `CLAUDE_CONFIG_DIR`, Claude Code mantiene el archivo `.credentials.json` bajo ese directorio en su lugar, incluido el archivo que escribe la alternativa de macOS, y también asigna la entrada del Keychain de macOS a ese directorio, por lo que una sesión con un `CLAUDE_CONFIG_DIR` diferente lee una entrada diferente.
  * Claude Code administra `.credentials.json` a través de `/login` y `/logout`. Para enrutar solicitudes a través de un punto final de API personalizado, establezca la variable de entorno [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) en su lugar.
* **Tipos de autenticación admitidos**: credenciales de claude.ai, credenciales de API de Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, credenciales de perfil de Anthropic y [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), y tokens de sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway).
* **Scripts de credenciales personalizados**: configure la configuración [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) para ejecutar un script de shell que devuelva una clave API.
* **Intervalos de actualización**: Claude Code vuelve a ejecutar `apiKeyHelper` después de cinco minutos de forma predeterminada. Establezca la variable de entorno `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` para intervalos de actualización personalizados. Consulte [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) para los otros casos en los que Claude Code vuelve a ejecutar el helper.
* **Aviso de helper lento**: si `apiKeyHelper` tarda más de 10 segundos en devolver una clave, Claude Code muestra un aviso de advertencia en la barra de símbolo del sistema mostrando el tiempo transcurrido. Si ve este aviso regularmente, verifique si su script de credenciales se puede optimizar.
* **Fallos del helper**: cuando el script sale con un error, agota el tiempo de espera o no imprime nada, las solicitudes fallan con [`Your apiKeyHelper script is failing`](/docs/es/errors#your-apikeyhelper-script-is-failing) dentro de tres intentos. Antes de v2.1.208, los fallos del helper aparecían como un 401 genérico después de aproximadamente diez reintentos silenciosos.

`apiKeyHelper`, `ANTHROPIC_API_KEY` y `ANTHROPIC_AUTH_TOKEN` se aplican a la CLI y a las superficies que la envuelven, incluida la extensión de VS Code, el Agent SDK y GitHub Actions. Claude Desktop y las sesiones en la nube no llaman a `apiKeyHelper` ni leen estas variables de entorno: utilizan OAuth, excepto las sesiones de escritorio que ejecutan una [configuración de inferencia de terceros](/docs/es/llm-gateway-connect#desktop-app), que se autentican con la credencial de esa configuración.

<h3 id="renew-an-expiring-login">
  Renovar un inicio de sesión que está por expirar
</h3>

Cuando el inicio de sesión que creó con `/login` está a menos de tres días de expirar, Claude Code muestra una advertencia al inicio: `Your login expires in 3 days · run /login to renew`. Requiere Claude Code v2.1.203 o posterior. Antes de v2.1.217, la advertencia aparecía cinco días antes.

Ejecute `/login` para renovar. La advertencia es informativa y nunca bloquea una solicitud: la autenticación sigue funcionando hasta que el inicio de sesión realmente expire. La duración del inicio de sesión en sí no cambia; la advertencia anticipada es lo que v2.1.203 añade.

Una vez que el inicio de sesión almacenado expira y no se puede actualizar, cada solicitud de modelo falla con [`Login expired · Please run /login`](/docs/es/errors#login-expired) hasta que inicie sesión nuevamente. Antes de v2.1.206, Claude Code reportaba un inicio de sesión expirado en solicitudes de modelo como un error de modelo en su lugar.

Puede verificar este estado antes de que una solicitud falle: [`/status`](/docs/es/commands) muestra una fila `Login` que dice `Expired — log in again`, más la organización y el correo electrónico que tiene guardados para el inicio de sesión expirado. La fila aparece solo cuando el inicio de sesión de claude.ai o Claude Console guardado es la credencial activa. La fila requiere Claude Code v2.1.210 o posterior.

La advertencia aparece solo cuando un inicio de sesión de claude.ai o Claude Console es la credencial activa, y no cuando un proveedor de nube, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` proporciona la credencial.

Renovar anticipadamente es más importante para sesiones que se ejecutan sin supervisión. Una [sesión en segundo plano en vista de agente](/docs/es/agent-view) o una sesión de [Remote Control](/docs/es/remote-control) que sobrevive al inicio de sesión deja de hacer progreso una vez que la credencial expira y no puede recuperarse hasta que inicie sesión nuevamente.

<h3 id="authentication-precedence">
  Precedencia de autenticación
</h3>

Cuando hay múltiples credenciales presentes, Claude Code elige una en este orden:

1. Credenciales del proveedor de nube, cuando `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` o `CLAUDE_CODE_USE_FOUNDRY` está establecido. Consulte [integraciones de terceros](/docs/es/third-party-integrations) para la configuración.
2. Variable de entorno `ANTHROPIC_AUTH_TOKEN`. Se envía como encabezado `Authorization: Bearer`. Use esto cuando enrute a través de una [puerta de enlace LLM o proxy](/docs/es/llm-gateway) que se autentica con tokens de portador en lugar de claves API de Anthropic.
3. Variable de entorno `ANTHROPIC_API_KEY`. Se envía como encabezado `X-Api-Key`. Use esto para acceso directo a la API de Anthropic con una clave de [Claude Console](https://platform.claude.com). En modo interactivo, se le solicita una vez que apruebe o rechace la clave, y su elección se recuerda. Para cambiarla más tarde, use el botón de alternancia "Use custom API key" en `/config`. El botón de alternancia solo aparece mientras `ANTHROPIC_API_KEY` está establecido en su entorno. En modo no interactivo (`-p`), la clave siempre se usa cuando está presente.
4. Salida del script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper). Use esto para credenciales dinámicas o rotativas, como tokens de corta duración obtenidos de un almacén.
5. Variable de entorno `CLAUDE_CODE_OAUTH_TOKEN`. Un token OAuth de larga duración generado por [`claude setup-token`](#generate-a-long-lived-token). Use esto para canalizaciones de CI y scripts donde el inicio de sesión del navegador no está disponible. Si ejecuta `/login` mientras la variable está establecida, Claude Code cambia la sesión actual al nuevo inicio de sesión, pero lee la variable nuevamente en cada nueva sesión hasta que la elimine del perfil de su shell o del bloque `env` de un [archivo de configuración](/docs/es/settings).
6. Credenciales de perfil de Anthropic y de federación, las credenciales que utiliza la CLI `ant` y Workload Identity Federation. Un perfil que `ant auth login` escribió se clasifica aquí solo cuando lo nombra en `ANTHROPIC_PROFILE`; de lo contrario, se clasifica por debajo de `/login`. Consulte [Perfiles de Anthropic y credenciales de federación](#anthropic-profiles-and-federation-credentials).
7. Credenciales OAuth de suscripción de `/login`. Este es el predeterminado para usuarios de Claude Pro, Max, Team y Enterprise.

Una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada se encuentra fuera de esta lista: es una selección de proveedor como Amazon Bedrock o la Plataforma de Agentes de Google Cloud, y los supera. Cuando existe una sesión de puerta de enlace, la CLI se autentica con el token de puerta de enlace incluso si `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` o `CLAUDE_CODE_USE_FOUNDRY` está establecido, y las fuentes de credenciales anteriores como el token de portador, la clave API, `apiKeyHelper` y los perfiles no se utilizan.

Si la [configuración administrada](/docs/es/managed-settings) de su máquina establece [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod) en `"gateway"` o establece [`forceLoginGatewayUrl`](/docs/es/settings-reference#forcelogingatewayurl), y no selecciona un proveedor de nube a través de una variable como `CLAUDE_CODE_USE_BEDROCK` o `CLAUDE_CODE_USE_VERTEX`, su sesión utiliza solo el inicio de sesión de puerta de enlace. Claude Code omite las otras fuentes de credenciales y le pide que inicie sesión con `/login`. Consulte [Administrator policy requires a Cloud gateway sign-in](/docs/es/errors#administrator-policy-requires-a-cloud-gateway-sign-in) para ver lo que ve con cada credencial restante. Antes de v2.1.261, o antes de v2.1.265 en una máquina que solo establece `forceLoginGatewayUrl`, Claude Code utilizaba un inicio de sesión guardado restante en estas máquinas hasta que iniciara sesión en la puerta de enlace.

Si tiene una suscripción activa de Claude pero también tiene `ANTHROPIC_API_KEY` establecido en su entorno, Claude Code usa la clave API una vez que la aprueba. Esto puede causar fallos de autenticación si la clave pertenece a una organización deshabilitada o expirada.

Ejecute `unset ANTHROPIC_API_KEY` para volver a su suscripción, y verifique `/status` para confirmar qué método está activo. Cuando un inicio de sesión y una clave API están ambos configurados, `/status` marca la credencial que no está en uso.

[Claude Code en la Web](/docs/es/claude-code-on-the-web) siempre usa sus credenciales de suscripción. Si establece `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` en el entorno en la nube, no anula sus credenciales de suscripción.

<h4 id="anthropic-profiles-and-federation-credentials">
  Perfiles de Anthropic y credenciales de federación
</h4>

Un perfil es un archivo de configuración de credenciales nombrado en su [directorio de configuración de Anthropic](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory), de forma predeterminada `~/.config/anthropic` en macOS y Linux o `%APPDATA%\Anthropic` en Windows. El modo de autenticación de un perfil es `oidc_federation` cuando lo configura para [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) o `user_oauth` cuando [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) lo escribió o usted [inició sesión en una cuenta de Console sin una clave API](#sign-in-without-an-api-key).

Claude Code no lee perfiles o variables de federación en [modo bare](/docs/es/headless#start-faster-with-bare-mode), en Claude Desktop o en sesiones en la nube. En esas sesiones, `/status` no muestra ninguna fila `Profile`.

Claude Code verifica tres fuentes en este orden y se detiene en la primera que está establecida. La tabla muestra qué establece cada fuente y dónde se clasifica contra su credencial `/login`.

| Fuente                  | Establecido por                                                                                                                                                                | Clasificación contra `/login`                                                                                                                               |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Perfil nombrado         | `ANTHROPIC_PROFILE`                                                                                                                                                            | Arriba, cualquiera que sea el modo de autenticación que tenga el perfil                                                                                     |
| Variables de federación | `ANTHROPIC_FEDERATION_RULE_ID` y `ANTHROPIC_ORGANIZATION_ID`, ambas establecidas                                                                                               | Arriba                                                                                                                                                      |
| Perfil activo           | El archivo [`active_config`](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) en su directorio de configuración, o un perfil nombrado `default` | Arriba cuando su modo de autenticación es `oidc_federation`; debajo de una credencial `/login` que funciona cuando su modo de autenticación es `user_oauth` |

La regla `user_oauth` evita que un perfil `ant auth login` sobrante mueva sus solicitudes fuera de la cuenta en la que inició sesión con `/login`. Para las variables de federación, Claude Code también lee las otras variables en la [referencia de WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), como `ANTHROPIC_IDENTITY_TOKEN_FILE`, cuando intercambia su token de identidad. Para el formato del archivo de perfil, consulte la [referencia de WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Para confirmar qué fuente eligió Claude Code, ejecute `/status`. Una fila `Profile` nombra la fuente en lugar de la fila `Login method`. Cuando el perfil es la credencial en uso, las filas `Organization` y `Email` muestran su cuenta.

Si inicia Claude Code con `--debug`, también escribe una línea `Using Anthropic profile auth` con el nombre de la fuente en el registro de depuración en `~/.claude/debug/<session-id>.txt`. Cuando Claude Code omite un perfil activo `user_oauth` porque tiene una credencial `/login` que funciona, escribe una advertencia en el registro de depuración diciendo que está usando el inicio de sesión de claude.ai en su lugar.

Cuando el inicio de sesión de un perfil `user_oauth` ha expirado y Claude Code no puede renovarlo, las solicitudes fallan con [Anthropic profile login expired](/docs/es/errors#anthropic-profile-login-expired).

Las características que necesitan su inicio de sesión de claude.ai, como [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) y [`/schedule`](/docs/es/routines), no están disponibles mientras una de estas fuentes está seleccionada. Para evitar que Claude Code seleccione una fuente:

* **Perfil nombrado o variables de federación**: desestablezca `ANTHROPIC_PROFILE`, o desestablezca cualquiera de las variables de federación
* **Perfil activo**: ejecute `/logout` para un perfil `user_oauth` cuya credencial actual escribió [iniciando sesión en una cuenta de Console sin una clave API](#sign-in-without-an-api-key), ejecute `ant auth logout` para uno cuya credencial actual escribió `ant auth login`, o elimine el archivo del perfil de `configs/` en su directorio de configuración para cualquier modo de autenticación

<h3 id="generate-a-long-lived-token">
  Generar un token de larga duración
</h3>

Para canalizaciones de CI, scripts u otros entornos donde el inicio de sesión interactivo del navegador no está disponible, genere un token OAuth de un año con `claude setup-token`:

```bash theme={null}
claude setup-token
```

El comando abre el mismo flujo de autorización del navegador que `/login`, y el token se imprime en la terminal después de que apruebe el acceso en el navegador. No guarda el token en ningún lugar; cópielo y establézcalo como la variable de entorno `CLAUDE_CODE_OAUTH_TOKEN` donde desee autenticarse:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Este token se autentica con su suscripción de Claude y requiere un plan Pro, Max, Team o Enterprise. Solo puede hacer solicitudes de modelo, por lo que no puede establecer sesiones de [Remote Control](/docs/es/remote-control) u obtener [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai). Los servidores MCP que configura localmente siguen funcionando.

[Bare mode](/docs/es/headless#start-faster-with-bare-mode) no lee `CLAUDE_CODE_OAUTH_TOKEN`. Si su script pasa `--bare`, autentíquese con `ANTHROPIC_API_KEY` o un `apiKeyHelper` en su lugar.
