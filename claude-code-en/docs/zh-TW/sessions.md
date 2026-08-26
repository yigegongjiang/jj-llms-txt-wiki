> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 管理 sessions

> 命名、恢復、分支和在 Claude Code 對話之間切換。涵蓋 `--continue`、`--resume`、`--from-pr`、`/resume` 選擇器、session 命名、匯出文字記錄，以及文字記錄的儲存位置。

session 是與專案目錄相關聯的已儲存對話。Claude Code 在您工作時將其儲存在本地，因此您可以從中斷的地方繼續、分支以嘗試不同的方法，或在任務之間切換。

[桌面應用程式](/docs/zh-TW/desktop#work-in-parallel-with-sessions)、[Claude Code 網頁版](/docs/zh-TW/claude-code-on-the-web)和 [VS Code 擴充功能](/docs/zh-TW/vs-code#resume-past-conversations)各自維護自己的 session 歷史記錄。本頁涵蓋 CLI。

<h2 id="resume-a-session">
  恢復 session
</h2>

Sessions 在您工作時會持續儲存到[本地文字記錄檔案](#export-and-locate-session-data)，因此您可以在退出或執行 `/clear` 後返回到一個。使用這些進入點：

| 命令                          | 功能                                        |
| :-------------------------- | :---------------------------------------- |
| `claude --continue`         | 恢復目前目錄中最近的 session                        |
| `claude --resume`           | 開啟 [session 選擇器](#use-the-session-picker) |
| `claude --resume <name>`    | 直接恢復命名的 session                           |
| `claude --from-pr <number>` | 恢復連結到該 pull request 的 session             |
| `/resume`                   | 從活躍 session 內切換到不同的對話                     |

使用 [`claude -p`](/docs/zh-TW/headless) 或 [Agent SDK](/docs/zh-TW/agent-sdk/overview) 建立的 sessions 不會出現在 session 選擇器中，但您仍然可以透過將其 session ID 傳遞給 `claude --resume <session-id>` 來恢復它。從啟動 session 的目錄執行此命令：session ID 查詢的範圍限於目前專案目錄及其 git worktrees，因此在其他地方建立的 session 會報告 `No conversation found with session ID: <session-id>`。

<h3 id="where-the-session-picker-looks">
  session 選擇器查看的位置
</h3>

Sessions 按專案目錄儲存。預設情況下，session 選擇器顯示來自目前 worktree 的互動式 sessions，以及在其他地方啟動並使用 `/add-dir` 新增目前目錄的 sessions。使用 `Ctrl+W` 擴展到儲存庫的所有 worktrees，或使用 `Ctrl+A` 擴展到此機器上的每個專案。

從 v2.1.169 開始，使用 [`/cd`](/docs/zh-TW/commands) 移動 session 會將其重新定位到新目錄的專案儲存空間，因此之後會出現在該目錄的選擇器中。從 v2.1.196 開始，移動的 session 即使在當機或強制退出後，也會保持不在舊目錄的選擇器中。在較早的版本上，當舊路徑包含特殊字元（例如底線）時，在不乾淨的退出後，它也可能在舊目錄的清單中重新出現。

從同一儲存庫的另一個 worktree 選擇 session 會在原地恢復它。從不相關的專案選擇 session 會將 `cd` 和恢復命令複製到您的剪貼簿。

按名稱恢復會在目前儲存庫及其 worktrees 中解析。兩種形式都會尋找完全相符的項目，並直接恢復它，即使它位於不同的 worktree 中：

| 命令                       | 完全相符 | 模糊名稱                                   |
| :----------------------- | :--- | :------------------------------------- |
| `claude --resume <name>` | 直接恢復 | 使用名稱預先填入作為搜尋詞開啟 session 選擇器            |
| `/resume <name>`         | 直接恢復 | 報告錯誤；執行不帶引數的 `/resume` 以開啟 session 選擇器 |

<h2 id="name-your-sessions">
  命名您的 sessions
</h2>

為 sessions 提供描述性名稱，以便在 session 選擇器中找到它們並按名稱恢復。當您並行處理多個任務時，這最為重要。

| 時間            | 如何設定名稱                                                                                                          |
| :------------ | :-------------------------------------------------------------------------------------------------------------- |
| 啟動時           | `claude -n auth-refactor`                                                                                       |
| 在 session 期間  | `/rename auth-refactor`。名稱也會出現在提示列上                                                                             |
| 從 session 選擇器 | 反白 session 並按 `Ctrl+R`                                                                                          |
| 在計畫接受時        | 在 [plan mode](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode) 中接受計畫會根據計畫內容命名 session，除非您已經設定了一個 |

session 命名後，使用 `claude --resume <name>` 或 `/resume <name>` 返回到它。請參閱[恢復 session](#resume-a-session) 以了解名稱解析在 worktrees 中的行為方式。

您從未命名的互動式 sessions 在啟動時仍會獲得預設顯示名稱。需要 Claude Code v2.1.196 或更新版本。預設名稱結合了工作目錄的名稱和一個兩字元的後綴，例如 `my-app-3f`，並在執行中 sessions 的列表中識別該 session，例如 [agent view](/docs/zh-TW/agent-view) 和 `claude agents --json` 輸出。

預設名稱不是恢復控制代碼：`claude --resume <name>`、`/resume <name>` 和 session 選擇器只符合您設定的名稱。命名 session 會取代預設名稱。

<h2 id="use-the-session-picker">
  使用 session 選擇器
</h2>

在 session 內執行 `/resume`，或不帶引數執行 `claude --resume`，以開啟互動式 session 選擇器。使用這些快捷鍵來導航、搜尋和擴展清單：

| 快捷鍵                      | 動作                                                                                                         |
| :----------------------- | :--------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                | 在 sessions 之間導航                                                                                            |
| `→` / `←`                | 展開或摺疊分組的 sessions                                                                                          |
| `Enter`                  | 恢復反白的 session                                                                                              |
| `Space`                  | 預覽 session 內容。在不將其捕獲為貼上的終端上也可以使用 `Ctrl+V`                                                                  |
| `Ctrl+R`                 | 重新命名反白的 session                                                                                            |
| `/` 或除 `Space` 外的任何可列印字元 | 進入搜尋模式並篩選 sessions。貼上 GitHub、GitHub Enterprise、GitLab 或 Bitbucket pull 或 merge request URL 以找到建立它的 session |
| `Ctrl+A`                 | 顯示此機器上所有專案的 sessions。再次按下以返回目前儲存庫                                                                          |
| `Ctrl+W`                 | 顯示目前儲存庫所有 worktrees 的 sessions。再次按下以返回目前 worktree。僅在多 worktree 儲存庫中顯示                                      |
| `Ctrl+B`                 | 篩選為目前 git 分支的 sessions。再次按下以顯示所有分支                                                                         |
| `Esc`                    | 退出 session 選擇器或搜尋模式                                                                                        |

每一列顯示 session 名稱（如果已設定），否則顯示對話摘要或第一個提示，以及自上次活動以來的時間、訊息計數和 git 分支。使用 `Ctrl+A` 擴展到所有專案後，專案路徑會出現。

使用 `/branch`、`/rewind` 或 `--fork-session` 建立的分支 sessions 會分組在其根 session 下。按 `→` 展開群組。

<h2 id="branch-a-session">
  分支 session
</h2>

分支會建立迄今為止對話的副本並將您切換到其中，保持原始對話完整。使用它來嘗試不同的方法，而不會失去您所在的路徑。

從 session 內，執行 `/branch` 並使用可選名稱：

```text theme={null}
/branch try-streaming-approach
```

如果您省略名稱，Claude Code 會根據對話中的第一個提示為新分支命名。從 v2.1.198 開始，這也適用於 [壓縮](/docs/zh-TW/how-claude-code-works#when-context-fills-up) 之後；較早的版本會回退到字面名稱 `Branched conversation`，而不是查看壓縮摘要之外的原始第一個提示。

從命令列，將 `--continue` 或 `--resume` 與 `--fork-session` 結合：

```bash theme={null}
claude --continue --fork-session
```

原始 session 保持不變，並在 session 選擇器中保持可用。`/branch` 確認會列印兩個 session ID：您現在所在的新分支和原始分支。要返回原始分支，將其 ID 傳遞給 `/resume`、使用 session 選擇器或執行 `/resume <original-name>`。您使用「允許此 session」核准的權限不會轉移到新分支。如果您在兩個終端中恢復同一 session 而不進行分支，來自兩者的訊息會交錯到一個文字記錄中。

有關單個 session 內基於 checkpoint 的 rewind，請參閱 [Checkpointing](/docs/zh-TW/checkpointing)。

<h2 id="manage-context-within-a-session">
  在 session 內管理上下文
</h2>

這些命令控制上下文視窗中的內容，而無需離開 session：

* **`/clear`**：以空上下文重新開始。先前的對話已儲存並可恢復，使用 `/resume` 恢復，或在同一個 Claude Code 程序中，從[倒帶選單的前一個 session 項目](/docs/zh-TW/checkpointing#rewind-past-a-cleared-conversation)
* **`/compact [instructions]`**：用摘要替換歷史記錄，可選擇性地專注於您指定的內容
* **`/context`**：顯示目前消耗上下文的內容

有關壓縮如何與 CLAUDE.md、skills 和規則互動，請參閱[上下文視窗指南](/docs/zh-TW/context-window)。有關何時清除與壓縮的策略，請參閱[最佳實踐](/docs/zh-TW/best-practices#manage-your-session)。

<h2 id="export-and-locate-session-data">
  匯出和定位 session 資料
</h2>

執行 `/export` 以開啟一個選單，讓您將目前對話複製到剪貼簿或將其儲存為純文字檔案，訊息和工具輸出呈現為可讀文字。傳遞檔案名以略過選單並直接寫入該檔案。

<h3 id="access-conversations-from-scripts">
  從指令碼存取對話
</h3>

`/export` 產生供人閱讀的呈現文字記錄。下列介面產生供指令碼解析的結構化資料：執行的 JSON 結果、session 文字記錄檔案的路徑，或事件的即時串流。根據觸發指令碼的內容選擇：

* **執行 Claude 一次並擷取結果**：使用 [`--output-format json` 或 `stream-json`](/docs/zh-TW/headless#get-structured-output) 叫用 `claude -p`，以將非互動執行的結果、session ID、使用情況和成本擷取為結構化 JSON。
* **詢問現有 session 一個問題**：將 session ID 傳遞給 [`claude -p --resume`](/docs/zh-TW/headless#continue-conversations)，以傳送後續提示（例如摘要要求），並擷取結構化回應。
* **對 session 事件做出反應**：讀取 [hooks](/docs/zh-TW/hooks#common-input-fields) 和 [status line commands](/docs/zh-TW/statusline#available-data) 作為輸入接收的 `transcript_path` 欄位。`SessionEnd` hook 可在 session 結束時封存文字記錄。
* **在 TypeScript 或 Python 應用程式中嵌入 Claude**：使用 [Agent SDK](/docs/zh-TW/agent-sdk/overview) 以程式設計方式接收每條訊息。

下列範例使用第二個介面。它傳送後續提示給現有 session，並使用 `jq` 讀取答案：

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  文字記錄儲存位置
</h3>

根據預設，文字記錄儲存為 JSONL，位置為 `~/.claude/projects/<project>/<session-id>.jsonl`，其中 `<project>` 是您的工作目錄路徑，非英數字元已被 `-` 取代。每一行都是訊息、工具使用或中繼資料項目的 JSON 物件。項目格式是 Claude Code 的內部格式，在版本之間會變更，因此直接解析這些檔案的指令碼可能在任何版本上中斷。若要建立在 session 資料上，請改用 `/export` 或 [指令碼介面](#access-conversations-from-scripts)。

位置、保留期和寫入行為可設定：

| 目的                  | 設定                                                        | 位置                      |
| ------------------- | --------------------------------------------------------- | ----------------------- |
| 將儲存空間移出 `~/.claude` | [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars)                    | 環境變數                    |
| 變更 30 天保留期          | [`cleanupPeriodDays`](/docs/zh-TW/settings#available-settings) | `settings.json`         |
| 在所有模式中禁止文字記錄寫入      | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/zh-TW/env-vars)      | 環境變數                    |
| 禁止一次非互動執行的寫入        | [`--no-session-persistence`](/docs/zh-TW/cli-reference)        | 搭配 `claude -p` 的 CLI 旗標 |

<h2 id="see-also">
  另請參閱
</h2>

這些頁面涵蓋相關的 session 和平行處理機制：

* [Worktrees](/docs/zh-TW/worktrees)：在單獨的分支上執行隔離的平行 sessions
* [Checkpointing](/docs/zh-TW/checkpointing)：將程式碼和對話 rewind 到較早的點
* [Context window](/docs/zh-TW/context-window)：什麼填充上下文以及什麼在壓縮中存活
* [Non-interactive mode](/docs/zh-TW/headless)：`claude -p` 下的 session 行為
