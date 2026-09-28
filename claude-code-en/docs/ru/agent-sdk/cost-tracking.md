> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Отслеживание затрат и использования

> Узнайте, как отслеживать использование токенов, оценивать затраты и настраивать кэширование подсказок с помощью Claude Agent SDK.

Claude Agent SDK предоставляет подробную информацию об использовании токенов для каждого взаимодействия с Claude. Это руководство объясняет, как правильно отслеживать использование и понимать отчеты о затратах, особенно при работе с параллельным использованием инструментов и многошаговыми диалогами.

Полную документацию API см. в [справочнике TypeScript SDK](/docs/ru/agent-sdk/typescript) и [справочнике Python SDK](/docs/ru/agent-sdk/python).

<Warning>
  Поля `total_cost_usd` и `costUSD` являются оценками на стороне клиента, а не авторитетными данными для выставления счетов. SDK вычисляет их локально из таблицы цен, встроенной во время сборки, если только не действует таблица [`modelPricing`](/docs/ru/settings-reference#modelpricing). Они могут отличаться от того, что вам фактически выставляется счет, когда:

  * изменяются цены
  * установленная версия SDK не распознает модель
  * применяются правила выставления счетов, которые клиент не может смоделировать

  Одно правило выставления счетов, которое моделирует SDK, — это [ценообразование на основе местоположения данных](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing). Когда `usage` ответа сообщает `inference_geo: "us"`, SDK умножает цену списка токенов этого ответа на 1,1. Плата за запрос, такая как веб-поиск, не умножается. Требуется TypeScript Agent SDK версии 0.3.239 или более поздней, либо Python Agent SDK версии 0.2.144 или более поздней.

  Используйте эти поля для получения информации о разработке и приблизительного бюджетирования. Для авторитетного выставления счетов используйте [API использования и затрат](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) или страницу использования в [Claude Console](https://platform.claude.com/usage). Не выставляйте счета конечным пользователям и не принимайте финансовые решения на основе этих полей.
</Warning>

<h2 id="understand-token-usage">
  Понимание использования токенов
</h2>

TypeScript и Python SDK предоставляют одни и те же данные об использовании с разными названиями полей:

* **TypeScript** предоставляет разбивку токенов по шагам для каждого сообщения ассистента (`message.message.id`, `message.message.usage`), стоимость для каждой модели через `modelUsage` в результирующем сообщении и совокупный итог в результирующем сообщении.
* **Python** предоставляет разбивку токенов по шагам для каждого сообщения ассистента как `message.usage` и `message.message_id`, стоимость для каждой модели через `model_usage` в результирующем сообщении и совокупный итог в результирующем сообщении как `total_cost_usd`.

Оба SDK используют одну и ту же базовую модель расчета стоимости и предоставляют одинаковую детализацию. Разница заключается в названии полей и в том, где вложено использование по шагам.

Отслеживание стоимости зависит от понимания того, как SDK определяет область действия данных об использовании:

* **Вызов `query()`:** одно вызывание функции `query()` SDK. Один вызов может включать несколько шагов: Claude отвечает, использует инструменты, получает результаты и отвечает снова. Каждый вызов создает одно сообщение [`result`](/docs/ru/agent-sdk/typescript#sdkresultmessage) в конце, за исключением [режима потоковой передачи входных данных](/docs/ru/agent-sdk/streaming-vs-single-mode), где один вызов `query()` содержит несколько ходов пользователя и каждый ход выдает свое собственное сообщение `result`.
* **Шаг:** один цикл запроса/ответа в рамках вызова `query()`. Каждый шаг создает сообщения ассистента с использованием токенов.
* **Сеанс:** серия вызовов `query()`, связанных идентификатором сеанса через опцию `resume`. Результаты возобновленного вызова сообщают об общих расходах сеанса, а не только о расходах этого вызова. См. [Накопление стоимости при нескольких вызовах](#accumulate-costs-across-multiple-calls) для получения информации о том, как переносятся итоги.

На следующей диаграмме показан поток сообщений от одного вызова `query()` с использованием токенов, указанным на каждом шаге и совокупной оценкой в конце:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Диаграмма, показывающая запрос, создающий два шага сообщений. Шаг 1 имеет четыре сообщения ассистента с одинаковым ID и использованием (считать один раз), Шаг 2 имеет одно сообщение ассистента с новым ID, и финальное результирующее сообщение показывает предполагаемый total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Диаграмма, показывающая запрос, создающий два шага сообщений. Шаг 1 имеет четыре сообщения ассистента с одинаковым ID и использованием (считать один раз), Шаг 2 имеет одно сообщение ассистента с новым ID, и финальное результирующее сообщение показывает предполагаемый total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="Каждый шаг создает сообщения ассистента">
    Когда Claude отвечает, он отправляет одно или несколько сообщений ассистента. В TypeScript каждое сообщение ассистента содержит вложенный `BetaMessage` (доступный через `message.message`) с `id` и объектом [`usage`](https://platform.claude.com/docs/en/api/messages) с подсчетом токенов (`input_tokens`, `output_tokens`). В Python класс данных `AssistantMessage` предоставляет одни и те же данные непосредственно через `message.usage` и `message.message_id`. Когда Claude использует несколько инструментов в одном ходе, все сообщения в этом ходе имеют одинаковый ID, поэтому дедублируйте по ID, чтобы избежать двойного подсчета.
  </Step>

  <Step title="Результирующее сообщение предоставляет совокупную оценку">
    Когда вызов `query()` завершается, SDK выдает результирующее сообщение с `total_cost_usd` и совокупным `usage`, типизированное как [`SDKResultMessage`](/docs/ru/agent-sdk/typescript#sdkresultmessage) в TypeScript и [`ResultMessage`](/docs/ru/agent-sdk/python#resultmessage) в Python. Если вам нужна только предполагаемая сумма, вы можете игнорировать использование по шагам и прочитать это единственное значение.

    Если вы делаете несколько независимых вызовов `query()`, каждый результат отражает только стоимость этого отдельного вызова. Вызов, который возобновляет сеанс, также учитывает более ранние расходы сеанса.

    В режиме потоковой передачи входных данных каждый ход выдает свое собственное результирующее сообщение. См. [Отслеживание стоимости в режиме потоковой передачи входных данных](#track-costs-in-streaming-input-mode) для получения информации о том, как читать итоги вызовов в этом режиме.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  Отслеживание затрат в режиме потоковой передачи входных данных
</h2>

В [режиме потоковой передачи входных данных](/docs/ru/agent-sdk/streaming-vs-single-mode) один вызов `query()` содержит несколько ходов пользователя, и каждый ход выдает собственное сообщение результата. Поля результата различаются по области действия:

* **`usage`**: охватывает только этот ход и в его пределах только основной цикл агента, а не какие-либо подагенты, которые он запустил.
* **`total_cost_usd` и `modelUsage`, или `model_usage` в Python**: содержат текущий итог для всего вызова на данный момент, плюс любые расходы, восстановленные при возобновлении вызовом сеанса.

В вызове, где ваше приложение никогда не отправляет `/clear`, `/reset` или `/new`, прочитайте последний результат для итогов вызова, а не суммируйте результаты.

Текущие итоги начинаются заново каждый раз, когда ваше приложение отправляет одну из этих трех команд, и внутри вызова `query()` ничто другое их не сбрасывает. Три результата имеют значение для вашего учета:

* **Собственный результат хода `/clear`**: охватывает только то, что было запущено с момента сброса, и содержит новый `session_id`.
* **Каждый последующий результат**: продолжает отсчет с этого сброса.
* **Последний результат перед каждым `/clear`**: содержит итог для ходов с момента предыдущего сброса.

Чтобы получить итог всего вызова, добавьте последний результат перед каждым `/clear` к финальному результату вызова. Все остальные результаты, включая собственный результат хода `/clear`, заменяются более поздним.

В TypeScript SDK также выдает [`SDKConversationResetMessage`](/docs/ru/agent-sdk/typescript#sdkconversationresetmessage) при каждом сбросе, поэтому вы можете обнаружить сбросы из потока. В Python SDK аналогично выдает `ConversationResetMessage`. До версии Python SDK v0.2.137 итератор Python отбрасывал это сообщение, поэтому в этих версиях считайте сбросы самостоятельно из ходов `/clear`, которые отправляет ваше приложение.

`maxBudgetUsd` (TypeScript) или `max_budget_usd` (Python) считает только расходы самого вызова: итоги, восстановленные из возобновленного сеанса, не учитываются в нем, и `/clear` начинает бюджет заново.

<h2 id="get-the-total-cost-of-a-query">
  Получить общую стоимость запроса
</h2>

Результирующее сообщение, типизированное как [`SDKResultMessage`](/docs/ru/agent-sdk/typescript#sdkresultmessage) в TypeScript и [`ResultMessage`](/docs/ru/agent-sdk/python#resultmessage) в Python, отмечает конец цикла агента для вызова `query()`. Оно включает `total_cost_usd`, совокупную предполагаемую стоимость всех шагов в этом вызове. Вызов, который возобновляет сеанс, также учитывает более ранние расходы сеанса. При чтении значения применяются два предостережения:

* В Python это поле типизировано как опциональное, поэтому проверьте, что оно не равно `None` перед его чтением.
* Результаты успеха и ошибки оба содержат его, хотя финальный результат [сбоя сеанса](#recover-totals-after-a-session-crash) может содержать его обнуленным.

В режиме потоковой передачи входных данных читайте итоги вызовов, как описано в [Отслеживание затрат в режиме потоковой передачи входных данных](#track-costs-in-streaming-input-mode).

Три поля на уровне результата отличаются тем, что они считают, когда агент порождает [подагентов](/docs/ru/agent-sdk/subagents). Используйте `modelUsage` или `model_usage` в Python для учета токенов всего дерева; поле `usage` недосчитывает, как только происходит вложение.

| Поле                         | Активность подагента                                                                                                  |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Исключено. Считает только цикл агента верхнего уровня, поэтому токены, потребленные внутри подагентов, не добавляются |
| `total_cost_usd`             | Включено. Считает запросы подагентов наряду с циклом верхнего уровня                                                  |
| `modelUsage` / `model_usage` | Включено. Считает запросы подагентов наряду с циклом верхнего уровня, разбитые по моделям                             |

В [режиме ввода одного сообщения](/docs/ru/agent-sdk/streaming-vs-single-mode#single-message-input), когда фоновые подагенты все еще работают в конце финального хода, Claude Code ждет их, вплоть до лимита, описанного в [фоновых задачах при выходе](/docs/ru/headless#background-tasks-at-exit), перед отправкой результата. `total_cost_usd`, `duration_api_ms` и `modelUsage` результата, или `model_usage` в Python, включают работу, выполненную во время этого ожидания.

Следующие примеры перебирают поток сообщений из вызова `query()` и выводят общую стоимость, когда приходит сообщение `result`:

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
          # failure was an error result, the branch above has already run;
          # connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Чтобы ограничить, сколько подагенты могут добавить к `total_cost_usd`, установите [лимиты глубины, параллелизма и расходов](/docs/ru/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) на запросе.

<h2 id="track-per-step-and-per-model-usage">
  Отслеживание использования на каждом шаге и для каждой модели
</h2>

Примеры в этом разделе используют имена полей TypeScript. В Python эквивалентные поля — это [`AssistantMessage.usage`](/docs/ru/agent-sdk/python#assistantmessage) и `AssistantMessage.message_id` для использования на каждом шаге, а также [`ResultMessage.model_usage`](/docs/ru/agent-sdk/python#resultmessage) для разбивки по моделям.

<h3 id="track-per-step-usage">
  Отслеживание использования на каждом шаге
</h3>

Каждое сообщение ассистента содержит вложенный `BetaMessage` (доступный через `message.message`) с `id` и объектом `usage` с подсчётом токенов. Когда Claude использует инструменты параллельно, несколько сообщений имеют одинаковый `id` с идентичными данными использования. Отслеживайте, какие ID вы уже подсчитали, и пропускайте дубликаты, чтобы избежать завышенных итогов.

<Warning>
  Дедублицированные значения на каждом шаге точны для входных и кэшированных токенов. Per-step `output_tokens` — это заполнитель, поэтому [читайте выходные токены из сообщения результата](#read-output-tokens-from-the-result-message).
</Warning>

Следующий пример накапливает входные токены на всех шагах, подсчитывая каждый уникальный ID сообщения основного цикла только один раз и пропуская сообщения подагентов, а также читает выходной итог из сообщения результата, который охватывает основной цикл:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let resultOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant" && !message.parent_tool_use_id) {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
      }
    }
    if (message.type === "result") {
      // Per-step output_tokens is a placeholder; the result message
      // carries the accumulated output total.
      resultOutputTokens = message.usage.output_tokens;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // input total below still reflects the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${resultOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Разбивка использования по моделям
</h3>

Сообщение результата включает [`modelUsage`](/docs/ru/agent-sdk/typescript#modelusage), карту имён моделей на подсчёты токенов для каждой модели и стоимость. Это полезно, когда вы запускаете несколько моделей (например, Haiku для подагентов и Opus для основного агента) и хотите увидеть, куда идут токены.

Поле `costBasis` каждой записи указывает, какая таблица цен определила цену последнего запроса этой модели: `list` для цены списка, `managed` для таблицы [`modelPricing`](/docs/ru/settings-reference#modelpricing), или `unknown`, когда ни одна из них не совпала с ID модели. Это поле требует Claude Code версии 2.1.246 или позже.

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
  Накопление затрат при нескольких вызовах
</h2>

Каждый вызов `query()` возвращает `total_cost_usd` в своих результатах. Способ объединения значений зависит от того, используют ли вызовы один сеанс:

* **Независимые вызовы без опции `resume` или `continue`**: каждый результат охватывает только свой собственный вызов, поэтому складывайте итоги самостоятельно, как это делается в примерах ниже.
* **Вызовы, которые возобновляют один и тот же сеанс**: Claude Code сохраняет итоги сеанса в его [транскрипт](/docs/ru/sessions#where-transcripts-are-stored) при нормальном завершении процесса и восстанавливает их, когда последующий вызов возобновляет или разветвляет сеанс. Каждый результат уже включает более ранние расходы сеанса. Прочитайте последний результат для получения общей суммы сеанса; суммирование результатов приводит к двойному подсчёту восстановленных расходов. До версии 2.1.277 сеанс, который вы возобновили через SDK или `claude -p`, начинал свои итоги с нуля, поэтому результаты каждого вызова охватывали только этот вызов.

В режиме потоковой передачи входных данных прочитайте общую сумму каждого вызова, как описано в разделе [Отслеживание затрат в режиме потоковой передачи входных данных](#track-costs-in-streaming-input-mode). Для вызова, который завершился сбоем, см. раздел [Восстановление итогов после сбоя сеанса](#recover-totals-after-a-session-crash).

В следующих примерах выполняются два вызова `query()` последовательно, каждый `total_cost_usd` вызова добавляется к текущему итогу, и выводятся как стоимость для каждого вызова, так и общая стоимость:

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

<h2 id="handle-errors-caching-and-output-token-counts">
  Обработка ошибок, кеширование и подсчет выходных токенов
</h2>

Для точного отслеживания затрат учитывайте количество выходных токенов-заполнителей в сообщениях ассистента, токены, потребленные неудачным разговором, и цены на токены кеша.

<h3 id="read-output-tokens-from-the-result-message">
  Чтение выходных токенов из результирующего сообщения
</h3>

Claude Code создает каждое сообщение ассистента на основе использования, которое API сообщил при начале ответа, поэтому `output_tokens` сообщения — это только количество, которое API сообщил в `message_start`, до того как был сгенерирован ответ. Один ответ API может создать несколько сообщений ассистента, и каждое из них содержит этот же заполнитель.

API сообщает реальное количество выходных данных в конце ответа, и Claude Code добавляет его в результирующее сообщение. Читайте выходные токены из `usage` результата или из `modelUsage` для разбивки по моделям.

Чтобы наблюдать, как растет количество выходных данных ответа во время потоковой передачи, установите `includePartialMessages` или `include_partial_messages` в Python и читайте `usage` из каждого события потока `message_delta`, типизированного как [`SDKPartialAssistantMessage`](/docs/ru/agent-sdk/typescript#sdkpartialassistantmessage) в TypeScript и [`StreamEvent`](/docs/ru/agent-sdk/python#streamevent) в Python.

<h3 id="track-costs-on-failed-conversations">
  Отслеживание затрат при неудачных разговорах
</h3>

Оба результирующих сообщения об успехе и ошибке включают `usage` и `total_cost_usd`; в Python оба поля типизированы как необязательные, поэтому проверьте, что они не равны `None` перед их чтением.

Если разговор прерывается на полпути, вы все равно потребили токены до момента сбоя. Читайте данные о затратах из каждого результирующего сообщения, независимо от того, является ли его `subtype` `success` или одним из подтипов ошибок. На некоторых результатах ошибок `usage` сообщает меньше, чем потратил вызов:

* **`error_during_execution` после [сбоя сеанса](#recover-totals-after-a-session-crash)**: каждое поле затрат может быть обнулено.
* **`error_max_budget_usd`**: `usage` исключает ответ, который превысил бюджет, в то время как `total_cost_usd` и `modelUsage` включают его.

Где у вас есть выбор, учитывайте данные из `total_cost_usd` или `modelUsage` вместо `usage`.

<h3 id="recover-totals-after-a-session-crash">
  Восстановление итогов после сбоя сеанса
</h3>

Когда процесс Claude Code падает, он выдает финальный результат `error_during_execution` и выходит как в режиме одноразового ввода, так и в режиме потокового ввода. Этот результат может содержать обнуленные `usage`, `total_cost_usd` и `modelUsage`, поэтому восстановите итоги вызова из того, что пришло до него. Шаг 1 восстанавливает полные итоги всякий раз, когда существует более ранний результат; резервный вариант на шаге 2 восстанавливает только входные токены и токены кеша основного цикла.

1. Используйте результат хода перед сбоем. В режиме потокового ввода он содержит текущий итог, описанный в [Track costs in streaming input mode](#track-costs-in-streaming-input-mode). Перейдите к шагу 2, если этот результат не может вам помочь:
   * Вызов был одноразовым, поэтому более ранний результат не существует.
   * Сбой произошел на первом ходу.
   * Ход перед сбоем был самим `/clear`, поэтому его результат охватывает только сброс.
2. Вместо этого суммируйте `usage` в сообщениях ассистента, считая каждый ответ API один раз, как это делает пример [Track per-step usage](#track-per-step-usage). В режиме одноразового ввода суммируйте все из них; в режиме потокового ввода суммируйте те, которые пришли после последнего результата. Это дает вам входные токены и токены кеша основного цикла. Использование подагента не восстанавливается таким образом, и выходные токены или стоимость в USD тоже, потому что [выходные токены на шаг — это заполнитель](#read-output-tokens-from-the-result-message).

<h3 id="track-cache-tokens">
  Отслеживание токенов кеша
</h3>

Agent SDK автоматически использует [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) для снижения затрат на повторяющееся содержимое. Вам не нужно самостоятельно настраивать кеширование. Объект использования включает два дополнительных поля для отслеживания кеша:

* `cache_creation_input_tokens`: токены, используемые для создания новых записей кеша (взимаются по более высокой ставке, чем стандартные входные токены).
* `cache_read_input_tokens`: токены, прочитанные из существующих записей кеша (взимаются по сниженной ставке).

Отслеживайте их отдельно от `input_tokens`, чтобы понять экономию кеша. В TypeScript эти поля типизированы на объекте [`Usage`](/docs/ru/agent-sdk/typescript#usage). В Python они появляются как ключи в словаре [`ResultMessage.usage`](/docs/ru/agent-sdk/python#resultmessage) (например, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Расширение TTL кеша подсказок до одного часа
</h3>

Ваши собственные ходы попадают в [основной сегмент TTL разговора](/docs/ru/prompt-caching#which-ttl-each-request-gets), вместе с помощниками, которые Claude Code запускает встроенными с ними. Запросы, которые Claude Code делает вне этого разговора, такие как [подагенты](/docs/ru/agent-sdk/subagents), имеют [отдельное управление TTL](/docs/ru/prompt-caching#choose-the-ttl-yourself).

Записи кеша для ваших собственных ходов используют TTL по умолчанию 5 минут, когда вы аутентифицируетесь с помощью ключа API или запускаете на Amazon Bedrock, платформе агентов Google Cloud, Microsoft Foundry или [Claude Platform on AWS](/docs/ru/claude-platform-on-aws). Если ваша рабочая нагрузка запускает много коротких сеансов с одной и той же системной подсказкой и контекстом с промежутками более 5 минут между ними, кеш истекает между сеансами и каждый новый сеанс платит полную входную цену.

Чтобы запросить TTL в 1 час при записи кеша, установите переменную окружения [`ENABLE_PROMPT_CACHING_1H`](/docs/ru/env-vars). Вы можете экспортировать ее в окружение вашей оболочки или контейнера или передать ее через `options.env`.

Следующий пример включает TTL в 1 час для агента, работающего на Amazon Bedrock. Поскольку он устанавливает `CLAUDE_CODE_USE_BEDROCK`, он требует рабочих учетных данных AWS для [Amazon Bedrock](/docs/ru/amazon-bedrock); без них запрос не выполняется.

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

Записи кеша с TTL в 1 час взимаются по более высокой ставке, чем записи на 5 минут, поэтому включение этого обменивает более высокую стоимость записи на больше чтений из кеша. Подробности см. в [ценах на prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching). На подписке Claude в рамках включенного использования вашего плана вы получаете TTL в 1 час на своих собственных ходах и на некоторых вспомогательных запросах, которые Claude Code делает рядом с ними, без установки этой переменной, и Claude Code снижает эти ходы до TTL в 5 минут, как только вы начинаете использовать [кредиты использования](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans).

`ENABLE_PROMPT_CACHING_1H` запрашивает TTL в 1 час для каждого запроса в обоих сегментах. Чтобы выбрать TTL для каждого сегмента отдельно, используйте эти элементы управления вместо этого. Каждый принимает `5m` или `1h` и имеет приоритет над `ENABLE_PROMPT_CACHING_1H`:

* Основной разговор: переменная окружения [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/ru/env-vars) или параметр [`promptCacheTtl`](/docs/ru/settings-reference#promptcachettl)
* Все остальное: переменная окружения `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` или параметр [`subagentPromptCacheTtl`](/docs/ru/settings-reference#subagentpromptcachettl)

Установка `promptCacheTtl` на `1h` сохраняет кеш в 1 час на основном разговоре, пока вы используете кредиты использования. Для полного порядка приоритета см. [выбор TTL самостоятельно](/docs/ru/prompt-caching#choose-the-ttl-yourself).

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript) - Полная документация API
* [Обзор SDK](/docs/ru/agent-sdk/overview) - Начало работы с SDK
* [Разрешения SDK](/docs/ru/agent-sdk/permissions) - Управление разрешениями инструментов
