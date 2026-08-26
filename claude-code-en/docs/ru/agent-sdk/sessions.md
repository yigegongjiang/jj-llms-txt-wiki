> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Работа с сеансами

> Как сеансы сохраняют историю разговора агента, и когда использовать continue, resume и fork для возврата к предыдущему запуску.

Сеанс — это история разговора, которую SDK накапливает во время работы вашего агента. Она содержит ваш запрос, каждый вызов инструмента, который сделал агент, каждый результат инструмента и каждый ответ. SDK автоматически записывает его на диск, чтобы вы могли вернуться к нему позже.

Возврат к сеансу означает, что агент имеет полный контекст с предыдущего момента: файлы, которые он уже прочитал, анализ, который он уже выполнил, решения, которые он уже принял. Вы можете задать дополнительный вопрос, восстановиться после прерывания или перейти к другому подходу.

<Note>
  Сеансы сохраняют **разговор**, а не файловую систему. Чтобы создать снимок и отменить изменения файлов, которые сделал агент, используйте [file checkpointing](/docs/ru/agent-sdk/file-checkpointing).
</Note>

Это руководство охватывает, как выбрать правильный подход для вашего приложения, интерфейсы SDK, которые автоматически отслеживают сеансы, как захватить ID сеанса и использовать `resume` и `fork` вручную, и что нужно знать о возобновлении сеансов на разных хостах.

<h2 id="choose-an-approach">
  Выберите подход
</h2>

Объем управления сеансами, который вам нужен, зависит от структуры вашего приложения. Управление сеансами вступает в игру, когда вы отправляете несколько запросов, которые должны совместно использовать контекст. В рамках одного вызова `query()` агент уже делает столько ходов, сколько ему нужно, и запросы разрешения и `AskUserQuestion` [обрабатываются в цикле](/docs/ru/agent-sdk/user-input) (они не завершают вызов).

