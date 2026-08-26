> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Transmitir respuestas en tiempo real

> Obtener respuestas en tiempo real del Agent SDK mientras el texto y las llamadas de herramientas se transmiten

De forma predeterminada, el Agent SDK produce objetos `AssistantMessage` completos después de que Claude termina de generar cada respuesta. Para recibir actualizaciones incrementales mientras se generan texto y llamadas de herramientas, habilite la transmisión de mensajes parciales estableciendo `include_partial_messages` (Python) o `includePartialMessages` (TypeScript) en `true` en sus opciones.

<Tip>
  Esta página cubre la transmisión de salida (recibir tokens en tiempo real). Para modos de entrada (cómo envía mensajes), consulte [Enviar mensajes a agentes](/docs/es/agent-sdk/streaming-vs-single-mode). También puede [transmitir respuestas usando el Agent SDK a través de la CLI](/docs/es/headless).
</Tip>

<h2 id="enable-streaming-output">
  Habilitar la transmisión de salida
</h2>

Para habilitar la transmisión, establezca `include_partial_messages` (Python) o `includePartialMessages` (TypeScript) en `true` en sus opciones. Esto hace que el SDK produzca mensajes `StreamEvent` que contienen eventos de API sin procesar a medida que llegan, además de los `AssistantMessage` y `ResultMessage` habituales.

Su código entonces necesita:

1. Verificar el tipo de cada mensaje para distinguir `StreamEvent` de otros tipos de mensaje
2. Para `StreamEvent`, extraer el campo `event` y verificar su `type`
3. Buscar eventos `content_block_delta` donde `delta.type` sea `text_delta`, que contienen los fragmentos de texto reales

