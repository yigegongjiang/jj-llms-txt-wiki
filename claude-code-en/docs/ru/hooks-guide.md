> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Автоматизация действий с помощью hooks

> Запускайте команды оболочки автоматически, когда Claude Code редактирует файлы, завершает задачи или требует ввода. Форматируйте код, отправляйте уведомления, проверяйте команды и применяйте правила проекта.

Hooks — это определяемые пользователем команды оболочки. Claude Code запускает их в определённых точках своего жизненного цикла, что даёт вам детерминированное управление: определённые действия всегда происходят, а не полагаются на то, что LLM выберет их запуск. Используйте hooks для применения правил проекта, автоматизации повторяющихся задач и интеграции Claude Code с вашими существующими инструментами.

Для решений, требующих суждения, а не детерминированных правил, вы также можете использовать [hooks на основе подсказок](#prompt-based-hooks) или [hooks на основе агентов](#agent-based-hooks), которые используют модель Claude для оценки условий.

Для других способов расширения Claude Code см. [skills](/docs/ru/skills) для предоставления Claude дополнительных инструкций и исполняемых команд, [subagents](/docs/ru/sub-agents) для запуска задач в изолированных контекстах и [plugins](/docs/ru/plugins/overview) для упаковки расширений для совместного использования в проектах.

<Tip>
  Это руководство охватывает распространённые варианты использования и как начать работу. Для полных схем событий, форматов JSON ввода/вывода и расширенных функций, таких как асинхронные hooks и MCP tool hooks, см. [справочник Hooks](/docs/ru/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Настройка вашего первого hook
</h2>

Чтобы создать hook, добавьте блок `hooks` в [файл параметров](#configure-hook-location). Это пошаговое руководство создаёт hook для уведомлений на рабочем столе, чтобы вы получали оповещение всякий раз, когда Claude ждёт вашего ввода вместо того, чтобы смотреть на терминал.

<Steps>
  <Step title="Добавьте hook в ваши параметры">
    Откройте `~/.claude/settings.json` и добавьте hook `Notification`. Если файл не существует, создайте его. Пример ниже использует `osascript` для macOS; см. [Получайте уведомления, когда Claude требует ввода](#get-notified-when-claude-needs-input) для команд Linux и Windows.

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
    Нажмите `Esc` для возврата в CLI. Нажимайте `Shift+Tab` до тех пор, пока в строке состояния не появится `⏸ manual mode on`, попросите Claude сделать что-то, требующее разрешения, затем переключитесь с терминала. Вы должны получить уведомление на рабочем столе.
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

Этот hook использует событие `Notification`, которое Claude Code срабатывает, когда Claude ждёт ввода или разрешения. См. [когда срабатывает каждый тип уведомления](/docs/ru/hooks#notification) для точного времени. Каждая вкладка ниже использует собственную команду уведомления платформы. Добавьте это в `~/.claude/settings.json`:

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

    <Accordion title="Если уведомление не появляется">
      `notify-send` требует демона уведомлений рабочего стола, который не имеют безголовые серверы, сеансы SSH и большинство контейнеров. Сначала протестируйте команду напрямую:

      ```bash theme={null}
      notify-send 'Claude Code' 'test'
      ```

      Если команда не найдена, установите пакет `libnotify-bin` на Debian и Ubuntu или эквивалент вашего дистрибутива.
    </Accordion>
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

    <Accordion title="Если диалог не появляется">
      Эта команда открывает диалоговое окно, а не уведомление в углу экрана, поэтому диалог может открыться позади окна терминала. Сначала протестируйте команду напрямую в PowerShell. Если вы запускаете Claude Code внутри WSL, `powershell.exe` должен быть доступен в вашем `PATH` через Windows interop.
    </Accordion>
  </Tab>
</Tabs>

Пустой `matcher` срабатывает на все типы уведомлений. Чтобы срабатывать только на определённые события, установите его на одно из этих значений:

| Matcher                      | Срабатывает когда                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude требует вашего одобрения использования инструмента или [сетевого запроса](/docs/ru/sandboxing#network-isolation) изолированной команды, и запрос ждал около шести секунд                                                                                                                                                                                                                                                                                                                                                        |
| `idle_prompt`                | Claude завершил ответ около 60 секунд назад и вы не печатали с тех пор                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `auth_success`               | Аутентификация завершена                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `elicitation_dialog`         | Сервер MCP открывает форму выяснения и вы не печатали около шести секунд                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `elicitation_url_dialog`     | Сервер MCP просит вас открыть URL браузера и вы не печатали около шести секунд                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `elicitation_complete`       | Сервер MCP сообщает, что [выяснение в режиме URL](/docs/ru/hooks#elicitation-input) завершено                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `elicitation_response`       | Ответ выяснения MCP отправлен обратно на сервер                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `agent_needs_input`          | Фоновый сеанс начинает ожидание вашего ввода, пока открыто [представление агента](/docs/ru/agent-view), или текущий сеанс задаёт вам вопрос о настройке терминала [товарища команды агента](/docs/ru/agent-teams#choose-a-display-mode) и вы не печатали около шести секунд                                                                                                                                                                                                                                                                 |
| `agent_completed`            | Фоновый сеанс завершается или завершается с ошибкой. Срабатывает только при открытом [представлении агента](/docs/ru/agent-view)                                                                                                                                                                                                                                                                                                                                                                                                       |
| `quota_auto_resume_fired`    | Claude Code продолжает вашу задачу после того, как лимит использования claude.ai приостановил её: при сбросе или раньше, когда что-то, что вы делаете в Claude Code во время ожидания, например добавление кредитов использования, обновление плана или переключение моделей, делает использование доступным снова, с [исключением параметра модели](/docs/ru/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                                        |
| `quota_auto_resume_stale`    | Лимит использования claude.ai сбросился, пока ваш компьютер спал более чем около 30 минут. Claude Code ждёт, пока вы нажмёте `Enter` вместо продолжения. После более короткого сна он продолжает и срабатывает `quota_auto_resume_fired` вместо этого                                                                                                                                                                                                                                                                             |
| `quota_auto_resume_disabled` | Claude Code заканчивает своё ожидание лимита использования claude.ai без продолжения вашей задачи: [`autoContinueAtUsageLimit`](/docs/ru/settings-reference#autocontinueatusagelimit) отключён или сброс переместился более чем на 24 часа во время ожидания, которое Claude Code запустил самостоятельно, продолжаемая задача продолжала достигать лимита, или продолжение было заблокировано до того, как оно достигло модели. Не срабатывает, когда вы нажимаете `Esc` или `Ctrl+C`, или выбираете **Don't continue automatically** |

Claude Code по-разному рассчитывает время `permission_prompt` в терминале и в Claude Desktop, расширении VS Code и других хостах, которые отвечают на запросы разрешений через Agent SDK. См. [когда срабатывает каждый тип уведомления](/docs/ru/hooks#notification) для обоих времён.

Matchers `agent_needs_input` и `agent_completed` требуют Claude Code v2.1.198 или позже.

Matchers `quota_auto_resume_fired`, `quota_auto_resume_stale` и `quota_auto_resume_disabled` требуют Claude Code v2.1.234 или позже.

В сеансах терминала `permission_prompt` для сетевого запроса изолированной команды требует Claude Code v2.1.246 или позже.

`agent_needs_input` для вопроса о настройке терминала товарища требует Claude Code v2.1.248 или позже.

Введите `/hooks` и выберите `Notification`, чтобы подтвердить, что hook зарегистрирован. Для полной схемы события см. [справочник Notification](/docs/ru/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Автоматическое форматирование кода после редактирования
</h3>

Автоматически запускайте [Prettier](https://prettier.io/) на каждом файле, который редактирует Claude, чтобы форматирование оставалось согласованным без ручного вмешательства.

Этот hook использует событие `PostToolUse` с matcher `Edit|Write`, поэтому он запускается только после инструментов редактирования файлов. Команда извлекает путь отредактированного файла с помощью [`jq`](https://jqlang.org/) и передаёт его в Prettier. Добавьте это в `.claude/settings.json` в корне вашего проекта:

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

Чтобы протестировать hook, попросите Claude добавить строку с одинарными кавычками в файл JavaScript, затем откройте файл: с параметрами Prettier по умолчанию hook переписывает их на двойные кавычки.

Когда hook успешен, Claude Code ничего не показывает в разговоре. Чтобы подтвердить, что hook запустился, проверьте, что отредактированный файл переформатирован, или см. [Методы отладки](#debug-techniques).

Чтобы переформатировать определённый файл при любом его изменении, включая когда команда `Bash` переписывает его, используйте hook [FileChanged](/docs/ru/hooks#filechanged) вместо этого.

<Note>
  Примеры Bash на этой странице используют `jq` для анализа JSON. Установите его с помощью `brew install jq` на macOS, `apt-get install jq` на Debian и Ubuntu или см. [загрузки `jq`](https://jqlang.org/download/).
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

    # Normalize Windows backslash separators so the patterns below match
    FILE_PATH="${FILE_PATH//\\//}"

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

  <Step title="Протестируйте hook">
    Попросите Claude добавить комментарий в ваш файл `.env`. Claude Code блокирует редактирование перед его выполнением и передаёт сообщение `Blocked:` скрипта Claude как обратную связь.
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Повторное внедрение контекста после компактирования
</h3>

Когда контекстное окно Claude заполняется, компактирование суммирует разговор для освобождения места. Это может привести к потере важных деталей. Используйте hook `SessionStart` с matcher `compact` для повторного внедрения критического контекста после каждого компактирования.

Claude Code добавляет простой текст, который ваша команда выводит в stdout, в контекст Claude. Этот пример напоминает Claude о соглашениях проекта и недавней работе. Добавьте это в `.claude/settings.json` в корне вашего проекта:

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

Чтобы подтвердить, что hook записывает изменения, отредактируйте файл параметров в другом редакторе, пока сеанс запущен, затем откройте `~/claude-config-audit.log`: hook добавляет одну строку JSON на изменение с временной меткой, источником и путём файла.

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

В отличие от примеров с кодом выхода выше, автоматическое одобрение требует, чтобы ваш hook написал решение JSON в stdout. Claude Code запускает hooks `PermissionRequest`, когда собирается попросить вас разрешение, и если ваш hook возвращает `"behavior": "allow"`, Claude Code отвечает на запрос от вашего имени.

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
  `bypassPermissions` применяется только если сеанс был запущен с уже доступным режимом обхода: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` или `permissions.defaultMode: "bypassPermissions"` в [параметрах пользователя, `--settings` или управляемых параметрах](/docs/ru/settings-reference#permissions-defaultmode). Это не применяется, если режим обхода отключён [`permissions.disableBypassPermissionsMode`](/docs/ru/permissions#managed-settings), или если вы запустили сеанс в [ограниченном режиме](/docs/ru/cli-reference#cli-flags).

  Claude Code никогда не сохраняет его как `defaultMode`.
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

Держите matcher как можно более узким. Соответствие `.*` или оставление matcher пустым автоматически одобрит каждый запрос разрешения инструмента, включая записи файлов и команды оболочки. См. [справочник PermissionRequest](/docs/ru/hooks#permissionrequest-decision-control) для полного набора полей решения.

<h2 id="how-hooks-work">
  Как работают hooks
</h2>

Claude Code срабатывает события hook в определённых точках жизненного цикла. Когда событие срабатывает, Claude Code запускает все соответствующие hooks параллельно; см. [Hook handler fields](/docs/ru/hooks#hook-handler-fields) для того, как обрабатываются дублирующиеся обработчики. Таблица ниже показывает каждое событие и когда оно срабатывает:

| Событие               | Когда оно срабатывает                                                                                                                                                                                                                                                                                                   |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Когда сеанс начинается или возобновляется                                                                                                                                                                                                                                                                               |
| `Setup`               | Когда вы запускаете Claude Code с `--init-only`, или с `--init` или `--maintenance` в режиме `-p`. Для одноразовой подготовки в CI или скриптах                                                                                                                                                                         |
| `UserPromptSubmit`    | Когда вы отправляете запрос, прежде чем Claude его обработает                                                                                                                                                                                                                                                           |
| `UserPromptExpansion` | Когда команда, введённая пользователем, расширяется в запрос, прежде чем она достигнет Claude. Может заблокировать расширение                                                                                                                                                                                           |
| `PreToolUse`          | Перед выполнением вызова инструмента. Может заблокировать его                                                                                                                                                                                                                                                           |
| `PermissionRequest`   | Когда вызов инструмента требует решения о разрешении                                                                                                                                                                                                                                                                    |
| `PermissionDenied`    | Когда автоматический режим отклоняет вызов инструмента, включая отклонения без вердикта классификатора. Используйте JSON `hookSpecificOutput.retry: true`, чтобы сообщить модели, что она может повторить попытку отклонённого вызова инструмента. Claude Code игнорирует `retry`, когда классификатор не выдал вердикт |
| `PostToolUse`         | После успешного выполнения вызова инструмента                                                                                                                                                                                                                                                                           |
| `PostToolUseFailure`  | После неудачного выполнения вызова инструмента                                                                                                                                                                                                                                                                          |
| `PostToolBatch`       | После разрешения полного пакета параллельных вызовов инструментов, перед следующим вызовом модели                                                                                                                                                                                                                       |
| `Notification`        | Когда Claude Code отправляет уведомление                                                                                                                                                                                                                                                                                |
| `MessageDisplay`      | Во время отображения текста сообщения помощника                                                                                                                                                                                                                                                                         |
| `SubagentStart`       | Когда порождается подагент                                                                                                                                                                                                                                                                                              |
| `SubagentStop`        | Когда подагент завершает работу                                                                                                                                                                                                                                                                                         |
| `TaskCreated`         | Когда задача создаётся через `TaskCreate`                                                                                                                                                                                                                                                                               |
| `TaskCompleted`       | Когда задача отмечается как завершённая                                                                                                                                                                                                                                                                                 |
| `Stop`                | Когда Claude завершает ответ                                                                                                                                                                                                                                                                                            |
| `StopFailure`         | Когда ход завершается из-за ошибки API                                                                                                                                                                                                                                                                                  |
| `TeammateIdle`        | Когда товарищ по команде [команды агентов](/docs/ru/agent-teams) собирается перейти в режим ожидания                                                                                                                                                                                                                         |
| `InstructionsLoaded`  | Когда файл CLAUDE.md или `.claude/rules/*.md` загружается в контекст. Срабатывает при запуске сеанса и когда файлы ленивой загрузки загружаются во время сеанса                                                                                                                                                         |
| `ConfigChange`        | Когда файл конфигурации изменяется во время сеанса                                                                                                                                                                                                                                                                      |
| `CwdChanged`          | Когда рабочий каталог изменяется, например когда Claude выполняет команду `cd`. Полезно для реактивного управления окружением с помощью инструментов, таких как direnv                                                                                                                                                  |
| `DirectoryAdded`      | Когда рабочий каталог добавляется в середине сеанса через `/add-dir` или запрос управления SDK `register_repo_root`                                                                                                                                                                                                     |
| `FileChanged`         | Когда наблюдаемый файл изменяется на диске. Поле `matcher` указывает, какие имена файлов отслеживать                                                                                                                                                                                                                    |
| `WorktreeCreate`      | Когда worktree создаётся через `--worktree`, `isolation: "worktree"`, или для фонового сеанса. Заменяет поведение git по умолчанию                                                                                                                                                                                      |
| `WorktreeRemove`      | Когда worktree удаляется при выходе из сеанса, когда подагент завершает работу, или когда вы удаляете фоновый сеанс                                                                                                                                                                                                     |
| `PreCompact`          | Перед компактизацией контекста                                                                                                                                                                                                                                                                                          |
| `PostCompact`         | После завершения компактизации контекста                                                                                                                                                                                                                                                                                |
| `PreModelSwitch`      | Перед тем как Claude Code применяет переключение модели, которое вы или клиент запросили. Может заблокировать переключение                                                                                                                                                                                              |
| `PostModelSwitch`     | После изменения модели сеанса, включая изменения, которые Claude Code делает самостоятельно, такие как восстановление модели при возобновлении сеанса                                                                                                                                                                   |
| `Elicitation`         | Когда сервер MCP запрашивает ввод пользователя во время вызова инструмента                                                                                                                                                                                                                                              |
| `ElicitationResult`   | После того как пользователь отвечает на запрос MCP, перед отправкой ответа обратно на сервер                                                                                                                                                                                                                            |
| `SessionEnd`          | Когда сеанс завершается                                                                                                                                                                                                                                                                                                 |

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

Каждое событие включает общие поля, такие как `session_id`, уникальный ID для сеанса, и `cwd`, рабочий каталог при срабатывании события, но каждый тип события добавляет разные данные. Когда Claude запускает команду Bash, hook `PreToolUse` получает эти поля на stdin:

* `hook_event_name`: событие, которое запустило hook
* `tool_name`: инструмент, который Claude собирается использовать
* `tool_input`: аргументы, которые Claude передал инструменту. Для Bash его поле `command` содержит команду оболочки.

Например, ввод hook для команды `npm test` выглядит так:

```json theme={null}
{
  "session_id": "abc123",
  "cwd": "/Users/sarah/myproject",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Ваш скрипт может анализировать этот JSON и действовать на основе любого из этих полей. Hooks `UserPromptSubmit` получают текст `prompt` вместо этого, hooks `SessionStart` получают `source` (`startup`, `resume`, `clear`, `compact` или `fork`), и так далее. См. [Common input fields](/docs/ru/hooks#common-input-fields) в справочнике для общих полей и раздел каждого события для схем, специфичных для события.

<h4 id="hook-output">
  Вывод hook
</h4>

Ваш скрипт сообщает Claude Code, что делать дальше, записывая в stdout или stderr и выходя с определённым кодом. Следующий hook `PreToolUse` блокирует команду:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr становится обратной связью для Claude
  exit 2 # exit 2 = заблокировать действие
fi

exit 0  # exit 0 = решения нет; применяется обычный поток разрешений
```

Код выхода определяет, что происходит дальше:

* **Exit 0**: ваш hook не возражает через код выхода.
  * Для hook `PreToolUse` это не одобряет вызов инструмента: нормальный [permission flow](/docs/ru/permissions) по-прежнему применяется.
  * Для hooks `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` и `PostModelSwitch`, Claude Code добавляет stdout, который он [обрабатывает как простой текст](/docs/ru/hooks#exit-code-0), в контекст Claude.
* **Exit 2**: Claude Code блокирует действие. Напишите причину в stderr. Где она попадает, зависит от события: некоторые события передают её Claude как обратную связь, чтобы он мог скорректировать, другие показывают её пользователю, а несколько, таких как `ConfigChange` и `Elicitation`, не выводят никакого сообщения. Некоторые события не могут быть заблокированы: для `SessionStart` и других, exit 2 показывает stderr пользователю и выполнение продолжается. См. [exit code 2 behavior per event](/docs/ru/hooks#exit-code-2-behavior-per-event) для полного списка.
* **Любой другой код выхода**: для большинства событий результат зависит от того, что ваш hook вывел в stdout:
  * Разобранный объект, который проходит проверку схемы: Claude Code игнорирует код выхода, только JSON определяет результат, и hook не сообщается как ошибка. Исключения для каждого события, такие как `WorktreeCreate`, отказывающий при любом ненулевом выходе, перечислены в разделе [Exit code output](/docs/ru/hooks#exit-code-output) справочника.
  * Разобранный объект, который не проходит проверку схемы, или stdout, который Claude Code [пытается разобрать как JSON](/docs/ru/hooks#exit-code-0), но это не является действительным JSON: неблокирующая ошибка; уведомление содержит сообщение проверки или разбора.
  * Stdout, который Claude Code [обрабатывает как простой текст](/docs/ru/hooks#exit-code-0), или пустой stdout: действие продолжается как неблокирующая ошибка. Стенограмма показывает уведомление `<hook name> hook error`, затем первую строку stderr с префиксом `Failed with non-blocking status code:`. Чтобы захватить полный stderr, включите [debug logging](/docs/ru/hooks#debug-hooks) с `claude --debug` или запустив `/debug` во время сеанса.

<h4 id="structured-json-output">
  Структурированный вывод JSON
</h4>

Коды выхода дают вам только возможность заблокировать или остаться молчаливым. Для большего контроля выйдите с 0 и выведите объект JSON в stdout вместо этого.

<Note>
  Используйте exit 2 для блокировки с сообщением stderr или exit 0 с JSON для структурированного управления. Выберите один подход для каждого hook. Для того, что происходит, когда вы их смешиваете, см. [Exit code output](/docs/ru/hooks#exit-code-output).
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

С `"deny"` Claude Code отменяет вызов инструмента и передаёт `permissionDecisionReason` обратно Claude.

На `PreToolUse` Claude Code обрабатывает каждое значение `permissionDecision` следующим образом:

* `"allow"`: пропустить интерактивный запрос разрешения. Правила отказа и запроса, включая управляемые списки отказов предприятия, по-прежнему применяются, как и запросы для инструментов MCP, отмеченных [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), и для инструментов соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) в сеансах, где эта настройка достигает Claude Code
* `"deny"`: отменить вызов инструмента и отправить причину Claude
* `"ask"`: показать запрос разрешения пользователю как обычно

Четвёртое значение, `"defer"`, доступно в [non-interactive mode](/docs/ru/headless) с флагом `-p`. Оно выходит из процесса с сохранённым вызовом инструмента, чтобы обёртка Agent SDK могла собрать ввод и возобновить. См. [Defer a tool call for later](/docs/ru/hooks#defer-a-tool-call-for-later) в справочнике.

Hook `PreModelSwitch` возвращает то же поле `permissionDecision`: `"allow"` позволяет переключению модели продолжиться, а `"deny"` отменяет его. `"ask"` заставляет вас подтвердить переключение при запуске `/model` в интерактивном сеансе; везде в другом месте Claude Code обрабатывает `"ask"` как отказ. См. [PreModelSwitch decision control](/docs/ru/hooks#premodelswitch-decision-control).

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

Matcher `"Edit|Write"` срабатывает только, когда Claude использует инструмент `Edit` или `Write`, а не когда он использует `Bash`, `Read` или любой другой инструмент. Запятая разделяет альтернативы так же, поэтому `"Edit, Write"` эквивалентна. См. [Matcher patterns](/docs/ru/hooks#matcher-patterns) для того, как простые имена и регулярные выражения оцениваются.

<Note>
  Claude может также создавать или изменять файлы, запуская команды оболочки. Если ваш hook должен видеть каждое изменение файла, например для сканирования соответствия или логирования аудита, добавьте hook [`Stop`](/docs/ru/hooks#stop), который сканирует рабочее дерево один раз за ход. Для покрытия за вызов вместо этого также соответствуйте `Bash|PowerShell` и пусть ваш скрипт перечисляет изменённые и неотслеживаемые файлы с помощью `git status --porcelain`. Раздел [PowerShell hook input](/docs/ru/hooks#powershell) объясняет, почему соответствие только `Bash` недостаточно. Чтобы запустить hook при изменении определённого файла на диске, независимо от того, что его записало, используйте hook [FileChanged](/docs/ru/hooks#filechanged).
</Note>

Каждый тип события соответствует определённому полю:

| Событие                                                                                                                                                         | Что фильтрует matcher                                                                                             | Примеры значений matcher                                                                                                                                                                                                                                                       |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | имя инструмента                                                                                                   | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                                  | как начался сеанс                                                                                                 | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                                         | какой флаг CLI запустил setup                                                                                     | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                                    | почему закончился сеанс                                                                                           | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                                  | тип уведомления                                                                                                   | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                                 | тип агента                                                                                                        | `general-purpose`, `Explore`, `Plan` или пользовательские имена агентов                                                                                                                                                                                                        |
| `PreCompact`, `PostCompact`                                                                                                                                     | что запустило компактирование                                                                                     | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                                             | каноническое имя модели, на которую переключается сеанс, как описано в [PreModelSwitch](/docs/ru/hooks#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                                  | тип агента                                                                                                        | те же значения, что и `SubagentStart`                                                                                                                                                                                                                                          |
| `ConfigChange`                                                                                                                                                  | источник конфигурации                                                                                             | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `DirectoryAdded`                                                                                                                                                | как был добавлен каталог                                                                                          | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `StopFailure`                                                                                                                                                   | тип ошибки                                                                                                        | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                                            | причина загрузки                                                                                                  | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `Elicitation`                                                                                                                                                   | имя MCP сервера                                                                                                   | ваши настроенные имена MCP серверов                                                                                                                                                                                                                                            |
| `ElicitationResult`                                                                                                                                             | имя MCP сервера                                                                                                   | те же значения, что и `Elicitation`                                                                                                                                                                                                                                            |
| `FileChanged`                                                                                                                                                   | буквальные имена файлов для наблюдения (см. [FileChanged](/docs/ru/hooks#filechanged))                                 | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `UserPromptExpansion`                                                                                                                                           | имя команды                                                                                                       | ваши имена skill или команд                                                                                                                                                                                                                                                    |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | поддержка matcher отсутствует                                                                                     | всегда срабатывает при каждом возникновении                                                                                                                                                                                                                                    |

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

Когда Claude Code не может определить, какие команды запускает ввод Bash, он запускает ваш hook независимо от шаблона. [Таблица соответствия Bash](/docs/ru/hooks#bash-if-matching) охватывает формы команд, которые Claude Code может и не может сузить по подкоманде. Поскольку фильтр работает по принципу лучшего усилия, используйте [permission system](/docs/ru/permissions) вместо hook для обеспечения жёсткого разрешения или отказа.

Поле `if` принимает те же шаблоны, что и правила разрешений: `"Bash(git *)"`, `"Edit(*.ts)"` и так далее. Для соответствия нескольким именам инструментов используйте отдельные обработчики каждый со своим значением `if`, или соответствуйте на уровне `matcher`, где поддерживается чередование трубой.

`if` работает только на событиях инструментов: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` и `PermissionDenied`. Добавление его к любому другому событию предотвращает запуск hook.

<h3 id="configure-hook-location">
  Настройка местоположения hook
</h3>

Где вы добавляете hook, определяет его область:

| Местоположение                                    | Область                                                                                                   | Общий доступ                                                |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------- |
| `~/.claude/settings.json`                         | Все ваши проекты                                                                                          | Нет, локально на вашей машине                               |
| `.claude/settings.json`                           | Один проект                                                                                               | Да, можно зафиксировать в репо                              |
| `.claude/settings.local.json`                     | Один проект                                                                                               | Нет, gitignored когда Claude Code сохраняет параметр в него |
| Managed policy settings                           | Организация                                                                                               | Да, контролируется администратором                          |
| [Plugin](/docs/ru/plugins/overview) `hooks/hooks.json` | Когда плагин включен                                                                                      | Да, упакован с плагином                                     |
| [Skill](/docs/ru/skills) frontmatter                   | Остаток сеанса после вызова skill. См. [Hooks in skills and agents](/docs/ru/hooks#hooks-in-skills-and-agents) | Да, определено в файле skill                                |
| [Subagent](/docs/ru/sub-agents) frontmatter            | Пока этот subagent запущен                                                                                | Да, определено в файле subagent                             |

Запустите [`/hooks`](/docs/ru/hooks#the-%2Fhooks-menu) в Claude Code для просмотра всех настроенных hooks, сгруппированных по событиям.

Чтобы отключить hooks, установите `"disableAllHooks": true` в вашем файле параметров. Claude Code читает значение, оставшееся после применения [settings precedence](/docs/ru/hooks#disable-or-remove-hooks), поэтому файл параметров проекта может переопределить ваш. Hooks, настроенные в управляемых параметрах, по-прежнему запускаются, если `disableAllHooks` также не установлен там. Для полного охвата каждого уровня см. [`disableAllHooks`](/docs/ru/settings-reference#disableallhooks).

Если вы редактируете файлы параметров напрямую во время работы Claude Code, наблюдатель файлов обычно автоматически подхватывает изменения hook.

<h2 id="prompt-based-hooks">
  Hooks на основе подсказок
</h2>

Для решений, требующих суждения, а не детерминированных правил, используйте hooks `type: "prompt"`. Вместо запуска команды оболочки Claude Code отправляет вашу подсказку и данные ввода hook модели Claude (Haiku по умолчанию) для принятия решения. Вы можете указать другую модель с полем `model`, если вам нужна большая возможность.

Единственная работа модели — вернуть решение в виде JSON:

* `"ok": true`: действие продолжается
* `"ok": false`: что происходит, зависит от события:
  * `Stop` и `SubagentStop`: значение `reason` передаётся обратно Claude, чтобы он продолжал работать, если только ответ также не устанавливает `"impossible": true` для обозначения условия, которое никогда не может быть выполнено, в этом случае Claude Code разрешает остановку и ход завершается
  * `PreToolUse`: вызов инструмента отклоняется; по умолчанию ход завершается и значение `reason` отказа появляется в чате как строка предупреждения. Установите `continueOnBlock: true` на hook, чтобы вместо этого вернуть значение `reason` Claude как ошибку инструмента, чтобы он мог скорректировать и продолжить. До версии 2.1.210 значение `reason` отказа возвращалось Claude как ошибка инструмента и ход продолжался
  * `PostToolUse`: по умолчанию ход завершается и значение `reason` появляется в чате как строка предупреждения. Установите `continueOnBlock: true`, чтобы передать значение `reason` обратно Claude и продолжить ход вместо этого
  * `PostToolBatch`, `UserPromptSubmit` и `UserPromptExpansion`: ход завершается и значение `reason` появляется в чате как строка предупреждения

Этот пример использует hook `Stop` для запроса модели, завершены ли все запрошенные задачи. Если модель возвращает `"ok": false`, потому что условие ещё не выполнено, Claude продолжает работать и использует значение `reason` как свою следующую инструкцию:

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

Hooks агентов используют формат ответа `"ok"` / `"reason"` с более длинным временем ожидания по умолчанию 60 секунд и до 50 оборотов использования инструмента. Они не поддерживают поле `impossible` из prompt hooks. При `ok: false` Claude Code обрабатывает hook агента так же, как он обрабатывает prompt hook с `continueOnBlock: true` на том же событии, поэтому на `PreToolUse` и `PostToolUse` ход продолжается; hooks агентов не имеют поля `continueOnBlock`. См. [конфигурацию hooks агентов](/docs/ru/hooks#agent-hook-configuration) для полей, включая заполнитель `$ARGUMENTS`, который Claude Code заменяет входными данными JSON hook.

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
  * `command`, `http`, `mcp_tool`: 10 минут. Claude Code снижает это значение по умолчанию до 30 секунд для hooks `UserPromptSubmit`, `PreModelSwitch` и `PostModelSwitch`, и до 10 секунд для `MessageDisplay`.
  * `prompt`: 30 секунд.
  * `agent`: 60 секунд.
  * Hooks [`SessionEnd`](/docs/ru/hooks#sessionend) любого типа совместно используют бюджет в 1,5 секунды. Если ваши параметры устанавливают более длительный `timeout` для каждого hook, Claude Code увеличивает бюджет в соответствии с этим, до 60 секунд.
* Hooks `PostToolUse` не могут отменить действия, так как инструмент уже выполнен.
* Hooks `PermissionRequest` срабатывают, когда Claude Code собирается попросить у вас разрешение.
  * В [неинтерактивном режиме](/docs/ru/headless) с флагом `-p` этот запрос существует только когда обратный вызов [`canUseTool`](/docs/ru/agent-sdk/permissions) Agent SDK его предоставляет. В обычных запусках `-p` или с `--permission-prompt-tool` используйте hooks `PreToolUse` для автоматизированных решений разрешений вместо этого.
  * Фоновые подагенты не могут показать запрос в неинтерактивном режиме. Claude Code всё ещё запускает hooks для их вызовов инструментов, и если ни один hook не возвращает решение, он отклоняет вызов. В интерактивном сеансе запросы фоновых подагентов появляются в вашем основном сеансе и hooks срабатывают как обычно.
* Hooks `Stop` срабатывают всякий раз, когда Claude завершает ответ, а не только при завершении задачи. Они не срабатывают при прерывании пользователем. Ошибки API срабатывают [StopFailure](/docs/ru/hooks#stopfailure) вместо этого.
* Когда несколько hooks `PreToolUse` возвращают [`updatedInput`](/docs/ru/hooks#pretooluse) для переписания аргументов инструмента, последний завершённый побеждает. Поскольку hooks запускаются параллельно, порядок недетерминирован. Избегайте наличия более одного hook, изменяющего ввод одного и того же инструмента.

<h3 id="hooks-and-permission-modes">
  Hooks и режимы разрешений
</h3>

Hooks `PreToolUse` срабатывают перед любой проверкой режима разрешений в каждом [режиме разрешений](/docs/ru/permission-modes), включая `dontAsk`. Hook, возвращающий `permissionDecision: "deny"`, блокирует инструмент даже в режиме `bypassPermissions` или с `--dangerously-skip-permissions`. Это позволяет вам применять политику, которую пользователи не могут обойти, изменив свой режим разрешений.

Обратное неверно: hook, возвращающий `"allow"`, не обходит правила отказа из параметров, и он не может подавить запрос для инструментов MCP, отмеченных [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), или для инструментов соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) в сеансах, где эта настройка достигает Claude Code. Hooks могут ужесточить ограничения, но не ослабить их сверх того, что разрешают правила разрешений.

<h3 id="hook-not-firing">
  Hook не срабатывает
</h3>

Hook настроен, но никогда не выполняется.

* Запустите `/hooks` и подтвердите, что hook появляется под правильным событием
* Проверьте, что шаблон matcher точно соответствует имени инструмента. Matchers чувствительны к регистру
* Убедитесь, что вы запускаете правильный тип события: `PreToolUse` срабатывает перед выполнением инструмента, `PostToolUse` срабатывает после. Hook `PermissionRequest` срабатывает, когда Claude Code собирается попросить у вас разрешение; см. [ограничения](#limitations) для неинтерактивных случаев

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
* Если уведомление показывает сообщение о валидации JSON, stdout вашего hook был проанализирован как JSON, но не прошёл валидацию схемы. Если оно показывает сообщение об анализе JSON, stdout выглядел как объект JSON, но не был действительным JSON. Оба случая происходят даже при выходе 0.

  Чтобы исправить ошибку анализа, создайте полезную нагрузку с помощью кодировщика JSON, такого как `jq`, вместо конкатенации строк, чтобы кавычки и обратные слэши внутри значений были экранированы. Раздел [Exit code output](/docs/ru/hooks#exit-code-output) справочника охватывает комбинации кодов выхода и JSON
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

<h3 id="hook-json-has-no-effect">
  Hook JSON не имеет эффекта
</h3>

Ваш hook выводит действительный JSON, но решение не вступает в силу и в стенограмме не появляется никакой ошибки. Проверьте, какая причина применима:

* **Дополнительный вывод перед JSON**: что-то ещё записывает в stdout первым, обычно безусловный `echo` в вашем профиле оболочки, поэтому вывод больше не начинается с `{` и Claude Code не анализирует его как JSON. Причина и исправление следуют после этого списка.
* **Поле на неправильном уровне**: сравните размещение каждого поля с форматом [JSON output](/docs/ru/hooks#json-output). Например, `permissionDecision` должен находиться внутри `hookSpecificOutput`, а не на верхнем уровне.

Когда Claude Code запускает hook-команду в форме shell (без `args`), он порождает `sh -c` на macOS и Linux, Git Bash на Windows, или PowerShell, когда Git Bash не установлен по умолчанию. Эта оболочка неинтерактивна, но Git Bash и некоторые конфигурации, такие как `BASH_ENV`, указывающий на `~/.bashrc`, всё ещё источают ваш профиль. Если этот профиль содержит безусловные операторы `echo`, вывод добавляется к JSON вашего hook:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Объединённый вывод больше не начинается с `{`, поэтому Claude Code рассматривает весь stdout как простой текст и игнорирует JSON. При выходе 0 ничего не сообщается в стенограмме; попытка анализа записывается только в [журнал отладки](/docs/ru/hooks#debug-hooks). Чтобы исправить это, оберните операторы echo в вашем профиле оболочки, чтобы они запускались только в интерактивных оболочках:

```bash theme={null}
# В ~/.zshrc или ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

Переменная `$-` содержит флаги оболочки, и `i` означает интерактивный. Hooks запускаются в неинтерактивных оболочках, поэтому echo пропускается.

Когда ваш hook возвращает `permissionDecision` или `additionalContext` на верхнем уровне вместо внутри `hookSpecificOutput`, JSON всё ещё анализируется, и Claude Code игнорирует неправильно размещённые поля без сообщения об ошибке. Чтобы увидеть, какие поля он игнорировал, запустите Claude Code с `claude --debug` и найдите в [журнале отладки](/docs/ru/hooks#debug-hooks) `Hook JSON output had unrecognized keys`.

<h3 id="debug-techniques">
  Методы отладки
</h3>

Нажмите `Ctrl+O` для открытия представления стенограммы, чтобы проверить результат запуска hook:

* **Успешный запуск**: вы ничего не видите, если только JSON hook не выводит что-то, например `systemMessage` или обратную связь Stop hook.
  * Чтобы подтвердить, что hook запустился, проверьте его эффект, например переформатированный файл, или включите логирование отладки, как описано ниже, и снова запустите hook
* **Блокирующая ошибка**: в большинстве событий вы видите обратную связь hook. Когда JSON hook принял блокирующее решение, обратная связь — это причина из этого решения; в противном случае это stderr hook. На нескольких событиях, таких как `ConfigChange` и `Elicitation`, блокировка не выводит сообщение.
* **Неблокирующая ошибка**: действие продолжилось, и вы видите уведомление `<hook name> hook error` с кратким объяснением, например первая строка stderr с префиксом `Failed with non-blocking status code:`, или сообщение о валидации JSON или анализе.

Какие комбинации кодов выхода и JSON производят каждый результат, включая исключения для каждого события, определены в разделе [Exit code output](/docs/ru/hooks#exit-code-output) справочника.

Для полных деталей выполнения, включая какие hooks совпали, их коды выхода, stdout и stderr, прочитайте журнал отладки. Запустите Claude Code с `claude --debug-file /tmp/claude.log` для записи в известный путь, затем `tail -f /tmp/claude.log` в другом терминале. Если вы запустили без этого флага, запустите `/debug` во время сеанса для включения логирования и поиска пути журнала.

<h2 id="learn-more">
  Узнайте больше
</h2>

* [Справочник Hooks](/docs/ru/hooks): полные схемы событий, формат вывода JSON, асинхронные hooks и MCP tool hooks
* [Соображения безопасности](/docs/ru/hooks#security-considerations): просмотрите перед развёртыванием hooks в общих или производственных средах
* [Пример валидатора команд Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): полная справочная реализация
