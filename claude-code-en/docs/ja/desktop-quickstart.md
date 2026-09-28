> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# デスクトップアプリを始める

> Claude Code をデスクトップにインストールして、最初のコーディングセッションを開始します

デスクトップアプリは、複数のセッションを並行して実行するために構築されたグラフィカルインターフェース付きの Claude Code を提供します。並列作業を管理するためのサイドバー、統合ターミナルとファイルエディター付きのドラッグアンドドロップレイアウト、ビジュアル diff レビュー、ライブアプリプレビュー、自動マージ機能付きの GitHub PR 監視、スケジュール済みタスクがあります。ターミナルは不要です。

<CardGroup cols={3}>
  <Card title="macOS 用にダウンロード" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Intel と Apple Silicon 向けのユニバーサルビルド
  </Card>

  <Card title="Windows 用にダウンロード" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    x64 プロセッサ向け
  </Card>

  <Card title="Linux 用 Claude を入手（ベータ版）" icon="linux" href="/docs/ja/desktop-linux">
    Ubuntu と Debian 向けの apt または .deb
  </Card>
</CardGroup>

Windows ARM64 の場合は、[ARM64 インストーラー](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)をダウンロードしてください。Linux では apt でインストールします。[Claude Desktop on Linux](/docs/ja/desktop-linux)を参照してください。

<Note>
  Claude Code には [Pro、Max、Team、または Enterprise サブスクリプション](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing)が必要です。
</Note>

このページでは、アプリのインストールと最初のセッションの開始について説明します。既にセットアップが完了している場合は、[Claude Code Desktop を使用する](/docs/ja/desktop)で完全なリファレンスを参照してください。

デスクトップアプリには 3 つのタブがあります。

* **Chat**: ファイルアクセスなしの一般的な会話。claude.ai と同様です。
* **Cowork**: サンドボックス化された仮想マシン内で独自の環境を持つ自律型バックグラウンドエージェント。あなたが他の作業をしている間も独立して実行できます。オンデバイス Cowork セッションはコンピューター上で VM を実行します。リモート Cowork セッションは代わりに Anthropic 管理の VM 上で実行されます。
* **Code**: ローカルファイルへの直接アクセスを備えたインタラクティブなコーディングアシスタント。権限モードに応じて、Claude が提案する各変更を承認するか、Claude が変更を加えた後にレビューします。

