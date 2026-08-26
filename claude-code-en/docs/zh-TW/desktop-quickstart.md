> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 開始使用桌面應用程式

> 在桌面上安裝 Claude Code 並開始您的第一個編碼會話

桌面應用程式為您提供具有圖形介面的 Claude Code，專為並行執行多個會話而設計：用於管理並行工作的側邊欄、具有整合終端機和檔案編輯器的拖放式佈局、視覺化差異檢查、即時應用程式預覽、GitHub PR 監控與自動合併，以及排程任務。無需終端機。

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

<Note>
  Claude Code 需要 [Pro、Max、Team 或 Enterprise 訂閱](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing)。
</Note>

本頁面將引導您安裝應用程式並開始您的第一個會話。如果您已經設定完成，請參閱 [使用 Claude Code Desktop](/docs/zh-TW/desktop) 以取得完整參考。

桌面應用程式有三個標籤：

* **Chat**：無檔案存取的一般對話，類似於 claude.ai。
* **Cowork**：一個自主背景代理，在沙箱虛擬機中處理任務，具有自己的環境，可以獨立執行，同時您進行其他工作。裝置上的 Cowork 會話在您的電腦上執行 VM；遠端 Cowork 會話改為在 Anthropic 管理的 VM 上執行。
* **Code**：具有直接存取本機檔案的互動式編碼助手。您可以即時檢查並批准每項變更。

