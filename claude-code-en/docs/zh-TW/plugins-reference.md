> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins 參考

> Claude Code 外掛系統的完整技術參考，包括架構、CLI 命令和元件規格。

<Tip>
  想要安裝外掛？請參閱 [探索和安裝外掛](/docs/zh-TW/discover-plugins)。如需建立外掛，請參閱 [Plugins](/docs/zh-TW/plugins)。如需發佈外掛，請參閱 [Plugin marketplaces](/docs/zh-TW/plugin-marketplaces)。
</Tip>

本參考提供 Claude Code 外掛系統的完整技術規格，包括元件架構、CLI 命令和開發工具。

**plugin** 是一個自包含的目錄，包含擴展 Claude Code 功能的元件。Plugin 元件包括 skills、agents、hooks、MCP servers、LSP servers 和 monitors。

<h2 id="plugin-components-reference">
  Plugin 元件參考
</h2>

<h3 id="skills">
  Skills
</h3>

Plugins 將 skills 新增至 Claude Code，建立可由您或 Claude 叫用的 `/name` 快捷方式。

**位置**：plugin 根目錄中的 `skills/` 或 `commands/` 目錄，或 plugin 根目錄中的單一 `SKILL.md` 檔案

**檔案格式**：Skills 是包含 `SKILL.md` 的目錄；commands 是簡單的 markdown 檔案

**Skill 結構**：

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (optional)
│   └── scripts/ (optional)
└── code-reviewer/
    └── SKILL.md
```

**整合行為**：

* 安裝 plugin 時會自動探索 skills 和 commands
* Claude 可以根據任務上下文自動叫用它們
* Skills 可以在 SKILL.md 旁邊包含支援檔案

如果 plugin 沒有 `skills/` 目錄且沒有 `skills` manifest 欄位，plugin 根目錄中的 `SKILL.md` 會被載入為單一 skill。設定 frontmatter `name` 欄位以控制 skill 的叫用名稱。沒有它的話，Claude Code 會回退到安裝目錄名稱，對於 marketplace 安裝的 plugins，這是一個在每次更新時都會變更的版本字串。對於提供多個 skills 的 plugins，請使用上面所示的 `skills/` 目錄配置。

如需完整詳細資訊，請參閱 [Skills](/docs/zh-TW/skills)。

<h3 id="agents">
  Agents
</h3>

Plugins 可以提供專門的 subagents，用於 Claude 在適當時自動叫用的特定任務。

**位置**：plugin 根目錄中的 `agents/` 目錄

**檔案格式**：描述 agent 功能的 Markdown 檔案

**Agent 結構**：

```markdown theme={null}
---
name: agent-name
description: 此 agent 的專長以及 Claude 應何時叫用它
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

