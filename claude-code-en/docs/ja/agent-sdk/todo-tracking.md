> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Todo を追跡する

> Agent SDK セッションで todo を追跡し、構造化されたツール呼び出しから Claude の進捗をアプリケーションでレンダリングします

Claude Code は、[モデル利用可能性](#model-availability)に記載されているモデルでのみデフォルトで[タスク追跡ツール](/docs/ja/tools-reference#task-tool-availability)を提供します。新しいモデルは書かれた todo リストなしで複数ステップの作業を追跡するため、これらのモデルでは Claude が複数ステップのタスクを処理するためにこのページの内容は必要ありません。

タスク追跡ツールを持つセッションでは、Claude は書かれた todo リストを保持し、作業を進めるにつれて各アイテムのステータスを更新します。メッセージストリーム内で各変更が構造化されたツール呼び出しとして表示されます。アプリケーションがこれらのツール呼び出しを読み取る場合にのみセッションをオプトインしてください。タスク活動をログするか、独自の進捗表示をレンダリングするかどうかに関わらず。

<h2 id="model-availability">
  モデル利用可能性
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

デフォルトではツールを持たないモデルでは、セッションをオプトインしない限り、メッセージストリーム内でこれらのツールの `tool_use` ブロックは表示されません。Agent SDK は、バンドルされている Claude Code バイナリを通じてこれらのデフォルトを適用します。`pathToClaudeCodeExecutable`（TypeScript）または `cli_path`（Python）を独自の Claude Code インストールに指定する場合、そのインストールが提供するツールが、独自のデフォルトの下で取得されます。実行中のセッションで正確なセットを確認するには、[利用可能なツールを確認](/docs/ja/tools-reference#check-which-tools-are-available)してください。セッションをオプトインするには、以下のいずれかを実行してください：

* [`allowedTools`](/docs/ja/agent-sdk/permissions#allow-and-deny-rules)（TypeScript）または `allowed_tools`（Python）オプションでツールの 1 つに名前を付ける
* `tools` オプションにツールをリストします。これはセッションの組み込みツールを、それが名前を付けるものに制限します。使用する他の組み込みツールと一緒に必要なツールを含めます
* このページの例のように、`env` オプションで `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` を設定します。TypeScript では、`env` はサブプロセス環境を置き換えるため、継承された変数を保持するために `...process.env` を展開します。Python では、`env` は継承された環境の上にマージされます

<h2 id="todo-lifecycle">
  Todo ライフサイクル
</h2>

Claude は各 todo を予測可能なライフサイクルを通じて移動させます：

1. **作成**：Claude がタスクを識別したときに `pending` として todo を追加します
2. **アクティベート**：Claude が作業を開始したときに todo を `in_progress` に設定します
3. **完了**：Claude がタスクが正常に完了したときにそれを完了としてマークします
4. **削除**：Claude が `TaskUpdate` 呼び出しで `status: "deleted"` を設定することで、不要になった todo を削除します

<h2 id="when-claude-creates-todos">
  Claude が todo を作成するとき
</h2>

[タスク追跡ツールを持つセッション](#model-availability)では、Claude は以下のような複数ステップの作業のほとんどに対して todo を作成します：

* **複雑な複数ステップのタスク** - 3 つ以上の異なるアクションが必要な場合
* **ユーザー提供のタスクリスト** - 複数のアイテムが言及されている場合
* **より長い操作** - 進捗追跡の恩恵を受ける場合
* **明示的なリクエスト** - ユーザーが todo 整理を要求する場合

Claude は非常に短いまたは単一ステップのリクエストに対しては todo をスキップする場合があります。

<h2 id="examples">
  例
</h2>

これらの例を実行する前に、[クイックスタート](/docs/ja/agent-sdk/quickstart)に従って Claude Agent SDK をインストールしてください。このページのすべての例は同じ権限設定と終了動作を共有します：

* **権限モード**：例のプロンプトは Claude にプロジェクトで実際の作業を行うよう要求するため、各例は `permissionMode: "acceptEdits"`（TypeScript）または `permission_mode="acceptEdits"`（Python）を設定して、作業が生成するファイル編集を自動承認します。[権限モード](/docs/ja/agent-sdk/permissions#permission-modes)で代替案を参照してください。
* **ターン制限**：各例はエージェントが完了して最終結果メッセージを生成するまで実行されます。セッションがターン制限に最初に達した場合、その結果メッセージは `error_max_turns` サブタイプを持ちます。終了を検出するために `subtype` を確認してください。
* **エラー処理**：これらの例は単一ショットの `query()` 呼び出しを使用します。`error_max_turns` 結果を生成した後、`query()` は `Reached maximum number of turns` を含むエラーを発生させます。各例はそれが発生したときにクリーンに終了するために、ループを try ブロックでラップします。結果サブタイプについては、[結果を処理する](/docs/ja/agent-sdk/agent-loop#handle-the-result)を参照してください。

<Note>
  タスクシステムメッセージ、[`SDKTaskNotificationMessage`](/docs/ja/agent-sdk/typescript#sdktasknotificationmessage)（TypeScript）または [`TaskNotificationMessage`](/docs/ja/agent-sdk/python#tasknotificationmessage)（Python）を含むものは、バックグラウンドコマンドやサブエージェントなどのバックグラウンドタスクを報告します。メッセージストリーム内では、todo アクティビティがアシスタントメッセージ内の `tool_use` ブロックとして表示されます。
</Note>

<h3 id="monitor-todo-changes">
  Todo 変更を監視する
</h3>

次の例はアシスタントストリームで `TaskCreate` と `TaskUpdate` の `tool_use` ブロックを監視し、各新しいタスクの件名を含む `+` 行と各ステータス変更のタスク ID と新しいステータスを含む更新行を出力します。タスク活動のログが必要で、レンダリングされた表示ではない場合、この形状を使用してください。`+` 行は割り当てられた ID を含まないため、このログは更新を作成に一致させることはできません。その相関を保つには、[リアルタイムで進捗を表示する](#display-progress-in-real-time)のように ID をキャプチャしてください。

ストリーミングされた `tool_use` 入力は、モデルが発行した生の形状です。Claude Code は実行前にいくつかの近いが正確でないキー名を修復し、`id` または `task_id` を `taskId` にマッピングし、`active_form` を `activeForm` にマッピングしますが、その修復はストリームに反映されません。以下の両方の例のように、常に正規名が存在すると仮定するのではなく、`TaskUpdate` 入力フィールドを防御的に読み取ります。

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
  リアルタイムで進捗を表示する
</h3>

次の例はアシスタントストリームで `TaskCreate` と `TaskUpdate` の `tool_use` ブロックを監視し、`TaskTracker` クラスでタスク ID でキー付けされたタスクのマップを保持し、変更のたびに進捗サマリーを再レンダリングします。サマリーは完了したタスクと進行中のタスクをカウントし、各アクティブなアイテムの `activeForm` ラベルを `subject` の代わりに表示します。アプリケーションが各イベントをログするのではなく進捗表示を保持する場合、この形状を使用してください。

割り当てられたタスク ID は `TaskCreate` 入力にはありません。Claude Code は各ツールの構造化された出力を、`tool_result` ブロックを含むユーザーメッセージで、`tool_use_result` フィールドで配信します。`TaskCreate` の場合、そのオブジェクトは TypeScript の [ツール出力タイプ](/docs/ja/agent-sdk/typescript#tool-output-types)の下で `TaskCreateOutput` として文書化され、Python ではフィールドは同じ形状の平文辞書です。トラッカーは `tool_use_id` で各 `tool_result` ブロックをその `tool_use` 呼び出しとペアリングし、ペアリングされたメッセージの `tool_use_result` から `task.id` を読み取ります。Claude は `TaskList` でリストを読み戻すことができ、`TaskGet` で 1 つのタスクの完全な詳細を読み取ることができます。

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
  関連ドキュメント
</h2>

* [Agent SDK リファレンス - TypeScript](/docs/ja/agent-sdk/typescript)：TypeScript SDK のオプション、タイプ、およびツールスキーマ。Task ツール入力および出力タイプを含みます
* [Agent SDK リファレンス - Python](/docs/ja/agent-sdk/python)：Python SDK のオプション、タイプ、およびツールドキュメント
* [ストリーミング入力](/docs/ja/agent-sdk/streaming-vs-single-mode)：2 つの入力モード、およびこれらの例が使用する単一ショット呼び出しの代わりにストリーミング入力を使用する場合
* [Claude にカスタムツールを提供する](/docs/ja/agent-sdk/custom-tools)：SDK のインプロセス MCP サーバーで独自のツールを定義します
