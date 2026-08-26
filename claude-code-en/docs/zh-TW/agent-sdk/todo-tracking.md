> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 待辦事項清單

> 使用 Claude Agent SDK 追蹤和顯示待辦事項，以實現有組織的任務管理

待辦事項追蹤提供了一種結構化的方式來管理任務並向用戶顯示進度。Claude Agent SDK 包含內置的待辦事項功能，可幫助組織複雜的工作流程並讓用戶了解任務進度。

<Note>
  自 TypeScript Agent SDK 0.3.142 和 Claude Code v2.1.142 起，會話使用結構化的 Task 工具 `TaskCreate`、`TaskUpdate`、`TaskGet` 和 `TaskList`，而不是 `TodoWrite`。Python SDK 從它啟動的 Claude Code CLI 獲得此變更，而不是從 Python 套件版本：一旦該 CLI（pip 套件內捆綁的副本，或您使用 `cli_path` 指向的副本）為 v2.1.142 或更新版本，此切換就會適用。請參閱[遷移到 Task 工具](#migrate-to-task-tools)以了解監控代碼如何變更。此頁面上的範例設置 `CLAUDE_CODE_ENABLE_TASKS=0` 以繼續為尚未遷移的會話顯示 `TodoWrite`。
</Note>

<h3 id="todo-lifecycle">
  待辦事項生命週期
</h3>

待辦事項遵循可預測的生命週期：

1. **建立**為 `pending` 當任務被識別時
2. **啟動**為 `in_progress` 當工作開始時
3. **完成**當任務成功完成時
4. **移除**當群組中的所有任務都完成時

<h3 id="when-todos-are-used">
  何時使用待辦事項
</h3>

SDK 會為大多數多步驟工作建立待辦事項，例如：

* **複雜的多步驟任務**需要 3 個或更多不同的操作
* **用戶提供的任務清單**當提及多個項目時
* **非平凡的操作**受益於進度追蹤
* **明確的請求**當用戶要求待辦事項組織時

它可能會跳過非常短或單步驟請求的待辦事項。

<h2 id="examples">
  範例
</h2>

在執行這些範例之前，請按照[快速入門](/docs/zh-TW/agent-sdk/quickstart)安裝 Claude Agent SDK。

每個範例會執行到代理程式完成並產生其最終結果訊息為止。如果工作階段先達到其輪次限制，該結果訊息會有 `error_max_turns` 子類型。檢查 `subtype` 以偵測該結束。

這些範例使用單次 `query()` 呼叫。在產生 `error_max_turns` 結果後，`query()` 會拋出包含 `Reached maximum number of turns` 的錯誤。每個範例都將其迴圈包裝在 try 區塊中，以便在發生這種情況時乾淨地退出。

請參閱[處理結果](/docs/zh-TW/agent-sdk/agent-loop#handle-the-result)以了解結果子類型。

<h3 id="monitoring-todo-changes">
  監控待辦事項變更
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      // Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
      // Task tools instead and these tool_use blocks never appear.
      options: { maxTurns: 15, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
    })) {
      // Todo updates are reflected in the message stream
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use" && block.name === "TodoWrite") {
            const todos = block.input.todos;

            console.log("Todo Status Update:");
            todos.forEach((todo, index) => {
              const status =
                todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
              console.log(`${index + 1}. ${status} ${todo.content}`);
            });
          }
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // such as when the maxTurns limit is hit.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock


  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              # Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
              # Task tools instead and these tool_use blocks never appear.
              options=ClaudeAgentOptions(max_turns=15, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
          ):
              # Todo updates are reflected in the message stream
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                          todos = block.input["todos"]

                          print("Todo Status Update:")
                          for i, todo in enumerate(todos):
                              status = (
   "✅"
   if todo["status"] == "completed"
   else "🔧"
   if todo["status"] == "in_progress"
   else "❌"
                              )
                              print(f"{i + 1}. {status} {todo['content']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # such as when the max_turns limit is hit.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="real-time-progress-display">
  實時進度顯示
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  class TodoTracker {
    private todos: any[] = [];

    displayProgress() {
      if (this.todos.length === 0) return;

      const completed = this.todos.filter((t) => t.status === "completed").length;
      const inProgress = this.todos.filter((t) => t.status === "in_progress").length;
      const total = this.todos.length;

      console.log(`\nProgress: ${completed}/${total} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      this.todos.forEach((todo, index) => {
        const icon =
          todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
        const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
        console.log(`${index + 1}. ${icon} ${text}`);
      });
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          // Re-enable TodoWrite, which this tracker watches for.
          options: { maxTurns: 20, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use" && block.name === "TodoWrite") {
                this.todos = block.input.todos;
                this.displayProgress();
              }
            }
          }
        }
      } catch (error) {
        // A single-shot query() throws after yielding an error result,
        // such as when the maxTurns limit is hit.
        console.log(`Session ended with an error: ${error}`);
      }
    }
  }

  // Usage
  const tracker = new TodoTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
  from typing import List, Dict


  class TodoTracker:
      def __init__(self):
          self.todos: List[Dict] = []

      def display_progress(self):
          if not self.todos:
              return

          completed = len([t for t in self.todos if t["status"] == "completed"])
          in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
          total = len(self.todos)

          print(f"\nProgress: {completed}/{total} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for i, todo in enumerate(self.todos):
              icon = (
                  "✅"
                  if todo["status"] == "completed"
                  else "🔧"
                  if todo["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  todo["activeForm"]
                  if todo["status"] == "in_progress"
                  else todo["content"]
              )
              print(f"{i + 1}. {icon} {text}")

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  # Re-enable TodoWrite, which this tracker watches for.
                  options=ClaudeAgentOptions(max_turns=20, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                              self.todos = block.input["todos"]
                              self.display_progress()
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TodoTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="migrate-to-task-tools">
  遷移到 Task 工具
</h2>

Task 工具將單個 `TodoWrite` 呼叫分割為每個新項目的 `TaskCreate` 和每個狀態變更的 `TaskUpdate`，並提供 `TaskList` 和 `TaskGet` 供模型讀回當前清單。您的監控代碼仍然檢查助手流中的 `tool_use` 區塊，但維護一個由任務 ID 鍵入的映射，而不是在每次呼叫時替換整個清單。Task 工具是 TypeScript Agent SDK 0.3.142 和 Claude Code v2.1.142 起的預設值，因此不需要 `options.env` 變更。

| 使用 `TodoWrite`                         | 使用 Task 工具                                                                                                                                                                                                                                                                    |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 一個工具呼叫重寫完整的 `todos` 陣列                 | `TaskCreate` 新增一個項目，`TaskUpdate` 按 `taskId` 修補一個項目                                                                                                                                                                                                                            |
| 匹配 `block.name === "TodoWrite"`        | 匹配 `block.name === "TaskCreate"` 或 `"TaskUpdate"`                                                                                                                                                                                                                             |
| 項目形狀：`{ content, status, activeForm }` | `TaskCreate` 輸入：`{ subject, description, activeForm?, metadata? }`。`TaskUpdate` 輸入：`{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`。`status` 是 `"pending"`、`"in_progress"` 或 `"completed"`；設置 `status: "deleted"` 以刪除 |
| 直接呈現 `block.input.todos`               | 跨呼叫累積項目，或從 `TaskList` 工具結果讀取快照                                                                                                                                                                                                                                                |

指派的任務 ID 不在 `TaskCreate` 輸入中。它在匹配的 `tool_result` 中作為 `{ task: { id, subject } }` 返回，因此從結果區塊捕獲它以鍵入您的映射。以下範例顯示了對[監控待辦事項變更](#monitoring-todo-changes)迴圈的最小變更。它僅讀取 `tool_use` 輸入並跳過從 `tool_result` 區塊捕獲 ID。要呈現完整清單，請在流中監視 `TaskList` 工具結果或將 `TaskCreate` 結果和 `TaskUpdate` 輸入累積到映射中。

串流的 `tool_use` 輸入是模型發出的原始形狀。Claude Code 在執行前修復一些接近但不正確的鍵名，將 `id` 或 `task_id` 映射到 `taskId` 和 `active_form` 映射到 `activeForm`，但該修復不會反映在流中。防禦性地讀取 `TaskUpdate` 輸入欄位，如下面的範例所示，而不是假設規範名稱始終存在。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      options: { maxTurns: 15 },
    })) {
      if (message.type !== "assistant") continue;
      for (const block of message.message.content) {
        if (block.type !== "tool_use") continue;
        if (block.name === "TaskCreate") {
          const input = block.input as { subject: string };
          console.log(`+ ${input.subject}`);
        } else if (block.name === "TaskUpdate") {
          const input = block.input as {
            taskId?: string;
            id?: string;
            task_id?: string;
            status?: string;
          };
          const taskId = input.taskId ?? input.id ?? input.task_id;
          if (taskId && input.status) console.log(`  ${taskId} -> ${input.status}`);
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock

  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              options=ClaudeAgentOptions(max_turns=15),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input['subject']}")
                  elif block.name == "TaskUpdate" and block.input.get("status"):
                      task_id = (
                          block.input.get("taskId")
                          or block.input.get("id")
                          or block.input.get("task_id")
                      )
                      if task_id:
                          print(f"  {task_id} -> {block.input['status']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  相關文檔
</h2>

* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python)
* [串流與單一模式](/docs/zh-TW/agent-sdk/streaming-vs-single-mode)
* [自訂工具](/docs/zh-TW/agent-sdk/custom-tools)
