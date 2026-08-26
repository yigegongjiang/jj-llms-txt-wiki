> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK 中的子代理

> 定義和調用子代理以隔離上下文、並行運行任務，以及在 Claude Agent SDK 應用程式中應用專門化指令。

子代理是您的主代理可以生成的獨立代理實例，用於處理專注的子任務。
使用子代理來隔離專注子任務的上下文、並行運行多個分析，以及應用專門化指令，而不會使主代理的提示詞變得臃腫。

本指南說明如何使用 `agents` 參數在 SDK 中定義和使用子代理。

<h2 id="overview">
  概述
</h2>

您可以通過三種方式建立子代理：

* **以程式方式**：在您的 `query()` 選項中使用 `agents` 參數。請參閱 [TypeScript](/docs/zh-TW/agent-sdk/typescript#agentdefinition) 和 [Python](/docs/zh-TW/agent-sdk/python#agentdefinition) 參考資料
* **基於檔案系統**：在 `.claude/agents/` 目錄中將代理定義為 markdown 檔案。請參閱[將子代理定義為檔案](/docs/zh-TW/sub-agents)
* **內置通用代理**：Claude 可以隨時通過 Agent 工具呼叫內置的 `general-purpose` 子代理，無需您定義任何內容

本指南重點介紹程式化方法，這是 SDK 應用程式的推薦方法。

定義子代理時，Claude 根據每個子代理的 `description` 欄位確定是否呼叫它。編寫清晰的描述，說明何時應使用子代理，Claude 將自動委派適當的任務。您也可以在提示詞中按名稱明確請求子代理，例如「使用代碼審查員代理來...」。

<h2 id="benefits-of-using-subagents">
  使用子代理的好處
</h2>

<h3 id="context-isolation">
  上下文隔離
</h3>

每個子代理在其自己的新對話中運行。中間工具調用和結果保留在子代理內；只有其最終消息返回到父代理。請參閱[子代理繼承的內容](#what-subagents-inherit)以了解子代理上下文中的確切內容。

**示例：** `research-assistant` 子代理可以探索數十個檔案，而無需任何該內容在主對話中累積。父代理接收簡潔的摘要，而不是子代理讀取的每個檔案。

<h3 id="parallelization">
  並行化
</h3>

多個子代理可以並發運行，因此獨立的子任務在最慢的一個的時間內完成，而不是所有任務時間的總和。

**示例：** 在代碼審查期間，您可以同時運行 `style-checker`、`security-scanner` 和 `test-coverage` 子代理，而不是按順序運行。

<h3 id="specialized-instructions-and-knowledge">
  專門化指令和知識
</h3>

每個子代理可以有具有特定專業知識、最佳實踐和約束的定製系統提示詞。

**示例：** `database-migration` 子代理可以具有有關 SQL 最佳實踐、回滾策略和資料完整性檢查的詳細知識，這些在主代理的指令中將是不必要的噪音。

<h3 id="tool-restrictions">
  工具限制
</h3>

子代理可以限制為特定工具，降低意外操作的風險。

**示例：** `doc-reviewer` 子代理可能只能訪問 Read 和 Grep 工具，確保它可以分析但永遠不會意外修改您的文檔檔案。

<h2 id="create-subagents">
  創建子代理
</h2>

<h3 id="programmatic-definition-recommended">
  程式化定義（推薦）
</h3>

使用 `agents` 參數直接在代碼中定義子代理。Claude 通過 `Agent` 工具調用子代理，因此請在 `allowedTools` 中包含 `Agent` 以自動批准子代理調用，無需權限提示。

本頁面上的大多數示例僅列印最終結果。若要確認 Claude 委派給子代理而不是直接回答，請參閱[檢測子代理調用](#detect-subagent-invocation)。

此示例創建兩個子代理：一個具有唯讀訪問權限的代碼審查員和一個可以執行命令的測試運行器。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools, including Agent for subagent invocation
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  AgentDefinition 配置
</h3>

| 欄位                | 類型                                                          | 必需 | 描述                                                                                                          |
| :---------------- | :---------------------------------------------------------- | :- | :---------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | 是  | 何時使用此代理的自然語言描述                                                                                              |
| `prompt`          | `string`                                                    | 是  | 代理的系統提示詞，定義其角色和行為                                                                                           |
| `tools`           | `string[]`                                                  | 否  | 允許的工具名稱陣列。如果省略，繼承所有工具                                                                                       |
| `disallowedTools` | `string[]`                                                  | 否  | 要從代理的工具集中移除的工具名稱陣列。MCP 伺服器級別的模式也被接受：`mcp__server` 或 `mcp__server__*` 移除該伺服器的每個工具，`mcp__*` 移除任何伺服器的每個 MCP 工具 |
| `model`           | `string`                                                    | 否  | 此代理的模型覆蓋。接受別名，例如 `'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'`，或完整模型 ID。如果省略，預設為主模型                   |
| `skills`          | `string[]`                                                  | 否  | 在啟動時預加載到代理上下文中的 skills 名稱列表。未列出的 skills 仍可通過 Skill 工具調用                                                     |
| `memory`          | `'user' \| 'project' \| 'local'`                            | 否  | 此代理的記憶體來源                                                                                                   |
| `mcpServers`      | `(string \| object)[]`                                      | 否  | 此代理可用的 MCP 伺服器，按名稱或內聯配置                                                                                     |
| `initialPrompt`   | `string`                                                    | 否  | 當此代理作為主執行緒代理運行時，自動提交為第一個使用者回合。當代理作為子代理調用時忽略                                                                 |
| `maxTurns`        | `number`                                                    | 否  | 代理停止前的最大代理轉數                                                                                                |
| `background`      | `boolean`                                                   | 否  | 調用時將此代理作為非阻塞背景任務運行                                                                                          |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | 否  | 此代理的推理努力級別                                                                                                  |
| `permissionMode`  | `PermissionMode`                                            | 否  | 此代理內工具執行的權限模式                                                                                               |

在 Python SDK 中，多字欄位名稱（例如 `disallowedTools` 和 `mcpServers`）保持其 camelCase 拼寫以匹配線路格式，而不是遵循 Python 的 snake\_case 慣例。有關詳細信息，請參閱 [`AgentDefinition` 參考](/docs/zh-TW/agent-sdk/python#agentdefinition)。

Claude Code v2.1.198 中的兩個子代理行為已更改：

* 子代理預設在背景中運行。省略 [`run_in_background`](/docs/zh-TW/agent-sdk/typescript) 輸入的 Agent 工具調用會啟動背景子代理，當 Claude 需要結果才能繼續時，它會設定 `run_in_background: false`。在 v2.1.198 之前，省略 `run_in_background` 會同步運行子代理。設定 `background` 欄位為 `true` 以強制特定代理進行背景執行，無論 Claude 請求什麼。
* 子代理繼承主會話的擴展思考配置。在較早的版本中，無論主會話的設定如何，擴展思考在子代理內被禁用。

<Note>
  自 Claude Code v2.1.172 起，子代理可以生成自己的子代理。位於主代理下方五個級別的子代理無法生成進一步的子代理，無論其是否在前景或背景中運行。若要防止子代理生成其他子代理，請從其 `tools` 陣列中省略 `Agent` 或將其添加到 `disallowedTools`。有關完整的深度規則，請參閱[嵌套子代理](/docs/zh-TW/sub-agents#spawn-nested-subagents)。
</Note>

<h3 id="filesystem-based-definition-alternative">
  基於檔案系統的定義（替代方案）
</h3>

您也可以在 `.claude/agents/` 目錄中將子代理定義為 markdown 檔案。有關此方法的詳細信息，請參閱 [Claude Code 子代理文檔](/docs/zh-TW/sub-agents)。以程式方式定義的代理優先於具有相同名稱的基於檔案系統的代理。

<Note>
  即使不定義自訂子代理，Claude 也可以生成內置的 `general-purpose` 子代理。這對於委派研究或探索任務而無需創建專門代理很有用。請在 `allowedTools` 中包含 `Agent`，以便這些調用自動批准，無需權限提示。
</Note>

<h2 id="what-subagents-inherit">
  子代理繼承的內容
</h2>

子代理的上下文窗口從新開始（無父對話），但並非空的。從父代理到子代理的唯一通道是 Agent 工具的提示詞字符串，因此請直接在該提示詞中包含子代理需要的任何檔案路徑、錯誤消息或決策。

具有 [`SendMessage`](/docs/zh-TW/tools-reference) 工具的子代理會在開始時獲得在該會話中運行的其他命名代理的列表，因此它知道可以向哪些名稱發送消息。Claude Code 會自動在子代理的第一輪中添加該列表。[分叉](/docs/zh-TW/sub-agents#fork-the-current-conversation)不會獲得該列表，因為它繼承了父對話。該列表需要 Claude Code v2.1.206 或更高版本。

| 子代理接收                                                                                                                         | 子代理不接收                                    |
| :---------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------- |
| 其自己的系統提示詞（`AgentDefinition.prompt`）和 Agent 工具的提示詞                                                                             | 父代理的對話歷史或工具結果                             |
| 項目 CLAUDE.md（通過 [`settingSources`](/docs/zh-TW/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) 加載） | 預加載的技能內容，除非在 `AgentDefinition.skills` 中列出 |
| 工具定義（從父代理繼承，或 `tools` 中的子集）                                                                                                   | 父代理的系統提示詞                                 |

<Note>
  父代理逐字接收子代理的最終消息作為 Agent 工具結果，但可能在其自己的回應中進行摘要。要在面向用戶的回應中逐字保留子代理輸出，請在您傳遞給主 `query()` 調用的提示詞或 `systemPrompt` 選項中包含執行此操作的指令。
</Note>

結束子代理早期的 API 錯誤（例如速率限制）永遠不會作為其結果傳遞。如果速率限制、過載或伺服器錯誤中斷已經產生文本輸出的前景子代理，Agent 工具會返回該部分輸出並附註子代理未完成。未產生任何內容的子代理，或其唯一輸出是沒有文本的工具調用，會失敗並顯示錯誤消息 `Agent terminated early due to an API error`，後跟錯誤詳情。請參閱[子代理中的 API 錯誤](/docs/zh-TW/sub-agents#api-errors-in-subagents)以了解前景和背景行為。

此部分輸出處理需要 Claude Code v2.1.199 或更高版本。在 v2.1.199 中，速率限制、過載或伺服器錯誤使僅工具調用的形狀保留為空的部分結果，僅包含截斷注釋。

<h2 id="invoke-subagents">
  調用子代理
</h2>

<h3 id="automatic-invocation">
  自動調用
</h3>

Claude 根據任務和每個子代理的 `description` 自動決定何時調用子代理。例如，如果您定義了一個 `performance-optimizer` 子代理，其描述為「查詢調優的性能優化專家」，當您的提示詞提到優化查詢時，Claude 將調用它。

編寫清晰、具體的描述，以便 Claude 可以將任務匹配到正確的子代理。

<h3 id="explicit-invocation">
  明確調用
</h3>

要保證 Claude 使用特定的子代理，請在提示詞中按名稱提及它：

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

這繞過自動匹配並直接調用命名的子代理。

<h3 id="dynamic-agent-configuration">
  動態代理配置
</h3>

您可以根據運行時條件動態創建代理定義。此示例創建一個安全審查員，具有不同的嚴格級別，對嚴格審查使用更強大的模型。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  檢測子代理調用
</h2>

Claude 通過 Agent 工具調用子代理。要檢測何時調用子代理，請檢查 `tool_use` 塊，其中 `name` 是 `"Agent"`。來自子代理上下文內的訊息包括 `parent_tool_use_id` 欄位。

<Note>
  工具名稱在 Claude Code v2.1.63 中從 `"Task"` 重新命名為 `"Agent"`。目前 SDK 版本在 `tool_use` 塊中發出 `"Agent"`，但在 `system:init` 工具清單和 `result.permission_denials[].tool_name` 中仍使用 `"Task"`。檢查 `block.name` 中的兩個值可確保跨 SDK 版本的相容性。
</Note>

訊息結構在 SDK 之間有所不同。在 Python 中，內容塊直接通過 `message.content` 存取。在 TypeScript 中，`SDKAssistantMessage` 包裝 Claude API 訊息，因此內容通過 `message.message.content` 存取。

此範例遍歷串流訊息，記錄何時調用子代理以及後續訊息何時源自該子代理的執行上下文。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  恢復子代理
</h2>

您可以恢復子代理以繼續中斷的地方，而不是從頭開始。恢復的子代理保留其完整的對話歷史，包括所有先前的工具調用、結果和推理。

當子代理完成時，Agent 工具結果包含一個文字區塊，其中包含 `agentId: <id>`。內置的 [`Explore` 和 `Plan` 代理](/docs/zh-TW/sub-agents#built-in-subagents) 是一次性的，不會返回 `agentId`，因此當您需要恢復時，請使用自訂代理或 `general-purpose`。要以程式方式恢復子代理：

1. **捕獲會話 ID**：在第一個查詢期間從訊息中提取 `session_id`
2. **提取代理 ID**：從 Agent 工具結果文字中解析 `agentId`
3. **恢復會話**：在第二個查詢的選項中傳遞 `resume: sessionId`，並在提示詞中包含代理 ID

<Note>
  您必須恢復相同的會話以訪問子代理的記錄。默認情況下，每個 `query()` 調用都會啟動一個新會話，因此傳遞 `resume: sessionId` 以在相同會話中繼續。

  使用自訂代理時，在兩個查詢的 `agents` 參數中傳遞相同的代理定義。
</Note>

下面的示例定義了一個自訂的 `endpoint-finder` 代理。第一個查詢運行它並從 Agent 工具結果中捕獲會話 ID 和代理 ID，然後第二個查詢恢復會話以提出需要來自第一個分析的上下文的後續問題。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

子代理記錄獨立於主對話而持續存在：

* **主對話壓縮**：當主對話壓縮時，子代理記錄不受影響。它們存儲在單獨的檔案中。
* **會話持久性**：子代理記錄在其會話內持續存在。您可以通過恢復相同會話在重新啟動 Claude Code 後恢復子代理。
* **自動清理**：記錄根據 `cleanupPeriodDays` 設定進行清理，預設為 30 天。

<h2 id="tool-restrictions-2">
  工具限制
</h2>

子代理可以通過 `tools` 欄位具有受限的工具訪問：

* **省略欄位**：代理繼承所有可用工具（預設）
* **指定工具**：代理只能使用列出的工具

此示例創建一個唯讀分析代理，可以檢查代碼但無法修改檔案或運行命令。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  常見工具組合
</h3>

| 使用案例 | 工具                                  | 描述                        |
| :--- | :---------------------------------- | :------------------------ |
| 唯讀分析 | `Read`、`Grep`、`Glob`                | 可以檢查代碼但不能修改或執行            |
| 測試執行 | `Bash`、`Read`、`Grep`                | 可以運行命令並分析輸出               |
| 代碼修改 | `Read`、`Edit`、`Write`、`Grep`、`Glob` | 完整的讀/寫訪問，無需命令執行           |
| 完全訪問 | 所有工具                                | 從父代理繼承所有工具（省略 `tools` 欄位） |

<h2 id="scale-up-with-dynamic-workflows">
  使用動態工作流程進行擴展
</h2>

子代理適用於每轉委派幾個任務。對於協調數十到數百個代理的運行，請使用 `Workflow` 工具，它將編排移到運行時在對話上下文外執行的腳本中。請參閱[動態工作流程](/docs/zh-TW/workflows)以了解工作流程與逐轉子代理委派的區別。

`Workflow` 工具在 TypeScript Agent SDK v0.3.149 及更高版本中可用。在 `allowedTools` 中包含 `Workflow` 以自動批准工作流程運行。工具輸入和輸出架構列在 [TypeScript 參考](/docs/zh-TW/agent-sdk/typescript#workflow)中。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude 不委派給子代理
</h3>

如果 Claude 直接完成任務而不是委派給您的子代理：

* **檢查 Agent 調用已獲批准**：在 `allowedTools` 中包含 `Agent` 以自動批准子代理調用。沒有它，Agent 調用會轉到您的 `canUseTool` 回調，或在 `dontAsk` 模式下被拒絕
* **使用明確提示**：在您的提示詞中按名稱提及子代理，例如「使用代碼審查員代理來...」
* **編寫清晰的描述**：準確解釋何時應使用子代理，以便 Claude 可以適當地匹配任務

<h3 id="filesystem-based-agents-not-loading">
  基於檔案系統的代理未加載
</h3>

Claude Code 監視 `~/.claude/agents/` 和 `.claude/agents/`，並在幾秒內拾取新的或編輯的代理檔案，無需重新啟動。如果定義從未出現，請檢查這些原因：

* **新的 `agents` 目錄**：監視程式僅涵蓋會話啟動時存在的目錄，因此新目錄中的第一個檔案需要會話重新啟動。這是最常見的原因。
* **無效的 frontmatter 或重複的 `name`**：檢查檔案的 YAML，以及現有代理是否已使用該 `name`。
* **`--disable-slash-commands`**：使用此旗標啟動的會話不監視這些目錄，並且始終需要重新啟動以加載新檔案。
* **具有相同名稱的程式化代理**：傳遞給 `query()` 的 `agents` 會覆蓋具有相同名稱的檔案系統代理。

有關檔案格式，請參閱[如何編寫子代理檔案](/docs/zh-TW/sub-agents#write-subagent-files)。

<h3 id="long-prompt-failures-on-windows">
  Windows 上的長提示詞失敗
</h3>

在 Windows 上，具有非常長提示詞的子代理可能因命令行長度限制（8191 個字符）而失敗。保持提示詞簡潔或使用基於檔案系統的代理來處理複雜指令。

<h2 id="related-documentation">
  相關文檔
</h2>

* [Claude Code 子代理](/docs/zh-TW/sub-agents)：包括基於檔案系統定義的全面子代理文檔
* [動態工作流程](/docs/zh-TW/workflows)：從腳本協調許多子代理，用於對話太大的工作
* [SDK 概述](/docs/zh-TW/agent-sdk/overview)：Claude Agent SDK 入門
