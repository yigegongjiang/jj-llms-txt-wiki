> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Расширьте агентов с помощью skills

> Управляйте тем, какие skills может вызывать Claude в сеансах Claude Agent SDK, отправляйте команды по имени и создавайте skills, которые обнаруживают ваши сеансы

Agent Skills расширяют Claude специализированными возможностями, которые Claude вызывает при необходимости. Skills упаковываются в виде файлов `SKILL.md`, содержащих инструкции, описания и дополнительные вспомогательные ресурсы. На этой странице также рассматриваются [команды в сеансах Agent SDK](#commands-in-agent-sdk-sessions).

Для получения полной информации о skills, включая преимущества, архитектуру и рекомендации по разработке, см. [обзор Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Как skills работают с Agent SDK
</h2>

При использовании Claude Agent SDK skills:

* **Определяются как артефакты файловой системы**: вы создаёте каждый skill как файл `SKILL.md` в его собственном каталоге, например `.claude/skills/<name>/SKILL.md`
* **Загружаются из файловой системы**: SDK загружает skills из расположений файловой системы, управляемых `settingSources` (TypeScript) или `setting_sources` (Python)
* **Автоматически обнаруживаются**: после загрузки параметров файловой системы SDK обнаруживает метаданные skill при запуске из пользовательских и проектных каталогов и загружает полное содержимое, когда Claude вызывает skill
* **Вызываются моделью**: Claude автономно выбирает, когда их использовать, на основе контекста
* **Вызываются пользователем**: вы отправляете skill напрямую, отправляя `/<name>` в подсказку. См. [Команды в сеансах Agent SDK](#commands-in-agent-sdk-sessions)
* **Ограничиваются через опцию `skills`**: обнаруженные skills включены по умолчанию. Передайте список имён skills, `"all"` или `[]` для управления тем, какие skills может вызывать Claude

В отличие от subagents, которые вы можете определить в [опции `agents`](/docs/ru/agent-sdk/subagents#programmatic-definition-recommended), вы создаёте skills как файлы на диске. SDK не предоставляет программный API для их регистрации.

<Note>
  Skills обнаруживаются через источники параметров файловой системы. С параметрами `query()` по умолчанию SDK загружает пользовательские и проектные источники, поэтому skills в `~/.claude/skills/`, `<cwd>/.claude/skills/` и `.claude/skills/` в любом родительском каталоге `<cwd>` вплоть до корня репозитория доступны. Проектный источник также охватывает `<dir>/.claude/skills/` в каждом каталоге, который вы передаёте через `additionalDirectories` (TypeScript) или `add_dirs` (Python), потому что SDK передаёт эти каталоги в Claude Code как [`--add-dir`](/docs/ru/skills#skills-from-additional-directories). Если вы явно установите `settingSources`, включите `'project'` для сохранения skills проекта и добавленного каталога и `'user'` для сохранения ваших личных skills, или используйте [опцию `plugins`](/docs/ru/agent-sdk/plugins) для загрузки skills из определённого пути.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Использование skills с Agent SDK
</h2>

Установите опцию `skills` на `query()` для управления тем, какие skills может вызывать Claude в сеансе. Если опция опущена, обнаруженные skills включены и инструмент Skill доступен, что соответствует поведению CLI. Передайте `"all"` для того, чтобы Claude мог вызывать каждый обнаруженный skill, список имён skills для разрешения только тех или `[]` для того, чтобы Claude не мог вызывать ни один.

Например, чтобы позволить Claude вызывать только два именованных skill:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Настройка skills в сеансе
</h3>

Когда вы устанавливаете `skills`, SDK автоматически добавляет инструмент Skill в `allowedTools`. Если вы также передаёте явный список `tools`, включите `"Skill"` в этот список, чтобы Claude мог вызывать skills.

После настройки Claude автоматически обнаруживает skills из файловой системы и вызывает их при необходимости для запроса пользователя.

Следующий пример включает каждый обнаруженный skill в сеансе и предварительно одобряет инструменты, которые skills обычно требуют. Пример устанавливает `cwd` на текущий рабочий каталог процесса, поэтому запустите его из проекта, который имеет каталог `.claude/skills/` в текущем каталоге или в любом родительском каталоге вплоть до корня репозитория:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Подтверждение загрузки skills
</h3>

В начале потока SDK выдаёт системное сообщение с подтипом `init`. Проверьте его массив `skills`, чтобы подтвердить, что ваши skills загружены, прежде чем Claude начнёт работу. Массив включает skills, которые можно вызывать пользователем, которые вы определили с полем frontmatter `description` или `when_to_use`, а также [встроенные skills, включённые в Claude Code](/docs/ru/skills#bundled-skills).

Массив содержит только skills, которые можно вызывать пользователем. Skill с [`user-invocable: false`](/docs/ru/skills#control-who-invokes-a-skill) в его frontmatter загружается и остаётся доступным для Claude, но не появляется в массиве. Массив отражает то, что обнаружил сеанс, и содержит одни и те же skills независимо от того, находятся ли они в вашем списке `skills`.

<h3 id="allow-only-specific-skills">
  Разрешить только определённые skills
</h3>

Чтобы позволить Claude вызывать только определённые skills, передайте их имена в списке `skills`. Имена соответствуют полю `name` в `SKILL.md` или имени каталога skill. Используйте `plugin:skill` для skills, предоставляемых плагинами.

Список принимает только точные имена skills. Если запись не может работать как точное имя, `query()` отклоняет список перед началом сеанса. См. [Ошибка неверного имени skill](#invalid-skill-name-error) для правил имён и ошибки, которую выдаёт каждый SDK.

Модель не видит неуказанные skills и инструмент Skill их отклоняет, в то время как их файлы остаются на диске и остаются доступными через Read и Bash. Ограничение списка не ограничивает [отправку по имени](#dispatch-commands-by-name).

Чтобы позволить Claude вызывать каждый обнаруженный skill, передайте `skills: "all"` вместо подстановочного знака.

<h2 id="commands-in-agent-sdk-sessions">
  Команды в сеансах Agent SDK
</h2>

Этот раздел — документация команд SDK. Команда — это всё, что вы запускаете, отправляя `/<name>` в подсказку. Записи на поверхности команд отличаются тем, что их поддерживает:

* **Встроенные команды**: выполняют логику, закодированную в процесс Claude Code, который запускает SDK, например `/compact`
* **Встроенные skills**: артефакты подсказок, включённые в Claude Code, например `/code-review`
* **Ваши skills**: артефакты подсказок, которые вы создаёте, каждый — каталог, содержащий файл `SKILL.md`. Имя skill, который можно вызывать пользователем, автоматически присоединяется к поверхности, поэтому отправка вашего собственного `/security-check` и запуск встроенного работают одинаково
* **Файлы пользовательских команд**: более старая форма артефакта с тем же поведением, плоские файлы Markdown в `.claude/commands/`, имена файлов которых становятся именами команд. Skills — их рекомендуемый преемник

По умолчанию как вы, так и Claude можете вызывать любой skill. Вы можете ограничить любой путь через [frontmatter](/docs/ru/skills#control-who-invokes-a-skill) skill. Для определений команды и skill см. записи глоссария [Команда](/docs/ru/glossary#command) и [Skill](/docs/ru/glossary#skill). См. [Команды в Claude Code](/docs/ru/commands) для каждой встроенной и [Расширьте Claude с помощью skills](/docs/ru/skills) для полного руководства по обеим формам артефактов.

<h3 id="discover-available-commands">
  Обнаружение доступных команд
</h3>

Вы можете отправлять команды, которые работают без интерактивного терминала, через SDK. Сообщение `system/init` содержит доступные в вашем сеансе в его поле `slash_commands`. Команды, которые требуют интерактивного терминала, такие как `/theme` и `/terminal-setup`, не появляются в списке. Получите доступ к полю при запуске вашего сеанса:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

Выведенный список смешивает встроенные команды, встроенные skills, ваши skills, которые можно вызывать пользователем, и файлы `.claude/commands/`:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Skill с [`user-invocable: false`](/docs/ru/skills#control-who-invokes-a-skill) в его frontmatter не появляется в этом списке или в массиве `skills` из [Подтверждение загрузки skills](#confirm-skills-loaded). Сеансы, которые настраивают [MCP серверы](/docs/ru/agent-sdk/mcp), также могут предоставлять [MCP подсказки как команды](/docs/ru/mcp#use-mcp-prompts-as-commands).

<h3 id="dispatch-commands-by-name">
  Отправка команд по имени
</h3>

Отправьте команду, включив её в строку подсказки, так же как вы отправляете обычный текст. Отправка не зависит от опции `skills`. Отправка `/<name>` запускает skill, который можно вызывать пользователем, даже когда ваш список `skills` его опускает. Команды, которые действуют на историю разговора, такие как `/compact`, требуют предыдущих сообщений для работы.

`/<name>`, который не совпадает ни с командой в сеансе, ни со встроенной командой Claude Code, не приводит к сбою запроса. Claude Code отправляет подсказку Claude как обычное сообщение с примечанием о том, что команда не была выполнена, поэтому запрос использует ход модели и возвращает ответ Claude. До версии 2.1.274 `/<name>`, который ничему не соответствовал, возвращал `Unknown command: /<name>` как результат без хода модели.

`/<name>`, который совпадает со встроенной командой Claude Code, которая недоступна в сеансе, такой как `/theme`, возвращает `/theme isn't available in this environment.` как результат без хода модели.

<Note>
  Команда может достичь лимита `maxTurns` / `max_turns` как любая другая подсказка, завершив запрос с результатом ошибки вместо `success`. Для контракта результата ошибки см. [Обработка результата](/docs/ru/agent-sdk/agent-loop#handle-the-result). Если ваша команда может достичь лимита, оберните цикл в `try`/`catch` в TypeScript или `try`/`except` в Python, как показано в [Ввод одного сообщения](/docs/ru/agent-sdk/streaming-vs-single-mode#single-message-input), или установите `maxTurns` достаточно высоко для завершения работы.
</Note>

<h3 id="compact-history-with-/compact">
  Сжатие истории с помощью `/compact`
</h3>

Команда `/compact` уменьшает размер истории вашего разговора путём суммирования старых сообщений при сохранении важного контекста. Сжатие требует существующего разговора с достаточным количеством предыдущих сообщений для суммирования. Этот пример сначала имеет разговор, затем сжимает его и читает системное сообщение `compact_boundary`, которое сообщает результат:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Сообщение `compact_boundary` поступает только при выполнении сжатия. Если нечего суммировать, `/compact` сообщает причину вместо выдачи ошибки. Запуск всё ещё заканчивается результатом `success` и без сообщения `compact_boundary`, и текст результата содержит причину, например `Not enough messages to compact.` после одного короткого обмена. Свежий одноразовый вызов `query()` начинается с пустого контекста, поэтому используйте этот паттерн в сеансе с предыдущими ходами, например в [режиме потокового ввода](/docs/ru/agent-sdk/streaming-vs-single-mode) или при возобновлении сеанса.
</Note>

<h3 id="reset-context-with-/clear">
  Сброс контекста с помощью `/clear`
</h3>

Команда `/clear` сбрасывает разговор в пустой контекст, поэтому последующие подсказки начинаются без предыдущей истории разговора. Предыдущий разговор остаётся на диске. Вы можете вернуться к этому разговору, передав его ID сеанса в [опцию `resume`](/docs/ru/agent-sdk/sessions#resume-by-id).

`/clear` полезна в [режиме потокового ввода](/docs/ru/agent-sdk/streaming-vs-single-mode), где вы отправляете несколько подсказок через одно соединение. Для одноразовых вызовов `query()` каждый вызов уже начинается с пустого контекста, поэтому отправка `/clear` не имеет практического эффекта. Вместо этого запустите новый `query()`.

<h2 id="create-skills">
  Создание skills
</h2>

Создайте каждый skill как каталог, содержащий файл `SKILL.md` с YAML frontmatter и содержимым Markdown. Поле `description` определяет, когда Claude вызывает ваш skill.

**Пример структуры каталога**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Выберите уровень обнаружения
</h3>

Сохраняйте skills на одном из двух наиболее распространённых [уровней обнаружения](/docs/ru/skills#where-skills-live):

* **Project skills**: `.claude/skills/`, доступны только в текущем проекте
* **Personal skills**: `~/.claude/skills/`, доступны во всех ваших проектах

Если у вас есть существующие файлы пользовательских команд в `.claude/commands/`, они продолжают работать. Файл команды в `.claude/commands/deploy.md` создаёт `/deploy` и работает так же, как skill в `.claude/skills/deploy/SKILL.md`. Если файл команды и skill имеют одно имя, см. [Разрешение skills, которые имеют одно имя](/docs/ru/skills#resolve-skills-that-share-a-name) для того, какой из них запускается. SDK загружает файлы `.claude/commands/` и `~/.claude/commands/` из тех же двух областей, что и skills. См. [Расширьте Claude с помощью skills](/docs/ru/skills) для полного руководства по обеим формам артефактов.

<h3 id="create-and-dispatch-your-first-skill">
  Создайте и отправьте ваш первый skill
</h3>

Чтобы увидеть полный поток, создайте `.claude/skills/security-check/SKILL.md`:

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

После создания файла skill доступен через SDK. Claude вызывает его, когда запрос соответствует его описанию, и вы можете отправить его напрямую:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Успешный запуск заканчивается результатом `success`, текст которого содержит результаты сканирования. Для небольшого приложения Express с внедрёнными проблемами текст результата начинается:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Имя skill также появляется в массиве `slash_commands` сообщения init.

<Note>
  Claude Code включает встроенные skills `code-review` и `verify`. Если вы назовёте файл `.claude/commands/` в честь одного из них, например `.claude/commands/code-review.md`, файл команды затеняет встроенный skill и `slash_commands` содержит имя один раз.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Предварительное одобрение инструментов для skills
</h2>

<Note>
  Для project и personal skills Claude Code применяет поле frontmatter [`allowed-tools`](/docs/ru/skills#pre-approve-tools-for-a-skill) в сеансах SDK. Вы также можете предварительно одобрить инструменты для этих skills через опцию `allowedTools` (`allowed_tools` в Python) в конфигурации вашего запроса. Skills [синхронизированные из claude.ai](/docs/ru/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) следуют своим собственным правилам frontmatter.
</Note>

Skills запускаются с инструментами сеанса. Пример ниже предварительно одобряет `Read`, `Grep` и `Glob` с `allowedTools` (`allowed_tools` в Python), поэтому Claude может проверять файлы при запуске [skill security-check](#create-and-dispatch-your-first-skill) без остановки для одобрения:

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

В потоке вызов skill появляется как использование инструмента Skill, за которым следуют вызовы Read на файлы проекта. Запуск заканчивается результатом `success`, текст которого содержит результаты.

Список предварительно одобряет названные инструменты, а не ограничивает остальные. Для полного потока разрешений, включая режимы разрешений и обратный вызов `canUseTool`, см. [Разрешения](/docs/ru/agent-sdk/permissions).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skills-not-found">
  Skills не найдены
</h3>

**Проверьте конфигурацию settingSources**: SDK обнаруживает skills через источники параметров `user` и `project`. Если вы явно установите `settingSources`/`setting_sources` и опустите эти источники, SDK не загружает skills:

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Для того, какие каталоги skills загружает каждый источник, см. [таблицу источников файловой системы](/docs/ru/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Для получения дополнительной информации о `settingSources`/`setting_sources` см. [справочник TypeScript SDK](/docs/ru/agent-sdk/typescript#settingsource) или [справочник Python SDK](/docs/ru/agent-sdk/python#settingsource).

**Проверьте рабочий каталог**: SDK загружает skills из `.claude/skills/` в опции `cwd` и в каждом родительском каталоге вплоть до корня репозитория. Убедитесь, что `cwd` указывает на каталог, содержащий `.claude/skills/`, или ниже него в пределах одного репозитория:

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

Полный паттерн см. в разделе [Использование skills с Agent SDK](#use-skills-with-the-agent-sdk).

**Проверьте расположение файловой системы**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill не используется
</h3>

**Проверьте опцию `skills`**: если вы передали список `skills`, подтвердите, что имя skill включено. Когда Claude пытается вызвать неуказанный skill, инструмент Skill возвращает `Skill <name> is not in this session's skills allowlist`. Добавьте имя в ваш список или отправьте skill напрямую, отправив `/<name>` в подсказку, что работает без указания.

**Проверьте описание**: убедитесь, что оно конкретно и включает соответствующие ключевые слова. См. [Agent Skills best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) для рекомендаций по написанию эффективных описаний.

<h3 id="invalid-skill-name-error">
  Ошибка неверного имени skill
</h3>

Когда имя в вашем списке `skills` не может работать как точное имя skill, `query()` отклоняет список перед запуском процесса Claude Code. Имена, которые вызывают отклонение, включают:

* Пустое имя
* Имя, содержащее скобки, запятые или управляющие символы
* Имя, дополненное пробелом
* Форма подстановочного знака, такая как голый `*` или суффикс `:*`

Каждый SDK выводит отклонение по-разному:

<Tabs>
  <Tab title="TypeScript">
    TypeScript SDK выдаёт `Error`, указывающую правило, которое нарушила запись. Например, `skills: ["docs:*"]` выдаёт:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Пустое имя сообщает `Skill names must be non-empty strings.`

    До TypeScript Agent SDK 0.3.221 SDK не выполнял эту проверку.
  </Tab>

  <Tab title="Python">
    Python SDK выдаёт `ValueError`, указывающую правило, которое нарушила запись. Например, `skills=["docs:*"]` выдаёт:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Пустое имя сообщает `Skill names must be non-empty strings`.

    До Python Agent SDK 0.2.129 SDK не выполнял эту проверку.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Дополнительное troubleshooting
</h3>

Для общего troubleshooting skills, такого как ошибки синтаксиса YAML и отладка, см. [раздел troubleshooting Claude Code skills](/docs/ru/skills#troubleshooting).

<h2 id="next-steps">
  Следующие шаги
</h2>

[Руководство Claude Code skills](/docs/ru/skills) охватывает разработку в глубину. Его рекомендации применяются к сеансам SDK. Начните с этих разделов:

* [Справочник Frontmatter](/docs/ru/skills#frontmatter-reference): каждое поддерживаемое поле
* [Передача аргументов в skills](/docs/ru/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1` и стекирование skills. [Полная таблица подстановок](/docs/ru/skills#available-string-substitutions) добавляет именованные аргументы и переменные `${CLAUDE_*}`
* [Внедрение динамического контекста](/docs/ru/skills#inject-dynamic-context): строки `` !`command` ``, которые запускаются перед тем, как Claude увидит содержимое skill
* [Выберите, где загружаются skills](/docs/ru/skills#where-skills-live): каждое расположение skill, пространство имён плагина и какой skill запускается, когда два имеют одно имя

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Команды в Claude Code](/docs/ru/commands): полная поверхность команд, включая каждую встроенную
* [Agent Skills overview](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): концептуальный обзор, преимущества и архитектура
* [Agent Skills best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): рекомендации по разработке для эффективных skills
* [Agent Skills cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): примеры skills и шаблоны
* [Subagents в SDK](/docs/ru/agent-sdk/subagents): похожие агенты на основе файловой системы с программными опциями
* [Обзор SDK](/docs/ru/agent-sdk/overview): общие концепции SDK
* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript): полная документация API
* [Справочник Python SDK](/docs/ru/agent-sdk/python): полная документация API
