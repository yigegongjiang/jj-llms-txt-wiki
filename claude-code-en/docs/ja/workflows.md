> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 動的ワークフローで大規模にサブエージェントをオーケストレーションする

> 動的ワークフローは、Claude が作成したスクリプトから多くのサブエージェントをオーケストレーションし、再実行できます。コードベース監査、大規模マイグレーション、相互検証研究に使用します。

<Note>
  動的ワークフローはすべての有料プランで利用可能で、Anthropic API アクセス、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry で利用できます。Pro では、`/config` の Dynamic workflows 行からオンにしてください。
</Note>

動的ワークフローは、[サブエージェント](/docs/ja/sub-agents)を大規模にオーケストレーションする JavaScript スクリプトです。Claude は説明したタスク用のスクリプトを作成し、ランタイムはバックグラウンドで実行しながら、セッションは応答性を保ちます。

1 つの会話が調整できるより多くのエージェントが必要なタスク、またはオーケストレーションを読み直して再実行できるスクリプトとしてコード化したい場合にワークフローを使用します。例としては、コードベース全体のバグスイープ、500 ファイルのマイグレーション、複数のソースに対して相互検証が必要な研究質問、1 つにコミットする前に複数の独立した角度から下書きする価値のある難しい計画があります。

<h2 id="when-to-use-a-workflow">
  ワークフローを使用するタイミング
</h2>

[サブエージェント](/docs/ja/sub-agents)、[スキル](/docs/ja/skills)、[エージェントチーム](/docs/ja/agent-teams)、およびワークフローはすべてマルチステップタスクを実行できます。違いは、計画を保持する者です。

|                  | サブエージェント            | スキル                 | エージェントチーム             | ワークフロー             |
| :--------------- | :------------------ | :------------------ | :-------------------- | :----------------- |
| それは何か            | Claude が生成するワーカー    | Claude が従う指示        | ピアセッションを監督するリードエージェント | ランタイムが実行するスクリプト    |
| 次に何が実行されるかを決定する者 | Claude、ターンごと        | Claude、プロンプトに従う     | リードエージェント、ターンごと       | スクリプト              |
| 中間結果が存在する場所      | Claude のコンテキストウィンドウ | Claude のコンテキストウィンドウ | 共有タスクリスト              | スクリプト変数            |
| 繰り返し可能なもの        | ワーカー定義              | 指示                  | チーム定義                 | オーケストレーション自体       |
| スケール             | ターンごとに委任されたいくつかのタスク | サブエージェントと同じ         | 長時間実行される少数のピア         | 実行ごとに数十から数百のエージェント |
| 中断               | ターンを再開始             | ターンを再開始             | チームメイトは実行を続ける         | 同じセッション内で再開可能      |

ワークフローは計画をコードに移動します。サブエージェント、スキル、およびエージェントチームでは、Claude がオーケストレーターです。ターンごとに次に何を生成または割り当てるかを決定し、すべての結果は Claude のコンテキストウィンドウに入ります。ワークフロースクリプトはループ、分岐、および中間結果自体を保持するため、Claude のコンテキストは最終的な答えのみを保持します。

計画をコードに移動することで、ワークフローは単に複数のエージェントを実行するだけでなく、繰り返し可能な品質パターンを適用することもできます。独立したエージェントが相互に対立的にレビューしてから報告されるようにすることも、複数の角度から計画を下書きして相互に比較することもできるため、単一パスより信頼性の高い結果が得られます。

<h2 id="run-a-bundled-workflow">
  バンドルされたワークフローを実行する
</h2>

