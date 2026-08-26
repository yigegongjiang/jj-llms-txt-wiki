> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 從代理獲取結構化輸出

> 使用 JSON Schema、Zod 或 Pydantic 從代理工作流程返回驗證的 JSON。在多輪工具使用後獲得類型安全的結構化資料。

結構化輸出讓您定義想要從代理返回的資料的確切形狀。代理可以使用任何需要的工具來完成任務，而您仍然會在最後獲得與您的架構相匹配的驗證 JSON。定義一個 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 來描述您需要的結構，SDK 會根據它驗證輸出，在不匹配時重新提示。如果驗證在重試限制內未成功，結果將是錯誤而不是結構化資料；請參閱 [錯誤處理](#error-handling)。

為了獲得完整的類型安全，使用 [Zod](#type-safe-schemas-with-zod-and-pydantic)（TypeScript）或 [Pydantic](#type-safe-schemas-with-zod-and-pydantic)（Python）來定義您的架構並獲得強類型物件。

<h2 id="why-structured-outputs">
  為什麼要使用結構化輸出？
</h2>

代理預設返回自由格式的文本，這適用於聊天但不適用於您需要以程式方式使用輸出的情況。結構化輸出為您提供類型化資料，您可以直接傳遞給應用程式邏輯、資料庫或 UI 元件。

考慮一個食譜應用程式，其中代理搜尋網路並帶回食譜。沒有結構化輸出，您會獲得自由格式的文本，需要自己解析。使用結構化輸出，您定義想要的形狀並獲得可以直接在應用程式中使用的類型化資料。

<AccordionGroup>
  <Accordion title="沒有結構化輸出">
    ```text theme={null}
    這是一個經典的巧克力晶片餅乾食譜！

    **巧克力晶片餅乾**
    準備時間：15 分鐘 | 烹飪時間：10 分鐘

    材料：
    - 2 1/4 杯通用麵粉
    - 1 杯軟化黃油
    ...
    ```

    要在您的應用程式中使用這個，您需要解析出標題，將'15 分鐘'轉換為數字，將材料與說明分開，並處理跨回應的不一致格式。
  </Accordion>

  <Accordion title="使用結構化輸出">
    ```json theme={null}
    {
      "name": "Chocolate Chip Cookies",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "all-purpose flour", "amount": 2.25, "unit": "cups" },
        { "item": "butter, softened", "amount": 1, "unit": "cup" }
        // ...
      ],
      "steps": ["Preheat oven to 375°F", "Cream butter and sugar" /* ... */]
    }
    ```

    您可以直接在 UI 中使用的類型化資料。
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  快速開始
</h2>

要使用結構化輸出，定義一個 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 來描述您想要的資料形狀，然後通過 `outputFormat` 選項（TypeScript）或 `output_format` 選項（Python）將其傳遞給 `query()`。當代理完成時，結果訊息包含一個 `structured_output` 欄位，其中包含與您的架構相匹配的驗證資料。

下面的範例要求代理研究 Anthropic 並返回公司名稱、成立年份和總部作為結構化輸出。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 定義您想要返回的資料形狀
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
    // 結果訊息包含具有驗證資料的 structured_output
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # 定義您想要返回的資料形狀
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
          # 結果訊息包含具有驗證資料的 structured_output
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  使用 Zod 和 Pydantic 的類型安全架構
</h2>

與其手動編寫 JSON Schema，您可以使用 [Zod](https://zod.dev/)（TypeScript）或 [Pydantic](https://docs.pydantic.dev/latest/)（Python）來定義您的架構。這些庫為您生成 JSON Schema，並讓您將回應解析為完全類型化的物件，您可以在整個程式碼庫中使用，具有自動完成和類型檢查。

下面的範例定義了一個功能實現計劃的架構，包括摘要、步驟列表（每個步驟都有複雜性級別）和潛在風險。代理計劃該功能並返回一個類型化的 `FeaturePlan` 物件。然後您可以訪問 `plan.summary` 等屬性，並使用完整的類型安全遍歷 `plan.steps`。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 使用 Zod 定義架構
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

  // 轉換為 JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // 在查詢中使用
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
      // 驗證並獲得完全類型化的結果
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
              # 驗證並獲得完全類型化的結果
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

**優點：**

* 完整的類型推斷（TypeScript）和類型提示（Python）
* 使用 `safeParse()` 或 `model_validate()` 進行運行時驗證
* 更好的錯誤訊息
* 可組合、可重用的架構

<h2 id="output-format-configuration">
  輸出格式配置
</h2>

`outputFormat`（TypeScript）或 `output_format`（Python）選項接受一個物件，其中包含：

* `type`：設置為 `"json_schema"` 以進行結構化輸出
* `schema`：一個 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 物件，定義您的輸出結構。您可以使用 `z.toJSONSchema()` 從 Zod 架構或使用 `.model_json_schema()` 從 Pydantic 模型生成此項。

SDK 支援標準 JSON Schema 功能，包括所有基本類型（object、array、string、number、boolean、null）、`enum`、`const`、`required`、嵌套物件和 `$ref` 定義。有關支援的功能和限制的完整列表，請參閱 [JSON Schema 限制](https://platform.claude.com/docs/zh-TW/build-with-claude/structured-outputs#json-schema-limitations)。

不是有效 JSON Schema 的架構在啟動時會因錯誤而失敗執行，並命名該問題。在 v2.1.205 之前，無效的架構會被無聲地忽略，代理會返回非結構化文字。

`format` 關鍵字（例如 `"format": "email"`）被接受為註解，SDK 的驗證器不會強制執行。在 v2.1.205 之前，任何包含 `format` 的架構都被視為無效。

<h2 id="example-todo-tracking-agent">
  範例：TODO 追蹤代理
</h2>

此範例演示了結構化輸出如何與多步驟工具使用配合使用。代理需要在程式碼庫中查找 TODO 註釋，然後為每個註釋查找 git blame 資訊。它自主決定使用哪些工具（Grep 搜尋、Bash 執行 git 命令）並將結果組合成單個結構化回應。

架構包括可選欄位（`author` 和 `date`），因為 git blame 資訊可能不適用於所有檔案。代理填入它能找到的內容並省略其餘部分。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 定義 TODO 提取的結構
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

  // 代理使用 Grep 查找 TODO，使用 Bash 獲取 git blame 資訊
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

  # 定義 TODO 提取的結構
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
      # 代理使用 Grep 查找 TODO，使用 Bash 獲取 git blame 資訊
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
  錯誤處理
</h2>

結構化輸出生成可能會失敗，當代理無法生成與您的架構相匹配的有效 JSON 時。這通常發生在架構對於任務過於複雜、任務本身不明確或代理在嘗試修復驗證錯誤時達到重試限制時。它也可能在沒有任何驗證失敗的情況下發生：[模型回退](/docs/zh-TW/model-config#automatic-model-fallback)可以在中途收回已完成的輸出，如果沒有重試替換它，運行將以相同的錯誤結束。在調試您的架構之前，請檢查結果訊息上的 `errors` 欄位以區分這兩個原因。

發生錯誤時，結果訊息有一個 `subtype` 指示出了什麼問題：

| Subtype                               | 含義                                 |
| ------------------------------------- | ---------------------------------- |
| `success`                             | 輸出已成功生成並驗證                         |
| `error_max_structured_output_retries` | 多次嘗試後沒有有效輸出存活（驗證失敗，或模型回退收回且沒有成功重試） |

下面的範例檢查 `subtype` 欄位以確定輸出是否成功生成或您是否需要處理失敗：

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
        // 使用驗證的輸出
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // 處理失敗 - 使用更簡單的提示重試、回退到非結構化等
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
              # 使用驗證的輸出
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # 處理失敗
              print("Could not produce valid output")
  ```
</CodeGroup>

**避免錯誤的提示：**

* **保持架構專注。** 具有許多必需欄位的深層嵌套架構更難滿足。從簡單開始，根據需要添加複雜性。
* **匹配架構與任務。** 如果任務可能沒有您的架構所需的所有資訊，請將這些欄位設為可選。
* **使用清晰的提示。** 不明確的提示使代理更難知道要生成什麼輸出。

<h2 id="related-resources">
  相關資源
</h2>

* [JSON Schema 文件](https://json-schema.org/)：學習 JSON Schema 語法以定義具有嵌套物件、陣列、列舉和驗證約束的複雜架構
* [API 結構化輸出](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)：直接使用 Claude API 的結構化輸出進行單輪請求，無需工具使用
* [自訂工具](/docs/zh-TW/agent-sdk/custom-tools)：在返回結構化輸出之前，在執行期間為您的代理提供自訂工具以供調用
