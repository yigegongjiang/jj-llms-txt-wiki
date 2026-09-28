> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Изменение системных подсказок

> Выберите между предустановкой `claude_code` и пользовательской системной подсказкой, и настройте поведение с помощью CLAUDE.md, стилей вывода, append или полностью пользовательской подсказки.

Системные подсказки определяют поведение Claude, его возможности и стиль ответов. Начните с предустановки `claude_code` для инструментов кодирования, похожих на CLI или IDE, где человек наблюдает и направляет работу. Напишите свою собственную подсказку для агентов с другой поверхностью, идентичностью или моделью разрешений.

<h2 id="how-system-prompts-work">
  Как работают системные подсказки
</h2>

Системная подсказка — это начальный набор инструкций, который определяет поведение Claude на протяжении всего разговора. Agent SDK имеет три начальные точки для неё:

* **Минимальное значение по умолчанию**: когда вы не устанавливаете `systemPrompt` в TypeScript или `system_prompt` в Python, SDK использует минимальную подсказку, которая охватывает вызов инструментов, но исключает остальное содержимое предустановки `claude_code`, включая её инструкции по безопасности и защите, а также контекст о рабочем каталоге и окружении. Это отличается от `claude -p`, который по умолчанию использует системную подсказку Claude Code. Если вы переходите с CLI и хотите совпадающее поведение, установите предустановку `claude_code`.
* **Предустановка `claude_code`**: системная подсказка, которую использует CLI Claude Code, с инструкциями по использованию инструментов, инструкциями по безопасности и защите, а также контекстом о рабочем каталоге и окружении. Установите `systemPrompt: { type: "preset", preset: "claude_code" }` в TypeScript или `system_prompt={"type": "preset", "preset": "claude_code"}` в Python, опционально с `append` для добавления ваших собственных инструкций в конец.
* **Пользовательская строка**: подсказка, которую вы пишете сами. SDK отправляет только то, что вы предоставляете.

<h3 id="decide-on-a-starting-point">
  Выберите начальную точку
</h3>

Решающий фактор — насколько ваш агент похож на Claude Code: агент кодирования, работающий в репозитории, с человеком, который смотрит потоковый вывод и управляет работой. Чем дальше ваш продукт от этого, тем больше вы захотите написать свою собственную подсказку.

| Что вы создаёте                                                                                                                                      | Используйте                            | Что вы получаете                                                                                                                                                    |
| :--------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Инструмент CLI или IDE-подобный инструмент кодирования, где человек смотрит и управляет, и значения по умолчанию Claude Code — это то, что вам нужно | Предустановка `claude_code`            | Подсказка Claude Code, включая руководство по инструментам, правила безопасности и контекст окружения                                                               |
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

`append` и пользовательская строка подсказки каждый изменяют системную подсказку напрямую, а стиль вывода изменяет инструкции, которые Claude Code дает Claude для каждого ответа. CLAUDE.md идёт другим путём: SDK читает его и внедряет его содержимое в разговор как контекст проекта, поэтому он формирует поведение наряду с любой выбранной вами системной подсказкой. [Skills](/docs/ru/agent-sdk/skills), [hooks](/docs/ru/agent-sdk/hooks) и [permissions](/docs/ru/agent-sdk/permissions) также формируют поведение вне системной подсказки и рассматриваются на отдельных страницах.

<h3 id="claude-md-files-for-project-level-instructions">
  Файлы CLAUDE.md для инструкций на уровне проекта
</h3>

