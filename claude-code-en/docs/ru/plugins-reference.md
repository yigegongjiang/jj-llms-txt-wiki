> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник по плагинам

> Полный технический справочник по системе плагинов Claude Code, включая схемы, команды CLI и спецификации компонентов.

<Tip>
  Ищете способ установить плагины? Смотрите [Обнаружение и установка плагинов](/docs/ru/discover-plugins). Для создания плагинов смотрите [Плагины](/docs/ru/plugins). Для распространения плагинов смотрите [Маркетплейсы плагинов](/docs/ru/plugin-marketplaces).
</Tip>

Этот справочник содержит полные технические спецификации для системы плагинов Claude Code, включая схемы компонентов, команды CLI и инструменты разработки.

**Плагин** — это самостоятельный каталог компонентов, который расширяет Claude Code пользовательской функциональностью. Компоненты плагина включают skills, agents, hooks, MCP servers, LSP servers и monitors.

<h2 id="plugin-components-reference">
  Справочник компонентов плагина
</h2>

<h3 id="skills">
  Skills
</h3>

Плагины добавляют skills в Claude Code, создавая сочетания клавиш `/name`, которые вы или Claude можете вызвать.

**Расположение**: каталог `skills/` или `commands/` в корне плагина, или один файл `SKILL.md` в корне плагина

**Формат файла**: Skills — это каталоги с `SKILL.md`; команды — это простые файлы markdown

**Структура skill**:

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (опционально)
│   └── scripts/ (опционально)
└── code-reviewer/
    └── SKILL.md
```

**Поведение интеграции**:

* Skills и команды автоматически обнаруживаются при установке плагина
* Claude может вызывать их автоматически на основе контекста задачи
* Skills могут включать вспомогательные файлы рядом с SKILL.md

Если плагин не имеет каталога `skills/` и не имеет поля манифеста `skills`, то `SKILL.md` в корне плагина загружается как один skill. Установите поле frontmatter `name` для управления именем вызова skill. Без него Claude Code возвращается к имени каталога установки, которое для плагинов, установленных из маркетплейса, является строкой версии, которая меняется при каждом обновлении. Для плагинов, которые поставляют более одного skill, используйте макет каталога `skills/`, показанный выше.

Для полной информации смотрите [Skills](/docs/ru/skills).

<h3 id="agents">
  Agents
</h3>

Плагины могут предоставлять специализированные subagents для конкретных задач, которые Claude может вызывать автоматически при необходимости.

**Расположение**: каталог `agents/` в корне плагина

**Формат файла**: Файлы markdown, описывающие возможности агента

**Структура агента**:

```markdown theme={null}
---
name: agent-name
description: Что специализирует этот агент и когда Claude должен его вызвать
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

