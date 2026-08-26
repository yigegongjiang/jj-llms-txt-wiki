> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 偵錯您的設定

> 診斷為什麼 CLAUDE.md、settings、hooks、MCP servers 或 skills 沒有生效。使用 /context、/doctor、/hooks 和 /mcp 查看實際載入的內容。

當 Claude 忽略您的指令或您設定的功能沒有出現時，通常是因為檔案沒有載入、從您預期以外的位置載入，或被另一個檔案覆蓋。本指南展示如何檢查 Claude Code 實際載入的內容，以便您縮小範圍。

如需安裝、驗證和連線問題的協助，請改為參閱 [Troubleshoot installation and login](/docs/zh-TW/troubleshoot-install)。

<h2 id="see-what-loaded-into-context">
  查看載入到 context 的內容
</h2>

`/context` 命令顯示佔用目前工作階段 context 視窗的所有內容，按類別細分：系統提示、記憶檔案、skills、自訂子代理及其載入來源、MCP tools 和對話訊息。首先執行它以確認您的 `CLAUDE.md`、規則或 skill 描述是否存在。

如需特定類別的詳細資訊，請使用專用命令進行後續操作：

| 命令               | 顯示                                                                               |
| :--------------- | :------------------------------------------------------------------------------- |
| `/memory`        | 載入了哪些 `CLAUDE.md` 和規則檔案，加上自動記憶項目                                                 |
| `/skills`        | 來自專案、使用者和外掛程式來源的可用 skills                                                        |
| `/hooks`         | 作用中的 hook 設定                                                                     |
| `/mcp`           | 已連線的 MCP servers 及其狀態                                                            |
| `/permissions`   | 目前生效的已解析允許和拒絕規則                                                                  |
| `/doctor`        | 設定檢查：安裝健康狀況、無效的設定檔案、未使用的擴充功能，以及同一目錄中重複的 [subagent](/docs/zh-TW/sub-agents) 名稱，並提出修復建議 |
| `/debug [issue]` | 啟用工作階段的偵錯日誌記錄，並提示 Claude 使用日誌輸出和設定路徑進行診斷                                         |
| `/status`        | 作用中的設定來源，包括是否啟用了受管設定                                                             |

