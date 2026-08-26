> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 追蹤成本和使用情況

> 了解如何追蹤 token 使用情況、估計成本，以及使用 Claude Agent SDK 配置 prompt caching。

Claude Agent SDK 為每次與 Claude 的互動提供詳細的 token 使用資訊。本指南說明如何正確追蹤使用情況和理解成本報告，特別是在處理平行工具使用和多步驟對話時。

如需完整的 API 文件，請參閱 [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript) 和 [Python SDK 參考](/docs/zh-TW/agent-sdk/python)。

<Warning>
  `total_cost_usd` 和 `costUSD` 欄位是客戶端估計值，不是權威的計費資料。SDK 從在建置時捆綁的價格表本地計算它們，因此當以下情況發生時，它們可能與您實際被計費的金額不同：

  * 定價變更
  * 已安裝的 SDK 版本無法識別某個模型
  * 適用客戶端無法建模的計費規則

  使用這些欄位進行開發洞察和大約預算編制。如需權威計費，請使用 [Usage and Cost API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) 或 [Claude Console](https://platform.claude.com/usage) 中的 Usage 頁面。不要向終端使用者計費或根據這些欄位觸發財務決策。
</Warning>

<h2 id="understand-token-usage">
  了解 token 使用情況
</h2>

TypeScript 和 Python SDK 使用不同的欄位名稱公開相同的使用資料：

* **TypeScript** 在每個助手訊息上提供每步 token 細分（`message.message.id`、`message.message.usage`），通過結果訊息上的 `modelUsage` 提供每個模型的成本，以及結果訊息上的累積總計。
* **Python** 在每個助手訊息上提供每步 token 細分（`message.usage`、`message.message_id`），通過結果訊息上的 `model_usage` 提供每個模型的成本，以及結果訊息上的累積總計（`total_cost_usd` 和 `usage` 字典）。

兩個 SDK 使用相同的基礎成本模型並公開相同的粒度。差異在於欄位命名和每步使用情況的嵌套位置。

成本追蹤取決於理解 SDK 如何限定使用資料的範圍：

* **`query()` 呼叫：** SDK 的 `query()` 函數的一次調用。單次呼叫可能涉及多個步驟（Claude 回應、使用工具、獲取結果、再次回應）。每次呼叫在末尾產生一個 [`result`](/docs/zh-TW/agent-sdk/typescript#sdkresultmessage) 訊息。
* **步驟：** `query()` 呼叫中的單個請求/回應週期。每個步驟產生帶有 token 使用情況的助手訊息。
* **會話：** 由會話 ID 連結的一系列 `query()` 呼叫（使用 `resume` 選項）。會話中的每個 `query()` 呼叫獨立報告其自己的成本。

下圖顯示來自單個 `query()` 呼叫的訊息流，在每個步驟報告 token 使用情況，並在末尾顯示累積估計：

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="圖表顯示一個查詢產生兩個步驟的訊息。步驟 1 有四個共享相同 ID 和使用情況的助手訊息（計數一次），步驟 2 有一個具有新 ID 的助手訊息，最終結果訊息顯示估計的 total_cost_usd。" width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="每個步驟產生助手訊息">
    當 Claude 回應時，它發送一個或多個助手訊息。在 TypeScript 中，每個助手訊息包含一個嵌套的 `BetaMessage`（通過 `message.message` 存取），具有 `id` 和一個 [`usage`](https://platform.claude.com/docs/en/api/messages) 物件，其中包含 token 計數（`input_tokens`、`output_tokens`）。在 Python 中，`AssistantMessage` 資料類別通過 `message.usage` 和 `message.message_id` 直接公開相同的資料。當 Claude 在一個回合中使用多個工具時，該回合中的所有訊息共享相同的 ID，因此按 ID 去重以避免重複計數。
  </Step>

  <Step title="結果訊息提供累積估計">
    當 `query()` 呼叫完成時，SDK 發出一個結果訊息，其中包含 `total_cost_usd` 和累積 `usage`。這在 TypeScript（[`SDKResultMessage`](/docs/zh-TW/agent-sdk/typescript#sdkresultmessage)）和 Python（[`ResultMessage`](/docs/zh-TW/agent-sdk/python#resultmessage)）中都可用。如果您進行多個 `query()` 呼叫（例如，在多回合會話中），每個結果只反映該個別呼叫的成本。如果您只需要估計的總計，您可以忽略每步使用情況並讀取此單一值。
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  取得查詢的總成本
</h2>

結果訊息（[TypeScript](/docs/zh-TW/agent-sdk/typescript#sdkresultmessage)、[Python](/docs/zh-TW/agent-sdk/python#resultmessage)）標記 `query()` 呼叫的代理迴圈的結束。它包含 `total_cost_usd`，即該呼叫中所有步驟的累積估計成本。這適用於成功和錯誤結果。如果您使用會話進行多個 `query()` 呼叫，每個結果只反映該個別呼叫的成本。

當代理產生[子代理](/docs/zh-TW/agent-sdk/subagents)時，三個結果層級的欄位在計算內容上有所不同。使用 `modelUsage` 或 Python 中的 `model_usage` 進行整個樹狀結構的權杖計算；`usage` 欄位一旦發生巢狀就會低估。

| 欄位                           | 子代理活動                          |
| ---------------------------- | ------------------------------ |
| `usage`                      | 已排除。僅計算頂層代理迴圈，因此子代理內消耗的權杖不會被加入 |
| `total_cost_usd`             | 已包含。計算子代理請求以及頂層迴圈              |
| `modelUsage` / `model_usage` | 已包含。計算子代理請求以及頂層迴圈，按模型細分        |

以下範例遍歷來自 `query()` 呼叫的訊息流，並在 `result` 訊息到達時列印總成本：

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
  追蹤每步和每個模型的使用情況
</h2>

本節中的範例使用 TypeScript 欄位名稱。在 Python 中，等效欄位是 [`AssistantMessage.usage`](/docs/zh-TW/agent-sdk/python#assistantmessage) 和 `AssistantMessage.message_id` 用於每步使用情況，以及 [`ResultMessage.model_usage`](/docs/zh-TW/agent-sdk/python#resultmessage) 用於每個模型的細分。

<h3 id="track-per-step-usage">
  追蹤每步使用情況
</h3>

每個助手訊息包含一個嵌套的 `BetaMessage`（通過 `message.message` 存取），具有 `id` 和 `usage` 物件，其中包含 token 計數。當 Claude 並行使用工具時，多個訊息共享相同的 `id` 和相同的使用資料。追蹤您已經計數的 ID，並跳過重複項以避免膨脹的總計。

<Warning>
  平行工具呼叫產生多個助手訊息，其嵌套的 `BetaMessage` 共享相同的 `id` 和相同的使用情況。始終按 ID 去重以獲得準確的每步 token 計數。
</Warning>

以下範例累積所有步驟中的輸入和輸出 token，每個唯一訊息 ID 只計數一次：

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
  按模型細分使用情況
</h3>

結果訊息包含 [`modelUsage`](/docs/zh-TW/agent-sdk/typescript#modelusage)，一個模型名稱到每個模型 token 計數和成本的映射。當您執行多個模型（例如，子代理使用 Haiku，主代理使用 Opus）並想查看 token 的去向時，這很有用。

以下範例執行查詢並列印每個使用的模型的成本和 token 細分：

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
  累積多個呼叫的成本
</h2>

每個 `query()` 呼叫返回其自己的 `total_cost_usd`。SDK 不提供會話級別的總計，因此如果您的應用程式進行多個 `query()` 呼叫（例如，在多回合會話中或跨不同使用者），請自行累積總計。

以下範例順序執行兩個 `query()` 呼叫，將每個呼叫的 `total_cost_usd` 加到運行總計中，並列印每個呼叫和合併的成本：

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
  處理錯誤、快取和 token 差異
</h2>

為了準確的成本追蹤，請考慮失敗的對話、快取 token 定價和偶爾的報告不一致。

<h3 id="resolve-output-token-discrepancies">
  解決輸出 token 差異
</h3>

在罕見情況下，您可能會觀察到具有相同 ID 的訊息的不同 `output_tokens` 值。當發生這種情況時：

1. **使用最高值：** 一組中的最終訊息通常包含準確的總計。
2. **優先使用結果訊息：** 結果訊息中的 `total_cost_usd` 反映 SDK 在所有步驟中的累積估計，因此比自己求和每步值更可靠。它仍然是一個估計值，可能與您的實際帳單不同。
3. **報告不一致：** 在 [Claude Code GitHub 儲存庫](https://github.com/anthropics/claude-code/issues) 提交問題。

<h3 id="track-costs-on-failed-conversations">
  追蹤失敗對話的成本
</h3>

成功和錯誤結果訊息都包含 `usage` 和 `total_cost_usd`。如果對話在中途失敗，您仍然消耗了到失敗點為止的 token。無論其 `subtype` 如何，始終從結果訊息讀取成本資料。

<h3 id="track-cache-tokens">
  追蹤快取 token
</h3>

Agent SDK 自動使用 [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) 來減少重複內容的成本。您不需要自己配置快取。使用物件包含兩個額外的欄位用於快取追蹤：

* `cache_creation_input_tokens`：用於建立新快取項目的 token（按比標準輸入 token 更高的費率計費）。
* `cache_read_input_tokens`：從現有快取項目讀取的 token（按降低的費率計費）。

將這些與 `input_tokens` 分開追蹤以了解快取節省。在 TypeScript 中，這些欄位在 [`Usage`](/docs/zh-TW/agent-sdk/typescript#usage) 物件上輸入。在 Python 中，它們作為 [`ResultMessage.usage`](/docs/zh-TW/agent-sdk/python#resultmessage) 字典中的鍵出現（例如，`message.usage.get("cache_read_input_tokens", 0)`）。

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  將 prompt 快取 TTL 延長至一小時
</h3>

當您使用 API 金鑰進行身份驗證或在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上執行時，SDK 寫入的快取項目預設使用 5 分鐘的 TTL。如果您的工作負載針對相同的系統提示和上下文執行許多短會話，且它們之間的間隔超過 5 分鐘，快取會在會話之間過期，每個新會話都會支付完整的輸入價格。

要請求快取寫入的 1 小時 TTL，請設定 [`ENABLE_PROMPT_CACHING_1H`](/docs/zh-TW/env-vars) 環境變數。您可以在 shell 或容器環境中匯出它，或通過 `options.env` 傳遞它。

以下範例為在 Amazon Bedrock 上執行的代理啟用 1 小時 TTL：

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

具有 1 小時 TTL 的快取寫入按比 5 分鐘寫入更高的費率計費，因此啟用此功能會用更高的寫入成本換取更多的快取讀取。有關詳細資訊，請參閱 [prompt caching 定價](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)。Claude 訂閱使用者已自動獲得 1 小時 TTL，不需要設定此變數。

<h2 id="related-documentation">
  相關文件
</h2>

* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript) - 完整的 API 文件
* [SDK 概述](/docs/zh-TW/agent-sdk/overview) - SDK 入門
* [SDK 權限](/docs/zh-TW/agent-sdk/permissions) - 管理工具權限
