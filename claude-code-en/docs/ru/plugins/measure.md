> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Измерение стоимости и использования плагина

> Измерьте стоимость токенов плагина Claude Code, узнайте, используют ли его люди, и выберите события телеметрии для вопросов плагинов на уровне организации.

Каждый сеанс, в котором включен плагин, включает имена и описания его skills, agents и commands в контексте Claude, и эти токены учитываются в использовании пользователя независимо от того, используется ли плагин или нет. На этой странице показано, как увидеть это число для плагина, как его уменьшить, если вы поддерживаете плагин, и где отображается использование, чтобы вы могли определить, все ли еще используется плагин.

Эта страница предназначена для авторов и разработчиков плагинов. Если вы администрируете Claude Code для организации, раздел [Измерение по всему парку](#measure-across-a-fleet) охватывает те же вопросы на каждой машине.

<Note>
  Эти случаи рассматриваются на других страницах:

  * **Тестирование того, насколько надежно плагин изменяет поведение Claude**: см. [Тестирование плагинов с помощью evals](/docs/ru/plugin-evals)
  * **Сокращение контекста вашего собственного сеанса**: см. [Управление установленными плагинами](/docs/ru/plugins/install#manage-installed-plugins) и страницу [контекстного окна](/docs/ru/context-window)
</Note>

Начните с раздела [Измерение стоимости плагина](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Измерение стоимости плагина
</h2>

Чтобы увидеть, что плагин добавляет в контекст Claude, запустите [`claude plugin details`](/docs/ru/plugins/cli-reference#plugin-details) с именем плагина. Вы запускаете его в своей оболочке, а не в приглашении запущенного сеанса Claude Code. Плагин должен быть загружен: установлен, находиться в каталоге skills или передан с помощью `--plugin-dir` в той же команде, как в `claude --plugin-dir ./formatter plugin details formatter`.

Этот пример читает установленный плагин с именем `formatter`, который имеет два skills, команду, agent, hook и MCP server:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Каждая часть вывода отвечает на другой вопрос:

* **Component inventory**: что Claude Code нашел в плагине. Commands считаются вместе с skills, поэтому `format-all` появляется под `Skills`. Hooks и MCP servers не получают оценку стоимости и не имеют строки per-component; чтобы увидеть, что добавляют MCP tools плагина, запустите `/context` в сеансе с включенным плагином и прочитайте категорию `MCP tools`.
* **Always-on**: токены, которые имена и описания skills, agents и commands плагина добавляют к каждому сеансу, в котором включен плагин, независимо от того, запускается ли что-либо. Это число, которое несет каждый пользователь, и то, которое нужно уменьшить.
* **Per-component**: каждая строка разбивает один skill, agent или command на его always-on долю и его on-invoke стоимость, которая является телом, загружаемым только при запуске этого компонента. Используйте столбец always-on, чтобы найти, какой компонент вносит наибольший вклад.

<h3 id="lower-the-always-on-figure">
  Снижение значения always-on
</h3>

Если вы поддерживаете плагин, эти изменения уменьшают то, что он добавляет к каждому сеансу. Если вы только его используете, ваши варианты — отключить или удалить его; см. [Управление установленными плагинами](/docs/ru/plugins/install#manage-installed-plugins).

Значение always-on считает имя каждого компонента плюс его `description` и `when_to_use` frontmatter. Чтобы его снизить:

* Сократите описания skill и agent.
* Разделите большой плагин, чтобы пользователи устанавливали только нужные им компоненты.

Описание skill также то, что Claude сопоставляет с запросом, поэтому более короткое может остановить срабатывание skill. После сокращения описаний проверьте срабатывание с помощью [`tool_used: Skill` grader](/docs/ru/plugin-evals#create-your-first-eval-suite) в вашем eval suite.

Для того, что вносит каждый тип компонента, см. [plugin components](/docs/ru/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Стоимость, показанная пользователям перед установкой
</h3>

Плагины в официальном marketplace показывают свою стоимость пользователям перед установкой. В `/plugin`, когда пользователь просматривает список плагинов marketplace и выбирает плагин, панель деталей показывает раздел **Context cost** с линией `Every turn:` и линией `When invoked:`. Когда значение always-on составляет 2000 токенов или более, линия `Every turn:` отображается выделенной.

Плагин в вашем собственном marketplace не имеет раздела **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Проверка использования плагина
</h2>

Claude Code не сообщает об использовании плагина его автору. Использование записывается на машине каждого человека, который установил плагин, поэтому то, что вы можете узнать, зависит от вашего отношения к этим людям:

* **Вы администрируете Claude Code для их организации**: события OpenTelemetry и Analytics API считают установки и активации skills на каждой машине. См. [Измерение по всему парку](#measure-across-a-fleet).
* **Это товарищи по команде, которых вы можете спросить**: собственный Claude Code каждого пользователя показывает им, используют ли они все еще плагин, в четырех местах: панель [`/plugin`](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor) и [`/usage`](#usage-share-in-/usage). Все четыре — это команды, которые пользователь запускает в приглашении Claude Code в сеансе на своей собственной машине.
* **Ни то, ни другое**: у вас нет сигнала об использовании Claude Code для этого плагина.

<h3 id="not-used-recently-in-/plugin">
  Не использовалось недавно в `/plugin`
</h3>

На вкладке **Installed** в `/plugin` плагин, который пользователь установил из marketplace, перемещается под заголовок **Not used recently** после того, как он не использовался в течение как минимум 14 дней и 10 сеансов. Детали плагина также показывают линию `Last used:`. Для того, что пользователи делают с этим заголовком и линией, см. [Поиск плагинов, которые вы больше не используете](/docs/ru/plugins/install#find-plugins-you-no-longer-use).

Заголовок **Not used recently** никогда не появляется для:

* Плагинов, загруженных с помощью `--plugin-dir` или из каталога skills
* Плагинов, включенных через управляемые параметры или смонтированных из [seed directory](/docs/ru/plugins/org#seed-containers-and-ci)
* Плагинов, которые включают тему, стиль вывода, монитор или workflow, потому что они используются без отслеживаемого вызова

[Language server](/docs/ru/plugins/components#lsp-servers) плагина считается используемым, когда он доставляет диагностику или отвечает на запрос навигации по коду, поэтому плагин LSP, сервер которого активен в ваших сеансах, не указывается как неиспользуемый.

Когда организация пользователя устанавливает [`strictKnownMarketplaces`](/docs/ru/plugins/org#restrict-what-users-can-install), ни заголовок, ни линия `Last used:` не отображаются.

<h3 id="find-skills-that-never-run">
  Поиск skills, которые никогда не запускаются
</h3>

Запустите `/skill-doctor`, чтобы увидеть, что стоит каждый из ваших skills и как часто он используется. Он отмечает skills, которые находятся в списке skills Claude, но никогда не были вызваны, включая skills из плагинов.

В интерактивном сеансе отчет открывается на вкладке **Stats** менеджера `/plugin`. См. [Поиск неиспользуемых skills](/docs/ru/skills#find-unused-skills) для того, что охватывает отчет и где он доступен.

<h3 id="unused-plugins-in-/doctor">
  Неиспользуемые плагины в `/doctor`
</h3>

Проверка `/doctor` перечисляет каждый установленный пользователем skill, MCP server и плагин и рекомендует отключить те, которые не использовались. См. [`/doctor` в справочнике команд](/docs/ru/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Доля использования в `/usage`
</h3>

На плане Pro, Max, Team или Enterprise разбивка `/usage` приписывает недавнее использование skills, subagents, плагинам и MCP servers как доле от общего количества. См. [Использование команды `/usage`](/docs/ru/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Измерение по всему парку
</h2>

Если вы администрируете Claude Code для организации, вы можете измерить стоимость и использование плагина на каждой машине из одного из этих источников:

* **События OpenTelemetry**: Claude Code экспортирует их в ваш собственный backend после [настройки экспортера](/docs/ru/monitoring-usage). См. [События OpenTelemetry для установок и использования плагинов](#pick-the-opentelemetry-event-for-each-question).
* **Analytics API**: обслуживается из записей Anthropic, без необходимости в экспортере. См. [Запрос Analytics API](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  События OpenTelemetry для установок и использования плагинов
</h3>

Эти события OpenTelemetry и атрибуты отвечают на каждый вопрос плагина из вашего backend:

| Вопрос                                              | Событие или атрибут OpenTelemetry                                                                                                                             |
| :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Какие плагины устанавливаются и откуда              | [`claude_code.plugin_installed`](/docs/ru/monitoring-usage#plugin-installed-event), один за установку                                                              |
| Какие плагины активны в скольких сеансах            | [`claude_code.plugin_loaded`](/docs/ru/monitoring-usage#plugin-loaded-event), один за включенный плагин в начале сеанса                                            |
| Какие skills активируются и какой плагин их владеет | [`claude_code.skill_activated`](/docs/ru/monitoring-usage#skill-activated-event), с `plugin.name` и `marketplace.name` для skills плагина                          |
| Что сообщают hooks плагина                          | [`claude_code.hook_plugin_metrics`](/docs/ru/monitoring-usage#hook-plugin-metrics-event), выпускается только для hooks в плагинах официального marketplace         |
| Что стоит плагин в расходах API                     | `plugin.name` и `marketplace.name` на [cost counter](/docs/ru/monitoring-usage#cost-counter), установленные, когда активный skill или subagent принадлежит плагину |

<h3 id="redacted-plugin-names-in-your-backend">
  Скрытые имена плагинов в вашем backend
</h3>

Плагины из официального marketplace сообщают свое имя плагина и имя marketplace в ваш backend дословно. Имя каждого другого плагина скрывается или опускается по умолчанию, включая плагин из собственного marketplace вашей организации. [Уровень доверия](/docs/ru/plugins/security#find-plugins-in-telemetry) плагина решает, какой.

Чтобы получить реальные имена для некоторых событий, установите переменную окружения [`OTEL_LOG_TOOL_DETAILS`](/docs/ru/monitoring-usage#common-configuration-variables) на `1` на машинах, которые экспортируют телеметрию, например в блоке `env` тех же [управляемых параметров](/docs/ru/monitoring-usage#administrator-configuration), которые настраивают экспортер:

| Событие                               | По умолчанию                                                                                        | С `OTEL_LOG_TOOL_DETAILS=1`                                    |
| :------------------------------------ | :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- |
| `plugin_loaded`                       | `plugin.name` и `marketplace.name` — это буквальная строка `third-party`                            | Реальные имена                                                 |
| `plugin_installed`, `skill_activated` | `plugin.name` и `marketplace.name` опущены; на `skill_activated`, `skill.name` — это `custom_skill` | Реальные имена                                                 |
| Cost counter                          | `plugin.name` — это `third-party`; `marketplace.name` отсутствует                                   | Реальный `plugin.name`; `marketplace.name` все еще отсутствует |

На `plugin_loaded`, `plugin_id_hash` все еще идентифицирует каждый плагин по умолчанию, поэтому вы можете считать отдельные плагины третьих сторон.

<h3 id="query-the-analytics-api">
  Запрос Analytics API
</h3>

На плане Enterprise Analytics API отвечает на вопрос "какие плагины устанавливает и вызывает моя организация" из записей Anthropic, без необходимости в экспортере. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) возвращает количество установок и вызовов за плагин, за день на Claude Code и Cowork, которые вы можете группировать по пользователю, группе RBAC или продукту.

Активность плагина, которая достигает Anthropic без имени плагина, отображается в одной агрегированной строке `third-party`. [Поиск плагинов в телеметрии](/docs/ru/plugins/security#find-plugins-in-telemetry) говорит, какие плагины Claude Code сообщает по имени.

Аутентифицируйте запрос с помощью ключа API, который имеет область `read:analytics`, которую Primary Owner создает, как описано в разделе [Доступ к данным программно](/docs/ru/analytics#access-data-programmatically).

См. [справочник конечной точки](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) для параметров и полей ответа.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Тестирование плагинов с помощью evals](/docs/ru/plugin-evals): измерьте, насколько надежно плагин направляет Claude, а не только то, что он стоит
* [Снижение значения always-on](#lower-the-always-on-figure): что изменить в плагине, чтобы уменьшить его стоимость за ход
* [Безопасность и доверие плагинов](/docs/ru/plugins/security#find-plugins-in-telemetry): какие поля телеметрии содержат имена плагинов и когда они скрыты
* [Мониторинг использования](/docs/ru/monitoring-usage): полный справочник событий OpenTelemetry
