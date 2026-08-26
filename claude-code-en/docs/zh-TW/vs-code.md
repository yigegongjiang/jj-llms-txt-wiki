> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 VS Code 中使用 Claude Code

> 安裝並配置 VS Code 的 Claude Code 擴充功能。透過內聯差異、@-提及、計畫審查和快捷鍵獲得 AI 編碼協助。

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="VS Code 編輯器，右側開啟 Claude Code 擴充功能面板，顯示與 Claude 的對話" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

VS Code 擴充功能為 Claude Code 提供了原生圖形介面，直接整合到您的 IDE 中。這是在 VS Code 中使用 Claude Code 的推薦方式。

使用此擴充功能，您可以在接受 Claude 的計畫之前進行審查和編輯，在進行編輯時自動接受，從您的選擇中 @-提及具有特定行範圍的檔案，存取對話歷史記錄，以及在單獨的標籤或視窗中開啟多個對話。

<h2 id="prerequisites">
  先決條件
</h2>

安裝前，請確保您擁有：

* VS Code 1.98.0 或更高版本
* Anthropic 帳戶：任何付費 Claude 訂閱（Pro、Max、Team 或 Enterprise）或 Claude Console 帳戶都可以使用，不需要 API 金鑰。首次開啟擴充功能時，您將[使用此帳戶登入](/docs/zh-TW/authentication#log-in-to-claude-code)。如果您透過第三方提供者（如 Amazon Bedrock 或 Google Cloud 的 Agent Platform）存取 Claude，請參閱[使用第三方提供者](#use-third-party-providers)以取得設定說明。

<Tip>
  此擴充功能包含其自有的 CLI（命令列介面）副本供聊天面板使用。若要在 VS Code 的整合終端機中執行 `claude`，您還需要[獨立 CLI 安裝](/docs/zh-TW/setup)。詳細資訊請參閱 [VS Code 擴充功能與 Claude Code CLI](#vs-code-extension-vs-claude-code-cli)。
</Tip>

<h2 id="install-the-extension">
  安裝擴充功能
</h2>

點擊您的 IDE 的連結以直接安裝：

* [為 VS Code 安裝](vscode:extension/anthropic.claude-code)
* [為 Cursor 安裝](cursor:extension/anthropic.claude-code)

或在 VS Code 中，按 `Cmd+Shift+X`（Mac）或 `Ctrl+Shift+X`（Windows/Linux）開啟擴充功能檢視，搜尋「Claude Code」，然後點擊**安裝**。

擴充功能也會安裝在其他 VS Code 分支中，例如 Devin Desktop 或 Kiro。在編輯器的擴充功能檢視中搜尋「Claude Code」，或從 [Open VSX registry](https://open-vsx.org/extension/Anthropic/claude-code) 安裝。如果您的編輯器無法安裝擴充功能，請[安裝 CLI](/docs/zh-TW/quickstart) 並在其整合終端中執行 `claude`。CLI 可在任何終端中運作。

<Note>如果安裝後擴充功能未出現，請重新啟動 VS Code 或從命令面板執行「Developer: Reload Window」。</Note>

<h2 id="get-started">
  開始使用
</h2>

安裝後，您可以透過 VS Code 介面開始使用 Claude Code：

<Steps>
  <Step title="開啟 Claude Code 面板">
    在整個 VS Code 中，Spark 圖示表示 Claude Code：<img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Spark icon" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    開啟 Claude 的最快方式是點擊**編輯器工具列**（編輯器右上角）中的 Spark 圖示。當您開啟檔案時，該圖示才會出現。

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="VS Code 編輯器顯示編輯器工具列中的 Spark 圖示" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    開啟 Claude Code 的其他方式：

    * **活動列**：點擊左側邊欄中的 Spark 圖示以開啟工作階段清單。點擊任何工作階段以將其作為完整編輯器標籤開啟，或開始新的工作階段。此圖示在活動列中始終可見。
    * **命令面板**：`Cmd+Shift+P`（Mac）或 `Ctrl+Shift+P`（Windows/Linux），輸入「Claude Code」，然後選擇一個選項，例如「在新標籤中開啟」
    * **狀態列**：點擊視窗右下角的 **✱ Claude Code**。即使沒有開啟檔案，這也有效。

    您可以拖動 Claude 面板以在 VS Code 中的任何位置重新定位它。有關詳細資訊，請參閱[自訂您的工作流程](#customize-your-workflow)。
  </Step>

  <Step title="登入">
    首次開啟面板時，會出現登入畫面。點擊**登入**並在您的瀏覽器中完成授權。

    如果您稍後看到**未登入 · 請執行 /login**，擴充功能會自動重新開啟登入畫面。如果它沒有出現，請從命令面板使用**Developer: Reload Window**重新載入視窗。

    如果您在 shell 中設定了 `ANTHROPIC_API_KEY` 但仍然看到登入提示，VS Code 可能沒有繼承您的 shell 環境。使用 `code .` 從終端機啟動 VS Code，以便它繼承您的環境變數，或改為使用您的 Claude 帳戶登入。

    登入後，會出現**學習 Claude Code** 檢查清單。透過點擊**顯示給我**來完成每一項，或用 X 關閉它。若要稍後重新開啟它，請在 VS Code 設定中的擴充功能 → Claude Code 下取消勾選**隱藏入門**。
  </Step>

  <Step title="傳送提示">
    要求 Claude 幫助您的程式碼或檔案，無論是解釋某些內容的工作原理、除錯問題還是進行變更。

    <Tip>Claude 會自動看到您選擇的文字。按 `Option+K`（Mac）/ `Alt+K`（Windows/Linux）也可以在您的提示中插入 @-提及參考（如 `@file.ts#5-10`）。</Tip>

    以下是詢問檔案中特定行的範例：

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="VS Code 編輯器，在 Python 檔案中選擇了第 2-3 行，Claude Code 面板顯示關於這些行的問題，帶有 @-提及參考" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="審查變更">
    當 Claude 想要編輯檔案時，它會顯示原始內容和建議變更的並排比較，然後要求許可。您可以接受、拒絕或告訴 Claude 改為做什麼。如果您在接受前直接在差異檢視中編輯建議的內容，Claude 會被告知您修改了它，因此它不會假設檔案與其原始提案相符。

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code 顯示 Claude 建議變更的差異，以及詢問是否進行編輯的許可提示" width="3292" height="1876" data-path="images/vs-code-edits.png" />
  </Step>
</Steps>

有關您可以使用 Claude Code 做什麼的更多想法，請參閱[常見工作流程](/docs/zh-TW/common-workflows)。

<Tip>
  從命令面板執行'Claude Code: Open Walkthrough'以獲得基礎知識的引導式導覽。
</Tip>

<h2 id="use-the-prompt-box">
  使用提示框
</h2>

提示框支援多項功能：

* **許可模式**：點擊提示框底部的模式指示器以切換模式，或在 VS Code 設定中的 `claudeCode.initialPermissionMode` 下設定預設值。請參閱[許可模式](/docs/zh-TW/permission-modes#switch-permission-modes)以了解指示器提供的每種模式。
  * **Manual**：Claude 在檔案編輯和大多數 shell 命令前要求許可。
  * **Plan**：Claude 描述它將做什麼，並在進行變更前等待批准。VS Code 會自動將計畫作為完整 Markdown 文件開啟，您可以在其中添加內聯評論以在 Claude 開始前提供反饋。
  * **Edit automatically**：Claude 進行編輯而不詢問。
* **命令菜單**：點擊 `/` 或輸入 `/` 以開啟命令菜單。選項包括附加檔案、切換模型、切換擴展思考、查看計畫使用情況（`/usage`）以及啟動 [Remote Control](/docs/zh-TW/remote-control) 工作階段（`/remote-control`）。自訂部分提供對 MCP servers、hooks、memory、permissions 和 plugins 的存取。帶有終端機圖示的項目在整合終端機中開啟。
  * 設定部分包括**為所有工作階段啟用 Remote Control**，它設定 [`remoteControlAtStartup`](/docs/zh-TW/settings#available-settings)，以便[每個新的互動工作階段都自動連接到 Remote Control](/docs/zh-TW/remote-control#enable-remote-control-for-all-sessions)。需要 Claude Code v2.1.203 或更新版本。
* **上下文指示器**：提示框顯示您使用了多少 Claude 的 context window。Claude 在需要時會自動壓縮，或您可以手動執行 `/compact`。
* **擴展思考**：讓 Claude 花更多時間推理複雜問題。透過命令菜單（`/`）切換它。Claude 的推理在對話中顯示為摺疊的區塊：點擊一個區塊以讀取它，或按 `Ctrl+O` 以展開或摺疊工作階段中的每個思考區塊。有關詳細資訊，請參閱 [Extended thinking](/docs/zh-TW/model-config#extended-thinking)。
* **多行輸入**：按 `Shift+Enter` 以添加新行而不傳送。這也適用於問題對話框的「其他」自由文字輸入。

<h3 id="reference-files-and-folders">
  參考檔案和資料夾
</h3>

使用 @-提及為 Claude 提供有關特定檔案或資料夾的上下文。當您輸入 `@` 後跟檔案或資料夾名稱時，Claude 會讀取該內容，並可以回答有關它的問題或對其進行變更。Claude Code 支援模糊匹配，因此您可以輸入部分名稱來找到您需要的內容：

```text theme={null}
> Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
> What's in @src/components/ (include a trailing slash for folders)
```

對於大型 PDF，您可以要求 Claude 讀取特定頁面而不是整個檔案：單一頁面、範圍（如第 1-10 頁）或開放式範圍（如第 3 頁起）。

當您在編輯器中選擇文字時，Claude 可以自動看到您突出顯示的程式碼。提示框頁腳顯示選擇了多少行。按 `Option+K`（Mac）/ `Alt+K`（Windows/Linux）以插入帶有檔案路徑和行號的 @-提及（例如 `@app.ts#5-10`）。點擊選擇指示器以切換 Claude 是否可以看到您突出顯示的文字 - 眼睛斜線圖示表示選擇對 Claude 隱藏。

您也可以在將檔案拖動到提示框時按住 `Shift` 以將它們添加為附件。點擊任何附件上的 X 以將其從上下文中移除。

<h3 id="resume-past-conversations">
  恢復過去的對話
</h3>

點擊 Claude Code 面板頂部的**工作階段歷史記錄**按鈕以存取您的對話歷史記錄。您可以按關鍵字搜尋或按時間瀏覽（今天、昨天、過去 7 天等）。點擊任何對話以使用完整訊息歷史記錄恢復它。新工作階段會根據您的第一條訊息接收 AI 生成的標題。將滑鼠懸停在工作階段上以顯示重新命名和移除操作：重新命名以給它一個描述性標題，或移除以將其從清單中刪除。有關恢復工作階段的更多資訊，請參閱 [Manage sessions](/docs/zh-TW/sessions)。

<h3 id="resume-cloud-sessions-from-claude-ai">
  從 Claude.ai 恢復遠端工作階段
</h3>

如果您使用[網路上的 Claude Code](/docs/zh-TW/claude-code-on-the-web)，您可以直接在 VS Code 中恢復這些遠端工作階段。這需要使用 **Claude.ai Subscription** 登入，而不是 Anthropic Console。

<Steps>
  <Step title="開啟工作階段歷史記錄">
    點擊 Claude Code 面板頂部的**工作階段歷史記錄**按鈕。
  </Step>

  <Step title="選擇遠端標籤">
    對話框顯示兩個標籤：本機和遠端。點擊**遠端**以查看來自 claude.ai 的工作階段。
  </Step>

  <Step title="選擇要恢復的工作階段">
    瀏覽或搜尋您的遠端工作階段。點擊任何工作階段以下載它並在本機繼續對話。
  </Step>
</Steps>

<Note>
  只有使用 GitHub 儲存庫啟動的網路工作階段才會出現在遠端標籤中。恢復會在本機載入對話歷史記錄；變更不會同步回 claude.ai。
</Note>

<h3 id="check-account-and-usage">
  檢查帳戶和使用情況
</h3>

從命令菜單執行 `/usage` 以開啟「帳戶與使用情況」對話框。它顯示您登入的帳戶、方案，以及目前工作階段和本週的使用情況列，包括每個限制重設的時間。

該對話框也會分解對您的方案限制有貢獻的內容。它會標記佔最近使用情況 10% 或以上的行為，例如快取未命中、長上下文和子代理程式密集或高度平行的工作階段，每個都有減少它的提示。歸因表顯示每個技能、子代理程式、外掛程式和 MCP server 貢獻了多少使用情況。需要 Claude Code v2.1.174 或更新版本。

使用「日」和「週」切換以在過去 24 小時和過去 7 天之間切換。這些數字是近似值，並從此機器上的本機工作階段計算，因此不包括來自其他裝置或 claude.ai 的使用情況。有關追蹤和減少使用情況的更多資訊，請參閱 [Track your costs](/docs/zh-TW/costs#track-your-costs)。

<h2 id="customize-your-workflow">
  自訂您的工作流程
</h2>

一旦您啟動並執行，您可以重新定位 Claude 面板、執行多個工作階段或切換到終端機模式。

<h3 id="choose-where-claude-lives">
  選擇 Claude 的位置
</h3>

您可以拖動 Claude 面板以在 VS Code 中的任何位置重新定位它。抓住面板的標籤或標題列並拖動到：

* **次要邊欄**：視窗的右側。在您編碼時保持 Claude 可見。
* **主要邊欄**：左側邊欄，帶有資源管理器、搜尋等圖示。
* **編輯器區域**：將 Claude 作為標籤與您的檔案一起開啟。適用於側面任務。

<Tip>
  將邊欄用於您的主要 Claude 工作階段，並為側面任務開啟其他標籤。Claude 會記住您偏好的位置。活動列工作階段清單圖示與 Claude 面板分開：工作階段清單在活動列中始終可見，而 Claude 面板圖示只有在面板停靠到左側邊欄時才會出現在那裡。
</Tip>

<h3 id="run-multiple-conversations">
  執行多個對話
</h3>

使用命令面板中的**在新標籤中開啟**或**在新視窗中開啟**以開始其他對話。每個對話維護自己的歷史記錄和上下文，允許您並行處理不同的任務。

使用標籤時，spark 圖示上的小彩色點表示狀態：藍色表示許可請求待處理，橙色表示 Claude 在標籤隱藏時完成。

<h3 id="switch-to-terminal-mode">
  切換到終端機模式
</h3>

預設情況下，擴充功能開啟圖形聊天面板。如果您偏好 CLI 風格的介面，請開啟[使用終端機設定](vscode://settings/claudeCode.useTerminal)並勾選該框。

您也可以開啟 VS Code 設定（Mac 上的 `Cmd+,` 或 Windows/Linux 上的 `Ctrl+,`），前往「擴充功能」→「Claude Code」，然後勾選**使用終端機**。

<h2 id="manage-plugins">
  管理 plugins
</h2>

VS Code 擴充功能包含用於安裝和管理 [plugins](/docs/zh-TW/plugins) 的圖形介面。在提示框中輸入 `/plugins` 以開啟**管理 plugins** 介面。

<h3 id="install-plugins">
  安裝 plugins
</h3>

plugin 對話框顯示兩個標籤：**Plugins** 和 **Marketplaces**。

在 Plugins 標籤中：

* **已安裝的 plugins** 出現在頂部，帶有切換開關以啟用或停用它們
* **來自您配置的市場的可用 plugins** 出現在下方
* 搜尋以按名稱或描述篩選 plugins
* 點擊任何可用 plugin 上的**安裝**

當您安裝 plugin 時，選擇安裝範圍：

* **為您安裝**：在您的所有專案中可用（使用者範圍）
* **為此專案安裝**：與專案協作者共享（專案範圍）
* **在本機安裝**：僅適用於您，僅在此儲存庫中（本機範圍）

<h3 id="manage-marketplaces">
  管理市場
</h3>

切換到 **Marketplaces** 標籤以添加或移除 plugin 來源：

* 輸入 GitHub 儲存庫、URL 或本機路徑以添加新市場
* 點擊重新整理圖示以更新市場的 plugin 清單
* 點擊垃圾桶圖示以移除市場

進行變更後，橫幅會提示您重新啟動 Claude Code 以應用更新。

<Note>
  VS Code 中的 plugin 管理在幕後使用相同的 CLI 命令。您在擴充功能中配置的 plugins 和市場也可在 CLI 中使用，反之亦然。
</Note>

有關 plugin 系統的更多資訊，請參閱 [Plugins](/docs/zh-TW/plugins) 和 [Plugin marketplaces](/docs/zh-TW/plugin-marketplaces)。

<h2 id="automate-browser-tasks-with-chrome">
  使用 Chrome 自動化瀏覽器任務
</h2>

將 Claude 連接到您的 Chrome 瀏覽器以測試網路應用程式、使用主控台日誌進行除錯，以及在不離開 VS Code 的情況下自動化瀏覽器工作流程。這需要 [Claude in Chrome extension](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) 版本 1.0.36 或更高版本。

在提示框中輸入 `@browser` 後跟您想要 Claude 做的事情：

```text theme={null}
@browser go to localhost:3000 and check the console for errors
```

您也可以開啟附件菜單以選擇特定的瀏覽器工具，例如開啟新標籤或讀取頁面內容。

Claude 為瀏覽器任務開啟新標籤並共享您的瀏覽器登入狀態，因此它可以存取您已登入的任何網站。

有關設定說明、完整功能清單和故障排除，請參閱[使用 Claude Code 與 Chrome](/docs/zh-TW/chrome)。

<h2 id="vs-code-commands-and-shortcuts">
  VS Code 命令和快捷鍵
</h2>

開啟命令面板（Mac 上的 `Cmd+Shift+P` 或 Windows/Linux 上的 `Ctrl+Shift+P`）並輸入「Claude Code」以查看 Claude Code 擴充功能的所有可用 VS Code 命令。

某些快捷鍵取決於哪個面板「獲得焦點」（接收鍵盤輸入）。當您的游標在程式碼檔案中時，編輯器獲得焦點。當您的游標在 Claude 的提示框中時，Claude 獲得焦點。使用 `Cmd+Esc` / `Ctrl+Esc` 在它們之間切換。

<Note>
  這些是用於控制擴充功能的 VS Code 命令。並非所有內建 Claude Code 命令都在擴充功能中可用。有關詳細資訊，請參閱 [VS Code 擴充功能與 Claude Code CLI](#vs-code-extension-vs-claude-code-cli)。
</Note>

| 命令                         | 快捷鍵                                                   | 描述                                                                                                                  |
| -------------------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc`（Mac）/ `Ctrl+Esc`（Windows/Linux）             | 在編輯器和 Claude 之間切換焦點                                                                                                 |
| Open in Side Bar           | -                                                     | 在左側邊欄中開啟 Claude                                                                                                     |
| Open in Terminal           | -                                                     | 在終端機模式中開啟 Claude                                                                                                    |
| Open in New Tab            | `Cmd+Shift+Esc`（Mac）/ `Ctrl+Shift+Esc`（Windows/Linux） | 將新對話作為編輯器標籤開啟                                                                                                       |
| Open in New Window         | -                                                     | 在單獨的視窗中開啟新對話                                                                                                        |
| New Conversation           | `Cmd+N`（Mac）/ `Ctrl+N`（Windows/Linux）                 | 開始新對話。需要 Claude 獲得焦點且 `enableNewConversationShortcut` 設定為 `true`                                                    |
| Reopen Closed Session      | `Cmd+Shift+T`（Mac）/ `Ctrl+Shift+T`（Windows/Linux）     | 重新開啟最近關閉的 Claude 工作階段標籤。當最後關閉的標籤不是 Claude 工作階段時，會回退到 VS Code 的正常重新開啟關閉編輯器。使用 `enableReopenClosedSessionShortcut` 停用 |
| Insert @-Mention Reference | `Option+K`（Mac）/ `Alt+K`（Windows/Linux）               | 插入對目前檔案和選擇的參考（需要編輯器獲得焦點）                                                                                            |
| Show Logs                  | -                                                     | 檢視擴充功能除錯日誌                                                                                                          |
| Logout                     | -                                                     | 登出您的 Anthropic 帳戶                                                                                                   |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  從其他工具啟動 VS Code 標籤
</h3>

擴充功能在 `vscode://anthropic.claude-code/open` 註冊 URI 處理程式。使用它從您自己的工具開啟新的 Claude Code 標籤：shell 別名、瀏覽器書籤，或任何可以開啟 URL 的指令碼。如果 VS Code 尚未執行，開啟 URL 會先啟動它。如果 VS Code 已在執行，URL 會在目前獲得焦點的視窗中開啟。

使用您的作業系統的 URL 開啟程式叫用處理程式。

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Windows">
    在 PowerShell 中：

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    在 `cmd.exe` 中，`start` 將其第一個引號引數視為視窗標題，因此在 URL 之前傳遞空標題：

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

處理程式接受兩個選擇性查詢參數：

| 參數        | 描述                                                                                                                                                                |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | 要在提示框中預先填入的文字。必須進行 URL 編碼。提示會預先填入但不會自動提交。                                                                                                                         |
| `session` | 要恢復的工作階段 ID，而不是開始新對話。工作階段必須屬於目前在 VS Code 中開啟的工作區。如果找不到工作階段，會改為開始新的對話。如果工作階段已在標籤中開啟，該標籤會獲得焦點。若要以程式設計方式擷取工作階段 ID，請參閱[繼續對話](/docs/zh-TW/headless#continue-conversations)。 |

例如，若要開啟預先填入「review my changes」的標籤：

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

若要啟動終端機工作階段而不是 VS Code 標籤，請使用 CLI 的 `claude-cli://` 處理程式。請參閱[從連結啟動工作階段](/docs/zh-TW/deep-links)。

<h2 id="configure-settings">
  配置設定
</h2>

擴充功能有兩種類型的設定：

* **VS Code 中的擴充功能設定**：控制擴充功能在 VS Code 中的行為。使用 `Cmd+,`（Mac）或 `Ctrl+,`（Windows/Linux）開啟，然後前往「擴充功能」→「Claude Code」。您也可以輸入 `/` 並選擇**一般配置**以開啟設定。
* **`~/.claude/settings.json` 中的 Claude Code 設定**：在擴充功能和 CLI 之間共享。用於允許的命令、環境變數、hooks 和 MCP servers。有關詳細資訊，請參閱[設定](/docs/zh-TW/settings)。

<Tip>
  將 `"$schema": "https://json.schemastore.org/claude-code-settings.json"` 添加到您的 `settings.json` 以在 VS Code 中直接獲得所有可用設定的自動完成和內聯驗證。
</Tip>

<h3 id="extension-settings">
  擴充功能設定
</h3>

| 設定                                  | 預設值       | 描述                                                                                                                                                                                                           |
| ----------------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `useTerminal`                       | `false`   | 以終端機模式而不是圖形面板啟動 Claude                                                                                                                                                                                       |
| `initialPermissionMode`             | `default` | 控制新對話的批准提示：`default`、`plan`、`acceptEdits` 或 `bypassPermissions`。`manual` 是 `default` 的別名，並選擇模式指示器中標記為**手動**的模式。需要 Claude Code v2.1.200 或更新版本。請參閱[許可模式](/docs/zh-TW/permission-modes)。                             |
| `preferredLocation`                 | `panel`   | Claude 開啟的位置：`sidebar`（右側）或 `panel`（新標籤）                                                                                                                                                                     |
| `autosave`                          | `true`    | Claude 讀取或寫入檔案前自動儲存檔案                                                                                                                                                                                        |
| `useCtrlEnterToSend`                | `false`   | 使用 Ctrl/Cmd+Enter 而不是 Enter 來傳送提示                                                                                                                                                                            |
| `enableNewConversationShortcut`     | `false`   | 啟用 Cmd/Ctrl+N 以開始新對話                                                                                                                                                                                         |
| `enableReopenClosedSessionShortcut` | `true`    | 使用 Cmd/Ctrl+Shift+T 重新開啟最近關閉的 Claude 工作階段標籤。當最後關閉的標籤不是 Claude 工作階段時，快捷鍵會改為執行 VS Code 的正常重新開啟已關閉編輯器命令。                                                                                                        |
| `hideOnboarding`                    | `false`   | 隱藏入門檢查清單（畢業帽圖示）                                                                                                                                                                                              |
| `respectGitIgnore`                  | `true`    | 從檔案搜尋中排除 .gitignore 模式                                                                                                                                                                                       |
| `usePythonEnvironment`              | `true`    | 執行 Claude 時啟動工作區的 Python 環境。需要 Python 擴充功能。                                                                                                                                                                  |
| `environmentVariables`              | `[]`      | 為 Claude 程序設定環境變數。改為使用 Claude Code 設定以進行共享配置。                                                                                                                                                                |
| `disableLoginPrompt`                | `false`   | 跳過身份驗證提示（用於第三方提供者設定）                                                                                                                                                                                         |
| `allowDangerouslySkipPermissions`   | `false`   | 將 Bypass permissions 添加到模式選擇器。僅在沒有網際網路存取的 sandbox 中使用。                                                                                                                                                       |
| `claudeProcessWrapper`              | -         | 用於啟動 Claude 程序的可執行檔。當存在時，捆綁的二進制檔案路徑會作為引數傳遞。如果擴充功能組建不包含您平台的二進制檔案，請將此設定為單獨安裝的 `claude` 二進制檔案。如果擴充功能組建不包含您平台的二進制檔案，請參閱[哪些平台有預先建置的二進制檔案](/docs/zh-TW/troubleshoot-install#native-binary-not-found-after-npm-install)。 |

<h2 id="vs-code-extension-vs-claude-code-cli">
  VS Code 擴充功能與 Claude Code CLI
</h2>

Claude Code 既可作為 VS Code 擴充功能（圖形面板）也可作為 CLI（終端機中的命令列介面）使用。某些功能僅在 CLI 中可用。如果您需要 CLI 專用功能，請在 VS Code 的整合終端機中執行 `claude`。這需要[獨立 CLI 安裝](/docs/zh-TW/setup)：擴充功能不會將 `claude` 新增到您的 PATH。請參閱[在 VS Code 中執行 CLI](#run-cli-in-vs-code)。

| 功能            | CLI                   | VS Code 擴充功能                             |
| ------------- | --------------------- | ---------------------------------------- |
| 命令和 skills    | [全部](/docs/zh-TW/commands) | 子集（輸入 `/` 以查看可用的）                        |
| MCP server 配置 | 是                     | 部分（透過 CLI 添加伺服器；使用聊天面板中的 `/mcp` 管理現有伺服器） |
| Checkpoints   | 是                     | 是                                        |
| `!` bash 快捷方式 | 是                     | 否                                        |
| Tab 完成        | 是                     | 否                                        |

<h3 id="rewind-with-checkpoints">
  使用 checkpoints 進行倒帶
</h3>

VS Code 擴充功能支援 checkpoints，它們追蹤 Claude 的檔案編輯並讓您倒帶到先前的狀態。將滑鼠懸停在任何訊息上以顯示倒帶按鈕，然後從三個選項中選擇：

* **從此處分支對話**：從此訊息開始新的對話分支，同時保持所有程式碼變更完整
* **將程式碼倒帶到此處**：將檔案變更還原回對話中的此點，同時保持完整的對話歷史記錄
* **分支對話並倒帶程式碼**：開始新的對話分支並將檔案變更還原到此點

有關 checkpoints 如何工作及其限制的完整詳細資訊，請參閱 [Checkpointing](/docs/zh-TW/checkpointing)。

<h3 id="run-cli-in-vs-code">
  在 VS Code 中執行 CLI
</h3>

若要在 VS Code 中使用 CLI，請開啟整合終端機（Windows/Linux 上的 `` Ctrl+` `` 或 Mac 上的 `` Cmd+` ``）並執行 `claude`。CLI 會自動與您的 IDE 整合，以獲得差異檢視和診斷共享等功能。

安裝擴充功能不會將 `claude` 放在您的 shell PATH 上。擴充功能為其聊天面板捆綁了 CLI 的私有副本，但在終端機中輸入 `claude` 需要[獨立 CLI 安裝](/docs/zh-TW/setup)。執行一次安裝，此頁面上的命令（包括 `claude mcp add` 和 `claude --resume`）在任何終端機中都可以運作。如果安裝後仍未找到 `claude`，請[驗證您的 PATH](/docs/zh-TW/troubleshoot-install#verify-your-path)。

如果使用外部終端機，請在 Claude Code 中執行 `/ide` 以將其連接到 VS Code。

<h3 id="switch-between-extension-and-cli">
  在擴充功能和 CLI 之間切換
</h3>

擴充功能和 CLI 共享相同的對話歷史記錄。若要在 CLI 中繼續擴充功能對話，請在終端機中執行 `claude --resume`。這會開啟一個互動式選擇器，您可以在其中搜尋並選擇您的對話。

<h3 id="include-terminal-output-in-prompts">
  在提示中包含終端機輸出
</h3>

使用 `@terminal:name` 在您的提示中參考終端機輸出，其中 `name` 是終端機的標題。這讓 Claude 可以看到命令輸出、錯誤訊息或日誌，而無需複製貼上。

<h3 id="monitor-background-processes">
  監控背景程序
</h3>

當 Claude 執行長時間執行的命令時，擴充功能在狀態列中顯示進度。但是，與 CLI 相比，背景任務的可見性受限。為了獲得更好的可見性，讓 Claude 輸出命令，以便您可以在 VS Code 的整合終端機中執行它。

<h3 id="connect-to-external-tools-with-mcp">
  使用 MCP 連接到外部工具
</h3>

MCP（Model Context Protocol）servers 讓 Claude 存取外部工具、資料庫和 API。

若要添加 MCP server，請開啟整合終端機（`` Ctrl+` `` 或 `` Cmd+` ``）並執行 `claude mcp add`。下面的範例添加了 GitHub 的遠端 MCP server，它使用作為標頭傳遞的[個人存取令牌](https://github.com/settings/personal-access-tokens)進行身份驗證：

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

配置後，要求 Claude 使用工具（例如「Review PR #456」）。

若要在不離開 VS Code 的情況下管理 MCP servers，請在聊天面板中輸入 `/mcp`。MCP 管理對話框讓您啟用或停用伺服器、重新連接到伺服器以及管理 OAuth 身份驗證。有關可用伺服器，請參閱 [MCP 文件](/docs/zh-TW/mcp)。

<h2 id="work-with-git">
  使用 git
</h2>

Claude Code 與 git 整合以幫助直接在 VS Code 中進行版本控制工作流程。要求 Claude 提交變更、建立拉取請求或跨分支工作。

<h3 id="create-commits-and-pull-requests">
  建立提交和拉取請求
</h3>

Claude 可以暫存變更、編寫提交訊息並根據您的工作建立拉取請求：

```text theme={null}
> commit my changes with a descriptive message
> create a pr for this feature
> summarize the changes I've made to the auth module
```

建立拉取請求時，Claude 會根據實際程式碼變更生成描述，並可以添加有關測試或實現決策的上下文。

<h3 id="use-git-worktrees-for-parallel-tasks">
  使用 git worktrees 進行並行任務
</h3>

使用 `--worktree`（`-w`）標誌以在具有自己的檔案和分支的隔離 worktree 中啟動 Claude：

```bash theme={null}
claude --worktree feature-auth
```

每個 worktree 維護獨立的檔案狀態，同時共享 git 歷史記錄。這可防止 Claude 實例在處理不同任務時相互干擾。有關更多詳細資訊，請參閱[使用 Git worktrees 執行並行工作階段](/docs/zh-TW/worktrees)。

<h2 id="use-third-party-providers">
  使用第三方提供者
</h2>

預設情況下，Claude Code 直接連接到 Anthropic 的 API。如果您的組織使用 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 來存取 Claude，請配置擴充功能以改為使用您的提供者：

<Steps>
  <Step title="停用登入提示">
    開啟[停用登入提示設定](vscode://settings/claudeCode.disableLoginPrompt)並勾選該框。

    您也可以開啟 VS Code 設定（Mac 上的 `Cmd+,` 或 Windows/Linux 上的 `Ctrl+,`），搜尋'Claude Code login'，然後勾選**停用登入提示**。
  </Step>

  <Step title="配置您的提供者">
    遵循您的提供者的設定指南：

    * [Amazon Bedrock 上的 Claude Code](/docs/zh-TW/amazon-bedrock)
    * [Google Cloud 的 Agent Platform 上的 Claude Code](/docs/zh-TW/google-vertex-ai)
    * [Microsoft Foundry 上的 Claude Code](/docs/zh-TW/microsoft-foundry)

    這些指南涵蓋在 `~/.claude/settings.json` 中配置您的提供者，這確保您的設定在 VS Code 擴充功能和 CLI 之間共享。
  </Step>
</Steps>

<h2 id="security-and-privacy">
  安全和隱私
</h2>

您的程式碼保持私密。Claude Code 處理您的程式碼以提供協助，但不使用它來訓練模型。有關資料處理和如何選擇退出日誌記錄的詳細資訊，請參閱[資料和隱私](/docs/zh-TW/data-usage)。

啟用自動編輯許可後，Claude Code 可以修改 VS Code 配置檔案（如 `settings.json` 或 `tasks.json`），VS Code 可能會自動執行。為了在處理不受信任的程式碼時降低風險：

* 為不受信任的工作區啟用 [VS Code 受限模式](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode)
* 使用手動批准模式而不是自動接受進行編輯
* 在接受變更前仔細審查它們

<h3 id="the-built-in-ide-mcp-server">
  內建 IDE MCP server
</h3>

當擴充功能處於活動狀態時，它會執行一個本機 MCP server，CLI 會自動連接到該伺服器。這是 CLI 在 VS Code 的原生差異檢視器中開啟差異、讀取您目前的選擇以進行 `@`-提及，以及 — 當您在 Jupyter notebook 中工作時 — 要求 VS Code 執行儲存格的方式。

伺服器名為 `ide`，從 `/mcp` 隱藏，因為沒有什麼可配置的。但是，如果您的組織使用 `PreToolUse` hook 來允許列出 MCP 工具，您需要知道它存在。

**選擇和開啟檔案的內容。** 連接時，CLI 會在您傳送的每個提示上包含您目前的編輯器選擇和活動檔案的路徑作為內容。當發生這種情況時，文字記錄會顯示 `⧉ Selected N lines from <file>` 行。若要排除敏感檔案（如 `.env`），請為其路徑新增 [`Read` 拒絕規則](/docs/zh-TW/permissions#read-and-edit)。匹配的拒絕規則會防止該檔案的選定文字和開啟檔案通知到達 Claude。

**傳輸和身份驗證。** 伺服器綁定到 `127.0.0.1` 上的隨機高埠，埠號範圍為 10000–65535，且埠號不可配置。傳輸是未加密的 `ws://`；因為通訊端是環回專用的，任何可以捕獲流量的程序也可以從鎖定檔案讀取令牌，所以 TLS 不會增加保護。每次擴充功能啟動都會生成一個新的隨機身份驗證令牌，將其寫入 `~/.claude/ide/<port>.lock` 的鎖定檔案，CLI 必須將其作為 `X-Claude-Code-Ide-Authorization` 標頭提供才能連接。鎖定檔案在 `0700` 目錄中具有 `0600` 權限，因此只有執行 VS Code 的使用者可以讀取它。如果設定了 `CLAUDE_CONFIG_DIR`，鎖定檔案會改為寫入 `$CLAUDE_CONFIG_DIR/ide/`。

**向模型公開的工具。** 伺服器託管十幾個工具，但只有兩個對模型可見。其餘的是 CLI 用於自己的 UI 的內部 RPC — 開啟差異、讀取選擇、儲存檔案 — 在工具清單到達 Claude 之前被篩選出來。

| 工具名稱（如 hooks 所見）           | 它的作用                                               | 唯讀 |
| -------------------------- | -------------------------------------------------- | -- |
| `mcp__ide__getDiagnostics` | 返回語言伺服器診斷 — VS Code 的「問題」面板中的錯誤和警告。可選地限定於一個檔案。     | 是  |
| `mcp__ide__executeCode`    | 在活動 Jupyter notebook 的核心中執行 Python 程式碼。請參閱下面的確認流程。 | 否  |

**Jupyter 執行始終先詢問。** `mcp__ide__executeCode` 無法以靜默方式執行任何內容。在每次呼叫時，程式碼會作為新儲存格插入到活動 notebook 的末尾，VS Code 會將其滾動到檢視中，原生 Quick Pick 會要求您**執行**或**取消**。取消 — 或使用 `Esc` 關閉選擇器 — 會向 Claude 返回錯誤，沒有任何內容執行。當沒有活動 notebook、未安裝 Jupyter 擴充功能（`ms-toolsai.jupyter`）或核心不是 Python 時，該工具也會直接拒絕。

<Note>
  Quick Pick 確認與 `PreToolUse` hooks 分開。`mcp__ide__executeCode` 的允許列表條目讓 Claude *提議*執行儲存格；VS Code 內的 Quick Pick 是讓它*實際*執行的原因。
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  修復常見問題
</h2>

<h3 id="extension-won’t-install">
  擴充功能無法安裝
</h3>

* 確保您有相容的 VS Code 版本（1.98.0 或更高版本）
* 檢查 VS Code 是否有權限安裝擴充功能
* 嘗試從 [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code) 直接安裝

<h3 id="spark-icon-not-visible">
  Spark 圖示不可見
</h3>

當您開啟檔案時，Spark 圖示會出現在**編輯器工具列**（編輯器右上角）中。如果您看不到它：

1. **開啟檔案**：該圖示需要開啟檔案。僅開啟資料夾是不夠的。
2. **檢查 VS Code 版本**：需要 1.98.0 或更高版本（幫助 → 關於）
3. **重新啟動 VS Code**：從命令面板執行「Developer: Reload Window」
4. **停用衝突的擴充功能**：暫時停用其他 AI 擴充功能（Cline、Continue 等）
5. **檢查工作區信任**：擴充功能在受限模式下不工作

或者，點擊**狀態列**（右下角）中的「✱ Claude Code」。即使沒有開啟檔案，這也有效。您也可以使用**命令面板**（`Cmd+Shift+P` / `Ctrl+Shift+P`）並輸入「Claude Code」。

<h3 id="cmd-esc-does-nothing-on-macos">
  macOS 上的 Cmd+Esc 無法執行任何操作
</h3>

在 macOS Tahoe 及更新版本上，系統遊戲覆蓋快捷鍵預設綁定到 `Cmd+Esc`，並在按鍵到達 VS Code 之前攔截它。若要釋放快捷鍵：

1. 開啟系統設定
2. 前往鍵盤，然後鍵盤快捷鍵，然後遊戲控制器
3. 清除遊戲覆蓋核取方塊

或者，將擴充功能重新綁定到不同的鍵：開啟 VS Code [鍵盤快捷鍵編輯器](https://code.visualstudio.com/docs/configure/keybindings)（`Cmd+K Cmd+S`），搜尋 `Claude Code: Focus input`，並指派新的綁定。

<h3 id="claude-code-never-responds">
  Claude Code 從不回應
</h3>

如果 Claude Code 沒有回應您的提示：

1. **檢查您的網際網路連接**：確保您有穩定的網際網路連接
2. **開始新對話**：嘗試開始新的對話以查看問題是否持續
3. **嘗試 CLI**：從終端機執行 `claude` 以查看您是否獲得更詳細的錯誤訊息

如果問題持續，請[在 GitHub 上提交問題](https://github.com/anthropics/claude-code/issues)，並提供有關錯誤的詳細資訊。

<h2 id="uninstall-the-extension">
  卸載擴充功能
</h2>

若要卸載 Claude Code 擴充功能：

1. 開啟擴充功能檢視（Mac 上的 `Cmd+Shift+X` 或 Windows/Linux 上的 `Ctrl+Shift+X`）
2. 搜尋「Claude Code」
3. 點擊**卸載**

在 VS Code 整合終端中執行 `claude` 會自動重新安裝擴充功能。若要保持卸載狀態，請在 `/config` 中關閉**自動安裝 IDE 擴充功能**，或將 [`autoInstallIdeExtension`](/docs/zh-TW/settings#global-config-settings) 設定為 `false`。您也可以將 [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/zh-TW/env-vars) 環境變數設定為 `1`。

若要也移除擴充功能資料並重設所有設定，請刪除您平台的擴充功能儲存目錄。

在 macOS 上：

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

在 Linux 上：

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

在 Windows 上，在 PowerShell 中：

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

如需其他幫助，請參閱[故障排除指南](/docs/zh-TW/troubleshooting)。

<h2 id="next-steps">
  後續步驟
</h2>

現在您已在 VS Code 中設定了 Claude Code：

* [探索常見工作流程](/docs/zh-TW/common-workflows)以充分利用 Claude Code
* [設定 MCP servers](/docs/zh-TW/mcp) 以使用外部工具擴展 Claude 的功能。使用 CLI 添加伺服器，然後使用聊天面板中的 `/mcp` 管理它們。
* [配置 Claude Code 設定](/docs/zh-TW/settings)以自訂允許的命令、hooks 等。這些設定在擴充功能和 CLI 之間共享。
