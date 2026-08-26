> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник по hooks

> Справочник по событиям hook Claude Code, схеме конфигурации, форматам JSON входа/выхода, кодам выхода, асинхронным hooks, HTTP hooks, prompt hooks и MCP tool hooks.

<Tip>
  Для краткого руководства с примерами см. [Автоматизация рабочих процессов с помощью hooks](/docs/ru/hooks-guide).
</Tip>

Hooks — это определяемые пользователем команды оболочки, конечные точки HTTP или подсказки LLM, которые выполняются автоматически в определённых точках жизненного цикла Claude Code. Используйте этот справочник для поиска схем событий, параметров конфигурации, форматов JSON входа/выхода и расширенных функций, таких как асинхронные hooks, HTTP hooks и MCP tool hooks. Если вы настраиваете hooks впервые, начните с [руководства](/docs/ru/hooks-guide).

<h2 id="hook-lifecycle">
  Жизненный цикл hook
</h2>

Hooks срабатывают в определённых точках во время сеанса Claude Code. Когда событие срабатывает и совпадает с фильтром, Claude Code передаёт JSON-контекст события вашему обработчику hook. Для command hooks входные данные поступают на stdin. Для HTTP hooks они поступают как тело POST-запроса. Ваш обработчик может затем проверить входные данные, выполнить действие и опционально вернуть решение.

События срабатывают в трёх ритмах:

* один раз за сеанс: `SessionStart` и `SessionEnd`
* один раз за ход: `UserPromptSubmit`, `Stop` и `StopFailure`
* при каждом вызове инструмента внутри агентного цикла: `PreToolUse` и `PostToolUse`

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/jhXrDR5TrSZ5hgXM/images/hooks-lifecycle.svg?fit=max&auto=format&n=jhXrDR5TrSZ5hgXM&q=85&s=3ca47113d5956460e6e4611b8dbc63b7" alt="Диаграмма жизненного цикла hook, показывающая опциональный Setup, переходящий в SessionStart, затем цикл за ход, содержащий UserPromptSubmit, UserPromptExpansion для slash commands, вложенный агентный цикл (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted) и Stop или StopFailure, за которым следуют TeammateIdle, PreCompact, PostCompact и SessionEnd, с Elicitation и ElicitationResult вложенными внутри выполнения MCP tool, PermissionDenied как боковая ветвь от PermissionRequest для автоматических отказов, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged и FileChanged как отдельные асинхронные события, и MessageDisplay как событие только для отображения, которое выполняется во время потоковой передачи текста сообщения помощника" width="520" height="1228" data-path="images/hooks-lifecycle.svg" />
  </Frame>
</div>

