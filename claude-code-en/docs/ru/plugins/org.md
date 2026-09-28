> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Управление плагинами Claude Code для вашей организации

> Контролируйте, какие плагины Claude Code устанавливает и разрешает на каждой машине в вашей организации через управляемые параметры.

Управляемые параметры позволяют вам решать, какие плагины Claude Code устанавливает и разрешает на каждой машине в вашей организации. Пользователи не могут их переопределить. Вы доставляете их либо как [управляемые параметры сервера](/docs/ru/server-managed-settings) из консоли администратора claude.ai, либо как управляемые параметры конечной точки через MDM или файл `managed-settings.json`. Большинство элементов управления на этой странице вступают в силу только из управляемых параметров.

Эта страница предназначена для администраторов и управляет Claude Code.

<Note>
  Эти случаи рассматриваются на других страницах:

  * **Установка плагинов для себя**: начните с [Установка плагинов](/docs/ru/plugins/install)
  * **Контроль, какие плагины члены могут использовать в claude.ai и Cowork**: см. [Управление плагинами для вашей организации](https://support.claude.com/en/articles/13837433) в справочном центре
  * **Страница плагинов в параметрах администратора claude.ai**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) включает плагины для учетных записей claude.ai членов, и они достигают Claude Code как [синхронизированные плагины](/docs/ru/plugins/loading#synced-plugins). Это не устанавливает ни один из ключей на этой странице
</Note>

Разделы следуют порядку, который обычно принимают развертывания: [требуют плагины](#pre-install-and-require-plugins) для всех или по репозиторию, [заполняют контейнеры и CI](#seed-containers-and-ci), [ограничивают](#restrict-what-users-can-install) то, что пользователи могут добавить самостоятельно, [устанавливают политику обновления](#set-update-policy), затем [проверяют](#audit-and-review) то, что установлено. Чтобы просмотреть каждый ключ политики в одном месте, см. [матрицу управления](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Pre-install and require plugins
</h2>

Marketplace — это каталог плагинов, который Claude Code получает из репозитория git, URL-адреса или локального пути. После регистрации marketplace на машине Claude Code может устанавливать плагины из него.

Чтобы установить плагины для флота, установите два ключа вместе в [управляемые параметры](/docs/ru/managed-settings), файл политики или политику, доставляемую сервером, которую каждая машина в вашей организации читает: `extraKnownMarketplaces` регистрирует marketplace на каждой машине, а `enabledPlugins` называет плагины для установки и включения из него. [Выберите механизм доставки](#choose-a-delivery-mechanism) охватывает, как управляемые параметры достигают каждой машины.

<h3 id="choose-a-delivery-mechanism">
  Choose a delivery mechanism
</h3>

Управляемые параметры достигают машины через один из трех механизмов доставки:

* **Server-managed settings**: установите ключи плагина как JSON в [**Organization settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code). Требует роль [Owner](/docs/ru/server-managed-settings#access-control) в вашей организации Claude. Облачная сессия получает эти параметры перед установкой плагинов.
* **MDM policies**: на macOS доставьте plist, чьи ключи верхнего уровня являются ключами параметров. На Windows сохраните весь документ JSON как строку в значение реестра. Домен plist и ключ реестра находятся в [Где каждый механизм хранит политику](/docs/ru/managed-settings#where-each-mechanism-stores-the-policy).
* **Managed settings file**: поместите `managed-settings.json` в системный путь платформы. Вы также можете добавить файлы в каталог drop-in `managed-settings.d/` рядом с ним. Пути файлов для каждой платформы находятся в [Где каждый механизм хранит политику](/docs/ru/managed-settings#where-each-mechanism-stores-the-policy), а правила слияния drop-in находятся в [Разделить файловую политику между командами](/docs/ru/managed-settings#split-a-file-based-policy-across-teams).

Используйте управляемые параметры сервера, если у вас есть организация Claude for Teams или Enterprise на claude.ai и ваши устройства не все находятся под MDM. В противном случае используйте политику MDM или файл управляемых параметров. Для компромисса см. [Выбор между управляемыми параметрами сервера и конечной точки](/docs/ru/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Which managed source applies on a machine
</h4>

По умолчанию на машине применяется только один из этих трех источников. Claude Code использует первый, который доставляет ключ политики, проверяя сначала управляемые параметры сервера, затем политики MDM, затем файл управляемых параметров. Если управляемые параметры сервера доставляют даже один несвязанный ключ политики, Claude Code игнорирует ключи плагина в политике MDM или файле управляемых параметров на этой машине, кроме [ключей, которые он читает из каждого источника](/docs/ru/managed-settings#keys-read-from-every-admin-source).

Чтобы применить каждый источник вместо этого, установите [`managedSourcesBehavior`](/docs/ru/managed-settings#compose-every-managed-source) на `"merge"`.

[Как Claude Code объединяет управляемые источники](/docs/ru/managed-settings#how-claude-code-combines-managed-sources) также перечисляет ключи, которые Claude Code читает из каждого источника в обоих режимах.

<h3 id="require-a-marketplace-and-its-plugins">
  Require a marketplace and its plugins
</h3>

Добавьте marketplace под `extraKnownMarketplaces`, ключ по собственному `name` marketplace из его `marketplace.json`. Затем добавьте каждый плагин под `enabledPlugins` как `plugin-name@marketplace-name`. Каждая запись marketplace содержит объект `source` с полем `source`, называющим тип, например `github`. Этот пример управляемых параметров регистрирует marketplace организации и принудительно включает два плагина из него:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

После того как параметры достигнут машины, Claude Code регистрирует marketplace и устанавливает два плагина в начале следующей сессии пользователя. Пользователи видят их в `/plugin`, и отключение одного в своей области не останавливает его загрузку, потому что управляемые параметры имеют приоритет над всеми остальными областями.

Чтобы заблокировать плагин во всех областях и скрыть его из списка marketplace, установите его на `false` в управляемом `enabledPlugins` вместо этого.

Отрегулируйте поля `autoUpdate` и `source` для вашего marketplace:

* **`autoUpdate`**: `true` сохраняет marketplace и его плагины обновляющимися в фоновом режиме, а `false` отключает это. См. [Установка политики обновления](#set-update-policy).
* **`source`**: `github` — один из нескольких типов источников. Источник `git` принимает `url` для GitLab или внутреннего хоста, а источник `url` принимает адрес размещенного `marketplace.json`. Каждая форма источника находится в [справочнике marketplace](/docs/ru/plugins/marketplace-reference).

Если marketplace — это приватный репозиторий git, каждому пользователю нужен доступ на чтение к нему. Клонирование marketplace на основе git работает с git на машине пользователя, используя сохраненные учетные данные и без подсказок. Для пользователей без учетных записей на хосте git используйте [seed](#seed-containers-and-ci) вместо этого.

Управляемая запись также переопределяет запись marketplace с тем же именем или копию `--plugin-dir` из другого источника:

* **Marketplaces**: управляемая запись marketplace заменяет запись с более низким приоритетом с тем же именем, и поля двух записей не объединяются.
* **`--plugin-dir` copies**: `--plugin-dir` загружает плагин из локального каталога для одной сессии. Для того, что происходит, когда имя этой копии совпадает с плагином, который ваш управляемый `enabledPlugins` называет, см. [Конфликты имен](/docs/ru/plugins/loading#name-conflicts).

Официальный marketplace Anthropic `claude-plugins-official` не требует записи `extraKnownMarketplaces` когда `enabledPlugins` устанавливает один из его плагинов на `true`. Эта запись `name@claude-plugins-official` объявляет marketplace сама по себе, где бы эти ключи ни применялись. Если вы не включаете ни один из его плагинов и все еще хотите, чтобы он был зарегистрирован на каждой машине, дайте ему явную запись, как это делает [Разрешить официальный marketplace и ваш собственный](#allow-the-official-marketplace-and-your-own).

<h3 id="require-plugins-per-repository">
  Require plugins per repository
</h3>

Чтобы охватить участников одного репозитория вместо всего флота, установите `extraKnownMarketplaces` и `enabledPlugins` в `.claude/settings.json` этого репозитория. Записи `extraKnownMarketplaces` применяются только в папке, которую участник доверил, и в недоверенной папке Claude Code игнорирует их без сообщения:

* **Interactive sessions**: Claude Code регистрирует marketplace только после того, как участник примет [диалог доверия рабочей области](/docs/ru/permissions#what-runs-before-you-trust-a-folder) для этой папки.
* **[Non-interactive `-p` runs](/docs/ru/headless)**: записи применяются только в папке, доверие которой пользователь уже принял интерактивно, или чей флаг `hasTrustDialogAccepted` вы установили в `~/.claude.json`.

Плагин, который marketplace перечисляет по относительному пути, загружается из копии marketplace после того, как записи `extraKnownMarketplaces` репозитория применяются. Плагин, чья запись marketplace указывает на внешний источник вместо этого, например собственный репозиторий GitHub плагина, не устанавливается только из параметров репозитория. Каждый участник видит `Plugin "<name>" is enabled in project settings but isn't installed` до тех пор, пока они не запустят `claude plugin install <name>@<marketplace> --scope project`, как описано в [Установка плагинов](/docs/ru/plugins/install).

Если вы используете локальный источник `directory` или `file` с относительным путем, путь разрешается относительно основного checkout вашего репозитория. Когда вы запускаете Claude Code из git worktree, путь все еще указывает на основной checkout, поэтому все worktrees совместно используют одно и то же местоположение marketplace.

Чтобы развернуть пакет плагинов с зависимостями, поместите плагин пакета в `enabledPlugins`, как описано в [Зависимости плагинов](/docs/ru/plugins/dependencies).

<h3 id="when-each-surface-applies-the-plugin-keys">
  When each surface applies the plugin keys
</h3>

Таблица показывает, когда каждый вид сессии Claude Code применяет `extraKnownMarketplaces` и `enabledPlugins`, из управляемых параметров и из `.claude/settings.json` репозитория. Для Desktop app и расширений IDE см. [Установка плагина](/docs/ru/plugins/install#install-a-plugin).

| Surface               | Managed `extraKnownMarketplaces` and `enabledPlugins`                                                                                                                                                                                                                                                                               | Repository `.claude/settings.json`                                                           |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| Terminal, interactive | Applied at session start on every machine that receives the settings                                                                                                                                                                                                                                                                | `extraKnownMarketplaces` applied after trust; `enabledPlugins` applied at session start      |
| `-p` and CI           | Applied at session start, with installs running in the background                                                                                                                                                                                                                                                                   | `extraKnownMarketplaces` in trusted folders only; `enabledPlugins` applied                   |
| Cloud sessions        | In an Anthropic-hosted environment, only server-managed settings reach the session, which waits for them before it installs plugins. MDM policies and managed settings files stay on the user's machine. For a self-hosted environment, see [Where and when a policy applies](/docs/ru/managed-settings#where-and-when-a-policy-applies) | See the **Cloud session** tab under [Install a plugin](/docs/ru/plugins/install#install-a-plugin) |

В запуске `-p` или CI marketplaces и плагины устанавливаются в фоновом режиме, поэтому плагин может отсутствовать в первом повороте. Установите `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1`, чтобы запуск ждал установки перед своим первым запросом.

<h3 id="confirm-the-rollout">
  Confirm the rollout
</h3>

Проверьте, что marketplace и плагины прибыли на машину или в запуск CI:

* **На одной машине**: запустите Claude Code и запустите `/plugin`. Marketplace и плагины перечислены.
* **В CI**: запустите `claude -p` с `--output-format stream-json --verbose`. Событие `init` перечисляет загруженные плагины под `plugins`.

<h2 id="seed-containers-and-ci">
  Seed containers and CI
</h2>

Для образов контейнеров и CI runners, которые не могут клонировать во время выполнения, предварительно заполните каталог плагинов во время сборки и укажите `CLAUDE_CODE_PLUGIN_SEED_DIR` на него. Claude Code регистрирует marketplaces seed при запуске и загружает кэши плагинов из seed на месте, без клонирования.

Seed также служит пользователям, у которых нет учетной записи на хосте git.

<Note>
  В средах CI/CD настройте помощник учетных данных git перед установкой плагинов из приватных репозиториев. На GitHub Actions экспортируйте токен с доступом на чтение к репозиторию marketplace как `GH_TOKEN`, затем запустите `gh auth setup-git`. Токен рабочего процесса по умолчанию может получить доступ только к репозиторию самого рабочего процесса, поэтому приватный marketplace в другом репозитории требует личного токена доступа или токена приложения.
</Note>

<Steps>
  <Step title="Install into the seed at build time">
    Установите `CLAUDE_CODE_PLUGIN_CACHE_DIR` на путь seed, чтобы marketplace и плагины устанавливались туда вместо `~/.claude/plugins`:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    Seed имеет тот же макет, что и `~/.claude/plugins`: `known_marketplaces.json`, `marketplaces/<name>/`, и `cache/<marketplace>/<plugin>/<version>/`. Вы можете смонтировать seed по другому пути, чем вы его построили.
  </Step>

  <Step title="Point the runtime at the seed">
    Установите `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` в окружении контейнера. Чтобы использовать несколько seeds, разделите их пути с `:` на Unix или `;` на Windows. Claude Code использует первый seed, который содержит данный marketplace или кэш плагина.
  </Step>

  <Step title="Enable the plugins">
    Плагины в seed не включены сами по себе. Установите `enabledPlugins` для каждого плагина seed, который вы хотите загрузить, в управляемые параметры или в `.claude/settings.json` репозитория.
  </Step>
</Steps>

Чтобы проверить seed, запустите `claude -p` с `--output-format stream-json --verbose` в образе. В списке `plugins` события `init` путь каждого загруженного плагина находится под seed, например `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

Seed marketplaces следуют этим правилам:

* **Read-only**: Claude Code никогда не пишет в seed и принудительно отключает `autoUpdate` для seed marketplaces.
* **Seed entries take precedence**: при каждом запуске marketplace, объявленный в seed, перезаписывает запись пользователя с тем же именем. Пользователи отказываются от плагина seed с `claude plugin disable`, а не удаляя marketplace.
* **Update and remove fail**: `claude plugin marketplace update <name>` и `remove` без `--scope` на seed marketplace не работают с сообщением, которое называет каталог seed.
* **Policy still applies**: [allowlist и blocklist](#restrict-what-users-can-install) проверяют записанный источник seed marketplace тоже. Разрешите источник, из которого вы построили seed.

Для флотов без исходящего доступа git объедините seed с источниками marketplace `directory` или `file` на общем монтировании. Установите `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` также, что также отключает [автоматическое обновление плагина](/docs/ru/plugins/loading#when-auto-update-runs). Если доступен прокси, см. [Конфигурация прокси](/docs/ru/network-config#proxy-configuration) для переменных, которые нужно установить.

<h2 id="restrict-what-users-can-install">
  Restrict what users can install
</h2>

Управляемый allowlist `strictKnownMarketplaces` и blocklist `blockedMarketplaces` решают, из каких источников marketplace могут поступать плагины. Источник marketplace — это репозиторий git, URL-адрес или локальный путь, который Claude Code получает из него. Оба списка совпадают с источником marketplace, из которого поступает плагин, а не с собственной записью плагина внутри этого marketplace.

Для обычной блокировки, которая разрешает официальный marketplace и ваш собственный, см. [Разрешить официальный marketplace и ваш собственный](#allow-the-official-marketplace-and-your-own). Объедините его с [`disableSideloadFlags`](#control-matrix), чтобы пользователи не могли загружать плагины из локального каталога или URL-адреса.

Оба списка применяются перед загрузкой чего-либо и снова при запуске сессии:

* **Before a download**: списки применяются, когда пользователь добавляет marketplace и при каждой установке, обновлении, обновлении и автоматическом обновлении.
* **At session start**: списки применяются снова к плагинам, которые уже установлены, поэтому установленный плагин, чей источник marketplace больше не совпадает, не загружается. `/plugin` перечисляет его с `Marketplace "<name>" is not in the allowed marketplace list` или `Marketplace "<name>" is blocked by enterprise policy`.

Где два списка применяются, зависит от того, где вы их установили:

* **The claude.ai admin console**: Claude Code применяет оба списка в сессиях, которые [читают управляемые параметры сервера](/docs/ru/managed-settings#where-and-when-a-policy-applies). claude.ai также проверяет их, когда кто-либо в вашей организации добавляет новый marketplace из репозитория git на claude.ai или из **Customize** в Claude Desktop app вне его вкладки Code. Это охватывает marketplace, который член добавляет для своей собственной учетной записи, и тот, который добавляется для всей организации под [**Organization settings > Plugins**](https://claude.ai/admin-settings/plugins). claude.ai отказывает репозиторию, который allowlist не допускает или который blocklist называет. Он не переопределяет marketplace, который был добавлен в любом месте до того, как вы установили списки, и он не проверяет загруженные плагины.
* **A managed settings file, OS-level policy, or other managed source**: Claude Code применяет оба списка, где он читает этот источник. claude.ai не читает его.

Пока установлен какой-либо allowlist, или blocklist называет любой источник, кроме [`skills-dir`](#blocklist-with-blockedmarketplaces), плагин, чей marketplace Claude Code не может найти, не загружается. `/plugin` показывает ошибку политики для него, а не ошибку не найдено. Обычный случай — устаревшая запись `enabledPlugins` для marketplace, который никто не регистрировал.

<h3 id="control-matrix">
  Control matrix
</h3>

Таблица перечисляет каждый ключ политики плагина, что он применяет, и что он не может делать.

| Key                                                                      | What it enforces                                                                                                                                                                                                                                                                                         | What it can't do                                                                                                                                                                                                                                               |
| :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | Allowlist источников marketplace. `[]` блокирует каждый источник, включая официальный marketplace. Alias: `allowedMarketplaces`                                                                                                                                                                          | Не регистрирует marketplace, не ограничивает записи внутри разрешенного marketplace или не блокирует `--plugin-dir`                                                                                                                                            |
| `blockedMarketplaces`                                                    | Blocklist источников marketplace, проверяется перед allowlist                                                                                                                                                                                                                                            | Не блокирует marketplace, уже зарегистрированный из источника, который он не совпадает                                                                                                                                                                         |
| `syncClaudeAiPlugins`                                                    | Установите `false`, чтобы остановить Claude Code загрузку и загрузку плагинов [синхронизированных из claude.ai](/docs/ru/plugins/loading#synced-plugins) для учетной записи каждого пользователя. Требует Claude Code v2.1.273 или позже                                                                      | Не отключает один синхронизированный плагин. Для этого установите `"<name>@synced": false` в [`enabledPlugins`](/docs/ru/settings-reference#enabledplugins)                                                                                                         |
| `enabledPlugins`                                                         | `true` принудительно включает, `false` блокирует во всех областях и скрывает плагин                                                                                                                                                                                                                      | Не устанавливает плагин, чей marketplace не зарегистрирован или не разрешен                                                                                                                                                                                    |
| `disableSideloadFlags`                                                   | Отклоняет `--plugin-dir`, `--plugin-url`, `--agents`, опцию Agent SDK `plugins` и non-SDK `--mcp-config` при запуске, и отклоняет папки, названные в переменной [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ru/env-vars#variables) так же                                                                               | Не ограничивает `.mcp.json`, `claude mcp add` или предоставленные SDK серверы. Объедините его с [`allowedMcpServers`](/docs/ru/managed-mcp)                                                                                                                         |
| `disableCommandPluginSources`                                            | Блокирует плагины с источником `command` от установки, обновления или загрузки. Источник `command` — это тот, чей каталог плагина создается путем запуска команды на машине. Когда не установлено, принимает значение `allowManagedHooksOnly`                                                            | Не влияет на другие типы источников                                                                                                                                                                                                                            |
| `allowManagedHooksOnly`                                                  | Ограничивает, какие hooks работают. См. [`allowManagedHooksOnly`](/docs/ru/settings-reference#allowmanagedhooksonly)                                                                                                                                                                                          | Не доверяет hooks из плагинов, которые пользователи включают сами                                                                                                                                                                                              |
| `strictPluginOnlyCustomization`                                          | Блокирует skills, agents, hooks и MCP серверы, которые не поступают из плагина, управляемых параметров или встроенных Claude Code. Установите `true`, чтобы охватить все четыре типа, или массив значений `skills`, `agents`, `hooks` и `mcp`, таких как `["skills", "hooks"]`, чтобы охватить некоторые | Не ограничивает, какие плагины пользователи устанавливают. Объедините его с `strictKnownMarketplaces`                                                                                                                                                          |
| `pluginSuggestionMarketplaces`                                           | Marketplaces, чьи плагины могут появляться как предложения установки. См. [Рекомендовать плагины](#recommend-plugins)                                                                                                                                                                                    | Не влияет на встроенные советы                                                                                                                                                                                                                                 |
| `pluginTrustMessage`                                                     | Добавляет ваш текст к предупреждению доверия, которое `/plugin` показывает перед установкой плагина                                                                                                                                                                                                      | Не изменяет собственный текст предупреждения                                                                                                                                                                                                                   |
| `allowedChannelPlugins`                                                  | Заменяет список плагинов по умолчанию, разрешенных для отправки сообщений канала. Требует `channelsEnabled: true`                                                                                                                                                                                        | См. [Ограничить, какие плагины канала могут работать](/docs/ru/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                     |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/ru/env-vars) | Останавливает интерактивные сессии терминала от автоматической регистрации официального marketplace                                                                                                                                                                                                      | Не удаляет marketplace, уже зарегистрированный. Allowlist и blocklist управляют той же автоматической регистрацией без него. Машина, которая запустилась один раз с установленным, не возобновляет автоматическую регистрацию после того, как вы его отключите |

Каждый ключ в таблице является управляемым параметром, кроме `enabledPlugins`, `syncClaudeAiPlugins` и `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`:

* **`enabledPlugins`**: вы можете установить его в любой области, и управляемые параметры его блокируют.
* **`syncClaudeAiPlugins`**: каждый пользователь также может установить его в своих собственных пользовательских или локальных параметрах. См. его [область в справочнике параметров](/docs/ru/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: это переменная окружения, которую вы доставляете через управляемый блок `env`, показанный под [Отключить обновления для всего флота](#turn-updates-off-for-the-whole-fleet).

Каждый ключ параметров здесь имеет запись в [справочнике параметров](/docs/ru/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Aliases for the marketplace keys
</h4>

`strictKnownMarketplaces` также может быть написан как `allowedMarketplaces`, а `extraKnownMarketplaces` также может быть написан как `additionalMarketplaces`.

* **Version**: aliases требуют Claude Code v2.1.232 или позже, и старые клиенты их игнорируют. В файле, который читает смешанный флот, сохраняйте канонические имена.
* **Both spellings set**: когда файл устанавливает оба написания, применяется значение канонического ключа.

<h3 id="allowlist-with-strictknownmarketplaces">
  Allowlist with `strictKnownMarketplaces`
</h3>

Установите allowlist на список этих объектов источника. Большинство записей совпадают точно, записи `hostPattern` и `pathPattern` совпадают как регулярные выражения, и подстановочные знаки владельца `github` совпадают по владельцу:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, с опциональными `ref` и `path`.
* **`github` owner wildcard**: `{ "source": "github", "repo": "your-org/*" }` совпадает с каждым репозиторием под этим владельцем. `*` должен стоять за всем именем репозитория. Claude Code игнорирует записи, такие как `*/plugins` и `your-org/tools-*`, как недействительные, поэтому они ничему не совпадают. Требует Claude Code v2.1.223 или позже.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, с опциональными `ref` и `path`.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, с опциональными `headers`.
* **`file` and `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` или `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, с абсолютными путями.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, совпадает с хостом источников `github`, `git` и `url`. Шаблон совпадает в любом месте имени хоста, поэтому закрепите его с `^` и `$`, как показано, чтобы совпадать со всем хостом. Источник `github` всегда считается `github.com`. Используйте запись `hostPattern` для GitHub Enterprise Server или хоста GitLab, где разработчики создают свои собственные marketplaces. [Страница GHES](/docs/ru/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) имеет отработанный пример.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, совпадает с `path` источников `file` и `directory`. Шаблон совпадает в любом месте пути, поэтому начните его с `^`, чтобы закрепить префикс каталога. `".*"` разрешает каждый локальный путь.
* **`skills-dir`**: `{ "source": "skills-dir" }` сохраняет [плагины skills-directory](#keep-skills-directory-plugins-loading) загружающимися при установленном allowlist и не совпадает ни с каким marketplace.

<h4 id="how-entries-match">
  How entries match
</h4>

Запись `url` совпадает по своему значению `url`; `headers` не сравниваются. Для записей `github` и `git`, `repo` или `url`, `ref` и `path` должны все совпадать или отсутствовать с обеих сторон:

* Запись без `ref` не охватывает источник с `ref: "main"`.
* Запись для `your-org/your-marketplace` не охватывает URL `git`, который клонирует тот же репозиторий.
* Косая черта в конце, суффикс `.git` или `ssh://` вместо `https://` — это другое значение. Когда marketplace может быть клонирован более чем одним URL-адресом, предпочитайте запись `hostPattern`.

Записи с подстановочным знаком владельца следуют точным правилам для `ref` и совпадают с любым `path` внутри репозитория, если запись не закрепляет один. Сопоставление подстановочных знаков чувствительно к регистру в allowlist.

<h4 id="keep-skills-directory-plugins-loading">
  Keep skills-directory plugins loading
</h4>

Плагины skills-directory — это плагины, которые пользователи хранят под `~/.claude/skills/` или `.claude/skills/` проекта в папках, которые содержат `.claude-plugin/plugin.json`. Если вы установите какой-либо allowlist без записи `{ "source": "skills-dir" }`, они перестанут загружаться. Простые [skills](/docs/ru/skills), означающие `SKILL.md` без этого манифеста, продолжают загружаться.

<h4 id="marketplaces-hosted-on-claude-ai">
  Marketplaces hosted on claude.ai
</h4>

Allowlist и blocklist совпадают с [marketplace, размещенным на claude.ai](/docs/ru/plugins/install#add-from-claude-ai), по его хосту. Чтобы разрешить или заблокировать один, добавьте запись `hostPattern`, которая совпадает с `claude.ai`, в `strictKnownMarketplaces` или `blockedMarketplaces`. На allowlist такая запись допускает ваши marketplaces claude.ai организации и marketplaces claude.ai по умолчанию, но не marketplace, состоящий из собственных загрузок claude.ai члена или тот, чья область claude.ai не указал. Требует Claude Code v2.1.273 или позже.

<h4 id="lock-every-source-out">
  Lock every source out
</h4>

Пустой allowlist, `[]`, блокирует каждый источник marketplace, включая официальный marketplace.

Эта блокировка не охватывает плагины [синхронизированные из claude.ai](/docs/ru/plugins/loading#synced-plugins), которые Claude Code загружает из учетной записи каждого пользователя, а не из marketplace. Чтобы остановить и те, установите [`syncClaudeAiPlugins`](/docs/ru/settings-reference#syncclaudeaiplugins) на `false` в управляемых параметрах или отключите Skills для вашей организации на claude.ai.

<h3 id="blocklist-with-blockedmarketplaces">
  Blocklist with `blockedMarketplaces`
</h3>

`blockedMarketplaces` принимает те же объекты источника, что и [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces), и проверяется первым, поэтому источник в обоих списках блокируется. Сопоставление blocklist шире, чем сопоставление allowlist:

* URL-адреса Git канонизируются, поэтому формы `git@` и `https://`, суффиксы `.git` и косые черты в конце одного репозитория `github.com` все совпадают с одной записью.
* Запись `github` также блокирует эквивалентный URL `git`, и наоборот.
* Для записи `owner/*`, сравнение владельца не чувствительно к регистру.
* Запись без `ref` или `path` блокирует каждый ref и path репозиториев, которые она совпадает.

Эта запись блокирует каждый репозиторий под одним владельцем GitHub:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

Записи `url` в `blockedMarketplaces` также применяются, когда пользователь добавляет URL репозитория `https://`, который Claude Code [клонирует, а не получает](/docs/ru/plugins/cli-reference#plugin-marketplace-add), например простой URL репозитория `github.com` или `gitlab.com`. Пользователь не может добавить этот URL, если запись его называет. Сопоставление игнорирует суффикс `.git` и любой ref, который пользователь добавляет после `#`. Требует Claude Code v2.1.232 или позже.

Запись `{ "source": "skills-dir" }` здесь останавливает [плагины skills-directory](#keep-skills-directory-plugins-loading) от загрузки, из обоих `~/.claude/skills/` и `.claude/skills/` проекта.

Blocklist, который называет только эту запись, не считается активным ограничением, поэтому он не [останавливает плагины, чей marketplace Claude Code не может найти](#restrict-what-users-can-install) от загрузки.

<h3 id="allow-the-official-marketplace-and-your-own">
  Allow the official marketplace and your own
</h3>

Большинство организаций разрешают официальный marketplace и свой собственный, и регистрируют оба, чтобы каждая машина имела их. Эта политика управляемых параметров разрешает оба marketplace, регистрирует оба, принудительно включает два плагина и отклоняет `--plugin-dir`:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

На машине с этой политикой добавление любого источника вне списка, например `/plugin marketplace add https://example.com/other-marketplace.git`, не работает с сообщением, содержащим `is blocked by enterprise policy`, за которым следуют разрешенные источники. `claude --plugin-dir ./x` выходит с сообщением, называющим `disableSideloadFlags`.

Запись `{ "source": "skills-dir" }` сохраняет [плагины skills-directory](#keep-skills-directory-plugins-loading) загружающимися под этим allowlist. Удалите эту запись и они перестанут загружаться.

Регистрируйте оба marketplace с явными записями `extraKnownMarketplaces`, как эта политика делает, а не полагаясь на allowlist или на официальный marketplace, регистрирующий себя:

* **Allowlist не регистрирует ничего**: запись `extraKnownMarketplaces` делает, и она сама должна пройти allowlist. Claude Code отказывает регистрировать управляемый marketplace, чей источник allowlist не совпадает.
* **Официальный marketplace регистрирует себя только в интерактивной сессии терминала**: даже там он регистрируется только, когда allowlist это разрешает. Запуск `-p` или терминал, подключенный к облачной сессии, никогда его не регистрирует.
* **Заблокированная попытка запоминается**: если машина когда-либо работала под политикой, которая блокировала официальный marketplace, Claude Code записывает заблокированную попытку и не повторяет попытку после изменения политики. Блокировка `[]` — это одна такая политика. Эта машина регистрирует его снова только через запись `extraKnownMarketplaces`, такую как в этой политике, запись `enabledPlugins` для одного из его плагинов или ручное `/plugin marketplace add`.

<h2 id="set-update-policy">
  Set update policy
</h2>

Вы можете установить политику обновления для каждого marketplace, для всего флота или для каждой группы пользователей через каналы выпуска.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Turn auto-update on or off per marketplace
</h3>

Автоматическое обновление плагина работает в фоновом режиме после запуска для marketplaces, у которых оно включено. Для того, какие marketplaces имеют его включенным по умолчанию, см. [Когда автоматическое обновление работает](/docs/ru/plugins/loading#when-auto-update-runs). Чтобы решить для флота, установите `"autoUpdate": true` или `false` на управляемой записи `extraKnownMarketplaces`:

* Если управляемая запись устанавливает поле, Claude Code отклоняет переключение `/plugin` пользователя с ошибкой, которая начинается с `Auto-update for '<name>' is set by`.
* Если управляемая запись оставляет поле неустановленным, переключение пользователя сохраняется.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Turn updates off for the whole fleet
</h3>

Чтобы отключить автоматическое обновление плагина для каждого marketplace, установите `DISABLE_AUTOUPDATER` в управляемом блоке `env`, как это делает этот пример. Та же переменная также останавливает собственные обновления Claude Code:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Чтобы остановить собственные обновления Claude Code, но сохранить автоматическое обновление плагина, добавьте `"FORCE_AUTOUPDATE_PLUGINS": "1"` в тот же блок. Другие [переменные окружения, которые останавливают автоматическое обновление плагина](/docs/ru/plugins/loading#when-auto-update-runs), работают так же.

`DISABLE_AUTOUPDATER` не охватывает плагины с источником [`command`](/docs/ru/plugins/marketplace-reference#command-plugin-source). Claude Code повторно запускает команду каждого включенного при каждой сессии и устанавливает вывод, когда он изменился. Для того, что останавливает эти запуски, см. [Когда источник команды повторно запускается](/docs/ru/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Assign release channels to user groups
</h3>

Чтобы запустить стабильные и ранние каналы доступа, разместите два marketplace, которые указывают на разные refs одних и тех же плагинов. Затем дайте каждой группе пользователей свой собственный marketplace через отдельные управляемые параметры конечной точки или политику шлюза. Управляемые параметры сервера из консоли администратора [применяются к каждому пользователю в вашей организации](/docs/ru/server-managed-settings#current-limitations), поэтому они не могут назначать разные параметры разным группам.

* Развертывайте отдельные [управляемые параметры конечной точки](/docs/ru/managed-settings#delivery-mechanisms), такие как файл управляемых параметров или профиль MDM, на устройства каждой группы. Чтобы проверить, применяется ли файл или профиль для каждой группы на устройстве, которое также имеет источник на уровне организации, см. [Как Claude Code объединяет управляемые источники](/docs/ru/managed-settings#precedence-within-the-managed-tier).
* Определите одну [политику шлюза Claude apps](/docs/ru/claude-apps-gateway-config#managed) для каждой группы. Шлюз применяет первую политику, чье правило совпадения подходит пользователю, поэтому упорядочите политики так, чтобы каждый пользователь достигал политики своей группы. Карта `extraKnownMarketplaces` этой политики не объединяется с картой любой другой политики, поэтому перечислите каждый marketplace, который группе нужен в ней, а не только ее marketplace канала.

С любым механизмом стабильная группа получает эту конфигурацию:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

Группа раннего доступа получает `latest-tools` вместо этого. Чтобы установить два marketplace, см. [Запустить каналы выпуска](/docs/ru/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Recommend plugins
</h2>

Владельцы Marketplace могут прикреплять сигналы `relevance` к записям, чтобы Claude Code предлагал плагин, когда проект совпадает.

Предложения из marketplace появляются только, когда он зарегистрирован на машине пользователя, вы перечисляете его имя в `pluginSuggestionMarketplaces` в управляемых параметрах, и вы объявляете его источник в той же политике. Объявите источник либо как запись `extraKnownMarketplaces` marketplace, либо как запись allowlist. Официальный marketplace требует только имя. См. [Включить предложения в управляемых параметрах](/docs/ru/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Audit and review
</h2>

События OpenTelemetry и Analytics API говорят вам, что ваш флот устанавливает и запускает.

Для того, что плагин может запустить на машине и что каждый уровень доверия разрешает, прочитайте [Безопасность плагина](/docs/ru/plugins/security) перед тем, как вы одобрите marketplace.

<h3 id="opentelemetry-events">
  OpenTelemetry events
</h3>

`claude_code.plugin_installed` записывает каждую установку, и `claude_code.plugin_loaded` записывает каждый включенный плагин при запуске сессии. Оба события редактируют или опускают имена плагинов и marketplace третьих сторон, если вы не установите `OTEL_LOG_TOOL_DETAILS=1`, как показано в [Редактированные имена плагинов в вашем бэкенде](/docs/ru/plugins/measure#redacted-plugin-names-in-your-backend). Списки полей находятся под [Событие установки плагина](/docs/ru/monitoring-usage#plugin-installed-event) и [Событие загрузки плагина](/docs/ru/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  Analytics API
</h3>

На плане Enterprise `GET /v1/organizations/analytics/plugins` возвращает подсчеты установки и вызова для каждого плагина, в день, по Claude Code и Cowork. Вы можете группировать подсчеты по пользователю или группе RBAC. Активность плагина, которая достигает Anthropic без имени плагина, появляется в одной совокупной строке `third-party`. См. [справочник конечной точки](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) и [Доступ к данным программно](/docs/ru/analytics#access-data-programmatically) для ключа, который ему нужен.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Plan for what managed settings can't enforce
</h2>

Эти запросы из проверок безопасности не имеют выделенного ключа в текущей схеме параметров. Ближайшие существующие элементы управления:

* **Per-user or per-group targeting**: каждый ключ плагина применяется к каждому пользователю, который получает параметры. Управляемые параметры сервера доставляют одну конфигурацию для каждой организации. Для политики для каждой группы используйте отдельные управляемые параметры конечной точки или политики шлюза, как под [Назначить каналы выпуска группам пользователей](#assign-release-channels-to-user-groups).
* **Restricting entries inside an allowed marketplace**: allowlist совпадает с источниками marketplace. Чтобы заблокировать один плагин из разрешенного marketplace, установите его на `false` в управляемом `enabledPlugins`.
* **Hiding `/plugin`**: нет ключа, который отключает команду. Ближайший эквивалент объединяет allowlist, называющий только ваш marketplace, управляемые записи `enabledPlugins` для плагинов, которые вы поставляете, и `disableSideloadFlags`.
* **Gating `--plugin-dir` through the allowlist**: allowlist не охватывает `--plugin-dir`. `disableSideloadFlags` делает.
* **Enforcing the claude.ai plugin toggles through these keys**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) не устанавливает ключи на этой странице. Что члены и ваша организация включают там достигает CLI как [синхронизированные плагины](/docs/ru/plugins/loading#synced-plugins), которые имеют свои собственные элементы управления.

<h2 id="troubleshoot-policy">
  Troubleshoot policy
</h2>

Если политика плагина не ведет себя как ожидается на машине, сначала проверьте эти симптомы:

* **The managed file didn't parse**: когда `managed-settings.json` не является действительным JSON, Claude Code отказывается запускаться и печатает [ошибку, называющую файл](/docs/ru/errors#managed-settings-document-could-not-be-parsed). Файл, который анализирует, но имеет одну недействительную запись, сохраняет остальную часть своей политики. См. [Недействительные записи в управляемых параметрах](/docs/ru/managed-settings#invalid-entries-in-managed-settings).
* **The managed source didn't load**: запустите `/status` и ищите `Enterprise managed settings` в строке `Setting sources`. Если его нет, источник не загрузился.
* **A user reports `blocked by enterprise policy`**: сообщение называет marketplace или его источник. Для allowlist он также перечисляет разрешенные источники. Записи, обращенные к пользователю, находятся на [Troubleshoot plugins](/docs/ru/plugins/troubleshooting).
* **A plugin the user disabled in `~/.claude/settings.json` still loads**: другой источник параметров повторно включил его, такой как управляемая запись `enabledPlugins`, которая принудительно включает его. `/plugin` и `claude plugin list` показывают `Disabled in ~/.claude/settings.json but still loads` с этим источником параметров.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/ru/plugins/marketplace-reference#marketplace-sources): значения `source`, которые `extraKnownMarketplaces`, `strictKnownMarketplaces` и `blockedMarketplaces` принимают
* [Host and maintain a marketplace](/docs/ru/plugins/host-marketplace): запустите marketplace, на который указывает ваша политика
* [Plugin security and trust](/docs/ru/plugins/security): что плагин может делать на машине и как просмотреть один перед установкой
* [Server-managed settings](/docs/ru/server-managed-settings): доставляйте эти ключи из консоли администратора claude.ai
* [Troubleshoot plugins](/docs/ru/plugins/troubleshooting#blocked-by-your-organization): сообщения, которые пользователи видят, когда политика их блокирует
