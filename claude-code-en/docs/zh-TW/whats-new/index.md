> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 最新動態

> Claude Code 功能的每週摘要，包含程式碼片段、示範和背景說明。

每週開發摘要重點介紹最有可能改變您工作方式的功能。每個條目都包含可執行的程式碼、簡短的示範和完整文件的連結。如需每個錯誤修復和次要改進，請參閱 [changelog](/docs/zh-TW/changelog)。

<Update label="Week 28" description="July 6–10, 2026" tags={["v2.1.202–v2.1.206"]}>
  **Desktop 上的應用內瀏覽器**：Desktop 上的 Claude Code 現在配備內建瀏覽器，讓 Claude 可以調出文件、設計或任何其他網站，並以與本機開發伺服器預覽相同的方式與頁面互動。

  本週還有：**`/doctor`** 是完整的設定檢查，可診斷問題並修復它們，`/checkup` 是其別名；**auto mode** 阻止文字記錄篡改，並在未解決的變數上執行 `rm -rf` 前詢問；以及 **agent view rows** 顯示彩色狀態字和分類器編寫的標題。

  [閱讀 Week 28 摘要 →](/docs/zh-TW/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="June 29 – July 3, 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**：Pro、Team Standard 和 Enterprise 訂閱座位的新預設模型，具有頂級編碼和工具使用能力（Sonnet 定價），原生 1M 代幣內容視窗，以及預設啟用自適應思考。

  本週還有：**Chrome 中的 Claude** 現已在所有直接 Anthropic 計畫上正式推出；**subagents 預設在背景執行**，讓 Claude 在它們執行時繼續工作；**Linux 上的 Claude Desktop** 在 Ubuntu 和 Debian 上進入測試版；以及 **`/radio`** 調入 Claude FM lo-fi 廣播。

  [閱讀 Week 27 摘要 →](/docs/zh-TW/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="June 22–26, 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**：從您的 shell 驗證已設定的 MCP 伺服器，而不是使用互動式 `/mcp` 選單，稍後可以使用 `claude mcp logout` 清除其儲存的認證。

  本週還有：**shell mode 回應命令輸出**（`! npm test` 無需第二個提示即可獲得說明）；**`/rewind`** 可以從執行 `/clear` 之前恢復對話；以及 **background subagents** 現在在主工作階段中顯示權限提示，而不是自動拒絕。

  [閱讀 Week 26 摘要 →](/docs/zh-TW/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="June 15–19, 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**：將工作階段的輸出轉換為 claude.ai 上的即時、可共享頁面，在工作階段進行時就地更新，現在在 Team 和 Enterprise 計畫上進行測試。

  本週還有：**deny 和 ask rules 使用 `Tool(param:value)` 符合工具參數**，例如 `Agent(model:opus)`；**`/config key=value`** 從提示、`-p` 模式和遠端控制設定任何設定；以及 **auto mode 在您未要求捨棄本機工作時阻止破壞性 git 命令**。

  [閱讀 Week 25 摘要 →](/docs/zh-TW/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="June 8–12, 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**：在對話中途將目前的工作階段移至新的工作目錄，無需重建提示快取。

  本週還有：**sub-agents 可以產生自己的 sub-agents**（背景鏈最多五層深）；**`--safe-mode`** 以所有自訂設定停用的狀態啟動 Claude Code 以進行疑難排解；以及 **`fallbackModel`** 配置最多三個依序嘗試的備用模型。

  [閱讀 Week 24 摘要 →](/docs/zh-TW/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="June 1–5, 2026" tags={["v2.1.158–v2.1.165"]}>
  **Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上的 Auto mode**：auto mode 現在可在第三方提供者上用於 Opus 4.7 和 Opus 4.8，用背景安全檢查取代權限提示。

  本週還有：**更安全的自動編輯**在 `acceptEdits` 模式中寫入可執行程式碼的檔案前提示；**`/plugin list`** 內聯列印您安裝的外掛程式；以及**版本需求**讓受管部署要求核准的 Claude Code 版本範圍。

  [閱讀 Week 23 摘要 →](/docs/zh-TW/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="May 25–29, 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**：Max、Team Premium、Enterprise 隨用隨付和 Anthropic API 帳戶的新預設模型，預設具有高努力等級，以及用於最困難任務的 `/effort xhigh`。

  本週還有：**dynamic workflows** 從 Claude 編寫的指令碼協調數十到數百個子代理；**security-guidance plugin** 在 Claude 工作時檢查其變更是否存在漏洞；以及 **fast mode** 在 Opus 4.8 上以 $10/$50 per MTok 執行。

  [閱讀 Week 22 摘要 →](/docs/zh-TW/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="May 18–22, 2026" tags={["v2.1.143–v2.1.149"]}>
  **Pro 計畫上的 Auto mode**：auto mode 現在在 Pro 帳戶上執行，並支援 Sonnet 4.6 與 Opus，用背景安全檢查取代權限提示。

  本週還有：**`/usage`** 按 skill、subagent、plugin 和 MCP server 細分驅動您計畫限制的因素；新的 **`/code-review`** 命令報告正確性錯誤；以及 **background sessions** 出現在 `/resume` 中，並在釘選時保持活躍。

  [閱讀 Week 21 摘要 →](/docs/zh-TW/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="May 11–15, 2026" tags={["v2.1.139–v2.1.142"]}>
  **Agent view**：`claude agents` 為每個 Claude Code 工作階段開啟一個畫面，顯示正在執行的內容、被您阻止的內容以及已完成的內容。

  本週還有：**`/goal`** 讓 Claude 在多個回合中持續工作，直到完成條件成立；**fast mode** 現在預設在 Opus 4.7 上執行；以及 **Rewind menu** 可以使用「Summarize up to here」壓縮較早的背景。

  [閱讀 Week 20 摘要 →](/docs/zh-TW/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="May 4–8, 2026" tags={["v2.1.128–v2.1.136"]}>
  **Plugins 從 `.zip` 檔案和 URL 載入**：`--plugin-dir` 現在接受 `.zip` 檔案，而 `--plugin-url` 會為目前的工作階段擷取外掛程式封存。

  本週還有：**`worktree.baseRef`** 選擇新的 worktrees 是否從遠端預設或本機 `HEAD` 分支；**auto mode hard deny rules** 無條件地阻止操作，不論允許例外；以及 **hooks 看到作用中的努力等級**，透過 `effort.level` 和 `$CLAUDE_EFFORT`。

  [閱讀 Week 19 摘要 →](/docs/zh-TW/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="April 27 – May 1, 2026" tags={["v2.1.120–v2.1.126"]}>
  **沒有 Git Bash 的 Windows**：不再需要 Git for Windows，當 Bash 不存在時，Claude Code 會使用 PowerShell 作為 shell 工具。

  本週還有：**`claude ultrareview`** 將雲端程式碼審查帶到 CI 和指令碼；**`claude project purge`** 清理專案的本機狀態；以及將 **PR URL 貼到 `/resume`** 中找到建立它的工作階段。

  [閱讀 Week 18 摘要 →](/docs/zh-TW/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="April 20–24, 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** 作為公開研究預覽版開放：一群除蟲代理在雲端執行，發現結果會自動回傳到您的 CLI 或桌面應用。

  本週還有：**session recap** 顯示終端機失焦時發生的情況；**custom themes** 讓您從 `/theme` 或外掛程式建立和發佈色彩調色板；**Claude Code on the web** 進行了重新設計，包含新的 sessions 側邊欄和拖放版面配置。

  [閱讀 Week 17 摘要 →](/docs/zh-TW/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="April 13–17, 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** 成為 Max 和 Team Premium 的新預設版本，具有新的 `xhigh` 努力等級（推薦用於大多數編碼工作）和互動式 `/effort` 滑桿來調整設定。

  本週還有：**Routines** 在 Claude Code on the web 上從排程、GitHub 事件或 API 呼叫觸發樣板化雲端代理；**mobile push notifications** 在長時間工作完成或 Claude 需要您時向您的手機發送通知；`/usage` 顯示驅動您限制的因素；CLI 移至原生二進位檔。

  [閱讀 Week 16 摘要 →](/docs/zh-TW/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="April 6–10, 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** 進入早期預覽版：從您的 CLI 在雲端草擬計畫，在網頁編輯器中檢閱和評論，然後遠端執行或拉回本機。第一次執行現在會自動為您建立雲端環境。

  本週還有：**Monitor** 工具將背景事件串流到對話中，讓 Claude 可以追蹤日誌並即時反應，`/loop` 在您省略間隔時自動調整步調，`/team-onboarding` 將您的設定打包成可重播的指南，`/autofix-pr` 從您的終端機開啟 PR 自動修復。

  [閱讀 Week 15 摘要 →](/docs/zh-TW/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="March 30 – April 3, 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** 在研究預覽版中推出至 CLI：Claude 可以開啟原生應用程式、點擊 UI 並從您的終端機驗證變更。最適合用於關閉只有 GUI 才能驗證的事項。

  本週還有：`/powerup` 互動式課程、無閃爍的 alt-screen 渲染、每個工具的 MCP 結果大小覆蓋（最高 500K），以及 Bash 工具 `PATH` 上的外掛程式可執行檔。

  [閱讀 Week 14 摘要 →](/docs/zh-TW/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="March 23–27, 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** 在研究預覽版中推出：分類器處理您的權限提示，讓安全操作無中斷執行，危險操作則被阻止。這是在核准所有操作和 `--dangerously-skip-permissions` 之間的折衷方案。

  本週還有：桌面應用中的 computer use、Web 上的 PR 自動修復、使用 `/` 進行文字記錄搜尋、適用於 Windows 的原生 PowerShell 工具，以及條件式 `if` hooks。

  [閱讀 Week 13 摘要 →](/docs/zh-TW/whats-new/2026-w13)
</Update>
