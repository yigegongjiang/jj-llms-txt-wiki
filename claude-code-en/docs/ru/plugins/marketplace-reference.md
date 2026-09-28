> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник Marketplace

> Полный справочник по полям marketplace.json, записям плагинов и объектам источников плагинов и marketplace, с указанием того, где каждый из них действителен.

`marketplace.json` — это файл, который определяет marketplace плагинов. Он содержит имя marketplace, его владельца и одну запись для каждого плагина. Источник плагина каждой записи указывает, откуда Claude Code загружает этот плагин.

Источник marketplace — это отдельный объект, который указывает, откуда Claude Code загружает сам файл marketplace. Вы пишете его в параметрах, или Claude Code создаёт его при запуске `claude plugin marketplace add`.

Этот справочник предназначен для разработчиков marketplace, которым нужно точное имя или значение поля, и для администраторов, которым нужно знать, какие значения `source` действительны в [`extraKnownMarketplaces`](/docs/ru/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/ru/settings-reference#strictknownmarketplaces) и [`blockedMarketplaces`](/docs/ru/plugins/org#restrict-what-users-can-install).

<Note>
  Эти случаи рассмотрены на других страницах:

  * **Создание или размещение marketplace**: см. [Создание marketplace](/docs/ru/plugins/create-marketplace) и [Размещение и поддержка marketplace](/docs/ru/plugins/host-marketplace)
  * **Рецепты списков разрешений и запретов**: см. [Управление плагинами для вашей организации](/docs/ru/plugins/org)
</Note>

Найдите раздел для того, что вы пишете или читаете:

* **Файл marketplace**: [Поля верхнего уровня](#top-level-fields) и [Записи плагинов](#plugin-entries)
* **`source` записи**: [Источники плагинов](#plugin-sources)
* **Объект `source` в параметрах**: [Источники marketplace](#marketplace-sources)
* **Вывод из [`claude plugin validate <path>`](/docs/ru/plugins/cli-reference)**: [Сообщения валидации](#validation-messages), которые сопоставляют каждое сообщение с полем, которое оно называет

<h2 id="marketplace-file">
  Файл marketplace
</h2>

Сохраняйте файл marketplace в `.claude-plugin/marketplace.json` в директории вашего marketplace. Если вы храните файл в другом месте репозитория, пользователи должны объявить marketplace в [`extraKnownMarketplaces`](/docs/ru/settings-reference#extraknownmarketplaces) с установленным `path` на его источник, потому что `claude plugin marketplace add` не имеет для этого опции.

Директория, которая содержит `.claude-plugin/`, называется корневой директорией marketplace, и каждый относительный источник плагина разрешается из неё, а не из `.claude-plugin/`.

Каждый пользователь регистрирует один marketplace на `name`, поэтому пользователь не может иметь два marketplace с одинаковым именем, зарегистрированные одновременно.

Claude Code игнорирует неизвестный ключ верхнего уровня или ключ записи плагина, а не отклоняет его, поэтому опечатка загружается молча. `claude plugin validate` сообщает о каждом неизвестном ключе как о предупреждении.

<h3 id="reserved-names">
  Зарезервированные имена
</h3>

Вы не можете дать вашему marketplace ни одно из следующих имён:

* **Имена официального marketplace**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins` и `claude-tag-plugins`. Зарезервированы, если только marketplace не поступает из источника `github` или `git` [marketplace source](#marketplace-sources) под `github.com/anthropics/`.
* **Имена community marketplace**: `claude-community`, `claude-plugins-community` и `healthcare`. Зарезервированы по тому же правилу, что и официальные имена.
* **Имена директории плагинов**: `anthropic-plugin-directory` и `claude-plugin-directory`. Зарезервированы по тому же правилу, что и официальные имена.
* **Имена, выдающие себя за официальный marketplace**: имена такие как `official-claude-plugins` или `claude-plugins-v2`, и любое имя, содержащее символ, отличный от ASCII. Ошибка: `Marketplace name impersonates an official Anthropic/Claude marketplace`. Управляющий символ или символ двунаправленного форматирования в имени также сообщает `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Другое написание зарезервированного имени**: имя, которое отличается от зарезервированного имени только конечной точкой, или символом, отличным от подчёркивания, вместо дефиса, поэтому `claude.code.plugins` считается `claude-code-plugins`. `claude plugin validate` принимает такое имя; добавление marketplace завершается ошибкой [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/ru/errors#marketplace-name-is-another-spelling-of-a-reserved-name), и marketplace, уже зарегистрированный под одним, перестаёт загружаться. Эта проверка требует Claude Code v2.1.280 или позже.
* **Имена, которые Claude Code использует для плагинов, которые не поступают из marketplace**: `inline` для плагинов, загруженных с [`--plugin-dir`](/docs/ru/cli-reference), `builtin` для встроенных плагинов, `skills-dir` для плагинов, автоматически загруженных из [`.claude/skills/`](/docs/ru/skills), и `synced` для плагинов, синхронизированных с вашего аккаунта claude.ai. `claude-plugin-test` также зарезервирован. `skills-dir` также появляется как `{"source": "skills-dir"}` в `strictKnownMarketplaces` и `blockedMarketplaces`, описанные в разделе [Source values valid only in policy lists](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github` и `gh`**: зарезервированы в любом регистре. Эта проверка требует Claude Code v2.1.275 или позже.
* **Имена, начинающиеся с `claudeai-`**: зарезервированы для marketplace, размещённых на claude.ai. `claude plugin marketplace add` отказывает любому другому marketplace, который использует один с `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`.

<h2 id="top-level-fields">
  Поля верхнего уровня
</h2>

Таблица перечисляет каждый ключ, который Claude Code читает из `marketplace.json`. `name`, `owner` и `plugins` являются обязательными.

| Поле                                       | Тип              | Описание                                                                                                                                                                                                                                                                      |
| :----------------------------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Идентификатор Marketplace. Без пробелов, управляющих символов или символов двунаправленного форматирования, без `/` или `\`, без `..` и не `.`. См. [Зарезервированные имена](#reserved-names). Пользователи вводят его после `@` при установке плагина                       |
| `owner`                                    | object           | Информация о разработчике. `name` обязателен; `email` и `url` опциональны                                                                                                                                                                                                     |
| `plugins`                                  | array            | [Записи плагинов](#plugin-entries). Каждая запись проверяется отдельно, поэтому одна неверная запись не приводит к отказу marketplace                                                                                                                                         |
| `$schema`                                  | string           | URL JSON Schema для автодополнения редактора. Игнорируется при загрузке                                                                                                                                                                                                       |
| `description`                              | string           | Описание Marketplace, показываемое пользователям. `claude plugin validate` предупреждает, когда оно отсутствует                                                                                                                                                               |
| `version`                                  | string           | Версия манифеста Marketplace                                                                                                                                                                                                                                                  |
| `metadata.description`, `metadata.version` | string           | Альтернативное местоположение для `description` и `version`                                                                                                                                                                                                                   |
| `metadata.pluginRoot`                      | string           | Каталог, под которым разрешаются имена источников плагинов без префикса. См. [Источник плагина с относительным путём](#relative-path-plugin-source). Требует Claude Code v2.1.239 или позже                                                                                   |
| `forceRemoveDeletedPlugins`                | boolean          | Когда `true`, плагин, который вы удаляете из `plugins`, удаляется на машинах пользователей. См. [Размещение и поддержка marketplace](/docs/ru/plugins/host-marketplace)                                                                                                            |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Имена marketplace, чьи плагины могут быть установлены как зависимости плагинов этого marketplace. При установке плагина применяется только список в собственном marketplace этого плагина для всей цепочки зависимостей. См. [Зависимости плагинов](/docs/ru/plugins/dependencies) |
| `renames`                                  | object           | Карта от бывшего `name` плагина к его текущему имени или к `null` для удалённого плагина. Требует Claude Code v2.1.193 или позже. См. [Размещение и поддержка marketplace](/docs/ru/plugins/host-marketplace)                                                                      |

<h2 id="plugin-entries">
  Записи плагинов
</h2>

Каждый объект в массиве `plugins` верхнего уровня `marketplace.json` называет плагин и указывает, откуда его загружать. `name` и `source` являются обязательными.

Запись также принимает каждое поле [`plugin.json`](/docs/ru/plugins/manifest-reference), такое как `description`, `version`, `author`, `commands` и `hooks`. Для того, когда эти поля применяются, см. [Как запись объединяется с plugin.json](#entry-and-plugin-json).

Таблица перечисляет собственные поля записи и поля манифеста, чьё значение изменяется в записи.

| Поле             | Тип              | Описание                                                                                                                                                                                                                                                                                                                               |
| :--------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | Идентификатор плагина без пробелов, управляющих символов или символов двунаправленного форматирования. Пользователи вводят его перед `@` при установке, даже когда собственный `plugin.json` плагина устанавливает другой `name`                                                                                                       |
| `source`         | string or object | Откуда загружать плагин. См. [Источники плагинов](#plugin-sources)                                                                                                                                                                                                                                                                     |
| `description`    | string           | Показывается в списках и деталях [`/plugin`](/docs/ru/plugins/install)                                                                                                                                                                                                                                                                      |
| `version`        | string           | Строка версии для плагина. Когда `plugin.json` также устанавливает `version`, `plugin.json` имеет приоритет и `claude plugin validate` предупреждает. См. [Справочник загрузки плагинов](/docs/ru/plugins/loading)                                                                                                                          |
| `category`       | string           | Свободная категория для организации каталога                                                                                                                                                                                                                                                                                           |
| `tags`           | array of strings | Свободные теги для поиска                                                                                                                                                                                                                                                                                                              |
| `strict`         | boolean          | По умолчанию `true`. Является ли `plugin.json` определяющим источником компонентов плагина. См. [Строгий режим](#strict-mode)                                                                                                                                                                                                          |
| `relevance`      | object           | Сигналы, которые говорят Claude Code, когда предложить плагин. См. [Рекомендация плагинов для вашей организации](/docs/ru/plugins/relevance)                                                                                                                                                                                                |
| `dependencies`   | array            | Плагины, которые должны быть включены, чтобы этот работал. Каждый элемент — это `"name"`, `"name@marketplace"` или объект. См. [Зависимости плагинов](/docs/ru/plugins/dependencies)                                                                                                                                                        |
| `defaultEnabled` | boolean          | По умолчанию `true`. Включен ли плагин по умолчанию, когда пользователь не установил его в [`enabledPlugins`](/docs/ru/settings-reference#enabledplugins). Значение записи имеет приоритет над `plugin.json`                                                                                                                                |
| `displayName`    | string           | Читаемое имя, показываемое в пользовательском интерфейсе. Когда ни запись, ни `plugin.json` плагина не устанавливают его, пользователи видят `name` плагина                                                                                                                                                                            |
| `metadata`       | object           | Свободный объект для ваших собственных полей. Claude Code не читает его. Требует Claude Code v2.1.222 или позже                                                                                                                                                                                                                        |
| `headers`        | object           | HTTP-заголовки, которые Claude Code отправляет при загрузке [архива](#archive-plugin-source) этой записи. Заголовок, установленный здесь, заменяет заголовок с тем же именем из [`headers`](#fields-by-type) источника marketplace. Требует Claude Code v2.1.238 или позже                                                             |
| `headersHelper`  | string           | Команда, которая выводит заголовки загрузки архива этой записи как один объект JSON для учётных данных, которые истекают. Запись также должна установить [`"strict": false`](#strict-mode). Требует Claude Code v2.1.238 или позже. См. [Аутентификация загрузок архивов](/docs/ru/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  Как запись объединяется с plugin.json
</h3>

Поля записи применяются по-разному к загруженному плагину, который имеет свой собственный `.claude-plugin/plugin.json`, и к тому, который его не имеет:

* **Нет `plugin.json`**: запись является манифестом независимо от `strict`. Каждое поле манифеста в записи применяется, включая [`mcpServers`, `lspServers`, `userConfig` и `channels`](/docs/ru/plugins/manifest-reference).
* **`plugin.json` присутствует**: `plugin.json` является манифестом. [Строгий режим](#strict-mode) решает, объединяются ли шесть полей компонентов записи, `commands`, `agents`, `skills`, `hooks`, `outputStyles` и `themes`, с ним или отклоняются как конфликт. Запись `mcpServers`, `lspServers`, `userConfig` и `channels` не применяются. Объявите их в `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks в записи
</h4>

Напишите запись `hooks` как встроенный объект, который сопоставляет имена событий hook с массивами сопоставителей. Если вы напишете путь к файлу или массив вместо этого, `claude plugin validate` пройдёт. Эти hooks никогда не запускаются, и Claude Code сообщает об ошибке `not yet supported in a marketplace entry` для плагина. Поместите hooks на основе файлов в собственный [`hooks/hooks.json`](/docs/ru/plugins/components) плагина или `plugin.json`.

<h4 id="display-fields">
  Поля отображения
</h4>

Как запись, так и собственный `plugin.json` плагина могут устанавливать поля отображения `displayName`, `description`, `author`, `homepage`, `repository`, `license` и `keywords`. Пользователи видят эти значения в списках и деталях плагинов до и после установки:

* Для поля, которое вы устанавливаете в записи, пользователи видят значение записи, даже когда `plugin.json` устанавливает другое.
* Для поля, которое запись оставляет неустановленным, пользователи видят значение `plugin.json`.

До установки Claude Code может читать `plugin.json` только для записей с [источником с относительным путём](#relative-path-plugin-source), чьи файлы плагинов находятся внутри самого marketplace. Для записи с любым другим типом источника пользователи видят только собственные поля записи до установки плагина.

<h3 id="strict-mode">
  Строгий режим
</h3>

`strict` решает, что происходит, когда загруженный плагин имеет свой собственный `plugin.json` и запись также объявляет любое из [полей компонентов](#entry-and-plugin-json): `commands`, `agents`, `skills`, `hooks`, `outputStyles` или `themes`. При `strict: true`, по умолчанию, Claude Code добавляет поля компонентов записи к `plugin.json`, кроме `hooks`, чьи сопоставители заменяют сопоставители манифеста на событие. При `strict: false`, запись, которая объявляет любое поле компонента, является конфликтом, и плагин не загружается. Таблица показывает каждую комбинацию `strict`, `plugin.json` и полей компонентов записи.

| `strict`             | `plugin.json` | Поля компонентов записи | Результат                                                                                                                                                                                                                                          |
| :------------------- | :------------ | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| любой                | отсутствует   | любой                   | Запись является манифестом                                                                                                                                                                                                                         |
| `true`, по умолчанию | присутствует  | любой                   | `plugin.json` является авторитетом. Claude Code добавляет поля компонентов записи к нему, кроме `hooks`, чьи сопоставители [заменяют сопоставители манифеста на событие](/docs/ru/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`              | присутствует  | нет                     | `plugin.json` является манифестом, как с `true`                                                                                                                                                                                                    |
| `false`              | присутствует  | один или более          | Конфликт. Плагин не загружается с `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                                             |

<h2 id="plugin-sources">
  Источники плагинов
</h2>

`source` записи плагина указывает, откуда Claude Code загружает этот плагин. Это либо строка относительного пути, либо объект, чей собственный ключ `source` называет тип, поэтому запись выглядит как `"source": { "source": "github", "repo": "your-org/formatter" }`.

Таблица перечисляет каждый тип источника плагина и его поля.

| Тип                | Поля                             | Примечания                                                                                                                                                                                                                            |
| :----------------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Относительный путь | сама строка                      | Каталог внутри marketplace, разрешённый от корня marketplace. Должен начинаться с `./`, если только вы не напишете [имя без префикса под `metadata.pluginRoot`](#relative-path-plugin-source). `"."` само по себе означает сам корень |
| `github`           | `repo`, `ref`, `sha`             | Репозиторий GitHub в форме `owner/repo`                                                                                                                                                                                               |
| `url`              | `url`, `ref`, `sha`              | Любой git-репозиторий по URL                                                                                                                                                                                                          |
| `git-subdir`       | `url`, `path`, `ref`, `sha`      | Один подкаталог git-репозитория, загруженный с разреженным частичным клоном                                                                                                                                                           |
| `npm`              | `package`, `version`, `registry` | npm-пакет, загруженный с вашим npm-клиентом и распакованный без запуска скриптов установки                                                                                                                                            |
| `archive`          | `url`, `sha256`                  | Zip-архив через HTTPS. Требует Claude Code v2.1.224 или позже                                                                                                                                                                         |
| `command`          | `command`, `timeout`, `mode`     | Каталог, выведенный командой, которую Claude Code запускает на машине пользователя. Требует Claude Code v2.1.229 или позже                                                                                                            |

Имена `url` и `github` также являются типами [источника marketplace](#marketplace-sources), где `url` означает прямую ссылку на файл `marketplace.json` вместо git-репозитория. `git` существует только как источник marketplace, и `npm` существует как оба. `git-subdir`, `archive` и `command` существуют только как источники плагинов.

Используйте относительный путь для плагина в подкаталоге самого репозитория marketplace. Используйте `git-subdir` для подкаталога какого-то другого репозитория.

Источники `github`, `url` и `git-subdir` совместно используют поля `ref` и `sha`:

* **`ref`**: ветка или тег. По умолчанию ветка репозитория по умолчанию.
* **`sha`**: полный 40-символьный SHA коммита в нижнем регистре. Когда вы устанавливаете оба `ref` и `sha`, Claude Code проверяет `sha`. На большинстве git-хостов, включая GitHub, GitLab и Bitbucket, это означает, что установка успешна, даже если ветка или тег, названные `ref`, были удалены выше по течению, пока коммит всё ещё достижим из репозитория. Некоторые серверы, такие как AWS CodeCommit, не поддерживают загрузку коммитов по SHA. На этих серверах `ref` всё ещё должен существовать и закреплённый коммит должен быть достижим из него.

Для того, как каждый тип загружается, кэшируется и версионируется, см. [Справочник загрузки плагинов](/docs/ru/plugins/loading).

<h3 id="relative-path-plugin-source">
  Источник плагина с относительным путём
</h3>

Путь разрешается от корня marketplace. `./plugins/formatter` — это `<root>/plugins/formatter`, даже хотя файл marketplace находится в `<root>/.claude-plugin/`.

Путь, содержащий `..`, не проходит валидацию. На macOS и Linux Claude Code отказывает в записи пути, содержащей обратную косую черту где-либо после начального `./`, поэтому напишите путь с прямыми косыми чертами.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Относительный путь разрешается только, когда Claude Code имеет файлы marketplace, поэтому проверьте тип [источника marketplace](#marketplace-sources):

* **`github`, `git`, `file` и `directory`**: Claude Code имеет файлы marketplace.
* **`url`**: Claude Code загружает только `marketplace.json`, поэтому относительные пути не могут разрешаться. Дайте каждому плагину источник объекта вместо этого, такой как `github` или `git-subdir`.
* **`settings`**: относительные пути отклоняются сразу.

<h4 id="bare-names-under-pluginroot">
  Имена без префикса под pluginRoot
</h4>

Имя без префикса — это одно имя каталога без `/`, такое как `"formatter"`. Чтобы писать имена без префикса вместо путей `./`, установите [`metadata.pluginRoot`](#top-level-fields) на каталог, под которым они разрешаются. С `"pluginRoot": "./plugins"`, `"source": "formatter"` разрешается в `./plugins/formatter`. Требует Claude Code v2.1.239 или позже.

`metadata.pluginRoot` имеет эти ограничения:

* Он сам должен быть относительным путём внутри marketplace.
* Он не влияет на источник, который уже начинается с `./`.
* Источник, содержащий `/`, такой как `team-a/formatter`, не является именем без префикса и всё ещё нуждается в префиксе `./`, даже когда установлен `metadata.pluginRoot`.

<h3 id="github-plugin-source">
  Источник плагина github
</h3>

`repo` принимает `owner/repo`. `ref` и `sha` опциональны.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  Источник плагина url
</h3>

`url` — это полный git URL: `https://`, `http://`, `file://` или `git@`. Суффикс `.git` не требуется, поэтому URL Azure DevOps и AWS CodeCommit работают как написано. Этот тип не принимает сокращение `owner/repo`.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  Источник плагина git-subdir
</h3>

`url` принимает полный git URL или сокращение GitHub `owner/repo`. `path` — это подкаталог, который содержит плагин, и Claude Code загружает только этот подкаталог.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  Источник плагина npm
</h3>

Источник `npm` принимает эти поля:

* `package`: имя пакета или имя с областью видимости, такое как `@your-org/formatter`
* `version`: версия или диапазон
* `registry`: URL реестра для пакета, который не находится в реестре по умолчанию

Claude Code загружает пакет с вашим npm-клиентом. Скрипты установки пакета, такие как `preinstall` или `postinstall`, никогда не запускаются, и его зависимости не устанавливаются во время загрузки. Если пакет имеет поддерживаемый файл блокировки рядом с его `package.json`, Claude Code устанавливает эти [зависимости пакетов Node.js](/docs/ru/plugins/loading#node-js-package-dependencies) в отдельном шаге, также с отключёнными скриптами.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  Источник плагина archive
</h3>

`url` должен использовать `https://` и не может указывать на хост обратной связи, локальной связи или облачных метаданных.

Корень плагина может быть в верхней части zip или на один каталог ниже.

`sha256` — это дайджест архива как 64 шестнадцатеричных символа, прописные или строчные. Когда вы его устанавливаете, Claude Code отказывает в загрузке, которая не совпадает.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  Источник плагина command
</h3>

Используйте источник `command`, когда инструмент, установленный на машине пользователя, производит каталог плагина, такой как IDE, которая отображает свой плагин для цепочки инструментов, которую выбрал пользователь. Claude Code запускает команду, когда пользователь устанавливает или обновляет плагин, и [снова один раз за сеанс](/docs/ru/plugins/loading#when-a-command-source-re-runs), поэтому пользователи получают изменённый вывод инструмента без переустановки.

Источник `command` принимает эти поля:

* `command`: команда оболочки, которая выводит абсолютный путь каталога плагина как одну строку и выходит 0. Claude Code показывает пользователям всю строку для проверки перед её запуском. Напишите её как печатаемый ASCII, максимум 500 символов, без прогона из четырёх или более пробелов.
* `timeout`: целое число секунд от 1 до 600. По умолчанию 60.
* `mode`: `copy`, по умолчанию, или `link`. См. [Режим копирования и режим ссылки](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Для того, как пользователи принимают команду, см. [Установка из вашей оболочки](/docs/ru/plugins/install#install-from-your-shell). Для того, что пользователи видят после того, как вы её измените, см. [Изменение команды источника command](/docs/ru/plugins/host-marketplace#change-the-command-of-a-command-source). Администраторы отключают источники command с [`disableCommandPluginSources`](/docs/ru/settings-reference#disablecommandpluginsources).

<h4 id="what-the-command-must-do">
  Что должна делать команда
</h4>

Напишите команду, чтобы соответствовать этим требованиям:

* **Оболочка и рабочий каталог**: Claude Code запускает команду через `sh` или через `cmd.exe` на Windows из домашнего каталога пользователя. Дайте абсолютный путь или команду на `PATH`.
* **Вывод**: выведите ровно одну строку на stdout, абсолютный путь каталога плагина, и выйдите 0 в течение `timeout` секунд.
* **Содержимое каталога**: каталог содержит полный плагин к моменту выхода команды. Путь может отличаться от одного запуска к другому.

<h4 id="output-that-fails-the-install-or-update">
  Вывод, который приводит к отказу установки или обновления
</h4>

Установка или обновление не удаётся, когда команда выходит с ненулевым кодом, работает дольше `timeout` или выводит что-либо, кроме одного абсолютного пути. Это также не удаётся, когда выведённый каталог является одним из этих:

* **Нет содержимого плагина**: выведённый каталог не имеет содержимого плагина на его верхнем уровне, такого как каталог `.claude-plugin/` или каталог `skills/`, `commands/`, `agents/` или `hooks/`.
* **Собственный каталог сеанса**: выведённый каталог — это тот, в котором был запущен Claude Code, или один из его родителей.
* **Сетевой путь**: на Windows выведённый путь — это путь UNC.
* **Слишком большой для копирования**: в режиме копирования каталог больше 256 МиБ или имеет более 20 000 записей.

<h4 id="copy-mode-and-link-mode">
  Режим копирования и режим ссылки
</h4>

`mode` решает, копирует ли Claude Code выведённый каталог или использует его на месте:

* **`copy`**: Claude Code копирует каталог в кэш плагина и выводит [версию плагина](/docs/ru/plugins/loading#how-claude-code-computes-the-version) из хэша скопированных файлов. Ваш инструмент может удалить или переписать каталог после выхода команды. Повторный запуск, который производит идентичные файлы, считается актуальным.
* **`link`**: Claude Code заполняет запись кэша плагина ссылкой на каждую запись верхнего уровня выведённого каталога и загружает файлы на месте. Ничего не копируется, содержимое файлов не хэшируется, и ограничения размера не применяются. Используйте его для каталога, слишком большого для копирования, такого как экспорт отображённого SDK.

Плагин в режиме ссылки имеет эти требования:

* **Держите каталог на месте**: Claude Code загружает плагин через ссылки при каждом запуске, поэтому выведённый каталог должен оставаться там, где он находится, пока плагин остаётся установленным.
* **Выведите другой путь, чтобы сигнализировать о новом содержимом**: версия поступает из реального пути выведённого каталога и его записей верхнего уровня, а не из файлов внутри них.
* **Держите символические ссылки верхнего уровня внутри каталога**: установка не удаётся, если запись верхнего уровня — это символическая ссылка, которая указывает вне выведённого каталога.
* **Включите `node_modules`**: Claude Code пропускает [установку зависимостей пакетов Node.js](/docs/ru/plugins/loading#node-js-package-dependencies) для плагина в режиме ссылки, поэтому выведите каталог, который уже содержит пакеты, которые нужны плагину.
* **Сеансы, запущенные внутри каталога**: сеанс, запущенный в выведённом каталоге или где-либо ниже, не загружает плагин.
* **Не на Windows**: Claude Code отказывает в установке плагина в режиме ссылки на Windows. Объявите `"mode": "copy"` там.

<h2 id="marketplace-sources">
  Источники Marketplace
</h2>

Источник marketplace указывает, откуда Claude Code загружает `marketplace.json`. CLI создаёт один для вас при добавлении marketplace, и вы пишете один сами в параметрах:

* **[`claude plugin marketplace add`](/docs/ru/plugins/cli-reference)**: Claude Code создаёт источник из строки, которую вы передаёте.
* **[`extraKnownMarketplaces`](/docs/ru/settings-reference#extraknownmarketplaces)**: вы пишете источник сами как объект `source`.
* **[`strictKnownMarketplaces`](/docs/ru/settings-reference#strictknownmarketplaces) и [`blockedMarketplaces`](/docs/ru/plugins/org#restrict-what-users-can-install)**: администраторы пишут источники в этих двух списках политик. `strictKnownMarketplaces` — это список разрешений, а `blockedMarketplaces` — это список запретов.

Имена типов `url`, `git` и `github` означают что-то другое в источнике marketplace, чем в [источнике плагина](#plugin-sources):

| Имя типа | Как источник marketplace                                                             | Как источник плагина                                            |
| :------- | :----------------------------------------------------------------------------------- | :-------------------------------------------------------------- |
| `url`    | Прямая ссылка на файл `marketplace.json` с полями `url`, `headers` и `headersHelper` | Git-репозиторий для клонирования с полями `url`, `ref` и `sha`  |
| `git`    | Git-репозиторий для клонирования с полями `url`, `ref`, `path` и `sparsePaths`       | Не существует                                                   |
| `github` | Репозиторий GitHub с полями `repo`, `ref`, `path` и `sparsePaths`                    | Репозиторий GitHub с полями `repo`, `ref` и `sha`, и без `path` |

Таблица перечисляет каждый тип источника marketplace с его полями, вводом `claude plugin marketplace add`, который его производит, и что он делает в каждом из трёх ключей параметров.

| Тип           | Поля                                 | Ввод `marketplace add`                                                                                                                                                | `extraKnownMarketplaces`                                      | `strictKnownMarketplaces`                                                                                                                                                                                                                              | `blockedMarketplaces`                                                   |
| :------------ | :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | URL `http://` или `https://`, который не совпадает с формой git                                                                                                       | Загружается                                                   | Разрешает тот же URL                                                                                                                                                                                                                                   | Блокирует тот же URL                                                    |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref` или `owner/repo#ref`                                                                                                                   | Загружается                                                   | Разрешает тот же `repo`, `ref` и `path`. `repo` может быть `owner/*`                                                                                                                                                                                   | Блокирует то же самое и URL `git` к тому же репозиторию                 |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | URL `user@host:path` или URL `https://`, который заканчивается на `.git`, содержит `/_git/` или называет репозиторий github.com или gitlab.com. `#ref` закрепляет ref | Загружается                                                   | Разрешает тот же URL, `ref` и `path`                                                                                                                                                                                                                   | Блокирует то же самое и другие написания того же репозитория github.com |
| `npm`         | `package`                            | Не производится                                                                                                                                                       | Не загружается: `NPM marketplace sources not yet implemented` | Анализирует, но ничего не совпадает, потому что ничего не регистрирует marketplace `npm`                                                                                                                                                               | Анализирует, но ничего не совпадает                                     |
| `file`        | `path`                               | Путь к файлу `.json`                                                                                                                                                  | Загружается                                                   | Разрешает тот же путь                                                                                                                                                                                                                                  | Блокирует тот же путь                                                   |
| `directory`   | `path`                               | Путь к каталогу                                                                                                                                                       | Загружается                                                   | Разрешает тот же путь                                                                                                                                                                                                                                  | Блокирует тот же путь                                                   |
| `settings`    | `name`, `plugins`, `owner`           | Не производится                                                                                                                                                       | Загружается                                                   | Разрешает запись с тем же `name` и идентичными `plugins`                                                                                                                                                                                               | Блокирует то же самое `name`                                            |
| `skills-dir`  | нет                                  | Не производится                                                                                                                                                       | Не загружается: `Unsupported marketplace source type`         | Держит [плагины каталога skills](/docs/ru/plugins/org#keep-skills-directory-plugins-loading) загруженными, пока установлен список разрешений. См. [Значения источников, действительные только в списках политик](#source-values-valid-only-in-policy-lists) | Останавливает загрузку плагинов каталога skills                         |
| `hostPattern` | `hostPattern`                        | Не производится                                                                                                                                                       | Не загружается: `Unsupported marketplace source type`         | Разрешает источники `github`, `git` и `url`, чей хост совпадает                                                                                                                                                                                        | Блокирует эти источники                                                 |
| `pathPattern` | `pathPattern`                        | Не производится                                                                                                                                                       | Не загружается: `Unsupported marketplace source type`         | Разрешает источники `file` и `directory`, чей `path` совпадает                                                                                                                                                                                         | Блокирует эти источники                                                 |

<h3 id="fields-by-type">
  Поля по типам
</h3>

Таблица перечисляет каждое поле источника marketplace, которое имеет значение по умолчанию, ограничение или значение, специфичное для его типа.

| Поле            | Типы            | Описание                                                                                                                                                                                                                                                                     |
| :-------------- | :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`           | `url`           | Ссылка на файл `marketplace.json`. Claude Code загружает только этот файл, поэтому плагины marketplace не могут использовать [источники с относительными путями](#relative-path-plugin-source)                                                                               |
| `url`           | `git`           | Git-репозиторий для клонирования                                                                                                                                                                                                                                             |
| `headers`       | `url`           | Карта HTTP-заголовков, которые Claude Code отправляет с загрузкой, для аутентифицированных хостов                                                                                                                                                                            |
| `headersHelper` | `url`           | Команда, которая выводит заголовки, чьи значения слишком недолговечны, чтобы их перечислять в `headers`. Требует Claude Code v2.1.238 или позже. См. [Аутентификация загрузок архивов](/docs/ru/plugins/host-marketplace#authenticate-archive-downloads)                          |
| `repo`          | `github`        | В `marketplace add` и `extraKnownMarketplaces`, `repo` должен называть один репозиторий. `marketplace add` отклоняет `owner/*` как неверное сокращение `owner/repo`; в `extraKnownMarketplaces` Claude Code берёт его буквально и клон не удаётся                            |
| `ref`           | `github`, `git` | Ветка или тег. По умолчанию ветка репозитория по умолчанию                                                                                                                                                                                                                   |
| `path`          | `github`, `git` | Путь файла marketplace внутри репозитория. По умолчанию `.claude-plugin/marketplace.json`                                                                                                                                                                                    |
| `path`          | `file`          | Сам файл marketplace. Claude Code читает его на месте и берёт каталог на два уровня выше как корень marketplace, поэтому держите файл в `<root>/.claude-plugin/marketplace.json`                                                                                             |
| `path`          | `directory`     | Корень marketplace, каталог, который содержит `.claude-plugin/marketplace.json`                                                                                                                                                                                              |
| `sparsePaths`   | `github`, `git` | Массив каталогов для разреженного извлечения, такой как `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` устанавливает его                                                                                                                          |
| `skipLfs`       | `github`, `git` | Принимается и не имеет эффекта. См. [Держите файлы плагинов вне Git LFS](/docs/ru/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                                      |
| `name`          | `settings`      | Должен равняться ключу `extraKnownMarketplaces` и не может быть [зарезервированным именем](#reserved-names)                                                                                                                                                                  |
| `plugins`       | `settings`      | Встроенный каталог без размещённого файла. Каждый элемент принимает `name`, `source`, `description`, `version`, `strict`, `headers` и `headersHelper`. Напишите `source` каждого элемента как тип объекта, потому что относительный путь не имеет репозитория для разрешения |

<h3 id="source-values-valid-only-in-policy-lists">
  Значения источников, действительные только в списках политик
</h3>

`hostPattern`, `pathPattern`, `skills-dir` и форма `owner/*` из `repo` действительны только в двух списках политик, `strictKnownMarketplaces` и `blockedMarketplaces`:

* **`hostPattern` и `pathPattern`**: регулярные выражения, которые Claude Code тестирует против источника перед загрузкой из него.
* **`skills-dir`**: не источник. Если вы вообще устанавливаете `strictKnownMarketplaces`, [плагины каталога skills](/docs/ru/plugins/org#keep-skills-directory-plugins-loading) перестают загружаться, пока вы не добавите `{"source": "skills-dir"}` в этот список.
* **`owner/*`**: как значение `repo` из `github`, совпадает с каждым репозиторием ровно под этим владельцем GitHub. Требует Claude Code v2.1.223 или позже.

Для порядка совпадения, точной семантики `ref` и рецептов см. [Управление плагинами для вашей организации](/docs/ru/plugins/org).

<h3 id="source-objects-in-settings">
  Объекты источников в параметрах
</h3>

Значение `extraKnownMarketplaces` — это карта от имени marketplace к объекту с `source`. Эта запись регистрирует marketplace из git-репозитория в его ветке `main`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` и `blockedMarketplaces` — это массивы объектов источников. Этот список разрешений допускает одного владельца GitHub и один внутренний хост:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Сообщения валидации
</h2>

`claude plugin validate <path>` принимает корень marketplace или сам файл marketplace. Он выводит ошибки и предупреждения. Для кодов выхода и `--strict` см. [plugin validate](/docs/ru/plugins/cli-reference#plugin-validate).

Сообщение называет запись плагина по её индексу, написанному как `plugins.1.source` или `plugins[1].source`.

Сообщение с префиксом индекса записи и `plugin.json →`, такое как `plugins[2] plugin.json →`, касается собственных файлов этого плагина. [`claude plugin validate` сообщает об ошибках](/docs/ru/plugins/troubleshooting#claude-plugin-validate-reports-errors) перечисляет эти сообщения с их исправлениями.

Предупреждения, которые упоминают имена флагов Claude Desktop, флаги, которые Claude Code принимает, но Claude Desktop отклоняет, потому что правила имён Claude Desktop более строгие.

Таблица сопоставляет сообщения уровня marketplace с полем, которое каждое касается.

| Сообщение                                                                                                                                                                                  | Уровень        | Поле                                                                                                                      |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `Marketplace must have a name`                                                                                                                                                             | Ошибка         | `name` пуст                                                                                                               |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                          | Ошибка         | `name`                                                                                                                    |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                      | Ошибка         | `name`                                                                                                                    |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                   | Ошибка         | `name`. См. [Зарезервированные имена](#reserved-names)                                                                    |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                           | Ошибка         | `name` содержит управляющий символ, такой как escape или новая строка, или символ двунаправленного форматирования Unicode |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, и варианты `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github` и `gh` | Ошибка         | `name`                                                                                                                    |
| `Author name cannot be empty`                                                                                                                                                              | Ошибка         | `owner.name`                                                                                                              |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                    | Ошибка         | `plugins[i].name`                                                                                                         |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                | Ошибка         | `plugins[i].name`                                                                                                         |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                           | Ошибка         | Две записи совместно используют `name`                                                                                    |
| `plugins.i.source: Invalid input`                                                                                                                                                          | Ошибка         | `source` записи не совпадает с типом. См. [Неверный ввод на источнике](#invalid-input-on-a-source)                        |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                            | Ошибка         | Относительный `source`, который выходит за пределы корня marketplace                                                      |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                          | Ошибка         | `plugins[i].source`                                                                                                       |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                 | Ошибка         | `plugins[i].headersHelper`, на записи `archive`                                                                           |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                        | Ошибка         | `renames.<old>`                                                                                                           |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                   | Ошибка         | `renames.<old>`                                                                                                           |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                  | Предупреждение | Названный ключ на верхнем уровне, под `metadata`, в записи или под `relevance` записи                                     |
| `Marketplace has no plugins defined`                                                                                                                                                       | Предупреждение | `plugins` пуст                                                                                                            |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                         | Предупреждение | `plugins[i].headers` или `plugins[i].headersHelper`, на записи, чей `source` не является `archive`                        |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                               | Предупреждение | `plugins[i].source.sha256`                                                                                                |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                 | Предупреждение | `plugins[i].headers.<name>`                                                                                               |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                       | Предупреждение | `plugins[i].source`                                                                                                       |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                            | Предупреждение | `description`                                                                                                             |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                            | Предупреждение | `plugins[i].version`, на записи с относительным путём                                                                     |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                 | Предупреждение | `plugins[i].relevance`                                                                                                    |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                      | Предупреждение | `plugins[i].metadata`                                                                                                     |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                         | Предупреждение | `plugins[i].experimental`                                                                                                 |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                       | Предупреждение | `name` — это `org`, `org-provisioned` или `unknown`. Claude Desktop отклоняет marketplace                                 |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                          | Предупреждение | `name`. Claude Desktop отклоняет marketplace                                                                              |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                               | Предупреждение | `plugins[i].name`. Claude Desktop отбрасывает запись                                                                      |

<h3 id="invalid-input-on-a-source">
  Неверный ввод на источнике
</h3>

`Invalid input` на `source` означает, что объект не совпадал с типом источника. Проверьте эти причины:

* Относительный путь, который не начинается с `./`, кроме `"."` или [имени без префикса под `metadata.pluginRoot`](#relative-path-plugin-source)
* `npm` `package`, содержащий `..`
* Тип `source`, который не является одним из [источников плагинов](#plugin-sources)
* Известный тип с отсутствующим обязательным полем или неправильного типа, такой как `github` без `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Сбои, которые валидация не ловит
</h3>

`claude plugin validate` не сообщает о каждом сбое. Запись `hooks`, написанная как путь к файлу или массив, проходит валидацию, и ошибка появляется только при загрузке плагина, как описывает [Hooks в записи](#hooks-in-an-entry). Ошибки загрузки `source` также появляются только после установки, а не при валидации.

[`claude plugin list`](/docs/ru/plugins/cli-reference) показывает плагин, который не загрузился, с его ошибкой, и [Устранение неполадок плагинов](/docs/ru/plugins/troubleshooting) охватывает строки загрузки.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Создание marketplace](/docs/ru/plugins/create-marketplace): создайте marketplace из этих полей и установите его локально
* [Размещение и поддержка marketplace](/docs/ru/plugins/host-marketplace): где поместить файл и как пользователи получают изменения
* [Справочник манифеста плагина](/docs/ru/plugins/manifest-reference): поля `plugin.json`, которые запись может переопределить
* [Управление плагинами для вашей организации](/docs/ru/plugins/org): рецепты списков разрешений и запретов, которые используют эти значения источников