Подробное системное приглашение для агента, описывающее его роль, опыт и поведение.
```

Плагины agents поддерживают поля frontmatter `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background` и `isolation`. Единственное допустимое значение `isolation` — это `"worktree"`. По соображениям безопасности `hooks`, `mcpServers` и `permissionMode` не поддерживаются для agents, поставляемых с плагинами.

**Точки интеграции**:

* Агенты появляются в интерфейсе [@-mention typeahead](/docs/ru/sub-agents#invoke-subagents-explicitly) под их областью видимости, такой как `my-plugin:code-reviewer`, после включения плагина
* Claude может вызывать агентов автоматически на основе контекста задачи
* Агенты могут быть вызваны вручную пользователями
* Плагины agents работают наряду со встроенными agents Claude

Для полной информации смотрите [Subagents](/docs/ru/sub-agents).

<h3 id="hooks">
  Hooks
</h3>

Плагины могут предоставлять обработчики событий, которые автоматически реагируют на события Claude Code.

**Расположение**: `hooks/hooks.json` в корне плагина или встроенный в plugin.json

**Формат**: Конфигурация JSON с сопоставителями событий и действиями

**Конфигурация hook**:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

Плагины hooks реагируют на те же события жизненного цикла, что и [определённые пользователем hooks](/docs/ru/hooks):

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Типы hook**:

* `command`: выполнение команд оболочки или скриптов
* `http`: отправка JSON события как POST запроса на URL
* `mcp_tool`: вызов инструмента на настроенном [MCP server](/docs/ru/mcp)
* `prompt`: оценка приглашения с помощью LLM (использует заполнитель `$ARGUMENTS` для контекста)
* `agent`: запуск проверки агента с инструментами для сложных задач проверки

Hooks, которые нацелены на собственный [bundled MCP server](#mcp-servers) плагина, должны использовать его scoped names. Сопоставители инструментов и поля `if` принимают scoped tool name `mcp__plugin_<plugin-name>_<server-name>__<tool>`, а поле `server` hook `mcp_tool` принимает `plugin:<plugin-name>:<server-name>`. Сопоставитель, написанный для простого ключа сервера, никогда не срабатывает. Смотрите [Match MCP tools](/docs/ru/hooks#match-mcp-tools) и [Plugin-provided MCP servers](/docs/ru/mcp#plugin-provided-mcp-servers).

<h3 id="mcp-servers">
  MCP servers
</h3>

Плагины могут включать серверы Model Context Protocol (MCP) для подключения Claude Code к внешним инструментам и сервисам.

**Расположение**: `.mcp.json` в корне плагина или встроенный в plugin.json

**Формат**: Стандартная конфигурация сервера MCP

**Конфигурация сервера MCP**:

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**Поведение интеграции**:

* Серверы MCP плагина запускаются автоматически при включении плагина
* Серверы отображаются как стандартные инструменты MCP в наборе инструментов Claude
* Возможности сервера беспрепятственно интегрируются с существующими инструментами Claude
* Серверы плагина можно настраивать независимо от серверов MCP пользователя

<h3 id="lsp-servers">
  LSP servers
</h3>

<Tip>
  Ищете способ использовать плагины LSP? Установите их из официального маркетплейса: найдите "lsp" на вкладке Discover в `/plugin`. Этот раздел документирует, как создавать плагины LSP для языков, не охватываемых официальным маркетплейсом.
</Tip>

Плагины могут предоставлять серверы [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) для предоставления Claude интеллектуальной информации о коде в реальном времени при работе с вашей кодовой базой.

Интеграция LSP предоставляет:

* **Мгновенная диагностика**: Claude видит ошибки и предупреждения сразу после каждого редактирования
* **Навигация по коду**: переход к определению, поиск ссылок и информация при наведении
* **Осведомлённость о языке**: информация о типах и документация для символов кода

**Расположение**: `.lsp.json` в корне плагина или встроенный в `plugin.json`

**Формат**: Конфигурация JSON, сопоставляющая имена языковых серверов с их конфигурациями

**Формат файла `.lsp.json`**:

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**Встроенный в `plugin.json`**:

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**Обязательные поля:**

| Поле                  | Описание                                                 |
| :-------------------- | :------------------------------------------------------- |
| `command`             | Двоичный файл LSP для выполнения (должен быть в PATH)    |
| `extensionToLanguage` | Сопоставляет расширения файлов с идентификаторами языков |

**Опциональные поля:**

| Поле                    | Описание                                                                                                                                                                                                          |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `args`                  | Аргументы командной строки для сервера LSP                                                                                                                                                                        |
| `transport`             | Транспорт связи: `stdio` (по умолчанию) или `socket`                                                                                                                                                              |
| `env`                   | Переменные окружения для установки при запуске сервера                                                                                                                                                            |
| `initializationOptions` | Опции, передаваемые серверу при инициализации                                                                                                                                                                     |
| `settings`              | Параметры, передаваемые через `workspace/didChangeConfiguration`                                                                                                                                                  |
| `workspaceFolder`       | Путь папки рабочей области для сервера                                                                                                                                                                            |
| `startupTimeout`        | Максимальное время ожидания запуска сервера (миллисекунды)                                                                                                                                                        |
| `shutdownTimeout`       | Максимальное время ожидания корректного завершения (миллисекунды). Когда истекает время ожидания, Claude Code завершает процесс сервера. Если не установлено, время ожидания не применяется                       |
| `restartOnCrash`        | Следует ли перезапускать сервер после его сбоя. По умолчанию `true`. Установите значение `false`, чтобы оставить сбойный сервер остановленным вместо его перезапуска                                              |
| `maxRestarts`           | Максимальное количество попыток перезапуска перед отказом                                                                                                                                                         |
| `diagnostics`           | Следует ли отправлять диагностику в контекст Claude после редактирования (по умолчанию `true`). Установите значение `false`, чтобы сохранить навигацию по коду, но подавить автоматическое внедрение диагностики. |

`restartOnCrash` и `shutdownTimeout` требуют Claude Code v2.1.205 или позже. До v2.1.205 схема конфигурации принимала обе опции, но установка любой из них заставляла Claude Code пропустить этот сервер LSP полностью при запуске, причём причина видна только в выводе `claude --debug`.

**Несколько серверов для одного расширения**: когда более одного включённого сервера LSP объявляет одно и то же расширение файла в `extensionToLanguage`, независимо от того, поступают ли серверы из одного плагина или из разных плагинов, первый зарегистрированный сервер обрабатывает файлы с этим расширением, а остальные никогда не запускаются. Интерфейс `/plugin` показывает предупреждение, называющее плагин, чей сервер активен.

**Серверы, которые не инициализируются**: Claude Code пропускает сервер, конфигурация которого недействительна, например один, в котором отсутствует `command` или `extensionToLanguage`, и остальные настроенные серверы всё ещё запускаются. Запустите `claude --debug`, чтобы увидеть, почему сервер был пропущен.

Пропущенный сервер не заявляет свои расширения файлов, поэтому другой действительный сервер, который объявляет то же расширение, из того же или другого плагина, всё ещё обрабатывает эти файлы. До v2.1.205 сервер, который не инициализировался, всё ещё заявлял свои расширения и блокировал другой действительный сервер для того же расширения.

<Warning>
  **Вы должны установить двоичный файл языкового сервера отдельно.** Плагины LSP настраивают способ подключения Claude Code к языковому серверу, но они не включают сам сервер. Если вы видите `Executable not found in $PATH` на вкладке Errors в `/plugin`, установите требуемый двоичный файл для вашего языка.
</Warning>

**Доступные плагины LSP:**

| Плагин              | Языковой сервер            | Команда установки                                                                            |
| :------------------ | :------------------------- | :------------------------------------------------------------------------------------------- |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` или `npm install -g pyright`                                           |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                                       |
| `rust-analyzer-lsp` | rust-analyzer              | [Смотрите установку rust-analyzer](https://rust-analyzer.github.io/manual.html#installation) |

Сначала установите языковой сервер, затем установите плагин из маркетплейса.

<h3 id="monitors">
  Monitors
</h3>

Плагины могут объявлять фоновые monitors, которые Claude Code автоматически запускает при активации плагина. Каждый monitor запускает команду оболочки на протяжении всего сеанса и доставляет каждую строку stdout Claude как уведомление, чтобы Claude мог реагировать на записи журнала, изменения статуса или опрашиваемые события без необходимости просить запустить наблюдение.

Плагины monitors используют тот же механизм, что и [инструмент Monitor](/docs/ru/tools-reference#monitor-tool), и разделяют его ограничения доступности. Они работают только в интерактивных сеансах CLI, работают без песочницы на том же уровне доверия, что и [hooks](#hooks), и пропускаются на хостах, где инструмент Monitor недоступен.

**Расположение**: `monitors/monitors.json` в корне плагина или встроенный в `plugin.json`

**Формат**: Массив JSON записей monitor

Следующий `monitors/monitors.json` отслеживает конечную точку статуса развёртывания и локальный журнал ошибок:

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "Deployment status changes"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log",
    "when": "on-skill-invoke:debug"
  }
]
```

Для объявления monitors встроенным образом установите `experimental.monitors` в `plugin.json` на тот же массив. Для загрузки из пути, отличного от пути по умолчанию, установите `experimental.monitors` на строку относительного пути, такую как `"./config/monitors.json"`. Monitors — это [экспериментальный компонент](#experimental-components).

**Обязательные поля:**

| Поле          | Описание                                                                                                                               |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Идентификатор, уникальный в пределах плагина. Предотвращает дублирование процессов при перезагрузке плагина или повторном вызове skill |
| `command`     | Команда оболочки, запускаемая как постоянный фоновый процесс в рабочем каталоге сеанса                                                 |
| `description` | Краткое резюме того, что отслеживается. Показывается в панели задач и в резюме уведомлений                                             |

**Опциональные поля:**

| Поле   | Описание                                                                                                                                                                                                                                                             |
| :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `when` | Управляет тем, когда запускается monitor. `"always"` запускает его при запуске сеанса и при перезагрузке плагина и является значением по умолчанию. `"on-skill-invoke:<skill-name>"` запускает его в первый раз, когда именованный skill в этом плагине отправляется |

Значение `command` поддерживает [подстановки переменных](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` и `${CLAUDE_PROJECT_DIR}`, плюс любой `${ENV_VAR}` из окружения. Добавьте префикс команды с `cd "${CLAUDE_PLUGIN_ROOT}" && `, если скрипт должен работать из собственного каталога плагина.

