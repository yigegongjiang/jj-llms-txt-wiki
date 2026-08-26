> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 快速開始

> 使用 Python 或 TypeScript Agent SDK 開始構建能夠自主工作的 AI 代理

使用 Agent SDK 構建一個 AI 代理，它可以讀取您的代碼、發現錯誤並自動修復它們，無需手動干預。

**您將執行的操作：**

1. 使用 Agent SDK 設置項目
2. 創建一個包含一些有缺陷代碼的文件
3. 運行一個代理，自動查找並修復錯誤

<h2 id="prerequisites">
  先決條件
</h2>

* **Node.js 18+** 或 **Python 3.10+**
* 一個 **Anthropic 帳戶**（[在此註冊](https://platform.claude.com/)）

<h2 id="setup">
  設置
</h2>

<Steps>
  <Step title="創建項目文件夾">
    為此快速開始創建一個新目錄：

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    對於您自己的項目，您可以從任何文件夾運行 SDK；默認情況下，它將有權訪問該目錄及其子目錄中的文件。
  </Step>

  <Step title="安裝 SDK">
    為您的語言安裝 Agent SDK 套件：

    <Tabs>
      <Tab title="TypeScript（新項目）">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        在 `package.json` 中設置 `"type": "module"` 讓您的代理腳本使用頂級 `await`，而 [tsx](https://tsx.is) 直接運行 TypeScript 文件。
      </Tab>

      <Tab title="TypeScript（現有項目）">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) 直接運行 TypeScript 文件。如果您的項目使用 CommonJS，請將您的代理腳本命名為 `agent.mts` 而不是 `agent.ts`。`.mts` 擴展名使 tsx 將文件視為 ES 模塊，因此頂級 `await` 無需將整個項目轉換為 ES 模塊即可工作。在本快速開始後面的創建和運行步驟中，使用 `agent.mts` 代替 `agent.ts`。
      </Tab>

      <Tab title="Python（uv）">
        [uv](https://docs.astral.sh/uv/) 是一個快速的 Python 套件管理器，可自動處理虛擬環境：

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python（pip）">
        創建並啟用虛擬環境，然後安裝該套件。

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

        如果 PowerShell 因執行策略錯誤而阻止 `Activate.ps1`，請先運行 `Set-ExecutionPolicy -Scope Process RemoteSigned`。
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK 為您的平台捆綁了一個原生 Claude Code 二進制文件作為可選依賴項，因此您無需單獨安裝 Claude Code。
    </Note>
  </Step>

  <Step title="設置您的 API 金鑰">
    從 [Claude 控制台](https://platform.claude.com/) 取得 API 金鑰，然後在您將運行代理的 shell 中將其設置為環境變數：

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows（PowerShell）">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    SDK 從運行您的代理的進程環境中讀取金鑰；它不會自動加載 `.env` 文件。如果您將金鑰保存在 `.env` 文件中，請自己加載它，例如使用 `dotenv` 套件，然後再調用 SDK。

    SDK 也支援透過第三方 API 提供商進行身份驗證：

    * **Amazon Bedrock**：設定 `CLAUDE_CODE_USE_BEDROCK=1` 環境變數並設定 AWS 認證
    * **Claude Platform on AWS**：設定 `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` 和 `ANTHROPIC_AWS_WORKSPACE_ID`，然後設定 AWS 認證
    * **Google Cloud 的 Agent Platform**：設定 `CLAUDE_CODE_USE_VERTEX=1` 環境變數並設定 Google Cloud 認證
    * **Microsoft Azure**：設定 `CLAUDE_CODE_USE_FOUNDRY=1` 環境變數並設定 Azure 認證

    如需詳細資訊，請參閱 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai) 或 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 的設置指南。

    <Note>
      除非事先獲得批准，否則 Anthropic 不允許第三方開發人員提供 claude.ai 登入或對其產品的速率限制，包括基於 Claude Agent SDK 構建的代理。請改用本文件中描述的 API 金鑰身份驗證方法。
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  創建有缺陷的文件
</h2>

此快速開始將引導您構建一個可以查找並修復代碼中的錯誤的代理。首先，您需要一個包含一些故意錯誤的文件供代理修復。在 `my-agent` 目錄中創建 `utils.py` 並粘貼以下代碼：

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

此代碼有兩個錯誤：

1. `calculate_average([])` 因除以零而崩潰
2. `get_user_name(None)` 因 TypeError 而崩潰

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  構建查找並修復錯誤的代理
</h2>

如果您使用 Python SDK，請創建 `agent.py`，或者如果使用 TypeScript，請創建 `agent.ts`。如果您現有的項目使用 CommonJS，請改用 `agent.mts`：

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

此代碼有三個主要部分：

1. **`query`**：創建 agentic 循環的主要入口點。它返回一個異步迭代器，因此您使用 `async for` 在 Claude 工作時流式傳輸消息。請參閱 [Python](/docs/zh-TW/agent-sdk/python#query) 或 [TypeScript](/docs/zh-TW/agent-sdk/typescript#query) SDK 參考中的完整 API。

2. **`prompt`**：您希望 Claude 執行的操作。Claude 根據任務確定要使用哪些工具。

3. **`options`**：代理的配置。此示例使用 `allowedTools` 預先批准 `Read`、`Edit` 和 `Glob`，並使用 `permissionMode: "acceptEdits"` 自動批准文件更改。其他選項包括 `systemPrompt`、`mcpServers` 等。請參閱 [Python](/docs/zh-TW/agent-sdk/python#claudeagentoptions) 或 [TypeScript](/docs/zh-TW/agent-sdk/typescript#options) 的所有選項。

`async for` 循環在 Claude 思考、調用工具、觀察結果並決定下一步操作時持續運行。每次迭代都會產生一條消息：Claude 的推理、工具調用、工具結果或最終結果。SDK 處理編排（工具執行、上下文管理、重試），因此您只需使用流。當 Claude 完成任務或遇到錯誤時，循環結束。

循環內的消息處理會過濾人類可讀的輸出。如果沒有過濾，您會看到原始消息對象，包括系統初始化和內部狀態，這對於調試很有用，但通常很冗長。

<Note>
  此示例使用流式傳輸來實時顯示進度。如果您不需要實時輸出（例如，對於後台作業或 CI 管道），您可以一次收集所有消息。有關詳細信息，請參閱[流式傳輸與單輪模式](/docs/zh-TW/agent-sdk/streaming-vs-single-mode)。
</Note>

<h3 id="run-your-agent">
  運行您的代理
</h3>

您的代理已準備好。使用以下命令運行它：

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    如果您將腳本命名為 `agent.mts`，請改為運行 `npx tsx agent.mts`。
  </Tab>

  <Tab title="Python（uv）">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python（pip）">
    使用您仍然激活的虛擬環境：

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

運行後，檢查 `utils.py`。您將看到處理空列表和空用戶的防禦性代碼。您的代理自主地：

1. **讀取** `utils.py` 以理解代碼
2. **分析**邏輯並識別會導致崩潰的邊界情況
3. **編輯**文件以添加適當的錯誤處理

這就是 Agent SDK 的不同之處：Claude 直接執行工具，而不是要求您實現它們。

<Note>
  如果您看到'API key not found'，請確保您已在運行代理的 shell 中設置 `ANTHROPIC_API_KEY` 環境變數。SDK 不會自動加載 `.env` 文件。如需更多幫助，請參閱[完整故障排除指南](/docs/zh-TW/troubleshooting)。
</Note>

<h3 id="try-other-prompts">
  嘗試其他提示
</h3>

現在您的代理已設置好，嘗試一些不同的提示：

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  自定義您的代理
</h3>

您可以通過更改選項來修改代理的行為。以下是一些示例：

**添加網絡搜索功能：**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**給 Claude 一個自定義系統提示：**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**在終端中運行命令：**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

啟用 `Bash` 後，嘗試：`"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  關鍵概念
</h2>

**工具**控制您的代理可以執行的操作：

| 工具                                 | 代理可以執行的操作 |
| ---------------------------------- | --------- |
| `Read`、`Glob`、`Grep`               | 只讀分析      |
| `Read`、`Edit`、`Glob`               | 分析和修改代碼   |
| `Read`、`Edit`、`Bash`、`Glob`、`Grep` | 完全自動化     |

**權限模式**控制您想要多少人工監督：

| 模式                  | 行為                                                                                                                                                                               | 用例             |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| `acceptEdits`       | 自動批准文件編輯和常見文件系統命令，詢問其他操作                                                                                                                                                         | 受信任的開發工作流      |
| `plan`              | 運行只讀工具；文件編輯永遠不會自動批准，並到達您的 `canUseTool` 回調                                                                                                                                        | 在批准執行前確定任務範圍   |
| `dontAsk`           | 拒絕不在 `allowedTools` 中的任何內容；連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要用戶互動的工具即使您已列出它們也會被拒絕                                                      | 鎖定的無頭代理        |
| `auto`              | 模型分類器批准或拒絕每個工具調用                                                                                                                                                                 | 具有安全防護的自主代理    |
| `bypassPermissions` | 運行每個工具而不提示，除了明確的 [`ask` 規則](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)符合的工具、連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要用戶互動的工具 | 沙箱 CI、完全受信任的環境 |
| `default`           | 需要 `canUseTool` 回調來處理批准                                                                                                                                                          | 自定義批准流程        |

上面的示例使用 `acceptEdits` 模式，它自動批准文件操作，以便代理可以無需交互提示地運行。如果您想提示用戶批准，請使用 `default` 模式並提供一個 [`canUseTool` 回調](/docs/zh-TW/agent-sdk/user-input)來收集用戶輸入。如需更多控制，請參閱[權限](/docs/zh-TW/agent-sdk/permissions)。

<h2 id="next-steps">
  後續步驟
</h2>

現在您已創建了第一個代理，了解如何擴展其功能並根據您的用例進行定制：

* **[權限](/docs/zh-TW/agent-sdk/permissions)**：控制您的代理可以執行的操作以及何時需要批准
* **[Hooks](/docs/zh-TW/agent-sdk/hooks)**：在工具調用之前或之後運行自定義代碼
* **[會話](/docs/zh-TW/agent-sdk/sessions)**：構建維護上下文的多輪代理
* **[MCP 服務器](/docs/zh-TW/agent-sdk/mcp)**：連接到數據庫、瀏覽器、API 和其他外部系統
* **[託管](/docs/zh-TW/agent-sdk/hosting)**：將代理部署到 Docker、雲和 CI/CD
* **[示例代理](https://github.com/anthropics/claude-agent-sdk-demos)**：查看完整示例：電子郵件助手、研究代理等
