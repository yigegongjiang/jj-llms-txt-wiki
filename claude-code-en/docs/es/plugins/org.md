> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Administrar plugins de Claude Code para su organización

> Controle qué plugins instala Claude Code y permite en todas las máquinas de su organización mediante configuración administrada.

La configuración administrada le permite decidir qué plugins instala Claude Code y permite en todas las máquinas de su organización. Los usuarios no pueden anularla. La entrega se realiza a través de [configuración administrada por servidor](/docs/es/server-managed-settings) desde la consola de administración de claude.ai o como configuración administrada por punto final a través de MDM o un archivo `managed-settings.json`. La mayoría de los controles en esta página solo tienen efecto desde la configuración administrada.

Esta página es para administradores y la configuración aquí rige Claude Code.

<Note>
  Estos casos se tratan en otras páginas:

  * **Instalar plugins para usted mismo**: comience en [Install plugins](/docs/es/plugins/install)
  * **Controlar qué plugins pueden usar los miembros en claude.ai y Cowork**: consulte [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) en el centro de ayuda
  * **La página de plugins en la configuración de administración de claude.ai**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) activa los plugins para las cuentas de claude.ai de los miembros, y esos llegan a Claude Code como [synced plugins](/docs/es/plugins/loading#synced-plugins). No establece ninguna de las claves en esta página
</Note>

Las secciones siguen el orden que toman la mayoría de los despliegues: [requerir plugins](#pre-install-and-require-plugins) para todos o por repositorio, [sembrar contenedores e IC](#seed-containers-and-ci), [restringir](#restrict-what-users-can-install) lo que los usuarios pueden agregar por su cuenta, [establecer política de actualización](#set-update-policy), luego [auditar](#audit-and-review) lo que está instalado. Para revisar cada clave de política en un solo lugar, consulte la [matriz de control](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Pre-install and require plugins
</h2>

Un marketplace es un catálogo de plugins que Claude Code obtiene de un repositorio git, una URL o una ruta local. Una vez que registra un marketplace en una máquina, Claude Code puede instalar plugins desde él.

Para instalar plugins para una flota, establezca dos claves juntas en [managed settings](/docs/es/managed-settings), el archivo de política o la política entregada por servidor que cada máquina en su organización lee: `extraKnownMarketplaces` registra un marketplace en cada máquina, y `enabledPlugins` nombra los plugins a instalar y habilitar desde él. [Choose a delivery mechanism](#choose-a-delivery-mechanism) cubre cómo la configuración administrada llega a cada máquina.

<h3 id="choose-a-delivery-mechanism">
  Choose a delivery mechanism
</h3>

La configuración administrada llega a una máquina a través de uno de tres mecanismos de entrega:

* **Server-managed settings**: establezca las claves de plugin como JSON en [**Organization settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code). Requiere un [Owner role](/docs/es/server-managed-settings#access-control) en su organización Claude. Una sesión en la nube obtiene esta configuración antes de instalar plugins.
* **MDM policies**: en macOS, entregue un plist cuyas claves de nivel superior sean las claves de configuración. En Windows, almacene todo el documento JSON como una cadena en un valor de registro. El dominio plist y la clave de registro están en [Where each mechanism stores the policy](/docs/es/managed-settings#where-each-mechanism-stores-the-policy).
* **Managed settings file**: coloque un `managed-settings.json` en la ruta del sistema de la plataforma. También puede agregar archivos al directorio drop-in `managed-settings.d/` junto a él. Las rutas de archivo por plataforma están en [Where each mechanism stores the policy](/docs/es/managed-settings#where-each-mechanism-stores-the-policy), y las reglas de fusión drop-in están en [Split a file-based policy across teams](/docs/es/managed-settings#split-a-file-based-policy-across-teams).

Use configuración administrada por servidor si tiene una organización Claude for Teams o Enterprise en claude.ai y sus dispositivos no están todos bajo MDM. De lo contrario, use una política MDM o el archivo de configuración administrada. Para el equilibrio, consulte [Choose between server-managed and endpoint-managed settings](/docs/es/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Which managed source applies on a machine
</h4>

De forma predeterminada, solo una de estas tres fuentes se aplica en una máquina. Claude Code usa la primera que entrega una clave de política, verificando primero la configuración administrada por servidor, luego las políticas MDM, luego el archivo de configuración administrada. Si la configuración administrada por servidor entrega incluso una clave de política no relacionada, Claude Code ignora las claves de plugin en una política MDM o archivo de configuración administrada en esa máquina, aparte de las [claves que lee de cada fuente](/docs/es/managed-settings#keys-read-from-every-admin-source).

Para aplicar cada fuente en su lugar, establezca [`managedSourcesBehavior`](/docs/es/managed-settings#compose-every-managed-source) en `"merge"`.

[How Claude Code combines managed sources](/docs/es/managed-settings#how-claude-code-combines-managed-sources) también enumera las claves que Claude Code lee de cada fuente en ambos modos.

<h3 id="require-a-marketplace-and-its-plugins">
  Require a marketplace and its plugins
</h3>

Agregue el marketplace bajo `extraKnownMarketplaces`, con clave por el `name` propio del marketplace desde su `marketplace.json`. Luego agregue cada plugin bajo `enabledPlugins` como `plugin-name@marketplace-name`. Cada entrada de marketplace lleva un objeto `source` con un campo `source` que nombra el tipo, como `github`. Este ejemplo de configuración administrada registra un marketplace de organización y fuerza la habilitación de dos plugins desde él:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

Después de que la configuración llegue a una máquina, Claude Code registra el marketplace e instala los dos plugins al inicio de la siguiente sesión del usuario. Los usuarios los ven en `/plugin`, y deshabilitar uno en su propio alcance no impide que se cargue, porque la configuración administrada tiene precedencia sobre todos los demás alcances.

Para bloquear un plugin en todos los alcances y ocultarlo del listado del marketplace, establézcalo en `false` en el `enabledPlugins` administrado en su lugar.

Ajuste los campos `autoUpdate` y `source` para su marketplace:

* **`autoUpdate`**: `true` mantiene el marketplace y sus plugins actualizándose en segundo plano, y `false` lo desactiva. Consulte [Set update policy](#set-update-policy).
* **`source`**: `github` es uno de varios tipos de fuente. Una fuente `git` toma una `url` para GitLab o un host interno, y una fuente `url` toma la dirección de un `marketplace.json` alojado. Cada forma de fuente está en la [marketplace reference](/docs/es/plugins/marketplace-reference).

Si el marketplace es un repositorio git privado, cada usuario necesita acceso de lectura a él. El clon de un marketplace basado en git se ejecuta con git en la máquina del usuario, utilizando credenciales almacenadas y sin indicadores. Para usuarios sin cuentas de host git, use un [seed](#seed-containers-and-ci) en su lugar.

Una entrada administrada también anula una entrada de marketplace con el mismo nombre o una copia `--plugin-dir` de otra fuente:

* **Marketplaces**: una entrada de marketplace administrada reemplaza una entrada de menor precedencia con el mismo nombre, y los campos de las dos entradas no se fusionan.
* **Copias `--plugin-dir`**: `--plugin-dir` carga un plugin desde un directorio local para una sesión. Para lo que sucede cuando el nombre de esa copia coincide con un plugin que su `enabledPlugins` administrado nombra, consulte [Name conflicts](/docs/es/plugins/loading#name-conflicts).

El marketplace oficial de Anthropic `claude-plugins-official` no necesita una entrada `extraKnownMarketplaces` cuando `enabledPlugins` establece uno de sus plugins en `true`. Esa entrada `name@claude-plugins-official` declara el marketplace por sí sola, dondequiera que se apliquen estas claves. Si no habilita ninguno de sus plugins y aún desea que se registre en cada máquina, dele una entrada explícita, como [Allow the official marketplace and your own](#allow-the-official-marketplace-and-your-own) hace.

<h3 id="require-plugins-per-repository">
  Require plugins per repository
</h3>

Para cubrir los colaboradores de un repositorio en lugar de toda su flota, establezca `extraKnownMarketplaces` y `enabledPlugins` en el `.claude/settings.json` de ese repositorio. Las entradas `extraKnownMarketplaces` se aplican solo en una carpeta que el colaborador ha confiado, y en una carpeta no confiada Claude Code las ignora sin un mensaje:

* **Sesiones interactivas**: Claude Code registra el marketplace solo después de que el colaborador acepta el [workspace trust dialog](/docs/es/permissions#what-runs-before-you-trust-a-folder) para esa carpeta.
* **[Ejecuciones no interactivas `-p`](/docs/es/headless)**: las entradas se aplican solo en una carpeta cuya confianza el usuario ya aceptó interactivamente, o cuya bandera `hasTrustDialogAccepted` establece en `~/.claude.json`.

Un plugin que el marketplace enumera por una ruta relativa se carga desde la copia del marketplace una vez que se aplican las entradas `extraKnownMarketplaces` del repositorio. Un plugin cuya entrada de marketplace apunta a una fuente externa en su lugar, como el repositorio de GitHub del plugin, no se instala solo desde la configuración del repositorio. Cada colaborador ve `Plugin "<name>" is enabled in project settings but isn't installed` hasta que ejecuta `claude plugin install <name>@<marketplace> --scope project`, como [Install plugins](/docs/es/plugins/install) describe.

Si usa una fuente `directory` o `file` local con una ruta relativa, la ruta se resuelve contra el checkout principal de su repositorio. Cuando ejecuta Claude Code desde un git worktree, la ruta aún apunta al checkout principal, por lo que todos los worktrees comparten la misma ubicación de marketplace.

Para desplegar un paquete de plugins con dependencias, coloque el plugin del paquete en `enabledPlugins`, como [Plugin dependencies](/docs/es/plugins/dependencies) describe.

<h3 id="when-each-surface-applies-the-plugin-keys">
  When each surface applies the plugin keys
</h3>

La tabla muestra cuándo cada tipo de sesión de Claude Code aplica `extraKnownMarketplaces` y `enabledPlugins`, desde la configuración administrada y desde el `.claude/settings.json` de un repositorio. Para la aplicación de escritorio y las extensiones IDE, consulte [Install a plugin](/docs/es/plugins/install#install-a-plugin).

| Surface               | Managed `extraKnownMarketplaces` and `enabledPlugins`                                                                                                                                                                                                                                                                               | Repository `.claude/settings.json`                                                           |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| Terminal, interactive | Applied at session start on every machine that receives the settings                                                                                                                                                                                                                                                                | `extraKnownMarketplaces` applied after trust; `enabledPlugins` applied at session start      |
| `-p` and CI           | Applied at session start, with installs running in the background                                                                                                                                                                                                                                                                   | `extraKnownMarketplaces` in trusted folders only; `enabledPlugins` applied                   |
| Cloud sessions        | In an Anthropic-hosted environment, only server-managed settings reach the session, which waits for them before it installs plugins. MDM policies and managed settings files stay on the user's machine. For a self-hosted environment, see [Where and when a policy applies](/docs/es/managed-settings#where-and-when-a-policy-applies) | See the **Cloud session** tab under [Install a plugin](/docs/es/plugins/install#install-a-plugin) |

En una ejecución `-p` o CI, los marketplaces y plugins se instalan en segundo plano, por lo que un plugin puede faltar en el primer turno. Establezca `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` para hacer que la ejecución espere la instalación antes de su primera consulta.

<h3 id="confirm-the-rollout">
  Confirm the rollout
</h3>

Verifique que el marketplace y los plugins llegaron a una máquina o en una ejecución de CI:

* **En una máquina**: inicie Claude Code y ejecute `/plugin`. El marketplace y los plugins se enumeran.
* **En CI**: ejecute `claude -p` con `--output-format stream-json --verbose`. El evento `init` enumera los plugins cargados bajo `plugins`.

<h2 id="seed-containers-and-ci">
  Seed containers and CI
</h2>

Para imágenes de contenedor y ejecutores de CI que no pueden clonar en tiempo de ejecución, pre-rellene un directorio de plugins en tiempo de compilación y apunte `CLAUDE_CODE_PLUGIN_SEED_DIR` a él. Claude Code registra los marketplaces del seed al inicio y carga cachés de plugins desde el seed en su lugar, sin clonar.

Un seed también sirve a usuarios que no tienen una cuenta de host git.

<Note>
  En entornos CI/CD, configure un asistente de credenciales git antes de instalar plugins desde repositorios privados. En GitHub Actions, exporte un token con acceso de lectura al repositorio del marketplace como `GH_TOKEN`, luego ejecute `gh auth setup-git`. El token de flujo de trabajo predeterminado solo puede acceder al repositorio del flujo de trabajo, por lo que un marketplace privado en otro repositorio necesita un token de acceso personal o un token de aplicación.
</Note>

<Steps>
  <Step title="Install into the seed at build time">
    Establezca `CLAUDE_CODE_PLUGIN_CACHE_DIR` en la ruta del seed para que el marketplace y los plugins se instalen allí en lugar de `~/.claude/plugins`:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    El seed tiene el mismo diseño que `~/.claude/plugins`: `known_marketplaces.json`, `marketplaces/<name>/`, y `cache/<marketplace>/<plugin>/<version>/`. Puede montar el seed en una ruta diferente a la que lo compiló.
  </Step>

  <Step title="Point the runtime at the seed">
    Establezca `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` en el entorno del contenedor. Para usar varios seeds, separe sus rutas con `:` en Unix o `;` en Windows. Claude Code usa el primer seed que contiene un marketplace o caché de plugin dado.
  </Step>

  <Step title="Enable the plugins">
    Los plugins en un seed no se habilitan por sí solos. Establezca `enabledPlugins` para cada plugin de seed que desee cargar, en la configuración administrada o en el `.claude/settings.json` del repositorio.
  </Step>
</Steps>

Para verificar un seed, ejecute `claude -p` con `--output-format stream-json --verbose` en la imagen. En la lista `plugins` del evento `init`, la `path` de cada plugin cargado está bajo el seed, como `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

Los marketplaces de seed siguen estas reglas:

* **Read-only**: Claude Code nunca escribe en el seed y fuerza `autoUpdate` apagado para marketplaces de seed.
* **Las entradas de seed tienen precedencia**: en cada inicio, un marketplace declarado en el seed sobrescribe la entrada del usuario con el mismo nombre. Los usuarios optan por no participar en un plugin de seed con `claude plugin disable`, no eliminando el marketplace.
* **La actualización y eliminación fallan**: `claude plugin marketplace update <name>` y `remove` sin `--scope` en un marketplace de seed fallan con un mensaje que nombra el directorio del seed.
* **La política aún se aplica**: la [allowlist y blocklist](#restrict-what-users-can-install) verifican la fuente registrada de un marketplace de seed también. Permita la fuente desde la que compiló el seed.

Para flotas sin acceso git saliente, combine un seed con fuentes de marketplace `directory` o `file` en un montaje compartido. Establezca también `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, que también desactiva [plugin auto-update](/docs/es/plugins/loading#when-auto-update-runs). Si hay un proxy disponible, consulte [Proxy configuration](/docs/es/network-config#proxy-configuration) para las variables a establecer.

<h2 id="restrict-what-users-can-install">
  Restrict what users can install
</h2>

La allowlist administrada `strictKnownMarketplaces` y la blocklist `blockedMarketplaces` deciden de qué fuentes de marketplace pueden venir los plugins. La fuente de un marketplace es el repositorio git, URL o ruta local que Claude Code obtiene de él. Ambas listas coinciden con la fuente del marketplace del que viene un plugin, no con la entrada propia del plugin dentro de ese marketplace.

Para el bloqueo común, que permite el marketplace oficial y el suyo, consulte [Allow the official marketplace and your own](#allow-the-official-marketplace-and-your-own). Emparéjelo con [`disableSideloadFlags`](#control-matrix) para que los usuarios no puedan cargar plugins desde un directorio local o URL tampoco.

Ambas listas se aplican antes de que se descargue nada y nuevamente al inicio de la sesión:

* **Antes de una descarga**: las listas se aplican cuando un usuario agrega un marketplace y en cada instalación, actualización, actualización y auto-actualización.
* **Al inicio de la sesión**: las listas se aplican nuevamente a los plugins que ya están instalados, por lo que un plugin instalado cuya fuente de marketplace ya no coincide no se carga. `/plugin` lo enumera con `Marketplace "<name>" is not in the allowed marketplace list` o `Marketplace "<name>" is blocked by enterprise policy`.

Dónde se aplican las dos listas depende de dónde las establezca:

* **La consola de administración de claude.ai**: Claude Code aplica ambas listas en las sesiones que [leen configuración administrada por servidor](/docs/es/managed-settings#where-and-when-a-policy-applies). claude.ai también las verifica cuando alguien en su organización agrega un nuevo marketplace desde un repositorio git en claude.ai, o desde **Customize** en la aplicación de escritorio de Claude fuera de su pestaña Code. Eso cubre un marketplace que un miembro agrega para su propia cuenta y uno agregado para toda la organización bajo [**Organization settings > Plugins**](https://claude.ai/admin-settings/plugins). claude.ai rechaza un repositorio que la allowlist no admite o que la blocklist nombra. No vuelve a verificar un marketplace que se agregó en cualquiera de los dos lugares antes de establecer las listas, y no verifica plugins cargados.
* **Un archivo de configuración administrada, política de nivel de SO u otra fuente administrada**: Claude Code aplica ambas listas donde lee esa fuente. claude.ai no la lee.

Mientras se establezca cualquier allowlist, o una blocklist nombre cualquier fuente que no sea [`skills-dir`](#blocklist-with-blockedmarketplaces), un plugin cuyo marketplace Claude Code no puede encontrar no se carga. `/plugin` muestra el error de política para él en lugar de un error de no encontrado. El caso común es una entrada `enabledPlugins` obsoleta para un marketplace que nadie registró.

<h3 id="control-matrix">
  Control matrix
</h3>

La tabla enumera cada clave de política de plugin, lo que aplica y lo que no puede hacer.

| Key                                                                      | What it enforces                                                                                                                                                                                                                                                                             | What it can't do                                                                                                                                                                                                   |
| :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | Allowlist de fuentes de marketplace. `[]` bloquea cada fuente, incluido el marketplace oficial. Alias: `allowedMarketplaces`                                                                                                                                                                 | No registra un marketplace, restringe entradas dentro de un marketplace permitido, o bloquea `--plugin-dir`                                                                                                        |
| `blockedMarketplaces`                                                    | Blocklist de fuentes de marketplace, verificada antes de la allowlist                                                                                                                                                                                                                        | No bloquea un marketplace ya registrado desde una fuente que no coincide                                                                                                                                           |
| `syncClaudeAiPlugins`                                                    | Establezca `false` para detener que Claude Code descargue y cargue los plugins [sincronizados desde claude.ai](/docs/es/plugins/loading#synced-plugins) para la cuenta de cada usuario. Requiere Claude Code v2.1.273 o posterior                                                                 | No desactiva un plugin sincronizado. Para eso, establezca `"<name>@synced": false` en [`enabledPlugins`](/docs/es/settings-reference#enabledplugins)                                                                    |
| `enabledPlugins`                                                         | `true` fuerza la habilitación, `false` bloquea en todos los alcances y oculta el plugin                                                                                                                                                                                                      | No instala un plugin cuyo marketplace no está registrado o permitido                                                                                                                                               |
| `disableSideloadFlags`                                                   | Rechaza `--plugin-dir`, `--plugin-url`, `--agents`, la opción `plugins` del SDK de Agent, y `--mcp-config` no SDK al inicio, y rechaza carpetas nombradas en la variable [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/es/env-vars#variables) de la misma manera                                              | No restringe `.mcp.json`, `claude mcp add`, o servidores proporcionados por SDK. Emparéjelo con [`allowedMcpServers`](/docs/es/managed-mcp)                                                                             |
| `disableCommandPluginSources`                                            | Bloquea plugins con una fuente `command` de instalar, actualizar o cargar. Una fuente `command` es una cuyo directorio de plugin se produce ejecutando un comando en la máquina. Cuando no se establece, toma el valor de `allowManagedHooksOnly`                                            | No afecta otros tipos de fuente                                                                                                                                                                                    |
| `allowManagedHooksOnly`                                                  | Restringe qué hooks se ejecutan. Consulte [`allowManagedHooksOnly`](/docs/es/settings-reference#allowmanagedhooksonly)                                                                                                                                                                            | No confía en hooks de plugins que los usuarios habilitan a sí mismos                                                                                                                                               |
| `strictPluginOnlyCustomization`                                          | Bloquea skills, agents, hooks y servidores MCP que no vienen de un plugin, configuración administrada o built-ins de Claude Code. Establezca `true` para cubrir los cuatro tipos, o una matriz de valores `skills`, `agents`, `hooks` y `mcp` como `["skills", "hooks"]` para cubrir algunos | No restringe qué plugins instalan los usuarios. Emparéjelo con `strictKnownMarketplaces`                                                                                                                           |
| `pluginSuggestionMarketplaces`                                           | Marketplaces cuyos plugins pueden aparecer como sugerencias de instalación. Consulte [Recommend plugins](#recommend-plugins)                                                                                                                                                                 | No afecta los consejos built-in                                                                                                                                                                                    |
| `pluginTrustMessage`                                                     | Agrega su texto al aviso de confianza que `/plugin` muestra antes de que se instale un plugin                                                                                                                                                                                                | No cambia el texto del aviso en sí                                                                                                                                                                                 |
| `allowedChannelPlugins`                                                  | Reemplaza la lista predeterminada de plugins permitidos para enviar mensajes de canal. Requiere `channelsEnabled: true`                                                                                                                                                                      | Consulte [Restrict which channel plugins can run](/docs/es/channels#restrict-which-channel-plugins-can-run)                                                                                                             |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/es/env-vars) | Detiene las sesiones de terminal interactivas de auto-registrar el marketplace oficial                                                                                                                                                                                                       | No elimina un marketplace ya registrado. La allowlist y blocklist cierran el mismo auto-registro sin él. Una máquina que comenzó una vez con él establecido no reanuda el auto-registro después de desestablecerlo |

Cada clave en la tabla es una configuración administrada, aparte de `enabledPlugins`, `syncClaudeAiPlugins` y `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`:

* **`enabledPlugins`**: puede establecerla en cualquier alcance, y la configuración administrada la bloquea.
* **`syncClaudeAiPlugins`**: cada usuario también puede establecerla en su propia configuración de usuario o local. Consulte su [alcance en la referencia de configuración](/docs/es/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: esta es una variable de entorno que entrega a través del bloque `env` administrado que se muestra bajo [Turn updates off for the whole fleet](#turn-updates-off-for-the-whole-fleet).

Cada clave de configuración aquí tiene una entrada en la [settings reference](/docs/es/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Aliases for the marketplace keys
</h4>

`strictKnownMarketplaces` también puede escribirse como `allowedMarketplaces`, y `extraKnownMarketplaces` también puede escribirse como `additionalMarketplaces`.

* **Version**: los alias requieren Claude Code v2.1.232 o posterior, y los clientes más antiguos los ignoran. En un archivo que una flota mixta lee, mantenga los nombres canónicos.
* **Ambas grafías establecidas**: cuando un archivo establece ambas grafías, se aplica el valor de la clave canónica.

<h3 id="allowlist-with-strictknownmarketplaces">
  Allowlist with `strictKnownMarketplaces`
</h3>

Establezca la allowlist en una lista de estos objetos de fuente. La mayoría de las entradas coinciden exactamente, las entradas `hostPattern` y `pathPattern` coinciden como expresiones regulares, y los comodines de propietario de `github` coinciden por propietario:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, con `ref` y `path` opcionales.
* **Comodín de propietario de `github`**: `{ "source": "github", "repo": "your-org/*" }` coincide con cada repositorio bajo ese propietario. El `*` debe representar el nombre completo del repositorio. Claude Code ignora entradas como `*/plugins` y `your-org/tools-*` como inválidas, por lo que no coinciden con nada. Requiere Claude Code v2.1.223 o posterior.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, con `ref` y `path` opcionales.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, con `headers` opcionales.
* **`file` y `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` o `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, con rutas absolutas.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, coincidido contra el host de fuentes `github`, `git` y `url`. El patrón coincide en cualquier lugar del nombre de host, así que anclelo con `^` y `$` como se muestra para coincidir con el host completo. Una fuente `github` siempre cuenta como `github.com`. Use una entrada `hostPattern` para un GitHub Enterprise Server o host GitLab donde los desarrolladores crean sus propios marketplaces. La [página GHES](/docs/es/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) tiene el ejemplo trabajado.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, coincidido contra la `path` de fuentes `file` y `directory`. El patrón coincide en cualquier lugar de la ruta, así que comience con `^` para fijar un prefijo de directorio. `".*"` permite cada ruta local.
* **`skills-dir`**: `{ "source": "skills-dir" }` mantiene [skills-directory plugins](#keep-skills-directory-plugins-loading) cargándose mientras se establece una allowlist, y no coincide con ningún marketplace.

<h4 id="how-entries-match">
  How entries match
</h4>

Una entrada `url` coincide en su valor `url`; `headers` no se comparan. Para entradas `github` y `git`, el `repo` o `url`, el `ref` y la `path` deben coincidir todos, o estar ausentes en ambos lados:

* Una entrada sin `ref` no cubre una fuente con `ref: "main"`.
* Una entrada para `your-org/your-marketplace` no cubre una URL `git` que clona el mismo repositorio.
* Una barra diagonal final, un sufijo `.git` o `ssh://` en lugar de `https://` es un valor diferente. Cuando un marketplace puede clonarse por más de una URL, prefiera una entrada `hostPattern`.

Las entradas de comodín de propietario siguen las reglas exactas para `ref` y coinciden con cualquier `path` dentro del repositorio a menos que la entrada fije una. La coincidencia de comodín distingue mayúsculas de minúsculas en la allowlist.

<h4 id="keep-skills-directory-plugins-loading">
  Keep skills-directory plugins loading
</h4>

Los plugins de directorio de skills son los plugins que los usuarios mantienen bajo `~/.claude/skills/` o el `.claude/skills/` de un proyecto en carpetas que llevan un `.claude-plugin/plugin.json`. Si establece cualquier allowlist sin una entrada `{ "source": "skills-dir" }`, dejan de cargarse. Los [skills](/docs/es/skills) simples, es decir, un `SKILL.md` sin ese manifiesto, siguen cargándose.

<h4 id="marketplaces-hosted-on-claude-ai">
  Marketplaces hosted on claude.ai
</h4>

La allowlist y blocklist coinciden con un [marketplace alojado en claude.ai](/docs/es/plugins/install#add-from-claude-ai) por su host. Para permitir o bloquear uno, agregue una entrada `hostPattern` que coincida con `claude.ai` a `strictKnownMarketplaces` o `blockedMarketplaces`. En la allowlist, tal entrada admite los marketplaces de claude.ai de su organización y los marketplaces predeterminados de claude.ai, pero no un marketplace hecho de cargas propias de claude.ai de un miembro o uno cuyo alcance claude.ai no declaró. Requiere Claude Code v2.1.273 o posterior.

<h4 id="lock-every-source-out">
  Lock every source out
</h4>

Una allowlist vacía, `[]`, bloquea cada fuente de marketplace, incluido el marketplace oficial.

Este bloqueo no cubre los plugins [sincronizados desde claude.ai](/docs/es/plugins/loading#synced-plugins), que Claude Code descarga de la cuenta de cada usuario en lugar de desde un marketplace. Para detener esos también, establezca [`syncClaudeAiPlugins`](/docs/es/settings-reference#syncclaudeaiplugins) en `false` en la configuración administrada, o desactive Skills para su organización en claude.ai.

<h3 id="blocklist-with-blockedmarketplaces">
  Blocklist with `blockedMarketplaces`
</h3>

`blockedMarketplaces` toma los mismos objetos de fuente que [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) y se verifica primero, por lo que una fuente en ambas listas se bloquea. La coincidencia de blocklist es más amplia que la coincidencia de allowlist:

* Las URLs de Git se canonicalizan, por lo que las formas `git@` y `https://`, sufijos `.git` y barras diagonales finales de un repositorio `github.com` coinciden con la misma entrada.
* Una entrada `github` también bloquea la URL `git` equivalente, y viceversa.
* Para una entrada `owner/*`, la comparación de propietario no distingue mayúsculas de minúsculas.
* Una entrada sin `ref` o `path` bloquea cada ref y path de los repositorios que coincide.

Esta entrada bloquea cada repositorio bajo un propietario de GitHub:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

Las entradas `url` en `blockedMarketplaces` también se aplican cuando un usuario agrega una URL de repositorio `https://` que Claude Code [clona en lugar de obtener](/docs/es/plugins/cli-reference#plugin-marketplace-add), como una URL de repositorio `github.com` o `gitlab.com` simple. El usuario no puede agregar esa URL si una entrada la nombra. La coincidencia ignora el sufijo `.git` y cualquier ref que el usuario agregue después de `#`. Requiere Claude Code v2.1.232 o posterior.

Una entrada `{ "source": "skills-dir" }` aquí detiene [skills-directory plugins](#keep-skills-directory-plugins-loading) de cargarse, desde `~/.claude/skills/` y el `.claude/skills/` de un proyecto.

Una blocklist que nombra solo esa entrada no cuenta como una restricción activa, por lo que no [detiene plugins cuyo marketplace Claude Code no puede encontrar](#restrict-what-users-can-install) de cargarse.

<h3 id="allow-the-official-marketplace-and-your-own">
  Allow the official marketplace and your own
</h3>

La mayoría de las organizaciones permiten el marketplace oficial y el suyo, y registran ambos para que cada máquina los tenga. Esta política de configuración administrada permite ambos marketplaces, registra ambos, fuerza la habilitación de dos plugins y rechaza `--plugin-dir`:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

En una máquina con esta política, agregar cualquier fuente fuera de la lista, por ejemplo `/plugin marketplace add https://example.com/other-marketplace.git`, falla con un mensaje que contiene `is blocked by enterprise policy` seguido de las fuentes permitidas. `claude --plugin-dir ./x` sale con un mensaje que nombra `disableSideloadFlags`.

La entrada `{ "source": "skills-dir" }` mantiene [skills-directory plugins](#keep-skills-directory-plugins-loading) cargándose bajo esta allowlist. Elimine esa entrada y dejan de cargarse.

Registre ambos marketplaces con entradas explícitas `extraKnownMarketplaces`, como esta política hace, en lugar de confiar en la allowlist o en el marketplace oficial registrándose a sí mismo:

* **La allowlist no registra nada**: una entrada `extraKnownMarketplaces` lo hace, y debe pasar la allowlist. Claude Code rechaza registrar un marketplace administrado cuya fuente la allowlist no coincide.
* **El marketplace oficial se registra a sí mismo solo en una sesión de terminal interactiva**: incluso allí, se registra solo cuando la allowlist lo permite. Una ejecución `-p` o una terminal adjunta a una sesión en la nube nunca lo registra.
* **Un intento bloqueado se recuerda**: si una máquina alguna vez se ejecutó bajo una política que bloqueaba el marketplace oficial, Claude Code registra el intento bloqueado y no lo reintenta después de que cambia la política. Un bloqueo `[]` es una política de ese tipo. Esa máquina lo registra nuevamente solo a través de una entrada `extraKnownMarketplaces` como la de esta política, una entrada `enabledPlugins` para uno de sus plugins, o un `/plugin marketplace add` manual.

<h2 id="set-update-policy">
  Set update policy
</h2>

Puede establecer la política de actualización por marketplace, para toda la flota, o por grupo de usuarios a través de canales de lanzamiento.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Turn auto-update on or off per marketplace
</h3>

La auto-actualización de plugins se ejecuta en segundo plano después del inicio para marketplaces que la tienen activada. Para qué marketplaces la tienen activada de forma predeterminada, consulte [When auto-update runs](/docs/es/plugins/loading#when-auto-update-runs). Para decidir para la flota, establezca `"autoUpdate": true` o `false` en una entrada `extraKnownMarketplaces` administrada:

* Si la entrada administrada establece el campo, Claude Code rechaza el toggle `/plugin` del usuario con un error que comienza `Auto-update for '<name>' is set by`.
* Si la entrada administrada deja el campo sin establecer, el toggle del usuario persiste.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Turn updates off for the whole fleet
</h3>

Para desactivar la auto-actualización de plugins para cada marketplace, establezca `DISABLE_AUTOUPDATER` en el bloque `env` administrado, como hace este ejemplo. La misma variable también detiene las actualizaciones propias de Claude Code:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Para detener las actualizaciones propias de Claude Code pero mantener la auto-actualización de plugins, agregue `"FORCE_AUTOUPDATE_PLUGINS": "1"` al mismo bloque. Las otras [variables de entorno que detienen la auto-actualización de plugins](/docs/es/plugins/loading#when-auto-update-runs) funcionan de la misma manera.

`DISABLE_AUTOUPDATER` no cubre plugins con una [fuente `command`](/docs/es/plugins/marketplace-reference#command-plugin-source). Claude Code re-ejecuta el comando de cada uno habilitado cada sesión e instala la salida cuando cambió. Para lo que detiene esas ejecuciones, consulte [When a command source re-runs](/docs/es/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Assign release channels to user groups
</h3>

Para ejecutar canales estables y de acceso temprano, aloje dos marketplaces que apunten a diferentes refs de los mismos plugins. Luego dé a cada grupo de usuarios su propio marketplace a través de configuración administrada por punto final separada o una política de puerta de enlace. La configuración administrada por servidor desde la consola de administración [se aplica a cada usuario en su organización](/docs/es/server-managed-settings#current-limitations), por lo que no pueden asignar diferentes configuraciones a diferentes grupos.

* Implemente [configuración administrada por punto final](/docs/es/managed-settings#delivery-mechanisms) separada, como un archivo de configuración administrada o un perfil MDM, en los dispositivos de cada grupo. Para verificar si el archivo o perfil por grupo se aplica en un dispositivo que también tiene una fuente de toda la organización, consulte [How Claude Code combines managed sources](/docs/es/managed-settings#precedence-within-the-managed-tier).
* Defina una [política de puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway-config#managed) por grupo. La puerta de enlace aplica la primera política cuya regla de coincidencia se ajusta a un usuario, así que ordene las políticas para que cada usuario llegue a la política de su grupo. El mapa `extraKnownMarketplaces` de esa política no se fusiona con el de ninguna otra política, así que enumere cada marketplace que el grupo necesita en él, no solo su marketplace de canal.

Con cualquiera de los mecanismos, el grupo estable recibe esta configuración:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

El grupo de acceso temprano recibe `latest-tools` en su lugar. Para configurar los dos marketplaces, consulte [Run release channels](/docs/es/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Recommend plugins
</h2>

Los propietarios de marketplace pueden adjuntar señales `relevance` a entradas para que Claude Code sugiera el plugin cuando un proyecto coincida.

Las sugerencias de un marketplace aparecen solo cuando está registrado en la máquina del usuario, enumera su nombre en `pluginSuggestionMarketplaces` en la configuración administrada, y declara su fuente en la misma política. Declare la fuente como la entrada `extraKnownMarketplaces` del marketplace o como una entrada de allowlist. El marketplace oficial solo necesita el nombre. Consulte [Enable suggestions in managed settings](/docs/es/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Audit and review
</h2>

Los eventos de OpenTelemetry y la API de Analytics le dicen qué instala y ejecuta su flota.

Para lo que un plugin puede ejecutar en una máquina y lo que cada nivel de confianza permite, lea [Plugin security](/docs/es/plugins/security) antes de aprobar un marketplace.

<h3 id="opentelemetry-events">
  OpenTelemetry events
</h3>

`claude_code.plugin_installed` registra cada instalación, y `claude_code.plugin_loaded` registra cada plugin habilitado al inicio de la sesión. Ambos eventos redactan u omiten nombres de plugins y marketplaces de terceros a menos que establezca `OTEL_LOG_TOOL_DETAILS=1`, como [Redacted plugin names in your backend](/docs/es/plugins/measure#redacted-plugin-names-in-your-backend) muestra. Las listas de campos están bajo [Plugin installed event](/docs/es/monitoring-usage#plugin-installed-event) y [Plugin loaded event](/docs/es/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  Analytics API
</h3>

En el plan Enterprise, `GET /v1/organizations/analytics/plugins` devuelve recuentos de instalación e invocación por plugin, por día en Claude Code y Cowork. Puede agrupar los recuentos por usuario o grupo RBAC. La actividad de plugins que llega a Anthropic sin un nombre de plugin aparece en una fila `third-party` agregada. Consulte la [referencia de punto final](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) y [Access data programmatically](/docs/es/analytics#access-data-programmatically) para la clave que necesita.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Plan for what managed settings can't enforce
</h2>

Estas solicitudes de revisiones de seguridad no tienen una clave dedicada en el esquema de configuración actual. Los controles existentes más cercanos son:

* **Orientación por usuario o por grupo**: cada clave de plugin se aplica a cada usuario que recibe la configuración. La configuración administrada por servidor entrega una configuración por organización. Para política por grupo, use configuración administrada por punto final separada o políticas de puerta de enlace, como bajo [Assign release channels to user groups](#assign-release-channels-to-user-groups).
* **Restricción de entradas dentro de un marketplace permitido**: la allowlist coincide con fuentes de marketplace. Para bloquear un plugin de un marketplace permitido, establézcalo en `false` en `enabledPlugins` administrado.
* **Ocultación de `/plugin`**: ninguna clave desactiva el comando. El equivalente más cercano combina una allowlist que nombra solo su marketplace, entradas `enabledPlugins` administradas para los plugins que suministra, y `disableSideloadFlags`.
* **Cierre de `--plugin-dir` a través de la allowlist**: la allowlist no cubre `--plugin-dir`. `disableSideloadFlags` lo hace.
* **Aplicación de los toggles de plugin de claude.ai a través de estas claves**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) no establece las claves en esta página. Lo que los miembros y su organización activan allí llega a la CLI como [synced plugins](/docs/es/plugins/loading#synced-plugins), que tienen sus propios controles.

<h2 id="troubleshoot-policy">
  Troubleshoot policy
</h2>

Si la política de plugin no se comporta como se espera en una máquina, verifique primero estos síntomas:

* **El archivo administrado no se analizó**: cuando un `managed-settings.json` no es JSON válido, Claude Code rechaza iniciar e imprime [un error que nombra el archivo](/docs/es/errors#managed-settings-document-could-not-be-parsed). Un archivo que se analiza pero tiene una entrada inválida mantiene el resto de su política. Consulte [Invalid entries in managed settings](/docs/es/managed-settings#invalid-entries-in-managed-settings).
* **La fuente administrada no se cargó**: ejecute `/status` y busque `Enterprise managed settings` en la línea `Setting sources`. Si falta, la fuente no se cargó.
* **Un usuario reporta `blocked by enterprise policy`**: el mensaje nombra el marketplace o su fuente. Para una allowlist, también enumera las fuentes permitidas. Las entradas orientadas al usuario están en [Troubleshoot plugins](/docs/es/plugins/troubleshooting).
* **Un plugin que el usuario deshabilitó en `~/.claude/settings.json` aún se carga**: otra fuente de configuración lo re-habilitó, como una entrada `enabledPlugins` administrada que lo fuerza-habilita. `/plugin` y `claude plugin list` muestran `Disabled in ~/.claude/settings.json but still loads` con esa fuente de configuración.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/es/plugins/marketplace-reference#marketplace-sources): los valores `source` que `extraKnownMarketplaces`, `strictKnownMarketplaces` y `blockedMarketplaces` aceptan
* [Host and maintain a marketplace](/docs/es/plugins/host-marketplace): ejecute el marketplace al que apunta su política
* [Plugin security and trust](/docs/es/plugins/security): lo que un plugin puede hacer en una máquina y cómo revisar uno antes de instalar
* [Server-managed settings](/docs/es/server-managed-settings): entregue estas claves desde la consola de administración de claude.ai
* [Troubleshoot plugins](/docs/es/plugins/troubleshooting#blocked-by-your-organization): los mensajes que ven los usuarios cuando la política los bloquea
