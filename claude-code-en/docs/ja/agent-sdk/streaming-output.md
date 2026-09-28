> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# リアルタイムでレスポンスをストリーミングする

> テキストとツール呼び出しがストリーミングされるときに、Agent SDK からリアルタイムレスポンスを取得します

デフォルトでは、Agent SDK は Claude がレスポンスの生成を完了した後に、完全な `AssistantMessage` オブジェクトを返します。テキストとツール呼び出しが生成されるときにインクリメンタルな更新を受け取るには、部分的なメッセージストリーミングを有効にします。

<Tip>
  このページは出力ストリーミング（リアルタイムでトークンを受け取ること）について説明しています。入力モード（メッセージの送信方法）については、[エージェントにメッセージを送信する](/docs/ja/agent-sdk/streaming-vs-single-mode)を参照してください。また、[CLI 経由で Agent SDK を使用してレスポンスをストリーミングする](/docs/ja/headless)こともできます。
</Tip>

<h2 id="enable-streaming-output">
  ストリーミング出力を有効にする
</h2>

ストリーミングを有効にするには、オプションで `include_partial_messages`（Python）または `includePartialMessages`（TypeScript）を `true` に設定します。これにより、SDK は到着した生の API イベントを含む `StreamEvent` メッセージを返すようになり、通常の `AssistantMessage` と `ResultMessage` に加えて返されます。

コードは以下の処理を実行する必要があります：

1. 各メッセージのタイプをチェックして、`StreamEvent` を他のメッセージタイプから区別する
2. `StreamEvent` の場合、`event` フィールドを抽出してそのタイプをチェックする
3. `delta.type` が `text_delta` である `content_block_delta` イベントを探す。これには実際のテキストチャンクが含まれます

