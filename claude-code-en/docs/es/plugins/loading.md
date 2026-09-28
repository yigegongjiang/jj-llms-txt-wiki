> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de carga de plugins

> Rastrear desde dónde Claude Code carga cada plugin, qué archivo de configuración decide si se carga, y por qué una actualización no cambió nada.

Utilice esta página cuando un plugin no se cargó, cargó una copia diferente a la que esperaba, o no recogió una actualización, y desea ver qué fuente, ámbito de configuración o archivo en disco decidió eso. Proporciona las reglas que Claude Code aplica cuando comienza una sesión y cada vez que ejecuta `/reload-plugins`. También puede pedirle a Claude que lea esta página y diagnostique su configuración.

<Note>
  Estos casos se tratan en otras páginas:

  * **Pasos de instalación, habilitación, deshabilitación y actualización**: consulte [Instalar y administrar plugins](/docs/es/plugins/install)
  * **Tiene un mensaje de error específico**: consulte [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting)
</Note>

Comience con [Verificar en qué etapa llegó un plugin](#check-which-stage-a-plugin-reached) para las tres etapas por las que pasa un plugin instalado, o vaya a la sección que coincida con lo que está viendo:

* Un plugin que desactivó sigue cargándose: [Encontrar dónde está habilitado un plugin](#find-where-a-plugin-is-enabled)
* Una actualización no cambió nada: [Versiones y actualizaciones](#versions-and-updates)
* Está mirando los archivos bajo `~/.claude/plugins/`: [Encontrar plugins en disco](#find-plugins-on-disk)
* Un plugin `--plugin-dir` no se cargó, o se cargó un plugin con el mismo nombre en su lugar: [Conflictos de nombres](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  Verificar en qué etapa llegó un plugin
</h2>

Una entrada `enabledPlugins` se convierte en un plugin que puede usar en etapas: su configuración lo declara, Claude Code lo obtiene en disco, y la sesión en ejecución lo carga. Cuando un plugin no se comporta como sugiere un archivo de configuración, verifique en qué etapa llegó:

* **Declarado, en configuración**: `enabledPlugins` dice qué plugins deben estar activados, y `extraKnownMarketplaces` dice qué mercados deben existir. Cuando ejecuta `claude plugin marketplace add`, Claude Code escribe el mercado en `extraKnownMarketplaces` en su configuración de usuario, así como en disco
* **Obtenido, en disco bajo `~/.claude/plugins/`**: los registros de lo que Claude Code ha obtenido, y los archivos obtenidos en sí:
  * `known_marketplaces.json` registra cada mercado que Claude Code ha obtenido, con su `source`, `installLocation`, `lastUpdated`, y `autoUpdate`. Hay un `known_marketplaces.json` por usuario, por lo que un mercado que agrega en un proyecto está disponible en cada proyecto
  * `installed_plugins.json` registra cada instalación con su `scope`, `installPath`, y `version`
  * `cache/` contiene los archivos del plugin
* **Cargado, en la sesión en ejecución**: el conjunto de plugins que Claude Code cargó al inicio o en el último `/reload-plugins`. Los cambios en la configuración o en disco no llegan a esta capa hasta que ejecute `/reload-plugins` o inicie una nueva sesión. Por eso `claude plugin update` termina con `Restart to apply changes.` y las actualizaciones en segundo plano le solicitan que ejecute `Run /reload-plugins to apply`

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  Plugins y mercados que no están en disco al inicio de la sesión
</h3>

Los plugins se cargan al inicio de la sesión desde `installed_plugins.json` y la caché sin usar la red. Después de que comienza la sesión, Claude Code verifica los mercados declarados en segundo plano:

* **Un mercado que la configuración declara pero que `known_marketplaces.json` carece**: Claude Code lo clona, luego recarga plugins y descarga plugins habilitados que aún no están en caché
* **Un mercado declarado cuya fuente cambió en la configuración**: Claude Code lo vuelve a obtener de la nueva fuente y muestra `Plugins changed. Run /reload-plugins to activate.`

Un plugin habilitado que ninguna ruta obtuvo y que no tiene un directorio de caché utilizable muestra `Plugin "<name>" not cached at <path>` en la pestaña **Errors** de `/plugin`, y `claude plugin list` agrega `— run /plugin to refresh` a la misma línea. Para la solución, consulte [`Plugin "<name>" not cached at <path>`](/docs/es/plugins/troubleshooting#plugin-not-cached-at).

<h2 id="find-where-a-plugin-came-from">
  Encontrar de dónde vino un plugin
</h2>

Cada plugin tiene un id de la forma `<name>@<origin>`, que es lo que ve en archivos de configuración y en `claude plugin list --json`. La parte después de `@` le dice dónde Claude Code encontró el plugin:

| El ID termina en | Cómo llegó el plugin allí                                                                                                                                                                                         | Cómo lo activa o desactiva                                                                                                                                                                                              |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@<marketplace>` | Lo instaló desde un mercado que agregó                                                                                                                                                                            | `"<name>@<marketplace>": true` o `false` bajo `enabledPlugins` en un archivo de configuración                                                                                                                           |
| `@inline`        | Inició Claude Code con `--plugin-dir` o `--plugin-url`, estableció [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/es/env-vars#variables), o una aplicación del SDK de Agent pasó la opción `plugins`. Se carga solo para esa sesión | Activado para la sesión a menos que el manifiesto establezca `defaultEnabled: false` o un archivo de configuración establezca `"<name>@inline": false`                                                                  |
| `@skills-dir`    | Guardó un directorio de plugin que tiene un `.claude-plugin/plugin.json` bajo `~/.claude/skills/` o el `.claude/skills/` del proyecto                                                                             | El `defaultEnabled` del manifiesto, a menos que un archivo de configuración establezca `"<name>@skills-dir"` en `true` o `false`                                                                                        |
| `@synced`        | Usted u su organización lo activaron para su cuenta claude.ai, y Claude Code lo [descargó](#synced-plugins)                                                                                                       | Activado a menos que el manifiesto establezca `defaultEnabled: false` o un archivo de configuración establezca `"<name>@synced": false`. Un plugin que su organización marca como requerido se carga independientemente |

Para un plugin de mercado, `<name>` es el nombre de entrada en `marketplace.json`; para `@inline` y `@skills-dir` es el `name` en el manifiesto del plugin.

Los nombres de origen en esta tabla están reservados, por lo que ningún mercado puede llamarse `inline`, `skills-dir`, o `synced`.

<h3 id="entry-name-and-manifest-name">
  Nombre de entrada y nombre de manifiesto
</h3>

Un plugin de mercado tiene dos nombres, y pueden diferir:

* **El nombre de entrada en `marketplace.json`**: la clave de instalación y habilitación. Es lo que escribe en `enabledPlugins`, lo que el directorio de caché se nombra después, y lo que `claude plugin list` muestra
* **El `name` en el manifiesto**: bajo lo que se espacian los componentes del plugin, y lo que [conflictos de nombres](#name-conflicts) comparan

<h3 id="plugins-shared-through-a-repository">
  Plugins compartidos a través de un repositorio
</h3>

Para compartir un plugin a través de un repositorio, enumérelo bajo `enabledPlugins` en `.claude/settings.json` o colóquelo bajo `.claude/skills/`. Claude Code no escanea el directorio `.claude/plugins/` de un proyecto.

Una sesión en la nube no agrega los mercados que un repositorio enumera bajo [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces), porque eso requiere el diálogo de confianza del espacio de trabajo, que una sesión en la nube nunca muestra.

Un plugin de directorio de habilidades de ámbito de proyecto se carga solo desde el `.claude/skills/` del [directorio de trabajo principal](/docs/es/permissions#working-directories) de la sesión, y solo después de que acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#what-runs-before-you-trust-a-folder) para esa carpeta. No [busca directorios principales hasta la raíz del repositorio](/docs/es/skills#discovery-from-parent-and-nested-directories) de la manera que lo hacen las habilidades y comandos simples. Si inicia desde un subdirectorio, un plugin en la raíz del repositorio no se carga. Inicie desde la raíz del repositorio en su lugar, o [mueva la sesión allí con `/cd`](/docs/es/permissions#move-the-session-to-another-directory) en v2.1.246 o posterior.

Un plugin de ámbito de proyecto se verifica en el repositorio y llega a cada colaborador que lo clona. Debido a que ese contenido proviene del repositorio en lugar de usted, se carga solo después de la misma verificación de confianza que se aplica a las reglas de permiso del proyecto en `.claude/settings.json`. Confiar en una carpeta principal o ejecutar con `-p` no es suficiente. Los componentes que ejecutan código están restringidos aún más:

* Los servidores MCP que declara pasan por la [misma aprobación por servidor](/docs/es/mcp) que un `.mcp.json` del proyecto
* Los servidores MCP que declara como un [paquete MCP](/docs/es/plugins/manifest-reference#mcpservers), un archivo `.mcpb` o `.dxt`, o desde un archivo fuera del directorio del plugin se omiten. Declárelos en línea o en un `.mcp.json` dentro del directorio del plugin
* [Los monitores en segundo plano](/docs/es/plugins/components#monitors) no se cargan

Los plugins de ámbito personal no tienen ninguna de estas restricciones.

Para saber cómo escribir plugins `--plugin-dir` y de directorio de habilidades, consulte [Crear plugins](/docs/es/plugins/create).

<h3 id="synced-plugins">
  Plugins sincronizados desde claude.ai
</h3>

Un plugin que activa para su cuenta claude.ai también se carga en Claude Code, junto con los plugins que instala desde mercados. Esto incluye plugins que su organización activa para sus miembros. Cada uno de estos plugins se carga como `<name>@synced`, sin mercado y sin [registro de instalación](#check-which-stage-a-plugin-reached).

En sesiones de terminal, las habilidades, agentes, hooks, servidores MCP y servidores LSP de un plugin sincronizado se cargan todos, con la misma confianza que un plugin de mercado que instaló.

Para los componentes que carga Cowork, consulte [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview) en claude.com.

Los plugins sincronizados se cargan en sesiones de Cowork y en sesiones de terminal donde inicia sesión con su cuenta claude.ai:

* **[Cowork](https://claude.com/product/cowork)**: Claude Code los descarga en el entorno propio de la sesión cuando comienza la sesión
* **Sesiones de terminal**: cada vez que inicia Claude Code, se sincroniza una vez en segundo plano, descargando plugins nuevos y actualizados y eliminando los que usted u su organización desactivaron. La sincronización en sesiones de terminal requiere Claude Code v2.1.273 o posterior

<h4 id="sync-timing-in-terminal-sessions">
  Tiempo de sincronización en sesiones de terminal
</h4>

Debido a que la sincronización de terminal se ejecuta en segundo plano, puede terminar después de que su sesión haya comenzado. Cuando agrega, actualiza o elimina un plugin sincronizado en una sesión interactiva, ve `Plugins changed. Run /reload-plugins to activate.` Ejecute `/reload-plugins` para cargar el cambio en esa sesión, o déjelo para la próxima vez que inicie Claude Code.

Si habilita un plugin en claude.ai mientras se ejecuta una sesión, el plugin se descarga la próxima vez que inicie Claude Code.

<h4 id="sign-in-requirements-for-terminal-sync">
  Requisitos de inicio de sesión para sincronización de terminal
</h4>

En su terminal, los plugins se sincronizan solo en sesiones donde inicia sesión con su cuenta claude.ai.

Si inició sesión en una versión anterior de Claude Code, ese inicio de sesión no cubre plugins hasta que Claude Code lo renueve en segundo plano. Para obtener acceso más rápido, ejecute `/login` nuevamente. La sincronización de plugins comienza la próxima vez que inicia Claude Code.

<h4 id="control-which-synced-plugins-load">
  Controlar qué plugins sincronizados se cargan
</h4>

Puede desactivar plugins sincronizados uno a la vez, excepto un plugin que su organización requiere, o desactivar todos los plugins sincronizados en la máquina:

* **Un plugin**: `claude plugin disable <name>@synced` en su shell y la pestaña **Installed** de `/plugin` en una sesión guardan `"<name>@synced": false` en su [`enabledPlugins`](/docs/es/settings-reference#enabledplugins) de nivel de usuario. Para mantener el plugin fuera de un proyecto en cada entorno, establezca la misma clave en el `.claude/settings.json` comprometido del proyecto
* **Todos los plugins sincronizados en una máquina**: establezca [`syncClaudeAiPlugins`](/docs/es/settings-reference#syncclaudeaiplugins) en `false` en su configuración de usuario, o su organización lo establece en [configuración administrada](/docs/es/managed-settings). Claude Code deja de descargar, y la próxima vez que lo inicie, mueve los plugins que ya sincronizó a `~/.claude/plugins/.trash/` y ya no los carga. Si su organización desactiva Skills en claude.ai, los plugins dejan de sincronizarse también
* **Un plugin que su organización requiere**: un plugin que su organización marca como requerido en claude.ai se carga incluso si lo desactivó anteriormente. `claude plugin disable` lo rechaza con `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.`, y `claude plugin list` lo marca como `required by your org`

Para eliminar un plugin en claude.ai, consulte [Administrar plugins instalados](/docs/es/plugins/install#manage-installed-plugins).

<h2 id="find-where-a-plugin-is-enabled">
  Encontrar dónde está habilitado un plugin
</h2>

Puede establecer una entrada `enabledPlugins` en cualquiera de seis fuentes. La tabla las enumera de menor a mayor precedencia, y a quién se aplica cada una. Para los archivos de configuración en sí, consulte [Archivos de configuración y a quién afectan](/docs/es/settings#where-settings-live).

| Fuente      | Dónde lo establece                                                                                | Alcanza                                                                                                              |
| :---------- | :------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------- |
| `--add-dir` | `.claude/settings.json` o `.claude/settings.local.json` en un directorio que pasa con `--add-dir` | Solo esta sesión. Solo un valor `true` tiene efecto, y todas las otras fuentes lo anulan                             |
| `user`      | `~/.claude/settings.json`                                                                         | Usted, en cada proyecto                                                                                              |
| `project`   | `.claude/settings.json`                                                                           | Todos los que clonan el repositorio                                                                                  |
| `local`     | `.claude/settings.local.json`                                                                     | Usted, solo en este repositorio                                                                                      |
| `flag`      | El valor `--settings` que pasa al iniciar                                                         | Solo esta sesión                                                                                                     |
| `managed`   | [Configuración administrada](/docs/es/managed-settings)                                                | Cada usuario que cubre la política. `true` fuerza la habilitación y `false` bloquea, y ninguna otra fuente los anula |

Estas fuentes se fusionan clave por clave. Para cada id de plugin, el valor que se aplica es el de la fuente de mayor precedencia que menciona el id. Una fuente que no menciona el id deja el valor de la fuente de menor precedencia en vigor.

<h3 id="disabled-in-user-settings-but-still-loads">
  Deshabilitado en configuración de usuario pero aún se carga
</h3>

Si establece un plugin en `false` en `~/.claude/settings.json` y aún se carga, un `true` en una fuente de mayor precedencia lo está anulando. La fila del plugin en `claude plugin list` y en `/plugin` muestra `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting`. El mensaje nombra la fuente que lo anuló: `project`, `project, gitignored` para `.claude/settings.local.json`, `cli flag`, o `managed`.

Para optar por no participar en un plugin habilitado por proyecto en su máquina, establezca el id en `false` en `.claude/settings.local.json`, que tiene mayor precedencia que el archivo del proyecto.

<h3 id="enabled-in-project-settings-but-not-installed">
  Habilitado en configuración del proyecto pero no instalado
</h3>

Cuando el único `true` de un plugin está en el `.claude/settings.json` del proyecto, Claude Code no lo obtiene en una máquina donde no está instalado, a menos que su entrada de mercado tenga una [fuente de ruta relativa](/docs/es/plugins/marketplace-reference#plugin-sources) o un [directorio semilla](/docs/es/plugins/org#seed-containers-and-ci) ya lo contenga. En su lugar, la pestaña **Errors** de `/plugin` muestra `Plugin "<name>" is enabled in project settings but isn't installed here`.

Un plugin de ruta relativa no necesita un registro de instalación porque se carga desde el mercado en sí.

Claude Code obtiene un plugin con una fuente externa solo cuando una de estas fuentes lo establece en `true`:

* Su configuración de usuario
* Un `.claude/settings.local.json` que git no rastrea
* La bandera `--settings`
* Configuración administrada

<h2 id="find-plugins-on-disk">
  Encontrar plugins en disco
</h2>

Claude Code mantiene archivos de plugin y registros de estado bajo una raíz de plugins, que es `~/.claude/plugins` a menos que establezca [`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/es/env-vars). Cada ruta en la tabla es relativa a esa raíz.

| Ruta                                                 | Lo que contiene                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :--------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache/<marketplace>/<plugin>/<version>/`            | Un directorio por versión instalada de un plugin de mercado. `<plugin>` es el nombre de entrada del mercado y `<version>` es la [versión resuelta](#versions-and-updates). `${CLAUDE_PLUGIN_ROOT}` apunta a este directorio                                                                                                                                                                                                                                |
| `data/<plugin-id>/`                                  | El directorio persistente del plugin, expuesto como `${CLAUDE_PLUGIN_DATA}`. Para saber cómo se forma `<plugin-id>`, consulte [Variables de ruta y datos persistentes](/docs/es/plugins/components#path-variables-and-persistent-data). Claude Code lo crea cuando un componente del plugin lo usa por primera vez y lo mantiene en las actualizaciones. Claude Code lo elimina cuando desinstala el plugin de su último ámbito, a menos que pase `--keep-data` |
| `marketplaces/<name>/`                               | El clon o descarga de un mercado agregado desde GitHub, otro host de Git, o una URL. Un mercado agregado desde una fuente `file` o `directory` local no tiene copia aquí, y su `installLocation` en `known_marketplaces.json` es la ruta que proporcionó                                                                                                                                                                                                   |
| `synced/`                                            | Los plugins que Claude Code [sincronizó desde su cuenta claude.ai](#synced-plugins)                                                                                                                                                                                                                                                                                                                                                                        |
| `.trash/`                                            | Plugins que la sincronización de claude.ai eliminó, como después de desactivar uno en claude.ai o dejar de sincronizar                                                                                                                                                                                                                                                                                                                                     |
| `installed_plugins.json` y `known_marketplaces.json` | Los registros de lo que Claude Code ha instalado y qué mercados ha obtenido, descritos bajo [Verificar en qué etapa llegó un plugin](#check-which-stage-a-plugin-reached). Un [mercado alojado en claude.ai](/docs/es/plugins/install#add-from-claude-ai) se registra en `known_marketplaces_claudeai.json` en su lugar                                                                                                                                         |
| `flagged-plugins.json`                               | Plugins que Claude Code desinstaló porque su mercado los eliminó de la lista. Aparecen en la sección **Flagged** de `/plugin`; consulte [Alojar un mercado](/docs/es/plugins/host-marketplace)                                                                                                                                                                                                                                                                  |

Debido a que `${CLAUDE_PLUGIN_ROOT}` apunta a un directorio de versión, la ruta raíz de un plugin cambia con cada versión. Mantenga los archivos duraderos de un plugin en `${CLAUDE_PLUGIN_DATA}` en su lugar.

<h3 id="in-place-and-copied-plugins">
  Plugins en lugar y copiados
</h3>

Claude Code carga algunos plugins en lugar desde donde los mantiene y copia el resto en la caché, según su origen:

* **Plugins `--plugin-dir` y de directorio de habilidades**: el directorio se carga en lugar y nunca se copia. Un archivo `.zip` de `--plugin-url` o `--plugin-dir` se extrae en un directorio temporal de sesión primero
* **Plugins de ruta relativa en un mercado que agregó desde un directorio local**: el plugin se carga en lugar desde su ruta dentro de la carpeta del mercado. Sus ediciones al directorio de origen surten efecto en el próximo inicio de sesión o `/reload-plugins`, y no necesita aumentar la versión. Los procesos de hook del plugin y los servidores MCP y LSP reciben un `CLAUDE_PLUGIN_ROOT` que apunta al directorio de origen. Para sus dependencias de paquete Node.js, consulte [Cuándo se ejecuta la instalación de dependencias](#when-the-dependency-install-runs)
* **Plugins de fuente `command` en [modo de enlace](/docs/es/plugins/marketplace-reference#command-plugin-source)**: el directorio que imprimió el comando se carga en lugar, a través de enlaces en la entrada de caché
* **Todos los demás plugins de mercado**: Claude Code copia el plugin en `cache/<marketplace>/<plugin>/<version>/` en la instalación y carga esa copia. Los archivos fuera del directorio del plugin no se copian, por lo que cuando un script dentro de un plugin copiado lee una ruta por encima de la raíz del plugin, como `../shared`, no los encuentra

<h3 id="paths-that-escape-the-plugin-directory">
  Rutas que escapan del directorio del plugin
</h3>

Ya sea que un plugin se cargue en lugar o desde una copia en caché, Claude Code no le permite declarar componentes fuera de su propio directorio. Rechaza una ruta de componente que se resuelve fuera de la raíz del plugin, ya sea que la ruta se declare en `plugin.json` o en una entrada de mercado:

* **Una ruta que apunta fuera del plugin como está escrita**, como `../shared-utils`
* **Un enlace simbólico que conduce fuera del plugin**, que no sean [enlaces entre plugins dentro de un mercado](/docs/es/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)
* **En macOS y Linux, una ruta que contiene una barra invertida en cualquier lugar**, incluso cuando la ruta permanece dentro del plugin. Los componentes declarados con rutas de barra invertida se cargan solo en Windows, por lo que escriba rutas de componentes con barras diagonales, como `./commands/deploy.md`

Una ruta rechazada aparece como un error [`path escapes plugin directory`](/docs/es/errors#path-escapes-plugin-directory), y el plugin se carga sin ese componente.

<h3 id="cleanup-of-previous-versions">
  Limpieza de versiones anteriores
</h3>

Cuando actualiza o desinstala un plugin, Claude Code escribe un marcador `.orphaned_at` en el directorio de versión anterior. Elimina ese directorio en una limpieza en segundo plano 14 días después, por lo que una sesión que ya cargó la versión anterior sigue ejecutándose.

El barrido se ejecuta solo mientras `installed_plugins.json` registra al menos una instalación. Después de desinstalar su último plugin, los directorios huérfanos permanecen hasta que instale otro.

<h3 id="node-js-package-dependencies">
  Dependencias de paquete Node.js
</h3>

Cuando Claude Code copia un plugin en la caché, también instala las dependencias de paquete Node.js del plugin allí, por lo que los hooks y servidores MCP del plugin pueden cargarlas.

Esta sección cubre los paquetes npm y Bun que un plugin declara en su propio `package.json`. Para plugins que dependen de otros plugins, consulte [versiones de dependencia de plugin](/docs/es/plugins/dependencies).

<h4 id="when-the-dependency-install-runs">
  Cuándo se ejecuta la instalación de dependencias
</h4>

Claude Code ejecuta la instalación dentro del directorio de versión copiada cada vez que crea uno:

* Cuando instala un plugin
* Cuando Claude Code actualiza un plugin a una nueva versión
* Al inicio de la sesión cuando un plugin habilitado aún no está en caché, como en una máquina nueva

Para un plugin de ruta relativa [cargado en lugar](#in-place-and-copied-plugins) desde un mercado de directorio local, Claude Code no instala las dependencias en el directorio de origen. Instálelas allí usted mismo, o desde un hook en [`${CLAUDE_PLUGIN_DATA}`](/docs/es/plugins/components#path-variables-and-persistent-data).

La instalación se ejecuta solo cuando el directorio raíz del plugin contiene tanto un `package.json` como un archivo de bloqueo compatible. El archivo de bloqueo decide qué comando ejecuta Claude Code:

| Archivo de bloqueo                          | Comando                                          |
| :------------------------------------------ | :----------------------------------------------- |
| `bun.lock` o `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` o `package-lock.json` | `npm ci --ignore-scripts`                        |

Si un plugin contiene más de uno de estos archivos de bloqueo, Claude Code usa la primera coincidencia, verificando en orden: `bun.lock`, `bun.lockb`, `npm-shrinkwrap.json`, `package-lock.json`.

Claude Code omite la instalación para archivos de bloqueo de Yarn y pnpm y para un `bunfig.toml` junto al archivo de bloqueo de Bun:

* Si su plugin tiene solo un `yarn.lock` o `pnpm-lock.yaml`, reemplácelo con un archivo de bloqueo npm
* Si un `bunfig.toml` está en el mismo directorio que el archivo de bloqueo de Bun, elimine el `bunfig.toml`, o reemplace el archivo de bloqueo de Bun con un archivo de bloqueo npm

Incluya un archivo de bloqueo npm para llegar a la mayoría de los usuarios. Claude Code ejecuta el administrador de paquetes del archivo de bloqueo coincidente desde el PATH del usuario y no intenta el otro archivo de bloqueo en su lugar si falta ese administrador de paquetes.

Para un plugin distribuido a través de una fuente npm, use `npm-shrinkwrap.json`, porque npm excluye `package-lock.json` de los paquetes publicados.

<h4 id="limits-on-the-dependency-install">
  Límites en la instalación de dependencias
</h4>

Claude Code limita esta instalación de dependencias para que ningún código del plugin o sus paquetes se ejecute durante ella, y limita cuánto tiempo puede ejecutarse:

* **Resolución congelada**: Bun y npm instalan exactamente lo que el archivo de bloqueo fija, y fallan en lugar de re-resolver versiones cuando `package.json` y el archivo de bloqueo no coinciden
* **Sin scripts de ciclo de vida**: `--ignore-scripts` evita que se ejecuten scripts `preinstall`, `install` y `postinstall`, por lo que las dependencias que construyen módulos nativos en esos scripts se descargan pero no se compilan durante esta instalación
* **Tiempo de espera de 60 segundos**: Claude Code detiene una instalación que se ejecuta más tiempo y la trata como fallida

Claude Code obtiene un plugin de fuente npm antes de esta instalación de dependencias, y ninguno de los scripts de instalación propios del paquete se ejecuta durante la obtención. Consulte [fuente de plugin npm](/docs/es/plugins/marketplace-reference#npm-plugin-source).

No puede desactivar la instalación automática. Ninguna configuración o variable de entorno la desactiva.

En redes restringidas, consulte los [requisitos de acceso a la red](/docs/es/network-config#network-access-requirements) para los hosts a permitir.

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  Cuándo falla o se omite la instalación de dependencias
</h4>

Una instalación fallida u omitida nunca bloquea el plugin, y cada caso deja un signo diferente:

* Una instalación fallida, u omitida debido a un archivo de bloqueo de Yarn o pnpm o un `bunfig.toml`, aparece como una advertencia en la salida `claude --debug`
* Un plugin con un `package.json` y sin archivo de bloqueo se omite sin una entrada de registro
* Una instalación que agota el tiempo de espera puede dejar un árbol `node_modules` parcial en la copia en caché

Cuando la instalación automática no puede proporcionar una dependencia, instálela desde un hook en el [directorio de datos persistentes](/docs/es/plugins/components#path-variables-and-persistent-data). Eso incluye paquetes que necesitan sus scripts de ciclo de vida para construir, dependencias de Python, y plugins bloqueados con Yarn o pnpm.

<h2 id="versions-and-updates">
  Versiones y actualizaciones
</h2>

Si el autor de un plugin envió nuevas confirmaciones y `claude plugin update` imprime `<name> is already at the latest version (<version>).`, la versión que Claude Code calcula para el plugin no cambia, por lo que nada cambia en disco.

Claude Code calcula una versión para cada plugin que instala, y esa versión es cómo detecta una actualización. `claude plugin update` y la actualización automática en segundo plano calculan la versión nuevamente y omiten el plugin cuando coincide con lo que `installed_plugins.json` registra.

La versión también nombra el directorio de caché del plugin.

Un manifiesto que fija `"version"` es una forma en que la versión calculada permanece igual en las confirmaciones. Consulte [Cómo Claude Code calcula la versión](#how-claude-code-computes-the-version) para el orden de resolución.

Un plugin [cargado en lugar](#in-place-and-copied-plugins) desde un mercado de directorio local carga sus archivos de origen actuales en cada inicio de sesión, sin importar lo que diga su cadena de versión. Para un plugin de un [mercado alojado en claude.ai](/docs/es/plugins/install#add-from-claude-ai), la versión que claude.ai registra para el plugin es su versión, y el `version` del manifiesto no se lee.

<h3 id="how-claude-code-computes-the-version">
  Cómo Claude Code calcula la versión
</h3>

Para un mercado que agregó por fuente, Claude Code elige la regla por el tipo `source` de la entrada del mercado del plugin. La [referencia de mercado](/docs/es/plugins/marketplace-reference#plugin-sources) enumera los tipos de fuente. Para cada tipo de fuente en esa lista excepto `command`:

1. El campo `version` en el manifiesto del plugin viene primero
2. Luego el campo `version` en la entrada del mercado del plugin
3. Cuando ninguno está establecido, la versión viene del tipo de fuente:

| Tipo de fuente                                                                           | Versión cuando no se establece ningún campo `version`                                                                                          |
| :--------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`, `url`, o `git-subdir`                                                          | El SHA de confirmación de la fuente, acortado a 12 caracteres. Una versión `git-subdir` también lleva un hash de la ruta del subdirectorio     |
| `archive`                                                                                | El resumen SHA-256, acortado a 12 caracteres: el pin `sha256` en la entrada del mercado, o el resumen del archivo descargado cuando no hay pin |
| Ruta relativa dentro de un mercado alojado en Git                                        | El SHA de confirmación del directorio instalado                                                                                                |
| Directorio local, cuando ni el directorio del plugin ni su mercado es un repositorio git | `unknown`                                                                                                                                      |
| `npm`                                                                                    | `unknown`                                                                                                                                      |

Claude Code no toma la versión de un repositorio que encierra la ruta de instalación, como un `~/.claude` administrado por git.

Para una fuente `command`, Claude Code siempre deriva la versión de lo que el comando produjo: un hash de 12 caracteres por sí solo, o `<manifest version>-<hash>` cuando el manifiesto establece uno. El `version` de la entrada del mercado se ignora para fuentes de comando. Para lo que cubre el hash, consulte [Modo de copia y modo de enlace](/docs/es/plugins/marketplace-reference#copy-mode-and-link-mode).

Debido a que el manifiesto viene primero, un manifiesto que fija `"version": "1.0.0"` mantiene a cada usuario en la copia en caché hasta que su autor cambie la cadena, sin importar cuántas confirmaciones envíe. Para permitir que los usuarios rastreen confirmaciones en su lugar, deje `version` fuera del manifiesto y la entrada. [Alojar un mercado](/docs/es/plugins/host-marketplace) cubre qué opción se ajusta a qué configuración de lanzamiento.

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Cuándo Claude Code actualiza un mercado antes de una instalación
</h3>

Cuando instala un plugin, Claude Code lo busca en su copia local del catálogo del mercado. Puede ejecutar `/plugin install` en una sesión o `claude plugin install` en su shell, y nombrar el plugin con o sin su mercado. La tabla muestra cuál de esas combinaciones actualiza la copia local.

| Nombre del plugin  | Comando                                     | Lo que Claude Code actualiza                                                                           |
| :----------------- | :------------------------------------------ | :----------------------------------------------------------------------------------------------------- |
| `name@marketplace` | `/plugin install` o `claude plugin install` | El mercado nombrado, antes de la búsqueda                                                              |
| `name` solo        | `/plugin install`                           | Solo mercados que tienen la actualización automática activada, y solo después de que la búsqueda falla |
| `name` solo        | `claude plugin install`                     | Nada. Lee los catálogos en caché sin actualizar                                                        |

La actualización antes de una instalación `name@marketplace` no depende de la configuración de actualización automática del mercado o de `DISABLE_AUTOUPDATER`.

Cuando la actualización falla, la instalación procede desde el catálogo en caché y `claude plugin install` reporta `marketplace not refreshed`.

Claude Code omite la actualización antes de una instalación `name@marketplace` cuando:

* El mercado se agregó desde una fuente `file` o `directory` local, o se define en línea en configuración con una [fuente `settings`](/docs/es/settings-reference#extraknownmarketplaces)
* Un [directorio semilla](/docs/es/env-vars) proporciona el mercado
* Claude Code actualizó el mercado en los últimos 30 segundos
* Establece `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* [Configuración administrada](/docs/es/plugins/org#restrict-what-users-can-install) bloquea el mercado, en cuyo caso Claude Code también rechaza la instalación

<h3 id="when-auto-update-runs">
  Cuándo se ejecuta la actualización automática
</h3>

En una sesión interactiva, después de enviar su primer mensaje, Claude Code espera un retraso aleatorio de hasta diez minutos. Luego actualiza cada mercado con actualización automática activada y actualiza los plugins instalados desde ellos en disco.

La sesión en ejecución mantiene las versiones que cargó, y ve `Plugin updated: <name> · Run /reload-plugins to apply`. Ya sea que recargue o no, las nuevas versiones se cargan en su próximo lanzamiento.

<h4 id="which-marketplaces-and-plugins-auto-update">
  Qué mercados y plugins se actualizan automáticamente
</h4>

Si un mercado se actualiza automáticamente sigue el primero de estos que está establecido:

1. **`autoUpdate` en su entrada `extraKnownMarketplaces`** en un archivo de configuración
2. **`autoUpdate` en su entrada `known_marketplaces.json`**, que el interruptor **Enable auto-update** bajo `/plugin` **Marketplaces** escribe. Cuando un archivo de configuración también declara el mercado bajo `extraKnownMarketplaces`, el interruptor escribe `autoUpdate` a esa entrada de configuración también
3. **El predeterminado**: activado para los mercados oficiales de Anthropic como `claude-plugins-official`, desactivado para `knowledge-work-plugins` y `first-party-plugins`, activado para [mercados agregados desde claude.ai](/docs/es/plugins/install#add-from-claude-ai), y desactivado para todos los demás mercados

Si establece `DISABLE_UPDATES=1`, `DISABLE_AUTOUPDATER=1`, o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, toda la pasada está desactivada y el interruptor **Enable auto-update** está oculto, a menos que también establezca `FORCE_AUTOUPDATE_PLUGINS=1`. La [referencia de variables de entorno](/docs/es/env-vars) cubre el efecto más amplio de cada variable.

La actualización automática también omite un plugin cuya entrada de mercado declara un `headersHelper`. [Instalaciones y actualizaciones que rechazan un comando en lugar de preguntar](/docs/es/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking) explica cuándo aparece tal plugin en la pestaña **Errors** de `/plugin` y cómo lo actualiza desde allí.

Cuando un plugin copiado se actualiza a mitad de sesión, los comandos de hook, monitores, servidores MCP y servidores LSP siguen usando la ruta de la versión anterior. Ejecute `/reload-plugins` para cambiar hooks, servidores MCP y servidores LSP a la nueva ruta. Los monitores requieren un reinicio de sesión.

<h3 id="when-a-command-source-re-runs">
  Cuándo se vuelve a ejecutar una fuente de comando
</h3>

Los plugins con una fuente `command` no esperan la [pasada de actualización automática](#when-auto-update-runs). El directorio impreso refleja el estado de la herramienta en el momento en que se ejecutó el comando, por lo que Claude Code ejecuta el [comando que aceptó](/docs/es/plugins/host-marketplace#change-the-command-of-a-command-source) nuevamente en estos momentos:

* Cada vez que instala o actualiza el plugin
* Una vez por sesión para cada plugin habilitado de fuente de comando, en segundo plano, poco después de que comienza la sesión. Esta ejecución no depende de la configuración de actualización automática del mercado o de `DISABLE_AUTOUPDATER`
* Al inicio o en `/reload-plugins`, cuando la versión instalada de un plugin habilitado falta en la caché del plugin

Claude Code omite las dos ejecuciones en segundo plano cuando establece [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars). Las instalaciones y actualizaciones explícitas aún ejecutan el comando con esa variable establecida.

Cuando la salida con hash del comando ha cambiado, Claude Code instala el resultado como una nueva versión y lo recarga en la sesión interactiva en ejecución, cambiando [los mismos componentes que `/reload-plugins` cambia](/docs/es/plugins/cli-reference#reload-plugins). Ve una notificación de que el plugin fue recargado.

Si recargar en lugar invalidaría la caché de solicitud de la sesión, Claude Code en su lugar le solicita que ejecute `/reload-plugins`, que [advierte sobre el costo de caché y se aplica cuando se vuelve a ejecutar con `--force`](/docs/es/prompt-caching#enabling-or-disabling-a-plugin).

<h2 id="name-conflicts">
  Conflictos de nombres
</h2>

Cuando los plugins habilitados de diferentes orígenes comparten un nombre de manifiesto, este orden decide cuál se carga, de mayor a menor precedencia:

1. Un plugin cuyo id aparece en `enabledPlugins` de configuración administrada, como `true` o `false`. Una copia `--plugin-dir` cuyo nombre de manifiesto coincide con la parte del nombre del id no se carga, y ve `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`
2. Un plugin `--plugin-dir`, `--plugin-url`, o `CLAUDE_CODE_PLUGIN_DIRS` habilitado. Reemplaza un plugin de mercado instalado o de directorio de habilidades con el mismo nombre:
   * **Un plugin de mercado instalado**: reemplazado silenciosamente. `claude plugin list` aún muestra la fila del mercado como habilitada, porque esa fila refleja su configuración. Solo el registro que Claude Code escribe bajo `~/.claude/debug/` cuando inicia con `--debug` registra `Plugin "<name>" from --plugin-dir overrides installed version`
   * **Un plugin de directorio de habilidades**: reemplazado con una fila de pestaña **Errors** de `/plugin` que lee `Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence`
3. Un plugin de mercado instalado. Un plugin de directorio de habilidades con el mismo nombre obtiene la misma fila `Not loaded`, nombrando el plugin instalado
4. Un plugin de directorio de habilidades. Entre dos de estos, la copia bajo `~/.claude/skills/` se carga y la copia de `.claude/skills/` del proyecto se descarta, con una fila que dice qué ruta la eclipsó
5. Un plugin [sincronizado desde claude.ai](#synced-plugins). Cuando un plugin habilitado de cualquier otro origen coincide con su nombre, Claude Code carga ese plugin e informa que la copia de claude.ai no se cargó. Para usar la copia de claude.ai en su lugar, desactive su propia copia

Debido a que el orden compara nombres de manifiesto, un plugin `--plugin-dir` llamado `hello-plugin` reemplaza `hello@example-marketplace` cuando el manifiesto de ese plugin también dice `"name": "hello-plugin"`.

<h3 id="keep-a-session-only-plugin-from-loading">
  Evitar que un plugin de solo sesión se cargue
</h3>

Para evitar que un plugin `--plugin-dir` eclipse algo, o para desactivarlo cuando un proceso principal pasa la bandera para usted, establezca su id en `false` en cualquier archivo de configuración. Para un plugin cuyo nombre de manifiesto es `hello-plugin`, la entrada es `"enabledPlugins": {"hello-plugin@inline": false}`. Un plugin de solo sesión deshabilitado no eclipsa, por lo que la copia de mercado o directorio de habilidades se carga en su lugar.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Instalar y administrar plugins](/docs/es/plugins/install): los pasos de instalación, habilitación, deshabilitación y actualización en sí
* [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting): mensajes de error por la etapa que los produce
* [Referencia de comandos de plugins](/docs/es/plugins/cli-reference): las banderas y comandos nombrados en esta página
* [Administrar plugins para su organización](/docs/es/plugins/org): la configuración administrada que fuerza la habilitación o bloquea plugins
