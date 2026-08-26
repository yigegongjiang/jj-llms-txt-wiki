> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar la configuración administrada por servidor

> Configure Claude Code centralmente para su organización a través de configuración entregada por servidor, sin requerir infraestructura de administración de dispositivos.

La configuración administrada por servidor permite a los propietarios de la organización configurar Claude Code centralmente desde [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) en la consola de claude.ai. Los clientes de Claude Code obtienen automáticamente estas configuraciones cuando los usuarios se autentican con un inicio de sesión OAuth organizacional o una clave API configurada directamente, en plataformas donde se admite la entrega administrada por servidor. Consulte [Disponibilidad de plataforma](#platform-availability).

Este enfoque está diseñado para organizaciones que no tienen infraestructura de administración de dispositivos implementada, o que necesitan administrar configuraciones para usuarios en dispositivos no administrados.

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

Claude Code admite dos enfoques para la configuración centralizada. La configuración administrada por servidor entrega la configuración desde los servidores de Anthropic. La [configuración administrada por endpoint](/docs/es/settings#settings-files) se implementa directamente en dispositivos a través de políticas nativas del sistema operativo (preferencias administradas de macOS, registro de Windows) o archivos de configuración administrados.

| Enfoque                                                                    | Mejor para                                                          | Modelo de seguridad                                                                                                                                   |
| :------------------------------------------------------------------------- | :------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Configuración administrada por servidor**                                | Organizaciones sin MDM, o usuarios en dispositivos no administrados | Configuración entregada desde los servidores de Anthropic en el momento de la autenticación                                                           |
| **[Configuración administrada por endpoint](/docs/es/settings#settings-files)** | Organizaciones con MDM o administración de endpoint                 | Configuración implementada en dispositivos a través de perfiles de configuración MDM, políticas de registro o archivos de configuración administrados |

Si sus dispositivos están inscritos en una solución MDM o de administración de endpoint, la configuración administrada por endpoint proporciona garantías de seguridad más sólidas porque el archivo de configuración puede protegerse de la modificación del usuario a nivel del sistema operativo. La configuración administrada por endpoint no llega a las [sesiones en la nube](/docs/es/model-config#surface-coverage), por lo que las organizaciones que utilizan Claude Code en la web también deben configurar la configuración administrada por servidor.

<h2 id="configure-server-managed-settings">
  Configurar la configuración administrada por servidor
</h2>

<Steps>
  <Step title="Abrir la consola de administración">
    En la consola de claude.ai, vaya a [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Si el enlace lo redirige a una página de Admin Settings diferente en lugar de la página de Claude Code, su cuenta no tiene el rol requerido. Los roles de Admin y otros roles que no sean Owner no pueden ver ni editar la configuración administrada, así que pida a un Owner o Primary Owner en su organización que realice el cambio. Consulte [Control de acceso](#access-control).
  </Step>

  <Step title="Definir su configuración">
    Agregue su configuración como JSON. Todas las [configuraciones disponibles en `settings.json`](/docs/es/settings#available-settings) son compatibles excepto las restringidas a la entrega de políticas a nivel del sistema operativo; consulte [Limitaciones actuales](#current-limitations) para esa lista breve. Esto incluye [hooks](/docs/es/hooks), [variables de entorno](/docs/es/env-vars) y [configuraciones solo administradas](/docs/es/permissions#managed-only-settings) como `allowManagedPermissionRulesOnly`.

    Este ejemplo aplica una lista de denegación de permisos, impide que los usuarios omitan permisos y restringe las reglas de permisos a las definidas en la configuración administrada:

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

    Para configurar el clasificador del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para que sepa qué repositorios, buckets y dominios confía su organización:

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Debido a que los hooks ejecutan comandos de shell, los usuarios ven un [diálogo de aprobación de seguridad](#security-approval-dialogs) antes de que se apliquen. Consulte [Configurar el modo automático](/docs/es/auto-mode-config) para ver cómo las entradas de `autoMode` afectan lo que el clasificador bloquea y advertencias importantes sobre los campos `environment`, `allow`, `soft_deny` y `hard_deny`.
  </Step>

  <Step title="Guardar e implementar">
    Guarde sus cambios. Los clientes de Claude Code reciben la configuración actualizada en su próximo inicio o ciclo de sondeo por hora.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verificar la entrega de configuración
</h3>

Para confirmar que la configuración se está aplicando, pida a un usuario que reinicie Claude Code. Si la configuración incluye configuraciones que activan el [diálogo de aprobación de seguridad](#security-approval-dialogs), el usuario ve un mensaje que describe la configuración administrada al inicio. También puede verificar que las reglas de permisos administrados estén activas haciendo que un usuario ejecute `/permissions` para ver sus reglas de permisos efectivas.

<h3 id="access-control">
  Control de acceso
</h3>

Los siguientes roles pueden administrar la configuración administrada por servidor:

* **Propietario principal**
* **Propietario**

Restrinja el acceso al personal de confianza, ya que los cambios de configuración se aplican a todos los usuarios de la organización.

<h3 id="managed-only-settings">
  Configuraciones solo administradas
</h3>

La mayoría de las [claves de configuración](/docs/es/settings#available-settings) funcionan en cualquier ámbito. Un puñado de claves solo se leen de la configuración administrada y no tienen efecto cuando se colocan en archivos de configuración de usuario o proyecto. Consulte [configuraciones solo administradas](/docs/es/permissions#managed-only-settings) para obtener la lista completa. Cualquier configuración que no esté en esa lista aún puede colocarse en la configuración administrada y tiene la precedencia más alta.

<h3 id="current-limitations">
  Limitaciones actuales
</h3>

La configuración administrada por servidor tiene las siguientes limitaciones:

* La configuración se aplica uniformemente a todos los usuarios de la organización. Las configuraciones por grupo aún no son compatibles.
* Un archivo [`managed-mcp.json`](/docs/es/managed-mcp) no se puede distribuir a través de la configuración administrada por servidor. Entregue las claves de política `allowedMcpServers` y `deniedMcpServers` allí en su lugar.
* Las configuraciones restringidas a fuentes de políticas a nivel del sistema operativo, como `policyHelper` y `wslInheritsWindowsSettings`, no se respetan. Impleméntelas a través de MDM o un archivo `managed-settings.json` del sistema en su lugar.

<h2 id="settings-delivery">
  Entrega de configuración
</h2>

<h3 id="settings-precedence">
  Precedencia de configuración
</h3>

La configuración administrada por servidor y la [configuración administrada por endpoint](/docs/es/settings#settings-files) ocupan el nivel más alto en la [jerarquía de configuración](/docs/es/settings#settings-precedence) de Claude Code. Ningún otro nivel de configuración puede anularlas, incluidos los argumentos de línea de comandos.

Dentro del nivel administrado, una [policyHelper](/docs/es/settings#compute-managed-settings-with-a-policy-helper) configurada tiene prioridad sobre todas las demás fuentes administradas, incluida la configuración administrada por servidor: su salida se convierte en la única configuración administrada para la ejecución.

De lo contrario, Claude Code utiliza la primera fuente que entrega una configuración no vacía. La configuración administrada por servidor se verifica primero, luego la configuración administrada por endpoint. Las fuentes no se fusionan: si la configuración administrada por servidor entrega alguna clave, la configuración administrada por endpoint se ignora. Si la configuración administrada por servidor no entrega nada, se aplica la configuración administrada por endpoint.

Se aplica una excepción: un pequeño conjunto de [claves de bloqueo entre fuentes](/docs/es/settings#settings-precedence), como los bloqueos de lista de permitidos de sandbox, se respeta cuando cualquier fuente administrada controlada por administrador los establece; el nivel de registro HKCU escribible por el usuario se excluye.

Si borra su configuración administrada por servidor en la consola de administración con la intención de volver a una política plist administrada por endpoint o de registro, tenga en cuenta que la [configuración en caché](#fetch-and-caching-behavior) persiste en máquinas cliente hasta la siguiente obtención exitosa. Ejecute `/status` para ver qué fuente administrada está activa.

<h3 id="fetch-and-caching-behavior">
  Comportamiento de obtención y almacenamiento en caché
</h3>

Claude Code obtiene la configuración de los servidores de Anthropic al inicio y sondea actualizaciones cada hora durante sesiones activas.

**Primer lanzamiento sin configuración en caché:**

* Claude Code obtiene la configuración de forma asincrónica
* Si la obtención falla, Claude Code continúa sin configuración administrada
* Hay una breve ventana antes de que se cargue la configuración donde las restricciones aún no se aplican

**Lanzamientos posteriores con configuración en caché:**

* La configuración en caché se aplica inmediatamente al inicio, excepto para las variables de entorno de transporte, enrutamiento y autenticación descritas a continuación
* Claude Code obtiene configuración nueva en segundo plano
* La configuración en caché persiste a través de fallos de red. Las variables retenidas permanecen retenidas hasta que una obtención se realiza correctamente

A partir de v2.1.198, Claude Code retiene tres categorías de variables en el bloque `env` en caché hasta que el servidor confirma la carga útil para la sesión. Esto evita que un valor de proxy, autoridad de certificación, endpoint o credencial en caché redirija, intercepte o reautentique la obtención de configuración que confirma la carga útil. El endurecimiento se aplica solo a la caché de configuración obtenida del servidor: la [configuración administrada por endpoint](/docs/es/settings#settings-files) implementada a través de MDM o `managed-settings.json` no se ve afectada. Las categorías retenidas son:

* Configuración de proxy y TLS, como `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` y las variables de certificado de cliente mTLS `CLAUDE_CODE_CLIENT_CERT` y `CLAUDE_CODE_CLIENT_KEY`
* Enrutamiento de API y selección de proveedor, incluido `ANTHROPIC_BASE_URL`, las variables de selección de proveedor como `CLAUDE_CODE_USE_BEDROCK` y `CLAUDE_CODE_USE_VERTEX`, y las URL de endpoint del proveedor como `ANTHROPIC_BEDROCK_BASE_URL`
* Credenciales de autenticación, como `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` y `CLAUDE_CODE_OAUTH_TOKEN`

Todas las demás claves en el bloque `env` en caché, como la telemetría y la configuración de OpenTelemetry, se aplican al inicio como antes. Una vez que la obtención se realiza correctamente, las variables retenidas se aplican para el resto de la sesión.

Si su organización necesita un proxy para llegar a `api.anthropic.com`, establézcalo en el entorno de shell o en la [configuración del usuario](/docs/es/settings#settings-files) en lugar de solo en el bloque `env` administrado. El primer lanzamiento no tiene caché, por lo que esas fuentes ya eran necesarias para la obtención inicial.

Claude Code aplica actualizaciones de configuración automáticamente sin reinicio, excepto para configuraciones avanzadas como la configuración de OpenTelemetry, que requieren un reinicio completo para tomar efecto.

<h3 id="invalid-entries-in-delivered-settings">
  Entradas inválidas en la configuración entregada
</h3>

Las cargas útiles entregadas se analizan de forma tolerante con las mismas reglas que las otras fuentes administradas. Cuando una carga útil contiene una entrada que falla en la validación del esquema, Claude Code elimina esa entrada, muestra un error de validación y aplica todas las configuraciones válidas restantes. Consulte [Entradas inválidas en configuración administrada](/docs/es/settings#invalid-entries-in-managed-settings) para el comportamiento a nivel de campo, incluida la forma en que se manejan los campos de aplicación de seguridad. Requiere Claude Code v2.1.169 o posterior.

La entrega administrada por servidor agrega estos comportamientos:

* La caché en `~/.claude/remote-settings.json` almacena la carga útil salvada con entradas inválidas eliminadas. La carga útil inválida sin procesar nunca se persiste.
* Cuando ningún campo en la carga útil puede salvarse, Claude Code mantiene la última configuración en caché aceptada y registra un error fatal.
* El [diálogo de aprobación de seguridad](#security-approval-dialogs) evalúa la carga útil salvada, por lo que una entrada inválida eliminada nunca se presenta para aprobación y nunca se ejecuta.

Para depurar problemas de entrega, ejecute `claude --debug-file <path>` y busque en el registro `Remote settings`. Valide un cambio de carga útil con `claude doctor` en una máquina de prueba antes de implementarlo en la organización.

<h3 id="enforce-fail-closed-startup">
  Aplicar inicio cerrado por fallo
</h3>

De forma predeterminada, si la obtención de configuración remota falla al inicio, la CLI continúa sin configuración administrada. Para entornos donde esta breve ventana no aplicada es inaceptable, establezca `forceRemoteSettingsRefresh: true` en su configuración administrada.

Cuando esta configuración está activa, la CLI se bloquea al inicio hasta que la configuración remota se obtiene recientemente. Si la obtención falla, la CLI se cierra en lugar de continuar sin la política. Esta configuración se autoperpetúa: una vez entregada desde el servidor, también se almacena en caché localmente para que los inicios posteriores apliquen el mismo comportamiento incluso antes de la primera obtención exitosa de una nueva sesión.

Para habilitarlo, agregue la clave a su configuración de configuración administrada:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

También puede establecer esta clave en un [perfil MDM administrado por endpoint](/docs/es/settings#settings-files) o archivo `managed-settings.json` del sistema para aplicar el comportamiento de cierre por fallo en el primer lanzamiento, antes de que se entregue ninguna carga útil del servidor. A partir de v2.1.191, esta bandera es una excepción a la [regla de precedencia](#settings-precedence) anterior: se respeta cuando se establece en cualquier fuente administrada incluso si también está presente una carga útil en caché administrada por servidor, por lo que un valor entregado por MDM no se ignora cuando existen configuraciones administradas por servidor.

La obtención de configuración también envía un encabezado `Cache-Control: no-cache` para que los proxies HTTP intermedios no sirvan una respuesta obsoleta.

Antes de habilitar esta configuración, asegúrese de que sus políticas de red permitan la conectividad a `api.anthropic.com`. Si ese endpoint no es accesible, la CLI se cierra al inicio y los usuarios no pueden iniciar Claude Code.

A partir de v2.1.139, los subcomandos `claude auth` como `claude auth login` están exentos de esta verificación, por lo que los usuarios pueden volver a autenticarse cuando las credenciales caducadas son la razón por la que falla la obtención de configuración.

<h3 id="security-approval-dialogs">
  Diálogos de aprobación de seguridad
</h3>

Ciertas configuraciones que podrían presentar riesgos de seguridad requieren aprobación explícita del usuario antes de que Claude Code las aplique:

* **Configuraciones de comandos de shell**: configuraciones que ejecutan comandos de shell
* **Variables de entorno personalizadas**: variables que no están en la lista de permitidos conocida y segura
* **Configuraciones de hooks**: cualquier definición de hook
* **Contenido CLAUDE.md administrado**: un valor `claudeMd` entregado a través de configuración administrada

Cuando estas configuraciones están presentes, los usuarios ven un diálogo de seguridad que explica qué se está configurando. Los usuarios deben aprobar para continuar. Si un usuario rechaza la configuración, Claude Code se cierra.

<Note>
  Una ejecución no interactiva, como `claude -p` o una sesión del SDK de Agent, no puede mostrar el diálogo. Cuando la configuración entregada requeriría aprobación, Claude Code la aplica solo para esa ejecución: no la registra como aprobada ni la escribe en la [caché local](#fetch-and-caching-behavior), y la siguiente sesión interactiva muestra el diálogo. Hasta que un usuario apruebe en una sesión interactiva, cada ejecución no interactiva obtiene la configuración nuevamente al inicio. Antes de v2.1.207, una ejecución no interactiva guardaba la configuración como aprobada, por lo que las sesiones interactivas posteriores nunca mostraban el diálogo para ellas.
</Note>

<h2 id="platform-availability">
  Disponibilidad de plataforma
</h2>

La configuración administrada por servidor requiere una conexión directa a `api.anthropic.com`, y la entrega requiere que la sesión se autentique con un inicio de sesión OAuth de organización o una clave API configurada directamente. Las claves devueltas por un script [`apiKeyHelper`](/docs/es/settings#available-settings) no activan la búsqueda de configuración.

La configuración administrada por servidor no está disponible cuando se utilizan proveedores de modelos de terceros:

* Amazon Bedrock
* Google Cloud's Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/es/claude-platform-on-aws)
* Endpoints de API personalizados a través de `ANTHROPIC_BASE_URL` o [puertas de enlace LLM](/docs/es/llm-gateway) de terceros

Si exporta una variable de proveedor `CLAUDE_CODE_USE_*` o una `ANTHROPIC_BASE_URL` no predeterminada en su shell, Claude Code omite la búsqueda de configuración para sus sesiones. No puede borrar la exportación con un bloque `env` administrado por servidor, porque el bloque llega a través de la búsqueda que la exportación impide. Un bloque `env` de [configuración administrada por endpoint](/docs/es/settings#settings-files) tampoco restaura la búsqueda: Claude Code verifica la elegibilidad antes de aplicar bloques `env` administrados, por lo que la anulación cambia la selección de proveedor de la sesión pero la búsqueda permanece omitida.

Para restaurar la entrega administrada por servidor, elimine la exportación de su shell, o establezca la variable en `""` en su bloque `env` de configuración de usuario, que se aplica antes de la verificación de elegibilidad. Para aplicar la política sin depender de que los usuarios cambien sus shells, entregue la configuración a través del canal administrado por endpoint en su lugar.

Para implementaciones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada proporciona la entrega equivalente de configuración administrada remota: los clientes con sesión iniciada en la puerta de enlace obtienen la configuración administrada de la puerta de enlace en lugar de `api.anthropic.com`. La semántica de fallos difiere al inicio: un cliente de puerta de enlace que no puede alcanzar la puerta de enlace sale con un error en lugar de recurrir a la configuración en caché, mientras que la actualización en segundo plano cada hora es de fallo abierto en ambos canales.

<h2 id="audit-logging">
  Registro de auditoría
</h2>

Los eventos del registro de auditoría para cambios de configuración están disponibles a través de la API de cumplimiento o exportación del registro de auditoría. Póngase en contacto con su equipo de cuenta de Anthropic para obtener acceso.

Los eventos de auditoría incluyen el tipo de acción realizada, la cuenta y el dispositivo que realizó la acción, y referencias a los valores anteriores y nuevos.

<h2 id="security-considerations">
  Consideraciones de seguridad
</h2>

La configuración administrada por servidor proporciona aplicación de políticas centralizada, pero funciona como un control del lado del cliente, no como un límite de seguridad. En dispositivos no administrados, un usuario no necesita acceso de administrador o sudo para omitirla.

| Escenario                                                                         | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :-------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| El usuario edita el archivo de configuración en caché                             | El archivo manipulado se aplica al inicio, pero la configuración correcta se restaura en la siguiente obtención del servidor. A partir de v2.1.198, las variables de entorno de transporte, enrutamiento de API y autenticación en el bloque `env` se [retienen hasta que el servidor confirma la carga útil](#fetch-and-caching-behavior)                                                                                                                                                                                                                        |
| El usuario elimina el archivo de configuración en caché                           | Ocurre el comportamiento del primer lanzamiento: la configuración se obtiene de forma asincrónica con una breve ventana no aplicada                                                                                                                                                                                                                                                                                                                                                                                                                               |
| El usuario ejecuta un binario de Claude Code modificado                           | Un usuario que puede ejecutar un cliente modificado puede omitir cualquier control del lado del cliente                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| El usuario ejecuta una versión anterior de Claude Code                            | Las versiones anteriores a la configuración administrada por servidor no obtienen ni aplican la configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| La API no está disponible                                                         | La configuración en caché se aplica si está disponible, de lo contrario, la configuración administrada no se aplica hasta la siguiente obtención exitosa. A partir de v2.1.198, las variables de entorno de transporte, enrutamiento de API y autenticación en el bloque `env` en caché se [retienen en caso de fallo de obtención](#fetch-and-caching-behavior); el resto de la caché se sigue aplicando. Con `forceRemoteSettingsRefresh: true`, la CLI se cierra en lugar de continuar, excepto para [subcomandos `claude auth`](#enforce-fail-closed-startup) |
| El usuario se autentica con una organización diferente                            | La configuración no se entrega para cuentas fuera de la organización administrada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| El usuario configura un [proveedor de modelo de terceros](#platform-availability) | La configuración administrada por servidor se omite. Esto incluye establecer `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, o un `ANTHROPIC_BASE_URL` no predeterminado                                                                                                                                                                                                                                                                                               |
| El tráfico de red se intercepta o se redirige                                     | La validación de TLS deshabilitada o el tráfico interceptado pueden alterar la configuración que recibe el cliente                                                                                                                                                                                                                                                                                                                                                                                                                                                |

Para detectar cambios de configuración en tiempo de ejecución, use [hooks `ConfigChange`](/docs/es/hooks#configchange) para registrar modificaciones o bloquear cambios no autorizados antes de que surtan efecto.

Para restringir a qué organizaciones pueden acceder los usuarios con las credenciales que proporciona el cliente, consulte [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) en el Centro de ayuda de Claude. Para garantías de aplicación más sólidas, use la [configuración administrada por endpoint](/docs/es/settings#settings-files) en dispositivos inscritos en una solución MDM.

<h2 id="see-also">
  Ver también
</h2>

Páginas relacionadas para administrar la configuración de Claude Code:

* [Settings](/docs/es/settings): referencia de configuración completa que incluye todas las configuraciones disponibles
* [Endpoint-managed settings](/docs/es/settings#settings-files): configuración administrada implementada en dispositivos por TI
* [Authentication](/docs/es/authentication): configurar el acceso de usuarios a Claude Code
* [Security](/docs/es/security): salvaguardas de seguridad y mejores prácticas