El ejemplo a continuación habilita la transmisión e imprime fragmentos de texto a medida que llegan. Observe las verificaciones de tipo anidadas: primero para `StreamEvent`, luego para `content_block_delta`, luego para `text_delta`:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_response():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Bash", "Read"],
      )

      async for message in query(prompt="List the files in my project", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              if event.get("type") == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "text_delta":
                      print(delta.get("text", ""), end="", flush=True)


  asyncio.run(stream_response())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the files in my project",
    options: {
      includePartialMessages: true,
      allowedTools: ["Bash", "Read"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;
      if (event.type === "content_block_delta") {
        if (event.delta.type === "text_delta") {
          process.stdout.write(event.delta.text);
        }
      }
    }
  }
  ```
</CodeGroup>

<h2 id="streamevent-reference">
  Referencia de StreamEvent
</h2>

Cuando los mensajes parciales están habilitados, recibe eventos de transmisión de API de Claude sin procesar envueltos en un objeto. El tipo tiene nombres diferentes en cada SDK:

* **Python**: `StreamEvent` (importar desde `claude_agent_sdk.types`)
* **TypeScript**: `SDKPartialAssistantMessage` con `type: 'stream_event'`

Ambos contienen eventos de API de Claude sin procesar, no texto acumulado. Necesita extraer y acumular deltas de texto usted mismo. Aquí está la estructura de cada tipo:

<CodeGroup>
  ```python Python theme={null}
  @dataclass
  class StreamEvent:
      uuid: str  # Unique identifier for this event
      session_id: str  # Session identifier
      event: dict[str, Any]  # The raw Claude API stream event
      parent_tool_use_id: str | None  # Always None
  ```

  ```typescript TypeScript theme={null}
  type SDKPartialAssistantMessage = {
    type: "stream_event";
    event: BetaRawMessageStreamEvent; // From Anthropic SDK
    parent_tool_use_id: string | null;
    uuid: UUID;
    session_id: string;
    ttft_ms?: number; // Time to first token in ms, present only on message_start events
  };
  ```
</CodeGroup>

El campo `parent_tool_use_id` siempre es `None` en Python y `null` en TypeScript. Los eventos de transmisión se emiten solo para la sesión principal; los deltas a nivel de token de subagentos no se reenvían. Para atribuir la salida a un subagentos, use mensajes completos, que llevan `parent_tool_use_id`. Consulte [Detectar invocación de subagentos](/docs/es/agent-sdk/subagents#detect-subagent-invocation).

El campo `event` contiene el evento de transmisión sin procesar de la [API de Claude](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Los tipos de eventos comunes incluyen:

| Tipo de evento        | Descripción                                                         |
| :-------------------- | :------------------------------------------------------------------ |
| `message_start`       | Inicio de un nuevo mensaje                                          |
| `content_block_start` | Inicio de un nuevo bloque de contenido (texto o uso de herramienta) |
| `content_block_delta` | Actualización incremental del contenido                             |
| `content_block_stop`  | Fin de un bloque de contenido                                       |
| `message_delta`       | Actualizaciones a nivel de mensaje (razón de parada, uso)           |
| `message_stop`        | Fin del mensaje                                                     |

<h2 id="message-flow">
  Flujo de mensajes
</h2>

Con mensajes parciales habilitados, recibe mensajes en este orden:

```text theme={null}
StreamEvent (message_start)
StreamEvent (content_block_start) - text block
StreamEvent (content_block_delta) - text chunks...
StreamEvent (content_block_stop)
StreamEvent (content_block_start) - tool_use block
StreamEvent (content_block_delta) - tool input chunks...
StreamEvent (content_block_stop)
StreamEvent (message_delta)
StreamEvent (message_stop)
AssistantMessage - complete message with all content
... tool executes ...
... more streaming events for next turn ...
ResultMessage - final result
```

Sin mensajes parciales habilitados (`include_partial_messages` en Python, `includePartialMessages` en TypeScript), recibe todos los tipos de mensaje excepto `StreamEvent`. Los tipos comunes incluyen `SystemMessage` (inicialización de sesión), `AssistantMessage` (respuestas completas), `ResultMessage` (resultado final) y un mensaje de límite compacto que indica cuándo se compactó el historial de conversación (`SDKCompactBoundaryMessage` en TypeScript; `SystemMessage` con subtipo `"compact_boundary"` en Python).

<h2 id="stream-text-responses">
  Transmitir respuestas de texto
</h2>

Para mostrar texto a medida que se genera, busque eventos `content_block_delta` donde `delta.type` sea `text_delta`. Estos contienen los fragmentos de texto incrementales. El ejemplo a continuación imprime cada fragmento a medida que llega:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_text():
      options = ClaudeAgentOptions(include_partial_messages=True)

      async for message in query(prompt="Explain how databases work", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              if event.get("type") == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "text_delta":
                      # Print each text chunk as it arrives
                      print(delta.get("text", ""), end="", flush=True)

      print()  # Final newline


  asyncio.run(stream_text())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Explain how databases work",
    options: { includePartialMessages: true }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;
      if (event.type === "content_block_delta" && event.delta.type === "text_delta") {
        process.stdout.write(event.delta.text);
      }
    }
  }

  console.log(); // Final newline
  ```
</CodeGroup>

<h2 id="stream-tool-calls">
  Transmitir llamadas de herramientas
</h2>

Las llamadas de herramientas también se transmiten incrementalmente. Puede rastrear cuándo comienzan las herramientas, recibir su entrada a medida que se genera y ver cuándo se completan. El ejemplo a continuación rastrea la herramienta actual que se está llamando y acumula la entrada JSON a medida que se transmite. Utiliza tres tipos de eventos:

* `content_block_start`: la herramienta comienza
* `content_block_delta` con `input_json_delta`: llegan fragmentos de entrada
* `content_block_stop`: llamada de herramienta completada

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_tool_calls():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Read", "Bash"],
      )

      # Track the current tool and accumulate its input JSON
      current_tool = None
      tool_input = ""

      async for message in query(prompt="Read the README.md file", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              event_type = event.get("type")

              if event_type == "content_block_start":
                  # New tool call is starting
                  content_block = event.get("content_block", {})
                  if content_block.get("type") == "tool_use":
                      current_tool = content_block.get("name")
                      tool_input = ""
                      print(f"Starting tool: {current_tool}")

              elif event_type == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "input_json_delta":
                      # Accumulate JSON input as it streams in
                      chunk = delta.get("partial_json", "")
                      tool_input += chunk
                      print(f"  Input chunk: {chunk}")

              elif event_type == "content_block_stop":
                  # Tool call complete - show final input
                  if current_tool:
                      print(f"Tool {current_tool} called with: {tool_input}")
                      current_tool = None


  asyncio.run(stream_tool_calls())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track the current tool and accumulate its input JSON
  let currentTool: string | null = null;
  let toolInput = "";

  for await (const message of query({
    prompt: "Read the README.md file",
    options: {
      includePartialMessages: true,
      allowedTools: ["Read", "Bash"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;

      if (event.type === "content_block_start") {
        // New tool call is starting
        if (event.content_block.type === "tool_use") {
          currentTool = event.content_block.name;
          toolInput = "";
          console.log(`Starting tool: ${currentTool}`);
        }
      } else if (event.type === "content_block_delta") {
        if (event.delta.type === "input_json_delta") {
          // Accumulate JSON input as it streams in
          const chunk = event.delta.partial_json;
          toolInput += chunk;
          console.log(`  Input chunk: ${chunk}`);
        }
      } else if (event.type === "content_block_stop") {
        // Tool call complete - show final input
        if (currentTool) {
          console.log(`Tool ${currentTool} called with: ${toolInput}`);
          currentTool = null;
        }
      }
    }
  }
  ```
</CodeGroup>

<h2 id="build-a-streaming-ui">
  Construir una interfaz de usuario de transmisión
</h2>

Este ejemplo combina la transmisión de texto y herramientas en una interfaz de usuario coherente. Rastrea si el agente está ejecutando actualmente una herramienta (usando una bandera `in_tool`) para mostrar indicadores de estado como `[Using Read...]` mientras se ejecutan las herramientas. El texto se transmite normalmente cuando no está en una herramienta, y la finalización de la herramienta desencadena un mensaje "done". Este patrón es útil para interfaces de chat que necesitan mostrar progreso durante tareas de agente de varios pasos.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
  from claude_agent_sdk.types import StreamEvent
  import asyncio
  import sys


  async def streaming_ui():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Read", "Bash", "Grep"],
      )

      # Track whether we're currently in a tool call
      in_tool = False

      async for message in query(
          prompt="Find all TODO comments in the codebase", options=options
      ):
          if isinstance(message, StreamEvent):
              event = message.event
              event_type = event.get("type")

              if event_type == "content_block_start":
                  content_block = event.get("content_block", {})
                  if content_block.get("type") == "tool_use":
                      # Tool call is starting - show status indicator
                      tool_name = content_block.get("name")
                      print(f"\n[Using {tool_name}...]", end="", flush=True)
                      in_tool = True

              elif event_type == "content_block_delta":
                  delta = event.get("delta", {})
                  # Only stream text when not executing a tool
                  if delta.get("type") == "text_delta" and not in_tool:
                      sys.stdout.write(delta.get("text", ""))
                      sys.stdout.flush()

              elif event_type == "content_block_stop":
                  if in_tool:
                      # Tool call finished
                      print(" done", flush=True)
                      in_tool = False

          elif isinstance(message, ResultMessage):
              # Agent finished all work
              print(f"\n\n--- Complete ---")


  asyncio.run(streaming_ui())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track whether we're currently in a tool call
  let inTool = false;

  for await (const message of query({
    prompt: "Find all TODO comments in the codebase",
    options: {
      includePartialMessages: true,
      allowedTools: ["Read", "Bash", "Grep"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;

      if (event.type === "content_block_start") {
        if (event.content_block.type === "tool_use") {
          // Tool call is starting - show status indicator
          process.stdout.write(`\n[Using ${event.content_block.name}...]`);
          inTool = true;
        }
      } else if (event.type === "content_block_delta") {
        // Only stream text when not executing a tool
        if (event.delta.type === "text_delta" && !inTool) {
          process.stdout.write(event.delta.text);
        }
      } else if (event.type === "content_block_stop") {
        if (inTool) {
          // Tool call finished
          console.log(" done");
          inTool = false;
        }
      }
    } else if (message.type === "result") {
      // Agent finished all work
      console.log("\n\n--- Complete ---");
    }
  }
  ```
</CodeGroup>

<h2 id="known-limitations">
  Limitaciones conocidas
</h2>

* **Structured output**: el resultado JSON aparece solo en el `ResultMessage.structured_output` final, no como deltas de transmisión. Consulte [structured outputs](/docs/es/agent-sdk/structured-outputs) para obtener detalles.

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que puede transmitir texto y llamadas de herramientas en tiempo real, explore estos temas relacionados:

* [Interactive vs one-shot queries](/docs/es/agent-sdk/streaming-vs-single-mode): elija entre modos de entrada para su caso de uso
* [Structured outputs](/docs/es/agent-sdk/structured-outputs): obtenga respuestas JSON tipificadas del agente
* [Permissions](/docs/es/agent-sdk/permissions): controle qué herramientas puede usar el agente
