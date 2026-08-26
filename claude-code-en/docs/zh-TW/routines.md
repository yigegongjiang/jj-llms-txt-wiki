> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用例行程序自動化工作

> 讓 Claude Code 自動運行。定義在排程上運行、在 API 呼叫時觸發或對來自 Anthropic 管理的雲端基礎設施的 GitHub 事件做出反應的例行程序。

<Note>
  例行程序處於研究預覽階段。行為、限制和 API 表面可能會變更。
</Note>

例行程序是一個已保存的 Claude Code 配置：一個提示、一個或多個存儲庫，以及一組 [connectors](/docs/zh-TW/mcp)，打包一次並自動運行。例行程序在 Anthropic 管理的雲端基礎設施上執行，因此當您的筆記本電腦關閉時它們仍會繼續運行。

每個例行程序可以附加一個或多個觸發器：

* **排程**：按照每小時、每晚或每週等定期節奏運行，或在特定的未來時間運行一次
* **API**：通過向每個例行程序端點發送帶有持有人令牌的 HTTP POST 來按需觸發
* **GitHub**：自動回應存儲庫事件，例如拉取請求或發佈

單個例行程序可以組合觸發器。例如，PR 審查例行程序可以每晚運行、從部署腳本觸發，也可以對每個新 PR 做出反應。

