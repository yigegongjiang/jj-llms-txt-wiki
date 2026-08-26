> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Обработка одобрений и пользовательского ввода

> Выводите запросы на одобрение Claude и уточняющие вопросы пользователям, а затем возвращайте их решения в SDK.

Во время работы над задачей Claude иногда нужно проверить информацию у пользователей. Ему может потребоваться разрешение перед удалением файлов или нужно спросить, какую базу данных использовать для нового проекта. Ваше приложение должно выводить эти запросы пользователям, чтобы Claude мог продолжить работу с их вводом.

Claude запрашивает пользовательский ввод в двух ситуациях: когда ему нужно **разрешение на использование инструмента** (например, удаление файлов или запуск команд) и когда у него есть **уточняющие вопросы** (через инструмент `AskUserQuestion`). Оба случая запускают ваш callback `canUseTool`, который приостанавливает выполнение до получения ответа. Это отличается от обычных диалоговых ходов, где Claude завершает работу и ждёт вашего следующего сообщения.

Для уточняющих вопросов Claude генерирует вопросы и варианты ответов. Ваша роль — представить их пользователям и вернуть их выборы. Вы не можете добавлять свои собственные вопросы в этот процесс; если вам нужно что-то спросить у пользователей, сделайте это отдельно в логике вашего приложения.

