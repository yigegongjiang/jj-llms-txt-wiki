> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Быстрый старт

> Добро пожаловать в Claude Code!

Это руководство по быстрому старту позволит вам использовать AI-powered кодирование всего за несколько минут. К концу вы поймёте, как использовать Claude Code для типичных задач разработки.

<h2 id="before-you-begin">
  Перед началом
</h2>

Убедитесь, что у вас есть:

* Открытый терминал или командная строка
  * Если вы никогда раньше не использовали терминал, ознакомьтесь с [руководством по терминалу](/docs/ru/terminal-guide)
* Проект кода для работы
* [Подписка Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team или Enterprise), учётная запись [Claude Console](https://console.anthropic.com/) или доступ через [поддерживаемого облачного провайдера](/docs/ru/third-party-integrations)

<Note>
  Это руководство охватывает CLI терминала. Claude Code также доступен в [веб-версии](https://claude.ai/code), как [настольное приложение](/docs/ru/desktop), в [VS Code](/docs/ru/vs-code) и [JetBrains IDEs](/docs/ru/jetbrains), в [Slack](/docs/ru/slack) и в CI/CD с [GitHub Actions](/docs/ru/github-actions) и [GitLab](/docs/ru/gitlab-ci-cd). Смотрите [все интерфейсы](/docs/ru/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Шаг 1: Установите Claude Code
</h2>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

<h2 id="step-2-log-in-to-your-account">
  Шаг 2: Войдите в свою учётную запись
</h2>

Claude Code требует учётную запись для использования. Начните интерактивный сеанс с командой `claude`, и при первом использовании вам будет предложено войти:

```bash theme={null}
claude
```

Для учётных записей Claude подписки или Console следуйте подсказкам для завершения аутентификации в вашем браузере. Чтобы позже переключиться на другую учётную запись или повторно пройти аутентификацию, введите `/login` в работающем сеансе:

```text theme={null}
/login
```

Вы можете войти, используя любой из этих типов учётных записей:

* [Claude Pro, Max, Team или Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (рекомендуется)
* [Claude Console](https://console.anthropic.com/) (доступ к API с предоплаченными кредитами). При первом входе рабочее пространство "Claude Code" автоматически создаётся в Console для централизованного отслеживания затрат.
* [Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry](/docs/ru/third-party-integrations) (облачные провайдеры для предприятий)
* Самостоятельно размещённый [шлюз приложений Claude](/docs/ru/claude-apps-gateway), если ваша организация его использует: ваш администратор предварительно настраивает URL шлюза, и `/login` открывает экран **Cloud gateway** для входа с корпоративным SSO

После входа ваши учётные данные сохраняются, и вам не нужно будет входить снова.

<h2 id="step-3-start-your-first-session">
  Шаг 3: Начните свой первый сеанс
</h2>

Откройте терминал в любом каталоге проекта и запустите Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Вы увидите приглашение Claude Code с версией, текущей моделью и рабочим каталогом, показанными выше. Введите `/help` для доступных команд или `/resume` для продолжения предыдущего разговора.

<Tip>
  После входа (Шаг 2) ваши учётные данные сохраняются на вашей системе. Узнайте больше в [Управлении учётными данными](/docs/ru/authentication#credential-management).
</Tip>

<h2 id="step-4-ask-your-first-question">
  Шаг 4: Задайте свой первый вопрос
</h2>

Давайте начнём с понимания вашей кодовой базы. Попробуйте одну из этих команд:

```text theme={null}
what does this project do?
```

Claude проанализирует ваши файлы и предоставит резюме. Вы также можете задать более конкретные вопросы:

```text theme={null}
what technologies does this project use?
```

```text theme={null}
where is the main entry point?
```

```text theme={null}
explain the folder structure
```

Вы также можете спросить Claude о его собственных возможностях:

```text theme={null}
what can Claude Code do?
```

```text theme={null}
how do I create custom skills in Claude Code?
```

```text theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code читает файлы вашего проекта по мере необходимости. Вам не нужно вручную добавлять контекст.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Шаг 5: Сделайте своё первое изменение кода
</h2>

Теперь давайте заставим Claude Code выполнить некоторое реальное кодирование. Попробуйте простую задачу:

```text theme={null}
add a hello world function to the main file
```

Claude Code будет:

1. Найти подходящий файл
2. Показать вам предложенные изменения
3. Попросить ваше одобрение
4. Сделать редактирование

<Note>
  Claude Code всегда просит разрешение перед изменением файлов. Вы можете одобрить отдельные изменения или включить режим "Принять всё" для сеанса.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Шаг 6: Используйте Git с Claude Code
</h2>

Claude Code делает операции Git разговорными:

```text theme={null}
what files have I changed?
```

```text theme={null}
commit my changes with a descriptive message
```

Вы также можете запросить более сложные операции Git:

```text theme={null}
create a new branch called feature/quickstart
```

```text theme={null}
show me the last 5 commits
```

```text theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Шаг 7: Исправьте ошибку или добавьте функцию
</h2>

Claude хорошо справляется с отладкой и реализацией функций.

Опишите то, что вы хотите, на естественном языке:

```text theme={null}
add input validation to the user registration form
```

Или исправьте существующие проблемы:

```text theme={null}
there's a bug where users can submit empty forms - fix it
```

Claude Code будет:

* Найти соответствующий код
* Понять контекст
* Реализовать решение
* Запустить тесты, если они доступны

<h2 id="step-8-test-out-other-common-workflows">
  Шаг 8: Попробуйте другие типичные рабочие процессы
</h2>

Есть несколько способов работать с Claude:

**Рефакторинг кода**

```text theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**Написание тестов**

```text theme={null}
write unit tests for the calculator functions
```

**Обновление документации**

```text theme={null}
update the README with installation instructions
```

**Проверка кода**

```text theme={null}
review my changes and suggest improvements
```

<Tip>
  Разговаривайте с Claude как с полезным коллегой. Опишите, чего вы хотите достичь, и он поможет вам это сделать.
</Tip>

<h2 id="essential-commands">
  Основные команды
</h2>

Вот наиболее важные команды для ежедневного использования. Команды оболочки запускаются из вашего терминала для запуска или возобновления Claude Code. Команды сеанса запускаются внутри Claude Code после его запуска.

**Команды оболочки**

| Команда             | Что она делает                                         | Пример                              |
| ------------------- | ------------------------------------------------------ | ----------------------------------- |
| `claude`            | Запустить интерактивный режим                          | `claude`                            |
| `claude "task"`     | Запустить одноразовую задачу                           | `claude "fix the build error"`      |
| `claude -p "query"` | Запустить одноразовый запрос, затем выйти              | `claude -p "explain this function"` |
| `claude -c`         | Продолжить самый последний разговор в текущем каталоге | `claude -c`                         |
| `claude -r`         | Возобновить предыдущий разговор                        | `claude -r`                         |

**Команды сеанса**

| Команда            | Что она делает             | Пример   |
| ------------------ | -------------------------- | -------- |
| `/clear`           | Очистить историю разговора | `/clear` |
| `/help`            | Показать доступные команды | `/help`  |
| `/exit` или Ctrl+D | Выйти из Claude Code       | `/exit`  |

Смотрите [справочник CLI](/docs/ru/cli-reference) для полного списка команд оболочки и [справочник команд](/docs/ru/commands) для полного списка команд сеанса.

<h2 id="pro-tips-for-beginners">
  Советы для начинающих
</h2>

Для большего, смотрите [лучшие практики](/docs/ru/best-practices) и [типичные рабочие процессы](/docs/ru/common-workflows).

<AccordionGroup>
  <Accordion title="Будьте конкретны в своих запросах">
    Вместо: "исправить ошибку"

    Попробуйте: "исправить ошибку входа, когда пользователи видят пустой экран после ввода неправильных учетных данных"
  </Accordion>

  <Accordion title="Используйте пошаговые инструкции">
    Разбейте сложные задачи на этапы:

    ```text theme={null}
    1. создать новую таблицу базы данных для профилей пользователей
    2. создать конечную точку API для получения и обновления профилей пользователей
    3. создать веб-страницу, которая позволяет пользователям просматривать и редактировать свою информацию
    ```
  </Accordion>

  <Accordion title="Позвольте Claude сначала исследовать">
    Перед внесением изменений позвольте Claude понять ваш код:

    ```text theme={null}
    проанализировать схему базы данных
    ```

    ```text theme={null}
    создать панель управления, показывающую продукты, которые чаще всего возвращаются нашими клиентами из Великобритании
    ```
  </Accordion>

  <Accordion title="Сэкономьте время с помощью ярлыков">
    * Введите `/` для просмотра всех команд и skills
    * Используйте Tab для завершения команды
    * Нажмите ↑ для истории команд
    * Нажмите `Shift+Tab` для переключения режимов разрешений
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  Что дальше?
</h2>

Теперь, когда вы изучили основы, исследуйте более продвинутые функции:

<CardGroup cols={2}>
  <Card title="Как работает Claude Code" icon="microchip" href="/docs/ru/how-claude-code-works">
    Поймите агентский цикл, встроенные инструменты и то, как Claude Code взаимодействует с вашим проектом
  </Card>

  <Card title="Лучшие практики" icon="star" href="/docs/ru/best-practices">
    Получайте лучшие результаты с эффективным запросом и настройкой проекта
  </Card>

  <Card title="Типичные рабочие процессы" icon="graduation-cap" href="/docs/ru/common-workflows">
    Пошаговые руководства для типичных задач
  </Card>

  <Card title="Расширьте Claude Code" icon="puzzle-piece" href="/docs/ru/features-overview">
    Настройте с помощью CLAUDE.md, skills, hooks, MCP и многого другого
  </Card>
</CardGroup>

<h2 id="getting-help">
  Получение помощи
</h2>

* **В Claude Code**: Введите `/help` или спросите "how do I..."
* **Документация**: Вы здесь! Просмотрите другие руководства
* **Сообщество**: Присоединитесь к нашему [Discord](https://www.anthropic.com/discord) для советов и поддержки
