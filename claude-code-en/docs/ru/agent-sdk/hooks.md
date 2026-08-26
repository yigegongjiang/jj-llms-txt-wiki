> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Перехватывайте и контролируйте поведение агента с помощью hooks

> Перехватывайте и настраивайте поведение агента в ключевых точках выполнения с помощью hooks

Hooks — это функции обратного вызова, которые выполняют ваш код в ответ на события агента, такие как вызов инструмента, начало сеанса или остановка выполнения. С помощью hooks вы можете:

* **Блокировать опасные операции** перед их выполнением, такие как деструктивные команды shell или несанкционированный доступ к файлам
* **Логировать и аудировать** каждый вызов инструмента для соответствия требованиям, отладки или аналитики
* **Преобразовывать входные и выходные данные** для санитизации данных, внедрения учетных данных или перенаправления путей файлов
* **Требовать одобрение человека** для чувствительных действий, таких как запись в базу данных или вызовы API
* **Отслеживать жизненный цикл сеанса** для управления состоянием, очистки ресурсов или отправки уведомлений

Это руководство охватывает, как работают hooks, как их настроить, и предоставляет примеры для распространенных паттернов, таких как блокировка инструментов, изменение входных данных и перенаправление уведомлений.

<h2 id="how-hooks-work">
  Как работают hooks
</h2>

