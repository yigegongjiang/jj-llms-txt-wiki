> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用工具搜尋擴展到許多工具

> 通過動態發現和按需加載，將您的代理擴展到數千個工具。

工具搜尋使您的代理能夠通過動態發現和按需加載來處理數百或數千個工具。代理不是將所有工具定義預先加載到上下文窗口中，而是搜尋您的工具目錄並僅加載它需要的工具。

隨著工具庫的擴展，這種方法解決了兩個挑戰：

* **上下文效率：** 工具定義可能會消耗上下文窗口的大部分（50 個工具可能使用 10-20K 個令牌），留下較少的空間用於實際工作。
* **工具選擇準確性：** 同時加載超過 30-50 個工具時，工具選擇準確性會下降。

工具搜尋預設為啟用。

<h2 id="how-tool-search-works">
  工具搜尋的工作原理
</h2>

當工具搜尋處於活動狀態時，工具定義會從上下文窗口中隱藏。代理會收到可用工具的摘要，並在任務需要尚未加載的功能時搜尋相關工具。最相關的五個工具會被加載到上下文中，在後續輪次中保持可用。如果對話足夠長，以至於 SDK 壓縮早期消息以釋放空間，之前發現的工具可能會被移除，代理會根據需要再次搜尋。

工具搜尋在 Claude 首次發現工具時增加一個額外的往返（搜尋步驟），但對於大型工具集，這會被每個輪次上下文較小所抵消。對於少於約 10 個工具的情況，預先加載所有工具通常更快。

