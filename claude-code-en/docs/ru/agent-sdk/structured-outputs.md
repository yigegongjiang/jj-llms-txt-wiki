> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Получение структурированного вывода от агентов

> Возвращайте валидированный JSON из рабочих процессов агентов, используя JSON Schema, Zod или Pydantic. Получайте типобезопасные структурированные данные после многоходового использования инструментов.

Структурированные выводы позволяют вам определить точную форму данных, которые вы хотите получить от агента. Агент может использовать любые инструменты, необходимые для выполнения задачи, и вы все равно получите валидированный JSON, соответствующий вашей схеме в конце. Определите [JSON Schema](https://json-schema.org/understanding-json-schema/about) для нужной вам структуры, и SDK валидирует вывод в соответствии с ней, переспрашивая при несоответствии. Если валидация не будет успешной в пределах лимита повторных попыток, результатом будет ошибка вместо структурированных данных; см. [Обработка ошибок](#error-handling).

Для полной типобезопасности используйте [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) или [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python) для определения вашей схемы и получения строго типизированных объектов.

<h2 id="why-structured-outputs">
  Зачем нужны структурированные выводы?
</h2>

Агенты по умолчанию возвращают свободный текст, что подходит для чата, но не когда вам нужно использовать вывод программно. Структурированные выводы дают вам типизированные данные, которые вы можете передать непосредственно в логику приложения, базу данных или компоненты UI.

Рассмотрим приложение рецептов, где агент ищет в интернете и приносит рецепты. Без структурированных выводов вы получаете свободный текст, который вам нужно было бы разбирать самостоятельно. Со структурированными выводами вы определяете нужную вам форму и получаете типизированные данные, которые можно использовать непосредственно в приложении.

<AccordionGroup>
  <Accordion title="Без структурированных выводов">
    ```text theme={null}
    Вот классический рецепт печенья с шоколадной крошкой!

    **Печенье с шоколадной крошкой**
    Время подготовки: 15 минут | Время приготовления: 10 минут

    Ингредиенты:
    - 2 1/4 чашки универсальной муки
    - 1 чашка масла, размягченного
    ...
    ```

    Чтобы использовать это в приложении, вам нужно было бы разобрать название, преобразовать "15 минут" в число, отделить ингредиенты от инструкций и справиться с несогласованным форматированием в разных ответах.
  </Accordion>

  <Accordion title="Со структурированными выводами">
    ```json theme={null}
    {
      "name": "Печенье с шоколадной крошкой",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "универсальная мука", "amount": 2.25, "unit": "чашки" },
        { "item": "масло, размягченное", "amount": 1, "unit": "чашка" }
        // ...
      ],
      "steps": ["Разогрейте духовку до 375°F", "Взбейте масло и сахар" /* ... */]
    }
    ```

    Типизированные данные, которые вы можете использовать непосредственно в вашем UI.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Быстрый старт
</h2>

Чтобы использовать структурированные выводы, определите [JSON Schema](https://json-schema.org/understanding-json-schema/about), описывающую форму данных, которые вы хотите, затем передайте его в `query()` через опцию `outputFormat` (TypeScript) или `output_format` (Python). Когда агент завершит работу, сообщение результата включает поле `structured_output` с валидированными данными, соответствующими вашей схеме.

Пример ниже просит агента исследовать Anthropic и вернуть название компании, год основания и штаб-квартиру как структурированный вывод.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Определите форму данных, которые вы хотите получить
  const schema = {
    type: "object",
    properties: {
      company_name: { type: "string" },
      founded_year: { type: "number" },
      headquarters: { type: "string" }
    },
    required: ["company_name"]
  };

  for await (const message of query({
    prompt: "Research Anthropic and provide key company information",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // Сообщение результата содержит structured_output с валидированными данными
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  // Определите форму данных, которые вы хотите получить
  schema = {
      "type": "object",
      "properties": {
          "company_name": {"type": "string"},
          "founded_year": {"type": "number"},
          "headquarters": {"type": "string"},
      },
      "required": ["company_name"],
  }


  async def main():
      async for message in query(
          prompt="Research Anthropic and provide key company information",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          // Сообщение результата содержит structured_output с валидированными данными
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              // {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Типобезопасные схемы с Zod и Pydantic
</h2>

Вместо написания JSON Schema вручную, вы можете использовать [Zod](https://zod.dev/) (TypeScript) или [Pydantic](https://docs.pydantic.dev/latest/) (Python) для определения вашей схемы. Эти библиотеки генерируют JSON Schema для вас и позволяют разобрать ответ в полностью типизированный объект, который вы можете использовать во всей кодовой базе с автодополнением и проверкой типов.

Пример ниже определяет схему для плана реализации функции с резюме, списком шагов (каждый с уровнем сложности) и потенциальными рисками. Агент планирует функцию и возвращает типизированный объект `FeaturePlan`. Затем вы можете получить доступ к свойствам, таким как `plan.summary`, и перебирать `plan.steps` с полной типобезопасностью.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Определите схему с Zod
  const FeaturePlan = z.object({
    feature_name: z.string(),
    summary: z.string(),
    steps: z.array(
      z.object({
        step_number: z.number(),
        description: z.string(),
        estimated_complexity: z.enum(["low", "medium", "high"])
      })
    ),
    risks: z.array(z.string())
  });

  type FeaturePlan = z.infer<typeof FeaturePlan>;

  // Преобразуйте в JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Используйте в запросе
  for await (const message of query({
    prompt:
      "Plan how to add dark mode support to a React app. Break it into implementation steps.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // Валидируйте и получите полностью типизированный результат
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Feature: ${plan.feature_name}`);
        console.log(`Summary: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from pydantic import BaseModel
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  class Step(BaseModel):
      step_number: int
      description: str
      estimated_complexity: str  # 'low', 'medium', 'high'


  class FeaturePlan(BaseModel):
      feature_name: str
      summary: str
      steps: list[Step]
      risks: list[str]


  async def main():
      async for message in query(
          prompt="Plan how to add dark mode support to a React app. Break it into implementation steps.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # Валидируйте и получите полностью типизированный результат
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"Feature: {plan.feature_name}")
              print(f"Summary: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**Преимущества:**

* Полный вывод типов (TypeScript) и подсказки типов (Python)
* Валидация во время выполнения с `safeParse()` или `model_validate()`
* Лучшие сообщения об ошибках
* Компонуемые, переиспользуемые схемы

<h2 id="output-format-configuration">
  Конфигурация формата вывода
</h2>

Опция `outputFormat` (TypeScript) или `output_format` (Python) принимает объект с:

* `type`: Установите на `"json_schema"` для структурированных выводов
* `schema`: Объект [JSON Schema](https://json-schema.org/understanding-json-schema/about), определяющий структуру вашего вывода. Вы можете сгенерировать это из схемы Zod с `z.toJSONSchema()` или модели Pydantic с `.model_json_schema()`

SDK поддерживает стандартные функции JSON Schema, включая все базовые типы (object, array, string, number, boolean, null), `enum`, `const`, `required`, вложенные объекты и определения `$ref`. Для полного списка поддерживаемых функций и ограничений см. [Ограничения JSON Schema](https://platform.claude.com/docs/ru/build-with-claude/structured-outputs#json-schema-limitations).

Схема, которая не является действительной JSON Schema, приводит к сбою запуска при запуске с ошибкой, указывающей на проблему. До версии 2.1.205 недействительная схема молча игнорировалась, и агент возвращал неструктурированный текст.

Ключевое слово `format`, такое как `"format": "email"`, принимается как аннотация и не применяется валидатором SDK. До версии 2.1.205 любая схема, содержащая `format`, рассматривалась как недействительная.

<h2 id="example-todo-tracking-agent">
  Пример: агент отслеживания TODO
</h2>

Этот пример демонстрирует, как работают структурированные выводы с многошаговым использованием инструментов. Агент должен найти комментарии TODO в кодовой базе, а затем найти информацию git blame для каждого. Он автономно решает, какие инструменты использовать (Grep для поиска, Bash для запуска команд git) и объединяет результаты в один структурированный ответ.

Схема включает необязательные поля (`author` и `date`), так как информация git blame может быть недоступна для всех файлов. Агент заполняет то, что может найти, и опускает остальное.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Определите структуру для извлечения TODO
  const todoSchema = {
    type: "object",
    properties: {
      todos: {
        type: "array",
        items: {
          type: "object",
          properties: {
            text: { type: "string" },
            file: { type: "string" },
            line: { type: "number" },
            author: { type: "string" },
            date: { type: "string" }
          },
          required: ["text", "file", "line"]
        }
      },
      total_count: { type: "number" }
    },
    required: ["todos", "total_count"]
  };

  // Агент использует Grep для поиска TODO, Bash для получения информации git blame
  for await (const message of query({
    prompt: "Find all TODO comments in this codebase and identify who added them",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Found ${data.total_count} TODOs`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Added by ${todo.author} on ${todo.date}`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Определите структуру для извлечения TODO
  todo_schema = {
      "type": "object",
      "properties": {
          "todos": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "text": {"type": "string"},
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "author": {"type": "string"},
                      "date": {"type": "string"},
                  },
                  "required": ["text", "file", "line"],
              },
          },
          "total_count": {"type": "number"},
      },
      "required": ["todos", "total_count"],
  }


  async def main():
      # Агент использует Grep для поиска TODO, Bash для получения информации git blame
      async for message in query(
          prompt="Find all TODO comments in this codebase and identify who added them",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"Found {data['total_count']} TODOs")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  Added by {todo['author']} on {todo['date']}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Обработка ошибок
</h2>

Генерация структурированного вывода может не удаться, когда агент не может создать валидный JSON, соответствующий вашей схеме. Это обычно происходит, когда схема слишком сложна для задачи, сама задача неоднозначна или агент достигает лимита повторных попыток при попытке исправить ошибки валидации. Это также может произойти без каких-либо ошибок валидации: [откат модели](/docs/ru/model-config#automatic-model-fallback) может отменить уже завершённый вывод в середине потока, и если никакая повторная попытка его не заменит, выполнение завершится с той же ошибкой. Проверьте поле `errors` в сообщении результата, чтобы различить две причины перед отладкой вашей схемы.

Когда происходит ошибка, сообщение результата имеет `subtype`, указывающий, что пошло не так:

| Subtype                               | Значение                                                                                                               |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `success`                             | Вывод был успешно сгенерирован и валидирован                                                                           |
| `error_max_structured_output_retries` | Ни один валидный вывод не пережил несколько попыток (ошибки валидации или откат модели без успешной повторной попытки) |

Пример ниже проверяет поле `subtype`, чтобы определить, был ли вывод успешно сгенерирован или вам нужно справиться с ошибкой:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // Используйте валидированный вывод
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // Справьтесь с ошибкой - повторите с более простым приглашением, вернитесь к неструктурированному и т.д.
        console.error("Could not produce valid output");
      }
    }
  }
  ```

  ```python Python theme={null}
  async for message in query(
      prompt="Extract contact info from the document",
      options=ClaudeAgentOptions(
          output_format={"type": "json_schema", "schema": contact_schema}
      ),
  ):
      if isinstance(message, ResultMessage):
          if message.subtype == "success" and message.structured_output:
              # Используйте валидированный вывод
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # Справьтесь с ошибкой
              print("Could not produce valid output")
  ```
</CodeGroup>

**Советы по избежанию ошибок:**

* **Держите схемы сосредоточенными.** Глубоко вложенные схемы со многими обязательными полями сложнее удовлетворить. Начните с простого и добавляйте сложность по мере необходимости.
* **Соответствуйте схему задаче.** Если задача может не иметь всю информацию, которую требует ваша схема, сделайте эти поля необязательными.
* **Используйте четкие приглашения.** Неоднозначные приглашения затрудняют для агента понимание того, какой вывод создавать.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Документация JSON Schema](https://json-schema.org/): изучите синтаксис JSON Schema для определения сложных схем с вложенными объектами, массивами, перечислениями и ограничениями валидации
* [API Structured Outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs): используйте структурированные выводы с Claude API непосредственно для однооходовых запросов без использования инструментов
* [Custom tools](/docs/ru/agent-sdk/custom-tools): дайте вашему агенту пользовательские инструменты для вызова во время выполнения перед возвращением структурированного вывода
