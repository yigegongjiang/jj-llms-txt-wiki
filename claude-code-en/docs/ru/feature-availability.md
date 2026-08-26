> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Доступность функций

> Сравните, какие функции Claude Code доступны в планах подписки Anthropic, Anthropic Console, Amazon Bedrock, Claude Platform на AWS, Google Cloud's Agent Platform и Microsoft Foundry.

CLI Claude Code и всё, что работает локально, функционирует одинаково у каждого поставщика. Инструкции по настройке для каждого поставщика см. в разделе [Обзор развёртывания для предприятий](/docs/ru/third-party-integrations). Чтобы сразу перейти к тому, что отсутствует у вашего поставщика, см. вкладки [сводки по поставщикам](#summary-by-provider).

В таблицах ниже ✓ означает доступно, ✗ означает недоступно, а «See note» ссылается на сноску для частичной поддержки. Квалификатор после ✓ сужает доступность до этого подмножества, а «Admin-enabled» означает, что функция отключена до тех пор, пока администратор организации её не включит.

<h2 id="availability-by-model-provider">
  Доступность по поставщику модели
</h2>

Способ аутентификации определяет, какие функции может использовать Claude Code. Для единого списка того, что отсутствует у вашего поставщика, см. вкладки [сводки по поставщикам](#summary-by-provider). Чтобы найти свой столбец в таблицах:

* **Подписка Claude**: вы входите с учётной записью claude.ai в плане Pro, Max, Team или Enterprise
* **Anthropic Console**: вы аутентифицируетесь с помощью ключа API Anthropic
* **Amazon Bedrock**: вы используете модели Claude из каталога моделей Bedrock и устанавливаете `CLAUDE_CODE_USE_BEDROCK`. [Конечная точка Mantle](/docs/ru/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) охватывается этим столбцом
* **Claude Platform на AWS**: вы приобрели Claude через AWS Marketplace, но вызываете API Anthropic и устанавливаете `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**: управляется Google; вы устанавливаете `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**: управляется Anthropic на Azure; вы устанавливаете `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Функции, доступные у каждого поставщика
</h3>

Они работают у каждого поставщика:

* [CLI](/docs/ru/quickstart) и [Agent SDK](/docs/ru/agent-sdk/overview)
* Расширения [VS Code](/docs/ru/vs-code) и [JetBrains](/docs/ru/jetbrains)
* [Subagents](/docs/ru/sub-agents), [hooks](/docs/ru/hooks-guide), [commands](/docs/ru/commands) и [skills](/docs/ru/skills)
* [Память CLAUDE.md](/docs/ru/memory), [plugins](/docs/ru/plugins) и [MCP servers](/docs/ru/mcp)
* [Checkpoints](/docs/ru/checkpointing), [sandboxing](/docs/ru/sandboxing) и [Workflows](/docs/ru/workflows)
* [Метрики OpenTelemetry](/docs/ru/monitoring-usage) и [управляемый файл параметров](/docs/ru/settings#settings-files)

Три из них имеют различия, зависящие от поставщика:

* **MCP servers**: [коннекторы из claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai) загружаются только когда ваша подписка claude.ai является активным методом аутентификации, и [поиск инструментов](/docs/ru/mcp#configure-tool-search) отключён по умолчанию на Google Cloud's Agent Platform и когда `ANTHROPIC_BASE_URL` указывает на хост, не принадлежащий первой стороне
* **Subagents**: встроенный [Explore subagent](/docs/ru/sub-agents#built-in-subagents) ограничивает унаследованную модель до Opus на Claude API и наследует модель основного разговора напрямую у любого другого поставщика, включая Claude Platform на AWS
* **[Commands](/docs/ru/commands#all-commands)**: `/design-sync` и `/radio` недоступны на Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry и Claude Platform на AWS, а `/voice` требует учётную запись claude.ai

<h3 id="features-that-require-a-claude-subscription">
  Функции, требующие подписку Claude
</h3>

Они требуют входа с учётной записью claude.ai и недоступны с ключом API Anthropic Console или от поставщика третьей стороны:

* [Claude Code в веб-версии](/docs/ru/claude-code-on-the-web), Claude Code на мобильном устройстве и [Claude Code в Slack](/docs/ru/slack)
* [Claude Code Desktop](/docs/ru/desktop)
* [Routines](/docs/ru/routines) (`/schedule`)
* [Ultraplan](/docs/ru/ultraplan) и [Ultrareview](/docs/ru/ultrareview)
* [Code Review](/docs/ru/code-review): планы Team и Enterprise
* [Remote Control](/docs/ru/remote-control)
* [Расширение Chrome](/docs/ru/chrome)
* [Computer use](/docs/ru/computer-use): планы Pro и Max
* [Artifacts](/docs/ru/artifacts): планы Pro, Max, Team и Enterprise
* [Voice dictation](/docs/ru/voice-dictation)

Desktop — частичное исключение: [маршрутизация шлюза может быть настроена в приложении или администратором](/docs/ru/llm-gateway-connect#desktop-app), развёртывания Enterprise могут маршрутизировать Desktop на Google Cloud's Agent Platform или поставщика шлюза через [управляемые параметры](https://claude.com/docs/third-party/claude-desktop/configuration), а [Claude Desktop на 3P](https://claude.com/docs/third-party/claude-desktop/overview) запускает вкладку Code на Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или самостоятельно размещённом шлюзе LLM. Для доступности этих функций по плану см. [Доступность по плану подписки](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  Возможности CLI, которые различаются по поставщикам
</h3>

Эти функции работают в локальном CLI, но зависят от возможности на стороне сервера, которую не каждый поставщик предоставляет.

<table>
  <thead>
    <tr>
      <th>Функция</th>
      <th>Подписка Claude</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform на AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/ru/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>См. примечание <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/ru/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/ru/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>См. примечание <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>См. примечание <sup><a href="#fn2">2</a></sup></td>
      <td>См. примечание <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/ru/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/ru/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` scheduled tasks](/docs/ru/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>См. примечание <sup><a href="#fn3">3</a></sup></td>
      <td>См. примечание <sup><a href="#fn3">3</a></sup></td>
      <td>См. примечание <sup><a href="#fn3">3</a></sup></td>
      <td>См. примечание <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/ru/github-actions) и [GitLab CI/CD](/docs/ru/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  Администрирование и аналитика
</h3>

Элементы управления на уровне организации и видимость использования.

<table>
  <thead>
    <tr>
      <th>Функция</th>
      <th>Подписка Claude</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform на AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Analytics dashboard и API](/docs/ru/analytics)</td>
      <td>✓ (панель управления: Team и Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Server-managed settings](/docs/ru/server-managed-settings)</td>
      <td>✓ (Team и Enterprise)</td>
      <td>✓ (Team и Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/ru/zero-data-retention)</td>
      <td>✓ (квалифицированные Enterprise аккаунты)</td>
      <td>✓ (квалифицированные аккаунты)</td>
      <td>См. примечание <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (квалифицированные аккаунты)</td>
      <td>См. примечание <sup><a href="#fn4">4</a></sup></td>
      <td>См. примечание <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> На Google Cloud's Agent Platform веб-поиск доступен для моделей Claude 4 и более поздних версий.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> На этих поставщиках auto mode поддерживает только Claude Sonnet 5, Opus 4.7 и Opus 4.8. См. [Конфигурация Auto mode](/docs/ru/auto-mode-config). В v2.1.158 через v2.1.206 auto mode на этих поставщиках также требовал установки `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 удалил это требование.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Явные интервалы, такие как `/loop every 2 hours`, работают у каждого поставщика. На Amazon Bedrock, Claude Platform на AWS, Google Cloud's Agent Platform и Microsoft Foundry `/loop` не может выбрать свой собственный интервал или предоставить подсказку обслуживания по умолчанию, поэтому подсказка без интервала выполняется каждые 10 минут, а `/loop` без аргументов показывает сообщение об использовании. См. [Scheduled tasks](/docs/ru/scheduled-tasks).<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> В соответствии с вашим соглашением с поставщиком облачных услуг.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Только панель управления и API. [Contribution metrics](/docs/ru/analytics#enable-contribution-metrics) требует организацию claude.ai Team или Enterprise.

<Note>
  Если вы аутентифицируетесь через [LLM gateway](/docs/ru/llm-gateway), доступность функций соответствует базовому поставщику, на который шлюз перенаправляет запросы. Некоторые функции, доступные только для Anthropic, такие как [Advisor](/docs/ru/advisor), работают только если шлюз перенаправляет запросы без изменений на API Anthropic.
</Note>

<h3 id="summary-by-provider">
  Сводка по поставщикам
</h3>

Каждая вкладка содержит список того, что недоступно или частично поддерживается у этого поставщика, с альтернативами, где они существуют. Всё, что не указано, работает так же, как в подписке Claude, кроме [различий, зависящих от поставщика](#features-available-on-every-provider), отмеченных выше. На Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry и Claude Platform на AWS отчёты об ошибках и телеметрия для Anthropic отключены по умолчанию. См. [поведение по умолчанию по поставщику API](/docs/ru/data-usage#default-behaviors-by-api-provider) для информации о том, какой трафик всё ещё достигает Anthropic и как отказаться.

<Tabs>
  <Tab title="Amazon Bedrock">
    **Недоступно:** все [функции, требующие подписку Claude](#features-that-require-a-claude-subscription), плюс [веб-поиск](/docs/ru/tools-reference#websearch-tool-behavior), [fast mode](/docs/ru/fast-mode), [Advisor](/docs/ru/advisor), [Channels](/docs/ru/channels), [панель управления аналитики](/docs/ru/analytics), [server-managed settings](/docs/ru/server-managed-settings) и [команды `/design-sync` и `/radio`](/docs/ru/commands#all-commands).

    **Частичная поддержка:**

    * [Desktop](/docs/ru/desktop): только через [Claude Desktop на 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/ru/auto-mode-config): только Sonnet 5, Opus 4.7 и Opus 4.8
    * [`/loop`](/docs/ru/scheduled-tasks): только явные интервалы
    * [Zero Data Retention](/docs/ru/zero-data-retention): в соответствии с вашим соглашением AWS

    **Альтернативы:** для планирования используйте [`/loop`](/docs/ru/scheduled-tasks) с явным интервалом вместо `/schedule`. Для облачных сеансов используйте [GitHub Actions](/docs/ru/github-actions) или [GitLab CI/CD](/docs/ru/gitlab-ci-cd). Для веб-поиска используйте инструмент [WebFetch](/docs/ru/tools-reference#webfetch-tool-behavior) с конкретным URL.
  </Tab>

  <Tab title="Claude Platform на AWS">
    **Недоступно:** все [функции, требующие подписку Claude](#features-that-require-a-claude-subscription), плюс [fast mode](/docs/ru/fast-mode), [Advisor](/docs/ru/advisor), [Channels](/docs/ru/channels), [панель управления аналитики](/docs/ru/analytics), [server-managed settings](/docs/ru/server-managed-settings) и [команды `/design-sync` и `/radio`](/docs/ru/commands#all-commands).

    **Доступно** там, где Amazon Bedrock недоступен: [веб-поиск](/docs/ru/tools-reference#websearch-tool-behavior).

    **Частичная поддержка:**

    * [`/loop`](/docs/ru/scheduled-tasks): только явные интервалы

    **Альтернативы:** для планирования используйте [`/loop`](/docs/ru/scheduled-tasks) с явным интервалом вместо `/schedule`. Для облачных сеансов используйте [GitHub Actions](/docs/ru/github-actions) или [GitLab CI/CD](/docs/ru/gitlab-ci-cd).
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **Недоступно:** все [функции, требующие подписку Claude](#features-that-require-a-claude-subscription), плюс [fast mode](/docs/ru/fast-mode), [Advisor](/docs/ru/advisor), [Channels](/docs/ru/channels), [панель управления аналитики](/docs/ru/analytics), [server-managed settings](/docs/ru/server-managed-settings) и [команды `/design-sync` и `/radio`](/docs/ru/commands#all-commands).

    **Частичная поддержка:**

    * [Desktop](/docs/ru/desktop): через [управляемые параметры](https://claude.com/docs/third-party/claude-desktop/configuration) или [Claude Desktop на 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/ru/tools-reference#websearch-tool-behavior): модели Claude 4 и более поздние
    * [Auto mode](/docs/ru/auto-mode-config): только Sonnet 5, Opus 4.7 и Opus 4.8
    * [`/loop`](/docs/ru/scheduled-tasks): только явные интервалы
    * [Zero Data Retention](/docs/ru/zero-data-retention): в соответствии с вашим соглашением Google Cloud

    **Альтернативы:** для планирования используйте [`/loop`](/docs/ru/scheduled-tasks) с явным интервалом вместо `/schedule`. Для облачных сеансов используйте [GitHub Actions](/docs/ru/github-actions) или [GitLab CI/CD](/docs/ru/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **Недоступно:** все [функции, требующие подписку Claude](#features-that-require-a-claude-subscription), плюс [fast mode](/docs/ru/fast-mode), [Advisor](/docs/ru/advisor), [Channels](/docs/ru/channels), [GitHub Actions](/docs/ru/github-actions) и [GitLab CI/CD](/docs/ru/gitlab-ci-cd), [панель управления аналитики](/docs/ru/analytics), [server-managed settings](/docs/ru/server-managed-settings) и [команды `/design-sync` и `/radio`](/docs/ru/commands#all-commands).

    **Частичная поддержка:**

    * [Desktop](/docs/ru/desktop): только через [Claude Desktop на 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/ru/auto-mode-config): только Sonnet 5, Opus 4.7 и Opus 4.8
    * [`/loop`](/docs/ru/scheduled-tasks): только явные интервалы
    * [Zero Data Retention](/docs/ru/zero-data-retention): в соответствии с вашим соглашением Azure

    **Альтернативы:** для планирования используйте [`/loop`](/docs/ru/scheduled-tasks) с явным интервалом вместо `/schedule`.
  </Tab>

  <Tab title="Anthropic Console">
    **Недоступно:** все [функции, требующие подписку Claude](#features-that-require-a-claude-subscription).

    Всё в разделе [Возможности CLI, которые различаются по поставщикам](#cli-capabilities-that-vary-by-provider) доступно, как и [server-managed settings](/docs/ru/server-managed-settings) когда ключ API принадлежит организации Team или Enterprise.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Доступность по плану подписки
</h2>

Если вы аутентифицируетесь через Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или ключ API Anthropic Console, этот раздел к вам не применяется. Когда вы входите с учётной записью claude.ai, ваш план определяет, какие из приведённых ниже функций доступны.

| Функция                                                                     | Pro | Max | Team          | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :------------ | :-------------------------------- |
| [Claude Code в веб-версии](/docs/ru/claude-code-on-the-web)                      | ✓   | ✓   | ✓             | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/ru/routines)                                                    | ✓   | ✓   | ✓             | ✓                                 |
| [Remote Control](/docs/ru/remote-control)                                        | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Channels](/docs/ru/channels)                                                    | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Computer use](/docs/ru/computer-use)                                            | ✓   | ✓   | ✗             | ✗                                 |
| Dispatch ([Desktop](/docs/ru/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗             | ✗                                 |
| [Code Review](/docs/ru/code-review)                                              | ✗   | ✗   | ✓             | ✓                                 |
| [Artifacts](/docs/ru/artifacts)                                                  | ✓   | ✓   | ✓             | Admin-enabled                     |
| [Analytics dashboard and contribution metrics](/docs/ru/analytics)               | ✗   | ✗   | ✓             | ✓                                 |
| [Enterprise Analytics API](/docs/ru/analytics#access-data-programmatically)      | ✗   | ✗   | ✗             | ✓                                 |
| [Server-managed settings](/docs/ru/server-managed-settings)                      | ✗   | ✗   | ✓             | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓             | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗             | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗             | ✓                                 |
| [Zero Data Retention](/docs/ru/zero-data-retention)                              | ✗   | ✗   | ✗             | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> На Enterprise требуется премиум-место или место Chat + Claude Code. См. [Claude Code в веб-версии](/docs/ru/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> Не включено в стандартный план Enterprise. Требует отдельного включения Anthropic для квалифицированных аккаунтов. См. [Zero Data Retention](/docs/ru/zero-data-retention).

Для информации о ценах и полного сравнения планов см. [Team plans](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) и [Enterprise plans](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Доступность моделей
</h2>

Для информации о том, какие модели Claude и размеры контекстного окна доступны для каждого поставщика и региона, см. [Model configuration](/docs/ru/model-config) и [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, PDF input и extended thinking — это возможности модели, а не функции Claude Code, и работают у каждого поставщика, который предлагает модель. [Prompt caching](/docs/ru/prompt-caching) работает одинаково у большинства поставщиков; на Amazon Bedrock поддержка варьируется в зависимости от модели.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Обзор развёртывания для предприятий](/docs/ru/third-party-integrations): сравните аутентификацию, выставление счётов и регионы у поставщиков
* Руководства по настройке поставщиков: [Amazon Bedrock](/docs/ru/amazon-bedrock), [Claude Platform на AWS](/docs/ru/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), [Microsoft Foundry](/docs/ru/microsoft-foundry)
* [Platforms and integrations](/docs/ru/platforms): где работает Claude Code, включая CLI, Desktop, расширения IDE, веб, мобильные устройства и CI/CD
