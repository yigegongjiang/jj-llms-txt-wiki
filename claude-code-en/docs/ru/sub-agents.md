> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Создание пользовательских subagents

> Создавайте и используйте специализированные AI subagents в Claude Code для рабочих процессов, ориентированных на конкретные задачи, и улучшенного управления контекстом.

Subagents — это специализированные AI-помощники, которые обрабатывают определённые типы задач. Используйте один, когда побочная задача заполнит основной разговор результатами поиска, логами или содержимым файлов, на которые вы больше не будете ссылаться: subagent выполняет эту работу в собственном контексте и возвращает только резюме. Определите пользовательский subagent, когда вы постоянно порождаете одного и того же рабочего с одинаковыми инструкциями.

Каждый subagent работает в собственном контекстном окне с пользовательским системным приглашением, специфическим доступом к инструментам и независимыми разрешениями. Когда Claude встречает задачу, соответствующую описанию subagent, он делегирует её этому subagent, который работает независимо и возвращает результаты. Чтобы увидеть экономию контекста на практике, [визуализация контекстного окна](/docs/ru/context-window) проходит через сессию, где subagent обрабатывает исследование в собственном отдельном окне.

<Note>
  Subagents работают в рамках одной сессии. Чтобы запустить множество независимых сессий параллельно и отслеживать их из одного места, см. [background agents](/docs/ru/agent-view). Для сессий, которые взаимодействуют друг с другом, см. [agent teams](/docs/ru/agent-teams).
</Note>

Subagents помогают вам:

* **Сохранять контекст**, отделяя исследование и реализацию от основного разговора
* **Применять ограничения**, ограничивая доступ subagent к определённым инструментам
* **Переиспользовать конфигурации** в проектах с помощью subagents уровня пользователя
* **Специализировать поведение** с помощью сфокусированных системных приглашений для конкретных областей
* **Контролировать затраты**, маршрутизируя задачи на более быстрые и дешёвые модели, такие как Haiku

Claude использует описание каждого subagent для решения о делегировании задач. Когда вы создаёте subagent, напишите чёткое описание, чтобы Claude знал, когда его использовать.

Claude Code включает несколько встроенных subagents, таких как Explore, Plan и general-purpose. Вы также можете создавать пользовательские subagents для обработки конкретных задач.

<h2 id="built-in-subagents">
  Встроенные subagents
</h2>

Claude Code включает встроенные subagents, которые Claude автоматически использует при необходимости. Каждый наследует разрешения родительского разговора с дополнительными ограничениями на инструменты.

