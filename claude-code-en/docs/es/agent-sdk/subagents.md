> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagentes en el SDK

> Define e invoque subagentes para aislar contexto, ejecutar tareas en paralelo y aplicar instrucciones especializadas en sus aplicaciones Claude Agent SDK.

Los subagentes son instancias de agente separadas que su agente principal puede generar para manejar subtareas enfocadas.
Utilice subagentes para aislar contexto, ejecutar múltiples análisis en paralelo y aplicar instrucciones especializadas sin agregar al prompt del agente principal.

Esta guía explica cómo definir y usar subagentes en el SDK utilizando el parámetro `agents`.

<h2 id="overview">
  Descripción general
</h2>

Puede crear subagentes de tres formas:

* **Programáticamente**: use el parámetro `agents` en sus opciones `query()`. Consulte las referencias de [TypeScript](/docs/es/agent-sdk/typescript#agentdefinition) y [Python](/docs/es/agent-sdk/python#agentdefinition)
* **Basado en sistema de archivos**: defina agentes como archivos markdown en directorios `.claude/agents/`. Consulte [definición de subagentes como archivos](/docs/es/sub-agents)
* **Propósito general integrado**: Claude puede invocar el subagente integrado `general-purpose` en cualquier momento a través de la herramienta Agent sin que usted defina nada

Esta guía se enfoca en el enfoque programático, que se recomienda para aplicaciones SDK.

Cuando define subagentes, Claude determina si invocarlos en función del campo `description` de cada subagente. Escriba descripciones claras que expliquen cuándo se debe usar el subagente, y Claude delegará automáticamente las tareas apropiadas. También puede solicitar explícitamente un subagente por nombre en su prompt, por ejemplo "Usa el agente code-reviewer para...".

<h2 id="benefits-of-using-subagents">
  Beneficios de usar subagentes
</h2>

<h3 id="context-isolation">
  Aislamiento de contexto
</h3>

Cada subagente se ejecuta en su propia conversación nueva. Las llamadas a herramientas intermedias y los resultados permanecen dentro del subagente; solo su mensaje final regresa al padre. Consulte [Qué heredan los subagentes](#what-subagents-inherit) para ver exactamente qué hay en el contexto del subagente.

**Ejemplo:** un subagente `research-assistant` puede explorar docenas de archivos sin que ninguno de ese contenido se acumule en la conversación principal. El padre recibe un resumen conciso, no cada archivo que leyó el subagente.

<h3 id="parallelization">
  Paralelización
</h3>

Múltiples subagentes pueden ejecutarse simultáneamente, por lo que las subtareas independientes se completan en el tiempo del más lento en lugar de la suma de todos ellos.

**Ejemplo:** durante una revisión de código, puede ejecutar los subagentes `style-checker`, `security-scanner` y `test-coverage` simultáneamente en lugar de secuencialmente.

<h3 id="specialized-instructions-and-knowledge">
  Instrucciones y conocimiento especializados
</h3>

Cada subagente puede tener prompts de sistema personalizados con experiencia específica, mejores prácticas y restricciones.

**Ejemplo:** un subagente `database-migration` puede tener conocimiento detallado sobre mejores prácticas de SQL, estrategias de reversión y verificaciones de integridad de datos que serían ruido innecesario en las instrucciones del agente principal.

<h3 id="tool-restrictions">
  Restricciones de herramientas
</h3>

Los subagentes pueden limitarse a herramientas específicas, reduciendo el riesgo de acciones no intencionadas.

**Ejemplo:** un subagente `doc-reviewer` podría tener acceso solo a las herramientas Read y Grep, asegurando que pueda analizar pero nunca modifique accidentalmente sus archivos de documentación.

<h2 id="create-subagents">
  Creación de subagentes
</h2>

<h3 id="programmatic-definition-recommended">
  Definición programática (recomendada)
</h3>

Defina subagentes directamente en su código utilizando el parámetro `agents`. Claude invoca subagentes a través de la herramienta `Agent`, por lo que incluya `Agent` en `allowedTools` para aprobar automáticamente las invocaciones de subagentes sin un aviso de permiso.

La mayoría de los ejemplos en esta página imprimen solo el resultado final. Para confirmar que Claude delegó a un subagente en lugar de responder directamente, consulte [Detección de invocación de subagentes](#detect-subagent-invocation).

Este ejemplo crea dos subagentes: un revisor de código con acceso de solo lectura y un ejecutor de pruebas que puede ejecutar comandos.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
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
      // Auto-approve these tools, including Agent for subagent invocation
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

| Campo             | Tipo                                                        | Requerido | Descripción                                                                                                                                                                                                                                                                                     |
| :---------------- | :---------------------------------------------------------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Sí        | Descripción en lenguaje natural de cuándo usar este agente                                                                                                                                                                                                                                      |
| `prompt`          | `string`                                                    | Sí        | El prompt del sistema del agente que define su rol y comportamiento                                                                                                                                                                                                                             |
| `tools`           | `string[]`                                                  | No        | Matriz de nombres de herramientas permitidas. Si se omite, hereda todas las herramientas                                                                                                                                                                                                        |
| `disallowedTools` | `string[]`                                                  | No        | Matriz de nombres de herramientas a eliminar del conjunto de herramientas del agente. También se aceptan patrones a nivel de servidor MCP: `mcp__server` o `mcp__server__*` elimina todas las herramientas de ese servidor, y `mcp__*` elimina todas las herramientas MCP de cualquier servidor |
| `model`           | `string`                                                    | No        | Anulación de modelo para este agente. Acepta un alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID de modelo completo. Por defecto es el modelo principal si se omite                                                                                                  |
| `skills`          | `string[]`                                                  | No        | Lista de nombres de skills a precargar en el contexto del agente al inicio. Los skills no listados permanecen invocables a través de la herramienta Skill                                                                                                                                       |
| `memory`          | `'user' \| 'project' \| 'local'`                            | No        | Fuente de memoria para este agente                                                                                                                                                                                                                                                              |
| `mcpServers`      | `(string \| object)[]`                                      | No        | Servidores MCP disponibles para este agente, por nombre o configuración en línea                                                                                                                                                                                                                |
| `initialPrompt`   | `string`                                                    | No        | Se envía automáticamente como el primer turno del usuario cuando este agente se ejecuta como el agente del hilo principal. Se ignora cuando el agente se invoca como subagente                                                                                                                  |
| `maxTurns`        | `number`                                                    | No        | Número máximo de turnos de agente antes de que el agente se detenga                                                                                                                                                                                                                             |
| `background`      | `boolean`                                                   | No        | Ejecutar este agente como una tarea de fondo no bloqueante cuando se invoca                                                                                                                                                                                                                     |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | No        | Nivel de esfuerzo de razonamiento para este agente                                                                                                                                                                                                                                              |
| `permissionMode`  | `PermissionMode`                                            | No        | Modo de permiso para la ejecución de herramientas dentro de este agente                                                                                                                                                                                                                         |

En el SDK de Python, los nombres de campo de varias palabras como `disallowedTools` y `mcpServers` mantienen su ortografía camelCase para coincidir con el formato de cable en lugar de seguir la convención snake\_case de Python. Consulte la referencia [`AgentDefinition`](/docs/es/agent-sdk/python#agentdefinition) para obtener detalles.

Dos comportamientos de subagentes cambiaron en Claude Code v2.1.198:

* Los subagentes se ejecutan en segundo plano de forma predeterminada. Una llamada a la herramienta Agent que omite la entrada [`run_in_background`](/docs/es/agent-sdk/typescript) inicia un subagente en segundo plano, y Claude establece `run_in_background: false` cuando necesita el resultado antes de continuar. Antes de v2.1.198, omitir `run_in_background` ejecutaba el subagente de forma síncrona. Establezca el campo `background` en `true` para forzar la ejecución en segundo plano para un agente específico independientemente de lo que Claude solicite.
* Un subagente hereda la configuración de pensamiento extendido de la sesión principal. En versiones anteriores, el pensamiento extendido está deshabilitado dentro de los subagentes independientemente de la configuración de la sesión principal.

<Note>
  A partir de Claude Code v2.1.172, los subagentes pueden generar sus propios subagentes. Un subagente cinco niveles por debajo del agente principal no puede generar más subagentes, independientemente de si se ejecuta en primer plano o en segundo plano. Para evitar que un subagente genere otros, omita `Agent` de su matriz `tools` o agréguelo a `disallowedTools`. Consulte [subagentes anidados](/docs/es/sub-agents#spawn-nested-subagents) para conocer las reglas de profundidad completas.
</Note>

<h3 id="filesystem-based-definition-alternative">
  Definición basada en sistema de archivos (alternativa)
</h3>

También puede definir subagentes como archivos markdown en directorios `.claude/agents/`. Consulte la [documentación de subagentes de Claude Code](/docs/es/sub-agents) para obtener detalles sobre este enfoque. Los agentes definidos programáticamente tienen prioridad sobre los agentes basados en sistema de archivos con el mismo nombre.

<Note>
  Incluso sin definir subagentes personalizados, Claude puede generar el subagente integrado `general-purpose`. Esto es útil para delegar tareas de investigación o exploración sin crear agentes especializados. Incluya `Agent` en `allowedTools` para que estas invocaciones se aprueben automáticamente sin un aviso de permiso.
</Note>

<h2 id="what-subagents-inherit">
  Qué heredan los subagentes
</h2>

La ventana de contexto de un subagente comienza nueva, sin conversación padre, pero no está vacía. El único contenido que pasa del padre al subagente es la cadena de prompt de la herramienta Agent, así que incluya cualquier ruta de archivo, mensaje de error o decisión que el subagente necesite directamente en ese prompt.

Un subagente que tiene la herramienta [`SendMessage`](/docs/es/tools-reference) comienza con una lista de los otros agentes nombrados ejecutándose en la sesión, así sabe qué nombres puede usar para enviar mensajes. Claude Code añade la lista al primer turno del subagente automáticamente. Un [fork](/docs/es/sub-agents#fork-the-current-conversation) no obtiene la lista porque hereda la conversación padre en su lugar. La lista requiere Claude Code v2.1.206 o posterior.

| El subagente recibe                                                                                                                                 | El subagente no recibe                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------- |
| Su propio prompt del sistema (`AgentDefinition.prompt`) y el prompt de la herramienta Agent                                                         | El historial de conversación del padre o resultados de herramientas                 |
| CLAUDE.md del proyecto (cargado a través de [`settingSources`](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Contenido de skill precargado, a menos que esté listado en `AgentDefinition.skills` |
| Definiciones de herramientas (heredadas del padre, o el subconjunto en `tools`)                                                                     | El prompt del sistema del padre                                                     |

<Note>
  El padre recibe el mensaje final del subagente textualmente como el resultado de la herramienta Agent, pero puede resumirlo en su propia respuesta. Para preservar la salida del subagente textualmente en la respuesta visible para el usuario, incluya una instrucción para hacerlo en el prompt u opción `systemPrompt` que pase a la llamada `query()` principal.
</Note>

Un error de API que termina el subagente anticipadamente, como un límite de velocidad, nunca se entrega como su resultado. Si un límite de velocidad, sobrecarga o error del servidor corta un subagente en primer plano que ya produjo salida de texto, la herramienta Agent devuelve esa salida parcial con una nota de que el subagente no terminó. Un subagente que no produjo nada, o cuya única salida fueron llamadas de herramientas sin texto, falla con un mensaje de error, `Agent terminated early due to an API error`, seguido del detalle del error. Consulte [API errors in subagents](/docs/es/sub-agents#api-errors-in-subagents) para el comportamiento en primer plano y en segundo plano.

Este manejo de salida parcial requiere Claude Code v2.1.199 o posterior. En v2.1.199, un límite de velocidad, sobrecarga o error del servidor dejó la forma de solo llamadas de herramientas con un resultado parcial vacío que contiene solo la nota de corte.

<h2 id="invoke-subagents">
  Invocación de subagentes
</h2>

<h3 id="automatic-invocation">
  Invocación automática
</h3>

Claude decide automáticamente cuándo invocar subagentes en función de la tarea y la `description` de cada subagente. Por ejemplo, si define un subagente `performance-optimizer` con la descripción "Especialista en optimización de rendimiento para ajuste de consultas", Claude lo invocará cuando su prompt mencione optimizar consultas.

Escriba descripciones claras y específicas para que Claude pueda hacer coincidir tareas con el subagente correcto.

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

Puede crear definiciones de agentes dinámicamente en función de condiciones en tiempo de ejecución. Este ejemplo crea un revisor de seguridad con diferentes niveles de rigor, utilizando un modelo más potente para revisiones estrictas.

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
  Detección de invocación de subagentes
</h2>

Claude invoca subagentes a través de la herramienta Agent. Para detectar cuándo se invoca un subagente, busque bloques `tool_use` donde `name` sea `"Agent"`. Los mensajes desde dentro del contexto de un subagente incluyen un campo `parent_tool_use_id`.

<Note>
  El nombre de la herramienta se cambió de `"Task"` a `"Agent"` en Claude Code v2.1.63. Los lanzamientos actuales del SDK emiten `"Agent"` en bloques `tool_use` pero aún usan `"Task"` en la lista de herramientas `system:init` y en `result.permission_denials[].tool_name`. Verificar ambos valores en `block.name` asegura compatibilidad entre versiones del SDK.
</Note>

La estructura del mensaje difiere entre SDKs. En Python, los bloques de contenido se acceden directamente a través de `message.content`. En TypeScript, `SDKAssistantMessage` envuelve el mensaje de la API de Claude, por lo que el contenido se accede a través de `message.message.content`.

Este ejemplo itera a través de mensajes transmitidos, registrando cuándo se invoca un subagente y cuándo los mensajes posteriores se originan dentro del contexto de ejecución de ese subagente.

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
  Reanudación de subagentes
</h2>

Puede reanudar un subagente para continuar donde se detuvo en lugar de comenzar de nuevo. Un subagente reanudado retiene su historial de conversación completo, incluidas todas las llamadas a herramientas anteriores, resultados y razonamiento.

Cuando un subagente se completa, el resultado de la herramienta Agent incluye un bloque de texto que contiene `agentId: <id>`. Los agentes integrados [`Explore` y `Plan`](/docs/es/sub-agents#built-in-subagents) son de una sola ejecución y no devuelven un `agentId`, así que use un agente personalizado o `general-purpose` cuando necesite reanudar. Para reanudar un subagente programáticamente:

1. **Capture el ID de sesión**: Extraiga `session_id` de los mensajes durante la primera consulta
2. **Extraiga el ID del agente**: Analice `agentId` del texto del resultado de la herramienta Agent
3. **Reanude la sesión**: Pase `resume: sessionId` en las opciones de la segunda consulta e incluya el ID del agente en su prompt

<Note>
  Debe reanudar la misma sesión para acceder a la transcripción del subagente. Cada llamada `query()` inicia una nueva sesión por defecto, así que pase `resume: sessionId` para continuar en la misma sesión.

  Cuando use un agente personalizado, pase la misma definición de agente en el parámetro `agents` para ambas consultas.
</Note>

El ejemplo a continuación define un agente personalizado `endpoint-finder`. La primera consulta lo ejecuta y captura el ID de sesión e ID de agente del resultado de la herramienta Agent, luego la segunda consulta reanuda la sesión para hacer una pregunta de seguimiento que requiere contexto del primer análisis.

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

Las transcripciones de subagentes persisten independientemente de la conversación principal:

* **Compactación de conversación principal**: Cuando la conversación principal se compacta, las transcripciones de subagentes no se ven afectadas. Se almacenan en archivos separados.
* **Persistencia de sesión**: Las transcripciones de subagentes persisten dentro de su sesión. Puede reanudar un subagente después de reiniciar Claude Code reanudando la misma sesión.
* **Limpieza automática**: Las transcripciones se limpian en función de la configuración `cleanupPeriodDays`, que tiene un valor predeterminado de 30 días.

<h2 id="tool-restrictions-2">
  Restricciones de herramientas
</h2>

Los subagentes pueden tener acceso restringido a herramientas a través del campo `tools`:

* **Omitir el campo**: el agente hereda todas las herramientas disponibles (predeterminado)
* **Especificar herramientas**: el agente solo puede usar las herramientas listadas

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

| Caso de uso              | Herramientas                            | Descripción                                                      |
| :----------------------- | :-------------------------------------- | :--------------------------------------------------------------- |
| Análisis de solo lectura | `Read`, `Grep`, `Glob`                  | Puede examinar código pero no modificar ni ejecutar              |
| Ejecución de pruebas     | `Bash`, `Read`, `Grep`                  | Puede ejecutar comandos y analizar salida                        |
| Modificación de código   | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Acceso completo de lectura/escritura sin ejecución de comandos   |
| Acceso completo          | Todas las herramientas                  | Hereda todas las herramientas del padre (omita el campo `tools`) |

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

* **Verifique que las invocaciones de Agent estén aprobadas**: incluya `Agent` en `allowedTools` para aprobar automáticamente las llamadas de subagentes. Sin esto, las invocaciones de Agent caen en su callback `canUseTool` o, en modo `dontAsk`, se deniegan
* **Use prompting explícito**: mencione el subagente por nombre en su prompt, por ejemplo "Use el agente code-reviewer para..."
* **Escriba una descripción clara**: explique exactamente cuándo se debe usar el subagente para que Claude pueda hacer coincidir las tareas apropiadamente

<h3 id="filesystem-based-agents-not-loading">
  Agentes basados en sistema de archivos no se cargan
</h3>

Claude Code observa `~/.claude/agents/` y `.claude/agents/` y detecta un archivo de agente nuevo o editado en unos pocos segundos, sin necesidad de reiniciar. Si una definición nunca aparece, trabaje a través de estas causas:

* **Nuevo directorio `agents`**: el observador cubre solo directorios que existían cuando se inició la sesión, por lo que el primer archivo en un directorio nuevo necesita un reinicio de sesión. Esta es la causa más común.
* **Frontmatter inválido o un `name` duplicado**: verifique el YAML del archivo y si un agente existente ya usa el `name`.
* **`--disable-slash-commands`**: las sesiones iniciadas con esta bandera no observan estos directorios y siempre necesitan un reinicio para cargar archivos nuevos.
* **Un agente programático con el mismo nombre**: los `agents` pasados a `query()` anulan un agente del sistema de archivos con el mismo nombre.

Para el formato de archivo, consulte [cómo escribir archivos de subagente](/docs/es/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Fallos de prompt largo en Windows
</h3>

En Windows, los subagentes con prompts muy largos pueden fallar debido al límite de longitud de línea de comandos de 8191 caracteres. Mantenga los prompts concisos o use agentes basados en sistema de archivos para instrucciones complejas.

<h2 id="related-documentation">
  Documentación relacionada
</h2>

* [Subagentes de Claude Code](/docs/es/sub-agents): documentación completa de subagentes incluyendo definiciones basadas en sistema de archivos
* [Flujos de trabajo dinámicos](/docs/es/workflows): orqueste muchos subagentes desde un script para trabajos demasiado grandes para una conversación
* [Descripción general del SDK](/docs/es/agent-sdk/overview): introducción al Claude Agent SDK
