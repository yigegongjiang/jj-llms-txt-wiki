> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Устранение неполадок Agent SDK

> Исправьте ошибки Agent SDK при сбое запуска Claude Code CLI, выходе процесса CLI или получении успешного результата без структурированного вывода.

Эта страница охватывает ошибки Agent SDK при запуске CLI, выходе процесса CLI и структурированных выводах. Записи на этой странице соответствуют ошибке, которую вы видите. Каждая указывает причину и что делать.

Симптомы, связанные с функцией, такие как неработающий hook или неиспользуемый skill, имеют раздел устранения неполадок на странице этой функции. В таблице указан раздел или страница, которая охватывает каждый симптом:

| Симптом                                                                                                                                                                                                                                                                                       | Перейти к                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| Skills не найдены, skill не используется, ошибка `Invalid skill name`                                                                                                                                                                                                                         | [Устранение неполадок Skills](/docs/ru/agent-sdk/skills#troubleshooting)                                                                    |
| MCP server показывает статус `failed`, tools не вызываются, timeout соединения, вывод tool превышает максимально допустимое количество токенов                                                                                                                                                | [Устранение неполадок MCP](/docs/ru/agent-sdk/mcp#troubleshooting)                                                                          |
| Plugin не загружается, skills плагина не отображаются                                                                                                                                                                                                                                         | [Устранение неполадок Plugins](/docs/ru/agent-sdk/plugins#troubleshooting)                                                                  |
| Claude не делегирует subagents, agents на основе файловой системы не загружаются                                                                                                                                                                                                              | [Устранение неполадок Subagents](/docs/ru/agent-sdk/subagents#troubleshooting)                                                              |
| Опции checkpointing не распознаны, пользовательские сообщения без UUID, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                              | [Устранение неполадок File checkpointing](/docs/ru/agent-sdk/file-checkpointing#troubleshooting)                                            |
| Hook не срабатывает, matcher не фильтрует как ожидается, timeout hook, tool заблокирован неожиданно, измененный input не применяется, session hooks недоступны в Python, prompts разрешения subagent умножаются, рекурсивные hook loops с subagents, `systemMessage` не отображается в выводе | [Исправление распространенных проблем](/docs/ru/agent-sdk/hooks#fix-common-issues) на странице hooks                                        |
| Agent, который работает на вашей машине, не работает в развернутом сервисе или контейнере                                                                                                                                                                                                     | [Устранение неполадок развертывания](/docs/ru/agent-sdk/hosting#troubleshoot-deployment-failures)                                           |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                            | [Справочник по ошибкам](/docs/ru/errors#find-your-error)                                                                                    |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` имеет значение `None`                                                                                                         | [Запуск CLI](#cli-startup), [Выход процесса CLI](#cli-process-exit) и [Структурированные выводы](#structured-outputs) на этой странице |

<h2 id="cli-startup">
  Запуск CLI
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code not found
</h3>

Python SDK запускает Claude Code CLI как подпроцесс. Когда он не может найти исполняемый файл `claude`, подключение завершается с ошибкой `CLINotFoundError`:

```
Claude Code not found at: /your/configured/path
```

Сообщение включает настроенный путь, когда вы устанавливаете `ClaudeAgentOptions(cli_path=...)` и он указывает на отсутствующий файл. Без `cli_path` SDK ищет в вашем `PATH` и в общих местах установки, и сообщение включает инструкции по установке для вашей платформы.

Чтобы исправить это:

* Установите Claude Code, если он не установлен. Смотрите [Установка Claude Code](/docs/ru/setup#install-claude-code) для команды на вашей платформе.
* Если вы установили `cli_path`, убедитесь, что файл существует и это исполняемый файл `claude`.
* Если вы полагаетесь на разрешение `PATH`, убедитесь, что `claude --version` работает в той же среде, в которой работает ваше приложение. Процессы, которые вы запускаете вне вашей оболочки, например из IDE или менеджера служб, часто работают с другим `PATH`.

TypeScript SDK ищет CLI в своем встроенном пакете платформы и в пути, который вы установили в `pathToClaudeCodeExecutable`. Сопоставьте сообщение, которое вы видите:

* `Native CLI binary for <platform>-<arch> not found`: встроенный пакет платформы отсутствует, чаще всего потому, что установка пропустила дополнительные зависимости. Переустановите `@anthropic-ai/claude-agent-sdk` без пропуска дополнительных зависимостей или укажите `pathToClaudeCodeExecutable` на [собственную установку](/docs/ru/setup#install-claude-code). В однофайловом исполняемом файле, созданном с помощью `bun build --compile`, то же сообщение имеет другую причину и исправление. Смотрите [Компиляция в один исполняемый файл](/docs/ru/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` или `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: файл по разрешённому пути отсутствует или процесс не может получить к нему доступ. Убедитесь, что файл существует по этому пути и что процесс может получить к нему доступ.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

В Windows подключение завершается с ошибкой `CLIConnectionError`, когда путь CLI, который использует Python SDK, является пакетным скриптом `.bat` или `.cmd`, включая прокладку `claude.cmd`, которую создаёт установка npm:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

Отказ является преднамеренным усилением безопасности, а не сломанной установкой. Windows запускает пакетные скрипты, переписывая spawn в вызов `cmd.exe /c`, и `cmd.exe` переанализирует всю командную строку во время выполнения, поэтому значение аргумента может выполнить внедрённые команды.

Большинство установок Windows никогда не достигают этой ошибки. Колесо Windows x64 `claude-agent-sdk` содержит `claude.exe`, и SDK предпочитает встроенный CLI, затем любой собственный `claude.exe`, который он может обнаружить, прежде чем вернуться к пакетной прокладке. Вы видите отказ в двух случаях:

* Вы установили `ClaudeAgentOptions(cli_path=...)` на файл `.bat` или `.cmd`, например прокладку npm `claude.cmd`.
* Ваша установка не имеет встроенного или собственного `claude.exe`, например исходная установка на ARM64 Windows, где единственный `claude` в вашем `PATH` — это прокладка npm.

Чтобы исправить это, дайте SDK собственный исполняемый файл вместо пакетного скрипта:

* Если вы установили `ClaudeAgentOptions(cli_path=...)`, укажите его на `claude.exe` или удалите опцию. SDK пропускает обнаружение, пока установлен `cli_path`, поэтому собственная установка одна не может вступить в силу.
* Установите Claude Code собственно в PowerShell: `irm https://claude.ai/install.ps1 | iex`
* На x64 Windows установите колесо `claude-agent-sdk`, которое содержит `claude.exe`.

До `claude-agent-sdk` 0.2.124 Python SDK запускал пакетные скрипты через `cmd.exe` без этой проверки.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

SDK нашёл файл по разрешённому пути, но не смог его запустить. Python вызывает эти сбои как `CLIConnectionError`. TypeScript отклоняет итерацию сообщения с ошибкой, не имеющей класса SDK. Таблица ниже сопоставляет каждое сообщение с тем, что оно вам говорит. Сопоставьте сообщение, которое вы видите:

| Сообщение                                                         | SDK        | Что это вам говорит                                                           |
| ----------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | Остаток сообщения — это собственная ошибка операционной системы               |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | Скрипт по настроенному пути не может работать                                 |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | Двоичный файл не может работать, с предложением libc, добавленным к сообщению |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Любой другой сбой запуска                                                     |

В обоих SDK обычная причина — разрешённый путь, который указывает на что-то, что не может работать, например текстовый файл, каталог или файл без разрешения на выполнение. Прочитайте предложение libc сообщения о собственном двоичном файле как одну возможную причину.

Чтобы исправить это в любом SDK:

* Убедитесь, что настроенный путь указывает на сам исполняемый файл `claude` и что файл имеет разрешение на выполнение.
* Если вам не нужен пользовательский путь, удалите `cli_path` в Python или `pathToClaudeCodeExecutable` в TypeScript, чтобы SDK нашёл CLI самостоятельно, предпочитая его встроенную копию.
* Когда неработающий двоичный файл — это встроенная копия SDK в образе контейнера, переустановите SDK во время сборки образа, чтобы встроенный двоичный файл соответствовал платформе контейнера, или перестройте образ для архитектуры, на которой он работает. Обычная причина — двоичный файл, который не соответствует архитектуре или libc контейнера, или файл, который потерял разрешение на выполнение при сборке образа.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Вызов метода `ClaudeSDKClient` в Python до подключения клиента или после его отключения вызывает `CLIConnectionError` с этим сообщением:

```
Not connected. Call connect() first.
```

Сделайте то, что говорит сообщение. Либо вызовите `await client.connect()` перед любым другим методом клиента, либо откройте клиент с помощью `async with ClaudeSDKClient() as client:`, который подключается при входе.

<h2 id="cli-process-exit">
  Выход процесса CLI
</h2>

Записи в этом разделе означают, что процесс Claude Code завершился, пока ваше приложение его использовало. Какую ошибку вы видите, зависит от языка SDK и от того, сообщил ли CLI об ошибке перед выходом.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

Python SDK вызывает `ProcessError`, когда процесс Claude Code завершается с ненулевым кодом:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

Сообщение указывает код выхода дважды, а строка `Error output` — это фиксированный текст, а не вывод ошибки вашего процесса. Тот же фиксированный текст заполняет атрибут `stderr` исключения. Атрибут `exit_code` исключения содержит код. Чтобы захватить то, что CLI фактически написал в stderr, передайте обратный вызов `stderr` в `ClaudeAgentOptions` и логируйте то, что он получает.

Простой `ProcessError` означает, что CLI завершился без сообщения об ошибке. Когда CLI сообщил об ошибке, SDK вместо этого вызывает [`ResultError`](/docs/ru/agent-sdk/python#resulterror), рассмотренный в [Claude Code returned an error result](#claude-code-returned-an-error-result). `ResultError` является подклассом `ProcessError`, поэтому `except ProcessError` ловит оба. Чтобы обработать их по-разному, поместите предложение `except ResultError` первым.

До `claude-agent-sdk` 0.2.140 Python SDK вызывал выходы с результатом ошибки как простое `Exception` вместо `ResultError`.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

Обёртки IDE также печатают это сообщение, и [справочник ошибок](/docs/ru/errors#claude-code-process-exited-with-code-n) охватывает его для VS Code и других средств запуска. Эта запись охватывает то, что получает ваш код TypeScript SDK. SDK отображает ненулевой выход CLI как простую `Error`, которая отклоняет цикл `for await` над сообщениями `query()`. Нет класса ошибки SDK для перехвата, поэтому оберните цикл в `try`/`catch` и сопоставьте сообщение:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Когда CLI писал в stderr, сообщение заканчивается хвостом этого. Чтобы захватить полный поток, передайте обратный вызов `stderr` в параметры запроса. Процесс, убитый сигналом, сообщает `Claude Code process terminated by signal <name>` в той же форме.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

Оба SDK заменяют ошибку выхода процесса этим сообщением, когда CLI сообщил об ошибке перед выходом:

```
Claude Code returned an error result: <the CLI's own error report>
```

Текст после двоеточия — это отчёт CLI о том, что пошло не так, поэтому начните с него, а не с самого выхода. Python вызывает это как [`ResultError`](/docs/ru/agent-sdk/python#resulterror), атрибут `data` которого содержит полный результат ошибки. TypeScript отклоняет цикл сообщений простой `Error`, несущей ту же форму сообщения.

<h2 id="structured-outputs">
  Структурированные выходы
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output is None but the result says success
</h3>

Сообщение результата может заканчиваться `subtype: "success"`, пока `structured_output` равен `None` в Python или `undefined` в TypeScript. Запуск завершается, но проверенный выход не существует. Один из способов попасть в это — схема, которую не может удовлетворить никакой выход, например конфликтующие ограничения длины. Запуск завершается без ошибки валидации, и единственный сигнал — отсутствующий `structured_output`.

Рассматривайте этот результат как сбой в коде приложения. Проверьте как то, что `subtype` равен `success`, так и то, что `structured_output` присутствует, прежде чем использовать его. Раздел [Error handling](/docs/ru/agent-sdk/structured-outputs#error-handling) показывает этот паттерн для обоих SDK.

Если это происходит повторно со схемой, которую вы считаете правильной, проверьте, что схема удовлетворяема, затем упростите её до тех пор, пока выходы не будут проверены, и переинтродуцируйте ограничения по одному.

<h2 id="report-a-new-issue">
  Сообщить о новой проблеме
</h2>

Если ваша ошибка не охвачена здесь, проверьте открытые проблемы или создайте новую в репозиториях SDK: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) или [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Включите полный текст ошибки и версию вашего SDK.