詳細的系統提示，描述 agent 的角色、專業知識和行為。
```

Plugin agents 支援 `name`、`description`、`model`、`effort`、`maxTurns`、`tools`、`disallowedTools`、`skills`、`memory`、`background` 和 `isolation` frontmatter 欄位。唯一有效的 `isolation` 值是 `"worktree"`。出於安全原因，plugin 提供的 agents 不支援 `hooks`、`mcpServers` 和 `permissionMode`。

**整合點**：

* Agents 出現在 [@-mention 下拉式選單](/docs/zh-TW/sub-agents#invoke-subagents-explicitly) 中，其範圍名稱為 `my-plugin:code-reviewer`，一旦啟用 plugin
* Claude 可以根據任務上下文自動叫用 agents
* Users 可以手動叫用 agents
* Plugin agents 與內建 Claude agents 一起運作

如需完整詳細資訊，請參閱 [Subagents](/docs/zh-TW/sub-agents)。

<h3 id="hooks">
  Hooks
</h3>

Plugins 可以提供事件處理程式，自動回應 Claude Code 事件。

**位置**：plugin 根目錄中的 `hooks/hooks.json`，或在 plugin.json 中內聯

**格式**：具有事件匹配器和動作的 JSON 設定

**Hook 設定**：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

Plugin hooks 回應與 [user-defined hooks](/docs/zh-TW/hooks) 相同的生命週期事件：

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Hook 類型**：

* `command`：執行 shell 命令或指令碼
* `http`：將事件 JSON 作為 POST 請求傳送到 URL
* `mcp_tool`：在已設定的 [MCP server](/docs/zh-TW/mcp) 上呼叫工具
* `prompt`：使用 LLM 評估提示（使用 `$ARGUMENTS` 佔位符表示上下文）
* `agent`：執行具有工具的 agentic 驗證器以進行複雜驗證任務

針對 plugin 自己的 [bundled MCP server](#mcp-servers) 的 Hooks 必須使用其範圍名稱。工具匹配器和 `if` 欄位採用範圍工具名稱 `mcp__plugin_<plugin-name>_<server-name>__<tool>`，而 `mcp_tool` hook 的 `server` 欄位採用 `plugin:<plugin-name>:<server-name>`。針對裸伺服器金鑰撰寫的匹配器永遠不會觸發。請參閱 [Match MCP tools](/docs/zh-TW/hooks#match-mcp-tools) 和 [Plugin-provided MCP servers](/docs/zh-TW/mcp#plugin-provided-mcp-servers)。

<h3 id="mcp-servers">
  MCP servers
</h3>

Plugins 可以捆綁 Model Context Protocol (MCP) servers，將 Claude Code 與外部工具和服務連接。

**位置**：plugin 根目錄中的 `.mcp.json`，或在 plugin.json 中內聯

**格式**：標準 MCP server 設定

**MCP server 設定**：

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**整合行為**：

* 啟用 plugin 時，Plugin MCP servers 會自動啟動
* Servers 在 Claude 的工具組中顯示為標準 MCP 工具
* Server 功能與 Claude 的現有工具無縫整合
* Plugin servers 可以獨立於使用者 MCP servers 進行設定

<h3 id="lsp-servers">
  LSP servers
</h3>

<Tip>
  想要使用 LSP plugins？從官方 marketplace 安裝它們：在 `/plugin` Discover 標籤中搜尋「lsp」。本節記錄如何為官方 marketplace 未涵蓋的語言建立 LSP plugins。
</Tip>

Plugins 可以提供 [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) servers，在處理程式碼庫時為 Claude 提供即時程式碼智慧。

LSP 整合提供：

* **即時診斷**：Claude 在每次編輯後立即看到錯誤和警告
* **程式碼導航**：前往定義、尋找參考和懸停資訊
* **語言感知**：程式碼符號的類型資訊和文件

**位置**：plugin 根目錄中的 `.lsp.json`，或在 `plugin.json` 中內聯

**格式**：將語言伺服器名稱對應到其設定的 JSON 設定

**`.lsp.json` 檔案格式**：

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**在 `plugin.json` 中內聯**：

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**必需欄位：**

| 欄位                    | 描述                        |
| :-------------------- | :------------------------ |
| `command`             | 要執行的 LSP 二進位檔（必須在 PATH 中） |
| `extensionToLanguage` | 將檔案副檔名對應到語言識別碼            |

**選用欄位：**

| 欄位                      | 描述                                                                  |
| :---------------------- | :------------------------------------------------------------------ |
| `args`                  | LSP server 的命令列引數                                                   |
| `transport`             | 通訊傳輸：`stdio`（預設）或 `socket`                                          |
| `env`                   | 啟動 server 時要設定的環境變數                                                 |
| `initializationOptions` | 在初始化期間傳遞給 server 的選項                                                |
| `settings`              | 透過 `workspace/didChangeConfiguration` 傳遞的設定                         |
| `workspaceFolder`       | server 的工作區資料夾路徑                                                    |
| `startupTimeout`        | 等待 server 啟動的最長時間（毫秒）                                               |
| `shutdownTimeout`       | 等待正常關閉的最長時間（毫秒）。當逾時時間過去時，Claude Code 會終止 server 程序。未設定時，不適用逾時       |
| `restartOnCrash`        | server 崩潰後是否重新啟動。預設為 `true`。設定為 `false` 以保持崩潰的 server 停止而不是重新啟動它    |
| `maxRestarts`           | 放棄前的最大重新啟動嘗試次數                                                      |
| `diagnostics`           | 是否在編輯後將診斷推送到 Claude 的上下文中（預設 `true`）。設定為 `false` 以保留程式碼導航但抑制自動診斷注入。 |

`restartOnCrash` 和 `shutdownTimeout` 需要 Claude Code v2.1.205 或更新版本。在 v2.1.205 之前，設定架構接受兩個選項，但設定其中任一個會導致 Claude Code 在啟動時完全跳過該 LSP server，原因僅在 `claude --debug` 輸出中可見。

**相同副檔名的多個 servers**：當多個已啟用的 LSP servers 在 `extensionToLanguage` 中宣告相同的檔案副檔名時，無論 servers 來自一個 plugin 還是來自不同的 plugins，第一個註冊的 server 會處理具有該副檔名的檔案，其他的永遠不會啟動。`/plugin` 介面會顯示一個警告，命名其 server 為作用中的 plugin。

**無法初始化的 Servers**：Claude Code 會跳過設定無效的 server，例如缺少 `command` 或 `extensionToLanguage` 的 server，其他已設定的 servers 仍會啟動。執行 `claude --debug` 以查看為什麼 server 被跳過。

被跳過的 server 不會聲稱其檔案副檔名，因此另一個宣告相同副檔名的有效 server（來自相同或不同的 plugin）仍會處理這些檔案。在 v2.1.205 之前，無法初始化的 server 仍會聲稱其副檔名並阻止另一個有效的 server 使用相同的副檔名。

<Warning>
  **您必須單獨安裝語言伺服器二進位檔。** LSP plugins 設定 Claude Code 如何連接到語言伺服器，但它們不包括伺服器本身。如果您在 `/plugin` Errors 標籤中看到 `Executable not found in $PATH`，請為您的語言安裝所需的二進位檔。
</Warning>

**可用的 LSP plugins：**

| Plugin              | 語言伺服器                      | 安裝命令                                                                            |
| :------------------ | :------------------------- | :------------------------------------------------------------------------------ |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` 或 `npm install -g pyright`                                |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                          |
| `rust-analyzer-lsp` | rust-analyzer              | [參閱 rust-analyzer 安裝](https://rust-analyzer.github.io/manual.html#installation) |

先安裝語言伺服器，然後從 marketplace 安裝 plugin。

<h3 id="monitors">
  Monitors
</h3>

Plugins 可以宣告背景 monitors，Claude Code 在 plugin 啟用時自動啟動。每個 monitor 執行一個 shell 命令，持續整個工作階段，並將每個 stdout 行傳遞給 Claude 作為通知，以便 Claude 可以對日誌項目、狀態變更或輪詢事件做出反應，而無需被要求自行啟動監視。

Plugin monitors 使用與 [Monitor tool](/docs/zh-TW/tools-reference#monitor-tool) 相同的機制，並共享其可用性限制。它們僅在互動式 CLI 工作階段中執行，以與 [hooks](#hooks) 相同的信任級別在未沙箱化的環境中執行，並在 Monitor tool 不可用的主機上被跳過。

**位置**：plugin 根目錄中的 `monitors/monitors.json`，或在 `plugin.json` 中內聯

**格式**：monitor 項目的 JSON 陣列

以下 `monitors/monitors.json` 監視部署狀態端點和本機錯誤日誌：

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "Deployment status changes"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log",
    "when": "on-skill-invoke:debug"
  }
]
```

若要內聯宣告 monitors，請將 `plugin.json` 中的 `experimental.monitors` 設定為相同的陣列。若要從非預設路徑載入，請將 `experimental.monitors` 設定為相對路徑字串，例如 `"./config/monitors.json"`。Monitors 是 [experimental component](#experimental-components)。

**必需欄位：**

| 欄位            | 描述                                                 |
| :------------ | :------------------------------------------------- |
| `name`        | 在 plugin 中唯一的識別碼。防止 plugin 重新載入或再次叫用 skill 時出現重複程序 |
| `command`     | 在工作階段工作目錄中作為持久背景程序執行的 shell 命令                     |
| `description` | 正在監視的內容的簡短摘要。顯示在任務面板和通知摘要中                         |

**選用欄位：**

| 欄位     | 描述                                                                                                                       |
| :----- | :----------------------------------------------------------------------------------------------------------------------- |
| `when` | 控制 monitor 何時啟動。`"always"` 在工作階段啟動和 plugin 重新載入時啟動它，是預設值。`"on-skill-invoke:<skill-name>"` 在此 plugin 中的命名 skill 首次被分派時啟動它 |

`command` 值支援 [path substitutions](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`、`${CLAUDE_PLUGIN_DATA}` 和 `${CLAUDE_PROJECT_DIR}`，加上環境中的任何 `${ENV_VAR}`。如果指令碼需要從 plugin 自己的目錄執行，請在命令前加上 `cd "${CLAUDE_PLUGIN_ROOT}" && `。