Файлы CLAUDE.md предоставляют Claude постоянный контекст проекта и инструкции. SDK внедряет их содержимое в разговор и оставляет системную подсказку нетронутой, поэтому они работают с любой конфигурацией системной подсказки. О том, что поместить в CLAUDE.md, где его разместить и как писать эффективные инструкции, см. [When to add to CLAUDE.md](/docs/ru/memory#when-to-add-to-claude-md) и остальную часть [How Claude remembers your project](/docs/ru/memory). Этот раздел охватывает то, что специфично для SDK: как загружается CLAUDE.md.

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
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


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

Когда вы запустите любой из примеров, SDK потоком передаёт сообщения по мере работы Claude: системное сообщение инициализации, сообщения помощника, пользовательские сообщения с результатами инструментов и финальное сообщение результата с исходом сеанса.

CLAUDE.md постоянен во всех сеансах в проекте, общий с вашей командой через git и обнаруживается автоматически без изменений кода. Он не загружается, если вы передаёте пустой массив `settingSources`.

<h3 id="output-styles-for-persistent-configurations">
  Стили вывода для постоянных конфигураций
</h3>

Стили вывода — это сохранённые наборы инструкций, которые изменяют роль, тон и формат вывода Claude. Они хранятся как файлы markdown и могут быть переиспользованы в разных сеансах и проектах.

<h4 id="create-an-output-style">
  Создание стиля вывода
</h4>

Стиль вывода — это файл markdown с [frontmatter](/docs/ru/output-styles#frontmatter) для метаданных, за которым следует содержимое подсказки. Сохраните его в `~/.claude/output-styles/` для стиля на уровне пользователя, доступного в каждом проекте, или `.claude/output-styles/` в вашем репозитории для стиля на уровне проекта, который вы можете зафиксировать и поделиться с вашей командой.

Пользовательский стиль вывода опускает инструкции по разработке программного обеспечения предустановки `claude_code` и использует ваши собственные. Чтобы сохранить их и наложить ваши инструкции сверху, установите `keep-coding-instructions: true` в frontmatter. Эти инструкции находятся только в полной системной подсказке Claude Code, поэтому параметр не имеет эффекта в сеансе на более короткой системной подсказке, которую вы включаете или отключаете с помощью [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/ru/env-vars#variables). Сохраняйте их, когда ваш агент всё ещё выполняет работу по разработке программного обеспечения. Опускайте их, когда вы полностью заменяете роль.

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

* **CLI**: запустите `/output-style <style>`, например `/output-style concise`, или запустите `/config` и выберите один. Команда `/output-style` требует Claude Code v2.1.269 или позже.
* **Параметры**: установите `outputStyle` в `.claude/settings.local.json`
* **TypeScript SDK**: установите `outputStyle` внутри встроенного объекта `settings`, передаваемого в `query()`, или укажите `settings` на файл параметров, который его устанавливает. `outputStyle` не является полем `Options` верхнего уровня:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

В Python SDK установите `outputStyle` через опцию `settings`, которая принимает строку JSON, такую как `'{"outputStyle": "Explanatory"}'`, или путь к файлу параметров, который его устанавливает.

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
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


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Улучшение кэширования подсказок между пользователями и машинами
</h4>

По умолчанию два сеанса, которые используют одну и ту же предустановку `claude_code` и текст `append`, всё ещё не могут совместно использовать запись кэша подсказок, если они запускаются из разных рабочих каталогов. Это потому, что предустановка встраивает контекст для каждого сеанса в системную подсказку перед вашим текстом `append`: рабочий каталог, является ли это репозиторием git, платформу, активную оболочку, версию ОС и пути автоматической памяти. Любое различие в этом контексте создаёт другую системную подсказку и промах кэша. Содержимое CLAUDE.md не влияет на кэш системной подсказки, потому что SDK внедряет его в разговор, а не в системную подсказку.

Чтобы сделать системную подсказку идентичной во всех сеансах, установите `excludeDynamicSections: true` в TypeScript или `"exclude_dynamic_sections": True` в Python. Контекст для каждого сеанса переходит в первое пользовательское сообщение, оставляя только статическую предустановку и ваш текст `append` в системной подсказке, чтобы идентичные конфигурации совместно использовали запись кэша между пользователями и машинами.

<Note>
  `excludeDynamicSections` требует `@anthropic-ai/claude-agent-sdk` v0.2.98 или позже, или `claude-agent-sdk` v0.1.58 или позже для Python. Установите его только на форме объекта предустановки. SDK игнорирует его, когда вы передаёте пользовательскую подсказку вместо предустановки; чтобы сохранить пользовательскую подсказку в кэше в TypeScript SDK, см. [Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt).
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
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


  asyncio.run(main())
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

В Python загружайте большую пользовательскую подсказку из файла с помощью `system_prompt={"type": "file", "path": "..."}` вместо передачи её в виде строки. Python SDK передаёт строковую подсказку как один аргумент командной строки подпроцессу CLI, поэтому подсказка, которая превышает лимит длины аргумента ОС, не работает при порождении процесса до отправки любого запроса API. На Linux ошибка — `Argument list too long`. См. [`SystemPromptFile`](/docs/ru/agent-sdk/python#systempromptfile) для пороговых значений платформы и поведения Windows.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Кэширование статической части пользовательской подсказки
</h4>

В TypeScript SDK вы можете передать пользовательскую подсказку как массив строк вместо одной строки, с маркером `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` между статической частью и остальным. Используйте это, когда ваша подсказка объединяет инструкции, которые одинаковы на каждом запросе, с контекстом, который изменяется для каждого запроса, например клиент или билет, который обрабатывает агент. Когда вы передаёте обе части как одну строку, изменение части для каждого запроса изменяет всю системную подсказку, поэтому статические инструкции пропускают кэш тоже. Форма массива недоступна в Python SDK; [`ClaudeAgentOptions`](/docs/ru/agent-sdk/python#claudeagentoptions) перечисляет формы, которые принимает `system_prompt`.

<Note>
  SDK разделяет подсказку только когда он вызывает Claude API напрямую или работает на [Claude Platform on AWS](/docs/ru/claude-platform-on-aws). Во всех остальных конфигурациях, таких как Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или [LLM gateway](/docs/ru/llm-gateway-connect), и когда вы установите [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ru/llm-gateway-protocol#disable-pre-release-capabilities), SDK отправляет всю подсказку как один блок, то же самое, что передача одной строки.
</Note>

Чтобы разделить подсказку, импортируйте `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` из `@anthropic-ai/claude-agent-sdk` и передайте его как собственный элемент массива между двумя частями. SDK отправляет строки перед маркером как один текстовый блок и строки после него как второй блок, каждый со своей собственной точкой разрыва кэша. В примере ниже агент поддержки загружает свои инструкции по сортировке из файла и получает детали об одном билете на каждый запрос, поэтому инструкции остаются в кэше, пока детали билета изменяются:

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/ru/agent-sdk/cost-tracking#track-cache-tokens) описывает поля `cache_creation_input_tokens` и `cache_read_input_tokens` на каждом сообщении результата.

SDK собирает блоки из массива следующим образом:

* SDK объединяет строки с каждой стороны маркера с пустой строкой между ними и удаляет сам маркер, поэтому текст маркера не достигает Claude.
* Если вы включите маркер более одного раза, первый — это разделение и SDK удаляет остальные.
* Если вы опустите маркер, SDK объединяет все строки в один блок, то же самое, что передача одной строки.

С флагами CLI [`--system-prompt` или `--system-prompt-file`](/docs/ru/cli-reference#system-prompt-flags), подсказка — это одна строка, поэтому нет массива для переноса маркера. Включите строку, содержащую только `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__`, между статической и частью для каждого запроса вместо этого. Claude Code разделяет подсказку на первой такой строке на те же два блока и удаляет эту строку. Требует Claude Code v2.1.275 или позже.

В SDK предпочитайте форму массива, которая переносит границу без строки маркера.

<h3 id="change-the-prompt-of-an-existing-session">
  Изменение подсказки существующего сеанса
</h3>

По умолчанию, если вы передаёте другой `append` или пользовательскую подсказку, когда вы возвращаетесь к сеансу с помощью `resume` или `continue`, Claude не видит это на следующем ходу. Claude Code записывает системную подсказку на первом запросе сеанса и переиспользует эту запись до сжатия сеанса. Новый текст вступает в силу после этого сжатия или в новом сеансе.

<h4 id="update-claude’s-instructions-mid-session">
  Обновление инструкций Claude в середине сеанса
</h4>

Если инструкции, которые вы поместили в системную подсказку, должны измениться во время работы сеанса, например потому что ваш пользователь переключил агента в режим только для чтения или отредактировал его конфигурацию в вашем приложении, отправьте новые инструкции в разговор вместо изменения `systemPrompt`:

* **В вашем следующем сообщении**: включите новые инструкции в следующее пользовательское сообщение, которое вы отправляете.
* **Из hook**: верните [`additionalContext`](/docs/ru/hooks#add-context-for-claude) из callback `UserPromptSubmit` или `PostToolUse` [hook](/docs/ru/agent-sdk/hooks#outputs), написанный как фактическое утверждение, такое как "The workspace is now read-only". SDK вставляет текст в разговор в точке, где сработал hook, поэтому записанная подсказка остаётся неизменной.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Отключение записи во время итерации формулировки
</h4>

Пока вы итерируете формулировку подсказки и хотите, чтобы каждое редактирование достигло сеанса, который вы возобновляете, установите `snapshot` на false на форме объекта системной подсказки. Claude Code затем перестраивает подсказку на каждом запросе. Поле доступно на форме preset и custom [`systemPrompt`](/docs/ru/agent-sdk/typescript#options) в TypeScript и [`system_prompt`](/docs/ru/agent-sdk/python#systempromptpreset) в Python, и требует `@anthropic-ai/claude-agent-sdk` v0.3.257 или позже, или `claude-agent-sdk` v0.2.153 или позже.

Сохраняйте запись включённой в production. С записью отключённой, другой `append` или пользовательская подсказка на возобновлённом сеансе достигает Claude на следующем ходу, и этот запрос не может переиспользовать [кэш подсказок](/docs/ru/prompt-caching#how-the-cache-is-organized) сеанса. Где API требует [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), Claude также теряет своё мышление из более ранних ходов.

Вне [cloud sessions](/docs/ru/cloud-environments), если вы запустите Claude Code в [bare mode](/docs/ru/headless#start-faster-with-bare-mode), передав `--bare` через `extraArgs` или установив `CLAUDE_CODE_SIMPLE=1`, запись остаётся отключённой, если вы не установите `snapshot: true`.

Запись `append` или пользовательской подсказки по умолчанию требует Claude Code v2.1.265 или позже, который TypeScript Agent SDK поставляет с v0.3.265 и Python Agent SDK с v0.2.153. До Claude Code v2.1.268 сеансы, которые не [получают флаги функций](/docs/ru/env-vars#features-that-need-feature-flag-fetching), включая сеансы на Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry, перестраивали подсказку на каждом запросе и `snapshot` не имел эффекта.

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

<h2 id="combine-approaches">
  Объединение подходов
</h2>

Подходы могут быть объединены. Постоянный стиль вывода или CLAUDE.md устанавливает долгосрочное поведение, а `append` добавляет инструкции, специфичные для сеанса, поверх без изменения сохраненной конфигурации.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Объединение стиля вывода с дополнениями, специфичными для сеанса
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
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


  asyncio.run(main())
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