Chat と Cowork は [Claude ヘルプセンター](https://support.claude.com/)で説明されています。デスクトップアプリのインストールとデプロイは [Claude Desktop サポート記事](https://support.claude.com/en/collections/16163169-claude-desktop)で説明されています。このページは **Code** タブに焦点を当てています。

<h2 id="install">
  インストール
</h2>

<Steps>
  <Step title="インストールしてサインインする">
    macOS と Windows では、上記のリンクからインストーラーをダウンロードして実行します。Linux では、[Claude Desktop on Linux](/docs/ja/desktop-linux) のインストール手順に従ってください。macOS の Applications フォルダ、Windows の Start メニュー、または Linux のアプリケーションランチャーから Claude を起動し、Anthropic アカウントでサインインします。
  </Step>

  <Step title="Code タブを開く">
    上部中央の **Code** タブをクリックします。Code をクリックするとアップグレードを促すメッセージが表示される場合は、最初に[有料プランにサブスクライブ](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade)する必要があります。オンラインでサインインするよう促すメッセージが表示される場合は、サインインを完了してアプリを再起動してください。403 エラーが表示される場合は、[認証のトラブルシューティング](/docs/ja/desktop#403-or-authentication-errors-in-the-code-tab)を参照してください。
  </Step>
</Steps>

デスクトップアプリには Claude Code が含まれています。Node.js または CLI を別途インストールする必要はありません。ターミナルから `claude` を使用するには、CLI を別途インストールしてください。[CLI を始める](/docs/ja/quickstart)を参照してください。

<h2 id="start-your-first-session">
  最初のセッションを開始する
</h2>

Code タブを開いた状態で、プロジェクトを選択し、Claude に何かをさせます。

<Steps>
  <Step title="環境とフォルダを選択する">
    **Local** を選択して、Claude をマシン上で実行し、ファイルを直接使用します。**Select folder** をクリックして、プロジェクトディレクトリを選択します。

    <Tip>
      よく知っている小さなプロジェクトから始めてください。Claude Code が何ができるかを見る最速の方法です。
    </Tip>

    次のオプションも選択できます。

    * **Cloud**: アプリを閉じても続行するクラウドセッションを実行します。クラウドセッションの動作方法については [Claude Code in the cloud](/docs/ja/claude-code-on-the-web) を参照してください。
    * **SSH**: SSH 経由でリモートマシン（独自のサーバー、クラウド VM、dev コンテナなど）に接続します。Desktop は初回接続時にリモートマシンに Claude Code を自動的にインストールします。
    * **WSL**（Windows）: [WSL 2 distribution](/docs/ja/desktop-wsl) 内でセッションを実行します。Claude Code、ツール、git は Linux 側でネイティブパスで実行されます。
  </Step>

  <Step title="モデルを選択する">
    送信ボタンの横のドロップダウンからモデルを選択します。利用可能なモデルの比較については [models](/docs/ja/model-config#available-models) を参照してください。後で同じドロップダウンからモデルを変更できます。
  </Step>

  <Step title="Claude に何をするかを指示する">
    Claude に何をさせたいかを入力します。

    * `TODO コメントを見つけて修正する`
    * `main 関数のテストを追加する`
    * `このコードベースの手順を含む CLAUDE.md を作成する`

    [session](/docs/ja/desktop#work-in-parallel-with-sessions) は、コードについて Claude との会話です。各セッションは独自のコンテキストと変更を追跡します。
  </Step>

  <Step title="変更を確認して受け入れる">
    次に何が起こるかは、送信ボタンの横のセレクタに表示される [permission mode](/docs/ja/desktop#choose-a-permission-mode) によって異なります。

    * **Auto or Accept edits**: Claude はファイルの変更を適用し、`+12 -1` などのインジケータが表示されるため、diff ビューで確認できます
    * **Manual**: Claude は各変更を提案し、適用する前にあなたの承認を待ちます。変更を受け入れるまでファイルは変更されず、変更を拒否した場合、Claude は代わりにどのように進めたいかを尋ねます

    Manual モードでは、以下が表示されます。

    1. 各ファイルで何が変更されるかを正確に示す [diff view](/docs/ja/desktop#review-changes-with-diff-view)
    2. 各変更を承認または拒否するための Accept/Reject ボタン
    3. Claude があなたのリクエストを処理する際のリアルタイム更新
  </Step>
</Steps>

<h2 id="now-what">
  次は何をしますか？
</h2>

最初の編集が完了しました。Desktop が実行できるすべての機能の完全なリファレンスについては、[Claude Code Desktop を使用する](/docs/ja/desktop)を参照してください。次に試すべきことをいくつか紹介します。

**中断して方向を変える。** Claude をいつでもリダイレクトできます。停止ボタンをクリックして即座に中断するか、修正を入力して **Enter** キーを押して、実行中のアクションを停止せずに送信します。どちらの方法でも、完了を待つ必要がなく、最初からやり直す必要もありません。

**Claude により多くのコンテキストを提供する。** プロンプトボックスに `@filename` と入力して特定のファイルを会話に取り込むか、添付ボタンを使用して画像と PDF を添付するか、ファイルをプロンプトに直接ドラッグアンドドロップします。Claude が持つコンテキストが多いほど、結果は良くなります。[ファイルとコンテキストを追加する](/docs/ja/desktop#add-files-and-context-to-prompts)を参照してください。

**繰り返し可能なタスクにスキルを使用する。** `/` を入力するか、**+** → **Slash commands** をクリックして、[組み込みコマンド](/docs/ja/commands)、[カスタムスキル](/docs/ja/skills)、およびプラグインスキルを参照します。スキルは、コードレビューチェックリストやデプロイメント手順など、必要なときに呼び出すことができる再利用可能なプロンプトです。

**コミット前に変更を確認する。** Claude がファイルを編集した後、`+12 -1` インジケーターが表示されます。それをクリックして [diff ビュー](/docs/ja/desktop#review-changes-with-diff-view)を開き、ファイルごとに変更を確認し、特定の行にコメントを付けます。Claude はコメントを読んで修正します。**Review code** をクリックして、Claude に diff を評価させ、インライン提案を残させます。

**制御の量を調整する。** [権限モード](/docs/ja/desktop#choose-a-permission-mode)は、Claude が承認を求めずに実行できる量を設定します。

* **Auto**: 分類器がバックグラウンドでアクションを確認し、質問する代わりにリスクのあるものをブロックします。
* **Manual**: Claude はファイルを編集またはコマンドを実行する前に確認を求めます。
* **Accept edits**: Claude はファイル編集を自動的に受け入れ、より高速な反復を実現します。
* **Plan**: Claude は任意のファイルを編集せずにアプローチを提案します。これは大規模なリファクタリング前に役立ちます。

**より多くの機能のためにプラグインを追加する。** プロンプトボックスの横にある **+** ボタンをクリックして **Plugins** を選択し、スキル、エージェント、MCP サーバーなどを追加する [プラグイン](/docs/ja/desktop#install-plugins)を参照してインストールします。

**ワークスペースを整理する。** チャット、diff、ターミナル、ファイル、ブラウザーペインを任意のレイアウトにドラッグします。**Ctrl+\`** でターミナルを開いてセッションと並行してコマンドを実行するか、ファイルパスをクリックしてファイルペインで開きます。[ワークスペースを整理する](/docs/ja/desktop#arrange-your-workspace)を参照してください。

**アプリをプレビューする。** Desktop で開発サーバーを実行すると、アプリはブラウザーペインで開きます。ブラウザーペインは [外部サイトを開く](/docs/ja/desktop#browse-external-sites)こともできます。Claude は実行中のアプリを表示し、エンドポイントをテストし、ログを検査し、表示されているものに対して反復できます。[アプリをプレビューする](/docs/ja/desktop#preview-your-app)を参照してください。

**プルリクエストを追跡する。** PR を開いた後、Claude Code は CI チェック結果を監視し、失敗を自動的に修正するか、すべてのチェックが成功したら PR をマージできます。[プルリクエストステータスを監視する](/docs/ja/desktop#monitor-pull-request-status)を参照してください。

**Claude をスケジュールに設定する。** [スケジュール済みタスク](/docs/ja/desktop-scheduled-tasks)を設定して、Claude を定期的に自動実行します。毎朝のコードレビュー、週次の依存関係監査、または接続されたツールから情報を取得するブリーフィングです。

**準備ができたらスケールアップする。** サイドバーから [並列セッション](/docs/ja/desktop#work-in-parallel-with-sessions)を開いて複数のタスクを同時に実行します。各セッションは独自の Git worktree で実行され、[タスクペイン](/docs/ja/desktop#watch-background-tasks)を開いてセッションが実行しているサブエージェントとバックグラウンドコマンドを監視します。[サイドチャット](/docs/ja/desktop#ask-a-side-question-without-derailing-the-session)を開いてメインスレッドを脱線させずに質問を尋ねます。[長時間実行される作業をクラウドに送信](/docs/ja/desktop#run-long-running-tasks-in-the-cloud)して、アプリを閉じても続行するか、タスクが予想より長くかかる場合は [Web または IDE でセッションを続行](/docs/ja/desktop#continue-in-another-surface)します。[GitHub、Slack、Linear などの外部ツールを接続](/docs/ja/desktop#extend-claude-code)して、ワークフローをまとめます。

<h2 id="what’s-next">
  次のステップ
</h2>

* [Claude Code Desktop を使用する](/docs/ja/desktop)：権限モード、並列セッション、diff ビュー、コネクタ、およびエンタープライズ設定
* [CLI から移行する場合](/docs/ja/desktop#coming-from-the-cli)：Desktop と CLI を同じプロジェクトで実行し、機能、フラグの同等物、および Desktop で利用できない機能を比較する
* [トラブルシューティング](/docs/ja/desktop#troubleshooting)：一般的なエラーとセットアップの問題の解決策
* [ベストプラクティス](/docs/ja/best-practices)：効果的なプロンプトを作成し、Claude Code を最大限に活用するためのヒント
* [一般的なワークフロー](/docs/ja/common-workflows)：デバッグ、リファクタリング、テストなどのチュートリアル
