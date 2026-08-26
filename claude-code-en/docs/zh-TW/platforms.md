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
| [Web](/docs/zh-TW/claude-code-on-the-web) | 不需要太多操控的長時間執行任務，或應該在您離線時繼續進行的工作                   | Anthropic 託管雲端、在您斷開連接後繼續                                                                                                                                  |
| Mobile                               | 在遠離電腦時啟動和監控任務                                     | iOS 和 Android 版 Claude 應用程式的雲端會話、用於本地會話的 [Remote Control](/docs/zh-TW/remote-control)、Pro 和 Max 上的 [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch) 到 Desktop |

CLI 是終端原生工作的最完整介面：指令碼和 Agent SDK 僅限 CLI。第三方提供商也可在 [VS Code](/docs/zh-TW/vs-code#use-third-party-providers) 中使用。企業 [Desktop](/docs/zh-TW/desktop) 部署支援 Google Cloud 的 Agent Platform，Desktop 支援[閘道提供商](/docs/zh-TW/llm-gateway-connect#desktop-app)；對於 Amazon Bedrock 或 Microsoft Foundry，請改用 CLI 或 VS Code，或 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)，其在這些提供商上執行 Code 標籤。Desktop 和 IDE 擴充功能用視覺審查和更緊密的編輯器整合來交換一些僅限 CLI 的功能。Web 在 Anthropic 的雲端中執行，因此任務在您斷開連接後會繼續進行。Mobile 是進入這些相同雲端會話或透過 Remote Control 進入本地會話的瘦用戶端，並可以透過 Dispatch 將任務發送到 Desktop。

您可以在同一專案上混合使用介面。配置、專案記憶體和 MCP 伺服器在本地介面之間共享。

<h2 id="connect-your-tools">
  連接您的工具
</h2>

整合讓 Claude 與程式碼庫外的服務協作。

| 整合                                      | 它的作用                           | 用途                             |
| :-------------------------------------- | :----------------------------- | :----------------------------- |
| [Chrome](/docs/zh-TW/chrome)                 | 使用您已登入的會話控制您的瀏覽器               | 測試 Web 應用程式、填寫表單、自動化沒有 API 的網站 |
| [GitHub Actions](/docs/zh-TW/github-actions) | 在您的 CI 管道中執行 Claude            | 自動化 PR 審查、問題分類、排程維護            |
| [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)     | 與 GitHub Actions 相同，但用於 GitLab | GitLab 上的 CI 驅動自動化             |
| [Code Review](/docs/zh-TW/code-review)       | 自動審查每個 PR                      | 在人工審查之前捕捉錯誤                    |
| [Slack](/docs/zh-TW/slack)                   | 回應您的頻道中的 `@Claude` 提及          | 將錯誤報告轉換為團隊聊天中的拉取請求             |

對於此處未列出的整合，[MCP servers](/docs/zh-TW/mcp) 和[連接器](/docs/zh-TW/desktop#connect-external-tools)讓您連接幾乎任何東西：Linear、Notion、Google Drive 或您自己的內部 API。

<h2 id="work-when-you-are-away-from-your-terminal">
  當您遠離終端時工作
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

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
* [Web 上的 Claude Code](/docs/zh-TW/claude-code-on-the-web)：在您斷開連接時繼續執行的雲端會話
* Mobile：用於在遠離電腦時啟動和監控任務的 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 和 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 版 Claude 應用程式

<h3 id="integrations">
  整合
</h3>

* [Chrome](/docs/zh-TW/chrome)：使用您已登入的會話自動化瀏覽器任務
* [Computer use](/docs/zh-TW/computer-use)：讓 Claude 在 macOS 上開啟應用程式並控制您的螢幕
* [GitHub Actions](/docs/zh-TW/github-actions)：在您的 CI 管道中執行 Claude
* [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)：GitLab 的相同功能
* [Code Review](/docs/zh-TW/code-review)：每個拉取請求上的自動審查
* [Slack](/docs/zh-TW/slack)：從團隊聊天發送任務，取回 PR

<h3 id="remote-access">
  遠端存取
</h3>

* [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch)：從您的手機傳送任務，它可以生成 Desktop 會話
* [Remote Control](/docs/zh-TW/remote-control)：從您的手機或瀏覽器驅動執行中的會話
* [Channels](/docs/zh-TW/channels)：將來自聊天應用程式或您自己的伺服器的事件推送到會話中
* [Scheduled tasks](/docs/zh-TW/scheduled-tasks)：按定期排程執行提示