Callback может оставаться в ожидании неопределённо долго. Выполнение остаётся приостановленным до возврата вашего callback, и SDK отменяет ожидание только при отмене самого запроса. Если пользователь может ответить дольше, чем ваш процесс может разумно оставаться запущенным, верните решение [`defer` hook](/docs/ru/hooks#defer-a-tool-call-for-later), которое позволяет процессу выйти и возобновиться позже из сохранённой сессии.

Это руководство показывает, как обнаружить каждый тип запроса и ответить надлежащим образом.

<h2 id="detect-when-claude-needs-input">
  Обнаружение, когда Claude нуждается в вводе
</h2>

Передайте callback `canUseTool` в параметры вашего запроса. Callback срабатывает всякий раз, когда Claude нуждается в пользовательском вводе, получая имя инструмента и ввод в качестве аргументов:

<CodeGroup>
  ```python Python theme={null}
  async def handle_tool_request(tool_name, input_data, context):
      # Запросить у пользователя и вернуть разрешение или отказ
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options включает { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Запросить у пользователя и вернуть разрешение или отказ
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

Callback срабатывает в двух случаях:

1. **Инструмент требует одобрения**: Claude хочет использовать инструмент, который не одобрен автоматически [правилом разрешений](/docs/ru/agent-sdk/permissions) или режимом разрешений. Проверьте `tool_name` на имя инструмента (например, `"Bash"`, `"Write"`).
2. **Claude задаёт вопрос**: Claude вызывает инструмент `AskUserQuestion`. Проверьте, равен ли `tool_name == "AskUserQuestion"`, чтобы обработать его иначе. Если вы указываете массив `tools`, включите `AskUserQuestion` для работы этого функционала. Подробнее см. [Обработка уточняющих вопросов](#handle-clarifying-questions).

<Warning>
  **Callback никогда не срабатывает для автоматически одобренных инструментов.** Любое одобрение на более ранних этапах [потока оценки разрешений](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated), правило разрешения или режим, такой как `acceptEdits` или `bypassPermissions`, разрешает вызов до того, как будет проверен `canUseTool`. Если вы указываете инструмент в `allowed_tools`, проверка `canUseTool` для этого инструмента никогда не выполняется, если только правило ask или режим `plan` не перенаправляют вызов обратно в prompt. Для логики, которая должна применяться к каждому вызову инструмента, используйте [`PreToolUse` hook](/docs/ru/agent-sdk/hooks), который выполняется перед остальной частью потока и может разрешить, отклонить или изменить запросы.

  `AskUserQuestion`, инструменты MCP, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), и инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) достигают callback даже когда правило разрешения совпадает. В режиме `dontAsk` эти вызовы вместо этого отклоняются без вызова callback.
</Warning>

Вы также можете использовать [`PermissionRequest` hook](/docs/ru/agent-sdk/hooks#available-hooks) для отправки внешних уведомлений (Slack, email, push) когда Claude ждёт одобрения.

<h2 id="handle-tool-approval-requests">
  Обработка запросов на одобрение инструмента
</h2>

После передачи callback `canUseTool` в параметры вашего запроса он срабатывает, когда Claude хочет использовать инструмент, который не одобрен ранее в потоке разрешений. Ваш callback получает три аргумента:

| Аргумент                            | Описание                                                                                                                                                                                                                                                                                                                                                           |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `toolName`                          | Имя инструмента, который Claude хочет использовать (например, `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                                       |
| `input`                             | Параметры, которые Claude передаёт инструменту. Содержимое варьируется в зависимости от инструмента.                                                                                                                                                                                                                                                               |
| `options` (TS) / `context` (Python) | Дополнительный контекст, включая опциональные `suggestions` (предложенные записи `PermissionUpdate` для избежания повторного запроса) и сигнал отмены. В TypeScript `signal` — это `AbortSignal`; в Python поле signal зарезервировано для будущего использования. Подробнее см. [`ToolPermissionContext`](/docs/ru/agent-sdk/python#toolpermissioncontext) для Python. |

Объект `input` содержит параметры, специфичные для инструмента. Распространённые примеры:

| Инструмент | Поля ввода                              |
| ---------- | --------------------------------------- |
| `Bash`     | `command`, `description`, `timeout`     |
| `Write`    | `file_path`, `content`                  |
| `Edit`     | `file_path`, `old_string`, `new_string` |
| `Read`     | `file_path`, `offset`, `limit`          |

Полные схемы ввода см. в справочнике SDK: [Python](/docs/ru/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/ru/agent-sdk/typescript#tool-input-types).

Вы можете отобразить эту информацию пользователю, чтобы он мог решить, разрешить или отклонить действие, а затем вернуть соответствующий ответ.

Следующий пример просит Claude создать и удалить тестовый файл. Когда Claude пытается выполнить каждую операцию, callback выводит запрос инструмента в терминал и запрашивает одобрение y/n.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Отобразить запрос инструмента
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Получить одобрение пользователя
      response = input("Allow this action? (y/n): ")

      # Вернуть разрешение или отказ на основе ответа пользователя
      if response.lower() == "y":
          # Разрешить: инструмент выполняется с исходным (или изменённым) вводом
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Отклонить: инструмент не выполняется, Claude видит сообщение
          return PermissionResultDeny(message="User denied this action")


  # Требуемый обходной путь: фиктивный hook держит поток открытым для can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Вспомогательная функция для запроса ввода пользователя в терминале
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Отобразить запрос инструмента
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Получить одобрение пользователя
        const response = await prompt("Allow this action? (y/n): ");

        // Вернуть разрешение или отказ на основе ответа пользователя
        if (response.toLowerCase() === "y") {
          // Разрешить: инструмент выполняется с исходным (или изменённым) вводом
          return { behavior: "allow", updatedInput: input };
        } else {
          // Отклонить: инструмент не выполняется, Claude видит сообщение
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<Note>
  В Python `can_use_tool` требует [режима потоковой передачи](/docs/ru/agent-sdk/streaming-vs-single-mode). Когда вы передаёте конечный поток сообщений через `query(prompt=generator)` или `ClaudeSDKClient.connect(prompt=async_iterable)`, SDK закрывает входной поток после последнего сообщения, прежде чем callback разрешения сможет быть вызван, если зарегистрированный hook или встроенный MCP-сервер не держит его открытым. Пример выше держит его открытым с помощью hook `PreToolUse`, который возвращает `{"continue_": True}`. Подключение без запроса и отправка сообщений через `ClaudeSDKClient.query()` держит поток открытым самостоятельно и не требует hook.
</Note>

Этот пример использует поток y/n, где любой ввод, отличный от `y`, рассматривается как отказ. На практике вы можете создать более богатый пользовательский интерфейс, который позволяет пользователям изменять запрос, предоставлять обратную связь или полностью перенаправлять Claude. Подробнее см. [Ответ на запросы инструментов](#respond-to-tool-requests).

<h3 id="respond-to-tool-requests">
  Ответ на запросы инструментов
</h3>

Ваш callback возвращает один из двух типов ответов:

| Ответ         | Python                                     | TypeScript                            |
| ------------- | ------------------------------------------ | ------------------------------------- |
| **Разрешить** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Отклонить** | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

При разрешении инструмент выполняется с вводом, который запросил Claude, если вы не вернёте изменённый ввод, `updatedInput` в TypeScript или `updated_input` в Python. До версии 2.1.207 Claude Code отклонял результат разрешения, который опускал `updatedInput`, и отклонял вызов инструмента с ошибкой валидации.

При отклонении предоставьте сообщение, объясняющее причину. Claude видит это сообщение и может скорректировать свой подход.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk.types import PermissionResultAllow, PermissionResultDeny

  # Разрешить выполнение инструмента
  return PermissionResultAllow(updated_input=input_data)

  # Заблокировать инструмент
  return PermissionResultDeny(message="User rejected this action")
  ```

  ```typescript TypeScript theme={null}
  // Разрешить выполнение инструмента
  return { behavior: "allow", updatedInput: input };

  // Заблокировать инструмент
  return { behavior: "deny", message: "User rejected this action" };
  ```
</CodeGroup>

Помимо разрешения или отклонения, вы можете изменить ввод инструмента или предоставить контекст, который помогает Claude скорректировать свой подход:

* **Одобрить**: позволить инструменту выполниться так, как запросил Claude
* **Одобрить с изменениями**: изменить ввод перед выполнением (например, санитизировать пути, добавить ограничения)
* **Одобрить и запомнить**: отправить обратно предложенное правило разрешения, чтобы совпадающие вызовы пропускали запрос в следующий раз
* **Отклонить**: заблокировать инструмент и объяснить Claude причину
* **Предложить альтернативу**: заблокировать, но направить Claude к тому, что хочет пользователь
* **Полностью перенаправить**: использовать [потоковый ввод](/docs/ru/agent-sdk/streaming-vs-single-mode) для отправки Claude совершенно новой инструкции

<Tabs>
  <Tab title="Одобрить">
    Пользователь одобряет действие как есть. Пропустите `input` из вашего callback без изменений и инструмент выполнится ровно так, как запросил Claude.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Одобрить с изменениями">
    Пользователь одобряет, но хочет сначала изменить запрос. Вы можете изменить ввод перед выполнением инструмента. Claude видит результат, но не сообщается, что вы что-то изменили. Полезно для санитизации параметров, добавления ограничений или ограничения доступа.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # Пользователь одобрил, но ограничить все команды песочницей
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // Пользователь одобрил, но ограничить все команды песочницей
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Одобрить и запомнить">
    Пользователь одобряет и не хочет, чтобы его спрашивали снова для этого вида вызова. Третий аргумент callback содержит `suggestions`, массив готовых записей [`PermissionUpdate`](/docs/ru/agent-sdk/typescript#permissionupdate). Отправьте одну обратно в `updatedPermissions` для её применения. Предложение с назначением `localSettings` записывает правило в `.claude/settings.local.json`, чтобы будущие сеансы пропускали запрос для совпадающих вызовов.

    Пример Python требует `claude-agent-sdk` версии 0.1.80 или позже.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Отклонить">
    Пользователь не хочет, чтобы это действие произошло. Заблокируйте инструмент и предоставьте сообщение, объясняющее причину. Claude видит это сообщение и может попробовать другой подход.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Предложить альтернативу">
    Пользователь не хочет это конкретное действие, но имеет другую идею. Заблокируйте инструмент и включите рекомендацию в ваше сообщение. Claude прочитает это и решит, как действовать на основе вашей обратной связи.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # Пользователь не хочет удалять, предложить архивирование вместо этого
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // Пользователь не хочет удалять, предложить архивирование вместо этого
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Полностью перенаправить">
    Для полного изменения направления (не просто подсказка), используйте [потоковый ввод](/docs/ru/agent-sdk/streaming-vs-single-mode) для отправки Claude новой инструкции напрямую. Это обходит текущий запрос инструмента и даёт Claude совершенно новые инструкции для следования.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Обработка уточняющих вопросов
</h2>

Когда Claude нуждается в дополнительном направлении для задачи с несколькими допустимыми подходами, он вызывает инструмент `AskUserQuestion`. Это запускает ваш callback `canUseTool` с `toolName`, установленным на `AskUserQuestion`. Ввод содержит вопросы Claude в виде вариантов с множественным выбором, которые вы выводите пользователю и возвращаете их выборы.

<Tip>
  Уточняющие вопросы особенно распространены в [режиме `plan`](/docs/ru/agent-sdk/permissions#plan-mode-plan), где Claude исследует кодовую базу и задаёт вопросы перед предложением плана. Это делает режим plan идеальным для интерактивных рабочих процессов, где вы хотите, чтобы Claude собрал требования перед внесением изменений.
</Tip>

Следующие шаги показывают, как обработать уточняющие вопросы:

<Steps>
  <Step title="Передайте callback canUseTool">
    Передайте callback `canUseTool` в параметры вашего запроса. По умолчанию `AskUserQuestion` доступен. Если вы указываете массив `tools` для ограничения возможностей Claude (например, агент только для чтения с только `Read`, `Glob` и `Grep`), включите `AskUserQuestion` в этот массив. В противном случае Claude не сможет задавать уточняющие вопросы:

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Включите AskUserQuestion в ваш список инструментов
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Включите AskUserQuestion в ваш список инструментов
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Обработайте уточняющие вопросы здесь
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Обнаружьте AskUserQuestion">
    В вашем callback проверьте, равен ли `toolName` `AskUserQuestion`, чтобы обработать его иначе, чем другие инструменты:

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # Ваша реализация для сбора ответов от пользователя
              return await handle_clarifying_questions(input_data)
          # Обработайте другие инструменты нормально
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // Ваша реализация для сбора ответов от пользователя
          return handleClarifyingQuestions(input);
        }
        // Обработайте другие инструменты нормально
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Разберите ввод вопроса">
    Ввод содержит вопросы Claude в массиве `questions`. Каждый вопрос имеет `question` (текст для отображения), `options` (варианты выбора) и `multiSelect` (разрешены ли множественные выборы):

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Полное описание полей см. в [Формат вопроса](#question-format).
  </Step>

  <Step title="Соберите ответы от пользователя">
    Представьте вопросы пользователю и соберите их выборы. Как вы это сделаете, зависит от вашего приложения: терминальный запрос, веб-форма, мобильный диалог и т. д.
  </Step>

  <Step title="Верните ответы Claude">
    Создайте объект `answers` как запись, где каждый ключ — это текст `question`, а каждое значение — это `label` выбранного варианта:

    | Из объекта вопроса                                              | Используйте как |
    | --------------------------------------------------------------- | --------------- |
    | Поле `question` (например, `"How should I format the output?"`) | Ключ            |
    | Поле `label` выбранного варианта (например, `"Summary"`)        | Значение        |

    Для вопросов с множественным выбором передайте массив меток или объедините их с `", "`. Если вы [поддерживаете свободный ввод текста](#support-free-text-input), используйте пользовательский текст пользователя как значение.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Формат вопроса
</h3>

Ввод содержит сгенерированные Claude вопросы в массиве `questions`. Каждый вопрос имеет эти поля:

| Поле          | Описание                                                                                                                                      |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `question`    | Полный текст вопроса для отображения                                                                                                          |
| `header`      | Короткая метка для вопроса (максимум 12 символов)                                                                                             |
| `options`     | Массив из 2-4 вариантов выбора, каждый с `label` и `description`. TypeScript: опционально `preview` (см. [ниже](#option-previews-typescript)) |
| `multiSelect` | Если `true`, пользователи могут выбрать несколько вариантов                                                                                   |

Структура, которую получает ваш callback:

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Предпросмотры вариантов (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` добавляет поле `preview` к каждому варианту, чтобы ваше приложение могло показать визуальный макет рядом с меткой. Без этого параметра Claude не генерирует предпросмотры и поле отсутствует.

| `previewFormat`               | `preview` содержит                                                                                                 |
| :---------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| не установлено (по умолчанию) | Поле отсутствует. Claude не генерирует предпросмотры.                                                              |
| `"markdown"`                  | ASCII-арт и блоки кода в ограде                                                                                    |
| `"html"`                      | Стилизованный фрагмент `<div>` (SDK отклоняет `<script>`, `<style>` и `<!DOCTYPE>` перед запуском вашего callback) |

Формат применяется ко всем вопросам в сессии. Claude включает `preview` на варианты, где визуальное сравнение помогает (выбор макета, цветовые схемы) и опускает его, где оно не помогает (да/нет подтверждения, только текстовые варианты). Проверьте `undefined` перед рендерингом.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview — это HTML-строка или undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Вариант с HTML-предпросмотром:

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Формат ответа
</h3>

Верните объект `answers`, сопоставляющий поле `question` каждого вопроса с `label` выбранного варианта:

| Поле        | Описание                                                                                           |
| ----------- | -------------------------------------------------------------------------------------------------- |
| `questions` | Пропустите исходный массив вопросов (требуется для обработки инструмента)                          |
| `answers`   | Объект, где ключи — это текст вопроса, а значения — это выбранные метки                            |
| `response`  | Опциональный свободный ответ, который пользователь ввёл вместо ответа на структурированные вопросы |

Для вопросов с множественным выбором передайте массив меток или объедините их с `", "`. Для свободного ввода текста для каждого вопроса, такого как опция "Other", поместите пользовательский текст пользователя в `answers[question]`, как показано в [Поддержка свободного ввода текста](#support-free-text-input). Установите `response` только когда ваш пользовательский интерфейс позволяет пользователю закрыть карточку вопроса и ввести общий ответ, который не является ответом на какой-либо конкретный вопрос. Когда установлен `response`, Claude получает "The user responded: …" вместо списка ответов для каждого вопроса.

```json theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Поддержка свободного ввода текста
</h4>

Предопределённые варианты Claude не всегда охватывают то, что хотят пользователи. Чтобы позволить пользователям вводить свой собственный ответ:

* Отобразите дополнительный выбор "Other" после вариантов Claude, который принимает текстовый ввод
* Используйте пользовательский текст пользователя как значение ответа (не слово "Other")

Полную реализацию см. в [полном примере](#complete-example) ниже.

<h3 id="complete-example">
  Полный пример
</h3>

Claude задаёт уточняющие вопросы, когда ему нужен пользовательский ввод для продолжения. Например, когда его просят помочь решить, какой технологический стек использовать для мобильного приложения, Claude может спросить о кроссплатформенности vs нативности, предпочтениях бэкенда или целевых платформах. Эти вопросы помогают Claude принимать решения, которые соответствуют предпочтениям пользователя, а не угадывать.

Этот пример обрабатывает эти вопросы в терминальном приложении. Вот что происходит на каждом шаге:

1. **Маршрутизация запроса**: callback `canUseTool` проверяет, равно ли имя инструмента `"AskUserQuestion"` и маршрутизирует к выделенному обработчику
2. **Отображение вопросов**: обработчик проходит по массиву `questions` и выводит каждый вопрос с пронумерованными вариантами
3. **Сбор ввода**: пользователь может ввести номер для выбора варианта или ввести свободный текст напрямую (например, "jquery", "i don't know")
4. **Сопоставление ответов**: код проверяет, является ли ввод числовым (использует метку варианта) или свободным текстом (использует текст напрямую)
5. **Возврат Claude**: ответ включает как исходный массив `questions`, так и сопоставление `answers`

Сохраните версию TypeScript как `ask.ts` и запустите её с помощью `npx tsx ask.ts`, или сохраните версию Python как `ask.py` и запустите её с помощью `python ask.py`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Разберите пользовательский ввод как номер(а) варианта или свободный текст."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Отобразите вопросы Claude и соберите ответы пользователя."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Маршрутизируйте AskUserQuestion к нашему обработчику вопросов
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Автоматически одобрите другие инструменты для этого примера
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Требуемый обходной путь: фиктивный hook держит поток открытым для can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Вспомогательная функция для запроса ввода пользователя в терминале
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Разберите пользовательский ввод как номер(а) варианта или свободный текст
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Отобразите вопросы Claude и соберите ответы пользователя
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Верните ответы Claude (должны включать исходные вопросы)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Маршрутизируйте AskUserQuestion к нашему обработчику вопросов
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Автоматически одобрите другие инструменты для этого примера
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Ограничения
</h2>

* **Подагенты**: `AskUserQuestion` в настоящее время недоступен в подагентах, порождённых через инструмент Agent
* **Ограничения вопросов**: каждый вызов `AskUserQuestion` поддерживает 1-4 вопроса с 2-4 вариантами каждый

<h2 id="other-ways-to-get-user-input">
  Другие способы получить пользовательский ввод
</h2>

Callback `canUseTool` и инструмент `AskUserQuestion` охватывают большинство сценариев одобрения и уточнения, но SDK предлагает другие способы получить ввод от пользователей:

<h3 id="streaming-input">
  Потоковый ввод
</h3>

Используйте [потоковый ввод](/docs/ru/agent-sdk/streaming-vs-single-mode) когда вам нужно:

* **Прервать агента в середине задачи**: отправить сигнал отмены или изменить направление, пока Claude работает
* **Предоставить дополнительный контекст**: добавить информацию, которая нужна Claude, без ожидания, пока он спросит
* **Создать интерфейсы чата**: позволить пользователям отправлять последующие сообщения во время долгоживущих операций

Потоковый ввод идеален для разговорных пользовательских интерфейсов, где пользователи взаимодействуют с агентом на протяжении всего выполнения, а не только в контрольных точках одобрения.

<h3 id="custom-tools">
  Пользовательские инструменты
</h3>

Используйте [пользовательские инструменты](/docs/ru/agent-sdk/custom-tools) когда вам нужно:

* **Собрать структурированный ввод**: создать формы, мастера или многошаговые рабочие процессы, которые выходят за рамки формата множественного выбора `AskUserQuestion`
* **Интегрировать внешние системы одобрения**: подключиться к существующим системам тикетов, рабочих процессов или одобрения
* **Реализовать взаимодействия, специфичные для домена**: создать инструменты, адаптированные к потребностям вашего приложения, такие как интерфейсы проверки кода или контрольные списки развёртывания

Пользовательские инструменты дают вам полный контроль над взаимодействием, но требуют больше работы по реализации, чем использование встроенного callback `canUseTool`.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Настройка разрешений](/docs/ru/agent-sdk/permissions): установите режимы и правила разрешений
* [Управление выполнением с помощью hooks](/docs/ru/agent-sdk/hooks): запустите пользовательский код в ключевых точках жизненного цикла агента
* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript#canusetool): полная документация API canUseTool
