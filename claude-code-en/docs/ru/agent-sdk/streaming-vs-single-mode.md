> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Streaming Input

> Понимание двух режимов ввода для Claude Agent SDK и когда использовать каждый

<h2 id="overview">
  Обзор
</h2>

Claude Agent SDK поддерживает два различных режима ввода для взаимодействия с агентами:

* **Режим Streaming Input**: постоянная интерактивная сессия
* **Single Message Input**: одноразовые запросы, которые используют состояние сессии и возобновление

<h2 id="streaming-input-mode-recommended">
  Режим Streaming Input (Рекомендуется)
</h2>

Режим streaming input - это **предпочтительный** способ использования Claude Agent SDK. Он обеспечивает полный доступ к возможностям агента и позволяет создавать богатые интерактивные впечатления.

Он позволяет агенту работать как долгоживущий процесс, который принимает пользовательский ввод, обрабатывает прерывания, выводит запросы разрешений и управляет сессией.

<h3 id="benefits">
  Преимущества
</h3>

В режиме streaming input вы работаете в постоянной сессии со следующими возможностями:

* **Загрузка изображений**: прикрепляйте изображения непосредственно к сообщениям для визуального анализа и понимания
* **Очередь сообщений**: отправляйте несколько сообщений, которые обрабатываются последовательно, с возможностью прерывания
* **Интеграция инструментов**: полный доступ ко всем инструментам и пользовательским MCP серверам во время сессии
* **Обратная связь в реальном времени**: смотрите ответы по мере их создания, а не только финальные результаты
* **Сохранение контекста**: сохраняйте контекст разговора между несколькими ходами естественным образом

<h3 id="implementation-example">
  Пример реализации
</h3>

Эти примеры читают изображение с именем `diagram.png` из рабочей директории. Создайте его там в первую очередь или измените имя файла, чтобы указать на ваше собственное изображение.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";
  import { readFile } from "fs/promises";

  async function* generateMessages(): AsyncGenerator<SDKUserMessage> {
    // First message
    yield {
      type: "user",
      message: {
        role: "user",
        content: "Analyze this codebase for security issues"
      },
      parent_tool_use_id: null
    };

    // Wait for conditions or user input
    await new Promise((resolve) => setTimeout(resolve, 2000));

    // Follow-up with image
    yield {
      type: "user",
      message: {
        role: "user",
        content: [
          {
            type: "text",
            text: "Review this architecture diagram"
          },
          {
            type: "image",
            source: {
              type: "base64",
              media_type: "image/png",
              data: await readFile("diagram.png", "base64")
            }
          }
        ]
      },
      parent_tool_use_id: null
    };
  }

  // Process streaming responses
  for await (const message of query({
    prompt: generateMessages(),
    options: {
      maxTurns: 10,
      allowedTools: ["Read", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import (
      ClaudeSDKClient,
      ClaudeAgentOptions,
      AssistantMessage,
      TextBlock,
  )
  import asyncio
  import base64


  async def streaming_analysis():
      async def message_generator():
          # First message
          yield {
              "type": "user",
              "message": {
                  "role": "user",
                  "content": "Analyze this codebase for security issues",
              },
          }

          # Wait for conditions
          await asyncio.sleep(2)

          # Follow-up with image
          with open("diagram.png", "rb") as f:
              image_data = base64.b64encode(f.read()).decode()

          yield {
              "type": "user",
              "message": {
                  "role": "user",
                  "content": [
                      {"type": "text", "text": "Review this architecture diagram"},
                      {
                          "type": "image",
                          "source": {
                              "type": "base64",
                              "media_type": "image/png",
                              "data": image_data,
                          },
                      },
                  ],
              },
          }

      # Use ClaudeSDKClient for streaming input
      options = ClaudeAgentOptions(max_turns=10, allowed_tools=["Read", "Grep"])

      async with ClaudeSDKClient(options) as client:
          # Send streaming input
          await client.query(message_generator())

          # Process responses
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, TextBlock):
                          print(block.text)


  asyncio.run(streaming_analysis())
  ```
</CodeGroup>

Когда вы запустите пример, версия TypeScript выводит каждый ответ по мере его завершения. Цикл `receive_response()` версии Python заканчивается на первом сообщении результата, поэтому он выводит анализ безопасности; чтобы прочитать оба ответа, используйте одну пару `query()` и `receive_response()` на сообщение, как показано в [примере продолжения разговора в справочнике Python](/docs/ru/agent-sdk/python#example-continuing-a-conversation).

<Note>
  В TypeScript SDK, если ваш генератор сообщений выбросит исключение, например, когда файл, который он читает, отсутствует, поток завершится с ошибкой, которая гласит `Claude Code process aborted by user` вместо исходной ошибки, поэтому сначала проверьте код внутри вашего генератора, когда вы видите это сообщение. Ошибке также может предшествовать длинная минифицированная строка объединённого исходного кода SDK, поэтому прочитайте до конца вывода, чтобы найти текст ошибки.

  В Python SDK исключение генератора регистрируется на уровне отладки, и сессия зависает без выброса исключения, поэтому если сессия streaming зависает без вывода, включите логирование отладки и проверьте ваш генератор.
</Note>

<h2 id="single-message-input">
  Ввод одного сообщения
</h2>

Ввод одного сообщения проще, но более ограничен.

<h3 id="when-to-use-single-message-input">
  Когда использовать ввод одного сообщения
</h3>

Используйте ввод одного сообщения когда:

* Вам нужен одноразовый ответ
* Вам не нужны вложения изображений или методы управления в середине сеанса
* Вам нужно работать в безгосударственной среде, такой как lambda функция

<h3 id="limitations">
  Ограничения
</h3>

<Warning>
  Режим ввода одного сообщения **не** поддерживает:

  * Прямое вложение изображений в сообщения
  * Динамическую очередь сообщений
  * Прерывание в реальном времени
  * Естественные многоходовые разговоры
</Warning>

Если запрос заканчивается результатом ошибки, например `error_max_turns`, один вызов `query()` вызывает ошибку, которая включает текст сбоя после выдачи финального сообщения результата, поэтому оберните цикл в блок try, если вашему коду нужно продолжить работу. См. [Обработка результата](/docs/ru/agent-sdk/agent-loop#handle-the-result) для подтипов результатов.

<h3 id="implementation-example-2">
  Пример реализации
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Simple one-shot query
  // query() throws after an error result, such as error_max_turns
  try {
    for await (const message of query({
      prompt: "Explain the authentication flow",
      options: {
        maxTurns: 5,
        allowedTools: ["Read", "Grep"]
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    console.error(`Query failed: ${error}`);
  }

  // Continue conversation with session management
  try {
    for await (const message of query({
      prompt: "Now explain the authorization process",
      options: {
        continue: true,
        maxTurns: 5
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    console.error(`Query failed: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
  import asyncio


  async def single_message_example():
      # Simple one-shot query using query() function
      # query() raises ResultError after an error result, such as error_max_turns
      try:
          async for message in query(
              prompt="Explain the authentication flow",
              options=ClaudeAgentOptions(max_turns=5, allowed_tools=["Read", "Grep"]),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as e:
          print(f"Query failed: {e}")

      # Continue conversation with session management
      try:
          async for message in query(
              prompt="Now explain the authorization process",
              options=ClaudeAgentOptions(continue_conversation=True, max_turns=5),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as e:
          print(f"Query failed: {e}")


  asyncio.run(single_message_example())
  ```
</CodeGroup>

Когда вы запустите пример, каждый запрос выведет его финальный текст результата: сначала объяснение аутентификации, затем объяснение авторизации.