以下の例はストリーミングを有効にし、テキストチャンクが到着するときに出力します。ネストされたタイプチェックに注意してください：最初に `StreamEvent`、次に `content_block_delta`、その後 `text_delta` です：

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_response():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Bash", "Read"],
      )

      async for message in query(prompt="List the files in my project", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              if event.get("type") == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "text_delta":
                      print(delta.get("text", ""), end="", flush=True)


  asyncio.run(stream_response())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the files in my project",
    options: {
      includePartialMessages: true,
      allowedTools: ["Bash", "Read"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;
      if (event.type === "content_block_delta") {
        if (event.delta.type === "text_delta") {
          process.stdout.write(event.delta.text);
        }
      }
    }
  }
  ```
</CodeGroup>

<h2 id="streamevent-reference">
  StreamEvent リファレンス
</h2>

部分的なメッセージが有効な場合、生の Claude API ストリーミングイベントがオブジェクトでラップされて返されます。タイプは各 SDK で異なる名前を持ちます：

* **Python**: [`StreamEvent`](/docs/ja/agent-sdk/python#streamevent)（`claude_agent_sdk.types` からインポート）
* **TypeScript**: [`SDKPartialAssistantMessage`](/docs/ja/agent-sdk/typescript#sdkpartialassistantmessage) with `type: 'stream_event'`

どちらも生の Claude API イベントを含み、蓄積されたテキストではありません。テキストデルタを自分で抽出して蓄積する必要があります。

`parent_tool_use_id` フィールドは Python では常に `None`、TypeScript では `null` です。ストリームイベントはメインセッションのみに対して発行されます。サブエージェントからのトークンレベルのデルタは転送されません。出力をサブエージェントに属性付けするには、`parent_tool_use_id` を含む完全なメッセージを使用してください。[サブエージェント呼び出しの検出](/docs/ja/agent-sdk/subagents#detect-subagent-invocation)を参照してください。

Claude Code は、ターンの最初の非 ping ストリームイベントで `user_message_uuid` を設定し、ターンが応答しているメッセージが変わるときに再度設定します。これは [`user_message_uuid`](/docs/ja/agent-sdk/typescript#user_message_uuid) の条件下で行われます。Python の `StreamEvent` はこのフィールドを公開していません。

`event` フィールドには、[Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types) からの生のストリーミングイベントが含まれます。一般的なイベントタイプは以下の通りです：

| イベントタイプ               | 説明                            |
| :-------------------- | :---------------------------- |
| `message_start`       | 新しいメッセージの開始                   |
| `content_block_start` | 新しいコンテンツブロック（テキストまたはツール使用）の開始 |
| `content_block_delta` | コンテンツへのインクリメンタルな更新            |
| `content_block_stop`  | コンテンツブロックの終了                  |
| `message_delta`       | メッセージレベルの更新（停止理由、使用量）         |
| `message_stop`        | メッセージの終了                      |

<h2 id="message-flow">
  メッセージフロー
</h2>

Claude Code は、空でない各コンテンツブロックが完了するたびに `AssistantMessage` を発行します。そのため、テキストブロックとツール呼び出しを含むレスポンスは 2 つの `AssistantMessage` オブジェクトを生成します。各メッセージは独自のコンテンツブロックのみを含み、両方とも同じメッセージ ID を共有します。これは TypeScript では `message.message.id` として、Python では `message.message_id` として読み取ります。部分メッセージが有効な場合、各 `AssistantMessage` はそのブロックの `content_block_stop` イベントの前に到着し、メッセージは以下の順序で受け取ります。

```text theme={null}
StreamEvent (message_start)
StreamEvent (content_block_start) - text block
StreamEvent (content_block_delta) - text chunks...
AssistantMessage - complete text block
StreamEvent (content_block_stop)
StreamEvent (content_block_start) - tool_use block
StreamEvent (content_block_delta) - tool input chunks...
AssistantMessage - complete tool_use block
StreamEvent (content_block_stop)
StreamEvent (message_delta)
StreamEvent (message_stop)
... tool executes ...
... more streaming events for next turn ...
ResultMessage - final result
```

部分メッセージが有効でない場合、`StreamEvent` を除くすべてのメッセージタイプを受け取ります。一般的なタイプには `SystemMessage`（セッション初期化）、`AssistantMessage`（完全なコンテンツブロック）、`ResultMessage`（最終結果）、および会話履歴がコンパクト化されたときを示すコンパクト境界メッセージ（TypeScript では `SDKCompactBoundaryMessage`、Python では subtype `"compact_boundary"` の `SystemMessage`）が含まれます。

<h2 id="stream-tool-calls">
  ツール呼び出しをストリーミングする
</h2>

ツール呼び出しもインクリメンタルにストリーミングされます。ツールが開始されるときを追跡し、生成されるときに入力を受け取り、完了するときを確認できます。以下の例は、現在呼び出されているツールを追跡し、ストリーミングされるときに JSON 入力を蓄積します。3 つのイベントタイプを使用します：

* `content_block_start`: ツール開始
* `content_block_delta` with `input_json_delta`: 入力チャンク到着
* `content_block_stop`: ツール呼び出し完了

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_tool_calls():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Read", "Bash"],
      )

      # 現在のツールを追跡し、その入力 JSON を蓄積
      current_tool = None
      tool_input = ""

      async for message in query(prompt="Read the README.md file", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              event_type = event.get("type")

              if event_type == "content_block_start":
                  # 新しいツール呼び出しが開始
                  content_block = event.get("content_block", {})
                  if content_block.get("type") == "tool_use":
                      current_tool = content_block.get("name")
                      tool_input = ""
                      print(f"Starting tool: {current_tool}")

              elif event_type == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "input_json_delta":
                      # ストリーミングされるときに JSON 入力を蓄積
                      chunk = delta.get("partial_json", "")
                      tool_input += chunk
                      print(f"  Input chunk: {chunk}")

              elif event_type == "content_block_stop":
                  # ツール呼び出し完了 - 最終入力を表示
                  if current_tool:
                      print(f"Tool {current_tool} called with: {tool_input}")
                      current_tool = None


  asyncio.run(stream_tool_calls())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 現在のツールを追跡し、その入力 JSON を蓄積
  let currentTool: string | null = null;
  let toolInput = "";

  for await (const message of query({
    prompt: "Read the README.md file",
    options: {
      includePartialMessages: true,
      allowedTools: ["Read", "Bash"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;

      if (event.type === "content_block_start") {
        // 新しいツール呼び出しが開始
        if (event.content_block.type === "tool_use") {
          currentTool = event.content_block.name;
          toolInput = "";
          console.log(`Starting tool: ${currentTool}`);
        }
      } else if (event.type === "content_block_delta") {
        if (event.delta.type === "input_json_delta") {
          // ストリーミングされるときに JSON 入力を蓄積
          const chunk = event.delta.partial_json;
          toolInput += chunk;
          console.log(`  Input chunk: ${chunk}`);
        }
      } else if (event.type === "content_block_stop") {
        // ツール呼び出し完了 - 最終入力を表示
        if (currentTool) {
          console.log(`Tool ${currentTool} called with: ${toolInput}`);
          currentTool = null;
        }
      }
    }
  }
  ```
</CodeGroup>

<h2 id="build-a-streaming-ui">
  ストリーミング UI を構築する
</h2>

この例は、テキストとツールストリーミングを統合された UI に組み合わせます。エージェントが現在ツールを実行しているかどうかを追跡します（`in_tool` フラグを使用）。ツールの実行中に `[Using Read...]` のようなステータスインジケータを表示します。ツールが実行されていないときはテキストが通常にストリーミングされ、ツール完了は「完了」メッセージをトリガーします。このパターンは、マルチステップエージェントタスク中に進捗を表示する必要があるチャットインターフェースに役立ちます。

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
  from claude_agent_sdk.types import StreamEvent
  import asyncio
  import sys


  async def streaming_ui():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Read", "Bash", "Grep"],
      )

      # 現在ツール呼び出し中かどうかを追跡
      in_tool = False

      async for message in query(
          prompt="Find all TODO comments in the codebase", options=options
      ):
          if isinstance(message, StreamEvent):
              event = message.event
              event_type = event.get("type")

              if event_type == "content_block_start":
                  content_block = event.get("content_block", {})
                  if content_block.get("type") == "tool_use":
                      # ツール呼び出しが開始 - ステータスインジケータを表示
                      tool_name = content_block.get("name")
                      print(f"\n[Using {tool_name}...]", end="", flush=True)
                      in_tool = True

              elif event_type == "content_block_delta":
                  delta = event.get("delta", {})
                  # ツール実行中でないときのみテキストをストリーミング
                  if delta.get("type") == "text_delta" and not in_tool:
                      sys.stdout.write(delta.get("text", ""))
                      sys.stdout.flush()

              elif event_type == "content_block_stop":
                  if in_tool:
                      # ツール呼び出し完了
                      print(" done", flush=True)
                      in_tool = False

          elif isinstance(message, ResultMessage):
              # エージェントがすべての作業を完了
              print(f"\n\n--- Complete ---")


  asyncio.run(streaming_ui())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 現在ツール呼び出し中かどうかを追跡
  let inTool = false;

  for await (const message of query({
    prompt: "Find all TODO comments in the codebase",
    options: {
      includePartialMessages: true,
      allowedTools: ["Read", "Bash", "Grep"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;

      if (event.type === "content_block_start") {
        if (event.content_block.type === "tool_use") {
          // ツール呼び出しが開始 - ステータスインジケータを表示
          process.stdout.write(`\n[Using ${event.content_block.name}...]`);
          inTool = true;
        }
      } else if (event.type === "content_block_delta") {
        // ツール実行中でないときのみテキストをストリーミング
        if (event.delta.type === "text_delta" && !inTool) {
          process.stdout.write(event.delta.text);
        }
      } else if (event.type === "content_block_stop") {
        if (inTool) {
          // ツール呼び出し完了
          console.log(" done");
          inTool = false;
        }
      }
    } else if (message.type === "result") {
      // エージェントがすべての作業を完了
      console.log("\n\n--- Complete ---");
    }
  }
  ```
</CodeGroup>

<h2 id="known-limitations">
  既知の制限事項
</h2>

* **構造化出力**: JSON 結果は最終的な `ResultMessage.structured_output` にのみ表示され、ストリーミングデルタとしては表示されません。詳細は[構造化出力](/docs/ja/agent-sdk/structured-outputs)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

テキストとツール呼び出しをリアルタイムでストリーミングできるようになったので、これらの関連トピックを探索してください：

* [インタラクティブ対ワンショットクエリ](/docs/ja/agent-sdk/streaming-vs-single-mode): ユースケースに応じて入力モードを選択する
* [構造化出力](/docs/ja/agent-sdk/structured-outputs): エージェントから型付き JSON レスポンスを取得する
* [権限](/docs/ja/agent-sdk/permissions): エージェントが使用できるツールを制御する
