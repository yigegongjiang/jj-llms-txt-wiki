> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# セルフホストされた環境のクイックスタート

> セルフホストされた環境を初めてセットアップします。Claude Code をインストールし、環境を作成し、ランナーを起動し、セッションをルーティングします。

<Note>
  セルフホストされた環境は Team および Enterprise プランでパブリックベータ版です。[利用可能性と制限事項](/docs/ja/self-hosted-environments#availability-and-limitations)は有効化パスをカバーしています。このページは最初のセッションを実行します。詳細は[セルフホストされた環境](/docs/ja/self-hosted-environments)を参照し、本番環境へのデプロイについては[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)を参照してください。
</Note>

[セルフホストされた環境](/docs/ja/self-hosted-environments)は、Claude Code の[クラウドセッション](/docs/ja/claude-code-on-the-web)を、組織が運用するインフラストラクチャ上で実行し、デプロイするランナープロセスによって実行されます。このクイックスタートは最初のセットアップを行います。最小限の構成は、単一ホスト上の 1 つのランナーで 1 つのテストセッションを実行することです。2 つのステップがあります。[環境とランナーを作成し、セッションをルーティングする](#set-up-an-environment-and-runner)、その後[実行中のセッションにターミナルからメッセージを送信する](#send-a-follow-up-message-to-a-running-session)。2 つのサーフェス間を移動します。claude.ai は環境の作成、ステータスの確認、セッションのルーティング用で、ホスト上のターミナルはランナーが行うすべてのことに使用します。

終了時には、[**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments)に環境があり、ランナーが仕事をポーリングしており、セッションがホスト上で実行されています。実際のリポジトリまたは内部システムを接続する前に、[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)を実行してください。これはセキュリティ体制、エグレス制御、git 認証情報、およびオーケストレーションをカバーしています。

<h2 id="prerequisites">
  前提条件
</h2>

<h3 id="organization-and-roles">
  組織とロール
</h3>

claude.ai 側には以下が必要です。

* **セルフホストされた環境を許可**は、[**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments)で[オーナー](/docs/ja/cloud-environments#organization-shared-environments)によってオンにされます。**新規**ボタンはそれがオンになるまで表示されません。ロールを保持していない場合、保持している人が環境を作成してシークレットを渡すことができます。このページのランナーとターミナルのステップには claude.ai ロールは不要です。ステップが管理 UI でステータスをチェックする場合、ランナー自身のログ行が同じシグナルを提供します。
* 組織の[GitHub 接続](/docs/ja/claude-code-on-the-web#github-authentication-options)。開発者がセッションを開始するときにリポジトリを選択できるようにします。

<h3 id="host-and-network">
  ホストとネットワーク
</h3>

ランナーホストには以下が必要です。

* `api.anthropic.com`、`claude.ai` および以下のインストールステップ用のダウンロードホストへのアウトバウンド HTTPS、および git ホストへのクローン用の Linux または macOS ホストまたはコンテナ。[ネットワーク要件テーブル](/docs/ja/self-hosted-environments-deploy#network-requirements)に完全なリストがあります。Windows はランナーホストとしてサポートされていません。代わりに Linux コンテナでランナーを実行してください。セッションは claude.ai のブラウザから開始されるため、開発者ワークステーションは影響を受けません。
* NTP などで実時間に同期されたクロック。クロックが 5 分以上ずれていると認証が失敗します。[トラブルシューティング](/docs/ja/self-hosted-environments-deploy#troubleshooting)を参照してください。

<h3 id="software-on-the-runner-host">
  ランナーホスト上のソフトウェア
</h3>

開始する前にホストにインストールしてください。

* **Claude Code v2.1.224 以降**。[標準インストール方法](/docs/ja/setup)のいずれかを使用します。ランナーは標準 `claude` バイナリの一部であり、以前のバージョンは `self-hosted-runner` サブコマンドを認識しません。ネイティブインストーラーのデフォルト `latest` チャネルは各リリースを公開直後に提供します。`stable` チャネル、Homebrew `claude-code` cask、および安定版 apt、dnf、apk リポジトリは約 1 週間遅れます。フロートが実行する正確なバージョンをピンするには、[特定のバージョンをインストール](/docs/ja/setup#install-a-specific-version)を参照してください。コンテナイメージについては、[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy#build-the-runner-image)の Dockerfile を参照してください。
* **Git 2.24 以降**。デプロイページの一部の git オプションはより新しいバージョンが必要です。[git を設定](/docs/ja/self-hosted-environments-deploy#configure-git)は各フロアを記載しています。

ホストの準備ができていることを確認します。

```bash theme={null}
claude self-hosted-runner --help
```

準備ができたホストはランナーの使用テキストを出力し、`--environment-secret-file` などのフラグをリストします。2.1.224 より古いバージョンでは、コマンドは代わりに一般的な `claude --help` 出力を出力します。`claude update` でアップグレードするか、`latest` チャネルから再インストールしてください。

<h2 id="set-up-an-environment-and-runner">
  環境とランナーをセットアップする
</h2>

Claude Code には、ガイド付きセットアップが含まれています。管理 UI で環境を作成する手順を説明するインタラクティブな Claude Code セッション。保存したシークレットファイルでローカルランナーを起動し、ランナーが登録されたことを確認し、`./runner-setup/CHEAT-SHEET.md` にチートシートを書き込みます。`claude auth login` でサインインしたマシンで実行します。オーナーロールを保持するアカウントを使用します。API キーまたはサードパーティモデルプロバイダーでは利用できません。インタラクティブセッションが不可能なホストでは、代わりに以下の手動ステップを使用してください。[バージョンチェック](#software-on-the-runner-host)が最初に合格したことを確認してください。2.1.224 より古いバージョンでは、このコマンドはガイド付きセットアップの代わりに、単語をプロンプトとして通常の Claude セッションを開始します。ガイド付きセットアップを開始するには、セットアップサブコマンドを実行してプロンプトに従います。

```bash theme={null}
claude self-hosted-runner setup
```

代わりに手動でセットアップするには。

<Steps>
  <Step title="環境を作成する">
    管理設定の[**Cloud environments** ページ](https://claude.ai/admin-settings/cloud-environments)に移動します。**セルフホストされた環境**の下で、**新規**を選択し、環境に名前を付けて、**作成**を選択します。ウィザードの 2 番目のステップで、**環境キーをコピー**を選択して環境シークレットをコピーします。管理 UI はこれを環境キーとしてラベル付けします。claude.ai はシークレットを 1 回表示し、後で取得することはできません。作成から 365 日後に期限切れになります。環境の `ccpool_...` ID は詳細ダイアログに表示されたままです。[トークン検証](/docs/ja/self-hosted-environments-identity)の `aud` チェックおよび[CI からのテストセッションのディスパッチ](/docs/ja/self-hosted-environments-testing#run-the-test-loop)に必要になります。

    シークレットを失った場合またはローテーションが必要な場合は、環境の**設定**タブから新しいシークレットを作成し、新しいシークレットをランナーにロールアウトしてから、古いシークレットを取り消します。取り消されたシークレットを保持するランナーは次の認証済みポーリングに失敗して終了し、`poll auth failed` をログに記録します。オーケストレーターは新しいシークレットで再起動します。
  </Step>

  <Step title="ランナーを起動する">
    シークレットディレクトリを作成します。このステップと次のステップは `/etc/claude` パスに root が必要です。ランナープロセスが読み取ることができるパスは機能するため、別のパスを使用する場合は両方のコマンドと `--environment-secret-file` 値を一緒に調整してください。

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    環境シークレットをファイルに書き込みます。以下のコマンドはターミナルから読み取るため、シークレットはシェル履歴から外れます。コピーした値を貼り付け、Enter キーを押してから Ctrl-D を押します。サブシェルの `umask` はファイルを所有者のみが読み取り可能にします。

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    ベースディレクトリを選択します。以下のランナーコマンドの `<writable-dir>` を、ランナーが書き込みまたは作成できる絶対パスに置き換えます。ランナーはスタートアップ時にディレクトリを作成し、リポジトリをチェックアウトし、その下にセッションごとのディレクトリを作成します。`--base-dir` がない場合は `/workspace` を使用します。これはそのディレクトリが既に存在し、書き込み可能であるか、ランナーを root として起動する場合にのみ機能します。

    ランナーがパスを作成または書き込みできない場合、スタートアップ時にディレクトリを名前付けするエラーで終了し、登録されません。[トラブルシューティング](/docs/ja/self-hosted-environments-deploy#troubleshooting)を参照してください。

    次に、`--environment-secret-file` と `--base-dir` でランナーを起動します。ランナーは環境に登録され、仕事をポーリングし始めます。ランナーが終了した場合、手動で再起動してください。本番環境デプロイメントはランナーをオーケストレーターの下で実行し、通常は再起動ごとに新しいファイルシステムで終了したランナーを再起動します。[事前にウォームアップされたチェックアウトを再利用](/docs/ja/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout)はサポートされている永続ディスクセットアップをカバーしています。

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="ランナーが表示されることを確認する">
    [**Cloud environments** ページ](https://claude.ai/admin-settings/cloud-environments)に戻ります。環境のステータスはランナーが起動してから数秒以内に**ランナーがデプロイされていません**から**正常**に変わります。環境を開いて**アクティビティ**を選択してランナー自体を確認します。
  </Step>

  <Step title="セッションを環境にルーティングする">
    claude.ai/code でセッションを開始し、環境ピッカーから環境を選択します。セルフホストされた環境は Anthropic ホストされた環境と並んで表示されます。ランナーはホストが既に持っている git 認証情報でクローンするため、このホストが既にクローンできるリポジトリまたはパブリックリポジトリを選択してください。本番環境のプライベートリポジトリの認証情報オプションは[git を設定](/docs/ja/self-hosted-environments-deploy#configure-git)にあります。次に利用可能なランナーはキューに入ったセッションを取得し、`Picked up session <session-id>` をアクティブカウントと容量と共にログに記録します。ランナー自身の出力からどのホストがセッションを取得したかを確認できます。[claude.ai/code](https://claude.ai/code)でセッションの動作を監視し、Claude の返信を読みます。セッションがキューに入ったままの場合は、[トラブルシューティング](/docs/ja/self-hosted-environments-deploy#troubleshooting)を参照してください。
  </Step>
</Steps>

ランナーはアクティブセッションが終了すると設計上終了します。[ランナーのライフサイクル](/docs/ja/self-hosted-environments#runner-lifecycle)を参照してください。本番環境では、終了時に再起動するオーケストレーターの下にデプロイしてください。[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)を参照してください。

<h2 id="send-a-follow-up-message-to-a-running-session">
  実行中のセッションにフォローアップメッセージを送信する
</h2>

セッションが環境で実行されたら、`claude auth login` でログインしているマシンの `claude` CLI からフォローアップを送信します。コマンドはセッションを開始したマシンから実行する必要はありません。コマンドは 1 つのメッセージを投稿します。

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

`<session-id>` については、ベアの `session_...` または `cse_...` ID またはセッションの claude.ai/code URL を渡します。成功した送信は `Sent to cloud session.` をセッション ID とビューリンク付きで出力します。受け入れられた ID フォーム、JSON 出力、アカウントとポリシー要件、およびエラーリファレンスは[CLI からフォローアップを送信](/docs/ja/claude-code-on-the-web#send-follow-ups-from-the-cli)にあります。コマンドは Anthropic ホストされたセッションに対して同じように機能するためです。

<h2 id="what’s-next">
  次のステップ
</h2>

* [本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)。デプロイメントを強化し、エグレスを制御し、git 認証情報を設定し、Kubernetes または Compose の下でフロートを実行します。
* [セッションをカスタマイズ](/docs/ja/self-hosted-environments-configuration)。ラッパースクリプト、ライフサイクルフック、オンデマンドランナー、MCP サーバー、および権限。
* [エンドツーエンドをテスト](/docs/ja/self-hosted-environments-testing)。セッションをディスパッチして Claude の返信を読む CI スモークテスト。
