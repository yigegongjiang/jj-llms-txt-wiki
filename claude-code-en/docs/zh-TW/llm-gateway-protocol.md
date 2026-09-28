> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 閘道相容性指南

> 保持 LLM 閘道與 Claude Code 相容：它呼叫的端點、必須轉發的標頭和本體欄位，以及移除它們時會中斷的功能。

本頁面記錄 Claude Code 傳送至閘道的請求，包括它呼叫的端點、閘道必須轉發的標頭和本體欄位，以及當閘道未轉發時停止運作的功能。本文件是為配置閘道產品以與 Claude Code 搭配運作的操作人員撰寫的。

[Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 是 Anthropic 的自託管閘道，在 `GET /protocol` 提供自己的端點參考，涵蓋該閘道的登入、推論、受管設定、模型探索和遙測端點。這是與本指南分開的文件。

<Note>
  * 若要為您的組織推出現有或第三方閘道，請參閱[推出 LLM 閘道](/docs/zh-TW/llm-gateway-rollout)
  * 如果您是使用獲得的認證向閘道驗證 Claude Code 的個別開發人員，請參閱[將 Claude Code 連線至 LLM 閘道](/docs/zh-TW/llm-gateway-connect)
</Note>

本頁面涵蓋：

* [API 格式](#api-formats)和每種格式要提供的端點
* [依連線方法的用戶端行為](#how-the-connection-method-changes-client-behavior)：模型 ID、`anthropic-beta` 值、請求欄位和預設值在格式和 Claude apps gateway 登入之間的差異
* [請求標頭](#request-headers)：哪些必須到達上游，以及您的閘道可以使用哪些
* [回應標頭](#response-headers)：要傳回什麼以便停滯偵測、重試和使用量限制顯示能夠運作
* [系統提示屬性區塊](#system-prompt-attribution-block)及其與提示快取的互動方式
* [功能傳遞](#feature-pass-through)：移除標頭或本體欄位時會中斷的功能
* [模型探索](#model-discovery)

本頁面使用兩個術語來說明您的閘道對每個標頭和本體欄位的處理方式：

* **轉發不變**：將其逐位元組傳遞至上游
* **使用**：閘道可能會讀取它以進行路由、屬性或追蹤，不需要轉發它

任何未標記為轉發不變的內容都可供您使用或忽略。

<h2 id="api-formats">
  API 格式
</h2>

閘道必須向 Claude Code 用戶端公開以下至少一種 API 格式。用戶端會選擇一種格式，並使用下表「選擇者」欄中的變數將 Claude Code 指向您的閘道。

Google Cloud 的 Agent Platform 是 Google Cloud 的 Claude 端點，前身為 Vertex AI；其變數名稱保留 `VERTEX` 拼寫。

| 格式                                       | 選擇者                                                         | 端點                                                                                                     | 轉發不變                                                                      |
| :--------------------------------------- | :---------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                        | `/v1/messages`、`/v1/messages/count_tokens`（選用）                                                         | `anthropic-beta` 和 `anthropic-version` 請求標頭                               |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` 搭配 `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`、`/model/{model}/invoke-with-response-stream`、`/model/{model}/count-tokens`（選用） | `anthropic_beta` 和 `anthropic_version` 請求本體欄位                             |
| Google Cloud 的 Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` 搭配 `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`、`:streamRawPredict`、`count-tokens:rawPredict`（選用）                                        | `anthropic-beta` 和 `anthropic-version` 請求標頭，以及 `anthropic_version` 請求本體欄位 |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry 和 AWS 上的 Claude Platform
</h3>

Microsoft Foundry 和 [AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws) 實作 Anthropic Messages 格式。Claude Code 透過自己的變數 `ANTHROPIC_FOUNDRY_BASE_URL` 和 `ANTHROPIC_AWS_BASE_URL` 路由到它們，但閘道在任一前面實作上述 Anthropic Messages 列。閘道在 AWS 上的 Claude Platform 前面也必須轉發 `anthropic-workspace-id` 標頭，[該平台在每個請求上都需要](/docs/zh-TW/claude-platform-on-aws)。

<h3 id="optional-endpoints-and-startup-traffic">
  選用端點和啟動流量
</h3>

權杖計數端點是唯一的選用端點：當它們不存在時，Claude Code 會回退到基於字元的內容使用估計。

根據路徑而非完整 URL 進行比對：

* 推論請求發佈到 `/v1/messages?beta=true`
* Google Cloud 的 Agent Platform 方法尾碼附加到發佈者模型路徑，如 `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

閘道也會看到最佳努力啟動流量，可以拒絕而不會破壞任何東西。Anthropic Messages 格式閘道會收到 `HEAD /api/hello` 連線預熱探測，當設定了 HTTP 代理或用戶端憑證時，Claude Code 會跳過此探測。Amazon Bedrock 格式閘道會收到 `GET /inference-profiles?type=SYSTEM_DEFINED` 請求，以及當設定的模型是推論設定檔時，`GET /inference-profiles/{profile}` 查詢。

[快速模式](/docs/zh-TW/fast-mode)可用性檢查永遠不會出現在閘道日誌中：它直接呼叫 `api.anthropic.com` 而不是遵循 `ANTHROPIC_BASE_URL`，因此在阻止直接出站到 `api.anthropic.com` 的網路上，快速模式可能會報告連線錯誤，而透過閘道的推論會繼續運作。[WebFetch 網域安全檢查](/docs/zh-TW/data-usage#webfetch-domain-safety-check)也直接呼叫 `api.anthropic.com`。[在代理和 LLM 閘道後面使用快速模式](/docs/zh-TW/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)涵蓋恢復它的變數。

<h3 id="streaming">
  串流
</h3>

串流推論回應。Claude Code 在到達時讀取串流，因此如果您的閘道在轉發前緩衝完整回應，Claude Code 會停滯。

當用戶端使用 Amazon Bedrock 格式時，不修改地轉發 `InvokeModelWithResponseStream` 回應本體及其 `Content-Type: application/vnd.amazon.eventstream` 標頭，並且不要將串流轉換為伺服器發送事件。請參閱[閘道或代理後面的串流錯誤](/docs/zh-TW/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)。

也轉發保活 ping。在透過 `ANTHROPIC_BASE_URL` 或 `ANTHROPIC_AWS_BASE_URL` 的連線上，Claude Code 計算您的閘道轉發的每一位元組，包括 SSE `ping` 事件和註解行，並預設在 300 秒內中止無聲的串流。上游的 ping 是長思考暫停期間唯一的流量，因此如果您的閘道剝離或緩衝它們，Claude Code 會在這些暫停期間中止串流；[自動重試](/docs/zh-TW/errors#automatic-retries)涵蓋根據回應進度有多遠而中止的串流報告。完全不發送 ping 的上游（例如 Amazon Bedrock 的二進位事件串流）在這些暫停期間沒有任何東西可轉發。從這樣的上游轉譯時，在無聲間隙期間發出您自己的 `ping` 事件。透過 `ANTHROPIC_BEDROCK_BASE_URL`、`ANTHROPIC_VERTEX_BASE_URL` 或 `ANTHROPIC_FOUNDRY_BASE_URL` 到達的閘道不會被此位元組級監視狗包裝，即使它們轉發 Anthropic Messages 格式；在那裡，[5 分鐘閒置逾時](/docs/zh-TW/env-vars)會改為中止無聲串流，在 `ANTHROPIC_BEDROCK_BASE_URL` 連線上，您可以使用 [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/zh-TW/env-vars) 新增位元組監視狗。

<h3 id="format-mismatch-with-the-upstream">
  與上游的格式不匹配
</h3>

用戶端使用的格式決定了您的閘道接收的內容。常見的失敗模式是用戶端發送到您的閘道的格式與其後面的上游提供者接受的格式不匹配。

* 當用戶端使用 Amazon Bedrock 或 Google Cloud 的 Agent Platform 格式時，Claude Code 只發送那些提供者接受的完整功能集的子集
* 當用戶端使用 Anthropic Messages 格式時，Claude Code 發送完整集，即使您的閘道轉發到 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上游

橋接該差異是您的閘道的工作。[功能傳遞](#feature-pass-through)描述當它不這樣做時會破壞什麼。

如果您的上游是 Amazon Bedrock 或 Google Cloud 的 Agent Platform，您可以透過改為公開該提供者的格式來避免橋接。[透過閘道路由到雲端提供者](/docs/zh-TW/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway)顯示該格式的用戶端設定。

<h2 id="how-the-connection-method-changes-client-behavior">
  連線方法如何改變用戶端行為
</h2>

開發人員連線到您的閘道的方式決定了 Claude Code 傳送的模型 ID、`anthropic-beta` 值和請求欄位，以及它套用的預設值。您的閘道會看到以下三種用戶端行為之一：

* **Amazon Bedrock 或 Agent Platform 格式**：開發人員設定 `CLAUDE_CODE_USE_BEDROCK=1` 搭配 `ANTHROPIC_BEDROCK_BASE_URL`，或 `CLAUDE_CODE_USE_VERTEX=1` 搭配 `ANTHROPIC_VERTEX_BASE_URL`，指向您的閘道。Claude Code 使用該提供者的模型 ID、請求欄位和預設值。
* **Anthropic Messages 格式**：開發人員將 `ANTHROPIC_BASE_URL` 設定為您的閘道。Claude Code 將閘道視為 Claude API，無法判斷您轉發到哪個上游。
* **Claude apps 閘道登入**：開發人員登入 [Claude apps 閘道](/docs/zh-TW/claude-apps-gateway)。該閘道使用 Anthropic Messages 格式，但可以路由到任何上游，因此 Claude Code 只傳送 Amazon Bedrock 和 Agent Platform 也接受的 `anthropic-beta` 值和模型功能假設。

<h3 id="requests-and-defaults-by-connection-method">
  按連線方法的請求和預設值
</h3>

下表比較三種連線方法，每行一個行為。它省略了 Microsoft Foundry 和 Claude Platform on AWS，它們也使用 Anthropic Messages 格式，但 Claude Code 透過自己的變數到達它們。如需這些，請參閱 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 和 [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 頁面。

| 行為                                                                                      | Amazon Bedrock 或 Agent Platform 格式                                                                                                                            | Anthropic Messages 格式                                                                                                        | Claude apps 閘道登入                                                                       |
| :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| 預設情況下請求中的模型 ID                                                                          | 提供者的形式，例如 Amazon Bedrock 上的 `us.anthropic.claude-opus-4-8`                                                                                                    | Anthropic ID，例如 `claude-opus-4-8`                                                                                            | Anthropic ID                                                                           |
| 傳送的 `anthropic-beta` 值                                                                  | Amazon Bedrock 和 Agent Platform 接受的子集                                                                                                                         | [功能傳遞](#feature-pass-through)下描述的完整集合，除非開發人員設定 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities) | Amazon Bedrock 和 Agent Platform 接受的子集                                                  |
| Claude Code 無法識別的模型 ID（例如閘道別名）的請求欄位                                                     | 使用固定預算進行思考而非自適應推理，且沒有努力或內容管理欄位                                                                                                                                | 目前 Claude 模型在 Claude API 上接受的所有內容，包括自適應推理、努力和內容管理，Amazon Bedrock 或 Agent Platform 上游可能會拒絕                                    | 與 Amazon Bedrock 或 Agent Platform 格式相同                                                 |
| 開發人員選擇加入時的一小時 [prompt 快取 TTL](/docs/zh-TW/prompt-caching#choose-the-ttl-yourself)            | 透過 `cache_control` 中的 `ttl` 欄位請求，沒有測試版值                                                                                                                       | 透過 `ttl` 欄位加上 `anthropic-beta` 中的 `extended-cache-ttl` 值請求，您必須轉發                                                             | 請參閱 Claude apps 閘道 [可用性和限制](/docs/zh-TW/claude-apps-gateway#availability-and-limitations) 表 |
| [背景工作](/docs/zh-TW/costs#background-token-usage) 的模型，除非 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 固定一個 | 預設 Sonnet 模型，或選擇主模型後的主模型，如 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock#4-pin-model-versions) 和 [Agent Platform](/docs/zh-TW/google-vertex-ai#5-pin-model-versions) 頁面所述 | 主模型，或當 `ANTHROPIC_API_KEY` 或 `apiKeyHelper` 提供 Anthropic Console 金鑰且 `ANTHROPIC_AUTH_TOKEN` 未設定時的預設 Haiku 模型                 | 主模型                                                                                    |

如需每個連線支援的功能以及它預設傳送給 Anthropic 的遙測，請參閱 [功能可用性](/docs/zh-TW/feature-availability#availability-by-model-provider) 和 [按 API 提供者的預設行為](/docs/zh-TW/data-usage#default-behaviors-by-api-provider)。

<h3 id="settings-for-unrecognized-model-ids">
  無法識別的模型 ID 的設定
</h3>

兩個用戶端設定會改變 Claude Code 對無法識別的模型 ID 的假設，無論開發人員使用哪種連線方法：

* **內容視窗**：Claude Code 假設 200K，或當 ID 帶有 `[1m]` 時為 1M。若要宣告實際視窗，請參閱 [更正閘道或自訂模型 ID 的視窗](/docs/zh-TW/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **功能**：若要給閘道別名提供其背後模型的功能，請在您分發的設定中使用 [`modelOverrides`](/docs/zh-TW/errors#unrecognized-model-id-on-a-request) 項目將該模型的 Anthropic ID 對應到您的別名。如需 `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` 變數適用的位置，請參閱 [功能傳遞](#feature-pass-through)

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

<h3 id="gateway-hint-headers">
  Gateway 提示標頭
</h3>

Claude Code 也可以傳送路由提示：gateway 或路由器可以用來排程、快取或歸屬請求的每個請求事實。需要 Claude Code v2.1.273 或更新版本。

請求是否攜帶它們取決於 Claude Code 將其傳送到何處：

* 直接連接到 Anthropic API：預設傳送
* 自訂基礎 URL：預設關閉，因為拒絕未知標頭的代理會導致請求失敗。若要接收它們，請為您的開發人員設定 [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/zh-TW/env-vars)，例如在[受管設定](/docs/zh-TW/managed-settings)的 `env` 區塊中
* 任何其他後端，包括 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和 AWS 上的 Claude Platform：僅在設定 `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` 時傳送

將 `CLAUDE_CODE_GATEWAY_HINT_HEADERS` 設定為 `0` 會停止每個連接上的標頭。

標頭只攜帶下面列出的內容：固定詞彙、工具名稱和持續時間，永遠不會是提示文字或檔案內容。每個值都是可列印的 ASCII。

| 標頭                                  | 描述                                                                                                                                                                                                                                                               |
| :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `x-claude-code-request-class`       | 這是什麼類型的請求：`main` 用於主對話的一個回合，`subagent` 用於[子代理](/docs/zh-TW/sub-agents)的一個回合，`workflow` 用於在工作流程內執行的代理，`compaction` 用於壓縮對話的摘要請求，或 `auxiliary` 用於側面請求，例如工作階段標題、分類器和摘要。在每個請求上傳送                                                                                           |
| `x-claude-code-agent-type`          | 發出請求的子代理類型：內建代理類型名稱，例如 `Explore`、`Plan` 或 `general-purpose`，或 `custom` 用於使用者定義的代理，`teammate` 用於在主導者程序中執行的[代理團隊](/docs/zh-TW/agent-teams)成員，或 `fork` 用於[分支](/docs/zh-TW/sub-agents#fork-the-current-conversation)。僅在子代理自己的回合上存在；子代理的壓縮或側面請求保留代理 ID，但不攜帶類型。使用者選擇的代理名稱永遠不會被傳送 |
| `x-claude-code-compaction`          | 在[壓縮](/docs/zh-TW/prompt-caching#compacting-the-conversation)期間摘要對話的請求上存在。該值說明觸發了什麼：`auto` 當上下文視窗接近容量時，`manual` 用於 `/compact`，或 `reactive` 當 API 拒絕請求太長時。在所有其他請求上不存在                                                                                                  |
| `x-claude-code-context-compacted`   | 在壓縮後的第一個主對話請求上存在一次，具有與 `x-claude-code-compaction` 相同的值。此請求之前的對話前綴不再使用，因此可以刪除以其為鍵的快取                                                                                                                                                                              |
| `x-claude-code-prev-tool-durations` | 此請求攜帶其結果的工具呼叫的測量執行時間，格式為 `<name>=<ms>;<name>=<ms>`，例如 `Bash=742;Read=9`。在同一對話的下一個請求之後傳送，來自主工作階段或子代理的一批工具呼叫                                                                                                                                                       |

在解析 `x-claude-code-prev-tool-durations` 之前，請檢查 Claude Code 如何建立該值以及它遺漏了什麼：

* 項目：每個執行的工具呼叫一個，按其結果被收集的順序，以整毫秒為單位
* 上限：Claude Code 最多傳送 32 個項目和 4 KB，保留第一個項目
* 編碼：工具名稱是百分比編碼的，涵蓋 `%`、`;`、`=`、逗號、空格和任何超出可列印 ASCII 的字元
* 解析：在 `;` 上分割，然後在 `=` 上分割，並解碼每個名稱
* 缺失：壓縮呼叫、側面請求和新提示的第一個請求永遠不會攜帶它。不要將缺失的標頭讀作執行無工具的回合
* 時間：每個時間都排除了權限提示和 hooks，平行工具呼叫各自報告自己的時間，因此項目不會加起來等於請求之間的間隙

<h3 id="forward-as-open-lists">
  作為開放清單轉發
</h3>

將標頭和請求體欄位視為開放清單，而不是封閉清單。Claude Code 在版本中獲得功能，它們作為新的 `anthropic-beta` 值、新的請求體欄位以及偶爾新的 `anthropic-*` 或 `x-claude-code-*` 標頭到達。

轉發到 Anthropic 格式上游時，傳遞 `anthropic-*` 請求標頭和請求體欄位不變，而不是將您今天看到的列入允許清單。固定到觀察清單的 gateway 會移除下一個功能的標頭或欄位，並在引入它的版本上破壞它。

例外是非 Anthropic 上游（如 Amazon Bedrock 或 Google Cloud 的 Agent Platform），其中橋接架構差異是 gateway 的工作；請參閱[功能傳遞](#feature-pass-through)。

<h2 id="response-headers">
  回應標頭
</h2>

Claude Code 讀取這些回應標頭以偵測停滯的串流、決定是否以及何時重試，以及顯示使用量限制。該表列出每個標頭應返回的內容。同時未修改地轉發錯誤回應本體，以便 Claude Code 的[能力拒絕復原](#automatic-retry-and-error-forwarding)可以符合上游的錯誤措辭。

| 標頭                              | 應返回的內容及原因                                                                                                                                                                                                                                    |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content-type`                  | 在串流的 Anthropic Messages 格式回應上返回 `text/event-stream`，在 Amazon Bedrock 格式回應上返回 `application/vnd.amazon.eventstream`（未修改），其中[不同的類型會導致請求失敗](/docs/zh-TW/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)。[串流](#streaming)列出哪些連線在這些串流上執行停滯偵測 |
| `retry-after`                   | 返回整數秒數而非 HTTP 日期。Claude Code 在下一次[自動重試](/docs/zh-TW/errors#automatic-retries)之前至少等待該時間長度，在 [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/zh-TW/env-vars) 工作階段外，超過 60 的值會停止重試並立即顯示錯誤                                                                             |
| `x-should-retry`                | 未修改地傳遞上游的值。Claude Code 在決定是否重試失敗的請求時將此標頭讀取為一個輸入：`true` 標記回應為可重試，`false` 標記為不可重試。如需重試計數、退避和 Claude Code 重試的失敗，請參閱[自動重試](/docs/zh-TW/errors#automatic-retries)                                                                                      |
| `anthropic-ratelimit-unified-*` | 在每個回應上未修改地轉發上游的值。Claude Code 在成功回應上讀取它們以向使用 claude.ai 登入的開發人員顯示針對計畫限制的使用量，在 `429` 上讀取以區分計畫限制或支出上限與暫時性節流；請參閱[使用量限制](/docs/zh-TW/errors#usage-limits)                                                                                               |

<h2 id="system-prompt-attribution-block">
  系統提示歸屬區塊
</h2>

Claude Code 在系統提示前面加上一個簡短的歸屬區塊，其中包含用戶端版本和從對話衍生的指紋。`api.anthropic.com` 端點在處理前移除該區塊，因此它不會影響第一方提示快取；任何其他上游都會將其作為提示的一部分接收。

該移除是位置性的，因此只有在 gateway 轉發 `system` 陣列保持不變時才有效。若要在不遺失其他系統內容的情況下將區塊排除在提示之外：

* 完全按照接收的方式轉發 `system` 陣列，將區塊保持在最前面：在前面加上另一個系統區塊、重新排序陣列或將其轉換為單一字串會破壞移除，區塊隨後會到達模型和提示快取鍵。
* 將區塊保持在自己的陣列項目中：端點將以歸屬標頭開頭的合併區塊視為完整的歸屬並刪除合併到其中的所有內容，包括系統提示的其餘部分。
* 如果您的 gateway 必須重新塑造系統內容，請設定 [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/zh-TW/env-vars) 以便 Claude Code 省略該區塊。Anthropic 和雲提供者的 Claude 端點讀取該區塊以進行歸屬，因此要省略它，請在用戶端而不是在 gateway 中移除或移動它。

該變數存在是為了 gateway 和第三方快取相容性，而不是作為隱私控制：在直接連線上，完整請求無論如何都已經進入 Anthropic API。當以下兩項都成立時，Claude Code 會在 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 分類器請求上保留該區塊，即使您將變數設定為 `0`：

* 請求進入 `api.anthropic.com`，`ANTHROPIC_BASE_URL` 未設定或命名該主機，且未選擇第三方提供者。
* 作用中的認證不是 [Anthropic 設定檔或聯盟認證](/docs/zh-TW/authentication#anthropic-profiles-and-federation-credentials)。

分類器請求會跳過 Claude Code 系統提示的其餘部分，因此在這些請求上，該區塊是請求體中唯一識別它們為 Claude Code 流量的標記。當任一條件失敗時，通過 LLM gateway、在第三方提供者上，或使用作用中的設定檔或聯盟認證，設定 `0` 也會從分類器請求中移除該區塊。在 v2.1.229 之前，此例外不存在：設定 `0` 會從這些分類器請求中移除該區塊，當 API 拒絕未識別的請求時，auto mode 在它發送給分類器的每個動作上都會失敗。

從 Claude Code v2.1.181 開始，當請求通過自訂基礎 URL 路由時，該區塊在對話的生命週期內是穩定的，因此以完整請求體為鍵的 gateway 端提示快取可以在不禁用它的情況下工作，且您的 gateway 轉發到的任何提供者都會接收穩定的提示前綴。在 v2.1.181 之前，該區塊包含每個請求的令牌，在請求的開始處改變了系統提示。在這些版本上，當您的 gateway 執行以下任一操作時，請設定 `CLAUDE_CODE_ATTRIBUTION_HEADER=0`：

* 實現以請求體為鍵的提示快取。
* 將請求轉發到第三方提供者，例如 Amazon Bedrock、Microsoft Foundry 或 Google Cloud 的 Agent Platform，採用 Anthropic Messages 格式或提供者自己的格式，其中變化的前綴會減少該提供者上的提示快取重複使用。

<h2 id="feature-pass-through">
  功能傳遞
</h2>

Claude Code 將 `ANTHROPIC_BASE_URL` gateway 視為 Anthropic 格式端點，並向其發送它發送給 `api.anthropic.com` 的測試版標頭和請求體欄位，除了為直接連接保留的一小組診斷和預設值，例如下面涵蓋的細粒度工具串流預設。該集合因版本而異，因此不要依賴其內容。

添加請求體欄位的功能將它們與測試版標頭配對，該對一起傳遞。移除標頭同時傳遞請求體的 gateway，或將 Anthropic 格式請求體轉發到具有不同架構的上游，會產生硬 `400` 錯誤；只有當兩個部分一起不存在時，功能才會安靜地關閉。重寫或編輯請求體以進行內容檢查的 gateway 會以與移除相同的方式破壞配對，因此請在不修改的情況下檢查。該表注意了功能偏離配對的位置。

細粒度工具串流是直接連接預設值之一：每當請求通過自訂基礎 URL 路由時，它預設為關閉，當開發人員設定 [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/zh-TW/env-vars) 時，gateway 會接收它。

| 功能                                                                                                                                                                                                                | 標頭和請求體對                                                                                                        | 破壞時的症狀                                                                                                              | 補救                                                                                     |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------- |
| [自適應推理](/docs/zh-TW/model-config#adjust-effort-level)                                                                                                                                                                  | 無測試版標頭。Claude Code 為 Claude 4.6 及更新版本發送 `thinking: {"type": "adaptive"}`，並將它不識別的模型名稱（如 gateway 別名）視為接收該欄位的目前模型 | 當上游模型組建不接受它時，命名 `thinking` 欄位或 `adaptive` 標籤的 `400`                                                                 | 升級上游。在 Opus 4.6 和 Sonnet 4.6 上，開發人員可以改為設定 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1`    |
| [上下文管理](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                    | 上下文管理測試版標頭與 `context_management` 請求體欄位配對                                                                       | `400` 搭配 `Extra inputs are not permitted`。常見於 gateway 接受 Anthropic 格式請求但將其轉發到 Amazon Bedrock 時                      | 轉發兩者，或 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/zh-TW/env-vars)                   |
| [擴展上下文](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)和[交錯思考](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | 僅測試版標頭，無請求體欄位                                                                                                  | 當標頭被移除時無聲地不可用；上游永遠不會看到功能請求                                                                                          | 逐字轉發 `anthropic-beta`                                                                  |
| 測試版[工具欄位](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                 | 工具相關的測試版標頭與工具架構欄位（如 `strict` 和 `defer_loading`）配對                                                              | 當請求體在沒有其標頭的情況下通過時，命名無法識別的工具架構欄位的 `400`                                                                              | 轉發兩者，或 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities) |
| [努力](https://platform.claude.com/docs/en/build-with-claude/effort)和[結構化輸出](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                              | `output_config` 請求體欄位攜帶努力、結構化輸出格式和任務預算設定；每個都與其自己的測試版標頭配對                                                       | 在 Amazon Bedrock 和 Google Cloud 的 Agent Platform 上游上命名 `output_config` 的 `400`，通常是 `Extra inputs are not permitted` | 一起轉發欄位及其標頭                                                                             |
| [提示詞快取](/docs/zh-TW/prompt-caching)                                                                                                                                                                                    | 無測試版配對。Claude Code 將 `cache_control` 標記附加到 `system` 區塊和 `messages` 項目，包括在對話中途附加的 `role: "system"` 項目           | 無錯誤：對話在每個回合上都計費為未快取的輸入，在 `usage` 中可見為高 `input_tokens` 且很少或沒有快取活動                                                    | 無論在何處出現，都逐字轉發 `cache_control`，並且不要將區塊形式的 `system` 或訊息內容轉換為純字串                          |
| [令牌計數](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                      | 無測試版配對；使用 `count_tokens` 端點                                                                                    | 無錯誤：Claude Code 回退到基於字元的估計，因此 `/context` 顯示近似計數                                                                     | 公開端點以取得精確令牌計數                                                                          |

`ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [變數](/docs/zh-TW/model-config)僅在提供者配置中聲明模型功能：`CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_VERTEX`、`CLAUDE_CODE_USE_FOUNDRY` 和 [`CLAUDE_CODE_USE_MANTLE`](/docs/zh-TW/amazon-bedrock#use-the-mantle-endpoint)。它們在 `ANTHROPIC_BASE_URL` gateway 後面沒有效果。

<h3 id="automatic-retry-and-error-forwarding">
  自動重試和錯誤轉發
</h3>

Claude Code 在上游拒絕後的行為取決於被拒絕的內容：

* 當上游拒絕 `thinking` 欄位、中途對話系統訊息或這類訊息上的 `cache_control` 標記時，Claude Code 會重試請求並為對話的其餘部分禁用被拒絕的功能
* 當上游拒絕[思考簽名](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)時，包括以 `400` 拒絕其中區塊 `bound to a different conversation` 時，Claude Code 會從請求中移除較早的思考區塊、重試，並將它們排除在每個後續請求之外。新回應仍包含思考
* 當 gateway 或其上游將[顧問工具](/docs/zh-TW/advisor)項目在 `tools` 中拒絕為無法識別的工具類型時，Claude Code 會重試一次請求，不包含該項目及其 `anthropic-beta` 值。對該基礎 URL 的後續請求會將顧問排除在外，直到 Claude Code 退出，且 `/advisor` 對開發人員在該時間內不可用。Claude Code 通過 `400` 或 `422` 回應識別此拒絕，其訊息在 `Input tag` 後命名工具類型，例如 `Input tag 'advisor_20260301'`。在 v2.1.280 之前，Claude Code 沒有重試此拒絕
* Claude Code 不會重試上下文管理或工具架構欄位的拒絕，因此這些 `400` 錯誤會到達開發人員

`bound to a different conversation` 拒絕來自 API 的[保留思考](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking)檢查，當 `system`、`tools` 或較早的 `messages` 內容與產生思考的請求不同時，該檢查會失敗。重寫任何該內容的 gateway 可能會導致拒絕本身；[程式庫、代理和 gateway](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways)涵蓋要逐字轉發的內容。

重試邏輯與上游的錯誤措辭相匹配，因此不修改地轉發錯誤回應體。在自己的信封中包裝上游錯誤的 gateway 會破壞恢復路徑，即使它保留了狀態碼，除非信封的訊息攜帶穩定的 `capability_rejected:` 令牌。[Claude 應用程式 gateway 為雲端提供者的錯誤措辭替換這些令牌](/docs/zh-TW/claude-apps-gateway-config#upstream-error-messages)，例如 `capability_rejected: prompt_too_long`。

<h3 id="disable-pre-release-capabilities">
  禁用預發佈功能
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` 停止 Claude Code 在每個提供者上發送預發佈功能及其請求體欄位，包括上下文管理和測試版工具欄位。該變數不影響自適應推理，後者由模型而不是測試版選擇。它永遠不會抑制訂閱驗證所需的 OAuth 功能。

在 Claude Code v2.1.227 或更新版本上，您的組織可以通過[受管設定](/docs/zh-TW/managed-settings)在此變數下保持 [MCP 工具搜尋](/docs/zh-TW/mcp#scale-with-mcp-tool-search)開啟。Claude Code 在該覆蓋就位時發送的內容取決於您如何連接：

* 在直接連接上，或通過設定了 `ANTHROPIC_BASE_URL` 的 gateway，Claude Code 繼續發送工具搜尋測試版標頭、`defer_loading` 工具欄位和 `tool_reference` 區塊，並移除其餘部分
* 在雲端提供者上，或通過 [Claude 應用程式 gateway](/docs/zh-TW/claude-apps-gateway) 登入，覆蓋沒有效果

Claude Code 發送的功能集在版本中增長。有關目前的測試版標頭字串，請參閱[測試版標頭參考](https://platform.claude.com/docs/en/api/beta-headers)；針對新的 Claude Code 版本測試您的 gateway，而不是固定到觀察清單。

<h2 id="model-discovery">
  模型發現
</h2>

當 `ANTHROPIC_BASE_URL` 指向公開 Anthropic Messages 格式的 gateway 時，Claude Code 可以在啟動時查詢 gateway 的 `/v1/models` 端點，並將返回的模型添加到 `/model` 選擇器。如果您或您的管理員在 [`modelPicker`](/docs/zh-TW/settings-reference#modelpicker) 陣容中設定 `replaceBuiltInOptions`，Claude Code 會從選擇器中隱藏發現的模型。

開發人員通過在自己的環境中或通過受管設定設定 [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/zh-TW/env-vars) 來啟用它。預設情況下發現是關閉的，以便由共享 API 金鑰支持的 gateway 不會向每個使用者公開金鑰可以存取的每個模型。

<h3 id="when-discovery-runs">
  發現何時運行
</h3>

發現僅適用於 Anthropic Messages 格式。當以下情況時不運行：

* 設定了任何 `CLAUDE_CODE_USE_*` 提供者變數，即使也設定了 `ANTHROPIC_BASE_URL`
* `ANTHROPIC_BASE_URL` 未設定或指向 `api.anthropic.com`

當[非必要流量被關閉](/docs/zh-TW/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path)時，發現仍會運行，因為請求只會進入您的 gateway。在 v2.1.257 之前，當非必要流量被關閉時，發現不會運行。

<h3 id="request-and-response">
  請求和回應
</h3>

請求是 `GET /v1/models?limit=1000`，超時時間為 3 秒，任何重定向都被視為失敗，因此認證不會洩露給重定向目標。回應緩慢或重定向 `/v1/models` 的 gateway，即使是 `http` 到 `https`，也會無聲地失敗發現；在配置的基礎 URL 處直接提供端點。

若要給緩慢的 gateway 更長的時間，請設定 [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/zh-TW/env-vars#variables)。該變數需要 Claude Code v2.1.269 或更新版本。

Claude Code 使用以下兩個認證標頭發送發現請求，並省略其值無法解析的標頭。發送兩個標頭需要 Claude Code v2.1.248 或更新版本。較早的版本在設定 `ANTHROPIC_AUTH_TOKEN` 時僅發送 `Authorization`，否則僅發送 `x-api-key`。

* `Authorization`：`ANTHROPIC_AUTH_TOKEN` 作為持有人令牌，否則 [`apiKeyHelper`](/docs/zh-TW/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 值作為持有人令牌。在這種情況下，Claude Code 會等待幫助程式返回後再發送請求。
* `x-api-key`：Claude Code 解析的 API 金鑰，例如 `ANTHROPIC_API_KEY`。當幫助程式值是唯一的認證時，此標頭也會攜帶它，因此該值會在兩個標頭中到達。

Claude Code 也會發送來自 `ANTHROPIC_CUSTOM_HEADERS` 的任何標頭。當自訂標頭具有非空值時，Claude Code 會發送它來代替同名的內建標頭，不區分大小寫地匹配名稱。

當兩個認證標頭的值都無法解析時，Claude Code 會跳過發現，並在 `claude --debug` 工作階段的偵錯日誌中寫入 `[gatewayDiscovery] skipped` 行。如果您僅通過 `ANTHROPIC_CUSTOM_HEADERS` 提供認證，Claude Code 仍會跳過發現。

Claude Code 從回應的 `data` 陣列中的每個條目讀取 `id`、可選的 `display_name` 和可選的 `description`：

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code 在其 `id` 中的任何位置包含 `claude` 或 `anthropic` 時保留條目，不區分大小寫，並忽略其餘的。提供者前綴的 ID（例如 `vertex_ai/claude-sonnet-4-6` 或 `bedrock/anthropic.claude-sonnet-4-5`）通過篩選器；不包含任何一個子字符串的 ID 則不通過。在 v2.1.223 之前，Claude Code 僅在其 `id` 以 `claude` 或 `anthropic` 開頭時保留條目，這隱藏了提供者前綴的 ID。

<h3 id="picker-entries-and-caching">
  選擇器條目和快取
</h3>

選擇器是當開發人員在 Claude Code 中運行 `/model` 時打開的互動式模型清單。每個發現的條目在 gateway 發送與 `id` 不同的條目時使用 `display_name` 作為其名稱。否則，當 Claude Code [識別 `id`](/docs/zh-TW/model-config#customize-pinned-model-display-and-capabilities) 時，條目會顯示模型的名稱，當它不識別時顯示 `id`。例如，具有 `id` `my-gateway-claude-sonnet-4-6` 且沒有 `display_name` 的條目顯示為 `Sonnet 4.6`。

發現僅添加 [`availableModels` 受管設定](/docs/zh-TW/settings-reference#availablemodels) 允許的模型。

每個條目也會顯示模型的 `description`，折疊為一行。沒有 `description` 的條目改為讀取「來自 gateway」。在 v2.1.257 之前，每個發現的條目都讀取「來自 gateway」。

當發現的 ID 與選擇器中已有的列匹配時，它不會獲得自己的列：

* 相同 ID：發現的 ID 完全匹配現有列的 ID，或兩個 ID 是同一 [Fable](/docs/zh-TW/model-config#work-with-fable) 版本的拼寫。
* 與內建別名相同的模型：當發現的明確 ID 命名內建別名目前解析到的模型時，選擇器僅顯示別名列。例如，當 `sonnet` 解析為 `claude-sonnet-5` 時，發現的 `claude-sonnet-5` 會折疊到 `sonnet` 列中，而發現的 `claude-sonnet-4-6` 仍會獲得自己的列。在 v2.1.197 之前，Claude Code 沒有將這些 ID 折疊到內建列中，因此 `claude-sonnet-5` 也會獲得自己的「來自 gateway」列。

結果被快取到 `~/.claude/cache/gateway-models.json`，或在 Windows 上 `%USERPROFILE%\.claude\cache\gateway-models.json`，並在每次啟動時刷新。如果您設定 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars)，快取會改為位於該目錄下。如果請求失敗或 gateway 未實現 `/v1/models`，選擇器會回退到上次啟動的快取清單或內建模型清單。如果您的 gateway 在不匹配發現篩選器的別名下提供 Claude 模型，開發人員可以使用[模型配置](/docs/zh-TW/model-config)變數手動添加這些別名。

<h2 id="related-resources">
  相關資源
</h2>

有關 gateway 文件集的其餘部分和基礎 API 參考：

* [Gateway 概述](/docs/zh-TW/gateways)：什麼是 gateway 以及如何在 Claude 應用程式 gateway 和其他產品之間進行選擇
* [其他 LLM gateway](/docs/zh-TW/llm-gateway)：如何推出您的組織執行的 gateway 以及它如何與 claude.ai 訂閱互動
* [為您的組織推出 LLM gateway](/docs/zh-TW/llm-gateway-rollout)：使用此指南的管理員檢查清單
* [將 Claude Code 連接到 LLM gateway](/docs/zh-TW/llm-gateway-connect)：每個開發人員的配置和故障排除表
* [測試版標頭參考](https://platform.claude.com/docs/en/api/beta-headers)：目前的 `anthropic-beta` 值集合
* [Messages API](https://platform.claude.com/docs/en/api/messages)：Anthropic 格式 gateway 實現的 API 格式
