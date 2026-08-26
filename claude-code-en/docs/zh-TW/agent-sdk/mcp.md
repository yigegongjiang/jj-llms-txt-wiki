> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 MCP 連接外部工具

> 配置 MCP 伺服器以擴展您的代理程式的外部工具。涵蓋傳輸類型、大型工具集的工具搜尋、身份驗證和錯誤處理。

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) 是一個開放標準，用於將 AI 代理程式連接到外部工具和資料來源。使用 MCP，您的代理程式可以查詢資料庫、與 Slack 和 GitHub 等 API 整合，以及連接到其他服務，而無需編寫自訂工具實現。

MCP 伺服器可以作為本地進程運行、通過 HTTP 連接或直接在您的 SDK 應用程式中執行。

<Note>
  本頁涵蓋 Agent SDK 的 MCP 配置。若要將 MCP 伺服器添加到 Claude Code CLI 以便在每個項目中加載，請參閱 [MCP 安裝範圍](/docs/zh-TW/mcp#mcp-installation-scopes)。
</Note>

<h2 id="quickstart">
  快速開始
</h2>

此示例使用 [HTTP 傳輸](#http%2Fsse-servers) 連接到 [Claude Code 文檔](https://code.claude.com/docs) MCP 伺服器，並使用 [`allowedTools`](#allow-mcp-tools) 與通配符來允許來自伺服器的所有工具。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

代理程式連接到文檔伺服器，搜尋有關 hooks 的信息，並返回結果。

<h2 id="add-an-mcp-server">
  新增 MCP 伺服器
</h2>

您可以在呼叫 `query()` 時在程式碼中設定 MCP 伺服器，或在通過 [`settingSources`](#from-a-config-file) 載入的 `.mcp.json` 檔案中設定。

<h3 id="in-code">
  在程式碼中
</h3>

在 `mcpServers` 選項中直接傳遞 MCP 伺服器：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  從設定檔案
</h3>

在您的專案根目錄建立一個 `.mcp.json` 檔案。當啟用 `project` 設定來源時，該檔案會被選取，這對於預設 `query()` 選項是預設的。如果您明確設定 `settingSources`，請包含 `"project"` 以便載入此檔案：

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="allow-mcp-tools">
  允許 MCP 工具
</h2>

MCP 工具需要明確的權限才能讓 Claude 使用它們。沒有權限，Claude 會看到工具可用，但無法調用它們。

<h3 id="tool-naming-convention">
  工具命名約定
</h3>

MCP 工具遵循命名模式 `mcp__<server-name>__<tool-name>`。例如，名為 `"github"` 的 GitHub 伺服器，其 `list_issues` 工具變為 `mcp__github__list_issues`。

<h3 id="auto-approve-with-allowedtools">
  使用 allowedTools 自動批准
</h3>

使用 `allowedTools` 自動批准特定的 MCP 工具，讓 Claude 可以在沒有權限提示的情況下使用它們：

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: [
      "mcp__github__*", // All tools from the github server
      "mcp__db__query", // Only the query tool from db server
      "mcp__slack__send_message" // Only send_message from slack server
    ]
  }
};
```

通配符 (`*`) 讓您允許來自伺服器的所有工具，而無需逐個列出。

<Note>
  **對於 MCP 訪問，優先使用 `allowedTools` 而不是權限模式。** `permissionMode: "acceptEdits"` 不會自動批准 MCP 工具（僅限文件編輯和文件系統 Bash 命令）。`permissionMode: "bypassPermissions"` 會自動批准 MCP 工具，但也會禁用大多數其他安全提示，這比必要的範圍更廣；請參閱[權限如何被評估](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)以了解保留的提示。`allowedTools` 中的通配符只授予您想要的 MCP 伺服器，沒有其他。有關完整比較，請參閱[權限模式](/docs/zh-TW/agent-sdk/permissions#permission-modes)。
</Note>

<h3 id="discover-available-tools">
  發現可用工具
</h3>

要查看 MCP 伺服器提供的工具，請檢查伺服器的文檔或連接到伺服器並檢查 `system` init 消息：

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available MCP tools:", message.mcp_servers);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, SystemMessage


  async def main():
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available MCP tools:", message.data["mcp_servers"])


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="transport-types">
  傳輸類型
</h2>

MCP 伺服器使用不同的傳輸協議與您的代理程式通信。檢查伺服器的文檔以查看它支持哪種傳輸：

* 如果文檔給您一個**要運行的命令**（如 `npx @modelcontextprotocol/server-github`），請使用 stdio
* 如果文檔給您一個 **URL**，請使用 HTTP 或 SSE
* 如果您在代碼中構建自己的工具，請使用 SDK MCP 伺服器

<h3 id="stdio-servers">
  stdio 伺服器
</h3>

通過 stdin/stdout 通信的本地進程。對於在同一台機器上運行的 MCP 伺服器，請使用此方法：

<Tabs>
  <Tab title="In code">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues", "mcp__github__search_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues", "mcp__github__search_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

<h3 id="http/sse-servers">
  HTTP/SSE 伺服器
</h3>

對於雲託管的 MCP 伺服器和遠程 API，請使用 HTTP 或 SSE：

<Tabs>
  <Tab title="In code">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "remote-api": {
              type: "sse",
              url: "https://api.example.com/mcp/sse",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__remote-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "remote-api": {
                  "type": "sse",
                  "url": "https://api.example.com/mcp/sse",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__remote-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "remote-api": {
          "type": "sse",
          "url": "https://api.example.com/mcp/sse",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

對於可流式傳輸的 HTTP 傳輸，請改用 `"type": "http"`。在 `.mcp.json` 和其他 JSON 配置文件中，`"streamable-http"` 被接受作為 `"http"` 的別名。程式化的 `mcpServers` 選項僅接受 `"http"`。

<h3 id="sdk-mcp-servers">
  SDK MCP 伺服器
</h3>

直接在應用程式代碼中定義自訂工具，而不是運行單獨的伺服器進程。有關實現詳情，請參閱[自訂工具指南](/docs/zh-TW/agent-sdk/custom-tools)。

<h2 id="mcp-tool-search">
  MCP 工具搜尋
</h2>

當您配置了許多 MCP 工具時，工具定義可能會消耗上下文窗口的很大一部分。工具搜尋通過從上下文中隱藏工具定義並僅加載 Claude 每次轉換所需的工具來解決此問題。

工具搜尋默認啟用。有關配置選項和詳情，請參閱[工具搜尋](/docs/zh-TW/agent-sdk/tool-search)。

有關更多詳情，包括最佳實踐和將工具搜尋與自訂 SDK 工具一起使用，請參閱[工具搜尋指南](/docs/zh-TW/agent-sdk/tool-search)。

<h2 id="authentication">
  身份驗證
</h2>

大多數 MCP 伺服器需要身份驗證才能訪問外部服務。通過伺服器配置中的環境變數傳遞憑據。

<h3 id="pass-credentials-via-environment-variables">
  通過環境變數傳遞憑據
</h3>

使用 `env` 字段將 API 密鑰、令牌和其他憑據傳遞給 MCP 伺服器：

<Tabs>
  <Tab title="In code">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```

    `${GITHUB_TOKEN}` 語法在運行時展開環境變數。
  </Tab>
</Tabs>

有關帶有調試日誌的完整工作示例，請參閱[從存儲庫列出問題](#list-issues-from-a-repository)。

<h3 id="http-headers-for-remote-servers">
  遠程伺服器的 HTTP 標頭
</h3>

對於 HTTP 和 SSE 伺服器，直接在伺服器配置中傳遞身份驗證標頭：

<Tabs>
  <Tab title="In code">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    `${API_TOKEN}` 語法在運行時展開環境變數。
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  OAuth2 身份驗證
</h3>

[MCP 規範支持 OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) 用於授權。SDK 不會開啟瀏覽器或運行互動式 OAuth 流程。當配置的伺服器返回授權質詢且沒有可用的儲存令牌時，代理運行會在沒有該伺服器工具的情況下繼續，並且伺服器在[系統初始化訊息](/docs/zh-TW/agent-sdk/typescript#sdksystemmessage)的 `mcp_servers` 陣列中報告狀態為 `needs-auth`。如果您的代理依賴於特定伺服器的連接，請在啟動時檢查該陣列。

要提供憑據，請在您自己的應用程式中完成 OAuth 流程，並在伺服器的 `headers` 中傳遞生成的訪問令牌：

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # After completing OAuth flow in your app
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  示例
</h2>

<h3 id="list-issues-from-a-repository">
  從存儲庫列出問題
</h3>

此示例連接到 [GitHub MCP 伺服器](https://github.com/modelcontextprotocol/servers/tree/main/src/github) 以列出最近的問題。該示例包括調試日誌以驗證 MCP 連接和工具調用。

運行前，創建一個具有 `repo` 範圍的 [GitHub 個人訪問令牌](https://github.com/settings/tokens)，並將其設置為環境變數：

```bash theme={null}
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-github"],
          env: {
            GITHUB_TOKEN: process.env.GITHUB_TOKEN
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="query-a-database">
  查詢資料庫
</h3>

此示例使用 [Postgres MCP 伺服器](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) 查詢資料庫。連接字符串作為參數傳遞給伺服器。代理程式自動發現資料庫架構、編寫 SQL 查詢並返回結果：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Connection string from environment variable
  const connectionString = process.env.DATABASE_URL;

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // Pass connection string as argument to the server
          args: ["-y", "@modelcontextprotocol/server-postgres", connectionString]
        }
      },
      // Allow only read queries, not writes
      allowedTools: ["mcp__postgres__query"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      # Connection string from environment variable
      connection_string = os.environ["DATABASE_URL"]

      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # Pass connection string as argument to the server
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-postgres",
                      connection_string,
                  ],
              }
          },
          # Allow only read queries, not writes
          allowed_tools=["mcp__postgres__query"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  錯誤處理
</h2>

MCP 伺服器可能因各種原因無法連接：伺服器進程可能未安裝、憑據可能無效，或遠程伺服器可能無法訪問。

SDK 在每個查詢開始時發出一個 `system` 訊息，子類型為 `init`。此訊息包括每個 MCP 伺服器的連接狀態。檢查 `status` 欄位以在代理程式開始工作前檢測連接失敗：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Process data",
    options: {
      mcpServers: {
        "data-processor": dataServer
      }
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      const failedServers = message.mcp_servers.filter((s) => s.status !== "connected");

      if (failedServers.length > 0) {
        console.warn("Failed to connect:", failedServers);
      }
    }

    if (message.type === "result" && message.subtype === "error_during_execution") {
      console.error("Execution failed");
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      async for message in query(prompt="Process data", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              failed_servers = [
                  s
                  for s in message.data.get("mcp_servers", [])
                  if s.get("status") != "connected"
              ]

              if failed_servers:
                  print(f"Failed to connect: {failed_servers}")

          if (
              isinstance(message, ResultMessage)
              and message.subtype == "error_during_execution"
          ):
              print("Execution failed")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="server-shows-failed-status">
  伺服器顯示「失敗」狀態
</h3>

檢查 `init` 訊息以查看哪些伺服器無法連接：

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

常見原因：

* **缺少環境變數**：確保設置了所需的令牌和憑證。對於 stdio 伺服器，檢查 `env` 欄位是否與伺服器期望的相符。
* **伺服器未安裝**：對於 `npx` 命令，驗證套件存在且 Node.js 在您的 PATH 中。
* **無效的連接字串**：對於資料庫伺服器，驗證連接字串格式以及資料庫是否可存取。
* **網路問題**：對於遠端 HTTP/SSE 伺服器，檢查 URL 是否可存取以及任何防火牆是否允許連接。

<h3 id="tools-not-being-called">
  工具未被呼叫
</h3>

如果 Claude 看到工具但不使用它們，請檢查您是否已使用 `allowedTools` 授予權限：

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: ["mcp__servername__*"] // 自動核准來自此伺服器的呼叫
  }
};
```

<h3 id="connection-timeouts">
  連接逾時
</h3>

MCP 伺服器連接預設逾時為 30 秒。如果您的伺服器需要更長時間才能啟動，連接將失敗。使用 [`MCP_TIMEOUT`](/docs/zh-TW/env-vars) 環境變數（以毫秒為單位）提高限制。對於需要更多啟動時間的伺服器，也請考慮：

* 使用更輕量級的伺服器（如果可用）
* 在啟動代理程式前預熱伺服器
* 檢查伺服器日誌以查找緩慢初始化的原因

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  工具輸出超過允許的最大令牌數
</h3>

SDK 應用與 Claude Code 相同的 MCP 輸出限制。當工具結果大於 25,000 個令牌時，完整輸出會儲存到檔案中，工具結果會被替換為錯誤訊息，該訊息會指出檔案路徑，以便代理程式可以分次讀取輸出。使用 [`MAX_MCP_OUTPUT_TOKENS`](/docs/zh-TW/env-vars) 環境變數提高限制。請參閱 [MCP 輸出限制和警告](/docs/zh-TW/mcp#mcp-output-limits-and-warnings) 以了解完整行為，包括伺服器如何聲明更高的每個工具限制。

<h2 id="related-resources">
  相關資源
</h2>

* **[自訂工具指南](/docs/zh-TW/agent-sdk/custom-tools)**：構建您自己的 MCP 伺服器，與您的 SDK 應用程式在進程中運行
* **[權限](/docs/zh-TW/agent-sdk/permissions)**：使用 `allowedTools` 和 `disallowedTools` 控制您的代理程式可以使用哪些 MCP 工具
* **[MCP 輸出限制和警告](/docs/zh-TW/mcp#mcp-output-limits-and-warnings)**：SDK 如何處理超過 `MAX_MCP_OUTPUT_TOKENS` 的工具結果，包括持久化到磁碟的備用方案和 `anthropic/maxResultSizeChars` 每個工具的註解
* **[TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)**：完整的 API 參考，包括 MCP 配置選項
* **[Python SDK 參考](/docs/zh-TW/agent-sdk/python)**：完整的 API 參考，包括 MCP 配置選項
* **[MCP 伺服器目錄](https://github.com/modelcontextprotocol/servers)**：瀏覽可用的 MCP 伺服器，用於資料庫、API 等
