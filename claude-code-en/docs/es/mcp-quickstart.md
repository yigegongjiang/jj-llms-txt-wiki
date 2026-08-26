> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectarse a servidores MCP

> Agregue un servidor MCP a Claude Code, verifique la conexión y encuentre la configuración en el disco.

El [Protocolo de Contexto de Modelo (MCP)](https://modelcontextprotocol.io/introduction) permite que Claude Code utilice herramientas más allá de su conjunto integrado, como buscar en un rastreador de problemas, consultar una base de datos o controlar un navegador web. Estas herramientas provienen de servidores MCP, que se ejecutan en su máquina o como servicios alojados.

Esta guía le muestra cómo conectar un servidor MCP de extremo a extremo con la CLI de Claude Code. Al final, tendrá un servidor conectado y respondiendo, sabrá dónde vive su configuración en el disco y sabrá cómo solucionar los errores de conexión más comunes.

<Note>
  También puede agregar servidores MCP desde otras superficies, incluida la aplicación de escritorio, VS Code y la web. Consulte [Conectarse desde otras superficies](#connect-from-other-surfaces).
</Note>

Para cada forma de conectar y configurar servidores MCP en Claude Code, consulte la [referencia de MCP](/docs/es/mcp).

<h2 id="before-you-begin">
  Antes de comenzar
</h2>

Asegúrese de tener:

* [Claude Code instalado](/docs/es/quickstart) y autenticado
* Una terminal abierta en un directorio de proyecto. Cualquier directorio funciona, incluido uno vacío.

<h2 id="add-and-verify-a-server">
  Agregar y verificar un servidor
</h2>

El ejemplo a continuación se conecta al [servidor MCP de documentación de Claude Code](https://code.claude.com/docs/mcp), un servidor alojado con búsqueda de texto completo en los documentos de Claude Code. No requiere autenticación ni ninguna configuración especial, por lo que funciona bien como primer servidor para probar el flujo de configuración.

Los pasos son los mismos para cualquier servidor: agréguelo, verifique el estado de la conexión, luego úselo en una sesión, con un paso de limpieza opcional al final. Algunos servidores agregan un paso, como un inicio de sesión del navegador, que se muestra en [Ejemplos adicionales de servidores MCP](#additional-mcp-server-examples). Para más servidores para conectar, explore el [Directorio de Anthropic](/docs/es/mcp#find-and-build-mcp-servers).

<Steps>
  <Step title="Agregar el servidor MCP">
    Registre el servidor con Claude Code. Ejecute esto en su terminal, no dentro de una sesión `claude`: está configurando el servidor antes de iniciar una conversación.

    ```bash theme={null}
    claude mcp add --transport http claude-code-docs https://code.claude.com/docs/mcp
    ```

    Las partes del comando:

    * `claude mcp add`: registra un servidor con Claude Code.
    * `--transport http`: el servidor se aloja en una URL en lugar de ejecutarse como un proceso local.
    * `claude-code-docs`: un nombre que usted elige. Llamar al mismo servidor `docs` funcionaría de manera idéntica. Claude Code utiliza el nombre que elija para etiquetar las herramientas del servidor en la salida de Claude y para referirse al servidor en comandos como `claude mcp remove`.
    * `https://code.claude.com/docs/mcp`: la URL donde se aloja el servidor.

    El comando imprime una confirmación como `Added HTTP MCP server claude-code-docs with URL: https://code.claude.com/docs/mcp to local config`. La parte `local config` significa que el servidor está registrado para usted, en este proyecto: si inicia Claude Code en un proyecto diferente, este servidor no está activo allí. Para registrar un servidor una vez para todos sus proyectos, agréguelo en el ámbito del usuario, cubierto en [Cambiar el ámbito del servidor](#change-server-scope).
  </Step>

  <Step title="Verificar el estado de la conexión">
    Confirme que el servidor aparece en su lista de servidores y verifique su estado:

    ```bash theme={null}
    claude mcp list
    ```

    El servidor aparece con un indicador de estado:

    | Estado                             | Significado                                                                                                                                                                                                        |
    | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `✓ Connected`                      | Listo para usar. Esto es lo que debería ver para `claude-code-docs`                                                                                                                                                |
    | `! Connected · tools fetch failed` | El servidor se conectó pero no pudo enumerar sus herramientas. Ejecute `claude mcp get <name>` para obtener el detalle del error                                                                                   |
    | `! Needs authentication`           | El servidor es accesible pero necesita un inicio de sesión del navegador, o un token pasado con `--header`. Consulte [Conectar un servidor que requiere inicio de sesión](#connect-a-server-that-requires-sign-in) |
    | `✗ Failed to connect`              | El servidor no respondió. Consulte [Solución de problemas](#troubleshooting)                                                                                                                                       |
    | `✗ Connection error`               | El intento de conexión lanzó un error. Consulte [Solución de problemas](#troubleshooting)                                                                                                                          |
    | `⏸ Pending approval`               | Un servidor con ámbito de proyecto que aún no ha aprobado. Consulte [Editar .mcp.json directamente](#edit-mcp-json-directly)                                                                                       |
  </Step>

  <Step title="Usar el servidor">
    Inicie una sesión y pida a Claude que use el nuevo servidor por nombre:

    ```bash theme={null}
    claude
    ```

    ```text theme={null}
    Use the claude-code-docs server to look up what MCP_TIMEOUT does
    ```

    <Info>
      Normalmente no necesita nombrar un servidor en su indicación, ya que Claude elige herramientas relevantes por su cuenta. Nombrarlo aquí garantiza que la demostración pase por el nuevo servidor en lugar de otra herramienta, como web fetch, que podría responder la misma pregunta.
    </Info>

    La primera vez que Claude llama al servidor, solicita permiso para usar la nueva herramienta. Apruébelo para continuar. La llamada de herramienta en la salida de Claude está etiquetada con el nombre del servidor, que es cómo confirma que la respuesta provino del servidor MCP en lugar del conocimiento integrado de Claude.
  </Step>

  <Step title="Eliminar el servidor">
    Este paso es opcional. Cuando termine de experimentar, puede eliminar el servidor:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    <Note>
      Cada servidor conectado ocupa algo de espacio en [la ventana de contexto de Claude](/docs/es/how-claude-code-works#the-context-window) porque sus nombres de herramientas e instrucciones del servidor se cargan en cada sesión. Eliminar servidores que ya no usa mantiene ese espacio libre.
    </Note>
  </Step>
</Steps>

<h2 id="where-servers-are-saved">
  Dónde se guardan los servidores
</h2>

El comando `claude mcp add` escribe los detalles del servidor en un archivo de configuración. De forma predeterminada, registra el servidor en el ámbito `local`: privado para usted, activo solo en el proyecto actual. Pase `--scope user` para registrarlo una vez para todos sus proyectos, o `--scope project` para compartirlo con sus compañeros de equipo. [Cambiar el ámbito del servidor](#change-server-scope) explica ambos.

<Note>
  `claude mcp add` funciona igual en cada shell, incluidos PowerShell y Command Prompt. Dentro de una sesión `claude`, use el comando `/mcp` para verificar y administrar servidores que ya ha agregado.
</Note>

Hay otras formas de agregar un servidor, cada una cubierta más adelante en esta página:

* [Agregar un servidor local](#add-a-local-server): ejecutar un programa en su máquina en lugar de conectarse a una URL.
* [Editar `.mcp.json` directamente](#edit-mcp-json-directly): escribir la entrada JSON usted mismo en lugar de usar el comando.
* [Conectar un servidor que requiere inicio de sesión](#connect-a-server-that-requires-sign-in): agregar un servidor alojado que necesita un inicio de sesión del navegador antes de que sus herramientas funcionen.

<h3 id="find-your-configuration-on-disk">
  Encontrar su configuración en el disco
</h3>

El comando `claude mcp add` escribe el servidor en uno de tres ámbitos, almacenados en dos archivos, dependiendo de la bandera `--scope`. No necesita editar estos archivos directamente, pero saber dónde están ayuda con la depuración y el control de versiones.

| Ámbito    | Archivo                                                        | Disponible para                                   |
| :-------- | :------------------------------------------------------------- | :------------------------------------------------ |
| `local`   | `~/.claude.json`, bajo la entrada para este proyecto           | Solo usted, solo este proyecto. El predeterminado |
| `project` | `.mcp.json` en la raíz de su proyecto                          | Todos los que clonan el proyecto                  |
| `user`    | `~/.claude.json`, bajo la clave `mcpServers` de nivel superior | Solo usted, todos los proyectos                   |

En Windows, `~/.claude.json` se resuelve a `%USERPROFILE%\.claude.json`, típicamente `C:\Users\YourName\.claude.json`. Si ha establecido [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars), Claude Code lee `.claude.json` desde dentro de ese directorio en su lugar.

Ejecute `claude mcp get claude-code-docs` para ver qué ámbito contiene la definición de un servidor. Para cómo interactúan los ámbitos cuando el mismo servidor se define en más de uno, consulte [Ámbitos de instalación de MCP](/docs/es/mcp#mcp-installation-scopes).

<h2 id="change-server-scope">
  Cambiar el ámbito del servidor
</h2>

El ámbito de un servidor se fija cuando lo agrega, por lo que cambiar el ámbito significa eliminar la entrada y volver a agregarla en la nueva. Ambos casos a continuación comienzan eliminando la entrada local del primer tutorial, por lo que el servidor tiene solo una definición. Si ya lo eliminó al final de ese tutorial, omita este comando:

```bash theme={null}
claude mcp remove claude-code-docs --scope local
```

<h3 id="use-a-server-in-all-your-projects">
  Usar un servidor en todos sus proyectos
</h3>

Vuelva a agregar el servidor en el ámbito `user` para hacerlo activo en cada proyecto que abra, aún privado para usted:

```bash theme={null}
claude mcp add --scope user --transport http claude-code-docs https://code.claude.com/docs/mcp
```

<h3 id="share-a-server-with-your-team">
  Compartir un servidor con su equipo
</h3>

Vuelva a agregar el servidor en el ámbito `project`, que escribe en `.mcp.json` en la raíz del proyecto:

```bash theme={null}
claude mcp add --scope project --transport http claude-code-docs https://code.claude.com/docs/mcp
```

Confirme `.mcp.json` en el control de versiones. Los compañeros de equipo que clonan el repositorio e inician Claude Code ven un mensaje para aprobar el servidor, luego se conecta para ellos también.

<h2 id="additional-mcp-server-examples">
  Ejemplos adicionales de servidores MCP
</h2>

El primer tutorial utilizó un servidor alojado que se conecta sin ningún inicio de sesión. Los ejemplos a continuación cubren las otras dos formas comunes, con el mismo flujo de agregar, verificar, usar.

<h3 id="add-a-local-server">
  Agregar un servidor local
</h3>

Un servidor stdio local es un programa que Claude Code inicia como un subproceso en su máquina, en lugar de un servicio al que accede a través de una URL. Use uno para herramientas que necesitan acceso a recursos locales como un navegador, su sistema de archivos o un socket de base de datos.

El [servidor MCP de Playwright](https://github.com/microsoft/playwright-mcp) es uno bueno para probar: le da a Claude un navegador que puede navegar, hacer clic y leer, y no necesita ninguna cuenta. Se ejecuta a través de `npx`, por lo que requiere [Node.js](https://nodejs.org/en/download) 18 o posterior.

<Steps>
  <Step title="Agregar el servidor Playwright">
    Registre el servidor con el comando que Claude Code debe ejecutar para iniciarlo:

    ```bash theme={null}
    claude mcp add playwright -- npx -y @playwright/mcp@latest
    ```

    Este comando difiere del ejemplo alojado de tres maneras:

    * No hay bandera `--transport`, porque los servidores locales usan el transporte predeterminado `stdio`.
    * Todo después del separador `--` es el comando que Claude Code ejecuta para iniciar el servidor.
    * `-y` le dice a `npx` que instale el paquete sin preguntar.

    Playwright controla cualquier Chrome que ya esté instalado en su máquina. Para usar un navegador diferente, agregue `--browser` con el nombre del navegador, por ejemplo `--browser firefox`, después de `@playwright/mcp@latest`.
  </Step>

  <Step title="Verificar la conexión">
    La confirmación `Added` significa que la entrada se guardó, no que el comando se ejecute. Verifique la conexión:

    ```bash theme={null}
    claude mcp list
    ```

    La primera verificación puede mostrar `✗ Failed to connect` mientras `npx` descarga el paquete, así que espere un momento y ejecútelo de nuevo.
  </Step>

  <Step title="Usar el navegador">
    Dale a Claude una tarea que necesite el navegador:

    ```text theme={null}
    Use playwright to open https://example.com and tell me the page title
    ```

    Se abre una ventana del navegador para que pueda verlo funcionar, y las llamadas de herramienta en la salida de Claude están etiquetadas con el nombre del servidor `playwright` y la acción, como `browser_navigate`.

    Intente apuntarlo a su servidor de desarrollo local para verificar que una página aún se renderiza después de un cambio, o haga que recorra un informe de error paso a paso.
  </Step>
</Steps>

<h3 id="connect-a-server-that-requires-sign-in">
  Conectar un servidor que requiere inicio de sesión
</h3>

Servicios alojados como Sentry, Linear y Notion ejecutan sus servidores MCP detrás de OAuth: agrega la URL del servidor, luego inicia sesión a través de su navegador.

Los pasos a continuación usan Sentry como ejemplo. Para conectar un servicio diferente, sustituya su URL, que puede encontrar en el [Directorio de Anthropic](/docs/es/mcp#find-and-build-mcp-servers) o en la documentación del servicio.

<Steps>
  <Step title="Agregar el servidor">
    El comando `add` es el mismo que para el servidor de documentos, con la URL de Sentry:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```

    Después de agregar, `claude mcp list` muestra el servidor con `! Needs authentication`. Eso es esperado: el siguiente paso completa el inicio de sesión.
  </Step>

  <Step title="Autenticarse en su navegador">
    Inicie una sesión de Claude Code y abra el panel MCP:

    ```text theme={null}
    /mcp
    ```

    Seleccione `sentry` de la lista, presione Intro y elija `Authenticate`. Su navegador se abre a la página de inicio de sesión de Sentry. Apruebe la conexión allí.

    De vuelta en Claude Code, el estado del servidor cambia a conectado. Si el inicio de sesión falla o el navegador no se abre, consulte [Solución de problemas](#troubleshooting).
  </Step>

  <Step title="Usar el servidor">
    Pregúntele a Claude algo que necesite el servicio, como `What Sentry projects do I have access to?`, y busque llamadas de herramienta etiquetadas con el nombre del servidor `sentry` en su salida.
  </Step>
</Steps>

Los servidores que se autentican con un token estático en lugar de OAuth toman el token en el momento de agregar con `--header "Authorization: Bearer <token>"`. Consulte el [ejemplo de GitHub](/docs/es/mcp#example-connect-to-github-for-code-reviews) para una versión trabajada.

<h2 id="edit-mcp-json-directly">
  Editar .mcp.json directamente
</h2>

Cada archivo en la [tabla de ámbitos](#find-your-configuration-on-disk) utiliza el mismo formato JSON para entradas de servidor. Esta sección edita `.mcp.json`, el archivo de ámbito de proyecto. Es el que más vale la pena escribir a mano porque se verifica en el repositorio, donde también funciona como configuración como código para su equipo.

Cree `.mcp.json` en la raíz de su proyecto. El ejemplo a continuación define ambos servidores de esta guía, el servidor de documentos alojado alcanzado a través de HTTP y el servidor Playwright como un proceso `stdio` local:

```json theme={null}
{
  "mcpServers": {
    "claude-code-docs": {
      "type": "http",
      "url": "https://code.claude.com/docs/mcp"
    },
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

Los campos difieren según el tipo de servidor:

* Para servidores HTTP, `url` es el punto final al que se conecta Claude Code.
* Para servidores stdio, `command` y `args` son el programa que ejecuta.

Después de guardar el archivo, inicie una nueva sesión de Claude Code en el proyecto. Claude Code lee `.mcp.json` al iniciar.

La primera vez que Claude Code ve un servidor con ámbito de proyecto, le pide que lo apruebe. El mensaje existe para que un repositorio que Clone no pueda lanzar procesos en su máquina sin su consentimiento. Apruebe el mensaje, o ejecute `/mcp` para aprobar más tarde si lo perdió.

Una vez que haya aprobado, ejecute `/mcp` y verifique que los servidores se muestren como conectados. Si uno muestra un error en su lugar, consulte [Solución de problemas](#troubleshooting).

<h2 id="connect-from-other-surfaces">
  Conectarse desde otras superficies
</h2>

Esta guía utiliza los comandos CLI `claude mcp`, pero cada superficie de Claude Code puede conectarse a servidores MCP:

* **Aplicación de escritorio de Claude Code**: agregue servidores a través de la [IU de Conectores](/docs/es/desktop#connect-external-tools).
* **Aplicación de chat de Claude Desktop**: una aplicación separada de Claude Code. Para copiar servidores de su `claude_desktop_config.json` en la CLI, ejecute `claude mcp add-from-claude-desktop` en macOS o WSL.
* **VS Code**: consulte [Conectarse a herramientas externas con MCP](/docs/es/vs-code#connect-to-external-tools-with-mcp).
* **Claude Code en la web**: lee `.mcp.json` de su repositorio. Consulte [Editar .mcp.json directamente](#edit-mcp-json-directly).
* **Claude.ai**: los conectores que agrega en [claude.ai/customize/connectors](https://claude.ai/customize/connectors) se cargan automáticamente en la CLI cuando inicia sesión con esa cuenta. Consulte [Usar servidores MCP desde Claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai).

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Si un servidor no se conecta, verifique su estado con `/mcp` dentro de una sesión o `claude mcp list` desde su shell, luego haga coincidir el síntoma a continuación. El panel `/mcp` también le permite reconectarse o autenticarse sin salir de la sesión.

<AccordionGroup>
  <Accordion title="/mcp shows No MCP servers configured">
    Claude Code no encontró ningún servidor para el directorio actual. Las causas más comunes:

    * Ejecutó `claude mcp add` desde un proyecto diferente. Los servidores con ámbito local están vinculados al proyecto donde los agregó: la raíz del repositorio, o el directorio exacto si no estaba en un repositorio git. Vuelva a agregar el servidor desde el proyecto en el que se encuentra ahora, o agréguelo con `--scope user` para que no esté vinculado a un proyecto.
    * Editó un archivo de configuración en la ruta incorrecta. Los archivos correctos son `~/.claude.json` y `<project>/.mcp.json`. Claude Code no lee rutas como `~/.claude/.mcp.json`, `~/.claude/config/mcp.json`, `~/.claude/mcp.json`, o `%APPDATA%\Claude\mcp.json`. Para servidores con ámbito de usuario, ejecute `claude mcp add --scope user`, que escribe en la clave `mcpServers` en `~/.claude.json`; para servidores con ámbito de proyecto, edite `.mcp.json` en la raíz del proyecto.
  </Accordion>

  <Accordion title="Status shows Failed to connect or Connection error">
    Ambos estados significan que el servidor no se inició o la URL no respondió. También pueden aparecer para servidores HTTP que esperan un token en lugar del inicio de sesión del navegador cubierto en [Conectar un servidor que requiere inicio de sesión](#connect-a-server-that-requires-sign-in).

    A partir de v2.1.191, un servidor HTTP que devuelve `404 Not Found` muestra `MCP endpoint not found at <url>. Check the URL in your MCP config.` cuando selecciona el servidor en `/mcp`, con la URL que Claude Code intentó. Las versiones anteriores muestran un mensaje genérico `Error POSTing to endpoint` sin la URL. Compare la URL con la ruta del punto final MCP documentada del servidor, luego ejecute `claude mcp remove <name>` y vuelva a agregar con la URL correcta.

    Para servidores HTTP, confirme que la URL es accesible desde su máquina:

    ```bash theme={null}
    curl -I https://mcp.sentry.dev/mcp
    ```

    En PowerShell, use `curl.exe` en lugar de `curl` para que la solicitud vaya al binario curl real en lugar del alias `Invoke-WebRequest`.

    La respuesta le dice qué tipo de problema tiene:

    * Un `404` o `405`: el servidor está activo. Muchos puntos finales de MCP responden solo a solicitudes POST, por lo que esto aún confirma que la URL es accesible desde su máquina.
    * Un `401` o `403`: el servidor está activo y necesita autenticarse. Use el inicio de sesión del navegador en [Conectar un servidor que requiere inicio de sesión](#connect-a-server-that-requires-sign-in), o para servidores que toman un token en su lugar, como el de GitHub, páselo con `--header "Authorization: Bearer <token>"` en el comando `claude mcp add`.
    * Sin respuesta en absoluto: verifique la URL y su red.

    Para servidores stdio, ejecute el comando configurado directamente en su terminal para ver el error subyacente. Para el servidor Playwright de esta guía, ejecute:

    ```bash theme={null}
    npx -y @playwright/mcp@latest
    ```

    Lo que sucede a continuación le dice dónde está el problema:

    * El comando se inicia y espera entrada: el servidor en sí funciona. Ejecute `claude mcp get <name>` y confirme que el comando mostrado allí coincida con lo que acaba de ejecutar. Si el comando mostrado difiere de lo que escribió, probablemente omitió el separador `--` antes del comando del servidor. Elimine el servidor y vuelva a agregarlo con `--` en su lugar. Si escribió `.mcp.json` a mano, verifique su sintaxis y ubicación.
    * El comando genera un error: el mensaje nombra lo que falta, como Node.js o un navegador.
  </Accordion>

  <Accordion title="Connection timed out at startup">
    El servidor tardó más que el tiempo de espera de inicio predeterminado de 30 segundos. La primera ejecución de un servidor stdio puede ser lenta mientras `npx` descarga el paquete. Aumente el límite con la variable de entorno [`MCP_TIMEOUT`](/docs/es/env-vars), en milisegundos:

    ```bash theme={null}
    MCP_TIMEOUT=60000 claude
    ```

    En PowerShell, establezca la variable antes del comando en la misma línea:

    ```powershell theme={null}
    $env:MCP_TIMEOUT = "60000"; claude
    ```
  </Accordion>

  <Accordion title="Server already exists">
    Ya ha agregado un servidor con ese nombre en el mismo ámbito. Elimine la entrada existente primero o elija un nombre diferente:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    Si el nombre existe en más de un ámbito, `remove` reporta `exists in multiple scopes`. Pase `--scope` para elegir cuál copiar para eliminar, por ejemplo `claude mcp remove claude-code-docs --scope local`.
  </Accordion>

  <Accordion title="Server connects but no tools appear">
    Ejecute `/mcp` dentro de una sesión y seleccione el servidor para ver su lista de herramientas. Si la lista está vacía, el servidor se inició pero no registró ninguna herramienta, lo que generalmente significa que le falta una variable de entorno requerida como una clave API.

    Pase la variable con `--env KEY=value` en `claude mcp add`, o en el campo `env` de la entrada `.mcp.json` del servidor. La documentación del servidor enumera las variables que necesita.
  </Accordion>

  <Accordion title="Changes to .mcp.json don't take effect">
    Claude Code lee `.mcp.json` al iniciar la sesión. Salga y reinicie la sesión después de editar el archivo.

    Si sus servidores aún no aparecen, ejecute `/mcp` y busque una advertencia de análisis. Claude Code omite entradas mal formadas y muestra el campo ofensivo allí.

    Si rechazó previamente el servidor cuando se le solicitó, restablezca las aprobaciones del proyecto:

    ```bash theme={null}
    claude mcp reset-project-choices
    ```
  </Accordion>

  <Accordion title="OAuth sign-in fails or browser doesn't open">
    Ejecute `/mcp`, seleccione el servidor y elija `Authenticate` de nuevo. Si el navegador no se abre automáticamente, copie la URL mostrada en la terminal y ábrala manualmente. Consulte [Autenticarse con servidores MCP remotos](/docs/es/mcp#authenticate-with-remote-mcp-servers) para puertos de devolución de llamada fijos y credenciales preconfiguradas.
  </Accordion>
</AccordionGroup>

<h2 id="next-steps">
  Próximos pasos
</h2>

Con un servidor conectado, explore el resto de lo que MCP permite:

* [Encontrar más servidores MCP](/docs/es/mcp#find-and-build-mcp-servers) en el Directorio de Anthropic
* [Compartir servidores con su equipo](/docs/es/mcp#mcp-installation-scopes) usando ámbitos de instalación
* [Administrar el acceso a MCP para una organización](/docs/es/managed-mcp) con configuraciones administradas y controles de política
* [Hacer referencia a recursos de MCP](/docs/es/mcp#use-mcp-resources) en indicaciones con menciones @
* [Ejecutar indicaciones de MCP como comandos](/docs/es/mcp#use-mcp-prompts-as-commands) desde el menú `/`
* [Construir su propio servidor](https://modelcontextprotocol.io/quickstart/server) con el SDK de MCP