<Steps>
  <Step title="Срабатывает событие">
    Что-то происходит во время выполнения агента, и SDK срабатывает событие: инструмент вот-вот будет вызван (`PreToolUse`), инструмент вернул результат (`PostToolUse`), подагент запустился или остановился, агент неактивен или выполнение завершилось. См. [полный список событий](#available-hooks).
  </Step>

  <Step title="SDK собирает зарегистрированные hooks">
    SDK проверяет наличие hooks, зарегистрированных для этого типа события. Это включает callback hooks, которые вы передаете в `options.hooks`, и hooks команд shell из файлов настроек, когда соответствующая запись [`settingSources`](/docs/ru/agent-sdk/typescript#settingsource) или [`setting_sources`](/docs/ru/agent-sdk/python#settingsource) включена, что она есть для параметров `query()` по умолчанию.
  </Step>

  <Step title="Matchers фильтруют, какие hooks запускаются">
    Если hook имеет паттерн [`matcher`](#matchers) (например, `"Write|Edit"`), SDK проверяет его против цели события (например, имя инструмента). Hooks без matcher запускаются для каждого события этого типа.
  </Step>

  <Step title="Выполняются функции обратного вызова">
    Каждая функция [обратного вызова](#callback-functions) matching hook получает информацию о том, что происходит: имя инструмента, его аргументы, ID сеанса и другие детали, специфичные для события.
  </Step>

  <Step title="Ваш callback возвращает решение">
    После выполнения любых операций (логирование, вызовы API, валидация), ваш callback возвращает [объект вывода](#outputs), который говорит агенту, что делать: разрешить операцию, заблокировать ее, изменить входные данные или внедрить контекст в разговор.
  </Step>
</Steps>

Следующий пример объединяет эти шаги. Он регистрирует hook `PreToolUse` (шаг 1) с matcher `"Write|Edit"` (шаг 3), поэтому callback срабатывает только для инструментов записи файлов. При срабатывании callback получает входные данные инструмента (шаг 4), проверяет, нацелена ли путь файла на файл `.env`, и возвращает `permissionDecision: "deny"` для блокировки операции (шаг 5):

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Update the database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Update the database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

<h2 id="available-hooks">
  Доступные hooks
</h2>

SDK предоставляет hooks для различных этапов выполнения агента. Некоторые hooks доступны в обоих SDK, в то время как другие доступны только в TypeScript.

| Hook Event                                             | Python SDK | TypeScript SDK | Что его срабатывает                                                                             | Пример использования                                                          |
| ------------------------------------------------------ | ---------- | -------------- | ----------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `PreToolUse`                                           | Да         | Да             | Запрос вызова инструмента (может блокировать или изменять)                                      | Блокировать опасные команды shell                                             |
| `PostToolUse`                                          | Да         | Да             | Результат выполнения инструмента                                                                | Логировать все изменения файлов в журнал аудита                               |
| `PostToolUseFailure`                                   | Да         | Да             | Ошибка выполнения инструмента                                                                   | Обработать или логировать ошибки инструмента                                  |
| `PostToolBatch`                                        | Нет        | Да             | Полный пакет вызовов инструментов разрешается, один раз за пакет перед следующим вызовом модели | Внедрить соглашения один раз для всего пакета                                 |
| `UserPromptSubmit`                                     | Да         | Да             | Отправка пользовательского приглашения                                                          | Внедрить дополнительный контекст в приглашения                                |
| [`UserPromptExpansion`](/docs/ru/hooks#userpromptexpansion) | Нет        | Да             | Команда, введённая пользователем, расширяется в приглашение перед тем, как она достигнет Claude | Заблокировать команду от прямого вызова или добавить контекст при вводе skill |
| `MessageDisplay`                                       | Нет        | Да             | Сообщение ассистента с текстом завершается, один раз за сообщение с полным текстом сообщения    | Скрыть или переформатировать отображаемый текст без изменения стенограммы     |
| `Stop`                                                 | Да         | Да             | Остановка выполнения агента                                                                     | Сохранить состояние сеанса перед выходом                                      |
| `SubagentStart`                                        | Да         | Да             | Инициализация подагента                                                                         | Отслеживать порождение параллельных задач                                     |
| `SubagentStop`                                         | Да         | Да             | Завершение подагента                                                                            | Агрегировать результаты из параллельных задач                                 |
| `PreCompact`                                           | Да         | Да             | Запрос сжатия разговора                                                                         | Архивировать полную стенограмму перед суммированием                           |
| `PermissionRequest`                                    | Да         | Да             | Диалог разрешения будет отображен                                                               | Пользовательская обработка разрешений                                         |
| `SessionStart`                                         | Нет        | Да             | Инициализация сеанса                                                                            | Инициализировать логирование и телеметрию                                     |
| `SessionEnd`                                           | Нет        | Да             | Завершение сеанса                                                                               | Очистить временные ресурсы                                                    |
| `Notification`                                         | Да         | Да             | Сообщения о статусе агента                                                                      | Отправить обновления статуса агента в Slack или PagerDuty                     |
| `Setup`                                                | Нет        | Да             | Настройка/обслуживание сеанса                                                                   | Запустить задачи инициализации                                                |
| `TeammateIdle`                                         | Нет        | Да             | Товарищ по команде становится неактивным                                                        | Переназначить работу или уведомить                                            |
| `TaskCompleted`                                        | Нет        | Да             | Фоновая задача завершена                                                                        | Агрегировать результаты из параллельных задач                                 |
| `ConfigChange`                                         | Нет        | Да             | Файл конфигурации изменился                                                                     | Динамически перезагрузить настройки                                           |
| `WorktreeCreate`                                       | Нет        | Да             | Git worktree создан                                                                             | Отслеживать изолированные рабочие пространства                                |
| `WorktreeRemove`                                       | Нет        | Да             | Git worktree удален                                                                             | Очистить ресурсы рабочего пространства                                        |

<h2 id="configure-hooks">
  Настройка hooks
</h2>

Чтобы настроить hook, передайте его в поле `hooks` ваших параметров агента (`ClaudeAgentOptions` в Python, объект `options` в TypeScript):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Опция `hooks` — это словарь (Python) или объект (TypeScript), где:

* **Ключи**: [имена событий hook](#available-hooks), такие как `'PreToolUse'`, `'PostToolUse'` и `'Stop'`
* **Значения**: массивы [matchers](#matchers), каждый содержащий необязательный паттерн фильтра и ваши [функции обратного вызова](#callback-functions)

<h3 id="matchers">
  Matchers
</h3>

Используйте matchers для фильтрации, когда срабатывают ваши callbacks. Поле `matcher` соответствует другому значению в зависимости от типа события hook. Например, hooks на основе инструментов соответствуют имени инструмента, в то время как hooks `Notification` соответствуют типу уведомления. См. [справочник hooks Claude Code](/docs/ru/hooks#matcher-patterns) для полного списка значений matcher для каждого типа события.

SDK matchers следуют тем же правилам, что и [matchers в файлах настроек](/docs/ru/hooks#matcher-patterns). Matcher, содержащий только буквы, цифры, `_`, `-`, пробелы, `,` и `|`, сравнивается как точная строка, где альтернативы разделены `|` или `,` с необязательными окружающими пробелами, поэтому `Write|Edit` и `Write, Edit` каждый соответствует ровно этим двум инструментам, а `code-reviewer` соответствует только этому типу агента. Matcher `*`, пустая строка или отсутствие matcher вообще соответствует каждому возникновению события.

Matcher, содержащий любой другой символ, вычисляется как якорь-независимое регулярное выражение, поэтому `^mcp__` соответствует каждому MCP инструменту и `Edit.*` соответствует как `Edit`, так и `NotebookEdit`. Оберните регулярное выражение в `^` и `$`, когда вам нужно совпадение всей строки.

Matcher вроде `mcp__memory` или `mcp__brave-search` содержит только символы точного совпадения, поэтому он сравнивается как точная строка и не соответствует никакому инструменту; используйте `mcp__memory__.*` для соответствия каждому инструменту с этого сервера.

Дефисы в наборе точного совпадения требуют Claude Code runtime версии 2.1.195 или позже. В более ранних версиях имя с дефисом, такое как `code-reviewer`, вычисляется как якорь-независимое регулярное выражение и должно быть якорировано как `^code-reviewer$` для точного совпадения.

| Опция     | Тип              | По умолчанию | Описание                                                                                                                                                                                                                                                                                                                                                                                         |
| --------- | ---------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `matcher` | `string`         | `undefined`  | Паттерн, сопоставляемый с полем фильтра события, следуя правилам сравнения выше. Для hooks инструментов это имя инструмента. Встроенные инструменты включают `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` и другие (см. [Tool Input Types](/docs/ru/agent-sdk/typescript#tool-input-types) для полного списка). MCP инструменты используют паттерн `mcp__<server>__<action>`. |
| `hooks`   | `HookCallback[]` | -            | Обязательно. Массив функций обратного вызова для выполнения, когда паттерн совпадает                                                                                                                                                                                                                                                                                                             |
| `timeout` | `number`         | `60`         | Timeout в секундах                                                                                                                                                                                                                                                                                                                                                                               |

Используйте паттерн `matcher` для нацеливания на конкретные инструменты, когда это возможно. Matcher с `'Bash'` запускается только для команд Bash, в то время как опущение паттерна запускает ваши callbacks для каждого возникновения события.

Для tool-based hooks, matchers фильтруют только по имени инструмента, а не по путям файлов или другим аргументам. Для фильтрации по пути файла проверьте `tool_input.file_path` внутри вашего callback.

<Tip>
  **Обнаружение имен инструментов:** См. [Tool Input Types](/docs/ru/agent-sdk/typescript#tool-input-types) для полного списка встроенных имен инструментов, или добавьте hook без matcher для логирования всех вызовов инструментов, которые делает ваш сеанс.

  **Именование MCP инструментов:** MCP инструменты всегда начинаются с `mcp__`, за которым следует имя сервера и действие: `mcp__<server>__<action>`. Например, если вы настроите сервер с именем `playwright`, его инструменты будут названы `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click` и так далее. Имя сервера берется из ключа, который вы используете в конфигурации `mcpServers`.
</Tip>

<h3 id="callback-functions">
  Функции обратного вызова
</h3>

<h4 id="inputs">
  Входные данные
</h4>

Каждый callback hook получает три аргумента:

* **Входные данные:** типизированный объект, содержащий детали события. Каждый тип hook имеет свою форму входных данных. Например, `PreToolUseHookInput` включает `tool_name` и `tool_input`, в то время как `NotificationHookInput` включает `message`. См. полные определения типов в справочниках [TypeScript](/docs/ru/agent-sdk/typescript#hookinput) и [Python](/docs/ru/agent-sdk/python#hookinput) SDK.
  * Все входные данные hook содержат `session_id`, `cwd` и `hook_event_name`.
  * `agent_id` и `agent_type` заполняются, когда hook срабатывает внутри подагента. В TypeScript они находятся на базовом входе hook и доступны для всех типов hook. В Python они являются необязательными полями на `PreToolUse`, `PostToolUse`, `PostToolUseFailure` и `PermissionRequest`, и обязательными полями на `SubagentStart` и `SubagentStop`.
* **ID использования инструмента** (`str | None` / `string | undefined`): коррелирует события `PreToolUse` и `PostToolUse` для одного и того же вызова инструмента.
* **Контекст:** в TypeScript содержит свойство `signal` (`AbortSignal`) для отмены. В Python этот аргумент зарезервирован для будущего использования.

<h4 id="outputs">
  Выходные данные
</h4>

Ваш callback возвращает объект с двумя категориями полей:

* **Поля верхнего уровня** работают одинаково для каждого события: `systemMessage` показывает сообщение пользователю, и `continue` (`continue_` в Python) определяет, продолжает ли агент работать после этого hook.
* **`hookSpecificOutput`** контролирует текущую операцию. Поля внутри зависят от типа события hook. Для hooks `PreToolUse` здесь вы устанавливаете `permissionDecision` (`"allow"`, `"deny"`, `"ask"` или `"defer"`), `permissionDecisionReason` и `updatedInput`. Возврат `"defer"` завершает запрос, чтобы вы могли [возобновить его позже](/docs/ru/hooks#defer-a-tool-call-for-later). Для hooks `PostToolUse` вы можете установить `additionalContext` для добавления информации к результату инструмента. Чтобы заменить выходные данные инструмента перед тем, как Claude их увидит, установите `updatedToolOutput`, который работает для любого инструмента в обоих SDK. Более старое поле `updatedMCPToolOutput` заменяет только выходные данные MCP инструмента и является устаревшим.

Возвращайте `{}` для разрешения операции без изменений. SDK callback hooks используют тот же формат вывода JSON, что и [hooks команд shell Claude Code](/docs/ru/hooks#json-output), который документирует каждое поле и опцию, специфичную для события. Для определений типов SDK см. справочники [TypeScript](/docs/ru/agent-sdk/typescript#synchookjsonoutput) и [Python](/docs/ru/agent-sdk/python#synchookjsonoutput) SDK.

<Note>
  Когда применяются несколько hooks или правил разрешений, `deny` имеет приоритет над `defer`, который имеет приоритет над `ask`, который имеет приоритет над `allow`. Если какой-либо hook возвращает `deny`, операция блокируется независимо от других hooks.
</Note>

<h4 id="asynchronous-output">
  Асинхронный вывод
</h4>

По умолчанию агент ждет, пока ваш hook вернется, прежде чем продолжить. Если ваш hook выполняет побочный эффект, такой как логирование или отправка webhook, и не нужно влиять на поведение агента, вы можете вернуть асинхронный вывод вместо этого. Это говорит агенту продолжить немедленно без ожидания завершения hook:

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Поле           | Тип      | Описание                                                                                                                                        |
| -------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `async`        | `true`   | Сигнализирует асинхронный режим. Агент продолжает без ожидания. В Python используйте `async_` для избежания зарезервированного ключевого слова. |
| `asyncTimeout` | `number` | Необязательный timeout в миллисекундах для фоновой операции                                                                                     |

<Note>
  Асинхронные выходы не могут блокировать, изменять или внедрять контекст в операцию, так как агент уже продолжил. Используйте их только для побочных эффектов, таких как логирование, метрики или уведомления.
</Note>

<h2 id="examples">
  Примеры
</h2>

<h3 id="modify-tool-input">
  Изменение входных данных инструмента
</h3>

Этот пример перехватывает вызовы инструмента Write и переписывает аргумент `file_path` для добавления префикса `/sandbox`, перенаправляя все записи файлов в изолированный каталог. Callback возвращает `updatedInput` с измененным путем и `permissionDecision: 'allow'` для автоматического одобрения переписанной операции:

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  При использовании `updatedInput` вы также должны включить `permissionDecision: 'allow'` для автоматического одобрения измененного входа или `permissionDecision: 'ask'` для отображения его пользователю. С `'defer'` `updatedInput` игнорируется. Всегда возвращайте новый объект вместо мутирования оригинального `tool_input`.
</Note>

<h3 id="add-context-and-block-a-tool">
  Добавление контекста и блокировка инструмента
</h3>

Этот пример блокирует записи в каталог `/etc` и объясняет причину как модели, так и пользователю:

* `permissionDecision: 'deny'` останавливает вызов инструмента.
* `permissionDecisionReason` сообщает модели причину, чтобы она избежала повторной попытки.
* `systemMessage` показывает пользователю, что произошло.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Автоматическое одобрение конкретных инструментов
</h3>

По умолчанию агент может запросить разрешение перед использованием определенных инструментов. Этот пример автоматически одобряет инструменты файловой системы только для чтения (Read, Glob, Grep), возвращая `permissionDecision: 'allow'`, позволяя им запускаться без подтверждения пользователя, в то время как оставляя все остальные инструменты подлежащими обычным проверкам разрешений:

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Регистрация нескольких hooks
</h3>

Когда событие срабатывает, все соответствующие hooks выполняются параллельно. Для решений о разрешениях побеждает наиболее ограничивающий результат: одно `deny` блокирует вызов инструмента независимо от того, что возвращают другие hooks. Поскольку порядок завершения недетерминирован, напишите каждый hook так, чтобы он действовал независимо, а не полагаясь на то, что другой hook уже выполнился.

Пример ниже регистрирует три независимые проверки для каждого вызова инструмента:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Фильтрация с помощью multi-tool matchers
</h3>

Используйте multi-tool matchers для совместного использования одного callback для связанных инструментов. Этот пример регистрирует три matcher с разными областями:

* Список с разделением через трубу (`Write|Edit|Delete`) срабатывает `file_security_hook` только для инструментов модификации файлов.
* Regex (`^mcp__`) срабатывает `mcp_audit_hook` для любого MCP инструмента, имя которого начинается с `mcp__`.
* Пропущенный matcher срабатывает `global_logger` для каждого вызова инструмента независимо от имени.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|Delete", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|Delete", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Отслеживание активности подагента
</h3>

Используйте hooks `SubagentStop` для мониторинга, когда подагенты завершают свою работу. См. полный тип входных данных в справочниках [TypeScript](/docs/ru/agent-sdk/typescript#hookinput) и [Python](/docs/ru/agent-sdk/python#hookinput) SDK. Этот пример логирует сводку каждый раз, когда подагент завершается:

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  Выполнение HTTP запросов из hooks
</h3>

Hooks могут выполнять асинхронные операции, такие как HTTP запросы. Ловите ошибки внутри вашего hook вместо того, чтобы позволить им распространяться, так как необработанное исключение может прервать агента.

Этот пример отправляет webhook после завершения каждого инструмента, логируя, какой инструмент запустился и когда. Hook ловит ошибки, чтобы неудачный webhook не прерывал агента:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise. A failed webhook shouldn't stop the agent
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw. A failed webhook shouldn't stop the agent
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="forward-notifications-to-slack">
  Перенаправление уведомлений в Slack
</h3>

Используйте hooks `Notification` для получения системных уведомлений от агента и перенаправления их во внешние сервисы. Уведомления срабатывают для типов событий, таких как:

* `permission_prompt` когда Claude нужно разрешение
* `idle_prompt` когда Claude ждет ввода
* `auth_success` когда аутентификация завершена
* `elicitation_dialog`, `elicitation_complete` и `elicitation_response` для потоков запроса пользователя

Каждое уведомление включает поле `message` с описанием, понятным человеку, и опционально `title`.

Этот пример перенаправляет каждое уведомление в канал Slack. Требуется [URL входящего webhook Slack](https://api.slack.com/messaging/webhooks), который вы создаете, добавляя приложение в ваше рабочее пространство Slack и включая входящие webhooks:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="fix-common-issues">
  Исправление распространенных проблем
</h2>

<h3 id="hook-not-firing">
  Hook не срабатывает
</h3>

* Проверьте, что имя события hook правильное и чувствительно к регистру (`PreToolUse`, а не `preToolUse`)
* Проверьте, что ваш паттерн matcher точно совпадает с именем инструмента
* Убедитесь, что hook находится под правильным типом события в `options.hooks`
* Для non-tool hooks, которые поддерживают matchers, таких как `Notification` и `SubagentStop`, matchers соответствуют разным полям, и `Stop` полностью игнорирует matchers (см. [matcher patterns](/docs/ru/hooks#matcher-patterns))
* Hooks могут не срабатывать, когда агент достигает лимита [`max_turns`](/docs/ru/agent-sdk/python#claudeagentoptions), потому что сеанс заканчивается перед тем, как hooks смогут выполниться

<h3 id="matcher-not-filtering-as-expected">
  Matcher не фильтрует как ожидается
</h3>

Matchers соответствуют только имени инструмента, а не путям файлов или другим аргументам. Для фильтрации по пути файла проверьте `tool_input.file_path` внутри вашего hook:

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Hook timeout
</h3>

* Увеличьте значение `timeout` в конфигурации `HookMatcher`
* Используйте `AbortSignal` из третьего аргумента callback для корректной обработки отмены в TypeScript

Callback `UserPromptSubmit` или [`UserPromptExpansion`](/docs/ru/hooks#userpromptexpansion), который превышает свой timeout, блокирует этот запрос с сообщением timeout, и сеанс продолжается. Прерывание запроса во время ожидания callback отменяет ожидающий вызов инструмента. До версии 2.1.208 timeout callback на этих событиях завершал запрос с `error_during_execution`, и прерывание во время ожидания callback `PreToolUse` могло позволить вызову инструмента продолжиться.

<h3 id="tool-blocked-unexpectedly">
  Инструмент заблокирован неожиданно
</h3>

* Проверьте все hooks `PreToolUse` на возвращение `permissionDecision: 'deny'`
* Добавьте логирование в ваши hooks, чтобы увидеть, какие `permissionDecisionReason` они возвращают
* Проверьте, что паттерны matcher не слишком широкие: пустой matcher соответствует всем инструментам

<h3 id="modified-input-not-applied">
  Измененный входной сигнал не применяется
</h3>

* Убедитесь, что `updatedInput` находится внутри `hookSpecificOutput`, а не на верхнем уровне:

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* Верните `permissionDecision: 'allow'` для автоматического одобрения измененного входного сигнала или `'ask'` для отображения его пользователю на утверждение

* Включите `hookEventName` в `hookSpecificOutput` для идентификации типа hook, для которого предназначен вывод

<h3 id="session-hooks-not-available-in-python">
  Hooks сеанса недоступны в Python
</h3>

`SessionStart` и `SessionEnd` могут быть зарегистрированы как SDK callback hooks в TypeScript, но недоступны в Python SDK, потому что его тип `HookEvent` их опускает. В Python они доступны только как [shell command hooks](/docs/ru/hooks#hook-events), определенные в файлах настроек, таких как `.claude/settings.json`. Для загрузки shell command hooks из вашего приложения SDK включите соответствующий источник настроек с [`setting_sources`](/docs/ru/agent-sdk/python#settingsource) или [`settingSources`](/docs/ru/agent-sdk/typescript#settingsource):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Для запуска логики инициализации как Python SDK callback вместо этого используйте первое сообщение из `client.receive_response()` как ваш триггер.

<h3 id="subagent-permission-prompts-multiplying">
  Запросы разрешений подагента умножаются
</h3>

При порождении нескольких подагентов каждый может запросить разрешения отдельно. Подагенты не автоматически наследуют разрешения родительского агента. Чтобы избежать повторных запросов, используйте hooks `PreToolUse` для автоматического одобрения конкретных инструментов или настройте правила разрешений, которые применяются к сеансам подагента.

<h3 id="recursive-hook-loops-with-subagents">
  Рекурсивные циклы hook с подагентами
</h3>

Hook `UserPromptSubmit`, который порождает подагентов, может создать бесконечные циклы, если эти подагенты срабатывают тот же hook. Чтобы предотвратить это:

* Проверьте индикатор подагента во входных данных hook перед порождением
* Используйте общую переменную или состояние сеанса для отслеживания, находитесь ли вы уже внутри подагента
* Ограничьте область действия hooks, чтобы они запускались только для сеанса агента верхнего уровня

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage не появляется в выводе
</h3>

Поле `systemMessage` показывает сообщение пользователю, а не модели. По умолчанию SDK выводит выходные данные hook в поток сообщений только для hooks `SessionStart` и `Setup`, поэтому сообщение из любого другого события hook не появляется, если вы не установите `includeHookEvents` (`include_hook_events` в Python). Для передачи контекста модели вместо этого верните [`additionalContext`](/docs/ru/hooks#add-context-for-claude).

Если вам нужно надежно вывести решения hook в ваше приложение, логируйте их отдельно или используйте выделенный канал вывода.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Справочник hooks Claude Code](/docs/ru/hooks): полные схемы входных/выходных данных JSON, документация событий и паттерны matcher
* [Руководство hooks Claude Code](/docs/ru/hooks-guide): примеры hooks команд shell и пошаговые инструкции
* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript): типы hooks, определения входных/выходных данных и параметры конфигурации
* [Справочник Python SDK](/docs/ru/agent-sdk/python): типы hooks, определения входных/выходных данных и параметры конфигурации
* [Разрешения](/docs/ru/agent-sdk/permissions): контролируйте, что может делать ваш агент
* [Пользовательские инструменты](/docs/ru/agent-sdk/custom-tools): создавайте инструменты для расширения возможностей агента
