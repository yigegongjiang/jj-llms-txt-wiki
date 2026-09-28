> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник по hooks

> Справочник по событиям hook Claude Code, схеме конфигурации, форматам JSON входа/выхода, кодам выхода, асинхронным hooks, HTTP hooks, prompt hooks и MCP tool hooks.

<Tip>
  Для краткого руководства с примерами см. [Автоматизация рабочих процессов с помощью hooks](/docs/ru/hooks-guide).
</Tip>

Hooks — это определяемые пользователем команды оболочки, конечные точки HTTP, вызовы инструментов MCP, подсказки LLM или подагенты, которые выполняются автоматически в определённых точках жизненного цикла Claude Code. Claude Code запускает одни и те же события hook везде, где он работает: сеансы в терминале, расширения IDE, [приложение Desktop](/docs/ru/desktop-quickstart) и [облачные сеансы](/docs/ru/claude-code-on-the-web). Используйте этот справочник для поиска схем событий, параметров конфигурации, форматов JSON входа/выхода и расширенных функций, таких как асинхронные hooks, HTTP hooks и MCP tool hooks.

<h2 id="hook-lifecycle">
  Жизненный цикл hook
</h2>

Claude Code запускает hooks в определённых точках во время сеанса. Когда событие срабатывает и совпадает с фильтром, Claude Code передаёт JSON-контекст события вашему обработчику hook. Для command hooks входные данные поступают на stdin. Для HTTP hooks они поступают как тело POST-запроса. Ваш обработчик может затем проверить входные данные, выполнить действие и опционально вернуть решение.

События срабатывают в трёх ритмах:

