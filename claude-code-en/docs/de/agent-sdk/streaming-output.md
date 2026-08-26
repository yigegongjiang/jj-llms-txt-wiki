> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Antworten in Echtzeit streamen

> Erhalten Sie Echtzeit-Antworten vom Agent SDK, während Text und Tool-Aufrufe gestreamt werden

Standardmäßig liefert das Agent SDK vollständige `AssistantMessage`-Objekte, nachdem Claude die Generierung jeder Antwort abgeschlossen hat. Um inkrementelle Aktualisierungen zu erhalten, während Text und Tool-Aufrufe generiert werden, aktivieren Sie das Streaming von Teillmeldungen, indem Sie `include_partial_messages` (Python) oder `includePartialMessages` (TypeScript) in Ihren Optionen auf `true` setzen.

<Tip>
  Diese Seite behandelt das Ausgabe-Streaming (Empfangen von Token in Echtzeit). Informationen zu Eingabemodi (wie Sie Nachrichten senden), finden Sie unter [Nachrichten an Agenten senden](/docs/de/agent-sdk/streaming-vs-single-mode). Sie können auch [Antworten mit dem Agent SDK über die CLI streamen](/docs/de/headless).
</Tip>

<h2 id="enable-streaming-output">
  Ausgabe-Streaming aktivieren
</h2>

Um Streaming zu aktivieren, setzen Sie `include_partial_messages` (Python) oder `includePartialMessages` (TypeScript) in Ihren Optionen auf `true`. Dies führt dazu, dass das SDK `StreamEvent`-Nachrichten mit rohen API-Ereignissen liefert, die ankommen, zusätzlich zu den üblichen `AssistantMessage`- und `ResultMessage`-Objekten.

Ihr Code muss dann:

1. Den Typ jeder Nachricht überprüfen, um `StreamEvent` von anderen Nachrichtentypen zu unterscheiden
2. Für `StreamEvent` das Feld `event` extrahieren und seinen `type` überprüfen
3. Nach `content_block_delta`-Ereignissen suchen, bei denen `delta.type` gleich `text_delta` ist, die die tatsächlichen Text-Chunks enthalten

Das folgende Beispiel aktiviert Streaming und gibt Text-Chunks aus, während sie ankommen. Beachten Sie die verschachtelten Typüberprüfungen: zuerst für `StreamEvent`, dann für `content_block_delta`, dann für `text_delta`:

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
  StreamEvent-Referenz
</h2>

Wenn Teillmeldungen aktiviert sind, erhalten Sie rohe Claude-API-Streaming-Ereignisse, die in einem Objekt verpackt sind. Der Typ hat in jedem SDK unterschiedliche Namen:

* **Python**: `StreamEvent` (importieren aus `claude_agent_sdk.types`)
* **TypeScript**: `SDKPartialAssistantMessage` mit `type: 'stream_event'`

Beide enthalten rohe Claude-API-Ereignisse, nicht angesammelte Text. Sie müssen Text-Deltas selbst extrahieren und ansammeln. Hier ist die Struktur jedes Typs:

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

Das Feld `parent_tool_use_id` ist in Python immer `None` und in TypeScript immer `null`. Streaming-Ereignisse werden nur für die Hauptsitzung ausgegeben; Token-Level-Deltas von Subagenten werden nicht weitergeleitet. Um die Ausgabe einem Subagenten zuzuordnen, verwenden Sie vollständige Nachrichten, die `parent_tool_use_id` enthalten. Siehe [Subagenten-Aufruf erkennen](/docs/de/agent-sdk/subagents#detect-subagent-invocation).

Das Feld `event` enthält das rohe Streaming-Ereignis aus der [Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Häufige Ereignistypen sind:

| Ereignistyp           | Beschreibung                                                 |
| :-------------------- | :----------------------------------------------------------- |
| `message_start`       | Beginn einer neuen Nachricht                                 |
| `content_block_start` | Beginn eines neuen Inhaltsblocks (Text oder Tool-Verwendung) |
| `content_block_delta` | Inkrementelle Aktualisierung des Inhalts                     |
| `content_block_stop`  | Ende eines Inhaltsblocks                                     |
| `message_delta`       | Aktualisierungen auf Nachrichtenebene (Stoppgrund, Nutzung)  |
| `message_stop`        | Ende der Nachricht                                           |

<h2 id="message-flow">
  Nachrichtenfluss
</h2>

Mit aktivierten Teillmeldungen erhalten Sie Nachrichten in dieser Reihenfolge:

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

Ohne aktivierte Teillmeldungen (`include_partial_messages` in Python, `includePartialMessages` in TypeScript) erhalten Sie alle Nachrichtentypen außer `StreamEvent`. Häufige Typen sind `SystemMessage` (Sitzungsinitialisierung), `AssistantMessage` (vollständige Antworten), `ResultMessage` (Endergebnis) und eine kompakte Grenzmarkierung, die anzeigt, wann der Gesprächsverlauf komprimiert wurde (`SDKCompactBoundaryMessage` in TypeScript; `SystemMessage` mit Subtyp `"compact_boundary"` in Python).

<h2 id="stream-text-responses">
  Text-Antworten streamen
</h2>

Um Text anzuzeigen, während er generiert wird, suchen Sie nach `content_block_delta`-Ereignissen, bei denen `delta.type` gleich `text_delta` ist. Diese enthalten die inkrementellen Text-Chunks. Das folgende Beispiel gibt jeden Chunk aus, während er ankommt:

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
  Tool-Aufrufe streamen
</h2>

Tool-Aufrufe werden auch inkrementell gestreamt. Sie können verfolgen, wann Tools starten, ihre Eingabe erhalten, während sie generiert wird, und sehen, wann sie abgeschlossen sind. Das folgende Beispiel verfolgt das aktuell aufgerufene Tool und sammelt die JSON-Eingabe, während sie gestreamt wird. Es verwendet drei Ereignistypen:

* `content_block_start`: Tool beginnt
* `content_block_delta` mit `input_json_delta`: Eingabe-Chunks kommen an
* `content_block_stop`: Tool-Aufruf abgeschlossen

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
  Streaming-UI erstellen
</h2>

Dieses Beispiel kombiniert Text- und Tool-Streaming in eine kohärente Benutzeroberfläche. Es verfolgt, ob der Agent gerade ein Tool ausführt (mit einem `in_tool`-Flag), um Statusanzeigen wie `[Using Read...]` anzuzeigen, während Tools ausgeführt werden. Text wird normal gestreamt, wenn nicht in einem Tool, und die Tool-Fertigstellung löst eine „done"-Nachricht aus. Dieses Muster ist nützlich für Chat-Schnittstellen, die während mehrstufiger Agent-Aufgaben Fortschritt anzeigen müssen.

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
  Bekannte Einschränkungen
</h2>

* **Strukturierte Ausgabe**: Das JSON-Ergebnis erscheint nur in der finalen `ResultMessage.structured_output`, nicht als Streaming-Deltas. Weitere Informationen finden Sie unter [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs).

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Text und Tool-Aufrufe in Echtzeit streamen können, erkunden Sie diese verwandten Themen:

* [Interaktive vs. One-Shot-Abfragen](/docs/de/agent-sdk/streaming-vs-single-mode): Wählen Sie zwischen Eingabemodi für Ihren Anwendungsfall
* [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs): Erhalten Sie typisierte JSON-Antworten vom Agent
* [Berechtigungen](/docs/de/agent-sdk/permissions): Kontrollieren Sie, welche Tools der Agent verwenden kann
