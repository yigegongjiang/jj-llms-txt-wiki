> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# セッションの管理

> Claude Code の会話に名前を付け、再開し、分岐し、切り替えます。`--continue`、`--resume`、`--from-pr`、`/resume` ピッカー、セッション命名、トランスクリプトのエクスポート、およびトランスクリプトの保存場所について説明します。

セッションはプロジェクトディレクトリに紐付けられた保存済みの会話です。Claude Code はローカルに保存されるため、中断したところから再開したり、別のアプローチを試すために分岐したり、タスク間を切り替えたりできます。

[デスクトップアプリ](/docs/ja/desktop#work-in-parallel-with-sessions)、[Web 上の Claude Code](/docs/ja/claude-code-on-the-web)、および [VS Code 拡張機能](/docs/ja/vs-code#resume-past-conversations)はそれぞれ独自のセッション履歴を保持しています。このページでは CLI について説明します。

<h2 id="resume-a-session">
  セッションを再開する
</h2>

セッションは作業中に [ローカルトランスクリプトファイル](#export-and-locate-session-data)に継続的に保存されるため、終了後または `/clear` を実行した後に再開できます。これらのエントリポイントを使用します。

| コマンド                                | 機能                                                                           |
| :---------------------------------- | :--------------------------------------------------------------------------- |
| `claude --continue`                 | 現在のディレクトリで最新のセッションを再開します                                                     |
| `claude --resume`                   | [セッションピッカー](#use-the-session-picker)を開きます                                    |
| `claude --resume <name>`            | 指定されたセッションを直接再開します                                                           |
| `claude --resume <transcript-path>` | そのパスの `.jsonl` [トランスクリプトファイル](#where-transcripts-are-stored)に保存されている会話を再開します |
| `claude --from-pr <number>`         | そのプルリクエストにリンクされたセッションでフィルタリングされたセッションピッカーを開きます                               |
| `/resume`                           | アクティブなセッション内から別の会話に切り替えます                                                    |

Claude Code は [`claude -p`](/docs/ja/headless)または [Agent SDK](/docs/ja/agent-sdk/overview)で作成されたセッションをセッションピッカーから除外し、`claude --continue` からも除外します。セッション ID を `claude --resume <session-id>` に渡すことで再開できます。`claude --continue` を使用する場合、Claude Code は [最初のプロンプトが `/loop` だったセッション](#where-the-session-picker-looks)もスキップします。[`claude -p --continue`](/docs/ja/headless#continue-conversations)を実行すると、Claude Code は `-p`、SDK、および `/loop` セッションを含めます。

`claude --continue` は完了した [バックグラウンドセッション](/docs/ja/agent-view)を開きますが、実行中のセッションは開きません。完了したバックグラウンドセッションを開くには Claude Code v2.1.257 以降が必要です。最新の会話が [バックグラウンドに移動した](/docs/ja/agent-view#send-the-session-to-the-background)セッションで、そこで実行中の場合、Claude Code は `Your most recent conversation is running in the background` と終了し、そのセッションの ID を表示します。[`claude agents`](/docs/ja/agent-view#attach-to-a-session)からセッションにアタッチするか、`claude --resume` を実行して別のセッションを選択します。

任意のディレクトリから `claude --resume <session-id>` を実行できます。Claude Code は現在のプロジェクトディレクトリとその git worktrees でまず ID を検索し、次にこのマシン上の他のすべてのプロジェクトで検索するため、他の場所で開始されたセッションや [`/cd`](/docs/ja/commands)で移動したセッションを見つけます。クロスプロジェクト検索は、正確に 1 つの他のプロジェクトがそれのメッセージを含むトランスクリプトを保持している場合にのみ ID を解決するため、手動でコピーされた重複は Claude Code が見つからないと報告し、任意のコピーを再開するのではなく、見つかりません。保存されたセッションが ID と一致しない場合、Claude Code は `No conversation found with session ID: <session-id>` と報告します。v2.1.223 より前は、ルックアップは現在のプロジェクトディレクトリとその git worktrees で停止したため、セッションが最後に機能していたディレクトリから再開する必要がありました。

<h3 id="what-a-resumed-session-restores">
  再開されたセッションが復元するもの
</h3>

再開されたセッションは、会話とそれに保存された状態を復元します。

* 会話履歴：ツール呼び出しと結果を含む完全な履歴。前のプロセスが終了したときに実行中だったツール（例えばクラッシュ）は、再開時に完了または再実行されません。Claude はその呼び出しが結果が記録される前に切断されたとマークされているのを見て、再度実行する前に有効になったかどうかを確認するよう指示されます。ただし、[`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/ja/env-vars#variables)が設定されている場合は除きます。v2.1.281 より前は、Claude Code は切断された呼び出しを会話から削除するか、中断したものとして Claude に表示していました。
* モデル：セッションは使用していたモデルで続行されます。モデルが廃止されたか `availableModels` で許可されていない場合、`--model` フラグまたは `ANTHROPIC_MODEL` ファミリー環境変数が起動時に 1 つを選択する場合、または [Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry](/docs/ja/third-party-integrations)などのプロバイダー固有のデプロイ ID を使用するプロバイダーの場合は復元されません。[モデル設定](/docs/ja/model-config#setting-your-model)の解決順序を参照してください。
* エージェント：[`--agent`](/docs/ja/sub-agents#invoke-subagents-explicitly)または `agent` 設定で開始されたセッションはそのエージェントとして続行され、ツール制限とモデルを保持します。再開時に `--agent` を渡して別のエージェントを選択します。どちらの場合のシステムプロンプトについては、[再開された会話のシステムプロンプトフラグ](/docs/ja/cli-reference#system-prompt-flags-in-resumed-conversations)を参照してください。Claude Code は 2 つの場所でエージェントを検索します。セッションの元のディレクトリ（[そのワークスペースを信頼している](/docs/ja/permissions#project-allow-rules-and-workspace-trust)場合）、次に再開するディレクトリ。プロジェクトスコープのエージェントは別のディレクトリから再開する場合でも読み込まれます。Claude Code がどちらの場所でもエージェントを見つけられない場合、セッションはデフォルトのツールで再開され、[エージェントに名前を付けた警告](/docs/ja/errors#session-agent-no-longer-available)が表示されます。
* 権限モード：`claude --continue`、`claude --resume <session-id>`、または `claude --resume <name>`（名前が 1 つのセッションと一致する場合）で `-p` なしでターミナルから再開する場合、Claude Code はセッションが存在していた権限モードを復元します。ただし、[再開時の権限モード](#permission-mode-on-resume)の場合は除きます。これはセッションピッカー、`/resume`、および `claude -p` で再開する場合もカバーします。`--permission-mode` または `--dangerously-skip-permissions` を渡して復元されたモードをオーバーライドします。
* アクティブなゴール：セッションが終了したときにまだアクティブだった [ゴール](/docs/ja/goal#resume-with-an-active-goal)は引き継がれます。ターン数、タイマー、およびトークン支出ベースラインはリセットされます。
* スケジュール済みタスク：[有効期限が切れていない](/docs/ja/scheduled-tasks#limitations)タスクが復元されます。バックグラウンド Bash およびモニタータスクは復元されません。

元の起動からのすべての設定フラグが復元されるわけではありません。セッションが `--mcp-config`、`--settings`、`--plugin-dir`、`--fallback-model`、または `--add-dir` で追加されたディレクトリに依存していた場合、再開時に再度渡します。セッション中に `/add-dir` で追加されたディレクトリは復元されませんが、セッションピッカーはセッションを見つけるためにそれらを使用します。`settings.json` や `settings.local.json` などの標準設定ファイルは起動時に再度読み込まれるため、それらに存在する設定を再度渡す必要はありません。`--system-prompt` および `--append-system-prompt` については、[再開された会話のシステムプロンプトフラグ](/docs/ja/cli-reference#system-prompt-flags-in-resumed-conversations)を参照してください。

<h4 id="permission-mode-on-resume">
  再開時の権限モード
</h4>

再開されたセッションが開始される権限モードは、再開方法によって異なります。

* ターミナル：`claude --continue`、`claude --resume <session-id>`、または `claude --resume <name>`（名前が 1 つのセッションと一致する場合）で `-p` なし。Claude Code はセッションが存在していた権限モードを復元します。ただし、表の場合は除きます。`--permission-mode` または `--dangerously-skip-permissions` を渡して復元されたモードをオーバーライドします。
* 非対話型：`claude -p --resume` または `claude -p --continue`。Claude Code は新しい `claude -p` 実行が開始される権限モードで実行を開始します。ただし、プランモードで終了したセッションは [以下の条件](#resume-in-plan-mode-with-p)下でプランモードで再開されます。
* VS Code：拡張機能の会話パネル。表は、プランモードで終了した会話のみをカバーします。その他については、[過去の会話を再開](/docs/ja/vs-code#resume-past-conversations)を参照してください。
* 起動時のセッションピッカー：[セッションピッカー](#use-the-session-picker)から選択したセッション。`claude --resume` だけで開いたか、`claude --from-pr` で開いたか、複数のセッションと一致する名前で開いたかに関わらず。Claude Code は保存された権限モードを復元しません。同じコマンドラインから新しいセッションを開始する権限モードでセッションを開始します。
* セッション内の `/resume`（引数の有無を問わず）：Claude Code は保存された権限モードを復元しません。切り替える会話は、現在のセッションが存在する権限モードで続行されます。

非対話型および VS Code パスでプランモードを復元するには Claude Code v2.1.246 以降が必要です。各行は、セッションが終了した権限モード、ターミナル、非対話型、および VS Code パスのどれで再開するか、および Claude Code が再開されたセッションを開始する権限モードを示します。

| セッションが終了した権限モード     | 再開方法                                       | 再開後の権限モード                                                                                                                                                                                                                                               |
| :------------------ | :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bypassPermissions` | ターミナル                                      | 新しいセッションが開始される権限モード。[権限をバイパス](/docs/ja/permission-modes#skip-all-checks-with-bypasspermissions-mode)するには、起動時に 1 つのフラグまたは [ユーザー、`--settings`、または管理設定](/docs/ja/settings-reference#permissions-defaultmode)の `permissions.defaultMode: "bypassPermissions"` で有効にします |
| `plan`              | ターミナル                                      | 新しいセッションが開始される権限モード                                                                                                                                                                                                                                     |
| `auto`              | ターミナル                                      | `auto`。[オートモード要件](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)をアカウントがまだ満たしている場合のみ                                                                                                                                                             |
| Manual              | ターミナル                                      | [組み込みデフォルト](/docs/ja/permission-modes#which-mode-a-session-starts-in)から新しいセッションがオートモードで開始される場合は Manual。設定ファイルの `defaultMode` が [有効になる](/docs/ja/permission-modes#which-mode-a-session-starts-in)場合、Claude Code は再開されたセッションをそのモードで開始します                            |
| `plan`              | 非対話型。[以下の条件](#resume-in-plan-mode-with-p)下 | プランモード                                                                                                                                                                                                                                                  |
| Any mode            | 非対話型。その他の場合                                | 新しい `claude -p` 実行が開始される権限モード                                                                                                                                                                                                                           |
| `plan`              | VS Code                                    | プランモード。[VS Code ページの例外](/docs/ja/vs-code#resume-past-conversations)付き                                                                                                                                                                                        |

<h5 id="resume-in-plan-mode-with-p">
  `-p` でプランモードで再開
</h5>

`claude -p --resume` または `claude -p --continue` 実行は、4 つの条件すべてが成立する場合にのみプランモードで再開されます。

* [`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags)を渡すため、Claude Code は承認のためにプランを提示できます
* `--permission-mode` または `--dangerously-skip-permissions` を渡さない
* `--fork-session` を渡さない
* 実行が [チャネル](/docs/ja/channels)を通じて開始されていない

<h3 id="resume-from-a-summary">
  サマリーから再開
</h3>

Pro または Max プランでは、約 1 時間以上非アクティブで 100,000 トークンを超えるセッションを再開すると、Claude Code は会話を復元してから、最初のメッセージを送信する前にダイアログを開きます。セッションの [プロンプトキャッシュ](/docs/ja/prompt-caching#cache-lifetime)はその時点で有効期限が切れているため、ダイアログのオプションのどれを選択しても、次のリクエストは完全な履歴を 1 回処理します。

ダイアログはセッションを続行する 3 つの方法を提供します。各方法は、会話の各部分が後のリクエストにどの程度引き継がれるかが異なり、これはすべての詳細を保持することと、リクエストごとに送信するトークンを減らすことのトレードオフです。

* **サマリーから再開**：[`/compact`](/docs/ja/context-window#what-survives-compaction)を直ちに実行します。Claude Code は完全な履歴に対して 1 つの要約リクエストを送信し、その後履歴をサマリー、最新の交換、および最近読んだファイルまで 5 つに置き換えます。後のリクエストは完全な履歴の代わりにサマリーを実行します。
* **セッション全体をそのまま再開**：会話を変更せずに読み込みます。最初のメッセージを送信した後、Claude Code は完全な履歴を再処理してキャッシュし、キャッシュが温かいままの間は後のリクエストでキャッシュから再度読み込みます。
* **今後は聞かないでください**：セッション全体を再開し、すべての将来の再開でダイアログの表示を停止します。

そのまま再開すると、会話のすべての詳細が利用可能に保たれ、会話のサイズに応じてスケーリングするリクエストごとのコストがあります。サマリーから再開すると、完全な履歴の代わりにサマリーを実行するため、後のリクエストごとのコストが低くなりますが、サマリーが除外したものは Claude のコンテキストに存在しなくなります。[長いセッションで使用量が増加する理由](/docs/ja/costs#why-usage-climbs-in-a-long-session)を参照して、そのリクエストごとのコストがどこから来るかを確認してください。

<h3 id="where-the-session-picker-looks">
  セッションピッカーが検索する場所
</h3>

Claude Code はセッションをプロジェクトディレクトリごとに保存します。デフォルトでは、セッションピッカーは以下を表示します。

* 現在の worktree からのセッション。[バックグラウンドセッション](/docs/ja/agent-view)を含み、リストで `bg` とマークされています
* 他の場所で開始され、`/add-dir` で現在のディレクトリを追加したセッション

`Ctrl+W` を使用してリポジトリのすべての worktree に拡張するか、`Ctrl+A` を使用してこのマシン上のすべてのプロジェクトに拡張します。

最初のプロンプトが [`/loop`](/docs/ja/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop)コマンドだったセッションはピッカーに表示されず、`claude --continue` もそれらをスキップします。会話の後半で `/loop` を実行してもセッションは非表示になりません。v2.1.211 より前は、会話の早い段階で `/loop` 実行を行うと、セッションはピッカーから永続的に非表示になりました。

[`/cd`](/docs/ja/commands)でセッションを移動すると、新しいディレクトリのプロジェクトストレージに再配置されるため、その後そのディレクトリのピッカーに表示されます。v2.1.196 以降、移動されたセッションはクラッシュまたは強制終了後も古いディレクトリのピッカーから除外されたままになります。以前のバージョンでは、古いパスにアンダースコアなどの特殊文字が含まれている場合、クリーンでない終了後に古いディレクトリのリストに再度表示される可能性がありました。

同じリポジトリの別の worktree からセッションを選択すると、Claude Code はそこで再開します。セッション自体の worktree が存在しなくなった場合、Claude Code は [現在のディレクトリで再開](/docs/ja/worktrees#resume-a-worktree-session)します。関連のないプロジェクトからセッションを選択すると、Claude Code は `cd` と再開コマンドをクリップボードにコピーします。そのプロジェクトのディレクトリが存在しなくなった場合、Claude Code は失敗する `cd` コマンドをコピーするのではなく、現在のディレクトリでセッションを再開します。

名前で再開する場合は、現在のリポジトリとその worktree 全体で解決されます。どちらの形式も完全一致を探し、別の worktree に存在する場合でも直接再開します。

| コマンド                     | 完全一致    | あいまいな名前                                         |
| :----------------------- | :------ | :---------------------------------------------- |
| `claude --resume <name>` | 直接再開します | セッションピッカーを開き、名前を検索用語として事前入力します                  |
| `/resume <name>`         | 直接再開します | エラーを報告します。セッションピッカーを開くには、引数なしで `/resume` を実行します |

<h2 id="name-your-sessions">
  セッションに名前を付ける
</h2>

セッションに説明的な名前を付けて、セッションピッカーで見つけやすく、名前で再開できるようにします。これは複数のタスクを並行して処理している場合に最も重要です。

| 時期                         | 名前を設定する方法                                                                                                                                   |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| 起動時                        | `claude -n auth-refactor`                                                                                                                   |
| セッション中                     | `/rename auth-refactor`。名前はプロンプトバーにも表示されます                                                                                                  |
| セッションピッカーから                | セッションをハイライトして `Ctrl+R` を押します                                                                                                                |
| プラン受け入れ時                   | [Plan Mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)でプランを受け入れると、既に設定していない限り、プランに基づいて生成されたタイトルがセッションに付けられます              |
| claude.ai または Claude アプリから | [Remote Control セッション](/docs/ja/remote-control#connect-from-another-device)の名前を変更します。Claude Code は CLI でも同じ名前を適用します。Claude Code v2.1.221 以降が必要です |
| デスクトップアプリから                | [デスクトップアプリ](/docs/ja/desktop#work-in-parallel-with-sessions)でセッションの名前を変更します                                                                      |

CLI ルートまたは claude.ai からセッションに名前を付けたら、`claude --resume <name>` または `/resume <name>` で再開できます。デスクトップアプリセッションはアプリで再開され、独自のセッション履歴が保持されます。worktree 全体での名前解決の動作については、[セッションを再開する](#resume-a-session)を参照してください。

このマシン上で既に実行中の別のセッションが既に使用している名前でインタラクティブセッションを開始または再開した場合、またはセッションをそのような名前に名前変更した場合、Claude Code は既に持っているセッションに名前を残し、`auth-refactor-graceful-unicorn` のような 2 語のサフィックスを持つバリアントに名前を変更し、その旨を通知します。自分で選びたい場合は、新しい名前で `/rename` を実行してください。v2.1.232 より前は、両方のセッションが名前を保持していました。

Claude Code が重複の名前を変更しない場合が 3 つあり、リストに同じ名前の 2 つのセッションが表示される場合があります。

* AI が生成したタイトルまたはデフォルト表示名をチェックしません。
* 起動時に[バックグラウンド](/docs/ja/agent-view#from-your-shell)または `-p` セッションの `--name` をチェックしません。
* 以前のバージョンの Claude Code でセッションの名前を変更することはできません。

名前を付けないセッションでも、Claude Code が割り当てる 2 つのラベルが付けられます。生成されたタイトルのみが再開ハンドルとして機能します。

* デフォルト表示名：名前を付けないインタラクティブセッションでも、起動時にデフォルト表示名が自動的に付けられます。Claude Code v2.1.196 以降が必要です。デフォルト名は、作業ディレクトリの名前と 2 文字のサフィックスを組み合わせたもので、例えば `my-app-3f` のようになり、[agent view](/docs/ja/agent-view)や `claude agents --json` 出力などの実行中セッションのリストでセッションを識別します。デフォルト名は再開ハンドルではありません。`claude --resume` または `/resume` に渡した場合、Claude Code はセッションを見つけません。セッションに名前を付けるとデフォルト名がリストに置き換わり、プランを受け入れても置き換わります。
* 生成されたタイトル：セッションに名前を付けない場合、Claude Code はセッションのタイトルを生成します。タイトルは最初のプロンプトの短い要約で、通常は Haiku クラスモデルである小型/高速モデルへのバックグラウンドリクエストで作成されます。シェルまたはスクリプトから直接開始する `claude -p` 実行は生成されません。

  プランを受け入れるとプランに基づいたタイトルに置き換わります。セッションに名前を付けると生成されたタイトルが置き換わります。

  最初のプロンプトタイトルは[セッションピッカー](#use-the-session-picker)と、名前が設定されていない場合のステータスライン [`session_name`](/docs/ja/statusline) フィールドに表示されます。プランタイトルは同じ 2 つの場所に表示され、実行中セッションのリストにも表示され、デフォルト表示名の代わりになります。

  `claude --resume` または `/resume` にいずれかのタイトルを渡すことができ、Claude Code は設定した名前と同じ方法で解決します。

<h2 id="use-the-session-picker">
  セッションピッカーを使用する
</h2>

セッション内で `/resume` を実行するか、引数なしで `claude --resume` を実行して、インタラクティブセッションピッカーを開きます。これらのキーボードショートカットを使用して、ナビゲート、検索、リストを拡張します。

| ショートカット                 | アクション                                                                                                                |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`               | セッション間をナビゲートします                                                                                                      |
| `→` / `←`               | グループ化されたセッションを展開または折りたたみます                                                                                           |
| `Enter`                 | ハイライトされたセッションを再開します                                                                                                  |
| `Space`                 | セッションコンテンツをプレビューします。ターミナルが貼り付けとしてキャプチャしない場合は `Ctrl+V` も機能します                                                         |
| `Ctrl+R`                | ハイライトされたセッションの名前を変更します                                                                                               |
| `/` またはスペース以外の任意の印字可能文字 | 検索モードに入り、セッションをフィルタリングします。GitHub、GitHub Enterprise、GitLab、または Bitbucket のプルまたはマージリクエスト URL を貼り付けて、それを作成したセッションを見つけます |
| `Ctrl+A`                | このマシン上のすべてのプロジェクトからセッションを表示します。もう一度押すと現在のリポジトリに戻ります                                                                  |
| `Ctrl+W`                | 現在のリポジトリのすべての worktree からセッションを表示します。もう一度押すと現在の worktree に戻ります。マルチ worktree リポジトリでのみ表示されます                           |
| `Ctrl+B`                | 現在の git ブランチからのセッションにフィルタリングします。もう一度押すとすべてのブランチを表示します                                                                |
| `Esc`                   | セッションピッカーまたは検索モードを終了します                                                                                              |

各行は、セッション名が設定されている場合はそれを表示し、そうでない場合は AI が生成したセッションタイトル、会話の概要、または最初のプロンプト、最後のアクティビティからの経過時間、git ブランチ、およびファイルサイズを表示します。`Ctrl+A` ですべてのプロジェクトに拡張して、各セッションのプロジェクトパスも表示します。

`/branch` または `--fork-session` で作成されたセッションは独自のセッション ID を取得し、別の行として表示されます。ピッカーが同じセッションの複数のエントリを見つけた場合、それらは単一の行の下にグループ化されます。グループを展開するには `→` を押します。

Claude Code が `claude --resume` ピッカーから選択したセッションを読み込めない場合、[`Failed to resume the conversation`](/docs/ja/errors#failed-to-resume-the-conversation) を出力し、再試行するコマンドを表示してから、終了コード 1 で終了します。セッション内の `/resume` ピッカーから、Claude Code は失敗を報告し、現在の会話は実行を続けます。

<h2 id="branch-a-session">
  セッションを分岐させる
</h2>

分岐は、これまでの会話のコピーを作成し、それに切り替え、元のセッションはそのままにしておきます。別のアプローチを試す際に、進めていたパスを失わないようにするために使用します。

セッション内から、オプションの名前を付けて `/branch` を実行します。

```text theme={null}
/branch try-streaming-approach
```

名前を省略した場合、Claude Code は会話の最初のプロンプトに基づいて新しいブランチに名前を付けます。v2.1.198 以降では、これは[コンパクション](/docs/ja/how-claude-code-works#when-context-fills-up)後にも適用されます。それより前のバージョンでは、元の最初のプロンプトを超えてコンパクション要約を参照する代わりに、リテラル名 `Branched conversation` にフォールバックしていました。

コマンドラインから、`--continue` または `--resume` を `--fork-session` と組み合わせます。

```bash theme={null}
claude --continue --fork-session
```

`/branch` 確認は 2 つのセッション ID を出力します。現在いる新しいブランチと元のセッションです。元のセッションは変更されず、セッションピッカーで利用可能なままです。`/resume <original-name>` で元のセッションに戻るか、その ID を `/resume` に渡します。

`/branch` はトランスクリプトをコピーし、実行中の Claude Code プロセスをそれに書き込むように切り替えます。この区別により、ブランチが継承するものが決まります。

| 状態                                                                                                                                                   | `/branch` 後                                                                                                  |
| :--------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| 会話履歴                                                                                                                                                 | `/branch` を実行した時点までブランチにコピーされます                                                                              |
| 「このセッションで許可」権限付与                                                                                                                                     | 引き継がれます。ブランチは同じプロセスで実行されるため、既存の付与はそのまま適用されます。`--fork-session` で別のプロセスにフォークした場合、新しいプロセスはそれらなしで開始され、そこで再度承認します |
| 実行中の[バックグラウンドサブエージェント](/docs/ja/sub-agents#run-subagents-in-foreground-or-background)と[バックグラウンド Bash コマンド](/docs/ja/interactive-mode#background-bash-commands) | 実行を続けます。それらの出力は、元のセッションではなく、切り替えた新しいブランチに表示されます                                                              |
| [Remote Control](/docs/ja/remote-control) 接続                                                                                                              | 接続されたままです。セッションに接続されている電話またはブラウザはブランチに従い、そこで新しいメッセージを受け取り続けます                                                |

フォークせずに 2 つのターミナルで同じセッションを再開すると、両方からのメッセージが 1 つのトランスクリプトにインターリーブされます。単一セッション内のチェックポイントベースの巻き戻しについては、[チェックポイント](/docs/ja/checkpointing)を参照してください。

<h2 id="manage-context-within-a-session">
  セッション内でコンテキストを管理する
</h2>

これらのコマンドは、セッションを離れることなくコンテキストウィンドウ内の内容を制御します。

* **`/clear`**：空のコンテキストで新たに開始します。Claude Code は以前の会話を保存します。`/resume` で再開するか、同じ Claude Code プロセス内では、[rewind メニューの前のセッションエントリ](/docs/ja/checkpointing#rewind-past-a-cleared-conversation)から再開できます。引数がない場合、新しい会話は `--name` または `/rename` で設定した名前を保持しますが、AI が生成したセッションタイトルは保持しません。代わりに、離れる会話に名前を付けるには、`/clear release-prep` のように名前を渡します。その後、新しい会話は名前なしで開始します
* **`/compact [instructions]`**：履歴を概要に置き換え、オプションで指定した内容に焦点を当てます
* **`/context`**：現在コンテキストを消費しているものを表示します

圧縮が CLAUDE.md、skills、およびルールとどのように相互作用するかについては、[コンテキストウィンドウガイド](/docs/ja/context-window)を参照してください。クリアと圧縮のどちらを使用するかについての戦略については、[ベストプラクティス](/docs/ja/best-practices#manage-your-session)を参照してください。

<h2 id="export-and-locate-session-data">
  セッションデータをエクスポートして見つける
</h2>

`/export` を実行して、現在の会話をクリップボードにコピーするか、プレーンテキストファイルとして保存するメニューを開きます。メッセージとツール出力は読みやすいテキストとしてレンダリングされます。ファイル名を渡して、メニューをスキップしてそのファイルに直接書き込みます。

<h3 id="access-conversations-from-scripts">
  スクリプトから会話にアクセスする
</h3>

`/export` は人が読むためのレンダリングされたトランスクリプトを生成します。以下のインターフェースはスクリプトが解析するための構造化データを生成します。実行からの JSON 結果、セッションのトランスクリプトファイルへのパス、またはイベントのライブストリームです。スクリプトをトリガーするものによって選択してください。

* **Claude を 1 回実行して結果をキャプチャする**: [`--output-format json` または `stream-json`](/docs/ja/headless#get-structured-output) で `claude -p` を呼び出して、非インタラクティブ実行の結果、セッション ID、使用状況、およびコストを構造化 JSON としてキャプチャします。
* **既存のセッションに質問する**: [`claude -p --resume`](/docs/ja/headless#continue-conversations) にセッション ID を渡して、フォローアップ プロンプト（要約リクエストなど）を送信し、構造化された応答をキャプチャします。
* **セッションイベントに反応する**: [hooks](/docs/ja/hooks#common-input-fields) と [status line commands](/docs/ja/statusline#available-data) が入力として受け取る `transcript_path` フィールドを読みます。`SessionEnd` hook はセッションが終了したときにトランスクリプトをアーカイブできます。
* **TypeScript または Python アプリに Claude を埋め込む**: [Agent SDK](/docs/ja/agent-sdk/overview) を使用して、各メッセージをプログラムで受け取ります。

以下の例は 2 番目のインターフェースを使用しています。既存のセッションにフォローアップ プロンプトを送信し、`jq` で答えを読みます。

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  トランスクリプトが保存される場所
</h3>

デフォルトでは、Claude Code はトランスクリプトを `~/.claude/projects/<project>/<session-id>.jsonl` に JSONL として保存します。ここで `<project>` は作業ディレクトリパスで、英数字以外の文字が `-` に置き換えられています。作業ディレクトリの変換後の名前が 200 文字を超える場合、Claude Code は名前を 200 文字に切り詰め、フルパスのハッシュを追加するため、ディレクトリ名はファイルシステムの制限内に留まります。

各行はメッセージ、ツール使用、またはメタデータエントリの JSON オブジェクトです。エントリ形式は Claude Code の内部形式であり、バージョン間で変更されるため、これらのファイルを直接解析するスクリプトはリリースごとに破損する可能性があります。セッションデータを構築するには、代わりに `/export` または [スクリプトインターフェース](#access-conversations-from-scripts) を使用してください。

場所、保持期間、および書き込み動作は設定可能です。

| 目的                                                                                           | 設定                                                                                          | 場所                           |
| -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ---------------------------- |
| `~/.claude` からストレージを移動する                                                                     | [`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars)                                                         | 環境変数                         |
| [`<project>` ディレクトリに自分で名前を付ける](#name-the-project-directory-yourself)                         | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ja/env-vars)                                              | 環境変数                         |
| 30 日間の保持期間を変更する                                                                              | [`cleanupPeriodDays`](/docs/ja/settings-reference#cleanupperioddays)                             | `settings.json`              |
| [Claude Desktop と Cowork トランスクリプト](/docs/ja/claude-directory#cleaned-up-automatically) の年齢制限を設定する | [`desktopSessionCleanupPeriodDays`](/docs/ja/settings-reference#desktopsessioncleanupperioddays) | ユーザー設定、管理設定、または `--settings` |
| すべてのモードでトランスクリプト書き込みを抑制する                                                                    | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ja/env-vars)                                           | 環境変数                         |
| 1 つの非インタラクティブ実行の書き込みを抑制する                                                                    | [`--no-session-persistence`](/docs/ja/cli-reference)                                             | `claude -p` を使用した CLI フラグ    |

<h3 id="delete-session-data">
  セッションデータを削除する
</h3>

トランスクリプトは [保持スイープルール](/docs/ja/claude-directory#cleaned-up-automatically) に基づいて期限切れになります。プロジェクトのトランスクリプトと関連する状態をより早く削除するには、[`claude project purge`](/docs/ja/claude-directory#clear-local-data) を実行してください。[`claude rm <id>`](/docs/ja/agent-view#what-deleting-a-session-removes) で [バックグラウンドセッション](/docs/ja/agent-view) を削除した場合、そのトランスクリプトはディスク上に残り、`claude --resume` を通じて利用可能なままです。

<h3 id="name-the-project-directory-yourself">
  プロジェクトディレクトリに自分で名前を付ける
</h3>

デフォルトでは、Claude Code は `<project>` 名をワーキングディレクトリパス全体から導出します。自分で名前を選択するには、`CLAUDE_CONFIG_DIR` と一緒に `CLAUDE_CODE_PROJECT_DIR_NAME` を設定してください。Claude Code はそのセッションのトランスクリプトと [自動メモリ](/docs/ja/memory#auto-memory) をあなたの名前の下に保存します。これは Claude Code を埋め込み、各セッションに独自の設定ディレクトリを与えるホストに適しています。Claude Code v2.1.234 以降が必要です。

たとえば、このローンチはテナント A のデータを `/srv/tenant-a` の下に保持し、そのプロジェクトディレクトリに `work` という名前を付けます。

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code はセッションのトランスクリプトを `/srv/tenant-a/projects/work/` に、その自動メモリを `/srv/tenant-a/projects/work/memory/` に書き込みます。ワーキングディレクトリがどこであるかは関係ありません。

これを設定する場合、3 つのルールが適用されます。

* **`CLAUDE_CONFIG_DIR` も設定してください**: 名前はワーキングディレクトリによって変わらないため、デフォルトの `~/.claude` の下では、すべてのプロジェクトのトランスクリプトと自動メモリが 1 つのディレクトリにマージされます。Claude Code は `CLAUDE_CONFIG_DIR` が設定されていない場合、`CLAUDE_CODE_PROJECT_DIR_NAME` を無視します。
* **1 ～ 64 文字の英字、数字、ハイフン、またはアンダースコアを使用してください**: `con` などの Windows デバイス名を使用しないでください。Claude Code は他の値を無視し、導出された名前を使用します。
* **`claude` を開始するシェル環境で設定してください**: Claude Code はスタートアップ時にその環境から一度だけ読み込むため、設定ファイルの `env` ブロックはそれを設定できません。

設定ディレクトリのプロジェクトディレクトリに名前を付けたら、その名前でローンチし続けてください。同じ `CLAUDE_CONFIG_DIR` で Claude Code を開始しても `CLAUDE_CODE_PROJECT_DIR_NAME` がない場合、導出されたディレクトリを再度読み書きします。あなたの名前の下に保存されたセッションはディスク上に残ります。[セッションピッカー](#use-the-session-picker) で `Ctrl+A` を押して、その設定ディレクトリの下のすべてのプロジェクトディレクトリ（ピン留めされたものを含む）からセッションをリストアップし、どのようにローンチしても、[`claude --resume <session-id>`](#resume-a-session) はどちらの名前の下に保存されたセッションでも見つけます。

<h2 id="see-also">
  関連項目
</h2>

これらのページは関連するセッションと並列処理のメカニクスについて説明しています。

* [Worktrees](/docs/ja/worktrees)：別のブランチで分離された並列セッションを実行します
* [Checkpointing](/docs/ja/checkpointing)：コードと会話を以前のポイントに巻き戻します
* [Context window](/docs/ja/context-window)：コンテキストを満たすもの、圧縮後に残るもの
* [Non-interactive mode](/docs/ja/headless)：`claude -p` の下でのセッション動作
