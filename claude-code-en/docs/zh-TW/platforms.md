> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 平台和整合

> 選擇在何處執行 Claude Code 以及要連接什麼。比較 CLI、Desktop、VS Code、JetBrains、Web 和 Chrome、Slack 和 CI/CD 等整合。

Claude Code 在各處執行相同的底層引擎，但每個介面都針對不同的工作方式進行了調整。此頁面幫助您為工作流程選擇合適的平台，並連接您已經使用的工具。

<h2 id="where-to-run-claude-code">
  在何處執行 Claude Code
</h2>

根據您喜歡的工作方式和專案所在位置選擇平台。

| 平台                                   | 最適合                                               | 您將獲得                                                                                                                                                      |
| :----------------------------------- | :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/zh-TW/quickstart)             | 終端工作流程、指令碼、遠端伺服器                                  | 完整功能集、[Agent SDK](/docs/zh-TW/headless)、macOS 上的[電腦使用](/docs/zh-TW/computer-use)（Pro 和 Max）、第三方提供商                                                                  |
| [Desktop](/docs/zh-TW/desktop)            | 視覺審查、並行會話、託管設定                                    | Diff 檢視器、應用程式預覽、Pro 和 Max 上的[電腦使用](/docs/zh-TW/desktop#let-claude-use-your-computer)和 [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch)                        |
| [VS Code](/docs/zh-TW/vs-code)            | 在 VS Code 內工作而無需切換到終端                             | 內聯 diff、整合終端、檔案上下文                                                                                                                                        |
| [JetBrains](/docs/zh-TW/jetbrains)        | 在 IntelliJ、PyCharm、WebStorm 或其他 JetBrains IDE 內工作 | Diff 檢視器、選擇共享、終端會話                                                                                                                                        |
| [Web](/docs/zh-TW/claude-code-on-the-web) | 不需要太多操控的長時間執行任務，或應該在您離線時繼續進行的工作                   | 雲端、Anthropic 託管（預設）；在您斷開連接後繼續                                                                                                                             |
| [Mobile](/docs/zh-TW/mobile)              | 在遠離電腦時啟動和監控任務                                     | iOS 和 Android 版 Claude 應用程式的雲端會話、用於本地會話的 [Remote Control](/docs/zh-TW/remote-control)、Pro 和 Max 上的 [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch) 到 Desktop |

CLI 是終端原生工作的最完整介面：指令碼和 Agent SDK 僅限 CLI。第三方提供商也可在 [VS Code](/docs/zh-TW/vs-code#use-third-party-providers) 和 [JetBrains](/docs/zh-TW/feature-availability#features-available-on-every-provider) 中使用，其在您 IDE 的終端中執行 CLI。企業 [Desktop](/docs/zh-TW/desktop) 部署支援 Google Cloud 的 Agent Platform，Desktop 支援[閘道提供商](/docs/zh-TW/llm-gateway-connect#desktop-app)；對於 Amazon Bedrock 或 Microsoft Foundry，請改用 CLI 或 IDE 擴充功能，或 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)，其在這些提供商上執行 Code 標籤。Desktop 和 IDE 擴充功能用視覺審查和更緊密的編輯器整合來交換一些僅限 CLI 的功能。Web 在雲端中執行，因此任務在您斷開連接後會繼續進行。Mobile 是進入這些相同雲端會話或透過 Remote Control 進入本地會話的瘦用戶端，並可以透過 Dispatch 將任務發送到 Desktop。

您可以在同一專案上混合使用介面。配置、專案記憶體和 MCP 伺服器在本地介面之間共享。

<h2 id="connect-your-tools">
  連接您的工具
</h2>

整合讓 Claude 與程式碼庫外的服務協作。

| 整合                                               | 它的作用                               | 用途                                             |
| :----------------------------------------------- | :--------------------------------- | :--------------------------------------------- |
| [Chrome](/docs/zh-TW/chrome)                          | 使用您已登入的會話控制您的瀏覽器                   | 測試 Web 應用程式、填寫表單、自動化沒有 API 的網站                 |
| [GitHub Actions](/docs/zh-TW/github-actions)          | 在您的 CI 管道中執行 Claude                | 自動化 PR 審查、問題分類、排程維護                            |
| [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)              | 與 GitHub Actions 相同，但用於 GitLab     | GitLab 上的 CI 驅動自動化                             |
| [Code Review](/docs/zh-TW/code-review)                | 自動審查每個 PR                          | 在人工審查之前捕捉錯誤                                    |
| [Slack](/docs/zh-TW/slack)                            | 回應您的頻道中的 `@Claude` 提及              | 將錯誤報告轉換為團隊聊天中的拉取請求                             |
| [Claude Tag](https://claude.com/docs/claude-tag) | 以您組織的共享身分執行 `@Claude`，具有管理員設定的存取權限 | Team 和 Enterprise 方案上的共享團隊存取，而不是按使用者的 Slack 會話 |

對於此處未列出的整合，[MCP servers](/docs/zh-TW/mcp) 和[連接器](/docs/zh-TW/desktop#connect-external-tools)讓您連接幾乎任何東西：Linear、Notion、Google Drive 或您自己的內部 API。

<h2 id="work-when-you-are-away-from-your-terminal">
  當您遠離終端時工作
</h2>

Claude Code 提供了多種方式讓您在不在終端機時進行工作。它們在觸發工作的方式、Claude 執行的位置以及您需要設定的程度上有所不同。

|                                                             | 觸發                                                                   | Claude 執行位置                                                                                     | 設定                                                                                                                  | 最適合                       |
| :---------------------------------------------------------- | :------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ | :------------------------ |
| [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch)           | 從 Claude 行動應用程式傳送任務訊息                                                | 您的機器 (Desktop)                                                                                  | [將行動應用程式與 Desktop 配對](https://support.claude.com/en/articles/13947068)                                              | 在您不在時委派工作，最少設定            |
| [Remote Control](/docs/zh-TW/remote-control)                     | 從 [claude.ai/code](https://claude.ai/code) 或 Claude 行動應用程式驅動執行中的工作階段 | 您的機器 (CLI 或 VS Code)                                                                            | 執行 `claude remote-control`                                                                                          | 從另一個裝置控制進行中的工作            |
| [Channels](/docs/zh-TW/channels)                                 | 從聊天應用程式 (如 Telegram 或 Discord) 或您自己的伺服器推送事件                          | 您的機器 (CLI)                                                                                      | [安裝頻道外掛程式](/docs/zh-TW/channels#quickstart) 或 [建立您自己的](/docs/zh-TW/channels-reference)                                        | 對外部事件 (如 CI 失敗或聊天訊息) 做出反應 |
| [Slack](/docs/zh-TW/slack)                                       | 在團隊頻道中提及 `@Claude`                                                   | Anthropic 雲端                                                                                    | [安裝 Slack 應用程式](/docs/zh-TW/slack#setting-up-claude-code-in-slack) 並啟用 [網路上的 Claude Code](/docs/zh-TW/claude-code-on-the-web) | 從團隊聊天進行 PR 和審查            |
| [Self-hosted environments](/docs/zh-TW/self-hosted-environments) | 啟動 [雲端工作階段](/docs/zh-TW/claude-code-on-the-web) 並選擇您組織的環境                 | 您組織的基礎設施                                                                                        | [部署執行器](/docs/zh-TW/self-hosted-environments-quickstart)，在 Team 和 Enterprise 方案上                                         | 必須在您的網路內執行的雲端工作階段         |
| [Scheduled tasks](/docs/zh-TW/scheduled-tasks)                   | 設定排程                                                                 | [CLI](/docs/zh-TW/scheduled-tasks)、[Desktop](/docs/zh-TW/desktop-scheduled-tasks) 或 [雲端](/docs/zh-TW/routines) | 選擇頻率                                                                                                                | 定期自動化 (如每日審查)             |

如果您不確定從何處開始，[安裝 CLI](/docs/zh-TW/quickstart) 並在專案目錄中執行它。如果您不想使用終端，[Desktop](/docs/zh-TW/desktop-quickstart) 為您提供相同的引擎和圖形介面。

<h2 id="related-resources">
  相關資源
</h2>

<h3 id="platforms">
  平台
</h3>

* [CLI 快速入門](/docs/zh-TW/quickstart)：在終端中安裝並執行您的第一個命令
* [Desktop](/docs/zh-TW/desktop)：視覺 diff 審查、並行會話、電腦使用和 Dispatch
* [VS Code](/docs/zh-TW/vs-code)：編輯器內的 Claude Code 擴充功能
* [JetBrains](/docs/zh-TW/jetbrains)：IntelliJ、PyCharm 和其他 JetBrains IDE 的擴充功能
* [Web](/docs/zh-TW/claude-code-on-the-web)：在您斷開連接時繼續執行的雲端會話，位於 claude.ai/code
* [Projects](/docs/zh-TW/claude-projects)：一個對話，Claude 在其中協調許多雲端會話以完成一項工作並回報結果
* [Mobile](/docs/zh-TW/mobile)：用於在遠離電腦時啟動和監控任務的 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 和 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 版 Claude 應用程式

<h3 id="integrations">
  整合
</h3>

* [Chrome](/docs/zh-TW/chrome)：使用您已登入的會話自動化瀏覽器任務
* [Computer use](/docs/zh-TW/computer-use)：讓 Claude 在 macOS 上開啟應用程式並控制您的螢幕
* [GitHub Actions](/docs/zh-TW/github-actions)：在您的 CI 管道中執行 Claude
* [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)：GitLab 的相同功能
* [Code Review](/docs/zh-TW/code-review)：每個拉取請求上的自動審查
* [Slack](/docs/zh-TW/slack)：從團隊聊天發送任務，取回 PR
* [Claude Tag](https://claude.com/docs/claude-tag)：在 Team 和 Enterprise 方案上執行 `@Claude` 作為您組織的共享身分

<h3 id="remote-access">
  遠端存取
</h3>

* [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch)：從您的手機傳送任務，它可以生成 Desktop 會話
* [Remote Control](/docs/zh-TW/remote-control)：從您的手機或瀏覽器驅動執行中的會話
* [Channels](/docs/zh-TW/channels)：將來自聊天應用程式或您自己的伺服器的事件推送到會話中
* [Scheduled tasks](/docs/zh-TW/scheduled-tasks)：按定期排程執行提示