* один раз за сеанс: `SessionStart` и `SessionEnd`
* один раз за ход: `UserPromptSubmit`, `Stop` и `StopFailure`
* при каждом вызове инструмента внутри агентного цикла: `PreToolUse` и `PostToolUse`, за исключением вызовов [`EndConversation`](/docs/ru/tools-reference#endconversation-tool-behavior), которые пропускают оба

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" className="dark:hidden" alt="Диаграмма жизненного цикла hook, показывающая опциональный Setup, переходящий в SessionStart, затем цикл за ход, содержащий UserPromptSubmit, UserPromptExpansion для slash commands, вложенный агентный цикл (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted) и Stop или StopFailure, за которым следуют TeammateIdle, PreCompact, PostCompact и SessionEnd, с Elicitation и ElicitationResult вложенными внутри выполнения MCP tool, PermissionDenied как боковая ветвь от PermissionRequest для автоматических отказов, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged и DirectoryAdded как отдельные асинхронные события, PreModelSwitch как отдельное последовательное событие, которое запускается перед запрошенным переключением модели, PostModelSwitch как отдельное асинхронное событие, которое запускается после изменения модели сеанса, и MessageDisplay как событие только для отображения, которое запускается во время потоковой передачи текста сообщения помощника" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />

    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle-dark.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=c9b3d88487335f58cce0b52e2f9e7531" className="hidden dark:block" alt="Диаграмма жизненного цикла hook, показывающая опциональный Setup, переходящий в SessionStart, затем цикл за ход, содержащий UserPromptSubmit, UserPromptExpansion для slash commands, вложенный агентный цикл (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted) и Stop или StopFailure, за которым следуют TeammateIdle, PreCompact, PostCompact и SessionEnd, с Elicitation и ElicitationResult вложенными внутри выполнения MCP tool, PermissionDenied как боковая ветвь от PermissionRequest для автоматических отказов, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged и DirectoryAdded как отдельные асинхронные события, PreModelSwitch как отдельное последовательное событие, которое запускается перед запрошенным переключением модели, PostModelSwitch как отдельное асинхронное событие, которое запускается после изменения модели сеанса, и MessageDisplay как событие только для отображения, которое запускается во время потоковой передачи текста сообщения помощника" width="520" height="1336" data-path="images/hooks-lifecycle-dark.svg" />
  </Frame>
</div>

Таблица ниже суммирует, когда срабатывает каждое событие. Раздел [Hook events](#hook-events) документирует полную схему входа и параметры управления решением для каждого события.

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

<h3 id="how-a-hook-resolves">
  Как разрешается hook
</h3>

Чтобы увидеть, как событие, фильтр и обработчик работают вместе, рассмотрим этот hook `PreToolUse`, который блокирует деструктивные команды оболочки.

<Tabs>
  <Tab title="macOS/Linux">
    Фильтр `matcher` сужает область до вызовов инструмента Bash, а условие `if` сужает её дальше до команд Bash, совпадающих с `rm *`, поэтому `block-rm.sh` запускается только когда оба фильтра совпадают:

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

    Скрипт читает JSON входные данные из stdin, извлекает команду и возвращает `permissionDecision` со значением `"deny"`, если она содержит `rm -rf`. Сохраните его в `.claude/hooks/block-rm.sh` в вашем проекте и сделайте его исполняемым с помощью `chmod +x .claude/hooks/block-rm.sh`, чтобы Claude Code мог его запустить:

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

    Этот скрипт, как и другие примеры Bash на этой странице, которые анализируют JSON входные данные, использует `jq`, поэтому установите `jq` и убедитесь, что он находится в вашем `PATH` перед попыткой их использования.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Фильтр `Bash|PowerShell` охватывает [инструмент PowerShell](#powershell) а также Bash. Одно правило `if` совпадает только с вызовами одного инструмента, поэтому каждый инструмент получает свой обработчик: первый сужает область до команд Bash, совпадающих с `rm *`, второй — до команд PowerShell, совпадающих с `Remove-Item *`. Оба запускают один и тот же скрипт через `powershell.exe`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash|PowerShell",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              },
              {
                "type": "command",
                "if": "PowerShell(Remove-Item *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Флаг `-NoProfile` пропускает загрузку вашего профиля PowerShell, чтобы hook запустился быстро, а `-ExecutionPolicy Bypass` позволяет PowerShell запустить локальный файл скрипта.

    Скрипт читает JSON входные данные из stdin, извлекает команду и возвращает `permissionDecision` со значением `"deny"`, если она содержит `rm -rf` или `Remove-Item` с последующим `-Recurse`. Сохраните его в `.claude/hooks/block-rm.ps1` в вашем проекте:

    ```powershell theme={null}
    # .claude/hooks/block-rm.ps1
    $callInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $command = $callInput.tool_input.command

    if ($command -match 'rm -rf|Remove-Item.*-Recurse') {
      @{
        hookSpecificOutput = @{
          hookEventName = "PreToolUse"
          permissionDecision = "deny"
          permissionDecisionReason = "Destructive command blocked by hook"
        }
      } | ConvertTo-Json
    } else {
      exit 0  # no decision; normal permission flow applies
    }
    ```
  </Tab>
</Tabs>

Теперь предположим, что Claude Code решает запустить `Bash "rm -rf /tmp/build"` с конфигурацией macOS/Linux. Вот что происходит:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" className="dark:hidden" alt="Диаграмма разрешения hook: срабатывает событие PreToolUse, фильтр проверяет совпадение Bash, затем условие if проверяет совпадение Bash(rm *). Если оба совпадают, команда hook запускается и возвращает permissionDecision deny, поэтому вызов инструмента блокируется и Claude Code продолжает работу. Если одна из проверок не совпадает, hook пропускается и вызов инструмента может продолжить работу." width="930" height="270" data-path="images/hook-resolution.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/hook-resolution-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e80af91f8507cee6bd51ac3c2dd92f63" className="hidden dark:block" alt="Диаграмма разрешения hook: срабатывает событие PreToolUse, фильтр проверяет совпадение Bash, затем условие if проверяет совпадение Bash(rm *). Если оба совпадают, команда hook запускается и возвращает permissionDecision deny, поэтому вызов инструмента блокируется и Claude Code продолжает работу. Если одна из проверок не совпадает, hook пропускается и вызов инструмента может продолжить работу." width="930" height="270" data-path="images/hook-resolution-dark.svg" />
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

| Расположение                                      | Область действия                                                                                 | Общий доступ                                                      |
| :------------------------------------------------ | :----------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Все ваши проекты                                                                                 | Нет, локально на вашей машине                                     |
| `.claude/settings.json`                           | Один проект                                                                                      | Да, можно зафиксировать в репозитории                             |
| `.claude/settings.local.json`                     | Один проект                                                                                      | Нет, игнорируется git когда Claude Code сохраняет параметр в него |
| Управляемые параметры политики                    | Организация                                                                                      | Да, контролируется администратором                                |
| [Plugin](/docs/ru/plugins/overview) `hooks/hooks.json` | Когда плагин включен                                                                             | Да, поставляется с плагином                                       |
| [Skill](/docs/ru/skills) frontmatter                   | Остаток сеанса после вызова skill. См. [Hooks in skills and agents](#hooks-in-skills-and-agents) | Да, определено в файле skill                                      |
| [Subagent](/docs/ru/sub-agents) frontmatter            | Пока этот subagent работает                                                                      | Да, определено в файле subagent                                   |

Облачные сеансы на [Claude Code в веб-версии](/docs/ru/claude-code-on-the-web) не читают ваш локальный `~/.claude/settings.json`. В [самостоятельно размещённой среде](/docs/ru/self-hosted-environments-configuration#permissions-and-tool-approval), Claude Code также запускает hooks, которые оператор инициализировал из `~/.claude/` хоста runner, и запускает hooks в файле управляемых параметров образа runner, когда этот файл находится среди [управляемых источников, которые применяет Claude Code](/docs/ru/managed-settings#how-claude-code-combines-managed-sources), что по умолчанию означает только когда ни управляемые на сервере параметры, ни доставленная MDM политика Claude Code не предоставляют управляемый уровень. См. [что переносится из вашей установки](/docs/ru/cloud-environments#what-carries-over-from-your-setup) для того, какие файлы настроек и плагины, и таким образом какие hooks, достигают облачного сеанса.

Для получения подробной информации о разрешении файлов настроек см. [settings](/docs/ru/settings).

Hooks из файлов настроек, управляемых параметров политики и плагинов также запускаются внутри [subagents](/docs/ru/sub-agents). Когда subagent вызывает инструмент, события инструмента, такие как `PreToolUse` и `PostToolUse`, запускают те же настроенные hooks, что и в основном разговоре, и входные данные содержат поля `agent_id` и `agent_type` [общих входных полей](#common-input-fields), которые идентифицируют subagent.

Администраторы предприятия могут использовать `allowManagedHooksOnly` для ограничения того, какие hooks запускаются:

* Ваши пользовательские, проектные, локальные и плагинные hooks блокируются. Hooks из плагинов, принудительно включённых в управляемых параметрах `enabledPlugins`, исключены
* Claude Code также сужает ваши параметры [`statusLine`](/docs/ru/statusline), [`fileSuggestion`](/docs/ru/settings-reference#filesuggestion) и [`subagentStatusLine`](/docs/ru/statusline#subagent-status-lines) до управляемых параметров
* Claude Code также отключает плагины с источником [`command`](/docs/ru/plugins/marketplace-reference#command-plugin-source), включая плагины, принудительно включённые в управляемых параметрах `enabledPlugins`, если только [`disableCommandPluginSources`](/docs/ru/settings-reference#disablecommandpluginsources) явно не установлен на `false`. Источники `command` требуют Claude Code v2.1.229 или позже
* Claude Code также блокирует команды marketplace [`headersHelper`](/docs/ru/plugins/host-marketplace#authenticate-archive-downloads) если только [`disableCommandPluginSources`](/docs/ru/settings-reference#disablecommandpluginsources) явно не установлен на `false`, за исключением marketplace, которые сами управляемые параметры объявляют

См. [что запускается под `allowManagedHooksOnly`](/docs/ru/settings-reference#what-runs-under-allowmanagedhooksonly).

Hook записи объединяются на уровнях параметров, а не заменяют друг друга: пользовательские, проектные и локальные параметры добавляют свои собственные hooks без удаления управляемых, и параметр [`disableAllHooks`](#disable-or-remove-hooks) не может отключить управляемые hooks извне управляемых параметров.

[HTTP hook allowlists](/docs/ru/settings-reference#hook-and-skill-settings) применяются к hooks из каждого источника, включая управляемые параметры политики:

* `allowedHttpHookUrls`: когда определено на любом уровне параметров, Claude Code запускает обработчик HTTP hook только если его URL совпадает с объединённым allowlist
* `httpHookAllowedEnvVars`: когда определено, Claude Code интерполирует только переменные окружения из этого списка в заголовки hook

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

Дефисы в наборе точного совпадения требуют Claude Code v2.1.195 или позже. На более ранних версиях дефисное имя, такое как `code-reviewer`, оценивается как регулярное выражение без привязки, поэтому оно также срабатывает для `senior-code-reviewer`; закрепите его как `^code-reviewer$` на этих версиях, чтобы совпадать только с этим именем.

`FileChanged` и `StopFailure` используют более узкий набор точного совпадения только букв, цифр, `_` и `|`. Дефис, пробел или запятая в фильтре для этих двух событий держит его на пути регулярного выражения, и только `|` разделяет альтернативы. Каждое другое событие с поддержкой фильтра в таблице ниже принимает `|` или `,`.

Событие `FileChanged` не следует этим правилам при построении своего списка наблюдения. См. [FileChanged](#filechanged).

Каждый тип события совпадает с другим полем:

| Событие                                                                                                                                           | На что фильтр влияет                                                                                     | Примеры значений фильтра                                                                                                                                                                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | имя инструмента                                                                                          | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                    | как сеанс начался                                                                                        | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                           | какой флаг CLI запустил setup                                                                            | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                      | почему сеанс закончился                                                                                  | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                    | тип уведомления                                                                                          | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                   | тип агента                                                                                               | `general-purpose`, `Explore`, `Plan`, пользовательские имена агентов или имена с областью плагина, такие как `^my-plugin:reviewer$`                                                                                                                                            |
| `PreCompact`, `PostCompact`                                                                                                                       | что вызвало компактирование                                                                              | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                               | каноническое имя модели, на которую переключается сеанс, как описано в [PreModelSwitch](#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                    | тип агента                                                                                               | те же значения, что и `SubagentStart`                                                                                                                                                                                                                                          |
| `ConfigChange`                                                                                                                                    | источник конфигурации                                                                                    | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `CwdChanged`                                                                                                                                      | поддержка фильтра отсутствует                                                                            | всегда срабатывает при каждом возникновении                                                                                                                                                                                                                                    |
| `DirectoryAdded`                                                                                                                                  | как был добавлен каталог                                                                                 | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `FileChanged`                                                                                                                                     | буквальные имена файлов для наблюдения (см. [FileChanged](#filechanged))                                 | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `StopFailure`                                                                                                                                     | тип ошибки                                                                                               | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                              | причина загрузки                                                                                         | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `UserPromptExpansion`                                                                                                                             | имя команды                                                                                              | ваши имена skill или команд                                                                                                                                                                                                                                                    |
| `Elicitation`                                                                                                                                     | имя MCP сервера                                                                                          | ваши настроенные имена MCP серверов                                                                                                                                                                                                                                            |
| `ElicitationResult`                                                                                                                               | имя MCP сервера                                                                                          | те же значения, что и `Elicitation`                                                                                                                                                                                                                                            |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | поддержка фильтра отсутствует                                                                            | всегда срабатывает при каждом вхождении                                                                                                                                                                                                                                        |

Совпадение `StopFailure` на `cloud_credential_error` требует Claude Code v2.1.267 или позже, первой версии, которая сообщает об ошибках загрузки учётных данных под этим значением вместо `server_error` или `unknown`.

Для большинства событий Claude Code оценивает фильтр против поля из [JSON входа](#hook-input-and-output), который он отправляет вашему hook на stdin. Для событий инструмента это поле — `tool_name`. Для `PreModelSwitch` и `PostModelSwitch`, Claude Code оценивает фильтр против канонического имени, которое он выводит из `to_model`, как описано в [PreModelSwitch](#premodelswitch). Каждый раздел [hook event](#hook-events) перечисляет полный набор значений фильтра и схему входа для этого события.

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

Если вы добавите поле `matcher` к событию без поддержки фильтра, оно будет молча проигнорировано.

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
* **[Prompt hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): отправляют подсказку модели Claude для однооборотной оценки. Модель возвращает решение как JSON. См. [Prompt-based hooks](#prompt-based-hooks).
* **[Agent hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): порождают subagent, который может использовать инструменты, такие как Read, Grep и Glob, для проверки условий перед возвратом решения. Agent hooks являются экспериментальными и могут измениться. См. [Agent-based hooks](#agent-based-hooks).

Все совпадающие hooks запускаются параллельно. Если вы определите один и тот же обработчик в более чем одном файле настроек, он запускается один раз. Копия плагина или skill одного и того же обработчика остаётся отдельной.

Обработчики запускаются в текущем каталоге с окружением Claude Code. Если текущий каталог больше не существует, например worktree или временный каталог, который другая оболочка удалила в середине сеанса, Claude Code запускает command hooks из первого из них, который всё ещё существует: каталог, в котором сеанс начался, корень проекта, ваш домашний каталог или системный временный каталог. Claude Code записывает предупреждение, называющее резервный каталог, в [debug log](#debug-hooks).

Переменная окружения `$CLAUDE_CODE_REMOTE` устанавливается на `"true"` в удалённых веб-окружениях и не устанавливается в локальном CLI. Claude Code v2.1.199 и позже устанавливает [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/ru/env-vars) на ID сеанса [Remote Control](/docs/ru/remote-control) пока локальный сеанс имеет активное соединение Remote Control.

<h4 id="common-fields">
  Common fields
</h4>

Эти поля применяются ко всем типам hooks:

| Поле            | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :-------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`          | да          | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` или `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `if`            | нет         | Синтаксис правила разрешения для фильтрации срабатывания этого hook, такой как `"Bash(git *)"` или `"Edit(*.ts)"`. Hook запускается только если вызов инструмента совпадает с шаблоном. См. таблицу [Bash matching table](#bash-if-matching) ниже для того, как Bash шаблоны оцениваются против подкоманд, `$()` и обратных кавычек. Оценивается только на событиях инструмента: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` и `PermissionDenied`. На других событиях hook с установленным `if` никогда не запускается. Использует тот же синтаксис, что и [правила разрешения](/docs/ru/permissions)                                                                                   |
| `timeout`       | нет         | Секунды перед отменой. Claude Code не применяет его на command hook, который вы запускаете с [`async: true`](#run-hooks-in-the-background). Значения по умолчанию: 600 для `command`, `http` и `mcp_tool`; 30 для `prompt`; 60 для `agent`. Claude Code снижает значение по умолчанию для `command`, `http` и `mcp_tool` до 30 на [`UserPromptSubmit`](#userpromptsubmit), [`PreModelSwitch`](#premodelswitch) и [`PostModelSwitch`](#postmodelswitch), и до 10 на [`MessageDisplay`](#messagedisplay). Hooks [`SessionEnd`](#sessionend) делят бюджет 1.5 секунды; если ваши параметры устанавливают более длительный `timeout` для каждого hook, Claude Code повышает бюджет, чтобы совпадать, до 60 секунд |
| `statusMessage` | нет         | Пользовательское сообщение спиннера, отображаемое во время выполнения hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `once`          | нет         | Если `true`, Claude Code удаляет hook после его первого успешного запуска. Запуск, который не удаётся, блокирует с кодом выхода 2 или истекает по времени, оставляет hook на месте, поэтому он запускается снова при следующем совпадающем событии. Только для hooks, объявленных в [skill frontmatter](#hooks-in-skills-and-agents); игнорируется в файлах настроек и agent frontmatter                                                                                                                                                                                                                                                                                                                      |

Поле `if` содержит ровно одно правило разрешения. Нет синтаксиса `&&`, `||` или списка для объединения правил; чтобы применить несколько условий, определите отдельный обработчик hook для каждого.

В условии `if` для инструмента файла, шаблон каталога с одним сегментом, такой как `"Edit(src/**)"`, совпадает только с каталогом `src` в рабочем каталоге и файлами под ним. Чтобы совпадать с каталогом с именем `src` на любой глубине, напишите `"Edit(**/src/**)"`. До v2.1.214, `"Edit(src/**)"` совпадал с каталогом с именем `src` на любой глубине под рабочим каталогом.

<span id="bash-if-matching" />Для Bash шаблонов, запускается ли ваша команда hook зависит от формы шаблона и команды Bash, которую вызывает Claude. Ведущие присваивания `VAR=value` удаляются перед совпадением.

| `if` шаблон        | Bash команда                | Hook запускается? | Почему                                                                                                                              |
| :----------------- | :-------------------------- | :---------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`          | да                | ведущие присваивания удаляются; `git push` совпадает                                                                                |
| `Bash(git *)`      | `npm test && git push`      | да                | каждая подкоманда проверяется; `git push` совпадает                                                                                 |
| `Bash(rm *)`       | `echo $(rm -rf /)`          | да                | команды внутри `$()` и обратных кавычек проверяются; `rm -rf /` совпадает                                                           |
| `Bash(rm *)`       | `echo $(date)`              | нет               | ни одна подкоманда не совпадает с `rm *`                                                                                            |
| `Bash(cat *)`      | `echo before $(date) after` | нет               | подстановка может находиться в любой позиции аргумента, поэтому проверяются полная команда и `date`; ни одна не совпадает с `cat *` |
| `Bash(git *)`      | `$TOOL git push`            | да                | Claude Code не может определить, на что расширяется имя команды, поэтому он запускает hook                                          |
| `Bash(git push *)` | `echo $(date)`              | да                | шаблоны, которые указывают больше чем имя команды, запускают hook в любом случае на `$()`, обратных кавычках или `$VAR`             |

Когда Claude Code не может определить, какие команды запускает входные данные Bash, он запускает ваш hook независимо от шаблона. Поскольку фильтр `if` является лучшим усилием, используйте [систему разрешений](/docs/ru/permissions) вместо hook для обеспечения жёсткого разрешения или отказа.

<h4 id="command-hook-fields">
  Command hook fields
</h4>

В дополнение к [общим полям](#common-fields), command hooks принимают эти поля:

| Поле          | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                                     |
| :------------ | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | да          | Команда оболочки для выполнения. С `args`, исполняемый файл для прямого запуска. См. [Exec form and shell form](#exec-form-and-shell-form)                                                                                                                                                                                                                                   |
| `args`        | нет         | Список аргументов. Когда присутствует, `command` разрешается как исполняемый файл и запускается напрямую с `args` как вектор аргументов, без участия оболочки. См. [Exec form and shell form](#exec-form-and-shell-form)                                                                                                                                                     |
| `async`       | нет         | Если `true`, запускается в фоне без блокировки. См. [Run hooks in the background](#run-hooks-in-the-background)                                                                                                                                                                                                                                                              |
| `asyncRewake` | нет         | Если `true`, запускается в фоне и пробуждает Claude при коде выхода 2. Hook stderr или stdout, если stderr пусто, показывается Claude как системное напоминание, чтобы он мог реагировать на долгоживущий фоновый сбой                                                                                                                                                       |
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

Plugin hooks дополнительно подставляют значения [`${user_config.*}`](/docs/ru/plugins/manifest-reference#user-configuration), только в exec form: значение подставляется в `command` и в каждый элемент `args` как простая строка, поэтому оболочка не переанализирует его.

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

Обработка ошибок отличается от command hooks; см. [HTTP response handling](#http-response-handling).

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

Claude Code читает текстовое содержимое инструмента так же, как читает command-hook stdout, следуя [правилу разбора под кодом выхода 0](#exit-code-0). Если названный сервер не подключён или инструмент возвращает `isError: true`, hook производит неблокирующую ошибку и выполнение продолжается.

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

MCP tool hook может запускаться только после того, как Claude Code сделал MCP серверы сеанса доступными для hooks. `SessionStart` и `Setup` могут срабатывать до этого момента:

* **При запуске**: `SessionStart` срабатывает перед доступностью серверов, включая когда вы запускаете с `--continue` или `--resume`. Claude Code пропускает `mcp_tool` hooks события без вызова их инструментов, и [debug log](#debug-hooks) записывает `mcp_tool hooks are not available for the 'SessionStart' hook event (no MCP client context)`.
* **Позже в работающем сеансе**: после `/clear` или компактирования, `SessionStart` срабатывает снова с серверами уже доступными, и его `mcp_tool` hooks запускаются.
* **На `Setup`**: `Setup` всегда срабатывает перед доступностью серверов, поэтому Claude Code пропускает его `mcp_tool` hooks каждый раз и записывает то же сообщение, называющее `Setup`.

Например, эта конфигурация вызывает инструмент `load_context` на MCP сервере `my_server` из hook `SessionStart` без фильтра, поэтому она применяется к каждому источнику `SessionStart`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "load_context"
          }
        ]
      }
    ]
  }
}
```

Когда вы запускаете `claude`, Claude Code пропускает этот hook, никогда не вызывает `load_context` и записывает сообщение `no MCP client context` в debug log. Запустите `/clear` в том же сеансе и hook запустится и вызовет `load_context`. Hook `type: "command"` на `SessionStart` запускается при запуске, поэтому используйте один для всего, что сеансу нужно с его первого хода.

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

* `${CLAUDE_PROJECT_DIR}`: корень проекта, где сеанс начался. Claude Code также устанавливает эту переменную в окружении [stdio MCP серверов](/docs/ru/mcp#option-3-add-a-local-stdio-server) и plugin LSP серверов.
* `${CLAUDE_PLUGIN_ROOT}`: каталог установки плагина, для скриптов, поставляемых с [плагином](/docs/ru/plugins/overview). См. [plugin environment variables](/docs/ru/plugins/manifest-reference#environment-variables) для того, как путь ведёт себя при обновлениях.
* `${CLAUDE_PLUGIN_DATA}`: [каталог постоянных данных](/docs/ru/plugins/components#path-variables-and-persistent-data) плагина, для зависимостей и состояния, которые должны пережить обновления плагина.

<Note>
  **Worktrees отличаются.** Если Claude входит в [worktree](/docs/ru/worktrees) во время сеанса, Claude Code держит `${CLAUDE_PROJECT_DIR}` там, где он был, и передаёт путь worktree вашим hooks другим способом:

  * **`${CLAUDE_PROJECT_DIR}` остаётся на месте**: он всё ещё указывает на корень проекта, где сеанс начался, поэтому команда, такая как `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh`, всё ещё запускает скрипт в основной checkout.
  * **`cwd` следует за Claude**: поле `cwd` в [входных JSON](#common-input-fields) hook — это корень worktree после того, как Claude входит в worktree, и новый каталог после того, как Claude запускает `cd`. Прочитайте его, когда hook нужно знать, в каком каталоге Claude работает.
</Note>

Предпочитайте [exec form](#exec-form-and-shell-form) для любого hook, который ссылается на path placeholder. В shell form оберните каждый заполнитель в двойные кавычки.

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

    См. [plugin components reference](/docs/ru/plugins/components#hooks) для получения подробной информации о создании plugin hooks.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks in skills and agents
</h3>

В дополнение к файлам настроек и плагинам, hooks могут быть определены непосредственно в [skills](/docs/ru/skills) и [subagents](/docs/ru/sub-agents) с использованием frontmatter, в том же формате конфигурации, что и hooks на основе настроек. Как долго Claude Code их регистрирует, зависит от компонента:

* **Subagent hooks**: Claude Code запускает их только пока этот subagent работает и удаляет их, когда он завершается. Claude Code преобразует hook `Stop` здесь в `SubagentStop`, событие, которое срабатывает при завершении subagent.
* **Skill hooks**: Claude Code регистрирует их, когда вы или Claude вызываете skill, и продолжает запускать их для остатка сеанса, на ходах после собственного хода skill. Чтобы Claude Code удалил hook после его первого успешного запуска вместо этого, установите [`once: true`](#common-fields) на нём.

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

Subagents используют тот же формат в своём YAML frontmatter.

Frontmatter hooks в project skill следуют тому же [правилу доверия рабочей области, что и hooks в файлах настроек](#workspace-trust). Claude Code регистрирует их, когда вы или Claude вызываете skill, включая в запуск `-p` в папке, которую вы не доверяли.

Frontmatter hooks в project subagent запускаются только после того, как вы примете [диалог доверия рабочей области](/docs/ru/permissions#project-allow-rules-and-workspace-trust) для папки, из которой пришёл файл агента. Сеанс `-p` не считается принятием. [Что запускается перед тем, как вы доверяете папке](/docs/ru/permissions#what-runs-before-you-trust-a-folder) сравнивает это с правилом файла настроек, и страница subagents перечисляет [какие области исключены](/docs/ru/sub-agents#hooks-in-subagent-frontmatter). До v2.1.218, эти hooks могли запускаться из папок, которым вы не доверяли.

<h3 id="the-/hooks-menu">
  Меню `/hooks`
</h3>

Введите `/hooks` в Claude Code, чтобы открыть браузер только для чтения ваших настроенных hooks. Меню показывает каждое hook событие с количеством настроенных hooks, позволяет вам углубиться в фильтры и показывает полные детали каждого hook обработчика. Используйте его для проверки конфигурации, проверки того, из какого файла настроек пришёл hook, или проверки команды, подсказки или URL hook.

Меню отображает все пять типов hook: `command`, `prompt`, `agent`, `http` и `mcp_tool`. Каждый hook помечен префиксом `[type]` и источником, указывающим, где он был определён:

* `User Settings`: из `~/.claude/settings.json`
* `Project Settings`: из `.claude/settings.json`
* `Local Settings`: из `.claude/settings.local.json`
* `Plugin Hooks`: из `hooks/hooks.json` плагина
* `Session Hooks`: зарегистрирован в памяти для текущего сеанса

Выбор hook открывает представление деталей, показывающее его событие, фильтр, тип, исходный файл и полную команду, подсказку или URL. Меню только для чтения: чтобы добавить, изменить или удалить hooks, отредактируйте JSON настроек напрямую или попросите Claude сделать изменение.

<h3 id="disable-or-remove-hooks">
  Отключение или удаление hooks
</h3>

Чтобы удалить hook, удалите его запись из JSON файла настроек.

Чтобы временно отключить все hooks без их удаления, установите `"disableAllHooks": true` в файле настроек. Claude Code читает значение, оставшееся после применения [приоритета параметров](/docs/ru/settings#settings-precedence), поэтому `"disableAllHooks": false` в `.claude/settings.json` проекта переопределяет `true` в ваших пользовательских параметрах. Чтобы отключить hooks для одного запуска, независимо от того, что говорят параметры проекта, передайте `--settings '{"disableAllHooks": true}'`, что имеет приоритет над параметрами проекта и локальными параметрами. Нет способа отключить отдельный hook, сохраняя его в конфигурации.

Параметр `disableAllHooks` соблюдает иерархию управляемых параметров. Если администратор настроил hooks через управляемые параметры политики, `disableAllHooks`, установленный в пользовательских, проектных или локальных параметрах, не может отключить эти управляемые hooks. Только `disableAllHooks`, установленный на уровне управляемых параметров, может отключить управляемые hooks. Для полного охвата каждого уровня см. [`disableAllHooks`](/docs/ru/settings-reference#disableallhooks).

Прямые редактирования hooks в файлах настроек обычно захватываются автоматически наблюдателем файлов.

<h2 id="hook-input-and-output">
  Входные и выходные данные Hook
</h2>

Hooks команд получают JSON-данные через stdin и передают результаты через коды выхода, stdout и stderr. HTTP hooks получают тот же JSON, что и тело POST-запроса, и передают результаты через тело HTTP-ответа. В этом разделе рассматриваются поля и поведение, общие для всех событий. Каждый раздел события в разделе [Hook events](#hook-events) включает его конкретную схему входных данных и параметры управления решением.

На macOS и Linux hooks команд запускаются в собственном сеансе без управляющего терминала. Процесс hook и любые дочерние процессы не могут открыть `/dev/tty` или отправлять последовательности escape непосредственно в интерфейс Claude Code. Windows не имеет `/dev/tty`.

Чтобы вывести сообщение пользователю на любой платформе, верните [`systemMessage`](#json-output) в JSON-выводе. Некоторые события игнорируют его или доставляют его в другое место, и в каждом [разделе события](#hook-events) указано, как это происходит. Чтобы вызвать уведомление рабочего стола, установить заголовок окна или издать звуковой сигнал, верните [`terminalSequence`](#emit-terminal-notifications) вместо этого.

<h3 id="common-input-fields">
  Общие входные поля
</h3>

Hook события получают эти поля в виде JSON в дополнение к полям, специфичным для события, задокументированным в каждом разделе [hook event](#hook-events). Для hooks команд этот JSON поступает через stdin. Для HTTP hooks он поступает как тело POST-запроса.

| Поле              | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `session_id`      | Текущий идентификатор сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `prompt_id`       | UUID, идентифицирующий пользовательский запрос, который в настоящее время обрабатывается. Совпадает с атрибутом [`prompt.id` на событиях OpenTelemetry](/docs/ru/monitoring-usage#event-correlation-attributes), поэтому вы можете коррелировать выход hook с телеметрией для одного запроса. Отсутствует до первого ввода пользователя. Требуется Claude Code v2.1.196 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `transcript_path` | Путь к файлу JSON разговора. Файл транскрипта записывается асинхронно и может отставать от разговора в памяти, поэтому он может еще не включать самые последние сообщения текущего хода, когда срабатывает hook. Hooks, которым нужен финальный текст ассистента текущего хода, должны использовать `last_assistant_message` на [Stop](#stop) и [SubagentStop](#subagentstop) вместо чтения транскрипта                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `cwd`             | Текущий рабочий каталог при вызове hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `scratchpad_dir`  | Путь к каталогу scratchpad сеанса, где Claude хранит временные рабочие файлы. Отсутствует, когда сеанс не имеет scratchpad или временный каталог недоступен. Требуется Claude Code v2.1.257 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `permission_mode` | Текущий [режим разрешений](/docs/ru/permissions#permission-modes): `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` или `"bypassPermissions"`. Режим, обозначенный как **Manual**, поступает как `"default"`, никогда не как `"manual"`, поэтому скрипты, которые совпадают с `"default"`, продолжают работать. Не все события получают это поле. Проверьте пример JSON в каждом разделе [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `effort`          | Объект с полем `level`, содержащим [уровень усилий](/docs/ru/model-config#adjust-effort-level), действующий при запуске hook: `"low"`, `"medium"`, `"high"`, `"xhigh"` или `"max"`. Если вы установите уровень, который активная модель не поддерживает, `level` сообщает уровень, который вместо этого запустил Claude Code; [Adjust effort level](/docs/ru/model-config#adjust-effort-level) говорит, как он выбирает этот уровень. Ultracode не является отдельным уровнем и сообщается как `"xhigh"`. Объект совпадает с полем `effort` [строки состояния](/docs/ru/statusline#available-data). Присутствует для событий, которые срабатывают в контексте использования инструмента, таких как `PreToolUse`, `PostToolUse`, `Stop` и `SubagentStop`, когда текущая модель поддерживает параметр усилий. Уровень также доступен для команд hook и инструмента Bash как переменная окружения `$CLAUDE_EFFORT`. |
| `hook_event_name` | Имя события, которое сработало                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

При запуске с `--agent` или внутри subagent включаются два дополнительных поля:

| Поле         | Описание                                                                                                                                                                                                                                                                                                                                                                                  |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Уникальный идентификатор для subagent. Присутствует только когда hook срабатывает внутри вызова subagent. Используйте это для различения вызовов hook subagent от вызовов основного потока.                                                                                                                                                                                               |
| `agent_type` | Имя агента (например, `"Explore"` или `"security-reviewer"`). Присутствует, когда сеанс использует `--agent` или hook срабатывает внутри subagent. Для subagents тип subagent имеет приоритет над значением `--agent` сеанса. См. [SubagentStart](#subagentstart) для значений, которые сообщают пользовательские и plugin subagents, и как написать matcher для имени с областью plugin. |

Только hooks [`SessionStart`](#sessionstart) могут получить поле `model`, и Claude Code не всегда его включает. Hooks [`PreModelSwitch`](#premodelswitch) и [`PostModelSwitch`](#postmodelswitch) получают `from_model` и `to_model` вместо этого, поэтому используйте hook PostModelSwitch для отслеживания модели по мере ее изменения во время сеанса.

Нет переменной окружения `$CLAUDE_MODEL`. Hook может читать `$ANTHROPIC_MODEL`, если вы установили ее в своей оболочке, но это значение не изменяется при переключении моделей с помощью `/model` во время сеанса.

Процесс hook наследует родительское окружение, за исключением переменных экспортера `OTEL_*`, которые Claude Code [удаляет из каждого подпроцесса, который он порождает](/docs/ru/monitoring-usage#administrator-configuration), и, когда установлена [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ru/env-vars#variables) в `1`, переменные, которые он удаляет.

Например, hook `PreToolUse` для команды Bash получает это на stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "scratchpad_dir": "/tmp/claude-1000/-home-user-my-project/abc123/scratchpad",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

Поля `tool_name`, `tool_input` и `tool_use_id` специфичны для события. Каждый раздел [hook event](#hook-events) документирует дополнительные поля для этого события.

<h3 id="exit-code-output">
  Выходные коды выхода
</h3>

Код выхода из вашей команды hook сообщает Claude Code, должно ли действие продолжаться, быть заблокировано или игнорироваться. Код выхода не действует в одиночку. Claude Code читает поля [JSON output](#json-output) из stdout при каждом коде выхода, не только 0, и для событий, которые используют стандартную модель решения, проанализированный объект, который проходит валидацию схемы, вступает в силу наряду с кодом. Блокировка Exit 2 — это единственный результат, который JSON не может переопределить.

Две таблицы владеют исключениями для каждого события: [Exit code 2 behavior per event](#exit-code-2-behavior-per-event) говорит, что коды выхода делают для каждого события, и [Decision control](#decision-control) говорит, какие поля решения каждое событие учитывает. Универсальные поля, такие как `systemMessage`, работают на большинстве событий и перечислены в таблице [JSON output](#json-output).

<h4 id="exit-code-0">
  Exit code 0
</h4>

Exit 0 означает успех и является предполагаемым кодом выхода, когда вы печатаете JSON для структурированного управления.

Для большинства событий Claude Code записывает stdout в журнал отладки и не показывает его в транскрипте. Исключения — это `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` и `PostModelSwitch`, где Claude Code добавляет простой текст stdout как контекст, который Claude может видеть и действовать.

Читает ли Claude Code ваш stdout как [JSON output](#json-output) или как простой текст, зависит от того, как он начинается и заканчивается, игнорируя окружающие пробелы:

* **Начинается с `{` и заканчивается на `}`**: Claude Code анализирует его как JSON. Когда выход состоит из двух или более строк, которые каждая анализируются как JSON самостоятельно, и ни одна строка не является объектом [JSON output](#json-output), который устанавливает поле, Claude Code рассматривает весь выход как простой текст. Когда одна из этих строк устанавливает поле, весь выход является ошибкой анализа, описанной ниже.
* **Начинается с `{` но не заканчивается на `}`**: Claude Code рассматривает это как простой текст.
* **Начинается с чего-либо еще**: Claude Code рассматривает это как простой текст, JSON массив или включенную строку JSON в кавычках.

Для событий, которые используют стандартную модель решения, exit 0 с проанализированным объектом, который не проходит валидацию схемы, является неблокирующей ошибкой: действие продолжается, и транскрипт показывает уведомление об ошибке `<hook name> hook error` с сообщением валидации. То же самое происходит при любом коде выхода, отличном от 2, в то время как [exit 2 все еще блокирует](#exit-code-2).

Для событий, которые используют стандартную модель решения, когда Claude Code пытается анализировать ваш stdout как JSON и не может, он сообщает о неблокирующей ошибке при каждом коде выхода, отличном от 2. Транскрипт показывает уведомление об ошибке `<hook name> hook error` с сообщением анализа. На событиях, которые добавляют простой текст stdout как контекст, Claude Code не добавляет текст. До v2.1.248 Claude Code рассматривал этот stdout как простой текст.

Stderr из hook, который выходит с 0, идет только в журнал отладки, никогда в транскрипт, и Claude его не видит. Чтобы прочитать его самостоятельно, включите [debug logging](#debug-hooks). Чтобы вывести предупреждение Claude из hook `PostToolUse` или `PostToolUseFailure`, выйдите с 2 вместо этого, чтобы [Claude видел stderr](#exit-code-2-behavior-per-event), даже если инструмент уже запустился.

<h4 id="exit-code-2">
  Exit code 2
</h4>

Exit 2 означает блокирующую ошибку. На [событиях, которые могут блокировать](#exit-code-2-behavior-per-event), exit 2 блокирует независимо от того, печатаете ли вы JSON: даже JSON `permissionDecision` из `"allow"` не может его переопределить. Claude Code все еще читает любой действительный [JSON output](#json-output) на stdout. На `Elicitation` и `ElicitationResult`, `hookSpecificOutput` hook с exit-2 игнорируется.

Сообщение блокировки — это причина из решения блокировки вашего JSON, когда оно его делает, и ваш текст stderr в противном случае. Что делает блокировка, варьируется в зависимости от события: `PreToolUse` блокирует вызов инструмента, `UserPromptSubmit` отклоняет запрос и так далее. [Exit code 2 behavior per event](#exit-code-2-behavior-per-event) перечисляет эффект для каждого события, и каждый раздел события говорит, куда идет сообщение.

Hook, который выходит с 2 при печати JSON, который не проходит валидацию схемы [JSON output](#json-output), все еще блокирует: Claude Code использует stderr как причину блокировки и записывает ошибку валидации в журнал отладки. До v2.1.214 Claude Code рассматривал эту комбинацию как неблокирующую ошибку и действие продолжалось.

Этот скрипт блокирует команды `rm`, выходя с 2 и оставляет каждую другую команду нормальному потоку разрешений:

```bash theme={null}
#!/bin/bash
# Reads JSON input from stdin, checks the command
input=$(cat)
command=$(jq -r '.tool_input.command' <<<"$input")

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Blocking error: tool call is prevented
fi

exit 0  # No decision: the normal permission flow applies
```

<h4 id="other-exit-codes">
  Другие коды выхода
</h4>

Любой другой код выхода не блокирует сам по себе для большинства hook событий. Что происходит, зависит от вашего stdout:

* С проанализированным объектом, который проходит валидацию схемы, для событий, которые используют стандартную модель решения, Claude Code игнорирует код выхода и только JSON решает результат:
  * Каждое поле, которое событие поддерживает, учитывается, включая `permissionDecision`, `additionalContext`, `updatedInput` и `systemMessage`, и hook не сообщается как ошибка.
  * [Decision control](#decision-control) перечисляет поля решения для каждого события; универсальные поля, такие как `systemMessage`, следуют таблице [JSON output](#json-output).
* С проанализированным объектом, который не проходит валидацию схемы, для событий, которые используют стандартную модель решения, это то же самое неблокирующее ошибка, что и [на exit 0](#exit-code-0): действие продолжается, и уведомление `<hook name> hook error` содержит сообщение валидации.
* С stdout, который Claude Code [пытается анализировать как JSON](#exit-code-0) и не может, Claude Code сообщает о той же неблокирующей ошибке, что и на exit 0 для событий, которые используют стандартную модель решения. Действие продолжается, и уведомление содержит сообщение анализа.
* С stdout, который Claude Code [рассматривает как простой текст](#exit-code-0), или с пустым stdout, это неблокирующая ошибка для большинства hook событий: действие продолжается, и транскрипт показывает уведомление об ошибке `<hook name> hook error`, за которым следует первая строка stderr, с префиксом `Failed with non-blocking status code:`. Чтобы захватить полный stderr, включите [debug logging](#debug-hooks).

События вне стандартной модели решения сохраняют свои собственные строки в [таблице для каждого события](#exit-code-2-behavior-per-event): `WorktreeCreate` не создает при любом ненулевом выходе, независимо от того, что говорит ваш JSON, и события, которые полностью игнорируют выход hook, такие как `StopFailure`, игнорируют ваш JSON при каждом коде выхода, кроме полей побочных эффектов, таких как `terminalSequence`, которые все еще срабатывают.

Hook, который не может запуститься, попадает в ту же неблокирующую корзину. Когда путь скрипта не существует или не исполняемый, оболочка выходит с кодом, например 127, и вы видите то же уведомление с сообщением интерпретатора, например `Failed with non-blocking status code: /bin/sh: /path/to/hook.sh: No such file or directory`. Для большинства hook событий действие продолжается. Когда вы устанавливаете hook политики, следите за этим уведомлением при его первом запуске: неправильно введенный путь в `settings.json` оставляет ворота молча отключенными.

<Warning>
  Для большинства hook событий exit code 2 — это единственный код выхода, который блокирует только через код. Без действительного JSON на stdout Claude Code рассматривает exit code 1 как неблокирующую ошибку и продолжает действие, даже хотя 1 — это обычный код отказа Unix. Если ваш hook предназначен для обеспечения политики, используйте `exit 2`. События worktree отличаются: любой ненулевой код выхода из `WorktreeCreate` прерывает создание worktree, и любой ненулевой код выхода из `WorktreeRemove` делает удаление worktree неудачным, если каталог все еще существует после этого.
</Warning>

<h4 id="timeouts">
  Timeouts
</h4>

Кроме hook команды, который вы запускаете с [`async: true`](#run-hooks-in-the-background), Claude Code отменяет hook `command`, `http` или `mcp_tool`, который достигает своего [`timeout`](#common-fields), отбрасывая выход hook, поэтому на большинстве событий истекший по времени hook не отображает решение.

На [`PreModelSwitch`](#premodelswitch), hook, отмененный при его timeout, блокирует переключение модели. На `PreToolUse` две семьи hook отличаются:

* Истекший по времени hook `command`, `http` или `mcp_tool` не блокирует вызов инструмента. Вызов продолжается через нормальный [поток разрешений](/docs/ru/permissions), поэтому не рассчитывайте на зависший hook, чтобы действовать как ворота.
* [Agent SDK callback hook](/docs/ru/agent-sdk/hooks), который превышает свой timeout, [блокирует вызов инструмента](#pretooluse).

<h4 id="exit-code-2-behavior-per-event">
  Exit code 2 behavior per event
</h4>

Exit code 2 — это способ, которым hook сигнализирует "стоп, не делай этого". Эффект зависит от события, потому что некоторые события представляют действия, которые могут быть заблокированы (например, вызов инструмента, который еще не произошел), а другие представляют вещи, которые уже произошли или не могут быть предотвращены.

| Hook event            | Может блокировать? | Что происходит на exit 2                                                                                                                                                                                                                                                   |
| :-------------------- | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Да                 | Блокирует вызов инструмента                                                                                                                                                                                                                                                |
| `PermissionRequest`   | Нет                | Exit code 2 не учитывается для этого события и поток разрешений продолжается без изменений. Отклоните через объект [`decision`](#permissionrequest-decision-control) вместо этого                                                                                          |
| `UserPromptSubmit`    | Да                 | Блокирует обработку запроса и стирает запрос                                                                                                                                                                                                                               |
| `UserPromptExpansion` | Да                 | Блокирует расширение                                                                                                                                                                                                                                                       |
| `Stop`                | Да                 | Предотвращает остановку Claude, продолжает разговор                                                                                                                                                                                                                        |
| `SubagentStop`        | Да                 | Предотвращает остановку subagent                                                                                                                                                                                                                                           |
| `TeammateIdle`        | Да                 | Предотвращает переход товарища в режим ожидания, поэтому он продолжает работать                                                                                                                                                                                            |
| `TaskCreated`         | Да                 | Откатывает создание задачи                                                                                                                                                                                                                                                 |
| `TaskCompleted`       | Да                 | Предотвращает отметку задачи как завершенной                                                                                                                                                                                                                               |
| `ConfigChange`        | Да                 | Блокирует вступление изменения конфигурации в силу (кроме `policy_settings`)                                                                                                                                                                                               |
| `StopFailure`         | Нет                | Выход и код выхода игнорируются, кроме `terminalSequence`                                                                                                                                                                                                                  |
| `PostToolUse`         | Нет                | Показывает stderr Claude; инструмент уже запустился                                                                                                                                                                                                                        |
| `PostToolUseFailure`  | Нет                | Показывает stderr Claude; инструмент уже не удался                                                                                                                                                                                                                         |
| `PostToolBatch`       | Да                 | Останавливает агентный цикл перед следующим вызовом модели                                                                                                                                                                                                                 |
| `PermissionDenied`    | Нет                | Выход и stderr игнорируются, потому что отказ уже произошел. Используйте JSON `hookSpecificOutput.retry: true`, чтобы сказать модели, что она может повторить попытку; Claude Code игнорирует `retry: true` для [отказов без вердикта](#permissiondenied-decision-control) |
| `Notification`        | Нет                | Выход и stderr игнорируются                                                                                                                                                                                                                                                |
| `SubagentStart`       | Нет                | Показывает stderr только пользователю                                                                                                                                                                                                                                      |
| `SessionStart`        | Нет                | Показывает stderr только пользователю                                                                                                                                                                                                                                      |
| `Setup`               | Нет                | Выход и stderr игнорируются                                                                                                                                                                                                                                                |
| `SessionEnd`          | Нет                | Показывает stderr только пользователю                                                                                                                                                                                                                                      |
| `CwdChanged`          | Нет                | Показывает stderr только пользователю                                                                                                                                                                                                                                      |
| `DirectoryAdded`      | Нет                | Stderr идет в журнал отладки; каталог уже добавлен                                                                                                                                                                                                                         |
| `FileChanged`         | Нет                | Показывает stderr только пользователю                                                                                                                                                                                                                                      |
| `PreCompact`          | Да                 | Блокирует компактирование                                                                                                                                                                                                                                                  |
| `PostCompact`         | Нет                | Показывает stderr только пользователю                                                                                                                                                                                                                                      |
| `PreModelSwitch`      | Да                 | Блокирует переключение модели и показывает stderr пользователю                                                                                                                                                                                                             |
| `PostModelSwitch`     | Нет                | Показывает stderr только пользователю; модель уже переключилась                                                                                                                                                                                                            |
| `Elicitation`         | Да                 | Отклоняет запрос информации                                                                                                                                                                                                                                                |
| `ElicitationResult`   | Да                 | Блокирует ответ (действие становится отклонением)                                                                                                                                                                                                                          |
| `WorktreeCreate`      | Да                 | Любой ненулевой код выхода вызывает ошибку создания worktree                                                                                                                                                                                                               |
| `WorktreeRemove`      | Да                 | Любой ненулевой код выхода вызывает ошибку удаления worktree, если каталог все еще существует после этого. См. [WorktreeRemove](#worktreeremove) для того, что происходит с каталогом                                                                                      |
| `InstructionsLoaded`  | Нет                | Код выхода игнорируется                                                                                                                                                                                                                                                    |
| `MessageDisplay`      | Нет                | Отображается исходный текст                                                                                                                                                                                                                                                |

Для `SessionStart`, `SubagentStart` и `PostModelSwitch`, Claude Code отображает stderr exit code 2 в транскрипте как уведомление об ошибке `<hook name> hook error`, так же как оно отображает [неблокирующую ошибку](#exit-code-output). Claude его не видит, и сеанс или subagent продолжается. Для `SubagentStart` уведомление появляется в собственном транскрипте subagent, а не в родительском разговоре.

<h3 id="http-response-handling">
  HTTP response handling
</h3>

HTTP hooks используют коды состояния HTTP и тела ответов вместо кодов выхода и stdout. Результаты ниже применяются к большинству событий; событие с его собственным контрактом отказа в [таблице для каждого события](#exit-code-2-behavior-per-event), такое как `WorktreeCreate`, применяет этот контракт к неудачному HTTP hook также:

* **2xx с пустым телом**: успех, эквивалентно exit code 0 без выхода
* **2xx с телом объекта JSON**: анализируется с использованием той же схемы [JSON output](#json-output), что и hooks команд. Тело, которое не проходит валидацию схемы, является неблокирующей ошибкой
* **2xx с любым другим телом, таким как простой текст**: неблокирующая ошибка, обрабатывается так же, как статус non-2xx. Claude Code не добавляет текст в контекст Claude
* **Статус non-2xx**: неблокирующая ошибка, выполнение продолжается
* **Ошибка соединения**: неблокирующая ошибка, выполнение продолжается
* **Timeout**: hook отменяется, как описано в разделе [Timeouts](#timeouts)

В отличие от hooks команд, HTTP hooks не могут сигнализировать блокирующую ошибку только через коды состояния. Чтобы заблокировать вызов инструмента или отклонить разрешение, верните ответ 2xx с телом JSON, содержащим соответствующие поля решения.

<h3 id="json-output">
  JSON output
</h3>

Коды выхода позволяют вам только блокировать или молчать, но JSON output дает вам более точное управление. Вместо выхода с кодом 2 для блокировки, выйдите с 0 и напечатайте объект JSON на stdout. Claude Code читает конкретные поля из этого JSON для управления поведением, включая [decision control](#decision-control) для блокировки, разрешения или эскалации пользователю.

<Note>
  Выберите один подход для каждого hook: либо используйте коды выхода только для сигнализации, либо выйдите с 0 и напечатайте JSON для структурированного управления. Если вы их смешиваете, exit 2 сохраняет свой [блокирующий эффект](#exit-code-2-behavior-per-event), и Claude Code все еще читает поля JSON, с единственным исключением elicitation, отмеченным в разделе [Exit code 2](#exit-code-2).
</Note>

Stdout вашего hook должен содержать только объект JSON. Если ваш профиль оболочки печатает текст при запуске, это может помешать анализу JSON. См. [Hook JSON has no effect](/docs/ru/hooks-guide#hook-json-has-no-effect) в руководстве по устранению неполадок.

Строки `additionalContext`, `systemMessage` и `initialUserMessage` hook, а также его простой stdout, ограничены 10 000 символов:

* **Область**: Claude Code измеряет каждую строку отдельно, даже когда несколько hooks запускаются для одного события. Для JSON output каждое поле измеряется отдельно; простой stdout измеряется целиком.
* **Превышение лимита**: Claude Code сохраняет выход в файл в каталоге сеанса и заменяет его путем к файлу и предпросмотром до первых 2000 символов. Большой действительный результат Bash обрабатывается так же, как описано в разделе [Output limits](/docs/ru/tools-reference#output-limits). В отличие от этого потолка Bash, эта крышка не имеет параметра или переменной окружения для ее повышения.
* **Чтение файла**: Claude Code не просит Claude прочитать файл, поэтому держите все, что Claude должен всегда видеть, в пределах крышки.

Объект JSON поддерживает три вида полей:

* **Универсальные поля**, такие как `continue`, перечислены в таблице ниже. Каждое событие их принимает, но некоторые события игнорируют их или доставляют `systemMessage` в другое место, чем транскрипт. Каждый раздел события говорит об этом. `terminalSequence` работает на этих событиях также, с исключениями, перечисленными в разделе [Emit terminal notifications](#emit-terminal-notifications).
* **Top-level `decision` и `reason`** используются некоторыми событиями для блокировки или предоставления обратной связи.
* **`hookSpecificOutput`** — это вложенный объект для событий, которым нужно более богатое управление. Он требует поле `hookEventName`, установленное на имя события.

| Поле               | По умолчанию | Описание                                                                                                                                                                                                                                                                                                                                              |
| :----------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`       | Если `false`, Claude полностью прекращает обработку после запуска hook. Имеет приоритет над любыми полями решения, специфичными для события                                                                                                                                                                                                           |
| `stopReason`       | нет          | Сообщение, показанное пользователю, когда `continue` равно `false`. Оно остается в разговоре, поэтому Claude видит его, если разговор продолжается                                                                                                                                                                                                    |
| `suppressOutput`   | `false`      | Не имеет эффекта: Claude Code принимает поле, но не действует на него. Stdout успешного hook никогда не показывается в транскрипте и записывается в журнал отладки                                                                                                                                                                                    |
| `systemMessage`    | нет          | Предупреждающее сообщение, показанное пользователю. В выводе [Agent SDK](/docs/ru/agent-sdk/overview) и [`--output-format stream-json`](/docs/ru/headless), оно может поступить как [`SDKInformationalMessage`](/docs/ru/agent-sdk/typescript#sdkinformationalmessage)                                                                                               |
| `terminalSequence` | нет          | Последовательность escape терминала для Claude Code для выпуска от вашего имени, такая как уведомление рабочего стола, заголовок окна или звонок. Ограничено OSC `0`/`1`/`2`/`9`/`99`/`777` и BEL. Если значение содержит что-либо вне списка разрешений, поле игнорируется. Используйте это вместо записи в `/dev/tty`, которая недоступна для hooks |

Чтобы полностью остановить Claude:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

Для hooks `PreToolUse` и `PostToolUse` остановка применяется даже когда вызов инструмента не удается или завершается, пока Claude все еще потоком ответ.

<h4 id="emit-terminal-notifications">
  Emit terminal notifications
</h4>

Hooks запускаются без управляющего терминала, поэтому запись последовательностей escape непосредственно в `/dev/tty` не удается. Вместо этого верните последовательность escape в поле `terminalSequence` и Claude Code выпустит ее от вашего имени через свой собственный путь записи терминала. Это свободно от гонок, работает внутри tmux и GNU screen, и работает на Windows, где нет `/dev/tty`.

Поле принимает строку из одной или нескольких последовательностей escape в списке разрешений:

* OSC `0`, `1`, `2`: заголовки окна и значков
* OSC `9`: уведомления iTerm2, ConEmu, Windows Terminal и WezTerm, включая прогресс панели задач `9;4`
* OSC `99`: уведомления Kitty
* OSC `777`: уведомления urxvt, Ghostty и Warp
* Bare BEL

Последовательности могут быть завершены BEL или ST. Все, что находится вне списка разрешений, включая последовательности курсора CSI и цвета, последовательности палитры OSC, гиперссылки OSC 8, записи буфера обмена OSC 52 и OSC 1337, отклоняется и поле игнорируется.

Claude Code выпускает саму последовательность, когда обрабатывает выход вашего hook, поэтому поле работает на событиях, которые игнорируют `systemMessage` и `continue`, такие как `Notification` и `StopFailure`. Оно имеет два ограничения:

* Claude Code выпускает последовательность только в интерактивном сеансе и только пока его интерфейс находится на экране. В неинтерактивном режиме с флагом `-p` и в Agent SDK он игнорирует поле.
* Hook команды `WorktreeCreate` не может вернуть JSON, потому что Claude Code читает его stdout как путь worktree. HTTP hook `WorktreeCreate` возвращает JSON и может включать поле.

Пример ниже срабатывает уведомление рабочего стола из hook `Notification`. Последовательность escape строится с помощью `printf` восьмеричных escape, поэтому управляющие байты никогда не появляются в командной строке оболочки, и `jq -n --arg` строит выход JSON, поэтому кавычки, обратные слэши и новые строки в сообщении уведомления правильно экранируются:

```bash theme={null}
#!/bin/bash
# Notification hook: ping the desktop when Claude Code needs attention.
input=$(cat)
title="Claude Code"
body=$(jq -r '.message // "Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

Форма `{ "terminalSequence": "..." }` одинакова из любой оболочки или языка.

<h4 id="add-context-for-claude">
  Add context for Claude
</h4>

Поле `additionalContext` передает строку из вашего hook в контекстное окно Claude. Claude Code оборачивает строку в напоминание системы и вставляет ее в разговор в точке, где сработал hook. Claude читает напоминание при следующем запросе модели, но оно не появляется как сообщение чата в интерфейсе.

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

* [SessionStart](#sessionstart) и [SubagentStart](#subagentstart): в начале разговора, перед первым запросом
* [UserPromptSubmit](#userpromptsubmit) и [UserPromptExpansion](#userpromptexpansion): рядом с отправленным запросом
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) и [PostToolBatch](#posttoolbatch): рядом с результатом инструмента
* [Stop](#stop) и [SubagentStop](#subagentstop): в конце хода. Разговор продолжается, поэтому Claude может действовать на обратную связь. См. [Stop decision control](#stop-decision-control)
* [PostModelSwitch](#postmodelswitch): со следующим запросом после переключения. См. [PostModelSwitch decision control](#postmodelswitch-decision-control) для синхронизации

Когда несколько hooks возвращают `additionalContext` для одного события, Claude получает все значения.

Если значение превышает 10 000 символов, Claude Code записывает текст в файл в каталоге сеанса и передает Claude путь к файлу с предпросмотром до первых 2000 символов вместо этого. Claude может прочитать файл, но Claude Code не просит его.

Используйте `additionalContext` для информации, которую Claude должен знать о текущем состоянии вашей среды или операции, которая только что запустилась:

* **Состояние среды**: текущая ветвь, цель развертывания или активные флаги функций
* **Условные правила проекта**: какая команда теста применяется к только что отредактированному файлу, какие каталоги доступны только для чтения в этом worktree
* **Внешние данные**: открытые проблемы, назначенные вам, недавние результаты CI, контент, полученный из внутреннего сервиса

Для инструкций, которые никогда не изменяются, предпочитайте [CLAUDE.md](/docs/ru/memory). Он загружается без запуска скрипта и является стандартным местом для статических соглашений проекта.

Напишите текст как фактические утверждения, а не как императивные системные инструкции. Фразировка, такая как "Цель развертывания — production" или "Этот репо использует `bun test`", читается как информация о проекте. Текст, сформулированный как внеполосные системные команды, может вызвать защиту Claude от инъекций подсказок, что заставляет Claude вывести текст вам вместо того, чтобы рассматривать его как контекст.

Claude Code сохраняет введенный текст в транскрипте сеанса. Для событий середины сеанса, таких как `PostToolUse` или `UserPromptSubmit`, когда вы возобновляете с `--continue` или `--resume`, Claude Code воспроизводит сохраненный текст, а не повторно запускает hook для прошлых ходов, поэтому значения, такие как временные метки или SHA коммитов, становятся устаревшими. Hooks `SessionStart` запускаются снова при возобновлении с `source`, установленным на `"resume"`, или `"fork"`, если вы добавили `--fork-session`, поэтому они могут обновить свой контекст.

<h4 id="decision-control">
  Decision control
</h4>

Не каждое событие поддерживает блокировку или управление поведением через JSON. События, которые это делают, каждое использует другой набор полей для выражения этого решения. Используйте эту таблицу как быструю ссылку перед написанием hook:

| События                                                                                                                             | Паттерн решения                               | Ключевые поля                                                                                                                                                                                                                                                                                     |
| :---------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | Top-level `decision`                          | `decision: "block"`, `reason`. Stop и SubagentStop также принимают `hookSpecificOutput.additionalContext` для [неошибочной обратной связи, которая продолжает разговор](#stop-decision-control)                                                                                                   |
| TeammateIdle, TaskCompleted                                                                                                         | Exit code или `continue: false`               | Exit code 2 блокирует действие с обратной связью stderr. JSON `{"continue": false, "stopReason": "..."}` также полностью останавливает товарища, совпадая с поведением hook `Stop`; [TaskCompleted игнорирует это, когда инструмент `TaskUpdate` вызвал событие](#taskcompleted-decision-control) |
| TaskCreated                                                                                                                         | Exit code или top-level `decision`            | Exit code 2 или `decision: "block"` [отменяет задачу](#taskcreated-decision-control) и возвращает сообщение Claude. `continue: false` игнорируется                                                                                                                                                |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                          | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                                                           |
| PreModelSwitch                                                                                                                      | `hookSpecificOutput` или top-level `decision` | `permissionDecision` (allow/deny/ask), `permissionDecisionReason`. `decision: "block"` также [отменяет переключение](#premodelswitch-decision-control)                                                                                                                                            |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                          | `decision.behavior` (allow/deny)                                                                                                                                                                                                                                                                  |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                          | `retry: true` сообщает модели, что она может повторить попытку отклоненного вызова инструмента; Claude Code игнорирует это для [отказов без вердикта](#permissiondenied-decision-control)                                                                                                         |
| WorktreeCreate                                                                                                                      | path return                                   | Hook команды печатает путь на stdout; HTTP hook возвращает `hookSpecificOutput.worktreePath`. Ошибка hook или отсутствующий путь не создает                                                                                                                                                       |
| WorktreeRemove                                                                                                                      | Exit code                                     | Любой ненулевой код выхода делает удаление неудачным, если каталог все еще существует после этого. Выход JSON отбрасывается                                                                                                                                                                       |
| Elicitation                                                                                                                         | `hookSpecificOutput`                          | `action` (accept/decline/cancel), `content` (значения полей формы для accept)                                                                                                                                                                                                                     |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                          | `action` (accept/decline/cancel), `content` (значения полей формы переопределяют)                                                                                                                                                                                                                 |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                          | `displayContent` заменяет отображаемый текст на экране. Только отображение: транскрипт и то, что видит Claude, сохраняют оригинал                                                                                                                                                                 |
| SessionStart, SubagentStart, PostModelSwitch                                                                                        | Только контекст                               | `hookSpecificOutput.additionalContext` добавляет контекст для Claude. SessionStart также принимает [`initialUserMessage`, `watchPaths`, `sessionTitle` и `reloadSkills`](#sessionstart-decision-control). Нет блокировки или управления решением                                                  |
| Setup, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, DirectoryAdded, FileChanged              | Нет                                           | Нет управления решением. Используется для побочных эффектов, таких как логирование или очистка                                                                                                                                                                                                    |

Несколько событий также могут переписывать контент, а не только разрешать или блокировать его:

* `PreToolUse`: `updatedInput` непосредственно под `hookSpecificOutput` заменяет аргументы инструмента перед его запуском. См. [PreToolUse decision control](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` внутри объекта `decision`. См. [PermissionRequest decision control](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` заменяет результат инструмента. См. [PostToolUse decision control](#posttooluse-decision-control)
* `UserPromptSubmit`: не может переписать запрос; он только вводит `additionalContext` рядом с ним

Для случаев использования редакции или трансформации перехватите на `PreToolUse` для исходящих входов инструмента и `PostToolUse` для входящих результатов инструмента.

Вот примеры каждого паттерна в действии:

<Tabs>
  <Tab title="Top-level decision">
    Единственное значение для `decision` — это `"block"`. Чтобы разрешить действию продолжаться, опустите `decision` из вашего JSON или выйдите с 0 без какого-либо JSON вообще:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Использует `hookSpecificOutput` для более богатого управления: разрешить, отклонить или эскалировать пользователю. Вы также можете изменить входные данные инструмента перед его запуском или вводить дополнительный контекст для Claude. См. [PreToolUse decision control](#pretooluse-decision-control) для полного набора опций.

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
    Использует `hookSpecificOutput` для разрешения или отклонения запроса разрешения от имени пользователя. При разрешении вы также можете изменить входные данные инструмента или применить правила разрешений, чтобы пользователь не был запрошен снова. См. [PermissionRequest decision control](#permissionrequest-decision-control) для полного набора опций.

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

Для расширенных примеров, включая валидацию команд Bash, фильтрацию запросов и скрипты автоматического одобрения, см. [What you can automate](/docs/ru/hooks-guide#what-you-can-automate) в руководстве и [Bash command validator reference implementation](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  События hooks
</h2>

Каждое событие соответствует точке в жизненном цикле Claude Code, где могут выполняться hooks. Разделы ниже упорядочены в соответствии с жизненным циклом: от настройки сеанса через агентский цикл к завершению сеанса. Каждый раздел описывает, когда срабатывает событие, какие matchers оно поддерживает, какой JSON-ввод оно получает и как управлять поведением через вывод.

<h3 id="sessionstart">
  SessionStart
</h3>

Запускается, когда Claude Code начинает новый сеанс или возобновляет существующий сеанс. Полезно для загрузки контекста разработки, такого как существующие проблемы или недавние изменения в вашей кодовой базе, или для установки переменных окружения. Для статического контекста, который не требует скрипта, используйте вместо этого [CLAUDE.md](/docs/ru/memory).

SessionStart запускается в каждом сеансе, поэтому держите эти hooks быстрыми. Поддерживаются только hooks `type: "command"` и `type: "mcp_tool"`. См. [MCP tool hook fields](#mcp-tool-hook-fields) для информации о том, когда выполняются hooks `mcp_tool`.

Значение matcher соответствует тому, как был инициирован сеанс:

| Matcher   | Когда срабатывает                                                                                                                |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `startup` | Новый сеанс                                                                                                                      |
| `resume`  | `--resume`, `--continue` или `/resume`                                                                                           |
| `clear`   | `/clear`                                                                                                                         |
| `compact` | Автоматическое или ручное сжатие                                                                                                 |
| `fork`    | Новый сеанс, разветвленный из существующего: `--fork-session` с `--resume` или `--continue`, фоновая копия `/fork` или `/branch` |

До версии 2.1.214 разветвленные сеансы сообщали источник `"resume"`.

Когда вы запускаете интерактивный сеанс, возобновляете разговор при запуске с `--continue` или `--resume` или запускаете `/clear`, hooks SessionStart выполняются в фоновом режиме. Вы можете сразу же печатать, и возобновленный разговор появляется без ожидания hooks. Первый ответ Claude все еще ждет завершения hooks, поэтому их контекст достигает Claude.

Когда вы переключаете разговоры с `/resume` внутри сеанса, переключение ждет завершения hooks. Если вы запустите `/clear` или переключитесь на другой разговор, пока фоновые hooks все еще выполняются, ничего из того, что они возвращают, не применяется к сеансу.

То же самое ожидание применяется при запуске, включая возобновленный сеанс: подсказка, которую вы отправляете, пока hooks SessionStart все еще выполняются, не достигает Claude до их завершения.

Во время любого ожидания нажмите `Esc`, чтобы вернуть подсказку в ввод без отправки. Hooks продолжают выполняться.

<h4 id="sessionstart-input">
  Ввод SessionStart
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks SessionStart получают `source` и опционально `model`, `agent_type` и `session_title`:

| Поле            | Описание                                                                                                                                                                                                                                      |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Как был запущен сеанс: `"startup"` для новых сеансов, `"resume"` для возобновленных сеансов, `"clear"` после `/clear`, `"compact"` после сжатия или `"fork"` для нового сеанса, разветвленного из существующего                               |
| `model`         | Идентификатор активной модели. Может быть опущен, например после `/clear` или когда сеанс восстанавливается через восстановление разговора, поэтому проверьте наличие поля перед его чтением                                                  |
| `agent_type`    | Имя агента, присутствует при запуске Claude Code с `claude --agent <name>`                                                                                                                                                                    |
| `session_title` | Текущее название сеанса, если оно уже установлено, например через `--name` или `/rename`. Hook, который выдает `sessionTitle`, может сначала проверить `session_title`, чтобы избежать перезаписи названия, установленного пользователем явно |

Когда `source` имеет значение `"resume"` или `"fork"` и транскрипт содержит по крайней мере один ответ от Claude, hooks SessionStart также получают четыре поля ниже. Ваш hook может использовать их для сообщения о стоимости возобновления устаревшего разговора перед первым запросом, например в [`systemMessage`](#json-output). Эти поля требуют Claude Code v2.1.251 или позже.

| Поле                          | Описание                                                                                                                                                              |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `seconds_since_last_response` | Настоящее время в секундах с момента последнего ответа в возобновленном транскрипте                                                                                   |
| `context_tokens`              | Токены, которые первый запрос возобновленного сеанса повторно отправляет как его подсказка                                                                            |
| `prompt_cache_likely_expired` | `true`, когда последний ответ старше [времени жизни кэша подсказок](/docs/ru/prompt-caching#cache-lifetime) сеанса или более позднее сжатие заменило кэшированный разговор |
| `estimated_cache_write_usd`   | Предполагаемая стоимость в долларах США записи `context_tokens` в кэш подсказок на модели сеанса, исключая ответ                                                      |

Этот пример показывает ввод для сеанса, возобновленного через 90 минут после его последнего ответа:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "resume",
  "model": "claude-opus-5",
  "seconds_since_last_response": 5400,
  "context_tokens": 182340,
  "prompt_cache_likely_expired": true,
  "estimated_cache_write_usd": 1.1396
}
```

<h4 id="sessionstart-decision-control">
  Управление решением SessionStart
</h4>

Claude Code добавляет stdout, который он [рассматривает как простой текст](#exit-code-0), в контекст Claude. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, вы можете вернуть эти поля, специфичные для события:

| Поле                 | Описание                                                                                                                                                                                                                                                                                                                                                              |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Строка, добавленная в контекст Claude в начале разговора, перед первой подсказкой. См. [Add context for Claude](#add-context-for-claude) для информации о том, как доставляется текст и что в него поместить                                                                                                                                                          |
| `initialUserMessage` | Строка, используемая как первое сообщение пользователя сеанса. Применяется в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`, где она становится первым ходом, даже если подсказка не предоставлена. Если подсказка предоставлена, она следует как следующий ход. В отличие от `additionalContext`, который присоединяется к существующему ходу, это создает ход |
| `sessionTitle`       | Устанавливает название сеанса с тем же эффектом, что и `/rename`. Используйте для автоматического именования сеансов из папки запуска, ветки git или имени worktree. Применяется, когда `source` имеет значение `"startup"`, `"resume"` или `"fork"`; игнорируется на `"clear"` и `"compact"`                                                                         |
| `watchPaths`         | Массив абсолютных путей для отслеживания событий [FileChanged](#filechanged) во время этого сеанса                                                                                                                                                                                                                                                                    |
| `reloadSkills`       | Логическое значение. Когда `true`, Claude Code повторно сканирует [skill](/docs/ru/skills) и директории команд после завершения hooks SessionStart, поэтому skills, установленные hook, доступны в том же сеансе, начиная с первой подсказки                                                                                                                               |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Поскольку простой stdout уже достигает Claude для этого события, hook, который только загружает контекст, может печатать в stdout напрямую без построения JSON. Используйте форму JSON, когда вам нужно объединить контекст с другими полями, такими как `sessionTitle`.

Используйте `reloadSkills`, когда hook SessionStart устанавливает или обновляет skills. Обнаружение skills обычно выполняется до завершения hooks SessionStart, поэтому файлы, которые hook записывает в `~/.claude/skills/` или `.claude/skills/`, в противном случае появятся только в следующем сеансе. Этот пример синхронизирует репозиторий общих skills и запрашивает повторное сканирование:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

URL репозитория является заполнителем; замените его на URL вашего репозитория skills. С заполнителем клонирование не удается и выводит сообщение `fatal:` в stderr. Stderr из hook SessionStart, который выходит с кодом 0, только информационный, поэтому запрос `reloadSkills` все еще применяется.

<h4 id="persist-environment-variables">
  Сохранение переменных окружения
</h4>

Hooks SessionStart имеют доступ к переменной окружения `CLAUDE_ENV_FILE`, которая предоставляет путь к файлу, где вы можете сохранить переменные окружения для последующих команд Bash.

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

<Note>
  `CLAUDE_ENV_FILE` доступен для hooks SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) и [FileChanged](#filechanged). Другие типы hooks не имеют доступа к этой переменной.
</Note>

<h3 id="setup">
  Setup
</h3>

Срабатывает только при запуске Claude Code с `--init-only` или с `--init` или `--maintenance` в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`. Не срабатывает при нормальном запуске. Используйте для одноразовой установки зависимостей или запланированной очистки, которую вы явно запускаете из CI или скриптов, отдельно от нормального запуска сеанса. Для инициализации для каждого сеанса используйте вместо этого [SessionStart](#sessionstart).

Значение matcher соответствует флагу CLI, который запустил hook:

| Matcher       | Когда срабатывает                           |
| :------------ | :------------------------------------------ |
| `init`        | `claude --init-only` или `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                   |

Когда вы запускаете `claude --init-only`, Claude Code запускает hooks Setup и hooks `SessionStart` с matcher `startup`, затем выходит без запуска разговора.

Когда вы запускаете или продолжаете разговор с `-p`, вам также нужно предоставить подсказку как аргумент или через stdin. Вы можете пропустить подсказку, когда hook `SessionStart` предоставляет [`initialUserMessage`](#sessionstart-decision-control) или когда вы возобновляете сеанс с [отложенным вызовом инструмента](#defer-a-tool-call-for-later).

При успехе `--init-only` ничего не выводит на терминал. Чтобы подтвердить, что hooks выполнились, начните с `claude --debug-file <path> --init-only`, заменив `<path>` на местоположение файла журнала, и проверьте журнал на наличие записей hooks Setup и SessionStart.

Поскольку Setup не срабатывает при каждом запуске, плагин, которому нужна установленная зависимость, не может полагаться только на Setup. Практический паттерн — проверить зависимость при первом использовании и установить при отсутствии, например hook или skill, который тестирует `${CLAUDE_PLUGIN_DATA}/node_modules` и запускает `npm install`, если отсутствует. См. [persistent data directory](/docs/ru/plugins/components#path-variables-and-persistent-data) для информации о том, где хранить установленные зависимости. Если вы распространяете свой плагин через marketplace, вам может не понадобиться этот паттерн: Claude Code [автоматически устанавливает подходящие зависимости пакетов Node.js](/docs/ru/plugins/loading#node-js-package-dependencies) при кэшировании плагина.

<h4 id="setup-input">
  Ввод Setup
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks Setup получают поле `trigger`, установленное либо на `"init"`, либо на `"maintenance"`:

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
  Управление решением Setup
</h4>

Hooks Setup не могут блокировать; выполнение продолжается при любом коде выхода. При каждом коде выхода Claude Code отбрасывает [поля JSON-вывода](#json-output) hook Setup, такие как `systemMessage`, `continue` и `hookSpecificOutput.additionalContext`. С `-p` stdout, stderr и код выхода hook Setup появляются в выводе запуска только как [`hook_response` события](/docs/ru/headless#read-session-metadata) при запуске с `--output-format stream-json --verbose`.

Hooks Setup имеют доступ к `CLAUDE_ENV_FILE`. Переменные, записанные в этот файл, сохраняются в последующих командах Bash для сеанса, как и в [hooks SessionStart](#persist-environment-variables). На `Setup` выполняются только hooks `type: "command"`. Hook `type: "mcp_tool"` на `Setup` всегда пропускается, как описано в [MCP tool hook fields](#mcp-tool-hook-fields).

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Срабатывает, когда файл `CLAUDE.md` или `.claude/rules/*.md` загружается в контекст. Это событие срабатывает при запуске сеанса для файлов, загружаемых с нетерпением, и снова позже, когда файлы загружаются с нетерпением, например когда Claude получает доступ к подпапке, содержащей вложенный `CLAUDE.md`, или когда условные правила с frontmatter `paths:` совпадают. Hook не поддерживает блокировку или управление решением. Он выполняется асинхронно в целях наблюдаемости.

Это событие не срабатывает, когда Claude [читает `AGENTS.md` напрямую](/docs/ru/memory#agents-md) через параметр **Project instructions**. Оно срабатывает, когда `CLAUDE.md` импортирует ваш `AGENTS.md` с `load_reason`, установленным на `include`, как для любого другого импортированного файла, и когда `CLAUDE.md` является символической ссылкой на него, как нормальная загрузка `CLAUDE.md`.

Matcher выполняется против `load_reason`. Например, используйте `"matcher": "session_start"` для срабатывания только для файлов, загруженных при запуске сеанса, или `"matcher": "path_glob_match|nested_traversal"` для срабатывания только для ленивых загрузок.

<h4 id="instructionsloaded-input">
  Ввод InstructionsLoaded
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks InstructionsLoaded получают эти поля:

| Поле                | Описание                                                                                                                                                                                                           |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Абсолютный путь к файлу инструкций, который был загружен                                                                                                                                                           |
| `memory_type`       | Область действия файла: `"User"`, `"Project"`, `"Local"` или `"Managed"`                                                                                                                                           |
| `load_reason`       | Почему файл был загружен: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` или `"compact"`. Значение `"compact"` срабатывает, когда файлы инструкций перезагружаются после события сжатия |
| `globs`             | Шаблоны глобов путей из frontmatter `paths:` файла, если они есть. Присутствует только для загрузок `path_glob_match`                                                                                              |
| `trigger_file_path` | Путь к файлу, доступ к которому запустил эту загрузку, для ленивых загрузок                                                                                                                                        |
| `parent_file_path`  | Путь к файлу инструкций родителя, который включил этот, для загрузок `include`                                                                                                                                     |

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
  Управление решением InstructionsLoaded
</h4>

Hooks InstructionsLoaded не имеют управления решением. Они не могут блокировать или изменять загрузку инструкций. Claude Code отбрасывает их [поля JSON-вывода](#json-output), такие как `systemMessage` и `continue`. Используйте это событие для аудита логирования, отслеживания соответствия или наблюдаемости.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Запускается, когда пользователь отправляет подсказку, перед обработкой Claude. Это позволяет вам добавить дополнительный контекст на основе подсказки/разговора, проверить подсказки или заблокировать определенные типы подсказок.

Hooks `UserPromptSubmit` имеют тайм-аут по умолчанию 30 секунд для типов `command`, `http` и `mcp_tool`, короче, чем 600-секундный стандарт для этих типов на большинстве других событий. Поскольку этот hook выполняется перед каждой подсказкой и блокирует обработку модели до его завершения, застрявший hook замораживает сеанс. Если вашему hook нужно больше времени, установите поле `timeout` в записи hook.

Помимо command hook, который вы запускаете с [`async: true`](#run-hooks-in-the-background), hook `UserPromptSubmit` command, HTTP или MCP tool, который достигает своего тайм-аута, отменяется и его вывод, включая любой `additionalContext`, отбрасывается. Подсказка все еще достигает Claude без этого контекста. Транскрипт показывает уведомление с названием hook, тайм-аутом, который сработал, и что вывод был отброшен.

[Callback hook Agent SDK](/docs/ru/agent-sdk/hooks) на `UserPromptSubmit`, который достигает своего тайм-аута, блокирует подсказку сообщением с названием hook и тайм-аутом, потому что callback там может действовать как политический шлюз, который не должен открываться при отказе. Сеанс продолжается. До версии 2.1.208 тайм-аут callback на этом событии заканчивал ход с ошибкой выполнения.

<h4 id="userpromptsubmit-input">
  Ввод UserPromptSubmit
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks UserPromptSubmit получают поле `prompt`, содержащее текст, отправленный пользователем. Вставленный контент, который свернулся в заполнитель `[Pasted text #N]`, прибывает развернутым на месте. В сеансах, где Claude Code [отмечает вставленный текст для Claude](/docs/ru/terminal-config#how-claude-treats-pasted-text), этот развернутый контент находится между строкой `<pasted_content id="…">` и строкой `</pasted_content id="…">`, поэтому учитывайте эти строки, если ваш hook анализирует подсказку.

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
  Управление решением UserPromptSubmit
</h4>

Hooks `UserPromptSubmit` могут управлять тем, обрабатывается ли подсказка пользователя, и добавлять контекст. Все [поля JSON-вывода](#json-output) доступны.

Есть два способа добавить контекст к разговору при коде выхода 0:

* **Простой текст stdout**: Claude Code добавляет stdout, который он [рассматривает как простой текст](#exit-code-0), в контекст Claude
* **JSON с `additionalContext`**: используйте формат JSON ниже для большего контроля. Значение `additionalContext` добавляется как контекст

Ни один канал не создает видимую запись в транскрипте. Простой stdout и значение `additionalContext` каждый вводятся как системное напоминание, которое начинается с названия hook; Claude читает оба. Чтобы подтвердить доставку, проверьте [debug log](#debug-hooks).

Чтобы заблокировать подсказку, верните объект JSON с `decision`, установленным на `"block"`:

| Поле                     | Описание                                                                                                                                    |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`               | `"block"` предотвращает обработку подсказки и стирает ее из контекста. Опустите, чтобы позволить подсказке продолжить                       |
| `reason`                 | Показано пользователю, когда `decision` имеет значение `"block"`. Не добавляется в контекст                                                 |
| `additionalContext`      | Строка, добавленная в контекст Claude рядом с отправленной подсказкой. См. [Add context for Claude](#add-context-for-claude)                |
| `sessionTitle`           | Устанавливает название сеанса. Используйте для автоматического именования сеансов на основе содержания подсказки                            |
| `suppressOriginalPrompt` | Если `true`, когда `decision` имеет значение `"block"`, опускает исходный текст подсказки из сообщения блокировки, показанного пользователю |

Hook, который блокирует выходом 2, маршрутизируется так же, как `reason`: сообщение блокировки показывает текст stderr пользователю и не добавляется в контекст.

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

Запускается, когда команда, введенная пользователем, расширяется в подсказку перед достижением Claude. Используйте это для блокировки определенных команд от прямого вызова, внедрения контекста для определенного skill или логирования того, какие команды вызывают пользователи. Например, hook, соответствующий `deploy`, может заблокировать `/deploy`, если отсутствует файл одобрения, или hook, соответствующий skill проверки, может добавить контрольный список проверки команды как `additionalContext`.

Это событие охватывает путь, который `PreToolUse` не охватывает: hook `PreToolUse`, соответствующий инструменту `Skill`, срабатывает только, когда Claude вызывает инструмент, но ввод `/skillname` напрямую обходит `PreToolUse`. `UserPromptExpansion` срабатывает на этом прямом пути.

Совпадает с `command_name`. Оставьте matcher пустым для срабатывания на каждой команде типа подсказки.

<h4 id="userpromptexpansion-input">
  Ввод UserPromptExpansion
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks UserPromptExpansion получают `expansion_type`, `command_name`, `command_args`, `command_source` и исходную строку `prompt`. Поле `expansion_type` имеет значение `slash_command` для skill и пользовательских команд или `mcp_prompt` для подсказок сервера MCP.

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
  Управление решением UserPromptExpansion
</h4>

Hooks `UserPromptExpansion` могут блокировать расширение или добавлять контекст. Все [поля JSON-вывода](#json-output) доступны.

| Поле                | Описание                                                                                                                    |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` предотвращает расширение команды. Опустите, чтобы позволить ей продолжить                                         |
| `reason`            | Показано пользователю, когда `decision` имеет значение `"block"`                                                            |
| `additionalContext` | Строка, добавленная в контекст Claude рядом с развернутой подсказкой. См. [Add context for Claude](#add-context-for-claude) |

Hook, который блокирует выходом 2, маршрутизируется так же, как `reason`: сообщение блокировки показывает текст stderr пользователю.

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

Запускается, пока сообщение помощника транслируется на экран. Claude Code отображает сообщение порциями: каждый раз, когда партия новых завершенных строк готова к отрисовке, hook выполняется один раз с этими строками, и Claude Code отображает текст замены hook на их месте. Длинное сообщение создает несколько вызовов; короткое сообщение может создать только один.

Используйте MessageDisplay для:

* удаления markdown для минимального отображения
* преобразования текста, который приложение Agent SDK показывает своим пользователям
* редактирования ключей API или внутренних имен хостов из ответов Claude

Claude Code удерживает каждую партию до возврата вашего hook, поэтому держите hook быстрым. Если hook не удается или истекает время ожидания, Claude Code отображает исходный текст. Тайм-аут по умолчанию для этого события составляет 10 секунд; если вашему hook нужно больше времени, установите поле `timeout` в записи hook.

MessageDisplay только для отображения: текст замены изменяет только то, что отображается на экране. Транскрипт и то, что видит Claude, сохраняют исходный текст, поэтому Claude никогда не видит замену, и подробный режим показывает исходный. Hook получает только текст сообщения помощника, поэтому результаты инструментов и текст, который вы вводите, отображаются без изменений.

MessageDisplay не поддерживает matchers и срабатывает для каждого сообщения помощника, которое транслирует текст; сообщения без текста, такие как ответы только с вызовом инструмента, не запускают его.

В неинтерактивных запусках, включая запросы Agent SDK и `claude -p`, MessageDisplay выполняется один раз для каждого сообщения помощника вместо один раз для каждой партии строк. Один вызов прибывает после завершения сообщения и несет полный текст сообщения: `index` имеет значение `0`, `final` имеет значение `true`, и `delta` содержит все сообщение. Hook, который собирает текст `delta` для каждого сообщения, получает одинаковый общий текст в обоих режимах.

<h4 id="messagedisplay-input">
  Ввод MessageDisplay
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks MessageDisplay получают идентификаторы для хода и сообщения, позицию этого вызова в сообщении и новый текст в `delta`. Границы партий зависят от того, как транслируется текст, поэтому используйте `index` и `final` для отслеживания прогресса через сообщение, а не ожидайте, что строки будут сгруппированы определенным образом.

| Поле         | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `turn_id`    | UUID текущего хода                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `message_id` | UUID сообщения помощника, которое отображается. Стабилен для каждой партии одного сообщения. Это не API `msg_…` id, поэтому его нельзя коррелировать с id сообщений транскрипта                                                                                                                                                                                                                                                             |
| `index`      | Индекс этой партии в сообщении, начиная с нуля                                                                                                                                                                                                                                                                                                                                                                                              |
| `final`      | `true` на последней партии сообщения. Каждое сообщение имеет ровно одну финальную партию                                                                                                                                                                                                                                                                                                                                                    |
| `delta`      | Новые завершенные строки с момента предыдущей партии, включая завершающие новые строки. Всегда целые строки, кроме финальной партии, которая может заканчиваться в середине строки. В интерактивных запусках delta финальной партии пуст, когда сообщение заканчивается на новой строке, поэтому рассматривайте `final`, а не непустой delta, как сигнал конца сообщения. В запусках Agent SDK и `claude -p` один вызов несет все сообщение |

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
  Вывод MessageDisplay
</h4>

Помимо [полей JSON-вывода](#json-output), доступных всем hooks, hooks MessageDisplay могут вернуть `displayContent` для замены delta на экране:

| Поле             | Описание                                                             |
| :--------------- | :------------------------------------------------------------------- |
| `displayContent` | Текст, отображаемый вместо delta. Опустите для отображения исходного |

Hooks MessageDisplay не имеют управления решением. Они не могут блокировать сообщение или изменять то, что хранится в транскрипте или отправляется Claude. Claude Code действует на `displayContent` из их JSON-вывода и отбрасывает `systemMessage` и `continue`.

Этот пример удаляет форматирование markdown из ответов Claude для отображения простого текста. Скрипт читает каждую партию из stdin, удаляет маркеры жирного шрифта и обратные кавычки встроенного кода из `delta` и возвращает результат как `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Зарегистрируйте command hook для события в файле параметров:

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

Партии без markdown проходят без изменений. Если скрипт не удается, например потому что `jq` отсутствует, Claude Code отображает исходный текст и отмечает сбой только в [debug output](#debug-hooks), а не в сеансе.

<h3 id="pretooluse">
  PreToolUse
</h3>

Запускается после того, как Claude создает параметры инструмента и перед обработкой вызова инструмента. Совпадает с любым названием инструмента, кроме `EndConversation`: встроенные инструменты, такие как `Bash`, `PowerShell`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `Workflow`, `WebFetch`, `WebSearch`, `AskUserQuestion` и `ExitPlanMode`, и любые [имена инструментов MCP](#match-mcp-tools).

Чтобы запустить hook, когда определенный файл изменяется на диске, независимо от того, что его написало, используйте вместо этого [FileChanged](#filechanged). В отличие от PreToolUse, Claude Code запускает hooks FileChanged после изменения, и они не имеют управления решением, поэтому они не могут блокировать запись.

<Warning>
  PreToolUse запускается только, когда Claude вызывает инструмент. Файлы, которые вы [ссылаетесь с `@` в вашей подсказке](/docs/ru/common-workflows#reference-files-and-directories), добавляются без вызова инструмента: Claude Code вставляет их содержимое при построении подсказки, поэтому для них не срабатывает hook PreToolUse, включая hooks, соответствующие `Read`. Чтобы заблокировать определенные пути от ссылок `@`, используйте вместо этого [`Read` deny rule](/docs/ru/permissions#read-and-edit).

  PreToolUse также не срабатывает для [`EndConversation`](/docs/ru/tools-reference#endconversation-tool-behavior).
</Warning>

Используйте [управление решением PreToolUse](#pretooluse-decision-control) для разрешения, отказа, запроса или отложения вызова инструмента.

[Callback hook Agent SDK](/docs/ru/agent-sdk/hooks) на `PreToolUse`, который превышает свой тайм-аут, блокирует вызов инструмента, и Claude получает результат ошибки с названием тайм-аута. Явный отказ, возвращенный другим hook, все еще имеет приоритет.

<h4 id="pretooluse-input">
  Ввод PreToolUse
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks PreToolUse получают `tool_name`, `tool_input` и `tool_use_id`.

Для [инструмента MCP](#match-mcp-tools) ввод также несет `mcp_server`, объект с `name` сервера и `source`, который говорит, откуда пришло определение сервера. Значения `source` включают `plugin`, `sdk` и области конфигурации, такие как `user` и `project`. [`McpServerProvenance`](/docs/ru/agent-sdk/typescript#mcpserverprovenance) в справочнике Agent SDK перечисляет их все и говорит, как рассматривать тот, который вы не узнаете. Основывайте решения о доверии на `source`, а не на `name` или префиксе инструмента `mcp__<server>__`. Поле `mcp_server` требует Claude Code v2.1.274 или позже.

Для инструментов файлов `Write`, `Edit` и `Read`, `tool_input.file_path` всегда абсолютен:

* Claude Code расширяет `~` и относительные пути перед выполнением hooks, поэтому hook, который совпадает с путями, не может быть обойден через `~` или относительное написание одного пути
* На Windows путь прибывает с разделителями обратной косой черты, даже когда ваш hook выполняется под Git Bash, где `$PWD` выглядит как `/c/project`
* Сравнение, написанное с прямыми косыми чертами, такое как проверка `/src/`, никогда не совпадает с путем обратной косой черты, и вызов инструмента продолжается, как если бы hook не имел ничего для блокировки
* Нормализуйте разделители перед сравнением: `FILE_PATH="${FILE_PATH//\\//}"` в Bash или `file_path.replace("\\", "/")` в Python, затем совпадайте с сегментом пути, такой как `/src/`, а не якорем с `^`, так как путь абсолютен

Вызов `Write` на Windows доставляет:

```json theme={null}
{
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "C:\\project\\src\\index.ts",
    "content": "..."
  },
  ...
}
```

Поля `tool_input` зависят от инструмента:

<a id="bash" />

<h5 id="bash">
  Bash
</h5>

Выполняет команды оболочки.

| Поле                | Тип     | Пример             | Описание                                                                                                                                            |
| :------------------ | :------ | :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | Команда оболочки для выполнения                                                                                                                     |
| `description`       | string  | `"Run test suite"` | Опциональное описание того, что делает команда                                                                                                      |
| `timeout`           | number  | `120000`           | Опциональный тайм-аут в миллисекундах. Значения выше [максимума](/docs/ru/tools-reference#bash-tool-behavior) уменьшаются до максимума, а не отклоняются |
| `run_in_background` | boolean | `false`            | Выполнять ли команду в фоновом режиме                                                                                                               |

Когда команда Bash изменяет файлы в репозитории Git, Claude Code может записать, что изменилось. Он записывает изменения в каждом режиме разрешений, когда параметр [`bashEditDiffEnabled`](/docs/ru/settings-reference#basheditdiffenabled) включает запись; запись этого параметра говорит, какие файлы могут его установить. В противном случае он записывает их только в режиме auto и режиме `bypassPermissions`, и только когда Claude Code направляет Claude на редактирование файлов через Bash. Установите `bashEditDiffEnabled` на `false`, чтобы отключить запись. Фоновые команды и команды только для чтения не несут diff.

Ваш [hook PostToolUse](#posttooluse) затем получает измененные файлы в `tool_response.bashEditDiff`. Список охватывает то, что изменилось в репозитории, пока выполнялась команда. Файлы, которые Git игнорирует, и файлы в подмодулях не указаны. Требует Claude Code v2.1.269 или позже.

<Note>
  Список лучше всего усилен и находится в публичной бета-версии. Claude Code может пропустить изменение, включить файл, который другой процесс изменил одновременно, или остановиться на его пределах размера. Форма поля может измениться. Используйте список для поиска того, что нужно проверить, а не для применения политики.
</Note>

`changedFiles` и `files` перечисляют то, что команда изменила; остальные поля говорят, насколько полон и надежен этот список.

| Поле           | Тип     | Пример                                                  | Описание                                                                                                                                                                                      |
| :------------- | :------ | :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `changedFiles` | array   | `["/path/to/src/app.ts"]`                               | Абсолютные пути файлов, которые команда изменила, максимум 200. Присутствует, когда `files` содержит diff или `moreFiles` выше нуля                                                           |
| `files`        | array   | `[{"filePath": "/path/to/src/app.ts", "hunks": [...]}]` | Diffs до 5 измененных файлов для отображения. `created` или `deleted` имеет значение `true` для файла, который команда добавила или удалила                                                   |
| `moreFiles`    | number  | `2`                                                     | Количество измененных файлов без diff в `files`                                                                                                                                               |
| `unavailable`  | boolean | `true`                                                  | Установлено, когда diff неполный или не может быть взят                                                                                                                                       |
| `skipped`      | boolean | `true`                                                  | Установлено для команды Git, которая перемещает рабочее дерево, такой как `git checkout` или `git stash`, поэтому Claude Code не берет diff                                                   |
| `shared`       | boolean | `true`                                                  | Установлено, когда другой вызов инструмента Bash, такой как вызов подагента, выполнялся в том же репозитории одновременно, поэтому некоторые перечисленные изменения могут быть этой командой |

<a id="powershell" />

<h5 id="powershell">
  PowerShell
</h5>

Выполняет команды PowerShell. См. [инструмент PowerShell](/docs/ru/tools-reference#powershell-tool) для доступности по платформе.

Поля совпадают с инструментом Bash, со строкой команды в `command`:

| Поле                | Тип     | Пример                     | Описание                                       |
| :------------------ | :------ | :------------------------- | :--------------------------------------------- |
| `command`           | string  | `"Get-ChildItem -Recurse"` | Команда PowerShell для выполнения              |
| `description`       | string  | `"List files recursively"` | Опциональное описание того, что делает команда |
| `timeout`           | number  | `120000`                   | Опциональный тайм-аут в миллисекундах          |
| `run_in_background` | boolean | `false`                    | Выполнять ли команду в фоновом режиме          |

Совпадайте с `Bash|PowerShell` в hooks, которые проверяют команды оболочки, чтобы они охватывали оба инструмента:

* На Windows, везде, где включен инструмент PowerShell, Claude рассматривает PowerShell как основную оболочку и маршрутизирует команды оболочки через него.
* На Windows без Git Bash инструмент включен автоматически и Claude Code не регистрирует инструмент Bash вообще.
* Hook, который совпадает только с `Bash`, никогда не срабатывает там.

<h5 id="write">
  Write
</h5>

Создает или перезаписывает файл.

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

| Поле      | Тип    | Пример           | Описание                                                                    |
| :-------- | :----- | :--------------- | :-------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Шаблон glob для совпадения файлов                                           |
| `path`    | string | `"/path/to/dir"` | Опциональная директория для поиска. По умолчанию текущая рабочая директория |

<h5 id="grep">
  Grep
</h5>

Ищет содержимое файлов с регулярными выражениями.

| Поле          | Тип     | Пример           | Описание                                                                               |
| :------------ | :------ | :--------------- | :------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Шаблон регулярного выражения для поиска                                                |
| `path`        | string  | `"/path/to/dir"` | Опциональный файл или директория для поиска                                            |
| `glob`        | string  | `"*.ts"`         | Опциональный шаблон glob для фильтрации файлов                                         |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` или `"count"`. По умолчанию `"files_with_matches"` |
| `-i`          | boolean | `true`           | Поиск без учета регистра                                                               |
| `multiline`   | boolean | `false`          | Включить многострочное совпадение                                                      |

<h5 id="webfetch">
  WebFetch
</h5>

Получает и обрабатывает веб-контент.

| Поле     | Тип    | Пример                        | Описание                                     |
| :------- | :----- | :---------------------------- | :------------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL для получения контента                   |
| `prompt` | string | `"Extract the API endpoints"` | Подсказка для запуска на полученном контенте |

<h5 id="websearch">
  WebSearch
</h5>

Ищет в веб.

| Поле              | Тип    | Пример                         | Описание                                                |
| :---------------- | :----- | :----------------------------- | :------------------------------------------------------ |
| `query`           | string | `"react hooks best practices"` | Поисковый запрос                                        |
| `allowed_domains` | array  | `["docs.example.com"]`         | Опциональный: включить результаты только с этих доменов |
| `blocked_domains` | array  | `["spam.example.com"]`         | Опциональный: исключить результаты с этих доменов       |

<h5 id="agent">
  Agent
</h5>

Порождает [подагента](/docs/ru/sub-agents).

| Поле            | Тип    | Пример                     | Описание                                                       |
| :-------------- | :----- | :------------------------- | :------------------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | Задача для выполнения агентом                                  |
| `description`   | string | `"Find API endpoints"`     | Краткое описание задачи                                        |
| `subagent_type` | string | `"Explore"`                | Тип специализированного агента для использования               |
| `model`         | string | `"sonnet"`                 | Опциональный псевдоним модели для переопределения стандартного |

Когда вызов Agent переднего плана завершается, ваш [hook PostToolUse](#posttooluse) получает результат подагента и телеметрию запуска в `tool_response`. Прочитайте эти поля для проверки запуска; для сводок токенов и затрат по подагентам используйте [счетчики токенов и затрат](/docs/ru/monitoring-usage#token-counter), отфильтрованные по `query_source` `"subagent"`, так как `totalTokens` и `usage` охватывают только финальный запрос:

| Поле                | Тип    | Пример                                                | Описание                                                                                                                                                                                                                                              |
| :------------------ | :----- | :---------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` для подагентов переднего плана, `"async_launched"` для подагентов фонового плана. Начиная с версии 2.1.198, подагенты выполняются в фоновом режиме по умолчанию, поэтому опущенный `run_in_background` также создает `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Идентификатор для запуска подагента                                                                                                                                                                                                                   |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Финальные текстовые блоки подагента или, для подагента, чей отчет проходит через `SubagentHandback`, краткая заметка об этой передаче на их месте                                                                                                     |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Модель, на которой подагент начал, которая может отличаться от запрошенной модели                                                                                                                                                                     |
| `modelsUsed`        | array  | `["claude-sonnet-4-5", "claude-haiku-4-5"]`           | Модели, используемые по порядку, с последовательными повторениями свернутыми; установлено только, когда модель была переключена во время запуска. Требует Claude Code v2.1.212 или позже                                                              |
| `totalTokens`       | number | `12450`                                               | Количество токенов из финального API запроса подагента: входные, выходные и кэшированные токены в сумме. Это не общее количество по всему запуску                                                                                                     |
| `totalDurationMs`   | number | `48211`                                               | Настоящее время выполнения запуска подагента                                                                                                                                                                                                          |
| `totalToolUseCount` | number | `7`                                                   | Количество вызовов инструментов, которые сделал подагент                                                                                                                                                                                              |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Разбор токенов по типам финального API запроса: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                             |

На Claude Code v2.1.271 или позже подагент, который выполняется с инструментом [`SubagentHandback`](/docs/ru/tools-reference), который Claude Code предоставляет в [режиме auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), доставляет свой отчет через этот инструмент, а не возвращает его как текст. Поле `content` его результата `completed` затем несет краткую заметку об этой передаче, а не сам отчет. Чтобы прочитать отчет, совпадайте с hook `PreToolUse` или `PostToolUse` на `SubagentHandback` и прочитайте `tool_input.message`.

Для подагентов фонового плана инструмент возвращается, когда задача переходит в фоновый режим, поэтому `tool_response` не несет полей использования: фоновый запуск возвращается немедленно, и задача переднего плана, которую Claude Code переводит в фоновый режим во время запуска, возвращается при этом переходе. Он имеет `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` и `resolvedModel`.

На ответе `completed`, `resolvedModel` называет модель, на которой подагент начал, которая может отличаться от значения `model` в `tool_input`, такой как когда `availableModels` или другое переопределение применяется. На ответе `async_launched`, `resolvedModel` называет модель в использовании, когда агент перешел в фоновый режим, поэтому переключение, которое произошло перед переходом в фоновый режим, отражается там. `modelsUsed` и поведение `resolvedModel` во время перехода в фоновый режим требуют Claude Code v2.1.212 или позже.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Задает пользователю один-четыре вопроса с множественным выбором.

| Поле        | Тип    | Пример                                                                                                             | Описание                                                                                                                                                                                                                    |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Вопросы для представления, каждый с строкой `question`, коротким `header`, массивом `options` и опциональным флагом `multiSelect`                                                                                           |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Опциональный. Отображает текст вопроса на выбранный ярлык опции. Ответы с множественным выбором объединяют ярлыки запятыми. Claude не устанавливает это поле; предоставьте его через `updatedInput` для программного ответа |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Представляет план и просит пользователя одобрить его перед тем, как Claude покинет [режим плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode). Claude записывает план в файл на диск перед вызовом инструмента, поэтому буквальный `tool_input` из модели обычно пуст. Claude Code вводит содержимое плана и путь к файлу перед передачей ввода в hooks.

| Поле             | Тип    | Пример                                      | Описание                                                                                                                                                          |
| :--------------- | :----- | :------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Содержимое плана в Markdown. Введено из файла плана на диске                                                                                                      |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Путь к файлу плана. Введено                                                                                                                                       |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Устарело. Claude Code принимает поле, но игнорирует его. До версии 2.1.205 оно несло разрешения на основе подсказок, которые Claude запросил для реализации плана |

В `PostToolUse`, `tool_response` — это объект с полями `plan` и `filePath`, содержащими одобренный план, плюс внутренние флаги статуса. Прочитайте `tool_response.plan` для содержимого плана, а не перечитывайте файл с диска.

<h4 id="pretooluse-decision-control">
  Управление решением PreToolUse
</h4>

Hooks `PreToolUse` могут управлять тем, продолжается ли вызов инструмента. В отличие от других hooks, которые используют поле `decision` верхнего уровня, PreToolUse возвращает свое решение внутри объекта `hookSpecificOutput`. Это дает ему более богатый контроль: четыре результата (разрешить, отказать, спросить или отложить) плюс возможность изменить ввод инструмента перед выполнением.

| Поле                       | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permissionDecision`       | `"allow"` пропускает подсказку разрешения, кроме [действий, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves) и для `AskUserQuestion` и `ExitPlanMode`, которым нужна [`updatedInput`, связанная с ней](#allow-with-updatedinput). `"deny"` предотвращает вызов инструмента. `"ask"` подсказывает пользователю подтвердить. `"defer"` выходит корректно, чтобы инструмент можно было возобновить позже. [Правила отказа и запроса](/docs/ru/permissions#manage-permissions) все еще оцениваются независимо от того, что возвращает hook |
| `permissionDecisionReason` | Для `"allow"` и `"ask"`, показано пользователю, но не Claude. Для `"deny"`, показано Claude. Для `"defer"`, игнорируется                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `updatedInput`             | Изменяет параметры ввода инструмента перед выполнением. Заменяет весь объект ввода, поэтому включите неизменные поля рядом с измененными. Claude Code оценивает правила разрешения и [автоматическую фоновую приемлемость](/docs/ru/tools-reference#background-commands) команды Bash против ввода, который возвращает ваш hook, а не ввода, который отправил Claude. Объедините с `"allow"` для автоматического одобрения или с `"ask"` для показа измененного ввода пользователю. Для `"defer"`, игнорируется                                                                            |
| `additionalContext`        | Строка, добавленная в контекст Claude рядом с результатом инструмента. Игнорируется, когда `permissionDecision` имеет значение `"defer"`. См. [Add context for Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                       |

Когда несколько hooks PreToolUse возвращают разные решения, приоритет — `deny` > `defer` > `ask` > `allow`.

Hook, который блокирует выходом 2, маршрутизируется так же, как `"deny"`: Claude видит сообщение stderr как причину отказа.

Когда hook возвращает `"ask"`, подсказка разрешения, отображаемая пользователю, включает ярлык, определяющий, откуда пришел hook: `[settings]` для hook из любого файла параметров или из frontmatter агента, `[plugin:<name>]` для hook плагина или `[skill]` для hook из frontmatter skill. Это помогает пользователям понять, какой источник конфигурации запрашивает подтверждение.

`"ask"` hook также принуждает подсказку разрешения в [режиме auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode): классификатор все еще может отказать вызову инструмента, но не может одобрить вызов молча. До версии 2.1.211 классификатор мог одобрить команду Bash, выполняющуюся вне [sandbox](/docs/ru/sandboxing), без показа подсказки, которую запросил hook; классификатор все еще применял свои собственные правила безопасности к этой команде, и hook `"deny"` всегда соблюдался.

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

<span id="allow-with-updatedinput" />

В [неинтерактивном режиме](/docs/ru/headless) с флагом `-p` Claude Code предлагает `AskUserQuestion` и `ExitPlanMode` только, когда запуск имеет [хост разрешений](/docs/ru/headless#turn-off-permission-prompts-in-unattended-runs) для получения подсказки, такой как callback `canUseTool` Agent SDK. Эти инструменты требуют взаимодействия с пользователем. Возврат `permissionDecision: "allow"` вместе с `updatedInput` удовлетворяет это требование: hook читает ввод инструмента из stdin, собирает ответ через ваш собственный UI и возвращает его в `updatedInput`, чтобы инструмент выполнялся без подсказки. Возврат только `"allow"` недостаточен для этих инструментов. Для `AskUserQuestion` повторите исходный массив `questions` и добавьте объект [`answers`](#askuserquestion), отображающий текст каждого вопроса на выбранный ответ.

Начиная с версии 2.1.199, инструмент MCP, сервер которого отмечает его с помощью [`_meta["anthropic/requiresUserInteraction"]`](/docs/ru/mcp#require-approval-for-a-specific-tool), более строг: hook не может пропустить его подсказку одобрения с `"allow"`, с `updatedInput` или без него, потому что Claude Code не может подтвердить, что hook собрал взаимодействие, которое требует инструмент.

<Note>
  PreToolUse ранее использовал поля `decision` и `reason` верхнего уровня, но они устарели для этого события. Используйте вместо этого `hookSpecificOutput.permissionDecision` и `hookSpecificOutput.permissionDecisionReason`. Устаревшие значения `"approve"` и `"block"` отображаются на `"allow"` и `"deny"` соответственно. Другие события, такие как PostToolUse и Stop, продолжают использовать `decision` и `reason` верхнего уровня как их текущий формат.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Отложить вызов инструмента на потом
</h4>

`"defer"` предназначен для интеграций, которые запускают `claude -p` как подпроцесс и читают его JSON-вывод, такие как приложение Agent SDK или пользовательский UI, построенный на основе Claude Code. Это позволяет этому вызывающему процессу приостановить Claude при вызове инструмента, собрать ввод через его собственный интерфейс и возобновить, где он остановился. Claude Code соблюдает это значение только в [неинтерактивном режиме](/docs/ru/headless) с флагом `-p`. В интерактивных сеансах он логирует предупреждение и игнорирует результат hook.

Инструмент `AskUserQuestion` — типичный случай: Claude хочет что-то спросить у пользователя, но нет терминала для ответа. Запуск `-p` предлагает `AskUserQuestion` только, когда он имеет [хост разрешений](/docs/ru/headless#turn-off-permission-prompts-in-unattended-runs), такой как инструмент MCP, который вы передаете с `--permission-prompt-tool`, поэтому начните запуск с одного. Круговой путь работает так:

1. Claude вызывает `AskUserQuestion`. Срабатывает hook `PreToolUse`.
2. Hook возвращает `permissionDecision: "defer"`. Инструмент не выполняется. Процесс выходит с `stop_reason: "tool_deferred"` и отложенный вызов инструмента сохраняется в транскрипте.
3. Вызывающий процесс читает `deferred_tool_use` из результата SDK, выводит вопрос в своем собственном UI и ждет ответа.
4. Вызывающий процесс запускает `claude -p --resume <session-id>` с тем же хостом разрешений. Тот же вызов инструмента срабатывает `PreToolUse` снова.
5. Hook возвращает `permissionDecision: "allow"` с ответом в `updatedInput`. Инструмент выполняется и Claude продолжает.

Поле `deferred_tool_use` несет `id`, `name` и `input` инструмента. `input` — это параметры, которые Claude сгенерировал для вызова инструмента, захваченные перед выполнением:

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

Нет тайм-аута или лимита повторных попыток. Сеанс остается на диске до возобновления, подлежит [правилам очистки](/docs/ru/claude-directory#cleaned-up-automatically) [retention sweep](/docs/ru/settings-reference#cleanupperioddays), которая удаляет файлы сеанса через 30 дней по умолчанию. Если ответ не готов при возобновлении, hook может вернуть `"defer"` снова и процесс выходит так же. Вызывающий процесс управляет тем, когда разорвать цикл, в конечном итоге возвращая `"allow"` или `"deny"` из hook.

`"defer"` работает только, когда Claude делает один вызов инструмента в ходе. Если Claude делает несколько вызовов инструментов одновременно, `"defer"` игнорируется с предупреждением и инструмент проходит через нормальный поток разрешений. Ограничение существует, потому что возобновление может повторно запустить только один инструмент: нет способа отложить один вызов из партии без оставления других неразрешенными.

Если отложенный инструмент больше не доступен при возобновлении, процесс выходит с `stop_reason: "tool_deferred_unavailable"` и `is_error: true` перед срабатыванием hook. Это происходит, когда сервер MCP, который предоставил инструмент, не подключен для возобновленного сеанса. Полезная нагрузка `deferred_tool_use` все еще включена, чтобы вы могли определить, какой инструмент исчез.

<Note>
  Чтобы возобновить отложенный сеанс в режиме плана, передайте [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags) вместе с `--resume`, чтобы Claude Code мог представить план для одобрения. Без него Claude Code не восстанавливает режим плана. Требует Claude Code v2.1.246 или позже.

  Когда вы возобновляете с `-p`, Claude Code не восстанавливает никакой другой сохраненный режим разрешений. Он запускает запуск в режиме разрешений, который новый запуск `claude -p` запустил бы, поэтому передайте `--permission-mode` или `--dangerously-skip-permissions` снова, если отложенный сеанс использовал один. Когда вы возобновляете с `claude --resume <session-id>` без `-p`, Claude Code восстанавливает сохраненный режим разрешений, с исключениями, перечисленными в [режим разрешений при возобновлении](/docs/ru/sessions#permission-mode-on-resume).
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Запускается, когда Claude Code собирается попросить вас разрешение на использование инструмента. В сеансах, которые не могут показать подсказку, такие как фоновые подагенты в [неинтерактивном режиме](/docs/ru/headless), Claude Code все еще запускает эти hooks, и если ни один hook не возвращает решение, он отказывает вызову инструмента.
Используйте [управление решением PermissionRequest](#permissionrequest-decision-control) для разрешения или отказа от имени пользователя.

Используйте это событие, когда вам нужен сигнал в момент, когда Claude просит разрешение на использование инструмента. Claude Code запускает hook [Notification](#notification) с типом `permission_prompt` только после того, как подсказка ждала около шести секунд.

Claude Code не запускает hooks PermissionRequest для [сетевого запроса](/docs/ru/sandboxing#network-isolation) изолированной команды. Чтобы получить сигнал для этой подсказки, используйте тип уведомления `permission_prompt`.

Совпадает с названием инструмента, те же значения, что и PreToolUse.

<h4 id="permissionrequest-input">
  Ввод PermissionRequest
</h4>

Hooks PermissionRequest получают поля `tool_name` и `tool_input`, как hooks PreToolUse, но без `tool_use_id`. Для инструмента MCP они также получают объект [`mcp_server`](#pretooluse-input). Опциональный массив `permission_suggestions` содержит [обновления разрешений](#permission-update-entries), которые Claude Code предлагает для этого запроса, такие как добавление правила разрешения или изменение режима разрешений.

Массив `permission_suggestions` не является точным списком опций, которые вы видите, потому что каждый диалог разрешений строит свои собственные опции. Некоторые диалоги, такие как диалог для редактирования файлов, вообще не читают массив и получают свои опции из самого запроса. Диалог, который читает его, все еще может скрыть опцию, чье предложение остается в массиве, например, когда [`allowManagedPermissionRulesOnly`](/docs/ru/settings-reference#allowmanagedpermissionrulesonly) скрывает опции сохранения правил. Он также может предложить опции, которые не имеют записи предложения, такие как [**Yes, and switch to auto mode**](/docs/ru/permission-modes#switch-permission-modes), которая изменяет режим разрешений напрямую, а не через обновление разрешений.

Hooks PreToolUse запускаются перед каждым вызовом инструмента, независимо от того, нужно ли ему разрешение. Hooks PermissionRequest запускаются только, когда Claude Code собирается попросить вас разрешение, или когда он в противном случае автоматически отказал бы вызову, который не может подсказать. Ни одно событие не срабатывает для [`EndConversation`](/docs/ru/tools-reference#endconversation-tool-behavior).

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
  Управление решением PermissionRequest
</h4>

Hooks `PermissionRequest` могут разрешить или отказать запросы разрешений. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, ваш скрипт hook может вернуть объект `decision` с этими полями, специфичными для события:

| Поле                 | Описание                                                                                                                                                                                                                                |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` предоставляет разрешение, `"deny"` отказывает его. [Правила отказа и запроса](/docs/ru/permissions#manage-permissions) все еще оцениваются, поэтому hook, возвращающий `"allow"`, не переопределяет соответствующее правило отказа |
| `updatedInput`       | Для `"allow"` только: изменяет параметры ввода инструмента перед выполнением. Заменяет весь объект ввода, поэтому включите неизменные поля рядом с измененными. Измененный ввод повторно оценивается против правил отказа и запроса     |
| `updatedPermissions` | Для `"allow"` только: массив [записей обновления разрешений](#permission-update-entries) для применения, такие как добавление правила разрешения или изменение режима разрешений сеанса                                                 |
| `message`            | Для `"deny"` только: говорит Claude, почему разрешение было отказано                                                                                                                                                                    |
| `interrupt`          | Для `"deny"` только: если `true`, останавливает Claude                                                                                                                                                                                  |

Hook, который выходит 2 без объекта `decision`, оставляет поток разрешений неизменным, и его stderr отбрасывается. Только объект `decision` может предоставить или отказать в запросе.

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
  Записи обновления разрешений
</h4>

Поле вывода `updatedPermissions` и поле ввода [`permission_suggestions`](#permissionrequest-input) оба используют один и тот же массив объектов записей. Каждая запись имеет `type`, который определяет ее другие поля, и `destination`, который управляет тем, где записывается изменение.

| `type`              | Поля                               | Эффект                                                                                                                                                                                                                    |
| :------------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `addRules`          | `rules`, `behavior`, `destination` | Добавляет правила разрешений. `rules` — это массив объектов `{toolName, ruleContent?}`. Опустите `ruleContent` для совпадения со всем инструментом. `behavior` — это `"allow"`, `"deny"` или `"ask"`                      |
| `replaceRules`      | `rules`, `behavior`, `destination` | Заменяет все правила данного `behavior` в `destination` предоставленными `rules`                                                                                                                                          |
| `removeRules`       | `rules`, `behavior`, `destination` | Удаляет соответствующие правила данного `behavior`                                                                                                                                                                        |
| `setMode`           | `mode`, `destination`              | Изменяет режим разрешений. Допустимые режимы — `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` и `manual` как псевдоним для `default`. Псевдоним `manual` требует Claude Code v2.1.200 или позже |
| `addDirectories`    | `directories`, `destination`       | Добавляет рабочие директории. `directories` — это массив строк путей                                                                                                                                                      |
| `removeDirectories` | `directories`, `destination`       | Удаляет рабочие директории                                                                                                                                                                                                |

<Note>
  `setMode` с `bypassPermissions` вступает в силу только, если вы запустили сеанс с режимом обхода, уже доступным: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` или `permissions.defaultMode: "bypassPermissions"` в [пользовательских, `--settings` или управляемых параметрах](/docs/ru/settings-reference#permissions-defaultmode). В противном случае обновление — это no-op. Обновление также является no-op, когда [`permissions.disableBypassPermissionsMode`](/docs/ru/permissions#managed-settings) отключает режим или когда сеанс запускается в [restricted mode](/docs/ru/cli-reference#cli-flags).

  `bypassPermissions` никогда не сохраняется как `defaultMode` независимо от `destination`.
</Note>

Поле `destination` на каждой записи определяет, остается ли изменение в памяти или сохраняется в файл параметров.

| `destination`     | Записывает в                                         |
| :---------------- | :--------------------------------------------------- |
| `session`         | только в памяти, отбрасывается при завершении сеанса |
| `localSettings`   | `.claude/settings.local.json`                        |
| `projectSettings` | `.claude/settings.json`                              |
| `userSettings`    | `~/.claude/settings.json`                            |

Hook может повторить одно из `permission_suggestions`, которые он получил, как свой собственный вывод `updatedPermissions`.

<h3 id="posttooluse">
  PostToolUse
</h3>

Запускается сразу после успешного завершения инструмента.

Совпадает с названием инструмента, те же значения, что и PreToolUse.

Совпадайте более широко, когда название инструмента не является правильным фильтром:

* Чтобы запустить hook после завершения любого инструмента успешно, опустите `matcher` или установите его на `"*"`. Ваш hook может затем обнаружить, что изменилось сам, например, запустив `git status --porcelain`, который также перечисляет неотслеживаемые файлы, которые `git diff` пропускает. Для вызовов инструментов, которые не удаются, добавьте тот же hook под [PostToolUseFailure](#posttoolusefailure).
* Чтобы запустить hook, когда определенный файл изменяется на диске, независимо от того, что его написало, используйте [FileChanged](#filechanged). Claude Code не запускает hook `PostToolUse`, соответствующий `Edit|Write`, когда команда `Bash` или процесс вне Claude Code переписывает тот же файл.

<h4 id="posttooluse-input">
  Ввод PostToolUse
</h4>

Hooks `PostToolUse` срабатывают после того, как инструмент уже выполнился успешно. Ввод включает как `tool_input`, аргументы, отправленные инструменту, так и `tool_response`, результат, который он вернул. Точная схема для обоих зависит от инструмента. Пути `tool_input` инструментов файлов прибывают в том же формате, что и для [PreToolUse](#pretooluse-input): всегда абсолютные, с собственными разделителями платформы, поэтому обратные косые черты на Windows. Для инструмента MCP ввод также несет объект [`mcp_server`](#pretooluse-input).

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
    "type": "create"
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Поле          | Описание                                                                                                                            |
| :------------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Опциональный. Время выполнения инструмента в миллисекундах. Исключает время, потраченное на подсказки разрешений и hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  Управление решением PostToolUse
</h4>

Hooks `PostToolUse` могут предоставить обратную связь Claude после выполнения инструмента. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                   | Описание                                                                                                                                                                                                                                                                                          |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`             | `"block"` добавляет `reason` рядом с результатом инструмента. Claude все еще видит исходный вывод; чтобы заменить его, используйте `updatedToolOutput`                                                                                                                                            |
| `reason`               | Объяснение, показанное Claude, когда `decision` имеет значение `"block"`                                                                                                                                                                                                                          |
| `additionalContext`    | Строка, добавленная в контекст Claude рядом с результатом инструмента. См. [Add context for Claude](#add-context-for-claude)                                                                                                                                                                      |
| `classifierContext`    | Краткая заметка об этом результате вызова для [классификатора режима auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), а не для Claude. См. [Annotate a result for the auto mode classifier](#annotate-a-result-for-the-auto-mode-classifier). Требует Claude Code v2.1.236 или позже |
| `updatedToolOutput`    | Заменяет вывод инструмента предоставленным значением перед отправкой Claude. Значение должно совпадать с формой вывода инструмента                                                                                                                                                                |
| `updatedMCPToolOutput` | Заменяет вывод для [инструментов MCP](#match-mcp-tools) только. Предпочитайте `updatedToolOutput`, который работает для всех инструментов                                                                                                                                                         |

Пример ниже заменяет вывод вызова `Bash`. Значение замены совпадает с формой вывода инструмента `Bash`:

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
  `updatedToolOutput` только изменяет то, что видит Claude. Инструмент уже выполнился к моменту срабатывания hook, поэтому любые написанные файлы, выполненные команды или отправленные сетевые запросы уже вступили в силу. Телеметрия, такая как spans инструментов OpenTelemetry и события аналитики, также захватывает исходный вывод перед выполнением hook. Чтобы предотвратить или изменить вызов инструмента перед его выполнением, используйте вместо этого hook [PreToolUse](#pretooluse).

  Значение замены должно совпадать с формой вывода инструмента. Встроенные инструменты возвращают структурированные объекты, а не простые строки. Например, `Bash` возвращает объект с полями `stdout`, `stderr`, `interrupted` и `isImage`. Для встроенных инструментов значение, которое не совпадает со схемой вывода инструмента, игнорируется и используется исходный вывод. Вывод инструмента MCP передается без проверки схемы. Удаление деталей ошибок, которые нужны Claude, может привести к тому, что он продолжит с ложным предположением.
</Warning>

<h4 id="annotate-a-result-for-the-auto-mode-classifier">
  Аннотирование результата для классификатора режима auto
</h4>

Верните `classifierContext` для отправки краткой заметки об этом результате вызова инструмента [классификатору режима auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), а не Claude. Классификатор [никогда не получает сами результаты инструментов](/docs/ru/permission-modes#how-the-classifier-evaluates-actions), поэтому это поле — поддерживаемый способ рассказать ему что-то о том, что вернул вызов, перед тем как он проверит более поздние действия. Поле требует Claude Code v2.1.236 или позже.

Пример ниже говорит классификатору, откуда пришел вывод запроса:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "classifierContext": "This query ran against the staging database, not production."
  }
}
```

Сколько веса классификатор дает заметке, зависит от того, где вы настроили hook:

* **Hooks, настроенные в Claude Code**: для hooks из файлов параметров, плагинов, skills и frontmatter агента классификатор рассматривает заметку как непроверенный, предоставленный приложением контекст. Заметка никогда не устанавливает намерение пользователя, и если она утверждает, что вы одобрили или запросили что-то, классификатор проверяет это утверждение против ваших собственных сообщений в разговоре
* **In-process callbacks Agent SDK**: когда приложение, встраивающее Claude Code, регистрирует hook как [callback TypeScript SDK](/docs/ru/agent-sdk/hooks) и возвращает заметку во время живого сеанса, классификатор может взвесить утверждение пользователя, переданное в заметке, как намерение пользователя. Такое утверждение может удовлетворить требование согласия, которое классификатор принял бы из сообщения, которое вы отправляете, но оно никогда не снимает блокировку, которую ваше собственное сообщение не могло бы снять. После возобновления сеанса Claude Code рассматривает восстановленные заметки как непроверенный контекст. Когда hooks из обеих групп аннотируют один и тот же вызов, классификатор рассматривает объединенную заметку как непроверенный контекст

Claude Code применяет эти ограничения при доставке заметки:

* **Длина**: Claude Code ограничивает заметки для одного вызова инструмента 2000 символами и усекает остальное. Ограничение делится между каждым hook, который отвечает на этот вызов
* **Только синхронные ответы**: Claude Code игнорирует поле в ответе hook, который [выполняется в фоновом режиме](#run-hooks-in-the-background), потому что этот ответ прибывает после того, как Claude Code записывает результат инструмента
* **Вызовы, которые классификатор не записывает**: транскрипт классификатора опускает поиски только для чтения, такие как чтение файлов и поиски. Claude Code отбрасывает заметку, прикрепленную к одному из этих вызовов
* **Взаимодействие с переписыванием**: когда заметка описывает вывод, который вы заменяете с помощью `updatedToolOutput`, верните оба поля в одном ответе hook. Claude Code отбрасывает заметку, если это переписывание отклонено или переписывание другого hook заменяет его. Claude Code доставляет заметку, которую вы возвращаете без переписывания, даже когда другой hook переписывает вывод

<Warning>
  Классификатор читает содержимое, которое вы помещаете в `classifierContext`, как информацию от приложения, размещающего сеанс, поэтому не копируйте в него ненадежный вывод инструмента или текст третьих сторон. Держите заметку к краткому утверждению об этом одном вызове, такому как факт о его происхождении или утверждение пользователя о нем; не используйте поле для доставки несвязанных сообщений или потока событий.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Запускается, когда инструмент, который начал выполняться, не удается: инструмент выбросил ошибку или инструмент MCP вернул результат ошибки. Используйте это для логирования сбоев, отправки оповещений или предоставления исправляющей обратной связи Claude.

Совпадает с названием инструмента, те же значения, что и PreToolUse.

<Note>
  Это событие не срабатывает для вызовов инструментов, отклоненных перед выполнением: неизвестное название инструмента, ввод, который не проходит проверку схемы или инструмента, или отказ в разрешении. Отказы в проверке возвращаются как результаты `tool_use_error` и происходят перед выполнением hooks, поэтому они не срабатывают ни `PreToolUse`, ни `PostToolUseFailure`. Отказы в разрешении срабатывают `PreToolUse`, но не это событие; см. [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Ввод PostToolUseFailure
</h4>

Hooks PostToolUseFailure получают те же поля `tool_name` и `tool_input`, что и PostToolUse, вместе с информацией об ошибке как полями верхнего уровня. Для инструмента MCP они также получают объект [`mcp_server`](#pretooluse-input). Например, неудачная команда `npm test` может доставить:

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
  "error": "Exit code 1\nError: Cannot find module 'express'",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Поле           | Описание                                                                                                                                                                                                                                                     |
| :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Строка, описывающая, что пошло не так. Формат зависит от инструмента, который не удался                                                                                                                                                                      |
| `is_interrupt` | Опциональное логическое значение. True, когда сбой достиг Claude Code как прерывание, а не как ошибка, которую сообщил инструмент. Отмена выполняющегося инструмента не срабатывает этот hook; результат инструмента несет сообщение прерывания вместо этого |
| `duration_ms`  | Опциональный. Время выполнения инструмента в миллисекундах. Исключает время, потраченное на подсказки разрешений и hooks PreToolUse                                                                                                                          |

Строка `error` обычно является тем же текстом, который Claude получает как результат неудачного инструмента. Его формат варьируется в зависимости от инструмента и сбоя. Ключ вашего hook на `tool_name`, `is_interrupt` и первой строке `Exit code N`; рассматривайте остальную строку как текст отображения, а не стабильный формат.

* Для Bash и PowerShell команда, которая выполнилась и вышла, создает первую строку `Exit code N`, затем любой вывод, который команда создала, как один блок с stdout и stderr перемешанными
* Полезная нагрузка также может нести сообщение об ошибке без строки кода выхода, когда Claude Code не мог запустить сам процесс оболочки
* Claude Code усекает длинные строки в середине вокруг маркера `... [N characters truncated] ...` и может вставлять свои собственные строки, такие как `Command timed out after 2m 0s`

<h4 id="posttoolusefailure-decision-control">
  Управление решением PostToolUseFailure
</h4>

Hooks `PostToolUseFailure` могут предоставить контекст Claude после сбоя инструмента. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                | Описание                                                                                                     |
| :------------------ | :----------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Строка, добавленная в контекст Claude рядом с ошибкой. См. [Add context for Claude](#add-context-for-claude) |

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

Запускается один раз после того, как каждый вызов инструмента в партии разрешится, перед тем как Claude Code отправит следующий запрос модели. `PostToolUse` срабатывает один раз для каждого инструмента, что означает, что он срабатывает одновременно, когда Claude делает параллельные вызовы инструментов. `PostToolBatch` срабатывает ровно один раз со всей партией, поэтому это правильное место для внедрения контекста, который зависит от набора инструментов, которые выполнились, а не от любого одного инструмента. Нет matcher для этого события.

<h4 id="posttoolbatch-input">
  Ввод PostToolBatch
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks PostToolBatch получают `tool_calls`, массив, описывающий каждый вызов инструмента в партии:

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

`tool_response` содержит то же содержимое, которое модель получает в соответствующем блоке `tool_result`. Значение — это сериализованная строка или массив блоков контента, ровно как инструмент выдал его. Для `Read` это означает текст с префиксом номера строки, а не необработанное содержимое файла. Ответы могут быть большими, поэтому анализируйте только нужные вам поля.

<Note>
  Форма `tool_response` отличается от `PostToolUse`. `PostToolUse` передает структурированный объект `Output` инструмента, такой как `{filePath: "...", type: "create"}` для `Write`; `PostToolBatch` передает сериализованное содержимое `tool_result`, которое видит модель.
</Note>

<h4 id="posttoolbatch-decision-control">
  Управление решением PostToolBatch
</h4>

Hooks `PostToolBatch` могут внедрить контекст для Claude. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                | Описание                                                                                                                                                                                                                        |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | Строка контекста, внедренная один раз перед следующим вызовом модели. См. [Add context for Claude](#add-context-for-claude) для деталей доставки, что в нее поместить и как возобновленные сеансы обрабатывают прошлые значения |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Возврат `decision: "block"` или `continue: false` останавливает агентский цикл перед следующим вызовом модели. Сообщение блокировки поступает из JSON `reason` или `stopReason` или из stderr при выходе 2. Вы видите его как предупреждение в транскрипте, и оно остается в разговоре, поэтому Claude видит его при продолжении разговора.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Запускается, когда [режим auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) отказывает вызову инструмента, включая когда он отказывает без вердикта классификатора, потому что [проверка безопасности, отдельная от режима auto, отказала в запросе классификатора](/docs/ru/errors#auto-mode-cannot-determine-the-safety-of-an-action) или его ответ не был проанализирован. Этот hook срабатывает только в режиме auto: он не запускается, когда вы вручную отказываете диалогу разрешений, когда hook `PreToolUse` блокирует вызов или когда совпадает правило `deny`. Используйте его для логирования отказов, настройки конфигурации или сообщения модели, что она может повторить попытку вызова инструмента.

Совпадает с названием инструмента, те же значения, что и PreToolUse.

<h4 id="permissiondenied-input">
  Ввод PermissionDenied
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks PermissionDenied получают `tool_name`, `tool_input`, `tool_use_id` и `reason`. Для инструмента MCP они также получают объект [`mcp_server`](#pretooluse-input).

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
  "reason": "[Irreversible Local Destruction]"
}
```

| Поле     | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reason` | Причина отказа. Для вердикта классификатора в большинстве сеансов он называет совпадающее правило в квадратных скобках, такое как `[Data Exfiltration]`; см. [Review denials](/docs/ru/auto-mode-config#review-denials) для других форм. Для [отказа без вердикта](#permissiondenied-decision-control) он начинается с `Auto mode could not evaluate this action and is blocking it for safety`. Для отказа, потому что модель классификатора была недоступна, это фиксированный текст `Classifier unavailable` |

<h4 id="permissiondenied-decision-control">
  Управление решением PermissionDenied
</h4>

Hooks PermissionDenied могут сказать модели, что она может повторить попытку отклоненного вызова инструмента. Верните объект JSON с `hookSpecificOutput.retry`, установленным на `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Когда `retry` имеет значение `true`, Claude Code добавляет сообщение в разговор, говорящее модели, что она может повторить попытку вызова инструмента. Claude Code не отменяет сам отказ. Если ваш hook не возвращает JSON или возвращает `retry: false`, отказ остается и модель получает исходное сообщение отказа.

Claude Code игнорирует `retry: true`, когда классификатор создал [отсутствие вердикта на действие](/docs/ru/errors#auto-mode-cannot-determine-the-safety-of-an-action): его ответ не был проанализирован или проверка безопасности, отдельная от режима auto, отказала в запросе классификатора. Для этих отказов Claude Code уже говорит модели в сообщении отказа, повторить ли попытку позже или продолжить.

<h3 id="notification">
  Notification
</h3>

Запускается, когда Claude Code отправляет уведомления. Совпадает с типом уведомления. Опустите matcher для запуска hooks для всех типов уведомлений.

Вы получаете эти события hook даже с отключенными уведомлениями рабочего стола: параметр `preferredNotifChannel`, включая `notifications_disabled`, изменяет только то, как вас оповещают, а не запускается ли ваш hook.

| Matcher                      | Когда срабатывает                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude нуждается в вашем разрешении на использование инструмента или [сетевого запроса](/docs/ru/sandboxing#network-isolation) изолированной команды, и подсказка ждала около шести секунд                                                                                                                                                                                                                                                                                                                                    |
| `idle_prompt`                | Claude закончил отвечать около 60 секунд назад и вы не печатали с тех пор                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `auth_success`               | Аутентификация завершена                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `elicitation_dialog`         | Сервер MCP открывает форму запроса и вы не печатали около шести секунд                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `elicitation_url_dialog`     | Сервер MCP просит вас открыть URL браузера и вы не печатали около шести секунд                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `elicitation_complete`       | Сервер MCP сообщает, что [URL-режим запроса](#elicitation-input) завершен                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `elicitation_response`       | Ответ запроса MCP отправляется обратно на сервер                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agent_needs_input`          | Фоновый сеанс начинает ждать вашего ввода, пока [agent view](/docs/ru/agent-view) открыт в терминале, или текущий сеанс задает вам вопрос [настройки терминала товарища команды агентов](/docs/ru/agent-teams#choose-a-display-mode) и вы не печатали около шести секунд                                                                                                                                                                                                                                                           |
| `agent_completed`            | Фоновый сеанс завершается или не удается. Срабатывает только, пока [agent view](/docs/ru/agent-view) открыт в терминале                                                                                                                                                                                                                                                                                                                                                                                                       |
| `quota_auto_resume_fired`    | Claude Code продолжает вашу задачу после того, как лимит использования claude.ai приостановил его: при сбросе или раньше, когда что-то, что вы делаете в Claude Code во время ожидания, такое как добавление кредитов использования, обновление вашего плана или переключение моделей, снова делает использование доступным, с [исключением параметра модели](/docs/ru/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                      |
| `quota_auto_resume_stale`    | Лимит использования claude.ai сбросился, пока ваш компьютер спал более чем около 30 минут. Claude Code ждет, пока вы нажмете `Enter`, вместо продолжения. После более короткого сна он продолжает и срабатывает `quota_auto_resume_fired` вместо этого                                                                                                                                                                                                                                                                   |
| `quota_auto_resume_disabled` | Claude Code заканчивает свое ожидание лимита использования claude.ai без продолжения вашей задачи: [`autoContinueAtUsageLimit`](/docs/ru/settings-reference#autocontinueatusagelimit) отключен или сброс переместился более чем на 24 часа во время ожидания, которое Claude Code запустил самостоятельно, продолженная задача продолжала попадать на лимит или продолжение было заблокировано перед достижением модели. Не срабатывает, когда вы нажимаете `Esc` или `Ctrl+C` или выбираете **Don't continue automatically** |

Типы `agent_needs_input` и `agent_completed` требуют Claude Code v2.1.198 или позже.

Типы `quota_auto_resume_fired`, `quota_auto_resume_stale` и `quota_auto_resume_disabled` требуют Claude Code v2.1.234 или позже.

В сеансах терминала `permission_prompt` для [сетевого запроса](/docs/ru/sandboxing#network-isolation) изолированной команды требует Claude Code v2.1.246 или позже.

`agent_needs_input` для вопроса настройки терминала товарища требует Claude Code v2.1.248 или позже.

<Note>
  Типы `permission_prompt`, `idle_prompt`, `elicitation_dialog` и `elicitation_url_dialog` делят свое время с уведомлениями рабочего стола, поэтому в сеансах терминала вы видите их только, когда вы кажетесь отсутствующим от терминала:

  * Ожидайте `permission_prompt` один раз, когда вы не печатали около шести секунд. Таймер начинается, когда появляется подсказка разрешения, и каждый нажатие клавиши откладывает его. Чтобы запустить hook немедленно, когда Claude просит разрешение на использование инструмента, используйте вместо этого [PermissionRequest](#permissionrequest).
  * Ожидайте `idle_prompt` около 60 секунд после того, как Claude закончит отвечать, и только если вы не печатали с тех пор. Claude Code не отправляет `idle_prompt`, пока ждет сброса лимита использования claude.ai. Когда ожидание заканчивается самостоятельно, один из типов `quota_auto_resume_*` срабатывает вместо этого.
  * Ожидайте `elicitation_dialog` для формы запроса или `elicitation_url_dialog` для запроса URL браузера один раз, когда вы не печатали около шести секунд. Оба делят один и тот же шестисекундный шлюз как `permission_prompt`: таймер начинается, когда появляется диалог, и каждый нажатие клавиши откладывает его.

  Запрос разрешения или запрос, который прибывает, пока другой диалог находится на экране, сохраняет один и тот же шестисекундный шлюз, рассчитанный с момента прибытия запроса. Его уведомление может достичь вас, пока запрос все еще ждет позади открытого диалога.
</Note>

Claude Code рассчитывает `permission_prompt` по-другому в сеансах, где он отправляет запросы разрешений на callback [`canUseTool`](/docs/ru/agent-sdk/user-input) Agent SDK, что является тем, как Claude Desktop и расширение VS Code размещают Claude Code:

* Ожидайте `permission_prompt` около шести секунд после того, как Claude просит разрешение. Claude Code не откладывает его, пока вы печатаете.
* Если вы или hook [PermissionRequest](#permissionrequest) ответите раньше, Claude Code не запускает `permission_prompt`.
* Установите [`CLAUDE_CODE_DISABLE_PERMISSION_PROMPT_NOTIFY_HOOKS`](/docs/ru/env-vars) на `1`, чтобы отключить `permission_prompt` в этих сеансах.

До версии 2.1.233 `permission_prompt` не срабатывал в этих сеансах.

Используйте отдельные matchers для запуска разных обработчиков в зависимости от типа уведомления. Эта конфигурация запускает скрипт оповещения, специфичный для разрешения, когда Claude нуждается в одобрении разрешения, и другое уведомление, когда Claude был неактивен:

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
  Ввод Notification
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks Notification получают `message` с текстом уведомления, опциональный `title` и `notification_type`, указывающий, какой тип срабатывает.

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

Hooks Notification не могут блокировать или изменять уведомления. Claude Code отбрасывает их поля `systemMessage` и `continue`, но все еще выдает [`terminalSequence`](#emit-terminal-notifications), на которую полагается пример уведомления рабочего стола. Hooks Notification предназначены для побочных эффектов, таких как пересылка уведомления на внешний сервис.

<h3 id="subagentstart">
  SubagentStart
</h3>

Запускается, когда Claude порождает подагента с инструментом Agent, когда Claude [возобновляет подагента](/docs/ru/sub-agents#resume-subagents) и каждый раз, когда товарищ [команды агентов](/docs/ru/agent-teams) в процессе обрабатывает новое сообщение. Поддерживает matchers для фильтрации по названию типа агента. Для встроенных агентов это имя агента, такое как `general-purpose`, `Explore` или `Plan`. Для [пользовательских подагентов](/docs/ru/sub-agents) это поле `name` из frontmatter агента, а не имя файла.

Для подагентов, поставляемых [плагином](/docs/ru/plugins/overview), тип агента — это идентификатор с областью плагина, такой как `my-plugin:reviewer`, а не голое имя frontmatter.  Двоеточие помещает имя с областью плагина на путь регулярного выражения, поэтому якорьте matcher с `^` и `$` для точного совпадения: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  Ввод SubagentStart
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks SubagentStart получают `agent_id` с уникальным идентификатором подагента и `agent_type` с названием агента, который matcher фильтрует.

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

Hooks SubagentStart не могут блокировать создание подагента, но они могут внедрить контекст в подагента. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, вы можете вернуть:

| Поле                | Описание                                                                                                                                            |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Строка, добавленная в контекст подагента в начале его разговора, перед его первой подсказкой. См. [Add context for Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

Когда hook запускается снова для того же подагента, Claude Code внедряет возвращенный контекст только, когда контекст подагента уже не содержит копию из более раннего запуска. Копия, внедренная при запуске, остается на месте, оставляя [кэш подсказок](/docs/ru/prompt-caching#subagents-and-the-cache) подагента нетронутым. После [автоматического сжатия](/docs/ru/sub-agents#auto-compaction) отбрасывает эту копию, Claude Code внедряет контекст следующего запуска снова.

<h3 id="subagentstop">
  SubagentStop
</h3>

Запускается, когда подагент Claude Code закончил отвечать. Совпадает с типом агента, те же значения, что и SubagentStart.

<h4 id="subagentstop-input">
  Ввод SubagentStop
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks SubagentStop получают `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` и `last_assistant_message`. Поле `agent_type` — это значение, используемое для фильтрации matcher. `transcript_path` — это транскрипт основного сеанса, пока `agent_transcript_path` — это собственный транскрипт подагента, хранящийся в вложенной папке `subagents/`. Поле `last_assistant_message` содержит текстовое содержимое финального ответа подагента, поэтому hooks могут получить доступ к нему без анализа файла транскрипта.

Не каждое событие SubagentStop поступает от подагента, который Claude порождает. Claude Code также запускает внутренних агентов для некоторых своих собственных функций, таких как [предложения подсказок](/docs/ru/interactive-mode#prompt-suggestions) и [`/btw` побочные вопросы](/docs/ru/interactive-mode#side-questions-with-%2Fbtw), и SubagentStop срабатывает, когда один из них завершается. Для этих событий `agent_type` — это имя агента, который запускает сам сеанс, такой как один, установленный с [`--agent`](/docs/ru/cli-reference#cli-flags) или параметром [`agent`](/docs/ru/settings-reference#agent), и пустая строка, когда сеанс запускается без одного.

`matcher`, который называет типы агентов, не совпадает с пустым `agent_type`. Hook, чей matcher опущен, `""` или `"*"`, или является регулярным выражением, которое совпадает с пустой строкой, запускается для событий с пустым `agent_type` тоже.

На Claude Code v2.1.271 или позже подагент, который выполняется с инструментом [`SubagentHandback`](/docs/ru/tools-reference), доставляет свой отчет через этот инструмент перед остановкой. Поле `last_assistant_message` затем содержит закрывающий текст подагента, если он есть, который не является доставленным отчетом. Отчет — это ввод `message` этого вызова, который hook `PreToolUse` или `PostToolUse`, соответствующий `SubagentHandback`, получает как `tool_input.message`.

Hooks SubagentStop также получают массивы `background_tasks` и `session_crons`, описанные в [Stop input](#stop-input). Оба массива ограничены родительским сеансом, а не подагентом.

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

Hooks SubagentStop используют тот же формат управления решением, что и [hooks Stop](#stop-decision-control), включая `hookSpecificOutput.additionalContext` с `hookEventName`, установленным на `"SubagentStop"`, для обратной связи без ошибок, которая держит подагента работающим. Возврат `decision: "block"` с `reason` держит подагента работающим и доставляет `reason` подагенту как его следующую инструкцию. Hook, который блокирует выходом 2, доставляет его сообщение stderr так же. Чтобы внедрить контекст в родительский сеанс после возврата подагента, используйте вместо этого hook [`PostToolUse`](#posttooluse) на инструменте `Agent`.

<h3 id="taskcreated">
  TaskCreated
</h3>

Запускается, когда задача создается через инструмент `TaskCreate`. Используйте это для применения соглашений об именовании, требования описаний задач или предотвращения создания определенных задач. В [сеансе без инструментов Task](/docs/ru/tools-reference#task-tool-availability) это событие не срабатывает.

Hooks TaskCreated не поддерживают matchers и срабатывают при каждом возникновении.

<h4 id="taskcreated-input">
  Ввод TaskCreated
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks TaskCreated получают `task_id`, `task_subject` и опционально `task_description`, `teammate_name` и `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Поле               | Описание                                                                     |
| :----------------- | :--------------------------------------------------------------------------- |
| `task_id`          | Идентификатор создаваемой задачи                                             |
| `task_subject`     | Название задачи                                                              |
| `task_description` | Подробное описание задачи. Может отсутствовать                               |
| `teammate_name`    | Имя товарища, создающего задачу. Может отсутствовать                         |
| `team_name`        | Устарело. Имя команды, полученное из сеанса; будет удалено в будущем выпуске |

<h4 id="taskcreated-decision-control">
  Управление решением TaskCreated
</h4>

Hook TaskCreated может заблокировать создание двумя способами. В любом случае Claude Code удаляет задачу и возвращает ваше сообщение Claude как ошибку инструмента. Claude Code игнорирует `continue: false` из этого события и Claude продолжает работать.

* **Код выхода 2**: Claude Code возвращает текст stderr как сообщение.
* **JSON `{"decision": "block", "reason": "..."}`**: Claude Code возвращает `reason` как сообщение.

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

Запускается, когда задача отмечается как завершенная. Это срабатывает в двух ситуациях: когда любой агент явно отмечает задачу как завершенную через инструмент TaskUpdate или когда товарищ [команды агентов](/docs/ru/agent-teams) завершает свой ход с выполняющимися задачами. Используйте это для применения критериев завершения, таких как прохождение тестов или проверок lint, перед закрытием задачи.

Hooks TaskCompleted не поддерживают matchers и срабатывают при каждом возникновении.

<h4 id="taskcompleted-input">
  Ввод TaskCompleted
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks TaskCompleted получают `task_id`, `task_subject` и опционально `task_description`, `teammate_name` и `team_name`.

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

| Поле               | Описание                                                                     |
| :----------------- | :--------------------------------------------------------------------------- |
| `task_id`          | Идентификатор завершаемой задачи                                             |
| `task_subject`     | Название задачи                                                              |
| `task_description` | Подробное описание задачи. Может отсутствовать                               |
| `teammate_name`    | Имя товарища, завершающего задачу. Может отсутствовать                       |
| `team_name`        | Устарело. Имя команды, полученное из сеанса; будет удалено в будущем выпуске |

<h4 id="taskcompleted-decision-control">
  Управление решением TaskCompleted
</h4>

Hooks TaskCompleted поддерживают два способа управления завершением задачи:

* **Код выхода 2**: задача не отмечается как завершенная и сообщение stderr передается обратно модели как обратная связь.
* **JSON `{"continue": false, "stopReason": "..."}`**: когда товарищ, завершающий свой ход, запустил событие, полностью останавливает товарища, совпадая с поведением hook `Stop`. `stopReason` показывается пользователю. Когда инструмент `TaskUpdate` запустил событие, Claude Code игнорирует `continue: false`; код выхода 2 все еще блокирует завершение.

Этот пример запускает тесты и блокирует завершение задачи, если они не пройдут:

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

Запускается, когда основной агент Claude Code закончил отвечать. Не запускается, если остановка произошла из-за прерывания пользователем. Ошибки API срабатывают вместо этого [StopFailure](#stopfailure).

<Tip>
  Команда [`/goal`](/docs/ru/goal) — это встроенный ярлык для hook Stop с областью сеанса на основе подсказки. Используйте его, когда вы хотите, чтобы Claude продолжал работать над условием без написания конфигурации hook.
</Tip>

<h4 id="stop-input">
  Ввод Stop
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks Stop получают `stop_hook_active`, `last_assistant_message`, `background_tasks` и `session_crons`. Поле `stop_hook_active` имеет значение `true`, когда Claude Code уже продолжает в результате hook stop. Проверьте это значение или обработайте транскрипт, чтобы избежать блокировки на условии, которое никогда не разрешится. Claude Code переопределяет hook и заканчивает ход после 8 последовательных блокировок.

Поле `last_assistant_message` содержит текстовое содержимое финального ответа Claude, поэтому hooks могут получить доступ к нему без анализа файла транскрипта. Для hooks, которые действуют на только что завершенный ход, такие как hooks чтения вслух или уведомления, используйте это поле, а не читайте `transcript_path`: файл транскрипта не гарантируется включать финальное сообщение во время Stop на всех версиях.

Массивы `background_tasks` и `session_crons` позволяют hooks различать "сеанс завершен" от "сеанс приостановлен, ожидая фоновой работы для пробуждения его обратно". Оба массива присутствуют, когда реестр задач доступен и пусты, когда ничего не выполняется или не запланировано.

Каждая запись в `background_tasks` описывает одну выполняющуюся задачу и использует эти поля:

| Поле          | Описание                                                                                                                                                                                                                                                                 |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Идентификатор задачи                                                                                                                                                                                                                                                     |
| `type`        | Дружественный ярлык типа задачи, такой как `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` или `MCP task`. Каждый ярлык определяет, какая функция Claude Code создала задачу. Возвращается к необработанному дискриминанту для неизвестных типов |
| `status`      | Текущий статус задачи                                                                                                                                                                                                                                                    |
| `description` | Описание в свободной форме, ограниченное 1000 символами с маркером `… [+N chars]` в строке при обрезке                                                                                                                                                                   |
| `command`     | Командная строка оболочки, ограниченная 1000 символами. Присутствует только для задач `shell`                                                                                                                                                                            |
| `agent_type`  | Имя типа подагента. Присутствует только для задач `subagent`                                                                                                                                                                                                             |
| `server`      | Имя сервера MCP. Присутствует только для задач `monitor` и `MCP task`                                                                                                                                                                                                    |
| `tool`        | Имя инструмента MCP. Присутствует только для задач `monitor` и `MCP task`                                                                                                                                                                                                |
| `name`        | Имя workflow. Присутствует только для задач `workflow`                                                                                                                                                                                                                   |

Каждая запись в `session_crons` описывает одно запланированное пробуждение с областью сеанса, полученное из `CronCreate`, `ScheduleWakeup` и `/loop`:

| Поле        | Описание                                                                                                                                                   |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Идентификатор задачи Cron                                                                                                                                  |
| `schedule`  | Выражение Cron, например `0 9 * * 1-5`                                                                                                                     |
| `recurring` | `false` для одноразовых пробуждений, чье расписание кодирует одно время срабатывания, `true` для задач, которые повторно срабатывают при каждом совпадении |
| `prompt`    | Подсказка, отправленная при срабатывании cron, ограниченная 1000 символами с тем же маркером `… [+N chars]`                                                |

Этот пример показывает ввод Stop с одной выполняющейся задачей shell и одним повторяющимся cron:

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
  Управление решением Stop
</h4>

Hooks `Stop` и `SubagentStop` могут управлять тем, продолжает ли Claude. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, ваш скрипт hook может вернуть эти поля, специфичные для события:

| Поле                                   | Описание                                                                                                                                                                                                   |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` предотвращает остановку Claude. Опустите, чтобы позволить Claude остановиться                                                                                                                    |
| `reason`                               | Требуется, когда `decision` имеет значение `"block"`. Говорит Claude, почему он должен продолжить                                                                                                          |
| `hookSpecificOutput.additionalContext` | Обратная связь без ошибок для Claude. Разговор продолжается, чтобы Claude мог действовать на ней, но в отличие от `decision: "block"` она показана в транскрипте как обратная связь hook, а не ошибка hook |

Hook, который блокирует выходом 2, маршрутизируется так же, как `reason`: Claude получает сообщение stderr как объяснение того, почему он должен продолжить.

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Используйте `additionalContext`, когда hook работает как задумано и дает Claude руководство, такое как "запустить набор тестов перед завершением". Это держит разговор идущим через те же защиты цикла, что и `decision: "block"`, а именно ввод `stop_hook_active` и ограничение 8-последовательного продолжения, но транскрипт помечает его `Stop hook feedback` и уведомление об ошибке hook не показывается:

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

Запускается вместо [Stop](#stop), когда ход заканчивается из-за ошибки API. Claude Code игнорирует вывод и код выхода hook, кроме [`terminalSequence`](#emit-terminal-notifications). Используйте это для логирования сбоев, отправки оповещений или принятия действий восстановления, когда Claude не может завершить ответ из-за ограничений скорости, проблем аутентификации или других ошибок API.

<h4 id="stopfailure-input">
  Ввод StopFailure
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks StopFailure получают `error`, опциональный `error_details` и опциональный `last_assistant_message`. Поле `error` определяет тип ошибки и используется для фильтрации matcher.

| Поле                     | Описание                                                                                                                                                                                                                                        |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Тип ошибки: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error` или `unknown` |
| `error_details`          | Дополнительные детали об ошибке, когда доступны                                                                                                                                                                                                 |
| `last_assistant_message` | Отображаемый текст ошибки, показанный в разговоре. В отличие от `Stop` и `SubagentStop`, где это поле содержит разговорный вывод Claude, для `StopFailure` оно содержит строку ошибки API, такую как `"API Error: Rate limit reached"`          |

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

Hooks StopFailure не имеют управления решением. Они запускаются только в целях уведомления и логирования.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Запускается, когда товарищ [команды агентов](/docs/ru/agent-teams) собирается перейти в режим ожидания после завершения своего хода. Используйте это для применения шлюзов качества перед остановкой товарища, такие как требование прохождения проверок lint или проверка существования выходных файлов.

Hooks TeammateIdle не поддерживают matchers и срабатывают при каждом возникновении.

<h4 id="teammateidle-input">
  Ввод TeammateIdle
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks TeammateIdle получают `teammate_name` и `team_name`.

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

| Поле            | Описание                                                                     |
| :-------------- | :--------------------------------------------------------------------------- |
| `teammate_name` | Имя товарища, который собирается перейти в режим ожидания                    |
| `team_name`     | Устарело. Имя команды, полученное из сеанса; будет удалено в будущем выпуске |

<h4 id="teammateidle-decision-control">
  Управление решением TeammateIdle
</h4>

Hooks TeammateIdle поддерживают два способа управления поведением товарища:

* **Код выхода 2**: товарищ получает сообщение stderr как обратную связь и продолжает работать вместо перехода в режим ожидания.
* **JSON `{"continue": false, "stopReason": "..."}`**: полностью останавливает товарища, совпадая с поведением hook `Stop`. `stopReason` показывается пользователю.

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

Запускается, когда файл конфигурации изменяется во время сеанса. Используйте это для аудита изменений параметров, применения политик безопасности или блокировки несанкционированных изменений файлов конфигурации.

Claude Code запускает hooks ConfigChange, когда файл параметров, файл управляемой политики или файл skill изменяется. Для управляемой политики он запускает их только, когда `managed-settings.json` или файл в `managed-settings.d/` изменяется. Он применяет [параметры, управляемые сервером](/docs/ru/server-managed-settings) и изменения в macOS управляемых предпочтениях или политике реестра Windows без их запуска. На WSL с [`wslInheritsWindowsSettings`](/docs/ru/settings-reference#wslinheritswindowssettings) он также применяет измененный файл управляемых параметров Windows на его опросе политики без их запуска.

Matcher фильтрует по источнику конфигурации:

| Matcher            | Когда срабатывает                                                   |
| :----------------- | :------------------------------------------------------------------ |
| `user_settings`    | `~/.claude/settings.json` изменяется                                |
| `project_settings` | `.claude/settings.json` изменяется                                  |
| `local_settings`   | `.claude/settings.local.json` изменяется                            |
| `policy_settings`  | `managed-settings.json` или файл в `managed-settings.d/` изменяется |
| `skills`           | Файл skill в `.claude/skills/` изменяется                           |

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
  Ввод ConfigChange
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks ConfigChange получают `source` и опционально `file_path`. Поле `source` указывает, какой тип конфигурации изменился, и `file_path` предоставляет путь к конкретному файлу, который был изменен.

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
  Управление решением ConfigChange
</h4>

Hooks ConfigChange могут блокировать изменения конфигурации от вступления в силу. Используйте код выхода 2 или JSON `decision` для предотвращения изменения. При блокировке новые параметры не применяются к работающему сеансу.

| Поле       | Описание                                                                                       |
| :--------- | :--------------------------------------------------------------------------------------------- |
| `decision` | `"block"` предотвращает применение изменения конфигурации. Опустите, чтобы позволить изменению |
| `reason`   | Принято, но никогда не показано                                                                |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Изменения `policy_settings` не могут быть заблокированы. Hooks все еще срабатывают для источников `policy_settings`, когда файл управляемых параметров на машине изменяется, поэтому вы можете использовать их для логирования этих редактирований, но любое решение блокировки игнорируется. Это гарантирует, что параметры, управляемые предприятием, всегда вступают в силу. Claude Code не запускает hooks `ConfigChange`, когда прибывают или обновляются [параметры, управляемые сервером](/docs/ru/server-managed-settings).

Claude Code действует на решение блокировки из JSON-вывода hook ConfigChange и отбрасывает `systemMessage` и `continue`. Заблокированное изменение не выводит никакого сообщения вам или Claude, независимо от того, блокируете ли вы с `reason` или с stderr при выходе 2. Claude Code только записывает строку в debug log.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Запускается, когда команда оболочки в основном разговоре изменяет рабочую директорию, например когда Claude выполняет команду `cd`. Используйте это для реакции на изменения директории: перезагрузка переменных окружения, активация цепочек инструментов для конкретного проекта или автоматический запуск скриптов настройки. Пары с [FileChanged](#filechanged) для инструментов, таких как [direnv](https://direnv.net/), которые управляют окружением для каждой директории.

Hooks CwdChanged имеют доступ к [`CLAUDE_ENV_FILE`](#persist-environment-variables). Переменные, записанные в этот файл, сохраняются в последующих командах Bash до следующего события CwdChanged, когда Claude Code их очищает.

CwdChanged не поддерживает matchers и срабатывает при каждом возникновении.

<h4 id="cwdchanged-input">
  Ввод CwdChanged
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks CwdChanged получают `old_cwd` и `new_cwd`.

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
  Вывод CwdChanged
</h4>

Помимо [полей JSON-вывода](#json-output), доступных всем hooks, hooks CwdChanged могут вернуть `watchPaths` для динамической установки того, какие пути файлов [FileChanged](#filechanged) отслеживает:

| Поле         | Описание                                                                                                                                                                                                                              |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `watchPaths` | Массив абсолютных путей. Заменяет текущий динамический список отслеживания. Пути из конфигурации вашего `matcher` всегда отслеживаются. Возврат пустого массива очищает динамический список, что типично при входе в новую директорию |

Hooks CwdChanged не имеют управления решением. Они не могут блокировать изменение директории.

Claude Code читает `watchPaths` и `systemMessage` из их JSON-вывода и отбрасывает `continue`. В интерактивных сеансах он показывает `systemMessage` как краткое уведомление терминала. Сообщение не достигает потока сообщений SDK.

<h3 id="directoryadded">
  DirectoryAdded
</h3>

Запускается после добавления рабочей директории во время сеанса с командой `/add-dir` или после добавления клиентом SDK с запросом управления `register_repo_root`. Используйте это для подготовки вновь добавленного репозитория, например установки его зависимостей.

Claude Code не срабатывает это событие, когда:

* Вы передаете директорию с флагом запуска `--add-dir`; [SessionStart](#sessionstart) охватывает эти директории
* Вы добавляете директорию на вкладку `/permissions` Workspace
* Вы добавляете директорию, которая уже является рабочей директорией или находится внутри одной

Claude Code срабатывает DirectoryAdded после обновления состояния sandbox и разрешений, поэтому изолированные инструменты уже видят новую директорию, когда выполняется ваш hook. Команды hook сами выполняются без изоляции.

Claude Code не ждет hook: добавление завершается немедленно, и hook выполняется в фоновом режиме с тайм-аутом по умолчанию 600 секунд.

Matcher фильтрует по тому, как была добавлена директория:

| Matcher              | Когда срабатывает                                                          |
| :------------------- | :------------------------------------------------------------------------- |
| `slash_command`      | Вы добавляете директорию с `/add-dir`                                      |
| `register_repo_root` | Клиент SDK добавляет директорию с запросом управления `register_repo_root` |

<h4 id="directoryadded-input">
  Ввод DirectoryAdded
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks DirectoryAdded получают `directory` и `source`.

| Поле        | Описание                                                                                                              |
| :---------- | :-------------------------------------------------------------------------------------------------------------------- |
| `directory` | Абсолютный путь директории, которая была добавлена                                                                    |
| `source`    | Как была добавлена директория, `"slash_command"` для `/add-dir` или `"register_repo_root"` для запроса управления SDK |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "DirectoryAdded",
  "directory": "/Users/my-other-repo",
  "source": "slash_command"
}
```

Hooks DirectoryAdded не имеют управления решением. Они не могут блокировать добавление, которое уже завершилось. Claude Code отбрасывает поле `continue` из их JSON-вывода и выводит остальное по-разному в зависимости от источника:

* `slash_command`: Claude Code доставляет `systemMessage` hook Claude как контекст на следующем ходе разговора, а не показывает вам. Количество неудачных hooks появляется в транскрипте. Полный вывод сбоя идет в debug log
* `register_repo_root`: Claude Code записывает вывод `systemMessage` и вывод сбоя только в debug log

<h3 id="filechanged">
  FileChanged
</h3>

Запускается, когда отслеживаемый файл изменяется на диске. Claude Code обнаруживает изменения с помощью наблюдателя файловой системы, а не путем проверки вызовов инструментов, поэтому он запускает hook независимо от того, что изменило файл: вызов инструмента `Edit` или `Write`, скрипт, который Claude запускает с `Bash`, или процесс вне Claude Code полностью. Обычное использование — перезагрузка переменных окружения при изменении файлов конфигурации проекта.

`matcher` для этого события служит двум целям:

* **Построить список отслеживания**: значение разделяется на `|` и каждый сегмент регистрируется как буквальное имя файла в рабочей директории, поэтому `".envrc|.env"` отслеживает ровно эти два файла. Шаблоны regex не полезны здесь: значение, такое как `^\.env`, отслеживало бы файл буквально названный `^\.env`.
* **Фильтровать, какие hooks запускаются**: когда отслеживаемый файл изменяется, то же значение фильтрует, какие группы hook запускаются, используя стандартные [правила matcher](#matcher-patterns) против базового имени измененного файла.

Этот пример нормализует окончания строк в `data.csv` после любого изменения, включая команду `Bash` или внешний скрипт, переписывающий файл:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": "data.csv",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/normalize-line-endings.sh"
          }
        ]
      }
    ]
  }
}
```

Hook читает абсолютный путь измененного файла из поля `file_path` JSON-ввода на stdin. Его охрана `grep` тестирует то же самое, что `perl` удаляет, CR в конце строки, поэтому запуск после нормализации выходит без касания файла. Более слабая охрана зацикливается навсегда, потому что `perl -i` переписывает файл, даже когда он ничего не заменяет, и Claude Code запускает hook снова после каждой переписи. Сохраните этот скрипт в `/path/to/normalize-line-endings.sh` и сделайте его исполняемым:

```bash theme={null}
#!/bin/bash
FILE=$(jq -r .file_path)
if grep -q $'\r$' "$FILE"; then
  perl -pi -e 's/\r$//' "$FILE"
fi
```

Чтобы подтвердить, что hook работает, попросите Claude добавить строку CRLF в `data.csv` с командой `Bash`. Claude Code запускает hook и файл заканчивается с окончаниями LF.

Чтобы отслеживать файлы, которые вы не можете назвать заранее, верните [`watchPaths`](#filechanged-output) из hook для динамического обновления списка отслеживания. Claude Code запускает наблюдатель только, когда что-то называет файл для отслеживания, поэтому заполните список группой FileChanged, чей matcher называет по крайней мере один файл, или с hook [SessionStart](#sessionstart-decision-control) или [CwdChanged](#cwdchanged), который возвращает `watchPaths`. Matcher все еще фильтрует, какие группы hook запускаются, когда отслеживаемый файл изменяется, поэтому дайте группе, которая обрабатывает динамические пути, опущенный matcher, который совпадает с каждым отслеживаемым файлом и ничего не добавляет в список отслеживания. Matcher `"*"` также совпадает с каждым файлом, но Claude Code регистрирует его в списке отслеживания, как любое другое значение, как буквальный файл названный `*`.

Hooks FileChanged имеют доступ к [`CLAUDE_ENV_FILE`](#persist-environment-variables). Переменные, записанные в этот файл, сохраняются в последующих командах Bash до следующего события [CwdChanged](#cwdchanged), когда Claude Code их очищает.

<h4 id="filechanged-input">
  Ввод FileChanged
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks FileChanged получают `file_path` и `event`.

| Поле        | Описание                                                                                                          |
| :---------- | :---------------------------------------------------------------------------------------------------------------- |
| `file_path` | Абсолютный путь к файлу, который изменился                                                                        |
| `event`     | Что произошло: `"change"` для измененного файла, `"add"` для созданного файла или `"unlink"` для удаленного файла |

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
  Вывод FileChanged
</h4>

Помимо [полей JSON-вывода](#json-output), доступных всем hooks, hooks FileChanged могут вернуть `watchPaths` для динамического обновления того, какие пути файлов отслеживаются:

| Поле         | Описание                                                                                                                                                                                                                                                      |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `watchPaths` | Массив абсолютных путей. Заменяет текущий динамический список отслеживания. Пути из конфигурации вашего `matcher` всегда отслеживаются. Используйте это, когда ваш скрипт hook обнаруживает дополнительные файлы для отслеживания на основе измененного файла |

Hooks FileChanged не имеют управления решением. Они не могут блокировать изменение файла от возникновения.

Claude Code читает `watchPaths` и `systemMessage` из их JSON-вывода и отбрасывает `continue`. В интерактивных сеансах он показывает `systemMessage` как краткое уведомление терминала. Сообщение не достигает потока сообщений SDK.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Запускается, когда создается worktree, будь то из `claude --worktree`, из [подагента, использующего `isolation: "worktree"`](/docs/ru/sub-agents#choose-the-subagent-scope), или для [фонового сеанса](/docs/ru/agent-view#how-file-edits-are-isolated), который Claude Code изолирует в своем собственном worktree. По умолчанию Claude Code создает изолированную рабочую копию с `git worktree`. Настройка hook WorktreeCreate заменяет это поведение git по умолчанию, позволяя вам использовать другую систему контроля версий, такую как SVN, Perforce или Mercurial.

Поскольку hook заменяет поведение по умолчанию полностью, [`.worktreeinclude`](/docs/ru/worktrees#copy-gitignored-files-into-worktrees) не обрабатывается. Если вам нужно скопировать локальные файлы конфигурации, такие как `.env`, в новый worktree, сделайте это внутри вашего скрипта hook.

Hook должен вернуть путь к созданной директории worktree. Claude Code использует этот путь как рабочую директорию для изолированного сеанса. См. [WorktreeCreate output](#worktreecreate-output) для того, как каждый тип hook возвращает путь.

Claude Code действует на успех hook и возвращенный путь и отбрасывает `systemMessage` и `continue`.

Этот пример создает рабочую копию SVN и выводит путь для использования Claude Code. Замените URL репозитория на свой собственный:

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

Hook читает `name` worktree из JSON-ввода на stdin, проверяет свежую копию в новую директорию и выводит путь директории. `echo` на последней строке — это то, что Claude Code читает как путь worktree. Перенаправьте любой другой вывод в stderr, чтобы он не мешал пути.

<h4 id="worktreecreate-input">
  Ввод WorktreeCreate
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks WorktreeCreate получают поле `name`. Это идентификатор slug для нового worktree, либо указанный пользователем, либо автоматически сгенерированный, например `bold-oak-a3f2`.

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
  Вывод WorktreeCreate
</h4>

Hooks WorktreeCreate не используют стандартную модель решения разрешить/блокировать. Вместо этого успех или сбой hook определяет результат. Hook должен вернуть путь к созданной директории worktree:

* **Command hooks** (`type: "command"`): выведите путь как последнюю непустую строку stdout. Claude Code удаляет коды ANSI перед чтением этой строки, поэтому баннеры запуска оболочки, выведенные перед вашим `echo`, игнорируются. Перенаправьте любой другой вывод hook в stderr.
* **HTTP hooks** (`type: "http"`): верните `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` в теле ответа.

Если hook не удается или не создает путь, создание worktree не удается с ошибкой.

Claude Code разрешает относительный путь против директории, в которой выполнялся hook, свернув любые сегменты `.` или `..` в нем. Если результирующий путь не является директорией, которую Claude Code может ввести, сеанс выводит ошибку с названием пути и выходит с кодом 1.

Claude Code отказывает абсолютному пути, который содержит сегменты `.` или `..`, и любому пути, который проходит через символическую ссылку ниже корня репозитория, потому что символическая ссылка, зафиксированная в репозитории, может перенаправить worktree вне его. Ошибка называет отклоненный компонент. Верните нормализованный путь, который не проходит через символическую ссылку внутри репозитория. До версии 2.1.216 создание worktree следовало пути hook без этого скрининга.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Запускается, когда worktree удаляется. Это очистка, соответствующая [WorktreeCreate](#worktreecreate). Событие срабатывает, когда:

* вы выходите из сеанса `--worktree` и выбираете его удаление
* подагент с `isolation: "worktree"` завершается
* вы удаляете [фоновый сеанс](/docs/ru/agent-view#what-deleting-a-session-removes), чей worktree создал hook

Для git-based worktrees Claude Code обрабатывает очистку автоматически с `git worktree remove`. Если вы настроили hook WorktreeCreate для системы контроля версий, не основанной на git, свяжите его с hook WorktreeRemove для обработки очистки. Без него директория worktree остается на диске.

Claude Code отбрасывает [поля JSON-вывода](#json-output) hook WorktreeRemove, такие как `systemMessage` и `continue`.

Для удаления фонового сеанса Claude Code проверяет сохраненный путь worktree перед запуском hook и отказывает пути, который является символической ссылкой или проходит через одну ниже корня репозитория. Hook запускается для worktree, который все еще содержит файлы только, когда вы подтверждаете удаление в [agent view](/docs/ru/agent-view#what-deleting-a-session-removes); для такого worktree [`claude rm`](/docs/ru/agent-view#manage-sessions-from-the-shell) сохраняет сеанс и worktree вместо этого. До версии 2.1.216 hook запускался на сохраненном пути без этих проверок.

Claude Code передает путь, возвращенный WorktreeCreate, как `worktree_path` в ввод hook. Этот пример читает этот путь и удаляет директорию:

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
  Ввод WorktreeRemove
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks WorktreeRemove получают поле `worktree_path`, которое является абсолютным путем к удаляемому worktree.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

Код выхода hook WorktreeRemove определяет результат. Когда hook выходит с ненулевым кодом и директория в `worktree_path` все еще существует после этого, удаление не удается:

* Worktree остается на диске, и команда hook и stderr идут в [debug log](#debug-hooks).
* Если вы удаляли фоновый сеанс, сеанс остается тоже. Сообщение отказа в [agent view](/docs/ru/agent-view#what-deleting-a-session-removes) сообщает, как закончился hook, такой как `exited 1`, цитирует начало его stderr и говорит, удаляет ли удаление сеанса снова директорию в любом случае.

<h3 id="precompact">
  PreCompact
</h3>

Запускается перед тем, как Claude Code собирается запустить операцию compact.

Значение matcher указывает, было ли сжатие запущено вручную или автоматически:

| Matcher  | Когда срабатывает                                                                                        |
| :------- | :------------------------------------------------------------------------------------------------------- |
| `manual` | `/compact`                                                                                               |
| `auto`   | Auto-compact, когда разговор достигает [окна auto-compact](/docs/ru/model-config#set-the-auto-compact-window) |

Выйдите с кодом 2 для блокировки сжатия. Для ручного `/compact` сообщение stderr показывается пользователю. Вы также можете блокировать, возвращая JSON с `"decision": "block"`.

Блокировка автоматического сжатия имеет разные эффекты в зависимости от того, когда оно срабатывает. Если сжатие было запущено упреждающе перед пределом контекста, Claude Code пропускает его и разговор продолжается несжатым. Если сжатие было запущено для восстановления от ошибки лимита контекста, уже возвращенной API, основная ошибка выводится и текущий запрос не удается.

Claude Code отбрасывает поля `systemMessage` и `continue` hook PreCompact.

<h4 id="precompact-input">
  Ввод PreCompact
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks PreCompact получают `trigger` и `custom_instructions`. Для `manual`, `custom_instructions` содержит то, что пользователь передает в `/compact` и имеет значение `null`, когда они ничего не передают. Для `auto`, `custom_instructions` имеет значение `null`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": null
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Запускается после завершения операции compact Claude Code. Используйте это событие для реакции на новое сжатое состояние, например для логирования сгенерированного резюме или обновления внешнего состояния. Claude Code отбрасывает поля `systemMessage` и `continue` hook PostCompact.

Те же значения matcher применяются, как для `PreCompact`:

| Matcher  | Когда срабатывает                                                                                              |
| :------- | :------------------------------------------------------------------------------------------------------------- |
| `manual` | После `/compact`                                                                                               |
| `auto`   | После auto-compact, когда разговор достигает [окна auto-compact](/docs/ru/model-config#set-the-auto-compact-window) |

<h4 id="postcompact-input">
  Ввод PostCompact
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks PostCompact получают `trigger` и `compact_summary`. Поле `compact_summary` содержит резюме разговора, сгенерированное операцией compact.

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

Hooks PostCompact не имеют управления решением. Они не могут влиять на результат сжатия, но могут выполнять последующие задачи.

<h3 id="premodelswitch">
  PreModelSwitch
</h3>

Запускается перед применением переключения модели, которое вы или клиент запросили. Используйте это для блокировки переключения, требования подтверждения или показа стоимости переключения перед его выполнением.

PreModelSwitch требует Claude Code v2.1.251 или позже. Claude Code запускает его для этих запросов:

* `/model <name>` и средство выбора `/model`
* Средство выбора модели `Option+P` или `Alt+P`
* Параметр Model в `/config`
* Включение [fast mode](/docs/ru/fast-mode), когда это изменяет модель сеанса
* Запрос `set_model` или изменение модели в запросе `apply_flag_settings` от хоста [Agent SDK](/docs/ru/agent-sdk/typescript#query-object) или [Remote Control](/docs/ru/remote-control)

Claude Code не запускает hooks PreModelSwitch для переключений, которые он делает самостоятельно, такие как [автоматический fallback модели](/docs/ru/model-config#automatic-model-fallback) или восстановление модели при возобновлении сеанса. Эти изменения достигают [PostModelSwitch](#postmodelswitch) только.

Claude Code сравнивает matcher против канонического имени модели, на которую сеанс переключается, игнорируя любой суффикс `[1m]`. Псевдоним, такой как `opus`, датированный ID модели и ID, специфичный для поставщика, такой как ID модели Amazon Bedrock, все совпадают с одним каноническим именем, на которое они разрешаются, поэтому `claude-opus-5` охватывает каждое написание Opus 5.

Когда Claude Code не может определить каноническое имя для цели, например пользовательский ID модели, который знает только ваш [LLM gateway](/docs/ru/llm-gateway), он запускает каждый hook PreModelSwitch независимо от matcher. Hook, который блокирует, должен поэтому проверить `to_model` из своего ввода, а не полагаться только на matcher.

Напишите matcher как точное имя, список, разделенный `|`, такой как `claude-opus-4-6|claude-opus-5`, или регулярное выражение, такое как `.*opus.*`. Этот пример использует matcher точного имени и также проверяет `to_model` из ввода hook, поэтому он отказывает переключению на Opus 4.6 выходом 2 и позволяет любой другой цели пройти:

<Tabs>
  <Tab title="macOS/Linux">
    Команда проверяет `to_model` с `jq`:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "jq -e '.to_model | test(\"opus-4-6\")' > /dev/null && { echo 'Opus 4.6 is retired for this project. Use a newer model.' >&2; exit 2; }; exit 0"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Зарегистрируйте command hook, который запускает скрипт через PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-opus-46.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Сохраните этот скрипт в `.claude/hooks/block-opus-46.ps1` в вашем проекте:

    ```powershell theme={null}
    $hookInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    if ($hookInput.to_model -match 'opus-4-6') {
      [Console]::Error.WriteLine('Opus 4.6 is retired for this project. Use a newer model.')
      exit 2
    }
    exit 0
    ```
  </Tab>
</Tabs>

Чтобы подтвердить, что hook работает, запустите `/model claude-opus-4-6` из сеанса, работающего на другой модели. Claude Code сохраняет текущую модель и сообщает, что hook PreModelSwitch заблокировал переключение, с вашим сообщением как причиной.

<h4 id="premodelswitch-input">
  Ввод PreModelSwitch
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks PreModelSwitch получают поля в этой таблице. Последние пять описывают, что стоит повторная отправка разговора на новую модель, поэтому hook может показать эту цифру перед переключением.

| Поле                        | Тип               | Описание                                                                                                                                                                                                                                                                      |
| :-------------------------- | :---------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `from_model`                | string            | ID модели, на которую переключается                                                                                                                                                                                                                                           |
| `to_model`                  | string            | ID модели, на которую переключается. Matcher сравнивает против канонического имени этой модели                                                                                                                                                                                |
| `requested_model`           | string или `null` | Модель, которую назвал запрос: псевдоним, такой как `opus`, полный ID модели или `null`, когда запрос был для модели по умолчанию                                                                                                                                             |
| `source`                    | string            | Откуда пришел запрос: `"command"` для `/model <name>`, параметра Model в `/config` или включения fast mode; `"picker"` для средства выбора модели; `"sdk"` для запроса `set_model` или изменения модели в запросе `apply_flag_settings` от хоста Agent SDK или Remote Control |
| `context_tokens`            | number            | Токены, которые следующий запрос повторно отправляет как его подсказка: входные, кэш-чтение, кэш-создание и выходные токены последнего ответа в основном разговоре, в сумме. `0` перед первым ответом                                                                         |
| `prompt_cache_warm`         | boolean           | Теплый ли кэш подсказок текущей модели, означая, что переключение его теряет                                                                                                                                                                                                  |
| `cache_ttl`                 | string            | [Время жизни кэша подсказок](/docs/ru/prompt-caching#cache-lifetime), которое Claude Code запрашивает для этого сеанса: `"5m"` или `"1h"`                                                                                                                                          |
| `estimated_cache_write_usd` | number            | Предполагаемая стоимость в долларах США записи `context_tokens` в кэш подсказок на `to_model` при ставке `cache_ttl`, исключая следующий ответ. Сервер может не нуждаться в повторном кэшировании всего контекста, поэтому рассматривайте это как оценку                      |
| `pricing`                   | string            | Как Claude Code оценил `estimated_cache_write_usd`: `"configured"` по вашим собственным ставкам организации, когда она их настроила, `"catalog"` по цене списка или `"default"`, когда `to_model` не имеет известной цены и Claude Code предположил ставку по умолчанию       |

Этот пример показывает ввод для `/model opus` в сеансе, работающем на Sonnet 5:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreModelSwitch",
  "from_model": "claude-sonnet-5",
  "to_model": "claude-opus-5",
  "requested_model": "opus",
  "source": "command",
  "context_tokens": 182340,
  "prompt_cache_warm": true,
  "cache_ttl": "5m",
  "estimated_cache_write_usd": 1.1396,
  "pricing": "catalog"
}
```

<h4 id="premodelswitch-decision-control">
  Управление решением PreModelSwitch
</h4>

Hooks `PreModelSwitch` могут отменить переключение, попросить пользователя подтвердить его или позволить ему продолжить. Код выхода 2 или `decision: "block"` верхнего уровня отменяет переключение.

Для более тонкого управления, верните `permissionDecision` и `permissionDecisionReason` в объекте `hookSpecificOutput`, как на [PreToolUse](#pretooluse-decision-control). `PreModelSwitch` принимает `"allow"`, `"deny"` и `"ask"`. Он не принимает `"defer"`, `updatedInput` или `additionalContext`. Таблица ниже описывает оба поля:

| Поле                       | Описание                                                                                                                                                                                                                             |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` продолжает и пропускает [подтверждение, которое Claude Code показывает, пока кэш подсказок теплый](/docs/ru/prompt-caching#switching-models). `"deny"` отменяет переключение. `"ask"` подсказывает пользователю подтвердить его |
| `permissionDecisionReason` | Для `"deny"`, показано пользователю как причина блокировки переключения или возвращено как ошибка для запроса `set_model`. Для `"ask"`, показано в подсказке подтверждения. Игнорируется для `"allow"`                               |

Только `/model` в интерактивном сеансе может показать подсказку `"ask"`. На каждой другой поверхности, включая неинтерактивный режим с флагом `-p`, `/config` и запросы `set_model`, Claude Code рассматривает `"ask"` как отказ.

Этот пример просит пользователя подтвердить и цитирует количество токенов из `context_tokens`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreModelSwitch",
    "permissionDecision": "ask",
    "permissionDecisionReason": "Switching now re-sends about 180k tokens to the new model. Continue?"
  }
}
```

Когда несколько hooks PreModelSwitch возвращают разные решения, приоритет — `deny` > `ask` > `allow`.

Claude Code показывает пользователю любой `systemMessage`, который возвращает ваш hook, независимо от решения, поэтому hook отчета стоимости может вернуть `{"systemMessage": "..."}` и выйти 0.

Hook PreModelSwitch, который не отвечает перед своим тайм-аутом, блокирует переключение. На [PreToolUse](#timeouts), в отличие от этого, тайм-аут command hook позволяет вызову инструмента продолжить. Тайм-аут по умолчанию для этого события составляет 30 секунд. `PreModelSwitch` запускает только hooks `command`, `http` и `mcp_tool`, поэтому стандарты `prompt` и `agent` не применяются.

Hook, который выходит с кодом, отличным от 0 или 2, и не выводит JSON решение, не блокирует: Claude Code показывает его stderr и применяет переключение, как описано в [Other exit codes](#other-exit-codes).

<h3 id="postmodelswitch">
  PostModelSwitch
</h3>

Запускается после изменения модели сеанса. Используйте это для предоставления руководства, специфичного для модели, без редактирования каждого CLAUDE.md, например организационной инструкции, которая применяется на определенных моделях.

PostModelSwitch требует Claude Code v2.1.251 или позже. Он не может блокировать, потому что модель уже изменилась. Claude Code запускает hooks PostModelSwitch после любого из этих изменений:

* Переключение, которое вы или клиент запросили
* [Автоматический fallback модели](/docs/ru/model-config#automatic-model-fallback), который изменяет модель сеанса
* Параметр, такой как [`opusplan`](/docs/ru/model-config#opusplan-model-setting), входящий или выходящий из режима плана
* Claude Code восстанавливает модель при возобновлении сеанса

Claude Code не запускает hooks PostModelSwitch, когда модель из [цепочки fallback модели](/docs/ru/model-config#fallback-model-chains) обслуживает ход, потому что эта замена длится один ход и оставляет модель сеанса неизменной.

Matcher следует тем же правилам, что и [PreModelSwitch](#premodelswitch): Claude Code сравнивает его против канонического имени модели, на которую переключился сеанс.

Этот пример добавляет руководство, когда модель сеанса изменяется на любую модель Opus:

```json theme={null}
{
  "hooks": {
    "PostModelSwitch": [
      {
        "matcher": ".*opus.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'On Opus, delegate implementation work to subagents and keep this conversation for planning and review.'"
          }
        ]
      }
    ]
  }
}
```

Чтобы подтвердить, что hook работает, переключитесь на модель Opus из сеанса, работающего на другой модели, например запустите `/model opus` из сеанса Sonnet, затем спросите Claude, какое руководство оно имеет о текущей модели.

<h4 id="postmodelswitch-input">
  Ввод PostModelSwitch
</h4>

Hooks PostModelSwitch получают те же поля, что и [PreModelSwitch](#premodelswitch-input), с `hook_event_name`, установленным на `"PostModelSwitch"`, и двумя дополнительными значениями `source`: `"auto"` для автоматического fallback или другого изменения, которое Claude Code сделал самостоятельно, и `"resume"` для модели, восстановленной при возобновлении сеанса.

`requested_model` имеет значение `null`, когда `source` имеет значение `"auto"`. Когда `source` имеет значение `"resume"`, это сохраненный параметр модели, который Claude Code восстановил.

<h4 id="postmodelswitch-decision-control">
  Управление решением PostModelSwitch
</h4>

Claude Code берет ваш [простой текст stdout](#exit-code-0) hook при выходе 0 или `additionalContext` из JSON-вывода и доставляет его Claude со следующим запросом после переключения. Помимо [полей JSON-вывода](#json-output), доступных всем hooks, вы можете вернуть:

| Поле                | Описание                                                                                                           |
| :------------------ | :----------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Строка, добавленная в контекст Claude со следующим запросом. См. [Add context for Claude](#add-context-for-claude) |

Если hook не завершится в течение пяти секунд после отправки следующей подсказки, Claude Code отправляет этот запрос без вывода и прикрепляет его к следующему запросу вместо этого. Если модель изменяется несколько раз перед следующим запросом, Claude Code доставляет только вывод для переключения целевой модели последнего.

<h3 id="sessionend">
  SessionEnd
</h3>

Запускается, когда сеанс Claude Code заканчивается. Полезно для задач очистки, логирования статистики сеанса или сохранения состояния сеанса. Поддерживает matchers для фильтрации по причине выхода.

Поле `reason` в ввод hook указывает, почему сеанс закончился:

| Причина                       | Описание                                                                                            |
| :---------------------------- | :-------------------------------------------------------------------------------------------------- |
| `clear`                       | Сеанс очищен с командой `/clear`                                                                    |
| `resume`                      | Сеанс переключен через интерактивный `/resume`                                                      |
| `logout`                      | Пользователь вышел                                                                                  |
| `prompt_input_exit`           | Пользователь вышел, пока ввод подсказки был видимым                                                 |
| `other`                       | Другие причины выхода                                                                               |
| `bypass_permissions_disabled` | Удалено в версии 2.1.234; Claude Code не отправляет его. Удалите его из ваших matchers `SessionEnd` |

<h4 id="sessionend-input">
  Ввод SessionEnd
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks SessionEnd получают поле `reason`, указывающее, почему сеанс закончился. См. [таблицу причин](#sessionend) выше для всех значений.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Hooks SessionEnd не имеют управления решением. Они не могут блокировать завершение сеанса, но могут выполнять задачи очистки. Claude Code отбрасывает их [поля JSON-вывода](#json-output), такие как `systemMessage`.

Hooks SessionEnd имеют тайм-аут по умолчанию 1.5 секунды. Он применяется, когда вы выходите, запускаете `/clear` или переключаете сеансы с интерактивным `/resume`. Вы можете дать hook больше времени двумя способами:

* **Per-hook `timeout`**: установите `timeout` в конфигурации этого hook. Общий бюджет автоматически повышается, чтобы совпадать с наивысшим `timeout` per-hook в ваших файлах параметров, до 60 секунд. Если вы повышаете бюджет таким образом, hook без своего собственного `timeout` все еще сохраняет стандарт. Тайм-ауты, установленные на hooks, предоставленные плагином, не повышают бюджет.
* **`CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS`**: установите эту переменную окружения в миллисекундах для явного переопределения бюджета. Значение, которое вы установили, также становится тайм-аутом для каждого hook без своего собственного `timeout`.

Этот пример устанавливает бюджет на 5 секунд:

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

До версии 2.1.268 `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` повышал только общий бюджет, и hook без своего собственного `timeout` все еще отменялся через 1.5 секунды.

<h3 id="elicitation">
  Elicitation
</h3>

Запускается, когда сервер MCP запрашивает ввод пользователя во время задачи. По умолчанию Claude Code показывает интерактивный диалог для ответа пользователя. Hooks могут перехватить этот запрос и ответить программно, полностью пропустив диалог.

Поле matcher совпадает с названием сервера MCP.

<h4 id="elicitation-input">
  Ввод Elicitation
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks Elicitation получают `mcp_server_name`, `message` и опциональные `mode`, `url`, `elicitation_id` и `requested_schema` поля.

Для запроса в режиме формы, наиболее распространенный случай:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

Для запроса в режиме URL, используемого для аутентификации на основе браузера:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Вывод Elicitation
</h4>

Чтобы ответить программно без показа диалога, верните объект JSON с `hookSpecificOutput`:

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

| Поле      | Значения                      | Описание                                                                                       |
| :-------- | :---------------------------- | :--------------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Принять ли, отклонить или отменить запрос                                                      |
| `content` | object                        | Значения полей формы для отправки. Используется только, когда `action` имеет значение `accept` |

Код выхода 2 отклоняет запрос. Claude Code не показывает ваше сообщение stderr нигде.

Claude Code действует на `hookSpecificOutput` из JSON-вывода hook Elicitation и отбрасывает `systemMessage` и `continue`.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Запускается после того, как пользователь ответит на запрос MCP. Hooks могут наблюдать, изменять или блокировать ответ перед его отправкой обратно на сервер MCP.

Поле matcher совпадает с названием сервера MCP.

<h4 id="elicitationresult-input">
  Ввод ElicitationResult
</h4>

Помимо [общих полей ввода](#common-input-fields), hooks ElicitationResult получают `mcp_server_name`, `action` и опциональные `mode`, `elicitation_id` и `content` поля.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  Вывод ElicitationResult
</h4>

Чтобы переопределить ответ пользователя, верните объект JSON с `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Поле      | Значения                      | Описание                                                                                        |
| :-------- | :---------------------------- | :---------------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Переопределяет действие пользователя                                                            |
| `content` | object                        | Переопределяет значения полей формы. Имеет смысл только, когда `action` имеет значение `accept` |

Код выхода 2 блокирует ответ, изменяя эффективное действие на `decline`. Claude Code не показывает ваше сообщение stderr нигде.

Claude Code действует на `hookSpecificOutput` из JSON-вывода hook ElicitationResult и отбрасывает `systemMessage` и `continue`.

<h2 id="prompt-based-hooks">
  Prompt-based hooks
</h2>

В дополнение к command, HTTP и MCP tool hooks, Claude Code поддерживает prompt-based hooks (`type: "prompt"`), которые используют LLM для оценки разрешения или блокировки действия, и agent hooks (`type: "agent"`), которые порождают агентного верификатора с доступом к инструментам. Не все события поддерживают каждый тип hook.

События, которые поддерживают все пять типов hook (`command`, `http`, `mcp_tool`, `prompt` и `agent`):

* `PermissionDenied`
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

`PermissionRequest` поддерживает `command`, `http`, `mcp_tool` и `prompt` hooks, но не `agent` hooks. Если вы настроите agent hook на этом событии, Claude Code пропустит его и поток разрешений продолжится без изменений. Чтобы разрешить или отклонить из hook, верните [объект решения](#permissionrequest-decision-control) из command или HTTP hook.

События, которые поддерживают `command`, `http` и `mcp_tool` hooks, но не `prompt` или `agent`:

* `ConfigChange`
* `CwdChanged`
* `DirectoryAdded`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `MessageDisplay`
* `Notification`
* `PostCompact`
* `PostModelSwitch`
* `PreCompact`
* `PreModelSwitch`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` и `Setup` поддерживают `command` и `mcp_tool` hooks, и [MCP tool hook fields](#mcp-tool-hook-fields) описывает, когда их `mcp_tool` hooks запускаются. Они не поддерживают `http`, `prompt` или `agent` hooks.

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

Установите `type` на `"prompt"` и предоставьте строку `prompt` вместо `command`. Используйте заполнитель `$ARGUMENTS` для внедрения данных JSON входа hook в текст вашей подсказки.

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

| Поле              | Обязательно | Описание                                                                                                                                                                                                                         |
| :---------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | да          | Должно быть `"prompt"`                                                                                                                                                                                                           |
| `prompt`          | да          | Текст подсказки для отправки LLM. Используйте `$ARGUMENTS` как заполнитель для JSON входа hook. Если `$ARGUMENTS` отсутствует, JSON входа добавляется к подсказке                                                                |
| `model`           | нет         | Модель для использования при оценке. По умолчанию быстрая модель                                                                                                                                                                 |
| `timeout`         | нет         | Таймаут в секундах. По умолчанию: 30                                                                                                                                                                                             |
| `continueOnBlock` | нет         | На событиях, к которым это применяется, `true` передаёт причину `ok: false` обратно Claude и продолжает вместо завершения хода. По умолчанию: `false`. См. [Response schema](#response-schema) для поведения для каждого события |

<h3 id="response-schema">
  Response schema
</h3>

LLM должен ответить JSON, содержащим:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision",
  "impossible": true | false
}
```

| Поле         | Описание                                                                                                                                                                                                                                                                  |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ok`         | `true` разрешает действие. Для `false`, см. поведение для каждого события ниже                                                                                                                                                                                            |
| `reason`     | Требуется при `ok` равном `false`                                                                                                                                                                                                                                         |
| `impossible` | Опционально. Модель возвращает его с `ok: false`, когда она судит, что условие никогда не может быть удовлетворено. На `Stop` и `SubagentStop`, Claude Code затем позволяет ходу закончиться вместо передачи причины обратно. Agent hooks и другие события игнорируют его |

Что происходит при `ok: false`, зависит от события:

* `Stop` и `SubagentStop`: причина передаётся обратно Claude как его следующая инструкция и ход продолжается, если только ответ также не устанавливает `impossible: true`, в этом случае Claude Code позволяет остановке и ход заканчивается
* `PreToolUse`: вызов инструмента отклоняется; по умолчанию ход заканчивается и причина отказа появляется в чате как строка предупреждения. Установите `continueOnBlock: true` для возврата причины Claude как ошибки инструмента, чтобы он мог скорректировать и продолжить, эквивалентно `permissionDecision: "deny"` из command hook. До v2.1.210 причина отказа возвращалась Claude как ошибка инструмента и ход продолжался
* `PostToolUse`: по умолчанию ход заканчивается и причина появляется в чате как строка предупреждения. Установите `continueOnBlock: true` для передачи причины обратно Claude и продолжения хода вместо этого
* `PostToolBatch`, `UserPromptSubmit` и `UserPromptExpansion`: ход заканчивается и причина появляется как строка предупреждения. Эти события заканчивают ход на `decision: "block"` независимо от `continue`
* `PostToolUseFailure` и `TaskCreated`: причина возвращается Claude как ошибка инструмента и ход продолжается, независимо от `continueOnBlock`
* `TaskCompleted`: когда он срабатывает, потому что задача отмечена как завершённая во время хода, причина возвращается Claude как ошибка инструмента и ход продолжается, независимо от `continueOnBlock`. Когда он срабатывает, потому что товарищ по команде останавливается, он ведёт себя как `TeammateIdle` и останавливает товарища по команде по умолчанию
* `TeammateIdle`: по умолчанию товарищ по команде останавливается и причина появляется как строка предупреждения. Установите `continueOnBlock: true` для передачи причины обратно товарищу по команде и продолжения его работы вместо этого
* `PermissionRequest`: `ok: false` не имеет эффекта. Чтобы отклонить одобрение из hook, используйте [command hook](#command-hook-fields), возвращающий `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` не имеет эффекта, потому что отказ уже произошёл. Единственный результат, который это событие читает, это `hookSpecificOutput.retry`, который prompt и agent hooks не могут установить. Они запускаются на этом событии, но их результат отбрасывается. Используйте [command hook](#command-hook-fields) для возврата `retry`

Если вам нужен более точный контроль над любым событием, используйте [command hook](#command-hook-fields) с полями для каждого события, описанными в [Decision control](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Check multiple conditions before stopping
</h3>

Этот hook `Stop` использует подробную подсказку для проверки трёх условий перед разрешением Claude остановиться. Hooks `SubagentStop` используют тот же формат для оценки, должен ли [subagent](/docs/ru/sub-agents) остановиться. Если модель возвращает `"ok": false`, потому что условие ещё не выполнено, Claude продолжает работать с предоставленной причиной как своей следующей инструкцией:

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

Agent-based hooks (`type: "agent"`) похожи на prompt-based hooks, но с многооборотным доступом к инструментам. Вместо одного вызова LLM, agent hook порождает subagent, который может читать файлы, искать код и проверять кодовую базу для проверки условий. Agent hooks поддерживают те же события, что и [prompt-based hooks](#prompt-based-hooks), за исключением `PermissionRequest`.

<h3 id="how-agent-hooks-work">
  How agent hooks work
</h3>

Когда срабатывает agent hook:

1. Claude Code порождает subagent с вашей подсказкой и JSON входом hook
2. Subagent может использовать инструменты, такие как Read, Grep и Glob, для исследования
3. После до 50 оборотов subagent возвращает структурированное решение `{ "ok": true/false }`
4. Claude Code разрешает действие, если `ok` имеет значение `true`. Если `ok` имеет значение `false`, Claude Code обрабатывает блокировку так же, как prompt hook с `continueOnBlock: true` на этом событии, как указано в разделе [Response schema](#response-schema)

Agent hooks полезны, когда проверка требует проверки фактических файлов или выхода тестов, а не только оценки данных входа hook.

<h3 id="agent-hook-configuration">
  Agent hook configuration
</h3>

Установите `type` на `"agent"` и предоставьте строку `prompt`, используя `$ARGUMENTS` как заполнитель для JSON входа hook. Поля конфигурации те же, что и [prompt hooks](#prompt-hook-configuration), за исключением того, что agent hooks имеют более длинный таймаут по умолчанию в 60 секунд и не имеют поля `continueOnBlock`.

Схема ответа — это `{ "ok": true }` для разрешения или `{ "ok": false, "reason": "..." }` для блокировки. При `ok: false`, Claude Code обрабатывает agent hook так же, как он обрабатывает [prompt hook с `continueOnBlock: true`](#response-schema) на том же событии; agent hooks не имеют поля `continueOnBlock` и не поддерживают поле `impossible` из prompt hook.

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

Этот hook запускает скрипт тестирования после каждого вызова инструмента `Write`. Claude продолжает работать немедленно, пока `run-tests.sh` выполняется. Когда скрипт завершается, его выход доставляется на следующий ход разговора:

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
            "async": true
          }
        ]
      }
    ]
  }
}
```

Как только асинхронный hook запущен в фоне, Claude Code не применяет `timeout` к нему. Claude Code по-прежнему применяет `timeout` к hook, который вы запускаете с `asyncRewake`.

Claude Code доставляет результаты асинхронного hook только во время работы сеанса:

* В [неинтерактивном режиме](/docs/ru/headless) с флагом `-p` Claude Code завершает любой асинхронный hook, который всё ещё работает при завершении, и завершает его с результатом `cancelled`
* Если работа вашего hook должна пережить сеанс `claude -p`, запустите полностью отделённый процесс из него

<h3 id="how-async-hooks-execute">
  Как выполняются асинхронные hooks
</h3>

Когда срабатывает асинхронный hook, Claude Code запускает процесс hook и немедленно продолжает без ожидания его завершения. Hook получает те же JSON входные данные через stdin, что и синхронный hook.

После выхода фонового процесса Claude Code доставляет поля `additionalContext` и `systemMessage` из JSON ответа hook к Claude на следующем ходу разговора. В отличие от `systemMessage` синхронного hook, ни одно из этих полей не показывается вам.

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
            "async": true
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

Асинхронные hooks имеют дополнительные ограничения по сравнению с синхронными hooks:

* Выход hook доставляется на следующий ход разговора. Если сеанс неактивен, ответ ждёт до следующего взаимодействия пользователя. Исключение: hook `asyncRewake`, который выходит с кодом 2, пробуждает Claude немедленно даже когда сеанс неактивен.
* Каждое выполнение создаёт отдельный фоновый процесс. Нет дедупликации между несколькими срабатываниями одного и того же асинхронного hook.

<h2 id="security-considerations">
  Соображения безопасности
</h2>

<h3 id="disclaimer">
  Отказ от ответственности
</h3>

<Warning>
  Command hooks выполняют команды оболочки с вашими полными разрешениями пользователя. Они могут изменять, удалять или получать доступ к любым файлам, к которым может получить доступ ваша учётная запись пользователя. Проверьте и протестируйте все команды hook перед добавлением их в вашу конфигурацию.
</Warning>

<h3 id="workspace-trust">
  Доверие рабочей области
</h3>

Claude Code проверяет доверие рабочей области перед запуском любого hook из файла параметров. Что считается доверенным, зависит от типа сеанса:

* **Интерактивный сеанс**: Claude Code удерживает hooks из каждого файла параметров, включая ваш собственный `~/.claude/settings.json`, пока вы не примете [диалог доверия рабочей области](/docs/ru/permissions#project-allow-rules-and-workspace-trust) для папки или для родительского каталога, чьё доверие распространяется на неё
* **Сеанс `-p` или SDK**: Claude Code никогда не показывает диалог и рассматривает папку как доверенную, поэтому hooks, зафиксированные в `.claude/settings.json` репозитория, запускаются в папке, которой вы никогда не доверяли

Перед тем как запустить `claude -p` над репозиторием, который вы не писали, проверьте его файлы параметров `.claude/`, начните с [`--bare`](/docs/ru/headless#start-faster-with-bare-mode) или [отключите hooks для этого запуска](#disable-or-remove-hooks) с помощью `--settings '{"disableAllHooks": true}'`. Frontmatter hooks в проектном подагенте следуют более строгому правилу, чем hooks файлов параметров. [Что запускается перед доверием папке](/docs/ru/permissions#what-runs-before-you-trust-a-folder) перечисляет каждый вид содержимого репозитория по типу сеанса.

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

На Windows вы можете запустить отдельные hooks в PowerShell, установив `"shell": "powershell"` на command hook. Claude Code автоматически обнаруживает `pwsh.exe`, исполняемый файл PowerShell 7 и более поздних версий, и переходит на `powershell.exe` для Windows PowerShell 5.1.

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

Детали выполнения hooks записываются в файл отладочного журнала. Запустите Claude Code с `claude --debug-file <path>` для записи журнала в известное расположение, или запустите `claude --debug` и прочитайте журнал в `~/.claude/debug/<session-id>.txt`. Флаг `--debug` не выводит на терминал.

Например, hook `PostToolUse` на `Write`, чья команда выводит `hook-ran`, создаёт записи вроде:

```text theme={null}
2026-07-19T02:03:24.382Z [DEBUG] Hook output does not start with {, treating as plain text
2026-07-19T02:03:24.382Z [DEBUG] "Hook PostToolUse:Write (PostToolUse) success:\nhook-ran"
```

Для более детальной информации о совпадении hooks установите `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` для просмотра дополнительных строк логирования, таких как количество совпадений фильтра hook и совпадение запроса.

Для устранения неполадок распространённых проблем, таких как hooks, которые не срабатывают, Stop hooks, которые продолжают блокировать, или ошибки конфигурации, см. [Limitations and troubleshooting](/docs/ru/hooks-guide#limitations-and-troubleshooting) в руководстве. Для более широкого диагностического пошагового руководства, охватывающего `/context`, `/doctor` и приоритет параметров, см. [Debug your config](/docs/ru/debug-your-config).
