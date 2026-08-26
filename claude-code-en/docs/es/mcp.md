> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectar Claude Code a herramientas mediante MCP

> Aprenda cómo conectar Claude Code a sus herramientas con el Model Context Protocol.

Claude Code puede conectarse a cientos de herramientas externas y fuentes de datos a través del [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), un estándar de código abierto para integraciones de IA con herramientas. Los servidores MCP dan a Claude Code acceso a sus herramientas, bases de datos y APIs.

Conecte un servidor cuando se encuentre copiando datos en el chat desde otra herramienta, como un rastreador de problemas o un panel de monitoreo. Una vez conectado, Claude puede leer y actuar en ese sistema directamente en lugar de trabajar con lo que pegue.

Si está conectando su primer servidor, comience con el [inicio rápido de MCP](/docs/es/mcp-quickstart) para un recorrido paso a paso. Esta página es la referencia completa.

<h2 id="what-you-can-do-with-mcp">
  Qué puede hacer con MCP
</h2>

Con servidores MCP conectados, puede pedirle a Claude Code que:

* **Implemente características desde rastreadores de problemas**: "Agregue la característica descrita en el problema JIRA ENG-4521 y cree un PR en GitHub."
* **Analice datos de monitoreo**: "Verifique Sentry y Statsig para verificar el uso de la característica descrita en ENG-4521."
* **Consulte bases de datos**: "Encuentre correos electrónicos de 10 usuarios aleatorios que utilizaron la característica ENG-4521, basándose en nuestra base de datos PostgreSQL."
* **Integre diseños**: "Actualice nuestra plantilla de correo electrónico estándar basándose en los nuevos diseños de Figma que se publicaron en Slack"
* **Automatice flujos de trabajo**: "Cree borradores de Gmail invitando a estos 10 usuarios a una sesión de retroalimentación sobre la nueva característica."
* **Reaccione a eventos externos**: Un servidor MCP también puede actuar como un [canal](/docs/es/channels) que envía mensajes a su sesión, para que Claude reaccione a mensajes de Telegram, chats de Discord o eventos de webhook mientras está fuera.

<h2 id="find-and-build-mcp-servers">
  Buscar y crear servidores MCP
</h2>

