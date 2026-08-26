> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Stream responses in real-time

> Dapatkan respons real-time dari Agent SDK saat teks dan tool calls streaming masuk

Secara default, Agent SDK menghasilkan objek `AssistantMessage` lengkap setelah Claude selesai menghasilkan setiap respons. Untuk menerima pembaruan inkremental saat teks dan tool calls dihasilkan, aktifkan partial message streaming dengan mengatur `include_partial_messages` (Python) atau `includePartialMessages` (TypeScript) ke `true` dalam opsi Anda.

<Tip>
  Halaman ini mencakup output streaming (menerima token secara real-time). Untuk input modes (cara Anda mengirim pesan), lihat [Kirim pesan ke agents](/docs/id/agent-sdk/streaming-vs-single-mode). Anda juga dapat [stream responses using the Agent SDK via the CLI](/docs/id/headless).
</Tip>

<h2 id="enable-streaming-output">
  Enable streaming output
</h2>

Untuk mengaktifkan streaming, atur `include_partial_messages` (Python) atau `includePartialMessages` (TypeScript) ke `true` dalam opsi Anda. Ini menyebabkan SDK menghasilkan pesan `StreamEvent` yang berisi raw API events saat tiba, selain `AssistantMessage` dan `ResultMessage` yang biasa.

Kode Anda kemudian perlu:

1. Memeriksa tipe setiap pesan untuk membedakan `StreamEvent` dari tipe pesan lainnya
2. Untuk `StreamEvent`, ekstrak field `event` dan periksa `type`-nya
3. Cari event `content_block_delta` di mana `delta.type` adalah `text_delta`, yang berisi chunk teks aktual

Contoh di bawah mengaktifkan streaming dan mencetak chunk teks saat tiba. Perhatikan pemeriksaan tipe bersarang: pertama untuk `StreamEvent`, kemudian untuk `content_block_delta`, kemudian untuk `text_delta`:

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
  StreamEvent reference
</h2>

Ketika partial messages diaktifkan, Anda menerima raw Claude API streaming events yang dibungkus dalam objek. Tipe memiliki nama berbeda di setiap SDK:

* **Python**: `StreamEvent` (import dari `claude_agent_sdk.types`)
* **TypeScript**: `SDKPartialAssistantMessage` dengan `type: 'stream_event'`

Keduanya berisi raw Claude API events, bukan teks terakumulasi. Anda perlu mengekstrak dan mengakumulasi text deltas sendiri. Berikut adalah struktur setiap tipe:

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

Field `parent_tool_use_id` selalu `None` di Python dan `null` di TypeScript. Stream events dipancarkan untuk sesi utama saja; token-level deltas dari subagents tidak diteruskan. Untuk mengatribusikan output ke subagent, gunakan complete messages, yang membawa `parent_tool_use_id`. Lihat [Detect subagent invocation](/docs/id/agent-sdk/subagents#detect-subagent-invocation).

Field `event` berisi raw streaming event dari [Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Tipe event umum meliputi:

| Event Type            | Description                                  |
| :-------------------- | :------------------------------------------- |
| `message_start`       | Awal pesan baru                              |
| `content_block_start` | Awal blok konten baru (teks atau tool use)   |
| `content_block_delta` | Pembaruan inkremental ke konten              |
| `content_block_stop`  | Akhir blok konten                            |
| `message_delta`       | Pembaruan tingkat pesan (stop reason, usage) |
| `message_stop`        | Akhir pesan                                  |

<h2 id="message-flow">
  Alur pesan
</h2>

Dengan partial messages diaktifkan, Anda menerima pesan dalam urutan ini:

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

Tanpa partial messages diaktifkan (`include_partial_messages` di Python, `includePartialMessages` di TypeScript), Anda menerima semua tipe pesan kecuali `StreamEvent`. Tipe umum meliputi `SystemMessage` (inisialisasi sesi), `AssistantMessage` (respons lengkap), `ResultMessage` (hasil akhir), dan pesan batas kompak yang menunjukkan kapan riwayat percakapan dikompres (`SDKCompactBoundaryMessage` di TypeScript; `SystemMessage` dengan subtype `"compact_boundary"` di Python).

<h2 id="stream-text-responses">
  Stream text responses
</h2>

Untuk menampilkan teks saat dihasilkan, cari event `content_block_delta` di mana `delta.type` adalah `text_delta`. Ini berisi chunk teks inkremental. Contoh di bawah mencetak setiap chunk saat tiba:

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
  Stream tool calls
</h2>

Tool calls juga streaming secara inkremental. Anda dapat melacak kapan tools dimulai, menerima input mereka saat dihasilkan, dan melihat kapan mereka selesai. Contoh di bawah melacak tool yang sedang dipanggil dan mengakumulasi input JSON saat streaming masuk. Ini menggunakan tiga tipe event:

* `content_block_start`: tool dimulai
* `content_block_delta` dengan `input_json_delta`: chunk input tiba
* `content_block_stop`: tool call selesai

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
  Build a streaming UI
</h2>

Contoh ini menggabungkan text dan tool streaming menjadi UI yang kohesif. Ini melacak apakah agent saat ini mengeksekusi tool (menggunakan flag `in_tool`) untuk menampilkan indikator status seperti `[Using Read...]` saat tools berjalan. Teks streaming secara normal ketika tidak dalam tool, dan penyelesaian tool memicu pesan "done". Pola ini berguna untuk antarmuka chat yang perlu menampilkan progress selama tugas agent multi-langkah.

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
  Keterbatasan yang Diketahui
</h2>

* **Structured output**: hasil JSON muncul hanya di `ResultMessage.structured_output` akhir, bukan sebagai streaming deltas. Lihat [structured outputs](/docs/id/agent-sdk/structured-outputs) untuk detail.

<h2 id="next-steps">
  Langkah selanjutnya
</h2>

Sekarang bahwa Anda dapat stream teks dan tool calls secara real-time, jelajahi topik terkait ini:

* [Interactive vs one-shot queries](/docs/id/agent-sdk/streaming-vs-single-mode): pilih antara input modes untuk use case Anda
* [Structured outputs](/docs/id/agent-sdk/structured-outputs): dapatkan respons JSON yang diketik dari agent
* [Permissions](/docs/id/agent-sdk/permissions): kontrol tools mana yang dapat digunakan agent
