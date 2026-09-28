> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solucionar problemas de plugins

> Corrija errores de plugins en Claude Code. Encuentre el mensaje exacto que vio, agrupado por etapa desde donde /plugin se ejecuta hasta la instalación y la política de la organización.

Esta página enumera mensajes de error y síntomas para plugins de Claude Code y para mercados, los catálogos desde los que Claude Code instala plugins. Cada entrada proporciona la causa, una solución y lo que ve una vez que la solución funciona.

Cuando un mensaje nombra un plugin o mercado, la entrada muestra un marcador de posición como `<name>` en su lugar.

Utilice esta página si instala plugins, los crea, aloja un mercado o administra plugins para una organización.

<Note>
  Estos casos se tratan en otras páginas:

  * **Por qué los alcances, la caché y la precedencia se comportan de la manera que lo hacen**: lea [Plugin loading reference](/docs/es/plugins/loading)
  * **Buscar un indicador, campo o comando**: use la [plugin commands reference](/docs/es/plugins/cli-reference), la [manifest reference](/docs/es/plugins/manifest-reference), o la [marketplace reference](/docs/es/plugins/marketplace-reference)
</Note>

Busque el mensaje exacto que vio. Cada mensaje se enumera bajo la etapa que lo produce, que no siempre es el comando que ejecutó. Por ejemplo, una instalación puede fallar porque falta un mercado, por lo que ese mensaje está bajo [Add a marketplace](#add-a-marketplace).

<h2 id="find-where-/plugin-runs">
  Find where `/plugin` runs
</h2>

`/plugin` es un comando que escribe dentro de una sesión de terminal de Claude Code en ejecución, y abre un panel interactivo. Las entradas en esta sección cubren los lugares donde puede escribirlo pero no puede ejecutarse, y los comandos que no existen.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

Escribió `/plugin` en algún lugar que no sea una sesión de terminal de Claude Code, y Claude respondió con esta línea en lugar de abrir nada.

Recibe esta respuesta en una sesión que no tiene terminal para dibujar el panel `/plugin` en: [non-interactive mode](/docs/es/headless) con `claude -p`, el Agent SDK, la pestaña Code de la aplicación de escritorio Claude, el panel de extensión de VS Code, y el navegador en claude.ai/code.

En el panel de extensión de VS Code, solo una línea `/plugin` con algo después, como `/plugin install <plugin>@<marketplace>`, recibe esta respuesta. `/plugin` o `/plugins` escrito solo abre el diálogo **Manage plugins**.

Instale el plugin desde la superficie en la que se encuentra:

* **Aplicación de escritorio Claude, sesión local o SSH**: haga clic en el botón **+** junto al prompt, luego **Plugins**, luego **Add plugin** para abrir el [plugin browser](/docs/es/desktop#install-plugins)
* **Extensión de VS Code**: use la pestaña **VS Code** bajo [Install a plugin](/docs/es/plugins/install#install-a-plugin)
* **Claude Code en la web, o una sesión de escritorio en la nube**: una sesión en la nube no tiene navegador de plugins. Vea la pestaña **Cloud session** bajo [Install a plugin](/docs/es/plugins/install#install-a-plugin) para lo que carga una sesión en la nube
* **Una terminal a la que tiene acceso**: ejecute `claude` y escriba `/plugin` allí, o ejecute `claude plugin install <plugin>@<marketplace>` en su shell sin iniciar una sesión

Cuando una instalación de terminal funciona, `/plugin` imprime un resumen de instalación que comienza con `✓ Installed <plugin>.` y `claude plugin install` imprime `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

Escribió `/plugin ...` en un prompt de shell, y el shell informó que no existe ningún archivo llamado `/plugin`. Bash informa `bash: /plugin: No such file or directory`.

`/plugin` es un comando que escribe dentro de una sesión de Claude Code, no en el prompt de shell. Inicie una sesión y escriba el mismo comando allí:

```shell theme={null}
claude
```

Luego, en el prompt de Claude Code:

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Una instalación exitosa imprime un resumen que comienza con `✓ Installed <plugin>.` Si la instalación en sí falla, su mensaje está bajo [Add a marketplace](#add-a-marketplace) o [Install a plugin](#install-a-plugin).

Para instalar desde el shell sin iniciar una sesión, ejecute `claude plugin install <plugin>@<marketplace>` en su lugar.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

Escribió `/plugin ...` en un prompt de PowerShell, y `/plugin` es un comando de Claude Code, no un programa. Bash y Zsh informan [su propia forma de este error](#zsh-no-such-file-or-directory-plugin).

Use cualquiera de estos en su lugar:

* Ejecute `claude`, luego escriba `/plugin` en el prompt de Claude Code
* Ejecute `claude plugin install <plugin>@<marketplace>` en PowerShell sin iniciar una sesión

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` after `claude plugin ...`
</h3>

Ejecutó `claude plugin install ...` en su shell, y el shell no pudo encontrar `claude` en absoluto. En Windows el mensaje es `'claude' is not recognized as the name of a cmdlet` o `'claude' is not recognized as an internal or external command`.

La causa no es el comando del plugin. O Claude Code no está instalado, o su directorio de instalación no está en su `PATH` en este shell. Siga [`command not found: claude` after installation](/docs/es/troubleshoot-install#command-not-found-claude-after-installation), luego reintente el comando del plugin.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` and command spellings that don't exist
</h3>

Escribió un comando de plugin que vio en algún lugar y obtuvo `Unknown command: /<name>` en una sesión, o `error: unknown command '<name>'` o `error: unknown option '<flag>'` del binario `claude` en su shell.

Hay varios comandos en uso que Claude Code no tiene. La tabla a continuación asigna cada uno al comando real. La [plugin commands reference](/docs/es/plugins/cli-reference) enumera todos los subcomandos e indicadores.

| Escribió                                   | Lo que Claude Code dice                                                      | Use en su lugar                                                                                                                            |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>` para agregar un mercado, o `claude plugin install <plugin>@<marketplace>` para instalar un plugin |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                             |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                   |
| `/plugin add <source>`                     | El panel `/plugin` se abre en la pestaña **Discover**                        | `/plugin marketplace add <source>`                                                                                                         |
| `marketplace.anthropic.com` como fuente    | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` para el mercado oficial                                                                               |

Estos comandos se ven mal pero funcionan:

* `claude plugins` es un alias de `claude plugin`
* `claude plugin remove` es un alias de `claude plugin uninstall`
* `/plugins` y `/marketplace` en una sesión abren el mismo panel que `/plugin`

<h2 id="add-a-marketplace">
  Add a marketplace
</h2>

Un mercado es un catálogo que agrega a Claude Code desde un repositorio git, una URL o una ruta local. Estas entradas cubren los mensajes que obtiene cuando agregar uno falla o una actualización posterior falla.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

Ejecutó `/plugin install <plugin>@claude-plugins-official` en una sesión, y Claude Code informó que no tiene ningún mercado con ese nombre.

El mercado oficial aún no está registrado en esta máquina. Claude Code normalmente lo registra por su cuenta la primera vez que inicia una sesión de terminal interactiva. No se ha ejecutado si solo ha usado Claude Code a través de la extensión de VS Code, y omite o difiere ese paso:

* Cuando una política bloquea la fuente
* Cuando se establece `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`
* Después de un intento fallido que está esperando para reintentar

Los comandos de shell `claude plugin` nunca lo registran por usted.

Agréguelo, luego reintente la instalación:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: claude-plugins-official`, y `/plugin marketplace list` muestra el mercado con su fuente.

Para cualquier otro nombre de mercado en este mensaje, vea [`Marketplace "<name>" not found`](#marketplace-not-found).

La misma cadena también aparece en la pestaña `/plugin` **Errors**, la lista de fallos de carga del panel, cuando un plugin listado en su configuración nombra un mercado que no ha agregado.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

Ejecutó `/plugin install <plugin>@<name>` en una sesión, a menudo desde una línea de instalación que alguien le envió, y Claude Code informó que no tiene ningún mercado con ese nombre.

Si el nombre comienza con `claudeai-`, el mercado se aloja en claude.ai, y lo agrega por nombre desde su shell con `claude plugin marketplace add --claudeai <name>`. Vea [Add a marketplace from claude.ai](/docs/es/plugins/install#add-from-claude-ai).

Para cualquier otro nombre, una línea de instalación nombra un mercado pero no dice dónde se aloja el mercado, y Claude Code no tiene un índice para buscar un nombre de mercado. Pida a quien le envió la línea la fuente del mercado, que es un `owner/repo` de GitHub, una URL de git, o una ruta. Luego [agregue el mercado](/docs/es/plugins/install#add-a-marketplace) y ejecute la línea de instalación nuevamente.

Un mercado que alguien le envía es de terceros, así que [revise el plugin antes de instalarlo](/docs/es/plugins/security#review-a-plugin-before-you-install).

Si ya agregó el mercado, verifique la ortografía contra `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

Ejecutó `/plugin marketplace add <source>` o `claude plugin marketplace add <source>`, y Claude Code respondió `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`.

Claude Code acepta una fuente en una de estas formas:

* Un atajo de `owner/repo` de GitHub
* Una URL `https://` o `http://`
* Una URL SSH `user@host:path`
* Una ruta local que comienza con `./`, `../`, `/`, o `~`

Un nombre simple como `claude-plugins-official` no coincide con ninguno de ellos. Tampoco un nombre de host simple como `marketplace.anthropic.com`.

Vuelva a escribir la fuente en una de las formas aceptadas:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: <name>` cuando la adición funciona.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

Pasó una fuente con una barra que no es `owner/repo`, como `github.com/owner/repo` o una ruta `gitlab.example.com/group/project`. Claude Code la rechazó con este mensaje y una lista de formas aceptadas.

El atajo `owner/repo` es solo de GitHub y tiene que seguir las reglas de nomenclatura de GitHub, por lo que un nombre de host o un segmento de ruta adicional falla. Pase la fuente en la forma que coincida con dónde se aloja el mercado:

* **Un repositorio en cualquier host**: la URL de clonación completa
* **Un `marketplace.json` alojado**: su URL `https://`
* **Un checkout local**: `./path` o una ruta absoluta

Por ejemplo, para agregar el mercado oficial por su URL de clonación, en una sesión:

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Una adición exitosa imprime `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

Pasó una ruta local a `marketplace add`, y nada existe en esa ruta. Una ruta relativa se resuelve contra su directorio actual.

Verifique la ruta resuelta en el mensaje. Luego ejecute el comando desde el directorio desde el que comienza la ruta relativa, o pase una ruta absoluta al directorio del mercado. Una adición exitosa imprime `Successfully added marketplace: <name>`.

Claude Code acepta un directorio que contiene `.claude-plugin/marketplace.json`, o una ruta a un archivo `.json`. Una ruta a cualquier otro archivo falla con `File path must point to a .json file (marketplace.json)`.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code clonó o descargó el mercado pero no encontró `marketplace.json` en la ruta esperada dentro de él. El comando de adición lo informa como `Failed to add marketplace: Marketplace file not found at ...`.

La ubicación predeterminada es `.claude-plugin/marketplace.json` en la raíz del repositorio, y la [marketplace reference](/docs/es/plugins/marketplace-reference) enumera las ubicaciones aceptadas.

La solución difiere para el propietario y para todos los demás:

* **Usted es el propietario del mercado**: coloque el archivo en esa ubicación y vuelva a agregar el mercado
* **Alguien más lo aloja**: pida al propietario la fuente exacta que publican

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` or `HTTPS authentication failed`
</h3>

Agregó o actualizó un mercado desde un repositorio git, y el clonado falló con `Failed to clone marketplace repository:` seguido de una de estas líneas.

Primero verifique el repositorio en sí: un `owner/repo` mal escrito, un repositorio que no existe, o un repositorio privado que no puede ver también termina en este mensaje. Abra la URL del repositorio en su navegador, o ejecute `git ls-remote <url>` en su terminal, para confirmar que existe y tiene acceso.

Si el repositorio es correcto, la causa son las credenciales. Claude Code ejecuta git con prompts interactivos deshabilitados, por lo que no puede pedirle una contraseña, una frase de contraseña de clave, o una credencial de la manera que lo haría su terminal. Si git necesita solicitar, ve `fatal: Cannot prompt because user interactivity has been disabled` o `terminal prompts disabled` en el error original. Solo las credenciales que ya funcionan de forma no interactiva tienen éxito:

* **SSH**: `ssh -T git@<host>` debe tener éxito sin solicitar una frase de contraseña, y el host ya debe estar en `known_hosts`
* **HTTPS**: su asistente de credenciales debe contener un token para el host. Para GitHub, ejecute `gh auth login` y `gh auth setup-git`. Para otro host, almacene un token de acceso personal en su asistente de credenciales de git. Pruebe con `git ls-remote <url>`

Una vez que `git ls-remote` tiene éxito en su terminal sin un prompt, ejecute la adición o actualización nuevamente. Una adición exitosa imprime `Successfully added marketplace: <name>`. Una actualización exitosa imprime `Successfully updated marketplace: <name>` desde su shell, o `✔ Updated 1 marketplace` en una sesión.

Para hacer que Claude Code omita SSH para fuentes `owner/repo` de GitHub, establezca `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Sin él, Claude Code clona esas fuentes sobre SSH cuando una clave SSH para `github.com` parece estar configurada, y vuelve a HTTPS cuando el clonado SSH falla.

Para lo que las actualizaciones automáticas en segundo plano pueden y no pueden hacer con sus credenciales, vea [What background auto-update does with credentials](/docs/es/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

Agregó un mercado sobre SSH desde un host al que nunca se ha conectado, y el clonado falló con esta línea y una sugerencia `ssh -T git@<host>`. Para un host cuya clave cambió, el mensaje es `SSH host key has changed` con una sugerencia `ssh-keygen -R <host>` en su lugar.

Claude Code clona con `StrictHostKeyChecking=yes`, por lo que rechaza un host cuya clave no ha aceptado aún en lugar de aceptar la clave automáticamente. Conéctese una vez desde su terminal para aceptar la huella digital, luego reintente:

```shell theme={null}
ssh -T git@github.com
```

Para un repositorio público, agregue el mercado por su URL `https://` en su lugar para evitar SSH completamente.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

En Windows, agregó un mercado y Claude Code informó `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`.

Claude Code busca `git` en su `PATH` y rechaza ejecutar uno encontrado solo en el directorio actual. Para solucionarlo, instale Git e intente nuevamente:

<Steps>
  <Step title="Install Git for Windows">
    Instale Git para Windows para que `git` esté en su `PATH`.
  </Step>

  <Step title="Open a new terminal">
    Abra una nueva terminal para que se aplique el `PATH` actualizado.
  </Step>

  <Step title="Confirm git runs">
    Confirme que `git --version` imprime una versión.
  </Step>

  <Step title="Retry the add">
    Ejecute el comando `marketplace add` nuevamente.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

Agregó o actualizó un mercado, y falló con `Git clone timed out after 120s`, seguido de una sugerencia para establecer `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

Clonar un mercado, y re-clonar uno para actualizarlo, obtiene 120 segundos por defecto. Para un repositorio grande o una conexión lenta, aumente el límite. El valor está en milisegundos:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Luego reintente en el mismo shell.

Si el repositorio es un monorepo, limite el checkout a los directorios que nombra con `claude plugin marketplace add <source> --sparse <paths>`.

<h3 id="marketplace-updates-keep-failing-offline">
  Marketplace updates keep failing offline
</h3>

Trabaja en un entorno donde el host git del mercado es inaccesible, y cada sesión repite una actualización fallida en segundo plano. Su checkout existente del mercado permanece en su lugar y el inicio no se retrasa.

Cada sesión, para un mercado con [auto-update on](/docs/es/plugins/loading#which-marketplaces-and-plugins-auto-update), Claude Code verifica el host git del mercado para nuevos commits en segundo plano. Cuando esa verificación no puede alcanzar el host, intenta clonar el mercado nuevamente, y sin conexión ese clonado también falla.

Establezca esta variable para omitir el intento de re-clonación y mantener el uso del checkout existente cuando la verificación no puede alcanzar el host:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

Con la variable establecida, Claude Code omite la re-clonación solo para un checkout que ya contiene `.claude-plugin/marketplace.json`. Un mercado que nunca fue clonado o cuyo clonado se detuvo a mitad de camino aún obtiene el intento de clonación, así que agréguelo una vez mientras está en línea.

Para una implementación completamente sin conexión, pre-rellene el directorio de plugins en el tiempo de compilación de imagen con `CLAUDE_CODE_PLUGIN_SEED_DIR` en su lugar, siguiendo [Seed containers and CI](/docs/es/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  Marketplace add fails on a GitHub Enterprise Server host
</h3>

Agregó un mercado desde una URL de GitHub Enterprise Server (GHES) y obtuvo un error de política, o lo agregó desde claude.ai y obtuvo un error de acceso a GitHub.

Ambos casos están en la página de GHES:

* [Un error de política](/docs/es/github-enterprise-server#marketplace-add-fails-with-a-policy-error) significa que su organización restringió las fuentes del mercado y un administrador necesita agregar un `hostPattern` para el host
* [Un error de acceso a GitHub en claude.ai](/docs/es/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) significa que su propia cuenta de GitHub Enterprise aún no está conectada

<h2 id="install-a-plugin">
  Install a plugin
</h2>

Agregó un mercado y ejecutó una instalación, y la instalación se detuvo con un mensaje en lugar de instalar nada. Estas entradas cubren esos mensajes. También cubren los mensajes relacionados que aparecen más tarde en la pestaña `/plugin` **Errors**, o como una pestaña **Discover** vacía, cuando un plugin o su mercado no se puede encontrar, leer o confiar.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

Ejecutó `/plugin install <name>@<marketplace>` o `claude plugin install <name>@<marketplace>`, y el nombre del plugin no está en la copia del catálogo de ese mercado en su máquina.

`claude plugin install` en su shell imprime el mismo mensaje cuando no ha agregado el mercado en absoluto. Si `claude plugin marketplace update <marketplace>` luego responde `Marketplace '<marketplace>' not found`, [agregue el mercado](#add-a-marketplace) primero.

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` with a refresh hint
</h4>

La sugerencia dice `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` o `The marketplace couldn't be refreshed (...)`. Claude Code no actualizó el mercado antes de la búsqueda, como cuando está sin conexión, por lo que su copia del catálogo puede estar desactualizada. Actualice con el nombre del mercado, luego instale nuevamente:

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` imprime `Successfully updated marketplace: <name>`, y `/plugin marketplace update` muestra `✔ Updated 1 marketplace`. Si la instalación reintentada imprime el mismo mensaje, verifique el nombre como [`not found in marketplace` with no hint](#the-message-has-no-hint) describe. [When Claude Code refreshes a marketplace before an install](/docs/es/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) enumera los otros casos donde la actualización no se ejecuta.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` with no hint
</h4>

El nombre es el problema más probable. Abra `/plugin`, vaya a **Discover**, y copie el nombre de la lista.

Antes de v2.1.232, Claude Code actualizaba el mercado nombrado solo después de que la búsqueda fallara, y solo cuando la actualización automática estaba activada para él.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

Ejecutó `/plugin install <name>` sin `@marketplace`, y ningún mercado registrado tiene ese plugin. `claude plugin install <name>` informa `Plugin "<name>" not found in any configured marketplace`.

Sin un nombre de mercado, `claude plugin install` busca en los catálogos que ya tiene y no los actualiza primero, y `/plugin install` actualiza solo los mercados que tienen la actualización automática activada. Nombre el mercado, y Claude Code lo actualiza antes de buscar el plugin:

```text theme={null}
/plugin install <name>@<marketplace>
```

Cuando la instalación funciona, ve `✓ Installed <plugin>.` en una sesión, o `Successfully installed plugin: <plugin>@<marketplace>` desde `claude plugin install`.

Si no sabe qué mercado enumera el plugin, ejecute `/plugin marketplace list` para los mercados que tiene, y navegue por **Discover** en `/plugin` para el nombre del plugin.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

Ejecutó `/plugin install` para un plugin que ya está instalado en el scope de usuario o por configuración administrada, y Claude Code rechazó con `Use '/plugin' to manage existing plugins.` Si escribió el nombre del plugin sin `@<marketplace>`, el mensaje omite `globally`.

El plugin ya está disponible en cada proyecto, por lo que no hay nada que agregar. Para cambiar su [scope](/docs/es/plugins/install), habilitarlo o deshabilitarlo, o configurarlo, abra `/plugin` y vaya a **Installed**.

Un plugin instalado solo en el scope de proyecto o local no activa este mensaje. Claude Code le permite instalarlo también en el scope de usuario, por lo que está disponible en otros proyectos.

`claude plugin install` en su shell imprime un mensaje diferente. Para un plugin ya instalado en el scope de destino, imprime `Plugin "<name>@<marketplace>" is already installed (scope: user)` y sale con 0. Si su directorio de caché falta, el mismo comando lo descarga nuevamente.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

Instaló un plugin cuya entrada de mercado usa un tipo de fuente que esta versión de Claude Code no puede obtener, y Claude Code se detuvo con este mensaje y `Update Claude Code and try again.`

Actualice Claude Code, luego reintente la instalación. Los tipos de fuente están en la [marketplace reference](/docs/es/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

Instaló un plugin que se distribuye como un archivo zip, y Claude Code lo rechazó con esta línea y `The archive was not installed.` La entrada de mercado del plugin usa una fuente [`archive`](/docs/es/plugins/marketplace-reference) con un pin `sha256`, y el resumen del archivo descargado no coincide con el pin.

El mensaje completo se ve así:

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

La solución difiere para el editor y el instalador:

* **Usted publica el plugin**: recompute el resumen del archivo exacto que sirve la URL y actualice el `sha256` en la entrada del mercado. Use `shasum -a 256 my-plugin.zip`, o `Get-FileHash -Algorithm SHA256 my-plugin.zip` en PowerShell
* **Usted instala el plugin**: ejecute `/plugin marketplace update <name>` en una sesión para actualizar el catálogo en caso de que la entrada haya sido corregida, luego reintente la instalación. Si los resúmenes aún no coinciden después de la actualización, pida al propietario del mercado qué archivo fijaron antes de instalar

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Un mercado que agregó anteriormente dejó de cargarse, y también sus plugins. Esta línea aparece en la pestaña `/plugin` **Errors** o en la siguiente actualización.

El mercado está registrado bajo un nombre que está [reservado para mercados oficiales de Anthropic](/docs/es/plugins/marketplace-reference), pero su fuente registrada no es un repositorio de GitHub `anthropics`. Los nombres reservados se re-verifican cada vez que un mercado se carga o se actualiza, por lo que el mercado y los plugins instalados desde él dejan de cargarse.

El mensaje completo nombra el nombre reservado y la solución:

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

La solución difiere para usuarios y editores:

* **Usted usa el mercado**: en su shell, ejecute `claude plugin marketplace remove <name>`, luego agregue el mercado nuevamente desde el repositorio oficial `github.com/anthropics`
* **Usted publica un mercado de terceros que usó el nombre antes de que se reservara**: cámbielo de nombre y pida a los usuarios que lo vuelvan a agregar desde su fuente

Antes de v2.1.205, Claude Code verificaba el nombre solo cuando agregaba el mercado, por lo que una entrada registrada antes de que su nombre se reservara seguía cargándose.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` or `has an invalid manifest file`
</h3>

Claude Code obtuvo el plugin, luego falló al leer su `.claude-plugin/plugin.json`. En el shell, el `<name>` en esta línea puede ser un nombre de directorio temporal; el prefijo `Failed to install plugin "<name>@<marketplace>"` lleva el nombre real del plugin. La redacción dice qué verificación falló:

* **`corrupt manifest file`, seguido de `JSON parse error:`**: el archivo no es JSON válido
* **`invalid manifest file`, seguido de `Validation errors:`**: el archivo se analiza pero falla el esquema, como `name: Invalid input` para un campo requerido faltante

`claude plugin install` informa cualquiera como `Failed to install plugin "<name>@<marketplace>":` y sale con código 1.

El autor del plugin tiene que corregir el archivo, y el plugin no se puede instalar hasta entonces:

* **Si eso es usted**: ejecute `claude plugin validate <plugin-directory>` en su shell para ver el mismo error con la ruta ofensiva, luego corrija el archivo
* **Si no es usted**: informe el mensaje al propietario del mercado

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

La pestaña **Errors** en `/plugin` muestra esto para un plugin habilitado que su mercado enumera por una ruta relativa, como `./plugins/my-plugin`, cuando no existe ningún directorio en esa ruta dentro del mercado. Si mantiene el mercado, corrija la ruta `source` de la entrada o restaure la carpeta. De lo contrario, informe el mensaje al propietario del mercado.

`Marketplace directory not found at path: <path>` significa que el directorio del mercado en sí falta. Para un mercado que agregó desde una ruta local, ese directorio se movió o se eliminó. Restáurelo, o elimine el mercado y agréguelo nuevamente desde su nueva ubicación.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` or `No marketplaces configured`
</h3>

Abrió `/plugin` y la pestaña **Discover** está vacía, o `claude plugin marketplace list` imprimió `No marketplaces configured`.

Ningún mercado está registrado, por lo que no hay catálogo para mostrar. En una sesión, agregue el mercado oficial, `anthropics/claude-plugins-official`:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: claude-plugins-official`, y **Discover** enumera sus plugins. La página [Anthropic marketplaces](/docs/es/plugins/anthropic-marketplaces) enumera los otros mercados que puede agregar.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

Confirmó agregar un mercado a través de [`/plugin install <plugin> --marketplace <source>`](/docs/es/plugins/install#add-a-marketplace-and-install-in-one-command), y el catálogo que Claude Code obtuvo de esa fuente tiene el mismo nombre que un mercado que ya agregó desde una fuente diferente. Claude Code mantiene el mercado existente en lugar de reemplazarlo, y el plugin no se instala.

El mensaje completo se ve así:

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Elija qué fuente desea:

* **El mercado que ya agregó**: instale desde él por nombre con `/plugin install <plugin>@<name>`
* **La nueva fuente**: ejecute `/plugin marketplace remove <name>`, luego reintente la instalación

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

Ejecutó `marketplace add`, y el catálogo en esa fuente tiene el mismo nombre que un mercado que un archivo de configuración ya declara bajo [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces) con una fuente diferente. Claude Code rechaza la adición y no registra nada.

El mensaje termina con la solución: la fuente debe coincidir con la que se declara para este nombre en la configuración, o cambia la declaración. Compare la fuente que pasó contra la entrada `extraKnownMarketplaces` para ese nombre, incluida su `ref`, `path`, y `headers`, luego haga uno de estos:

* **Use la fuente declarada**: agregue el mercado desde la fuente que la entrada de configuración nombra
* **Use la nueva fuente**: edite o elimine la entrada `extraKnownMarketplaces`, luego agregue el mercado nuevamente. Si la configuración administrada la declara, pida a su administrador

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Seleccionó plugins para instalar en el menú `/plugin`, ninguno de ellos se instaló, y el menú se cerró con este resumen de lo que falló.

Algunas razones, como la salida de git después de un clonado fallido, muestran solo su primera línea. Cuando tal razón fue acortada, el resumen termina con `Installing a plugin from its details (Enter) in /plugin shows its full error.`

Qué hacer depende de si el resumen acortó la razón:

* Corrija lo que la razón entre paréntesis nombra
* Cuando la razón fue acortada, ejecute `/plugin`, seleccione el plugin en la pestaña **Discover**, y presione **Enter** para instalarlo desde sus detalles. Si la instalación falla allí, la vista de detalles muestra el error completo

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Cuando instala un plugin, Claude Code descarga una copia fresca de sus archivos y la mueve a la carpeta de esa versión en la [plugin cache](/docs/es/plugins/loading#find-plugins-on-disk). Este mensaje significa que el movimiento falló, generalmente porque otro programa estaba usando la carpeta mientras se ejecutaba la instalación. El código del sistema de archivos aparece entre paréntesis:

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

El mensaje dice qué sucedió con la copia que estaba instalada antes, lo que le dice si el plugin aún funciona:

* `The previously installed copy was moved back`: la versión que tenía aún está instalada
* `had to be removed first`, `was not moved back`, o `could not be moved back`: esa versión del plugin no está instalada hasta que una instalación tenga éxito
* Sin tal oración: no había una copia anterior, por lo que la versión aún no está instalada

En Windows, cuando otro programa mantiene la copia instalada en sí, el mensaje en su lugar dice que esa copia `could not be replaced` y que `It was not replaced and the new copy was discarded`, por lo que la versión que tenía aún está instalada.

Una lista `Left on disk` nombra carpetas apartadas dentro de la caché. Una instalación posterior de esa versión o una limpieza de caché de plugins las elimina, por lo que no necesita eliminarlas.

Para corregir la instalación:

* Cierre otras sesiones de Claude Code, editores y terminales que estén usando la carpeta del plugin bajo `~/.claude/plugins/cache`, luego ejecute la instalación nuevamente
* Cuando el mensaje dice verificar los permisos de la carpeta de caché de plugins, restaure su permiso de escritura en la carpeta que nombra y libere espacio en disco, luego ejecute la instalación nuevamente

<h3 id="dependency-errors">
  Dependency errors
</h3>

Un plugin que declara dependencias puede fallar al instalar, o instalar y permanecer deshabilitado, cuando una dependencia no se puede satisfacer. El mensaje lo alcanza en el momento de la instalación o en el momento de la carga:

* **Durante la instalación**: el rechazo regresa como el mensaje de error de la instalación
* **Cuando el plugin se carga**: el problema aparece en `claude plugin list` y la pestaña `/plugin` **Errors**, y Claude Code mantiene el plugin afectado deshabilitado hasta que lo resuelva

La tabla enumera cada mensaje y su solución. Para declarar dependencias como autor, vea [Plugin dependencies](/docs/es/plugins/dependencies).

| Mensaje                                                                                        | Significado                                                                                               | Cómo resolver                                                                                                                                                                                                                                                         |
| :--------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Dependency "<dep>" is not installed`                                                          | Una dependencia declarada no está instalada.                                                              | Instálela en su shell con `claude plugin install <dep>@<marketplace>`, o desinstale el plugin. Si el mercado de la dependencia aún no está registrado, agréguelo y ejecute `/reload-plugins` en su sesión, que instala las dependencias faltantes que puede resolver. |
| `Dependency "<dep>" is disabled`                                                               | La dependencia está instalada pero desactivada.                                                           | Habilite la dependencia, o desinstale el plugin que la necesita.                                                                                                                                                                                                      |
| `Requires "<dep>" <range>, installed <version>`                                                | La versión de la dependencia instalada está fuera del rango declarado del plugin.                         | Actualice la dependencia a una versión en el rango, o desinstale el plugin.                                                                                                                                                                                           |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                         | Ninguna versión satisface cada rango que la fija. El mensaje enumera los rangos.                          | Desinstale o actualice uno de los plugins en conflicto, o pida al autor ascendente que amplíe su restricción.                                                                                                                                                         |
| `... has version requirements too complex to intersect` o `has an invalid version requirement` | Un rango no es semver válido, o los rangos combinados no se pueden intersectar.                           | Corrija el rango inválido o simplifique las cadenas `\|\|` largas.                                                                                                                                                                                                    |
| `... has no git tag satisfying <range>`                                                        | El repositorio de la dependencia no tiene una etiqueta `<name>--v*` en el rango.                          | Verifique que las etiquetas ascendentes se lancen con esa convención, o relaje el rango.                                                                                                                                                                              |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist` | La dependencia está en un mercado diferente, y la resolución entre mercados está desactivada por defecto. | Instale la dependencia usted mismo en el mismo scope, en su shell con `claude plugin install <dep>@<marketplace>` más el `--scope` en el que está instalando el plugin, luego reintente.                                                                              |

Para ver estos programáticamente, ejecute `claude plugin list --json` en su shell. Los plugins con problemas llevan un campo `errors` con los mensajes y un campo `errorDetails` con un `type` para cada uno: las primeras dos filas son `dependency-unsatisfied` y la tercera es `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Plugin installed but not working
</h2>

La instalación tuvo éxito, pero las skills, hooks, o servidores del plugin no están haciendo nada. Comience con [Plugin doesn't appear or its skills don't show up](#plugin-doesnt-appear-or-its-skills-dont-show-up), que le dice dónde Claude Code informa lo que cargó, luego coincida con el mensaje.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Plugin doesn't appear or its skills don't show up
</h3>

Instaló un plugin y escribió `/` esperando sus skills, o le pidió a Claude que lo use, y nada sucedió.

Verifique el estado del plugin antes de cambiar nada:

<Steps>
  <Step title="Confirm the plugin is installed and enabled">
    Ejecute `/plugin` y abra **Installed**. Confirme que el plugin está listado y habilitado. `claude plugin list` en su shell imprime la misma lista con la versión, scope, y `Status: ✔ enabled` de cada plugin.
  </Step>

  <Step title="Read the Errors tab">
    Abra la pestaña **Errors** en el mismo panel. Cada entrada empareja un mensaje con una línea de orientación. La mayoría de los mensajes en el resto de esta sección provienen de esa pestaña.
  </Step>

  <Step title="Reload if you installed during this session">
    Si el plugin está instalado y sin errores pero lo instaló durante esta sesión, ejecute `/reload-plugins`. Imprime `Reloaded:` con conteos de plugins, skills, agents, hooks, y servidores. Cuando algo falló agrega `N errors during load. Run /plugin for details.`
  </Step>
</Steps>

Si el plugin se carga sin error y sus skills aún no aparecen, el siguiente paso difiere para su propio plugin y para el de alguien más:

* **Un plugin que está construyendo**: vea [Plugin loads but its skills are missing](#plugin-loads-but-its-skills-are-missing)
* **Un plugin que alguien más publicó**: abra **Installed** en `/plugin` y abra el panel de detalles del plugin, que enumera lo que contiene el plugin. Un plugin que no enumera skills allí no tiene ninguno para ofrecer cuando escribe `/`

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

El resumen de instalación en `/plugin` terminó con `Run /reload-plugins to activate.` en lugar de `Plugin is now active.`

Claude Code no activó el plugin durante la instalación, ya sea porque activarlo [invalidaría la caché de prompt](/docs/es/prompt-caching#enabling-or-disabling-a-plugin) o porque el intento de activación falló.

No necesita escribir el comando. El panel se cierra y Claude Code ejecuta `/reload-plugins` para usted, o lo pone en cola hasta que termine la respuesta que se está transmitiendo.

Lea lo que esa recarga imprime:

* **`Reloaded:` con conteos de plugins, skills, agents, hooks, y servidores**: el plugin ahora está activo. Cuando algo falló al cargar, la línea agrega `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**: la recarga agregaría o eliminaría un servidor MCP de plugin, o la herramienta `LSP`, e invalidaría su caché de prompt. Para el caso de LSP la línea comienza `This reload adds the LSP tool` o `This reload removes the LSP tool`. Ejecutelo con `--force` para activar el plugin de todas formas, o inicie una nueva sesión

Antes de v2.1.268, una instalación que no se activó durante la instalación permaneció pendiente hasta que ejecutó `/reload-plugins` usted mismo.

Antes de v2.1.246, el conteo de skills en ese resumen incluía solo las entradas `commands/` de un plugin, por lo que una recarga podría cargar las skills `SKILL.md` de un plugin e informar `0 skills`.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

La pestaña **Errors** muestra esta línea con la orientación `Run /plugin to refresh the plugin cache`. Claude Code tiene un registro de instalación para el plugin, pero el directorio al que apunta el registro falta, por ejemplo después de que limpió la caché.

Reinstale el plugin desde su shell. `claude plugin install <name>@<marketplace>` descarga nuevamente un plugin cuyo directorio de instalación falta aunque su registro exista:

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Luego ejecute `/reload-plugins` en su sesión. La entrada de la pestaña **Errors** desaparece y el plugin está de vuelta bajo **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

Estableció un plugin en `false` en `~/.claude/settings.json`, y su fila en `claude plugin list` o `/plugin` muestra este mensaje seguido de la fuente que lo habilita, como `— project settings enable it, which overrides your user setting`. Un `true` en esa fuente de mayor precedencia está anulando su configuración de usuario.

Para optar por no participar en un plugin habilitado por proyecto en su máquina, establezca el id en `false` en `.claude/settings.local.json`, que tiene mayor precedencia que el archivo del proyecto. Para las otras fuentes que el mensaje puede nombrar, vea [Disabled in user settings but still loads](/docs/es/plugins/loading#disabled-in-user-settings-but-still-loads).

Si `claude plugin list` en su lugar marca el plugin `required by your org`, ningún archivo de configuración está involucrado: su organización marca ese plugin sincronizado como requerido en claude.ai, y se carga incluso si lo deshabilitó anteriormente. Vea [Plugins synced from claude.ai](/docs/es/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

La pestaña **Errors** muestra esta línea para un plugin que el `.claude/settings.json` de su proyecto habilita, con la orientación `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

La configuración de un repositorio puede habilitar un plugin para todos los que lo abren, pero no lo instala. Cuando el plugin proviene de una fuente externa como un repositorio de GitHub o un paquete npm, Claude Code no lo descarga hasta que lo instale usted mismo. Ejecute el comando de la línea de orientación en su shell, luego recargue:

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

Después de ejecutar `/reload-plugins` en su sesión, la entrada de la pestaña **Errors** se ha ido y el plugin está listado bajo **Installed**.

Si su organización pre-instala plugins para usted, lo hace a través de configuración administrada en su lugar. Vea [Pre-install and require plugins](/docs/es/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` and hooks that don't fire
</h3>

Los hooks de un plugin no se ejecutan. O la pestaña **Errors** muestra un fallo de carga para ellos, los hooks se cargan y ve avisos `<Event> hook error` en la transcripción, o un hook se carga sin error y nunca se dispara.

<h4 id="hooks-fail-to-load">
  Hooks fail to load
</h4>

La pestaña **Errors** muestra uno de estos mensajes:

* **`Failed to load hooks from <path>: <reason>`**: `hooks/hooks.json` no es JSON válido o falla el esquema de hooks. La razón nombra el error de análisis o validación. Corrija el archivo. Para detectar un problema de sintaxis JSON en `hooks/hooks.json` antes de publicar el plugin, ejecute `claude plugin validate <plugin-directory>` en su shell
* **`hooks path not found: <path>`**: el campo `hooks` del manifiesto nombra un archivo que no existe en esa ruta relativa a la raíz del plugin. Corrija la ruta o agregue el archivo

<h4 id="hook-error-notices-in-the-transcript">
  `hook error` notices in the transcript
</h4>

Un aviso de la forma `... hook error: Failed with non-blocking status code: <stderr>` significa que el hook se ejecutó y su comando falló. Por ejemplo, `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` significa que el shell que Claude Code generó no pudo encontrar `node`. Instálelo, o asegúrese de que esté en el `PATH` del terminal desde el que inicia `claude`.

Para cualquier otro error, ejecute el comando del hook usted mismo desde el directorio del plugin para ver la salida completa, o capture el stderr completo con [debug logging](/docs/es/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Hook loads but never fires
</h4>

Si un hook se carga sin error pero nunca se dispara, verifique su definición y luego mírelo ejecutarse:

<Steps>
  <Step title="Check the event name">
    Los nombres de eventos distinguen mayúsculas de minúsculas, así que confirme que el suyo coincida exactamente, por ejemplo `PostToolUse`.
  </Step>

  <Step title="Check the matcher">
    Confirme que el `matcher` del hook coincida con el nombre de la herramienta.
  </Step>

  <Step title="Trigger the event on purpose">
    Para un hook `PostToolUse`, pida a Claude que edite un archivo.
  </Step>

  <Step title="Read the debug log">
    Abra el [debug log](/docs/es/hooks#debug-hooks), que registra qué hooks coincidieron. Un hook que se ejecutó aparece allí con su código de salida.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` and MCP servers that don't start
</h3>

Un plugin agrupa un servidor MCP, y la pestaña **Errors** muestra `Invalid MCP server config for "<server>": <error>`, o el servidor está listado pero `/mcp` nunca lo muestra conectado.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

La configuración del servidor pasa la verificación del esquema, pero Claude Code no puede resolverla para esta sesión. El texto después de los dos puntos nombra la causa y decide la solución:

* **`Missing environment variables: <names>`**: establezca esas variables en el shell desde el que inicia Claude Code, luego inicie una nueva sesión
* **`URL is unset or invalid`**: una opción `${user_config.*}` que usa la URL no está establecida. Ejecute `/plugin configure <plugin>` para establecerla
* **`has an invalid MCP url`** o **`headersHelper for MCP server '<server>' references ${user_config.*}`**: la configuración del plugin en sí es culpable. Corrija la `url` o `headersHelper` en la configuración MCP de su plugin, o infórmelo al autor del plugin si el plugin no es suyo. El caso `headersHelper` tiene su propia entrada bajo [plugin command references user\_config](/docs/es/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Server is configured but never connects
</h4>

Ejecute `/mcp` para ver el estado del servidor. Cuando el servidor es saludable, `/mcp` lo enumera como conectado.

Para leer el error que el servidor imprimió mientras se iniciaba, ejecute `claude --debug` y abra el registro en `~/.claude/debug/<session-id>.txt`. El indicador `--debug` no imprime en la terminal.

Una entrada de servidor en `.mcp.json` que falla el esquema no aparece en la pestaña **Errors**. Claude Code descarta ese servidor y registra `Invalid MCP server config for <server> in <path>` solo en ese registro de depuración. Para encontrar la entrada sin cargar el plugin, ejecute `claude plugin validate` en su shell en el directorio del plugin, que lo informa como un error.

Antes de v2.1.281, `claude plugin validate` no verificaba `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Server works with `--plugin-dir` but fails after install
</h4>

Usted es el autor del plugin, y el servidor se inicia cuando carga el plugin desde su directorio de fuente con `--plugin-dir` pero falla una vez que el plugin está instalado.

Claude Code copia un plugin instalado en su caché, por lo que una ruta que solo funciona desde el directorio de fuente se rompe. Escriba rutas dentro del plugin con `${CLAUDE_PLUGIN_ROOT}`.

Para rutas que alcanzan fuera del directorio del plugin, vea [Files the plugin references outside its directory aren't found](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Language server doesn't start, uses too much memory, or reports wrong diagnostics
</h3>

Instaló un [code intelligence plugin](/docs/es/plugins/code-intelligence) y Claude no ve diagnósticos, o el servidor de lenguaje está usando demasiada memoria o reportando errores que no son reales.

<h4 id="language-server-doesn’t-start">
  Language server doesn't start
</h4>

El plugin se conecta a un binario de servidor de lenguaje que instala por separado, y Claude Code lo genera por nombre desde su `PATH`.

La pestaña `/plugin` **Errors** muestra el fallo con su razón, como `Executable not found in $PATH: "<binary>"`, y `claude --debug` lo registra como `LSP server <name> failed to start: <reason>`.

Instale el binario y confirme que está en el `PATH` del terminal desde el que inicia `claude`, por ejemplo con `which typescript-language-server`. Luego inicie una nueva sesión.

<h4 id="language-server-uses-too-much-memory">
  Language server uses too much memory
</h4>

Los servidores de lenguaje como `rust-analyzer` y `pyright` indexan todo el proyecto. Deshabilite el plugin con `/plugin disable <plugin>` en una sesión y confíe en las herramientas de búsqueda integradas de Claude en su lugar.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  False positive diagnostics in a monorepo
</h4>

Un servidor de lenguaje que no está configurado para el espacio de trabajo puede reportar importaciones no resueltas para paquetes internos. No hay nada que corregir en el lado de Claude Code, y los diagnósticos no impiden que Claude edite código.

<h2 id="build-a-plugin">
  Build a plugin
</h2>

Está desarrollando un plugin y cargándolo con `--plugin-dir` o instalándolo desde un mercado local. Estas entradas cubren los fallos que encuentra mientras desarrolla un plugin. Para que las verificaciones se ejecuten después de cada cambio, vea [Test and debug](/docs/es/plugins/create#test-and-debug).

Dos fallos que también alcanzan a los usuarios de un plugin tienen sus entradas bajo [Plugin installed but not working](#plugin-installed-but-not-working):

* **Un hook que no se dispara**: vea [hooks that don't fire](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **Un servidor MCP que no se inicia**: vea [MCP servers that don't start](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

La pestaña **Errors** muestra `commands path not found: <absolute path>` con la orientación `Check that the path in your manifest or marketplace config is correct`. El mismo mensaje aparece para `skills`, `agents`, y `hooks`.

Claude Code resolvió una ruta desde su `plugin.json` o entrada de mercado contra la raíz del plugin y no encontró nada allí. La ruta en el mensaje es la ruta absoluta que verificó, así que compárela con lo que está en el disco. Corrija la ruta o cree el directorio, luego ejecute `/reload-plugins`.

Las rutas en el manifiesto son relativas a la raíz del plugin y comienzan con `./`. Una ruta que se resuelve fuera de la raíz del plugin se informa como `<component> path escapes plugin directory` en su lugar y se descarta.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` at a marketplace root doesn't load the plugins under `plugins/`
</h3>

Inició `claude --plugin-dir <path>` y no ve error, pero las skills, agents, y hooks del plugin no están allí.

`--plugin-dir` toma el directorio raíz del plugin, el que contiene `.claude-plugin/plugin.json` y los directorios de componentes como `skills/`. Si lo apunta a una raíz de mercado en su lugar, Claude Code no lee `marketplace.json`, por lo que un plugin bajo `plugins/` no se carga, y no ve error. Antes de v2.1.281, Claude Code cargaba una raíz de mercado como un plugin vacío nombrado después de ese directorio. Apunte el indicador al directorio del plugin en sí:

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Luego abra **Installed** en `/plugin`, donde el panel de detalles del plugin enumera sus componentes.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Files the plugin references outside its directory aren't found
</h3>

Un plugin funciona desde su directorio de fuente con `--plugin-dir` pero falla después de instalar, con errores sobre una ruta como `../shared-utils`.

Claude Code copia un plugin instalado en su caché y lo carga desde allí, por lo que una ruta que alcanza fuera del directorio propio del plugin no apunta a nada en la caché. Mueva los archivos compartidos dentro del directorio del plugin, o haga referencia a ellos a través de un enlace simbólico dentro de él. Para dónde está la caché y cómo se resuelven las rutas, vea [Find plugins on disk](/docs/es/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` shows forward slashes on Windows
</h3>

En Windows, un hook de plugin recibe `${CLAUDE_PLUGIN_ROOT}` como `C:/Users/you/...` en lugar de `C:\Users\you\...`, y un script que esperaba barras invertidas se rompe.

Claude Code ejecuta hooks de forma de shell a través de Git Bash en Windows y sustituye la raíz del plugin en la forma Win32 de barra inclinada hacia adelante a propósito. Los builtins de Bash, las herramientas MSYS, y los binarios nativos de Windows todos aceptan esa forma.

Si su script necesita barras invertidas, cambie el hook a una de las formas que mantienen rutas nativas, descritas bajo [exec form and shell form](/docs/es/hooks#exec-form-and-shell-form):

* Un hook de forma exec, que genera el proceso directamente con una matriz `args`
* Un hook con `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Plugin loads but its skills are missing
</h3>

Su plugin está listado bajo **Installed** sin errores, pero sus skills no se ofrecen cuando escribe `/`.

Las skills se cargan desde `skills/` en la raíz del plugin y los comandos desde `commands/` en la raíz del plugin. Solo `plugin.json` pertenece dentro de `.claude-plugin/`, y un directorio `skills/` dentro de `.claude-plugin/` no se escanea. Mueva los directorios a la raíz del plugin y ejecute `/reload-plugins`. Después, el panel de detalles del plugin en `/plugin` enumera las skills, y escribir `/` las ofrece.

Cada skill es un directorio que contiene `SKILL.md`. Una entrada `skills` en el manifiesto que apunta a un archivo `SKILL.md` en lugar de su directorio se informa como `path is a file; skills entries must be directories containing SKILL.md`.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Skill loads but Claude never invokes the skill
</h3>

La skill de su plugin se ejecuta cuando escribe su comando `/<plugin>:<skill>`, pero Claude nunca la invoca en respuesta a una solicitud simple.

Verifique estas causas en orden:

* **La skill establece `disable-model-invocation: true`**: con ese campo establecido, solo usted puede invocar la skill. La skill de plantilla en [Create your first plugin](/docs/es/plugins/create#create-your-first-plugin) lo establece. Elimine la línea de una skill que desea que Claude invoque por su cuenta. [Control who invokes a skill](/docs/es/skills#control-who-invokes-a-skill) cubre el campo
* **La descripción no coincide con cómo la gente pregunta**: trabaje a través de las verificaciones en [Skill not triggering](/docs/es/skills#skill-not-triggering)
* **La descripción está truncada**: cuando se instalan muchas skills, Claude Code acorta las descripciones para ajustarse al presupuesto de caracteres del listado, que puede eliminar las palabras clave que Claude necesita para coincidir con una solicitud. Vea [Skill descriptions are cut short](/docs/es/skills#skill-descriptions-are-cut-short)

Para medir con qué frecuencia la skill se dispara en prompts realistas en lugar de verificar uno a la vez, escriba un caso de eval con un [grader `tool_used: Skill`](/docs/es/plugin-evals#create-your-first-eval-suite) y ejecútelo con `claude plugin eval` después de cada cambio de descripción.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` from `claude plugin eval init`
</h3>

Ejecutó `claude plugin eval init` desde un directorio que no es la raíz de un plugin, como su directorio de inicio o la raíz de un repositorio que mantiene el plugin en un subdirectorio. `init` escribe la suite bajo el directorio de trabajo, por lo que se detiene en lugar de crear un directorio `evals/` que el plugin nunca vería.

Cambie a la raíz del plugin, el directorio que contiene `.claude-plugin/plugin.json` o el `SKILL.md` de la skill, y ejecute el comando nuevamente. Para armar la suite en otro lugar a propósito, pase `--eval-dir`. Vea [Test plugins with evals](/docs/es/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  The `userConfig` dialog never appears
</h3>

Su plugin declara opciones `userConfig`, pero no aparece ningún diálogo de configuración cuando lo instala.

La instalación interactiva muestra el diálogo, y el comando de shell toma los valores como indicadores en su lugar:

* **`/plugin install` en una sesión, o la pestaña Discover en `/plugin`**: el diálogo es parte de esta instalación interactiva
* **`claude plugin install` en su shell**: nunca solicita valores `userConfig`. Guarda cualquier valor `--config KEY=VALUE` que pase, y cuando las opciones permanecen sin establecer imprime `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Cuando cualquiera de las opciones sin establecer es requerida, `(M required)` sigue `not yet set`.

Si instaló desde el shell, pase los valores con `--config`, un indicador por opción:

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Cuando cada opción está establecida, la salida de instalación no lleva ninguna línea `not yet set`. Para abrir el diálogo después en su lugar, ejecute `/plugin configure my-plugin@my-marketplace` en una sesión.

Si pasa una clave `--config` que el manifiesto no declara, el plugin aún se instala, y el comando imprime `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` seguido de las claves que el plugin sí declara.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` reports errors
</h3>

Ejecutó `claude plugin validate <path>`, o `/plugin validate <path>` en una sesión, e imprimió `Found N errors` y `Validation failed`, luego salió con código 1.

El validador lee el manifiesto en la ruta que proporciona: `.claude-plugin/plugin.json` para un directorio de plugin, o `.claude-plugin/marketplace.json` para un directorio de mercado. Para un mercado, prefija problemas en el manifiesto propio de una entrada con el índice de entrada, como `plugins[1] plugin.json → json: ...`.

La tabla cubre los mensajes que detienen la validación y dos advertencias, `No frontmatter block found` y `Unknown field '<key>'`, que la detienen solo cuando pasa `--strict`. Otras advertencias, como una descripción faltante, no se enumeran.

| Mensaje                                                                                                  | Causa                                                                             | Solución                                                                                                         |
| :------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | La ruta no tiene manifiesto, o no existe.                                         | Ejecute el comando contra la raíz del plugin o mercado, el directorio que contiene `.claude-plugin/`.            |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | El directorio no tiene manifiesto `.claude-plugin/`.                              | Cree el manifiesto, o apunte al directorio correcto.                                                             |
| `Invalid JSON syntax: <parse error>`                                                                     | El manifiesto, o `hooks/hooks.json`, no es JSON válido.                           | Corrija el JSON. Hasta que corrija `hooks/hooks.json`, una sesión carga el plugin sin los hooks en ese archivo.  |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Una ruta de componente en el manifiesto no existe.                                | Corrija la ruta o cree el directorio.                                                                            |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Una ruta de componente escapa del directorio del plugin.                          | Use rutas dentro de la raíz del plugin.                                                                          |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Una entrada `skills` apunta a `SKILL.md` en lugar de su directorio.               | Apunte al directorio padre, o `.` para un `SKILL.md` a nivel de raíz.                                            |
| `No frontmatter block found` o `YAML frontmatter failed to parse: <error>`                               | Un archivo de skill, agent, o comando tiene frontmatter YAML faltante o inválido. | Agregue o corrija el frontmatter entre delimitadores `---`. Se informa al validar un directorio de plugin.       |
| `Unknown field '<key>'`                                                                                  | El manifiesto tiene un campo que el esquema no define.                            | Elimínelo, o use el nombre que el mensaje sugiere. Claude Code ignora campos desconocidos en el tiempo de carga. |

Ejecute el comando nuevamente después de cada solución hasta que no imprima errores.

Los campos `plugin.json` están en la [manifest reference](/docs/es/plugins/manifest-reference), y los mensajes a nivel de mercado están bajo [Marketplace validation errors](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

El plugin falla al cargar con `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

El plugin tiene su propio `plugin.json`, y su entrada de mercado establece `strict: false` mientras también declara cualquiera de `commands`, `agents`, `skills`, `hooks`, `outputStyles`, o `themes`. Elimine esos campos de la entrada, o establezca `strict: true` en la entrada para que Claude Code los agregue a `plugin.json`. Vea [Strict mode](/docs/es/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Cuando el plugin se carga, el registro `claude --debug` en `~/.claude/debug/<session-id>.txt` registra `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` Nada aparece en la sesión o la pestaña **Errors**.

La ruta `commands` en el manifiesto existe pero no contiene archivos `.md` y no `SKILL.md` en un subdirectorio. Agregue los archivos de comando, o elimine la ruta del manifiesto.

<h2 id="host-a-marketplace">
  Host a marketplace
</h2>

Publica un mercado y un usuario informa un error, o su propia validación falla. Estas entradas son para el propietario del mercado.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugins with relative paths fail in URL-based marketplaces
</h3>

Los usuarios agregaron su mercado con una URL `https://example.com/marketplace.json`. Las instalaciones de plugins cuya `source` es una ruta relativa, como `./plugins/my-plugin`, fallan con `its marketplace entry path does not stay inside the marketplace directory`. Los plugins ya instalados fallan al cargar con `Plugin source path refused`. Ambos mensajes tienen una [entrada de referencia de error](/docs/es/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Cuando un usuario agrega un mercado basado en URL, Claude Code descarga solo el archivo `marketplace.json` en sí. No obtiene archivos de plugin por ruta relativa desde ese servidor, por lo que una ruta relativa en una entrada apunta a un directorio que nunca fue obtenido. Dé a cada entrada una fuente que Claude Code pueda obtener por su cuenta, como un repositorio de GitHub:

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

Alternativamente, aloje el mercado en un repositorio git y diga a los usuarios que lo agreguen con la URL del repositorio. Para una fuente de git, Claude Code clona todo el repositorio, por lo que las rutas relativas se resuelven. Los tipos de fuente están en la [marketplace reference](/docs/es/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Marketplace validation errors
</h3>

Ejecutó `claude plugin validate .` desde su directorio de mercado e informó errores o advertencias en el archivo del mercado en sí.

`claude plugin validate` también valida cada entrada cuya `source` es una ruta local y advierte cuando la `version` de la entrada no coincide con el manifiesto propio del plugin.

La tabla enumera los mensajes a nivel de mercado. Los mensajes a nivel de entrada son los mensajes de plugin bajo [`claude plugin validate` reports errors](#claude-plugin-validate-reports-errors), prefijados con `plugins[N] plugin.json →`.

| Mensaje                                                                                                                  | Tipo        | Solución                                                                                                                                                        |
| :----------------------------------------------------------------------------------------------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Duplicate plugin name "<name>" found in marketplace`                                                                    | Error       | Dé a cada plugin un `name` único.                                                                                                                               |
| `Path contains "..": <path>` bajo `plugins[N].source`                                                                    | Error       | Use rutas relativas a la raíz del mercado sin segmentos `..`.                                                                                                   |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                         | Error       | Elimine el carácter del nombre, como un escape o una nueva línea.                                                                                               |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                              | Error       | Elimine el carácter del `name` del plugin.                                                                                                                      |
| `Marketplace has no plugins defined`                                                                                     | Advertencia | Agregue al menos una entrada a `plugins`.                                                                                                                       |
| `No marketplace description provided`                                                                                    | Advertencia | Agregue una `description` a nivel superior.                                                                                                                     |
| `Plugin name "<name>" is not kebab-case` bajo `plugins[N] plugin.json → name`                                            | Advertencia | Cambie el nombre a letras minúsculas, dígitos, y guiones. Claude Code acepta otras formas, pero la sincronización del mercado de claude.ai las rechaza.         |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                         | Advertencia | Actualice la entrada para que coincida con `plugin.json`, que es autoritario en el tiempo de instalación.                                                       |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                | Advertencia | Cambie el nombre del mercado. La sincronización de mercado administrada de Claude Desktop rechaza `org`, `org-provisioned`, y `unknown` en cualquier mayúscula. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` o `Plugin name "<name>" is not accepted by Claude Desktop` | Advertencia | Cambie el nombre a como máximo 128 caracteres de letras, dígitos, `.`, `_`, y `-`, comenzando con una letra o dígito.                                           |

Antes de v2.1.247, un nombre de mercado que contiene caracteres de control o de formato bidireccional se informaba solo como `Marketplace name impersonates an official Anthropic/Claude marketplace`.

<h2 id="blocked-by-your-organization">
  Blocked by your organization
</h2>

Su organización implementa configuración administrada que restringe plugins, y un comando fue rechazado con un mensaje de política. Estas entradas nombran la configuración detrás de cada rechazo para que sepa qué pedir a su administrador. Para el lado del administrador, vea [Manage plugins for your organization](/docs/es/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

Ejecutó `/plugin marketplace add`, `update`, o una instalación, y Claude Code rechazó con esta línea. Para una fuente de GitHub o git, el host sigue la fuente entre paréntesis, como en `'github:owner/repo' (github.com)`.

Su administrador estableció `blockedMarketplaces` o `strictKnownMarketplaces` en configuración administrada, y esta fuente no está permitida. Pida a su administrador que permita la fuente, o agregue una de las fuentes permitidas que el mensaje enumera.

Haga coincidir el resto del mensaje para ver qué tipo de política bloqueó la fuente:

* **`Allowed sources: <list>`**: el bloqueo proviene de la lista de permisos `strictKnownMarketplaces` en lugar de la lista de bloqueos `blockedMarketplaces`
* **`No external marketplaces are allowed.`**: la lista de permisos `strictKnownMarketplaces` está vacía
* **Un `Tip:` que el atajo asume github.com**: la lista de permisos permite un host de git por nombre de host, y el atajo `owner/repo` que pasó apunta a github.com. Si el repositorio vive en su host interno, agréguelo nuevamente con su URL completa, como `git@your-git-host.com:owner/repo.git`

Un mercado que agregó antes de que la política se volviera más restrictiva también deja de actualizarse, porque la política se aplica en cada actualización.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

La pestaña **Errors** muestra esta línea, o `Marketplace "<name>" is blocked by enterprise policy`, para un mercado que ya tiene registrado.

La misma configuración administrada que bloquea una [marketplace source](#marketplace-source-is-blocked-by-enterprise-policy) se aplica en el tiempo de carga. `strictKnownMarketplaces` no incluye este mercado, o `blockedMarketplaces` lo nombra, por lo que Claude Code deja de cargarlo y sus plugins. Para la variante de lista de permisos, la línea de orientación muestra las fuentes permitidas, o `Contact your administrator to configure allowed marketplace sources`. Para la variante de lista de bloqueos dice `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Una instalación fue rechazada con esta línea, una habilitación con la misma línea terminando `cannot be enabled`, o una instalación o actualización con una que nombra la razón: `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy`, o `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

La configuración administrada bloquea este plugin, su mercado, o una dependencia que necesita. Pida a su administrador qué entrada se aplica. Una dependencia bloqueada significa que el plugin no puede instalar hasta que el mercado de la dependencia sea permitido.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

Inició `claude` con `--plugin-dir`, `--plugin-url`, `--agents`, o `--mcp-config`. Claude Code salió con este mensaje y `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

Su administrador estableció `disableSideloadFlags` en configuración administrada, que desactiva los indicadores que cargan plugins, agents, y servidores desde rutas arbitrarias. Cargue el plugin desde un mercado aprobado en su lugar, o pida a su administrador que elimine la configuración.

Un mensaje relacionado en la pestaña `/plugin` **Errors** es `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. La configuración administrada habilita o deshabilita ese plugin por nombre, y Claude Code ignora su copia `--plugin-dir` de él para que el indicador no pueda anular la política.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

Ejecutó `claude plugin init` o `claude plugin enable`, y se detuvo con esta línea. El mensaje nombra `strictKnownMarketplaces or blockedMarketplaces` y pide a su administrador que agregue `{"source":"skills-dir"}` a `strictKnownMarketplaces` o lo elimine de `blockedMarketplaces`.

La fuente `skills-dir` representa plugins que Claude Code carga desde su directorio `~/.claude/skills/`. Pida a su administrador que haga el cambio que el mensaje nombra.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

Instaló o actualizó un plugin con una fuente `command`, y se detuvo con esta línea y `The plugin was not installed or updated and its command was not run.`

Su administrador estableció `disableCommandPluginSources`, por lo que Claude Code rechaza ejecutar el comando declarado por mercado que produce el plugin. Establecer `allowManagedHooksOnly` solo tiene el mismo efecto cuando `disableCommandPluginSources` no está establecido. Pida a su administrador si el plugin puede publicarse desde un tipo de fuente que la política permite.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

Ejecutó `claude plugin marketplace update <name>`, y falló con `Marketplace '<name>' is seed-managed (<dir>)` y una sugerencia para pedir a su administrador.

Un operador pre-rellenó este mercado a través de `CLAUDE_CODE_PLUGIN_SEED_DIR`, y Claude Code trata un mercado administrado por semilla como de solo lectura. Una `marketplace update` masiva lo omite y actualiza los otros.

Para cambiar el contenido del mercado, pida a la persona que mantiene la imagen de semilla que lo actualice. Para el procedimiento, vea [Seed containers and CI](/docs/es/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Next steps
</h2>

* [Plugin loading reference](/docs/es/plugins/loading): por qué los scopes, la caché, y la precedencia se comportan de la manera que lo hacen
* [Plugin commands reference](/docs/es/plugins/cli-reference): indicadores, valores predeterminados, salida, y códigos de salida para los comandos `claude plugin`
* [Install and manage plugins](/docs/es/plugins/install): los pasos de instalación desde el principio
* [Manage plugins for your organization](/docs/es/plugins/org#troubleshoot-policy): solución de problemas del lado de la política para administradores
