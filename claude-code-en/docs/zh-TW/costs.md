> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 有效管理成本

> 追蹤 token 使用情況、設定團隊支出限制，並透過上下文管理、模型選擇、延伸思考設定和預處理 hooks 來降低 Claude Code 成本。

Claude Code 按 API token 消耗量計費。如需訂閱計畫定價（Pro、Max、Team、Enterprise），請參閱 [claude.com/pricing](https://claude.com/pricing)。每位開發人員的成本差異很大，取決於模型選擇、程式碼庫大小和使用模式，例如執行多個執行個體或自動化。

在企業部署中，平均成本約為每位開發人員每個活躍日 $13，每位開發人員每月 $150-250，90% 的使用者成本保持在每個活躍日 \$30 以下。若要估計您自己團隊的支出，請從小型試點群組開始，並使用下面的追蹤工具建立基準，然後再進行更廣泛的推出。

本頁涵蓋如何[追蹤您的成本](#track-your-costs)、[管理團隊成本](#manage-costs-for-your-organization)和[減少 token 使用](#reduce-token-usage)。

<h2 id="track-your-costs">
  追蹤您的成本
</h2>

<h3 id="using-the-/usage-command">
  使用 `/usage` 命令
</h3>

<Note>
  `/usage` 中的 Session 區塊顯示 API token 使用情況，適用於 API 使用者。Claude Max 和 Pro 訂閱者的使用情況已包含在其訂閱中，因此工作階段成本數字與計費無關。訂閱者會在同一畫面上看到計畫使用情況列、活動統計資訊和使用情況明細。
</Note>

`/usage` 頂部的 Session 區塊顯示您目前工作階段的詳細 token 使用統計資訊。美元數字是根據 token 計數在本地計算的估計值，可能與您的實際帳單不同。如需權威計費資訊，請參閱 [Claude Console](https://platform.claude.com/usage) 中的「使用情況」頁面。

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

在 Pro、Max、Team 或 Enterprise 計畫上，`/usage` 還會顯示計入您計畫限制的內容明細。它將最近的使用情況歸因於 skills、subagents、plugins 和個別 MCP servers，每個都顯示為總數的百分比。按 `d` 或 `w` 在過去 24 小時和過去 7 天之間切換。這些數字是近似值，根據此機器上的本地工作階段歷史記錄計算，因此不包括來自其他裝置或 claude.ai 的使用情況。

當您的計畫限制請求失敗時（通常是因為使用情況端點受到速率限制），`/usage` 會顯示它在過去 60 分鐘內在此機器上載入的最後使用情況列，以及一個 `Showing last-known usage` 備註，說明該資料是多久前取得的。按 `r` 重試；成功重試會用新資料取代最後已知的列。如果沒有過去 60 分鐘內的快照，`/usage` 會報告使用情況端點受到速率限制，並提供相同的重試快捷方式。在 v2.1.208 之前，在尚未載入使用情況的工作階段中受到速率限制的請求始終會顯示錯誤，沒有任何列。

在 [VS Code 擴充功能](/docs/zh-TW/vs-code#check-account-and-usage) 中，相同的明細會出現在「帳戶與使用情況」對話框中，並提供「日」和「週」切換。需要 Claude Code v2.1.174 或更新版本。

<h3 id="set-a-spend-limit-on-pro-and-max">
  在 Pro 和 Max 上設定支出限制
</h3>

在 Pro 和 Max 計畫上，`/usage-credits` 命令會在 CLI 中開啟一個對話框，您可以在其中管理[使用額度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)。從對話框中，您可以：

* 為您的帳戶開啟使用額度
* 購買更多使用額度，可以是列出的套組或自訂金額
* 設定、變更或移除您的每月支出限制
* 配置自動重新載入，當您的餘額低於您設定的閾值時，會自動購買更多使用額度

在 Claude Code v2.1.207 之前的版本上，以及在 CLI 內對話框不可用的帳戶上，`/usage-credits` 會在您的瀏覽器中開啟使用額度計費頁面。在 Team 和 Enterprise 計畫上，具有計費存取權限的成員會獲得相同的瀏覽器頁面，而沒有計費存取權限的成員會從 CLI 傳送請求，要求其管理員開啟使用額度或提高限制。

變更每月支出限制需要帳戶的計費存取權限。如果您在仍有可用使用額度的情況下達到該限制，Claude Code 會提示您提高或移除該限制，以便您可以繼續使用而不離開 CLI。

您輸入到對話框中的金額，例如自訂購買金額、每月支出限制或自動重新載入閾值和目標，必須是數字，可選地後跟句號和一或兩位小數位數，例如 `20` 或 `20.50`。任何其他輸入（包括逗號）都會顯示內嵌錯誤，且不會被儲存。v2.1.207 之前的版本不會顯示對話框，而是開啟計費頁面。

Claude Code 會要求您輸入 `yes` 來確認每次購買和每次自動重新載入變更，無論金額多少，購買確認會顯示您正在批准的稅後總額。變更每月支出限制只在超過 \$1,000 或超過 1,000 個非美元計費貨幣單位時才要求相同的輸入確認。在 v2.1.208 之前，購買和自動重新載入變更也使用該閾值，因此較小的金額會通過標準對話框流程進行，而無需額外的輸入 `yes` 步驟。

金額欄位會開啟並預先填入建議值，您輸入的第一位數字會取代建議值，而不是附加到它。開啟使用額度的畫面會以「取消」被選中的狀態開啟，因此開啟它們需要刻意選擇，而不是誤按 Enter。兩者都需要 Claude Code v2.1.208 或更新版本。

<h2 id="manage-costs-for-your-organization">
  管理組織的成本
</h2>

您對 Claude Code 的控制方式取決於您的組織如何存取 Claude Code：透過 Claude for Teams 或 Enterprise 方案、Claude Console 或雲端提供者。在 Teams 和 Enterprise 方案上，使用量會從每位成員的座位額度中扣除。在 Console 和雲端提供者上，使用量按 token 計費至您的組織。如果您的組織混合使用登入方法，每位開發人員會根據他們驗證的方法進行計量。

下表將每種設定對應到您查看支出的位置、您限制支出的位置，以及您如何提取每位使用者的數字。

| 您的設定                                                                                 | 查看支出                                                                                                             | 限制支出        | 每位使用者報告                                                                                                                                                                                                          |
| :----------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams 或 Enterprise](#claude-for-teams-and-enterprise)                    | [組織分析中的支出報告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | 管理員設定中的支出限制 | [支出報告 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)；Enterprise 上的 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) |
| [Claude Console (API)](#claude-console)                                              | [Console 使用情況頁面](https://platform.claude.com/usage)                                                              | 工作區支出限制     | [Console 儀表板](https://platform.claude.com/claude-code)、[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                              |
| [Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry](#cloud-providers) | 您的雲端計費主控台                                                                                                        | 您的雲端預算控制    | [OpenTelemetry](/docs/zh-TW/monitoring-usage) 或 [LLM gateway](/docs/zh-TW/llm-gateway)                                                                                                                                     |

[OpenTelemetry 匯出](/docs/zh-TW/monitoring-usage)適用於每種設定，是唯一能以近乎即時的方式將每位使用者 token 和成本指標串流到您自己的可觀測性堆疊的選項。

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams 和 Enterprise
</h3>

在 Claude for Teams 和 Enterprise 方案上，每位成員的 Claude Code 使用量會從每座位額度中扣除，該額度在滾動五小時視窗和每週視窗上重設。該額度與 Claude chat 和 Cowork 共享，其大小取決於成員的[座位層級](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)（Standard 或 Premium）。您的控制項位於 claude.ai 管理員主控台中，而不是 Claude Console。

* **查看支出**：[組織分析中的支出報告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)顯示每位使用者和每個模型的估計支出，具有 CSV 匯出功能，每日更新。該報告涵蓋使用額度支出，並在啟用使用額度後出現。座位額度內的使用量不以美元計量。
* **查看採用情況**：[分析儀表板](https://claude.ai/analytics/claude-code)顯示每日活躍使用者、工作階段和貢獻指標，具有貢獻資料的 CSV 匯出。請參閱[使用分析追蹤團隊使用情況](/docs/zh-TW/analytics)。
* **限制支出**：座位額度是預設上限。若要讓成員超過該額度繼續使用，請啟用[使用額度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)，並在組織、群組或個別成員層級設定支出限制。
* **提取每位使用者的數字**：在 Enterprise 方案上，[Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) 會傳回跨 Claude 表面（包括 Claude Code）的每位使用者使用量和成本報告。主要擁有者在 [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys) 建立具有 `read:analytics` 範圍的金鑰。在 Teams 方案上，匯出[支出報告 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)，其中列出每位使用者和每個模型的 token 使用量和估計支出。

[Claude Enterprise 消費指南](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide)是管理員的規劃參考。它說明消費在 Claude chat、Claude Code 和 Cowork 之間的差異，並提供每位使用者的美元起點以供預算編制。為編碼座位預算比聊天座位更多：每個 Claude Code 回合都包含檔案內容、工具呼叫和多步驟推理，因此一個除錯工作階段可能會消耗超過一天的聊天。

<h3 id="claude-console">
  Claude Console
</h3>

API 組織透過[工作區](https://platform.claude.com/docs/en/build-with-claude/workspaces)管理 Claude Code 支出。您可以[設定工作區支出限制](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits)以限制 Claude Code 總支出，並在 Console 中[檢視成本和使用情況報告](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking)。

<Note>
  當您首次使用 Claude Console 帳戶驗證 Claude Code 時，系統會自動為您建立一個名為「Claude Code」的工作區。此工作區為您的組織中所有 Claude Code 使用情況提供集中式成本追蹤和管理。您無法為此工作區建立 API 金鑰；它專門用於 Claude Code 驗證和使用。

  對於具有自訂速率限制的組織，此工作區中的 Claude Code 流量計入您的組織整體 API 速率限制。您可以在 Claude Console 的此工作區的「限制」頁面上設定[工作區速率限制](https://platform.claude.com/docs/zh-TW/api/rate-limits#setting-lower-limits-for-workspaces)，以限制 Claude Code 的份額並保護其他生產工作負載。
</Note>

對於每位使用者報告，[Console 儀表板](https://platform.claude.com/claude-code)顯示每位成員的支出和接受的行數，[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) 使用 [Admin API 金鑰](https://platform.claude.com/settings/admin-keys)以程式設計方式傳回相同的每日每位使用者指標。請參閱 [API 客戶的分析](/docs/zh-TW/analytics#access-analytics-for-api-customers)。

<h4 id="rate-limit-recommendations">
  速率限制建議
</h4>

為團隊設定 Claude Code 時，請根據您的組織規模考慮這些每位使用者的 Token Per Minute (TPM) 和 Request Per Minute (RPM) 建議：

| 團隊規模         | 每位使用者 TPM | 每位使用者 RPM |
| ------------ | --------- | --------- |
| 1-5 位使用者     | 200k-300k | 5-7       |
| 5-20 位使用者    | 100k-150k | 2.5-3.5   |
| 20-50 位使用者   | 50k-75k   | 1.25-1.75 |
| 50-100 位使用者  | 25k-35k   | 0.62-0.87 |
| 100-500 位使用者 | 15k-20k   | 0.37-0.47 |
| 500+ 位使用者    | 10k-15k   | 0.25-0.35 |

例如，如果您有 200 位使用者，您可能會為每位使用者請求 20k TPM，或總共 400 萬 TPM (200\*20,000 = 400 萬)。

隨著團隊規模增長，每位使用者的 TPM 會減少，因為在較大的組織中，傾向於較少的使用者同時使用 Claude Code。這些速率限制適用於組織層級，而不是每個個別使用者，這意味著當其他人未主動使用該服務時，個別使用者可以暫時消耗超過其計算份額的資源。

<Note>
  如果您預期會出現異常高的並行使用情況（例如與大型群組進行的即時培訓課程），您可能需要更高的每位使用者 TPM 配置。
</Note>

<h3 id="cloud-providers">
  雲端提供者
</h3>

在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，Claude Code 按 token 計費至您的雲端帳戶，支出控制項位於您的雲端提供者的計費主控台中。Claude Code 不會從您的雲端傳送指標回 Anthropic，因此[分析儀表板](/docs/zh-TW/analytics)和 Claude Code Analytics API 不涵蓋此使用情況。

對於每位使用者成本歸因，您有三個選項：

* **OpenTelemetry**：[匯出指標](/docs/zh-TW/monitoring-usage)從每位開發人員的機器到您自己的可觀測性堆疊。無論提供者為何，這都會為您提供每位使用者的 token 計數、成本和工具活動。
* **Claude apps gateway**：自託管的 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 提供每位使用者的使用情況歸因、包含 token 計數的 OTLP 指標，以及這些提供者上的[每位使用者支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)。
* **LLM gateway**：透過追蹤每個金鑰支出的代理路由所有 Claude Code 流量。幾家大型企業報告使用 [LiteLLM](/docs/zh-TW/llm-gateway)，一個開源工具，可[按金鑰追蹤支出](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend)。此專案與 Anthropic 無關，尚未進行安全審計。

<h3 id="when-a-developer-asks-about-a-limit">
  當開發人員詢問限制時
</h3>

開發人員通常會向其管理員提出限制問題，因此了解他們達到的上限會很有幫助。三種情況意味著不同的事情：

* **「您已達到工作階段限制」或「您已達到每週限制」**：訂閱方案上基於座位的使用視窗。這些視窗在所有模型中共享，因此使用 `/model` 切換模型不會恢復存取權限，儘管在模型特定的「您已達到 Opus 限制」訊息後它確實讓開發人員繼續工作。該訊息顯示視窗何時重設，開發人員可以執行 `/usage-credits` 以請求超過額度的使用量（如果您已啟用[使用額度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)）。請參閱[使用限制錯誤](/docs/zh-TW/errors#youve-hit-your-session-limit)。
* **上下文或自動壓縮警告**：不是使用限制。對話已接近模型的最大輸入大小，Claude Code 會總結較舊的歷史記錄以釋放空間。將開發人員指向[減少 token 使用量](#reduce-token-usage)。
* **API 或雲端提供者方案上的意外高支出**：通常可追溯到從未清除的長工作階段或留作預設模型的 Opus。分享的最高影響習慣是在不相關的任務之間清除和將模型與工作相匹配，兩者都涵蓋在[減少 token 使用量](#reduce-token-usage)中。

<h3 id="agent-team-token-costs">
  Agent 團隊 token 成本
</h3>

[Agent 團隊](/docs/zh-TW/agent-teams)會產生多個 Claude Code 執行個體，每個都有自己的上下文視窗。Token 使用量會隨著活躍隊友數量和每個隊友執行時間的長短而擴展。

為了保持 agent 團隊成本可控：

* 為隊友使用 Sonnet。它為協調任務平衡了功能和成本。
* 保持團隊規模小。每位隊友執行自己的上下文視窗，因此 token 使用量大致與團隊規模成正比。
* 保持產生提示的焦點。隊友會自動載入 CLAUDE.md、MCP 伺服器和 skills，但產生提示中的所有內容都會從一開始就新增到其上下文中。
* 工作完成時清理團隊。活躍的隊友即使閒置也會繼續消耗 token。
* Agent 團隊預設為停用。在您的[settings.json](/docs/zh-TW/settings)或環境中設定 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` 以啟用它們。請參閱[啟用 agent 團隊](/docs/zh-TW/agent-teams#enable-agent-teams)。

<h2 id="reduce-token-usage">
  減少 token 使用
</h2>

Token 成本隨上下文大小而擴展：Claude 處理的上下文越多，您使用的 token 就越多。Claude Code 透過 [prompt caching](/docs/zh-TW/prompt-caching)（減少重複內容（如系統提示）的成本）和 auto-compact（在接近上下文限制時總結對話歷史記錄）自動優化成本。

以下策略可幫助您保持上下文較小並降低每條訊息的成本。

<h3 id="manage-context-proactively">
  主動管理上下文
</h3>

使用 `/usage` 檢查您目前的 token 使用情況，或[設定您的狀態行](/docs/zh-TW/statusline#context-window-usage)以持續顯示它。

* **在任務之間清除**：切換到不相關的工作時，使用 `/clear` 重新開始。過時的上下文會在後續的每條訊息上浪費 token。在清除之前使用 `/rename` 以便稍後輕鬆找到工作階段，然後使用 `/resume` 返回到它。
* **新增自訂壓縮指示**：`/compact Focus on code samples and API usage` 告訴 Claude 在總結期間要保留什麼。

您也可以在 CLAUDE.md 中自訂壓縮行為：

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  選擇正確的模型
</h3>

Sonnet 能很好地處理大多數編碼任務，成本低於 Opus。為複雜的架構決策或多步驟推理保留 Opus。使用 `/model` 在工作階段中途切換模型，或在 `/config` 中設定預設值。對於簡單的 subagent 任務，在您的 [subagent 設定](/docs/zh-TW/sub-agents#choose-a-model)中指定 `model: haiku`。

<h3 id="reduce-mcp-server-overhead">
  減少 MCP 伺服器開銷
</h3>

MCP 工具定義[預設為延遲](/docs/zh-TW/mcp#scale-with-mcp-tool-search)，因此只有工具名稱進入上下文，直到 Claude 使用特定工具。執行 `/context` 以查看消耗空間的內容。

* **在可用時偏好 CLI 工具**：`gh`、`aws`、`gcloud` 和 `sentry-cli` 等工具比 MCP 伺服器更具上下文效率，因為它們不會新增任何每個工具的列表。Claude 可以直接執行 CLI 命令。
* **停用未使用的伺服器**：執行 `/mcp` 以查看已設定的伺服器，並停用任何您未主動使用的伺服器。

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  為型別化語言安裝程式碼智慧外掛
</h3>

[程式碼智慧外掛](/docs/zh-TW/discover-plugins#code-intelligence)為 Claude 提供精確的符號導航，而不是基於文字的搜尋，在探索不熟悉的程式碼時減少不必要的檔案讀取。單一「前往定義」呼叫取代了可能需要的 grep 後跟讀取多個候選檔案。已安裝的語言伺服器也會在編輯後自動報告型別錯誤，因此 Claude 無需執行編譯器即可捕捉錯誤。

<h3 id="offload-processing-to-hooks-and-skills">
  將處理卸載到 hooks 和 skills
</h3>

自訂 [hooks](/docs/zh-TW/hooks)可以在 Claude 看到資料之前對其進行預處理。Claude 不是讀取 10,000 行日誌檔案來尋找錯誤，hook 可以 grep `ERROR` 並僅返回匹配的行，將上下文從數萬個 token 減少到數百個。

[skill](/docs/zh-TW/skills)可以為 Claude 提供領域知識，因此它不必進行探索。例如，「codebase-overview」skill 可以描述您的專案架構、關鍵目錄和命名慣例。當 Claude 呼叫該 skill 時，它會立即獲得此上下文，而不是花費 token 讀取多個檔案來理解結構。

例如，此 PreToolUse hook 篩選測試輸出以僅顯示失敗：

<Tabs>
  <Tab title="settings.json">
    將此新增到您的 [settings.json](/docs/zh-TW/settings#settings-files)以在每個 Bash 命令之前執行 hook：

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    hook 呼叫此指令碼。建立資料夾，使用 `mkdir -p ~/.claude/hooks`，將下面的指令碼儲存為 `~/.claude/hooks/filter-test-output.sh`，並使用 `chmod +x ~/.claude/hooks/filter-test-output.sh` 使其可執行。它檢查命令是否為測試執行器並修改它以僅顯示失敗：

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  將指示從 CLAUDE.md 移至 skills
</h3>

您的 [CLAUDE.md](/docs/zh-TW/memory)檔案在工作階段開始時載入到上下文中。如果它包含特定工作流程的詳細指示（例如 PR 審查或資料庫遷移），即使您在進行不相關的工作時，這些 token 也會存在。[Skills](/docs/zh-TW/skills)僅在呼叫時按需載入，因此將專門指示移至 skills 可以保持您的基本上下文較小。目標是透過僅包含必要內容來將 CLAUDE.md 保持在 200 行以下。

<h3 id="adjust-extended-thinking">
  調整延伸思考
</h3>

延伸思考預設為啟用，因為它可以顯著改善複雜規劃和推理任務的效能。思考 token 會作為輸出 token 計費，預設預算可能是每個請求數萬個 token，取決於模型。對於不需要深度推理的較簡單任務，您可以透過在 `/effort` 中降低[努力等級](/docs/zh-TW/model-config#adjust-effort-level)或在 `/model` 中降低、在 `/config` 中停用思考，或在具有[固定思考預算](/docs/zh-TW/model-config#adaptive-reasoning-and-fixed-thinking-budgets)的模型上，透過設定 `MAX_THINKING_TOKENS` [環境變數](/docs/zh-TW/env-vars)（例如 `MAX_THINKING_TOKENS=8000`）來降低預算，以降低成本。自適應推理模型會忽略非零預算，因此請改用努力等級。Fable 5 上無法停用思考，它始終使用延伸思考。

<h3 id="delegate-verbose-operations-to-subagents">
  將詳細操作委派給 subagents
</h3>

執行測試、擷取文件或處理日誌檔案可能會消耗大量上下文。將這些委派給 [subagents](/docs/zh-TW/sub-agents#isolate-high-volume-operations)，以便詳細輸出保留在 subagent 的上下文中，而只有摘要返回到您的主要對話。

<h3 id="manage-agent-team-costs">
  管理 agent 團隊成本
</h3>

當隊友在 plan mode 中執行時，Agent 團隊使用的 token 大約是標準工作階段的 7 倍，因為每位隊友維護自己的上下文視窗並作為單獨的 Claude 執行個體執行。保持團隊任務小且自成一體，以限制每位隊友的 token 使用。有關詳細資訊，請參閱 [agent 團隊](/docs/zh-TW/agent-teams)。

<h3 id="write-specific-prompts">
  撰寫具體提示
</h3>

模糊的請求（例如「改進此程式碼庫」）會觸發廣泛掃描。具體的請求（例如「在 auth.ts 中的登入函式中新增輸入驗證」）讓 Claude 能夠以最少的檔案讀取高效地工作。

<h3 id="work-efficiently-on-complex-tasks">
  有效處理複雜任務
</h3>

對於較長或更複雜的工作，這些習慣有助於避免因走錯方向而浪費的 token：

* **對複雜任務使用 plan mode**：按 Shift+Tab 進入 [plan mode](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode)，然後再進行實施。Claude 探索程式碼庫並提出一個方法供您批准，防止當初始方向錯誤時進行昂貴的返工。
* **及早糾正方向**：如果 Claude 開始朝著錯誤的方向前進，按 Escape 立即停止。使用 `/rewind` 或雙擊 Escape 將對話和程式碼恢復到先前的 checkpoint。
* **提供驗證目標**：在您的提示中包含測試案例、貼上螢幕截圖或定義預期輸出。當 Claude 可以驗證自己的工作時，它會在您需要請求修復之前捕捉問題。
* **增量測試**：寫一個檔案、測試它，然後繼續。這會在問題便宜時及早捕捉問題。

<h2 id="background-token-usage">
  背景 token 使用
</h2>

Claude Code 即使在閒置時也會為某些背景功能使用 token：

* **對話總結**：為 `claude --resume` 功能總結先前對話的背景工作
* **命令處理**：某些命令（例如 `/usage`）可能會產生檢查狀態的請求

這些背景程序即使沒有主動互動也會消耗少量 token（通常每個工作階段不到 \$0.04）。

<h2 id="understanding-changes-in-claude-code-behavior">
  瞭解 Claude Code 行為的變化
</h2>

Claude Code 定期接收可能改變功能工作方式的更新，包括成本報告。執行 `claude --version` 以檢查您目前的版本。如有具體計費問題，請透過您的[Console 帳戶](https://platform.claude.com/login)聯絡 Anthropic 支援。
