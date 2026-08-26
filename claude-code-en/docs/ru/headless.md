> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Запуск Claude Code программно

> Используйте Agent SDK для программного запуска Claude Code из CLI, Python или TypeScript.

[Agent SDK](/docs/ru/agent-sdk/overview) предоставляет вам те же инструменты, цикл агента и управление контекстом, которые питают Claude Code. Он доступен как CLI для скриптов и CI/CD, или как пакеты [Python](/docs/ru/agent-sdk/python) и [TypeScript](/docs/ru/agent-sdk/typescript) для полного программного управления.

Чтобы запустить Claude Code в неинтерактивном режиме, передайте `-p` с вашим запросом и любыми [параметрами CLI](/docs/ru/cli-reference):

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

На этой странице рассматривается использование Agent SDK через CLI (`claude -p`). Для пакетов Python и TypeScript SDK со структурированными выходами, обратными вызовами одобрения инструментов и собственными объектами сообщений см. [полную документацию Agent SDK](/docs/ru/agent-sdk/overview).

<h2 id="basic-usage">
  Базовое использование
</h2>

Добавьте флаг `-p` (или `--print`) к любой команде `claude` для запуска её в неинтерактивном режиме. Все [параметры CLI](/docs/ru/cli-reference) работают с `-p`, включая:

