> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Seguridad y confianza de plugins

> Decide si confiar en un plugin antes de instalarlo, desde lo que un plugin puede hacer en tu máquina hasta cómo revisar uno y eliminarlo.

Un plugin de Claude Code que instales puede ejecutar código arbitrario en tu máquina con tus privilegios de usuario.

Instalas un plugin desde un marketplace, que es el catálogo del que Claude Code lo obtiene. Algunos nombres de marketplace están [reservados para los propios marketplaces de Anthropic](#marketplace-tiers), y todos los demás marketplaces son de terceros. El nombre de un marketplace te dice quién publica el catálogo, no qué hace cada plugin en él, así que [revisa un plugin antes de instalarlo](#review-a-plugin-before-you-install) sea cual sea el marketplace del que provenga.

Lee esta página si estás decidiendo si instalar un plugin, o si revisas herramientas antes de que tu equipo pueda usarlas.

<Note>
  Estos casos se cubren en otras páginas:

  * **Modelo de seguridad propio de Claude Code**: consulta [Security](/docs/es/security)
  * **Restringir o requerir plugins para una organización**: consulta [Manage plugins for your organization](/docs/es/plugins/org)
  * **Los plugins `security-guidance` o `claude-security`**: esta página no trata sobre ellos. Consulta [`security-guidance`](/docs/es/security-guidance) y [`claude-security`](/docs/es/claude-security)
</Note>

Comienza con [qué puede hacer un plugin](#understand-what-a-plugin-can-do) y [cuáles son los marketplaces de Anthropic](#marketplace-tiers), luego [revisa el plugin antes de instalarlo](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Entender qué puede hacer un plugin
</h2>

Un plugin puede contener contenido que ejecuta código en tu máquina con tus privilegios de usuario y contenido que entra en el contexto de Claude como instrucciones, así que [revisa un plugin antes de instalarlo](#review-a-plugin-before-you-install). Esto es lo que un plugin instalado puede hacer:

* **Hooks**: los [hooks](/docs/es/hooks) de un plugin se ejecutan como comandos de shell en puntos del ciclo de vida de Claude Code, como antes o después de una llamada a herramienta.
* **Servidores MCP y LSP**: Claude Code se conecta a los [servidores MCP](/docs/es/mcp) que declara un plugin habilitado y le da a Claude sus herramientas. Un servidor MCP de stdio se ejecuta como un proceso que Claude Code inicia en tu máquina. Claude Code también inicia los servidores de lenguaje que declara el plugin.
* **Directorio `bin/`**: Claude Code añade el directorio `bin/` de cada plugin habilitado a la `PATH` del shell de la herramienta Bash, así que los comandos Bash de Claude pueden ejecutar cualquier ejecutable allí.
* **Skills, comandos y agentes**: estos entran en el contexto de Claude como instrucciones, así que influyen en lo que Claude hace con las herramientas que ya tiene.
* **Actualizaciones**: cuando la actualización automática está activada para el marketplace desde el que instalaste un plugin, Claude Code actualiza ese plugin en segundo plano, así que los archivos que revisaste pueden cambiar en el disco. [Cuándo se ejecuta la actualización automática](/docs/es/plugins/loading#when-auto-update-runs) tiene el cronograma. Para activar o desactivar la actualización automática por marketplace, consulta [Keep plugins updated](/docs/es/plugins/install#keep-plugins-updated).

Las [reglas de permisos](/docs/es/permissions) y [sandbox](/docs/es/sandboxing) de Claude Code cubren las llamadas a herramientas que hace Claude, no el código que ejecuta un plugin por sí solo:

* **Hooks y procesos de servidor**: los hooks de comando ejecutan comandos de shell con tus permisos de usuario completos. Claude Code ejecuta hooks y servidores MCP fuera del sandbox.
* **Llamadas a herramientas de Claude**: una llamada a una de las herramientas MCP del plugin, y un comando Bash que ejecuta un ejecutable del `bin/` del plugin, son llamadas a herramientas, así que tus reglas de permisos se aplican a ellas.

Instalar un plugin también lo habilita, a menos que su manifiesto o entrada de marketplace establezca [`defaultEnabled: false`](/docs/es/plugins/install#choose-an-install-scope) y no lo hayas habilitado tú mismo.

Para eliminar un plugin en el que ya no confías, consulta [Remove a plugin you no longer trust](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Identificar los marketplaces de Anthropic por nombre
</h2>

El nombre de un marketplace lo coloca en uno de tres niveles: oficial, comunidad o terceros. Claude Code acepta los nombres oficial y comunidad solo para marketplaces obtenidos de repositorios `github.com/anthropics/`, así que un marketplace de terceros no puede presentarse como uno de Anthropic. Un marketplace que publica un compañero de trabajo u tu organización es de terceros.

La tabla enumera qué nombres caen en cada nivel:

| Nivel     | Cuáles marketplaces                                                                               |
| :-------- | :------------------------------------------------------------------------------------------------ |
| Oficial   | Los [nombres de marketplace oficial](#official-marketplace-names), como `claude-plugins-official` |
| Comunidad | `claude-community`, `claude-plugins-community`, y `healthcare`                                    |
| Terceros  | Todos los demás marketplaces                                                                      |

Donde el catálogo `claude-community` fija un plugin a un SHA de commit, que lo hace para casi todas las entradas, Claude Code se niega a instalar un commit diferente.

<h3 id="official-marketplace-names">
  Nombres de marketplace oficial
</h3>

Estos nombres de marketplace conforman el nivel oficial:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

Para saber cómo difieren los marketplaces oficial, comunidad y demo y dónde explorar lo que enumera cada uno, consulta [Anthropic's marketplaces](/docs/es/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Revisar un plugin antes de instalarlo
</h2>

Antes de instalar un plugin, mira qué añade y de dónde viene.

<Steps>
  <Step title="Verificar la fuente del marketplace">
    En tu shell, ejecuta `claude plugin marketplace list` para imprimir la fuente desde la que se añadió cada marketplace, como un repositorio de GitHub o un directorio.
  </Step>

  <Step title="Leer el panel de detalles">
    En una sesión de Claude Code, ejecuta `/plugin` y selecciona el plugin. El panel de detalles muestra una sección **Will install** que enumera los comandos, agentes, skills, hooks y servidores MCP y LSP del plugin. Para un plugin del que Anthropic no ha publicado datos de componentes, la sección muestra lo que declara la entrada del marketplace, o una nota: `Components will be discovered at installation` para un plugin almacenado dentro del marketplace, o `Component summary not available for remote plugin` para uno obtenido de otro lugar.
  </Step>

  <Step title="Leer la fuente del plugin">
    En el panel de detalles, selecciona **Open homepage** o **View on GitHub** debajo de las opciones de instalación. Si el panel no ofrece ninguno, abre el repositorio del marketplace que encontraste en el primer paso. Encuentra el directorio del plugin allí. La sección **Will install** muestra que existe un hook pero no qué ejecuta, así que lee estos archivos en el directorio del plugin:

    * **`hooks/hooks.json`**: el comando que ejecuta cada hook
    * **`.mcp.json`**: el comando o URL de cada servidor
    * **`bin/`**: cada archivo en el directorio
  </Step>

  <Step title="Enumerar lo que contiene el plugin">
    Clona el repositorio que contiene el directorio del plugin, luego ejecuta `claude --plugin-dir <plugin directory> plugin details <plugin name>` en tu shell para ver qué encuentra Claude Code en él. El comando lee los archivos del plugin sin iniciar una sesión e imprime un `Component inventory` que enumera los skills y comandos del plugin, agentes, hooks con el evento de cada hook, y servidores MCP y LSP.
  </Step>
</Steps>

Después de instalar un plugin, ejecuta `claude plugin details <plugin name>` en tu shell para imprimir el mismo `Component inventory` para la copia instalada bajo `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Eliminar un plugin en el que ya no confías
</h3>

En tu shell, ejecuta [`claude plugin uninstall <plugin>`](/docs/es/plugins/cli-reference#plugin-uninstall) con el `--scope` en el que lo instalaste. Luego verifica qué eliminó la desinstalación y qué dejó:

* **Datos persistentes**: cuando esa fue la última scope en la que se instaló el plugin, desinstalar también elimina el directorio de datos persistentes del plugin, a menos que pases `--keep-data`.
* **Archivos en caché**: los archivos del plugin permanecen en el disco bajo `~/.claude/plugins/cache/` durante 14 días antes de que un [barrido de fondo los elimine](/docs/es/plugins/loading#cleanup-of-previous-versions). Después de desinstalar tu último plugin, los directorios huérfanos permanecen hasta que instales otro. Para eliminar los archivos ahora, elimina el directorio del plugin bajo `~/.claude/plugins/cache/<marketplace>/<plugin>/` tú mismo.
* **El marketplace**: si tampoco confías en el propietario del marketplace, [elimina el marketplace](/docs/es/plugins/install#manage-marketplaces) también, que desinstala cada plugin que instalaste desde él.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Reconocer cuándo Claude Code se niega o advierte
</h2>

El panel de detalles que abres desde la pestaña **Discover** o **Marketplaces** en `/plugin` muestra la misma advertencia de confianza para cada plugin. Claude Code se niega en lugar de advertir en casos como los de [Untrusted marketplace sources and failed integrity checks](#untrusted-marketplace-sources-and-failed-integrity-checks).

<h3 id="trust-warning-before-you-install">
  Advertencia de confianza antes de instalar
</h3>

La advertencia lee igual sea cual sea el marketplace del que provenga el plugin:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Si tu organización establece `pluginTrustMessage` en [managed settings](/docs/es/plugins/org), Claude Code añade ese texto a la advertencia.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Fuentes de marketplace no confiables y fallos en las comprobaciones de integridad
</h3>

Claude Code se niega a cargar un marketplace o a instalar un plugin en estos casos, cada uno con su propio mensaje de error:

* **Fuente de marketplace no confiable**: cuando un marketplace usa un nombre oficial o de comunidad pero su fuente está fuera de `github.com/anthropics/`, Claude Code deja de cargar el marketplace y los plugins que instalaste desde él. El error es [Marketplace is registered from an untrusted source](/docs/es/errors#marketplace-is-registered-from-an-untrusted-source).
* **Integridad del archivo**: cuando una entrada de marketplace fija una [fuente `archive`](/docs/es/plugins/marketplace-reference#archive-plugin-source) a un resumen `sha256` y el resumen del archivo descargado no coincide, Claude Code se niega a la instalación. El error es [Plugin archive integrity check failed](/docs/es/errors#plugin-archive-integrity-check-failed).

El pin `sha256` es separado del pin de SHA de commit del catálogo de comunidad, que selecciona el commit de git a verificar.

<h2 id="enforce-plugin-controls-for-your-organization">
  Aplicar controles de plugins para tu organización
</h2>

Con [managed settings](/docs/es/plugins/org), un administrador puede aplicar estos controles de plugins:

* Permitir o bloquear fuentes de marketplace
* Forzar la habilitación de plugins
* Desactivar los indicadores `--plugin-dir` y `--plugin-url` y la variable `CLAUDE_CODE_PLUGIN_DIRS`
* Limitar hooks a los de managed settings y plugins forzados a habilitarse
* Evitar que los plugins de las cuentas claude.ai de los miembros se carguen en Claude Code, con [`syncClaudeAiPlugins`](/docs/es/plugins/org#control-matrix)

La [matriz de control](/docs/es/plugins/org#control-matrix) dice qué cubre y qué no cubre cada clave.

<h2 id="find-plugins-in-telemetry">
  Encontrar plugins en telemetría
</h2>

Si tu organización exporta los [eventos OpenTelemetry](/docs/es/monitoring-usage) de Claude Code a su propio backend, los [niveles de marketplace](#marketplace-tiers) deciden qué nombres de plugin aparecen allí:

* **[Evento Plugin loaded](/docs/es/monitoring-usage#plugin-loaded-event)**: el evento reporta los nombres de plugin y marketplace de nivel oficial tal como son. Para los niveles de comunidad y terceros, `plugin.name` y `marketplace.name` son la cadena literal `third-party` a menos que establezca `OTEL_LOG_TOOL_DETAILS=1`.
* **Scope del plugin**: el `plugin.scope` del evento cargado aún reporta de dónde vino el plugin, como `org` para un plugin que tu managed settings habilita u `user-local` para cualquier otro plugin de terceros. El [evento plugin loaded](/docs/es/monitoring-usage#plugin-loaded-event) enumera cada valor.
* **[Evento Plugin installed](/docs/es/monitoring-usage#plugin-installed-event)**: a menos que establezca `OTEL_LOG_TOOL_DETAILS=1`, el evento omite los campos de nombre para plugins no oficiales en lugar de reportar `third-party`.
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code reporta plugins de los niveles oficial y comunidad por nombre y reporta cada otro plugin como `third-party`.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Manage plugins for your organization](/docs/es/plugins/org): restringe qué marketplaces pueden instalar los usuarios y requiere los que confías
* [Install and manage plugins](/docs/es/plugins/install): revisa el panel de detalles de un plugin antes de elegir una scope
* [Anthropic's marketplaces](/docs/es/plugins/anthropic-marketplaces): cuáles nombres de marketplace son de Anthropic
* [Security](/docs/es/security): el modelo de seguridad propio de Claude Code
