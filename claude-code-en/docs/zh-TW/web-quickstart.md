> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在網頁上開始使用 Claude Code

> 在雲端從瀏覽器或手機執行 Claude Code。連接 GitHub 儲存庫、提交任務，並在無需本地設定的情況下檢查 PR。

<Note>
  Claude Code on the web 目前處於研究預覽階段，適用於 Pro、Max 和 Team 用戶，以及擁有高級席位或 Chat + Claude Code 席位的企業用戶。
</Note>

Claude Code on the web 在 Anthropic 管理的雲端基礎設施上執行，而不是在您的機器上。從瀏覽器或 Claude 行動應用程式的 [claude.ai/code](https://claude.ai/code) 提交任務。

您需要一個 GitHub 儲存庫來[開始使用](#connect-github-and-create-an-environment)。Claude 將其複製到隔離的虛擬機器中、進行更改，並為您推送一個分支以供檢查。會話在設備間持續存在，因此您在筆記型電腦上開始的任務可以稍後從手機上檢查。

Claude Code on the web 適用於：

* **並行任務**：同時執行多個獨立任務，每個任務在自己的會話和分支中，無需管理多個 worktrees
* **您本地沒有的儲存庫**：Claude 在每個會話中新鮮複製儲存庫，因此您無需簽出它
* **不需要頻繁引導的任務**：提交一個定義明確的任務，做其他事情，並在 Claude 完成時檢查結果
* **代碼問題和探索**：理解代碼庫或追蹤功能如何實現，無需本地簽出

對於需要您本地配置、工具或環境的工作，在本地執行 Claude Code 或使用 [Remote Control](/docs/zh-TW/remote-control) 更合適。

<h2 id="how-sessions-run">
  會話如何執行
</h2>

當您提交任務時：

1. **複製和準備**：您的儲存庫被複製到 Anthropic 管理的 VM，並且您的[設定指令碼](/docs/zh-TW/claude-code-on-the-web#setup-scripts)會在配置時執行。
2. **配置網路**：根據您環境的[存取級別](/docs/zh-TW/claude-code-on-the-web#access-levels)設定網際網路存取。
3. **工作**：Claude 分析代碼、進行更改、執行測試並檢查其工作。您可以全程觀看和引導，或者離開並在完成時返回。
4. **推送分支**：當 Claude 達到停止點時，它會將其分支推送到 GitHub。您檢查差異、留下內聯評論、建立 PR 或發送另一條訊息以繼續。

推送分支時會話不會關閉。PR 建立和進一步編輯都在同一對話中進行。

<h2 id="compare-ways-to-run-claude-code">
  比較執行 Claude Code 的方式
</h2>

Claude Code 在任何地方的行為都相同。改變的是代碼執行的位置以及您的本地配置是否可用。Desktop 應用程式提供本地和雲端會話，因此其下面的答案取決於您選擇的是哪一個：

|                                     | 在網頁上                                                                                            | Remote Control    | Terminal CLI | Desktop 應用程式 |
| :---------------------------------- | :---------------------------------------------------------------------------------------------- | :---------------- | :----------- | :----------- |
| **代碼執行於**                           | Anthropic 雲端 VM                                                                                 | 您的機器              | 您的機器         | 您的機器或雲端 VM   |
| **您從以下位置聊天**                        | claude.ai 或行動應用程式                                                                               | claude.ai 或行動應用程式 | 您的終端         | Desktop UI   |
| **使用您的本地配置**                        | 否，僅儲存庫                                                                                          | 是                 | 是            | 本地為是，雲端為否    |
| **需要 GitHub**                       | 是，或透過 `--cloud` [捆綁本地儲存庫](/docs/zh-TW/claude-code-on-the-web#send-local-repositories-without-github) | 否                 | 否            | 僅限雲端會話       |
| **如果您斷開連接，保持執行**                    | 是                                                                                               | 終端保持開啟時           | 否            | 取決於會話類型      |
| **[權限模式](/docs/zh-TW/permission-modes)** | 接受編輯、Plan、Auto                                                                                  | 詢問、自動接受編輯、Plan    | 所有模式         | 取決於會話類型      |
| **網路存取**                            | 每個環境可配置                                                                                         | 您機器的網路            | 您機器的網路       | 取決於會話類型      |

請參閱 [terminal quickstart](/docs/zh-TW/quickstart)、[Desktop 應用程式](/docs/zh-TW/desktop) 或 [Remote Control](/docs/zh-TW/remote-control) 文件以設定這些。

<h2 id="connect-github-and-create-an-environment">
  連接 GitHub 並建立環境
</h2>

設定是一次性過程。如果您已經使用 GitHub CLI，您可以[從您的終端執行此操作](#connect-from-your-terminal)，而不是使用瀏覽器。

<Steps>
  <Step title="訪問 claude.ai/code">
    前往 [claude.ai/code](https://claude.ai/code) 並使用您的 Anthropic 帳戶登入。
  </Step>

  <Step title="安裝 Claude GitHub App">
    登入後，claude.ai/code 會提示您連接 GitHub。按照提示安裝 Claude GitHub App 並授予其存取您的儲存庫的權限。雲端會話適用於現有的 GitHub 儲存庫，因此要啟動新項目，請先[在 GitHub 上建立空儲存庫](https://github.com/new)。
  </Step>

  <Step title="建立您的環境">
    連接 GitHub 後，系統會提示您建立雲端環境。環境控制 Claude 在會話期間可以存取的網路以及在建立新會話時執行的內容。請參閱[已安裝的工具](/docs/zh-TW/claude-code-on-the-web#installed-tools)以了解無需任何配置即可使用的內容。

    表單具有以下欄位：

    * **名稱**：顯示標籤。當您為不同的項目或存取級別有多個環境時很有用。
    * **網路存取**：控制會話可以在網際網路上到達的內容。預設值 `Trusted` 允許連接到[常見套件登錄](/docs/zh-TW/claude-code-on-the-web#default-allowed-domains)（如 npm、PyPI 和 RubyGems），同時阻止一般網際網路存取。
    * **環境變數**：可選變數，在每個會話中可用，採用 `.env` 格式。不要用引號包裝值，因為引號會儲存為值的一部分。這些對任何可以編輯此環境的人都可見。
    * **設定指令碼**：可選的 Bash 指令碼，在 Claude Code 啟動前執行。使用它來安裝雲端 VM 不包含的系統工具，如 `apt install -y gh`。結果會被[快取](/docs/zh-TW/claude-code-on-the-web#environment-caching)，因此指令碼不會在每個會話上重新執行。請參閱[設定指令碼](/docs/zh-TW/claude-code-on-the-web#setup-scripts)以了解範例和除錯提示。

    對於第一個項目，保留預設值並點擊**建立環境**。您可以[稍後編輯它或為不同的項目建立其他環境](/docs/zh-TW/claude-code-on-the-web#configure-your-environment)。
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  從您的終端連接
</h3>

如果您已經使用 GitHub CLI (`gh`)，您可以在不打開瀏覽器的情況下設定 Claude Code on the web。這需要 [Claude Code CLI](/docs/zh-TW/quickstart)。`/web-setup` 讀取您的本地 `gh` 令牌，將其連結到您的 Claude 帳戶，並在您沒有雲端環境時建立預設雲端環境。

<Note>
  啟用了[零資料保留](/docs/zh-TW/zero-data-retention)的組織無法使用 `/web-setup` 或其他雲端會話功能。如果未安裝或驗證 GitHub CLI，`/web-setup` 會改為開啟瀏覽器上線流程。
</Note>

<Steps>
  <Step title="使用 GitHub CLI 進行驗證">
    在您的 shell 中，如果您還沒有驗證 GitHub CLI，請進行驗證：

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="登入 Claude">
    在 Claude Code CLI 中，執行 `/login` 以使用您的 claude.ai 帳戶登入。如果您已經登入，請跳過此步驟。
  </Step>

  <Step title="執行 /web-setup">
    在 Claude Code CLI 中，執行：

    ```text theme={null}
    /web-setup
    ```

    這會將您的 `gh` 令牌同步到您的 Claude 帳戶。如果您還沒有雲端環境，`/web-setup` 會建立一個具有 Trusted 網路存取且沒有設定指令碼的環境。您可以[稍後編輯環境或新增變數](/docs/zh-TW/claude-code-on-the-web#configure-your-environment)。一旦 `/web-setup` 完成，您可以使用 [`--cloud`](/docs/zh-TW/claude-code-on-the-web#from-terminal-to-web) 從您的終端啟動雲端會話，或使用 [`/schedule`](/docs/zh-TW/routines) 設定定期任務。
  </Step>
</Steps>

<h2 id="start-a-task">
  啟動任務
</h2>

連接 GitHub 並建立環境後，您就可以提交任務了。

<Steps>
  <Step title="選擇儲存庫和分支">
    從 [claude.ai/code](https://claude.ai/code) 或 Claude 行動應用程式中的 Code 標籤，點擊輸入框下方的儲存庫選擇器，並為 Claude 選擇要在其中工作的儲存庫。每個儲存庫都顯示一個分支選擇器。將其更改為從功能分支而不是預設分支啟動 Claude。您可以新增多個儲存庫以在一個會話中跨它們工作。
  </Step>

  <Step title="選擇權限模式">
    輸入框旁邊的模式下拉菜單預設為**接受編輯**，其中 Claude 進行更改並推送分支而無需停止以獲得批准。如果您希望 Claude 提出方法並在編輯文件前等待您的同意，請切換到 **Plan Mode**。雲端會話不提供 Manual 或 Bypass 權限。請參閱[權限模式完整列表](/docs/zh-TW/permission-modes#available-modes)以了解每個模式允許的操作。
  </Step>

  <Step title="描述任務並提交">
    輸入您想要的內容的描述並按 Enter。要具體：

    * 命名文件或函數："新增帶有設定說明的 README" 或 "修復 `tests/test_auth.py` 中失敗的驗證測試" 比 "修復測試" 更好
    * 如果您有錯誤輸出，請貼上
    * 描述預期行為，而不僅僅是症狀

    Claude 複製儲存庫、執行您配置的設定指令碼（如果已配置）並開始工作。每個任務都有自己的會話和自己的分支，因此您無需等待一個完成就可以啟動另一個。
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  預填充會話
</h2>

您可以透過將查詢參數新增到 [claude.ai/code](https://claude.ai/code) URL 來預填充新會話的提示、儲存庫和環境。使用此功能來建立整合，例如問題追蹤器中的按鈕，該按鈕使用問題描述作為提示打開 Claude Code。

| 參數             | 描述                                                             |
| :------------- | :------------------------------------------------------------- |
| `prompt`       | 要在輸入框中預填充的提示文本。也接受別名 `q`。                                      |
| `prompt_url`   | 要從中獲取提示文本的 URL，用於太長而無法嵌入查詢字符串的提示。URL 必須允許跨源請求。設定 `prompt` 時忽略。 |
| `repositories` | 要預選的 `owner/repo` 段的逗號分隔列表。也接受別名 `repo`。                       |
| `environment`  | [環境](#connect-github-and-create-an-environment)的名稱或 ID 以預選。    |

對每個值進行 URL 編碼。下面的範例使用已選擇的提示和儲存庫打開表單：

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  檢查和迭代
</h2>

當 Claude 完成時，檢查更改、在特定行上留下反饋，並繼續進行直到差異看起來正確。

<Steps>
  <Step title="打開差異檢視">
    差異指示器顯示整個會話中新增和移除的行，例如 `+42 -18`。選擇它以打開差異檢視，左側有文件列表，右側有更改。
  </Step>

  <Step title="留下內聯評論">
    選擇差異中的任何行，輸入您的反饋，然後按 Enter。評論會排隊直到您發送下一條訊息，然後它們會與其捆綁。Claude 看到 "在 `src/auth.ts:47`，不要在這裡捕捉錯誤" 以及您的主要指令，因此您無需描述問題所在。
  </Step>

  <Step title="建立拉取請求">
    當差異看起來正確時，選擇差異檢視頂部的**建立 PR**。您可以將其作為完整 PR、草稿打開，或跳轉到 GitHub 的撰寫頁面，其中包含生成的標題和描述。
  </Step>

  <Step title="在 PR 後繼續迭代">
    建立 PR 後會話保持活躍。將 CI 失敗輸出或審查者評論貼上到聊天中，並要求 Claude 解決它們。要讓 Claude 自動監控 PR，請參閱[自動修復拉取請求](/docs/zh-TW/claude-code-on-the-web#auto-fix-pull-requests)。
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  設定故障排除
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  連接 GitHub 後沒有儲存庫出現
</h3>

雲端會話可以使用連接的 GitHub 帳戶可以看到的任何儲存庫，無論 Claude GitHub App 安裝在哪些儲存庫上。如果儲存庫遺失，請驗證連接的 GitHub 帳戶在 GitHub 上是否有權存取它。如果您還想要儲存庫的[自動修復](/docs/zh-TW/claude-code-on-the-web#auto-fix-pull-requests)，請在其上安裝應用程式：在 github.com 上，打開**設定 → 應用程式 → Claude → 配置**並驗證儲存庫是否列在**儲存庫存取**下。私有儲存庫需要與公開儲存庫相同的授權。

<h3 id="the-page-only-shows-a-github-login-button">
  頁面只顯示 GitHub 登入按鈕
</h3>

雲端會話需要連接的 GitHub 帳戶。透過上面的瀏覽器流程連接，或者如果您使用 GitHub CLI，從您的終端執行 `/web-setup`。如果您根本不想連接 GitHub，請參閱 [Remote Control](/docs/zh-TW/remote-control) 以在您自己的機器上執行 Claude Code 並從網頁監控它。

<h3 id="not-available-for-the-selected-organization">
  "不適用於選定的組織"
</h3>

企業組織可能需要管理員啟用 Claude Code on the web。聯繫您的 Anthropic 帳戶團隊。

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` 顯示 "No commands match" 或 "Unknown command"
</h3>

`/web-setup` 在 Claude Code CLI 內執行，而不是在您的 shell 中。首先啟動 `claude`，然後在提示符處輸入 `/web-setup`。

如果您在 Claude Code 內輸入它並且命令菜單顯示 `No commands match "/web-setup"`，或提交它返回 `Unknown command: /web-setup`，該命令被隱藏是因為未滿足要求。原因通常是您使用 API 金鑰或第三方提供商而不是 claude.ai 訂閱進行驗證。執行 `/login` 以使用您的 claude.ai 帳戶登入。

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  使用 `--cloud` 或 ultraplan 時出現 "Could not create a cloud environment" 或 "No cloud environment available"
</h3>

遠端會話功能會在您沒有雲端環境時自動建立預設雲端環境。如果您看到 "Could not create a cloud environment"，自動建立失敗。如果您看到 "No cloud environment available"，您的 CLI 早於自動建立。在任何一種情況下，在 Claude Code CLI 中執行 `/web-setup` 以手動建立一個，或訪問 [claude.ai/code](https://claude.ai/code) 並按照上面的**建立您的環境**步驟進行。

<h3 id="setup-script-failed">
  設定指令碼失敗
</h3>

設定指令碼以非零狀態退出，這會阻止會話啟動。常見原因：

* 套件安裝失敗，因為登錄不在您的[網路存取級別](/docs/zh-TW/claude-code-on-the-web#access-levels)中。`Trusted` 涵蓋大多數套件管理器；`None` 阻止它們全部。
* 指令碼引用在新鮮複製中不存在的文件或路徑。
* 在本地工作的命令在 Ubuntu 上需要不同的調用。

要除錯，在指令碼頂部新增 `set -x` 以查看哪個命令失敗。對於非關鍵命令，附加 `|| true` 以便它們不會阻止會話啟動。

<h3 id="new-sessions-hang-or-time-out-during-setup">
  新會話在設定期間掛起或逾時
</h3>

如果新會話在設定指令碼步驟上停滯或在指令碼完成前因通用容器錯誤而失敗，指令碼可能超過了大約五分鐘的時間預算來建立[環境快取](/docs/zh-TW/claude-code-on-the-web#environment-caching)。繁重的步驟，例如拉取大型 Docker 映像、同步完整依賴樹或下載模型權重，通常會將總數推過限制，特別是當它們一個接一個執行時。

要修復此問題，修剪指令碼以便它可靠地在五分鐘內完成：

* 使用 `&` 和最終 `wait` 並行執行獨立安裝，而不是按順序執行。
* 將最大的下載移出設定指令碼，進入[SessionStart hook](/docs/zh-TW/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks)，在背景中啟動它們，以便會話在它們完成時變得可用。
* 從設定指令碼中移除長重試睡眠，因為停滯的重試迴圈會計入預算。

<h3 id="session-keeps-running-after-closing-the-tab">
  會話在關閉標籤後保持執行
</h3>

這是設計使然。關閉標籤或導航離開不會停止會話。它在背景中繼續執行，直到 Claude 完成當前任務，然後閒置。從側邊欄，您可以[存檔會話](/docs/zh-TW/claude-code-on-the-web#archive-sessions)以將其從列表中隱藏，或[刪除它](/docs/zh-TW/claude-code-on-the-web#delete-sessions)以永久移除它。

<h2 id="next-steps">
  後續步驟
</h2>

現在您可以提交和檢查任務，這些頁面涵蓋接下來的內容：從您的終端啟動雲端會話、安排定期工作以及為 Claude 提供常設指令。

* [使用 Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)：完整參考，包括將會話傳送到您的終端、設定指令碼、環境變數和網路配置
* [Routines](/docs/zh-TW/routines)：按計劃、透過 API 呼叫或回應 GitHub 事件自動化工作
* [CLAUDE.md](/docs/zh-TW/memory)：為 Claude 提供在每個會話開始時載入的持久指令和上下文
* 安裝 Claude 行動應用程式以用於 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 或 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 以從您的手機監控會話。從 Claude Code CLI，`/mobile` 顯示 QR 碼。
