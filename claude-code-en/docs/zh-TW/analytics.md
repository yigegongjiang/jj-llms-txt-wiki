> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用分析追蹤團隊使用情況

> 在分析儀表板中查看 Claude Code 使用指標、追蹤採用情況並衡量工程速度。

Claude Code 提供分析儀表板，幫助組織了解開發人員使用模式、追蹤貢獻指標，並衡量 Claude Code 對工程速度的影響。根據您的計劃訪問儀表板：

| 計劃                            | 儀表板 URL                                                                    | 包含內容                           | 了解更多                                            |
| ----------------------------- | -------------------------------------------------------------------------- | ------------------------------ | ----------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | 使用指標、與 GitHub 整合的貢獻指標、排行榜、數據匯出 | [詳情](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | 使用指標、支出追蹤、團隊洞察                 | [詳情](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  訪問 Team 和 Enterprise 的分析
</h2>

導航至 [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code)。管理員和所有者可以查看儀表板。

Team 和 Enterprise 儀表板包括：

* **使用指標**：已接受的代碼行數、建議接受率、每日活躍用戶和會話
* **貢獻指標**：使用 Claude Code 協助發布的 PR 和代碼行數，具有 [GitHub 整合](#enable-contribution-metrics)
* **排行榜**：按 Claude Code 使用情況排名的頂級貢獻者
* **數據匯出**：將貢獻數據下載為 CSV 格式以進行自訂報告

如需每個用戶的令牌計數和成本估計，請配置 [OpenTelemetry 匯出](/docs/zh-TW/monitoring-usage)，或從您組織的分析設定匯出 [支出報告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)，其中列出每個用戶和每個模型的令牌使用情況和估計使用額度支出。

<h3 id="enable-contribution-metrics">
  啟用貢獻指標
</h3>

<Note>
  貢獻指標處於公開測試版，可在 Claude for Teams 和 Claude for Enterprise 計劃上使用。這些指標僅涵蓋您 claude.ai 組織內的用戶。通過 Claude Console API 或第三方整合的使用不包括在內。
</Note>

使用和採用數據適用於所有 Claude for Teams 和 Claude for Enterprise 帳戶。貢獻指標需要額外設置以連接您的 GitHub 組織。

您需要所有者角色來配置分析設置。GitHub 管理員必須安裝 GitHub 應用程序。

<Warning>
  啟用了 [Zero Data Retention](/docs/zh-TW/zero-data-retention) 的組織無法使用貢獻指標。分析儀表板將僅顯示使用指標。
</Warning>

<Steps>
  <Step title="安裝 GitHub 應用程序">
    GitHub 管理員在您組織的 GitHub 帳戶上安裝 Claude GitHub 應用程序，位址為 [github.com/apps/claude](https://github.com/apps/claude)。
  </Step>

  <Step title="啟用 Claude Code 分析">
    Claude 所有者導航至 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 並啟用 Claude Code 分析功能。
  </Step>

  <Step title="啟用 GitHub 分析">
    在同一頁面上，啟用'GitHub 分析'切換。
  </Step>

  <Step title="使用 GitHub 進行身份驗證">
    完成 GitHub 身份驗證流程並選擇要包含在分析中的 GitHub 組織。
  </Step>
</Steps>

啟用後，數據通常在 24 小時內出現，並進行每日更新。如果沒有數據出現，您可能會看到以下消息之一：

* **「需要 GitHub 應用程序」**：安裝 GitHub 應用程序以查看貢獻指標
* **「數據處理進行中」**：幾天後重新檢查，如果數據未出現，請確認 GitHub 應用程序已安裝

貢獻指標支持 GitHub Cloud 和 GitHub Enterprise Server。

<h3 id="review-summary-metrics">
  查看摘要指標
</h3>

<Note>
  這些指標故意保守，代表對 Claude Code 實際影響的低估。只有高度確信 Claude Code 參與的代碼行和 PR 才會被計算。
</Note>

儀表板在頂部顯示這些摘要指標：

* **帶有 CC 的 PR**：包含至少一行使用 Claude Code 編寫的代碼的已合併拉取請求的總計數
* **帶有 CC 的代碼行**：所有已合併 PR 中使用 Claude Code 協助編寫的代碼行總數。僅計算「有效行」：規範化後超過 3 個字符的行，不包括空行和僅包含括號或瑣碎標點符號的行。
* **帶有 Claude Code 的 PR (%)**：包含 Claude Code 協助代碼的所有已合併 PR 的百分比
* **建議接受率**：用戶接受 Claude Code 代碼編輯建議的次數百分比，包括 Edit、Write 和 NotebookEdit 工具使用
* **已接受的代碼行**：Claude Code 編寫且用戶在其會話中已接受的代碼行總數。這不包括被拒絕的建議，也不追蹤後續刪除。

<h3 id="explore-the-charts">
  探索圖表
</h3>

儀表板包括多個圖表以可視化一段時間內的趨勢。

<h4 id="track-adoption">
  追蹤採用
</h4>

採用圖表顯示每日使用趨勢：

* **用戶**：每日活躍用戶
* **會話**：每天活躍 Claude Code 會話的數量

<h4 id="measure-prs-per-user">
  衡量每個用戶的 PR
</h4>

此圖表顯示一段時間內的個人開發人員活動：

* **每個用戶的 PR**：每天合併的 PR 總數除以每日活躍用戶
* **用戶**：每日活躍用戶

使用此功能了解隨著 Claude Code 採用增加，個人生產力如何變化。

<h4 id="view-pull-requests-breakdown">
  查看拉取請求細分
</h4>

拉取請求圖表顯示已合併 PR 的每日細分：

* **帶有 CC 的 PR**：包含 Claude Code 協助代碼的拉取請求
* **不帶 CC 的 PR**：不包含 Claude Code 協助代碼的拉取請求

切換至**代碼行**視圖以按代碼行而不是 PR 計數查看相同的細分。

<h4 id="find-top-contributors">
  查找頂級貢獻者
</h4>

排行榜顯示按貢獻量排名的前 10 名用戶。在以下之間切換：

* **拉取請求**：顯示每個用戶的帶有 Claude Code 的 PR 與所有 PR
* **代碼行**：顯示每個用戶的帶有 Claude Code 的行與所有行

點擊**匯出所有用戶**以將所有用戶的完整貢獻數據下載為 CSV 文件。匯出包括所有用戶，而不僅僅是顯示的前 10 名。

<h3 id="pr-attribution">
  PR 歸因
</h3>

啟用貢獻指標後，Claude Code 會分析已合併的拉取請求，以確定哪些代碼是使用 Claude Code 協助編寫的。這是通過將 Claude Code 會話活動與每個 PR 中的代碼進行匹配來完成的。

<h4 id="tagging-criteria">
  標記標準
</h4>

如果 PR 包含在 Claude Code 會話期間編寫的至少一行代碼，則將其標記為「帶有 Claude Code」。系統使用保守匹配：只有高度確信 Claude Code 參與的代碼才被計為協助。

<h4 id="attribution-process">
  歸因過程
</h4>

當拉取請求被合併時：

1. 從 PR diff 中提取添加的行
2. 識別在時間窗口內編輯匹配文件的 Claude Code 會話
3. 使用多種策略將 PR 行與 Claude Code 輸出進行匹配
4. 計算 AI 協助行和總行的指標

在比較之前，行被規範化：空格被修剪、多個空格被折疊、引號被標準化、文本被轉換為小寫。

包含 Claude Code 協助行的已合併拉取請求在 GitHub 中被標記為 `claude-code-assisted`。

<h4 id="time-window">
  時間窗口
</h4>

PR 合併日期前 21 天至後 2 天的會話被考慮用於歸因匹配。

<h4 id="excluded-files">
  排除的文件
</h4>

某些文件會自動從分析中排除，因為它們是自動生成的：

* 鎖定文件：package-lock.json、yarn.lock、Cargo.lock 及類似文件
* 生成的代碼：Protobuf 輸出、構建工件、縮小的文件
* 構建目錄：dist/、build/、node\_modules/、target/
* 測試夾具：快照、盒帶、模擬數據
* 超過 1,000 個字符的行，可能是縮小或生成的

<h4 id="attribution-notes">
  歸因說明
</h4>

在解釋歸因數據時，請記住這些額外詳情：

* 由開發人員大幅重寫的代碼，差異超過 20%，不歸因於 Claude Code
* 21 天窗口外的會話不被考慮
* 該算法在執行歸因時不考慮 PR 源或目標分支

<h3 id="get-the-most-from-analytics">
  從分析中獲得最大收益
</h3>

使用貢獻指標來展示 ROI、識別採用模式並找到可以幫助他人入門的團隊成員。

<h4 id="monitor-adoption">
  監控採用
</h4>

追蹤採用圖表和用戶計數以識別：

* 可以分享最佳實踐的活躍用戶
* 整個組織的整體採用趨勢
* 可能表示摩擦或問題的使用下降

<h4 id="measure-roi">
  衡量 ROI
</h4>

貢獻指標幫助回答「這個工具值得投資嗎？」，使用來自您自己代碼庫的數據：

* 隨著採用增加，追蹤一段時間內每個用戶的 PR 變化
* 比較使用和不使用 Claude Code 發布的 PR 和代碼行
* 與 [DORA 指標](https://dora.dev/)、衝刺速度或其他工程 KPI 一起使用，以了解採用 Claude Code 帶來的變化

<h4 id="identify-power-users">
  識別超級用戶
</h4>

排行榜幫助您找到具有高 Claude Code 採用率的團隊成員，他們可以：

* 與團隊分享提示技術和工作流程
* 提供有關什麼運作良好的反饋
* 幫助新用戶入門

<h4 id="access-data-programmatically">
  以編程方式訪問數據
</h4>

在 Enterprise 計劃上，[Claude Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) 為您的組織返回每個用戶的參與度、使用情況和成本報告，涵蓋所有 Claude 表面，包括 Claude Code。主要所有者在 [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys) 處使用 `read:analytics` 範圍創建密鑰。該 API 在 Teams 計劃上不可用。

要通過 GitHub 查詢貢獻數據，請搜索標記為 `claude-code-assisted` 的 PR。

<h2 id="access-analytics-for-api-customers">
  訪問 API 客戶的分析
</h2>

使用 Claude Console 的 API 客戶可以在 [platform.claude.com/claude-code](https://platform.claude.com/claude-code) 訪問分析。您需要 UsageView 權限才能訪問儀表板，該權限授予開發人員、計費、管理員、所有者和主要所有者角色。若要以程式設計方式提取相同的每日每用戶指標，請使用 [Claude Code Analytics API](https://platform.claude.com/docs/zh-TW/build-with-claude/claude-code-analytics-api) 搭配管理員 API 金鑰。

<Note>
  GitHub 整合的貢獻指標目前不適用於 API 客戶。Console 儀表板僅顯示使用和支出指標。
</Note>

Console 儀表板顯示：

* **已接受的代碼行**：Claude Code 編寫且用戶在其會話中已接受的代碼行總數。這不包括被拒絕的建議，也不追蹤後續刪除。
* **建議接受率**：用戶接受代碼編輯工具使用的次數百分比，包括 Edit、Write 和 NotebookEdit 工具。
* **活動**：圖表上顯示的每日活躍用戶和會話。
* **支出**：每日 API 成本（以美元計）以及用戶計數。

<h3 id="view-team-insights">
  查看團隊洞察
</h3>

團隊洞察表顯示每個用戶的指標：

* **成員**：所有已向 Claude Code 進行身份驗證的用戶。API 密鑰用戶按密鑰標識符顯示，OAuth 用戶按電子郵件地址顯示。
* **本月支出**：每個用戶當前月份的每用戶 API 成本總計。
* **本月代碼行**：每個用戶當前月份已接受代碼行的每用戶總計。

<Note>
  Console 儀表板中的支出數字是用於分析目的的估計值。有關實際成本，請參閱您的計費頁面。
</Note>

<h2 id="related-resources">
  相關資源
</h2>

* [使用 OpenTelemetry 進行監控](/docs/zh-TW/monitoring-usage)：將實時指標和事件匯出到您的可觀測性堆棧
* [有效管理成本](/docs/zh-TW/costs)：設置支出限制並優化令牌使用
* [權限](/docs/zh-TW/permissions)：配置角色和權限
