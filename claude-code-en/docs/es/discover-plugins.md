> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descubra e instale plugins pregenerados a través de mercados

> Encuentre e instale plugins de mercados para extender Claude Code con nuevas skills, agentes y capacidades.

Los plugins extienden Claude Code con skills, agentes, hooks y servidores MCP. Los mercados de plugins son catálogos que le ayudan a descubrir e instalar estas extensiones sin construirlas usted mismo.

¿Busca crear y distribuir su propio mercado? Consulte [Crear y distribuir un mercado de plugins](/docs/es/plugin-marketplaces).

<h2 id="how-marketplaces-work">
  Cómo funcionan los mercados
</h2>

Un mercado es un catálogo de plugins que alguien más ha creado y compartido. Usar un mercado es un proceso de dos pasos:

<Steps>
  <Step title="Agregar el mercado">
    Esto registra el catálogo con Claude Code para que pueda explorar lo que está disponible. Aún no se instalan plugins.
  </Step>

  <Step title="Instalar plugins individuales">
    Explore el catálogo e instale los plugins que desee.
  </Step>
</Steps>

Piénselo como agregar una tienda de aplicaciones: agregar la tienda le da acceso para explorar su colección, pero usted sigue eligiendo qué aplicaciones descargar individualmente.

<h2 id="official-anthropic-marketplace">
  Mercado oficial de Anthropic
</h2>

