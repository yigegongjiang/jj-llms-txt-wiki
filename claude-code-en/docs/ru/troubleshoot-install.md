> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Устранение неполадок при установке и входе

> Исправьте ошибки command not found, PATH, разрешений, сети и аутентификации при установке или входе в Claude Code.

Если установка не удалась или вы не можете войти, найдите вашу ошибку ниже. Для проблем во время выполнения после того, как Claude Code работает, см. [Troubleshooting](/docs/ru/troubleshooting). Для проблем конфигурации, таких как неприменение параметров или неработающие hooks, см. [Debug your configuration](/docs/ru/debug-your-config).

<h2 id="find-your-error">
  Найдите вашу ошибку
</h2>

Сопоставьте сообщение об ошибке или симптом, который вы видите, с исправлением:

| Что вы видите                                                                                              | Решение                                                                                                                                          |
| :--------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` или `'claude' is not recognized`                                               | [Исправьте ваш PATH](#command-not-found-claude-after-installation)                                                                               |
| `syntax error near unexpected token '<'`                                                                   | [Install script returns HTML](#install-script-returns-html-instead-of-a-shell-script)                                                            |
| `curl: (22) The requested URL returned error: 403`                                                         | [Install script returned 403](#install-script-returns-html-instead-of-a-shell-script)                                                            |
| `curl: (23)` или `curl: (56) Failure writing output to destination`                                        | [Проверьте подключение или используйте альтернативный установщик](#curl-56-failure-writing-output-to-destination)                                |
| `Killed` во время установки на Linux, или `Installation was killed before it could finish (exit code 137)` | [Освободите память или добавьте пространство подкачки](#install-killed-on-low-memory-linux-servers)                                              |
| `TLS connect error` или `SSL/TLS secure channel`                                                           | [Обновите сертификаты CA](#tls-or-ssl-connection-errors)                                                                                         |
| `Failed to fetch version` или невозможно достичь сервера загрузки                                          | [Проверьте параметры сети и прокси](#check-network-connectivity)                                                                                 |
| `irm is not recognized` или `&& is not valid`                                                              | [Используйте правильную команду для вашей оболочки](#wrong-install-command-on-windows)                                                           |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                         | [Обновите Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                                      |
| `'bash' is not recognized as the name of a cmdlet`                                                         | [Используйте команду установщика Windows](#wrong-install-command-on-windows)                                                                     |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                           | [Используйте команду установщика Windows](#wrong-install-command-on-windows)                                                                     |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                          | [Установите оболочку](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                            |
| `Claude Code does not support 32-bit Windows`                                                              | [Откройте Windows PowerShell, а не запись x86](#claude-code-does-not-support-32-bit-windows)                                                     |
| `The process cannot access the file ... because it is being used by another process`                       | [Очистите папку загрузок и повторите попытку](#the-process-cannot-access-the-file-during-windows-install)                                        |
| `Error loading shared library`                                                                             | [Неправильный вариант двоичного файла для вашей системы](#linux-musl-or-glibc-binary-mismatch)                                                   |
| `Illegal instruction`                                                                                      | [Несоответствие архитектуры или набора инструкций процессора](#illegal-instruction)                                                              |
| `cannot execute binary file: Exec format error` в WSL                                                      | [WSL1 native-binary regression](#exec-format-error-on-wsl1)                                                                                      |
| Установщик PowerShell завершается, но `claude` не найден или показывает старую версию                      | [Добавьте каталог установки в ваш PATH](#verify-your-path), затем откройте новый терминал                                                        |
| `dyld: cannot load`, `dyld: Symbol not found` или `Abort trap` на macOS                                    | [Несовместимость двоичного файла](#dyld-cannot-load-on-macos)                                                                                    |
| `Invoke-Expression: Missing argument in parameter list`                                                    | [Install script returns HTML](#install-script-returns-html-instead-of-a-shell-script)                                                            |
| `App unavailable in region`                                                                                | Claude Code недоступен в вашей стране. См. [поддерживаемые страны](https://www.anthropic.com/supported-countries).                               |
| `unable to get local issuer certificate`                                                                   | [Настройте корпоративные сертификаты CA](#tls-or-ssl-connection-errors)                                                                          |
| `OAuth error` или `403 Forbidden`                                                                          | [Исправьте аутентификацию](#login-and-authentication)                                                                                            |
| `Could not load the default credentials` или `Could not load credentials from any providers`               | [Учетные данные Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` или `CredentialUnavailableError`                            | [Учетные данные Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429` или другие ошибки 4xx и 5xx, не указанные выше                   | См. [справочник ошибок](/docs/ru/errors)                                                                                                              |

Если вашей проблемы нет в списке, выполните диагностические проверки ниже, чтобы сузить причину.

<Tip>
  Если вы предпочитаете полностью избежать терминала, [Claude Code Desktop app](/docs/ru/desktop-quickstart) позволяет вам установить и использовать Claude Code через графический интерфейс. Загрузите его для [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) или [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) и начните кодировать без какой-либо настройки командной строки. На Linux установите приложение с помощью apt, следуя [инструкциям по установке Linux](/docs/ru/desktop-linux).
</Tip>

<h2 id="run-diagnostic-checks">
  Запустите диагностические проверки
</h2>

<h3 id="check-network-connectivity">
  Проверьте подключение к сети
</h3>

Установщик загружает с `downloads.claude.ai`. Убедитесь, что вы можете его достичь:

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

В PowerShell запустите `curl.exe -sI` вместо этого. PowerShell создаёт псевдоним `curl` на `Invoke-WebRequest`, который отклоняет флаги `-sI`.

Строка `HTTP/2 200` означает, что вы достигли сервера. Если вы видите отсутствие вывода, `Could not resolve host` или timeout соединения, ваша сеть блокирует соединение. Распространённые причины:

* Корпоративные брандмауэры или прокси, блокирующие `downloads.claude.ai`
* Региональные ограничения сети: попробуйте VPN или альтернативную сеть
* Проблемы TLS/SSL: обновите сертификаты CA вашей системы или проверьте, настроен ли `HTTPS_PROXY`

Если вы находитесь за корпоративным прокси, установите `HTTPS_PROXY` и `HTTP_PROXY` на адрес вашего прокси перед установкой. Попросите URL прокси у вашей IT-команды, если вы его не знаете, или проверьте параметры прокси вашего браузера.

Этот пример устанавливает обе переменные прокси, а затем запускает установщик через ваш прокси:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Проверьте ваш PATH
</h3>

Если установка прошла успешно, но вы получаете ошибку `command not found` или `not recognized` при запуске `claude`, директория установки не находится в вашем PATH. Ваша оболочка ищет программы в директориях, указанных в PATH, и установщик размещает `claude` в `~/.local/bin/claude` на macOS/Linux или `%USERPROFILE%\.local\bin\claude.exe` на Windows.

<Note>
  Расширение [VS Code](/docs/ru/vs-code) не размещает `claude` в этом месте. Оно содержит приватную копию CLI внутри директории расширения для своей собственной панели чата и не добавляет её в PATH. Если вы установили только расширение, `~/.local/bin/claude` не будет существовать. Запустите [автономную установку](/docs/ru/setup), чтобы использовать `claude` из терминала, а затем продолжите ниже.
</Note>

Проверьте, находится ли директория установки в вашем PATH, перечислив записи PATH и фильтруя по `local/bin`:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Если это выводит `/Users/you/.local/bin` или `/home/you/.local/bin`, директория находится в вашем PATH и вы можете перейти к [Проверьте наличие конфликтующих установок](#check-for-conflicting-installations). Если вывода нет, добавьте её в конфигурацию вашей оболочки.

    Для Zsh, по умолчанию на macOS:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Для Bash, по умолчанию на большинстве дистрибутивов Linux:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    Или закройте и снова откройте ваш терминал.

    Для других оболочек, таких как fish или Nushell, добавьте `~/.local/bin` в ваш PATH, используя синтаксис конфигурации вашей оболочки, а затем перезагрузите ваш терминал.

    Проверьте, что исправление сработало:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    Если вывода нет, добавьте директорию установки в ваш User PATH:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Перезагрузите ваш терминал, чтобы изменение вступило в силу.

    Проверьте, что исправление сработало:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    Если вывода нет, откройте System Settings, перейдите в Environment Variables и добавьте `%USERPROFILE%\.local\bin` в вашу переменную User PATH. Перезагрузите ваш терминал.

    Проверьте, что исправление сработало:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Проверьте наличие конфликтующих установок
</h3>

Несколько установок Claude Code могут вызвать несоответствия версий или неожиданное поведение. Проверьте, что установлено:

<Tabs>
  <Tab title="macOS/Linux">
    Перечислите все бинарные файлы `claude`, найденные в вашем PATH:

    ```bash theme={null}
    which -a claude
    ```

    Если это ничего не выводит, `claude` ещё не находится в вашем PATH. Вернитесь к [Проверьте ваш PATH](#verify-your-path).

    Проверьте три места, откуда может поступить бинарный файл `claude`. `~/.local/bin/claude` — это встроенный установщик, `~/.claude/local/` — это устаревшая локальная установка npm, созданная старыми версиями Claude Code, и список глобального npm показывает установку `-g`:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Встроенная установка показывает символическую ссылку в `~/.local/share/claude/versions/`. Скрипт или символическая ссылка, которую вы создали сами в этом пути, — это пользовательский запускатель, который [автоматическое обновление оставляет на месте](/docs/ru/setup#auto-updates).

    Если какая-либо команда `ls` выводит `No such file or directory`, это не ошибка. Это означает, что ничего не установлено в этом месте, поэтому переходите к следующей проверке.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Перечислите все бинарные файлы `claude`, найденные в вашем PATH:

    ```powershell theme={null}
    where.exe claude
    ```

    Проверьте, разместил ли встроенный установщик бинарный файл:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Если вы найдёте несколько установок, оставьте только одну. Встроенная установка в `~/.local/bin/claude` на macOS/Linux или `%USERPROFILE%\.local\bin\claude.exe` на Windows рекомендуется. Удалите лишние:

Удалите глобальную установку npm:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Удалите устаревшую локальную установку npm:

```bash theme={null}
rm -rf ~/.claude/local
```

На Windows используйте PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Удалите установку Homebrew на macOS. Если вы установили кэш `claude-code@latest`, замените это имя:

```bash theme={null}
brew uninstall --cask claude-code
```

Удалите установку WinGet на Windows:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Проверьте разрешения директорий
</h3>

Установщику нужен доступ на запись в `~/.local/bin/` и `~/.claude/` на macOS и Linux. На Windows место установки находится под `%USERPROFILE%`, которое по умолчанию доступно для записи вашим пользователем, поэтому этот раздел редко применяется там.

Проверьте, доступны ли директории для записи:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Если какая-либо директория недоступна для записи, создайте директорию установки и установите вашего пользователя в качестве владельца:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Проверьте, работает ли бинарный файл
</h3>

Если `claude --version` выводит версию, но `claude` падает или зависает при запуске, запустите эти проверки, чтобы сузить причину. Если `claude --version` говорит command not found, сначала перейдите к [Проверьте ваш PATH](#verify-your-path); команды ниже предполагают, что `claude` находится в вашем PATH.

Подтвердите, что бинарный файл существует и исполняемый:

```bash theme={null}
ls -la "$(command -v claude)"
```

На Windows используйте PowerShell:

```powershell theme={null}
Get-Command claude | Select-Object Source
```

На Linux проверьте отсутствующие общие библиотеки. Если `ldd` показывает отсутствующие библиотеки, вам может потребоваться установить системные пакеты. На Alpine Linux и других дистрибутивах на основе musl см. [Alpine Linux setup](/docs/ru/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Подтвердите, что бинарный файл может выполняться:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Распространённые проблемы установки
</h2>

Это наиболее часто встречающиеся проблемы установки и их решения.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Install script returns HTML instead of a shell script
</h3>

При запуске команды установки вы можете увидеть одну из этих ошибок:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

На PowerShell та же проблема выглядит как:

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

В зависимости от того, как был маршрутизирован запрос, вы можете вместо этого увидеть 403 без HTML-тела:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Всё это означает, что URL установки вернул HTML-страницу или статус ошибки вместо скрипта установки. Если HTML-страница говорит "App unavailable in region", Claude Code недоступен в вашей стране. См. [поддерживаемые страны](https://www.anthropic.com/supported-countries).

Простой 403 без тела часто имеет ту же причину, но это также может быть вызвано корпоративным прокси или брандмауэром, блокирующим загрузку. Если вы находитесь в поддерживаемой стране и всё ещё видите 403, пройдите через [Проверка подключения к сети](#check-network-connectivity) перед попыткой альтернативных установщиков ниже, так как они достигают тех же хостов.

В противном случае это может произойти из-за проблем с сетью, региональной маршрутизации или временного сбоя сервиса.

**Решения:**

1. **Используйте альтернативный метод установки**:

   На macOS установите через Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   На Windows установите через WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Повторите попытку через несколько минут**: проблема часто временная. Подождите и попробуйте исходную команду снова.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` after installation
</h3>

Установка завершилась, но `claude` не работает. Точная ошибка варьируется в зависимости от платформы:

| Платформа   | Сообщение об ошибке                                                    |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Это означает, что директория установки не находится в пути поиска вашей оболочки. См. [Проверка вашего PATH](#verify-your-path) для исправления на каждой платформе.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

Команда `curl ... | bash` загружает скрипт и передаёт его в Bash для выполнения. Эта ошибка и связанная с ней `curl: (23) Failure writing output to destination` означают, что Bash не получил полный скрипт. Код выхода 56 указывает, что сама загрузка была прервана, а код выхода 23 указывает, что curl не смог записать полученное в канал, обычно потому что Bash завершился рано.

**Решения:**

1. **Проверьте стабильность сети**: бинарные файлы Claude Code размещены на `downloads.claude.ai`. Проверьте, что вы можете его достичь:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Строка `HTTP/2 200` означает, что вы достигли сервера и исходный сбой был вероятно временным; повторите команду установки. Если вы видите `Could not resolve host` или timeout соединения, ваша сеть блокирует загрузку.

2. **Попробуйте альтернативный метод установки**:

   На macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   На Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Homebrew cask unavailable or outdated
</h3>

Homebrew сообщает `Error: Cask 'claude-code' is unavailable: No Cask with this name exists`, когда ваша локальная копия индекса Homebrew cask предшествует публикации cask. Обновите индекс и повторите попытку:

```bash theme={null}
brew update
brew install --cask claude-code
```

Если Homebrew устанавливает более старую версию Claude Code, чем вы ожидаете, обычно причина в том же устаревшем индексе. Cask `claude-code` отслеживает стабильный канал и обычно отстаёт на неделю от последнего выпуска; для самой новой версии запустите вместо этого `brew install --cask claude-code@latest`. См. [Настройка канала выпуска](/docs/ru/setup#configure-release-channel) для различия между двумя cask.

<h3 id="tls-or-ssl-connection-errors">
  TLS or SSL connection errors
</h3>

Ошибки вроде `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed` или PowerShell's `Could not establish trust relationship for the SSL/TLS secure channel` указывают на сбои TLS handshake.

**Решения:**

1. **Обновите сертификаты CA вашей системы**:

   На Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   На macOS системный curl использует хранилище доверия Keychain; обновление самого macOS обновляет корневые сертификаты.

2. **На Windows включите TLS 1.2** в PowerShell перед запуском установщика:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Проверьте помехи прокси или брандмауэра**: корпоративные прокси, выполняющие TLS inspection, могут вызвать эти ошибки, включая `unable to get local issuer certificate` и `SELF_SIGNED_CERT_IN_CHAIN`. Для шага установки укажите curl на ваш корпоративный пакет CA с `--cacert`:
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Для самого Claude Code после установки установите `NODE_EXTRA_CA_CERTS` так, чтобы запросы API доверяли тому же пакету:
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Попросите файл сертификата у вашей IT-команды, если у вас его нет. Вы также можете попробовать на прямом соединении, чтобы подтвердить, что прокси является причиной.

4. **На Windows переключитесь на установщики, если ваша сеть блокирует проверки отзыва**. Ошибки `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` и `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` означают, что curl достиг сервера, но ваша сеть блокирует поиск отзыва сертификата, что распространено за корпоративными брандмауэрами. Добавление флага `--ssl-revoke-best-effort` к curl не исправляет это: флаг применяется только к загрузке самого `install.cmd`, а собственные загрузки скрипта работают без него, поэтому установка не удаётся с той же ошибкой. Используйте метод установки, который допускает заблокированный поиск. Откройте PowerShell и запустите установщик PowerShell, который загружается через .NET и не падает, когда сервер отзыва недоступен:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Вы также можете установить с `winget install Anthropic.ClaudeCode`, что полностью избегает curl.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

Установщик не смог достичь сервера загрузки. Это обычно означает, что `downloads.claude.ai` заблокирован в вашей сети.

**Решения:**

1. **Проверьте подключение напрямую**:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Если за прокси**, установите `HTTPS_PROXY` так, чтобы установщик мог маршрутизировать через него. См. [конфигурация прокси](/docs/ru/network-config#proxy-configuration) для деталей.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Если в ограниченной сети**, попробуйте другую сеть или VPN, или используйте альтернативный метод установки:

   На macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   На Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Wrong install command on Windows
</h3>

Если вы видите `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'` или `'bash' is not recognized as the name of a cmdlet`, вы скопировали команду установки для другой оболочки или операционной системы.

* **`irm` не распознан**: вы находитесь в CMD, а не PowerShell. У вас есть два варианта:

  Откройте PowerShell, поиск "PowerShell" в меню Start, затем запустите исходную команду установки:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Или оставайтесь в CMD и используйте вместо этого установщик CMD:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` не действителен**: вы находитесь в PowerShell, но запустили команду установщика CMD. Используйте установщик PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: вы запустили установщик macOS/Linux `curl -fsSL ... | bash` в Windows PowerShell, где `curl` является псевдонимом для `Invoke-WebRequest` и отклоняет флаги `-fsSL`. Используйте вместо этого установщик PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` не распознан**: вы запустили установщик macOS/Linux на Windows. Используйте вместо этого установщик PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` during Windows install
</h3>

Если установщик PowerShell не удаётся с `Failed to download binary: The process cannot access the file ... because it is being used by another process`, установщик не смог записать в `%USERPROFILE%\.claude\downloads`. Это обычно означает, что предыдущая попытка установки всё ещё работает, или антивирусное программное обеспечение сканирует частично загруженный бинарный файл в этой папке.

Закройте любые другие окна PowerShell, запускающие установщик, и дождитесь завершения сканирования антивирусом. Затем удалите папку загрузок и запустите установщик снова:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  Install killed on low-memory Linux servers
</h3>

Сообщение `Killed` во время установки обычно означает, что убийца Linux out-of-memory (OOM) завершил шаг `claude install`, потому что система исчерпала свободную память. Это распространено на небольших VPS и облачных экземплярах. Скрипт установки сообщает причину и выходит с кодом 137:

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

До v2.1.200 скрипт выходил только с голой строкой `Killed` оболочки и без объяснения.

Установка требует примерно 512 МБ свободной памяти, а запуск Claude Code требует больше. См. [системные требования](/docs/ru/setup#system-requirements).

**Решения:**

1. **Добавьте пространство подкачки**, если ваш сервер имеет ограниченную оперативную память. Подкачка использует дисковое пространство как переполнение памяти, позволяя установке завершиться даже при низкой физической оперативной памяти.

   Создайте файл подкачки размером 2 ГБ и включите его:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Затем повторите установку:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Закройте другие процессы**, чтобы освободить память перед установкой.

3. **Используйте больший экземпляр**, если возможно. Claude Code требует по крайней мере 4 ГБ оперативной памяти.

<h3 id="install-hangs-in-docker">
  Install hangs in Docker
</h3>

При установке Claude Code в контейнер Docker установка от root в `/` может вызвать зависания.

**Решения:**

1. **Установите рабочую директорию** перед запуском установщика. При запуске из `/` установщик сканирует всю файловую систему, что вызывает чрезмерное использование памяти. Установка `WORKDIR` ограничивает сканирование небольшой директорией:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Увеличьте лимиты памяти Docker**, если используете Docker Desktop:
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop overrides the `claude` command on Windows
</h3>

Если вы установили старую версию Claude Desktop, она может зарегистрировать `Claude.exe` в директории `WindowsApps`, которая имеет приоритет PATH над Claude Code CLI. Запуск `claude` открывает приложение Desktop вместо CLI.

Обновите Claude Desktop до последней версии, чтобы исправить эту проблему.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code on Windows requires either Git for Windows (for bash) or PowerShell
</h3>

Git for Windows является опциональным. Claude Code использует [PowerShell tool](/docs/ru/tools-reference#powershell-tool) при отсутствии Git Bash, поэтому эта ошибка означает, что ни одна оболочка не была найдена.

**Если PowerShell отсутствует в вашем PATH**, его местоположение по умолчанию — `C:\Windows\System32\WindowsPowerShell\v1.0\`. Добавьте эту директорию в ваш `PATH`, или установите [PowerShell 7](https://aka.ms/powershell), который предоставляет `pwsh`.

**Чтобы вместо этого установить Git for Windows**, загрузите его с [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Во время установки выберите "Add to PATH." Перезагрузите ваш терминал после установки. Установка его включает инструмент Bash, полезный при работе со скриптами и инструментами на основе Bash.

**Если Git уже установлен**, но Claude Code не может его найти, установите путь в вашем [settings.json file](/docs/ru/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Если ваш Git установлен где-то ещё, найдите путь, запустив `where.exe git` в PowerShell и используйте путь `bin\bash.exe` из этой директории.

**Если путь правильный и файл существует**, но Claude Code всё ещё сообщает, что он не найден, программное обеспечение безопасности конечной точки, такое как AppLocker, политики ограничения программного обеспечения Group Policy или агенты EDR, могут вмешиваться. На версиях до v2.1.116 Claude Code порождал дочерний процесс (`cmd.exe`) для проверки пути, который эти политики могут блокировать — распространённый сигнал заключается в том, что `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` работает, когда вы запускаете его непосредственно в PowerShell, но молча не удаётся при запуске `claude.exe`.

Claude Code v2.1.116 и позже проверяют файловую систему напрямую, поэтому сначала обновитесь. Если ошибка сохраняется на текущей версии, попросите вашу IT-команду добавить в список разрешений `claude.exe` и процессы, которые он порождает, включая `cmd.exe` и `bash.exe`, в вашей политике защиты конечной точки.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code does not support 32-bit Windows
</h3>

Windows включает две записи PowerShell в меню Start: `Windows PowerShell` и `Windows PowerShell (x86)`. Запись x86 запускается как 32-битный процесс и вызывает эту ошибку даже на 64-битной машине. Чтобы проверить, в каком случае вы находитесь, запустите это в том же окне, которое произвело ошибку:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Если это выводит `True`, ваша операционная система в порядке. Закройте окно, откройте `Windows PowerShell` без суффикса x86 и запустите команду установки снова.

Если это выводит `False`, вы находитесь на 32-битном издании Windows. Claude Code требует 64-битную операционную систему. См. [системные требования](/docs/ru/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Linux musl or glibc binary mismatch
</h3>

Если вы видите ошибки об отсутствующих общих библиотеках вроде `libstdc++.so.6` или `libgcc_s.so.1` после установки, установщик мог загрузить неправильный вариант бинарного файла для вашей системы.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Это может произойти на системах на основе glibc, которые имеют установленные пакеты кросс-компиляции musl, вызывая установщик неправильно определить систему как musl.

**Решения:**

1. **Проверьте, какой libc использует ваша система**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   Вывод, упоминающий `GNU libc` или `GLIBC`, означает glibc. Вывод, упоминающий `musl`, означает musl.

2. **Если вы на glibc, но получили бинарный файл musl**, удалите установку и переустановите. Вы также можете вручную загрузить правильный бинарный файл, используя манифест в `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. Подайте [GitHub issue](https://github.com/anthropics/claude-code/issues) с выводом `ldd --version` и `ls /lib/libc.musl*`.

3. **Если вы действительно на musl**, такой как Alpine Linux, установите требуемые пакеты:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Если запуск `claude` или установщика выводит `Illegal instruction`, встроенный бинарный файл использует инструкции CPU, которые ваш процессор не поддерживает. Есть две отдельные причины.

**Несоответствие архитектуры.** Установщик загрузил неправильный бинарный файл, например x86 на ARM-сервере. Проверьте с `uname -m` на macOS или Linux, или `$env:PROCESSOR_ARCHITECTURE` в PowerShell. Если результат не совпадает с полученным вами бинарным файлом, [подайте GitHub issue](https://github.com/anthropics/claude-code/issues) с выводом.

**Отсутствующий набор инструкций AVX.** Если ваша архитектура правильная, но вы всё ещё видите `Illegal instruction`, ваш CPU вероятно не имеет AVX или другой инструкции, которую требует бинарный файл. Это влияет примерно на процессоры Intel и AMD до 2013 года, и виртуальные машины, где гипервизор не передаёт AVX гостю.

На VPS или VM запустите `grep -m1 -ow avx /proc/cpuinfo`; пустой результат означает, что AVX недоступен гостю.

Встроенного обходного пути нет; отслеживайте [issue #50384](https://github.com/anthropics/claude-code/issues/50384) для статуса и включайте модель вашего CPU из `grep -m1 "model name" /proc/cpuinfo` на Linux или `sysctl -n machdep.cpu.brand_string` на macOS при сообщении.

Альтернативные методы установки загружают тот же встроенный бинарный файл и не разрешат ни одну из причин.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` on macOS
</h3>

Если вы видите `dyld: cannot load`, `dyld: Symbol not found` или `Abort trap: 6` во время установки, бинарный файл несовместим с вашей версией macOS или оборудованием.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Ошибка `Symbol not found`, которая ссылается на `libicucore`, также указывает, что ваша версия macOS старше, чем поддерживает бинарный файл:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Решения:**

1. **Проверьте вашу версию macOS**: Claude Code требует macOS 13.0 или позже. Откройте меню Apple и выберите About This Mac, чтобы проверить вашу версию.

2. **Обновите macOS**, если вы на старой версии. Бинарный файл использует команды загрузки и системные библиотеки, которые старые версии macOS не поддерживают. Альтернативные методы установки, такие как Homebrew, загружают тот же бинарный файл и не разрешат эту ошибку.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` on WSL1
</h3>

Если запуск `claude` в WSL выводит `cannot execute binary file: Exec format error`, вы находитесь на WSL1 и попадаете в известную регрессию встроенного бинарного файла, отслеживаемую в [issue #38788](https://github.com/anthropics/claude-code/issues/38788). Заголовки программы бинарного файла изменились таким образом, что загрузчик WSL1 не может обработать.

Самое чистое исправление — преобразовать ваш дистрибутив в WSL2 из PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Если вам нужно оставаться на WSL1, вызовите бинарный файл через динамический компоновщик. Добавьте эту функцию в `~/.bashrc` внутри WSL, заменив путь, если ваша домашняя директория отличается:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Затем запустите `source ~/.bashrc` и повторите `claude`.

<h3 id="npm-install-errors-in-wsl">
  npm install errors in WSL
</h3>

Эти проблемы применяются, если вы установили Claude Code с `npm install -g` внутри WSL. Если вы использовали [native installer](/docs/ru/setup), пропустите этот раздел.

**Проблемы обнаружения ОС или платформы.** Если npm сообщает о несоответствии платформы во время установки, WSL вероятно выбирает Windows `npm`. Сначала запустите `npm config set os linux`, затем установите с `npm install -g @anthropic-ai/claude-code --force`. Не используйте `sudo`.

**`exec: node: not found` при запуске `claude`.** Ваша среда WSL вероятно использует установку Node.js для Windows. Подтвердите с `which npm` и `which node`: пути, начинающиеся с `/mnt/c/`, — это бинарные файлы Windows, в то время как пути Linux начинаются с `/usr/`. Чтобы исправить это, установите Node через менеджер пакетов вашего дистрибутива Linux или через [`nvm`](https://github.com/nvm-sh/nvm).

**Конфликты версий nvm.** Если у вас установлен nvm как в WSL, так и в Windows, переключение версий Node в WSL может сломаться, потому что WSL импортирует Windows PATH по умолчанию и Windows nvm имеет приоритет. Наиболее распространённая причина — что nvm не загружен в вашу оболочку. Добавьте загрузчик nvm в `~/.bashrc` или `~/.zshrc`:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

Или загрузите его в вашу текущую сессию:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Если nvm загружен, но пути Windows всё ещё имеют приоритет, явно добавьте ваш путь Linux Node:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Избегайте отключения импорта Windows PATH через `appendWindowsPath = false`, так как это нарушает возможность вызывать исполняемые файлы Windows из WSL. Аналогично, избегайте удаления Node.js из Windows, если вы используете его для разработки Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Permission errors during installation
</h3>

Если встроенный установщик не удаётся с ошибками разрешений, целевая директория может быть недоступна для записи. См. [Проверка разрешений директории](#check-directory-permissions).

Если вы ранее установили с npm и получаете ошибки разрешений, специфичные для npm, переключитесь на встроенный установщик:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Native binary not found after npm install
</h3>

Пакет npm `@anthropic-ai/claude-code` получает встроенный бинарный файл через зависимость, специфичную для платформы, такую как `@anthropic-ai/claude-code-darwin-arm64`. Если запуск `claude` после установки выводит `Could not find native binary package "@anthropic-ai/claude-code-<platform>"`, проверьте следующие причины:

* **Опциональные зависимости отключены.** Удалите `--omit=optional` из вашей команды npm install, `--no-optional` из pnpm или `--ignore-optional` из yarn, и проверьте, что `.npmrc` не устанавливает `optional=false`. Затем переустановите. Встроенный бинарный файл доставляется только как опциональная зависимость, поэтому нет JavaScript fallback, если он пропущен.
* **Неподдерживаемая платформа.** Предварительно собранные бинарные файлы опубликованы для `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` и `win32-arm64`. Claude Code не поставляет бинарный файл для других платформ; см. [системные требования](/docs/ru/setup#system-requirements). На FreeBSD установщик сообщает платформу как неподдерживаемую. До v2.1.205 он рассматривал FreeBSD как Linux и загружал бинарный файл, который не мог работать.
* **Корпоративное зеркало npm отсутствуют пакеты платформы.** Убедитесь, что ваш реестр зеркалирует все восемь пакетов `@anthropic-ai/claude-code-*` платформы в дополнение к мета-пакету.

Установка с `--ignore-scripts` не вызывает эту ошибку. Шаг postinstall, который связывает бинарный файл на место, пропускается, поэтому Claude Code возвращается к обёртке, которая находит и порождает бинарный файл платформы при каждом запуске. Это работает, но запускается медленнее; переустановите со скриптами, включёнными для прямого выполнения.

<h2 id="login-and-authentication">
  Вход и аутентификация
</h2>

Эти разделы решают проблемы входа, ошибки OAuth и проблемы с токенами.

<h3 id="reset-your-login">
  Сброс вашего входа
</h3>

Когда вход не удаётся и причина не очевидна, чистая повторная аутентификация разрешает большинство случаев:

1. Запустите `/logout`, чтобы полностью выйти
2. Закройте Claude Code
3. Перезагрузитесь с `claude` и завершите процесс аутентификации снова

Если браузер не открывается автоматически во время входа, нажмите `c`, чтобы скопировать URL OAuth в буфер обмена, затем вставьте его в браузер вручную. Это также работает, когда URL переносится на несколько строк в узком или SSH терминале и не может быть нажат напрямую.

<h3 id="oauth-error-invalid-code">
  OAuth error: Invalid code
</h3>

Если вы видите `OAuth error: Invalid code. Please make sure the full code was copied`, код входа истёк или был усечён во время копирования-вставки.

**Решения:**

* Нажмите Enter, чтобы повторить и завершить вход быстро после открытия браузера
* Введите `c`, чтобы скопировать полный URL, если браузер не открывается автоматически
* Если используете удалённую/SSH сессию, браузер может открыться на неправильной машине. Скопируйте URL, отображаемый в терминале, и откройте его в вашем локальном браузере вместо этого.

<h3 id="403-forbidden-after-login">
  403 Forbidden after login
</h3>

Если вы видите `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` после входа:

* **Пользователи Claude Pro/Max**: проверьте, что ваша подписка активна на [claude.ai/settings](https://claude.ai/settings)
* **Пользователи Anthropic Console**: подтвердите, что ваша учётная запись имеет роль "Claude Code" или "Developer". Администраторы назначают это в Anthropic Console под Settings → Members.
* **За прокси**: корпоративные прокси могут помешать запросам API. См. [network configuration](/docs/ru/network-config) для настройки прокси.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  This organization has been disabled with an active subscription
</h3>

Если вы видите `API Error: 400 ... "This organization has been disabled"` несмотря на активную подписку Claude, переменная окружения `ANTHROPIC_API_KEY` переопределяет вашу подписку. Это обычно происходит, когда старый API ключ от предыдущего работодателя или проекта всё ещё установлен в вашем профиле оболочки.

Когда `ANTHROPIC_API_KEY` присутствует и вы его одобрили, Claude Code использует этот ключ вместо учётных данных OAuth вашей подписки. В неинтерактивном режиме с флагом `-p` ключ всегда используется, когда присутствует. См. [authentication precedence](/docs/ru/authentication#authentication-precedence) для полного порядка разрешения.

Чтобы использовать вашу подписку вместо этого, отмените установку переменной окружения и удалите её из вашего профиля оболочки:

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Проверьте `~/.zshrc`, `~/.bashrc` или `~/.profile` на строки `export ANTHROPIC_API_KEY=...` и удалите их, чтобы сделать изменение постоянным. На Windows проверьте ваш профиль PowerShell в `$PROFILE` и ваши переменные окружения User на `ANTHROPIC_API_KEY`. Запустите `/status` внутри Claude Code, чтобы подтвердить, какой метод аутентификации активен.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  OAuth login fails in WSL2, SSH, or containers
</h3>

Когда Claude Code работает в WSL2, на удалённой машине через SSH или внутри контейнера, браузер обычно открывается на другом хосте и его перенаправление не может достичь локального сервера обратного вызова Claude Code. После того как вы войдёте, браузер показывает код входа вместо автоматического перенаправления обратно. Вставьте этот код в терминал в приглашение `Paste code here if prompted`, чтобы завершить вход.

Если браузер вообще не открывается из WSL2, установите переменную окружения `BROWSER` на путь вашего браузера Windows:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

Или нажмите `c` на интерактивном приглашении входа, чтобы скопировать URL OAuth, или скопируйте URL, который печатает `claude auth login`, и откройте его в браузере на вашей локальной машине.

Если вставка кода в интерактивное приглашение ничего не делает, привязка вставки вашего терминала вероятно не достигает поля ввода. Попробуйте альтернативное сочетание клавиш вставки вашего терминала, часто правый клик или Shift+Insert в Windows Terminal, или используйте `claude auth login` вместо этого, который читает вставленный код из стандартного ввода:

```bash theme={null}
claude auth login
```

Этот fallback также применяется на нативном Windows или любом терминале, где вставка в интерактивное приглашение не удаётся.

<h3 id="not-logged-in-or-token-expired">
  Not logged in or token expired
</h3>

Если Claude Code предлагает вам войти снова после сессии, ваш токен OAuth может истечь.

Запустите `/login`, чтобы повторно аутентифицироваться. Если это происходит часто, проверьте, что ваши системные часы точны, так как валидация токена зависит от правильных временных меток.

На macOS вход также может не удаться, когда Keychain заблокирован или его пароль не синхронизирован с паролем вашей учётной записи, что предотвращает Claude Code от сохранения учётных данных. Запустите `claude doctor`, чтобы проверить доступ Keychain. Чтобы разблокировать Keychain вручную, запустите `security unlock-keychain ~/Library/Keychains/login.keychain-db`. Если разблокировка не помогает, откройте Keychain Access, выберите keychain `login` и выберите Edit > Change Password for Keychain "login", чтобы пересинхронизировать его с паролем вашей учётной записи.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Bedrock, Agent Platform, or Foundry credentials not loading
</h3>

Если вы настроили Claude Code для использования облачного провайдера и видите `Could not load credentials from any providers` на Amazon Bedrock, `Could not load the default credentials` на Google Cloud's Agent Platform или `ChainedTokenCredential authentication failed` на Microsoft Foundry, ваш CLI облачного провайдера вероятно не аутентифицирован в текущей оболочке.

Для Amazon Bedrock подтвердите, что ваши учётные данные AWS действительны:

```bash theme={null}
aws sts get-caller-identity
```

Для Google Cloud's Agent Platform подтвердите, что `ANTHROPIC_VERTEX_PROJECT_ID` и `CLOUD_ML_REGION` установлены в вашей оболочке, затем установите учётные данные приложения по умолчанию:

```bash theme={null}
gcloud auth application-default login
```

Для Microsoft Foundry подтвердите, что `ANTHROPIC_FOUNDRY_API_KEY` установлен, или войдите с Azure CLI, чтобы цепь учётных данных по умолчанию могла найти вашу учётную запись:

```bash theme={null}
az login
```

Если учётные данные работают в вашем терминале, но не в расширении VS Code или JetBrains, процесс IDE вероятно не унаследовал вашу среду оболочки. Установите переменные окружения провайдера в собственных параметрах IDE или запустите IDE из терминала, где они уже экспортированы.

См. [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) или [Microsoft Foundry](/docs/ru/microsoft-foundry) для полной настройки провайдера.

<h2 id="still-stuck">
  Still stuck
</h2>

Если ничего из вышеперечисленного не разрешает вашу проблему:

1. Проверьте [GitHub repository](https://github.com/anthropics/claude-code/issues) на известные проблемы или откройте новую с вашей операционной системой, командой установки, которую вы запустили, и полным выводом ошибки
2. Если `claude --version` работает, но что-то ещё не так, запустите `claude doctor` для автоматического диагностического отчёта
3. Если вы можете запустить сессию, используйте `/feedback` внутри Claude Code, чтобы сообщить о проблеме
