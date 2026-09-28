> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# ツールリファレンス

> Claude Code が使用できるツールの完全なリファレンス。権限要件とツール別の動作を含みます。

Claude Code は、コードベースを理解および変更するのに役立つ組み込みツールのセットにアクセスできます。ツール名は、[権限ルール](/docs/ja/permissions#tool-specific-permission-rules)、[サブエージェントツールリスト](/docs/ja/sub-agents)、および[フックマッチャー](/docs/ja/hooks)で使用する正確な文字列です。

Claude が使用できるツールと、いつ最初に確認するかを制御するには、設定、[フック](/docs/ja/hooks)、または[サブエージェントのツールリスト](/docs/ja/sub-agents#supported-frontmatter-fields)で[権限ルール](/docs/ja/permissions#tool-specific-permission-rules)を設定します。ツール名を受け入れる各場所については、[権限ルールとフックでツールを設定する](#configure-tools-with-permission-rules-and-hooks)を参照してください。

カスタムツールを追加するには、[MCP サーバー](/docs/ja/mcp)を接続します。再利用可能なプロンプトベースのワークフローで Claude を拡張するには、[スキル](/docs/ja/skills#control-who-invokes-a-skill)を作成します。これは新しいツールエントリを追加するのではなく、既存の `Skill` ツールを通じて実行されます。

<Info>
  Pro、Max、Team プランでは、Claude Code は[オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)でセッションを開始します。ここでは、分類器がこれらのプロンプトのほとんどを決定します。`Permission required` 列は、ツールが[マニュアルモード](/docs/ja/permission-modes)でワーキングディレクトリ内のパスに対してプロンプトを表示するかどうかを示します。`Read`、`Grep`、`Glob` を含むファイルアクセスツールは「いいえ」とマークされていますが、[ワーキングディレクトリと追加ディレクトリ](/docs/ja/permissions#working-directories)外のパスに対してはプロンプトを表示します。`Bash` は「はい」とマークされていますが、プロンプトなしで組み込みの[読み取り専用コマンド](/docs/ja/permissions#read-only-commands)セットを実行します。
</Info>

| ツール                    | 説明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | 権限が必要 |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---- |
| `Agent`                | タスクを処理するために独自のコンテキストウィンドウを持つ[サブエージェント](/docs/ja/sub-agents)を生成します。[エージェントチーム](/docs/ja/agent-teams)が有効な場合、`name` を含む呼び出しは[チームメイト](/docs/ja/agent-teams#how-claude-starts-agent-teams)を起動できます。[Agent ツールの動作](#agent-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | いいえ   |
| `Artifact`             | HTML または Markdown ファイルを[アーティファクト](/docs/ja/artifacts)として公開します。これは claude.ai 上のプライベートでインタラクティブなページです。公開リンクで共有することも、Team および Enterprise プランの組織内で共有することもできます。公開共有には Owner による[有効化](/docs/ja/artifacts#control-public-sharing)が必要です。Pro、Max、Team、または Enterprise プランが必要で、`/login` 認証が必要です。[利用可能性](/docs/ja/artifacts#availability)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                             | はい    |
| `AskUserQuestion`      | 要件を収集したり曖昧さを明確にしたりするために、複数選択肢の質問をします。デフォルトでは、質問に答えるまで質問は開いたままになります。[AskUserQuestion ツールの動作](#askuserquestion-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | いいえ   |
| `Bash`                 | 環境でシェルコマンドを実行します。[Bash ツールの動作](#bash-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | はい    |
| `CronCreate`           | 現在のセッション内で定期的または 1 回限りのプロンプトをスケジュールします。タスクはセッションスコープであり、`--resume` または `--continue` で復元されます（有効期限が切れていない場合）。[スケジュール済みタスク](/docs/ja/scheduled-tasks)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | いいえ   |
| `CronDelete`           | ID でスケジュール済みタスクをキャンセルします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | いいえ   |
| `CronList`             | セッション内のすべてのスケジュール済みタスクをリストします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | いいえ   |
| `Edit`                 | 特定のファイルに対して対象を絞った編集を行います。[Edit ツールの動作](#edit-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | はい    |
| `EndConversation`      | セッションを終了します。持続的な不正使用入力の稀なケースまたは Claude にツールのデモンストレーションを求める場合に使用します。Claude Code v2.1.213 以降が必要です。[EndConversation ツールの動作](#endconversation-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | いいえ   |
| `EnterPlanMode`        | プランモードに切り替えて、コーディング前にアプローチを設計します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | いいえ   |
| `EnterWorktree`        | 分離された[git worktree](/docs/ja/worktrees)を作成し、それに切り替えます。新しいものを作成する代わりに、既存の worktree に切り替えるには `path` を渡します。最初のエントリでは、ターゲットは現在のリポジトリの worktree、またはマルチリポジトリワークスペースでは、その中にネストされたリポジトリの worktree である可能性があります。v2.1.203 より前では、ネストされたリポジトリの worktree は拒否されていました。`.claude/worktrees/` の外の `path` は、セッションのワーキングディレクトリと書き込みアクセスをその場所に移動するため、承認を求めるプロンプトが表示されます。新しい worktree の作成と `.claude/worktrees/` の下のパスはプロンプトを表示しません。v2.1.206 より前では、Claude は `.claude/worktrees/` の外のパスにプロンプトなしで入りました。worktree セッション内から、または [`isolation: worktree`](/docs/ja/sub-agents#supported-frontmatter-fields) などのピン留めされたワーキングディレクトリを持つサブエージェントから、`path` フォームのみが利用可能で、ターゲットはセッションのリポジトリの `.claude/worktrees/` の下にある必要があります                                                      | はい    |
| `ExitPlanMode`         | 承認のためのプランを提示し、プランモードを終了します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | はい    |
| `ExitWorktree`         | worktree セッションを終了し、元のディレクトリに戻ります。[`isolation: worktree`](/docs/ja/sub-agents#supported-frontmatter-fields) などのように既に独自のワーキングディレクトリで実行されているサブエージェントは利用できません                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | いいえ   |
| `Glob`                 | パターンマッチングに基づいてファイルを検索します。[Glob ツールの動作](#glob-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | いいえ   |
| `Grep`                 | ファイルコンテンツ内のパターンを検索します。[Grep ツールの動作](#grep-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | いいえ   |
| `ListAgents`           | Claude が `SendMessage` でメッセージを送信できるエージェントをリストします。セッション内のサブエージェント、[エージェントチーム](/docs/ja/agent-teams)チームメイト、その他のローカル Claude Code セッション、およびこのセッションが[リモートコントロール](/docs/ja/remote-control)に接続されている間、[ウェブ上の Claude Code](/docs/ja/claude-code-on-the-web)セッションと他のマシン上のリモートコントロールセッション。`/list-agents` コマンドをサポートします。[クロスセッションメッセージング](/docs/ja/cross-session-messaging)を参照してください。Claude Code v2.1.224 以降が必要で、[クロスセッションメッセージングが有効](/docs/ja/cross-session-messaging#availability)なセッションにのみ表示されます。チームメイト行とこのセッション独自の名前を示す最初の行には v2.1.239 以降が必要です                                                                                                                                                                                                                        | いいえ   |
| `ListMcpResourcesTool` | 接続された[MCP サーバー](/docs/ja/mcp)によって公開されたリソースをリストします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | いいえ   |
| `LSP`                  | 言語サーバー経由のコードインテリジェンス。定義にジャンプ、参照を検索、型エラーと警告をレポートします。[LSP ツールの動作](#lsp-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | いいえ   |
| `Monitor`              | コマンドをバックグラウンドで実行し、各出力行を Claude にフィードバックして、ログエントリ、ファイル変更、またはポーリングされたステータスに対応できるようにします。WebSocket を開いて、各受信メッセージをイベントとして扱うこともできます。[Monitor ツール](#monitor-tool)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | はい    |
| `NotebookEdit`         | Jupyter ノートブックセルを変更します。[NotebookEdit ツールの動作](#notebookedit-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | はい    |
| `PowerShell`           | PowerShell コマンドをネイティブに実行します。[PowerShell ツール](#powershell-tool)の利用可能性を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | はい    |
| `PushNotification`     | デスクトップ通知を送信し、[リモートコントロール](/docs/ja/remote-control)が接続されている場合は電話プッシュを送信します。長時間実行されるタスクまたは[スケジュール済みタスク](/docs/ja/scheduled-tasks)が、あなたが離れているときに到達できるようにします。プッシュ配信は Anthropic ホスト型インフラストラクチャを通じて実行されます。これは Amazon Bedrock、AWS 上の Claude Platform、Google Cloud の Agent Platform、または Microsoft Foundry からはアクセスできません                                                                                                                                                                                                                                                                                                                                                                                                                                | いいえ   |
| `Read`                 | ファイルの内容を読み取ります。[Read ツールの動作](#read-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | いいえ   |
| `ReadMcpResourceTool`  | URI で特定の MCP リソースを読み取ります                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | いいえ   |
| `RemoteTrigger`        | claude.ai で[ルーチン](/docs/ja/routines)を作成、更新、実行、リストします。`/schedule` コマンドをサポートします。[`RemoteTrigger` 入力リファレンス](/docs/ja/agent-sdk/typescript#remotetrigger)は、すべてのアクションと、ツールを削除する組織ポリシーを文書化しています。ルーチンは claude.ai に存在し、Pro、Max、Team、または Enterprise プランが必要です。そのため、このツールは Amazon Bedrock、AWS 上の Claude Platform、Google Cloud の Agent Platform、または Microsoft Foundry からはアクセスできません                                                                                                                                                                                                                                                                                                                                                                        | いいえ   |
| `ReportFindings`       | コードレビューの検出結果を構造化リストとしてレポートします。検出結果ごとにファイル、概要、失敗シナリオがあり、Claude Code はテキストとして出力する代わりにレンダリングできます。Claude はアクティブなコードレビュー指示がこれを呼び出すように指示する場合に呼び出します。Claude Code v2.1.196 以降が必要です。v2.1.199 以降、検出結果は `correctness` または `test-coverage` などのオプションの `category` スラッグを含むことができ、レンダリングされたリストのファイルの場所の横に表示されます                                                                                                                                                                                                                                                                                                                                                                                                                                      | いいえ   |
| `ScheduleWakeup`       | [自分のペースで進む `/loop`](/docs/ja/scheduled-tasks#let-claude-choose-the-interval) の次の反復をスケジュールし直します。Claude は各反復の終了時にこれを呼び出して、次の反復をいつ実行するかを 1 分後から 1 時間後の間で選択します。ユーザーが直接呼び出すことはありません。代わりにループを終了するには、Claude はこれを `stop: true` で呼び出します。これは保留中のウェイクアップをキャンセルします。`stop` フィールドには Claude Code v2.1.202 以降が必要です。保留中のウェイクアップは[Stop フックの入力](/docs/ja/hooks#stop-input)の `session_crons` に表示されます                                                                                                                                                                                                                                                                                                                                                              | いいえ   |
| `SendFeedback`         | Claude Code に関するフィードバックレポートを作成します。製品の問題または Claude Code セッション内での Claude 自身の動作をカバーします。ユーザーがレビューできるよう、お使いのマシン上のキューに入れます。Claude Code は、ドラフトを送信することを選択するまで何も送信しません。[SendFeedback ツールの動作](#sendfeedback-tool-behavior)を参照してください。Claude Code v2.1.238 以降が必要です                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | いいえ   |
| `SendMessage`          | 別のエージェントにメッセージを送信します。[エージェントチーム](/docs/ja/agent-teams)チームメイト、[エージェント ID または名前で再開するサブエージェント](/docs/ja/sub-agents#resume-subagents)、またはこのマシン上またはその外にある他の Claude Code セッションのいずれか。他のセッションへのメッセージングには Claude Code v2.1.224 以降が必要です。[クロスセッションメッセージング](/docs/ja/cross-session-messaging)は、Claude が到達できるセッション、[メッセージが到着したときの外観](/docs/ja/cross-session-messaging#what-a-message-looks-like)、および[別のセッションがアイドル状態になったときに Claude が通知を受け取る方法](/docs/ja/cross-session-messaging#get-a-notice-when-another-session-goes-idle)をカバーしています。Claude はオプションの `summary` 入力を含めることができます。通常は 5～10 語で、Claude Code は 1 行のプレビューとして表示します。Claude が[プレーンテキストメッセージ](/docs/ja/cross-session-messaging#limitations)で省略した場合、Claude Code はメッセージの最初の行を概要として使用します。Claude Code は 200 文字を超える概要を省略記号で切り詰めます | いいえ   |
| `SendUserFile`         | セッションからファイルをオプションのキャプション付きで送信します。生成されたレポート、図、スクリーンショット、または構築されたアーティファクトがトランスクリプトでのみ言及されるのではなく、デバイスに到達するようにします。v2.1.196 以降、オプションの `display` 入力はプレゼンテーションを制御します。`render` はファイルをクライアントにインラインで開き、`attach` はダウンロードカードのみを表示し、設定されていない場合、クライアントはファイルタイプで決定します。[リモートコントロール](/docs/ja/remote-control)クライアントが接続されている場合、またはセッションが[ウェブ上の Claude Code](/docs/ja/claude-code-on-the-web)などのマネージドクラウド環境で実行されている場合に利用可能です。配信は Anthropic ホスト型インフラストラクチャを通じて実行されるため、このツールは Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry では利用できません                                                                                                                                                                                                         | いいえ   |
| `ShareOnboardingGuide` | ガイドが作成された後、`ONBOARDING.md` をアップロードし、チームメイトが Claude Code で開くことができる共有リンクを返します。`/team-onboarding` から呼び出されます。claude.ai サブスクライバーが Pro、Max、Team、Enterprise プランで利用可能です                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | はい    |
| `Skill`                | メイン会話内で[スキル](/docs/ja/skills#control-who-invokes-a-skill)を実行します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | はい    |
| `SubagentHandback`     | サブエージェントの最終レポートを、そのサブエージェントの結果を受け取る会話に配信します。[オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)でのみ提供され、Agent ツールがローカルで実行するサブエージェント（[フォーク](/docs/ja/sub-agents#fork-the-current-conversation)以外）に提供され、ターミナル CLI、IDE 拡張機能、クラウドセッション、および Agent SDK で利用可能です。分類器はレポートが配信される前にレビューします。Claude Code v2.1.271 以降が必要です                                                                                                                                                                                                                                                                                                                                                                                                                 | いいえ   |
| `TaskCreate`           | タスクリストに新しいタスクを作成します。[タスクツール利用可能性](#task-tool-availability)の下にリストされているモデルでデフォルトで提供され、他のモデルではオプトインした場合に提供されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | いいえ   |
| `TaskGet`              | 特定のタスクの完全な詳細を取得します。[タスクツール利用可能性](#task-tool-availability)の下にリストされているモデルでデフォルトで提供され、他のモデルではオプトインした場合に提供されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | いいえ   |
| `TaskList`             | すべてのタスクを現在のステータスでリストします。[タスクツール利用可能性](#task-tool-availability)の下にリストされているモデルでデフォルトで提供され、他のモデルではオプトインした場合に提供されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | いいえ   |
| `TaskOutput`           | バックグラウンドタスクから出力を取得します。タスクの出力ファイルパスで `Read` を優先して廃止されました。ID に一致するタスクがない場合、エラーは実行中のバックグラウンドエージェントを ID と説明でリストします。v2.1.203 より前では、エラーは欠落している ID のみを名前付けていました                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | いいえ   |
| `TaskStop`             | ID でバックグラウンドタスクを実行中に停止します。また、[エージェントチームチームメイト](/docs/ja/agent-teams)またはエージェント ID または名前でバックグラウンドエージェントを受け入れます。v2.1.198 より前では、バックグラウンドタスク ID のみを受け入れていました。ID に一致するタスクがない場合、エラーは実行中のバックグラウンドエージェントを ID と説明でリストします。別のエージェントが生成したエージェントを含みます。v2.1.203 より前では、エラーは実行中のチームメイトと名前付きエージェントをリストしていましたが、別のエージェントが生成したバックグラウンドエージェントはリストしていなかったため、メイン会話から識別または停止できませんでした                                                                                                                                                                                                                                                                                                                                                                              | いいえ   |
| `TaskUpdate`           | タスクステータス、依存関係、詳細を更新するか、タスクを削除します。[タスクツール利用可能性](#task-tool-availability)の下にリストされているモデルでデフォルトで提供され、他のモデルではオプトインした場合に提供されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | いいえ   |
| `TodoWrite`            | セッションタスクチェックリストを管理します。`TaskCreate`、`TaskGet`、`TaskList`、`TaskUpdate` を優先して、デフォルトで無効になっています。[タスク追跡ツールを持つセッション](#task-tool-availability)で再度有効にするには、`CLAUDE_CODE_ENABLE_TASKS=0` を設定します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | いいえ   |
| `ToolSearch`           | [ツール検索](/docs/ja/mcp#scale-with-mcp-tool-search)が有効な場合、遅延ツールを検索してロードします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | いいえ   |
| `WaitForMcpServers`    | バックグラウンドでまだ接続中の 1 つ以上の[MCP サーバー](/docs/ja/mcp)を待機して、セッションを再開しなくてもリクエストがそのツールを使用できるようにします。必要なサーバーがまだ接続されていない場合、Claude はこれを呼び出します。[ツール検索](/docs/ja/mcp#scale-with-mcp-tool-search)が無効な場合にのみ表示されます。有効な場合は `ToolSearch` が待機を処理します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | いいえ   |
| `WebFetch`             | 指定された URL からコンテンツを取得します。[WebFetch ツールの動作](#webfetch-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | はい    |
| `WebSearch`            | ウェブ検索を実行します。[WebSearch ツールの動作](#websearch-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | はい    |
| `Workflow`             | [動的ワークフロー](/docs/ja/workflows)を実行します。バックグラウンドで多くのサブエージェントをオーケストレーションし、1 つの統合結果を返すスクリプト                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | はい    |
| `Write`                | ファイルを作成または上書きします。[Write ツールの動作](#write-tool-behavior)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | はい    |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  権限ルールとフックでツールを設定する
</h2>

ほとんどの場合、Claude がこれらのツールをいつ使用するかを決定するため、Claude と対話するときに自分でツール名を指定する必要はありません。権限やその他の設定を定義するときにツール名を直接参照します。

* 設定の [`permissions.allow`](/docs/ja/settings-reference#permissions-allow) と [`permissions.deny`](/docs/ja/settings-reference#permissions-deny)、および `/permissions` インターフェース内
* [`CLI フラグ`](/docs/ja/cli-reference)の `--allowedTools` と `--disallowedTools`
* Agent SDK の [`allowedTools` と `disallowedTools`](/docs/ja/agent-sdk/permissions#allow-and-deny-rules) オプション内
* [スキルの `allowed-tools`](/docs/ja/skills#frontmatter-reference) frontmatter 内
* フックの [`if` 条件](/docs/ja/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field)内

これらはすべて同じルール形式 `ToolName(specifier)` を受け入れます。指定子はツールによって異なり、複数のツールが形式を共有しています。

| ルール形式                          | 適用対象                    | 詳細                                                             |
| :----------------------------- | :---------------------- | :------------------------------------------------------------- |
| `Bash(npm run *)`              | Bash、Monitor            | [コマンドパターンマッチング](/docs/ja/permissions#bash)                          |
| `PowerShell(Get-ChildItem *)`  | PowerShell              | [コマンドパターンマッチング](/docs/ja/permissions#powershell)                    |
| `Read(~/secrets/**)`           | Read、Grep、Glob、LSP      | [パスパターンマッチング](/docs/ja/permissions#read-and-edit)                   |
| `Edit(/src/**)`                | Edit、Write、NotebookEdit | [パスパターンマッチング](/docs/ja/permissions#read-and-edit)                   |
| `Skill(deploy *)`              | Skill                   | [スキル名マッチング](/docs/ja/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                   | [サブエージェントタイプマッチング](/docs/ja/permissions#agent-subagents)            |
| `WebFetch(domain:example.com)` | WebFetch                | [ドメインマッチング](/docs/ja/permissions#webfetch)                          |
| `WebSearch`                    | WebSearch               | 指定子なし。ツール全体を許可または拒否                                            |

ここに記載されていないツール（`ExitPlanMode` や `ShareOnboardingGuide` など）は、指定子なしの裸のツール名のみを受け入れます。

`Edit(...)` 許可ルールは同じパスへの読み取りアクセスも付与するため、一致する `Read(...)` ルールは必要ありません。`Read(...)` 拒否ルールは同じパス上の Edit および Write ツールもブロックします。これには新しいファイルの作成も含まれます。両方のツールが Claude が読み戻す必要があるコンテンツを変更するためです。`Read` 拒否チェックには、編集時に Claude Code v2.1.208 以降が必要です。書き込み時には v2.1.228 以降が必要です。

フックの `matcher` フィールドは、括弧で囲まれたルール形式ではなく、裸のツール名を使用します。マッチングルールについては [matcher パターン](/docs/ja/hooks#matcher-patterns)を参照してください。各ツールがフック内の `tool_input` に渡すフィールド名については、[PreToolUse 入力リファレンス](/docs/ja/hooks#pretooluse-input)を参照してください。

<h2 id="agent-tool-behavior">
  Agent ツールの動作
</h2>

Agent ツールは、別のコンテキストウィンドウでサブエージェントを起動します。サブエージェントはそのタスクを自律的に処理し、親の会話に結果を返します。親はサブエージェントの中間的なツール呼び出しや出力を見ることはなく、最終的な結果のみを見ます。[agent teams](/docs/ja/agent-teams) が有効な場合、`name` を持つ呼び出しは [teammate](/docs/ja/agent-teams#how-claude-starts-agent-teams) を起動することができ、結果を返す代わりにチームメッセージを通じて報告します。

サブエージェントが実行するターン数を制限するには、[subagent definition](/docs/ja/sub-agents#supported-frontmatter-fields) で `maxTurns` を設定します。サブエージェントが制限に達すると、Claude Code は返された結果を部分的な出力としてマークし、Claude は [subagent を再開](/docs/ja/sub-agents#resume-subagents) して続行することができます。

同じ Agent ツールは、[fork mode](/docs/ja/sub-agents#turn-fork-mode-on-or-off) がオンの場所で [forked subagents](/docs/ja/sub-agents#fork-the-current-conversation) も起動します。フォークは新規に開始する代わりに親の会話全体を継承し、[foreground に留まるケース](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) を除いてバックグラウンドで実行され、ターミナルで権限プロンプトを表示します。このセクションの残りは、非フォーク subagent について説明します。

非フォーク subagent が使用できるツールは、[subagent definition](/docs/ja/sub-agents) の `tools` フィールドと `disallowedTools` フィールドに依存します。

* **どちらのフィールドも設定されていない場合**: subagent は [subagent で利用可能なすべてのツール](/docs/ja/sub-agents#available-tools) を継承します。
* **`tools` のみ**: subagent はリストされたツールのみを取得します。
* **`disallowedTools` のみ**: subagent はリストされたツール以外のすべての親ツールを取得します。
* **両方設定されている場合**: `disallowedTools` が優先されます。両方にリストされているツールは削除されます。

すべての場合において、解決されたセットは [subagent で利用可能なツール](/docs/ja/sub-agents#available-tools) に制限されます。subagent で利用可能でないツールは、`tools` にリストされている場合でも付与されることはありません。`SubagentHandback` ツールテーブルエントリの条件が成立する場合、Claude Code は `tools` から除外されているか `disallowedTools` にリストされている場合でも、subagent にそのツールを付与します。

subagent の `tools` リスト内のすべてのエントリが使用可能なツールと一致しない場合、Agent ツールは通常、subagent を起動する代わりにエントリを名前で指定するエラーを返します。[Agent would be spawned with zero tools](/docs/ja/errors#agent-would-be-spawned-with-zero-tools) でメッセージと各エントリの修正方法を参照してください。

subagent を起動すること自体は権限の確認を促しません。Claude Code は実行時に subagent 自身のツール呼び出しを権限ルールに対してチェックします。

subagent の権限プロンプトが表示される場所は、foreground で実行されるか background で実行されるかによって異なります。Claude Code は [foreground で実行されるケース](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) を除き、デフォルトで background で subagent を実行します。

* **Foreground subagents** は、メイン会話で見られるのと同じ権限プロンプトを表示し、各ツール呼び出しが発生した時点で表示されます。
* **Background subagents** v2.1.186 以降、メインセッションで権限プロンプトを表示します。プロンプトはどの subagent がリクエストしているかを名前で示し、Esc キーを押すとそのツール呼び出しのみを拒否し、subagent を停止しません。v2.1.186 より前は、background subagent は権限プロンプトが表示されるツール呼び出しを自動的に拒否し、そのツールなしで続行していました。

[subagent が到達できる範囲を制限](/docs/ja/sub-agents#control-subagent-capabilities) するには、まず `tools` フィールドを絞り込みます。例えば、Bash をリストから除外するか、設定で拒否ルールを設定します。

<h2 id="askuserquestion-tool-behavior">
  AskUserQuestion ツールの動作
</h2>

Claude は `AskUserQuestion` を使用して、決定や確認が必要な場合に複数選択肢の質問をします。オプションを選択して回答するか、`Other` 行またはメモフィールドを通じて独自のテキストを入力してください。

独自のテキストを入力して回答する場合、Claude Code は中立的な表現で回答をリレーするため、Claude はあなたが書いたものに従い、最初に待機または説明するよう要求します。

<h3 id="question-auto-continue-timeout">
  質問の自動継続タイムアウト
</h3>

質問は、あなたが回答するまで開いたままになります。回答しないままにした質問が最終的に閉じて Claude が あなたなしで続行できるようにしたい場合は、[`askUserQuestionTimeout`](/docs/ja/settings-reference#askuserquestiontimeout) 設定を `60s`、`5m`、または `10m` に設定します。これはユーザーの `settings.json` または `/config` の **Question auto-continue timeout** 行から設定できます。

質問がその期間入力なしで放置された後、ダイアログは自動的に閉じます。既に選択したオプションを送信し、Claude にあなたがキーボードから離れている可能性があることを伝えるため、Claude は独自の判断で進行し、後で再度質問できます。最後の 20 秒間のカウントダウンが表示されます。任意のキーを押してタイマーを再開します。フォーカスを報告するターミナルでは、ウィンドウに切り替えることでもタイマーが再開されます。

タイムアウトは `AskUserQuestion` の複数選択肢の質問にのみ適用されます。権限プロンプト（計画承認を含む）は、アイドル時に自動解決されることはありません。

<h2 id="bash-tool-behavior">
  Bash ツールの動作
</h2>

Bash ツールは各コマンドを別々のプロセスで実行します。

<h3 id="what-persists-between-commands">
  コマンド間で保持されるもの
</h3>

* Claude がメインセッションで `cd` を実行すると、新しい作業ディレクトリは、プロジェクトディレクトリ内に留まっている限り、または `--add-dir`、`/add-dir`、もしくは設定の `additionalDirectories` で追加した[追加の作業ディレクトリ](/docs/ja/permissions#working-directories)内に留まっている限り、後続の Bash コマンドに引き継がれます。これには、後続のメッセージに応答して Claude が実行するコマンドが含まれます。
  * サブエージェントセッションは作業ディレクトリの変更を引き継ぎません。
  * `cd` がこれらのディレクトリの外に出た場合、Claude Code はプロジェクトディレクトリにリセットし、ツール結果に `Shell cwd was reset to <dir>` を追加します。
  * この引き継ぎを無効にして、すべての Bash コマンドがプロジェクトディレクトリで開始されるようにするには、`CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1` を設定します。
* 環境変数は保持されません。1 つのコマンドで `export` しても、次のコマンドでは利用できません。
* シェルスタートアップファイルで定義されたエイリアスとシェル関数は利用可能です。セッション開始時に、Claude Code は `~/.zshrc`、`~/.bashrc`、または `~/.profile` をシェルに応じてソースし、結果のエイリアス、関数、シェルオプションをキャプチャして、すべての Bash コマンドに適用します。

Claude Code を起動する前に virtualenv または conda 環境をアクティベートしてください。環境変数を Bash コマンド間で保持するには、Claude Code を起動する前に [`CLAUDE_ENV_FILE`](/docs/ja/env-vars) をシェルスクリプトに設定するか、[SessionStart フック](/docs/ja/hooks#persist-environment-variables)を使用して動的に設定してください。

<h3 id="timeout-and-output-limits">
  タイムアウトと出力制限
</h3>

各コマンドはタイムアウト下で実行され、Claude がそれを管理します。コマンドにデフォルトより長い時間が必要な場合、その呼び出しで `timeout` パラメータを渡します。ユーザーがコマンドごとのタイムアウトを設定することはありません。2 つの[環境変数](/docs/ja/env-vars)が Claude が取得するものを制限します。

* `BASH_DEFAULT_TIMEOUT_MS` — Claude がタイムアウトを渡さない場合のデフォルト。デフォルトでは 2 分です。
* `BASH_MAX_TIMEOUT_MS` — デフォルトでは、Claude が要求するものを上限で制限します。有効な上限は 2 つの値の大きい方です。デフォルトでは 10 分です。

<h4 id="output-limits">
  出力制限
</h4>

Claude Code はコマンドの出力をコマンド実行中に作業ファイルにストリーミングします。出力が 5 GB を超えるコマンドは強制終了されます。コマンドが完了すると、Claude Code はそのファイルから出力を読み戻します。読み戻しウィンドウは以下で説明されています。出力のどの程度が Claude にインラインで到達するかは、Claude Code が結果を失敗として扱うかどうかによって異なります。

| 結果 | Claude が取得するもの                                                                                                                           |
| :- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| 有効 | デフォルトでは約 30,000 文字までインライン。それ以上の場合、セッションディレクトリに保存され 64 MiB を超える部分が切り詰められたファイルのパスと、先頭の最大 2,000 文字のプレビュー。残りが必要なときは、Claude がファイルを読み取るか検索します。 |
| 失敗 | 約 10,000 文字までインライン。それ以上の場合、読み戻しウィンドウから切り出された同じサイズの先頭と末尾の抜粋。ファイルパスはありません。                                                                 |

終了コード 1 で終了するコマンドは、Claude Code がそのコマンドに対して終了コード 1 を良好な結果として認識する場合にのみ、Bash ツールの有効な結果としてカウントされます。`grep`、`rg`、`egrep`、`fgrep`、`find`、`diff`、`test`、`[`、および `git diff` と `git grep` です。終了コード 1 で終了する他のすべてのコマンドは失敗としてカウントされます。終了コード 1 が良好な情報結果である場合でも同様です。`pgrep` と `jq -e` の一致なし、`cmp` の異なるファイルなど。

[`BASH_MAX_OUTPUT_LENGTH`](/docs/ja/env-vars) は、Claude Code が作業ファイルからコマンド結果に読み戻す出力の文字数を設定します。デフォルトは 30,000 文字で、ハード上限は 150,000 文字です。コマンドが定期的にそのウィンドウをオーバーフローする場合、例えば詳細なビルドまたは完全なテストスイートログの場合は、これを上げてください。これを上げると読み戻しウィンドウが拡大され、これは失敗したコマンドの抜粋が切り出されるウィンドウでもあります。インラインの上限は上げません。インラインの上限を超える有効な結果は、この変数に関係なくファイルパスとプレビューとして到達します。

有効な結果の Claude が受け取るインラインの量を変更するには、代わりに [`bashOutputMaxChars`](/docs/ja/settings-reference#bashoutputmaxchars) 設定を設定してください。最大 128,000 文字まで設定できます。これはインラインの上限と読み戻しウィンドウのサイズを一緒に設定し、Claude Code は `BASH_MAX_OUTPUT_LENGTH` を無視します。Claude Code v2.1.261 以降が必要です。

<h3 id="background-commands">
  バックグラウンドコマンド
</h3>

開発サーバーやウォッチビルドなどの長時間実行プロセスの場合、Claude は `run_in_background: true` を設定してコマンドをバックグラウンドタスクとして開始し、実行中に作業を続けることができます。`/tasks` でバックグラウンドタスクをリストアップして停止します。そこから停止するか、デスクトップアプリなどの接続されたクライアントから停止すると、Claude は待機する代わりに先に進みます。サブエージェントがコマンドを開始した場合、先に進むのはそのサブエージェントです。

[フォアグラウンドサブエージェント](/docs/ja/sub-agents#run-subagents-in-foreground-or-background)が開始したコマンドは、そのサブエージェントが最終応答を提供するときに停止します。メインの会話またはバックグラウンドサブエージェントが開始したコマンドは、最終応答の後も実行し続けます。`-p` フラグを使用した非対話型モードでは、[バックグラウンドコマンドは実行の最終結果の直後に終了します](/docs/ja/headless#background-tasks-at-exit)。

コマンドがタイムアウトに達しても完了しない場合、Claude Code はそれを停止する代わりにバックグラウンドに移動します。ただし、コマンドが `sleep` で始まる場合は除きます。Claude はコマンドが続行している間、作業を続けます。Claude Code は移動されたコマンドに他のバックグラウンドコマンドと同じライフタイムルールを適用するため、フォアグラウンドサブエージェントのコマンドはそのサブエージェントの最終応答で終了します。[`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/ja/env-vars#variables) を設定すると、バックグラウンドタスク機能の残りと共に自動バックグラウンド化を無効にします。

バックグラウンドに移動されたコマンドの結果は、何が起こったかを示します。

* タイムアウトが移動をトリガーする場合、結果は明示的に報告します。`Command did not complete within its 120s timeout and was moved to the background`。秒はタイムアウトに一致し、その後にタスク ID と出力が書き込まれているファイルのパスが続きます。
* バックグラウンドに移動されるコマンド内の `cd`、`pushd`、`popd`、または `chdir` は決して引き継がれません。結果は `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.` と述べているため、Claude は発生しなかったディレクトリ変更に対して行動しません。

<h3 id="memory-limit-on-linux-and-wsl">
  Linux と WSL のメモリ制限
</h3>

Linux と WSL では、[`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/ja/env-vars#variables) を `4G` などのサイズに設定して、Bash、PowerShell、および[Monitor](#monitor-tool) ツールコマンドが使用できるメモリを上限に設定し、1 つの暴走ビルドがセッションの残りが必要とするメモリを奪わないようにします。Claude Code v2.1.233 以降が必要です。v2.1.246 より前では、Monitor ツールコマンドは上限の外で実行されていました。

* サイズをバイト数として、または `K`、`M`、`G`、または `T` サフィックス付きで記述します。`0`、`off`、`false`、`no`、または `none` を設定して上限をオフにします。Claude Code は `4e9` などのサイズとして読み取ることができない他の値を無視します。
* Claude Code は、各コマンドごとではなく、1 つの上限に対してセッションのすべての Bash、PowerShell、および Monitor コマンドをカウントします。
* Claude Code はメモリ cgroup で上限を適用します。cgroup をセットアップできない場合、コマンドは上限なしで実行され、`claude --debug` からのデバッグログは理由を示します。
* Claude Code が開始した最初のプロセスが上限をオンにした後、またはオフ値または cgroup セットアップの失敗のためにオフにした後、Claude Code はその結果を保持します。変更または削除された値、または固定されたセットアップを適用するには、`claude` を再度起動します。
* コマンドが上限の下に留まることができない場合、カーネルはコマンドを強制終了し、その結果に上限を名前で示すものはありません。

Claude Code は、開始する他の種類のプロセスも同じ制限に対してカウントできます。[`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/ja/env-vars#variables) を上限から除外する種類のカンマ区切りリストに設定します。Claude Code はリストにない種類に上限を適用します。`none` に設定してすべての種類に上限を設定するか、`all-new` に設定して Bash、PowerShell、および Monitor ツールコマンドのみに上限を設定します。Claude Code v2.1.246 以降が必要です。名前を付けることができる種類は次のとおりです。

* `mcp`: ローカル[MCP サーバー](/docs/ja/mcp)
* `lsp`: [言語サーバー](#lsp-tool-behavior)
* `hooks`: [フック](/docs/ja/hooks)コマンド
* `plugin`: [プラグイン](/docs/ja/plugins/overview)が実行するコマンド
* `helper`: Claude Code 独自のヘルパーコマンド（`git` など）
* `agent`: [エージェントチームメイト](/docs/ja/agent-teams)などの子 Claude Code プロセス

リストするものに関係なく、これらのルールが適用されます。

* **不明な名前**: Claude Code は認識しない名前を無視します。
* **Bash、PowerShell、および Monitor**: Claude Code は、リストするものに関係なく、Bash、PowerShell、および Monitor ツールコマンドを上限の下に保ちます。
* **変数が設定されていない**: Claude Code は、Anthropic がサーバーから配信する設定から他のキャップされた種類のセットを取得し、そのセットは時間とともに変わる可能性があるため、変更されないセットが必要な場合は変数を設定します。
* **権限ゲーティングフック**: すべての種類がキャップされている場合でも、Claude Code はアクションをブロックまたは変更できるフック、およびそのようなフックが呼び出す MCP サーバーを上限から除外するため、カーネルが権限ゲーティングフックを強制終了してもブロックしていたアクションを許可することはできません。

<h2 id="edit-tool-behavior">
  Edit ツールの動作
</h2>

Edit ツールは正確な文字列置換を実行します。`old_string` と `new_string` を受け取り、最初のものを 2 番目のものに置き換えます。正規表現やあいまい一致は使用しません。

編集を適用するには、3 つのチェックに合格する必要があります。その前に、[`Read` 拒否ルール](/docs/ja/permissions#tool-specific-permission-rules)に一致するパスは拒否されます。新しいファイルをそこに作成することも含まれます。この拒否には Claude Code v2.1.208 以降が必要です。

* **Read-before-edit**: Claude は編集前に現在の会話でファイルを読み取り、[`PARTIAL view` 通知](#read-tool-behavior)で短縮された読み取りはカウントされません。Claude Opus 4.6、Claude Haiku 4.5、およびそれ以前のモデルは常に読み取りが必要です。新しいモデルは、読み取りが権限プロンプトを必要としない場合、および Read ツールが利用可能な場合、読み取られていないファイルを編集できます。
* **Match**: `old_string` はファイルに記述されたとおりに正確に表示される必要があります。空白またはインデントの 1 文字の違いでも一致を逃す可能性があります。
* **Uniqueness**: `old_string` は正確に 1 回だけ表示される必要があります。複数回表示される場合、Claude は 1 つの出現を特定するのに十分な周囲のコンテキストを含む長い文字列を提供するか、`replace_all: true` を設定してすべてを置換します。

Claude が最後に読み取った後、ディスク上で変更されたファイルは、`old_string` が現在のコンテンツと正確かつ明確に一致し、Claude Code がプロンプトなしでファイルを読み取ることができる場合でも編集できます。ファイルの現在のコンテンツに対してマッチングすることでこれを安全に保ち、結果はファイルが他の変更を含むことを示すため、Claude は周囲のコンテンツに依存する編集の前に再度読み取ります。古い `old_string` や `replace_all` なしで複数回一致するものなど、その他の場合は、Claude は編集前にファイルを再度読み取ります。読み取られていないファイルと変更されたファイルの緩和された処理には Claude Code v2.1.208 以降が必要です。それ以前は、Claude Code は会話で読み取られていないファイルまたは読み取り後にディスク上で変更されたファイルへの編集を拒否していました。

Bash でファイルを表示することは、コマンドが `cat`、`nl`、`bat`、`batcat`、`head`、`tail`、`sed -n 'X,Yp'`、`grep`、`egrep`、`fgrep`、または `rg` である場合、パイプまたはリダイレクトなしで単一ファイルに対して read-before-edit 要件を満たします。パイプされた出力およびその他の Bash コマンドは read-before-edit チェックにはカウントされません。

Bash でファイルを表示することは、権限ではなく編集適格性にのみ影響します。[Read と Edit の権限ルール](/docs/ja/permissions#read-and-edit)を参照して、`Read` と `Edit` 拒否ルールがどの Bash コマンドをカバーするかを確認してください。

<h2 id="endconversation-tool-behavior">
  EndConversation ツールの動作
</h2>

EndConversation ツールは現在のセッションを終了します。Claude がこれを使用するのは 2 つの状況のみです。

* 継続的な不適切な入力に対する最後の手段として、会話をリダイレクトする試みが失敗し、前のメッセージで明確な警告を出した後
* カスタマーが明示的にツールのデモンストレーションを見たいと要求し、セッションを終了したいことを確認した場合

一般的なフラストレーション、下品な言葉遣い、またはタスクが上手くいかないことは該当しません。有害なコンテンツのリクエストも該当しません。Claude はセッションを終了する代わりにそれらを拒否します。Claude Code は claude.ai と同じアプローチに従い、[チャットの限定的なサブセットを終了](https://www.anthropic.com/research/end-subset-conversations)することができます。

Claude がインタラクティブセッションを終了した後、セッションはロックされます。新しいプロンプトとほとんどのコマンドは `Claude ended this conversation. Start a new session (or /clear) to continue.` を返し、`/clear`、`/resume`、`/help`、`/exit`、および `/feedback` のみが実行されます。Claude Code はセッションのトランスクリプトに終了を記録するため、終了したセッションを再開するとロックが復元されます。セッションの履歴は削除されません。

[非インタラクティブモード](/docs/ja/headless)で `-p` フラグを使用して終了したセッションを再開するとエラーが発生し、コード 1 で終了するため、スクリプトは終了した実行を成功として読み取りません。

このツールは権限を求めるプロンプトを表示することはなく、[PreToolUse フック](/docs/ja/hooks#pretooluse)はこのツールに対して実行されません。他のツールが残っている間は、これをブロックすることもできません。[EndConversation という名前の deny and ask ルール](/docs/ja/permissions#tool-specific-permission-rules)は効果がなく、`--disallowedTools` も `--tools` リストもこれを削除できません。この除外は意図的です。このツールはセッションを終了する以外に何もしません。ファイルやデータを読み取ったり変更したりすることはなく、この種のセーフガードは、それが適用されるセッションがそれをオフにできない場合にのみ機能します。deny ルールが他のすべてのツールを削除し、`"*"` のように EndConversation にも一致する場合、Claude Code は allow ルールが EndConversation を明示的に指定しない限り、唯一のツールとして残すのではなく、それを削除します。EndConversation に一致しずに他のすべてのツールを削除する deny リストは、それを所定の位置に残します。

[サブエージェント](/docs/ja/sub-agents)はこのツールを取得しません。メインの会話のツールリストを共有するバックグラウンドタスクはそれを見ますが、そこでそれを呼び出しても何も終了しません。

このツールは以下のすべてが当てはまる場合にのみ表示されます。

* **バージョン**: Claude Code v2.1.213 以降。
* **モデル**: セッションのモデルが Claude Opus 4.8、Claude Sonnet 5、Claude Fable 5、またはこれらのファミリーの後続バージョンです。
* **サーフェス**: インタラクティブターミナルセッション。IDE の統合ターミナルの `claude` セッションを含みます。これは [JetBrains プラグイン](/docs/ja/jetbrains)がそれを実行する方法です。他のサーフェスはこのツールを含みません。例えば：
  * 非インタラクティブな `-p` 実行
  * [Agent SDK](/docs/ja/agent-sdk/overview) TypeScript および Python パッケージを通じたセッション
  * [VS Code 拡張機能](/docs/ja/vs-code)パネル。独自の CLI をバンドルしています
  * [GitHub Actions](/docs/ja/github-actions)
  * [Claude Code on the web](/docs/ja/claude-code-on-the-web)
* **スタートアップモード**: [`--bare`](/docs/ja/headless#start-faster-with-bare-mode) セッションではありません。ベアモードはシェルとファイルツールのみをロードするため、このツールはそこに登録されません。
* **プロバイダー**: [Amazon Bedrock](/docs/ja/amazon-bedrock)、[Claude Platform on AWS](/docs/ja/claude-platform-on-aws)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、または [Microsoft Foundry](/docs/ja/microsoft-foundry) では利用できません。また、[クラウドゲートウェイ](/docs/ja/claude-apps-gateway)を通じてサインインしたセッションでも利用できません。

<h2 id="glob-tool-behavior">
  Glob ツールの動作
</h2>

Glob ツールはファイル名パターンでファイルを検索します。Windows では、デフォルトツールセットの一部です。macOS、Linux、および WSL では、Claude Code は Glob と [Grep](#grep-tool-behavior) をデフォルトツールセットから除外し、Claude は Bash ツールを通じて `find` と `grep` で検索します。Claude のシェルでは、これら 2 つのコマンドは `bfs` と `ugrep` の組み込みバージョンを実行し、検索は Bash 呼び出しとしてフック と権限ルールに到達します。

macOS、Linux、および WSL では、以下の場合に Glob と Grep ツールが戻ります。

* セッションを開始するときに [`--tools` または `--allowedTools`](/docs/ja/cli-reference#cli-flags) で `Glob` または `Grep` を指定するか、同等の [Agent SDK](/docs/ja/agent-sdk/overview) オプションで指定します。`--tools` を使用すると、リストしたものが取得され、`--allowedTools` でいずれかのツールを指定すると両方が復元されます。設定ファイルの許可ルールはこの効果を持ちません。
* 権限 [拒否ルール](/docs/ja/permissions#match-all-uses-of-a-tool)、`--disallowedTools` フラグ、または [`--restricted`](/docs/ja/cli-reference#cli-flags) がセッションから `Bash` を削除します。
* [サブエージェント](/docs/ja/sub-agents#available-tools) が `tools` フィールドに `Glob` または `Grep` をリストし、`Bash` を除外します。リストされたツールはそのサブエージェントのみ、または [`--agent`](/docs/ja/sub-agents#invoke-subagents-explicitly) またはエージェント設定を通じてメインセッションエージェントとして実行される場合はセッション全体に戻ります。

Glob は再帰的なディレクトリマッチングのための `**` を含む標準的な glob 構文をサポートしています。

* `**/*.js` は任意の深さにあるすべての `.js` ファイルにマッチします
* `src/**/*.ts` は `src/` 以下のすべての `.ts` ファイルにマッチします
* `*.{json,yaml}` は現在のディレクトリ内の `.json` および `.yaml` ファイルにマッチします

結果は変更時刻でソートされ、100 ファイルの上限があります。上限に達した場合、Claude は結果に切り詰めフラグを表示し、パターンを絞り込むことができます。

Glob はデフォルトでは `.gitignore` を尊重しないため、gitignore されたファイルと追跡されたファイルの両方を検索します。これは gitignore されたファイルをスキップする [Grep](#grep-tool-behavior) とは異なります。Glob が `.gitignore` を尊重するようにするには、Claude Code を起動する前に `CLAUDE_CODE_GLOB_NO_IGNORE=false` を設定してください。

Claude Code は Glob 呼び出しの権限を決定してから、検索ディレクトリが存在するかどうかを確認します。[作業ディレクトリ](/docs/ja/permissions#working-directories) 外の存在しない `path` に対しても読み取り権限チェックを実行するため、パスの権限プロンプトはそのパスが存在することを意味しません。

null バイトを含む `pattern` または `path` 値はエラーを返し、Claude にそれを削除するよう求めます。

<h2 id="grep-tool-behavior">
  Grep ツールの動作
</h2>

Grep ツールはファイルの内容からパターンを検索します。[Glob](#glob-tool-behavior) がファイル名でファイルを検索するのに対し、Grep はそれらの内部の行を検索します。macOS、Linux、WSL では、Grep は Glob と同じ条件下ではデフォルトで存在しません。両方のツールが利用可能な場合については、[Glob ツールの動作](#glob-tool-behavior) を参照してください。

Grep は [ripgrep](https://github.com/BurntSushi/ripgrep) に基づいており、POSIX grep ではなく ripgrep の正規表現構文を使用します。正規表現のメタ文字を含むパターンはエスケープが必要です。例えば、Go コードで `interface{}` を検索する場合、パターン `interface\{\}` が必要です。

ripgrep が拒否するパターン、glob、またはファイルタイプは、ripgrep の診断を含むエラーを返すため、Claude は入力を修正して再度検索できます。v2.1.208 より前では、Claude Code は拒否された入力を、検索対象のテキストが対象ファイルに存在する場合でも、エラーではなく `No files found` として報告していました。

3 つの出力モードが何を返すかを制御します。

* `files_with_matches`: ファイルパスのみで、行の内容はありません。これがデフォルトです。
* `content`: ファイルと行番号を含む一致する行。ツールの `offset` パラメータが一致を持つパターンの最後の一致を超えて指す場合、Grep は `No entries at this offset` を返すため、Claude はパターンが一致しないと結論付けるのではなく、offset を拡大またはリセットします。
* `count`: ファイルごとの一致数、その後すべての一致するファイル全体の合計。合計はツールの `head_limit` または `offset` パラメータがリストされたファイルごとのエントリを切り詰める場合でも、すべての一致をカバーします。v2.1.208 より前では、合計はリストされたエントリのみを合計していました。

Claude は `glob` パラメータ（例：`**/*.tsx`）でファイルごとに結果をスコープできます。または `type` パラメータ（例：`py` または `rust`）で言語ごとにスコープできます。デフォルトでは、パターンは単一行内で一致します。Claude は `multiline: true` を設定して、行の境界を越えて一致させることができます。

Grep は `.gitignore` を尊重するため、gitignore されたファイルはスキップされます。gitignore されたファイルを検索するには、Claude はそのパスを直接渡します。

Claude Code は Grep 呼び出しの権限を、検索 `path` が存在するかどうかを確認する前に決定します。[作業ディレクトリ](/docs/ja/permissions#working-directories) 外の存在しない `path` に対して読み取り権限チェックを実行するため、パスの権限プロンプトはそのパスが存在することを意味しません。

<h2 id="lsp-tool-behavior">
  LSP ツールの動作
</h2>

LSP ツールは、実行中の言語サーバーから Claude にコード インテリジェンスを提供します。ファイルを編集するたびに、型エラーと警告を自動的に報告するため、Claude は別のビルド ステップなしで問題を修正できます。Claude はコードをナビゲートするために直接呼び出すこともできます。

* シンボルの定義にジャンプ
* シンボルへのすべての参照を検索
* 位置での型情報を取得
* ファイル内のシンボルをリスト表示
* ワークスペース全体でシンボルを名前で検索
* インターフェースの実装を検索
* コール階層をトレース

Claude Code は、言語の[コード インテリジェンス プラグイン](/docs/ja/plugins/code-intelligence)をインストールするまで、ツールを非アクティブに保ちます。[クラウド セッション](/docs/ja/claude-code-on-the-web)では、Claude Code はプラグイン言語サーバーを起動しないため、LSP ツールはそこで非アクティブなままです。Claude Code は言語サーバーの設定をプラグインから取得し、サーバー バイナリは自分でインストールします。

Claude Code は、言語サーバーを起動できないファイルの各 LSP 呼び出しに対してエラー結果を返します。

<h2 id="monitor-tool">
  Monitor ツール
</h2>

Monitor ツールは Claude がバックグラウンドで何かを監視し、会話を一時停止することなく変更時に反応することができます。Claude に以下のことを依頼できます。

* ログファイルをテールして、エラーが表示されたらフラグを立てる
* PR または CI ジョブをポーリングして、ステータスが変更されたときに報告する
* ディレクトリのファイル変更を監視する
* 指定した長時間実行スクリプトからの出力を追跡する
* WebSocket フィードに接続して、到着した各メッセージを報告する

ほとんどの監視では、Claude は小さなスクリプトを作成し、バックグラウンドで実行し、到着した各出力行を受け取ります。イベントをプッシュするサーバーの場合、Claude はスクリプトを実行する代わりに [WebSocket](#websocket-source) を開くことができます。

同じセッションで作業を続けることができ、イベントが到着すると Claude が割り込みます。

Claude が開始するすべての監視には期限があります。デフォルトでは 5 分、最大 30 分、および `-p` で単一のプロンプトが与えられた [非対話型](/docs/ja/headless) 実行では最大 10 分です。

期限に達すると監視が終了します。Claude は 1 つの通知を受け取るため、必要に応じて監視を再度開始できます。

Monitor をキャンセルするよう Claude に依頼するか、セッションを終了することで Monitor を停止できます。例えば `/tasks` から開始された Monitor を停止する [subagent](/docs/ja/sub-agents) を停止すると、それらの Monitor も一緒に停止します。

Monitor がコマンドを実行するとき、[Bash と同じ権限ルール](/docs/ja/permissions#tool-specific-permission-rules) を使用するため、Bash に設定した `allow` および `deny` パターンがここにも適用されます。[auto mode](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) がアクティブな場合、Claude Code は `Monitor` 自体を名前に含む allow ルール、および [削除する他のブロード allow ルール](/docs/ja/permission-modes#how-the-classifier-evaluates-actions) を脇に置くため、分類器は Monitor コマンドを Bash コマンドと同じ方法で確認します。

[WebSocket ソース](#websocket-source) には独自の承認プロンプトがあり、分類器も auto mode で決定します。

このツールは Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry では利用できません。また、`DISABLE_TELEMETRY` または `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` が設定されている場合も利用できません。

プラグインは Claude に開始を依頼する代わりに、プラグインがアクティブな場合に自動的に開始される Monitor を宣言できます。[プラグイン Monitor](/docs/ja/plugins/components#monitors) を参照してください。

<h3 id="websocket-source">
  WebSocket ソース
</h3>

<Note>
  WebSocket ソースには Claude Code v2.1.195 以降が必要です。
</Note>

サーバーが既に WebSocket 経由でイベントをプッシュしている場合、Claude はポーリング スクリプトを作成する代わりに直接接続できます。各種のソケット アクティビティはイベントになるか、監視を終了します。

* **テキスト メッセージ**: メッセージが複数行にまたがる場合でも、各メッセージが 1 つのイベントになります。
* **バイナリ メッセージ**: 渡されません。Claude は `[binary frame, 512 bytes]` などのプレースホルダー行を受け取ります。
* **1 MiB より大きいメッセージ**: 監視が終了するため、フィルタリングされたフィードが存在する場合はそれにサブスクライブしてください。
* **ソケット クローズ**: 監視が終了し、Claude はクローズ コードを受け取ります。

WebSocket 監視は `command` の代わりに `ws` 入力を取り、単一の Monitor 呼び出しは 2 つを組み合わせることはできません。`ws` 入力には 2 つのフィールドがあります。

| フィールド       | 必須  | 説明                                                                                       |
| :---------- | :-- | :--------------------------------------------------------------------------------------- |
| `url`       | はい  | 接続するエンドポイント。埋め込まれた認証情報またはホワイトスペースなしで、ASCII 文字のみを使用する `ws://` または `wss://` URL である必要があります |
| `protocols` | いいえ | ハンドシェイク中に提供する WebSocket サブプロトコル名。各エントリは有効なサブプロトコル トークンである必要があり、リストに重複を含めることはできません        |

`timeout_ms` 期限は WebSocket 監視にも適用されます。監視は期限で終了し、`TaskStop` は早期にキャンセルします。

WebSocket を開くと承認を求めるプロンプトが表示されます。[auto mode](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) では分類器が代わりに決定します。プロンプトは同じホストの将来のプロンプトをスキップするオプションを提供しません。

Claude Code は、ホスト名が 1 つに解決される場合を含む、プライベート、リンク ローカル、またはクラウド メタデータ アドレスを指すURL を拒否します。また、`sandbox.network.deniedDomains` 内のホストも拒否し、マネージド設定で [`allowManagedDomainsOnly`](/docs/ja/settings-reference#sandbox-network-allowmanageddomainsonly) が設定されている場合、マネージド許可リスト外のホストも拒否します。

<h2 id="notebookedit-tool-behavior">
  NotebookEdit ツールの動作
</h2>

NotebookEdit は、Jupyter ノートブックを 1 回に 1 セル、`cell_id` でセルをターゲットにして変更します。[Edit](#edit-tool-behavior)がプレーン ファイルで行うようにノートブック全体で文字列置換を実行しません。

3 つの編集モードは、ターゲット セルに何が起こるかを制御します：

* `replace`：セルのソースを上書きします。これがデフォルトです。
* `insert`：ターゲットの後に新しいセルを追加します。`cell_id` がない場合、新しいセルはノートブックの開始に移動します。`cell_type` を `code` または `markdown` に設定する必要があります。
* `delete`：ターゲット セルを削除します。

権限ルールは `Edit(...)` パス形式を使用します。`Edit(notebooks/**)` のようなルールは、そのディレクトリ内のファイルに対する NotebookEdit 呼び出しをカバーします。

<h2 id="powershell-tool">
  PowerShell ツール
</h2>

PowerShell ツールを使用すると、Claude は PowerShell コマンドをネイティブに実行できます。Windows では、これはコマンドが Git Bash を経由するのではなく PowerShell で実行されることを意味します。ツールが利用可能になる方法はプラットフォームによって異なります。

* **Git Bash がない Windows**: ツールは自動的に有効になります。
* **Git Bash がインストールされている Windows**: ツールは claude.ai と Console アカウントではデフォルトで有効です。Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry セッションで有効にするには `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` を設定するか、無効にするには `0` を設定します。
* **Linux、macOS、WSL**: ツールはオプトインです。

[PreToolUse フック](/docs/ja/hooks#powershell)は、Bash ツールと同じフィールドを持つ `tool_input.command` でツールのコマンド文字列を受け取ります。

シェルコマンドを検査するフックで `Bash|PowerShell` と一致させます。[PowerShell フック入力セクション](/docs/ja/hooks#powershell)は、`Bash` だけと一致させるだけでは十分でない理由を説明しています。

<h3 id="enable-the-powershell-tool">
  PowerShell ツールを有効にする
</h3>

環境または `settings.json` で `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` を設定します。

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

Windows では、ツールを無効にするために変数を `0` に設定します。Linux、macOS、WSL では、ツールは PowerShell 7 以降が必要です。`pwsh` をインストールして、`PATH` に含まれていることを確認します。

Windows では、Claude Code は PowerShell 7 以降の `pwsh.exe` を自動検出し、PowerShell 5.1 の `powershell.exe` にフォールバックします。ツールが有効な場合、Claude は PowerShell をプライマリシェルとして扱います。Git Bash がインストールされている場合、Bash ツールは POSIX スクリプト用に利用可能なままです。

Claude Code は PowerShell をプロセススコープのみで `-ExecutionPolicy Bypass` で起動するため、`.ps1` スクリプトとモジュールインポートは、マシンのポリシーを変更することなく、デフォルトの Windows インストールで機能します。プロセススコープのバイパスは、グループポリシーの `MachinePolicy` または `UserPolicy` をオーバーライドしないため、エンタープライズポリシーは引き続き適用されます。マシンの有効な実行ポリシーを尊重するには、`CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1` を設定します。

<h3 id="shell-selection-in-settings-hooks-and-skills">
  設定、フック、スキルでのシェル選択
</h3>

3 つの追加設定は PowerShell が使用される場所を制御します。

* [`settings.json`](/docs/ja/settings-reference#all-settings) の `"defaultShell": "powershell"`: インタラクティブな `!` コマンドを PowerShell を経由してルーティングします。PowerShell ツールが有効になっている必要があります。
* 個別の[コマンドフック](/docs/ja/hooks#command-hook-fields)の `"shell": "powershell"`: そのフックを PowerShell で実行します。フックは PowerShell を直接起動するため、`CLAUDE_CODE_USE_POWERSHELL_TOOL` に関係なく機能します。
* [スキルフロントマター](/docs/ja/skills#frontmatter-reference)の `shell: powershell`: `` !`command` `` ブロックを PowerShell で実行します。PowerShell ツールが有効になっている必要があります。

Bash ツールセクションで説明されているのと同じメインセッションの作業ディレクトリリセット動作が PowerShell コマンドに適用されます。これには `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR` 環境変数が含まれます。

v2.1.196 以降、`grep`、`rg`、`egrep`、`fgrep`、`findstr`、`git grep` からの終了コード 1 は一致がないことを意味します。`git diff` からの終了コード 1 は差分が存在することを意味します。どちらの結果も Claude にコマンド失敗として報告されません。`robocopy` の場合、終了コード 0 から 7 は情報結果です。コピーされたファイルや検出された追加ファイルなどです。終了コード 8 以上は失敗としてカウントされます。

<h3 id="windows-encoding-and-exit-codes">
  Windows エンコーディングと終了コード
</h3>

Windows では、以下の PowerShell エンコーディングと終了コード動作には Claude Code v2.1.214 以降が必要です。

* `>` と `>>` でのリダイレクトは PowerShell 5.1 で UTF-8 ファイルを書き込みます
* Claude Code はネイティブコマンドの標準入力にパイプされたテキストを UTF-8 としてエンコードします
* Claude Code は ANSI エスケープシーケンスなしでエラー出力をキャプチャします
* 子プロセスが標準入力で待機するコマンドは、ハングするのではなくファイルの終わりを受け取ります
* `where.exe` からの終了コード 1 は一致がないことを意味し、`fc.exe` と `diff.exe` からはファイルが異なることを意味するため、コマンドが出力を生成する場合、Claude Code はその終了コードをコマンドエラーではなく有効な否定的な回答として扱います。Claude Code は `where.exe /Q` または `$null` へのリダイレクトなどのサイレント形式を終了コード 1 での失敗として報告します

v2.1.214 より前では、PowerShell 5.1 の `>` は UTF-16LE ファイルを書き込み、非 ASCII パイプ入力は `?` として到着し、Python スクリプトは非 ASCII 文字を出力するときに `UnicodeEncodeError` でクラッシュする可能性がありました。

<h3 id="preview-limitations">
  プレビューの制限事項
</h3>

PowerShell ツールには、プレビュー中に以下の既知の制限事項があります。

* PowerShell プロファイルは読み込まれません
* Windows では、サンドボックス化はサポートされていません

<h2 id="read-tool-behavior">
  Read ツールの動作
</h2>

Read ツールはファイルパスを受け取り、行番号付きでコンテンツを返します。Claude は常に絶対パスを渡すように指示されています。

デフォルトでは、Read はファイルの開始から返します。ファイル全体の読み込みがトークン制限を超える場合、Read は最初のページを `PARTIAL view` 通知とともに返し、Claude が受け取ったファイルの量と `offset` および `limit` を使用してさらに読み込む方法を伝えます。明示的な `offset` または `limit` を渡す読み込みがトークン制限を超える場合、エラーが返されます。

明示的な `limit` を指定した読み込みは、選択された行がトークン制限に収まる量を超えるとすぐに停止し、範囲の残りを読み込まずにエラーを返します。エラーは Claude に、より小さい `limit` を使用するか、単一行がそれほど大きい場合は [Grep](#grep-tool-behavior) で特定のコンテンツを検索するよう指示します。v2.1.208 より前では、Claude Code は範囲全体をメモリに読み込んでから拒否していたため、非常に長い単一行を持つファイルを読み込むとメモリ不足になる可能性がありました。

空のファイルを読み込むと、ファイルは存在するがコンテンツが空であることを示す通知が返され、最後の行を超えた `offset` はファイルの行数を示す通知を返します。v2.1.208 より前では、空のファイルを読み込むと末尾を超えた通知が返されていました。

Read は平文テキスト以外のいくつかのファイルタイプを処理します。

* **画像**: PNG、JPG、およびその他の画像形式は、生バイトではなく Claude が見ることができるビジュアルコンテンツとして返されます。Claude Code は大きな画像をモデルの画像サイズ制限に合わせるようにリサイズして再圧縮してから送信するため、Claude は大きなスクリーンショットのダウンスケール版を見る可能性があります。v2.1.196 以降、そのリサイズ後も 500KB より大きい画像は、ピクセル寸法を変更せずに品質を低下させた JPEG として再エンコードされます。Claude が大きな画像の細かいピクセルレベルの詳細を見落とした場合、ImageMagick を使用して Bash で領域をトリミングするなど、関心のある領域を最初にトリミングするよう指示してください。
* **PDF**: Claude は短い `.pdf` ファイルを全体として読み込みます。10 ページを超える PDF の場合、`pages` パラメータ（例：`"1-5"`）を使用して範囲で読み込み、一度に最大 20 ページまで読み込みます。
* **Jupyter ノートブック**: `.ipynb` ファイルは、コード、マークダウン、ビジュアライゼーションを含むすべてのセルとその出力を返します。Claude Code は 100 MB を超えるノートブックファイルの読み込みを拒否します。エラーは Claude に、Bash シェルコマンドを使用してセルのスライスなど、ノートブックの一部を読み込む方法を指示します。

Read はファイルのみを読み込み、ディレクトリは読み込みません。Claude は `ls` などのシェルコマンドを使用してディレクトリコンテンツをリストします。

<h2 id="sendfeedback-tool-behavior">
  SendFeedback ツールの動作
</h2>

Claude が作成したフィードバックは、Claude Code に関する Claude が作成するフィードバックレポートです。Claude Code v2.1.238 以降が必要です。Claude Code は各ドラフトをマシン上の `~/.claude/feedback/drafts/` に保存し、送信するまで何も Anthropic に到達しません。Claude は以下の場合に SendFeedback ツールでドラフトを作成します。

* ツールまたはコマンドが失敗し続ける
* 要求されたことに対応できない
* 自分が犯した間違いを指摘するか、それに気付く
* フィードバックを提出するよう要求する

<h3 id="what-you-see-when-claude-drafts">
  Claude がドラフトを作成するときに表示される内容
</h3>

Claude がドラフトをキューに入れた後、プロンプトの上にドラフトのタイトルが表示されたカードが表示されます。`1` を押してドラフトを確認し、`2` を 2 回押して記述されたとおりに送信するか、`0` を押して却下します。却下されたドラフトはキューに残ります。カードを却下した後、Claude Code は Claude が作成したフィードバックをオフにするかどうかを尋ねます。2 回拒否すると、質問が停止します。

デフォルトでは、セッションで最大 3 つのカードが表示されます。Anthropic はリリースなしでサーバーからその制限を調整できます。制限後、および [`feedbackDrafts`](/docs/ja/settings-reference#feedbackdrafts) を `quiet` に設定するたびに、プロンプトフッターにキューに入れられたドラフトの数のみが表示されます。

<h3 id="review-and-edit-a-draft">
  ドラフトを確認して編集する
</h3>

引数なしで `/feedback` を実行してキューを開きます。これにより、カードを却下したセッションや表示されなかったセッションを含む、すべてのセッションからキューに入れられたすべてのドラフトが一覧表示されます。ドラフトを選択して確認用に開くと、以下を実行できます。

* タイトル、領域、詳細を編集する
* **トランスクリプトを送信** を `yes` または `no` に設定します。Claude がドラフトをキューに入れたセッションからのトランスクリプトがまだ利用可能な場合、`yes` で開始され、その会話を Anthropic に送信します。`no` はレポートのみを送信します
* ドラフトを送信するか、破棄するか、後で使用するためにキューに残す

代わりに自分でレポートを作成するには、`w` を押して標準フィードバックダイアログを開きます。その後のテキストを含む `/feedback` と `/bug` がそのダイアログを直接開きます。

<h3 id="send-a-draft">
  ドラフトを送信する
</h3>

ドラフトを送信すると、Claude Code は `/feedback` レポートと同じ方法で送信し、同じ [保持期間](/docs/ja/data-usage#feedback-using-the-%2Ffeedback-command) を使用し、マシンからドラフトを削除します。カードから送信すると、`✓ Sent` が表示されます。キューから送信すると、受信 ID で閉じます。

レポートには以下が含まれます。

* タイトル、領域、詳細
* Claude Code バージョン、オペレーティングシステム、モデルなどの環境情報
* 最近の API リクエストの ID
* 確認画面で **トランスクリプトを送信** を `yes` のままにした場合の会話トランスクリプト。カードから送信する場合、トランスクリプトは含まれません

Claude Code はローカルドラフトに作業ディレクトリを保持してトランスクリプトを見つけることができ、ディレクトリは送信しません。

[ゼロデータ保持を使用する組織](/docs/ja/zero-data-retention#features-disabled-under-zdr) では、Claude Code は `/feedback` と同様にツールを除外します。そのような組織のセッションがまだツールを提供している場合、ドラフトはマシンに残り、送信は `Feedback collection is not available for organizations with custom data retention policies.` で失敗します。

<h3 id="discard-or-keep-a-draft">
  ドラフトを破棄または保持する
</h3>

ドラフトを破棄すると、Claude Code はマシンからそれを削除します。キューに残すドラフトは 30 日後、または [`cleanupPeriodDays`](/docs/ja/settings-reference#cleanupperioddays) がより短い場合はその後に期限切れになります。キューはすべてのセッション全体で 10 個のドラフトを保持し、Claude が 11 番目をキューに入れると、Claude Code は最も古いものを削除します。セッションからのドラフトがまだキューに入っている状態で `/exit` を実行すると、Claude Code は終了する前にそれらを確認するか破棄するかを尋ねます。

<h3 id="turn-claude-drafted-feedback-off">
  Claude が作成したフィードバックをオフにする
</h3>

`/config` で **Claude が作成したフィードバック** を `off` に設定します。これにより [`feedbackDrafts`](/docs/ja/settings-reference#feedbackdrafts) 設定が書き込まれるか、1 つのセッションに対して [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/ja/env-vars) を設定します。どちらでも、Claude はドラフトをキューに入れることができません。カードなしでドラフトを続行するには、代わりに `feedbackDrafts` を `quiet` に設定します。管理者は [管理設定](/docs/ja/managed-settings) で `feedbackDrafts` を設定でき、これは独自の設定よりも優先されます。

<h3 id="sessions-without-claude-drafted-feedback">
  Claude が作成したフィードバックなしのセッション
</h3>

Claude Code には、Claude API を使用する独自のマシン上のインタラクティブターミナルセッションにツールが含まれています。以下からツールを除外します。

* キューを確認する画面がない非インタラクティブ `-p` 実行と [Agent SDK](/docs/ja/agent-sdk/overview) セッション
* マシン上のキューに書き込むことができない [Claude Code on the web](/docs/ja/claude-code-on-the-web) などのクラウドセッション
* [Amazon Bedrock](/docs/ja/amazon-bedrock)、[Claude Platform on AWS](/docs/ja/claude-platform-on-aws)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、または [Microsoft Foundry](/docs/ja/microsoft-foundry) 上のセッション
* [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/ja/env-vars) または [`DISABLE_FEEDBACK_COMMAND=1`](/docs/ja/env-vars) を設定したセッション、`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` を空でない値に設定したセッション、または [機能フラグ取得](/docs/ja/env-vars#features-that-need-feature-flag-fetching) をオフにしたセッション
* 製品フィードバックをオフにした組織、および [ゼロデータ保持を使用する組織](/docs/ja/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Task ツールの利用可能性
</h2>

Task トラッキングツール（`TaskCreate`、`TaskGet`、`TaskUpdate`、`TaskList`、および `TodoWrite`）は、デフォルトでは Claude 3.x モデル、Opus 4 から 4.7、Sonnet 4 から 4.6、および Haiku 4.5 でのみ利用可能です。ツールが利用可能な場所では、4 つの Task ツール、または [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/ja/env-vars) を設定した場合は `TodoWrite` が提供されます。

他のすべてのモデルでは、Claude Code はオプトインしない限りツールを除外します。Claude Code が認識しないモデル ID（[LLM ゲートウェイ](/docs/ja/llm-gateway) を通じて提供されるカスタムモデル名など）にも同じことが当てはまります。新しいモデルでは、Claude は書かれたチェックリストなしで複数ステップの作業を追跡でき、ツールの定義とリマインダーはコンテキストを占有します。ツールがない場合、Claude は作業中に[タスクリスト](/docs/ja/interactive-mode#task-list)に何も追加しません。

デフォルトではこれらのツールを持たないモデルでこれらのツールを使用したい場合は、以下のいずれかを実行してください：

* Claude Code を開始する前に [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/ja/env-vars) をエクスポートします。例えば `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`。Claude Code はすべてのモデルとすべてのプロバイダーで同じツールを提供します
* [`--allowedTools`](/docs/ja/cli-reference#cli-flags) で、例えば `claude --allowedTools TaskCreate` のようにツールの 1 つを指定します
* [`--tools`](/docs/ja/cli-reference#cli-flags) でツールをリストします。これはセッションの組み込みツールを指定されたものに制限します。使用する他の組み込みツールと一緒に必要なツールを含めます
* Agent SDK では、[`allowedTools` と `tools` オプション](/docs/ja/agent-sdk/todo-tracking#model-availability) は 2 つのフラグと同じように機能します

[バックグラウンドセッション](/docs/ja/agent-view) および[ウェブ上の Claude Code](/docs/ja/claude-code-on-the-web) では、Claude Code はリストされているかどうかに関わらず、すべてのモデルで同じツールを提供します。

Claude Code はサブエージェントにツールを提供するのは、セッションがそれらを持っている場合のみです。サブエージェントが異なるモデルを実行している場合でも同じです。プロセス内の[エージェントチーム](/docs/ja/agent-teams) メンバーはセッションと同じ方法で従いますが、独自の[分割ペイン](/docs/ja/agent-teams#choose-a-display-mode) にいるメンバーは別の Claude Code プロセスとして実行されるため、独自のモデルが決定します。Task ツールがない場合、エージェントは[共有タスクリスト](/docs/ja/agent-teams#assign-and-claim-tasks) の代わりにメッセージを通じてチームと調整します。

ここで説明されているデフォルトセットは Claude Code v2.1.268 以降に適用されます。

<h2 id="webfetch-tool-behavior">
  WebFetch ツールの動作
</h2>

WebFetch は URL とそこから抽出する内容を説明するプロンプトを受け取ります。ページをフェッチし、サーバーが HTML を返した場合はレスポンスを Markdown に変換し、小型で高速なモデルを使用してコンテンツに対してプロンプトを実行します。ほとんどのフェッチでは、Claude はそのモデルの回答を受け取り、生のページではありません。変換ステップは設定できません。

これにより WebFetch は設計上、情報損失が発生します。抽出プロンプトが Claude に到達する内容を決定するため、ページがあるものについて言及していないという結果は、単にプロンプトがそれについて尋ねなかったことを意味するだけかもしれません。Claude に別のより具体的なプロンプトでもう一度フェッチするよう依頼するか、Bash 経由で `curl` を使用して未処理のページを取得してください。

Claude が受け取るレスポンスを形作るいくつかの動作があります。

* WebFetch は `localhost` およびドットのない他のホスト名（ベアなイントラネット名など）をリクエストを行う前に拒否します。[返されるエラー](/docs/ja/errors#webfetch-cannot-fetch-localhost) は Claude に Bash 経由で `curl` を使用してローカルサーバーに到達するよう指示します。
* HTTP URL は自動的に HTTPS にアップグレードされます。
* 大きなページは処理前に固定文字数制限に切り詰められます。
* WebFetch はデフォルトで各レスポンスを 15 分間キャッシュするため、同じ URL の繰り返しフェッチは迅速に返されます。Claude Code v2.1.233 以降では、[`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/ja/env-vars#variables) を設定して、WebFetch が各レスポンスを保持する期間を変更できます。
* ページが 5 分以内（WebFetch が従うリダイレクトを含む）にダウンロードを完了しない場合、デッドラインエラーで失敗します。Claude Code v2.1.268 以降では、[`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/ja/env-vars#variables) を設定して制限を変更するか、`0` に設定して削除できます。
* URL が別のホストにリダイレクトされる場合、WebFetch はそれに従う代わりに、元の URL とリダイレクト先を名前で示すテキスト結果を返します。その後 Claude は 2 番目の WebFetch 呼び出しで新しい URL をフェッチします。
* 抽出ステップが過負荷の API にヒットした場合、Claude Code はバックオフで再試行します。それでも失敗するフェッチはエラー結果を返します。v2.1.212 より前では、API エラーテキストが抽出されたページコンテンツであるかのように Claude に到達する可能性がありました。

Manual および `acceptEdits` [権限モード](/docs/ja/permission-modes) では、WebFetch はフェッチ前にプロンプトを表示します。ただし、[権限ルール](/docs/ja/permissions#manage-permissions) で既に許可または拒否されているドメインと、プロンプトなしでフェッチされる事前承認済みドキュメンテーションドメインの組み込みセットは例外です。ルールが許可するものが何であれ、フェッチは最初に [WebFetch ドメイン安全性チェック](/docs/ja/data-usage#webfetch-domain-safety-check) に合格する必要があります。そのセクションではチェックが送信する内容とそれをスキップする設定について説明しています。プロンプトは 3 つのオプションを提供します。

* **Yes**: このフェッチのみを承認します。次の WebFetch 呼び出しは、同じドメインであってもプロンプトを再度表示します。
* **Yes, and don't ask again for `<domain>`**: フェッチを承認し、そのドメインの `WebFetch(domain:...)` 許可ルールをそのリポジトリの `.claude/settings.local.json` に保存します。[保存された承認の永続化方法](/docs/ja/permissions#permission-system) を参照してください。組織が [`allowManagedPermissionRulesOnly`](/docs/ja/permissions#managed-only-settings) を設定している場合、Claude Code はこのオプションを非表示にします。
* **No, and tell Claude what to do differently**: フェッチを拒否します。

プロンプトなしで事前にドメインを許可するには、`WebFetch(domain:example.com)` のような許可ルールを追加します。`WebFetch(domain:*)` はすべてのドメインを許可します。`auto` および `bypassPermissions` [権限モード](/docs/ja/permissions#permission-modes) はプロンプトをスキップします。ただし、明示的な `ask` ルールが一致するドメインは除きます。

`deny`、`ask`、または `allow` の明示的な `WebFetch(domain:...)` ルールは事前承認セットより優先されるため、事前承認されたドメインをブロックするか、そのドメインに対してプロンプトを要求できます。

WebFetch は `Claude-User` で始まる `User-Agent` ヘッダーと、コンテンツネゴシエーションをサポートするサーバーが Markdown を直接返すことができるように HTML より Markdown を優先する `Accept` ヘッダーを設定します。

サンドボックス化されたコマンドは WebFetch の事前承認されたドキュメンテーションドメインの組み込みセットを継承しません。サンドボックス化されたコマンドがプロンプトなしでドメインに到達できるようにするには、ドメインを [`allowedDomains`](/docs/ja/settings-reference#sandbox-network-alloweddomains) に追加するか、`WebFetch(domain:...)` ルールで許可します。[サンドボックスもこれを尊重します](/docs/ja/sandboxing#network-isolation)。WebFetch は代わりにサンドボックス許可リストを読み取ることはないため、ドメインをサンドボックスまたは組織ネットワーク許可リストに追加しても、WebFetch がそれについてプロンプトを表示するのを止めることはできません。

<h2 id="websearch-tool-behavior">
  WebSearch ツールの動作
</h2>

WebSearch は Anthropic の [ウェブ検索](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) バックエンドに対してクエリを実行し、結果のタイトルと URL を返します。結果ページを取得することはありません。Claude が検索結果で見つけたページを読むには、[WebFetch](#webfetch-tool-behavior) でフォローアップします。

このツールは呼び出しごとに最大 8 つのバックエンド検索を実行でき、結果を返す前に内部で検索を絞り込みます。Claude は `allowed_domains` で結果を特定のホストのみに含めるようにスコープできます。または `blocked_domains` で結果を除外できます。この 2 つのリストは 1 回の呼び出しで組み合わせることはできません。

検索リクエストが過負荷状態の API にヒットした場合、Claude Code はバックオフで再試行します。それでも失敗した呼び出しはエラー結果を返します。v2.1.212 より前では、API エラーテキストが検索結果のように Claude に到達する可能性がありました。

WebSearch 権限ルールは指定子を取りません。`allow` または `deny` の単なる `WebSearch` エントリが唯一の形式です。

検索バックエンドは設定不可です。別のプロバイダーで検索するには、検索ツールを公開する [MCP サーバー](/docs/ja/mcp) を追加します。

<Note>
  WebSearch は Claude API と [AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws) で利用可能です。Microsoft Foundry では [Anthropic でホストされているデプロイメント](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) が必要です。Azure でホストされているデプロイメントはサーバー側ツールをサポートしていないため、WebSearch 呼び出しは失敗します。Google Cloud の Agent Platform では Claude 4 以降のモデル（Opus、Sonnet、Haiku を含む）で動作します。Amazon Bedrock はサーバー側ウェブ検索ツールを公開していません。
</Note>

<h3 id="session-search-limit">
  セッション検索制限
</h3>

セッションは最大 200 回の WebSearch 呼び出しを実行できます。メイン会話とそれが生成するすべての [サブエージェント](/docs/ja/sub-agents) 全体でカウントされるため、並列リサーチ ファンアウトによって行われた検索は同じ制限にカウントされます。この制限には Claude Code v2.1.212 以降が必要です。Claude が制限に達すると、さらなる呼び出しは、再試行を促すエラーではなく、既に収集した情報で続行するよう Claude に指示する通知を返します。通知は表示されません。キャップされた呼び出しは会話に何もしなかった検索として表示され、Claude がさらに検索が必要な場合、通知は制限を引き上げるよう求めるよう指示します。

[`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/ja/env-vars) 環境変数を設定して上限を変更します。正の整数を受け入れるため、上限を引き上げることはできますが、オフにすることはできません。[`/clear`](/docs/ja/commands#all-commands) を実行するとカウントがリセットされます。ワークフローの実行など、[サブエージェント](/docs/ja/sub-agents) を生成できる作業がクリア後も存在する場合、カウントは代わりに引き継がれます。

<h2 id="write-tool-behavior">
  Write ツールの動作
</h2>

Write ツールは新しいファイルを作成するか、既存のファイルを提供された完全なコンテンツで上書きします。追記やマージは行いません。

Claude が現在の会話で既存ファイルを上書きする前に読む必要があるかどうかは、モデルとファイルによって異なります。

* Claude Opus 4.6、Claude Haiku 4.5、およびそれ以前のモデルは常に読み込みが必要なため、読み込まれていない既存ファイルへの Write は エラーで失敗します。
* より新しいモデルは、[read-before-edit](#edit-tool-behavior) と同じ条件下で、このセッション中に読み込んだことのないファイルを上書きできます。読み込みが権限プロンプトを必要とせず、Read ツールが利用可能な場合です。
* Jupyter ノートブック、および Claude が [`PARTIAL view` 通知](#read-tool-behavior) で部分的にのみ読み込んだファイルは、すべてのモデルで読み込みが必要です。

この制約は新しいファイルには適用されません。v2.1.228 より前は、すべてのモデルが既存ファイルを上書きする前に読み込みが必要でした。

Bash でファイルを表示することも、[Edit ツールの動作](#edit-tool-behavior) で説明されている同じルールの下でこの要件を満たします。

既存ファイルへの部分的な変更の場合、Claude は Write ではなく Edit を使用します。

<h2 id="check-which-tools-are-available">
  利用可能なツールを確認する
</h2>

正確なツール セットは、プロバイダー、プラットフォーム、および設定によって異なります。実行中のセッションで読み込まれているものを確認するには、Claude に直接尋ねます：

```text theme={null}
What tools do you have access to?
```

Claude は会話形式の概要を提供します。正確な MCP ツール名については、`/mcp` を実行します。

<Note>
  [advisor tool](/docs/ja/advisor) は、Claude Code が実装するツールではなく、API が実行する [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) です。権限ルールやフック マッチャーで参照できる名前はありません。
</Note>

<h2 id="see-also">
  関連項目
</h2>

* [MCP servers](/docs/ja/mcp)：外部サーバーを接続してカスタム ツールを追加する
* [権限](/docs/ja/permissions)：権限システム、ルール構文、ツール固有のパターン
* [Subagents](/docs/ja/sub-agents)：subagent のツール アクセスを構成する
* [フック](/docs/ja/hooks-guide)：ツール実行の前後にカスタム コマンドを実行する