* `--continue` для [продолжения разговоров](#continue-conversations)
* `--allowedTools` для [автоматического одобрения инструментов](#auto-approve-tools)
* `--output-format` для [структурированного вывода](#get-structured-output)

Этот пример задаёт Claude вопрос о вашей кодовой базе и выводит ответ:

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  Начните быстрее с режимом bare
</h3>

Добавьте `--bare` для сокращения времени запуска путём пропуска автоматического обнаружения hooks, skills, plugins, MCP серверов, автоматической памяти и CLAUDE.md. Без этого `claude -p` загружает тот же [контекст](/docs/ru/how-claude-code-works#the-context-window), что и интерактивная сессия, включая всё, что настроено в рабочем каталоге или `~/.claude`.

Режим bare полезен для CI и скриптов, где вам нужен одинаковый результат на каждой машине. Hook в `~/.claude` коллеги или MCP сервер в `.mcp.json` проекта не будут запущены, потому что режим bare никогда их не читает. Действуют только явно переданные флаги.

Этот пример запускает одноразовую задачу суммирования в режиме bare и предварительно одобряет инструмент Read, чтобы вызов завершился без запроса разрешения:

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

В режиме bare Claude имеет доступ к инструментам Bash, чтения файлов и редактирования файлов. Передайте любой необходимый контекст с флагом:

| Для загрузки                  | Используйте                                             |
| ----------------------------- | ------------------------------------------------------- |
| Дополнения системного запроса | `--append-system-prompt`, `--append-system-prompt-file` |
| Параметры                     | `--settings <file-or-json>`                             |
| MCP серверы                   | `--mcp-config <file-or-json>`                           |
| Пользовательские агенты       | `--agents <json>`                                       |
| Плагин                        | `--plugin-dir <path>`, `--plugin-url <url>`             |

Режим bare пропускает OAuth и чтение из связки ключей. Аутентификация Anthropic должна поступать из `ANTHROPIC_API_KEY` или `apiKeyHelper` в JSON, переданном в `--settings`. Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry используют обычные учётные данные поставщика.

<Note>
  `--bare` — это рекомендуемый режим для скриптовых и SDK вызовов, и он станет режимом по умолчанию для `-p` в будущем выпуске.
</Note>

<h3 id="background-tasks-at-exit">
  Фоновые задачи при выходе
</h3>

Если Claude запускает [фоновую задачу Bash](/docs/ru/tools-reference#bash-tool-behavior) во время выполнения `claude -p`, например сервер разработки или сборку с отслеживанием, эта задача завершается примерно через пять секунд после того, как Claude вернул свой окончательный результат и stdin закрыт. Период ожидания позволяет задаче, которая завершается сразу после результата, всё ещё доставить свой вывод. До версии v2.1.163 никогда не завершающийся фоновый процесс держал бы вызов `claude -p` открытым неопределённо долго.

Фоновые [подагенты](/docs/ru/sub-agents) и рабочие процессы освобождены от пятисекундного периода ожидания, потому что их результат является частью окончательного вывода, поэтому `claude -p` ждёт их завершения. Начиная с версии v2.1.182, это ожидание ограничено десятью минутами по умолчанию, чтобы застрявший фоновый агент не мог держать процесс открытым неопределённо долго. Отрегулируйте ограничение с помощью [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/ru/env-vars) или установите его на `0`, чтобы ждать без ограничений.

<h2 id="examples">
  Примеры
</h2>

Эти примеры выделяют общие паттерны CLI. Для CI и других скриптовых вызовов добавьте [`--bare`](#start-faster-with-bare-mode), чтобы они не подхватывали то, что случайно настроено локально.

<h3 id="pipe-data-through-claude">
  Передача данных через Claude
</h3>

Неинтерактивный режим читает stdin, поэтому вы можете передавать данные и перенаправлять ответ, как любой другой инструмент командной строки.

Этот пример передаёт журнал сборки в Claude и записывает объяснение в файл:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

С `--output-format json` полезная нагрузка ответа включает `total_cost_usd` и разбивку затрат по моделям, поэтому скриптовые вызывающие стороны могут отслеживать расходы на вызов без обращения к [панели использования](/docs/ru/costs).

<Note>
  Начиная с Claude Code v2.1.128, piped stdin ограничен 10MB. Если вы превысите лимит, Claude Code выходит с чётким сообщением об ошибке и ненулевым статусом. Для работы с большими входными данными запишите содержимое в файл и ссылайтесь на путь файла в вашем запросе вместо передачи через pipe.
</Note>

<h3 id="add-claude-to-a-build-script">
  Добавление Claude в скрипт сборки
</h3>

Вы можете обернуть неинтерактивный вызов в скрипт, чтобы использовать Claude как проектный линтер или рецензент.

Этот скрипт `package.json` передаёт diff относительно `main` в Claude и просит его сообщить об опечатках. Передача diff означает, что Claude не нуждается в разрешении Bash для его чтения, а экранированные двойные кавычки делают скрипт портативным для Windows:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

<h3 id="get-structured-output">
  Получение структурированного вывода
</h3>

Используйте `--output-format` для управления тем, как возвращаются ответы:

* `text` (по умолчанию): простой текстовый вывод
* `json`: структурированный JSON с результатом, ID сессии и метаданными
* `stream-json`: JSON с разделением по строкам для потоковой передачи в реальном времени

Этот пример возвращает сводку проекта в виде JSON с метаданными сессии, с текстовым результатом в поле `result`:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Чтобы получить вывод, соответствующий определённой схеме, используйте `--output-format json` с `--json-schema` и определением [JSON Schema](https://json-schema.org/). Ответ включает метаданные о запросе (ID сессии, использование и т.д.) со структурированным выводом в поле `structured_output`.

Этот пример извлекает имена функций и возвращает их как массив строк:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Если значение не является действительной JSON Schema, `claude` выходит с `Error: --json-schema is not a valid JSON Schema`, за которым следует диагностика валидатора. Claude Code принимает схемы, которые используют ключевое слово `format`, такие как `"format": "email"`, но рассматривает `format` как аннотацию и не применяет его. До версии v2.1.205 Claude Code молча игнорировал недействительную схему и возвращал неструктурированный текст, а также рассматривал любую схему, содержащую `format`, как недействительную.

<Tip>
  Используйте инструмент вроде [jq](https://jqlang.github.io/jq/) для анализа ответа и извлечения определённых полей:

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Потоковая передача ответов
</h3>

Используйте `--output-format stream-json` с `--verbose` и `--include-partial-messages` для получения токенов по мере их генерации. Каждая строка — это объект JSON, представляющий событие:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

Последняя строка потока — это сообщение `result` с финальным текстом ответа, стоимостью и метаданными сессии. До версии v2.1.208 передача большого ответа могла обрезать финальную строку и опустить сообщение `result`.

Следующий пример использует [jq](https://jqlang.github.io/jq/) для фильтрации текстовых дельт и отображения только потокового текста. Флаг `-r` выводит необработанные строки (без кавычек), а `-j` объединяет без новых строк, чтобы токены передавались непрерывно:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Когда запрос API завершается с повторяемой ошибкой, Claude Code выдаёт событие `system/api_retry` перед повторной попыткой. Вы можете использовать это для отображения прогресса повторной попытки или реализации пользовательской логики отката.

| Поле             | Тип                  | Описание                                                                                                                                                                                                 |
| ---------------- | -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`           | тип сообщения                                                                                                                                                                                            |
| `subtype`        | `"api_retry"`        | определяет это как событие повторной попытки                                                                                                                                                             |
| `attempt`        | целое число          | номер текущей попытки, начиная с 1                                                                                                                                                                       |
| `max_retries`    | целое число          | всего разрешённых повторных попыток                                                                                                                                                                      |
| `retry_delay_ms` | целое число          | миллисекунды до следующей попытки                                                                                                                                                                        |
| `error_status`   | целое число или null | код состояния HTTP или `null` для ошибок соединения без HTTP ответа                                                                                                                                      |
| `error`          | строка               | категория ошибки: `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` или `unknown` |
| `uuid`           | строка               | уникальный идентификатор события                                                                                                                                                                         |
| `session_id`     | строка               | сессия, к которой принадлежит событие                                                                                                                                                                    |

Событие `system/init` сообщает метаданные сессии, включая модель, инструменты, MCP серверы и загруженные плагины. Это первое событие в потоке, если не установлены события запуска:

* события `plugin_install`, когда установлена [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ru/env-vars).
* [события `hook_started`, `hook_progress` и `hook_response`](/docs/ru/agent-sdk/typescript#sdkhookstartedmessage), пока работает настроенный hook [`SessionStart`](/docs/ru/hooks#sessionstart) или [`Setup`](/docs/ru/hooks#setup). Они передаются потоком по мере их создания. Claude Code v2.1.169 через v2.1.203 доставлял их одной партией после завершения hook, всё ещё впереди `system/init`; v2.1.204 восстановил живую доставку.

Событие также содержит опциональный массив `capabilities` строк, называющих поведения протокола, которые реализует эта версия Claude Code, такие как `interrupt_receipt_v1`. Проверьте его для обнаружения функций вместо сравнения строк версий и игнорируйте значения, которые вы не распознаёте. Поле требует Claude Code v2.1.205 или позже и отсутствует в более ранних версиях. См. [`SDKSystemMessage`](/docs/ru/agent-sdk/typescript#sdksystemmessage) для списка возможностей.

Используйте поля плагина для отказа CI, когда плагин не загрузился:

| Поле            | Тип    | Описание                                                                                                                                                                                                                                                                                                        |
| --------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | массив | плагины, которые успешно загрузились, каждый с `name` и `path`                                                                                                                                                                                                                                                  |
| `plugin_errors` | массив | ошибки загрузки плагина, каждая с `plugin`, `type` и `message`. Включает неудовлетворённые версии зависимостей и ошибки загрузки `--plugin-dir`, такие как отсутствующий путь или недействительный архив. Затронутые плагины понижены в приоритете и отсутствуют в `plugins`. Ключ опускается, когда ошибок нет |

Когда установлена [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ru/env-vars), Claude Code выдаёт события `system/plugin_install` во время установки плагинов marketplace перед первым ходом. Используйте их для отображения прогресса установки в вашем собственном пользовательском интерфейсе.

| Поле         | Тип                                                      | Описание                                                                                                      |
| ------------ | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                               | тип сообщения                                                                                                 |
| `subtype`    | `"plugin_install"`                                       | определяет это как событие установки плагина                                                                  |
| `status`     | `"started"`, `"installed"`, `"failed"` или `"completed"` | `started` и `completed` охватывают общую установку; `installed` и `failed` сообщают об отдельных marketplaces |
| `name`       | строка, опционально                                      | имя marketplace, присутствует на `installed` и `failed`                                                       |
| `error`      | строка, опционально                                      | сообщение об ошибке, присутствует на `failed`                                                                 |
| `uuid`       | строка                                                   | уникальный идентификатор события                                                                              |
| `session_id` | строка                                                   | сессия, к которой принадлежит событие                                                                         |

Для программной потоковой передачи с обратными вызовами и объектами сообщений см. [Stream responses in real-time](/docs/ru/agent-sdk/streaming-output) в документации Agent SDK.

<h3 id="auto-approve-tools">
  Автоматическое одобрение инструментов
</h3>

Используйте `--allowedTools` для разрешения Claude использовать определённые инструменты без запроса. Этот пример запускает набор тестов и исправляет ошибки, позволяя Claude выполнять команды Bash и читать/редактировать файлы без запроса разрешения:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Чтобы установить базовый уровень для всей сессии вместо перечисления отдельных инструментов, передайте [режим разрешений](/docs/ru/permission-modes). `dontAsk` отклоняет всё, что не входит в ваши правила `permissions.allow` или [набор команд только для чтения](/docs/ru/permissions#read-only-commands), что полезно для заблокированных CI запусков. `AskUserQuestion`, инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и MCP инструменты, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), отклоняются даже когда правило разрешения совпадает.

`acceptEdits` позволяет Claude писать файлы без запроса и также автоматически одобряет общие команды файловой системы, такие как `mkdir`, `touch`, `mv` и `cp`. Другие команды оболочки и сетевые запросы по-прежнему требуют записи `--allowedTools` или правила `permissions.allow`, иначе запуск прерывается при попытке выполнить одну из них:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  Создание коммита
</h3>

Этот пример проверяет поставленные в очередь изменения и создаёт коммит с соответствующим сообщением:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Флаг `--allowedTools` использует [синтаксис правил разрешений](/docs/ru/settings#permission-rule-syntax). Завершающий ` *` включает сопоставление префиксов, поэтому `Bash(git diff *)` разрешает любую команду, начинающуюся с `git diff`. Пробел перед `*` важен: без него `Bash(git diff*)` также совпадал бы с `git diff-index`.

<Note>
  Вызываемые пользователем [skills](/docs/ru/skills) и пользовательские команды работают в режиме `-p`: включите `/skill-name` в строку запроса, и Claude Code развернёт её перед запуском. Встроенные команды, которые только работают в интерфейсе терминала, такие как `/login`, недоступны в режиме `-p`. `/model`, `/effort`, `/fast`, `/color` и `/rename` принимают значение как аргумент, например `/model sonnet`, и `/mcp` без аргумента выводит текстовую сводку статуса сервера; эти формы требуют Claude Code v2.1.205 или позже и следуют [примечаниям доступности](/docs/ru/commands#all-commands) каждой команды. Чтобы изменить параметр из вызова `-p`, передайте `key=value` в `/config`, например `/config thinking=false`.
</Note>

<h3 id="customize-the-system-prompt">
  Настройка системного запроса
</h3>

Используйте `--append-system-prompt` для добавления инструкций при сохранении поведения Claude Code по умолчанию. Этот пример передаёт diff PR в Claude и инструктирует его проверить на уязвимости безопасности:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

См. [флаги системного запроса](/docs/ru/cli-reference#system-prompt-flags) для получения дополнительных параметров, включая `--system-prompt` для полной замены запроса по умолчанию.

<h3 id="continue-conversations">
  Продолжение разговоров
</h3>

Используйте `--continue` для продолжения самого последнего разговора или `--resume` с ID сессии для продолжения определённого разговора. Этот пример запускает проверку, а затем отправляет дополнительные запросы:

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Если вы запускаете несколько разговоров, захватите ID сессии для возобновления определённого:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Запустите обе команды из одного каталога: поиск ID сессии ограничен текущим каталогом проекта и его git worktrees. См. [Resume a session](/docs/ru/sessions#resume-a-session) для полного набора правил области видимости.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Agent SDK quickstart](/docs/ru/agent-sdk/quickstart): создайте своего первого агента с помощью Python или TypeScript
* [CLI reference](/docs/ru/cli-reference): все флаги и параметры CLI
* [GitHub Actions](/docs/ru/github-actions): используйте Agent SDK в рабочих процессах GitHub
* [GitLab CI/CD](/docs/ru/gitlab-ci-cd): используйте Agent SDK в конвейерах GitLab