Chat 和 Cowork 涵蓋在 [Claude 說明中心](https://support.claude.com/)；安裝和部署桌面應用程式涵蓋在 [Claude Desktop 支援文章](https://support.claude.com/en/collections/16163169-claude-desktop)。本頁面重點關注 **Code** 標籤。

<h2 id="install">
  安裝
</h2>

<Steps>
  <Step title="安裝並登入">
    在 macOS 和 Windows 上，從上方連結下載安裝程式並執行它。在 Linux 上，請按照 [Claude Desktop on Linux](/docs/zh-TW/desktop-linux) 中的安裝步驟進行。從 macOS 上的應用程式資料夾、Windows 上的開始功能表或 Linux 上的應用程式啟動器啟動 Claude，然後使用您的 Anthropic 帳戶登入。
  </Step>

  <Step title="開啟 Code 標籤">
    點擊頂部中央的 **Code** 標籤。如果點擊 Code 提示您升級，您需要先 [訂閱付費方案](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade)。如果它提示您線上登入，請完成登入並重新啟動應用程式。如果您看到 403 錯誤，請參閱 [驗證疑難排解](/docs/zh-TW/desktop#403-or-authentication-errors-in-the-code-tab)。
  </Step>
</Steps>

桌面應用程式包含 Claude Code。您無需單獨安裝 Node.js 或 CLI。若要從終端機使用 `claude`，請單獨安裝 CLI。請參閱 [開始使用 CLI](/docs/zh-TW/quickstart)。

<h2 id="start-your-first-session">
  開始您的第一個會話
</h2>

開啟 Code 標籤後，選擇一個專案並給 Claude 一些工作。

<Steps>
  <Step title="選擇環境和資料夾">
    選擇 **Local** 以在您的機器上執行 Claude，直接使用您的檔案。點擊 **Select folder** 並選擇您的專案目錄。

    <Tip>
      從您熟悉的小型專案開始。這是查看 Claude Code 能做什麼的最快方式。在 Windows 上，必須安裝 [Git](https://git-scm.com/downloads/win) 才能使本機會話正常運作。大多數 Mac 預設包含 Git。
    </Tip>

    您也可以選擇：

    * **Remote**：在 Anthropic 的雲端基礎設施上執行會話，即使您關閉應用程式也會繼續。遠端會話使用與 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 相同的基礎設施。
    * **SSH**：透過 SSH 連接到遠端機器，例如您自己的伺服器、雲端 VM 或開發容器。Desktop 在您第一次連接時會自動在遠端機器上安裝 Claude Code。
    * **WSL** (Windows)：在 [WSL 2 distribution](/docs/zh-TW/desktop-wsl) 內執行會話；Claude Code、工具和 git 在 Linux 端執行，具有原生路徑。
  </Step>

  <Step title="選擇模型">
    從傳送按鈕旁的下拉式選單中選擇模型。請參閱 [models](/docs/zh-TW/model-config#available-models) 以比較可用的模型。您可以稍後從相同的下拉式選單變更模型。
  </Step>

  <Step title="告訴 Claude 要做什麼">
    輸入您想要 Claude 做的事情：

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    [會話](/docs/zh-TW/desktop#work-in-parallel-with-sessions) 是與 Claude 關於您的程式碼的對話。每個會話追蹤自己的上下文和變更，因此您可以處理多個任務而不會相互干擾。
  </Step>

  <Step title="檢查並接受變更">
    預設情況下，Code 標籤以 [Manual mode](/docs/zh-TW/desktop#choose-a-permission-mode) 啟動，其中 Claude 提出變更並等待您的批准才能應用它們。您將看到：

    1. [差異檢視](/docs/zh-TW/desktop#review-changes-with-diff-view) 顯示每個檔案中將發生的確切變更
    2. 接受/拒絕按鈕以批准或拒絕每項變更
    3. Claude 處理您的請求時的即時更新

    如果您拒絕變更，Claude 將詢問您希望如何以不同方式進行。在您接受之前，您的檔案不會被修改。
  </Step>
</Steps>

<h2 id="now-what">
  接下來呢？
</h2>

您已進行了第一次編輯。如需 Desktop 可執行的所有操作的完整參考，請參閱 [使用 Claude Code Desktop](/docs/zh-TW/desktop)。以下是一些接下來要嘗試的事項。

**中斷並引導。** 您可以隨時中斷 Claude。點擊停止按鈕立即中斷，或輸入更正並按 **Enter** 以在不停止執行中操作的情況下發送。無論哪種方式，您都無需等待它完成或重新開始。

**給 Claude 更多上下文。** 在提示框中輸入 `@filename` 以將特定檔案拉入對話，使用附件按鈕附加影像和 PDF，或直接將檔案拖放到提示中。Claude 擁有的上下文越多，結果越好。請參閱 [新增檔案和上下文](/docs/zh-TW/desktop#add-files-and-context-to-prompts)。

**使用 skills 執行可重複的任務。** 輸入 `/` 或點擊 **+** → **Slash commands** 以瀏覽 [內建命令](/docs/zh-TW/commands)、[自訂 skills](/docs/zh-TW/skills) 和外掛程式 skills。Skills 是可重複使用的提示，您可以在需要時調用，例如程式碼檢查清單或部署步驟。

**在提交前檢查變更。** Claude 編輯檔案後，會出現 `+12 -1` 指示器。點擊它以開啟 [差異檢視](/docs/zh-TW/desktop#review-changes-with-diff-view)，逐個檔案檢查修改，並在特定行上評論。Claude 讀取您的評論並進行修訂。點擊 **Review code** 讓 Claude 自己評估差異並留下內聯建議。

**調整您擁有的控制量。** 您的 [權限模式](/docs/zh-TW/desktop#choose-a-permission-mode) 設定 Claude 在不詢問批准的情況下可以執行的操作：

* **Manual**：預設值。Claude 在編輯檔案或執行命令前會詢問。
* **Accept edits**：Claude 自動接受檔案編輯以加快迭代。
* **Plan**：Claude 提出方法而不編輯任何檔案，這在大型重構前很有用。

**新增外掛程式以獲得更多功能。** 點擊提示框旁的 **+** 按鈕並選擇 **Plugins** 以瀏覽並安裝 [外掛程式](/docs/zh-TW/desktop#install-plugins)，這些外掛程式新增 skills、代理、MCP servers 等。

**排列您的工作區。** 將聊天、差異、終端機、檔案和瀏覽器窗格拖放到您想要的任何佈局中。使用 **Ctrl+\`** 開啟終端機以在您的會話旁執行命令，或點擊檔案路徑以在檔案窗格中開啟它。請參閱 [排列您的工作區](/docs/zh-TW/desktop#arrange-your-workspace)。

**預覽您的應用程式。** 當您在桌面中執行開發伺服器時，您的應用程式會在瀏覽器窗格中開啟，該窗格也可以 [開啟外部網站](/docs/zh-TW/desktop#browse-external-sites)。Claude 可以檢視執行中的應用程式、測試端點、檢查日誌並對其看到的內容進行迭代。請參閱 [預覽您的應用程式](/docs/zh-TW/desktop#preview-your-app)。

**追蹤您的提取請求。** 開啟 PR 後，Claude Code 監控 CI 檢查結果，並可以自動修復失敗或在所有檢查通過後合併 PR。請參閱 [監控提取請求狀態](/docs/zh-TW/desktop#monitor-pull-request-status)。

**將 Claude 放在排程上。** 設定 [排程任務](/docs/zh-TW/desktop-scheduled-tasks) 以定期自動執行 Claude：每天早上進行程式碼檢查、每週進行依賴項審計，或從您連接的工具中提取的簡報。

**準備好時擴展。** 從側邊欄開啟 [並行會話](/docs/zh-TW/desktop#work-in-parallel-with-sessions) 以同時處理多個任務，每個任務都在自己的 Git worktree 中，並開啟 [任務窗格](/docs/zh-TW/desktop#watch-background-tasks) 以監看會話正在執行的子代理和背景命令。開啟 [側邊聊天](/docs/zh-TW/desktop#ask-a-side-question-without-derailing-the-session) 以提出問題而不會偏離主線。將 [長期執行的工作發送到雲端](/docs/zh-TW/desktop#run-long-running-tasks-remotely)，以便即使您關閉應用程式也會繼續，或 [在網路或 IDE 中繼續會話](/docs/zh-TW/desktop#continue-in-another-surface)（如果任務花費的時間比預期長）。[連接外部工具](/docs/zh-TW/desktop#extend-claude-code)，例如 GitHub、Slack 和 Linear，以整合您的工作流程。

<h2 id="coming-from-the-cli">
  來自 CLI？
</h2>

Desktop 使用與 CLI 相同的引擎，但具有圖形介面。您可以在同一專案上同時執行兩者，它們共享配置（CLAUDE.md 檔案、MCP servers、hooks、skills 和設定）。如需功能、標誌等效項和 Desktop 中不可用的內容的完整比較，請參閱 [CLI 比較](/docs/zh-TW/desktop#coming-from-the-cli)。

<h2 id="what’s-next">
  接下來
</h2>

* [使用 Claude Code Desktop](/docs/zh-TW/desktop)：權限模式、並行會話、差異檢視、連接器和企業配置
* [疑難排解](/docs/zh-TW/desktop#troubleshooting)：常見錯誤和設定問題的解決方案
* [最佳實踐](/docs/zh-TW/best-practices)：撰寫有效提示和充分利用 Claude Code 的提示
* [常見工作流程](/docs/zh-TW/common-workflows)：除錯、重構、測試等的教學課程
