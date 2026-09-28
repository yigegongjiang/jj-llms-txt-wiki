> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Instalar y administrar plugins

> Instale plugins de Claude Code desde un marketplace en cualquier superficie que utilice, elija un alcance de instalación y actualícelos o elimínelos más tarde.

Instalar un plugin añade sus skills, agentes, hooks y servidores MCP a Claude Code en su máquina.

Esta página es para cualquiera que use plugins en su propia máquina o cuenta, ya sea en la terminal, la aplicación de escritorio, un IDE o una sesión en la nube: cubre la instalación, la elección de un alcance, la adición de marketplaces y el mantenimiento de los plugins actualizados.

<Note>
  Estos casos se tratan en otras páginas:

  * **Utiliza claude.ai chat o Cowork, no Claude Code**: consulte [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code imprimió un error**: encuéntrelo en [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting)
</Note>

Comience con [Instalar un plugin](#install-a-plugin). Si alguien le envió un comando de instalación cuyo nombre `@` no es `claude-plugins-official`, [agregue ese marketplace](#add-a-marketplace) primero.

<h2 id="install-a-plugin">
  Instalar un plugin
</h2>

Como ejemplo, esta sección instala [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) desde [el marketplace oficial de Anthropic](/docs/es/plugins/anthropic-marketplaces), que añade comandos para hacer commits, hacer push y abrir pull requests.

Los mismos pasos instalan cualquier otro plugin: sustituya su nombre y el nombre de su marketplace dondequiera que aparezcan `commit-commands` y `claude-plugins-official`. Si ese plugin proviene de un marketplace diferente, [agregue el marketplace](#add-a-marketplace) primero.

Elija la pestaña para donde ejecuta Claude Code.

<Tabs>
  <Tab title="Terminal">
    Inicie Claude Code con `claude` en su proyecto, luego:

    <Steps>
      <Step title="Abra los detalles del plugin con el comando de instalación">
        Ejecute `/plugin install` con el nombre del plugin y el marketplace. En una sesión, este comando no instala de inmediato: abre el panel `/plugin` en los detalles de ese plugin para que pueda revisarlo y elegir un alcance primero.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Para explorar en su lugar, ejecute `/plugin` sin nombre de plugin: el panel se abre en la pestaña **Discover**, que enumera plugins de cada marketplace que ha agregado, y puede escribir para buscar, luego presionar **Enter** en un plugin para abrir sus detalles.
      </Step>

      <Step title="Revise lo que añade el plugin">
        El panel de detalles muestra la descripción del plugin. También puede mostrar:

        * **Will install**: los comandos, agentes, skills, hooks y servidores MCP y LSP que añade el plugin.
        * **Last updated**: se muestra para un plugin en el marketplace oficial de Anthropic.
        * **Context cost**: para un plugin en el marketplace oficial de Anthropic, dos estimaciones de tokens. **Every turn** es lo que el plugin añade a cada mensaje que envía, y **When invoked** es lo que sus skills y agentes añaden una vez que Claude los carga. Las estimaciones aparecen cuando abre el plugin nombrando su marketplace, como hace el comando del paso 1, o desde la pestaña **Marketplaces**. El panel de detalles al que llega desde la lista **Discover** no las muestra.

        Los plugins de un marketplace local o personalizado pueden mostrar `Components will be discovered at installation` en su lugar.

        Un plugin puede ejecutar hooks y servidores MCP, así que lea el panel antes de instalar. Consulte [Plugin security and trust](/docs/es/plugins/security).
      </Step>

      <Step title="Elija un alcance">
        Seleccione una de las tres opciones de instalación:

        * **Install for you (user scope)**: obtiene el plugin en cada proyecto en esta máquina
        * **Install for all collaborators on this repository (project scope)**: está habilitado para todos los que trabajan en este repositorio
        * **Install for you, in this repo only (local scope)**: lo obtiene en este repositorio solamente

        [Choose an install scope](#choose-an-install-scope) dice qué archivo de configuración escribe cada uno y cuál se aplica cuando el mismo plugin se establece en más de uno.

        Después de seleccionar un alcance, Claude Code instala el plugin junto con cualquier dependencia que declare, luego imprime un resumen de instalación.
      </Step>

      <Step title="Lea el resumen de instalación">
        La última oración del resumen le dice si el plugin es utilizable en esta sesión aún:

        * **Active now**: `Plugin is now active.` No se necesita recarga.
        * **Reload needed**: `Run /reload-plugins to activate.` El panel se cierra y Claude Code ejecuta esa recarga por usted. Si la recarga [invalidaría el prompt cache](/docs/es/prompt-caching#enabling-or-disabling-a-plugin), advierte y deja el plugin pendiente en su lugar. Ejecute `/reload-plugins --force` para activarlo de todas formas, lo que cuesta una solicitud sin caché.
        * **Load failed**: `The plugin couldn't be loaded`. Abra la pestaña **Errors** en `/plugin` para la razón, luego consulte [After install: plugin not working](/docs/es/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Confirme que el plugin funciona">
        Escriba `/` y busque los skills del plugin bajo su nombre, en la forma `/<plugin>:<skill>`. Para `commit-commands`, aparece `/commit-commands:commit`. Otros dos lugares también enumeran el plugin:

        * Abra la pestaña **Installed** en `/plugin`, que enumera el plugin con su alcance.
        * En su shell, ejecute `claude plugin list`, que imprime la misma lista con líneas `Version`, `Scope` y `Status`.

        Si `/commit-commands:commit` no aparece, consulte [After install: plugin not working](/docs/es/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    La instalación desde cualquier otro marketplace requiere un paso extra primero: [agregue el marketplace](#add-a-marketplace). Claude Code añade el marketplace oficial de Anthropic por usted la primera vez que inicia una sesión de terminal interactiva, por lo que el ejemplo omite ese paso. Si encontró un plugin en [claude.com/marketplace](https://claude.com/marketplace), su botón **Claude Code** copia el comando de instalación en su [forma de shell](#install-from-your-shell), `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Desktop app">
    En una sesión local o SSH en la pestaña **Code** de la aplicación de escritorio:

    <Steps>
      <Step title="Abra el navegador de plugins">
        Haga clic en el botón **+** junto a la caja de prompt y seleccione **Plugins**, luego **Add plugin**. Se abre el navegador de plugins con los plugins de sus marketplaces.
      </Step>

      <Step title="Seleccione el plugin">
        Encuentre `commit-commands` y selecciónelo.
      </Step>

      <Step title="Elija un alcance">
        Elija un [alcance](#choose-an-install-scope): su cuenta de usuario, este proyecto o solo local.
      </Step>
    </Steps>

    Para habilitar, deshabilitar o desinstalar más tarde, use **+ > Plugins > Manage plugins**. El navegador de plugins no está disponible en las sesiones en la nube de la aplicación de escritorio. Consulte [Install plugins in the desktop app](/docs/es/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    En el panel de Claude Code en VS Code:

    <Steps>
      <Step title="Abra Manage plugins">
        Escriba `/plugins` en la caja de prompt para abrir **Manage plugins**.
      </Step>

      <Step title="Instale el plugin">
        En la pestaña **Plugins**, busque `commit-commands` y haga clic en **Install**. Si la pestaña no enumera plugins, agregue primero `anthropics/claude-plugins-official` en la pestaña **Marketplaces**.
      </Step>

      <Step title="Elija un alcance">
        Elija un [alcance](#choose-an-install-scope): **Install for you**, **Install for this project** o **Install locally**.
      </Step>
    </Steps>

    Sus cambios se aplican a las sesiones abiertas sin reinicio. Consulte [Manage plugins in VS Code](/docs/es/vs-code#manage-plugins).
  </Tab>

  <Tab title="Cloud session">
    Una [sesión en la nube](/docs/es/cloud-environments), incluyendo [el navegador en claude.ai/code](/docs/es/claude-code-on-the-web), no tiene navegador de plugins y no carga los plugins que instaló en su propia máquina o los que el `.claude/settings.json` de su repositorio activa. Para plugins que su organización distribuye a través de configuración administrada, consulte [Manage plugins for your organization](/docs/es/plugins/org).

    Consulte [qué partes de su configuración también están disponibles en una sesión en la nube](/docs/es/cloud-environments#what-carries-over-from-your-setup) para el resto de su configuración.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Elija un alcance de instalación
</h3>

El alcance de instalación de un plugin decide quién obtiene el plugin y qué archivo de configuración lo registra como habilitado:

* **User scope**: el plugin está habilitado para usted en cada proyecto en esta máquina. La entrada va en `enabledPlugins` en `~/.claude/settings.json`.
* **Project scope**: el plugin está habilitado para todos los que trabajan en este repositorio. La entrada va en `.claude/settings.json`, que usted confirma.
* **Local scope**: el plugin está habilitado para usted en este repositorio solamente. La entrada va en `.claude/settings.local.json`.

Algunos plugins están configurados por su autor para comenzar desactivados, a través del campo [`defaultEnabled`](/docs/es/plugins/manifest-reference#defaultenabled). Tal plugin se instala pero permanece desactivado hasta que lo active con `claude plugin enable <name>` en su shell, o desde la pestaña **Installed** de `/plugin` en una sesión.

Cuando el mismo plugin se establece en varios alcances, la configuración local anula la configuración del proyecto, y la configuración del proyecto anula la configuración del usuario. Consulte [Find where a plugin is enabled](/docs/es/plugins/loading#find-where-a-plugin-is-enabled) para la regla completa.

La terminal, las sesiones locales de la aplicación de escritorio y la extensión VS Code en una computadora leen los mismos archivos de configuración, por lo que un plugin que instale en alcance de usuario en cualquiera de ellos está disponible en los otros dos.

<h3 id="other-places-you-run-claude-code">
  JetBrains, ejecuciones no interactivas y el Agent SDK
</h3>

Algunos lugares donde ejecuta Claude Code no tienen su propio navegador de plugins:

* **JetBrains IDEs**: el plugin de JetBrains ejecuta Claude Code en la terminal del IDE, así que use los pasos de la pestaña **Terminal** allí.
* **`claude -p` y otras ejecuciones no interactivas**: `/plugin` no se ejecuta, y Claude responde `/plugin isn't available in this environment.` Los plugins que ya instaló sí se cargan. Instálelos y adminístrelos desde su shell con [comandos `claude plugin`](#install-from-your-shell).
* **Agent SDK**: cargue plugins a través de la opción de plugin del SDK. Consulte [Load plugins in the Agent SDK](/docs/es/agent-sdk/plugins).

Si Claude Code informa que un plugin habilitado en el `.claude/settings.json` del repositorio no está instalado, consulte [Enabled in project settings but not installed](/docs/es/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Si es un autor de plugins que prueba una copia de su plugin en disco, inicie Claude Code desde su shell con `--plugin-dir` para cargarlo durante una sesión en lugar de instalarlo. Consulte [Flags that load a plugin for one session](/docs/es/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Plugins de su cuenta claude.ai
</h3>

Su cuenta claude.ai es una fuente separada de plugins, junto con los marketplaces desde los que instala:

* **What arrives**: cada plugin que activa para su cuenta claude.ai, y cada plugin que su organización activa para sus miembros. En una sesión de terminal se sincronizan en segundo plano cada vez que inicia Claude Code mientras está conectado con esa cuenta; en sesiones de Cowork se descargan cuando comienza la sesión.
* **Where you see them**: en `/plugin` y `claude plugin list` bajo el ID `<name>@synced`. Puede desactivar uno en su propio alcance a menos que su organización lo requiera.
* **What doesn't go the other way**: los plugins que instala con `/plugin` o `claude plugin install` permanecen en esta máquina y no se añaden a su cuenta claude.ai.

Para el tiempo de sincronización, requisitos de inicio de sesión y desactivación de sincronización, consulte [Plugins synced from claude.ai](/docs/es/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Instalar desde su shell
</h3>

Ejecute `claude plugin install` en su shell para instalar un plugin sin iniciar una sesión de Claude Code, por ejemplo desde un script de configuración.

* **Scope**: alcance de usuario por defecto. Pase `--scope project` o `--scope local` para cambiarlo.
* **When the plugins load**: los plugins que instala se cargan la próxima vez que inicia Claude Code, o cuando ejecuta `/reload-plugins` en una sesión que ya está abierta.
* **The marketplace must be added first**: en una máquina donde nadie ha abierto una sesión interactiva de Claude Code aún, el marketplace oficial no está registrado, así que un script que instala desde él ejecuta `claude plugin marketplace add anthropics/claude-plugins-official` antes de la instalación.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

El comando imprime `Successfully installed plugin: formatter@your-org (scope: project)` cuando termina.

Algunos plugins se instalan ejecutando un comando que su marketplace nombra, llamado una [fuente `command`](/docs/es/plugins/marketplace-reference#command-plugin-source). Claude Code le muestra ese comando y le pide que lo acepte antes de ejecutarlo. Un script no tiene a nadie para responder ese prompt, así que pase `--yes` allí para aceptarlo.

Para cada bandera `claude plugin install`, consulte [plugin install](/docs/es/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Agregue un marketplace
</h2>

Solo necesita esta sección cuando el plugin que desea no está en el marketplace oficial de Anthropic, por ejemplo uno que un colega publicó o uno del marketplace comunitario de Anthropic.

Un marketplace es un catálogo de plugins, y Claude Code tiene que conocer un marketplace antes de poder instalar desde él. Agrega un marketplace una vez. Después de eso, sus plugins aparecen en la pestaña **Discover** e instalan con `/plugin install <plugin>@<marketplace>` en una sesión o `claude plugin install <plugin>@<marketplace>` en su shell, donde `<marketplace>` es el nombre bajo el cual se registró el marketplace. Para hacer ambos en un paso, consulte [Add a marketplace and install in one command](#add-a-marketplace-and-install-in-one-command).

En una sesión de Claude Code, ejecute `/plugin marketplace add` seguido de la fuente del marketplace: un repositorio de GitHub, un repositorio git en cualquier host, un directorio o archivo local, o un `marketplace.json` alojado.

| Fuente                            | Lo que escribe                                                                                                                                                                                                                                  | Ejemplo                                                                                                                               |
| :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| Repositorio de GitHub             | `owner/repo`. Agregue `#ref` para fijar una rama o etiqueta.                                                                                                                                                                                    | `/plugin marketplace add anthropics/claude-code`, o `/plugin marketplace add your-org/plugins#v1.2.0` para fijar la etiqueta `v1.2.0` |
| Repositorio Git en cualquier host | La URL de clonación completa. Agregue `#ref` para fijar una rama o etiqueta.                                                                                                                                                                    | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                           |
| Directorio o archivo local        | Una ruta relativa o absoluta a un directorio que contiene `.claude-plugin/marketplace.json`, o al archivo JSON en sí. Comience una ruta relativa con `./` o `../`, porque Claude Code lee un `name/name` desnudo como un repositorio de GitHub. | `/plugin marketplace add ./my-marketplace`                                                                                            |
| `marketplace.json` alojado        | Su URL `https://`                                                                                                                                                                                                                               | `/plugin marketplace add https://example.com/marketplace.json`                                                                        |

Desde su shell, `claude plugin marketplace add` toma las mismas fuentes.

<Tip>
  `/plugin market` también funciona como una forma más corta de `/plugin marketplace`.
</Tip>

Incluya el prefijo `https://` en cada URL, o use la forma `git@host:path` para SSH. Si escribe un `gitlab.example.com/your-group/your-marketplace.git` desnudo, Claude Code lo lee como abreviatura de GitHub `owner/repo` y lo rechaza.

Cuando el comando tiene éxito, imprime `Successfully added marketplace: <name>`, y los plugins del marketplace aparecen en la pestaña **Discover** la próxima vez que abre `/plugin`, sin necesidad de recarga. Si falla, haga coincidir el mensaje de error en [Troubleshoot plugins](/docs/es/plugins/troubleshooting#add-a-marketplace).

<h3 id="add-a-marketplace-and-install-in-one-command">
  Agregue un marketplace e instale en un comando
</h3>

Para instalar un plugin desde un marketplace que aún no ha agregado, ejecute `/plugin install` en una sesión de Claude Code y nombre la fuente del marketplace con `--marketplace`. Requiere Claude Code v2.1.275 o posterior.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

La fuente toma [las mismas formas que `/plugin marketplace add`](#add-a-marketplace), como GitHub `owner/repo`, una URL git, o una ruta local, excepto que no puede contener espacios. Dé el nombre del plugin por sí solo, sin un sufijo `@marketplace`.

Si aún no ha agregado ese marketplace, Claude Code muestra la fuente que resolvió y le pide que confirme antes de agregarlo. Una vez que se agrega el marketplace, los detalles del plugin se abren y elige un [alcance de instalación](#install-a-plugin). Si la fuente coincide con un marketplace que ya ha agregado, Claude Code omite la confirmación y abre los detalles del plugin en ese marketplace.

<h3 id="add-a-private-marketplace">
  Agregue un marketplace privado
</h3>

Un marketplace privado es uno en un repositorio al que necesita credenciales para clonar, en GitHub o cualquier otro host git. Lo agrega con el mismo comando `/plugin marketplace add` o `claude plugin marketplace add` que uno público. Claude Code lo clona con las credenciales git ya en su máquina y nunca solicita, así que cada forma de conectarse tiene un requisito:

* **HTTPS**: sus ayudantes de credenciales git se aplican, así que el acceso que configuró con `gh auth login`, el Keychain de macOS, o `git-credential-store` funciona. Los prompts interactivos se suprimen, así que un host al que nunca se ha autenticado falla en lugar de pedir una contraseña.
* **SSH**: el host ya debe estar en su archivo `known_hosts` y la clave debe funcionar sin un prompt de frase de contraseña, porque los prompts de huella digital del host y frase de contraseña también se suprimen.
* **Abreviatura de GitHub `owner/repo`**: Claude Code verifica si su clave SSH se autentica en `github.com`, luego clona sobre SSH si lo hace y sobre HTTPS si no. Establezca [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/es/env-vars#variables) para omitir esa verificación y siempre clonar sobre HTTPS.

Las mismas credenciales se aplican cuando ejecuta `/plugin install`, `/plugin marketplace update` y `claude plugin update`.

En un host de GitHub Enterprise Server, consulte [Plugin marketplaces on GHES](/docs/es/github-enterprise-server#plugin-marketplaces-on-ghes) para las credenciales que cada operación necesita.

Si su organización registra el marketplace para usted a través de configuración administrada, no lo agrega usted mismo. Consulte [Pre-install and require plugins](/docs/es/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Agregue un marketplace desde claude.ai
</h3>

En sesiones de terminal donde [los plugins se sincronizan desde su cuenta claude.ai](/docs/es/plugins/loading#synced-plugins), claude.ai también puede enumerar marketplaces de plugins para usted, como la biblioteca de plugins de su organización y sus propias cargas de claude.ai. Agrega uno de estos por su nombre en lugar de por una fuente. Agregar un marketplace desde claude.ai requiere Claude Code v2.1.273 o posterior.

Agregue un marketplace de claude.ai desde el panel `/plugin` o desde su shell:

* **Inside a session**: ejecute `/plugin` y vaya a la pestaña **Marketplaces**, que enumera los marketplaces desde claude.ai. Seleccione uno allí para agregarlo.
* **From your shell**: ejecute `claude plugin marketplace list`, que los imprime en una sección `From claude.ai:`. Luego ejecute `claude plugin marketplace add` con la bandera `--claudeai` y el nombre mostrado en la lista.

Por ejemplo, este comando agrega un marketplace llamado `claudeai-organization-library`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code registra el marketplace bajo un nombre local que comienza con `claudeai-`, derivado del nombre que claude.ai lo enumera. Por ejemplo, un marketplace enumerado como "Organization library" se convierte en `claudeai-organization-library`. Instale sus plugins por ese nombre, por ejemplo con `claude plugin install <plugin>@claudeai-organization-library`.

Si cierra sesión, o inicia sesión en una organización claude.ai diferente, el marketplace permanece configurado pero no muestra plugins, y los plugins que ya instaló desde él siguen cargándose.

La sección `From claude.ai:` también puede enumerar marketplaces basados en git compartidos a través de claude.ai, e imprime una fuente para cada uno de esos. Agréguelos por esa fuente como en [Add a marketplace](#add-a-marketplace), no con `--claudeai`.

<h2 id="manage-installed-plugins">
  Administre plugins instalados
</h2>

La pestaña **Installed** en `/plugin` enumera sus plugins con acciones para habilitar, deshabilitar, actualizar o desinstalar cada uno. En una sesión de Claude Code, ejecute `/plugin` y presione **Tab** para llegar a ella, o ejecute `/plugin enable`, `/plugin disable` o `/plugin uninstall` para abrir el panel y hacer ese cambio allí. Los plugins deshabilitados se agrupan bajo un encabezado contraído en la parte inferior de la lista. Use estas teclas en la lista:

* Escriba para filtrar por nombre o descripción.
* Presione **Space** para habilitar o deshabilitar el plugin seleccionado, y **f** para marcarlo como favorito.
* Presione **Enter** para abrir los detalles de un plugin. El menú allí ofrece **Disable plugin** o **Enable plugin**, **Update now** y **Uninstall**. Los plugins que toman configuración también ofrecen **Configure options**.

La pestaña también puede mostrar plugins en alcance **Managed**. Su organización instaló esos a través de [configuración administrada](/docs/es/settings#settings-files), y no puede habilitarlos, deshabilitarlos o desinstalarlos aquí.

Para un plugin sincronizado que su organización requiere en claude.ai, consulte [Manage plugins synced from claude.ai](#manage-plugins-synced-from-claude-ai).

Cuando cierra el panel `/plugin` con cambios pendientes que hizo en él, Claude Code ejecuta `/reload-plugins` para usted para aplicarlos. Si la recarga [invalidaría el prompt cache](/docs/es/prompt-caching#enabling-or-disabling-a-plugin), advierte y deja los cambios pendientes en su lugar. Ejecute `/reload-plugins --force` para aplicarlos de todas formas.

<h3 id="manage-plugins-synced-from-claude-ai">
  Administre plugins sincronizados desde claude.ai
</h3>

La pestaña **Installed** en `/plugin` también enumera los [plugins sincronizados desde su cuenta claude.ai](/docs/es/plugins/loading#synced-plugins), con `synced` como su fuente. Los plugins sincronizados aparecen en sesiones de terminal en Claude Code v2.1.273 o posterior.

* **Enable or disable**: use la pestaña **Installed**, a menos que su organización haya marcado el plugin como requerido.
* **Remove**: desactive el plugin en claude.ai.

Cuando Claude Code sincroniza un plugin agregado, actualizado o eliminado en una sesión interactiva, ve `Plugins changed. Run /reload-plugins to activate.` Ejecute `/reload-plugins` para cargar el cambio en esa sesión, o déjelo para la próxima vez que inicie Claude Code.

<h3 id="uninstall-a-plugin-the-project-enables">
  Desinstale un plugin que el proyecto habilita
</h3>

Cuando elige **Uninstall** para un plugin que el `.claude/settings.json` de este repositorio habilita, ya sea desde la pestaña **Installed** o con `/plugin uninstall`, Claude Code le pregunta si deshabilitarlo para usted o desinstalarlo para todos:

* **Disable for me**: presione **y**. Claude Code escribe `false` para el plugin en su `.claude/settings.local.json` y lo deja instalado para el proyecto.
* **Uninstall for everyone**: presione **u**. Claude Code elimina el plugin del `.claude/settings.json` compartido.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Vea lo que un plugin instalado añade a sus sesiones
</h3>

En su shell, ejecute `claude plugin details <name>` para un plugin instalado. La línea `Always-on` es el número de tokens que el plugin añade a cada sesión donde está habilitado, y las filas por componente muestran qué skill o agente contribuye más. Para la salida completa y lo que significa cada figura, consulte [Measure what a plugin costs](/docs/es/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Encuentre plugins que ya no usa
</h3>

En la pestaña **Installed** en `/plugin`, los plugins que instaló usted mismo y no ha usado recientemente aparecen bajo un encabezado **Not used recently**, y los detalles de cada plugin muestran una línea **Last used**. Use ese encabezado y esa línea para encontrar plugins que aún añaden costo de inicio y contexto, luego deshabílitelos o desinstálelos.

<h3 id="plugins-with-dependencies">
  Plugins con dependencias
</h3>

Un plugin puede declarar otros plugins de los que depende. Cuando instala, deshabilita o desinstala tal plugin desde un marketplace, Claude Code actúa sobre esas dependencias también:

* **Install**: Claude Code también instala y habilita las dependencias declaradas del plugin en el mismo alcance. El mensaje de éxito las enumera.
* **Enable**: Claude Code también habilita las dependencias del plugin que están instaladas pero deshabilitadas. Si una dependencia declarada no está instalada, la habilitación falla y el mensaje le dice que la instale primero.
* **Disable**: cuando otro plugin habilitado aún necesita el que nombró, Claude Code se niega e imprime un comando encadenado que deshabilita ambos en el orden correcto.
* **Uninstall**: las dependencias instaladas automáticamente permanecen hasta que ejecuta `claude plugin prune` en su shell; consulte [plugin prune](/docs/es/plugins/cli-reference#plugin-prune).

Si cargó el plugin con `--plugin-dir` en su lugar, consulte [Test a plugin and its dependency locally](/docs/es/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Administre plugins desde su shell
</h3>

También puede administrar plugins sin iniciar una sesión de Claude Code. En su shell, ejecute `claude plugin install`, `enable`, `disable` o `uninstall` como comandos de terminal ordinarios; cambian la misma configuración que el panel `/plugin` hace. Cada uno toma `--scope` para dirigirse a un alcance, y usa un alcance predeterminado cuando lo omite:

* `enable` y `disable` actúan sobre el alcance más específico cuya configuración ya enumera el plugin.
* `install` y `uninstall` actúan sobre el alcance de usuario.

Por ejemplo, estos comandos deshabilitan y vuelven a habilitar un plugin, luego lo desinstalan en alcance de proyecto:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Mantenga los plugins actualizados
</h2>

Los plugins se actualizan automáticamente cuando el marketplace del que provienen tiene la actualización automática activada. Después de que comienza una sesión, Claude Code actualiza esos marketplaces y actualiza las copias en disco de los plugins que instaló desde ellos.

La sesión en ejecución mantiene las versiones que ya cargó. Después de una actualización, ve `Plugin updated: <name> · Run /reload-plugins to apply`, y la próxima sesión carga las nuevas versiones automáticamente.

Estos son los valores predeterminados de actualización automática para cada tipo de marketplace:

* **On by default**: `claude-plugins-official` y los otros [nombres de marketplace oficial](/docs/es/plugins/security#official-marketplace-names) excepto `knowledge-work-plugins` y `first-party-plugins`, más [marketplaces agregados desde claude.ai](#add-from-claude-ai).
* **Off by default**: cada otro marketplace, incluyendo el marketplace comunitario, marketplaces de terceros y marketplaces de desarrollo local.

Para cuándo se ejecuta la actualización automática, qué plugins omite y las variables de entorno que la desactivan, consulte [When auto-update runs](/docs/es/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Active o desactive la actualización automática para un marketplace
</h3>

En una sesión de Claude Code, ejecute `/plugin` y vaya a la pestaña **Marketplaces**. Seleccione el marketplace, luego seleccione **Enable auto-update** o **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Actualice un plugin ahora
</h3>

En una sesión, abra el plugin en la pestaña **Installed** en `/plugin` y seleccione **Update now**, o en su shell ejecute `claude plugin update <plugin>@<marketplace>`.

<h3 id="auto-update-from-a-private-marketplace">
  Actualización automática desde un marketplace privado
</h3>

Para un marketplace privado, consulte [What background auto-update does with credentials](/docs/es/plugins/host-marketplace#what-background-auto-update-does-with-credentials) para cómo las actualizaciones automáticas en segundo plano se autentican sobre SSH y HTTPS, y [Troubleshoot plugins](/docs/es/plugins/troubleshooting#add-a-marketplace) para los mensajes que ve cuando fallan.

<h2 id="manage-marketplaces">
  Administre marketplaces
</h2>

La pestaña **Marketplaces** en `/plugin` enumera cada marketplace que registró, junto con su fuente. Seleccione uno para explorar sus plugins, actualizar su listado, activar o desactivar la actualización automática, o eliminarlo.

También puede enumerar, actualizar y eliminar marketplaces con comandos, desde su shell o dentro de una sesión:

| Acción                                 | En su shell                               | Dentro de una sesión                |
| :------------------------------------- | :---------------------------------------- | :---------------------------------- |
| Enumere marketplaces                   | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Actualice el listado de un marketplace | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Elimine un marketplace                 | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Cuando elimina un marketplace, Claude Code desinstala cada plugin que instaló desde él y elimina sus entradas `enabledPlugins` de sus archivos de configuración. La pestaña **Marketplaces** nombra esos plugins antes de pedirle que confirme.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Marketplaces de Anthropic](/docs/es/plugins/anthropic-marketplaces): cómo difieren los marketplaces oficiales, comunitarios y de demostración, y dónde explorar cada uno
* [Referencia de carga de plugins](/docs/es/plugins/loading): por qué un plugin se cargó, no se cargó, o no cambió después de una actualización
* [Seguridad y confianza de plugins](/docs/es/plugins/security): qué revisar antes de instalar un plugin de un marketplace que no conoce
* [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting): mensajes de error de instalación y marketplace con sus soluciones
* [Crear un plugin](/docs/es/plugins/create): construya el suyo propio