monitor `command` 無法參考 [`${user_config.*}`](#user-configuration) 值。命令透過 shell 執行，所以 Claude Code 會以 [error](/docs/zh-TW/errors#plugin-command-references-user-config) 拒絕 monitor，而不是替換該值。Monitor 程序不會接收 `CLAUDE_PLUGIN_OPTION_<KEY>` 環境變數，所以讓 monitor 指令碼從它擁有的設定檔讀取該值。在 v2.1.207 之前，monitor 命令替換了 `${user_config.*}` 值。

在工作階段中途停用 plugin 不會停止已在執行的 monitors。它們在工作階段結束時停止。

<h3 id="themes">
  Themes
</h3>

Plugins 可以提供顏色主題，這些主題與內建預設值和使用者的本機主題一起出現在 `/theme` 中。主題是 `themes/` 中的 JSON 檔案，具有 `base` 預設值和稀疏的 `overrides` 顏色令牌對應。Themes 是 [experimental component](#experimental-components)。

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

選擇 plugin 主題會在使用者的設定中保留 `custom:<plugin-name>:<slug>`。Plugin 主題是唯讀的；在 `/theme` 中按 `Ctrl+E` 會將其複製到 `~/.claude/themes/`，以便使用者可以編輯副本。

***

<h2 id="plugin-installation-scopes">
  Plugin 安裝範圍
</h2>

安裝 plugin 時，您選擇一個**範圍**，決定 plugin 的可用位置和誰可以使用它：

| 範圍        | 設定檔                                                | 使用案例                     |
| :-------- | :------------------------------------------------- | :----------------------- |
| `user`    | `~/.claude/settings.json`                          | 在所有專案中可用的個人 plugins（預設）  |
| `project` | `.claude/settings.json`                            | 透過版本控制共享的團隊 plugins      |
| `local`   | `.claude/settings.local.json`                      | 專案特定的 plugins，gitignored |
| `managed` | [Managed settings](/docs/zh-TW/settings#settings-files) | 受管理的 plugins（唯讀，僅更新）     |

Plugins 使用與其他 Claude Code 設定相同的範圍系統。如需安裝說明和範圍旗標，請參閱 [安裝 plugins](/docs/zh-TW/discover-plugins#install-plugins)。如需範圍的完整說明，請參閱 [Configuration scopes](/docs/zh-TW/settings#configuration-scopes)。

***

<h2 id="skills-directory-plugins">
  Skills 目錄 plugins
</h2>

任何 skills 目錄下包含 `.claude-plugin/plugin.json` manifest 的資料夾都會在下一個工作階段中作為名為 `<name>@skills-dir` 的 plugin 載入，無需 marketplace 和無需安裝步驟。使用 [`plugin init`](#plugin-init) 進行搭建。與 marketplace 安裝不同，plugin 是在原地發現的，而不是複製到 plugin 快取中。

Skills 目錄樹支援三個不同的東西：

| 您擁有的內容                                        | 它是什麼                                                     |
| :-------------------------------------------- | :------------------------------------------------------- |
| `<skills-dir>/foo/SKILL.md`，沒有 manifest       | 一個名為 `foo` 的純 [skill](/docs/zh-TW/skills)                     |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | 一個 plugin `foo@skills-dir`，可以捆綁自己的 skills、agents、hooks 等 |
| `<plugin>/skills/bar/SKILL.md`                | 一個 skill `bar`，打包在 plugin 內                              |

<h3 id="choose-where-the-plugin-loads-from">
  選擇 plugin 載入的位置
</h3>

| Skills 目錄               | 範圍       | 載入                                               |
| :---------------------- | :------- | :----------------------------------------------- |
| `~/.claude/skills/`     | personal | 在每個專案中，因為位置只屬於您                                  |
| `<cwd>/.claude/skills/` | project  | 只有在您接受該資料夾的工作區 [trust dialog](/docs/zh-TW/settings) 後 |

專案範圍的 plugin 被簽入存放庫，並到達克隆它的每個協作者。因為該內容來自存放庫而不是來自您，它只在與 `.claude/settings.json` 相同的信任閘道後載入，並且執行程式碼的元件受到進一步限制：

* 它宣告的 MCP servers 會經過與專案 `.mcp.json` 相同的 [per-server approval](/docs/zh-TW/mcp)
* LSP servers 只有在您信任工作區後才會啟動
* [Background monitors](#monitors) 不會載入

個人範圍的 plugins 沒有這些限制。

<Warning>
  專案範圍的 `@skills-dir` plugins 只從您啟動 Claude Code 的目錄的 `.claude/skills/` 載入。它們不會像純 skills 和 commands 那樣 [walk up to the repository root](/docs/zh-TW/skills#automatic-discovery-from-parent-and-nested-directories)，所以從子目錄啟動會錯過位於存放庫根目錄的 plugin。從存放庫根目錄啟動，或在變更目錄後執行 `/reload-plugins`。
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  編輯、重新載入和停用 skills 目錄 plugin
</h3>

您對 skill 的 `SKILL.md` 所做的變更會立即在目前工作階段中生效。對 plugin 的其他元件（例如 `hooks/`、`.mcp.json`、`agents/` 和 `output-styles/`）的變更則不會。執行 `/reload-plugins` 或重新啟動 Claude Code 以取得這些變更。請參閱 [Live change detection](/docs/zh-TW/skills#live-change-detection)。

若要停止載入 skills 目錄 plugin，請刪除其資料夾或按名稱停用它。沒有 `uninstall` 步驟，因為沒有從 marketplace 安裝任何內容。

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  Plugin manifest 架構
</h2>

`.claude-plugin/plugin.json` 檔案定義您的 plugin 的中繼資料和設定。本節記錄所有支援的欄位和選項。

manifest 是選用的。如果省略，Claude Code 會自動探索[預設位置](#file-locations-reference)中的元件，並從目錄名稱衍生 plugin 名稱。當您需要提供中繼資料或自訂元件路徑時，請使用 manifest。

<h3 id="complete-schema">
  完整架構
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  必需欄位
</h3>

如果您包含 manifest，`name` 是唯一必需的欄位。

| 欄位     | 類型     | 描述                                                                                                                                                        | 範例                   |
| :----- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `name` | string | 唯一識別碼（kebab-case，無空格）。當[marketplace 項目](/docs/zh-TW/plugin-marketplaces#plugin-entries)以不同名稱列出 plugin 時，marketplace 項目名稱是 `enabledPlugins` 金鑰和 `/plugin` 使用的名稱 | `"deployment-tools"` |

此名稱用於命名空間元件。例如，在 UI 中，名稱為 `plugin-dev` 的 plugin 的 agent `agent-creator` 將顯示為 `plugin-dev:agent-creator`。

<h3 id="unrecognized-fields">
  無法識別的欄位
</h3>

Claude Code 會忽略它無法識別的頂層欄位。您可以在 `plugin.json` 中保留來自另一個生態系統的中繼資料，plugin 仍會載入。這使得維護一個 manifest 作為 VS Code 或 Cursor 擴充功能 manifest、npm `package.json` 或 MCPB/DXT bundle manifest 變得實用。

`claude plugin validate` 將無法識別的欄位報告為警告，而不是錯誤。如果欄位與已識別的欄位相差一或兩個字元，警告會建議可能的預期名稱。只有無法識別欄位警告的 plugin 仍會通過驗證並在執行時載入。

類型錯誤的欄位仍會失敗。例如，`keywords` 值是字串而不是陣列是載入錯誤，`claude plugin validate` 會將其報告為錯誤。

傳遞 `--strict` 以將警告視為錯誤。在 CI 中使用它來捕捉拼寫錯誤的欄位名稱或來自另一個工具的 manifest 中遺留的欄位，然後再發佈，即使 plugin 會在執行時載入。

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  中繼資料欄位
</h3>

| 欄位               | 類型      | 描述                                                                                                                                                                                 | 範例                                                                |
| :--------------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| `$schema`        | string  | JSON Schema URL，用於編輯器自動完成和驗證。Claude Code 在載入時忽略此欄位。                                                                                                                                | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | 在 `/plugin` 選擇器和其他 UI 介面中顯示的人類可讀名稱。當省略時回退到 `name`。與 `name` 不同，可能包含空格和任何大小寫。不用於命名空間或查詢。需要 Claude Code v2.1.143 或更新版本。                                                               | `"Deployment Tools"`                                              |
| `version`        | string  | 選用。語義版本。設定此項會將 plugin 固定到該版本字串，因此使用者只會在您提升版本時收到更新。如果省略，Claude Code 會回退到 git commit SHA，因此每個 commit 都被視為新版本。如果也在 marketplace 項目中設定，`plugin.json` 優先。請參閱[版本管理](#version-management)。 | `"2.1.0"`                                                         |
| `description`    | string  | plugin 用途的簡短說明                                                                                                                                                                     | `"Deployment automation tools"`                                   |
| `author`         | object  | 作者資訊                                                                                                                                                                               | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | 文件 URL                                                                                                                                                                             | `"https://docs.example.com"`                                      |
| `repository`     | string  | 原始程式碼 URL                                                                                                                                                                          | `"https://github.com/user/plugin"`                                |
| `license`        | string  | 授權識別碼                                                                                                                                                                              | `"MIT"`、`"Apache-2.0"`                                            |
| `keywords`       | array   | 探索標籤                                                                                                                                                                               | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | 當使用者未設定時，plugin 是否以啟用狀態啟動。預設為 `true`。請參閱[預設啟用](#default-enablement)。需要 Claude Code v2.1.154 或更新版本。                                                                                 | `false`                                                           |

<h3 id="default-enablement">
  預設啟用
</h3>

在 `plugin.json` 中設定 `defaultEnabled: false` 以提供安裝時停用的 plugin。使用者使用 `claude plugin enable <plugin>` 或 `/plugin` 介面將其開啟。對於新增成本或使用者應選擇加入的範圍的 plugins 使用此方法，例如連接到外部服務的 plugin。這需要 Claude Code v2.1.154 或更新版本。較早的版本會忽略該欄位並在安裝時啟用 plugin。

`defaultEnabled` 是當沒有其他因素決定 plugin 狀態時的後備。有兩件事優先於它：

* **使用者的設定**：在任何設定範圍的 `enabledPlugins` 中為 plugin 的項目。一旦寫入，它會在 plugin 更新和重新安裝中保留，因此在後續版本中變更 `defaultEnabled` 不會翻轉現有使用者。
* **相依性要求**：當 plugin 被另一個啟用的 plugin 所需時，Claude Code 會在安裝或啟用時為其寫入 `true`。這給了它一個明確的設定，所以它自己的預設不再適用。請參閱[啟用或停用具有相依性的 plugin](/docs/zh-TW/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies)。

相同的欄位可以出現在 plugin 的 marketplace 項目中，其優先於 `plugin.json` 中的值。請參閱[選用 plugin 欄位](/docs/zh-TW/plugin-marketplaces#optional-plugin-fields)。

<h3 id="component-path-fields">
  元件路徑欄位
</h3>

| 欄位                      | 類型                    | 描述                                                                                                          | 範例                                                   |
| :---------------------- | :-------------------- | :---------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `skills`                | string\|array         | 包含 `<name>/SKILL.md` 的自訂 skill 目錄。新增到預設 `skills/` 掃描。請參閱[路徑行為規則](#path-behavior-rules)以了解 marketplace 根目錄例外 | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | 自訂平面 `.md` skill 檔案或目錄（取代預設 `commands/`）                                                                    | `"./custom/cmd.md"` 或 `["./cmd1.md"]`                |
| `agents`                | string\|array         | 自訂 agent 檔案（取代預設 `agents/`）                                                                                 | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Hook 設定路徑或內聯設定                                                                                              | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | MCP 設定路徑或內聯設定                                                                                               | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | 自訂輸出樣式檔案/目錄（取代預設 `output-styles/`）                                                                          | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) 設定，用於程式碼智慧（前往定義、尋找參考等）    | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | 色彩主題檔案/目錄（取代預設 `themes/`）。請參閱[主題](#themes)                                                                  | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | 背景 [Monitor](/docs/zh-TW/tools-reference#monitor-tool) 設定，在 plugin 啟用時自動啟動。請參閱[監視器](#monitors)                   | `"./monitors.json"`                                  |
| `userConfig`            | object                | 在啟用時提示使用者的使用者可設定值。請參閱[使用者設定](#user-configuration)                                                           | 請參閱下方                                                |
| `channels`              | array                 | 訊息注入的頻道宣告（Telegram、Slack、Discord 風格）。請參閱[頻道](#channels)                                                     | 請參閱下方                                                |
| `dependencies`          | array                 | 此 plugin 需要的其他 plugins，可選擇使用 semver 版本限制。請參閱[限制 plugin 相依性版本](/docs/zh-TW/plugin-dependencies)                   | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  實驗性元件
</h3>

`experimental` 金鑰下的元件 `themes` 和 `monitors` 具有在版本之間穩定時可能會變更的 manifest 架構。您宣告它們的位置是一個單獨的遷移：頂層仍然有效，`claude plugin validate` 會發出警告，未來的版本將需要 `experimental.*`。

<h3 id="user-configuration">
  使用者設定
</h3>

`userConfig` 欄位宣告 Claude Code 在啟用 plugin 時提示使用者的值。使用此方法而不是要求使用者手動編輯 `settings.json`。

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

金鑰必須是有效的識別碼。每個選項支援這些欄位：

| 欄位            | 必需  | 描述                                                    |
| :------------ | :-- | :---------------------------------------------------- |
| `type`        | Yes | 其中之一：`string`、`number`、`boolean`、`directory` 或 `file` |
| `title`       | Yes | 設定對話方塊中顯示的標籤                                          |
| `description` | Yes | 欄位下方顯示的說明文字                                           |
| `sensitive`   | No  | 如果 `true`，遮罩輸入並將值儲存在安全儲存體中，而不是 `settings.json`        |
| `required`    | No  | 如果 `true`，當欄位為空時驗證失敗                                  |
| `default`     | No  | 使用者未提供任何內容時使用的值                                       |
| `multiple`    | No  | 對於 `string` 類型，允許字串陣列                                 |
| `min` / `max` | No  | `number` 類型的界限                                        |

每個值都可用於在 MCP 和 LSP server 設定和 hook 命令中替換為 `${user_config.KEY}`。非敏感值也可以在 skill 和 agent 內容中替換。所有值都會匯出到 hook 程序作為 `CLAUDE_PLUGIN_OPTION_<KEY>` 環境變數，其中 `<KEY>` 是選項金鑰大寫。

在 shell 中執行的欄位拒絕 `${user_config.*}`：將設定的值替換到 shell 命令中會讓 shell 執行該值包含的任何內容，因此元件會失敗並出現[錯誤](/docs/zh-TW/errors#plugin-command-references-user-config)。每個被拒絕的欄位都有一個替代方式來傳遞值：

| 被拒絕的欄位                                                                          | 如何傳遞值                                                                                               |
| :------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------- |
| Shell 形式的 hook 命令                                                               | 使用[執行形式](/docs/zh-TW/hooks#exec-form-and-shell-form)搭配 `args`，或從 hook 的環境讀取 `CLAUDE_PLUGIN_OPTION_<KEY>` |
| [Monitor](#monitors) 命令                                                         | 從指令碼中的設定檔讀取值                                                                                        |
| MCP [`headersHelper`](/docs/zh-TW/mcp#use-dynamic-headers-for-custom-authentication) | 從指令碼中的設定檔讀取值                                                                                        |

在 v2.1.207 之前，這些欄位替換 `${user_config.KEY}` 值；更新依賴此功能的 plugins。

非敏感值儲存在 `settings.json` 中的 [`pluginConfigs`](/docs/zh-TW/settings#pluginconfigs) 金鑰下，作為 `pluginConfigs[<plugin-id>].options`。Claude Code 將金鑰寫入使用者設定並從使用者設定、`--settings` 旗標和受管設定讀取；專案的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的項目會被忽略。在 v2.1.207 之前，Claude Code 也讀取專案和本地設定。

敏感值進入 macOS Keychain，或在沒有支援的 keychain 可用的平台上進入 `~/.claude/.credentials.json`。Keychain 儲存與 OAuth 令牌共享，總限制約為 2 KB，因此請保持敏感值較小。

<h3 id="channels">
  頻道
</h3>

`channels` 欄位讓 plugin 宣告一個或多個訊息頻道，將內容注入對話中。每個頻道繫結到 plugin 提供的 MCP server。

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "Owner ID",
          "description": "Your Telegram user ID"
        }
      }
    }
  ]
}
```

`server` 欄位是必需的，必須與 plugin 的 `mcpServers` 中的金鑰相符。選用的每個頻道 `userConfig` 使用與頂層欄位相同的架構，讓 plugin 在啟用 plugin 時提示輸入機器人令牌或擁有者 ID。

<h3 id="path-behavior-rules">
  路徑行為規則
</h3>

自訂路徑是否取代或擴展 plugin 的預設目錄取決於欄位：

* **取代預設值**：`commands`、`agents`、`outputStyles`、`experimental.themes`、`experimental.monitors`。例如，當 manifest 指定 `commands` 時，預設 `commands/` 目錄不會被掃描。若要保留預設值並新增更多，請明確列出：`"commands": ["./commands/", "./extras/"]`
* **新增到預設值**：`skills`。預設 `skills/` 目錄始終被掃描，`skills` 中列出的目錄與其一起載入。例外：對於[其 `source` 解析為 marketplace 根目錄的 marketplace 項目](/docs/zh-TW/plugin-marketplaces#advanced-plugin-entries)，宣告特定子目錄會取代掃描
* **自有合併規則**：[hooks](#hooks)、[MCP servers](#mcp-servers) 和 [LSP servers](#lsp-servers)。請參閱每個部分以了解多個來源如何組合

當 plugin 同時具有預設資料夾和相符的 manifest 金鑰時，Claude Code v2.1.140 及更新版本會在 `claude plugin list` 和 `/plugin` 詳細檢視中標記被忽略的資料夾。plugin 仍會使用 manifest 路徑載入。當 manifest 金鑰指向預設資料夾時不會顯示警告，例如 `"commands": ["./commands/deploy.md"]`，因為在這種情況下資料夾是明確定址的。

對於所有路徑欄位：

* 所有路徑必須相對於 plugin 根目錄，並以 `./` 開頭
* 來自自訂路徑的元件使用相同的命名和命名空間規則
* 可以將多個路徑指定為陣列
* 當 skill 路徑指向直接包含 `SKILL.md` 的目錄時，例如 `"skills": ["./"]` 指向 plugin 根目錄，frontmatter 中的 `name` 欄位決定 skill 的叫用名稱。這提供了一個穩定的名稱，無論安裝目錄如何。如果 frontmatter 中未設定 `name`，目錄基名將用作後備。

在其根目錄中具有 `SKILL.md`、沒有 `skills/` 子目錄且沒有 `skills` manifest 欄位的 plugin 在 Claude Code v2.1.142 及更新版本中會自動載入為單一 skill plugin。您不需要在 `plugin.json` 中設定 `"skills": ["./"]` 來進行此配置。skill 的叫用名稱遵循相同的規則：frontmatter `name` 欄位，或目錄基名作為後備。

**路徑範例**：

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  環境變數
</h3>

Claude Code 提供三個變數用於參考路徑：

| 變數                      | 解析為                                                        | 用途                                                |
| :---------------------- | :--------------------------------------------------------- | :------------------------------------------------ |
| `${CLAUDE_PLUGIN_ROOT}` | plugin 安裝目錄的絕對路徑                                           | 與 plugin 捆綁的指令碼、二進位檔和設定檔                          |
| `${CLAUDE_PLUGIN_DATA}` | [持久目錄](#persistent-data-directory)，在首次參考時建立，在 plugin 更新後保留 | 已安裝的依賴項，例如 `node_modules` 或 Python 虛擬環境、生成的程式碼和快取 |
| `${CLAUDE_PROJECT_DIR}` | 專案根目錄                                                      | 專案本地指令碼和設定檔                                       |

所有三個都會匯出為環境變數到 hook 程序和 MCP 及 LSP server 子程序。哪些欄位內聯替換它們取決於 plugin 元件：

| Plugin 元件                     | 佔位符解析的欄位                                 |
| :---------------------------- | :--------------------------------------- |
| Skill 和 agent 內容              | 佔位符出現的任何地方                               |
| Hook 和 monitor 命令             | 佔位符出現的任何地方                               |
| MCP `stdio` servers           | `command`、`args`、`env`                   |
| MCP `http`、`sse`、`ws` servers | `url`、`headers`、`headersHelper`          |
| LSP servers                   | `command`、`args`、`env`、`workspaceFolder` |

在 hook 命令中，使用[執行形式](/docs/zh-TW/hooks#exec-form-and-shell-form)搭配 `args`，以便每個路徑作為一個引數傳遞，無需引號。在 shell 形式的 hooks 和 monitor 命令中，將變數包裝在雙引號中，如 `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`。此 shell 形式的 hook 執行與 plugin 捆綁的指令碼：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` 在 plugin 更新時會變更。前一個版本的目錄在更新後約七天內保留在磁碟上，然後才進行清理，但應將其視為暫時性的，不要在此處寫入狀態。

當 plugin 在工作階段中途更新時，hook 命令、monitors、MCP servers 和 LSP servers 會繼續使用前一個版本的路徑。執行 `/reload-plugins` 以將 hooks、MCP servers 和 LSP servers 切換到新路徑；monitors 需要工作階段重新啟動。

MCP servers 也可以呼叫 `roots/list` 請求以在執行時讀取工作階段的工作目錄。請參閱[`roots/list` 傳回的內容以及 Claude Code 何時通知伺服器變更](/docs/zh-TW/mcp#option-3-add-a-local-stdio-server)。

<h4 id="persistent-data-directory">
  持久資料目錄
</h4>

`${CLAUDE_PLUGIN_DATA}` 目錄解析為 `~/.claude/plugins/data/{id}/`，其中 `{id}` 是 plugin 識別碼，其中 `a-z`、`A-Z`、`0-9`、`_` 和 `-` 以外的字元被替換為 `-`。對於安裝為 `formatter@my-marketplace` 的 plugin，目錄是 `~/.claude/plugins/data/formatter-my-marketplace/`。

常見用途是一次安裝語言依賴項並在工作階段和 plugin 更新中重複使用它們。因為資料目錄的壽命超過任何單一 plugin 版本，僅檢查目錄存在無法偵測更新何時變更 plugin 的依賴項清單。建議的模式是比較捆綁的清單與資料目錄中的副本，並在它們不同時重新安裝。

此 `SessionStart` hook 在第一次執行時安裝 `node_modules`，並在 plugin 更新包含變更的 `package.json` 時再次安裝：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

`diff` 在儲存的副本遺失或與捆綁的副本不同時以非零值退出，涵蓋第一次執行和依賴項變更更新。如果 `npm install` 失敗，尾部 `rm` 會移除複製的清單，以便下一個工作階段重試。

捆綁在 `${CLAUDE_PLUGIN_ROOT}` 中的指令碼可以針對保留的 `node_modules` 執行：

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

當您從最後一個安裝 plugin 的範圍卸載 plugin 時，資料目錄會自動刪除。`/plugin` 介面顯示目錄大小並在刪除前提示。CLI 預設刪除；傳遞 [`--keep-data`](#plugin-uninstall) 以保留它。

***

<h2 id="plugin-caching-and-file-resolution">
  Plugin 快取和檔案解析
</h2>

Plugins 可以透過以下兩種方式之一指定：

* 透過 `claude --plugin-dir` 或 `claude --plugin-url`，在工作階段期間。
* 透過 marketplace，為未來的工作階段安裝。

出於安全和驗證目的，Claude Code 將 *marketplace* plugins 複製到使用者的本機 **plugin 快取**（`~/.claude/plugins/cache`），而不是就地使用它們。在開發參考外部檔案的 plugins 時，理解此行為很重要。

每個已安裝的版本是快取中的單獨目錄。當您更新或卸載 plugin 時，先前的版本目錄被標記為孤立，並在 7 天後自動移除。寬限期讓已載入舊版本的並行 Claude Code 工作階段繼續執行而不出錯。

Claude 的 Glob 和 Grep 工具在搜尋期間跳過孤立的版本目錄，因此檔案結果不包含過時的 plugin 程式碼。

<h3 id="path-traversal-limitations">
  路徑遍歷限制
</h3>

已安裝的 plugins 無法參考其目錄外的檔案。遍歷 plugin 根目錄外的路徑（例如 `../shared-utils`）在安裝後將無法運作，因為這些外部檔案不會複製到快取中。

<h3 id="share-files-within-a-marketplace-with-symlinks">
  使用 symlinks 在 marketplace 內共享檔案
</h3>

如果您的 plugin 需要與同一 marketplace 的其他部分共享檔案，您可以在 plugin 目錄內建立符號連結。當 plugin 被複製到快取時，symlink 的處理方式取決於其目標的解析位置：

* **在 plugin 自身目錄內：** symlink 在快取中被保留為相對 symlink，因此在執行時繼續解析到複製的目標。
* **在同一 marketplace 內的其他位置：** symlink 被取消參考。目標的內容被複製到快取中以取代它。這讓 meta-plugin 的 `skills/` 目錄可以連結到 marketplace 中其他 plugins 定義的 skills。
* **在 marketplace 外：** symlink 因安全考量而被跳過。這防止 plugins 將任意主機檔案（例如系統路徑）拉入快取。

對於使用 `--plugin-dir` 安裝或從本機路徑安裝的 plugins，只有解析在 plugin 自身目錄內的 symlinks 被保留。所有其他的都被跳過。

以下命令從 marketplace plugin 內建立到由同級 plugin 定義的共享 skill 的連結。在 Windows 上，從提升的命令提示字元使用 `mklink /D` 或啟用開發人員模式：

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

這在維持快取系統安全優勢的同時提供了靈活性。

***

<h2 id="plugin-directory-structure">
  Plugin 目錄結構
</h2>

<h3 id="standard-plugin-layout">
  標準 plugin 配置
</h3>

完整的 plugin 遵循此結構：

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # Metadata directory (optional)
│   └── plugin.json             # plugin manifest
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # Skills as flat .md files
│   ├── status.md
│   └── logs.md
├── agents/                   # Subagent definitions
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # Output style definitions
│   └── terse.md
├── themes/                   # Color theme definitions
│   └── dracula.json
├── monitors/                 # Background monitor configurations
│   └── monitors.json
├── hooks/                    # Hook configurations
│   ├── hooks.json           # Main hook config
│   └── security-hooks.json  # Additional hooks
├── bin/                      # Plugin executables added to PATH
│   └── my-tool               # Invokable as bare command in Bash tool
├── settings.json            # Default settings for the plugin
├── .mcp.json                # MCP server definitions
├── .lsp.json                # LSP server configurations
├── scripts/                 # Hook and utility scripts
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # License file
└── CHANGELOG.md             # Version history
```

<Warning>
  `.claude-plugin/` 目錄包含 `plugin.json` 檔案。所有其他目錄（commands/、agents/、skills/、output-styles/、themes/、monitors/、hooks/）必須位於 plugin 根目錄，而不是在 `.claude-plugin/` 內。
</Warning>

plugin 根目錄的 `CLAUDE.md` 檔案不會作為專案內容載入。Plugin 透過 skills、agents 和 hooks 貢獻內容，而不是透過 CLAUDE.md。若要提供載入到 Claude 內容中的指示，請將其放在 [skill](#skills) 中。

<h3 id="file-locations-reference">
  檔案位置參考
</h3>

| 元件                | 預設位置                         | 用途                                                                                                                         |
| :---------------- | :--------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| **Manifest**      | `.claude-plugin/plugin.json` | Plugin 中繼資料和設定（選用）                                                                                                         |
| **Skills**        | `skills/`                    | 具有 `<name>/SKILL.md` 結構的 Skills                                                                                            |
| **Commands**      | `commands/`                  | 作為平面 Markdown 檔案的 Skills。新 plugins 使用 `skills/`                                                                            |
| **Agents**        | `agents/`                    | Subagent Markdown 檔案                                                                                                       |
| **Output styles** | `output-styles/`             | 輸出樣式定義                                                                                                                     |
| **Themes**        | `themes/`                    | 色彩主題定義                                                                                                                     |
| **Hooks**         | `hooks/hooks.json`           | Hook 設定                                                                                                                    |
| **MCP servers**   | `.mcp.json`                  | MCP server 定義                                                                                                              |
| **LSP servers**   | `.lsp.json`                  | 語言伺服器設定                                                                                                                    |
| **Monitors**      | `monitors/monitors.json`     | 背景 monitor 設定                                                                                                              |
| **Executables**   | `bin/`                       | 新增到 Bash tool 的 `PATH` 的可執行檔。此處的檔案在 plugin 啟用時可在任何 Bash tool 呼叫中作為裸命令叫用                                                    |
| **Settings**      | `settings.json`              | 啟用 plugin 時套用的預設設定。目前僅支援 [`agent`](/docs/zh-TW/sub-agents) 和 [`subagentStatusLine`](/docs/zh-TW/statusline#subagent-status-lines) 金鑰 |

***

<h2 id="cli-commands-reference">
  CLI 命令參考
</h2>

Claude Code 提供 CLI 命令用於非互動式 plugin 管理，適用於指令碼和自動化。

<h3 id="plugin-init">
  plugin init
</h3>

在 `~/.claude/skills/<name>/` 搭建新 plugin。在下一個 Claude Code 工作階段中，它會自動作為 `<name>@skills-dir` 載入，並出現在 `/plugin` 和 `claude plugin list` 中，無需安裝步驟。

請參閱 [Skills-directory plugins](#skills-directory-plugins) 以了解範圍和信任要求。

```bash theme={null}
claude plugin init <name> [options]
```

**引數：**

* `<name>`：Plugin 名稱。成為 skill 命名空間和 `~/.claude/skills/` 下的目錄名稱，因此不能包含空格或路徑分隔符。

**選項：**

| 選項                       | 描述                                                                           | 預設                      |
| :----------------------- | :--------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | Manifest 描述                                                                  |                         |
| `--author <name>`        | 作者名稱                                                                         | `git config user.name`  |
| `--author-email <email>` | 作者電子郵件                                                                       | `git config user.email` |
| `--with <components...>` | 同時搭建元件資料夾。有效值：`skills`、`agents`、`hooks`、`mcp`、`lsp`、`output-style`、`channel` |                         |
| `-f, --force`            | 覆寫目標的現有 `.claude-plugin/`                                                    |                         |
| `-h, --help`             | 顯示命令說明                                                                       |                         |

**別名：** `new`

每個 `--with` 值都會為該元件新增一個入門檔案，準備好編輯：

| 元件             | 它搭建什麼                                                                                                |
| :------------- | :--------------------------------------------------------------------------------------------------- |
| `skills`       | 一個額外的命名空間 `<name>:example` skill，與預設 skill 一起                                                        |
| `agents`       | 一個 `agents/` subagent 定義                                                                             |
| `hooks`        | 一個 `hooks/hooks.json`，包含範例事件處理程式                                                                     |
| `mcp`          | 一個 `.mcp.json`，包含 HTTP 和 stdio server 範例                                                             |
| `lsp`          | 一個 `.lsp.json` 語言伺服器範例                                                                               |
| `output-style` | 一個 `output-styles/<name>.md`，在 plugin 啟用時自動套用                                                        |
| `channel`      | 一個基於 MCP 的 [channel](/docs/zh-TW/channels)：一個 stdio server (`server.ts`)、其 `.mcp.json` 和一個 `package.json` |

搭建的 plugin 使用 `@skills-dir` 來源而不是 marketplace。管理員可以使用 `strictKnownMarketplaces` 或透過在 [managed settings](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions) 中新增 `{"source": "skills-dir"}` 到 `blockedMarketplaces` 來阻止此來源。當被阻止時，`plugin init` 在寫入前失敗。

**範例：**

```bash theme={null}
# Scaffold a minimal plugin
claude plugin init my-helper

# Scaffold with skill and hook folders
claude plugin init my-helper --with skills hooks

# Overwrite an existing scaffold
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

從可用的 marketplaces 安裝 plugin。

```bash theme={null}
claude plugin install <plugin> [options]
```

**引數：**

* `<plugin>`：Plugin 名稱或 `plugin-name@marketplace-name` 用於特定 marketplace

**選項：**

| 選項                     | 描述                                                                            | 預設     |
| :--------------------- | :---------------------------------------------------------------------------- | :----- |
| `-s, --scope <scope>`  | 安裝範圍：`user`、`project` 或 `local`                                               | `user` |
| `--config <key=value>` | 設定 plugin manifest 中宣告的 [`userConfig`](#user-configuration) 選項。重複使用此旗標以設定多個選項 |        |
| `-h, --help`           | 顯示命令說明                                                                        |        |

範圍決定已安裝的 plugin 新增到哪個設定檔。例如，`--scope project` 寫入 `.claude/settings.json` 中的 `enabledPlugins`，使 plugin 對克隆專案存放庫的每個人都可用。

**範例：**

```bash theme={null}
# Install to user scope (default)
claude plugin install formatter@my-marketplace

# Install to project scope (shared with team)
claude plugin install formatter@my-marketplace --scope project

# Install to local scope (gitignored)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

移除已安裝的 plugin。

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**引數：**

* `<plugin>`：Plugin 名稱或 `plugin-name@marketplace-name`

**選項：**

| 選項                    | 描述                                                                  | 預設     |
| :-------------------- | :------------------------------------------------------------------ | :----- |
| `-s, --scope <scope>` | 從範圍卸載：`user`、`project` 或 `local`                                    | `user` |
| `--keep-data`         | 保留 plugin 的 [persistent data directory](#persistent-data-directory) |        |
| `--prune`             | 同時移除其他 plugin 不需要的自動安裝相依性。請參閱 [plugin prune](#plugin-prune)         |        |
| `-y, --yes`           | 跳過 `--prune` 確認提示。當 stdin 或 stdout 不是 TTY 時為必需                      |        |
| `-h, --help`          | 顯示命令說明                                                              |        |

**別名：** `remove`、`rm`

預設情況下，從最後一個剩餘範圍卸載也會刪除 plugin 的 `${CLAUDE_PLUGIN_DATA}` 目錄。使用 `--keep-data` 保留它，例如在測試新版本後重新安裝時。

<Note>
  當來自不同 marketplaces 的已安裝 plugins 共用名稱時，`plugin-name@marketplace-name` 形式只會卸載指定 marketplace 的 plugin。在 v2.1.212 之前，限定形式可能會比對並卸載來自不同 marketplace 的同名 plugin。
</Note>

<h3 id="plugin-prune">
  plugin prune
</h3>

移除不再被任何已安裝 plugin 所需的自動安裝 plugin 相依性。Claude Code 為滿足另一個 plugin 的 [`dependencies`](/docs/zh-TW/plugin-dependencies) 欄位而引入的相依性會被移除；您直接安裝的 plugins 永遠不會被觸及。

```bash theme={null}
claude plugin prune [options]
```

**選項：**

| 選項                    | 描述                                  | 預設     |
| :-------------------- | :---------------------------------- | :----- |
| `-s, --scope <scope>` | 在範圍進行修剪：`user`、`project` 或 `local`  | `user` |
| `--dry-run`           | 列出將被移除的內容而不實際移除                     |        |
| `-y, --yes`           | 跳過確認提示。當 stdin 或 stdout 不是 TTY 時為必需 |        |
| `-h, --help`          | 顯示命令說明                              |        |

**別名：** `autoremove`

該命令列出孤立的相依性並在移除前要求確認。若要在一個步驟中移除 plugin 並清理其相依性，請執行 `claude plugin uninstall <plugin> --prune`。

<Note>
  `claude plugin prune` 需要 Claude Code v2.1.121 或更新版本。
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

啟用已停用的 plugin。如果 plugin 宣告 [dependencies](/docs/zh-TW/plugin-dependencies)，Claude Code 會在相同範圍內以傳遞方式啟用它們，當相依性未安裝時命令會失敗。

```bash theme={null}
claude plugin enable <plugin> [options]
```

**引數：**

* `<plugin>`：Plugin 名稱或 `plugin-name@marketplace-name`

**選項：**

| 選項                    | 描述                                                                   | 預設   |
| :-------------------- | :------------------------------------------------------------------- | :--- |
| `-s, --scope <scope>` | 要啟用的範圍：`user`、`project` 或 `local`。省略時，Claude Code 會偵測 plugin 安裝所在的範圍 | 自動偵測 |
| `-h, --help`          | 顯示命令說明                                                               |      |

<h3 id="plugin-disable">
  plugin disable
</h3>

停用 plugin 而不卸載它。當另一個已啟用的 plugin [depends on](/docs/zh-TW/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) 目標時失敗。錯誤訊息包含一個鏈式命令，該命令會先停用每個相依項。

```bash theme={null}
claude plugin disable [plugin] [options]
```

**引數：**

* `[plugin]`：Plugin 名稱或 `plugin-name@marketplace-name`。使用 `--all` 時可省略

**選項：**

| 選項                    | 描述                                                                   | 預設   |
| :-------------------- | :------------------------------------------------------------------- | :--- |
| `-a, --all`           | 停用所有已啟用的 plugins。不能與 `--scope` 結合使用                                  |      |
| `-s, --scope <scope>` | 要停用的範圍：`user`、`project` 或 `local`。省略時，Claude Code 會偵測 plugin 安裝所在的範圍 | 自動偵測 |
| `-h, --help`          | 顯示命令說明                                                               |      |

<h3 id="plugin-update">
  plugin update
</h3>

將 plugin 更新到最新版本。

```bash theme={null}
claude plugin update <plugin> [options]
```

**引數：**

* `<plugin>`：Plugin 名稱或 `plugin-name@marketplace-name`

**選項：**

| 選項                    | 描述                                          | 預設     |
| :-------------------- | :------------------------------------------ | :----- |
| `-s, --scope <scope>` | 要更新的範圍：`user`、`project`、`local` 或 `managed` | `user` |
| `-h, --help`          | 顯示命令說明                                      |        |

***

<h3 id="plugin-list">
  plugin list
</h3>

列出已安裝的 plugins 及其版本、來源 marketplace 和啟用狀態。

```bash theme={null}
claude plugin list [options]
```

**選項：**

| 選項            | 描述                                        | 預設 |
| :------------ | :---------------------------------------- | :- |
| `--json`      | 輸出為 JSON                                  |    |
| `--available` | 包含來自 marketplaces 的可用 plugins。需要 `--json` |    |
| `-h, --help`  | 顯示命令說明                                    |    |

在互動式工作階段中，`/plugin list` 會內嵌列印相同的列表。互動式表單接受 `--enabled` 或 `--disabled` 以僅顯示該狀態中的 plugins，以及 `ls` 作為 `list` 的簡寫。

<h3 id="plugin-details">
  plugin details
</h3>

顯示 plugin 的元件清單和預計的 token 成本。輸出列出 plugin 貢獻的所有元件，分組為 Skills、Agents、Hooks、MCP servers 和 LSP servers，以及它為每個工作階段新增多少 tokens 的估計。Skills 群組包括 `skills/` 和 `commands/` 項目。

```bash theme={null}
claude plugin details <name>
```

**引數：**

* `<name>`：Plugin 名稱或 `plugin-name@marketplace-name`

**選項：**

| 選項           | 描述     | 預設 |
| :----------- | :----- | :- |
| `-h, --help` | 顯示命令說明 |    |

輸出為每個元件顯示兩個成本數字：

* **Always-on：** plugin 的列表文字新增到每個工作階段的 tokens，例如技能描述、agent 描述和命令名稱，無論任何元件是否觸發。
* **On-invoke：** 元件觸發時的成本。按元件顯示，而不是作為 plugin 總計，因為典型的工作階段只會呼叫元件的子集。

此範例顯示具有兩個技能的 plugin 的輸出外觀：

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

always-on 總計是透過您的作用中模型的 `count_tokens` API 計算的。按元件的數字按比例從該總計縮放。如果 API 無法連線，該命令會回退到基於字元的估計。

<h3 id="plugin-tag">
  plugin tag
</h3>

為 plugin 建立發行版 git 標籤。預設情況下，命令會標記目前目錄中的 plugin；傳遞路徑即可標記其他位置的 plugin。請參閱 [Tag plugin releases](/docs/zh-TW/plugin-dependencies#tag-plugin-releases-for-version-resolution)。

```bash theme={null}
claude plugin tag [path] [options]
```

**引數：**

* `[path]`：Plugin 目錄的路徑。預設為目前目錄。

**選項：**

| 選項                    | 描述                      | 預設       |
| :-------------------- | :---------------------- | :------- |
| `--push`              | 建立標籤後將其推送到遠端            |          |
| `--dry-run`           | 列印將被標籤的內容而不建立標籤         |          |
| `-f, --force`         | 即使工作樹髒污或標籤已存在也建立標籤      |          |
| `-m, --message <msg>` | 標籤註解訊息。使用 `%s` 作為版本的佔位符 |          |
| `--remote <name>`     | 使用 `--push` 時推送到的遠端     | `origin` |
| `-h, --help`          | 顯示命令說明                  |          |

***

<h2 id="debugging-and-development-tools">
  偵錯和開發工具
</h2>

<h3 id="debugging-commands">
  偵錯命令
</h3>

使用 `claude --debug` 查看 plugin 載入詳細資訊：

這會顯示：

* 正在載入哪些 plugins
* plugin manifests 中的任何錯誤
* Skill、agent 和 hook 註冊
* MCP server 初始化

<h3 id="common-issues">
  常見問題
</h3>

| 問題                                  | 原因                         | 解決方案                                                                                                                            |
| :---------------------------------- | :------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| Plugin 未載入                          | 無效的 `plugin.json`          | 執行 `claude plugin validate` 或 `/plugin validate` 檢查 `plugin.json`、skill/agent/command frontmatter 和 `hooks/hooks.json` 的語法和架構錯誤 |
| Skills 未出現                          | 目錄結構錯誤                     | 確保 `skills/` 或 `commands/` 在 plugin 根目錄，而不是在 `.claude-plugin/` 中                                                                |
| Hooks 未觸發                           | 指令碼不可執行                    | 執行 `chmod +x script.sh`                                                                                                         |
| MCP server 失敗                       | 缺少 `${CLAUDE_PLUGIN_ROOT}` | 對所有 plugin 路徑使用變數                                                                                                               |
| 路徑錯誤                                | 使用了絕對路徑                    | 所有路徑必須是相對的，並以 `./` 開頭                                                                                                           |
| LSP `Executable not found in $PATH` | 未安裝語言伺服器                   | 安裝二進位檔（例如 `npm install -g typescript-language-server typescript`）                                                               |

<h3 id="example-error-messages">
  範例錯誤訊息
</h3>

**Manifest 驗證錯誤**：

* `Invalid JSON syntax: Unexpected token } in JSON at position 142`：檢查是否缺少逗號、多餘逗號或未引用的字串
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required`：缺少必需欄位
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...`：JSON 語法錯誤

**Plugin 載入錯誤**：

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.`：命令路徑存在但不包含有效的命令檔案
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.`：marketplace.json 中的 `source` 路徑指向不存在的目錄
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.`：移除重複的元件定義或移除 marketplace 項目中的 `strict: false`

<h3 id="hook-troubleshooting">
  Hook 疑難排解
</h3>

**Hook 指令碼未執行**：

1. 檢查指令碼是否可執行：`chmod +x ./scripts/your-script.sh`
2. 驗證 shebang 行：第一行應為 `#!/bin/bash` 或 `#!/usr/bin/env bash`
3. 檢查路徑是否使用 `${CLAUDE_PLUGIN_ROOT}`：`"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. 手動測試指令碼：`./scripts/your-script.sh`

**Hook 未在預期事件上觸發**：

1. 驗證事件名稱是否正確（區分大小寫）：`PostToolUse`，而不是 `postToolUse`
2. 檢查匹配器模式是否與您的工具相符：`"matcher": "Write|Edit"` 用於檔案操作
3. 確認 hook 類型有效：`command`、`http`、`mcp_tool`、`prompt` 或 `agent`

<h3 id="mcp-server-troubleshooting">
  MCP server 疑難排解
</h3>

**Server 未啟動**：

1. 檢查命令是否存在且可執行
2. 驗證所有路徑是否使用 `${CLAUDE_PLUGIN_ROOT}` 變數
3. 檢查 MCP server 日誌：`claude --debug` 顯示初始化錯誤
4. 在 Claude Code 外手動測試 server

**Server 工具未出現**：

1. 確保 server 在 `.mcp.json` 或 `plugin.json` 中正確設定
2. 驗證 server 是否正確實現 MCP 協定
3. 檢查偵錯輸出中的連接逾時

<h3 id="directory-structure-mistakes">
  目錄結構錯誤
</h3>

**症狀**：Plugin 載入但元件（skills、agents、hooks）遺失。

**正確結構**：元件必須位於 plugin 根目錄，而不是在 `.claude-plugin/` 內。只有 `plugin.json` 屬於 `.claude-plugin/`。

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← Only manifest here
├── commands/            ← At root level
├── agents/              ← At root level
└── hooks/               ← At root level
```

如果您的元件在 `.claude-plugin/` 內，請將它們移到 plugin 根目錄。

**偵錯檢查清單**：

1. 執行 `claude --debug` 並查找「loading plugin」訊息
2. 檢查每個元件目錄是否列在偵錯輸出中
3. 驗證檔案權限允許讀取 plugin 檔案

***

<h2 id="distribution-and-versioning-reference">
  發佈和版本控制參考
</h2>

<h3 id="version-management">
  版本管理
</h3>

Claude Code 使用 plugin 的版本作為快取金鑰，以決定是否有可用的更新。當您執行 `/plugin update` 或自動更新觸發時，Claude Code 會計算目前版本，如果與已安裝的版本相符，則跳過更新。

版本會從以下第一個設定的項目解析：

1. Plugin 的 `plugin.json` 中的 `version` 欄位
2. Plugin 在 `marketplace.json` 中的 marketplace 項目中的 `version` 欄位
3. Plugin 來源的 git commit SHA，適用於 git 託管 marketplace 中的 `github`、`url`、`git-subdir` 和相對路徑來源
4. `unknown`，適用於 `npm` 來源或不在 git 儲存庫內的本機目錄

這為您提供了兩種方式來版本化 plugin：

| 方法                | 如何操作                                          | 更新行為                                                                     | 最適合                  |
| :---------------- | :-------------------------------------------- | :----------------------------------------------------------------------- | :------------------- |
| **明確版本**          | 在 `plugin.json` 中設定 `"version": "2.1.0"`      | 使用者只有在您提升此欄位時才會獲得更新。推送新的 commit 而不提升版本沒有效果，`/plugin update` 會報告「已是最新版本」。 | 具有穩定發行週期的已發佈 plugin  |
| **Commit-SHA 版本** | 從 `plugin.json` 和 marketplace 項目中省略 `version` | 使用者在每次 plugin 的 git 來源有新 commit 時都會獲得更新                                  | 正在積極開發中的內部或團隊 plugin |

<Warning>
  如果您在 `plugin.json` 中設定 `version`，每次您想讓使用者接收變更時，都必須提升它。僅推送新的 commit 是不夠的，因為 Claude Code 會看到相同的版本字串並保留快取副本。如果您正在快速迭代，請保持 `version` 未設定，以便改為使用 git commit SHA。
</Warning>

如果您使用明確版本，請遵循 [semantic versioning](https://semver.org)（`MAJOR.MINOR.PATCH`）：針對破壞性變更提升 MAJOR，針對新功能提升 MINOR，針對錯誤修正提升 PATCH。在 `CHANGELOG.md` 中記錄變更。

***

<h2 id="see-also">
  另請參閱
</h2>

* [Plugins](/docs/zh-TW/plugins) - 教學和實際使用
* [Plugin marketplaces](/docs/zh-TW/plugin-marketplaces) - 建立和管理 marketplaces
* [Skills](/docs/zh-TW/skills) - Skill 開發詳細資訊
* [Subagents](/docs/zh-TW/sub-agents) - Agent 設定和功能
* [Hooks](/docs/zh-TW/hooks) - 事件處理和自動化
* [MCP](/docs/zh-TW/mcp) - 外部工具整合
* [Settings](/docs/zh-TW/settings) - Plugins 的設定選項
