> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code をクラウドで始める

> ブラウザまたはスマートフォンからクラウドで Claude Code を実行します。GitHub リポジトリを接続し、タスクを送信し、ローカルセットアップなしで PR をレビューします。

<Note>
  クラウドセッションは、Pro、Max、Team プランおよび premium seats または Chat + Claude Code seats を持つ Enterprise ユーザーを対象としています。
</Note>

クラウドセッションは、あなたのマシンではなく Anthropic が管理するクラウドインフラストラクチャで Claude Code を実行します。このクイックスタートでは、ブラウザから [claude.ai/code](https://claude.ai/code) で開始します。Claude モバイルアプリ、Desktop アプリ、またはターミナルから `claude --cloud` で開始することもできます。

[始めるには](#connect-github) GitHub リポジトリが必要です。Claude はそれを分離された仮想マシンにクローンし、変更を加え、レビュー用のブランチをプッシュします。セッションはデバイス間で永続化されるため、ラップトップで開始したタスクは後でスマートフォンからレビューする準備ができています。

クラウドセッションは以下に適しています：

* **並列タスク**：複数の worktrees を管理することなく、複数の独立したタスクを同時に実行し、それぞれ独自のセッションとブランチで実行します
* **ローカルにないリポジトリ**：Claude はセッションごとにリポジトリを新規クローンするため、チェックアウトする必要がありません
* **頻繁なステアリングが不要なタスク**：明確に定義されたタスクを送信し、他のことをして、Claude が完了したときに結果をレビューします
* **コードの質問と探索**：ローカルチェックアウトなしでコードベースを理解したり、機能がどのように実装されているかをトレースします

ローカル設定、ツール、または環境が必要な作業の場合は、Claude Code をローカルで実行するか、[Remote Control](/docs/ja/remote-control) を使用する方が適しています。

<h2 id="how-sessions-run">
  セッションの実行方法
</h2>

以下の手順は Anthropic がホストするセッションについて説明しています。[自己ホスト環境](/docs/ja/self-hosted-environments)では、クローンとそれ以降のすべてが組織独自のランナー上で実行され、ネットワーク境界、セットアップ、プッシュ動作はオペレーターが設定します。タスクを送信すると：

1. **クローンと準備**：リポジトリが Anthropic が管理する VM にクローンされ、設定されている場合は [setup script](/docs/ja/cloud-environments#setup-scripts) が実行されます。
2. **ネットワークの設定**：インターネットアクセスは環境の [access level](/docs/ja/cloud-environments#access-levels) に基づいて設定されます。
3. **作業**：Claude はコードを分析し、変更を加え、テストを実行し、その作業をチェックします。全体を監視してステアリングすることも、完了したら戻ってくることもできます。
4. **ブランチをプッシュ**：Claude が停止ポイントに達すると、ブランチを GitHub にプッシュします。diff をレビューし、インラインコメントを残し、PR を作成するか、別のメッセージを送信して続行します。

ブランチがプッシュされてもセッションは閉じません。PR の作成とさらなる編集はすべて同じ会話内で行われます。

<h2 id="compare-ways-to-run-claude-code">
  Claude Code を実行する方法を比較
</h2>

Claude Code はどこでも同じように動作します。変わるのは、セッションが実行される場所とローカル設定が利用可能かどうかです。

|                                   | クラウドセッション                                                                                               | ローカルセッション                                                                                                 | [リモートコントロール](/docs/ja/remote-control)を使用したローカルセッション |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| **コードが実行される場所**                   | クラウド VM、Anthropic 管理（デフォルト）                                                                             | お客様のマシン                                                                                                   | お客様のマシン                                        |
| **セッションを開始する場所**                  | claude.ai/code、Claude モバイルアプリ、**Cloud** が選択された Desktop app、または `claude --cloud`                         | お客様のターミナル、お客様の IDE、または **Local** が選択された Desktop app                                                       | お客様のターミナル、VS Code 拡張機能、または Desktop app         |
| **チャットを行う場所**                     | claude.ai、モバイルアプリ、または Desktop app                                                                       | セッションを開始した場所                                                                                              | claude.ai またはモバイルアプリ、およびセッションを開始した場所           |
| **ローカル設定を使用**                     | いいえ、リポジトリのみ                                                                                             | はい                                                                                                        | はい                                             |
| **GitHub が必要**                    | はい、または `--cloud` 経由で[ローカルリポジトリをバンドル](/docs/ja/claude-code-on-the-web#send-local-repositories-without-github) | いいえ                                                                                                       | いいえ                                            |
| **切断時に実行を継続**                     | はい                                                                                                      | いいえ                                                                                                       | セッションがお客様のマシンで開いている間                           |
| **[権限モード](/docs/ja/permission-modes)** | 編集を受け入れる、Plan、Auto                                                                                      | ターミナルのすべてのモード。IDE と Desktop app については[権限モードを切り替える](/docs/ja/permission-modes#switch-permission-modes)を参照してください | claude.ai とモバイルアプリから Manual、編集を受け入れる、または Plan  |
| **ネットワークアクセス**                    | 環境ごとに設定可能                                                                                               | お客様のマシンのネットワーク                                                                                            | お客様のマシンのネットワーク                                 |

ローカルセッションをセットアップするには、[ターミナルクイックスタート](/docs/ja/quickstart)、[Desktop app](/docs/ja/desktop)、または [リモートコントロール](/docs/ja/remote-control) ドキュメントを参照してください。

<h2 id="connect-github">
  GitHub を接続
</h2>

GitHub への接続は 1 回限りのステップです。既に GitHub CLI を使用している場合は、ブラウザの代わりに [ターミナルからこれを実行](#connect-from-your-terminal) できます。

<Note>
  Team および Enterprise プランでは、**Sign in with GitHub** ステップは、Claude 組織の [Owner](/docs/ja/server-managed-settings#access-control) が [**Admin settings > Connectors**](https://claude.ai/admin-settings/connectors) で GitHub コネクタをオンにした後にのみ機能します。それまでは、そのステップはサインインボタンの代わりに「GitHub access is required for Claude Code on the web」と表示されます。コネクタがオンになった後、[claude.ai/code](https://claude.ai/code) をリロードして、最初のステップから再度開始します。[**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code) の [Quick web setup](/docs/ja/claude-code-on-the-web#github-authentication-options) という 2 番目のトグルはオプションです。オンにすると、`/web-setup` が機能し、オンボーディングはメンバーの環境を作成します。
</Note>

<Steps>
  <Step title="claude.ai/code にアクセス">
    [claude.ai/code](https://claude.ai/code) にアクセスし、claude.ai アカウントでサインインします。
  </Step>

  <Step title="GitHub でサインイン">
    サインイン後、claude.ai/code は GitHub を接続するよう促します。プロンプトに従うと、claude.ai/code は GitHub の認可ページに移動します。認可リクエストを承認すると、GitHub は claude.ai/code に戻ります。Cloud セッションは既存の GitHub リポジトリで機能します。新しいプロジェクトを開始するには、まず [GitHub に空のリポジトリを作成](https://github.com/new) してください。

    この接続により、セッションは任意のパブリックリポジトリをクローンできますが、プライベートリポジトリで機能するのは Claude GitHub App がインストールされている場合のみです。[Claude GitHub App をインストール](https://github.com/apps/claude/installations/new) してください。使用したいプライベートリポジトリを持つ各 GitHub アカウントまたは Organization に対してインストールします。GitHub Organization では、Organization オーナーがインストールを承認する必要がある場合があります。App をインストールすると、[Auto-fix](/docs/ja/claude-code-on-the-web#auto-fix-pull-requests) も有効になります。これにより、Claude はこれらのリポジトリの pull request の CI 失敗とレビューコメントに応答できます。

    オンボーディングがこの時点で Claude GitHub App をインストールするよう促し、後で実行したい場合は、**Skip** をクリックします。
  </Step>

  <Step title="デフォルト環境をセットアップ">
    [cloud environment](/docs/ja/cloud-environments) は、セッション中に Claude が持つネットワークアクセスと、セッション開始時に実行される内容を制御する保存された設定です。GitHub を接続した後の動作はプランによって異なります。

    * **Pro および Max**：オンボーディングは **Default** という名前の環境を作成します。
    * **Team および Enterprise**：オンボーディングは **Create your first cloud environment** フォームを表示します。事前入力された名前とネットワークアクセスを変更せず、**Create & finish** をクリックして **Default** 環境を作成します。Owner が [Quick web setup](/docs/ja/claude-code-on-the-web#github-authentication-options) をオンにしている場合、オンボーディングは代わりに **Default** を作成します。

    **Default** は [`Trusted` ネットワークアクセス](/docs/ja/cloud-environments#access-levels) を使用します。セッションは [common package registries](/docs/ja/cloud-environments#default-allowed-domains) およびその他のホワイトリストに登録されたドメインに到達し、セッションのネットワークを通じて他には何も到達しません。設定なしで利用可能な内容については、[Installed tools](/docs/ja/cloud-environments#installed-tools) を参照してください。

    最初のプロジェクトの場合、**Default** 環境はそのまま機能します。ネットワークアクセスを変更したり、環境変数を追加したり、セッション開始前に [setup script](/docs/ja/cloud-environments#setup-scripts) を実行したりするには、[環境を編集するか、追加の環境を作成](/docs/ja/cloud-environments#configure-your-environment) してください。
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  ターミナルから接続
</h3>

既に GitHub CLI（`gh`）を使用している場合は、ターミナルから cloud セッション用に GitHub を接続できます。これには [Claude Code CLI](/docs/ja/quickstart) が必要です。Team および Enterprise プランでは、`/web-setup` は Owner が [Quick web setup](/docs/ja/claude-code-on-the-web#github-authentication-options) をオンにした後にのみ利用可能です。

`/web-setup` を実行すると、Claude Code は `gh auth token` が出力するトークンを読み取り、確認を求め、トークンを Anthropic に送信します。Anthropic はそれを claude.ai アカウントで暗号化して保存し、cloud セッションはそれを GitHub アクセスに使用します。これは [削除](#remove-the-web-setup-token) するまで続きます。自分で開始した cloud セッションは、そのトークンがアクセスできる任意のリポジトリにアクセスでき、Claude GitHub App をインストールする必要はありません。[project](/docs/ja/claude-projects#set-up-github-access) 内のスレッドは引き続き Claude GitHub App が必要です。

既にブラウザで GitHub を接続している場合、`/web-setup` は続行すると cloud セッションの接続が置き換わることを警告します。

<Note>
  [Zero Data Retention](/docs/ja/zero-data-retention) が有効な Organization は `/web-setup` または他の cloud セッション機能を使用できません。GitHub CLI がインストールされていない、または認証されていない場合、Claude Code はブラウザオンボーディングフローを開きます。
</Note>

<Steps>
  <Step title="GitHub CLI で認証">
    シェルで、まだ認証していない場合は GitHub CLI を認証します：

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Claude にサインイン">
    Claude Code CLI で `/login` を実行して、claude.ai アカウントでサインインします。既に claude.ai アカウントでサインインしている場合はこのステップをスキップします。API キーで認証することはカウントされません。確認するには、`/status` を実行し、**Login method** 行に claude.ai アカウントが表示されることを確認します。
  </Step>

  <Step title="/web-setup を実行">
    Claude Code CLI で以下を実行します：

    ```text theme={null}
    /web-setup
    ```

    `gh` トークンを Claude アカウントに送信するプロンプトを確認します。成功すると、Claude Code は `Connected as <your-github-username>` を出力し、[claude.ai/code](https://claude.ai/code) をブラウザで開きます。cloud 環境がまだない場合、`/web-setup` は Trusted ネットワークアクセスと setup script なしで環境を作成します。後で [環境を編集したり、変数を追加](/docs/ja/cloud-environments#configure-your-environment) できます。`/web-setup` が完了したら、[`--cloud`](/docs/ja/claude-code-on-the-web#from-terminal-to-cloud) でターミナルから cloud セッションを開始するか、[`/schedule`](/docs/ja/routines) で定期的なタスクをセットアップできます。
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  `/web-setup` トークンを削除
</h4>

Claude アカウントからトークンを削除するには、[claude.ai/customize/connectors](https://claude.ai/customize/connectors) で GitHub を切断します。切断すると、ブラウザから来たか `/web-setup` から来たかに関わらず、cloud セッションが使用する GitHub 認証情報が削除されるため、cloud セッションは再度接続するまで GitHub アクセスを失います。ローカルの `gh` はサインインしたままで、トークンは GitHub で有効なままです。

トークン自体を無効にするには、GitHub でそれを取り消します。ブラウザを通じて `gh` にサインインした場合、トークンは GitHub の [**Settings > Applications > Authorized OAuth Apps**](https://github.com/settings/applications) の **GitHub CLI** エントリに属し、そのエントリを取り消すと、マシン上の GitHub CLI もサインアウトします。Cloud セッションは `gh auth login` と `/web-setup` を再度実行するまで GitHub アクセスを失います。

<h2 id="start-a-task">
  タスクを開始
</h2>

GitHub が接続され、環境が作成されたら、タスクを送信する準備ができています。

<Steps>
  <Step title="リポジトリとブランチを選択">
    [claude.ai/code](https://claude.ai/code) または Claude モバイルアプリの Code タブから、入力ボックスの下のリポジトリセレクターをクリックし、Claude が作業するリポジトリを選択します。各リポジトリはブランチセレクターを表示します。デフォルトの代わりに feature ブランチから Claude を開始するように変更します。複数のリポジトリを追加して、1 つのセッション内で複数のリポジトリで作業できます。
  </Step>

  <Step title="権限モードを選択">
    入力の横の mode ドロップダウンは、セッションが実行される権限モードを表示します：

    * **Auto**：分類器が Claude のアクションをレビューします。組織が auto モードを許可し、選択されたモデルがそれをサポートしている場合に表示されます
    * **Accept edits**：Claude は承認を待たずに変更を加えてブランチをプッシュします
    * **Plan**：Claude がアプローチを提案し、ファイルを編集する前にあなたの承認を待ちます

    Cloud セッションは Manual または Bypass 権限を提供しません。各権限モードが何を許可するかについては、[権限モードの完全なリスト](/docs/ja/permission-modes#available-modes)を参照してください。
  </Step>

  <Step title="タスクを説明して送信">
    実行したい内容の説明を入力して Enter キーを押します。具体的にしてください：

    * ファイルまたは関数に名前を付けます：「Add a README with setup instructions」または「Fix the failing auth test in `tests/test_auth.py`」は「fix tests」より良いです
    * エラー出力がある場合は貼り付けます
    * 症状だけでなく、期待される動作を説明します

    Claude はリポジトリをクローンし、設定されている場合は setup script を実行し、作業を開始します。各タスクは独自のセッションと独自のブランチを取得するため、1 つが完了するのを待つ必要はありません。
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  セッションを事前入力
</h2>

[claude.ai/code](https://claude.ai/code) URL にクエリパラメータを追加することで、新しいセッションのプロンプト、リポジトリ、環境を事前入力できます。これを使用して、issue tracker のボタンなどの統合を構築し、issue の説明をプロンプトとして Claude Code を開きます。

| Parameter      | Description                                                                                         |
| :------------- | :-------------------------------------------------------------------------------------------------- |
| `prompt`       | 入力ボックスに事前入力するプロンプトテキスト。エイリアス `q` も受け入れられます。                                                         |
| `prompt_url`   | クエリ文字列に埋め込むには長すぎるプロンプトのプロンプトテキストを取得する URL。URL はクロスオリジンリクエストを許可する必要があります。`prompt` も設定されている場合は無視されます。 |
| `repositories` | 事前選択する `owner/repo` スラッグのコンマ区切りリスト。エイリアス `repo` も受け入れられます。                                          |
| `environment`  | 事前選択する [environment](#connect-github) の名前または ID。                                                    |

各値を URL エンコードします。以下の例は、プロンプトとリポジトリが既に選択された状態でフォームを開きます：

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  レビューと反復
</h2>

Claude が完了したら、変更をレビューし、特定の行にフィードバックを残し、diff が正しく見えるまで続行します。

<Steps>
  <Step title="diff ビューを開く">
    diff インジケーターはセッション全体で追加および削除された行を表示します（例：`+42 -18`）。それを選択して diff ビューを開き、左側にファイルリスト、右側に変更が表示されます。

    diff はデフォルトではセッションの変更をベースブランチと比較します。別のブランチと比較するには、**Compare against** を選択して 1 つを選びます。
  </Step>

  <Step title="インラインコメントを残す">
    diff 内の任意の行を選択し、フィードバックを入力して Enter キーを押します。コメントは次のメッセージを送信するまでキューに入り、その後バンドルされます。Claude は「`src/auth.ts:47` で、ここでエラーをキャッチしないでください」をメインの指示と一緒に見るため、問題がどこにあるかを説明する必要はありません。
  </Step>

  <Step title="pull request を作成">
    diff が正しく見えたら、diff ビューの上部にある **Create PR** を選択します。完全な PR として開くか、ドラフトとして開くか、生成されたタイトルと説明で GitHub の作成ページにジャンプできます。
  </Step>

  <Step title="PR 作成後も反復を続ける">
    PR が作成された後もセッションはライブのままです。CI 失敗出力またはレビュアーのコメントをチャットに貼り付け、Claude にそれらに対処するよう依頼します。Claude に PR を自動的に監視させるには、[Auto-fix pull requests](/docs/ja/claude-code-on-the-web#auto-fix-pull-requests) を参照してください。
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  セットアップのトラブルシューティング
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  GitHub 接続後にリポジトリが表示されない
</h3>

ブラウザで GitHub を接続した場合、セッションはすべてのパブリックリポジトリをクローンできますが、プライベートリポジトリは Claude GitHub App がそれを所有するアカウントまたは組織にインストールされており、インストールのリポジトリアクセスにそれが含まれている場合にのみ表示されます。[Claude GitHub App をインストール](https://github.com/apps/claude/installations/new)するか、組織の所有者にインストールまたは承認を依頼してください。

`/web-setup` で接続した場合、セッションは `gh` トークンがアクセスできるすべてのリポジトリに到達できます。シェルで `gh repo view OWNER/REPO` を実行して、GitHub CLI ログインがリポジトリを見ることができることを確認し、接続以降に `gh` アカウントを切り替えた場合は `/web-setup` を再度実行してください。

<h3 id="the-page-only-shows-a-github-login-button">
  ページに GitHub ログインボタンのみが表示される
</h3>

クラウドセッションには接続された GitHub アカウントが必要です。上記のブラウザフローで接続するか、GitHub CLI を使用する場合はターミナルから `/web-setup` を実行してください。GitHub をまったく接続したくない場合は、[Remote Control](/docs/ja/remote-control) を参照して、自分のマシンで Claude Code を実行し、ブラウザまたは電話から監視してください。

<h3 id="not-available-for-the-selected-organization">
  「選択した組織では利用できません」
</h3>

エンタープライズ組織では、所有者がクラウドセッションを有効にする必要がある場合があります。Anthropic アカウントチームにお問い合わせください。

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` が「Claude にサインインしていません」と表示される
</h3>

`/web-setup` が「Not signed in to Claude. Run /login first.」と応答する場合、CLI には有効な claude.ai サインインがありません。これは以前のサインインの有効期限が切れた場合にも発生する可能性があります。`/login` を実行し、claude.ai アカウントでサインインしてから、`/web-setup` を再度実行してください。

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` がトークンに `workflow` スコープがないことを警告する
</h3>

`/web-setup` が GitHub CLI トークンに `workflow` スコープがないと表示される場合、続行できますが、GitHub はそのトークンで行われた一部のプッシュ（GitHub Actions ワークフローファイルを変更するプッシュなど）を拒否する可能性があります。スコープを追加するには、シェルで `gh auth refresh -s workflow` を実行してから、`/web-setup` を再度実行してください。

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` が「No commands match」または「Unknown command」を表示する
</h3>

`/web-setup` は Claude Code CLI 内で実行され、シェルではありません。まず `claude` を起動してから、プロンプトで `/web-setup` と入力してください。

Claude Code 内に入力した場合、コマンドメニューが「No commands match "/web-setup"」を表示するか、送信すると「Unknown command: /web-setup」が返される場合、要件が満たされていないため、コマンドは非表示になっています。通常の原因は、claude.ai サブスクリプションではなく API キーまたはサードパーティプロバイダーで認証されていることです。`/login` を実行して claude.ai アカウントでサインインしてください。

Team および Enterprise プランでは、コマンドはデフォルトで非表示になっています。[Quick web setup トグル](/docs/ja/claude-code-on-the-web#github-authentication-options)は、所有者がオンにするまでオフになっています。オフの間は、代わりに[ブラウザから GitHub を接続](#connect-github)してください。

コマンドは他の 2 つのケースでも非表示になります。

* 管理者が組織のクラウドセッションを無効にしました。この場合、`/web-setup` を送信すると [`Cloud sessions are disabled by your organization's policy`](/docs/ja/errors#cloud-sessions-are-disabled-by-your-organizations-policy) が返されます。v2.1.268 より前では、このケースも `Unknown command: /web-setup` を返していました。
* Enterprise 組織が [Zero Data Retention](/docs/ja/zero-data-retention) を有効にしており、これによりクラウドセッションが利用できなくなります。

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  `--cloud` を使用する場合に「Could not create a cloud environment」または「No cloud environment available」が表示される
</h3>

クラウドセッション機能は、環境がない場合、デフォルトのクラウド環境を自動的に作成します。「Could not create a cloud environment」が表示される場合、自動作成に失敗しました。「No cloud environment available」が表示される場合、CLI は自動作成より前のバージョンです。どちらの場合でも、Claude Code CLI で `/web-setup` を実行するか、[claude.ai/code](https://claude.ai/code) の[環境セレクター](/docs/ja/cloud-environments#configure-your-environment)から環境を追加してください。

<h3 id="setup-script-failed">
  セットアップスクリプトが失敗した
</h3>

セットアップスクリプトがゼロ以外のステータスで終了し、セッションの開始がブロックされました。一般的な原因は以下の通りです。

* パッケージインストールが失敗しました。レジストリが[ネットワークアクセスレベル](/docs/ja/cloud-environments#access-levels)にないためです。`Trusted` はほとんどのパッケージマネージャーをカバーしており、`None` はすべてをブロックします。
* スクリプトが新しいクローンに存在しないファイルまたはパスを参照しています。
* ローカルで機能するコマンドが Ubuntu では異なる呼び出しが必要です。

デバッグするには、スクリプトの先頭に `set -x` を追加して、どのコマンドが失敗したかを確認してください。重要でないコマンドの場合は、セッション開始をブロックしないように `|| true` を追加してください。

<h3 id="new-sessions-hang-or-time-out-during-setup">
  新しいセッションがセットアップ中にハングするか、タイムアウトする
</h3>

新しいセッションがセットアップスクリプトステップで停止するか、スクリプトが完了する前に一般的なコンテナエラーで失敗する場合、スクリプトは[環境キャッシュ](/docs/ja/cloud-environments#environment-caching)を構築するための約 5 分間の時間予算を超えている可能性があります。大きな Docker イメージのプル、完全な依存関係ツリーの同期、モデルの重みのダウンロードなどの重い手順は、特に連続して実行される場合、合計を制限を超えることがよくあります。

これを修正するには、スクリプトを調整して、5 分以内に確実に完了するようにしてください。

* `&` と最終的な `wait` を使用して独立したインストールを並列で実行し、順序に実行する代わりに実行します。
* 最大のダウンロードをセットアップスクリプトから[SessionStart hook](/docs/ja/cloud-environments#setup-scripts-vs-sessionstart-hooks)に移動して、バックグラウンドで起動し、セッションが完了中に使用可能になるようにします。
* セットアップスクリプトから長い再試行スリープを削除してください。停止した再試行ループは予算に対してカウントされるためです。

<h3 id="session-keeps-running-after-closing-the-tab">
  タブを閉じた後もセッションが実行され続ける
</h3>

これは仕様です。タブを閉じたり、移動したりしてもセッションは停止しません。Claude が現在のタスクを完了するまでバックグラウンドで実行され続け、その後アイドル状態になります。サイドバーから、セッションをリストから非表示にするために[セッションをアーカイブ](/docs/ja/claude-code-on-the-web#archive-sessions)するか、永続的に削除するために[削除](/docs/ja/claude-code-on-the-web#delete-sessions)できます。

<h2 id="next-steps">
  次のステップ
</h2>

タスクを送信してレビューできるようになったので、これらのページは次に来るものをカバーしています：ターミナルから cloud セッションを開始し、定期的な作業をスケジュールし、Claude に常設の指示を与えます。

* [Claude Code をクラウドで使用する](/docs/ja/claude-code-on-the-web)：完全なリファレンス。セッションをターミナルにテレポートする、セッション共有、auto-fixing pull requests を含みます
* [クラウド環境を設定する](/docs/ja/cloud-environments)：ネットワークアクセスレベル、環境変数、cloud セッション用のセットアップスクリプト
* [Routines](/docs/ja/routines)：スケジュール、API 呼び出し、または GitHub イベントへの応答で作業を自動化します
* [CLAUDE.md](/docs/ja/memory)：すべてのセッションの開始時に読み込まれる永続的な指示とコンテキストを Claude に提供します
* [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) または [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 用の Claude モバイルアプリをインストールして、スマートフォンからセッションを監視します。Claude Code CLI から、`/mobile` は [claude.ai/mobile](https://claude.ai/mobile) 用の QR コードを表示します。これはお使いのスマートフォンに適切なアプリストアを開きます。
