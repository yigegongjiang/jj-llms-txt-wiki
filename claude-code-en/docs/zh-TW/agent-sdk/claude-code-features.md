> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 SDK 中使用 Claude Code 功能

> 將專案指令、skills、hooks 和其他 Claude Code 功能載入到您的 SDK 代理中。

Agent SDK 建立在與 Claude Code 相同的基礎上，這意味著您的 SDK 代理可以存取相同的基於檔案系統的功能：專案指令（`CLAUDE.md` 和規則）、skills、hooks 等。

當您省略 `settingSources` 時，`query()` 會讀取與 Claude Code CLI 相同的檔案系統設定：使用者、專案和本機設定、CLAUDE.md 檔案以及 `.claude/` skills、代理和命令。若要在沒有這些的情況下執行，請傳遞 `settingSources: []`，這會將代理限制為您以程式設計方式設定的內容。無論此選項如何，都會讀取受管原則設定和全域 `~/.claude.json` 設定。請參閱 [settingSources 不控制的內容](#what-settingsources-does-not-control)。

如需每項功能的概念概述及何時使用，請參閱 [擴展 Claude Code](/docs/zh-TW/features-overview)。

<h2 id="control-filesystem-settings-with-settingsources">
  使用 settingSources 控制檔案系統設定
</h2>

設定來源選項（Python 中的 [`setting_sources`](/docs/zh-TW/agent-sdk/python#claudeagentoptions)、TypeScript 中的 [`settingSources`](/docs/zh-TW/agent-sdk/typescript#settingsource)）控制 SDK 載入哪些基於檔案系統的設定。傳遞明確清單以選擇加入特定來源，或傳遞空陣列以停用使用者、專案和本機設定。

此範例透過將 `settingSources` 設定為 `["user", "project"]` 來載入使用者層級和專案層級設定：

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

  async for message in query(
      prompt="Help me refactor the auth module",
      options=ClaudeAgentOptions(
          # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
          # Together they give the agent access to CLAUDE.md, skills, hooks, and
          # permissions from both locations.
          setting_sources=["user", "project"],
          allowed_tools=["Read", "Edit", "Bash"],
      ),
  ):
      if isinstance(message, AssistantMessage):
          for block in message.content:
              if hasattr(block, "text"):
                  print(block.text)
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(f"\nResult: {message.result}")
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

每個來源都會從特定位置載入設定，其中 `<cwd>` 是您透過 `cwd` 選項傳遞的工作目錄，或如果未設定則為程序的目前目錄。如需完整的型別定義，請參閱 [`SettingSource`](/docs/zh-TW/agent-sdk/typescript#settingsource)（TypeScript）或 [`SettingSource`](/docs/zh-TW/agent-sdk/python#settingsource)（Python）。

| 來源          | 載入的內容                                                                   | 位置                                                                                                             |
| :---------- | :---------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| `"project"` | 專案 CLAUDE.md、`.claude/rules/*.md`、專案 skills、專案 hooks、專案 `settings.json` | `<cwd>/.claude/` 用於 `settings.json` 和 hooks；`<cwd>` 和每個父目錄用於 CLAUDE.md 和 rules；`<cwd>` 和每個父目錄直到儲存庫根目錄用於 skills |
| `"user"`    | 使用者 CLAUDE.md、`~/.claude/rules/*.md`、使用者 skills、使用者設定                   | `~/.claude/`                                                                                                   |
| `"local"`   | CLAUDE.local.md、`.claude/settings.local.json`                           | `<cwd>/.claude/` 用於 `settings.local.json`；`<cwd>` 和每個父目錄用於 CLAUDE.local.md                                     |

省略 `settingSources` 等同於 `["user", "project", "local"]`。

`cwd` 選項決定 SDK 在何處尋找專案層級輸入。CLAUDE.md 和 rules 從 `<cwd>` 和每個父目錄載入。Skills 從 `<cwd>` 和每個父目錄直到儲存庫根目錄載入。專案 `settings.json` 和 hooks 僅從 `<cwd>/.claude/` 載入，沒有父目錄回退。

<h3 id="what-settingsources-does-not-control">
  settingSources 不控制的內容
</h3>

`settingSources` 涵蓋使用者、專案和本機設定。無論其值如何，都會讀取一些輸入：

| 輸入                                                             | 行為                                                                                                                                                                                        | 停用方式                                                                                                                                                          |
| :------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 受管原則設定                                                         | 端點管理的原則（無論是 MDM plist、登錄原則或受管設定檔）從主機載入；[伺服器管理的設定](/docs/zh-TW/server-managed-settings)在工作階段使用組織 OAuth 登入或直接配置的 API 金鑰進行驗證時擷取，在[符合條件的配置](/docs/zh-TW/server-managed-settings#platform-availability)上 | 端點原則：從主機移除受管設定檔、plist 或登錄原則。伺服器管理的設定：由您的組織管理員控制；無法從 SDK 停用                                                                                                    |
| `~/.claude.json` 全域設定                                          | 始終讀取                                                                                                                                                                                      | 在 `env` 中使用 `CLAUDE_CONFIG_DIR` 重新定位                                                                                                                          |
| `~/.claude/projects/<project>/memory/` 的自動記憶體                  | 預設載入到系統提示中。代理程式使用標準 `Write` 和 `Edit` 工具而非專用記憶體工具寫入新記憶體，因此必須啟用這些工具才能讓代理程式儲存記憶體                                                                                                             | 在設定中設定 `autoMemoryEnabled: false`，或在 `env` 中設定 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`                                                                            |
| [claude.ai MCP 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai) | 當作用中驗證方法是 claude.ai 訂閱時載入。傳遞 `mcpServers: {}` 不會抑制它們                                                                                                                                      | 設定 `strictMcpConfig: true`、[`disableClaudeAiConnectors: true`](/docs/zh-TW/mcp#disable-claude-ai-connectors) 在設定中，或在 `env` 中設定 `ENABLE_CLAUDEAI_MCP_SERVERS=false` |

<Warning>
  不要依賴預設 `query()` 選項進行多租戶隔離。因為上述輸入無論 `settingSources` 如何都會被讀取，SDK 程序可能會拾取主機層級設定和每個目錄的記憶體。對於多租戶部署，在其自己的檔案系統中執行每個租戶，並設定 `settingSources: []` 加上在 `env` 中設定 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`。[伺服器管理的設定](/docs/zh-TW/server-managed-settings)在程序使用組織認證進行驗證時擷取；檔案系統隔離不會移除它們。請參閱[安全部署](/docs/zh-TW/agent-sdk/secure-deployment)。
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  專案指令（CLAUDE.md 和規則）
</h2>

`CLAUDE.md` 檔案和 `.claude/rules/*.md` 檔案為您的代理提供有關您的專案的持久上下文：編碼慣例、建置命令、架構決策和指令。當 `settingSources` 包含 `"project"`（如上面的範例所示）時，SDK 在工作階段開始時將這些檔案載入到上下文中。代理隨後會遵循您的專案慣例，而無需在每個提示中重複它們。

<h3 id="claude-md-load-locations">
  CLAUDE.md 載入位置
</h3>

| 層級      | 位置                                                        | 何時載入                                              |
| :------ | :-------------------------------------------------------- | :------------------------------------------------ |
| 專案（根目錄） | `<cwd>/CLAUDE.md` 或 `<cwd>/.claude/CLAUDE.md`             | `settingSources` 包含 `"project"`                   |
| 專案規則    | `<cwd>/.claude/rules/*.md` 和 `.claude/rules/*.md` 在每個父目錄中 | `settingSources` 包含 `"project"`                   |
| 專案（父目錄） | `cwd` 上方目錄中的 `CLAUDE.md` 檔案                               | `settingSources` 包含 `"project"`，在工作階段開始時載入        |
| 專案（子目錄） | `cwd` 子目錄中的 `CLAUDE.md` 檔案                                | `settingSources` 包含 `"project"`，當代理讀取該子樹中的檔案時按需載入 |
| 本機      | `<cwd>/CLAUDE.local.md` 和 `CLAUDE.local.md` 在每個父目錄中       | `settingSources` 包含 `"local"`                     |
| 使用者     | `~/.claude/CLAUDE.md`                                     | `settingSources` 包含 `"user"`                      |
| 使用者規則   | `~/.claude/rules/*.md`                                    | `settingSources` 包含 `"user"`                      |

所有層級都是累加的：如果專案和使用者 CLAUDE.md 檔案都存在，代理會看到兩者。層級之間沒有硬性優先順序規則；如果指令衝突，結果取決於 Claude 如何解釋它們。編寫不衝突的規則，或在更具體的檔案中明確說明優先順序（「這些專案指令會覆蓋任何衝突的使用者層級預設值」）。

<Tip>
  您也可以透過 `systemPrompt` 直接注入上下文，而無需使用 CLAUDE.md 檔案。請參閱 [修改系統提示](/docs/zh-TW/agent-sdk/modifying-system-prompts)。當您想要在互動式 Claude Code 工作階段和 SDK 代理之間共享相同上下文時，請使用 CLAUDE.md。
</Tip>

如需如何結構化和組織 CLAUDE.md 內容的資訊，請參閱 [管理 Claude 的記憶體](/docs/zh-TW/memory)。

<h2 id="skills">
  Skills
</h2>

Skills 是 markdown 檔案，為您的代理提供專門知識和可呼叫的工作流程。與 `CLAUDE.md`（每個工作階段都載入）不同，skills 按需載入。代理在啟動時接收 skill 描述，並在相關時載入完整內容。

Skills 透過 `settingSources` 從檔案系統中發現。當 `query()` 上的 `skills` 選項被省略時，已發現的使用者和專案 skills 會被啟用，且 Skill 工具可用，符合 CLI 行為。若要控制啟用哪些 skills，請將 `skills` 傳遞為 `"all"`、skill 名稱清單或 `[]` 以停用全部。當 `skills` 被設定時，SDK 會自動將 Skill 工具新增至 `allowedTools`。如果您也傳遞明確的 `tools` 清單，請在該清單中包含 `"Skill"`，以便 Claude 可以呼叫 skills。

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async for message in query(
      prompt="Review this PR using our code review checklist",
      options=ClaudeAgentOptions(
          setting_sources=["user", "project"],
          skills="all",
          allowed_tools=["Read", "Grep", "Glob"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Skills 必須建立為檔案系統成品（`.claude/skills/<name>/SKILL.md`）。SDK 沒有用於註冊 skills 的程式設計 API。請參閱 [SDK 中的代理 Skills](/docs/zh-TW/agent-sdk/skills) 以取得完整詳細資訊。
</Note>

如需建立和使用 skills 的詳細資訊，請參閱 [SDK 中的代理 Skills](/docs/zh-TW/agent-sdk/skills)。

<h2 id="hooks">
  Hooks
</h2>

SDK 支援兩種定義 hooks 的方式，它們並行執行：

* **檔案系統 hooks：** 在 `settings.json` 中定義的 shell 命令，當 `settingSources` 包含相關來源時載入。這些是您為 [互動式 Claude Code 工作階段](/docs/zh-TW/hooks-guide) 設定的相同 hooks。
* **程式設計 hooks：** 直接傳遞給 `query()` 的回呼函式。這些在您的應用程式程序中執行，可以返回結構化決策。請參閱 [使用 hooks 控制執行](/docs/zh-TW/agent-sdk/hooks)。

兩種類型都在相同的 hook 生命週期中執行。如果您已經在專案的 `.claude/settings.json` 中有 hooks，並且您設定 `settingSources: ["project"]`，那些 hooks 會在 SDK 中自動執行，無需額外設定。

Hook 回呼接收工具輸入並返回決策字典。返回 `{}` 表示允許工具繼續。若要阻止執行，請返回一個 `hookSpecificOutput` 物件，其中包含 `permissionDecision: "deny"` 和 `permissionDecisionReason`。原因會作為工具結果發送給 Claude。頂層的 `decision` 和 `reason` 欄位已針對 `PreToolUse` 棄用。請參閱 [hooks 指南](/docs/zh-TW/agent-sdk/hooks) 以取得完整的回呼簽名和返回型別。

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, carries session metadata
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async for message in query(
      prompt="Refactor the auth module",
      options=ClaudeAgentOptions(
          setting_sources=["project"],  # Loads hooks from .claude/settings.json
          hooks={
              "PreToolUse": [
                  HookMatcher(matcher="Bash", hooks=[audit_bash]),
              ]
          },
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  何時使用哪種 hook 類型
</h3>

| Hook 類型                   | 最適合                                                                                                                                                                  |
| :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **檔案系統**（`settings.json`） | 在 CLI 和 SDK 工作階段之間共享 hooks。支援 `"command"`（shell 指令碼）、`"http"`（POST 到端點）、`"mcp_tool"`（呼叫連接的 MCP 伺服器的工具）、`"prompt"`（LLM 評估提示）和 `"agent"`（生成驗證器代理）。這些在主代理和它生成的任何子代理中執行。 |
| **程式設計**（`query()` 中的回呼）  | 應用程式特定邏輯、結構化決策和進程內整合。這些也在子代理內執行。回呼接收 `agent_id` 和 `agent_type` 以進行區分。                                                                                                |

<Note>
  TypeScript SDK 支援超出 Python 的其他 hook 事件，包括 `SessionStart`、`SessionEnd`、`TeammateIdle` 和 `TaskCompleted`。請參閱 [hooks 指南](/docs/zh-TW/agent-sdk/hooks) 以取得完整的事件相容性表。
</Note>

如需程式設計 hooks 的完整詳細資訊，請參閱 [使用 hooks 控制執行](/docs/zh-TW/agent-sdk/hooks)。如需檔案系統 hook 語法，請參閱 [Hooks](/docs/zh-TW/hooks)。

<h2 id="choose-the-right-feature">
  選擇正確的功能
</h2>

Agent SDK 為您提供了多種方式來擴展代理的行為。如果您不確定要使用哪一種，此表將常見目標對應到正確的方法。

| 您想要...                                  | 使用                                        | SDK 表面                                                   |
| :-------------------------------------- | :---------------------------------------- | :------------------------------------------------------- |
| 設定代理始終遵循的專案慣例                           | [CLAUDE.md](/docs/zh-TW/memory)                | `settingSources: ["project"]` 會自動載入它                     |
| 為代理提供它在相關時載入的參考資料                       | [Skills](/docs/zh-TW/agent-sdk/skills)         | `settingSources` + `skills` 選項                           |
| 執行可重複使用的工作流程（部署、審查、發佈）                  | [使用者可呼叫的 skills](/docs/zh-TW/agent-sdk/skills) | `settingSources` + `skills` 選項                           |
| 將隔離的子任務委派給新的上下文（研究、審查）                  | [子代理](/docs/zh-TW/agent-sdk/subagents)         | `agents` 參數 + `allowedTools: ["Agent"]`                  |
| 協調多個 Claude Code 實例，具有共享任務清單和直接的代理間訊息傳遞 | [代理團隊](/docs/zh-TW/agent-teams)                | 不直接透過 SDK 選項設定。代理團隊是一個 CLI 功能，其中一個工作階段充當團隊主管，協調獨立隊友之間的工作 |
| 在工具呼叫上執行確定性邏輯（審計、阻止、轉換）                 | [Hooks](/docs/zh-TW/agent-sdk/hooks)           | `hooks` 參數與回呼，或透過 `settingSources` 載入的 shell 指令碼         |
| 為 Claude 提供對外部服務的結構化工具存取                | [MCP](/docs/zh-TW/agent-sdk/mcp)               | `mcpServers` 參數                                          |

<Tip>
  **子代理與代理團隊：** 子代理是短暫的和隔離的：新的對話、一個任務、摘要返回給父代理。代理團隊協調多個獨立的 Claude Code 實例，這些實例共享任務清單並直接相互訊息傳遞。代理團隊是一個 CLI 功能。請參閱 [子代理繼承的內容](/docs/zh-TW/agent-sdk/subagents#what-subagents-inherit) 和 [代理團隊比較](/docs/zh-TW/agent-teams#compare-with-subagents) 以取得詳細資訊。
</Tip>

您啟用的每項功能都會增加代理的上下文視窗。如需每項功能的成本以及這些功能如何分層組合，請參閱 [擴展 Claude Code](/docs/zh-TW/features-overview#understand-context-costs)。

<h2 id="related-resources">
  相關資源
</h2>

* [擴展 Claude Code](/docs/zh-TW/features-overview)：所有擴展功能的概念概述，包含比較表和上下文成本分析
* [SDK 中的 Skills](/docs/zh-TW/agent-sdk/skills)：以程式設計方式使用 skills 的完整指南
* [子代理](/docs/zh-TW/agent-sdk/subagents)：為隔離的子任務定義和呼叫子代理
* [Hooks](/docs/zh-TW/agent-sdk/hooks)：在關鍵執行點攔截和控制代理行為
* [權限](/docs/zh-TW/agent-sdk/permissions)：使用模式、規則和回呼控制工具存取
* [系統提示](/docs/zh-TW/agent-sdk/modifying-system-prompts)：在不使用 CLAUDE.md 檔案的情況下注入上下文
