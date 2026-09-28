> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins de inteligencia de código

> Instale un plugin de servidor de lenguaje para que Claude vea errores de tipo después de ediciones y navegue el código por símbolo, y responda al diálogo de recomendación del plugin LSP.

Un plugin de inteligencia de código le da a Claude los diagnósticos en vivo y la navegación a definición que tiene su editor, para que Claude detecte errores de tipo e importaciones faltantes que sus propias ediciones introducen antes de ejecutar su compilación, y encuentre definiciones y referencias por símbolo en lugar de por búsqueda de texto.

Cada plugin conecta Claude Code a un servidor de lenguaje para un idioma a través del Protocolo de Servidor de Lenguaje (LSP). Instala el plugin desde el marketplace oficial de Anthropic y el binario del servidor de lenguaje en su máquina.

<Note>
  Los plugins de inteligencia de código funcionan en sesiones de terminal. En [sesiones en la nube](/docs/es/claude-code-on-the-web), Claude Code no inicia servidores de lenguaje de plugins, por lo que Claude no obtiene diagnósticos ni navegación de código allí. Para escribir su propio plugin de servidor de lenguaje, o para conectar un servidor de lenguaje que no tiene plugin, consulte [Servidores LSP en componentes de plugins](/docs/es/plugins/components#lsp-servers).
</Note>

Para comenzar, encuentre su idioma en la tabla bajo [Instalar un plugin de inteligencia de código](#install-a-code-intelligence-plugin). Los plugins en esa tabla provienen del [marketplace oficial de plugins](/docs/es/plugins/anthropic-marketplaces) de Anthropic.

Si ya vio un diálogo de **recomendación de plugin LSP**, consulte [Aceptar o descartar el diálogo de recomendación](#accept-or-dismiss-the-recommendation-dialog) para saber qué hace cada opción.

<h2 id="install-a-code-intelligence-plugin">
  Instalar un plugin de inteligencia de código
</h2>

Un plugin de inteligencia de código le dice a Claude Code qué comando inicia el servidor de lenguaje y qué extensiones de archivo maneja. No incluye el servidor de lenguaje. Instale primero el binario del servidor de lenguaje, luego el plugin, luego confirme que el servidor se inicia.

<Steps>
  <Step title="Instalar el binario del servidor de lenguaje">
    Encuentre su idioma en la tabla a continuación e instale el binario en su fila. Si su idioma no está listado, consulte [Agregar un idioma sin un plugin oficial](#add-a-language-without-an-official-plugin).

    | Idioma                  | Plugin                                                                                                           | Binario                            |
    | :---------------------- | :--------------------------------------------------------------------------------------------------------------- | :--------------------------------- |
    | C/C++                   | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                           |
    | C#                      | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                        |
    | Go                      | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                            |
    | Java                    | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                            |
    | Kotlin                  | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                       |
    | Liquid                  | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, desde la CLI de Shopify |
    | Lua                     | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`              |
    | PHP                     | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`                     |
    | Python                  | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`               |
    | Ruby                    | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                         |
    | Rust                    | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`                    |
    | Swift                   | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`                    |
    | TypeScript y JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server`       |

    Anthropic mantiene cada plugin en la tabla excepto `liquid-lsp`, que Shopify mantiene y el marketplace oficial lista.

    Para encontrar el comando que instala el binario, siga el enlace del plugin en la tabla a su README. Para TypeScript, ese comando es `npm install -g typescript-language-server typescript`.

    Después de instalar el binario, confirme que está en el `PATH` del shell desde el que inicia `claude`, por ejemplo con `which typescript-language-server`, o `Get-Command typescript-language-server` en PowerShell.
  </Step>

  <Step title="Instalar el plugin">
    Para instalar el plugin listado para su idioma en la tabla del paso 1, ejecute `/plugin install` en una sesión de Claude Code, reemplazando `typescript-lsp` con el nombre de ese plugin:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Un mensaje de confirmación dice si el plugin está activo ahora o necesita `/reload-plugins`. Si la instalación falla con `Marketplace "claude-plugins-official" not found`, consulte la [entrada de solución de problemas para ese error](/docs/es/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Para controlar dónde se instala el plugin, o para ejecutar la instalación desde su shell en lugar de dentro de Claude Code, consulte [Instalar plugins](/docs/es/plugins/install).
  </Step>

  <Step title="Confirmar que el servidor se inicia">
    El servidor de lenguaje se inicia la primera vez que Claude edita un archivo con una de las extensiones del plugin. Para verlo funcionar, pida a Claude que introduzca un error de tipo en un archivo de ese idioma y luego lo corrija. Luego verifique la conversación para una línea de diagnósticos:

    * **Aparece una línea de diagnósticos**: `Found N new diagnostic issues in M files (ctrl+o to expand)` bajo la edición que introdujo el error significa que el servidor se inició.
    * **No aparece una línea de diagnósticos**: ejecute `/plugin` y abra la pestaña **Errors**. Una fila que dice `Executable not found in $PATH: "<binary>"` nombra el binario a instalar. Si la pestaña no tiene tal fila, consulte [Solucionar problemas de inteligencia de código](#troubleshoot-code-intelligence).

    Después de instalar un binario faltante, Claude Code lo intenta de nuevo la próxima vez que Claude edita un archivo coincidente. Si instaló el binario en un directorio que no está en el `PATH` del shell desde el que inició `claude`, inicie una nueva sesión desde un shell donde esté.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Ver qué gana Claude
</h2>

Con un servidor de lenguaje ejecutándose, Claude gana diagnósticos y navegación de código:

* **Diagnósticos después de ediciones**: cada vez que Claude edita o escribe un archivo que el servidor maneja, Claude obtiene los errores y advertencias que el servidor reporta. Ve un error de tipo, importación faltante, o error de sintaxis que introdujo sin ejecutar un compilador.
* **Navegación de código**: Claude obtiene una herramienta `LSP` que busca símbolos a través del servidor en lugar de buscarlos por texto. La herramienta es de solo lectura. Para saber qué puede buscar Claude con la herramienta y cómo se aplican los permisos a ella, consulte [Comportamiento de la herramienta LSP](/docs/es/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Leer los diagnósticos usted mismo
</h3>

Después de que Claude edita un archivo que el servidor maneja, la conversación muestra solo el resumen `Found N new diagnostic issues`. Para leer los problemas en sí, presione **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Aceptar o descartar el diálogo de recomendación
</h2>

Si un binario de servidor de lenguaje ya está en su `PATH` y el plugin que lo usa no está instalado, Claude Code le ofrece instalar el plugin para usted en un diálogo titulado **LSP plugin recommendation**.

<h3 id="when-the-recommendation-dialog-appears">
  Cuándo aparece el diálogo de recomendación
</h3>

El diálogo **LSP plugin recommendation** puede aparecer después de que Claude edita un archivo. Estas condiciones deciden si aparece y qué plugin ofrece:

* **Un plugin coincide con el archivo**: uno de los marketplaces que ha agregado, o el marketplace oficial que Claude Code registró para usted, lista un plugin de inteligencia de código para la extensión de ese archivo, y el binario del plugin está instalado.
* **Oficial primero**: cuando más de un marketplace ofrece un plugin para la extensión, el diálogo ofrece el plugin del marketplace oficial.
* **Una vez por sesión**: el diálogo aparece como máximo una vez en una sesión, para el primer archivo coincidente que Claude edita.
* **No para sesiones en la nube**: el diálogo nunca aparece cuando su terminal está conectada a una sesión en la nube, como una que inició con [`claude --cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-cloud).

<h3 id="respond-to-the-recommendation-dialog">
  Responder al diálogo de recomendación
</h3>

El diálogo **LSP plugin recommendation** nombra el plugin y ofrece estas opciones:

* **Yes, install**: Claude Code instala el plugin para su cuenta de usuario e imprime `<plugin> installed · restart to apply`. Inicie una nueva sesión para cargar el servidor.
* **No, not now**: el diálogo se cierra, y una sesión posterior puede ofrecer el plugin nuevamente. Presionar **Esc** hace lo mismo.
* **Never for this plugin**: el diálogo deja de aparecer para ese plugin y sigue apareciendo para otros.
* **Disable all LSP recommendations**: el diálogo deja de aparecer para cada idioma.

Si no elige una opción, Claude Code la cierra después de 30 segundos y cuenta eso como ignorado. El conteo se mantiene entre sesiones. Después de cinco diálogos ignorados, Claude Code deja de recomendar plugins, lo mismo que si hubiera elegido **Disable all LSP recommendations**.

<h3 id="turn-recommendations-back-on">
  Activar recomendaciones nuevamente
</h3>

El diálogo **LSP plugin recommendation** deja de aparecer después de que elige **Disable all LSP recommendations** o lo ignora cinco veces.

* **Deshabilitado o ignorado cinco veces**: para activarlo nuevamente en cualquier caso, elimine las claves `lspRecommendationDisabled` y `lspRecommendationIgnoredCount` de `~/.claude.json`, el archivo de configuración propio de Claude Code.
* **Nunca para este plugin**: si eligió **Never for this plugin** y desea que ese plugin se ofrezca nuevamente, elimine su id `name@marketplace` de la lista `lspRecommendationNeverPlugins` en el mismo archivo.

<h2 id="troubleshoot-code-intelligence">
  Solucionar problemas de inteligencia de código
</h2>

La página de solución de problemas de plugins cubre los síntomas específicos de plugins de inteligencia de código bajo [Language server doesn't start, uses too much memory, or reports wrong diagnostics](/docs/es/plugins/troubleshooting#language-server-doesnt-start):

* **El servidor de lenguaje no se inicia**: ve `Executable not found in $PATH` en la pestaña **Errors** de `/plugin`, o Claude nunca reporta diagnósticos para el idioma.
* **Uso de memoria alto**: el uso de memoria aumenta mientras el servidor indexa el proyecto.
* **Diagnósticos falsos positivos en un monorepo**: los diagnósticos reportan importaciones como no resueltas cuando no lo son.

<h2 id="add-a-language-without-an-official-plugin">
  Agregar un idioma sin un plugin oficial
</h2>

Si su idioma no está en la [tabla de plugins oficiales](#install-a-code-intelligence-plugin), aún puede conectar un servidor de lenguaje.

1. Escriba un plugin con un archivo `.lsp.json` que nombre el comando del servidor y las extensiones de archivo que maneja.
2. Luego cargue el plugin con [`--plugin-dir`](/docs/es/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) o publíquelo en un marketplace.

Para los campos del archivo y un ejemplo trabajado, consulte [Servidores LSP en componentes de plugins](/docs/es/plugins/components#lsp-servers).

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Servidores LSP en componentes de plugins](/docs/es/plugins/components#lsp-servers): escriba el `.lsp.json` para un servidor de lenguaje que no tiene plugin oficial
* [Instalar y administrar plugins](/docs/es/plugins/install): alcances, actualizaciones y desinstalación
* [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting): cargar errores más allá de los específicos del servidor de lenguaje en esta página
* [Encontrar plugins en el marketplace oficial](/docs/es/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): dónde explorar el resto del marketplace oficial
