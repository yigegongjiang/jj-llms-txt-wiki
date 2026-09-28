> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# フックを使用してエージェントの動作をインターセプトして制御する

> フックを使用して、エージェント実行の重要なポイントでエージェントの動作をインターセプトしてカスタマイズします

フックはエージェントイベントに応答してコードを実行するコールバック関数です。ツールが呼び出されたり、セッションが開始したり、実行が停止したりするなどのイベントに対応します。フックを使用すると、以下のことができます。

* **危険な操作をブロック**する：破壊的なシェルコマンドや不正なファイルアクセスなど、実行前に危険な操作をブロックします
* **ログと監査**：コンプライアンス、デバッグ、分析のためにすべてのツール呼び出しをログして監査します
* **入力と出力を変換**する：データをサニタイズしたり、認証情報を注入したり、ファイルパスをリダイレクトしたりします
* **人間の承認を要求**する：データベース書き込みや API 呼び出しなどの機密アクションに対して
* **セッションライフサイクルを追跡**する：状態を管理したり、リソースをクリーンアップしたり、通知を送信したりします

<h2 id="how-hooks-work">
  フックの仕組み
</h2>

<Steps>
  <Step title="イベントが発火する">
    エージェント実行中に何かが起こり、SDK がイベントを発火します。ツールが呼び出されようとしている（`PreToolUse`）、ツールが結果を返した（`PostToolUse`）、サブエージェントが開始または停止した、エージェントがアイドル状態である、または実行が完了したなどです。[イベントの完全なリスト](#available-hooks)を参照してください。
  </Step>

  <Step title="SDK が登録されたフックを収集する">
    SDK は、そのイベントタイプに登録されたフックをチェックします。これには、`options.hooks` に渡すコールバックフックと、対応する [`settingSources`](/docs/ja/agent-sdk/typescript#settingsource) または [`setting_sources`](/docs/ja/agent-sdk/python#settingsource) エントリが有効になっているときの設定ファイルからのシェルコマンドフックが含まれます。これはデフォルトの `query()` オプションで有効になっています。
  </Step>

  <Step title="マッチャーがどのフックを実行するかをフィルタリングする">
    フックに [`matcher`](#matchers) パターン（`"Write|Edit"` など）がある場合、SDK はそれをイベントのターゲット（たとえば、ツール名）に対してテストします。マッチャーのないフックは、そのタイプのすべてのイベントに対して実行されます。
  </Step>

  <Step title="コールバック関数が実行される">
    各マッチングフックの[コールバック関数](#callback-functions)は、何が起こっているかについての入力を受け取ります。ツール名、その引数、セッション ID、およびその他のイベント固有の詳細です。
  </Step>

  <Step title="コールバックが決定を返す">
    任意の操作（ログ、API 呼び出し、検証）を実行した後、コールバックは[出力オブジェクト](#outputs)を返します。これはエージェントに何をするかを指示します。操作を許可する、ブロックする、入力を変更する、または会話にコンテキストを注入するなどです。
  </Step>
</Steps>

次の例は、これらのステップをまとめたものです。`PreToolUse` フック（ステップ 1）を `"Write|Edit"` マッチャー（ステップ 3）で登録して、コールバックがファイル書き込みツールに対してのみ発火するようにします。トリガーされると、コールバックはツールの入力（ステップ 4）を受け取り、ファイルパスが `.env` ファイルをターゲットにしているかどうかをチェックし、`permissionDecision: "deny"` を返して操作をブロックします（ステップ 5）。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # ツール呼び出しの詳細を受け取るフックコールバックを定義する
  async def protect_env_files(input_data, tool_use_id, context):
      # ツールの入力引数からファイルパスを抽出する
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # .env ファイルをターゲットにしている場合は操作をブロックする
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # 空のオブジェクトを返して操作を許可する
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # PreToolUse イベントのフックを登録する
              # マッチャーは Write と Edit ツール呼び出しのみにフィルタリングする
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Create a .env file with the standard local development database configuration")
          async for message in client.receive_response():
              # アシスタントとリザルトメッセージをフィルタリングする
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // HookCallback 型でフックコールバックを定義する
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // 型安全性のために入力を特定のフック型にキャストする
    const preInput = input as PreToolUseHookInput;

    // tool_input をキャストしてそのプロパティにアクセスする（SDK では unknown として型付けされている）
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // .env ファイルをターゲットにしている場合は操作をブロックする
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // 空のオブジェクトを返して操作を許可する
    return {};
  };

  for await (const message of query({
    prompt: "Create a .env file with the standard local development database configuration",
    options: {
      hooks: {
        // PreToolUse イベントのフックを登録する
        // マッチャーは Write と Edit ツール呼び出しのみにフィルタリングする
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // アシスタントとリザルトメッセージをフィルタリングする
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

どちらのスクリプトを実行しても、Claude は `.env` ファイルを作成しようとし、フックはツール呼び出しを拒否し、Claude の最終的な応答は `.env` ファイルを作成できないことを説明します。

<h2 id="available-hooks">
  利用可能なフック
</h2>

SDK はエージェント実行のさまざまなステージのフックを提供します。一部のフックは両方の SDK で利用可能ですが、その他は TypeScript のみです。

| フックイベント                                                | Python SDK | TypeScript SDK | トリガーされる条件                                                                         | 使用例                                                                                                                       |
| ------------------------------------------------------ | ---------- | -------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | はい         | はい             | ツール呼び出しリクエスト（ブロックまたは変更可能）                                                         | 危険なシェルコマンドをブロックする                                                                                                         |
| `PostToolUse`                                          | はい         | はい             | ツール実行結果                                                                           | すべてのファイル変更を監査証跡にログする                                                                                                      |
| `PostToolUseFailure`                                   | はい         | はい             | ツール実行失敗                                                                           | ツールエラーを処理またはログする                                                                                                          |
| `PostToolBatch`                                        | いいえ        | はい             | ツール呼び出しの完全なバッチが解決される。次のモデル呼び出しの前に 1 回                                             | バッチ全体に対して規約を 1 回注入する                                                                                                      |
| `UserPromptSubmit`                                     | はい         | はい             | ユーザープロンプト送信                                                                       | プロンプトに追加のコンテキストを注入する                                                                                                      |
| [`UserPromptExpansion`](/docs/ja/hooks#userpromptexpansion) | いいえ        | はい             | ユーザーが入力したコマンド、または MCP プロンプトが Claude に到達する前にプロンプトに展開される。Claude がスキル自体を呼び出すときは発火しない | コマンドの直接呼び出しをブロックするか、スキルが入力されたときにコンテキストを追加する                                                                               |
| `MessageDisplay`                                       | いいえ        | はい             | テキスト付きのアシスタントメッセージが完了する。メッセージごとに 1 回、完全なメッセージテキスト付き                               | 表示されたテキストを編集または再フォーマットする（トランスクリプトは変更しない）                                                                                  |
| `Stop`                                                 | はい         | はい             | エージェント実行停止                                                                        | 終了前にセッション状態を保存する                                                                                                          |
| `StopFailure`                                          | いいえ        | はい             | ターンが通常の停止ではなく API エラーで終了する                                                        | 失敗をログするか、アラートを送信する                                                                                                        |
| `SubagentStart`                                        | はい         | はい             | サブエージェント初期化                                                                       | 並列タスク生成を追跡する                                                                                                              |
| `SubagentStop`                                         | はい         | はい             | サブエージェント完了                                                                        | 並列タスクから結果を集約する                                                                                                            |
| `PreCompact`                                           | はい         | はい             | 会話圧縮リクエスト                                                                         | 要約する前に完全なトランスクリプトをアーカイブする                                                                                                 |
| `PostCompact`                                          | いいえ        | はい             | 会話圧縮が完了する                                                                         | 生成されたサマリーをログする                                                                                                            |
| [`PreModelSwitch`](/docs/ja/hooks#premodelswitch)           | いいえ        | はい             | リクエストされたモデルスイッチ。実行前（ブロック可能）                                                       | 特定のモデルへの切り替えをブロックする                                                                                                       |
| [`PostModelSwitch`](/docs/ja/hooks#postmodelswitch)         | いいえ        | はい             | セッションのモデルが変更される。自動フォールバックを含む                                                      | 新しいモデルに対して Claude モデル固有のガイダンスを提供する                                                                                        |
| `PermissionRequest`                                    | はい         | はい             | ツール呼び出しが権限決定を必要とする                                                                | カスタム権限処理                                                                                                                  |
| `PermissionDenied`                                     | いいえ        | はい             | オートモードがツール呼び出しを拒否する。分類器の判定がない拒否を含む                                                | 拒否をログするか、モデルに再試行できることを伝える。Claude Code は判定なしの拒否に対して `retry: true` を無視する。[PermissionDenied](/docs/ja/hooks#permissiondenied) を参照 |
| `SessionStart`                                         | いいえ        | はい             | セッション初期化                                                                          | ログとテレメトリを初期化する                                                                                                            |
| `SessionEnd`                                           | いいえ        | はい             | セッション終了                                                                           | 一時的なリソースをクリーンアップする                                                                                                        |
| `Notification`                                         | はい         | はい             | エージェントステータスメッセージ                                                                  | エージェントステータス更新を Slack または PagerDuty に送信する                                                                                  |
| `Setup`                                                | いいえ        | はい             | セッション設定/メンテナンス                                                                    | 初期化タスクを実行する                                                                                                               |
| `TeammateIdle`                                         | いいえ        | はい             | チームメイトがアイドル状態になる                                                                  | 作業を再割り当てするか通知する                                                                                                           |
| `TaskCreated`                                          | いいえ        | はい             | `TaskCreate` ツール経由でタスクが作成される                                                      | タスク命名規約を強制する                                                                                                              |
| [`TaskCompleted`](/docs/ja/hooks#taskcompleted)             | いいえ        | はい             | タスクが完了としてマークされる                                                                   | タスクが閉じる前にテストに合格することを要求する                                                                                                  |
| `Elicitation`                                          | いいえ        | はい             | MCP サーバーがタスク中にユーザー入力をリクエストする                                                      | MCP 入力リクエストにプログラムで応答する                                                                                                    |
| `ElicitationResult`                                    | いいえ        | はい             | ユーザーが MCP エリシテーションに応答する                                                           | サーバーに返される前に応答を変更またはブロックする                                                                                                 |
| `ConfigChange`                                         | いいえ        | はい             | 設定ファイル変更                                                                          | 設定を動的に再ロードする                                                                                                              |
| `InstructionsLoaded`                                   | いいえ        | はい             | `CLAUDE.md` またはルールファイルがコンテキストにロードされる                                              | どの命令ファイルがロードされるかを監査する                                                                                                     |
| `WorktreeCreate`                                       | いいえ        | はい             | Git ワークツリー作成                                                                      | 分離されたワークスペースを追跡する                                                                                                         |
| `WorktreeRemove`                                       | いいえ        | はい             | Git ワークツリー削除                                                                      | ワークスペースリソースをクリーンアップする                                                                                                     |
| `CwdChanged`                                           | いいえ        | はい             | セッション中に作業ディレクトリが変更される                                                             | ディレクトリごとに環境変数を再ロードする                                                                                                      |
| `FileChanged`                                          | いいえ        | はい             | 監視対象ファイルが変更、作成、または削除される                                                           | プロジェクトファイルが変更されたときに設定を再ロードする                                                                                              |
| `DirectoryAdded`                                       | いいえ        | はい             | セッション中に作業ディレクトリが追加される                                                             | セッション中に追加されたリポジトリの依存関係をインストールする                                                                                           |

<h2 id="configure-hooks">
  フックを設定する
</h2>

フックを設定するには、エージェントオプション（Python では `ClaudeAgentOptions`、TypeScript では `options` オブジェクト）の `hooks` フィールドに渡します。このスニペットは、上記の例から `protect_env_files`（Python）または `protectEnvFiles`（TypeScript）のようなフックコールバックを既に定義していることを前提としています。

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

`hooks` オプションは辞書（Python）またはオブジェクト（TypeScript）です。ここで：

* **キー**は[フックイベント名](#available-hooks)です（例：`'PreToolUse'`、`'PostToolUse'`、`'Stop'`）
* **値**は[マッチャー](#matchers)の配列です。各マッチャーには、オプションのフィルタパターンと[コールバック関数](#callback-functions)が含まれます

<h3 id="matchers">
  マッチャー
</h3>

マッチャーを使用して、コールバックがいつ発火するかをフィルタリングします。`matcher` フィールドは、フックイベントタイプに応じて異なる値に対してマッチングされます。たとえば、ツールベースのフックはツール名に対してマッチングされ、`Notification` フックは通知タイプに対してマッチングされます。

SDK マッチャーは[設定ファイルのマッチャー](/docs/ja/hooks#matcher-patterns)と同じルールに従います。そのセクションでは、正確な文字列と正規表現の評価パス、バージョン要件、および各イベントタイプのマッチャー値を文書化しています。

| オプション     | 型                | デフォルト       | 説明                                                                                                                                                                                                                                                                                                                                                  |
| --------- | ---------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `matcher` | `string`         | `undefined` | イベントのフィルタフィールドに対してマッチングされるパターン。[設定ファイルのマッチャーのルール](/docs/ja/hooks#matcher-patterns)に従います。ツールフックの場合、これはツール名です。組み込みツールには `Bash`、`Read`、`Write`、`Edit`、`Glob`、`Grep`、`WebFetch`、`Agent` などが含まれます（完全なリストについては[ツール入力型](/docs/ja/agent-sdk/typescript#tool-input-types)を参照）。MCP ツールはパターン `mcp__<server>__<action>` を使用します。ここで `<server>` は `mcpServers` 設定で使用するキーです。 |
| `hooks`   | `HookCallback[]` | -           | 必須。パターンがマッチしたときに実行するコールバック関数の配列                                                                                                                                                                                                                                                                                                                     |
| `timeout` | `number`         | `undefined` | タイムアウト（秒単位）。省略した場合、Claude Code は[イベントのデフォルトタイムアウト](#hook-timeout)を適用します。SDK コールバックは `command` フックのデフォルトに従います                                                                                                                                                                                                                                        |

可能な限り `matcher` パターンを使用して特定のツールをターゲットにします。`'Bash'` のマッチャーは Bash コマンドに対してのみ実行されますが、パターンを省略するとコールバックはそのイベントのすべての発生に対して実行されます。セッションが行うすべてのツール呼び出しをログするために、意図的にパターンを省略します。

<h3 id="callback-functions">
  コールバック関数
</h3>

<h4 id="inputs">
  入力
</h4>

すべてのフックコールバックは 3 つの引数を受け取ります。

* **入力データ：** イベント詳細を含む型付きオブジェクト。各フック型には独自の入力形状があります。たとえば、`PreToolUseHookInput` には `tool_name` と `tool_input` が含まれ、`NotificationHookInput` には `message` が含まれます。[TypeScript](/docs/ja/agent-sdk/typescript#hookinput) および [Python](/docs/ja/agent-sdk/python#hookinput) SDK リファレンスで完全な型定義を参照してください。
  * すべてのフック入力は `session_id`、`cwd`、および `hook_event_name` を共有します。
  * `agent_id` と `agent_type` は、フックがサブエージェント内で発火するときに入力されます。TypeScript では、これらはベースフック入力にあり、すべてのフック型で利用可能です。Python では、これらは `PreToolUse`、`PostToolUse`、`PostToolUseFailure`、および `PermissionRequest` のオプションフィールドであり、`SubagentStart` および `SubagentStop` の必須フィールドです。
* **ツール使用 ID**（`str | None` / `string | undefined`）：同じツール呼び出しの `PreToolUse` と `PostToolUse` イベントを相関させます。
* **コンテキスト：** TypeScript では、キャンセル用の `signal` プロパティ（`AbortSignal`）を含みます。Python では、この引数は将来の使用のために予約されています。

<h4 id="outputs">
  出力
</h4>

コールバックは 2 つのカテゴリのフィールドを持つオブジェクトを返します。

* **トップレベルフィールド**はすべてのイベントで受け入れられます。`systemMessage` はユーザーにメッセージを表示し、`continue`（Python では `continue_`）はこのフック後にエージェントが実行を続けるかどうかを決定します。一部のイベントはこれらを破棄するか、別の場所に配信します。各[イベントのセクション](/docs/ja/hooks#hook-events)はフックページでそれらがどこに着地するかを説明しています。
* **`hookSpecificOutput`** は現在の操作を制御します。内部に設定するフィールドはフックイベントタイプに依存します。
  * `PreToolUse` フックの場合、ここで `permissionDecision`（`"allow"`、`"deny"`、`"ask"`、または `"defer"`）、`permissionDecisionReason`、および `updatedInput` を設定します。`"defer"` を返すとクエリが終了し、[後で再開](/docs/ja/hooks#defer-a-tool-call-for-later)できます。
  * `PostToolUse` フックの場合、`additionalContext` を設定してツール結果に情報を追加できます。Claude がそれを見る前にツールの出力を置き換えるには、`updatedToolOutput` を設定します。これは両方の SDK のすべてのツールで機能します。古い `updatedMCPToolOutput` フィールドは MCP ツール出力のみを置き換え、非推奨です。
  * TypeScript SDK では、`PostToolUse` コールバックは `classifierContext` を返すこともできます。これはツール呼び出しの結果に関する短いメモで、[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)権限分類器用です。コールバックはアプリケーション独自のプロセスで実行されるため、分類器はメモで中継するユーザーステートメントをユーザーの意図として重視する可能性があります。このフィールドは TypeScript Agent SDK v0.3.236 以降が必要です。[自動モード分類器の結果に注釈を付ける](/docs/ja/hooks#annotate-a-result-for-the-auto-mode-classifier)は長さの上限、同期のみのルール、およびメモに何を入れないかをカバーしています。

変更なしで操作を許可するには `{}` を返します。SDK コールバックフックは、[Claude Code シェルコマンドフック](/docs/ja/hooks#json-output)と同じ JSON 出力形式を使用します。これはすべてのフィールドとイベント固有のオプションを文書化しています。SDK 型定義については、[TypeScript](/docs/ja/agent-sdk/typescript#synchookjsonoutput) および [Python](/docs/ja/agent-sdk/python#synchookjsonoutput) SDK リファレンスを参照してください。

<Note>
  複数のフックまたはパーミッションルールが適用される場合、`deny` は `defer` より優先され、`defer` は `ask` より優先され、`ask` は `allow` より優先されます。いずれかのフックが `deny` を返す場合、他のフックに関係なく操作はブロックされます。
</Note>

<h4 id="asynchronous-output">
  非同期出力
</h4>

デフォルトでは、エージェントはフックが返されるのを待ってから続行します。フックがログやウェブフック送信などの副作用を実行し、エージェントの動作に影響を与える必要がない場合、代わりに非同期出力を返すことができます。これはエージェントに、フックが完了するのを待たずに即座に続行するよう指示します。このスニペットでは、Python の `send_to_logging_service` と TypeScript の `sendToLoggingService` は、定義する任意のログ関数の代わりです。

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # バックグラウンドタスクを開始してから即座に返す
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // バックグラウンドタスクを開始してから即座に返す
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| フィールド          | 型        | 説明                                                                      |
| -------------- | -------- | ----------------------------------------------------------------------- |
| `async`        | `true`   | 非同期モードを通知します。エージェントは待たずに続行します。Python では、予約キーワードを避けるために `async_` を使用します。 |
| `asyncTimeout` | `number` | バックグラウンド操作のオプションのタイムアウト（ミリ秒単位）                                          |

<Note>
  非同期出力はエージェントが既に先に進んでいるため、ブロック、変更、またはコンテキストを注入することはできません。ログ、メトリクス、または通知などの副作用にのみ使用します。
</Note>

<h2 id="examples">
  例
</h2>

このセクションの例の多くはコールバック関数のみを示しています。実行するには、[フックの設定](#configure-hooks)に示されているように、コールバックをオプションの `hooks` フィールドの対応するイベントに登録してください。

<h3 id="modify-tool-input">
  ツール入力を変更する
</h3>

この例は Write ツール呼び出しをインターセプトし、`file_path` 引数を書き直して `/sandbox` を先頭に追加し、すべてのファイル書き込みをサンドボックス化されたディレクトリにリダイレクトします。コールバックは変更されたパスを含む `updatedInput` と `permissionDecision: 'allow'` を返して、書き直された操作を自動承認します。

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  `updatedInput` を `permissionDecision: 'allow'` と組み合わせて変更された入力を自動承認するか、`permissionDecision: 'ask'` を使用してユーザーに表示します。`permissionDecision` を省略した場合、変更された入力は依然として適用され、通常の権限評価を通じて流れます。`'defer'` を使用する場合、`updatedInput` は無視されます。元の `tool_input` を変更するのではなく、常に新しいオブジェクトを返してください。
</Note>

リダイレクトを確認するには、プレフィックスを書き込み可能なパス（`./sandbox` または `/tmp/sandbox` など）に設定します（macOS はルートレベルの `/sandbox` ディレクトリの作成を許可していません）。その後、エージェントにファイルを書き込むよう指示します。メッセージストリーム内の Write ツールの結果は、Claude が要求したパスではなく、サンドボックスプレフィックス付きのパスを示します。

<h3 id="add-context-and-block-a-tool">
  コンテキストを追加してツールをブロックする
</h3>

この例は `/etc` ディレクトリへの書き込みをブロックし、その理由をモデルとユーザーの両方に説明します。

* `permissionDecision: 'deny'` はツール呼び出しを停止します。
* `permissionDecisionReason` はモデルに理由を伝えるため、再試行を避けます。
* `systemMessage` はユーザーに何が起こったかを表示します。

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  特定のツールを自動承認する
</h3>

デフォルトでは、エージェントは特定のツールを使用する前に権限を求めるプロンプトを表示する場合があります。この例は `permissionDecision: 'allow'` を返すことで読み取り専用ファイルシステムツール（Read、Glob、Grep）を自動承認し、ユーザーの確認なしで実行できるようにしながら、他のすべてのツールは通常の権限チェックの対象のままにします。

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  複数のフックを登録する
</h3>

イベントが発火すると、すべての一致するフックが並列で実行されます。権限決定については、最も制限的な結果が適用されます。単一の `deny` は他のフックが何を返すかに関わらずツール呼び出しをブロックします。完了順序は非決定的であるため、別のフックが最初に実行されたことに依存するのではなく、各フックが独立して動作するように記述してください。

以下の例は、すべてのツール呼び出しに対して 3 つの独立したチェックを登録します。

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  マルチツールマッチャーでフィルタリングする
</h3>

マルチツールマッチャーを使用して、関連するツール間で 1 つのコールバックを共有します。この例は異なるスコープを持つ 3 つのマッチャーを登録します。

* パイプで区切られた正確なリスト（`Write|Edit|NotebookEdit`）は、ファイル変更ツールに対してのみ `file_security_hook` をトリガーします。
* 正規表現（`^mcp__`）は、`mcp__` で始まる名前を持つ MCP ツールに対して `mcp_audit_hook` をトリガーします。
* 省略されたマッチャーは、名前に関わらずすべてのツール呼び出しに対して `global_logger` をトリガーします。

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|NotebookEdit", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|NotebookEdit", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  サブエージェントアクティビティを追跡する
</h3>

`SubagentStop` フックを使用して、サブエージェントが作業を完了したときを監視します。[TypeScript](/docs/ja/agent-sdk/typescript#hookinput) および [Python](/docs/ja/agent-sdk/python#hookinput) SDK リファレンスで完全な入力タイプを参照してください。この例は、サブエージェントが完了するたびに概要をログに記録します。

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  フックから HTTP リクエストを実行する
</h3>

フックは HTTP リクエストなどの非同期操作を実行できます。フックの内部でエラーをキャッチし、伝播させないようにしてください。

この例は各ツール完了後にウェブフックを送信し、どのツールが実行されたかと実行時刻をログに記録します。フックは失敗したウェブフックからのエラーをキャッチします。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

フックが発火することを確認するには、ウェブフック URL をウォッチできるエンドポイントに指定し、ツールを使用するプロンプトを送信します。フックは各ツール完了後にツール名とタイムスタンプを含む POST を送信します。

<h3 id="forward-notifications-to-slack">
  Slack に通知を転送する
</h3>

`Notification` フックを使用して、エージェントからのシステム通知を受け取り、外部サービスに転送します。SDK セッションでは、Claude Code は以下の通知タイプに対してこのフックを実行します。

* [`permission_prompt`](/docs/ja/hooks#notification) は、権限リクエストが [`canUseTool` コールバック](/docs/ja/agent-sdk/user-input)で約 6 秒待機した後に 1 回発火します。TypeScript Agent SDK v0.3.233 以降または Python Agent SDK v0.2.139 以降が必要です。
* ユーザープロンプト引き出しフロー用の `elicitation_complete` および `elicitation_response`

Claude Code は、SDK セッションが実行しないインタラクティブ UI から `idle_prompt`、`auth_success`、`elicitation_dialog` などの他のタイプを発行します。

各通知には、人間が読める説明を含む `message` フィールドと、オプションで `title` が含まれます。

この例は、すべての通知を Slack チャネルに転送します。[Slack 受信ウェブフック URL](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/) が必要です。これは、Slack ワークスペースにアプリを追加し、受信ウェブフックを有効にすることで作成します。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

`Notification` イベントが発火すると、フックは通知の `message` を `Agent status:` というプレフィックス付きでウェブフックが対象とするチャネルに投稿します。

<h2 id="fix-common-issues">
  一般的な問題を修正する
</h2>

<h3 id="hook-not-firing">
  フックが発火しない
</h3>

* フックイベント名が正しく、大文字と小文字が区別されていることを確認してください（`preToolUse` ではなく `PreToolUse`）
* マッチャーパターンがツール名と正確に一致していることを確認してください
* フックが `options.hooks` の正しいイベントタイプの下にあることを確認してください
* マッチャーをサポートする非ツールフック（`Notification` や `SubagentStop` など）の場合、マッチャーは異なるフィールドに対してマッチし、`Stop` はマッチャーを完全に無視します（[マッチャーパターン](/docs/ja/hooks#matcher-patterns)を参照）
* エージェントが [`max_turns`](/docs/ja/agent-sdk/python#claudeagentoptions) 制限に達した場合、セッションが終了してからフックが実行される前にセッションが終了するため、フックが発火しない可能性があります

<h3 id="matcher-not-filtering-as-expected">
  マッチャーが期待通りにフィルタリングしない
</h3>

マッチャーはツール名のみをマッチし、ファイルパスや他の引数はマッチしません。ファイルパスでフィルタリングするには、フック内で `tool_input.file_path` を確認してください：

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  フックタイムアウト
</h3>

Claude Code は各コールバックをタイムアウト付きで実行します。タイムアウトは `HookMatcher` の `timeout` フィールドで秒単位で設定します。設定しない場合、Claude Code はイベントのデフォルトを使用します：ほとんどのイベントで 600 秒、`UserPromptSubmit`、`PreModelSwitch`、`PostModelSwitch` で 30 秒、`MessageDisplay` で 10 秒です。Claude Code は `SessionEnd` コールバックをシャットダウン中に実行します。これは短い [SessionEnd タイムアウト予算](/docs/ja/hooks#sessionend-input)（デフォルトで 1.5 秒）の下で実行されます。

コールバックがタイムアウトを超過した場合、Claude Code はそれをキャンセルし、その出力を破棄し、セッションはハングするのではなく続行します。次に何が起こるかはイベントによって異なります：

* `PreToolUse`：Claude Code はツール呼び出しを実行せず、Claude はフックがタイムアウト前に応答しなかったことを示すツール結果を受け取り、ターンが続行されます。別の `PreToolUse` フックが明示的な拒否を返した場合、Claude はタイムアウトエラーの代わりにその拒否を受け取ります。v2.1.210 より前では、Claude Code はタイムアウトを Claude にユーザー拒否として報告していたため、無人セッションは停止して入力を待つようになっていました。
* `PostToolUse` および `PostToolUseFailure`：Claude Code はツール結果を保持し、ターンが続行されます。
* `UserPromptSubmit` および [`UserPromptExpansion`](/docs/ja/hooks#userpromptexpansion)：Claude Code はフックとタイムアウトを名前で示すメッセージでプロンプトをブロックし、セッションが続行されます。これらのイベントのコールバックはポリシーゲートとして機能できるため、Claude Code はタイムアウトしたプロンプトをスクリーニングなしで通すことはありません。v2.1.208 より前では、これらのイベントのコールバックがタイムアウトした場合、Claude Code はクエリを `error_during_execution` で終了していました。
* `Stop` および `SubagentStop`：タイムアウトしたコールバックは決定を返さないとカウントされます。エージェントまたはサブエージェントは、そのコールバックがそれを許可したかのように停止し、イベントの他のフックからの決定が適用されます。Claude Code v2.1.273 より前では、タイムアウトした `Stop` または `SubagentStop` コールバックは失敗したフック実行としてカウントされ、Claude Code はイベントの他のフックの決定を破棄していました。
* `SessionStart`：タイムアウトしたコールバックは出力を返さないとカウントされ、セッションは他の `SessionStart` フックの出力で続行されます。
* `PreModelSwitch`：Claude Code はモデルスイッチをブロックします。応答しないフックはスイッチを承認していません。
* `Notification`、`PreCompact`、`PostModelSwitch` などの他のイベント：Claude Code は失敗をログに記録して続行します。

メインセッションで `Stop` または `SessionStart` コールバックが初めてタイムアウトした場合、Claude Code はメッセージストリームに [`SDKInformationalMessage`](/docs/ja/agent-sdk/typescript#sdkinformationalmessage) も追加します。これはセッションを駆動するアプリが応答しなかったことを示します。その後のタイムアウトはアプリが応答しない間、そのメッセージを繰り返しません。

コールバックが保留中の間にクエリを中断した場合、Claude Code は保留中のツール呼び出しをキャンセルします。v2.1.208 より前では、`PreToolUse` コールバックが保留中の間に中断した場合、ツール呼び出しは引き続き進行する可能性がありました。

コールバックがより多くの時間を必要とする場合は、その `HookMatcher` で高い `timeout` を設定してください。TypeScript では、タイムアウトが発火したときにキャンセルを適切に処理するために、3 番目のコールバック引数から `AbortSignal` を使用してください。

<h3 id="tool-blocked-unexpectedly">
  ツールが予期せずブロックされた
</h3>

* すべての `PreToolUse` フックで `permissionDecision: 'deny'` の戻り値を確認してください
* フックにログを追加して、返している `permissionDecisionReason` を確認してください
* マッチャーパターンが広すぎないことを確認してください：空のマッチャーはすべてのツールにマッチします

<h3 id="modified-input-not-applied">
  変更された入力が適用されない
</h3>

* `updatedInput` がトップレベルではなく `hookSpecificOutput` の内側にあることを確認してください：

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* `updatedInput` を `permissionDecision: 'defer'` と組み合わせないでください。これは変更された入力を削除します。`permissionDecision` を省略することは問題ありません：変更された入力は通常の権限評価を通じて引き続き適用されます。また、`'allow'` を返して変更された入力を自動承認するか、`'ask'` を返してユーザーに承認を求めることもできます

* `hookSpecificOutput` に `hookEventName` を含めて、出力がどのフックタイプ用であるかを識別してください

<h3 id="session-hooks-not-available-in-python">
  Python でセッションフックが利用できない
</h3>

`SessionStart` と `SessionEnd` は TypeScript で SDK コールバックフックとして登録できますが、その `HookEvent` タイプがそれらを省略しているため、Python SDK では利用できません。Python では、`.claude/settings.json` などの設定ファイルで定義された [シェルコマンドフック](/docs/ja/hooks#hook-events)としてのみ利用できます。SDK アプリケーションからシェルコマンドフックをロードするには、[`setting_sources`](/docs/ja/agent-sdk/python#settingsource) または [`settingSources`](/docs/ja/agent-sdk/typescript#settingsource) で適切な設定ソースを含めてください：

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Python SDK コールバックとして初期化ロジックを実行するには、`client.receive_response()` からの最初のメッセージをトリガーとして使用してください。

<h3 id="subagent-permission-prompts-multiplying">
  サブエージェント権限プロンプトが増加する
</h3>

複数のサブエージェントをスポーンする場合、各サブエージェントは独自のツール呼び出しに対して権限を個別にリクエストする可能性があります。繰り返されるプロンプトを避けるには、`PreToolUse` フックを使用して特定のツールを自動承認するか、権限ルールを設定してください。サブエージェントは [親会話から権限ルールを継承](/docs/ja/sub-agents#permission-modes)します。

<h3 id="recursive-hook-loops-with-subagents">
  サブエージェントを使用した再帰的フックループ
</h3>

サブエージェントをスポーンする `UserPromptSubmit` フックは、それらのサブエージェントが同じフックをトリガーする場合、無限ループを作成できます。これを防ぐには：

* 共有変数またはセッション状態を使用して、既にサブエージェント内にいるかどうかを追跡してください
* フックをトップレベルエージェントセッションのみで実行するようにスコープしてください

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage が出力に表示されない
</h3>

`systemMessage` フィールドはモデルではなく、ユーザーにメッセージを表示します。Claude Code v2.1.227 以降では、フックの `systemMessage` はメッセージストリームに [`SDKInformationalMessage`](/docs/ja/agent-sdk/typescript#sdkinformationalmessage) として表示される可能性があります。表示されるかどうかはイベントによって異なります。フックページの各 [イベントのセクション](/docs/ja/hooks#hook-events)は、出力がどのように表示されるかを説明しています。代わりにモデルにコンテキストを渡すには、[`additionalContext`](/docs/ja/hooks#add-context-for-claude) を返してください。

v2.1.227 より前では、SDK はメッセージストリームのフック出力を `SessionStart` および `Setup` フックのみで表示していました。他のイベントの場合、出力は [`includeHookEvents`](/docs/ja/agent-sdk/typescript#options)（Python では `include_hook_events`）が追加するライフサイクルイベントにのみ表示されていました。そのオプションのエントリは、各フックイベントが生成するライフサイクルイベントをカバーしています。

フック決定をアプリケーションに確実に表示する必要がある場合は、それらを個別にログに記録するか、専用の出力チャネルを使用してください。

<h2 id="related-resources">
  関連リソース
</h2>

* [Claude Code フックリファレンス](/docs/ja/hooks)：完全な JSON 入力/出力スキーマ、イベント文書、およびマッチャーパターン
* [Claude Code フックガイド](/docs/ja/hooks-guide)：シェルコマンドフックの例とウォークスルー
* [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)：フック型、入力/出力定義、および設定オプション
* [Python SDK リファレンス](/docs/ja/agent-sdk/python)：フック型、入力/出力定義、および設定オプション
* [パーミッション](/docs/ja/agent-sdk/permissions)：エージェントが何をできるかを制御します
* [カスタムツール](/docs/ja/agent-sdk/custom-tools)：エージェント機能を拡張するツールを構築します
