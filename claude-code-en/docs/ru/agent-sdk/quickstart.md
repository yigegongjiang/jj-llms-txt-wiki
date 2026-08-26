> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Быстрый старт

> Начните работу с Python или TypeScript Agent SDK для создания AI-агентов, которые работают автономно

Используйте Agent SDK для создания AI-агента, который читает ваш код, находит ошибки и исправляет их, всё без ручного вмешательства.

**Что вы будете делать:**

1. Настроить проект с Agent SDK
2. Создать файл с некорректным кодом
3. Запустить агента, который автоматически находит и исправляет ошибки

<h2 id="prerequisites">
  Предварительные требования
</h2>

* **Node.js 18+** или **Python 3.10+**
* **Учётная запись Anthropic** ([зарегистрируйтесь здесь](https://platform.claude.com/))

<h2 id="setup">
  Настройка
</h2>

<Steps>
  <Step title="Создайте папку проекта">
    Создайте новый каталог для этого быстрого старта:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Для собственных проектов вы можете запустить SDK из любой папки; по умолчанию он будет иметь доступ к файлам в этом каталоге и его подкаталогах.
  </Step>

  <Step title="Установите SDK">
    Установите пакет Agent SDK для вашего языка:

    <Tabs>
      <Tab title="TypeScript (новый проект)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Установка `"type": "module"` в `package.json` позволяет вашему скрипту агента использовать верхнеуровневый `await`, а [tsx](https://tsx.is) запускает файлы TypeScript напрямую.
      </Tab>

      <Tab title="TypeScript (существующий проект)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) запускает файлы TypeScript напрямую. Если ваш проект использует CommonJS, назовите ваш скрипт агента `agent.mts` вместо `agent.ts`. Расширение `.mts` заставляет tsx рассматривать файл как ES модуль, поэтому верхнеуровневый `await` работает без преобразования всего вашего проекта в ES модули. Используйте `agent.mts` вместо `agent.ts` на этапах создания и запуска далее в этом быстром старте.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) — это быстрый менеджер пакетов Python, который автоматически управляет виртуальными окружениями:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Создайте и активируйте виртуальное окружение, затем установите пакет.

        На macOS или Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        На Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Если PowerShell блокирует `Activate.ps1` с ошибкой политики выполнения, сначала выполните `Set-ExecutionPolicy -Scope Process RemoteSigned`.
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK включает нативный бинарный файл Claude Code для вашей платформы в качестве опциональной зависимости, поэтому вам не нужно устанавливать Claude Code отдельно.
    </Note>
  </Step>

  <Step title="Установите ваш API ключ">
    Получите API ключ из [Claude Console](https://platform.claude.com/), затем установите его как переменную окружения в оболочке, где вы будете запускать вашего агента:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    SDK читает ключ из окружения процесса, который запускает вашего агента; он не загружает файлы `.env` автоматически. Если вы храните ключ в файле `.env`, загрузите его самостоятельно, например с помощью пакета `dotenv`, перед вызовом SDK.

    SDK также поддерживает аутентификацию через сторонних поставщиков API:

    * **Amazon Bedrock**: установите переменную окружения `CLAUDE_CODE_USE_BEDROCK=1` и настройте учётные данные AWS
    * **Claude Platform on AWS**: установите `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` и `ANTHROPIC_AWS_WORKSPACE_ID`, затем настройте учётные данные AWS
    * **Google Cloud's Agent Platform**: установите переменную окружения `CLAUDE_CODE_USE_VERTEX=1` и настройте учётные данные Google Cloud
    * **Microsoft Azure**: установите переменную окружения `CLAUDE_CODE_USE_FOUNDRY=1` и настройте учётные данные Azure

    Подробности см. в руководствах по настройке для [Amazon Bedrock](/docs/ru/amazon-bedrock), [Claude Platform on AWS](/docs/ru/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), или [Microsoft Foundry](/docs/ru/microsoft-foundry).

    <Note>
      Если не было предварительного одобрения, Anthropic не разрешает сторонним разработчикам предлагать вход через claude.ai или ограничения скорости для своих продуктов, включая агентов, созданных на основе Claude Agent SDK. Вместо этого используйте методы аутентификации через API ключ, описанные в этом документе.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Создайте файл с ошибками
</h2>

Этот быстрый старт проведёт вас через создание агента, который может находить и исправлять ошибки в коде. Сначала вам нужен файл с некоторыми намеренными ошибками для исправления агентом. Создайте `utils.py` в каталоге `my-agent` и вставьте следующий код:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Этот код содержит две ошибки:

1. `calculate_average([])` падает с ошибкой деления на ноль
2. `get_user_name(None)` падает с TypeError

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Создайте агента, который находит и исправляет ошибки
</h2>

Создайте `agent.py`, если вы используете Python SDK, или `agent.ts` для TypeScript. Используйте `agent.mts` вместо этого, если ваш существующий проект использует CommonJS:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Этот код состоит из трёх основных частей:

1. **`query`**: основная точка входа, которая создаёт цикл агента. Она возвращает асинхронный итератор, поэтому вы используете `async for` для потоковой передачи сообщений по мере работы Claude. Полный API см. в справочнике [Python](/docs/ru/agent-sdk/python#query) или [TypeScript](/docs/ru/agent-sdk/typescript#query) SDK.

2. **`prompt`**: то, что вы хотите, чтобы сделал Claude. Claude определяет, какие инструменты использовать, на основе задачи.

3. **`options`**: конфигурация для агента. В этом примере используется `allowedTools` для предварительного одобрения `Read`, `Edit` и `Glob`, а также `permissionMode: "acceptEdits"` для автоматического одобрения изменений файлов. Другие опции включают `systemPrompt`, `mcpServers` и многое другое. Все опции для [Python](/docs/ru/agent-sdk/python#claudeagentoptions) или [TypeScript](/docs/ru/agent-sdk/typescript#options).

Цикл `async for` продолжает работать, пока Claude думает, вызывает инструменты, наблюдает результаты и решает, что делать дальше. Каждая итерация выдаёт сообщение: рассуждение Claude, вызов инструмента, результат инструмента или окончательный результат. SDK обрабатывает оркестровку (выполнение инструментов, управление контекстом, повторные попытки), поэтому вы просто потребляете поток. Цикл заканчивается, когда Claude завершает задачу или возникает ошибка.

Обработка сообщений внутри цикла фильтрует удобочитаемый вывод. Без фильтрации вы увидите необработанные объекты сообщений, включая инициализацию системы и внутреннее состояние, что полезно для отладки, но в остальном шумно.

<Note>
  В этом примере используется потоковая передача для отображения прогресса в реальном времени. Если вам не нужен живой вывод (например, для фоновых заданий или конвейеров CI), вы можете собрать все сообщения сразу. Подробности см. в разделе [Потоковая передача и однооборотный режим](/docs/ru/agent-sdk/streaming-vs-single-mode).
</Note>

<h3 id="run-your-agent">
  Запустите вашего агента
</h3>

Ваш агент готов. Запустите его с помощью следующей команды:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Если вы назвали ваш скрипт `agent.mts`, запустите `npx tsx agent.mts` вместо этого.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    С активированной виртуальной средой:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

По мере работы агент выводит своё рассуждение и каждый инструмент, который он вызывает, заканчивая с `Done: success`. После запуска проверьте `utils.py`. Вы увидите защитный код, обрабатывающий пустые списки и нулевых пользователей. Ваш агент автономно:

1. **Прочитал** `utils.py` для понимания кода
2. **Проанализировал** логику и определил граничные случаи, которые вызовут сбой
3. **Отредактировал** файл для добавления надлежащей обработки ошибок

Это то, что отличает Agent SDK: Claude выполняет инструменты напрямую вместо того, чтобы просить вас их реализовать.

<Note>
  Если вы видите "API key not found", убедитесь, что вы установили переменную окружения `ANTHROPIC_API_KEY` в оболочке, где вы запускаете вашего агента. SDK не загружает файлы `.env` автоматически. Подробнее см. в [полном руководстве по устранению неполадок](/docs/ru/troubleshooting).
</Note>

<h3 id="try-other-prompts">
  Попробуйте другие подсказки
</h3>

Теперь, когда ваш агент настроен, попробуйте некоторые другие подсказки:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Настройте вашего агента
</h3>

Вы можете изменить поведение вашего агента, изменив опции. Вот несколько примеров:

**Добавьте возможность веб-поиска:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Дайте Claude пользовательскую системную подсказку:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Запускайте команды в терминале:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

С включённым `Bash` попробуйте: `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  Ключевые концепции
</h2>

**Инструменты** контролируют, что может делать ваш агент:

| Инструменты                            | Что может делать агент   |
| -------------------------------------- | ------------------------ |
| `Read`, `Glob`, `Grep`                 | Анализ только для чтения |
| `Read`, `Edit`, `Glob`                 | Анализ и изменение кода  |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Полная автоматизация     |

**Режимы разрешений** контролируют, сколько человеческого надзора вы хотите:

| Режим               | Поведение                                                                                                                                                                                                                                                                                                                                  | Вариант использования                                 |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------- |
| `acceptEdits`       | Автоматически одобряет редактирование файлов и общие команды файловой системы, запрашивает другие действия                                                                                                                                                                                                                                 | Надёжные рабочие процессы разработки                  |
| `plan`              | Запускает инструменты только для чтения; редактирование файлов никогда не одобряется автоматически и достигает вашего обратного вызова `canUseTool`                                                                                                                                                                                        | Определение объёма задачи перед одобрением выполнения |
| `dontAsk`           | Отклоняет всё, что не в `allowedTools`; инструменты соединителя [ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и инструменты, требующие взаимодействия с пользователем, отклоняются даже если вы их указали                                                                                      | Заблокированные автономные агенты                     |
| `auto`              | Классификатор модели одобряет или отклоняет каждый вызов инструмента                                                                                                                                                                                                                                                                       | Автономные агенты с защитой безопасности              |
| `bypassPermissions` | Запускает каждый инструмент без подсказок, кроме инструментов, соответствующих явному правилу [`ask`](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated), инструментов соединителя [ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и инструментов, требующих взаимодействия с пользователем | Изолированный CI, полностью доверенные окружения      |
| `default`           | Требует обратного вызова `canUseTool` для обработки одобрения                                                                                                                                                                                                                                                                              | Пользовательские потоки одобрения                     |

Приведённый выше пример использует режим `acceptEdits`, который автоматически одобряет файловые операции, чтобы агент мог работать без интерактивных подсказок. Если вы хотите запрашивать у пользователей одобрение, используйте режим `default` и предоставьте обратный вызов [`canUseTool`](/docs/ru/agent-sdk/user-input), который собирает пользовательский ввод. Для большего контроля см. [Разрешения](/docs/ru/agent-sdk/permissions).

<h2 id="next-steps">
  Следующие шаги
</h2>

Теперь, когда вы создали своего первого агента, узнайте, как расширить его возможности и адаптировать его к вашему варианту использования:

* **[Разрешения](/docs/ru/agent-sdk/permissions)**: контролируйте, что может делать ваш агент и когда ему нужно одобрение
* **[Hooks](/docs/ru/agent-sdk/hooks)**: запускайте пользовательский код до или после вызовов инструментов
* **[Сессии](/docs/ru/agent-sdk/sessions)**: создавайте многооборотных агентов, которые сохраняют контекст
* **[MCP servers](/docs/ru/agent-sdk/mcp)**: подключайтесь к базам данных, браузерам, API и другим внешним системам
* **[Хостинг](/docs/ru/agent-sdk/hosting)**: развёртывайте агентов в Docker, облако и CI/CD
* **[Примеры агентов](https://github.com/anthropics/claude-agent-sdk-demos)**: см. полные примеры: помощник по электронной почте, исследовательский агент и многое другое
