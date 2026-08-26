> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Подагенты в SDK

> Определяйте и вызывайте подагентов для изоляции контекста, параллельного выполнения задач и применения специализированных инструкций в приложениях Claude Agent SDK.

Подагенты — это отдельные экземпляры агентов, которые ваш основной агент может создавать для обработки сосредоточенных подзадач.
Используйте подагентов для изоляции контекста, параллельного запуска нескольких анализов и применения специализированных инструкций без перегрузки основного промпта агента.

В этом руководстве объясняется, как определять и использовать подагентов в SDK с помощью параметра `agents`.

<h2 id="overview">
  Обзор
</h2>

Вы можете создавать подагентов тремя способами:

* **Программно**: используйте параметр `agents` в параметрах `query()`. См. справочники [TypeScript](/docs/ru/agent-sdk/typescript#agentdefinition) и [Python](/docs/ru/agent-sdk/python#agentdefinition)
* **На основе файловой системы**: определяйте агентов как файлы markdown в директориях `.claude/agents/`. См. [определение подагентов как файлов](/docs/ru/sub-agents)
* **Встроенный универсальный**: Claude может вызывать встроенного подагента `general-purpose` в любое время через инструмент Agent без необходимости что-либо определять

Это руководство сосредоточено на программном подходе, который рекомендуется для приложений SDK.

Когда вы определяете подагентов, Claude определяет, следует ли их вызывать, на основе поля `description` каждого подагента. Напишите четкие описания, которые объясняют, когда следует использовать подагента, и Claude автоматически делегирует соответствующие задачи. Вы также можете явно запросить подагента по имени в своем промпте, например "Используйте агента code-reviewer для...".

<h2 id="benefits-of-using-subagents">
  Преимущества использования подагентов
</h2>

<h3 id="context-isolation">
  Изоляция контекста
</h3>

Каждый подагент работает в своей собственной свежей беседе. Промежуточные вызовы инструментов и результаты остаются внутри подагента; только его финальное сообщение возвращается к родительскому агенту. См. [Что наследуют подагенты](#what-subagents-inherit) для точного понимания того, что находится в контексте подагента.

**Пример:** подагент `research-assistant` может исследовать десятки файлов без накопления этого содержимого в основной беседе. Родительский агент получает краткое резюме, а не каждый файл, который прочитал подагент.

<h3 id="parallelization">
  Параллелизация
</h3>

Несколько подагентов могут работать одновременно, поэтому независимые подзадачи завершаются за время самого медленного из них, а не за сумму всех времён.

**Пример:** во время проверки кода вы можете одновременно запустить подагентов `style-checker`, `security-scanner` и `test-coverage` вместо последовательного запуска.

<h3 id="specialized-instructions-and-knowledge">
  Специализированные инструкции и знания
</h3>

Каждый подагент может иметь адаптированные системные промпты со специфической экспертизой, лучшими практиками и ограничениями.

**Пример:** подагент `database-migration` может иметь подробные знания о лучших практиках SQL, стратегиях отката и проверках целостности данных, которые были бы ненужным шумом в инструкциях основного агента.

<h3 id="tool-restrictions">
  Ограничения инструментов
</h3>

Подагенты могут быть ограничены определенными инструментами, снижая риск непредвиденных действий.

**Пример:** подагент `doc-reviewer` может иметь доступ только к инструментам Read и Grep, обеспечивая анализ, но никогда случайно не модифицируя файлы документации.

<h2 id="create-subagents">
  Создание подагентов
</h2>

<h3 id="programmatic-definition-recommended">
  Программное определение (рекомендуется)
</h3>

Определяйте подагентов непосредственно в коде, используя параметр `agents`. Claude вызывает подагентов через инструмент `Agent`, поэтому включите `Agent` в `allowedTools` для автоматического одобрения вызовов подагентов без запроса разрешения.

Большинство примеров на этой странице выводят только окончательный результат. Чтобы подтвердить, что Claude делегировал работу подагенту, а не ответил напрямую, см. раздел [Обнаружение вызова подагента](#detect-subagent-invocation).

Этот пример создает двух подагентов: рецензента кода с доступом только для чтения и средство запуска тестов, которое может выполнять команды.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
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
      // Auto-approve these tools, including Agent for subagent invocation
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

| Поле              | Тип                                                         | Обязательно | Описание                                                                                                                                                                                                                                                   |
| :---------------- | :---------------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Да          | Описание на естественном языке того, когда использовать этого агента                                                                                                                                                                                       |
| `prompt`          | `string`                                                    | Да          | Системный промпт агента, определяющий его роль и поведение                                                                                                                                                                                                 |
| `tools`           | `string[]`                                                  | Нет         | Массив разрешенных имен инструментов. Если опущено, наследует все инструменты                                                                                                                                                                              |
| `disallowedTools` | `string[]`                                                  | Нет         | Массив имен инструментов для удаления из набора инструментов агента. Также принимаются шаблоны уровня MCP сервера: `mcp__server` или `mcp__server__*` удаляет каждый инструмент с этого сервера, а `mcp__*` удаляет каждый инструмент MCP с любого сервера |
| `model`           | `string`                                                    | Нет         | Переопределение модели для этого агента. Принимает псевдоним, такой как `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, или полный ID модели. По умолчанию используется основная модель, если опущено                                             |
| `skills`          | `string[]`                                                  | Нет         | Список имен skills для предварительной загрузки в контекст агента при запуске. Неперечисленные skills остаются вызываемыми через инструмент Skill                                                                                                          |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Нет         | Источник памяти для этого агента                                                                                                                                                                                                                           |
| `mcpServers`      | `(string \| object)[]`                                      | Нет         | MCP серверы, доступные этому агенту, по имени или встроенной конфигурации                                                                                                                                                                                  |
| `initialPrompt`   | `string`                                                    | Нет         | Автоматически отправляется как первый ход пользователя, когда этот агент работает как основной потоковый агент. Игнорируется, когда агент вызывается как подагент                                                                                          |
| `maxTurns`        | `number`                                                    | Нет         | Максимальное количество ходов агента перед остановкой                                                                                                                                                                                                      |
| `background`      | `boolean`                                                   | Нет         | Запустить этого агента как неблокирующую фоновую задачу при вызове                                                                                                                                                                                         |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Нет         | Уровень усилий рассуждения для этого агента                                                                                                                                                                                                                |
| `permissionMode`  | `PermissionMode`                                            | Нет         | Режим разрешений для выполнения инструментов в этом агенте                                                                                                                                                                                                 |

В Python SDK многословные имена полей, такие как `disallowedTools` и `mcpServers`, сохраняют написание camelCase для соответствия формату передачи, а не следуют соглашению Python snake\_case. Подробности см. в [справочнике `AgentDefinition`](/docs/ru/agent-sdk/python#agentdefinition).

Два поведения подагентов изменились в Claude Code v2.1.198:

* Подагенты работают в фоновом режиме по умолчанию. Вызов инструмента Agent, который опускает входной параметр [`run_in_background`](/docs/ru/agent-sdk/typescript), запускает фоновый подагент, и Claude устанавливает `run_in_background: false`, когда ему нужен результат перед продолжением. До версии v2.1.198 опускание `run_in_background` запускало подагента синхронно. Установите поле `background` в `true`, чтобы принудительно включить фоновое выполнение для конкретного агента независимо от того, что запрашивает Claude.
* Подагент наследует конфигурацию расширенного мышления основной сессии. В более ранних версиях расширенное мышление отключено внутри подагентов независимо от параметра основной сессии.

<Note>
  Начиная с Claude Code v2.1.172, подагенты могут создавать своих собственных подагентов. Подагент на пять уровней ниже основного агента не может создавать дополнительных подагентов, независимо от того, работает ли он в переднем плане или в фоновом режиме. Чтобы предотвратить создание подагентом других подагентов, опустите `Agent` из его массива `tools` или добавьте его в `disallowedTools`. Полные правила глубины см. в разделе [вложенные подагенты](/docs/ru/sub-agents#spawn-nested-subagents).
</Note>

<h3 id="filesystem-based-definition-alternative">
  Определение на основе файловой системы (альтернатива)
</h3>

Вы также можете определять подагентов как файлы markdown в директориях `.claude/agents/`. Подробности об этом подходе см. в [документации подагентов Claude Code](/docs/ru/sub-agents). Программно определенные агенты имеют приоритет над агентами на основе файловой системы с тем же именем.

<Note>
  Даже без определения пользовательских подагентов, Claude может создавать встроенного подагента `general-purpose`. Это полезно для делегирования задач исследования или исследования без создания специализированных агентов. Включите `Agent` в `allowedTools`, чтобы эти вызовы автоматически одобрялись без запроса разрешения.
</Note>

<h2 id="what-subagents-inherit">
  Что наследуют подагенты
</h2>

Окно контекста подагента начинается свежим (без истории родительской беседы), но не пусто. Единственный канал от родителя к подагенту — это строка промпта инструмента Agent, поэтому включайте любые пути файлов, сообщения об ошибках или решения, которые нужны подагенту, непосредственно в этот промпт.

Подагент, у которого есть инструмент [`SendMessage`](/docs/ru/tools-reference), начинает с списка других именованных агентов, работающих в сеансе, поэтому он знает, каким именам он может отправлять сообщения. Claude Code автоматически добавляет список в первый ход подагента. [Ветвление](/docs/ru/sub-agents#fork-the-current-conversation) не получает список, потому что оно наследует родительскую беседу вместо этого. Список требует Claude Code v2.1.206 или более поздней версии.

| Подагент получает                                                                                                                           | Подагент не получает                                                                     |
| :------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------- |
| Его собственный системный промпт (`AgentDefinition.prompt`) и промпт инструмента Agent                                                      | История беседы родителя или результаты инструментов                                      |
| Проект CLAUDE.md (загруженный через [`settingSources`](/docs/ru/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Предварительно загруженное содержимое skills, если не указано в `AgentDefinition.skills` |
| Определения инструментов (унаследованные от родителя или подмножество в `tools`)                                                            | Системный промпт родителя                                                                |

<Note>
  Родитель получает финальное сообщение подагента дословно как результат инструмента Agent, но может суммировать его в своем собственном ответе. Чтобы сохранить выходные данные подагента дословно в ответе, обращенном к пользователю, включите инструкцию об этом в промпт или опцию `systemPrompt`, которую вы передаете основному вызову `query()`.
</Note>

Ошибка API, которая завершает работу подагента раньше времени, такая как ограничение скорости, никогда не доставляется как его результат. Если ограничение скорости, перегрузка или ошибка сервера прерывает подагента на переднем плане, который уже произвел текстовый выход, инструмент Agent возвращает этот частичный выход с примечанием о том, что подагент не завершил работу. Подагент, который не произвел ничего или чей единственный выход был вызовами инструментов без текста, завершается с сообщением об ошибке `Agent terminated early due to an API error`, за которым следует деталь ошибки. См. [API errors in subagents](/docs/ru/sub-agents#api-errors-in-subagents) для поведения на переднем плане и в фоновом режиме.

Эта обработка частичного выхода требует Claude Code v2.1.199 или более поздней версии. В v2.1.199 ограничение скорости, перегрузка или ошибка сервера оставляли форму, содержащую только вызовы инструментов, с пустым частичным результатом, содержащим только примечание об отсечении.

<h2 id="invoke-subagents">
  Вызов подагентов
</h2>

<h3 id="automatic-invocation">
  Автоматический вызов
</h3>

Claude автоматически решает, когда вызывать подагентов, на основе задачи и поля `description` каждого подагента. Например, если вы определяете подагента `performance-optimizer` с описанием "Performance optimization specialist for query tuning", Claude вызовет его, когда ваш промпт упоминает оптимизацию запросов.

Напишите четкие, конкретные описания, чтобы Claude мог сопоставить задачи с правильным подагентом.

<h3 id="explicit-invocation">
  Явный вызов
</h3>

Чтобы гарантировать, что Claude использует определенного подагента, упомяните его по имени в своем промпте:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Это обходит автоматическое сопоставление и напрямую вызывает названного подагента.

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

Claude вызывает подагентов через инструмент Agent. Чтобы обнаружить, когда вызывается подагент, проверьте блоки `tool_use`, где `name` — это `"Agent"`. Сообщения из контекста подагента включают поле `parent_tool_use_id`.

<Note>
  Имя инструмента было переименовано с `"Task"` на `"Agent"` в Claude Code v2.1.63. Текущие выпуски SDK выдают `"Agent"` в блоках `tool_use`, но все еще используют `"Task"` в списке инструментов `system:init` и в `result.permission_denials[].tool_name`. Проверка обоих значений в `block.name` обеспечивает совместимость между версиями SDK.
</Note>

Структура сообщения отличается между SDK. В Python блоки содержимого доступны непосредственно через `message.content`. В TypeScript `SDKAssistantMessage` оборачивает сообщение Claude API, поэтому содержимое доступно через `message.message.content`.

Этот пример проходит через потоковые сообщения, логируя, когда вызывается подагент и когда последующие сообщения исходят из контекста выполнения этого подагента.

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

Вы можете возобновить подагента, чтобы продолжить с того места, где он остановился, а не начинать заново. Возобновленный подагент сохраняет полную историю беседы, включая все предыдущие вызовы инструментов, результаты и рассуждения.

Когда подагент завершается, результат инструмента Agent включает текстовый блок, содержащий `agentId: <id>`. Встроенные агенты [`Explore` и `Plan`](/docs/ru/sub-agents#built-in-subagents) работают в один проход и не возвращают `agentId`, поэтому используйте пользовательского агента или `general-purpose`, когда вам нужно возобновить. Чтобы программно возобновить подагента:

1. **Захватите ID сессии**: извлеките `session_id` из сообщений во время первого запроса
2. **Извлеките ID агента**: разберите `agentId` из текста результата инструмента Agent
3. **Возобновите сессию**: передайте `resume: sessionId` в параметрах второго запроса и включите ID агента в ваш промпт

<Note>
  Вы должны возобновить ту же сессию, чтобы получить доступ к стенограмме подагента. Каждый вызов `query()` по умолчанию начинает новую сессию, поэтому передайте `resume: sessionId`, чтобы продолжить в той же сессии.

  При использовании пользовательского агента передайте то же определение агента в параметр `agents` для обоих запросов.
</Note>

Пример ниже определяет пользовательского агента `endpoint-finder`. Первый запрос запускает его и захватывает ID сессии и ID агента из результата инструмента Agent, затем второй запрос возобновляет сессию, чтобы задать вопрос для уточнения, требующий контекста из первого анализа.

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

Стенограммы подагентов сохраняются независимо от основной беседы:

* **Компактирование основной беседы**: когда основная беседа компактируется, стенограммы подагентов не затрагиваются. Они хранятся в отдельных файлах.
* **Сохранение сессии**: стенограммы подагентов сохраняются в пределах их сессии. Вы можете возобновить подагента после перезагрузки Claude Code, возобновив ту же сессию.
* **Автоматическая очистка**: стенограммы очищаются на основе параметра `cleanupPeriodDays`, который по умолчанию составляет 30 дней.

<h2 id="tool-restrictions-2">
  Ограничения инструментов
</h2>

Подагенты могут иметь ограниченный доступ к инструментам через поле `tools`:

* **Опустить поле**: агент наследует все доступные инструменты (по умолчанию)
* **Указать инструменты**: агент может использовать только перечисленные инструменты

Этот пример создает агента анализа только для чтения, который может изучать код, но не может изменять файлы или запускать команды.

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

| Вариант использования    | Инструменты                             | Описание                                                      |
| :----------------------- | :-------------------------------------- | :------------------------------------------------------------ |
| Анализ только для чтения | `Read`, `Grep`, `Glob`                  | Может изучать код, но не может изменять или выполнять         |
| Выполнение тестов        | `Bash`, `Read`, `Grep`                  | Может запускать команды и анализировать выходные данные       |
| Модификация кода         | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Полный доступ на чтение/запись без выполнения команд          |
| Полный доступ            | Все инструменты                         | Наследует все инструменты от родителя (опустите поле `tools`) |

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

* **Проверьте, что вызовы Agent одобрены**: включите `Agent` в `allowedTools` для автоматического одобрения вызовов подагента. Без этого вызовы Agent переходят к вашему обратному вызову `canUseTool` или в режиме `dontAsk` отклоняются
* **Используйте явное указание**: упомяните подагента по имени в своем промпте, например "Use the code-reviewer agent to..."
* **Напишите четкое описание**: объясните ровно, когда следует использовать подагента, чтобы Claude мог правильно сопоставить задачи

<h3 id="filesystem-based-agents-not-loading">
  Агенты на основе файловой системы не загружаются
</h3>

Claude Code отслеживает `~/.claude/agents/` и `.claude/agents/` и подхватывает новый или отредактированный файл агента в течение нескольких секунд без необходимости перезагрузки. Если определение никогда не появляется, проработайте эти причины:

* **Новая директория `agents`**: наблюдатель охватывает только директории, которые существовали при запуске сессии, поэтому первый файл в новой директории требует перезагрузки сессии. Это наиболее частая причина.
* **Неверный frontmatter или дублирующееся имя `name`**: проверьте YAML файла и то, использует ли существующий агент уже это имя `name`.
* **`--disable-slash-commands`**: сессии, запущенные с этим флагом, не отслеживают эти директории и всегда требуют перезагрузки для загрузки новых файлов.
* **Программный агент с тем же именем**: `agents`, переданные в `query()`, переопределяют агента файловой системы с тем же именем.

Для формата файла см. [как писать файлы подагентов](/docs/ru/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Сбои при длинных промптах в Windows
</h3>

В Windows подагенты с очень длинными промптами могут не работать из-за ограничения длины командной строки в 8191 символов. Держите промпты краткими или используйте агентов на основе файловой системы для сложных инструкций.

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Подагенты Claude Code](/docs/ru/sub-agents): полная документация подагентов, включая определения на основе файловой системы
* [Динамические рабочие процессы](/docs/ru/workflows): оркестрируйте множество подагентов из скрипта для работ, слишком больших для одной беседы
* [Обзор SDK](/docs/ru/agent-sdk/overview): начало работы с Claude Agent SDK
