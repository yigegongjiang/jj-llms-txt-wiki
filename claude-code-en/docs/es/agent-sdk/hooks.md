> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Interceptar y controlar el comportamiento del agente con hooks

> Interceptar y personalizar el comportamiento del agente en puntos clave de ejecución con hooks

Los hooks son funciones de devolución de llamada que ejecutan su código en respuesta a eventos del agente, como una herramienta siendo llamada, una sesión iniciándose, o la ejecución deteniéndose. Con hooks, puede:

* **Bloquear operaciones peligrosas** antes de que se ejecuten, como comandos de shell destructivos o acceso a archivos no autorizado
* **Registrar y auditar** cada llamada de herramienta para cumplimiento, depuración o análisis
* **Transformar entradas y salidas** para sanitizar datos, inyectar credenciales o redirigir rutas de archivos
* **Requerir aprobación humana** para acciones sensibles como escrituras en bases de datos o llamadas a API
* **Rastrear el ciclo de vida de la sesión** para gestionar estado, limpiar recursos o enviar notificaciones

<h2 id="how-hooks-work">
  Cómo funcionan los hooks
</h2>

<Steps>
  <Step title="Se dispara un evento">
    Algo sucede durante la ejecución del agente y el SDK dispara un evento: una herramienta está a punto de ser llamada (`PreToolUse`), una herramienta devolvió un resultado (`PostToolUse`), un subagente se inició o se detuvo, el agente está inactivo, o la ejecución finalizó. Vea la [lista completa de eventos](#available-hooks).
  </Step>

  <Step title="El SDK recopila hooks registrados">
    El SDK verifica si hay hooks registrados para ese tipo de evento. Esto incluye hooks de devolución de llamada que pasa en `options.hooks` y hooks de comandos de shell de archivos de configuración cuando la entrada [`settingSources`](/docs/es/agent-sdk/typescript#settingsource) o [`setting_sources`](/docs/es/agent-sdk/python#settingsource) correspondiente está habilitada, lo cual lo está para las opciones predeterminadas de `query()`.
  </Step>

  <Step title="Los matchers filtran qué hooks se ejecutan">
    Si un hook tiene un patrón [`matcher`](#matchers) (como `"Write|Edit"`), el SDK lo prueba contra el objetivo del evento (por ejemplo, el nombre de la herramienta). Los hooks sin un matcher se ejecutan para cada evento de ese tipo.
  </Step>

  <Step title="Se ejecutan las funciones de devolución de llamada">
    Cada hook coincidente recibe su [función de devolución de llamada](#callback-functions) con información sobre lo que está sucediendo: el nombre de la herramienta, sus argumentos, el ID de sesión y otros detalles específicos del evento.
  </Step>

  <Step title="Su devolución de llamada devuelve una decisión">
    Después de realizar cualquier operación (registro, llamadas a API, validación), su devolución de llamada devuelve un [objeto de salida](#outputs) que le dice al agente qué hacer: permitir la operación, bloquearla, modificar la entrada o inyectar contexto en la conversación.
  </Step>
</Steps>

El siguiente ejemplo reúne estos pasos. Registra un hook `PreToolUse` (paso 1) con un matcher `"Write|Edit"` (paso 3) para que la devolución de llamada solo se dispare para herramientas de escritura de archivos. Cuando se activa, la devolución de llamada recibe la entrada de la herramienta (paso 4), verifica si la ruta del archivo apunta a un archivo `.env`, y devuelve `permissionDecision: "deny"` para bloquear la operación (paso 5):

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Create a .env file with the standard local development database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Create a .env file with the standard local development database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

Cuando ejecuta cualquiera de los scripts, Claude intenta crear el archivo `.env`, el hook niega la llamada de herramienta, y la respuesta final de Claude explica que no puede crear archivos `.env`.

<h2 id="available-hooks">
  Hooks disponibles
</h2>

El SDK proporciona hooks para diferentes etapas de la ejecución del agente. Algunos hooks están disponibles en ambos SDK, mientras que otros son solo para TypeScript.

| Evento de Hook                                         | SDK de Python | SDK de TypeScript | Qué lo dispara                                                                                                                                                         | Caso de uso de ejemplo                                                                                                                                                                 |
| ------------------------------------------------------ | ------------- | ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | Sí            | Sí                | Solicitud de llamada de herramienta (puede bloquear o modificar)                                                                                                       | Bloquear comandos de shell peligrosos                                                                                                                                                  |
| `PostToolUse`                                          | Sí            | Sí                | Resultado de ejecución de herramienta                                                                                                                                  | Registrar todos los cambios de archivo en pista de auditoría                                                                                                                           |
| `PostToolUseFailure`                                   | Sí            | Sí                | Fallo de ejecución de herramienta                                                                                                                                      | Manejar o registrar errores de herramienta                                                                                                                                             |
| `PostToolBatch`                                        | No            | Sí                | Un lote completo de llamadas de herramienta se resuelve, una vez por lote antes de la siguiente llamada del modelo                                                     | Inyectar convenciones una vez para todo el lote                                                                                                                                        |
| `UserPromptSubmit`                                     | Sí            | Sí                | Envío de solicitud del usuario                                                                                                                                         | Inyectar contexto adicional en solicitudes                                                                                                                                             |
| [`UserPromptExpansion`](/docs/es/hooks#userpromptexpansion) | No            | Sí                | Un comando escrito por el usuario, o un prompt de MCP, se expande en una solicitud antes de llegar a Claude. No se dispara cuando Claude invoca una skill por sí mismo | Bloquear un comando de invocación directa o agregar contexto cuando se escribe una skill                                                                                               |
| `MessageDisplay`                                       | No            | Sí                | Un mensaje del asistente con texto se completa, una vez por mensaje con el texto completo del mensaje                                                                  | Redactar o reformatear el texto mostrado sin cambiar la transcripción                                                                                                                  |
| `Stop`                                                 | Sí            | Sí                | Detención de ejecución del agente                                                                                                                                      | Guardar estado de sesión antes de salir                                                                                                                                                |
| `StopFailure`                                          | No            | Sí                | El turno termina con un error de API en lugar de una detención normal                                                                                                  | Registrar fallos o enviar alertas                                                                                                                                                      |
| `SubagentStart`                                        | Sí            | Sí                | Inicialización de subagente                                                                                                                                            | Rastrear generación de tareas paralelas                                                                                                                                                |
| `SubagentStop`                                         | Sí            | Sí                | Finalización de subagente                                                                                                                                              | Agregar resultados de tareas paralelas                                                                                                                                                 |
| `PreCompact`                                           | Sí            | Sí                | Solicitud de compactación de conversación                                                                                                                              | Archivar transcripción completa antes de resumir                                                                                                                                       |
| `PostCompact`                                          | No            | Sí                | Compactación de conversación se completa                                                                                                                               | Registrar el resumen generado                                                                                                                                                          |
| [`PreModelSwitch`](/docs/es/hooks#premodelswitch)           | No            | Sí                | Un cambio de modelo solicitado, antes de que suceda (puede bloquear)                                                                                                   | Bloquear el cambio a un modelo específico                                                                                                                                              |
| [`PostModelSwitch`](/docs/es/hooks#postmodelswitch)         | No            | Sí                | El modelo de la sesión cambia, incluyendo una alternancia automática                                                                                                   | Dar a Claude orientación específica del modelo para el nuevo modelo                                                                                                                    |
| `PermissionRequest`                                    | Sí            | Sí                | Una llamada de herramienta necesita una decisión de permiso                                                                                                            | Manejo de permisos personalizado                                                                                                                                                       |
| `PermissionDenied`                                     | No            | Sí                | El modo automático deniega una llamada de herramienta, incluyendo denegaciones sin un veredicto del clasificador                                                       | Registrar denegaciones, o decirle al modelo que puede reintentar; Claude Code ignora `retry: true` para denegaciones sin veredicto. Ver [PermissionDenied](/docs/es/hooks#permissiondenied) |
| `SessionStart`                                         | No            | Sí                | Inicialización de sesión                                                                                                                                               | Inicializar registro y telemetría                                                                                                                                                      |
| `SessionEnd`                                           | No            | Sí                | Terminación de sesión                                                                                                                                                  | Limpiar recursos temporales                                                                                                                                                            |
| `Notification`                                         | Sí            | Sí                | Mensajes de estado del agente                                                                                                                                          | Enviar actualizaciones de estado del agente a Slack o PagerDuty                                                                                                                        |
| `Setup`                                                | No            | Sí                | Configuración/mantenimiento de sesión                                                                                                                                  | Ejecutar tareas de inicialización                                                                                                                                                      |
| `TeammateIdle`                                         | No            | Sí                | El compañero se vuelve inactivo                                                                                                                                        | Reasignar trabajo o notificar                                                                                                                                                          |
| `TaskCreated`                                          | No            | Sí                | Se crea una tarea a través de la herramienta `TaskCreate`                                                                                                              | Aplicar convenciones de nomenclatura de tareas                                                                                                                                         |
| [`TaskCompleted`](/docs/es/hooks#taskcompleted)             | No            | Sí                | Una tarea se marca como completada                                                                                                                                     | Requerir pruebas exitosas antes de que se cierre una tarea                                                                                                                             |
| `Elicitation`                                          | No            | Sí                | Un servidor MCP solicita entrada del usuario a mitad de la tarea                                                                                                       | Responder a solicitudes de entrada de MCP mediante programación                                                                                                                        |
| `ElicitationResult`                                    | No            | Sí                | Un usuario responde a una elicitación de MCP                                                                                                                           | Modificar o bloquear la respuesta antes de que regrese al servidor                                                                                                                     |
| `ConfigChange`                                         | No            | Sí                | Archivo de configuración cambia                                                                                                                                        | Recargar configuración dinámicamente                                                                                                                                                   |
| `InstructionsLoaded`                                   | No            | Sí                | Se carga un archivo `CLAUDE.md` o de reglas en el contexto                                                                                                             | Auditar qué archivos de instrucciones se cargan                                                                                                                                        |
| `WorktreeCreate`                                       | No            | Sí                | Git worktree creado                                                                                                                                                    | Rastrear espacios de trabajo aislados                                                                                                                                                  |
| `WorktreeRemove`                                       | No            | Sí                | Git worktree eliminado                                                                                                                                                 | Limpiar recursos de espacio de trabajo                                                                                                                                                 |
| `CwdChanged`                                           | No            | Sí                | El directorio de trabajo cambia durante una sesión                                                                                                                     | Recargar variables de entorno por directorio                                                                                                                                           |
| `FileChanged`                                          | No            | Sí                | Un archivo vigilado se modifica, crea o elimina                                                                                                                        | Recargar configuración cuando cambian archivos del proyecto                                                                                                                            |
| `DirectoryAdded`                                       | No            | Sí                | Se agrega un directorio de trabajo durante una sesión                                                                                                                  | Instalar dependencias para un repositorio agregado a mitad de sesión                                                                                                                   |

<h2 id="configure-hooks">
  Configurar hooks
</h2>

Para configurar un hook, páselo en el campo `hooks` de sus opciones de agente (`ClaudeAgentOptions` en Python, el objeto `options` en TypeScript). Este fragmento asume que ya ha definido una devolución de llamada de hook, como `protect_env_files` en Python o `protectEnvFiles` en TypeScript del ejemplo anterior:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

La opción `hooks` es un diccionario en Python u objeto en TypeScript, donde:

* **Las claves**: [nombres de eventos de hook](#available-hooks) como `'PreToolUse'`, `'PostToolUse'` y `'Stop'`
* **Los valores**: matrices de [matchers](#matchers), cada una conteniendo un patrón de filtro opcional y sus [funciones de devolución de llamada](#callback-functions)

<h3 id="matchers">
  Matchers
</h3>

Use matchers para filtrar cuándo se disparan sus devoluciones de llamada. El campo `matcher` coincide con un valor diferente dependiendo del tipo de evento de hook. Por ejemplo, los hooks basados en herramientas coinciden con el nombre de la herramienta, mientras que los hooks `Notification` coinciden con el tipo de notificación.

Los matchers del SDK siguen las mismas reglas que [matchers en archivos de configuración](/docs/es/hooks#matcher-patterns). Esa sección documenta las rutas de evaluación de cadena exacta y expresión regular, sus requisitos de versión, y los valores de matcher para cada tipo de evento.

| Opción    | Tipo             | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| --------- | ---------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `matcher` | `string`         | `undefined`    | Patrón coincidido contra el campo de filtro del evento, siguiendo las [reglas para matchers en archivos de configuración](/docs/es/hooks#matcher-patterns). Para hooks de herramientas, este es el nombre de la herramienta. Las herramientas integradas incluyen `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` y otros (vea [Tipos de entrada de herramienta](/docs/es/agent-sdk/typescript#tool-input-types) para la lista completa). Las herramientas MCP usan el patrón `mcp__<server>__<action>`, donde `<server>` es la clave que usa en la configuración `mcpServers`. |
| `hooks`   | `HookCallback[]` | -              | Requerido. Matriz de funciones de devolución de llamada a ejecutar cuando el patrón coincide                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `timeout` | `number`         | `undefined`    | Tiempo de espera en segundos. Cuando se omite, Claude Code aplica el [tiempo de espera predeterminado del evento](#hook-timeout). Sus devoluciones de llamada del SDK siguen los valores predeterminados del hook `command`                                                                                                                                                                                                                                                                                                                                                                |

Use el patrón `matcher` para dirigirse a herramientas específicas siempre que sea posible. Un matcher con `'Bash'` solo se ejecuta para comandos Bash, mientras que omitir el patrón ejecuta sus devoluciones de llamada para cada ocurrencia del evento. Omítalo a propósito para registrar cada llamada de herramienta que su sesión realiza.

<h3 id="callback-functions">
  Funciones de devolución de llamada
</h3>

<h4 id="inputs">
  Entradas
</h4>

Cada devolución de llamada de hook recibe tres argumentos:

* **Datos de entrada:** un objeto tipado que contiene detalles del evento. Cada tipo de hook tiene su propia forma de entrada. Por ejemplo, `PreToolUseHookInput` incluye `tool_name` y `tool_input`, mientras que `NotificationHookInput` incluye `message`. Vea las definiciones de tipo completas en las referencias del SDK de [TypeScript](/docs/es/agent-sdk/typescript#hookinput) y [Python](/docs/es/agent-sdk/python#hookinput).
  * Todas las entradas de hook comparten `session_id`, `cwd` y `hook_event_name`.
  * `agent_id` y `agent_type` se rellenan cuando el hook se dispara dentro de un subagente. En TypeScript, estos están en la entrada de hook base y disponibles para todos los tipos de hook. En Python, son campos opcionales en `PreToolUse`, `PostToolUse`, `PostToolUseFailure` y `PermissionRequest`, y campos requeridos en `SubagentStart` y `SubagentStop`.
* **ID de uso de herramienta** (`str | None` / `string | undefined`): correlaciona eventos `PreToolUse` y `PostToolUse` para la misma llamada de herramienta.
* **Contexto:** en TypeScript, contiene una propiedad `signal` (`AbortSignal`) para cancelación. En Python, este argumento está reservado para uso futuro.

<h4 id="outputs">
  Salidas
</h4>

Su devolución de llamada devuelve un objeto con dos categorías de campos:

* **Campos de nivel superior** se aceptan en cada evento: `systemMessage` muestra un mensaje al usuario, y `continue` (`continue_` en Python) determina si el agente sigue ejecutándose después de este hook. Algunos eventos los descartan o los entregan en otro lugar. Cada [sección del evento](/docs/es/hooks#hook-events) en la página de hooks dice dónde llegan.
* **`hookSpecificOutput`** controla la operación actual. Los campos que establece dentro dependen del tipo de evento de hook:
  * Para hooks `PreToolUse`, aquí es donde establece `permissionDecision` (`"allow"`, `"deny"`, `"ask"` o `"defer"`), `permissionDecisionReason` e `updatedInput`. Si devuelve `"defer"`, la consulta termina para que pueda [reanudarla más tarde](/docs/es/hooks#defer-a-tool-call-for-later).
  * Para hooks `PostToolUse`, puede establecer `additionalContext` para agregar información al resultado de la herramienta. Para reemplazar la salida de la herramienta antes de que Claude la vea, establezca `updatedToolOutput`, que funciona para cualquier herramienta en ambos SDK. El campo anterior `updatedMCPToolOutput` reemplaza solo la salida de herramientas MCP y está deprecado.
  * En el SDK de TypeScript, una devolución de llamada `PostToolUse` también puede devolver `classifierContext`, una nota breve sobre el resultado de la llamada de herramienta para el clasificador de permisos del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode). Debido a que su devolución de llamada se ejecuta en el proceso propio de su aplicación, el clasificador puede pesar una declaración de usuario que retransmita en la nota como intención del usuario. El campo requiere TypeScript Agent SDK v0.3.236 o posterior. [Anotar un resultado para el clasificador del modo automático](/docs/es/hooks#annotate-a-result-for-the-auto-mode-classifier) cubre el límite de longitud, la regla de solo sincrónico, y qué no poner en la nota.

Devuelva `{}` para permitir la operación sin cambios. Los hooks de devolución de llamada del SDK usan el mismo formato de salida JSON que [hooks de comandos de shell de Claude Code](/docs/es/hooks#json-output), que documenta cada campo y opción específica del evento. Para las definiciones de tipo del SDK, vea las referencias del SDK de [TypeScript](/docs/es/agent-sdk/typescript#synchookjsonoutput) y [Python](/docs/es/agent-sdk/python#synchookjsonoutput).

<Note>
  Cuando se aplican múltiples hooks o reglas de permiso, `deny` tiene prioridad sobre `defer`, que tiene prioridad sobre `ask`, que tiene prioridad sobre `allow`. Si algún hook devuelve `deny`, la operación se bloquea independientemente de otros hooks.
</Note>

<h4 id="asynchronous-output">
  Salida asincrónica
</h4>

De forma predeterminada, el agente espera a que su hook devuelva antes de continuar. Si su hook realiza un efecto secundario, como registro o envío de webhook, y no necesita influir en el comportamiento del agente, puede devolver una salida asincrónica en su lugar. Esto le dice al agente que continúe inmediatamente sin esperar a que el hook termine. En este fragmento, `send_to_logging_service` en Python y `sendToLoggingService` en TypeScript representan cualquier función de registro que defina:

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Campo          | Tipo     | Descripción                                                                                                              |
| -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------ |
| `async`        | `true`   | Señala modo asincrónico. El agente continúa sin esperar. En Python, use `async_` para evitar la palabra clave reservada. |
| `asyncTimeout` | `number` | Tiempo de espera opcional en milisegundos para la operación de fondo                                                     |

<Note>
  Las salidas asincrónicas no pueden bloquear, modificar o inyectar contexto en la operación ya que el agente ya ha avanzado. Úselas solo para efectos secundarios como registro, métricas o notificaciones.
</Note>

<h2 id="examples">
  Ejemplos
</h2>

Varios ejemplos en esta sección muestran solo la función de devolución de llamada. Para ejecutar uno, registre la devolución de llamada bajo el evento coincidente en el campo `hooks` de sus opciones, como se muestra en [Configurar hooks](#configure-hooks).

<h3 id="modify-tool-input">
  Modificar entrada de herramienta
</h3>

Este ejemplo intercepta llamadas de herramienta Write y reescribe el argumento `file_path` para anteponer `/sandbox`, redirigiendo todas las escrituras de archivo a un directorio aislado. La devolución de llamada devuelve `updatedInput` con la ruta modificada y `permissionDecision: 'allow'` para aprobar automáticamente la operación reescrita:

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  Empareje `updatedInput` con `permissionDecision: 'allow'` para aprobar automáticamente la entrada modificada, o `permissionDecision: 'ask'` para mostrársela al usuario. Si omite `permissionDecision`, la entrada modificada aún se aplica y fluye a través de la evaluación de permiso normal. Con `'defer'`, `updatedInput` se ignora. Siempre devuelva un nuevo objeto en lugar de mutar el `tool_input` original.
</Note>

Para confirmar la redirección, establezca el prefijo en una ruta en la que pueda escribir, como `./sandbox` o `/tmp/sandbox` (macOS no permite crear un directorio `/sandbox` a nivel raíz), luego pida al agente que escriba un archivo: el resultado de la herramienta Write en el flujo de mensajes nombra la ruta con su prefijo de sandbox en lugar del que Claude solicitó.

<h3 id="add-context-and-block-a-tool">
  Agregar contexto y bloquear una herramienta
</h3>

Este ejemplo bloquea escrituras en el directorio `/etc` y explica por qué tanto al modelo como al usuario:

* `permissionDecision: 'deny'` detiene la llamada de herramienta.
* `permissionDecisionReason` le dice al modelo por qué, para que evite reintentar.
* `systemMessage` muestra al usuario qué sucedió.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Aprobar automáticamente herramientas específicas
</h3>

De forma predeterminada, el agente puede solicitar permiso antes de usar ciertas herramientas. Este ejemplo aprueba automáticamente herramientas del sistema de archivos de solo lectura (Read, Glob, Grep) devolviendo `permissionDecision: 'allow'`, permitiéndoles ejecutarse sin confirmación del usuario mientras deja todas las otras herramientas sujetas a verificaciones de permiso normales:

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Registrar múltiples hooks
</h3>

Cuando se dispara un evento, todos los hooks coincidentes se ejecutan en paralelo. Para decisiones de permiso, el resultado más restrictivo se aplica: un único `deny` bloquea la llamada de herramienta independientemente de lo que devuelvan los otros hooks. Debido a que el orden de finalización es no determinista, escriba cada hook para actuar de forma independiente en lugar de depender de que otro hook se haya ejecutado primero.

El ejemplo a continuación registra tres verificaciones independientes para cada llamada de herramienta:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Filtrar con matchers de múltiples herramientas
</h3>

Use matchers de múltiples herramientas para compartir una devolución de llamada entre herramientas relacionadas. Este ejemplo registra tres matchers con diferentes alcances:

* Una lista exacta separada por tuberías (`Write|Edit|NotebookEdit`) dispara `file_security_hook` solo para herramientas de modificación de archivos.
* Una expresión regular (`^mcp__`) dispara `mcp_audit_hook` para cualquier herramienta MCP cuyo nombre comience con `mcp__`.
* Un matcher omitido dispara `global_logger` para cada llamada de herramienta independientemente del nombre.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|NotebookEdit", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|NotebookEdit", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Rastrear actividad de subagente
</h3>

Use hooks `SubagentStop` para monitorear cuándo los subagentes terminan su trabajo. Vea el tipo de entrada completo en las referencias del SDK de [TypeScript](/docs/es/agent-sdk/typescript#hookinput) y [Python](/docs/es/agent-sdk/python#hookinput). Este ejemplo registra un resumen cada vez que un subagente se completa:

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  Realizar solicitudes HTTP desde hooks
</h3>

Los hooks pueden realizar operaciones asincrónicas como solicitudes HTTP. Capture errores dentro de su hook en lugar de dejarlos propagarse.

Este ejemplo envía un webhook después de que cada herramienta se completa, registrando qué herramienta se ejecutó y cuándo. El hook captura errores de un webhook fallido:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Para confirmar que el hook se dispara, apunte la URL del webhook a un punto final que pueda observar y envíe un mensaje que use una herramienta: el hook envía un POST con el nombre de la herramienta y la marca de tiempo después de que cada herramienta se completa.

<h3 id="forward-notifications-to-slack">
  Reenviar notificaciones a Slack
</h3>

Use hooks `Notification` para recibir notificaciones del sistema del agente y reenviarlas a servicios externos. En sesiones del SDK, Claude Code ejecuta este hook para los siguientes tipos de notificación:

* [`permission_prompt`](/docs/es/hooks#notification) una vez que una solicitud de permiso ha esperado aproximadamente seis segundos en su devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input). Requiere TypeScript Agent SDK v0.3.233 o posterior, o Python Agent SDK v0.2.139 o posterior
* `elicitation_complete` y `elicitation_response` para flujos de elicitación de solicitud del usuario

Claude Code emite los otros tipos, como `idle_prompt`, `auth_success` y `elicitation_dialog`, desde la interfaz de usuario interactiva que las sesiones del SDK no ejecutan.

Cada notificación incluye un campo `message` con una descripción legible por humanos y opcionalmente un `title`.

Este ejemplo reenvía cada notificación a un canal de Slack. Requiere una [URL de webhook entrante de Slack](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/), que crea agregando una aplicación a su espacio de trabajo de Slack y habilitando webhooks entrantes:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Cuando se dispara un evento `Notification`, el hook publica el `message` de la notificación, con el prefijo `Agent status:`, al canal que apunta su webhook.

<h2 id="fix-common-issues">
  Solucionar problemas comunes
</h2>

<h3 id="hook-not-firing">
  Hook no se dispara
</h3>

* Verifique que el nombre del evento de hook sea correcto y sensible a mayúsculas (`PreToolUse`, no `preToolUse`)
* Verifique que su patrón de matcher coincida exactamente con el nombre de la herramienta
* Asegúrese de que el hook esté bajo el tipo de evento correcto en `options.hooks`
* Para hooks que no son de herramientas que admiten matchers, como `Notification` y `SubagentStop`, los matchers coinciden contra campos diferentes, y `Stop` ignora los matchers por completo (vea [patrones de matcher](/docs/es/hooks#matcher-patterns))
* Los hooks pueden no dispararse cuando el agente alcanza el límite [`max_turns`](/docs/es/agent-sdk/python#claudeagentoptions) porque la sesión termina antes de que los hooks puedan ejecutarse

<h3 id="matcher-not-filtering-as-expected">
  Matcher no filtra como se esperaba
</h3>

Los matchers solo coinciden con nombres de herramientas, no con rutas de archivo u otros argumentos. Para filtrar por ruta de archivo, verifique `tool_input.file_path` dentro de su hook:

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Tiempo de espera del hook
</h3>

Claude Code ejecuta cada devolución de llamada con un tiempo de espera, que usted establece en segundos con el campo `timeout` en su `HookMatcher`. Cuando no establece uno, Claude Code utiliza el valor predeterminado del evento: 600 segundos para la mayoría de eventos, 30 segundos para `UserPromptSubmit`, `PreModelSwitch` y `PostModelSwitch`, y 10 segundos para `MessageDisplay`. Claude Code ejecuta devoluciones de llamada `SessionEnd` durante el apagado bajo el [presupuesto de tiempo de espera SessionEnd](/docs/es/hooks#sessionend-input) más corto, 1,5 segundos de forma predeterminada.

Cuando una devolución de llamada excede su tiempo de espera, Claude Code la cancela y descarta su salida, y la sesión continúa en lugar de colgarse. Lo que sucede a continuación depende del evento:

* `PreToolUse`: Claude Code no ejecuta la llamada de herramienta, Claude recibe un resultado de herramienta indicando que el hook no respondió antes de su tiempo de espera, y el turno continúa. Si otro hook `PreToolUse` devolvió una denegación explícita, Claude recibe esa denegación en lugar del error de tiempo de espera. Antes de v2.1.210, Claude Code reportaba el tiempo de espera a Claude como un rechazo del usuario, lo que hacía que las sesiones desatendidas se detuvieran y esperaran entrada.
* `PostToolUse` y `PostToolUseFailure`: Claude Code mantiene el resultado de la herramienta y el turno continúa.
* `UserPromptSubmit` y [`UserPromptExpansion`](/docs/es/hooks#userpromptexpansion): Claude Code bloquea el mensaje con un mensaje que nombra el hook y el tiempo de espera, y la sesión continúa. Debido a que una devolución de llamada en estos eventos puede actuar como una puerta de política, Claude Code nunca permite que un mensaje con tiempo de espera agotado pase sin ser revisado. Antes de v2.1.208, Claude Code terminaba la consulta con `error_during_execution` cuando una devolución de llamada en estos eventos agotaba el tiempo de espera.
* `Stop` y `SubagentStop`: la devolución de llamada con tiempo de espera agotado cuenta como no devolver ninguna decisión. El agente o subagente se detiene como si esa devolución de llamada lo hubiera permitido, y una decisión de sus otros hooks en el evento aún se aplica. Antes de Claude Code v2.1.273, una devolución de llamada `Stop` o `SubagentStop` con tiempo de espera agotado contaba como una ejecución de hook fallida, y Claude Code descartaba las decisiones de sus otros hooks en el evento.
* `SessionStart`: la devolución de llamada con tiempo de espera agotado cuenta como no devolver ninguna salida, y la sesión continúa con la salida de sus otros hooks `SessionStart`.
* `PreModelSwitch`: Claude Code bloquea el cambio de modelo. Un hook que no responde no ha aprobado el cambio.
* Otros eventos, como `Notification`, `PreCompact` y `PostModelSwitch`: Claude Code registra el fallo y continúa.

La primera vez que una devolución de llamada `Stop` o `SessionStart` agota el tiempo de espera en la sesión principal, Claude Code también agrega un [`SDKInformationalMessage`](/docs/es/agent-sdk/typescript#sdkinformationalmessage) al flujo de mensajes diciendo que la aplicación que impulsa la sesión no respondió. Los tiempos de espera posteriores no repiten ese mensaje mientras su aplicación permanece sin responder.

Si interrumpe la consulta mientras una devolución de llamada está pendiente, Claude Code cancela la llamada de herramienta pendiente. Antes de v2.1.208, la llamada de herramienta podría proceder si interrumpía durante una devolución de llamada `PreToolUse` pendiente.

Si su devolución de llamada necesita más tiempo, establezca un `timeout` más alto en su `HookMatcher`. En TypeScript, use el `AbortSignal` del tercer argumento de devolución de llamada para manejar la cancelación correctamente cuando se agote el tiempo de espera.

<h3 id="tool-blocked-unexpectedly">
  Herramienta bloqueada inesperadamente
</h3>

* Verifique todos los hooks `PreToolUse` para devoluciones de `permissionDecision: 'deny'`
* Agregue registro a sus hooks para ver qué `permissionDecisionReason` están devolviendo
* Verifique que los patrones de matcher no sean demasiado amplios: un matcher vacío coincide con todas las herramientas

<h3 id="modified-input-not-applied">
  Entrada modificada no aplicada
</h3>

* Asegúrese de que `updatedInput` esté dentro de `hookSpecificOutput`, no en el nivel superior:

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* No empareje `updatedInput` con `permissionDecision: 'defer'`, que descarta la entrada modificada. Omitir `permissionDecision` está bien: la entrada modificada aún se aplica a través de la evaluación de permiso normal. También puede devolver `'allow'` para aprobar automáticamente la entrada modificada u `'ask'` para mostrarla al usuario para su aprobación

* Incluya `hookEventName` en `hookSpecificOutput` para identificar para qué tipo de hook es la salida

<h3 id="session-hooks-not-available-in-python">
  Hooks de sesión no disponibles en Python
</h3>

`SessionStart` y `SessionEnd` pueden registrarse como hooks de devolución de llamada del SDK en TypeScript, pero no están disponibles en el SDK de Python porque su tipo `HookEvent` los omite. En Python, solo están disponibles como [hooks de comandos de shell](/docs/es/hooks#hook-events) definidos en archivos de configuración como `.claude/settings.json`. Para cargar hooks de comandos de shell desde su aplicación SDK, incluya la fuente de configuración apropiada con [`setting_sources`](/docs/es/agent-sdk/python#settingsource) o [`settingSources`](/docs/es/agent-sdk/typescript#settingsource):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Para ejecutar lógica de inicialización como una devolución de llamada del SDK de Python en su lugar, use el primer mensaje de `client.receive_response()` como su disparador.

<h3 id="subagent-permission-prompts-multiplying">
  Solicitudes de permiso de subagente multiplicándose
</h3>

Al generar múltiples subagentes, cada uno puede solicitar permisos por separado para sus propias llamadas de herramienta. Para evitar solicitudes repetidas, use hooks `PreToolUse` para aprobar automáticamente herramientas específicas, o configure reglas de permiso, que los subagentes [heredan de la conversación padre](/docs/es/sub-agents#permission-modes).

<h3 id="recursive-hook-loops-with-subagents">
  Bucles recursivos de hook con subagentes
</h3>

Un hook `UserPromptSubmit` que genera subagentes puede crear bucles infinitos si esos subagentes disparan el mismo hook. Para prevenir esto:

* Use una variable compartida o estado de sesión para rastrear si ya está dentro de un subagente
* Alcance los hooks para ejecutarse solo para la sesión del agente de nivel superior

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage no aparece en la salida
</h3>

El campo `systemMessage` muestra un mensaje al usuario, no al modelo. En Claude Code v2.1.227 o posterior, el `systemMessage` de un hook puede aparecer en el flujo de mensajes como un [`SDKInformationalMessage`](/docs/es/agent-sdk/typescript#sdkinformationalmessage). Si lo hace depende del evento. Cada [sección del evento](/docs/es/hooks#hook-events) en la página de hooks dice cómo aparece la salida. Para pasar contexto al modelo en su lugar, devuelva [`additionalContext`](/docs/es/hooks#add-context-for-claude).

Antes de v2.1.227, el SDK exponía la salida de hooks en el flujo de mensajes solo para hooks `SessionStart` y `Setup`. Para cualquier otro evento, la salida aparecía solo en los eventos del ciclo de vida que [`includeHookEvents`](/docs/es/agent-sdk/typescript#options) (`include_hook_events` en Python) agrega. La entrada de esa opción cubre qué eventos del ciclo de vida produce cada evento de hook.

Si necesita exponer decisiones de hook a su aplicación de manera confiable, regístrelas por separado o use un canal de salida dedicado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Referencia de hooks de Claude Code](/docs/es/hooks): esquemas completos de entrada/salida JSON, documentación de eventos y patrones de matcher
* [Guía de hooks de Claude Code](/docs/es/hooks-guide): ejemplos de hooks de comandos de shell y tutoriales
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): tipos de hook, definiciones de entrada/salida y opciones de configuración
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): tipos de hook, definiciones de entrada/salida y opciones de configuración
* [Permisos](/docs/es/agent-sdk/permissions): controlar qué puede hacer su agente
* [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools): crear herramientas para extender las capacidades del agente
