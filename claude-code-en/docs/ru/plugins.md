> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Создание plugins

> Создавайте пользовательские plugins для расширения Claude Code с помощью skills, agents, hooks и MCP servers.

Plugins позволяют расширить Claude Code пользовательской функциональностью, которая может быть общей для проектов и команд. Это руководство охватывает создание собственных plugins с skills, agents, hooks и MCP servers.

Ищете установку существующих plugins? См. [Обнаружение и установка plugins](/docs/ru/discover-plugins). Для полных технических спецификаций см. [Справочник plugins](/docs/ru/plugins-reference).

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  Когда использовать plugins в сравнении с автономной конфигурацией
</h2>

Claude Code поддерживает два способа добавления пользовательских skills, agents и hooks:

| Подход                                                                                                       | Имена Skill          | Лучше всего для                                                                                                            |
| :----------------------------------------------------------------------------------------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| **Автономная** (директория `.claude/`)                                                                       | `/hello`             | Личные рабочие процессы, настройки для конкретного проекта, быстрые эксперименты                                           |
| **Plugins** (самостоятельные директории с skills, agents, hooks или манифестом `.claude-plugin/plugin.json`) | `/plugin-name:hello` | Совместное использование с коллегами, распространение в сообществе, версионные выпуски, повторное использование в проектах |

**Используйте автономную конфигурацию когда**:

* Вы настраиваете Claude Code для одного проекта
* Конфигурация личная и не требует совместного использования
* Вы экспериментируете с skills или hooks перед их упаковкой
* Вы хотите короткие имена skills, такие как `/hello` или `/deploy`

**Используйте plugins когда**:

* Вы хотите поделиться функциональностью с вашей командой или сообществом
* Вам нужны одинаковые skills/agents в нескольких проектах
* Вы хотите контроль версий и простые обновления для ваших расширений
* Вы распространяете через marketplace
* Вы согласны с пространством имён skills, такими как `/my-plugin:hello` (пространство имён предотвращает конфликты между plugins)