| Что вы создаёте                                                           | Что использовать                                                                                                                                           |
| :------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Одноразовая задача: один запрос, без дополнительных вопросов              | Ничего дополнительного. Один вызов `query()` справляется с этим.                                                                                           |
| Многоходовой чат в одном процессе                                         | [`ClaudeSDKClient` (Python) или `continue: true` (TypeScript)](#automatic-session-management). SDK отслеживает сеанс для вас без обработки ID.             |
| Продолжить с того же места после перезагрузки процесса                    | `continue_conversation=True` (Python) / `continue: true` (TypeScript). Возобновляет самый последний сеанс в каталоге, ID не требуется.                     |
| Возобновить конкретный прошлый сеанс (не самый последний)                 | Захватите ID сеанса и передайте его в `resume`.                                                                                                            |
| Попробовать альтернативный подход без потери оригинала                    | Разветвите сеанс.                                                                                                                                          |
| Задача без состояния, не хотите ничего писать на диск (только TypeScript) | Установите [`persistSession: false`](/docs/ru/agent-sdk/typescript#options). Сеанс существует только в памяти на время вызова. Python всегда сохраняет на диск. |

<h3 id="continue-resume-and-fork">
  Continue, resume и fork
</h3>

Continue, resume и fork — это поля опций, которые вы устанавливаете на `query()` ([`ClaudeAgentOptions`](/docs/ru/agent-sdk/python#claudeagentoptions) в Python, [`Options`](/docs/ru/agent-sdk/typescript#options) в TypeScript).

**Continue** и **resume** оба подхватывают существующий сеанс и добавляют к нему. Разница в том, как они находят этот сеанс:

* **Continue** находит самый последний сеанс в текущем каталоге. Вы ничего не отслеживаете. Хорошо работает, когда ваше приложение запускает один разговор за раз.
* **Resume** принимает конкретный ID сеанса. Вы отслеживаете ID. Требуется, когда у вас есть несколько сеансов (например, один на пользователя в многопользовательском приложении) или вы хотите вернуться к тому, который не является самым последним.

**Fork** отличается: он создаёт новый сеанс, который начинается с копии истории оригинала. Оригинал остаётся неизменным. Используйте fork, чтобы попробовать другое направление, сохраняя возможность вернуться назад.

<h2 id="automatic-session-management">
  Автоматическое управление сеансами
</h2>

Оба SDK предлагают интерфейс, который отслеживает состояние сеанса для вас между вызовами, поэтому вам не нужно вручную передавать ID. Используйте их для многоходовых разговоров в рамках одного процесса.

<h3 id="python-claudesdkclient">
  Python: `ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/ru/agent-sdk/python#claudesdkclient) обрабатывает ID сеансов внутри. Каждый вызов `client.query()` автоматически продолжает тот же сеанс. Вызовите [`client.receive_response()`](/docs/ru/agent-sdk/python#claudesdkclient) для итерации по сообщениям для текущего запроса. Используйте клиент как асинхронный контекстный менеджер, чтобы установка и разрыв соединения обрабатывались автоматически, или вызовите `connect()` и `disconnect()` вручную.

Этот пример запускает два запроса к одному и тому же `client`. Первый просит агента проанализировать модуль; второй просит его переделать этот модуль. Поскольку оба вызова проходят через один и тот же экземпляр клиента, второй запрос имеет полный контекст из первого без явного `resume` или ID сеанса:

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

Смотрите [справку Python SDK](/docs/ru/agent-sdk/python#choosing-between-query-and-claudesdkclient) для деталей о том, когда использовать `ClaudeSDKClient` в сравнении с автономной функцией `query()`.

<h3 id="typescript-continue-true">
  TypeScript: `continue: true`
</h3>

TypeScript SDK не имеет объекта клиента, удерживающего сеанс, как Python's `ClaudeSDKClient`. Вместо этого передайте `continue: true` на каждом последующем вызове `query()` и SDK подхватит самый последний сеанс в текущем каталоге. Отслеживание ID не требуется.

Этот пример делает два отдельных вызова `query()`. Первый создаёт свежий сеанс; второй устанавливает `continue: true`, что говорит SDK найти и возобновить самый последний сеанс на диске. Агент имеет полный контекст из первого вызова:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
for await (const message of query({
  prompt: "Analyze the auth module",
  options: { allowedTools: ["Read", "Glob", "Grep"] }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  Экспериментальный [V2 session API](/docs/ru/agent-sdk/typescript-v2-preview), который предоставлял `createSession()` с паттерном `send` / `stream`, был удален в TypeScript Agent SDK 0.3.142. Используйте функцию `query()` и параметры сеанса, описанные на этой странице.
</Note>

<h2 id="use-session-options-with-query">
  Используйте параметры сеанса с `query()`
</h2>

<h3 id="capture-the-session-id">
  Захватите ID сеанса
</h3>

Resume и fork требуют ID сеанса. Прочитайте его из поля `session_id` на сообщении результата ([`ResultMessage`](/docs/ru/agent-sdk/python#resultmessage) в Python, [`SDKResultMessage`](/docs/ru/agent-sdk/typescript#sdkresultmessage) в TypeScript), которое присутствует на каждом результате независимо от успеха или ошибки. В TypeScript ID также доступен раньше как прямое поле на инициализирующем `SystemMessage`; в Python он вложен внутри `SystemMessage.data`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      async for message in query(
          prompt="Analyze the auth module and suggest improvements",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id
              if message.subtype == "success":
                  print(message.result)

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  for await (const message of query({
    prompt: "Analyze the auth module and suggest improvements",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
      if (message.subtype === "success") {
        console.log(message.result);
      }
    }
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

<h3 id="resume-by-id">
  Возобновите по ID
</h3>

Передайте ID сеанса в `resume`, чтобы вернуться к этому конкретному сеансу. Агент подхватывает с полным контекстом с того момента, где сеанс остановился. Распространённые причины для возобновления:

* **Продолжить завершённую задачу.** Агент уже что-то проанализировал; теперь вы хотите, чтобы он действовал на основе этого анализа без повторного чтения файлов.
* **Восстановиться от лимита.** Первый запуск закончился с `error_max_turns` или `error_max_budget_usd` (смотрите [Handle the result](/docs/ru/agent-sdk/agent-loop#handle-the-result)); возобновите с более высоким лимитом.
* **Перезагрузить ваш процесс.** Вы захватили ID перед выключением и хотите восстановить разговор.

Этот пример возобновляет сеанс из [Захватите ID сеанса](#capture-the-session-id) с дополнительным запросом. Поскольку вы возобновляете, агент уже имеет предыдущий анализ в контексте:

<CodeGroup>
  ```python Python theme={null}
  # Earlier session analyzed the code; now build on that analysis
  async for message in query(
      prompt="Now implement the refactoring you suggested",
      options=ClaudeAgentOptions(
          resume=session_id,
          allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Вы должны увидеть ответ, который основывается на предыдущем анализе вместо того, чтобы начинать с нуля. Это подтверждает, что агент возобновил сеанс с его предыдущим контекстом нетронутым.

<Tip>
  Если вызов `resume` возвращает свежий сеанс вместо ожидаемой истории, наиболее распространённая причина — несовпадающий `cwd`. Сеансы хранятся под `~/.claude/projects/<encoded-cwd>/*.jsonl`, или под `$CLAUDE_CONFIG_DIR/projects/<encoded-cwd>/*.jsonl`, если вы установили переменную окружения `CLAUDE_CONFIG_DIR`, где `<encoded-cwd>` — это абсолютный рабочий каталог со всеми буквенно-цифровыми символами, заменённые на `-` (так `/Users/me/proj` становится `-Users-me-proj`). Если ваш вызов resume запускается из другого каталога, SDK ищет в неправильном месте. Файл сеанса также должен существовать на текущей машине.
</Tip>

Чтобы возобновить сеансы на разных машинах или в бессерверных окружениях, зеркалируйте стенограммы в общее хранилище с адаптером [`SessionStore`](/docs/ru/agent-sdk/session-storage).

<h3 id="fork-to-explore-alternatives">
  Разветвите для изучения альтернатив
</h3>

Разветвление создаёт новый сеанс, который начинается с копии истории оригинала, но расходится с этого момента. Разветвление получает свой собственный ID сеанса; ID оригинала и история остаются неизменными. Вы получаете два независимых сеанса, которые можете возобновить отдельно.

<Note>
  Разветвление ветвит историю разговора, а не файловую систему. Если разветвлённый агент редактирует файлы, эти изменения реальны и видны любому сеансу, работающему в том же каталоге. Чтобы разветвить и отменить изменения файлов, используйте [file checkpointing](/docs/ru/agent-sdk/file-checkpointing).
</Note>

Этот пример основывается на [Захватите ID сеанса](#capture-the-session-id): вы уже проанализировали модуль аутентификации в `session_id` и хотите изучить OAuth2 без потери потока, сосредоточенного на JWT. Первый блок разветвляет сеанс и захватывает ID разветвления (`forked_id`); второй блок возобновляет оригинальный `session_id` для продолжения по пути JWT. Теперь у вас есть два ID сеанса, указывающих на две отдельные истории:

<CodeGroup>
  ```python Python theme={null}
  # Fork: branch from session_id into a new session
  forked_id = None
  async for message in query(
      prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
      options=ClaudeAgentOptions(
          resume=session_id,
          fork_session=True,
          max_turns=5,
      ),
  ):
      if isinstance(message, ResultMessage):
          forked_id = message.session_id  # The fork's ID, distinct from session_id
          if message.subtype == "success":
              print(message.result)

  print(f"Forked session: {forked_id}")

  # Original session is untouched; resuming it continues the JWT thread
  async for message in query(
      prompt="Continue with the JWT approach",
      options=ClaudeAgentOptions(resume=session_id),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  for await (const message of query({
    prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
    options: {
      resume: sessionId,
      forkSession: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      forkedId = message.session_id; // The fork's ID, distinct from sessionId
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  for await (const message of query({
    prompt: "Continue with the JWT approach",
    options: { resume: sessionId }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Вы должны увидеть, что `forkedId` отличается от оригинального ID сеанса. Возобновление оригинального сеанса по-прежнему продолжает поток JWT, что подтверждает, что разветвление не изменило оригинальную историю.

<h2 id="resume-across-hosts">
  Возобновите на разных хостах
</h2>

Файлы сеансов локальны для машины, которая их создала. Чтобы возобновить сеанс на другом хосте (рабочие CI, эфемерные контейнеры, бессерверные), у вас есть два варианта:

* **Переместите файл сеанса.** Сохраните `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` из первого запуска и восстановите его по тому же пути на новом хосте перед вызовом `resume`. `cwd` должен совпадать.
* **Не полагайтесь на возобновление сеанса.** Захватите результаты, которые вам нужны (вывод анализа, решения, различия файлов) как состояние приложения и передайте их в запрос свежего сеанса. Это часто более надёжно, чем отправка файлов стенограмм.

Оба SDK предоставляют функции для перечисления сеансов на диске и чтения их сообщений: [`listSessions()`](/docs/ru/agent-sdk/typescript#listsessions) и [`getSessionMessages()`](/docs/ru/agent-sdk/typescript#getsessionmessages) в TypeScript, [`list_sessions()`](/docs/ru/agent-sdk/python#list_sessions) и [`get_session_messages()`](/docs/ru/agent-sdk/python#get_session_messages) в Python. Используйте их для создания пользовательских средств выбора сеансов, логики очистки или средств просмотра стенограмм.

Оба SDK также предоставляют функции для поиска и изменения отдельных сеансов: [`get_session_info()`](/docs/ru/agent-sdk/python#get_session_info), [`rename_session()`](/docs/ru/agent-sdk/python#rename_session) и [`tag_session()`](/docs/ru/agent-sdk/python#tag_session) в Python, и [`getSessionInfo()`](/docs/ru/agent-sdk/typescript#getsessioninfo), [`renameSession()`](/docs/ru/agent-sdk/typescript#renamesession) и [`tagSession()`](/docs/ru/agent-sdk/typescript#tagsession) в TypeScript. Используйте их для организации сеансов по тегам или присвоения им удобочитаемых названий.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Как работает цикл агента](/docs/ru/agent-sdk/agent-loop): Поймите ходы, сообщения и накопление контекста в рамках сеанса
* [File checkpointing](/docs/ru/agent-sdk/file-checkpointing): Создавайте снимки состояния и отменяйте изменения файлов, которые агент внес в рамках сеанса
* [Python `ClaudeAgentOptions`](/docs/ru/agent-sdk/python#claudeagentoptions): Полная справка параметров сеанса для Python
* [TypeScript `Options`](/docs/ru/agent-sdk/typescript#options): Полная справка параметров сеанса для TypeScript
