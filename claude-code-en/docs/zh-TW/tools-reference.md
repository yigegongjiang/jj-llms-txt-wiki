> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 工具參考

> Claude Code 可以使用的工具的完整參考，包括權限要求和各工具行為。

Claude Code 可以存取一組內建工具，幫助它理解和修改您的程式碼庫。工具名稱是您在 [權限規則](/docs/zh-TW/permissions#tool-specific-permission-rules)、[subagent 工具清單](/docs/zh-TW/sub-agents) 和 [hook 匹配器](/docs/zh-TW/hooks) 中使用的確切字串。若要完全停用工具，請將其名稱新增到您的 [權限設定](/docs/zh-TW/permissions#tool-specific-permission-rules) 中的 `deny` 陣列。

若要新增自訂工具，請連接 [MCP server](/docs/zh-TW/mcp)。若要使用可重複使用的提示型工作流程擴展 Claude，請撰寫 [skill](/docs/zh-TW/skills)，它透過現有的 `Skill` 工具執行，而不是新增工具項目。

Permission required 欄位顯示工具是否在預設權限模式下針對工作目錄內的路徑進行提示。標記為「否」的檔案存取工具（包括 `Read`、`Grep` 和 `Glob`）仍會針對 [工作目錄和其他目錄](/docs/zh-TW/permissions#working-directories) 外的路徑進行提示。`Bash` 標記為「是」，但執行內建的 [唯讀命令](/docs/zh-TW/permissions#read-only-commands) 集合，無需提示。

| 工具                     | 描述                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | 需要權限 |
| :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--- |
| `Agent`                | 生成一個具有自己 context window 的 [subagent](/docs/zh-TW/sub-agents)，以處理任務。請參閱 [Agent 工具行為](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                           | 否    |
| `Artifact`             | 將 HTML 或 Markdown 檔案發佈為 [artifact](/docs/zh-TW/artifacts)：一個私人的互動式頁面，在 claude.ai 上。在 Team 和 Enterprise 計畫上，您可以在組織內部分享。需要 Pro、Max、Team 或 Enterprise 計畫和 `/login` 驗證；請參閱 [可用性](/docs/zh-TW/artifacts#availability)                                                                                                                                                                                                                                                                                                                         | 是    |
| `AskUserQuestion`      | 提出多選題以收集需求或澄清歧義。問題會保持開啟直到您回答：預設情況下沒有閒置逾時。若要讓閒置對話框自動繼續，請將 [`askUserQuestionTimeout`](/docs/zh-TW/settings#available-settings) 設定設為 `60s`、`5m` 或 `10m`，可在您的使用者 `settings.json` 中或從 `/config` 中的**問題自動繼續逾時**列進行設定。一旦經過所選的閒置時間且沒有輸入，對話框會自動關閉：它會提交您已選擇的任何選項，並告訴 Claude 您可能離開了鍵盤，因此 Claude 會根據自己的判斷進行，稍後可以重新提問。最後 20 秒會出現倒數計時。任何按鍵都會重新啟動計時器，報告焦點的終端機上的焦點視窗也會。逾時僅適用於 `AskUserQuestion` 的多選題；權限提示（包括計畫批准）在閒置時永遠不會自動解決。在 v2.1.198 和 v2.1.199 中，對話框預設在 60 秒閒置後自動繼續，[`CLAUDE_AFK_TIMEOUT_MS`](/docs/zh-TW/env-vars#variables) 是唯一改變該行為的方式           | 否    |
| `Bash`                 | 在您的環境中執行 shell 命令。請參閱 [Bash 工具行為](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | 是    |
| `CronCreate`           | 在目前工作階段內排程定期或一次性提示。任務的範圍限於工作階段，並在 `--resume` 或 `--continue` 時恢復（如果未過期）。請參閱 [排程任務](/docs/zh-TW/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                      | 否    |
| `CronDelete`           | 按 ID 取消排程任務                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | 否    |
| `CronList`             | 列出工作階段中的所有排程任務                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | 否    |
| `Edit`                 | 對特定檔案進行目標編輯。請參閱 [Edit 工具行為](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | 是    |
| `EnterPlanMode`        | 切換到 Plan Mode 以在編碼前設計方法                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | 否    |
| `EnterWorktree`        | 建立隔離的 [git worktree](/docs/zh-TW/worktrees) 並切換到其中。傳遞 `path` 以切換到現有 worktree，而不是建立新的。首次進入時，目標可能是目前儲存庫的 worktree，或在多儲存庫工作區中，是其中嵌套的儲存庫的 worktree。在 v2.1.203 之前，嵌套儲存庫的 worktree 會被拒絕。`.claude/worktrees/` 外的 `path` 會在進入前提示您的批准，因為它會移動工作階段的工作目錄和寫入存取權限到該位置。新 worktree 建立和 `.claude/worktrees/` 下的路徑不會提示。在 v2.1.206 之前，Claude 進入 `.claude/worktrees/` 外的路徑時不會提示。從 worktree 工作階段內，或從具有固定工作目錄的 subagent（例如 [`isolation: worktree`](/docs/zh-TW/sub-agents#supported-frontmatter-fields)）中，只有 `path` 形式可用，且目標必須在工作階段儲存庫的 `.claude/worktrees/` 下 | 是    |
| `ExitPlanMode`         | 提出計畫以供批准並退出 Plan Mode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | 是    |
| `ExitWorktree`         | 退出 worktree 工作階段並返回原始目錄。不適用於已在自己的工作目錄中執行的 subagents，例如使用 [`isolation: worktree`](/docs/zh-TW/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                               | 否    |
| `Glob`                 | 根據模式匹配查找檔案。請參閱 [Glob 工具行為](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 否    |
| `Grep`                 | 在檔案內容中搜尋模式。請參閱 [Grep 工具行為](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 否    |
| `ListMcpResourcesTool` | 列出連接的 [MCP servers](/docs/zh-TW/mcp) 公開的資源                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | 否    |
| `LSP`                  | 透過語言伺服器進行程式碼智慧：跳轉到定義、尋找參考、報告型別錯誤和警告。請參閱 [LSP 工具行為](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                         | 否    |
| `Monitor`              | 在背景執行命令，並將每個輸出行回饋給 Claude，以便它可以對日誌項目、檔案變更或輪詢狀態做出反應。也可以開啟 WebSocket 並將每個傳入訊息視為事件。請參閱 [Monitor 工具](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                | 是    |
| `NotebookEdit`         | 修改 Jupyter notebook 儲存格。請參閱 [NotebookEdit 工具行為](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 是    |
| `PowerShell`           | 原生執行 PowerShell 命令。請參閱 [PowerShell 工具](#powershell-tool) 以了解可用性                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 是    |
| `PushNotification`     | 傳送桌面通知，以及當 [Remote Control](/docs/zh-TW/remote-control) 已連接時傳送手機推播，以便長時間執行的任務或 [排程任務](/docs/zh-TW/scheduled-tasks) 可以在您離開時聯繫您。推播傳遞透過 Anthropic 託管的基礎設施執行，無法從 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 存取                                                                                                                                                                                                                                                                                                         | 否    |
| `Read`                 | 讀取檔案的內容。請參閱 [Read 工具行為](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 否    |
| `ReadMcpResourceTool`  | 按 URI 讀取特定 MCP 資源                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | 否    |
| `RemoteTrigger`        | 在 claude.ai 上建立、更新、執行和列出 [Routines](/docs/zh-TW/routines)。支援 `/schedule` 命令。Routines 位於 claude.ai 上，需要 Pro、Max、Team 或 Enterprise 計畫，因此此工具無法從 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 存取                                                                                                                                                                                                                                                                                                                    | 否    |
| `ReportFindings`       | 將程式碼審查結果報告為結構化清單，每個結果包含檔案、摘要和失敗情景，以便 Claude Code 可以呈現它們而不是將其列印為文字。當有效的程式碼審查指示告訴它時，Claude 會呼叫它。需要 Claude Code v2.1.196 或更新版本。自 v2.1.199 起，結果也可以攜帶選擇性的 `category` slug，例如 `correctness` 或 `test-coverage`，顯示在呈現清單中的檔案位置旁邊                                                                                                                                                                                                                                                                                                        | 否    |
| `ScheduleWakeup`       | 重新排程 [自主進行的 `/loop`](/docs/zh-TW/scheduled-tasks#let-claude-choose-the-interval) 的下一次迭代。Claude 在每次迭代結束時呼叫此工具，以選擇下一次執行的時間，範圍在一分鐘到一小時之間；您不需要直接呼叫它。若要改為結束迴圈，Claude 會以 `stop: true` 呼叫它，這會取消待處理的喚醒。`stop` 欄位需要 Claude Code v2.1.202 或更新版本。待處理的喚醒會出現在 [Stop hook input](/docs/zh-TW/hooks#stop-input) 的 `session_crons` 中。在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用，其中沒有間隔的 `/loop` 提示會改為按固定時間表執行                                                                                        | 否    |
| `SendMessage`          | 傳送訊息給 [agent team](/docs/zh-TW/agent-teams) 隊友，或按 agent ID 或名稱 [恢復 subagent](/docs/zh-TW/sub-agents#resume-subagents)。已完成的 subagent 會在背景中自動恢復；您從 `/tasks` 停止的 subagent 不會，且呼叫會傳回拒絕。結構化的團隊協議訊息需要 agent teams。接收者永遠不會將來自另一個 agent 的訊息視為您的同意或批准。自 v2.1.198 起，subagent 將來自啟動它的 agent 的訊息視為正常任務指示，而不是對等請求。自 v2.1.199 起，傳送到現在解析為與對話中較早時間不同的 agent 的名稱會被拒絕而不是傳遞；請參閱 [恢復 subagents](/docs/zh-TW/sub-agents#resume-subagents)                                                                                                                          | 否    |
| `SendUserFile`         | 將工作階段中的檔案傳送給您，並附上選擇性標題，以便生成的報告、圖表、螢幕擷取畫面或建置的成品可以到達您的裝置，而不是只在文字記錄中提及。自 v2.1.196 起，選擇性的 `display` 輸入控制呈現方式：`render` 在用戶端中內聯開啟檔案，`attach` 僅顯示下載卡片，未設定時用戶端會根據檔案類型決定。當連接了 [Remote Control](/docs/zh-TW/remote-control) 用戶端或工作階段在受管雲端環境（例如 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)）中執行時可用。傳遞透過 Anthropic 託管的基礎設施執行，因此該工具在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用                                                                                                                           | 否    |
| `ShareOnboardingGuide` | 上傳 `ONBOARDING.md` 並傳回隊友可以在 Claude Code 中開啟的分享連結。在撰寫指南後從 `/team-onboarding` 呼叫。適用於 Pro、Max、Team 和 Enterprise 計畫上的 claude.ai 訂閱者                                                                                                                                                                                                                                                                                                                                                                                                | 是    |
| `Skill`                | 在主對話中執行 [skill](/docs/zh-TW/skills#control-who-invokes-a-skill)                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | 是    |
| `TaskCreate`           | 在任務清單中建立新任務                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | 否    |
| `TaskGet`              | 檢索特定任務的完整詳細資訊                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | 否    |
| `TaskList`             | 列出所有任務及其目前狀態                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 否    |
| `TaskOutput`           | 檢索背景任務的輸出。已在任務的輸出檔案路徑上使用 `Read` 取代。當沒有任務符合 ID 時，錯誤會列出執行中的背景 agents（按 ID 和描述）。在 v2.1.203 之前，錯誤只命名遺失的 ID                                                                                                                                                                                                                                                                                                                                                                                                                         | 否    |
| `TaskStop`             | 按 ID 終止執行中的背景任務。它也接受 [agent-team 隊友](/docs/zh-TW/agent-teams) 或按 agent ID 或名稱的具名背景 agent。在 v2.1.198 之前，它只接受背景任務 ID。當沒有任務符合 ID 時，錯誤會列出執行中的背景 agents（按 ID 和描述），包括另一個 agent 生成的 agents。在 v2.1.203 之前，錯誤列出執行中的隊友和具名 agents，但不包括另一個 agent 生成的背景 agents，因此無法從主對話中識別或停止這些 agents                                                                                                                                                                                                                                                           | 否    |
| `TaskUpdate`           | 更新任務狀態、依賴項、詳細資訊或刪除任務                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | 否    |
| `TodoWrite`            | 管理工作階段任務檢查清單。自 v2.1.142 起預設停用，改用 `TaskCreate`、`TaskGet`、`TaskList` 和 `TaskUpdate`。設定 `CLAUDE_CODE_ENABLE_TASKS=0` 以重新啟用                                                                                                                                                                                                                                                                                                                                                                                                        | 否    |
| `ToolSearch`           | 當啟用 [tool search](/docs/zh-TW/mcp#scale-with-mcp-tool-search) 時，搜尋並載入延遲工具                                                                                                                                                                                                                                                                                                                                                                                                                                                           | 否    |
| `WaitForMcpServers`    | 等待一個或多個仍在背景連接的 [MCP servers](/docs/zh-TW/mcp)，以便請求可以使用其工具而無需重新啟動工作階段。Claude 會在所需的伺服器尚未連接時呼叫它。僅在停用 [tool search](/docs/zh-TW/mcp#scale-with-mcp-tool-search) 時出現，因為啟用時 `ToolSearch` 會處理等待                                                                                                                                                                                                                                                                                                                                                 | 否    |
| `WebFetch`             | 從指定 URL 擷取內容。請參閱 [WebFetch 工具行為](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | 是    |
| `WebSearch`            | 執行網路搜尋。請參閱 [WebSearch 工具行為](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | 是    |
| `Workflow`             | 執行 [dynamic workflow](/docs/zh-TW/workflows)：一個在背景協調許多 subagents 並傳回一個統一結果的指令碼                                                                                                                                                                                                                                                                                                                                                                                                                                                      | 是    |
| `Write`                | 建立或覆寫檔案。請參閱 [Write 工具行為](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | 是    |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  使用權限規則和 hooks 設定工具
</h2>

在大多數情況下，Claude 會決定何時使用這些工具，您在與 Claude 互動時不需要自己命名它們。當定義權限和其他設定時，您直接參考工具名稱：

* 在設定中的 [`permissions.allow` 和 `permissions.deny`](/docs/zh-TW/settings#available-settings)，以及 `/permissions` 介面
* 在 `--allowedTools` 和 `--disallowedTools` [CLI 旗標](/docs/zh-TW/cli-reference) 中
* 在 Agent SDK 的 [`allowedTools` 和 `disallowedTools`](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules) 選項中
* 在 [subagent 的 `tools` 或 `disallowedTools`](/docs/zh-TW/sub-agents#supported-frontmatter-fields) frontmatter 中
* 在 [skill 的 `allowed-tools`](/docs/zh-TW/skills#frontmatter-reference) frontmatter 中
* 在 hook 的 [`if` 條件](/docs/zh-TW/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) 中

所有這些都接受相同的規則格式 `ToolName(specifier)`。specifier 取決於工具，多個工具共享一種格式：

| 規則格式                           | 適用於                     | 詳細資訊                                                               |
| :----------------------------- | :---------------------- | :----------------------------------------------------------------- |
| `Bash(npm run *)`              | Bash、Monitor            | [命令模式匹配](/docs/zh-TW/permissions#bash)                                  |
| `PowerShell(Get-ChildItem *)`  | PowerShell              | [命令模式匹配](/docs/zh-TW/permissions#powershell)                            |
| `Read(~/secrets/**)`           | Read、Grep、Glob、LSP      | [路徑模式匹配](/docs/zh-TW/permissions#read-and-edit)                         |
| `Edit(/src/**)`                | Edit、Write、NotebookEdit | [路徑模式匹配](/docs/zh-TW/permissions#read-and-edit)                         |
| `Skill(deploy *)`              | Skill                   | [Skill 名稱匹配](/docs/zh-TW/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                   | [Subagent 類型匹配](/docs/zh-TW/permissions#agent-subagents)                |
| `WebFetch(domain:example.com)` | WebFetch                | [網域匹配](/docs/zh-TW/permissions#webfetch)                                |
| `WebSearch`                    | WebSearch               | 無 specifier；允許或拒絕整個工具                                              |

此處未列出的工具，例如 `ExitPlanMode` 或 `ShareOnboardingGuide`，僅接受不帶 specifier 的裸工具名稱。

`Edit(...)` 允許規則也授予對相同路徑的讀取存取權，因此您不需要匹配的 `Read(...)` 規則。`Read(...)` 拒絕規則也會在相同路徑上阻止 Edit 工具，包括在該處建立新檔案，因為編輯需要讀取結果。Edit 上的 `Read` 拒絕檢查需要 Claude Code v2.1.208 或更新版本。

Hook `matcher` 欄位使用裸工具名稱，而不是括號括起的規則格式。請參閱 [matcher 模式](/docs/zh-TW/hooks#matcher-patterns) 以了解匹配規則。如需每個工具在 hooks 中傳遞給 `tool_input` 的欄位名稱，請參閱 [PreToolUse 輸入參考](/docs/zh-TW/hooks#pretooluse-input)。

<h2 id="agent-tool-behavior">
  Agent 工具行為
</h2>

Agent 工具在單獨的 context window 中生成一個 subagent。subagent 自主地完成其任務，然後將單個文字結果傳回父對話。父對話看不到 subagent 的中間工具呼叫或輸出，只看到最終結果。

若要限制 subagent 執行的轉數，請在 [subagent 定義](/docs/zh-TW/sub-agents#supported-frontmatter-fields) 中設定 `maxTurns`。

相同的 Agent 工具也會在啟用 fork 模式時啟動 [forked subagents](/docs/zh-TW/sub-agents#fork-the-current-conversation)。fork 繼承完整的父對話，而不是從頭開始，始終在背景執行，並仍在您的終端中顯示權限提示。本節的其餘部分描述命名的 subagents。

命名的 subagent 可以使用哪些工具取決於 [subagent 定義](/docs/zh-TW/sub-agents) 中的 `tools` 和 `disallowedTools` 欄位：

* **兩個欄位都未設定**：subagent 繼承父對話可用的每個工具。
* **僅 `tools`**：subagent 僅獲得列出的工具。
* **僅 `disallowedTools`**：subagent 獲得除列出的工具外的每個父工具。
* **兩者都設定**：`disallowedTools` 優先。同時列在兩者中的工具會被移除。

當 subagent 的 `tools` 清單解析為完全沒有工具時，例如因為每個項目都拼寫錯誤或命名了一個對 subagents 不可用的工具，Agent 工具會傳回一個錯誤，列出這些項目，而不是啟動 subagent。在 v2.1.208 之前，subagent 會以無工具的方式啟動，並可能傳回空白或令人困惑的結果。

啟動 subagent 本身不會提示權限。Claude Code 在執行時會根據您的權限規則檢查 subagent 自己的工具呼叫。

自 v2.1.198 起，subagents 預設在背景執行；當 Claude 需要結果才能繼續時，會在前景執行一個。

* **前景 subagents** 顯示您在主對話中會看到的相同權限提示，在每個工具呼叫發生時。
* **背景 subagents** 自 v2.1.186 起在您的主要工作階段中顯示權限提示。提示會指出哪個 subagent 在要求，按下 Esc 會拒絕該單一工具呼叫，而不會停止 subagent。在 v2.1.186 之前，背景 subagents 會自動拒絕任何否則會提示的工具呼叫，並在沒有該工具的情況下繼續。

若要首先限制 subagent 可以到達的內容，請縮小其 `tools` 欄位、將 Bash 排除在清單之外，或在您的設定中設定拒絕規則，如 [控制 subagent 功能](/docs/zh-TW/sub-agents#control-subagent-capabilities) 中所述。如需有關選擇前景或背景的更多資訊，請參閱 [在前景或背景中執行 subagents](/docs/zh-TW/sub-agents#run-subagents-in-foreground-or-background)。

<h2 id="bash-tool-behavior">
  Bash 工具行為
</h2>

Bash 工具在單獨的程序中執行每個命令，具有以下持久性行為：

* 當 Claude 在主工作階段中執行 `cd` 時，只要新的工作目錄保持在專案目錄內或您使用 `--add-dir`、`/add-dir` 或設定中的 `additionalDirectories` 新增的 [額外工作目錄](/docs/zh-TW/permissions#working-directories) 內，新的工作目錄就會延續到後續的 Bash 命令。Subagent 工作階段永遠不會延續工作目錄變更。
  * 如果 `cd` 落在這些目錄之外，Claude Code 會重設為專案目錄，並將 `Shell cwd was reset to <dir>` 附加到工具結果。
  * 若要停用此延續，使每個 Bash 命令都在專案目錄中啟動，請設定 `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`。
* 環境變數不持久化。一個命令中的 `export` 在下一個命令中將不可用。
* 在您的 shell 啟動檔案中定義的別名和 shell 函式可用。在工作階段開始時，Claude Code 會來源 `~/.zshrc`、`~/.bashrc` 或 `~/.profile`（取決於您的 shell），擷取產生的別名、函式和 shell 選項，並將其應用於每個 Bash 命令。

在啟動 Claude Code 之前啟動您的 virtualenv 或 conda 環境。若要讓環境變數在 Bash 命令之間持久化，請在啟動 Claude Code 之前將 [`CLAUDE_ENV_FILE`](/docs/zh-TW/env-vars) 設定為 shell 指令碼，或使用 [SessionStart hook](/docs/zh-TW/hooks#persist-environment-variables) 動態填充它。

兩個限制限制每個命令：

* **逾時**：預設為兩分鐘。Claude 可以使用 `timeout` 參數要求每個命令最多 10 分鐘。使用 [`BASH_DEFAULT_TIMEOUT_MS` 和 `BASH_MAX_TIMEOUT_MS`](/docs/zh-TW/env-vars) 覆寫預設值和上限。
* **輸出長度**：預設為 30,000 個字元。當命令產生超過該值的輸出時，Claude Code 會將完整輸出儲存到工作階段目錄中的檔案，並給予 Claude 檔案路徑加上開始處的簡短預覽。Claude 在需要其餘部分時讀取或搜尋該檔案。使用 [`BASH_MAX_OUTPUT_LENGTH`](/docs/zh-TW/env-vars) 提高限制，最高可達 150,000 個字元的硬上限。

對於長時間執行的程序（例如開發伺服器或監視建置），Claude 可以設定 `run_in_background: true` 以將命令作為背景任務啟動，並在其執行時繼續工作。使用 `/tasks` 列出和停止背景任務。在使用 `-p` 旗標的非互動模式中，[背景任務在執行的最終結果後不久結束](/docs/zh-TW/headless#background-tasks-at-exit)。

<h2 id="edit-tool-behavior">
  Edit 工具行為
</h2>

Edit 工具執行確切的字串替換。它採用 `old_string` 和 `new_string` 並用第二個替換第一個。它不使用正規表達式或模糊匹配。

三個檢查必須通過才能應用編輯。在任何檢查之前，由 [`Read` 拒絕規則](/docs/zh-TW/permissions#tool-specific-permission-rules) 匹配的路徑會被拒絕，包括在該處建立新檔案。此拒絕需要 Claude Code v2.1.208 或更新版本。

* **編輯前讀取**：Claude 在目前對話中讀取檔案後才編輯它，且以 [`PARTIAL view` 通知](#read-tool-behavior) 中斷的讀取不計算。Claude Opus 4.6、Claude Haiku 4.5 和更舊的模型始終需要讀取。較新的模型可以在讀取不需要權限提示且 Read 工具可用時編輯未讀檔案。
* **匹配**：`old_string` 必須在檔案中完全按照撰寫方式出現。單個空白字元或縮排差異足以導致不匹配。
* **唯一性**：`old_string` 必須恰好出現一次。當它出現多次時，Claude 要麼提供更長的字串，其周圍有足夠的上下文來確定一個出現，要麼設定 `replace_all: true` 以替換所有出現。

在 Claude 最後讀取檔案後在磁碟上變更的檔案仍然可以編輯，當 `old_string` 與目前內容完全且明確匹配，且 Claude Code 可以讀取檔案而不提示時。針對檔案的目前內容進行匹配可保持安全，結果會注意到檔案包含其他變更，因此 Claude 在依賴周圍內容的編輯之前重新讀取它。在任何其他情況下，例如過時的 `old_string` 或不使用 `replace_all` 匹配多次的情況，Claude 在編輯前重新讀取檔案。未讀和已變更檔案的寬鬆處理需要 Claude Code v2.1.208 或更新版本；在此之前，Claude Code 拒絕對它在對話中未讀過或在讀取後在磁碟上變更的任何檔案進行編輯。

使用 Bash 檢視檔案也滿足編輯前讀取要求，當命令是 `cat`、`head`、`tail`、`sed -n 'X,Yp'`、`grep`、`egrep` 或 `fgrep` 在單個檔案上，沒有管道或重定向時。管道輸出和其他 Bash 命令不計算編輯前讀取檢查。

這僅影響編輯資格，不影響權限。[Read 和 Edit 拒絕規則](/docs/zh-TW/permissions#tool-specific-permission-rules) 也適用於 Claude Code 在 Bash 中識別的檔案命令，例如 `cat`、`head`、`tail`、`sed` 和 `grep`，但不適用於間接讀取或寫入檔案的任意子程序，例如自己開啟檔案的 Python 或 Node 指令碼。識別用於拒絕規則的命令集與上述編輯前讀取清單不同：例如，`egrep` 和 `fgrep` 計算編輯前讀取但不針對 Read 拒絕規則進行檢查。如需涵蓋每個程序的作業系統級別強制執行，請 [啟用沙箱](/docs/zh-TW/sandboxing)。

<h2 id="glob-tool-behavior">
  Glob 工具行為
</h2>

Glob 工具按名稱模式查找檔案。它支援標準 glob 語法，包括 `**` 用於遞迴目錄匹配：

* `**/*.js` 匹配任何深度的所有 `.js` 檔案
* `src/**/*.ts` 匹配 `src/` 下的所有 `.ts` 檔案
* `*.{json,yaml}` 匹配目前目錄中的 `.json` 和 `.yaml` 檔案

結果按修改時間排序，並限制為 100 個檔案。如果達到上限，Claude 會在結果中看到截斷標誌，並可以縮小模式。

Glob 預設不尊重 `.gitignore`，因此它會找到 gitignored 檔案以及追蹤的檔案。這與 [Grep](#grep-tool-behavior) 不同，後者跳過 gitignored 檔案。若要讓 Glob 尊重 `.gitignore`，請在啟動 Claude Code 之前設定 `CLAUDE_CODE_GLOB_NO_IGNORE=false`。

包含空位元組的 `pattern` 或 `path` 值會傳回錯誤，要求 Claude 移除它。

<h2 id="grep-tool-behavior">
  Grep 工具行為
</h2>

Grep 工具在檔案內容中搜尋模式。其中 [Glob](#glob-tool-behavior) 按名稱查找檔案，Grep 在其中查找行。

Grep 建立在 [ripgrep](https://github.com/BurntSushi/ripgrep) 上，使用 ripgrep 的正規表達式語法，而不是 POSIX grep。包含正規表達式元字元的模式需要轉義。例如，在 Go 程式碼中查找 `interface{}` 需要模式 `interface\{\}`。

一個模式、glob 或檔案類型若被 ripgrep 拒絕，會返回包含 ripgrep 診斷的錯誤，以便 Claude 可以更正輸入並再次搜尋。在 v2.1.208 之前，Claude Code 將被拒絕的輸入報告為 `No files found`，而不是錯誤，即使搜尋的文字存在於目標檔案中。

三個輸出模式控制返回的內容：

* `files_with_matches`：僅檔案路徑，無行內容。這是預設值。
* `content`：匹配的行，帶有檔案和行號。
* `count`：每個檔案的匹配計數，後面跟著所有匹配檔案的總計。總計涵蓋每個匹配，即使工具的 `head_limit` 或 `offset` 參數截斷了列出的每個檔案項目。在 v2.1.208 之前，總計只加總列出的項目。

Claude 可以使用 `glob` 參數（例如 `**/*.tsx`）按檔案限制結果，或使用 `type` 參數（例如 `py` 或 `rust`）按語言限制結果。預設情況下，模式在單行內匹配。Claude 可以設定 `multiline: true` 以跨行邊界匹配。

Grep 尊重 `.gitignore`，因此 gitignored 檔案會被跳過。若要搜尋 gitignored 檔案，Claude 直接傳遞其路徑。

<h2 id="lsp-tool-behavior">
  LSP 工具行為
</h2>

LSP 工具從執行中的語言伺服器為 Claude 提供程式碼智慧。在每次檔案編輯後，它會自動報告型別錯誤和警告，以便 Claude 可以在沒有單獨建置步驟的情況下修復問題。Claude 也可以直接呼叫它來導航程式碼：

* 跳轉到符號的定義
* 尋找符號的所有參考
* 取得位置的型別資訊
* 列出檔案中的符號
* 在工作區中按名稱搜尋符號
* 尋找介面的實作
* 追蹤呼叫階層

該工具在您安裝您的語言的 [程式碼智慧外掛](/docs/zh-TW/discover-plugins#code-intelligence) 之前處於非作用中狀態。該外掛包含語言伺服器設定，您需要單獨安裝伺服器二進位檔。

<h2 id="monitor-tool">
  Monitor 工具
</h2>

Monitor 工具讓 Claude 在背景監視某些內容，並在其變更時做出反應，而無需暫停對話。要求 Claude：

* 追蹤日誌檔案並在錯誤出現時標記
* 輪詢 PR 或 CI 工作，並在其狀態變更時報告
* 監視目錄以查看檔案變更
* 追蹤您指向的任何長時間執行指令碼的輸出
* 連接到 WebSocket 摘要並在每條訊息到達時報告

對於大多數監視，Claude 會編寫一個小指令碼，在背景執行它，並在每行到達時接收它。對於已經推送事件的伺服器，Claude 可以開啟 [WebSocket](#websocket-source) 而不是執行指令碼。

您可以在同一工作階段中繼續工作，Claude 會在事件發生時插入。透過要求 Claude 取消監視或結束工作階段來停止監視。

當 Monitor 執行命令時，它使用與 [Bash 相同的權限規則](/docs/zh-TW/permissions#tool-specific-permission-rules)，因此您為 Bash 設定的 `allow` 和 `deny` 模式也適用於此處。[WebSocket 來源](#websocket-source)有其自己的核准提示。

該工具在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。當設定 `DISABLE_TELEMETRY` 或 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 時，它也不可用。

外掛可以宣告在外掛啟用時自動啟動的監視，而不是要求 Claude 啟動它們。請參閱 [外掛監視](/docs/zh-TW/plugins-reference#monitors)。

<h3 id="websocket-source">
  WebSocket 來源
</h3>

<Note>
  WebSocket 來源需要 Claude Code v2.1.195 或更新版本。
</Note>

當伺服器已經透過 WebSocket 推送事件時，Claude 可以直接連接到它，而不是編寫輪詢指令碼。每種套接字活動要麼變成一個事件，要麼結束監視：

* **文字訊息**：每一條都變成一個事件，即使訊息跨越多行。
* **二進位訊息**：不會通過。Claude 會收到一個佔位符行，例如 `[binary frame, 512 bytes]`。
* **大於 1 MiB 的訊息**：監視結束，因此請訂閱存在的篩選摘要。
* **套接字關閉**：監視結束，Claude 會收到關閉代碼。

WebSocket 監視採用 `ws` 輸入代替 `command`，單一 Monitor 呼叫無法結合兩者。`ws` 輸入有兩個欄位：

| 欄位          | 必需 | 說明                                                        |
| :---------- | :- | :-------------------------------------------------------- |
| `url`       | 是  | 要連接的端點。必須是 `ws://` 或 `wss://` URL，不含嵌入的認證或空白，僅使用 ASCII 字元 |
| `protocols` | 否  | 在握手期間提供的 WebSocket 子協議名稱。每個項目必須是有效的子協議令牌，清單不能包含重複項        |

`timeout_ms` 和 `persistent` 輸入的行為與它們對命令的行為相同：監視在截止時間結束，除非設定 `persistent`，`TaskStop` 會提前取消它。

開啟 WebSocket 會提示核准，提示不提供跳過相同主機的未來提示的選項。

Claude Code 拒絕指向私有、連結本地或雲端中繼資料位址的 URL，包括解析為該位址的主機名稱。它也拒絕 `sandbox.network.deniedDomains` 中的主機，以及當在受管設定中設定 [`allowManagedDomainsOnly`](/docs/zh-TW/settings#sandbox-settings) 時，受管允許清單外的任何主機。

<h2 id="notebookedit-tool-behavior">
  NotebookEdit 工具行為
</h2>

NotebookEdit 一次修改一個 Jupyter notebook 儲存格，按其 `cell_id` 定位儲存格。它不像 [Edit](#edit-tool-behavior) 在純文字檔案上那樣跨 notebook 執行字串替換。

三個編輯模式控制目標儲存格發生的情況：

* `replace`：覆寫儲存格的來源。這是預設值。
* `insert`：在目標後新增新儲存格。沒有 `cell_id` 時，新儲存格位於 notebook 的開始。需要 `cell_type` 設定為 `code` 或 `markdown`。
* `delete`：移除目標儲存格。

權限規則使用 `Edit(...)` 路徑格式。像 `Edit(notebooks/**)` 這樣的規則涵蓋該目錄中檔案上的 NotebookEdit 呼叫。

<h2 id="powershell-tool">
  PowerShell 工具
</h2>

PowerShell 工具讓 Claude 原生執行 PowerShell 命令。在 Windows 上，這表示命令在 PowerShell 中執行，而不是透過 Git Bash 路由。該工具的可用方式取決於您的平台：

* **沒有 Git Bash 的 Windows**：該工具會自動啟用。
* **安裝了 Git Bash 的 Windows**：該工具正在逐步推出。
* **Linux、macOS 和 WSL**：該工具是選擇加入的。

<h3 id="enable-the-powershell-tool">
  啟用 PowerShell 工具
</h3>

在您的環境或 `settings.json` 中設定 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

在 Windows 上，將變數設定為 `0` 以選擇退出推出。在 Linux、macOS 和 WSL 上，該工具需要 PowerShell 7 或更新版本：安裝 `pwsh` 並確保它在您的 `PATH` 上。

在 Windows 上，Claude Code 自動偵測 `pwsh.exe`（PowerShell 7+），並回退到 `powershell.exe`（PowerShell 5.1）。啟用該工具時，Claude 將 PowerShell 視為主要 shell。當安裝了 Git Bash 時，Bash 工具仍可用於 POSIX 指令碼。

Claude Code 使用 `-ExecutionPolicy Bypass` 在程序範圍內生成 PowerShell，因此 `.ps1` 指令碼和模組匯入可在預設 Windows 安裝上運作，無需變更機器的原則。程序範圍的略過不會覆蓋群組原則 `MachinePolicy` 或 `UserPolicy`，因此企業原則仍然適用。若要改為尊重機器的有效執行原則，請設定 `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`。

<h3 id="shell-selection-in-settings-hooks-and-skills">
  設定、hooks 和 skills 中的 shell 選擇
</h3>

三個額外的設定控制 PowerShell 的使用位置：

* [`settings.json`](/docs/zh-TW/settings#available-settings) 中的 `"defaultShell": "powershell"`：透過 PowerShell 路由互動式 `!` 命令。需要啟用 PowerShell 工具。
* 個別 [command hooks](/docs/zh-TW/hooks#command-hook-fields) 上的 `"shell": "powershell"`：在 PowerShell 中執行該 hook。Hooks 直接生成 PowerShell，因此無論 `CLAUDE_CODE_USE_POWERSHELL_TOOL` 如何，這都有效。
* [skill frontmatter](/docs/zh-TW/skills#frontmatter-reference) 中的 `shell: powershell`：在 PowerShell 中執行 `` !`command` `` 區塊。需要啟用 PowerShell 工具。

Bash 工具部分中描述的相同主工作階段工作目錄重設行為適用於 PowerShell 命令，包括 `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR` 環境變數。

自 v2.1.196 起，PowerShell 工具符合 Bash 工具對搜尋和 diff 結束代碼的處理方式。來自 `grep`、`egrep`、`fgrep` 和 `git grep` 的結束代碼 1 表示沒有符合項目，來自 `git diff` 的結束代碼 1 表示存在差異，因此這些結果不會作為命令失敗報告給 Claude。

<h3 id="preview-limitations">
  預覽限制
</h3>

PowerShell 工具在預覽期間有以下已知限制：

* PowerShell 設定檔未載入
* 在 Windows 上，不支援 sandboxing

<h2 id="read-tool-behavior">
  Read 工具行為
</h2>

Read 工具採用檔案路徑並傳回帶有行號的內容。Claude 被指示始終傳遞絕對路徑。

預設情況下，Read 從開始傳回檔案。當整個檔案讀取超過令牌限制時，Read 傳回第一頁並附帶 `PARTIAL view` 通知，告訴 Claude 它收到了多少檔案內容以及如何使用 `offset` 和 `limit` 讀取更多內容。傳遞明確的 `offset` 或 `limit` 且仍然超過令牌限制的讀取會傳回錯誤。

具有明確 `limit` 的讀取會在選定的行超過令牌限制可能容納的內容時立即停止，並傳回錯誤而不載入其餘範圍。該錯誤告訴 Claude 使用較小的 `limit`，或在單一行非常大時改用 [Grep](#grep-tool-behavior) 搜尋特定內容。在 v2.1.208 之前，Claude Code 在拒絕前會將整個範圍載入記憶體，因此具有極長單一行的檔案可能會耗盡記憶體。

讀取空檔案會傳回通知，表示檔案存在但其內容為空，而超過最後一行的 `offset` 會傳回通知，提供檔案的行數。在 v2.1.208 之前，讀取空檔案會傳回超過末尾的通知。

Read 處理純文字以外的多種檔案類型：

* **影像**：PNG、JPG 和其他影像格式作為 Claude 可以看到的視覺內容傳回，而不是原始位元組。Claude Code 在傳送前調整大小並重新壓縮大型影像以適應模型的影像大小限制，因此 Claude 可能會看到大型螢幕截圖的縮小版本。自 v2.1.196 起，在調整大小後仍然大於 500KB 的影像會以降低品質的 JPEG 重新編碼，其像素尺寸保持不變。如果 Claude 在大型影像中遺漏細微的像素級詳細資訊，請要求它先裁剪感興趣的區域，例如使用 ImageMagick 透過 Bash。
* **PDF**：Claude 完整讀取短 `.pdf` 檔案。對於超過 10 頁的 PDF，它使用 `pages` 參數（例如 `"1-5"`）按範圍讀取，一次最多 20 頁。
* **Jupyter notebooks**：`.ipynb` 檔案傳回所有儲存格及其輸出，包括程式碼、markdown 和視覺化。

Read 僅讀取檔案，不讀取目錄。Claude 使用透過 Bash 工具的 `ls` 列出目錄內容。

<h2 id="webfetch-tool-behavior">
  WebFetch 工具行為
</h2>

WebFetch 採用 URL 和描述要提取內容的提示。它擷取頁面，當伺服器傳回 HTML 時將回應轉換為 Markdown，並使用小型快速模型針對內容執行提示。對於大多數擷取，Claude 會收到該模型的答案，而不是原始頁面。轉換步驟不可設定。

這使 WebFetch 在設計上是有損的。提取提示決定了到達 Claude 的內容，因此說頁面未提及某事的結果可能只是意味著提示未詢問它。要求 Claude 使用更具體的提示再次擷取，或使用透過 Bash 的 `curl` 獲取未處理的頁面。

一些行為塑造了 Claude 接收的回應：

* HTTP URL 會自動升級為 HTTPS。
* 大型頁面在處理前被截斷為固定字元限制。
* 回應會快取 15 分鐘，因此相同 URL 的重複擷取會快速傳回。
* 當 URL 重定向到不同的主機時，WebFetch 傳回文字結果，命名原始 URL 和重定向目標，而不是跟隨它。Claude 然後使用第二個 WebFetch 呼叫擷取新 URL。

在預設和 `acceptEdits` 權限模式中，WebFetch 在首次到達新網域時提示，但有一組內建的預先核准文件網域除外，這些網域無需提示即可擷取。若要提前允許另一個網域而不提示，請新增像 `WebFetch(domain:example.com)` 這樣的權限規則。`auto` 和 `bypassPermissions` [權限模式](/docs/zh-TW/permissions#permission-modes) 完全跳過提示。

`deny`、`ask` 或 `allow` 中的明確 `WebFetch(domain:...)` 規則優先於預先核准的集合，因此您可以封鎖預先核准的網域或要求提示。

WebFetch 設定以 `Claude-User` 開頭的 `User-Agent` 標頭，以及偏好 Markdown 而不是 HTML 的 `Accept` 標頭，以便支援內容協商的伺服器可以直接傳回 Markdown。您可以單獨設定 [sandbox](/docs/zh-TW/sandboxing) 網路規則，因此您希望沙箱程序到達的網域仍需要明確的沙箱權限規則。

<h2 id="websearch-tool-behavior">
  WebSearch 工具行為
</h2>

WebSearch 針對 Anthropic 的 [web search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) 後端執行查詢，並傳回結果標題和 URL。它不擷取結果頁面。若要讀取 Claude 在搜尋結果中找到的頁面，它會跟進 [WebFetch](#webfetch-tool-behavior)。

該工具可能在傳回結果之前發出最多八個後端搜尋，在內部精煉搜尋。Claude 可以使用 `allowed_domains` 限制結果以僅包含某些主機，或使用 `blocked_domains` 排除它們。這兩個清單不能在單個呼叫中組合。

搜尋後端不可設定。若要使用不同的提供者進行搜尋，請新增公開搜尋工具的 [MCP server](/docs/zh-TW/mcp)。

WebSearch 權限規則不採用 specifier。`allow` 或 `deny` 中的裸 `WebSearch` 項目是唯一的形式。

<Note>
  WebSearch 在 Claude API、[Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 和 Microsoft Foundry 上可用。在 Google Cloud 的 Agent Platform 上，它適用於 Claude 4 及更新版本的模型，包括 Opus、Sonnet 和 Haiku。Amazon Bedrock 不公開伺服器端 web search 工具。
</Note>

<h2 id="write-tool-behavior">
  Write 工具行為
</h2>

Write 工具建立新檔案或使用提供的完整內容覆寫現有檔案。它不附加或合併。

如果目標路徑已存在，Claude 必須在目前對話中至少讀取過該檔案一次才能覆寫它。寫入未讀的現有檔案會失敗並出現錯誤。此限制不適用於新檔案。

使用 Bash 檢視檔案也滿足此要求，如 [Edit 工具行為](#edit-tool-behavior) 中所述的相同規則。

對於現有檔案的部分變更，Claude 使用 Edit 而不是 Write。

<h2 id="check-which-tools-are-available">
  檢查哪些工具可用
</h2>

您的確切工具集取決於您的提供者、平台和設定。若要檢查在執行中的工作階段中載入了什麼，請直接詢問 Claude：

```text theme={null}
What tools do you have access to?
```

Claude 提供對話摘要。如需確切的 MCP 工具名稱，請執行 `/mcp`。

<Note>
  [advisor tool](/docs/zh-TW/advisor) 是一個 [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)，由 API 執行，而不是 Claude Code 實作的工具。它沒有您可以在權限規則或 hook 匹配器中參考的名稱。
</Note>

<h2 id="see-also">
  另請參閱
</h2>

* [MCP servers](/docs/zh-TW/mcp)：透過連接外部伺服器新增自訂工具
* [權限](/docs/zh-TW/permissions)：權限系統、規則語法和工具特定模式
* [Subagents](/docs/zh-TW/sub-agents)：為 subagents 設定工具存取
* [Hooks](/docs/zh-TW/hooks-guide)：在工具執行前後執行自訂命令
