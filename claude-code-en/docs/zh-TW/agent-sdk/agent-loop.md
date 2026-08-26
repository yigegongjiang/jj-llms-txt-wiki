> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 代理程式迴圈如何運作

> 了解訊息生命週期、工具執行、上下文視窗和支援 SDK 代理程式的架構。

Agent SDK 讓您可以在自己的應用程式中嵌入 Claude Code 的自主代理程式迴圈。SDK 是一個獨立套件，可讓您以程式設計方式控制工具、權限、成本限制和輸出。您不需要安裝 Claude Code CLI 即可使用它。

當您啟動代理程式時，SDK 會執行與 [Claude Code 相同的執行迴圈](/docs/zh-TW/how-claude-code-works#the-agentic-loop)：Claude 評估您的提示、呼叫工具採取行動、接收結果，並重複直到任務完成。本頁說明該迴圈內發生的情況，以便您可以有效地建立、除錯和最佳化代理程式。

<h2 id="the-loop-at-a-glance">
  迴圈概覽
</h2>

每個代理程式工作階段都遵循相同的週期：

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" alt="代理程式迴圈的圖表：您的提示進入代理程式迴圈，Claude 評估並要求工具呼叫（其結果回饋到另一個評估中），或返回最終答案" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

1. **接收提示。** Claude 接收您的提示，以及系統提示、工具定義和對話歷史記錄。SDK 會產生一個 [`SystemMessage`](#message-types)，其子類型為 `"init"`，包含工作階段中繼資料。
2. **評估並回應。** Claude 評估目前狀態並決定如何進行。它可能會以文字回應、要求一個或多個工具呼叫，或兩者都有。SDK 會產生一個 [`AssistantMessage`](#message-types)，包含文字和任何工具呼叫要求。
3. **執行工具。** SDK 執行每個要求的工具並收集結果。每組工具結果都會回饋給 Claude 以進行下一個決定。您可以使用 [hooks](/docs/zh-TW/agent-sdk/hooks) 在工具執行前攔截、修改或阻止工具呼叫。
4. **重複。** 步驟 2 和 3 重複為一個週期。每個完整週期是一個回合。Claude 繼續呼叫工具並處理結果，直到它產生沒有工具呼叫的回應。
5. **返回結果。** SDK 會產生最終的 [`AssistantMessage`](#message-types)，包含文字回應（無工具呼叫），然後是 [`ResultMessage`](#message-types)，包含最終文字、代幣使用量、成本和工作階段 ID。

一個快速問題（「這裡有什麼檔案？」）可能需要一到兩個回合的 `Glob` 呼叫和結果回應。一個複雜的任務（「重構驗證模組並更新測試」）可以在許多回合中鏈接數十個工具呼叫，讀取檔案、編輯程式碼和執行測試，Claude 根據每個結果調整其方法。

<h2 id="turns-and-messages">
  回合和訊息
</h2>

回合是迴圈內的一個往返：Claude 產生包含工具呼叫的輸出，SDK 執行這些工具，結果自動回饋給 Claude。這發生在不將控制權交回給您的程式碼的情況下。回合繼續進行，直到 Claude 產生沒有工具呼叫的輸出，此時迴圈結束並傳遞最終結果。

考慮提示「修復 auth.ts 中失敗的測試」的完整工作階段可能是什麼樣子。

首先，SDK 將您的提示發送給 Claude 並產生一個 [`SystemMessage`](#message-types)，包含工作階段中繼資料。然後迴圈開始：

1. **回合 1：** Claude 呼叫 `Bash` 執行 `npm test`。SDK 產生一個 [`AssistantMessage`](#message-types)，包含工具呼叫，執行命令，然後產生一個 [`UserMessage`](#message-types)，包含輸出（三個失敗）。
2. **回合 2：** Claude 在 `auth.ts` 和 `auth.test.ts` 上呼叫 `Read`。SDK 返回檔案內容並產生一個 `AssistantMessage`。
3. **回合 3：** Claude 呼叫 `Edit` 修復 `auth.ts`，然後呼叫 `Bash` 重新執行 `npm test`。所有三個測試都通過。SDK 產生一個 `AssistantMessage`。
4. **最終回合：** Claude 產生一個純文字回應，沒有工具呼叫：「修復了驗證錯誤，所有三個測試現在都通過了。」SDK 產生最終的 `AssistantMessage`，包含此文字，然後是 [`ResultMessage`](#message-types)，包含相同的文字加上成本和使用量。

那是四個回合：三個有工具呼叫，一個最終純文字回應。

您可以使用 `max_turns` / `maxTurns` 限制迴圈，它只計算工具使用回合。例如，上面迴圈中的 `max_turns=2` 會在編輯步驟之前停止。您也可以使用 `max_budget_usd` / `maxBudgetUsd` 根據支出閾值限制回合。

沒有限制，迴圈會執行到 Claude 自己完成為止，這對於範圍明確的任務很好，但對於開放式提示（「改進此程式碼庫」）可能會執行很長時間。設定預算是生產代理程式的好預設值。請參閱下面的 [回合和預算](#turns-and-budget) 以了解選項參考。

<h2 id="message-types">
  訊息類型
</h2>

當迴圈執行時，SDK 會產生一串訊息。每個訊息都帶有一個類型，告訴您它來自迴圈的哪個階段。五個核心類型是：

* **`SystemMessage`：** 工作階段生命週期事件。`subtype` 欄位區分它們：

  * `"init"`：執行的工作階段中繼資料。當 `SessionStart` 或 `Setup` hook 在工作階段啟動期間執行時，其 [hook 生命週期訊息](/docs/zh-TW/agent-sdk/typescript#sdkhookstartedmessage) 會在 `init` 訊息之前到達
  * `"compact_boundary"`：在 [壓縮](#automatic-compaction) 後觸發
  * `"informational"`：來自迴圈的純文字狀態橫幅
  * `"worker_shutting_down"`：迴圈將在目前回合後結束，因為主機正在退出或遠端控制已斷開連線

  在 TypeScript 中，除了 `"init"` 之外的每個子類型都是 [`SDKMessage` 聯合](/docs/zh-TW/agent-sdk/typescript#sdkmessage) 中的自己的類型，而不是 `SDKSystemMessage` 的子類型。
* **`AssistantMessage`：** 在每個 Claude 回應後發出，包括最終純文字回應。包含該回合的文字內容區塊和工具呼叫區塊。
* **`UserMessage`：** 在每個工具執行後發出，包含發送回 Claude 的工具結果內容。也針對您在迴圈中期串流的任何使用者輸入發出。
* **`StreamEvent`：** 僅在啟用部分訊息時發出。包含原始 API 串流事件（文字增量、工具輸入區塊）。請參閱 [串流回應](/docs/zh-TW/agent-sdk/streaming-output)。
* **`ResultMessage`：** 標記代理程式迴圈的結束。包含最終文字結果、代幣使用量、成本和工作階段 ID。檢查 `subtype` 欄位以確定任務是否成功或達到限制。少數尾隨系統事件（例如 `prompt_suggestion`）可能在其後到達，因此請迭代串流至完成，而不是在結果時中斷。請參閱 [處理結果](#handle-the-result)。

這五種類型涵蓋了兩個 SDK 中完整的代理程式迴圈生命週期。TypeScript SDK 還會產生額外的可觀測性事件（hook 事件、工具進度、速率限制、任務通知），提供額外詳細資訊，但不需要驅動迴圈。請參閱 [Python 訊息類型參考](/docs/zh-TW/agent-sdk/python#message-types) 和 [TypeScript 訊息類型參考](/docs/zh-TW/agent-sdk/typescript#message-types) 以了解完整清單。

<h3 id="handle-messages">
  處理訊息
</h3>

您處理哪些訊息取決於您正在建立的內容：

* **僅最終結果：** 處理 `ResultMessage` 以取得輸出、成本以及任務是否成功或達到限制。
* **進度更新：** 處理 `AssistantMessage` 以查看 Claude 在每個回合中做什麼，包括它呼叫了哪些工具。
* **即時串流：** 啟用部分訊息（Python 中的 `include_partial_messages`、TypeScript 中的 `includePartialMessages`）以實時取得 `StreamEvent` 訊息。請參閱 [即時串流回應](/docs/zh-TW/agent-sdk/streaming-output)。

您檢查訊息類型的方式取決於 SDK：

* **Python：** 使用從 `claude_agent_sdk` 匯入的類別檢查訊息類型，使用 `isinstance()`（例如，`isinstance(message, ResultMessage)`）。
* **TypeScript：** 檢查 `type` 字串欄位（例如，`message.type === "result"`）。`AssistantMessage` 和 `UserMessage` 將原始 API 訊息包裝在 `.message` 欄位中，因此內容區塊位於 `message.message.content`，而不是 `message.content`。

<Accordion title="範例：檢查訊息類型並處理結果">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    print(f"Turn completed: {len(message.content)} content blocks")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. If the
            # failure was an error result, the error subtype branches above have
            # already run; connection or process failures yield no result message.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          console.log(`Turn completed: ${message.message.content.length} content blocks`);
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, the error subtype branches above have
      // already run; connection or process failures yield no result message.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  工具執行
</h2>

工具讓您的代理程式能夠採取行動。沒有工具，Claude 只能以文字回應。有了工具，Claude 可以讀取檔案、執行命令、搜尋程式碼並與外部服務互動。

<h3 id="built-in-tools">
  內建工具
</h3>

SDK 包含與 Claude Code 相同的工具：

| 類別       | 工具                                                          | 它們的作用                   |
| :------- | :---------------------------------------------------------- | :---------------------- |
| **檔案操作** | `Read`、`Edit`、`Write`                                       | 讀取、修改和建立檔案              |
| **搜尋**   | `Glob`、`Grep`                                               | 按模式查找檔案、使用正規表達式搜尋內容     |
| **執行**   | `Bash`                                                      | 執行 shell 命令、指令碼、git 操作  |
| **Web**  | `WebSearch`、`WebFetch`                                      | 搜尋網路、擷取和解析頁面            |
| **探索**   | `ToolSearch`                                                | 動態查找和按需加載工具，而不是預先加載所有工具 |
| **協調**   | `Agent`、`Skill`、`AskUserQuestion`、`TaskCreate`、`TaskUpdate` | 生成子代理程式、呼叫技能、詢問使用者、追蹤任務 |

除了內建工具，您還可以：

* **使用 [MCP 伺服器](/docs/zh-TW/agent-sdk/mcp) 連接外部服務**（資料庫、瀏覽器、API）
* **使用 [自訂工具處理程式](/docs/zh-TW/agent-sdk/custom-tools) 定義自訂工具**
* **透過 [設定來源](/docs/zh-TW/agent-sdk/claude-code-features) 加載專案技能**以實現可重複使用的工作流程

<h3 id="tool-permissions">
  工具權限
</h3>

Claude 根據任務決定呼叫哪些工具，但您控制這些呼叫是否允許執行。您可以自動批准特定工具、完全阻止其他工具，或要求對所有工具進行批准。三個選項一起工作以確定運行的內容：

* **`allowed_tools` / `allowedTools`** 自動批准列出的工具。具有 `["Read", "Glob", "Grep"]` 在其允許工具清單中的唯讀代理程式會執行這些工具而不提示。未列出的工具仍然可用，但需要權限。
* **`disallowed_tools` / `disallowedTools`** 阻止列出的工具，無論其他設定如何。請參閱 [權限](/docs/zh-TW/agent-sdk/permissions) 以了解在工具執行前檢查規則的順序。
* **`permission_mode` / `permissionMode`** 控制對不受允許或拒絕規則涵蓋的工具會發生什麼。請參閱 [權限模式](#permission-mode) 以了解可用的模式。

您也可以使用 `"Bash(npm *)"` 之類的規則來限定個別工具，以僅允許特定命令。請參閱 [權限](/docs/zh-TW/agent-sdk/permissions) 以了解完整的規則語法。

當工具被拒絕時，Claude 會收到一條拒絕訊息作為工具結果，通常會嘗試不同的方法或報告它無法繼續。

<h3 id="parallel-tool-execution">
  平行工具執行
</h3>

當 Claude 在單個回合中要求多個工具呼叫時，兩個 SDK 可以根據工具同時或順序執行它們。唯讀工具（如 `Read`、`Glob`、`Grep` 和標記為唯讀的 MCP 工具）可以同時執行。修改狀態的工具（如 `Edit`、`Write` 和 `Bash`）順序執行以避免衝突。

自訂工具預設為順序執行。要為自訂工具啟用平行執行，請在其註釋中設定 `readOnlyHint`。[TypeScript](/docs/zh-TW/agent-sdk/typescript#tool) 和 [Python](/docs/zh-TW/agent-sdk/python#tool) SDK 都使用 MCP SDK 中的此欄位名稱。

<h2 id="control-how-the-loop-runs">
  控制迴圈如何執行
</h2>

您可以限制迴圈執行的回合數、成本、Claude 推理的深度，以及工具是否需要在執行前獲得批准。所有這些都是 [`ClaudeAgentOptions`](/docs/zh-TW/agent-sdk/python#claudeagentoptions)（Python）/ [`Options`](/docs/zh-TW/agent-sdk/typescript#options)（TypeScript）上的欄位。

<h3 id="turns-and-budget">
  回合和預算
</h3>

| 選項                                      | 它控制什麼      | 預設值 |
| :-------------------------------------- | :--------- | :-- |
| 最大回合（`max_turns` / `maxTurns`）          | 最大工具使用往返次數 | 無限制 |
| 最大預算（`max_budget_usd` / `maxBudgetUsd`） | 停止前的最大成本   | 無限制 |

當達到任一限制時，SDK 會返回一個 `ResultMessage`，其中包含相應的錯誤子類型（`error_max_turns` 或 `error_max_budget_usd`）。請參閱 [處理結果](#handle-the-result) 以了解如何檢查這些子類型，以及 [`ClaudeAgentOptions`](/docs/zh-TW/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/zh-TW/agent-sdk/typescript#options) 以了解語法。

使用 [串流輸入](/docs/zh-TW/agent-sdk/streaming-vs-single-mode)，當回合在最大回合限制時結束時，您在回合仍在執行時發送的訊息會保持佇列狀態，並且它會以自己的最大回合限制開始自己的回合。在 v2.1.205 之前，在回合最後一次迭代時到達的訊息可能會被消耗到結束回合中並遺失，而不會到達模型。

<h3 id="effort-level">
  努力等級
</h3>

`effort` 選項控制 Claude 應用多少推理。較低的努力等級每個回合使用更少的代幣並降低成本。並非所有模型都支援努力參數。請參閱 [努力](https://platform.claude.com/docs/en/build-with-claude/effort) 以了解哪些模型支援它。

| 等級         | 行為        | 適合                                           |
| :--------- | :-------- | :------------------------------------------- |
| `"low"`    | 最少推理、快速回應 | 檔案查找、列出目錄                                    |
| `"medium"` | 平衡推理      | 常規編輯、標準任務                                    |
| `"high"`   | 徹底分析      | 重構、除錯                                        |
| `"xhigh"`  | 擴展推理深度    | 編碼和代理任務；建議在 Fable 5、Opus 4.7+ 和 Sonnet 5 上使用 |
| `"max"`    | 最大推理深度    | 需要深入分析的多步驟問題                                 |

如果您不設定 `effort`，兩個 SDK 都會保留參數未設定，並遵循模型的預設行為。

<Note>
  `effort` 在每個回應內交換延遲和代幣成本以獲得推理深度。[擴展思考](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) 是一個單獨的功能，在輸出中產生可見的思考鏈區塊。它們是獨立的：您可以設定 `effort: "low"` 並啟用擴展思考，或 `effort: "max"` 而不啟用它。
</Note>

對於執行簡單、範圍明確的任務（如列出檔案或執行單個 grep）的代理程式，使用較低的努力來降低成本和延遲。在頂級 `query()` 選項中設定 `effort` 以用於整個工作階段，或在 [`AgentDefinition`](/docs/zh-TW/agent-sdk/subagents#agentdefinition-configuration) 上使用 `effort` 欄位以每個子代理程式為基礎覆蓋工作階段等級。

<h3 id="permission-mode">
  權限模式
</h3>

權限模式選項（Python 中的 `permission_mode`、TypeScript 中的 `permissionMode`）控制代理程式是否在使用工具前要求批准：

| 模式                    | 行為                                                                                                                                                                                                                                                                                                           |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"default"`           | 不受允許規則涵蓋的工具會觸發您的批准回呼；沒有回呼意味著拒絕                                                                                                                                                                                                                                                                               |
| `"acceptEdits"`       | 自動批准檔案編輯和常見的檔案系統命令（`mkdir`、`touch`、`mv`、`cp` 等）；其他 Bash 命令遵循預設規則                                                                                                                                                                                                                                             |
| `"plan"`              | Claude 探索並規劃而不編輯您的原始檔案；檔案編輯永遠不會自動批准，並透過您的 `canUseTool` 回呼提示                                                                                                                                                                                                                                                  |
| `"dontAsk"`           | 永不提示。由 [權限規則](/docs/zh-TW/settings#permission-settings) 預先批准的工具執行；其他所有工具都被拒絕。`AskUserQuestion`、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具即使您已允許它們也會被拒絕                                 |
| `"auto"`              | 使用模型分類器批准或拒絕每個工具呼叫。請參閱 [自動模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 以了解可用性和行為                                                                                                                                                                                                            |
| `"bypassPermissions"` | 執行所有允許的工具而不詢問，除了由明確的 [`ask` 規則](/docs/zh-TW/settings#permission-settings) 符合的工具、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和需要使用者互動的工具；請參閱 [權限如何被評估](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated) 以了解優先順序順序。在 Unix 上以 root 身份執行時無法使用。僅在隔離環境中使用，其中代理程式的操作無法影響您關心的系統 |

對於互動式應用程式，使用 `"default"` 和工具批准回呼來顯示批准提示。對於開發機器上的自主代理程式，`"acceptEdits"` 自動批准檔案編輯和常見的檔案系統命令（`mkdir`、`touch`、`mv`、`cp` 等），同時仍然在允許規則後面限制其他 `Bash` 命令。為 CI、容器或其他隔離環境保留 `"bypassPermissions"`。請參閱 [權限](/docs/zh-TW/agent-sdk/permissions) 以了解完整詳細資訊。

<h3 id="model">
  模型
</h3>

如果您不設定 `model`，SDK 會使用 Claude Code 的預設值，這取決於您的驗證方法和訂閱。明確設定它（例如，`model="claude-sonnet-5"`）以固定特定模型或使用較小的模型以獲得更快、更便宜的代理程式。請參閱 [模型](https://platform.claude.com/docs/en/about-claude/models) 以了解可用的 ID。

<h2 id="the-context-window">
  上下文視窗
</h2>

上下文視窗是工作階段期間可用於 Claude 的資訊總量。它不會在工作階段內的回合之間重置。所有內容都會累積：系統提示、工具定義、對話歷史記錄、工具輸入和工具輸出。在回合之間保持相同的內容（系統提示、工具定義、CLAUDE.md）會自動進行 [提示快取](https://platform.claude.com/docs/zh-TW/build-with-claude/prompt-caching)，這會減少重複前綴的成本和延遲。

<h3 id="what-consumes-context">
  什麼消耗上下文
</h3>

以下是每個元件如何影響 SDK 中上下文的方式：

| 來源               | 何時加載                                                                | 影響                                                                                                                                                                                                                     |
| :--------------- | :------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **系統提示**         | 每個請求                                                                | 小的固定成本，始終存在                                                                                                                                                                                                            |
| **CLAUDE.md 檔案** | 工作階段開始，透過 [`settingSources`](/docs/zh-TW/agent-sdk/claude-code-features) | 每個請求中的完整內容（但提示快取，因此只有第一個請求支付完整成本）                                                                                                                                                                                      |
| **工具定義**         | 每個請求；MCP 架構預設延遲                                                     | 內建工具架構在每個請求時加載。[工具搜尋](/docs/zh-TW/agent-sdk/mcp#mcp-tool-search) 預設延遲 MCP 工具架構，在 Google Cloud 的 Agent Platform 或非第一方 `ANTHROPIC_BASE_URL` 上回退到預先加載。請參閱 [配置工具搜尋](/docs/zh-TW/agent-sdk/tool-search#configure-tool-search) 以了解完整矩陣 |
| **對話歷史記錄**       | 在回合中累積                                                              | 隨著每個回合增長：提示、回應、工具輸入、工具輸出                                                                                                                                                                                               |
| **技能描述**         | 工作階段開始，透過設定來源                                                       | 簡短摘要；完整內容僅在呼叫時加載                                                                                                                                                                                                       |

大型工具輸出消耗大量上下文。讀取大檔案或執行具有詳細輸出的命令可以在單個回合中使用數千個代幣。上下文在回合中累積，因此具有許多工具呼叫的較長工作階段比短工作階段建立更多上下文。

<h3 id="automatic-compaction">
  自動壓縮
</h3>

當上下文視窗接近其限制時，SDK 會自動壓縮對話：它總結較舊的歷史記錄以釋放空間，保持您最近的交換和關鍵決定完整。SDK 在串流中發出一個 `type: "system"` 和 `subtype: "compact_boundary"` 的訊息（在 Python 中這是一個 `SystemMessage`；在 TypeScript 中它是一個單獨的 `SDKCompactBoundaryMessage` 類型）。

壓縮用摘要替換較舊的訊息，因此對話早期的特定指示可能不會被保留。持久規則應該在 CLAUDE.md 中（透過 [`settingSources`](/docs/zh-TW/agent-sdk/claude-code-features) 加載），而不是在初始提示中，因為 CLAUDE.md 內容在每個請求時重新注入。

您可以透過多種方式自訂壓縮行為：

* **CLAUDE.md 中的摘要指示：** 壓縮器像任何其他上下文一樣讀取您的 CLAUDE.md，因此您可以包含一個部分，告訴它在摘要時要保留什麼。部分標題是自由格式的（不是魔法字串）；壓縮器根據意圖匹配。
* **`PreCompact` hook：** 在壓縮發生前執行自訂邏輯，例如存檔完整成績單。hook 接收一個 `trigger` 欄位（`manual` 或 `auto`）。請參閱 [hooks](/docs/zh-TW/agent-sdk/hooks)。
* **手動壓縮：** 將 `/compact` 作為提示字串發送以按需觸發壓縮。以這種方式發送的命令是 SDK 輸入，而不是僅限 CLI 的快捷方式。請參閱 [SDK 中的命令](/docs/zh-TW/agent-sdk/slash-commands)。

<Accordion title="範例：CLAUDE.md 中的摘要指示">
  將一個部分添加到您的專案的 CLAUDE.md，告訴壓縮器要保留什麼。標題名稱不是特殊的；使用任何清晰的標籤。

  ```markdown CLAUDE.md theme={null}
  # Summary instructions

  When summarizing this conversation, always preserve:
  - The current task objective and acceptance criteria
  - File paths that have been read or modified
  - Test results and error messages
  - Decisions made and the reasoning behind them
  ```
</Accordion>

<h3 id="keep-context-efficient">
  保持上下文高效
</h3>

長時間執行代理程式的幾個策略：

* **為子任務使用子代理程式。** 每個子代理程式以新鮮的對話開始（沒有先前的訊息歷史記錄，儘管它確實加載自己的系統提示和專案級上下文，如 CLAUDE.md）。它看不到父級的回合，只有其最終回應作為工具結果返回給父級。主代理程式的上下文增長該摘要，而不是完整的子任務成績單。請參閱 [子代理程式繼承什麼](/docs/zh-TW/agent-sdk/subagents#what-subagents-inherit) 以了解詳細資訊。
* **選擇性地使用工具。** 每個工具定義都佔用上下文空間。在 [`AgentDefinition`](/docs/zh-TW/agent-sdk/subagents#agentdefinition-configuration) 上使用 `tools` 欄位將子代理程式限制在它們需要的最小集合。
* **監視 MCP 伺服器成本。** [MCP 工具搜尋](/docs/zh-TW/agent-sdk/mcp#mcp-tool-search) 預設延遲 MCP 工具架構，並按需加載它們。當工具搜尋關閉、在 Google Cloud 的 Agent Platform 上或在非第一方 `ANTHROPIC_BASE_URL` 後面時，每個 MCP 伺服器將其所有工具架構添加到每個請求，因此具有許多工具的幾個伺服器可以在代理程式執行任何工作之前消耗大量上下文。
* **為常規任務使用較低的努力。** 為僅需要讀取檔案或列出目錄的代理程式設定 [努力](#effort-level) 為 `"low"`。這會減少代幣使用量和成本。

有關每個功能上下文成本的詳細分解，請參閱 [了解上下文成本](/docs/zh-TW/features-overview#understand-context-costs)。

<h2 id="sessions-and-continuity">
  工作階段和連續性
</h2>

與 SDK 的每次互動都會建立或繼續一個工作階段。從 `ResultMessage.session_id`（在兩個 SDK 中都可用）捕獲工作階段 ID 以稍後恢復。TypeScript SDK 也將其公開為初始化 `SystemMessage` 上的直接欄位；在 Python 中，它嵌套在 `SystemMessage.data` 中。

當您恢復時，先前回合的完整上下文會被恢復：讀取的檔案、執行的分析和採取的操作。您也可以分叉一個工作階段以分支到不同的方法，而不修改原始工作階段。

請參閱 [工作階段管理](/docs/zh-TW/agent-sdk/sessions) 以了解恢復、繼續和分叉模式的完整指南。

<Note>
  在 Python 中，`ClaudeSDKClient` 在多個呼叫中自動處理工作階段 ID。請參閱 [Python SDK 參考](/docs/zh-TW/agent-sdk/python#choosing-between-query-and-claudesdkclient) 以了解詳細資訊。
</Note>

<h2 id="handle-the-result">
  處理結果
</h2>

當迴圈結束時，`ResultMessage` 告訴您發生了什麼並給您輸出。`subtype` 欄位（在兩個 SDK 中都可用）是檢查終止狀態的主要方式。

| 結果子類型                                 | 發生了什麼                                                 | `result` 欄位可用？ |
| :------------------------------------ | :---------------------------------------------------- | :------------: |
| `success`                             | Claude 正常完成了任務                                        |        是       |
| `error_max_turns`                     | 在完成前達到 `maxTurns` 限制                                  |        否       |
| `error_max_budget_usd`                | 在完成前達到 `maxBudgetUsd` 限制                              |        否       |
| `error_during_execution`              | 錯誤中斷了迴圈（例如，API 失敗或取消的請求）                              |        否       |
| `error_max_structured_output_retries` | 在配置的重試限制內未產生有效的結構化輸出：每次嘗試都未通過驗證，或模型後備撤回了已完成的輸出且沒有成功重試 |        否       |

`result` 欄位（最終文字輸出）僅在 `success` 變體上存在，因此在讀取它之前始終檢查子類型。所有結果子類型都帶有 `total_cost_usd`、`usage`、`num_turns` 和 `session_id`，因此您可以追蹤成本並在錯誤後恢復。在 Python 中，`total_cost_usd` 和 `usage` 被類型化為可選的，在某些錯誤路徑上可能是 `None`，因此在格式化它們之前進行保護。請參閱 [追蹤成本和使用量](/docs/zh-TW/agent-sdk/cost-tracking) 以了解有關解釋 `usage` 欄位的詳細資訊。

<Note>
  當查詢以錯誤結果結束時：

  * 單次 `query()` 呼叫會產生最終結果訊息，然後引發包含失敗文字的錯誤，例如 `Reached maximum number of turns`。引發是有意的 — 如果您的程式碼需要在其後繼續，請將迴圈包裝在 try 區塊中。底層 Claude Code 程序也會以非零代碼退出。
  * 串流輸入工作階段保持活躍，您可以繼續傳送訊息。
</Note>

結果還包括一個 `stop_reason` 欄位（TypeScript 中的 `string | null`、Python 中的 `str | None`），指示模型為什麼在最後一個回合停止生成。常見值是 `end_turn`（模型正常完成）、`max_tokens`（達到輸出代幣限制）和 `refusal`（模型拒絕了請求）。在錯誤結果子類型上，`stop_reason` 帶有迴圈結束前最後一個助手回應的值。要檢測拒絕，請檢查 `stop_reason === "refusal"`（TypeScript）或 `stop_reason == "refusal"`（Python）。請參閱 [`SDKResultMessage`](/docs/zh-TW/agent-sdk/typescript#sdkresultmessage)（TypeScript）或 [`ResultMessage`](/docs/zh-TW/agent-sdk/python#resultmessage)（Python）以了解完整類型。

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/zh-TW/agent-sdk/hooks) 是在迴圈中的特定點觸發的回呼：在工具執行前、執行後、代理程式完成時等。一些常用的 hooks 是：

| Hook                             | 何時觸發         | 常見用途          |
| :------------------------------- | :----------- | :------------ |
| `PreToolUse`                     | 在工具執行前       | 驗證輸入、阻止危險命令   |
| `PostToolUse`                    | 在工具返回後       | 審計輸出、觸發副作用    |
| `UserPromptSubmit`               | 當提示被發送時      | 將額外上下文注入提示    |
| `Stop`                           | 當代理程式完成時     | 驗證結果、保存工作階段狀態 |
| `SubagentStart` / `SubagentStop` | 當子代理程式生成或完成時 | 追蹤和聚合平行任務結果   |
| `PreCompact`                     | 在上下文壓縮前      | 在摘要前存檔完整成績單   |

Hooks 在您的應用程式進程中執行，而不是在代理程式的上下文視窗內，因此它們不消耗上下文。Hooks 也可以短路迴圈：拒絕工具呼叫的 `PreToolUse` hook 會阻止它執行，Claude 會收到拒絕訊息。

兩個 SDK 都支援上述所有事件。TypeScript SDK 包含 Python 尚不支援的額外事件。請參閱 [使用 hooks 控制執行](/docs/zh-TW/agent-sdk/hooks) 以了解完整的事件清單、每個 SDK 的可用性和完整的回呼 API。

<h2 id="put-it-all-together">
  將其全部整合在一起
</h2>

此範例將本頁的關鍵概念組合成一個修復失敗測試的單個代理程式。它使用允許的工具（自動批准，以便代理程式自主執行）、專案設定和回合和推理努力的安全限制來配置代理程式。當迴圈執行時，它捕獲工作階段 ID 以進行潛在恢復、處理最終結果並列印總成本。

因為單一的 `query()` 呼叫在產生錯誤結果後會引發異常，迴圈被包裝在 try 區塊中，以便在達到限制時指令碼能夠乾淨地退出。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  後續步驟
</h2>

現在您了解了迴圈，以下是根據您正在建立的內容去往何處：

* **還沒有執行代理程式？** 從 [快速入門](/docs/zh-TW/agent-sdk/quickstart) 開始，以安裝 SDK 並查看完整範例端到端執行。
* **準備好連接到您的專案？** [加載 CLAUDE.md、技能和檔案系統 hooks](/docs/zh-TW/agent-sdk/claude-code-features)，以便代理程式自動遵循您的專案約定。
* **建立互動式 UI？** 啟用 [串流](/docs/zh-TW/agent-sdk/streaming-output) 以在迴圈執行時顯示即時文字和工具呼叫。
* **需要對代理程式可以做什麼進行更嚴格的控制？** 使用 [權限](/docs/zh-TW/agent-sdk/permissions) 鎖定工具存取，並使用 [hooks](/docs/zh-TW/agent-sdk/hooks) 在工具執行前審計、阻止或轉換工具呼叫。
* **執行長期或昂貴的任務？** 將隔離的工作卸載到 [子代理程式](/docs/zh-TW/agent-sdk/subagents) 以保持主上下文精簡。

有關代理程式迴圈的更廣泛概念圖片（不是 SDK 特定的），請參閱 [Claude Code 如何運作](/docs/zh-TW/how-claude-code-works)。如需在 Claude Code 中設計迴圈的實用指南，從輪流制到目標導向和主動迴圈，請參閱部落格上的 [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops)。
