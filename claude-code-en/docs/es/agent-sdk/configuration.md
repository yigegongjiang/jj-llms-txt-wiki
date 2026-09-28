> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura tu agente

> Configura sesiones del Agent SDK: compone el objeto de opciones, establece el modelo, el entorno y los límites, y encuentra la página de cada opción de característica.

Una sesión del Agent SDK lee la configuración desde archivos de configuración, variables de entorno y el objeto `options` que pasas cuando la inicias. Esta página muestra cómo componer el objeto `options` y qué archivos de configuración y variables de entorno lo controlan.

Para cada tipo de opción y valor predeterminado, consulta las referencias [`Options`](/docs/es/agent-sdk/typescript#options) (TypeScript) y [`ClaudeAgentOptions`](/docs/es/agent-sdk/python#claudeagentoptions) (Python).

<h2 id="pass-options-to-a-session">
  Pasar opciones a una sesión
</h2>

Cada llamada a `query()` acepta un objeto de opciones: `Options` en TypeScript, `ClaudeAgentOptions` en Python. Cada campo es opcional, y una sesión iniciada sin opciones se ejecuta con los valores predeterminados del SDK. El ejemplo a continuación configura una sesión de solo lectura que resume los TODOs abiertos de un proyecto. Los pares se leen como TypeScript / Python donde los nombres difieren:

* **`model`**: elige el modelo
* **`allowedTools` / `allowed_tools`**: aprueba previamente una lista de herramientas de solo lectura
* **`maxTurns` / `max_turns`**: limita el número de turnos
* **`cwd`**: establece el directorio de trabajo

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

Apunta `cwd` a uno de tus propios proyectos y ejecuta el ejemplo. El resumen de los TODOs abiertos de ese proyecto se imprime cuando llega el mensaje de resultado.

`allowedTools` (TypeScript) o `allowed_tools` (Python) aprueba previamente las herramientas listadas, por lo que las llamadas a ellas se ejecutan sin detenerse para solicitar aprobación. Las herramientas fuera de la lista siguen disponibles. Cuando Claude llama a una herramienta no listada, el modo de permisos decide si la llamada se ejecuta. Para más información, consulta [Reglas de permitir y denegar](/docs/es/agent-sdk/permissions#allow-and-deny-rules).

<h2 id="load-settings-files">
  Cargar archivos de configuración
</h2>

Los archivos de configuración proporcionan configuración más allá del objeto de opciones. Dos opciones controlan cómo se cargan:

* **`settingSources` / `setting_sources`**: controla qué fuentes del sistema de archivos se cargan: usuario, proyecto y local. Los archivos de configuración y los archivos CLAUDE.md llegan a través de estas fuentes.
* **`settings`**: carga una ruta de archivo de configuración o una cadena JSON en línea en cualquier idioma, y TypeScript también acepta un objeto de configuración. Sea cual sea la forma que pases, anula la configuración del sistema de archivos de usuario, proyecto y local; solo la configuración de política administrada tiene un rango más alto. Las referencias documentan el orden de precedencia completo bajo [Precedencia de configuración](/docs/es/agent-sdk/typescript#settings-precedence) para TypeScript y [Precedencia de configuración](/docs/es/agent-sdk/python#settings-precedence) para Python.

Pasa `[]` para desactivar la configuración de usuario, proyecto y local. Para más información, consulta [Usar características de Claude Code en el SDK](/docs/es/agent-sdk/claude-code-features).

<h2 id="choose-a-model">
  Elige un modelo
</h2>

A menos que la opción `model`, tu configuración o tu entorno seleccione un modelo, una nueva sesión comienza en [el modelo predeterminado de Claude Code](/docs/es/model-config#default-model-setting). Para el orden de esas fuentes, consulta [Establecer tu modelo](/docs/es/model-config#setting-your-model). Establece `model` para fijar un modelo específico, o para elegir uno más pequeño para agentes más rápidos y económicos. El valor toma un alias de modelo o un nombre de modelo completo; los alias y las versiones a las que se resuelven se enumeran bajo [Alias de modelos](/docs/es/model-config#model-aliases).

Establece `fallbackModel` (TypeScript) o `fallback_model` (Python) para nombrar un modelo de respaldo. Cuando el principal está sobrecargado o no disponible, la sesión cambia al respaldo. El principal se reintenta al inicio de cada turno del usuario, por lo que la sesión vuelve a él una vez que la interrupción pasa.

En cualquier idioma, la opción acepta un único modelo o una lista separada por comas de respaldos. Para el orden y el límite de cadena, consulta [Cadenas de modelo de respaldo](/docs/es/model-config#fallback-model-chains). En TypeScript, un respaldo igual a `model` lanza un error al inicio.

Los ejemplos a continuación muestran una lista de respaldo en TypeScript y un único respaldo en Python:

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  Los parámetros de solicitud de la [API de Mensajes](https://platform.claude.com/docs/en/api/messages) `temperature`, `top_p` y `max_tokens` no tienen campos en el objeto de opciones en ninguno de los idiomas. Establece el [nivel de esfuerzo](/docs/es/agent-sdk/agent-loop#effort-level) o un [límite de gasto](#limit-turns-and-spend) en su lugar, o llama a la API de Mensajes cuando necesites esos parámetros directamente.
</Note>

<h2 id="set-environment-variables">
  Establecer variables de entorno
</h2>

La opción `env` establece variables de entorno para el proceso de Claude Code que ejecuta tu sesión. Si tus valores reemplazan el entorno heredado o se fusionan con él difiere según el idioma:

* **TypeScript**: `env` reemplaza el entorno del subproceso
* **Python**: el SDK fusiona tus valores sobre el entorno heredado, y tus valores anulan los heredados

En TypeScript, expande `process.env` en `env` para mantener variables heredadas como `PATH`, `HOME` y `ANTHROPIC_API_KEY`. Cuando dejas `env` sin establecer, el subproceso hereda tu entorno en ambos idiomas.

El ejemplo enruta el tráfico de API a través de una puerta de enlace estableciendo `ANTHROPIC_BASE_URL`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

Las variables que pasas también pueden configurar Claude Code en sí. Para las variables que el proceso de Claude Code lee, consulta [Variables de entorno](/docs/es/env-vars). Para ajustar los tiempos de espera de API y la detección de estancamiento de esta manera, sigue la sección Manejar respuestas de API lentas o estancadas en la [referencia de TypeScript](/docs/es/agent-sdk/typescript#handle-slow-or-stalled-api-responses) o la [referencia de Python](/docs/es/agent-sdk/python#handle-slow-or-stalled-api-responses).

<h2 id="set-the-working-directory">
  Establecer el directorio de trabajo
</h2>

Establece `cwd` para ejecutar la sesión en un directorio específico. Cuando dejas `cwd` sin establecer, la sesión se ejecuta en el directorio de trabajo de tu proceso. Ninguno de los SDK tiene un setter para `cwd`. Para ejecutar en un directorio diferente, inicia otra sesión con ese `cwd`.

Claude Code lee el directorio de trabajo para determinar:

* **Configuración y hooks del proyecto**: qué [configuración y hooks del proyecto se cargan](/docs/es/agent-sdk/claude-code-features)
* **Skills**: dónde [se descubren las skills de la sesión](/docs/es/agent-sdk/skills)
* **Almacenamiento de sesión**: a qué proyecto [pertenece una sesión almacenada](/docs/es/agent-sdk/session-storage)

Para permitir que las herramientas accedan a archivos fuera del directorio de trabajo, agrega rutas con `additionalDirectories` (TypeScript) o `add_dirs` (Python). Para el alcance de esa concesión, consulta [Los directorios adicionales otorgan acceso a archivos, no configuración](/docs/es/permissions#additional-directories-grant-file-access-not-configuration).

<h2 id="limit-turns-and-spend">
  Limitar turnos y gasto
</h2>

Limita turnos y gasto con `maxTurns` / `max_turns` y `maxBudgetUsd` / `max_budget_usd`. Ambos límites están desactivados cuando no se establecen. Cuando una sesión alcanza un límite, la ejecución termina con un mensaje de resultado cuyo subtipo nombra el límite, `error_max_turns` o `error_max_budget_usd`. Lo que sucede después difiere según el modo de entrada:

* **`query()` de un solo disparo**: el SDK produce el resultado del límite y luego lanza, así que envuelve el bucle en un bloque try para continuar más allá del error
* **Entrada de transmisión**: la sesión permanece activa después de un resultado de límite, y el conteo de turnos máximos comienza de nuevo para cada mensaje en cola. El total del presupuesto se acumula entre mensajes, y una vez que el gasto alcanza el límite, los mensajes posteriores en la misma conversación terminan con el mismo resultado de presupuesto. Un [`/clear`](/docs/es/agent-sdk/cost-tracking) comienza el presupuesto de nuevo

Los dos límites tratan `0` de manera diferente:

* **`maxTurns` / `max_turns`**: `0` ejecuta la sesión sin un límite de turnos, lo mismo que dejar la opción sin establecer
* **`maxBudgetUsd` / `max_budget_usd`**: la CLI rechaza `0` como una cantidad inválida al inicio, y la sesión nunca se ejecuta

Para más información sobre ambos límites, incluido el gasto de subagentes, consulta [Turnos y presupuesto](/docs/es/agent-sdk/agent-loop#turns-and-budget).

<h2 id="change-configuration-mid-session">
  Cambiar configuración durante la sesión
</h2>

Cuando inicias una sesión con [entrada de transmisión](/docs/es/agent-sdk/streaming-vs-single-mode), puedes cambiar su modelo y modo de permisos mientras se ejecuta. Dónde llamas a los setters difiere según el idioma:

* **TypeScript**: métodos en el objeto que `query()` devuelve
* **Python**: métodos en [`ClaudeSDKClient`](/docs/es/agent-sdk/python#claudesdkclient), ya que `query()` devuelve un iterador simple sin métodos de control

Ambos idiomas tienen los mismos setters:

* **`setModel()` / `set_model()`**: cambia el modelo. Llámalo sin modelo para cambiar al [modelo predeterminado de Claude Code](/docs/es/model-config#default-model-setting) en lugar del `model` que pasaste en opciones.
* **`setPermissionMode()` / `set_permission_mode()`**: cambia el modo de permisos

TypeScript también tiene `applyFlagSettings()` y `updateSettings()`:

* **`applyFlagSettings()`**: aplica configuración en tiempo de ejecución, como en `await session.applyFlagSettings({ effortLevel: "high" })`. El método toma claves de archivo de configuración en lugar de campos de opciones, así que consulta la [referencia de `applyFlagSettings()`](/docs/es/agent-sdk/typescript#applyflagsettings) para el esquema y para qué claves tienen efecto durante la sesión.
* **`updateSettings()`**: escribe una clave permitida en un archivo de configuración. La [referencia de `updateSettings()`](/docs/es/agent-sdk/typescript#updatesettings) nombra la clave que cada fuente acepta y el piso de versión.
  * Pasa `"localSettings"` para escribir el archivo de configuración local del proyecto, como en `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`. La clave escrita tiene efecto en la siguiente solicitud de la sesión y persiste para sesiones posteriores que cargan configuración `local`.
  * Pasa `"userSettings"` para escribir `effortLevel`, la única clave que esa fuente acepta. Claude Code la guarda como el nivel de esfuerzo predeterminado para el modelo actual de la sesión, y el esfuerzo de la sesión en ejecución no cambia.

El ejemplo a continuación ejecuta una sesión de dos turnos, cambia la configuración entre los turnos e imprime el modelo que respondió cada turno. En TypeScript, la transmisión de solicitud mantiene el segundo mensaje hasta que los setters se hayan ejecutado, y el segundo turno se ejecuta en el nuevo modelo.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

En la API de Claude, el programa imprime `First turn model: claude-sonnet-5`, luego `Second turn model: claude-opus-5` después del cambio.

<Note>
  Cada modelo tiene su propio caché de solicitud, por lo que después de un cambio durante la sesión, la siguiente solicitud recomputa la conversación completa sin caché a las tasas del nuevo modelo. Para más información, consulta [Cambiar modelos](/docs/es/prompt-caching#switching-models).
</Note>

<h2 id="configure-specific-features">
  Configurar características específicas
</h2>

La tabla a continuación asigna cada opción a la característica que configura. Para opciones que esta página no cubre, consulta las referencias de [TypeScript](/docs/es/agent-sdk/typescript#options) y [Python](/docs/es/agent-sdk/python#claudeagentoptions). Si conoces tu objetivo pero no qué opción lo sirve, comienza desde [Elige la característica correcta](/docs/es/agent-sdk/claude-code-features#choose-the-right-feature).

| TypeScript                | Python                      | Controla                                                             | Cubierto en                                                                                                                                                                                                                    |
| ------------------------- | --------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permissionMode`          | `permission_mode`           | Qué puede hacer el agente sin aprobación                             | [Configurar permisos](/docs/es/agent-sdk/permissions)                                                                                                                                                                               |
| `allowedTools`            | `allowed_tools`             | Qué llamadas de herramientas están preaprobadas                      | [Configurar permisos](/docs/es/agent-sdk/permissions)                                                                                                                                                                               |
| `canUseTool`              | `can_use_tool`              | Tu devolución de llamada de aprobación para llamadas de herramientas | [Manejar solicitudes de aprobación de herramientas](/docs/es/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                                    |
| `systemPrompt`            | `system_prompt`             | Las instrucciones del agente                                         | [Modificar solicitudes del sistema](/docs/es/agent-sdk/modifying-system-prompts)                                                                                                                                                    |
| `settingSources`          | `setting_sources`           | Qué configuración del sistema de archivos se carga                   | [Usar características de Claude Code en el SDK](/docs/es/agent-sdk/claude-code-features)                                                                                                                                            |
| `mcpServers`              | `mcp_servers`               | Servidores de herramientas externas                                  | [Conectar a herramientas externas con MCP](/docs/es/agent-sdk/mcp)                                                                                                                                                                  |
| `agents`                  | `agents`                    | Definiciones de subagentes                                           | [Subagentes](/docs/es/agent-sdk/subagents)                                                                                                                                                                                          |
| `hooks`                   | `hooks`                     | Devoluciones de llamada en puntos del ciclo de vida                  | [Hooks](/docs/es/agent-sdk/hooks)                                                                                                                                                                                                   |
| `skills`                  | `skills`                    | Qué skills se cargan                                                 | [Extender agentes con skills](/docs/es/agent-sdk/skills)                                                                                                                                                                            |
| `plugins`                 | `plugins`                   | Qué plugins se cargan                                                | [Plugins](/docs/es/agent-sdk/plugins)                                                                                                                                                                                               |
| `outputFormat`            | `output_format`             | Esquemas de salida estructurada                                      | [Salidas estructuradas](/docs/es/agent-sdk/structured-outputs)                                                                                                                                                                      |
| `resume`                  | `resume`                    | Continuando una sesión almacenada                                    | [Sesiones](/docs/es/agent-sdk/sessions)                                                                                                                                                                                             |
| `forkSession`             | `fork_session`              | Ramificación de una sesión                                           | [Sesiones](/docs/es/agent-sdk/sessions)                                                                                                                                                                                             |
| `sessionStore`            | `session_store`             | Persistencia de sesión externa                                       | [Almacenamiento de sesión](/docs/es/agent-sdk/session-storage)                                                                                                                                                                      |
| `enableFileCheckpointing` | `enable_file_checkpointing` | Ediciones de archivo rebobinables                                    | [Punto de control de archivo](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                |
| `effort`                  | `effort`                    | Cuánto trabajo pone Claude en las respuestas                         | [Nivel de esfuerzo](/docs/es/agent-sdk/agent-loop#effort-level)                                                                                                                                                                     |
| `sandbox`                 | `sandbox`                   | Comportamiento de sandbox para la ejecución de herramientas          | [TypeScript](/docs/es/agent-sdk/typescript#sandbox-configuration) y referencias de [Python](/docs/es/agent-sdk/python#sandbox-configuration), con contexto de implementación en [Implementación segura](/docs/es/agent-sdk/secure-deployment) |

<h2 id="next-steps">
  Próximos pasos
</h2>

Para ver la configuración compuesta en agentes funcionales:

* **[Inicio rápido](/docs/es/agent-sdk/quickstart)**: construye y ejecuta un primer agente de principio a fin
* **[Ejemplos](/docs/es/agent-sdk/examples)**: encuentra un proyecto completo y ejecutable o una receta guiada de Claude Cookbook que coincida con lo que quieres construir
* **[Aislamiento multiinquilino](/docs/es/agent-sdk/hosting#multi-tenant-isolation)**: aísla la configuración y la memoria de cada inquilino con `settingSources` / `setting_sources`, `env` y `cwd`
