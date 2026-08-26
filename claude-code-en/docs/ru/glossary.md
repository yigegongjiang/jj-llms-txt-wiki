> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Глоссарий

> Определения терминологии Claude Code. Узнайте, что означают agentic loop, compaction, CLAUDE.md, hooks, subagents, MCP и другие основные концепции.

Этот глоссарий определяет терминологию Claude Code. Каждая запись ссылается на страницу, где концепция рассматривается подробно. Для концепций уровня модели, таких как tokens, temperature и RAG, см. [глоссарий платформы](https://platform.claude.com/docs/ru/about-claude/glossary).

<h2 id="a">
  A
</h2>

<h3 id="agent-teams">
  Agent teams
</h3>

Несколько независимых сеансов Claude Code, координируемых лидером команды, с общим списком задач и обменом сообщениями между участниками. В отличие от [subagents](#subagent), которые работают в одном сеансе и отчитываются только перед родительским агентом, члены команды имеют собственное окно контекста, и вы можете взаимодействовать с любым из них напрямую. Agent teams являются экспериментальной функцией и должны быть включены путём установки `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`.

Подробнее: [Run agent teams](/docs/ru/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

Рабочий процесс, при котором ИИ может автономно читать файлы, выполнять команды и вносить изменения, пока вы наблюдаете, перенаправляете или отсутствуете, в отличие от чат-ассистентов на основе текста, которые только отвечают текстом, который вы должны применить сами. Claude Code является agentic, потому что он имеет [tools](#tool), которые позволяют ему действовать, а не только давать советы.

Подробнее: [How Claude Code works](/docs/ru/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

Инструменты, управление контекстом и среда выполнения, которые превращают языковую модель в способного агента кодирования. Claude Code — это harness; Claude — это модель внутри него. Harness предоставляет доступ к файлам, выполнение shell-команд, управление разрешениями, загрузку памяти и цикл, который связывает действия вместе.

Подробнее: [How Claude Code works](/docs/ru/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

Цикл, через который проходит Claude для каждой задачи: собрать контекст, предпринять действие, проверить результаты и повторять до завершения. Каждое использование tool возвращает информацию, которая информирует следующий шаг. Вы можете прервать цикл в любой момент для перенаправления. Большинство точек расширения, включая [hooks](#hook), [skills](#skill) и [MCP](#mcp-model-context-protocol), подключаются к определённым фазам этого цикла.

Подробнее: [How Claude Code works](/docs/ru/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Живая интерактивная веб-страница, которую Claude Code публикует из вашего сеанса на приватный URL на claude.ai, чтобы вы могли видеть результат визуально или поделиться им вместо чтения текста терминала. Страница обновляется на месте, когда сеанс переиздаёт её. Артефакты, которые вы создаёте из Claude Code, появляются в той же галерее, что и артефакты, созданные в разговорах claude.ai. Совместное использование зависит от вашего плана: на Pro и Max — общедоступная ссылка, которую может открыть кто угодно; на Team и Enterprise — совместное использование в вашей организации, плюс общедоступные ссылки после того, как владелец их включит.

Подробнее: [Share session output as artifacts](/docs/ru/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Заметки, которые Claude пишет для себя на основе ваших исправлений и предпочтений, хранящиеся в репозитории git в `~/.claude/projects/`. Все worktrees одного репозитория используют один каталог auto memory. Первые 200 строк или 25 КБ индекса `MEMORY.md` загружаются в начале каждого сеанса. Auto memory — это написанный Claude аналог [CLAUDE.md](#claude-md), который вы пишете.

Подробнее: [Auto memory](/docs/ru/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

[permission mode](#permission-mode), где отдельная модель классификатора проверяет действия в фоновом режиме, поэтому большинство выполняются без запросов на одобрение; явные правила запроса всё ещё выводят запросы. Классификатор блокирует расширение области, ненадёжную инфраструктуру и [prompt injection](#prompt-injection). Он никогда не видит результаты tool, поэтому внедрённые инструкции не могут повлиять на его решения.

Подробнее: [Eliminate prompts with auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

Флаг запуска, `--bare`, который пропускает автоматическое обнаружение hooks, skills, plugins, MCP servers, auto memory и CLAUDE.md. Действуют только явно переданные флаги. Рекомендуется для CI и скриптовых вызовов, где вам нужно одинаковое поведение на разных машинах независимо от локальной конфигурации.

Подробнее: [Start faster with bare mode](/docs/ru/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Основанные на prompt playbooks, включённые в Claude Code, такие как `/batch`, `/code-review`, `/debug` и `/loop`. В отличие от встроенных команд, которые выполняют фиксированную логику, bundled skills дают Claude подробный prompt и позволяют ему организовать работу, поэтому они могут порождать агентов, читать файлы и адаптироваться к вашей кодовой базе.

Подробнее: [Bundled skills](/docs/ru/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

[MCP server](#mcp-model-context-protocol), который отправляет события в ваш работающий сеанс, чтобы Claude мог реагировать на события, происходящие, пока вы отсутствуете в терминале. Channels могут быть двусторонними: Claude читает входящее событие и отвечает через тот же channel. Telegram, Discord и iMessage включены в исследовательский предпросмотр.

Подробнее: [Channels](/docs/ru/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

Точка восстановления, созданная при каждом отправляемом вами prompt. Claude Code создаёт снимки файлов перед каждым редактированием, чтобы checkpoint мог их восстановить. Нажмите `Esc` дважды или запустите `/rewind`, чтобы восстановить код, разговор или оба на более ранний момент, или чтобы резюмировать часть разговора из выбранного сообщения. Checkpoints сохраняются вместе с разговором, поэтому возобновленный сеанс всё ещё может использовать `/rewind` для их восстановления. Они отделены от git и не отслеживают изменения, сделанные через Bash tool.

Подробнее: [Checkpointing](/docs/ru/checkpointing)

<h3 id="claude-directory">
  `.claude` directory
</h3>

Каталог, из которого Claude Code читает конфигурацию, ограниченную проектом: settings, hooks, skills, subagents, rules и auto memory. Проект имеет `.claude/` в своём корне; ваши пользовательские значения по умолчанию находятся в `~/.claude/`.

Подробнее: [The `.claude` directory](/docs/ru/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

Файл markdown с постоянными инструкциями, которые вы пишете для Claude, загружаемый в начале каждого сеанса как пользовательское сообщение после системного prompt. Поместите сюда соглашения проекта, заметки об архитектуре и правила "всегда делай X". CLAUDE.md в корне проекта сохраняется при [compaction](#compaction) и перечитывается свежим с диска после этого.

Вы можете разместить CLAUDE.md в области проекта в `./CLAUDE.md` или `./.claude/CLAUDE.md`, в области пользователя в `~/.claude/CLAUDE.md` или как [managed policy](#managed-settings) для вашей организации. Все обнаруженные файлы объединяются в контекст, а не переопределяют друг друга, упорядочены от самой широкой области к наиболее специфичной.

Подробнее: [CLAUDE.md files](/docs/ru/memory#claude-md-files)

<h3 id="command">
  Command
</h3>

Переиспользуемая инструкция, которую вы вызываете, введя `/name` в prompt. Встроенные команды, такие как `/clear`, `/model` и `/compact`, управляют сеансом. Вы можете определить свои собственные команды как файлы в `.claude/commands/` или установить их из [plugin](#plugin). [Skills](#skill) — это рекомендуемый способ упаковки многошаговых команд.

Подробнее: [Commands](/docs/ru/commands) · [Skills](/docs/ru/skills)

<h3 id="compaction">
  Compaction
</h3>

Автоматическое резюмирование вашего разговора, когда [context window](#context-window) приближается к своему пределу. Сначала очищаются старые выходы tool, затем разговор резюмируется. CLAUDE.md в корне проекта и auto memory сохраняются при compaction и перезагружаются с диска; инструкции, данные только в разговоре, могут быть потеряны. Запустите `/compact` для ручного запуска, опционально с фокусом, например `/compact focus on the API changes`.

Подробнее: [What survives compaction](/docs/ru/context-window#what-survives-compaction) · [When context fills up](/docs/ru/how-claude-code-works#when-context-fills-up)

<h3 id="context-window">
  Context window
</h3>

Рабочая память для сеанса, содержащая историю разговора, содержимое файлов, выходы команд, CLAUDE.md, auto memory, загруженные skills и системные инструкции. По мере работы контекст заполняется до [compaction](#compaction), который его резюмирует. Запустите `/context`, чтобы увидеть, что использует пространство. Для базовой концепции модели см. [глоссарий платформы](https://platform.claude.com/docs/ru/about-claude/glossary#context-window).

Подробнее: [Explore the context window](/docs/ru/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Инициируемый телефоном маршрутизатор задач, который порождает сеанс Claude Code в приложении Desktop, когда вы отправляете задачу кодирования из мобильного приложения Claude. Ваш prompt маршрутизируется к правильному tool автоматически. Доступно на планах Pro и Max.

Подробнее: [Sessions from Dispatch](/docs/ru/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

Параметр, который управляет тем, сколько адаптивного бюджета thinking Claude использует на каждый ход. Более высокий effort означает больше thinking tokens и более глубокое рассуждение; более низкий effort быстрее и дешевле. Effort поддерживается на Fable 5, на Opus 4.6 и более поздних версиях, а также на Sonnet 4.6 и более поздних версиях.

Подробнее: [Adjust effort level](/docs/ru/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

Видимое пошаговое рассуждение, которое модель выполняет перед ответом. Вы можете отрегулировать его с помощью [effort level](#effort-level), или ограничить thinking tokens с помощью `MAX_THINKING_TOKENS` на моделях с фиксированным бюджетом thinking. Thinking появляется серым курсивным текстом в терминале.

Подробнее: [Use extended thinking](/docs/ru/model-config#extended-thinking)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Определённый пользователем обработчик, который выполняется автоматически в определённой точке жизненного цикла Claude Code, например перед запуском tool, после редактирования файла или при запуске сеанса. Обработчики могут быть shell-командой, HTTP endpoint, MCP tool, LLM prompt или subagent. Hooks являются детерминированными: они срабатывают в фиксированных точках жизненного цикла, а не по усмотрению модели.

Конфигурация hook имеет три уровня:

* **Hook event**: точка жизненного цикла
* **Matcher**: фильтрует, какие события его срабатывают
* **Hook handler**: что запускается

Подробнее: [Get started with hooks](/docs/ru/hooks-guide) · [Hooks reference](/docs/ru/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

Параметры, применяемые организацией IT или DevOps, доставляемые с серверов Anthropic через консоль администратора или развёрнутые на устройствах по пути уровня ОС вне `~/.claude`. Пользователи и параметры проекта не могут переопределять управляемые параметры. Доставка, управляемая сервером, применяется на [подходящих конфигурациях](/docs/ru/server-managed-settings#platform-availability); см. [Соображения безопасности](/docs/ru/server-managed-settings#security-considerations). Используйте это для политик безопасности, требований соответствия или стандартизированного инструментария на всём парке.

Подробнее: [Server-managed settings](/docs/ru/server-managed-settings) · [Settings files](/docs/ru/settings#settings-files)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Открытый стандарт для подключения инструментов ИИ к внешним источникам данных и сервисам. MCP servers дают Claude новые tools для Slack, Jira, баз данных, браузеров и сотен других интеграций. Вы подключаете servers через `/mcp` или добавляя их в `.mcp.json`. Для самого протокола см. [глоссарий платформы](https://platform.claude.com/docs/ru/about-claude/glossary#mcp-model-context-protocol).

Подробнее: [Model Context Protocol](/docs/ru/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Механизм сохранения контекста, который откладывает схемы MCP tool до необходимости. Только имена tool загружаются при запуске; Claude получает полную схему по требованию, когда решает использовать определённый tool. Это предотвращает потребление большого контекста неиспользуемыми MCP servers.

Подробнее: [Scale with MCP Tool Search](/docs/ru/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

Режим, который выполняет один prompt и выходит без интерактивного сеанса, вызываемый с `-p` или `--print`. Используется для CI, скриптов и piping. Запуск по-прежнему сохраняется как возобновляемый сеанс, если вы не передадите `--no-session-persistence`. [Agent SDK](/docs/ru/agent-sdk/overview) — это эквивалент Python и TypeScript. Ранее называлось headless mode.

Подробнее: [Run Claude Code programmatically](/docs/ru/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

Конфигурация, которая изменяет системный prompt Claude для изменения поведения ответа, тона или формата. Output styles отключают части системного prompt, специфичные для разработки программного обеспечения, в отличие от [CLAUDE.md](#claude-md), который доставляется как пользовательское сообщение, следующее за системным prompt. Встроенные стили включают Default, Proactive, Explanatory и Learning.

Подробнее: [Output styles](/docs/ru/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission mode
</h3>

Базовое поведение одобрения для сеанса. Переключайтесь с `Shift+Tab` в CLI или используйте селектор режима в VS Code, Desktop и claude.ai. Доступные режимы: `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` и `bypassPermissions`.

Режим `default` обозначен как Manual в CLI и в расширениях VS Code и JetBrains, и Claude Code принимает `manual` как псевдоним для этого значения.

Подробнее: [Выберите режим разрешений](/docs/ru/permission-modes)

<h3 id="permission-rule">
  Permission rule
</h3>

Запись settings, которая разрешает, спрашивает или отрицает вызов tool на основе имени tool и шаблона аргумента. Правила оцениваются deny→ask→allow, первое совпадение побеждает. Permission rules — это детальные элементы управления, наложенные на более широкий [permission mode](#permission-mode).

Подробнее: [Настройте разрешения](/docs/ru/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

[permission mode](#permission-mode), где Claude исследует и предлагает изменения без редактирования ваших исходных файлов. Он может читать, искать и выполнять команды исследования, затем представляет план для одобрения перед тем, как что-либо трогать. Войдите в plan mode с `/plan` или нажав `Shift+Tab`.

Подробнее: [Анализируйте перед редактированием с помощью plan mode](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Пакет skills, hooks, subagents и MCP servers, упакованный как единица установки. Plugin skills имеют пространство имён как `plugin-name:skill-name`, поэтому несколько plugins сосуществуют. Распределяйте plugins по командам через [marketplace](/docs/ru/plugin-marketplaces).

Подробнее: [Plugins](/docs/ru/plugins)

<h3 id="project-trust">
  Project trust
</h3>

Диалог принятия каталога перед загрузкой Claude Code его конфигурации. Принятие сохраняется для каждого каталога проекта, за исключением вашего домашнего каталога, где доверие сохраняется только для текущего сеанса и приглашение появляется снова при каждом запуске. Trust gates автоматическую установку marketplace plugins и выполнение определённых проектом hooks. Доверие к каталогу означает, что его `.claude/settings.json`, `.mcp.json` и другие файлы конфигурации вступают в силу.

Подробнее: [Каталог `.claude`](/docs/ru/claude-directory)

<h3 id="prompt-injection">
  Prompt injection
</h3>

Враждебные инструкции, встроенные в файл, веб-страницу или результат tool, которые пытаются перенаправить Claude к действиям, которые вы никогда не просили. Защита Claude Code включает систему разрешений, обнаружение инъекций команд и проверку доверия. [Auto mode](#auto-mode) добавляет зонд на стороне сервера, который сканирует результаты tool на предмет подозрительного содержимого, и классификатор, который никогда не видит результаты tool, поэтому внедрённый текст не может повлиять на его решения об одобрении.

Подробнее: [Защититесь от prompt injection](/docs/ru/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Способ продолжить локальный сеанс Claude Code с вашего телефона или браузера через claude.ai. Ваш код остаётся на вашей машине; только UI является удалённым. Отличается от Claude Code в веб-версии, который работает в облачной песочнице.

Подробнее: [Remote Control](/docs/ru/remote-control)

<h3 id="rules">
  Rules
</h3>

Модульные файлы инструкций в `.claude/rules/`, которые загружаются вместе с CLAUDE.md. Правило может быть ограничено по пути с помощью YAML frontmatter `paths:`, поэтому оно загружается только, когда Claude читает соответствующий файл, сохраняя контекст стройным до тех пор, пока он не станет релевантным.

Подробнее: [Organize rules with `.claude/rules/`](/docs/ru/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Изоляция файловой системы и сети уровня ОС для Bash tool. Команды выполняются внутри границы, которую вы определяете заранее, поэтому Claude может свободно работать внутри неё без запросов на одобрение для каждой команды. Sandboxing — это отдельный слой от [permission rules](#permission-rule).

Подробнее: [Sandboxing](/docs/ru/sandboxing)

<h3 id="session">
  Session
</h3>

Разговор, привязанный к вашему текущему каталогу, с собственным независимым [context window](#context-window). Сеансы можно возобновить с помощью `claude -c`, разветвить с помощью `--fork-session` для сохранения истории под новым ID сеанса или запустить параллельно на разных терминалах. Запуск `/clear` начинает новый сеанс; предыдущий остаётся сохранённым и доступен через `/resume`. Стенограмма каждого сеанса хранится в `~/.claude/projects/`.

Подробнее: [Work with sessions](/docs/ru/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

Иерархия, из которой Claude Code читает конфигурацию, в порядке приоритета от наивысшего к наинизшему: [managed policy](#managed-settings), аргументы командной строки, локальные settings в `.claude/settings.local.json`, settings проекта в `.claude/settings.json`, затем пользовательские settings в `~/.claude/settings.json`. Массивы объединяются по слоям; скаляры на более высоком слое переопределяют более низкие.

Подробнее: [Settings files](/docs/ru/settings#settings-files)

<h3 id="skill">
  Skill
</h3>

Файл `SKILL.md`, содержащий инструкции, знания или рабочий процесс, который Claude добавляет в свой набор инструментов. Claude загружает skill автоматически, когда это релевантно, или вы вызываете его напрямую с помощью `/skill-name`. Skills следуют открытому стандарту Agent Skills; Claude Code расширяет его с помощью управления вызовом и выполнения subagent.

Skills — это рекомендуемый преемник пользовательских команд. Файл в `.claude/commands/deploy.md` и один в `.claude/skills/deploy/SKILL.md` оба создают `/deploy` и работают одинаково; существующие файлы команд продолжают работать.

Подробнее: [Extend Claude with skills](/docs/ru/skills)

<h3 id="subagent">
  Subagent
</h3>

Специализированный ИИ-ассистент, который работает в собственном окне контекста с пользовательским системным prompt, определённым доступом к tool и независимыми разрешениями. Он работает над делегированной задачей и возвращает резюме в основной разговор. Используйте subagents, чтобы держать большие исследования вне вашего основного контекста или запускать параллельные исследования. Отличается от [agent teams](#agent-teams), где каждый агент — это полный независимый сеанс, с которым вы можете разговаривать напрямую.

Встроенные subagents включают Explore, Plan и общего назначения.

Подробнее: [Create custom subagents](/docs/ru/sub-agents)

<h3 id="surface">
  Surface
</h3>

Любое место, где вы получаете доступ к Claude Code: CLI, VS Code, JetBrains, Desktop или claude.ai. Все surfaces используют один и тот же engine, поэтому ваш CLAUDE.md, settings и skills работают одинаково на всех них. Slack и расширение Chrome — это интеграции, которые подключаются к surface, а не сами surfaces.

Подробнее: [Platforms and integrations](/docs/ru/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Команда, `/teleport`, которая вытягивает облачный сеанс Claude Code в ваш локальный терминал. Claude получает ветку, загружает историю разговора и возобновляет с последнего состояния веб-сеанса. Обратное направление — `--cloud`, которое отправляет локальную задачу для запуска в веб-версии.

Подробнее: [From web to terminal](/docs/ru/claude-code-on-the-web#from-web-to-terminal)

<h3 id="tool">
  Tool
</h3>

Действие, которое может предпринять Claude: прочитать файл, отредактировать код, выполнить shell-команду, поискать в веб-сети, порождать subagent. Tools — это то, что делает Claude Code agentic. Без них Claude может только отвечать текстом. Каждое использование tool возвращает результат, который информирует следующее решение Claude в [agentic loop](#agentic-loop).

Подробнее: [Tools available to Claude](/docs/ru/tools-reference)

<h3 id="turn">
  Turn
</h3>

Один полный ответ от Claude в рамках [session](#session). Turn начинается, когда вы отправляете сообщение, и заканчивается, когда Claude завершает ответ, с любым количеством вызовов [tool](#tool) между ними. [Stop hooks](#hook) срабатывают в конце каждого turn. Session состоит из множества turn, и [agentic loop](#agentic-loop) описывает, что происходит внутри одного.

Подробнее: [How Claude Code works](/docs/ru/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

Как сеанс узнает, что работа действительно завершена, а не просто правдоподобна. Вы даёте Claude проверку, которую он может запустить, такую как набор тестов, сборка или сравнение скриншотов, и Claude повторяет до тех пор, пока проверка не пройдёт, вместо того чтобы остановиться после одной попытки. Verification loop — это предварительное условие для [`/goal`](/docs/ru/goal), автоматических запусков и [dynamic workflows](/docs/ru/workflows): без него единственное, что решает, что агент закончил, — это сам агент.

Подробнее: [Give Claude a way to verify its work](/docs/ru/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

Режим изоляции, который запускает Claude в отдельном git worktree в `.claude/worktrees/`, включаемый флагом `-w` или `isolation: worktree` в конфигурации subagent. Изменения остаются на отдельной ветке в отдельном каталоге, поэтому параллельные агенты не перезаписывают файлы друг друга.

Подробнее: [Запуск параллельных сеансов с git worktrees](/docs/ru/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  Deprecated and renamed terms
</h2>

Эти термины появляются в старых документах, постах блога и содержимом сообщества. Используйте текущее имя при поиске на этом сайте.

| Old term        | Now called                                    | Notes                                |
| --------------- | --------------------------------------------- | ------------------------------------ |
| Headless mode   | [Non-interactive mode](#non-interactive-mode) | Same `-p` flag, same behavior        |
| Custom commands | [Skills](#skill)                              | `.claude/commands/` files still work |
| Slash commands  | Commands                                      | "Slash" dropped from product copy    |
