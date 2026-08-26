> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 概述

> 使用 Claude Code 作為程式庫構建生產級 AI 代理

構建能夠自主讀取檔案、執行命令、搜尋網路、編輯程式碼等的 AI 代理。Agent SDK 提供與 Claude Code 相同的工具、代理迴圈和上下文管理，可在 Python 和 TypeScript 中進行程式設計。如需了解代理工具設計背後的思考，請參閱部落格上的 [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code)。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

Agent SDK 包含用於讀取檔案、執行命令和編輯程式碼的內建工具，因此您的代理可以立即開始工作，無需您實現工具執行。深入了解快速入門或探索使用 SDK 構建的真實代理：

<CardGroup cols={2}>
  <Card title="快速入門" icon="play" href="/docs/zh-TW/agent-sdk/quickstart">
    在幾分鐘內構建一個除錯代理
  </Card>

  <Card title="範例代理" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    電子郵件助手、研究代理等
  </Card>
</CardGroup>

<h2 id="get-started">
  開始使用
</h2>

<Steps>
  <Step title="安裝 SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) 是一個快速的 Python 套件管理器，可自動處理虛擬環境：

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        建立並啟動虛擬環境，然後安裝套件。安裝到虛擬環境可避免在最近的 Debian、Ubuntu 和 Homebrew 安裝上執行 `pip install` 時系統 Python 返回的 `error: externally-managed-environment` 失敗。

        在 macOS 或 Linux 上：

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        在 Windows 上：

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        如果 PowerShell 因執行原則錯誤而阻止 `Activate.ps1`，請先執行 `Set-ExecutionPolicy -Scope Process RemoteSigned`。

        Python 套件需要 Python 3.10 或更新版本。如果 pip 報告 `No matching distribution found for claude-agent-sdk`，表示您的直譯器版本早於 3.10。在 macOS 或 Linux 上執行 `python3 --version`，或在 Windows 上執行 `py --version`，以檢查版本。
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK 為您的平台捆綁了原生 Claude Code 二進位檔案作為可選依賴項，因此您無需單獨安裝 Claude Code。
    </Note>
  </Step>

  <Step title="設定您的 API 金鑰">
    從[主控台](https://platform.claude.com/)取得 API 金鑰，然後將其設定為環境變數。

    在 macOS 或 Linux 上：

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    在 Windows PowerShell 上：

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    SDK 也支援透過第三方 API 提供者進行身份驗證：

    * **Amazon Bedrock**：設定 `CLAUDE_CODE_USE_BEDROCK=1` 環境變數並配置 AWS 認證
    * **Claude Platform on AWS**：設定 `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` 和 `ANTHROPIC_AWS_WORKSPACE_ID`，然後配置 AWS 認證
    * **Google Cloud's Agent Platform**：設定 `CLAUDE_CODE_USE_VERTEX=1` 環境變數並配置 Google Cloud 認證
    * **Microsoft Azure**：設定 `CLAUDE_CODE_USE_FOUNDRY=1` 環境變數並配置 Azure 認證

    有關詳細資訊，請參閱 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws)、[Google Cloud's Agent Platform](/docs/zh-TW/google-vertex-ai) 或 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 的設定指南。

    <Note>
      除非事先獲得批准，否則 Anthropic 不允許第三方開發人員為其產品（包括基於 Claude Agent SDK 構建的代理）提供 claude.ai 登入或速率限制。請改用本文件中描述的 API 金鑰身份驗證方法。
    </Note>
  </Step>

  <Step title="執行您的第一個代理">
    此範例建立一個使用內建工具列出目前目錄中檔案的代理。

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**準備好構建了嗎？** 遵循[快速入門](/docs/zh-TW/agent-sdk/quickstart)在幾分鐘內建立一個尋找和修復錯誤的代理。

<h2 id="capabilities">
  功能
</h2>

使 Claude Code 強大的一切都可在 SDK 中使用：

<Tabs>
  <Tab title="內建工具">
    您的代理可以開箱即用地讀取檔案、執行命令和搜尋程式碼庫。主要工具包括：

    | 工具                                                                             | 功能                               |
    | ------------------------------------------------------------------------------ | -------------------------------- |
    | **Read**                                                                       | 讀取工作目錄中的任何檔案                     |
    | **Write**                                                                      | 建立新檔案                            |
    | **Edit**                                                                       | 對現有檔案進行精確編輯                      |
    | **Bash**                                                                       | 執行終端命令、指令碼、git 操作                |
    | **Monitor**                                                                    | 監視背景指令碼並對每個輸出行作為事件做出反應           |
    | **Glob**                                                                       | 按模式尋找檔案（`**/*.ts`、`src/**/*.py`） |
    | **Grep**                                                                       | 使用正規表達式搜尋檔案內容                    |
    | **WebSearch**                                                                  | 搜尋網路以獲取最新資訊                      |
    | **WebFetch**                                                                   | 擷取並解析網頁內容                        |
    | **[AskUserQuestion](/docs/zh-TW/agent-sdk/user-input#handle-clarifying-questions)** | 向使用者提出具有多選選項的澄清問題                |

    此範例建立一個搜尋程式碼庫中 TODO 註解的代理：

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Hooks">
    在代理生命週期的關鍵點執行自訂程式碼。SDK hooks 使用回呼函式來驗證、記錄、阻止或轉換代理行為。

    **可用 hooks：** `PreToolUse`、`PostToolUse`、`Stop`、`SessionStart`、`SessionEnd`、`UserPromptSubmit` 等。

    此範例將所有檔案變更記錄到稽核檔案：

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [深入了解 hooks →](/docs/zh-TW/agent-sdk/hooks)
  </Tab>

  <Tab title="子代理">
    生成專門的代理來處理集中的子任務。您的主代理委派工作，子代理報告結果。

    定義具有專門指令的自訂代理。子代理透過 Agent 工具呼叫，因此在 `allowedTools` 中包含 `Agent` 以自動批准這些呼叫：

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
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
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    來自子代理上下文內的訊息包含 `parent_tool_use_id` 欄位，讓您追蹤哪些訊息屬於哪個子代理執行。

    [深入了解子代理 →](/docs/zh-TW/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    透過 Model Context Protocol 連接到外部系統：資料庫、瀏覽器、API 和[數百個更多](https://github.com/modelcontextprotocol/servers)。

    此範例連接 [Playwright MCP 伺服器](https://github.com/microsoft/playwright-mcp)以為您的代理提供瀏覽器自動化功能：

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [深入了解 MCP →](/docs/zh-TW/agent-sdk/mcp)
  </Tab>

  <Tab title="權限">
    精確控制您的代理可以使用哪些工具。允許安全操作、阻止危險操作或要求對敏感操作進行批准。

    <Note>
      有關互動式批准提示和 `AskUserQuestion` 工具，請參閱[處理批准和使用者輸入](/docs/zh-TW/agent-sdk/user-input)。
    </Note>

    此範例建立一個唯讀代理，可以分析但不能修改程式碼。`allowed_tools` 預先批准 `Read`、`Glob` 和 `Grep`。

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [深入了解權限 →](/docs/zh-TW/agent-sdk/permissions)
  </Tab>

  <Tab title="工作階段">
    在多次交換中保持上下文。Claude 記住讀取的檔案、完成的分析和對話歷史。稍後恢復工作階段，或分叉它們以探索不同的方法。

    此範例從第一個查詢中擷取工作階段 ID，然後恢復以繼續進行完整上下文：

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [深入了解工作階段 →](/docs/zh-TW/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Claude Code 功能
</h3>

SDK 也支援 Claude Code 的基於檔案系統的設定。使用預設選項，SDK 從工作目錄中的 `.claude/` 和 `~/.claude/` 載入這些。要限制載入哪些來源，請在選項中設定 `setting_sources`（Python）或 `settingSources`（TypeScript）。

| 功能                                                  | 描述                               | 位置                                |
| --------------------------------------------------- | -------------------------------- | --------------------------------- |
| [Skills](/docs/zh-TW/agent-sdk/skills)                   | Claude 自動使用或您使用 `/name` 呼叫的專門功能  | `.claude/skills/*/SKILL.md`       |
| [Commands](/docs/zh-TW/agent-sdk/slash-commands)         | 舊版格式的自訂命令。新的自訂命令請使用 skills       | `.claude/commands/*.md`           |
| [Memory](/docs/zh-TW/agent-sdk/modifying-system-prompts) | 專案上下文和指令                         | `CLAUDE.md` 或 `.claude/CLAUDE.md` |
| [Plugins](/docs/zh-TW/agent-sdk/plugins)                 | 使用 skills、代理、hooks 和 MCP 伺服器進行擴展 | 透過 `plugins` 選項進行程式設計             |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  將 Agent SDK 與其他 Claude 工具進行比較
</h2>

Claude 平台提供多種方式來使用 Claude 進行構建。以下是 Agent SDK 的適用方式：

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    [Anthropic Client SDK](https://platform.claude.com/docs/zh-TW/api/client-sdks) 為您提供直接 API 存取：您傳送提示並自己實現工具執行。**Agent SDK** 為您提供具有內建工具執行的 Claude。

    使用 Client SDK，您實現工具迴圈。使用 Agent SDK，Claude 處理它：

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    相同的功能，不同的介面：

    | 使用案例     | 最佳選擇 |
    | -------- | ---- |
    | 互動式開發    | CLI  |
    | CI/CD 管道 | SDK  |
    | 自訂應用程式   | SDK  |
    | 一次性任務    | CLI  |
    | 生產自動化    | SDK  |

    許多團隊同時使用兩者：CLI 用於日常開發，SDK 用於生產。工作流程在它們之間直接轉換。
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/zh-TW/managed-agents/overview) 是一個託管的 REST API：Anthropic 執行代理和沙箱，您的應用程式傳送事件並串流回結果。**Agent SDK** 是一個在您自己的流程內執行代理迴圈的程式庫。

    |            | Agent SDK                  | Managed Agents                    |
    | ---------- | -------------------------- | --------------------------------- |
    | **執行位置**   | 您的流程、您的基礎設施                | Anthropic 管理的基礎設施                 |
    | **介面**     | Python 或 TypeScript 程式庫    | REST API                          |
    | **代理工作於**  | 您基礎設施上的檔案                  | 每個工作階段的託管沙箱                       |
    | **工作階段狀態** | 您檔案系統上的 JSONL              | Anthropic 託管的事件日誌                 |
    | **自訂工具**   | 進程內 Python 或 TypeScript 函數 | Claude 觸發工具；您執行並返回結果              |
    | **最適合**    | 本地原型設計、直接在您的檔案系統和服務上工作的代理  | 生產代理，無需操作沙箱或工作階段基礎設施、長期執行和非同步工作階段 |

    常見的路徑是先使用 Agent SDK 在本地進行原型設計，然後移至 Managed Agents 進行生產。
  </Tab>
</Tabs>

<h2 id="changelog">
  變更日誌
</h2>

查看完整的變更日誌以了解 SDK 更新、錯誤修復和新功能：

* **TypeScript SDK**：[檢視 CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**：[檢視 CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  報告錯誤
</h2>

如果您遇到 Agent SDK 的錯誤或問題：

* **TypeScript SDK**：[在 GitHub 上報告問題](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**：[在 GitHub 上報告問題](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  品牌指南
</h2>

對於整合 Claude Agent SDK 的合作夥伴，使用 Claude 品牌是可選的。在您的產品中引用 Claude 時：

**允許：**

* "Claude Agent"（下拉選單的首選）
* "Claude"（當已在標記為"Agents"的選單中時）
* "{YourAgentName} Powered by Claude"（如果您有現有的代理名稱）

**不允許：**

* "Claude Code" 或 "Claude Code Agent"
* Claude Code 品牌的 ASCII 藝術或模仿 Claude Code 的視覺元素

您的產品應保持自己的品牌，不應顯示為 Claude Code 或任何 Anthropic 產品。有關品牌合規性的問題，請聯絡 Anthropic [銷售團隊](https://www.anthropic.com/contact-sales)。

<h2 id="license-and-terms">
  許可證和條款
</h2>

Claude Agent SDK 的使用受 [Anthropic 商業服務條款](https://www.anthropic.com/legal/commercial-terms)管制，包括當您使用它為您自己的客戶和最終使用者提供的產品和服務提供動力時，除非特定元件或依賴項受到該元件 LICENSE 檔案中指示的不同許可證的保護。

<h2 id="next-steps">
  後續步驟
</h2>

<CardGroup cols={2}>
  <Card title="快速入門" icon="play" href="/docs/zh-TW/agent-sdk/quickstart">
    構建在幾分鐘內尋找和修復錯誤的代理
  </Card>

  <Card title="範例代理" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    電子郵件助手、研究代理等
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/zh-TW/agent-sdk/typescript">
    完整的 TypeScript API 參考和範例
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/zh-TW/agent-sdk/python">
    完整的 Python API 參考和範例
  </Card>
</CardGroup>
