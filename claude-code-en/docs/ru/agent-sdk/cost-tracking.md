> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Отслеживание затрат и использования

> Узнайте, как отслеживать использование токенов, оценивать затраты и настраивать кэширование подсказок с помощью Claude Agent SDK.

Claude Agent SDK предоставляет подробную информацию об использовании токенов для каждого взаимодействия с Claude. Это руководство объясняет, как правильно отслеживать использование и понимать отчёты о затратах, особенно при работе с параллельными использованиями инструментов и многошаговыми диалогами.

Полную документацию API см. в [справочнике TypeScript SDK](/docs/ru/agent-sdk/typescript) и [справочнике Python SDK](/docs/ru/agent-sdk/python).

<Warning>
  Поля `total_cost_usd` и `costUSD` являются оценками на стороне клиента, а не авторитетными данными для выставления счётов. SDK вычисляет их локально из таблицы цен, встроенной во время сборки, поэтому они могут отличаться от того, что вам фактически выставляется в счёт, когда:

  * изменяются цены
  * установленная версия SDK не распознаёт модель
  * применяются правила выставления счётов, которые клиент не может смоделировать

  Используйте эти поля для получения информации о разработке и приблизительного бюджетирования. Для авторитетного выставления счётов используйте [API использования и затрат](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) или страницу использования в [Claude Console](https://platform.claude.com/usage). Не выставляйте счета конечным пользователям и не принимайте финансовые решения на основе этих полей.
</Warning>

<h2 id="understand-token-usage">
  Понимание использования токенов
</h2>

SDK TypeScript и Python предоставляют одни и те же данные об использовании с разными названиями полей:

* **TypeScript** предоставляет разбивку токенов по шагам для каждого сообщения помощника (`message.message.id`, `message.message.usage`), стоимость для каждой модели через `modelUsage` в результирующем сообщении и совокупный итог в результирующем сообщении.
* **Python** предоставляет разбивку токенов по шагам для каждого сообщения помощника (`message.usage`, `message.message_id`), стоимость для каждой модели через `model_usage` в результирующем сообщении и накопленный итог в результирующем сообщении (`total_cost_usd` и словарь `usage`).

Оба SDK используют одну и ту же базовую модель затрат и предоставляют одинаковую детализацию. Разница заключается в названии полей и в том, где вложено использование по шагам.

Отслеживание затрат зависит от понимания того, как SDK определяет область действия данных об использовании:

* **Вызов `query()`:** одно вызывание функции `query()` SDK. Один вызов может включать несколько шагов (Claude отвечает, использует инструменты, получает результаты, отвечает снова). Каждый вызов создаёт одно сообщение [`result`](/docs/ru/agent-sdk/typescript#sdkresultmessage) в конце.
* **Шаг:** один цикл запроса/ответа в пределах вызова `query()`. Каждый шаг создаёт сообщения помощника с использованием токенов.
* **Сессия:** серия вызовов `query()`, связанных идентификатором сессии (с использованием опции `resume`). Каждый вызов `query()` в пределах сессии сообщает о своих затратах независимо.

На следующей диаграмме показан поток сообщений от одного вызова `query()` с использованием токенов, сообщаемым на каждом шаге и совокупной оценкой в конце:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Диаграмма, показывающая запрос, создающий два шага сообщений. Шаг 1 имеет четыре сообщения помощника с одинаковым ID и использованием (считать один раз), Шаг 2 имеет одно сообщение помощника с новым ID, и финальное результирующее сообщение показывает предполагаемый total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Каждый шаг создаёт сообщения помощника">
    Когда Claude отвечает, он отправляет одно или несколько сообщений помощника. В TypeScript каждое сообщение помощника содержит вложенное `BetaMessage` (доступное через `message.message`) с `id` и объектом [`usage`](https://platform.claude.com/docs/en/api/messages) с подсчётом токенов (`input_tokens`, `output_tokens`). В Python класс данных `AssistantMessage` предоставляет одни и те же данные непосредственно через `message.usage` и `message.message_id`. Когда Claude использует несколько инструментов в одном ходу, все сообщения в этом ходу имеют одинаковый ID, поэтому дедублируйте по ID, чтобы избежать двойного подсчёта.
  </Step>

  <Step title="Результирующее сообщение предоставляет совокупную оценку">
    Когда вызов `query()` завершается, SDK выдаёт результирующее сообщение с `total_cost_usd` и совокупным `usage`. Это доступно как в TypeScript ([`SDKResultMessage`](/docs/ru/agent-sdk/typescript#sdkresultmessage)), так и в Python ([`ResultMessage`](/docs/ru/agent-sdk/python#resultmessage)). Если вы делаете несколько вызовов `query()` (например, в многошаговой сессии), каждый результат отражает только стоимость этого отдельного вызова. Если вам нужна только предполагаемая сумма, вы можете игнорировать использование по шагам и прочитать это единственное значение.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Получение общей стоимости запроса
</h2>

Результирующее сообщение ([TypeScript](/docs/ru/agent-sdk/typescript#sdkresultmessage), [Python](/docs/ru/agent-sdk/python#resultmessage)) отмечает конец цикла агента для вызова `query()`. Оно включает `total_cost_usd`, совокупную предполагаемую стоимость всех шагов в этом вызове. Это работает как для успешных, так и для ошибочных результатов. Если вы используете сессии для создания нескольких вызовов `query()`, каждый результат отражает только стоимость этого отдельного вызова.

Три поля уровня результата отличаются тем, что они считают, когда агент порождает [подагентов](/docs/ru/agent-sdk/subagents). Используйте `modelUsage` или `model_usage` в Python для учета токенов всего дерева; поле `usage` недосчитывает, как только происходит вложение.

| Field                        | Subagent activity                                                                                                     |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Исключено. Считает только цикл агента верхнего уровня, поэтому токены, потребленные внутри подагентов, не добавляются |
| `total_cost_usd`             | Включено. Считает запросы подагентов наряду с циклом верхнего уровня                                                  |
| `modelUsage` / `model_usage` | Включено. Считает запросы подагентов наряду с циклом верхнего уровня, разбитые по моделям                             |

Следующие примеры перебирают поток сообщений из вызова `query()` и выводят общую стоимость при поступлении сообщения `result`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({ prompt: "Summarize this project" })) {
      if (message.type === "result") {
        console.log(`Total cost: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, it still carried total_cost_usd and the
    // branch above has already run; connection or process failures yield
    // no result message.
    console.error(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      try:
          async for message in query(prompt="Summarize this project"):
              if isinstance(message, ResultMessage):
                  print(f"Total cost: ${message.total_cost_usd or 0}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, it still carried total_cost_usd and the
          # branch above has already run; connection or process failures yield
          # no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="track-per-step-and-per-model-usage">
  Отслеживание использования по шагам и по моделям
</h2>

Примеры в этом разделе используют названия полей TypeScript. В Python эквивалентные поля — это [`AssistantMessage.usage`](/docs/ru/agent-sdk/python#assistantmessage) и `AssistantMessage.message_id` для использования по шагам, и [`ResultMessage.model_usage`](/docs/ru/agent-sdk/python#resultmessage) для разбивки по моделям.

<h3 id="track-per-step-usage">
  Отслеживание использования по шагам
</h3>

Каждое сообщение помощника содержит вложенное `BetaMessage` (доступное через `message.message`) с `id` и объектом `usage` с подсчётом токенов. Когда Claude использует инструменты параллельно, несколько сообщений имеют одинаковый `id` с идентичными данными об использовании. Отслеживайте, какие ID вы уже подсчитали, и пропускайте дубликаты, чтобы избежать завышенных итогов.

<Warning>
  Параллельные вызовы инструментов создают несколько сообщений помощника, чьё вложенное `BetaMessage` имеет одинаковый `id` и идентичное использование. Всегда дедублируйте по ID, чтобы получить точные подсчёты токенов по шагам.
</Warning>

Следующий пример накапливает входные и выходные токены на всех шагах, считая каждый уникальный ID сообщения только один раз:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let totalOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant") {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
        totalOutputTokens += message.message.usage.output_tokens;
      }
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // totals below still reflect the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${totalOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Разбивка использования по моделям
</h3>

Результирующее сообщение включает [`modelUsage`](/docs/ru/agent-sdk/typescript#modelusage), карту имени модели на подсчёты токенов и стоимость для каждой модели. Это полезно, когда вы запускаете несколько моделей (например, Haiku для подагентов и Opus для основного агента) и хотите увидеть, куда идут токены.

Следующий пример запускает запрос и выводит разбивку стоимости и токенов для каждой используемой модели:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type !== "result") continue;

    for (const [modelName, usage] of Object.entries(message.modelUsage)) {
      console.log(`${modelName}: $${usage.costUSD.toFixed(4)}`);
      console.log(`  Input tokens: ${usage.inputTokens}`);
      console.log(`  Output tokens: ${usage.outputTokens}`);
      console.log(`  Cache read: ${usage.cacheReadInputTokens}`);
      console.log(`  Cache creation: ${usage.cacheCreationInputTokens}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, the per-model breakdown above has already
  // printed; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}
```

<h2 id="accumulate-costs-across-multiple-calls">
  Накопление затрат на несколько вызовов
</h2>

Каждый вызов `query()` возвращает свой собственный `total_cost_usd`. SDK не предоставляет итог на уровне сессии, поэтому если ваше приложение делает несколько вызовов `query()` (например, в многошаговой сессии или для разных пользователей), накапливайте итоги самостоятельно.

Следующие примеры запускают два вызова `query()` последовательно, добавляют `total_cost_usd` каждого вызова к текущему итогу и выводят как стоимость за вызов, так и совокупную стоимость:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track cumulative cost across multiple query() calls
  let totalSpend = 0;

  const prompts = [
    "Read the files in src/ and summarize the architecture",
    "List all exported functions in src/auth.ts"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({ prompt })) {
        if (message.type === "result") {
          totalSpend += message.total_cost_usd;
          console.log(`This call: $${message.total_cost_usd}`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, this call's cost was already counted;
      // connection or process failures yield no result message. Continue
      // with the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }

  console.log(`Total spend: $${totalSpend.toFixed(4)}`);
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      # Track cumulative cost across multiple query() calls
      total_spend = 0.0

      prompts = [
          "Read the files in src/ and summarize the architecture",
          "List all exported functions in src/auth.ts",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt):
                  if isinstance(message, ResultMessage):
                      cost = message.total_cost_usd or 0
                      total_spend += cost
                      print(f"This call: ${cost}")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. If
              # the failure was an error result, this call's cost was already
              # counted; connection or process failures yield no result message.
              # Continue with the next prompt.
              print(f"Call failed: {error}")

      print(f"Total spend: ${total_spend:.4f}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="handle-errors-caching-and-token-discrepancies">
  Обработка ошибок, кэширования и расхождений в токенах
</h2>

Для точного отслеживания затрат учитывайте неудачные диалоги, цены на кэшированные токены и случайные несоответствия в отчётах.

<h3 id="resolve-output-token-discrepancies">
  Разрешение расхождений в выходных токенах
</h3>

В редких случаях вы можете заметить разные значения `output_tokens` для сообщений с одинаковым ID. Когда это происходит:

1. **Используйте наибольшее значение:** финальное сообщение в группе обычно содержит точный итог.
2. **Предпочитайте результирующее сообщение:** `total_cost_usd` в результирующем сообщении отражает накопленную оценку SDK на всех шагах, поэтому оно более надёжно, чем суммирование значений по шагам самостоятельно. Это всё ещё оценка и может отличаться от вашего фактического счёта.
3. **Сообщайте о несоответствиях:** подавайте проблемы в [репозитории Claude Code на GitHub](https://github.com/anthropics/claude-code/issues).

<h3 id="track-costs-on-failed-conversations">
  Отслеживание затрат на неудачные диалоги
</h3>

Как успешные, так и ошибочные результирующие сообщения включают `usage` и `total_cost_usd`. Если диалог завершится неудачей на полпути, вы всё равно потребили токены до момента отказа. Всегда читайте данные о затратах из результирующего сообщения независимо от его `subtype`.

<h3 id="track-cache-tokens">
  Отслеживание кэшированных токенов
</h3>

Agent SDK автоматически использует [кэширование подсказок](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) для снижения затрат на повторяющееся содержимое. Вам не нужно самостоятельно настраивать кэширование. Объект использования включает два дополнительных поля для отслеживания кэша:

* `cache_creation_input_tokens`: токены, используемые для создания новых записей кэша (взимаются по более высокой ставке, чем стандартные входные токены).
* `cache_read_input_tokens`: токены, прочитанные из существующих записей кэша (взимаются по сниженной ставке).

Отслеживайте их отдельно от `input_tokens`, чтобы понять экономию от кэширования. В TypeScript эти поля типизированы на объекте [`Usage`](/docs/ru/agent-sdk/typescript#usage). В Python они появляются как ключи в словаре [`ResultMessage.usage`](/docs/ru/agent-sdk/python#resultmessage) (например, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Расширение TTL кэша подсказок до одного часа
</h3>

Записи кэша, написанные SDK, используют TTL в 5 минут по умолчанию, когда вы аутентифицируетесь с помощью ключа API или запускаете на Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry. Если ваша рабочая нагрузка запускает много коротких сессий против одной и той же системной подсказки и контекста с промежутками более 5 минут между ними, кэш истекает между сессиями и каждая новая сессия платит полную входную цену.

Чтобы запросить TTL в 1 час на записи кэша, установите переменную окружения [`ENABLE_PROMPT_CACHING_1H`](/docs/ru/env-vars). Вы можете экспортировать её в окружение вашей оболочки или контейнера, или передать её через `options.env`.

Следующий пример включает TTL в 1 час для агента, работающего на Amazon Bedrock:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions, query
  import asyncio


  async def main():
      options = ClaudeAgentOptions(
          env={
              "CLAUDE_CODE_USE_BEDROCK": "1",
              "ENABLE_PROMPT_CACHING_1H": "1",
          },
      )

      async for message in query(prompt="Summarize this project", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    env: {
      ...process.env,
      CLAUDE_CODE_USE_BEDROCK: "1",
      ENABLE_PROMPT_CACHING_1H: "1",
    },
  };

  for await (const message of query({ prompt: "Summarize this project", options })) {
    console.log(message);
  }
  ```
</CodeGroup>

Записи кэша с TTL в 1 час взимаются по более высокой ставке, чем записи в 5 минут, поэтому включение этого обменивает более высокую стоимость записи на больше чтений из кэша. Подробности см. в [ценах на кэширование подсказок](https://platform.claude.com/docs/en/build-with-claude/prompt-caching). Пользователи подписки Claude уже получают TTL в 1 час автоматически и не нужно устанавливать эту переменную.

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript) - Полная документация API
* [Обзор SDK](/docs/ru/agent-sdk/overview) - Начало работы с SDK
* [Разрешения SDK](/docs/ru/agent-sdk/permissions) - Управление разрешениями инструментов
