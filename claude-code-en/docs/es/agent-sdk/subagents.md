> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagentes en el SDK

> Define e invoque subagentes para aislar contexto, ejecutar tareas en paralelo y aplicar instrucciones especializadas en sus aplicaciones de Claude Agent SDK.

Los subagentes son instancias de agente separadas que su agente principal puede generar para manejar subtareas enfocadas.
Úselos para aislar contexto, ejecutar múltiples análisis en paralelo y aplicar instrucciones especializadas sin agregar a la solicitud del agente principal.

<h2 id="overview">
  Descripción general
</h2>

Puede crear subagentes de tres formas:

* **Programáticamente**: use el parámetro `agents` en sus opciones de `query()`. Consulte las referencias de [TypeScript](/docs/es/agent-sdk/typescript#agentdefinition) y [Python](/docs/es/agent-sdk/python#agentdefinition)
* **Basado en el sistema de archivos**: defina agentes como archivos markdown en directorios `.claude/agents/`. Consulte [definir subagentes como archivos](/docs/es/sub-agents)
* **Propósito general integrado**: Claude puede invocar el subagente `general-purpose` integrado en cualquier momento a través de la herramienta Agent sin que usted defina nada

Esta guía se enfoca en el enfoque programático, que se recomienda para aplicaciones SDK.

<h2 id="benefits-of-using-subagents">
  Beneficios de usar subagentes
</h2>

Debido a que los subagentes son instancias de agente separadas, delegar trabajo a ellos le proporciona cuatro beneficios:

* **Aislamiento de contexto**: cada subagente se ejecuta en su propia conversación, que comienza de cero a menos que el subagente sea un [fork](/docs/es/sub-agents#fork-the-current-conversation). De cualquier forma, las llamadas a herramientas intermedias y los resultados permanecen dentro del subagente; solo su mensaje final regresa al padre. Un subagente `research-assistant` puede explorar docenas de archivos sin que ninguno de esos contenidos se acumule en la conversación principal. El padre recibe un resumen conciso, no cada archivo que leyó el subagente. Consulte [What subagents inherit](#what-subagents-inherit) para ver exactamente qué hay en el contexto del subagente.
* **Paralelización**: múltiples subagentes pueden ejecutarse simultáneamente, por lo que las subtareas independientes se completan en el tiempo del más lento en lugar de la suma de todos ellos. Durante una revisión de código, puede ejecutar los subagentes `style-checker`, `security-scanner` y `test-coverage` simultáneamente en lugar de secuencialmente.
* **Instrucciones y conocimiento especializados**: cada subagente puede tener un prompt del sistema personalizado con experiencia específica, mejores prácticas y restricciones. Un subagente `database-migration` puede tener conocimiento detallado sobre mejores prácticas de SQL, estrategias de reversión y verificaciones de integridad de datos que serían ruido innecesario en las instrucciones del agente principal.
* **Restricciones de herramientas**: los subagentes pueden limitarse a herramientas específicas, reduciendo el riesgo de acciones no intencionadas. Un subagente `doc-reviewer` podría tener acceso solo a las herramientas Read y Grep, asegurando que pueda analizar pero nunca modificar accidentalmente sus archivos de documentación.

<h2 id="create-subagents">
  Crear subagentes
</h2>

<h3 id="programmatic-definition-recommended">
  Definición programática (recomendado)
</h3>

Defina subagentes directamente en su código utilizando el parámetro `agents`. Claude invoca subagentes a través de la herramienta `Agent`.

La mayoría de los ejemplos en esta página imprimen solo el resultado final. Para confirmar que Claude delegó a un subagente en lugar de responder directamente, consulte [Detectar invocación de subagente](#detect-subagent-invocation).

Este ejemplo crea dos subagentes: un revisor de código con acceso de solo lectura y un ejecutor de pruebas que puede ejecutar comandos.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  Configuración de AgentDefinition
</h3>

| Campo             | Tipo                                                        | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                       |
| :---------------- | :---------------------------------------------------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | `string`                                                    | Sí        | Descripción en lenguaje natural de cuándo usar este agente                                                                                                                                                                                                                                                                                                                                        |
| `prompt`          | `string`                                                    | Sí        | El prompt del sistema del agente que define su rol y comportamiento                                                                                                                                                                                                                                                                                                                               |
| `tools`           | `string[]`                                                  | No        | Matriz de nombres de herramientas permitidas. Si se omite, hereda todas las [herramientas disponibles para subagentes](/docs/es/sub-agents#available-tools)                                                                                                                                                                                                                                            |
| `disallowedTools` | `string[]`                                                  | No        | Matriz de nombres de herramientas a eliminar del conjunto de herramientas del agente. También se aceptan patrones a nivel de servidor MCP: `mcp__server` o `mcp__server__*` elimina todas las herramientas de ese servidor, y `mcp__*` elimina todas las herramientas MCP de cualquier servidor                                                                                                   |
| `model`           | `string`                                                    | No        | Anulación de modelo para este agente. Acepta un alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID de modelo completo. `'inherit'` utiliza el modelo principal. Cuando lo omite, Claude Code elige el modelo en el [orden de modelo de subagente](/docs/es/sub-agents#choose-a-model)                                                                                         |
| `skills`          | `string[]`                                                  | No        | Lista de nombres de skills a precargar en el contexto del agente al inicio. Los skills no listados siguen siendo invocables a través de la herramienta Skill                                                                                                                                                                                                                                      |
| `memory`          | `'user' \| 'project' \| 'local'`                            | No        | Fuente de memoria para este agente                                                                                                                                                                                                                                                                                                                                                                |
| `mcpServers`      | `(string \| object)[]`                                      | No        | Servidores MCP disponibles para este agente, por nombre o configuración en línea                                                                                                                                                                                                                                                                                                                  |
| `initialPrompt`   | `string`                                                    | No        | Se envía automáticamente como el primer turno del usuario cuando este agente se ejecuta como el agente del hilo principal. Se ignora cuando el agente se invoca como subagente                                                                                                                                                                                                                    |
| `maxTurns`        | `number`                                                    | No        | Número máximo de turnos de agente antes de que el agente se detenga. Cuando el agente alcanza el límite, Claude Code devuelve su salida marcada como parcial, y puede [reanudar el agente](#resume-subagents) para continuar. La marca parcial requiere Claude Code v2.1.246 o posterior                                                                                                          |
| `background`      | `boolean`                                                   | No        | Ejecutar este agente como una tarea de fondo no bloqueante cuando se invoque                                                                                                                                                                                                                                                                                                                      |
| `omitClaudeMd`    | `boolean`                                                   | No        | Ejecutar este agente sin los archivos CLAUDE.md del usuario, proyecto y local cuando se ejecuta como subagente; los archivos de política administrados aún se cargan. Se ignora cuando el agente se ejecuta como el agente del hilo principal. Requiere TypeScript Agent SDK v0.3.271 o posterior. El SDK de Python [`AgentDefinition`](/docs/es/agent-sdk/python#agentdefinition) no tiene este campo |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | No        | Nivel de esfuerzo de razonamiento para este agente                                                                                                                                                                                                                                                                                                                                                |
| `permissionMode`  | `PermissionMode`                                            | No        | Modo de permiso para la ejecución de herramientas dentro de este agente. Las [reglas de herencia de subagentes](/docs/es/agent-sdk/permissions#available-modes) deciden cuándo se aplica                                                                                                                                                                                                               |

En el SDK de Python, los nombres de campo de varias palabras como `disallowedTools` y `mcpServers` mantienen su ortografía camelCase para coincidir con el formato de cable en lugar de seguir la convención snake\_case de Python. Consulte la [referencia de `AgentDefinition`](/docs/es/agent-sdk/python#agentdefinition) para obtener más detalles.

Los subagentes se ejecutan en segundo plano de forma predeterminada. Una llamada a la herramienta Agent que omite la entrada [`run_in_background`](/docs/es/sub-agents#run-subagents-in-foreground-or-background) inicia un subagente en segundo plano, y Claude establece `run_in_background: false` cuando necesita el resultado antes de continuar. Establezca el campo `background` en `true` para forzar la ejecución en segundo plano para un agente específico independientemente de lo que Claude solicite. Antes de Claude Code v2.1.198, el valor predeterminado de fondo se estaba implementando gradualmente, y una llamada a la herramienta Agent que omitía `run_in_background` podría ejecutar el subagente de forma síncrona.

Los subagentes también pueden generar subagentes propios. Para limitar qué tan profundo es ese anidamiento, cuántos subagentes se ejecutan a la vez y cuánto gasta una consulta, consulte [Limitar profundidad, concurrencia y gasto de subagentes](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Definición basada en sistema de archivos (alternativa)
</h3>

También puede definir subagentes como archivos markdown en directorios `.claude/agents/`. Consulte la [documentación de subagentes de Claude Code](/docs/es/sub-agents) para obtener detalles sobre este enfoque. Los agentes definidos programáticamente tienen prioridad sobre los agentes basados en sistema de archivos con el mismo nombre.

<Note>
  Cuando Claude llama a la herramienta Agent sin un `subagent_type`, obtiene el subagente `general-purpose` integrado, que Claude puede generar incluso cuando no define agentes propios. Establecer [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/es/env-vars) elimina ese valor predeterminado, y tal llamada falla con [`subagent_type is required`](/docs/es/errors#subagent-type-is-required).
</Note>

<h2 id="what-subagents-inherit">
  Lo que heredan los subagentes
</h2>

A menos que el subagente sea un [fork](/docs/es/sub-agents#fork-the-current-conversation), su ventana de contexto comienza de nuevo, sin la conversación padre, pero no está vacía. El único contenido que pasa de padre a subagente es la cadena de prompt de la herramienta Agent, así que incluya cualquier ruta de archivo, mensaje de error o decisión que el subagente necesite directamente en ese prompt.

Un subagente que tiene la herramienta [`SendMessage`](/docs/es/tools-reference) comienza con una lista de los otros agentes nombrados que se ejecutan en la sesión, por lo que sabe qué nombres puede usar para enviar mensajes. Claude Code añade la lista al primer turno del subagente automáticamente. Un [fork](/docs/es/sub-agents#fork-the-current-conversation) no obtiene la lista porque hereda la conversación padre en su lugar.

Un subagente también hereda la configuración de pensamiento extendido de la sesión principal.

La tabla a continuación enumera lo que contiene el contexto de un subagente que no es fork y lo que deja fuera.

| El subagente recibe                                                                                                                                                                                                               | El subagente no recibe                                                               |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| Su propio prompt del sistema (`AgentDefinition.prompt`) y el prompt de la herramienta Agent                                                                                                                                       | El historial de conversación del padre o resultados de herramientas                  |
| Project CLAUDE.md (cargado a través de [`settingSources`](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), a menos que el agente establezca [`omitClaudeMd`](#agentdefinition-configuration) | Contenido de skills precargado, a menos que esté listado en `AgentDefinition.skills` |
| Definiciones de herramientas (heredadas del padre o el subconjunto en `tools`, [filtrado para ejecuciones en segundo plano](/docs/es/sub-agents#available-tools))                                                                      | El prompt del sistema del padre                                                      |

<Note>
  El padre recibe el mensaje final del subagente como resultado de la herramienta Agent, pero puede resumirlo en su propia respuesta. Para preservar la salida del subagente textualmente en la respuesta visible para el usuario, incluya una instrucción para hacerlo en el prompt u opción `systemPrompt` que pase a la llamada principal `query()`.

  En v2.1.210 y posteriores, Claude Code [escanea el mensaje final en busca de patrones con forma de instrucción](/docs/es/sub-agents#subagent-output-scanning) antes de que el padre lo lea. El escaneo trata tres tipos de patrón de manera diferente:

  * **Imitación de etiqueta de control**: Claude Code neutraliza una etiqueta que solo emite el arnés, como un bloque `<system-reminder>`, en su lugar. Inserta una barra invertida después del corchete de apertura y no elimina nada.
  * **Menciones de configuración de permisos**: Claude Code mantiene referencias a la configuración de permisos, como `.claude/settings.json`, `bypassPermissions`, o `--dangerously-skip-permissions`, tal como están escritas.
  * **Marcadores de turno**: una línea que comienza con `Human:` o `Assistant:` obtiene una barra invertida antes de los dos puntos, por lo que el mensaje no puede imitar un límite de turno de conversación.

  Para una coincidencia de etiqueta de control o configuración de permisos, Claude Code antepone una línea de marcador `[harness: ...]` que nombra los patrones coincidentes; una coincidencia de marcador de turno no añade la línea de marcador. Esas son las únicas modificaciones que hace el escaneo: nunca elimina ni reformula el texto del subagente.
</Note>

Un error de API que termina el subagente temprano, como un límite de velocidad, nunca se entrega como su resultado. Consulte [Errores de API en subagentes](/docs/es/sub-agents#api-errors-in-subagents) para el comportamiento en primer plano y segundo plano.

<h2 id="invoke-subagents">
  Invocar subagentes
</h2>

<h3 id="automatic-invocation">
  Invocación automática
</h3>

Claude decide automáticamente cuándo invocar subagentes en función de la tarea y la `description` de cada subagente. Por ejemplo, si define un subagente `performance-optimizer` con la descripción "Especialista en optimización de rendimiento para ajuste de consultas", Claude lo invocará cuando su prompt mencione optimizar consultas.

Escriba descripciones claras y específicas para que Claude pueda hacer coincidir las tareas con el subagente correcto.

<h3 id="explicit-invocation">
  Invocación explícita
</h3>

Para garantizar que Claude use un subagente específico, mencione su nombre en su prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Esto omite la coincidencia automática e invoca directamente el subagente nombrado.

<h3 id="dynamic-agent-configuration">
  Configuración dinámica de agentes
</h3>

Puede crear definiciones de agentes dinámicamente en función de condiciones en tiempo de ejecución. Este ejemplo crea un revisor de seguridad con diferentes niveles de rigor, utilizando un modelo más capaz para revisiones estrictas.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Detectar invocación de subagente
</h2>

Claude invoca subagentes a través de la herramienta Agent. Para detectar cuándo se invoca un subagente, busque bloques `tool_use` donde `name` sea `"Agent"`. Los mensajes desde dentro del contexto de un subagente incluyen un campo `parent_tool_use_id`.

<Note>
  La herramienta aparece como `"Agent"` en bloques `tool_use` pero como `"Task"` en la lista de herramientas `system:init`. Antes de Claude Code v2.1.63, los bloques `tool_use` también la nombraban `"Task"`. Para mantener la detección funcionando en todas las versiones del SDK, haga coincidir ambos valores en `block.name`.
</Note>

La estructura del mensaje difiere entre SDKs. En Python, accede a los bloques de contenido directamente a través de `message.content`. En TypeScript, `SDKAssistantMessage` envuelve el mensaje de la API de Claude, por lo que accede al contenido a través de `message.message.content`.

Este ejemplo itera a través de mensajes transmitidos, registrando cuándo se invoca un subagente y cuándo los mensajes posteriores se originan desde dentro del contexto de ejecución de ese subagente.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Reanudar subagentes
</h2>

Puede reanudar un subagente para continuar donde se detuvo en lugar de comenzar de nuevo. Un subagente reanudado retiene su historial de conversación completo, incluidas todas las llamadas de herramientas anteriores, resultados y razonamiento.

Cuando un subagente se detiene en su límite de [`maxTurns`](#agentdefinition-configuration), Claude Code marca la salida en el resultado de la herramienta Agent como parcial, para que Claude sepa que la ejecución está incompleta.

Cuando un subagente se completa, el resultado de la herramienta Agent incluye un bloque de texto que contiene `agentId: <id>`. Los agentes [`Explore` y `Plan`](/docs/es/sub-agents#built-in-subagents) integrados son de una sola ejecución y no devuelven un `agentId`, así que use un agente personalizado o `general-purpose` cuando necesite reanudar. Para reanudar un subagente mediante programación:

1. **Capturar el ID de sesión**: extraiga `session_id` de los mensajes durante la primera consulta
2. **Extraer el ID del agente**: analice `agentId` del texto del resultado de la herramienta Agent
3. **Reanudar la sesión**: pase `resume: sessionId` en las opciones de la segunda consulta e incluya el ID del agente en su indicación. Cada llamada a `query()` inicia una nueva sesión de forma predeterminada, y debe reanudar la misma sesión para acceder a la transcripción del subagente.

<Note>
  Cuando use un agente personalizado, pase la misma definición de agente en el parámetro `agents` para ambas consultas.
</Note>

El ejemplo a continuación define un agente personalizado `endpoint-finder`. La primera consulta lo ejecuta y captura el ID de sesión y el ID del agente del resultado de la herramienta Agent, luego la segunda consulta reanuda la sesión para hacer una pregunta de seguimiento que requiere contexto del primer análisis.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

Las transcripciones de subagentes se almacenan en archivos separados y persisten independientemente de la conversación principal. Consulte [reanudar subagentes en Claude Code](/docs/es/sub-agents#resume-subagents) para el comportamiento de compactación y el período de limpieza `cleanupPeriodDays`.

<h2 id="tool-restrictions">
  Restricciones de herramientas
</h2>

Use el campo `tools` para limitar lo que puede hacer un subagente:

* **Omitir `tools`**: el subagente obtiene todas las [herramientas disponibles para subagentes](/docs/es/sub-agents#available-tools)
* **Listar herramientas**: el subagente obtiene solo las que especifique. Por ejemplo, un revisor de código que nunca debe editar archivos obtiene `["Read", "Grep", "Glob"]`

Una herramienta que omita no estará en la sesión del subagente en absoluto: Claude funciona sin ella, sin solicitud de permiso ni error.

Este ejemplo crea un agente de análisis de solo lectura que puede examinar código pero no puede modificar archivos ni ejecutar comandos.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Combinaciones comunes de herramientas
</h3>

| Caso de uso              | Herramientas                            | Descripción                                                                  |
| :----------------------- | :-------------------------------------- | :--------------------------------------------------------------------------- |
| Análisis de solo lectura | `Read`, `Grep`, `Glob`                  | Puede examinar código pero no modificar ni ejecutar                          |
| Ejecución de pruebas     | `Bash`, `Read`, `Grep`                  | Puede ejecutar comandos y analizar la salida                                 |
| Modificación de código   | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Acceso completo de lectura/escritura sin ejecución de comandos               |
| Acceso completo          | Todas las herramientas                  | Hereda las herramientas disponibles para subagentes (omita el campo `tools`) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Limitar la profundidad, concurrencia y gasto de subagentes
</h2>

<Note>
  Esta sección describe TypeScript SDK v0.3.219 y Python SDK v0.2.127 y posteriores, los lanzamientos que incluyen Claude Code v2.1.219 o posteriores. En lanzamientos anteriores, algunos de estos límites faltan o tienen valores predeterminados diferentes, así que actualice antes de confiar en ellos para acotar una ejecución. La [referencia de variables de entorno](/docs/es/env-vars) y [turnos y presupuesto](/docs/es/agent-sdk/agent-loop#turns-and-budget) registran la versión de Claude Code que agregó cada variable y la aplicación del límite de gasto del subagente.
</Note>

Claude decide por su cuenta cuándo generar un subagente y cuántos generar. Cada subagente realiza sus propias solicitudes de API, que cuentan hacia el `total_cost_usd` de la consulta, y un subagente puede generar subagentes propios, por lo que un prompt puede crecer en un árbol de agentes.

Puede limitar ese crecimiento de tres formas: qué tan profundamente se anidan los subagentes, cuántos se ejecutan a la vez y cuánto gasta la consulta completa. Establezca los límites de profundidad y concurrencia como variables de entorno a través de la opción [`env`](/docs/es/agent-sdk/typescript#options), y el límite de gasto como una opción de consulta:

| Límite       | Establézcalo con                                         | Predeterminado                                                                                                        | Qué hace Claude Code en el límite                                                                                                                                                                                                                                                                                                                                                          |
| :----------- | :------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Profundidad  | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/es/env-vars)   | `3` capas de subagentes por debajo de su agente principal. `1` impide que sus subagentes generen ninguno de los suyos | Deja un subagente en la capa inferior incapaz de generar, por lo que realiza su trabajo delegado por sí mismo. Consulte [subagentes anidados](/docs/es/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                                                      |
| Concurrencia | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/es/env-vars)   | `20` subagentes ejecutándose a la vez, contando cada subagente que Claude genera con la herramienta Agent             | Se niega a generar otro subagente, devolviendo `Concurrent subagent limit reached`, hasta que el recuento en ejecución caiga por debajo del límite. Las sesiones con [ultracode](/docs/es/model-config#adjust-effort-level) activo nunca se rechazan. Consulte el [límite de subagentes concurrentes](/docs/es/sub-agents#concurrent-subagent-limit)                                                 |
| Gasto        | `maxBudgetUsd` en TypeScript, `max_budget_usd` en Python | Sin límite. Se compara con `total_cost_usd`, por lo que las solicitudes de subagentes cuentan                         | Aplica el límite de tres formas: se niega a generar más subagentes, devolviendo `Budget limit reached`, detiene los subagentes en segundo plano que aún se están ejecutando, y finaliza la consulta con el subtipo de resultado `error_max_budget_usd`. Para saber cómo se comportan los límites en una sesión, consulte [turnos y presupuesto](/docs/es/agent-sdk/agent-loop#turns-and-budget) |

Los dos SDK tratan la opción `env` de manera diferente: el SDK de TypeScript reemplaza el entorno del subproceso con ella, por lo que distribuya `process.env` en ella para mantener variables como `PATH`, mientras que el SDK de Python la fusiona en el entorno heredado. Este ejemplo desactiva el anidamiento, permite como máximo cinco subagentes a la vez, y detiene la consulta una vez que el gasto estimado alcanza \$5:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Lo que ve depende de qué límite, si es que hay alguno, alcanza la consulta:

* **Por debajo del límite de gasto**: ve `success` y el costo estimado.
* **En el límite de gasto**: ve `error_max_budget_usd` con un costo en o por encima de `5`, y luego se ejecuta su controlador de errores.
* **En el límite de concurrencia**: ve un bloque `tool_result` en el flujo de mensajes que lleva `Concurrent subagent limit reached`. Claude recibe el mismo bloque como resultado de la herramienta Agent.

<h3 id="run-opus-5-with-subagents">
  Ejecutar Opus 5 con subagentes
</h3>

Claude Opus 5 delega a subagentes más fácilmente que modelos anteriores, por lo que los [límites de profundidad, concurrencia y gasto](#cap-subagent-depth-concurrency-and-spend) importan más en consultas que ejecutan Opus 5. La [guía de prompting de Opus 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) tiene una instrucción de delegación que puede agregar a cualquier prompt. Si Claude Code agrega una instrucción propia depende de qué [prompt del sistema](/docs/es/agent-sdk/modifying-system-prompts#how-system-prompts-work) use:

* **Preset `claude_code`**: cuando el modelo es Opus 5, Claude Code agrega una línea a su prompt del sistema indicando a Claude que no llame a la herramienta Agent a menos que se le pida. La herramienta Agent permanece disponible.
* **Un prompt personalizado, o sin `systemPrompt`**: Claude Code no construye su prompt del sistema, por lo que esa línea está ausente. Agregue la instrucción de delegación de la guía de prompting a su propio prompt.

Cualquiera de las instrucciones solo orienta a Claude, así que establezca también los límites. Claude Code los aplica sin importar cómo Claude decida delegar.

<h2 id="scale-up-with-dynamic-workflows">
  Escalar con flujos de trabajo dinámicos
</h2>

Los subagentes funcionan bien para algunas tareas delegadas por turno. Para ejecuciones que coordinan docenas a cientos de agentes, use la herramienta `Workflow`, que mueve la orquestación a un script que el tiempo de ejecución ejecuta fuera del contexto de la conversación. Consulte [flujos de trabajo dinámicos](/docs/es/workflows) para ver cómo los flujos de trabajo difieren de la delegación de subagentes turno a turno.

La herramienta `Workflow` está disponible en el SDK de TypeScript Agent v0.3.149 y posterior. Incluya `Workflow` en `allowedTools` para aprobar automáticamente las ejecuciones de flujo de trabajo. Los esquemas de entrada y salida de la herramienta se enumeran en la [referencia de TypeScript](/docs/es/agent-sdk/typescript#workflow).

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude no delega a subagentes
</h3>

Si Claude completa tareas directamente en lugar de delegar a su subagente:

* **Use prompting explícito**: mencione el subagente por nombre en su prompt, por ejemplo "Use el agente code-reviewer para verificar el módulo de autenticación"
* **Escriba una descripción clara**: explique exactamente cuándo se debe usar el subagente para que Claude pueda hacer coincidir las tareas apropiadamente

<h3 id="filesystem-based-agents-not-loading">
  Agentes basados en sistema de archivos no se cargan
</h3>

Claude Code observa `~/.claude/agents/` y `.claude/agents/` y detecta un archivo de agente nuevo o editado en unos pocos segundos, sin necesidad de reiniciar. Si una definición nunca aparece, trabaje a través de estas causas:

* **Nuevo directorio `agents`**: el observador cubre solo directorios que existían cuando se inició la sesión, por lo que el primer archivo en un directorio nuevo necesita un reinicio de sesión. Esta es la causa más común.
* **Frontmatter inválido o un `name` duplicado**: verifique el YAML del archivo y si un agente existente ya usa el `name`.
* **`--disable-slash-commands`**: las sesiones iniciadas con esta bandera no observan estos directorios y siempre necesitan un reinicio para cargar archivos nuevos.
* **Un archivo bajo un directorio agregado**: Claude Code carga `.claude/agents/` desde directorios agregados con la opción `add_dirs` (Python) o `additionalDirectories` (TypeScript), o la CLI `--add-dir` o `/add-dir`, pero no los observa, por lo que un archivo nuevo o editado allí necesita un reinicio de sesión.
* **Un agente programático con el mismo nombre**: los `agents` pasados a `query()` anulan un agente del sistema de archivos con el mismo nombre.

Para el formato de archivo, consulte [cómo escribir archivos de subagente](/docs/es/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Documentación relacionada
</h2>

* [Subagentes de Claude Code](/docs/es/sub-agents): documentación completa de subagentes incluyendo definiciones basadas en sistema de archivos
* [Flujos de trabajo dinámicos](/docs/es/workflows): orqueste muchos subagentes desde un script para trabajos demasiado grandes para una conversación
* [Descripción general del SDK](/docs/es/agent-sdk/overview): introducción al Claude Agent SDK
