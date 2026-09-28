> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自己ホスト環境

> 自分たちが管理するインフラストラクチャで Claude Code クラウドセッションを実行します。自己ホスト環境をセットアップし、ランナーをデプロイし、セッションを自分たちのコンピュートにルーティングします。

<Note>
  自己ホスト環境は Team および Enterprise プランでパブリックベータ版であり、デフォルトではオフになっています。有効化パスと除外される内容については、[利用可能性と制限事項](#availability-and-limitations)を参照してください。
</Note>

自己ホスト環境は、組織が運用するインフラストラクチャで Claude Code クラウドセッションを実行します。[クラウドセッション](/docs/ja/claude-code-on-the-web)は、開発者のマシン以外の場所で実行されるセッションです。開発者は claude.ai、モバイルおよびデスクトップアプリ、[`claude --cloud`](/docs/ja/claude-code-on-the-web#from-terminal-to-cloud)を使用したターミナル、および[スケジュール済みルーチン](/docs/ja/routines)から開始でき、デフォルトでは Anthropic のインフラストラクチャで実行されます。自己ホスト環境では、これらの同じセッションがネットワーク内で実行され、開発者体験は[利用可能性と制限事項](#availability-and-limitations)の違いとデプロイページの[既知の問題](/docs/ja/self-hosted-environments-deploy#known-issues-and-limitations)を除いて同じです。

チームがクラウドセッションを使用していない場合、ここで設定することはありません。ターミナルまたは IDE のセッションは常に開発者自身のマシンで実行されます。Claude Code を常時稼働しているマシンで実行し、他のデバイスから駆動したい場合は、[リモートコントロール](/docs/ja/remote-control)を使用してください。これは Pro および Max プランでも利用可能です。セットアップの準備ができたら、[クイックスタート](/docs/ja/self-hosted-environments-quickstart)に直接進んでください。セキュリティ体制を最初に確認したい場合は、[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)から始めてください。このページの残りの部分では、自己ホスティングの仕組みと、それを選択する時期について説明します。

<h2 id="how-self-hosted-environments-work">
  自己ホスト環境の仕組み
</h2>

自己ホスティングには 3 つの部分があります。

* **環境**: クラウドセッションを送信できる名前付きの宛先。組織は claude.ai 管理設定で環境を作成し、各環境はランナーのセットをグループ化します。
* **ランナー**: ネットワーク内のホストで実行されるプログラム。ランナーはセッションを実行します。概念は自己ホスト CI ランナーと同じです。
* **セッション**: 開発者が開始した 1 つの Claude Code タスク。

開発者がクラウドセッションを開始すると、セッション開始 UI に環境ピッカーが表示され、Anthropic ホスト環境と組織が作成した環境が一覧表示されます。組織の環境を選択すると、Anthropic のコントロールプレーンはセッションを環境のキューに配置し、ランナーがそれを要求し、開発者が選択したリポジトリをクローンし、ホストで Claude Code プロセスを開始して実行します。ランナーは設定した認証情報を使用して git ホストに認証します。[git の設定](/docs/ja/self-hosted-environments-deploy#configure-git)では、オプションについて説明しています。セッションはネットワーク内からの内部サービスに到達し、内部の場合は同じ方法で git ホストに到達します。Anthropic へのトラフィック、キューポーリング、セッションのイベントストリーム、およびモデル推論は、`api.anthropic.com`への送信 HTTPS であり、セッションが到達できるホストの短いリストは[ネットワーク要件](/docs/ja/self-hosted-environments-deploy#network-requirements)にあります。Anthropic はネットワークに接続することはありません。

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="自己ホスト環境のアーキテクチャ図。ネットワーク境界内にはランナー、その内部の 2 つの Claude Code セッションプロセス、および git ホストが含まれており、api.anthropic.com の外側にはキュー、セッションストリーム、および推論があります。ランナーはキューをポーリングして git ホストに到達し、各セッションプロセスは独自のストリーム、推論、および git 接続を開き、すべての接続はネットワークからの送信であり、受信はありません。" width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="自己ホスト環境のアーキテクチャ図。ネットワーク境界内にはランナー、その内部の 2 つの Claude Code セッションプロセス、および git ホストが含まれており、api.anthropic.com の外側にはキュー、セッションストリーム、および推論があります。ランナーはキューをポーリングして git ホストに到達し、各セッションプロセスは独自のストリーム、推論、および git 接続を開き、すべての接続はネットワークからの送信であり、受信はありません。" width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

図の 2 つの Claude Code ボックスはセッションプロセスです。1 つのランナーが最大で設定容量まで 2 つのセッションを同時に実行しています。ランナーは一度に 1 つの[オーナー](#key-concepts)に対応し、最初のセッションを要求するときにそのオーナーにロックされるため、チェックアウトされたコードはオーナー間で混在しません。[ランナーのライフサイクル](#runner-lifecycle)では、このルールについて説明しています。

ランナーを自分で開始して実行し続けるか、ホストする[オートスケーリングオーケストレーター](/docs/ja/self-hosted-environments-configuration#on-demand-runners)（セッションがキューに入るときにランナーを開始する 2 番目のプロセス）を実行できます。各ランナーは作業が完了すると自動的に終了します。どちらの方法でも、環境を一度セットアップすると、サポートされているすべてのサーフェスのピッカーに表示されます。

<h2 id="availability-and-limitations">
  利用可能性と制限事項
</h2>

ロールアウトを計画する前に、これらを確認してください。

* **プラン**: Team および Enterprise 組織向けのパブリックベータ版。自己ホスト環境はデフォルトではオフになっています。[オーナー](/docs/ja/cloud-environments#organization-shared-environments)が[**クラウド環境**管理ページ](https://claude.ai/admin-settings/cloud-environments)で**自己ホスト環境を許可**をオンにします。これには、組織に対して[ウェブ上の Claude Code](/docs/ja/claude-code-on-the-web)が有効になっている必要があります。
* **ゼロデータ保持**: [ゼロデータ保持](/docs/ja/zero-data-retention)が有効になっている組織では利用できません。
* **モデル推論**: セッションは Anthropic API を使用し、推論は[Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry](/docs/ja/third-party-integrations)、または[LLM ゲートウェイ](/docs/ja/llm-gateway)を通じてルーティングできません。
* **サーフェス**: [claude.ai/code](https://claude.ai/code)、モバイルおよびデスクトップアプリ、[スケジュール済みルーチン](/docs/ja/routines)、およびターミナルから開始されたセッション（[`claude --cloud`](/docs/ja/claude-code-on-the-web#from-terminal-to-cloud)または[`--environment`ディスパッチ](/docs/ja/self-hosted-environments-testing#run-the-test-loop)を使用）は、自己ホスト環境で実行できます。[Claude Tag](https://claude.com/docs/claude-tag/overview)セッションもそれらで実行できますが、Claude はまだそれらのセッションで[アクセスバンドル](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle)を使用できません。[Claude Security](/docs/ja/claude-security)および[Code Review](/docs/ja/code-review)セッションはまだそれらにルーティングされません。これら 2 つのサーフェスのサポートは別途提供されます。
* **リポジトリ**: セッションは GitHub からリポジトリをチェックアウトします。[GitHub 認証オプション](/docs/ja/claude-code-on-the-web#github-authentication-options)を参照してください。
* **請求**: 自己ホスト環境のセッションは、Anthropic ホスト環境のセッションと同じ方法で組織の Claude Code 使用量を消費します。

<h2 id="why-self-host">
  自己ホスティングを選ぶ理由
</h2>

ほとんどのチームは、実行または保守するインフラストラクチャが不要な Anthropic ホスト環境の方が適しています。自己ホスティングは、ネットワーク、ツール、またはコンプライアンス要件により、セッション実行を管理するインフラストラクチャに保つ必要があるチーム向けです。その場合、運用上の所有権を計画してください。ランナーイメージを構築および保守し、フリートを運用し、ネットワークを制御します。

その代わりに、自己ホスティングはネットワークアクセス、カスタムツール、およびコンプライアンス制御を提供します。

* **ネットワークアクセス**: セッションはネットワーク内で実行され、内部サービス、データベース、およびレジストリに到達でき、それらをパブリックインターネットに公開する必要がありません。
* **カスタムツール**: コンパイラ、SDK、および内部 CLI をランナーイメージにプリインストールして、すべてのセッションが構築の準備ができた状態で開始されるようにします。
* **コンプライアンス**: リポジトリのチェックアウトとビルドアーティファクトは、管理するインフラストラクチャに保たれます。セッションコンテンツは、モデル推論のために`api.anthropic.com`に送信されます。

<h2 id="environments-runners-and-sessions">
  環境、ランナー、およびセッション
</h2>

環境は claude.ai 管理設定の**クラウド環境**ページで管理されます。ランナーは、自分のインフラストラクチャで開始および管理するプロセスです。

<h3 id="key-concepts">
  主要な概念
</h3>

これらの用語は自己ホストページ全体に表示されます。

| 用語       | 説明                                                                                                                                      |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| 環境       | claude.ai 設定で作成された、ランナーの名前付きグループ。セッションは個別のランナーではなく、環境にルーティングされます。                                                                       |
| 環境シークレット | ランナーが環境に認証および登録するために使用する単一の共有認証情報。環境作成時に 1 回表示され、管理 UI では**環境キー**とラベル付けされます。                                                            |
| ランナー     | デプロイする長時間実行プロセス。ランナーは環境に登録し、ランナートークンを受け取り、セッションをポーリングします。                                                                               |
| セッション    | claude.ai、モバイルアプリ、またはスケジュール済みルーチンやエージェントなどの別の Anthropic サーフェスから開始された 1 つの Claude Code タスク。各セッションは、ランナーが生成する子 Claude Code プロセスとして実行されます。 |

API フィールド、トークンクレーム、およびメトリック名では、環境は`pool`として表示され、環境 ID は`pool_id`です。[リファレンス](/docs/ja/self-hosted-environments-reference)は 2 つのスペルをマップします。これには、非推奨の`pool`フラグ名も含まれます。

ランナーは一度に 1 つのオーナーに対応します。ランナーが最初に取得するセッションはランナーをそのセッションのオーナーにロックし、ランナーはそのオーナーのセッションのみを実行し、設定容量まで実行します。オーナーが誰であるかは、セッションがどのように開始されたかによって異なります。

* **ユーザーが開始するセッション**: オーナーはそのユーザーのアカウントです。
* **Claude Tag チャネルセッション**: Claude はそれらをユーザーアカウントなしで実行するため、オーナーはセッションを開始した[Claude Tag エージェント](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity)です。そのエージェントが開始するすべてのチャネルセッションは同じオーナーを持ち、Slack メッセージを送信した人です。そのため、`--capacity`が 1 より大きい場合、または正の`--drain-grace-sec`で実行する場合、ランナーはそれにロックされたセッションを提供し、異なる人が開始したセッションを実行します。ユーザーにロックされたランナーはこれらを取得しません。Claude Tag エージェントにロックされたランナーはユーザーのセッションを取得しません。

したがって、最小フリートサイズは、一度にアクティブであると予想されるオーナーの数です。ユーザーと Claude Tag エージェントをカウントします。

<h3 id="session-lifecycle">
  セッションのライフサイクル
</h3>

開発者がセッションを開始して環境を選択すると、Anthropic のコントロールプレーンはセッションを環境のキューに配置します。そこから。

1. 空き容量のあるランナーがセッションを要求し、それに対するリースを保持します。
2. ランナーはリポジトリを作業ディレクトリにクローンし、子 Claude Code プロセスを生成します。
3. 子はランナーがポーリングを続ける間、HTTPS 経由でイベントをストリーミングします。各ポーリングはリースをリフレッシュし、ハートビートとしても機能します。
4. ランナーが約 60 秒間ポーリングを停止すると、サーバーはセッションを別のランナーのキューに戻します。

ランナーは各ポーリングリクエストに 10 秒を与えます。リクエストがタイムアウト、失われた、またはランナーが解析できない応答を取得した場合、ランナーはライブセッションの提供を続け、次のスケジュール済みポーリングを待つ代わりに、1 ～ 2 秒後に再試行します。たとえば、独自のページでポーリングに応答するインターセプティングプロキシは、ランナーが解析できない応答を生成します。別のリクエストが失敗するたびに、ランナーは次の再試行前のギャップを 2 倍にし、最大 20 秒まで、リースの有効期限が近づくたびにギャップを短縮します。

<h3 id="runner-lifecycle">
  ランナーのライフサイクル
</h3>

ランナーが最初に取得するセッションはランナーをそのセッションのオーナーにロックし、ランナーはそのオーナーの最大`--capacity`個の同時セッションを実行します。ランナーがアクティブなセッションを持ち、シャットダウン信号を受け取っていない、または退職時間に達していない間、ランナーはロックされたオーナーのキューに入った作業を要求し続けます。完了後の動作は[`--drain-grace-sec`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)によって異なります。

* **デフォルトの`0`**: ランナーはアクティブなセッションが完了するとすぐに終了し、ポーリングを続けません。デプロイされたオーケストレーター（Kubernetes など）は、新しいディスクで再起動でき、任意のオーナーに対応する準備ができています。
* **正の値**: ランナーは終了する前に、ロックされたオーナーのキューをその秒数ポーリングし続けます。

このライフサイクルは、ランナーがオーナー間でディスク状態を削除する必要なく、各オーナーのチェックアウトされたコードを分離します。

インフラストラクチャがランナーを停止する方法によって、`--retire-at`が必要かどうかが決まります。`SIGTERM`を配信するキルには、フラグは不要です。ランナーは[シャットダウンタイミング](/docs/ja/self-hosted-environments-deploy#shutdown-timing)で説明されているようにドレインするか、[`--defer-shutdown-max-min`](/docs/ja/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal)を設定するときに既に保持しているセッションの提供を続けます。インフラストラクチャが代わりに既知の壁時計時刻でホストを破棄する場合、またはシグナルなしで、またはサンドボックスライフタイムキャップやスポットインスタンス再利用などのドレインに短すぎるグレースピリオドで、`--retire-at <epoch-seconds>`を渡します。その時刻の数分前に設定します。退職時刻に。

1. ランナーは新しい作業を受け取るのを停止します。
2. ランナーは、[`--release-idle-session-min`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)フラグが使用する同じリリースパスを通じて各アクティブセッションをリリースするため、セッションはユーザーが次のメッセージを送信するときに新しいランナーで再開されます。ランナーが各セッションをリリースするタイミングはその状態によって異なります。
   * ランナーはターン中のセッションをそのターンが完了するとすぐにリリースします。
   * ターンが完了し、バックグラウンドタスクが実行されたままの場合、ランナーは最大 60 秒待機してから、まだ実行中でもセッションをリリースします。タスクが完了しているが、その結果を読む後続のターンがまだ実行されていない場合、ランナーはそのターンが完了するまでセッションを保持し、[`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/ja/self-hosted-environments-reference#environment-variable-only-settings)以上待機しません。そのターンが開始されるまで。
3. ランナーはすべてのセッションがリリースされると、0 で終了します。

キルを超えるターンはまだ失われています。[シャットダウンタイミング](/docs/ja/self-hosted-environments-deploy#shutdown-timing)では、マージンのサイジングについて説明しています。`--retire-at`がない場合、シグナルレスホストキルはクラッシュと区別できません。コントロールプレーンはクリーンリリースではなく、失われたワーカーを記録し、セッションは別のランナーにキューに戻されます。

<h3 id="network-paths">
  ネットワークパス
</h3>

ランナーとそのセッションはいくつかの種類の送信接続を行い、Anthropic からのインバウンド接続は不要です。

* **コントロールプレーン**: ランナーは`api.anthropic.com`をポーリングして作業を取得し、セットアップ進捗とエラーイベントを投稿します。すべて送信 HTTPS です。ポーリングはランナーのハートビートとしても機能します。
* **SCM コネクタ**: オプションのオーケストレーター[SCM コネクタ](/docs/ja/self-hosted-environments-reference#scm-connector-flags)トンネルは唯一の WebSocket 接続です。
* **Git**: ランナーは HTTPS または SSH 経由で git ホストからクローンおよびプッシュし、デプロイが提供する認証情報で認証されます。[git の設定](/docs/ja/self-hosted-environments-deploy#configure-git)では、セッションごとにミントされた認証情報や[Anthropic git プロキシ](/docs/ja/self-hosted-environments-deploy#use-the-anthropic-git-proxy)（git を`api.anthropic.com`経由でルーティング）を含むオプションについて説明しています。
* **セッション子**: 子 Claude Code プロセスはセッションのイベントストリームを`api.anthropic.com`に保持し、モデル推論とセッション中に実行される git コマンドの送信呼び出しを行います。完全な送信リストについては、[ネットワーク要件](/docs/ja/self-hosted-environments-deploy#network-requirements)を参照してください。[上記の図](#how-self-hosted-environments-work)はこれらのパスを示しており、オプションの SCM コネクタを除きます。

モデル推論は Anthropic API を使用します。コントロールプレーンは各セッションに API エンドポイントを配信し、セッションは Anthropic が発行したセッションスコープの OAuth トークンで認証するため、自己ホスト環境では推論を[Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry](/docs/ja/third-party-integrations)、または[LLM ゲートウェイ](/docs/ja/llm-gateway)を通じてルーティングできません。

企業の送信プロキシはサポートされています。ランナーとオプションの[オートスケーリングオーケストレーター](/docs/ja/self-hosted-environments-configuration#on-demand-runners)は、[ネットワーク設定](/docs/ja/network-config)で説明されているプロキシと mTLS 環境変数（`HTTPS_PROXY`や`NO_PROXY`など）を尊重します。各プロセスの環境で設定します。変数はコントロールプレーン呼び出し、オーケストレーターの[SCM コネクタ](/docs/ja/self-hosted-environments-reference#scm-connector-flags)WebSocket、および HTTPS リモートの組み込みクローンをカバーし、セッションはランナーからそれらを継承します。セッションストリーミングは HTTPS 経由のサーバー送信イベントを使用するため、パス内のプロキシは応答をバッファリングしてはいけません。

プロキシが`Proxy-Authorization`ヘッダーも必要とする場合、ランナーはプロキシへの各接続にそれを追加できます。[送信プロキシへの認証](/docs/ja/self-hosted-environments-deploy#authenticate-to-an-egress-proxy)を参照してください。

<h2 id="what-stays-on-your-infrastructure">
  インフラストラクチャに保たれるもの
</h2>

リポジトリのチェックアウト、ビルドアーティファクト、シークレット、およびセッションが作成または変更するファイルは、プロビジョニングしたマシンに保たれます。会話自体（プロンプト、応答、ツール結果を含む）は、モデル推論のために`api.anthropic.com`に送信され、Anthropic はセッショントランスクリプトを保存して、別の[サポートされているサーフェス](#availability-and-limitations)からセッションを再開できるようにします。

自己ホスト環境はセッション実行をネットワークに移動させます。コントロールプレーンは Anthropic ホスト型のままです。セッションオーケストレーション、キューイング、および claude.ai インターフェースは Anthropic のインフラストラクチャで実行され続けます。

<h2 id="get-started">
  開始する
</h2>

自己ホスト環境ページは、実行している内容によって整理されています。

* [クイックスタート](/docs/ja/self-hosted-environments-quickstart): Claude Code をインストールし、環境を作成し、ランナーを開始し、最初のセッションをルーティングします。
* [本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy): セキュリティ強化、ネットワーク送信、git 認証情報、Kubernetes および Compose レシピ、既知の問題、およびトラブルシューティング
* [セッションをカスタマイズ](/docs/ja/self-hosted-environments-configuration): セッションごとの認証情報、ライフサイクルフック、オンデマンドランナー、MCP サーバー、および権限のためのラッパースクリプト
* [エンドツーエンドをテスト](/docs/ja/self-hosted-environments-testing): ランナーイメージをプロモーション前に検証する CI スモークテスト
* [リファレンス](/docs/ja/self-hosted-environments-reference): すべての CLI フラグ、環境変数、メトリック、およびヘルスエンドポイント
* [セッション ID を検証](/docs/ja/self-hosted-environments-identity): 独自のサービスからセッショントークンを検証してから、アクセスを許可します。
