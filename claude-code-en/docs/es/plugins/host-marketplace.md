> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Alojar y mantener un marketplace

> Publique un marketplace de plugins donde los usuarios puedan acceder a él, otorgue acceso a uno privado y lance actualizaciones y cambios de nombre sin romper las instalaciones.

Alojar un marketplace significa poner su archivo `marketplace.json` donde otras personas puedan agregarlo con `/plugin marketplace add`, instalar sus plugins y seguir recibiendo sus cambios después de que realice push.

Esta página es para la persona que opera un marketplace.

<Note>
  Estos casos se cubren en otras páginas:

  * **Aún no ha escrito el archivo de catálogo**: comience con [Create a marketplace](/docs/es/plugins/create-marketplace)
  * **Es un administrador que requiere, restringe o preinstala marketplaces en las máquinas de su organización**: lea [Manage plugins for your organization](/docs/es/plugins/org)
</Note>

Comience con [Host your marketplace](#host-your-marketplace) para elegir un host y el comando que ejecutan sus usuarios. Lea [Keep users up to date](#keep-users-up-to-date) antes de su primer lanzamiento. Lea [Rename or remove a plugin](#rename-or-remove-a-plugin) antes de cambiar el `name` de un plugin.

<h2 id="host-your-marketplace">
  Host your marketplace
</h2>

Puede alojar el marketplace en GitHub, en otro host de git, como una URL `marketplace.json` alojada, o en un directorio en un sistema de archivos compartido. Envíe a sus usuarios el comando add para su host y dígales qué necesitan en su máquina:

| Host                                                           | Los usuarios ejecutan, en una sesión de Claude Code                    | Qué necesitan los usuarios                                                                                                                 |
| :------------------------------------------------------------- | :--------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| GitHub                                                         | `/plugin marketplace add your-org/your-marketplace`                    | `git`, y para un repositorio privado el acceso descrito en [Grant access to a private marketplace](#grant-access-to-a-private-marketplace) |
| GitLab, Bitbucket, GitHub Enterprise Server u otro host de git | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git` y acceso al host desde su máquina. Envíe la URL completa, porque el atajo `owner/repo` siempre significa github.com                  |
| Una URL `marketplace.json` alojada                             | `/plugin marketplace add https://plugins.example.com/marketplace.json` | Acceso HTTPS a la URL. Los usuarios no necesitan `git` para el catálogo en sí                                                              |
| Un directorio en un sistema de archivos compartido             | `/plugin marketplace add /Volumes/shared/claude-plugins`               | Acceso de lectura a la ruta                                                                                                                |

Para fijar una rama o etiqueta de un marketplace de GitHub o URL de git, diga a los usuarios que agreguen `#<ref>`, como en `your-org/your-marketplace#stable`. La [plugin commands reference](/docs/es/plugins/cli-reference#plugin-marketplace-add) enumera todas las formas que acepta el comando.

Un add exitoso imprime `Successfully added marketplace: your-marketplace`. Claude Code toma ese nombre del campo `name` en su `marketplace.json`, no del nombre del repositorio.

Los usuarios luego instalan un plugin por el `name` de su entrada y el `name` del marketplace, como en `/plugin install code-formatter@your-marketplace`.

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  Register the marketplace for everyone in a repository
</h3>

Para compartir el marketplace con todos los que trabajan en un repositorio, ejecute `claude plugin marketplace add your-org/your-marketplace --scope project` allí una vez desde su shell y confirme el `.claude/settings.json` que escribe. Claude Code luego registra el marketplace para cada compañero de equipo que [confía en la carpeta](/docs/es/plugins/org#require-plugins-per-repository).

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  Avoid relative-path entries in a URL-hosted marketplace
</h3>

Cuando los usuarios agregan su marketplace como una URL `marketplace.json` simple, Claude Code descarga solo ese archivo. Una entrada en su matriz `plugins` cuyo `source` es una ruta relativa como `./plugins/formatter` luego falla en la instalación con [`its marketplace entry path does not stay inside the marketplace directory`](/docs/es/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces). Dé a cada entrada un source que pueda obtenerse por sí solo, como un repositorio `github` o una URL `archive`, u aloje el marketplace en un repositorio de git para que Claude Code clone el árbol completo.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  Edit plugins in place on a shared directory
</h3>

Cuando los usuarios agregan su marketplace desde un directorio compartido, Claude Code lee plugins con sources de ruta relativa directamente desde ese directorio en lugar de copiarlos. Los usuarios ven sus ediciones cuando inician la siguiente sesión o ejecutan `/reload-plugins`, sin un paso de actualización o un aumento de versión.

<h3 id="keep-plugin-files-out-of-git-lfs">
  Keep plugin files out of Git LFS
</h3>

Mantenga los archivos que sus plugins necesitan fuera de [Git LFS](https://git-lfs.com). Cuando los usuarios agregan un marketplace alojado en un repositorio de git, o instalan un plugin basado en git que enumera, Claude Code clona ese marketplace o repositorio de plugin en su máquina. El clon nunca descarga contenido de LFS, por lo que los archivos rastreados por LFS llegan como archivos de puntero.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Share files within a marketplace with symlinks
</h3>

Para compartir archivos entre su plugin y otras partes del mismo marketplace, cree enlaces simbólicos dentro de su directorio de plugin. Cuando Claude Code copia el plugin en su caché, maneja cada enlace simbólico por donde se resuelve el destino:

* **Dentro del directorio del plugin**: el enlace simbólico se conserva como un enlace simbólico relativo en el caché, por lo que sigue resolviendo al destino copiado en tiempo de ejecución.
* **En otro lugar dentro del mismo marketplace**: el enlace simbólico se desreferencia. El contenido del destino se copia en el caché en su lugar. Esto permite que el directorio `skills/` de un meta-plugin se vincule a skills definidas por otros plugins en el marketplace.
* **Fuera del marketplace**: el enlace simbólico se omite por seguridad.

Para plugins instalados desde una ruta local, o desde un [`command` source](/docs/es/plugins/marketplace-reference#command-plugin-source) cuyo `mode` es el `copy` predeterminado, Claude Code preserva solo enlaces simbólicos que se resuelven dentro del directorio del plugin y omite todos los demás.

El siguiente comando crea un enlace desde dentro de un plugin de marketplace a una skill compartida definida por un plugin hermano. En Windows, use `mklink /D` desde un símbolo del sistema elevado o habilite el Modo de desarrollador:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  Distribute through organization settings
</h2>

En un plan Team o Enterprise, también puede distribuir el marketplace a través de [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) en claude.ai en lugar de alojarlo en algún lugar donde los usuarios lo agreguen ellos mismos. La sincronización de la organización lee el repositorio a través de su conexión de GitHub o GitLab de la organización en claude.ai, por lo que las credenciales de git de sus usuarios no están involucradas.

La sincronización de la organización es más estricta con el repositorio que `/plugin marketplace add`:

* **Repositorio de marketplace**: en github.com y gitlab.com, debe ser privado o interno
* **Plugin sources**: cada plugin source debe ser de tipo `github`, `url` o `git-subdir`, o una [ruta relativa](/docs/es/plugins/marketplace-reference#relative-path-plugin-source) que comience con `./`
* **Directorio `bin/` de nivel superior**: claude.ai rechaza un plugin que tiene uno y sincroniza el resto del marketplace. El mensaje de error comienza con `Plugin contains a top-level bin/ directory`. Mantenga los ejecutables en otro directorio, como `scripts/`, y haga referencia a ellos como `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` desde sus hooks o configuraciones de servidor MCP

Consulte [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) para el flujo de trabajo del administrador.

<h2 id="grant-access-to-a-private-marketplace">
  Grant access to a private marketplace
</h2>

Cuando un usuario agrega, instala desde o actualiza su marketplace, Claude Code ejecuta `git` en su máquina con las indicaciones interactivas desactivadas y se basa en las credenciales que esa máquina ya posee. Claude Code no tiene su propio token de git, y `marketplace.json` no tiene campo para uno.

Usted elige si el clon se ejecuta sobre SSH o HTTPS por la forma del comando add que envía a los usuarios:

* **GitHub `owner/repo`**: Claude Code prueba `ssh -T git@github.com` y clona sobre SSH cuando la prueba tiene éxito. Si la prueba falla, o el clon SSH en sí falla, clona sobre HTTPS. Los usuarios en máquinas sin una clave SSH de GitHub pueden establecer `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` para omitir la prueba y clonar sobre HTTPS.
* **`git@host:path.git`**: SSH.
* **`https://example.com/repo.git`**: HTTPS.

Diga a los usuarios qué necesita cada protocolo en su máquina:

* **SSH**: la clave debe funcionar sin una indicación de contraseña, por ejemplo porque está cargada en `ssh-agent`. El host ya debe estar en `known_hosts`.
* **HTTPS**: Claude Code deja habilitado el asistente de credenciales de git del usuario pero le prohíbe que solicite. Una credencial que el asistente ya almacena funciona; una que tendría que pedir falla. En GitHub, `gh auth login` seguido de `gh auth setup-git` almacena una.

Para un host de GitHub Enterprise Server, los usuarios necesitan acceso de git a ese host desde su máquina. Consulte [Plugin marketplaces on GHES](/docs/es/github-enterprise-server#plugin-marketplaces-on-ghes) para ver qué necesita cada superficie de Claude Code para alcanzar un marketplace alojado en GHES.

Si distribuye a través de **Organization settings > Plugins & skills** en claude.ai en su lugar, las credenciales de git de sus usuarios no están involucradas. Consulte [Distribute through organization settings](#distribute-through-organization-settings) para ver qué plugin sources pueden ser privados allí.

<h3 id="serve-users-who-have-no-git-host-account">
  Serve users who have no git-host account
</h3>

Los usuarios sin una cuenta de host de git pueden agregar un marketplace que sirve como una URL `marketplace.json` o desde un directorio compartido, pero solo pueden instalar los plugins cuyas fuentes de entrada también pueden alcanzar. Una entrada que apunta a un repositorio `github` privado aún falla en la instalación para ellos, porque Claude Code lo obtiene con el mismo `git` no interactivo que usa para un marketplace alojado en git.

Estas fuentes de entrada no necesitan una cuenta de git:

* **`archive`**: un zip descargado sobre HTTPS. Los usuarios no necesitan ni `git` ni una cuenta, solo acceso de red a la URL. Requiere Claude Code v2.1.224 o posterior. Fije cada archivo con `sha256` para que Claude Code rechace una descarga cambiada. Para enviar credenciales con la descarga, consulte [Authenticate archive downloads](#authenticate-archive-downloads).
* **Un repositorio de git público**: Claude Code clona un source `url` o `git-subdir` público sobre HTTPS sin credenciales cuando la entrada proporciona una URL `https://`. Para un source `github`, o un source `git-subdir` escrito como `owner/repo`, los usuarios sin una clave SSH de GitHub establecen `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`.

Para un equipo en una red, un marketplace `directory` en un sistema de archivos compartido también funciona sin cuentas de git. Los usuarios solo necesitan acceso de lectura a la ruta.

<h3 id="what-background-auto-update-does-with-credentials">
  What background auto-update does with credentials
</h3>

La actualización automática en segundo plano es la actualización desatendida de Claude Code de marketplaces y plugins instalados después de que comienza una sesión. Está desactivada para su marketplace hasta que un usuario o administrador la activa, como se cubre en [Keep users up to date](#keep-users-up-to-date).

Cuando está activada para un marketplace privado, la verificación de fondo de nuevos commits utiliza los asistentes de credenciales de git configurados del usuario y nunca solicita. Cada tipo de remoto y asistente da un resultado diferente:

* **Remotos SSH**: una clave cargada en `ssh-agent` autentica la verificación.
* **Remotos HTTPS con una credencial almacenada**: un asistente que puede suministrar una credencial almacenada sin solicitar autentica la verificación. Git Credential Manager, el asistente de Keychain de macOS y `git-credential-store` funcionan de esta manera una vez que tienen una credencial para el host.
* **Remotos HTTPS con un asistente que necesita solicitar**: el asistente no puede responder en segundo plano. La actualización falla silenciosamente y el checkout existente permanece en su lugar, por lo que los plugins del usuario siguen funcionando desde el último estado sincronizado.

Después de la verificación, Claude Code hace una de las siguientes:

* **El checkout está actualizado**: Claude Code lo deja como está.
* **La verificación encuentra nuevos commits, o falla porque no puede alcanzar o autenticarse en el remoto**: Claude Code clona el marketplace nuevamente y reemplaza el checkout existente con el nuevo clon. Si ese clon falla, el checkout existente permanece en su lugar. El re-clon puede [agotar el tiempo de espera en repositorios grandes](/docs/es/plugins/troubleshooting#git-clone-timed-out-after-120s).

Para mantener un marketplace privado actualizado, un usuario puede hacer cualquiera de lo siguiente:

* **Almacenar una credencial**: inicie sesión en el asistente de credenciales primero para que tenga una credencial para el host. Para GitHub, ejecute `gh auth login`, luego `gh auth setup-git`.
* **Mantener el checkout en caso de fallo**: si el usuario establece `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, Claude Code mantiene el checkout existente sin intentar el re-clon cuando la verificación de fondo no puede alcanzar o autenticarse en el remoto. Los plugins siguen funcionando desde el último estado sincronizado.

Si un usuario establece `GITHUB_TOKEN` u otro token de proveedor en el entorno, eso solo no autentica la verificación de fondo. Un token toma efecto a través de un asistente de credenciales, como el asistente de la CLI `gh`, que lee `GH_TOKEN` y `GITHUB_TOKEN`.

<h2 id="roll-out-to-a-whole-company">
  Roll out to a whole company
</h2>

Desplegar un plugin en toda una empresa implica a usted como propietario del marketplace, un administrador que controla la configuración administrada y cada persona que usa Claude Code. Puede ejecutar el despliegue sin el administrador, en cuyo caso cada persona agrega el marketplace e instala el plugin ellos mismos.

| Quién                                 | Qué hacen                                                                                                                                                                    | Dónde se cubre                                                                                                                                          |
| :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Usted, el propietario del marketplace | Mantenga el catálogo en un repositorio que solo la empresa pueda leer, envíe el comando add para su host y diga qué necesita cada persona en su máquina                      | [Host your marketplace](#host-your-marketplace) y [Grant access to a private marketplace](#grant-access-to-a-private-marketplace)                       |
| Un administrador                      | Registra el marketplace y activa sus plugins para todos con `extraKnownMarketplaces` y `enabledPlugins` en la configuración administrada, y establece `autoUpdate` allí      | [Require a marketplace and its plugins](/docs/es/plugins/org#require-a-marketplace-and-its-plugins) y [Set update policy](/docs/es/plugins/org#set-update-policy) |
| Cada persona                          | Necesita acceso de lectura a un repositorio de git privado, con credenciales ya almacenadas en su máquina. Sin un administrador, también ejecutan los comandos add e install | [Add a private marketplace](/docs/es/plugins/install#add-a-private-marketplace)                                                                              |

Para personas que no tienen una cuenta de host de git, estas secciones cubren una forma de alcanzarlas:

* **Entry sources que no necesitan una cuenta de git**: [Serve users who have no git-host account](#serve-users-who-have-no-git-host-account)
* **Un directorio de plugins precompletado**: [Seed containers and CI](/docs/es/plugins/org#seed-containers-and-ci), que también sirve a usuarios que no tienen una cuenta de host de git
* **Configuración de la organización de claude.ai**: [Distribute through organization settings](#distribute-through-organization-settings), donde las credenciales de git de sus usuarios no están involucradas

<h2 id="keep-users-up-to-date">
  Keep users up to date
</h2>

Sus cambios llegan a los usuarios a través de la actualización automática en segundo plano, una vez que está activada para su marketplace, o cuando los usuarios actualizan el plugin ellos mismos. En ambos casos, un usuario obtiene una nueva copia de un plugin solo cuando su versión calculada cambia, como se describe en [Release a new version](#release-a-new-version).

<h3 id="turn-on-auto-update">
  Turn on auto-update
</h3>

La actualización automática en segundo plano está desactivada para su marketplace de forma predeterminada, y `marketplace.json` no tiene campo para activarla. Un usuario o un administrador la activa:

* **Diga a los usuarios que la activen**: cada usuario va a **Marketplaces** en `/plugin`, selecciona su marketplace y selecciona **Enable auto-update**.
* **Pida a un administrador que la establezca**: si un administrador establece `"autoUpdate": true` en la entrada `extraKnownMarketplaces` de su marketplace en la configuración administrada, está activada para todos los que reciben esa configuración. Consulte [Set update policy](/docs/es/plugins/org#set-update-policy).

Sin actualización automática, los usuarios reciben sus cambios cuando ejecutan `/plugin marketplace update <name>` en una sesión o `claude plugin update <plugin>@<name>` en el shell.

Para ver qué ven los usuarios cuando llega una actualización, consulte [When auto-update runs](/docs/es/plugins/loading#when-auto-update-runs).

<h3 id="release-a-new-version">
  Release a new version
</h3>

Para lanzar una nueva versión a los usuarios, cambie el `version` del plugin. Los usuarios obtienen una nueva copia solo cuando la versión calculada del plugin difiere de la que tienen. Esa versión proviene de `plugin.json` primero, luego de la entrada del marketplace, según [Versions and updates](/docs/es/plugins/loading#versions-and-updates).

Un plugin que los usuarios [cargan en su lugar](/docs/es/plugins/loading#find-plugins-on-disk) desde un marketplace que agregaron como un directorio local no está controlado por `version`. Carga sus archivos actuales en cada inicio de sesión, sin importar lo que diga su cadena de versión.

Para cada instalación que no sea una carga en su lugar o una de un source `command`, aumente `version` en cada lanzamiento u omítalo:

* **Aumente `version` en cada lanzamiento**: los usuarios permanecen en su copia en caché hasta que la cadena cambia. Si establece `"version": "1.0.0"` y realiza push de nuevos commits sin cambiarlo, los usuarios no los reciben.
* **Omita `version`**: los usuarios rastrean sus commits en su lugar. Deje `version` fuera de `plugin.json` y de la entrada del marketplace.

No establezca `version` en `plugin.json` y en la entrada del marketplace. Si lo hace, Claude Code usa el valor `plugin.json` sin advertencia, y `claude plugin validate` reporta la discrepancia como `Entry declares version "<a>" but <path>/plugin.json says "<b>"`.

<h3 id="hold-users-on-one-version">
  Hold users on one version
</h3>

Un marketplace sirve una versión de cada plugin a la vez, por lo que mantiene a los usuarios en una versión eligiendo a qué apunta cada entrada:

* **`ref` y `sha` en la entrada del plugin**: `ref` nombra una rama o etiqueta y `sha` nombra un commit para un source `github`, `url` o `git-subdir`. Consulte [Plugin sources](/docs/es/plugins/marketplace-reference#plugin-sources).
* **`#<ref>` en el comando add**: los usuarios que agregan `your-org/your-marketplace#stable` obtienen esa rama o etiqueta del catálogo. Para dos líneas de lanzamiento a la vez, consulte [Run release channels](#run-release-channels).
* **Etiquetas `<plugin>--v<version>`**: un rango de versión de una dependencia se resuelve contra estas etiquetas. Consulte [Release a plugin that others depend on](/docs/es/plugins/dependencies#tag-plugin-releases-for-version-resolution).

[Release a new version](#release-a-new-version) dice cuándo llega una entrada cambiada a los usuarios.

<h3 id="change-the-command-of-a-command-source">
  Change the command of a command source
</h3>

Si cambia el `command` de un [`command` source](/docs/es/plugins/marketplace-reference#command-plugin-source), o cambia su `mode`, cada usuario tiene que aceptar el nuevo comando antes de que Claude Code lo ejecute. Claude Code ejecuta solo el comando exacto que un usuario aceptó cuando instaló o actualizó por última vez el plugin.

Después de que la copia del marketplace de un usuario recoja el cambio, ese usuario ve lo siguiente:

* **Sin más ejecuciones en segundo plano**: la [ejecución una vez por sesión](/docs/es/plugins/loading#when-a-command-source-re-runs) del comando se detiene para ese usuario, por lo que la nueva salida de la herramienta no llega a ellos.
* **Una entrada en la pestaña `/plugin` Errors**: la entrada muestra el nuevo comando y el comando `claude plugin update` a ejecutar.

Diga a los usuarios que ejecuten el comando `claude plugin update` que esa entrada muestra, en una terminal. Claude Code les muestra el nuevo comando y les pide que lo acepten.

<h2 id="run-release-channels">
  Run release channels
</h2>

Para ofrecer pistas estables y de acceso temprano, aloje dos marketplaces cuyas entradas apunten a diferentes refs del mismo plugin, y deje que cada usuario agregue el que quiera. Claude Code no tiene concepto de canal de lanzamiento, y un marketplace sirve una versión de cada plugin a la vez.

Dé a los dos archivos `marketplace.json` diferentes valores `name`. Claude Code identifica un marketplace por su `name`, por lo que un usuario no puede tener dos marketplaces con el mismo nombre registrados a la vez.

Con estos dos catálogos, los usuarios que agregan `stable-tools` instalan `code-formatter` desde la rama `stable`, y los usuarios que agregan `latest-tools` lo instalan desde `latest`:

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

Dé a los dos refs diferentes versiones `plugin.json`, u omita `version` para que el SHA del commit los distinga. Las actualizaciones se detectan comparando versiones, por lo que un ref que se mueve sin un cambio de versión deja a los usuarios en la copia en caché.

Para asignar los canales a grupos de usuarios en lugar de dejar que los usuarios elijan, un administrador da a cada grupo la entrada `extraKnownMarketplaces` coincidente, como se describe en [Set update policy](/docs/es/plugins/org#set-update-policy).

<h2 id="rename-or-remove-a-plugin">
  Rename or remove a plugin
</h2>

El `name` de un plugin es su identificador. Los usuarios lo hacen referencia en las claves de configuración `enabledPlugins` y `pluginConfigs` y en `/plugin install`, por lo que cambiarlo rompe cada instalación existente.

Para cambiar la etiqueta que los usuarios ven en `/plugin` sin romper nada, establezca `displayName` en `plugin.json` y mantenga `name` sin cambios.

<h3 id="migrate-users-with-a-renames-map">
  Migrate users with a renames map
</h3>

Cuando debe cambiar un `name`, agregue un mapa `renames` de nivel superior a `marketplace.json` para que Claude Code migre a los usuarios existentes en lugar de reportar [`Plugin "<name>" not found in marketplace`](/docs/es/plugins/troubleshooting#plugin-not-found-in-marketplace). Haga lo mismo cuando elimine una entrada de `plugins`. La migración automática requiere Claude Code v2.1.193 o posterior.

Asigne cada nombre anterior a su nombre actual, o a `null` cuando el plugin se haya ido. Este marketplace renombra `formatter` a `code-formatter` y registra que `legacy-linter` fue eliminado:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Después de que realice push, un usuario que aún tiene el nombre anterior habilitado ve uno de estos resultados:

* **Entrada renombrada**: el plugin se carga bajo su nuevo nombre. `claude plugin list` y los detalles del plugin en `/plugin` muestran `Renamed to "code-formatter" in the "your-marketplace" marketplace` una vez, y Claude Code reescribe la clave anterior a la nueva en `enabledPlugins` y `pluginConfigs` en los ámbitos de configuración del usuario, proyecto y local.
* **Entrada `null`**: la clave anterior se elimina de esos ámbitos y el usuario ve `Removed from the "your-marketplace" marketplace`.
* **Habilitado en la configuración administrada**: el plugin aún se carga bajo su nuevo nombre, pero Claude Code no puede reescribir la configuración administrada, por lo que el aviso se repite hasta que un administrador actualiza `enabledPlugins` allí.

Para un marketplace que los usuarios agregaron desde un repositorio de git o URL, un plugin renombrado reporta [`Plugin "<name>" not cached at <path>`](/docs/es/plugins/troubleshooting#plugin-not-cached-at) hasta que el usuario ejecuta `/plugin install code-formatter@your-marketplace` una vez en una sesión.

Trate `renames` como historial de solo anexión. Mantenga las entradas antiguas después de que todos hayan migrado. Cuando renombre nuevamente, agregue una segunda entrada en lugar de editar la primera, porque Claude Code sigue la cadena desde el nombre más antiguo.

En su shell, ejecute `claude plugin validate .` después de editar el mapa. Rechaza una cadena que cicla o que termina en cualquier lugar que no sea `null` o un nombre en `plugins`, con `renames.<name>: chain does not resolve`.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  Uninstall removed plugins from users' machines
</h3>

Para desinstalar un plugin eliminado de las máquinas de los usuarios en lugar de dejar una copia atrás, establezca `"forceRemoveDeletedPlugins": true` en el nivel superior de `marketplace.json`. Sin el campo, un plugin eliminado permanece instalado y reporta `Plugin "<name>" not found in marketplace` cuando una sesión lo carga. Con él, Claude Code hace lo siguiente en cada inicio de sesión:

1. Compara lo que los usuarios instalaron desde su marketplace contra las entradas y el mapa `renames`, y trata cualquier plugin que no esté listado ni renombrado como eliminado.
2. Desinstala cada plugin eliminado del usuario, proyecto y ámbitos locales. Los plugins que solo la configuración administrada instaló permanecen en su lugar.
3. Enumera cada plugin eliminado bajo un encabezado **Flagged** en `/plugin` con el estado `Removed from marketplace`.

<h2 id="authenticate-archive-downloads">
  Authenticate archive downloads
</h2>

Para autenticar una descarga de [`archive`](/docs/es/plugins/marketplace-reference#archive-plugin-source), como una descarga de un registro privado, establezca los encabezados HTTP que Claude Code envía con ella. Puede establecer `headers` en cualquiera de estos lugares:

* **El source `url` del marketplace**: el source `url` desde el que registró el marketplace, como una entrada [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces).
* **La entrada del plugin**: en Claude Code v2.1.238 o posterior, puede establecerlo en la entrada `marketplace.json` del plugin en su lugar, junto a `source`.

En cualquier lugar, establezca un comando `headersHelper` en lugar de `headers` cuando el valor es de corta duración, como un token que su registro genera bajo demanda. Claude Code ejecuta el comando y envía el objeto JSON que imprime como los encabezados de ese lugar. Requiere Claude Code v2.1.238 o posterior.

La [marketplace reference](/docs/es/plugins/marketplace-reference#plugin-entries) enumera los campos de entrada `headers` y `headersHelper`.

El lugar que elige decide qué descargas obtienen los encabezados y cuándo Claude Code ejecuta el comando:

| Lugar                        | Descargas que obtienen los encabezados                                                                        | Cuándo Claude Code ejecuta un `headersHelper` establecido allí                                                                                                                               |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Source `url` del marketplace | Descargas de archivo en el origen de la URL del marketplace, lo que significa el mismo esquema, host y puerto | Antes de cada obtención del `marketplace.json` del marketplace y antes de cada descarga de archivo en ese origen. Claude Code reutiliza la salida de una ejecución durante hasta 60 segundos |
| Entrada del plugin           | Solo la descarga de esa entrada                                                                               | Solo cuando un usuario instala o actualiza ese plugin solo y [acepta el comando](#how-users-accept-a-headershelper-command)                                                                  |

Donde ambos lugares establecen un encabezado del mismo nombre, Claude Code envía el valor de la entrada. Dentro de un lugar, un encabezado que el comando imprime anula un encabezado del mismo nombre listado en `headers`.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  Add a headersHelper to a plugin entry
</h3>

Esta entrada establece `headersHelper` junto a `source`. También establece [`"strict": false`](/docs/es/plugins/marketplace-reference#strict-mode), que Claude Code requiere de una entrada `marketplace.json` que establece `headersHelper`:

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

Para verificar la entrada, ejecute `claude plugin install my-plugin@your-marketplace` en su shell. Claude Code le muestra el comando y la URL del archivo, y descarga el zip después de que acepte.

<h3 id="write-the-headershelper-command">
  Write the headersHelper command
</h3>

Ya sea que establezca `headersHelper` en un source `url` del marketplace o en una entrada del plugin, escriba el comando para cumplir con estos requisitos:

* **Texto del comando**: como máximo 500 caracteres de ASCII imprimible, sin una ejecución de cuatro o más espacios.
* **Salida**: imprima un objeto JSON de nombres de encabezados y valores de cadena en stdout, luego salga 0 dentro de 10 segundos.
* **Shell y directorio de trabajo**: Claude Code ejecuta el comando a través de `sh`, o a través de `cmd.exe` en Windows. El directorio de trabajo es el directorio de configuración, que es `~/.claude` o [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars#variables). Dé una ruta absoluta o un comando en `PATH`, porque una ruta relativa se resuelve contra ese directorio, no el proyecto del usuario.
* **Variables que Claude Code elimina**: cuando el comando se establece en una entrada `marketplace.json`, o en el `.claude/settings.json` o `.claude/settings.local.json` de un proyecto, Claude Code elimina del entorno cada variable cuyo nombre parece una credencial, por la [misma regla que aplica a un `headersHelper` de MCP](/docs/es/mcp#which-variables-a-helper-can-read). `ANTHROPIC_API_KEY` y `MY_REGISTRY_TOKEN` se eliminan, por lo que el comando debe leer su credencial de un archivo o un almacén de credenciales. Esta eliminación no se aplica a un comando establecido en la configuración del usuario, un archivo `--settings` o la configuración administrada.
* **Variables que Claude Code establece**: `CLAUDE_CODE_MARKETPLACE_URL` y `CLAUDE_CODE_MARKETPLACE_NAME` para el comando de un source `url`, y `CLAUDE_CODE_PLUGIN_NAME` y `CLAUDE_CODE_PLUGIN_ARCHIVE_URL` para el comando de una entrada. `CLAUDE_CODE_MARKETPLACE_NAME` no se establece en la primera obtención después de que un usuario agrega un marketplace por URL, porque esa obtención es lo que proporciona el nombre.

Un comando que acuña un token de portador imprime un objeto como este:

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  When Claude Code skips a headersHelper command or drops its output
</h3>

Un comando `headersHelper` no se ejecuta, o los encabezados de `headers` o de la salida del comando se descartan, cuando se aplica uno de los siguientes:

* **El comando falla**: si el comando sale con código distinto de cero, se ejecuta más de 10 segundos, o imprime algo que no sea un objeto JSON de valores de cadena, la obtención o descarga para la que se ejecutó el comando no sucede.
* **La URL del marketplace no comienza con `https://`**: el comando de ese source `url` no se ejecuta, y las solicitudes llevan solo los encabezados listados en su campo `headers`.
* **La redirección deja el origen**: cuando una descarga se redirige fuera del origen de la URL del archivo, la solicitud redirigida no lleva valores de `headers` o salida de comando de ninguno del source `url` del marketplace o de la entrada del plugin.
* **La entrada establece un encabezado de enrutamiento o identidad**: Claude Code descarta nombres de enrutamiento de solicitud e identidad de cliente como `Host`, `Cookie` y `X-Forwarded-*` de los `headers` de una entrada y la salida del comando, y mantiene nombres de autenticación como `Authorization`. Cada entrada `marketplace.json` se filtra de esta manera. Para una entrada de plugin en línea en la configuración, consulte [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces).
* **Comando establecido en la configuración de un directorio `--add-dir`**: el comando se ignora, en un source `url` y en una [entrada de plugin en línea](/docs/es/settings-reference#extraknownmarketplaces) por igual, y solo se envían los `headers` de ese archivo.
* **La configuración administrada bloquea el comando**: establecer [`disableCommandPluginSources`](/docs/es/settings-reference#disablecommandpluginsources) a `true` bloquea comandos `headersHelper`, y [`allowManagedHooksOnly`](/docs/es/settings-reference#allowmanagedhooksonly) también los bloquea a menos que `disableCommandPluginSources` sea explícitamente `false`. Bajo cualquiera de los bloqueos, Claude Code aún ejecuta el comando para un marketplace que la configuración administrada declara.

<h3 id="how-users-accept-a-headershelper-command">
  How users accept a headersHelper command
</h3>

Un usuario acepta el comando de una entrada del plugin cada vez que instala o actualiza ese plugin solo. Lo hacen desde la vista propia del plugin en `/plugin`, o con `claude plugin install` o `claude plugin update`. Claude Code muestra el comando y la URL del archivo, y ejecuta el comando solo después de que el usuario acepta.

En un shell no interactivo, pase [`--yes`](/docs/es/plugins/cli-reference#plugin-install) para aceptar el comando. Para aceptar solo el comando que una ejecución anterior de `--json` mostró, pase [`--accept-command`](/docs/es/plugins/cli-reference#plugin-install) con el `sha256` que la ejecución reportó.

Claude Code ejecuta solo el comando que mostró, para la URL del archivo que mostró. Si el comando o la URL del archivo de la entrada cambiaron entre tanto, Claude Code rechaza la instalación o actualización. Un cambio en la cadena de consulta solo no cuenta.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  Installs and updates that refuse a command instead of asking
</h3>

En cualquier operación que no sea una instalación o actualización de un solo plugin, Claude Code ni ejecuta el comando de una entrada ni descarga su archivo. El plugin permanece en su versión instalada o permanece desinstalado, y el usuario ve uno de estos resultados:

* **Instalar varios plugins a la vez, desde una sugerencia de plugin, o como dependencia de otro plugin**: Claude Code rechaza el plugin que tiene el comando y dirige al usuario a la vista propia de ese plugin en `/plugin`. Los otros plugins en una instalación masiva aún se instalan. Un plugin que depende del plugin rechazado falla en instalar hasta que el usuario instale el plugin rechazado por sí solo.
* **Actualización automática en segundo plano, o inicio de sesión para un plugin cuyo archivo nunca se descargó**: Claude Code enumera el plugin en la pestaña `/plugin` Errors para que el usuario sepa que debe instalarlo o actualizarlo ellos mismos.

<h3 id="when-a-marketplace-url-sources-command-runs">
  When a marketplace `url` source's command runs
</h3>

Usted declara el `headersHelper` de un source `url` del marketplace en un archivo de configuración, como una entrada [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces), en lugar de en el catálogo que publica el marketplace. Claude Code por lo tanto no pide al usuario que lo acepte en cada instalación o actualización. En su lugar, el archivo de configuración que lo declara decide cuándo Claude Code lo ejecuta:

| Archivo de configuración                                                                                    | Cuándo Claude Code ejecuta el comando                                                                                                                                                                                                                                    |
| :---------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Configuración del usuario, un archivo `--settings` o un archivo de configuración administrada en la máquina | Sin preguntar, incluyendo durante una actualización de marketplace en segundo plano                                                                                                                                                                                      |
| El `.claude/settings.json` o `.claude/settings.local.json` de un proyecto                                   | Solo después de que el usuario acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#what-runs-before-you-trust-a-folder) para esa carpeta en sí. Una sesión `-p` o SDK no cuenta como aceptarlo, ni tampoco la confianza otorgada a una carpeta padre |
| Configuración administrada por servidor                                                                     | En una sesión interactiva, solo después de que el usuario apruebe la configuración entregada en el [diálogo de aprobación de seguridad](/docs/es/server-managed-settings#security-approval-dialogs)                                                                           |

Para una [entrada de plugin en línea](/docs/es/settings-reference#extraknownmarketplaces) en uno de estos archivos, Claude Code requiere la misma confianza de carpeta o aprobación de configuración que para un comando de nivel de marketplace en ese archivo, y el usuario también acepta el comando de la entrada en cada instalación o actualización.

<h2 id="depend-on-and-recommend-other-plugins">
  Depender de otros plugins y recomendarlos
</h2>

Una entrada puede declarar dependencias de otros plugins.

* **Rangos de versión**: una dependencia puede llevar un rango semver.
* **Dependencias entre marketplaces**: una dependencia de otro marketplace se instala solo cuando su marketplace lista ese marketplace en `allowCrossMarketplaceDependenciesOn`.

Para rangos de versión, la convención de etiqueta git `<plugin>--v<version>` contra la que se resuelven, y la confianza entre marketplaces, consulte [Plugin dependencies](/docs/es/plugins/dependencies).

Para que Claude Code sugiera un plugin cuando un proyecto coincida con él, agregue un bloque `relevance` a la entrada con las señales que identifican el proyecto. Los usuarios ven sugerencias de su marketplace solo cuando un administrador lo lista en `pluginSuggestionMarketplaces`. Para las señales y el paso de habilitación, consulte [Plugin relevance](/docs/es/plugins/relevance).

<h2 id="work-around-what-a-marketplace-can’t-do">
  Work around what a marketplace can't do
</h2>

Algunas cosas que los propietarios piden no tienen campo en `marketplace.json`. Aquí está la opción más cercana para cada una:

* **Restringir qué más instalan los usuarios**: la lista de permitidos del marketplace es una configuración administrada, `strictKnownMarketplaces`. Consulte [Restrict what users can install](/docs/es/plugins/org#restrict-what-users-can-install).
* **Instalar o habilitar un plugin sin que el usuario lo pida**: ningún campo de entrada instala un plugin. El `enabledPlugins` administrado lo hace para una flota; consulte [Pre-install and require plugins](/docs/es/plugins/org#pre-install-and-require-plugins).
* **Mostrar diferentes entradas a diferentes usuarios**: las entradas no llevan campo de audiencia, y cada usuario que agrega el marketplace ve el catálogo completo. Aloje marketplaces separados para audiencias separadas.
* **Marcar un plugin como obsoleto**: no hay estado de obsolescencia. La opción es eliminar la entrada, asignar su nombre a `null` en `renames` y opcionalmente establecer `forceRemoveDeletedPlugins`.
* **Activar la actualización automática para sus usuarios**: cada usuario la activa en **Marketplaces** en `/plugin`, o un administrador establece `autoUpdate` en la configuración administrada. Consulte [Turn on auto-update](#turn-on-auto-update).
* **Llevar credenciales de git**: ningún campo de marketplace contiene un token de git. El acceso a un marketplace o plugin alojado en git sigue la configuración de git del usuario, según [Grant access to a private marketplace](#grant-access-to-a-private-marketplace). Para sources `archive`, una entrada puede establecer [`headers` o `headersHelper`](#authenticate-archive-downloads) en su lugar.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/es/plugins/marketplace-reference): campos `marketplace.json`, tipos de source y mensajes de validación
* [Manage plugins for your organization](/docs/es/plugins/org): requiera, restrinja o propague su marketplace en las máquinas de su organización
* [Plugin dependencies](/docs/es/plugins/dependencies): etiquete lanzamientos para que los plugins que dependen del suyo puedan resolver versiones
* [Troubleshoot plugins](/docs/es/plugins/troubleshooting): los errores que ven sus usuarios al agregar o actualizar desde su marketplace
