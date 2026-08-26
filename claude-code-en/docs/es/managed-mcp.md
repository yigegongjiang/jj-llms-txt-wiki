> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Controlar el acceso al servidor MCP para su organización

> Restrinja qué servidores MCP pueden agregar o conectar los usuarios con archivos de configuración administrados, listas de permitidos y listas de bloqueados.

De forma predeterminada, cualquiera que ejecute Claude Code puede conectar cualquier [servidor MCP](/docs/es/mcp) que elija. Anthropic revisa los conectores según sus [criterios de listado](https://claude.com/docs/connectors/building/review-criteria) antes de agregarlos al [Directorio de Anthropic](https://claude.ai/directory), pero no realiza auditorías de seguridad ni administra ningún servidor MCP. Como administrador, puede restringir qué servidores se ejecutan en su organización, desde implementar un conjunto fijo aprobado hasta deshabilitar MCP completamente.

Esta página cubre cómo:

* [Elegir un patrón](#choose-a-pattern) que se ajuste a cuánto control necesita
* [Implementar un conjunto de servidor fijo con `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), incluido cómo [deshabilitar MCP completamente](#disable-mcp-entirely)
* [Controlar servidores con listas de permitidos y listas de bloqueados](#policy-based-control-with-allowlists-and-denylists)
* [Informar a los usuarios qué esperar](#how-restrictions-appear-to-users) cuando una restricción bloquea un servidor
* [Monitorear qué servidores usa realmente su organización](#monitor-mcp-usage)

<Note>
  La página [Security](/docs/es/security) cubre el modelo de amenaza de MCP y cómo evaluar un servidor antes de aprobarlo. [Decide what to enforce](/docs/es/admin-setup#decide-what-to-enforce) cubre restricciones de MCP junto con los otros controles administrativos.
</Note>

<h2 id="choose-a-pattern">
  Elegir un patrón
</h2>

Claude Code admite una variedad de niveles de restricción. Cada patrón utiliza uno o ambos de los mecanismos cubiertos a continuación: `managed-mcp.json` para implementar un conjunto fijo, y `allowedMcpServers`/`deniedMcpServers` para filtrar lo que los usuarios configuran.

| Patrón                         | Qué hace                                                                                                  | Configurar                                                                                          |
| :----------------------------- | :-------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| **Deshabilitar MCP**           | Ningún servidor se carga en ningún lugar                                                                  | `managed-mcp.json` con un mapa de servidor vacío                                                    |
| **Implementación fija**        | Cada usuario obtiene los mismos servidores y no puede agregar otros                                       | `managed-mcp.json` con los servidores que desea                                                     |
| **Catálogo aprobado**          | Publique una lista de servidores aprobados; los usuarios agregan los que desean, todo lo demás se bloquea | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                            |
| **Solo servidores de plugins** | Los servidores solo pueden provenir de plugins; los usuarios no pueden agregar los suyos propios          | [`strictPluginOnlyCustomization`](/docs/es/settings#strictpluginonlycustomization) con `mcp` en la lista |
| **Lista de permitidos suave**  | Aplicar una lista de permitidos que los usuarios pueden ampliar en su propia configuración                | `allowedMcpServers` sin `allowManagedMcpServersOnly`                                                |
| **Solo lista de bloqueados**   | Bloquee servidores conocidos como malos, permita todo lo demás                                            | `deniedMcpServers`                                                                                  |
| **Sin restricciones**          | Los usuarios agregan cualquier cosa                                                                       | No implemente ninguna configuración administrada de MCP                                             |

<Note>
  Claude Code no tiene un registro de servidor MCP integrado que los usuarios puedan examinar e instalar. Para el patrón de catálogo aprobado, comparta la lista aprobada y sus comandos `claude mcp add` en algún lugar donde sus usuarios los encuentren, como un wiki interno, o distribuya los servidores como plugins a través de un [marketplace de plugins administrado](/docs/es/plugin-marketplaces#managed-marketplace-restrictions) para que los usuarios puedan examinarlos e instalarlos desde `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Control exclusivo con managed-mcp.json
</h2>

Si implementa un archivo `managed-mcp.json`, Claude Code carga solo los servidores que ese archivo define. Los usuarios no pueden agregar, modificar ni usar ningún otro servidor MCP, incluidos los servidores proporcionados por plugins. El archivo también suprime los conectores de claude.ai a menos que [los permita junto con el conjunto administrado](#allow-claude-ai-connectors-alongside-the-managed-set).

Dos otras configuraciones pueden filtrar aún más el conjunto administrado:

* `allowedMcpServers` y `deniedMcpServers` también se aplican a servidores administrados, por lo que un servidor administrado que no los cumpla no se cargará.
* El propio `deniedMcpServers` de un usuario se fusiona desde su configuración, por lo que los usuarios pueden bloquear un servidor administrado para sí mismos.

Consulte [Cómo se evalúa un servidor](#how-a-server-is-evaluated) para el orden completo de verificaciones.

`managed-mcp.json` es un archivo independiente, por lo que no se puede entregar a través de [configuración administrada por servidor](/docs/es/server-managed-settings). Cualquier proceso que pueda escribir en una ruta del sistema con privilegios de administrador puede implementarlo. A escala, eso suele ser a través de herramientas de administración de dispositivos, como Jamf o un perfil de configuración en macOS, Directiva de grupo o Intune en Windows, o su administración de flota de su elección en Linux. Claude Code busca el archivo en una de estas rutas:

| Plataforma  | Ruta                                                       |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux y WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

El archivo utiliza el mismo formato que un archivo de proyecto [`.mcp.json`](/docs/es/mcp#project-scope):

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  Autenticar con credenciales por usuario
</h3>

Cualquier usuario en la máquina puede leer este archivo, así que no almacene claves API u otras credenciales en bloques `env`. Pase credenciales por usuario con una de estas en su lugar:

* [Expansión `${VAR}`](/docs/es/mcp#environment-variable-expansion-in-mcp-json) para leer secretos del entorno de cada usuario.
* [OAuth o encabezados por usuario](/docs/es/mcp#authenticate-with-remote-mcp-servers) para que cada usuario se autentique como sí mismo.
* [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) para generar credenciales en el momento de la conexión.

<h3 id="validate-the-configuration">
  Validar la configuración
</h3>

Para confirmar que el archivo está en vigor, ejecute dos verificaciones en una máquina administrada:

1. `claude mcp list` muestra solo los servidores en `managed-mcp.json`. Si los propios servidores de un usuario aún aparecen, el archivo no se está leyendo; verifique la ruta y los permisos.
2. `claude mcp add --transport http test https://example.com/mcp` falla con `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. La URL no necesita ser un servidor real, ya que la verificación de política rechaza el comando antes de que se contacte con nada.

<h3 id="disable-mcp-entirely">
  Deshabilitar MCP completamente
</h3>

Implemente un `managed-mcp.json` que contenga un mapa de servidor vacío para bloquear cada servidor MCP:

```json theme={null}
{
  "mcpServers": {}
}
```

Los usuarios no ven servidores MCP en `/mcp`, y `claude mcp add` falla con el error de política empresarial anterior. Los servidores que los usuarios habían configurado previamente dejan de cargarse la próxima vez que inician una sesión, sin advertencia de que la política es la razón.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Permitir conectores de claude.ai junto con el conjunto administrado
</h3>

Implementar `managed-mcp.json` suprime [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) de forma predeterminada, incluidos los conectores que un administrador configuró para la organización en la consola de administración de claude.ai. Para cargar esos conectores junto con los servidores en `managed-mcp.json`, establezca `"allowAllClaudeAiMcps": true` en una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices). Requiere Claude Code v2.1.149 o posterior.

Con la configuración habilitada, Claude Code carga los mismos conectores de claude.ai que cargaría si `managed-mcp.json` no estuviera implementado. [Las listas de permitidos y denegar](#policy-based-control-with-allowlists-and-denylists) aún se aplican a esos conectores, por lo que puede bloquear los específicos con `deniedMcpServers`. La configuración afecta solo a los conectores de claude.ai; los servidores proporcionados por plugins permanecen suprimidos.

Claude Code lee esta configuración solo desde niveles de política controlados por administrador: configuración administrada por servidor, una clave de registro plist o HKLM implementada por MDM, o un archivo `managed-settings.json` del sistema. Colocarla en configuración de usuario o proyecto no tiene efecto, por lo que los usuarios no pueden volver a habilitar los conectores que el control exclusivo suprimió.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Control basado en políticas con listas de permitidos y listas de bloqueados
</h2>

Las listas de permitidos y listas de bloqueados filtran qué servidores configurados pueden cargarse. No son un registro: un servidor aún tiene que ser agregado por un usuario, un plugin o `managed-mcp.json` antes de que la lista de permitidos o la lista de bloqueados se aplique a él. Para implementar servidores a los usuarios, use [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json). Ambas listas también filtran servidores pasados con la bandera CLI [`--mcp-config`](/docs/es/cli-reference#cli-flags); `--strict-mcp-config` limita qué archivos de configuración se cargan y no omite ninguna de las dos listas.

Para hacer que la lista de permitidos sea autoritaria, establezca `allowedMcpServers` y `allowManagedMcpServersOnly: true` juntos en una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices), como configuración administrada por servidor o un archivo `managed-settings.json` implementado. [Restringir la lista de permitidos solo a configuración administrada](#restrict-the-allowlist-to-managed-settings-only) muestra la configuración. Sin `allowManagedMcpServersOnly`, las listas de permitidos de cada fuente de configuración se fusionan, incluido el propio `~/.claude/settings.json` de un usuario, por lo que un usuario puede ampliar lo que su lista de permitidos permite. Las listas de bloqueados se fusionan desde cada fuente independientemente.

<Note>
  `allowManagedMcpServersOnly` es separado de `allowManagedPermissionRulesOnly`, que bloquea solo [reglas de permisos](/docs/es/permissions#managed-settings). Establecer esa bandera no aplica la lista de permitidos de MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Hacer coincidir servidores por URL, comando o nombre
</h3>

`allowedMcpServers` y `deniedMcpServers` son listas de entradas. Cada entrada es un objeto con una sola clave que identifica servidores por su URL, su comando o su nombre:

| Clave           | Coincide                                                                                    | Usar para                                              |
| :-------------- | :------------------------------------------------------------------------------------------ | :----------------------------------------------------- |
| `serverUrl`     | Una URL de servidor remoto, exacta o con comodines `*`                                      | Servidores HTTP y SSE                                  |
| `serverCommand` | El comando exacto y los argumentos que inician un servidor stdio                            | Servidores stdio                                       |
| `serverName`    | La etiqueta asignada por el usuario. Solo coincidencia exacta; los comodines no se expanden | Cualquier tipo, pero vea la Advertencia a continuación |

Dejar `allowedMcpServers` sin establecer es diferente a establecerlo en una matriz vacía:

| Configuración       | Sin establecer (predeterminado) | Matriz vacía `[]`         | Poblada                                 |
| :------------------ | :------------------------------ | :------------------------ | :-------------------------------------- |
| `allowedMcpServers` | Todos los servidores permitidos | Ningún servidor permitido | Solo servidores coincidentes permitidos |
| `deniedMcpServers`  | Ningún servidor bloqueado       | Ningún servidor bloqueado | Servidores coincidentes bloqueados      |

Vea [Entradas inválidas en configuración administrada](/docs/es/settings#invalid-entries-in-managed-settings) para saber qué sucede cuando una entrada falla la validación del esquema.

<Warning>
  Una entrada `serverName`, en cualquiera de las listas, no es un control de seguridad. El nombre es la etiqueta que un usuario asigna al ejecutar `claude mcp add` o editar un archivo de configuración, no el servidor subyacente, por lo que un usuario puede llamar a cualquier servidor `github`. Para conectores de claude.ai, el nombre es el nombre mostrado devuelto por claude.ai, que puede cambiar. Para aplicar qué servidores realmente se ejecutan, agregue entradas `serverCommand` o `serverUrl`.
</Warning>

La validación de `serverName` difiere entre las dos listas:

* En `deniedMcpServers`, `serverName` acepta cualquier cadena no vacía, por lo que puede bloquear [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) por su nombre mostrado. Por ejemplo, `{ "serverName": "claude.ai Slack" }` bloquea el conector de Slack. Prefiera una entrada `serverUrl` cuando necesite que la denegación sea robusta ante cambios de nombre, o cuando un nombre de conector colisiona y gana un sufijo ` (N)`.
* En `allowedMcpServers`, `serverName` se limita a letras, números, guiones e guiones bajos. Use `serverUrl` para permitir un conector de claude.ai.

Para desactivar todos los conectores de claude.ai, vea [`disableClaudeAiConnectors`](/docs/es/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Cómo se evalúa un servidor
</h3>

Antes de cargar un servidor, incluido uno de `managed-mcp.json`, Claude Code ejecuta tres verificaciones en orden:

1. **Fusionar las listas.** Las entradas de lista de permitidos y lista de bloqueados de cada fuente de configuración se combinan en una lista de permitidos y una lista de bloqueados. Cuando `allowManagedMcpServersOnly` es `true`, solo se mantiene la lista de permitidos administrada; la lista de bloqueados siempre se fusiona desde cada fuente.
2. **Verificar la lista de bloqueados.** Un servidor que coincida con cualquier entrada de lista de bloqueados, por URL, comando o nombre, se bloquea. Nada anula una coincidencia de lista de bloqueados.
3. **Verificar la lista de permitidos.** Si `allowedMcpServers` no está establecido en ningún lugar, cada servidor que pasó la lista de bloqueados se carga. Si está establecido, lo que el servidor debe coincidir depende de su tipo, que se muestra en la tabla a continuación.

| Tipo de servidor    | Permitido cuando coincide                                                                                                                 |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Remoto (HTTP o SSE) | Una entrada `serverUrl`. Una coincidencia `serverName` cuenta solo cuando la lista de permitidos no contiene entradas `serverUrl`         |
| Stdio               | Una entrada `serverCommand`. Una coincidencia `serverName` cuenta solo cuando la lista de permitidos no contiene entradas `serverCommand` |

Tres reglas de coincidencia se aplican dentro de esas verificaciones:

* **Los comandos coinciden exactamente.** Cada argumento, en orden. `["npx", "-y", "server"]` no coincide con `["npx", "server"]` o `["npx", "-y", "server", "--flag"]`.
* **Los valores `serverCommand` y `serverUrl` se expanden antes de coincidir.** Tanto la entrada de política como el valor configurado del servidor pasan por la misma [expansión `${VAR}` y `${VAR:-default}`](/docs/es/mcp#environment-variable-expansion-in-mcp-json) que `.mcp.json`, por lo que una entrada escrita como `["${HOME}/bin/server"]` coincide con una configuración de servidor que usa la misma referencia o la ruta expandida. En Windows, haga referencia a una variable de entorno que esté configurada allí, como `${USERPROFILE}` en lugar de `${HOME}`. Los valores `serverName` coinciden literalmente y nunca se expanden.
* **Las URLs admiten comodines `*`** en cualquier lugar del patrón, incluido el esquema. La coincidencia del nombre de host no distingue mayúsculas de minúsculas e ignora un punto FQDN final, por lo que `https://Mcp.Example.com/*` coincide con `https://mcp.example.com/api`. Las rutas permanecen sensibles a mayúsculas y minúsculas.

| Patrón                      | Permite                                                                                |
| :-------------------------- | :------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Todas las rutas en un dominio específico                                               |
| `https://mcp.example.com`   | También todas las rutas en ese dominio. Un patrón sin ruta coincide con cualquier ruta |
| `https://*.example.com/*`   | Cualquier subdominio de `example.com`                                                  |
| `http://localhost:*/*`      | Cualquier puerto en localhost                                                          |
| `*://mcp.example.com/*`     | Cualquier esquema a un dominio específico                                              |

Debido a que la expansión `${VAR}` lee el entorno de proceso propio de Claude Code, una entrada de política `serverCommand` o `serverUrl` que haga referencia a una variable se expande a cualquier valor que establezca un usuario. Use URLs y comandos literales para entradas en las que confía para la aplicación.

<h3 id="example-configuration">
  Configuración de ejemplo
</h3>

La configuración a continuación configura una lista de permitidos dura con una lista de bloqueados. Las líneas resaltadas cambian cómo se evalúa el resto de la lista, y las llamadas después del bloque explican cada una:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Línea 3**: la primera entrada `serverUrl`. Una vez que existe una, cada servidor remoto debe coincidir con un patrón de URL, por lo que un usuario no puede obtener un servidor remoto no listado dándole un nombre permitido.
* **Línea 5**: la primera entrada `serverCommand`. El mismo efecto para servidores stdio, por lo que cada servidor local debe coincidir exactamente con un comando listado.
* **Línea 11**: una entrada `serverName` en la lista de bloqueados. Las entradas de lista de bloqueados siempre se aplican, por lo que cualquier servidor llamado `dangerous-server` se bloquea independientemente de su URL o comando.

Una entrada `serverName` en esta lista de permitidos nunca coincidiría con nada, ya que ambos tipos de transporte ya tienen entradas más estrictas.

Los acordeones a continuación recorren cómo se evalúa un servidor contra otras combinaciones de lista de permitidos y lista de bloqueados.

<Accordion title="Lista de permitidos solo URL">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Servidor                                                | Resultado                                                  |
  | :------------------------------------------------------ | :--------------------------------------------------------- |
  | Servidor HTTP en `https://mcp.example.com/api`          | Permitido: coincide con patrón de URL                      |
  | Servidor HTTP en `https://api.internal.example.com/mcp` | Permitido: coincide con subdominio comodín                 |
  | Servidor HTTP en `https://external.example.com/mcp`     | Bloqueado: no coincide con ningún patrón de URL            |
  | Servidor stdio con cualquier comando                    | Bloqueado: sin entradas de nombre o comando para coincidir |
</Accordion>

<Accordion title="Lista de permitidos solo comando">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Servidor                                               | Resultado                                        |
  | :----------------------------------------------------- | :----------------------------------------------- |
  | Servidor stdio con `["npx", "-y", "approved-package"]` | Permitido: coincide con comando                  |
  | Servidor stdio con `["node", "server.js"]`             | Bloqueado: no coincide con comando               |
  | Servidor HTTP llamado `my-api`                         | Bloqueado: sin entradas de nombre para coincidir |
</Accordion>

<Accordion title="Lista de permitidos mixta de nombre y comando">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Servidor                                                                    | Resultado                                                                                       |
  | :-------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
  | Servidor stdio llamado `local-tool` con `["npx", "-y", "approved-package"]` | Permitido: coincide con comando                                                                 |
  | Servidor stdio llamado `local-tool` con `["node", "server.js"]`             | Bloqueado: existen entradas de comando pero no coincide                                         |
  | Servidor stdio llamado `github` con `["node", "server.js"]`                 | Bloqueado: los servidores stdio deben coincidir con comandos cuando existen entradas de comando |
  | Servidor HTTP llamado `github`                                              | Permitido: coincide con nombre                                                                  |
  | Servidor HTTP llamado `other-api`                                           | Bloqueado: el nombre no coincide                                                                |
</Accordion>

<Accordion title="Lista de permitidos solo nombre">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Servidor                                                     | Resultado                               |
  | :----------------------------------------------------------- | :-------------------------------------- |
  | Servidor stdio llamado `github` con cualquier comando        | Permitido: sin restricciones de comando |
  | Servidor stdio llamado `internal-tool` con cualquier comando | Permitido: sin restricciones de comando |
  | Servidor HTTP llamado `github`                               | Permitido: coincide con nombre          |
  | Cualquier servidor llamado `other`                           | Bloqueado: el nombre no coincide        |
</Accordion>

<Accordion title="Lista de permitidos con anulación de lista de bloqueados">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Servidor                                           | Resultado                                                                                             |
  | :------------------------------------------------- | :---------------------------------------------------------------------------------------------------- |
  | Servidor HTTP en `https://mcp.example.com/api`     | Permitido: coincide con patrón de URL de lista de permitidos, sin coincidencia de lista de bloqueados |
  | Servidor HTTP en `https://staging.example.com/api` | Bloqueado: coincide con ambos, pero la lista de bloqueados tiene prioridad                            |
  | Servidor HTTP en `https://other.com/mcp`           | Bloqueado: no coincide con la lista de permitidos                                                     |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Restringir la lista de permitidos solo a configuración administrada
</h3>

Para hacer que la lista de permitidos administrada sea la única que se aplique, establezca `allowManagedMcpServersOnly` en el archivo de configuración administrada:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Cuando `allowManagedMcpServersOnly` es `true`, las listas de permitidos de configuración de usuario, proyecto y local se ignoran. La lista de bloqueados aún se fusiona desde todas las fuentes, por lo que los usuarios siempre pueden bloquear servidores para sí mismos.

<h2 id="how-restrictions-appear-to-users">
  Cómo aparecen las restricciones a los usuarios
</h2>

Cuando una restricción bloquea un servidor, el usuario ve un error de `claude mcp add` o el servidor deja de cargarse silenciosamente. Use esta tabla para reconocer esos informes y para informar a los usuarios qué esperar antes de implementar un cambio:

| Restricción                                                                         | Lo que ve el usuario                                                                                       |
| :---------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` está presente y el usuario ejecuta `claude mcp add`              | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| El servidor está en una lista de bloqueados y el usuario ejecuta `claude mcp add`   | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| El servidor no está en la lista de permitidos y el usuario ejecuta `claude mcp add` | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| Un servidor configurado previamente ahora está bloqueado por política               | El servidor desaparece silenciosamente de `/mcp` y `claude mcp list` sin advertencia                       |

En el último caso, el usuario no recibe ninguna señal de que la política es la razón por la que su servidor desapareció, así que informe a los usuarios afectados qué servidores se bloquean cuando implemente una nueva restricción.

<h2 id="monitor-mcp-usage">
  Monitorear el uso de MCP
</h2>

Cuando se configura [exportación de OpenTelemetry](/docs/es/monitoring-usage), Claude Code puede registrar qué servidores MCP y herramientas invocan los usuarios. Establezca `OTEL_LOG_TOOL_DETAILS=1` para incluir nombres de servidor MCP y herramientas en eventos de herramientas, luego agréguelos en su recopilador para ver qué servidores conectan realmente sus usuarios. Consulte [Monitoring](/docs/es/monitoring-usage) para configurar el exportador y para el esquema de evento completo.

<h2 id="configuration-summary">
  Resumen de configuración
</h2>

Cada archivo y configuración que cubre esta página, qué controla y cómo entregarlo:

| Superficie                   | Qué controla                                                                       | Dónde vive                                                                                                                                                           | Cómo entregar                                                                                                                                                                                            |
| :--------------------------- | :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Conjunto de servidor fijo, control exclusivo                                       | Ruta del sistema: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, o `C:\Program Files\ClaudeCode\`                                                  | MDM, GPO, administración de flota, o cualquier proceso con privilegios de administrador. No se puede establecer a través de configuración administrada por servidor                                      |
| `allowedMcpServers`          | Lista de permitidos de servidores permitidos                                       | Cualquier [archivo de configuración](/docs/es/settings#settings-files); las entradas de cada fuente se fusionan a menos que `allowManagedMcpServersOnly` esté establecido | Para aplicación, una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices): configuración administrada por servidor, `managed-settings.json`, perfil MDM, o registro |
| `deniedMcpServers`           | Lista de bloqueados de servidores bloqueados                                       | Cualquier archivo de configuración; las entradas de cada fuente se fusionan                                                                                          | Igual que `allowedMcpServers`                                                                                                                                                                            |
| `allowManagedMcpServersOnly` | Bloquea la lista de permitidos solo a fuentes administradas                        | Solo fuentes de configuración administrada; la configuración no tiene efecto en otros lugares                                                                        | Igual que `allowedMcpServers`                                                                                                                                                                            |
| `allowAllClaudeAiMcps`       | Carga conectores de claude.ai junto con `managed-mcp.json` en lugar de suprimirlos | Solo fuentes de configuración administrada; la configuración no tiene efecto en otros lugares                                                                        | Igual que `allowedMcpServers`                                                                                                                                                                            |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Decide what to enforce](/docs/es/admin-setup#decide-what-to-enforce): restricciones de MCP junto con reglas de permisos, sandboxing y los otros controles de administrador
* [Connect Claude Code to tools via MCP](/docs/es/mcp): la referencia completa de MCP, incluidos transportes, alcances y autenticación
* [Settings](/docs/es/settings): la jerarquía de configuración y cómo la configuración administrada tiene prioridad
* [Server-managed settings](/docs/es/server-managed-settings): entregar `allowedMcpServers` y `deniedMcpServers` desde la consola de administrador de Claude.ai
* [Security](/docs/es/security): el modelo de amenaza que estos controles defienden
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, administración de asientos y guía de implementación
