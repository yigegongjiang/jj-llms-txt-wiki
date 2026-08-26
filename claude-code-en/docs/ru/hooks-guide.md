> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Автоматизация действий с помощью hooks

> Запускайте команды оболочки автоматически, когда Claude Code редактирует файлы, завершает задачи или требует ввода. Форматируйте код, отправляйте уведомления, проверяйте команды и применяйте правила проекта.

Hooks — это определяемые пользователем команды оболочки, которые выполняются в определённых точках жизненного цикла Claude Code. Они обеспечивают детерминированное управление поведением Claude Code, гарантируя, что определённые действия всегда происходят, а не полагаясь на то, что LLM выберет их запуск. Используйте hooks для применения правил проекта, автоматизации повторяющихся задач и интеграции Claude Code с вашими существующими инструментами.

Для решений, требующих суждения, а не детерминированных правил, вы также можете использовать [hooks на основе подсказок](#prompt-based-hooks) или [hooks на основе агентов](#agent-based-hooks), которые используют модель Claude для оценки условий.

Для других способов расширения Claude Code см. [skills](/docs/ru/skills) для предоставления Claude дополнительных инструкций и исполняемых команд, [subagents](/docs/ru/sub-agents) для запуска задач в изолированных контекстах и [plugins](/docs/ru/plugins) для упаковки расширений для совместного использования в проектах.

<Tip>
  Это руководство охватывает распространённые варианты использования и как начать работу. Для полных схем событий, форматов JSON ввода/вывода и расширенных функций, таких как асинхронные hooks и MCP tool hooks, см. [справочник Hooks](/docs/ru/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Настройка вашего первого hook
</h2>

Чтобы создать hook, добавьте блок `hooks` в [файл параметров](#configure-hook-location). Это пошаговое руководство создаёт hook для уведомлений на рабочем столе, чтобы вы получали оповещение всякий раз, когда Claude ждёт вашего ввода вместо того, чтобы смотреть на терминал.

<Steps>
  <Step title="Добавьте hook в ваши параметры">
    Откройте `~/.claude/settings.json` и добавьте hook `Notification`. Пример ниже использует `osascript` для macOS; см. [Получайте уведомления, когда Claude требует ввода](#get-notified-when-claude-needs-input) для команд Linux и Windows.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    Если ваш файл параметров уже имеет ключ `hooks`, добавьте `Notification` как соседний элемент существующих ключей событий, а не заменяйте весь объект. Каждое имя события — это ключ внутри единственного объекта `hooks`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    Вы также можете попросить Claude написать hook для вас, описав то, что вы хотите, в CLI.
  </Step>

  <Step title="Проверьте конфигурацию">
    Введите `/hooks` для открытия браузера hooks. Вы увидите список всех доступных событий hook с количеством рядом с каждым событием, которое имеет настроенные hooks. Выберите `Notification` для подтверждения того, что ваш новый hook появляется в списке. Выбор hook показывает его детали: событие, matcher, тип, исходный файл и команду.
  </Step>

  <Step title="Протестируйте hook">
    Нажмите `Esc` для возврата в CLI. Попросите Claude сделать что-то, требующее разрешения, затем переключитесь с терминала. Вы должны получить уведомление на рабочем столе.
  </Step>
</Steps>

<Tip>
  Меню `/hooks` доступно только для чтения. Чтобы добавить, изменить или удалить hooks, отредактируйте JSON параметров напрямую или попросите Claude сделать изменение.
</Tip>

<h2 id="what-you-can-automate">
  Что вы можете автоматизировать
</h2>

Hooks позволяют запускать код в ключевых точках жизненного цикла Claude Code: форматировать файлы после редактирования, блокировать команды перед их выполнением, отправлять уведомления, когда Claude требует ввода, внедрять контекст при запуске сеанса и многое другое. Для полного списка событий hook см. [справочник Hooks](/docs/ru/hooks#hook-lifecycle).

Каждый пример включает готовый к использованию блок конфигурации, который вы добавляете в [файл параметров](#configure-hook-location).

Для примера использования в production с hooks, которые запускают отдельную проверку модели и передают результаты обратно в сеанс, см. [как плагин `security-guidance` интегрируется с Claude Code](/docs/ru/security-guidance#how-the-plugin-integrates-with-claude-code).

<h3 id="get-notified-when-claude-needs-input">
  Получайте уведомления, когда Claude требует ввода
</h3>

Получайте уведомление на рабочем столе всякий раз, когда Claude завершает работу и требует вашего ввода, чтобы вы могли переключиться на другие задачи без проверки терминала.

Этот hook использует событие `Notification`, которое срабатывает, когда Claude ждёт ввода или разрешения. Каждая вкладка ниже использует собственную команду уведомления платформы. Добавьте это в `~/.claude/settings.json`:

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Если уведомление не появляется">
      `osascript` маршрутизирует уведомления через встроенное приложение Script Editor. Если Script Editor не имеет разрешения на уведомления, команда молча не выполняется, и macOS не будет вас просить предоставить его. Запустите это в Terminal один раз, чтобы Script Editor появился в ваших параметрах уведомлений:

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      Ничего не появится пока. Откройте **System Settings > Notifications**, найдите **Script Editor** в списке и включите **Allow Notifications**. Запустите команду снова, чтобы подтвердить, что тестовое уведомление появляется.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Пустой `matcher` срабатывает на все типы уведомлений. Чтобы срабатывать только на определённые события, установите его на одно из этих значений:

| Matcher                | Срабатывает когда                                                                                                           |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude требует вашего одобрения использования инструмента                                                                   |
| `idle_prompt`          | Claude завершил работу и ждёт вашего следующего запроса                                                                     |
| `auth_success`         | Аутентификация завершена                                                                                                    |
| `elicitation_dialog`   | Сервер MCP открывает форму выяснения                                                                                        |
| `elicitation_complete` | Форма выяснения MCP отправлена или закрыта                                                                                  |
| `elicitation_response` | Ответ выяснения MCP отправлен обратно на сервер                                                                             |
| `agent_needs_input`    | Фоновый сеанс начинает ожидание вашего ввода. Срабатывает только при открытом [представлении агента](/docs/ru/agent-view)        |
| `agent_completed`      | Фоновый сеанс завершается или завершается с ошибкой. Срабатывает только при открытом [представлении агента](/docs/ru/agent-view) |

Matchers `agent_needs_input` и `agent_completed` требуют Claude Code v2.1.198 или позже.

Введите `/hooks` и выберите `Notification`, чтобы подтвердить, что hook зарегистрирован. Для полной схемы события см. [справочник Notification](/docs/ru/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Автоматическое форматирование кода после редактирования
</h3>

Автоматически запускайте [Prettier](https://prettier.io/) на каждом файле, который редактирует Claude, чтобы форматирование оставалось согласованным без ручного вмешательства.

Этот hook использует событие `PostToolUse` с matcher `Edit|Write`, поэтому он запускается только после инструментов редактирования файлов. Команда извлекает путь отредактированного файла с помощью [`jq`](https://jqlang.github.io/jq/) и передаёт его в Prettier. Добавьте это в `.claude/settings.json` в корне вашего проекта:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

На Claude Code v2.1.191 или позже вы также можете написать matcher как `Edit,Write`, так как `|` и `,` являются взаимозаменяемыми разделителями списков для matcher инструментов на этих версиях.

<Note>
  Примеры Bash на этой странице используют `jq` для анализа JSON. Установите его с помощью `brew install jq` на macOS, `apt-get install jq` на Debian и Ubuntu или см. [загрузки `jq`](https://jqlang.github.io/jq/download/).
</Note>

<h3 id="block-edits-to-protected-files">
  Блокировка редактирования защищённых файлов
</h3>

Предотвратите изменение Claude чувствительных файлов, таких как `.env`, `package-lock.json` или что-либо в `.git/`. Claude получает обратную связь, объясняющую, почему редактирование было заблокировано, чтобы он мог скорректировать свой подход.

Этот пример использует отдельный файл скрипта, который вызывает hook. Скрипт проверяет путь целевого файла против списка защищённых шаблонов и выходит с кодом 2 для блокировки редактирования.

<Steps>
  <Step title="Создайте скрипт hook">
    Сохраните это в `.claude/hooks/protect-files.sh`:

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="Сделайте скрипт исполняемым на macOS и Linux">
    Скрипты hook должны быть исполняемыми для запуска Claude Code:

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="Зарегистрируйте hook">
    Добавьте hook `PreToolUse` в `.claude/settings.json`, который запускает скрипт перед любым вызовом инструмента `Edit` или `Write`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Повторное внедрение контекста после компактирования
</h3>

Когда контекстное окно Claude заполняется, компактирование суммирует разговор для освобождения места. Это может привести к потере важных деталей. Используйте hook `SessionStart` с matcher `compact` для повторного внедрения критического контекста после каждого компактирования.

Любой текст, который ваша команда выводит в stdout, добавляется в контекст Claude. Этот пример напоминает Claude о соглашениях проекта и недавней работе. Добавьте это в `.claude/settings.json` в корне вашего проекта:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

Вы можете заменить `echo` любой командой, которая производит динамический вывод, например `git log --oneline -5` для отображения недавних коммитов. Для внедрения контекста при каждом запуске сеанса рассмотрите использование [CLAUDE.md](/docs/ru/memory) вместо этого. Для переменных окружения см. [`CLAUDE_ENV_FILE`](/docs/ru/hooks#persist-environment-variables) в справочнике.

<h3 id="audit-configuration-changes">
  Аудит изменений конфигурации
</h3>

Отслеживайте, когда файлы параметров или skills изменяются во время сеанса. Событие `ConfigChange` срабатывает, когда внешний процесс или редактор изменяет файл конфигурации, поэтому вы можете регистрировать изменения для соответствия или блокировать несанкционированные изменения.

Этот пример добавляет каждое изменение в журнал аудита. Добавьте это в `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

Matcher фильтрует по типу конфигурации: `user_settings`, `project_settings`, `local_settings`, `policy_settings` или `skills`. Для блокировки вступления изменения в силу выйдите с кодом 2 или верните `{"decision": "block"}`. См. [справочник ConfigChange](/docs/ru/hooks#configchange) для полной схемы ввода.

<h3 id="reload-environment-when-directory-or-files-change">
  Перезагрузка окружения при изменении каталога или файлов
</h3>

Некоторые проекты устанавливают разные переменные окружения в зависимости от того, в каком каталоге вы находитесь. Инструменты, такие как [direnv](https://direnv.net/), делают это автоматически в вашей оболочке, но инструмент Bash Claude не подхватывает эти изменения самостоятельно.

Сочетание hook `SessionStart` с hook `CwdChanged` исправляет это. `SessionStart` загружает переменные для каталога, в котором вы запускаетесь, а `CwdChanged` перезагружает их каждый раз, когда Claude меняет каталог. Оба записывают в `CLAUDE_ENV_FILE`, который Claude Code запускает как преамбулу скрипта перед каждой командой Bash. Добавьте это в `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Запустите `direnv allow` один раз в каждом каталоге, который имеет `.envrc`, чтобы direnv был разрешён загружать его. Если вы используете devbox или nix вместо direnv, тот же шаблон работает с `devbox shellenv` или `devbox global shellenv` вместо `direnv export bash`.

Чтобы реагировать на определённые файлы вместо каждого изменения каталога, используйте `FileChanged` с matcher, указывающим имена файлов для наблюдения, разделённые `|`. Для построения списка наблюдения это значение разбивается на буквальные имена файлов, а не оценивается как regex. См. [FileChanged](/docs/ru/hooks#filechanged) для того, как то же значение также фильтрует, какие группы hook запускаются при изменении файла. Этот пример наблюдает `.envrc` и `.env` в рабочем каталоге:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

См. справочные записи [CwdChanged](/docs/ru/hooks#cwdchanged) и [FileChanged](/docs/ru/hooks#filechanged) для схем ввода, вывода `watchPaths` и деталей `CLAUDE_ENV_FILE`.

<h3 id="auto-approve-specific-permission-prompts">
  Автоматическое одобрение определённых запросов разрешений
</h3>

Пропустите диалог одобрения для вызовов инструментов, которые вы всегда разрешаете. Этот пример автоматически одобряет `ExitPlanMode`, инструмент, который Claude вызывает, когда он завершает представление плана и просит продолжить, чтобы вас не спрашивали каждый раз, когда план готов.

В отличие от примеров с кодом выхода выше, автоматическое одобрение требует, чтобы ваш hook написал решение JSON в stdout. Hook `PermissionRequest` срабатывает, когда Claude Code собирается показать диалог разрешения, и возврат `"behavior": "allow"` отвечает на него от вашего имени.

Matcher ограничивает hook только `ExitPlanMode`, поэтому никакие другие запросы не затрагиваются. Добавьте это в `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Когда hook одобряет, Claude Code выходит из режима плана и восстанавливает любой режим разрешения, который был активен перед входом в режим плана. Стенограмма показывает "Allowed by PermissionRequest hook" там, где появился бы диалог. Путь hook всегда сохраняет текущий разговор: он не может очистить контекст и начать свежий сеанс реализации так, как может диалог.

Чтобы установить определённый режим разрешения вместо этого, вывод вашего hook может включать массив `updatedPermissions` с записью `setMode`. Значение `mode` — это любой режим разрешения, такой как `default`, `acceptEdits` или `bypassPermissions`, и `destination: "session"` применяет его только для текущего сеанса.

<Note>
  `bypassPermissions` применяется только если сеанс был запущен с уже доступным режимом обхода: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` или `permissions.defaultMode: "bypassPermissions"` в параметрах, и не отключено [`permissions.disableBypassPermissionsMode`](/docs/ru/permissions#managed-settings). Это никогда не сохраняется как `defaultMode`.
</Note>

Чтобы переключить сеанс на `acceptEdits`, ваш hook пишет этот JSON в stdout:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Держите matcher как можно более узким. Соответствие `.*` или оставление matcher пустым автоматически одобрит каждый запрос разрешения, включая записи файлов и команды оболочки. См. [справочник PermissionRequest](/docs/ru/hooks#permissionrequest-decision-control) для полного набора полей решения.

<h2 id="how-hooks-work">
  Как работают hooks
</h2>

События hook срабатывают в определённых точках жизненного цикла Claude Code. Когда событие срабатывает, все соответствующие hooks запускаются параллельно, и идентичные команды hook автоматически дедублируются. Таблица ниже показывает каждое событие и когда оно срабатывает:

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

Каждый hook имеет `type`, который определяет, как он запускается. Большинство hooks используют `"type": "command"`, который запускает команду оболочки. Доступны четыре других типа:

* `"type": "http"`: POST данные события на URL. См. [HTTP hooks](#http-hooks).
* `"type": "mcp_tool"`: вызвать инструмент на уже подключённом MCP сервере. См. [MCP tool hooks](/docs/ru/hooks#mcp-tool-hook-fields).
* `"type": "prompt"`: однооборотная оценка LLM. См. [Prompt-based hooks](#prompt-based-hooks).
* `"type": "agent"`: многооборотная проверка с доступом к инструментам. Agent hooks являются экспериментальными и могут измениться. См. [Agent-based hooks](#agent-based-hooks).

<h3 id="combine-results-from-multiple-hooks">
  Объединение результатов из нескольких hooks
</h3>

Когда несколько hooks совпадают с одним и тем же событием, каждая команда hook выполняется до завершения, прежде чем Claude Code объединит результаты. Один hook, возвращающий `deny`, не останавливает выполнение родственных hooks. Не полагайтесь на `deny` одного hook для подавления побочных эффектов в другом hook.

После завершения всех соответствующих hooks Claude Code объединяет их выходные данные. Для решений о разрешениях `PreToolUse` побеждает наиболее ограничивающий ответ в порядке `deny`, `defer`, `ask`, `allow`. Текст из `additionalContext` сохраняется от каждого hook и передаётся Claude вместе.

Пример ниже регистрирует два hook `PreToolUse` на `Bash`. Первый добавляет каждую команду в файл журнала и выходит с 0. Второй запускает скрипт, который выходит с 2, чтобы отклонить, когда команда содержит `rm -rf`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Когда Claude пытается запустить `rm -rf /tmp/build`, оба hook выполняются параллельно. Hook логирования записывает команду в `~/.claude/bash.log` и выходит с 0, что не сообщает никакого решения. Hook защиты выходит с 2, что отклоняет вызов инструмента. Отклонение побеждает, поэтому Claude Code блокирует команду и показывает Claude stderr защиты. Запись в журнал по-прежнему записывается, потому что hook логирования уже выполнился.

<h3 id="read-input-and-return-output">
  Чтение ввода и возврат вывода
</h3>

Hooks взаимодействуют с Claude Code через stdin, stdout, stderr и коды выхода. Когда событие срабатывает, Claude Code передаёт данные, специфичные для события, в виде JSON в stdin вашего скрипта. Ваш скрипт читает эти данные, выполняет свою работу и сообщает Claude Code, что делать дальше, через код выхода.

<h4 id="hook-input">
  Ввод hook
</h4>

Каждое событие включает общие поля, такие как `session_id` и `cwd`, но каждый тип события добавляет разные данные. Например, когда Claude запускает команду Bash, hook `PreToolUse` получает что-то вроде этого на stdin:

```json theme={null}
{
  "session_id": "abc123",          // уникальный ID для этого сеанса
  "cwd": "/Users/sarah/myproject", // рабочий каталог при срабатывании события
  "hook_event_name": "PreToolUse", // какое событие запустило этот hook
  "tool_name": "Bash",             // инструмент, который Claude собирается использовать
  "tool_input": {                  // аргументы, которые Claude передал инструменту
    "command": "npm test"          // для Bash это команда оболочки
  }
}
```

Ваш скрипт может анализировать этот JSON и действовать на основе любого из этих полей. Hooks `UserPromptSubmit` получают текст `prompt` вместо этого, hooks `SessionStart` получают `source` (`startup`, `resume`, `clear` или `compact`) и так далее. См. [Common input fields](/docs/ru/hooks#common-input-fields) в справочнике для общих полей и раздел каждого события для схем, специфичных для события.

<h4 id="hook-output">
  Вывод hook
</h4>

Ваш скрипт сообщает Claude Code, что делать дальше, записывая в stdout или stderr и выходя с определённым кодом. Следующий hook `PreToolUse` блокирует команду:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  // stderr становится обратной связью Claude
  exit 2 // exit 2 = блокировать действие
fi

exit 0  // exit 0 = нет возражений; нормальный поток разрешений применяется
```

Код выхода определяет, что происходит дальше:

* **Exit 0**: hook не возражает и действие продолжается нормально. Для hook `PreToolUse` это не одобряет вызов инструмента: нормальный [permission flow](/docs/ru/permissions) по-прежнему применяется. Для hooks `UserPromptSubmit`, `UserPromptExpansion` и `SessionStart` всё, что вы пишете в stdout, добавляется в контекст Claude.
* **Exit 2**: действие блокируется. Напишите причину в stderr, и Claude получит её как обратную связь, чтобы он мог скорректировать. Некоторые события не могут быть заблокированы: для `SessionStart`, `Setup`, `Notification` и других, exit 2 показывает stderr пользователю и выполнение продолжается. См. [exit code 2 behavior per event](/docs/ru/hooks#exit-code-2-behavior-per-event) для полного списка.
* **Любой другой код выхода**: действие продолжается. Стенограмма показывает уведомление об ошибке `<hook name> hook error`, за которым следует первая строка stderr; полный stderr переходит в [debug log](/docs/ru/hooks#debug-hooks).

<h4 id="structured-json-output">
  Структурированный вывод JSON
</h4>

Коды выхода дают вам только возможность заблокировать или остаться молчаливым. Для большего контроля выйдите с 0 и выведите объект JSON в stdout вместо этого.

<Note>
  Используйте exit 2 для блокировки с сообщением stderr или exit 0 с JSON для структурированного управления. Не смешивайте их: Claude Code игнорирует JSON при выходе 2.
</Note>

Например, hook `PreToolUse` может отклонить вызов инструмента и сказать Claude почему, или передать его пользователю на одобрение:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

С `"deny"` Claude Code отменяет вызов инструмента и передаёт `permissionDecisionReason` обратно Claude. Эти значения `permissionDecision` специфичны для `PreToolUse`:

* `"allow"`: пропустить интерактивный запрос разрешения. Правила отказа и запроса, включая управляемые списки отказов предприятия, по-прежнему применяются, как и запросы для инструментов соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и MCP инструменты, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool)
* `"deny"`: отменить вызов инструмента и отправить причину Claude
* `"ask"`: показать запрос разрешения пользователю как обычно

Четвёртое значение, `"defer"`, доступно в [non-interactive mode](/docs/ru/headless) с флагом `-p`. Оно выходит из процесса с сохранённым вызовом инструмента, чтобы обёртка Agent SDK могла собрать ввод и возобновить. См. [Defer a tool call for later](/docs/ru/hooks#defer-a-tool-call-for-later) в справочнике.

Возврат `"allow"` пропускает интерактивный запрос, но не переопределяет [permission rules](/docs/ru/permissions#manage-permissions). Если правило отказа соответствует вызову инструмента, вызов блокируется даже когда ваш hook возвращает `"allow"`. Если правило запроса соответствует, пользователь по-прежнему получает запрос, как и инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и MCP инструменты, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool). Это означает, что правила отказа из любой области параметров, включая [managed settings](/docs/ru/settings#settings-files), всегда имеют приоритет над одобрениями hook.

Другие события используют разные шаблоны решений. Например, hooks `PostToolUse` и `Stop` используют поле `decision: "block"` верхнего уровня, а `PermissionRequest` использует `hookSpecificOutput.decision.behavior`. См. [summary table](/docs/ru/hooks#decision-control) в справочнике для полного разбора по событиям.

Для hooks `UserPromptSubmit` используйте `hookSpecificOutput.additionalContext` вместо этого для внедрения текста в контекст Claude. Вложите `additionalContext` внутри `hookSpecificOutput`; если вы поместите его на верхний уровень JSON, Claude Code молча его игнорирует. Например, этот вывод добавляет текущее состояние ветви к каждому запросу:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

См. [UserPromptSubmit decision control](/docs/ru/hooks#userpromptsubmit-decision-control) для полной формы вывода, включая блокировку запросов и установку заголовка сеанса.

Hooks с `type: "prompt"` обрабатывают вывод иначе: см. [Prompt-based hooks](#prompt-based-hooks).

<h3 id="filter-hooks-with-matchers">
  Фильтрация hooks с помощью matchers
</h3>

Без matcher hook срабатывает при каждом возникновении его события. Matchers позволяют вам сузить это. Например, если вы хотите запустить форматер только после редактирования файлов, а не после каждого вызова инструмента, добавьте matcher к вашему hook `PostToolUse`:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

Matcher `"Edit|Write"` срабатывает только, когда Claude использует инструмент `Edit` или `Write`, а не когда он использует `Bash`, `Read` или любой другой инструмент. На Claude Code v2.1.191 или позже запятая разделяет альтернативы так же, поэтому `"Edit, Write"` эквивалентна. См. [Matcher patterns](/docs/ru/hooks#matcher-patterns) для того, как простые имена и регулярные выражения оцениваются.

<Note>
  Claude может также создавать или изменять файлы, запуская команды оболочки через инструмент `Bash`. Если ваш hook должен видеть каждое изменение файла, например для сканирования соответствия или логирования аудита, добавьте hook [`Stop`](/docs/ru/hooks#stop), который сканирует рабочее дерево один раз за ход. Для покрытия за вызов вместо этого также соответствуйте `Bash` и пусть ваш скрипт перечисляет изменённые и неотслеживаемые файлы с помощью `git status --porcelain`.
</Note>

Каждый тип события соответствует определённому полю:

| Событие                                                                                                                                                         | Что фильтрует matcher                                                             | Примеры значений matcher                                                                                                                                                            |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | имя инструмента                                                                   | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                                  | как начался сеанс                                                                 | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                                         | какой флаг CLI запустил setup                                                     | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                                    | почему закончился сеанс                                                           | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                                  | тип уведомления                                                                   | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                                 | тип агента                                                                        | `general-purpose`, `Explore`, `Plan` или пользовательские имена агентов                                                                                                             |
| `PreCompact`, `PostCompact`                                                                                                                                     | что запустило компактирование                                                     | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                                  | тип агента                                                                        | те же значения, что и `SubagentStart`                                                                                                                                               |
| `ConfigChange`                                                                                                                                                  | источник конфигурации                                                             | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `StopFailure`                                                                                                                                                   | тип ошибки                                                                        | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                                            | причина загрузки                                                                  | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `Elicitation`                                                                                                                                                   | имя MCP сервера                                                                   | ваши настроенные имена MCP серверов                                                                                                                                                 |
| `ElicitationResult`                                                                                                                                             | имя MCP сервера                                                                   | те же значения, что и `Elicitation`                                                                                                                                                 |
| `FileChanged`                                                                                                                                                   | буквальные имена файлов для наблюдения (см. [FileChanged](/docs/ru/hooks#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                           | имя команды                                                                       | ваши имена skill или команд                                                                                                                                                         |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | поддержка matcher отсутствует                                                     | всегда срабатывает при каждом возникновении                                                                                                                                         |

Вкладки ниже показывают несколько дополнительных matchers на разных типах событий.

<Tabs>
  <Tab title="Логирование каждой команды Bash">
    Соответствуйте только вызовам инструмента `Bash` и логируйте каждую команду в файл. Событие `PostToolUse` срабатывает после завершения команды, поэтому `tool_input.command` содержит то, что запустилось. Hook получает данные события в виде JSON на stdin, и `jq -r '.tool_input.command'` извлекает только строку команды, которую `>>` добавляет в файл журнала:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Соответствие MCP инструментам">
    MCP инструменты используют другое соглашение об именовании, чем встроенные инструменты: `mcp__<server>__<tool>`, где `<server>` — имя MCP сервера, а `<tool>` — инструмент, который он предоставляет. Например, `mcp__github__search_repositories` или `mcp__filesystem__read_file`. Инструменты из [plugin-bundled server](/docs/ru/mcp#plugin-provided-mcp-servers) используют вместо этого сегмент сервера с областью видимости, такой как `mcp__plugin_my-plugin_db__query`. Используйте matcher regex для нацеливания на все инструменты с определённого сервера, или соответствуйте серверам с шаблоном, таким как `mcp__.*__write.*`. См. [Match MCP tools](/docs/ru/hooks#match-mcp-tools) в справочнике для полного списка примеров.

    Команда ниже извлекает имя инструмента из JSON ввода hook с помощью `jq` и записывает его в stderr. Запись в stderr сохраняет stdout чистым для вывода JSON и отправляет сообщение в [debug log](/docs/ru/hooks#debug-hooks):

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Очистка при завершении сеанса">
    Событие `SessionEnd` поддерживает matchers на причину завершения сеанса. Этот hook срабатывает только на `clear` (когда вы запускаете `/clear`), а не на нормальные выходы:

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Для полного синтаксиса matcher см. [Hooks reference](/docs/ru/hooks#configuration).

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  Фильтрация по имени инструмента и аргументам с помощью поля `if`
</h4>

Поле `if` использует [permission rule syntax](/docs/ru/permissions) для фильтрации hooks по имени инструмента и аргументам вместе, поэтому процесс hook порождается только когда вызов инструмента совпадает. Это выходит за рамки `matcher`, который фильтрует на уровне группы только по имени инструмента.

Например, чтобы запустить hook только когда Claude использует команды `git` вместо всех команд Bash:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Запускается ли процесс hook команды зависит от формы вашего шаблона `if` и команды Bash, которую Claude вызывает:

| Шаблон `if`        | Команда Bash           | Hook запускается? | Почему                                                                                                                   |
| :----------------- | :--------------------- | :---------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | да                | имя команды совпадает                                                                                                    |
| `Bash(git *)`      | `npm test && git push` | да                | каждая подкоманда проверяется; `git push` совпадает                                                                      |
| `Bash(git *)`      | `echo $(git log)`      | да                | команды внутри `$()` и обратных кавычек проверяются; `git log` совпадает                                                 |
| `Bash(git *)`      | `echo $(date)`         | нет               | ни одна подкоманда не совпадает с `git *`                                                                                |
| `Bash(git push *)` | `echo $(date)`         | да                | шаблоны, которые указывают больше, чем имя команды, запускают hook в любом случае на `$()`, обратных кавычках или `$VAR` |

Фильтр также открывается с ошибкой, запуская ваш hook независимо от шаблона, когда команда Bash не может быть проанализирована. Поскольку фильтр работает по принципу лучшего усилия, используйте [permission system](/docs/ru/permissions) вместо hook для обеспечения жёсткого разрешения или отказа.

Поле `if` принимает те же шаблоны, что и правила разрешений: `"Bash(git *)"`, `"Edit(*.ts)"` и так далее. Для соответствия нескольким именам инструментов используйте отдельные обработчики каждый со своим значением `if`, или соответствуйте на уровне `matcher`, где поддерживается чередование трубой.

`if` работает только на событиях инструментов: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` и `PermissionDenied`. Добавление его к любому другому событию предотвращает запуск hook.

<h3 id="configure-hook-location">
  Настройка местоположения hook
</h3>

Где вы добавляете hook, определяет его область:

| Местоположение                                              | Область                      | Общий доступ                                  |
| :---------------------------------------------------------- | :--------------------------- | :-------------------------------------------- |
| `~/.claude/settings.json`                                   | Все ваши проекты             | Нет, локально на вашей машине                 |
| `.claude/settings.json`                                     | Один проект                  | Да, можно зафиксировать в репо                |
| `.claude/settings.local.json`                               | Один проект                  | Нет, gitignored когда Claude Code создаёт его |
| Managed policy settings                                     | Организация                  | Да, контролируется администратором            |
| [Plugin](/docs/ru/plugins) `hooks/hooks.json`                    | Когда плагин включен         | Да, упакован с плагином                       |
| [Skill](/docs/ru/skills) или [agent](/docs/ru/sub-agents) frontmatter | Пока skill или agent активны | Да, определено в файле компонента             |

Запустите [`/hooks`](/docs/ru/hooks#the-%2Fhooks-menu) в Claude Code для просмотра всех настроенных hooks, сгруппированных по событиям.

Чтобы отключить hooks, установите `"disableAllHooks": true` в вашем файле параметров. Hooks, настроенные в управляемых параметрах, по-прежнему запускаются, если `disableAllHooks` также не установлен там.

Если вы редактируете файлы параметров напрямую во время работы Claude Code, наблюдатель файлов обычно автоматически подхватывает изменения hook.

<h2 id="prompt-based-hooks">
  Hooks на основе подсказок
</h2>

Для решений, требующих суждения, а не детерминированных правил, используйте hooks `type: "prompt"`. Вместо запуска команды оболочки Claude Code отправляет вашу подсказку и данные ввода hook модели Claude (Haiku по умолчанию) для принятия решения. Вы можете указать другую модель с полем `model`, если вам нужна большая возможность.

Единственная работа модели — вернуть решение да/нет в виде JSON:

* `"ok": true`: действие продолжается
* `"ok": false`: что происходит, зависит от события:
  * `Stop` и `SubagentStop`: значение `reason` передаётся обратно Claude, чтобы он продолжал работать
  * `PreToolUse`: вызов инструмента отклоняется и значение `reason` возвращается Claude как ошибка инструмента, чтобы он мог скорректировать и продолжить
  * `PostToolUse`, `PostToolBatch`, `UserPromptSubmit` и `UserPromptExpansion`: ход завершается и значение `reason` появляется в чате как строка предупреждения

Этот пример использует hook `Stop` для запроса модели, завершены ли все запрошенные задачи. Если модель возвращает `"ok": false`, Claude продолжает работать и использует `reason` как свою следующую инструкцию:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

Для полных параметров конфигурации см. [Hooks на основе подсказок](/docs/ru/hooks#prompt-based-hooks) в справочнике.

<h2 id="agent-based-hooks">
  Hooks на основе агентов
</h2>

<Warning>
  Hooks агентов являются экспериментальными. Поведение и конфигурация могут измениться в будущих выпусках. Для производственных рабочих процессов предпочитайте [command hooks](/docs/ru/hooks#command-hook-fields).
</Warning>

Когда проверка требует проверки файлов или запуска команд, используйте hooks `type: "agent"`. В отличие от hooks подсказок, которые делают один вызов LLM, hooks агентов порождают subagent, который может читать файлы, искать код и использовать другие инструменты для проверки условий перед возвратом решения.

Hooks агентов используют тот же формат ответа `"ok"` / `"reason"`, что и hooks подсказок, но с более длинным временем ожидания по умолчанию 60 секунд и до 50 оборотов использования инструмента.

Этот пример проверяет, что тесты проходят перед тем, как позволить Claude остановиться:

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

Используйте hooks подсказок, когда данных ввода hook достаточно для принятия решения. Используйте hooks агентов, когда вам нужно проверить что-то против фактического состояния кодовой базы.

Для полных параметров конфигурации см. [Hooks на основе агентов](/docs/ru/hooks#agent-based-hooks) в справочнике.

<h2 id="http-hooks">
  HTTP hooks
</h2>

Используйте hooks `type: "http"` для POST данных события на HTTP конечную точку вместо запуска команды оболочки. Конечная точка получает тот же JSON, который hook команды получил бы на stdin, и возвращает результаты через тело ответа HTTP, используя тот же формат JSON.

HTTP hooks полезны, когда вы хотите, чтобы веб-сервер, облачная функция или внешний сервис обрабатывали логику hook: например, общий сервис аудита, который регистрирует события использования инструмента в команде.

Этот пример отправляет каждое использование инструмента на локальный сервис логирования:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
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

Конечная точка должна вернуть тело ответа JSON, используя тот же [формат вывода](/docs/ru/hooks#json-output), что и hooks команд. Для блокировки вызова инструмента верните ответ 2xx с соответствующими полями `hookSpecificOutput`. Коды состояния HTTP сами по себе не могут блокировать действия.

Значения заголовков поддерживают интерполяцию переменных окружения, используя синтаксис `$VAR_NAME` или `${VAR_NAME}`. Разрешены только переменные, указанные в массиве `allowedEnvVars`; все остальные ссылки `$VAR` остаются пустыми.

Для полных параметров конфигурации и обработки ответов см. [HTTP hooks](/docs/ru/hooks#http-hook-fields) в справочнике.

<h2 id="limitations-and-troubleshooting">
  Ограничения и устранение неполадок
</h2>

<h3 id="limitations">
  Ограничения
</h3>

Помните об этих ограничениях при разработке hooks:

* Command hooks взаимодействуют только через stdout, stderr и коды выхода. Они не могут запускать команды `/` или вызовы инструментов. Текст, возвращённый через `additionalContext`, внедряется как системное напоминание, которое Claude читает как простой текст. HTTP hooks взаимодействуют через тело ответа вместо этого.
* Время ожидания hook варьируется в зависимости от типа. Переопределите для каждого hook с помощью поля `timeout` в секундах.
  * `command`, `http`, `mcp_tool`: 10 минут. `UserPromptSubmit` снижает эти значения до 30 секунд, а `MessageDisplay` снижает их до 10 секунд.
  * `prompt`: 30 секунд.
  * `agent`: 60 секунд.
* Hooks `PostToolUse` не могут отменить действия, так как инструмент уже выполнен.
* Hooks `PermissionRequest` не срабатывают в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`. Используйте hooks `PreToolUse` для автоматизированных решений разрешений.
* Hooks `Stop` срабатывают всякий раз, когда Claude завершает ответ, а не только при завершении задачи. Они не срабатывают при прерывании пользователем. Ошибки API срабатывают [StopFailure](/docs/ru/hooks#stopfailure) вместо этого.
* Когда несколько hooks `PreToolUse` возвращают [`updatedInput`](/docs/ru/hooks#pretooluse) для переписания аргументов инструмента, последний завершённый побеждает. Поскольку hooks запускаются параллельно, порядок недетерминирован. Избегайте наличия более одного hook, изменяющего ввод одного и того же инструмента.

<h3 id="hooks-and-permission-modes">
  Hooks и режимы разрешений
</h3>

Hooks `PreToolUse` срабатывают перед любой проверкой режима разрешений. Hook, возвращающий `permissionDecision: "deny"`, блокирует инструмент даже в режиме `bypassPermissions` или с `--dangerously-skip-permissions`. Это позволяет вам применять политику, которую пользователи не могут обойти, изменив свой режим разрешений.

Обратное неверно: hook, возвращающий `"allow"`, не обходит правила отказа из параметров, и он не может подавить запрос для инструментов соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) или инструментов MCP, отмеченных [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool). Hooks могут ужесточить ограничения, но не ослабить их сверх того, что разрешают правила разрешений.

<h3 id="hook-not-firing">
  Hook не срабатывает
</h3>

Hook настроен, но никогда не выполняется.

* Запустите `/hooks` и подтвердите, что hook появляется под правильным событием
* Проверьте, что шаблон matcher точно соответствует имени инструмента. Matchers чувствительны к регистру
* Убедитесь, что вы запускаете правильный тип события: `PreToolUse` срабатывает перед выполнением инструмента, `PostToolUse` срабатывает после
* Если используете hooks `PermissionRequest` в неинтерактивном режиме с флагом `-p`, переключитесь на `PreToolUse` вместо этого

<h3 id="hook-error-in-output">
  Ошибка hook в выводе
</h3>

Вы видите сообщение вроде "PreToolUse hook error: ..." в стенограмме.

* Ваш скрипт неожиданно вышел с ненулевым кодом. Протестируйте его вручную, передав образец JSON:
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Проверьте код выхода
  ```
* Если вы видите "command not found", используйте абсолютные пути или `${CLAUDE_PROJECT_DIR}` для ссылки на скрипты. Чтобы полностью избежать экранирования оболочки, добавьте `"args": []` для переключения на [exec form](/docs/ru/hooks#exec-form-and-shell-form), который порождает скрипт напрямую без оболочки
* Если вы видите "jq: command not found", установите `jq` или используйте Python/Node.js для анализа JSON
* Если скрипт вообще не запускается, сделайте его исполняемым: `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` показывает, что hooks не настроены
</h3>

Вы отредактировали файл параметров, но hooks не появляются в меню.

* Редактирования файлов обычно подхватываются автоматически. Если они не появились через несколько секунд, наблюдатель файлов мог пропустить изменение: перезагрузите сеанс для принудительной перезагрузки.
* Убедитесь, что ваш JSON действителен: конечные запятые и комментарии не допускаются
* Подтвердите, что файл параметров находится в правильном месте: `.claude/settings.json` для hooks проекта, `~/.claude/settings.json` для глобальных hooks

<h3 id="stop-hook-hits-the-block-cap">
  Stop hook достигает предела блокировки
</h3>

Claude продолжает работать вместо остановки, а затем завершает ход с предупреждением о том, что Stop hook заблокировал слишком много раз подряд без прогресса.

Claude Code переопределяет Stop hook после того, как он блокирует восемь раз подряд без прогресса. Ваш скрипт hook должен проверить, не срабатывал ли он уже. Проанализируйте поле `stop_hook_active` из JSON ввода и выйдите рано, если оно `true`:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Позволить Claude остановиться
fi
# ... остальная логика вашего hook
```

Если ваш hook законно нуждается в более чем восьми итерациях для сходимости, повысьте предел с помощью [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/ru/env-vars).

<h3 id="json-validation-failed">
  Ошибка валидации JSON
</h3>

Claude Code показывает ошибку анализа JSON, даже если ваш скрипт hook выводит действительный JSON.

Когда Claude Code запускает hook-команду в форме shell (без `args`), он порождает `sh -c` на macOS и Linux или Git Bash на Windows по умолчанию. Эта оболочка неинтерактивна, но Git Bash и некоторые конфигурации, такие как `BASH_ENV`, указывающий на `~/.bashrc`, всё ещё источают ваш профиль. Если этот профиль содержит безусловные операторы `echo`, вывод добавляется к JSON вашего hook:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code пытается проанализировать это как JSON и не удаётся. Чтобы исправить это, оберните операторы echo в вашем профиле оболочки, чтобы они запускались только в интерактивных оболочках:

```bash theme={null}
# В ~/.zshrc или ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

Переменная `$-` содержит флаги оболочки, и `i` означает интерактивный. Hooks запускаются в неинтерактивных оболочках, поэтому echo пропускается.

<h3 id="debug-techniques">
  Методы отладки
</h3>

Представление стенограммы, переключаемое с помощью `Ctrl+O`, показывает однострочную сводку для каждого hook, который срабатывал: успех молчит, блокирующие ошибки показывают stderr, и неблокирующие ошибки показывают уведомление об ошибке `<hook name> hook error`, за которым следует первая строка stderr.

Для полных деталей выполнения, включая какие hooks совпали, их коды выхода, stdout и stderr, прочитайте журнал отладки. Запустите Claude Code с `claude --debug-file /tmp/claude.log` для записи в известный путь, затем `tail -f /tmp/claude.log` в другом терминале. Если вы запустили без этого флага, запустите `/debug` во время сеанса для включения логирования и поиска пути журнала.

<h2 id="learn-more">
  Узнайте больше
</h2>

* [Справочник Hooks](/docs/ru/hooks): полные схемы событий, формат вывода JSON, асинхронные hooks и MCP tool hooks
* [Соображения безопасности](/docs/ru/hooks#security-considerations): просмотрите перед развёртыванием hooks в общих или производственных средах
* [Пример валидатора команд Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): полная справочная реализация
