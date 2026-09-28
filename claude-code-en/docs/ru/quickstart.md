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
* [Подписка Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team или Enterprise), учётная запись [Claude Console](https://platform.claude.com/) или доступ через [поддерживаемого облачного провайдера](/docs/ru/third-party-integrations)

<Note>
  Это руководство охватывает CLI терминала. Claude Code также доступен в [веб-версии](https://claude.ai/code), как [настольное приложение](/docs/ru/desktop), в [VS Code](/docs/ru/vs-code) и [JetBrains IDEs](/docs/ru/jetbrains), в [Slack](/docs/ru/slack) и в CI/CD с [GitHub Actions](/docs/ru/github-actions) и [GitLab](/docs/ru/gitlab-ci-cd). Смотрите [все интерфейсы](/docs/ru/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Шаг 1: Установите Claude Code
</h2>

Для установки Claude Code используйте один из следующих методов:

<Tabs>
  <Tab title="Встроенная установка (рекомендуется)">
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

    Если вы видите `The token '&&' is not a valid statement separator`, вы находитесь в PowerShell, а не в CMD. Если вы видите `'irm' is not recognized as an internal or external command`, вы находитесь в CMD, а не в PowerShell. Ваша подсказка показывает `PS C:\` когда вы находитесь в PowerShell и `C:\` без `PS` когда вы находитесь в CMD.

    Если команда установки завершается с ошибкой `syntax error near unexpected token '<'`, `403` или другой ошибкой curl, см. [Устранение неполадок при установке](/docs/ru/troubleshoot-install#find-your-error) чтобы сопоставить ошибку с исправлением и для альтернативных методов установки.

    [Git for Windows](https://git-scm.com/downloads/win) рекомендуется на встроенной Windows, чтобы Claude Code мог использовать инструмент Bash. Если Git for Windows не установлен, Claude Code использует PowerShell в качестве инструмента оболочки. Установки WSL не требуют Git for Windows.

    <Info>
      Встроенные установки автоматически обновляются в фоновом режиме, чтобы вы всегда использовали последнюю версию.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew предлагает два пакета. `claude-code` отслеживает канал стабильного выпуска, который обычно отстает примерно на неделю и пропускает выпуски с серьезными регрессиями. `claude-code@latest` отслеживает последний канал и получает новые версии сразу после их выпуска.

    <Info>
      Установки Homebrew не обновляются автоматически. Запустите `brew upgrade claude-code` или `brew upgrade claude-code@latest`, в зависимости от того, какой пакет вы установили, чтобы получить последние функции и исправления безопасности.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      Установки WinGet не обновляются автоматически. Периодически запускайте `winget upgrade Anthropic.ClaudeCode` чтобы получить последние функции и исправления безопасности.
    </Info>
  </Tab>
</Tabs>

Вы также можете установить с помощью [apt, dnf или apk](/docs/ru/setup#install-with-linux-package-managers) на Debian, Fedora, RHEL и Alpine.

Чтобы подтвердить, что установка прошла успешно, выполните:

```bash theme={null}
claude --version
```

Команда выводит номер версии, за которым следует `(Claude Code)`.

<h2 id="step-2-log-in-to-your-account">
  Шаг 2: Войдите в свою учётную запись
</h2>

Claude Code требует учётную запись для использования. Начните интерактивный сеанс с командой `claude`, и при первом использовании вам будет предложено войти:

```bash theme={null}
claude
```

Для учётных записей Claude подписки или Console следуйте подсказкам для завершения аутентификации в вашем браузере. Если вы установили переменную окружения `ANTHROPIC_API_KEY`, Claude Code пропускает приглашение входа и вместо этого просит вас одобрить ключ. Чтобы позже переключиться на другую учётную запись или повторно пройти аутентификацию, введите `/login` в работающем сеансе:

```text wrap theme={null}
/login
```

Вы можете войти, используя любой из этих типов учётных записей:

* [Claude Pro, Max, Team или Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (рекомендуется)
* [Claude Console](https://platform.claude.com/) (доступ к API с предоплаченными кредитами). При первом входе рабочее пространство "Claude Code" автоматически создаётся в Console для централизованного отслеживания затрат.
* [Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry](/docs/ru/third-party-integrations) (облачные провайдеры для предприятий)
* Самостоятельно размещённый [шлюз приложений Claude](/docs/ru/claude-apps-gateway), если ваша организация его использует: ваш администратор предварительно настраивает URL шлюза, и `/login` открывает экран **Cloud gateway** для входа с корпоративным SSO

После входа ваши учётные данные сохраняются, и вам не нужно будет входить снова. Узнайте больше в разделе [Управление учётными данными](/docs/ru/authentication#credential-management).

<h2 id="step-3-start-your-first-session">
  Шаг 3: Начните свой первый сеанс
</h2>

Откройте терминал в любом каталоге проекта и запустите Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Замените `/path/to/your/project` на путь к проекту, над которым вы хотите работать.

Вы увидите приглашение Claude Code с версией, текущей моделью и рабочим каталогом, показанными выше. Введите `/help` для доступных команд или `/resume` для продолжения предыдущего разговора.

<h2 id="step-4-ask-your-first-question">
  Шаг 4: Задайте свой первый вопрос
</h2>

Давайте начнём с понимания вашей кодовой базы. Попробуйте одну из этих команд:

```text wrap theme={null}
what does this project do?
```

Claude проанализирует ваши файлы и предоставит резюме. Вы также можете задать более конкретные вопросы:

```text wrap theme={null}
what technologies does this project use?
```

```text wrap theme={null}
where is the main entry point?
```

```text wrap theme={null}
explain the folder structure
```

Вы также можете спросить Claude о его собственных возможностях:

```text wrap theme={null}
what can Claude Code do?
```

```text wrap theme={null}
how do I create custom skills in Claude Code?
```

```text wrap theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code читает файлы вашего проекта по мере необходимости. Вам не нужно вручную добавлять контекст.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Шаг 5: Сделайте своё первое изменение кода
</h2>

Теперь давайте заставим Claude Code выполнить некоторое реальное кодирование. Попробуйте простую задачу:

```text wrap theme={null}
add a hello world function to the main file
```

Claude Code находит подходящий файл и показывает вам изменение. Если он просит разрешение перед внесением изменения, выберите **Да** для одобрения.

Auto mode — это [встроенный начальный режим разрешений](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) для интерактивных сеансов терминала на планах Pro, Max и Team: классификатор проверяет действия вместо вас, и Claude редактирует большинство файлов и выполняет большинство команд без запроса. На других планах Manual mode — это встроенный начальный режим разрешений. Для сеанса, который вы запускаете сразу после установки, см. [Первый сеанс после установки или обновления](/docs/ru/env-vars#first-session-after-an-install-or-upgrade).

<Note>
  Ваши параметры или ваша организация могут установить другой начальный режим разрешений. [Какой режим разрешений начинается в сеансе](/docs/ru/permission-modes#which-mode-a-session-starts-in) перечисляет, что это делает. Нажмите `Shift+Tab` в любой момент, чтобы переключить режим разрешений сеанса, в котором вы находитесь.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Шаг 6: Используйте Git с Claude Code
</h2>

Claude Code делает операции Git разговорными:

```text wrap theme={null}
what files have I changed?
```

```text wrap theme={null}
commit my changes with a descriptive message
```

Вы также можете запросить более сложные операции Git:

```text wrap theme={null}
create a new branch called feature/quickstart
```

```text wrap theme={null}
show me the last 5 commits
```

```text wrap theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Шаг 7: Исправьте ошибку или добавьте функцию
</h2>

Claude хорошо справляется с отладкой и реализацией функций.

Опишите то, что вы хотите, на естественном языке:

```text wrap theme={null}
add input validation to the user registration form
```

Или исправьте существующие проблемы:

```text wrap theme={null}
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

```text wrap theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**Написание тестов**

```text wrap theme={null}
write unit tests for the calculator functions
```

**Обновление документации**

```text wrap theme={null}
update the README with installation instructions
```

**Проверка кода**

```text wrap theme={null}
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
| `claude "task"`     | Запустить интерактивный режим с начальным приглашением | `claude "fix the build error"`      |
| `claude -p "query"` | Запустить одноразовый запрос, затем выйти              | `claude -p "explain this function"` |
| `claude -c`         | Продолжить самый последний разговор в текущем каталоге | `claude -c`                         |
| `claude -r`         | Возобновить предыдущий разговор                        | `claude -r`                         |

**Команды сеанса**

| Команда                   | Что она делает             | Пример   |
| ------------------------- | -------------------------- | -------- |
| `/clear`                  | Очистить историю разговора | `/clear` |
| `/help`                   | Показать доступные команды | `/help`  |
| `/exit` или Ctrl+D дважды | Выйти из Claude Code       | `/exit`  |

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

    ```text wrap theme={null}
    1. создать новую таблицу базы данных для профилей пользователей
    2. создать конечную точку API для получения и обновления профилей пользователей
    3. создать веб-страницу, которая позволяет пользователям просматривать и редактировать свою информацию
    ```
  </Accordion>

  <Accordion title="Позвольте Claude сначала исследовать">
    Перед внесением изменений позвольте Claude понять ваш код:

    ```text wrap theme={null}
    проанализировать схему базы данных
    ```

    ```text wrap theme={null}
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

* **В Claude Code**: Введите `/help` или спросите "how do I" вопрос
* **Документация**: Вы здесь! Просмотрите другие руководства
* **Курсы**: Пройдите [Claude Code 101](https://academy.claude.com/courses/claude-code-101) и другие бесплатные самостоятельные курсы на [Claude Academy](https://academy.claude.com/)
* **Сообщество**: Присоединитесь к нашему [Discord](https://www.anthropic.com/discord) для советов и поддержки