<Tip>
  Начните с автономной конфигурации в `.claude/` для быстрой итерации, затем [преобразуйте в plugin](#convert-existing-configurations-to-plugins) когда будете готовы поделиться.
</Tip>

<h2 id="quickstart">
  Быстрый старт
</h2>

Этот быстрый старт проведёт вас через создание plugin с пользовательским skill. Вы создадите манифест (файл конфигурации, который определяет ваш plugin), добавите skill и протестируете его локально, используя флаг `--plugin-dir`.

<h3 id="prerequisites">
  Предварительные требования
</h3>

* Claude Code [установлен и аутентифицирован](/docs/ru/quickstart#step-1-install-claude-code)

<Note>
  Если вы не видите команду `/plugin`, обновите Claude Code до последней версии. См. [Troubleshooting](/docs/ru/troubleshooting) для инструкций по обновлению.
</Note>

<h3 id="create-your-first-plugin">
  Создайте ваш первый plugin
</h3>

<Steps>
  <Step title="Создайте директорию plugin">
    Каждый plugin находится в собственной директории, содержащей ваши skills, agents или hooks, опционально вместе с манифестом `.claude-plugin/plugin.json`. Расположение не имеет значения для этого быстрого старта, потому что вы будете указывать Claude Code на директорию с помощью `--plugin-dir` на этапе тестирования. Создайте её в любом удобном месте, например в папке для черновиков или в директории проектов:

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    Остальные шаги выполняются из родительской директории и ссылаются на пути вроде `my-first-plugin/...` относительно неё.
  </Step>

  <Step title="Создайте манифест plugin">
    Файл манифеста в `.claude-plugin/plugin.json` определяет идентичность вашего plugin: его имя, описание и версию. Claude Code использует эти метаданные для отображения вашего plugin в менеджере plugins.

    Создайте директорию `.claude-plugin` внутри папки вашего plugin:

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    Затем создайте `my-first-plugin/.claude-plugin/plugin.json` с этим содержимым:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    | Поле          | Назначение                                                                                                                                                                                                                                                                                          |
    | :------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `name`        | Уникальный идентификатор и пространство имён skill. Skills имеют префикс этого (например, `/my-first-plugin:hello`).                                                                                                                                                                                |
    | `description` | Показывается в менеджере plugins при просмотре или установке plugins.                                                                                                                                                                                                                               |
    | `version`     | Опционально. Если установлено, пользователи получают обновления только когда вы увеличиваете это поле. Если опущено и ваш plugin распространяется через git, используется SHA коммита и каждый коммит считается новой версией. См. [управление версиями](/docs/ru/plugins-reference#version-management). |
    | `author`      | Опционально. Полезно для атрибуции.                                                                                                                                                                                                                                                                 |

    Для дополнительных полей, таких как `homepage`, `repository` и `license`, см. [полную схему манифеста](/docs/ru/plugins-reference#plugin-manifest-schema).
  </Step>

  <Step title="Добавьте skill">
    Skills находятся в директории `skills/`. Каждый skill — это папка, содержащая файл `SKILL.md`. Имя папки становится именем skill, с префиксом пространства имён plugin (`hello/` в plugin с именем `my-first-plugin` создаёт `/my-first-plugin:hello`).

    Создайте директорию skill в папке вашего plugin:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Затем создайте `my-first-plugin/skills/hello/SKILL.md` с этим содержимым:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="Протестируйте ваш plugin">
    Запустите Claude Code с флагом `--plugin-dir` для загрузки вашего plugin:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    После запуска Claude Code попробуйте ваш новый skill:

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Вы увидите, как Claude ответит приветствием. Запустите `/help` для просмотра вашего skill, указанного в пространстве имён plugin.

    <Note>
      **Почему пространство имён?** Skills plugin всегда имеют пространство имён (например, `/my-first-plugin:hello`) для предотвращения конфликтов, когда несколько plugins имеют skills с одинаковым именем.

      Чтобы изменить префикс пространства имён, обновите поле `name` в `plugin.json`.
    </Note>
  </Step>

  <Step title="Добавьте аргументы skill">
    Сделайте ваш skill динамичным, принимая пользовательский ввод. Заполнитель `$ARGUMENTS` захватывает любой текст, который пользователь предоставляет после имени skill.

    Обновите ваш файл `SKILL.md`:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    Запустите `/reload-plugins` для применения изменений, затем попробуйте skill с вашим именем:

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude поприветствует вас по имени. Для получения дополнительной информации о передаче аргументов в skills см. [Skills](/docs/ru/skills#pass-arguments-to-skills).
  </Step>
</Steps>

Вы успешно создали и протестировали plugin с этими ключевыми компонентами:

* **Манифест plugin** (`.claude-plugin/plugin.json`): описывает метаданные вашего plugin
* **Директория skills** (`skills/`): содержит ваши пользовательские skills
* **Аргументы skill** (`$ARGUMENTS`): захватывает пользовательский ввод для динамического поведения

<Tip>
  Флаг `--plugin-dir` полезен для разработки и тестирования. Когда вы будете готовы поделиться вашим plugin с другими, см. [Создание и распространение marketplace plugin](/docs/ru/plugin-marketplaces).
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  Разработка plugin в вашей директории skills
</h2>

Вместо передачи `--plugin-dir` при каждом запуске, вы можете сохранить plugin в вашей директории skills и позволить Claude Code загружать его автоматически. `claude plugin init` создаёт каркас:

```bash theme={null}
claude plugin init my-tool
```

Это создаёт `~/.claude/skills/my-tool/` с манифестом `.claude-plugin/plugin.json` и стартовым `SKILL.md`. На следующем сеансе он загружается как `my-tool@skills-dir` без marketplace или шага установки.

Для правил автозагрузки, личной и проектной области, требования доверия рабочей области и способов обновления или удаления, см. [Skills-directory plugins](/docs/ru/plugins-reference#skills-directory-plugins).

<h2 id="plugin-structure-overview">
  Обзор структуры plugin
</h2>

Вы создали plugin с skill, но plugins могут включать намного больше: пользовательские agents, hooks, MCP servers, LSP servers и фоновые мониторы.

<Warning>
  **Частая ошибка**: Не помещайте `commands/`, `agents/`, `skills/` или `hooks/` внутри директории `.claude-plugin/`. Только `plugin.json` находится внутри `.claude-plugin/`. Все остальные директории должны быть на уровне корня plugin.

  Корень plugin — это собственная директория отдельного plugin: та, которая содержит `.claude-plugin/plugin.json`. Это никогда не `~/.claude/`. Например, Claude Code не читает `.mcp.json`, размещённый в `~/.claude/.mcp.json`.
</Warning>

| Директория        | Местоположение | Назначение                                                                                            |
| :---------------- | :------------- | :---------------------------------------------------------------------------------------------------- |
| `.claude-plugin/` | Корень plugin  | Содержит манифест `plugin.json` (опционально, если компоненты используют местоположения по умолчанию) |
| `skills/`         | Корень plugin  | Skills как директории `<name>/SKILL.md`                                                               |
| `commands/`       | Корень plugin  | Skills как плоские файлы Markdown. Используйте `skills/` для новых plugins                            |
| `agents/`         | Корень plugin  | Определения пользовательских agents                                                                   |
| `hooks/`          | Корень plugin  | Обработчики событий в `hooks.json`                                                                    |
| `.mcp.json`       | Корень plugin  | Конфигурации MCP server                                                                               |
| `.lsp.json`       | Корень plugin  | Конфигурации LSP server для интеллекта кода                                                           |
| `monitors/`       | Корень plugin  | Конфигурации фонового монитора в `monitors.json`                                                      |
| `bin/`            | Корень plugin  | Исполняемые файлы, добавленные в `PATH` инструмента Bash во время включения plugin                    |
| `settings.json`   | Корень plugin  | Параметры по умолчанию [settings](/docs/ru/settings), применяемые при включении plugin                     |

Plugin, который поставляется ровно с одним skill, может поместить `SKILL.md` непосредственно в корень plugin вместо создания директории `skills/`. Claude Code загружает его как один skill и использует поле frontmatter `name` для имени вызова. Используйте макет `skills/` для plugins, которые могут расширяться до более чем одного skill.

<Note>
  **Следующие шаги**: Готовы добавить больше функций? Перейдите к [Разработка более сложных plugins](#develop-more-complex-plugins) для добавления agents, hooks, MCP servers и LSP servers. Для полных технических спецификаций всех компонентов plugin см. [Справочник plugins](/docs/ru/plugins-reference).
</Note>

<h2 id="develop-more-complex-plugins">
  Разработка более сложных plugins
</h2>

Когда вы будете комфортно чувствовать себя с базовыми plugins, вы сможете создавать более сложные расширения.

<h3 id="add-skills-to-your-plugin">
  Добавьте Skills в ваш plugin
</h3>

Plugins могут включать [Agent Skills](/docs/ru/skills) для расширения возможностей Claude. Skills вызываются моделью: Claude автоматически использует их на основе контекста задачи.

Добавьте директорию `skills/` в корень вашего plugin с папками Skill, содержащими файлы `SKILL.md`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

Каждый `SKILL.md` содержит YAML frontmatter и инструкции. Включите `description` чтобы Claude знал, когда использовать skill:

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

После установки plugin запустите `/reload-plugins` для загрузки Skills. Для полного руководства по созданию Skill, включая прогрессивное раскрытие и ограничения инструментов, см. [Agent Skills](/docs/ru/skills).

<h3 id="add-lsp-servers-to-your-plugin">
  Добавьте LSP servers в ваш plugin
</h3>

<Tip>
  Для распространённых языков, таких как TypeScript, Python и Rust, установите предварительно созданные LSP plugins из официального marketplace. Создавайте пользовательские LSP plugins только когда вам нужна поддержка языков, которые ещё не охвачены.
</Tip>

LSP (Language Server Protocol) plugins дают Claude интеллект кода в реальном времени. Если вам нужна поддержка языка, который не имеет официального LSP plugin, вы можете создать свой собственный, добавив файл `.lsp.json` в ваш plugin:

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Пользователи, устанавливающие ваш plugin, должны иметь двоичный файл языкового сервера, установленный на их машине.

Для полных опций конфигурации LSP см. [LSP servers](/docs/ru/plugins-reference#lsp-servers).

<h3 id="add-background-monitors-to-your-plugin">
  Добавьте фоновые мониторы в ваш plugin
</h3>

Фоновые мониторы позволяют вашему plugin отслеживать логи, файлы или внешний статус в фоне и уведомлять Claude по мере поступления событий. Claude Code автоматически запускает каждый монитор при активации plugin, поэтому вам не нужно инструктировать Claude запустить наблюдение.

Добавьте файл `monitors/monitors.json` в корень plugin с массивом записей монитора:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Каждая строка stdout из `command` доставляется Claude как уведомление во время сеанса. Для полной схемы, включая триггер `when` и подстановку переменных, см. [Monitors](/docs/ru/plugins-reference#monitors).

<h3 id="ship-default-settings-with-your-plugin">
  Поставляйте параметры по умолчанию с вашим plugin
</h3>

Plugins могут включать файл `settings.json` в корне plugin для применения конфигурации по умолчанию при включении plugin. В настоящее время поддерживаются только ключи `agent` и `subagentStatusLine`.

Установка `agent` активирует один из [пользовательских agents](/docs/ru/sub-agents) plugin в качестве основного потока, применяя его системный prompt, ограничения инструментов и модель. Это позволяет plugin изменить поведение Claude Code по умолчанию при включении.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Этот пример активирует agent `security-reviewer`, определённый в директории `agents/` plugin. Параметры из `settings.json` имеют приоритет над `settings`, объявленными в `plugin.json`. Неизвестные ключи молча игнорируются.

<h3 id="organize-complex-plugins">
  Организуйте сложные plugins
</h3>

Для plugins с множеством компонентов организуйте структуру вашей директории по функциональности. Для полных макетов директорий и шаблонов организации см. [Структура директории Plugin](/docs/ru/plugins-reference#plugin-directory-structure).

<h3 id="test-your-plugins-locally">
  Протестируйте ваши plugins локально
</h3>

Используйте флаг `--plugin-dir` для тестирования plugins во время разработки. Это загружает ваш plugin напрямую без необходимости установки.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

Флаг также принимает архив `.zip` директории plugin, для которого требуется Claude Code v2.1.128 или более поздняя версия.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

Когда `--plugin-dir` plugin имеет то же имя, что и установленный marketplace plugin, локальная копия имеет приоритет для этого сеанса. Это позволяет вам протестировать изменения plugin, который у вас уже установлен, без необходимости его предварительной деинсталляции. Исключением являются plugins, которые управляемые параметры принудительно включают или отключают: `--plugin-dir` не может переопределить эти параметры.

По мере внесения изменений в ваш plugin запустите `/reload-plugins` для применения обновлений без перезагрузки. Это перезагружает plugins, skills, agents, hooks, plugin MCP servers и plugin LSP servers. Протестируйте компоненты вашего plugin:

* Попробуйте ваши skills с `/plugin-name:skill-name`
* Проверьте, что agents появляются в `/context` под Custom Agents, или упомяните один по его scoped name с помощью @
* Убедитесь, что hooks работают как ожидается

<Tip>
  Вы можете загружать несколько plugins одновременно, указав флаг несколько раз:

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

Чтобы протестировать plugin, который уже упакован как архив `.zip` и размещён по URL-адресу, например артефакт сборки CI, используйте вместо этого `--plugin-url`. Claude Code загружает архив при запуске и загружает его только для этого сеанса. Если загрузка не удаётся или архив недействителен, Claude Code сообщает об ошибке загрузки plugin и запускается без него. Те же [соображения доверия](/docs/ru/discover-plugins#security) применяются как для любого источника plugin: указывайте этот флаг только на архивы, которыми вы управляете или которым доверяете.

Чтобы загружать несколько plugins, повторите флаг для каждого URL:

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

Или передайте разделённые пробелами URL-адреса как один аргумент в кавычках:

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  Отладка проблем plugin
</h3>

Если ваш plugin не работает как ожидается:

1. **Проверьте структуру**: Убедитесь, что ваши директории находятся в корне plugin, а не внутри `.claude-plugin/`
2. **Протестируйте компоненты отдельно**: Проверьте каждый skill, agent и hook отдельно
3. **Используйте инструменты валидации и отладки**: См. [Инструменты отладки и разработки](/docs/ru/plugins-reference#debugging-and-development-tools) для команд CLI и методов troubleshooting

<h3 id="share-your-plugins">
  Поделитесь вашими plugins
</h3>

Когда ваш plugin готов к совместному использованию:

1. **Добавьте документацию**: Включите `README.md` с инструкциями по установке и использованию
2. **Выберите стратегию версионирования**: Решите, устанавливать ли явную `version` или полагаться на SHA коммита git. См. [управление версиями](/docs/ru/plugins-reference#version-management)
3. **Создайте или используйте marketplace**: Распространяйте через [plugin marketplaces](/docs/ru/plugin-marketplaces) для установки
4. **Протестируйте с другими**: Попросите членов команды протестировать plugin перед более широким распространением

Когда ваш plugin находится в marketplace, другие могут установить его, используя инструкции в [Обнаружение и установка plugins](/docs/ru/discover-plugins). Чтобы сохранить plugin внутри вашей команды, разместите marketplace в [приватном репозитории](/docs/ru/plugin-marketplaces#private-repositories).

<h3 id="submit-your-plugin-to-the-community-marketplace">
  Отправьте ваш plugin на официальный marketplace сообщества
</h3>

Anthropic поддерживает два публичных marketplace для plugins Claude Code:

* **`claude-plugins-official`**: курируемый набор plugins, поддерживаемый Anthropic. Зарегистрирован автоматически при первом запуске Claude Code в интерактивном режиме. Неинтерактивный скрипт, который запускается перед этим первым запуском, должен добавить его явно с помощью `claude plugin marketplace add anthropics/claude-plugins-official`.
* **`claude-community`**: публичный marketplace сообщества, где размещаются сторонние отправки после проверки. Пользователи добавляют его с помощью `/plugin marketplace add anthropics/claude-plugins-community` и устанавливают из него как `@claude-community`.

Чтобы отправить ваш plugin на проверку в marketplace сообщества, используйте одну из встроенных форм:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Форма claude.ai требует организацию Team или Enterprise и доступ к управлению директорией; владельцы организации имеют этот доступ по умолчанию. Отдельные авторы, которые не являются частью организации Team или Enterprise, могут использовать форму Console вместо этого.

Запустите `claude plugin validate` локально перед отправкой. Конвейер проверки запускает ту же проверку для каждой отправки, а также автоматизированный скрининг безопасности.

Одобренные plugins закреплены на определённом коммите SHA в каталоге [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), и CI автоматически обновляет закрепление по мере того, как вы отправляете новые коммиты в ваш репозиторий. Публичный каталог синхронизируется ночью из конвейера проверки, поэтому может быть задержка между одобрением и появлением вашего plugin в `marketplace.json`. Чтобы проверить, установлен ли ваш plugin, выполните поиск его имени в [каталоге сообщества](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Официальный marketplace `claude-plugins-official` курируется отдельно. Anthropic решает, какие plugins включить по своему усмотрению. Нет процесса подачи заявки, и форма отправки не добавляет plugins в официальный marketplace.

Если Anthropic включит ваш plugin в официальный marketplace, ваш CLI может предложить пользователям Claude Code установить его. См. [Рекомендуйте ваш plugin из вашего CLI](/docs/ru/plugin-hints).

<Note>
  Для полных технических спецификаций, методов отладки и стратегий распространения см. [Справочник plugins](/docs/ru/plugins-reference).
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  Преобразование существующих конфигураций в plugins
</h2>

Если у вас уже есть skills или hooks в вашей директории `.claude/`, вы можете преобразовать их в plugin для более лёгкого совместного использования и распространения.

<h3 id="migration-steps">
  Шаги миграции
</h3>

<Steps>
  <Step title="Создайте структуру plugin">
    Создайте новую директорию plugin в корне вашего проекта, рядом с существующей папкой `.claude/`, чтобы относительные пути `cp` на следующем шаге разрешались:

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Создайте файл манифеста в `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Скопируйте ваши существующие файлы">
    Скопируйте ваши существующие конфигурации в директорию plugin:

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Мигрируйте hooks">
    Если у вас есть hooks в ваших параметрах, создайте директорию hooks:

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    Создайте `my-plugin/hooks/hooks.json` с конфигурацией вашего hooks. Скопируйте объект `hooks` из вашего `.claude/settings.json` или `settings.local.json`, так как формат одинаков. Команда получает входные данные hook как JSON на stdin, поэтому используйте `jq` для извлечения пути файла:

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Протестируйте ваш мигрированный plugin">
    Загрузите ваш plugin для проверки того, что всё работает:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Протестируйте каждый компонент: запустите ваши команды, проверьте, что agents появляются в `/context`, и убедитесь, что hooks срабатывают правильно.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  Что изменяется при миграции
</h3>

| Автономная (`.claude/`)                                     | Plugin                              |
| :---------------------------------------------------------- | :---------------------------------- |
| Доступна только в одном проекте                             | Может быть общей через marketplaces |
| Файлы в `.claude/commands/`                                 | Файлы в `plugin-name/commands/`     |
| Hooks в `settings.json`                                     | Hooks в `hooks/hooks.json`          |
| Необходимо вручную копировать для совместного использования | Установить с `/plugin install`      |

<Note>
  После миграции удалите исходные файлы из `.claude/` для избежания дубликатов. Определения project и user `.claude/agents/` переопределяют agents plugin с тем же именем, поэтому версия plugin вступает в силу только после удаления исходных файлов. Plugin skills имеют пространство имён как `/plugin-name:skill-name`, поэтому исходный `/skill-name` и копия plugin остаются доступными, а не один переопределяет другой.
</Note>

<h2 id="next-steps">
  Следующие шаги
</h2>

Теперь, когда вы понимаете систему plugins Claude Code, вот предлагаемые пути для различных целей:

<h3 id="for-plugin-users">
  Для пользователей plugin
</h3>

* [Обнаружение и установка plugins](/docs/ru/discover-plugins): просмотр marketplaces и установка plugins
* [Настройка team marketplaces](/docs/ru/discover-plugins#configure-team-marketplaces): установка plugins на уровне репозитория для вашей команды

<h3 id="for-plugin-developers">
  Для разработчиков plugin
</h3>

* [Создание и распространение marketplace](/docs/ru/plugin-marketplaces): упаковка и совместное использование ваших plugins
* [Справочник plugins](/docs/ru/plugins-reference): полные технические спецификации
* Углубитесь в конкретные компоненты plugin:
  * [Skills](/docs/ru/skills): детали разработки skill
  * [Subagents](/docs/ru/sub-agents): конфигурация и возможности agent
  * [Hooks](/docs/ru/hooks): обработка событий и автоматизация
  * [MCP](/docs/ru/mcp): интеграция внешних инструментов
