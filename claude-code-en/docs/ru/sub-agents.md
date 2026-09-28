> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Создание пользовательских subagents

> Создавайте и используйте специализированные AI subagents в Claude Code для рабочих процессов, ориентированных на конкретные задачи, и улучшенного управления контекстом.

Subagents — это специализированные AI-помощники, которые обрабатывают определённые типы задач. Используйте один, когда побочная задача заполнит основной разговор результатами поиска, логами или содержимым файлов, на которые вы больше не будете ссылаться: subagent выполняет эту работу в собственном контексте и возвращает только резюме. Определите пользовательский subagent, когда вы постоянно порождаете одного и того же рабочего с одинаковыми инструкциями.

Каждый subagent работает в собственном контекстном окне с пользовательским системным приглашением, специфическим доступом к инструментам и независимыми разрешениями. Когда Claude встречает задачу, соответствующую описанию subagent, он делегирует её этому subagent, который работает независимо и возвращает результаты. Чтобы увидеть экономию контекста на практике, [визуализация контекстного окна](/docs/ru/context-window) проходит через сессию, где subagent обрабатывает исследование в собственном отдельном окне.

<Note>
  Subagents работают в рамках одной сессии. Чтобы запустить множество независимых сессий параллельно и отслеживать их из одного места, см. [background agents](/docs/ru/agent-view). Для отдельных сессий, которые передают сообщения друг другу, см. [cross-session messaging](/docs/ru/cross-session-messaging). Для скоординированной команды сессий, которые Claude порождает и контролирует, см. [agent teams](/docs/ru/agent-teams).
</Note>

Subagents помогают вам:

* **Сохранять контекст**, отделяя исследование и реализацию от основного разговора
* **Применять ограничения**, ограничивая доступ subagent к определённым инструментам
* **Переиспользовать конфигурации** в проектах с помощью subagents уровня пользователя
* **Специализировать поведение** с помощью сфокусированных системных приглашений для конкретных областей
* **Контролировать затраты**, маршрутизируя задачи на более быстрые и дешёвые модели, такие как Haiku

Claude использует описание каждого subagent для решения о делегировании задач. Когда вы создаёте subagent, напишите чёткое описание, чтобы Claude знал, когда его использовать.