Explore conectores revisados en el [Directorio de Anthropic](https://claude.ai/directory). Los conectores del Directorio utilizan la misma infraestructura MCP que Claude Code, por lo que puede agregar cualquier servidor remoto listado allí con `claude mcp add`.

<Warning>
  Verifique que confía en cada servidor antes de conectarlo. Los servidores que obtienen contenido externo pueden exponerlo al [riesgo de inyección de indicaciones](/docs/es/security#protect-against-prompt-injection).
</Warning>

Para crear su propio servidor, consulte la [guía del servidor MCP](https://modelcontextprotocol.io/docs/develop/build-server) para los fundamentos del protocolo y la [documentación de construcción de conectores de Claude](https://claude.com/docs/connectors/building) para autenticación, pruebas y envío al Directorio.

También puede hacer que Claude cree un servidor para usted con el plugin oficial [`mcp-server-dev`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev).

<Steps>
  <Step title="Instalar el plugin">
    En una sesión de Claude Code, ejecute:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Si Claude Code informa que el marketplace no se encuentra, ejecute `/plugin marketplace add anthropics/claude-plugins-official` primero, luego reintente la instalación. Una vez instalado, ejecute `/reload-plugins` para activarlo en la sesión actual.
  </Step>

  <Step title="Ejecutar la skill de construcción">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude le pregunta sobre su caso de uso y crea un servidor HTTP remoto o un servidor stdio local.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Instalación de servidores MCP
</h2>

Los servidores MCP se pueden configurar de varias formas según sus necesidades:

<h3 id="option-1-add-a-remote-http-server">
  Opción 1: Agregar un servidor HTTP remoto
</h3>

Los servidores HTTP son la opción recomendada para conectarse a servidores MCP remotos. Este es el transporte más ampliamente soportado para servicios basados en la nube.

```bash theme={null}
# Sintaxis básica
claude mcp add --transport http <name> <url>

# Ejemplo real: Conectar a Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Ejemplo con token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Cuando configure servidores MCP a través de JSON en `.mcp.json`, `~/.claude.json`, o `claude mcp add-json`, el campo `type` acepta `streamable-http` como un alias para `http`. La especificación de MCP utiliza el nombre `streamable-http` para este transporte, por lo que las configuraciones copiadas de la documentación del servidor funcionan sin modificación.

Una entrada JSON que tiene una `url` pero no tiene `type` es un error de configuración, porque Claude Code lee una entrada sin `type` como un servidor stdio. Claude Code omite ese servidor e informa `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Antes de v2.1.202, Claude Code informaba esta configuración incorrecta como `command: expected string, received undefined`.

<h3 id="option-2-add-a-remote-sse-server">
  Opción 2: Agregar un servidor SSE remoto
</h3>

<Warning>
  El transporte SSE (Server-Sent Events) está deprecado. Use servidores HTTP en su lugar, donde estén disponibles.
</Warning>

```bash theme={null}
# Sintaxis básica
claude mcp add --transport sse <name> <url>

# Ejemplo real: Conectar a Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Ejemplo con encabezado de autenticación
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Opción 3: Agregar un servidor stdio local
</h3>

Los servidores stdio se ejecutan como procesos locales en su máquina. Son ideales para herramientas que necesitan acceso directo al sistema o scripts personalizados.

Claude Code establece `CLAUDE_PROJECT_DIR` en el entorno del servidor generado a la raíz del proyecto, por lo que su servidor puede resolver rutas relativas al proyecto sin depender del directorio de trabajo. Este es el mismo directorio que los hooks reciben en su variable `CLAUDE_PROJECT_DIR`. Léalo desde dentro de su proceso de servidor, por ejemplo `process.env.CLAUDE_PROJECT_DIR` en Node o `os.environ["CLAUDE_PROJECT_DIR"]` en Python.

`CLAUDE_PROJECT_DIR` es la raíz del proyecto estable y no cambia cuando agrega o elimina directorios de trabajo durante la sesión. Un servidor que limita su propio acceso al sistema de archivos a un conjunto de directorios permitidos debe implementar la solicitud MCP `roots/list` en su lugar. Claude Code responde a `roots/list` con el directorio de lanzamiento de la sesión más cada [directorio de trabajo adicional](/docs/es/permissions#working-directories) que ha otorgado con `--add-dir`, `/add-dir`, o la configuración `additionalDirectories`. Claude Code envía `notifications/roots/list_changed` cuando ese conjunto cambia. Antes de v2.1.203, `roots/list` devolvía solo el directorio de lanzamiento y Claude Code no enviaba `notifications/roots/list_changed`.

Esta variable se establece en el entorno del servidor, no en el entorno propio de Claude Code, por lo que hacer referencia a ella mediante la expansión `${VAR}` en un archivo `.mcp.json` con alcance de proyecto o usuario en `command` o `args` requiere un valor predeterminado como `${CLAUDE_PROJECT_DIR:-.}`. Las configuraciones MCP proporcionadas por plugins sustituyen `${CLAUDE_PROJECT_DIR}` directamente y no necesitan el valor predeterminado.

```bash theme={null}
# Sintaxis básica
claude mcp add [options] <name> -- <command> [args...]

# Ejemplo real: Agregar servidor Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Importante: Separar argumentos del servidor con `--`**

  Para servidores stdio, el `--` (doble guión) separa las opciones propias de Claude, como `--transport`, `--env` y `--scope`, del comando y los argumentos que ejecutan el servidor. Todo lo que viene después de `--` se pasa al servidor sin modificar.

  Por ejemplo:

  * `claude mcp add --transport stdio myserver -- npx server` → ejecuta `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → ejecuta `python server.py --port 8080` con `KEY=value` en el entorno

  Sin `--`, Claude Code intentaría analizar las banderas del servidor, como `--port` arriba, como sus propias opciones.

  `--env` acepta múltiples pares `KEY=value`. Si el nombre del servidor viene directamente después de `--env`, la CLI lee el nombre como otro par y lo rechaza, por lo que coloque al menos otra opción entre `--env` y el nombre del servidor, como en los ejemplos anteriores.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opción 4: Agregar un servidor WebSocket remoto
</h3>

Los servidores WebSocket mantienen una conexión bidireccional persistente, que es adecuada para servidores MCP remotos que envían eventos a Claude sin ser solicitados. Use HTTP en su lugar cuando su servidor solo responda a solicitudes, ya que HTTP admite OAuth y la bandera `claude mcp add --transport`, mientras que WebSocket no admite ninguno de los dos.

Configure servidores WebSocket en `.mcp.json` o con `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

La entrada `type: "ws"` acepta los mismos campos `url`, `headers`, `headersHelper`, `timeout` y `alwaysLoad` que `http`. La autenticación es solo por encabezado, por lo que pase un token estático en `headers` o genere uno en el momento de la conexión con [`headersHelper`](#use-dynamic-headers-for-custom-authentication). La bandera `claude mcp add --transport` no acepta `ws`.

<h3 id="managing-your-servers">
  Gestión de sus servidores
</h3>

Una vez configurados, puede gestionar sus servidores MCP con estos comandos:

```bash theme={null}
# Listar todos los servidores configurados
claude mcp list

# Obtener detalles para un servidor específico
claude mcp get github

# Eliminar un servidor
claude mcp remove github

# (dentro de Claude Code) Verificar estado del servidor
/mcp
```

Los servidores con alcance de proyecto desde `.mcp.json` que están esperando su aprobación aparecen en `claude mcp list` como `⏸ Pending approval`. Ejecute `claude` de forma interactiva para revisar y aprobar. `claude mcp get <name>` muestra servidores pendientes como `⏸ Pending approval` y servidores rechazados como `✗ Rejected`.

A partir de v2.1.196, `claude mcp list` y `claude mcp get` leen aprobaciones de `.mcp.json` solo desde archivos de configuración que no están comprometidos en el repositorio hasta que confíe en el espacio de trabajo ejecutando `claude` en él y aceptando el diálogo de confianza del espacio de trabajo. Un repositorio clonado no puede aprobar sus propios servidores: [`enableAllProjectMcpServers` o `enabledMcpjsonServers`](/docs/es/settings#available-settings) comprometido en el archivo `.claude/settings.json` del proyecto se ignora en una carpeta no confiable, y el servidor permanece en `⏸ Pending approval` en lugar de estar conectado y verificado de salud.

Las aprobaciones de estas fuentes aún se aplican en una carpeta no confiable:

* su archivo `~/.claude/settings.json` del usuario
* configuración gestionada
* configuración pasada con `--settings`

Las aprobaciones en un archivo `.claude/settings.local.json` sin rastrear también se aplican, pero solo después de que acepte un diálogo de confianza para esa carpeta o uno de sus directorios principales: Claude Code ejecuta git para verificar si el archivo está rastreado, y ejecuta esa verificación solo en una carpeta confiable. En una carpeta que nunca ha confiado, las aprobaciones del archivo esperan el diálogo de confianza a menos que la carpeta sea su directorio de configuración personal: su directorio de inicio, o un directorio cuyo `.claude` ha establecido como [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars). Antes de v2.1.207, un archivo `.claude/settings.local.json` sin rastrear aprobaba servidores en una carpeta que nunca había confiado.

Una entrada `disabledMcpjsonServers` en cualquier archivo de configuración aún rechaza el servidor.

El panel `/mcp` muestra el recuento de herramientas junto a cada servidor conectado e indica los servidores que anuncian la capacidad de herramientas pero no exponen ninguna herramienta.

Un servidor remoto cuya configuración tiene una `url` vacía se muestra como `not configured` en `/mcp`, en `claude mcp list`, y en el [gestor `/plugin`](/docs/es/plugins), y Claude Code no intenta conectarse a él. Un plugin puede incluir una entrada de marcador de posición como esta para un conector que configura más tarde, por lo que Claude Code no lo informa como un error o un problema de configuración. La vista de detalles del servidor en `/mcp` lee `No URL configured for this server`; establezca la `url` de la entrada para conectarse. Antes de v2.1.208, Claude Code informaba una `url` vacía como un problema de configuración con un aviso para reconectar.

Si su solicitud necesita herramientas de un servidor que aún se está conectando en segundo plano, Claude espera a que ese servidor continúe. Con [búsqueda de herramientas](#scale-with-mcp-tool-search) habilitada, que es la predeterminada, la espera ocurre dentro de la llamada `ToolSearch`. En configuraciones sin búsqueda de herramientas, como Google Cloud's Agent Platform, un `ANTHROPIC_BASE_URL` personalizado, o `ENABLE_TOOL_SEARCH=false`, Claude utiliza la herramienta `WaitForMcpServers` en su lugar.

Algunos nombres de servidor están reservados para los servidores integrados de Claude Code: `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, y `Claude Browser`. Si su configuración define un servidor con un nombre reservado, Claude Code lo omite al cargar y muestra una advertencia pidiéndole que lo renombre. `claude mcp add` rechaza un nombre reservado con un error.

`Claude Preview` y `Claude Browser` ambos nombran el servidor integrado que el [panel de vista previa de la aplicación de escritorio de Claude Code](/docs/es/desktop#preview-your-app) utiliza. Antes de v2.1.205, `Claude Browser` no estaba reservado, por lo que un servidor configurado por el usuario podría registrarse bajo ese nombre.

<h3 id="dynamic-tool-updates">
  Actualizaciones dinámicas de herramientas
</h3>

Claude Code admite notificaciones `list_changed` de MCP, permitiendo que los servidores MCP actualicen dinámicamente sus herramientas disponibles, indicaciones y recursos sin requerir que se desconecte y reconecte. Cuando un servidor MCP envía una notificación `list_changed`, Claude Code actualiza automáticamente las capacidades disponibles de ese servidor.

<h3 id="automatic-reconnection">
  Reconexión automática
</h3>

Si un servidor HTTP o SSE se desconecta durante la sesión, Claude Code se reconecta automáticamente con retroceso exponencial: hasta cinco intentos, comenzando con un retraso de un segundo y duplicándose cada vez. El servidor aparece como pendiente en `/mcp` mientras la reconexión está en progreso. Después de cinco intentos fallidos, el servidor se marca como fallido y puede reintentar manualmente desde `/mcp`. Los servidores stdio son procesos locales y no se reconectan automáticamente.

El mismo retroceso se aplica cuando un servidor HTTP o SSE falla su conexión inicial al iniciar. A partir de v2.1.121, Claude Code reintenta la conexión inicial hasta tres veces en errores transitorios como una respuesta 5xx, una conexión rechazada o un tiempo de espera agotado, luego marca el servidor como fallido si aún no puede conectarse. Los errores de autenticación y no encontrado no se reintentan porque requieren un cambio de configuración para resolverse.

Cuando un servidor configurado falla al conectarse, Claude Code le dice a Claude qué servidor falló y su error de conexión, incluyendo en resultados de `ToolSearch` que no encuentran ninguna herramienta coincidente, por lo que Claude informa el fallo de conexión en su respuesta. Requiere [búsqueda de herramientas](#scale-with-mcp-tool-search), que está habilitada de forma predeterminada. En configuraciones sin búsqueda de herramientas, como un `ANTHROPIC_BASE_URL` personalizado, `ENABLE_TOOL_SEARCH=false`, o un modelo que no admite búsqueda de herramientas, y en Amazon Bedrock, Google Cloud's Agent Platform, y Microsoft Foundry, Claude Code no informa fallos de conexión de servidores a Claude. Antes de v2.1.205, Claude Code no pasaba errores de conexión a Claude, y Claude podría responder como si las herramientas del servidor fallido nunca estuvieran configuradas.

A partir de v2.1.191, las solicitudes de descubrimiento de capacidades que se ejecutan después de una conexión exitosa, como `tools/list`, `prompts/list` y `resources/list`, también reintentan errores de red transitorios y del servidor hasta tres veces con retroceso corto. Los errores de autenticación, respuestas 4xx y tiempos de espera de solicitud no se reintentan.

<h3 id="push-messages-with-channels">
  Mensajes push con canales
</h3>

Un servidor MCP también puede enviar mensajes directamente a su sesión para que Claude pueda reaccionar a eventos externos como resultados de CI, alertas de monitoreo o mensajes de chat. Para habilitar esto, su servidor declara la capacidad `claude/channel` y usted la activa con la bandera `--channels` al iniciar. Vea [Canales](/docs/es/channels) para usar un canal oficialmente soportado, o [Referencia de canales](/docs/es/channels-reference) para construir el suyo propio.

<Tip>
  Consejos:

  * Use la bandera `-s` o `--scope` para especificar dónde se almacena la configuración:
    * `local` (predeterminado): Disponible solo para usted en el proyecto actual. Las versiones anteriores llamaban a este alcance `project`
    * `project`: Compartido con todos en el proyecto a través del archivo `.mcp.json`
    * `user`: Disponible para usted en todos los proyectos. Las versiones anteriores llamaban a este alcance `global`
  * Establezca variables de entorno con banderas `-e` o `--env` (por ejemplo, `-e KEY=value`)
  * Las banderas `--transport` y `--header` también aceptan formas cortas `-t` y `-H`
  * Configure el tiempo de espera de inicio del servidor MCP usando la variable de entorno `MCP_TIMEOUT` (por ejemplo, `MCP_TIMEOUT=10000 claude` establece un tiempo de espera de 10 segundos)
  * Establezca un tiempo de espera de ejecución de herramientas por servidor agregando un campo `timeout` en milisegundos a la entrada `.mcp.json` de ese servidor, por ejemplo `"timeout": 600000` para diez minutos. Esto anula la variable de entorno `MCP_TOOL_TIMEOUT` solo para ese servidor
  * Claude Code muestra una advertencia cuando la salida de la herramienta MCP excede 10,000 tokens y limita la salida a 25,000 tokens de forma predeterminada. Para aumentar el límite, establezca la variable de entorno `MAX_MCP_OUTPUT_TOKENS` (por ejemplo, `MAX_MCP_OUTPUT_TOKENS=50000`); el umbral de advertencia es fijo. Vea [Límites de salida de MCP y advertencias](#mcp-output-limits-and-warnings)
  * Use `/mcp` para autenticarse con servidores remotos que requieren autenticación OAuth 2.0
</Tip>

El `timeout` por servidor es un límite de reloj de pared duro por llamada de herramienta, y las notificaciones de progreso del servidor no lo extienden. Los valores por debajo de 1000 se ignoran y caen a `MCP_TOOL_TIMEOUT`, o a su predeterminado de aproximadamente 28 horas cuando esa variable no está establecida. Para un servidor HTTP, SSE, o [conector de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) también hay un segundo temporizador por solicitud que cubre cada solicitud hasta el primer byte de respuesta del servidor. Ese temporizador es de 60 segundos a menos que establezca el `timeout` por servidor o `MCP_TOOL_TIMEOUT`; establecer cualquiera de ellos a 60 segundos o superior eleva el temporizador por solicitud a ese valor, un valor inferior no lo acorta, y el predeterminado de 28 horas de un `MCP_TOOL_TIMEOUT` no establecido nunca lo alimenta. Los servidores stdio y WebSocket no tienen temporizador por solicitud. Antes de v2.1.162, los valores por debajo de 1000 se redondeaban hacia abajo a un segundo en su lugar.

Un `timeout` por servidor de al menos 1000 también actúa como un piso en el tiempo de espera de inactividad descrito a continuación: Claude Code nunca cancela las llamadas de herramientas de ese servidor por inactividad antes que el `timeout` por servidor. Requiere Claude Code v2.1.203 o posterior.

Una llamada de herramienta a un servidor MCP que no envía respuesta ni notificación de progreso durante la ventana de inactividad se cancela con un error en lugar de esperar el límite de reloj de pared. El tiempo de espera de inactividad requiere Claude Code v2.1.187 o posterior. Se aplica a todos los tipos de servidor excepto servidores IDE y servidores en proceso SDK. La ventana de inactividad predeterminada es de cinco minutos para servidores HTTP, SSE, WebSocket y [conector de claude.ai](#use-mcp-servers-from-claude-ai), y de 30 minutos para servidores stdio. Antes de v2.1.203, los servidores stdio estaban exentos del tiempo de espera de inactividad.

Establezca la variable de entorno [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/es/env-vars) en milisegundos para cambiar la ventana de inactividad, o establézcala en `0` para desactivar la verificación.

<h3 id="plugin-provided-mcp-servers">
  Servidores MCP proporcionados por plugins
</h3>

Los [plugins](/docs/es/plugins) pueden agrupar servidores MCP, proporcionando automáticamente herramientas e integraciones cuando el plugin está habilitado. Los servidores MCP de plugins funcionan de manera idéntica a los servidores configurados por el usuario.

**Cómo funcionan los servidores MCP de plugins**:

* Los plugins definen servidores MCP en `.mcp.json` en la raíz del plugin o en línea en `plugin.json`
* Cuando un plugin está habilitado, sus servidores MCP se inician automáticamente
* Las herramientas MCP del plugin aparecen junto a las herramientas MCP configuradas manualmente
* Los servidores de plugins se gestionan a través de la instalación de plugins, no mediante comandos `/mcp`

**Ejemplo de configuración MCP de plugin**:

En `.mcp.json` en la raíz del plugin:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

O en línea en `plugin.json`:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Características de MCP de plugins**:

* **Ciclo de vida automático**: Al iniciar la sesión, los servidores de los plugins habilitados se conectan automáticamente. Si habilita o deshabilita un plugin durante una sesión, ejecute `/reload-plugins` para conectar o desconectar sus servidores MCP
* **Variables de entorno**: Use `${CLAUDE_PLUGIN_ROOT}` para archivos agrupados en el plugin, `${CLAUDE_PLUGIN_DATA}` para [estado persistente](/docs/es/plugins-reference#persistent-data-directory) que sobrevive a las actualizaciones de plugins, y `${CLAUDE_PROJECT_DIR}` para la raíz del proyecto estable. La sustitución se aplica a:
  * servidores `stdio`: `command`, `args`, `env`
  * servidores `http`, `sse` y `ws`: `url`, `headers` y `headersHelper`. Antes de v2.1.195, `headersHelper` pasaba el marcador de posición como una cadena literal
* **Acceso a variables de entorno del usuario**: Acceso a las mismas variables de entorno que los servidores configurados manualmente
* **Múltiples tipos de transporte**: Soporte para transportes stdio, SSE, HTTP y WebSocket, aunque el soporte de transporte puede variar según el servidor

**Visualización de servidores MCP de plugins**:

```bash theme={null}
# Dentro de Claude Code, vea todos los servidores MCP incluyendo los de plugins
/mcp
```

Los servidores de plugins aparecen en la lista con indicadores que muestran que provienen de plugins.

**Nombres de herramientas MCP de plugins**:

Las herramientas de un servidor MCP agrupado en un plugin incluyen tanto el nombre del plugin como la clave del servidor en su nombre invocable. La forma completa es `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, donde cualquier carácter fuera de `A-Z`, `a-z`, `0-9`, `_` y `-` se reemplaza con `_`. Para el servidor `database-tools` agrupado en un plugin llamado `my-plugin`, una herramienta `query` es invocable como:

```
mcp__plugin_my-plugin_database-tools__query
```

Use este nombre completo cuando haga referencia a la herramienta en [reglas de permisos](/docs/es/permissions), la lista `allowed-tools` de una skill, un [campo `tools` de un subagente](/docs/es/sub-agents#available-tools), o un [matcher de hook](/docs/es/hooks#match-mcp-tools). Un matcher de hook escrito contra la clave del servidor simple, como `mcp__database-tools__.*`, nunca se activa para un servidor agrupado en un plugin.

El servidor mismo se registra bajo el nombre con alcance `plugin:<plugin-name>:<server-name>`, como `plugin:my-plugin:database-tools`. Use ese nombre donde se espera un nombre de servidor configurado, como un [campo `server` del hook `mcp_tool`](/docs/es/hooks#mcp-tool-hook-fields).

**Beneficios de los servidores MCP de plugins**:

* **Distribución agrupada**: Herramientas y servidores empaquetados juntos
* **Configuración automática**: No se necesita configuración manual de MCP
* **Consistencia del equipo**: Todos obtienen las mismas herramientas cuando se instala el plugin

Vea la [referencia de componentes de plugins](/docs/es/plugins-reference#mcp-servers) para detalles sobre cómo agrupar servidores MCP con plugins.

<h2 id="mcp-installation-scopes">
  Alcances de instalación de MCP
</h2>

Los servidores MCP se pueden configurar en tres alcances. El alcance que elija controla en qué proyectos se carga el servidor y si la configuración se comparte con su equipo. Los administradores también pueden implementar servidores a nivel empresarial a través de [configuración administrada](#managed-mcp-configuration).

| Alcance                    | Se carga en          | Compartido con equipo                 | Almacenado en                       |
| -------------------------- | -------------------- | ------------------------------------- | ----------------------------------- |
| [Local](#local-scope)      | Solo proyecto actual | No                                    | `~/.claude.json`                    |
| [Proyecto](#project-scope) | Solo proyecto actual | Sí, a través del control de versiones | `.mcp.json` en la raíz del proyecto |
| [Usuario](#user-scope)     | Todos sus proyectos  | No                                    | `~/.claude.json`                    |

<h3 id="local-scope">
  Alcance local
</h3>

El alcance local es el predeterminado. Un servidor con alcance local se carga solo en el proyecto donde lo agregó y permanece privado para usted. Claude Code lo almacena en `~/.claude.json` bajo la ruta de ese proyecto, por lo que el mismo servidor no aparecerá en sus otros proyectos. Use el alcance local para servidores de desarrollo personal, configuraciones experimentales o servidores con credenciales que no desea en el control de versiones.

<Note>
  El término "alcance local" para servidores MCP difiere de la configuración local general. Los servidores MCP con alcance local se almacenan en `~/.claude.json` (su directorio de inicio), mientras que la configuración local general usa `.claude/settings.local.json` (en el directorio del proyecto). Vea [Configuración](/docs/es/settings#settings-files) para detalles sobre ubicaciones de archivos de configuración.
</Note>

```bash theme={null}
# Agregar un servidor con alcance local (predeterminado)
claude mcp add --transport http stripe https://mcp.stripe.com

# Especificar explícitamente alcance local
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

El comando escribe el servidor en la entrada de su proyecto actual dentro de `~/.claude.json`. El ejemplo a continuación muestra el resultado cuando lo ejecuta desde `/path/to/your/project`:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Alcance de proyecto
</h3>

Los servidores con alcance de proyecto habilitan la colaboración en equipo al almacenar configuraciones en un archivo `.mcp.json` en el directorio raíz de su proyecto. Este archivo está diseñado para ser verificado en el control de versiones, asegurando que todos los miembros del equipo tengan acceso a las mismas herramientas y servicios MCP. Cuando agrega un servidor con alcance de proyecto, Claude Code crea o actualiza automáticamente este archivo con la estructura de configuración apropiada.

```bash theme={null}
# Agregar un servidor con alcance de proyecto
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

El archivo `.mcp.json` resultante sigue un formato estandarizado:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

Por razones de seguridad, Claude Code solicita aprobación antes de usar servidores con alcance de proyecto desde archivos `.mcp.json`. Si necesita restablecer estas opciones de aprobación, use el comando `claude mcp reset-project-choices`.

<h3 id="user-scope">
  Alcance de usuario
</h3>

Los servidores con alcance de usuario se almacenan en `~/.claude.json` y proporcionan accesibilidad entre proyectos, haciéndolos disponibles en todos los proyectos en su máquina mientras permanecen privados para su cuenta de usuario. Este alcance funciona bien para servidores de utilidad personal, herramientas de desarrollo o servicios que usa frecuentemente en diferentes proyectos.

```bash theme={null}
# Agregar un servidor de usuario
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Jerarquía de alcance y precedencia
</h3>

Cuando el mismo servidor está definido en más de un lugar, Claude Code se conecta a él una sola vez, usando la definición de la fuente de mayor precedencia. La entrada completa del servidor de esa fuente se utiliza; los campos no se fusionan entre alcances.

1. Alcance local
2. Alcance de proyecto
3. Alcance de usuario
4. [Servidores proporcionados por plugins](/docs/es/plugins)
5. [Conectores de claude.ai](#use-mcp-servers-from-claude-ai)

Los tres alcances coinciden duplicados por nombre. Los plugins y conectores coinciden por punto final, por lo que uno que apunta a la misma URL o comando que un servidor anterior se trata como un duplicado.

<h3 id="environment-variable-expansion-in-mcp-json">
  Expansión de variables de entorno en `.mcp.json`
</h3>

Claude Code admite la expansión de variables de entorno en archivos `.mcp.json`, permitiendo que los equipos compartan configuraciones mientras mantienen flexibilidad para rutas específicas de máquinas y valores sensibles como claves API.

**Sintaxis soportada:**

* `${VAR}`: se expande al valor de la variable de entorno `VAR`
* `${VAR:-default}`: se expande a `VAR` si está establecida, de lo contrario usa `default`

**Ubicaciones de expansión:**
Las variables de entorno se pueden expandir en:

* `command`: la ruta del ejecutable del servidor
* `args`: argumentos de línea de comandos
* `env`: variables de entorno pasadas al servidor
* `url`: para tipos de servidor HTTP
* `headers`: para autenticación de servidor HTTP

**Ejemplo con expansión de variables:**

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

Si una variable de entorno requerida no está establecida y no tiene un valor predeterminado, Claude Code deja el texto literal `${VAR}` en el valor e informa una advertencia de variable faltante para ese servidor. La configuración aún se carga, por lo que establezca la variable o agregue un fallback `:-default` para que el servidor se inicie con el valor que pretende.

<h2 id="practical-examples">
  Ejemplos prácticos
</h2>

<h3 id="example-monitor-errors-with-sentry">
  Ejemplo: Monitorear errores con Sentry
</h3>

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

Autentíquese con su cuenta de Sentry:

```text theme={null}
/mcp
```

Luego depure problemas de producción:

```text theme={null}
¿Cuáles son los errores más comunes en las últimas 24 horas?
```

```text theme={null}
Muéstrame el seguimiento de pila para el error ID abc123
```

```text theme={null}
¿Qué despliegue introdujo estos nuevos errores?
```

<h3 id="example-connect-to-github-for-code-reviews">
  Ejemplo: Conectar a GitHub para revisiones de código
</h3>

El servidor MCP remoto de GitHub se autentica con un token de acceso personal de GitHub pasado como encabezado. Para obtener uno, abra su [configuración de token de GitHub](https://github.com/settings/personal-access-tokens), genere un nuevo token de grano fino con acceso a los repositorios con los que desea que Claude trabaje, luego agregue el servidor:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Luego trabaje con GitHub:

```text theme={null}
Revise el PR #456 y sugiera mejoras
```

```text theme={null}
Cree un nuevo problema para el error que acabamos de encontrar
```

```text theme={null}
Muéstrame todos los PR abiertos asignados a mí
```

<h3 id="example-query-your-postgresql-database">
  Ejemplo: Consultar su base de datos PostgreSQL
</h3>

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Luego consulte su base de datos de forma natural:

```text theme={null}
¿Cuál es nuestro ingreso total este mes?
```

```text theme={null}
Muéstrame el esquema para la tabla de pedidos
```

```text theme={null}
Encuentre clientes que no han realizado una compra en 90 días
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autenticarse con servidores MCP remotos
</h2>

Muchos servidores MCP basados en la nube requieren autenticación. Claude Code admite OAuth 2.0 para conexiones seguras.

Claude Code marca un servidor remoto como que requiere autenticación cuando el servidor responde con `401 Unauthorized` o `403 Forbidden`. Para un servidor en el que no ha iniciado sesión, cualquiera de estos códigos de estado lo marca en `/mcp` para que pueda completar el flujo de OAuth.

Cuando una solicitud a un servidor OAuth en el que ya ha iniciado sesión devuelve `401 Unauthorized`, Claude Code actualiza el token almacenado, se reconecta e intenta la solicitud una vez más. Solo marca el servidor en `/mcp` si ese reintento también falla. Antes de v2.1.206, una actualización de token que falló por una razón transitoria, como un error de red, marcaba un servidor OAuth como que necesitaba autenticación durante el resto de la sesión aunque su token de actualización seguía siendo válido.

A partir de v2.1.195, cuando una actualización de token falla porque el servidor rechaza el token de actualización almacenado, Claude Code muestra inmediatamente un aviso que apunta a `/mcp`. El menú del servidor conectado allí ofrece Re-authenticate, para que pueda iniciar sesión nuevamente antes de que la siguiente llamada de herramienta falle.

Un servidor personalizado que devuelve un encabezado `WWW-Authenticate` que apunta a su servidor de autorización obtiene el mismo descubrimiento automático que cualquier otro servidor remoto.

A partir de v2.1.193, Claude Code también muestra un aviso de inicio cuando uno o más servidores configurados necesitan autenticación, por lo que no tiene que abrir `/mcp` para descubrir qué servidores necesitan iniciar sesión.

En modo no interactivo no hay panel `/mcp`, por lo que Claude Code no puede ejecutar el flujo de OAuth para usted. A partir de v2.1.196, cuando un servidor configurado necesita autenticación durante una ejecución de `claude -p` o Agent SDK con [búsqueda de herramientas](#scale-with-mcp-tool-search) habilitada, que es la predeterminada, Claude Code le dice a Claude que las herramientas del servidor no están disponibles hasta que lo autorice. Claude puede entonces nombrar el servidor que necesita iniciar sesión en lugar de responder como si el servidor no estuviera configurado. Complete el inicio de sesión desde una sesión interactiva con `/mcp` o `claude mcp login <name>`.

Si configuró `headers.Authorization` para el servidor y el servidor rechaza ese encabezado, Claude Code reporta la conexión como fallida en lugar de recurrir a OAuth. Verifique que el token sea válido para el punto final de MCP, o elimine el encabezado para usar el flujo de OAuth.

<Steps>
  <Step title="Agregar el servidor que requiere autenticación">
    Por ejemplo:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Use el comando /mcp dentro de Claude Code">
    En Claude Code, use el comando:

    ```text theme={null}
    /mcp
    ```

    Luego siga los pasos en su navegador para iniciar sesión.
  </Step>
</Steps>

<Tip>
  Consejos:

  * Los tokens de autenticación se almacenan de forma segura y se actualizan automáticamente
  * Use "Clear authentication" en el menú `/mcp` para revocar el acceso
  * Si su navegador no se abre automáticamente, copie la URL proporcionada y ábrala manualmente
  * Si el redireccionamiento del navegador falla con un error de conexión después de autenticarse, pegue la URL de devolución de llamada completa de la barra de direcciones de su navegador en el indicador de URL que aparece en Claude Code
  * La autenticación OAuth funciona con servidores HTTP
</Tip>

<h3 id="authenticate-from-the-command-line">
  Autenticarse desde la línea de comandos
</h3>

A partir de v2.1.186, `claude mcp login <name>` ejecuta el flujo de OAuth de un servidor configurado directamente desde su shell, por lo que no necesita abrir el panel `/mcp` dentro de una sesión.

```bash theme={null}
claude mcp login sentry
```

Para borrar las credenciales almacenadas más tarde, ejecute `claude mcp logout <name>`.

A partir de v2.1.191, el comando detecta cuando no hay navegador local disponible, como durante una sesión SSH o en Linux sin un servidor de pantalla, e imprime la URL de autorización en lugar de intentar abrir un navegador. Abra la URL en su máquina local, luego pegue la URL de redireccionamiento completa de la barra de direcciones de su navegador nuevamente en el indicador. El comando necesita una terminal interactiva para el paso de pegado, así que conéctese con `ssh -t`. Pase `--no-browser` para forzar el indicador de URL incluso cuando se detecta un navegador local.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Usar un puerto de devolución de llamada OAuth fijo
</h3>

Algunos servidores MCP requieren un URI de redireccionamiento específico registrado de antemano. De forma predeterminada, Claude Code elige un puerto disponible aleatorio para la devolución de llamada de OAuth. Use `--callback-port` para fijar el puerto de modo que coincida con un URI de redireccionamiento preregistrado de la forma `http://localhost:PORT/callback`.

Puede usar `--callback-port` por sí solo (con registro dinámico de clientes) o junto con `--client-id` (con credenciales preconfiguradas).

```bash theme={null}
# Puerto de devolución de llamada fijo con registro dinámico de clientes
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Usar credenciales OAuth preconfiguradas
</h3>

Algunos servidores MCP no admiten configuración automática de OAuth mediante Registro Dinámico de Clientes. Si ve un error como "Incompatible auth server: does not support dynamic client registration", el servidor requiere credenciales preconfiguradas. Claude Code también admite servidores que usan un Documento de Metadatos de ID de Cliente (CIMD) en lugar de Registro Dinámico de Clientes, y los descubre automáticamente. Si el descubrimiento automático falla, registre una aplicación OAuth a través del portal de desarrolladores del servidor primero, luego proporcione las credenciales al agregar el servidor.

<Steps>
  <Step title="Registrar una aplicación OAuth con el servidor">
    Cree una aplicación a través del portal de desarrolladores del servidor y anote su ID de cliente y secreto de cliente.

    Muchos servidores también requieren un URI de redireccionamiento. Si es así, elija un puerto y registre un URI de redireccionamiento en el formato `http://localhost:PORT/callback`. Use ese mismo puerto con `--callback-port` en el siguiente paso.
  </Step>

  <Step title="Agregar el servidor con sus credenciales">
    Elija uno de los siguientes métodos. El puerto utilizado para `--callback-port` puede ser cualquier puerto disponible. Solo necesita coincidir con el URI de redireccionamiento que registró en el paso anterior.

    <Tabs>
      <Tab title="claude mcp add">
        Use `--client-id` para pasar el ID de cliente de su aplicación. La bandera `--client-secret` solicita el secreto con entrada enmascarada:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Incluya el objeto `oauth` en la configuración JSON y pase `--client-secret` como una bandera separada:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (solo puerto de devolución de llamada)">
        Use `--callback-port` sin un ID de cliente para fijar el puerto mientras usa registro dinámico de clientes:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / variable de entorno">
        Establezca el secreto a través de una variable de entorno para omitir el indicador interactivo:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Autenticarse en Claude Code">
    Ejecute `/mcp` en Claude Code y siga el flujo de inicio de sesión del navegador.
  </Step>
</Steps>

<Tip>
  Consejos:

  * El secreto del cliente se almacena de forma segura en su llavero del sistema (macOS) o un archivo de credenciales, no en su configuración
  * Si el servidor usa un cliente OAuth público sin secreto, use solo `--client-id` sin `--client-secret`
  * `--callback-port` se puede usar con o sin `--client-id`
  * Estas banderas solo se aplican a transportes HTTP y SSE. No tienen efecto en servidores stdio
  * Use `claude mcp get <name>` para verificar que las credenciales OAuth estén configuradas para un servidor
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Anular el descubrimiento de metadatos de OAuth
</h3>

Apunte Claude Code a una URL de metadatos de servidor de autorización OAuth específica para omitir la cadena de descubrimiento predeterminada. Establezca `authServerMetadataUrl` cuando los puntos finales estándar del servidor MCP generen errores, o cuando desee enrutar el descubrimiento a través de un proxy interno. De forma predeterminada, Claude Code primero verifica los Metadatos de Recursos Protegidos RFC 9728 en `/.well-known/oauth-protected-resource`, luego recurre a los metadatos del servidor de autorización RFC 8414 en `/.well-known/oauth-authorization-server`.

Establezca `authServerMetadataUrl` en el objeto `oauth` de la configuración de su servidor en `.mcp.json`:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

La URL debe usar `https://`. Los `scopes_supported` de la URL de metadatos anulan los alcances que el servidor ascendente anuncia.

<h3 id="restrict-oauth-scopes">
  Restringir alcances de OAuth
</h3>

Establezca `oauth.scopes` para fijar los alcances que Claude Code solicita durante el flujo de autorización. Esta es la forma soportada de restringir un servidor MCP a un subconjunto aprobado por el equipo de seguridad cuando el servidor de autorización ascendente anuncia más alcances de los que desea otorgar. El valor es una cadena única separada por espacios, que coincide con el formato del parámetro `scope` en RFC 6749 §3.3.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` tiene precedencia sobre tanto `authServerMetadataUrl` como los alcances que el servidor descubre en `/.well-known`. Déjelo sin establecer para permitir que el servidor MCP determine el conjunto de alcances solicitados.

A partir de v2.1.196, cuando `oauth.scopes` no está establecido, Claude Code solicita el alcance proporcionado por el encabezado `WWW-Authenticate` del servidor o sus metadatos de recursos protegidos, y no envía ningún parámetro `scope` cuando ninguno proporciona uno. Ya no solicita el catálogo completo de `scopes_supported` de los metadatos del servidor de autorización descubiertos automáticamente. Solicitar ese catálogo hizo que los proveedores de identidad que anuncian alcances solo para administrador o de plantilla rechazaran la solicitud de autorización con un error `invalid_scope`. Los metadatos obtenidos de un `authServerMetadataUrl` configurado aún proporcionan su `scopes_supported` como los alcances solicitados.

Si el servidor de autorización anuncia `offline_access` en `scopes_supported`, Claude Code lo añade a los alcances fijados para que el token de acceso pueda actualizarse sin un nuevo inicio de sesión en el navegador.

Si el servidor luego devuelve un 403 `insufficient_scope` para una llamada de herramienta, Claude Code se reautentica con los mismos alcances fijados. Amplíe `oauth.scopes` cuando una herramienta que necesita requiera un alcance fuera del fijo.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Usar encabezados dinámicos para autenticación personalizada
</h3>

Si su servidor MCP usa un esquema de autenticación diferente a OAuth, como Kerberos, tokens de corta duración o un SSO interno, use `headersHelper` para generar encabezados de solicitud en el momento de la conexión. Claude Code ejecuta el comando y fusiona su salida en los encabezados de conexión.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

El comando también puede ser en línea:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Requisitos:**

* El comando debe escribir un objeto JSON de pares clave-valor de cadena en stdout
* El comando se ejecuta en un shell con un tiempo de espera de 10 segundos, desde el directorio de trabajo actual de la sesión. Use una ruta absoluta o un comando en `PATH` para el script
* Los encabezados dinámicos anulan cualquier `headers` estático con el mismo nombre

El ayudante se ejecuta nuevamente en cada conexión, al iniciar la sesión y al reconectar. No hay almacenamiento en caché, por lo que su script es responsable de cualquier reutilización de tokens.

A partir de v2.1.193, si una llamada de herramienta devuelve `401 Unauthorized` o `403 Forbidden`, Claude Code automáticamente vuelve a ejecutar el ayudante, se reconecta con los encabezados frescos, e intenta la llamada una vez más. Claude Code marca el servidor como que necesita autenticación en `/mcp` solo si ese reintento también falla.

Claude Code establece estas variables de entorno al ejecutar el ayudante:

| Variable                      | Valor                                                                                                                          |
| :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | el nombre del servidor MCP                                                                                                     |
| `CLAUDE_CODE_MCP_SERVER_URL`  | la URL del servidor MCP                                                                                                        |
| `CLAUDE_PLUGIN_ROOT`          | el directorio raíz del plugin. Se establece solo cuando un [plugin](/docs/es/plugins-reference#mcp-servers) proporciona el servidor |

Use estas para escribir un único script de ayudante que sirva múltiples servidores MCP.

Para un servidor proporcionado por un plugin, el ayudante también se ejecuta con su directorio de trabajo establecido en la raíz del plugin, por lo que una ruta `headersHelper` relativa se resuelve dentro del directorio del plugin en lugar de contra el directorio de trabajo de la sesión. Requiere Claude Code v2.1.195 o posterior.

Un `headersHelper` proporcionado por un plugin no puede hacer referencia a los valores [`${user_config.*}`](/docs/es/plugins-reference#user-configuration) del plugin, porque el comando se ejecuta a través de un shell. Claude Code reporta el servidor como mal configurado con un [error](/docs/es/errors#plugin-command-references-user-config) y no sustituye el valor. Ponga `${user_config.KEY}` en el campo `headers` del servidor en su lugar, que no se analiza como shell, o haga que el script de ayudante lea el valor de su propio entorno o un archivo de configuración. Antes de v2.1.207, `headersHelper` sustituía valores `${user_config.*}`.

<Note>
  `headersHelper` ejecuta comandos de shell arbitrarios. Cuando se define en alcance de proyecto o local, solo se ejecuta después de que acepte el diálogo de confianza del espacio de trabajo.
</Note>

<h2 id="add-mcp-servers-from-json-configuration">
  Agregar servidores MCP desde configuración JSON
</h2>

Si tiene una configuración JSON para un servidor MCP, puede agregarla directamente:

<Steps>
  <Step title="Agregar un servidor MCP desde JSON">
    ```bash theme={null}
    # Sintaxis básica
    claude mcp add-json <name> '<json>'

    # Ejemplo: Agregar un servidor HTTP con configuración JSON
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Ejemplo: Agregar un servidor stdio con configuración JSON
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Ejemplo: Agregar un servidor HTTP con credenciales OAuth preconfiguradas
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Verificar que el servidor fue agregado">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Asegúrese de que el JSON esté correctamente escapado en su shell
  * El JSON debe cumplir con el esquema de configuración del servidor MCP
  * Puede usar `--scope user` para agregar el servidor a su configuración de usuario en lugar de la específica del proyecto
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Importar servidores MCP desde Claude Desktop
</h2>

Si ya ha configurado servidores MCP en Claude Desktop, puede importarlos:

<Steps>
  <Step title="Importar servidores desde Claude Desktop">
    ```bash theme={null}
    # Sintaxis básica 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Seleccionar qué servidores importar">
    Después de ejecutar el comando, verá un diálogo interactivo que le permite seleccionar qué servidores desea importar.
  </Step>

  <Step title="Verificar que los servidores fueron importados">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

Los nombres de servidores agregados a través de comandos `claude mcp` pueden contener solo letras, números, guiones y guiones bajos. Claude Desktop no aplica esa restricción, por lo que un servidor de Claude Desktop cuyo nombre contiene cualquier otro carácter, como un espacio, no puede ser importado. La importación reporta cada nombre que rechaza e importa los otros servidores que seleccionó. Antes de v2.1.205, el primer nombre inválido detenía la importación y ninguno de los servidores seleccionados se agregaba.

<Tip>
  Consejos:

  * Esta característica solo funciona en macOS y Windows Subsystem for Linux (WSL)
  * Lee el archivo de configuración de Claude Desktop desde su ubicación estándar en esas plataformas
  * Use la bandera `--scope user` para agregar servidores a su configuración de usuario
  * Los servidores importados mantienen los mismos nombres que en Claude Desktop cuando el nombre contiene solo letras, números, guiones y guiones bajos. Claude Code reporta un servidor cuyo nombre contiene cualquier otro carácter y lo omite
  * Si ya existen servidores con los mismos nombres, obtendrán un sufijo numérico (por ejemplo, `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Usar servidores MCP desde claude.ai
</h2>

Si ha iniciado sesión en Claude Code con una cuenta de [claude.ai](https://claude.ai), los servidores MCP que ha agregado en claude.ai, conocidos como [conectores](https://claude.com/docs/connectors), están automáticamente disponibles en Claude Code:

<Steps>
  <Step title="Configurar servidores MCP en claude.ai">
    Agregue servidores en [claude.ai/customize/connectors](https://claude.ai/customize/connectors). En planes de Equipo y Empresa, solo los administradores pueden agregar servidores.
  </Step>

  <Step title="Autenticar el servidor MCP">
    Complete los pasos de autenticación requeridos en claude.ai.
  </Step>

  <Step title="Ver y gestionar servidores en Claude Code">
    En Claude Code, use el comando:

    ```text theme={null}
    /mcp
    ```

    Los servidores de claude.ai aparecen en la lista con indicadores que muestran que provienen de claude.ai.
  </Step>
</Steps>

A partir de v2.1.161, los conectores en los que nunca ha iniciado sesión se contraen detrás de una fila `Show unused connectors` al final de la sección de claude.ai, por lo que una lista provista por la organización no llena el panel. Seleccione la fila para expandirlos. Un conector en el que inició sesión anteriormente permanece visible incluso cuando actualmente necesita reautenticación.

Los conectores de claude.ai se obtienen solo cuando su [método de autenticación](/docs/es/authentication#authentication-precedence) activo es su suscripción a claude.ai. No se cargan cuando `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `apiKeyHelper`, o un proveedor de terceros como Amazon Bedrock o Google Cloud's Agent Platform está activo, incluso si ejecutó previamente `/login`.

Si `/mcp` no enumera un conector que agregó, ejecute `/status` para confirmar qué método de autenticación está activo, desestablezca esa variable de entorno o elimine la configuración `apiKeyHelper`, luego ejecute `/login` para seleccionar su cuenta de claude.ai.

Un servidor que ha agregado en Claude Code tiene [precedencia](#scope-hierarchy-and-precedence) sobre un conector de claude.ai que apunta a la misma URL. Cuando esto sucede, `/mcp` enumera el conector como oculto y muestra cómo eliminar el duplicado si prefiere usar el conector.

Algunos conectores alojados por Anthropic, como Microsoft 365, Gmail y Google Calendar, no admiten OAuth local desde Claude Code porque el proveedor de identidad ascendente solo acepta la URL de redirección que registró claude.ai. A partir de v2.1.162, autenticar uno de estos hosts en `/mcp` muestra un mensaje que lo dirige a conectarlo en Configuración → Conectores en claude.ai en su lugar. Una vez conectado allí, el conector aparece en Claude Code automáticamente.

<h3 id="organization-controls-on-connector-tools">
  Controles de la organización en herramientas de conectores
</h3>

Su organización puede establecer controles por herramienta en [conectores de claude.ai](https://claude.com/docs/connectors). Claude Code lee estas configuraciones al iniciarse y las aplica localmente. Ejecute `/mcp` para ver qué configuración se aplica a cada herramienta en un conector.

* **Herramienta establecida en `ask`**: Claude Code solicita en cada llamada con la razón `Your organization requires approval for this tool`. La solicitud aparece incluso en los [modos de permiso](/docs/es/permissions#permission-modes) `acceptEdits`, `auto` y `bypassPermissions`, y nunca ofrece una opción para recordar su elección. Las [reglas de permiso](/docs/es/permissions) que coinciden con la herramienta tampoco omiten la solicitud. En modo `dontAsk`, que nunca solicita, Claude Code deniega la llamada en su lugar.
* **Herramienta establecida en `blocked`**: Claude Code filtra la herramienta antes de que Claude la vea, por lo que nunca aparece en la lista de herramientas.

Aplicar estos controles requiere Claude Code v2.1.129 o posterior. Las versiones anteriores ignoran la configuración y aplican el flujo de permiso estándar.

<h3 id="disable-claude-ai-connectors">
  Desactivar conectores de claude.ai
</h3>

Para desactivar servidores MCP de claude.ai en Claude Code, establezca [`disableClaudeAiConnectors`](/docs/es/settings#available-settings) en `true` en cualquier ámbito de configuración:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Esta configuración utiliza semántica de verdadero desde cualquier fuente: `true` en cualquier fuente de configuración tiene precedencia. Un archivo `.claude/settings.json` de proyecto verificado puede optar por no usar conectores en la nube, pero un `false` a nivel de proyecto no puede volver a habilitar conectores que un `true` a nivel de usuario o política ha deshabilitado. Los servidores pasados explícitamente a través de `--mcp-config` no se ven afectados.

También puede establecer la variable de entorno `ENABLE_CLAUDEAI_MCP_SERVERS` en `false`, que tiene el mismo efecto para la sesión de shell actual:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Para bloquear conectores individuales de claude.ai en lugar de todos ellos, agréguelos a [`deniedMcpServers`](/docs/es/managed-mcp) por nombre o por patrón de URL. Por ejemplo, una entrada `serverName` de `"claude.ai Slack"` bloquea el conector de Slack. Para activar o desactivar un conector solo para el proyecto actual, use el panel `/mcp`.

<Note>
  Estas configuraciones del lado del cliente rigen las sesiones locales de Claude Code. En sesiones de [Claude Code en la web](/docs/es/claude-code-on-the-web), los conectores de claude.ai son aprovisionados por el host remoto y llegan como entradas explícitas de `--mcp-config`, por lo que `disableClaudeAiConnectors` no se aplica allí. Las URL de conectores también se reescriben a través del proxy de sesión, por lo que un patrón `serverUrl` de `deniedMcpServers` dirigido a la URL del proveedor no coincidirá. Gestione qué conectores puede usar una sesión en la nube desde la configuración de su organización en claude.ai.
</Note>

<h2 id="use-claude-code-as-an-mcp-server">
  Usar Claude Code como servidor MCP
</h2>

Puede usar Claude Code mismo como servidor MCP al que otras aplicaciones pueden conectarse:

```bash theme={null}
# Iniciar Claude como servidor MCP stdio
claude mcp serve
```

Puede usar esto en Claude Desktop agregando esta configuración a claude\_desktop\_config.json:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Configurar la ruta del ejecutable**: El campo `command` debe hacer referencia al ejecutable de Claude Code. Si el comando `claude` no está en el PATH del sistema, deberá especificar la ruta completa al ejecutable.

  Para encontrar la ruta completa:

  ```bash theme={null}
  which claude
  ```

  Luego use la ruta completa en su configuración:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Sin la ruta correcta del ejecutable, encontrará errores como `spawn claude ENOENT`.
</Warning>

<Tip>
  Consejos:

  * El servidor proporciona acceso a las herramientas de Claude como View, Edit, LS, etc.
  * En Claude Desktop, intente pedirle a Claude que lea archivos en un directorio, haga ediciones y más.
  * Este servidor MCP solo expone las herramientas de Claude Code a su cliente MCP, por lo que su propio cliente es responsable de implementar la confirmación del usuario para llamadas de herramientas individuales.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Límites de salida de MCP y advertencias
</h2>

Cuando las herramientas MCP producen salidas grandes, Claude Code ayuda a gestionar el uso de tokens para evitar abrumar el contexto de su conversación:

* **Umbral de advertencia de salida**: Claude Code muestra una advertencia cuando la salida de cualquier herramienta MCP excede 10,000 tokens
* **Límite configurable**: Puede ajustar los tokens de salida MCP máximos permitidos usando la variable de entorno `MAX_MCP_OUTPUT_TOKENS`
* **Límite predeterminado**: El máximo predeterminado es 25,000 tokens
* **Alcance**: La variable de entorno se aplica a herramientas que no declaran su propio límite. Las herramientas que establecen [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) usan ese valor en su lugar para contenido de texto, independientemente de lo que `MAX_MCP_OUTPUT_TOKENS` esté establecido. Las herramientas que devuelven datos de imagen aún están sujetas a `MAX_MCP_OUTPUT_TOKENS`

Para aumentar el límite para herramientas que producen salidas grandes:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

Esto es particularmente útil cuando se trabaja con servidores MCP que:

* Consultan grandes conjuntos de datos o bases de datos
* Generan reportes o documentación detallados
* Procesan archivos de registro extensos o información de depuración

<h3 id="raise-the-limit-for-a-specific-tool">
  Aumentar el límite para una herramienta específica
</h3>

Si está construyendo un servidor MCP, puede permitir que herramientas individuales devuelvan resultados más grandes que el umbral predeterminado de persistencia en disco estableciendo `_meta["anthropic/maxResultSizeChars"]` en la entrada de la herramienta en la respuesta `tools/list`. Claude Code aumenta el umbral de esa herramienta al valor anotado, hasta un límite máximo de 500,000 caracteres.

Esto es útil para herramientas que devuelven salidas inherentemente grandes pero necesarias, como esquemas de bases de datos o árboles de archivos completos. Sin la anotación, los resultados que exceden el umbral predeterminado se persisten en disco y se reemplazan con una referencia de archivo en la conversación.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

La anotación se aplica independientemente de `MAX_MCP_OUTPUT_TOKENS` para contenido de texto, por lo que los usuarios no necesitan aumentar la variable de entorno para herramientas que la declaran. Las herramientas que devuelven datos de imagen aún están sujetas al límite de tokens.

<Warning>
  Si frecuentemente encuentra advertencias de salida con servidores MCP específicos que no controla, considere aumentar el límite `MAX_MCP_OUTPUT_TOKENS`. También puede pedirle al autor del servidor que agregue la anotación `anthropic/maxResultSizeChars` o que pagine sus respuestas. La anotación no tiene efecto en herramientas que devuelven contenido de imagen; para esas, aumentar `MAX_MCP_OUTPUT_TOKENS` es la única opción.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Esquemas de entrada de herramientas con un combinador a nivel raíz
</h2>

Algunos servidores MCP declaran el esquema de entrada de una herramienta como una unión de JSON Schema, con `anyOf`, `oneOf`, o `allOf` en el nivel superior del esquema. La API de Claude no acepta esas palabras clave en la raíz del esquema. Sí acepta combinadores anidados dentro de `properties`, que Claude Code envía sin cambios.

A partir de Claude Code v2.1.195, las herramientas con un combinador a nivel raíz permanecen disponibles. Antes de enviar la herramienta a la API, Claude Code aplana el esquema en un único objeto y antepone una oración a la descripción de la herramienta que le dice a Claude qué grupos de parámetros pertenecen juntos:

* `allOf`: las propiedades de cada rama se fusionan, y la lista `required` de cada rama aún se aplica
* `anyOf` y `oneOf`: las propiedades de cada rama se fusionan, y la lista `required` de cada rama se describe en la descripción de la herramienta en lugar de ser aplicada por el esquema

Su servidor recibe los argumentos que Claude eligió, así que siga validando la combinación del lado del servidor.

Cuando Claude Code no puede producir un esquema que la API acepte, o en una implementación que no recibe la configuración remota que habilita la reescritura, como una máquina sin conexión, omite esa herramienta, registra la razón en el registro del servidor, y deja disponibles las otras herramientas del servidor. Las versiones anteriores a v2.1.195 omiten cada herramienta cuyo esquema de entrada tiene un `anyOf`, `oneOf`, o `allOf` a nivel raíz.

<h2 id="require-approval-for-a-specific-tool">
  Requerir aprobación para una herramienta específica
</h2>

Si está construyendo un servidor MCP, puede marcar una herramienta como que requiere aprobación explícita en cada llamada estableciendo `_meta["anthropic/requiresUserInteraction"]` en `true` en la entrada de la herramienta en la respuesta `tools/list`. El valor debe ser el booleano JSON `true`; cualquier otro valor se ignora.

Claude Code muestra el indicador de permiso de esa herramienta en cada llamada, incluso en modos de permiso `acceptEdits`, `auto` y `bypassPermissions` [permission modes](/docs/es/permissions#permission-modes), y no ofrece una opción "no preguntar de nuevo" para ella. Las [reglas de permiso](/docs/es/permissions#permission-rule-syntax) que coinciden con la herramienta tampoco omiten el indicador. En modo `dontAsk`, que nunca solicita, Claude Code niega la llamada en su lugar.

El indicador tiene que llegar a una persona. En modo no interactivo con [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags), un resultado `allow` del indicador de permiso para una herramienta marcada se convierte en una negación con el mensaje `MCP tool requires user interaction; not supported via --permission-prompt-tool`. La devolución de llamada [`canUseTool`](/docs/es/agent-sdk/permissions) del Agent SDK sí recibe estas llamadas y puede aprobarlas, porque se espera que el host del SDK las muestre a un usuario.

Use esto para herramientas cuyo indicador de permiso es en sí el punto, como un paso de consentimiento o concesión de acceso donde la aprobación automática significaría que ningún humano nunca estuvo de acuerdo. Otras herramientas del mismo servidor mantienen su comportamiento de permiso normal.

La siguiente entrada `tools/list` marca una herramienta como siempre requiriendo aprobación.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

La anotación `anthropic/requiresUserInteraction` requiere Claude Code v2.1.199 o posterior. Las versiones anteriores la ignoran y aplican el flujo de permiso estándar.

Cuando una sesión está conectada a [Remote Control](/docs/es/remote-control) o a un host del SDK, Claude Code marca la solicitud de permiso como que requiere interacción del usuario, por lo que el cliente muestra el indicador de permiso de la herramienta para que usted responda en lugar de una acción de aprobación de un toque.

<h2 id="respond-to-mcp-elicitation-requests">
  Responder a solicitudes de elicitación de MCP
</h2>

Los servidores MCP pueden solicitar entrada estructurada de usted durante una tarea usando elicitación. Cuando un servidor necesita información que no puede obtener por sí solo, Claude Code muestra un diálogo interactivo y pasa su respuesta de vuelta al servidor. No se requiere configuración de su parte: los diálogos de elicitación aparecen automáticamente cuando un servidor los solicita.

Los servidores pueden solicitar entrada de dos formas:

* **Modo de formulario**: Claude Code muestra un diálogo con campos de formulario definidos por el servidor (por ejemplo, un indicador de nombre de usuario y contraseña). Complete los campos y envíe.
* **Modo de URL**: Claude Code abre una URL del navegador para autenticación o aprobación. Complete el flujo en el navegador, luego confirme en la CLI.

Para responder automáticamente a solicitudes de elicitación sin mostrar un diálogo, use el [hook `Elicitation`](/docs/es/hooks#elicitation).

Si está construyendo un servidor MCP que usa elicitación, vea la [especificación de elicitación de MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) para detalles de protocolo y ejemplos de esquema.

<h2 id="use-mcp-resources">
  Usar recursos MCP
</h2>

Los servidores MCP pueden exponer recursos que puede referenciar usando menciones @, similar a cómo referencia archivos.

<h3 id="reference-mcp-resources">
  Referenciar recursos MCP
</h3>

<Steps>
  <Step title="Listar recursos disponibles">
    Escriba `@` en su indicación para ver los recursos disponibles de todos los servidores MCP conectados. Los recursos aparecen junto a los archivos en el menú de autocompletado.
  </Step>

  <Step title="Referenciar un recurso específico">
    Use el formato `@server:protocol://resource/path` para referenciar un recurso:

    ```text theme={null}
    ¿Puede analizar @github:issue://123 y sugerir una solución?
    ```

    ```text theme={null}
    Por favor revise la documentación de API en @docs:file://api/authentication
    ```
  </Step>

  <Step title="Múltiples referencias de recursos">
    Puede referenciar múltiples recursos en una sola indicación:

    ```text theme={null}
    Compare @postgres:schema://users con @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Los recursos se obtienen automáticamente e incluyen como adjuntos cuando se referencian
  * Las rutas de recursos son búsquedas difusas en el autocompletado de menciones @
  * Claude Code proporciona automáticamente herramientas para listar y leer recursos MCP cuando los servidores los admiten
  * Los recursos pueden contener cualquier tipo de contenido que proporcione el servidor MCP (texto, JSON, datos estructurados, etc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Escalar con búsqueda de herramientas MCP
</h2>

La búsqueda de herramientas mantiene el uso de contexto MCP bajo al diferir las definiciones de herramientas hasta que Claude las necesite. Solo los nombres de herramientas e instrucciones del servidor se cargan al iniciar la sesión, por lo que agregar más servidores MCP tiene un impacto mínimo en su ventana de contexto. Claude Code no impone un límite fijo de herramientas por servidor; el límite práctico es su presupuesto de ventana de contexto.

<h3 id="how-it-works">
  Cómo funciona
</h3>

La búsqueda de herramientas está habilitada de forma predeterminada. Las herramientas MCP se difieren en lugar de cargarse en el contexto de antemano, y Claude usa una herramienta de búsqueda para descubrir las relevantes cuando una tarea las necesita. Solo las herramientas que Claude realmente usa entran en el contexto. Desde su perspectiva, las herramientas MCP funcionan exactamente como antes.

Si prefiere carga basada en umbral, establezca `ENABLE_TOOL_SEARCH=auto` para cargar esquemas de antemano cuando se ajusten dentro del 10% de la ventana de contexto y diferir solo el desbordamiento. Vea [Configurar búsqueda de herramientas](#configure-tool-search) para todas las opciones.

<h3 id="for-mcp-server-authors">
  Para autores de servidores MCP
</h3>

Si está construyendo un servidor MCP, el campo de instrucciones del servidor se vuelve más útil con la búsqueda de herramientas habilitada. Las instrucciones del servidor ayudan a Claude a entender cuándo buscar sus herramientas, similar a cómo funcionan las [skills](/docs/es/skills).

Agregue instrucciones claras y descriptivas del servidor que expliquen:

* Qué categoría de tareas manejan sus herramientas
* Cuándo Claude debe buscar sus herramientas
* Capacidades clave que proporciona su servidor

Claude Code trunca descripciones de herramientas e instrucciones del servidor en 2KB cada una. Manténgalas concisas para evitar truncamiento, y ponga detalles críticos cerca del inicio.

<h3 id="configure-tool-search">
  Configurar búsqueda de herramientas
</h3>

La búsqueda de herramientas está habilitada de forma predeterminada: las herramientas MCP se difieren y se descubren bajo demanda. Claude Code lo desactiva de forma predeterminada en la Plataforma de Agentes de Google Cloud. También se desactiva cuando `ANTHROPIC_BASE_URL` apunta a un host que no es de primera parte, ya que la mayoría de los proxies no reenvían bloques `tool_reference`. Establezca `ENABLE_TOOL_SEARCH` explícitamente para anular cualquiera de estos comportamientos predeterminados.

Configurar [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/es/env-vars) mantiene la búsqueda de herramientas desactivada, y `ENABLE_TOOL_SEARCH` no puede anularla. La variable elimina el encabezado beta que requieren las definiciones de herramientas `defer_loading` y los bloques de contenido `tool_reference`.

La búsqueda de herramientas requiere un modelo que admita bloques `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 y modelos posteriores. Vea [compatibilidad de modelos en la documentación de API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para la lista actual. En la Plataforma de Agentes de Google Cloud, la búsqueda de herramientas se admite para Claude Sonnet 4.5 y posterior y Claude Opus 4.5 y posterior.

Controle el comportamiento de búsqueda de herramientas con la variable de entorno `ENABLE_TOOL_SEARCH`:

| Valor            | Comportamiento                                                                                                                                                                                                                                                                                                               |
| :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (sin establecer) | Todas las herramientas MCP diferidas y cargadas bajo demanda. Recurre a carga de antemano en la Plataforma de Agentes de Google Cloud o cuando `ANTHROPIC_BASE_URL` es un host que no es de primera parte                                                                                                                    |
| `true`           | Todas las herramientas MCP diferidas. Claude Code envía el encabezado beta incluso en la Plataforma de Agentes de Google Cloud y a través de proxies. Las solicitudes fallan en modelos de la Plataforma de Agentes de Google Cloud anteriores a Sonnet 4.5 u Opus 4.5, o en proxies que no admiten bloques `tool_reference` |
| `auto`           | Modo de umbral: las herramientas se cargan de antemano si se ajustan dentro del 10% de la ventana de contexto, diferidas de lo contrario                                                                                                                                                                                     |
| `auto:N`         | Modo de umbral con un porcentaje personalizado, donde `N` es 0-100. Por ejemplo, `auto:5` para 5%                                                                                                                                                                                                                            |
| `false`          | Todas las herramientas MCP cargadas de antemano, sin diferimiento                                                                                                                                                                                                                                                            |

```bash theme={null}
# Usar un umbral personalizado del 5%
ENABLE_TOOL_SEARCH=auto:5 claude

# Desactivar búsqueda de herramientas completamente
ENABLE_TOOL_SEARCH=false claude
```

O establezca el valor en su [campo `env` de settings.json](/docs/es/settings#available-settings).

También puede desactivar la herramienta `ToolSearch` específicamente:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Eximir un servidor del diferimiento
</h3>

Si las herramientas de un servidor deben ser siempre visibles para Claude sin un paso de búsqueda, establezca `alwaysLoad` en `true` en la configuración de ese servidor. Cada herramienta de ese servidor se carga entonces en el contexto al iniciar la sesión independientemente de la configuración `ENABLE_TOOL_SEARCH`. Use esto para un pequeño número de herramientas que Claude necesita en cada turno, ya que cada herramienta de antemano consume contexto que de otro modo estaría disponible para su conversación.

La siguiente entrada `.mcp.json` exime un servidor HTTP mientras deja otros servidores diferidos:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

El campo `alwaysLoad` está disponible en todos los tipos de servidor y requiere Claude Code v2.1.121 o posterior. Un servidor MCP también puede marcar herramientas individuales como siempre cargadas incluyendo `"anthropic/alwaysLoad": true` en el objeto `_meta` de la herramienta, que tiene el mismo efecto solo para esa herramienta.

Establecer `alwaysLoad: true` también bloquea el inicio hasta que el servidor se conecte, limitado al tiempo de espera de conexión estándar de 5 segundos. Esto se aplica incluso cuando MCP startup es de otro modo [no bloqueante de forma predeterminada](/docs/es/env-vars), ya que las herramientas deben estar presentes cuando se construye el primer mensaje. Otros servidores aún se conectan en segundo plano.

<h2 id="use-mcp-prompts-as-commands">
  Usar indicaciones MCP como comandos
</h2>

Los servidores MCP pueden exponer indicaciones que se vuelven disponibles como comandos en Claude Code.

<h3 id="execute-mcp-prompts">
  Ejecutar indicaciones MCP
</h3>

<Steps>
  <Step title="Descubrir indicaciones disponibles">
    Escriba `/` para ver todos los comandos disponibles, incluyendo los de servidores MCP. Las indicaciones MCP aparecen con el formato `/mcp__servername__promptname`.
  </Step>

  <Step title="Ejecutar una indicación sin argumentos">
    ```text theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Ejecutar una indicación con argumentos">
    Muchas indicaciones aceptan argumentos. Páselos separados por espacios después del comando:

    ```text theme={null}
    /mcp__github__pr_review 456
    ```

    ```text theme={null}
    /mcp__jira__create_issue "Bug en flujo de inicio de sesión" high
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Las indicaciones MCP se descubren dinámicamente desde servidores conectados
  * Los argumentos se analizan basándose en los parámetros definidos de la indicación
  * Los resultados de la indicación se inyectan directamente en la conversación
  * Los nombres de servidor e indicación se normalizan, con espacios convertidos en guiones bajos
</Tip>

<h2 id="managed-mcp-configuration">
  Configuración MCP gestionada
</h2>

Para organizaciones que necesitan control centralizado sobre qué servidores MCP pueden conectar los usuarios, consulte [Configuración MCP gestionada](/docs/es/managed-mcp). Cubre la implementación de un conjunto de servidores fijo con `managed-mcp.json`, la restricción de servidores con `allowedMcpServers` y `deniedMcpServers`, y lo que los usuarios ven cuando un servidor está bloqueado.
