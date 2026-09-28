> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Controlar el acceso a servidores MCP para su organización

> Restrinja qué servidores MCP pueden agregar o conectar los usuarios, o proporcione servidores a todos los usuarios, con archivos de configuración administrados, configuración administrada, listas de permitidos y listas de denegados.

De forma predeterminada, cualquiera que ejecute Claude Code puede conectar cualquier [servidor MCP](/docs/es/mcp) que elija. Anthropic revisa los conectores según sus [criterios de listado](https://claude.com/docs/connectors/building/review-criteria) antes de agregarlos al [Directorio de Anthropic](https://claude.ai/directory), pero no realiza auditorías de seguridad ni administra ningún servidor MCP. Como administrador, puede restringir qué servidores se ejecutan en su organización, desde implementar un conjunto fijo aprobado hasta deshabilitar MCP completamente, y puede proporcionar servidores a todos los usuarios.

Estas restricciones cubren los servidores que Claude Code carga por sí mismo, incluidos los conectores que obtiene de claude.ai. Los conectores que la aplicación de escritorio entrega a sus sesiones locales y SSH llegan en proceso y se rigen desde la configuración de su organización en claude.ai; [Cómo los conectores llegan a Claude Code](/docs/es/mcp#how-connectors-reach-claude-code) muestra qué controles se aplican a los conectores en cada tipo de sesión, incluidas las sesiones en la nube.

Esta página cubre cómo:

* [Elegir un patrón](#choose-a-pattern) que se ajuste a cuánto control necesita
* [Implementar un conjunto de servidores fijo con `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), incluido cómo [deshabilitar MCP completamente](#disable-mcp-entirely)
* [Proporcionar servidores a través de configuración administrada](#provide-servers-through-managed-settings) mientras los usuarios mantienen los suyos
* [Controlar servidores con listas de permitidos y listas de denegados](#policy-based-control-with-allowlists-and-denylists)
* [Informar a los usuarios qué esperar](#how-restrictions-appear-to-users) cuando una restricción bloquea un servidor
* [Monitorear qué servidores usa realmente su organización](#monitor-mcp-usage)

<Note>
  La página [Security](/docs/es/security) cubre el modelo de amenaza de MCP y cómo evaluar un servidor antes de aprobarlo. [Decide what to enforce](/docs/es/admin-setup#decide-what-to-enforce) cubre las restricciones de MCP junto con los otros controles administrativos.
</Note>

<h2 id="choose-a-pattern">
  Elegir un patrón
</h2>

Claude Code admite una variedad de niveles de restricción. Cada patrón utiliza uno o más de los mecanismos cubiertos a continuación: `managed-mcp.json` para implementar un conjunto fijo, la configuración administrada `managedMcpServers` para proporcionar servidores junto con los que agregan los usuarios, y `allowedMcpServers`/`deniedMcpServers` para filtrar lo que los usuarios configuran.

| Patrón                         | Qué hace                                                                                                                                                                                                                                                       | Configurar                                                                                                    |
| :----------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| **Deshabilitar MCP**           | No se cargan servidores, aparte de [servidores en proceso que la aplicación que inició la sesión registra](#exclusive-control-with-managed-mcp-json) y cualquiera que [proporcione a través de `managedMcpServers`](#provide-servers-through-managed-settings) | `managed-mcp.json` con un mapa de servidores vacío                                                            |
| **Implementación fija**        | Cada usuario obtiene los mismos servidores y no puede agregar otros                                                                                                                                                                                            | `managed-mcp.json` con los servidores que desea                                                               |
| **Servidores proporcionados**  | Cada usuario obtiene los servidores remotos que enumera y mantiene los suyos propios                                                                                                                                                                           | `managedMcpServers` en configuración administrada                                                             |
| **Catálogo aprobado**          | Publique una lista de servidores aprobados; los usuarios agregan los que desean, todo lo demás se bloquea                                                                                                                                                      | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                      |
| **Solo servidores de plugins** | Los usuarios no pueden agregar servidores a través de `~/.claude.json` o `.mcp.json`; los servidores de plugins aún se cargan                                                                                                                                  | [`strictPluginOnlyCustomization`](/docs/es/settings-reference#strictpluginonlycustomization) con `mcp` en la lista |
| **Lista de permitidos suave**  | Aplicar una lista de permitidos que los usuarios pueden ampliar en su propia configuración                                                                                                                                                                     | `allowedMcpServers` sin `allowManagedMcpServersOnly`                                                          |
| **Solo lista de denegados**    | Bloquear servidores conocidos como malos, permitir todo lo demás                                                                                                                                                                                               | `deniedMcpServers`                                                                                            |
| **Sin restricciones**          | Los usuarios agregan cualquier cosa                                                                                                                                                                                                                            | No implemente ninguna configuración MCP administrada                                                          |

<Note>
  Claude Code no tiene un registro de servidores MCP integrado que los usuarios puedan examinar e instalar. Para el patrón de catálogo aprobado, comparta la lista aprobada y sus comandos `claude mcp add` en algún lugar donde sus usuarios los encuentren, como un wiki interno, o distribuya los servidores como plugins a través de un [mercado de plugins administrado](/docs/es/plugins/org#restrict-what-users-can-install) para que los usuarios puedan examinarlos e instalarlos desde `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Control exclusivo con managed-mcp.json
</h2>

Cuando implementa un archivo `managed-mcp.json`, Claude Code carga solo estos servidores MCP:

* Los servidores que el archivo define
* Servidores que [proporciona a través de `managedMcpServers`](#provide-servers-through-managed-settings)
* Servidores en proceso que la aplicación que inició la sesión registra, como el servidor propio de la extensión de VS Code o los [conectores que entrega la aplicación de escritorio](/docs/es/mcp#how-connectors-reach-claude-code)

Los usuarios no pueden agregar, modificar ni usar ningún otro servidor MCP, incluidos los servidores proporcionados por plugins y los servidores pasados con la [bandera CLI `--mcp-config`](/docs/es/cli-reference#cli-flags). El archivo también suprime los conectores de claude.ai que Claude Code obtiene por sí mismo a menos que [los permita junto con el conjunto administrado](#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="deploy-managed-mcp-json">
  Implementar managed-mcp.json
</h3>

`managed-mcp.json` es un archivo independiente, por lo que no se puede entregar a través de [configuración administrada por servidor](/docs/es/server-managed-settings). Para entregar servidores a través de configuración administrada en su lugar, sin control exclusivo, use [`managedMcpServers`](#provide-servers-through-managed-settings).

Cualquier proceso que pueda escribir en una ruta del sistema con privilegios de administrador puede implementar el archivo. En toda una flota, eso suele ser a través de herramientas de administración de dispositivos, como Jamf o un perfil de configuración en macOS, Directiva de grupo o Intune en Windows, o su administración de flota preferida en Linux. Claude Code busca el archivo en una de estas rutas:

| Plataforma  | Ruta                                                       |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux y WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

El archivo utiliza el mismo formato que un archivo [`.mcp.json`](/docs/es/mcp#project-scope) de proyecto:

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

Cualquier usuario en la máquina puede leer este archivo, así que no almacene claves API u otras credenciales en bloques `env`. Pase credenciales por usuario con una de estas opciones en su lugar:

* [Expansión `${VAR}`](/docs/es/mcp#environment-variable-expansion-in-mcp-json) para leer secretos del entorno de cada usuario.
* [OAuth o encabezados por usuario](/docs/es/mcp#authenticate-with-remote-mcp-servers) para que cada usuario se autentique como sí mismo.
* [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) para generar credenciales en el momento de la conexión.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  Servidores pasados con `--mcp-config` o `--strict-mcp-config`
</h3>

Cuando una sesión recibe servidores a través de `--mcp-config` mientras un `managed-mcp.json` que Claude Code puede leer y analizar está implementado, lo que el usuario ve difiere entre una estación de trabajo y una sesión en la nube:

* En una estación de trabajo, Claude Code sale al inicio con `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* En [sesiones en la nube](/docs/es/claude-code-on-the-web) en un host donde el archivo está implementado, como un [ejecutor autohospedado](/docs/es/self-hosted-environments-configuration#mcp-servers), Claude Code se inicia solo con los servidores administrados y omite los conectores de claude.ai y otros servidores que el host en la nube entrega a través de `--mcp-config`. Nada en la sesión le dice al usuario qué servidores se dejaron fuera. Claude Code los nombra en una advertencia en su stderr, que un ejecutor autohospedado registra en el nivel de registro `debug`.

La bandera `--strict-mcp-config` solicita reemplazar el conjunto administrado. Si un usuario la pasa mientras tal archivo está implementado, Claude Code sale al inicio en una estación de trabajo y en una sesión en la nube por igual.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  Cómo se aplican las listas de permitidos y denegar al conjunto administrado
</h3>

La lista de denegación puede filtrar aún más los servidores en `managed-mcp.json`:

* `deniedMcpServers` también se aplica a los servidores administrados, por lo que un servidor administrado que coincida con una entrada no se cargará.
* El propio `deniedMcpServers` de un usuario se fusiona desde su configuración, por lo que los usuarios pueden bloquear un servidor administrado para sí mismos.

`allowedMcpServers` no se aplica a los servidores en `managed-mcp.json`, con una excepción: Claude Code aún verifica un servidor cuya definición utiliza [expansión `${VAR}`](/docs/es/mcp#environment-variable-expansion-in-mcp-json) contra la lista de permitidos, porque la configuración efectiva de ese servidor proviene del entorno de cada usuario en lugar de solo del archivo. Antes de v2.1.259, cada servidor administrado tenía que pasar la lista de permitidos siempre que se estableciera una. Consulte [Cómo se evalúa un servidor](#how-a-server-is-evaluated) para ver qué campos activan la verificación `${VAR}` y el orden completo de verificaciones.

Si utilizó `allowedMcpServers` para evitar que algunos de sus propios servidores `managed-mcp.json` se cargaran, esos servidores comienzan a cargarse en el primer lanzamiento de cada usuario de v2.1.259 o posterior a menos que utilicen expansión `${VAR}`, sin aviso ni notificación: solo `deniedMcpServers` aún se resta de esos servidores. Agregue entradas de lista de denegación para ellos, o implemente un `managed-mcp.json` separado por grupo, antes de que sus usuarios actualicen.

<h3 id="validate-the-configuration">
  Validar la configuración
</h3>

Para confirmar que el archivo está en vigor, ejecute dos verificaciones en una máquina administrada:

1. `claude mcp list` muestra solo los servidores en `managed-mcp.json`, más cualquiera que proporcione a través de `managedMcpServers`. Dos otros resultados significan que algo está mal:
   * Si los propios servidores de un usuario aún aparecen, Claude Code no está leyendo el archivo, así que verifique su ruta y los permisos en sus directorios principales.
   * Si los servidores del archivo no aparecen y la sección `MCP config diagnostics` marca la configuración empresarial como error al analizar, Claude Code no puede leer ni analizar el archivo. Corrija el error que esa sección nombra, luego haga que el usuario reinicie Claude Code.
2. `claude mcp add --transport http test https://example.com/mcp` falla con `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. La URL no necesita ser un servidor real, ya que la verificación de política rechaza el comando antes de que se contacte con nada.

<h3 id="disable-mcp-entirely">
  Deshabilitar MCP completamente
</h3>

Implemente un `managed-mcp.json` que contenga un mapa de servidor vacío para bloquear cada servidor MCP aparte de [servidores en proceso que la aplicación que inició la sesión registra](#exclusive-control-with-managed-mcp-json):

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` falla con el error de política empresarial anterior. Los servidores que los usuarios habían configurado previamente dejan de cargarse la próxima vez que inician una sesión, sin advertencia de que la política es la razón. Los servidores que proporciona a través de `managedMcpServers` aún se cargan bajo un mapa vacío, así que deje esa clave sin establecer también para deshabilitar MCP completamente.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Permitir conectores de claude.ai junto con el conjunto administrado
</h3>

De forma predeterminada, implementar `managed-mcp.json` suprime los [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) que Claude Code obtiene por sí mismo, incluidos los conectores que un administrador configuró para la organización en la consola de administración de claude.ai. Para cargar esos conectores junto con los servidores en `managed-mcp.json`, establezca `"allowAllClaudeAiMcps": true` en una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices).

Con la configuración habilitada, Claude Code carga los mismos conectores de claude.ai que cargaría si `managed-mcp.json` no estuviera implementado. [Las listas de permitidos y denegar](#policy-based-control-with-allowlists-and-denylists) aún se aplican a esos conectores, por lo que puede bloquear específicos con `deniedMcpServers`. La configuración afecta solo a los conectores de claude.ai que Claude Code obtiene por sí mismo; los servidores proporcionados por plugins permanecen suprimidos.

Las sesiones en la nube y las sesiones locales y SSH de la aplicación de escritorio reciben conectores de otra manera, descrita en [Cómo los conectores llegan a Claude Code](/docs/es/mcp#how-connectors-reach-claude-code). Un `managed-mcp.json` en el host que ejecuta una sesión en la nube, como un [host de ejecutor autohospedado](/docs/es/self-hosted-environments-configuration#mcp-servers), suprime los conectores de esa sesión independientemente de si establece `allowAllClaudeAiMcps`. Ningún `managed-mcp.json` llega a los conectores que la aplicación de escritorio entrega a sus sesiones locales y SSH.

Claude Code lee `allowAllClaudeAiMcps` solo desde niveles de política controlados por administrador: configuración administrada por servidor, una clave de registro plist o HKLM implementada por MDM, o un archivo `managed-settings.json` del sistema. Colocarlo en configuración de usuario o proyecto no tiene efecto, por lo que los usuarios no pueden volver a habilitar conectores que el control exclusivo suprimió.

<h2 id="provide-servers-through-managed-settings">
  Proporcionar servidores a través de configuración administrada
</h2>

Para proporcionar a cada usuario un conjunto de servidores MCP remotos sin tomar control exclusivo de MCP, enumérelos bajo `managedMcpServers` en una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices): configuración administrada por servidor, una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway-config#what-goes-in-cli), un perfil MDM o política de registro, o `managed-settings.json`. Los usuarios conservan los servidores que agregan ellos mismos y reciben los suyos además. Requiere Claude Code v2.1.259 o posterior. Los clientes anteriores ignoran la clave.

El valor es un objeto con clave por nombre de servidor. Cada entrada tiene la misma forma que un servidor HTTP o SSE en un archivo de proyecto [`.mcp.json`](/docs/es/mcp#project-scope), incluidos los miembros opcionales `headers` y `oauth` descritos en [Autenticarse con servidores MCP remotos](/docs/es/mcp#authenticate-with-remote-mcp-servers). Este ejemplo proporciona un servidor de búsqueda en el que cada usuario inicia sesión con OAuth, y un servidor de registros que envía un encabezado que emite su organización:

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

Cualquiera que pueda leer la configuración administrada en una máquina, incluido el usuario, puede leer un valor de encabezado que establezca aquí. Use una credencial emitida para toda esa audiencia, o deje `headers` fuera y permita que cada usuario inicie sesión con OAuth.

<h3 id="what-an-entry-can-contain">
  Qué puede contener una entrada
</h3>

Claude Code carga una entrada solo cuando pasa cada verificación a continuación. Descarta una entrada que falla una, registra un aviso que puede leer con `/status`, y aún carga las otras entradas:

* `type` es `http` o `sse`. Como en `.mcp.json`, `streamable-http` se acepta como alias para `http`.
* `url` es una URL `https://`. Claude Code rechaza una URL `http://` simple, incluida una que apunte a `localhost`.
* La entrada no tiene miembro `command`, `args`, `env` o `headersHelper`, por lo que un documento de configuración administrada nunca nombra un programa para ejecutar en la máquina de un usuario.
* Ningún valor contiene una referencia `${VAR}`. Claude Code no expande variables de entorno en estas entradas, así que escriba valores literales.
* El nombre del servidor contiene solo letras, números, guiones e guiones bajos, y ninguna clave o valor contiene caracteres de control o formato invisible.

Claude Desktop tiene una configuración administrada con el mismo nombre cuyo valor es una matriz de una forma de entrada diferente, así que no copie una en la otra. Claude Code no acepta la forma de matriz y registra un aviso en lugar de cargarla.

Una puerta de enlace de aplicaciones Claude ejecuta las mismas verificaciones cuando arranca; consulte [Servidores MCP en una política](/docs/es/claude-apps-gateway-config#mcp-servers-in-a-policy).

<h3 id="how-provided-servers-load">
  Cómo se cargan los servidores proporcionados
</h3>

Estas reglas deciden qué se carga cuando un servidor proporcionado se superpone con otra definición de servidor o con otra configuración en esta página:

* Un servidor proporcionado tiene precedencia sobre un servidor con el mismo nombre en alcance local, de proyecto o de usuario, y sobre un servidor de plugin o conector de claude.ai que apunta a la misma URL.
* Si también implementa `managed-mcp.json`, Claude Code carga sus servidores y los servidores proporcionados juntos, y la entrada del archivo tiene precedencia cuando ambos definen un nombre.
* Los servidores proporcionados siguen cargándose cuando [`strictPluginOnlyCustomization`](/docs/es/settings-reference#strictpluginonlycustomization) bloquea la superficie `mcp`.
* `deniedMcpServers` se aplica a los servidores proporcionados, incluidas las entradas de la configuración propia del usuario, por lo que un usuario puede bloquear uno para sí mismo. Los servidores proporcionados no necesitan una entrada `allowedMcpServers`.

Cuando no haya implementado también `managed-mcp.json`, las banderas por ejecución mantienen su significado:

* Un servidor que un usuario pasa con `--mcp-config` bajo el mismo nombre reemplaza el proporcionado para esa ejecución y se verifica contra `allowedMcpServers`.
* `--strict-mcp-config` deja fuera los servidores proporcionados junto con todos los demás servidores configurados.

Con `managed-mcp.json` implementado, ambas banderas se comportan como [Control exclusivo con managed-mcp.json](#exclusive-control-with-managed-mcp-json) describe.

<h3 id="what-users-can-see-and-change">
  Qué pueden ver y cambiar los usuarios
</h3>

Los usuarios no pueden editar ni eliminar un servidor proporcionado:

* `claude mcp remove` informa que el servidor es proporcionado por la organización.
* Cuando no haya implementado también `managed-mcp.json`, una entrada que un usuario agrega bajo el mismo nombre se guarda pero no se usa mientras la suya esté presente.
* Los usuarios aún pueden desactivar un servidor proporcionado para sí mismos en [`/mcp`](/docs/es/mcp#disable-a-server-without-removing-it), que enumera los servidores proporcionados bajo **Managed MCPs**.

`claude mcp get` y `/mcp` muestran la URL de un servidor proporcionado solo como su host, por ejemplo `https://mcp.example.com/…`, y `claude mcp get` muestra sus nombres de encabezado sin sus valores.

<h3 id="where-managedmcpservers-applies">
  Dónde se aplica `managedMcpServers`
</h3>

Claude Code lee `managedMcpServers` de la fuente administrada que selecciona bajo [Cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources). Cuando esa fuente establece [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) en `"merge"`, Claude Code proporciona los servidores de cada fuente de administrador en su lugar, y cuando dos fuentes definen el mismo nombre, la entrada de la fuente de rango superior se aplica completamente. Nunca lee la clave del registro HKCU escribible por el usuario, de [configuración principal que un host de incrustación proporciona](/docs/es/managed-settings#parent-settings-from-embedding-hosts), o de archivos de configuración de usuario, proyecto o local, donde descarta la clave con una advertencia.

Claude Code no lee la clave en la pestaña Código de la aplicación Claude Desktop en una implementación de terceros o en las sesiones de Cowork de la aplicación, porque Claude Desktop proporciona y bloquea los servidores MCP de esas sesiones. `/status` y `claude doctor` lo dicen cuando su configuración administrada lleva la clave allí.

<h3 id="when-provided-servers-connect">
  Cuándo se conectan los servidores proporcionados
</h3>

Cuando `managedMcpServers` llega a través de configuración administrada por servidor, su tiempo sigue [Comportamiento de obtención y almacenamiento en caché](/docs/es/server-managed-settings#fetch-and-caching-behavior):

* En una máquina con configuración almacenada en caché, Claude Code retiene la copia almacenada en caché de esta clave hasta que el servidor confirme la configuración para la sesión, y espera esa confirmación antes de cargar los servidores MCP. Si la confirmación falla, la sesión continúa sin los servidores proporcionados y `/status` dice que se retienen.
* En el primer lanzamiento de una máquina, sin nada almacenado en caché aún, una sesión interactiva que comienza antes de que llegue la configuración conecta los servidores proporcionados tan pronto como lo hacen, y una ejecución `claude -p` que ya ha comenzado puede terminar sin ellos.

Con [inicio de sesión de puerta de enlace](/docs/es/claude-apps-gateway-config#precedence-with-other-managed-sources), Claude Code carga la política antes de que comience la sesión, por lo que ninguno de los dos casos retrasa u omite los servidores proporcionados.

Las sesiones interactivas que ya se están ejecutando aplican sus ediciones a la clave:

* **Agregar un servidor**: Claude Code lo conecta cuando llega la configuración actualizada, sin un reinicio.
* **Cambiar la entrada de un servidor**: esas sesiones se reconectan a él con la nueva definición.
* **Eliminar un servidor**: una sesión interactiva en ejecución lo desconecta una vez que lee la configuración cambiada. Una ejecución no interactiva (`-p`) lo mantiene hasta que termina.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Control basado en políticas con listas de permitidos y listas de bloqueados
</h2>

Las listas de permitidos y listas de bloqueados filtran qué servidores configurados pueden cargarse. No son un registro: un servidor aún tiene que ser añadido por un usuario, un plugin o su organización antes de que cualquiera de las listas se aplique a él.

Los servidores que su organización entrega a través de `managedMcpServers` se cargan sin una entrada de lista de permitidos, y [Cómo se evalúa un servidor](#how-a-server-is-evaluated) cubre los servidores de `managed-mcp.json`. La lista de bloqueados se aplica a cada servidor independientemente de dónde provenga, excepto las entradas `type: "sdk"` en proceso.

Para entregar servidores a los usuarios, use [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) o [`managedMcpServers`](#provide-servers-through-managed-settings). Ambas listas también filtran servidores pasados con la bandera CLI [`--mcp-config`](/docs/es/cli-reference#cli-flags), excepto las entradas `type: "sdk"` en proceso; `--strict-mcp-config` limita qué archivos de configuración se cargan y no omite ninguna de las dos listas.

Para hacer que la lista de permitidos sea autoritaria, establezca `allowedMcpServers` y `allowManagedMcpServersOnly: true` juntos en una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices), como configuración administrada por servidor o un archivo `managed-settings.json` implementado.

El bloqueo se aplica desde cada fuente administrada controlada por administrador, por lo que un bloqueo en un archivo implementado aún se aplica cuando la configuración administrada por servidor que no menciona MCP también está en uso. Mientras el bloqueo está activado, la lista de permitidos administrada proviene de la fuente administrada de mayor rango que establece una. Leer el bloqueo y la lista de permitidos entre fuentes requiere Claude Code v2.1.273 o posterior.

[Restringir la lista de permitidos solo a configuración administrada](#restrict-the-allowlist-to-managed-settings-only) muestra la configuración.

Sin `allowManagedMcpServersOnly`, las listas de permitidos de cada ámbito de configuración se fusionan, incluido el `~/.claude/settings.json` del usuario, por lo que un usuario puede ampliar lo que su lista de permitidos permite. Las listas de bloqueados se fusionan desde cada ámbito independientemente.

<Note>
  `allowManagedMcpServersOnly` es independiente de `allowManagedPermissionRulesOnly`, que bloquea solo [reglas de permisos](/docs/es/permissions#managed-settings). Establecer esa bandera no aplica la lista de permitidos de MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Hacer coincidir servidores por URL, comando o nombre
</h3>

`allowedMcpServers` y `deniedMcpServers` son listas de entradas. Cada entrada es un objeto con una única clave que identifica servidores por su URL, su comando o su nombre:

| Clave           | Coincide con                                                                                | Usar para                                              |
| :-------------- | :------------------------------------------------------------------------------------------ | :----------------------------------------------------- |
| `serverUrl`     | Una URL de servidor remoto, exacta o con comodines `*`                                      | Servidores HTTP y SSE                                  |
| `serverCommand` | El comando exacto y los argumentos que inician un servidor stdio                            | Servidores stdio                                       |
| `serverName`    | La etiqueta asignada por el usuario. Solo coincidencia exacta; los comodines no se expanden | Cualquier tipo, pero vea la Advertencia a continuación |

Dejar `allowedMcpServers` sin establecer es diferente a establecerlo en una matriz vacía:

| Configuración       | Sin establecer (predeterminado)  | Matriz vacía `[]`                                                                                     | Poblada                                                                                                          |
| :------------------ | :------------------------------- | :---------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers` | Se permiten todos los servidores | No se permite ningún servidor, aparte de [los propios de la organización](#how-a-server-is-evaluated) | Solo se permiten servidores coincidentes, aparte de [los propios de la organización](#how-a-server-is-evaluated) |
| `deniedMcpServers`  | No se bloquea ningún servidor    | No se bloquea ningún servidor                                                                         | Se bloquean servidores coincidentes                                                                              |

Vea [Entradas inválidas en configuración administrada](/docs/es/managed-settings#invalid-entries-in-managed-settings) para saber qué sucede cuando una entrada falla la validación del esquema.

<Warning>
  Una entrada `serverName`, en cualquiera de las listas, no es un control de seguridad. El nombre es la etiqueta que un usuario asigna al ejecutar `claude mcp add` o editar un archivo de configuración, no el servidor subyacente, por lo que un usuario puede llamar a cualquier servidor `github`. Para conectores de claude.ai, el nombre es el nombre para mostrar devuelto por claude.ai, que puede cambiar. Para aplicar qué servidores se ejecutan realmente, agregue entradas `serverCommand` o `serverUrl`.
</Warning>

La validación de `serverName` difiere entre las dos listas:

* En `deniedMcpServers`, `serverName` acepta cualquier cadena no vacía sin espacios en blanco iniciales o finales, por lo que puede bloquear [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) por su nombre para mostrar. Por ejemplo, `{ "serverName": "claude.ai Slack" }` bloquea el conector de Slack. Prefiera una entrada `serverUrl` cuando necesite que la denegación sea robusta ante cambios de nombre, o cuando un nombre de conector colisiona y gana un sufijo ` (N)`.
* En `allowedMcpServers`, `serverName` se limita a letras, números, guiones e guiones bajos. Use `serverUrl` para permitir un conector de claude.ai que Claude Code obtiene por sí mismo; para conectores que un host en la nube entrega a sesiones autohospedadas, use las entradas enumeradas en [El tráfico del conector sale de su red](/docs/es/self-hosted-environments-deploy#connector-traffic-leaves-your-network) en su lugar.

Para desactivar todos los conectores de claude.ai que Claude Code obtiene por sí mismo, vea [`disableClaudeAiConnectors`](/docs/es/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Cómo se evalúa un servidor
</h3>

Antes de cargar un servidor, incluido uno de `managed-mcp.json`, Claude Code ejecuta las tres comprobaciones a continuación en orden. Las ejecuta nuevamente cuando un usuario reconecta un servidor o activa uno deshabilitado en `/mcp`. Los servidores `type: "sdk"` en proceso, que [la aplicación que inició la sesión registra](/docs/es/mcp#how-connectors-reach-claude-code), omiten las tres.

1. **Fusionar las listas.** Las entradas de lista de permitidos y lista de bloqueados de cada ámbito de configuración se combinan en una lista de permitidos y una lista de bloqueados. Cuando `allowManagedMcpServersOnly` es `true`, solo se mantiene la lista de permitidos administrada; la lista de bloqueados siempre se fusiona desde cada ámbito. Cuando hay más de una fuente administrada presente, [Las claves se leen desde cada fuente administrada](/docs/es/managed-settings#keys-read-from-every-admin-source) dice cuál de ellas suministra las listas del ámbito administrado.
2. **Verificar la lista de bloqueados.** Un servidor que coincida con cualquier entrada de lista de bloqueados, por URL, comando o nombre, se bloquea. Nada anula una coincidencia de lista de bloqueados.
3. **Verificar la lista de permitidos.** Si `allowedMcpServers` no está establecido en ningún lugar, cada servidor que pasó la lista de bloqueados se carga. Si está establecido, lo que el servidor debe coincidir depende de su tipo, que se muestra en la tabla a continuación.

   Los servidores propios de la organización omiten esta comprobación: cada entrada `managedMcpServers`, y cualquier entrada `managed-mcp.json` cuyos valores no usen expansión `${VAR}`. Los servidores integrados también la omiten, como Claude en Chrome, el servidor `ide` al que Claude Code se conecta en un IDE de VS Code o JetBrains en ejecución, y servidores que la CLI misma configura.

   Un servidor `managed-mcp.json` que usa expansión `${VAR}` en su comando, argumentos, `env`, URL o encabezados aún se verifica, al igual que cada servidor que un usuario, un plugin, `--mcp-config` o claude.ai añade.

| Tipo de servidor    | Se permite cuando coincide                                                                                                                |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Remoto (HTTP o SSE) | Una entrada `serverUrl`. Una coincidencia `serverName` cuenta solo cuando la lista de permitidos no contiene entradas `serverUrl`         |
| Stdio               | Una entrada `serverCommand`. Una coincidencia `serverName` cuenta solo cuando la lista de permitidos no contiene entradas `serverCommand` |

Tres reglas de coincidencia se aplican dentro de esas comprobaciones:

* **Los comandos coinciden exactamente.** Cada argumento, en orden. `["npx", "-y", "server"]` no coincide con `["npx", "server"]` o `["npx", "-y", "server", "--flag"]`.
* **Los valores `serverCommand` y `serverUrl` se expanden antes de coincidir.** Tanto la entrada de política como el valor configurado del servidor pasan por expansión [`${VAR}` y `${VAR:-default}`](/docs/es/mcp#environment-variable-expansion-in-mcp-json), por lo que una entrada escrita como `["${HOME}/bin/server"]` coincide con una configuración de servidor que usa la misma referencia o la ruta expandida. En Windows, haga referencia a una variable de entorno que esté establecida allí, como `${USERPROFILE}` en lugar de `${HOME}`. Los valores `serverName` coinciden literalmente y nunca se expanden. Los dos lados leen entornos diferentes; [Cómo se expanden las entradas de política](#how-policy-entries-expand) cubre cuál es cuál, y cómo difieren las entradas de lista de permitidos y lista de bloqueados.
* **Las URLs admiten comodines `*`** en cualquier lugar del patrón, incluido el esquema. La coincidencia de nombre de host no distingue mayúsculas de minúsculas e ignora un punto FQDN final, por lo que `https://Mcp.Example.com/*` coincide con `https://mcp.example.com/api`. Las rutas permanecen sensibles a mayúsculas y minúsculas.

| Patrón                      | Permite                                                                                |
| :-------------------------- | :------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Todas las rutas en un dominio específico                                               |
| `https://mcp.example.com`   | También todas las rutas en ese dominio. Un patrón sin ruta coincide con cualquier ruta |
| `https://*.example.com/*`   | Cualquier subdominio de `example.com`                                                  |
| `http://localhost:*/*`      | Cualquier puerto en localhost                                                          |
| `*://mcp.example.com/*`     | Cualquier esquema a un dominio específico                                              |

<h4 id="how-policy-entries-expand">
  Cómo se expanden las entradas de política
</h4>

El valor configurado del servidor se expande desde el entorno de proceso activo, como el resto de `.mcp.json`. Una entrada de política se expande desde un entorno fijado en su lugar, por lo que una variable establecida por un proyecto o archivo de configuración de usuario no puede cambiar lo que significa una entrada de lista de permitidos. Debido a que una entrada de política aún depende del valor de la shell de inicio para cualquier variable a la que haga referencia, use URLs y comandos literales para entradas en las que confía para la aplicación.

| Lista de entradas   | Se expande desde                                                                                                                                                                                                          | Expansión que cambiaría el esquema, host o ámbito de ruta de una entrada de URL |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `allowedMcpServers` | El entorno desde el que Claude Code se inició, más valores `env` de configuración administrada                                                                                                                            | Claude Code ignora la entrada                                                   |
| `deniedMcpServers`  | Lo mismo, y una variable sin valor de inicio y sin `:-default` se rellena desde archivos de configuración fuera del repositorio, como configuración de usuario o administrada, que solo amplía lo que la entrada coincide | La entrada aún coincide                                                         |

Requiere Claude Code v2.1.219 o posterior.

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

<Accordion title="Lista de permitidos solo de URL">
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

<Accordion title="Lista de permitidos solo de comando">
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

<Accordion title="Lista de permitidos solo de nombre">
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

Cuando `allowManagedMcpServersOnly` es `true`, las listas de permitidos de configuración de usuario, proyecto y local se ignoran. La lista de bloqueados aún se fusiona desde cada ámbito de configuración, por lo que los usuarios siempre pueden bloquear servidores para sí mismos.

<h2 id="how-restrictions-appear-to-users">
  Cómo aparecen las restricciones a los usuarios
</h2>

Para ver lo que los usuarios ven al iniciar cuando se implementa `managed-mcp.json` y la sesión también tiene servidores `--mcp-config`, consulte [Control exclusivo con managed-mcp.json](#exclusive-control-with-managed-mcp-json). Utilice esta tabla para reconocer los otros informes y para indicar a los usuarios qué esperar antes de implementar un cambio:

| Restricción                                                                                                                 | Lo que ve el usuario                                                                                                         |
| :-------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` está presente y el usuario ejecuta `claude mcp add`                                                      | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| El servidor está en una lista de bloqueo y el usuario ejecuta `claude mcp add`                                              | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| El servidor no está en la lista de permitidos y el usuario ejecuta `claude mcp add`                                         | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| El usuario ejecuta `claude mcp remove` en un servidor de `managedMcpServers`                                                | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| Un servidor previamente configurado ahora está bloqueado por política                                                       | El servidor desaparece de `/mcp` y `claude mcp list`                                                                         |
| Un servidor se bloquea mientras se ejecuta una sesión y el usuario selecciona **Reconnect** o lo vuelve a activar en `/mcp` | [`MCP server <name> is blocked by enterprise managed policy`](/docs/es/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

Cuando un servidor desaparece silenciosamente, el usuario no recibe ninguna señal de que la política sea la razón, así que informe a los usuarios afectados qué servidores están bloqueados cuando implemente una nueva restricción.

<h2 id="monitor-mcp-usage">
  Monitorear el uso de MCP
</h2>

Cuando se configura [exportación de OpenTelemetry](/docs/es/monitoring-usage), Claude Code puede registrar qué servidores MCP y herramientas invocan los usuarios. Establezca `OTEL_LOG_TOOL_DETAILS=1` para incluir nombres de servidor MCP y herramientas en eventos de herramientas, luego agréguelos en su recopilador para ver qué servidores conectan realmente sus usuarios. Consulte [Monitoring](/docs/es/monitoring-usage) para configurar el exportador y para el esquema de evento completo.

<h2 id="configuration-summary">
  Resumen de configuración
</h2>

Cada archivo y configuración que cubre esta página, qué controla y cómo entregarlo:

| Superficie                   | Qué controla                                                                                                                                                                                                                                                              | Dónde vive                                                                                                                                                                                                                                                 | Cómo entregar                                                                                                                                                                                                                  |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Conjunto de servidor fijo, control exclusivo                                                                                                                                                                                                                              | Ruta del sistema: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, o `C:\Program Files\ClaudeCode\`                                                                                                                                        | MDM, GPO, administración de flota, o cualquier proceso con privilegios de administrador. No se puede establecer a través de configuración administrada por servidor                                                            |
| `managedMcpServers`          | Servidores remotos proporcionados a cada usuario junto con los suyos propios                                                                                                                                                                                              | Solo fuentes de configuración administrada; la configuración no tiene efecto en otros lugares                                                                                                                                                              | Una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices): configuración administrada por servidor, una política de puerta de enlace, `managed-settings.json`, perfil MDM, o registro HKLM |
| `allowedMcpServers`          | Lista de permitidos de servidores permitidos                                                                                                                                                                                                                              | Cualquier [ámbito de configuración](/docs/es/settings#where-settings-live); [Cómo se evalúa un servidor](#how-a-server-is-evaluated) dice cómo se combinan las listas de varios ámbitos y fuentes administradas                                                 | Para aplicación, una [fuente de configuración administrada](/docs/es/admin-setup#decide-how-settings-reach-devices): configuración administrada por servidor, `managed-settings.json`, perfil MDM, o registro                       |
| `deniedMcpServers`           | Lista de bloqueados de servidores bloqueados                                                                                                                                                                                                                              | Cualquier ámbito de configuración; [Cómo se evalúa un servidor](#how-a-server-is-evaluated) dice cómo se combinan las listas de varios ámbitos y fuentes administradas                                                                                     | Igual que `allowedMcpServers`                                                                                                                                                                                                  |
| `allowManagedMcpServersOnly` | Bloquea la lista de permitidos solo a fuentes administradas                                                                                                                                                                                                               | Solo fuentes de configuración administrada; [Las claves leídas de cada fuente de administrador](/docs/es/managed-settings#keys-read-from-every-admin-source) dice qué fuentes administradas pueden activarla. La configuración no tiene efecto en otros ámbitos | Igual que `allowedMcpServers`                                                                                                                                                                                                  |
| `allowAllClaudeAiMcps`       | Carga los conectores de claude.ai que Claude Code obtiene por sí mismo junto con `managed-mcp.json`. [Un `managed-mcp.json` en el host que ejecuta una sesión en la nube aún suprime los conectores de esa sesión](#allow-claude-ai-connectors-alongside-the-managed-set) | Solo fuentes de configuración administrada; la configuración no tiene efecto en otros lugares                                                                                                                                                              | Igual que `allowedMcpServers`                                                                                                                                                                                                  |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Decide what to enforce](/docs/es/admin-setup#decide-what-to-enforce): restricciones de MCP junto con reglas de permisos, sandboxing y los otros controles de administrador
* [Connect Claude Code to tools via MCP](/docs/es/mcp): la referencia completa de MCP, incluidos transportes, alcances y autenticación
* [Settings](/docs/es/settings): la jerarquía de configuración y cómo la configuración administrada tiene prioridad
* [Server-managed settings](/docs/es/server-managed-settings): entregar `allowedMcpServers` y `deniedMcpServers` desde la consola de administrador de Claude.ai
* [Security](/docs/es/security): el modelo de amenaza que estos controles defienden
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, administración de asientos y guía de implementación
