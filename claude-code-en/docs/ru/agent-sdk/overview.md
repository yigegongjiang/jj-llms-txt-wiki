> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Обзор Agent SDK

> Создавайте производственные AI-агентов с Claude Code как библиотеку

Создавайте AI-агентов, которые автономно читают файлы, запускают команды, ищут в интернете, редактируют код и многое другое. Agent SDK предоставляет вам те же инструменты, цикл агента и управление контекстом, которые питают Claude Code, программируемые на Python и TypeScript. Для получения информации о логике проектирования агентского каркаса см. [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) в блоге.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

Agent SDK включает встроенные инструменты для чтения файлов, запуска команд и редактирования кода, поэтому ваш агент может начать работу немедленно без необходимости реализации выполнения инструментов. Погрузитесь в быстрый старт или изучите реальных агентов, созданных с помощью SDK:

<CardGroup cols={2}>
  <Card title="Быстрый старт" icon="play" href="/docs/ru/agent-sdk/quickstart">
    Создайте агента по исправлению ошибок за несколько минут
  </Card>

  <Card title="Примеры агентов" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Помощник по электронной почте, исследовательский агент и многое другое
  </Card>
</CardGroup>

<h2 id="get-started">
  Начало работы
</h2>

<Steps>
  <Step title="Установите SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) — это быстрый менеджер пакетов Python, который автоматически обрабатывает виртуальные окружения:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Создайте и активируйте виртуальное окружение, затем установите пакет. Установка в виртуальное окружение избегает ошибки `error: externally-managed-environment`, которую системный Python на недавних установках Debian, Ubuntu и Homebrew возвращает для `pip install` вне venv.

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

        Если PowerShell блокирует `Activate.ps1` с ошибкой политики выполнения, сначала запустите `Set-ExecutionPolicy -Scope Process RemoteSigned`.

        Пакет Python требует Python 3.10 или более поздней версии. Если pip сообщает `No matching distribution found for claude-agent-sdk`, ваш интерпретатор старше 3.10. Запустите `python3 --version` на macOS или Linux или `py --version` на Windows, чтобы проверить.
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK поставляется с собственным бинарным файлом Claude Code для вашей платформы в качестве дополнительной зависимости, поэтому вам не нужно устанавливать Claude Code отдельно.
    </Note>
  </Step>

  <Step title="Установите ваш API ключ">
    Получите API ключ из [Console](https://platform.claude.com/), затем установите его как переменную окружения.

    На macOS или Linux:

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    На Windows PowerShell:

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    SDK также поддерживает аутентификацию через сторонних поставщиков API:

    * **Amazon Bedrock**: установите переменную окружения `CLAUDE_CODE_USE_BEDROCK=1` и настройте учетные данные AWS
    * **Claude Platform on AWS**: установите `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` и `ANTHROPIC_AWS_WORKSPACE_ID`, затем настройте учетные данные AWS
    * **Google Cloud's Agent Platform**: установите переменную окружения `CLAUDE_CODE_USE_VERTEX=1` и настройте учетные данные Google Cloud
    * **Microsoft Azure**: установите переменную окружения `CLAUDE_CODE_USE_FOUNDRY=1` и настройте учетные данные Azure

    См. руководства по настройке для [Amazon Bedrock](/docs/ru/amazon-bedrock), [Claude Platform on AWS](/docs/ru/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) или [Microsoft Foundry](/docs/ru/microsoft-foundry) для получения подробной информации.

    <Note>
      Если не одобрено ранее, Anthropic не разрешает сторонним разработчикам предлагать вход в claude.ai или ограничения скорости для своих продуктов, включая агентов, созданных на Claude Agent SDK. Вместо этого используйте методы аутентификации по API ключу, описанные в этом документе.
    </Note>
  </Step>

  <Step title="Запустите вашего первого агента">
    Этот пример создает агента, который перечисляет файлы в вашем текущем каталоге, используя встроенные инструменты.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**Готовы к разработке?** Следуйте [Быстрому старту](/docs/ru/agent-sdk/quickstart), чтобы создать агента, который находит и исправляет ошибки за несколько минут.

<h2 id="capabilities">
  Возможности
</h2>

Все, что делает Claude Code мощным, доступно в SDK:

<Tabs>
  <Tab title="Встроенные инструменты">
    Ваш агент может читать файлы, запускать команды и искать в кодовых базах из коробки. Ключевые инструменты включают:

    | Инструмент                                                                  | Что он делает                                                                 |
    | --------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
    | **Read**                                                                    | Читать любой файл в рабочем каталоге                                          |
    | **Write**                                                                   | Создавать новые файлы                                                         |
    | **Edit**                                                                    | Делать точные правки в существующих файлах                                    |
    | **Bash**                                                                    | Запускать команды терминала, скрипты, операции git                            |
    | **Monitor**                                                                 | Наблюдать фоновый скрипт и реагировать на каждую строку вывода как на событие |
    | **Glob**                                                                    | Находить файлы по шаблону (`**/*.ts`, `src/**/*.py`)                          |
    | **Grep**                                                                    | Искать содержимое файлов с помощью regex                                      |
    | **WebSearch**                                                               | Искать в интернете текущую информацию                                         |
    | **WebFetch**                                                                | Получать и анализировать содержимое веб-страниц                               |
    | **[AskUserQuestion](/docs/ru/agent-sdk/user-input#handle-clarifying-questions)** | Задавать пользователю уточняющие вопросы с вариантами множественного выбора   |

    Этот пример создает агента, который ищет в вашей кодовой базе комментарии TODO:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="hooks">
    Запускайте пользовательский код в ключевых точках жизненного цикла агента. SDK hooks используют функции обратного вызова для проверки, логирования, блокирования или преобразования поведения агента.

    **Доступные hooks:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit` и другие.

    Этот пример логирует все изменения файлов в файл аудита:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Узнайте больше о hooks →](/docs/ru/agent-sdk/hooks)
  </Tab>

  <Tab title="Subagents">
    Создавайте специализированных агентов для обработки сосредоточенных подзадач. Ваш основной агент делегирует работу, а подагенты сообщают результаты.

    Определите пользовательских агентов со специализированными инструкциями. Подагенты вызываются через инструмент Agent, поэтому включите `Agent` в `allowedTools` для автоматического одобрения этих вызовов:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
                      )
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    Сообщения из контекста подагента включают поле `parent_tool_use_id`, позволяющее отследить, какие сообщения принадлежат какому выполнению подагента.

    [Узнайте больше о subagents →](/docs/ru/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Подключайтесь к внешним системам через Model Context Protocol: базы данных, браузеры, API и [сотни других](https://github.com/modelcontextprotocol/servers).

    Этот пример подключает [Playwright MCP server](https://github.com/microsoft/playwright-mcp), чтобы дать вашему агенту возможности автоматизации браузера:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Узнайте больше о MCP →](/docs/ru/agent-sdk/mcp)
  </Tab>

  <Tab title="Permissions">
    Контролируйте точно, какие инструменты может использовать ваш агент. Разрешите безопасные операции, заблокируйте опасные или требуйте одобрения для чувствительных действий.

    <Note>
      Для интерактивных подсказок одобрения и инструмента `AskUserQuestion`, см. [Обработка одобрений и ввода пользователя](/docs/ru/agent-sdk/user-input).
    </Note>

    Этот пример создает агента только для чтения, который может анализировать, но не изменять код. `allowed_tools` предварительно одобряет `Read`, `Glob` и `Grep`.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Узнайте больше о permissions →](/docs/ru/agent-sdk/permissions)
  </Tab>

  <Tab title="Sessions">
    Сохраняйте контекст между несколькими обменами. Claude помнит прочитанные файлы, выполненный анализ и историю разговора. Возобновляйте сеансы позже или разветвляйте их, чтобы исследовать различные подходы.

    Этот пример захватывает ID сеанса из первого запроса, затем возобновляет работу с полным контекстом:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Узнайте больше о sessions →](/docs/ru/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Функции Claude Code
</h3>

SDK также поддерживает конфигурацию на основе файловой системы Claude Code. С параметрами по умолчанию SDK загружает их из `.claude/` в вашем рабочем каталоге и `~/.claude/`. Чтобы ограничить, какие источники загружаются, установите `setting_sources` (Python) или `settingSources` (TypeScript) в ваших параметрах.

| Функция                                          | Описание                                                                                                   | Местоположение                      |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| [Skills](/docs/ru/agent-sdk/skills)                   | Специализированные возможности, которые Claude использует автоматически или вы вызываете с помощью `/name` | `.claude/skills/*/SKILL.md`         |
| [Commands](/docs/ru/agent-sdk/slash-commands)         | Пользовательские команды в устаревшем формате. Используйте skills для новых пользовательских команд        | `.claude/commands/*.md`             |
| [Memory](/docs/ru/agent-sdk/modifying-system-prompts) | Контекст проекта и инструкции                                                                              | `CLAUDE.md` или `.claude/CLAUDE.md` |
| [Plugins](/docs/ru/agent-sdk/plugins)                 | Расширяйте с помощью skills, агентов, hooks и MCP серверов                                                 | Программно через опцию `plugins`    |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Сравнение Agent SDK с другими инструментами Claude
</h2>

Claude Platform предлагает несколько способов разработки с Claude. Вот как Agent SDK вписывается:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    [Anthropic Client SDK](https://platform.claude.com/docs/ru/api/client-sdks) дает вам прямой доступ к API: вы отправляете подсказки и реализуете выполнение инструментов самостоятельно. **Agent SDK** дает вам Claude со встроенным выполнением инструментов.

    С Client SDK вы реализуете цикл инструментов. С Agent SDK Claude обрабатывает это:

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    Те же возможности, другой интерфейс:

    | Вариант использования          | Лучший выбор |
    | ------------------------------ | ------------ |
    | Интерактивная разработка       | CLI          |
    | CI/CD конвейеры                | SDK          |
    | Пользовательские приложения    | SDK          |
    | Одноразовые задачи             | CLI          |
    | Производственная автоматизация | SDK          |

    Многие команды используют оба: CLI для ежедневной разработки, SDK для производства. Рабочие процессы напрямую переводятся между ними.
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/ru/managed-agents/overview) — это размещенный REST API: Anthropic запускает агента и песочницу, а ваше приложение отправляет события и получает результаты потоком. **Agent SDK** — это библиотека, которая запускает цикл агента внутри вашего собственного процесса.

    |                                  | Agent SDK                                                                                                  | Managed Agents                                                                                                                |
    | -------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
    | **Запускается в**                | Ваш процесс, ваша инфраструктура                                                                           | Инфраструктура, управляемая Anthropic                                                                                         |
    | **Интерфейс**                    | Библиотека Python или TypeScript                                                                           | REST API                                                                                                                      |
    | **Агент работает с**             | Файлами в вашей инфраструктуре                                                                             | Управляемой песочницей на сеанс                                                                                               |
    | **Состояние сеанса**             | JSONL в вашей файловой системе                                                                             | Журнал событий, размещенный в Anthropic                                                                                       |
    | **Пользовательские инструменты** | Функции Python или TypeScript в процессе                                                                   | Claude запускает инструмент; вы выполняете и возвращаете результаты                                                           |
    | **Лучше всего подходит для**     | Локальное прототипирование, агенты, которые работают непосредственно с вашей файловой системой и сервисами | Производственные агенты без необходимости управления песочницей или инфраструктурой сеанса, долгоживущие и асинхронные сеансы |

    Обычный путь — прототипирование с Agent SDK локально, а затем переход на Managed Agents для производства.
  </Tab>
</Tabs>

<h2 id="changelog">
  Журнал изменений
</h2>

Просмотрите полный журнал изменений для обновлений SDK, исправлений ошибок и новых функций:

* **TypeScript SDK**: [просмотреть CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [просмотреть CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Сообщение об ошибках
</h2>

Если вы столкнулись с ошибками или проблемами с Agent SDK:

* **TypeScript SDK**: [сообщить об ошибках на GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [сообщить об ошибках на GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Рекомендации по брендингу
</h2>

Для партнеров, интегрирующих Claude Agent SDK, использование брендинга Claude является необязательным. При ссылке на Claude в вашем продукте:

**Разрешено:**

* "Claude Agent" (предпочтительно для раскрывающихся меню)
* "Claude" (когда находится в меню, уже помеченном как "Agents")
* "{YourAgentName} Powered by Claude" (если у вас есть существующее имя агента)

**Не разрешено:**

* "Claude Code" или "Claude Code Agent"
* ASCII-арт с брендингом Claude Code или визуальные элементы, которые имитируют Claude Code

Ваш продукт должен сохранять свой собственный брендинг и не должен выглядеть как Claude Code или любой продукт Anthropic. Для вопросов о соответствии брендингу свяжитесь с командой Anthropic [sales team](https://www.anthropic.com/contact-sales).

<h2 id="license-and-terms">
  Лицензия и условия
</h2>

Использование Claude Agent SDK регулируется [Коммерческими условиями обслуживания Anthropic](https://www.anthropic.com/legal/commercial-terms), включая случаи, когда вы используете его для питания продуктов и услуг, которые вы предоставляете своим собственным клиентам и конечным пользователям, за исключением случаев, когда конкретный компонент или зависимость покрыты другой лицензией, как указано в файле LICENSE этого компонента.

<h2 id="next-steps">
  Следующие шаги
</h2>

<CardGroup cols={2}>
  <Card title="Быстрый старт" icon="play" href="/docs/ru/agent-sdk/quickstart">
    Создайте агента, который находит и исправляет ошибки за несколько минут
  </Card>

  <Card title="Примеры агентов" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Помощник по электронной почте, исследовательский агент и многое другое
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/ru/agent-sdk/typescript">
    Полная справка API TypeScript и примеры
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/ru/agent-sdk/python">
    Полная справка API Python и примеры
  </Card>
</CardGroup>
