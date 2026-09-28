> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Обзор

> Claude Code — это агентский инструмент кодирования, который читает вашу кодовую базу, редактирует файлы, выполняет команды и интегрируется с вашими инструментами разработки. Доступен в вашем терминале, IDE, приложении для рабочего стола и браузере.

Claude Code — это AI-помощник по кодированию, который помогает вам создавать функции, исправлять ошибки и автоматизировать задачи разработки. Он понимает всю вашу кодовую базу и может работать с несколькими файлами и инструментами для выполнения задач.

<h2 id="get-started">
  Начало работы
</h2>

Claude Code работает на нескольких платформах: в терминале, расширениях IDE, настольном приложении и в веб-версии. Выберите одну из вкладок ниже, чтобы начать. Большинство платформ требуют [подписку Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_pricing) или учетную запись [Anthropic Console](https://platform.claude.com/). Terminal CLI, VS Code и JetBrains также поддерживают [сторонних поставщиков](/docs/ru/third-party-integrations).

<Tabs>
  <Tab title="Terminal">
    Полнофункциональный CLI для работы с Claude Code прямо в вашем терминале. Редактируйте файлы, выполняйте команды и управляйте всем проектом из командной строки.

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

    Затем запустите Claude Code в любом проекте. Замените `your-project` на путь к каталогу проекта на вашей машине:

    ```bash theme={null}
    cd your-project
    claude
    ```

    При первом использовании вам будет предложено войти. Если вы установили переменную окружения `ANTHROPIC_API_KEY`, Claude Code пропустит приглашение входа и попросит вас одобрить ключ вместо этого. Вот и все! [Продолжите с Quickstart →](/docs/ru/quickstart)

    <Tip>
      Смотрите [расширенную настройку](/docs/ru/setup) для опций установки, ручных обновлений или инструкций по удалению. Посетите [troubleshooting установки](/docs/ru/troubleshoot-install), если у вас возникли проблемы.
    </Tip>
  </Tab>

  <Tab title="VS Code">
    Расширение VS Code предоставляет встроенные различия, @-упоминания, просмотр плана и историю разговоров прямо в вашем редакторе.

    * [Установить для VS Code](vscode:extension/anthropic.claude-code)
    * [Установить для Cursor](cursor:extension/anthropic.claude-code)

    Или найдите "Claude Code" в представлении Extensions (`Cmd+Shift+X` на Mac, `Ctrl+Shift+X` на Windows/Linux). После установки откройте Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`), введите "Claude Code" и выберите **Open in New Tab**.

    [Начните работу с VS Code →](/docs/ru/vs-code#get-started)
  </Tab>

  <Tab title="Desktop app">
    Автономное приложение для запуска Claude Code вне вашей IDE или терминала. Просматривайте различия визуально, запускайте несколько сеансов рядом, планируйте повторяющиеся задачи и запускайте облачные сеансы.

    Загрузите и установите:

    * [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) (Intel и Apple Silicon)
    * [Windows](https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs) (x64)
    * [Windows ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)
    * На Ubuntu или Debian, где приложение находится в бета-версии, установите его с помощью apt, следуя [инструкциям по установке Linux](/docs/ru/desktop-linux)

    После установки запустите Claude, войдите и нажмите вкладку **Code** для начала кодирования. Приложение включает Claude Code, поэтому вам не нужно устанавливать CLI отдельно. Требуется [платная подписка](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_desktop_pricing).

    [Узнайте больше о приложении для рабочего стола →](/docs/ru/desktop-quickstart)
  </Tab>

  <Tab title="Web">
    Запустите Claude Code в вашем браузере без локальной настройки. Запускайте долгоживущие задачи и возвращайтесь, когда они будут готовы, работайте с репозиториями, которые у вас нет локально, или запускайте несколько задач параллельно. Для более длительной работы создайте [проект](/docs/ru/claude-projects) и позвольте Claude координировать параллельные сеансы для вас. Доступно на настольных браузерах и [приложении Claude для iOS и Android](/docs/ru/mobile).

    Начните кодирование на [claude.ai/code](https://claude.ai/code).

    [Начните работу →](/docs/ru/web-quickstart)
  </Tab>

  <Tab title="JetBrains">
    Плагин для IntelliJ IDEA, PyCharm, WebStorm и других IDE JetBrains с интерактивным просмотром различий и совместным использованием контекста выделения.

    Установите [плагин Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) из JetBrains Marketplace и перезагрузите вашу IDE. Плагин требует Claude Code CLI, установленный отдельно; см. [шаги настройки JetBrains](/docs/ru/jetbrains#installation).

    [Начните работу с JetBrains →](/docs/ru/jetbrains)
  </Tab>
</Tabs>

<h2 id="what-you-can-do">
  Что вы можете делать
</h2>

Вот некоторые способы использования Claude Code:

<AccordionGroup>
  <Accordion title="Автоматизируйте работу, которую вы постоянно откладываете" icon="wand-magic-sparkles">
    Claude Code справляется с утомительными задачами, которые съедают ваш день: написание тестов для непроверенного кода, исправление ошибок lint по всему проекту, разрешение конфликтов слияния, обновление зависимостей и написание заметок о выпуске.

    ```bash theme={null}
    claude "write tests for the auth module, run them, and fix any failures"
    ```
  </Accordion>

  <Accordion title="Создавайте функции и исправляйте ошибки" icon="hammer">
    Опишите то, что вы хотите, на простом языке. Claude Code планирует подход, пишет код в нескольких файлах и проверяет, что он работает.

    Для ошибок вставьте сообщение об ошибке или опишите симптом. Claude Code отслеживает проблему через вашу кодовую базу, определяет основную причину и реализует исправление. Смотрите [common workflows](/docs/ru/common-workflows) для получения дополнительных примеров.
  </Accordion>

  <Accordion title="Создавайте коммиты и pull requests" icon="code-branch">
    Claude Code работает непосредственно с git. Он подготавливает изменения, пишет сообщения коммитов, создает ветки и открывает pull requests.

    ```bash theme={null}
    claude "commit my changes with a descriptive message"
    ```

    В CI вы можете автоматизировать проверку кода и сортировку проблем с помощью [GitHub Actions](/docs/ru/github-actions) или [GitLab CI/CD](/docs/ru/gitlab-ci-cd).
  </Accordion>

  <Accordion title="Подключите свои инструменты с помощью MCP" icon="plug">
    [Model Context Protocol (MCP)](/docs/ru/mcp) — это открытый стандарт для подключения инструментов AI к внешним источникам данных. С помощью MCP Claude Code может читать ваши документы дизайна в Google Drive, обновлять задачи в Jira, извлекать данные из Slack или использовать ваши собственные пользовательские инструменты. [MCP quickstart](/docs/ru/mcp-quickstart) подключает ваш первый сервер от начала до конца.
  </Accordion>

  <Accordion title="Настройте с помощью инструкций, skills и hooks" icon="sliders">
    [`CLAUDE.md`](/docs/ru/memory) — это файл markdown, который вы добавляете в корень вашего проекта, и Claude Code читает его в начале каждого сеанса. Используйте его для установки стандартов кодирования, архитектурных решений, предпочитаемых библиотек и контрольных списков проверки. Если ваш репозиторий уже имеет `AGENTS.md` для других агентов кодирования, Claude Code [может читать это](/docs/ru/memory#agents-md) самостоятельно или вместе с `CLAUDE.md`. Claude также создает [auto memory](/docs/ru/memory#auto-memory) по мере работы, сохраняя знания в разных сеансах без необходимости что-либо писать.

    Создавайте [skills](/docs/ru/skills) для упаковки повторяемых рабочих процессов, которые ваша команда может использовать, например `/review-pr` или `/deploy-staging`.

    [Hooks](/docs/ru/hooks) позволяют вам запускать команды shell до или после действий Claude Code, например автоматическое форматирование после каждого редактирования файла или запуск lint перед коммитом.
  </Accordion>

  <Accordion title="Запустите агентов параллельно и создавайте пользовательских агентов" icon="users">
    Запустите [несколько агентов Claude Code](/docs/ru/sub-agents), которые работают над разными частями задачи одновременно. Главный агент координирует работу, назначает подзадачи и объединяет результаты.

    Для запуска нескольких полных сеансов параллельно и наблюдения за ними с одного экрана используйте [background agents](/docs/ru/agent-view). Для полностью пользовательских рабочих процессов [Agent SDK](/docs/ru/agent-sdk/overview) позволяет вам создавать собственных агентов, работающих на инструментах и возможностях Claude Code, с полным контролем над оркестровкой, доступом к инструментам и разрешениями.
  </Accordion>

  <Accordion title="Передавайте, создавайте скрипты и автоматизируйте с помощью CLI" icon="terminal">
    Claude Code является составным и следует философии Unix. Передавайте в него логи, запускайте его в CI или объединяйте его с другими инструментами:

    ```bash theme={null}
    # Анализируйте недавний вывод логов
    tail -200 app.log | claude -p "Slack me if you see any anomalies"

    # Автоматизируйте переводы в CI
    claude -p "translate new strings into French and raise a PR for review"

    # Массовые операции по файлам
    git diff main --name-only | claude -p "review these changed files for security issues"
    ```

    Смотрите [CLI reference](/docs/ru/cli-reference) для полного набора команд и флагов.
  </Accordion>

  <Accordion title="Планируйте повторяющиеся задачи" icon="clock">
    Запускайте Claude по расписанию для автоматизации работы, которая повторяется: утренние проверки PR, анализ сбоев CI в ночное время, еженедельные аудиты зависимостей или синхронизация документов после слияния PR.

    * [Routines](/docs/ru/routines) работают в облаке, поэтому они продолжают работать даже когда ваш компьютер выключен. Они также могут срабатывать при вызовах API или событиях GitHub. Создавайте их из веб-версии, приложения Desktop или запустив `/schedule` в CLI.
    * [Запланированные задачи Desktop](/docs/ru/desktop-scheduled-tasks) работают на вашей машине с прямым доступом к вашим локальным файлам и инструментам
    * [`/loop`](/docs/ru/scheduled-tasks) повторяет подсказку в сеансе CLI для быстрого опроса
  </Accordion>

  <Accordion title="Работайте откуда угодно" icon="globe">
    Сеансы не привязаны к одной поверхности. Перемещайте работу между ними по мере изменения вашего контекста:

    * Отойдите от своего стола и продолжайте работать со своего телефона или любого браузера с помощью [Remote Control](/docs/ru/remote-control)
    * Отправьте сообщение [Dispatch](/docs/ru/desktop#sessions-from-dispatch) с задачей со своего телефона и откройте сеанс Desktop, который он создает
    * Запустите долгоживущую задачу в [веб-версии](/docs/ru/claude-code-on-the-web) или [приложении Claude для мобильных устройств](/docs/ru/mobile), затем перенесите ее в свой терминал с помощью `claude --teleport`. Teleport требует подписку claude.ai.
    * Запустите `/desktop` для продолжения вашего текущего сеанса терминала в [приложении Desktop](/docs/ru/desktop), где вы можете просмотреть различия визуально. Передача `/desktop` требует подписку claude.ai. Доступно на macOS и x64 Windows.
    * Маршрутизируйте задачи из командного чата: упомяните `@Claude` в [Slack](/docs/ru/slack) с отчетом об ошибке и получите pull request обратно
  </Accordion>
</AccordionGroup>

<h2 id="use-claude-code-everywhere">
  Используйте Claude Code везде
</h2>

Каждая [поверхность](/docs/ru/glossary#surface) подключается к одному и тому же базовому механизму Claude Code, поэтому ваши файлы CLAUDE.md, параметры и MCP servers работают на всех них.

Помимо сред [Terminal](/docs/ru/quickstart), [VS Code](/docs/ru/vs-code), [JetBrains](/docs/ru/jetbrains), [Desktop](/docs/ru/desktop) и [Web](/docs/ru/claude-code-on-the-web) выше, Claude Code интегрируется с CI/CD, чатом и рабочими процессами браузера:

| Что я хочу сделать                                                                     | Лучший вариант                                                                                                         |
| -------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Продолжить локальный сеанс со своего телефона или другого устройства                   | [Remote Control](/docs/ru/remote-control)                                                                                   |
| Отправить события из Telegram, Discord, iMessage или моих собственных webhooks в сеанс | [Channels](/docs/ru/channels)                                                                                               |
| Начать задачу локально, продолжить на мобильном                                        | [`claude --cloud`](/docs/ru/claude-code-on-the-web#from-terminal-to-cloud), затем [мобильное приложение Claude](/docs/ru/mobile) |
| Запустить Claude по расписанию                                                         | [Routines](/docs/ru/routines) или [Запланированные задачи Desktop](/docs/ru/desktop-scheduled-tasks)                             |
| Автоматизировать проверки PR и сортировку проблем                                      | [GitHub Actions](/docs/ru/github-actions) или [GitLab CI/CD](/docs/ru/gitlab-ci-cd)                                              |
| Получить автоматическую проверку кода на каждый PR                                     | [GitHub Code Review](/docs/ru/code-review)                                                                                  |
| Маршрутизировать отчеты об ошибках из Slack в pull requests                            | [Slack](/docs/ru/slack)                                                                                                     |
| Отладить живые веб-приложения                                                          | [Chrome](/docs/ru/chrome)                                                                                                   |
| Создавайте пользовательских агентов для ваших собственных рабочих процессов            | [Agent SDK](/docs/ru/agent-sdk/overview)                                                                                    |

<h2 id="next-steps">
  Следующие шаги
</h2>

После установки Claude Code эти руководства помогут вам углубиться.

* [Quickstart](/docs/ru/quickstart): пройдите через вашу первую реальную задачу, от изучения кодовой базы до коммита исправления
* [Сохраняйте инструкции и воспоминания](/docs/ru/memory): дайте Claude постоянные инструкции с файлами CLAUDE.md и auto memory
* [Common workflows](/docs/ru/common-workflows) и [best practices](/docs/ru/best-practices): шаблоны для получения максимума от Claude Code
* [Claude Academy](https://academy.claude.com/): бесплатные самостоятельные курсы, включая [Claude Code 101](https://academy.claude.com/courses/claude-code-101) и [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action)
* [A harness for every task](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): как команда Claude Code использует [dynamic workflows](/docs/ru/workflows) для организации подагентов в масштабе
* [Settings](/docs/ru/settings): настройте Claude Code для вашего рабочего процесса
* [Troubleshooting](/docs/ru/troubleshooting): решения для распространенных проблем
* [code.claude.com](https://code.claude.com/): демонстрации, цены и детали продукта
