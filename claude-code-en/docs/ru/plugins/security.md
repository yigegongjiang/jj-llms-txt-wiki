> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Безопасность и доверие к плагинам

> Решите, доверять ли плагину перед его установкой: от того, что плагин может делать на вашем компьютере, до того, как его проверить и удалить.

Плагин Claude Code, который вы устанавливаете, может выполнять произвольный код на вашем компьютере с вашими привилегиями пользователя.

Вы устанавливаете плагин из marketplace, который является каталогом, из которого Claude Code его получает. Некоторые названия marketplace [зарезервированы для собственных marketplace Anthropic](#marketplace-tiers), а все остальные marketplace являются сторонними. Название marketplace говорит вам, кто публикует каталог, а не то, что делает каждый плагин в нём, поэтому [проверьте плагин перед установкой](#review-a-plugin-before-you-install) независимо от того, из какого marketplace он поступает.

Прочитайте эту страницу, если вы решаете, устанавливать ли плагин, или если вы проверяете инструменты перед тем, как ваша команда сможет их использовать.

<Note>
  Эти случаи рассматриваются на других страницах:

  * **Собственная модель безопасности Claude Code**: см. [Security](/docs/ru/security)
  * **Ограничение или требование плагинов для организации**: см. [Manage plugins for your organization](/docs/ru/plugins/org)
  * **Плагины `security-guidance` или `claude-security`**: эта страница о них не идёт. См. [`security-guidance`](/docs/ru/security-guidance) и [`claude-security`](/docs/ru/claude-security)
</Note>

Начните с [того, что может делать плагин](#understand-what-a-plugin-can-do) и [какие marketplace принадлежат Anthropic](#marketplace-tiers), затем [проверьте плагин перед установкой](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Поймите, что может делать плагин
</h2>

Плагин может содержать контент, который выполняет код на вашей машине с вашими привилегиями пользователя, и контент, который входит в контекст Claude в качестве инструкций, поэтому [перед установкой плагина его следует проверить](#review-a-plugin-before-you-install). Вот что может делать установленный плагин:

* **Hooks**: [hooks](/docs/ru/hooks) плагина выполняются как команды shell в определённых точках жизненного цикла Claude Code, например до или после вызова инструмента.
* **MCP и LSP серверы**: Claude Code подключается к [MCP серверам](/docs/ru/mcp), которые объявляет включённый плагин, и предоставляет Claude их инструменты. Stdio MCP сервер работает как процесс, который Claude Code запускает на вашей машине. Claude Code также запускает языковые серверы, которые объявляет плагин.
* **`bin/` директория**: Claude Code добавляет директорию `bin/` каждого включённого плагина в `PATH` shell инструмента Bash, поэтому команды Bash от Claude могут запускать любой исполняемый файл там.
* **Skills, команды и агенты**: они входят в контекст Claude в качестве инструкций, поэтому они влияют на то, что Claude делает с инструментами, которые у него уже есть.
* **Обновления**: когда автоматическое обновление включено для маркетплейса, из которого вы установили плагин, Claude Code обновляет этот плагин в фоновом режиме, поэтому файлы, которые вы проверили, могут измениться на диске. [Когда выполняется автоматическое обновление](/docs/ru/plugins/loading#when-auto-update-runs) содержит информацию о времени. Чтобы включить или отключить автоматическое обновление для каждого маркетплейса, см. [Держите плагины в актуальном состоянии](/docs/ru/plugins/install#keep-plugins-updated).

[Правила разрешений](/docs/ru/permissions) Claude Code и [sandbox](/docs/ru/sandboxing) охватывают вызовы инструментов, которые делает Claude, а не код, который плагин запускает самостоятельно:

* **Hooks и серверные процессы**: command hooks выполняют команды shell с вашими полными привилегиями пользователя. Claude Code запускает hooks и MCP серверы вне sandbox.
* **Вызовы инструментов Claude**: вызов одного из инструментов MCP плагина и команда Bash, которая запускает исполняемый файл из `bin/` плагина, являются вызовами инструментов, поэтому к ним применяются ваши правила разрешений.

Установка плагина также включает его, если только его манифест или запись маркетплейса не устанавливают [`defaultEnabled: false`](/docs/ru/plugins/install#choose-an-install-scope) и вы сами его не включили.

Чтобы удалить плагин, которому вы больше не доверяете, см. [Удалите плагин, которому вы больше не доверяете](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Определите marketplace Anthropic по названию
</h2>

Название marketplace помещает его в один из трёх уровней: official, community или third-party. Claude Code принимает названия official и community только для marketplace, полученных из репозиториев `github.com/anthropics/`, поэтому сторонний marketplace не может выдавать себя за marketplace Anthropic. Marketplace, который публикует коллега или ваша организация, является third-party.

В таблице указано, какие названия относятся к каждому уровню:

| Уровень     | Какие marketplace                                                                                    |
| :---------- | :--------------------------------------------------------------------------------------------------- |
| Official    | [Официальные названия marketplace](#official-marketplace-names), такие как `claude-plugins-official` |
| Community   | `claude-community`, `claude-plugins-community` и `healthcare`                                        |
| Third-party | Все остальные marketplace                                                                            |

Где каталог `claude-community` закрепляет плагин к commit SHA, что он делает почти для каждой записи, Claude Code отказывается устанавливать другой commit.

<h3 id="official-marketplace-names">
  Официальные названия marketplace
</h3>

Эти названия marketplace составляют official уровень:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

О том, чем отличаются official, community и demo marketplace и где можно просмотреть, что указано в каждом, см. [Anthropic's marketplaces](/docs/ru/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Проверьте плагин перед установкой
</h2>

Перед установкой плагина посмотрите, что он добавляет и откуда он поступает.

<Steps>
  <Step title="Проверьте источник marketplace">
    В вашей shell выполните `claude plugin marketplace list`, чтобы вывести источник, из которого был добавлен каждый marketplace, например репозиторий GitHub или директорию.
  </Step>

  <Step title="Прочитайте панель деталей">
    В сеансе Claude Code выполните `/plugin` и выберите плагин. Панель деталей показывает раздел **Will install**, в котором перечислены команды, agents, skills, hooks и MCP и LSP серверы плагина. Для плагина, для которого Anthropic не опубликовала данные компонентов, раздел показывает то, что объявляет запись marketplace, или примечание: `Components will be discovered at installation` для плагина, хранящегося внутри marketplace, или `Component summary not available for remote plugin` для плагина, полученного откуда-то ещё.
  </Step>

  <Step title="Прочитайте исходный код плагина">
    На панели деталей выберите **Open homepage** или **View on GitHub** под опциями установки. Если панель не предлагает ни того, ни другого, откройте репозиторий marketplace, который вы нашли на первом шаге. Найдите там директорию плагина. Раздел **Will install** показывает, что hook существует, но не то, что он запускает, поэтому прочитайте эти файлы в директории плагина:

    * **`hooks/hooks.json`**: команда, которую запускает каждый hook
    * **`.mcp.json`**: команда или URL каждого сервера
    * **`bin/`**: каждый файл в директории
  </Step>

  <Step title="Перечислите содержимое плагина">
    Клонируйте репозиторий, который содержит директорию плагина, затем выполните `claude --plugin-dir <plugin directory> plugin details <plugin name>` в вашей shell, чтобы увидеть, что Claude Code находит в нём. Команда читает файлы плагина без запуска сеанса и выводит `Component inventory`, в котором перечислены skills и commands плагина, agents, hooks с событием каждого hook и MCP и LSP серверы.
  </Step>
</Steps>

После установки плагина выполните `claude plugin details <plugin name>` в вашей shell, чтобы вывести тот же `Component inventory` для установленной копии под `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Удалите плагин, которому вы больше не доверяете
</h3>

В вашей shell выполните [`claude plugin uninstall <plugin>`](/docs/ru/plugins/cli-reference#plugin-uninstall) с `--scope`, в котором вы его установили. Затем проверьте, что удалила деинсталляция и что она оставила:

* **Постоянные данные**: когда это был последний scope, в котором был установлен плагин, деинсталляция также удаляет директорию постоянных данных плагина, если вы не передадите `--keep-data`.
* **Кэшированные файлы**: файлы плагина остаются на диске под `~/.claude/plugins/cache/` в течение 14 дней, прежде чем [фоновая очистка их удалит](/docs/ru/plugins/loading#cleanup-of-previous-versions). После деинсталляции последнего плагина orphaned директории остаются, пока вы не установите другой. Чтобы удалить файлы сейчас, удалите директорию плагина под `~/.claude/plugins/cache/<marketplace>/<plugin>/` самостоятельно.
* **Marketplace**: если вы также не доверяете владельцу marketplace, [удалите marketplace](/docs/ru/plugins/install#manage-marketplaces) тоже, что деинсталлирует каждый плагин, который вы установили из него.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Распознайте, когда Claude Code отказывает или предупреждает
</h2>

Панель деталей, которую вы открываете из вкладки **Discover** или **Marketplaces** в `/plugin`, показывает одно и то же предупреждение о доверии для каждого плагина. Claude Code отказывает вместо предупреждения в случаях, таких как те, что указаны в [Untrusted marketplace sources and failed integrity checks](#untrusted-marketplace-sources-and-failed-integrity-checks).

<h3 id="trust-warning-before-you-install">
  Предупреждение о доверии перед установкой
</h3>

Предупреждение читается одинаково независимо от того, из какого marketplace поступает плагин:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Если ваша организация устанавливает `pluginTrustMessage` в [managed settings](/docs/ru/plugins/org), Claude Code добавляет этот текст к предупреждению.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Ненадёжные источники marketplace и неудачные проверки целостности
</h3>

Claude Code отказывает загружать marketplace или устанавливать плагин в этих случаях, каждый со своим сообщением об ошибке:

* **Ненадёжный источник marketplace**: когда marketplace использует официальное или community название, но его источник находится вне `github.com/anthropics/`, Claude Code прекращает загрузку marketplace и плагинов, которые вы установили из него. Ошибка: [Marketplace is registered from an untrusted source](/docs/ru/errors#marketplace-is-registered-from-an-untrusted-source).
* **Целостность архива**: когда запись marketplace закрепляет [`archive` источник](/docs/ru/plugins/marketplace-reference#archive-plugin-source) к дайджесту `sha256` и дайджест загруженного файла не совпадает с ним, Claude Code отказывает установку. Ошибка: [Plugin archive integrity check failed](/docs/ru/errors#plugin-archive-integrity-check-failed).

Закрепление `sha256` отделено от закрепления commit SHA каталога community, которое выбирает git commit для проверки.

<h2 id="enforce-plugin-controls-for-your-organization">
  Применяйте элементы управления плагинами для вашей организации
</h2>

С помощью [managed settings](/docs/ru/plugins/org) администратор может применять эти элементы управления плагинами:

* Allowlist или blocklist источников marketplace
* Force-enable плагины
* Отключить флаги `--plugin-dir` и `--plugin-url` и переменную `CLAUDE_CODE_PLUGIN_DIRS`
* Ограничить hooks теми, которые из managed settings и force-enabled плагинов
* Остановить загрузку плагинов из учётных записей claude.ai членов в Claude Code с помощью [`syncClaudeAiPlugins`](/docs/ru/plugins/org#control-matrix)

[Матрица управления](/docs/ru/plugins/org#control-matrix) говорит, что делает каждый ключ и что он не охватывает.

<h2 id="find-plugins-in-telemetry">
  Найдите плагины в телеметрии
</h2>

Если ваша организация экспортирует [OpenTelemetry события](/docs/ru/monitoring-usage) Claude Code на свой собственный backend, [уровни marketplace](#marketplace-tiers) определяют, какие названия плагинов появляются там:

* **[Plugin loaded event](/docs/ru/monitoring-usage#plugin-loaded-event)**: событие сообщает официальные названия плагинов и marketplace как они есть. Для уровней community и third-party `plugin.name` и `marketplace.name` являются буквальной строкой `third-party`, если вы не установите `OTEL_LOG_TOOL_DETAILS=1`.
* **Plugin scope**: `plugin.scope` загруженного события по-прежнему сообщает, откуда поступил плагин, например `org` для плагина, который включают ваши managed settings, или `user-local` для любого другого плагина third-party. [Plugin loaded event](/docs/ru/monitoring-usage#plugin-loaded-event) перечисляет каждое значение.
* **[Plugin installed event](/docs/ru/monitoring-usage#plugin-installed-event)**: если вы не установите `OTEL_LOG_TOOL_DETAILS=1`, событие опускает поля имён для плагинов, не являющихся official, вместо того чтобы сообщать `third-party`.
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code сообщает плагины из official и community уровней по названию и сообщает каждый другой плагин как `third-party`.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Manage plugins for your organization](/docs/ru/plugins/org): ограничьте, из каких marketplace пользователи могут устанавливать, и требуйте те, которым вы доверяете
* [Install and manage plugins](/docs/ru/plugins/install): проверьте панель деталей плагина перед выбором scope
* [Anthropic's marketplaces](/docs/ru/plugins/anthropic-marketplaces): какие названия marketplace принадлежат Anthropic
* [Security](/docs/ru/security): собственная модель безопасности Claude Code
