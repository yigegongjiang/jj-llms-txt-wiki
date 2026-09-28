> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Распространённые рабочие процессы

> Пошаговые руководства по изучению кодовых баз, исправлению ошибок, рефакторингу, тестированию и другим повседневным задачам с Claude Code.

На этой странице собраны короткие рецепты для повседневной разработки. Для получения более высокоуровневого руководства по подсказкам и управлению контекстом см. [Лучшие практики](/docs/ru/best-practices).

На этой странице рассматриваются:

* [Рецепты подсказок](#prompt-recipes) для изучения кода, исправления ошибок, рефакторинга, тестирования, PR и документации
* [Возобновление предыдущих разговоров](#resume-previous-conversations) для того, чтобы задача могла охватывать несколько сеансов
* [Запуск параллельных сеансов с worktrees](#run-parallel-sessions-with-worktrees) для того, чтобы одновременные редактирования не конфликтовали
* [Планирование перед редактированием](#plan-before-editing) для проверки изменений перед их записью на диск
* [Делегирование исследований subagents](#delegate-research-to-subagents) для сохранения чистоты вашего основного контекста
* [Передача Claude в скрипты](#pipe-claude-into-scripts) для CI и пакетной обработки

<h2 id="prompt-recipes">
  Рецепты подсказок
</h2>

Это паттерны подсказок для повседневных задач, таких как изучение незнакомого кода, отладка, рефакторинг, написание тестов и создание PR. Каждый работает на любой поверхности Claude Code; адаптируйте формулировку к вашему проекту.

<h3 id="understand-new-codebases">
  Понимание новых кодовых баз
</h3>

Для настройки Claude Code в монорепозитории или большой кодовой базе см. [Монорепозитории и большие репозитории](/docs/ru/large-codebases).

<h4 id="get-a-quick-codebase-overview">
  Получение быстрого обзора кодовой базы
</h4>

Предположим, вы только что присоединились к новому проекту и вам нужно быстро понять его структуру.

<Steps>
  <Step title="Перейдите в корневой каталог проекта">
    ```bash theme={null}
    cd /path/to/project 
    ```

    Замените `/path/to/project` на путь к вашему проекту.
  </Step>

  <Step title="Запустите Claude Code">
    ```bash theme={null}
    claude 
    ```
  </Step>

  <Step title="Попросите высокоуровневый обзор">
    ```text wrap theme={null}
    give me an overview of this codebase
    ```
  </Step>

  <Step title="Углубитесь в конкретные компоненты">
    ```text wrap theme={null}
    explain the main architecture patterns used here
    ```

    ```text wrap theme={null}
    what are the key data models?
    ```

    ```text wrap theme={null}
    how is authentication handled?
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Начните с широких вопросов, затем сужайте до конкретных областей
  * Спросите о соглашениях кодирования и паттернах, используемых в проекте
  * Запросите глоссарий терминов, специфичных для проекта
</Tip>

<h4 id="find-relevant-code">
  Поиск релевантного кода
</h4>

Предположим, вам нужно найти код, связанный с конкретной функцией или функциональностью.

<Steps>
  <Step title="Попросите Claude найти релевантные файлы">
    ```text wrap theme={null}
    find the files that handle user authentication
    ```
  </Step>

  <Step title="Получите контекст о том, как компоненты взаимодействуют">
    ```text wrap theme={null}
    how do these authentication files work together?
    ```
  </Step>

  <Step title="Поймите поток выполнения">
    ```text wrap theme={null}
    trace the login process from front-end to database
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Будьте конкретны в том, что вы ищете
  * Используйте предметный язык из проекта
  * Установите [плагин анализа кода](/docs/ru/plugins/code-intelligence) для вашего языка, чтобы дать Claude точную навигацию "перейти к определению" и "найти ссылки"
</Tip>

***

<h3 id="fix-bugs-efficiently">
  Эффективное исправление ошибок
</h3>

Предположим, вы столкнулись с сообщением об ошибке и вам нужно найти и исправить его источник.

<Steps>
  <Step title="Поделитесь ошибкой с Claude">
    ```text wrap theme={null}
    I'm seeing an error when I run npm test
    ```
  </Step>

  <Step title="Попросите рекомендации по исправлению">
    ```text wrap theme={null}
    suggest a few ways to fix the @ts-ignore in user.ts
    ```
  </Step>

  <Step title="Примените исправление">
    ```text wrap theme={null}
    update user.ts to add the null check you suggested
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Скажите Claude команду для воспроизведения проблемы и получите трассировку стека
  * Упомяните любые шаги для воспроизведения ошибки
  * Дайте Claude знать, является ли ошибка прерывистой или постоянной
</Tip>

***

<h3 id="refactor-code">
  Рефакторинг кода
</h3>

Предположим, вам нужно обновить старый код для использования современных паттернов и практик.

Для переноса всей кодовой базы на новый язык см. [как Anthropic проводит крупномасштабные миграции кода с помощью Claude Code](https://claude.com/blog/ai-code-migration) в блоге.

<Steps>
  <Step title="Определите устаревший код для рефакторинга">
    ```text wrap theme={null}
    find deprecated API usage in our codebase
    ```
  </Step>

  <Step title="Получите рекомендации по рефакторингу">
    ```text wrap theme={null}
    suggest how to refactor utils.js to use modern JavaScript features
    ```
  </Step>

  <Step title="Примените изменения безопасно">
    ```text wrap theme={null}
    refactor utils.js to use ES2024 features while maintaining the same behavior
    ```
  </Step>

  <Step title="Проверьте рефакторинг">
    ```text wrap theme={null}
    run tests for the refactored code
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Попросите Claude объяснить преимущества современного подхода
  * Запросите, чтобы изменения сохраняли обратную совместимость при необходимости
  * Выполняйте рефакторинг небольшими, тестируемыми шагами
</Tip>

***

<h3 id="work-with-tests">
  Работа с тестами
</h3>

Предположим, вам нужно добавить тесты для непокрытого кода.

<Steps>
  <Step title="Определите непротестированный код">
    ```text wrap theme={null}
    find functions in NotificationsService.swift that are not covered by tests
    ```
  </Step>

  <Step title="Создайте каркас тестов">
    ```text wrap theme={null}
    add tests for the notification service
    ```
  </Step>

  <Step title="Добавьте значимые тестовые случаи">
    ```text wrap theme={null}
    add test cases for edge conditions in the notification service
    ```
  </Step>

  <Step title="Запустите и проверьте тесты">
    ```text wrap theme={null}
    run the new tests and fix any failures
    ```
  </Step>
</Steps>

Claude может создавать тесты, которые следуют существующим паттернам и соглашениям вашего проекта. При запросе тестов будьте конкретны в отношении поведения, которое вы хотите проверить. Claude изучает ваши существующие файлы тестов, чтобы соответствовать стилю, фреймворкам и паттернам утверждений, уже используемым в проекте.

Для полного покрытия попросите Claude определить граничные случаи, которые вы могли пропустить. Claude может анализировать пути вашего кода и предлагать тесты для условий ошибок, граничных значений и неожиданных входных данных, которые легко упустить.

***

<h3 id="create-pull-requests">
  Создание pull requests
</h3>

Вы можете создавать pull requests, попросив Claude напрямую ("create a pr for my changes"), или направить Claude пошагово:

<Steps>
  <Step title="Суммируйте ваши изменения">
    ```text wrap theme={null}
    summarize the changes I've made to the authentication module
    ```
  </Step>

  <Step title="Создайте pull request">
    ```text wrap theme={null}
    create a pr
    ```
  </Step>

  <Step title="Просмотрите и уточните">
    ```text wrap theme={null}
    enhance the PR description with more context about the security improvements
    ```
  </Step>
</Steps>

Чтобы найти сеанс позже, запустите `claude --from-pr 1234` с номером вашего PR, который открывает средство выбора сеанса, отфильтрованное по сеансам, связанным с этим PR, или вставьте URL PR в средство выбора [`/resume`](/docs/ru/sessions#use-the-session-picker). Claude Code связывает сеанс с PR, когда Claude создает его с помощью `gh pr create` или `glab mr create`, и когда Claude [работает над существующим PR](/docs/ru/agent-view#pull-request-status).

<Tip>
  Просмотрите PR, созданный Claude, перед отправкой и попросите Claude выделить потенциальные риски или соображения.
</Tip>

<h3 id="handle-documentation">
  Работа с документацией
</h3>

Предположим, вам нужно добавить или обновить документацию для вашего кода.

<Steps>
  <Step title="Определите недокументированный код">
    ```text wrap theme={null}
    find functions without proper JSDoc comments in the auth module
    ```
  </Step>

  <Step title="Создайте документацию">
    ```text wrap theme={null}
    add JSDoc comments to the undocumented functions in auth.js
    ```
  </Step>

  <Step title="Просмотрите и улучшите">
    ```text wrap theme={null}
    improve the generated documentation with more context and examples
    ```
  </Step>

  <Step title="Проверьте документацию">
    ```text wrap theme={null}
    check if the documentation follows our project standards
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Укажите стиль документации, который вы хотите (JSDoc, docstrings и т.д.)
  * Попросите примеры в документации
  * Запросите документацию для публичных API, интерфейсов и сложной логики
</Tip>

***

<h3 id="work-in-notes-and-non-code-folders">
  Работа с заметками и папками, не содержащими код
</h3>

Claude Code работает в любом каталоге. Запустите его внутри хранилища заметок, папки документации или любой коллекции файлов markdown для поиска, редактирования и переорганизации содержимого так же, как вы работаете с кодом.

Каталог `.claude/` и `CLAUDE.md` находятся рядом с каталогами конфигурации других инструментов без конфликтов. Claude читает файлы заново при каждом вызове инструмента, поэтому он видит изменения, которые вы вносите в другом приложении при следующем чтении этого файла.

***

<h3 id="work-with-images">
  Работа с изображениями
</h3>

Предположим, вам нужно работать с изображениями в вашей кодовой базе, и вы хотите помощь Claude в анализе содержимого изображения.

<Steps>
  <Step title="Добавьте изображение в разговор">
    Вы можете использовать любой из этих методов:

    1. Перетащите изображение в окно Claude Code
    2. Скопируйте изображение и вставьте его в CLI с помощью `Ctrl+V`, или с помощью [`Alt+V` на Windows и WSL](/docs/ru/interactive-mode#general-controls)
    3. Предоставьте Claude путь к изображению, например "Analyze this image: /path/to/your/image.png"
  </Step>

  <Step title="Попросите Claude проанализировать изображение">
    ```text wrap theme={null}
    What does this image show?
    ```

    ```text wrap theme={null}
    Describe the UI elements in this screenshot
    ```

    ```text wrap theme={null}
    Are there any problematic elements in this diagram?
    ```
  </Step>

  <Step title="Используйте изображения для контекста">
    ```text wrap theme={null}
    Here's a screenshot of the error. What's causing it?
    ```

    ```text wrap theme={null}
    This is our current database schema. How should we modify it for the new feature?
    ```
  </Step>

  <Step title="Получите предложения кода из визуального содержимого">
    ```text wrap theme={null}
    Generate CSS to match this design mockup
    ```

    ```text wrap theme={null}
    What HTML structure would recreate this component?
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Используйте изображения, когда текстовые описания были бы неясными или громоздкими
  * Включайте скриншоты ошибок, дизайны пользовательского интерфейса или диаграммы для лучшего контекста
  * Вы можете работать с несколькими изображениями в разговоре
  * Анализ изображений работает с диаграммами, скриншотами, макетами и многим другим
  * Когда Claude ссылается на изображения (например, `[Image #1]`), `Cmd+Click` (Mac) или `Ctrl+Click` (Windows/Linux) ссылку для открытия изображения в вашем средстве просмотра по умолчанию
</Tip>

***

<h3 id="reference-files-and-directories">
  Ссылка на файлы и каталоги
</h3>

Используйте @ для быстрого включения файлов или каталогов без ожидания, пока Claude их прочитает.

<Steps>
  <Step title="Ссылка на один файл">
    ```text wrap theme={null}
    Explain the logic in @src/utils/auth.js
    ```

    Это включает полное содержимое файла в разговор.
  </Step>

  <Step title="Ссылка на каталог">
    ```text wrap theme={null}
    What's the structure of @src/components?
    ```
  </Step>

  <Step title="Ссылка на MCP ресурсы">
    ```text wrap theme={null}
    Show me the data from @github:repos/owner/repo/issues
    ```

    Это получает данные из подключённых MCP серверов, используя формат @server:resource. См. [MCP ресурсы](/docs/ru/mcp#use-mcp-resources) для подробностей.
  </Step>
</Steps>

<Tip>
  Советы:

  * Пути к файлам могут быть относительными или абсолютными
  * Введите `@` для открытия меню предложений пути, затем нажмите Enter или Tab для принятия выделенного пути и снова Enter для отправки сообщения
  * Ссылки на файлы @ добавляют `CLAUDE.md` в каталог файла и родительские каталоги в контекст
  * Ссылки на каталоги показывают списки файлов, а не содержимое
  * Вы можете ссылаться на несколько файлов в одном сообщении (например, "@file1.js and @file2.js")
</Tip>

***

<h3 id="run-claude-on-a-schedule">
  Запуск Claude по расписанию
</h3>

Предположим, вы хотите, чтобы Claude автоматически выполнял задачу на повторяющейся основе, например просматривая открытые PR каждое утро, проверяя зависимости еженедельно или проверяя сбои CI в течение ночи.

Выберите вариант планирования на основе того, где вы хотите, чтобы задача выполнялась:

| Вариант                                                              | Где выполняется                            | Лучше всего для                                                                                                                                                                                                                    |
| :------------------------------------------------------------------- | :----------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Routines](/docs/ru/routines)                                             | Облако, управляемое Anthropic по умолчанию | Задачи, которые должны выполняться даже когда ваш компьютер выключен. Также может срабатывать на вызовы API или события GitHub в дополнение к расписанию. Настройте на [claude.ai/code/routines](https://claude.ai/code/routines). |
| [Запланированные задачи рабочего стола](/docs/ru/desktop-scheduled-tasks) | Ваша машина, через настольное приложение   | Задачи, которым нужен прямой доступ к локальным файлам, инструментам или незафиксированным изменениям.                                                                                                                             |
| [GitHub Actions](/docs/ru/github-actions)                                 | Ваш конвейер CI                            | Задачи, связанные с событиями репозитория, такими как открытые PR, или расписания cron, которые должны находиться рядом с конфигурацией рабочего процесса.                                                                         |
| [`/loop`](/docs/ru/scheduled-tasks)                                       | Текущий сеанс CLI                          | Быстрый опрос во время открытого сеанса. `--resume` и `--continue` восстанавливают неистекшие циклы с фиксированным интервалом.                                                                                                    |

<Tip>
  При написании подсказок для запланированных задач будьте явны в отношении того, что означает успех и что делать с результатами. Задача выполняется автономно, поэтому она не может задавать уточняющие вопросы. Например: "Просмотрите открытые PR с меткой `needs-review`, оставьте встроенные комментарии по любым проблемам и опубликуйте сводку в канале `#eng-reviews` Slack."
</Tip>

***

<h3 id="ask-claude-about-its-capabilities">
  Спросите Claude о его возможностях
</h3>

Claude имеет встроенный доступ к своей документации и может ответить на вопросы о своих собственных функциях и ограничениях.

<h4 id="example-questions">
  Примеры вопросов
</h4>

```text wrap theme={null}
can Claude Code create pull requests?
```

```text wrap theme={null}
how does Claude Code handle permissions?
```

```text wrap theme={null}
what skills are available?
```

```text wrap theme={null}
how do I use MCP with Claude Code?
```

```text wrap theme={null}
how do I configure Claude Code for Amazon Bedrock?
```

```text wrap theme={null}
what are the limitations of Claude Code?
```

<Note>
  Claude предоставляет ответы на основе документации на эти вопросы. Для исполняемых примеров запустите `/powerup` для интерактивных уроков с анимированными демонстрациями, или обратитесь к конкретным разделам рабочих процессов выше.
</Note>

<Tip>
  Советы:

  * Claude всегда имеет доступ к последней документации Claude Code, независимо от версии, которую вы используете
  * Задавайте конкретные вопросы для получения подробных ответов
  * Claude может объяснить сложные функции, такие как интеграция MCP, конфигурации предприятия и продвинутые рабочие процессы
</Tip>

***

<h2 id="resume-previous-conversations">
  Возобновление предыдущих разговоров
</h2>

Когда задача охватывает несколько сеансов, продолжайте с того места, где вы остановились, вместо того чтобы переобъяснять контекст. Claude Code сохраняет каждый разговор локально.

```bash theme={null}
claude --continue
```

Это возобновляет самый последний сеанс в текущем каталоге; если его ещё нет, выводится `No conversation found to continue` и программа выходит. Используйте `claude --resume` для выбора из списка, или `/resume` из активного сеанса. См. [Управление сеансами](/docs/ru/sessions) для именования, ветвления и полного справочника средства выбора.

<h2 id="run-parallel-sessions-with-worktrees">
  Запуск параллельных сеансов с worktrees
</h2>

Работайте над функцией в одном терминале, пока Claude исправляет ошибку в другом, без конфликтов редактирования. Каждый [git worktree](https://git-scm.com/docs/git-worktree) — это отдельный checkout на своей собственной ветке, созданный из существующего коммита, поэтому репозиторий должен иметь хотя бы один коммит в первую очередь.

```bash theme={null}
claude --worktree feature-auth
```

Запустите ту же команду с другим именем во втором терминале для запуска изолированного параллельного сеанса. В репозитории без коммитов команда завершается с ошибкой `Failed to resolve base branch "HEAD": git rev-parse failed`. См. [Worktrees](/docs/ru/worktrees) для очистки, `.worktreeinclude` и поддержки VCS, не основанной на git. Для мониторинга параллельных сеансов с одного экрана вместо отдельных терминалов см. [фоновые агенты](/docs/ru/agent-view).

<h2 id="plan-before-editing">
  Планирование перед редактированием
</h2>

Для изменений, которые вы хотите просмотреть перед их записью на диск, переключитесь в режим плана. Claude читает файлы и предлагает план, но не вносит изменения, пока вы не одобрите. Строка состояния показывает `⏸ plan mode on` во время активного режима плана.

```bash theme={null}
claude --permission-mode plan
```

Вы также можете нажать `Shift+Tab` во время сеанса до тех пор, пока строка состояния не покажет `⏸ plan mode on`. См. [Plan mode](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode) для потока одобрения и редактирования плана в вашем текстовом редакторе.

<h2 id="delegate-research-to-subagents">
  Делегирование исследований subagents
</h2>

Изучение большой кодовой базы заполняет ваш контекст чтением файлов. Делегируйте исследование, чтобы только результаты вернулись.

```text wrap theme={null}
use a subagent to investigate how our auth system handles token refresh
```

Subagent читает файлы в своём собственном контекстном окне и сообщает сводку. См. [Subagents](/docs/ru/sub-agents) для определения пользовательских агентов с их собственными инструментами и подсказками.

<h2 id="pipe-claude-into-scripts">
  Передача Claude в скрипты
</h2>

Запустите Claude неинтерактивно для CI, pre-commit hooks или пакетной обработки. Stdin и stdout работают как любой Unix инструмент.

```bash theme={null}
git log --oneline -20 | claude -p "summarize these recent commits"
```

См. [Неинтерактивный режим](/docs/ru/headless) для форматов вывода, флагов разрешений и паттернов fan-out.

<h2 id="next-steps">
  Следующие шаги
</h2>

<CardGroup cols={2}>
  <Card title="Лучшие практики" icon="lightbulb" href="/docs/ru/best-practices">
    Паттерны для получения максимума от Claude Code
  </Card>

  <Card title="Управление сеансами" icon="rotate-left" href="/docs/ru/sessions">
    Возобновление, именование и ветвление разговоров
  </Card>

  <Card title="Worktrees" icon="code-branch" href="/docs/ru/worktrees">
    Запуск изолированных параллельных сеансов
  </Card>

  <Card title="Расширение Claude Code" icon="puzzle-piece" href="/docs/ru/features-overview">
    Добавление skills, hooks, MCP, subagents и plugins
  </Card>
</CardGroup>
