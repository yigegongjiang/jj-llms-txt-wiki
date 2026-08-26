> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 sessions

> Sessions 如何保持代理對話歷史，以及何時使用 continue、resume 和 fork 返回到先前的運行。

Session 是 SDK 在您的代理工作時累積的對話歷史。它包含您的提示、代理進行的每個工具呼叫、每個工具結果和每個回應。SDK 會自動將其寫入磁碟，以便您稍後可以返回它。

返回 session 意味著代理具有之前的完整上下文：它已經讀取的檔案、它已經執行的分析、它已經做出的決定。您可以提出後續問題、從中斷中恢復，或分支以嘗試不同的方法。

<Note>
  Sessions 保持**對話**，而不是檔案系統。要快照和還原代理所做的檔案更改，請使用[檔案檢查點](/docs/zh-TW/agent-sdk/file-checkpointing)。
</Note>

本指南涵蓋如何為您的應用選擇正確的方法、自動追蹤 sessions 的 SDK 介面、如何捕獲 session ID 並手動使用 `resume` 和 `fork`，以及關於跨主機恢復 sessions 的注意事項。

<h2 id="choose-an-approach">
  選擇一個方法
</h2>

您需要多少 session 處理取決於您的應用程式的形狀。當您發送應該共享上下文的多個提示時，session 管理就會發揮作用。在單個 `query()` 呼叫中，代理已經根據需要進行了盡可能多的轉換，並且權限提示和 `AskUserQuestion` 是[在迴圈中處理](/docs/zh-TW/agent-sdk/user-input)的（它們不會結束呼叫）。

