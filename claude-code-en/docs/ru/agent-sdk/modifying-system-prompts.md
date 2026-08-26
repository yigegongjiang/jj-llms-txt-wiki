> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Изменение системных подсказок

> Выберите между предустановкой `claude_code` и пользовательской системной подсказкой, и настройте поведение с помощью CLAUDE.md, стилей вывода, append или полностью пользовательской подсказки.

Системные подсказки определяют поведение Claude, его возможности и стиль ответов. Начните с предустановки `claude_code` для инструментов кодирования, похожих на CLI или IDE, где человек наблюдает и направляет работу. Напишите свою собственную подсказку для агентов с другой поверхностью, идентичностью или моделью разрешений.

На этой странице рассматривается:

* [Как работают системные подсказки](#how-system-prompts-work), с таблицей решений для выбора между предустановкой, предустановкой с `append` и пользовательской подсказкой
* [Настройка поведения агента](#customize-agent-behavior) с файлами CLAUDE.md, стилями вывода, `append` или пользовательской строкой
* [Сравнение четырёх подходов](#compare-the-four-approaches) по постоянству, области действия и тому, что они сохраняют
* [Объединение подходов](#combine-approaches) для наслоения методов настройки вместе

<h2 id="how-system-prompts-work">
  Как работают системные подсказки
</h2>

Системная подсказка — это начальный набор инструкций, который определяет поведение Claude на протяжении всего разговора. Agent SDK имеет три начальные точки для неё:

* **Минимальное значение по умолчанию**: когда вы не устанавливаете `systemPrompt` в TypeScript или `system_prompt` в Python, SDK использует минимальную подсказку, которая охватывает вызов инструментов, но исключает рекомендации по кодированию Claude Code, стиль ответов и контекст проекта. Это отличается от `claude -p`, который по умолчанию использует полную подсказку Claude Code. Если вы переходите с CLI и хотите совпадающее поведение, установите предустановку `claude_code`.
* **Предустановка `claude_code`**: полная системная подсказка, которую использует CLI Claude Code, с инструкциями по использованию инструментов, рекомендациями по стилю и форматированию кода, правилами тона и многословности ответов, инструкциями по безопасности и защите, а также контекстом о рабочем каталоге и окружении. Установите `systemPrompt: { type: "preset", preset: "claude_code" }` в TypeScript или `system_prompt={"type": "preset", "preset": "claude_code"}` в Python, опционально с `append` для добавления ваших собственных инструкций в конец.
* **Пользовательская строка**: подсказка, которую вы пишете сами. SDK отправляет только то, что вы предоставляете.

<h3 id="decide-on-a-starting-point">
  Выберите начальную точку
</h3>

Решающий фактор — насколько ваш агент похож на Claude Code: агент кодирования, работающий в репозитории, с человеком, который смотрит потоковый вывод и управляет работой. Чем дальше ваш продукт от этого, тем больше вы захотите написать свою собственную подсказку.

| Что вы создаёте                                                                                                                                      | Используйте                            | Что вы получаете                                                                                                                                                    |
| :--------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Инструмент CLI или IDE-подобный инструмент кодирования, где человек смотрит и управляет, и значения по умолчанию Claude Code — это то, что вам нужно | Предустановка `claude_code`            | Полная подсказка Claude Code: руководство по инструментам, правила безопасности, удобные для терминала ответы, осведомлённость о соглашениях репозитория            |
| Тот же вид инструмента, плюс правила, специфичные для продукта, такие как стандарты кодирования, формат вывода или контекст домена                   | Предустановка `claude_code` с `append` | Всё вышеперечисленное, с вашими инструкциями, добавленными после предустановки. Ничего не удаляется, поэтому это наименее рискованная настройка                     |
| Агент с другой поверхностью, идентичностью или моделью разрешений, или агент без кодирования                                                         | Пользовательская строка подсказки      | Только то, что вы пишете. Вы берёте на себя ответственность за замену руководства по инструментам и инструкций по безопасности, которые вашему агенту всё ещё нужны |
| Тонкий цикл вызова инструментов без персоны агента, где вы предоставляете всё поведение в подсказке пользователя                                     | Без опции `systemPrompt`               | Минимальное значение по умолчанию: поддержка вызова инструментов и ничего больше                                                                                    |

«Отличается от Claude Code» обычно означает одно из следующего:

* **Другая поверхность**: вывод не читается в терминале человеком, который его запустил. Интерфейсы чата, потребители структурированного вывода и автоматизация без кодирования каждый требуют подсказку, которая соответствует тому, как их вывод отображается и проверяется. Автоматизация кодирования без присмотра, такая как задание CI, которое исправляет ошибки lint или проверяет различия, всё ещё соответствует предустановке, потому что сама работа — это то, для чего написана предустановка.
* **Другая идентичность**: агент не должен представлять себя как Claude Code. Бот поддержки, помощник по анализу данных или любой агент, специфичный для домена, нуждается в собственном имени, области и персоне.
* **Другая модель разрешений**: агент работает автономно без одобрения человеком каждого шага или работает с узким набором ресурсов. Подсказка Claude Code предполагает, что человек находится в цикле с доступом к полному набору инструментов.
* **Задачи без кодирования**: большая часть подсказки Claude Code — это руководство по кодированию. Для агентов исследования, контента или операций это руководство конкурирует с инструкциями, которые вам действительно нужны.

[Таблица сравнения](#compare-the-four-approaches) показывает, что сохраняет каждый метод настройки.

<h2 id="customize-agent-behavior">
  Настройка поведения агента
</h2>

Стили вывода, `append` и пользовательская строка подсказки каждый изменяют системную подсказку напрямую. CLAUDE.md идёт другим путём: SDK читает его и внедряет его содержимое в разговор как контекст проекта, а не в системную подсказку, поэтому он формирует поведение наряду с любой выбранной вами системной подсказкой. [Skills](/docs/ru/agent-sdk/skills), [hooks](/docs/ru/agent-sdk/hooks) и [permissions](/docs/ru/agent-sdk/permissions) также формируют поведение вне системной подсказки и рассматриваются на отдельных страницах.

<h3 id="claude-md-files-for-project-level-instructions">
  Файлы CLAUDE.md для инструкций на уровне проекта
</h3>

Файлы CLAUDE.md предоставляют Claude постоянный контекст проекта и инструкции. SDK внедряет их содержимое в разговор, а не в системную подсказку, поэтому они работают с любой конфигурацией системной подсказки. О том, что поместить в CLAUDE.md, где его разместить и как писать эффективные инструкции, см. [How Claude remembers your project](/docs/ru/memory). Этот раздел охватывает то, что специфично для SDK: как загружается CLAUDE.md.

SDK читает CLAUDE.md, когда включен соответствующий источник параметров: `'project'` загружает `CLAUDE.md` или `.claude/CLAUDE.md` из рабочего каталога, а `'user'` загружает `~/.claude/CLAUDE.md`. Параметры `query()` по умолчанию включают оба источника, поэтому CLAUDE.md загружается автоматически. Если вы явно установите `settingSources` в TypeScript или `setting_sources` в Python, включите необходимые источники. Загрузка CLAUDE.md контролируется источниками параметров, а не предустановкой `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Загрузка CLAUDE.md с SDK
</h4>

Чтобы загрузить CLAUDE.md, установите `settingSources` для включения уровня, на котором находится ваш CLAUDE.md. Пример ниже загружает CLAUDE.md на уровне проекта наряду с предустановкой `claude_code`, поэтому Claude имеет как полную подсказку агента кодирования, так и соглашения вашего проекта:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

  async for message in query(
      prompt="Add a new React component for user profiles",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",  # Use Claude Code's system prompt
          },
          setting_sources=["project"],  # Loads CLAUDE.md from project
      ),
  ):
      messages.append(message)

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

CLAUDE.md постоянен во всех сеансах в проекте, общий с вашей командой через git и обнаруживается автоматически без изменений кода. Он не загружается, если вы передаёте пустой массив `settingSources`.

<h3 id="output-styles-for-persistent-configurations">
  Стили вывода для постоянных конфигураций
</h3>

Стили вывода — это сохранённые конфигурации, которые изменяют системную подсказку Claude. Они хранятся как файлы markdown и могут быть переиспользованы в разных сеансах и проектах.

<h4 id="create-an-output-style">
  Создание стиля вывода
</h4>

Стиль вывода — это файл markdown с [frontmatter](/docs/ru/output-styles#frontmatter) для метаданных, за которым следует содержимое подсказки. Сохраните его в `~/.claude/output-styles/` для стиля на уровне пользователя, доступного в каждом проекте, или `.claude/output-styles/` в вашем репозитории для стиля на уровне проекта, который вы можете зафиксировать и поделиться с вашей командой.

По умолчанию пользовательский стиль вывода заменяет инструкции по разработке программного обеспечения предустановки `claude_code` на ваши собственные. Чтобы сохранить их и наложить ваши инструкции сверху, установите `keep-coding-instructions: true` в frontmatter. Сохраняйте их, когда ваш агент всё ещё выполняет работу по разработке программного обеспечения. Оставляйте их, когда вы полностью заменяете роль.

Пример ниже определяет персону рецензента кода, которая сохраняет инструкции кодирования, поскольку проверка кода всё ещё выигрывает от руководства Claude Code по безопасности и качеству кода. Сохраните его как `~/.claude/output-styles/code-reviewer.md`, чтобы сделать его доступным во всех проектах:

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Активация стиля вывода
</h4>

После создания активируйте стили вывода через:

* **CLI**: запустите `/config` и выберите стиль вывода
* **Параметры**: установите `outputStyle` в `.claude/settings.local.json`
* **TypeScript SDK**: установите `outputStyle` внутри встроенного объекта `settings`, передаваемого в `query()`, или укажите `settings` на файл параметров, который его устанавливает. `outputStyle` не является полем `Options` верхнего уровня:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Python SDK не имеет опции для программного выбора стиля вывода. Для развёртываний только с кодом, где вы не можете писать в `.claude/settings.local.json`, используйте `append` или пользовательскую строку подсказки вместо этого.

**Примечание для пользователей SDK:** Стили вывода загружаются, когда вы включаете `settingSources: ['user']` или `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` или `setting_sources=["project"]` (Python) в ваши параметры.

<h3 id="append-to-the-claude_code-preset">
  Добавление к предустановке `claude_code`
</h3>

Вы можете использовать предустановку Claude Code со свойством `append` для добавления ваших пользовательских инструкций при сохранении всей встроенной функциональности.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

  async for message in query(
      prompt="Help me write a Python function to calculate fibonacci numbers",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "Always include detailed docstrings and type hints in Python code.",
          }
      ),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Улучшение кэширования подсказок между пользователями и машинами
</h4>

По умолчанию два сеанса, которые используют одну и ту же предустановку `claude_code` и текст `append`, всё ещё не могут совместно использовать запись кэша подсказок, если они запускаются из разных рабочих каталогов. Это потому, что предустановка встраивает контекст для каждого сеанса в системную подсказку перед вашим текстом `append`: рабочий каталог, является ли это репозиторием git, платформу, активную оболочку, версию ОС и пути автоматической памяти. Любое различие в этом контексте создаёт другую системную подсказку и промах кэша. Содержимое CLAUDE.md не влияет на кэш системной подсказки, потому что SDK внедряет его в разговор, а не в системную подсказку.

Чтобы сделать системную подсказку идентичной во всех сеансах, установите `excludeDynamicSections: true` в TypeScript или `"exclude_dynamic_sections": True` в Python. Контекст для каждого сеанса переходит в первое пользовательское сообщение, оставляя только статическую предустановку и ваш текст `append` в системной подсказке, чтобы идентичные конфигурации совместно использовали запись кэша между пользователями и машинами.

<Note>
  `excludeDynamicSections` требует `@anthropic-ai/claude-agent-sdk` v0.2.98 или позже, или `claude-agent-sdk` v0.1.58 или позже для Python. Это применяется только к форме объекта предустановки и не имеет эффекта, когда `systemPrompt` является строкой.
</Note>

Следующий пример объединяет общий блок `append` с `excludeDynamicSections`, чтобы флот агентов, работающих из разных каталогов, мог переиспользовать одну и ту же кэшированную системную подсказку:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Triage the open issues in this repo",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
              "exclude_dynamic_sections": True,
          },
      ),
  ):
      ...
  ```
</CodeGroup>

**Компромиссы:** рабочий каталог, флаг git-repo, платформа, активная оболочка, версия ОС и пути автоматической памяти всё ещё достигают Claude, но как часть первого пользовательского сообщения, а не системной подсказки. Инструкции в пользовательском сообщении имеют немного меньший вес, чем тот же текст в системной подсказке, поэтому Claude может полагаться на них менее сильно при рассуждении о текущем каталоге или путях автоматической памяти. Включите эту опцию, когда переиспользование кэша между сеансами важнее, чем максимально авторитетный контекст окружения.

Для эквивалентного флага в неинтерактивном режиме CLI см. [`--exclude-dynamic-system-prompt-sections`](/docs/ru/cli-reference).

<h3 id="custom-system-prompts">
  Пользовательские системные подсказки
</h3>

Вы можете предоставить пользовательскую строку как `systemPrompt` для полной замены значения по умолчанию вашими собственными инструкциями.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h2 id="compare-the-four-approaches">
  Сравнение четырех подходов
</h2>

Четыре метода настройки различаются по месту их расположения, способу совместного использования и тому, что они сохраняют из предустановки `claude_code`.

| Функция                      | CLAUDE.md                | Стили вывода                       | `systemPrompt` с добавлением | Пользовательский `systemPrompt` |
| ---------------------------- | ------------------------ | ---------------------------------- | ---------------------------- | ------------------------------- |
| **Постоянство**              | Файл для каждого проекта | Сохранено как файлы                | Только сеанс                 | Только сеанс                    |
| **Переиспользуемость**       | Для каждого проекта      | Между проектами                    | Дублирование кода            | Дублирование кода               |
| **Управление**               | На файловой системе      | CLI + файлы                        | В коде                       | В коде                          |
| **Инструменты по умолчанию** | Сохранены                | Сохранены                          | Сохранены                    | Потеряны (если не включены)     |
| **Встроенная безопасность**  | Поддерживается           | Поддерживается                     | Поддерживается               | Должна быть добавлена           |
| **Контекст окружения**       | Автоматический           | Автоматический                     | Автоматический               | Должен быть предоставлен        |
| **Уровень настройки**        | Только добавления        | Замена или расширение по умолчанию | Только добавления            | Полный контроль                 |
| **Контроль версий**          | С проектом               | Да                                 | С кодом                      | С кодом                         |
| **Область действия**         | Специфично для проекта   | Пользователь или проект            | Сеанс кода                   | Сеанс кода                      |

"С добавлением" означает использование `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` в TypeScript или `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` в Python. CLAUDE.md не изменяет сам системный запрос: SDK внедряет его содержимое в разговор как контекст проекта.

<h2 id="use-cases-and-best-practices">
  Варианты использования и лучшие практики
</h2>

<h3 id="when-to-use-claude-md">
  Когда использовать CLAUDE.md
</h3>

Используйте CLAUDE.md для инструкций, которые должны применяться к каждому сеансу в проекте, независимо от того, какой системный запрос использует сеанс: стандарты кодирования, общие команды, контекст архитектуры и соглашения команды. CLAUDE.md фиксируется в вашем репозитории, поэтому он остается синхронизированным с кодом, который он описывает. Полное руководство см. в разделе [Когда добавлять в CLAUDE.md](/docs/ru/memory#when-to-add-to-claude-md).

Файлы CLAUDE.md загружаются, когда включен источник параметров `project`, что происходит для параметров `query()` по умолчанию. Если вы явно установите `settingSources` в TypeScript или `setting_sources` в Python, включите `'project'` для продолжения загрузки CLAUDE.md на уровне проекта.

<h3 id="when-to-use-output-styles">
  Когда использовать стили вывода
</h3>

Стили вывода предназначены для персон, которые вы хотите повторно использовать в CLI и SDK без изменения кода приложения. Поскольку они находятся в виде файлов в `.claude/output-styles`, одна и та же персона доступна из `/config` в CLI и из любого сеанса SDK, который загружает соответствующий источник параметров.

**Лучше всего подходит для:**

* Постоянных изменений поведения между сеансами
* Конфигураций, общих для команды
* Специализированных помощников, таких как рецензент кода, специалист по данным или DevOps помощник
* Сложных изменений подсказок, которые требуют версионирования

**Примеры:**

* Создание выделенного помощника по оптимизации SQL
* Построение рецензента кода, ориентированного на безопасность
* Разработка помощника по обучению с определенной педагогикой

<h3 id="when-to-use-systemprompt-with-append">
  Когда использовать `systemPrompt` с добавлением
</h3>

Используйте `append`, когда предустановка `claude_code` уже подходит для вашего продукта и вам нужно только добавить дополнительные инструкции. Вы сохраняете руководство инструментов предустановки, правила безопасности и соглашения кодирования без их переимплементации.

**Лучше всего подходит для:**

* Добавления определенных стандартов кодирования или предпочтений
* Настройки форматирования вывода
* Добавления знаний, специфичных для домена
* Изменения многословности ответов
* Улучшения поведения Claude Code по умолчанию без потери инструкций инструментов

<h3 id="when-to-use-custom-systemprompt">
  Когда использовать пользовательский `systemPrompt`
</h3>

Используйте пользовательский запрос, когда поверхность вашего агента, идентичность или модель разрешений отличаются от Claude Code, как описано в разделе [Решите на начальную точку](#decide-on-a-starting-point). Вы определяете полный набор инструкций, включая любое руководство инструментов и правила безопасности, которые требуются вашему агенту.

**Лучше всего подходит для:**

* Полного контроля над поведением Claude
* Специализированных однократных задач
* Тестирования новых стратегий подсказок
* Ситуаций, когда инструменты по умолчанию не требуются
* Построения специализированных агентов с уникальным поведением

<h2 id="combine-approaches">
  Объединение подходов
</h2>

Эти методы можно комбинировать. Постоянный стиль вывода или CLAUDE.md устанавливает долгосрочное поведение, а `append` добавляет инструкции, специфичные для сеанса, поверх них без изменения сохраненной конфигурации.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Объединение стиля вывода с добавлениями, специфичными для сеанса
</h3>

Приведенный ниже пример предполагает, что стиль вывода Code Reviewer уже активен. Блок `append` добавляет области фокуса, специфичные для сеанса, поверх персоны, так что один сеанс проверки может приоритизировать OAuth и хранение токенов без изменения сохраненного стиля вывода:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []

  async for message in query(
      prompt="Review this authentication module",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": """
              For this review, prioritize:
              - OAuth 2.0 compliance
              - Token storage security
              - Session management
              """,
          }
      ),
  ):
      messages.append(message)
  ```
</CodeGroup>

<h2 id="see-also">
  См. также
</h2>

* [Стили вывода](/docs/ru/output-styles): создание, управление и совместное использование стилей вывода для CLI, включая формат файла и места хранения
* [Как Claude запоминает ваш проект](/docs/ru/memory): что поместить в CLAUDE.md, где его разместить и как писать эффективные инструкции проекта
* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript): полный тип `Options`, включая `systemPrompt`, `settingSources` и `settings`
* [Справочник Python SDK](/docs/ru/agent-sdk/python): полный тип `ClaudeAgentOptions`, включая `system_prompt` и `setting_sources`
* [Параметры](/docs/ru/settings): справочник `settings.json`, включая место хранения стилей вывода и другой конфигурации