如果記憶檔案在 `/memory` 中遺失，請根據 [CLAUDE.md 檔案如何載入](/docs/zh-TW/memory#how-claude-md-files-load) 檢查其位置。子目錄 `CLAUDE.md` 檔案在 Claude 使用 Read 工具讀取該目錄中的檔案時按需載入，而不是在工作階段開始時載入。

如果 `/memory` 確認檔案已載入但 Claude 仍未遵循特定指令，問題可能在於指令的編寫方式，而不是是否載入。CLAUDE.md 適用於您會給新隊友的指導類型，例如專案慣例、建置命令和檔案所在位置。

當指令模糊到可以多種方式解釋時、當兩個檔案給出衝突的方向時，或當檔案變得足夠長以至於個別規則獲得較少關注時，遵循度會下降。[編寫有效的指令](/docs/zh-TW/memory#write-effective-instructions) 涵蓋保持遵循度高的特異性、大小和結構模式。

<Note>
  CLAUDE.md 和 permissions 解決不同的問題。CLAUDE.md 告訴 Claude 您的專案如何運作，以便它做出良好決策。[Permissions](/docs/zh-TW/permissions) 和 [hooks](/docs/zh-TW/hooks) 無論 Claude 決定什麼，都會強制執行限制。使用 CLAUDE.md 表示「我們在這裡這樣做」。使用 permissions 或 hooks 表示安全邊界和任何必須永遠不會發生的事情，其中您需要保證而不是指導。
</Note>

<h2 id="check-resolved-settings">
  檢查已解析的設定
</h2>

設定在受管、使用者、專案和本機範圍之間合併。受管設定在存在時始終優先。在其餘的設定中，較近的範圍會按本機、專案、使用者的順序覆蓋較廣的範圍。某些設定也可以由命令列旗標或 [環境變數](/docs/zh-TW/env-vars) 設定，這些變數充當另一個覆蓋層。當設定似乎不適用時，您設定的值通常被另一個範圍或環境變數覆蓋。

執行 `/doctor` 以檢查您的設定和安裝。它會報告它發現的內容，包括無效的設定檔案、重複的安裝、未使用的擴充功能，以及 簽入的 `CLAUDE.md` 內容 Claude 可以從程式碼庫衍生，然後提議它在您確認後才會套用的修正。`CLAUDE.md` 修剪檢查需要 Claude Code v2.1.206 或更新版本。在 v2.1.205 之前，`/doctor` 會開啟唯讀診斷畫面，按 `f` 會將報告傳送給 Claude 以進行修正。

從終端機，`claude doctor` 會列印唯讀的安裝和設定診斷，而不會啟動工作階段。

執行 `/status` 以查看哪些設定來源處於作用中，包括是否啟用了受管設定。若要瞭解給定鍵的哪個範圍優先，請參閱 [範圍如何互動](/docs/zh-TW/settings#how-scopes-interact)。

<h2 id="check-mcp-servers">
  檢查 MCP servers
</h2>

執行 `/mcp` 以查看每個已設定的 server、其連線狀態，以及您是否已為目前專案核准它。server 可以定義正確但仍然不提供 tools，原因有幾個常見的：

* `.mcp.json` 中的專案範圍 servers 需要一次性核准。如果提示被關閉，server 將保持停用狀態，直到您從 `/mcp` 核准它。
* 啟動失敗的 server 在 `/mcp` 中顯示為失敗。`command` 或 `args` 中的相對檔案路徑是常見原因，因為它們相對於您啟動 Claude Code 的目錄而不是 `.mcp.json` 的位置進行解析。
* 顯示為已連線但列出零個 tools 的 server 已成功啟動但未返回 tool 清單。從 `/mcp` 選擇 **Reconnect**。如果計數保持為零，執行 `claude --debug mcp` 以查看 server 的 stderr 輸出。

如需設定位置和範圍規則，請參閱 [MCP](/docs/zh-TW/mcp)。

<h2 id="check-hooks">
  檢查 hooks
</h2>

執行 `/hooks` 以列出為目前工作階段註冊的每個 hook，按事件分組。如果您定義的 hook 沒有出現，則它未被讀取：hooks 位於設定檔案中的 `"hooks"` 鍵下，而不是在獨立檔案中。

如果 hook 出現但不觸發，通常是 matcher 的問題。檢查它是否有這些錯誤：

* `matcher` 欄位是一個使用 `|` 匹配多個 tool 名稱的單一字串，例如 `"Edit|Write"`。`,` 分隔符是等效的，因此 `"Edit,Write"` 匹配相同的 tools。在 v2.1.191 之前，逗號會進入正規表達式評估，matcher 永遠不會匹配，因此如果您不在 v2.1.191 版本上，請使用 `|`。
* 拼寫錯誤的 tool 名稱會產生一個不匹配任何內容的 matcher，因此 hook 會無聲地失敗。
* 陣列值是 schema 錯誤：Claude Code 顯示設定錯誤通知並拒絕整個使用者、專案或本機設定檔案，`claude doctor` 報告驗證失敗，該檔案中的任何 hook 都不會出現在 `/hooks` 中。在[受管設定](/docs/zh-TW/settings#settings-files)中，只有無效項目被刪除，檔案的其他 hooks 仍然適用。

對 `settings.json` 的編輯在短暫的檔案穩定延遲後在執行中的工作階段中生效。您不需要重新啟動。如果在保存後幾秒鐘 `/hooks` 仍顯示舊定義，請再次執行 `/hooks` 以重新整理檢視。

如果 `/hooks` 顯示 hook 但它仍然不觸發，下一步是即時監視 hook 評估。使用 `claude --debug hooks` 啟動工作階段並觸發 tool 呼叫。偵錯日誌記錄每個事件、檢查了哪些 matchers 以及 hook 的結束代碼和輸出。如需日誌格式，請參閱 [Debug hooks](/docs/zh-TW/hooks#debug-hooks)，如需常見失敗模式，請參閱 [hooks 疑難排解](/docs/zh-TW/hooks-guide#limitations-and-troubleshooting)。

<h2 id="test-against-a-clean-configuration">
  針對乾淨的設定進行測試
</h2>

使用 [`claude --safe-mode`](/docs/zh-TW/cli-reference#cli-flags) 開始，它會啟動一個工作階段，其中所有自訂項目都被停用，包括 `CLAUDE.md`、skills、plugins、hooks、MCP servers 和自訂命令與代理程式。驗證、模型選擇、內建工具和權限正常運作。如果問題在安全模式中消失，則其中一個表面是原因；使用上面的目標檢查來找出是哪一個。安全模式仍然會套用來自您組織的受管 hooks 和設定原則。受管 plugins、skills、`CLAUDE.md` 和 MCP servers 會被關閉。

如果問題在安全模式中持續存在，或您的設定本身令人懷疑，請與不從您常用設定載入任何內容的工作階段進行比較。將 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars) 指向空目錄以略過 `~/.claude` 下的所有內容，並從沒有 `.claude` 資料夾、`.mcp.json` 或 `CLAUDE.md` 的目錄啟動，以便也跳過專案設定。

```bash theme={null}
cd /tmp && CLAUDE_CONFIG_DIR=/tmp/claude-clean claude
```

乾淨的工作階段沒有使用者或專案設定、hooks、MCP servers、plugins 或記憶。

* 如果您的組織部署受管設定，它們仍然適用，因為它們位於 `~/.claude` 外的系統路徑
* 在 Linux 和 Windows 上，您將被提示再次登入，因為認證儲存在設定目錄下
* 在 macOS 上，認證在 Keychain 中，並會轉移到乾淨的工作階段

如果問題在此消失，原因在於您的真實 `~/.claude` 或專案 `.claude` 檔案中的某處。一次一個地重新引入它們，方法是將檔案複製到臨時目錄或從您的專案啟動，以找到哪一個。如果它在乾淨的工作階段中持續存在，原因在於您的使用者和專案設定之外。執行 `/status` 以檢查是否啟用了受管設定，查找影響 Claude Code 的 [環境變數](/docs/zh-TW/env-vars)，然後參閱 [Troubleshooting](/docs/zh-TW/troubleshooting)。

<h2 id="check-common-causes">
  檢查常見原因
</h2>

大多數設定意外可以追溯到一小組位置和語法規則。在假設有 bug 之前檢查這些：

| 症狀                                                       | 原因                                                                       | 修正                                                                                                                                                                  |
| :------------------------------------------------------- | :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Hook 永遠不觸發                                               | `matcher` 是 JSON 陣列而不是字串                                                 | 使用單一字串搭配 `\|` 來匹配多個 tools，例如 `"Edit\|Write"`。請參閱 [matcher 模式](/docs/zh-TW/hooks#matcher-patterns)。                                                                       |
| Hook 永遠不觸發                                               | `matcher` 在 v2.1.191 之前的版本中使用 `,` 作為分隔符                                  | Claude Code v2.1.191 或更新版本將 `,` 視為列表分隔符，如 `\|`。較早的版本將逗號評估為字面字元，因此 `"Edit,Write"` 不匹配任何內容。改用 `\|`，或升級 Claude Code。                                                   |
| Hook 永遠不觸發                                               | `matcher` 值是小寫，例如 `"bash"`                                               | 匹配區分大小寫。Tool 名稱是大寫的：`Bash`、`Edit`、`Write`、`Read`。                                                                                                                   |
| Hook 永遠不觸發                                               | Hooks 在獨立檔案而不是 `settings.json` 中定義                                       | 專案或使用者設定沒有獨立的 hooks 檔案。在 `settings.json` 中的 `"hooks"` 鍵下定義 hooks。只有 [plugins](/docs/zh-TW/plugins-reference#hooks) 載入獨立的 `hooks/hooks.json`。請參閱 [hook 設定](/docs/zh-TW/hooks)。 |
| 全域設定的 Permissions、hooks 或 env 被忽略                        | 設定已新增到 `~/.claude.json`                                                  | `~/.claude.json` 保存應用程式狀態和 UI 切換。`permissions`、`hooks` 和 `env` 屬於 `~/.claude/settings.json`。這是兩個不同的檔案。                                                              |
| `settings.json` 值似乎被忽略                                   | 相同的鍵在 `settings.local.json` 中設定                                          | `settings.local.json` 覆蓋 `settings.json`，兩者都覆蓋 `~/.claude/settings.json`。請參閱 [settings 優先順序](/docs/zh-TW/settings#how-scopes-interact)。                                  |
| Skill 不出現在 `/skills` 中                                   | Skill 檔案位於 `.claude/skills/name.md` 而不是在資料夾中                             | 使用包含 `SKILL.md` 的資料夾：`.claude/skills/name/SKILL.md`。                                                                                                                |
| Skill 出現在 `/skills` 中但 Claude 永遠不呼叫它                     | Skill 在其 frontmatter 中有 `disable-model-invocation: true`，或其描述與您表述請求的方式不符 | 檢查 `/skills` 中的徽章：「user-only」標籤表示 Claude 不會自動觸發它。請參閱 [skill 呼叫](/docs/zh-TW/skills)。                                                                                     |
| 子目錄 `CLAUDE.md` 指令似乎被忽略                                  | 子目錄檔案按需載入，而不是在工作階段開始時載入                                                  | 它們在 Claude 使用 Read 工具讀取該目錄中的檔案時載入，而不是在啟動時，也不是在寫入或建立檔案時。請參閱 [CLAUDE.md 檔案如何載入](/docs/zh-TW/memory#how-claude-md-files-load)。                                              |
| 子代理忽略 `CLAUDE.md` 指令                                     | 內建的 Explore 和 Plan 代理會跳過 `CLAUDE.md`。自訂子代理以與主對話相同的方式載入它                  | 對於 Explore 或 Plan，在您的委派提示中重新陳述指令。對於自訂子代理，將關鍵指令放在代理檔案主體中，該主體成為代理的系統提示。請參閱 [啟動時載入的內容](/docs/zh-TW/sub-agents#what-loads-at-startup)。                                       |
| 清理邏輯在工作階段結束時永遠不執行                                        | 未設定 `SessionEnd` hook                                                    | 在 `settings.json` 中新增 `SessionEnd` hook。請參閱 [hook 事件清單](/docs/zh-TW/hooks#hook-events)。                                                                                  |
| `.mcp.json` 中的 MCP servers 永遠不載入                         | 檔案位於 `.claude/` 下或使用 Claude Desktop 的設定格式                                | 專案 MCP 設定位於儲存庫根目錄為 `.mcp.json`，而不是在 `.claude/` 內。請參閱 [MCP 設定](/docs/zh-TW/mcp)。                                                                                          |
| 新增在 `settings.json` 中的 `mcpServers` 下的 MCP servers 永遠不出現 | `settings.json` 不讀取 `mcpServers` 鍵                                       | 在儲存庫根目錄的 `.mcp.json` 中定義專案 servers，或執行 `claude mcp add --scope user` 以取得使用者範圍的 servers。請參閱 [MCP 設定](/docs/zh-TW/mcp)。                                                    |
| 新增的專案 MCP server 但不出現                                    | 一次性核准提示被關閉                                                               | 專案範圍 servers 需要核准。執行 `/mcp` 以查看狀態並核准。                                                                                                                               |
| MCP server 從某些目錄啟動失敗                                     | `command` 或 `args` 使用相對檔案路徑                                              | 對本機指令碼使用絕對路徑。您 `PATH` 上的可執行檔（如 `npx` 或 `uvx`）可以按原樣使用。                                                                                                               |
| MCP server 啟動時沒有預期的環境變數                                  | 變數在 `settings.json` `env` 中，不會傳播到 MCP 子程序                                | 改為在 `.mcp.json` 內設定每個 server 的 `env`。                                                                                                                               |
| `Bash(rm *)` 拒絕規則不阻止 `/bin/rm` 或 `find -delete`          | 前綴規則匹配字面命令字串，而不是基礎可執行檔                                                   | 為每個變體新增明確模式，或使用 [PreToolUse hook](/docs/zh-TW/hooks-guide) 或 [sandbox](/docs/zh-TW/sandboxing) 以獲得硬保證。                                                                        |

<h2 id="related-resources">
  相關資源
</h2>

如需每個設定表面的完整參考，請參閱專用頁面：

* **[`.claude` 目錄參考](/docs/zh-TW/claude-directory)**：每個設定檔案位置及其讀取方式
* **[Settings](/docs/zh-TW/settings)**：優先順序和完整鍵清單
* **[Hooks 參考](/docs/zh-TW/hooks)**：事件名稱、承載和 `--debug hooks` 輸出格式
* **[MCP](/docs/zh-TW/mcp)**：server 設定、核准和 `/mcp` 輸出
* **[Troubleshoot installation and login](/docs/zh-TW/troubleshoot-install)**：`command not found`、PATH 和身份驗證問題
* **[Troubleshooting](/docs/zh-TW/troubleshooting)**：效能、掛起和搜尋問題
