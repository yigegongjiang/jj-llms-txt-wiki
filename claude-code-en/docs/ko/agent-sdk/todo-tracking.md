> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 할일 목록

> Claude Agent SDK를 사용하여 할일을 추적하고 표시하여 체계적인 작업 관리를 수행합니다

할일 추적은 작업을 관리하고 사용자에게 진행 상황을 표시하는 구조화된 방법을 제공합니다. Claude Agent SDK에는 복잡한 워크플로우를 구성하고 사용자에게 작업 진행 상황을 알리는 데 도움이 되는 기본 제공 할일 기능이 포함되어 있습니다.

<Note>
  TypeScript Agent SDK 0.3.142 및 Claude Code v2.1.142부터 세션은 `TodoWrite` 대신 구조화된 Task 도구인 `TaskCreate`, `TaskUpdate`, `TaskGet`, `TaskList`를 사용합니다. Python SDK는 Python 패키지 버전이 아닌 실행하는 Claude Code CLI에서 이 변경 사항을 가져옵니다. pip 패키지 내에 번들된 CLI 또는 `cli_path`로 지정한 CLI가 v2.1.142 이상이면 전환이 적용됩니다. 모니터링 코드 변경 방법은 [Task 도구로 마이그레이션](#migrate-to-task-tools)을 참조하십시오. 이 페이지의 예제는 아직 마이그레이션하지 않은 세션에 대해 `TodoWrite`를 계속 표시하기 위해 `CLAUDE_CODE_ENABLE_TASKS=0`을 설정합니다.
</Note>

<h3 id="todo-lifecycle">
  할일 생명주기
</h3>

할일은 예측 가능한 생명주기를 따릅니다:

1. **생성됨** - 작업이 식별될 때 `pending`으로 생성됨
2. **활성화됨** - 작업이 시작될 때 `in_progress`로 활성화됨
3. **완료됨** - 작업이 성공적으로 완료될 때
4. **제거됨** - 그룹의 모든 작업이 완료될 때

<h3 id="when-todos-are-used">
  할일이 사용되는 경우
</h3>

SDK는 대부분의 다단계 작업에 대해 할일을 생성합니다. 예를 들면:

* **복잡한 다단계 작업** - 3개 이상의 서로 다른 작업이 필요한 경우
* **사용자 제공 작업 목록** - 여러 항목이 언급될 때
* **중요한 작업** - 진행 상황 추적이 도움이 되는 경우
* **명시적 요청** - 사용자가 할일 구성을 요청할 때

매우 짧거나 단일 단계의 요청에 대해서는 할일을 건너뛸 수 있습니다.

<h2 id="examples">
  예제
</h2>

이 예제들을 실행하기 전에 [빠른 시작](/docs/ko/agent-sdk/quickstart)을 따라 Claude Agent SDK를 설치하십시오.

각 예제는 에이전트가 완료될 때까지 실행되고 최종 결과 메시지를 생성합니다. 세션이 먼저 턴 제한에 도달하면 해당 결과 메시지는 `error_max_turns` 서브타입을 가집니다. 해당 종료를 감지하려면 `subtype`을 확인하십시오.

이 예제들은 단일 `query()` 호출을 사용합니다. `error_max_turns` 결과를 생성한 후 `query()`는 `Reached maximum number of turns`를 포함하는 오류를 발생시킵니다. 각 예제는 이것이 발생할 때 깔끔하게 종료하기 위해 루프를 try 블록으로 래핑합니다.

결과 서브타입에 대해서는 [결과 처리](/docs/ko/agent-sdk/agent-loop#handle-the-result)를 참조하십시오.

<h3 id="monitoring-todo-changes">
  할일 변경 모니터링
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
  실시간 진행 상황 표시
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
  Task 도구로 마이그레이션
</h2>

Task 도구는 단일 `TodoWrite` 호출을 각 새 항목에 대한 `TaskCreate`와 각 상태 변경에 대한 `TaskUpdate`로 분할하며, `TaskList`와 `TaskGet`은 모델이 현재 목록을 다시 읽을 수 있도록 사용 가능합니다. 모니터링 코드는 여전히 어시스턴트 스트림의 `tool_use` 블록을 검사하지만, 모든 호출에서 전체 목록을 바꾸는 대신 작업 ID로 키가 지정된 맵을 유지합니다. Task 도구는 TypeScript Agent SDK 0.3.142 및 Claude Code v2.1.142부터 기본값이므로 `options.env` 변경이 필요하지 않습니다.

| `TodoWrite` 사용                           | Task 도구 사용                                                                                                                                                                                                                                                                                |
| ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 한 번의 도구 호출로 전체 `todos` 배열을 다시 작성         | `TaskCreate`는 한 항목을 추가하고, `TaskUpdate`는 `taskId`로 한 항목을 패치                                                                                                                                                                                                                                |
| `block.name === "TodoWrite"` 일치          | `block.name === "TaskCreate"` 또는 `"TaskUpdate"` 일치                                                                                                                                                                                                                                        |
| 항목 형태: `{ content, status, activeForm }` | `TaskCreate` 입력: `{ subject, description, activeForm?, metadata? }`. `TaskUpdate` 입력: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status`는 `"pending"`, `"in_progress"`, 또는 `"completed"`이며, 삭제하려면 `status: "deleted"`를 설정 |
| `block.input.todos`를 직접 렌더링              | 호출 전체에서 항목을 누적하거나, `TaskList` 도구 결과에서 스냅샷을 읽음                                                                                                                                                                                                                                             |

할당된 작업 ID는 `TaskCreate` 입력에 없습니다. 일치하는 `tool_result`에서 `{ task: { id, subject } }`로 반환되므로, 맵을 키로 지정하기 위해 결과 블록에서 캡처합니다. 다음 예제는 [할일 변경 모니터링](#monitoring-todo-changes) 루프에 대한 최소한의 변경을 보여줍니다. 이는 `tool_use` 입력만 읽고 `tool_result` 블록에서 ID 캡처를 건너뜁니다. 전체 목록을 렌더링하려면 스트림에서 `TaskList` 도구 결과를 감시하거나 `TaskCreate` 결과와 `TaskUpdate` 입력을 맵으로 누적합니다.

스트리밍된 `tool_use` 입력은 모델이 내보낸 원본 형태입니다. Claude Code는 실행 전에 일부 거의 올바른 키 이름을 수정하여 `id` 또는 `task_id`를 `taskId`로, `active_form`을 `activeForm`으로 매핑하지만, 이 수정은 스트림에 반영되지 않습니다. 아래 샘플처럼 `TaskUpdate` 입력 필드를 방어적으로 읽으십시오. 정규 이름이 항상 존재한다고 가정하지 마십시오.

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
  관련 문서
</h2>

* [TypeScript SDK 참고](/docs/ko/agent-sdk/typescript)
* [Python SDK 참고](/docs/ko/agent-sdk/python)
* [스트리밍 vs 단일 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)
* [사용자 정의 도구](/docs/ko/agent-sdk/custom-tools)
