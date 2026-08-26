> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Transmitir respostas em tempo real

> Obtenha respostas em tempo real do Agent SDK conforme o texto e as chamadas de ferramentas são transmitidas

Por padrão, o Agent SDK produz objetos `AssistantMessage` completos após Claude terminar de gerar cada resposta. Para receber atualizações incrementais conforme o texto e as chamadas de ferramentas são geradas, ative o streaming de mensagens parciais definindo `include_partial_messages` (Python) ou `includePartialMessages` (TypeScript) como `true` nas suas opções.

<Tip>
  Esta página aborda o streaming de saída (recebimento de tokens em tempo real). Para modos de entrada (como você envia mensagens), consulte [Enviar mensagens para agentes](/docs/pt/agent-sdk/streaming-vs-single-mode). Você também pode [transmitir respostas usando o Agent SDK via CLI](/docs/pt/headless).
</Tip>

<h2 id="enable-streaming-output">
  Ativar streaming de saída
</h2>

Para ativar o streaming, defina `include_partial_messages` (Python) ou `includePartialMessages` (TypeScript) como `true` nas suas opções. Isso faz com que o SDK produza mensagens `StreamEvent` contendo eventos brutos da API conforme chegam, além das mensagens `AssistantMessage` e `ResultMessage` usuais.

Seu código então precisa:

1. Verificar o tipo de cada mensagem para distinguir `StreamEvent` de outros tipos de mensagem
2. Para `StreamEvent`, extrair o campo `event` e verificar seu `type`
3. Procurar por eventos `content_block_delta` onde `delta.type` é `text_delta`, que contêm os chunks de texto reais

O exemplo abaixo ativa o streaming e imprime chunks de texto conforme chegam. Observe as verificações de tipo aninhadas: primeiro para `StreamEvent`, depois para `content_block_delta`, depois para `text_delta`:

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
  Referência de StreamEvent
</h2>

Quando mensagens parciais estão ativadas, você recebe eventos brutos de streaming da Claude API encapsulados em um objeto. O tipo tem nomes diferentes em cada SDK:

* **Python**: `StreamEvent` (importar de `claude_agent_sdk.types`)
* **TypeScript**: `SDKPartialAssistantMessage` com `type: 'stream_event'`

Ambos contêm eventos brutos da Claude API, não texto acumulado. Você precisa extrair e acumular deltas de texto você mesmo. Aqui está a estrutura de cada tipo:

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

O campo `parent_tool_use_id` é sempre `None` em Python e `null` em TypeScript. Eventos de stream são emitidos apenas para a sessão principal; deltas no nível de token de subagentos não são encaminhados. Para atribuir saída a um subagentos, use mensagens completas, que carregam `parent_tool_use_id`. Veja [Detectar invocação de subagentos](/docs/pt/agent-sdk/subagents#detect-subagent-invocation).

O campo `event` contém o evento de streaming bruto da [Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Os tipos de evento comuns incluem:

| Tipo de Evento        | Descrição                                                        |
| :-------------------- | :--------------------------------------------------------------- |
| `message_start`       | Início de uma nova mensagem                                      |
| `content_block_start` | Início de um novo bloco de conteúdo (texto ou uso de ferramenta) |
| `content_block_delta` | Atualização incremental do conteúdo                              |
| `content_block_stop`  | Fim de um bloco de conteúdo                                      |
| `message_delta`       | Atualizações no nível da mensagem (motivo de parada, uso)        |
| `message_stop`        | Fim da mensagem                                                  |

<h2 id="message-flow">
  Fluxo de mensagens
</h2>

Com mensagens parciais ativadas, você recebe mensagens nesta ordem:

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

Sem mensagens parciais ativadas (`include_partial_messages` em Python, `includePartialMessages` em TypeScript), você recebe todos os tipos de mensagem exceto `StreamEvent`. Os tipos comuns incluem `SystemMessage` (inicialização de sessão), `AssistantMessage` (respostas completas), `ResultMessage` (resultado final) e uma mensagem de limite compacta indicando quando o histórico de conversa foi compactado (`SDKCompactBoundaryMessage` em TypeScript; `SystemMessage` com subtipo `"compact_boundary"` em Python).

<h2 id="stream-text-responses">
  Transmitir respostas de texto
</h2>

Para exibir o texto conforme é gerado, procure por eventos `content_block_delta` onde `delta.type` é `text_delta`. Estes contêm os chunks de texto incrementais. O exemplo abaixo imprime cada chunk conforme chega:

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
  Transmitir chamadas de ferramentas
</h2>

As chamadas de ferramentas também são transmitidas incrementalmente. Você pode rastrear quando as ferramentas começam, receber sua entrada conforme é gerada e ver quando são concluídas. O exemplo abaixo rastreia a ferramenta atual sendo chamada e acumula a entrada JSON conforme é transmitida. Ele usa três tipos de evento:

* `content_block_start`: ferramenta começa
* `content_block_delta` com `input_json_delta`: chunks de entrada chegam
* `content_block_stop`: chamada de ferramenta concluída

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
  Construir uma interface de usuário de streaming
</h2>

Este exemplo combina streaming de texto e ferramentas em uma interface coerente. Ele rastreia se o agente está executando uma ferramenta (usando um sinalizador `in_tool`) para mostrar indicadores de status como `[Using Read...]` enquanto as ferramentas são executadas. O texto é transmitido normalmente quando não está em uma ferramenta, e a conclusão da ferramenta dispara uma mensagem "done". Este padrão é útil para interfaces de chat que precisam mostrar progresso durante tarefas de agente em várias etapas.

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
  Limitações conhecidas
</h2>

* **Structured output**: o resultado JSON aparece apenas no `ResultMessage.structured_output` final, não como deltas de streaming. Consulte [structured outputs](/docs/pt/agent-sdk/structured-outputs) para detalhes.

<h2 id="next-steps">
  Próximas etapas
</h2>

Agora que você pode transmitir texto e chamadas de ferramentas em tempo real, explore estes tópicos relacionados:

* [Interactive vs one-shot queries](/docs/pt/agent-sdk/streaming-vs-single-mode): escolha entre modos de entrada para seu caso de uso
* [Structured outputs](/docs/pt/agent-sdk/structured-outputs): obtenha respostas JSON digitadas do agente
* [Permissions](/docs/pt/agent-sdk/permissions): controle quais ferramentas o agente pode usar
