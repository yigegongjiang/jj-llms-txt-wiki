> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 建立自訂 subagents

> 在 Claude Code 中建立和使用專門的 AI subagents，用於特定任務的工作流程和改進的上下文管理。

Subagents 是專門的 AI 助手，用於處理特定類型的任務。當側面任務會用搜尋結果、日誌或檔案內容淹沒您的主要對話時，請使用一個 subagent，而您不會再次參考這些內容：subagent 在自己的上下文中執行該工作，並僅返回摘要。當您持續產生相同類型的工作者並使用相同指令時，定義自訂 subagent。

每個 subagent 在自己的 context window 中執行，具有自訂系統提示、特定工具存取和獨立權限。當 Claude 遇到與 subagent 描述相符的任務時，它會委派給該 subagent，該 subagent 獨立工作並返回結果。若要在實踐中查看上下文節省，[context window visualization](/docs/zh-TW/context-window) 會逐步說明一個 subagent 在自己的獨立視窗中處理研究的工作階段。

<Note>
  Subagents 在單一工作階段內工作。若要執行許多獨立工作階段並行並從一個地方監控它們，請參閱 [background agents](/docs/zh-TW/agent-view)。對於相互通訊的工作階段，請參閱 [agent teams](/docs/zh-TW/agent-teams)。
</Note>

Subagents 可以幫助您：

* **保留上下文**，將探索和實現保持在主要對話之外
* **強制執行約束**，限制 subagent 可以使用的工具
* **跨專案重複使用配置**，使用使用者層級的 subagents
* **專門化行為**，針對特定領域使用專注的系統提示
* **控制成本**，將任務路由到更快、更便宜的模型，如 Haiku

Claude 使用每個 subagent 的描述來決定何時委派任務。當您建立 subagent 時，請寫一個清晰的描述，以便 Claude 知道何時使用它。

Claude Code 包括幾個內建 subagents，如 Explore、Plan 和 general-purpose。您也可以建立自訂 subagents 來處理特定任務。

<h2 id="built-in-subagents">
  內建 subagents
</h2>

Claude Code 包括內建 subagents，Claude 在適當時會自動使用。每個都繼承父對話的權限，並有額外的工具限制。

