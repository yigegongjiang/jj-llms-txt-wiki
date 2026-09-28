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

    Si la instalación falla, haga coincidir el mensaje que Claude Code reporta:

    * `Marketplace "claude-plugins-official" not found`: agregue el marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, luego reintente la instalación.
    * El plugin [no se encuentra en el marketplace](/docs/es/plugins/install#install-a-plugin): verifique el nombre del plugin.

    Si el resumen de instalación reporta `Run /reload-plugins to activate.`, Claude Code ejecuta ese recarga para usted. Si la recarga advierte que su próximo mensaje volvería a leer la conversación, ejecute `/reload-plugins --force`.
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

Los servidores HTTP son la opción recomendada para conectarse a servidores MCP remotos. Este es el transporte más ampliamente compatible para servicios basados en la nube.

```bash theme={null}
# Sintaxis básica
claude mcp add --transport http <name> <url>

# Ejemplo real: Conectar a Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Ejemplo con token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Al configurar servidores MCP a través de JSON en `.mcp.json`, `~/.claude.json`, o `claude mcp add-json`, el campo `type` acepta `streamable-http` como alias para `http`. La especificación MCP utiliza el nombre `streamable-http` para este transporte, por lo que las configuraciones copiadas de la documentación del servidor funcionan sin modificación.

Una entrada JSON que tiene una `url` pero no tiene `type` es un error de configuración, porque Claude Code lee una entrada sin `type` como un servidor stdio. Claude Code omite ese servidor e informa `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Antes de v2.1.202, Claude Code informaba esta configuración incorrecta como `command: expected string, received undefined`.

En ejecuciones `--output-format stream-json`, Claude Code también informa una entrada `--mcp-config` omitida en el campo [`mcp_server_errors`](/docs/es/headless#stream-responses) del evento `system/init`, para que los scripts puedan detectar que el servidor nunca se cargó. Esto requiere Claude Code v2.1.219 o posterior.

<h3 id="option-2-add-a-remote-sse-server">
  Opción 2: Agregar un servidor SSE remoto
</h3>

<Warning>
  El transporte SSE (Server-Sent Events) está deprecado. Utilice servidores HTTP en su lugar, cuando estén disponibles.
</Warning>

Algunos servicios aún exponen solo un punto final SSE. Agréguelos con el mismo comando `claude mcp add --transport http <name> <url>` que [un servidor HTTP](#option-1-add-a-remote-http-server). Claude Code intenta el transporte HTTP primero y cambia a SSE cuando el servidor no lo acepta. El cambio automático requiere Claude Code v2.1.265 o posterior.

En una versión anterior, o para conectarse sobre SSE directamente, pase `--transport sse` en su lugar:

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

Claude Code establece `CLAUDE_PROJECT_DIR` en el entorno del servidor generado a la raíz del proyecto, para que su servidor pueda resolver rutas relativas al proyecto sin depender del directorio de trabajo. Este es el mismo directorio que los hooks reciben en su variable `CLAUDE_PROJECT_DIR`. Léalo desde dentro de su proceso de servidor, por ejemplo `process.env.CLAUDE_PROJECT_DIR` en Node o `os.environ["CLAUDE_PROJECT_DIR"]` en Python.

`CLAUDE_PROJECT_DIR` es la raíz del proyecto estable y no cambia cuando agrega o elimina directorios de trabajo a mitad de sesión. Un servidor que limita su propio acceso al sistema de archivos a un conjunto de directorios permitidos debe implementar la solicitud MCP `roots/list` en su lugar. Claude Code responde `roots/list` con el directorio de lanzamiento de la sesión más cada [directorio de trabajo adicional](/docs/es/permissions#working-directories) que haya otorgado con `--add-dir`, `/add-dir`, o la configuración `additionalDirectories`. Claude Code envía `notifications/roots/list_changed` cuando ese conjunto cambia. Antes de v2.1.203, `roots/list` devolvía solo el directorio de lanzamiento y Claude Code no enviaba `notifications/roots/list_changed`.

Esta variable se establece en el entorno del servidor, no en el entorno propio de Claude Code, por lo que hacer referencia a ella a través de la expansión `${VAR}` en el `command` o `args` de una entrada `.mcp.json` con alcance de proyecto o una entrada de servidor local o de usuario en `~/.claude.json` requiere un valor predeterminado como `${CLAUDE_PROJECT_DIR:-.}`. Las configuraciones MCP proporcionadas por plugins sustituyen `${CLAUDE_PROJECT_DIR}` directamente y no necesitan el valor predeterminado.

```bash theme={null}
# Sintaxis básica
claude mcp add [options] <name> -- <command> [args...]

# Ejemplo real: Agregar servidor Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Importante: Separar argumentos del servidor con `--`**

  Para servidores stdio, el `--` (doble guión) separa las opciones propias de Claude, como `--transport`, `--env`, y `--scope`, del comando y argumentos que ejecutan el servidor. Todo lo que viene después de `--` se pasa al servidor sin modificación.

  Por ejemplo:

  * `claude mcp add --transport stdio myserver -- npx server` → ejecuta `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → ejecuta `python server.py --port 8080` con `KEY=value` en el entorno

  Sin `--`, Claude Code intentaría analizar las banderas del servidor, como `--port` arriba, como sus propias opciones.

  `--env` acepta múltiples pares `KEY=value`. Si el nombre del servidor viene directamente después de `--env`, la CLI lee el nombre como otro par y lo rechaza, por lo que debe colocar al menos otra opción, como `--transport stdio`, entre `--env` y el nombre del servidor.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opción 4: Agregar un servidor WebSocket remoto
</h3>

Los servidores WebSocket mantienen una conexión bidireccional persistente, que es adecuada para servidores MCP remotos que envían eventos a Claude sin ser solicitados. Utilice HTTP en su lugar cuando su servidor solo responda a solicitudes, ya que HTTP admite OAuth y la bandera `claude mcp add --transport`, mientras que WebSocket no admite ninguno de los dos.

Configure servidores WebSocket en `.mcp.json` o con `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

La entrada `type: "ws"` acepta los mismos campos `url`, `headers`, `headersHelper`, `timeout`, y `alwaysLoad` que `http`. La autenticación es solo por encabezado, por lo que pase un token estático en `headers` o genere uno en el momento de la conexión con [`headersHelper`](#use-dynamic-headers-for-custom-authentication). La bandera `claude mcp add --transport` no acepta `ws`.

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Agregar un servidor desde instrucciones de configuración escritas para otro cliente
</h3>

Los servidores MCP no son específicos de Claude Code, por lo que las instrucciones de configuración de un servidor pueden estar escritas para Claude Desktop, Cursor, u otro cliente MCP y no proporcionar ningún comando `claude mcp add`. Para agregar el servidor de todas formas, busque en esas instrucciones una URL, un comando de lanzamiento, o un bloque JSON:

* **Una URL** como `https://mcp.example.com/mcp`: el servidor es remoto.
* **Un comando de lanzamiento** como `npx -y @example/mcp-server`: el servidor se ejecuta en su máquina.
* **Un bloque JSON `mcpServers`**: configuración escrita para el archivo de configuración de otro cliente.

Cada uno es una de las entradas que las cuatro opciones en [Instalación de servidores MCP](#installing-mcp-servers) toman. Encuentre la forma que tiene a continuación para convertirla en el comando que Claude Code acepta. Cada comando escribe en [alcance local](#local-scope) a menos que agregue `--scope project` o `--scope user`.

<h4 id="from-a-url">
  Desde una URL
</h4>

Una URL significa que el servidor es remoto. Para un punto final `https://`, agréguelo con `--transport http`, o siga [Opción 2](#option-2-add-a-remote-sse-server) cuando las instrucciones digan que el punto final utiliza SSE. Para un punto final `wss://`, utilice [Opción 4](#option-4-add-a-remote-websocket-server) en su lugar, ya que `--transport` no acepta `ws`:

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Si las instrucciones también proporcionan una clave API o encabezado de token, páselo con `--header` como se muestra en [Opción 1](#option-1-add-a-remote-http-server).

<h4 id="from-an-npx-uvx-or-binary-command">
  Desde un comando `npx`, `uvx`, o binario
</h4>

Un comando de lanzamiento significa que el servidor se ejecuta como un proceso stdio local. Coloque todo el comando después de `--`, para que Claude Code pase banderas como `-y` al comando que inicia el servidor en lugar de leerlas como sus propias opciones. Pase cualquier variable de entorno que las instrucciones soliciten con `--env`, después del nombre del servidor y antes de `--`:

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[Opción 3](#option-3-add-a-local-stdio-server) cubre el separador `--` en su totalidad.

<h4 id="from-an-mcpservers-json-block">
  Desde un bloque JSON `mcpServers`
</h4>

Un bloque `mcpServers` escrito para otro cliente MCP, como Claude Desktop, utiliza la clave contenedora y la forma de entrada que Claude Code lee. Pase a `claude mcp add-json` el objeto dentro de `mcpServers`, no el contenedor. Dos entradas necesitan una reparación primero:

* **Una `url` sin `type`**: agregue `"type": "http"`, `"type": "sse"`, o `"type": "ws"` para coincidir con el punto final. Claude Code lee una entrada sin `type` como un servidor stdio, por lo que una entrada `url` sin `type` falla.
* **Una clave con caracteres distintos de letras, números, guiones e guiones bajos**: elija un nombre de servidor que utilice solo esos caracteres. De lo contrario, la clave es el nombre del servidor.

Por ejemplo, este bloque:

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

se convierte en este comando:

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[Agregar servidores MCP desde configuración JSON](#add-mcp-servers-from-json-configuration) cubre el escape de shell y la bandera `--scope` para `add-json`. Para compartir el servidor con su equipo en su lugar, agregue `--scope project`, o agregue la entrada bajo `mcpServers` en `.mcp.json` en la raíz de su proyecto y confírmela. [Alcance de proyecto](#project-scope) cubre cómo Claude Code carga y aprueba ese archivo.

Cada comando `claude mcp add` y `claude mcp add-json` imprime una línea `Added ...`. Para verificar que Claude Code se conectó, ejecute `claude mcp get <name>`; [Estado del servidor](#server-status) cubre los estados que muestra y el paso de aprobación para servidores `.mcp.json`.

<h3 id="managing-your-servers">
  Administración de sus servidores
</h3>

Una vez configurados, puede administrar sus servidores MCP con estos comandos:

```bash theme={null}
# Listar todos los servidores configurados
claude mcp list

# Obtener detalles de un servidor específico
claude mcp get notion

# Eliminar un servidor
claude mcp remove notion

# (dentro de Claude Code) Verificar estado del servidor
/mcp
```

Cuando elimina un servidor remoto, Claude Code también elimina los tokens OAuth y el registro de cliente que almacenó para ese servidor.

<h4 id="server-status">
  Estado del servidor
</h4>

`claude mcp add` confirma una adición exitosa imprimiendo una línea `Added ...`, lo que significa que la configuración se escribió. `claude mcp list` luego muestra un estado de salud junto a cada servidor que enumera, como `✔ Connected`, `! Needs authentication`, o `✘ Failed to connect`. Un estado de falla significa que Claude Code no pudo conectarse a ese servidor, no que el comando list haya fallado.

Los estados en esta lista informan una decisión de configuración en lugar de un intento de conexión, por lo que Claude Code los imprime sin conectarse al servidor:

* ``⏸ Pending approval (run `claude` to approve)``: un servidor con alcance de proyecto de `.mcp.json` que aún no ha aprobado. Claude Code lo muestra tanto en `claude mcp list` como en `claude mcp get <name>`. Ejecute `claude` interactivamente para revisarlo y aprobarlo.
* `✘ Rejected (see disabledMcpjsonServers in settings)`: un servidor `.mcp.json` que una entrada [`disabledMcpjsonServers`](/docs/es/settings-reference#disabledmcpjsonservers) rechaza. Claude Code lo muestra solo en `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)`: un servidor que la lista [`disabledMcpServers`](#disable-a-server-without-removing-it) del proyecto nombra. Claude Code lo muestra tanto en `claude mcp list` como en `claude mcp get <name>`. Active el servidor nuevamente desde el panel `/mcp`. Antes de v2.1.238, ambos comandos se conectaban a un servidor deshabilitado para verificar su salud e informaban el resultado de la conexión.

Los servidores WebSocket no aparecen en la salida de `claude mcp list`. Utilice `claude mcp get <name>` o el panel `/mcp` para verificarlos.

<h4 id="project-server-approvals-and-workspace-trust">
  Aprobaciones de servidores de proyecto y confianza del espacio de trabajo
</h4>

A partir de v2.1.196, `claude mcp list` y `claude mcp get` leen aprobaciones `.mcp.json` solo de archivos de configuración que no se registran en el repositorio hasta que confíe en el espacio de trabajo ejecutando `claude` en él y aceptando el diálogo de confianza del espacio de trabajo. Un repositorio clonado no puede aprobar sus propios servidores: [`enableAllProjectMcpServers`](/docs/es/settings-reference#enableallprojectmcpservers) o [`enabledMcpjsonServers`](/docs/es/settings-reference#enabledmcpjsonservers) confirmados en `.claude/settings.json` del proyecto se ignoran en una carpeta no confiable, y el servidor permanece en `⏸ Pending approval` en lugar de estar conectado y verificado de salud.

Las aprobaciones de estas fuentes aún se aplican en una carpeta no confiable:

* su `~/.claude/settings.json` de usuario
* configuración administrada
* configuración pasada con `--settings`

Claude Code también aplica aprobaciones de un `.claude/settings.local.json` sin seguimiento, pero ejecuta git para verificar si el archivo se rastrea, y ejecuta esa verificación solo en una [carpeta confiable](/docs/es/permissions#project-allow-rules-and-workspace-trust). En una carpeta que nunca ha confiado, Claude Code espera el diálogo de confianza antes de aplicar las aprobaciones del archivo, a menos que la carpeta sea su propio hogar de configuración: su directorio de inicio, o un directorio cuyo `.claude` haya establecido como [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars). Antes de v2.1.207, Claude Code aplicaba aprobaciones de un `.claude/settings.local.json` sin seguimiento incluso en una carpeta que nunca había confiado.

Una entrada `disabledMcpjsonServers` en cualquier archivo de configuración aún rechaza el servidor.

<h4 id="server-status-detail">
  Detalle del estado del servidor
</h4>

En `/mcp`, incluido el menú de un servidor allí, y en el administrador [`/plugin`](/docs/es/plugins/install), un servidor HTTP o SSE remoto que ha utilizado antes puede mostrar un estado `cached` como `cached 2h ago · connects on first use · 5 tools`. Claude Code cargó la lista de herramientas del servidor desde su caché de descubrimiento, guardado en una sesión anterior, en lugar de conectarse al inicio, y Claude Code conecta el servidor la primera vez que Claude llama a una de las herramientas del servidor. Las herramientas están disponibles desde su primer mensaje, por lo que no necesita hacer nada. El caché de descubrimiento y su estado `cached` requieren Claude Code v2.1.221 o posterior.

El caché de descubrimiento está desactivado de forma predeterminada a menos que un lanzamiento gradual lo haya habilitado para su cuenta. Establezca [`MCP_DISCOVERY_CACHE=1`](/docs/es/env-vars) para activarlo, o `0` para mantenerlo desactivado incluso cuando el lanzamiento lo haya habilitado. Antes de v2.1.238, el caché estaba activado de forma predeterminada.

Dos acciones en el menú de un servidor en `/mcp` también afectan la entrada de caché de ese servidor:

* **Reconnect**: en un servidor `cached`, Claude Code lo conecta ahora en lugar de en su primera llamada de herramienta y mantiene la entrada. En un servidor conectado o fallido, Claude Code lo reconecta y también descarta la entrada.
* **Clear authentication**: Claude Code revoca la autenticación del servidor y también descarta la entrada.

Después de descartar la entrada, Claude Code obtiene la lista de herramientas del servidor desde el servidor en lugar de desde el caché.

Cuando el estado de un servidor es `✘ Failed to connect`, `claude mcp list` agrega el detalle de falla a esa línea de estado, y `claude mcp get <name>` lo muestra en una línea `Issue:`: el estado HTTP o código de error, más cualquier texto de error que el servidor devolvió. La vista de detalle del servidor en `/mcp` incluye el mismo texto informado por el servidor en su fila `Issue:`. Claude Code redacta texto similar a credenciales de este detalle y nunca incluye la URL del servidor expandida, que puede llevar secretos. Claude Code no agrega detalle a un estado `✘ Connection error`, porque el texto de excepción que imprimiría allí puede incrustar esa URL. Antes de v2.1.219, ambos comandos mostraban solo el estado de falla desnudo, sin el código de estado o el texto de error del servidor.

Cuando completa la autenticación desde `/mcp` y la conexión aún falla con un estado HTTP o un código de error de transporte, Claude Code agrega ese código y el origen de la URL del servidor al mensaje que imprime después del intento. El origen es el esquema y host, más el puerto cuando la URL nombra uno, como `https://mcp.example.com`.

* La ruta y consulta nunca aparecen en ese mensaje.
* Para un servidor en el [alcance](#mcp-installation-scopes) local, de proyecto o de usuario o en configuración MCP administrada, el origen muestra el host tal como está escrito en esa configuración, por lo que una referencia `${VAR}` en el host no se expande en el mensaje.
* Para una falla sin código de estado o de error, Claude Code muestra el texto de error sin el origen.

Un servidor remoto cuya configuración tiene una `url` vacía se muestra como `not configured` en `/mcp`, en `claude mcp list`, y en el administrador [`/plugin`](/docs/es/plugins/install), y Claude Code no intenta conectarse a él. Un plugin puede incluir una entrada de marcador de posición como esta para un conector que configura más tarde, por lo que Claude Code no lo informa como un error o un problema de configuración. La vista de detalle del servidor en `/mcp` lee `No URL configured for this server`; establezca la `url` de la entrada para conectarla. Antes de v2.1.208, Claude Code informaba una `url` vacía como un problema de configuración con un aviso para reconectar.

<h4 id="configuration-warnings">
  Advertencias de configuración
</h4>

Claude Code advierte sobre los problemas de configuración a continuación. Cada entrada dice qué verifica Claude Code y cómo borrar la advertencia:

* **Espacios en blanco ocultos**: Claude Code advierte cuando un valor de configuración MCP lleva espacios en blanco ocultos al principio o al final, que a menudo provienen de pegar un token con una nueva línea al final. Claude Code verifica `command`, `url`, cada entrada `args`, y los valores y nombres de clave bajo `env` y `headers`. Claude Code muestra la advertencia en la salida de `claude mcp list` y en `/mcp`, nombrando los campos afectados sin repetir sus valores, por ejemplo `Leading or trailing whitespace in: headers.Authorization`. Claude Code no recorta el espacio en blanco y utiliza los valores exactamente como están escritos, por lo que edite la configuración para eliminarlo.
* **Mismo nombre en más de un alcance**: si define el mismo nombre de servidor en más de un [alcance](#mcp-installation-scopes) con diferentes puntos finales, Claude Code advierte sobre el conflicto en la salida de `claude mcp list` y en `/mcp`. Claude Code almacena inicios de sesión OAuth por punto final, por lo que cuando autentica la definición que se carga en un proyecto, aún necesita iniciar sesión por separado en un proyecto donde se carga una definición diferente. Mantenga el punto final que desea y elimine los otros con `claude mcp remove <name> --scope <scope>`. En la advertencia, Claude Code cita el punto final de cada alcance tal como está escrito en su configuración, con referencias [`${VAR}`](#environment-variable-expansion-in-mcp-json) sin expandir, por lo que nunca muestra un valor resuelto como una clave API.
* **Nombres reservados**: Claude Code reserva los nombres de sus servidores integrados, incluidos `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, y `Claude Browser`. Si su configuración define un servidor con un nombre reservado, Claude Code lo omite en el tiempo de carga y muestra una advertencia pidiéndole que lo renombre. `claude mcp add` rechaza un nombre reservado con un error. `Claude Preview` y `Claude Browser` ambos nombran el servidor integrado que el [panel de vista previa de la aplicación de escritorio Claude Code](/docs/es/desktop#preview-your-app) utiliza. Antes de v2.1.205, `Claude Browser` no estaba reservado, por lo que un servidor configurado por el usuario podría registrarse bajo ese nombre.
* **Variable de entorno faltante**: si una referencia [`${VAR}`](#environment-variable-expansion-in-mcp-json) en la configuración de un servidor nombra una variable que no está establecida y no tiene `:-default`, Claude Code advierte en la salida de `claude mcp list` y en `/mcp`, nombrando la variable, y aún carga el servidor con el texto `${VAR}` sin expandir. Establezca la variable o agregue un respaldo `${VAR:-default}`. En la `url` y `headers` de un servidor remoto, algunas variables de credencial [se leen como vacías](#credential-variables-that-read-as-empty) en su lugar, sin advertencia.

<h4 id="tool-availability">
  Disponibilidad de herramientas
</h4>

El panel `/mcp` muestra el recuento de herramientas junto a cada servidor conectado e indica servidores que anuncian la capacidad de herramientas pero no exponen herramientas.

Si su solicitud necesita herramientas de un servidor que aún se está conectando en segundo plano, Claude espera a ese servidor antes de continuar. Cómo sucede la espera depende de su configuración:

* **Con [búsqueda de herramientas](#scale-with-mcp-tool-search), el valor predeterminado**: la espera sucede dentro de la llamada `ToolSearch`.
* **Sin búsqueda de herramientas**: Claude utiliza la herramienta `WaitForMcpServers` en su lugar. Las configuraciones sin búsqueda de herramientas incluyen un `ANTHROPIC_BASE_URL` personalizado, `ENABLE_TOOL_SEARCH=false`, y un modelo anterior a la generación Claude 4.5 en la Plataforma de Agentes de Google Cloud.
* **En un despliegue de Microsoft Foundry [alojado en Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude comienza en la ruta de búsqueda de herramientas en lugar de con `WaitForMcpServers`, ya que Claude Code descubre el rechazo del lado del servidor solo desde la API. Después de que Claude Code cambie ese despliegue a [carga anticipada](#scale-with-mcp-tool-search), las herramientas de un servidor que termina de conectarse están disponibles en la siguiente solicitud de Claude.

Con la búsqueda de herramientas habilitada, cuando un servidor termina de conectarse mientras Claude está trabajando, Claude Code enumera los nombres de herramientas del servidor a Claude en su siguiente solicitud en el mismo turno. Claude puede entonces buscar y llamar a esas herramientas sin esperar su siguiente mensaje.

<h3 id="disable-a-server-without-removing-it">
  Deshabilitar un servidor sin eliminarlo
</h3>

Alterne un servidor en el panel `/mcp` para detener que Claude Code se conecte a él sin perder su configuración. Claude Code aún enumera el servidor en `/mcp`, marcado como deshabilitado.

Cuando alterna un servidor, Claude Code registra su elección por proyecto en `~/.claude.json`, en una de dos listas que cubren conjuntos disjuntos de servidores:

* `disabledMcpServers`: una lista de exclusión para servidores configurados por el usuario, servidores de plugins, servidores que su organización [proporciona a través de configuración administrada](/docs/es/managed-mcp#provide-servers-through-managed-settings), los conectores claude.ai que Claude Code [obtiene por sí mismo](#how-connectors-reach-claude-code), y servidores integrados que están habilitados de forma predeterminada. Claude Code no se conecta a un servidor que enumere aquí. Cuando deshabilita un conector claude.ai con el alternador `/mcp` por proyecto descrito en [Deshabilitar conectores claude.ai](#disable-claude-ai-connectors), Claude Code lo escribe en esta lista bajo su nombre de visualización, por ejemplo `claude.ai Slack`.
* `enabledMcpServers`: una lista de inclusión para servidores integrados que están deshabilitados de forma predeterminada, como `computer-use`. Claude Code se conecta a un servidor deshabilitado de forma predeterminada solo cuando lo enumera aquí.

Claude Code consulta exactamente una de las dos listas para cada servidor, por lo que ninguna lista anula la otra. Si agrega un servidor normal a `enabledMcpServers`, o un servidor integrado deshabilitado de forma predeterminada a `disabledMcpServers`, Claude Code ignora la entrada.

`disabledMcpServers` y `enabledMcpServers` no están relacionados con [`enabledMcpjsonServers`](/docs/es/settings-reference#enabledmcpjsonservers) y [`disabledMcpjsonServers`](/docs/es/settings-reference#disabledmcpjsonservers), que controlan la aprobación de servidores definidos en el archivo `.mcp.json` de un proyecto.

<h3 id="mcp-client-runtimes">
  Tiempos de ejecución del cliente MCP
</h3>

Claude Code se conecta a servidores MCP a través de uno de dos tiempos de ejecución del cliente. El tiempo de ejecución v1 se basa en MCP TypeScript SDK 1.x. El tiempo de ejecución v2 es el mismo código en [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), que agrega la revisión del protocolo MCP 2026-07-28. El resto de esta página se aplica a ambos tiempos de ejecución, excepto donde una sección nombra el tiempo de ejecución v2.

Claude Code elige un tiempo de ejecución cada vez que lo inicia y lo mantiene hasta que sale. En sesiones donde [obtiene banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), utiliza el tiempo de ejecución v2 en Claude Code v2.1.232 o posterior.

En las sesiones donde no obtiene banderas de características, Claude Code utiliza el tiempo de ejecución v2 de forma predeterminada en Claude Code v2.1.274 o posterior:

* Sesiones en Amazon Bedrock, Claude Platform en AWS, Plataforma de Agentes de Google Cloud, o Microsoft Foundry, a menos que una plataforma anfitriona que integra Claude Code establezca [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars)
* Sesiones iniciadas sesión a través de una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway)
* Sesiones donde desactiva la telemetría o la obtención de banderas de características, por ejemplo con `DISABLE_TELEMETRY`

En v2, Claude Code también:

* Pregunta a servidores HTTP si admiten la revisión más nueva, y la utiliza con los que lo hacen. También pregunta a servidores conectores claude.ai en sesiones donde obtiene banderas de características. Para que pregunte a servidores stdio, o a servidores conectores en cada sesión, establezca [`MCP_PROTOCOL_NEGOTIATION`](/docs/es/env-vars) en `auto`. Se conecta a todos los demás servidores como v1 lo hace.
* Recibe notificaciones `list_changed` de servidores en la revisión más nueva sobre una [secuencia que mantiene abierta](#notification-streams-on-the-v2-runtime).
* No registra un servidor de [canal](#push-messages-with-channels) que se conecta en la revisión más nueva, porque esa revisión no puede llevar mensajes de canal.
* Falla un [inicio de sesión OAuth de MCP](#authenticate-with-remote-mcp-servers) cuya respuesta de autorización nombra un emisor inesperado.

Anthropic puede mantener un servidor específico en el protocolo anterior, u off esa secuencia, con una bandera de características que Claude Code obtiene.

Para elegir el tiempo de ejecución usted mismo, establezca [`MCP_SDK_GENERATION`](/docs/es/env-vars) en `v1` o `v2`. Para decidir si Claude Code pregunta, establezca [`MCP_PROTOCOL_NEGOTIATION`](/docs/es/env-vars) en `auto` o `legacy`.

<h3 id="dynamic-tool-updates">
  Actualizaciones dinámicas de herramientas
</h3>

Claude Code admite notificaciones MCP `list_changed`, permitiendo que servidores MCP actualicen dinámicamente sus herramientas, indicaciones y recursos disponibles sin requerir que se desconecte y reconecte. Cuando un servidor MCP envía una notificación `list_changed`, Claude Code actualiza automáticamente las capacidades disponibles de ese servidor.

Si una solicitud de actualización falla, Claude Code mantiene las herramientas, indicaciones y recursos descubiertos anteriormente del servidor hasta que una actualización posterior tenga éxito. Antes de v2.1.214, un error transitorio durante la actualización reemplazaba las herramientas, indicaciones y recursos del servidor con una lista vacía.

<h4 id="notification-streams-on-the-v2-runtime">
  Secuencias de notificación en el tiempo de ejecución v2
</h4>

En el [tiempo de ejecución v2](#mcp-client-runtimes), Claude Code recibe notificaciones `list_changed` de un servidor en la revisión del protocolo más nueva sobre una secuencia que mantiene abierta. Cuando la secuencia se cierra, Claude Code la reabre, con dos límites:

* **La secuencia se cierra nuevamente dentro de 10 segundos**: Claude Code la reabre hasta tres veces, luego se detiene para esa conexión.
* **La secuencia permanece abierta más de 10 segundos, luego se cierra**, como las secuencias a hosts sin servidor comúnmente lo hacen: después de cinco reaperturas en una hora, Claude Code espera aproximadamente seis horas antes de la siguiente.

Hasta que la secuencia se reabre, mantiene las últimas herramientas, indicaciones y recursos obtenidos del servidor. Para recoger sus cambios más pronto, reconecte el servidor desde `/mcp`.

<h3 id="automatic-reconnection">
  Reconexión automática
</h3>

Claude Code reconecta un servidor remoto que se cae a mitad de sesión e intenta nuevamente la primera conexión de un servidor HTTP o SSE después de un error transitorio. Los servidores stdio son procesos locales, y Claude Code no los reconecta automáticamente.

<h4 id="mid-session-drops-of-a-remote-server">
  Caídas a mitad de sesión de un servidor remoto
</h4>

Claude Code reconecta un servidor remoto caído con retroceso exponencial: hasta cinco intentos, comenzando con un retraso de un segundo y duplicándolo cada vez. Lo que ve depende de cómo esté ejecutando Claude Code:

* **En una sesión interactiva**: `/mcp` muestra el servidor como pendiente mientras Claude Code se reconecta. Después de cinco intentos fallidos, Claude Code marca el servidor como fallido, o como necesitando autenticación cuando el servidor necesita autorización nuevamente. Cuando marca el servidor como fallido, ve una notificación `MCP server "<name>" disconnected · open /mcp to reconnect`. Puede reintentar manualmente desde `/mcp`.
* **En ejecuciones [`claude -p`](/docs/es/headless) y sesiones [Agent SDK](/docs/es/agent-sdk/overview)**: Claude Code se reconecta en el mismo cronograma, sin panel `/mcp` para mostrar los intentos.

<h4 id="failed-first-connections">
  Conexiones iniciales fallidas
</h4>

Cuando la primera conexión de un servidor HTTP o SSE falla con un error transitorio, como una respuesta 5xx, una conexión rechazada, o un tiempo de espera, Claude Code reintenta hasta tres veces. Si la conexión aún falla, Claude Code marca el servidor como fallido. Claude Code reintenta de esta manera al inicio y cuando se agrega un servidor a mitad de sesión. Eso incluye un servidor que Claude Code agrega a una [sesión en la nube](/docs/es/claude-code-on-the-web) desde su configuración y un servidor que agrega con el método [`setMcpServers()`](/docs/es/agent-sdk/typescript) del Agent SDK.

Claude Code no reintenta en estos casos:

* La primera conexión de un servidor WebSocket
* Un error de autenticación o no encontrado, porque requiere un cambio de configuración para resolverse. Cuando un [`headersHelper`](#use-dynamic-headers-for-custom-authentication) es la única fuente del servidor del encabezado `Authorization`, Claude Code reintenta un error de autenticación de todas formas, porque vuelve a ejecutar el ayudante en cada intento y puede recoger una credencial fresca

<h4 id="failed-discovery-requests">
  Solicitudes de descubrimiento fallidas
</h4>

Después de que un servidor se conecta, Claude Code le envía solicitudes de descubrimiento de capacidades como `tools/list`, `prompts/list`, y `resources/list`. Claude Code reintenta esas solicitudes hasta tres veces con retroceso corto después de un error de red o servidor transitorio. No reintenta errores de autenticación, respuestas 4xx, o tiempos de espera de solicitud.

<h4 id="how-claude-learns-that-a-server-failed">
  Cómo Claude aprende que un servidor falló
</h4>

Si Claude Code le dice a Claude sobre un servidor configurado que no se conectó depende de [búsqueda de herramientas](#scale-with-mcp-tool-search), que está activada de forma predeterminada:

* Con búsqueda de herramientas, Claude Code le dice a Claude qué servidor falló y su error de conexión, por lo que Claude informa la falla de conexión en su respuesta. Claude Code incluye la misma información en resultados de `ToolSearch` que no encuentran herramientas coincidentes.
* En cualquier [configuración sin búsqueda de herramientas](#configure-tool-search), Claude Code no informa fallas de conexión de servidor configurado a Claude.

<h3 id="push-messages-with-channels">
  Mensajes de inserción con canales
</h3>

Un servidor MCP también puede insertar mensajes directamente en su sesión para que Claude pueda reaccionar a eventos externos como resultados de CI, alertas de monitoreo, o mensajes de chat. Para habilitar esto, su servidor declara la capacidad `claude/channel` y usted lo activa con la bandera `--channels` al inicio. Consulte [Canales](/docs/es/channels) para utilizar un canal oficialmente compatible, o [Referencia de canales](/docs/es/channels-reference) para construir el suyo propio.

En el [tiempo de ejecución v2](#mcp-client-runtimes), si establece [`MCP_PROTOCOL_NEGOTIATION`](/docs/es/env-vars) en `auto` y un servidor de canal negocia la revisión del protocolo MCP 2026-07-28, no puede entregar mensajes de canal, por lo que Claude Code no lo registra como un canal. Dejar la variable sin establecer, o establecerla en `legacy`, mantiene servidores stdio en el protocolo anterior.

<Tip>
  Consejos:

  * Utilice la bandera `-s` o `--scope` para especificar dónde se almacena la configuración:
    * `local` (predeterminado): disponible solo para usted en el proyecto actual
    * `project`: compartido con todos en el proyecto a través del archivo `.mcp.json`
    * `user`: disponible para usted en todos los proyectos
  * Establezca variables de entorno con banderas `-e` o `--env` (por ejemplo, `-e KEY=value`)
  * Las banderas `--transport` y `--header` también aceptan formas cortas `-t` y `-H`
  * Configure el tiempo de espera de inicio del servidor MCP utilizando la variable de entorno `MCP_TIMEOUT` (por ejemplo, `MCP_TIMEOUT=10000 claude` establece un tiempo de espera de 10 segundos)
  * Establezca un tiempo de espera de ejecución de herramienta por servidor agregando un campo `timeout` en milisegundos a la entrada `.mcp.json` de ese servidor, por ejemplo `"timeout": 600000` para diez minutos. Esto anula la variable de entorno `MCP_TOOL_TIMEOUT` solo para ese servidor
  * Claude Code muestra una advertencia cuando la salida de herramientas MCP excede 10,000 tokens y limita la salida a 25,000 tokens de forma predeterminada. Para aumentar el límite, establezca la variable de entorno `MAX_MCP_OUTPUT_TOKENS` (por ejemplo, `MAX_MCP_OUTPUT_TOKENS=50000`); el umbral de advertencia es fijo. Consulte [Límites de salida de MCP y advertencias](#mcp-output-limits-and-warnings)
  * Utilice `/mcp` para autenticarse con servidores remotos que requieren autenticación OAuth 2.0
</Tip>

El `timeout` por servidor es un límite de reloj de pared duro por llamada de herramienta, y las notificaciones de progreso del servidor no lo extienden. Los valores por debajo de 1000 se ignoran y caen a `MCP_TOOL_TIMEOUT`, o a su valor predeterminado de aproximadamente 28 horas cuando esa variable no está establecida. Para un servidor HTTP, SSE, o [conector claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) también hay un segundo temporizador por solicitud que cubre cada solicitud hasta el primer byte de respuesta del servidor. Claude Code establece ese temporizador al mayor de tres valores: 60 segundos, el tiempo de espera de herramienta que se aplica al servidor, y `MCP_TIMEOUT`. El valor predeterminado de 28 horas de un `MCP_TOOL_TIMEOUT` sin establecer no entra en esa comparación, y un valor por debajo de 60 segundos no acorta el temporizador. Los servidores stdio y WebSocket no tienen temporizador por solicitud.

Un `timeout` por servidor de al menos 1000 también actúa como un piso en el tiempo de espera de inactividad descrito a continuación: Claude Code nunca aborta las llamadas de herramienta de ese servidor por inactividad más pronto que el `timeout` por servidor. Requiere Claude Code v2.1.203 o posterior.

Una llamada de herramienta a un servidor MCP que no envía respuesta y ninguna notificación de progreso para la ventana de inactividad aborta con un error en lugar de esperar el límite de reloj de pared. Se aplica a todos los tipos de servidor excepto servidores IDE y servidores en proceso del SDK. La ventana de inactividad tiene un valor predeterminado de cinco minutos para servidores HTTP, SSE, WebSocket, y [conector claude.ai](#use-mcp-servers-from-claude-ai), y de 30 minutos para servidores stdio. Antes de v2.1.203, los servidores stdio estaban exentos del tiempo de espera de inactividad.

Establezca la variable de entorno [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/es/env-vars) en milisegundos para cambiar la ventana de inactividad, o establézcala en `0` para deshabilitar la verificación.

Estos tiempos de espera limitan cuánto tiempo puede ejecutarse una llamada, no siempre cuánto tiempo bloquea la sesión: una llamada de conversación principal que se ejecuta más de dos minutos se mueve a una tarea de fondo primero. Consulte [Envío automático a segundo plano de llamadas de herramientas largas](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Envío automático a segundo plano de llamadas de herramientas largas
</h3>

Una llamada de herramienta MCP en la conversación principal que aún se está ejecutando después de dos minutos se mueve a una tarea de fondo en lugar de bloquear la sesión. Claude recibe el ID de tarea inmediatamente y continúa trabajando, y el resultado llega como una notificación de tarea cuando la llamada se resuelve. El envío automático a segundo plano requiere Claude Code v2.1.212 o posterior.

La tarea aparece en [`/tasks`](/docs/es/commands#all-commands), donde también puede detenerla, y no sobrevive a la salida de la sesión. Los límites por llamada aún se aplican mientras la llamada se ejecuta en segundo plano: el límite de reloj de pared establecido por el `timeout` por servidor o [`MCP_TOOL_TIMEOUT`](/docs/es/env-vars), y el tiempo de espera de inactividad establecido por [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/es/env-vars).

Establezca la variable de entorno [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/es/env-vars) en milisegundos para cambiar el umbral, o establézcala en `0` para desactivar el envío automático a segundo plano. Establecer `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` en `1` también lo desactiva, junto con todas las otras características de tareas de fondo.

Algunas llamadas nunca se mueven al fondo:

* Llamadas de [subagentes](/docs/es/sub-agents); Claude Code solo envía a segundo plano llamadas de conversación principal
* Llamadas a servidores IDE
* Llamadas en [modo no interactivo](/docs/es/headless), a menos que `CLAUDE_AUTO_BACKGROUND_TASKS` esté establecido en `1`, ya que una ejecución única puede terminar antes de que llegue el resultado

Una llamada esperando un [diálogo de elicitación](#respond-to-mcp-elicitation-requests) abierto no se envía a segundo plano mientras el diálogo está abierto; el servidor está bloqueado en su entrada, no es lento, por lo que Claude Code difiere el movimiento hasta que el diálogo se cierre.

<h3 id="plugin-provided-mcp-servers">
  Servidores MCP proporcionados por plugins
</h3>

[Los plugins](/docs/es/plugins/overview) pueden agrupar servidores MCP que proporcionan herramientas e integraciones cuando habilita el plugin. Los servidores MCP de plugins funcionan de manera idéntica a los servidores configurados por el usuario.

**Cómo funcionan los servidores MCP de plugins**:

* Los plugins definen servidores MCP en `.mcp.json` en la raíz del plugin o en línea en `plugin.json`
* Cuando habilita un plugin, Claude Code inicia sus servidores MCP automáticamente
* Claude Code ofrece herramientas MCP de plugins junto con herramientas MCP configuradas manualmente
* Agrega y elimina servidores de plugins instalando o desinstalando el plugin, no con comandos `/mcp`. Aún puede [alternar un servidor de plugin instalado](#disable-a-server-without-removing-it) en `/mcp`, que detiene que Claude Code se conecte a él sin eliminar el plugin

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

**Características MCP de plugins**:

* **Ciclo de vida automático**: los servidores se conectan y desconectan en estos puntos:
  * Al inicio de la sesión, Claude Code conecta automáticamente los servidores de plugins habilitados. En `/mcp`, un servidor de plugin remoto (HTTP o SSE) que ha utilizado antes puede mostrar el estado [`cached`](#server-status-detail) en su lugar; Claude Code lo conecta cuando Claude llama por primera vez a una de sus herramientas
  * Si habilita o deshabilita un plugin durante una sesión, Claude Code conecta o desconecta sus servidores MCP cuando el cambio se aplica. [Aplicar cambios de plugins sin reiniciar](/docs/es/plugins/cli-reference#reload-plugins) describe cuándo es eso. En una sesión sin terminal interactiva, `/reload-plugins` no conecta o desconecta servidores MCP de plugins; esos cambios tienen efecto en su próxima sesión
  * Cuando recarga, Claude Code mantiene las conexiones activas de servidores de plugins cuya configuración no ha cambiado, y hace lo mismo cuando [reemplaza la lista de servidores MCP de la sesión](/docs/es/agent-sdk/typescript#mcpsetserversresult) desde el Agent SDK sin nombrarlos
  * Cuando [mueve la sesión con `/cd`](/docs/es/permissions#move-the-session-to-another-directory) en v2.1.246 o posterior, Claude Code conecta los servidores de plugins que la configuración del nuevo directorio habilita y desconecta los servidores de plugins que ya no están habilitados, por lo que no necesita ejecutar `/reload-plugins` después del movimiento
  * En [sesiones en la nube](/docs/es/claude-code-on-the-web), una llamada MCP a un servidor de plugin que aún no está conectado, como justo después de que una sesión inactiva se despierte, inicia el servidor bajo demanda y espera a que se conecte
* **Marcadores de posición de ruta**: `${CLAUDE_PLUGIN_ROOT}` se resuelve al directorio de instalación del plugin, `${CLAUDE_PLUGIN_DATA}` a su directorio de [estado persistente](/docs/es/plugins/components#path-variables-and-persistent-data), y `${CLAUDE_PROJECT_DIR}` a la raíz del proyecto estable. La sustitución se aplica a:
  * servidores `stdio`: `command`, `args`, `env`
  * servidores `http`, `sse`, y `ws`: `url`, `headers`, y `headersHelper`. Antes de v2.1.195, `headersHelper` pasaba el marcador de posición como una cadena literal
* **Acceso al entorno del usuario**: acceso a las mismas variables de entorno que los servidores configurados manualmente
* **Múltiples tipos de transporte**: soporte para transportes stdio, SSE, HTTP, y WebSocket, aunque el soporte de transporte puede variar según el servidor

Los servidores de plugins aparecen en `/mcp` con indicadores que muestran que provienen de plugins.

**Nombres de herramientas MCP de plugins**:

Las herramientas de un servidor MCP agrupado en un plugin incluyen tanto el nombre del plugin como la clave del servidor en su nombre invocable. La forma completa es `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, donde cualquier carácter fuera de `A-Z`, `a-z`, `0-9`, `_`, y `-` se reemplaza con `_`. Para el servidor `database-tools` agrupado en un plugin llamado `my-plugin`, una herramienta `query` es invocable como:

```
mcp__plugin_my-plugin_database-tools__query
```

Utilice este nombre completo cuando haga referencia a la herramienta en [reglas de permisos](/docs/es/permissions), la lista `allowed-tools` de una habilidad, el [campo `tools` de un subagente](/docs/es/sub-agents#available-tools), o un [coincidente de hook](/docs/es/hooks#match-mcp-tools). Un coincidente de hook escrito contra la clave del servidor desnuda, como `mcp__database-tools__.*`, nunca se dispara para un servidor agrupado en un plugin.

El servidor mismo se registra bajo el nombre con alcance `plugin:<plugin-name>:<server-name>`, como `plugin:my-plugin:database-tools`. Utilice ese nombre donde se espera un nombre de servidor configurado, como el [campo `server` de un hook `mcp_tool`](/docs/es/hooks#mcp-tool-hook-fields).

Consulte la [referencia de componentes de plugins](/docs/es/plugins/components#mcp-servers) para obtener detalles sobre cómo agrupar servidores MCP con plugins.

<h2 id="mcp-installation-scopes">
  Alcances de instalación de MCP
</h2>

Los servidores MCP se pueden configurar en tres alcances. El alcance que elija controla en qué proyectos se carga el servidor y si la configuración se comparte con su equipo. Los administradores también pueden implementar o proporcionar servidores para cada usuario a través de [configuración administrada](#managed-mcp-configuration).

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
  El término "alcance local" para servidores MCP difiere de la configuración local general. Los servidores MCP con alcance local se almacenan en `~/.claude.json` (su directorio de inicio), mientras que la configuración local general usa `.claude/settings.local.json` (en el directorio del proyecto). Vea [Configuración](/docs/es/settings#where-settings-live) para detalles sobre ubicaciones de archivos de configuración.
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

Los servidores con alcance de proyecto habilitan la colaboración en equipo al almacenar configuraciones en un archivo `.mcp.json` en el directorio raíz de su proyecto. Cuando agrega un servidor con alcance de proyecto, Claude Code crea o actualiza automáticamente este archivo con la estructura de configuración apropiada. Verifique `.mcp.json` en el control de versiones para que todos en su equipo obtengan las mismas herramientas y servicios MCP.

```bash theme={null}
# Agregar un servidor con alcance de proyecto
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

El archivo `.mcp.json` resultante sigue un formato estandarizado:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

Por razones de seguridad, Claude Code solicita aprobación en sesiones interactivas antes de usar servidores con alcance de proyecto desde archivos `.mcp.json`. Para restablecer esas opciones de aprobación, ejecute `claude mcp reset-project-choices`.

En ejecuciones de `claude -p`, sesiones de [Agent SDK](/docs/es/headless) y [sesiones en la nube](/docs/es/claude-code-on-the-web), Claude Code no puede mostrar ese mensaje: carga servidores con alcance de proyecto sin preguntar. Claude Code también omite el mensaje en una sesión que inicia en modo `bypassPermissions` con [`skipDangerousModePermissionPrompt`](/docs/es/settings-reference#skipdangerousmodepermissionprompt) establecido en su configuración de usuario o en configuración administrada. Para mantener un servidor fuera de todas formas:

* Agréguelo a [`disabledMcpjsonServers`](/docs/es/settings-reference#disabledmcpjsonservers), que lo bloquea en cada modo de permiso.
* Excluya la configuración del proyecto completamente con [`--setting-sources`](/docs/es/cli-reference#cli-flags) o la opción `settingSources` del SDK.
* Inicie la sesión con [`--strict-mcp-config`](/docs/es/cli-reference#cli-flags). Claude Code entonces usa solo los servidores MCP que pasa con `--mcp-config`. Omitir el mensaje de aprobación para los servidores con alcance de proyecto que Claude Code no está cargando requiere Claude Code v2.1.246 o posterior; antes de v2.1.246, una sesión estricta aún esperaba aprobación para ellos, lo que dejaba las sesiones en segundo plano esperando al inicio. Vea [Control exclusivo con managed-mcp.json](/docs/es/managed-mcp#exclusive-control-with-managed-mcp-json) para lo que hace la bandera bajo un archivo MCP administrado.

[Aprobaciones de servidor de proyecto y confianza del espacio de trabajo](#project-server-approvals-and-workspace-trust) cubre cómo las aprobaciones confirmadas en el repositorio interactúan con la confianza del espacio de trabajo.

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
4. [Servidores proporcionados por plugins](/docs/es/plugins/components#mcp-servers)
5. [Conectores de claude.ai](#use-mcp-servers-from-claude-ai)

Los tres alcances coinciden duplicados por nombre. Los plugins y conectores coinciden por punto final, por lo que uno que apunta a la misma URL o comando que un servidor anterior se trata como un duplicado.

Un servidor que su organización proporciona a través de la configuración administrada [`managedMcpServers`](/docs/es/managed-mcp#provide-servers-through-managed-settings) se clasifica por encima de todos estos, por lo que cuando uno de ellos lo duplica, Claude Code conecta la definición de la organización. Requiere Claude Code v2.1.259 o posterior.

Si abre una sesión local en la [pestaña Code de la aplicación de escritorio](/docs/es/desktop#mcp-servers-from-the-claude-desktop-chat-app) con el mismo nombre de servidor stdio en el nivel superior de `~/.claude.json` (alcance de usuario) y en `.mcp.json`, la pestaña Code usa la definición de `~/.claude.json`.

<h3 id="environment-variable-expansion-in-mcp-json">
  Expansión de variables de entorno en `.mcp.json`
</h3>

Claude Code admite la expansión de variables de entorno en archivos `.mcp.json`, permitiendo que los equipos compartan configuraciones mientras mantienen flexibilidad para rutas específicas de máquinas y valores sensibles como claves API.

<h4 id="supported-syntax">
  Sintaxis soportada
</h4>

* `${VAR}`: se expande al valor de la variable de entorno `VAR`
* `${VAR:-default}`: se expande a `VAR` si está establecida, de lo contrario usa `default`

<h4 id="expansion-locations">
  Ubicaciones de expansión
</h4>

Las variables de entorno se pueden expandir en:

* `command`: la ruta del ejecutable del servidor
* `args`: argumentos de línea de comandos
* `env`: variables de entorno pasadas al servidor
* `url`: para tipos de servidor HTTP
* `headers`: para autenticación de servidor HTTP

<h4 id="example-with-variable-expansion">
  Ejemplo con expansión de variables
</h4>

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

<h4 id="unset-variables-without-a-default">
  Variables de entorno no establecidas sin un valor predeterminado
</h4>

Si una variable de entorno referenciada no está establecida y no tiene un valor predeterminado, la configuración aún se carga: Claude Code informa una advertencia de variable faltante para ese servidor en la salida de `claude mcp list` y usa el texto `${VAR}` sin expandir tal como está. Establezca la variable o agregue un fallback `:-default` para que el servidor se inicie con el valor que pretende. En la `url` y `headers` de un servidor remoto, algunas variables de credenciales [se leen como vacías](#credential-variables-that-read-as-empty) en su lugar, sin advertencia.

<h4 id="credential-variables-that-read-as-empty">
  Variables de credenciales que se leen como vacías
</h4>

En la `url` y `headers` de un servidor remoto, Claude Code lee las variables de credenciales de su entorno como vacías en lugar de expandirlas. Esto evita que la `.mcp.json` de un proyecto o un plugin envíe sus credenciales de Claude Code o proveedor de nube a un servidor que nombra. Si escribe `Bearer ${ANTHROPIC_AUTH_TOKEN}`, el servidor recibe `Bearer ` sin credencial y rechaza la solicitud, generalmente con un `401`. Claude Code informa eso como una conexión fallida.

Los nombres cubiertos son:

* Las propias credenciales de Claude Code, como `ANTHROPIC_API_KEY` y `ANTHROPIC_AUTH_TOKEN`
* Las credenciales de su proveedor de nube, como `AWS_BEARER_TOKEN_BEDROCK`
* Otras credenciales que su entorno lleva, como `HTTPS_PROXY` y `NPM_TOKEN`

Un nombre cubierto se lee como vacío independientemente de si ha establecido la variable, y un fallback `:-default` en él se ignora. Una URL base del proveedor como `ANTHROPIC_BASE_URL` aún se expande, por lo que `"url": "${ANTHROPIC_BASE_URL}/mcp"` funciona, a menos que el valor de la URL en sí incruste una credencial como un nombre de usuario y contraseña.

Un nombre fuera de este conjunto, como `API_KEY`, se expande tal como está escrito. Para dar al servidor una de las credenciales cubiertas, cópiela en una variable con un nombre de su propia elección y haga referencia a ese nombre en su lugar.

Cuando la `url` o `headers` de un servidor remoto hace referencia a una variable cubierta que ha establecido, Claude Code la nombra en una línea de registro de depuración. Para leer la línea, ejecute `claude --debug-file /tmp/claude-debug.log` y busque en ese archivo `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Cómo aparecen las referencias en `/mcp` y salida de CLI
</h4>

Para un servidor en el [alcance](#mcp-installation-scopes) local, de proyecto o de usuario, las siguientes superficies muestran una referencia `${VAR}` por nombre en lugar de como su valor resuelto:

* La URL o línea de comandos en la vista de detalle `/mcp` de un servidor
* Salida de `claude mcp list` y `claude mcp get`

La vista de detalle `/mcp` muestra referencias de esta manera en Claude Code v2.1.268 o posterior.

Para un servidor que su organización proporciona a través de la configuración `managedMcpServers`, estas superficies muestran [solo el host de la URL](/docs/es/managed-mcp#what-users-can-see-and-change).

Para verificar qué muestran `claude mcp list`, `claude mcp get` e `/mcp` cuando una conexión falla, vea [Detalle de estado del servidor](#server-status-detail).

<h2 id="practical-examples">
  Ejemplos prácticos
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Ejemplo: Conectar a GitHub para revisiones de código
</h3>

El servidor MCP remoto de GitHub se autentica con un token de acceso personal de GitHub pasado como encabezado. Para obtener uno, abra su [configuración de token de GitHub](https://github.com/settings/personal-access-tokens), genere un nuevo token de grano fino con acceso a los repositorios con los que desea que Claude trabaje, luego agregue el servidor:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Reemplace `YOUR_GITHUB_PAT` con su token de acceso personal. El comando `claude mcp add` guarda la configuración sin validar credenciales, por lo que se acepta un valor de marcador de posición aquí pero el servidor no se conecta más tarde. Para verificar la conexión, ejecute `/mcp` y compruebe que el servidor muestre `connected`. Un servidor con credenciales incorrectas muestra `failed`, y el detalle de fallo incluye el estado HTTP que devolvió el servidor, como un 401.

Luego trabaje con GitHub:

```text wrap theme={null}
Revise el PR #456 y sugiera mejoras
```

```text wrap theme={null}
Cree un nuevo problema para el error que acabamos de encontrar
```

```text wrap theme={null}
Muéstrame todos los PR abiertos asignados a mí
```

<h3 id="example-query-your-postgresql-database">
  Ejemplo: Consultar su base de datos PostgreSQL
</h3>

[DBHub](https://github.com/bytebase/dbhub), el paquete `@bytebase/dbhub`, es un servidor MCP que conecta Claude a una base de datos relacional a través de la cadena de conexión que pasa en `--dsn`. Use un usuario de base de datos de solo lectura en la cadena de conexión para que las consultas que ejecuta Claude no puedan modificar datos:

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Para confirmar que el servidor se inicia, ejecute `/mcp` y compruebe que `db` muestre `connected`.

Luego consulte su base de datos de forma natural:

```text wrap theme={null}
¿Cuál es nuestro ingreso total este mes?
```

```text wrap theme={null}
Muéstrame el esquema para la tabla de pedidos
```

```text wrap theme={null}
Encuentre clientes que no han realizado una compra en 90 días
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autenticarse con servidores MCP remotos
</h2>

Muchos servidores MCP basados en la nube requieren autenticación. Claude Code admite OAuth 2.0 para conexiones seguras.

Claude Code marca un servidor remoto como que requiere autenticación cuando el servidor responde con `401 Unauthorized` o `403 Forbidden`. Lo que Claude Code muestra depende del servidor:

* Para un servidor en el que no ha iniciado sesión, cualquiera de estos códigos de estado lo marca en `/mcp` para que pueda completar el flujo de OAuth.
* Para un [conector de claude.ai](#use-mcp-servers-from-claude-ai), un `401` causado por claude.ai rechazando su token de sesión no marca el conector, porque reautorizar el conector no puede solucionar su inicio de sesión. Claude Code muestra el [estado de token de sesión rechazado](/docs/es/errors#claude-ai-rejected-the-session-token) en su lugar.
* Para un servidor cuyo encabezado `Authorization` configuró, en `headers` o a través de un [`headersHelper`](#use-dynamic-headers-for-custom-authentication), un `401` o `403` mientras se conecta no marca el servidor, porque la credencial a corregir es la que configuró. Claude Code reporta la conexión como fallida en su lugar. Si establece ese encabezado desde una referencia `${VAR}`, verifique si esa variable es una que Claude Code [lee como vacía](#credential-variables-that-read-as-empty).
* Para un conector [entregado a una sesión en la nube](#how-connectors-reach-claude-code), Claude Code no ejecuta un flujo de inicio de sesión, porque el proxy de la sesión se autentica con el conector con la autorización que otorgó en claude.ai. Cuando un conector allí necesita autorización nuevamente, reconéctelo en [claude.ai/customize/connectors](https://claude.ai/customize/connectors) en lugar de desde la sesión.

Cuando una solicitud a un servidor OAuth en el que ya ha iniciado sesión devuelve `401 Unauthorized`, Claude Code actualiza el token almacenado, se reconecta e intenta la solicitud una vez más. Solo marca el servidor en `/mcp` si ese reintento también falla. Antes de v2.1.206, una actualización de token que falló por una razón transitoria, como un error de red, marcaba un servidor OAuth como que necesitaba autenticación durante el resto de la sesión aunque su token de actualización seguía siendo válido.

Cuando el servidor rechaza el token de actualización almacenado, Claude Code muestra inmediatamente un aviso que apunta a `/mcp`. Abra `/mcp` y seleccione **Re-authenticate** en el servidor para iniciar sesión nuevamente antes de que la siguiente llamada de herramienta falle.

Un servidor personalizado que devuelve un encabezado `WWW-Authenticate` que apunta a su servidor de autorización obtiene el mismo descubrimiento automático que cualquier otro servidor remoto.

Claude Code también muestra un aviso de inicio cuando uno o más servidores configurados necesitan autenticación, por lo que no tiene que abrir `/mcp` para descubrir qué servidores necesitan iniciar sesión. El aviso requiere Claude Code v2.1.193 o posterior. Solo cuenta servidores en los que puede iniciar sesión desde Claude Code. Antes de v2.1.218, también contaba [conectores de claude.ai](#use-mcp-servers-from-claude-ai) que no estaban conectados en claude.ai, que solo puede conectar desde la configuración de claude.ai.

El aviso anuncia cada servidor una vez y lo excluye del recuento en lanzamientos posteriores hasta que ese servidor se haya conectado y necesite iniciar sesión nuevamente. `/mcp` aún enumera todos los servidores que necesitan iniciar sesión.

En modo no interactivo no hay panel `/mcp`, por lo que Claude Code no puede ejecutar el flujo de OAuth para usted. A partir de v2.1.196, cuando un servidor configurado necesita autenticación durante una ejecución de `claude -p` o Agent SDK con [búsqueda de herramientas](#scale-with-mcp-tool-search) habilitada, que es la predeterminada, Claude Code le dice a Claude que las herramientas del servidor no están disponibles hasta que lo autorice. Claude puede entonces nombrar el servidor que necesita iniciar sesión en lugar de responder como si el servidor no estuviera configurado. Complete el inicio de sesión desde una sesión interactiva con `/mcp` o `claude mcp login <name>`.

Si configuró `headers.Authorization` para el servidor y el servidor rechaza ese encabezado, Claude Code reporta la conexión como fallida en lugar de recurrir a OAuth. Verifique que el token sea válido para el punto final de MCP, o elimine el encabezado para usar el flujo de OAuth.

<Steps>
  <Step title="Agregar el servidor que requiere autenticación">
    Si ya agregó el servidor `sentry` en el [inicio rápido de MCP](/docs/es/mcp-quickstart#connect-a-server-that-requires-sign-in), omita este paso: ejecutar `claude mcp add` nuevamente con el mismo nombre de servidor en el mismo alcance falla con `MCP server sentry already exists in local config`. De lo contrario, ejecute:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Use el comando /mcp dentro de Claude Code">
    En Claude Code, use el comando:

    ```text wrap theme={null}
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

El comando `claude mcp login <name>` ejecuta el flujo de OAuth de un servidor configurado directamente desde su shell, por lo que no necesita abrir el panel `/mcp` dentro de una sesión.

```bash theme={null}
claude mcp login sentry
```

Para borrar las credenciales almacenadas más tarde, ejecute `claude mcp logout <name>`.

`claude mcp login` detecta cuando no hay navegador local disponible, como durante una sesión SSH o en Linux sin un servidor de pantalla, e imprime la URL de autorización en lugar de intentar abrir un navegador. Abra la URL en su máquina local, luego pegue la URL de redireccionamiento completa de la barra de direcciones de su navegador nuevamente en el indicador. El comando necesita una terminal interactiva para el paso de pegado, así que conéctese con `ssh -t`. Pase `--no-browser` para forzar el indicador de URL incluso cuando se detecta un navegador local.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Usar un puerto de devolución de llamada OAuth fijo
</h3>

Algunos servidores MCP requieren un URI de redireccionamiento específico registrado de antemano. De forma predeterminada, Claude Code elige un puerto disponible aleatorio para la devolución de llamada de OAuth. Use `--callback-port` para fijar el puerto de modo que coincida con un URI de redireccionamiento preregistrado de la forma `http://localhost:PORT/callback`. Si el inicio de sesión en Claude Code v2.1.229 falla con un error de desajuste de URI de redireccionamiento, consulte la nota de versión en [Usar credenciales OAuth preconfiguradas](#use-pre-configured-oauth-credentials).

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

    En v2.1.229, Claude Code envió `http://127.0.0.1:PORT/callback` en su lugar, y los servidores que coincidían exactamente con el URI de redireccionamiento registrado rechazaban el inicio de sesión con un error de desajuste de URI de redireccionamiento. Claude Code v2.1.231 restauró la forma `localhost`. Para recuperarse en v2.1.229, actualice Claude Code, o agregue temporalmente la forma `http://127.0.0.1:PORT/callback` a los URI de redireccionamiento registrados del servidor.
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
  * Puede establecer el secreto del cliente solo cuando agrega el servidor. Cuando se autentica con `claude mcp login` o desde `/mcp`, Claude Code usa el secreto almacenado y no solicita uno ni lee `MCP_CLIENT_SECRET`
  * Para agregar o cambiar el secreto más tarde, elimine el servidor con `claude mcp remove <name>`, luego agréguelo nuevamente con `--client-secret` y el mismo `--scope`
  * Si el servidor usa un cliente OAuth público sin secreto, use solo `--client-id` sin `--client-secret`
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

Si el servidor luego devuelve un 403 `insufficient_scope` para una llamada de herramienta, la llamada falla con un mensaje de [`necesita permisos adicionales`](/docs/es/errors#mcp-server-needs-you-to-sign-in-again) que nombra el alcance que el servidor solicita. El servidor se muestra como que necesita autenticación en `/mcp`.

Si ese alcance no está en su `oauth.scopes` fijado, agréguelo, luego ejecute `/mcp` y autentique el servidor nuevamente. Claude Code solicita los alcances fijados en lugar del alcance que el servidor nombró, por lo que si se autentica nuevamente sin agregarlo, el token que obtiene aún carece de él.

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
* Claude Code ejecuta el comando en un shell y se rinde después de 10 segundos
* Claude Code elige el directorio de trabajo del comando por [dónde configuró el servidor](#where-the-helper-runs), así que proporcione el script como una ruta absoluta o colóquelo en `PATH`
* Los encabezados dinámicos anulan cualquier `headers` estático con el mismo nombre

Claude Code ejecuta el ayudante nuevamente en cada conexión, al iniciar la sesión y al reconectar, una vez que la [regla de confianza para servidores de alcance de proyecto y local](#trust-a-folder-before-its-headershelper-runs) le permite ejecutarse. No almacena en caché el resultado, por lo que su script es responsable de cualquier reutilización de tokens.

Si una llamada de herramienta devuelve `401 Unauthorized` o `403 Forbidden`, Claude Code automáticamente vuelve a ejecutar el ayudante bajo la misma regla, se reconecta con los encabezados frescos, e intenta la llamada una vez más. Claude Code marca el servidor como que necesita autenticación en `/mcp` solo si ese reintento también falla.

Cuando la salida del ayudante incluye un encabezado `Authorization`, Claude Code usa esa credencial como la autenticación del servidor y no recurre a OAuth para el servidor.

Si el servidor rechaza la credencial del ayudante mientras se conecta, Claude Code reporta la conexión como fallida en lugar de marcar el servidor como que necesita autenticación. Corrija la credencial que devuelve su ayudante, luego reconéctese desde `/mcp` para volver a ejecutar el ayudante.

Claude Code establece estas variables de entorno al ejecutar el ayudante:

| Variable                      | Valor                                                                                                                           |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `CLAUDE_CODE_MCP_SERVER_NAME` | el nombre del servidor MCP                                                                                                      |
| `CLAUDE_CODE_MCP_SERVER_URL`  | la URL del servidor MCP                                                                                                         |
| `CLAUDE_PLUGIN_ROOT`          | el directorio raíz del plugin. Se establece solo cuando un [plugin](/docs/es/plugins/components#mcp-servers) proporciona el servidor |

Use estas para escribir un único script de ayudante que sirva múltiples servidores MCP.

Un `headersHelper` proporcionado por un plugin no puede hacer referencia a los valores [`${user_config.*}`](/docs/es/plugins/manifest-reference#user-configuration) del plugin, porque el comando se ejecuta a través de un shell. Claude Code reporta el servidor como mal configurado con un [error](/docs/es/errors#plugin-command-references-user-config) y no sustituye el valor. Ponga `${user_config.KEY}` en el campo `headers` del servidor en su lugar, que no se analiza como shell, o haga que el script de ayudante lea el valor de un archivo de configuración. Antes de v2.1.207, `headersHelper` sustituía valores `${user_config.*}`.

<h4 id="where-the-helper-runs">
  Dónde se ejecuta el ayudante
</h4>

Claude Code elige el directorio de trabajo del comando `headersHelper` de la configuración que declara el servidor. Un `cd` que Claude ejecuta en Bash no lo mueve, y [`/cd`](/docs/es/permissions#move-the-session-to-another-directory) lo mueve solo para servidores que se ejecutan desde el directorio de trabajo principal de la sesión. Cada fila a continuación proporciona el directorio contra el cual se resuelve una ruta relativa en su comando `headersHelper`.

| Dónde configuró el servidor                                                                                                                                                                                                    | Directorio de trabajo                                                                                  |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| Un [plugin](/docs/es/plugins/components#mcp-servers)                                                                                                                                                                                | El directorio raíz del plugin. Requiere Claude Code v2.1.195 o posterior                               |
| Un `.mcp.json` de proyecto o un servidor de [alcance local](#local-scope)                                                                                                                                                      | El directorio del proyecto donde se declara el servidor                                                |
| Un archivo de agente en su proyecto, un servidor de la opción `mcpServers` del SDK o el método `setMcpServers()`, o [`--mcp-config`](/docs/es/cli-reference)                                                                        | El [directorio de trabajo principal](/docs/es/permissions#working-directories) de la sesión                 |
| [Alcance de usuario](#user-scope), [MCP administrado](/docs/es/managed-mcp), un [conector de claude.ai](#use-mcp-servers-from-claude-ai), o un archivo de agente de fuera de su proyecto, incluido uno de un directorio `--add-dir` | Su directorio de configuración, `~/.claude` a menos que establezca [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars) |

Antes de v2.1.238, Claude Code también ejecutaba los ayudantes de servidores de alcance de usuario, administrados y conectores de claude.ai, y de archivos de agente de fuera de su proyecto, desde el directorio desde el que lo inició.

<h4 id="which-variables-a-helper-can-read">
  Qué variables puede leer un ayudante
</h4>

Un `headersHelper` que un repositorio o plugin proporciona es un comando que no escribió, por lo que Claude Code lo ejecuta sin las variables de credencial de su entorno, como `ANTHROPIC_API_KEY`. Dónde configuró el servidor decide si esto se aplica:

* **Eliminadas**: un servidor en un `.mcp.json` de proyecto o en un plugin, y un servidor en línea en un archivo de agente de su proyecto o de un directorio `--add-dir`
* **No eliminadas**: un servidor en [alcance de usuario](#user-scope) o [alcance local](#local-scope), en [MCP administrado](/docs/es/managed-mcp), de un [conector de claude.ai](#use-mcp-servers-from-claude-ai), o proporcionado por el SDK o [`--mcp-config`](/docs/es/cli-reference), y un servidor en línea en un archivo de agente de `~/.claude/agents/`, de configuración administrada, o pasado con `--agents`

Aparte de las variables `GIT_CONFIG_KEY_<n>` de Git, Claude Code elimina cada variable de su entorno cuyo nombre se parece a una credencial, como un nombre con `TOKEN`, `SECRET`, `PASSWORD`, `KEY`, o `AUTH` en él en cualquier caso de letra, por lo que `ANTHROPIC_API_KEY` y `MY_REGISTRY_TOKEN` se eliminan. Claude Code también elimina una lista fija de variables de credencial cuyos nombres no siguen ese patrón, como `ANTHROPIC_CUSTOM_HEADERS`.

Cuando esto se aplica a su ayudante, haga que el script lea su credencial de un archivo o un almacén de credenciales. Si la `url` del servidor [expande una de estas variables](#environment-variable-expansion-in-mcp-json), el valor `CLAUDE_CODE_MCP_SERVER_URL` que recibe el ayudante tiene esa parte reemplazada con `REDACTED` también.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Confiar en una carpeta antes de que se ejecute su headersHelper
</h4>

Claude Code ejecuta un `headersHelper` como un comando de shell arbitrario. Para un servidor en un `.mcp.json` de proyecto o en [alcance local](#local-scope), lo ejecuta solo después de que acepte el [diálogo de confianza](/docs/es/permissions#project-allow-rules-and-workspace-trust) para el directorio del proyecto donde se declara el servidor. Antes de v2.1.238, una sesión `claude -p` o SDK ejecutaba estos ayudantes sin verificar confianza, y una sesión interactiva los ejecutaba una vez que había confiado en una carpeta principal.

* **Confianza que no cuenta**: la confianza de una carpeta principal, y la confianza automática que una sesión `claude -p` o SDK obtiene para [hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder)
* **Hasta que confíe en la carpeta**: Claude Code conecta el servidor solo con sus `headers` estáticos. En una sesión `claude -p` o SDK también imprime una línea [`headersHelper not run`](/docs/es/errors#headershelper-not-run) por servidor en stderr, diciéndole cómo otorgar la confianza.
* **Confianza sin un diálogo**: establezca `projects["<path>"].hasTrustDialogAccepted` en `true` en `~/.claude.json`. `<path>` es la carpeta en la que [Reglas de permiso de proyecto y confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) dice que Claude Code basa la confianza.

Claude Code aplica la misma regla a un servidor declarado en línea en un [archivo de agente](/docs/es/sub-agents#scope-mcp-servers-to-a-subagent), verificando de dónde vino ese archivo de agente: su proyecto, para un archivo en su directorio `.claude/agents/`, o un directorio `--add-dir`. Hasta que [confíe en ese proyecto o directorio mismo](/docs/es/permissions#what-runs-before-you-trust-a-folder), Claude Code no carga el servidor en absoluto, por lo que su ayudante nunca se ejecuta tampoco.

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

Si ha iniciado sesión en Claude Code con una cuenta de [claude.ai](https://claude.ai), los servidores MCP que ha añadido en claude.ai, conocidos como [conectores](https://claude.com/docs/connectors), están disponibles automáticamente en Claude Code:

<Steps>
  <Step title="Configurar servidores MCP en claude.ai">
    Añada servidores en [claude.ai/customize/connectors](https://claude.ai/customize/connectors). En planes Team y Enterprise, solo los administradores pueden añadir servidores.
  </Step>

  <Step title="Autenticar el servidor MCP">
    Complete los pasos de autenticación requeridos en claude.ai.
  </Step>

  <Step title="Ver y gestionar servidores en Claude Code">
    En Claude Code, use el comando:

    ```text wrap theme={null}
    /mcp
    ```

    Los servidores de claude.ai aparecen en la lista con indicadores que muestran que provienen de claude.ai.
  </Step>
</Steps>

Claude Code marca un conector como `managed` en `/mcp` y en el gestor de [`/plugin`](/docs/es/plugins/install) cuando su organización gestiona su autenticación en claude.ai. El estado managed no cambia cómo Claude Code se conecta al conector ni aplica los [controles de herramientas](#organization-controls-on-connector-tools) de su organización.

Los conectores a los que nunca ha iniciado sesión se contraen detrás de una fila `Show unused connectors` al final de la sección de claude.ai, por lo que una lista provisionada por la organización no llena el panel. Seleccione la fila para expandirlos. Un conector en el que inició sesión antes permanece visible incluso cuando actualmente necesita reautenticación.

Los conectores de claude.ai se obtienen solo cuando su [método de autenticación](/docs/es/authentication#authentication-precedence) activo es un inicio de sesión de suscripción de claude.ai. No se cargan, incluso si ejecutó `/login` anteriormente, cuando:

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper` está activo
* Un proveedor de terceros como Amazon Bedrock o Agent Platform de Google Cloud está activo
* `ANTHROPIC_PROFILE`, las variables de federación, o un [perfil de Anthropic](/docs/es/authentication#anthropic-profiles-and-federation-credentials) activo proporciona la credencial
* `CLAUDE_CODE_OAUTH_TOKEN` contiene un token de [`claude setup-token`](/docs/es/authentication#generate-a-long-lived-token), que solo puede hacer solicitudes de modelo

Si `/mcp` no enumera un conector que añadió, ejecute `/status` para confirmar qué método de autenticación está activo. Desactive esa variable de entorno, elimine la configuración `apiKeyHelper`, o [desactive el perfil](/docs/es/authentication#anthropic-profiles-and-federation-credentials), luego ejecute `/login` para seleccionar su cuenta de claude.ai.

Si un problema de red temporal mantiene la lista de conectores sin cargar cuando su sesión comienza, Claude Code reintenta la obtención hasta tres veces en segundo plano, y los conectores aparecen una vez que un reintento tiene éxito. Si aún no han aparecido, reinicie Claude Code para obtener la lista nuevamente.

Si `/mcp` muestra un conector como `connected · session token rejected`, o su vista de detalle muestra [`claude.ai rejected the session token`](/docs/es/errors#claude-ai-rejected-the-session-token), claude.ai rechazó el token de su inicio de sesión de Claude Code, generalmente porque el inicio de sesión expiró y no se pudo actualizar. Autorizar el conector nuevamente no borra este estado, porque la autorización propia del conector en claude.ai no es lo que fue rechazado. Para borrarlo:

1. Ejecute `/login` para iniciar sesión nuevamente.
2. Reconecte el conector desde `/mcp`.

Antes de v2.1.222, Claude Code marcaba los conectores como que necesitaban autenticación, y autorizarlos no lo resolvía.

Un servidor que ha añadido en Claude Code tiene [precedencia](#scope-hierarchy-and-precedence) sobre un conector de claude.ai que apunta a la misma URL. Cuando esto sucede, `/mcp` enumera el conector como oculto y muestra cómo eliminar el duplicado si prefiere usar el conector.

Algunos conectores alojados por Anthropic, como Microsoft 365, Gmail y Google Calendar, no admiten OAuth local desde Claude Code porque el proveedor de identidad ascendente solo acepta la URL de redirección que claude.ai registró. Cuando un servidor que añadió con `claude mcp add` o en `.mcp.json` apunta a uno de estos hosts e inicia sesión en él desde `/mcp` o con `claude mcp login`, Claude Code muestra [`is Anthropic-hosted and doesn't support local OAuth`](/docs/es/errors#anthropic-hosted-and-doesnt-support-local-oauth), dirigiéndole a conectar el servicio en [claude.ai/customize/connectors](https://claude.ai/customize/connectors) en su lugar.

Después de eliminar su entrada con `claude mcp remove <name>` y conectar el servicio en claude.ai, el conector aparece en Claude Code automáticamente.

<h3 id="how-connectors-reach-claude-code">
  Cómo los conectores llegan a Claude Code
</h3>

Qué configuración rige un conector de claude.ai depende de dónde se ejecute su sesión, porque solo algunas sesiones obtienen conectores de claude.ai por sí mismas. Cada fila a continuación nombra cómo los conectores llegan en un tipo de sesión y qué los controla allí. Las [sesiones WSL](/docs/es/desktop-wsl#what-works-in-a-wsl-session) de la aplicación de escritorio no tienen fila porque los conectores aún no están disponibles en ellas.

| Dónde se ejecuta la sesión                                                                                                  | Cómo llegan los conectores                         | Qué los rige                                                                                                                                                                                                                                                         |
| :-------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sesiones de Terminal, [VS Code](/docs/es/vs-code), [JetBrains](/docs/es/jetbrains), y [Agent SDK](/docs/es/agent-sdk/claude-code-features) | Claude Code los obtiene de claude.ai               | La configuración en esta sección y [configuración MCP gestionada](/docs/es/managed-mcp)                                                                                                                                                                                   |
| [Sesiones en la nube](/docs/es/claude-code-on-the-web)                                                                           | El host remoto los pasa                            | Su configuración de organización de claude.ai, más la configuración de [lista de permitidos y lista de denegados](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists) que llega a la sesión y cualquier `managed-mcp.json` en el host que la ejecuta |
| Las sesiones locales y SSH de la [aplicación de escritorio](/docs/es/desktop)                                                    | La aplicación de escritorio los entrega en proceso | Entradas `blocked` en los [controles de herramientas de conector](#organization-controls-on-connector-tools) de su organización                                                                                                                                      |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS`, y [`allowAllClaudeAiMcps`](/docs/es/settings-reference#allowallclaudeaimcps) actúan solo en la primera fila, los conectores que Claude Code obtiene por sí mismo. Las otras dos filas difieren de ella de estas maneras:

* **Sesiones en la nube**: Las entradas `allowedMcpServers` y `deniedMcpServers` que llegan a la sesión, por ejemplo a través de [configuración gestionada por servidor](/docs/es/server-managed-settings), también filtran los conectores entregados. El proxy de la sesión reescribe la URL de cada conector, por lo que un patrón `serverUrl` escrito para la URL propia del conector no coincide con él. Para admitir conectores entregados junto con una lista de permitidos de URL en un entorno autohospedado, añada las entradas `serverUrl` enumeradas en [El tráfico de conectores sale de su red](/docs/es/self-hosted-environments-deploy#connector-traffic-leaves-your-network). Claude Code descarta los conectores entregados cuando hay un `managed-mcp.json` presente en el host que ejecuta la sesión, como un [host de ejecutor autohospedado](/docs/es/self-hosted-environments-configuration#mcp-servers), independientemente de si establece `allowAllClaudeAiMcps`.
* **Sesiones locales y SSH de la aplicación de escritorio**: la aplicación de escritorio registra los conectores como servidores `type: "sdk"` en proceso, y ninguna configuración MCP o `managed-mcp.json` llega a ellos. Un usuario mantiene un conector fuera de sus propias sesiones desconectándolo en [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Una organización bloquea las [herramientas](#organization-controls-on-connector-tools) de un conector o desactiva [Claude Code en la aplicación de escritorio](/docs/es/desktop#admin-console-controls) completamente.

<h3 id="organization-controls-on-connector-tools">
  Controles de organización en herramientas de conector
</h3>

Su organización puede establecer controles por herramienta en [conectores de claude.ai](https://claude.com/docs/connectors). Claude Code lee esta configuración al iniciar y la aplica localmente, excepto en las [sesiones locales y SSH](#how-connectors-reach-claude-code) de la aplicación de escritorio. Allí, la aplicación de escritorio retiene las herramientas `blocked` antes de entregar un conector, y la configuración `ask` no llega a Claude Code, por lo que aplica las [reglas de permiso](/docs/es/permissions) ordinarias de la sesión a esas herramientas en lugar de solicitar en cada llamada. En sesiones donde Claude Code obtiene conectores por sí mismo, ejecute `/mcp` para ver qué configuración se aplica a cada herramienta en un conector.

* **Herramienta establecida en `ask`**: Claude Code solicita en cada llamada con la razón `Your organization requires approval for this tool`. La solicitud aparece incluso en los [modos de permiso](/docs/es/permissions#permission-modes) `acceptEdits`, `auto`, y `bypassPermissions`, y nunca ofrece una opción para recordar su elección. Las [reglas de permitir](/docs/es/permissions) que coinciden con la herramienta tampoco omiten la solicitud. En modo `dontAsk`, que nunca solicita, Claude Code niega la llamada en su lugar.
* **Herramienta establecida en `blocked`**: Claude Code filtra la herramienta antes de que Claude la vea, por lo que nunca aparece en la lista de herramientas. La aplicación de escritorio y el chat de claude.ai aplican la misma configuración `blocked`, por lo que Claude tampoco puede usar la herramienta allí, y no puede retener una herramienta de las sesiones de la aplicación de escritorio mientras la mantiene disponible en el chat. La aplicación de escritorio omite un conector cuyas herramientas están todas bloqueadas.

<h3 id="disable-claude-ai-connectors">
  Desactivar conectores de claude.ai
</h3>

Claude Code aplica [`disableClaudeAiConnectors`](/docs/es/settings-reference#disableclaudeaiconnectors) solo a los conectores que [obtiene por sí mismo](#how-connectors-reach-claude-code), no a los conectores que entrega un host en la nube o la aplicación de escritorio. Para desactivar los conectores que obtiene, establezca la configuración en `true` en cualquier ámbito de configuración:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Esta configuración utiliza semántica any-source-true: `true` en cualquier fuente de configuración tiene precedencia. Un `.claude/settings.json` de proyecto verificado puede optar por que un repositorio no use los conectores que Claude Code obtiene por sí mismo, pero un `false` a nivel de proyecto no puede reactivar los conectores que un `true` a nivel de usuario o política ha desactivado. Los servidores pasados explícitamente a través de `--mcp-config` no se ven afectados.

También puede establecer la variable de entorno `ENABLE_CLAUDEAI_MCP_SERVERS` en `false`, que tiene el mismo efecto para la sesión de shell actual:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Para bloquear conectores individuales de claude.ai en lugar de todos ellos, añádalos a [`deniedMcpServers`](/docs/es/managed-mcp) por nombre o por patrón de URL. Por ejemplo, una entrada `serverName` de `"claude.ai Slack"` bloquea el conector de Slack. También puede ejecutar `/mcp` para activar o desactivar cualquier conector que Claude Code obtiene solo para el proyecto actual.

<h2 id="use-claude-code-as-an-mcp-server">
  Usar Claude Code como servidor MCP
</h2>

Puede usar Claude Code como servidor MCP que otras aplicaciones pueden conectar:

```bash theme={null}
# Inicia Claude como servidor MCP stdio
claude mcp serve
```

El comando no imprime nada cuando se inicia. Un servidor MCP stdio se comunica a través de stdin y stdout, por lo que una terminal silenciosa y bloqueada significa que el servidor está ejecutándose y esperando que un cliente se conecte.

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
  **Configuración de la ruta del ejecutable**: el campo `command` debe hacer referencia al ejecutable de Claude Code. Si el comando `claude` no está en la ruta del sistema, deberá especificar la ruta completa al ejecutable.

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

  * En Claude Desktop, intente pedirle a Claude que lea archivos en un directorio, realice ediciones y más.
  * Este servidor MCP solo expone las herramientas de Claude Code a su cliente MCP, por lo que su propio cliente es responsable de implementar la confirmación del usuario para llamadas de herramientas individuales.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Límites de salida de MCP y advertencias
</h2>

Cuando las herramientas de MCP producen salidas grandes, Claude Code ayuda a gestionar el uso de tokens para evitar sobrecargar el contexto de su conversación:

* **Umbral de advertencia de salida**: Claude Code muestra una advertencia cuando la salida de cualquier herramienta de MCP excede 10,000 tokens
* **Límite configurable**: puede ajustar el máximo de tokens de salida de MCP permitidos usando la variable de entorno `MAX_MCP_OUTPUT_TOKENS`
* **Límite predeterminado**: el máximo predeterminado es 25,000 tokens
* **Alcance**: la variable de entorno se aplica a herramientas que no declaran su propio límite. Las herramientas que establecen [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) usan ese valor en su lugar para contenido de texto, independientemente de lo que `MAX_MCP_OUTPUT_TOKENS` esté configurado. Las herramientas que devuelven datos de imagen siguen estando sujetas a `MAX_MCP_OUTPUT_TOKENS`
* **Superando el límite**: cuando un resultado sin contenido de imagen excede el límite, Claude Code lo guarda en un archivo y lo reemplaza en la conversación con un mensaje que nombra la ruta del archivo, para que Claude lea el archivo cuando necesite el contenido. El archivo se encuentra en el directorio `tool-results` de la sesión bajo [`~/.claude/projects/`](/docs/es/claude-directory#cleaned-up-automatically).

Para aumentar el límite de herramientas que producen salidas grandes:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Aumentar el límite para una herramienta específica
</h3>

Si está creando un servidor de MCP, puede permitir que herramientas individuales devuelvan resultados más grandes que el umbral predeterminado de persistencia en disco estableciendo `_meta["anthropic/maxResultSizeChars"]` en la entrada de respuesta `tools/list` de la herramienta. Claude Code eleva el umbral de esa herramienta al valor anotado, hasta un límite máximo de 500,000 caracteres.

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

La anotación se aplica independientemente de `MAX_MCP_OUTPUT_TOKENS` para contenido de texto, por lo que los usuarios no necesitan elevar la variable de entorno para herramientas que la declaran. Las herramientas que devuelven datos de imagen siguen estando sujetas al límite de tokens.

<Warning>
  Si frecuentemente encuentra advertencias de salida con servidores de MCP específicos que no controla, considere aumentar el límite de `MAX_MCP_OUTPUT_TOKENS`. También puede pedirle al autor del servidor que agregue la anotación `anthropic/maxResultSizeChars` o que paginen sus respuestas. La anotación no tiene efecto en herramientas que devuelven contenido de imagen; para esas, aumentar `MAX_MCP_OUTPUT_TOKENS` es la única opción.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Esquemas de entrada de herramientas con un combinador a nivel raíz
</h2>

Algunos servidores MCP declaran el esquema de entrada de una herramienta como una unión de JSON Schema, con `anyOf`, `oneOf` o `allOf` en el nivel superior del esquema. La API de Claude no acepta esas palabras clave en la raíz del esquema. Sí acepta combinadores anidados dentro de `properties`, que Claude Code envía sin cambios.

Las herramientas con un combinador a nivel raíz permanecen disponibles. Antes de enviar la herramienta a la API, Claude Code aplana el esquema en un único objeto y antepone una oración a la descripción de la herramienta que le indica a Claude qué grupos de parámetros pertenecen juntos:

* `allOf`: las propiedades de cada rama se fusionan, y la lista `required` de cada rama sigue aplicándose
* `anyOf` y `oneOf`: las propiedades de cada rama se fusionan, y la lista `required` de cada rama se describe en la descripción de la herramienta en lugar de ser aplicada por el esquema

Su servidor recibe los argumentos que Claude eligió, así que continúe validando la combinación del lado del servidor.

Cuando Claude Code no puede producir un esquema que la API acepte, o en una implementación que no recibe la configuración remota que habilita la reescritura, omite esa herramienta, registra el motivo en el registro del servidor y deja disponibles las otras herramientas del servidor. Las versiones anteriores a v2.1.195 omiten todas las herramientas cuyo esquema de entrada tiene un `anyOf`, `oneOf` o `allOf` a nivel raíz.

<h2 id="tools-with-invalid-input-schemas">
  Herramientas con esquemas de entrada inválidos
</h2>

La API de Claude verifica el esquema de entrada de cada herramienta en una solicitud y rechaza toda la solicitud cuando algún esquema falla, por lo que una única herramienta MCP con un esquema malformado haría que todas las solicitudes que la incluyan fallen con un error 400. Claude Code ejecuta dos de las comprobaciones de la API por sí mismo cuando carga las herramientas de un servidor y excluye cada herramienta que fallaría en ellas, de modo que las otras herramientas del servidor sigan funcionando:

* Los nombres de propiedades de nivel superior deben tener entre 1 y 64 caracteres de largo y usar solo letras ASCII y dígitos, `_`, `.` y `-`
* El esquema debe ser válido contra el meta-esquema de JSON Schema draft 2020-12. Claude Code aplica esta comprobación a esquemas que no declaran `$schema` y esquemas que declaran draft 2020-12. Un esquema que declara cualquier otro dialecto omite esta comprobación, aunque la comprobación de nombres de propiedades anterior sigue aplicándose

Claude Code ejecuta las comprobaciones después de la [reescritura de combinador de nivel raíz](#tool-input-schemas-with-a-root-level-combinator), en el esquema que realmente enviaría.

Cuando Claude Code excluye una herramienta, registra el motivo en el registro del servidor e indica a Claude qué herramientas excluyó y por qué, para que pueda preguntarle a Claude por qué falta una herramienta. Si corrige el esquema en el servidor, la herramienta reaparece la próxima vez que Claude Code carga las herramientas del servidor.

Claude Code activa la exclusión a través de una bandera de características que obtiene de Anthropic. En una [implementación donde la obtención de banderas está desactivada](/docs/es/env-vars#features-that-need-feature-flag-fetching), o en una máquina cuyas banderas nunca han llegado, como una máquina aislada, Claude Code sigue ejecutando las comprobaciones y registra en el registro del servidor qué herramienta sería rechazada, pero envía el esquema de la herramienta a la API de todas formas. La API rechaza una solicitud que incluya ese esquema con [un error 400 que nombra la herramienta por su posición](/docs/es/errors#tool-input-schema-is-invalid). Antes de v2.1.216, ninguna implementación ejecutaba estas comprobaciones.

El [manejo de combinador de nivel raíz](#tool-input-schemas-with-a-root-level-combinator) es independiente y mantiene su propio comportamiento cuando la obtención de banderas está desactivada o las banderas nunca han llegado.

<h2 id="require-approval-for-a-specific-tool">
  Requerir aprobación para una herramienta específica
</h2>

Si está construyendo un servidor MCP, puede marcar una herramienta como que requiere aprobación explícita en cada llamada estableciendo `_meta["anthropic/requiresUserInteraction"]` en `true` en la entrada de respuesta `tools/list` de la herramienta. El valor debe ser el booleano JSON `true`; cualquier otro valor se ignora.

Claude Code muestra el aviso de permiso de esa herramienta en cada llamada, incluso en los [modos de permiso](/docs/es/permissions#permission-modes) `acceptEdits`, `auto` y `bypassPermissions`, y no ofrece una opción "no volver a preguntar" para ella. Las [reglas de permiso](/docs/es/permissions#permission-rule-syntax) que coinciden con la herramienta tampoco omiten el aviso. En modo `dontAsk`, que nunca solicita confirmación, Claude Code deniega la llamada en su lugar.

El aviso tiene que llegar a una persona. En modo no interactivo con [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags), un resultado `allow` de la herramienta de aviso de permiso para una herramienta marcada se convierte en una denegación con el mensaje `MCP tool requires user interaction; not supported via --permission-prompt-tool`. La devolución de llamada [`canUseTool`](/docs/es/agent-sdk/permissions) del Agent SDK sí recibe estas llamadas y puede aprobarlas, porque se espera que su aplicación SDK muestre estas llamadas a un usuario.

Utilice esto para herramientas cuyo aviso de permiso es en sí el punto, como un paso de consentimiento o concesión de acceso donde la aprobación automática significaría que ningún humano nunca estuvo de acuerdo. Otras herramientas del mismo servidor mantienen su comportamiento de permiso normal.

La siguiente entrada `tools/list` marca una herramienta como que siempre requiere aprobación.

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

Algunas superficies, como [Remote Control](/docs/es/remote-control) y aplicaciones construidas en el [Agent SDK](/docs/es/agent-sdk/overview), normalmente le permiten aprobar llamadas de herramientas con un toque. Para una herramienta marcada con esta anotación, Claude Code retiene la acción de un toque y muestra el aviso de permiso completo de la herramienta en su lugar, por lo que la aprobación sigue viniendo de una persona respondiendo al aviso en lugar de un toque.

Claude Code retiene la aprobación de un toque de la misma manera para cualquier solicitud de permiso que solo el diálogo de terminal pueda renderizar completamente, como una que lleva una advertencia de seguridad u una opción de permitir siempre que la superficie remota no puede mostrar. Usted responde esa solicitud en el diálogo de terminal en lugar de desde Remote Control. Requiere Claude Code v2.1.214 o posterior.

<h2 id="respond-to-mcp-elicitation-requests">
  Responder a solicitudes de elicitación de MCP
</h2>

Los servidores MCP pueden solicitar información estructurada de su parte durante una tarea mediante elicitación. Cuando un servidor necesita información que no puede obtener por sí solo, Claude Code muestra un diálogo interactivo y devuelve su respuesta al servidor. No se requiere configuración de su parte: los diálogos de elicitación aparecen automáticamente cuando un servidor los solicita.

Los servidores pueden solicitar información de dos formas:

* **Modo de formulario**: Claude Code muestra un diálogo con campos de formulario definidos por el servidor (por ejemplo, una solicitud de nombre de usuario y contraseña). Complete los campos y envíe.
* **Modo de URL**: Claude Code abre una URL del navegador para autenticación o aprobación. Complete el flujo en el navegador y luego confirme en la CLI.

En modo de URL, Claude Code pasa la URL como argumento de línea de comandos al controlador de URL de su sistema y limita la longitud de ese argumento. Cuando la URL, una vez escapada para la línea de comandos, supera ese límite, solo puede rechazar la solicitud. Cada carácter que necesita escaparse, como `%` o `&`, cuenta cuatro veces hacia el límite: su propio carácter más tres caracteres de escape. Una URL sin ninguno de ellos alcanza el límite en aproximadamente 8.000 caracteres. Una URL construida en gran medida con escapes de porcentaje, donde cada tercer carácter es un `%`, lo alcanza en aproximadamente 4.000.

Para responder automáticamente a solicitudes de elicitación sin mostrar un diálogo, use el hook [`Elicitation`](/docs/es/hooks#elicitation).

Si está creando un servidor MCP que utiliza elicitación, consulte la [especificación de elicitación de MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) para obtener detalles del protocolo y ejemplos de esquema.

<h2 id="use-mcp-resources">
  Usar recursos MCP
</h2>

Los servidores MCP pueden exponer recursos que puede referenciar usando menciones @, de manera similar a cómo referencia archivos.

<h3 id="reference-mcp-resources">
  Referenciar recursos MCP
</h3>

<Steps>
  <Step title="Listar recursos disponibles">
    Escriba `@` en su indicación para ver los recursos disponibles de todos los servidores MCP conectados. Los recursos aparecen junto a los archivos en el menú de autocompletado.
  </Step>

  <Step title="Referenciar un recurso específico">
    Utilice el formato `@server:protocol://resource/path` para referenciar un recurso:

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Referencias de múltiples recursos">
    Puede referenciar múltiples recursos en una sola indicación:

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Los recursos se obtienen automáticamente e incluyen como adjuntos cuando se referencian
  * Las rutas de recursos se pueden buscar de forma difusa en el autocompletado de menciones @
  * Claude Code proporciona automáticamente herramientas para listar y leer recursos MCP cuando los servidores las admiten
  * Los recursos pueden contener cualquier tipo de contenido que proporcione el servidor MCP (texto, JSON, datos estructurados, etc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Escalar con búsqueda de herramientas MCP
</h2>

La búsqueda de herramientas mantiene el uso de contexto MCP bajo al diferir las definiciones de herramientas hasta que Claude las necesita. Solo los nombres de herramientas e instrucciones del servidor se cargan al inicio de la sesión, por lo que agregar más servidores MCP tiene un impacto mínimo en su ventana de contexto. Claude Code no impone un límite fijo de herramientas por servidor; el límite práctico es su presupuesto de ventana de contexto.

<Note>
  La búsqueda de herramientas no es compatible con Microsoft Foundry [implementaciones alojadas en Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), que la rechazan del lado del servidor: Claude Code detecta el rechazo y carga las herramientas MCP por adelantado para esa implementación en su lugar. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) no puede anular esto, ya que el rechazo proviene de la implementación misma.
</Note>

<h3 id="for-mcp-server-authors">
  Para autores de servidores MCP
</h3>

Si está creando un servidor MCP, el campo de instrucciones del servidor se vuelve más útil con la búsqueda de herramientas habilitada. Las instrucciones del servidor ayudan a Claude a entender cuándo buscar sus herramientas, de manera similar a cómo funcionan las [skills](/docs/es/skills).

Agregue instrucciones de servidor claras y descriptivas que expliquen:

* Qué categoría de tareas manejan sus herramientas
* Cuándo Claude debe buscar sus herramientas
* Capacidades clave que proporciona su servidor

Claude Code trunca cada descripción de herramienta e instrucciones de cada servidor en 2.048 caracteres de forma predeterminada. Manténgalas concisas y coloque los detalles críticos cerca del inicio.

Para cambiar el límite para cada servidor MCP en su sesión, establezca [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/es/env-vars#variables) en un número de caracteres. Esta variable requiere Claude Code v2.1.280 o posterior.

<h3 id="configure-tool-search">
  Configurar búsqueda de herramientas
</h3>

La búsqueda de herramientas está habilitada de forma predeterminada: las herramientas MCP se difieren y se descubren bajo demanda. Claude Code la deshabilita cuando `ANTHROPIC_BASE_URL` apunta a un host que no es de primera parte, ya que la mayoría de los proxies no reenvían bloques `tool_reference`. Establezca `ENABLE_TOOL_SEARCH` explícitamente para anular ese comportamiento predeterminado.

Configurar [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/es/env-vars) mantiene la búsqueda de herramientas desactivada. No puede anularla configurando `ENABLE_TOOL_SEARCH` usted mismo. Su organización puede mantener la búsqueda de herramientas activada a través de [configuración administrada](/docs/es/managed-settings), en Claude Code v2.1.227 o posterior. [Deshabilitar capacidades de pre-lanzamiento](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities) cubre dónde se aplica la anulación y qué variable elimina.

La búsqueda de herramientas requiere un modelo que admita bloques `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 y modelos posteriores. Consulte [compatibilidad de modelos en la documentación de API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para la lista actual.

En Agent Platform de Google Cloud, Claude Code decide por generación de modelo:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 y posteriores**: la búsqueda de herramientas está activada de forma predeterminada, igual que en la API de Anthropic.
* **Modelos anteriores de Agent Platform**: Claude Code carga todas las herramientas MCP por adelantado, porque sus pilas de servicio rechazan el encabezado beta requerido. `ENABLE_TOOL_SEARCH=true` no anula esto.

Antes de v2.1.221, Claude Code deshabilitaba la búsqueda de herramientas para todos los modelos en Agent Platform de Google Cloud a menos que configurara `ENABLE_TOOL_SEARCH=true`.

Controle el comportamiento de búsqueda de herramientas con la variable de entorno `ENABLE_TOOL_SEARCH`:

| Valor            | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| (sin establecer) | Todas las herramientas MCP diferidas y cargadas bajo demanda. Se revierte a carga por adelantado en modelos de Agent Platform de Google Cloud anteriores a la generación Claude 4.5, cuando `ANTHROPIC_BASE_URL` es un host que no es de primera parte, o en una implementación de Microsoft Foundry alojada en Azure                                                                                                                                                           |
| `true`           | Todas las herramientas MCP diferidas, excepto en una implementación de Microsoft Foundry alojada en Azure, donde el rechazo del lado del servidor aún fuerza la carga por adelantado, y en modelos de Agent Platform de Google Cloud anteriores a la generación Claude 4.5, donde Claude Code sigue cargando herramientas por adelantado. Claude Code envía el encabezado beta a través de proxies, y las solicitudes fallan en proxies que no admiten bloques `tool_reference` |
| `auto`           | Modo de umbral: Claude Code carga las herramientas que de otro modo diferiría por adelantado mientras sus definiciones totalizan menos del 10% de la ventana de contexto, y difiere todas ellas una vez que las definiciones alcanzan el 10%                                                                                                                                                                                                                                    |
| `auto:N`         | Modo de umbral con un porcentaje personalizado, donde `N` es 0-100. Por ejemplo, `auto:5` para 5%                                                                                                                                                                                                                                                                                                                                                                               |
| `false`          | Todas las herramientas MCP cargadas por adelantado, sin diferimiento                                                                                                                                                                                                                                                                                                                                                                                                            |

```bash theme={null}
# Use a custom 5% threshold
ENABLE_TOOL_SEARCH=auto:5 claude

# Disable tool search entirely
ENABLE_TOOL_SEARCH=false claude
```

O establezca el valor en el campo `env` de su [settings.json](/docs/es/settings-reference#env).

También puede deshabilitar la herramienta `ToolSearch` específicamente:

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

Si las herramientas de un servidor siempre deben ser visibles para Claude sin un paso de búsqueda, establezca `alwaysLoad` en `true` en la configuración de ese servidor. Cada herramienta de ese servidor se carga en el contexto al inicio de la sesión independientemente de la configuración `ENABLE_TOOL_SEARCH`. Úselo para un pequeño número de herramientas que Claude necesita en cada turno, ya que cada herramienta por adelantado consume contexto que de otro modo estaría disponible para su conversación.

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

El campo `alwaysLoad` está disponible en todos los tipos de servidor. Un servidor MCP también puede marcar herramientas individuales como siempre cargadas incluyendo `"anthropic/alwaysLoad": true` en el objeto `_meta` de la herramienta, que tiene el mismo efecto solo para esa herramienta.

Configurar `alwaysLoad: true` también hace que el inicio espere las herramientas del servidor, limitado al tiempo de espera de conexión estándar de 5 segundos, ya que deben estar presentes cuando se construye el primer mensaje. Un servidor remoto con una entrada [`cached`](#server-status-detail) válida proporciona sus herramientas desde la caché sin conectarse, por lo que no retiene el inicio. Otros servidores se conectan en segundo plano de forma predeterminada; establezca [`MCP_CONNECTION_NONBLOCKING=0`](/docs/es/env-vars) para hacer que el inicio también los espere.

<h2 id="use-mcp-prompts-as-commands">
  Usar prompts de MCP como comandos
</h2>

Los servidores MCP pueden exponer prompts que se vuelven disponibles como comandos en Claude Code.

<h3 id="execute-mcp-prompts">
  Ejecutar prompts de MCP
</h3>

<Steps>
  <Step title="Descubrir prompts disponibles">
    Escriba `/` para ver los comandos disponibles para usted, incluidos los de los servidores MCP. Claude Code enumera cada prompt de MCP como `/servername:promptname (MCP)`. Escribir `/mcp__servername__promptname` también lo ejecuta.
  </Step>

  <Step title="Ejecutar un prompt sin argumentos">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Ejecutar un prompt con argumentos">
    Muchos prompts aceptan argumentos. Páselos separados por espacios después del comando. Claude Code divide los argumentos en espacios en blanco, por lo que cada argumento es un único token:

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Los prompts de MCP se descubren dinámicamente desde los servidores conectados
  * Los argumentos se analizan según los parámetros definidos del prompt
  * Los resultados del prompt se inyectan directamente en la conversación
  * En la forma `/mcp__servername__promptname`, Claude Code reemplaza cualquier carácter en el nombre del servidor fuera de `A-Z`, `a-z`, `0-9`, `_` y `-` con `_`, y utiliza el nombre del prompt tal como lo declara el servidor
</Tip>

<h2 id="managed-mcp-configuration">
  Configuración MCP gestionada
</h2>

Para organizaciones que necesitan control centralizado sobre qué servidores MCP pueden conectar los usuarios, consulte [Configuración MCP gestionada](/docs/es/managed-mcp). Cubre la implementación de un conjunto de servidores fijo con `managed-mcp.json`, la provisión de servidores a cada usuario con `managedMcpServers`, la restricción de servidores con `allowedMcpServers` y `deniedMcpServers`, y lo que los usuarios ven cuando un servidor está bloqueado.