ワークフローの動作を最も簡単に確認する方法は、Claude Code に含まれている[組み込みワークフロー](#bundled-workflows)である `/deep-research` を実行することです。このワークフローは、複数のソースにわたって質問を調査するためのものです。セッションがバックグラウンドで一連のフェーズを処理している間、セッションは自由に使用でき、ターンバイターンのトランスクリプトではなく、最後に 1 つのレポートが得られます。

<Steps>
  <Step title="ワークフローを実行する">
    調査したい質問を使用して `/deep-research` を実行します。複数の角度から Web 検索を展開し、見つけたソースを取得してクロスチェックし、引用されたレポートを合成します。

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="ワークフローを許可する">
    Claude Code はワークフローを許可するかどうかを尋ねます。**Yes** を選択して続行します。正確なプロンプトは権限モードによって異なります。[実行前にプランを承認する](#approve-the-plan-before-it-runs)を参照して、モード別のオプションを確認してください。
  </Step>

  <Step title="進捗を監視する">
    実行がバックグラウンドで開始されます。`/workflows` を実行し、矢印キーを使用して実行を選択し、Enter キーを押して進捗ビューを開きます。

    ```text wrap theme={null}
    /workflows
    ```

    ビューには、各フェーズがエージェント数、トークン合計、経過時間とともに表示されます。任意のフェーズをドリルダウンして、そのエージェントと各エージェントが見つけたものを確認します。[実行を監視する](#watch-the-run)を参照して、コントロールの完全なセットを確認してください。

    入力ボックスの下のタスクパネルからも監視できます。実行中は、1 行の進捗サマリーがそこに表示されます。下矢印を押してフォーカスし、Enter キーを押して展開します。
  </Step>

  <Step title="レポートを読む">
    実行が完了すると、レポートがセッションに表示されます。各クレームの出所を引用し、クロスチェックで生き残らなかったクレームは既にフィルタリングされています。

    検証エージェントがレート制限や API エラーの後など、クレームをチェックできない場合、レポートはそのクレームを未検証として列挙し、反論されたものとしてカウントしません。
  </Step>
</Steps>

独自のタスク用にワークフローを実行するには、[Claude にワークフローを作成させ](#have-claude-write-a-workflow)、実行が目的を達成したら、それを[保存](#save-the-workflow-for-reuse)して独自のコマンドとして使用できます。

<h3 id="bundled-workflows">
  バンドルされたワークフロー
</h3>

Claude Code には、組み込みワークフローとして `/deep-research` が含まれています。

| コマンド                        | 機能                                                                                                                                                                                |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | 複数の角度から質問に対する Web 検索を展開し、見つけたソースを取得してクロスチェックし、各クレームに投票し、クロスチェックで生き残らなかったクレームがフィルタリングされた引用されたレポートを返します。[WebSearch ツール](/docs/ja/tools-reference#websearch-tool-behavior)が利用可能である必要があります |

`/deep-research` は、呼び出すときのみ実行されます。

[自分で保存](#save-the-workflow-for-reuse)したワークフローは同じ方法でコマンドになり、バンドルされたものと一緒に `/` オートコンプリートに表示されます。

<h3 id="watch-the-run">
  実行を監視する
</h3>

ワークフローはバックグラウンドで実行されるため、エージェントが作業している間、セッションは応答性を保ちます。任意の時点で `/workflows` を実行して、実行中および完了したワークフローをリストアップし、1 つを選択してその進捗ビューを開きます。

進捗ビューには、各フェーズがエージェント数、トークン合計、経過時間とともに表示されます。フッターには各アクションのキーが表示されます。

| キー              | アクション                                                                                      |
| :-------------- | :----------------------------------------------------------------------------------------- |
| `↑` / `↓`       | フェーズまたはエージェントを選択します                                                                        |
| `Enter` または `→` | 選択したフェーズをドリルダウンし、次にエージェントの詳細をドリルダウンします。詳細では、`Enter` で展開または折りたたみます                          |
| `Esc` または `←`   | 1 レベル戻ります。v2.1.203 から v2.1.205 では、`←` はフェーズまたはエージェントから戻りませんでした。これらのバージョンでは `Esc` を使用してください |
| `j` / `k`       | エージェント詳細がオーバーフローしたときにスクロールします                                                              |
| `f`             | 選択したフェーズのエージェントリストをステータスでフィルタリングします。もう一度押すとサイクルします                                         |
| `p`             | 実行を一時停止または再開します                                                                            |
| `x`             | 選択したエージェントを停止するか、フォーカスが実行にある場合はワークフロー全体を停止します                                              |
| `r`             | 選択した実行中のエージェントを再起動します                                                                      |
| `s`             | 実行のスクリプトを[保存](#save-the-workflow-for-reuse)してコマンドにします                                      |

エージェント詳細には、エージェントのプロンプト、最近のツール呼び出し、および結果が表示されます。各呼び出しは、実行中またはエラーなどの状態を示します。エージェントが独自のタスクリストを保持している場合、詳細にはそれも表示され、各タスクのステータスが表示されます。

`Enter` を押して詳細を展開します。プロンプトと結果は完全に表示され、リストされた各呼び出しはその入力と結果の開始を表示します。

<h2 id="have-claude-write-a-workflow">
  Claude にワークフローを書かせる
</h2>

Claude にタスク用のワークフローを書かせるには、2 つの方法があります。

* [プロンプトでワークフローをリクエストする](#ask-for-a-workflow-in-your-prompt)。自分の言葉で、またはキーワード `ultracode` を含めることで、Claude がそのタスク用のワークフローを書きます。
* [ultracode で Claude に決めさせる](#let-claude-decide-with-ultracode)。`/effort ultracode` を設定すると、Claude がセッション内のすべての実質的なタスク用にワークフローを計画します。

既に存在するワークフローコマンドを実行することもできます。`/deep-research` のような[バンドルされたワークフロー](#bundled-workflows)、または[保存した](#save-the-workflow-for-reuse)ワークフローです。

<h3 id="ask-for-a-workflow-in-your-prompt">
  プロンプトでワークフローをリクエストする
</h3>

セッションの努力レベルを変更せずに単一のタスクをワークフローとして実行するには、プロンプトにキーワード `ultracode` を含めます。「ワークフローを使用する」または「ワークフローを実行する」など、自分の言葉で尋ねることも機能します。Claude は直接的なリクエストを同じオプトインとして扱います。

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code はあなたの入力でキーワードをハイライトし、Claude はターンバイターンで作業するのではなく、タスク用のワークフロースクリプトを書きます。キーワードは Claude が作業をどのように構成するかのみを選択します。エージェントのツール呼び出しは、セッション内の他のツール呼び出しと同じ権限チェックと[サンドボックス化](/docs/ja/sandboxing)を受けます。

実行が望んだことを実行した場合、その後[コマンドとして保存](#save-the-workflow-for-reuse)できます。別の方法で構築されたオーケストレーターが既にある場合（サブエージェントプロンプトのフォルダーやスキルなど）、Claude にそれを指し示し、同じことを行うワークフローをリクエストできます。

<h4 id="dismiss-or-turn-off-the-keyword">
  キーワードを無視するか、オフにする
</h4>

ワークフローを開始するつもりがなかった場合、macOS では `Option+W`、Windows と Linux では `Alt+W` を押してこのプロンプトのハイライトを無視するか、ハイライトされたキーワードの直後にカーソルがある状態でバックスペースを押します。キーワードがまったくトリガーされないようにするには、`/config` で Ultracode キーワードトリガーをオフにします。

<h4 id="where-the-keyword-works">
  キーワードが機能する場所
</h4>

キーワードは、自分で入力したプロンプトでのみオプトインです。対話型プロンプト、IDE 拡張機能パネル、[Remote Control](/docs/ja/remote-control) クライアント、またはキーボード入力の [`origin`](/docs/ja/agent-sdk/typescript#sdkmessageorigin) を `{ kind: "human" }` としてスタンプする Agent SDK アプリケーションです。セッションに別の方法で到達した場合、ワークフローを開始しません。

* `-p` で渡されたプロンプト
* Agent SDK アプリケーションが人間の入力としてスタンプせずに送信するプロンプト
* スケジュールされたタスクプロンプト
* ウェブフック ペイロードまたはプルリクエストコメントが会話にリレーされた場合

<Note>
  v2.1.210 より前は、キーワードはこれらのルートのいずれからでもワークフローを開始しました。ウェブフック ペイロードまたはプルリクエストコメントが会話にリレーされた場合も含みます。
</Note>

<h3 id="let-claude-decide-with-ultracode">
  ultracode で Claude に決めさせる
</h3>

Ultracode は Claude Code の設定で、`xhigh` [推論努力](/docs/ja/model-config#adjust-effort-level)と自動ワークフローオーケストレーションを組み合わせます。オンにすると、Claude はあなたが尋ねるのを待つのではなく、各実質的なタスク用にワークフローを計画します。

```text wrap theme={null}
/effort ultracode
```

ultracode が既にオンの状態でセッションを開始するには、`claude --effort ultracode` で起動します。Claude Code v2.1.203 以降が必要です。

モデルを選択しながらオンにするには、`/model` ピッカーの努力スライダーを矢印キーで `ultracode` に移動します。[努力レベルを調整する](/docs/ja/model-config#adjust-effort-level)は ultracode をオンにするルートをリストします。

ultracode がオンの場合、Claude はタスクがワークフローを必要とするかどうかを決定します。単一のリクエストは複数のワークフローに変わる可能性があります。コードを理解するためのワークフロー、変更を加えるためのワークフロー、それを検証するためのワークフローです。これはセッション内のすべてのタスクに適用されるため、各リクエストはより多くのトークンを使用し、低い努力レベルよりも長くかかります。

`/effort ultracode` は現在のセッション用です。すべてのセッションをそれで開始するには、[`ultracode`](/docs/ja/settings-reference#ultracode) 設定を設定します。日常的な作業に戻るときは `/effort high` で戻ります。`/effort` メニューは [ultracode が利用可能な場合](/docs/ja/model-config#when-ultracode-is-available)のみそれを提供します。

<h3 id="approve-the-plan-before-it-runs">
  実行前にプランを承認する
</h3>

CLI では、実行ごとのプロンプトは計画されたフェーズとこれらのオプションを表示します。

* **Yes, run it**: 実行を開始する
* **Yes, and don't ask again for `<name>` in `<path>`**: 開始し、このプロジェクトでこのワークフローに対してこのプロンプトをスキップします。Claude Code は、現在のタスク用に Claude が書いたスクリプトではなく、バンドルされた、保存された、またはプラグインワークフローを名前で実行する場合にこのオプションを提供します。
* **View raw script**: 決定する前にスクリプトを読む
* **No**: キャンセル

`Ctrl+G` はスクリプトをエディターで開きます。`Tab` を使用すると、実行が開始される前にプロンプトを調整できます。

このプロンプトが表示されるかどうかは、[権限モード](/docs/ja/permission-modes)によって異なります。

| 権限モード                  | プロンプトが表示される場合                                                                      |
| :--------------------- | :--------------------------------------------------------------------------------- |
| Auto                   | 最初の起動のみ。任意の **Yes** はユーザー設定に同意を記録し、後の起動はプロンプトなしで開始します。ultracode がオンの場合は完全にスキップされます |
| Manual, accept edits   | すべての実行。ただし、このプロジェクトでそのワークフローに対して **Yes, and don't ask again** を選択した場合を除きます         |
| Bypass permissions     | Claude Code はプロンプトを表示しません。実行は直ちに開始されます                                             |
| `claude -p`, Agent SDK | Claude Code はプロンプトを表示しません                                                          |

`claude -p` と Agent SDK では、Claude Code はこのプロンプトを表示しません。セッションの残りの部分と同じ[権限評価](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated)を通じてワークフロー ツール呼び出しを実行するため、拒否ルール、質問ルール、および `dontAsk` モードはすべてのツール呼び出しに適用されるのと同じように適用されます。これらの実行でワークフローを開始させるには、次のいずれかを使用します。

* **権限ルール**: 許可ルール内の `Workflow` はすべてのワークフローを承認し、`Workflow(<name>)` は名前で 1 つの保存されたワークフローを承認します。
* **自動権限モード**: [分類器](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)は呼び出しをレビューし、それを承認できます。
* **バイパス権限モード**: Claude Code は呼び出しを承認します。
* **`PreToolUse` フック**: 呼び出しに対して `allow` を返す[フック](/docs/ja/hooks#pretooluse)はそれを承認します。
* **ホスト**: [`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags)がそれを承認するか、Agent SDK では [`canUseTool`](/docs/ja/agent-sdk/permissions) コールバックまたは [`PermissionRequest` フック](/docs/ja/hooks#permissionrequest)がそれを承認します。

デスクトップアプリでは、承認カードはワークフロー名、フェーズリスト、トークン使用量の注意を表示し、**Once**、**Always**、**Deny** アクションを表示します。進行状況ビューは、バックグラウンドタスクサイドペインに表示されます。

ワークフローが生成するサブエージェントは、[権限ルール](/docs/ja/settings-reference#permission-settings)を使用し、Claude Code は[サブエージェントが実行される権限モード](/docs/ja/sub-agents#permission-modes)の下のルールによって権限モードを選択します。長い実行でプロンプトを避けるには、エージェントが必要とするツールを開始する前に許可ルールに追加します。

<h3 id="save-the-workflow-for-reuse">
  再利用するためにワークフローを保存する
</h3>

Claude が繰り返すタスク用のワークフローを書く場合、その実行のスクリプトをコマンドとして保存できます。すべてのブランチで実行するレビューなどのプロセスは、毎回同じオーケストレーションを実行します。

`/workflows` を実行し、保持したい実行を選択して、`s` を押します。保存ダイアログで、Tab は 2 つの保存場所を切り替えます。

* プロジェクト内の `.claude/workflows/`。リポジトリをクローンする全員と共有されます
* ホームディレクトリ内の `~/.claude/workflows/`。すべてのプロジェクトで利用可能で、あなたにのみ表示されます。[`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars) を設定した場合、この場所はそのパスの下の `workflows/` ディレクトリです。

保存ダイアログは個人用の場所の解決されたパスを表示します。

Enter を押して保存します。ワークフローは、どちらかの場所からの将来のセッションで `/<name>` として実行されます。

Claude Code は書き込み前に保存場所をシンボリックリンクでチェックし、エラーを表示してシンボリックリンクを通じて書き込みません。チェックする内容は、保存する場所によって異なります。

* プロジェクト場所: `.claude`、`.claude/workflows`、またはターゲットファイルがシンボリックリンクの場合、Claude Code は拒否します。
* 個人用の場所: Claude Code はターゲットファイル自体がシンボリックリンクの場合のみ拒否するため、ドットファイルツールで管理される `~/.claude` ディレクトリは引き続き機能します。

v2.1.216 より前は、Claude Code はリンクをたどり、選択した場所の外にファイルを配置する可能性がありました。

複数の `.claude/` ディレクトリを持つモノレポでは、ワークフローを適用するパッケージの横に保持できます。プロジェクト場所に保存すると、作業ディレクトリとリポジトリルートの間に既に存在する最も近い `.claude/workflows/` ディレクトリに書き込むか、まだ存在しない場合はリポジトリルートに書き込みます。プロジェクトワークフローはそのパスに沿ったすべての `.claude/workflows/` からも読み込まれ、複数が同じ名前を定義する場合、Claude Code は作業ディレクトリに最も近いものを実行します。

プロジェクトワークフローと個人用ワークフローが名前を共有する場合、プロジェクトのものが実行されます。

<h3 id="distribute-a-workflow-in-a-plugin">
  プラグインでワークフローを配布する
</h3>

チーム間またはリポジトリ間でワークフローを共有するには、[プラグイン](/docs/ja/plugins/overview)に含めます。スクリプトをプラグインルートの `workflows/` ディレクトリに配置するか、[`workflows` マニフェストフィールド](/docs/ja/plugins/manifest-reference#fields)で別の場所を指します。

プラグインワークフローはプラグイン名でネームスペースされます。`meta.name` が `release-audit` のスクリプトを含む `acme-tools` というプラグインは `/acme-tools:release-audit` として実行されます。

<h3 id="pass-input-to-a-saved-workflow">
  保存されたワークフローに入力を渡す
</h3>

保存されたワークフローは `args` パラメーターを通じて入力を受け入れることができます。スクリプトは `args` という名前のグローバルとして読み込みます。スクリプトを編集するのではなく、呼び出し時に研究質問、ターゲットパスのリスト、または構成オブジェクトを提供するために使用します。

次のプロンプトは、問題番号のリストを使用して保存されたワークフローを実行します。

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
```

Claude はリストを構造化データとして渡すため、スクリプトは最初に解析することなく `args` に対して配列とオブジェクトメソッドを直接呼び出すことができます。`args` が省略された場合、グローバルはスクリプト内で `undefined` です。

<h2 id="example-workflow-prompts">
  ワークフローの実行例プロンプト
</h2>

ワークフローは、タスクが 1 つのエージェントがコンテキストに保持できるより大きい場合、または同じステップが多くのアイテムにわたって実行する必要がある場合に最適です。以下のプロンプトは一般的な形を示しています。それぞれは Claude にそのタスク用のワークフローを作成して実行するよう依頼します。スクリプト自体は作成しません。

<h3 id="audit-many-files-for-the-same-issue">
  同じ問題について多くのファイルを監査する
</h3>

1 つのエージェントをファイルごとにファンアウトし、その後、検出結果を収集して検証します。

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  チェックが合格するまで修正を続ける
</h3>

チェッカーを実行し、失敗したものを修正し、合格するか進捗が止まるまで繰り返します。

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  多くのファイルを並列でマイグレーションする
</h3>

マイグレーションするファイルを検出し、編集が競合しないように各ファイルを分離されたコピーで変換し、各結果を検証します。

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  すべての変更されたファイルをレビューして 1 つのサマリーを作成する
</h3>

ファイルごとにレビュアーを実行し、その後、すべての検出結果を 1 つのエージェントに渡して、それらをランク付けして重複排除します。

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  多くのソースにわたってトピックを研究する
</h3>

チェンジログ、問題、ドキュメント全体でリーダーをファンアウトし、その後、合成します。バンドルされた `/deep-research` ワークフローはこれを実行します。より狭いバージョンを説明することもできます。

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  リストが成長を停止するまで問題を見つける
</h3>

ラウンドで検索を続け、新しいラウンドが新しいものを見つけなくなったら停止します。

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  保存されたスクリプトの外観
</h3>

[ワークフローを保存](#save-the-workflow-for-reuse)すると、`.claude/workflows/` のファイルは `meta` ブロックの後にサブエージェントをオーケストレーションするスクリプト本体を保持します。通常は編集する必要はありませんが、ここは小さいものの形なので、Claude が生成したものを認識できます。

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

本体は最上位の `await` を持つプレーン JavaScript です。`agent()` は 1 つのサブエージェントを生成し、`pipeline()` はリスト内の 1 つのアイテムごとに 1 つを実行し、`parallel()` は一連のエージェント タスクを同時に実行してすべてが完了するのを待ちます。

`agent()` 呼び出しは、実行中に停止した場合または回復不可能な API エラーが発生した場合は `null` に解決されます。`pipeline()` はその `null` を結果配列に保持するため、例は `.filter(Boolean)` で終わってそれらのエントリを削除します。

[auto モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)では、スクリプトが `agent()` に渡すプロンプトは、分類器がそのサブエージェントのアクションをレビューするときにあなたからのリクエストとしてカウントされません。Claude Code はそれをスクリプトが計算したテキストとしてマークするためです。

`agent()` 呼び出しで `schema` を渡す場合、そのサブエージェントはプローズの代わりに形状に一致する JSON を返します。Claude Code はサブエージェントを開始する前にスキーマをチェックします。スキーマが矛盾していることを証明できる場合、呼び出しは矛盾を名前付けするエラーで失敗し、サブエージェントは開始されません。証明できる 1 つの矛盾は、`additionalProperties: false` が除外する `required` キーです。

サブエージェントの出力が 5 回の試行後も検証に失敗する場合、呼び出しは最後の検証失敗を含むエラーで失敗します。試行回数を変更するには、[`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/ja/env-vars) を設定します。

<h3 id="edit-a-saved-script">
  保存されたスクリプトを編集する
</h3>

[保存したワークフロー](#save-the-workflow-for-reuse)を変更するには、その `.js` ファイルを編集するか、Claude に変更を依頼します。編集または依頼する前に、`/workflow-authoring` [バンドルされたスキル](/docs/ja/skills#bundled-skills)を実行して、Claude が作業する対象のスクリプト作成リファレンスを読み込みます。スキルには Claude Code v2.1.248 以降が必要です。

現在のセッションで編集されたバージョンを実行するには、[`/reload-skills`](/docs/ja/commands#all-commands) を実行してワークフロー ディレクトリを再度読み込み、その後 `/<name>` を再度実行します。

Claude Code はスクリプトを読み込んで実行するときに、ファイルの各部分に次のルールを適用します。

* **`meta` ブロック**: `export const meta` を最初のステートメントとして保持し、`name` と `description` を持つプレーン オブジェクト リテラルとして保持します。変数、関数呼び出し、スプレッドなどのリテラル値以外のものが含まれている場合、Claude Code は `/` オートコンプリートから `/<name>` を削除します。
* **本体**: `agent()`、`pipeline()`、`parallel()` の他に、`phase()` を呼び出して、進捗ビューのタイトルの下に続くエージェントをグループ化し、`log()` を呼び出してフェーズの上にメッセージを表示し、[`args`](#pass-input-to-a-saved-workflow) グローバルを読み取ることができます。本体に構文エラーがある場合、Claude Code はワークフローを実行するときにそれを報告します。
* **`phases`**: `meta` にそれらをリストする場合、`phase()` に渡す各エントリに正確にタイトルを付けます。エントリのない `phase()` タイトルは独自の進捗グループを取得します。
* **タイムスタンプとランダム性**: Claude Code はスクリプト内で `Date.now()`、`Math.random()`、および引数なしの `new Date()` をスローするため、[再開された実行](#resume-after-a-pause)は同じ `agent()` 呼び出しを繰り返します。代わりに `args` を通じてタイムスタンプを渡します。

保存されたコピーではなく、[単一の実行のスクリプト](#how-a-workflow-runs)を編集することもできます。[一時停止後に再開](#resume-after-a-pause)は、編集されたスクリプトを再開したときにどのエージェントが再度実行されるかについて説明します。Workflow ツールの入力については、[Agent SDK リファレンス](/docs/ja/agent-sdk/typescript#workflow)のそのエントリを参照してください。

<h2 id="how-a-workflow-runs">
  ワークフローの実行方法
</h2>

ワークフローランタイムは、会話から分離された隔離環境でスクリプトを実行します。中間結果は Claude のコンテキストに入る代わりにスクリプト変数に留まります。

すべての実行は、セッションディレクトリの `~/.claude/projects/` 配下のファイルにスクリプトを書き込みます。実行が開始されると Claude はパスを受け取るため、それを尋ねることができます。そのファイルを開いて、Claude が作成したオーケストレーションを読んだり、前回の実行のスクリプトと比較したり、編集して Claude に編集版から再起動するよう依頼したりできます。

Claude がワークフローを開始できるのは、セッションが既に読み取りを許可されているスクリプトファイルからのみです。作業ディレクトリの外に保存されているスクリプトを実行するには、まず [`/add-dir`](/docs/ja/permissions#working-directories) でそのディレクトリを追加するか、[Read 許可ルール](/docs/ja/permissions#read-and-edit)を設定してください。

ランタイムは実行が進むにつれて各エージェントの結果を追跡します。これが実行を[一時停止後に再開](#resume-after-a-pause)可能にする理由です。同じセッション内で。

<h3 id="prompt-caching-in-a-fan-out">
  ファンアウトでのプロンプトキャッシング
</h3>

同じ実行内のエージェントは、互いの[プロンプトキャッシュ](/docs/ja/prompt-caching#subagents-and-the-cache)を読み取ることができます。同じモデル、努力レベル、エージェントタイプ、ツール、出力スキーマ、および作業ディレクトリで実行される 2 つのエージェントは、同じツールおよびシステムプロンプトプレフィックスを構築するため、マッチングする兄弟の応答が開始された後に開始されるエージェントは、最初のリクエストでその兄弟のキャッシュを読み取ります。

ワークフローエージェントのリクエストはメイン会話の[キャッシュ TTL バケット](/docs/ja/prompt-caching#which-ttl-each-request-gets)の外にあるため、そのキャッシュはデフォルトで 5 分間保持されます。Claude サブスクリプションでも同様です。1 時間保持するには、[`subagentPromptCacheTtl`](/docs/ja/settings-reference#subagentpromptcachettl) を `1h` に設定してください。API は 1 時間のキャッシュ書き込みをより高いレートで課金します。

ファンアウトが複数のマッチングエージェントを一度に開始する場合、Claude Code は最初のエージェント以外をすべて保持し、最初のエージェントの応答が開始されるまで待機してから、保持されたエージェントを一緒にリリースして、最初のリクエストで共有プレフィックスを読み取り、各エージェントがキャッシュなしで処理するのを避けます。Claude Code は保持を [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/ja/env-vars) ミリ秒でキャップします。デフォルトは `5000` です。保持を無効にするには `0` に設定してください。

<h3 id="behavior-and-limits">
  動作と制限
</h3>

ランタイムは以下の制約を適用します。

| 制約                                                                                                                                                                                                                     | 理由                                                                                                              |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| 実行中のユーザー入力なし                                                                                                                                                                                                           | 実行は、エージェント権限プロンプトと[使用制限待機](#when-a-run-hits-your-usage-limit)のみで一時停止します。ステージ間の署名のために、各ステージを独自のワークフローとして実行してください |
| ワークフロー自体からの直接ファイルシステムまたはシェルアクセスなし                                                                                                                                                                                      | エージェントは読み取り、書き込み、コマンドを実行します。スクリプトはエージェントを調整します                                                                  |
| モジュール読み込みなし：`import()` を含むスクリプトは実行開始前に失敗します                                                                                                                                                                            | スクリプト本体はプレーン JavaScript です。ライブラリが必要な作業はエージェントのタスクに配置してください                                                      |
| 最大 16 個の同時エージェント。デフォルトでは、CPU が限定されたコンテナ内を含め、Claude Code が利用可能な CPU が少ない場合はより少ない。制限を変更するには、[`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/ja/env-vars#variables) を 1 から 256 の値に設定します。これには Claude Code v2.1.269 以降が必要です | ローカルリソース使用を制限します                                                                                                |
| ファンアウトでは、最初のエージェントのプロンプトキャッシュプレフィックスを共有するエージェントはデフォルトで最初のエージェントの 5 秒後までに開始します                                                                                                                                          | すべてが最初のエージェント以外は、[最初のエージェントがキャッシュしたプレフィックス](#prompt-caching-in-a-fan-out)を読み取り、各エージェントがキャッシュなしで処理するのを避けます       |
| 単一の `parallel()` または `pipeline()` 呼び出しで最大 4,096 個のアイテム：ランタイムはより長いリストをエラーで拒否します                                                                                                                                         | サイレント上限はスクリプトに通知せずにワークロードの一部をドロップします                                                                            |
| 実行ごとに合計 1,000 エージェント                                                                                                                                                                                                   | 暴走ループを防止します                                                                                                     |

<h2 id="manage-runs">
  ランを管理する
</h2>

ランが開始されると、`/workflows` ビューから、または入力ボックス下のタスクパネルの進捗行を展開して管理できます。

ランを停止すると、そのエージェントのプロセスがまだ実行中の間、タスクパネルに留まります。もう一度停止すると、Claude Code はそれらのプロセスに再度シグナルを送信します。

<h3 id="resume-after-a-pause">
  一時停止後に再開する
</h3>

一時停止されたランを `/workflows` から再開するには、それを選択して `p` を押します。停止したランの場合は、Claude に同じスクリプトでワークフローを再起動するよう依頼してください。停止したランのエージェントがまだ終了していない場合、Claude Code は再起動を拒否するため、それらのエージェントの 2 番目のコピーが並行して実行されることはありません。

Claude Code はエージェントが開始した順序でランを再生し、各エージェントは保存された結果を返すか、再度実行します。

* **完了**: 保存された結果を返します。スクリプトを編集したか、前のエージェントが異なる結果を返したため、プロンプトが前回の実行と異なる最初のエージェントが再度実行され、その後のすべてのエージェント（完了したものも含む）も実行されます。
* **停止時にまだ実行中**: 最初からやり直します。ランぜんたいを停止しても、どのエージェントも失敗とはカウントされません。
* **失敗**: 再度実行され、その後に開始したすべてのエージェント（完了したものも含む）も実行されます。[`/workflows`](#watch-the-run) でエージェントを選択して `x` を押すことで 1 つのエージェントだけを停止することは、失敗とカウントされます。

最後のケースは、既に完了した作業をファンアウトで再実行する中間の失敗を意味します。スクリプトが A、B、C、D をその順序で開始し、B が失敗した場合、再起動すると A はキャッシュから返され、B、C、D が再度実行されます。

同じ Claude Code セッション内でランを再開できます。セッションを離れるときに実行中のワークフローに何が起こるかは、どのように離れるかによって異なります。

* [セッションをバックグラウンドにする](/docs/ja/agent-view#what-carries-over-when-you-background)場合、Claude Code はバックグラウンドセッションで同じ方法でランを再生し、それを続行します。
* [エージェントビューがオン](/docs/ja/agent-view#from-inside-a-session)の状態で Claude Code を終了しながらワークフローが実行中の場合、終了ダイアログは `Move to background and exit` を提供し、ランを同じ方法で引き継ぎます。代わりに `Exit and stop tasks` を選択するか、オプションが提供されない場合、ランはセッションで停止します。Claude Code はランの保存された結果を `~/.claude/projects/` のそのセッションのディレクトリの下に保持するため、`claude --resume` で再開するセッションは、Claude にワークフローを再起動するよう依頼するときにそれらを再生できます。新しく開始するセッションでは、Claude は再起動する前のランを持たず、ワークフローを新しいランとして最初から開始します。

[クラウドセッション](/docs/ja/claude-code-on-the-web)では、Claude Code はランの結果をセッションの会話履歴とともに保存し、セッションの VM が回収されるときに生き残ります。[そのようなセッションを再度開く](/docs/ja/claude-code-on-the-web#environment-expired)ときに Claude にワークフローを再起動するよう依頼すると、完了したエージェントは依然として保存された結果を返します。

ローカルセッションとクラウドセッションの両方で、Claude が前のランを再起動し、Claude Code がそのランの保存された結果をまったく見つけられない場合、再起動は `nothing to resume` エラーで失敗し、ランを独自に最初からやり直しません。Claude にワークフローを新しいランとして最初からやり直すよう依頼してください。

<h3 id="when-a-run-hits-your-usage-limit">
  ランが使用制限に達したとき
</h3>

エージェントが claude.ai [使用制限](/docs/ja/interactive-mode#wait-for-a-usage-limit-to-reset)に達すると、そのエージェントを失敗させるのではなく、ランは一時停止します。制限に達したエージェントはリセットを待ち、新しいエージェントは開始しません。制限がリセットされた直後に、待機中のエージェントが再度実行され、ランは自動的に続行されます。Claude Code v2.1.271 以降が必要です。以前のバージョンでは、影響を受けたエージェントは失敗します。

ランが待機している間、タスクパネルの進捗行と [`/workflows`](#watch-the-run) ヘッダーは制限がいつリセットされるかを表示します。

ランは、これらすべてが成立する場合にのみ一時停止します。1 つが成立しない場合、影響を受けたエージェントは代わりに失敗します。

* セッションはインタラクティブで、claude.ai サブスクリプションでサインインしています。[非インタラクティブモード](/docs/ja/headless)で `claude -p` または [Agent SDK](/docs/ja/agent-sdk/overview) を使用する場合、[バックグラウンドセッション](/docs/ja/agent-view)、または [Remote Control](/docs/ja/remote-control) または [エージェントチーム](/docs/ja/agent-teams)のチームメイトセッションではランは一時停止しません。
* [`autoContinueAtUsageLimit`](/docs/ja/settings-reference#autocontinueatusagelimit) がオンで、セッション自体が [使用制限がリセットされるのを待つ](/docs/ja/interactive-mode#wait-for-a-usage-limit-to-reset)ことができる同じ設定です。待機中にオフにすると、待機が終了し、待機中のエージェントは失敗します。
* 制限は 24 時間以内にリセットされます。週単位の制限はさらに先にリセットされる可能性があります。
* ランはまだ 2 回待機していません。3 回目に制限に達すると、エージェントは失敗します。

<h3 id="cost">
  コスト
</h3>

ワークフローは多くのエージェントを生成するため、1 つのランは会話で同じタスクを処理するよりも意味のあるほど多くのトークンを使用できます。ランはプランの使用量とレート制限にカウントされます。

大きなタスクにコミットする前に支出を測定するには、小さなスライスでワークフローを実行してください。1 つのディレクトリではなくリポジトリ全体、または広い質問ではなく狭い質問です。`/workflows` ビューはランが進行するにつれて各エージェントのトークン使用量を表示し、いつでもランを停止できます。通常、完了した作業を失うことなく停止できます。[一時停止後に再開する](#resume-after-a-pause)は、停止したランが何を保持するかについて説明しています。ランタイムの[エージェント上限](#behavior-and-limits)は、1 つのランが生成できるエージェント数を制限し、暴走スクリプトのコストを制限します。ランを少ないエージェント数に保つには、`small` [サイズガイドライン](#set-a-size-guideline)を選択してください。

Claude Code はまた、異常に大きくなるランにフラグを立てます。ワークフローが 25 個以上のエージェントをスケジュールするか、その予想トークン合計が 150 万を超える場合、入力ボックス下のタスクパネルの進捗行は `Large workflow` 警告を表示します。警告は [`/workflows`](#watch-the-run) を指し、そこでランを停止できます。

警告は参考情報です。ランを一時停止または制限しません。警告が表示されるときに 2 つの設定が変わります。

* [サイズガイドライン](#set-a-size-guideline)を自分で選択する場合、そのエージェント数は 25 エージェント閾値を置き換えます。組み込みのデフォルトガイドラインは閾値を 25 のままにします。
* [ultracode](#let-claude-decide-with-ultracode) がオンのセッションは警告を表示しません。ultracode をオンにすることで既に大規模なランにオプトインしているためです。

Claude Code は各ワークフローエージェントのモデルを、[サブエージェントに使用する同じ順序](/docs/ja/sub-agents#choose-a-model)で選択します。スクリプトがステージに名前を付けるモデルは、その順序でのエージェントごとのモデルとしてカウントされます。他に何も割り当てない場合、エージェントはセッションのモデルで実行されます。

モデルコストを制御するには。

* 通常のルーチン作業のために小さいモデルに切り替える場合は、大規模なランの前に `/model` を確認してください
* タスクを説明するときに、最も強力なものを必要としないステージに対して小さいモデルを使用するよう Claude に依頼してください

組織の [`availableModels` 許可リスト](/docs/ja/model-config#restrict-model-selection)がスクリプトがエージェントに要求するモデルをブロックする場合、そのエージェントは代わりに置き換えられたモデルで実行され、[サブエージェントと同じ置き換えルール](/docs/ja/sub-agents#choose-a-model)に従います。[`/workflows`](#watch-the-run) の実行の進捗ビューは、要求されたモデルと置き換えられたモデルの両方に名前を付ける警告を表示します。

<h3 id="set-a-size-guideline">
  サイズガイドラインを設定する
</h3>

サイズガイドラインは、動的ワークフローを作成するときに Claude が目指すエージェント数を Claude に指示します。Claude Code はガイドラインを上限ではなくアドバイスとして Claude に送信するため、異なるスケールを要求するプロンプトはそれでも上書きします。Claude Code v2.1.202 以降が必要です。

各値はエージェント数にマップされます。

| 値              | Claude が目指すエージェント数                   |
| :------------- | :----------------------------------- |
| `unrestricted` | ガイドラインなし。Claude はワークフローをタスクにサイズ設定します |
| `small`        | 5 個未満のエージェント                         |
| `medium`       | 10 個未満のエージェント                        |
| `large`        | 50 個未満のエージェント                        |

デフォルトは `medium` です。または Claude Code v2.1.271 以降で Pro プランでサインインしている場合は `small` です。値を選択するまで、`/config` 行は値をデフォルトとしてマークし、ワークフローの `Running in background` 行は有効なサイズに名前を付けます。Claude Code v2.1.219 以降が必要です。以前のバージョンはデフォルトで `unrestricted` です。

ガイドラインを変更するには、`/config` で Dynamic workflow size 設定の値を選択するか、`/config workflowSizeGuideline=small` を実行します。v2.1.219 以降では、任意の設定ファイルで [`workflowSizeGuideline` キー](/docs/ja/settings-reference#workflowsizeguideline)を設定することもできます。その値は `/config` より優先され、Claude Code は設定ファイルが 1 つを提供している間は `/config` 行を非表示にします。

変更は次のプロンプトで有効になります。[ランタイムエージェント上限](#behavior-and-limits)は設定に関係なく依然として適用されます。

<h3 id="turn-workflows-off">
  ワークフローをオフにする
</h3>

ワークフローは CLI、デスクトップアプリ、IDE 拡張機能、[非インタラクティブモード](/docs/ja/headless)で `claude -p`、および [Agent SDK](/docs/ja/agent-sdk/overview) で利用できます。同じ無効化設定がすべてのサーフェスに適用されます。

自分自身のワークフローをオフにするには。

* `/config` で Dynamic workflows をオフに切り替えます。セッション間で保持されます。
* `~/.claude/settings.json` で `"disableWorkflows": true` を設定します。セッション間で保持されます。
* `CLAUDE_CODE_DISABLE_WORKFLOWS=1` を設定します。起動時に読み込まれるため、設定した場所に適用されます。

組織全体のワークフローをオフにするには、[管理設定](/docs/ja/server-managed-settings)で `"disableWorkflows": true` を設定するか、[Claude Code 管理設定](https://claude.ai/admin-settings/claude-code)ページのトグルを使用します。

ワークフローが無効化されると、バンドルされたワークフローコマンドと `/workflow-authoring` スキルは利用できなくなり、`ultracode` キーワードはランをトリガーしなくなり、`ultracode` は `/effort` メニューから削除されます。

<h2 id="related-resources">
  関連リソース
</h2>

* [エージェントを並列実行](/docs/ja/agents)：サブエージェント、エージェントビュー、エージェントチーム、ワークフローを比較
* [カスタムサブエージェントを作成](/docs/ja/sub-agents)：ワークフローがオーケストレーションするワーカープリミティブ
* [コストを管理](/docs/ja/costs)：マルチエージェント実行が使用制限にカウントされる方法
