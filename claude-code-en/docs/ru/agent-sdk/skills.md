> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills в SDK

> Расширьте Claude специализированными возможностями, используя Agent Skills в Claude Agent SDK

<h2 id="overview">
  Обзор
</h2>

Agent Skills расширяют Claude специализированными возможностями, которые Claude автономно вызывает при необходимости. Skills упаковываются в виде файлов `SKILL.md`, содержащих инструкции, описания и дополнительные вспомогательные ресурсы.

Для получения полной информации о Skills, включая преимущества, архитектуру и рекомендации по разработке, см. [обзор Agent Skills](https://platform.claude.com/docs/ru/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Как Skills работают с SDK
</h2>

При использовании Claude Agent SDK Skills:

1. **Определяются как артефакты файловой системы**: Создаются как файлы `SKILL.md` в определённых каталогах (`.claude/skills/`)
2. **Загружаются из файловой системы**: Skills загружаются из расположений файловой системы, управляемых `settingSources` (TypeScript) или `setting_sources` (Python)
3. **Автоматически обнаруживаются**: После загрузки параметров файловой системы метаданные Skill обнаруживаются при запуске из пользовательских и проектных каталогов; полное содержимое загружается при срабатывании
4. **Вызываются моделью**: Claude автономно выбирает, когда их использовать, на основе контекста
5. **Фильтруются через опцию `skills`**: Обнаруженные Skills включены по умолчанию. Передайте список имён Skills, `"all"` или `[]` для управления доступными в сеансе

В отличие от subagents (которые можно определить программно), Skills должны быть созданы как артефакты файловой системы. SDK не предоставляет программный API для регистрации Skills.

<Note>
  Skills обнаруживаются через источники параметров файловой системы. С параметрами `query()` по умолчанию SDK загружает пользовательские и проектные источники, поэтому Skills в `~/.claude/skills/`, `<cwd>/.claude/skills/` и `.claude/skills/` в любом родительском каталоге `<cwd>` вплоть до корня репозитория доступны. Если вы явно установите `settingSources`, включите `'user'` или `'project'` для сохранения обнаружения Skills, или используйте [опцию `plugins`](/docs/ru/agent-sdk/plugins) для загрузки Skills из определённого пути.
</Note>

<h2 id="using-skills-with-the-sdk">
  Использование Skills с SDK
</h2>

Установите опцию `skills` на `query()` для управления тем, какие Skills доступны для сеанса. Если опция опущена, обнаруженные Skills включены и инструмент Skill доступен, что соответствует поведению CLI. Передайте `"all"` для включения каждого обнаруженного Skill, список имён Skill для включения только тех или `[]` для отключения всех. Когда вы устанавливаете `skills`, SDK автоматически добавляет инструмент Skill в `allowedTools`. Если вы также передаёте явный список `tools`, включите `"Skill"` в этот список, чтобы Claude мог вызывать skills.

После настройки Claude автоматически обнаруживает Skills из файловой системы и вызывает их при необходимости для запроса пользователя.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
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
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Для включения только определённых Skills передайте их имена. Имена соответствуют полю `name` в `SKILL.md` или имени каталога Skill. Используйте `plugin:skill` для Skills, предоставляемых плагинами.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

Опция `skills` является фильтром контекста, а не песочницей. Неуказанные Skills скрыты от модели и отклонены инструментом Skill, но их файлы остаются на диске и доступны через Read и Bash.

<h2 id="skill-locations">
  Расположение Skills
</h2>

Skills загружаются из каталогов файловой системы на основе конфигурации `settingSources`/`setting_sources`:

* **Project Skills** (`.claude/skills/`): Общие с вашей командой через git - загружаются, когда `setting_sources` включает `"project"`
* **User Skills** (`~/.claude/skills/`): Личные Skills для всех проектов - загружаются, когда `setting_sources` включает `"user"`
* **Plugin Skills**: Поставляются с установленными Claude Code плагинами

<h2 id="creating-skills">
  Создание Skills
</h2>

Skills определяются как каталоги, содержащие файл `SKILL.md` с YAML frontmatter и содержимым Markdown. Поле `description` определяет, когда Claude вызывает ваш Skill.

**Пример структуры каталога**:

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Для полного руководства по созданию Skills, включая структуру SKILL.md, многофайловые Skills и примеры, см.:

* [Agent Skills в Claude Code](/docs/ru/skills): Полное руководство с примерами
* [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): Рекомендации по разработке и соглашения об именовании

<h2 id="tool-restrictions">
  Ограничения инструментов
</h2>

<Note>
  Поле frontmatter `allowed-tools` в SKILL.md поддерживается только при прямом использовании Claude Code CLI. **Оно не применяется при использовании Skills через SDK**.

  При использовании SDK управляйте доступом к инструментам через основную опцию `allowedTools` в конфигурации запроса.
</Note>

Для управления доступом к инструментам для Skills в приложениях SDK используйте `allowedTools` для предварительного одобрения определённых инструментов. Без обратного вызова `canUseTool` всё, что не в списке, отклоняется:

<Note>
  Предполагается, что операторы импорта из первого примера используются в следующих фрагментах кода.
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  Обнаружение доступных Skills
</h2>

Чтобы узнать, какие Skills доступны в вашем приложении SDK, просто спросите Claude:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude выведет список доступных Skills на основе вашего текущего рабочего каталога и установленных плагинов.

<h2 id="testing-skills">
  Тестирование Skills
</h2>

Тестируйте Skills, задавая вопросы, которые соответствуют их описаниям:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude автоматически вызывает соответствующий Skill, если описание соответствует вашему запросу.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skills-not-found">
  Skills не найдены
</h3>

**Проверьте конфигурацию settingSources**: Skills обнаруживаются через источники параметров `user` и `project`. Если вы явно установите `settingSources`/`setting_sources` и опустите эти источники, Skills не загружаются:

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
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Для получения дополнительной информации о `settingSources`/`setting_sources` см. [справочник TypeScript SDK](/docs/ru/agent-sdk/typescript#settingsource) или [справочник Python SDK](/docs/ru/agent-sdk/python#settingsource).

**Проверьте рабочий каталог**: SDK загружает Skills из `.claude/skills/` в опции `cwd` и в каждом родительском каталоге вплоть до корня репозитория. Убедитесь, что `cwd` указывает на каталог, содержащий `.claude/skills/`, или ниже него в пределах одного репозитория:

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

Полный паттерн см. в разделе "Использование Skills с SDK" выше.

**Проверьте расположение файловой системы**:

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill не используется
</h3>

**Проверьте опцию `skills`**: Если вы передали список `skills`, подтвердите, что имя Skill включено. Передача `[]` отключает все Skills.

**Проверьте описание**: Убедитесь, что оно конкретно и включает соответствующие ключевые слова. Рекомендации по написанию эффективных описаний см. в [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions).

<h3 id="additional-troubleshooting">
  Дополнительное Troubleshooting
</h3>

Для общего Troubleshooting Skills (синтаксис YAML, отладка и т. д.) см. [раздел Troubleshooting Claude Code Skills](/docs/ru/skills#troubleshooting).

<h2 id="related-documentation">
  Связанная документация
</h2>

<h3 id="skills-guides">
  Руководства Skills
</h3>

* [Agent Skills в Claude Code](/docs/ru/skills): Полное руководство Skills с созданием, примерами и Troubleshooting
* [Agent Skills Overview](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): Концептуальный обзор, преимущества и архитектура
* [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): Рекомендации по разработке эффективных Skills
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Примеры Skills и шаблоны

<h3 id="sdk-resources">
  Ресурсы SDK
</h3>

* [Subagents в SDK](/docs/ru/agent-sdk/subagents): Похожие агенты на основе файловой системы с программными опциями
* [Slash Commands в SDK](/docs/ru/agent-sdk/slash-commands): Команды, вызываемые пользователем
* [Обзор SDK](/docs/ru/agent-sdk/overview): Общие концепции SDK
* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript): Полная документация API
* [Справочник Python SDK](/docs/ru/agent-sdk/python): Полная документация API
