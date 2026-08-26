> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定權限

> 使用細粒度權限規則、模式和受管理原則來控制 Claude Code 可以存取和執行的操作。

Claude Code 支援細粒度權限，讓您可以精確指定代理允許執行和不允許執行的操作。權限設定可以簽入版本控制並分發給組織中的所有開發人員，也可以由個別開發人員自訂。

<h2 id="permission-system">
  權限系統
</h2>

Claude Code 使用分層權限系統來平衡功能和安全性：

| 工具類型    | 範例            | 需要批准                                  | "是，不要再問"行為    |
| :------ | :------------ | :------------------------------------ | :------------ |
| 唯讀      | 檔案讀取、Grep     | 否，在[工作目錄和其他目錄](#working-directories)內 | 不適用           |
| Bash 命令 | Shell 執行      | 是，除了內建的[唯讀命令](#read-only-commands)集合  | 每個專案目錄和命令永久有效 |
| 檔案修改    | Edit/Write 檔案 | 是                                     | 直到工作階段結束      |

在 Bash 或 PowerShell 權限提示上，按 `Ctrl+E` 以顯示命令的說明：它的功能、Claude 為什麼執行它，以及可能出現的問題，標記為**低風險**、**中風險**或**高風險**。Claude Code 只在您按 `Ctrl+E` 時才會將命令和 Claude 自己對該呼叫的描述傳送給模型以產生說明，而不是在每次提示時都這樣做。顯示說明不會執行命令；再次按 `Ctrl+E` 以隱藏它。

若要關閉快捷鍵，請在 `~/.claude.json` 中將 [`permissionExplainerEnabled`](/docs/zh-TW/settings#global-config-settings) 設定為 `false`。

<h2 id="manage-permissions">
  管理權限
</h2>

您可以使用 `/permissions` 檢視和管理 Claude Code 的工具權限。此 UI 列出所有權限規則及其來源的 `settings.json` 檔案。

* **Allow** 規則讓 Claude Code 使用指定的工具，無需手動批准。
* **Ask** 規則在 Claude Code 嘗試使用指定工具時提示確認。
* **Deny** 規則防止 Claude Code 使用指定的工具。

規則按順序評估：deny、ask，然後 allow。該順序中的第一個符合項決定結果，規則特異性不會改變順序。

一個廣泛的 deny 規則（例如 `Bash(aws *)`）會阻止每個符合的呼叫，包括也符合較窄 allow 規則（例如 `Bash(aws s3 ls)`）的呼叫，因此 deny 規則無法攜帶允許清單例外。ask 和 allow 之間也適用相同的優先順序：符合的 ask 規則即使有更具體的 allow 規則也符合相同的呼叫時，也會提示。

Deny 規則的行為取決於它們是否命名工具或在工具內限定模式。像 `Bash` 這樣的裸工具名稱會將工具從 Claude 的上下文中完全移除，因此 Claude 永遠看不到它。像 `Bash(rm *)` 這樣的限定規則會保留工具可用性，並在 Claude 嘗試時阻止符合的呼叫。

<Note>
  權限規則由 Claude Code 強制執行，而不是由模型強制執行。您的提示或 `CLAUDE.md` 中的指令會影響 Claude 嘗試執行的操作，但不會改變 Claude Code 允許的操作。若要授予或撤銷存取權限，請使用 `/permissions`、此處描述的規則、[permission mode](/docs/zh-TW/permission-modes) 或 [PreToolUse hook](#extend-permissions-with-hooks)。
</Note>

<h2 id="permission-modes">
  權限模式
</h2>

Claude Code 支援多種權限模式來控制工具的批准方式。請參閱 [Permission modes](/docs/zh-TW/permission-modes) 以了解何時使用每一種。在您的 [settings files](/docs/zh-TW/settings#settings-files) 中設定 `defaultMode`：

| 模式                  | 描述                                                                                                                                                                                                                                                           |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 標準行為：在首次使用每個工具時提示權限。在 CLI、VS Code 和 JetBrains 擴充功能以及桌面應用程式中標示為 Manual，Claude Code 接受 `manual` 作為別名。標籤和別名需要 Claude Code v2.1.200 或更新版本。桌面應用程式的標籤不取決於您的 CLI 版本                                                                                                 |
| `acceptEdits`       | 自動接受工作目錄或 `additionalDirectories` 中路徑的檔案編輯和常見檔案系統命令（`mkdir`、`touch`、`mv`、`cp` 等）                                                                                                                                                                             |
| `plan`              | Claude 讀取檔案並執行唯讀 shell 命令以探索，但不編輯您的原始檔案。在 CLI 和 VS Code 擴充功能中標示為 Plan                                                                                                                                                                                        |
| `auto`              | 自動批准工具呼叫，並進行背景安全檢查以驗證操作是否符合您的要求                                                                                                                                                                                                                              |
| `dontAsk`           | 自動拒絕工具，除非透過 `/permissions` 或 `permissions.allow` 規則預先批准。`AskUserQuestion`、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標示為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具即使您已允許它們也會被拒絕 |
| `bypassPermissions` | 跳過權限提示，但明確的 `ask` 規則、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標示為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具強制的提示除外。根目錄和主目錄移除（例如 `rm -rf /`）仍會作為斷路器提示                          |

<Warning>
  `bypassPermissions` 模式會跳過權限提示，包括對 `.git`、`.config/git`、`.claude`、`.vscode`、`.idea`、`.husky`、`.cargo`、`.devcontainer`、`.yarn` 和 `.mvn` 的寫入。僅在隔離環境（如容器或虛擬機）中使用此模式，其中 Claude Code 無法造成損害。

  此模式中仍會觸發一些提示。明確的 `ask` 規則、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標示為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具仍會提示。針對檔案系統根目錄或主目錄的移除（例如 `rm -rf /` 和 `rm -rf ~`）也會作為斷路器提示以防止模型錯誤，包括當命令包含使用 `$(...)` 或反引號的命令替換或使用 `<(...)` 的程序替換時。在 v2.1.208 之前，只有純形式（例如 `rm -rf ~` 作為其自身命令輸入）會提示；透過替換到達移除的命令不會提示。
</Warning>

若要防止 `bypassPermissions` 或 `auto` 模式被使用，請在任何 [settings file](/docs/zh-TW/settings#settings-files) 中將 `permissions.disableBypassPermissionsMode` 或 `permissions.disableAutoMode` 設定為 `"disable"`。這些在 [managed settings](#managed-settings) 中最有用，因為它們無法被覆蓋。

<h2 id="permission-rule-syntax">
  權限規則語法
</h2>

權限規則遵循格式 `Tool` 或 `Tool(specifier)`。

<h3 id="match-all-uses-of-a-tool">
  符合工具的所有使用
</h3>

若要符合工具的所有使用，請使用不帶括號的工具名稱：

| 規則         | 效果           |
| :--------- | :----------- |
| `Bash`     | 符合所有 Bash 命令 |
| `WebFetch` | 符合所有網頁擷取請求   |
| `Read`     | 符合所有檔案讀取     |

`Bash(*)` 等同於 `Bash` 並符合所有 Bash 命令。作為拒絕規則，兩種形式都會從 Claude 的上下文中移除該工具。

<h3 id="use-specifiers-for-fine-grained-control">
  使用指定符進行細粒度控制
</h3>

在括號中新增指定符以符合特定工具使用：

| 規則                             | 效果                     |
| :----------------------------- | :--------------------- |
| `Bash(npm run build)`          | 符合確切命令 `npm run build` |
| `Read(./.env)`                 | 符合讀取目前目錄中的 `.env` 檔案   |
| `WebFetch(domain:example.com)` | 符合對 example.com 的擷取請求  |

<h3 id="match-by-input-parameter">
  按輸入參數進行符合
</h3>

拒絕和詢問規則可以使用 `Tool(param:value)` 符合任何工具上的頂層輸入參數。當 Claude 呼叫該工具且該參數設定為該確切值時，規則會符合。允許規則對於一個參數值不會確立該呼叫整體是安全的，因此允許規則繼續使用每個工具自己的指定符語法。這適用於工具接受的任何純量參數：

| 規則                             | 符合                         |
| :----------------------------- | :------------------------- |
| `Agent(model:opus)`            | 要求 Opus 模型層級的 Agent 呼叫     |
| `Agent(isolation:worktree)`    | 要求 git worktree 的 Agent 呼叫 |
| `Bash(run_in_background:true)` | 在背景執行的 Bash 呼叫             |

參數符合遵循這些規則：

* 參數名稱必須是工具輸入的直接欄位，例如 Agent 工具上的 `model`。巢狀在物件或陣列內的欄位不可符合
* 每個規則命名一個參數。若要在 `model` 和 `isolation` 上設定閘道，請寫入兩個規則 `Agent(model:opus)` 和 `Agent(isolation:worktree)`，而不是在一個規則中組合它們
* 值支援 `*` 作為符合任何字元序列的萬用字元，因此 `Agent(isolation:*)` 符合任何明確的隔離值。沒有 `*` 時，符合是確切的
* 模型省略的參數永遠不會被符合，因此 `Agent(model:*)` 不符合留下 `model` 未設定的呼叫
* 值與 Claude 傳送的字面輸入進行比較，在任何正規化之前。`Agent(model:opus)` 符合別名 `opus` 但不符合完整模型 ID。使用 [`--verbose`](/docs/zh-TW/cli-reference) 執行以查看每個工具呼叫中的確切參數名稱和值
* 冒號周圍的空格被忽略

工具已使用自己的規範化規則符合的欄位不可以這種方式符合：Bash 和 PowerShell 的 `command`、Read、Edit 和 Write 的 `file_path`、Grep 和 Glob 的 `path`、NotebookEdit 的 `notebook_path`，以及 WebFetch 的 `url`。像 `Bash(command:rm *)` 這樣的規則可能會被複合命令繞過，因此 Claude Code 會忽略它並在啟動時發出警告。改用 `Bash(rm *)`、`Read(./path)` 或 `WebFetch(domain:host)`。

<h3 id="wildcard-patterns">
  萬用字元模式
</h3>

Bash 規則支援使用 `*` 的 glob 模式。萬用字元可以出現在命令中的任何位置。此設定允許 npm 和 git commit 命令，同時阻止 git push：

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

`*` 前的空格很重要：`Bash(ls *)` 符合 `ls -la` 但不符合 `lsof`，而 `Bash(ls*)` 兩者都符合。`:*` 後綴是寫入尾部萬用字元的等效方式，所以 `Bash(ls:*)` 符合與 `Bash(ls *)` 相同的命令。

當您為命令前綴選擇「是，不要再問」時，權限對話框會寫入空格分隔的形式。`:*` 形式僅在模式末尾被識別。在像 `Bash(git:* push)` 這樣的模式中，冒號被視為字面字元，不會符合 git 命令。

<h3 id="tool-name-wildcards">
  工具名稱萬用字元
</h3>

拒絕和詢問規則也接受工具名稱位置中的 glob 模式。該模式必須符合完整工具名稱：`"*"` 符合每個工具，而 `"mcp__*"` 符合所有伺服器上的每個 MCP 工具。由裸名稱 glob 拒絕規則符合的工具會從 Claude 的上下文中移除，與裸工具名稱相同。此設定拒絕每個 MCP 工具：

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

允許規則僅在字面 `mcp__<server>__` 前綴之後接受工具名稱 glob。伺服器區段必須無 glob，以便規則命名您設定的特定伺服器。`mcp__puppeteer__*` 符合來自 `puppeteer` 伺服器的每個工具，而 `mcp__github__get_*` 符合其 `get_` 工具。未錨定的允許 glob（例如 `"*"`、`"B*"` 或 `"mcp__*"`）會被跳過並顯示警告，不會自動核准任何內容。

拒絕或詢問規則，其工具名稱不符合任何已知工具，會在啟動時產生警告以捕捉拼寫錯誤。包含 `_` 或 `*` 的工具名稱不受檢查限制。

工具在文字記錄和權限對話框中顯示的標籤可能與其規範名稱不同。例如，文字記錄中標記為 `Stop Task` 的工具具有規範名稱 `TaskStop`。權限規則和 [hook 匹配器](/docs/zh-TW/hooks)僅符合規範名稱，因此寫成 `Stop Task` 的規則不符合。對於拒絕和詢問規則，上述啟動警告會捕捉不匹配。使用 [工具參考](/docs/zh-TW/tools-reference)中列出的規範名稱。

<h2 id="tool-specific-permission-rules">
  工具特定的權限規則
</h2>

<h3 id="bash">
  Bash
</h3>

Bash 權限規則支援使用 `*` 的萬用字元符合。萬用字元可以出現在命令中的任何位置，包括開頭、中間或結尾：

* `Bash(npm run build)` 符合確切的 Bash 命令 `npm run build`
* `Bash(npm run test *)` 符合以 `npm run test` 開頭的 Bash 命令
* `Bash(npm *)` 符合任何以 `npm ` 開頭的命令
* `Bash(* install)` 符合任何以 ` install` 結尾的命令
* `Bash(git * main)` 符合命令如 `git checkout main` 和 `git log --oneline main`

單一 `*` 符合任何字元序列，包括空格，所以一個萬用字元可以跨越多個引數。`Bash(git *)` 符合 `git log --oneline --all`，而 `Bash(git * main)` 符合 `git push origin main` 以及 `git merge main`。

當 `*` 出現在末尾且前面有空格時（如 `Bash(ls *)`），它會強制執行字邊界，要求前綴後面跟著空格或字串結尾。例如，`Bash(ls *)` 符合 `ls -la` 但不符合 `lsof`。相比之下，`Bash(ls*)` 沒有空格會同時符合 `ls -la` 和 `lsof`，因為沒有字邊界限制。

<h4 id="compound-commands">
  複合命令
</h4>

<Tip>
  Claude Code 知道 shell 運算子，所以前綴符合規則如 `Bash(safe-cmd *)` 不會給它執行命令 `safe-cmd && other-cmd` 的權限。已識別的命令分隔符是 `&&`、`||`、`;`、`|`、`|&`、`&` 和換行符。規則必須獨立符合每個子命令。
</Tip>

當您使用「是，不要再問」批准複合命令時，Claude Code 會為每個需要批准的子命令儲存一個單獨的規則，而不是為完整複合字串儲存單一規則。例如，批准 `git status && npm test` 會為 `npm test` 儲存一個規則，因此未來的 `npm test` 呼叫會被識別，無論 `&&` 前面是什麼。子命令如 `cd` 進入子目錄會為該路徑產生自己的 Read 規則。單一複合命令最多可能儲存 5 個規則。

<h4 id="process-wrappers">
  程序包裝器
</h4>

在符合 Bash 規則之前，Claude Code 會移除一組固定的程序包裝器，所以像 `Bash(npm test *)` 這樣的規則也符合 `timeout 30 npm test`。已識別的包裝器是 `timeout`、`time`、`nice`、`nohup` 和 `stdbuf`。

裸 `xargs` 也會被移除，所以 `Bash(grep *)` 符合 `xargs grep pattern`。移除僅在 `xargs` 沒有旗標時適用：像 `xargs -n1 grep pattern` 這樣的呼叫被符合為 `xargs` 命令，所以為內部命令編寫的規則不涵蓋它。

此包裝器清單是內建的，不可設定。開發環境執行器如 `direnv exec`、`devbox run`、`mise exec`、`npx` 和 `docker exec` 不在清單中。因為這些工具將其引數作為命令執行，像 `Bash(devbox run *)` 這樣的規則符合 `run` 後面的任何內容，包括 `devbox run rm -rf .`。若要批准環境執行器內的工作，請編寫包含執行器和內部命令的特定規則，如 `Bash(devbox run npm test)`。為您想要允許的每個內部命令新增一個規則。

Exec 包裝器如 `watch`、`setsid`、`ionice` 和 `flock` 始終提示，無法透過像 `Bash(watch *)` 這樣的前綴規則自動批准。同樣適用於帶有 `-exec` 或 `-delete` 的 `find`：`Bash(find *)` 規則不涵蓋這些形式。若要批准特定呼叫，請為完整命令字串編寫精確符合規則。

<h4 id="read-only-commands">
  唯讀命令
</h4>

Claude Code 將一組內建的 Bash 命令識別為唯讀，並在每種模式中無需權限提示即可執行它們。這些包括 `ls`、`cat`、`echo`、`pwd`、`head`、`tail`、`grep`、`find`、`wc`、`which`、`diff`、`stat`、`du`、`cd` 和 `git` 的唯讀形式。該集合不可設定；若要要求其中一個命令的提示，請為其新增 `ask` 或 `deny` 規則。

對於每個旗標都是唯讀的命令，允許未引用的 glob 模式，所以 `ls *.ts` 和 `wc -l src/*.py` 無需提示即可執行。具有寫入能力或執行能力旗標的命令，如 `find`、`sort`、`sed` 和 `git`，在存在未引用的 glob 時仍會提示，因為 glob 可能會擴展為像 `-delete` 這樣的旗標。

`cd` 進入工作目錄或[額外目錄](#working-directories)內的路徑也是唯讀的。像 `cd packages/api && ls` 這樣的複合命令在每個部分都符合時無需提示即可執行。在一個複合命令中結合 `cd` 和 `git` 會在 `cd` 變更進入不同目錄時提示，因為在新目錄中執行 `git` 可能會執行該目錄的 hooks。`cd` 其目標解析為目前工作目錄的是無操作的，不會觸發此提示。

在一個複合命令中結合 `cd` 與輸出重新導向也會在 Claude Code 無法判斷重新導向目標在 `cd` 執行後針對哪個目錄解析時提示。其唯一重新導向目標是 `/dev/null` 的命令，如 `cd app; grep -r pattern . 2>/dev/null`，不會觸發此提示，因為 `/dev/null` 不依賴於工作目錄。在 v2.1.207 之前，包含 `cd` 的複合命令會針對任何輸出重新導向提示，包括其唯一目標是 `/dev/null` 的重新導向。

<Warning>
  嘗試限制命令引數的 Bash 權限模式很脆弱。例如，`Bash(curl http://github.com/ *)` 旨在將 curl 限制為 GitHub URL，但不會符合以下變化：

  * URL 前的選項：`curl -X GET http://github.com/...`
  * 不同的協定：`curl https://github.com/...`
  * 重新導向：`curl -L http://bit.ly/xyz`（重新導向到 GitHub）
  * 變數：`URL=http://github.com && curl $URL`
  * 額外空格：`curl  http://github.com`

  為了更可靠的 URL 篩選，請考慮：

  * **限制 Bash 網路工具**：使用 deny 規則阻止 `curl`、`wget` 和類似命令，然後使用 WebFetch 工具搭配 `WebFetch(domain:github.com)` 權限以允許的網域
  * **使用 PreToolUse hooks**：實作一個 hook 來驗證 Bash 命令中的 URL 並阻止不允許的網域
  * **新增 CLAUDE.md 指導**：在 `CLAUDE.md` 中描述您允許的 curl 模式。這會形塑 Claude 嘗試的內容，但不會強制執行邊界，所以請將其與上述選項之一配對

  請注意，單獨使用 WebFetch 不會防止網路存取。如果允許 Bash，Claude 仍然可以使用 `curl`、`wget` 或其他工具來存取任何 URL。
</Warning>

<h3 id="powershell">
  PowerShell
</h3>

PowerShell 權限規則使用與 Bash 規則相同的形式。使用 `*` 的萬用字元在任何位置符合，`:*` 後綴等同於尾部 ` *`，而裸 `PowerShell` 或 `PowerShell(*)` 符合每個命令。此設定允許 `Get-ChildItem` 和 `git commit` 命令，同時阻止 `Remove-Item`：

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

常見別名在符合前會被正規化。為 cmdlet 名稱編寫的規則也符合其別名，所以 `PowerShell(Get-ChildItem *)` 符合 `gci`、`ls` 和 `dir`。符合不區分大小寫。

Claude Code 解析 PowerShell AST 並獨立檢查複合命令中的每個命令。管道運算子 `|`、陳述式分隔符 `;` 和在 PowerShell 7+ 上的鏈運算子 `&&` 和 `||` 將複合命令分割為子命令。規則必須符合每個子命令才能允許複合命令。

<h3 id="read-and-edit">
  Read 和 Edit
</h3>

`Edit` 規則適用於所有編輯檔案的內建工具。Claude 會盡力嘗試將 `Read` 規則應用於所有讀取檔案的內建工具，如 Grep 和 Glob，以及您提示中的 `@file` 提及，以及連接的 [IDE](/docs/zh-TW/vs-code#the-built-in-ide-mcp-server) 與 Claude 共享的選擇和開啟檔案內容。

`Read` deny 規則也會阻止同一路徑上的 [Edit 工具](/docs/zh-TW/errors#file-is-covered-by-a-read-deny-rule)，包括在該處建立新檔案。Write 和 NotebookEdit 不涵蓋，所以為任何工具都不可變更的路徑新增 `Edit` deny 規則。需要 Claude Code v2.1.208 或更新版本。

<Warning>
  Read 和 Edit deny 規則適用於 Claude 的內建檔案工具和 Claude Code 在 Bash 中識別的檔案命令，如 `cat`、`head`、`tail` 和 `sed`。它們不適用於間接讀取或寫入檔案的任意子程序，如自行開啟檔案的 Python 或 Node 指令碼。為了進行作業系統級別的強制執行，以阻止所有程序存取路徑，請[啟用沙箱](/docs/zh-TW/sandboxing)。
</Warning>

Read 和 Edit 規則都遵循 [gitignore](https://git-scm.com/docs/gitignore) 規格，具有四種不同的模式類型：

| 模式                | 意義             | 範例                               | 符合                                  |
| ----------------- | -------------- | -------------------------------- | ----------------------------------- |
| `//path`          | 來自檔案系統根目錄的絕對路徑 | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`           |
| `~/path`          | 來自主目錄的路徑       | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`      |
| `/path`           | 相對於設定來源的路徑     | `Edit(/src/**/*.ts)`             | `<project root>/src/**/*.ts` 在專案設定中 |
| `path` 或 `./path` | 相對於目前目錄的路徑     | `Read(*.env)`                    | `<cwd>/*.env`                       |

<Warning>
  像 `/Users/alice/file` 這樣的模式不是絕對路徑。單一前導斜線錨定在設定來源，而不是檔案系統根目錄。使用 `//Users/alice/file` 表示絕對路徑。
</Warning>

`/path` 模式錨定在與定義它的設定檔案相關聯的目錄，所以相同的規則根據您放置它的位置符合不同的位置：

| 規則定義在                             | `/path` 解析為                |
| :-------------------------------- | :------------------------- |
| 專案或本機設定，如 `.claude/settings.json` | `<project root>/path`      |
| 使用者設定在 `~/.claude/settings.json`  | `~/.claude/path`           |
| 使用 `--settings <file>` 傳遞的檔案      | `<directory of file>/path` |
| CLI 旗標、`/permissions` 或工作階段規則     | `<original cwd>/path`      |

像 `Read(/secrets/**)` 這樣的 deny 規則在使用者設定中會阻止 `~/.claude/secrets/**`，而不是您專案中的 `secrets` 目錄。若要在使用者設定中編寫適用於每個專案內部的規則，請改用 `//` 絕對路徑或 `~/` 主目錄相對路徑。

在 Windows 上，路徑在符合前會被正規化為 POSIX 形式。`C:\Users\alice` 變成 `/c/Users/alice`，所以使用 `//c/**/.env` 來符合該磁碟上任何位置的 `.env` 檔案。若要符合所有磁碟，請使用 `//**/.env`。

範例：

* `Edit(/docs/**)`: 編輯 `<project>/docs/` 中的檔案（不是 `/docs/` 也不是 `<project>/.claude/docs/`）
* `Read(~/.zshrc)`: 讀取您主目錄的 `.zshrc`
* `Edit(//tmp/scratch.txt)`: 編輯絕對路徑 `/tmp/scratch.txt`
* `Read(src/**)`: 從 `<current-directory>/src/` 讀取

一個規則只符合其錨點下的檔案，所以錨點決定了 deny 規則的範圍。裸檔案名稱遵循 gitignore 語義，並在任何深度符合，所以 `Read(.env)` 和 `Read(**/.env)` 是等價的：

| Deny 規則                        | 阻止                  | 不阻止                |
| ------------------------------ | ------------------- | ------------------ |
| `Read(.env)` 或 `Read(**/.env)` | 目前目錄或其下的任何 `.env`   | 父目錄或另一個專案中的 `.env` |
| `Read(//**/.env)`              | 檔案系統上任何位置的任何 `.env` | 無；規則錨定在檔案系統根目錄     |

<Note>
  在 gitignore 模式中，`*` 符合單一目錄中的檔案，而 `**` 遞迴符合目錄。若要允許所有檔案存取，請使用不帶括號的工具名稱：`Read`、`Edit` 或 `Write`。
</Note>

當您使用「是，不要再問」批准檔案路徑時，Claude Code 會逸出該路徑中的 gitignore 模式字元，如 `[`、`]` 和 `*`，所以產生的規則只符合您批准的字面路徑。您自己編寫的規則不會被逸出。在 v2.1.202 之前，Claude Code 會儲存未逸出的路徑，所以名為 `[2024-06] Reports` 的目錄產生的規則可能無法符合其自己的路徑或符合無意的同級目錄。

當 Claude 存取符號連結時，權限規則檢查兩個路徑：符號連結本身和它解析到的檔案。Allow 和 deny 規則對該對的處理方式不同：allow 規則回退到提示您，而 deny 規則直接阻止。

* **Allow 規則**：僅在符號連結路徑及其目標都符合時適用。允許目錄內的符號連結指向外部仍會提示您。
* **Deny 規則**：在符號連結路徑或其目標符合時適用。指向被拒絕檔案的符號連結本身被拒絕。

例如，使用 `Read(./project/**)` 允許和 `Read(~/.ssh/**)` 拒絕，位於 `./project/key` 指向 `~/.ssh/id_rsa` 的符號連結被阻止：目標未通過 allow 規則且符合 deny 規則。

<h3 id="webfetch">
  WebFetch
</h3>

WebFetch 規則使用 `domain:` 前綴，並針對請求 URL 的主機名進行符合。符合不區分大小寫，支援 `*` 萬用字元，並從規則和主機名中移除尾部 `.`，所以 `example.com.` 和 `example.com` 被視為相同。

* `WebFetch(domain:example.com)` 符合對 `example.com` 的請求
* `WebFetch(domain:*.example.com)` 符合任何深度的任何子網域，如 `api.example.com` 或 `a.b.example.com`，但不符合 `example.com` 本身
* `WebFetch(domain:*)` 符合每個網域，等同於裸 `WebFetch` 規則

在前導 `*.` 或裸 `*` 以外的任何位置，萬用字元僅符合兩個點之間的文字。`WebFetch(domain:example.*)` 符合 `example.org`，其中 `*` 變成 `org`，但不符合 `example.evil.com`，其中 `*` 必須變成 `evil.com` 並跨越一個點。這可防止尾部萬用字元符合攻擊者可以註冊的網域。

<h3 id="mcp">
  MCP
</h3>

MCP 規則使用在 Claude Code 中設定的伺服器名稱，選擇性地後跟來自該伺服器的工具名稱。

* `mcp__puppeteer` 符合由 `puppeteer` 伺服器提供的任何工具
* `mcp__puppeteer__*` 使用萬用字元語法，也符合來自 `puppeteer` 伺服器的所有工具
* `mcp__puppeteer__puppeteer_navigate` 符合由 `puppeteer` 伺服器提供的 `puppeteer_navigate` 工具

如果您的組織已將 [claude.ai 連接器](/docs/zh-TW/mcp#organization-controls-on-connector-tools)工具設定為 `ask`，該工具的 allow 規則不會生效：Claude Code 會在每次呼叫時提示，即使在 `auto` 和 `bypassPermissions` 模式中也是如此。在 `dontAsk` 模式中（永不提示），Claude Code 會改為拒絕呼叫。連接器工具顯示為 `mcp__claude_ai_<server>__<tool>`。

<h3 id="agent-subagents">
  Agent（subagents）
</h3>

使用 `Agent(AgentName)` 規則來控制 Claude 可以使用哪些 [subagents](/docs/zh-TW/sub-agents)：

* `Agent(Explore)` 符合 Explore subagent
* `Agent(Plan)` 符合 Plan subagent
* `Agent(my-custom-agent)` 符合名為 `my-custom-agent` 的自訂 subagent

將這些規則新增到您設定中的 `deny` 陣列，或使用 `--disallowedTools` CLI 旗標來停用特定代理。若要停用 Explore 代理：

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

`Cd` 規則控制 [`/cd` 命令](/docs/zh-TW/commands) 可以將工作階段移動到哪些目錄。`Cd` 不是模型可呼叫的工具：Claude 無法呼叫它，規則僅在您自己執行 `/cd` 時適用。

裸 `Cd` deny 規則會完全停用 `/cd`。`Cd(<path-pattern>)` deny 規則會阻止符合的目標。Deny 規則檢查目標的每個拼寫，包括它解析通過的每個符號連結跳躍，所以為一個路徑編寫的規則也會阻止解析到它的目標。

新增任何 `Cd` allow 規則會將 `/cd` 切換到允許清單模式：已解析的目標目錄必須符合您的其中一個 allow 規則，否則 `/cd` 會拒絕。未設定 `Cd` 規則時，`/cd` 會保持其預設行為並提示您信任不熟悉的目錄。

路徑模式共享來自 [Read 和 Edit 規則](#read-and-edit) 的 `//`、`~/` 和 `/` 錨點，但符合是錨定到整個目錄路徑而不是 gitignore 風格。`*` 符合恰好一個路徑段，`**` 符合跨段。尾部 `/**` 也符合其命名根。

| 規則                    | 符合                        | 不符合                       |
| --------------------- | ------------------------- | ------------------------- |
| `Cd(~/code/*)`        | `~/code/app`              | `~/code/app/src`、`~/code` |
| `Cd(~/code/**)`       | `~/code` 和其下的任何目錄         | `~/code` 外的目錄             |
| `Cd(**/node_modules)` | 任何深度的任何 `node_modules` 目錄 | `node_modules/pkg`        |

<h2 id="extend-permissions-with-hooks">
  使用 hooks 擴展權限
</h2>

[Claude Code hooks](/docs/zh-TW/hooks-guide) 提供了一種方式來註冊自訂 shell 命令，以在執行時執行權限評估。當 Claude Code 進行工具呼叫時，PreToolUse hooks 在權限提示之前執行。hook 輸出可以拒絕工具呼叫、強制提示或跳過提示以讓呼叫繼續進行。

Hook 決定不會繞過權限規則。Claude Code 會評估 deny 和 ask 規則，無論 PreToolUse hook 返回什麼：符合的 deny 規則會阻止呼叫，符合的 ask 規則即使在 hook 返回 `"allow"` 或 `"ask"` 時仍會提示。這保留了 [Manage permissions](#manage-permissions) 中描述的 deny 優先順序，包括在受管理設定中設定的 deny 規則。

連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具在 hook 返回 `"allow"` 時仍會提示。

阻止 hook 也優先於 allow 規則。以代碼 2 退出的 hook 會在評估權限規則之前停止工具呼叫，因此即使 allow 規則會允許呼叫，該阻止也會適用。若要執行所有 Bash 命令而無需提示，除了您想要阻止的少數幾個，請將 `"Bash"` 新增到您的 allow 清單，並註冊一個 PreToolUse hook 來拒絕那些特定命令。請參閱 [Block edits to protected files](/docs/zh-TW/hooks-guide#block-edits-to-protected-files) 以取得您可以調整的 hook 指令碼。

<h2 id="working-directories">
  工作目錄
</h2>

根據預設，Claude 可以存取啟動它的目錄中的檔案。您可以擴展此存取：

* **在啟動期間**：使用 `--add-dir <path>` CLI 引數
* **在工作階段期間**：使用 `/add-dir` 命令
* **持久設定**：新增到 [settings files](/docs/zh-TW/settings#settings-files) 中的 `additionalDirectories`

其他目錄中的檔案遵循與原始工作目錄相同的權限規則：它們變成可讀的而無需提示，檔案編輯權限遵循目前的權限模式。

在 macOS 的背景工作階段中，當 Claude 需要讀取或寫入檔案時，工作階段主機會分別從您的終端機要求存取受保護的資料夾，例如 `~/Desktop`、`~/Documents` 和 `~/Downloads`；如果讀取失敗並出現 `Operation not permitted`，請參閱[如何授予背景工作階段對資料夾的存取權](/docs/zh-TW/agent-view#background-sessions-can't-read-desktop-documents-or-downloads-on-macos)。

若要改變工作階段的主要工作目錄而不是新增另一個，請使用 [`/cd`](/docs/zh-TW/commands)。`/cd` 命令需要 Claude Code v2.1.169 或更新版本。與 `/add-dir` 不同，它會重新定位工作階段：新目錄的 `CLAUDE.md` 會被載入，而 `--resume` 會從該處找到工作階段。

<h3 id="additional-directories-grant-file-access-not-configuration">
  其他目錄授予檔案存取權，而非設定
</h3>

新增目錄會擴展 Claude 可以讀取和編輯檔案的位置。它不會使該目錄成為完整的設定根目錄：大多數 `.claude/` 設定不會從其他目錄發現，儘管有幾種類型作為例外被載入。

這些例外僅適用於使用 `--add-dir` 旗標或 `/add-dir` 命令新增的目錄。在設定檔中的 `permissions.additionalDirectories` 中列出的目錄僅授予檔案存取權，不會載入以下任何設定。

以下設定類型從 `--add-dir` 目錄載入：

| 設定                                                                                     | 從 `--add-dir` 載入                                                                                 |
| :------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| `.claude/skills/` 中的 [Skills](/docs/zh-TW/skills)                                           | 是，具有即時重新載入                                                                                       |
| `.claude/agents/` 中的 [Subagents](/docs/zh-TW/sub-agents)                                    | 是                                                                                                |
| `.claude/settings.json` 和 `.claude/settings.local.json` 中的 [Settings](/docs/zh-TW/settings) | 僅 `enabledPlugins` 和 `extraKnownMarketplaces` 金鑰                                                 |
| [CLAUDE.md](/docs/zh-TW/memory) 檔案、`.claude/rules/` 和 `CLAUDE.local.md`                     | 僅當設定 `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` 時。`CLAUDE.local.md` 另外需要 `local` 設定來源，預設啟用 |

命令和輸出樣式是從目前工作目錄及其父目錄、您在 `~/.claude/` 的使用者目錄和受管理設定發現的。Hooks 和其他 `settings.json` 金鑰從目前工作目錄的 `.claude/` 資料夾載入，沒有父目錄回退，同時也從您的使用者 `~/.claude/settings.json` 和受管理設定載入。若要在專案間共享該設定，請使用以下方法之一：

* **使用者級別設定**：將檔案放在 `~/.claude/agents/`、`~/.claude/output-styles/` 或 `~/.claude/settings.json` 中，使其在每個專案中可用
* **外掛**：將設定打包並分發為 [plugin](/docs/zh-TW/plugins)，供團隊安裝
* **從設定目錄啟動**：從包含您想要的 `.claude/` 設定的目錄執行 Claude Code

<h2 id="how-permissions-interact-with-sandboxing">
  權限如何與沙箱互動
</h2>

權限和 [sandboxing](/docs/zh-TW/sandboxing) 是互補的安全層：

* **權限**控制 Claude Code 可以使用哪些工具以及它可以存取哪些檔案或網域。它們適用於所有工具，包括 Bash、Read、Edit、WebFetch 和 MCP。
* **沙箱**提供作業系統級別的強制執行，限制 Bash 工具的檔案系統和網路存取。它僅適用於 Bash 命令及其子程序。

使用兩者進行深度防禦：

* 權限 deny 規則阻止 Claude 甚至嘗試存取受限資源
* 沙箱限制防止 Bash 命令到達定義邊界外的資源，即使提示注入繞過 Claude 的決策制定
* 沙箱中的檔案系統限制結合 [`sandbox.filesystem`](/docs/zh-TW/sandboxing) 設定與 Read 和 Edit deny 規則；兩者都合併到最終沙箱邊界中
* 網路限制結合 WebFetch 權限規則與沙箱的 `allowedDomains` 和 `deniedDomains` 清單

當沙箱啟用 `autoAllowBashIfSandboxed: true`（預設值）時，沙箱化 Bash 命令無需提示即可執行，即使您的權限包括 bare `Bash` ask 規則，或 [等效的 `Bash(*)` 形式](#match-all-uses-of-a-tool)：沙箱邊界替代整個工具提示。這些檢查仍然適用：

* 內容範圍的 ask 規則（如 `Bash(git push *)`）仍然強制提示
* 明確的 deny 規則仍然適用
* 針對 `/`、您的主目錄或其他關鍵系統路徑的 `rm` 或 `rmdir` 命令仍然會觸發提示

不會在沙箱中執行的命令（例如排除的命令）會遵守 bare `Bash` ask 規則。請參閱 [sandbox modes](/docs/zh-TW/sandboxing#sandbox-modes) 以變更此行為。

<h2 id="managed-settings">
  受管理設定
</h2>

對於需要集中控制 Claude Code 設定的組織，管理員可以部署無法被使用者或專案設定覆蓋的受管理設定。這些原則設定遵循與一般設定檔案相同的格式，可以透過 MDM/OS 級別原則、受管理設定檔案、[server-managed settings](/docs/zh-TW/server-managed-settings) 或自我託管的 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 傳遞。請參閱 [settings files](/docs/zh-TW/settings#settings-files) 以了解傳遞機制和檔案位置。

<h3 id="managed-only-settings">
  僅受管理的設定
</h3>

以下設定僅在受管理設定中有效。將它們放在使用者或專案設定檔案中沒有效果。

| 設定                                             | 描述                                                                                                                                                                                                                            |
| :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | 當為 `true` 時，claude.ai 連接器會與已部署的 `managed-mcp.json` 一起載入，而不是被其獨佔控制所抑制。請參閱 [Managed MCP configuration](/docs/zh-TW/managed-mcp)                                                                                                      |
| `allowedChannelPlugins`                        | 可能推送訊息的頻道外掛的允許清單。設定時替換預設 Anthropic 允許清單。需要 `channelsEnabled: true`。請參閱 [Restrict which channel plugins can run](/docs/zh-TW/channels#restrict-which-channel-plugins-can-run)                                                       |
| `allowManagedHooksOnly`                        | 當為 `true` 時，僅載入受管理 hooks、SDK hooks 和在受管理設定 `enabledPlugins` 中強制啟用的外掛中的 hooks。使用者、專案和所有其他外掛 hooks 被阻止                                                                                                                          |
| `allowManagedMcpServersOnly`                   | 當為 `true` 時，僅尊重受管理設定中的 `allowedMcpServers`。`deniedMcpServers` 仍然從所有來源合併。請參閱 [Managed MCP configuration](/docs/zh-TW/managed-mcp)                                                                                                   |
| `allowManagedPermissionRulesOnly`              | 當為 `true` 時，防止使用者和專案設定定義 `allow`、`ask` 或 `deny` 權限規則。僅套用受管理設定中的規則。不影響 MCP 伺服器允許清單；如需設定，請設定 `allowManagedMcpServersOnly`                                                                                                       |
| `blockedMarketplaces`                          | 市場來源的封鎖清單。在下載前檢查被封鎖的來源，因此它們永遠不會接觸檔案系統。請參閱 [managed marketplace restrictions](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)                                                                                     |
| `channelsEnabled`                              | 允許組織使用 [channels](/docs/zh-TW/channels)。請參閱 [enterprise controls](/docs/zh-TW/channels#enterprise-controls) 以了解每個方案的預設值                                                                                                                 |
| `disableSideloadFlags`                         | 在啟動時拒絕 `--plugin-dir`、`--plugin-url`、`--agents` 和 `--mcp-config` CLI 旗標。沒有這個設定，使用者可以透過傳遞這些旗標來為單次執行繞過 `strictKnownMarketplaces`。請參閱 [`disableSideloadFlags`](/docs/zh-TW/settings#available-settings)。需要 Claude Code v2.1.193 或更新版本 |
| `forceRemoteSettingsRefresh`                   | 當為 `true` 時，阻止 CLI 啟動直到遠端受管理設定被新鮮擷取，如果擷取失敗則退出。請參閱 [fail-closed enforcement](/docs/zh-TW/server-managed-settings#enforce-fail-closed-startup)                                                                                       |
| `pluginTrustMessage`                           | 自訂訊息，附加到安裝前顯示的外掛信任警告                                                                                                                                                                                                          |
| `sandbox.filesystem.allowManagedReadPathsOnly` | 當為 `true` 時，僅尊重受管理設定中的 `filesystem.allowRead` 路徑。`denyRead` 仍然從所有來源合併                                                                                                                                                         |
| `sandbox.network.allowManagedDomainsOnly`      | 當為 `true` 時，僅尊重來自受管理設定的 `allowedDomains` 和 `WebFetch(domain:...)` allow 規則。非允許的網域會自動被阻止，無需提示使用者。被拒絕的網域仍然從所有來源合併                                                                                                               |
| `strictKnownMarketplaces`                      | 控制使用者可以新增和安裝外掛的外掛市場來源。請參閱 [managed marketplace restrictions](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                     |
| `strictPluginOnlyCustomization`                | 阻止 skills、agents、hooks 和 MCP servers 來自使用者和專案來源，因此它們只能來自外掛或受管理設定。`true` 鎖定所有四個表面；像 `["skills", "hooks"]` 這樣的陣列僅鎖定命名的表面。請參閱 [`strictPluginOnlyCustomization`](/docs/zh-TW/settings#strictpluginonlycustomization)                   |
| `wslInheritsWindowsSettings`                   | 當在 Windows HKLM 登錄機碼或 `C:\Program Files\ClaudeCode\managed-settings.json` 中為 `true` 時，WSL 從 Windows 原則鏈以及 `/etc/claude-code` 讀取受管理設定。請參閱 [Settings files](/docs/zh-TW/settings#settings-files)                                     |

`disableBypassPermissionsMode` 通常放在受管理設定中以強制執行組織原則，但它可以從任何範圍工作。使用者可以在自己的設定中設定它以鎖定自己的繞過模式。

<Note>
  在 Team 和 Enterprise 方案上，管理員在 [Claude Code admin settings](https://claude.ai/admin-settings/claude-code) 中啟用或停用 [Remote Control](/docs/zh-TW/remote-control) 和 [web sessions](/docs/zh-TW/claude-code-on-the-web) 組織範圍內。Remote Control 可以另外透過 [`disableRemoteControl`](/docs/zh-TW/settings#available-settings) 受管理設定按裝置停用。Web sessions 沒有按裝置受管理設定金鑰。
</Note>

<h2 id="settings-precedence">
  設定優先順序
</h2>

權限規則遵循與所有其他 Claude Code 設定相同的 [settings precedence](/docs/zh-TW/settings#settings-precedence)：

1. **受管理設定**：無法被任何其他級別覆蓋，包括命令列引數
2. **命令列引數**：臨時工作階段覆蓋
3. **本機專案設定** (`.claude/settings.local.json`)
4. **共用專案設定** (`.claude/settings.json`)
5. **使用者設定** (`~/.claude/settings.json`)

如果工具在任何級別被拒絕，沒有其他級別可以允許它。例如，受管理設定 deny 無法被 `--allowedTools` 覆蓋，`--disallowedTools` 可以新增超出受管理設定定義的限制。

相同的規則也適用於設定範圍：如果使用者設定允許某項權限而專案設定拒絕它，deny 規則會阻止它。反之亦然：使用者級別的 deny 會阻止專案級別的 allow，因為來自任何範圍的 deny 規則會在 allow 規則之前進行評估。

嵌入主機可以在 [`parentSettingsBehavior`](/docs/zh-TW/settings#settings-precedence) 設定為 `"merge"` 時，透過 SDK `managedSettings` 選項提供額外的受管理原則；嵌入器值可以收緊原則但不能放寬它。

<h2 id="project-allow-rules-and-workspace-trust">
  專案允許規則和工作區信任
</h2>

`permissions.allow` 規則和專案 `.claude/settings.json` 中的 `permissions.additionalDirectories` 項目會授予功能，因此 Claude Code 只有在您接受該工作區的[工作區信任對話框](/docs/zh-TW/security#additional-safeguards)後才會套用這些規則。在此之前，Claude Code 會讀取規則但不會套用它們。信任對話框會列出資料夾將授予的允許規則和其他目錄，以便您在接受前進行檢查。`deny` 和 `ask` 規則不受影響，因為它們只會限制。

Claude Code 按工作區儲存信任，以 git 儲存庫根目錄為鍵，或在儲存庫外，以您啟動 Claude Code 的目錄為鍵。當您在主目錄中啟動時，信任僅在目前工作階段內保持，不會寫入磁碟；請參閱[額外保護措施](/docs/zh-TW/security#additional-safeguards)說明。信任父目錄不會套用巢狀專案的允許規則。

`.claude/settings.local.json` 是您自己的檔案，因此工作區信任檢查通常不適用於它。當儲存庫可能已提供該檔案時，例如當它被提交到 git 或 `.claude` 是符號連結時，其允許規則和其他目錄會經過信任檢查，就像專案設定一樣。

Claude Code 執行 git 來檢查儲存庫是否提供了該檔案，並且只在被接受的信任對話框涵蓋的資料夾中執行該檢查，無論是針對該資料夾還是其父目錄之一。在您尚未信任的資料夾中的互動工作階段中，`.claude/settings.local.json` 中的允許規則和其他目錄會經過信任檢查，就像專案設定一樣，直到您接受對話框，除非工作階段在您自己的設定主目錄中執行，如下所述。在以下兩個例外中，只有設定主目錄例外在對話框之前適用，因為它不需要執行 git。確定目錄不在 git 儲存庫內使用相同的 git 檢查，因此不在儲存庫內的例外在接受涵蓋該資料夾的信任對話框後生效。在 v2.1.207 之前，未追蹤的 `.claude/settings.local.json` 在您接受對話框之前在該資料夾中套用其允許規則。

`.claude/settings.local.json` 中的允許規則和其他目錄在兩種情況下也可以在沒有工作區信任的情況下套用：

* 您啟動 Claude Code 的目錄不在 git 儲存庫內。
* 工作階段在您自己的設定主目錄中執行：您的主目錄或任何您已設定為 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars) 的 `.claude` 子目錄。

在這兩種情況下，該檔案都是您建立的，而不是儲存庫可能提供的檔案，而儲存庫提交的 `.claude/settings.local.json` 仍然需要工作區信任。版本 2.1.196 至 2.1.199 在這些工作區中將該檔案視為儲存庫提供的檔案，忽略其允許規則，並向 stderr 列印[`this workspace has not been trusted`](/docs/zh-TW/errors#workspace-has-not-been-trusted)警告。上述兩個例外與 v2.1.195 及更早版本相符，並在 v2.1.200 中恢復。

同樣從 v2.1.200 開始，工作區的允許規則或其他目錄仍未被套用，但由於父目錄已被信任而從未顯示信任對話框，在您下次以互動方式在該處啟動 Claude Code 時會顯示對話框。對話框提供兩個選擇：

* **Yes, I trust this folder**：儲存該工作區的信任並在同一工作階段中套用規則。
* **No, continue without these permissions**：繼續工作，忽略這些規則。對話框將在下一個工作階段中再次出現。

在[非互動模式](/docs/zh-TW/headless)中使用 `-p`，不會出現對話框，規則保持被忽略。

<h2 id="example-configurations">
  範例設定
</h2>

此 [repository](https://github.com/anthropics/claude-code/tree/main/examples/settings) 包含常見部署情境的入門設定。使用這些作為起點並根據您的需求進行調整。

<h2 id="see-also">
  另請參閱
</h2>

* [Settings](/docs/zh-TW/settings)：完整設定參考，包括權限設定表
* [Configure auto mode](/docs/zh-TW/auto-mode-config)：告訴 auto mode 分類器您的組織信任哪些基礎設施
* [Sandboxing](/docs/zh-TW/sandboxing)：Bash 命令的作業系統級別檔案系統和網路隔離
* [Authentication](/docs/zh-TW/authentication)：設定使用者對 Claude Code 的存取
* [Security](/docs/zh-TW/security)：安全防護措施和最佳實踐
* [Hooks](/docs/zh-TW/hooks-guide)：自動化工作流程並擴展權限評估