El mercado oficial de Anthropic (`claude-plugins-official`) está disponible automáticamente cuando inicia Claude Code. Ejecute `/plugin` y vaya a la pestaña **Discover** para explorar lo que está disponible, o vea el catálogo en [claude.com/plugins](https://claude.com/plugins).

Para instalar un plugin del mercado oficial, use `/plugin install <name>@claude-plugins-official`. Por ejemplo, para instalar la integración de GitHub:

```shell theme={null}
/plugin install github@claude-plugins-official
```

Si Claude Code reporta que el plugin no se encuentra en ningún mercado, su mercado está faltando o desactualizado. Ejecute `/plugin marketplace update claude-plugins-official` para actualizarlo, o `/plugin marketplace add anthropics/claude-plugins-official` si no lo ha agregado antes. Luego reintente la instalación.

<Note>
  El mercado oficial es curado por Anthropic, y la inclusión está a discreción de Anthropic. Los formularios de envío en la aplicación agregan plugins al [mercado comunitario](#community-marketplace), no al oficial. Para distribuir plugins de forma independiente, [cree su propio mercado](/docs/es/plugin-marketplaces) y compártalo con los usuarios.
</Note>

El mercado oficial incluye varias categorías de plugins:

<h3 id="code-intelligence">
  Code intelligence
</h3>

Los plugins de code intelligence habilitan la herramienta LSP integrada de Claude Code, dándole a Claude la capacidad de saltar a definiciones, encontrar referencias y ver errores de tipo inmediatamente después de ediciones. Estos plugins configuran conexiones de [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), la misma tecnología que potencia la inteligencia de código de VS Code.

Estos plugins requieren que el binario del servidor de lenguaje esté instalado en su sistema. Si ya tiene un servidor de lenguaje instalado, Claude puede solicitarle que instale el plugin correspondiente cuando abra un proyecto.

| Lenguaje   | Plugin              | Binario requerido            |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

También puede [crear su propio plugin LSP](/docs/es/plugins-reference#lsp-servers) para otros lenguajes.

<Note>
  Si ve `Executable not found in $PATH` en la pestaña Errors de `/plugin` después de instalar un plugin, instale el binario requerido de la tabla anterior.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  Lo que Claude gana con los plugins de code intelligence
</h4>

Una vez que se instala un plugin de code intelligence y su binario de servidor de lenguaje está disponible, Claude gana dos capacidades:

* **Diagnósticos automáticos**: después de cada edición de archivo que Claude realiza, el servidor de lenguaje analiza los cambios e informa errores y advertencias automáticamente. Claude ve errores de tipo, importaciones faltantes y problemas de sintaxis sin necesidad de ejecutar un compilador o linter. Si Claude introduce un error, lo nota y corrige el problema en el mismo turno. Esto no requiere configuración más allá de instalar el plugin. Puede ver diagnósticos en línea presionando **Ctrl+O** cuando aparece el indicador "diagnostics found".
* **Navegación de código**: Claude puede usar el servidor de lenguaje para saltar a definiciones, encontrar referencias, obtener información de tipo al pasar el ratón, listar símbolos, encontrar implementaciones y rastrear jerarquías de llamadas. Estas operaciones dan a Claude una navegación más precisa que la búsqueda basada en grep, aunque la disponibilidad puede variar según el lenguaje y el entorno.

Si encuentra problemas, consulte [Solución de problemas de code intelligence](#code-intelligence-issues).

<h3 id="external-integrations">
  External integrations
</h3>

Estos plugins incluyen [servidores MCP](/docs/es/mcp) preconfigurados para que pueda conectar Claude a servicios externos sin configuración manual:

* **Source control**: `github`, `gitlab`
* **Project management**: `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **Design**: `figma`
* **Infrastructure**: `vercel`, `firebase`, `supabase`
* **Communication**: `slack`
* **Monitoring**: `sentry`

<h3 id="automatic-security-review">
  Automatic security review
</h3>

El plugin `security-guidance` revisa cada cambio que Claude realiza en busca de vulnerabilidades comunes e instruye a Claude para que corrija lo que encuentra en la misma sesión. Consulte [Detectar problemas de seguridad mientras Claude escribe código](/docs/es/security-guidance) para ver qué verifica y cómo agregar reglas específicas del proyecto.

<h3 id="development-workflows">
  Development workflows
</h3>

Plugins que agregan skills y agentes para tareas de desarrollo comunes:

* **commit-commands**: Flujos de trabajo de confirmación de Git incluyendo confirmación, push y creación de PR
* **pr-review-toolkit**: Agentes especializados para revisar solicitudes de extracción
* **agent-sdk-dev**: Herramientas para construir con el Claude Agent SDK
* **plugin-dev**: Kit de herramientas para crear sus propios plugins

<h3 id="output-styles">
  Output styles
</h3>

Personalice cómo responde Claude:

* **explanatory-output-style**: Información educativa sobre opciones de implementación
* **learning-output-style**: Modo de aprendizaje interactivo para construcción de skills

<h2 id="community-marketplace">
  Community marketplace
</h2>

El mercado comunitario en [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) aloja plugins de terceros que han pasado la validación automatizada de Anthropic y el análisis de seguridad. Cada plugin está fijado a un SHA de confirmación específico en el catálogo. A diferencia del mercado oficial, lo agrega manualmente:

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

Luego instale plugins desde él usando el nombre de mercado `claude-community`:

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

Para enviar su propio plugin al mercado comunitario, consulte [Enviar su plugin al mercado comunitario](/docs/es/plugins#submit-your-plugin-to-the-community-marketplace) en la guía de creación de plugins.

<h2 id="try-it-add-the-demo-marketplace">
  Pruébelo: agregue el mercado de demostración
</h2>

Anthropic también mantiene un [mercado de plugins de demostración](https://github.com/anthropics/claude-code/tree/main/plugins) (`claude-code-plugins`) con plugins de ejemplo que muestran lo que es posible con el sistema de plugins. A diferencia del mercado oficial, debe agregar este manualmente.

<Steps>
  <Step title="Agregar el mercado">
    Desde dentro de Claude Code, ejecute el comando `plugin marketplace add` para el mercado `anthropics/claude-code`:

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    Esto descarga el catálogo del mercado y pone sus plugins a su disposición.
  </Step>

  <Step title="Explorar plugins disponibles">
    Ejecute `/plugin` para abrir el administrador de plugins. Esto abre una interfaz con pestañas con cuatro pestañas por las que puede ciclar usando **Tab**, o **Shift+Tab** para ir hacia atrás:

    * **Discover**: explore plugins disponibles de todos sus mercados
    * **Installed**: vea y administre sus plugins instalados
    * **Marketplaces**: agregue, elimine o actualice sus mercados agregados
    * **Errors**: vea cualquier error de carga de plugins

    Vaya a la pestaña **Discover** para ver plugins del mercado que acaba de agregar. Cuando su administrador ha incluido en la lista blanca el mercado a través de la configuración administrada [`pluginSuggestionMarketplaces`](/docs/es/settings#available-settings), los plugins marcados como relevantes para su directorio de trabajo actual se fijan en la parte superior con una etiqueta **suggested for this directory**.
  </Step>

  <Step title="Instalar un plugin">
    Seleccione un plugin para ver sus detalles. El panel de detalles muestra lo que contiene el plugin y cuánto cuesta:

    * Una estimación de **Context cost** para que pueda ver cuántos tokens el plugin agregará a su [ventana de contexto](/docs/es/features-overview#understand-context-costs) en cada turno (Claude Code v2.1.143 y posteriores)
    * La fecha de **Last updated** del plugin (v2.1.144 y posteriores)
    * Una sección **Will install** que enumera los comandos, agentes, skills, hooks y servidores MCP y LSP del plugin, para que pueda revisar exactamente qué agrega antes de instalar (v2.1.145 y posteriores)

    Elija un alcance de instalación:

    * **User scope**: instale para usted en todos los proyectos
    * **Project scope**: instale para todos los colaboradores en este repositorio
    * **Local scope**: instale para usted en este repositorio solamente

    Por ejemplo, seleccione **commit-commands**, un plugin que agrega skills de flujo de trabajo de git, e instálelo en su alcance de usuario.

    También puede instalar directamente desde la línea de comandos:

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    Consulte [Configuration scopes](/docs/es/settings#configuration-scopes) para obtener más información sobre alcances.
  </Step>

  <Step title="Usar su nuevo plugin">
    Después de instalar, ejecute `/reload-plugins` para activar el plugin. Las skills de plugin tienen espacios de nombres por el nombre del plugin, por lo que **commit-commands** proporciona skills como `/commit-commands:commit`.

    Pruébelo haciendo un cambio en un archivo y ejecutando:

    ```shell theme={null}
    /commit-commands:commit
    ```

    Esto prepara sus cambios, genera un mensaje de confirmación y crea la confirmación.

    Cada plugin funciona de manera diferente. Consulte los detalles del plugin en la pestaña **Discover** para ver los comandos y skills que proporciona, o visite su página de inicio para obtener orientación sobre el uso.
  </Step>
</Steps>

El resto de esta guía cubre todas las formas en que puede agregar mercados, instalar plugins y administrar su configuración.

<h2 id="add-marketplaces">
  Agregar mercados
</h2>

Use el comando `/plugin marketplace add` para agregar mercados de diferentes fuentes.

<Tip>
  **Atajos**: Puede usar `/plugin market` en lugar de `/plugin marketplace`, y `rm` en lugar de `remove`.
</Tip>

* **Repositorios de GitHub**: formato `owner/repo` (por ejemplo, `anthropics/claude-code`)
* **URLs de Git**: cualquier URL de repositorio de git, incluyendo GitLab, Bitbucket y servidores auto-hospedados
* **Rutas locales**: directorios o rutas directas a archivos `marketplace.json`
* **URLs remotas**: URLs directas a archivos `marketplace.json` hospedados

<h3 id="add-from-github">
  Agregar desde GitHub
</h3>

Agregue un repositorio de GitHub que contenga un archivo `.claude-plugin/marketplace.json` usando el formato `owner/repo`, donde `owner` es el nombre de usuario o la organización de GitHub y `repo` es el nombre del repositorio.

Por ejemplo, `anthropics/claude-code` se refiere al repositorio `claude-code` propiedad de `anthropics`:

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  Agregar desde otros hosts de Git
</h3>

Agregue cualquier repositorio de git proporcionando la URL completa. Esto funciona con cualquier host de Git, incluyendo GitLab, Bitbucket y servidores auto-hospedados. Incluya el sufijo `.git` para que Claude Code clone el repositorio en lugar de tratar la URL como un enlace directo a un archivo `marketplace.json` hospedado.

Incluya el prefijo `https://` también. Claude Code v2.1.196 y versiones posteriores rechazan un host escrito sin él, como `gitlab.com/company/plugins.git`, como un atajo `owner/repo` de GitHub inválido, y el error le indica que agregue el prefijo. Las versiones anteriores lo malinterpretaron como una ruta de repositorio de GitHub y fallan en el momento de la clonación.

Usando HTTPS:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

Usando SSH:

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

Para agregar una rama o etiqueta específica, agregue `#` seguido de la ref:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  Agregar desde rutas locales
</h3>

Agregue un directorio local que contenga un archivo `.claude-plugin/marketplace.json`:

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

También puede agregar una ruta directa a un archivo `marketplace.json`:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  Agregar desde URLs remotas
</h3>

Agregue un archivo `marketplace.json` remoto a través de URL:

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  Los mercados basados en URL tienen algunas limitaciones en comparación con los mercados basados en Git. Si encuentra errores "path not found" al instalar plugins, consulte [Troubleshooting](/docs/es/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces).
</Note>

<h2 id="install-plugins">
  Instalar plugins
</h2>

Una vez que haya agregado mercados, puede instalar plugins directamente:

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

El comando abre los detalles de ese plugin, donde elige un [alcance de instalación](/docs/es/settings#configuration-scopes). Verá las mismas opciones cuando ejecute `/plugin`, vaya a la pestaña **Discover** y presione **Enter** en un plugin:

* **User scope** (predeterminado): instale para usted en todos los proyectos
* **Project scope**: instale para todos los colaboradores en este repositorio, lo que agrega el plugin a `.claude/settings.json`
* **Local scope**: instale para usted en este repositorio solamente, no compartido con colaboradores

Para instalar sin un paso interactivo, use el comando shell [`claude plugin install`](/docs/es/plugins-reference#plugin-install), que instala en alcance de usuario a menos que pase `--scope`.

También puede ver plugins con alcance **managed**. Estos se instalan por administradores a través de [managed settings](/docs/es/settings#settings-files) y no pueden ser modificados.

<Warning>
  Asegúrese de confiar en un plugin antes de instalarlo. Anthropic no controla qué servidores MCP, archivos u otro software se incluyen en los plugins y no puede verificar que funcionen como se pretende. Consulte la página de inicio de cada plugin para obtener más información.
</Warning>

<h2 id="manage-installed-plugins">
  Administrar plugins instalados
</h2>

Ejecute `/plugin` y vaya a la pestaña **Installed** para ver, habilitar, deshabilitar o desinstalar sus plugins. La lista se agrupa por alcance y se ordena para que vea problemas primero: los plugins con errores de carga o dependencias no resueltas aparecen en la parte superior, seguidos de sus favoritos, con plugins deshabilitados plegados detrás de un encabezado colapsado en la parte inferior.

Desde la lista puede:

* presionar `f` para marcar como favorito o desmarcar como favorito el plugin seleccionado
* escribir para filtrar por nombre o descripción del plugin
* presionar Enter para abrir la vista de detalles de un plugin y habilitarlo, deshabilitarlo o desinstalarlo

Desinstalar un plugin que el `.claude/settings.json` de un proyecto habilita pregunta qué alcance quiere decir: deshabilitarlo solo para usted, lo que escribe una anulación en su `.claude/settings.local.json` y deja el plugin instalado para el proyecto, o desinstalarlo para todos, lo que lo elimina del `.claude/settings.json` compartido. Requiere Claude Code v2.1.203 o posterior. Antes de v2.1.203, el diálogo ofrecía solo la deshabilitación local.

La vista de detalles muestra los componentes que contribuye el plugin: comandos, skills, agentes, hooks, servidores MCP y servidores LSP. El mismo inventario está disponible desde la línea de comandos con `claude plugin details`.

La pestaña **Installed** también recopila plugins del marketplace que instaló usted mismo pero que no ha usado en al menos dos semanas, en un período de al menos 10 sesiones, bajo un encabezado **Not used recently**. La vista de detalles muestra una línea **Last used** para cada plugin. Utilice estos para encontrar plugins que aún añaden costo de inicio y contexto aunque ya no los use, luego deshabilítelos o desinstálelos. Requiere Claude Code v2.1.187 o posterior.

Dos tipos de plugins nunca se enumeran como no utilizados:

* plugins que su organización administra o que carga con `--plugin-dir`
* plugins que contribuyen un tema, estilo de salida, monitor o flujo de trabajo, ya que proporcionan valor sin una invocación para rastrear

El encabezado **Not used recently** y la línea **Last used** se ocultan cuando su organización restringe los marketplaces con [`strictKnownMarketplaces`](/docs/es/settings#strictknownmarketplaces).

Un [servidor de lenguaje](/docs/es/plugins#add-lsp-servers-to-your-plugin) de un plugin se cuenta como usado cuando entrega diagnósticos o responde a una solicitud de navegación de código, por lo que un plugin LSP cuyo servidor está activo en sus sesiones no se enumera como no utilizado. Antes de v2.1.203, la actividad del servidor de lenguaje no se podía contar como uso, por lo que los plugins que contribuyen un servidor LSP estaban exentos del grupo completamente, de la misma manera que los plugins de tema y estilo de salida aún lo están.

La primera sesión en una versión que cuenta la actividad del servidor de lenguaje también restablece el registro de uso de cada plugin LSP que aún no había registrado ningún uso, por lo que Claude Code no juzga un plugin que instaló anteriormente como no utilizado basándose en datos registrados antes de que se rastreara la actividad de su servidor. Antes de v2.1.206, esa primera sesión podría enumerar un plugin LSP usado activamente bajo **Not used recently** y sugerir revisarlo.

Cuando instala un plugin que declara dependencias, la salida de instalación enumera qué dependencias se instalaron automáticamente junto con él.

También puede administrar plugins con comandos directos.

Enumere los plugins instalados sin abrir el menú:

```shell theme={null}
/plugin list
```

Pase `--enabled` o `--disabled` para mostrar solo los plugins en ese estado.

Deshabilite un plugin sin desinstalarlo:

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

Vuelva a habilitar un plugin deshabilitado:

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

En estos identificadores, `plugin-name` es el `name` del plugin en la [entrada del marketplace](/docs/es/plugin-marketplaces#plugin-entries), que puede diferir del `name` en el `plugin.json` del plugin.

A partir de Claude Code v2.1.195, **Enable** y **Disable** en la interfaz `/plugin` funcionan para plugins cuyos dos nombres difieren, y `/plugin enable` y `/plugin disable` aceptan cualquiera de los nombres. Cuando deshabilita tal plugin en una versión anterior, Claude Code reporta `already disabled` y lo deja habilitado.

Elimine completamente un plugin:

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

La opción `--scope` le permite dirigirse a un alcance específico con comandos CLI:

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  Aplicar cambios de plugins sin reiniciar
</h3>

Cuando instala, habilita o deshabilita plugins durante una sesión, ejecute `/reload-plugins` para recopilar todos los cambios sin reiniciar:

```shell theme={null}
/reload-plugins
```

Claude Code recarga todos los plugins activos y muestra conteos para plugins, skills, agentes, hooks, servidores MCP de plugins y servidores LSP de plugins.

La recarga tiene un costo de tokens en la siguiente solicitud: los componentes recién cargados se anuncian a sí mismos en el contenido añadido a la conversación, mientras que el historial existente aún se lee desde la caché de prompts. Un plugin que proporciona servidores MCP cuesta más cuando sus herramientas no se difieren por [búsqueda de herramientas MCP](/docs/es/mcp#scale-with-mcp-tool-search): el cambio invalida la caché y la siguiente solicitud vuelve a leer toda la conversación. En ese caso `/reload-plugins` muestra una advertencia y no aplica la recarga; pase `--force` para aplicarla de todas formas. Consulte [habilitar o deshabilitar un plugin](/docs/es/prompt-caching#enabling-or-disabling-a-plugin) para obtener más detalles.

<h2 id="manage-marketplaces">
  Administrar mercados
</h2>

Puede administrar mercados a través de la interfaz interactiva `/plugin` o con comandos CLI.

<h3 id="use-the-interactive-interface">
  Usar la interfaz interactiva
</h3>

Ejecute `/plugin` y vaya a la pestaña **Marketplaces** para:

* Ver todos sus mercados agregados con sus fuentes y estado
* Agregar nuevos mercados
* Actualizar listados de mercados para obtener los últimos plugins
* Eliminar mercados que ya no necesita

<h3 id="use-cli-commands">
  Usar comandos CLI
</h3>

También puede administrar mercados con comandos directos.

Enumere todos los mercados configurados:

```shell theme={null}
/plugin marketplace list
```

Actualice listados de plugins de un mercado:

```shell theme={null}
/plugin marketplace update marketplace-name
```

Elimine un mercado:

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  Eliminar un mercado desinstalará cualquier plugin que haya instalado desde él.
</Warning>

<h3 id="configure-auto-updates">
  Configurar actualizaciones automáticas
</h3>

Claude Code puede actualizar automáticamente mercados y sus plugins instalados en segundo plano después del inicio. Cuando la actualización automática está habilitada para un mercado, Claude Code actualiza los datos del mercado e actualiza los plugins instalados a sus versiones más recientes en disco.

Claude Code verifica si hay actualizaciones de mercado y plugins después de que inicia su sesión, con un retraso aleatorio de hasta diez minutos, por lo que la sesión en ejecución sigue utilizando las versiones que cargó al inicio. Si se actualizaron plugins, verá una notificación pidiéndole que ejecute `/reload-plugins`, o las nuevas versiones se cargan en su próximo inicio.

Alterne la actualización automática para mercados individuales a través de la interfaz:

1. Ejecute `/plugin` para abrir el administrador de plugins
2. Seleccione **Marketplaces**
3. Elija un mercado de la lista
4. Seleccione **Enable auto-update** o **Disable auto-update**

Los mercados oficiales de Anthropic tienen la actualización automática habilitada por defecto. Los mercados de terceros y de desarrollo local tienen la actualización automática deshabilitada por defecto.

Los administradores también pueden establecer `"autoUpdate": true` en cada entrada [`extraKnownMarketplaces`](/docs/es/settings#extraknownmarketplaces) en la configuración administrada para habilitar la actualización automática para un mercado de la organización sin requerir que cada usuario la active.

Para deshabilitar todas las actualizaciones automáticas completamente tanto para Claude Code como para todos los plugins, establezca la variable de entorno `DISABLE_AUTOUPDATER`. Consulte [Auto updates](/docs/es/setup#auto-updates) para obtener detalles.

Para mantener las actualizaciones automáticas de plugins habilitadas mientras se deshabilitan las actualizaciones automáticas de Claude Code, establezca `FORCE_AUTOUPDATE_PLUGINS=1` junto con `DISABLE_AUTOUPDATER`:

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Esto es útil cuando desea administrar las actualizaciones de Claude Code manualmente pero aún recibir actualizaciones automáticas de plugins.

<h2 id="configure-team-marketplaces">
  Configurar mercados de equipo
</h2>

Los administradores de equipo pueden configurar la instalación automática de mercados para proyectos agregando configuración de mercado a `.claude/settings.json`. Cuando los miembros del equipo confían en la carpeta del repositorio, Claude Code les solicita que instalen estos mercados y plugins.

A partir de Claude Code v2.1.195, este paso de instalación se aplica en cada ruta que carga plugins. Un plugin que solo el `.claude/settings.json` del proyecto habilita, y que proviene de una fuente externa como un repositorio de GitHub o un paquete npm, no se carga hasta que el miembro del equipo lo instale. Hasta entonces, Claude Code reporta el plugin como no instalado y muestra el comando `claude plugin install` para ejecutar.

Agregue `extraKnownMarketplaces` a su `.claude/settings.json` del proyecto:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Para opciones de configuración completas incluyendo `extraKnownMarketplaces` y `enabledPlugins`, consulte [Plugin settings](/docs/es/settings#plugin-settings).

<h2 id="security">
  Seguridad
</h2>

Los plugins y mercados son componentes altamente confiables que pueden ejecutar código arbitrario en su máquina con sus privilegios de usuario. Solo instale plugins y agregue mercados de fuentes en las que confíe. Las organizaciones pueden restringir qué mercados se permite a los usuarios agregar usando [restricciones de mercado administrado](/docs/es/plugin-marketplaces#managed-marketplace-restrictions).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="/plugin-command-not-recognized">
  /plugin command not recognized
</h3>

Si ve "unknown command" o el comando `/plugin` no aparece:

1. **Verifique su versión**: Ejecute `claude --version` para ver qué está instalado.
2. **Actualice Claude Code**:
   * **Homebrew**: `brew upgrade claude-code`, o `brew upgrade claude-code@latest` si instaló ese cask
   * **npm**: `npm install -g @anthropic-ai/claude-code@latest`
   * **Native installer**: Vuelva a ejecutar el comando de instalación desde [Setup](/docs/es/setup)
3. **Reinicie Claude Code**: Después de actualizar, reinicie su terminal y ejecute `claude` nuevamente.

<h3 id="common-issues">
  Common issues
</h3>

* **Marketplace not loading**: Verifique que la URL sea accesible y que `.claude-plugin/marketplace.json` exista en la ruta
* **Plugin installation failures**: Verifique que las URLs de fuente de plugins sean accesibles y que los repositorios sean públicos, o que tenga acceso a ellos
* **Files not found after installation**: Los plugins se copian a un caché, por lo que las rutas que hacen referencia a archivos fuera del directorio del plugin no funcionarán
* **Plugin skills not appearing**: Limpie el caché con `rm -rf ~/.claude/plugins/cache`, reinicie Claude Code e reinstale el plugin.

Para solución de problemas detallada con soluciones, consulte [Troubleshooting](/docs/es/plugin-marketplaces#troubleshooting) en la guía de mercados. Para herramientas de depuración, consulte [Debugging and development tools](/docs/es/plugins-reference#debugging-and-development-tools).

<h3 id="code-intelligence-issues">
  Code intelligence issues
</h3>

* **Language server not starting**: Verifique que el binario esté instalado y disponible en su `$PATH`. Consulte la pestaña Errors de `/plugin` para obtener detalles.
* **High memory usage**: Los servidores de lenguaje como `rust-analyzer` y `pyright` pueden consumir memoria significativa en proyectos grandes. Si experimenta problemas de memoria, deshabilite el plugin con `/plugin disable <plugin-name>` y confíe en las herramientas de búsqueda integradas de Claude en su lugar.
* **False positive diagnostics in monorepos**: Los servidores de lenguaje pueden reportar errores de importación no resuelta para paquetes internos si el espacio de trabajo no está configurado correctamente. Estos no afectan la capacidad de Claude para editar código.

<h2 id="next-steps">
  Próximos pasos
</h2>

* **Cree sus propios plugins**: Consulte [Plugins](/docs/es/plugins) para crear skills, agentes y hooks
* **Cree un marketplace**: Consulte [Cree un marketplace de plugins](/docs/es/plugin-marketplaces) para distribuir plugins a su equipo o comunidad
* **Referencia técnica**: Consulte [Referencia de Plugins](/docs/es/plugins-reference) para especificaciones completas
