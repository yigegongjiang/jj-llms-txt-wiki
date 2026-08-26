> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Использование функций Claude Code в SDK

> Загружайте инструкции проекта, skills, hooks и другие функции Claude Code в ваши SDK-агентов.

Agent SDK построен на той же основе, что и Claude Code, что означает, что ваши SDK-агенты имеют доступ к тем же функциям на основе файловой системы: инструкции проекта (`CLAUDE.md` и правила), skills, hooks и многое другое.

Когда вы опускаете `settingSources`, `query()` читает те же параметры файловой системы, что и Claude Code CLI: пользовательские, проектные и локальные параметры, файлы `CLAUDE.md` и skills, агенты и команды в `.claude/`. Чтобы запустить без них, передайте `settingSources: []`, что ограничивает агента только тем, что вы настраиваете программно. Параметры управляемой политики и глобальная конфигурация `~/.claude.json` читаются независимо от этого параметра. См. [Что settingSources не контролирует](#what-settingsources-does-not-control).

Для концептуального обзора того, что делает каждая функция и когда её использовать, см. [Расширение Claude Code](/docs/ru/features-overview).

<h2 id="control-filesystem-settings-with-settingsources">
  Управление параметрами файловой системы с помощью settingSources
</h2>

Параметр источников параметров ([`setting_sources`](/docs/ru/agent-sdk/python#claudeagentoptions) в Python, [`settingSources`](/docs/ru/agent-sdk/typescript#settingsource) в TypeScript) контролирует, какие параметры на основе файловой системы загружает SDK. Передайте явный список для включения определённых источников или передайте пустой массив для отключения пользовательских, проектных и локальных параметров.

Этот пример загружает как пользовательские, так и проектные параметры, устанавливая `settingSources` на `["user", "project"]`:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

  async for message in query(
      prompt="Help me refactor the auth module",
      options=ClaudeAgentOptions(
          # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
          # Together they give the agent access to CLAUDE.md, skills, hooks, and
          # permissions from both locations.
          setting_sources=["user", "project"],
          allowed_tools=["Read", "Edit", "Bash"],
      ),
  ):
      if isinstance(message, AssistantMessage):
          for block in message.content:
              if hasattr(block, "text"):
                  print(block.text)
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(f"\nResult: {message.result}")
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

Каждый источник загружает параметры из определённого местоположения, где `<cwd>` — это рабочий каталог, который вы передаёте через параметр `cwd`, или текущий каталог процесса, если он не установлен. Для полного определения типа см. [`SettingSource`](/docs/ru/agent-sdk/typescript#settingsource) (TypeScript) или [`SettingSource`](/docs/ru/agent-sdk/python#settingsource) (Python).

| Источник    | Что он загружает                                                                                | Местоположение                                                                                                                                                                         |
| :---------- | :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | Project CLAUDE.md, `.claude/rules/*.md`, project skills, project hooks, project `settings.json` | `<cwd>/.claude/` для `settings.json` и hooks; `<cwd>` и каждый родительский каталог для CLAUDE.md и rules; `<cwd>` и каждый родительский каталог вверх до корня репозитория для skills |
| `"user"`    | User CLAUDE.md, `~/.claude/rules/*.md`, user skills, user settings                              | `~/.claude/`                                                                                                                                                                           |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                                  | `<cwd>/.claude/` для `settings.local.json`; `<cwd>` и каждый родительский каталог для CLAUDE.local.md                                                                                  |

Опускание `settingSources` эквивалентно `["user", "project", "local"]`.

Параметр `cwd` определяет, где SDK ищет входные данные уровня проекта. CLAUDE.md и rules загружаются из `<cwd>` и из каждого родительского каталога. Skills загружаются из `<cwd>` и из каждого родительского каталога вверх до корня репозитория. Project `settings.json` и hooks загружаются только из `<cwd>/.claude/` без резервного варианта для родительского каталога.

<h3 id="what-settingsources-does-not-control">
  Что settingSources не контролирует
</h3>

`settingSources` охватывает пользовательские, проектные и локальные параметры. Несколько входов читаются независимо от его значения:

| Вход                                                               | Поведение                                                                                                                                                                                                                                                                                                                                                                                    | Для отключения                                                                                                                                                                                                      |
| :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Параметры управляемой политики                                     | Политика, управляемая конечной точкой, будь то MDM plist, политика реестра или файлы управляемых параметров, загружается с хоста. [Параметры, управляемые сервером](/docs/ru/server-managed-settings) загружаются при аутентификации сеанса с помощью входа OAuth организации или напрямую настроенного ключа API на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability) | Политика конечной точки: удалите файл управляемых параметров, plist или политику реестра с хоста. Параметры, управляемые сервером: контролируются администратором вашей организации; не могут быть отключены из SDK |
| `~/.claude.json` глобальная конфигурация                           | Всегда читается                                                                                                                                                                                                                                                                                                                                                                              | Переместите с помощью `CLAUDE_CONFIG_DIR` в `env`                                                                                                                                                                   |
| Автоматическая память в `~/.claude/projects/<project>/memory/`     | Загружается в системный запрос при запуске сеанса. Агент записывает новые воспоминания туда с помощью стандартных инструментов `Write` и `Edit` вместо специального инструмента памяти, поэтому эти инструменты должны быть включены, чтобы агент мог сохранять воспоминания                                                                                                                 | Установите `autoMemoryEnabled: false` в параметрах или `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` в `env`                                                                                                                  |
| [claude.ai MCP connectors](/docs/ru/mcp#use-mcp-servers-from-claude-ai) | Загружаются, когда активный метод аутентификации — это подписка claude.ai. Передача `mcpServers: {}` их не подавляет                                                                                                                                                                                                                                                                         | Установите `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/ru/mcp#disable-claude-ai-connectors) в параметрах или `ENABLE_CLAUDEAI_MCP_SERVERS=false` в `env`                                          |

<Warning>
  Не полагайтесь на параметры `query()` по умолчанию для изоляции в многопользовательской среде. Поскольку входы выше читаются независимо от `settingSources`, процесс SDK может подхватить конфигурацию уровня хоста и память для каждого каталога. Для развёртываний в многопользовательской среде запустите каждого пользователя в собственной файловой системе и установите `settingSources: []` плюс `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` в `env`. [Параметры, управляемые сервером](/docs/ru/server-managed-settings), загружаются при аутентификации процесса с помощью учётных данных организации; изоляция файловой системы их не удаляет. См. [Безопасное развёртывание](/docs/ru/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Инструкции проекта (CLAUDE.md и правила)
</h2>

Файлы `CLAUDE.md` и файлы `.claude/rules/*.md` дают вашему агенту постоянный контекст о вашем проекте: соглашения кодирования, команды сборки, архитектурные решения и инструкции. Когда `settingSources` включает `"project"` (как в примере выше), SDK загружает эти файлы в контекст при запуске сеанса. Затем агент следует вашим соглашениям проекта без необходимости повторять их в каждом запросе.

<h3 id="claude-md-load-locations">
  Местоположения загрузки CLAUDE.md
</h3>

| Уровень               | Местоположение                                                                   | Когда загружается                                                                                          |
| :-------------------- | :------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| Project (root)        | `<cwd>/CLAUDE.md` или `<cwd>/.claude/CLAUDE.md`                                  | `settingSources` включает `"project"`                                                                      |
| Project rules         | `<cwd>/.claude/rules/*.md` и `.claude/rules/*.md` в каждом родительском каталоге | `settingSources` включает `"project"`                                                                      |
| Project (parent dirs) | Файлы `CLAUDE.md` в каталогах выше `cwd`                                         | `settingSources` включает `"project"`, загружается при запуске сеанса                                      |
| Project (child dirs)  | Файлы `CLAUDE.md` в подкаталогах `cwd`                                           | `settingSources` включает `"project"`, загружается по требованию, когда агент читает файл в этом поддереве |
| Local                 | `<cwd>/CLAUDE.local.md` и `CLAUDE.local.md` в каждом родительском каталоге       | `settingSources` включает `"local"`                                                                        |
| User                  | `~/.claude/CLAUDE.md`                                                            | `settingSources` включает `"user"`                                                                         |
| User rules            | `~/.claude/rules/*.md`                                                           | `settingSources` включает `"user"`                                                                         |

Все уровни являются аддитивными: если существуют как проектные, так и пользовательские файлы `CLAUDE.md`, агент видит оба. Между уровнями нет жёсткого правила приоритета; если инструкции конфликтуют, результат зависит от того, как Claude их интерпретирует. Напишите неконфликтующие правила или явно укажите приоритет в более специфичном файле («Эти инструкции проекта переопределяют любые конфликтующие пользовательские значения по умолчанию»).

<Tip>
  Вы также можете внедрить контекст непосредственно через `systemPrompt` без использования файлов `CLAUDE.md`. См. [Изменение системных запросов](/docs/ru/agent-sdk/modifying-system-prompts). Используйте `CLAUDE.md`, когда вы хотите, чтобы один и тот же контекст был общим между интерактивными сеансами Claude Code и вашими SDK-агентами.
</Tip>

О том, как структурировать и организовать содержимое `CLAUDE.md`, см. [Управление памятью Claude](/docs/ru/memory).

<h2 id="skills">
  Skills
</h2>

Skills — это файлы markdown, которые дают вашему агенту специализированные знания и вызываемые рабочие процессы. В отличие от `CLAUDE.md` (который загружается каждый сеанс), skills загружаются по требованию. Агент получает описания skills при запуске и загружает полное содержимое при необходимости.

Skills обнаруживаются из файловой системы через `settingSources`. Когда параметр `skills` в `query()` опущен, обнаруженные пользовательские и проектные skills включены и инструмент Skill доступен, что соответствует поведению CLI. Для управления тем, какие skills включены, передайте `skills` как `"all"`, список имён skills или `[]` для отключения всех. Когда `skills` установлен, SDK автоматически добавляет инструмент Skill в `allowedTools`. Если вы также передаёте явный список `tools`, включите `"Skill"` в этот список, чтобы Claude мог вызывать skills.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async for message in query(
      prompt="Review this PR using our code review checklist",
      options=ClaudeAgentOptions(
          setting_sources=["user", "project"],
          skills="all",
          allowed_tools=["Read", "Grep", "Glob"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Skills должны быть созданы как артефакты файловой системы (`.claude/skills/<name>/SKILL.md`). SDK не имеет программного API для регистрации skills. См. [Agent Skills в SDK](/docs/ru/agent-sdk/skills) для полных деталей.
</Note>

Дополнительную информацию о создании и использовании skills см. в разделе [Agent Skills в SDK](/docs/ru/agent-sdk/skills).

<h2 id="hooks">
  Hooks
</h2>

SDK поддерживает два способа определения hooks, и они работают рядом:

* **Filesystem hooks:** команды оболочки, определённые в `settings.json`, загружаются, когда `settingSources` включает соответствующий источник. Это те же hooks, которые вы бы настроили для [интерактивных сеансов Claude Code](/docs/ru/hooks-guide).
* **Programmatic hooks:** функции обратного вызова, передаваемые непосредственно в `query()`. Они выполняются в процессе вашего приложения и могут возвращать структурированные решения. См. [Управление выполнением с помощью hooks](/docs/ru/agent-sdk/hooks).

Оба типа выполняются во время одного и того же жизненного цикла hook. Если у вас уже есть hooks в файле `.claude/settings.json` вашего проекта и вы установили `settingSources: ["project"]`, эти hooks автоматически запускаются в SDK без дополнительной конфигурации.

Обратные вызовы Hook получают входные данные инструмента и возвращают словарь решения. Возврат `{}` означает разрешить инструменту продолжить. Чтобы заблокировать выполнение, верните объект `hookSpecificOutput` с `permissionDecision: "deny"` и `permissionDecisionReason`. Причина отправляется Claude как результат инструмента. Поля верхнего уровня `decision` и `reason` устарели для `PreToolUse`. См. [руководство hooks](/docs/ru/agent-sdk/hooks) для полной сигнатуры обратного вызова и типов возврата.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, carries session metadata
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async for message in query(
      prompt="Refactor the auth module",
      options=ClaudeAgentOptions(
          setting_sources=["project"],  # Loads hooks from .claude/settings.json
          hooks={
              "PreToolUse": [
                  HookMatcher(matcher="Bash", hooks=[audit_bash]),
              ]
          },
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  Когда использовать какой тип hook
</h3>

| Тип hook                                       | Лучше всего для                                                                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Filesystem** (`settings.json`)               | Совместное использование hooks между сеансами CLI и SDK. Поддерживает `"command"` (shell-скрипты), `"http"` (POST на конечную точку), `"mcp_tool"` (вызов инструмента подключённого MCP-сервера), `"prompt"` (LLM оценивает запрос) и `"agent"` (порождает агента-верификатора). Они срабатывают в основном агенте и любых подагентах, которые он порождает. |
| **Programmatic** (обратные вызовы в `query()`) | Логика, специфичная для приложения, структурированные решения и интеграция в процессе. Эти обратные вызовы также срабатывают внутри подагентов. Обратный вызов получает `agent_id` и `agent_type` для различения.                                                                                                                                            |

<Note>
  TypeScript SDK поддерживает дополнительные события hook помимо Python, включая `SessionStart`, `SessionEnd`, `TeammateIdle` и `TaskCompleted`. См. [руководство hooks](/docs/ru/agent-sdk/hooks) для полной таблицы совместимости событий.
</Note>

Для полных деталей о programmatic hooks см. [Управление выполнением с помощью hooks](/docs/ru/agent-sdk/hooks). Для синтаксиса filesystem hook см. [Hooks](/docs/ru/hooks).

<h2 id="choose-the-right-feature">
  Выбор правильной функции
</h2>

Agent SDK предоставляет вам доступ к нескольким способам расширения поведения вашего агента. Если вы не уверены, какой использовать, эта таблица отображает общие цели на правильный подход.

| Вы хотите...                                                                                                         | Используйте                                   | Поверхность SDK                                                                                                                                                               |
| :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Установить соглашения проекта, которые ваш агент всегда соблюдает                                                    | [CLAUDE.md](/docs/ru/memory)                       | `settingSources: ["project"]` загружает его автоматически                                                                                                                     |
| Дать агенту справочный материал, который он загружает при необходимости                                              | [Skills](/docs/ru/agent-sdk/skills)                | `settingSources` + `skills` опция                                                                                                                                             |
| Запустить повторно используемый рабочий процесс (развёртывание, проверка, выпуск)                                    | [User-invocable skills](/docs/ru/agent-sdk/skills) | `settingSources` + `skills` опция                                                                                                                                             |
| Делегировать изолированную подзадачу свежему контексту (исследование, проверка)                                      | [Subagents](/docs/ru/agent-sdk/subagents)          | параметр `agents` + `allowedTools: ["Agent"]`                                                                                                                                 |
| Координировать несколько экземпляров Claude Code с общими списками задач и прямой передачей сообщений между агентами | [Agent teams](/docs/ru/agent-teams)                | Не настраивается напрямую через параметры SDK. Agent teams — это функция CLI, где один сеанс действует как лидер команды, координируя работу независимых товарищей по команде |
| Запустить детерминированную логику на вызовах инструментов (аудит, блокировка, преобразование)                       | [Hooks](/docs/ru/agent-sdk/hooks)                  | параметр `hooks` с обратными вызовами или shell-скрипты, загруженные через `settingSources`                                                                                   |
| Дать Claude структурированный доступ к инструменту для внешнего сервиса                                              | [MCP](/docs/ru/agent-sdk/mcp)                      | параметр `mcpServers`                                                                                                                                                         |

<Tip>
  **Subagents против agent teams:** Subagents являются эфемерными и изолированными: свежий разговор, одна задача, резюме возвращается родителю. Agent teams координируют несколько независимых экземпляров Claude Code, которые совместно используют список задач и обмениваются сообщениями напрямую. Agent teams — это функция CLI. См. [Что наследуют subagents](/docs/ru/agent-sdk/subagents#what-subagents-inherit) и [сравнение agent teams](/docs/ru/agent-teams#compare-with-subagents) для деталей.
</Tip>

Каждая функция, которую вы включаете, добавляет к контекстному окну вашего агента. Для затрат на функцию и того, как эти функции слоятся вместе, см. [Расширение Claude Code](/docs/ru/features-overview#understand-context-costs).

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Расширение Claude Code](/docs/ru/features-overview): Концептуальный обзор всех функций расширения с таблицами сравнения и анализом затрат контекста
* [Skills в SDK](/docs/ru/agent-sdk/skills): Полное руководство по использованию skills программно
* [Subagents](/docs/ru/agent-sdk/subagents): Определение и вызов subagents для изолированных подзадач
* [Hooks](/docs/ru/agent-sdk/hooks): Перехват и управление поведением агента в ключевых точках выполнения
* [Permissions](/docs/ru/agent-sdk/permissions): Управление доступом к инструментам с помощью режимов, правил и обратных вызовов
* [System prompts](/docs/ru/agent-sdk/modifying-system-prompts): Внедрение контекста без файлов CLAUDE.md