例行程序在啟用了 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 的 Pro、Max、Team 和 Enterprise 計劃上可用。在 [claude.ai/code/routines](https://claude.ai/code/routines) 創建和管理它們，或使用 CLI 中的 `/schedule` 命令。

Team 和 Enterprise 管理員可以在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 的例行程序切換中為所有成員禁用例行程序。禁用後，現有例行程序停止運行，成員無法創建新的。

本頁涵蓋創建例行程序、配置每種觸發器類型、管理運行以及使用限制如何應用。

<h2 id="example-use-cases">
  示例用例
</h2>

每個示例將觸發器類型與例行程序適合的工作類型配對：無人值守、可重複且與明確結果相關。

**待辦事項維護。** 排程觸發器每個工作日晚上針對您的問題跟蹤器通過 connector 運行。例行程序讀取自上次運行以來打開的問題、應用標籤、根據引用的代碼區域分配所有者，並將摘要發佈到 Slack，以便團隊以整理好的隊列開始新的一天。

**警報分類。** 您的監控工具在錯誤閾值被超過時調用例行程序的 API 端點，將警報正文作為 `text` 傳遞。例行程序提取堆棧跟蹤、將其與存儲庫中的最近提交相關聯，並打開一個帶有建議修復和返回警報鏈接的草稿拉取請求。值班人員審查 PR 而不是從空白終端開始。

**定製代碼審查。** GitHub 觸發器在 `pull_request.opened` 上運行。例行程序應用您團隊自己的審查檢查清單，為安全性、性能和風格問題留下內聯評論，並添加摘要評論，以便人工審查者可以專注於設計而不是機械檢查。

**部署驗證。** 您的 CD 管道在每次生產部署後調用例行程序的 API 端點。例行程序針對新構建運行煙霧測試、掃描錯誤日誌以查找回歸，並在部署窗口關閉前向發佈頻道發佈是否可以部署。

**文檔漂移。** 排程觸發器每週運行。例行程序掃描自上次運行以來合併的 PR、標記引用已更改 API 的文檔，並針對文檔存儲庫打開更新 PR 供編輯者審查。

**庫移植。** GitHub 觸發器在 `pull_request.closed` 上運行，篩選為一個 SDK 存儲庫中的合併 PR。例行程序將更改移植到另一種語言的並行 SDK，並打開匹配的 PR，使兩個庫保持同步，而無需人工重新實現每個更改。

下面的部分將逐步介紹創建例行程序和配置每種觸發器類型。

<h2 id="create-a-routine">
  創建例行程序
</h2>

從 Web 的 [claude.ai/code/routines](https://claude.ai/code/routines)、Desktop 應用或 CLI 創建例行程序。所有三個界面都寫入同一個雲帳戶，因此您在其中一個創建的例行程序會立即顯示在其他界面中。在 Desktop 應用中，點擊側邊欄中的 **Routines**，然後點擊 **New routine**，並選擇 **Remote**；選擇 **Local** 會創建一個 [Desktop scheduled task](/docs/zh-TW/desktop-scheduled-tasks)，它在您的機器上運行，而不是在雲中運行。

創建表單設置例行程序的提示、存儲庫、環境、connectors 和觸發器。

例行程序作為完整的 Claude Code 雲會話自主運行：沒有權限模式選擇器，運行期間也沒有批准提示。會話可以運行 shell 命令、使用 [skills](/docs/zh-TW/skills) 提交到克隆的存儲庫，並調用您包含的任何 connectors。例行程序可以到達的內容由您選擇的存儲庫及其分支推送設置、[環境的](/docs/zh-TW/claude-code-on-the-web#the-cloud-environment)網絡訪問和變量，以及您包含的 connectors 決定。將每個範圍限制在例行程序實際需要的內容。

例行程序屬於您的個人 claude.ai 帳戶。它們不與隊友共享，並且計入您帳戶的每日運行配額。例行程序通過您連接的 GitHub 身份或 connectors 執行的任何操作都顯示為您：提交和拉取請求帶有您的 GitHub 用戶，Slack 消息、Linear 票證或其他 connector 操作使用您為這些服務鏈接的帳戶。

<h3 id="create-from-the-web">
  從 Web 創建
</h3>

<Steps>
  <Step title="打開創建表單">
    訪問 [claude.ai/code/routines](https://claude.ai/code/routines) 並點擊 **New routine**。
  </Step>

  <Step title="命名例行程序並編寫提示">
    給例行程序一個描述性名稱並編寫 Claude 每次運行的提示。提示是最重要的部分：例行程序自主運行，因此提示必須是自包含的，並明確說明要做什麼以及成功是什麼樣子。

    提示輸入包括一個模型選擇器。Claude 在每次運行時使用選定的模型。
  </Step>

  <Step title="選擇存儲庫">
    添加一個或多個 GitHub 存儲庫供 Claude 在其中工作。每個存儲庫在運行開始時從默認分支克隆。Claude 為其更改創建 `claude/` 前綴的分支。
  </Step>

  <Step title="選擇環境">
    為例行程序選擇一個 [cloud environment](/docs/zh-TW/claude-code-on-the-web#the-cloud-environment)。環境控制雲會話可以訪問的內容：

    * **Network access**：設置每次運行期間可用的互聯網訪問級別
    * **Environment variables**：提供 API 密鑰、令牌或其他 Claude 可以使用的機密
    * **Setup script**：安裝例行程序需要的依賴項和工具。結果是 [cached](/docs/zh-TW/claude-code-on-the-web#environment-caching)，因此腳本不會在每個會話上重新運行

    提供了一個 **Default** 環境，具有 **Trusted** 網絡訪問，允許 [default set](/docs/zh-TW/claude-code-on-the-web#default-allowed-domains) 的包註冊表、雲提供商 API、容器註冊表和常見開發域，但阻止其他所有內容。如果您的例行程序需要到達您自己的服務或該列表之外的域，請在運行前編輯環境的 [network access](/docs/zh-TW/claude-code-on-the-web#network-access)。要使用單獨的環境，請先 [create one](/docs/zh-TW/claude-code-on-the-web#configure-your-environment)。
  </Step>

  <Step title="選擇觸發器">
    在 **Select a trigger** 下，選擇例行程序如何啟動。您可以選擇一種觸發器類型或組合多種。

    <Tabs>
      <Tab title="Schedule">
        選擇預設頻率進行定期運行，或在特定時間戳安排一次性運行。有關時區處理、交錯、自定義 cron 間隔和一次性運行，請參閱 [Add a schedule trigger](#add-a-schedule-trigger)。
      </Tab>

      <Tab title="GitHub event">
        選擇存儲庫、要反應的事件和可選篩選器。有關支持的事件和篩選字段的完整列表，請參閱 [Add a GitHub trigger](#add-a-github-trigger)。
      </Tab>

      <Tab title="API">
        在此選擇 **API**，然後保存例行程序。URL 和令牌在保存例行程序後生成，因為它們取決於例行程序 ID。請參閱 [Add an API trigger](#add-an-api-trigger) 以複製 URL 並生成令牌。
      </Tab>
    </Tabs>
  </Step>

  <Step title="審查 connectors 和權限">
    表單底部的 **Connectors** 和 **Permissions** 選項卡控制例行程序可以到達的內容。

    在 Connectors 下，默認情況下包括您所有連接的 [MCP connectors](/docs/zh-TW/mcp)。移除例行程序不需要的任何。Claude 可以使用包含的 connector 中的每個工具，包括寫入，無需在運行期間請求權限。

    在 Permissions 下，為任何 Claude 應該能夠推送到現有分支而不是僅 `claude/` 前綴分支的存儲庫啟用 **Allow unrestricted branch pushes**。
  </Step>

  <Step title="創建例行程序">
    點擊 **Create**。例行程序出現在列表中，並在下次其觸發器之一匹配時運行。要立即開始運行，請在例行程序的詳細信息頁面上點擊 **Run now**。

    每次運行都會在您的其他會話旁邊創建一個新會話，您可以在其中查看 Claude 做了什麼、審查更改並創建拉取請求。
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  從 CLI 創建
</h3>

在任何會話中運行 `/schedule` 以對話方式創建排程例行程序。您也可以直接傳遞描述，例如定期例行程序如 `/schedule daily PR review at 9am` 或一次性例行程序如 `/schedule clean up feature flag in one week`。Claude 會逐步介紹 Web 表單收集的相同信息，然後將例行程序保存到您的帳戶。

成功開始看起來像一次對話：Claude 在保存前詢問有關計劃、存儲庫和提示的後續問題。如果 Claude 改為回覆您需要進行身份驗證或無法連接到您的遠程 claude.ai 帳戶，則未創建例行程序；請參閱 [Troubleshooting](#troubleshooting)。

CLI 中的 `/schedule` 僅創建排程例行程序。要添加 API 或 GitHub 觸發器，請在 [claude.ai/code/routines](https://claude.ai/code/routines) 的 Web 上編輯例行程序。

CLI 還支持管理現有例行程序。運行 `/schedule list` 查看所有例行程序，`/schedule update` 更改一個，或 `/schedule run` 立即觸發它。

<h2 id="configure-triggers">
  配置觸發器
</h2>

當其觸發器之一匹配時，例行程序啟動。您可以將排程、API 和 GitHub 觸發器的任何組合附加到同一例行程序，並可以隨時從例行程序編輯表單的 **Select a trigger** 部分添加或移除它們。

<h3 id="add-a-schedule-trigger">
  添加排程觸發器
</h3>

排程觸發器按定期節奏運行例行程序，或在特定的未來時間運行一次。在 **Select a trigger** 部分中選擇預設頻率：每小時、每天、工作日或每週。時間以您的本地時區輸入並自動轉換，因此例行程序在該掛鐘時間運行，無論雲端基礎設施位於何處。

運行可能在排程時間後幾分鐘開始，原因是交錯。每個例行程序的偏移是一致的。

對於自定義間隔，例如每兩小時或每月的第一天，在表單中選擇最接近的預設，然後在 CLI 中運行 `/schedule update` 以設置特定的 cron 表達式。最小間隔是一小時；運行頻率更高的表達式會被拒絕。

<h4 id="schedule-a-one-off-run">
  排程一次性運行
</h4>

一次性排程在特定時間戳處觸發例行程序一次。使用它來提醒自己本週稍後、在推出完成後打開清理 PR，或在上游更改到達時啟動後續任務。例行程序觸發後，它會自動禁用，Web UI 將其標記為 **Ran**。要再次運行它，編輯例行程序並設置新的一次性時間。

<Note>
  一次性排程從 CLI 逐步推出，可能在您的帳戶上還不可用。如果 `/schedule` 只提供定期排程，請改為在 [claude.ai/code/routines](https://claude.ai/code/routines) 從 Web 創建一次性運行。
</Note>

通過在 CLI 中用自然語言描述時間來創建一次性運行。Claude 根據當前時間解析該短語，並在保存前確認絕對時間戳。

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

與定期排程相同的本地到 UTC 轉換適用於一次性時間戳。

一次性運行不計入每日例行程序運行上限。它們像任何其他會話一樣消耗您計劃的常規訂閱使用量。有關詳細信息，請參閱 [Usage and limits](#usage-and-limits)。

<h3 id="add-an-api-trigger">
  添加 API 觸發器
</h3>

API 觸發器為例行程序提供專用的 HTTP 端點。使用例行程序的持有人令牌 POST 到端點會啟動新會話並返回會話 URL。使用此功能將 Claude Code 連接到警報系統、部署管道、內部工具或任何可以進行身份驗證 HTTP 請求的地方。

API 觸發器從 Web 添加到現有例行程序。CLI 目前無法創建或撤銷令牌。

<Steps>
  <Step title="打開例行程序進行編輯">
    轉到 [claude.ai/code/routines](https://claude.ai/code/routines)，點擊您想通過 API 觸發的例行程序，然後點擊鉛筆圖標打開 **Edit routine**。
  </Step>

  <Step title="添加 API 觸發器">
    滾動到 **Instructions** 框下方的 **Select a trigger** 部分，點擊 **Add another trigger**，並選擇 **API**。
  </Step>

  <Step title="複製 URL 並生成令牌">
    模態框顯示此例行程序的 URL 以及示例 curl 命令。複製 URL，然後點擊 **Generate token** 並立即複製令牌。令牌只顯示一次，之後無法檢索，因此請將其存儲在安全的地方，例如您的警報工具的機密存儲。
  </Step>

  <Step title="調用端點">
    POST 到 URL 時在 `Authorization: Bearer` 標頭中發送令牌。下面的 [Trigger a routine](#trigger-a-routine) 部分顯示完整示例。
  </Step>
</Steps>

每個例行程序都有自己的令牌，範圍限於僅觸發該例行程序。要輪換或撤銷它，返回同一模態框並點擊 **Regenerate** 或 **Revoke**。

<h4 id="trigger-a-routine">
  觸發例行程序
</h4>

向 `/fire` 端點發送 POST 請求，在 `Authorization` 標頭中包含持有人令牌。請求正文接受可選的 `text` 字段，用於運行特定的上下文，例如警報正文或失敗的日誌，與其保存的提示一起傳遞給例行程序。該值是自由格式文本，不被解析：如果您發送 JSON 或其他結構化有效負載，例行程序會將其作為文字字符串接收。

下面的示例從 shell 觸發例行程序。所示的例行程序 ID 和令牌是佔位符：將它們替換為您在 [添加 API 觸發器](#add-an-api-trigger) 時複製的 URL 和令牌，否則請求會失敗並出現 `401` 身份驗證錯誤：

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

成功的請求返回一個 JSON 正文，包含新的會話 ID 和 URL：

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

在瀏覽器中打開會話 URL 以實時觀看運行、審查更改或手動繼續對話。

<Warning>
  `/fire` 端點在 `experimental-cc-routine-2026-04-01` beta 標頭下發佈。請求和響應形狀、速率限制和令牌語義可能在功能處於研究預覽階段時變更。破壞性更改在新的日期 beta 標頭版本後發佈，最近的兩個先前標頭版本繼續工作，以便調用者有時間遷移。
</Warning>

<h4 id="api-reference">
  API 參考
</h4>

有關完整的 API 參考，包括所有錯誤響應、驗證規則和字段限制，請參閱 Claude Platform 文檔中的 [Trigger a routine via API](https://platform.claude.com/docs/zh-TW/api/claude-code/routines-fire)。

`/fire` 端點僅對 claude.ai 用戶可用，不是 Claude Platform API 表面的一部分。

<h3 id="add-a-github-trigger">
  添加 GitHub 觸發器
</h3>

GitHub 觸發器在連接的存儲庫上發生匹配事件時自動啟動新會話。每個匹配事件啟動自己的會話。

<Note>
  在研究預覽期間，GitHub webhook 事件受每個例行程序和每個帳戶的每小時上限限制。超過限制的事件會被丟棄，直到窗口重置。在 [claude.ai/code/routines](https://claude.ai/code/routines) 查看您當前的限制。
</Note>

GitHub 觸發器僅從 Web UI 配置。

<Steps>
  <Step title="打開例行程序進行編輯">
    轉到 [claude.ai/code/routines](https://claude.ai/code/routines)，點擊例行程序，然後點擊鉛筆圖標打開 **Edit routine**。
  </Step>

  <Step title="添加 GitHub 事件觸發器">
    滾動到 **Select a trigger** 部分，點擊 **Add another trigger**，並選擇 **GitHub event**。
  </Step>

  <Step title="安裝 Claude GitHub App">
    Claude GitHub App 必須安裝在您想訂閱的存儲庫上。如果尚未安裝，觸發器設置會提示您安裝它。

    <Note>
      在 CLI 中運行 `/web-setup` 授予存儲庫訪問權限以進行克隆，但它不安裝 Claude GitHub App，也不啟用 webhook 傳遞。GitHub 觸發器需要安裝 Claude GitHub App，觸發器設置會提示您執行此操作。
    </Note>
  </Step>

  <Step title="配置觸發器">
    選擇存儲庫，從 [supported events](#supported-events) 列表中選擇事件，並可選地添加篩選器。保存觸發器。
  </Step>
</Steps>

<h4 id="supported-events">
  支持的事件
</h4>

GitHub 觸發器可以訂閱以下事件類別之一。在每個類別中，您可以選擇特定操作，例如 `pull_request.opened`，或對類別中的所有操作做出反應。

| 事件           | 觸發時機                       |
| :----------- | :------------------------- |
| Pull request | PR 被打開、關閉、分配、標記、同步或以其他方式更新 |
| Release      | 發佈被創建、發佈、編輯或刪除             |

<h4 id="filter-pull-requests">
  篩選拉取請求
</h4>

使用篩選器縮小哪些拉取請求啟動新會話。所有篩選條件必須匹配才能觸發例行程序。可用的篩選字段是：

| 篩選器         | 匹配                |
| :---------- | :---------------- |
| Author      | PR 作者的 GitHub 用戶名 |
| Title       | PR 標題文本           |
| Body        | PR 描述文本           |
| Base branch | PR 目標的分支          |
| Head branch | PR 來自的分支          |
| Labels      | 應用於 PR 的標籤        |
| Is draft    | PR 是否處於草稿狀態       |
| Is merged   | PR 是否已合併          |

每個篩選器將字段與運算符配對：等於、包含、開始於、是其中之一、不是其中之一或匹配正則表達式。

`matches regex` 運算符測試整個字段值，而不是其中的子字符串。要匹配任何包含 `hotfix` 的標題，請寫 `.*hotfix.*`。沒有周圍的 `.*`，篩選器僅匹配完全是 `hotfix` 的標題，前後沒有任何內容。對於不使用正則表達式語法的文字子字符串匹配，請改用 `contains` 運算符。

一些示例篩選器組合：

* **Auth module review**：base branch `main`，head branch 包含 `auth-provider`。將任何涉及身份驗證的 PR 發送給專注的審查者。
* **Ready-for-review only**：is draft 是 `false`。跳過草稿，以便例行程序僅在 PR 準備好審查時運行。
* **Label-gated backport**：labels 包括 `needs-backport`。僅當維護者標記 PR 時才觸發移植到另一分支的例行程序。

<h4 id="how-sessions-map-to-events">
  會話如何映射到事件
</h4>

每個匹配的 GitHub 事件啟動新會話。GitHub 觸發的例行程序不提供跨事件的會話重用，因此兩個 PR 更新會產生兩個獨立會話。

<h2 id="manage-routines">
  管理例行程序
</h2>

點擊列表中的例行程序以打開其詳細信息頁面。詳細信息頁面顯示例行程序的存儲庫、connectors、提示、排程、API 令牌、GitHub 觸發器和過去運行的列表。

<h3 id="view-and-interact-with-runs">
  查看和交互運行
</h3>

點擊任何運行以將其作為完整會話打開。從那裡您可以看到 Claude 做了什麼、審查更改、創建拉取請求或繼續對話。每個運行會話的工作方式與任何其他會話相同：使用會話標題旁邊的下拉菜單重命名、存檔或刪除它。

<Note>
  運行列表中的綠色狀態表示會話已啟動並退出，沒有基礎設施錯誤。這並不意味著您提示中的任務成功。打開運行以讀取記錄並確認 Claude 實際上做了什麼。被阻止的網絡請求、缺失的 connector 工具和任務級別的失敗都會在那裡顯示，而不是在狀態指示器中。
</Note>

<h3 id="edit-and-control-routines">
  編輯和控制例行程序
</h3>

從例行程序詳細信息頁面，您可以：

* 點擊 **Run now** 立即開始運行，無需等待下一個排程時間。
* 使用 **Repeats** 部分中的切換暫停或恢復排程。暫停的例行程序保留其配置但不運行，直到您重新啟用它們。
* 點擊鉛筆圖標打開 **Edit routine** 並更改名稱、提示、存儲庫、環境、connectors 或例行程序的任何觸發器。**Select a trigger** 部分是您添加或移除排程、API 令牌和 GitHub 事件觸發器的地方。
* 點擊刪除圖標移除例行程序。由例行程序創建的過去會話保留在您的會話列表中。

<h3 id="repositories-and-branch-permissions">
  存儲庫和分支權限
</h3>

例行程序需要 GitHub 訪問權限來克隆存儲庫。當您使用 `/schedule` 從 CLI 創建例行程序時，Claude 檢查您的帳戶是否連接了 GitHub，如果沒有則提示您運行 `/web-setup`。有關授予訪問權限的兩種方式，請參閱 [GitHub authentication options](/docs/zh-TW/claude-code-on-the-web#github-authentication-options)。

您添加的每個存儲庫在每次運行時都會被克隆。Claude 從存儲庫的默認分支開始，除非您的提示另有指定。

默認情況下，Claude 只能推送到以 `claude/` 為前綴的分支。這可以防止例行程序意外修改受保護或長期分支。要移除特定存儲庫的此限制，請在創建或編輯例行程序時為該存儲庫啟用 **Allow unrestricted branch pushes**。

<h3 id="connectors">
  Connectors
</h3>

例行程序可以使用您連接的 MCP connectors 在每次運行期間讀取和寫入外部服務。例如，分類支持請求的例行程序可能從 Slack 頻道讀取並在 Linear 中創建問題。

Connectors 是您帳戶上的 [claude.ai integrations](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)。您在 CLI 中使用 `claude mcp add` 本地添加的 MCP 伺服器存儲在您的機器上而不是您的 claude.ai 帳戶上，因此它們不會出現在 connectors 列表中。要在例行程序中使用其中一個伺服器，請在 [claude.ai/customize/connectors](https://claude.ai/customize/connectors) 添加它作為 connector，或在已提交的 [`.mcp.json`](/docs/zh-TW/mcp#project-scope) 中聲明它，以便它是克隆存儲庫的一部分。

當您創建例行程序時，默認情況下包括您所有當前連接的 connectors。移除任何不需要的以限制 Claude 在運行期間可以訪問的工具。您也可以直接從例行程序表單添加 connectors。

要在例行程序表單外管理或添加 connectors，請訪問 claude.ai 上的 **Settings > Connectors** 或在 CLI 中使用 `/schedule update`。

<h3 id="environments-and-network-access">
  環境和網絡訪問
</h3>

每個例行程序在 [cloud environment](/docs/zh-TW/claude-code-on-the-web#the-cloud-environment) 中運行，該環境控制網絡訪問、環境變量和設置腳本。例行程序在每次運行時繼承環境的網絡策略。

**Default** 環境使用 **Trusted** 網絡訪問：[默認允許列表](/docs/zh-TW/claude-code-on-the-web#default-allowed-domains)中的包註冊表、雲提供商 API、容器註冊表和常見開發域是可達的，但任意域不可達。對其他主機的出站請求失敗，返回 `403` 和 `x-deny-reason: host_not_allowed`。MCP connector 流量通過 Anthropic 的伺服器路由，因此您添加到例行程序的 connectors 無需將其主機添加到 **Allowed domains** 即可工作。移除您不需要的任何 connectors，詳見 [Connectors](#connectors)。

要允許其他域：

<Steps>
  <Step title="打開例行程序進行編輯">
    在例行程序的詳細信息頁面上，點擊鉛筆圖標打開 **Edit routine**。
  </Step>

  <Step title="打開環境選擇器">
    在 **Instructions** 框下方，選擇顯示您環境名稱的雲圖標，例如 **Default**。
  </Step>

  <Step title="打開環境設置">
    將滑鼠懸停在列表中的環境上，然後點擊右側出現的設置圖標。
  </Step>

  <Step title="更改網絡訪問級別">
    在 **Update cloud environment** 對話框中，將 **Network access** 更改為 **Custom** 並在 **Allowed domains** 中輸入您的域。檢查 **Also include default list of common package managers** 以在自定義域旁邊保留 [默認允許列表](/docs/zh-TW/claude-code-on-the-web#default-allowed-domains)。選擇 **Full** 以獲得不受限制的訪問。
  </Step>

  <Step title="保存">
    點擊 **Save changes**。新策略從下一次運行開始應用。
  </Step>
</Steps>

有關訪問級別和默認允許列表的詳細信息，請參閱 [Network access](/docs/zh-TW/claude-code-on-the-web#network-access)。

<h2 id="usage-and-limits">
  使用和限制
</h2>

例行程序以與互動式會話相同的方式消耗訂閱使用量。除了標準訂閱限制外，例行程序還有每個帳戶每天可以啟動多少次運行的每日上限。在 [claude.ai/code/routines](https://claude.ai/code/routines) 或 [claude.ai/settings/usage](https://claude.ai/settings/usage) 查看您目前的消耗和剩餘的每日例行程序運行。

當例行程序達到每日上限或您的訂閱使用限制時，啟用了使用額度的組織可以繼續在計量超額上運行例行程序。沒有使用額度，額外運行會被拒絕，直到時間窗口重置。從 claude.ai 上的 **Settings > Billing** 啟用使用額度。

一次性運行不計入每日例行程序運行上限。它們像任何其他會話一樣消耗您的常規訂閱使用量，但它們不受每個帳戶每日例行程序運行額度的限制。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="/schedule-returns-unknown-command">
  `/schedule` 返回「Unknown command」
</h3>

當不滿足其中一項要求時，CLI 會隱藏 `/schedule`：命令菜單在您輸入時會顯示 `No commands match "/schedule"`，提交時會返回 `Unknown command: /schedule`。原因通常是以下之一：

* 您使用 Console API 金鑰或雲端提供商（例如 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry）進行身份驗證。`/schedule` 需要 claude.ai 訂閱登入。如果在您的 shell 中設定了 `ANTHROPIC_API_KEY` 或 `ANTHROPIC_AUTH_TOKEN`，或在 `settings.json` 中設定了 `apiKeyHelper`，請先移除它，因為這些設定優先於 claude.ai 登入
* `DISABLE_TELEMETRY`、`DO_NOT_TRACK`、`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 或 `DISABLE_GROWTHBOOK` 在您的 shell 環境或 [`settings.json` 檔案](/docs/zh-TW/settings#available-settings)的 `env` 區塊中設定。這些會停用功能旗標擷取，而 `/schedule` 依賴於此功能
* 您在 Claude Code 網頁工作階段中。改為從 [web UI](https://claude.ai/code/routines) 管理例行程序

無論 CLI 如何配置，您始終可以在 [claude.ai/code/routines](https://claude.ai/code/routines) 建立和管理例行程序。

<h3 id="/schedule-asks-you-to-authenticate">
  `/schedule` 要求您進行身份驗證
</h3>

如果 `/schedule` 執行但 Claude 回應您需要先使用 claude.ai 帳戶進行身份驗證，則 CLI 沒有儲存的 claude.ai 登入。API 帳戶不支援例行程序。執行 `/login`，使用您的 claude.ai 帳戶登入，然後再次執行 `/schedule`。

<h3 id="routines-are-disabled-by-your-organization’s-policy">
  "例行程序已被您的組織政策禁用"
</h3>

您的 Team 或 Enterprise 組織中的擁有者可能已在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 關閉了 **Routines** 切換。這是一個伺服器端組織設定，因此無法從您的本地配置中覆蓋。請聯繫擁有者為您的組織啟用例行程序。

<h2 id="related-resources">
  相關資源
</h2>

* [`/loop` 和會話內排程](/docs/zh-TW/scheduled-tasks)：在打開的 CLI 會話中排程本地任務
* [Desktop scheduled tasks](/docs/zh-TW/desktop-scheduled-tasks)：在您的機器上運行的本地排程任務，可以訪問本地文件
* [Cloud environment](/docs/zh-TW/claude-code-on-the-web#the-cloud-environment)：為雲會話配置運行時環境
* [MCP connectors](/docs/zh-TW/mcp)：連接外部服務，如 Slack、Linear 和 Google Drive
* [GitHub Actions](/docs/zh-TW/github-actions)：在存儲庫事件上在您的 CI 管道中運行 Claude
