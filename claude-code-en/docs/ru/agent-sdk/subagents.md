> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagents в SDK

> Определяйте и вызывайте subagents для изоляции контекста, параллельного выполнения задач и применения специализированных инструкций в приложениях Claude Agent SDK.

Subagents — это отдельные экземпляры агентов, которые ваш основной агент может создавать для обработки сосредоточенных подзадач.
Используйте их для изоляции контекста, параллельного выполнения нескольких анализов и применения специализированных инструкций без добавления к подсказке основного агента.

<h2 id="overview">
  Обзор
</h2>

Вы можете создавать подагентов тремя способами:

* **Программно**: используйте параметр `agents` в параметрах вашего `query()`. См. справочники [TypeScript](/docs/ru/agent-sdk/typescript#agentdefinition) и [Python](/docs/ru/agent-sdk/python#agentdefinition)
* **На основе файловой системы**: определите агентов как файлы markdown в директориях `.claude/agents/`. См. [определение подагентов как файлов](/docs/ru/sub-agents)
* **Встроенный универсальный**: Claude может вызывать встроенного подагента `general-purpose` в любое время через инструмент Agent без необходимости что-либо определять

Это руководство сосредоточено на программном подходе, который рекомендуется для приложений SDK.

<h2 id="benefits-of-using-subagents">
  Преимущества использования подагентов
</h2>

Поскольку подагенты являются отдельными экземплярами агентов, делегирование работы им дает вам четыре преимущества:

* **Изоляция контекста**: каждый подагент работает в своем собственном разговоре, который начинается с чистого листа, если только подагент не является [форком](/docs/ru/sub-agents#fork-the-current-conversation). В любом случае промежуточные вызовы инструментов и результаты остаются внутри подагента; только его финальное сообщение возвращается к родительскому агенту. Подагент `research-assistant` может исследовать десятки файлов без накопления этого содержимого в основном разговоре. Родительский агент получает краткое резюме, а не каждый файл, который прочитал подагент. Подробнее см. в разделе [What subagents inherit](#what-subagents-inherit) о том, что именно находится в контексте подагента.
* **Параллелизация**: несколько подагентов могут работать одновременно, поэтому независимые подзадачи завершаются за время самой медленной из них, а не за сумму всех времен. Во время проверки кода вы можете запустить подагентов `style-checker`, `security-scanner` и `test-coverage` одновременно вместо последовательного выполнения.
* **Специализированные инструкции и знания**: каждый подагент может иметь адаптированный системный запрос с конкретной экспертизой, лучшими практиками и ограничениями. Подагент `database-migration` может иметь подробные знания о лучших практиках SQL, стратегиях отката и проверках целостности данных, которые были бы ненужным шумом в инструкциях основного агента.
* **Ограничения инструментов**: подагентам можно ограничить доступ к определенным инструментам, снижая риск непредвиденных действий. Подагент `doc-reviewer` может иметь доступ только к инструментам Read и Grep, обеспечивая возможность анализа, но никогда случайно не изменяя ваши файлы документации.

<h2 id="create-subagents">
  Создание подагентов
</h2>

<h3 id="programmatic-definition-recommended">
  Программное определение (рекомендуется)
</h3>

Определите подагентов непосредственно в вашем коде, используя параметр `agents`. Claude вызывает подагентов через инструмент `Agent`.

Большинство примеров на этой странице выводят только окончательный результат. Чтобы подтвердить, что Claude делегировал работу подагенту, а не ответил напрямую, см. [Обнаружение вызова подагента](#detect-subagent-invocation).

Этот пример создает двух подагентов: рецензента кода с доступом только для чтения и средство запуска тестов, которое может выполнять команды.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
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
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  Конфигурация AgentDefinition
</h3>

| Поле              | Тип                                                         | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------- | :---------------------------------------------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Да          | Описание на естественном языке того, когда использовать этого агента                                                                                                                                                                                                                                                                                                          |
| `prompt`          | `string`                                                    | Да          | Системный запрос агента, определяющий его роль и поведение                                                                                                                                                                                                                                                                                                                    |
| `tools`           | `string[]`                                                  | Нет         | Массив разрешенных имен инструментов. Если опущено, наследует каждый [инструмент, доступный подагентам](/docs/ru/sub-agents#available-tools)                                                                                                                                                                                                                                       |
| `disallowedTools` | `string[]`                                                  | Нет         | Массив имен инструментов для удаления из набора инструментов агента. Также принимаются шаблоны уровня сервера MCP: `mcp__server` или `mcp__server__*` удаляет каждый инструмент с этого сервера, а `mcp__*` удаляет каждый инструмент MCP с любого сервера                                                                                                                    |
| `model`           | `string`                                                    | Нет         | Переопределение модели для этого агента. Принимает псевдоним, такой как `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, или полный ID модели. `'inherit'` использует основную модель. Если вы его опустите, Claude Code выбирает модель в [порядке моделей подагентов](/docs/ru/sub-agents#choose-a-model)                                                                |
| `skills`          | `string[]`                                                  | Нет         | Список имен навыков для предварительной загрузки в контекст агента при запуске. Неуказанные навыки остаются вызываемыми через инструмент Skill                                                                                                                                                                                                                                |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Нет         | Источник памяти для этого агента                                                                                                                                                                                                                                                                                                                                              |
| `mcpServers`      | `(string \| object)[]`                                      | Нет         | Серверы MCP, доступные этому агенту, по имени или встроенной конфигурации                                                                                                                                                                                                                                                                                                     |
| `initialPrompt`   | `string`                                                    | Нет         | Автоматически отправляется как первый ход пользователя, когда этот агент работает как основной агент потока. Игнорируется, когда агент вызывается как подагент                                                                                                                                                                                                                |
| `maxTurns`        | `number`                                                    | Нет         | Максимальное количество ходов агента перед остановкой агента. Когда агент достигает лимита, Claude Code возвращает его вывод, отмеченный как частичный, и вы можете [возобновить агента](#resume-subagents) для продолжения. Отметка о частичности требует Claude Code v2.1.246 или позже                                                                                     |
| `background`      | `boolean`                                                   | Нет         | Запустить этого агента как неблокирующую фоновую задачу при вызове                                                                                                                                                                                                                                                                                                            |
| `omitClaudeMd`    | `boolean`                                                   | Нет         | Запустить этого агента без файлов CLAUDE.md пользователя, проекта и локальных файлов, когда он работает как подагент; управляемые файлы политики все еще загружаются. Игнорируется, когда агент работает как основной агент потока. Требует TypeScript Agent SDK v0.3.271 или позже. Python SDK [`AgentDefinition`](/docs/ru/agent-sdk/python#agentdefinition) не имеет этого поля |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Нет         | Уровень усилий рассуждения для этого агента                                                                                                                                                                                                                                                                                                                                   |
| `permissionMode`  | `PermissionMode`                                            | Нет         | Режим разрешений для выполнения инструментов в этом агенте. [Правила наследования подагентов](/docs/ru/agent-sdk/permissions#available-modes) определяют, когда это применяется                                                                                                                                                                                                    |

В Python SDK многословные имена полей, такие как `disallowedTools` и `mcpServers`, сохраняют написание camelCase для соответствия формату передачи, а не следуют соглашению Python snake\_case. Подробности см. в [справочнике `AgentDefinition`](/docs/ru/agent-sdk/python#agentdefinition).

Подагенты работают в фоновом режиме по умолчанию. Вызов инструмента Agent, который опускает входные данные [`run_in_background`](/docs/ru/sub-agents#run-subagents-in-foreground-or-background), запускает фоновый подагент, и Claude устанавливает `run_in_background: false`, когда ему нужен результат перед продолжением. Установите поле `background` на `true`, чтобы принудительно выполнить фоновое выполнение для конкретного агента независимо от того, что запрашивает Claude. До Claude Code v2.1.198 фоновое значение по умолчанию постепенно развертывалось, и вызов инструмента Agent, который опускал `run_in_background`, мог запустить подагента синхронно.

Подагенты также могут порождать подагентов самостоятельно. Чтобы ограничить глубину этого вложения, количество одновременно работающих подагентов и сумму, которую запрос тратит, см. [Ограничение глубины подагента, параллелизма и расходов](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Определение на основе файловой системы (альтернатива)
</h3>

Вы также можете определить подагентов как файлы markdown в каталогах `.claude/agents/`. Подробности об этом подходе см. в [документации подагентов Claude Code](/docs/ru/sub-agents). Программно определенные агенты имеют приоритет над агентами на основе файловой системы с тем же именем.

<Note>
  Когда Claude вызывает инструмент Agent без `subagent_type`, он получает встроенного подагента `general-purpose`, который Claude может порождать даже если вы не определяете никаких агентов самостоятельно. Установка [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ru/env-vars) удаляет это значение по умолчанию, и такой вызов завершается ошибкой [`subagent_type is required`](/docs/ru/errors#subagent-type-is-required).
</Note>

<h2 id="what-subagents-inherit">
  Что наследуют подагенты
</h2>

Если подагент не является [форком](/docs/ru/sub-agents#fork-the-current-conversation), его контекстное окно начинается заново, без родительского разговора, но не пусто. Единственное содержимое, которое вы передаёте от родителя к подагенту, — это строка приглашения инструмента Agent, поэтому включайте любые пути к файлам, сообщения об ошибках или решения, которые нужны подагенту, непосредственно в это приглашение.

Подагент, у которого есть инструмент [`SendMessage`](/docs/ru/tools-reference), начинает со списка других именованных агентов, работающих в сеансе, поэтому он знает, каким именам он может отправлять сообщения. Claude Code автоматически добавляет список в первый ход подагента. [Форк](/docs/ru/sub-agents#fork-the-current-conversation) не получает список, потому что он наследует родительский разговор вместо этого.

Подагент также наследует конфигурацию расширенного мышления основного сеанса.

В таблице ниже указано, что содержит контекст подагента, не являющегося форком, и что он исключает.

| Подагент получает                                                                                                                                                                                                          | Подагент не получает                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- |
| Его собственное системное приглашение (`AgentDefinition.prompt`) и приглашение инструмента Agent                                                                                                                           | История разговора родителя или результаты инструментов                             |
| Project CLAUDE.md (загружается через [`settingSources`](/docs/ru/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), если агент не устанавливает [`omitClaudeMd`](#agentdefinition-configuration) | Предзагруженное содержимое навыков, если оно не указано в `AgentDefinition.skills` |
| Определения инструментов (унаследованные от родителя или подмножество в `tools`, [отфильтрованные для фоновых запусков](/docs/ru/sub-agents#available-tools))                                                                   | Системное приглашение родителя                                                     |

<Note>
  Родитель получает финальное сообщение подагента как результат инструмента Agent, но может обобщить его в своём ответе. Чтобы сохранить выходные данные подагента дословно в ответе, видимом пользователю, включите инструкцию об этом в приглашение или опцию `systemPrompt`, которую вы передаёте основному вызову `query()`.

  В v2.1.210 и более поздних версиях Claude Code [сканирует финальное сообщение на предмет шаблонов, похожих на инструкции](/docs/ru/sub-agents#subagent-output-scanning), прежде чем родитель его прочитает. Сканирование обрабатывает три вида шаблонов по-разному:

  * **Имитация управляющего тега**: Claude Code нейтрализует тег, который излучает только обвязка, например блок `<system-reminder>`, на месте. Он вставляет обратную косую черту после открывающей угловой скобки и ничего не удаляет.
  * **Упоминания конфигурации разрешений**: Claude Code сохраняет ссылки на конфигурацию разрешений, такие как `.claude/settings.json`, `bypassPermissions` или `--dangerously-skip-permissions`, как они написаны.
  * **Маркеры ходов**: строка, которая начинается с `Human:` или `Assistant:`, получает обратную косую черту перед двоеточием, чтобы сообщение не могло имитировать границу хода разговора.

  Для совпадения управляющего тега или конфигурации разрешений Claude Code добавляет в начало строку маркера `[harness: ...]`, называющую совпадающие шаблоны; совпадение маркера хода не добавляет строку маркера. Это единственные изменения, которые делает сканирование: оно никогда не удаляет и не переформулирует текст подагента.
</Note>

Ошибка API, которая завершает работу подагента досрочно, например ограничение скорости, никогда не доставляется как его результат. См. [Ошибки API в подагентах](/docs/ru/sub-agents#api-errors-in-subagents) для поведения переднего плана и фона.

<h2 id="invoke-subagents">
  Вызов подагентов
</h2>

<h3 id="automatic-invocation">
  Автоматический вызов
</h3>

Claude автоматически решает, когда вызвать подагентов на основе задачи и `description` каждого подагента. Например, если вы определите подагента `performance-optimizer` с описанием "Performance optimization specialist for query tuning", Claude вызовет его, когда ваш запрос упоминает оптимизацию запросов.

Пишите четкие, конкретные описания, чтобы Claude мог сопоставить задачи с правильным подагентом.

<h3 id="explicit-invocation">
  Явный вызов
</h3>

Чтобы гарантировать, что Claude использует конкретный подагент, упомяните его по имени в вашем запросе:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Это обходит автоматическое сопоставление и напрямую вызывает названный подагент.

<h3 id="dynamic-agent-configuration">
  Динамическая конфигурация агента
</h3>

Вы можете создавать определения агентов динамически на основе условий во время выполнения. Этот пример создает рецензента безопасности с разными уровнями строгости, используя более мощную модель для строгих проверок.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Обнаружение вызова подагента
</h2>

Claude вызывает подагентов через инструмент Agent. Чтобы обнаружить, когда подагент вызывается, проверьте блоки `tool_use`, где `name` равен `"Agent"`. Сообщения из контекста подагента включают поле `parent_tool_use_id`.

<Note>
  Инструмент отображается как `"Agent"` в блоках `tool_use`, но как `"Task"` в списке инструментов `system:init`. До Claude Code v2.1.63 блоки `tool_use` также называли его `"Task"`. Чтобы обнаружение работало во всех версиях SDK, сопоставьте оба значения в `block.name`.
</Note>

Структура сообщения отличается между SDK. В Python вы получаете доступ к блокам содержимого напрямую через `message.content`. В TypeScript `SDKAssistantMessage` оборачивает сообщение API Claude, поэтому вы получаете доступ к содержимому через `message.message.content`.

Этот пример проходит по потоковым сообщениям, логируя, когда подагент вызывается и когда последующие сообщения поступают из контекста выполнения этого подагента.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

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
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Возобновление подагентов
</h2>

Вы можете возобновить подагента, чтобы продолжить с того места, где он остановился, вместо того чтобы начинать заново. Возобновленный подагент сохраняет полную историю разговора, включая все предыдущие вызовы инструментов, результаты и рассуждения.

Когда подагент останавливается на пределе [`maxTurns`](#agentdefinition-configuration), Claude Code отмечает выходные данные в результате инструмента Agent как частичные, чтобы Claude знал, что выполнение незавершено.

Когда подагент завершает работу, результат инструмента Agent включает текстовый блок, содержащий `agentId: <id>`. Встроенные агенты [`Explore` и `Plan`](/docs/ru/sub-agents#built-in-subagents) являются одноразовыми и не возвращают `agentId`, поэтому используйте пользовательский агент или `general-purpose`, когда вам нужно возобновить. Чтобы возобновить подагента программно:

1. **Захватите ID сеанса**: извлеките `session_id` из сообщений во время первого запроса
2. **Извлеките ID агента**: разберите `agentId` из текста результата инструмента Agent
3. **Возобновите сеанс**: передайте `resume: sessionId` в параметрах второго запроса и включите ID агента в ваше приглашение. Каждый вызов `query()` по умолчанию запускает новый сеанс, и вы должны возобновить тот же сеанс, чтобы получить доступ к стенограмме подагента.

<Note>
  При использовании пользовательского агента передайте одно и то же определение агента в параметр `agents` для обоих запросов.
</Note>

Пример ниже определяет пользовательского агента `endpoint-finder`. Первый запрос запускает его и захватывает ID сеанса и ID агента из результата инструмента Agent, затем второй запрос возобновляет сеанс, чтобы задать дополнительный вопрос, требующий контекста из первого анализа.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

Стенограммы подагентов хранятся в отдельных файлах и сохраняются независимо от основного разговора. См. [возобновление подагентов в Claude Code](/docs/ru/sub-agents#resume-subagents) для поведения компактирования и периода очистки `cleanupPeriodDays`.

<h2 id="tool-restrictions">
  Ограничения инструментов
</h2>

Используйте поле `tools` для ограничения возможностей подагента:

* **Опустить `tools`**: подагент получает все [инструменты, доступные подагентам](/docs/ru/sub-agents#available-tools)
* **Список инструментов**: подагент получает только те. Например, проверяющий код, который никогда не должен редактировать файлы, получает `["Read", "Grep", "Glob"]`

Инструмент, который вы исключите, вообще не будет в сеансе подагента: Claude работает без него, без запроса разрешения или ошибки.

Этот пример создает агента анализа только для чтения, который может изучать код, но не может изменять файлы или выполнять команды.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
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
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Распространенные комбинации инструментов
</h3>

| Вариант использования    | Инструменты                             | Описание                                                            |
| :----------------------- | :-------------------------------------- | :------------------------------------------------------------------ |
| Анализ только для чтения | `Read`, `Grep`, `Glob`                  | Может изучать код, но не может изменять или выполнять               |
| Выполнение тестов        | `Bash`, `Read`, `Grep`                  | Может выполнять команды и анализировать результаты                  |
| Изменение кода           | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Полный доступ на чтение/запись без выполнения команд                |
| Полный доступ            | Все инструменты                         | Наследует инструменты, доступные подагентам (опустите поле `tools`) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Ограничение глубины, параллелизма и расходов подагентов
</h2>

<Note>
  В этом разделе описываются TypeScript SDK v0.3.219 и Python SDK v0.2.127 и более поздние версии, релизы, которые включают Claude Code v2.1.219 или более поздние версии. В более ранних релизах некоторые из этих ограничений отсутствуют или имеют другие значения по умолчанию, поэтому обновитесь перед тем, как полагаться на них для ограничения выполнения. [Справочник переменных окружения](/docs/ru/env-vars) и [ходы и бюджет](/docs/ru/agent-sdk/agent-loop#turns-and-budget) содержат информацию о версии Claude Code, которая добавила каждую переменную, и о применении ограничения расходов подагентами.
</Note>

Claude самостоятельно решает, когда создать подагента и сколько их создать. Каждый подагент делает свои собственные запросы API, которые учитываются в `total_cost_usd` запроса, и подагент может создавать подагентов самостоятельно, поэтому один запрос может превратиться в дерево агентов.

Вы можете ограничить этот рост тремя способами: насколько глубоко вложены подагенты, сколько их работает одновременно и сколько потратит весь запрос. Установите ограничения глубины и параллелизма как переменные окружения через опцию [`env`](/docs/ru/agent-sdk/typescript#options), а ограничение расходов как опцию запроса:

| Ограничение | Установить с помощью                                   | По умолчанию                                                                                                          | Что делает Claude Code при достижении ограничения                                                                                                                                                                                                                                                                                                                                         |
| :---------- | :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Глубина     | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/ru/env-vars) | `3` слоя подагентов ниже вашего основного агента. `1` предотвращает создание подагентами своих собственных подагентов | Оставляет подагента на нижнем слое неспособным создавать новых, поэтому он выполняет свою делегированную работу самостоятельно. См. [вложенные подагенты](/docs/ru/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                                         |
| Параллелизм | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/ru/env-vars) | `20` подагентов, работающих одновременно, считая каждого подагента, созданного Claude с помощью инструмента Agent     | Отказывает в создании другого подагента, возвращая `Concurrent subagent limit reached`, пока количество работающих не упадет ниже ограничения. Сеансы с активным [ultracode](/docs/ru/model-config#adjust-effort-level) никогда не отклоняются. См. [ограничение параллельных подагентов](/docs/ru/sub-agents#concurrent-subagent-limit)                                                            |
| Расходы     | `maxBudgetUsd` в TypeScript, `max_budget_usd` в Python | Без ограничений. Сравнивается с `total_cost_usd`, поэтому запросы подагентов учитываются                              | Применяет ограничение тремя способами: отказывает в создании дополнительных подагентов, возвращая `Budget limit reached`, останавливает фоновые подагенты, которые все еще работают, и завершает запрос с подтипом результата `error_max_budget_usd`. Для получения информации о том, как ограничения ведут себя в сеансе, см. [ходы и бюджет](/docs/ru/agent-sdk/agent-loop#turns-and-budget) |

Два SDK по-разному обрабатывают опцию `env`: TypeScript SDK заменяет окружение подпроцесса на него, поэтому распределите `process.env` в него, чтобы сохранить переменные вроде `PATH`, в то время как Python SDK объединяет его с унаследованным окружением. Этот пример отключает вложение, позволяет максимум пять подагентов одновременно и останавливает запрос, когда предполагаемые расходы достигают \$5:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

То, что вы видите, зависит от того, какое ограничение, если оно есть, достигает запрос:

* **Ниже ограничения расходов**: вы видите `success` и предполагаемую стоимость.
* **При ограничении расходов**: вы видите `error_max_budget_usd` со стоимостью на уровне или выше `5`, а затем запускается ваш обработчик ошибок.
* **При ограничении параллелизма**: вы видите блок `tool_result` в потоке сообщений, содержащий `Concurrent subagent limit reached`. Claude получает тот же блок как результат инструмента Agent.

<h3 id="run-opus-5-with-subagents">
  Запуск Opus 5 с подагентами
</h3>

Claude Opus 5 делегирует подагентам более охотно, чем более ранние модели, поэтому [ограничения глубины, параллелизма и расходов](#cap-subagent-depth-concurrency-and-spend) имеют наибольшее значение для запросов, которые запускают Opus 5. [Руководство по подсказкам Opus 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) содержит инструкцию делегирования, которую вы можете добавить к любой подсказке. Добавляет ли Claude Code свою собственную инструкцию, зависит от того, какой [системный запрос](/docs/ru/agent-sdk/modifying-system-prompts#how-system-prompts-work) вы используете:

* **Предустановка `claude_code`**: когда моделью является Opus 5, Claude Code добавляет строку в свой системный запрос, указывая Claude не вызывать инструмент Agent, если его об этом не попросят. Инструмент Agent остается доступным.
* **Пользовательский запрос или отсутствие `systemPrompt`**: Claude Code не создает свой системный запрос, поэтому эта строка отсутствует. Добавьте инструкцию делегирования из руководства подсказок в свой собственный запрос.

Любая инструкция только направляет Claude, поэтому установите ограничения также. Claude Code применяет их независимо от того, как Claude решит делегировать.

<h2 id="scale-up-with-dynamic-workflows">
  Масштабирование с помощью динамических рабочих процессов
</h2>

Подагенты хорошо работают для нескольких делегированных задач за ход. Для запусков, которые координируют десятки или сотни агентов, используйте инструмент `Workflow`, который перемещает оркестровку в скрипт, который среда выполнения выполняет вне контекста беседы. Подробнее о том, чем рабочие процессы отличаются от делегирования подагентов по ходам, см. в [динамических рабочих процессах](/docs/ru/workflows).

Инструмент `Workflow` доступен в TypeScript Agent SDK v0.3.149 и позже. Включите `Workflow` в `allowedTools` для автоматического одобрения запусков рабочих процессов. Схемы входных и выходных данных инструмента указаны в [справочнике TypeScript](/docs/ru/agent-sdk/typescript#workflow).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude не делегирует подагентам
</h3>

Если Claude выполняет задачи напрямую вместо делегирования вашему подагенту:

* **Используйте явное указание**: упомяните подагента по имени в своем промпте, например "Use the code-reviewer agent to check the authentication module"
* **Напишите четкое описание**: объясните ровно, когда следует использовать подагента, чтобы Claude мог правильно сопоставить задачи

<h3 id="filesystem-based-agents-not-loading">
  Агенты на основе файловой системы не загружаются
</h3>

Claude Code отслеживает `~/.claude/agents/` и `.claude/agents/` и подхватывает новый или отредактированный файл агента в течение нескольких секунд без необходимости перезагрузки. Если определение никогда не появляется, проработайте эти причины:

* **Новая директория `agents`**: наблюдатель охватывает только директории, которые существовали при запуске сессии, поэтому первый файл в новой директории требует перезагрузки сессии. Это наиболее частая причина.
* **Неверный frontmatter или дублирующееся имя `name`**: проверьте YAML файла и то, использует ли существующий агент уже это имя `name`.
* **`--disable-slash-commands`**: сессии, запущенные с этим флагом, не отслеживают эти директории и всегда требуют перезагрузки для загрузки новых файлов.
* **Файл в добавленной директории**: Claude Code загружает `.claude/agents/` из директорий, добавленных с помощью опции `add_dirs` (Python) или `additionalDirectories` (TypeScript), или CLI флагов `--add-dir` или `/add-dir`, но не отслеживает их, поэтому новый или отредактированный файл там требует перезагрузки сессии.
* **Программный агент с тем же именем**: `agents`, переданные в `query()`, переопределяют агента файловой системы с тем же именем.

Для формата файла см. [как писать файлы подагентов](/docs/ru/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Подагенты Claude Code](/docs/ru/sub-agents): полная документация подагентов, включая определения на основе файловой системы
* [Динамические рабочие процессы](/docs/ru/workflows): оркестрируйте множество подагентов из скрипта для работ, слишком больших для одной беседы
* [Обзор SDK](/docs/ru/agent-sdk/overview): начало работы с Claude Agent SDK