有關底層 API 機制的詳細信息，請參閱 [API 中的工具搜尋](https://platform.claude.com/docs/zh-TW/agents-and-tools/tool-use/tool-search-tool)。

<Note>
  工具搜尋在 Claude Sonnet 4.5、Claude Haiku 4.5、Claude Opus 4.5 及更新版本上受支援；請參閱 [API 文件中的模型相容性](https://platform.claude.com/docs/zh-TW/agents-and-tools/tool-use/tool-search-tool#model-compatibility)以取得目前清單。在 Google Cloud 的 Agent Platform 上，最低支援的模型是 Claude Sonnet 4.5 和 Claude Opus 4.5。
</Note>

<h2 id="configure-tool-search">
  配置工具搜尋
</h2>

工具搜尋預設為開啟。在 Google Cloud 的 Agent Platform 上預設為關閉，其中支援 Claude Sonnet 4.5 及更高版本以及 Claude Opus 4.5 及更高版本。當 `ANTHROPIC_BASE_URL` 指向非第一方主機時，它也會被禁用，因為大多數代理不轉發 `tool_reference` 塊。您可以使用 `ENABLE_TOOL_SEARCH` 環境變數覆蓋任一預設值：

| 值        | 行為                                                                                                                                                                  |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| （未設置）    | 工具搜尋已開啟。工具定義被延遲並按需發現。在 Google Cloud 的 Agent Platform 或非第一方 `ANTHROPIC_BASE_URL` 上回退到預先加載。                                                                           |
| `true`   | 工具搜尋始終開啟。SDK 即使在 Google Cloud 的 Agent Platform 和通過代理時也會發送 beta 標頭。在 Sonnet 4.5 或 Opus 4.5 之前的 Google Cloud 的 Agent Platform 模型上，或在不支援 `tool_reference` 塊的代理上，請求會失敗。 |
| `auto`   | 檢查所有工具定義的組合令牌計數與模型的上下文窗口。如果超過 10%，工具搜尋會啟動。如果低於 10%，所有工具會正常加載到上下文中。                                                                                                  |
| `auto:N` | 與 `auto` 相同，但具有自訂百分比。`auto:5` 在工具定義超過上下文窗口的 5% 時啟動。較低的值會更早啟動。                                                                                                       |
| `false`  | 工具搜尋已關閉。所有工具定義在每個輪次都被加載到上下文中。                                                                                                                                       |

設置 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/zh-TW/env-vars) 會保持工具搜尋關閉，且 `ENABLE_TOOL_SEARCH` 無法覆蓋它。該變數會移除 `defer_loading` 工具定義和 `tool_reference` 內容塊所需的 beta 標頭。

工具搜尋適用於所有已註冊的工具，無論它們來自遠端 MCP 伺服器還是[自訂 SDK MCP 伺服器](/docs/zh-TW/agent-sdk/custom-tools)。使用 `auto` 時，閾值基於所有伺服器上所有工具定義的組合大小。

在 `query()` 上的 `env` 選項中設置該值。在 TypeScript 中，`env` 會取代子程序環境，因此請展開 `...process.env` 以保留繼承的變數。在 Python 中，`env` 會合併到繼承的環境之上。此示例連接到公開許多工具的遠端 MCP 伺服器，使用萬用字元預先批准所有工具，並使用 `auto:5` 以便在工具定義超過上下文窗口的 5% 時啟動工具搜尋：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when tools exceed 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when tools exceed 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

若要執行此示例，請將 `https://tools.example.com/mcp` 替換為您自己的 MCP 伺服器的 URL。成功時，結果文字會列印到主控台。

因為這是單次 `query()` 呼叫，SDK 會在產生錯誤結果後引發，所以此示例會將迴圈包裝在 try 區塊中。若要查看執行失敗的原因，請檢查結果訊息的 `subtype`（例如 `error_during_execution`）在迴圈內。如需有關結果訊息的詳細資訊，請參閱[處理結果](/docs/zh-TW/agent-sdk/agent-loop#handle-the-result)。

將 `ENABLE_TOOL_SEARCH` 設置為 `"false"` 會禁用工具搜尋，並在每個輪次將所有工具定義加載到上下文中。這會移除搜尋往返，當工具集較小（少於約 10 個工具）且定義舒適地適應上下文窗口時，這可能會更快。

<h2 id="optimize-tool-discovery">
  優化工具發現
</h2>

搜尋機制將查詢與工具名稱和描述進行匹配。`search_slack_messages` 之類的名稱比 `query_slack` 適用於更廣泛的請求。具有特定關鍵字的描述（「按關鍵字、頻道或日期範圍搜尋 Slack 消息」）比通用描述（「查詢 Slack」）匹配更多查詢。

您也可以添加一個系統提示部分，列出可用的工具類別。這為代理提供了有關可搜尋的工具類型的上下文。通過 TypeScript 中的 `systemPrompt` 選項或 Python 中的 `system_prompt` 傳遞文本，使用 `claude_code` 預設搭配 `append`，這會將您的文本添加到預設的提示中，而不是替換它：

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

如需完整的系統提示選項集合，請參閱[修改系統提示](/docs/zh-TW/agent-sdk/modifying-system-prompts)。

<h2 id="limits">
  限制
</h2>

* **最大工具數：** 您的目錄中有 10,000 個工具
* **搜尋結果：** 預設情況下每次搜尋返回最多五個最相關的工具
* **模型支援：** Claude Sonnet 4.5、Claude Haiku 4.5、Claude Opus 4.5 及更新版本；請參閱 [API 文件中的模型相容性](https://platform.claude.com/docs/zh-TW/agents-and-tools/tool-use/tool-search-tool#model-compatibility)以取得目前清單。在 Google Cloud 的 Agent Platform 上，支援 Claude Sonnet 4.5 及更新版本和 Claude Opus 4.5 及更新版本。

<h2 id="related-documentation">
  相關文檔
</h2>

* [API 中的工具搜尋](https://platform.claude.com/docs/zh-TW/agents-and-tools/tool-use/tool-search-tool)：工具搜尋的完整 API 文檔，包括自訂實現
* [連接 MCP 伺服器](/docs/zh-TW/agent-sdk/mcp)：通過 MCP 伺服器連接到外部工具
* [自訂工具](/docs/zh-TW/agent-sdk/custom-tools)：使用 SDK MCP 伺服器構建您自己的工具
* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)：完整 API 參考
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python)：完整 API 參考
