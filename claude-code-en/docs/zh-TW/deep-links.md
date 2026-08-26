> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 從連結啟動工作階段

> 從 URL 開啟 Claude Code 終端機工作階段。在執行手冊、警報和儀表板中嵌入 `claude-cli://` 連結，只需點擊即可在正確的儲存庫中使用正確的提示開啟 Claude Code。

深層連結是一個 `claude-cli://` URL，可在新的終端機視窗中開啟 Claude Code。該 URL 可以攜帶工作目錄和預先填入的提示。

這讓您可以為任務分享一個一鍵式的起點：任何安裝了 Claude Code 的人點擊該連結時，都會看到一個工作階段開啟，提示已經輸入。提示已填入但在您按下 Enter 之前不會發送。

由於深層連結是一個 URL，您可以將其放在任何可以放置連結的地方：

* 一個事件執行手冊步驟，開啟受影響服務的儲存庫並附帶診斷提示
* 一個監控警報或儀表板，連結到特定指標的調查提示
* 一個 README 或 wiki 頁面，使用入職提示開啟專案
* 一個 CI 失敗通知，預先填入失敗工作的名稱

本頁涵蓋如何[建立連結](#build-a-link)、[在執行手冊中嵌入連結或從 shell 觸發](#examples)，以及[在每個平台上管理或停用處理程式註冊](#registration-and-supported-platforms)。

<h2 id="how-it-works">
  運作方式
</h2>

`claude-cli://` 前綴是一個自訂 URL 配置，Claude Code 會向您的作業系統註冊，類似於 `mailto:` 連結如何開啟您的電子郵件用戶端。該連結可以存在於網頁、wiki、Slack 訊息或任何呈現連結的應用程式中。當您點擊一個時：

1. 瀏覽器或應用程式將 URL 交給您的作業系統。
2. 作業系統識別 `claude-cli://` 前綴並在您的機器上啟動 Claude Code。
3. 一個新的終端機視窗開啟，Claude Code 在連結指定的目錄中執行，連結的提示文字已在輸入框中。
4. 您閱讀提示，如果需要可編輯它，然後按下 Enter 發送它。

連結本身可以託管在任何地方，但工作階段始終在您點擊的電腦上本地開啟。請參閱[註冊和支援的平台](#registration-and-supported-platforms)以了解在每個作業系統上開啟哪個終端機模擬器。

<Note>
  顯示連結的平台必須允許自訂 URL 配置。GitHub 呈現的 Markdown 允許 `http` 和 `https`，但會在 README、問題、拉取請求和 wiki 中移除 `claude-cli://` 等配置。只有連結文字顯示，沒有連結在其後面，URL 被隱藏。請參閱[疑難排解](#the-link-renders-as-plain-text-instead-of-being-clickable)以了解解決方法。
</Note>

<h3 id="what-a-launched-session-shows">
  啟動的工作階段顯示什麼
</h3>

深層連結本身永遠不會執行任何操作。連結只選擇一個目錄並填入提示框。如果您點擊來自您不信任的頁面的連結，提示仍然是惰性的：在您閱讀填入的內容並按下 Enter 之前，沒有任何內容到達模型。

當工作階段開啟時，輸入框下方的警告行顯示 `Prompt from an external link`，並在您發送或清除提示之前保持可見。對於超過 1,000 個字元的提示，警告包括字元計數，並告訴您在按下 Enter 之前滾動並檢查完整文字，因為長提示可能會將指令推出螢幕。權限規則、`CLAUDE.md` 和所選目錄的信任提示的應用方式與任何其他工作階段相同。

<h2 id="build-a-link">
  建立連結
</h2>

每個深層連結都以 `claude-cli://open` 開始，這是處理程式接受的唯一路徑，後面跟著可選的查詢參數。最小形式在您的主目錄中開啟 Claude Code，提示為空：

```text theme={null}
claude-cli://open
```

新增參數以控制工作階段開始的位置和提示框包含的內容：

| 參數     | 描述                                                                                                                                                             |
| ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`    | 要預先填入提示框的文字。[URL 編碼](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent)該值。在多行提示中使用 `%0A` 表示換行符。最多 5,000 個字元。 |
| `cwd`  | 用作工作目錄的絕對路徑。網路和 UNC 路徑被拒絕，包含隱形或雙向控制字元的路徑也被拒絕。                                                                                                                  |
| `repo` | 一個 GitHub `owner/name` slug。Claude Code 將其解析為它之前看過的本地複製，並從那裡開始。如果您沒有匹配的複製，工作階段將在您的主目錄中開啟。                                                                      |

`cwd` 和 `repo` 是[設定工作目錄的兩種方式](#choose-between-cwd-and-repo)。如果您同時傳遞兩者，`cwd` 優先，`repo` 被忽略，即使 `cwd` 路徑不存在。

以下連結指向一個名為 `acme/payments` 的儲存庫，帶有一個兩行診斷提示。建立您自己的連結時，將 `acme/payments` 替換為您儲存庫的 `owner/name` slug：

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

點擊它會開啟一個新的終端機視窗，在 `acme/payments` 的本地複製中啟動 Claude Code，並用解碼的文字填入提示框：

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

您可以在按下 Enter 發送之前編輯提示。如果您沒有儲存庫的本地複製，工作階段將在您的主目錄中開啟。請參閱[在 `cwd` 和 `repo` 之間選擇](#choose-between-cwd-and-repo)以了解當您有多個複製或 worktrees 時如何選擇本地路徑。

<h3 id="choose-between-cwd-and-repo">
  在 `cwd` 和 `repo` 之間選擇
</h3>

當點擊連結的每個人都在相同的絕對路徑上有專案時，使用 `cwd`，例如標準化的 devcontainer 或 VM 映像。

當連結被共享且每個人將其複製到不同位置時，使用 `repo`。Claude Code 將 slug 解析為本地路徑，如下所示：

* 每次您在 Git 儲存庫中執行 `claude` 時，該目錄的檔案系統路徑都會針對儲存庫的 GitHub `owner/name` slug 進行記錄。
* 當深層連結到達時，`repo` 開啟您最近使用的任何匹配路徑。多個複製和 worktrees 被分別追蹤，因此它選擇您最後工作的那個。
* 查詢只找到您已至少執行過一次 Claude Code 的路徑。
* 連結不會改變簽出的分支。工作階段在該目錄目前的任何狀態下開啟。

啟動的工作階段顯示它選擇了哪個路徑，因此您可以確認正確的複製已開啟。

<h2 id="examples">
  範例
</h2>

下面的部分展示了使用深層連結的兩種常見方式：作為文件中的 Markdown 連結和作為指令碼或 shell 別名中的命令。

<h3 id="embed-a-link-in-a-runbook">
  在執行手冊中嵌入連結
</h3>

執行手冊中的深層連結為進行分類的人提供了一個一鍵式的方式，可以在正確的儲存庫中開始調查，並附帶準備好的提示。呈現執行手冊的平台必須允許自訂 URL 配置。GitHub 呈現的 Markdown 不允許 `claude-cli://`，因此 GitHub README、問題或 wiki 中的深層連結只顯示其標籤，沒有可點擊的連結。請參閱[疑難排解說明](#the-link-renders-as-plain-text-instead-of-being-clickable)以了解解決方法。

提示是 URL 的一部分，必須進行 URL 編碼。要產生編碼值，請在瀏覽器控制台或任何 URL 編碼器中通過 `encodeURIComponent` 傳遞您的提示文字。

下面的範例為名為 `web-gateway` 的服務的事件執行手冊新增了一個調查進入點：

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

要在您自己的執行手冊中使用此功能，請將 `acme/web-gateway` 替換為您服務的儲存庫 slug。這允許安裝了 Claude Code 並擁有該儲存庫本地複製的工程師點擊第 2 步並開始調查，提示已準備好發送。

<h3 id="open-a-link-from-the-shell">
  從 shell 開啟連結
</h3>

您也可以從 shell 指令碼、別名或自動化中開啟深層連結，而不是點擊它。使用連結作為參數呼叫您作業系統的 URL 開啟命令。

<Tabs>
  <Tab title="macOS">
    內建的 `open` 命令將 URL 傳遞給已註冊的 `claude-cli://` 處理程式：

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    大多數桌面環境提供 `xdg-open`，它將 URL 傳遞給已註冊的處理程式：

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    在 PowerShell 中，`Start-Process` 將 URL 傳遞給已註冊的處理程式：

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    在 `cmd.exe` 中，`start` 將其第一個引用的參數視為視窗標題，因此在 URL 之前傳遞一個空標題：

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  註冊和支援的平台
</h2>

Claude Code 在您第一次在 macOS、Linux 和 Windows 上啟動互動式工作階段時，向您的作業系統註冊 `claude-cli://` 處理程式。您不需要執行單獨的安裝命令。註冊只寫入使用者級別的位置：

| 平台      | 處理程式位置                                                                                                |
| ------- | ----------------------------------------------------------------------------------------------------- |
| macOS   | `~/Applications/Claude Code URL Handler.app`                                                          |
| Linux   | `claude-code-url-handler.desktop` 在 `$XDG_DATA_HOME/applications` 下，預設為 `~/.local/share/applications` |
| Windows | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                       |

處理程式在偵測到的終端機模擬器中啟動 Claude Code。在 macOS 上，Claude Code 記住您最近互動式工作階段中的終端機並重複使用它，支援 iTerm2、Ghostty、kitty、Alacritty、WezTerm 和 Terminal.app。在 Linux 上，它遵守 `$TERMINAL` 環境變數，然後是 `x-terminal-emulator`，然後是常見模擬器的列表。在 Windows 上，它優先選擇 Windows Terminal，然後是 PowerShell，然後是 `cmd.exe`。

要完全防止註冊，請在 `settings.json` 中將 [`disableDeepLinkRegistration`](/docs/zh-TW/settings) 設定為 `"disable"`。要在整個組織中強制執行此操作，使用者無法重新啟用它，請改為在[受管設定](/docs/zh-TW/server-managed-settings)中設定它。

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  開啟 VS Code 標籤而不是終端機
</h2>

VS Code 擴充功能在 `vscode://anthropic.claude-code/open` 註冊自己的處理程式，它開啟一個 Claude Code 編輯器標籤而不是終端機視窗。請參閱[從其他工具啟動 VS Code 標籤](/docs/zh-TW/vs-code#launch-a-vs-code-tab-from-other-tools)以了解該 URL 的參數。

<h2 id="troubleshooting">
  疑難排解
</h2>

<h3 id="clicking-the-link-does-nothing">
  點擊連結沒有反應
</h3>

處理程式可能尚未註冊。在該機器上啟動一個互動式 `claude` 工作階段一次，退出，然後再試一次連結。如果您在沒有桌面環境的 Linux 上，`xdg-open` 可能沒有任何東西可以分派。

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  連結呈現為純文字而不是可點擊的
</h3>

某些 Markdown 呈現器只允許 `http` 和 `https` 連結，並移除其他 URL 配置。GitHub 在 README、問題、拉取請求和 wiki 中執行此操作：`[label](claude-cli://...)` 呈現為只是 `label`，沒有連結，URL 被移除。在這些平台上，將深層連結放在程式碼區塊中，以便讀者可以看到 URL 並將其貼到瀏覽器的位址列中。

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  工作階段在我的主目錄中開啟而不是儲存庫
</h3>

`repo` 參數只解析為 Claude Code 已經看過的複製。在複製中執行一次 `claude`，以便記錄其路徑，或將連結切換為使用 `cwd` 和絕對路徑。

<h3 id="the-link-opens-the-wrong-terminal">
  連結開啟了錯誤的終端機
</h3>

在 macOS 上，在您首選的終端機中啟動 `claude` 一次，下一個深層連結將使用它。在 Linux 上，將 `$TERMINAL` 環境變數設定為您首選模擬器的命令名稱。在 Windows 上，順序是固定的：安裝 Windows Terminal，如果您想讓連結在那裡開啟而不是在 PowerShell 或 `cmd.exe` 視窗中。

<h2 id="learn-more">
  深入瞭解
</h2>

這些頁面涵蓋了啟動或擴充 Claude Code 工作階段的相關方式：

* [Skills](/docs/zh-TW/skills)：將長執行手冊提示儲存為儲存庫中的 `/skill`，以便深層連結的 `q` 參數只需命名它
* [Non-interactive mode](/docs/zh-TW/headless)：從指令碼執行 Claude 並捕獲輸出，而無需開啟終端機