Explore 和 Plan 會跳過您的 CLAUDE.md 檔案和父工作階段的 git status，以保持研究快速且經濟高效。其他所有內建和[自訂 subagent](#configure-subagents) 都會載入兩者。如需了解到達 subagent 的完整詳細資訊，請參閱[啟動時載入的內容](#what-loads-at-startup)。

<Tabs>
  <Tab title="Explore">
    一個快速、唯讀的代理，針對搜尋和分析程式碼庫進行最佳化。

    * **Model**：繼承自主要對話，在 Claude API 上限制為 Opus，因此 Explore 永遠不會在比您已為工作階段選擇的模型更昂貴的模型上執行
    * **Tools**：唯讀工具；Write 和 Edit 被拒絕
    * **Purpose**：檔案發現、程式碼搜尋、程式碼庫探索

    自 v2.1.198 起，Explore 繼承主要對話的模型，而不是始終在 Haiku 上執行。在 Claude API 上，繼承的模型限制為 Opus：主要對話在更高層級上會在 Opus 上執行 Explore，而主要對話在 Sonnet 或 Haiku 上會在相同模型上執行 Explore。在任何其他提供者上，例如 [Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 AWS 上的 Claude Platform](/docs/zh-TW/third-party-integrations)，Explore 直接繼承主要對話的模型。

    一個名為 `Explore` 的[使用者或專案 subagent](#choose-the-subagent-scope) 會覆蓋內建的，並保留其自己的 `model` 欄位，因此定義一個具有 `model: haiku` 的以保持探索在較低成本的模型上。

    當 Claude 需要搜尋或理解程式碼庫而不進行更改時，它會委派給 Explore。這樣可以將探索結果保持在主要對話上下文之外。

    當呼叫 Explore 時，Claude 指定一個徹底程度：**quick** 用於目標查詢，**medium** 用於平衡探索，或 **very thorough** 用於全面分析。
  </Tab>

  <Tab title="Plan">
    一個研究代理，在 [plan mode](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode) 期間使用，以在呈現計畫之前收集上下文。

    * **Model**：從主要對話繼承
    * **Tools**：唯讀工具（拒絕存取 Write 和 Edit 工具）
    * **Purpose**：用於規劃的程式碼庫研究

    當您處於 plan mode 且 Claude 需要理解您的程式碼庫時，它會將研究委派給 Plan subagent，以便探索輸出保持在單獨的上下文視窗中，而主要對話保持唯讀。
  </Tab>

  <Tab title="General-purpose">
    一個能力強大的代理，用於需要探索和行動的複雜多步驟任務。

    * **Model**：從主要對話繼承
    * **Tools**：所有工具
    * **Purpose**：複雜研究、多步驟操作、程式碼修改

    當任務需要探索和修改、複雜推理來解釋結果或多個相依步驟時，Claude 會委派給 general-purpose。
  </Tab>

  <Tab title="Other">
    Claude Code 包括用於特定任務的其他輔助代理。這些通常會自動呼叫，因此您不需要直接使用它們。

    | Agent             | Model  | Claude 何時使用它                 |
    | :---------------- | :----- | :--------------------------- |
    | statusline-setup  | Sonnet | 當您執行 `/statusline` 來配置您的狀態行時 |
    | claude-code-guide | Haiku  | 當您提出有關 Claude Code 功能的問題時    |
  </Tab>
</Tabs>

內建 subagents 在互動式工作階段中預設會被註冊。若要限制它們：

* 若要封鎖特定的內建類型，請將其新增至 `permissions.deny`，如[停用特定 subagents](#disable-specific-subagents) 中所示。
* 若要防止 Claude 委派給任何 subagent，請使用 [`permissions.deny`](/docs/zh-TW/permissions#tool-specific-permission-rules) 拒絕 `Agent` 工具本身。
* 若要僅移除內建的 `Explore` 和 `Plan` subagents，請設定 [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/zh-TW/env-vars)。Claude 會直接讀取和探索檔案，而不是委派給它們。需要 Claude Code v2.1.198 或更新版本。
* 在[非互動式模式](/docs/zh-TW/headless)和 [Agent SDK](/docs/zh-TW/agent-sdk/overview) 中，設定 [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/zh-TW/env-vars) 以移除所有內建類型，並僅提供您自己的。

除了這些內建 subagents 之外，您可以建立自己的 subagents，具有自訂提示、工具限制、權限模式、hooks 和 skills。以下部分展示如何開始和自訂 subagents。

<h2 id="quickstart-create-your-first-subagent">
  快速入門：建立您的第一個 subagent
</h2>

Subagents 是具有 YAML frontmatter 的 Markdown 檔案。若要建立一個，請要求 Claude 為您撰寫，或 [自行撰寫檔案](#write-subagent-files)。

自 v2.1.198 起，`/agents` 命令不再開啟互動式建立精靈；執行它會列印提醒，要求您詢問 Claude 或直接編輯 `.claude/agents/`。Subagent 檔案、frontmatter 欄位以及 `.claude/agents/` 和 `~/.claude/agents/` 位置保持不變；只有終端精靈被移除。

本逐步指南建立一個使用者層級的 subagent，用於審查程式碼並提出改進建議。

<Steps>
  <Step title="要求 Claude 建立 subagent">
    在 Claude Code 中，描述您想要的 subagent 及其儲存位置：

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude 會撰寫包含 `name`、`description`、`tools` 清單、`model` 和系統提示的檔案。
  </Step>

  <Step title="檢查檔案">
    開啟 `~/.claude/agents/code-improver.md` 並確認 frontmatter 符合您的要求。結果如下所示：

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    因為檔案位於 `~/.claude/agents/`，所以 subagent 在您機器上的每個專案中都可用。若要改為將其範圍限制在一個專案，請將其移至該專案的 `.claude/agents/` 目錄。[選擇 subagent 範圍](#choose-the-subagent-scope) 比較了兩者。
  </Step>

  <Step title="試試看">
    要求 Claude 委派給新的 subagent：

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude 委派給您的新 subagent，它掃描程式碼庫並返回改進建議。

    如果 Claude 找不到新的 subagent，請重新啟動 Claude Code 並再試一次。這只在 `~/.claude/agents/` 在工作階段開始前不存在時發生，因為執行中的工作階段不會偵測到新建立的 `agents` 目錄。
  </Step>
</Steps>

現在您有一個 subagent，可以在機器上的任何專案中使用它來分析程式碼庫並提出改進建議。

您也可以手動撰寫 subagent 檔案、透過 CLI 旗標定義它們，或透過外掛程式分發它們。以下部分涵蓋所有配置選項。

<Note>
  在 Claude Code v2.1.197 及更早版本上，`/agents` 開啟一個互動式精靈，其中包含列出即時 subagents 的 **Running** 標籤和用於建立、編輯和刪除它們的 **Library** 標籤。
</Note>

<h2 id="configure-subagents">
  配置 subagents
</h2>

Subagent 的檔案位置決定了誰可以使用它，其 frontmatter 決定了它可以做什麼。本節涵蓋 subagent 檔案的位置以及它們支援的每個欄位。

<h3 id="choose-the-subagent-scope">
  選擇 subagent 範圍
</h3>

根據範圍將 subagent 檔案儲存在不同位置。當多個 subagents 共享相同名稱時，Claude Code 使用來自優先級較高位置的那個。

| Location              | Scope     | Priority | 如何建立                                      |
| :-------------------- | :-------- | :------- | :---------------------------------------- |
| 受管設定                  | 組織範圍      | 1（最高）    | 透過 [managed settings](/docs/zh-TW/settings) 部署 |
| `--agents` CLI 標誌     | 目前工作階段    | 2        | 啟動 Claude Code 時傳遞 JSON                   |
| `.claude/agents/`     | 目前專案      | 3        | 詢問 Claude，或手動建立檔案                         |
| `~/.claude/agents/`   | 所有您的專案    | 4        | 詢問 Claude，或手動建立檔案                         |
| Plugin 的 `agents/` 目錄 | 啟用外掛程式的位置 | 5（最低）    | 使用 [plugins](/docs/zh-TW/plugins) 安裝           |

**專案 subagents**（`.claude/agents/`）非常適合特定於程式碼庫的 subagents。將它們簽入版本控制，以便您的團隊可以協作使用和改進它們。

專案 subagents 是透過從目前工作目錄向上走來發現的，因此會掃描那裡和儲存庫根目錄之間的每個 `.claude/agents/`。自 v2.1.178 起，當這些巢狀目錄中的多個定義相同的 `name` 時，Claude Code 使用最接近工作目錄的定義。

使用 `--add-dir` 新增的目錄也會被掃描：新增目錄內的 `.claude/agents/` 資料夾與專案 subagents 一起載入。請參閱 [Additional directories](/docs/zh-TW/permissions#additional-directories-grant-file-access-not-configuration) 以了解哪些其他配置類型從 `--add-dir` 載入。若要跨專案共享 subagents 而不使用 `--add-dir`，請使用 `~/.claude/agents/` 或 [plugin](/docs/zh-TW/plugins)。

**使用者 subagents**（`~/.claude/agents/`）是在所有專案中可用的個人 subagents。

Claude Code 會遞迴掃描 `.claude/agents/` 和 `~/.claude/agents/`，因此您可以將定義組織到子資料夾中，例如 `agents/review/` 或 `agents/research/`。子目錄路徑不會影響 subagent 的識別或呼叫方式，因為身份僅來自 `name` frontmatter 欄位。

在整個樹中保持 `name` 值唯一：如果一個 `.claude/agents/` 目錄下的兩個檔案（包括其子資料夾）宣告相同的名稱，Claude Code 只會載入其中一個，由檔案系統讀取順序選擇，而不是有文件記載的優先級。在巢狀專案目錄中，最接近工作目錄的定義獲勝，如上所述。[`/doctor`](/docs/zh-TW/commands#all-commands) 設定檢查會報告同一目錄中共享名稱的檔案，並建議重新命名或移除除一個以外的所有檔案。在 v2.1.205 之前，`/doctor` 開啟診斷畫面，列出重複項並顯示哪個定義處於活動狀態。

外掛程式 `agents/` 目錄也會遞迴掃描。與專案和使用者範圍不同，外掛程式 `agents/` 目錄內的子資料夾成為 [scoped identifier](#invoke-subagents-explicitly) 的一部分：外掛程式 `my-plugin` 中位於 `agents/review/security.md` 的檔案註冊為 `my-plugin:review:security`。

**CLI 定義的 subagents** 在啟動 Claude Code 時作為 JSON 傳遞。它們僅存在於該工作階段，不會儲存到磁碟，使其適用於快速測試或自動化指令碼。您可以在單一 `--agents` 呼叫中定義多個 subagents：

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

`--agents` 標誌接受 JSON，具有與基於檔案的 subagents 相同的 [frontmatter](#supported-frontmatter-fields) 欄位：`description`、`prompt`、`tools`、`disallowedTools`、`model`、`permissionMode`、`mcpServers`、`hooks`、`maxTurns`、`skills`、`initialPrompt`、`memory`、`effort`、`background`、`isolation` 和 `color`。使用 `prompt` 作為系統提示，等同於基於檔案的 subagents 中的 markdown 主體。

**受管 subagents** 由組織管理員部署。將 markdown 檔案放在 [managed settings directory](/docs/zh-TW/settings#settings-files) 內的 `.claude/agents/` 中，使用與專案和使用者 subagents 相同的 frontmatter 格式。受管定義優先於具有相同名稱的專案和使用者 subagents。

**外掛程式 subagents** 來自您已安裝的 [plugins](/docs/zh-TW/plugins)。它們與您的自訂 subagents 一起載入，並在 @-mention 類型提前中以其範圍名稱出現。請參閱 [plugin components reference](/docs/zh-TW/plugins-reference#agents) 以了解建立外掛程式 subagents 的詳細資訊。

<Note>
  基於安全考慮，外掛程式 subagents 不支援 `hooks`、`mcpServers` 或 `permissionMode` frontmatter 欄位。從外掛程式載入代理時，這些欄位會被忽略。如果您需要它們，請將代理檔案複製到 `.claude/agents/` 或 `~/.claude/agents/`。您也可以在 `settings.json` 或 `settings.local.json` 中的 [`permissions.allow`](/docs/zh-TW/settings#permission-settings) 新增規則，但這些規則適用於整個工作階段，而不僅僅是外掛程式 subagent。
</Note>

來自任何這些範圍的 subagent 定義也可用於 [agent teams](/docs/zh-TW/agent-teams#use-subagent-definitions-for-teammates)：當產生隊友時，您可以參考 subagent 類型，隊友會使用其 `tools` 和 `model`，定義的主體作為額外指令附加到隊友的系統提示。請參閱 [agent teams](/docs/zh-TW/agent-teams#use-subagent-definitions-for-teammates) 以了解哪些 frontmatter 欄位適用於該路徑。

<h3 id="write-subagent-files">
  編寫 subagent 檔案
</h3>

Subagent 檔案使用 YAML frontmatter 進行配置，後面跟著 Markdown 中的系統提示：

<Note>
  Claude Code 監視 `~/.claude/agents/` 和 `.claude/agents/`。當您在磁碟上新增或編輯 subagent 檔案，或要求 Claude 為您編寫一個時，Claude Code 會在幾秒內偵測到變更，下次委派會使用更新的定義，無需重新啟動。

  仍有兩種情況需要重新啟動：

  * 監視程式僅涵蓋工作階段開始時存在的目錄，因此在新的 `agents` 目錄中建立範圍的第一個代理檔案後，重新啟動以載入它。
  * 使用 `--disable-slash-commands` 啟動的工作階段根本不監視這些目錄。
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Frontmatter 定義 subagent 的中繼資料和配置。主體成為指導 subagent 行為的系統提示。Subagents 只接收此系統提示（加上基本環境詳細資訊，如工作目錄），而不是完整的 Claude Code 系統提示。

在 [non-interactive mode](/docs/zh-TW/headless) 中，[`--append-subagent-system-prompt`](/docs/zh-TW/cli-reference#cli-flags) 標誌將您提供的文字附加到每個 subagent 的系統提示末尾，包括巢狀 subagents。需要 Claude Code v2.1.205 或更高版本。

一個 subagent 在主要對話的目前工作目錄中啟動。在 subagent 內，`cd` 命令不會在 Bash 或 PowerShell 工具呼叫之間持續，也不會影響主要對話的工作目錄。若要改為給 subagent 儲存庫的隔離副本，請設定 [`isolation: worktree`](#supported-frontmatter-fields)。

具有 `isolation: worktree` 的 subagent 在其 worktree 內執行其 Bash 和 PowerShell 命令。一個工作目錄解析到您的主要簽出的命令（例如，因為 subagent 執行時 worktree 目錄被移除）會失敗並出現錯誤。在 v2.1.203 之前，此類命令可能在主要簽出中執行。

<h4 id="supported-frontmatter-fields">
  支援的 frontmatter 欄位
</h4>

以下欄位可用於 YAML frontmatter。只有 `name` 和 `description` 是必需的。

| Field             | 必需 | Description                                                                                                                                                                                                                         |
| :---------------- | :- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | 是  | 使用小寫字母和連字號的唯一識別碼。[Hooks](/docs/zh-TW/hooks#subagentstart) 將此值作為 `agent_type` 接收。檔案名稱不必相符                                                                                                                                                 |
| `description`     | 是  | Claude 何時應委派給此 subagent                                                                                                                                                                                                             |
| `tools`           | 否  | [Tools](#available-tools) subagent 可以使用。如果省略，繼承所有工具。若要將 Skills 預載入上下文，請使用 `skills` 欄位而不是在此列出 `Skill`                                                                                                                                |
| `disallowedTools` | 否  | 要拒絕的工具，從繼承或指定的清單中移除                                                                                                                                                                                                                 |
| `model`           | 否  | [Model](#choose-a-model) 使用：`sonnet`、`opus`、`haiku`、`fable`、完整模型 ID（例如，`claude-opus-4-8`）或 `inherit`。預設為 `inherit`                                                                                                                  |
| `permissionMode`  | 否  | [Permission mode](#permission-modes)：`default`、`acceptEdits`、`auto`、`dontAsk`、`bypassPermissions`、`plan` 或 `manual` 作為 `default` 的別名。`manual` 別名需要 Claude Code v2.1.200 或更高版本。針對 [plugin subagents](#choose-the-subagent-scope) 被忽略 |
| `maxTurns`        | 否  | subagent 停止前的最大代理轉數                                                                                                                                                                                                                 |
| `skills`          | 否  | [Skills](/docs/zh-TW/skills) 在啟動時預載入到 subagent 的上下文中。注入完整技能內容，而不僅僅是描述。Subagents 仍然可以透過 Skill 工具呼叫未列出的專案、使用者和外掛程式技能                                                                                                                       |
| `mcpServers`      | 否  | [MCP servers](/docs/zh-TW/mcp) 可用於此 subagent。每個條目要麼是參考已配置伺服器的伺服器名稱（例如，`"slack"`），要麼是內聯定義，其中伺服器名稱為鍵，完整 [MCP server config](/docs/zh-TW/mcp#installing-mcp-servers) 為值。針對 [plugin subagents](#choose-the-subagent-scope) 被忽略                    |
| `hooks`           | 否  | [Lifecycle hooks](#define-hooks-for-subagents) 限定於此 subagent。針對 [plugin subagents](#choose-the-subagent-scope) 被忽略                                                                                                                  |
| `memory`          | 否  | [Persistent memory scope](#enable-persistent-memory)：`user`、`project` 或 `local`。啟用跨工作階段學習                                                                                                                                           |
| `background`      | 否  | 設定為 `true` 以始終將此 subagent 作為 [background task](#run-subagents-in-foreground-or-background) 執行，即使 Claude 需要其結果。未設定時，Claude 選擇，自 v2.1.198 起，它預設在背景執行 subagents                                                                        |
| `effort`          | 否  | 此 subagent 活動時的努力程度。覆蓋工作階段努力程度。預設：從工作階段繼承。選項：`low`、`medium`、`high`、`xhigh`、`max`；可用的層級取決於模型                                                                                                                                         |
| `isolation`       | 否  | 設定為 `worktree` 以在臨時 [git worktree](/docs/zh-TW/worktrees) 中執行 subagent，為其提供儲存庫的隔離副本，預設從您的 [default branch](/docs/zh-TW/worktrees#choose-the-base-branch) 分支，而不是父工作階段的 `HEAD`。如果 subagent 不進行任何更改，worktree 會自動清理                               |
| `color`           | 否  | Subagent 在任務清單和文字中的顯示顏色。接受 `red`、`blue`、`green`、`yellow`、`purple`、`orange`、`pink` 或 `cyan`                                                                                                                                          |
| `initialPrompt`   | 否  | 當此代理作為主工作階段代理執行時（透過 `--agent` 或 `agent` 設定），自動提交為第一個使用者轉數。[Commands](/docs/zh-TW/commands) 和 [skills](/docs/zh-TW/skills) 會被處理。前置於任何使用者提供的提示                                                                                                  |

<h3 id="choose-a-model">
  選擇模型
</h3>

`model` 欄位控制 subagent 使用的 [AI model](/docs/zh-TW/model-config)：

* **Model alias**：使用可用的別名之一：`sonnet`、`opus`、`haiku` 或 `fable`
* **Full model ID**：使用完整模型 ID，例如 `claude-opus-4-8` 或 `claude-sonnet-5`。接受與 `--model` 標誌相同的值
* **inherit**：使用與主要對話相同的模型
* **Omitted**：如果未指定，預設為 `inherit`（使用與主要對話相同的模型）

當 Claude 呼叫 subagent 時，它也可以為該特定呼叫傳遞 `model` 參數。Claude Code 按此順序解析 subagent 的模型：

1. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/zh-TW/model-config#environment-variables) 環境變數（如果設定為模型別名或模型 ID）
2. 每次呼叫的 `model` 參數
3. Subagent 定義的 `model` frontmatter
4. 主要對話的模型

自 v2.1.196 起，將 `CLAUDE_CODE_SUBAGENT_MODEL` 設定為 `inherit` 與不設定相同：解析繼續進行每次呼叫的 `model` 參數，然後是 frontmatter。在較早的版本中，`inherit` 強制 subagents 使用主要對話的模型，並忽略這兩個來源。

環境變數、每次呼叫的參數和 frontmatter 值會根據您組織的 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許清單進行檢查。解析為排除模型的值不會被使用，subagent 會改為在繼承的模型上執行。

自 v2.1.198 起，subagents 也繼承主要對話的 [extended thinking](/docs/zh-TW/model-config#extended-thinking) 配置：如果思考在您的工作階段中開啟，它對 subagent 也開啟，如果關閉，它保持關閉。沒有每個 subagent 的思考設定。在 v2.1.198 之前，subagents 執行時禁用擴展思考，無論主要對話的設定如何。

<h3 id="control-subagent-capabilities">
  控制 subagent 功能
</h3>

您可以透過工具存取、權限模式和條件規則來控制 subagents 可以執行的操作。

<h4 id="available-tools">
  可用工具
</h4>

Subagents 預設從主要對話繼承 [internal tools](/docs/zh-TW/tools-reference) 和 MCP 工具。以下工具取決於主要對話的 UI 或工作階段狀態，即使在 `tools` 欄位中列出也不可用於 subagents：

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`，除非 subagent 的 [`permissionMode`](#permission-modes) 是 `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

若要限制工具，請使用 `tools` 欄位（允許清單）或 `disallowedTools` 欄位（拒絕清單）。此範例使用 `tools` 來專門允許 Read、Grep、Glob 和 Bash。Subagent 無法編輯檔案、寫入檔案或使用任何 MCP 工具：

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

此範例使用 `disallowedTools` 來繼承主要對話中的每個工具，除了 Write 和 Edit。Subagent 保留 Bash、MCP 工具和其他所有內容：

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

如果兩者都設定，`disallowedTools` 首先應用，然後 `tools` 針對剩餘的池進行解析。同時列在兩者中的工具會被移除。

當 `tools` 清單中沒有任何內容解析為工具時（例如，因為每個條目都拼寫錯誤或命名一個對 subagents 不可用的工具），Claude Code 拒絕啟動 subagent，Agent 工具會傳回一個錯誤，命名未解析的條目。在 v2.1.208 之前，該 subagent 啟動時沒有工具，可能會傳回空的或令人困惑的結果。

兩個欄位都接受 MCP 伺服器層級的模式，除了確切的工具名稱：`mcp__<server>` 或 `mcp__<server>__*` 授予或移除來自命名伺服器的每個工具。在 `disallowedTools` 中，`mcp__*` 也會移除來自任何伺服器的每個 MCP 工具。此範例移除來自 `github` MCP 伺服器的每個工具，同時保留來自其他伺服器的工具和每個內建工具：

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

<h4 id="restrict-which-subagents-can-be-spawned">
  限制可以產生的 subagents
</h4>

當代理以 `claude --agent` 作為主執行緒執行時，它可以使用 Agent 工具產生 subagents。若要限制它可以產生的 subagent 類型，請在 `tools` 欄位中使用 `Agent(agent_type)` 語法。

<Note>在版本 2.1.63 中，Task 工具已重新命名為 Agent。設定和代理定義中的現有 `Task(...)` 參考仍然作為別名工作。</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

這是一個允許清單：只有 `worker` 和 `researcher` subagents 可以產生。如果代理嘗試產生任何其他類型，請求失敗，代理在其提示中只看到允許的類型。若要在允許所有其他類型的同時阻止特定代理，請改用 [`permissions.deny`](#disable-specific-subagents)。

若要允許產生任何 subagent 而不受限制，請使用不帶括號的 `Agent`：

```yaml theme={null}
tools: Agent, Read, Bash
```

如果 `Agent` 完全從 `tools` 清單中省略，代理無法產生任何 subagents。

`Agent(agent_type)` 允許清單語法僅適用於以 `claude --agent` 作為主執行緒執行的代理。在 subagent 定義中，在 `tools` 中列出 `Agent` 讓該 subagent [產生巢狀 subagents](#spawn-nested-subagents)，但括號內的任何類型清單都會被忽略。

<h4 id="scope-mcp-servers-to-a-subagent">
  將 MCP 伺服器限定於 subagent
</h4>

使用 `mcpServers` 欄位為 subagent 提供對主要對話中不可用的 [MCP](/docs/zh-TW/mcp) 伺服器的存取。此處定義的內聯伺服器在 subagent 啟動時連接，在完成時斷開連接。字串參考共享父工作階段的連接。

<Note>
  `mcpServers` 欄位適用於代理檔案可以執行的兩個上下文：

  * 作為 subagent，透過 Agent 工具或 @-mention 產生
  * 作為主工作階段，使用 [`--agent`](#invoke-subagents-explicitly) 或 `agent` 設定啟動

  當代理是主工作階段時，內聯伺服器定義在啟動時與來自 [`.mcp.json`](/docs/zh-TW/mcp) 和設定檔案的伺服器一起連接。
</Note>

清單中的每個條目要麼是內聯伺服器定義，要麼是參考工作階段中已配置的 MCP 伺服器的字串：

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

內聯定義使用與 `.mcp.json` 伺服器條目相同的架構，由伺服器名稱鍵入，並支援 `stdio`、`http`、`sse` 和 `ws` 類型。

若要將 MCP 伺服器保持在主要對話之外，並避免其工具描述在那裡消耗上下文，請在此處內聯定義它，而不是在 `.mcp.json` 中。Subagent 獲得工具；父對話不獲得。

自 v2.1.153 起，適用於主工作階段的 MCP 限制也涵蓋在 subagent frontmatter 中宣告的伺服器：

* [`--strict-mcp-config`](/docs/zh-TW/cli-reference) 和 [`--bare`](/docs/zh-TW/cli-reference)
* [Enterprise managed MCP configuration](/docs/zh-TW/managed-mcp)
* [`allowedMcpServers` 和 `deniedMcpServers` 政策](/docs/zh-TW/managed-mcp#policy-based-control-with-allowlists-and-denylists)

當其中之一阻止伺服器時，Claude Code 會跳過它並顯示警告，命名被阻止的伺服器。

受管設定限制適用於每個 subagent，無論其如何定義。`--strict-mcp-config` 不會過濾您透過 `--agents` 或 SDK `agents` 選項內聯傳遞的伺服器，因為這些是明確的呼叫者輸入。

<h4 id="permission-modes">
  權限模式
</h4>

`permissionMode` 欄位控制 subagent 如何處理權限提示。Subagents 從主要對話繼承權限上下文，並可以覆蓋模式，除非父模式優先，如下所述。

| Mode                | Behavior                                                                                                                                                                                                                  |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | 標準權限檢查，帶有提示                                                                                                                                                                                                               |
| `acceptEdits`       | 自動接受檔案編輯和工作目錄或 `additionalDirectories` 中路徑的常見檔案系統命令                                                                                                                                                                       |
| `auto`              | [Auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)：背景分類器審查命令和受保護目錄寫入                                                                                                                                   |
| `dontAsk`           | 自動拒絕權限提示。明確允許的工具仍然工作；`AskUserQuestion`、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具即使您已允許它們也會被拒絕 |
| `bypassPermissions` | 跳過權限提示                                                                                                                                                                                                                    |
| `plan`              | Plan mode（唯讀探索）                                                                                                                                                                                                           |

<Warning>
  謹慎使用 `bypassPermissions`。它跳過權限提示，允許 subagent 執行操作而無需批准，包括寫入 `.git`、`.config/git`、`.claude`、`.vscode`、`.idea`、`.husky`、`.cargo`、`.devcontainer`、`.yarn` 和 `.mvn`。

  明確的 [`ask` 規則](/docs/zh-TW/permissions#manage-permissions)、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)、標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具，以及根和主目錄移除（例如 `rm -rf /`）仍然會提示。請參閱 [permission modes](/docs/zh-TW/permission-modes#skip-all-checks-with-bypasspermissions-mode) 以了解詳細資訊。
</Warning>

如果父級使用 `bypassPermissions` 或 `acceptEdits`，這優先並且無法被覆蓋。如果父級使用 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)，subagent 繼承 auto mode，其 frontmatter 中的任何 `permissionMode` 都會被忽略：分類器使用與父工作階段相同的阻止和允許規則評估 subagent 的工具呼叫。

<h4 id="preload-skills-into-subagents">
  將技能預載入 subagents
</h4>

使用 `skills` 欄位在啟動時將技能內容注入到 subagent 的上下文中。這為 subagent 提供領域知識，而無需在執行期間發現和載入技能。

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

每個列出的技能的完整內容被注入到 subagent 的上下文中。此欄位控制哪些技能被預載入，而不是 subagent 可以存取哪些技能：沒有它，subagent 仍然可以在執行期間透過 Skill 工具發現和呼叫專案、使用者和外掛程式技能。若要防止 subagent 完全呼叫技能，請從 [`tools`](#available-tools) 清單中省略 `Skill` 或將其新增到 `disallowedTools`。

您無法預載入設定 [`disable-model-invocation: true`](/docs/zh-TW/skills#control-who-invokes-a-skill) 的技能，因為預載入來自 Claude 可以呼叫的相同技能集。如果列出的技能遺失或已停用，Claude Code 會跳過它並將警告記錄到除錯日誌。

<Note>
  這與 [在 subagent 中執行技能](/docs/zh-TW/skills#run-skills-in-a-subagent) 相反。使用 subagent 中的 `skills`，subagent 控制系統提示並載入技能內容。使用技能中的 `context: fork`，技能內容被注入到您指定的代理中。兩者都使用相同的基礎系統。
</Note>

<h4 id="enable-persistent-memory">
  啟用持久記憶
</h4>

`memory` 欄位為 subagent 提供一個在對話之間存活的持久目錄。Subagent 使用此目錄隨著時間建立知識，例如程式碼庫模式、除錯見解和架構決策。

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

根據記憶應該應用的廣泛程度選擇範圍：

| Scope     | Location                                      | 使用時機                          |
| :-------- | :-------------------------------------------- | :---------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | subagent 應該記住跨所有專案的學習         |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | subagent 的知識是特定於專案的，可透過版本控制共享 |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | subagent 的知識是特定於專案的，但不應簽入版本控制 |

啟用記憶時：

* Subagent 的系統提示包括讀取和寫入記憶目錄的說明。
* Subagent 的系統提示還包括記憶目錄中 `MEMORY.md` 的前 200 行或 25KB（以先到者為準），以及如果超過該限制則策劃 `MEMORY.md` 的說明。
* Read、Write 和 Edit 工具會自動啟用，以便 subagent 可以管理其記憶檔案。

<h5 id="persistent-memory-tips">
  持久記憶提示
</h5>

* `project` 是建議的預設範圍。它使 subagent 知識可透過版本控制共享。
* 要求 subagent 在開始工作前查閱其記憶："Review this PR, and check your memory for patterns you've seen before."
* 要求 subagent 在完成任務後更新其記憶："Now that you're done, save what you learned to your memory." 隨著時間的推移，這會建立一個知識庫，使 subagent 更有效。
* 直接在 subagent 的 markdown 檔案中包括記憶說明，以便它主動維護自己的知識庫：

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  使用 hooks 的條件規則
</h4>

為了更動態地控制工具使用，請使用 `PreToolUse` hooks 在執行前驗證操作。當您需要允許工具的某些操作同時阻止其他操作時，這很有用。

此範例建立一個只允許唯讀資料庫查詢的 subagent。`PreToolUse` hook 在每個 Bash 命令執行前執行 `command` 中指定的指令碼：

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [透過 stdin 將 hook 輸入作為 JSON 傳遞](/docs/zh-TW/hooks#pretooluse-input) 給 hook 命令。驗證指令碼讀取此 JSON，提取 Bash 命令，並 [以代碼 2 退出](/docs/zh-TW/hooks#exit-code-2-behavior-per-event) 以阻止寫入操作：

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

請參閱 [Hook input](/docs/zh-TW/hooks#pretooluse-input) 以了解完整的輸入架構，以及 [exit codes](/docs/zh-TW/hooks#exit-code-output) 以了解退出代碼如何影響行為。在 Windows 上，在 PowerShell 中編寫 hook 指令碼，並在 hook 條目中新增 `shell: powershell`，如 [在 PowerShell 中執行 hooks](/docs/zh-TW/hooks#windows-powershell-tool) 所示。

<h4 id="disable-specific-subagents">
  禁用特定 subagents
</h4>

您可以透過將 subagents 新增到 [settings](/docs/zh-TW/settings#permission-settings) 中的 `deny` 陣列來防止 Claude 使用特定 subagents。使用格式 `Agent(subagent-name)`，其中 `subagent-name` 與 subagent 的 name 欄位相符。

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

這適用於內建和自訂 subagents。您也可以使用 `--disallowedTools` CLI 標誌：

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

請參閱 [Permissions documentation](/docs/zh-TW/permissions#tool-specific-permission-rules) 以了解有關權限規則的更多詳細資訊。

<h3 id="define-hooks-for-subagents">
  為 subagents 定義 hooks
</h3>

Subagents 可以定義在 subagent 生命週期期間執行的 [hooks](/docs/zh-TW/hooks)。有兩種方式來配置 hooks：

* **在 subagent 的 frontmatter 中**：定義只在該 subagent 活動時執行的 hooks
* **在 `settings.json` 中**：定義在 subagents 啟動或停止時在主工作階段中執行的 hooks

<h4 id="hooks-in-subagent-frontmatter">
  Subagent frontmatter 中的 Hooks
</h4>

直接在 subagent 的 markdown 檔案中定義 hooks。這些 hooks 只在該特定 subagent 活動時執行，並在完成時清理。

<Note>
  Frontmatter hooks 在代理透過 Agent 工具或 @-mention 作為 subagent 產生時觸發，以及當代理透過 [`--agent`](#invoke-subagents-explicitly) 或 `agent` 設定作為主工作階段執行時觸發。在主工作階段情況下，它們與在 [`settings.json`](/docs/zh-TW/hooks) 中定義的任何 hooks 一起執行。
</Note>

支援所有 [hook events](/docs/zh-TW/hooks#hook-events)。subagents 最常見的事件是：

| Event         | Matcher input | 何時觸發                                   |
| :------------ | :------------ | :------------------------------------- |
| `PreToolUse`  | Tool name     | 在 subagent 使用工具之前                      |
| `PostToolUse` | Tool name     | 在 subagent 使用工具之後                      |
| `Stop`        | (none)        | 當 subagent 完成時（在執行時轉換為 `SubagentStop`） |

此範例使用 `PreToolUse` hook 驗證 Bash 命令，並在檔案編輯後使用 `PostToolUse` 執行 linter：

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Frontmatter 中的 `Stop` hooks 會自動轉換為 `SubagentStop` 事件。

<h4 id="project-level-hooks-for-subagent-events">
  用於 subagent 事件的專案層級 hooks
</h4>

在 `settings.json` 中配置 hooks，以回應主工作階段中的 subagent 生命週期事件。

| Event           | Matcher input   | 何時觸發             |
| :-------------- | :-------------- | :--------------- |
| `SubagentStart` | Agent type name | 當 subagent 開始執行時 |
| `SubagentStop`  | Agent type name | 當 subagent 完成時   |

兩個事件都支援匹配器以按名稱針對特定代理類型。匹配器值是專案層級和使用者層級 subagents 的代理 frontmatter `name`，或 [plugin subagents](/docs/zh-TW/plugins) 的外掛程式範圍識別碼，例如 `my-plugin:db-agent`。範圍名稱包含冒號，因此它被評估為 [unanchored regular expression](/docs/zh-TW/hooks#matcher-patterns)；使用 `^` 和 `$` 錨定它，如 `^my-plugin:db-agent$`，以僅匹配該代理。

此範例僅在 `db-agent` subagent 啟動時執行設定指令碼，並在任何 subagent 停止時執行清理指令碼：

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

連字號匹配器（如 `db-agent`）在 Claude Code v2.1.195 或更高版本上精確匹配。在較早的版本上，它被評估為 unanchored regular expression，也會針對任何包含它的代理類型觸發，例如 `prod-db-agent`；在這些版本上使用 `^db-agent$` 錨定它。

請參閱 [Hooks](/docs/zh-TW/hooks) 以了解完整的 hook 配置格式。

<h2 id="work-with-subagents">
  使用 subagents
</h2>

<h3 id="understand-automatic-delegation">
  理解自動委派
</h3>

Claude 根據您請求中的任務描述、subagent 配置中的 `description` 欄位和目前上下文自動委派任務。為了鼓勵主動委派，在 subagent 的 description 欄位中包括「use proactively」之類的短語。

<h3 id="invoke-subagents-explicitly">
  明確呼叫 subagents
</h3>

當自動委派不夠時，您可以自己要求 subagent。三種模式從一次性建議升級到工作階段範圍的預設：

* **自然語言**：在提示中命名 subagent；Claude 決定是否委派
* **@-mention**：保證 subagent 為一個任務執行
* **工作階段範圍**：整個工作階段使用該 subagent 的系統提示、工具限制和模型，透過 `--agent` 標誌或 `agent` 設定

對於自然語言，沒有特殊語法。命名 subagent，Claude 通常會委派：

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mention subagent。** 輸入 `@` 並從預輸入中選擇 subagent，就像您 @-mention 檔案一樣。這確保該特定 subagent 執行，而不是將選擇留給 Claude：

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

您的完整訊息仍然會傳送給 Claude，它根據您要求的內容為 subagent 編寫任務提示。@-mention 控制 Claude 呼叫哪個 subagent，而不是它接收什麼提示。

由啟用的 [plugin](/docs/zh-TW/plugins) 提供的 Subagents 在預輸入中顯示為其限定名稱，例如 `my-plugin:code-reviewer` 或 `my-plugin:review:security`（當 plugin [將 agents 組織到子資料夾](#choose-the-subagent-scope) 時）。名為背景 subagents 目前在工作階段中執行也出現在預輸入中，在名稱旁邊顯示其狀態。

您也可以手動輸入提及而不使用選擇器：`@agent-<name>` 用於本地 subagents，或 `@agent-` 後跟外掛程式 subagents 的限定名稱，例如 `@agent-my-plugin:code-reviewer`。

**將整個工作階段作為 subagent 執行。** 傳遞 [`--agent <name>`](/docs/zh-TW/cli-reference) 以啟動一個工作階段，其中主執行緒本身採用該 subagent 的系統提示、工具限制和模型：

```bash theme={null}
claude --agent code-reviewer
```

Subagent 的系統提示完全替換預設 Claude Code 系統提示，就像 [`--system-prompt`](/docs/zh-TW/cli-reference) 一樣。`CLAUDE.md` 檔案和專案記憶仍然透過正常訊息流載入。代理名稱在啟動標題中顯示為 `@<name>`，以便您可以確認它是活動的。

這適用於內建和自訂 subagents，選擇在您恢復工作階段時持續。

對於外掛程式提供的 subagent，您可以只傳遞代理名稱，Claude Code 會找到它：

```bash theme={null}
claude --agent security-reviewer
```

如果多個外掛程式提供具有相同名稱的代理，傳遞限定名稱以消除歧義：

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

如果外掛程式將代理放在其 `agents/` 目錄的子資料夾中，請在限定名稱中包括子資料夾，例如 `claude --agent my-plugin:review:security`。

若要使其成為專案中每個工作階段的預設值，請在 `.claude/settings.json` 中設定 `agent`：

```json theme={null}
{
  "agent": "code-reviewer"
}
```

如果兩者都存在，CLI 標誌會覆蓋設定。

<h3 id="run-subagents-in-foreground-or-background">
  在前景或背景中執行 subagents
</h3>

Subagents 可以在前景或背景中執行：

* **前景 subagents** 阻止主要對話直到完成。權限提示會在出現時傳遞給您。
* **背景 subagents** 在您繼續工作時並行執行。自 v2.1.186 起，當背景 subagent 到達需要權限的工具呼叫時，提示會在您的主要工作階段中出現，並命名要求的 subagent。批准以讓 subagent 繼續，或按 Esc 拒絕該單一工具呼叫而不停止 subagent。在 v2.1.186 之前，背景 subagents 自動拒絕任何會提示的工具呼叫。

自 v2.1.198 起，subagents 預設在背景中執行。Claude 在需要結果才能繼續時在前景中執行 subagent。預設值改變 subagent 執行的位置，而不是它被允許做什麼：背景 subagents 仍然在您的主要工作階段中出現每個權限提示。在 v2.1.198 之前，Claude 根據任務在前景和背景之間選擇。

您也可以自己引導這個：

* 要求 Claude 在背景或前景中執行任務
* 按 **Ctrl+B** 將執行中的任務放在背景中

完成的背景 subagent 保持列在 [`/tasks`](/docs/zh-TW/commands) 中，標記為完成並排序在執行中的工作下方，直到工作階段清理其任務列表。當 subagent 完成時，其詳細檢視保持開啟。失敗或您停止的 Subagents 會離開列表。在 v2.1.208 之前，完成的 subagent 在完成時立即離開列表，其詳細檢視關閉。

若要禁用所有背景任務功能，請將 `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` 環境變數設定為 `1`。請參閱 [Environment variables](/docs/zh-TW/env-vars)。

當 [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) 設定為 `1` 時，每個 subagent 產生都在背景中執行，frontmatter `background` 欄位沒有效果，因為 fork 模式從 `Agent` 工具中移除 `run_in_background` 參數。`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` 優先於 fork 模式，並將 subagent 產生保持在前景中。

<h3 id="api-errors-in-subagents">
  Subagents 中的 API 錯誤
</h3>

自 v2.1.199 起，subagent 的執行因 API 錯誤（例如使用限制或重複的伺服器錯誤）而結束時，會將該失敗報告回 Claude，而不是將錯誤文字作為 subagent 的發現返回。Claude 接收的內容取決於 subagent 執行的位置：

* **前景**：如果速率限制、過載或伺服器錯誤切斷已經產生輸出的 subagent，Agent 工具會返回該部分輸出，並附註 subagent 被切斷且未完成其任務。未產生任何內容或其唯一輸出為工具呼叫的 subagent 會失敗，並顯示 [`Agent terminated early due to an API error`](/docs/zh-TW/errors#agent-terminated-early-due-to-an-api-error)，後跟錯誤詳細資訊。在 v2.1.199 中，切斷工具呼叫專用形狀的速率限制、過載或伺服器錯誤返回了只包含切斷注記的空部分結果。
* **背景**：subagent 被標記為失敗，Claude 在其結束時接收的訊息命名 API 錯誤並包括 subagent 的最後輸出，所以部分工作不會丟失。

一旦基礎 API 錯誤清除，要求 Claude 重試任務或 [恢復 subagent](#resume-subagents)。

<h3 id="common-patterns">
  常見模式
</h3>

<h4 id="isolate-high-volume-operations">
  隔離高容量操作
</h4>

subagents 最有效的用途之一是隔離產生大量輸出的操作。執行測試、獲取文件或處理日誌檔案可能會消耗大量上下文。透過將這些委派給 subagent，詳細輸出保留在 subagent 的上下文中，而只有相關摘要返回到主要對話。

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  執行並行研究
</h4>

對於獨立調查，產生多個 subagents 以同時工作：

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

每個 subagent 獨立探索其領域，然後 Claude 綜合發現。當研究路徑彼此不相依時，這效果最好。

<Warning>
  當 subagents 完成時，其結果返回到主要對話。執行許多 subagents，每個都返回詳細結果，可能會消耗大量上下文。
</Warning>

對於需要持續並行性或超過上下文視窗的任務，[agent teams](/docs/zh-TW/agent-teams) 為每個工作者提供自己的獨立上下文。

<h4 id="chain-subagents">
  鏈接 subagents
</h4>

對於多步驟工作流程，要求 Claude 按順序使用 subagents。每個 subagent 完成其任務並將結果返回給 Claude，然後將相關上下文傳遞給下一個 subagent。

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  在 subagents 和主要對話之間選擇
</h3>

在以下情況下使用 **主要對話**：

* 任務需要頻繁的來回或反覆改進
* 多個階段共享重要上下文，例如規劃、實現和測試
* 您正在進行快速、有針對性的更改
* 延遲很重要。Subagents 從頭開始，可能需要時間收集上下文

在以下情況下使用 **subagents**：

* 任務產生您不需要在主要上下文中的詳細輸出
* 您想強制執行特定的工具限制或權限
* 工作是自包含的，可以返回摘要

當您想要可重複使用的提示或在主要對話上下文中執行的工作流程而不是隔離的 subagent 上下文時，請改為考慮 [Skills](/docs/zh-TW/skills)。

對於關於對話中已有內容的快速問題，請使用 [`/btw`](/docs/zh-TW/interactive-mode#side-questions-with-%2Fbtw) 而不是 subagent。它看到您的完整上下文，但沒有工具存取，答案被丟棄而不是新增到歷史記錄。

<h3 id="spawn-nested-subagents">
  產生嵌套 subagents
</h3>

自 Claude Code v2.1.172 起，subagent 可以產生自己的 subagents。當委派的任務本身分裂成並行子任務時使用此功能，例如審查者 subagent 為每個發現分派驗證者，所以中間輸出永遠不會到達您的主要對話。只有頂級 subagent 的摘要返回給您。

嵌套 subagent 的配置方式與頂級 subagent 相同，並從相同的 [scopes](#choose-the-subagent-scope) 解析。

提示輸入下方的 subagent 面板顯示完整樹：每一行顯示後代的 `(+N)` 計數，自 v2.1.193 起，打開一行會顯示該 subagent 的同級和直接子代，以及返回到 `main` 的路徑。

深度計算為主要對話下方的 subagent 級別數，無論每個級別是否在 [前景或背景](#run-subagents-in-foreground-or-background) 中執行。深度為五的 subagent 不接收 Agent 工具，無法進一步產生。限制是固定的且不可配置。

自 Claude Code v2.1.187 起，背景 subagent 的深度在首次產生時固定，之後 [恢復](#resume-subagents) 它不會改變該深度。例如，如果您的主要對話產生 subagent A，而 A 在深度二產生背景 subagent B，當您直接從主要對話恢復 B 時，B 仍然在深度二。從較淺的上下文恢復 subagent 不會讓它產生深度限制已經阻止的額外級別。

若要防止特定 subagent 產生其他 subagents，請從其 [`tools`](#available-tools) 列表中省略 `Agent` 或將其新增到 `disallowedTools`。

[fork](#fork-the-current-conversation) 仍然無法產生另一個 fork。它可以產生其他 subagent 類型，這些計入深度限制。

<h3 id="manage-subagent-context">
  管理 subagent 上下文
</h3>

<h4 id="what-loads-at-startup">
  啟動時載入的內容
</h4>

每個 subagent 都以新鮮、隔離的上下文視窗開始。它看不到您的對話歷史記錄、您已經呼叫的技能或 Claude 已經讀取的檔案。Claude 撰寫一條委派訊息來總結任務，subagent 從那裡開始工作。例外是 [fork](#fork-the-current-conversation)，它繼承父對話而不是從頭開始。

非 fork subagent 的初始上下文包含：

* **系統提示**：代理自己的提示加上 Claude Code 附加的環境詳細資訊，而不是完整的 Claude Code 系統提示。自訂 subagents 在 [markdown 正文](#write-subagent-files) 或 `prompt` 欄位中定義它們。內建代理有預定義的提示。
* **任務訊息**：Claude 在交接工作時編寫的委派提示。
* **CLAUDE.md 和記憶**：主要對話載入的 [記憶層級](/docs/zh-TW/memory#how-claude-md-files-load) 的每個級別，包括 `~/.claude/CLAUDE.md`、專案規則、`CLAUDE.local.md` 和受管理的政策檔案。內建的 Explore 和 Plan 代理跳過這個。
* **Git 狀態**：在父工作階段開始時拍攝的快照。當工作目錄不是 Git 儲存庫或當 [`includeGitInstructions`](/docs/zh-TW/settings#available-settings) 為 `false` 時不存在。Explore 和 Plan 無論如何都跳過它。
* **預載入的技能**：代理的 [`skills` 欄位](#preload-skills-into-subagents) 中命名的任何技能的完整內容。內建代理不預載入技能。
* **同級名單**：系統提醒，列出 `main` 和工作階段中的每個其他命名代理，每個都是 [`SendMessage`](#resume-subagents) 的有效 `to` 值。需要 Claude Code v2.1.206 或更新版本。名單僅在 subagent 的工具包括 `SendMessage` 且至少有一個其他代理有名稱時出現，無論 Claude 在產生時命名它還是它作為 [agent teams](/docs/zh-TW/agent-teams) 隊友執行。它是在 subagent 啟動時拍攝的快照，所以稍後命名的代理不會出現。

Explore 和 Plan 是唯一省略 CLAUDE.md 和 git 狀態的 subagents。沒有 frontmatter 欄位或每個代理設定來改變哪些代理跳過它們。

主要對話使用完整 CLAUDE.md 上下文讀取 Explore 和 Plan 結果，所以大多數規則不需要到達 subagent 本身。如果規則必須，例如「忽略 `vendor/` 目錄」，在您委派時給 Claude 的提示中重新陳述它。

<h4 id="resume-subagents">
  恢復 subagents
</h4>

每個 subagent 呼叫都會建立一個具有新鮮上下文的新實例。若要繼續現有 subagent 的工作而不是重新開始，請要求 Claude 恢復它。

恢復的 subagents 保留其完整對話歷史記錄，包括所有先前的工具呼叫、結果和推理。Subagent 從停止的地方精確繼續，而不是從頭開始。

當 subagent 完成時，Claude 接收其代理 ID。內建的 Explore 和 Plan 代理是一次性的，不返回代理 ID，所以它們無法被恢復；當您需要繼續工作時，請使用 `general-purpose` 或自訂 subagent。

Claude 使用 `SendMessage` 工具，將代理的 ID 或名稱作為 `to` 欄位來恢復它。`SendMessage` 不需要啟用 [agent teams](/docs/zh-TW/agent-teams)；只有結構化的團隊協議訊息，例如 `shutdown_request` 和 `plan_approval_response`，才需要啟用。

若要恢復 subagent，請要求 Claude 繼續先前的工作：

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

如果停止的 subagent 接收 `SendMessage`，它會自動在背景中恢復，無需新的 `Agent` 呼叫。同樣適用於 Claude 使用 `TaskStop` 工具停止的 subagent。

自 v2.1.191 起，您自己停止的 subagent，使用 `/tasks` 中的 `x` 或 SDK `stop_task` 請求，不會自動恢復。`SendMessage` 呼叫返回拒絕，告訴 Claude 代理已被取消。在 subagent 面板中輸入該 subagent 的文字以自己恢復它，這會清除停止，以便稍後 `SendMessage` 呼叫可以再次自動恢復它。

恢復會在相同 ID 下啟動代理的新執行，所以已經失敗或完成的 subagent 在任務列表和 Agent SDK 的任務事件中再次顯示為執行中。在 v2.1.205 之前，它在恢復的執行工作時保持顯示其較早的失敗或完成狀態。

自 v2.1.199 起，`SendMessage` 檢查名稱是否仍然指向它在對話中較早時到達的同一代理。如果較新的代理已取得該名稱，例如重新產生的背景代理重複使用了它，Claude Code 會拒絕發送，而不是將其傳遞給錯誤的代理，錯誤會報告該名稱現在到達的代理，以便 Claude 可以重新定位。若要在較早的代理仍在執行時到達它，Claude 會透過其產生結果中的代理 ID 來定址它。檢查的範圍是目前對話，並在 `/clear` 時重置。

自 v2.1.198 起，subagent 將來自啟動它的代理的訊息視為正常任務方向，包括中途任務課程更正，並在其自己的權限設定內對其進行操作。無論誰發送訊息，兩個限制仍然成立：來自任何代理的任何訊息都不計為您對待處理權限提示的批准，任何代理訊息都無法改變 subagent 的權限設定、`CLAUDE.md` 或配置。只有權限系統或您自己的訊息可以授予批准。

您也可以要求 Claude 提供代理 ID，如果您想明確參考它，或在 `~/.claude/projects/{project}/{sessionId}/subagents/` 的文字檔案中找到 ID。每個文字都儲存為 `agent-{agentId}.jsonl`。

Subagent 文字獨立於主要對話持續存在：

* **主要對話壓縮**：當主要對話壓縮時，subagent 文字不受影響。它們儲存在單獨的檔案中。
* **工作階段持續性**：Subagent 文字在其工作階段內持續存在。您可以透過恢復相同工作階段在重新啟動 Claude Code 後 [恢復 subagent](#resume-subagents)。
* **自動清理**：文字根據 `cleanupPeriodDays` 設定進行清理，預設為 30 天。

<h4 id="auto-compaction">
  自動壓縮
</h4>

Subagents 支援使用與主要對話相同的邏輯進行自動壓縮。壓縮在相同條件下觸發，`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` 也適用於 subagents。請參閱 [environment variables](/docs/zh-TW/env-vars) 以了解何時覆蓋生效。

壓縮事件記錄在 subagent 文字檔案中：

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

`preTokens` 值顯示壓縮發生前使用了多少個 tokens。

<h2 id="fork-the-current-conversation">
  Fork 目前的對話
</h2>

<Note>
  Forked subagents 需要 Claude Code v2.1.117 或更新版本。從 v2.1.161 開始，`/fork` 命令預設啟用；在較早版本中，它需要將 [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/zh-TW/env-vars) 環境變數設定為 `1`。讓 Claude 本身產生 forks 是實驗性的，可能在未來版本中變更。此功能也可能在互動式工作階段中啟用，作為分階段推出的一部分。
</Note>

Fork 是一個 subagent，它繼承到目前為止的整個對話，而不是從頭開始。這會放棄 subagents 否則提供的輸入隔離：fork 看到與主工作階段相同的系統提示、工具、模型和訊息歷史記錄，因此您可以將側面任務交給它，而無需重新解釋情況。Fork 自己的工具呼叫仍然保持在您的對話之外，只有其最終結果返回，因此您的主要上下文視窗保持乾淨。當命名 subagent 需要太多背景才能有用時，或當您想從相同的起點並行嘗試多種方法時，使用 fork。

若要控制 fork 模式，無論分階段推出如何，請將 [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/zh-TW/env-vars) 設定為 `1` 以明確啟用它，或設定為 `0` 以停用它。該變數在互動模式中以及透過 SDK 或 `claude -p` 被接受。

啟用 fork 模式會以兩種方式改變 Claude Code：

* Claude 可以透過明確要求 `fork` subagent 類型來產生 fork。沒有 subagent 類型的產生仍然使用 [general-purpose](#built-in-subagents) subagent，而命名 subagents（例如 Explore）仍然像以前一樣產生。
* 每個 subagent 產生都在 [background](#run-subagents-in-foreground-or-background) 中執行，無論它是 fork 還是命名 subagent。設定 `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` 為 `1` 以保持產生同步。

您可以使用 `/fork` 後跟指令自己啟動 fork，無論是否設定了變數。Claude Code 從指令的前幾個詞命名 fork。以下範例 forks 對話以在您在主工作階段中繼續實現時草擬測試案例：

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

Fork 出現在提示下方的面板中，並在您繼續工作時在背景中執行。完成後，其結果作為訊息到達您的主要對話。下一部分涵蓋面板控制項，用於在 forks 執行時觀察和引導它們。

<h3 id="observe-and-steer-running-forks">
  觀察和引導執行中的 forks
</h3>

執行中的 forks 出現在提示輸入下方的面板中，主工作階段有一行，每個 fork 有一行。使用這些鍵與面板互動：

| Key       | Action                   |
| :-------- | :----------------------- |
| `↑` / `↓` | 在行之間移動                   |
| `Enter`   | 開啟選定 fork 的文字記錄並向其發送後續訊息 |
| `x`       | 關閉完成的 fork 或停止執行中的 fork  |
| `Esc`     | 將焦點返回到提示輸入               |

使用 fork 或 subagent 的文字記錄開啟時，後續訊息和 [skills](/docs/zh-TW/skills) 會傳送到該代理，但內建命令仍在您的主要對話中執行。從 v2.1.199 開始，在該檢視中輸入 `/model` 或 `/fast` 會顯示通知，表示它會變更主要對話的模型或快速模式，而不是檢視的代理，而不是以無聲方式執行。

<h3 id="how-forks-differ-from-named-subagents">
  Forks 與命名 subagents 的區別
</h3>

Fork 繼承主工作階段在產生時擁有的所有內容。命名 subagent 從自己的定義開始。

|              | Fork       | 命名 subagent                                                                                |
| :----------- | :--------- | :----------------------------------------------------------------------------------------- |
| Context      | 完整對話歷史記錄   | 新鮮上下文，帶有您傳遞的提示                                                                             |
| 系統提示和工具      | 與主工作階段相同   | 來自 subagent 的 [definition file](#write-subagent-files)                                     |
| Model        | 與主工作階段相同   | 來自 subagent 的 `model` 欄位                                                                   |
| Permissions  | 提示出現在您的終端中 | [Prompts surface in your main session](#run-subagents-in-foreground-or-background) 在背景中執行時 |
| Prompt cache | 與主工作階段共享   | 單獨的快取                                                                                      |

因為 fork 的系統提示和工具定義與父級相同，其第一個請求重複使用父級的 [prompt cache](/docs/zh-TW/prompt-caching#subagents-and-the-cache)。這使得 forking 比為需要相同上下文的任務產生新 subagent 更便宜。

當 Claude 透過 Agent 工具產生 fork 時，它可以傳遞 `isolation: "worktree"`，以便 fork 的檔案編輯被寫入單獨的 git worktree 而不是您的簽出。

<h3 id="limitations">
  限制
</h3>

設定 `CLAUDE_CODE_FORK_SUBAGENT=1` 在互動式工作階段、[non-interactive mode](/docs/zh-TW/headless) 和 Agent SDK 中啟用 fork 模式；將其設定為 `0` 會在所有地方停用 fork 模式，包括任何伺服器端推出。Fork 無法產生進一步的 forks。

<h2 id="example-subagents">
  範例 subagents
</h2>

這些範例展示了建立 subagents 的有效模式。將它們用作起點，或使用 Claude 生成自訂版本。

<Tip>
  **最佳實踐：**

  * **設計專注的 subagents：** 每個 subagent 應該在一個特定任務上表現出色
  * **寫詳細的描述：** Claude 使用描述來決定何時委派
  * **限制工具存取：** 僅授予必要的權限以確保安全和專注
  * **簽入版本控制：** 與您的團隊共享專案 subagents
</Tip>

<h3 id="code-reviewer">
  程式碼審查者
</h3>

一個唯讀 subagent，審查程式碼而不修改它。此範例展示如何設計一個具有有限工具存取（無 Edit 或 Write）和詳細提示的專注 subagent，該提示明確指定要查找的內容以及如何格式化輸出。

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  除錯器
</h3>

一個可以分析和修復問題的 subagent。與程式碼審查者不同，這個包括 Edit，因為修復錯誤需要修改程式碼。提示提供了從診斷到驗證的清晰工作流程。

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  資料科學家
</h3>

一個用於資料分析工作的特定領域 subagent。此範例展示如何為典型編碼任務之外的專門工作流程建立 subagents。它明確設定 `model: sonnet` 以進行更有能力的分析。

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  資料庫查詢驗證器
</h3>

一個允許 Bash 存取但驗證命令以僅允許唯讀 SQL 查詢的 subagent。此範例展示如何在需要比 `tools` 欄位提供的更精細控制時使用 `PreToolUse` hooks。

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [透過 stdin 將 hook 輸入作為 JSON 傳遞](/docs/zh-TW/hooks#pretooluse-input) 給 hook 命令。驗證指令碼讀取此 JSON，提取正在執行的命令，並根據 SQL 寫入操作清單檢查它。如果檢測到寫入操作，指令碼 [以代碼 2 退出](/docs/zh-TW/hooks#exit-code-2-behavior-per-event) 以阻止執行並透過 stderr 向 Claude 返回錯誤訊息。

在專案中的任何位置建立驗證指令碼。路徑必須與 hook 配置中的 `command` 欄位相符：

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

在 macOS 和 Linux 上，使指令碼可執行：

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

在 Windows 上，使用 PowerShell 編寫驗證指令碼，並將 `shell: powershell` 新增至 hook 項目。請參閱 [在 PowerShell 中執行 hooks](/docs/zh-TW/hooks#windows-powershell-tool)。

Hook 透過 stdin 接收 JSON，Bash 命令在 `tool_input.command` 中。退出代碼 2 阻止操作並將錯誤訊息反饋給 Claude。請參閱 [Hooks](/docs/zh-TW/hooks#exit-code-output) 以了解退出代碼和 [Hook input](/docs/zh-TW/hooks#pretooluse-input) 以了解完整的輸入架構。

<h2 id="next-steps">
  後續步驟
</h2>

現在您理解了 subagents，請探索這些相關功能：

* [使用外掛程式分發 subagents](/docs/zh-TW/plugins) 以跨團隊或專案共享 subagents
* [以程式方式執行 Claude Code](/docs/zh-TW/headless) 使用 Agent SDK 進行 CI/CD 和自動化
* [使用 MCP 伺服器](/docs/zh-TW/mcp) 為 subagents 提供對外部工具和資料的存取
