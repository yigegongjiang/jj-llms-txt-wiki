> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Платформы и интеграции

> Выберите, где запустить Claude Code и что к нему подключить. Сравните CLI, Desktop, VS Code, JetBrains, веб, мобильные приложения и интеграции, такие как Chrome, Slack и CI/CD.

Claude Code запускает один и тот же базовый движок везде, но каждая поверхность оптимизирована для разного способа работы. Эта страница поможет вам выбрать правильную платформу для вашего рабочего процесса и подключить инструменты, которые вы уже используете.

<h2 id="where-to-run-claude-code">
  Где запустить Claude Code
</h2>

Выберите платформу в зависимости от того, как вы предпочитаете работать и где находится ваш проект.

| Платформа                         | Лучше всего для                                                                                                       | Что вы получаете                                                                                                                                                                           |
| :-------------------------------- | :-------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/ru/quickstart)             | Рабочие процессы терминала, скриптинг, удалённые серверы                                                              | Полный набор функций, [Agent SDK](/docs/ru/headless), [использование компьютера](/docs/ru/computer-use) на macOS (Pro и Max), сторонние поставщики                                                   |
| [Desktop](/docs/ru/desktop)            | Визуальный просмотр, параллельные сеансы, управляемая установка                                                       | Средство просмотра различий, предпросмотр приложения, [использование компьютера](/docs/ru/desktop#let-claude-use-your-computer) и [Dispatch](/docs/ru/desktop#sessions-from-dispatch) на Pro и Max   |
| [VS Code](/docs/ru/vs-code)            | Работа внутри VS Code без переключения на терминал                                                                    | Встроенные различия, интегрированный терминал, контекст файла                                                                                                                              |
| [JetBrains](/docs/ru/jetbrains)        | Работа внутри IntelliJ, PyCharm, WebStorm или других IDE JetBrains                                                    | Средство просмотра различий, совместное использование выделения, сеанс терминала                                                                                                           |
| [Web](/docs/ru/claude-code-on-the-web) | Долгосрочные задачи, которые не требуют большого управления, или работа, которая должна продолжаться, когда вы в сети | Облако, управляемое Anthropic, продолжается после отключения                                                                                                                               |
| Mobile                            | Запуск и мониторинг задач вдали от вашего компьютера                                                                  | Облачные сеансы из приложения Claude для iOS и Android, [Remote Control](/docs/ru/remote-control) для локальных сеансов, [Dispatch](/docs/ru/desktop#sessions-from-dispatch) на Desktop на Pro и Max |

CLI — это наиболее полная поверхность для работы, ориентированной на терминал: скриптинг и Agent SDK доступны только в CLI. Сторонние поставщики также работают в [VS Code](/docs/ru/vs-code#use-third-party-providers). Корпоративные развёртывания [Desktop](/docs/ru/desktop) поддерживают Google Cloud's Agent Platform, и Desktop поддерживает [поставщиков шлюза](/docs/ru/llm-gateway-connect#desktop-app); для Amazon Bedrock или Microsoft Foundry используйте CLI или VS Code, или [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), который запускает вкладку Code на этих поставщиках. Desktop и расширения IDE обменивают некоторые функции, доступные только в CLI, на визуальный просмотр и более тесную интеграцию с редактором. Веб работает в облаке Anthropic, поэтому задачи продолжают выполняться после отключения. Mobile — это тонкий клиент для этих же облачных сеансов или для локального сеанса через Remote Control, и может отправлять задачи на Desktop с помощью Dispatch.

Вы можете смешивать поверхности в одном проекте. Конфигурация, память проекта и MCP серверы совместно используются на локальных поверхностях.

<h2 id="connect-your-tools">
  Подключите ваши инструменты
</h2>

Интеграции позволяют Claude работать с сервисами вне вашей кодовой базы.

| Интеграция                           | Что она делает                                              | Используйте для                                                              |
| :----------------------------------- | :---------------------------------------------------------- | :--------------------------------------------------------------------------- |
| [Chrome](/docs/ru/chrome)                 | Управляет вашим браузером с вашими авторизованными сеансами | Тестирование веб-приложений, заполнение форм, автоматизация сайтов без API   |
| [GitHub Actions](/docs/ru/github-actions) | Запускает Claude в вашем конвейере CI                       | Автоматические проверки PR, сортировка проблем, запланированное обслуживание |
| [GitLab CI/CD](/docs/ru/gitlab-ci-cd)     | То же самое, что GitHub Actions для GitLab                  | Автоматизация, управляемая CI на GitLab                                      |
| [Code Review](/docs/ru/code-review)       | Автоматически проверяет каждый PR                           | Выявление ошибок перед проверкой человеком                                   |
| [Slack](/docs/ru/slack)                   | Отвечает на упоминания `@Claude` в ваших каналах            | Превращение отчётов об ошибках в pull requests из чата команды               |

Для интеграций, не указанных здесь, [MCP серверы](/docs/ru/mcp) и [соединители](/docs/ru/desktop#connect-external-tools) позволяют подключить почти всё: Linear, Notion, Google Drive или ваши собственные внутренние API.

<h2 id="work-when-you-are-away-from-your-terminal">
  Работайте, когда вы вдали от вашего терминала
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

Если вы не уверены, с чего начать, [установите CLI](/docs/ru/quickstart) и запустите его в каталоге проекта. Если вы предпочитаете не использовать терминал, [Desktop](/docs/ru/desktop-quickstart) предоставляет вам тот же движок с графическим интерфейсом.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

<h3 id="platforms">
  Платформы
</h3>

* [Быстрый старт CLI](/docs/ru/quickstart): установка и запуск вашей первой команды в терминале
* [Desktop](/docs/ru/desktop): визуальный просмотр различий, параллельные сеансы, использование компьютера и Dispatch
* [VS Code](/docs/ru/vs-code): расширение Claude Code внутри вашего редактора
* [JetBrains](/docs/ru/jetbrains): расширение для IntelliJ, PyCharm и других IDE JetBrains
* [Claude Code в веб](/docs/ru/claude-code-on-the-web): облачные сеансы, которые продолжают работать при отключении
* Mobile: приложение Claude для [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) и [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) для запуска и мониторинга задач вдали от вашего компьютера

<h3 id="integrations">
  Интеграции
</h3>

* [Chrome](/docs/ru/chrome): автоматизация задач браузера с вашими авторизованными сеансами
* [Использование компьютера](/docs/ru/computer-use): позвольте Claude открывать приложения и управлять вашим экраном на macOS
* [GitHub Actions](/docs/ru/github-actions): запуск Claude в вашем конвейере CI
* [GitLab CI/CD](/docs/ru/gitlab-ci-cd): то же самое для GitLab
* [Code Review](/docs/ru/code-review): автоматическая проверка при каждом pull request
* [Slack](/docs/ru/slack): отправка задач из чата команды, получение PR обратно

<h3 id="remote-access">
  Удалённый доступ
</h3>

* [Dispatch](/docs/ru/desktop#sessions-from-dispatch): отправьте задачу со своего телефона, и она может создать сеанс Desktop
* [Remote Control](/docs/ru/remote-control): управляйте работающим сеансом со своего телефона или браузера
* [Channels](/docs/ru/channels): отправляйте события из приложений чата или ваших собственных серверов в сеанс
* [Scheduled tasks](/docs/ru/scheduled-tasks): запускайте подсказки по повторяющемуся расписанию