Эти описания занимают контекст, поэтому держите их краткими. Когда объединённые описания ваших subagents, кроме встроенных, превышают 15 000 токенов, Claude Code показывает [предупреждение при запуске с общим количеством токенов](/docs/ru/errors#agent-descriptions-are-over-the-15000-token-limit). Сократите поля `description` ваших subagents и переместите детали в системное приглашение каждого subagent, которое загружается только при запуске этого subagent.

<h2 id="built-in-subagents">
  Встроенные subagents
</h2>

Claude Code включает встроенные subagents, которые Claude автоматически использует при необходимости. Каждый наследует разрешения родительского разговора; большинство работают с ограниченным набором инструментов.

Explore и Plan пропускают ваши файлы CLAUDE.md и снимок статуса git, чтобы исследование было быстрым и экономичным. Все остальные встроенные и [пользовательские subagents](#configure-subagents) загружают оба, если только их определение не устанавливает поле [`omitClaudeMd`](#supported-frontmatter-fields) для пропуска файлов CLAUDE.md пользователя, проекта и локального. Для полного разбора того, что достигает subagent, см. [что загружается при запуске](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Быстрый агент, доступный только для чтения, оптимизированный для поиска и анализа кодовых баз.

    * **Model**: наследуется из основного разговора, ограничен Opus на Claude API, поэтому Explore никогда не работает на более дорогой модели, чем та, которую вы уже выбрали для сессии, если только вы не установите `CLAUDE_CODE_SUBAGENT_MODEL` и не [заставите его работать на каждом subagent](#run-every-subagent-on-one-model)
    * **Tools**: инструменты только для чтения; Write и Edit запрещены
    * **Purpose**: обнаружение файлов, поиск кода, исследование кодовой базы

    Начиная с версии 2.1.198, Explore наследует модель основного разговора вместо того, чтобы всегда работать на Haiku. На Claude API унаследованная модель ограничена Opus: основной разговор на более высоком уровне запускает Explore на Opus, а основной разговор на Sonnet или Haiku запускает Explore на той же модели. На любом другом поставщике, таком как [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или Claude Platform on AWS](/docs/ru/third-party-integrations), Explore наследует модель основного разговора напрямую.

    [Пользовательский или проектный subagent](#choose-the-subagent-scope) с именем `Explore` переопределяет встроенный и сохраняет собственное поле `model`, поэтому определите его с `model: haiku`, чтобы сохранить исследование на модели с более низкой стоимостью.

    Claude делегирует Explore, когда ему нужно искать или понимать кодовую базу без внесения изменений. Это сохраняет результаты исследования вне контекста основного разговора.

    При вызове Explore Claude указывает уровень тщательности: **quick** для целевых поисков, **medium** для сбалансированного исследования или **very thorough** для комплексного анализа.
  </Tab>

  <Tab title="Plan">
    Исследовательский агент, используемый во время [plan mode](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode) для сбора контекста перед представлением плана.

    * **Model**: наследуется из основного разговора, если только вы не установите `CLAUDE_CODE_SUBAGENT_MODEL` и не [заставите его работать на каждом subagent](#run-every-subagent-on-one-model)
    * **Tools**: инструменты только для чтения; Write и Edit запрещены
    * **Purpose**: исследование кодовой базы для планирования

    Когда вы находитесь в режиме плана и Claude нужно понять вашу кодовую базу, он делегирует исследование subagent Plan, чтобы результаты исследования оставались в отдельном окне контекста, а основной разговор оставался доступным только для чтения.
  </Tab>

  <Tab title="General-purpose">
    Способный агент для сложных многошаговых задач, требующих как исследования, так и действия.

    * **Model**: модель [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model), если вы её установили и ничто не назначает модель другим способом, иначе модель основного разговора; [Выбор модели](#choose-a-model) указывает полный порядок, и [Запуск каждого subagent на одной модели](#run-every-subagent-on-one-model) показывает, как сделать переменную переопределением этих источников
    * **Tools**: каждый инструмент [доступный для subagents](#available-tools)
    * **Purpose**: сложное исследование, многошаговые операции, модификация кода

    Claude делегирует general-purpose, когда задача требует как исследования, так и модификации, сложного рассуждения для интерпретации результатов или нескольких зависимых шагов.
  </Tab>

  <Tab title="Other">
    Claude Code включает дополнительные вспомогательные агенты для конкретных задач. Обычно они вызываются автоматически, поэтому вам не нужно использовать их напрямую.

    | Agent             | Model                                                                                                | Когда Claude его использует                                                                                                                                                                                                                                                                                                                                           |
    | :---------------- | :--------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | Нет собственной; следует [порядку моделей](#choose-a-model), когда Claude порождает его как subagent | Когда задача не подходит более специализированному агенту. Универсальный агент со всеми инструментами [доступными для subagents](#available-tools). Также агент по умолчанию для отправленной [фоновой сессии](/docs/ru/agent-view); [в каком режиме разрешений он запускается](/docs/ru/agent-view#permission-mode-model-and-effort) зависит от того, как была запущена сессия |
    | statusline-setup  | Sonnet                                                                                               | Когда вы запускаете `/statusline` для настройки строки состояния                                                                                                                                                                                                                                                                                                      |
    | claude-code-guide | Haiku                                                                                                | Когда вы задаёте вопросы о функциях Claude Code                                                                                                                                                                                                                                                                                                                       |
  </Tab>
</Tabs>

Встроенные subagents регистрируются по умолчанию в интерактивных сессиях. Чтобы ограничить их:

* Чтобы заблокировать определённый встроенный тип, добавьте его в `permissions.deny`, как показано в [Отключение определённых subagents](#disable-specific-subagents).
* Чтобы предотвратить делегирование Claude к любому subagent, запретите сам инструмент `Agent` с помощью [`permissions.deny`](/docs/ru/permissions#tool-specific-permission-rules).
* Чтобы удалить только встроенные subagents `Explore` и `Plan`, установите [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/ru/env-vars). Claude читает и исследует файлы напрямую вместо делегирования им. Требуется Claude Code версии 2.1.198 или позже.
* В [неинтерактивном режиме](/docs/ru/headless) и [Agent SDK](/docs/ru/agent-sdk/overview) установите [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ru/env-vars) для удаления всех встроенных типов и предоставления только ваших собственных.

Вызов инструмента Agent, который опускает `subagent_type`, завершается ошибкой [`subagent_type is required`](/docs/ru/errors#subagent-type-is-required), когда сессия не имеет `general-purpose` subagent для отката.

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

    Claude делегирует вашему новому subagent, который сканирует кодовую базу и возвращает предложения по улучшению. В расшифровке делегирование отображается как строка вызова инструмента, показывающая имя subagent, за которым следует краткое описание задачи, например `code-improver(Suggest code improvements)`.

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
| Директория `agents/` plugin | Где включен plugin | 5 (наименьший) | Установлено с [plugins](/docs/ru/plugins/overview)     |

**Project subagents** (`.claude/agents/`) идеальны для subagents, специфичных для кодовой базы. Проверьте их в систему контроля версий, чтобы ваша команда могла использовать и улучшать их совместно.

Project subagents обнаруживаются путём прохода вверх от текущей рабочей директории, поэтому каждый `.claude/agents/` между ней и корнем репозитория сканируется. Когда более одной из этих вложенных директорий определяет одно и то же `name`, Claude Code использует определение, ближайшее к рабочей директории.

Когда вы добавляете директорию с помощью `--add-dir` или `/add-dir`, Claude Code также загружает её папку `.claude/agents/` вместе с project subagents. См. [Additional directories](/docs/ru/permissions#additional-directories-grant-file-access-not-configuration) для того, какие другие типы конфигурации загружаются из `--add-dir`. Чтобы поделиться subagents в проектах без `--add-dir`, используйте `~/.claude/agents/` или [plugin](/docs/ru/plugins/overview).

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

Флаг `--agents` принимает JSON с полем `prompt` плюс эти поля [frontmatter](#supported-frontmatter-fields): `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd` и `isolation`. Используйте `prompt` для системного приглашения, эквивалентного телу markdown в файловых subagents. `color` и `experimental` не принимаются здесь и игнорируются, а не отклоняются.

Каждый ключ верхнего уровня в JSON — это имя агента. Не начинайте имя с `-`.

Для того, что Claude Code делает со значением, которое он не может загрузить, и флагов и переменной окружения, которые пропускают эту проверку, см. [`Invalid --agents configuration`](/docs/ru/errors#invalid-agents-configuration).

**Managed subagents** развёртываются администраторами организации. Поместите файлы markdown в `.claude/agents/` внутри [директории managed settings](/docs/ru/managed-settings#delivery-mechanisms), используя тот же формат frontmatter, что и project и user subagents. Managed определения имеют приоритет над project и user subagents с тем же именем.

**Plugin subagents** поступают из [plugins](/docs/ru/plugins/overview), которые вы установили. Они загружаются вместе с вашими пользовательскими subagents и появляются в typeahead @-упоминания под их scoped name. См. [справку по компонентам plugin](/docs/ru/plugins/components#agents) для деталей создания plugin subagents.

<Note>
  По соображениям безопасности plugin subagents не поддерживают поля frontmatter `hooks`, `mcpServers` или `permissionMode`. Эти поля игнорируются при загрузке агентов из plugin. Если они вам нужны, скопируйте файл агента в `.claude/agents/` или `~/.claude/agents/`. Вы также можете добавить правила в [`permissions.allow`](/docs/ru/settings-reference#permissions-allow) в `settings.json` или `settings.local.json`, но эти правила применяются ко всей сессии, а не только к plugin subagent.
</Note>

Определения subagent из любой из этих областей также доступны для [agent teams](/docs/ru/agent-teams#use-subagent-definitions-for-teammates): при порождении товарища по команде вы можете ссылаться на тип subagent, и Claude Code применяет части этого определения к товарищу. См. [agent teams](/docs/ru/agent-teams#use-subagent-definitions-for-teammates) для того, какие части применяются в каждом режиме отображения.

<h3 id="write-subagent-files">
  Напишите файлы subagent
</h3>

Файлы subagent используют YAML frontmatter для конфигурации, за которым следует системное приглашение в Markdown:

<Note>
  Claude Code наблюдает за `~/.claude/agents/` и `.claude/agents/`. Когда вы добавляете или редактируете файл subagent на диск, или просите Claude написать его для вас, Claude Code обнаруживает изменение в течение нескольких секунд и следующее делегирование использует обновленное определение без необходимости перезагрузки.

  Три случая по-прежнему требуют перезагрузки:

  * Наблюдатель охватывает только директории, которые существовали при запуске сессии, поэтому после создания первого файла агента области в новой директории `agents`, перезагрузитесь для его загрузки.
  * Claude Code не наблюдает `.claude/agents/` внутри директорий, добавленных с помощью `--add-dir` или `/add-dir`, поэтому после добавления или редактирования subagent там, перезагрузитесь для загрузки изменения.
  * Сессии, запущенные с `--disable-slash-commands`, вообще не наблюдают эти директории.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Frontmatter определяет метаданные и конфигурацию subagent. Тело становится системным приглашением, которое направляет поведение subagent. Subagents получают только это системное приглашение плюс базовые детали окружения, такие как рабочая директория, а не системное приглашение Claude Code.

В [non-interactive mode](/docs/ru/headless) передайте [`--append-subagent-system-prompt`](/docs/ru/cli-reference#cli-flags) для добавления вашего текста в конец системного приглашения каждого subagent, включая вложенные subagents, кроме [forked subagent](#fork-the-current-conversation), который переиспользует приглашение разговора. Требует Claude Code v2.1.205 или позже. Если ваш текст слишком длинный для передачи в командной строке, сохраните его в файл и передайте путь с помощью `--append-subagent-system-prompt-file` вместо этого. Флаг файла требует Claude Code v2.1.261 или позже.

Subagent начинает работу в текущей рабочей директории основного разговора. В пределах subagent команды `cd` не сохраняются между вызовами инструментов Bash или PowerShell и не влияют на рабочую директорию основного разговора. Чтобы дать subagent изолированную копию репозитория вместо этого, установите [`isolation: worktree`](#supported-frontmatter-fields).

Subagent с `isolation: worktree` запускает свои команды Bash и PowerShell внутри своего worktree. Команда, рабочая директория которой разрешается в вашу основную копию вместо этого, например потому что директория worktree была удалена во время работы subagent, завершается с ошибкой. До версии 2.1.203 такая команда могла запуститься в основной копии.

Эта проверка рабочей директории охватывает весь репозиторий, содержащий директорию, из которой вы запустили Claude Code. Когда ваша сессия работает в связанном [worktree](/docs/ru/worktrees) своём собственном, проверка также охватывает основную копию, из которой этот worktree связан. До версии 2.1.210 проверка охватывала только саму директорию запуска. Команда, рабочая директория которой разрешалась в другом месте в том же репозитории, такая как корень репозитория, когда вы запустили Claude Code из подпапки monorepo, запускалась там вместо того, чтобы не выполняться.

Для команд Bash Claude Code также проверяет саму команду двумя способами:

* Он блокирует команду, которая перенаправляет git в основную копию.
* Он отказывает в команде, когда он не может проверить из текста команды, что любой git, который команда запускает, остаётся внутри worktree, например когда имя команды вычисляется во время выполнения.

Векторы перенаправления и правила формы перечислены в разделе [How Claude Code enforces isolation](/docs/ru/worktrees#how-claude-code-enforces-isolation). Команды PowerShell получают только проверку рабочей директории.

Команды [Monitor](/docs/ru/tools-reference#monitor-tool) проходят через те же проверки рабочей директории и содержимого команды, что и команды Bash.

Когда основной разговор сам работает изолированно в worktree, Claude Code применяет те же проверки к сессии и к каждому subagent, который он порождает, включая subagents без `isolation: worktree`; см. [How Claude Code enforces isolation](/docs/ru/worktrees#how-claude-code-enforces-isolation).

<h3 id="supported-frontmatter-fields">
  Справочник Frontmatter
</h3>

Настройте subagent с помощью YAML [frontmatter](/docs/ru/glossary#frontmatter) между маркерами `---` в верхней части его файла, и напишите его системное приглашение как Markdown после закрывающего `---`. Требуются только `name` и `description`.

Многословные имена полей используют camelCase, такие как `maxTurns` и `disallowedTools`, и должны точно совпадать с таблицей: Claude Code игнорирует поле, которое он не распознаёт, без сообщения об ошибке. Чтобы узнать, почему файл subagent не загрузился, см. [Subagent files Claude Code skips](#subagent-files-claude-code-skips).

| Field             | Требуется | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :---------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Да        | Уникальный идентификатор, такой как `code-reviewer` или `reviewer-v2`. [Hooks](/docs/ru/hooks#subagentstart) получают это значение как `agent_type`. Имя файла не должно совпадать. Имена не могут содержать `:`, который зарезервирован для [plugin-scoped identifiers](/docs/ru/plugins/overview) таких как `my-plugin:reviewer`. Claude Code не загружает файл, чьё имя содержит один, и регистрирует ошибку в журнал отладки. До версии 2.1.218 такие имена были приняты                                                     |
| `description`     | Да        | Когда Claude должен делегировать этому subagent                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `tools`           | Нет       | [Инструменты](#available-tools), которые может использовать subagent, как строка, разделённая запятыми, такая как `Read, Grep, Bash` или список YAML. Наследует каждый инструмент, доступный для subagents, если опущено. Если ни один элемент в списке не разрешается в инструмент, subagent обычно [не запускается](/docs/ru/errors#agent-would-be-spawned-with-zero-tools) с ошибкой, называющей элементы. Чтобы предварительно загрузить Skills в контекст, используйте поле `skills` вместо перечисления `Skill` здесь |
| `disallowedTools` | Нет       | Инструменты для запрета, удалённые из унаследованного или указанного списка. Тот же формат, что и `tools`. Запись со спецификатором, такая как `Bash(git push *)`, по-прежнему [удаляет весь инструмент](#available-tools)                                                                                                                                                                                                                                                                                             |
| `model`           | Нет       | [Модель](#choose-a-model) для использования: `sonnet`, `opus`, `haiku`, `fable`, полный ID модели, такой как `claude-opus-5-5`, или `inherit`. Когда вы опускаете его, Claude Code выбирает модель в [subagent model order](#choose-a-model)                                                                                                                                                                                                                                                                           |
| `permissionMode`  | Нет       | [Режим разрешений](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, или `manual` как псевдоним для `default`. Псевдоним `manual` требует Claude Code v2.1.200 или позже. Игнорируется для [plugin subagents](#choose-the-subagent-scope)                                                                                                                                                                                                                                  |
| `maxTurns`        | Нет       | Максимальное количество агентских ходов перед остановкой subagent. Когда subagent достигает лимита, Claude Code возвращает его вывод, отмеченный как частичный, и Claude может [возобновить его](#resume-subagents) для продолжения. Частичная маркировка требует Claude Code v2.1.246 или позже                                                                                                                                                                                                                       |
| `skills`          | Нет       | [Skills](/docs/ru/skills) для предварительной загрузки в контекст subagent при запуске. Полное содержимое skill инжектируется, а не просто описание. Subagents по-прежнему могут вызывать неперечисленные project, user и plugin skills через инструмент Skill                                                                                                                                                                                                                                                              |
| `mcpServers`      | Нет       | [MCP servers](/docs/ru/mcp) доступные этому subagent. Каждая запись — это либо имя сервера, ссылающееся на уже настроенный сервер (например, `"slack"`), либо встроенное определение с именем сервера в качестве ключа и полной [конфигурацией MCP server](/docs/ru/mcp#installing-mcp-servers) в качестве значения. Игнорируется для [plugin subagents](#choose-the-subagent-scope)                                                                                                                                             |
| `hooks`           | Нет       | [Lifecycle hooks](#define-hooks-for-subagents) в области этого subagent. Игнорируется для [plugin subagents](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                                                                                                               |
| `memory`          | Нет       | [Область постоянной памяти](#enable-persistent-memory): `user`, `project` или `local`. Включает кросс-сессионное обучение                                                                                                                                                                                                                                                                                                                                                                                              |
| `background`      | Нет       | Установите на `true`, чтобы держать этот subagent в фоне даже когда Claude просит запустить его на переднем плане. Где [fork mode](#turn-fork-mode-on-or-off) включен, Claude Code уже запускает subagents, которые Claude порождает, [в фоне](#run-subagents-in-foreground-or-background)                                                                                                                                                                                                                             |
| `omitClaudeMd`    | Нет       | Установите на `true`, чтобы запустить этот subagent без файлов user, project и local CLAUDE.md; [managed policy files](/docs/ru/memory#how-claude-md-files-load) по-прежнему загружаются, кроме [managed subagents](#choose-the-subagent-scope). Используйте это для subagents, которые берут всё необходимое из [delegation prompt](#what-loads-at-startup). Игнорируется, когда агент работает как основной агент сессии через `--agent` или параметр `agent`. Требует Claude Code v2.1.271 или позже                     |
| `effort`          | Нет       | Уровень усилий, когда этот subagent активен. Переопределяет уровень усилий сессии. По умолчанию: наследуется из сессии. Параметры: `low`, `medium`, `high`, `xhigh`, `max`; доступные уровни зависят от модели                                                                                                                                                                                                                                                                                                         |
| `isolation`       | Нет       | Установите на `worktree`, чтобы запустить subagent во временном [git worktree](/docs/ru/worktrees), дав ему изолированную копию репозитория, разветвлённую по умолчанию от вашей [ветки по умолчанию](/docs/ru/worktrees#choose-the-base-branch), а не от `HEAD` родительской сессии. Worktree автоматически очищается, если subagent не вносит изменения                                                                                                                                                                        |
| `color`           | Нет       | Цвет отображения для subagent в списке задач и транскрипте. Принимает `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` или `cyan`                                                                                                                                                                                                                                                                                                                                                                          |
| `initialPrompt`   | Нет       | Автоматически отправляется как первый ход пользователя, когда этот агент работает как основной агент сессии (через `--agent` или параметр `agent`). [Commands](/docs/ru/commands) и [skills](/docs/ru/skills) обрабатываются. Добавляется в начало любого предоставленного пользователем приглашения. Игнорируется для [plugin subagents](#choose-the-subagent-scope)                                                                                                                                                            |
| `experimental`    | Нет       | Карта экспериментальных опций. Установите её ключ `cacheTtl` на `5m` или `1h` для выбора [lifetime кэша приглашения](/docs/ru/prompt-caching#choose-the-ttl-yourself) для запросов этого subagent, в месте frontmatter в [cache lifetime precedence](/docs/ru/prompt-caching#choose-the-ttl-yourself). Claude Code игнорирует любое другое значение, игнорирует `1h` пока ваша подписка Claude использует кредиты использования, и читает поле только из файлов subagent. Требует Claude Code v2.1.248 или позже                 |

Напишите `cacheTtl` внутри карты `experimental`, а не на верхнем уровне frontmatter.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Файлы subagent, которые Claude Code пропускает
</h4>

Claude Code пропускает файл в директории project, user или managed `agents`, или в одной под директорией, которую вы добавляете с помощью `--add-dir`, без сообщения об этом в сессии, когда frontmatter имеет любую из этих проблем:

* **Нет `name`**: Claude Code рассматривает файл как документацию, хранящуюся рядом с вашими агентами.
* **Открывающий `---`, который не является первой строкой файла**: Claude Code читает файл как не имеющий frontmatter и рассматривает его как документацию.
* **`name`, который начинается с `-` или содержит `:`**: Claude Code пропускает файл и записывает ошибку в журнал отладки. См. строку `name` в таблице выше.
* **`name`, но нет `description`**: Claude Code пропускает файл и записывает причину в журнал отладки.
* **YAML, который не парсится**: Claude Code не читает никакие поля из файла, пропускает его и записывает ошибку парсинга в журнал отладки.

Чтобы увидеть журнал отладки, запустите Claude Code с `--debug`.

[Plugin subagent](/docs/ru/plugins/components#agents), чей frontmatter не имеет `name` или не парсится, по-прежнему загружается под его именем файла.

<h5 id="check-an-agents-directory-before-a-session">
  Проверьте директорию `agents` перед сессией
</h5>

Чтобы найти файлы в директории `agents`, чей frontmatter не парсится, запустите `claude plugin validate` против директории, например `.claude/agents` или `~/.claude/agents`. Claude Code проверяет только [директорию, которую вы называете](/docs/ru/plugins/cli-reference#validate-a-directory), и не помечает файл, чей frontmatter парсится, но не имеет `name`. Требует Claude Code v2.1.233 или позже.

<h3 id="choose-a-model">
  Выберите модель
</h3>

Поле `model` контролирует, какую модель использует subagent:

* **Model alias**: используйте один из доступных псевдонимов: `sonnet`, `opus`, `haiku` или `fable`
* **Full model ID**: используйте полный ID модели, такой как `claude-opus-5-5` или `claude-sonnet-5`. Принимает те же значения, что и флаг `--model`
* **inherit**: используйте ту же модель, что и основной разговор

Когда Claude вызывает subagent, он также может передать параметр `model` для этого конкретного вызова. Claude Code разрешает модель subagent в этом порядке:

1. Параметр `model` для конкретного вызова
2. Frontmatter `model` определения subagent, где `inherit` выбирает модель основного разговора
3. Переменная окружения [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/ru/model-config#environment-variables), когда вы устанавливаете её на псевдоним модели или ID модели
4. Модель основного разговора

В двух случаях семейный псевдоним, такой как `opus`, в параметре для конкретного вызова или frontmatter разрешается в модель основного разговора вместо [версии, на которую указывает псевдоним](/docs/ru/model-config#model-aliases):

* **Модель основного разговора принадлежит этому семейству**: subagent работает на точной модели основного разговора, включая любой суффикс `[1m]`, поэтому он получает то же [extended context](/docs/ru/model-config#extended-context) окно, что и основной разговор.
* **Claude Code не может определить семейство модели основного разговора, на [поставщике, отличном от Anthropic API](/docs/ru/third-party-integrations)**: это может произойти с [application inference profile ARN](/docs/ru/amazon-bedrock#iam-configuration) на Amazon Bedrock, который Claude Code не разрешил в резервную модель. Этот случай охватывает только псевдоним `opus` и не применяется, когда вы устанавливаете [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/ru/model-config#environment-variables), поскольку `opus` затем разрешается в модель, которую вы установили.

Псевдоним в `CLAUDE_CODE_SUBAGENT_MODEL` всегда разрешается в версию, на которую указывает псевдоним, даже когда он называет семейство основного разговора.

Установка `CLAUDE_CODE_SUBAGENT_MODEL` сама по себе не изменяет модель, на которой работают встроенные subagents Explore и Plan. Чтобы изменить её, см. [Run every subagent on one model](#run-every-subagent-on-one-model).

До версии 2.1.251 `CLAUDE_CODE_SUBAGENT_MODEL` был первым в этом порядке и переопределял как параметр для конкретного вызова, так и frontmatter, включая `model: inherit`.

Установка переменной на `inherit` — это то же самое, что оставить её неустановленной. До версии 2.1.196 это значение заставляло subagents использовать модель основного разговора и игнорировало оба этих источника.

Claude Code проверяет параметр для конкретного вызова, frontmatter и значения переменной окружения на соответствие списку разрешений [`availableModels`](/docs/ru/model-config#restrict-model-selection) вашей организации. Для заблокированного значения он подставляет другую модель:

* Когда заблокированное значение — это семейный псевдоним, такой как `opus`, Claude Code запускает subagent на самой новой версии этого семейства, которое разрешает список разрешений, следуя тем же [правилам подстановки и области поставщика](/docs/ru/model-config#restrict-model-selection), что и `/model`. До версии 2.1.222 Claude Code запускал subagent на унаследованной модели для заблокированного семейного псевдонима также.
* Для любого другого заблокированного значения, на поставщиках, где эта подстановка не работает, или когда список разрешений не разрешает никакую версию семейства, Claude Code запускает subagent на унаследованной модели вместо этого. Если вы установили `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code сначала пробует эту модель, под этими же правилами.

В интерактивных сессиях Claude Code показывает предупреждение, называющее запрошенную модель и модель, на которой работает subagent, для любой подстановки.

Чтобы проверить, на какой модели работает subagent, запустите [`/tasks`](/docs/ru/commands). Claude Code называет модель в строке subagent и добавляет [уровень усилий](/docs/ru/model-config#adjust-effort-level), когда определение subagent или skill, из которого он был разветвлён, устанавливает [`effort`](#supported-frontmatter-fields). Требует Claude Code v2.1.242 или позже.

Параметр `model` для конкретного вызова также применяется, когда subagent [возобновляется или получает последующее сообщение](#resume-subagents), поэтому subagent остаётся на этой модели. До версии 2.1.211 возобновление отбрасывало значение для конкретного вызова и subagent возвращался к полю `model` его определения или, без него, модели основного разговора.

Начиная с версии 2.1.198, subagents также наследуют конфигурацию [extended thinking](/docs/ru/model-config#extended-thinking) основного разговора: если thinking включен в вашей сессии, он включен для subagent, и если он выключен, он остаётся выключенным. Нет параметра thinking для каждого subagent. До версии 2.1.198 subagents запускались с отключённым extended thinking независимо от параметра основного разговора.

<h4 id="run-every-subagent-on-one-model">
  Запустите каждый subagent на одной модели
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` — это значение по умолчанию, поэтому определение subagent или модель, которую передаёт Claude, по-прежнему имеют приоритет над ним. Чтобы применить одну модель к каждому subagent, [teammate](/docs/ru/agent-teams#specify-teammates-and-models) и [workflow agent](/docs/ru/workflows), также установите `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` на `1`. Требует Claude Code v2.1.257 или позже.

* Если вы установили обе переменные, subagents работают на модели в `CLAUDE_CODE_SUBAGENT_MODEL`.
* Если вы установили только `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, subagents работают на модели основного разговора.

Например, чтобы запустить каждый subagent на Haiku, установите обе переменные в блоке `env` [файла настроек](/docs/ru/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

Чтобы проверить, что параметр вступил в силу, запустите [`/tasks`](/docs/ru/commands) пока работает subagent. Строка subagent показывает модель, на которой он работает.

Пока `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` [включен](/docs/ru/env-vars), Claude Code игнорирует поле `model` каждого определения subagent, включая встроенные subagents Explore и Plan, и Claude не может передать модель при запуске subagent. Два вида subagent по-прежнему работают на модели основного разговора:

* [Fork](#fork-the-current-conversation)
* [Skill, который работает в subagent](/docs/ru/skills#run-skills-in-a-subagent) с `model: inherit`

Когда вы устанавливаете только `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, встроенный subagent Explore сохраняет свой [model cap](#built-in-subagents).

<h3 id="control-subagent-capabilities">
  Контролируйте возможности subagent
</h3>

Вы можете контролировать, что могут делать subagents, через доступ к инструментам, режимы разрешений и условные правила.

<h4 id="available-tools">
  Доступные инструменты
</h4>

Subagents наследуют [встроенные инструменты](/docs/ru/tools-reference) и MCP инструменты, доступные в основном разговоре, сужены двумя фильтрами: первый удаляет короткий список инструментов из каждого subagent, и второй уменьшает набор встроенных инструментов для subagents, которые работают в [фоне](#run-subagents-in-foreground-or-background), что является значением по умолчанию. На macOS, Linux и WSL subagent также может получить инструменты Glob и Grep, когда основной разговор их не имеет, как описано в разделе [Glob tool behavior](/docs/ru/tools-reference#glob-tool-behavior). [Forks](#fork-the-current-conversation) пропускают оба фильтра и получают точный пул инструментов основного разговора. Первый фильтр удаляет эти инструменты, даже когда они указаны в поле `tools`:

* `Agent`, когда subagent находится на [depth limit](#let-subagents-spawn-their-own-subagents); в [fork](#fork-the-current-conversation) инструмент остаётся в списке, но возвращает ошибку вместо порождения
* `AskUserQuestion`
* `EndConversation`, который может завершить только основной разговор; см. [EndConversation tool behavior](/docs/ru/tools-reference#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`, если только [`permissionMode`](#permission-modes) subagent не является `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

Второй фильтр применяется к subagents, работающим в фоне. Кроме `Agent` и `ExitPlanMode`, которые следуют условиям первого фильтра везде, где работает subagent, фоновый subagent сохраняет каждый MCP инструмент, но только эти встроенные инструменты: `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage` и `Artifact`, плюс [`SubagentHandback`](/docs/ru/tools-reference) для subagent, который сообщает через него. Claude Code удаляет каждый другой встроенный инструмент из фонового subagent, унаследованный или указанный в поле `tools`, поэтому одно и то же определение может разрешаться в разные инструменты на переднем плане и в фоне. Удаление сообщает об ошибке только в том случае, если после него список `tools` [оказывается пустым](/docs/ru/errors#agent-would-be-spawned-with-zero-tools).

До версии 2.1.280 фоновые subagents не могли использовать `LSP`.

[`ListAgents`](/docs/ru/cross-session-messaging) следует этим фильтрам как любой встроенный инструмент: передний subagent наследует его в сессиях, где включен кросс-сессионный обмен сообщениями, и фоновый subagent его не сохраняет.

Teammates в [agent teams](/docs/ru/agent-teams) дополнительно сохраняют инструменты задач и инструменты cron: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete` и `CronList`.

В [сессии без инструментов Task](/docs/ru/tools-reference#task-tool-availability) Claude Code не предоставляет инструменты задач subagents либо, даже когда subagent запускает другую модель. Встроенный teammate следует вашей сессии так же, пока teammate в своей собственной [split pane](/docs/ru/agent-teams#choose-a-display-mode) работает как отдельный процесс Claude Code, поэтому его собственная модель решает.

Чтобы ограничить инструменты, используйте поле `tools` как список разрешений или поле `disallowedTools` как список запретов. Этот пример использует `tools` для исключительного разрешения Read, Grep, Glob и Bash. Subagent не может редактировать файлы, писать файлы или использовать какие-либо MCP инструменты:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Этот пример использует `disallowedTools` для наследования доступных инструментов, кроме Write и Edit. Subagent сохраняет Bash, MCP инструменты и остальное из своего пула:

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

Если оба установлены, `disallowedTools` применяется первым, затем `tools` разрешается против оставшегося пула. Инструмент, указанный в обоих, удаляется.

Когда ничего в списке `tools` не разрешается в инструмент, например потому что каждая запись неправильно написана или называет инструмент, который недоступен для subagents, Claude Code обычно отказывается запускать subagent и инструмент Agent возвращает ошибку, называющую неразрешённые записи; см. [Agent would be spawned with zero tools](/docs/ru/errors#agent-would-be-spawned-with-zero-tools) для сообщения и как исправить каждую запись. До версии 2.1.208 этот subagent запускался без инструментов и мог вернуть пустой или запутанный результат.

Оба поля принимают паттерны уровня MCP сервера в дополнение к точным названиям инструментов: `mcp__<server>` или `mcp__<server>__*` предоставляет или удаляет каждый инструмент из названного сервера. В `disallowedTools`, `mcp__*` также удаляет каждый MCP инструмент из любого сервера. Этот пример удаляет каждый инструмент из MCP сервера `github`, сохраняя инструменты из других серверов и встроенные инструменты в его пуле:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

Запись `disallowedTools` со спецификатором, такая как `Bash(git push *)`, по-прежнему удаляет весь инструмент из subagent, а не только соответствующие команды. Чтобы сохранить Bash и заблокировать конкретные команды, добавьте [Bash deny rule](/docs/ru/permissions#bash), такую как `Bash(git push *)`, в `permissions.deny` в ваших настройках. Правило применяется к основному разговору и к subagents.

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

Если `Agent` полностью опущен из списка `tools`, агент не может порождать никакие subagents с инструментом Agent.

Синтаксис списка разрешений `Agent(agent_type)` применяется только к агенту, работающему как основной поток с `claude --agent`. В определении subagent перечисление `Agent` в `tools` позволяет этому subagent порождать subagents своего собственного, пока [depth limit](#let-subagents-spawn-their-own-subagents) позволяет это, но любой список типов внутри скобок игнорируется.

<h4 id="scope-mcp-servers-to-a-subagent">
  Область MCP servers для subagent
</h4>

Используйте поле `mcpServers` для предоставления subagent доступа к [MCP](/docs/ru/mcp) серверам, которые недоступны в основном разговоре. Встроенные серверы, определённые здесь, подключаются при запуске subagent, в соответствии с [правилом доверия для папки файла агента](#inline-server-trust), и отключаются при его завершении. Строковые ссылки используют соединение родительской сессии.

<Note>
  Поле `mcpServers` применяется в обоих контекстах, где может работать файл агента:

  * Как subagent, порождённый через инструмент Agent или @-упоминание
  * Как основная сессия, запущенная с [`--agent`](#invoke-subagents-explicitly) или параметром `agent`

  Когда агент является основной сессией, встроенные определения серверов подключаются при запуске вместе с серверами из [`.mcp.json`](/docs/ru/mcp) и файлов настроек, под тем же [правилом доверия для папки файла агента](#inline-server-trust). В `/mcp` удалённый (HTTP или SSE) сервер, который вы использовали раньше, может показать [`cached` статус](/docs/ru/mcp#managing-your-servers) вместо этого; Claude Code подключает его, когда Claude впервые вызывает один из его инструментов.
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

<span id="inline-server-trust" />Claude Code загружает встроенный сервер из файла агента в директории `.claude/agents/` вашего проекта, или в директории `.claude/agents/` директории, добавленной с помощью `--add-dir`, только после того, как вы [доверяете папке, из которой пришёл файл агента](/docs/ru/permissions#what-runs-before-you-trust-a-folder). До версии 2.1.238 Claude Code загружал эти серверы без проверки доверия.

* **Доверие, которое не считается**: доверие родительской папки и автоматическое доверие, которое сессия `-p` или SDK получает для [hooks в файлах настроек](/docs/ru/permissions#what-runs-before-you-trust-a-folder)
* **До тех пор**: Claude Code пропускает каждый встроенный сервер в этом файле агента и записывает точный ключ `projects["<path>"].hasTrustDialogAccepted` для `~/.claude.json` в журнал отладки
* **Директории `--add-dir`**: директория вне репозитория вашего доверенного рабочего пространства нуждается в собственной записи доверия, поскольку её файлы `.claude/agents/` не наследуют доверие вашего рабочего пространства

Claude Code загружает два вида сервера без проверки доверия для папки, из которой пришёл файл агента:

* Имя, которое ссылается на сервер, который вы уже настроили
* Встроенный сервер в файле агента из `~/.claude/agents/`, в одном, который вы передаёте с помощью `--agents` или опции SDK `agents`, или в одном, который управляемые настройки предоставляют

Ограничения MCP, которые применяются к основной сессии, также охватывают серверы, объявленные в frontmatter subagent:

* [`--strict-mcp-config`](/docs/ru/cli-reference) и [`--bare`](/docs/ru/cli-reference)
* [Enterprise управляемая конфигурация MCP](/docs/ru/managed-mcp)
* [`allowedMcpServers` и `deniedMcpServers` политики](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Когда один из них блокирует сервер, Claude Code пропускает его и показывает предупреждение, называющее заблокированные серверы.

Ограничения управляемых настроек применяются к каждому subagent независимо от того, как он определён. `--strict-mcp-config` не фильтрует серверы, которые вы передаёте встроенным образом через `--agents` или опцию SDK `agents`, поскольку это явный ввод вызывающей стороны.

<h4 id="permission-modes">
  Режимы разрешений
</h4>

Установите `permissionMode` для выбора режима разрешений, в котором работает subagent. Используйте значения конфигурации режимов, поэтому режим Manual — это `default`. Если вы оставите его неустановленным, subagent наследует режим основного разговора, который начинается как [auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) на планах Pro, Max и Team, если ваши настройки или ваша организация не изменяют его.

Режим разрешений основного разговора решает, использует ли Claude Code значение, которое вы установили:

* Когда основной разговор находится в `bypassPermissions`, `acceptEdits` или [auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), subagent работает в этом же режиме и Claude Code игнорирует `permissionMode`, который вы установили. Под auto mode классификатор оценивает вызовы инструментов subagent с правилами блокировки и разрешения основного разговора. Когда subagent завершается, классификатор также проверяет его работу и его финальный отчёт перед доставкой отчёта, как [How auto mode handles subagents](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) описывает.
* Когда основной разговор находится в режиме `default`, `dontAsk` или `plan`, subagent работает в режиме разрешений, который вы установили, кроме `bypassPermissions`. Subagent, который объявляет `bypassPermissions`, сохраняет режим основного разговора вместо этого. Исключение `bypassPermissions` требует Claude Code v2.1.267 или позже.

`permissionMode` принимает эти значения и `manual` как псевдоним для `default`:

| Mode                | Behavior                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Режим Manual: запрашивает разрешение                                                                                                                                                                                                                                                                                                                                                                                                           |
| `acceptEdits`       | Автоматически принимать редактирование файлов и общие команды файловой системы для путей в рабочей директории или `additionalDirectories`                                                                                                                                                                                                                                                                                                      |
| `auto`              | [Auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode): классификатор в фоне проверяет команды и записи в защищённые директории                                                                                                                                                                                                                                                                                                    |
| `dontAsk`           | Автоматически отклонять запросы разрешений. Явно разрешённые инструменты по-прежнему работают; `AskUserQuestion`, MCP инструменты, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), и инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) в сессиях, где этот параметр достигает Claude Code, отклоняются, даже если вы их разрешили |
| `bypassPermissions` | [Пропустить запросы разрешений](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode). Subagent работает в этом режиме только когда основной разговор это делает                                                                                                                                                                                                                                                                   |
| `plan`              | Режим Plan (исследование только для чтения)                                                                                                                                                                                                                                                                                                                                                                                                    |

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

Вы не можете предварительно загружать skills, которые устанавливают [`disable-model-invocation: true`](/docs/ru/skills#control-who-invokes-a-skill), поскольку предварительная загрузка берёт из того же набора skills, который Claude может вызывать. Это включает встроенный skill `/verify`: только вы можете его запустить, поэтому он не может быть предварительно загружен либо.

Если указанный skill отсутствует или отключен, например политикой вашей организации, Claude Code пропускает его и регистрирует предупреждение в журнал отладки.

<Note>
  Это противоположность [запуску skill в subagent](/docs/ru/skills#run-skills-in-a-subagent). С `skills` в subagent, subagent контролирует системное приглашение и загружает содержимое skill. С `context: fork` в skill, содержимое skill инжектируется в агента, который вы указываете. В обоих случаях subagent начинает работу без истории вашего разговора.
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

Память subagent является частью [auto memory](/docs/ru/memory#auto-memory): если вы отключите auto memory с помощью параметра `autoMemoryEnabled` или `CLAUDE_CODE_DISABLE_AUTO_MEMORY`, поле `memory` не имеет эффекта и subagent запускается без инструкций памяти или доступа к инструменту памяти, описанного ниже.

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

На macOS и Linux сделайте скрипт исполняемым, или hook не выполнится вместо блокировки чего-либо:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Чтобы протестировать правило, попросите subagent запустить оператор `UPDATE`: скрипт выходит с кодом 2, Claude Code блокирует команду, и subagent видит сообщение `Blocked: Only SELECT queries are allowed`.

См. [Hook input](/docs/ru/hooks#pretooluse-input) для полной схемы входных данных и [exit codes](/docs/ru/hooks#exit-code-output) для того, как коды выхода влияют на поведение. На Windows напишите скрипты hook в PowerShell и добавьте `shell: powershell` к записи hook, как показано в [запуске hooks в PowerShell](/docs/ru/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Отключите конкретные subagents
</h4>

Вы можете предотвратить использование Claude конкретных subagents, добавив их в массив `deny` в ваших [settings](/docs/ru/settings-reference#permission-settings). Используйте формат `Agent(subagent-name)`, где `subagent-name` соответствует полю name subagent.

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

* **В frontmatter subagent**: определите hooks, которые запускаются только во время активности этого subagent
* **В `settings.json`**: определите hooks уровня сессии, которые также срабатывают внутри subagents. События инструментов, такие как `PreToolUse` и `PostToolUse`, срабатывают для вызовов инструментов subagent так же, как они срабатывают в основном разговоре, и `SubagentStart` и `SubagentStop` срабатывают, когда subagent начинает или завершает работу

Hooks из [файлов настроек, управляемых параметров политики и plugins](/docs/ru/hooks#hook-locations) все применяются внутри subagents, поэтому hook `PreToolUse` в `settings.json` также запускается перед каждым инструментом, который использует subagent.

<h4 id="hooks-in-subagent-frontmatter">
  Hooks в frontmatter subagent
</h4>

Определите hooks непосредственно в файле markdown subagent. Эти hooks запускаются только во время активности этого конкретного subagent и очищаются при его завершении.

<Note>
  Frontmatter hooks срабатывают, когда агент порождается как subagent через инструмент Agent или @-упоминание, и когда агент работает как основной агент сессии через [`--agent`](#invoke-subagents-explicitly) или параметр `agent`. В случае основной сессии они запускаются вместе с любыми hooks, определёнными в [`settings.json`](/docs/ru/hooks).
</Note>

Чтобы позволить frontmatter hooks subagent уровня проекта запуститься, примите [диалог доверия рабочего пространства](/docs/ru/permissions#project-allow-rules-and-workspace-trust) для папки, которая содержит файл агента. Hooks из user-level subagents в `~/.claude/agents/` и из определений, которые вы передаёте с помощью `--agents`, запускаются без этого шага. Если вы добавили папку с помощью `--add-dir` из вне репозитория вашего доверенного рабочего пространства, доверьте эту папку отдельно: её hooks `.claude/agents/` не наследуют доверие рабочего пространства.

До тех пор, пока вы не доверяете папке, subagent по-прежнему работает, но Claude Code пропускает его frontmatter hooks и регистрирует ошибку в журнал отладки, объясняющую, как доверить папке. Это более строгое правило, чем для hooks в файлах настроек: доверие родительской папки недостаточно, и сессия `-p` не считается доверенной. [What runs before you trust a folder](/docs/ru/permissions#what-runs-before-you-trust-a-folder) сравнивает эти два. До версии 2.1.218 frontmatter hooks могли запускаться из папок, которым вы не доверяли, включая в non-interactive сессиях.

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

Когда агент вызывается как subagent, hooks `Stop` в frontmatter автоматически преобразуются в события `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks уровня проекта для событий subagent
</h4>

Настройте hooks в `settings.json`, которые реагируют на события жизненного цикла subagent в основной сессии.

| Event           | Matcher input   | Когда это срабатывает               |
| :-------------- | :-------------- | :---------------------------------- |
| `SubagentStart` | Имя типа агента | Когда subagent начинает выполнение  |
| `SubagentStop`  | Имя типа агента | Когда subagent завершает выполнение |

Оба события поддерживают matchers для нацеливания на конкретные типы агентов по имени. Значение matcher — это поле frontmatter `name` для project-level и user-level subagents, или scoped identifier, такой как `my-plugin:db-agent` для [plugin subagents](/docs/ru/plugins/components#agents). Scoped name содержит двоеточие, поэтому он оценивается как [unanchored regular expression](/docs/ru/hooks#matcher-patterns); закрепите его с помощью `^` и `$`, как в `^my-plugin:db-agent$`, чтобы соответствовать только этому агенту.

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

Держите описания краткими: Claude Code показывает предупреждение при запуске, когда объединённые описания ваших subagents превышают [лимит в 15 000 токенов](/docs/ru/errors#agent-descriptions-are-over-the-15000-token-limit), и всё ещё загружает каждый subagent.

Если subagent поставляется в [plugin](/docs/ru/plugins/overview), вы можете измерить, насколько надёжно Claude делегирует ему задачи на реалистичных приглашениях вместо проверки по одному: [`claude plugin eval`](/docs/ru/plugin-evals) запускает каждое приглашение с плагином и без него и оценивает результаты.

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

Subagents, предоставленные включённым [plugin](/docs/ru/plugins/overview), появляются в автодополнении под их именем с областью видимости, например `my-plugin:code-reviewer` или `my-plugin:review:security`, когда plugin [организует агентов в подпапки](#choose-the-subagent-scope). Именованные фоновые subagents, в настоящее время работающие в сессии, также появляются в автодополнении, показывая их статус рядом с именем.

Вы также можете ввести упоминание вручную без использования средства выбора: `@agent-<name>` для локальных subagents или `@agent-` с последующим именем с областью видимости для plugin subagents, например `@agent-my-plugin:code-reviewer`. Пока вы вводите эту форму, автодополнение показывает совпадения файлов, а не агентов. Упоминание агента всё ещё разрешается при отправке.

**Запустите всю сессию как subagent.** Передайте [`--agent <name>`](/docs/ru/cli-reference) для запуска сессии, где основной поток сам принимает системное приглашение, ограничения инструментов и модель этого subagent:

```bash theme={null}
claude --agent code-reviewer
```

Системное приглашение subagent полностью заменяет системное приглашение Claude Code по умолчанию, так же как [`--system-prompt`](/docs/ru/cli-reference) это делает. Файлы `CLAUDE.md` и память проекта по-прежнему загружаются через обычный поток сообщений, даже когда определение агента устанавливает [`omitClaudeMd`](#supported-frontmatter-fields).

Имя агента появляется как `@<name>` в заголовке запуска, чтобы вы могли подтвердить, что он активен.

Это работает с встроенными и пользовательскими subagents, и выбор сохраняется при возобновлении сессии: Claude Code восстанавливает ограничения инструментов и модель агента вместе с разговором. Если агент больше не существует при возобновлении, сессия продолжается с инструментами по умолчанию и показывает [предупреждение с названием агента](/docs/ru/errors#session-agent-no-longer-available). Для системного приглашения в любом случае см. [System prompt flags in resumed conversations](/docs/ru/cli-reference#system-prompt-flags-in-resumed-conversations).

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
* **Background subagents** работают параллельно, пока вы продолжаете работать. Когда фоновый subagent достигает вызова инструмента, требующего разрешения, Claude Code выводит приглашение в вашу основную сессию и называет subagent, который спрашивает. Одобрите, чтобы позволить subagent продолжить, или нажмите Esc, чтобы отклонить этот вызов инструмента без остановки subagent.

Для каждого subagent, который Claude порождает с помощью инструмента Agent, Claude Code выбирает передний план или фон из первого из этих случаев, который применяется:

* Если товарищ [agent team](/docs/ru/agent-teams#limitations) в процессе порождает subagent, Claude Code запускает его в переднем плане. Claude Code отказывает с ошибкой при попытке порождения subagent товарища, чьё определение устанавливает [`background: true`](#supported-frontmatter-fields). Где [fork mode](#turn-fork-mode-on-or-off) отключен и вы не [отключили фоновые задачи](/docs/ru/env-vars), Claude Code также отказывает с ошибкой, когда товарищ устанавливает `run_in_background: true`.
* Если вы установили [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/ru/env-vars) на `1`, Claude Code запускает subagent в переднем плане в каждом виде сессии и независимо от того, включен ли fork mode.
* Где [fork mode](#turn-fork-mode-on-or-off) включен, как это по умолчанию в интерактивной сессии, Claude Code запускает subagent в фоне, как fork, так и non-fork subagents, и Claude не может просить передний план.
* Где fork mode отключен, Claude запускает subagent в фоне по умолчанию и в переднем плане, когда ему нужен результат перед продолжением. Fork mode отключен в [non-interactive mode](/docs/ru/headless) с `-p` и в Agent SDK, если вы его не включите. Чтобы держать конкретный subagent в фоне даже когда Claude хочет результат, установите его frontmatter поле [`background`](#supported-frontmatter-fields) на `true`.

Для навыка с `context: fork`, Claude Code следует правилам в [Run skills in a subagent](/docs/ru/skills#run-skills-in-a-subagent) вместо этого, независимо от того, включен ли fork mode.

Фоновые subagents работают с [меньшим встроенным набором инструментов](#available-tools), чем foreground subagents, за исключением conversation forks и [возобновленных](#resume-subagents) foreground subagents.

Фоновые subagents выводят каждый запрос разрешения в вашу основную сессию. Когда вы ответите на один из этих запросов выбором, который длится дольше этого одного вызова инструмента, например грантом, который длится для остальной сессии, Claude Code применяет ваш ответ ко всей сессии, включая ваш основной разговор.

Фоновый subagent может оставить фоновую [Bash или PowerShell команду](/docs/ru/tools-reference#background-commands) [работающей после конца его хода](/docs/ru/interactive-mode#how-backgrounding-works). Когда эта команда заканчивается, Claude Code отправляет subagent уведомление.

Результаты фонового subagent достигают Claude как уведомление о завершении в более позднем ходе. Claude ждёт этого уведомления перед тем, как сообщить результаты subagent, и если вы сначала спросите о прогрессе, он сообщает, что subagent всё ещё работает. До v2.1.211 Claude иногда сообщал результаты для фонового subagent, который не закончил.

Вы также можете управлять этим самостоятельно:

* Где fork mode отключен, попросите Claude запустить задачу в фоне или в переднем плане
* Нажмите **Ctrl+B** для фонового выполнения работающей задачи

Claude Code очищает строку фонового subagent из панели subagent ниже ввода приглашения одним из двух способов, в зависимости от того, как закончился subagent:

* Когда subagent завершается успешно, Claude Code удаляет его строку немедленно и, кроме как в [screen reader mode](/docs/ru/accessibility), показывает `/tasks to see subagents` в нижнем колонтитуле в течение 30 секунд. В течение этих 30 секунд запустите [`/tasks`](/docs/ru/commands) и нажмите `Enter` на subagent, чтобы открыть его транскрипт. До v2.1.232 Claude Code держал строку в течение 30 секунд после завершения subagent, так же как неудачный, и не показывал подсказку нижнего колонтитула.
* Когда subagent не удаётся или вы его остановили, Claude Code держит его строку в течение 30 секунд. Чтобы очистить строку раньше, выберите её и нажмите `x`.

Фоновый subagent, который завершается, остаётся в списке [`/tasks`](/docs/ru/commands), отмечен как выполненный и отсортирован ниже работающих задач, в течение тех же 30 секунд, что и подсказка нижнего колонтитула. Его представление деталей остаётся открытым, когда subagent завершается. Subagents, которые не удаются или которые вы остановили, покидают список. До v2.1.208 завершённый subagent покидал список в момент завершения и его представление деталей закрывалось.

<h3 id="subagent-names">
  Имена Subagent
</h3>

Claude может дать subagent имя, передав параметр `name` при вызове инструмента Agent, и может сделать это самостоятельно, без предварительного спроса. Имя делает subagent адресуемым: Claude может [отправить сообщение или возобновить его по имени](#resume-subagents) после завершения.

В интерактивной сессии с включенными [agent teams](/docs/ru/agent-teams), subagent, который Claude порождает из основного разговора с `name`, запускается как товарищ вместо этого, если только вызов не является [fork](#fork-the-current-conversation) или не передаёт `isolation` при самом вызове. Значение `isolation` в frontmatter subagent не предотвращает это, и товарищ затем работает в рабочей директории основной сессии. См. [How Claude starts agent teams](/docs/ru/agent-teams#how-claude-starts-agent-teams).

<h3 id="api-errors-in-subagents">
  Ошибки API в subagents
</h3>

Когда что-то [прерывает ответ subagent в середине потока](/docs/ru/errors#the-response-above-may-be-incomplete), и частичный ответ содержит текст, но не вызовы инструментов, Claude Code приглашает subagent продолжить, а не заканчивать запуск. Это происходит и в интерактивных сессиях. Запуск заканчивается на ошибке только после того, как эти продолжения исчерпаны.

Начиная с v2.1.199, subagent, чей запуск заканчивается ошибкой API, такой как лимит использования или повторяющаяся ошибка сервера, сообщает об этом отказе Claude вместо возврата текста ошибки, как если бы это были результаты subagent. То, что получает Claude, зависит от того, где работал subagent:

* **Foreground**: если лимит скорости, перегрузка или ошибка сервера прерывает subagent, который уже произвёл текстовый выход, инструмент Agent возвращает этот частичный выход с примечанием, что subagent был прерван и не завершил свою задачу. Subagent, который не произвёл ничего, или чьим единственным выходом были вызовы инструментов, завершается с ошибкой [`Agent terminated early due to an API error`](/docs/ru/errors#agent-terminated-early-due-to-an-api-error), за которой следует деталь ошибки. В v2.1.199 лимит скорости, перегрузка или ошибка сервера, которые прервали форму, содержащую только вызовы инструментов, возвращали пустой частичный результат, содержащий только примечание об отключении вместо этого.
* **Background**: subagent помечается как неудачный, и сообщение, которое Claude получает при завершении, называет ошибку API и включает последний выход subagent, поэтому частичная работа не теряется.

Когда вы конфигурируете [fallback model chain](/docs/ru/model-config#fallback-model-chains) и subagent встречает отказ, который цепь покрывает, такой как недоступность его модели, Claude Code переключает subagent на первую модель в цепи, которая принимает запрос. Subagent продолжает работать вместо завершения на ошибке.

После того как основная ошибка API исчезнет, попросите Claude повторить задачу или [возобновить subagent](#resume-subagents).

<h3 id="subagent-output-scanning">
  Сканирование выхода Subagent
</h3>

Claude Code сканирует финальный отчёт каждого subagent перед тем, как Claude его прочитает. Subagent может прочитать файлы, веб-страницы или выход команды, которые вы никогда не проверяли, и текст из этих источников может содержать инструкции, направленные на основной разговор. Сканирование никогда не удаляет и не переформулирует ничего; оно делает два вида изменений, которые вы можете заметить в отчёте:

* **Вставка обратной косой черты**: сканирование вставляет обратную косую черту в текст, который имитирует собственный выход Claude Code, такой как тег `<system-reminder>` или строка, начинающаяся с `Human:` или `Assistant:`, так что имитация читается как обычный текст вместо того, чтобы быть ошибочно принятой за часть разговора.
* **Строка маркера**: сканирование добавляет строку, начинающуюся с `[harness: subagent output matched instruction-shaped pattern(s):`, когда отчёт имитирует тег вроде `<system-reminder>` или упоминает параметры разрешения такие как `bypassPermissions` или `--dangerously-skip-permissions`. Упоминания параметров разрешения получают строку маркера, но сам текст остаётся как написано.

Сканирование не судит, является ли содержание вредоносным, и оно не изменяет то, что инструкция в отчёте может сделать: вызов инструмента, который отчёт приводит Claude к выполнению, всё ещё проходит через [проверки разрешений](/docs/ru/permissions) сессии и [sandboxing](/docs/ru/sandboxing). Это не замена для [ограничения того, что может достичь subagent](#control-subagent-capabilities).

Отчёт, который возвращается Claude как результат subagent, также прибывает под заголовком, отмечающим его как выход subagent. Заголовок указывает, что инструкции или утверждения об одобрении внутри отчёта — это слова subagent и не несут никакого авторитета от вас.

Отчёт [фонового subagent](#run-subagents-in-foreground-or-background) прибывает внутри уведомления о завершении, которое отмечается как автоматизированное событие, а не сообщение от вас.

<Note>
  Сканирование выхода Subagent требует Claude Code v2.1.210 или позже.
</Note>

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

Для работы, которая должна продолжать работать параллельно или не поместится в одно контекстное окно, запустите её в [отдельных сессиях](/docs/ru/agents) и позвольте Claude [передавать результаты между ними](/docs/ru/cross-session-messaging).

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
* Задержка имеет значение. Subagent, который не является [fork](#fork-the-current-conversation), начинает с нуля и может потребовать время для сбора контекста

Используйте **subagents** когда:

* Задача производит подробный выход, который вам не нужен в основном контексте
* Вы хотите применить конкретные ограничения инструментов или разрешений
* Работа самодостаточна и может вернуть резюме

Рассмотрите [Skills](/docs/ru/skills) вместо этого, когда вы хотите переиспользуемые приглашения или рабочие процессы, которые работают в контексте основного разговора, а не в изолированном контексте subagent.

Для быстрого вопроса о чём-то уже в вашем разговоре используйте [`/btw`](/docs/ru/interactive-mode#side-questions-with-%2Fbtw) вместо subagent. Он видит ваш полный контекст, но не имеет доступа к инструментам, и ответ не добавляется в историю.

<h3 id="let-subagents-spawn-their-own-subagents">
  Позвольте subagents порождать собственные subagents
</h3>

По умолчанию subagent может порождать subagents собственные, вплоть до трёх слоёв ниже основного разговора. На лимите глубины Claude Code удерживает инструмент `Agent` от каждого subagent, кроме [fork](#fork-the-current-conversation), поэтому subagent на лимите выполняет свою делегированную работу сам и возвращает одно резюме. Fork на лимите сохраняет `Agent` в своём унаследованном списке инструментов, но инструмент возвращает ошибку вместо порождения.

Вложенные subagents подходят для делегированной задачи, которая сама разбивается на параллельные подзадачи, такие как subagent-рецензент, который отправляет верификатор для каждого обнаружения. В интерактивной сессии только резюме subagent верхнего уровня возвращается вам и промежуточный выход остаётся вне вашего основного разговора: subagent, который запускает фоновые subagents, ждёт их результатов перед завершением. В [non-interactive mode](/docs/ru/headless) и Agent SDK запускающий subagent не ждёт, поэтому вложенный фоновый subagent, который завершается после того, как его запускатель закончился, сообщает вашему основному разговору вместо этого.

Чтобы изменить лимит, установите [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/ru/env-vars) на количество слоёв subagent, которые вы хотите ниже основного разговора. Например, эта запись в [`settings.json`](/docs/ru/settings) ограничивает вложение двумя слоями:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

С этим значением ваши subagents могут делегировать второму слою своих собственных, и этот второй слой не может делегировать дальше. Установите `1` для отключения вложения.

Вложенный subagent конфигурируется так же, как subagent верхнего уровня и разрешается из тех же [областей видимости](#choose-the-subagent-scope). Чтобы предотвратить порождение одного subagent, пока вложение включено, такой как рецензент, который должен оставаться только для чтения, опустите `Agent` из его списка [`tools`](#available-tools) или добавьте его в `disallowedTools`.

Claude Code показывает вложенные subagents как дерево в панели subagent ниже ввода приглашения и отмечает каждую строку, которая всё ещё имеет потомков в панели с подсчётом `(+N)` их. Откройте строку, чтобы увидеть братьев и сестёр этого subagent и прямых потомков с путём обратно к `main`.

<Note>
  Более ранние версии использовали разные значения по умолчанию:

  * **v2.1.172 через v2.1.216**: subagents могли вкладываться по умолчанию, вплоть до пяти слоёв глубины, и лимит не мог быть изменён.
  * **v2.1.217 через v2.1.218**: лимит по умолчанию был один, поэтому subagent не мог порождать свои собственные, если вы не повысили его; v2.1.219 повысил значение по умолчанию до трёх.
</Note>

<h3 id="concurrent-subagent-limit">
  Лимит одновременных subagents
</h3>

Два лимита контролируют использование subagent, каждый со своей собственной переменной: этот останавливает Claude от порождения большего количества subagents, пока слишком много работают, и [лимит глубины](#let-subagents-spawn-their-own-subagents) ограничивает, насколько глубоко вкладываются subagents. Нет лимита на общее количество subagents, которые Claude может порождать в течение сессии.

По умолчанию, когда 20 subagents работают в сессии, порождение другого с инструментом Agent завершается с ошибкой `Concurrent subagent limit reached`, и ошибка говорит Claude не повторять попытку. Порождение снова успешно, когда работающий счётчик падает ниже лимита. Чтобы изменить лимит, установите [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/ru/env-vars) на любое положительное целое число. Сессии с активным [ultracode](/docs/ru/model-config#adjust-effort-level) исключены: лимит не применяется там. Требует Claude Code v2.1.217 или позже.

Лимит блокирует только subagents, которые Claude порождает с инструментом Agent, но другие запуски занимают те же слоты:

* In-session fork, который вы запускаете с [`/subtask`](#fork-the-current-conversation), занимает слот во время работы и никогда не блокируется лимитом.
* [Возобновление subagent](#resume-subagents), который уже завершился, занимает свежий слот без проверки лимита, поэтому возобновления могут толкнуть работающий счётчик выше лимита.

Агенты, которых запускают другие функции, такие как [workflow](/docs/ru/workflows) агенты и [agent team](/docs/ru/agent-teams) товарищи, следуют своим собственным лимитам вместо этого.

<h3 id="manage-subagent-context">
  Управляйте контекстом subagent
</h3>

<h4 id="what-loads-at-startup">
  Что загружается при запуске
</h4>

Каждый subagent начинает со свежего, изолированного контекстного окна. Он не видит историю вашего разговора, навыки, которые вы уже вызвали, или файлы, которые Claude уже прочитал. Claude составляет сообщение делегирования, которое резюмирует задачу, и subagent работает на основе этого. Исключением является [fork](#fork-the-current-conversation), который наследует родительский разговор вместо начала с нуля.

Начальный контекст non-fork subagent содержит:

* **System prompt**: собственное приглашение агента плюс детали окружения, которые добавляет Claude Code, а не системное приглашение Claude Code. Пользовательские subagents определяют свои в [markdown body](#write-subagent-files) или поле `prompt`. Встроенные агенты имеют предопределённые приглашения.
* **Task message**: приглашение делегирования, которое Claude пишет при передаче работы.
* **CLAUDE.md files**: каждый уровень [иерархии CLAUDE.md](/docs/ru/memory#how-claude-md-files-load), который загружает основной разговор, включая `~/.claude/CLAUDE.md`, правила проекта, `CLAUDE.local.md`, управляемые файлы политики и любые [`AGENTS.md` файлы](/docs/ru/memory#agents-md), загруженные как инструкции проекта. Встроенные агенты Explore и Plan пропускают это. Subagent, чьё определение устанавливает [`omitClaudeMd`](#supported-frontmatter-fields), загружает только управляемые файлы политики, или ничего вообще, когда определение поступает из [managed settings](#choose-the-subagent-scope).
* **Git status**: снимок, который Claude Code читает из вашего репозитория при запуске subagent. Отсутствует вне Git репозитория или когда снимок отключен; см. [`includeGitInstructions`](/docs/ru/settings-reference#includegitinstructions). Explore и Plan пропускают это независимо.
* **Preloaded skills**: полное содержание любого навыка, названного в поле [`skills`](#preload-skills-into-subagents) агента. Встроенные агенты не предзагружают навыки.
* **Sibling roster**: системное напоминание, в котором перечислены `main` и каждый другой именованный агент в сессии, каждый является допустимым значением `to` для [`SendMessage`](#resume-subagents). Требует Claude Code v2.1.206 или позже. Реестр появляется только когда инструменты subagent включают `SendMessage` и по крайней мере один другой агент имеет имя, независимо от того, назвал ли его Claude при порождении или он работает как товарищ [agent team](/docs/ru/agent-teams). Это снимок, сделанный при запуске subagent, поэтому агенты, названные позже, не появляются.

Чтобы запустить один из ваших собственных subagents без файлов пользователя, проекта и локального CLAUDE.md, установите [`omitClaudeMd: true`](#supported-frontmatter-fields) в его frontmatter или `--agents` JSON.

Основной разговор по-прежнему имеет ваш полный CLAUDE.md, когда он читает результаты этих subagents, поэтому большинству правил не нужно достигать самого subagent. Если правило должно, такое как "ignore the `vendor/` directory," переформулируйте его в приглашении, которое вы даёте Claude при делегировании.

Вы не можете изменить, какие subagents получают статус git. Только Explore и Plan пропускают его.

Некоторое состояние основного разговора никогда не достигает non-fork subagent:

* **Output style**: subagent запускает собственное системное приглашение, поэтому ваш [output style](/docs/ru/output-styles) не формирует его ответы, кроме как в [fork](#fork-the-current-conversation).
* **Auto memory**: [auto memory](/docs/ru/memory#auto-memory) основного разговора не загружается. Чтобы дать subagent собственную постоянную память, используйте поле [`memory`](#enable-persistent-memory).
* **Context window size**: контекстное окно subagent определяется его собственной моделью, а не родительской. Делегирование модели с меньшим окном даёт этому subagent меньшее окно.

<h4 id="resume-subagents">
  Возобновите subagents
</h4>

Каждый вызов subagent создаёт новый экземпляр, а не продолжает более ранний. Чтобы продолжить работу существующего subagent вместо начала с нуля, попросите Claude возобновить его.

Возобновлённые subagents сохраняют полную историю разговора, включая все предыдущие вызовы инструментов, результаты и рассуждения. Если subagent порождал [фоновые subagents собственные](#let-subagents-spawn-their-own-subagents), эта история включает результаты, которые они доставили, пока он работал. Subagent продолжает ровно там, где он остановился, а не начинает с нуля.

* Когда subagent завершается, Claude получает его agent ID.
* Встроенные агенты Explore и Plan — это одноразовые и не возвращают agent ID, поэтому Claude не может их возобновить. Используйте `general-purpose` или пользовательский subagent, когда вам нужно продолжить работу.
* Когда subagent останавливается на его лимите [`maxTurns`](#supported-frontmatter-fields), Claude Code отмечает возвращённый выход как частичный. Для subagents, которые возвращают agent ID, Claude Code также отмечает в результате, что Claude может отправить сообщение subagent, чтобы продолжить с того места, где он остановился.

Claude использует инструмент `SendMessage` с ID агента или именем агента в качестве поля `to` для возобновления его. `SendMessage` не требует включения [agent teams](/docs/ru/agent-teams); только структурированные сообщения протокола команды, такие как `shutdown_request` и `plan_approval_response`, это требуют. Помимо subagents и товарищей, в сессиях, где включена cross-session messaging, Claude может использовать тот же инструмент для отправки сообщений [вашим другим сессиям Claude Code](/docs/ru/cross-session-messaging), на этой машине или [за её пределами](/docs/ru/cross-session-messaging#message-sessions-on-other-machines).

Чтобы возобновить subagent, попросите Claude продолжить предыдущую работу:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Когда Claude отправляет завершённому subagent сообщение с инструментом `SendMessage`, subagent возобновляется в фоне без новой инвокации `Agent`. То же самое применяется к subagent, который Claude остановил с помощью инструмента `TaskStop`, после того как его остановленный запуск вышел. Возобновленный запуск сохраняет [набор инструментов с того места, где subagent впервые запустился](#run-subagents-in-foreground-or-background) и может продолжить чтение [prompt cache, который исходный запуск разогрел](/docs/ru/prompt-caching#subagents-and-the-cache).

Subagent, который имеет инструмент `SendMessage`, может отправить это сообщение тоже. В интерактивной сессии возобновленный агент затем сообщает обратно subagent, который его возобновил, а не вашему основному разговору. Этот subagent ждёт результата перед завершением своей собственной работы. Когда subagent отправляет сообщение агенту, которому он сообщает, такому как его собственный запускатель, Claude Code возобновляет этого агента без перенаправления его результатов.

Subagent, который вы остановили сами, с `x` в `/tasks` или запросом SDK `stop_task`, не автоматически возобновляется. Если Claude отправит ему сообщение, сообщение отказывается и Claude сообщается, что агент был отменён.

Пока [строка этого subagent всё ещё находится в панели subagent](#run-subagents-in-foreground-or-background), введите в его транскрипт, чтобы возобновить его самостоятельно. После этого сообщение от Claude может автоматически возобновить его снова.

Возобновление начинает новый запуск агента под тем же ID, поэтому subagent, который уже завершился или был неудачным, показывается как работающий снова в списке задач и в событиях задач Agent SDK. До v2.1.205 он продолжал показывать свой более ранний статус неудачи или завершения, пока возобновленный запуск работал.

Начиная с v2.1.199, `SendMessage` проверяет, что имя по-прежнему ссылается на того же агента, которого оно достигло ранее в разговоре. Если более новый агент взял имя, например повторно порождённый фоновый агент, который его переиспользовал, Claude Code отказывает в отправке, а не доставляет его неправильному агенту, и ошибка сообщает, какого агента имя теперь достигает, чтобы Claude мог перенаправить. Чтобы достичь более раннего агента, пока он всё ещё работает, Claude обращается к нему по ID агента, который он получил при порождении этого агента. Проверка ограничена текущим разговором и сбрасывается на `/clear`.

Начиная с v2.1.198, subagent рассматривает сообщения от агента, который его запустил, как нормальное направление задачи, включая коррекции курса во время выполнения, и действует в соответствии с ними в рамках своих собственных параметров разрешения. Два лимита по-прежнему действуют независимо от того, кто отправил сообщение: ни одно сообщение от любого агента не считается вашим одобрением для ожидающего запроса разрешения, и ни один агент не может изменить параметры разрешения subagent, `CLAUDE.md` или конфигурацию. Только система разрешений или ваши собственные сообщения могут предоставить одобрение.

Вы также можете попросить Claude ID агента, если хотите ссылаться на него явно, или найти ID в файлах транскрипта в `~/.claude/projects/{project}/{sessionId}/subagents/`. Каждый транскрипт сохраняется как `agent-{agentId}.jsonl`.

Транскрипты subagent сохраняются независимо от основного разговора:

* **Компактирование основного разговора**: когда основной разговор компактируется, транскрипты subagent не затрагиваются. Они сохраняются в отдельных файлах.
* **Сохранение сессии**: транскрипты subagent сохраняются в пределах их сессии. Вы можете [возобновить subagent](#resume-subagents) после перезагрузки Claude Code, возобновив ту же сессию.
* **Автоматическая очистка**: Claude Code удаляет транскрипты subagent после периода удержания `cleanupPeriodDays`, 30 дней по умолчанию, следуя [правилам очистки](/docs/ru/claude-directory#cleaned-up-automatically).

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
  Запустите разветвленный subagent с помощью `/subtask`, что требует Claude Code v2.1.212 или позже. Когда [представление агента отключено](/docs/ru/agent-view#turn-off-agent-view), `/subtask` недоступен и `/fork` запускает разветвленный subagent вместо этого; в противном случае `/fork` копирует всю сессию в новую [фоновую сессию](/docs/ru/agent-view#from-inside-a-session).
</Note>

Fork — это subagent, который наследует весь разговор до сих пор вместо начала с нуля. Это отбрасывает входную изоляцию, которую subagents иначе предоставляют: fork видит то же системное приглашение, инструменты, модель и историю сообщений, что и основная сессия, поэтому вы можете передать ему побочную задачу без переобъяснения ситуации. Вызовы инструментов fork по-прежнему остаются вне вашего разговора и только его окончательный результат возвращается, поэтому ваше основное контекстное окно остаётся чистым. Используйте fork, когда любой другой subagent потребовал бы слишком много фона, чтобы быть полезным, или когда вы хотите попробовать несколько подходов параллельно с одной и той же отправной точки.

Claude запускает fork, запрашивая тип subagent `fork` через инструмент Agent. Вы контролируете, может ли он это делать, с помощью [режима fork](#turn-fork-mode-on-or-off), который включен по умолчанию в интерактивных сессиях.

Вы можете запустить fork самостоятельно с помощью `/subtask` за которым следует задача, независимо от того, включен ли режим fork или нет. На v2.1.161 через v2.1.211 команда — это `/fork`. Claude Code называет fork из первых слов задачи. Следующий пример разветвляет разговор для черновика тестовых случаев, пока вы продолжаете с реализацией в основной сессии:

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

Fork появляется в панели ниже вашего приглашения и работает в фоне, пока вы продолжаете работать. Когда он завершается, его результат приходит как сообщение в вашем основном разговоре. Следующий раздел охватывает элементы управления панели для наблюдения и управления forks во время их работы.

<h3 id="observe-and-steer-running-forks">
  Наблюдение и управление работающими forks
</h3>

Работающие forks появляются в панели ниже входа приглашения, с одной строкой для основной сессии и одной для каждого fork.

Когда fork завершается успешно, Claude Code удаляет его строку. Claude Code сохраняет строку fork, который не удался или который вы остановили, на 30 секунд, [то же самое, что и для любого другого фонового subagent](#run-subagents-in-foreground-or-background). До v2.1.232 Claude Code также сохранял строку завершённого fork на 30 секунд.

Используйте эти клавиши для взаимодействия с панелью:

| Key       | Action                                                                                                                                                                                                                                    |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓` | Перемещение между строками                                                                                                                                                                                                                |
| `Enter`   | Откройте транскрипт выбранного fork и отправьте ему последующие сообщения                                                                                                                                                                 |
| `x`       | Остановите выбранный fork, если он работает, или отклоните его строку, если он больше не работает. На строке основной сессии или на строке fork, чей транскрипт вы открыли с помощью `Enter`, `x` вводит текст в приглашение вместо этого |
| `Esc`     | Верните фокус на входное приглашение                                                                                                                                                                                                      |

С открытым транскриптом fork или subagent, последующие сообщения и [skills](/docs/ru/skills) идут к этому агенту, но встроенные команды по-прежнему работают в вашем основном разговоре. Начиная с v2.1.199, ввод `/model` или `/fast` в этом представлении показывает уведомление о том, что это изменяет модель основного разговора или режим быстрого выполнения, а не просмотренного агента, вместо того чтобы запускать его молча.

<h3 id="how-forks-differ-from-other-subagents">
  Как forks отличаются от других subagents
</h3>

Fork наследует всё, что основная сессия имеет в момент его порождения. Любой другой subagent начинает с нуля из своего определения.

|                         | Fork                                | Non-fork subagent                                                                                                |
| :---------------------- | :---------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| Context                 | Полная история разговора            | Свежий контекст с приглашением, которое вы передаёте                                                             |
| System prompt and tools | Такие же как основная сессия        | Из [файла определения](#write-subagent-files) subagent, [отфильтрованные для фоновых запусков](#available-tools) |
| Model                   | Такая же как основная сессия        | Из поля `model` subagent                                                                                         |
| Permissions             | Запросы выводятся в вашем терминале | [Запросы выводятся в вашей основной сессии](#run-subagents-in-foreground-or-background) при запуске в фоне       |
| Prompt cache            | Общий с основной сессией            | Отдельный кэш                                                                                                    |

Поскольку системное приглашение fork и определения инструментов идентичны родителю, его первый запрос повторно использует кэш приглашений родителя [prompt cache](/docs/ru/prompt-caching#subagents-and-the-cache). Это делает forking дешевле, чем порождение свежего subagent для задач, которые нуждаются в том же контексте.

Когда Claude порождает fork через инструмент Agent, он может передать `isolation: "worktree"`, чтобы редактирования файлов fork были написаны в отдельный git worktree вместо вашего checkout. Fork не может порождать дальнейшие forks.

<h3 id="turn-fork-mode-on-or-off">
  Включение или отключение режима fork
</h3>

Claude Code включает режим fork по умолчанию в интерактивных сессиях и оставляет его отключённым по умолчанию в [non-interactive mode](/docs/ru/headless) с `-p` и в Agent SDK. Интерактивное значение по умолчанию требует Claude Code v2.1.232 или позже. На более ранних версиях установите `CLAUDE_CODE_FORK_SUBAGENT` на `1`, чтобы включить режим fork.

Вы можете сказать, что режим fork включен, по тому, как Claude Code обрабатывает инструмент Agent:

* Claude может порождать fork, запрашивая тип subagent `fork`. Когда Claude не запрашивает тип, он получает [general-purpose](#built-in-subagents) subagent, если сессия всё ещё имеет этот тип. Subagents, порождённые из определения, такие как Explore, работают как обычно.
* Claude Code запускает subagents, которые Claude порождает, в фоне, forks и non-fork subagents в равной степени, кроме [случаев, которые остаются в переднем плане](#run-subagents-in-foreground-or-background). Claude Code также удаляет параметр `run_in_background` инструмента Agent, поэтому Claude не может просить передний план.

Установите переменную окружения [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/ru/env-vars), чтобы переопределить значения по умолчанию:

* `1` включает режим fork в non-interactive mode и Agent SDK также
* `0` отключает режим fork в каждом виде сессии

Чтобы сохранить режим fork включённым, но остановить Claude от порождения forks, [запретите тип subagent `fork`](#disable-specific-subagents) с помощью правила `Agent(fork)`. Claude Code по-прежнему запускает subagents, которые Claude порождает, в фоне, кроме тех же [случаев, которые остаются в переднем плане](#run-subagents-in-foreground-or-background).

<h2 id="example-subagents">
  Примеры subagents
</h2>

Эти примеры демонстрируют эффективные паттерны для создания subagents. Используйте их как отправные точки или генерируйте настроенную версию с Claude.

<Tip>
  **Best practices:**

  * **Проектируйте сфокусированные subagents:** каждый subagent должен превосходить в одной конкретной задаче
  * **Напишите описания, которые выделяют один subagent:** Claude использует описание для решения о делегировании. Сделайте каждое описание достаточно специфичным для маршрутизации к правильному subagent и держите объединённый набор в пределах [бюджета описания в 15 000 токенов](#understand-automatic-delegation)
  * **Ограничьте доступ к инструментам:** предоставьте только необходимые разрешения для безопасности и сфокусированности
  * **Проверьте в систему контроля версий:** поделитесь project subagents с вашей командой
</Tip>

<h3 id="code-reviewer">
  Code reviewer
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
  Debugger
</h3>

Subagent, который может как анализировать, так и исправлять проблемы. В отличие от code reviewer, этот включает Edit, потому что исправление ошибок требует модификации кода. Приглашение предоставляет чёткий рабочий процесс от диагностики к проверке.

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
  Data scientist
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
  Database query validator
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

Системное приглашение говорит subagent отказывать запросам на запись, поэтому hook является подстраховкой: если subagent попытается выполнить запись в любом случае, Claude Code блокирует команду и subagent видит сообщение `Blocked: Write operations not allowed. Use SELECT queries only.`.

<h2 id="next-steps">
  Следующие шаги
</h2>

Теперь, когда вы понимаете subagents, изучите эти связанные функции:

* [Распространяйте subagents с помощью plugins](/docs/ru/plugins/components#agents) для совместного использования subagents в командах или проектах
* [Запустите Claude Code программно](/docs/ru/headless) с помощью Agent SDK для CI/CD и автоматизации
* [Используйте MCP servers](/docs/ru/mcp) для предоставления subagents доступа к внешним инструментам и данным
