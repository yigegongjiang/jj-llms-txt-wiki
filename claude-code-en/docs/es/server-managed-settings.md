> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar la configuración administrada por servidor

> Configure Claude Code centralmente para su organización a través de configuración entregada por servidor, sin requerir infraestructura de administración de dispositivos.

La configuración administrada por servidor permite a los propietarios de la organización configurar Claude Code centralmente desde [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) en la consola de claude.ai. Los clientes de Claude Code obtienen automáticamente estas configuraciones cuando los usuarios se autentican con una credencial elegible en una plataforma donde se admite la entrega administrada por servidor. Consulte [Disponibilidad de plataforma](#platform-availability) para conocer las credenciales y plataformas que califican.

<Note>
  La configuración administrada por servidor está disponible para clientes de [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) y [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise).
</Note>

<h2 id="requirements">
  Requisitos
</h2>

Para usar la configuración administrada por servidor, necesita:

* Plan Claude for Teams o Claude for Enterprise
* El rol de Propietario o Propietario Principal en su organización de Claude, para ver y editar la configuración
* Acceso de red a `api.anthropic.com`

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  Elegir entre configuración administrada por servidor y administrada por endpoint
</h2>

Claude Code admite dos enfoques para la configuración centralizada. La configuración administrada por servidor entrega la configuración desde los servidores de Anthropic. La [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) se implementa directamente en dispositivos a través de políticas nativas del sistema operativo (preferencias administradas de macOS, registro de Windows) o archivos de configuración administrados.

| Enfoque                                                                                 | Mejor para                                                          | Modelo de seguridad                                                                                                                                   |
| :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Configuración administrada por servidor**                                             | Organizaciones sin MDM, o usuarios en dispositivos no administrados | Configuración que Claude Code obtiene de los servidores de Anthropic al iniciar y actualiza cada hora durante la sesión                               |
| **[Configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms)** | Organizaciones con MDM o administración de endpoint                 | Configuración implementada en dispositivos a través de perfiles de configuración MDM, políticas de registro o archivos de configuración administrados |

Si sus dispositivos están inscritos en una solución MDM o de administración de endpoint, la configuración administrada por endpoint proporciona garantías de seguridad más sólidas porque el archivo de configuración puede protegerse de la modificación del usuario a nivel del sistema operativo. La configuración administrada por endpoint no llega a las [sesiones en la nube](/docs/es/model-config#surface-coverage) en entornos alojados por Anthropic, por lo que las organizaciones cuyos desarrolladores ejecutan sesiones en la nube también deben configurar la configuración administrada por servidor. Las sesiones en un [entorno autohospedado](/docs/es/self-hosted-environments) también leen el archivo de configuración administrada en la imagen del ejecutor. La [precedencia de configuración](#settings-precedence) a continuación indica cuándo se aplica ese archivo.

<h2 id="configure-server-managed-settings">
  Configurar la configuración administrada por servidor
</h2>

<Steps>
  <Step title="Abrir la consola de administración">
    En la consola de claude.ai, vaya a [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Si el enlace lo redirige a una página de Admin Settings diferente en lugar de la página de Claude Code, su cuenta no tiene el rol requerido. Los roles de Admin y otros roles que no sean Owner no pueden ver ni editar la configuración administrada, así que pida a un Owner o Primary Owner en su organización que realice el cambio. Consulte [Control de acceso](#access-control).
  </Step>

  <Step title="Definir su configuración">
    Agregue su configuración como JSON. Todas las [configuraciones disponibles en `settings.json`](/docs/es/settings-reference#all-settings) son compatibles excepto las restringidas a la entrega de políticas a nivel del sistema operativo; consulte [Limitaciones actuales](#current-limitations) para esa lista breve. Esto incluye [hooks](/docs/es/hooks), [variables de entorno](/docs/es/env-vars) y [configuraciones solo administradas](/docs/es/managed-settings#managed-only-settings) como `allowManagedPermissionRulesOnly`.

    Este ejemplo aplica una lista de denegación de permisos, impide que los usuarios omitan permisos y restringe las reglas de permisos a las definidas en la configuración administrada. La regla `Bash(curl *)` coincide con `curl` [tal como Claude la escribe](/docs/es/permissions#bash-rule-limits), no `/usr/bin/curl` o `sh -c 'curl …'`; para la aplicación de red que no depende del texto del comando, agregue un [bloque `sandbox` con `allowManagedDomainsOnly`](/docs/es/sandboxing#configure-the-sandbox-for-your-organization).

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Los hooks utilizan el mismo formato que en `settings.json`.

    Este ejemplo ejecuta un script de auditoría después de cada edición de archivo en toda la organización:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Debido a que los hooks ejecutan comandos de shell, los usuarios en sesiones interactivas ven un [diálogo de aprobación de seguridad](#security-approval-dialogs) antes de que Claude Code los aplique.

    Para configurar el clasificador del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para que sepa qué repositorios, buckets y dominios confía su organización, entregue un bloque `autoMode` de la misma manera; consulte [Configurar el modo automático](/docs/es/auto-mode-config) para ver cómo las entradas de `autoMode` afectan lo que el clasificador bloquea y advertencias importantes sobre los campos `environment`, `allow`, `soft_deny` y `hard_deny`.
  </Step>

  <Step title="Guardar e implementar">
    Guarde sus cambios. Los clientes de Claude Code reciben la configuración actualizada en su próximo inicio o ciclo de sondeo por hora.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verificar la entrega de configuración
</h3>

Para confirmar que la configuración se está aplicando, pida a un usuario que reinicie Claude Code. Si la configuración incluye configuraciones que activan el [diálogo de aprobación de seguridad](#security-approval-dialogs), el usuario ve un mensaje que describe la configuración administrada la próxima vez que Claude Code la obtiene: en el próximo inicio o dentro de una hora en una sesión interactiva en ejecución. También puede verificar que las reglas de permisos administrados estén activas haciendo que un usuario ejecute `/permissions` para ver sus reglas de permisos efectivas.

Para verificar el resultado de la obtención en una máquina específica, haga que el usuario ejecute `claude doctor` y lea la línea `Managed settings (remote)`. Requiere Claude Code v2.1.248 o posterior. La línea informa uno de cuatro resultados:

* La configuración entregada se cargó
* Su organización no tiene configuración administrada por servidor configurada
* La obtención falló, con la causa y si una política en caché aún se aplica
* Claude Code omitió la obtención, con la razón. Consulte [Disponibilidad de plataforma](#platform-availability) para los proveedores y configuraciones que la omiten

Mientras la obtención aún está en progreso, la línea informa eso en su lugar.

En una sesión en ejecución, `/status` muestra la misma línea después de una obtención fallida, y para algunas causas de obtención omitida, como una variable de proveedor de terceros o una `ANTHROPIC_BASE_URL` personalizada exportada en el shell del usuario.

<h3 id="access-control">
  Control de acceso
</h3>

Los siguientes roles pueden administrar la configuración administrada por servidor:

* **Primary Owner**
* **Owner**

Restrinja el acceso al personal de confianza, ya que los cambios de configuración se aplican a todos los usuarios de la organización.

<h3 id="managed-only-settings">
  Configuraciones solo administradas
</h3>

La mayoría de las [claves de configuración](/docs/es/settings-reference#all-settings) funcionan en cualquier ámbito. Un puñado de claves solo se leen de la configuración administrada y no tienen efecto cuando se colocan en archivos de configuración de usuario o proyecto. Consulte [configuraciones solo administradas](/docs/es/managed-settings#managed-only-settings) para los controles de permisos y plugins, o lea la columna Scope del índice [Todas las configuraciones](/docs/es/settings-reference#all-settings) para el conjunto completo.

<h3 id="current-limitations">
  Limitaciones actuales
</h3>

La configuración administrada por servidor tiene las siguientes limitaciones:

* La configuración se aplica uniformemente a todos los usuarios de la organización. Las configuraciones por grupo aún no son compatibles.
* Un archivo [`managed-mcp.json`](/docs/es/managed-mcp) no se puede distribuir a través de la configuración administrada por servidor. Entregue las claves de política `allowedMcpServers` y `deniedMcpServers` allí en su lugar. En Claude Code v2.1.259 o posterior, también puede proporcionar servidores remotos con [`managedMcpServers`](/docs/es/managed-mcp#provide-servers-through-managed-settings), que acepta solo servidores `http` y `sse` y no toma control exclusivo de la manera que lo hace el archivo.

  Claude Code lee un archivo `managed-mcp.json` implementado en su [ruta del sistema](/docs/es/managed-mcp#exclusive-control-with-managed-mcp-json) por separado del nivel de configuración administrada, por lo que el archivo aún se aplica cuando la configuración administrada por servidor está en vigor.
* Las configuraciones restringidas a fuentes de políticas a nivel del sistema operativo, como `policyHelper` y `wslInheritsWindowsSettings`, no se respetan. Impleméntelas a través de MDM o un archivo `managed-settings.json` del sistema en su lugar. Un `policyHelper` implementado de esa manera se ejecuta solo cuando su fuente es la seleccionada en [precedencia dentro del nivel administrado](/docs/es/managed-settings#precedence-within-the-managed-tier).

<h2 id="settings-delivery">
  Entrega de configuración
</h2>

<h3 id="settings-precedence">
  Precedencia de configuración
</h3>

La configuración administrada por servidor y la [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) ocupan el nivel más alto en la [jerarquía de configuración](/docs/es/settings#settings-precedence) de Claude Code. Ningún otro nivel de configuración puede anularlas, incluidos los argumentos de línea de comandos, aparte de las [excepciones a la precedencia de configuración administrada](/docs/es/settings#exceptions-to-managed-settings-precedence).

Dentro del nivel administrado, Claude Code utiliza de forma predeterminada la primera fuente que entrega al menos una clave de política, verificando primero la configuración administrada por servidor y luego la configuración administrada por endpoint, aparte de las [claves de excepción cubiertas a continuación](#per-key-exceptions-across-managed-sources). [Cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#precedence-within-the-managed-tier) tiene la clasificación completa, la exclusión para las claves de control y la opción de participación que se aplica a cada fuente.

Si la fuente seleccionada es una política MDM o un archivo de configuración administrada cuyo [`policyHelper`](/docs/es/settings-reference#policyhelper) proporciona configuración administrada, la salida del asistente reemplaza esa fuente como la única configuración administrada para la ejecución. Claude Code no consulta un `policyHelper` configurado en MDM o configuración basada en archivos mientras la configuración administrada por servidor entrega una clave de política.

Si una búsqueda posterior encuentra que se ha eliminado la configuración administrada por servidor, Claude Code ejecuta ese asistente de inmediato en lugar de en el siguiente lanzamiento. La entrada [`policyHelper`](/docs/es/settings-reference#policyhelper) cubre lo que sucede cuando esa ejecución falla.

Si borra su configuración administrada por servidor en la consola de administración con la intención de volver a una política de plist administrada por endpoint o de registro, tenga en cuenta que la [configuración en caché](#fetch-and-caching-behavior) persiste en las máquinas cliente hasta la siguiente búsqueda exitosa, y las claves que [se aplican solo en el siguiente lanzamiento](#fetch-and-caching-behavior), como `model`, permanecen en vigor hasta que cada cliente se reinicie. Ejecute `/status` para ver qué fuente administrada está activa.

<h3 id="per-key-exceptions-across-managed-sources">
  Excepciones por clave en fuentes administradas
</h3>

Tres tipos de claves son excepciones a la regla de no fusión:

* **Claves de bloqueo entre fuentes**: un pequeño conjunto de claves, como los bloqueos de lista de permitidos de sandbox, [enumeradas en la página de configuración administrada](/docs/es/managed-settings#precedence-within-the-managed-tier). Claude Code las honra cuando cualquier fuente administrada controlada por administrador las establece; el nivel de registro HKCU escribible por el usuario se excluye.

  Cuando un [`policyHelper`](/docs/es/settings-reference#policyhelper) proporciona configuración administrada, su salida es la única fuente que estas comprobaciones leen, aparte de [`forceRemoteSettingsRefresh`](/docs/es/settings-reference#forceremotesettingsrefresh), que Claude Code lee de las fuentes de administrador directamente al inicio.
* **El bloque `env`**: aparte de la unidad de telemetría y las variables de enrutamiento emparejadas con una clave de credencial, ambas cubiertas a continuación, se fusiona por clave en las fuentes controladas por administrador. Para cada variable de entorno, la fuente de mayor prioridad que la define gana, y las fuentes de administrador inferiores rellenan las variables que las fuentes superiores dejan sin establecer. Una entrada `env` administrada por endpoint se aplica, por lo tanto, siempre que la configuración administrada por servidor deje esa variable sin establecer, o mientras un valor de servidor en caché para ella se [retiene pendiente de confirmación del servidor](#fetch-and-caching-behavior). Requiere Claude Code v2.1.223 o posterior. Antes de v2.1.223, Claude Code aplica solo el bloque `env` de la fuente seleccionada.
  * **Unidad de telemetría**: las claves del exportador `OTEL_EXPORTER_OTLP_*`, los conmutadores de captura de contenido `OTEL_LOG_*`, `OTEL_LOGS_EXPORTER` y las variables de rastreo beta `ENABLE_BETA_TRACING_DETAILED` y `BETA_TRACING_ENDPOINT` siguen la fuente más alta que establece cualquiera de ellas como una unidad. Una fuente que entrega la clave de credencial `otelHeadersHelper` reclama la unidad también, pero coloca estas variables solo cuando es la fuente seleccionada: una fuente que no está seleccionada pero entrega la clave no contribuye ninguna de ellas y aún bloquea las fuentes inferiores de rellenarlas. De cualquier forma, un punto final del exportador de una fuente nunca puede emparejarse con credenciales de otra.
  * **Enrutamiento emparejado con credencial**: una fuente que empareja variables de enrutamiento con una clave de credencial solo para fuente seleccionada, como `apiKeyHelper` u `otelHeadersHelper`, contribuye esas variables de enrutamiento solo cuando gana la ranura.
* **Claves de inicio de sesión de puerta de enlace**: Claude Code nunca lee [`forceLoginGatewayUrl`](/docs/es/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/es/settings-reference#gatewayinternalnetworks), ni el valor `"gateway"` de [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod) de la configuración administrada por servidor, por lo que un valor allí ni se aplica ni oculta uno establecido en una política MDM o archivo de configuración administrada. La entrada [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) dice qué fuente de administrador en la máquina los suministra.

<h3 id="fetch-and-caching-behavior">
  Comportamiento de búsqueda y almacenamiento en caché
</h3>

Claude Code obtiene la configuración de los servidores de Anthropic al inicio y sondea actualizaciones cada hora durante sesiones activas.

Un cliente que inicia sesión a través de una [puerta de enlace de aplicaciones Claude](#platform-availability) obtiene su configuración de la puerta de enlace y espera esa búsqueda antes de que comience la sesión, por lo que la búsqueda en las listas a continuación no se aplica a ella. [Aplicar inicio cerrado por error](#enforce-fail-closed-startup) cubre lo que sucede cuando esa búsqueda falla.

**Primer lanzamiento sin configuración en caché:**

* Cuando un desarrollador inicia sesión al inicio, como en una primera ejecución o después de `/logout`, Claude Code espera hasta cinco segundos para la búsqueda antes de abrir la sesión. Cuando la política llega a tiempo, Claude Code la aplica desde la primera pantalla y muestra su [`companyAnnouncements`](/docs/es/settings-reference#companyannouncements) en ella. Cuando la carga útil necesita [aprobación de seguridad](#security-approval-dialogs), Claude Code termina la espera y aplica la carga útil una vez que el desarrollador la aprueba
* En cualquier otro inicio, y cuando se agota esa espera de cinco segundos, Claude Code abre la sesión mientras la búsqueda continúa, por lo que pasa una breve ventana antes de que se cargue la configuración y entren en vigor las restricciones
* Si la búsqueda falla, Claude Code continúa sin configuración administrada por servidor y advierte en sesiones interactivas que no se aplica ninguna política remota; la configuración administrada por endpoint aún se aplica. Si una fuente administrada establece [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup), Claude Code sale en su lugar

**Lanzamientos posteriores con configuración en caché:**

* La configuración en caché se aplica inmediatamente al inicio, excepto para los valores en caché `modelPricing` y `managedMcpServers` y las variables de entorno que Claude Code retiene hasta que el servidor confirma la carga útil
* Un [`modelPricing`](/docs/es/settings-reference#modelpricing) en caché no se aplica hasta que la búsqueda de la sesión confirma la carga útil. Hasta entonces, las cifras de costo que los desarrolladores ven en `/usage` y la línea de estado están al precio de lista
* Un bloque [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers) en caché no se aplica hasta que la búsqueda de la sesión confirma la carga útil. Claude Code espera hasta 30 segundos para esa búsqueda antes de conectar servidores MCP. Si la búsqueda falla o agota el tiempo de espera, la sesión comienza sin los servidores de la organización, `/status` lo dice, y se conectan una vez que una búsqueda posterior los confirma. Consulte [Cuándo se conectan los servidores proporcionados](/docs/es/managed-mcp#when-provided-servers-connect) para el comportamiento completo, incluido el primer lanzamiento. Requiere Claude Code v2.1.259 o posterior
* Claude Code obtiene configuración fresca en segundo plano
* La configuración en caché persiste a través de fallos de red. Si la búsqueda de inicio falla, Claude Code advierte en sesiones interactivas que la política en caché está en vigor
* Hasta que una búsqueda tenga éxito, los valores retenidos al inicio permanecen retenidos

Claude Code retiene varias categorías de variables en el bloque `env` en caché hasta que el servidor confirma la carga útil para la sesión. Esto evita que un valor de proxy, autoridad de certificado, punto final o credencial en caché redirija, intercepte o reautentique la búsqueda de configuración que confirma la carga útil. El endurecimiento se aplica solo al caché de configuración obtenido del servidor: la [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) implementada a través de MDM o `managed-settings.json` no se ve afectada. La retención requiere Claude Code v2.1.198 o posterior; antes de v2.1.198, todo el bloque `env` en caché se aplica al inicio. Las categorías retenidas incluyen:

* Configuración de proxy y TLS, como `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` y las variables de certificado de cliente mTLS `CLAUDE_CODE_CLIENT_CERT` y `CLAUDE_CODE_CLIENT_KEY`
* Enrutamiento de API y selección de proveedor, incluido `ANTHROPIC_BASE_URL`, las variables de selección de proveedor como `CLAUDE_CODE_USE_BEDROCK` y `CLAUDE_CODE_USE_VERTEX`, y las URL de punto final del proveedor como `ANTHROPIC_BEDROCK_BASE_URL`
* Credenciales de autenticación, como `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` y `CLAUDE_CODE_OAUTH_TOKEN`
* El selector de directorio de configuración `CLAUDE_CONFIG_DIR`
* Selectores de fuente de credencial y directorio de configuración, en Claude Code v2.1.223 o posterior: las variables de Workload Identity Federation como `ANTHROPIC_FEDERATION_RULE_ID` e `ANTHROPIC_IDENTITY_TOKEN`, los selectores de perfil y directorio de configuración `ANTHROPIC_PROFILE` y `ANTHROPIC_CONFIG_DIR`, y las variables de directorio del sistema operativo `HOME`, `XDG_CONFIG_HOME`, `APPDATA` y `USERPROFILE`

Claude Code lee las variables de Workload Identity Federation y los selectores `ANTHROPIC_PROFILE` y `ANTHROPIC_CONFIG_DIR` solo al inicio, por lo que un valor entregado por servidor para ellos no cambia la fuente de credencial de la sesión incluso después de que la búsqueda tenga éxito. Para entregar esos selectores en Claude Code v2.1.223 o posterior, use [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) como MDM o `managed-settings.json`. Para `CLAUDE_CONFIG_DIR` y las variables de directorio del sistema operativo, la retención en sí es la protección: el valor en caché permanece fuera del entorno hasta que el servidor confirma la carga útil.

Todas las demás claves en el bloque `env` en caché se aplican al inicio. Una vez que el servidor confirma la carga útil, y usted la aprueba si necesita [aprobación de seguridad](#security-approval-dialogs), las variables retenidas se aplican durante el resto de la sesión.

Si su organización necesita un proxy para llegar a `api.anthropic.com`, la retención solo afecta al bloque `env` entregado por servidor en sí: un proxy establecido en un bloque `env` [administrado por endpoint](/docs/es/managed-settings#delivery-mechanisms) a través de MDM o `managed-settings.json`, en el entorno del shell, o en [configuración de usuario](/docs/es/settings#where-settings-live) llega a la búsqueda de configuración. La fuente administrada por endpoint requiere Claude Code v2.1.223 o posterior: el valor de proxy administrado por servidor en caché se retiene hasta que la búsqueda lo confirma, por lo que el valor administrado por endpoint se rellena por clave y llega a la búsqueda en sí. Antes de v2.1.223, use el entorno del shell o la configuración de usuario para que el proxy se aplique junto con una carga útil de servidor en caché. El primer lanzamiento no tiene caché, por lo que una fuente administrada por endpoint, el entorno del shell o la configuración de usuario sigue siendo necesaria para la búsqueda inicial.

Claude Code aplica la mayoría de las actualizaciones de configuración a sesiones en ejecución sin reinicio. Algunas actualizaciones se aplican solo en el siguiente lanzamiento, incluida la configuración del exportador OpenTelemetry, la clave `model` y la eliminación de una variable del bloque `env`.

<h3 id="invalid-entries-in-delivered-settings">
  Entradas inválidas en la configuración entregada
</h3>

Cuando parte de una carga útil falla la validación del esquema, Claude Code muestra un error de validación y aplica todas las configuraciones válidas restantes; [Entradas inválidas en configuración administrada](/docs/es/managed-settings#invalid-entries-in-managed-settings) dice qué descarta y qué claves se replantean a un valor más estricto. Requiere Claude Code v2.1.169 o posterior.

La entrega administrada por servidor agrega estos comportamientos:

* El caché en `~/.claude/remote-settings.json` almacena la carga útil salvada con entradas inválidas eliminadas, aparte de valores `cleanupPeriodDays` y `desktopSessionCleanupPeriodDays` inválidos, que permanecen en la copia en caché y nunca se aplican.
* Cuando ningún campo en la carga útil puede salvarse y la carga útil no es solo esas claves de retención, Claude Code rechaza la carga útil, mantiene la última configuración en caché aceptada y escribe `Remote settings: Settings validation failed - no fields could be salvaged` en el registro de depuración. Con `forceRemoteSettingsRefresh` establecido, la CLI sale en su lugar.
* El [diálogo de aprobación de seguridad](#security-approval-dialogs) evalúa la carga útil salvada, por lo que una entrada inválida eliminada nunca se presenta para aprobación y nunca se ejecuta.

Para depurar problemas de entrega, ejecute `claude --debug-file <path>` y busque `Remote settings` en el registro. Valide un cambio de carga útil con `claude doctor` en una máquina de prueba antes de implementarlo en la organización.

<h3 id="enforce-fail-closed-startup">
  Aplicar inicio cerrado por error
</h3>

De forma predeterminada, si la búsqueda de configuración remota falla al inicio, la CLI continúa con la configuración en caché de la última búsqueda exitosa, excepto por los [valores que Claude Code retiene](#fetch-and-caching-behavior) hasta que una búsqueda tenga éxito. En una máquina que nunca los ha obtenido, la CLI continúa sin configuración administrada por servidor y aún aplica cualquier [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) en el dispositivo.

Para evitar que los clientes comiencen con configuración administrada por servidor en caché o ausente, establezca `forceRemoteSettingsRefresh: true` en su configuración administrada.

Los clientes que inician sesión a través de una [puerta de enlace de aplicaciones Claude](#platform-availability) esperan la búsqueda de inicio independientemente de si establece esto o no, y manejan una búsqueda fallida de la siguiente manera:

* Si la puerta de enlace responde a un lanzamiento interactivo asistido con un `401` y esta configuración está desactivada, la puerta de enlace ha terminado ese inicio de sesión. Claude Code imprime [`Cloud gateway session expired — run /login to reconnect.`](/docs/es/errors#cloud-gateway-session-expired) y abre la sesión sin iniciar sesión en la puerta de enlace hasta que el usuario ejecute `/login`.
* Cuando la búsqueda falla de cualquier otra forma, o en cualquier otro tipo de lanzamiento excepto un subcomando `claude auth`, el cliente sale con un error.

Cuando esta configuración está activa en una sesión que obtiene configuración administrada por servidor, la CLI se bloquea al inicio hasta que la configuración remota se obtenga recientemente. Si la búsqueda falla, la CLI sale en lugar de continuar sin la política. Esta configuración se perpetúa a sí misma: una vez entregada desde el servidor, también se almacena en caché localmente para que los inicios posteriores apliquen el mismo comportamiento incluso antes de la primera búsqueda exitosa de una nueva sesión. Una sesión que [no obtiene configuración administrada por servidor](#platform-availability) comienza sin esperar.

Para habilitar esto, agregue la clave a su configuración de configuración administrada:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

También puede establecer esta clave en un perfil MDM [administrado por endpoint](/docs/es/managed-settings#delivery-mechanisms) o archivo `managed-settings.json` del sistema para aplicar comportamiento de cierre por error en el primer lanzamiento, antes de que llegue cualquier carga útil de servidor. Esta bandera es una excepción a la [regla de precedencia](#settings-precedence) anterior: Claude Code la honra cuando cualquier fuente administrada controlada por administrador la establece, incluso si también está presente una carga útil administrada por servidor en caché, por lo que no ignora un valor entregado por MDM cuando existe configuración administrada por servidor.

Cuando un [`policyHelper`](/docs/es/settings-reference#policyhelper) proporciona configuración administrada, su salida reemplaza todas las demás fuentes administradas para las claves que Claude Code lee después del inicio. Para las fuentes de las que Claude Code lee esta clave, consulte [su entrada de configuración](/docs/es/settings-reference#forceremotesettingsrefresh). La entrada `policyHelper` dice de qué fuentes Claude Code lee el asistente y cuándo se ejecuta.

La búsqueda de configuración también envía un encabezado `Cache-Control: no-cache` para que los proxies HTTP intermedios no sirvan una respuesta obsoleta.

Antes de habilitar esta configuración, asegúrese de que sus políticas de red permitan conectividad a `api.anthropic.com`. Si ese punto final es inaccesible, la CLI sale al inicio y los usuarios no pueden iniciar Claude Code.

Los subcomandos `claude auth` como `claude auth login` están exentos de esta comprobación y de la salida de inicio de la puerta de enlace, por lo que los usuarios pueden reautenticarse cuando las credenciales caducadas son la razón por la que falla la búsqueda de configuración.

<h3 id="security-approval-dialogs">
  Diálogos de aprobación de seguridad
</h3>

Cierta configuración que podría plantear riesgos de seguridad requiere aprobación explícita del usuario antes de que Claude Code la aplique en una sesión interactiva:

* **Configuración de comandos de shell**: configuración que ejecuta comandos de shell, como `apiKeyHelper`, `statusLine` y `otelHeadersHelper`
* **Configuración de binarios de sandbox**: `sandbox.bwrapPath`, `sandbox.socatPath` y `sandbox.ripgrep`. Cada una de estas configuraciones apunta a un ejecutable, y Claude Code ejecuta ese ejecutable
* **Configuración de red y aislamiento de sandbox**: configuración de [sandbox](/docs/es/sandboxing) que permite que el proxy de sandbox lea, redirija o autentique tráfico, o que debilite el aislamiento del sandbox: `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets` y `sandbox.network.allowMachLookup`. Un bloque `sandbox.credentials` que contiene solo reglas `deny` no necesita aprobación, ya que restringe el sandbox sin dar al proxy una credencial. Antes de v2.1.251, Claude Code aplicaba estas configuraciones sin aprobación
* **Variables de entorno personalizadas**: variables `env` entregadas que requieren la aprobación del usuario, como variables de proxy y URL base; consulte [Variables de entorno y el diálogo de aprobación](#environment-variables-and-the-approval-dialog)
* **Configuraciones de hooks**: cualquier definición de hook

Cuando estas configuraciones están presentes, los usuarios ven un diálogo de seguridad que explica qué se está configurando. Los usuarios deben aprobar para continuar. Si un usuario rechaza la configuración, Claude Code sale.

Un CLAUDE.md administrado entregado a través de la clave [`claudeMd`](/docs/es/settings-reference#claudemd) no requiere aprobación, porque es texto de instrucción para Claude en lugar de un comando que Claude Code ejecuta. Claude Code aún verifica [permisos](/docs/es/permissions) para las herramientas que Claude usa mientras sigue esas instrucciones. Antes de v2.1.260, un valor `claudeMd` también requería aprobación.

<h4 id="approval-memory">
  Memoria de aprobación
</h4>

Claude Code registra su aprobación en su directorio de configuración, `~/.claude` a menos que establezca [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars). Lo que registra depende de la credencial que usa la búsqueda de configuración:

* **Un inicio de sesión de claude.ai guardado por `/login` o `claude auth login`, o el [inicio de sesión de consola sin clave](/docs/es/authentication#sign-in-without-an-api-key)**: una aprobación por organización, mantenida por la cuenta que aprobó más recientemente.
* **Un inicio de sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway)**: una aprobación por puerta de enlace.

  Si cierra sesión e inicia sesión nuevamente en la misma puerta de enlace, Claude Code no muestra el diálogo nuevamente mientras la configuración que requiere aprobación no cambie. Claude Code lo muestra nuevamente cuando esa configuración cambia, cuando inicia sesión en una puerta de enlace diferente y cuando acepta un nuevo certificado para la misma puerta de enlace.

  Claude Code no guarda aprobación para una puerta de enlace de desarrollo de bucle invertido alcanzada a través de HTTP simple, por lo que el diálogo aparece nuevamente después de cada inicio de sesión.
* **Cualquier otra credencial**, como una clave API u `CLAUDE_CODE_OAUTH_TOKEN`: una aprobación para la configuración entregada, mantenida con la copia en caché de la configuración en ese directorio de configuración. Claude Code muestra el diálogo nuevamente cuando la configuración que requiere aprobación cambia, y después de ejecutar `/logout` o `claude auth logout`, cualquiera de los cuales elimina la copia en caché.

Una aprobación para `sandbox.credentials` o `sandbox.network.tlsTerminate` también cubre las entradas [`sandbox.network.allowedDomains`](/docs/es/settings-reference#sandbox-network-alloweddomains) en esa misma configuración entregada, porque ambas configuraciones actúan en esa lista de permitidos. El diálogo aparece nuevamente cuando su administrador agrega o elimina una de esas entradas, aunque `sandbox.network.allowedDomains` no requiera aprobación por sí sola.

Con un inicio de sesión de claude.ai guardado:

* Si cierra sesión e inicia sesión nuevamente, o cambia a otra organización y luego regresa, Claude Code no muestra el diálogo nuevamente mientras esa configuración no cambie, a menos que otra cuenta las haya aprobado para esa organización en el mismo directorio de configuración en el medio.
* Si inicia sesión en la misma organización con una cuenta diferente, Claude Code muestra el diálogo nuevamente incluso cuando la configuración no cambia. La aprobación de esa cuenta reemplaza la anterior, por lo que cuando cambia, Claude Code muestra el diálogo una vez más.

Claude Code no siempre puede mostrar el diálogo. Cada caso a continuación dice qué configuración se aplica cuando no puede y cuándo ve el diálogo a continuación:

* **Una sesión interactiva que no puede mostrar el diálogo**: Claude Code no aplica la configuración entregada y mantiene la última configuración aprobada. El diálogo aparece en la siguiente sesión que puede mostrarlo. Requiere Claude Code v2.1.211 o posterior.
* **`claude install` o `claude update`**: Claude Code no muestra el diálogo durante ninguno de los comandos. El comando se ejecuta con la última configuración aprobada, y el diálogo aparece en su siguiente sesión interactiva. Si Claude Code espera la búsqueda de configuración al inicio, como con [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) establecido o en una implementación de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), muestra el diálogo durante el comando en su lugar, y una ejecución de instalación desde una tubería falla; consulte [`Raw mode is not supported` durante la instalación](/docs/es/troubleshoot-install#raw-mode-is-not-supported-during-install). Antes de v2.1.246, Claude Code intentaba mostrar el diálogo durante estos comandos también.
* **Un error cierra el diálogo antes de que responda**: Claude Code no aplica la configuración entregada y mantiene la última configuración aprobada. Lo muestra nuevamente en la siguiente sesión que puede mostrarlo.
* **Una ejecución no interactiva**, como `claude -p` o una sesión del SDK del Agente: Claude Code no puede mostrar el diálogo, por lo que cuando la configuración entregada requeriría aprobación, la aplica solo para esa ejecución. No la registra como aprobada ni la escribe en el [caché local](#fetch-and-caching-behavior), y la siguiente sesión interactiva muestra el diálogo. Hasta que un usuario apruebe en una sesión interactiva, cada ejecución no interactiva obtiene la configuración nuevamente al inicio. Antes de v2.1.207, una ejecución no interactiva guardaba la configuración como aprobada, por lo que las sesiones interactivas posteriores nunca mostraban el diálogo para ellas.

<h4 id="environment-variables-and-the-approval-dialog">
  Variables de entorno y el diálogo de aprobación
</h4>

Claude Code aplica algunas variables `env` entregadas sin mostrar al usuario el diálogo de aprobación, incluidas:

* Conmutadores de características y comandos
* Configuración de selección y comportamiento del modelo, como `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING` y `CLAUDE_CODE_EFFORT_LEVEL`
* Configuración de ventana de contexto y compactación, como `DISABLE_AUTO_COMPACT`
* Opciones de interfaz de usuario de terminal y accesibilidad
* Límites numéricos, presupuestos y tiempos de espera

Otras variables entregadas pueden requerir la aprobación del usuario antes de que entren en vigor; un valor de proxy, URL base u `OTEL_EXPORTER_OTLP_ENDPOINT` no vacío siempre lo hace. Cuando una variable entregada necesita aprobación, el diálogo la nombra, por lo que el usuario ve exactamente qué es lo que la política le pide que establezca. Antes de v2.1.218, Claude Code aplicaba menos variables sin preguntar al usuario, por lo que configuraciones como `DISABLE_AUTO_COMPACT` activaban el diálogo en cualquier valor no vacío.

Claude Code decide si cuatro conmutadores de privacidad necesitan aprobación por el valor entregado en lugar de por el nombre de la variable: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY` y `DO_NOT_TRACK`. Un valor verdadero como `1` o `true` solo desactiva el seguimiento, la notificación de errores u otro tráfico no esencial, por lo que Claude Code lo aplica sin preguntar al usuario. Para cualquier otro valor no vacío, Claude Code muestra el diálogo. Antes de v2.1.218, todos excepto `DO_NOT_TRACK` se aplicaban sin aprobación en cualquier valor, y `DO_NOT_TRACK` activaba el diálogo en cualquier valor no vacío.

Claude Code también decide si [`API_FORCE_IDLE_TIMEOUT`](/docs/es/env-vars) necesita aprobación por el valor entregado: un valor verdadero solo activa el [tiempo de espera de inactividad del cuerpo](/docs/es/network-config#streaming-idle-watchdogs), por lo que Claude Code lo aplica sin preguntar al usuario. Para cualquier otro valor no vacío, Claude Code muestra el diálogo. Antes de v2.1.248, cualquier valor no vacío activaba el diálogo.

Si [`ANTHROPIC_CUSTOM_HEADERS`](/docs/es/env-vars#variables) necesita aprobación también depende del valor entregado. Los encabezados que solo etiquetan solicitudes, como `Accept-Language`, se aplican sin el diálogo. Una línea que nombra una credencial, un selector de organización o inquilino, un enrutamiento o anulación de host, o un encabezado de comportamiento de API, como `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta` o los encabezados `X-Amzn-Bedrock-*`, requiere aprobación. Una línea también requiere aprobación cuando su nombre no es un token de encabezado HTTP válido o su valor contiene un carácter que un encabezado HTTP no puede llevar. La comprobación coincide con palabras dentro del nombre del encabezado, por lo que `X-Client-Version`, que contiene `client` y `version`, también requiere aprobación. Antes de v2.1.251, cualquier valor `ANTHROPIC_CUSTOM_HEADERS` se aplicaba sin él.

Un valor falso como `0` o `false` para [`ENABLE_BETA_TRACING_DETAILED`](/docs/es/env-vars#variables) o [`OTEL_LOG_RAW_API_BODIES`](/docs/es/env-vars#variables) se aplica sin el diálogo, porque solo desactiva el rastreo detallado o la captura de cuerpo de API sin procesar. Cualquier otro valor no vacío para cualquiera de las variables requiere aprobación.

<h2 id="platform-availability">
  Disponibilidad de plataforma
</h2>

La configuración administrada por servidor requiere una conexión directa a `api.anthropic.com`. La entrega también requiere que la sesión se autentique con una de estas credenciales:

* Un inicio de sesión OAuth de organización o empresa
* Un token OAuth suministrado a través de `CLAUDE_CODE_OAUTH_TOKEN`
* Una clave API configurada directamente
* Un [perfil de Anthropic](/docs/es/authentication#anthropic-profiles-and-federation-credentials) `user_oauth`, a menos que el perfil establezca una `base_url` diferente a la API de Anthropic. Requiere Claude Code v2.1.257 o posterior.

Ni las claves devueltas por un script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) ni las credenciales de [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) activan la búsqueda de configuración.

En una sesión de [Cowork](https://claude.com/docs/cowork/overview) en la aplicación Claude Desktop, Claude Code no obtiene la configuración administrada por servidor de la consola de administración de claude.ai, incluso cuando el usuario inicia sesión con una cuenta de organización o empresa. [Dónde y cuándo se aplica una política](/docs/es/managed-settings#where-and-when-a-policy-applies) cubre qué política llega a las sesiones de Cowork en la máquina del usuario y las sesiones remotas de Cowork. claude.ai sigue aplicando sus listas [`strictKnownMarketplaces`](/docs/es/settings-reference#strictknownmarketplaces) y [`blockedMarketplaces`](/docs/es/settings-reference#blockedmarketplaces) cuando un usuario de Cowork agrega un marketplace desde un repositorio git en claude.ai o desde **Customize** en la pestaña Cowork. [Cómo funcionan las restricciones](/docs/es/plugins/org#restrict-what-users-can-install) describe esa verificación.

Si exporta una variable de proveedor `CLAUDE_CODE_USE_*` o una `ANTHROPIC_BASE_URL` no predeterminada en su shell, Claude Code omite la búsqueda de configuración para sus sesiones. [`claude doctor` y `/status` informan sobre la búsqueda omitida y su causa](#verify-settings-delivery).

No puede borrar la exportación con un bloque `env` administrado por servidor, porque el bloque llega a través de la búsqueda que la exportación impide. Un bloque `env` de [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) tampoco restaura la búsqueda: Claude Code verifica la elegibilidad antes de aplicar bloques `env` administrados, por lo que el cambio de valor administrado por endpoint cambia la selección de proveedor de la sesión pero la búsqueda permanece omitida.

Para restaurar la entrega administrada por servidor, elimine la exportación de su shell, o establezca la variable en `""` en su bloque `env` de configuración de usuario, que se aplica antes de la verificación de elegibilidad. Para aplicar la política sin depender de que los usuarios cambien sus shells, entregue la configuración a través del canal administrado por endpoint en su lugar.

Para implementaciones de Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y [Claude Platform on AWS](/docs/es/claude-platform-on-aws), una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada proporciona la entrega equivalente de configuración administrada remota: los clientes con sesión iniciada en la puerta de enlace obtienen la configuración administrada de la puerta de enlace en lugar de `api.anthropic.com`. La semántica de fallos difiere al inicio: un cliente de puerta de enlace que no puede alcanzar la puerta de enlace sale con un error en lugar de recurrir a la configuración en caché, mientras que la actualización en segundo plano cada hora es de fallo abierto en ambos canales.

<h2 id="audit-logging">
  Registro de auditoría
</h2>

Los eventos del registro de auditoría para cambios de configuración están disponibles a través de la API de cumplimiento o exportación del registro de auditoría. Póngase en contacto con su equipo de cuenta de Anthropic para obtener acceso.

Los eventos de auditoría incluyen el tipo de acción realizada, la cuenta y el dispositivo que realizó la acción, y referencias a los valores anteriores y nuevos.

<h2 id="security-considerations">
  Consideraciones de seguridad
</h2>

La configuración administrada por servidor proporciona aplicación de políticas centralizada, pero funciona como un control del lado del cliente, no como un límite de seguridad. En dispositivos no administrados, un usuario no necesita acceso de administrador o sudo para omitirla.

| Escenario                                                                         | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :-------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| El usuario edita el archivo de configuración en caché                             | El archivo manipulado se aplica al inicio, excepto para los [valores que Claude Code retiene](#fetch-and-caching-behavior) hasta que el servidor confirma la carga útil. La siguiente obtención del servidor restaura la configuración correcta, excepto para las [claves que se aplican solo en el siguiente lanzamiento](#fetch-and-caching-behavior), como `model` o una variable agregada al bloque `env`, que permanecen en vigor hasta el relanzamiento                                                                                                                                                                                                                                                                                                                                        |
| El usuario elimina el archivo de configuración en caché                           | Ocurre el [comportamiento del primer lanzamiento](#fetch-and-caching-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| El usuario ejecuta un binario de Claude Code modificado                           | Un usuario que puede ejecutar un cliente modificado puede omitir cualquier control del lado del cliente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| El usuario ejecuta una versión anterior de Claude Code                            | Las versiones anteriores a la configuración administrada por servidor no obtienen ni aplican la configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| La API no está disponible                                                         | La configuración en caché se aplica si está disponible, excepto para los [valores que Claude Code retiene](#fetch-and-caching-behavior) hasta que una obtención tenga éxito. Sin una caché, Claude Code no aplica ninguna configuración administrada por servidor hasta la siguiente obtención exitosa y aún aplica cualquier [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) en el dispositivo. Con `forceRemoteSettingsRefresh: true`, la CLI se cierra en lugar de continuar, excepto para [subcomandos `claude auth`](#enforce-fail-closed-startup). Los clientes que iniciaron sesión a través de una [puerta de enlace de aplicaciones Claude](#platform-availability) se cierran al inicio sin esa configuración, con la misma excepción de `claude auth` |
| El usuario se autentica con una organización diferente                            | La configuración no se entrega para cuentas fuera de la organización administrada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| El usuario configura un [proveedor de modelo de terceros](#platform-availability) | La configuración administrada por servidor se omite. Esto incluye establecer `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, o un `ANTHROPIC_BASE_URL` no predeterminado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| El tráfico de red se intercepta o se redirige                                     | La validación de TLS deshabilitada o el tráfico interceptado pueden alterar la configuración que recibe el cliente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

Para registrar ediciones en archivos de configuración local, incluido `managed-settings.json`, use [hooks `ConfigChange`](/docs/es/hooks#configchange). Claude Code no los ejecuta cuando llegan o se actualizan configuraciones administradas por servidor, o cuando cambia un perfil MDM o una política de registro, y un hook no puede bloquear un cambio de `policy_settings`.

Para restringir a qué organizaciones pueden acceder los usuarios con las credenciales que proporciona el cliente, consulte [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) en el Centro de ayuda de Claude. Para garantías de aplicación más sólidas, use [configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms) en dispositivos inscritos en una solución MDM.

<h2 id="see-also">
  Ver también
</h2>

Páginas relacionadas para administrar la configuración de Claude Code:

* [Todos los ajustes](/docs/es/settings-reference): cada clave de configuración
* [Configuración administrada por endpoint](/docs/es/managed-settings#delivery-mechanisms): configuración administrada implementada en dispositivos por TI
* [Autenticación](/docs/es/authentication): configurar el acceso de usuarios a Claude Code
* [Seguridad](/docs/es/security): salvaguardas de seguridad y mejores prácticas