Explore и Plan пропускают ваши файлы CLAUDE.md и статус git родительской сессии, чтобы исследование было быстрым и экономичным. Все остальные встроенные и [пользовательские subagents](#configure-subagents) загружают оба. Для полного разбора того, что достигает subagent, см. [что загружается при запуске](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Быстрый агент, доступный только для чтения, оптимизированный для поиска и анализа кодовых баз.

    * **Model**: наследуется из основного разговора, ограничен Opus на Claude API, поэтому Explore никогда не работает на более дорогой модели, чем та, которую вы уже выбрали для сессии
    * **Tools**: инструменты только для чтения; Write и Edit запрещены
    * **Purpose**: обнаружение файлов, поиск кода, исследование кодовой базы

    Начиная с версии 2.1.198, Explore наследует модель основного разговора вместо того, чтобы всегда работать на Haiku. На Claude API унаследованная модель ограничена Opus: основной разговор на более высоком уровне запускает Explore на Opus, а основной разговор на Sonnet или Haiku запускает Explore на той же модели. На любом другом поставщике, таком как [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или Claude Platform on AWS](/docs/ru/third-party-integrations), Explore наследует модель основного разговора напрямую.

    [Пользовательский или проектный subagent](#choose-the-subagent-scope) с именем `Explore` переопределяет встроенный и сохраняет собственное поле `model`, поэтому определите его с `model: haiku`, чтобы сохранить исследование на модели с более низкой стоимостью.

    Claude делегирует Explore, когда ему нужно искать или понимать кодовую базу без внесения изменений. Это сохраняет результаты исследования вне контекста основного разговора.

    При вызове Explore Claude указывает уровень тщательности: **quick** для целевых поисков, **medium** для сбалансированного исследования или **very thorough** для комплексного анализа.
  </Tab>

  <Tab title="Plan">
    Исследовательский агент, используемый во время [plan mode](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode) для сбора контекста перед представлением плана.

    * **Model**: наследуется из основного разговора
    * **Tools**: инструменты только для чтения; Write и Edit запрещены
    * **Purpose**: исследование кодовой базы для планирования

    Когда вы находитесь в режиме плана и Claude нужно понять вашу кодовую базу, он делегирует исследование subagent Plan, чтобы результаты исследования оставались в отдельном окне контекста, а основной разговор оставался доступным только для чтения.
  </Tab>

  <Tab title="General-purpose">
    Способный агент для сложных многошаговых задач, требующих как исследования, так и действия.

    * **Model**: наследуется из основного разговора
    * **Tools**: все инструменты
    * **Purpose**: сложное исследование, многошаговые операции, модификация кода

    Claude делегирует general-purpose, когда задача требует как исследования, так и модификации, сложного рассуждения для интерпретации результатов или нескольких зависимых шагов.
  </Tab>

  <Tab title="Other">
    Claude Code включает дополнительные вспомогательные агенты для конкретных задач. Обычно они вызываются автоматически, поэтому вам не нужно использовать их напрямую.

    | Agent             | Model  | Когда Claude его использует                                      |
    | :---------------- | :----- | :--------------------------------------------------------------- |
    | statusline-setup  | Sonnet | Когда вы запускаете `/statusline` для настройки строки состояния |
    | claude-code-guide | Haiku  | Когда вы задаёте вопросы о функциях Claude Code                  |
  </Tab>
</Tabs>

Встроенные subagents регистрируются по умолчанию в интерактивных сессиях. Чтобы ограничить их:

* Чтобы заблокировать определённый встроенный тип, добавьте его в `permissions.deny`, как показано в [Отключение определённых subagents](#disable-specific-subagents).
* Чтобы предотвратить делегирование Claude к любому subagent, запретите сам инструмент `Agent` с помощью [`permissions.deny`](/docs/ru/permissions#tool-specific-permission-rules).
* Чтобы удалить только встроенные subagents `Explore` и `Plan`, установите [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/ru/env-vars). Claude читает и исследует файлы напрямую вместо делегирования им. Требуется Claude Code версии 2.1.198 или позже.
* В [неинтерактивном режиме](/docs/ru/headless) и [Agent SDK](/docs/ru/agent-sdk/overview) установите [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ru/env-vars) для удаления всех встроенных типов и предоставления только ваших собственных.

Помимо этих встроенных subagents, вы можете создавать свои собственные с пользовательскими приглашениями, ограничениями инструментов, режимами разрешений, hooks и skills. В следующих разделах показано, как начать работу и настроить subagents.

<h2 id="quickstart-create-your-first-subagent">
  Quickstart: создание вашего первого subagent
</h2>

Subagents определяются в файлах Markdown с YAML frontmatter. Чтобы создать один, попросите Claude написать его для вас или [напишите файл самостоятельно](#write-subagent-files).

Начиная с версии v2.1.198, команда `/agents` больше не открывает интерактивный мастер создания; её запуск выводит напоминание попросить Claude или отредактировать `.claude/agents/` напрямую. Файлы subagent, поля frontmatter и местоположения `.claude/agents/` и `~/.claude/agents/` остаются неизменными; удалён только терминальный мастер.

Это пошаговое руководство создаёт subagent уровня пользователя, который проверяет код и предлагает улучшения.

<Steps>
  <Step title="Попросите Claude создать subagent">
    В Claude Code опишите subagent, который вы хотите создать, и где его сохранить:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude создаёт файл с `name`, `description`, списком `tools`, `model` и системным приглашением.
  </Step>

  <Step title="Проверьте файл">
    Откройте `~/.claude/agents/code-improver.md` и убедитесь, что frontmatter соответствует тому, что вы запросили. Результат выглядит так:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Поскольку файл находится в `~/.claude/agents/`, subagent доступен в каждом проекте на вашей машине. Чтобы ограничить его одним проектом, переместите его в каталог `.claude/agents/` этого проекта. [Выберите область действия subagent](#choose-the-subagent-scope) сравнивает эти два варианта.
  </Step>

  <Step title="Попробуйте">
    Попросите Claude делегировать новому subagent:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude делегирует вашему новому subagent, который сканирует кодовую базу и возвращает предложения по улучшению.

    Если Claude не может найти новый subagent, перезагрузите Claude Code и попробуйте снова. Это происходит только когда `~/.claude/agents/` не существовал до начала сеанса, потому что работающий сеанс не обнаруживает вновь созданный каталог `agents`.
  </Step>
</Steps>

Теперь у вас есть subagent, который вы можете использовать в любом проекте на вашей машине для анализа кодовых баз и предложения улучшений.

Вы также можете писать файлы subagent вручную, определять их через флаги CLI или распространять их через plugins. В следующих разделах рассматриваются все параметры конфигурации.

<Note>
  На Claude Code v2.1.197 и более ранних версиях `/agents` открывает интерактивный мастер с вкладкой **Running**, которая отображает активные subagents, и вкладкой **Library** для их создания, редактирования и удаления.&#x20;
</Note>

<h2 id="configure-subagents">
  Настройка subagents
</h2>

Местоположение файла subagent определяет, кому он доступен, а его frontmatter определяет, что он может делать. В этом разделе рассматривается, где находятся файлы subagent и каждое поле, которое они поддерживают.

<h3 id="choose-the-subagent-scope">
  Выберите область subagent
</h3>

Сохраняйте файлы subagent в разных местах в зависимости от области. Когда несколько subagents имеют одно и то же имя, Claude Code использует тот, который находится в местоположении с более высоким приоритетом.

| Location                    | Scope              | Priority       | Как создать                                       |
| :-------------------------- | :----------------- | :------------- | :------------------------------------------------ |
| Managed settings            | Организация        | 1 (наивысший)  | Развёрнуто через [managed settings](/docs/ru/settings) |
| `--agents` CLI flag         | Текущая сессия     | 2              | Передайте JSON при запуске Claude Code            |
| `.claude/agents/`           | Текущий проект     | 3              | Попросите Claude или создайте файл вручную        |
| `~/.claude/agents/`         | Все ваши проекты   | 4              | Попросите Claude или создайте файл вручную        |
| Директория `agents/` plugin | Где включен plugin | 5 (наименьший) | Установлено с [plugins](/docs/ru/plugins)              |

**Project subagents** (`.claude/agents/`) идеальны для subagents, специфичных для кодовой базы. Проверьте их в систему контроля версий, чтобы ваша команда могла использовать и улучшать их совместно.

Project subagents обнаруживаются путём прохода вверх от текущей рабочей директории, поэтому каждый `.claude/agents/` между ней и корнем репозитория сканируется. Начиная с версии 2.1.178, когда более одной из этих вложенных директорий определяет одно и то же `name`, Claude Code использует определение, ближайшее к рабочей директории.

Директории, добавленные с помощью `--add-dir`, также сканируются: папка `.claude/agents/` внутри добавленной директории загружается вместе с project subagents. См. [Additional directories](/docs/ru/permissions#additional-directories-grant-file-access-not-configuration) для того, какие другие типы конфигурации загружаются из `--add-dir`. Чтобы поделиться subagents в проектах без `--add-dir`, используйте `~/.claude/agents/` или [plugin](/docs/ru/plugins).

**User subagents** (`~/.claude/agents/`) — это личные subagents, доступные во всех ваших проектах.

Claude Code сканирует `.claude/agents/` и `~/.claude/agents/` рекурсивно, поэтому вы можете организовать определения в подпапки, такие как `agents/review/` или `agents/research/`. Путь подпапки не влияет на то, как идентифицируется или вызывается subagent, потому что идентичность происходит только из поля `name` frontmatter.

Сохраняйте значения `name` уникальными по всему дереву: если два файла под одной директорией `.claude/agents/`, включая её подпапки, объявляют одно и то же имя, Claude Code загружает только один из них, выбранный по порядку чтения файловой системы, а не по документированному приоритету. Во вложенных директориях проекта определение, ближайшее к рабочей директории, побеждает, как описано выше. Проверка настройки [`/doctor`](/docs/ru/commands#all-commands) сообщает о файлах в одной директории, которые имеют одно и то же имя, и предлагает переименовать или удалить все, кроме одного. До версии 2.1.205 `/doctor` открывал экран диагностики, который перечислял дубликаты и показывал, какое определение было активно.

Директории `agents/` plugin также сканируются рекурсивно. В отличие от областей проекта и пользователя, подпапка внутри директории `agents/` plugin становится частью [scoped identifier](#invoke-subagents-explicitly): файл в `agents/review/security.md` в plugin `my-plugin` регистрируется как `my-plugin:review:security`.

**CLI-определённые subagents** передаются как JSON при запуске Claude Code. Они существуют только для этой сессии и не сохраняются на диск, что делает их полезными для быстрого тестирования или скриптов автоматизации. Вы можете определить несколько subagents в одном вызове `--agents`:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

Флаг `--agents` принимает JSON с теми же полями [frontmatter](#supported-frontmatter-fields), что и файловые subagents: `description`, `prompt`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `isolation` и `color`. Используйте `prompt` для системного приглашения, эквивалентного телу markdown в файловых subagents.

**Managed subagents** развёртываются администраторами организации. Поместите файлы markdown в `.claude/agents/` внутри [директории managed settings](/docs/ru/settings#settings-files), используя тот же формат frontmatter, что и project и user subagents. Managed определения имеют приоритет над project и user subagents с тем же именем.

**Plugin subagents** поступают из [plugins](/docs/ru/plugins), которые вы установили. Они загружаются вместе с вашими пользовательскими subagents и появляются в typeahead @-упоминания под их scoped name. См. [справку по компонентам plugin](/docs/ru/plugins-reference#agents) для деталей создания plugin subagents.

<Note>
  По соображениям безопасности plugin subagents не поддерживают поля frontmatter `hooks`, `mcpServers` или `permissionMode`. Эти поля игнорируются при загрузке агентов из plugin. Если они вам нужны, скопируйте файл агента в `.claude/agents/` или `~/.claude/agents/`. Вы также можете добавить правила в [`permissions.allow`](/docs/ru/settings#permission-settings) в `settings.json` или `settings.local.json`, но эти правила применяются ко всей сессии, а не только к plugin subagent.
</Note>

Определения subagent из любой из этих областей также доступны для [agent teams](/docs/ru/agent-teams#use-subagent-definitions-for-teammates): при порождении товарища по команде вы можете ссылаться на тип subagent, и товарищ использует его `tools` и `model`, с телом определения добавленным в качестве дополнительных инструкций к системному приглашению товарища. См. [agent teams](/docs/ru/agent-teams#use-subagent-definitions-for-teammates) для того, какие поля frontmatter применяются на этом пути.

<h3 id="write-subagent-files">
  Напишите файлы subagent
</h3>

Файлы subagent используют YAML frontmatter для конфигурации, за которым следует системное приглашение в Markdown:

<Note>
  Claude Code наблюдает за `~/.claude/agents/` и `.claude/agents/`. Когда вы добавляете или редактируете файл subagent на диск, или просите Claude написать его для вас, Claude Code обнаруживает изменение в течение нескольких секунд и следующее делегирование использует обновленное определение без необходимости перезагрузки.

  Два случая по-прежнему требуют перезагрузки:

  * Наблюдатель охватывает только директории, которые существовали при запуске сессии, поэтому после создания первого файла агента области в новой директории `agents`, перезагрузитесь для его загрузки.
  * Сессии, запущенные с `--disable-slash-commands`, вообще не наблюдают эти директории.
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Frontmatter определяет метаданные и конфигурацию subagent. Тело становится системным приглашением, которое направляет поведение subagent. Subagents получают только это системное приглашение плюс базовые детали окружения, такие как рабочая директория, а не полное системное приглашение Claude Code.

В [non-interactive mode](/docs/ru/headless) флаг [`--append-subagent-system-prompt`](/docs/ru/cli-reference#cli-flags) добавляет текст, который вы предоставляете, в конец системного приглашения каждого subagent, включая вложенные subagents. Требует Claude Code v2.1.205 или позже.

Subagent начинает работу в текущей рабочей директории основного разговора. В пределах subagent команды `cd` не сохраняются между вызовами инструментов Bash или PowerShell и не влияют на рабочую директорию основного разговора. Чтобы дать subagent изолированную копию репозитория вместо этого, установите [`isolation: worktree`](#supported-frontmatter-fields).

Subagent с `isolation: worktree` запускает свои команды Bash и PowerShell внутри своего worktree. Команда, рабочая директория которой разрешается в вашу основную копию вместо этого, например потому что директория worktree была удалена во время работы subagent, завершается с ошибкой. До версии 2.1.203 такая команда могла запуститься в основной копии.

<h4 id="supported-frontmatter-fields">
  Поддерживаемые поля frontmatter
</h4>

Следующие поля могут использоваться в YAML frontmatter. Требуются только `name` и `description`.

| Field             | Требуется | Description                                                                                                                                                                                                                                                                                                                                                                |
| :---------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Да        | Уникальный идентификатор, использующий строчные буквы и дефисы. [Hooks](/docs/ru/hooks#subagentstart) получают это значение как `agent_type`. Имя файла не должно совпадать                                                                                                                                                                                                     |
| `description`     | Да        | Когда Claude должен делегировать этому subagent                                                                                                                                                                                                                                                                                                                            |
| `tools`           | Нет       | [Инструменты](#available-tools), которые может использовать subagent. Наследует все инструменты, если опущено. Если ни один элемент в списке не разрешается в инструмент, subagent не запускается с ошибкой, называющей элементы. Чтобы предварительно загрузить Skills в контекст, используйте поле `skills` вместо перечисления `Skill` здесь                            |
| `disallowedTools` | Нет       | Инструменты для запрета, удалённые из унаследованного или указанного списка                                                                                                                                                                                                                                                                                                |
| `model`           | Нет       | [Модель](#choose-a-model) для использования: `sonnet`, `opus`, `haiku`, `fable`, полный ID модели (например, `claude-opus-4-8`), или `inherit`. По умолчанию `inherit`                                                                                                                                                                                                     |
| `permissionMode`  | Нет       | [Режим разрешений](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, или `manual` как псевдоним для `default`. Псевдоним `manual` требует Claude Code v2.1.200 или позже. Игнорируется для [plugin subagents](#choose-the-subagent-scope)                                                                                      |
| `maxTurns`        | Нет       | Максимальное количество агентских ходов перед остановкой subagent                                                                                                                                                                                                                                                                                                          |
| `skills`          | Нет       | [Skills](/docs/ru/skills) для предварительной загрузки в контекст subagent при запуске. Полное содержимое skill инжектируется, а не просто описание. Subagents по-прежнему могут вызывать неперечисленные project, user и plugin skills через инструмент Skill                                                                                                                  |
| `mcpServers`      | Нет       | [MCP servers](/docs/ru/mcp) доступные этому subagent. Каждая запись — это либо имя сервера, ссылающееся на уже настроенный сервер (например, `"slack"`), либо встроенное определение с именем сервера в качестве ключа и полной [конфигурацией MCP server](/docs/ru/mcp#installing-mcp-servers) в качестве значения. Игнорируется для [plugin subagents](#choose-the-subagent-scope) |
| `hooks`           | Нет       | [Lifecycle hooks](#define-hooks-for-subagents) в области этого subagent. Игнорируется для [plugin subagents](#choose-the-subagent-scope)                                                                                                                                                                                                                                   |
| `memory`          | Нет       | [Область постоянной памяти](#enable-persistent-memory): `user`, `project` или `local`. Включает кросс-сессионное обучение                                                                                                                                                                                                                                                  |
| `background`      | Нет       | Установите на `true`, чтобы всегда запускать этот subagent как [фоновую задачу](#run-subagents-in-foreground-or-background), даже когда Claude нуждается в его результате прямо сейчас. Если не установлено, Claude выбирает, и начиная с версии 2.1.198 он запускает subagents в фоне по умолчанию                                                                        |
| `effort`          | Нет       | Уровень усилий, когда этот subagent активен. Переопределяет уровень усилий сессии. По умолчанию: наследуется из сессии. Параметры: `low`, `medium`, `high`, `xhigh`, `max`; доступные уровни зависят от модели                                                                                                                                                             |
| `isolation`       | Нет       | Установите на `worktree`, чтобы запустить subagent во временном [git worktree](/docs/ru/worktrees), дав ему изолированную копию репозитория, разветвлённую по умолчанию от вашей [ветки по умолчанию](/docs/ru/worktrees#choose-the-base-branch), а не от `HEAD` родительской сессии. Worktree автоматически очищается, если subagent не вносит изменения                            |
| `color`           | Нет       | Цвет отображения для subagent в списке задач и транскрипте. Принимает `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` или `cyan`                                                                                                                                                                                                                              |
| `initialPrompt`   | Нет       | Автоматически отправляется как первый ход пользователя, когда этот агент работает как основной агент сессии (через `--agent` или параметр `agent`). [Commands](/docs/ru/commands) и [skills](/docs/ru/skills) обрабатываются. Добавляется в начало любого предоставленного пользователем приглашения                                                                                 |

<h3 id="choose-a-model">
  Выберите модель
</h3>

Поле `model` контролирует, какую [AI модель](/docs/ru/model-config) использует subagent:

* **Model alias**: Используйте один из доступных псевдонимов: `sonnet`, `opus`, `haiku` или `fable`
* **Full model ID**: Используйте полный ID модели, такой как `claude-opus-4-8` или `claude-sonnet-5`. Принимает те же значения, что и флаг `--model`
* **inherit**: Используйте ту же модель, что и основной разговор
* **Omitted**: Если не указано, по умолчанию `inherit` и использует ту же модель, что и основной разговор

Когда Claude вызывает subagent, он также может передать параметр `model` для этого конкретного вызова. Claude Code разрешает модель subagent в этом порядке:

1. Переменная окружения [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/ru/model-config#environment-variables), если установлена на псевдоним модели или ID модели
2. Параметр `model` для конкретного вызова
3. Frontmatter `model` определения subagent
4. Модель основного разговора

Начиная с версии 2.1.196, установка `CLAUDE_CODE_SUBAGENT_MODEL` на `inherit` — это то же самое, что оставить его неустановленным: разрешение продолжается с параметром `model` для конкретного вызова, затем frontmatter. В более ранних версиях `inherit` заставлял subagents использовать модель основного разговора и игнорировал оба этих источника.

Переменная окружения, параметр для конкретного вызова и значения frontmatter проверяются на соответствие списку разрешений [`availableModels`](/docs/ru/model-config#restrict-model-selection) вашей организации. Значение, которое разрешается в исключённую модель, не используется, и subagent вместо этого работает на унаследованной модели.

Начиная с версии 2.1.198, subagents также наследуют конфигурацию [extended thinking](/docs/ru/model-config#extended-thinking) основного разговора: если thinking включен в вашей сессии, он включен для subagent, и если он выключен, он остаётся выключенным. Нет параметра thinking для каждого subagent. До версии 2.1.198 subagents запускались с отключённым extended thinking независимо от параметра основного разговора.

<h3 id="control-subagent-capabilities">
  Контролируйте возможности subagent
</h3>

Вы можете контролировать, что могут делать subagents, через доступ к инструментам, режимы разрешений и условные правила.

<h4 id="available-tools">
  Доступные инструменты
</h4>

Subagents наследуют [внутренние инструменты](/docs/ru/tools-reference) и MCP инструменты, доступные в основном разговоре по умолчанию. Следующие инструменты зависят от пользовательского интерфейса основного разговора или состояния сессии и недоступны для subagents, даже если они указаны в поле `tools`:

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`, если только [`permissionMode`](#permission-modes) subagent не является `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

Чтобы ограничить инструменты, используйте поле `tools` (список разрешений) или поле `disallowedTools` (список запретов). Этот пример использует `tools` для исключительного разрешения Read, Grep, Glob и Bash. Subagent не может редактировать файлы, писать файлы или использовать какие-либо MCP инструменты:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Этот пример использует `disallowedTools` для наследования каждого инструмента из основного разговора, кроме Write и Edit. Subagent сохраняет Bash, MCP инструменты и всё остальное:

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

Если оба установлены, `disallowedTools` применяется первым, затем `tools` разрешается против оставшегося пула. Инструмент, указанный в обоих, удаляется.

Когда ничего в списке `tools` не разрешается в инструмент, например потому что каждая запись неправильно написана или называет инструмент, который недоступен для subagents, Claude Code отказывается запускать subagent и инструмент Agent возвращает ошибку, называющую неразрешённые записи. До версии 2.1.208 этот subagent запускался без инструментов и мог вернуть пустой или запутанный результат.

Оба поля принимают паттерны уровня MCP сервера в дополнение к точным названиям инструментов: `mcp__<server>` или `mcp__<server>__*` предоставляет или удаляет каждый инструмент из названного сервера. В `disallowedTools`, `mcp__*` также удаляет каждый MCP инструмент из любого сервера. Этот пример удаляет каждый инструмент из MCP сервера `github`, сохраняя инструменты из других серверов и каждый встроенный инструмент:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

<h4 id="restrict-which-subagents-can-be-spawned">
  Ограничьте, какие subagents могут быть порождены
</h4>

Когда агент работает как основной поток с `claude --agent`, он может порождать subagents, используя инструмент Agent. Чтобы ограничить, какие типы subagent он может порождать, используйте синтаксис `Agent(agent_type)` в поле `tools`.

<Note>В версии 2.1.63 инструмент Task был переименован в Agent. Существующие ссылки `Task(...)` в настройках и определениях агентов по-прежнему работают как псевдонимы.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

Это список разрешений: только subagents `worker` и `researcher` могут быть порождены. Если агент попытается порождать любой другой тип, запрос не удастся и агент увидит только разрешённые типы в своём приглашении. Чтобы заблокировать конкретные агенты, разрешив все остальные, используйте [`permissions.deny`](#disable-specific-subagents) вместо этого.

Чтобы разрешить порождение любого subagent без ограничений, используйте `Agent` без скобок:

```yaml theme={null}
tools: Agent, Read, Bash
```

Если `Agent` полностью опущен из списка `tools`, агент не может порождать никакие subagents.

Синтаксис списка разрешений `Agent(agent_type)` применяется только к агенту, работающему как основной поток с `claude --agent`. В определении subagent перечисление `Agent` в `tools` позволяет этому subagent [порождать вложенные subagents](#spawn-nested-subagents), но любой список типов внутри скобок игнорируется.

<h4 id="scope-mcp-servers-to-a-subagent">
  Область MCP servers для subagent
</h4>

Используйте поле `mcpServers` для предоставления subagent доступа к [MCP](/docs/ru/mcp) серверам, которые недоступны в основном разговоре. Встроенные серверы, определённые здесь, подключаются при запуске subagent и отключаются при его завершении. Строковые ссылки используют соединение родительской сессии.

<Note>
  Поле `mcpServers` применяется в обоих контекстах, где может работать файл агента:

  * Как subagent, порождённый через инструмент Agent или @-упоминание
  * Как основная сессия, запущенная с [`--agent`](#invoke-subagents-explicitly) или параметром `agent`

  Когда агент является основной сессией, встроенные определения серверов подключаются при запуске вместе с серверами из [`.mcp.json`](/docs/ru/mcp) и файлов настроек.
</Note>

Каждая запись в списке — это либо встроенное определение сервера, либо строка, ссылающаяся на MCP сервер, уже настроенный в вашей сессии:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Встроенные определения используют ту же схему, что и записи сервера `.mcp.json`, ключевые по имени сервера, и поддерживают типы `stdio`, `http`, `sse` и `ws`.

Чтобы исключить MCP сервер из основного разговора полностью и избежать того, чтобы описания его инструментов потребляли контекст там, определите его встроенным здесь, а не в `.mcp.json`. Subagent получает инструменты; родительский разговор — нет.

Начиная с версии 2.1.153, ограничения MCP, которые применяются к основной сессии, также охватывают серверы, объявленные в frontmatter subagent:

* [`--strict-mcp-config`](/docs/ru/cli-reference) и [`--bare`](/docs/ru/cli-reference)
* [Enterprise управляемая конфигурация MCP](/docs/ru/managed-mcp)
* [`allowedMcpServers` и `deniedMcpServers` политики](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Когда один из них блокирует сервер, Claude Code пропускает его и показывает предупреждение с названием заблокированных серверов.

Ограничения управляемых настроек применяются к каждому subagent независимо от того, как он определён. `--strict-mcp-config` не фильтрует серверы, которые вы передаёте встроенным образом через `--agents` или опцию SDK `agents`, поскольку это явный ввод вызывающей стороны.

<h4 id="permission-modes">
  Режимы разрешений
</h4>

Поле `permissionMode` контролирует, как subagent обрабатывает запросы разрешений. Subagents наследуют контекст разрешений из основного разговора и могут переопределить режим, кроме случаев, когда режим родителя имеет приоритет, как описано ниже.

| Mode                | Behavior                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Стандартная проверка разрешений с запросами                                                                                                                                                                                                                                                                                                                                                |
| `acceptEdits`       | Автоматически принимать редактирование файлов и общие команды файловой системы для путей в рабочей директории или `additionalDirectories`                                                                                                                                                                                                                                                  |
| `auto`              | [Auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode): классификатор AI оценивает каждый вызов инструмента и записи в защищённые директории                                                                                                                                                                                                                                   |
| `dontAsk`           | Автоматически отклонять запросы разрешений. Явно разрешённые инструменты по-прежнему работают; `AskUserQuestion`, инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и MCP инструменты, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), отклоняются, даже если вы их разрешили |
| `bypassPermissions` | Пропустить все проверки разрешений                                                                                                                                                                                                                                                                                                                                                         |
| `plan`              | Режим плана (исследование только для чтения)                                                                                                                                                                                                                                                                                                                                               |

<Warning>
  Используйте `bypassPermissions` с осторожностью. Это пропускает все проверки разрешений, позволяя subagent выполнять любую операцию без одобрения, включая записи в `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` и `.mvn`.

  Явные правила [`ask`](/docs/ru/permissions#manage-permissions), инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools), MCP инструменты, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), и удаления корневого и домашнего каталога, такие как `rm -rf /`, по-прежнему требуют подтверждения. См. [permission modes](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode) для деталей.
</Warning>

Если родитель использует `bypassPermissions` или `acceptEdits`, это имеет приоритет и не может быть переопределено. Если родитель использует [auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), subagent наследует auto mode и любой `permissionMode` в его frontmatter игнорируется: классификатор оценивает вызовы инструментов subagent с теми же правилами блокировки и разрешения, что и родительская сессия.

<h4 id="preload-skills-into-subagents">
  Предварительная загрузка skills в subagents
</h4>

Используйте поле `skills` для инжекции содержимого skill в контекст subagent при запуске. Это даёт subagent знания в области без необходимости открывать и загружать skills во время выполнения.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

Полное содержимое каждого перечисленного skill инжектируется в контекст subagent при запуске. Это поле контролирует, какие skills предварительно загружаются, а не какие skills может использовать subagent: без него subagent по-прежнему может открывать и вызывать project, user и plugin skills через инструмент Skill во время выполнения. Чтобы предотвратить использование subagent skills полностью, опустите `Skill` из списка [`tools`](#available-tools) или добавьте его в `disallowedTools`.

Вы не можете предварительно загружать skills, которые устанавливают [`disable-model-invocation: true`](/docs/ru/skills#control-who-invokes-a-skill), поскольку предварительная загрузка берёт из того же набора skills, который Claude может вызывать. Если указанный skill отсутствует или отключен, Claude Code пропускает его и регистрирует предупреждение в журнал отладки.

<Note>
  Это противоположность [запуску skill в subagent](/docs/ru/skills#run-skills-in-a-subagent). С `skills` в subagent, subagent контролирует системное приглашение и загружает содержимое skill. С `context: fork` в skill, содержимое skill инжектируется в агента, который вы указываете. Оба используют одну и ту же базовую систему.
</Note>

<h4 id="enable-persistent-memory">
  Включите постоянную память
</h4>

Поле `memory` даёт subagent постоянный каталог, который сохраняется между разговорами. Subagent использует этот каталог для накопления знаний со временем, таких как паттерны кодовой базы, идеи отладки и архитектурные решения.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Выберите область в зависимости от того, насколько широко должна применяться память:

| Scope     | Location                                      | Используйте когда                                                                                             |
| :-------- | :-------------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | subagent должен помнить обучение во всех проектах                                                             |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | знания subagent специфичны для проекта и доступны для совместного использования через систему контроля версий |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | знания subagent специфичны для проекта, но не должны проверяться в систему контроля версий                    |

Когда память включена:

* Системное приглашение subagent включает инструкции для чтения и записи в каталог памяти.
* Системное приглашение subagent также включает первые 200 строк или 25KB `MEMORY.md` в каталоге памяти, в зависимости от того, что меньше, с инструкциями по курированию `MEMORY.md`, если она превышает этот лимит.
* Инструменты Read, Write и Edit автоматически включаются, чтобы subagent мог управлять своими файлами памяти.

<h5 id="persistent-memory-tips">
  Советы по постоянной памяти
</h5>

* `project` — рекомендуемая область по умолчанию. Это делает знания subagent доступными для совместного использования через систему контроля версий.
* Попросите subagent проверить его память перед началом работы: "Review this PR, and check your memory for patterns you've seen before."
* Попросите subagent обновить его память после завершения задачи: "Now that you're done, save what you learned to your memory." Со временем это создаёт базу знаний, которая делает subagent более эффективным.
* Включите инструкции по памяти непосредственно в файл markdown subagent, чтобы он активно поддерживал свою собственную базу знаний:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Условные правила с hooks
</h4>

Для более динамического контроля использования инструментов используйте `PreToolUse` hooks для проверки операций перед их выполнением. Это полезно, когда вам нужно разрешить некоторые операции инструмента, блокируя другие.

Этот пример создаёт subagent, который разрешает только запросы к базе данных только для чтения. Hook `PreToolUse` запускает скрипт, указанный в `command`, перед каждым выполнением команды Bash:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [передаёт входные данные hook как JSON](/docs/ru/hooks#pretooluse-input) через stdin командам hook. Скрипт валидации читает этот JSON, извлекает команду Bash и [выходит с кодом 2](/docs/ru/hooks#exit-code-2-behavior-per-event) для блокировки операций записи:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

См. [Hook input](/docs/ru/hooks#pretooluse-input) для полной схемы входных данных и [exit codes](/docs/ru/hooks#exit-code-output) для того, как коды выхода влияют на поведение. На Windows напишите скрипты hook в PowerShell и добавьте `shell: powershell` к записи hook, как показано в [запуске hooks в PowerShell](/docs/ru/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Отключите конкретные subagents
</h4>

Вы можете предотвратить использование Claude конкретных subagents, добавив их в массив `deny` в ваших [settings](/docs/ru/settings#permission-settings). Используйте формат `Agent(subagent-name)`, где `subagent-name` соответствует полю name subagent.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Это работает как для встроенных, так и для пользовательских subagents. Вы также можете использовать флаг CLI `--disallowedTools`:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

См. [документацию Permissions](/docs/ru/permissions#tool-specific-permission-rules) для получения дополнительной информации о правилах разрешений.

<h3 id="define-hooks-for-subagents">
  Определите hooks для subagents
</h3>

Subagents могут определять [hooks](/docs/ru/hooks), которые запускаются во время жизненного цикла subagent. Есть два способа настройки hooks:

* **В frontmatter subagent**: Определите hooks, которые запускаются только во время активности этого subagent
* **В `settings.json`**: Определите hooks, которые запускаются в основной сессии при запуске или остановке subagents

<h4 id="hooks-in-subagent-frontmatter">
  Hooks в frontmatter subagent
</h4>

Определите hooks непосредственно в файле markdown subagent. Эти hooks запускаются только во время активности этого конкретного subagent и очищаются при его завершении.

<Note>
  Frontmatter hooks срабатывают, когда агент порождается как subagent через инструмент Agent или @-упоминание, и когда агент работает как основной агент сессии через [`--agent`](#invoke-subagents-explicitly) или параметр `agent`. В случае основной сессии они запускаются вместе с любыми hooks, определёнными в [`settings.json`](/docs/ru/hooks).
</Note>

Поддерживаются все [hook events](/docs/ru/hooks#hook-events). Наиболее распространённые события для subagents:

| Event         | Matcher input   | Когда это срабатывает                                                           |
| :------------ | :-------------- | :------------------------------------------------------------------------------ |
| `PreToolUse`  | Имя инструмента | Перед использованием инструмента subagent                                       |
| `PostToolUse` | Имя инструмента | После использования инструмента subagent                                        |
| `Stop`        | (none)          | Когда subagent завершается (преобразуется в `SubagentStop` во время выполнения) |

Этот пример проверяет команды Bash с помощью hook `PreToolUse` и запускает linter после редактирования файлов с помощью `PostToolUse`:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Hooks `Stop` в frontmatter автоматически преобразуются в события `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks уровня проекта для событий subagent
</h4>

Настройте hooks в `settings.json`, которые реагируют на события жизненного цикла subagent в основной сессии.

| Event           | Matcher input   | Когда это срабатывает               |
| :-------------- | :-------------- | :---------------------------------- |
| `SubagentStart` | Имя типа агента | Когда subagent начинает выполнение  |
| `SubagentStop`  | Имя типа агента | Когда subagent завершает выполнение |

Оба события поддерживают matchers для нацеливания на конкретные типы агентов по имени. Значение matcher — это поле frontmatter `name` для project-level и user-level subagents, или scoped identifier, такой как `my-plugin:db-agent` для [plugin subagents](/docs/ru/plugins). Scoped name содержит двоеточие, поэтому он оценивается как [unanchored regular expression](/docs/ru/hooks#matcher-patterns); закрепите его с помощью `^` и `$`, как в `^my-plugin:db-agent$`, чтобы соответствовать только этому агенту.

Этот пример запускает скрипт установки только при запуске subagent `db-agent` и скрипт очистки при остановке любого subagent:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Matcher с дефисом, такой как `db-agent`, точно совпадает на Claude Code v2.1.195 или позже. На более ранних версиях он оценивается как unanchored regular expression и также срабатывает для любого типа агента, который его содержит, такого как `prod-db-agent`; закрепите его как `^db-agent$` на этих версиях.

См. [Hooks](/docs/ru/hooks) для полного формата конфигурации hook.

<h2 id="work-with-subagents">
  Работа с subagents
</h2>

<h3 id="understand-automatic-delegation">
  Поймите автоматическое делегирование
</h3>

Claude автоматически делегирует задачи на основе описания задачи в вашем запросе, поля `description` в конфигурациях subagent и текущего контекста. Чтобы поощрить активное делегирование, включите фразы вроде "use proactively" в поле description вашего subagent.

<h3 id="invoke-subagents-explicitly">
  Явно вызывайте subagents
</h3>

Когда автоматического делегирования недостаточно, вы можете запросить subagent самостоятельно. Три паттерна переходят от одноразового предложения к сессионному по умолчанию:

* **Естественный язык**: назовите subagent в вашем приглашении; Claude решает, делегировать ли
* **@-упоминание**: гарантирует, что subagent запустится для одной задачи
* **Сессионный уровень**: вся сессия использует системное приглашение, ограничения инструментов и модель этого subagent через флаг `--agent` или параметр `agent`

Для естественного языка нет специального синтаксиса. Назовите subagent и Claude обычно делегирует:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-упомяните subagent.** Введите `@` и выберите subagent из автодополнения, так же как вы упоминаете файлы. Это гарантирует, что запустится конкретный subagent, а не оставляет выбор Claude:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Ваше полное сообщение по-прежнему идёт Claude, который пишет приглашение задачи subagent на основе того, что вы попросили. @-упоминание контролирует, какой subagent Claude вызывает, а не какое приглашение он получает.

Subagents, предоставленные включённым [plugin](/docs/ru/plugins), появляются в автодополнении под их именем с областью видимости, например `my-plugin:code-reviewer` или `my-plugin:review:security`, когда plugin [организует агентов в подпапки](#choose-the-subagent-scope). Именованные фоновые subagents, в настоящее время работающие в сессии, также появляются в автодополнении, показывая их статус рядом с именем.

Вы также можете ввести упоминание вручную без использования средства выбора: `@agent-<name>` для локальных subagents или `@agent-` с последующим именем с областью видимости для plugin subagents, например `@agent-my-plugin:code-reviewer`.

**Запустите всю сессию как subagent.** Передайте [`--agent <name>`](/docs/ru/cli-reference) для запуска сессии, где основной поток сам принимает системное приглашение, ограничения инструментов и модель этого subagent:

```bash theme={null}
claude --agent code-reviewer
```

Системное приглашение subagent полностью заменяет системное приглашение Claude Code по умолчанию, так же как [`--system-prompt`](/docs/ru/cli-reference) это делает. Файлы `CLAUDE.md` и память проекта по-прежнему загружаются через обычный поток сообщений. Имя агента появляется как `@<name>` в заголовке запуска, чтобы вы могли подтвердить, что он активен.

Это работает с встроенными и пользовательскими subagents, и выбор сохраняется при возобновлении сессии.

Для plugin-предоставленного subagent вы можете передать просто имя агента и Claude Code найдёт его:

```bash theme={null}
claude --agent security-reviewer
```

Если несколько plugins предоставляют агентов с одинаковым именем, передайте имя с областью видимости для уточнения:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Если plugin размещает агента в подпапке своей директории `agents/`, включите подпапку в имя с областью видимости, например `claude --agent my-plugin:review:security`.

Чтобы сделать это по умолчанию для каждой сессии в проекте, установите `agent` в `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

Флаг CLI переопределяет параметр, если оба присутствуют.

<h3 id="run-subagents-in-foreground-or-background">
  Запустите subagents в переднем плане или фоне
</h3>

Subagents могут работать в переднем плане или фоне:

* **Foreground subagents** блокируют основной разговор до завершения. Запросы разрешений передаются вам по мере их возникновения.
* **Background subagents** работают параллельно, пока вы продолжаете работать. Начиная с v2.1.186, когда фоновый subagent достигает вызова инструмента, требующего разрешения, приглашение появляется в вашей основной сессии и называет subagent, который спрашивает. Одобрите, чтобы позволить subagent продолжить, или нажмите Esc, чтобы отклонить этот вызов инструмента без остановки subagent. До v2.1.186 фоновые subagents автоматически отклоняли любой вызов инструмента, который иначе потребовал бы приглашения.

Начиная с v2.1.198, subagents работают в фоне по умолчанию. Claude запускает subagent в переднем плане, когда ему нужен результат перед продолжением. По умолчанию изменяется место выполнения subagent, а не то, что ему разрешено делать: фоновые subagents по-прежнему выводят каждый запрос разрешения в вашу основную сессию. До v2.1.198 Claude выбирал между передним планом и фоном на основе задачи.

Вы также можете управлять этим самостоятельно:

* Попросите Claude запустить задачу в фоне или в переднем плане
* Нажмите **Ctrl+B** для фонового выполнения работающей задачи

Фоновый subagent, который завершается, остаётся в списке [`/tasks`](/docs/ru/commands), отмечен как выполненный и отсортирован ниже работающих задач, пока сессия не очистит список задач. Его представление деталей остаётся открытым, когда subagent завершается. Subagents, которые не удаются или которые вы остановили, покидают список. До v2.1.208 завершённый subagent покидал список в момент завершения и его представление деталей закрывалось.

Чтобы отключить всю функциональность фоновых задач, установите переменную окружения `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` на `1`. См. [Environment variables](/docs/ru/env-vars).

Когда [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) установлена на `1`, каждый spawn subagent работает в фоне и поле frontmatter `background` не имеет эффекта, потому что режим fork удаляет параметр `run_in_background` из инструмента `Agent`. `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` имеет приоритет над режимом fork и сохраняет spawns subagent в переднем плане.

<h3 id="api-errors-in-subagents">
  Ошибки API в subagents
</h3>

Начиная с v2.1.199, subagent, чей запуск заканчивается ошибкой API, такой как лимит использования или повторяющаяся ошибка сервера, сообщает об этом отказе Claude вместо возврата текста ошибки, как если бы это были результаты subagent. То, что получает Claude, зависит от того, где работал subagent:

* **Foreground**: если лимит скорости, перегрузка или ошибка сервера прерывает subagent, который уже произвёл выходные данные, инструмент Agent возвращает эти частичные выходные данные с примечанием, что subagent был прерван и не завершил свою задачу. Subagent, который не произвёл ничего, или чьим единственным выходом были вызовы инструментов, завершается с ошибкой [`Agent terminated early due to an API error`](/docs/ru/errors#agent-terminated-early-due-to-an-api-error), за которой следует деталь ошибки. В v2.1.199 лимит скорости, перегрузка или ошибка сервера, которые прервали форму, содержащую только вызовы инструментов, возвращали пустой частичный результат, содержащий только примечание об отключении вместо этого.
* **Background**: subagent помечается как неудачный, и сообщение, которое Claude получает при завершении, называет ошибку API и включает последний выход subagent, поэтому частичная работа не теряется.

После того как основная ошибка API исчезнет, попросите Claude повторить задачу или [возобновить subagent](#resume-subagents).

<h3 id="common-patterns">
  Распространённые паттерны
</h3>

<h4 id="isolate-high-volume-operations">
  Изолируйте высокообъёмные операции
</h4>

Одно из наиболее эффективных применений subagents — изоляция операций, которые производят большой объём выходных данных. Запуск тестов, получение документации или обработка файлов журналов может потребить значительный контекст. Делегируя эти операции subagent, подробный выход остаётся в контексте subagent, пока только релевантное резюме возвращается в основной разговор.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Запустите параллельное исследование
</h4>

Для независимых исследований порождайте несколько subagents для одновременной работы:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Каждый subagent исследует свою область независимо, затем Claude синтезирует результаты. Это работает лучше всего, когда пути исследования не зависят друг от друга.

<Warning>
  Когда subagents завершаются, их результаты возвращаются в основной разговор. Запуск многих subagents, каждый из которых возвращает подробные результаты, может потребить значительный контекст.
</Warning>

Для задач, требующих устойчивого параллелизма или превышающих контекстное окно, [agent teams](/docs/ru/agent-teams) дают каждому работнику собственный независимый контекст.

<h4 id="chain-subagents">
  Цепочка subagents
</h4>

Для многошаговых рабочих процессов попросите Claude использовать subagents последовательно. Каждый subagent завершает свою задачу и возвращает результаты Claude, который затем передаёт релевантный контекст следующему subagent.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Выберите между subagents и основным разговором
</h3>

Используйте **основной разговор** когда:

* Задача требует частого взаимодействия или итеративного уточнения
* Несколько фаз имеют значительный общий контекст, такие как планирование, реализация и тестирование
* Вы вносите быстрое, целевое изменение
* Задержка имеет значение. Subagents начинают с нуля и могут потребовать время для сбора контекста

Используйте **subagents** когда:

* Задача производит подробный выход, который вам не нужен в основном контексте
* Вы хотите применить конкретные ограничения инструментов или разрешений
* Работа самодостаточна и может вернуть резюме

Рассмотрите [Skills](/docs/ru/skills) вместо этого, когда вы хотите переиспользуемые приглашения или рабочие процессы, которые работают в контексте основного разговора, а не в изолированном контексте subagent.

Для быстрого вопроса о чём-то уже в вашем разговоре используйте [`/btw`](/docs/ru/interactive-mode#side-questions-with-%2Fbtw) вместо subagent. Он видит ваш полный контекст, но не имеет доступа к инструментам, и ответ отбрасывается, а не добавляется в историю.

<h3 id="spawn-nested-subagents">
  Порождайте вложенные subagents
</h3>

Начиная с Claude Code v2.1.172, subagent может порождать собственные subagents. Используйте это, когда делегированная задача сама разбивается на параллельные подзадачи, например subagent-рецензент, который отправляет верификатор для каждого обнаружения, так что промежуточный выход никогда не достигает основного разговора. Только резюме subagent верхнего уровня возвращается вам.

Вложенный subagent конфигурируется так же, как subagent верхнего уровня и разрешается из тех же [областей видимости](#choose-the-subagent-scope).

Панель subagent ниже ввода приглашения показывает полное дерево: каждая строка отображает счётчик `(+N)` потомков, и начиная с v2.1.193, открытие строки показывает прямых потомков этого subagent с путём обратно к `main`.

Глубина считается количеством уровней subagent ниже основного разговора, независимо от того, работает ли каждый уровень в [переднем плане или фоне](#run-subagents-in-foreground-or-background). Subagent на глубине пять не получает инструмент Agent и не может порождать дальше. Лимит фиксирован и не конфигурируется.

Начиная с Claude Code v2.1.187, глубина фонового subagent фиксируется при его первом порождении, и [возобновление](#resume-subagents) его позже не изменяет эту глубину. Например, если ваш основной разговор порождает subagent A, и A порождает фоновый subagent B на глубине два, B по-прежнему находится на глубине два при возобновлении его непосредственно из основного разговора. Возобновление subagent из более поверхностного контекста не позволяет ему порождать дополнительные уровни, которые лимит глубины уже предотвратил.

Чтобы предотвратить порождение других subagents конкретным subagent, опустите `Agent` из его списка [`tools`](#available-tools) или добавьте его в `disallowedTools`.

[Fork](#fork-the-current-conversation) по-прежнему не может порождать другой fork. Он может порождать другие типы subagent, и они считаются в сторону лимита глубины.

<h3 id="manage-subagent-context">
  Управляйте контекстом subagent
</h3>

<h4 id="what-loads-at-startup">
  Что загружается при запуске
</h4>

Каждый subagent начинает со свежего, изолированного контекстного окна. Он не видит историю вашего разговора, навыки, которые вы уже вызвали, или файлы, которые Claude уже прочитал. Claude составляет сообщение делегирования, которое резюмирует задачу, и subagent работает на основе этого. Исключением является [fork](#fork-the-current-conversation), который наследует родительский разговор вместо начала с нуля.

Начальный контекст non-fork subagent содержит:

* **System prompt**: собственное приглашение агента плюс детали окружения, которые добавляет Claude Code, а не полное системное приглашение Claude Code. Пользовательские subagents определяют свои в [markdown body](#write-subagent-files) или поле `prompt`. Встроенные агенты имеют предопределённые приглашения.
* **Task message**: приглашение делегирования, которое Claude пишет при передаче работы.
* **CLAUDE.md and memory**: каждый уровень [иерархии памяти](/docs/ru/memory#how-claude-md-files-load), который загружает основной разговор, включая `~/.claude/CLAUDE.md`, правила проекта, `CLAUDE.local.md` и управляемые файлы политики. Встроенные агенты Explore и Plan пропускают это.
* **Git status**: снимок, сделанный в начале родительской сессии. Отсутствует, когда рабочая директория не является Git репозиторием или когда [`includeGitInstructions`](/docs/ru/settings#available-settings) имеет значение `false`. Explore и Plan пропускают это независимо.
* **Preloaded skills**: полное содержание любого навыка, названного в поле [`skills`](#preload-skills-into-subagents) агента. Встроенные агенты не предзагружают навыки.
* **Sibling roster**: системное напоминание, в котором перечислены `main` и каждый другой именованный агент в сессии, каждый является допустимым значением `to` для [`SendMessage`](#resume-subagents). Требует Claude Code v2.1.206 или позже. Реестр появляется только когда инструменты subagent включают `SendMessage` и по крайней мере один другой агент имеет имя, независимо от того, назвал ли его Claude при порождении или он работает как товарищ [agent team](/docs/ru/agent-teams). Это снимок, сделанный при запуске subagent, поэтому агенты, названные позже, не появляются.

Explore и Plan — единственные subagents, которые опускают CLAUDE.md и git status. Нет поля frontmatter или параметра per-agent для изменения того, какие агенты их пропускают.

Основной разговор читает результаты Explore и Plan с полным контекстом CLAUDE.md, поэтому большинству правил не нужно достигать самого subagent. Если правило должно, например "ignore the `vendor/` directory," переформулируйте его в приглашении, которое вы даёте Claude при делегировании.

<h4 id="resume-subagents">
  Возобновите subagents
</h4>

Каждый вызов subagent создаёт новый экземпляр со свежим контекстом. Чтобы продолжить работу существующего subagent вместо начала с нуля, попросите Claude возобновить его.

Возобновлённые subagents сохраняют полную историю разговора, включая все предыдущие вызовы инструментов, результаты и рассуждения. Subagent продолжает ровно там, где он остановился, а не начинает с нуля.

Когда subagent завершается, Claude получает его ID агента. Встроенные агенты Explore и Plan — это одноразовые и не возвращают ID агента, поэтому они не могут быть возобновлены; используйте `general-purpose` или пользовательский subagent, когда вам нужно продолжить работу.

Claude использует инструмент `SendMessage` с ID агента или именем агента в качестве поля `to` для возобновления его. `SendMessage` не требует включения [agent teams](/docs/ru/agent-teams); только структурированные сообщения протокола команды, такие как `shutdown_request` и `plan_approval_response`, это требуют.

Чтобы возобновить subagent, попросите Claude продолжить предыдущую работу:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Завершённый subagent, который получает `SendMessage`, автоматически возобновляется в фоне без новой инвокации `Agent`. То же самое применяется к subagent, который Claude остановил с помощью инструмента `TaskStop`.

Начиная с v2.1.191, subagent, который вы остановили сами, с помощью `x` в `/tasks` или запроса SDK `stop_task`, не автоматически возобновляется. Вызов `SendMessage` возвращает отказ, сообщающий Claude, что агент был отменён. Введите в транскрипт этого subagent в панели subagent, чтобы возобновить его самостоятельно, что очищает остановку, чтобы более поздние вызовы `SendMessage` могли автоматически возобновить его снова.

Возобновление начинает новый запуск агента под тем же ID, поэтому subagent, который уже завершился или был неудачным, показывается как работающий снова в списке задач и в событиях задач Agent SDK. До v2.1.205 он продолжал показывать свой более ранний статус неудачи или завершения, пока возобновленный запуск работал.

Начиная с v2.1.199, `SendMessage` проверяет, что имя по-прежнему ссылается на того же агента, которого оно достигло ранее в разговоре. Если более новый агент взял имя, например повторно порождённый фоновый агент, который его переиспользовал, Claude Code отказывает в отправке, а не доставляет его неправильному агенту, и ошибка сообщает, какого агента имя теперь достигает, чтобы Claude мог перенаправить. Чтобы достичь более раннего агента, пока он всё ещё работает, Claude обращается к нему по ID агента из результата spawn. Проверка ограничена текущим разговором и сбрасывается на `/clear`.

Начиная с v2.1.198, subagent рассматривает сообщения от агента, который его запустил, как нормальное направление задачи, включая коррекции курса во время выполнения, и действует в соответствии с ними в рамках своих собственных параметров разрешения. Два лимита по-прежнему действуют независимо от того, кто отправил сообщение: ни одно сообщение от любого агента не считается вашим одобрением для ожидающего запроса разрешения, и ни один агент не может изменить параметры разрешения subagent, `CLAUDE.md` или конфигурацию. Только система разрешений или ваши собственные сообщения могут предоставить одобрение.

Вы также можете попросить Claude ID агента, если хотите ссылаться на него явно, или найти ID в файлах транскрипта в `~/.claude/projects/{project}/{sessionId}/subagents/`. Каждый транскрипт сохраняется как `agent-{agentId}.jsonl`.

Транскрипты subagent сохраняются независимо от основного разговора:

* **Компактирование основного разговора**: Когда основной разговор компактируется, транскрипты subagent не затрагиваются. Они сохраняются в отдельных файлах.
* **Сохранение сессии**: Транскрипты subagent сохраняются в пределах их сессии. Вы можете [возобновить subagent](#resume-subagents) после перезагрузки Claude Code, возобновив ту же сессию.
* **Автоматическая очистка**: Транскрипты очищаются на основе параметра `cleanupPeriodDays`, который по умолчанию составляет 30 дней.

<h4 id="auto-compaction">
  Auto-compaction
</h4>

Subagents поддерживают автоматическое компактирование, используя ту же логику, что и основной разговор. Компактирование срабатывает при тех же условиях, и `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` применяется к subagents также. См. [environment variables](/docs/ru/env-vars) для того, когда переопределение вступает в силу.

События компактирования регистрируются в файлах транскрипта subagent:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

Значение `preTokens` показывает, сколько токенов было использовано перед компактированием.

<h2 id="fork-the-current-conversation">
  Разветвление текущего разговора
</h2>

<Note>
  Разветвленные subagents требуют Claude Code v2.1.117 или позже. Начиная с v2.1.161 команда `/fork` включена по умолчанию; в более ранних версиях требуется установка переменной окружения [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/ru/env-vars) на `1`. Позволение Claude самому порождать разветвления является экспериментальным и может измениться в будущих выпусках. Эта возможность также может быть включена в интерактивных сессиях как часть поэтапного развертывания.
</Note>

Fork — это subagent, который наследует весь разговор до сих пор вместо начала с нуля. Это отбрасывает входную изоляцию, которую subagents иначе предоставляют: fork видит то же системное приглашение, инструменты, модель и историю сообщений, что и основная сессия, поэтому вы можете передать ему побочную задачу без переобъяснения ситуации. Вызовы инструментов fork по-прежнему остаются вне вашего разговора и только его окончательный результат возвращается, поэтому ваше основное контекстное окно остаётся чистым. Используйте fork, когда именованный subagent потребовал бы слишком много фона, чтобы быть полезным, или когда вы хотите попробовать несколько подходов параллельно с одной и той же отправной точки.

Чтобы управлять режимом fork независимо от поэтапного развертывания, установите [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/ru/env-vars) на `1`, чтобы явно включить его, или на `0`, чтобы отключить его. Переменная учитывается в интерактивном режиме и через SDK или `claude -p`.

Включение режима fork изменяет Claude Code двумя способами:

* Claude может порождать fork, явно запрашивая тип subagent `fork`. Порождения без типа subagent по-прежнему используют [general-purpose](#built-in-subagents) subagent, и именованные subagents, такие как Explore, по-прежнему порождаются как раньше.
* Каждый spawn subagent работает в [фоне](#run-subagents-in-foreground-or-background), будь то fork или именованный subagent. Установите `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` на `1`, чтобы сохранить spawns синхронными.

Вы можете запустить fork самостоятельно с `/fork` за которым следует директива, с переменной или без неё. Claude Code называет fork из первых слов директивы. Следующий пример разветвляет разговор для черновика тестовых случаев, пока вы продолжаете с реализацией в основной сессии:

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

Fork появляется в панели ниже вашего приглашения и работает в фоне, пока вы продолжаете работать. Когда он завершается, его результат приходит как сообщение в вашем основном разговоре. Следующий раздел охватывает элементы управления панели для наблюдения и управления forks во время их работы.

<h3 id="observe-and-steer-running-forks">
  Наблюдение и управление работающими forks
</h3>

Работающие forks появляются в панели ниже входа приглашения, с одной строкой для основной сессии и одной для каждого fork. Используйте эти клавиши для взаимодействия с панелью:

| Key       | Action                                                                    |
| :-------- | :------------------------------------------------------------------------ |
| `↑` / `↓` | Перемещение между строками                                                |
| `Enter`   | Откройте транскрипт выбранного fork и отправьте ему последующие сообщения |
| `x`       | Отклоните завершённый fork или остановите работающий                      |
| `Esc`     | Верните фокус на входное приглашение                                      |

С открытым транскриптом fork или subagent, последующие сообщения и [skills](/docs/ru/skills) идут к этому агенту, но встроенные команды по-прежнему работают в вашем основном разговоре. Начиная с v2.1.199, ввод `/model` или `/fast` в этом представлении показывает уведомление о том, что это изменяет модель основного разговора или режим быстрого выполнения, а не просмотренного агента, вместо того чтобы запускать его молча.

<h3 id="how-forks-differ-from-named-subagents">
  Как forks отличаются от именованных subagents
</h3>

Fork наследует всё, что основная сессия имеет в момент его порождения. Именованный subagent начинает с собственного определения.

|                         | Fork                                | Named subagent                                                                                             |
| :---------------------- | :---------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| Context                 | Полная история разговора            | Свежий контекст с приглашением, которое вы передаёте                                                       |
| System prompt and tools | Такие же как основная сессия        | Из [определения файла](#write-subagent-files) subagent                                                     |
| Model                   | Такая же как основная сессия        | Из поля `model` subagent                                                                                   |
| Permissions             | Запросы выводятся в вашем терминале | [Запросы выводятся в вашей основной сессии](#run-subagents-in-foreground-or-background) при запуске в фоне |
| Prompt cache            | Общий с основной сессией            | Отдельный кэш                                                                                              |

Поскольку системное приглашение fork и определения инструментов идентичны родителю, его первый запрос повторно использует кэш приглашений родителя [prompt cache](/docs/ru/prompt-caching#subagents-and-the-cache). Это делает forking дешевле, чем порождение свежего subagent для задач, которые нуждаются в том же контексте.

Когда Claude порождает fork через инструмент Agent, он может передать `isolation: "worktree"`, чтобы редактирования файлов fork были написаны в отдельный git worktree вместо вашего checkout.

<h3 id="limitations">
  Ограничения
</h3>

Установка `CLAUDE_CODE_FORK_SUBAGENT=1` включает режим fork в интерактивных сессиях, [non-interactive mode](/docs/ru/headless) и Agent SDK; установка на `0` отключает режим fork везде, включая любое развертывание на стороне сервера. Fork не может порождать дальнейшие forks.

<h2 id="example-subagents">
  Примеры subagents
</h2>

Эти примеры демонстрируют эффективные паттерны для создания subagents. Используйте их как отправные точки или генерируйте настроенную версию с Claude.

<Tip>
  **Best practices:**

  * **Проектируйте сфокусированные subagents:** каждый subagent должен превосходить в одной конкретной задаче
  * **Напишите подробные описания:** Claude использует описание для решения о делегировании
  * **Ограничьте доступ к инструментам:** предоставьте только необходимые разрешения для безопасности и сфокусированности
  * **Проверьте в систему контроля версий:** поделитесь project subagents с вашей командой
</Tip>

<h3 id="code-reviewer">
  Проверяющий кода
</h3>

Subagent только для чтения, который проверяет код без его модификации. Этот пример показывает, как спроектировать сфокусированный subagent с ограниченным доступом к инструментам, который исключает Edit и Write, и подробным приглашением, которое точно указывает, что искать и как форматировать выход.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Отладчик
</h3>

Subagent, который может как анализировать, так и исправлять проблемы. В отличие от проверяющего кода, этот включает Edit, потому что исправление ошибок требует модификации кода. Приглашение предоставляет чёткий рабочий процесс от диагностики к проверке.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Специалист по данным
</h3>

Специализированный subagent для работы анализа данных. Этот пример показывает, как создавать subagents для специализированных рабочих процессов вне типичных задач кодирования. Он явно устанавливает `model: sonnet` для более способного анализа.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Валидатор запросов к базе данных
</h3>

Subagent, который разрешает доступ Bash, но проверяет команды для разрешения только запросов SQL только для чтения. Этот пример показывает, как использовать `PreToolUse` hooks для условной валидации, когда вам нужен более тонкий контроль, чем предоставляет поле `tools`.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [передаёт входные данные hook как JSON](/docs/ru/hooks#pretooluse-input) через stdin командам hook. Скрипт валидации читает этот JSON, извлекает выполняемую команду и проверяет её против списка операций записи SQL. Если обнаружена операция записи, скрипт [выходит с кодом 2](/docs/ru/hooks#exit-code-2-behavior-per-event) для блокировки выполнения и возвращает сообщение об ошибке Claude через stderr.

Создайте скрипт валидации где-нибудь в вашем проекте. Путь должен соответствовать полю `command` в конфигурации hook:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

На macOS и Linux сделайте скрипт исполняемым:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

На Windows напишите скрипт валидации на PowerShell и добавьте `shell: powershell` к записи hook. См. [запуск hooks в PowerShell](/docs/ru/hooks#windows-powershell-tool).

Hook получает JSON через stdin с командой Bash в `tool_input.command`. Код выхода 2 блокирует операцию и передаёт сообщение об ошибке обратно Claude. См. [Hooks](/docs/ru/hooks#exit-code-output) для деталей кодов выхода и [Hook input](/docs/ru/hooks#pretooluse-input) для полной схемы входных данных.

<h2 id="next-steps">
  Следующие шаги
</h2>

Теперь, когда вы понимаете subagents, изучите эти связанные функции:

* [Распространяйте subagents с помощью plugins](/docs/ru/plugins) для совместного использования subagents в командах или проектах
* [Запустите Claude Code программно](/docs/ru/headless) с помощью Agent SDK для CI/CD и автоматизации
* [Используйте MCP servers](/docs/ru/mcp) для предоставления subagents доступа к внешним инструментам и данным
