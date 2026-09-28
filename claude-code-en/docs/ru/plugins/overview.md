> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Обзор plugins

> Узнайте, что такое Claude Code plugin, когда вам нужен plugin вместо отдельного skill или MCP server, и какую страницу прочитать для установки или создания plugin.

Claude Code plugin — это директория skills, agents, hooks, MCP servers или других компонентов, которые Claude Code устанавливает и загружает как одну единицу. Большинство plugins поступают из marketplace, который представляет собой каталог, в котором перечислены plugins и указано, где получить каждый из них. Вы также можете загрузить plugin из папки, которую вам дал кто-то другой, или [создать свой собственный](/docs/ru/plugins/create).

<Note>
  Если вы используете claude.ai chat или Cowork, а не Claude Code, см. [Plugins на claude.ai и в Cowork](https://claude.com/docs/plugins/overview).
</Note>

Чтобы попробовать plugin прямо сейчас, запустите `/plugin` в сеансе терминала Claude Code и установите один из вкладки **Discover**, на которой перечислены plugins из официального marketplace Anthropic и любого marketplace, который вы добавили. Оттуда:

* [Установка и управление plugins](/docs/ru/plugins/install): полные шаги установки, области действия и другие поверхности
* [Создание plugin](/docs/ru/plugins/create): создайте свой собственный
* [Решите, нужен ли вам plugin](#decide-whether-you-need-a-plugin): является ли plugin правильным инструментом для того, что вы хотите

<h2 id="understand-what-a-plugin-is">
  Поймите, что такое plugin
</h2>

Plugin — это директория компонентов, обычно с манифестом. Манифест, JSON-файл в `.claude-plugin/plugin.json`, дает plugin его имя и может добавить версию, описание и другие [метаданные](/docs/ru/plugins/manifest-reference). Компоненты — это то, что plugin добавляет в Claude Code, например:

* [**Skills**](/docs/ru/plugins/components#skills): инструкции `SKILL.md`, которые Claude загружает при необходимости, и которые вы также можете запустить как команду
* [**Agents**](/docs/ru/plugins/components#agents): определения подагентов, которым Claude может делегировать
* [**Hooks**](/docs/ru/plugins/components#hooks): команды, которые Claude Code запускает в точках своего жизненного цикла, например после каждого редактирования
* [**MCP servers**](/docs/ru/plugins/components#mcp-servers): серверы инструментов, к которым Claude Code подключается, пока plugin включен

На этой диаграмме показан plugin с именем `my-plugin`, который содержит по одному из каждого из этих компонентов, и что вы получаете от каждого файла после загрузки plugin.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Диаграмма в двух столбцах, соединенных пятью прямыми стрелками. Слева — директория plugin с именем my-plugin, содержащая манифест в .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json и другие компоненты. Справа — то, что каждый файл дает вам в вашем сеансе: манифест устанавливает имя plugin, my-plugin; skill запускается как /my-plugin:review; файл agent — это подагент, которому Claude может делегировать; файл hooks содержит hooks, которые запускаются на событиях жизненного цикла; и .mcp.json добавляет MCP server, который дает Claude инструменты." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Диаграмма в двух столбцах, соединенных пятью прямыми стрелками. Слева — директория plugin с именем my-plugin, содержащая манифест в .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json и другие компоненты. Справа — то, что каждый файл дает вам в вашем сеансе: манифест устанавливает имя plugin, my-plugin; skill запускается как /my-plugin:review; файл agent — это подагент, которому Claude может делегировать; файл hooks содержит hooks, которые запускаются на событиях жизненного цикла; и .mcp.json добавляет MCP server, который дает Claude инструменты." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Для каждого типа компонента, который может содержать plugin, с примером каждого, см. [Plugin components](/docs/ru/plugins/components). Чтобы увидеть, где находится каждая часть в директории plugin, используйте [plugin explorer](/docs/ru/plugins/components#explore-the-plugin-directory) на этой странице.

<h3 id="decide-whether-you-need-a-plugin">
  Решите, нужен ли вам plugin
</h3>

Skills, подагенты, hooks и MCP servers работают самостоятельно, без plugin. Skill, который вы сохраняете в `~/.claude/skills/`, например, доступен в каждом проекте на вашем компьютере. Чтобы установить один самостоятельно, см. [Skills](/docs/ru/skills), [Subagents](/docs/ru/sub-agents), [Hooks](/docs/ru/hooks-guide) или [MCP](/docs/ru/mcp).

Используйте plugin, когда вы хотите упаковать несколько skills, подагентов, hooks или MCP servers как одну единицу. Установите один, чтобы получить настройку, которую создал кто-то другой, с одной командой и обновлениями из его marketplace. Создайте один, чтобы дать вашу собственную настройку товарищам по команде, установить его во многих проектах или опубликовать версионные релизы.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  Что включенный plugin добавляет в ваши сеансы
</h3>

Включенный plugin является частью каждого сеанса, а не только сеансов, в которых вы его используете. Это имеет несколько последствий, о которых стоит знать перед установкой:

* **Контекст и использование**: для каждого skill, agent и команды, которые [Claude может вызвать самостоятельно](/docs/ru/skills#control-who-invokes-a-skill), имя и описание находятся в контексте Claude на каждом ходу, чтобы Claude знал об их существовании. Эти токены учитываются в вашем использовании и оставляют меньше места в [контекстном окне](/docs/ru/context-window) даже в сеансах, где ничего из plugin не запускается. Полный текст skill или agent загружается только при его использовании. То, что MCP servers plugin добавляют за ход, следует [MCP tool search](/docs/ru/mcp#scale-with-mcp-tool-search).
* **Процессы**: MCP servers, которые определяет plugin, работают параллельно с каждым сеансом, в котором он включен, и его hooks срабатывают при их событиях.
* **Разрешения**: то, что запускает plugin, запускается от вас. См. [Plugin security and trust](/docs/ru/plugins/security) для того, что нужно проверить в первую очередь.

Вы можете проверить отпечаток plugin на каждом этапе:

* **Перед установкой**: откройте plugin из вкладки **Marketplaces** в `/plugin`. Plugins в официальном marketplace Anthropic показывают там оценку **Context cost**.
* **После установки**: [Measure what a plugin costs](/docs/ru/plugins/measure#measure-what-a-plugin-costs) показывает, как прочитать отпечаток plugin, и группа **Not used recently** вкладки **Installed** перечисляет plugins, которые вы можете отключить.
* **Чтобы остановить его без удаления**: отключите plugin с помощью `/plugin` или в вашей оболочке `claude plugin disable`. См. [Manage installed plugins](/docs/ru/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Получайте plugins из marketplace
</h2>

Marketplace — это репозиторий или директория с файлом `.claude-plugin/marketplace.json`, который перечисляет plugins и указывает, где получить каждый из них. Это каталог, а не размещенный магазин. Вы добавляете marketplace один раз, затем устанавливаете plugins из него по имени, например `commit-commands@claude-plugins-official`.

<Note>
  Marketplace plugin — это не [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace — это веб-сайт на claude.com/marketplace, где вы просматриваете plugins, connectors, партнерские продукты и партнеров услуг. Это не marketplace, который вы добавляете с помощью `/plugin marketplace add`.
</Note>

Claude Code добавляет официальный marketplace Anthropic при первом запуске интерактивного сеанса терминала, если [управляемая политика](/docs/ru/plugins/org#allow-the-official-marketplace-and-your-own) это не блокирует. Claude Code не добавляет никакой другой marketplace самостоятельно, включая community и demo marketplaces Anthropic. Чтобы различить три marketplace Anthropic, прочитайте [Anthropic's marketplaces](/docs/ru/plugins/anthropic-marketplaces). Чтобы увидеть, что перечисляет официальный, откройте вкладку **Discover** в `/plugin` в сеансе или просмотрите [Claude Marketplace](https://claude.com/marketplace/plugins).

На этой диаграмме показан путь от marketplace к вашему сеансу. Marketplace перечисляет plugin, вы устанавливаете этот plugin, и Claude Code загружает его компоненты.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Диаграмма пути marketplace в трех ящиках, слева направо. Marketplace, каталог plugins, перечисляет plugin. Plugin — это одна директория, установленная как единица, содержащая skills, agents, hooks, MCP servers и другие компоненты. Вы устанавливаете plugin в Claude Code, который загружает его компоненты." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Диаграмма пути marketplace в трех ящиках, слева направо. Marketplace, каталог plugins, перечисляет plugin. Plugin — это одна директория, установленная как единица, содержащая skills, agents, hooks, MCP servers и другие компоненты. Вы устанавливаете plugin в Claude Code, который загружает его компоненты." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Install and manage plugins](/docs/ru/plugins/install#install-a-plugin) содержит шаги установки для каждого места, где вы запускаете Claude Code. Пока вы разрабатываете plugin, вам не нужен marketplace: загружайте его прямо из его папки с помощью `--plugin-dir`, как показано в [Develop without a marketplace](/docs/ru/plugins/create#develop-without-a-marketplace).

<h3 id="make-an-installed-plugin-available-in-your-session">
  Сделайте установленный plugin доступным в вашем сеансе
</h3>

Прежде чем plugin, который вы установили, даст вам skill, который вы можете запустить, он должен присутствовать на каждом из этих уровней:

* **Settings**: ваши settings перечисляют marketplaces, которые вы добавили, и plugins, которые включены.
* **Disk**: `~/.claude/plugins/` содержит то, что Claude Code загрузил и установил.
* **Session**: plugins загружаются при запуске или когда вы [reload plugins](/docs/ru/plugins/loading#check-which-stage-a-plugin-reached).

Прочитайте [Plugin loading reference](/docs/ru/plugins/loading) для правил на каждом уровне, включая то, какой файл settings имеет приоритет и где находятся файлы на диске.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Отличайте marketplace Anthropic от marketplace третьих сторон
</h2>

Имя marketplace помещает его в один из трех уровней. Claude Code принимает официальные и community имена только для marketplaces, полученных из репозиториев `github.com/anthropics/`:

* **Official**: marketplaces с одним из [официальных имен marketplace](/docs/ru/plugins/security#official-marketplace-names) Anthropic, включая `claude-plugins-official` и demo marketplace `claude-code-plugins`.
* **Community**: marketplaces с одним из community имен Anthropic, например `claude-community`. [Identify Anthropic's marketplaces by name](/docs/ru/plugins/security#marketplace-tiers) их перечисляет.
* **Third-party**: все остальные marketplaces. Marketplace, который публикует ваш коллега или ваша организация, является third-party.

Независимо от уровня, plugin, который вы устанавливаете, может запускать код с вашими привилегиями пользователя. Прочитайте [Plugin security and trust](/docs/ru/plugins/security) для того, как проверить plugin перед его установкой.

Через [managed settings](/docs/ru/settings#settings-files) организация может добавить в список разрешений или заблокировать marketplaces, принудительно установить plugins и отключить загрузку только для сеанса. Прочитайте [Manage plugins for your organization](/docs/ru/plugins/org) для этих элементов управления.

<h2 id="understand-install-scopes">
  Поймите области действия установки
</h2>

Когда вы устанавливаете plugin, вы выбираете область действия, и область действия определяет, для кого включен plugin:

* **User scope**: включен для вас в каждом проекте на этом компьютере
* **Project scope**: включен для всех, кто работает в этом репозитории, через committed `.claude/settings.json`. Каждый сотрудник все еще [устанавливает его на своей машине](/docs/ru/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Local scope**: включен для вас только в этом репозитории

Plugin, который вы устанавливаете в области действия пользователя в терминале, локальных сеансах настольного приложения или расширении VS Code, доступен в двух других на этом компьютере, потому что все три читают одни и те же файлы settings. См. [Choose an install scope](/docs/ru/plugins/install#choose-an-install-scope) для того, как выбрать один.

Облачный сеанс, включая сеанс в браузере на claude.ai/code, не загружает plugins в ваши локальные settings. Для шагов установки в терминале, VS Code и настольном приложении, а также для того, что загружает облачный сеанс, см. [Install a plugin](/docs/ru/plugins/install#install-a-plugin).

<Note>
  Тот же формат plugin также устанавливается на claude.ai и в Cowork, где загружается другой набор компонентов. Для этих поверхностей см. [Plugins на claude.ai и в Cowork](https://claude.com/docs/plugins/overview) на claude.com.
</Note>

<h2 id="next-steps">
  Следующие шаги
</h2>

Большинство людей начинают с установки plugin из официального marketplace Anthropic, который Claude Code добавляет при первом запуске интерактивного сеанса терминала. Запустите `/plugin` в сеансе терминала для его просмотра или следуйте [Install and manage plugins](/docs/ru/plugins/install), который также охватывает настольное приложение и VS Code. Чтобы увидеть, что находится в этом marketplace перед открытием Claude Code, просмотрите [Claude Marketplace](https://claude.com/marketplace/plugins) в Интернете.

Чтобы создать свой собственный, [Create a plugin](/docs/ru/plugins/create) начинается с пустой директории и заканчивается рабочим plugin.

После установки или создания plugin эти страницы охватывают то, что дальше:

* **Поделитесь тем, что вы создали**: [Publish and distribute a plugin](/docs/ru/plugins/publish)
* **Проверьте, работает ли он и используется ли**: [Test plugins with evals](/docs/ru/plugin-evals) и [Measure plugin cost and usage](/docs/ru/plugins/measure)
* **Запустите marketplace для вашей команды**: [Create a marketplace](/docs/ru/plugins/create-marketplace), затем [Host and maintain a marketplace](/docs/ru/plugins/host-marketplace)
* **Установите политику plugin для организации**: [Manage plugins for your organization](/docs/ru/plugins/org)
* **Исправьте проблему**: [Troubleshoot plugins](/docs/ru/plugins/troubleshooting)
