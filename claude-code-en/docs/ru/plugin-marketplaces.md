> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Создание и распространение marketplace плагинов

> Создавайте и размещайте marketplace плагинов для распространения расширений Claude Code по командам и сообществам.

**plugin marketplace** — это каталог, который позволяет вам распространять плагины другим пользователям. Marketplace обеспечивают централизованное обнаружение, отслеживание версий, автоматические обновления и поддержку нескольких типов источников, включая репозитории Git и локальные пути. Это руководство показывает, как создать собственный marketplace для совместного использования плагинов с вашей командой или сообществом.

Ищете способ установить плагины из существующего marketplace? См. [Обнаружение и установка готовых плагинов](/docs/ru/discover-plugins).

<h2 id="overview">
  Обзор
</h2>

Создание и распространение marketplace включает:

1. **Создание плагинов**: создайте один или несколько плагинов с skills, агентами, hooks, MCP servers или LSP servers. Это руководство предполагает, что у вас уже есть плагины для распространения; см. [Создание плагинов](/docs/ru/plugins) для получения подробной информации о том, как их создавать.
2. **Создание файла marketplace**: определите `marketplace.json`, который перечисляет ваши плагины и где их найти. См. [Создание файла marketplace](#create-the-marketplace-file).
3. **Размещение marketplace**: отправьте на GitHub, GitLab или другой хост Git. См. [Размещение и распространение marketplace](#host-and-distribute-marketplaces).
4. **Совместное использование с пользователями**: пользователи добавляют ваш marketplace с помощью `/plugin marketplace add` и устанавливают отдельные плагины. См. [Обнаружение и установка плагинов](/docs/ru/discover-plugins).

После того как ваш marketplace будет запущен, вы можете обновить его, отправив изменения в ваш репозиторий. Пользователи обновляют свою локальную копию с помощью `/plugin marketplace update`.

<h2 id="walkthrough-create-a-local-marketplace">
  Пошаговое руководство: создание локального marketplace
</h2>

Этот пример создает marketplace с одним плагином: skill `quality-review` для проверки кода. Вы создадите структуру каталогов, добавите skill, создадите манифест плагина и каталог marketplace, затем установите и протестируете его.

<Steps>
  <Step title="Создание структуры каталогов">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="Создание skill">
    Создайте файл `SKILL.md`, который определяет, что делает skill `quality-review`.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Review code for bugs, security, and performance
    ---

    Проверьте выбранный мной код или недавние изменения на предмет:
    - Потенциальных ошибок или граничных случаев
    - Проблем безопасности
    - Проблем производительности
    - Улучшений читаемости

    Будьте лаконичны и конкретны.
    ```
  </Step>

  <Step title="Создание манифеста плагина">
    Создайте файл `plugin.json`, который описывает плагин. Манифест находится в каталоге `.claude-plugin/`.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Adds a quality-review skill for quick code reviews",
      "version": "1.0.0"
    }
    ```

    <Note>
      Установка `version` означает, что пользователи получают обновления только при изменении этого поля, поэтому увеличивайте его при каждом выпуске. Если вы опустите `version` и разместите этот marketplace в git, каждый коммит автоматически считается новой версией. См. [Разрешение версий](#version-resolution-and-release-channels), чтобы выбрать правильный подход.
    </Note>
  </Step>

  <Step title="Создание файла marketplace">
    Создайте каталог marketplace, который перечисляет ваш плагин.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "Adds a quality-review skill for quick code reviews"
        }
      ]
    }
    ```
  </Step>

  <Step title="Добавление и установка">
    Добавьте marketplace и установите плагин.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="Попробуйте">
    Выберите некоторый код в вашем редакторе и запустите вашу новую skill. Plugin skills имеют пространство имен с именем плагина.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

Чтобы узнать больше о том, что могут делать плагины, включая hooks, агентов, MCP servers и LSP servers, см. [Плагины](/docs/ru/plugins).

<Note>
  **Как устанавливаются плагины**: Когда пользователи устанавливают плагин, Claude Code копирует каталог плагина в место кэша. Это означает, что плагины не могут ссылаться на файлы вне их каталога, используя пути вроде `../shared-utils`, потому что эти файлы не будут скопированы.

  Если вам нужно совместно использовать файлы между плагинами, используйте символические ссылки. См. [Кэширование плагинов и разрешение файлов](/docs/ru/plugins-reference#plugin-caching-and-file-resolution) для получения подробной информации.
</Note>

<h2 id="create-the-marketplace-file">
  Создание файла marketplace
</h2>

Создайте `.claude-plugin/marketplace.json` в корне вашего репозитория. Этот файл определяет имя вашего marketplace, информацию о владельце и список плагинов с их источниками.

Каждая запись плагина требует как минимум `name` и `source`, который указывает Claude Code, откуда его получить. См. [полную схему](#marketplace-schema) ниже для всех доступных полей.

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "Автоматическое форматирование кода при сохранении",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "Инструменты автоматизации развертывания"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Схема marketplace
</h2>

<h3 id="required-fields">
  Обязательные поля
</h3>

| Поле      | Тип    | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Пример         |
| :-------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `name`    | string | Идентификатор marketplace (kebab-case, без пробелов). Это общедоступное поле: пользователи видят его при установке плагинов (например, `/plugin install my-tool@your-marketplace`). Каждый пользователь может зарегистрировать только один marketplace с одним именем: добавление второго marketplace с тем же именем заменяет первый. Чтобы опубликовать несколько плагинов под одним именем marketplace, перечислите их все в [одном файле `marketplace.json`](#create-the-marketplace-file). | `"acme-tools"` |
| `owner`   | object | Информация о сопровождающем marketplace ([см. поля ниже](#owner-fields))                                                                                                                                                                                                                                                                                                                                                                                                                        |                |
| `plugins` | array  | Список доступных плагинов                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | См. ниже       |

<Note>
  **Зарезервированные имена**: следующие имена marketplace зарезервированы для официального использования Anthropic и не могут использоваться сторонними marketplace: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. Имена, которые выдают себя за официальные marketplace, такие как `official-claude-plugins` или `anthropic-plugins-v2`, также заблокированы. Резервирование этих имен предотвращает представление стороннего marketplace как источника, опубликованного Anthropic.

  Claude Code повторно проверяет зарезервированные имена каждый раз при загрузке marketplace, а не только при добавлении. Marketplace, зарегистрированный под одним из этих имен до того, как имя было зарезервировано, перестает загружаться и сообщает, что он [зарегистрирован из ненадежного источника](/docs/ru/errors#marketplace-is-registered-from-an-untrusted-source). Удалите этот marketplace и добавьте его снова из официального источника Anthropic. Сторонний marketplace, затронутый вновь зарезервированным именем, загружается снова, как только вы добавите его под другим именем. До версии v2.1.205 `first-party-plugins` и `healthcare` не были зарезервированы, и marketplace, уже зарегистрированный под зарезервированным именем, продолжал загружаться.
</Note>

<h3 id="owner-fields">
  Поля владельца
</h3>

| Поле    | Тип    | Обязательно | Описание                                           |
| :------ | :----- | :---------- | :------------------------------------------------- |
| `name`  | string | Да          | Имя сопровождающего или команды                    |
| `email` | string | Нет         | Контактный адрес электронной почты сопровождающего |

<h3 id="optional-fields">
  Дополнительные поля
</h3>

| Поле                                  | Тип    | Описание                                                                                                                                                                                                                                                                                                                              |
| :------------------------------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$schema`                             | string | URL JSON Schema для автодополнения редактора и валидации. Claude Code игнорирует это поле при загрузке.                                                                                                                                                                                                                               |
| `description`                         | string | Краткое описание marketplace                                                                                                                                                                                                                                                                                                          |
| `version`                             | string | Версия манифеста marketplace                                                                                                                                                                                                                                                                                                          |
| `metadata.pluginRoot`                 | string | Базовый каталог, добавляемый к относительным путям источников плагинов (например, `"./plugins"` позволяет вам писать `"source": "formatter"` вместо `"source": "./plugins/formatter"`)                                                                                                                                                |
| `allowCrossMarketplaceDependenciesOn` | array  | Другие marketplace, на которые плагины в этом marketplace могут зависеть. Зависимости от marketplace, не указанного здесь, блокируются при установке. См. [Зависимость от плагина из другого marketplace](/docs/ru/plugin-dependencies#depend-on-a-plugin-from-another-marketplace).                                                       |
| `renames`                             | object | Карта от прежнего имени плагина `name` к его текущему имени или к `null`, если плагин был удален. Позволяет существующим пользователям автоматически мигрировать при переименовании или удалении записи в `plugins`. См. [Переименование или удаление плагина](#rename-or-remove-a-plugin). Требуется Claude Code v2.1.193 или позже. |

`description` и `version` также принимаются в `metadata` для обратной совместимости.

<h2 id="plugin-entries">
  Записи плагинов
</h2>

Каждая запись плагина в массиве `plugins` описывает плагин и где его найти. Вы можете включить любое поле из [схемы манифеста плагина](/docs/ru/plugins-reference#plugin-manifest-schema), такие как `description`, `version`, `author`, `commands` и `hooks`, плюс эти поля, специфичные для marketplace: `source`, `category`, `tags`, `strict` и `relevance`.

<h3 id="required-fields-2">
  Обязательные поля
</h3>

| Поле     | Тип            | Описание                                                                                                                                                            |
| :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`   | string         | Идентификатор плагина (kebab-case, без пробелов). Это общедоступное поле: пользователи видят его при установке (например, `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | Откуда получить плагин (см. [Источники плагинов](#plugin-sources) ниже)                                                                                             |

<h3 id="optional-plugin-fields">
  Дополнительные поля плагина
</h3>

**Поля стандартных метаданных:**

| Поле             | Тип     | Описание                                                                                                                                                                                                                                                                                                                                          |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `displayName`    | string  | Удобочитаемое имя, отображаемое в интерфейсе. Возвращается к `name` при отсутствии. Может содержать пробелы и любой регистр. Не используется для пространства имён или поиска. Требуется Claude Code v2.1.143 или позже.                                                                                                                          |
| `description`    | string  | Краткое описание плагина                                                                                                                                                                                                                                                                                                                          |
| `version`        | string  | Версия плагина. Если установлено (здесь или в `plugin.json`), плагин закреплен на этой строке и пользователи получают обновления только при её изменении. Опустите, чтобы вернуться к SHA коммита git. См. [Разрешение версий](#version-resolution-and-release-channels).                                                                         |
| `author`         | object  | Информация об авторе плагина (`name` обязательно, `email` опционально)                                                                                                                                                                                                                                                                            |
| `homepage`       | string  | URL домашней страницы или документации плагина                                                                                                                                                                                                                                                                                                    |
| `repository`     | string  | URL репозитория исходного кода                                                                                                                                                                                                                                                                                                                    |
| `license`        | string  | Идентификатор лицензии SPDX (например, MIT, Apache-2.0)                                                                                                                                                                                                                                                                                           |
| `keywords`       | array   | Теги для обнаружения и категоризации плагинов                                                                                                                                                                                                                                                                                                     |
| `category`       | string  | Категория плагина для организации                                                                                                                                                                                                                                                                                                                 |
| `tags`           | array   | Теги для поиска                                                                                                                                                                                                                                                                                                                                   |
| `strict`         | boolean | Контролирует, является ли `plugin.json` авторитетом для определений компонентов (по умолчанию: true). См. [Strict mode](#strict-mode) ниже.                                                                                                                                                                                                       |
| `relevance`      | object  | Сигналы, которые сообщают Claude Code, когда предложить этот плагин пользователям. Вступает в силу только для marketplaces, которые администратор добавляет в список разрешённых в управляемых параметрах. См. [Рекомендовать плагины для вашей организации](/docs/ru/plugin-relevance). Требуется Claude Code v2.1.152 или позже.                     |
| `defaultEnabled` | boolean | Включен ли плагин после установки (по умолчанию: true). Установите значение `false`, чтобы установить плагин отключённым до тех пор, пока пользователь не согласится. Имеет приоритет над тем же полем в `plugin.json` плагина. См. [Включение по умолчанию](/docs/ru/plugins-reference#default-enablement). Требуется Claude Code v2.1.154 или позже. |

**Поля конфигурации компонентов:**

| Поле         | Тип            | Описание                                                               |
| :----------- | :------------- | :--------------------------------------------------------------------- |
| `skills`     | string\|array  | Пользовательские пути к каталогам skills, содержащим `<name>/SKILL.md` |
| `commands`   | string\|array  | Пользовательские пути к плоским файлам `.md` skills или каталогам      |
| `agents`     | string\|array  | Пользовательские пути к файлам агентов                                 |
| `hooks`      | string\|object | Конфигурация пользовательских hooks или путь к файлу hooks             |
| `mcpServers` | string\|object | Конфигурации MCP server или путь к конфигурации MCP                    |
| `lspServers` | string\|object | Конфигурации LSP server или путь к конфигурации LSP                    |

<h2 id="plugin-sources">
  Источники плагинов
</h2>

Источники плагинов указывают Claude Code, откуда получить каждый отдельный плагин, указанный в вашем marketplace. Они устанавливаются в поле `source` каждой записи плагина в `marketplace.json`.

После того как плагин клонирован или скопирован на локальную машину, он копируется в локальный кэш плагинов с версией в `~/.claude/plugins/cache`.

| Источник           | Тип                                  | Поля                               | Примечания                                                                                                                                         |
| ------------------ | ------------------------------------ | ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Относительный путь | `string` (например, `"./my-plugin"`) | none                               | Локальный каталог в репозитории marketplace. Должен начинаться с `./`. Разрешается относительно корня marketplace, а не каталога `.claude-plugin/` |
| `github`           | object                               | `repo`, `ref?`, `sha?`             |                                                                                                                                                    |
| `url`              | object                               | `url`, `ref?`, `sha?`              | Источник URL Git                                                                                                                                   |
| `git-subdir`       | object                               | `url`, `path`, `ref?`, `sha?`      | Подкаталог в репозитории Git. Клонирует разреженно, чтобы минимизировать пропускную способность для монорепозиториев                               |
| `npm`              | object                               | `package`, `version?`, `registry?` | Установлено через `npm install`                                                                                                                    |

<Note>
  **Источники marketplace и источники плагинов**: Это разные концепции, которые контролируют разные вещи.

  * **Источник marketplace** — откуда получить сам каталог `marketplace.json`. Устанавливается, когда пользователи запускают `/plugin marketplace add` или в параметрах `extraKnownMarketplaces`. Поддерживает `ref` (ветка/тег), но не `sha`.
  * **Источник плагина** — откуда получить отдельный плагин, указанный в marketplace. Устанавливается в поле `source` каждой записи плагина внутри `marketplace.json`. Поддерживает как `ref` (ветка/тег), так и `sha` (точный коммит).

  Например, marketplace, размещенный в `acme-corp/plugin-catalog` (источник marketplace), может перечислять плагин, полученный из `acme-corp/code-formatter` (источник плагина). Источник marketplace и источник плагина указывают на разные репозитории и закреплены независимо.
</Note>

Типы источников на основе Git ниже — это `github`, `url` и `git-subdir`. Когда оба `ref` и `sha` установлены на любом из них, `sha` является эффективным закреплением. Claude Code получает и проверяет закрепленный коммит напрямую.

На большинстве хостов Git, включая GitHub, GitLab и Bitbucket, это означает, что установка успешна даже если ветка или тег, названные `ref`, были удалены выше по течению, при условии, что коммит все еще доступен из репозитория. Некоторые серверы, такие как AWS CodeCommit, не поддерживают получение коммитов по SHA. На этих серверах `ref` все еще должен существовать и закрепленный коммит должен быть доступен из него.

<h3 id="relative-paths">
  Относительные пути
</h3>

Для плагинов в одном репозитории используйте путь, начинающийся с `./`:

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

Пути разрешаются относительно корня marketplace, который является каталогом, содержащим `.claude-plugin/`. В приведенном выше примере `./plugins/my-plugin` указывает на `<repo>/plugins/my-plugin`, даже если `marketplace.json` находится в `<repo>/.claude-plugin/marketplace.json`. Не используйте `../` для ссылки на пути вне корня marketplace.

<Note>
  Относительные пути разрешаются относительно локальной копии marketplace, поэтому они работают, когда пользователи добавляют ваш marketplace из источника Git или локального каталога. Если пользователи добавляют ваш marketplace через прямой URL к файлу `marketplace.json`, относительные пути не будут разрешены, потому что загружается только этот файл. Для распространения на основе URL используйте вместо этого источники GitHub, npm или URL Git. См. [Устранение неполадок](#plugins-with-relative-paths-fail-in-url-based-marketplaces) для получения подробной информации.
</Note>

<h3 id="github-repositories">
  Репозитории GitHub
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

Вы можете закрепить определенную ветку, тег или коммит:

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Поле   | Тип    | Описание                                                                           |
| :----- | :----- | :--------------------------------------------------------------------------------- |
| `repo` | string | Обязательно. Репозиторий GitHub в формате `owner/repo`                             |
| `ref`  | string | Опционально. Ветка или тег Git (по умолчанию ветка по умолчанию репозитория)       |
| `sha`  | string | Опционально. Полный 40-символьный SHA коммита Git для закрепления на точной версии |

<h3 id="git-repositories">
  Репозитории Git
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

Вы можете закрепить определенную ветку, тег или коммит:

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Поле  | Тип    | Описание                                                                                                                                                    |
| :---- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url` | string | Обязательно. Полный URL репозитория Git (`https://` или `git@`). Суффикс `.git` опционален, поэтому URL Azure DevOps и AWS CodeCommit без суффикса работают |
| `ref` | string | Опционально. Ветка или тег Git (по умолчанию ветка по умолчанию репозитория)                                                                                |
| `sha` | string | Опционально. Полный 40-символьный SHA коммита Git для закрепления на точной версии                                                                          |

<h3 id="git-subdirectories">
  Подкаталоги Git
</h3>

Используйте `git-subdir` для указания плагина, который находится в подкаталоге репозитория Git. Claude Code использует разреженный, частичный клон для получения только подкаталога, минимизируя пропускную способность для больших монорепозиториев.

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

Вы можете закрепить определенную ветку, тег или коммит:

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

Поле `url` также принимает сокращение GitHub (`owner/repo`) или SSH URL (`git@github.com:owner/repo.git`).

| Поле   | Тип    | Описание                                                                                           |
| :----- | :----- | :------------------------------------------------------------------------------------------------- |
| `url`  | string | Обязательно. URL репозитория Git, сокращение GitHub `owner/repo` или SSH URL                       |
| `path` | string | Обязательно. Путь подкаталога в репозитории, содержащий плагин (например, `"tools/claude-plugin"`) |
| `ref`  | string | Опционально. Ветка или тег Git (по умолчанию ветка по умолчанию репозитория)                       |
| `sha`  | string | Опционально. Полный 40-символьный SHA коммита Git для закрепления на точной версии                 |

<h3 id="npm-packages">
  Пакеты npm
</h3>

Плагины, распространяемые как пакеты npm, устанавливаются с помощью `npm install`. Это работает с любым пакетом в общедоступном реестре npm или в частном реестре, который размещает ваша команда.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

Чтобы закрепить определенную версию, добавьте поле `version`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

Для установки из частного или внутреннего реестра добавьте поле `registry`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| Поле       | Тип    | Описание                                                                                            |
| :--------- | :----- | :-------------------------------------------------------------------------------------------------- |
| `package`  | string | Обязательно. Имя пакета или область пакета (например, `@org/plugin`)                                |
| `version`  | string | Опционально. Версия или диапазон версий (например, `2.1.0`, `^2.0.0`, `~1.5.0`)                     |
| `registry` | string | Опционально. Пользовательский URL реестра npm. По умолчанию системный реестр npm (обычно npmjs.org) |

<h3 id="advanced-plugin-entries">
  Расширенные записи плагинов
</h3>

Этот пример показывает запись плагина, использующую множество дополнительных полей, включая пользовательские пути для команд, агентов, hooks и MCP servers:

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Инструменты автоматизации корпоративного рабочего процесса",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

Ключевые моменты, на которые следует обратить внимание:

* **`commands` и `agents`**: вы можете указать несколько каталогов или отдельные файлы. Пути относительны к корню плагина.
* **`${CLAUDE_PLUGIN_ROOT}`**: используйте эту переменную в командах hooks и конфигурациях MCP server для ссылки на файлы в каталоге установки плагина. Это необходимо, потому что плагины копируются в место кэша при установке.
  * См. [таблицу подстановки](/docs/ru/plugins-reference#environment-variables) для того, какие поля конфигурации подставляют её для каждого типа сервера
  * Для зависимостей или состояния, которое должно сохраняться при обновлениях плагина, используйте [`${CLAUDE_PLUGIN_DATA}`](/docs/ru/plugins-reference#persistent-data-directory) вместо этого
* **`strict: false`**: поскольку это установлено на false, плагину не нужен собственный `plugin.json`. Запись marketplace определяет все. См. [Strict mode](#strict-mode) ниже.

По умолчанию skills плагина загружаются из каталога `skills/` в его `source`. Пути, указанные в поле `skills`, добавляются к этому сканированию:

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

Когда несколько записей плагинов совместно используют один каталог `skills/` в корне marketplace (`source: "./"`), вместо этого указывайте конкретные подкаталоги, чтобы каждая запись загружала только свои собственные skills:

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

С источником в корне marketplace указанные пути являются полным набором для этой записи, и другие каталоги в общем каталоге `skills/` не загружаются. Указание самого `./skills/` или корня плагина сохраняет полное сканирование. Если ни один из указанных путей не существует, вместо этого запускается сканирование по умолчанию.

<h3 id="strict-mode">
  Strict mode
</h3>

Поле `strict` контролирует, является ли `plugin.json` авторитетом для определений компонентов (skills, агенты, hooks, MCP servers, стили вывода).

| Значение              | Поведение                                                                                                                                                   |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `true` (по умолчанию) | `plugin.json` является авторитетом. Запись marketplace может дополнить его дополнительными компонентами, и оба источника объединяются.                      |
| `false`               | Запись marketplace является полным определением. Если плагин также имеет `plugin.json`, который объявляет компоненты, это конфликт и плагин не загружается. |

**Когда использовать каждый режим:**

* **`strict: true`**: плагин имеет собственный `plugin.json` и управляет своими компонентами. Запись marketplace может добавить дополнительные skills или hooks сверху. Это значение по умолчанию и работает для большинства плагинов.
* **`strict: false`**: оператор marketplace хочет полный контроль. Репозиторий плагина предоставляет необработанные файлы, и запись marketplace определяет, какие из этих файлов открыты как skills, агенты, hooks и т. д. Полезно, когда оператор marketplace переструктурирует или курирует компоненты плагина иначе, чем предполагал автор плагина.

<h2 id="host-and-distribute-marketplaces">
  Размещение и распространение marketplace
</h2>

<h3 id="host-on-github-recommended">
  Размещение на GitHub (рекомендуется)
</h3>

GitHub обеспечивает рекомендуемый способ размещения и распространения marketplace:

1. **Создание репозитория**: установите новый репозиторий для вашего marketplace
2. **Добавление файла marketplace**: создайте `.claude-plugin/marketplace.json` с определениями ваших плагинов
3. **Совместное использование с командами**: пользователи добавляют ваш marketplace с помощью `/plugin marketplace add owner/repo`

**Преимущества**: встроенное управление версиями, отслеживание проблем и функции совместной работы команды.

<h3 id="host-on-other-git-services">
  Размещение на других сервисах Git
</h3>

Любой сервис хостинга Git работает, например GitLab, Bitbucket и самостоятельно размещаемые серверы. Пользователи добавляют с полным URL репозитория:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  Частные репозитории
</h3>

Claude Code поддерживает установку плагинов из частных репозиториев. Для ручной установки и обновлений Claude Code использует ваши существующие помощники учетных данных Git, поэтому доступ HTTPS через `gh auth login`, macOS Keychain или `git-credential-store` работает так же, как в вашем терминале. Доступ SSH работает, пока хост уже находится в вашем файле `known_hosts` и ключ загружен в `ssh-agent`, так как Claude Code подавляет интерактивные подсказки SSH для отпечатка хоста и пароля ключа. Сокращение GitHub `owner/repo` по умолчанию клонирует через SSH; установите [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/ru/env-vars#variables), чтобы вместо этого клонировать их через HTTPS.

Фоновые автоматические обновления работают иначе. По умолчанию фоновое обновление отключает помощников учетных данных Git для своего `git pull`, поэтому pull не может аутентифицироваться в частных репозиториях через HTTPS, даже если помощник настроен. Удаленные SSH не затронуты: ключ, загруженный в `ssh-agent`, аутентифицирует фоновые pull так же, как ручные операции. Когда фоновый pull не удается, Claude Code возвращается к повторному клонированию marketplace с нуля. Повторное клонирование использует ваши сохраненные учетные данные Git, но оно может [истечь на больших репозиториях](#git-operations-time-out), поэтому автоматические обновления частного marketplace могут периодически не удаваться.

Два параметра делают частные marketplace предсказуемыми:

* Установите `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, чтобы сохранить существующий клон при неудаче фонового pull, вместо удаления и повторного клонирования. Ваши плагины продолжают работать из последнего синхронизированного состояния, и ручные обновления с `/plugin marketplace update` по-прежнему pull с вашими учетными данными.
* Настройте помощника учетных данных Git, например с помощью `gh auth setup-git` для GitHub, чтобы fallback повторного клонирования мог аутентифицироваться без подсказок.

Установка токена поставщика, такого как `GITHUB_TOKEN`, в вашей среде не включает фоновую аутентификацию сама по себе. Токены вступают в силу только через настроенного помощника учетных данных, например помощника CLI `gh`, который читает `GH_TOKEN` и `GITHUB_TOKEN`.

Чтобы сам фоновый pull аутентифицировался через HTTPS, настройте глобальное переписывание URL Git. Переписывание встраивает токен в удаленный URL, поэтому оно вступает в силу, несмотря на то, что фоновый pull отключает помощников учетных данных, и успешный pull пропускает fallback повторного клонирования. Следующий пример переписывает URL репозитория marketplace, чтобы включить токен доступа:

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

Ограничьте переписывание путем репозитория marketplace или организации. Переписывание, чья база является только хостом, применяется к каждому fetch и push к этому хосту на машине и переопределяет ваши обычные учетные данные, включая push в ваши собственные репозитории.

Каждый поставщик ожидает другое имя пользователя в переписанном URL, и то же самое ограничение пути применяется к каждому поставщику. Для самостоятельно размещаемых серверов замените имя хоста на имя хоста вашего сервера:

| Поставщик | Форма переписанного URL                                           |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

Переписывание хранит токен в открытом виде в вашем gitconfig, поэтому используйте токен с доступом только для чтения к репозиторию marketplace.

<Note>
  В средах CI/CD настройте помощника учетных данных Git перед установкой плагинов из частных репозиториев. На GitHub Actions экспортируйте токен с доступом для чтения к репозиторию marketplace как `GH_TOKEN`, затем запустите `gh auth setup-git`. Токен рабочего процесса по умолчанию может получить доступ только к репозиторию самого рабочего процесса, поэтому частный marketplace в другом репозитории требует личного токена доступа или токена приложения. Глобальное переписывание URL, настроенное в конвейере, также аутентифицирует фоновый pull напрямую.
</Note>

<h3 id="test-locally-before-distribution">
  Тестирование локально перед распространением
</h3>

Протестируйте ваш marketplace локально перед совместным использованием:

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

Для полного диапазона команд добавления (GitHub, URL Git, локальные пути, удаленные URL), см. [Добавление marketplace](/docs/ru/discover-plugins#add-marketplaces).

<h3 id="require-marketplaces-for-your-team">
  Требование marketplace для вашей команды
</h3>

Вы можете настроить ваш репозиторий так, чтобы члены команды автоматически получали предложение установить ваш marketplace, когда они доверяют папке проекта. Добавьте ваш marketplace в `.claude/settings.json`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Вы также можете указать, какие плагины должны быть включены по умолчанию:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

Для полных параметров конфигурации см. [Параметры плагинов](/docs/ru/settings#plugin-settings).

<Note>
  Если вы используете локальный источник `directory` или `file` с относительным путем, путь разрешается относительно основного checkout вашего репозитория. Когда вы запускаете Claude Code из git worktree, путь все еще указывает на основной checkout, поэтому все worktrees совместно используют одно и то же расположение marketplace. Состояние marketplace хранится один раз для каждого пользователя в `~/.claude/plugins/known_marketplaces.json`, а не для каждого проекта.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  Предварительное заполнение плагинов для контейнеров
</h3>

Для образов контейнеров и сред CI вы можете предварительно заполнить каталог плагинов во время сборки, чтобы Claude Code запускался с уже доступными marketplace и плагинами, без клонирования во время выполнения. Установите переменную окружения `CLAUDE_CODE_PLUGIN_SEED_DIR` на этот каталог.

Чтобы наслоить несколько каталогов seed, разделите пути с `:` на Unix или `;` на Windows. Claude Code ищет каждый каталог по порядку и использует первый seed, содержащий данный marketplace или кэш плагина.

Каталог seed отражает структуру `~/.claude/plugins`:

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

Чтобы построить каталог seed, запустите Claude Code один раз во время сборки образа, установите нужные вам плагины, затем скопируйте полученный каталог `~/.claude/plugins` в ваш образ и укажите `CLAUDE_CODE_PLUGIN_SEED_DIR` на него.

Чтобы пропустить шаг копирования, установите `CLAUDE_CODE_PLUGIN_CACHE_DIR` на путь целевого seed во время сборки, чтобы плагины устанавливались непосредственно туда:

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

Затем установите `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` в среде выполнения вашего контейнера, чтобы Claude Code читал из seed при запуске.

При запуске Claude Code регистрирует marketplace, найденные в `known_marketplaces.json` seed, в основную конфигурацию и использует кэши плагинов, найденные под `cache/`, на месте без повторного клонирования. Это работает как в интерактивном режиме, так и в неинтерактивном режиме с флагом `-p`.

Детали поведения:

* **Только для чтения**: каталог seed никогда не записывается. Автоматические обновления отключены для seed marketplace, так как git pull не удастся на файловой системе только для чтения.
* **Записи seed имеют приоритет**: marketplace, объявленные в seed, перезаписывают любые совпадающие записи в конфигурации пользователя при каждом запуске. Чтобы отказаться от seed плагина, используйте `/plugin disable` вместо удаления marketplace.
* **Разрешение пути**: Claude Code находит содержимое marketplace, проверяя `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` во время выполнения, а не доверяя путям, хранящимся внутри JSON seed. Это означает, что seed работает правильно, даже если он смонтирован по другому пути, чем где он был построен.
* **Мутация заблокирована**: запуск `/plugin marketplace remove` или `/plugin marketplace update` против seed-управляемого marketplace не удается с указанием попросить вашего администратора обновить образ seed.
* **Компонуется с параметрами**: если `extraKnownMarketplaces` или `enabledPlugins` объявляют marketplace, который уже существует в seed, Claude Code использует копию seed вместо клонирования.

<h3 id="managed-marketplace-restrictions">
  Ограничения управляемого marketplace
</h3>

Для организаций, требующих строгого контроля над источниками плагинов, администраторы могут ограничить, какие marketplace плагинов пользователи могут добавлять, используя параметр [`strictKnownMarketplaces`](/docs/ru/settings#strictknownmarketplaces) в управляемых параметрах. Чтобы также отклонить флаги CLI, которые загружают плагины, агентов и MCP серверы для одного запуска, объедините его с [`disableSideloadFlags`](/docs/ru/settings#available-settings). Чтобы разрешить список, какие marketplace плагинов могут появляться как предложения контекстной установки, установите [`pluginSuggestionMarketplaces`](/docs/ru/settings#available-settings).

Когда `strictKnownMarketplaces` настроен в управляемых параметрах, поведение ограничения зависит от значения:

| Значение                     | Поведение                                                                                      |
| ---------------------------- | ---------------------------------------------------------------------------------------------- |
| Не определено (по умолчанию) | Нет ограничений. Пользователи могут добавлять любой marketplace                                |
| Пустой массив `[]`           | Полная блокировка. Пользователи не могут добавлять новые marketplace                           |
| Список источников            | Пользователи могут добавлять только marketplace, которые точно совпадают со списком разрешений |

<h4 id="common-configurations">
  Общие конфигурации
</h4>

Отключение всех добавлений marketplace:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

Разрешение только определенных marketplace:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

Разрешение всех marketplace с внутреннего сервера Git с использованием сопоставления шаблонов регулярных выражений на хосте. Это рекомендуемый подход для [GitHub Enterprise Server](/docs/ru/github-enterprise-server#plugin-marketplaces-on-ghes) или самостоятельно размещаемых экземпляров GitLab:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Разрешение marketplace на основе файловой системы из определенного каталога с использованием сопоставления шаблонов регулярных выражений на пути:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

Используйте `".*"` как `pathPattern` для разрешения любого пути файловой системы при одновременном контроле сетевых источников с помощью `hostPattern`.

<Note>
  `strictKnownMarketplaces` ограничивает то, что пользователи могут добавлять, но не регистрирует marketplace самостоятельно. Чтобы сделать разрешенные marketplace доступными автоматически без запуска пользователями `/plugin marketplace add`, объедините его с [`extraKnownMarketplaces`](/docs/ru/settings#extraknownmarketplaces) в одном файле `managed-settings.json`. См. [Использование обоих вместе](/docs/ru/settings#strictknownmarketplaces).
</Note>

<h4 id="how-restrictions-work">
  Как работают ограничения
</h4>

Ограничения проверяются перед любой сетевой или файловой операцией. Проверка выполняется при добавлении marketplace и при установке, обновлении, обновлении и автоматическом обновлении плагина. Если marketplace был добавлен до настройки политики и его источник больше не совпадает со списком разрешений, Claude Code отказывает в установке или обновлении плагинов из него. То же самое применяется к `blockedMarketplaces`.

Список разрешений использует точное сопоставление для большинства типов источников. Чтобы marketplace был разрешен, все указанные поля должны совпадать точно:

* Для источников GitHub: `repo` обязателен, и `ref` или `path` также должны совпадать, если указаны в списке разрешений
* Для источников URL: полный URL должен совпадать точно
* Для источников `hostPattern`: хост marketplace сопоставляется с шаблоном регулярного выражения
* Для источников `pathPattern`: путь файловой системы marketplace сопоставляется с шаблоном регулярного выражения

Точное сопоставление не нормализует URL: конечная косая черта, суффикс `.git` или форма `ssh://` в сравнении с `https://` рассматриваются как разные значения. Если marketplace вашей организации можно клонировать более чем одной формой URL, предпочтите запись `hostPattern` буквальному URL, чтобы все формы совпадали.

Поскольку `strictKnownMarketplaces` установлен в [управляемых параметрах](/docs/ru/settings#settings-files), отдельные пользователи и конфигурации проекта не могут переопределить эти ограничения.

Для полных деталей конфигурации, включая все поддерживаемые типы источников и сравнение с `extraKnownMarketplaces`, см. [справку strictKnownMarketplaces](/docs/ru/settings#strictknownmarketplaces).

<h3 id="version-resolution-and-release-channels">
  Разрешение версий и каналы выпуска
</h3>

Версии плагинов определяют пути кэша и обнаружение обновлений: если разрешенная версия совпадает с тем, что уже есть у пользователя, `/plugin update` и автоматическое обновление пропускают плагин.

Claude Code разрешает версию плагина из первого из этих параметров, который установлен:

1. `version` в `plugin.json` плагина
2. `version` в записи marketplace плагина
3. SHA коммита Git источника плагина

Для типов источников на основе Git `github`, `url`, `git-subdir` и относительных путей внутри marketplace, размещенного на Git, вы можете полностью опустить `version` и каждый новый коммит будет рассматриваться как новая версия. Это самая простая установка для внутренних или активно разрабатываемых плагинов.

<Warning>
  Установка `version` закрепляет плагин. Если `plugin.json` объявляет `"version": "1.0.0"`, отправка новых коммитов без изменения этой строки ничего не делает для существующих пользователей, потому что Claude Code видит ту же версию и сохраняет кэшированную копию. Увеличивайте поле при каждом выпуске или опустите его, чтобы использовать SHA коммита.

  Избегайте установки `version` одновременно в `plugin.json` и в записи marketplace. Значение `plugin.json` всегда побеждает молча, поэтому устаревшая версия манифеста может скрыть версию, которую вы установили в `marketplace.json`.
</Warning>

<h4 id="set-up-release-channels">
  Установка каналов выпуска
</h4>

Для поддержки каналов выпуска "stable" и "latest" для ваших плагинов вы можете установить два marketplace, которые указывают на разные refs или SHAs одного репозитория. Затем вы можете назначить два marketplace разным группам пользователей через [управляемые параметры](/docs/ru/settings#settings-files).

<Warning>
  Каждый канал должен разрешаться в другую версию. Если вы используете явные версии, `plugin.json` должен объявлять другую `version` в каждом закрепленном ref. Если вы опустите `version`, различные SHA коммитов уже различают каналы. Если два refs разрешаются в одну и ту же строку версии, Claude Code рассматривает их как идентичные и пропускает обновление.
</Warning>

<h5 id="example">
  Пример
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  Назначение каналов группам пользователей
</h5>

Назначьте каждый marketplace соответствующей группе пользователей через управляемые параметры. Например, стабильная группа получает:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

Группа ранних доступов получает вместо этого `latest-tools`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  Закрепление версий зависимостей плагинов
</h4>

Плагин может ограничить свои зависимости диапазоном semver, чтобы обновления зависимости не нарушили зависимый плагин. См. [Ограничение версий зависимостей плагинов](/docs/ru/plugin-dependencies) для соглашения о тегах Git `{plugin-name}--v{version}`, синтаксиса диапазона и того, как несколько ограничений на одну и ту же зависимость объединяются.

<h3 id="rename-or-remove-a-plugin">
  Переименование или удаление плагина
</h3>

`name` плагина является его стабильным идентификатором. Пользователи ссылаются на него в `enabledPlugins`, `pluginConfigs` и командах `/plugin install`, поэтому изменение его нарушает каждую существующую установку. Чтобы изменить метку, отображаемую в пользовательском интерфейсе, без нарушения установок, установите [`displayName`](#optional-plugin-fields) и оставьте `name` неизменным.

Если вы должны изменить `name` плагина или удалить плагин из массива `plugins`, добавьте запись `renames` верхнего уровня, чтобы существующие пользователи мигрировали вместо того, чтобы видеть ошибку `plugin-not-found`. Автоматическая миграция требует Claude Code v2.1.193 или позже. Сопоставьте каждое бывшее имя с его текущим именем или с `null`, если плагин больше не существует. Следующий пример переименовывает `formatter` в `code-formatter` и записывает, что `legacy-linter` был удален:

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Когда пользователь запускает Claude Code со старым именем все еще в своих параметрах, Claude Code следует карте `renames`:

* Если запись указывает на новое имя, Claude Code загружает плагин под его новым именем и показывает однострочное уведомление, такое как `Renamed to "code-formatter" in the "acme-tools" marketplace`. Затем он переписывает старый ключ на новый ключ в областях параметров пользователя, проекта и локальных параметров для обоих `enabledPlugins` и `pluginConfigs`, поэтому уведомление появляется один раз.
* Для записи `null` Claude Code удаляет старый ключ и уведомление сообщает, что плагин был удален из marketplace.
* Если переименованный плагин использует удаленный источник, такой как `github` или `npm`, Claude Code сообщает `plugin-cache-miss` после переименования и пользователь должен запустить `/plugin install` один раз, чтобы получить его под новым именем.

Рассматривайте `renames` как историю только для добавления: сохраняйте старые записи на месте даже после того, как вы ожидаете, что каждый пользователь мигрировал. Claude Code следует цепочкам, поэтому если вы позже переименуете `code-formatter` в `formatter-pro`, добавьте вторую запись вместо редактирования первой. Пользователь, который все еще имеет оригинальный `formatter` включенным, затем разрешается через обе записи в `formatter-pro`.

Запустите `claude plugin validate .` после редактирования карты; он отклоняет любую запись, цепочка которой образует цикл или не заканчивается на `null` или имя, указанное в `plugins`.

<Note>
  Управляемые и политические параметры доступны только для чтения для Claude Code, поэтому плагины, включенные там, не могут быть переписаны автоматически. Переименованный плагин все еще загружается каждый сеанс, но уведомление о переименовании повторяется до тех пор, пока администратор не обновит `enabledPlugins` в файле управляемых параметров, чтобы использовать новое имя. То же самое применяется к плагинам, включенным через другие источники только для чтения, такие как `--add-dir`.
</Note>

Более ранние версии Claude Code игнорируют поле `renames` и сообщают `plugin-not-found` для старого имени.

<h2 id="validation-and-testing">
  Валидация и тестирование
</h2>

Протестируйте ваш marketplace перед совместным использованием.

Проверьте синтаксис JSON вашего marketplace:

```bash theme={null}
claude plugin validate .
```

Или из Claude Code:

```shell theme={null}
/plugin validate .
```

Добавьте marketplace для тестирования:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

Установите тестовый плагин, чтобы проверить, что все работает:

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

Для полного диапазона рабочих процессов тестирования плагинов см. [Тестирование ваших плагинов локально](/docs/ru/plugins#test-your-plugins-locally). Для технического устранения неполадок см. [Справка плагинов](/docs/ru/plugins-reference).

<h2 id="manage-marketplaces-from-the-cli">
  Управление marketplace из CLI
</h2>

Claude Code предоставляет неинтерактивные подкоманды `claude plugin marketplace` для написания скриптов и автоматизации. Они эквивалентны командам `/plugin marketplace`, доступным в интерактивном сеансе.

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

Добавьте marketplace из репозитория GitHub, URL Git, удаленного URL или локального пути.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**Аргументы:**

* `<source>`: Сокращение GitHub `owner/repo`, URL Git, удаленный URL к файлу `marketplace.json` или путь локального каталога. Чтобы закрепить на ветке или теге, добавьте `@ref` к сокращению GitHub или `#ref` к URL Git

URL должен включать свою схему. Начиная с Claude Code v2.1.196, хост, введенный без схемы, такой как `gitlab.example.com/team/plugins`, отклоняется как недействительное сокращение `owner/repo`, и ошибка указывает вам добавить `https://` или использовать `./` для локального пути. Более ранние версии неправильно интерпретировали его как путь репозитория GitHub и не удаются при клонировании с ошибкой GitHub not-found.

**Параметры:**

| Параметр              | Описание                                                                                                                                    | По умолчанию |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ | :----------- |
| `--scope <scope>`     | Где объявить marketplace: `user`, `project` или `local`. См. [Области установки плагинов](/docs/ru/plugins-reference#plugin-installation-scopes) | `user`       |
| `--sparse <paths...>` | Ограничить checkout определенными каталогами через git sparse-checkout. Полезно для монорепозиториев                                        |              |

Добавьте marketplace из GitHub, используя сокращение `owner/repo`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

Закрепите на определенной ветке или теге с помощью `@ref`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

Добавьте из URL Git на хосте, отличном от GitHub:

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

Добавьте из удаленного URL, который служит файлом `marketplace.json` напрямую:

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

Добавьте из локального каталога для тестирования:

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

Объявите marketplace в области проекта, чтобы он был общим с вашей командой через `.claude/settings.json`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

Для монорепозитория ограничьте checkout каталогами, содержащими содержимое плагина:

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

Перечислите все настроенные marketplace.

```bash theme={null}
claude plugin marketplace list [options]
```

**Параметры:**

| Параметр | Описание         |
| :------- | :--------------- |
| `--json` | Вывести как JSON |

С помощью `--json` каждая запись включает `name`, `source` и поля, специфичные для источника: `repo` для источников GitHub, `url` для источников Git и URL, а также `path` для локальных источников. Источники GitHub и Git также включают поле `ref`, когда marketplace был добавлен с закрепленной веткой или тегом.

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

Удалите настроенный marketplace. Также принимается псевдоним `rm`.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**Аргументы:**

* `<name>`: имя marketplace для удаления, как показано в `claude plugin marketplace list`. Это `name` из `marketplace.json`, а не источник, который вы передали в `add`

**Параметры:**

| Параметр          | Описание                                                                                                                                                                                                                                                                                                                                                                                                           | По умолчанию  |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------ |
| `--scope <scope>` | Ограничить удаление одной областью параметров: `user`, `project` или `local`. См. [Области установки плагинов](/docs/ru/plugins-reference#plugin-installation-scopes). Если опущено, объявление удаляется из каждой редактируемой области. Если указано, удаляется только объявление этой области; общее состояние, кэш и установленные данные плагина сохраняются, когда marketplace все еще объявлен в другой области | (все области) |

<Warning>
  Удаление marketplace из его последней оставшейся области также удаляет все плагины, которые вы установили из него. Чтобы обновить marketplace без потери установленных плагинов, используйте `claude plugin marketplace update` вместо этого.
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

Обновите marketplace из их источников, чтобы получить новые плагины и изменения версий. Marketplace, добавленный с веткой или тегом `ref`, обновляется до последнего коммита этого ref, а не до ветки по умолчанию репозитория.

```bash theme={null}
claude plugin marketplace update [name]
```

**Аргументы:**

* `[name]`: имя marketplace для обновления, как показано в `claude plugin marketplace list`. Обновляет все marketplace, если опущено

Оба `remove` и `update` не удаются при запуске против seed-управляемого marketplace, который доступен только для чтения. При обновлении всех marketplace записи, управляемые seed, пропускаются, и другие marketplace все еще обновляются. Чтобы изменить плагины, предоставленные seed, попросите вашего администратора обновить образ seed. См. [Предварительное заполнение плагинов для контейнеров](#pre-populate-plugins-for-containers).

<h2 id="troubleshooting">
  Устранение неполадок
</h2>

<h3 id="marketplace-not-loading">
  Marketplace не загружается
</h3>

**Симптомы**: Не удается добавить marketplace или увидеть плагины из него

**Решения**:

* Проверьте, что URL marketplace доступен
* Убедитесь, что `.claude-plugin/marketplace.json` существует по указанному пути
* Убедитесь, что синтаксис JSON действителен, используя `claude plugin validate` или `/plugin validate`. Чтобы проверить frontmatter skill, agent и command, запустите команду для каждого каталога плагина
* Для частных репозиториев подтвердите, что у вас есть разрешения доступа

<h3 id="marketplace-validation-errors">
  Ошибки валидации marketplace
</h3>

Запустите `claude plugin validate .` или `/plugin validate .` из каталога вашего marketplace, чтобы проверить наличие проблем. Когда валидатор указывает на каталог marketplace, он проверяет `marketplace.json` на ошибки схемы, дублирующиеся имена плагинов и обход пути источника. Для каждой записи, чей `source` является локальным путем, он также валидирует собственный `plugin.json` этого плагина и предупреждает, когда `version` записи не совпадает с версией в `plugin.json`. Проблемы, найденные в `plugin.json` плагина, имеют префикс с индексом записи в форме `plugins[2] plugin.json →`.

Начиная с Claude Code v2.1.196, проверка для каждой записи также:

* включает плагины, чей `source` является `.`
* запускается, когда `marketplace.json` находится вне каталога `.claude-plugin`, разрешая источники относительно собственного каталога файла
* сообщает о проблемах каждой записи даже когда другая часть файла имеет ошибки схемы

Более ранние версии пропускают плагины в корне marketplace и спускаются только из `.claude-plugin/marketplace.json`.

Чтобы валидировать `plugin.json` отдельного плагина и его файлы skill, agent, command и hook, запустите команду для самого каталога плагина, например `claude plugin validate ./plugins/my-plugin`. Распространенные ошибки:

| Ошибка                                            | Причина                                        | Решение                                                                                                                                                  |
| :------------------------------------------------ | :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | Отсутствует манифест                           | Создайте `.claude-plugin/marketplace.json` с обязательными полями                                                                                        |
| `Invalid JSON syntax: Unexpected token...`        | Ошибка синтаксиса JSON в marketplace.json      | Проверьте отсутствующие запятые, лишние запятые или неквотированные строки                                                                               |
| `Duplicate plugin name "x" found in marketplace`  | Два плагина имеют одно имя                     | Дайте каждому плагину уникальное значение `name`                                                                                                         |
| `plugins[0].source: Path contains ".."`           | Путь источника содержит `..`                   | Используйте пути относительно корня marketplace без `..`. См. [Относительные пути](#relative-paths)                                                      |
| `YAML frontmatter failed to parse: ...`           | Неверный YAML в файле skill, agent или command | Исправьте синтаксис YAML в блоке frontmatter. Во время выполнения этот файл загружается без метаданных. Сообщается только при валидации каталога плагина |
| `Invalid JSON syntax: ...` (hooks.json)           | Неправильный формат `hooks/hooks.json`         | Исправьте синтаксис JSON. Неправильный `hooks/hooks.json` предотвращает загрузку всего плагина. Сообщается только при валидации каталога плагина         |

**Предупреждения** (не блокирующие):

* `Marketplace has no plugins defined`: добавьте хотя бы один плагин в массив `plugins`
* `No marketplace description provided`: добавьте описание верхнего уровня `description`, чтобы помочь пользователям понять ваш marketplace
* `Plugin name "x" is not kebab-case`: имя плагина содержит прописные буквы, пробелы или специальные символы. Переименуйте в строчные буквы, цифры и дефисы только (например, `my-plugin`). Claude Code принимает другие формы, но синхронизация marketplace claude.ai их отклоняет.

<h3 id="plugin-installation-failures">
  Ошибки установки плагина
</h3>

**Симптомы**: Marketplace появляется, но установка плагина не удается

**Решения**:

* Проверьте, что URL источников плагинов доступны
* Убедитесь, что каталоги плагинов содержат необходимые файлы
* Для источников GitHub убедитесь, что репозитории являются общедоступными или у вас есть доступ
* Протестируйте источники плагинов вручную, клонируя/загружая их
* Если источник закрепляет как `ref`, так и `sha`, удаленная ветвь или тег не блокируют установку на большинстве хостов git, включая GitHub, GitLab и Bitbucket. На серверах, которые не поддерживают получение коммитов по SHA, таких как AWS CodeCommit, `ref` все еще должен существовать и закрепленный коммит должен быть достижим из него. Если установка все еще не удается, подтвердите, что закрепленный коммит все еще существует в репозитории

<h3 id="private-repository-authentication-fails">
  Ошибка аутентификации частного репозитория
</h3>

**Симптомы**: Ошибки аутентификации при установке плагинов из частных репозиториев

**Решения**:

Для ручной установки и обновлений:

* Проверьте, что вы аутентифицированы у вашего поставщика Git (например, запустите `gh auth status` для GitHub)
* Проверьте, что ваш помощник учетных данных настроен правильно: `git config --global credential.helper`
* Попробуйте клонировать репозиторий вручную, чтобы проверить, что ваши учетные данные работают

Для фоновых автоматических обновлений:

* По умолчанию фоновые обновления отключают помощники учетных данных Git для pull, поэтому pull не может аутентифицироваться через HTTPS. SSH удаленные хосты с загруженным ключом в `ssh-agent` все еще аутентифицируются. Неудачный pull запускает повторное клонирование с нуля, которое использует ваши сохраненные учетные данные, но может истечь по времени на больших репозиториях
* Установите `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, чтобы сохранить существующий клон при сбое фонового pull
* Настройте помощник учетных данных Git, например `gh auth setup-git`, чтобы резервное повторное клонирование могло аутентифицироваться
* Если повторное клонирование истекает по времени на большом репозитории, увеличьте лимит с помощью [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)
* Настройте [переписывание URL Git](#private-repositories), ограниченное репозиторием marketplace, чтобы фоновый pull аутентифицировался напрямую
* Или обновляйте частные marketplace вручную с помощью `/plugin marketplace update <name>`, которая использует ваши учетные данные

<h3 id="marketplace-updates-fail-in-offline-environments">
  Обновления marketplace не работают в автономных средах
</h3>

**Симптомы**: Marketplace `git pull` не удается в фоне, и Claude Code многократно пытается повторно клонировать, что не может успешно завершиться.

**Причина**: По умолчанию, когда `git pull` не удается, Claude Code пытается повторно клонировать с нуля. В автономных или изолированных средах повторное клонирование не удается так же, и восстановление предыдущего кэша после этого является наилучшим усилием. Обновление запускается в фоне после запуска, поэтому оно не задерживает запуск, но каждая сессия повторяет неудачные попытки и каждая операция Git может ждать [120-секундного таймаута](#git-operations-time-out).

**Решение**: Установите `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, чтобы пропустить попытку повторного клонирования и продолжить использование существующего кэша при сбое pull:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

С этой переменной установленной, Claude Code сохраняет устаревший клон marketplace при сбое `git pull` и продолжает использовать последнее известное хорошее состояние. Для полностью автономных развертываний, где репозиторий никогда не будет доступен, используйте [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers) для предварительного заполнения каталога плагинов во время сборки вместо этого.

<h3 id="git-operations-time-out">
  Операции Git истекают по времени
</h3>

**Симптомы**: Установка плагина или обновление marketplace не удается с ошибкой истечения времени, например "Git clone timed out after 120s" или "Git pull timed out after 120s".

**Причина**: Claude Code использует 120-секундный таймаут для всех операций Git, включая клонирование репозиториев плагинов и извлечение обновлений marketplace. Большие репозитории или медленные сетевые соединения могут превысить этот лимит.

**Решение**: Увеличьте таймаут, используя переменную окружения `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`. Значение указывается в миллисекундах:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 минут
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Плагины с относительными путями не работают в marketplace на основе URL
</h3>

**Симптомы**: Добавлен marketplace через URL (например, `https://example.com/marketplace.json`), но плагины с источниками относительных путей, такие как `"./plugins/my-plugin"`, не устанавливаются с ошибками "path not found".

**Причина**: Marketplace на основе URL загружают только сам файл `marketplace.json`. Они не загружают файлы плагинов с сервера. Относительные пути в записи marketplace ссылаются на файлы на удаленном сервере, которые не были загружены.

**Решения**:

* **Используйте внешние источники**: Измените записи плагинов, чтобы использовать источники GitHub, npm или URL Git вместо относительных путей:
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Используйте marketplace на основе Git**: Разместите ваш marketplace в репозитории Git и добавьте его с URL Git. Marketplace на основе Git клонируют весь репозиторий, что делает относительные пути рабочими.

<h3 id="files-not-found-after-installation">
  Файлы не найдены после установки
</h3>

**Симптомы**: Плагин устанавливается, но ссылки на файлы не работают, особенно файлы вне каталога плагина

**Причина**: Плагины копируются в каталог кэша, а не используются на месте. Пути, которые ссылаются на файлы вне каталога плагина (например, `../shared-utils`), не будут работать, потому что эти файлы не копируются.

**Решения**: См. [Кэширование плагинов и разрешение файлов](/docs/ru/plugins-reference#plugin-caching-and-file-resolution) для обходных путей, включая символические ссылки и переструктурирование каталогов.

Для дополнительных инструментов отладки и распространенных проблем см. [Инструменты отладки и разработки](/docs/ru/plugins-reference#debugging-and-development-tools).

<h2 id="see-also">
  См. также
</h2>

* [Обнаружение и установка готовых плагинов](/docs/ru/discover-plugins) - Установка плагинов из существующих marketplace
* [Плагины](/docs/ru/plugins) - Создание собственных плагинов
* [Справка плагинов](/docs/ru/plugins-reference) - Полные технические спецификации и схемы
* [Параметры плагинов](/docs/ru/settings#plugin-settings) - Параметры конфигурации плагинов
* [Справка strictKnownMarketplaces](/docs/ru/settings#strictknownmarketplaces) - Ограничения управляемого marketplace
