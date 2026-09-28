> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 할일 추적

> Agent SDK 세션에서 할일을 추적하고 구조화된 도구 호출에서 Claude의 진행 상황을 애플리케이션에 렌더링합니다

Claude Code는 [모델 가용성](#model-availability)에 나열된 모델에서만 기본적으로 [작업 추적 도구](/docs/ko/tools-reference#task-tool-availability)를 제공합니다. 최신 모델은 작성된 할일 목록 없이 다단계 작업을 추적하므로, 해당 모델에서는 Claude가 다단계 작업을 수행하기 위해 이 페이지의 내용이 필요하지 않습니다.

작업 추적 도구가 있는 세션에서 Claude는 작성된 할일 목록을 유지하며 작업하면서 각 항목의 상태를 업데이트합니다. 메시지 스트림에서 각 변경 사항을 구조화된 도구 호출로 볼 수 있습니다. 애플리케이션이 작업 활동을 기록하거나 자체 진행 상황 표시를 렌더링하기 위해 해당 도구 호출을 읽을 때만 세션을 옵트인합니다.

<h2 id="model-availability">
  모델 가용성
</h2>

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.
</Note>

기본적으로 도구가 없는 모델에서 세션을 옵트인하지 않으면 메시지 스트림에서 이러한 도구에 대한 `tool_use` 블록을 볼 수 없습니다. Agent SDK는 번들된 Claude Code 바이너리를 통해 이러한 기본값을 적용합니다. `pathToClaudeCodeExecutable`(TypeScript) 또는 `cli_path`(Python)를 자신의 Claude Code 설치로 지정하면 해당 설치가 제공하는 도구를 자체 기본값에 따라 얻습니다. 실행 중인 세션에서 정확한 집합을 보려면 [사용 가능한 도구 확인](/docs/ko/tools-reference#check-which-tools-are-available)을 참조하십시오. 세션을 옵트인하려면 다음 중 하나를 수행하십시오:

* [`allowedTools`](/docs/ko/agent-sdk/permissions#allow-and-deny-rules)(TypeScript) 또는 `allowed_tools`(Python) 옵션에서 도구 중 하나의 이름을 지정합니다
* `tools` 옵션에 도구를 나열합니다. 이는 세션의 기본 제공 도구를 이름이 지정된 도구로 제한합니다. 사용하는 다른 기본 제공 도구와 함께 원하는 도구를 포함합니다
* 이 페이지의 예제처럼 `env` 옵션에서 `CLAUDE_CODE_ENABLE_TODO_TOOLS=1`을 설정합니다. TypeScript에서 `env`는 서브프로세스 환경을 대체하므로 `...process.env`를 전개하여 상속된 변수를 유지합니다. Python에서 `env`는 상속된 환경 위에 병합됩니다

<h2 id="todo-lifecycle">
  할일 생명주기
</h2>

Claude는 각 할일을 예측 가능한 생명주기를 통해 이동합니다:

1. **생성됨**: Claude가 작업을 식별할 때 할일을 `pending`으로 추가합니다
2. **활성화됨**: Claude가 작업을 시작할 때 할일을 `in_progress`로 설정합니다
3. **완료됨**: Claude가 작업이 성공적으로 완료되었을 때 표시합니다
4. **제거됨**: Claude가 더 이상 필요하지 않은 할일을 `TaskUpdate` 호출에서 `status: "deleted"`를 설정하여 삭제합니다

<h2 id="when-claude-creates-todos">
  Claude가 할일을 생성하는 경우
</h2>

[작업 추적 도구가 있는 세션](#model-availability)에서 Claude는 다음과 같은 대부분의 다단계 작업에 대해 할일을 생성합니다:

* **복잡한 다단계 작업** - 3개 이상의 서로 다른 작업이 필요한 경우
* **사용자 제공 작업 목록** - 여러 항목이 언급될 때
* **더 긴 작업** - 진행 상황 추적이 도움이 되는 경우
* **명시적 요청** - 사용자가 할일 구성을 요청할 때

Claude는 매우 짧거나 단일 단계의 요청에 대해 할일을 건너뛸 수 있습니다.

<h2 id="examples">
  예제
</h2>

이 예제들을 실행하기 전에 [빠른 시작](/docs/ko/agent-sdk/quickstart)을 따라 Claude Agent SDK를 설치하십시오. 이 페이지의 모든 예제는 동일한 권한 설정 및 종료 동작을 공유합니다:

* **권한 모드**: 예제 프롬프트는 Claude에게 프로젝트에서 실제 작업을 수행하도록 요청하므로 각 예제는 `permissionMode: "acceptEdits"`(TypeScript) 또는 `permission_mode="acceptEdits"`(Python)를 설정하여 작업이 생성하는 파일 편집을 자동 승인합니다. [권한 모드](/docs/ko/agent-sdk/permissions#permission-modes)에서 대안을 참조하십시오.
* **턴 제한**: 각 예제는 에이전트가 완료되고 최종 결과 메시지를 생성할 때까지 실행됩니다. 세션이 먼저 턴 제한에 도달하면 해당 결과 메시지는 `error_max_turns` 서브타입을 가집니다. 해당 종료를 감지하려면 `subtype`을 확인하십시오.
* **오류 처리**: 이 예제들은 단일 `query()` 호출을 사용합니다. `error_max_turns` 결과를 생성한 후 `query()`는 `Reached maximum number of turns`를 포함하는 오류를 발생시킵니다. 각 예제는 이것이 발생할 때 깔끔하게 종료하기 위해 루프를 try 블록으로 래핑합니다. 결과 서브타입에 대해서는 [결과 처리](/docs/ko/agent-sdk/agent-loop#handle-the-result)를 참조하십시오.

<Note>
  작업 시스템 메시지인 [`SDKTaskNotificationMessage`](/docs/ko/agent-sdk/typescript#sdktasknotificationmessage)(TypeScript) 또는 [`TaskNotificationMessage`](/docs/ko/agent-sdk/python#tasknotificationmessage)(Python)는 백그라운드 명령 및 서브에이전트와 같은 백그라운드 작업을 보고합니다. 메시지 스트림에서 할일 활동을 어시스턴트 메시지의 `tool_use` 블록으로 볼 수 있습니다.
</Note>

<h3 id="monitor-todo-changes">
  할일 변경 모니터링
</h3>

다음 예제는 어시스턴트 스트림에서 `TaskCreate` 및 `TaskUpdate` `tool_use` 블록을 감시하고 각 새 작업의 주제와 함께 `+` 줄을 인쇄하며 각 상태 변경의 작업 ID와 새 상태와 함께 업데이트 줄을 인쇄합니다. 렌더링된 표시 대신 작업 활동의 로그를 원할 때 이 형태를 사용하십시오. `+` 줄에는 할당된 ID가 포함되지 않으므로 이 로그는 업데이트를 생성과 다시 일치시킬 수 없습니다. 해당 상관관계를 유지하려면 [실시간 진행 상황 표시](#display-progress-in-real-time)처럼 ID를 캡처하십시오.

스트리밍된 `tool_use` 입력은 모델이 내보낸 원본 형태입니다. Claude Code는 실행 전에 일부 거의 올바른 키 이름을 수정하여 `id` 또는 `task_id`를 `taskId`로, `active_form`을 `activeForm`으로 매핑하지만, 이 수정은 스트림에 반영되지 않습니다. 이 페이지의 두 예제처럼 `TaskUpdate` 입력 필드를 방어적으로 읽으십시오. 정규 이름이 항상 존재한다고 가정하지 마십시오.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
      // Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
      options: { maxTurns: 15, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
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
              prompt="Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
              # Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
              options=ClaudeAgentOptions(max_turns=15, permission_mode="acceptEdits", env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"}),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input.get('subject', '')}")
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

<h3 id="display-progress-in-real-time">
  실시간 진행 상황 표시
</h3>

다음 예제는 어시스턴트 스트림에서 `TaskCreate` 및 `TaskUpdate` `tool_use` 블록을 감시하고 `TaskTracker` 클래스에서 작업 ID로 키가 지정된 작업 맵을 유지하며 모든 변경에서 진행 상황 요약을 다시 렌더링합니다. 요약은 완료되고 진행 중인 작업을 계산하고 각 활성 항목의 `activeForm` 레이블을 `subject` 대신 표시합니다. 애플리케이션이 각 이벤트를 기록하는 대신 진행 상황 표시를 유지할 때 이 형태를 사용하십시오.

할당된 작업 ID는 `TaskCreate` 입력에 없습니다. Claude Code는 각 도구의 구조화된 출력을 `tool_result` 블록을 포함하는 사용자 메시지에서 `tool_use_result` 필드로 전달합니다. `TaskCreate`의 경우 해당 객체는 TypeScript의 [도구 출력 유형](/docs/ko/agent-sdk/typescript#tool-output-types) 아래 `TaskCreateOutput`으로 문서화되며, Python에서 필드는 동일한 형태의 일반 dict입니다. 추적기는 `tool_use_id`로 각 `tool_result` 블록을 해당 `tool_use` 호출과 쌍을 이루고 쌍을 이룬 메시지의 `tool_use_result`에서 `task.id`를 읽습니다. Claude는 `TaskList`로 목록을 다시 읽을 수 있고 `TaskGet`으로 한 작업의 전체 세부 정보를 읽을 수 있습니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  type Task = { subject: string; activeForm?: string; status: string };

  class TaskTracker {
    private tasks = new Map<string, Task>();
    private pendingCreates = new Map<string, { subject: string; activeForm?: string }>();

    displayProgress() {
      if (this.tasks.size === 0) {
        console.log("\nProgress: no open tasks\n");
        return;
      }

      const items = [...this.tasks.values()];
      const completed = items.filter((t) => t.status === "completed").length;
      const inProgress = items.filter((t) => t.status === "in_progress").length;

      console.log(`\nProgress: ${completed}/${this.tasks.size} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      for (const [id, task] of this.tasks) {
        const icon =
          task.status === "completed" ? "✅" : task.status === "in_progress" ? "🔧" : "❌";
        const text = task.status === "in_progress" && task.activeForm ? task.activeForm : task.subject;
        console.log(`${id}. ${icon} ${text}`);
      }
    }

    handleToolUse(block: { id: string; name: string; input: unknown }) {
      if (block.name === "TaskCreate") {
        const input = block.input as { subject: string; activeForm?: string; active_form?: string };
        this.pendingCreates.set(block.id, {
          subject: input.subject,
          activeForm: input.activeForm ?? input.active_form,
        });
      } else if (block.name === "TaskUpdate") {
        const input = block.input as {
          taskId?: string;
          id?: string;
          task_id?: string;
          status?: string;
          activeForm?: string;
          active_form?: string;
        };
        const taskId = input.taskId ?? input.id ?? input.task_id;
        if (!taskId) return;
        if (input.status === "deleted") {
          this.tasks.delete(taskId);
          this.displayProgress();
          return;
        }
        const task = this.tasks.get(taskId);
        if (!task) return;
        if (input.status) task.status = input.status;
        const active = input.activeForm ?? input.active_form;
        if (active) task.activeForm = active;
        this.displayProgress();
      }
    }

    handleToolResult(block: { tool_use_id: string; is_error?: boolean }, result: unknown) {
      const create = this.pendingCreates.get(block.tool_use_id);
      if (!create) return;
      this.pendingCreates.delete(block.tool_use_id);
      if (block.is_error) return;
      // The result's user message carries the tool's structured output as
      // tool_use_result; for TaskCreate that's TaskCreateOutput,
      // { task: { id, subject } }.
      const out = result as { task?: { id: string } };
      if (!out?.task?.id) return;
      this.tasks.set(out.task.id, { ...create, status: "pending" });
      this.displayProgress();
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          options: { maxTurns: 20, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use") this.handleToolUse(block);
            }
          }
          if (message.type === "user" && Array.isArray(message.message.content)) {
            for (const block of message.message.content) {
              if (block.type === "tool_result") this.handleToolResult(block, message.tool_use_result);
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
  const tracker = new TaskTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      AssistantMessage,
      UserMessage,
      ToolUseBlock,
      ToolResultBlock,
  )


  class TaskTracker:
      def __init__(self):
          self.tasks: dict[str, dict] = {}
          self.pending_creates: dict[str, dict] = {}

      def display_progress(self):
          if not self.tasks:
              print("\nProgress: no open tasks\n")
              return

          completed = len([t for t in self.tasks.values() if t["status"] == "completed"])
          in_progress = len([t for t in self.tasks.values() if t["status"] == "in_progress"])

          print(f"\nProgress: {completed}/{len(self.tasks)} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for task_id, task in self.tasks.items():
              icon = (
                  "✅"
                  if task["status"] == "completed"
                  else "🔧"
                  if task["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  task["activeForm"]
                  if task["status"] == "in_progress" and task.get("activeForm")
                  else task["subject"]
              )
              print(f"{task_id}. {icon} {text}")

      def handle_tool_use(self, block: ToolUseBlock):
          if block.name == "TaskCreate":
              self.pending_creates[block.id] = {
                  "subject": block.input.get("subject", ""),
                  "activeForm": block.input.get("activeForm") or block.input.get("active_form"),
              }
          elif block.name == "TaskUpdate":
              task_id = (
                  block.input.get("taskId")
                  or block.input.get("id")
                  or block.input.get("task_id")
              )
              if not task_id:
                  return
              if block.input.get("status") == "deleted":
                  self.tasks.pop(task_id, None)
                  self.display_progress()
                  return
              task = self.tasks.get(task_id)
              if not task:
                  return
              if block.input.get("status"):
                  task["status"] = block.input["status"]
              active = block.input.get("activeForm") or block.input.get("active_form")
              if active:
                  task["activeForm"] = active
              self.display_progress()

      def handle_tool_result(self, block: ToolResultBlock, tool_use_result):
          create = self.pending_creates.pop(block.tool_use_id, None)
          if create is None or block.is_error:
              return
          # The result's user message carries the tool's structured output as
          # tool_use_result; for TaskCreate that's {"task": {"id": ..., "subject": ...}}.
          task = (tool_use_result or {}).get("task") or {}
          if not task.get("id"):
              return
          self.tasks[task["id"]] = {**create, "status": "pending"}
          self.display_progress()

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  options=ClaudeAgentOptions(
                      max_turns=20,
                      permission_mode="acceptEdits",
                      env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"},
                  ),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              self.handle_tool_use(block)
                  if isinstance(message, UserMessage) and isinstance(message.content, list):
                      for block in message.content:
                          if isinstance(block, ToolResultBlock):
                              self.handle_tool_result(block, message.tool_use_result)
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TaskTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  관련 문서
</h2>

* [Agent SDK 참고 - TypeScript](/docs/ko/agent-sdk/typescript): TypeScript SDK의 옵션, 유형 및 도구 스키마(작업 도구 입력 및 출력 유형 포함)
* [Agent SDK 참고 - Python](/docs/ko/agent-sdk/python): Python SDK의 옵션, 유형 및 도구 문서
* [스트리밍 입력](/docs/ko/agent-sdk/streaming-vs-single-mode): 두 입력 모드 및 이 예제들이 사용하는 단일 샷 호출 대신 스트리밍 입력을 사용할 때
* [Claude에 사용자 정의 도구 제공](/docs/ko/agent-sdk/custom-tools): SDK의 인프로세스 MCP 서버로 자신의 도구를 정의합니다
