> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway 協議參考

> Claude Code 與 LLM gateway 之間的 API 契約：端點、要轉發的標頭和請求體欄位、欄位被移除時的功能降級、用於成本追蹤的歸屬標頭，以及模型發現。

本頁面記錄了 Claude Code 發送給 gateway 的請求，包括它呼叫的端點、gateway 必須轉發的標頭和請求體欄位，以及當 gateway 不這樣做時哪些功能會停止運作。本文件是為配置 gateway 產品以與 Claude Code 搭配運作的操作人員編寫的。

執行中的 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 在 `GET /protocol` 提供此契約的機器可讀版本，涵蓋相同的轉發要求以及 Claude apps gateway 特定的端點，用於 SSO 登入、受管設定傳遞和遙測。Claude apps gateway 從與 CLI 相同的 `claude` 二進位檔案執行，因此 [Claude apps gateway 快速入門](/docs/zh-TW/claude-apps-gateway#quickstart) 是取得您可以從中擷取規格的執行中實例的最短路徑。

<Note>
  * 若要為您的組織推出現有或第三方 gateway，請參閱[推出 LLM gateway](/docs/zh-TW/llm-gateway-rollout)
  * 如果您是使用提供給您的認證向 gateway 驗證 Claude Code 的個人開發人員，請參閱[將 Claude Code 連接到 LLM gateway](/docs/zh-TW/llm-gateway-connect)
</Note>

本頁面涵蓋：

* [API 格式](#api-formats)及每種格式要提供的端點
* [請求標頭](#request-headers)：哪些必須到達上游，哪些您的 gateway 可以使用
* [系統提示歸屬區塊](#system-prompt-attribution-block)及其與提示快取的互動方式
* [功能傳遞](#feature-pass-through)：當標頭或請求體欄位被移除時會發生什麼
* [模型發現](#model-discovery)

本頁面使用兩個術語來描述您的 gateway 對每個標頭和請求體欄位的處理方式：

* **轉發不變**：逐位元組傳遞給上游
* **使用**：gateway 可能會讀取它以進行路由、歸屬或追蹤，不需要轉發它

任何未標記為轉發不變的內容都可以由您使用或忽略。

<h2 id="api-formats">
  API 格式
</h2>

Gateway 必須向 Claude Code 用戶端公開以下至少一種 API 格式。Claude Code 使用哪種格式由用戶端的配置決定：下表「選擇者」欄中的變數指向您的 gateway 使用該格式。Google Cloud 的 Agent Platform 是 Google Cloud 的 Claude 端點，前身為 Vertex AI；其變數名稱保留 `VERTEX` 拼寫。

| 格式                                       | 選擇者                                                         | 端點                                                                   | 轉發不變                                                                     |
| :--------------------------------------- | :---------------------------------------------------------- | :------------------------------------------------------------------- | :----------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                        | `/v1/messages`、`/v1/messages/count_tokens`（可選）                       | `anthropic-beta` 和 `anthropic-version` 請求標頭                              |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` 搭配 `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`、`/model/{model}/invoke-with-response-stream` | `anthropic_beta` 和 `anthropic_version` 請求體欄位                             |
| Google Cloud 的 Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` 搭配 `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`、`:streamRawPredict`、`count-tokens:rawPredict`（可選）      | `anthropic-beta` 和 `anthropic-version` 請求標頭，以及 `anthropic_version` 請求體欄位 |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry 和 AWS 上的 Claude Platform
</h3>

Microsoft Foundry 和 [AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws) 實現了 Anthropic Messages 格式。Claude Code 通過它們自己的變數 `ANTHROPIC_FOUNDRY_BASE_URL` 和 `ANTHROPIC_AWS_BASE_URL` 路由到它們，但 gateway 在任一前面實現上述 Anthropic Messages 列。在 AWS 上的 Claude Platform 前面的 gateway 還必須轉發 `anthropic-workspace-id` 標頭，[該平台在每個請求上都需要](/docs/zh-TW/claude-platform-on-aws)。

<h3 id="optional-endpoints-and-startup-traffic">
  可選端點和啟動流量
</h3>

令牌計數端點是唯一可選的：當它們不存在時，Claude Code 會在本地估計上下文使用情況。推理請求發佈到 `/v1/messages?beta=true`，因此請匹配路徑，而不是完整 URL。Google Cloud 的 Agent Platform 方法後綴附加到發佈者模型路徑，如 `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`。

Gateway 也會看到最佳努力的啟動流量，它可以拒絕而不會破壞任何東西：一個 `HEAD /` 連接探測，以及在 Amazon Bedrock 格式 gateway 上的 `GET /inference-profiles?type=SYSTEM_DEFINED` 請求。

<h3 id="streaming">
  串流
</h3>

推理回應必須串流。Claude Code 在接收時使用伺服器發送事件，因此在轉發完整回應之前進行緩衝的 gateway 會使客戶端停滯。

<h3 id="format-mismatch-with-the-upstream">
  與上游的格式不匹配
</h3>

用戶端使用的格式決定了您的 gateway 接收的內容。常見的失敗模式是用戶端發送給您的 gateway 的格式與其後面的上游提供者接受的格式不匹配。

* 當用戶端使用 Amazon Bedrock 或 Google Cloud 的 Agent Platform 格式時，Claude Code 只發送那些提供者接受的完整功能集的子集
* 當用戶端使用 Anthropic Messages 格式時，Claude Code 發送完整集合，即使您的 gateway 轉發到 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上游

橋接該差異是您的 gateway 的工作。[功能傳遞](#feature-pass-through)描述了當它不這樣做時會發生什麼。

<h2 id="request-headers">
  請求標頭
</h2>

Claude Code 在 API 請求上包含這些標頭。標頭名稱在線路上不區分大小寫。轉發 `anthropic-version` 和 `anthropic-beta` 不變，加上當上游是 [AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws) 時的 `anthropic-workspace-id`；其餘的 gateway 可以使用以進行路由、歸屬和追蹤，不需要轉發。

| 標頭                              | 描述                                                                                                                                                                          |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization`、`x-api-key`     | 開發人員的 gateway 認證，根據他們設定的[認證變數](/docs/zh-TW/llm-gateway-connect#set-the-credential-variable)在一個或兩個標頭中                                                                             |
| `anthropic-version`             | API 版本，目前為 `2023-06-01`。Amazon Bedrock 和 Google Cloud 的 Agent Platform 格式請求也攜帶 `anthropic_version` 請求體欄位，其值是提供者方言字串，而不是此標頭的值                                                |
| `anthropic-beta`                | 請求的逗號分隔功能值。逐字轉發標頭；不要將個別值列入允許清單，因為該集合隨 Claude Code 版本而變化。當開發人員使用 claude.ai 登入進行驗證時（當設定 `ANTHROPIC_BASE_URL` 而沒有 gateway 認證變數時可能），此標頭也會攜帶上游需要的 OAuth 功能，移除它會導致這些請求失敗並出現 `401` |
| `x-claude-code-session-id`      | 目前 Claude Code 工作階段的唯一識別碼。使用它來聚合來自一個工作階段的所有請求，而無需解析請求體                                                                                                                      |
| `x-claude-code-agent-id`        | 發出請求的[子代理](/docs/zh-TW/sub-agents)的識別碼，僅在來自 Claude Code 在工作階段內生成的代理的請求上存在。將其與工作階段 ID 一起使用以將成本歸屬於平行代理                                                                             |
| `x-claude-code-parent-agent-id` | 生成請求代理的代理的識別碼，僅對嵌套代理存在                                                                                                                                                      |

子代理 ID 在每次生成時都會新生成。隊友代理（[代理團隊](/docs/zh-TW/agent-teams)的命名成員）在重新連接時重複使用穩定的基於名稱的 ID。在兩種情況下，ID 都識別一個代理，而不是一個人或設備，因此不要將代理 ID 標頭視為使用者識別碼。

如果您的開發人員設定了 `ANTHROPIC_CUSTOM_HEADERS`，這些標頭也會出現在請求上。

<h3 id="forward-as-open-lists">
  作為開放清單轉發
</h3>

將標頭和請求體欄位視為開放清單，而不是封閉清單。Claude Code 在版本中獲得功能，它們作為新的 `anthropic-beta` 值、新的請求體欄位以及偶爾新的 `anthropic-*` 或 `x-claude-code-*` 標頭到達。

轉發到 Anthropic 格式上游時，傳遞 `anthropic-*` 請求標頭和請求體欄位不變，而不是將您今天看到的列入允許清單。固定到觀察清單的 gateway 會移除下一個功能的標頭或欄位，並在引入它的版本上破壞它。

例外是非 Anthropic 上游（如 Amazon Bedrock 或 Google Cloud 的 Agent Platform），其中橋接架構差異是 gateway 的工作；請參閱[功能傳遞](#feature-pass-through)。

<h2 id="system-prompt-attribution-block">
  系統提示歸屬區塊
</h2>

Claude Code 在系統提示前面加上一個簡短的歸屬區塊，其中包含用戶端版本和從對話衍生的指紋。`api.anthropic.com` 端點在處理前移除該區塊，因此它不會影響第一方提示快取；任何其他上游都會將其作為提示的一部分接收。

該移除是位置性的，因此只有在 gateway 轉發 `system` 陣列保持不變時才有效。若要在不遺失其他系統內容的情況下將區塊排除在提示之外：

* 完全按照接收的方式轉發 `system` 陣列，將區塊保持在最前面：在前面加上另一個系統區塊、重新排序陣列或將其轉換為單一字串會破壞移除，區塊隨後會到達模型和提示快取鍵。
* 將區塊保持在自己的陣列項目中：端點將以歸屬標頭開頭的合併區塊視為完整的歸屬並刪除合併到其中的所有內容，包括系統提示的其餘部分。
* 如果您的 gateway 必須重新塑造系統內容，請設定 [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/zh-TW/env-vars) 以便 Claude Code 省略該區塊。Anthropic 和雲提供者的 Claude 端點讀取該區塊以進行歸屬，因此要省略它，請在用戶端而不是在 gateway 中移除或移動它。

未經修改到達端點的請求不受影響。

從 Claude Code v2.1.181 開始，當請求通過自訂基礎 URL 路由時，該區塊在對話的生命週期內是穩定的，因此以完整請求體為鍵的 gateway 端提示快取可以在不禁用它的情況下工作。在 v2.1.181 之前，該區塊包含每個請求的令牌；在這些版本上，如果您的 gateway 實現了這樣的快取，請設定 `CLAUDE_CODE_ATTRIBUTION_HEADER=0`。

<h2 id="feature-pass-through">
  功能傳遞
</h2>

Claude Code 將 `ANTHROPIC_BASE_URL` gateway 視為 Anthropic 格式端點，並向其發送它發送給 `api.anthropic.com` 的測試版標頭和請求體欄位，除了為直接連接保留的一小組診斷和預設值，例如下面涵蓋的細粒度工具串流預設。該集合因版本而異，因此不要依賴其內容。

添加請求體欄位的功能將它們與測試版標頭配對，該對一起傳遞。移除標頭同時傳遞請求體的 gateway，或將 Anthropic 格式請求體轉發到具有不同架構的上游，會產生硬 `400` 錯誤；只有當兩個部分一起不存在時，功能才會安靜地關閉。重寫或編輯請求體以進行內容檢查的 gateway 會以與移除相同的方式破壞配對，因此請在不修改的情況下檢查。該表注意了功能偏離配對的位置。

細粒度工具串流是直接連接預設值之一：每當請求通過自訂基礎 URL 路由時，它預設為關閉，當開發人員設定 [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/zh-TW/env-vars) 時，gateway 會接收它。

| 功能                                                                                                                                                                                                                | 標頭和請求體對                                                                                                        | 破壞時的症狀                                                                                                              | 補救                                                                                  |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------- |
| [自適應推理](/docs/zh-TW/model-config#adjust-effort-level)                                                                                                                                                                  | 無測試版標頭。Claude Code 為 Claude 4.6 及更新版本發送 `thinking: {"type": "adaptive"}`，並將它不識別的模型名稱（如 gateway 別名）視為接收該欄位的目前模型 | 當上游模型組建不接受它時，命名 `thinking` 欄位或 `adaptive` 標籤的 `400`                                                                 | 升級上游。在 Opus 4.6 和 Sonnet 4.6 上，開發人員可以改為設定 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` |
| [上下文管理](https://platform.claude.com/docs/en/build-with-claude/context-management)                                                                                                                                 | 上下文管理測試版標頭與 `context_management` 請求體欄位配對                                                                       | `400` 搭配 `Extra inputs are not permitted`。常見於 gateway 接受 Anthropic 格式請求但將其轉發到 Amazon Bedrock 時                      | 轉發兩者，或 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/zh-TW/env-vars)                |
| [擴展上下文](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)和[交錯思考](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | 僅測試版標頭，無請求體欄位                                                                                                  | 當標頭被移除時無聲地不可用；上游永遠不會看到功能請求                                                                                          | 逐字轉發 `anthropic-beta`                                                               |
| 測試版[工具欄位](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                 | 工具相關的測試版標頭與工具架構欄位（如 `strict` 和 `defer_loading`）配對                                                              | 當請求體在沒有其標頭的情況下通過時，命名無法識別的工具架構欄位的 `400`                                                                              | 轉發兩者，或 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                   |
| [努力](https://platform.claude.com/docs/en/build-with-claude/effort)和[結構化輸出](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                              | `output_config` 請求體欄位攜帶努力、結構化輸出格式和任務預算設定；每個都與其自己的測試版標頭配對                                                       | 在 Amazon Bedrock 和 Google Cloud 的 Agent Platform 上游上命名 `output_config` 的 `400`，通常是 `Extra inputs are not permitted` | 一起轉發欄位及其標頭                                                                          |
| [令牌計數](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                      | 無測試版配對；使用 `count_tokens` 端點                                                                                    | Claude Code 回退到在本地估計上下文使用情況                                                                                         | 如果您想要精確計數，請公開端點                                                                     |

`ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [變數](/docs/zh-TW/model-config)僅在提供者配置中聲明模型功能：`CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_VERTEX`、`CLAUDE_CODE_USE_FOUNDRY` 和 [`CLAUDE_CODE_USE_MANTLE`](/docs/zh-TW/amazon-bedrock#use-the-mantle-endpoint)。它們在 `ANTHROPIC_BASE_URL` gateway 後面沒有效果。

<h3 id="automatic-retry-and-error-forwarding">
  自動重試和錯誤轉發
</h3>

Claude Code 在某些上游拒絕後自動重試，並為對話的其餘部分禁用被拒絕的功能。`thinking` 欄位的拒絕、[思考簽名](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)的拒絕和中途對話系統訊息的拒絕都以這種方式恢復。上下文管理和工具架構欄位拒絕不重試；這些 `400` 錯誤到達開發人員。

重試邏輯與上游的錯誤措辭相匹配，因此不修改地轉發錯誤回應體。在自己的信封中包裝上游錯誤的 gateway 會破壞恢復路徑，即使它保留了狀態碼。

<h3 id="disable-pre-release-capabilities">
  禁用預發佈功能
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` 停止 Claude Code 在每個提供者上發送預發佈功能及其請求體欄位，包括上下文管理和測試版工具欄位。它不影響自適應推理，後者由模型而不是測試版選擇，它永遠不會抑制訂閱驗證所需的 OAuth 功能。

Claude Code 發送的功能集在版本中增長。有關目前的測試版標頭字串，請參閱[測試版標頭參考](https://platform.claude.com/docs/en/api/beta-headers)；針對新的 Claude Code 版本測試您的 gateway，而不是固定到觀察清單。

<h2 id="model-discovery">
  模型發現
</h2>

當 `ANTHROPIC_BASE_URL` 指向公開 Anthropic Messages 格式的 gateway 時，Claude Code 可以在啟動時查詢 gateway 的 `/v1/models` 端點，並將返回的模型添加到 `/model` 選擇器。

開發人員通過在自己的環境中或通過受管設定設定 [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/zh-TW/env-vars) 來啟用它。預設情況下發現是關閉的，以便由共享 API 金鑰支持的 gateway 不會向每個使用者公開金鑰可以存取的每個模型。這需要 Claude Code v2.1.129 或更新版本。

<h3 id="when-discovery-runs">
  發現何時運行
</h3>

發現僅適用於 Anthropic Messages 格式。當以下情況時不運行：

* 設定了任何 `CLAUDE_CODE_USE_*` 提供者變數，即使也設定了 `ANTHROPIC_BASE_URL`
* `ANTHROPIC_BASE_URL` 未設定或指向 `api.anthropic.com`
* 非必要流量被禁用，通過 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/zh-TW/env-vars) 或組織政策

<h3 id="request-and-response">
  請求和回應
</h3>

請求是 `GET /v1/models?limit=1000`，超時時間為 3 秒，任何重定向都被視為失敗，因此認證不會洩露給重定向目標。回應緩慢或重定向 `/v1/models` 的 gateway，即使是 `http` 到 `https`，也會無聲地失敗發現；在配置的基礎 URL 處直接提供端點。

發現請求恰好發送一個認證標頭：

* 設定時 `ANTHROPIC_AUTH_TOKEN` 作為持有人令牌
* 否則解析的 API 金鑰，包括 [`apiKeyHelper`](/docs/zh-TW/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 值，在 `x-api-key` 標頭中

這與推理請求不同，後者在兩個標頭中發送幫助程式值。驗證 `/v1/models` 的 gateway 必須為幫助程式部署接受 `x-api-key`。`ANTHROPIC_CUSTOM_HEADERS` 中的任何標頭也包括在內。

Claude Code 從回應的 `data` 陣列中的每個條目讀取 `id` 和可選的 `display_name`，並忽略其 `id` 不以 `claude` 或 `anthropic` 開頭的條目：

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  選擇器條目和快取
</h3>

選擇器是當開發人員在 Claude Code 中運行 `/model` 時打開的互動式模型清單。每個發現的條目都標記為「來自 gateway」，並在提供時使用 `display_name`。[`availableModels` 受管設定](/docs/zh-TW/settings#available-settings)限制發現可以添加的內容。

發現的 ID 僅在它完全匹配選擇器中已有的列，或當發現的和現有的 ID 都解析為 [Fable](/docs/zh-TW/model-config#work-with-fable-5) 時才被跳過。自 Claude Code v2.1.197 起，發現的明確 ID 在兩者都解析為同一模型時也會折疊到內建條目中。內建列由別名（如 `sonnet`）鍵入，因此發現的明確 ID（如該別名目前解析到的模型 `claude-sonnet-5`）會折疊到 `sonnet` 列中，而別名不解析到的 ID（如 `claude-sonnet-4-6`）仍會在內建條目旁邊添加其自己的「來自 gateway」列。

結果被快取到 `~/.claude/cache/gateway-models.json`，或在 Windows 上 `%USERPROFILE%\.claude\cache\gateway-models.json`，並在每次啟動時刷新。如果請求失敗或 gateway 未實現 `/v1/models`，選擇器會回退到上次啟動的快取清單或內建模型清單。如果您的 gateway 在不匹配發現篩選器的別名下提供 Claude 模型，開發人員可以使用[模型配置](/docs/zh-TW/model-config)變數手動添加這些別名。

<h2 id="related-resources">
  相關資源
</h2>

有關 gateway 文件集的其餘部分和基礎 API 參考：

* [Gateway 概述](/docs/zh-TW/gateways)：什麼是 gateway 以及如何在 Claude 應用程式 gateway 和其他產品之間進行選擇
* [其他 LLM gateway](/docs/zh-TW/llm-gateway)：如何推出您的組織執行的 gateway 以及它如何與 claude.ai 訂閱互動
* [為您的組織推出 LLM gateway](/docs/zh-TW/llm-gateway-rollout)：使用此契約的管理員檢查清單
* [將 Claude Code 連接到 LLM gateway](/docs/zh-TW/llm-gateway-connect)：每個開發人員的配置和故障排除表
* [測試版標頭參考](https://platform.claude.com/docs/en/api/beta-headers)：目前的 `anthropic-beta` 值集合
* [Messages API](https://platform.claude.com/docs/en/api/messages)：Anthropic 格式 gateway 實現的 API 格式
