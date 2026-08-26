> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Trasmettere risposte in tempo reale

> Ricevere risposte in tempo reale dall'Agent SDK mentre il testo e le chiamate di strumenti vengono trasmessi

Per impostazione predefinita, l'Agent SDK restituisce oggetti `AssistantMessage` completi dopo che Claude ha terminato di generare ogni risposta. Per ricevere aggiornamenti incrementali mentre il testo e le chiamate di strumenti vengono generati, abilita lo streaming di messaggi parziali impostando `include_partial_messages` (Python) o `includePartialMessages` (TypeScript) su `true` nelle tue opzioni.

<Tip>
  Questa pagina copre lo streaming di output (ricezione di token in tempo reale). Per le modalità di input (come invii messaggi), vedi [Inviare messaggi agli agenti](/docs/it/agent-sdk/streaming-vs-single-mode). Puoi anche [trasmettere risposte utilizzando l'Agent SDK tramite la CLI](/docs/it/headless).
</Tip>

<h2 id="enable-streaming-output">
  Abilita lo streaming di output
</h2>

Per abilitare lo streaming, imposta `include_partial_messages` (Python) o `includePartialMessages` (TypeScript) su `true` nelle tue opzioni. Questo fa sì che l'SDK restituisca messaggi `StreamEvent` contenenti eventi API grezzi mentre arrivano, oltre ai soliti `AssistantMessage` e `ResultMessage`.

Il tuo codice deve quindi:

1. Controllare il tipo di ogni messaggio per distinguere `StreamEvent` da altri tipi di messaggio
2. Per `StreamEvent`, estrarre il campo `event` e controllare il suo `type`
3. Cercare eventi `content_block_delta` dove `delta.type` è `text_delta`, che contengono i veri frammenti di testo

L'esempio seguente abilita lo streaming e stampa i frammenti di testo mentre arrivano. Nota i controlli di tipo annidati: prima per `StreamEvent`, poi per `content_block_delta`, poi per `text_delta`:

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
  Riferimento StreamEvent
</h2>

Quando i messaggi parziali sono abilitati, ricevi eventi di streaming API Claude grezzi avvolti in un oggetto. Il tipo ha nomi diversi in ogni SDK:

* **Python**: `StreamEvent` (importa da `claude_agent_sdk.types`)
* **TypeScript**: `SDKPartialAssistantMessage` con `type: 'stream_event'`

Entrambi contengono eventi API Claude grezzi, non testo accumulato. Devi estrarre e accumulare i delta di testo da solo. Ecco la struttura di ogni tipo:

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

Il campo `parent_tool_use_id` è sempre `None` in Python e `null` in TypeScript. Gli eventi di streaming vengono emessi solo per la sessione principale; i delta a livello di token dai subagent non vengono inoltrati. Per attribuire l'output a un subagent, utilizza messaggi completi, che contengono `parent_tool_use_id`. Vedi [Rilevare l'invocazione di subagent](/docs/it/agent-sdk/subagents#detect-subagent-invocation).

Il campo `event` contiene l'evento di streaming grezzo dall'[API Claude](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). I tipi di evento comuni includono:

| Tipo di evento        | Descrizione                                                        |
| :-------------------- | :----------------------------------------------------------------- |
| `message_start`       | Inizio di un nuovo messaggio                                       |
| `content_block_start` | Inizio di un nuovo blocco di contenuto (testo o uso di strumento)  |
| `content_block_delta` | Aggiornamento incrementale al contenuto                            |
| `content_block_stop`  | Fine di un blocco di contenuto                                     |
| `message_delta`       | Aggiornamenti a livello di messaggio (motivo di arresto, utilizzo) |
| `message_stop`        | Fine del messaggio                                                 |

<h2 id="message-flow">
  Flusso di messaggi
</h2>

Con i messaggi parziali abilitati, ricevi messaggi in questo ordine:

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

Senza i messaggi parziali abilitati (`include_partial_messages` in Python, `includePartialMessages` in TypeScript), ricevi tutti i tipi di messaggio tranne `StreamEvent`. I tipi comuni includono `SystemMessage` (inizializzazione della sessione), `AssistantMessage` (risposte complete), `ResultMessage` (risultato finale) e un messaggio di confine compatto che indica quando la cronologia della conversazione è stata compattata (`SDKCompactBoundaryMessage` in TypeScript; `SystemMessage` con sottotipo `"compact_boundary"` in Python).

<h2 id="stream-text-responses">
  Trasmettere risposte di testo
</h2>

Per visualizzare il testo mentre viene generato, cerca eventi `content_block_delta` dove `delta.type` è `text_delta`. Questi contengono i frammenti di testo incrementali. L'esempio seguente stampa ogni frammento mentre arriva:

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
  Trasmettere chiamate di strumenti
</h2>

Le chiamate di strumenti vengono trasmesse anche in modo incrementale. Puoi tracciare quando gli strumenti iniziano, ricevere il loro input mentre viene generato e vedere quando si completano. L'esempio seguente traccia lo strumento attualmente chiamato e accumula l'input JSON mentre viene trasmesso. Utilizza tre tipi di evento:

* `content_block_start`: lo strumento inizia
* `content_block_delta` con `input_json_delta`: i frammenti di input arrivano
* `content_block_stop`: la chiamata dello strumento è completa

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
  Costruire un'interfaccia utente di streaming
</h2>

Questo esempio combina il testo e lo streaming di strumenti in un'interfaccia utente coerente. Traccia se l'agente sta attualmente eseguendo uno strumento (utilizzando un flag `in_tool`) per mostrare indicatori di stato come `[Using Read...]` mentre gli strumenti vengono eseguiti. Il testo viene trasmesso normalmente quando non è in uno strumento e il completamento dello strumento attiva un messaggio "done". Questo modello è utile per le interfacce di chat che devono mostrare lo stato di avanzamento durante attività di agenti multi-step.

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
  Limitazioni note
</h2>

* **Structured output**: il risultato JSON appare solo nel `ResultMessage.structured_output` finale, non come delta di streaming. Vedi [structured outputs](/docs/it/agent-sdk/structured-outputs) per i dettagli.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che puoi trasmettere testo e chiamate di strumenti in tempo reale, esplora questi argomenti correlati:

* [Interactive vs one-shot queries](/docs/it/agent-sdk/streaming-vs-single-mode): scegli tra le modalità di input per il tuo caso d'uso
* [Structured outputs](/docs/it/agent-sdk/structured-outputs): ottieni risposte JSON tipizzate dall'agente
* [Permissions](/docs/it/agent-sdk/permissions): controlla quali strumenti l'agente può utilizzare
