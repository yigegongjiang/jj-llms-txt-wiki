> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code をプログラムで実行する

> Agent SDK を使用して、CLI、Python、または TypeScript からプログラムで Claude Code を実行します。

[Agent SDK](/docs/ja/agent-sdk/overview) は、Claude Code を支える同じツール、エージェントループ、およびコンテキスト管理を提供します。スクリプトと CI/CD 用の CLI として、または完全なプログラムによる制御のための [Python](/docs/ja/agent-sdk/python) および [TypeScript](/docs/ja/agent-sdk/typescript) パッケージとして利用できます。

Claude Code を非対話型モードで実行するには、プロンプトと任意の [CLI オプション](/docs/ja/cli-reference) を指定して `-p` を渡します。

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

このページでは、CLI（`claude -p`）経由で Agent SDK を使用することについて説明しています。構造化された出力、ツール承認コールバック、およびネイティブメッセージオブジェクトを備えた Python および TypeScript SDK パッケージについては、[完全な Agent SDK ドキュメント](/docs/ja/agent-sdk/overview) を参照してください。

<h2 id="basic-usage">
  基本的な使用方法
</h2>

任意の `claude` コマンドに `-p`（または `--print`）フラグを追加して、非対話的に実行します。すべての [CLI オプション](/docs/ja/cli-reference) が `-p` と組み合わさるわけではありません。Claude Code は `--bg` を拒否し、タスク説明付きの `--cloud` を拒否します。競合を名前付きエラーで報告します。セッション ID 付きの `--cloud` と `-p` は代わりに [そのクラウドセッションにメッセージをキューイングして終了します](/docs/ja/claude-code-on-the-web#send-follow-ups-from-the-cli)。`-p` と組み合わせることが多いオプションには以下が含まれます。

* `--continue` は [会話を続ける](#continue-conversations) 場合
* `--allowedTools` は [ツールを自動承認する](#auto-approve-tools) 場合
* `--output-format` は [構造化された出力を取得する](#get-structured-output) 場合

この例は、コードベースについて Claude に質問し、応答を出力します。

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code は成功時にコード 0 で終了し、実行が失敗した場合は 0 以外のコードで終了するため、スクリプトは終了ステータスで分岐できます。無効なフラグを渡すと、Claude Code は実行開始前にエラーを stderr に報告します。実行内での失敗（認証の欠落など）が発生した場合、Claude Code は失敗を stdout の結果として出力します。

<h3 id="start-faster-with-bare-mode">
  ベアモードでより高速に開始する
</h3>

`--bare` を追加して、hooks、skills、カスタムコマンド、[サブエージェント](/docs/ja/sub-agents)、インストール済みプラグイン、MCP サーバー、auto memory、および CLAUDE.md の自動検出をスキップすることで、起動時間を短縮します。これがない場合、`claude -p` は対話型セッションと同じ [コンテキスト](/docs/ja/how-claude-code-works#the-context-window) を読み込みます。これには、作業ディレクトリまたは `~/.claude` で設定されたすべてのものが含まれます。

ベアモードは、すべてのマシンで同じ結果が必要な CI とスクリプトに役立ちます。チームメイトの `~/.claude` のフック、またはプロジェクトの `.mcp.json` の MCP サーバーは実行されません。ベアモードはそれらを読み込まないためです。`--add-dir` で指定するディレクトリは部分的な例外です。ベアモードはその `.claude/skills/` フォルダからスキルを読み込みますが、その `.claude/commands/` および `.claude/agents/` フォルダはスキップします。[追加ディレクトリからのスキル](/docs/ja/skills#skills-from-additional-directories) は、何が読み込まれ、何が読み込まれないかについて説明しています。

`--bare` がない場合、`-p` セッションはプロジェクトの `.claude/settings.json` のフックを実行し、その `.mcp.json` のサーバーを接続します。これは信頼したことのないフォルダでも同様です。`-p` セッションはワークスペース信頼ダイアログもサーバーごとの承認プロンプトも表示しません。[フォルダを信頼する前に実行されるもの](/docs/ja/permissions#what-runs-before-you-trust-a-folder) は、`-p` の下での各種リポジトリコンテンツと、それを除外する方法について説明しています。

この例は、ベアモードで 1 回限りの要約タスクを実行し、Read ツールを事前承認して、呼び出しが許可プロンプトなしで完了するようにします。実行前に `ANTHROPIC_API_KEY` を設定してください。ベアモードはサブスクリプションログインを使用しないためです。

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

ベアモードでは、Claude Code は OAuth 認証情報またはシステムキーチェーンを読み込みません。Anthropic API の場合、環境に `ANTHROPIC_API_KEY` を設定します。キーは [Claude Console](https://platform.claude.com) で作成するか、`--settings` JSON に `apiKeyHelper` を指定します。Amazon Bedrock、Google Cloud の Agent Platform、および Microsoft Foundry は通常のプロバイダー認証情報を読み込み続けます。

ベアモードでは Claude は Bash、ファイル読み取り、およびファイル編集ツールにアクセスできます。フラグを使用して必要なコンテキストを渡します。

| 読み込むもの      | 使用するもの                                                 |
| ----------- | ------------------------------------------------------ |
| システムプロンプト追加 | `--append-system-prompt`、`--append-system-prompt-file` |
| 設定          | `--settings <file-or-json>`                            |
| MCP サーバー    | `--mcp-config <file-or-json>`                          |
| カスタムエージェント  | `--agents <json>`                                      |
| プラグイン       | `--plugin-dir <path>`、`--plugin-url <url>`             |

<Note>
  `--bare` はスクリプトおよび SDK 呼び出しの推奨モードであり、将来のリリースで `-p` のデフォルトになります。
</Note>

<h3 id="background-tasks-at-exit">
  終了時のバックグラウンドタスク
</h3>

Claude が `claude -p` 実行中に [バックグラウンド Bash タスク](/docs/ja/tools-reference#bash-tool-behavior) を開始する場合（例えば、開発サーバーまたはウォッチビルド）、そのシェルは Claude が最終結果を返し、stdin が閉じられてから約 5 秒後に終了します。猶予期間により、結果の直後に終了するタスクでも出力を配信できます。

Claude が [サブエージェント](/docs/ja/sub-agents) またはワークフローをバックグラウンドで開始する場合、その結果が最終出力の一部であるため、`claude -p` は代わりにその作業が完了するまで開いたままになります。

デフォルトでは、待機は 10 分間の継続的なアイドル待機後に終了するため、スタックしたサブエージェントまたはワークフローがプロセスを無期限に開いたままにすることはできません。その時点で Claude Code は実行中のものをすべて停止し、その部分的な結果をドロップします。制限を変更するには、[`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/ja/env-vars) を設定するか、`0` に設定して制限なく待機します。

Claude が `claude -p` 実行中に [Monitor](/docs/ja/tools-reference#monitor-tool) ウォッチを開始する場合、Claude Code はウォッチがタイムアウトするか 10 分の上限がその待機を終了するまで、どちらか先に来た方まで待機します。待機中、Claude はウォッチが報告することに応答し続けます。デフォルトでは、ウォッチは Claude が開始してから 5 分後にタイムアウトします。

<h3 id="stop-a-run-with-sigterm">
  SIGTERM で実行を停止する
</h3>

`claude -p` 実行を SIGTERM で停止する場合（例えば `kill` またはプロセススーパーバイザーから）、Claude Code はコード 143 で終了します。Claude Code は進行中のターンを未完了のままにし、そのための結果を記録しません。代わりにターンを終了するには、SIGINT を送信するか、Agent SDK の `interrupt()` を呼び出してから、プロセスを停止します。

SIGTERM では、Claude Code はまだ実行中の Bash コマンドのプロセスツリーを終了します。Claude Code は [`SessionEnd` フック](/docs/ja/hooks#sessionend) を実行して終了します。終了中、Claude Code は新しいツール呼び出しを開始せず、新しいモデルリクエストを送信せず、`SessionEnd` 以外のフックを実行しません。実行がコマンド実行中またはシグナル到着時に許可プロンプトへの回答を待機中だった場合、Claude Code はそのステップを以下のように処理します。

* **コマンド実行中**: Claude Code はコマンドをセッションで killed として記録します。
* **許可プロンプトへの回答を待機中**: プロセスに SIGTERM を送信する場合、Claude Code はプロンプトを未回答のままにします。プログラムが Agent SDK を通じてセッションを閉じる場合、SDK はシグナルを送信する前に Claude Code の入力を終了し、Claude Code は入力が終了するとすぐにプロンプトをキャンセルします。

[セッションを再開](#continue-conversations) する場合、Claude Code は SIGTERM が未完了のままにしたターンを続行します。

<h2 id="examples">
  例
</h2>

これらの例は、一般的な CLI パターンを強調しています。`auth.py` や `build-error.txt` などのファイルを指定するコマンドの場合は、自分のプロジェクトのファイルに置き換えてください。CI やその他のスクリプト環境では、[`--bare`](#start-faster-with-bare-mode) を追加して、Claude Code がホストの hooks、plugins、auto memory、または `CLAUDE.md` を読み込まずに起動するようにしてください。

<h3 id="pipe-data-through-claude">
  Claude にデータをパイプする
</h3>

非対話モードは stdin を読み込むため、他のコマンドラインツールと同様にデータをパイプして応答をリダイレクトできます。

この例は、ビルドログを Claude にパイプして、説明をファイルに書き込みます。

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

`--output-format json` を使用すると、応答ペイロードに `total_cost_usd` とモデルごとのコスト内訳が含まれるため、スクリプト呼び出し元は [usage dashboard](/docs/ja/costs) を参照せずに支出を追跡できます。`--continue` または `--resume` で以前の会話を続ける場合、実行は会話全体の合計を報告し、[以前の実行の支出を含めて](/docs/ja/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls)。両方の数値は [client-side estimates](/docs/ja/agent-sdk/cost-tracking) であり、実際の請求額と異なる場合があります。

<Note>
  パイプされた stdin は 10MB に制限されています。制限を超えた場合、Claude Code は明確なエラーメッセージを表示して終了し、ゼロ以外のステータスを返します。より大きな入力を処理するには、コンテンツをファイルに書き込み、パイプする代わりにプロンプトでファイルパスを参照してください。
</Note>

Claude Code が stdin を読み込めない場合（例えば、それを開始したプロセスが終了した場合）、Claude Code は stderr に警告を出力して、コマンドラインからのプロンプトで続行します。v2.1.211 より前では、Windows で読み込み不可能な stdin はセッションをクラッシュさせるか、出力なしで静かに終了していました。

<h3 id="add-claude-to-a-build-script">
  ビルドスクリプトに Claude を追加する
</h3>

非対話呼び出しをスクリプトでラップして、Claude をプロジェクト固有のリンターまたはレビュアーとして使用できます。

この `package.json` スクリプトは `main` に対する diff をパイプして Claude に渡し、タイプミスを報告するよう指示します。diff をパイプすることで、Claude は Bash 権限がなくても読み込むことができ、エスケープされたダブルクォートはスクリプトを Windows に対応させます。

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

`npm run lint:claude` で実行します。

<h3 id="get-structured-output">
  構造化された出力を取得する
</h3>

`--output-format` を使用して、応答の返され方を制御します。

* `text`（デフォルト）：プレーンテキスト出力
* `json`：結果、セッション ID、メタデータを含む構造化 JSON
* `stream-json`：リアルタイムストリーミング用の改行区切り JSON

この例は、プロジェクト概要を JSON で返し、セッションメタデータを含め、テキスト結果は `result` フィールドに入ります。

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

特定のスキーマに準拠した出力を取得するには、`--output-format json` を `--json-schema` と [JSON Schema](https://json-schema.org/) 定義と共に使用します。応答には、リクエストに関するメタデータ（セッション ID、使用状況など）が含まれ、構造化出力は `structured_output` フィールドに入ります。

この例は、関数名を抽出して文字列の配列として返します。

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

値が有効な JSON Schema でない場合、`claude` は `Error: --json-schema is not a valid JSON Schema` で終了し、その後にバリデータの診断が続きます。Claude Code は `format` キーワード（例：`"format": "email"`）を使用するスキーマを受け入れますが、`format` を注釈として扱い、強制しません。v2.1.205 より前では、Claude Code は無効なスキーマを静かに無視して非構造化テキストを返し、`format` を含むスキーマを無効として扱っていました。

<Tip>
  [jq](https://jqlang.org/) などのツールを使用して応答を解析し、特定のフィールドを抽出します。

  ```bash theme={null}
  # テキスト結果を抽出
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # 構造化出力を抽出
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  応答をストリーミングする
</h3>

`--output-format stream-json` を `--verbose` と `--include-partial-messages` と共に使用して、生成されるトークンをリアルタイムで受け取ります。各行は、イベントを表す JSON オブジェクトです。

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

ストリームの最後の行は、最終応答テキスト、コスト、セッションメタデータを含む `result` メッセージです。

コンシューマーがストリームをゆっくり読む場合、Claude Code はキューに入った出力がドレインされるまで待機し、待機時間をまだキューに入っているもの量に応じてスケーリングし、最大 30 秒に制限されます。v2.1.214 より前では、終了待機は約 2 秒に制限されており、大きな応答の終わりが切り取られる可能性がありました。

次の例は [jq](https://jqlang.org/) を使用してテキストデルタをフィルタリングし、ストリーミングテキストのみを表示します。`-r` フラグは生の文字列（引用符なし）を出力し、`-j` は改行なしで結合するため、トークンは継続的にストリーミングされます。

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

コールバックとメッセージオブジェクトを使用したプログラマティックストリーミングについては、Agent SDK ドキュメントの [Stream responses in real-time](/docs/ja/agent-sdk/streaming-output) を参照してください。

<h4 id="follow-subagent-messages">
  サブエージェントメッセージをフォローする
</h4>

[subagents](/docs/ja/sub-agents) からのメッセージは、ストリームに `assistant` および `user` メッセージとして表示され、その `parent_tool_use_id` フィールドはサブエージェントを生成したツール呼び出しの ID です。メインの会話からのメッセージは、そのフィールドに `null` を含みます。

[foreground](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) で実行されているサブエージェントからの最初のメッセージは、それを駆動するプロンプトを含む `user` メッセージです。その最初のメッセージの後、Claude Code は以下を発行します。

* **デフォルト**：サブエージェントの `tool_use` および `tool_result` ブロック。
* **[`--forward-subagent-text`](/docs/ja/cli-reference#cli-flags) または [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/ja/env-vars) を使用**：サブエージェントのテキストおよび thinking ブロックも含まれるため、各サブエージェントのトランスクリプトを再構築できます。これには Claude Code v2.1.211 以降が必要です。

いずれかのオプションを有効にすると、Claude Code は [subagents at every nesting depth](/docs/ja/sub-agents#let-subagents-spawn-their-own-subagents) からのメッセージを転送します。Agent ツールまたは [forked skill](/docs/ja/skills#run-skills-in-a-subagent) として開始された場合、サブエージェントが生成されたかどうかに関わらず。
フォークされたスキルが生成するサブエージェントのメッセージ、およびサブエージェント内またはフォークされたスキル内で開始されたフォークされたスキルには、Claude Code v2.1.275 以降が必要です。`parent_tool_use_id` では、ネストされたサブエージェントのメッセージは、それを開始した Agent または Skill ツール呼び出しの ID を含むため、これらの ID をフォローして完全なネストツリーを再構築できます。v2.1.219 より前では、ネストされたサブエージェントからのメッセージはストリームに表示されていませんでした。

[subagent で実行される](/docs/ja/skills#run-skills-in-a-subagent) Skills は、ストリームに同じ方法で表示されます。フォークされたスキルの最初のメッセージは、実行を駆動するスキルコンテンツを含む `user` メッセージです。いずれかのオプションを有効にすると、ストリームはフォークされたスキルのテキストおよび thinking ブロックも含みます。v2.1.265 より前では、フォークされたスキルの `tool_use` および `tool_result` ブロックのみがストリームに表示されていました。

<h4 id="handle-api-retries">
  API 再試行を処理する
</h4>

API リクエストが再試行可能なエラーで失敗すると、Claude Code は再試行する前に `system/api_retry` イベントを発行します。v2.1.246 以降では、`401` または `403` が [`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper) 認証情報を拒否する場合、Claude Code は最初の 2 回の再試行を静かに行い、イベントなしで実行してから、3 回目の連続再試行からイベントを通常通り発行します。静かな再試行は依然として `attempt` にカウントされます。イベントを使用して、独自のインターフェースで再試行の進行状況を表示できます。

| フィールド            | 型                | 説明                                                                                                                                                                                                                                                |
| ---------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`       | メッセージタイプ                                                                                                                                                                                                                                          |
| `subtype`        | `"api_retry"`    | これを再試行イベントとして識別                                                                                                                                                                                                                                   |
| `attempt`        | integer          | 現在の試行番号（1 から開始）                                                                                                                                                                                                                                   |
| `max_retries`    | integer          | このエラーの原因に許可される再試行の合計（セッション全体の予算より少ない場合がある）                                                                                                                                                                                                        |
| `retry_delay_ms` | integer          | 次の試行までのミリ秒                                                                                                                                                                                                                                        |
| `error_status`   | integer または null | 失敗した試行の HTTP ステータスコード、または試行が API から HTTP レスポンスを受け取らなかった場合は `null`                                                                                                                                                                                 |
| `no_response`    | object、optional  | 失敗した試行が [no response headers in time](/docs/ja/errors#no-response-from-api) を受け取った場合にのみ存在します。`waited_ms` はその試行が待機した時間で、`retry_wait_ms` は再試行が待機する時間です。これらのイベントでは、`max_retries` はセッション全体の予算ではなく、この原因が通常受け取る 1 回の再試行を反映しています。Claude Code v2.1.261 以降が必要です |
| `error`          | string           | エラーカテゴリ：`authentication_failed`、`oauth_org_not_allowed`、`account_on_hold`、`billing_error`、`rate_limit`、`overloaded`、`invalid_request`、`model_not_found`、`server_error`、`max_output_tokens`、`cloud_credential_error`、または `unknown`                 |
| `uuid`           | string           | 一意のイベント識別子                                                                                                                                                                                                                                        |
| `session_id`     | string           | イベントが属するセッション                                                                                                                                                                                                                                     |

<h4 id="read-session-metadata">
  セッションメタデータを読む
</h4>

`system/init` イベントは、モデル、ツール、MCP サーバー、読み込まれたプラグインを含むセッションメタデータを報告します。スタートアップイベントが先行しない限り、ストリームの最初のイベントです。

* `plugin_install` イベント（[`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ja/env-vars) が設定されている場合）。
* [`hook_started`、`hook_progress`、および `hook_response` イベント](/docs/ja/agent-sdk/typescript#sdkhookstartedmessage)（設定された [`SessionStart`](/docs/ja/hooks#sessionstart) または [`Setup`](/docs/ja/hooks#setup) hook が実行されている間）。これらは hook が生成するときにストリーミングされます。Claude Code v2.1.169 から v2.1.203 はそれらを hook 完了後に 1 つのバッチで配信し、依然として `system/init` より前でした。v2.1.204 はライブ配信を復元しました。

イベントは、このバージョンの Claude Code が実装するプロトコル動作を命名する文字列の optional `capabilities` 配列も含みます（例：`interrupt_receipt_v1` または `interrupt_cancel_queued_v1`）。バージョン文字列を比較する代わりに、これを使用して機能を検出し、認識しない値は無視してください。フィールドは Claude Code v2.1.205 以降が必要で、以前のバージョンでは存在しません。機能リストについては [`SDKSystemMessage`](/docs/ja/agent-sdk/typescript#sdksystemmessage) を参照してください。

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  プラグインまたは MCP サーバーが読み込まれない場合に CI を失敗させる
</h4>

`system/init` イベントのプラグインフィールドを使用して、読み込まれなかったプラグインをキャッチします。

| フィールド           | 型     | 説明                                                                                                                                                                     |
| --------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | array | 正常に読み込まれたプラグイン（各々 `name` と `path` を含む）                                                                                                                                 |
| `plugin_errors` | array | プラグイン読み込み時エラー（各々 `plugin`、`type`、`message` を含む）。満たされていない依存関係バージョンと `--plugin-dir` 読み込み失敗（パスの欠落やアーカイブが無効など）を含みます。影響を受けたプラグインは降格され、`plugins` から削除されます。エラーがない場合、キーは省略されます |

MCP サーバーフィールドも同じ方法で使用します。
`-p` で [`--mcp-config`](/docs/ja/cli-reference#cli-flags) を渡す場合、Claude Code は最初のターンを実行する前に、まだ保留中のサーバーを待機します（[`MCP_TIMEOUT`](/docs/ja/env-vars) スタートアップタイムアウト（デフォルト 30 秒）まで）。[cached tool list](/docs/ja/agent-sdk/mcp#connection-timing) を持つリモートサーバーは待機をスキップし、`system/init` に `pending` を表示し、最初のツール呼び出しで接続します。待機には Claude Code v2.1.221 以降が必要です。

Claude Code は起動時に各 `--mcp-config` エントリを検証し、検証に失敗したエントリをスキップします（例えば、`type` のない `url` エントリ）。実行は続行され、クリーンに終了するため、これらのフィールドをチェックして、読み込まれなかったサーバーをキャッチします。

| フィールド               | 型     | 説明                                                                                                                                                                                                                                                                                             |
| ------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | array | セッション内の MCP サーバー（各々 `name` と `status` を含む）                                                                                                                                                                                                                                                     |
| `mcp_server_errors` | array | 設定検証によってスキップされた `--mcp-config` エントリ（各々 `name`、`type`、`message` を含む）。`type` はスキップカテゴリ（`unknown_type`、`url_missing_type`、`invalid_config`、`reserved_name` など）です。認識しない値は汎用スキップとして扱ってください。影響を受けたサーバーは `mcp_servers` から削除されます。エラーがない場合、キーは省略されるため、CI ゲートは空でない配列で失敗できます。Claude Code v2.1.219 以降が必要です |

コマンドを手でターミナルで実行する場合、Claude Code は stderr にスタートアップ警告も出力します（例：`Warning: 1 MCP server skipped due to invalid config:`）。その後に各スキップエントリの理由が続きます。stderr をリダイレクトする場合、または CI ランナーなどのプログラムがそれをキャプチャする場合、Claude Code は警告を出力せず、スキップされたエントリを `mcp_server_errors` フィールドでのみ報告します。警告には Claude Code v2.1.219 以降が必要です。

<h4 id="track-plugin-installs">
  プラグインインストールを追跡する
</h4>

[`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ja/env-vars) が設定されている場合、Claude Code は最初のターンの前にマーケットプレイスプラグインがインストールされている間、`system/plugin_install` イベントを発行します。これらを使用して、独自の UI にインストール進行状況を表示します。

| フィールド        | 型                                                      | 説明                                                                                      |
| ------------ | ------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                             | メッセージタイプ                                                                                |
| `subtype`    | `"plugin_install"`                                     | これをプラグインインストールイベントとして識別                                                                 |
| `status`     | `"started"`、`"installed"`、`"failed"`、または `"completed"` | `started` と `completed` は全体的なインストールをブラケットします。`installed` と `failed` は個別のマーケットプレイスを報告します |
| `name`       | string、optional                                        | マーケットプレイス名（`installed` と `failed` に存在）                                                  |
| `error`      | string、optional                                        | 失敗メッセージ（`failed` に存在）                                                                   |
| `uuid`       | string                                                 | 一意のイベント識別子                                                                              |
| `session_id` | string                                                 | イベントが属するセッション                                                                           |

<h3 id="auto-approve-tools">
  ツールを自動承認する
</h3>

`--allowedTools` を使用して、Claude が特定のツールをプロンプトなしで使用できるようにします。この例はテストスイートを実行して失敗を修正し、Claude が Bash コマンドを実行してファイルを読み書きできるようにします（権限を求めずに）。

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

個別のツールをリストする代わりにセッション全体のベースラインを設定するには、[permission mode](/docs/ja/permission-modes) を渡します。`-p` の場合、[built-in starting permission mode](/docs/ja/permission-modes#which-mode-a-session-starts-in) はすべてのプランで Manual なので、必要な権限モードを渡します。

* **`auto`**：`--permission-mode auto` を渡して、ほとんどのアクションをあなたの代わりに分類器にレビューさせます
* **`dontAsk`**：Claude Code はそれ以外の場合はプロンプトするすべての呼び出しを拒否します。これはロックダウンされた CI 実行に役立ちます。Manual モードで承認が不要なアクション（作業ディレクトリでのファイル読み取りや [read-only command set](/docs/ja/permissions#read-only-commands)）は依然として実行され、`--allowedTools` エントリまたは `permissions.allow` ルールがカバーするアクションも実行されます。`AskUserQuestion`、connector tools [your organization set to `ask`](/docs/ja/mcp#organization-controls-on-connector-tools)、および [`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool) とマークされた MCP ツールは、許可ルールが一致する場合でも拒否されます
* **`acceptEdits`**：Claude はプロンプトなしでファイルを書き込み、Claude Code は `mkdir`、`touch`、`mv`、`cp` などの一般的なファイルシステムコマンドを自動承認します。[actions no mode auto-approves](/docs/ja/permission-modes#actions-no-mode-auto-approves) は依然として適用されます。read-only command set を除き、他のシェルコマンドとネットワークリクエストは依然として `--allowedTools` エントリまたは `permissions.allow` ルールが必要です。[what `acceptEdits` auto-approves](/docs/ja/permission-modes#auto-approve-file-edits-with-acceptedits-mode) を参照して、完全なリストを確認してください

この例は `acceptEdits` をベースラインとしてリント修正を適用します。

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  無人実行で権限プロンプトをオフにする
</h3>

誰も権限プロンプトに答えられない場合（例えば、スケジュール済みジョブ）、`--permission-prompts none` を渡します。フラグは、実行に権限ホストがある場合に最も重要です。Agent SDK アプリ（[`canUseTool` callback](/docs/ja/agent-sdk/user-input) を含む）、または [`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags) で渡す MCP ツール。フラグなしでは、実行は各権限リクエストに対してそのホストを待機します。

フラグを使用すると、実行はホストを参照せず、それを待機しません。プロンプトするすべてのものは、`PermissionRequest` hook が許可しない限り拒否され、Claude には誰も要求を承認できず、再試行しないことが伝えられ、実行は続行されます。ホストのない `-p` 実行では、これらのリクエストはいずれにせよ拒否され、フラグは Claude に再試行しないことも伝えます。権限ルール、[`PermissionRequest` hooks](/docs/ja/hooks#permissionrequest)、および設定した権限モードは依然としてすべての呼び出しを最初に決定します。Claude Code は、他に何も解決しないリクエストのみを拒否します。

この例は [auto mode](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) で無人タスクを実行します。分類器は通常通り各アクションをレビューし、Claude Code はプロンプトにフォールバックしたであろうすべてのものを拒否します。

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

`--permission-prompts none` を使用すると、Claude Code は [`AskUserQuestion`](/docs/ja/tools-reference#askuserquestion-tool-behavior) など、人からの答えが必要なツールを削除するため、Claude はそれらを呼び出すことができません。[`Elicitation` hook](/docs/ja/hooks#elicitation) が答えない [MCP elicitation request](/docs/ja/mcp#respond-to-mcp-elicitation-requests) はキャンセルされます。

`--output-format stream-json` を使用すると、拒否は `permission_denied` システムメッセージとして表示され、最終結果メッセージは `permission_denials` にそれらをリストします。

<Note>
  `--permission-prompts` フラグには Claude Code v2.1.259 以降が必要です。以前のバージョンは不明なオプションエラーで拒否します。
</Note>

<h3 id="create-a-commit">
  コミットを作成する
</h3>

この例はステージされた変更をレビューして、適切なメッセージでコミットを作成します。

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

`--allowedTools` フラグは [permission rule syntax](/docs/ja/settings-reference#permission-rule-syntax) を使用します。末尾の ` *` はプレフィックスマッチングを有効にするため、`Bash(git diff *)` は `git diff` で始まるすべてのコマンドを許可します。スペースは `*` の前に重要です。なければ、`Bash(git diff*)` は `git diff-index` も一致させます。

<Note>
  コマンドサポートは `-p` モードで異なります。

  * ユーザーが呼び出した [skills](/docs/ja/skills) とカスタムコマンドは機能します。プロンプト文字列に `/skill-name` を含めると、Claude Code は実行前にそれを展開します。
  * ターミナルインターフェースでのみ実行される `/login` などの組み込みコマンドは利用できません。
  *

  `/model`、`/effort`、`/fast`、`/color`、`/rename` は値を引数として受け入れます（例：`/model sonnet`）。`/mcp` は引数なしでサーバーステータスのテキスト概要を出力します。これらのフォームには Claude Code v2.1.205 以降が必要で、各コマンドの [availability notes](/docs/ja/commands#all-commands) に従います。

  * 設定を変更するには、`/config` に `key=value` を渡します（例：`/config thinking=false`）。
  *

  `/output-style <style>` は [output styles](/docs/ja/output-styles) を切り替え、`/output-style` のみはそれらをリストします。Claude Code v2.1.269 以降が必要です。
</Note>

<h3 id="customize-the-system-prompt">
  システムプロンプトをカスタマイズする
</h3>

`--append-system-prompt` を使用して、Claude Code のデフォルト動作を保持しながら指示を追加します。この例は PR diff を Claude にパイプして、セキュリティ脆弱性をレビューするよう指示します。シェルスクリプトとして保存します（例：`review.sh`）。

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

スクリプトでは、`"$1"` はコマンドラインで渡す最初の引数を表します。`bash review.sh 123` を実行すると、シェルは `"$1"` を `123` に置き換えるため、スクリプトは PR 123 の diff をフェッチします。Claude Code はレビューを JSON として出力し、テキストは `result` フィールドにあります。

詳細については、[system prompt flags](/docs/ja/cli-reference#system-prompt-flags) を参照してください。`--system-prompt` を使用してデフォルトプロンプトを完全に置き換えるオプションも含まれています。

<h3 id="continue-conversations">
  会話を続ける
</h3>

`--continue` を使用して最新の会話を続けるか、`--resume` をセッション ID と共に使用して特定の会話を続けます。
Claude Code v2.1.257 以降では、`--continue` を渡すと、Claude Code は完了した [background session](/docs/ja/sessions#resume-a-session) を開きますが、まだ実行中のセッションは開きません。この例はレビューを実行してから、フォローアッププロンプトを送信します。

```bash theme={null}
# 最初のリクエスト
claude -p "Review this codebase for performance issues"

# 最新の会話を続ける
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

複数の会話を実行している場合は、セッション ID をキャプチャして特定のセッションを再開します。

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

2 つのコマンドを異なるディレクトリから実行できます。Claude Code は [finds the session by its ID](/docs/ja/sessions#resume-a-session) をこのマシン上の任意のプロジェクトで実行します。v2.1.223 より前では、Claude Code は現在のプロジェクトディレクトリとその git worktrees でのみ ID を探していたため、両方のコマンドを同じディレクトリから実行する必要がありました。

セッション ID の代わりに、`--resume` にセッションの `.jsonl` [transcript file](/docs/ja/sessions#where-transcripts-are-stored) への絶対パスを渡すことができます。Claude Code はそのファイルに保存されている会話を続けます。

<h2 id="next-steps">
  次のステップ
</h2>

* [Agent SDK クイックスタート](/docs/ja/agent-sdk/quickstart)：Python または TypeScript で最初のエージェントを構築します
* [CLI リファレンス](/docs/ja/cli-reference)：すべての CLI フラグとオプション
* [GitHub Actions](/docs/ja/github-actions)：GitHub ワークフローで Agent SDK を使用します
* [GitLab CI/CD](/docs/ja/gitlab-ci-cd)：GitLab パイプラインで Agent SDK を使用します