| 您正在構建的內容                         | 使用什麼                                                                                                                |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| 一次性任務：單個提示，無後續                   | 無需額外操作。一個 `query()` 呼叫可以處理它。                                                                                        |
| 在一個進程中進行多轉對話                     | [`ClaudeSDKClient`（Python）或 `continue: true`（TypeScript）](#automatic-session-management)。SDK 為您追蹤 session，無需 ID 處理。 |
| 在進程重新啟動後從中斷處繼續                   | `continue_conversation=True`（Python）/ `continue: true`（TypeScript）。恢復目錄中最近的 session，無需 ID。                          |
| 恢復特定的過去 session（不是最近的）           | 捕獲 session ID 並將其傳遞給 `resume`。                                                                                      |
| 嘗試替代方法而不失去原始方法                   | Fork session。                                                                                                       |
| 無狀態任務，不想將任何內容寫入磁碟（僅限 TypeScript） | 設定 [`persistSession: false`](/docs/zh-TW/agent-sdk/typescript#options)。Session 僅在呼叫期間存在於記憶體中。Python 始終保持到磁碟。             |

<h3 id="continue-resume-and-fork">
  Continue、resume 和 fork
</h3>

Continue、resume 和 fork 是您在 `query()` 上設定的選項欄位（Python 中的 [`ClaudeAgentOptions`](/docs/zh-TW/agent-sdk/python#claudeagentoptions)，TypeScript 中的 [`Options`](/docs/zh-TW/agent-sdk/typescript#options)）。

**Continue** 和 **resume** 都會拾取現有 session 並將其添加到其中。區別在於它們如何找到該 session：

* **Continue** 在當前目錄中找到最近的 session。您無需追蹤任何內容。當您的應用一次運行一個對話時效果很好。
* **Resume** 採用特定的 session ID。您追蹤 ID。當您有多個 sessions（例如，多使用者應用中每個使用者一個）或想要返回不是最近的 session 時需要。

**Fork** 不同：它創建一個新 session，從原始 session 的歷史副本開始。原始 session 保持不變。使用 fork 嘗試不同的方向，同時保持返回的選項。

<h2 id="automatic-session-management">
  自動 session 管理
</h2>

兩個 SDK 都提供一個介面，可以跨呼叫為您追蹤 session 狀態，因此您無需手動傳遞 ID。將這些用於單個進程中的多轉對話。

<h3 id="python-claudesdkclient">
  Python：`ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/zh-TW/agent-sdk/python#claudesdkclient) 在內部處理 session ID。每次呼叫 `client.query()` 都會自動繼續相同的 session。呼叫 [`client.receive_response()`](/docs/zh-TW/agent-sdk/python#claudesdkclient) 以迭代當前查詢的訊息。使用客戶端作為非同步上下文管理器，以便為您處理連接設置和拆卸，或手動呼叫 `connect()` 和 `disconnect()`。

此示例針對相同的 `client` 運行兩個查詢。第一個要求代理分析一個模組；第二個要求它重構該模組。因為兩個呼叫都通過相同的客戶端實例進行，第二個查詢具有來自第一個的完整上下文，無需任何明確的 `resume` 或 session ID：

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

有關何時使用 `ClaudeSDKClient` 與獨立 `query()` 函數的詳細信息，請參閱 [Python SDK 參考](/docs/zh-TW/agent-sdk/python#choosing-between-query-and-claudesdkclient)。

<h3 id="typescript-continue-true">
  TypeScript：`continue: true`
</h3>

TypeScript SDK 沒有像 Python 的 `ClaudeSDKClient` 那樣的 session 持有客戶端物件。相反，在每個後續 `query()` 呼叫上傳遞 `continue: true`，SDK 會在當前目錄中拾取最近的 session。無需 ID 追蹤。

此示例進行兩個單獨的 `query()` 呼叫。第一個創建一個新 session；第二個設定 `continue: true`，這告訴 SDK 在磁碟上找到並恢復最近的 session。代理具有來自第一個呼叫的完整上下文：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
for await (const message of query({
  prompt: "Analyze the auth module",
  options: { allowedTools: ["Read", "Glob", "Grep"] }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  實驗性的 [V2 session API](/docs/zh-TW/agent-sdk/typescript-v2-preview)（提供了 `createSession()` 與 `send` / `stream` 模式）已在 TypeScript Agent SDK 0.3.142 中移除。改用 `query()` 函數和本頁面上描述的 session 選項。
</Note>

<h2 id="use-session-options-with-query">
  使用 session 選項與 `query()`
</h2>

<h3 id="capture-the-session-id">
  捕獲 session ID
</h3>

Resume 和 fork 需要 session ID。從結果訊息上的 `session_id` 欄位讀取它（Python 中的 [`ResultMessage`](/docs/zh-TW/agent-sdk/python#resultmessage)，TypeScript 中的 [`SDKResultMessage`](/docs/zh-TW/agent-sdk/typescript#sdkresultmessage)），無論成功或錯誤，它都存在於每個結果上。在 TypeScript 中，ID 也可以作為初始化 `SystemMessage` 上的直接欄位更早獲得；在 Python 中，它嵌套在 `SystemMessage.data` 內。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      async for message in query(
          prompt="Analyze the auth module and suggest improvements",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id
              if message.subtype == "success":
                  print(message.result)

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  for await (const message of query({
    prompt: "Analyze the auth module and suggest improvements",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
      if (message.subtype === "success") {
        console.log(message.result);
      }
    }
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

<h3 id="resume-by-id">
  按 ID 恢復
</h3>

將 session ID 傳遞給 `resume` 以返回到該特定 session。代理從 session 中斷的任何地方拾取完整上下文。恢復的常見原因：

* **跟進已完成的任務。** 代理已經分析了某些內容；現在您希望它根據該分析採取行動，而無需重新讀取文件。
* **從限制中恢復。** 第一次運行以 `error_max_turns` 或 `error_max_budget_usd` 結束（請參閱[處理結果](/docs/zh-TW/agent-sdk/agent-loop#handle-the-result)）；以更高的限制恢復。
* **重新啟動您的進程。** 您在關閉前捕獲了 ID，並想恢復對話。

此示例使用後續提示恢復[捕獲 session ID](#capture-the-session-id) 中的 session。因為您正在恢復，代理已經在上下文中具有先前的分析：

<CodeGroup>
  ```python Python theme={null}
  # Earlier session analyzed the code; now build on that analysis
  async for message in query(
      prompt="Now implement the refactoring you suggested",
      options=ClaudeAgentOptions(
          resume=session_id,
          allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

您應該會看到一個基於先前分析而構建的回應，而不是從頭開始。這確認了代理以其先前的上下文完整恢復了 session。

<Tip>
  如果 `resume` 呼叫返回新 session 而不是預期的歷史，最常見的原因是不匹配的 `cwd`。Sessions 存儲在 `~/.claude/projects/<encoded-cwd>/*.jsonl` 下，或者如果您設定了 `CLAUDE_CONFIG_DIR` 環境變數，則存儲在 `$CLAUDE_CONFIG_DIR/projects/<encoded-cwd>/*.jsonl` 下，其中 `<encoded-cwd>` 是絕對工作目錄，每個非英數字元都被替換為 `-`（所以 `/Users/me/proj` 變成 `-Users-me-proj`）。如果您的 resume 呼叫從不同的目錄運行，SDK 會在錯誤的位置查找。session 文件也需要存在於當前機器上。
</Tip>

要跨機器或在無伺服器環境中恢復 sessions，請使用 [`SessionStore` 適配器](/docs/zh-TW/agent-sdk/session-storage)將記錄鏡像到共享存儲。

<h3 id="fork-to-explore-alternatives">
  Fork 以探索替代方案
</h3>

Forking 創建一個新 session，從原始 session 的歷史副本開始，但從該點開始分歧。fork 獲得自己的 session ID；原始的 ID 和歷史保持不變。您最終得到兩個獨立的 sessions，可以分別恢復。

<Note>
  Forking 分支對話歷史，而不是文件系統。如果 forked 代理編輯文件，這些更改是真實的，對在同一目錄中工作的任何 session 都可見。要分支和還原文件更改，請使用[文件檢查點](/docs/zh-TW/agent-sdk/file-checkpointing)。
</Note>

此示例基於[捕獲 session ID](#capture-the-session-id)：您已經在 `session_id` 中分析了一個 auth 模組，並想探索 OAuth2 而不失去 JWT 焦點的線程。第一個塊 forks session 並捕獲 fork 的 ID（`forked_id`）；第二個塊恢復原始 `session_id` 以繼續沿著 JWT 路徑。您現在有兩個 session ID 指向兩個單獨的歷史：

<CodeGroup>
  ```python Python theme={null}
  # Fork: branch from session_id into a new session
  forked_id = None
  async for message in query(
      prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
      options=ClaudeAgentOptions(
          resume=session_id,
          fork_session=True,
          max_turns=5,
      ),
  ):
      if isinstance(message, ResultMessage):
          forked_id = message.session_id  # The fork's ID, distinct from session_id
          if message.subtype == "success":
              print(message.result)

  print(f"Forked session: {forked_id}")

  # Original session is untouched; resuming it continues the JWT thread
  async for message in query(
      prompt="Continue with the JWT approach",
      options=ClaudeAgentOptions(resume=session_id),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  for await (const message of query({
    prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
    options: {
      resume: sessionId,
      forkSession: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      forkedId = message.session_id; // The fork's ID, distinct from sessionId
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  for await (const message of query({
    prompt: "Continue with the JWT approach",
    options: { resume: sessionId }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

您應該會看到 `forkedId` 與原始 session ID 不同。恢復原始 session 仍然會繼續 JWT 線程，這確認了 fork 沒有修改原始歷史。

<h2 id="resume-across-hosts">
  跨主機恢復
</h2>

Session 文件是創建它們的機器的本地文件。要在不同的主機上恢復 session（CI 工作者、臨時容器、無伺服器），您有兩個選項：

* **移動 session 文件。** 從第一次運行中保持 `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl`，並在呼叫 `resume` 之前將其恢復到新主機上的相同路徑。`cwd` 必須匹配。
* **不依賴 session 恢復。** 捕獲您需要的結果（分析輸出、決定、文件差異）作為應用程式狀態，並將其傳遞到新 session 的提示中。這通常比運送記錄文件更穩健。

兩個 SDK 都公開用於列舉磁碟上的 sessions 和讀取其訊息的函數：TypeScript 中的 [`listSessions()`](/docs/zh-TW/agent-sdk/typescript#listsessions) 和 [`getSessionMessages()`](/docs/zh-TW/agent-sdk/typescript#getsessionmessages)，Python 中的 [`list_sessions()`](/docs/zh-TW/agent-sdk/python#list_sessions) 和 [`get_session_messages()`](/docs/zh-TW/agent-sdk/python#get_session_messages)。使用它們構建自訂 session 選擇器、清理邏輯或記錄檢視器。

兩個 SDK 也公開用於查找和變更個別 sessions 的函數：Python 中的 [`get_session_info()`](/docs/zh-TW/agent-sdk/python#get_session_info)、[`rename_session()`](/docs/zh-TW/agent-sdk/python#rename_session) 和 [`tag_session()`](/docs/zh-TW/agent-sdk/python#tag_session)，以及 TypeScript 中的 [`getSessionInfo()`](/docs/zh-TW/agent-sdk/typescript#getsessioninfo)、[`renameSession()`](/docs/zh-TW/agent-sdk/typescript#renamesession) 和 [`tagSession()`](/docs/zh-TW/agent-sdk/typescript#tagsession)。使用它們按標籤組織 sessions 或給它們人類可讀的標題。

<h2 id="related-resources">
  相關資源
</h2>

* [代理迴圈如何工作](/docs/zh-TW/agent-sdk/agent-loop)：了解 session 中的轉換、訊息和上下文累積
* [文件檢查點](/docs/zh-TW/agent-sdk/file-checkpointing)：快照和還原代理在 session 中所做的文件更改
* [Python `ClaudeAgentOptions`](/docs/zh-TW/agent-sdk/python#claudeagentoptions)：Python 的完整 session 選項參考
* [TypeScript `Options`](/docs/zh-TW/agent-sdk/typescript#options)：TypeScript 的完整 session 選項參考
