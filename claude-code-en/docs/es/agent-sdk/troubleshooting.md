> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solucionar problemas del Agent SDK

> Corrija los errores del Agent SDK cuando la CLI de Claude Code no se inicia, el proceso de CLI se cierra o llega un resultado exitoso sin salida estructurada.

Esta página cubre errores del Agent SDK en el inicio de la CLI, la salida del proceso de CLI y las salidas estructuradas. Las entradas en esta página están codificadas según el error que ve. Cada entrada nombra la causa y qué hacer.

Los síntomas vinculados a una característica, como un hook que no se dispara o una skill que no se está utilizando, tienen una sección de solución de problemas en la página de esa característica. La tabla nombra la sección o página que cubre cada síntoma:

| Síntoma                                                                                                                                                                                                                                                                                                                                       | Ir a                                                                                                                                        |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| Skills no encontradas, una skill no se está utilizando, error `Invalid skill name`                                                                                                                                                                                                                                                            | [Solución de problemas de skills](/docs/es/agent-sdk/skills#troubleshooting)                                                                     |
| El servidor MCP muestra estado `failed`, las herramientas no se están llamando, tiempos de espera de conexión, salida de herramienta que excede el máximo de tokens permitidos                                                                                                                                                                | [Solución de problemas de MCP](/docs/es/agent-sdk/mcp#troubleshooting)                                                                           |
| Plugin no se carga, las skills del plugin no aparecen                                                                                                                                                                                                                                                                                         | [Solución de problemas de plugins](/docs/es/agent-sdk/plugins#troubleshooting)                                                                   |
| Claude no delega a subagentes, los agentes basados en sistema de archivos no se cargan                                                                                                                                                                                                                                                        | [Solución de problemas de subagentes](/docs/es/agent-sdk/subagents#troubleshooting)                                                              |
| Las opciones de checkpointing no se reconocen, mensajes de usuario sin UUID, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                                                                         | [Solución de problemas de checkpointing de archivos](/docs/es/agent-sdk/file-checkpointing#troubleshooting)                                      |
| Hook no se dispara, el matcher no filtra como se esperaba, tiempo de espera del hook, herramienta bloqueada inesperadamente, entrada modificada no aplicada, hooks de sesión no disponibles en Python, solicitudes de permiso de subagente multiplicándose, bucles de hook recursivos con subagentes, `systemMessage` no aparece en la salida | [Corregir problemas comunes](/docs/es/agent-sdk/hooks#fix-common-issues) en la página de hooks                                                   |
| Un agente que funciona en su máquina falla en un servicio implementado o contenedor                                                                                                                                                                                                                                                           | [Solucionar problemas de implementación](/docs/es/agent-sdk/hosting#troubleshoot-deployment-failures)                                            |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                                                                            | [Referencia de errores](/docs/es/errors#find-your-error)                                                                                         |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` es `None`                                                                                                                                                                     | [Inicio de CLI](#cli-startup), [Salida del proceso de CLI](#cli-process-exit) y [Salidas estructuradas](#structured-outputs) en esta página |

<h2 id="cli-startup">
  Inicio de CLI
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code no encontrado
</h3>

El SDK de Python inicia la CLI de Claude Code como un subproceso. Cuando no puede encontrar un ejecutable `claude`, la conexión falla con un `CLINotFoundError`:

```
Claude Code not found at: /your/configured/path
```

El mensaje incluye la ruta configurada cuando establece `ClaudeAgentOptions(cli_path=...)` y apunta a un archivo faltante. Sin `cli_path`, el SDK busca en su `PATH` y ubicaciones de instalación comunes, e incluye instrucciones de instalación para su plataforma.

Para corregirlo:

* Instale Claude Code si no está instalado. Consulte [Install Claude Code](/docs/es/setup#install-claude-code) para el comando en su plataforma.
* Si establece `cli_path`, confirme que el archivo existe y es el ejecutable `claude`.
* Si confía en la resolución de `PATH`, confirme que `claude --version` funciona en el mismo entorno en el que se ejecuta su aplicación. Los procesos que inicia fuera de su shell, como desde un IDE o un administrador de servicios, a menudo se ejecutan con un `PATH` diferente.

El SDK de TypeScript busca la CLI en su paquete de plataforma incluido y la ruta que establece en `pathToClaudeCodeExecutable`. Haga coincidir el mensaje que ve:

* `Native CLI binary for <platform>-<arch> not found`: el paquete de plataforma incluido falta, la mayoría de las veces porque la instalación omitió dependencias opcionales. Reinstale `@anthropic-ai/claude-agent-sdk` sin omitir dependencias opcionales, o apunte `pathToClaudeCodeExecutable` a una [instalación nativa](/docs/es/setup#install-claude-code). En un ejecutable de archivo único compilado con `bun build --compile`, el mismo mensaje tiene una causa y solución diferentes. Consulte [Compile to a single executable](/docs/es/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` o `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: el archivo en la ruta resuelta falta, o el proceso no puede acceder a él. Confirme que el archivo existe en esa ruta y que el proceso puede acceder a él.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

En Windows, la conexión falla con un `CLIConnectionError` cuando la ruta de CLI que usa el SDK de Python es un script por lotes `.bat` o `.cmd`, incluido el shim `claude.cmd` que crea una instalación npm:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

La negativa es un endurecimiento de seguridad deliberado, no una instalación rota. Windows ejecuta scripts por lotes reescribiendo el spawn en una invocación `cmd.exe /c`, y `cmd.exe` reanaliza toda la línea de comandos en tiempo de ejecución, por lo que un valor de argumento puede ejecutar comandos inyectados.

La mayoría de las instalaciones de Windows nunca alcanzan este error. La rueda x64 de Windows de `claude-agent-sdk` incluye un `claude.exe`, y el SDK prefiere la CLI incluida, luego cualquier `claude.exe` nativo que pueda descubrir, antes de recurrir a un shim por lotes. Ve la negativa en dos casos:

* Establece `ClaudeAgentOptions(cli_path=...)` en un archivo `.bat` o `.cmd`, como el shim `claude.cmd` de npm.
* Su instalación no tiene un `claude.exe` incluido o nativo, por ejemplo una instalación de fuente en ARM64 Windows donde el único `claude` en su `PATH` es el shim npm.

Para corregirlo, proporcione al SDK un ejecutable nativo en lugar de un script por lotes:

* Si establece `ClaudeAgentOptions(cli_path=...)`, apúntelo a un `claude.exe` o elimine la opción. El SDK omite el descubrimiento mientras `cli_path` está establecido, por lo que una instalación nativa sola no puede tener efecto.
* Instale Claude Code de forma nativa en PowerShell: `irm https://claude.ai/install.ps1 | iex`
* En Windows x64, instale la rueda `claude-agent-sdk`, que incluye `claude.exe`.

Antes de `claude-agent-sdk` 0.2.124, el SDK de Python generaba scripts por lotes a través de `cmd.exe` sin esta verificación.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

El SDK encontró un archivo en la ruta resuelta pero no pudo iniciarlo. Python genera estos errores como un `CLIConnectionError`. TypeScript rechaza la iteración del mensaje con un error sin clase SDK. La tabla a continuación asigna cada mensaje a lo que le dice. Haga coincidir el mensaje que ve:

| Mensaje                                                           | SDK        | Lo que le dice                                                                |
| ----------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | El resto del mensaje es el error propio del sistema operativo                 |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | El script en la ruta configurada no puede ejecutarse                          |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | El binario no puede ejecutarse, con una sugerencia de libc añadida al mensaje |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Cualquier otro error de inicio                                                |

En ambos SDK, la causa habitual es una ruta resuelta que apunta a algo que no puede ejecutarse, como un archivo de texto, un directorio o un archivo sin permiso de ejecución. Lea la sugerencia de libc del mensaje de binario nativo como una posible causa.

Para corregirlo en cualquier SDK:

* Confirme que la ruta configurada apunta al ejecutable `claude` en sí y que el archivo tiene permiso de ejecución.
* Si no necesita una ruta personalizada, elimine `cli_path` en Python o `pathToClaudeCodeExecutable` en TypeScript para que el SDK encuentre una CLI por su cuenta, prefiriendo su copia incluida.
* Cuando el binario que falla es la copia incluida del SDK en una imagen de contenedor, reinstale el SDK durante la compilación de la imagen para que el binario incluido coincida con la plataforma del contenedor, o reconstruya la imagen para la arquitectura en la que se ejecuta. La causa habitual es un binario que no coincide con la arquitectura o libc del contenedor, o uno que perdió su permiso de ejecución en la compilación de la imagen.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Llamar a un método `ClaudeSDKClient` en Python antes de que el cliente se haya conectado, o después de que se haya desconectado, genera un `CLIConnectionError` con este mensaje:

```
Not connected. Call connect() first.
```

Haga lo que dice el mensaje. Llame a `await client.connect()` antes de cualquier otro método de cliente, o abra el cliente con `async with ClaudeSDKClient() as client:`, que se conecta al entrar.

<h2 id="cli-process-exit">
  Salida del proceso CLI
</h2>

Las entradas en esta sección significan que el proceso de Claude Code terminó mientras su aplicación lo estaba usando. Qué error ve depende del idioma del SDK y de si la CLI reportó un resultado de error antes de salir.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

El SDK de Python genera un `ProcessError` cuando el proceso de Claude Code sale con un código distinto de cero:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

El mensaje indica el código de salida dos veces, y la línea `Error output` es texto fijo en lugar de la salida de error de su proceso. El mismo texto fijo llena el atributo `stderr` de la excepción. El atributo `exit_code` de la excepción lleva el código. Para capturar lo que la CLI realmente escribió en stderr, pase un callback `stderr` en `ClaudeAgentOptions` y registre lo que recibe.

Un `ProcessError` simple significa que la CLI salió sin reportar un resultado de error. Cuando la CLI reportó uno, el SDK genera [`ResultError`](/docs/es/agent-sdk/python#resulterror) en su lugar, cubierto en [Claude Code returned an error result](#claude-code-returned-an-error-result). `ResultError` subclasifica `ProcessError`, por lo que `except ProcessError` captura ambos. Para manejarlos de manera diferente, coloque la cláusula `except ResultError` primero.

Antes de `claude-agent-sdk` 0.2.140, el SDK de Python generaba salidas de resultado de error como una `Exception` simple en lugar de un `ResultError`.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

Los wrappers de IDE también imprimen este mensaje, y la [referencia de error](/docs/es/errors#claude-code-process-exited-with-code-n) la cubre para VS Code y otros lanzadores. Esta entrada cubre lo que recibe su código del SDK de TypeScript. El SDK presenta una salida de CLI con código distinto de cero como un `Error` simple que rechaza el bucle `for await` sobre los mensajes de `query()`. No hay clase de error SDK para capturar, así que envuelva el bucle en `try`/`catch` y haga coincidir el mensaje:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Cuando la CLI escribió en stderr, el mensaje termina con la cola de la misma. Para capturar la secuencia completa, pase un callback `stderr` en las opciones de consulta. Un proceso eliminado por una señal reporta `Claude Code process terminated by signal <name>` en la misma forma.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

Ambos SDK reemplazan el error de salida del proceso con este mensaje cuando la CLI reportó un resultado de error antes de salir:

```
Claude Code returned an error result: <the CLI's own error report>
```

El texto después de los dos puntos es el informe de la CLI sobre qué salió mal, así que comience allí en lugar de con la salida en sí. Python genera esto como un [`ResultError`](/docs/es/agent-sdk/python#resulterror), cuyo atributo `data` lleva el resultado de error completo. TypeScript rechaza el bucle de mensajes con un `Error` simple que lleva la misma forma de mensaje.

<h2 id="structured-outputs">
  Salidas estructuradas
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output es None pero el resultado dice éxito
</h3>

Un mensaje de resultado puede terminar con `subtype: "success"` mientras `structured_output` es `None` en Python o `undefined` en TypeScript. La ejecución se completa, pero no existe una salida validada. Una forma de alcanzar esto es un esquema que ninguna salida puede satisfacer, por ejemplo restricciones de longitud conflictivas. La ejecución termina sin un error de validación, y la única señal es el `structured_output` faltante.

Trate este resultado como un error en el código de la aplicación. Verifique tanto que `subtype` sea `success` como que `structured_output` esté presente antes de usarlo. La sección [Error handling](/docs/es/agent-sdk/structured-outputs#error-handling) muestra este patrón para ambos SDK.

Si sucede repetidamente con un esquema que cree que es correcto, verifique que el esquema sea satisfacible, luego simplifíquelo hasta que las salidas se validen, e reintroduzca las restricciones una a la vez.

<h2 id="report-a-new-issue">
  Reportar un nuevo problema
</h2>

Si su error no está cubierto aquí, verifique los problemas abiertos o presente uno nuevo en los repositorios del SDK: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) o [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Incluya el texto de error completo y su versión del SDK.