Таблица ниже суммирует, когда срабатывает каждое событие. Раздел [Hook events](#hook-events) документирует полную схему входа и параметры управления решением для каждого события.

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

<h3 id="how-a-hook-resolves">
  Как разрешается hook
</h3>

Чтобы увидеть, как эти части работают вместе, рассмотрим этот hook `PreToolUse`, который блокирует деструктивные команды оболочки. Фильтр `matcher` сужает область до вызовов инструмента Bash, а условие `if` сужает её дальше до команд Bash, совпадающих с `rm *`, поэтому `block-rm.sh` запускается только когда оба фильтра совпадают:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(rm *)",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

Скрипт читает JSON входные данные из stdin, извлекает команду и возвращает `permissionDecision` со значением `"deny"`, если она содержит `rm -rf`:

```bash theme={null}
#!/bin/bash
# .claude/hooks/block-rm.sh
COMMAND=$(jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q 'rm -rf'; then
  jq -n '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: "Destructive command blocked by hook"
    }
  }'
else
  exit 0  # no decision; normal permission flow applies
fi
```

Теперь предположим, что Claude Code решает запустить `Bash "rm -rf /tmp/build"`. Вот что происходит:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" alt="Диаграмма разрешения hook: срабатывает событие PreToolUse, фильтр проверяет совпадение Bash, затем условие if проверяет совпадение Bash(rm *). Если оба совпадают, команда hook запускается и возвращает permissionDecision deny, поэтому вызов инструмента блокируется и Claude Code продолжает работу. Если одна из проверок не совпадает, hook пропускается и вызов инструмента может продолжить работу." width="930" height="270" data-path="images/hook-resolution.svg" />
</Frame>

<Steps>
  <Step title="Событие срабатывает">
    Событие `PreToolUse` срабатывает. Claude Code отправляет входные данные инструмента как JSON на stdin hook:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Фильтр проверяет">
    Фильтр `"Bash"` совпадает с именем инструмента, поэтому эта группа hook активируется. Если вы опустите фильтр или используете `"*"`, группа активируется при каждом возникновении события.
  </Step>

  <Step title="Условие if проверяет">
    Условие `if` `"Bash(rm *)"` совпадает, потому что `rm -rf /tmp/build` — это подкоманда, совпадающая с `rm *`, поэтому этот обработчик запускается. Если бы команда была `npm test`, проверка `if` не удалась бы и `block-rm.sh` никогда не запустился бы, избегая затрат на порождение процесса. Поле `if` опционально; без него каждый обработчик в совпадающей группе запускается.
  </Step>

  <Step title="Обработчик hook запускается">
    Скрипт проверяет полную команду и находит `rm -rf`, поэтому выводит решение на stdout:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Если бы команда была более безопасным вариантом `rm`, таким как `rm file.txt`, скрипт выполнил бы `exit 0` вместо этого. Код выхода 0 без вывода означает, что hook не имеет решения для отчёта, поэтому вызов инструмента продолжается через нормальный [поток разрешений](/docs/ru/permissions). Hook может отклонить вызов, но молчание не одобряет его.
  </Step>

  <Step title="Claude Code действует на основе результата">
    Claude Code читает JSON решение, блокирует вызов инструмента и показывает Claude причину.
  </Step>
</Steps>

Раздел [Configuration](#configuration) ниже документирует полную схему, и каждый раздел [hook event](#hook-events) документирует, какой входной JSON получает ваша команда и какой выход она может вернуть.

<h2 id="configuration">
  Конфигурация
</h2>

Hooks определяются в JSON файлах настроек. Конфигурация имеет три уровня вложенности:

1. Выберите [hook event](#hook-events) для ответа, например `PreToolUse` или `Stop`
2. Добавьте [matcher group](#matcher-patterns) для фильтрации срабатывания, например "только для инструмента Bash"
3. Определите один или несколько [hook handlers](#hook-handler-fields) для запуска при совпадении

См. [Как разрешается hook](#how-a-hook-resolves) выше для полного пошагового руководства с аннотированным примером.

<Note>
  На этой странице используются специальные термины для каждого уровня: **hook event** для точки жизненного цикла, **matcher group** для фильтра и **hook handler** для команды оболочки, конечной точки HTTP, инструмента MCP, подсказки или агента, который запускается. "Hook" сам по себе относится к общей функции.
</Note>

<h3 id="hook-locations">
  Расположение hook
</h3>

Место, где вы определяете hook, определяет его область действия:

| Расположение                                                | Область действия       | Общий доступ                                        |
| :---------------------------------------------------------- | :--------------------- | :-------------------------------------------------- |
| `~/.claude/settings.json`                                   | Все ваши проекты       | Нет, локально на вашей машине                       |
| `.claude/settings.json`                                     | Один проект            | Да, можно зафиксировать в репозитории               |
| `.claude/settings.local.json`                               | Один проект            | Нет, игнорируется git когда Claude Code создаёт его |
| Управляемые параметры политики                              | Организация            | Да, контролируется администратором                  |
| [Plugin](/docs/ru/plugins) `hooks/hooks.json`                    | Когда плагин включен   | Да, поставляется с плагином                         |
| [Skill](/docs/ru/skills) или [agent](/docs/ru/sub-agents) frontmatter | Пока компонент активен | Да, определено в файле компонента                   |

Для получения подробной информации о разрешении файлов настроек см. [settings](/docs/ru/settings). Администраторы предприятия могут использовать `allowManagedHooksOnly` для блокировки пользовательских, проектных и плагинных hooks. Hooks из плагинов, принудительно включённых в управляемых параметрах `enabledPlugins`, исключены, поэтому администраторы могут распространять проверенные hooks через организационный marketplace. См. [Hook configuration](/docs/ru/settings#hook-configuration).

<h3 id="matcher-patterns">
  Matcher patterns
</h3>

Поле `matcher` фильтрует срабатывание hooks. Способ оценки фильтра зависит от содержащихся в нём символов:

| Значение фильтра                                   | Оценивается как                                                                                    | Пример                                                                                                                                                                       |
| :------------------------------------------------- | :------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` или опущено                            | Совпадение со всеми                                                                                | срабатывает при каждом возникновении события                                                                                                                                 |
| Только буквы, цифры, `_`, `-`, пробелы, `,` и `\|` | Точная строка или список точных строк, разделённых `\|` или `,` с опциональным окружающим пробелом | `Bash` совпадает только с инструментом Bash; `Edit\|Write` и `Edit, Write` каждый совпадает с любым инструментом точно; `code-reviewer` совпадает только с этим типом агента |
| Содержит любой другой символ                       | Регулярное выражение JavaScript, без привязки                                                      | `^Notebook` совпадает с любым инструментом, начинающимся с Notebook; `mcp__memory__.*` совпадает с каждым инструментом с сервера `memory`                                    |

Фильтр на пути регулярного выражения проверяется с помощью `RegExp.prototype.test` JavaScript, который успешно совпадает в любом месте значения. `Edit.*` совпадает как с `Edit`, так и с `NotebookEdit`; оберните шаблон в `^` и `$`, как в `^Edit$`, когда вам нужно совпадение всей строки.

Разделители запятых и допуск окружающего пробела требуют Claude Code v2.1.191 или позже.

Дефисы в наборе точного совпадения требуют Claude Code v2.1.195 или позже. На более ранних версиях дефисное имя, такое как `code-reviewer`, оценивается как регулярное выражение без привязки, поэтому оно также срабатывает для `senior-code-reviewer`; закрепите его как `^code-reviewer$` на этих версиях, чтобы совпадать только с этим именем.

`FileChanged` и `StopFailure` используют более узкий набор точного совпадения только букв, цифр, `_` и `|`. Дефис, пробел или запятая в фильтре для этих двух событий держит его на пути регулярного выражения, и только `|` разделяет альтернативы. Каждое другое событие с поддержкой фильтра в таблице ниже принимает `|` или `,`.

Событие `FileChanged` не следует этим правилам при построении своего списка наблюдения. См. [FileChanged](#filechanged).

Каждый тип события совпадает с другим полем:

| Событие                                                                                                                                           | На что фильтр влияет                                                     | Примеры значений фильтра                                                                                                                                                            |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | имя инструмента                                                          | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                    | как сеанс начался                                                        | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                           | какой флаг CLI запустил setup                                            | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                      | почему сеанс закончился                                                  | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                    | тип уведомления                                                          | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                   | тип агента                                                               | `general-purpose`, `Explore`, `Plan`, пользовательские имена агентов или имена с областью плагина, такие как `^my-plugin:reviewer$`                                                 |
| `PreCompact`, `PostCompact`                                                                                                                       | что вызвало компактирование                                              | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                    | тип агента                                                               | те же значения, что и `SubagentStart`                                                                                                                                               |
| `ConfigChange`                                                                                                                                    | источник конфигурации                                                    | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `CwdChanged`                                                                                                                                      | поддержка фильтра отсутствует                                            | всегда срабатывает при каждом изменении каталога                                                                                                                                    |
| `FileChanged`                                                                                                                                     | буквальные имена файлов для наблюдения (см. [FileChanged](#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `StopFailure`                                                                                                                                     | тип ошибки                                                               | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                              | причина загрузки                                                         | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `UserPromptExpansion`                                                                                                                             | имя команды                                                              | ваши имена skills или команд                                                                                                                                                        |
| `Elicitation`                                                                                                                                     | имя MCP сервера                                                          | ваши настроенные имена MCP серверов                                                                                                                                                 |
| `ElicitationResult`                                                                                                                               | имя MCP сервера                                                          | те же значения, что и `Elicitation`                                                                                                                                                 |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | поддержка фильтра отсутствует                                            | всегда срабатывает при каждом вхождении                                                                                                                                             |

Фильтр запускается против поля из [JSON входа](#hook-input-and-output), который Claude Code отправляет вашему hook на stdin. Для событий инструмента это поле — `tool_name`. Каждый раздел [hook event](#hook-events) перечисляет полный набор значений фильтра и схему входа для этого события.

Этот пример запускает скрипт линтинга только когда Claude пишет или редактирует файл:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

`UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` и `CwdChanged` не поддерживают фильтры и всегда срабатывают при каждом вхождении. Если вы добавите поле `matcher` к этим событиям, оно будет молча проигнорировано.

Для событий инструмента вы можете фильтровать более узко, установив поле [`if`](#common-fields) на отдельных обработчиках hook. `if` использует [синтаксис правила разрешения](/docs/ru/permissions) для совпадения с именем инструмента и аргументами вместе, поэтому `"Bash(git *)"` запускается когда любая подкоманда входа Bash совпадает с `git *` и `"Edit(*.ts)"` запускается только для файлов TypeScript.

<h4 id="match-mcp-tools">
  Match MCP tools
</h4>

[MCP](/docs/ru/mcp) server инструменты отображаются как обычные инструменты в событиях инструментов (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), поэтому вы можете совпадать с ними так же, как с любым другим именем инструмента.

MCP инструменты следуют шаблону именования `mcp__<server>__<tool>`, например:

* `mcp__memory__create_entities`: инструмент create entities сервера Memory
* `mcp__filesystem__read_file`: инструмент read file сервера Filesystem
* `mcp__github__search_repositories`: инструмент поиска сервера GitHub

Чтобы совпадать с каждым инструментом с сервера, добавьте `.*` к префиксу сервера. `.*` требуется: фильтр, такой как `mcp__memory` или `mcp__brave-search`, содержит только символы точного совпадения, поэтому он сравнивается как точная строка и не совпадает ни с одним инструментом.

* `mcp__memory__.*` совпадает со всеми инструментами сервера `memory`
* `mcp__brave-search__.*` совпадает со всеми инструментами с сервера, чьё имя содержит дефис
* `mcp__.*__write.*` совпадает с любым инструментом, чьё имя начинается с `write` из любого сервера

Дефисы в наборе точного совпадения требуют Claude Code v2.1.195 или позже. На более ранних версиях голый дефисный префикс, такой как `mcp__brave-search`, оценивается как регулярное выражение без привязки и совпадает с каждым инструментом с этого сервера. Форма `mcp__brave-search__.*` работает на каждой версии.

Инструменты из [plugin-bundled MCP server](/docs/ru/mcp#plugin-provided-mcp-servers) используют сегмент сервера с областью, который включает имя плагина: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Фильтр, написанный против голого ключа сервера, никогда не срабатывает для этих инструментов. Для плагина с именем `my-plugin`, который объединяет сервер под ключом `db`, инструмент `query` отображается как `mcp__plugin_my-plugin_db__query`, поэтому фильтр для каждого инструмента с этого сервера — `mcp__plugin_my-plugin_db__.*`. Используйте то же имя инструмента с областью в поле [`if`](#common-fields) обработчика. См. [Plugin-provided MCP servers](/docs/ru/mcp#plugin-provided-mcp-servers) для того, как строится имя с областью.

Этот пример логирует все операции сервера memory и проверяет операции записи из любого MCP сервера:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Hook handler fields
</h3>

Каждый объект во внутреннем массиве `hooks` — это hook handler: команда оболочки, конечная точка HTTP, инструмент MCP, подсказка LLM или агент, который запускается при совпадении фильтра. Есть пять типов:

* **[Command hooks](#command-hook-fields)** (`type: "command"`): запускают команду оболочки. Ваш скрипт получает [JSON входные данные](#hook-input-and-output) события на stdin и передаёт результаты обратно через коды выхода и stdout.
* **[HTTP hooks](#http-hook-fields)** (`type: "http"`): отправляют JSON входные данные события как HTTP POST запрос на URL. Конечная точка передаёт результаты обратно через тело ответа, используя тот же [JSON формат выхода](#json-output), что и command hooks.
* **[MCP tool hooks](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): вызывают инструмент на уже подключённом [MCP сервере](/docs/ru/mcp). Текстовый вывод инструмента обрабатывается как stdout command hook.
* **[Prompt hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): отправляют подсказку модели Claude для однооборотной оценки. Модель возвращает решение да/нет как JSON. См. [Prompt-based hooks](#prompt-based-hooks).
* **[Agent hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): порождают subagent, который может использовать инструменты, такие как Read, Grep и Glob, для проверки условий перед возвратом решения. Agent hooks являются экспериментальными и могут измениться. См. [Agent-based hooks](#agent-based-hooks).

Все совпадающие hooks запускаются параллельно, и идентичные обработчики автоматически дедублируются. Command hooks дедублируются по строке команды и `args`, а HTTP hooks дедублируются по URL.

Обработчики запускаются в текущем каталоге с окружением Claude Code. Переменная окружения `$CLAUDE_CODE_REMOTE` устанавливается на `"true"` в удалённых веб-окружениях и не устанавливается в локальном CLI. Начиная с v2.1.199, [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/ru/env-vars) устанавливается на ID сеанса [Remote Control](/docs/ru/remote-control) пока локальный сеанс имеет активное соединение Remote Control.

<h4 id="common-fields">
  Common fields
</h4>

Эти поля применяются ко всем типам hooks:

| Поле            | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :-------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | да          | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` или `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `if`            | нет         | Синтаксис правила разрешения для фильтрации срабатывания этого hook, такой как `"Bash(git *)"` или `"Edit(*.ts)"`. Hook запускается только если вызов инструмента совпадает с шаблоном. См. таблицу [Bash matching table](#bash-if-matching) ниже для того, как Bash шаблоны оцениваются против подкоманд, `$()` и обратных кавычек. Оценивается только на событиях инструмента: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` и `PermissionDenied`. На других событиях hook с установленным `if` никогда не запускается. Использует тот же синтаксис, что и [правила разрешения](/docs/ru/permissions) |
| `timeout`       | нет         | Секунды перед отменой. Значения по умолчанию: 600 для `command`, `http` и `mcp_tool`; 30 для `prompt`; 60 для `agent`. [`UserPromptSubmit`](#userpromptsubmit) снижает значение по умолчанию для `command`, `http` и `mcp_tool` до 30, и [`MessageDisplay`](#messagedisplay) снижает его до 10                                                                                                                                                                                                                                                                                                                              |
| `statusMessage` | нет         | Пользовательское сообщение спиннера, отображаемое во время выполнения hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `once`          | нет         | Если `true`, запускается один раз за сеанс затем удаляется. Только для hooks, объявленных в [skill frontmatter](#hooks-in-skills-and-agents); игнорируется в файлах настроек и agent frontmatter                                                                                                                                                                                                                                                                                                                                                                                                                            |

Поле `if` содержит ровно одно правило разрешения. Нет синтаксиса `&&`, `||` или списка для объединения правил; чтобы применить несколько условий, определите отдельный обработчик hook для каждого.

<span id="bash-if-matching" />Для Bash шаблонов, запускается ли ваша команда hook зависит от формы шаблона и команды Bash, которую вызывает Claude. Ведущие присваивания `VAR=value` удаляются перед совпадением.

| `if` шаблон        | Bash команда           | Hook запускается? | Почему                                                                                                                  |
| :----------------- | :--------------------- | :---------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`     | да                | ведущие присваивания удаляются; `git push` совпадает                                                                    |
| `Bash(git *)`      | `npm test && git push` | да                | каждая подкоманда проверяется; `git push` совпадает                                                                     |
| `Bash(rm *)`       | `echo $(rm -rf /)`     | да                | команды внутри `$()` и обратных кавычек проверяются; `rm -rf /` совпадает                                               |
| `Bash(rm *)`       | `echo $(date)`         | нет               | ни одна подкоманда не совпадает с `rm *`                                                                                |
| `Bash(git push *)` | `echo $(date)`         | да                | шаблоны, которые указывают больше чем имя команды, запускают hook в любом случае на `$()`, обратных кавычках или `$VAR` |

Фильтр также открывается с ошибкой, запуская ваш hook независимо от шаблона, когда команда Bash не может быть проанализирована. Поскольку фильтр `if` является лучшим усилием, используйте [систему разрешений](/docs/ru/permissions) вместо hook для обеспечения жёсткого разрешения или отказа.

<h4 id="command-hook-fields">
  Command hook fields
</h4>

В дополнение к [общим полям](#common-fields), command hooks принимают эти поля:

| Поле          | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                                     |
| :------------ | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | да          | Команда оболочки для выполнения. С `args`, исполняемый файл для прямого запуска. См. [Exec form and shell form](#exec-form-and-shell-form)                                                                                                                                                                                                                                   |
| `args`        | нет         | Список аргументов. Когда присутствует, `command` разрешается как исполняемый файл и запускается напрямую с `args` как вектор аргументов, без участия оболочки. См. [Exec form and shell form](#exec-form-and-shell-form)                                                                                                                                                     |
| `async`       | нет         | Если `true`, запускается в фоне без блокировки. См. [Run hooks in the background](#run-hooks-in-the-background)                                                                                                                                                                                                                                                              |
| `asyncRewake` | нет         | Если `true`, запускается в фоне и пробуждает Claude при коде выхода 2. Подразумевает `async`. Stderr hook или stdout, если stderr пусто, показывается Claude как системное напоминание, чтобы он мог реагировать на долгоживущий фоновый сбой                                                                                                                                |
| `shell`       | нет         | Оболочка для использования для этого hook. Принимает `"bash"` или `"powershell"`. По умолчанию `"bash"`, или `"powershell"` на Windows когда Git Bash не установлен. Установка `"powershell"` запускает команду через PowerShell на Windows. Не требует `CLAUDE_CODE_USE_POWERSHELL_TOOL`, так как hooks порождают PowerShell напрямую. Игнорируется когда установлен `args` |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec form and shell form
</h5>

Command hook запускается в exec form когда установлен `args`, и в shell form когда `args` опущен. Установите `args` всякий раз, когда hook ссылается на [path placeholder](#reference-scripts-by-path), так как каждый элемент передаётся как один аргумент без кавычек. Опустите `args` когда вам нужны функции оболочки, такие как pipes или `&&`, или когда ни одна из этих проблем не применяется.

**Exec form** запускается когда присутствует `args`. Claude Code разрешает `command` как исполняемый файл на `PATH` и запускает его напрямую с `args` как вектор аргументов. Нет оболочки, поэтому каждый элемент `args` — это ровно один аргумент, написанный как есть, и path placeholders, такие как `${CLAUDE_PLUGIN_ROOT}`, подставляются в `command` и в каждый элемент `args` как простые строки. Специальные символы, такие как апострофы, `$` и обратные кавычки, проходят дословно, потому что нет оболочки для их интерпретации. На любой платформе не происходит никакой токенизации оболочки.

**Shell form** запускается когда `args` отсутствует. Строка `command` передаётся в оболочку: `sh -c` на macOS и Linux, Git Bash на Windows, или PowerShell когда Git Bash не установлен. Установите поле `shell` для явного выбора. Оболочка токенизирует строку, расширяет переменные и интерпретирует pipes, `&&`, redirects и globs.

<Note>
  На Windows, exec form требует, чтобы `command` разрешался в реальный исполняемый файл, такой как `.exe`. Shims `.cmd` и `.bat`, которые npm, npx, eslint и другие инструменты устанавливают в `node_modules/.bin`, не являются исполняемыми файлами и не могут быть запущены без оболочки. Чтобы запустить их в exec form, вызовите базовый скрипт с `node` напрямую, например `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. Паттерн `node` плюс script-path работает на каждой платформе, потому что `node.exe` — это реальный бинарный файл. Чтобы запустить shim `.cmd` или `.bat` по имени, используйте shell form.
</Note>

Этот пример запускает Node скрипт, поставляемый с плагином. Exec form передаёт разрешённый путь скрипта как один аргумент без кавычек:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

Эквивалентная shell form нуждается в кавычках для обработки путей с пробелами или специальными символами:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Обе формы поддерживают одни и те же [path placeholders](#reference-scripts-by-path), и обе экспортируют их как переменные окружения `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` и `CLAUDE_PLUGIN_DATA` на порождённом процессе, поэтому скрипт может читать `process.env.CLAUDE_PLUGIN_ROOT` независимо от того, как он был запущен.

Plugin hooks дополнительно подставляют значения [`${user_config.*}`](/docs/ru/plugins-reference#user-configuration), только в exec form: значение подставляется в `command` и в каждый элемент `args` как простая строка, поэтому оболочка не переанализирует его.

Shell-form plugin hook, чей `command` ссылается на `${user_config.*}`, завершается с [ошибкой](/docs/ru/errors#plugin-command-references-user-config) вместо запуска. Чтобы использовать значение опции из shell-form hook, прочитайте переменную окружения `$CLAUDE_PLUGIN_OPTION_<KEY>`, такую как `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` для опции `webhook_url`, или установите `args` для переключения hook на exec form. До v2.1.207, shell-form plugin hook команды также подставляли `${user_config.*}`.

<Note>
  В exec form, `command` — это только имя исполняемого файла или путь. Если `command` — это голое имя без разделителя пути и содержит пробелы рядом с `args`, Claude Code логирует предупреждение, потому что spawn не удастся: нет исполняемого файла с именем `node script.js`. Переместите дополнительные токены в `args`. Абсолютные пути с пробелами, такие как `C:\Program Files\nodejs\node.exe`, — это один действительный исполняемый файл и не вызывают предупреждение.
</Note>

<h4 id="http-hook-fields">
  HTTP hook fields
</h4>

В дополнение к [общим полям](#common-fields), HTTP hooks принимают эти поля:

| Поле             | Обязательно | Описание                                                                                                                                                                                                                           |
| :--------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | да          | URL для отправки POST запроса                                                                                                                                                                                                      |
| `headers`        | нет         | Дополнительные HTTP заголовки как пары ключ-значение. Значения поддерживают интерполяцию переменных окружения с использованием синтаксиса `$VAR_NAME` или `${VAR_NAME}`. Разрешены только переменные, указанные в `allowedEnvVars` |
| `allowedEnvVars` | нет         | Список имён переменных окружения, которые могут быть интерполированы в значения заголовков. Ссылки на неуказанные переменные заменяются пустыми строками. Требуется для любой интерполяции переменных окружения                    |

Claude Code отправляет [JSON входные данные](#hook-input-and-output) hook как тело POST запроса с `Content-Type: application/json`. Тело ответа использует тот же [JSON формат выхода](#json-output), что и command hooks.

Обработка ошибок отличается от command hooks: ответы не 2xx, сбои соединения и таймауты все производят неблокирующие ошибки, которые позволяют выполнению продолжаться. Чтобы заблокировать вызов инструмента или отклонить разрешение, верните ответ 2xx с JSON телом, содержащим `decision: "block"` или `hookSpecificOutput` с `permissionDecision: "deny"`.

Этот пример отправляет события `PreToolUse` на локальный сервис валидации, аутентифицируясь с токеном из переменной окружения `MY_TOKEN`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  MCP tool hook fields
</h4>

В дополнение к [общим полям](#common-fields), MCP tool hooks принимают эти поля:

| Поле     | Обязательно | Описание                                                                                                                                                                                                                                                                                                 |
| :------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | да          | Имя настроенного MCP сервера. Для [plugin-bundled server](/docs/ru/mcp#plugin-provided-mcp-servers), это имя с областью `plugin:<plugin-name>:<server-name>`, такое как `plugin:my-plugin:db`, не голый ключ сервера. Сервер должен быть уже подключён; hook никогда не запускает поток OAuth или подключения |
| `tool`   | да          | Имя инструмента для вызова на этом сервере                                                                                                                                                                                                                                                               |
| `input`  | нет         | Аргументы, передаваемые инструменту. Строковые значения поддерживают подстановку `${path}` из [JSON входа](#hook-input-and-output) hook, такую как `"${tool_input.file_path}"`                                                                                                                           |

Текстовое содержимое инструмента обрабатывается как stdout command hook: если оно анализируется как действительный [JSON выход](#json-output), оно обрабатывается как решение, в противном случае оно показывается как простой текст. Если названный сервер не подключён или инструмент возвращает `isError: true`, hook производит неблокирующую ошибку и выполнение продолжается.

MCP tool hooks доступны на каждом hook событии после того, как Claude Code подключился к вашим MCP серверам. `SessionStart` и `Setup` обычно срабатывают до завершения подключения серверов, поэтому hooks на этих событиях должны ожидать ошибку "не подключено" при первом запуске.

Этот пример вызывает инструмент `security_scan` на MCP сервере `my_server` после каждого `Write` или `Edit`, передавая путь отредактированного файла:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

<h4 id="prompt-and-agent-hook-fields">
  Prompt and agent hook fields
</h4>

В дополнение к [общим полям](#common-fields), prompt и agent hooks принимают эти поля:

| Поле     | Обязательно | Описание                                                                                                                                                                                                 |
| :------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | да          | Текст подсказки для отправки модели. Используйте `$ARGUMENTS` как заполнитель для JSON входа hook. Экранируйте обратной косой чертой для включения буквального текста: `\$1.00` отображается как `$1.00` |
| `model`  | нет         | Модель для использования при оценке. По умолчанию быстрая модель                                                                                                                                         |

<h3 id="reference-scripts-by-path">
  Reference scripts by path
</h3>

Используйте эти заполнители для ссылки на скрипты hook относительно корня проекта или плагина, независимо от рабочего каталога при запуске hook:

* `${CLAUDE_PROJECT_DIR}`: корень проекта. Claude Code также устанавливает эту переменную в окружении [stdio MCP серверов](/docs/ru/mcp#option-3-add-a-local-stdio-server) и plugin LSP серверов.
* `${CLAUDE_PLUGIN_ROOT}`: каталог установки плагина, для скриптов, поставляемых с [плагином](/docs/ru/plugins). Изменяется при каждом обновлении плагина.
* `${CLAUDE_PLUGIN_DATA}`: [каталог постоянных данных](/docs/ru/plugins-reference#persistent-data-directory) плагина, для зависимостей и состояния, которые должны пережить обновления плагина.

Предпочитайте [exec form](#exec-form-and-shell-form) для любого hook, который ссылается на path placeholder. Exec form передаёт каждый элемент `args` как один аргумент без токенизации оболочки, поэтому пути с пробелами или специальными символами не нуждаются в кавычках. В shell form оберните каждый заполнитель в двойные кавычки.

<Tabs>
  <Tab title="Project scripts">
    Этот пример использует `${CLAUDE_PROJECT_DIR}` для запуска проверки стиля из каталога `.claude/hooks/` проекта после любого вызова инструмента `Write` или `Edit`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Plugin scripts">
    Определите plugin hooks в `hooks/hooks.json` с опциональным полем `description` верхнего уровня. Когда плагин включен, его hooks объединяются с вашими пользовательскими и проектными hooks.

    Этот пример запускает скрипт форматирования, поставляемый с плагином:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    См. [plugin components reference](/docs/ru/plugins-reference#hooks) для получения подробной информации о создании plugin hooks.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks in skills and agents
</h3>

В дополнение к файлам настроек и плагинам, hooks могут быть определены непосредственно в [skills](/docs/ru/skills) и [subagents](/docs/ru/sub-agents) с использованием frontmatter. Эти hooks ограничены жизненным циклом компонента и запускаются только когда этот компонент активен.

Поддерживаются все hook события. Для subagents, `Stop` hooks автоматически преобразуются в `SubagentStop`, так как это событие, которое срабатывает при завершении subagent.

Hooks используют тот же формат конфигурации, что и hooks на основе настроек, но ограничены жизненным циклом компонента и очищаются при его завершении.

Этот skill определяет hook `PreToolUse`, который запускает скрипт проверки безопасности перед каждой командой `Bash`:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

Agents используют тот же формат в своём YAML frontmatter.

<h3 id="the-/hooks-menu">
  Меню `/hooks`
</h3>

Введите `/hooks` в Claude Code, чтобы открыть браузер только для чтения ваших настроенных hooks. Меню показывает каждое hook событие с количеством настроенных hooks, позволяет вам углубиться в фильтры и показывает полные детали каждого hook обработчика. Используйте его для проверки конфигурации, проверки того, из какого файла настроек пришёл hook, или проверки команды, подсказки или URL hook.

Меню отображает все пять типов hook: `command`, `prompt`, `agent`, `http` и `mcp_tool`. Каждый hook помечен префиксом `[type]` и источником, указывающим, где он был определён:

* `User`: из `~/.claude/settings.json`
* `Project`: из `.claude/settings.json`
* `Local`: из `.claude/settings.local.json`
* `Plugin`: из `hooks/hooks.json` плагина
* `Session`: зарегистрирован в памяти для текущего сеанса
* `Built-in`: зарегистрирован внутри Claude Code

Выбор hook открывает представление деталей, показывающее его событие, фильтр, тип, исходный файл и полную команду, подсказку или URL. Меню только для чтения: чтобы добавить, изменить или удалить hooks, отредактируйте JSON настроек напрямую или попросите Claude сделать изменение.

<h3 id="disable-or-remove-hooks">
  Отключение или удаление hooks
</h3>

Чтобы удалить hook, удалите его запись из JSON файла настроек.

Чтобы временно отключить все hooks без их удаления, установите `"disableAllHooks": true` в файле настроек. Нет способа отключить отдельный hook, сохраняя его в конфигурации.

Параметр `disableAllHooks` соблюдает иерархию управляемых настроек. Если администратор настроил hooks через управляемые параметры политики, `disableAllHooks`, установленный в пользовательских, проектных или локальных настройках, не может отключить эти управляемые hooks. Только `disableAllHooks`, установленный на уровне управляемых настроек, может отключить управляемые hooks.

Прямые редактирования hooks в файлах настроек обычно захватываются автоматически наблюдателем файлов.

<h2 id="hook-input-and-output">
  Входные и выходные данные Hook
</h2>

Command hooks получают JSON данные через stdin и передают результаты через коды выхода, stdout и stderr. HTTP hooks получают тот же JSON как тело POST запроса и передают результаты через тело HTTP ответа. Этот раздел охватывает поля и поведение, общие для всех событий. Каждый раздел события под [Hook events](#hook-events) включает его специфическую схему входа и параметры управления решением.

На macOS и Linux command hooks запускаются в своём собственном сеансе без управляющего терминала начиная с v2.1.139. Процесс hook и любые дочерние процессы не могут открыть `/dev/tty` или отправлять escape последовательности непосредственно в интерфейс Claude Code. Windows не имеет `/dev/tty`. Чтобы вывести сообщение пользователю на любой платформе, верните [`systemMessage`](#json-output) в JSON выходе. Чтобы вызвать уведомление рабочего стола, установить заголовок окна или издать звуковой сигнал, верните [`terminalSequence`](#emit-terminal-notifications) вместо этого.

<h3 id="common-input-fields">
  Общие входные поля
</h3>

Hook события получают эти поля как JSON, в дополнение к полям, специфичным для события, документированным в каждом разделе [hook event](#hook-events). Для command hooks этот JSON поступает через stdin. Для HTTP hooks он поступает как тело POST запроса.

| Поле              | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Текущий идентификатор сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `prompt_id`       | UUID, идентифицирующий пользовательский запрос, который в настоящее время обрабатывается. Совпадает с атрибутом [`prompt.id` на событиях OpenTelemetry](/docs/ru/monitoring-usage#event-correlation-attributes), поэтому вы можете коррелировать выход hook с телеметрией для одного запроса. Отсутствует до первого пользовательского ввода. Требует Claude Code v2.1.196 или позже                                                                                                                                                                                                                                                                                                                                                                                                               |
| `transcript_path` | Путь к JSON разговора. Файл транскрипта записывается асинхронно и может отставать от разговора в памяти, поэтому он может ещё не включать самые последние сообщения текущего хода, когда срабатывает hook. Hooks, которым нужен финальный текст ассистента текущего хода, должны использовать `last_assistant_message` на [Stop](#stop) и [SubagentStop](#subagentstop) вместо чтения транскрипта                                                                                                                                                                                                                                                                                                                                                                                             |
| `cwd`             | Текущий рабочий каталог при вызове hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `permission_mode` | Текущий [режим разрешения](/docs/ru/permissions#permission-modes): `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` или `"bypassPermissions"`. Режим, обозначенный как **Manual**, поступает как `"default"`, никогда не как `"manual"`, поэтому скрипты, которые совпадают с `"default"`, продолжают работать. Не все события получают это поле. Проверьте пример JSON в каждом разделе [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                                   |
| `effort`          | Объект с полем `level`, содержащим активный [уровень усилий](/docs/ru/model-config#adjust-effort-level) для хода: `"low"`, `"medium"`, `"high"`, `"xhigh"` или `"max"`. Если запрошенный уровень усилий модели превышает то, что поддерживает текущая модель, это понижающий уровень, который модель фактически использовала. Ultracode не является отдельным уровнем и сообщается как `"xhigh"`. Объект соответствует полю `effort` [строки статуса](/docs/ru/statusline#available-data). Присутствует для событий, которые срабатывают в контексте использования инструмента, таких как `PreToolUse`, `PostToolUse`, `Stop` и `SubagentStop`, когда текущая модель поддерживает параметр усилий. Уровень также доступен для команд hook и инструмента Bash как переменная окружения `$CLAUDE_EFFORT`. |
| `hook_event_name` | Имя события, которое сработало                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

При запуске с `--agent` или внутри subagent включаются два дополнительных поля:

| Поле         | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Уникальный идентификатор для subagent. Присутствует только когда hook срабатывает внутри вызова subagent. Используйте это для различения вызовов hook subagent от вызовов основного потока.                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agent_type` | Имя агента (например, `"Explore"` или `"security-reviewer"`). Присутствует когда сеанс использует `--agent` или hook срабатывает внутри subagent. Для subagents тип subagent имеет приоритет над значением `--agent` сеанса. Для [пользовательских subagents](/docs/ru/sub-agents) это поле `name` из frontmatter агента, а не имя файла. Для subagents, поставляемых [plugin](/docs/ru/plugins), это идентификатор с областью видимости плагина, такой как `my-plugin:reviewer`, а не простое имя frontmatter. См. [SubagentStart](#subagentstart) для того, как написать matcher для имени с областью видимости плагина. |

Только hooks [`SessionStart`](#sessionstart) могут получать поле `model`, и его присутствие не гарантировано. Нет переменной окружения `$CLAUDE_MODEL`. Процесс hook наследует родительское окружение, поэтому он может читать `$ANTHROPIC_MODEL`, если вы установили её в вашей оболочке, но это значение не меняется при переключении моделей с `/model` во время сеанса. Один набор переменных не наследуется: Claude Code [удаляет переменные экспортера `OTEL_*` из каждого подпроцесса, который он порождает](/docs/ru/monitoring-usage#administrator-configuration), включая hooks.

Например, hook `PreToolUse` для команды Bash получает это на stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Поля `tool_name` и `tool_input` специфичны для события. Каждый раздел [hook event](#hook-events) документирует дополнительные поля для этого события.

<h3 id="exit-code-output">
  Выходные коды выхода
</h3>

Код выхода из вашей команды hook говорит Claude Code, должно ли действие продолжаться, быть заблокировано или быть проигнорировано.

**Exit 0** означает успех. Claude Code анализирует stdout для [JSON полей выхода](#json-output). JSON выход обрабатывается только при exit 0. Для большинства событий stdout записывается в журнал отладки, но не показывается в транскрипте. Исключения — `UserPromptSubmit`, `UserPromptExpansion` и `SessionStart`, где stdout добавляется как контекст, который Claude может видеть и действовать.

**Exit 2** означает блокирующую ошибку. Claude Code игнорирует stdout и любой JSON в нём. Вместо этого текст stderr передаётся обратно Claude как сообщение об ошибке. Эффект зависит от события: `PreToolUse` блокирует вызов инструмента, `UserPromptSubmit` отклоняет подсказку и так далее. См. [exit code 2 behavior](#exit-code-2-behavior-per-event) для полного списка.

**Любой другой код выхода** — это неблокирующая ошибка для большинства событий hook. Транскрипт показывает уведомление об ошибке `<hook name> hook error`, за которым следует первая строка stderr, поэтому вы можете определить причину без `--debug`. Выполнение продолжается и полный stderr записывается в журнал отладки.

Например, скрипт команды hook, который блокирует опасные команды Bash:

```bash theme={null}
#!/bin/bash
# Читает JSON входные данные из stdin, проверяет команду
command=$(jq -r '.tool_input.command' < /dev/stdin)

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Blocking error: tool call is prevented
fi

exit 0  # No decision: the normal permission flow applies
```

<Warning>
  Для большинства событий hook только exit code 2 блокирует действие. Claude Code рассматривает exit code 1 как неблокирующую ошибку и продолжает действие, даже хотя 1 — это обычный код отказа Unix. Если ваш hook предназначен для обеспечения политики, используйте `exit 2`. Исключение — `WorktreeCreate`, где любой ненулевой код выхода прерывает создание worktree.
</Warning>

<h4 id="exit-code-2-behavior-per-event">
  Поведение exit code 2 для каждого события
</h4>

Exit code 2 — это способ hook сигнализировать "стоп, не делай этого". Эффект зависит от события, потому что некоторые события представляют действия, которые могут быть заблокированы (например, вызов инструмента, который ещё не произошёл), а другие представляют вещи, которые уже произошли или не могут быть предотвращены.

| Hook событие          | Может блокировать? | Что происходит при exit 2                                                                                                                                                 |
| :-------------------- | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `PreToolUse`          | Да                 | Блокирует вызов инструмента                                                                                                                                               |
| `PermissionRequest`   | Да                 | Отклоняет разрешение                                                                                                                                                      |
| `UserPromptSubmit`    | Да                 | Блокирует обработку подсказки и стирает подсказку                                                                                                                         |
| `UserPromptExpansion` | Да                 | Блокирует расширение                                                                                                                                                      |
| `Stop`                | Да                 | Предотвращает остановку Claude, продолжает разговор                                                                                                                       |
| `SubagentStop`        | Да                 | Предотвращает остановку subagent                                                                                                                                          |
| `TeammateIdle`        | Да                 | Предотвращает переход товарища в режим ожидания, так что он продолжает работать                                                                                           |
| `TaskCreated`         | Да                 | Откатывает создание задачи                                                                                                                                                |
| `TaskCompleted`       | Да                 | Предотвращает отметку задачи как завершённой                                                                                                                              |
| `ConfigChange`        | Да                 | Блокирует применение изменения конфигурации (кроме `policy_settings`)                                                                                                     |
| `StopFailure`         | Нет                | Выход и код выхода игнорируются                                                                                                                                           |
| `PostToolUse`         | Нет                | Показывает stderr Claude; инструмент уже запустился                                                                                                                       |
| `PostToolUseFailure`  | Нет                | Показывает stderr Claude; инструмент уже не удался                                                                                                                        |
| `PostToolBatch`       | Да                 | Останавливает цикл агента перед следующим вызовом модели                                                                                                                  |
| `PermissionDenied`    | Нет                | Код выхода и stderr игнорируются, потому что отказ уже произошёл. Используйте JSON `hookSpecificOutput.retry: true` для сообщения модели, что она может повторить попытку |
| `Notification`        | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `SubagentStart`       | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `SessionStart`        | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `Setup`               | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `SessionEnd`          | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `CwdChanged`          | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `FileChanged`         | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `PreCompact`          | Да                 | Блокирует компактирование                                                                                                                                                 |
| `PostCompact`         | Нет                | Показывает stderr только пользователю                                                                                                                                     |
| `Elicitation`         | Да                 | Отклоняет elicitation                                                                                                                                                     |
| `ElicitationResult`   | Да                 | Блокирует ответ (действие становится decline)                                                                                                                             |
| `WorktreeCreate`      | Да                 | Любой ненулевой код выхода вызывает сбой создания worktree                                                                                                                |
| `WorktreeRemove`      | Нет                | Сбои логируются только в режиме отладки                                                                                                                                   |
| `InstructionsLoaded`  | Нет                | Код выхода игнорируется                                                                                                                                                   |
| `MessageDisplay`      | Нет                | Исходный текст отображается                                                                                                                                               |

Для `SessionStart`, `Setup` и `SubagentStart` stderr exit code 2 отображается в транскрипте как уведомление об ошибке `<hook name> hook error`, так же как [неблокирующая ошибка](#exit-code-output). Claude не видит это, и сеанс или subagent продолжается. Для `SubagentStart` уведомление появляется в собственном транскрипте subagent, а не в родительском разговоре.

Начиная с Claude Code v2.1.199, `SessionStart`, `Setup` и `SubagentStart` показывают stderr exit code 2 в транскрипте. Более ранние версии записывали это только в журнал отладки.

<h3 id="http-response-handling">
  Обработка HTTP ответа
</h3>

HTTP hooks используют коды статуса HTTP и тела ответов вместо кодов выхода и stdout:

* **2xx с пустым телом**: успех, эквивалентно exit code 0 без выхода
* **2xx с телом простого текста**: успех, текст добавляется как контекст
* **2xx с JSON телом**: успех, анализируется с использованием той же [JSON выхода](#json-output) схемы, что и command hooks
* **Статус не 2xx**: неблокирующая ошибка, выполнение продолжается
* **Сбой соединения или таймаут**: неблокирующая ошибка, выполнение продолжается

В отличие от command hooks, HTTP hooks не могут сигнализировать блокирующую ошибку только через коды статуса. Чтобы заблокировать вызов инструмента или отклонить разрешение, верните ответ 2xx с JSON телом, содержащим соответствующие поля решения.

<h3 id="json-output">
  JSON выход
</h3>

Коды выхода позволяют вам разрешить или заблокировать, но JSON выход даёт вам более точное управление. Вместо выхода с кодом 2 для блокировки, выйдите с 0 и выведите JSON объект на stdout. Claude Code читает специфические поля из этого JSON для управления поведением, включая [decision control](#decision-control) для блокировки, разрешения или эскалации пользователю.

<Note>
  Вы должны выбрать один подход на hook, не оба: либо используйте коды выхода отдельно для сигнализации, либо выйдите с 0 и выведите JSON для структурированного управления. Claude Code обрабатывает JSON только при exit 0. Если вы выйдете с 2, любой JSON игнорируется.
</Note>

Stdout вашего hook должен содержать только JSON объект. Если ваш профиль оболочки выводит текст при запуске, это может помешать анализу JSON. См. [JSON validation failed](/docs/ru/hooks-guide#json-validation-failed) в руководстве по устранению неполадок.

Выходные строки hook, включая `additionalContext`, `systemMessage` и простой stdout, ограничены 10 000 символами. Выход, превышающий этот лимит, сохраняется в файл и заменяется предпросмотром и путём к файлу, так же как обрабатываются большие результаты инструментов.

JSON объект поддерживает три вида полей:

* **Универсальные поля** как `continue` работают во всех событиях. Они перечислены в таблице ниже.
* **Верхнеуровневые `decision` и `reason`** используются некоторыми событиями для блокировки или предоставления обратной связи.
* **`hookSpecificOutput`** — это вложенный объект для событий, которым нужно более богатое управление. Он требует поле `hookEventName`, установленное на имя события.

| Поле               | По умолчанию | Описание                                                                                                                                                                                                                                                                                                                                                                  |
| :----------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `continue`         | `true`       | Если `false`, Claude полностью прекращает обработку после запуска hook. Имеет приоритет над любыми полями решения, специфичными для события                                                                                                                                                                                                                               |
| `stopReason`       | нет          | Сообщение, показываемое пользователю при `continue` равном `false`. Не показывается Claude                                                                                                                                                                                                                                                                                |
| `suppressOutput`   | `false`      | Если `true`, скрывает stdout hook из транскрипта. Stdout всё ещё появляется в журнале отладки                                                                                                                                                                                                                                                                             |
| `systemMessage`    | нет          | Предупреждающее сообщение, показываемое пользователю                                                                                                                                                                                                                                                                                                                      |
| `terminalSequence` | нет          | Escape последовательность терминала для Claude Code, которую нужно выдать от вашего имени, такая как уведомление рабочего стола, заголовок окна или звуковой сигнал. Ограничено OSC `0`/`1`/`2`/`9`/`99`/`777` и BEL. Если значение содержит что-либо вне списка разрешённых, поле игнорируется. Используйте это вместо записи в `/dev/tty`, которая недоступна для hooks |

Чтобы полностью остановить Claude независимо от типа события:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

<h4 id="emit-terminal-notifications">
  Выдача уведомлений терминала
</h4>

Поле `terminalSequence` требует Claude Code v2.1.141 или позже.

Hooks запускаются без управляющего терминала, поэтому запись escape последовательностей непосредственно в `/dev/tty` не удаётся. Вместо этого верните escape последовательность в поле `terminalSequence` и Claude Code выдаст её от вашего имени через собственный путь записи терминала. Это свободно от гонок, работает внутри tmux и GNU screen, и работает на Windows, где нет `/dev/tty`.

Поле принимает строку из одной или нескольких разрешённых escape последовательностей:

* OSC `0`, `1`, `2`: заголовки окна и значков
* OSC `9`: уведомления iTerm2, ConEmu, Windows Terminal и WezTerm, включая `9;4` прогресс панели задач
* OSC `99`: уведомления Kitty
* OSC `777`: уведомления urxvt, Ghostty и Warp
* Bare BEL

Последовательности могут быть завершены BEL или ST. Что-либо вне списка разрешённых, включая CSI курсор и цветовые последовательности, OSC палитру последовательности, OSC 8 гиперссылки, OSC 52 записи буфера обмена и OSC 1337, отклоняется и поле игнорируется.

Пример ниже срабатывает уведомление рабочего стола из hook `Notification`. Escape последовательность строится с `printf` восьмеричными экранами, поэтому управляющие байты никогда не появляются в командной строке оболочки, и `jq -n --arg` строит JSON выход, поэтому кавычки, обратные слэши и новые строки в сообщении уведомления правильно экранируются:

```bash theme={null}
#!/bin/bash
# Notification hook: ping the desktop when Claude Code needs attention.
input=$(cat)
title="Claude Code'
body=$(jq -r '.message // 'Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

Форма `{ "terminalSequence": "..." }` одинакова из любой оболочки или языка. На Windows постройте escape строку в PowerShell или скрипте и выдайте тот же JSON объект.

<Note>
  `terminalSequence` — это поддерживаемая замена для hooks, которые ранее писали escape последовательности непосредственно в `/dev/tty`. Список разрешённых ограничен последовательностями, которые не могут перемещать курсор или изменять цвета, поэтому hook никогда не может повредить подсказку на экране.
</Note>

<h4 id="add-context-for-claude">
  Добавить контекст для Claude
</h4>

Поле `additionalContext` передаёт строку из вашего hook в контекстное окно Claude. Claude Code оборачивает строку в системное напоминание и вставляет её в разговор в точке, где сработал hook. Claude читает напоминание при следующем запросе модели, но оно не появляется как сообщение чата в интерфейсе.

Верните `additionalContext` внутри `hookSpecificOutput` рядом с именем события:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

Где появляется напоминание, зависит от события:

* [SessionStart](#sessionstart), [Setup](#setup) и [SubagentStart](#subagentstart): в начале разговора, перед первой подсказкой
* [UserPromptSubmit](#userpromptsubmit) и [UserPromptExpansion](#userpromptexpansion): рядом с отправленной подсказкой
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) и [PostToolBatch](#posttoolbatch): рядом с результатом инструмента
* [Stop](#stop) и [SubagentStop](#subagentstop): в конце хода. Разговор продолжается, поэтому Claude может действовать на основе обратной связи. См. [Stop decision control](#stop-decision-control)

Когда несколько hooks возвращают `additionalContext` для одного события, Claude получает все значения. Если значение превышает 10 000 символов, Claude Code записывает полный текст в файл в каталоге сеанса и передаёт Claude путь к файлу с кратким предпросмотром вместо этого.

Используйте `additionalContext` для информации, которую Claude должен знать о текущем состоянии вашей среды или операции, которая только что запустилась:

* **Состояние среды**: текущая ветка, цель развёртывания или активные флаги функций
* **Условные правила проекта**: какая команда тестирования применяется к только что отредактированному файлу, какие каталоги доступны только для чтения в этом worktree
* **Внешние данные**: открытые проблемы, назначенные вам, недавние результаты CI, содержимое, полученное из внутреннего сервиса

Для инструкций, которые никогда не меняются, предпочитайте [CLAUDE.md](/docs/ru/memory). Он загружается без запуска скрипта и является стандартным местом для статических соглашений проекта.

Напишите текст как фактические утверждения, а не как императивные системные инструкции. Формулировки такие как "Цель развёртывания — production" или "Этот репозиторий использует `bun test`" читаются как информация о проекте. Текст, сформулированный как внеполосные системные команды, может активировать защиту Claude от внедрения подсказок, что заставляет Claude вывести текст вам вместо того, чтобы рассматривать его как контекст.

После внедрения текст сохраняется в транскрипте сеанса. Для событий в середине сеанса, таких как `PostToolUse` или `UserPromptSubmit`, возобновление с `--continue` или `--resume` воспроизводит сохранённый текст вместо повторного запуска hook для прошлых ходов, поэтому значения, такие как временные метки или SHA коммитов, становятся устаревшими при возобновлении. Hooks `SessionStart` запускаются снова при возобновлении с `source` установленным на `"resume"`, поэтому они могут обновить свой контекст.

<h4 id="decision-control">
  Управление решением
</h4>

Не каждое событие поддерживает блокировку или управление поведением через JSON. События, которые это делают, каждое использует другой набор полей для выражения этого решения. Используйте эту таблицу как быструю ссылку перед написанием hook:

| События                                                                                                                             | Шаблон решения                  | Ключевые поля                                                                                                                                                                                                                                    |
| :---------------------------------------------------------------------------------------------------------------------------------- | :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | Верхнеуровневое `decision`      | `decision: "block"`, `reason`. Stop и SubagentStop также принимают `hookSpecificOutput.additionalContext` для [неошибочной обратной связи, которая продолжает разговор](#stop-decision-control)                                                  |
| TeammateIdle, TaskCreated, TaskCompleted                                                                                            | Exit code или `continue: false` | Exit code 2 блокирует действие с обратной связью stderr. JSON `{"continue": false, "stopReason": "..."}` также полностью останавливает товарища, соответствуя поведению hook `Stop`                                                              |
| PreToolUse                                                                                                                          | `hookSpecificOutput`            | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                          |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`            | `decision.behavior` (allow/deny)                                                                                                                                                                                                                 |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`            | `retry: true` говорит модели, что она может повторить попытку отклонённого вызова инструмента                                                                                                                                                    |
| WorktreeCreate                                                                                                                      | path return                     | Command hook выводит путь на stdout; HTTP hook возвращает `hookSpecificOutput.worktreePath`. Сбой hook или отсутствие пути вызывает сбой создания                                                                                                |
| Elicitation                                                                                                                         | `hookSpecificOutput`            | `action` (accept/decline/cancel), `content` (значения полей формы для accept)                                                                                                                                                                    |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`            | `action` (accept/decline/cancel), `content` (переопределение значений полей формы)                                                                                                                                                               |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`            | `displayContent` заменяет отображаемый текст на экране. Только отображение: транскрипт и то, что видит Claude, сохраняют исходный                                                                                                                |
| SessionStart, Setup, SubagentStart                                                                                                  | Только контекст                 | `hookSpecificOutput.additionalContext` добавляет контекст для Claude. SessionStart также принимает [`initialUserMessage`, `watchPaths`, `sessionTitle` и `reloadSkills`](#sessionstart-decision-control). Нет блокировки или управления решением |
| WorktreeRemove, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, FileChanged                     | Нет                             | Нет управления решением. Используется для побочных эффектов, таких как логирование или очистка                                                                                                                                                   |

Несколько событий также могут переписывать содержимое, а не только разрешать или блокировать его:

* `PreToolUse`: `updatedInput` непосредственно под `hookSpecificOutput` заменяет аргументы инструмента перед его запуском. См. [PreToolUse decision control](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` внутри объекта `decision`. См. [PermissionRequest decision control](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` заменяет результат инструмента. См. [PostToolUse decision control](#posttooluse-decision-control)
* `UserPromptSubmit`: не может заменить подсказку; только внедряет `additionalContext` рядом с ней

Для редактирования или трансформации используйте перехват на `PreToolUse` для исходящих входных данных инструмента и `PostToolUse` для входящих результатов инструмента.

Вот примеры каждого шаблона в действии:

<Tabs>
  <Tab title="Top-level decision">
    Используется `UserPromptSubmit`, `UserPromptExpansion`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `Stop`, `SubagentStop`, `ConfigChange` и `PreCompact`. Единственное значение — `"block"`. Чтобы разрешить действию продолжаться, опустите `decision` из вашего JSON или выйдите с 0 без какого-либо JSON вообще:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Использует `hookSpecificOutput` для более богатого управления: разрешить, отклонить или отложить. Вы также можете изменить входные данные инструмента перед его запуском или внедрить дополнительный контекст для Claude. См. [PreToolUse decision control](#pretooluse-decision-control) для полного набора параметров.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Использует `hookSpecificOutput` для разрешения или отклонения запроса разрешения от имени пользователя. При разрешении вы также можете изменить входные данные инструмента или применить правила разрешения, чтобы пользователю не было предложено снова. См. [PermissionRequest decision control](#permissionrequest-decision-control) для полного набора параметров.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Для расширенных примеров, включая валидацию команд Bash, фильтрацию подсказок и скрипты автоматического одобрения, см. [What you can automate](/docs/ru/hooks-guide#what-you-can-automate) в руководстве и [Bash command validator reference implementation](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Hook events
</h2>

Каждое событие соответствует точке в жизненном цикле Claude Code, где могут запускаться hooks. Разделы ниже упорядочены в соответствии с жизненным циклом: от настройки сеанса через агентный цикл к концу сеанса. Каждый раздел описывает, когда срабатывает событие, какие фильтры оно поддерживает, JSON входные данные, которые оно получает, и как управлять поведением через выход.

<h3 id="sessionstart">
  SessionStart
</h3>

Запускается при запуске Claude Code нового сеанса или возобновлении существующего сеанса. Полезно для загрузки контекста разработки, такого как существующие проблемы или недавние изменения в вашей кодовой базе, или установки переменных окружения. Для статического контекста, который не требует скрипта, используйте [CLAUDE.md](/docs/ru/memory) вместо этого.

SessionStart запускается при каждом сеансе, поэтому держите эти hooks быстрыми. Поддерживаются только hooks `type: "command"` и `type: "mcp_tool"`.

Значение фильтра соответствует тому, как был инициирован сеанс:

| Фильтр    | Когда он срабатывает                      |
| :-------- | :---------------------------------------- |
| `startup` | Новый сеанс                               |
| `resume`  | `--resume`, `--continue` или `/resume`    |
| `clear`   | `/clear`                                  |
| `compact` | Автоматическое или ручное компактирование |

<h4 id="sessionstart-input">
  SessionStart input
</h4>

В дополнение к [общим полям входа](#common-input-fields), SessionStart hooks получают `source` и опционально `model`, `agent_type` и `session_title`:

| Поле            | Описание                                                                                                                                                                                                                                        |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Как был запущен сеанс: `"startup"` для новых сеансов, `"resume"` для возобновлённых сеансов, `"clear"` после `/clear` или `"compact"` после компактирования                                                                                     |
| `model`         | Идентификатор активной модели. Может быть опущено, например после `/clear` или когда сеанс восстанавливается через восстановление разговора, поэтому проверьте поле перед его чтением                                                           |
| `agent_type`    | Имя агента, присутствует при запуске Claude Code с `claude --agent <name>`                                                                                                                                                                      |
| `session_title` | Текущее название сеанса, если оно уже установлено, например через `--name` или `/rename`. Hook, который выдаёт `sessionTitle`, может сначала проверить `session_title`, чтобы избежать перезаписи названия, которое пользователь установил явно |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-5"
}
```

<h4 id="sessionstart-decision-control">
  SessionStart decision control
</h4>

Любой текст, который ваш скрипт hook выводит на stdout, добавляется как контекст для Claude. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, вы можете вернуть эти поля, специфичные для события:

| Поле                 | Описание                                                                                                                                                                                                                                                                                                                                                              |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Строка, добавленная в контекст Claude в начале разговора, перед первой подсказкой. См. [Add context for Claude](#add-context-for-claude) для того, как текст доставляется и что в него поместить                                                                                                                                                                      |
| `initialUserMessage` | Строка, используемая как первое сообщение пользователя сеанса. Применяется в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`, где оно становится первым ходом, даже если подсказка не предоставлена. Если подсказка предоставлена, она следует как следующий ход. В отличие от `additionalContext`, который присоединяется к существующему ходу, это создаёт ход |
| `sessionTitle`       | Устанавливает название сеанса, с тем же эффектом, что и `/rename`. Используйте для автоматического именования сеансов из папки запуска, ветки git или имени worktree. Применяется только когда `source` равен `"startup"` или `"resume"`; игнорируется на `"clear"` и `"compact"`                                                                                     |
| `watchPaths`         | Массив абсолютных путей для отслеживания событий [FileChanged](#filechanged) во время этого сеанса                                                                                                                                                                                                                                                                    |
| `reloadSkills`       | Логическое значение. Когда `true`, Claude Code повторно сканирует каталоги [skill](/docs/ru/skills) и команд после завершения SessionStart hooks, поэтому skills, которые установил hook, доступны в том же сеансе, начиная с первой подсказки                                                                                                                             |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Поскольку простой stdout уже достигает Claude для этого события, hook, который только загружает контекст, может выводить на stdout напрямую без построения JSON. Используйте форму JSON, когда вам нужно объединить контекст с другими полями, такими как `suppressOutput` или `sessionTitle`.

Используйте `reloadSkills`, когда hook SessionStart устанавливает или обновляет skills. Обнаружение skills обычно запускается перед завершением SessionStart hooks, поэтому файлы, которые hook записывает в `~/.claude/skills/` или `.claude/skills/`, в противном случае появились бы только в следующем сеансе. Этот пример синхронизирует репозиторий общих skills и запрашивает повторное сканирование:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

<h4 id="persist-environment-variables">
  Persist environment variables
</h4>

SessionStart hooks имеют доступ к переменной окружения `CLAUDE_ENV_FILE`, которая предоставляет путь к файлу, где вы можете сохранять переменные окружения для последующих команд Bash.

Чтобы установить отдельные переменные окружения, напишите операторы `export` в `CLAUDE_ENV_FILE`. Используйте добавление (`>>`) для сохранения переменных, установленных другими hooks:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Чтобы захватить все изменения окружения из команд настройки, сравните экспортированные переменные до и после:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Run your setup commands that modify the environment
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Любые переменные, написанные в этот файл, будут доступны во всех последующих командах Bash, которые Claude Code выполняет во время сеанса.

<Note>
  `CLAUDE_ENV_FILE` доступен для SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) и [FileChanged](#filechanged) hooks. Другие типы hooks не имеют доступа к этой переменной.
</Note>

<h3 id="setup">
  Setup
</h3>

Срабатывает только при запуске Claude Code с `--init-only` или с `--init` или `--maintenance` в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`. Не срабатывает при нормальном запуске. Используйте это для одноразовой установки зависимостей или запланированной очистки, которую вы запускаете явно из CI или скриптов, отдельно от нормального запуска сеанса. Для инициализации для каждого сеанса используйте [SessionStart](#sessionstart) вместо этого.

Значение фильтра соответствует флагу CLI, который запустил hook:

| Фильтр        | Когда он срабатывает                        |
| :------------ | :------------------------------------------ |
| `init`        | `claude --init-only` или `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                   |

`--init-only` запускает Setup hooks и SessionStart hooks с фильтром `startup`, затем выходит без запуска разговора. `--init` и `--maintenance` срабатывают Setup hooks только при объединении с `-p`; в интерактивном сеансе эти два флага в настоящее время не срабатывают Setup hooks.

Поскольку Setup не срабатывает при каждом запуске, плагин, которому нужна установленная зависимость, не может полагаться только на Setup. Практический паттерн — проверить зависимость при первом использовании и установить при отсутствии, например hook или skill, который проверяет `${CLAUDE_PLUGIN_DATA}/node_modules` и запускает `npm install` при отсутствии. См. [persistent data directory](/docs/ru/plugins-reference#persistent-data-directory) для того, где хранить установленные зависимости.

<h4 id="setup-input">
  Setup input
</h4>

В дополнение к [общим полям входа](#common-input-fields), Setup hooks получают поле `trigger`, установленное на `"init"` или `"maintenance"`:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Setup decision control
</h4>

Setup hooks не могут блокировать. При exit code 2 stderr показывается пользователю как уведомление об ошибке hook, и выполнение продолжается. В [неинтерактивном режиме](/docs/ru/headless) выход hook появляется только при запуске с `--verbose`. Чтобы передать информацию в контекст Claude, верните `additionalContext` в JSON выходе; простой stdout записывается только в журнал отладки. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, вы можете вернуть эти поля, специфичные для события:

| Поле                | Описание                                                                      |
| :------------------ | :---------------------------------------------------------------------------- |
| `additionalContext` | Строка, добавленная в контекст Claude. Значения нескольких hooks объединяются |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Setup",
    "additionalContext": "Dependencies installed: node_modules, .venv"
  }
}
```

Setup hooks имеют доступ к `CLAUDE_ENV_FILE`. Переменные, написанные в этот файл, сохраняются в последующих командах Bash для сеанса, как и в [SessionStart hooks](#persist-environment-variables). Поддерживаются только hooks `type: "command"` и `type: "mcp_tool"`.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Срабатывает при загрузке файла `CLAUDE.md` или `.claude/rules/*.md` в контекст. Это событие срабатывает при запуске сеанса для нетерпеливо загруженных файлов и снова позже при ленивой загрузке, например когда Claude получает доступ к подкаталогу, содержащему вложенный `CLAUDE.md`, или когда условные правила с frontmatter `paths:` совпадают. Hook не поддерживает блокировку или управление решением. Он запускается асинхронно в целях наблюдаемости.

Фильтр запускается против `load_reason`. Например, используйте `"matcher": "session_start"` для срабатывания только для файлов, загруженных при запуске сеанса, или `"matcher": "path_glob_match|nested_traversal"` для срабатывания только для ленивых загрузок.

<h4 id="instructionsloaded-input">
  InstructionsLoaded input
</h4>

В дополнение к [общим полям входа](#common-input-fields), InstructionsLoaded hooks получают эти поля:

| Поле                | Описание                                                                                                                                                                                                               |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Абсолютный путь к файлу инструкций, который был загружен                                                                                                                                                               |
| `memory_type`       | Область действия файла: `"User"`, `"Project"`, `"Local"` или `"Managed"`                                                                                                                                               |
| `load_reason`       | Почему файл был загружен: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` или `"compact"`. Значение `"compact"` срабатывает при перезагрузке файлов инструкций после события компактирования |
| `globs`             | Шаблоны glob пути из frontmatter `paths:` файла, если есть. Присутствует только для загрузок `path_glob_match`                                                                                                         |
| `trigger_file_path` | Путь к файлу, доступ к которому вызвал эту загрузку, для ленивых загрузок                                                                                                                                              |
| `parent_file_path`  | Путь к родительскому файлу инструкций, который включил этот, для загрузок `include`                                                                                                                                    |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  InstructionsLoaded decision control
</h4>

InstructionsLoaded hooks не имеют управления решением. Они не могут блокировать или изменять загрузку инструкций. Используйте это событие для аудита логирования, отслеживания соответствия или наблюдаемости.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Запускается при отправке пользователем подсказки, перед обработкой Claude. Это позволяет вам добавить дополнительный контекст на основе подсказки/разговора, проверить подсказки или заблокировать определённые типы подсказок.

Hooks `UserPromptSubmit` имеют таймаут по умолчанию 30 секунд для типов `command`, `http` и `mcp_tool`, что короче, чем таймаут по умолчанию 600 секунд для этих типов на других событиях. Поскольку этот hook запускается перед каждой подсказкой и блокирует обработку модели до его завершения, застрявший hook замораживает сеанс. Если вашему hook нужно больше времени, установите поле `timeout` в записи hook.

Hook `UserPromptSubmit`, который достигает своего таймаута, отменяется и его выход, включая любой `additionalContext`, отбрасывается. Подсказка всё ещё достигает Claude без этого контекста. Начиная с v2.1.196, транскрипт показывает уведомление, называющее hook, таймаут, который сработал, и что выход был отброшен. Более ранние версии отменяют hook без уведомления.

Hook [Agent SDK callback](/docs/ru/agent-sdk/hooks) на `UserPromptSubmit`, который достигает своего таймаута, блокирует подсказку с сообщением, называющим hook и таймаут, потому что callback там может действовать как политический шлюз, который не должен отказать открыто. Сеанс продолжается. До v2.1.208 таймаут callback на этом событии заканчивал ход с ошибкой выполнения.

<h4 id="userpromptsubmit-input">
  UserPromptSubmit input
</h4>

В дополнение к [общим полям входа](#common-input-fields), UserPromptSubmit hooks получают поле `prompt`, содержащее текст, отправленный пользователем.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  UserPromptSubmit decision control
</h4>

Hooks `UserPromptSubmit` могут управлять тем, обрабатывается ли подсказка пользователя, и добавлять контекст. Доступны все [JSON поля выхода](#json-output).

Есть два способа добавить контекст в разговор при exit code 0:

* **Простой текст stdout**: любой текст, не являющийся JSON, написанный на stdout, добавляется как контекст
* **JSON с `additionalContext`**: используйте формат JSON ниже для большего управления. Поле `additionalContext` добавляется как контекст

Простой stdout показывается как выход hook в транскрипте. Значение `additionalContext` внедряется как системное напоминание, которое Claude читает без видимой записи в транскрипте.

Чтобы заблокировать подсказку, верните JSON объект с `decision`, установленным на `"block"`:

| Поле                     | Описание                                                                                                                           |
| :----------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` предотвращает обработку подсказки и стирает её из контекста. Опустите, чтобы разрешить подсказке продолжаться            |
| `reason`                 | Показывается пользователю при `decision` равном `"block"`. Не добавляется в контекст                                               |
| `additionalContext`      | Строка, добавленная в контекст Claude наряду с отправленной подсказкой. См. [Add context for Claude](#add-context-for-claude)      |
| `sessionTitle`           | Устанавливает название сеанса. Используйте для автоматического именования сеансов на основе содержимого подсказки                  |
| `suppressOriginalPrompt` | Если `true` при `decision` равном `"block"`, опускает исходный текст подсказки из сообщения блокировки, показываемого пользователю |

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

Запускается, когда пользователь вводит slash command, который расширяется в подсказку перед достижением Claude. Используйте это для блокировки определённых команд от прямого вызова, внедрения контекста для определённого skill или логирования, какие команды вызывают пользователи. Например, hook, соответствующий `deploy`, может заблокировать `/deploy`, если файл одобрения отсутствует, или hook, соответствующий skill проверки, может добавить контрольный список проверки команды как `additionalContext`.

Это событие охватывает путь, который `PreToolUse` не охватывает: hook `PreToolUse`, соответствующий инструменту `Skill`, срабатывает только когда Claude вызывает инструмент, но ввод `/skillname` напрямую обходит `PreToolUse`. `UserPromptExpansion` срабатывает на этом прямом пути.

Совпадает с `command_name`. Оставьте фильтр пустым для срабатывания на каждой подсказке-типе slash command.

<h4 id="userpromptexpansion-input">
  UserPromptExpansion input
</h4>

В дополнение к [общим полям входа](#common-input-fields), UserPromptExpansion hooks получают `expansion_type`, `command_name`, `command_args`, `command_source` и исходную строку `prompt`. Поле `expansion_type` равно `slash_command` для skill и пользовательских команд или `mcp_prompt` для подсказок MCP сервера.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  UserPromptExpansion decision control
</h4>

Hooks `UserPromptExpansion` могут блокировать расширение или добавлять контекст. Доступны все [JSON поля выхода](#json-output).

| Поле                | Описание                                                                                                                     |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` предотвращает расширение slash command. Опустите, чтобы разрешить ему продолжаться                                 |
| `reason`            | Показывается пользователю при `decision` равном `"block"`                                                                    |
| `additionalContext` | Строка, добавленная в контекст Claude наряду с расширенной подсказкой. См. [Add context for Claude](#add-context-for-claude) |

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

Запускается во время потоковой передачи сообщения помощника на экран. Claude Code отображает сообщение порциями: каждый раз, когда пакет новых завершённых строк готов к отрисовке, hook запускается один раз с этими строками, и Claude Code отображает текст замены hook вместо них. Длинное сообщение производит несколько вызовов; короткое сообщение может произвести только один.

Используйте MessageDisplay для:

* удаления markdown для минимального отображения
* преобразования текста, который приложение Agent SDK показывает своим пользователям
* редактирования API ключей или внутренних имён хостов из ответов Claude

Claude Code удерживает каждый пакет до возврата вашего hook, поэтому держите hook быстрым. Если hook не удаётся или истекает время ожидания, Claude Code отображает исходный текст. Таймаут по умолчанию для этого события составляет 10 секунд; если вашему hook нужно больше времени, установите поле `timeout` в записи hook.

MessageDisplay предназначен только для отображения: текст замены изменяет только то, что отрисовывается на экране. Транскрипт и то, что видит Claude, сохраняют исходный текст, поэтому Claude никогда не видит замену, и подробный режим показывает исходный. Hook получает только текст сообщения помощника, поэтому результаты инструментов и текст, который вы вводите, отрисовываются без изменений.

MessageDisplay не поддерживает фильтры и срабатывает для каждого сообщения помощника, которое потоком передаёт текст; сообщения без текста, такие как ответы только с вызовом инструмента, не запускают его.

В неинтерактивных запусках, включая запросы Agent SDK и `claude -p`, MessageDisplay запускается один раз за сообщение помощника вместо один раз за пакет строк. Единственный вызов приходит после завершения сообщения и содержит полный текст сообщения: `index` равен `0`, `final` равен `true`, и `delta` содержит всё сообщение. Hook, который собирает текст `delta` для каждого сообщения, получает одинаковый общий текст в обоих режимах.

<h4 id="messagedisplay-input">
  MessageDisplay input
</h4>

В дополнение к [общим полям входа](#common-input-fields), MessageDisplay hooks получают идентификаторы для хода и сообщения, позицию этого вызова в сообщении и новый текст в `delta`. Границы пакетов зависят от того, как текст потоком передаётся, поэтому используйте `index` и `final` для отслеживания прогресса через сообщение, а не ожидайте, что строки будут сгруппированы определённым образом.

| Поле         | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID текущего хода                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `message_id` | UUID сообщения помощника, которое отображается. Стабилен во всех пакетах одного сообщения. Это не API `msg_…` id, поэтому его нельзя коррелировать с id сообщений транскрипта                                                                                                                                                                                                                                                                                |
| `index`      | Нулевой индекс этого пакета в сообщении                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `final`      | `true` на последнем пакете сообщения. Каждое сообщение имеет ровно один финальный пакет                                                                                                                                                                                                                                                                                                                                                                      |
| `delta`      | Новые завершённые строки с момента предыдущего пакета, включая завершающие новые строки. Всегда целые строки, кроме финального пакета, который может заканчиваться в середине строки. В интерактивных запусках дельта финального пакета пуста, когда сообщение заканчивается на новой строке, поэтому рассматривайте `final`, а не непустую дельту, как сигнал конца сообщения. В запусках Agent SDK и `claude -p` единственный вызов содержит всё сообщение |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  MessageDisplay output
</h4>

В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, MessageDisplay hooks могут вернуть `displayContent` для замены дельты на экране:

| Поле             | Описание                                                              |
| :--------------- | :-------------------------------------------------------------------- |
| `displayContent` | Текст, отображаемый вместо дельты. Опустите для отображения исходного |

MessageDisplay hooks не имеют управления решением. Они не могут блокировать сообщение или изменять то, что хранится в транскрипте или отправляется Claude.

Этот пример удаляет форматирование markdown из ответов Claude для отображения в виде простого текста. Скрипт читает каждый пакет из stdin, удаляет маркеры жирного шрифта и обратные кавычки встроенного кода из `delta` и возвращает результат как `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Зарегистрируйте command hook для события в файле настроек:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Сохраните этот скрипт в `.claude/hooks/plain-display.sh` в вашем проекте и сделайте его исполняемым с помощью `chmod +x`:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```

    Скрипту нужен `jq` в вашем `PATH`.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Зарегистрируйте command hook, который запускает скрипт через PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Флаг `-NoProfile` пропускает загрузку вашего профиля PowerShell, чтобы hook запустился быстро, а `-ExecutionPolicy Bypass` позволяет PowerShell запустить локальный файл скрипта.

    Сохраните этот скрипт в `.claude/hooks/plain-display.ps1` в вашем проекте:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

Пакеты без markdown проходят без изменений. Если скрипт не удаётся, например потому что `jq` отсутствует, Claude Code отображает исходный текст и отмечает сбой только в [debug output](#debug-hooks), а не в сеансе.

<h3 id="pretooluse">
  PreToolUse
</h3>

Запускается после того, как Claude создаёт параметры инструмента и перед обработкой вызова инструмента. Совпадает с именем инструмента: `Bash`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `WebFetch`, `WebSearch`, `AskUserQuestion`, `ExitPlanMode` и любые [имена MCP инструментов](#match-mcp-tools).

<Warning>
  PreToolUse запускается только когда Claude вызывает инструмент. Файлы, которые вы [ссылаетесь с `@` в вашей подсказке](/docs/ru/common-workflows#reference-files-and-directories), добавляются без какого-либо вызова инструмента: Claude Code вставляет их содержимое при построении подсказки, поэтому никакой hook PreToolUse не срабатывает для них, включая hooks, соответствующие `Read`. Чтобы заблокировать определённые пути от ссылок `@`, используйте [правило отклонения `Read`](/docs/ru/permissions#read-and-edit) вместо этого.
</Warning>

Используйте [PreToolUse decision control](#pretooluse-decision-control) для разрешения, отклонения, запроса или отложения вызова инструмента.

<h4 id="pretooluse-input">
  PreToolUse input
</h4>

В дополнение к [общим полям входа](#common-input-fields), PreToolUse hooks получают `tool_name`, `tool_input` и `tool_use_id`. Поля `tool_input` зависят от инструмента:

<h5 id="bash">
  Bash
</h5>

Выполняет команды оболочки.

| Поле                | Тип     | Пример             | Описание                                                                                                                                           |
| :------------------ | :------ | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | Команда оболочки для выполнения                                                                                                                    |
| `description`       | string  | `"Run test suite"` | Опциональное описание того, что делает команда                                                                                                     |
| `timeout`           | number  | `120000`           | Опциональный таймаут в миллисекундах. Значения выше [максимума](/docs/ru/tools-reference#bash-tool-behavior) уменьшаются до максимума, а не отклоняются |
| `run_in_background` | boolean | `false`            | Запускать ли команду в фоне                                                                                                                        |

<h5 id="write">
  Write
</h5>

Создаёт или перезаписывает файл.

| Поле        | Тип    | Пример                | Описание                           |
| :---------- | :----- | :-------------------- | :--------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Абсолютный путь к файлу для записи |
| `content`   | string | `"file content"`      | Содержимое для записи в файл       |

<h5 id="edit">
  Edit
</h5>

Заменяет строку в существующем файле.

| Поле          | Тип     | Пример                | Описание                                   |
| :------------ | :------ | :-------------------- | :----------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Абсолютный путь к файлу для редактирования |
| `old_string`  | string  | `"original text"`     | Текст для поиска и замены                  |
| `new_string`  | string  | `"replacement text"`  | Текст замены                               |
| `replace_all` | boolean | `false`               | Заменять ли все вхождения                  |

<h5 id="read">
  Read
</h5>

Читает содержимое файла.

| Поле        | Тип    | Пример                | Описание                                    |
| :---------- | :----- | :-------------------- | :------------------------------------------ |
| `file_path` | string | `"/path/to/file.txt"` | Абсолютный путь к файлу для чтения          |
| `offset`    | number | `10`                  | Опциональный номер строки для начала чтения |
| `limit`     | number | `50`                  | Опциональное количество строк для чтения    |

<h5 id="glob">
  Glob
</h5>

Находит файлы, соответствующие шаблону glob.

| Поле      | Тип    | Пример           | Описание                                                              |
| :-------- | :----- | :--------------- | :-------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Шаблон glob для совпадения файлов                                     |
| `path`    | string | `"/path/to/dir"` | Опциональный каталог для поиска. По умолчанию текущий рабочий каталог |

<h5 id="grep">
  Grep
</h5>

Ищет содержимое файла с регулярными выражениями.

| Поле          | Тип     | Пример           | Описание                                                                               |
| :------------ | :------ | :--------------- | :------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Шаблон регулярного выражения для поиска                                                |
| `path`        | string  | `"/path/to/dir"` | Опциональный файл или каталог для поиска                                               |
| `glob`        | string  | `"*.ts"`         | Опциональный шаблон glob для фильтрации файлов                                         |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` или `"count"`. По умолчанию `"files_with_matches"` |
| `-i`          | boolean | `true`           | Поиск без учёта регистра                                                               |
| `multiline`   | boolean | `false`          | Включить многострочное совпадение                                                      |

<h5 id="webfetch">
  WebFetch
</h5>

Получает и обрабатывает веб-содержимое.

| Поле     | Тип    | Пример                        | Описание                                       |
| :------- | :----- | :---------------------------- | :--------------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL для получения содержимого                  |
| `prompt` | string | `"Extract the API endpoints"` | Подсказка для запуска на полученном содержимом |

<h5 id="websearch">
  WebSearch
</h5>

Ищет в веб.

| Поле              | Тип    | Пример                         | Описание                                                |
| :---------------- | :----- | :----------------------------- | :------------------------------------------------------ |
| `query`           | string | `"react hooks best practices"` | Поисковый запрос                                        |
| `allowed_domains` | array  | `["docs.example.com"]`         | Опциональный: включать результаты только с этих доменов |
| `blocked_domains` | array  | `["spam.example.com"]`         | Опциональный: исключить результаты с этих доменов       |

<h5 id="agent">
  Agent
</h5>

Порождает [subagent](/docs/ru/sub-agents).

| Поле            | Тип    | Пример                     | Описание                                                       |
| :-------------- | :----- | :------------------------- | :------------------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | Задача для выполнения агентом                                  |
| `description`   | string | `"Find API endpoints"`     | Краткое описание задачи                                        |
| `subagent_type` | string | `"Explore"`                | Тип специализированного агента для использования               |
| `model`         | string | `"sonnet"`                 | Опциональный псевдоним модели для переопределения по умолчанию |

В `PostToolUse`, `tool_response` для завершённого вызова Agent содержит финальный текст subagent вместе с телеметрией использования. Читайте эти поля для записи затрат для каждого subagent из hook:

| Поле                | Тип    | Пример                                                | Описание                                                                                                                                                                                                                 |
| :------------------ | :----- | :---------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` для синхронных вызовов, `"async_launched"` для фоновых subagents. Начиная с v2.1.198, subagents запускаются в фоне по умолчанию, поэтому опущенный `run_in_background` также производит `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Идентификатор для запуска subagent                                                                                                                                                                                       |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Финальные текстовые блоки subagent                                                                                                                                                                                       |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Модель, на которой запустился subagent, которая может отличаться от запрошенной модели. Требует Claude Code v2.1.174 или позже                                                                                           |
| `totalTokens`       | number | `12450`                                               | Всего токенов, выставленных счётом по ходам subagent                                                                                                                                                                     |
| `totalDurationMs`   | number | `48211`                                               | Реальная длительность запуска subagent                                                                                                                                                                                   |
| `totalToolUseCount` | number | `7`                                                   | Количество вызовов инструментов, которые сделал subagent                                                                                                                                                                 |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Разбор токенов по типам: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                       |

Для фоновых subagents инструмент возвращается сразу после запуска, поэтому `tool_response` не содержит полей использования. Он имеет `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` и `resolvedModel` вместо этого.

Поле `resolvedModel` называет модель, на которой subagent фактически запустился, что может отличаться от значения `model` в `tool_input`. Оно требует Claude Code v2.1.174 или позже.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Задаёт пользователю один-четыре вопроса с множественным выбором.

| Поле        | Тип    | Пример                                                                                                             | Описание                                                                                                                                                                                                                      |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Вопросы для представления, каждый с текстом `question`, коротким `header`, массивом `options` и опциональным флагом `multiSelect`                                                                                             |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Опциональный. Соответствует текст вопроса выбранному ярлыку опции. Ответы с множественным выбором объединяют ярлыки запятыми. Claude не устанавливает это поле; предоставьте его через `updatedInput` для программного ответа |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Представляет план и просит пользователя одобрить его перед тем, как Claude покинет [plan mode](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode). Claude записывает план в файл на диск перед вызовом инструмента, поэтому буквальный `tool_input` от модели обычно пуст. Claude Code внедряет содержимое плана и путь файла перед передачей входных данных в hooks.

| Поле             | Тип    | Пример                                      | Описание                                                                                                                                                          |
| :--------------- | :----- | :------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Содержимое плана в Markdown. Внедрено из файла плана на диске                                                                                                     |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Путь к файлу плана. Внедрено                                                                                                                                      |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Устарело. Claude Code принимает поле, но игнорирует его. До v2.1.205 оно содержало разрешения на основе подсказки, которые Claude запрашивал для реализации плана |

В `PostToolUse`, `tool_response` — это объект с полями `plan` и `filePath`, содержащими одобренный план, плюс внутренние флаги статуса. Читайте `tool_response.plan` для содержимого плана, а не перечитывайте файл с диска.

<h4 id="pretooluse-decision-control">
  PreToolUse decision control
</h4>

Hooks `PreToolUse` могут управлять тем, продолжается ли вызов инструмента. В отличие от других hooks, которые используют верхнеуровневое поле `decision`, PreToolUse возвращает своё решение внутри объекта `hookSpecificOutput`. Это даёт ему более богатое управление: четыре результата (разрешить, отклонить, спросить или отложить) плюс возможность изменить входные данные инструмента перед выполнением.

| Поле                       | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` пропускает диалог разрешения, кроме [инструментов, требующих взаимодействия пользователя](#pretooluse-decision-control) и инструментов соединителя [ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools). `"deny"` предотвращает вызов инструмента. `"ask"` предлагает пользователю подтвердить. `"defer"` выходит корректно, чтобы инструмент мог быть возобновлён позже. [Правила отклонения и запроса](/docs/ru/permissions#manage-permissions) всё ещё применяются независимо от того, что возвращает hook |
| `permissionDecisionReason` | Для `"allow"` и `"ask"`, показывается пользователю, но не Claude. Для `"deny"`, показывается Claude. Для `"defer"`, игнорируется                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `updatedInput`             | Изменяет параметры входа инструмента перед выполнением. Заменяет весь объект входа, поэтому включите неизменённые поля наряду с изменёнными. Объедините с `"allow"` для автоматического одобрения или `"ask"` для показа изменённого входа пользователю. Для `"defer"`, игнорируется                                                                                                                                                                                                                                                                   |
| `additionalContext`        | Строка, добавленная в контекст Claude наряду с результатом инструмента. Игнорируется при `permissionDecision` равном `"defer"`. См. [Add context for Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                  |

Когда несколько PreToolUse hooks возвращают разные решения, приоритет — `deny` > `defer` > `ask` > `allow`.

Когда hook возвращает `"ask"`, диалог разрешения, отображаемый пользователю, включает метку, идентифицирующую источник hook: например, `[User]`, `[Project]`, `[Plugin]` или `[Local]`. Это помогает пользователям понять, какой источник конфигурации запрашивает подтверждение.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

`AskUserQuestion` и `ExitPlanMode` требуют взаимодействия пользователя и обычно блокируют в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`. Возврат `permissionDecision: "allow"` вместе с `updatedInput` удовлетворяет этому требованию: hook читает входные данные инструмента из stdin, собирает ответ через ваш собственный UI и возвращает его в `updatedInput`, чтобы инструмент запустился без запроса. Возврат только `"allow"` недостаточен для этих инструментов. Для `AskUserQuestion` повторите исходный массив `questions` и добавьте объект [`answers`](#askuserquestion), соответствующий тексту каждого вопроса выбранному ответу.

Инструменты соединителя [ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) запрашивают даже когда hook возвращает `"allow"`.

Начиная с v2.1.199, инструмент MCP, чей сервер помечает его с помощью [`_meta["anthropic/requiresUserInteraction"]`](/docs/ru/mcp#require-approval-for-a-specific-tool), более строг: hook не может пропустить его диалог одобрения с помощью `"allow"`, с `updatedInput` или без, потому что Claude Code не может подтвердить, что hook собрал взаимодействие, которое нужно инструменту.

<Note>
  PreToolUse ранее использовал верхнеуровневые поля `decision` и `reason`, но они устарели для этого события. Используйте `hookSpecificOutput.permissionDecision` и `hookSpecificOutput.permissionDecisionReason` вместо этого. Устаревшие значения `"approve"` и `"block"` соответствуют `"allow"` и `"deny"` соответственно. Другие события, такие как PostToolUse и Stop, продолжают использовать верхнеуровневые `decision` и `reason` как их текущий формат.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Defer a tool call for later
</h4>

`"defer"` предназначен для интеграций, которые запускают `claude -p` как подпроцесс и читают его JSON выход, таких как приложение Agent SDK или пользовательский UI, построенный на основе Claude Code. Это позволяет этому вызывающему процессу приостановить Claude при вызове инструмента, собрать входные данные через его собственный интерфейс и возобновить с того же места. Claude Code соблюдает это значение только в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`. В интерактивных сеансах он логирует предупреждение и игнорирует результат hook.

Инструмент `AskUserQuestion` — это типичный случай: Claude хочет что-то спросить у пользователя, но нет терминала для ответа. Круговой путь работает так:

1. Claude вызывает `AskUserQuestion`. Срабатывает hook `PreToolUse`.
2. Hook возвращает `permissionDecision: "defer"`. Инструмент не выполняется. Процесс выходит с `stop_reason: "tool_deferred"` и отложенный вызов инструмента сохраняется в транскрипте.
3. Вызывающий процесс читает `deferred_tool_use` из результата SDK, выводит вопрос в своём UI и ждёт ответа.
4. Вызывающий процесс запускает `claude -p --resume <session-id>`. Тот же вызов инструмента срабатывает `PreToolUse` снова.
5. Hook возвращает `permissionDecision: "allow"` с ответом в `updatedInput`. Инструмент выполняется и Claude продолжает.

Поле `deferred_tool_use` содержит `id`, `name` и `input` инструмента. `input` — это параметры, которые Claude сгенерировал для вызова инструмента, захваченные перед выполнением:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

Нет таймаута или лимита повторных попыток. Сеанс остаётся на диске до возобновления, в соответствии с операцией очистки [`cleanupPeriodDays`](/docs/ru/settings#available-settings), которая удаляет файлы сеанса через 30 дней по умолчанию. Если ответ не готов при возобновлении, hook может вернуть `"defer"` снова и процесс выходит так же. Вызывающий процесс управляет тем, когда разорвать цикл, в конечном итоге возвращая `"allow"` или `"deny"` из hook.

`"defer"` работает только когда Claude делает один вызов инструмента в ходе. Если Claude делает несколько вызовов инструментов одновременно, `"defer"` игнорируется с предупреждением и инструмент проходит через обычный поток разрешений. Ограничение существует потому что возобновление может только повторно запустить один инструмент: нет способа отложить один вызов из пакета без оставления других неразрешённых.

Если отложенный инструмент больше не доступен при возобновлении, процесс выходит с `stop_reason: "tool_deferred_unavailable"` и `is_error: true` перед срабатыванием hook. Это происходит когда MCP сервер, который предоставил инструмент, не подключен для возобновлённого сеанса. Полезная нагрузка `deferred_tool_use` всё ещё включена, чтобы вы могли идентифицировать, какой инструмент исчез.

<Note>
  `--resume` восстанавливает режим разрешения, который был активен при отложении инструмента, поэтому вам не нужно передавать `--permission-mode` снова. Исключения — это `plan` и `bypassPermissions`, которые никогда не переносятся. Передача `--permission-mode` явно при возобновлении переопределяет восстановленное значение.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Запускается при показе пользователю диалога разрешения.
Используйте [PermissionRequest decision control](#permissionrequest-decision-control) для разрешения или отклонения от имени пользователя.

Совпадает с именем инструмента, те же значения, что и PreToolUse.

<h4 id="permissionrequest-input">
  PermissionRequest input
</h4>

PermissionRequest hooks получают поля `tool_name` и `tool_input` как PreToolUse hooks, но без `tool_use_id`. Опциональный массив `permission_suggestions` содержит параметры "всегда разрешить", которые пользователь обычно видит в диалоге разрешения. Разница в том, когда срабатывает hook: PermissionRequest hooks запускаются при показе диалога разрешения пользователю, в то время как PreToolUse hooks запускаются перед выполнением инструмента независимо от статуса разрешения.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  PermissionRequest decision control
</h4>

Hooks `PermissionRequest` могут разрешить или отклонить запросы разрешения. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, ваш скрипт hook может вернуть объект `decision` с этими полями, специфичными для события:

| Поле                 | Описание                                                                                                                                                                                                                                   |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` предоставляет разрешение, `"deny"` отклоняет его. [Правила отклонения и запроса](/docs/ru/permissions#manage-permissions) всё ещё применяются, поэтому hook, возвращающий `"allow"`, не переопределяет совпадающее правило отклонения |
| `updatedInput`       | Только для `"allow"`: изменяет параметры входа инструмента перед выполнением. Заменяет весь объект входа, поэтому включите неизменённые поля наряду с изменёнными. Изменённый вход повторно оценивается против правил отклонения и запроса |
| `updatedPermissions` | Только для `"allow"`: массив [записей обновления разрешения](#permission-update-entries) для применения, таких как добавление правила разрешения или изменение режима разрешения сеанса                                                    |
| `message`            | Только для `"deny"`: говорит Claude, почему разрешение было отклонено                                                                                                                                                                      |
| `interrupt`          | Только для `"deny"`: если `true`, останавливает Claude                                                                                                                                                                                     |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Permission update entries
</h4>

Поле выхода `updatedPermissions` и поле входа [`permission_suggestions`](#permissionrequest-input) оба используют один и тот же массив объектов записей. Каждая запись имеет `type`, который определяет её другие поля, и `destination`, который управляет тем, где применяется изменение.

| `type`              | Поля                               | Эффект                                                                                                                                                                                                                    |
| :------------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `addRules`          | `rules`, `behavior`, `destination` | Добавляет правила разрешения. `rules` — это массив объектов `{toolName, ruleContent?}`. Опустите `ruleContent` для совпадения со всем инструментом. `behavior` — это `"allow"`, `"deny"` или `"ask"`                      |
| `replaceRules`      | `rules`, `behavior`, `destination` | Заменяет все правила данного `behavior` в `destination` предоставленными `rules`                                                                                                                                          |
| `removeRules`       | `rules`, `behavior`, `destination` | Удаляет совпадающие правила данного `behavior`                                                                                                                                                                            |
| `setMode`           | `mode`, `destination`              | Изменяет режим разрешения. Допустимые режимы — `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` и `manual` как псевдоним для `default`. Псевдоним `manual` требует Claude Code v2.1.200 или позже |
| `addDirectories`    | `directories`, `destination`       | Добавляет рабочие каталоги. `directories` — это массив строк пути                                                                                                                                                         |
| `removeDirectories` | `directories`, `destination`       | Удаляет рабочие каталоги                                                                                                                                                                                                  |

<Note>
  `setMode` с `bypassPermissions` только вступает в силу, если сеанс был запущен с режимом обхода, уже доступным: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` или `permissions.defaultMode: "bypassPermissions"` в настройках, и режим не отключен [`permissions.disableBypassPermissionsMode`](/docs/ru/permissions#managed-settings). В противном случае обновление — это no-op. `bypassPermissions` никогда не сохраняется как `defaultMode` независимо от `destination`.
</Note>

Поле `destination` на каждой записи определяет, остаётся ли изменение в памяти или сохраняется в файл настроек.

| `destination`     | Записывает в                                         |
| :---------------- | :--------------------------------------------------- |
| `session`         | только в памяти, отбрасывается при завершении сеанса |
| `localSettings`   | `.claude/settings.local.json`                        |
| `projectSettings` | `.claude/settings.json`                              |
| `userSettings`    | `~/.claude/settings.json`                            |

Hook может вывести одно из `permission_suggestions`, которые он получил, как свой собственный выход `updatedPermissions`, что эквивалентно выбору пользователем этого параметра "всегда разрешить" в диалоге.

<h3 id="posttooluse">
  PostToolUse
</h3>

Запускается сразу после успешного завершения инструмента.

Совпадает с именем инструмента, те же значения, что и PreToolUse.

<h4 id="posttooluse-input">
  PostToolUse input
</h4>

Hooks `PostToolUse` срабатывают после того, как инструмент уже выполнился успешно. Входные данные включают как `tool_input`, аргументы, отправленные инструменту, так и `tool_response`, результат, который он вернул. Точная схема для обоих зависит от инструмента.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "success": true
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Поле          | Описание                                                                                                                          |
| :------------ | :-------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Опциональный. Время выполнения инструмента в миллисекундах. Исключает время, потраченное на диалоги разрешения и hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  PostToolUse decision control
</h4>

Hooks `PostToolUse` могут предоставить обратную связь Claude после выполнения инструмента. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                   | Описание                                                                                                                                               |
| :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` добавляет `reason` рядом с результатом инструмента. Claude всё ещё видит исходный выход; чтобы заменить его, используйте `updatedToolOutput` |
| `reason`               | Объяснение, показываемое Claude при `decision` равном `"block"`                                                                                        |
| `additionalContext`    | Строка, добавленная в контекст Claude наряду с результатом инструмента. См. [Add context for Claude](#add-context-for-claude)                          |
| `updatedToolOutput`    | Заменяет выход инструмента предоставленным значением перед отправкой Claude. Значение должно соответствовать форме выхода инструмента                  |
| `updatedMCPToolOutput` | Заменяет выход только для [MCP инструментов](#match-mcp-tools). Предпочитайте `updatedToolOutput`, который работает для всех инструментов              |

Пример ниже заменяет выход вызова `Bash`. Значение замены соответствует форме выхода инструмента `Bash`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` только изменяет то, что видит Claude. Инструмент уже запустился к моменту срабатывания hook, поэтому любые написанные файлы, выполненные команды или отправленные сетевые запросы уже вступили в силу. Телеметрия, такая как spans инструментов OpenTelemetry и события аналитики, также захватывает исходный выход перед запуском hook. Чтобы предотвратить или изменить вызов инструмента перед его запуском, используйте hook [PreToolUse](#pretooluse) вместо этого.

  Значение замены должно соответствовать форме выхода инструмента. Встроенные инструменты возвращают структурированные объекты, а не простые строки. Например, `Bash` возвращает объект с полями `stdout`, `stderr`, `interrupted` и `isImage`. Для встроенных инструментов значение, которое не соответствует схеме выхода инструмента, игнорируется и используется исходный выход. Выход инструмента MCP передаётся без проверки схемы. Удаление деталей ошибок, которые нужны Claude, может привести к тому, что он продолжит с неправильным предположением.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Запускается при сбое выполнения инструмента: инструмент выбросил ошибку или инструмент MCP вернул результат ошибки. Используйте это для логирования сбоев, отправки оповещений или предоставления исправляющей обратной связи Claude.

Совпадает с именем инструмента, те же значения, что и PreToolUse.

<Note>
  Это событие не срабатывает для вызовов инструментов, отклонённых перед выполнением: неизвестное имя инструмента, входные данные, которые не проходят проверку схемы или инструмента, или отклонение разрешения. Отклонения проверки возвращаются как результаты `tool_use_error` и происходят перед запуском hooks, поэтому они не срабатывают ни `PreToolUse` ни этим событием. Отклонения разрешения срабатывают `PreToolUse`, но не этим событием; см. [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  PostToolUseFailure input
</h4>

PostToolUseFailure hooks получают те же поля `tool_name` и `tool_input`, что и PostToolUse, вместе с информацией об ошибке как верхнеуровневые поля:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Command exited with non-zero status code 1",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Поле           | Описание                                                                                                                          |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Строка, описывающая, что пошло не так                                                                                             |
| `is_interrupt` | Опциональное логическое значение, указывающее, был ли сбой вызван прерыванием пользователя                                        |
| `duration_ms`  | Опциональный. Время выполнения инструмента в миллисекундах. Исключает время, потраченное на диалоги разрешения и hooks PreToolUse |

<h4 id="posttoolusefailure-decision-control">
  PostToolUseFailure decision control
</h4>

Hooks `PostToolUseFailure` могут предоставить контекст Claude после сбоя инструмента. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                | Описание                                                                                                      |
| :------------------ | :------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | Строка, добавленная в контекст Claude наряду с ошибкой. См. [Add context for Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

Запускается один раз после разрешения каждого вызова инструмента в пакете, перед отправкой Claude Code следующего запроса модели. `PostToolUse` срабатывает один раз за инструмент, что означает, что он срабатывает одновременно, когда Claude делает параллельные вызовы инструментов. `PostToolBatch` срабатывает ровно один раз со всем пакетом, поэтому это правильное место для внедрения контекста, который зависит от набора инструментов, которые запустились, а не от какого-либо одного инструмента. Нет фильтра для этого события.

<h4 id="posttoolbatch-input">
  PostToolBatch input
</h4>

В дополнение к [общим полям входа](#common-input-fields), PostToolBatch hooks получают `tool_calls`, массив, описывающий каждый вызов инструмента в пакете:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` содержит то же содержимое, которое модель получает в соответствующем блоке `tool_result`. Значение — это сериализованная строка или массив блоков содержимого, ровно как инструмент его выдал. Для `Read` это означает текст с префиксом номера строки, а не необработанное содержимое файла. Ответы могут быть большими, поэтому анализируйте только нужные вам поля.

<Note>
  Форма `tool_response` отличается от `PostToolUse`. `PostToolUse` передаёт структурированный объект `Output` инструмента, такой как `{filePath: "...", success: true}` для `Write`; `PostToolBatch` передаёт сериализованное содержимое `tool_result`, которое видит модель.
</Note>

<h4 id="posttoolbatch-decision-control">
  PostToolBatch decision control
</h4>

Hooks `PostToolBatch` могут внедрить контекст для Claude. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                | Описание                                                                                                                                                                                                                        |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | Строка контекста, внедрённая один раз перед следующим вызовом модели. См. [Add context for Claude](#add-context-for-claude) для деталей доставки, что в неё поместить и как возобновлённые сеансы обрабатывают прошлые значения |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Возврат `decision: "block"` или `continue: false` останавливает агентный цикл перед следующим вызовом модели.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Запускается когда классификатор [auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) отклоняет вызов инструмента. Этот hook срабатывает только в auto mode: он не запускается когда вы вручную отклоняете диалог разрешения, когда hook `PreToolUse` блокирует вызов или когда совпадает правило `deny`. Используйте это для логирования отказов классификатора, корректировки конфигурации или сообщения модели, что она может повторить попытку вызова инструмента.

Совпадает с именем инструмента, те же значения, что и PreToolUse.

<h4 id="permissiondenied-input">
  PermissionDenied input
</h4>

В дополнение к [общим полям входа](#common-input-fields), PermissionDenied hooks получают `tool_name`, `tool_input`, `tool_use_id` и `reason`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "Auto mode denied: command targets a path outside the project"
}
```

| Поле     | Описание                                                              |
| :------- | :-------------------------------------------------------------------- |
| `reason` | Объяснение классификатора того, почему вызов инструмента был отклонён |

<h4 id="permissiondenied-decision-control">
  PermissionDenied decision control
</h4>

PermissionDenied hooks могут сообщить модели, что она может повторить попытку отклонённого вызова инструмента. Верните JSON объект с `hookSpecificOutput.retry`, установленным на `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Когда `retry` равно `true`, Claude Code добавляет сообщение в разговор, говорящее модели, что она может повторить попытку вызова инструмента. Отказ сам по себе не отменяется. Если ваш hook не возвращает JSON или возвращает `retry: false`, отказ остаётся и модель получает исходное сообщение об отклонении.

<h3 id="notification">
  Notification
</h3>

Запускается при отправке Claude Code уведомлений. Совпадает с типом уведомления. Опустите фильтр для запуска hooks для всех типов уведомлений.

| Фильтр                 | Когда он срабатывает                                                                                                |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------ |
| `permission_prompt`    | Claude нуждается в одобрении использования инструмента                                                              |
| `idle_prompt`          | Claude завершил работу и ждёт вашей следующей подсказки                                                             |
| `auth_success`         | Аутентификация завершена                                                                                            |
| `elicitation_dialog`   | MCP сервер открывает форму запроса                                                                                  |
| `elicitation_complete` | Форма запроса MCP отправлена или отклонена                                                                          |
| `elicitation_response` | Ответ на запрос MCP отправлен обратно на сервер                                                                     |
| `agent_needs_input`    | Фоновый сеанс начинает ждать вашего ввода. Срабатывает только при открытом [agent view](/docs/ru/agent-view) в терминале |
| `agent_completed`      | Фоновый сеанс завершается или не удаётся. Срабатывает только при открытом [agent view](/docs/ru/agent-view) в терминале  |

Типы `agent_needs_input` и `agent_completed` требуют Claude Code v2.1.198 или позже.

Используйте отдельные фильтры для запуска разных обработчиков в зависимости от типа уведомления. Эта конфигурация запускает скрипт оповещения, специфичный для разрешения, когда Claude нуждается в одобрении разрешения, и другое уведомление, когда Claude был неактивен:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Notification input
</h4>

В дополнение к [общим полям входа](#common-input-fields), Notification hooks получают `message` с текстом уведомления, опциональный `title` и `notification_type`, указывающий, какой тип сработал.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Notification hooks не могут блокировать или изменять уведомления. Они предназначены для побочных эффектов, таких как пересылка уведомления во внешний сервис. [Общие JSON поля выхода](#json-output) такие как `systemMessage` применяются.

<h3 id="subagentstart">
  SubagentStart
</h3>

Запускается при порождении Claude Code subagent через инструмент Agent. Поддерживает фильтры для фильтрации по имени типа агента. Для встроенных агентов это имя агента, такое как `general-purpose`, `Explore` или `Plan`. Для [пользовательских subagents](/docs/ru/sub-agents), это поле `name` из frontmatter агента, а не имя файла.

Для subagents, поставляемых [плагином](/docs/ru/plugins), тип агента — это идентификатор, ограниченный плагином, такой как `my-plugin:reviewer`, а не простое имя frontmatter. Двоеточие помещает имя, ограниченное плагином, на путь регулярного выражения, поэтому закрепите фильтр с `^` и `$` для точного совпадения: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  SubagentStart input
</h4>

В дополнение к [общим полям входа](#common-input-fields), SubagentStart hooks получают `agent_id` с уникальным идентификатором для subagent и `agent_type` с именем агента, которое фильтр использует для фильтрации.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

SubagentStart hooks не могут блокировать создание subagent, но они могут внедрить контекст в subagent. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, вы можете вернуть:

| Поле                | Описание                                                                                                                                           |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Строка, добавленная в контекст subagent в начале его разговора, перед его первой подсказкой. См. [Add context for Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

<h3 id="subagentstop">
  SubagentStop
</h3>

Запускается при завершении ответа Claude Code subagent. Совпадает с типом агента, те же значения, что и SubagentStart.

<h4 id="subagentstop-input">
  SubagentStop input
</h4>

В дополнение к [общим полям входа](#common-input-fields), SubagentStop hooks получают `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` и `last_assistant_message`. Поле `agent_type` — это значение, используемое для фильтрации фильтра. `transcript_path` — это транскрипт основного сеанса, в то время как `agent_transcript_path` — это собственный транскрипт subagent, хранящийся в вложенной папке `subagents/`. Поле `last_assistant_message` содержит текстовое содержимое финального ответа subagent, поэтому hooks могут получить к нему доступ без анализа файла транскрипта.

SubagentStop hooks также получают массивы `background_tasks` и `session_crons`, описанные в разделе [Stop input](#stop-input), доступные в Claude Code v2.1.145 или позже. Оба массива ограничены родительским сеансом, а не subagent.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

SubagentStop hooks используют тот же формат управления решением, что и [Stop hooks](#stop-decision-control), включая `hookSpecificOutput.additionalContext` с `hookEventName`, установленным на `"SubagentStop"`, для ненаправленной обратной связи, которая держит subagent работающим. Возврат `decision: "block"` с `reason` держит subagent работающим и доставляет `reason` subagent как его следующую инструкцию. Чтобы внедрить контекст в родительский сеанс после возврата subagent, используйте hook [`PostToolUse`](#posttooluse) на инструменте `Agent` вместо этого.

<h3 id="taskcreated">
  TaskCreated
</h3>

Запускается при создании задачи через инструмент `TaskCreate`. Используйте это для обеспечения соглашений об именовании, требования описаний задач или предотвращения создания определённых задач.

Когда hook `TaskCreated` выходит с кодом 2, задача не создаётся и сообщение stderr передаётся обратно модели как обратная связь. Чтобы полностью остановить товарища вместо его повторного запуска, верните JSON с `{"continue": false, "stopReason": "..."}`. TaskCreated hooks не поддерживают фильтры и срабатывают при каждом вхождении.

<h4 id="taskcreated-input">
  TaskCreated input
</h4>

В дополнение к [общим полям входа](#common-input-fields), TaskCreated hooks получают `task_id`, `task_subject` и опционально `task_description`, `teammate_name` и `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Поле               | Описание                                                           |
| :----------------- | :----------------------------------------------------------------- |
| `task_id`          | Идентификатор создаваемой задачи                                   |
| `task_subject`     | Название задачи                                                    |
| `task_description` | Подробное описание задачи. Может отсутствовать                     |
| `teammate_name`    | Имя товарища, создающего задачу. Может отсутствовать               |
| `team_name`        | Имя команды, полученное из сеанса; будет удалено в будущем выпуске |

<h4 id="taskcreated-decision-control">
  TaskCreated decision control
</h4>

TaskCreated hooks поддерживают два способа управления созданием задачи:

* **Exit code 2**: задача не создаётся и сообщение stderr передаётся обратно модели как обратная связь.
* **JSON `{"continue": false, "stopReason": "..."}`**: полностью останавливает товарища, соответствуя поведению hook `Stop`. `stopReason` показывается пользователю.

Этот пример блокирует задачи, чьи названия не следуют требуемому формату:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

Запускается при отметке задачи как завершённой. Это срабатывает в двух ситуациях: когда любой агент явно отмечает задачу как завершённую через инструмент TaskUpdate, или когда товарищ [agent team](/docs/ru/agent-teams) завершает свой ход с незавершёнными задачами. Используйте это для обеспечения критериев завершения, таких как прохождение тестов или проверок линтинга перед закрытием задачи.

Когда hook `TaskCompleted` выходит с кодом 2, задача не отмечается как завершённая и сообщение stderr передаётся обратно модели как обратная связь. Чтобы полностью остановить товарища вместо его повторного запуска, верните JSON с `{"continue": false, "stopReason": "..."}`. TaskCompleted hooks не поддерживают фильтры и срабатывают при каждом вхождении.

<h4 id="taskcompleted-input">
  TaskCompleted input
</h4>

В дополнение к [общим полям входа](#common-input-fields), TaskCompleted hooks получают `task_id`, `task_subject` и опционально `task_description`, `teammate_name` и `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Поле               | Описание                                                           |
| :----------------- | :----------------------------------------------------------------- |
| `task_id`          | Идентификатор завершаемой задачи                                   |
| `task_subject`     | Название задачи                                                    |
| `task_description` | Подробное описание задачи. Может отсутствовать                     |
| `teammate_name`    | Имя товарища, завершающего задачу. Может отсутствовать             |
| `team_name`        | Имя команды, полученное из сеанса; будет удалено в будущем выпуске |

<h4 id="taskcompleted-decision-control">
  TaskCompleted decision control
</h4>

TaskCompleted hooks поддерживают два способа управления завершением задачи:

* **Exit code 2**: задача не отмечается как завершённая и сообщение stderr передаётся обратно модели как обратная связь.
* **JSON `{"continue": false, "stopReason": "..."}`**: полностью останавливает товарища, соответствуя поведению hook `Stop`. `stopReason` показывается пользователю.

Этот пример запускает тесты и блокирует завершение задачи, если они не пройдены:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Run the test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Запускается при завершении ответа основного агента Claude Code. Не запускается, если остановка произошла из-за прерывания пользователя. Ошибки API срабатывают [StopFailure](#stopfailure) вместо этого.

<Tip>
  Команда [`/goal`](/docs/ru/goal) — это встроенный ярлык для stop hook, ограниченного сеансом на основе подсказки. Используйте её, когда вы хотите, чтобы Claude продолжал работать до выполнения условия без написания конфигурации hook.
</Tip>

<h4 id="stop-input">
  Stop input
</h4>

В дополнение к [общим полям входа](#common-input-fields), Stop hooks получают `stop_hook_active`, `last_assistant_message`, `background_tasks` и `session_crons`. Поле `stop_hook_active` равно `true`, когда Claude Code уже продолжает в результате stop hook. Проверьте это значение или обработайте транскрипт, чтобы предотвратить блокировку на условии, которое никогда не разрешится. Claude Code переопределяет hook и заканчивает ход после 8 последовательных блокировок.

Поле `last_assistant_message` содержит текстовое содержимое финального ответа Claude, поэтому hooks могут получить к нему доступ без анализа файла транскрипта. Для hooks, которые действуют на только что завершённый ход, такие как hooks для чтения вслух или уведомления, используйте это поле, а не читайте `transcript_path`: файл транскрипта не гарантирует включение финального сообщения в момент Stop на всех версиях.

Массивы `background_tasks` и `session_crons`, доступные в Claude Code v2.1.145 или позже, позволяют hooks различать "сеанс завершён" от "сеанс приостановлен в ожидании фоновой работы для его пробуждения". Оба массива присутствуют, когда реестр задач доступен, и пусты, когда ничего не выполняется или не запланировано.

Каждая запись в `background_tasks` описывает одну выполняемую задачу и использует эти поля:

| Поле          | Описание                                                                                                                                                                                                                                                                 |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Идентификатор задачи                                                                                                                                                                                                                                                     |
| `type`        | Дружественный ярлык типа задачи, такой как `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` или `MCP task`. Каждый ярлык определяет, какая функция Claude Code создала задачу. Возвращается к необработанному дискриминанту для неизвестных типов |
| `status`      | Текущий статус задачи                                                                                                                                                                                                                                                    |
| `description` | Описание в свободной форме, ограниченное 1000 символами с маркером `… [+N chars]` в строке при обрезке                                                                                                                                                                   |
| `command`     | Командная строка оболочки, ограниченная 1000 символами. Присутствует только для задач `shell`                                                                                                                                                                            |
| `agent_type`  | Имя типа subagent. Присутствует только для задач `subagent`                                                                                                                                                                                                              |
| `server`      | Имя MCP сервера. Присутствует только для задач `monitor` и `MCP task`                                                                                                                                                                                                    |
| `tool`        | Имя MCP инструмента. Присутствует только для задач `monitor` и `MCP task`                                                                                                                                                                                                |
| `name`        | Имя workflow. Присутствует только для задач `workflow`                                                                                                                                                                                                                   |

Каждая запись в `session_crons` описывает одно запланированное пробуждение, ограниченное сеансом, полученное из `CronCreate`, `ScheduleWakeup` и `/loop`:

| Поле        | Описание                                                                                                                                                   |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Идентификатор cron задачи                                                                                                                                  |
| `schedule`  | Выражение cron, например `0 9 * * 1-5`                                                                                                                     |
| `recurring` | `false` для одноразовых пробуждений, чьё расписание кодирует одно время срабатывания, `true` для задач, которые повторно срабатывают при каждом совпадении |
| `prompt`    | Подсказка, отправленная при срабатывании cron, ограниченная 1000 символами с тем же маркером `… [+N chars]`                                                |

Этот пример показывает Stop input с одной выполняемой shell задачей и одним повторяющимся cron:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Stop decision control
</h4>

Hooks `Stop` и `SubagentStop` могут управлять тем, продолжает ли Claude. В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                                   | Описание                                                                                                                                                                                                                    |
| :------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` предотвращает остановку Claude. Опустите, чтобы разрешить Claude остановиться                                                                                                                                     |
| `reason`                               | Требуется при `decision` равном `"block"`. Говорит Claude, почему оно должно продолжить                                                                                                                                     |
| `hookSpecificOutput.additionalContext` | Ненаправленная обратная связь для Claude. Разговор продолжается, чтобы Claude мог действовать на основе этого, но в отличие от `decision: "block"` это показывается в транскрипте как обратная связь hook, а не ошибка hook |

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Используйте `additionalContext`, когда hook работает как задумано и даёт Claude руководство, такое как "запустите набор тестов перед завершением". Это держит разговор идущим через те же защиты цикла, что и `decision: "block"`, а именно вход `stop_hook_active` и ограничение 8 последовательных продолжений, но транскрипт помечает его как `Stop hook feedback` и уведомление об ошибке hook не показывается:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

Запускается вместо [Stop](#stop) когда ход заканчивается из-за ошибки API. Выход и код выхода игнорируются. Используйте это для логирования сбоев, отправки оповещений или принятия действий восстановления, когда Claude не может завершить ответ из-за ограничений скорости, проблем аутентификации или других ошибок API.

<h4 id="stopfailure-input">
  StopFailure input
</h4>

В дополнение к [общим полям входа](#common-input-fields), StopFailure hooks получают `error`, опциональные `error_details` и `last_assistant_message`. Поле `error` определяет тип ошибки и используется для фильтрации фильтра.

| Поле                     | Описание                                                                                                                                                                                                                                |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Тип ошибки: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` или `unknown`                                      |
| `error_details`          | Дополнительные детали об ошибке, когда доступны                                                                                                                                                                                         |
| `last_assistant_message` | Отрендеренный текст ошибки, показанный в разговоре. В отличие от `Stop` и `SubagentStop`, где это поле содержит разговорный выход Claude, для `StopFailure` оно содержит строку ошибки API, такую как `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

StopFailure hooks не имеют управления решением. Они запускаются только в целях уведомления и логирования.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Запускается когда товарищ [agent team](/docs/ru/agent-teams) собирается перейти в режим ожидания после завершения своего хода. Используйте это для обеспечения качественных ворот перед остановкой работы товарища, такие как требование прохождения проверок линтинга или проверка существования выходных файлов.

Когда hook `TeammateIdle` выходит с кодом 2, товарищ получает сообщение stderr как обратную связь и продолжает работать вместо перехода в режим ожидания. Чтобы полностью остановить товарища вместо его повторного запуска, верните JSON с `{"continue": false, "stopReason": "..."}`. TeammateIdle hooks не поддерживают фильтры и срабатывают при каждом вхождении.

<h4 id="teammateidle-input">
  TeammateIdle input
</h4>

В дополнение к [общим полям входа](#common-input-fields), TeammateIdle hooks получают `teammate_name` и `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Поле            | Описание                                                           |
| :-------------- | :----------------------------------------------------------------- |
| `teammate_name` | Имя товарища, который собирается перейти в режим ожидания          |
| `team_name`     | Имя команды, полученное из сеанса; будет удалено в будущем выпуске |

<h4 id="teammateidle-decision-control">
  TeammateIdle decision control
</h4>

TeammateIdle hooks поддерживают два способа управления поведением товарища:

* **Exit code 2**: товарищ получает сообщение stderr как обратную связь и продолжает работать вместо перехода в режим ожидания.
* **JSON `{"continue": false, "stopReason": "..."}`**: полностью останавливает товарища, соответствуя поведению hook `Stop`. `stopReason` показывается пользователю.

Этот пример проверяет, что артефакт сборки существует перед разрешением товарищу перейти в режим ожидания:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

Запускается при изменении файла конфигурации во время сеанса. Используйте это для аудита изменений настроек, обеспечения политик безопасности или блокировки несанкционированных изменений файлов конфигурации.

ConfigChange hooks срабатывают для изменений файлов настроек, управляемых параметров политики и файлов skills. Поле `source` во входных данных говорит вам, какой тип конфигурации изменился, и опциональное поле `file_path` предоставляет путь к изменённому файлу.

Фильтр фильтрует по источнику конфигурации:

| Фильтр             | Когда он срабатывает                      |
| :----------------- | :---------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` изменяется      |
| `project_settings` | `.claude/settings.json` изменяется        |
| `local_settings`   | `.claude/settings.local.json` изменяется  |
| `policy_settings`  | Управляемые параметры политики изменяются |
| `skills`           | Файл skill в `.claude/skills/` изменяется |

Этот пример логирует все изменения конфигурации для аудита безопасности:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  ConfigChange input
</h4>

В дополнение к [общим полям входа](#common-input-fields), ConfigChange hooks получают `source` и опционально `file_path`. Поле `source` указывает, какой тип конфигурации изменился, и `file_path` предоставляет путь к конкретному изменённому файлу.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  ConfigChange decision control
</h4>

ConfigChange hooks могут блокировать применение изменений конфигурации. Используйте exit code 2 или JSON `decision` для предотвращения изменения. При блокировке новые параметры не применяются к запущенному сеансу.

| Поле       | Описание                                                                                       |
| :--------- | :--------------------------------------------------------------------------------------------- |
| `decision` | `"block"` предотвращает применение изменения конфигурации. Опустите, чтобы разрешить изменение |
| `reason`   | Объяснение, показываемое пользователю при `decision` равном `"block"`                          |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Изменения `policy_settings` не могут быть заблокированы. Hooks всё ещё срабатывают для источников `policy_settings`, поэтому вы можете использовать их для аудита логирования, но любое решение блокировки игнорируется. Это гарантирует, что управляемые предприятием параметры всегда вступают в силу.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Запускается при изменении рабочего каталога во время сеанса, например когда Claude выполняет команду `cd`. Используйте это для реакции на изменения каталога: перезагрузка переменных окружения, активация специфичных для проекта цепочек инструментов или автоматический запуск скриптов настройки. Объединяется с [FileChanged](#filechanged) для инструментов, таких как [direnv](https://direnv.net/), которые управляют окружением для каждого каталога.

CwdChanged hooks имеют доступ к `CLAUDE_ENV_FILE`. Переменные, написанные в этот файл, сохраняются в последующих командах Bash для сеанса, как и в [SessionStart hooks](#persist-environment-variables).

CwdChanged не поддерживает фильтры и срабатывает при каждом изменении каталога.

<h4 id="cwdchanged-input">
  CwdChanged input
</h4>

В дополнение к [общим полям входа](#common-input-fields), CwdChanged hooks получают `old_cwd` и `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  CwdChanged output
</h4>

В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, CwdChanged hooks могут вернуть `watchPaths` для динамической установки, какие пути файлов [FileChanged](#filechanged) отслеживает:

| Поле         | Описание                                                                                                                                                                                                                  |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `watchPaths` | Массив абсолютных путей. Заменяет текущий динамический список наблюдения. Пути из конфигурации `matcher` всегда отслеживаются. Возврат пустого массива очищает динамический список, что типично при входе в новый каталог |

CwdChanged hooks не имеют управления решением. Они не могут блокировать изменение каталога.

<h3 id="filechanged">
  FileChanged
</h3>

Запускается при изменении отслеживаемого файла на диске. Полезно для перезагрузки переменных окружения при изменении файлов конфигурации проекта.

Поле `matcher` для этого события служит двум целям:

* **Построение списка наблюдения**: значение разделяется на `|` и каждый сегмент регистрируется как буквальное имя файла в рабочем каталоге, поэтому `".envrc|.env"` отслеживает ровно эти два файла. Regex шаблоны здесь не полезны: значение, такое как `^\.env`, отслеживало бы файл буквально названный `^\.env`.
* **Фильтрация, какие hooks запускаются**: когда отслеживаемый файл изменяется, то же значение фильтрует, какие группы hook запускаются, используя стандартные [правила фильтра](#matcher-patterns) против базового имени изменённого файла.

FileChanged hooks имеют доступ к `CLAUDE_ENV_FILE`. Переменные, написанные в этот файл, сохраняются в последующих командах Bash для сеанса, как и в [SessionStart hooks](#persist-environment-variables).

<h4 id="filechanged-input">
  FileChanged input
</h4>

В дополнение к [общим полям входа](#common-input-fields), FileChanged hooks получают `file_path` и `event`.

| Поле        | Описание                                                                                                          |
| :---------- | :---------------------------------------------------------------------------------------------------------------- |
| `file_path` | Абсолютный путь к файлу, который изменился                                                                        |
| `event`     | Что произошло: `"change"` для изменённого файла, `"add"` для созданного файла или `"unlink"` для удалённого файла |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  FileChanged output
</h4>

В дополнение к [JSON полям выхода](#json-output), доступным для всех hooks, FileChanged hooks могут вернуть `watchPaths` для динамического обновления, какие пути файлов отслеживаются:

| Поле         | Описание                                                                                                                                                                                                                                             |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Массив абсолютных путей. Заменяет текущий динамический список наблюдения. Пути из конфигурации `matcher` всегда отслеживаются. Используйте это, когда ваш скрипт hook обнаруживает дополнительные файлы для отслеживания на основе изменённого файла |

FileChanged hooks не имеют управления решением. Они не могут блокировать изменение файла от возникновения.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Запускается при создании worktree, либо из `claude --worktree`, либо из [subagent, использующего `isolation: "worktree"`](/docs/ru/sub-agents#choose-the-subagent-scope). По умолчанию Claude Code создаёт изолированную рабочую копию с помощью `git worktree`. Настройка hook WorktreeCreate заменяет это поведение git по умолчанию, позволяя вам использовать другую систему контроля версий, такую как SVN, Perforce или Mercurial.

Потому что hook заменяет поведение по умолчанию полностью, [`.worktreeinclude`](/docs/ru/worktrees#copy-gitignored-files-into-worktrees) не обрабатывается. Если вам нужно скопировать локальные файлы конфигурации, такие как `.env`, в новый worktree, сделайте это внутри вашего скрипта hook.

Hook должен вернуть путь к созданному worktree каталогу. Claude Code использует этот путь как рабочий каталог для изолированного сеанса. См. [WorktreeCreate output](#worktreecreate-output) для того, как каждый тип hook возвращает путь.

Этот пример создаёт рабочую копию SVN и выводит путь для использования Claude Code. Замените URL репозитория на свой:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Hook читает имя worktree `name` из JSON входа на stdin, проверяет свежую копию в новый каталог и выводит путь каталога. `echo` на последней строке — это то, что Claude Code читает как путь worktree. Перенаправьте любой другой выход на stderr, чтобы он не мешал пути.

<h4 id="worktreecreate-input">
  WorktreeCreate input
</h4>

В дополнение к [общим полям входа](#common-input-fields), WorktreeCreate hooks получают поле `name`. Это идентификатор slug для нового worktree, либо указанный пользователем, либо автоматически сгенерированный, например `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  WorktreeCreate output
</h4>

WorktreeCreate hooks не используют стандартную модель решения разрешить/заблокировать. Вместо этого успех или сбой hook определяет результат. Hook должен вернуть путь к созданному worktree каталогу:

* **Command hooks** (`type: "command"`): выводят путь как последнюю непустую строку stdout. Claude Code удаляет коды ANSI перед чтением этой строки, поэтому баннеры запуска оболочки, выведенные перед вашим `echo`, игнорируются. Перенаправьте любой другой выход hook на stderr.
* **HTTP hooks** (`type: "http"`): возвращают `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` в теле ответа.

Если hook не удаётся или не производит путь, создание worktree не удаётся с ошибкой.

Claude Code разрешает относительный путь против каталога, в котором запустился hook. Если результирующий путь не является каталогом, который Claude Code может ввести, сеанс выводит ошибку, называющую путь, и выходит с кодом 1. До v2.1.205 относительный путь или путь, который не существовал на диске, вызывал сбой сеанса при запуске, и с `-p` он зависал примерно на 30 секунд перед выходом с кодом 0.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Запускается при удалении worktree, либо при выходе из сеанса `--worktree` и выборе его удаления, либо при завершении subagent с `isolation: "worktree"`. Это аналог очистки для [WorktreeCreate](#worktreecreate).

Для git-based worktrees Claude Code обрабатывает очистку автоматически с помощью `git worktree remove`. Если вы настроили hook WorktreeCreate для системы контроля версий, не основанной на git, объедините его с hook WorktreeRemove для обработки очистки. Без него каталог worktree остаётся на диске.

Claude Code передаёт путь, который WorktreeCreate вывел на stdout, как `worktree_path` во входных данных hook. Этот пример читает этот путь и удаляет каталог:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  WorktreeRemove input
</h4>

В дополнение к [общим полям входа](#common-input-fields), WorktreeRemove hooks получают поле `worktree_path`, которое является абсолютным путём к удаляемому worktree.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

WorktreeRemove hooks не имеют управления решением. Они не могут блокировать удаление worktree, но мог выполнять задачи очистки, такие как удаление состояния контроля версий или архивирование изменений. Сбои hook логируются только в режиме отладки.

<h3 id="precompact">
  PreCompact
</h3>

Запускается перед тем, как Claude Code собирается запустить операцию компактирования.

Значение фильтра указывает, было ли компактирование запущено вручную или автоматически:

| Фильтр   | Когда он срабатывает                                            |
| :------- | :-------------------------------------------------------------- |
| `manual` | `/compact`                                                      |
| `auto`   | Автоматическое компактирование при заполнении контекстного окна |

Exit code 2 блокирует компактирование. Для ручного `/compact` сообщение stderr показывается пользователю. Вы также можете заблокировать, возвращая JSON с `"decision": "block"`.

Блокировка автоматического компактирования имеет разные эффекты в зависимости от того, когда оно срабатывает. Если компактирование было запущено проактивно перед лимитом контекста, Claude Code пропускает его и разговор продолжается некомпактированным. Если компактирование было запущено для восстановления от ошибки лимита контекста, уже возвращённой API, основная ошибка выходит на поверхность и текущий запрос не удаётся.

<h4 id="precompact-input">
  PreCompact input
</h4>

В дополнение к [общим полям входа](#common-input-fields), PreCompact hooks получают `trigger` и `custom_instructions`. Для `manual`, `custom_instructions` содержит то, что пользователь передаёт в `/compact`. Для `auto`, `custom_instructions` пусто.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Запускается после завершения Claude Code операции компактирования. Используйте это событие для реакции на новое компактирован состояние, например для логирования сгенерированного резюме или обновления внешнего состояния.

Те же значения фильтра применяются как для `PreCompact`:

| Фильтр   | Когда он срабатывает                                                   |
| :------- | :--------------------------------------------------------------------- |
| `manual` | После `/compact`                                                       |
| `auto`   | После автоматического компактирования при заполнении контекстного окна |

<h4 id="postcompact-input">
  PostCompact input
</h4>

В дополнение к [общим полям входа](#common-input-fields), PostCompact hooks получают `trigger` и `compact_summary`. Поле `compact_summary` содержит резюме разговора, сгенерированное операцией компактирования.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

PostCompact hooks не имеют управления решением. Они не могут влиять на результат компактирования, но могут выполнять последующие задачи.

<h3 id="sessionend">
  SessionEnd
</h3>

Запускается при завершении сеанса Claude Code. Полезно для задач очистки, логирования статистики сеанса или сохранения состояния сеанса. Поддерживает фильтры для фильтрации по причине выхода.

Поле `reason` во входных данных hook указывает, почему сеанс закончился:

| Причина                       | Описание                                          |
| :---------------------------- | :------------------------------------------------ |
| `clear`                       | Сеанс очищен с помощью команды `/clear`           |
| `resume`                      | Сеанс переключен через интерактивный `/resume`    |
| `logout`                      | Пользователь вышел                                |
| `prompt_input_exit`           | Пользователь вышел, пока был виден ввод подсказки |
| `bypass_permissions_disabled` | Режим обхода разрешений был отключен              |
| `other`                       | Другие причины выхода                             |

<h4 id="sessionend-input">
  SessionEnd input
</h4>

В дополнение к [общим полям входа](#common-input-fields), SessionEnd hooks получают поле `reason`, указывающее, почему сеанс закончился. См. таблицу выше для всех значений.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

SessionEnd hooks не имеют управления решением. Они не могут блокировать завершение сеанса, но могут выполнять задачи очистки.

SessionEnd hooks имеют таймаут по умолчанию 1,5 секунды. Это применяется как к выходу из сеанса, так и к `/clear` и переключению сеансов через интерактивный `/resume`. Если hook нуждается в большем времени, установите поле `timeout` в конфигурации hook. Общий бюджет автоматически повышается до наибольшего per-hook таймаута, настроенного в файлах настроек, до 60 секунд. Таймауты, установленные на hooks, предоставленные плагинами, не повышают бюджет. Чтобы явно переопределить бюджет, установите переменную окружения `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` в миллисекундах.

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

<h3 id="elicitation">
  Elicitation
</h3>

Запускается, когда MCP сервер запрашивает ввод пользователя во время выполнения задачи. По умолчанию Claude Code показывает интерактивный диалог для ответа пользователя. Hooks могут перехватить этот запрос и ответить программно, полностью пропустив диалог.

Поле фильтра совпадает с именем MCP сервера.

<h4 id="elicitation-input">
  Elicitation input
</h4>

В дополнение к [общим полям входа](#common-input-fields), Elicitation hooks получают `mcp_server_name`, `message` и опциональные `mode`, `url`, `elicitation_id` и `requested_schema` поля.

Для form-mode elicitation (наиболее распространённый случай):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Для URL-mode elicitation (аутентификация на основе браузера):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Elicitation output
</h4>

Чтобы ответить программно без показа диалога, верните JSON объект с `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Поле      | Значения                      | Описание                                                                             |
| :-------- | :---------------------------- | :----------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Принять, отклонить или отменить запрос                                               |
| `content` | object                        | Значения полей формы для отправки. Используется только когда `action` равен `accept` |

Exit code 2 отклоняет elicitation и показывает stderr пользователю.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Запускается после ответа пользователя на MCP elicitation. Hooks могут наблюдать, изменять или блокировать ответ перед его отправкой обратно на MCP сервер.

Поле фильтра совпадает с именем MCP сервера.

<h4 id="elicitationresult-input">
  ElicitationResult input
</h4>

В дополнение к [общим полям входа](#common-input-fields), ElicitationResult hooks получают `mcp_server_name`, `action` и опциональные `mode`, `elicitation_id` и `content` поля.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  ElicitationResult output
</h4>

Чтобы переопределить ответ пользователя, верните JSON объект с `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Поле      | Значения                      | Описание                                                                              |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------------ |
| `action`  | `accept`, `decline`, `cancel` | Переопределяет действие пользователя                                                  |
| `content` | object                        | Переопределяет значения полей формы. Имеет смысл только когда `action` равен `accept` |

Exit code 2 блокирует ответ, изменяя эффективное действие на `decline`.

<h2 id="prompt-based-hooks">
  Prompt-based hooks
</h2>

В дополнение к command, HTTP и MCP tool hooks, Claude Code поддерживает prompt-based hooks (`type: "prompt"`), которые используют LLM для оценки разрешения или блокировки действия, и agent hooks (`type: "agent"`), которые порождают агентного верификатора с доступом к инструментам. Не все события поддерживают каждый тип hook.

События, которые поддерживают все пять типов hook (`command`, `http`, `mcp_tool`, `prompt` и `agent`):

* `PermissionDenied`
* `PermissionRequest`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

События, которые поддерживают `command`, `http` и `mcp_tool` hooks, но не `prompt` или `agent`:

* `ConfigChange`
* `CwdChanged`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `Notification`
* `PostCompact`
* `PreCompact`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` и `Setup` поддерживают `command` и `mcp_tool` hooks. Они не поддерживают `http`, `prompt` или `agent` hooks.

<h3 id="how-prompt-based-hooks-work">
  How prompt-based hooks work
</h3>

Вместо выполнения команды Bash, prompt-based hooks:

1. Отправляют входные данные hook и вашу подсказку модели Claude, Haiku по умолчанию
2. LLM отвечает структурированным JSON, содержащим решение
3. Claude Code автоматически обрабатывает решение

<h3 id="prompt-hook-configuration">
  Prompt hook configuration
</h3>

Установите `type` на `"prompt"` и предоставьте строку `prompt` вместо `command`. Используйте заполнитель `$ARGUMENTS` для внедрения данных JSON входа hook в текст вашей подсказки. Claude Code отправляет объединённую подсказку и входные данные быстрой модели Claude, которая возвращает JSON решение.

Этот hook `Stop` просит LLM оценить, должен ли Claude остановиться перед разрешением Claude закончить:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Поле              | Обязательно | Описание                                                                                                                                                                                                                                                                             |
| :---------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | да          | Должно быть `"prompt"`                                                                                                                                                                                                                                                               |
| `prompt`          | да          | Текст подсказки для отправки LLM. Используйте `$ARGUMENTS` как заполнитель для JSON входа hook. Если `$ARGUMENTS` отсутствует, JSON входа добавляется к подсказке                                                                                                                    |
| `model`           | нет         | Модель для использования при оценке. По умолчанию быстрая модель                                                                                                                                                                                                                     |
| `timeout`         | нет         | Таймаут в секундах. По умолчанию: 30                                                                                                                                                                                                                                                 |
| `continueOnBlock` | нет         | Когда подсказка возвращает `ok: false`, передайте причину обратно Claude и продолжите ход вместо остановки. По умолчанию: `false`. Реализуется как `continue: true` на результирующем `decision: "block"`. См. [Response schema](#response-schema) для поведения для каждого события |

<h3 id="response-schema">
  Response schema
</h3>

LLM должен ответить JSON, содержащим:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision"
}
```

| Поле     | Описание                                                                                                  |
| :------- | :-------------------------------------------------------------------------------------------------------- |
| `ok`     | `true` разрешает действие. `false` производит `decision: "block"`. См. поведение для каждого события ниже |
| `reason` | Требуется при `ok` равном `false`. Используется как причина блокировки                                    |

Что происходит при `ok: false`, зависит от события:

* `Stop` и `SubagentStop`: причина передаётся обратно Claude как его следующая инструкция и ход продолжается
* `PreToolUse`: вызов инструмента отклоняется и причина возвращается Claude как ошибка инструмента, эквивалентно `permissionDecision: "deny"` из command hook
* `PostToolUse`: по умолчанию ход заканчивается и причина появляется в чате как строка предупреждения. Установите `continueOnBlock: true` для передачи причины обратно Claude и продолжения хода вместо этого
* `PostToolBatch`, `UserPromptSubmit` и `UserPromptExpansion`: ход заканчивается и причина появляется как строка предупреждения. Эти события заканчивают ход на `decision: "block"` независимо от `continue`
* `PostToolUseFailure`, `TaskCreated` и `TaskCompleted`: причина возвращается Claude как ошибка инструмента, аналогично `PreToolUse`
* `TeammateIdle`: по умолчанию товарищ по команде останавливается и причина появляется как строка предупреждения. Установите `continueOnBlock: true` для передачи причины обратно товарищу по команде и продолжения его работы вместо этого
* `PermissionRequest`: `ok: false` не имеет эффекта. Чтобы отклонить одобрение из hook, используйте [command hook](#command-hook-fields), возвращающий `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` не имеет эффекта, потому что отказ уже произошёл. Единственный результат, который это событие читает, это `hookSpecificOutput.retry`, который prompt и agent hooks не могут установить. Они запускаются на этом событии, но их результат отбрасывается. Используйте [command hook](#command-hook-fields) для возврата `retry`

Если вам нужен более точный контроль над любым событием, используйте [command hook](#command-hook-fields) с полями для каждого события, описанными в [Decision control](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Check multiple conditions before stopping
</h3>

Этот hook `Stop` использует подробную подсказку для проверки трёх условий перед разрешением Claude остановиться. Hooks `SubagentStop` используют тот же формат для оценки, должен ли [subagent](/docs/ru/sub-agents) остановиться. Если `"ok"` равно `false`, Claude продолжает работать с предоставленной причиной как своей следующей инструкцией:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Agent-based hooks
</h2>

<Warning>
  Agent hooks являются экспериментальными. Поведение и конфигурация могут измениться в будущих выпусках. Для рабочих процессов в производстве предпочитайте [command hooks](#command-hook-fields).
</Warning>

Agent-based hooks (`type: "agent"`) похожи на prompt-based hooks, но с многооборотным доступом к инструментам. Вместо одного вызова LLM, agent hook порождает subagent, который может читать файлы, искать код и проверять кодовую базу для проверки условий. Agent hooks поддерживают те же события, что и prompt-based hooks.

<h3 id="how-agent-hooks-work">
  How agent hooks work
</h3>

Когда срабатывает agent hook:

1. Claude Code порождает subagent с вашей подсказкой и JSON входом hook
2. Subagent может использовать инструменты, такие как Read, Grep и Glob, для исследования
3. После до 50 оборотов subagent возвращает структурированное решение `{ "ok": true/false }`
4. Claude Code обрабатывает решение так же, как prompt hook

Agent hooks полезны, когда проверка требует проверки фактических файлов или выхода тестов, а не только оценки данных входа hook.

<h3 id="agent-hook-configuration">
  Agent hook configuration
</h3>

Установите `type` на `"agent"` и предоставьте строку `prompt`. Поля конфигурации те же, что и [prompt hooks](#prompt-hook-configuration), с более длинным таймаутом по умолчанию:

| Поле      | Обязательно | Описание                                                                                            |
| :-------- | :---------- | :-------------------------------------------------------------------------------------------------- |
| `type`    | да          | Должно быть `"agent"`                                                                               |
| `prompt`  | да          | Подсказка, описывающая, что проверять. Используйте `$ARGUMENTS` как заполнитель для JSON входа hook |
| `model`   | нет         | Модель для использования. По умолчанию быстрая модель                                               |
| `timeout` | нет         | Таймаут в секундах. По умолчанию: 60                                                                |

Схема ответа та же, что и prompt hooks: `{ "ok": true }` для разрешения или `{ "ok": false, "reason": "..." }` для блокировки.

Этот hook `Stop` проверяет, что все модульные тесты проходят перед разрешением Claude закончить:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Запуск hooks в фоне
</h2>

По умолчанию hooks блокируют выполнение Claude до их завершения. Для долгоживущих задач, таких как развёртывания, наборы тестов или вызовы внешних API, установите `"async": true` для запуска hook в фоне, пока Claude продолжает работать. Асинхронные hooks не могут блокировать или управлять поведением Claude: поля ответа, такие как `decision`, `permissionDecision` и `continue`, не имеют эффекта, потому что действие, которое они контролировали, уже завершено.

<h3 id="configure-an-async-hook">
  Настройка асинхронного hook
</h3>

Добавьте `"async": true` к конфигурации command hook для запуска его в фоне без блокировки Claude. Это поле доступно только на hooks `type: "command"`.

Этот hook запускает скрипт тестирования после каждого вызова инструмента `Write`. Claude продолжает работать немедленно, пока `run-tests.sh` выполняется до 120 секунд. Когда скрипт завершается, его выход доставляется на следующий ход разговора:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true,
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

Поле `timeout` устанавливает максимальное время в секундах для фонового процесса. Если не указано, асинхронные hooks используют тот же 10-минутный таймаут по умолчанию, что и синхронные hooks.

<h3 id="how-async-hooks-execute">
  Как выполняются асинхронные hooks
</h3>

Когда срабатывает асинхронный hook, Claude Code запускает процесс hook и немедленно продолжает без ожидания его завершения. Hook получает те же JSON входные данные через stdin, что и синхронный hook.

После выхода фонового процесса, если hook произвёл JSON ответ с полем `additionalContext`, это содержимое доставляется Claude как контекст на следующем ходу разговора. Поле `systemMessage` показывается вам, а не Claude.

Claude Code проверяет, что JSON ответ соответствует той же [схеме выходных данных](#json-output), что и синхронные hooks, и отбрасывает любое поле, значение которого имеет неправильный тип, например `systemMessage`, который не является строкой, вместо его доставки. Запустите с `--debug` для просмотра предупреждения, называющего каждое отброшенное поле. До версии v2.1.202 неправильно сформированный JSON выход из асинхронного hook мог привести к сбою сеанса, и сбой повторялся каждый раз при возобновлении сеанса.

Уведомления о завершении асинхронного hook подавляются по умолчанию. Чтобы их увидеть, включите подробный режим с помощью `Ctrl+O` или запустите Claude Code с `--verbose`.

<h3 id="run-tests-after-file-changes">
  Запуск тестов после изменения файлов
</h3>

Этот hook запускает набор тестов в фоне всякий раз, когда Claude пишет файл, затем сообщает результаты обратно Claude при завершении тестов. Сохраните этот скрипт в `.claude/hooks/run-tests-async.sh` в вашем проекте и сделайте его исполняемым с помощью `chmod +x`:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Read hook input from stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Only run tests for source files
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Run tests and report results to Claude via additionalContext
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Затем добавьте эту конфигурацию в `.claude/settings.json` в корне вашего проекта. Флаг `async: true` позволяет Claude продолжать работу, пока тесты запускаются:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true,
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Ограничения
</h3>

Асинхронные hooks имеют несколько ограничений по сравнению с синхронными hooks:

* Только hooks `type: "command"` поддерживают `async`. Prompt-based hooks не могут запускаться асинхронно.
* Асинхронные hooks не могут блокировать вызовы инструментов или возвращать решения. К моменту завершения hook действие, вызвавшее его, уже произошло.
* Выход hook доставляется на следующий ход разговора. Если сеанс неактивен, ответ ждёт до следующего взаимодействия пользователя. Исключение: hook `asyncRewake`, который выходит с кодом 2, пробуждает Claude немедленно даже когда сеанс неактивен.
* Каждое выполнение создаёт отдельный фоновый процесс. Нет дедупликации между несколькими срабатываниями одного и того же асинхронного hook.

<h2 id="security-considerations">
  Соображения безопасности
</h2>

<h3 id="disclaimer">
  Отказ от ответственности
</h3>

Command hooks запускаются с полными разрешениями системного пользователя.

<Warning>
  Command hooks выполняют команды оболочки с вашими полными разрешениями пользователя. Они могут изменять, удалять или получать доступ к любым файлам, к которым может получить доступ ваша учётная запись пользователя. Проверьте и протестируйте все команды hook перед добавлением их в вашу конфигурацию.
</Warning>

<h3 id="security-best-practices">
  Лучшие практики безопасности
</h3>

Помните об этих практиках при написании hooks:

* **Проверяйте и санитизируйте входные данные**: никогда не доверяйте входным данным вслепую
* **Всегда заключайте переменные оболочки в кавычки**: используйте `"$VAR"` не `$VAR`
* **Блокируйте обход пути**: проверяйте наличие `..` в путях файлов
* **Используйте абсолютные пути**: указывайте полные пути для скриптов. В форме exec используйте `${CLAUDE_PROJECT_DIR}` и путь не требует кавычек. В форме shell оберните его в двойные кавычки
* **Пропускайте чувствительные файлы**: избегайте `.env`, `.git/`, ключей и т. д.

<h2 id="windows-powershell-tool">
  Windows PowerShell tool
</h2>

На Windows вы можете запустить отдельные hooks в PowerShell, установив `"shell": "powershell"` на command hook. Hooks порождают PowerShell напрямую, поэтому это работает независимо от того, установлен ли `CLAUDE_CODE_USE_POWERSHELL_TOOL`. Claude Code автоматически обнаруживает `pwsh.exe`, исполняемый файл PowerShell 7 и более поздних версий, и переходит на `powershell.exe` для Windows PowerShell 5.1.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Чтобы ссылаться на корневой каталог проекта из команды PowerShell в форме shell, напишите `${CLAUDE_PROJECT_DIR}` или `$env:CLAUDE_PROJECT_DIR`. Начиная с версии 2.1.198, Claude Code переписывает заполнители `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` и `${CLAUDE_PLUGIN_DATA}` в команде PowerShell в форме shell в форму `${env:NAME}` PowerShell, независимо от того, определён ли hook в `settings.json`, плагине или навыке. PowerShell затем разрешает значение из экспортированного окружения после анализа, поэтому заполнитель работает внутри строк в двойных кавычках, но не внутри строк в одинарных кавычках, где PowerShell никогда не расширяет переменные.

До версии 2.1.198 эта переписка применялась только к plugin hooks. В более ранних версиях hook в `settings.json` требует формы `$env:` или [exec form](#exec-form-and-shell-form), где `${CLAUDE_PROJECT_DIR}` подставляется в каждый элемент `args` независимо от того, где определён hook.

Не пишите простое написание `$CLAUDE_PROJECT_DIR` в hook PowerShell. PowerShell анализирует его как неопределённую локальную переменную и разрешает её в `$null`, что оставляет путь скрипта без префикса корневого каталога проекта. Claude Code не переписывает эту форму; вместо этого она регистрирует предупреждение в [debug log](#debug-hooks).

Пример ниже показывает hook в `settings.json`, который запускает скрипт проекта с формой `$env:`, которая работает на каждой версии:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Debug hooks
</h2>

Детали выполнения hooks, включая информацию о том, какие hooks совпали, их коды выхода и полный stdout и stderr, записываются в файл отладочного журнала. Запустите Claude Code с `claude --debug-file <path>` для записи журнала в известное расположение, или запустите `claude --debug` и прочитайте журнал в `~/.claude/debug/<session-id>.txt`. Флаг `--debug` не выводит на терминал.

```text theme={null}
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 600000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

Для более детальной информации о совпадении hooks установите `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` для просмотра дополнительных строк логирования, таких как количество совпадений фильтра hook и совпадение запроса.

Для устранения неполадок распространённых проблем, таких как hooks, которые не срабатывают, бесконечные циклы Stop hook или ошибки конфигурации, см. [Limitations and troubleshooting](/docs/ru/hooks-guide#limitations-and-troubleshooting) в руководстве. Для более широкого диагностического пошагового руководства, охватывающего `/context`, `/doctor` и приоритет параметров, см. [Debug your config](/docs/ru/debug-your-config).
