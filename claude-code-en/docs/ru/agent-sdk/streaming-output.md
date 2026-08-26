> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Потоковая передача ответов в реальном времени

> Получайте ответы в реальном времени от Agent SDK по мере поступления текста и вызовов инструментов

По умолчанию Agent SDK выдает полные объекты `AssistantMessage` после того, как Claude завершит генерацию каждого ответа. Чтобы получать добавочные обновления по мере генерации текста и вызовов инструментов, включите потоковую передачу частичных сообщений, установив `include_partial_messages` (Python) или `includePartialMessages` (TypeScript) в значение `true` в ваших параметрах.

<Tip>
  На этой странице рассматривается потоковая передача выходных данных (получение токенов в реальном времени). Для режимов ввода (как вы отправляете сообщения), см. [Отправка сообщений агентам](/docs/ru/agent-sdk/streaming-vs-single-mode). Вы также можете [передавать ответы потоком с помощью Agent SDK через CLI](/docs/ru/headless).
</Tip>

<h2 id="enable-streaming-output">
  Включение потоковой передачи выходных данных
</h2>

Чтобы включить потоковую передачу, установите `include_partial_messages` (Python) или `includePartialMessages` (TypeScript) в значение `true` в ваших параметрах. Это заставляет SDK выдавать сообщения `StreamEvent`, содержащие необработанные события API по мере их поступления, в дополнение к обычным `AssistantMessage` и `ResultMessage`.

Ваш код затем должен:

1. Проверить тип каждого сообщения, чтобы отличить `StreamEvent` от других типов сообщений
2. Для `StreamEvent` извлечь поле `event` и проверить его `type`
3. Искать события `content_block_delta`, где `delta.type` — это `text_delta`, которые содержат фактические текстовые фрагменты

Пример ниже включает потоковую передачу и выводит текстовые фрагменты по мере их поступления. Обратите внимание на вложенные проверки типов: сначала для `StreamEvent`, затем для `content_block_delta`, затем для `text_delta`:

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
  Справочник StreamEvent
</h2>

Когда включены частичные сообщения, вы получаете необработанные события потоковой передачи Claude API, завернутые в объект. Тип имеет разные имена в каждом SDK:

* **Python**: `StreamEvent` (импортируется из `claude_agent_sdk.types`)
* **TypeScript**: `SDKPartialAssistantMessage` с `type: 'stream_event'`

Оба содержат необработанные события Claude API, а не накопленный текст. Вам нужно самостоятельно извлекать и накапливать текстовые дельты. Вот структура каждого типа:

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

Поле `parent_tool_use_id` всегда имеет значение `None` в Python и `null` в TypeScript. События потока генерируются только для основной сессии; дельты на уровне токенов от подагентов не пересылаются. Чтобы атрибутировать вывод подагенту, используйте полные сообщения, которые содержат `parent_tool_use_id`. См. [Обнаружение вызова подагента](/docs/ru/agent-sdk/subagents#detect-subagent-invocation).

Поле `event` содержит необработанное событие потоковой передачи из [Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Распространенные типы событий включают:

| Тип события           | Описание                                                              |
| :-------------------- | :-------------------------------------------------------------------- |
| `message_start`       | Начало нового сообщения                                               |
| `content_block_start` | Начало нового блока содержимого (текст или использование инструмента) |
| `content_block_delta` | Добавочное обновление содержимого                                     |
| `content_block_stop`  | Конец блока содержимого                                               |
| `message_delta`       | Обновления на уровне сообщения (причина остановки, использование)     |
| `message_stop`        | Конец сообщения                                                       |

<h2 id="message-flow">
  Поток сообщений
</h2>

С включенными частичными сообщениями вы получаете сообщения в этом порядке:

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

Без включенных частичных сообщений (`include_partial_messages` в Python, `includePartialMessages` в TypeScript) вы получаете все типы сообщений, кроме `StreamEvent`. Распространенные типы включают `SystemMessage` (инициализация сеанса), `AssistantMessage` (полные ответы), `ResultMessage` (финальный результат) и компактное граничное сообщение, указывающее на то, когда история разговора была сжата (`SDKCompactBoundaryMessage` в TypeScript; `SystemMessage` с подтипом `"compact_boundary"` в Python).

<h2 id="stream-text-responses">
  Потоковая передача текстовых ответов
</h2>

Чтобы отобразить текст по мере его генерации, ищите события `content_block_delta`, где `delta.type` — это `text_delta`. Они содержат добавочные текстовые фрагменты. Пример ниже выводит каждый фрагмент по мере его поступления:

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
  Потоковая передача вызовов инструментов
</h2>

Вызовы инструментов также передаются потоком добавочно. Вы можете отследить, когда инструменты начинают работу, получить их входные данные по мере их генерации и увидеть, когда они завершаются. Пример ниже отслеживает текущий вызываемый инструмент и накапливает входные данные JSON по мере их поступления. Он использует три типа событий:

* `content_block_start`: инструмент начинает работу
* `content_block_delta` с `input_json_delta`: поступают фрагменты входных данных
* `content_block_stop`: вызов инструмента завершен

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
  Создание пользовательского интерфейса потоковой передачи
</h2>

Этот пример объединяет потоковую передачу текста и инструментов в единый пользовательский интерфейс. Он отслеживает, выполняет ли агент в настоящее время инструмент (используя флаг `in_tool`), чтобы показать индикаторы состояния, такие как `[Using Read...]`, пока работают инструменты. Текст передается потоком нормально, когда инструмент не используется, а завершение инструмента вызывает сообщение "done". Этот паттерн полезен для интерфейсов чата, которым нужно показывать прогресс во время многошаговых задач агента.

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
  Известные ограничения
</h2>

* **Structured output**: результат JSON появляется только в финальном `ResultMessage.structured_output`, а не как потоковые дельта-обновления. Подробнее см. в разделе [structured outputs](/docs/ru/agent-sdk/structured-outputs).

<h2 id="next-steps">
  Следующие шаги
</h2>

Теперь, когда вы можете передавать текст и вызовы инструментов потоком в реальном времени, изучите эти связанные темы:

* [Интерактивные и одноразовые запросы](/docs/ru/agent-sdk/streaming-vs-single-mode): выберите режим ввода для вашего случая использования
* [Структурированные выходные данные](/docs/ru/agent-sdk/structured-outputs): получайте типизированные ответы JSON от агента
* [Разрешения](/docs/ru/agent-sdk/permissions): контролируйте, какие инструменты может использовать агент
