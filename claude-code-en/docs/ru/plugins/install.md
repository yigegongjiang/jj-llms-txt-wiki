> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Установка и управление плагинами

> Установите плагины Claude Code из маркетплейса на любой поверхности, которую вы используете, выберите область установки и обновляйте или удаляйте их позже.

Установка плагина добавляет его skills, agents, hooks и MCP серверы в Claude Code на вашу машину.

Эта страница предназначена для всех, кто использует плагины на своей машине или аккаунте, будь то в терминале, настольном приложении, IDE или облачной сессии: она охватывает установку, выбор области, добавление маркетплейсов и поддержание плагинов в актуальном состоянии.

<Note>
  Эти случаи рассматриваются на других страницах:

  * **Вы используете claude.ai chat или Cowork, а не Claude Code**: см. [Плагины на claude.ai и в Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code вывел ошибку**: найдите её в [Troubleshoot plugins](/docs/ru/plugins/troubleshooting)
</Note>

Начните с [Установка плагина](#install-a-plugin). Если кто-то отправил вам команду установки, чьё имя `@` не является `claude-plugins-official`, сначала [добавьте этот маркетплейс](#add-a-marketplace).

<h2 id="install-a-plugin">
  Установка плагина
</h2>

В качестве примера этот раздел устанавливает [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) из [официального маркетплейса Anthropic](/docs/ru/plugins/anthropic-marketplaces), который добавляет команды для коммитов, пушей и открытия pull requests.

Те же шаги устанавливают любой другой плагин: замените его имя и имя его маркетплейса везде, где появляются `commit-commands` и `claude-plugins-official`. Если этот плагин поступает из другого маркетплейса, сначала [добавьте маркетплейс](#add-a-marketplace).

Выберите вкладку для того места, где вы запускаете Claude Code.

<Tabs>
  <Tab title="Terminal">
    Запустите Claude Code с помощью `claude` в вашем проекте, затем:

    <Steps>
      <Step title="Откройте детали плагина с помощью команды установки">
        Запустите `/plugin install` с именем плагина и маркетплейса. В сессии эта команда не устанавливает сразу: она открывает панель `/plugin` с деталями этого плагина, чтобы вы могли его просмотреть и сначала выбрать область.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Чтобы вместо этого просмотреть список, запустите `/plugin` без имени плагина: панель откроется на вкладке **Discover**, которая перечисляет плагины из каждого добавленного вами маркетплейса, и вы можете вводить для поиска, затем нажать **Enter** на плагине, чтобы открыть его детали.
      </Step>

      <Step title="Просмотрите, что добавляет плагин">
        Панель деталей показывает описание плагина. Она также может показывать:

        * **Will install**: команды, agents, skills, hooks и MCP и LSP серверы, которые добавляет плагин.
        * **Last updated**: показывается для плагина в официальном маркетплейсе Anthropic.
        * **Context cost**: для плагина в официальном маркетплейсе Anthropic две оценки токенов. **Every turn** — это то, что плагин добавляет к каждому отправляемому вами сообщению, а **When invoked** — это то, что его skills и agents добавляют после того, как Claude их загружает. Оценки появляются, когда вы открываете плагин, назвав его маркетплейс, как это делает команда шага 1, или со вкладки **Marketplaces**. Панель деталей, которую вы достигаете из списка **Discover**, их не показывает.

        Плагины из локального или пользовательского маркетплейса могут вместо этого показывать `Components will be discovered at installation`.

        Плагин может запускать hooks и MCP серверы, поэтому прочитайте панель перед установкой. См. [Plugin security and trust](/docs/ru/plugins/security).
      </Step>

      <Step title="Выберите область">
        Выберите один из трёх вариантов установки:

        * **Install for you (user scope)**: вы получаете плагин в каждом проекте на этой машине
        * **Install for all collaborators on this repository (project scope)**: он включен для всех, кто работает в этом репозитории
        * **Install for you, in this repo only (local scope)**: вы получаете его только в этом репозитории

        [Выберите область установки](#choose-an-install-scope) говорит, в какой файл настроек записывается каждый из них и какой применяется, когда один и тот же плагин установлен в нескольких местах.

        После выбора области Claude Code устанавливает плагин вместе с любыми зависимостями, которые он объявляет, затем выводит сводку установки.
      </Step>

      <Step title="Прочитайте сводку установки">
        Последнее предложение сводки говорит вам, доступен ли плагин в этой сессии прямо сейчас:

        * **Active now**: `Plugin is now active.` Перезагрузка не требуется.
        * **Reload needed**: `Run /reload-plugins to activate.` Панель закрывается и Claude Code запускает эту перезагрузку для вас. Если перезагрузка [инвалидирует кэш подсказок](/docs/ru/prompt-caching#enabling-or-disabling-a-plugin), она предупреждает и оставляет плагин в ожидании. Запустите `/reload-plugins --force`, чтобы активировать его в любом случае, что стоит один некэшированный запрос.
        * **Load failed**: `The plugin couldn't be loaded`. Откройте вкладку **Errors** в `/plugin`, чтобы узнать причину, затем см. [After install: plugin not working](/docs/ru/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Подтвердите, что плагин работает">
        Введите `/` и ищите skills плагина под его именем в форме `/<plugin>:<skill>`. Для `commit-commands` появляется `/commit-commands:commit`. Два других места также перечисляют плагин:

        * Откройте вкладку **Installed** в `/plugin`, которая перечисляет плагин с его областью.
        * В вашей оболочке запустите `claude plugin list`, который выводит тот же список с строками `Version`, `Scope` и `Status`.

        Если `/commit-commands:commit` не появляется, см. [After install: plugin not working](/docs/ru/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    Установка из любого другого маркетплейса требует одного дополнительного шага: [добавьте маркетплейс](#add-a-marketplace). Claude Code добавляет официальный маркетплейс Anthropic для вас в первый раз, когда вы запускаете интерактивную сессию терминала, поэтому пример пропускает этот шаг. Если вы нашли плагин на [claude.com/marketplace](https://claude.com/marketplace), его кнопка **Claude Code** копирует команду установки в её [форме оболочки](#install-from-your-shell), `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Desktop app">
    В локальной или SSH сессии на вкладке **Code** настольного приложения:

    <Steps>
      <Step title="Откройте браузер плагинов">
        Нажмите кнопку **+** рядом с полем подсказки и выберите **Plugins**, затем **Add plugin**. Браузер плагинов откроется с плагинами из ваших маркетплейсов.
      </Step>

      <Step title="Выберите плагин">
        Найдите `commit-commands` и выберите его.
      </Step>

      <Step title="Выберите область">
        Выберите [область](#choose-an-install-scope): ваш пользовательский аккаунт, этот проект или только локально.
      </Step>
    </Steps>

    Чтобы включить, отключить или удалить позже, используйте **+ > Plugins > Manage plugins**. Браузер плагинов недоступен в облачных сессиях настольного приложения. См. [Install plugins in the desktop app](/docs/ru/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    На панели Claude Code в VS Code:

    <Steps>
      <Step title="Откройте Manage plugins">
        Введите `/plugins` в поле подсказки, чтобы открыть **Manage plugins**.
      </Step>

      <Step title="Установите плагин">
        На вкладке **Plugins** найдите `commit-commands` и нажмите **Install**. Если вкладка не перечисляет плагины, сначала добавьте `anthropics/claude-plugins-official` на вкладке **Marketplaces**.
      </Step>

      <Step title="Выберите область">
        Выберите [область](#choose-an-install-scope): **Install for you**, **Install for this project** или **Install locally**.
      </Step>
    </Steps>

    Ваши изменения применяются к открытым сессиям без перезагрузки. См. [Manage plugins in VS Code](/docs/ru/vs-code#manage-plugins).
  </Tab>

  <Tab title="Cloud session">
    [Облачная сессия](/docs/ru/cloud-environments), включая [браузер на claude.ai/code](/docs/ru/claude-code-on-the-web), не имеет браузера плагинов и не загружает плагины, которые вы установили на своей машине, или те, которые включены в `.claude/settings.json` вашего репозитория. Для плагинов, которые ваша организация распространяет через управляемые настройки, см. [Manage plugins for your organization](/docs/ru/plugins/org).

    См. [какие части вашей настройки также доступны в облачной сессии](/docs/ru/cloud-environments#what-carries-over-from-your-setup) для остальной части вашей настройки.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Выберите область установки
</h3>

Область установки плагина определяет, кто получает плагин и какой файл настроек записывает его как включённый:

* **User scope**: плагин включен для вас в каждом проекте на этой машине. Запись идёт в `enabledPlugins` в `~/.claude/settings.json`.
* **Project scope**: плагин включен для всех, кто работает в этом репозитории. Запись идёт в `.claude/settings.json`, который вы коммитите.
* **Local scope**: плагин включен для вас только в этом репозитории. Запись идёт в `.claude/settings.local.json`.

Некоторые плагины установлены их автором на запуск в отключённом состоянии через поле [`defaultEnabled`](/docs/ru/plugins/manifest-reference#defaultenabled). Такой плагин установлен, но остаётся отключённым, пока вы не включите его с помощью `claude plugin enable <name>` в вашей оболочке или со вкладки **Installed** в `/plugin` в сессии.

Когда один и тот же плагин установлен в нескольких областях, локальная настройка переопределяет настройку проекта, а настройка проекта переопределяет пользовательскую настройку. См. [Find where a plugin is enabled](/docs/ru/plugins/loading#find-where-a-plugin-is-enabled) для полного правила.

Терминал, локальные сессии настольного приложения и расширение VS Code на одном компьютере читают одни и те же файлы настроек, поэтому плагин, который вы устанавливаете в пользовательской области в любом из них, доступен в двух других.

<h3 id="other-places-you-run-claude-code">
  JetBrains, неинтерактивные запуски и Agent SDK
</h3>

Некоторые места, где вы запускаете Claude Code, не имеют собственного браузера плагинов:

* **JetBrains IDEs**: плагин JetBrains запускает Claude Code в терминале IDE, поэтому используйте шаги вкладки **Terminal** там.
* **`claude -p` и другие неинтерактивные запуски**: `/plugin` не запускается, и Claude отвечает `/plugin isn't available in this environment.` Плагины, которые вы уже установили, загружаются. Устанавливайте и управляйте ими из вашей оболочки с помощью [`claude plugin` команд](#install-from-your-shell).
* **Agent SDK**: загружайте плагины через опцию плагина SDK. См. [Load plugins in the Agent SDK](/docs/ru/agent-sdk/plugins).

Если Claude Code сообщает, что плагин, включённый в `.claude/settings.json` репозитория, не установлен, см. [Enabled in project settings but not installed](/docs/ru/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Если вы автор плагина, тестирующий копию вашего плагина на диске, запустите Claude Code из вашей оболочки с `--plugin-dir`, чтобы загрузить его для одной сессии вместо установки. См. [Flags that load a plugin for one session](/docs/ru/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Плагины из вашего аккаунта claude.ai
</h3>

Ваш аккаунт claude.ai — это отдельный источник плагинов, наряду с маркетплейсами, из которых вы устанавливаете:

* **Что приходит**: каждый плагин, который вы включаете для своего аккаунта claude.ai, и каждый плагин, который ваша организация включает для своих членов. В сессии терминала они синхронизируются в фоне каждый раз, когда вы запускаете Claude Code, подписавшись с этим аккаунтом; в сессиях Cowork они загружаются при запуске сессии.
* **Где вы их видите**: в `/plugin` и `claude plugin list` под ID `<name>@synced`. Вы можете отключить один в своей области, если ваша организация этого не требует.
* **Что не идёт в другую сторону**: плагины, которые вы устанавливаете с помощью `/plugin` или `claude plugin install`, остаются на этой машине и не добавляются в ваш аккаунт claude.ai.

Для синхронизации времени, требований входа и отключения синхронизации см. [Plugins synced from claude.ai](/docs/ru/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Установка из вашей оболочки
</h3>

Запустите `claude plugin install` в вашей оболочке, чтобы установить плагин без запуска сессии Claude Code, например из скрипта настройки.

* **Область**: пользовательская область по умолчанию. Передайте `--scope project` или `--scope local`, чтобы изменить её.
* **Когда плагины загружаются**: плагины, которые он устанавливает, загружаются в следующий раз, когда вы запускаете Claude Code, или когда вы запускаете `/reload-plugins` в уже открытой сессии.
* **Маркетплейс должен быть добавлен первым**: на машине, где никто ещё не открывал интерактивную сессию Claude Code, официальный маркетплейс не зарегистрирован, поэтому скрипт, который устанавливает из него, запускает `claude plugin marketplace add anthropics/claude-plugins-official` перед установкой.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

Команда выводит `Successfully installed plugin: formatter@your-org (scope: project)` при завершении.

Некоторые плагины устанавливаются путём запуска команды, которую называет их маркетплейс, называемой [`command` source](/docs/ru/plugins/marketplace-reference#command-plugin-source). Claude Code показывает вам эту команду и просит вас принять её перед запуском. Скрипт не имеет никого, кто ответит на этот запрос, поэтому передайте `--yes` там, чтобы принять его.

Для каждого флага `claude plugin install` см. [plugin install](/docs/ru/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Добавление маркетплейса
</h2>

Вам нужен этот раздел только когда плагин, который вы хотите, не находится в официальном маркетплейсе Anthropic, например один, который опубликовал коллега, или один из маркетплейса сообщества Anthropic.

Маркетплейс — это каталог плагинов, и Claude Code должен знать о маркетплейсе, прежде чем вы сможете установить из него. Вы добавляете маркетплейс один раз. После этого его плагины появляются на вкладке **Discover** и устанавливаются с помощью `/plugin install <plugin>@<marketplace>` в сессии или `claude plugin install <plugin>@<marketplace>` в вашей оболочке, где `<marketplace>` — это имя, под которым маркетплейс зарегистрирован. Чтобы сделать оба в один шаг, см. [Add a marketplace and install in one command](#add-a-marketplace-and-install-in-one-command).

В сессии Claude Code запустите `/plugin marketplace add`, за которым следует источник маркетплейса: репозиторий GitHub, репозиторий git на любом хосте, локальный каталог или файл, или размещённый `marketplace.json`.

| Источник                       | Что вы вводите                                                                                                                                                                                                                                  | Пример                                                                                                                                |
| :----------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| Репозиторий GitHub             | `owner/repo`. Добавьте `#ref`, чтобы закрепить ветку или тег.                                                                                                                                                                                   | `/plugin marketplace add anthropics/claude-code`, или `/plugin marketplace add your-org/plugins#v1.2.0`, чтобы закрепить тег `v1.2.0` |
| Репозиторий Git на любом хосте | Полный URL клонирования. Добавьте `#ref`, чтобы закрепить ветку или тег.                                                                                                                                                                        | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                           |
| Локальный каталог или файл     | Относительный или абсолютный путь к каталогу, который содержит `.claude-plugin/marketplace.json`, или к самому JSON файлу. Начните относительный путь с `./` или `../`, потому что Claude Code читает голый `name/name` как репозиторий GitHub. | `/plugin marketplace add ./my-marketplace`                                                                                            |
| Размещённый `marketplace.json` | Его URL `https://`                                                                                                                                                                                                                              | `/plugin marketplace add https://example.com/marketplace.json`                                                                        |

Из вашей оболочки `claude plugin marketplace add` принимает те же источники.

<Tip>
  `/plugin market` также работает как более короткая форма `/plugin marketplace`.
</Tip>

Включайте префикс `https://` на каждый URL или используйте форму `git@host:path` для SSH. Если вы введёте голый `gitlab.example.com/your-group/your-marketplace.git`, Claude Code прочитает его как сокращение GitHub `owner/repo` и отклонит его.

Когда команда успешна, она выводит `Successfully added marketplace: <name>`, и плагины маркетплейса появляются на вкладке **Discover** в следующий раз, когда вы откроете `/plugin`, без необходимости перезагрузки. Если она не удаётся, сопоставьте сообщение об ошибке в [Troubleshoot plugins](/docs/ru/plugins/troubleshooting#add-a-marketplace).

<h3 id="add-a-marketplace-and-install-in-one-command">
  Добавление маркетплейса и установка в одной команде
</h3>

Чтобы установить плагин из маркетплейса, который вы ещё не добавили, запустите `/plugin install` в сессии Claude Code и назовите источник маркетплейса с помощью `--marketplace`. Требуется Claude Code v2.1.275 или позже.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

Источник принимает [те же формы, что и `/plugin marketplace add`](#add-a-marketplace), такие как GitHub `owner/repo`, URL git или локальный путь, за исключением того, что он не может содержать пробелы. Дайте имя плагина само по себе, без суффикса `@marketplace`.

Если вы ещё не добавили этот маркетплейс, Claude Code показывает разрешённый источник и просит вас подтвердить перед добавлением. После добавления маркетплейса детали плагина открываются и вы выбираете [область установки](#install-a-plugin). Если источник совпадает с маркетплейсом, который вы уже добавили, Claude Code пропускает подтверждение и открывает детали плагина в этом маркетплейсе.

<h3 id="add-a-private-marketplace">
  Добавление приватного маркетплейса
</h3>

Приватный маркетплейс — это один в репозитории, для которого вам нужны учётные данные для клонирования, на GitHub или любом другом хосте git. Вы добавляете его с помощью той же команды `/plugin marketplace add` или `claude plugin marketplace add`, что и публичный. Claude Code клонирует его с учётными данными git, уже находящимися на вашей машине, и никогда не запрашивает, поэтому каждый способ подключения имеет требование:

* **HTTPS**: применяются ваши помощники учётных данных git, поэтому доступ, который вы установили с помощью `gh auth login`, macOS Keychain или `git-credential-store`, работает. Интерактивные запросы подавляются, поэтому хост, к которому вы никогда не аутентифицировались, не удаётся вместо запроса пароля.
* **SSH**: хост должен уже находиться в вашем файле `known_hosts` и ключ должен работать без запроса парольной фразы, потому что запросы отпечатка хоста и парольной фразы также подавляются.
* **Сокращение GitHub `owner/repo`**: Claude Code проверяет, аутентифицирует ли ваш ключ SSH на `github.com`, затем клонирует по SSH, если это так, и по HTTPS, если нет. Установите [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/ru/env-vars#variables), чтобы пропустить эту проверку и всегда клонировать по HTTPS.

Те же учётные данные применяются, когда вы запускаете `/plugin install`, `/plugin marketplace update` и `claude plugin update`.

На хосте GitHub Enterprise Server см. [Plugin marketplaces on GHES](/docs/ru/github-enterprise-server#plugin-marketplaces-on-ghes) для учётных данных, которые требует каждая операция.

Если ваша организация регистрирует маркетплейс для вас через управляемые настройки, вы не добавляете его сами. См. [Pre-install and require plugins](/docs/ru/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Добавление маркетплейса из claude.ai
</h3>

В сессиях терминала, где [плагины синхронизируются из вашего аккаунта claude.ai](/docs/ru/plugins/loading#synced-plugins), claude.ai также может перечислять маркетплейсы плагинов для вас, такие как библиотека плагинов вашей организации и ваши собственные загрузки claude.ai. Вы добавляете один из них по его имени, а не по источнику. Добавление маркетплейса из claude.ai требует Claude Code v2.1.273 или позже.

Добавьте маркетплейс claude.ai из панели `/plugin` или из вашей оболочки:

* **Внутри сессии**: запустите `/plugin` и перейдите на вкладку **Marketplaces**, которая перечисляет маркетплейсы из claude.ai. Выберите один там, чтобы добавить его.
* **Из вашей оболочки**: запустите `claude plugin marketplace list`, который выводит их в разделе `From claude.ai:`. Затем запустите `claude plugin marketplace add` с флагом `--claudeai` и именем, показанным в списке.

Например, эта команда добавляет маркетплейс с именем `claudeai-organization-library`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code регистрирует маркетплейс под локальным именем, которое начинается с `claudeai-`, полученным из имени, которое claude.ai перечисляет под ним. Например, маркетплейс, перечисленный как "Organization library", становится `claudeai-organization-library`. Устанавливайте его плагины по этому имени, например с помощью `claude plugin install <plugin>@claudeai-organization-library`.

Если вы выходите или входите в другую организацию claude.ai, маркетплейс остаётся настроенным, но не показывает плагины, и плагины, которые вы уже установили из него, продолжают загружаться.

Раздел `From claude.ai:` также может перечислять маркетплейсы на основе git, общие через claude.ai, и выводит источник для каждого из них. Добавляйте их по этому источнику, как в [Add a marketplace](#add-a-marketplace), а не с `--claudeai`.

<h2 id="manage-installed-plugins">
  Управление установленными плагинами
</h2>

Вкладка **Installed** в `/plugin` перечисляет ваши плагины с действиями для включения, отключения, обновления или удаления каждого. В сессии Claude Code запустите `/plugin` и нажмите **Tab**, чтобы достичь её, или запустите `/plugin enable`, `/plugin disable` или `/plugin uninstall`, чтобы открыть панель и сделать это изменение там. Отключённые плагины сгруппированы под свёрнутым заголовком в нижней части списка. Используйте эти клавиши на списке:

* Введите для фильтрации по имени или описанию.
* Нажмите **Space**, чтобы включить или отключить выбранный плагин, и **f**, чтобы добавить его в избранное.
* Нажмите **Enter**, чтобы открыть детали плагина. Меню там предлагает **Disable plugin** или **Enable plugin**, **Update now** и **Uninstall**. Плагины, которые принимают настройки, также предлагают **Configure options**.

Вкладка также может показывать плагины в области **Managed**. Ваша организация установила их через [управляемые настройки](/docs/ru/settings#settings-files), и вы не можете включить, отключить или удалить их здесь.

Для синхронизированного плагина, который ваша организация требует на claude.ai, см. [Manage plugins synced from claude.ai](#manage-plugins-synced-from-claude-ai).

Когда вы закрываете панель `/plugin` с ожидающими изменениями, которые вы сделали в ней, Claude Code запускает `/reload-plugins` для вас, чтобы применить их. Если перезагрузка [инвалидирует кэш подсказок](/docs/ru/prompt-caching#enabling-or-disabling-a-plugin), она предупреждает и оставляет изменения в ожидании. Запустите `/reload-plugins --force`, чтобы применить их в любом случае.

<h3 id="manage-plugins-synced-from-claude-ai">
  Управление плагинами, синхронизированными из claude.ai
</h3>

Вкладка **Installed** в `/plugin` также перечисляет [плагины, синхронизированные из вашего аккаунта claude.ai](/docs/ru/plugins/loading#synced-plugins), с `synced` в качестве их источника. Синхронизированные плагины появляются в сессиях терминала на Claude Code v2.1.273 или позже.

* **Включить или отключить**: используйте вкладку **Installed**, если ваша организация не отметила плагин как обязательный.
* **Удалить**: отключите плагин на claude.ai.

Когда Claude Code синхронизирует добавленный, обновлённый или удалённый плагин в интерактивную сессию, вы видите `Plugins changed. Run /reload-plugins to activate.` Запустите `/reload-plugins`, чтобы загрузить изменение в этой сессии, или оставьте его на следующий раз, когда вы запустите Claude Code.

<h3 id="uninstall-a-plugin-the-project-enables">
  Удаление плагина, который включает проект
</h3>

Когда вы выбираете **Uninstall** для плагина, который этот репозиторий `.claude/settings.json` включает, будь то со вкладки **Installed** или с `/plugin uninstall`, Claude Code спрашивает, отключить ли его для вас или удалить для всех:

* **Disable for me**: нажмите **y**. Claude Code записывает `false` для плагина в вашем `.claude/settings.local.json` и оставляет его установленным для проекта.
* **Uninstall for everyone**: нажмите **u**. Claude Code удаляет плагин из общего `.claude/settings.json`.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Посмотрите, что добавляет установленный плагин к вашим сессиям
</h3>

В вашей оболочке запустите `claude plugin details <name>` для установленного плагина. Строка `Always-on` — это количество токенов, которые плагин добавляет к каждой сессии, где он включен, и строки для каждого компонента показывают, какой skill или agent вносит наибольший вклад. Для полного вывода и того, что означает каждая цифра, см. [Measure what a plugin costs](/docs/ru/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Найдите плагины, которые вы больше не используете
</h3>

На вкладке **Installed** в `/plugin` плагины, которые вы установили сами и не использовали недавно, появляются под заголовком **Not used recently**, и детали каждого плагина показывают строку **Last used**. Используйте этот заголовок и эту строку, чтобы найти плагины, которые всё ещё добавляют стартовые и контекстные затраты, затем отключите или удалите их.

<h3 id="plugins-with-dependencies">
  Плагины с зависимостями
</h3>

Плагин может объявить другие плагины, от которых он зависит. Когда вы устанавливаете, отключаете или удаляете такой плагин из маркетплейса, Claude Code действует на эти зависимости тоже:

* **Install**: Claude Code также устанавливает и включает объявленные зависимости плагина в той же области. Сообщение об успехе их перечисляет.
* **Enable**: Claude Code также включает зависимости плагина, которые установлены, но отключены. Если объявленная зависимость не установлена, включение не удаётся и сообщение говорит вам установить её сначала.
* **Disable**: когда другой включённый плагин всё ещё нуждается в том, который вы назвали, Claude Code отказывает и выводит цепную команду, которая отключает оба в правильном порядке.
* **Uninstall**: автоустановленные зависимости остаются, пока вы не запустите `claude plugin prune` в вашей оболочке; см. [plugin prune](/docs/ru/plugins/cli-reference#plugin-prune).

Если вы загрузили плагин с `--plugin-dir` вместо этого, см. [Test a plugin and its dependency locally](/docs/ru/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Управление плагинами из вашей оболочки
</h3>

Вы также можете управлять плагинами без запуска сессии Claude Code. В вашей оболочке запустите `claude plugin install`, `enable`, `disable` или `uninstall` как обычные команды терминала; они изменяют те же настройки, что и панель `/plugin`. Каждый принимает `--scope`, чтобы нацелиться на одну область, и использует область по умолчанию, когда вы её опускаете:

* `enable` и `disable` действуют на наиболее специфичную область, чьи настройки уже перечисляют плагин.
* `install` и `uninstall` действуют на пользовательскую область.

Например, эти команды отключают и повторно включают плагин, затем удаляют его в области проекта:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Поддержание плагинов в актуальном состоянии
</h2>

Плагины обновляются автоматически, когда маркетплейс, из которого они поступают, имеет включённое автообновление. После запуска сессии Claude Code обновляет эти маркетплейсы и обновляет копии плагинов на диске, которые вы установили из них.

Запущенная сессия сохраняет версии, которые она уже загрузила. После обновления вы видите `Plugin updated: <name> · Run /reload-plugins to apply`, и следующая сессия автоматически загружает новые версии.

Это значения по умолчанию для автообновления для каждого вида маркетплейса:

* **On by default**: `claude-plugins-official` и другие [официальные имена маркетплейсов](/docs/ru/plugins/security#official-marketplace-names), кроме `knowledge-work-plugins` и `first-party-plugins`, плюс [маркетплейсы, добавленные из claude.ai](#add-from-claude-ai).
* **Off by default**: каждый другой маркетплейс, включая маркетплейс сообщества, маркетплейсы третьих сторон и локальные маркетплейсы разработки.

Для того, когда запускается автообновление, какие плагины оно пропускает и переменные окружения, которые его отключают, см. [When auto-update runs](/docs/ru/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Включение или отключение автообновления для маркетплейса
</h3>

В сессии Claude Code запустите `/plugin` и перейдите на вкладку **Marketplaces**. Выберите маркетплейс, затем выберите **Enable auto-update** или **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Обновление одного плагина сейчас
</h3>

В сессии откройте плагин на вкладке **Installed** в `/plugin` и выберите **Update now**, или в вашей оболочке запустите `claude plugin update <plugin>@<marketplace>`.

<h3 id="auto-update-from-a-private-marketplace">
  Автообновление из приватного маркетплейса
</h3>

Для приватного маркетплейса см. [What background auto-update does with credentials](/docs/ru/plugins/host-marketplace#what-background-auto-update-does-with-credentials) для того, как фоновые автообновления аутентифицируются по SSH и HTTPS, и [Troubleshoot plugins](/docs/ru/plugins/troubleshooting#add-a-marketplace) для сообщений, которые вы видите, когда они не удаются.

<h2 id="manage-marketplaces">
  Управление маркетплейсами
</h2>

Вкладка **Marketplaces** в `/plugin` перечисляет каждый маркетплейс, который вы зарегистрировали, вместе с его источником. Выберите один, чтобы просмотреть его плагины, обновить его список, включить или отключить автообновление или удалить его.

Вы также можете перечислять, обновлять и удалять маркетплейсы с помощью команд, из вашей оболочки или внутри сессии:

| Действие                     | В вашей оболочке                          | Внутри сессии                       |
| :--------------------------- | :---------------------------------------- | :---------------------------------- |
| Перечислить маркетплейсы     | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Обновить список маркетплейса | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Удалить маркетплейс          | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Когда вы удаляете маркетплейс, Claude Code удаляет каждый плагин, который вы установили из него, и удаляет их записи `enabledPlugins` из ваших файлов настроек. Вкладка **Marketplaces** называет эти плагины перед тем, как попросить вас подтвердить.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Anthropic's marketplaces](/docs/ru/plugins/anthropic-marketplaces): как отличаются официальный, маркетплейсы сообщества и демонстрационный и где просмотреть каждый
* [Plugin loading reference](/docs/ru/plugins/loading): почему плагин загрузился, не загрузился или не изменился после обновления
* [Plugin security and trust](/docs/ru/plugins/security): что просмотреть перед установкой плагина из маркетплейса, который вы не знаете
* [Troubleshoot plugins](/docs/ru/plugins/troubleshooting): установка и сообщения об ошибках маркетплейса с их исправлениями
* [Create a plugin](/docs/ru/plugins/create): создайте свой собственный