Команда monitor не может ссылаться на значения [`${user_config.*}`](#user-configuration). Команда работает через оболочку, поэтому Claude Code отклоняет monitor с [ошибкой](/docs/ru/errors#plugin-command-references-user-config) вместо подстановки значения. Процессы monitor не получают переменные окружения `CLAUDE_PLUGIN_OPTION_<KEY>`, поэтому пусть скрипт monitor читает значение из файла конфигурации, который он владеет. До v2.1.207 команды monitor подставляли значения `${user_config.*}`.

Отключение плагина в середине сеанса не останавливает monitors, которые уже работают. Они останавливаются при завершении сеанса.

<h3 id="themes">
  Themes
</h3>

Плагины могут поставлять цветовые темы, которые появляются в `/theme` наряду со встроенными предустановками и локальными темами пользователя. Тема — это JSON файл в `themes/` с предустановкой `base` и разреженной картой переопределений `overrides` цветовых токенов. Themes — это [экспериментальный компонент](#experimental-components).

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Выбор темы плагина сохраняет `custom:<plugin-name>:<slug>` в конфигурации пользователя. Темы плагина доступны только для чтения; нажатие `Ctrl+E` на одной из них в `/theme` копирует её в `~/.claude/themes/`, чтобы пользователь мог редактировать копию.

***

<h2 id="plugin-installation-scopes">
  Области установки плагина
</h2>

При установке плагина вы выбираете **область**, которая определяет, где плагин доступен и кто ещё может его использовать:

| Область   | Файл параметров                                      | Вариант использования                                      |
| :-------- | :--------------------------------------------------- | :--------------------------------------------------------- |
| `user`    | `~/.claude/settings.json`                            | Личные плагины, доступные во всех проектах (по умолчанию)  |
| `project` | `.claude/settings.json`                              | Плагины команды, общие через контроль версий               |
| `local`   | `.claude/settings.local.json`                        | Плагины, специфичные для проекта, игнорируемые git         |
| `managed` | [Управляемые параметры](/docs/ru/settings#settings-files) | Управляемые плагины (только для чтения, только обновление) |

Плагины используют ту же систему областей, что и другие конфигурации Claude Code. Для инструкций по установке и флагов области смотрите [Установка плагинов](/docs/ru/discover-plugins#install-plugins). Для полного объяснения областей смотрите [Области конфигурации](/docs/ru/settings#configuration-scopes).

***

<h2 id="skills-directory-plugins">
  Плагины в каталоге skills
</h2>

Любая папка в каталоге skills, которая содержит манифест `.claude-plugin/plugin.json`, загружается как плагин с именем `<name>@skills-dir` в следующем сеансе, без маркетплейса и без шага установки. Создайте один с помощью [`plugin init`](#plugin-init). В отличие от установки маркетплейса, плагин обнаруживается на месте, а не копируется в кэш плагина.

Дерево каталога skills поддерживает три различных вещи:

| Что у вас есть                                | Что это такое                                                                                            |
| :-------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| `<skills-dir>/foo/SKILL.md` без манифеста     | Простой [skill](/docs/ru/skills) с именем `foo`                                                               |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | Плагин `foo@skills-dir`, который может объединять свои собственные skills, agents, hooks и многое другое |
| `<plugin>/skills/bar/SKILL.md`                | Skill `bar`, упакованный внутри плагина                                                                  |

<h3 id="choose-where-the-plugin-loads-from">
  Выберите, откуда загружается плагин
</h3>

| Каталог skills          | Область | Загружает                                                                                                    |
| :---------------------- | :------ | :----------------------------------------------------------------------------------------------------------- |
| `~/.claude/skills/`     | личная  | В каждом проекте, так как местоположение только ваше                                                         |
| `<cwd>/.claude/skills/` | проект  | Только после того, как вы примете диалог доверия рабочей области [trust dialog](/docs/ru/settings) для этой папки |

Плагин области проекта проверяется в репозитории и достигает каждого сотрудника, который его клонирует. Поскольку это содержимое поступает из репозитория, а не от вас, оно загружается только после того же шлюза доверия, который управляет `.claude/settings.json`, и компоненты, которые запускают код, дополнительно ограничены:

* Серверы MCP, которые он объявляет, проходят через [то же одобрение для каждого сервера](/docs/ru/mcp) что и проект `.mcp.json`
* Серверы LSP запускаются только после того, как вы доверяете рабочей области
* [Фоновые monitors](#monitors) не загружаются

Плагины личной области не имеют этих ограничений.

<Warning>
  Плагины `@skills-dir` области проекта загружаются только из `.claude/skills/` каталога, где вы запускаете Claude Code. Они не [поднимаются к корню репозитория](/docs/ru/skills#automatic-discovery-from-parent-and-nested-directories) так, как это делают простые skills и команды, поэтому запуск из подкаталога пропускает плагин, который находится в корне репо. Запустите из корня репозитория или запустите `/reload-plugins` после изменения каталогов.
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  Редактирование, перезагрузка и отключение плагина в каталоге skills
</h3>

Изменения, которые вы вносите в `SKILL.md` skill, вступают в силу немедленно в текущем сеансе. Изменения в других компонентах плагина, таких как `hooks/`, `.mcp.json`, `agents/` и `output-styles/`, не вступают в силу. Запустите `/reload-plugins` или перезагрузите Claude Code, чтобы их подхватить. Смотрите [Обнаружение изменений в реальном времени](/docs/ru/skills#live-change-detection).

Чтобы остановить загрузку плагина в каталоге skills, удалите его папку или отключите его по имени. Нет шага `uninstall`, потому что ничего не было установлено из маркетплейса.

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  Схема манифеста плагина
</h2>

Файл `.claude-plugin/plugin.json` определяет метаданные и конфигурацию вашего плагина. Этот раздел документирует все поддерживаемые поля и опции.

Манифест опционален. Если он опущен, Claude Code автоматически обнаруживает компоненты в [местоположениях по умолчанию](#file-locations-reference) и выводит имя плагина из имени каталога. Используйте манифест, когда вам нужно предоставить метаданные или пользовательские пути компонентов.

<h3 id="complete-schema">
  Полная схема
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  Обязательные поля
</h3>

Если вы включаете манифест, `name` — единственное обязательное поле.

| Поле   | Тип    | Описание                                                                                                                                                                                                                                         | Пример               |
| :----- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `name` | string | Уникальный идентификатор (kebab-case, без пробелов). Когда [запись маркетплейса](/docs/ru/plugin-marketplaces#plugin-entries) указывает плагин под другим именем, имя записи маркетплейса — это то, что используют ключи `enabledPlugins` и `/plugin` | `"deployment-tools"` |

Это имя используется для пространства имён компонентов. Например, в пользовательском интерфейсе агент `agent-creator` для плагина с именем `plugin-dev` будет отображаться как `plugin-dev:agent-creator`.

<h3 id="unrecognized-fields">
  Нераспознанные поля
</h3>

Claude Code игнорирует поля верхнего уровня, которые он не распознаёт. Вы можете сохранить метаданные из другой экосистемы в `plugin.json`, и плагин всё ещё будет загружаться. Это делает практичным поддержание одного манифеста, который одновременно служит манифестом расширения VS Code или Cursor, npm `package.json` или манифестом пакета MCPB/DXT.

`claude plugin validate` сообщает о нераспознанных полях как о предупреждениях, а не об ошибках. Если поле отличается на один или два символа от распознанного, предупреждение предлагает вероятное предполагаемое имя. Плагин только с предупреждениями о нераспознанных полях всё ещё проходит валидацию и загружается во время выполнения.

Поля с неправильным типом всё ещё вызывают ошибку. Например, значение `keywords`, которое является строкой вместо массива, является ошибкой загрузки, и `claude plugin validate` сообщает об этом.

Передайте `--strict` для обработки предупреждений как ошибок. Используйте это в CI для перехвата опечатки в имени поля или поля, оставшегося от манифеста другого инструмента перед публикацией, даже если плагин загружается во время выполнения.

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  Поля метаданных
</h3>

| Поле             | Тип     | Описание                                                                                                                                                                                                                                                                                                                                                                                                                | Пример                                                            |
| :--------------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| `$schema`        | string  | URL JSON Schema для автодополнения и валидации редактора. Claude Code игнорирует это поле при загрузке.                                                                                                                                                                                                                                                                                                                 | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | Удобочитаемое имя, отображаемое в средстве выбора `/plugin` и других поверхностях пользовательского интерфейса. Возвращается к `name` при опущении. В отличие от `name`, может содержать пробелы и любой регистр. Не используется для пространства имён или поиска. Требует Claude Code v2.1.143 или более поздней версии.                                                                                              | `"Deployment Tools"`                                              |
| `version`        | string  | Опционально. Семантическая версия. Установка этого параметра закрепляет плагин на этой строке версии, поэтому пользователи получают обновления только при её изменении. Если опущено, Claude Code использует SHA коммита git, поэтому каждый коммит рассматривается как новая версия. Если также установлено в записи маркетплейса, `plugin.json` имеет приоритет. Смотрите [Управление версиями](#version-management). | `"2.1.0"`                                                         |
| `description`    | string  | Краткое объяснение назначения плагина                                                                                                                                                                                                                                                                                                                                                                                   | `"Deployment automation tools"`                                   |
| `author`         | object  | Информация об авторе                                                                                                                                                                                                                                                                                                                                                                                                    | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | URL документации                                                                                                                                                                                                                                                                                                                                                                                                        | `"https://docs.example.com"`                                      |
| `repository`     | string  | URL исходного кода                                                                                                                                                                                                                                                                                                                                                                                                      | `"https://github.com/user/plugin"`                                |
| `license`        | string  | Идентификатор лицензии                                                                                                                                                                                                                                                                                                                                                                                                  | `"MIT"`, `"Apache-2.0"`                                           |
| `keywords`       | array   | Теги обнаружения                                                                                                                                                                                                                                                                                                                                                                                                        | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | Включен ли плагин в состояние включения, когда пользователь не установил его. По умолчанию `true`. Смотрите [Включение по умолчанию](#default-enablement). Требует Claude Code v2.1.154 или более поздней версии.                                                                                                                                                                                                       | `false`                                                           |

<h3 id="default-enablement">
  Включение по умолчанию
</h3>

Установите `defaultEnabled: false` в `plugin.json`, чтобы отправить плагин, который устанавливается отключённым. Пользователь включает его с помощью `claude plugin enable <plugin>` или интерфейса `/plugin`. Используйте это для плагинов, которые добавляют стоимость или область, в которую пользователь должен согласиться, например для плагина, который подключается к внешнему сервису. Это требует Claude Code v2.1.154 или более поздней версии. Более ранние версии игнорируют поле и включают плагин при установке.

`defaultEnabled` — это резервный вариант, когда ничто другое не решило состояние плагина. Два вещи имеют приоритет над ним:

* **Параметр пользователя**: запись для плагина в `enabledPlugins` в любой области параметров. После записи она сохраняется при обновлениях и переустановках плагина, поэтому изменение `defaultEnabled` в более позднем выпуске не переключает существующего пользователя.
* **Требование зависимости**: когда плагин требуется другим активным плагином, Claude Code записывает `true` для него при установке или включении. Это даёт ему явный параметр, поэтому его собственное значение по умолчанию больше не применяется. Смотрите [Включение или отключение плагина с зависимостями](/docs/ru/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies).

То же поле может появляться в записи маркетплейса плагина, где оно имеет приоритет над значением в `plugin.json`. Смотрите [Опциональные поля плагина](/docs/ru/plugin-marketplaces#optional-plugin-fields).

<h3 id="component-path-fields">
  Поля пути компонента
</h3>

| Поле                    | Тип                   | Описание                                                                                                                                                                                      | Пример                                               |
| :---------------------- | :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `skills`                | string\|array         | Пользовательские каталоги skills, содержащие `<name>/SKILL.md`. Добавляет к по умолчанию `skills/`. Смотрите [Правила поведения пути](#path-behavior-rules) для исключения корня маркетплейса | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | Пользовательские плоские файлы `.md` skill или каталоги (заменяет по умолчанию `commands/`)                                                                                                   | `"./custom/cmd.md"` или `["./cmd1.md"]`              |
| `agents`                | string\|array         | Пользовательские файлы агентов (заменяет по умолчанию `agents/`)                                                                                                                              | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Пути конфигурации hooks или встроенная конфигурация                                                                                                                                           | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | Пути конфигурации MCP или встроенная конфигурация                                                                                                                                             | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | Пользовательские файлы/каталоги стилей вывода (заменяет по умолчанию `output-styles/`)                                                                                                        | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | Конфигурации [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) для интеллектуальной информации о коде (переход к определению, поиск ссылок и т. д.)           | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | Файлы/каталоги цветовых тем (заменяет по умолчанию `themes/`). Смотрите [Themes](#themes)                                                                                                     | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | Конфигурации фонового [Monitor](/docs/ru/tools-reference#monitor-tool), которые запускаются автоматически при активации плагина. Смотрите [Monitors](#monitors)                                    | `"./monitors.json"`                                  |
| `userConfig`            | object                | Значения, настраиваемые пользователем, запрашиваемые при включении. Смотрите [Конфигурация пользователя](#user-configuration)                                                                 | Смотрите ниже                                        |
| `channels`              | array                 | Объявления каналов для внедрения сообщений (стиль Telegram, Slack, Discord). Смотрите [Каналы](#channels)                                                                                     | Смотрите ниже                                        |
| `dependencies`          | array                 | Другие плагины, которые требует этот плагин, опционально с ограничениями версии semver. Смотрите [Ограничение версий зависимостей плагина](/docs/ru/plugin-dependencies)                           | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  Экспериментальные компоненты
</h3>

Компоненты под ключом `experimental`, `themes` и `monitors`, имеют схему манифеста, которая может измениться между выпусками во время их стабилизации. Место, где вы их объявляете, — это отдельная миграция: верхний уровень всё ещё работает, `claude plugin validate` выдаёт предупреждение, и будущий выпуск потребует `experimental.*`.

<h3 id="user-configuration">
  Конфигурация пользователя
</h3>

Поле `userConfig` объявляет значения, которые Claude Code запрашивает у пользователя при включении плагина. Используйте это вместо требования пользователям вручную редактировать `settings.json`.

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

Ключи должны быть допустимыми идентификаторами. Каждое значение поддерживает эти поля:

| Поле          | Обязательно | Описание                                                                                      |
| :------------ | :---------- | :-------------------------------------------------------------------------------------------- |
| `type`        | Да          | Одно из `string`, `number`, `boolean`, `directory` или `file`                                 |
| `title`       | Да          | Метка, показываемая в диалоге конфигурации                                                    |
| `description` | Да          | Справочный текст, показываемый под полем                                                      |
| `sensitive`   | Нет         | Если `true`, скрывает ввод и сохраняет значение в защищённом хранилище вместо `settings.json` |
| `required`    | Нет         | Если `true`, проверка не пройдёт, когда поле пусто                                            |
| `default`     | Нет         | Значение, используемое, когда пользователь ничего не предоставляет                            |
| `multiple`    | Нет         | Для типа `string`, разрешить массив строк                                                     |
| `min` / `max` | Нет         | Границы для типа `number`                                                                     |

Каждое значение доступно для подстановки как `${user_config.KEY}` в конфигурациях серверов MCP и LSP и командах hooks. Нечувствительные значения также могут быть подставлены в содержимое skills и agents. Все значения экспортируются в процессы hooks как переменные окружения `CLAUDE_PLUGIN_OPTION_<KEY>`, где `<KEY>` — это ключ опции в верхнем регистре.

Поля, которые работают в shell, отклоняют `${user_config.*}`: подстановка настроенного значения в команду shell позволила бы shell выполнить всё, что содержит это значение, поэтому компонент не работает с [ошибкой](/docs/ru/errors#plugin-command-references-user-config). Каждое отклонённое поле имеет альтернативный способ передачи значения:

| Отклонённое поле                                                             | Как передать значение                                                                                                            |
| :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Команды hooks в форме shell                                                  | Используйте [форму exec](/docs/ru/hooks#exec-form-and-shell-form) с `args` или читайте `CLAUDE_PLUGIN_OPTION_<KEY>` из окружения hook |
| Команды [Monitor](#monitors)                                                 | Читайте значение из файла конфигурации в скрипте                                                                                 |
| MCP [`headersHelper`](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication) | Читайте значение из файла конфигурации в скрипте                                                                                 |

До v2.1.207 эти поля подставляли значения `${user_config.KEY}`; обновите плагины, которые полагались на это.

Нечувствительные значения хранятся под ключом [`pluginConfigs`](/docs/ru/settings#pluginconfigs) в `settings.json` как `pluginConfigs[<plugin-id>].options`. Claude Code записывает ключ в параметры пользователя и читает его обратно из параметров пользователя, флага `--settings` и управляемых параметров только; записи в `.claude/settings.json` или `.claude/settings.local.json` проекта игнорируются. До v2.1.207 Claude Code также читал параметры проекта и локальные параметры.

Чувствительные значения переходят в macOS Keychain или в `~/.claude/.credentials.json` на платформах, где keychain недоступен. Хранилище Keychain общее с OAuth токенами и имеет приблизительный лимит 2 КБ, поэтому держите чувствительные значения небольшими.

<h3 id="channels">
  Каналы
</h3>

Поле `channels` позволяет плагину объявить один или несколько каналов сообщений, которые внедряют содержимое в разговор. Каждый канал привязывается к серверу MCP, который предоставляет плагин.

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "Owner ID",
          "description": "Your Telegram user ID"
        }
      }
    }
  ]
}
```

Поле `server` обязательно и должно соответствовать ключу в `mcpServers` плагина. Опциональный `userConfig` для каждого канала использует ту же схему, что и поле верхнего уровня, позволяя плагину запрашивать токены ботов или ID владельцев при включении плагина.

<h3 id="path-behavior-rules">
  Правила поведения пути
</h3>

Замена ли пользовательский путь или расширяет каталог по умолчанию плагина, зависит от поля:

* **Заменяет по умолчанию**: `commands`, `agents`, `outputStyles`, `experimental.themes`, `experimental.monitors`. Например, когда манифест указывает `commands`, каталог по умолчанию `commands/` не сканируется. Чтобы сохранить по умолчанию и добавить больше, перечислите его явно: `"commands": ["./commands/", "./extras/"]`
* **Добавляет к по умолчанию**: `skills`. Каталог по умолчанию `skills/` всегда сканируется, и каталоги, перечисленные в `skills`, загружаются вместе с ним. Исключение: для [записи маркетплейса, чей `source` разрешается в корень маркетплейса](/docs/ru/plugin-marketplaces#advanced-plugin-entries), объявление конкретных подкаталогов заменяет сканирование по умолчанию `skills/`
* **Собственные правила слияния**: [hooks](#hooks), [MCP servers](#mcp-servers) и [LSP servers](#lsp-servers). Смотрите каждый раздел для того, как несколько источников объединяются

Когда плагин имеет как папку по умолчанию, так и соответствующий ключ манифеста, Claude Code v2.1.140 и более поздние версии отмечают игнорируемую папку в `claude plugin list` и представлении деталей `/plugin`. Плагин всё ещё загружается с использованием путей манифеста. Предупреждение не показывается, когда ключ манифеста указывает на папку по умолчанию, например `"commands": ["./commands/deploy.md"]`, потому что папка явно адресуется в этом случае.

Для всех полей пути:

* Все пути должны быть относительны к корню плагина и начинаться с `./`
* Компоненты из пользовательских путей используют те же правила именования и пространства имён
* Несколько путей можно указать как массивы
* Когда путь skill указывает на каталог, который содержит `SKILL.md` напрямую, например `"skills": ["./"]`, указывающий на корень плагина, поле frontmatter `name` в `SKILL.md` определяет имя вызова skill. Это обеспечивает стабильное имя независимо от каталога установки. Если `name` не установлен в frontmatter, в качестве резервного варианта используется имя каталога.

Плагин, который имеет `SKILL.md` в своём корне, не имеет подкаталога `skills/` и не имеет поля манифеста `skills`, автоматически загружается как плагин с одним skill в Claude Code v2.1.142 и более поздних версиях. Вам не нужно устанавливать `"skills": ["./"]` в `plugin.json` для этого макета. Имя вызова skill следует тому же правилу, что и выше: поле frontmatter `name` или имя каталога в качестве резервного варианта.

**Примеры путей**:

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  Переменные окружения
</h3>

Claude Code предоставляет три переменные для ссылки на пути:

| Переменная              | Разрешается в                                                                                                                 | Используйте для                                                                                                  |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Абсолютный путь к каталогу установки плагина                                                                                  | Скрипты, двоичные файлы и файлы конфигурации, поставляемые с плагином                                            |
| `${CLAUDE_PLUGIN_DATA}` | [Постоянный каталог](#persistent-data-directory), который сохраняется при обновлениях плагина, создаётся при первом обращении | Установленные зависимости, такие как `node_modules` или виртуальные окружения Python, сгенерированный код и кэши |
| `${CLAUDE_PROJECT_DIR}` | Корень проекта                                                                                                                | Скрипты и файлы конфигурации, локальные для проекта                                                              |

Все три экспортируются как переменные окружения в процессы hooks и в подпроцессы серверов MCP и LSP. Какие поля подставляют их встроенно, зависит от компонента плагина:

| Компонент плагина               | Поля, где разрешаются заполнители           |
| :------------------------------ | :------------------------------------------ |
| Содержимое skill и agent        | Везде, где появляется заполнитель           |
| Команды hook и monitor          | Везде, где появляется заполнитель           |
| MCP серверы `stdio`             | `command`, `args`, `env`                    |
| MCP серверы `http`, `sse`, `ws` | `url`, `headers`, `headersHelper`           |
| Серверы LSP                     | `command`, `args`, `env`, `workspaceFolder` |

В командах hook используйте [форму exec](/docs/ru/hooks#exec-form-and-shell-form) с `args`, чтобы каждый путь передавался как один аргумент без кавычек. В hooks в форме shell и командах monitor оборачивайте переменные в двойные кавычки, как в `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`. Этот hook в форме shell запускает скрипт, поставляемый с плагином:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` изменяется при обновлении плагина. Каталог предыдущей версии остаётся на диске примерно семь дней после обновления перед очисткой, но рассматривайте его как временный и не записывайте состояние там.

Когда плагин обновляется во время сеанса, команды hooks, monitors, серверы MCP и серверы LSP продолжают использовать путь предыдущей версии. Запустите `/reload-plugins` для переключения hooks, серверов MCP и серверов LSP на новый путь; monitors требуют перезагрузки сеанса.

Серверы MCP также могут вызывать запрос `roots/list` для чтения рабочих каталогов сеанса во время выполнения. Смотрите [что возвращает `roots/list` и когда Claude Code уведомляет сервер об изменениях](/docs/ru/mcp#option-3-add-a-local-stdio-server).

<h4 id="persistent-data-directory">
  Каталог постоянных данных
</h4>

Каталог `${CLAUDE_PLUGIN_DATA}` разрешается в `~/.claude/plugins/data/{id}/`, где `{id}` — это идентификатор плагина с символами вне `a-z`, `A-Z`, `0-9`, `_` и `-`, заменённые на `-`. Для плагина, установленного как `formatter@my-marketplace`, каталог — это `~/.claude/plugins/data/formatter-my-marketplace/`.

Распространённое использование — установка языковых зависимостей один раз и их повторное использование в сеансах и обновлениях плагина. Поскольку каталог данных пережидает любую отдельную версию плагина, проверка только существования каталога не может обнаружить, когда обновление изменяет манифест зависимостей плагина. Рекомендуемый паттерн сравнивает поставляемый манифест с копией в каталоге данных и переустанавливает при различиях.

Этот hook `SessionStart` устанавливает `node_modules` при первом запуске и снова всякий раз, когда обновление плагина включает изменённый `package.json`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

`diff` выходит с ненулевым кодом, когда сохранённая копия отсутствует или отличается от поставляемой, охватывая как первый запуск, так и обновления, изменяющие зависимости. Если `npm install` не удаётся, завершающий `rm` удаляет скопированный манифест, чтобы следующий сеанс повторил попытку.

Скрипты, поставляемые в `${CLAUDE_PLUGIN_ROOT}`, затем могут работать с сохранённым `node_modules`:

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

Каталог данных удаляется автоматически при удалении плагина из последней области, где он установлен. Интерфейс `/plugin` показывает размер каталога и запрашивает перед удалением. CLI удаляет по умолчанию; передайте [`--keep-data`](#plugin-uninstall) для сохранения.

***

<h2 id="plugin-caching-and-file-resolution">
  Кэширование плагина и разрешение файлов
</h2>

Плагины указываются одним из двух способов:

* Через `claude --plugin-dir` или `claude --plugin-url`, на время сеанса.
* Через маркетплейс, установленный для будущих сеансов.

В целях безопасности и проверки Claude Code копирует плагины *маркетплейса* в локальный **кэш плагина** пользователя (`~/.claude/plugins/cache`) вместо использования их на месте. Понимание этого поведения важно при разработке плагинов, которые ссылаются на внешние файлы.

Каждая установленная версия — это отдельный каталог в кэше. Когда вы обновляете или удаляете плагин, предыдущий каталог версии помечается как сиротский и удаляется автоматически через 7 дней. Период отсрочки позволяет одновременным сеансам Claude Code, которые уже загрузили старую версию, продолжать работу без ошибок.

Инструменты Glob и Grep Claude пропускают сиротские каталоги версий при поиске, поэтому результаты файлов не включают устаревший код плагина.

<h3 id="path-traversal-limitations">
  Ограничения обхода пути
</h3>

Установленные плагины не могут ссылаться на файлы вне их каталога. Пути, которые выходят за пределы корня плагина (такие как `../shared-utils`), не будут работать после установки, потому что эти внешние файлы не копируются в кэш.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Совместное использование файлов в маркетплейсе с помощью символических ссылок
</h3>

Если вашему плагину нужно совместно использовать файлы с другими частями того же маркетплейса, вы можете создать символические ссылки внутри каталога вашего плагина. То, как символическая ссылка обрабатывается при копировании плагина в кэш, зависит от того, где разрешается её цель:

* **В собственном каталоге плагина:** символическая ссылка сохраняется как относительная символическая ссылка в кэше, поэтому она продолжает разрешаться к скопированной цели во время выполнения.
* **В другом месте в том же маркетплейсе:** символическая ссылка разыменовывается. Содержимое цели копируется в кэш на её место. Это позволяет каталогу `skills/` мета-плагина ссылаться на навыки, определённые другими плагинами в маркетплейсе.
* **Вне маркетплейса:** символическая ссылка пропускается в целях безопасности. Это предотвращает извлечение плагинами произвольных файлов хоста, таких как системные пути, в кэш.

Для плагинов, установленных с помощью `--plugin-dir` или из локального пути, сохраняются только символические ссылки, которые разрешаются в собственном каталоге плагина. Все остальные пропускаются.

Следующая команда создаёт ссылку из плагина маркетплейса на общий навык, определённый соседним плагином. В Windows используйте `mklink /D` из командной строки с повышенными привилегиями или включите режим разработчика:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

Это обеспечивает гибкость при сохранении преимуществ безопасности системы кэширования.

***

<h2 id="plugin-directory-structure">
  Структура каталога плагина
</h2>

<h3 id="standard-plugin-layout">
  Стандартная раскладка плагина
</h3>

Полный плагин следует этой структуре:

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # Каталог метаданных (опционально)
│   └── plugin.json             # манифест плагина
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # Skills как плоские файлы .md
│   ├── status.md
│   └── logs.md
├── agents/                   # Определения subagent
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # Определения стиля вывода
│   └── terse.md
├── themes/                   # Определения цветовой темы
│   └── dracula.json
├── monitors/                 # Конфигурации фонового monitor
│   └── monitors.json
├── hooks/                    # Конфигурации hook
│   ├── hooks.json           # Основная конфигурация hook
│   └── security-hooks.json  # Дополнительные hooks
├── bin/                      # Исполняемые файлы плагина, добавленные в PATH
│   └── my-tool               # Вызываемый как голая команда в инструменте Bash
├── settings.json            # Параметры по умолчанию для плагина
├── .mcp.json                # Определения сервера MCP
├── .lsp.json                # Конфигурации сервера LSP
├── scripts/                 # Скрипты hook и утилиты
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # Файл лицензии
└── CHANGELOG.md             # История версий
```

<Warning>
  Каталог `.claude-plugin/` содержит файл `plugin.json`. Все остальные каталоги (commands/, agents/, skills/, output-styles/, themes/, monitors/, hooks/) должны быть в корне плагина, а не внутри `.claude-plugin/`.
</Warning>

Файл `CLAUDE.md` в корне плагина не загружается как контекст проекта. Плагины предоставляют контекст через skills, agents и hooks, а не через CLAUDE.md. Чтобы отправить инструкции, которые загружаются в контекст Claude, поместите их в [skill](#skills).

<h3 id="file-locations-reference">
  Справочник местоположений файлов
</h3>

| Компонент         | Местоположение по умолчанию  | Назначение                                                                                                                                                                                           |
| :---------------- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Манифест**      | `.claude-plugin/plugin.json` | Метаданные и конфигурация плагина (опционально)                                                                                                                                                      |
| **Skills**        | `skills/`                    | Skills со структурой `<name>/SKILL.md`                                                                                                                                                               |
| **Commands**      | `commands/`                  | Skills как плоские файлы Markdown. Используйте `skills/` для новых плагинов                                                                                                                          |
| **Agents**        | `agents/`                    | Файлы Subagent Markdown                                                                                                                                                                              |
| **Output styles** | `output-styles/`             | Определения стиля вывода                                                                                                                                                                             |
| **Themes**        | `themes/`                    | Определения цветовой темы                                                                                                                                                                            |
| **Hooks**         | `hooks/hooks.json`           | Конфигурация hook                                                                                                                                                                                    |
| **MCP servers**   | `.mcp.json`                  | Определения сервера MCP                                                                                                                                                                              |
| **LSP servers**   | `.lsp.json`                  | Конфигурации языкового сервера                                                                                                                                                                       |
| **Monitors**      | `monitors/monitors.json`     | Конфигурации фонового monitor                                                                                                                                                                        |
| **Executables**   | `bin/`                       | Исполняемые файлы, добавленные в PATH инструмента Bash. Файлы здесь вызываются как голые команды в любом вызове инструмента Bash, пока плагин включен                                                |
| **Settings**      | `settings.json`              | Конфигурация по умолчанию, применяемая при включении плагина. В настоящее время поддерживаются только ключи [`agent`](/docs/ru/sub-agents) и [`subagentStatusLine`](/docs/ru/statusline#subagent-status-lines) |

***

<h2 id="cli-commands-reference">
  Справочник команд CLI
</h2>

Claude Code предоставляет команды CLI для неинтерактивного управления плагинами, полезные для написания скриптов и автоматизации.

<h3 id="plugin-init">
  plugin init
</h3>

Создайте новый плагин в `~/.claude/skills/<name>/`. В следующем сеансе Claude Code он загружается автоматически как `<name>@skills-dir` и появляется в `/plugin` и `claude plugin list` без шага установки.

Смотрите [Плагины в каталоге skills](#skills-directory-plugins) для требований области и доверия.

```bash theme={null}
claude plugin init <name> [options]
```

**Аргументы:**

* `<name>`: Имя плагина. Становится пространством имён skill и именем каталога в `~/.claude/skills/`, поэтому не может содержать пробелы или разделители пути.

**Опции:**

| Опция                    | Описание                                                                                                                    | По умолчанию            |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | Описание манифеста                                                                                                          |                         |
| `--author <name>`        | Имя автора                                                                                                                  | `git config user.name`  |
| `--author-email <email>` | Email автора                                                                                                                | `git config user.email` |
| `--with <components...>` | Также создайте папки компонентов. Допустимые значения: `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, `channel` |                         |
| `-f, --force`            | Перезаписать существующий `.claude-plugin/` в целевом месте                                                                 |                         |
| `-h, --help`             | Отобразить справку для команды                                                                                              |                         |

**Псевдонимы:** `new`

Каждое значение `--with` добавляет стартовый файл для этого компонента, готовый к редактированию:

| Компонент      | Что он создаёт                                                                                        |
| :------------- | :---------------------------------------------------------------------------------------------------- |
| `skills`       | Дополнительный skill `<name>:example` наряду с основным                                               |
| `agents`       | Определение subagent в `agents/`                                                                      |
| `hooks`        | `hooks/hooks.json` с примером обработчика события                                                     |
| `mcp`          | `.mcp.json` с примерами серверов HTTP и stdio                                                         |
| `lsp`          | Пример языкового сервера в `.lsp.json`                                                                |
| `output-style` | `output-styles/<name>.md`, который применяется автоматически при включении плагина                    |
| `channel`      | Основанный на MCP [канал](/docs/ru/channels): сервер stdio (`server.ts`), его `.mcp.json` и `package.json` |

Созданный плагин использует источник `@skills-dir` вместо маркетплейса. Администраторы могут заблокировать этот источник с помощью `strictKnownMarketplaces` или добавив `{"source": "skills-dir"}` в `blockedMarketplaces` в [управляемых параметрах](/docs/ru/plugin-marketplaces#managed-marketplace-restrictions). При блокировке `plugin init` завершается с ошибкой перед записью.

**Примеры:**

```bash theme={null}
# Создайте минимальный плагин
claude plugin init my-helper

# Создайте с папками skill и hook
claude plugin init my-helper --with skills hooks

# Перезаписать существующий scaffold
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

Установите плагин из доступных маркетплейсов.

```bash theme={null}
claude plugin install <plugin> [options]
```

**Аргументы:**

* `<plugin>`: Имя плагина или `plugin-name@marketplace-name` для конкретного маркетплейса

**Опции:**

| Опция                 | Описание                                         | По умолчанию |
| :-------------------- | :----------------------------------------------- | :----------- |
| `-s, --scope <scope>` | Область установки: `user`, `project` или `local` | `user`       |
| `-h, --help`          | Отобразить справку для команды                   |              |

Область определяет, в какой файл параметров добавляется установленный плагин. Например, `--scope project` записывает в `enabledPlugins` в .claude/settings.json, делая плагин доступным для всех, кто клонирует репозиторий проекта.

**Примеры:**

```bash theme={null}
# Установить в область пользователя (по умолчанию)
claude plugin install formatter@my-marketplace

# Установить в область проекта (общее с командой)
claude plugin install formatter@my-marketplace --scope project

# Установить в локальную область (игнорируется git)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Удалите установленный плагин.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**Аргументы:**

* `<plugin>`: Имя плагина или `plugin-name@marketplace-name`

**Опции:**

| Опция                 | Описание                                                                                                                            | По умолчанию |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------- | :----------- |
| `-s, --scope <scope>` | Удалить из области: `user`, `project` или `local`                                                                                   | `user`       |
| `--keep-data`         | Сохранить [каталог постоянных данных](#persistent-data-directory) плагина                                                           |              |
| `--prune`             | Также удалить автоматически установленные зависимости, которые не требуются другим плагинам. Смотрите [plugin prune](#plugin-prune) |              |
| `-y, --yes`           | Пропустить подтверждение `--prune`. Требуется, когда stdin не является TTY                                                          |              |
| `-h, --help`          | Отобразить справку для команды                                                                                                      |              |

**Псевдонимы:** `remove`, `rm`

По умолчанию удаление из последней оставшейся области также удаляет каталог `${CLAUDE_PLUGIN_DATA}` плагина. Используйте `--keep-data` для сохранения, например при переустановке после тестирования новой версии.

<h3 id="plugin-prune">
  plugin prune
</h3>

Удалите автоматически установленные зависимости плагинов, которые больше не требуются ни одному установленному плагину. Зависимости, которые Claude Code подтянул для удовлетворения поля [`dependencies`](/docs/ru/plugin-dependencies) другого плагина, удаляются; плагины, которые вы установили напрямую, никогда не затрагиваются.

```bash theme={null}
claude plugin prune [options]
```

**Опции:**

| Опция                 | Описание                                                         | По умолчанию |
| :-------------------- | :--------------------------------------------------------------- | :----------- |
| `-s, --scope <scope>` | Очистить в области: `user`, `project` или `local`                | `user`       |
| `--dry-run`           | Список того, что будет удалено, без фактического удаления        |              |
| `-y, --yes`           | Пропустить подтверждение. Требуется, когда stdin не является TTY |              |
| `-h, --help`          | Отобразить справку для команды                                   |              |

**Псевдонимы:** `autoremove`

Команда выводит список потерянных зависимостей и запрашивает подтверждение перед их удалением. Чтобы удалить плагин и очистить его зависимости в один шаг, запустите `claude plugin uninstall <plugin> --prune`.

<Note>
  `claude plugin prune` требует Claude Code v2.1.121 или более поздней версии.
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

Включите отключённый плагин. Если плагин объявляет [зависимости](/docs/ru/plugin-dependencies), Claude Code включает их транзитивно в той же области, и команда завершается с ошибкой, когда зависимость не установлена.

```bash theme={null}
claude plugin enable <plugin> [options]
```

**Аргументы:**

* `<plugin>`: Имя плагина или `plugin-name@marketplace-name`

**Опции:**

| Опция                 | Описание                                             | По умолчанию |
| :-------------------- | :--------------------------------------------------- | :----------- |
| `-s, --scope <scope>` | Область для включения: `user`, `project` или `local` | `user`       |
| `-h, --help`          | Отобразить справку для команды                       |              |

<h3 id="plugin-disable">
  plugin disable
</h3>

Отключите плагин без его удаления. Завершается с ошибкой, когда другой включённый плагин [зависит от](/docs/ru/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) целевого плагина. Сообщение об ошибке включает цепочку команд, которая сначала отключает каждый зависимый плагин.

```bash theme={null}
claude plugin disable <plugin> [options]
```

**Аргументы:**

* `<plugin>`: Имя плагина или `plugin-name@marketplace-name`

**Опции:**

| Опция                 | Описание                                              | По умолчанию |
| :-------------------- | :---------------------------------------------------- | :----------- |
| `-s, --scope <scope>` | Область для отключения: `user`, `project` или `local` | `user`       |
| `-h, --help`          | Отобразить справку для команды                        |              |

<h3 id="plugin-update">
  plugin update
</h3>

Обновите плагин до последней версии.

```bash theme={null}
claude plugin update <plugin> [options]
```

**Аргументы:**

* `<plugin>`: Имя плагина или `plugin-name@marketplace-name`

**Опции:**

| Опция                 | Описание                                                         | По умолчанию |
| :-------------------- | :--------------------------------------------------------------- | :----------- |
| `-s, --scope <scope>` | Область для обновления: `user`, `project`, `local` или `managed` | `user`       |
| `-h, --help`          | Отобразить справку для команды                                   |              |

***

<h3 id="plugin-list">
  plugin list
</h3>

Список установленных плагинов с их версией, источником маркетплейса и статусом включения.

```bash theme={null}
claude plugin list [options]
```

**Опции:**

| Опция         | Описание                                                      | По умолчанию |
| :------------ | :------------------------------------------------------------ | :----------- |
| `--json`      | Вывести как JSON                                              |              |
| `--available` | Включить доступные плагины из маркетплейсов. Требует `--json` |              |
| `-h, --help`  | Отобразить справку для команды                                |              |

В интерактивном сеансе `/plugin list` выводит тот же список встроенным образом. Интерактивная форма принимает `--enabled` или `--disabled` для отображения только плагинов в этом состоянии, и `ls` как сокращение для `list`.

<h3 id="plugin-details">
  plugin details
</h3>

Показать инвентарь компонентов плагина и прогнозируемую стоимость в токенах. Вывод содержит список всех компонентов, которые вносит плагин, сгруппированных как Skills, Agents, Hooks, MCP серверы и LSP серверы, вместе с оценкой того, сколько токенов он добавляет к каждой сессии. Группа Skills включает как записи `skills/`, так и `commands/`.

```bash theme={null}
claude plugin details <name>
```

**Аргументы:**

* `<name>`: Имя плагина или `plugin-name@marketplace-name`

**Опции:**

| Опция        | Описание                       | По умолчанию |
| :----------- | :----------------------------- | :----------- |
| `-h, --help` | Отобразить справку для команды |              |

Вывод показывает две цифры стоимости для каждого компонента:

* **Always-on:** токены, добавляемые к каждой сессии текстом описания плагина, такие как описания навыков, описания агентов и имена команд, независимо от того, срабатывает ли какой-либо компонент.
* **On-invoke:** токены, которые стоит компонент при срабатывании. Показано для каждого компонента отдельно, а не как итог плагина, потому что типичная сессия вызывает только подмножество компонентов.

Этот пример показывает, как выглядит вывод для плагина с двумя навыками:

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Итог always-on вычисляется через API `count_tokens` для вашей активной модели. Числа для каждого компонента пропорционально масштабируются от этого итога. Если API недоступен, команда переходит на оценку на основе количества символов.

<h3 id="plugin-tag">
  plugin tag
</h3>

Создайте тег выпуска git для плагина в текущем каталоге. Запустите из папки плагина. Смотрите [Теги выпусков плагинов](/docs/ru/plugin-dependencies#tag-plugin-releases-for-version-resolution).

```bash theme={null}
claude plugin tag [options]
```

**Опции:**

| Опция         | Описание                                                            | По умолчанию |
| :------------ | :------------------------------------------------------------------ | :----------- |
| `--push`      | Отправить тег на удалённый репозиторий после его создания           |              |
| `--dry-run`   | Вывести, что будет помечено тегом, без создания самого тега         |              |
| `-f, --force` | Создать тег даже если рабочее дерево грязное или тег уже существует |              |
| `-h, --help`  | Отобразить справку для команды                                      |              |

***

<h2 id="debugging-and-development-tools">
  Инструменты отладки и разработки
</h2>

<h3 id="debugging-commands">
  Команды отладки
</h3>

Используйте `claude --debug` для просмотра деталей загрузки плагина:

Это показывает:

* Какие плагины загружаются
* Любые ошибки в манифестах плагинов
* Регистрацию skill, agent и hook
* Инициализацию сервера MCP

<h3 id="common-issues">
  Распространённые проблемы
</h3>

| Проблема                            | Причина                             | Решение                                                                                                                                                                |
| :---------------------------------- | :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Плагин не загружается               | Неверный `plugin.json`              | Запустите `claude plugin validate` или `/plugin validate` для проверки `plugin.json`, frontmatter skill/agent/command и `hooks/hooks.json` на синтаксис и ошибки схемы |
| Skills не отображаются              | Неправильная структура каталога     | Убедитесь, что `skills/` или `commands/` находится в корне плагина, а не в `.claude-plugin/`                                                                           |
| Hooks не срабатывают                | Скрипт не исполняемый               | Запустите `chmod +x script.sh`                                                                                                                                         |
| Сервер MCP не работает              | Отсутствует `${CLAUDE_PLUGIN_ROOT}` | Используйте переменную для всех путей плагина                                                                                                                          |
| Ошибки пути                         | Используются абсолютные пути        | Все пути должны быть относительными и начинаться с `./`                                                                                                                |
| LSP `Executable not found in $PATH` | Языковой сервер не установлен       | Установите двоичный файл (например, `npm install -g typescript-language-server typescript`)                                                                            |

<h3 id="example-error-messages">
  Примеры сообщений об ошибках
</h3>

**Ошибки проверки манифеста**:

* `Invalid JSON syntax: Unexpected token } in JSON at position 142`: проверьте наличие пропущенных запятых, лишних запятых или неквотированных строк
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required`: отсутствует обязательное поле
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...`: ошибка синтаксиса JSON

**Ошибки загрузки плагина**:

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.`: путь команды существует, но не содержит действительных файлов команд
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.`: путь `source` в marketplace.json указывает на несуществующий каталог
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.`: удалите дублирующиеся определения компонентов или удалите `strict: false` в записи маркетплейса

<h3 id="hook-troubleshooting">
  Устранение неполадок Hook
</h3>

**Скрипт hook не выполняется**:

1. Проверьте, что скрипт исполняемый: `chmod +x ./scripts/your-script.sh`
2. Проверьте строку shebang: Первая строка должна быть `#!/bin/bash` или `#!/usr/bin/env bash`
3. Проверьте, что путь использует `${CLAUDE_PLUGIN_ROOT}`: `"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. Протестируйте скрипт вручную: `./scripts/your-script.sh`

**Hook не срабатывает на ожидаемых событиях**:

1. Проверьте, что имя события правильное (чувствительно к регистру): `PostToolUse`, а не `postToolUse`
2. Проверьте, что шаблон сопоставления соответствует вашим инструментам: `"matcher": "Write|Edit"` для операций с файлами
3. Подтвердите, что тип hook действителен: `command`, `http`, `mcp_tool`, `prompt` или `agent`

<h3 id="mcp-server-troubleshooting">
  Устранение неполадок сервера MCP
</h3>

**Сервер не запускается**:

1. Проверьте, что команда существует и исполняемая
2. Проверьте, что все пути используют переменную `${CLAUDE_PLUGIN_ROOT}`
3. Проверьте журналы сервера MCP: `claude --debug` показывает ошибки инициализации
4. Протестируйте сервер вручную вне Claude Code

**Инструменты сервера не отображаются**:

1. Убедитесь, что сервер правильно настроен в `.mcp.json` или `plugin.json`
2. Проверьте, что сервер правильно реализует протокол MCP
3. Проверьте наличие тайм-аутов соединения в выводе отладки

<h3 id="directory-structure-mistakes">
  Ошибки структуры каталога
</h3>

**Симптомы**: Плагин загружается, но компоненты (skills, agents, hooks) отсутствуют.

**Правильная структура**: Компоненты должны быть в корне плагина, а не внутри `.claude-plugin/`. Только `plugin.json` должен быть в `.claude-plugin/`.

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← Только манифест здесь
├── commands/            ← На уровне корня
├── agents/              ← На уровне корня
└── hooks/               ← На уровне корня
```

Если ваши компоненты находятся внутри `.claude-plugin/`, переместите их в корень плагина.

**Контрольный список отладки**:

1. Запустите `claude --debug` и ищите сообщения "loading plugin"
2. Проверьте, что каждый каталог компонента указан в выводе отладки
3. Проверьте, что разрешения файлов позволяют читать файлы плагина

***

<h2 id="distribution-and-versioning-reference">
  Справочник по распространению и версионированию
</h2>

<h3 id="version-management">
  Управление версиями
</h3>

Claude Code использует версию плагина в качестве ключа кэша, который определяет, доступно ли обновление. Когда вы запускаете `/plugin update` или срабатывает автоматическое обновление, Claude Code вычисляет текущую версию и пропускает обновление, если она совпадает с уже установленной.

Версия определяется из первого из следующих параметров, который установлен:

1. Поле `version` в `plugin.json` плагина
2. Поле `version` в записи плагина на маркетплейсе в `marketplace.json`
3. SHA коммита git источника плагина для источников `github`, `url`, `git-subdir` и relative-path в маркетплейсе, размещённом на git
4. `unknown` для источников `npm` или локальных каталогов, не находящихся в репозитории git

Это дает вам два способа версионирования плагина:

| Подход                 | Как                                                       | Поведение обновления                                                                                                                                                                        | Лучше всего для                                        |
| :--------------------- | :-------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------- |
| **Явная версия**       | Установите `"version": "2.1.0"` в `plugin.json`           | Пользователи получают обновления только когда вы обновляете это поле. Отправка новых коммитов без обновления не имеет эффекта, и `/plugin update` сообщает "already at the latest version". | Опубликованные плагины со стабильными циклами выпуска  |
| **Версия коммита SHA** | Опустите `version` из `plugin.json` и записи маркетплейса | Пользователи получают обновления при каждом новом коммите в источник git плагина                                                                                                            | Внутренние или командные плагины в активной разработке |

<Warning>
  Если вы установите `version` в `plugin.json`, вы должны обновлять его каждый раз, когда хотите, чтобы пользователи получили изменения. Отправка новых коммитов недостаточна, потому что Claude Code видит ту же строку версии и сохраняет кэшированную копию. Если вы быстро итерируете, оставьте `version` неустановленным, чтобы вместо этого использовалась SHA коммита git.
</Warning>

Если вы используете явные версии, следуйте [семантическому версионированию](https://semver.org) (`MAJOR.MINOR.PATCH`): обновляйте MAJOR для критических изменений, MINOR для новых функций, PATCH для исправлений ошибок. Документируйте изменения в `CHANGELOG.md`.

***

<h2 id="see-also">
  Смотрите также
</h2>

* [Плагины](/docs/ru/plugins) - Учебные материалы и практическое использование
* [Маркетплейсы плагинов](/docs/ru/plugin-marketplaces) - Создание и управление маркетплейсами
* [Skills](/docs/ru/skills) - Детали разработки skills
* [Subagents](/docs/ru/sub-agents) - Конфигурация и возможности агентов
* [Hooks](/docs/ru/hooks) - Обработка событий и автоматизация
* [MCP](/docs/ru/mcp) - Интеграция внешних инструментов
* [Параметры](/docs/ru/settings) - Опции конфигурации для плагинов
