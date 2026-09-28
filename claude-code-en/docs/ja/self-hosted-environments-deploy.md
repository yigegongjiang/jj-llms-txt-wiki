> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 本番環境へのセルフホスト環境のデプロイ

> 本番環境でセルフホストランナーを実行する：セキュリティ強化、ネットワーク出力制御、git 認証情報、Kubernetes と Compose レシピ、トラブルシューティング。

<Note>
  セルフホスト環境は Team および Enterprise プランで公開ベータ版です。[利用可能性と制限事項](/docs/ja/self-hosted-environments#availability-and-limitations)では有効化パスについて説明しています。このページではフリートを本番環境で実行する方法について説明します。最初のランナーとセッションについては[クイックスタート](/docs/ja/self-hosted-environments-quickstart)を参照してください。
</Note>

[セルフホスト環境](/docs/ja/self-hosted-environments)は、ネットワーク内にデプロイしたランナー上で Claude Code [クラウドセッション](/docs/ja/claude-code-on-the-web)を実行し、本番環境ではそれらのセッションが環境にセッションをディスパッチできるすべてのユーザーに代わってモデル指向のコードを実行します。このページは、動作している環境を本番環境に移行するオペレーター向けです。デプロイメントを順番に説明します：実際のシステムに接続する前にロックダウンすべき内容、フリートが必要とする出力、セッションが git ホストに認証する方法、デプロイメントレシピ自体、セッションが不正に動作する場合に確認すべき内容です。

<h2 id="harden-your-deployment">
  デプロイメントを強化する
</h2>

セルフホストランナーは、環境にセッションをディスパッチできるすべてのユーザーに代わって、インフラストラクチャ上で任意のモデル指向のコードを実行します。これは Anthropic 組織のすべてのメンバーと、[Claude Tag](https://claude.com/docs/claude-tag/overview) チャネルセッションを環境にルーティングされたスコープで開始できるすべてのユーザーです。本番環境システムに環境を接続する前に、各項目を確認してください：

* **エフェメラルなセッションごとのコンテナ**：各ランナープロセスを、プロセスが終了するときに破棄される新しいコンテナまたは VM で実行します。`--capacity 1` と デフォルトの `--drain-grace-sec 0` を使用して、各コンテナが正確に 1 つのセッションを処理するようにします。容量が高い場合、またはドレイングレースが正の場合、1 つのコンテナが同じ[ロックされたオーナー](/docs/ja/self-hosted-environments#key-concepts)からの複数のセッションを処理します。[ランナーのライフサイクル](/docs/ja/self-hosted-environments#runner-lifecycle)を参照してください。ランナーの再起動間でファイルシステムを再利用しないでください。ただし、意図的な[プリウォーミングされたチェックアウト](#reuse-a-pre-warmed-checkout)セットアップは除きます。また、オーナー間では再利用しないでください。
* **イメージに広範な認証情報を含めない**：長期的な SSH キー、クラウドプロバイダーの認証情報、またはセッションが必要とする以上の権限を付与するパーソナルアクセストークンを含めないでください。セッション中に使用される認証情報（プッシュトークンや API トークン）は、[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts)からセッションごとにミントしてください。初期クローンの場合（ラッパーが実行される前に発生）、[`checkout` ライフサイクルフック](/docs/ja/self-hosted-environments-configuration#checkout)または [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) を使用してください。[git を設定する](#configure-git)を参照してください。
* **セッション実行ホストから環境シークレットを保持する**：環境シークレットはランナーを登録し、環境でキューに入っているセッションを取得できます。固定フリートでは、すべてのランナーホストに存在し、セッションのコードはシークレットファイルを読み取ることができます。[オンデマンドランナー](/docs/ja/self-hosted-environments-configuration#on-demand-runners)を優先してください。ここではシークレットはオーケストレーターホストに留まり、ユーザーコードは実行されず、各ランナーは正確に 1 つのランナーを登録する単一使用の作業指示を受け取ります。固定フリートでは、環境シークレットファイルをすべてのセッションで読み取り可能として扱い、疑わしいセッション侵害後にシークレットをローテーションしてください。
* **デフォルト拒否ネットワーク出力**：すべての環境でランナーとセッションコンテナのアウトバウンドトラフィックをネットワーク境界で制限してください。[デフォルト拒否出力](#default-deny-egress)では、許可する内容と理由について説明しています。
* **最小権限ホスト IAM**：ランナーホストに接続されたコンピュート ID（インスタンスプロファイルやノードサービスアカウントなど）は、ランナー自体が必要とするもののみを付与する必要があります。セッションは、ホストの ID を継承するのではなく、ラッパースクリプトを通じて独自の認証情報を取得する必要があります。
* **セッションからクラウドメタデータエンドポイントをブロックする**：セッションをホスト ID から保持するには、クラウドメタデータエンドポイントへのアクセスをブロックする必要があります。サブネットレベルの出力ポリシーはリンクローカルメタデータトラフィックをインターセプトしないため、コンテナ自体でブロックしてください：

  * ホップリミットが 1 の IMDSv2
  * メタデータ隠蔽を備えた GKE Workload Identity
  * セッションコンテナのネットワーク名前空間で `169.254.169.254` の明示的な拒否

  ブロックはラッパースクリプトとライフサイクルフックにも適用されます。これらはコンテナを共有するためです。トークン交換を[セッション JWT](/docs/ja/self-hosted-environments-identity)で認証し、許可リストに登録された出力を通じて独自のトークンサービスに対して行うか、Amazon EKS の IAM Roles for Service Accounts（IRSA）などのファイルベースの Web ID を使用してください。
* **ランナーごとのファイルシステム分離**：各ランナープロセスは、ホスト上の他のプロセスが読み取りまたは書き込みできない独自の作業ディレクトリを取得します。`--hooks-dir`、ラッパースクリプト、ホストの `~/.claude/` をセッションに対して読み取り専用にします。イメージに組み込むか、読み取り専用でマウントしてください。
* **ディスパッチには環境ごとのアクセス制御がない**：Anthropic 組織のすべてのメンバーは、セッションを任意の環境にディスパッチできます。オーナーが [Claude Tag チャネルを環境にルーティング](/docs/ja/cloud-environments#set-the-environment-a-claude-tag-channel-uses)する場合、[Claude Tag アクセス設定](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude)が許可するすべてのユーザーがそこで実行されるチャネルセッションを開始できます。デフォルトでは、Claude アカウントの有無に関わらず、接続された Slack ワークスペース内のすべてのユーザーです。すべてのランナーホストを、それにディスパッチできるすべてのユーザーがコード実行に到達可能として扱い、ランナーホストには、それらのユーザーすべてが読み取ることを許可されているデータと認証情報のみを配置してください。[`--lock-to-account`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)は、特定のホストが実行するアカウントのセッションを制限しますが、環境にディスパッチできるユーザーを絞り込みません。セルフホスト環境を唯一のピッカーオプションにするには、[オーナー](/docs/ja/cloud-environments#organization-shared-environments)が [**クラウド環境**ページ](https://claude.ai/admin-settings/cloud-environments)から組織全体の Anthropic ホスト環境を非表示にできます。
* **リポジトリ設定ガードを適用する**：[`--confine-repo-settings`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)でガードモードを選択してください。デフォルトの `warn` は違反をログに記録してもセッションを生成し、`enforce` はセッションを拒否し、`off` はスキャンを無効にします。ランナーは各リポジトリのコミットされた設定をスキャンします：

  * そのセッション独自のワークスペースの外で解決される付与：`additionalDirectories` エントリ、`permissions.allow` の `Edit`、`Write`、または `NotebookEdit` ルール、または `sandbox.filesystem.allowWrite` または `allowRead` エントリ
  * 空でない `env` ブロック
  * `sandbox.enabled: false` などのオペレーター姿勢オーバーライド

  ガードは [`--trust-workspace`](/docs/ja/self-hosted-environments-reference#runner-cli-flags) に関係なく実行され、リポジトリフック、`.mcp.json`、または Bash ルールはカバーしません。[権限とツール承認](/docs/ja/self-hosted-environments-configuration#permissions-and-tool-approval)では、これらの付与がどこに属するかについて説明しています。

<Note>
  組織の IP 許可リストはデフォルトではセルフホストランナートラフィックをカバーしません。ランナーまたはセッショントラフィックのネットワーク制御として依存しないでください。代わりに、独自のネットワーク境界でデフォルト拒否出力を適用し、組織の IP 許可リスト適用が必要な場合は Anthropic アカウントチームに連絡してください。
</Note>

<h2 id="network-requirements">
  ネットワーク要件
</h2>

ランナーとそれが生成するセッション子は、以下のホストへのアウトバウンド接続を行います。セッションコンテナの出力をこれらのホストとセッションが到達する必要がある特定の内部サービスに制限してください。[デフォルト拒否出力](#default-deny-egress)では、方法と理由について説明しています。

これらのホストは常に必須です：

| ホスト                                                   | ポート                      | 用途                                                                                                                                                                                                                                                                                |
| :---------------------------------------------------- | :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                                   | 443、HTTPS；SCM コネクタのみ WSS | ランナーコントロールプレーンとセッションストリーミング、モデル推論、機能フラグ、製品分析、[JWKS](/docs/ja/self-hosted-environments-identity) キーフェッチ、コミット署名、`--use-anthropic-git-proxy` が設定されている場合の git プロキシ、`--scm-connector-host` が設定されている場合のオーケストレーターの [SCM コネクタ](/docs/ja/self-hosted-environments-reference#scm-connector-flags)トンネル |
| `github.com` またはお客様の GitHub Enterprise ホストなどの git ホスト | 443 または 22               | リポジトリのクローンとプッシュ。ランナーが `--use-anthropic-git-proxy` を使用する場合は不要です。これは git トラフィックを `api.anthropic.com` を通じてルーティングします。                                                                                                                                                                 |

これらのホストが必要かどうかは、設定によって異なります：

| ホスト                                  | ポート | 必須の場合                                                                                                                                                                 |
| :----------------------------------- | :-- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `downloads.claude.ai`                | 443 | インストール時に、ネイティブインストーラーでホストに Claude Code をインストールまたは更新する場合。`install.sh` スクリプト自体は `claude.ai` から提供されます。セッション実行時には、セッションが公式 Anthropic マーケットプレイスからプラグインをインストールする場合のみです。     |
| `storage.googleapis.com`             | 443 | セッション実行時に、`/plugin` に表示されるプラグインインストール数とメタデータの場合。                                                                                                                      |
| `code.claude.com` および `claude.com`   | 443 | 組み込みの claude-code-guide エージェントによるドキュメント検索と、セッション中の事前承認された WebFetch リクエストの場合。これらのホストをブロックするとドキュメント検索のみに影響します。                                                          |
| `*.frame.claudeusercontent.com`      | 443 | 組織内のセッションで [Artifact ツール](/docs/ja/artifacts#availability)が利用可能な場合のみ。デフォルトはプランによって異なり、そこの利用可能性テーブルに従います。ランナーで `CLAUDE_CODE_DISABLE_ARTIFACT=1` を設定して、組織設定に関係なくツールを無効に保ちます。 |
| `registry.npmjs.org`                 | 443 | セッションがプラグインをインストールする場合、npm ソースプラグインパッケージのフェッチとプラグインの Node.js 依存関係のインストール、または `npx` で起動された MCP サーバーが実行される場合。                                                           |
| `http-intake.logs.us5.datadoghq.com` | 443 | Anthropic 運用メトリクス。`CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` が設定されている場合のみ。セルフホスト環境ではデフォルトでオフです。                                                                              |
| `browser-intake-us5-datadoghq.com`   | 443 | Anthropic エラーレポートアップロード。セッションのアカウントで[エラーレポート](/docs/ja/data-usage#telemetry-services)が有効な場合のみ送信されます。`DISABLE_ERROR_REPORTING=1` または `DISABLE_TELEMETRY=1` で抑制されます。         |

ランナーは `statsig.anthropic.com`、`*.sentry.io`、`claude.ai`、または `platform.claude.com` に到達しません。これらのホストは古いエンタープライズネットワークチェックリストに表示されますが、ランナーまたはセッショントラフィックのために許可リストに登録する必要はありません：機能フラグフェッチは `api.anthropic.com` に移動し、ランナーはインタラクティブ OAuth ではなく環境シークレットで認証します。 2 つのホスト側フローは `claude.ai` に到達するため、出力を許可するホストから実行してください。セッションコンテナ出力を広げるのではなく：ワンラインインストーラーはインストール時に `claude.ai` から `install.sh` をフェッチし、インタラクティブな `claude auth login`（[ガイド付きセットアップ](/docs/ja/self-hosted-environments-quickstart#set-up-an-environment-and-runner)、`doctor` の署名入りモード、[CI ディスパッチ](/docs/ja/self-hosted-environments-testing#authenticate-from-ci)が使用）は `claude.ai`、`claude.com`、`platform.claude.com` を通じてサインインします。`mcp-proxy.anthropic.com` も必須ではありません：セルフホストセッションはそれを使用せず、組織の claude.ai コネクタをセッションに配信する場合（組織で有効な場合）、`api.anthropic.com` を通じてルーティングされます。[MCP サーバー](/docs/ja/self-hosted-environments-configuration#mcp-servers)を参照してください。

<h3 id="default-deny-egress">
  デフォルト拒否出力
</h3>

ランナーとセッションコンテナを、アウトバウンドトラフィックが[ネットワーク要件テーブル](#network-requirements)のホスト、git ホスト、セッションが到達する必要がある特定の内部サービスに制限されるネットワークセグメントまたは名前空間にデプロイします。製品はこれを検証または適用できないため、すべての環境でネットワーク境界に適用してください。セッションコードはモデル指向であり、任意のホストへの接続を試みることができます。ネットワークレイヤーでのデフォルト拒否出力は、これらの試みが到達できる場所を制限します。これは権限モードに関係なく適用されます：デフォルトの事前承認ツールセットには既に `Bash` が含まれているため、[自動モード](/docs/ja/self-hosted-environments-configuration#permissions-and-tool-approval)がなくてもシェル出力はプロンプトなしで実行されます。

各セッションが出力するテレメトリとそれをオフにする方法の詳細については、[テレメトリ](/docs/ja/self-hosted-environments-reference#telemetry)を参照してください。

<h3 id="authenticate-to-an-egress-proxy">
  出力プロキシに認証する
</h3>

一部の企業出力プロキシは、すべての接続で `Proxy-Authorization` ヘッダーを必要とします。そのヘッダーのトークンは、`HTTPS_PROXY` に設定するプロキシ URL に書き込むには速すぎるペースでローテーションすることが多いです。通常どおり `HTTPS_PROXY` または `HTTP_PROXY` をプロキシの URL に設定してから、`--proxy-authorization-command` または `--proxy-authorization-file` を設定して、ランナーにヘッダー値を読み取る場所を指示してください。両方のフラグには Claude Code v2.1.238 以降が必要です。

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  `Proxy-Authorization` 値の出所を選択する
</h4>

`Proxy-Authorization` トークンを生成する方法に一致するフラグを選択してください：

* **[`--proxy-authorization-command <command>`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)**：オンデマンドで生成するトークンの場合はこれを選択してください。ランナーはシェルコマンドを実行し、トリミングされた stdout をヘッダー値として使用します。例えば `Bearer <token>`。
* **[`--proxy-authorization-file <path>`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)**：別のプロセスがローテーションするトークンの場合はこれを選択してください。ランナーはファイルを読み取り、トリミングされた内容をヘッダー値として使用します。

<h4 id="configurations-the-runner-refuses-to-start-with">
  ランナーが起動を拒否する設定
</h4>

各フラグには環境変数形式もあり、[ランナー CLI フラグリファレンス](/docs/ja/self-hosted-environments-reference#runner-cli-flags)に記載されています。ランナーがプロキシまたはコントロールプレーンに接続する前に、フラグとその変数をチェックし、3 つの場合に起動を拒否します：

* **両方のフラグが設定されている**：1 つのフラグと他のフラグの環境変数は、両方を設定することとしてカウントされます。
* **プロキシ URL がない**：`HTTPS_PROXY` も `HTTP_PROXY` も `http://` または `https://` URL を保持していません。ランナーは両方の変数を大文字または小文字で読み取り、`ALL_PROXY` を参照しません。
* **フラグがオーケストレーターサブコマンドに渡される**：`self-hosted-runner orchestrator` はフラグまたはそれらの環境変数を受け入れません。代わりに、オーケストレーターが開始する各ランナーにフラグを渡してください。

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  プロキシ認可フラグが設定されている間にランナーが変更する内容
</h4>

いずれかのフラグが設定されている場合、ランナーは独自のリスナーを開始し、プロキシトラフィックをそれ自体、ライフサイクルフック、セッションからそのリスナーを通じて送信します。リスナーはプロキシへの途中で `Proxy-Authorization` ヘッダーを追加します。

* **リスナー**：リスナーは `127.0.0.1` 上のフォワードプロキシです。ランナーはコントロールプレーンに登録する前にリスナーを開始し、リスナーが開始できない場合は起動時に終了します。
* **プロキシ変数**：ランナーは、設定した `HTTPS_PROXY` と `HTTP_PROXY` のいずれかをリスナーを指すように書き直します。その書き直された値はランナー自体、ライフサイクルフック、実行するすべてのセッションに到達します。
* **トークンローテーション**：ローテーションされたトークンは再起動なしで有効になります。リスナーがプロキシに開く各接続について、ランナーはコマンドを実行するか、ファイルを再度読み取り、結果をヘッダーとして追加します。
* **セッション環境**：セッションはリスナーを通じてのみプロキシに到達します。各セッションの環境で、ランナーは `ALL_PROXY` を削除し、設定しなかった `HTTPS_PROXY` または `HTTP_PROXY` のスペルを削除し、`NO_PROXY` をランナー独自の値にピンします。
* **ログ**：ランナーはヘッダー値をログに記録しません。

<h2 id="configure-git">
  git を設定する
</h2>

ランナーはリポジトリチェックアウトを管理しますが、デフォルトでは git ID または認証情報を設定しません。ランナーのイメージとプロセス環境を制御するため、git 設定を制御します。2 つのアプローチのいずれかを選択してください：

* **ランナーに git を設定させる**：`--configure-git` でランナーを開始して、Anthropic ホストセッションが使用する同じ ID とコミット署名設定を書き込ませます
* **イメージに git 設定を含める**：ID とプッシュ認証情報を自分で設定します。例えば、独自のボット ID でコミットするため

ランナーホストの Git バージョンフロア：[`--configure-git`](#let-the-runner-configure-git) SSH コミット署名には Git 2.34 以降が必要です。[`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) には 2.32 以降が必要です。[`--push-outcome-on-release`](/docs/ja/self-hosted-environments-reference#runner-cli-flags) でプッシュされたブランチからセッションを再開するには 2.29 以降が必要です。3 つすべてを省略して git ID を自分で管理する場合は、Git 2.24 で十分です。

<h3 id="let-the-runner-configure-git">
  ランナーに git を設定させる
</h3>

`--configure-git` でランナーを開始するか、`SELF_HOSTED_RUNNER_CONFIGURE_GIT=1` を設定して、起動時にグローバル git 設定を書き込ませます：

* `user.name = Claude` および `user.email = noreply@anthropic.com`。Anthropic ホストセッションと一致します
* SSH 形式のコミットとタグ署名。ランナー管理のシムを通じてルーティングされ、セッション独自の認証情報を使用して Anthropic の署名サービスを通じて各コミットに署名します。署名は GitHub で Anthropic の公開 SSH 署名キーに対して検証可能です。
* `push.negotiate = true`。git がプッシュをパックする前に git ホストが既に持っているコミットを尋ねます。Claude Code v2.1.257 以降が必要です。
* `core.hooksPath` はランナー管理のフックディレクトリを指します。その `commit-msg` および `prepare-commit-msg` フックは、各コミットにセッションの作成者の `Co-authored-by:` トレーラーを追加します。[`CCR_SESSION_ACCOUNT_EMAIL`](/docs/ja/self-hosted-environments-configuration#wrapper-scripts) のメールから構築され、その変数が設定されていない場合は省略されます。イメージが既に `core.hooksPath` を設定している場合、ランナーは設定を保持し、これらのフックのインストールをスキップし、`[runner:git]` 警告を出力します。

コミット署名には git 2.34 以降が必要です。ランナーは起動時にチェックし、git が古い場合はエラーで終了します。このフラグはプッシュ認証情報を設定しません。これはイメージで提供する必要があります。

<h3 id="ship-git-config-in-your-image">
  イメージに git 設定を含める
</h3>

git ID はすべてのコミットに必須です。Dockerfile でシステム全体に設定して、ランナープロセスが実行されるユーザーに関係なく設定が適用されるようにします：

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

ID がない場合、`git commit` は `Please tell me who you are` で失敗し、セッションは進行できません。代わりに独自のボット ID を使用できます。ランナーはこれらの値をオーバーライドしません。

長期的または広くスコープされたプッシュ認証情報を共有ランナーイメージにベイクしないでください：イメージの認証情報は、イメージが実行するすべてのセッションで利用可能です。誰が開始したかに関係なく。代わりに、セッション JWT からデコードされたセッション作成者の ID を使用して、[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts)からセッションごとに短期的で最小スコープのトークンをミントしてください。エフェメラルなセッションごとのコンテナと組み合わせます。これには `--capacity 1` が必要です。認証情報がセッションを超えて存続しないようにします。[強化セクション](#harden-your-deployment)を参照してください。

イメージレベルでプッシュ認証情報を設定する必要がある場合（例えば、読み取り専用デプロイキーの場合）、git ホストが許可する限りスコープを厳しくしてください：

* `url.<base>.insteadOf` 書き直しで 1 つのリポジトリに制限された SSH デプロイキー
* 最小限のスコープトークンを返す `credential.helper`
* 狭くスコープされたキーを指す `GIT_SSH_COMMAND`

設定するメカニズムは、ランナーの組み込みクローンとフェッチがプロンプトを無効にするため、プロンプトなしで動作する必要があります。git、SSH、Git Credential Manager が表示するプロンプト：

* ランナーは `GIT_TERMINAL_PROMPT=0` を設定するため、git はユーザー名またはパスワードを要求しません。
* ランナーは `BatchMode=yes` で SSH を実行します。`GIT_SSH_COMMAND` を設定した場合は追加されます。SSH はパスフレーズまたはホスト確認を要求しません。
* ランナーは `GCM_INTERACTIVE=never` を設定するため、Git Credential Manager はサインインダイアログを開きません。
* ランナーは `core.askPass` をクリアするため、askpass ヘルパーを使用する場合は、`GIT_ASKPASS` 環境変数を通じて設定してください。

git ホストが認証情報を拒否するか、認証情報を設定しなかった場合、ランナーは数回再試行してから失敗します。リポジトリがセッションがプッシュする結果のリポジトリである場合、ランナーはリポジトリ準備に失敗します。セッションが読み取り専用のリポジトリの場合、[トラブルシューティング](#troubleshooting)はランナーがスキップする場合をカバーしています。ランナーはこれらの設定をセッション環境に渡しません。

チェックアウトディレクトリがランナープロセスと異なる uid で所有されている場合、git は操作を拒否します。`safe.directory` を追加してください：

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Anthropic git プロキシを使用する
</h3>

`--use-anthropic-git-proxy` でランナーを開始するか、`CLAUDE_RUNNER_USE_GIT_PROXY=1` を設定して、セッション独自の短期トークンで認証された Anthropic の git プロキシを通じてクローンさせます。通常のユーザーセッションの場合、プロキシはセッション作成者用に保存された GitHub または GitHub Enterprise OAuth トークンを使用します。ボットおよびエージェントセッションの場合、組織の GitHub App インストールトークンを使用します。どちらの場合でも、ランナーイメージは git 認証情報をまったく必要としません：SSH キーなし、認証情報ヘルパーなし、`.netrc` なし。これは Anthropic ホスト環境が使用する同じ認証パスです。

プロキシは `--capacity 1` を必要とします。プロキシ URL はセッションごとであり、git 2.32 以降が必要です。古い git はプロキシがセッションを相互に分離するために使用する設定メカニズムを無視するためです。ランナーは要件のいずれかが満たされない場合、起動を拒否します。プロキシは Anthropic 側からフェッチするため、git ホストは Anthropic インフラストラクチャから到達可能である必要があります。これは Anthropic ホストセッションと同じ要件です。ネットワーク内でのみルーティング可能な git ホストの場合は、代わりに [`checkout` ライフサイクルフック](/docs/ja/self-hosted-environments-configuration#checkout)を使用してください。各ランナープロセスは一度に 1 つのセッションを処理するため、並列処理のためにより多くのレプリカを実行してください。プロキシが有効な場合、`--git-host-rewrite` と `--git-ssh-rewrite` は効果がありません：プロキシ URL は git ホストではなく `api.anthropic.com` を指します。

ランナーは登録時に Anthropic にオプトインを報告し、起動時に `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` を出力します。オプトインの報告には Claude Code v2.1.267 以降が必要です。それより前のバージョンはフラグを受け入れますが、報告しないか、その行を出力しません。その後、オプトインランナー上の各セッションは、Anthropic 管理の git またはセッションごとのプロキシ URL のいずれかを使用します。セッションがセッションごとのプロキシ URL を使用する場合、ランナーは 1 つの `[runner:warn]` 行をログに記録します。

<h3 id="rewrite-git-urls-for-private-networks">
  プライベートネットワークの git URL を書き直す
</h3>

リポジトリ URL はコントロールプレーンから HTTPS として到達します。git ホストのホスト名を使用します。GitHub Enterprise の場合、Claude Code 管理設定で [GitHub Enterprise 統合](/docs/ja/github-enterprise-server)用に設定したホスト名です。2 つの繰り返し可能なフラグはクローン前にこれらの URL を書き直します：

* `--git-host-rewrite <from>=<to>`：スプリットホライズン DNS の場合。Anthropic は外部ホスト名を通じて git ホストに到達しますが、ランナーは内部ホスト名を使用する必要があります
* `--git-ssh-rewrite <host>`：SSH のみを受け入れる git ホストの場合。`https://<host>/owner/repo` を `git@<host>:owner/repo` に書き直します

ホスト書き直しが最初に実行されるため、両方が必要な場合は `--git-ssh-rewrite` に内部ホスト名をリストします。チェックアウトを完全に制御するには、[`checkout` ライフサイクルフック](/docs/ja/self-hosted-environments-configuration#checkout)を使用してください。

<h2 id="build-the-runner-image">
  ランナーイメージをビルドする
</h2>

Anthropic は事前構築されたランナーイメージを公開していません。`claude` バイナリの周りに独自のイメージをビルドし、リポジトリが必要とするツールチェーンをレイヤーします：言語ランタイム、コンパイラ、パッケージマネージャー、[MCP](/docs/ja/mcp) サイドカー。

以下のレシピは `--capacity 4` を使用するため、1 つのコンテナは同じロックされたオーナーからの最大 4 つの同時セッションを処理します。これは[強化セクション](#harden-your-deployment)のセッションごとのコンテナ分離を提供しません：本番環境システムに環境を接続する前に、レシピを `--capacity 1` で実行してセッションごとに 1 つのコンテナを使用するか、[オンデマンドランナー](/docs/ja/self-hosted-environments-configuration#on-demand-runners)を使用してください。オンデマンドランナーは、環境シークレットをセッション実行ホストに置かないようにもします。

このDockerfile は最小限の出発点です：

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

ノードが ARM の場合は `linux-x64` を `linux-arm64` に、Alpine などの musl ベースのイメージの場合は `linux-x64-musl` または `linux-arm64-musl` に置き換えます。[Alpine Linux セットアップ](/docs/ja/setup#alpine-linux-and-musl-based-distributions)を参照して、musl イメージが必要とする追加パッケージについて確認してください。URL は標準 Claude Code リリースロケーションであるため、[バイナリ整合性とコード署名](/docs/ja/setup#binary-integrity-and-code-signing)で説明されているように、ダウンロードされたバイナリをリリースの署名されたマニフェストに対して検証できます。Claude Code バージョン 2.1.224 以降でイメージをビルドしてから、レジストリにプッシュし、以下のレシピで参照してください：

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  セッション用に CPU とメモリをサイズ設定する
</h2>

ランナープロセス自体ではなく、ランナーが実行するセッション用にランナーのコンテナまたはホストをサイズ設定してください。ランナー自体は作業をポーリングし、各セッションのチェックアウトを準備し、[ライフサイクルフック](/docs/ja/self-hosted-environments-configuration#lifecycle-hooks)を実行し、セッションプロセスを開始および監視します。負荷はセッションから発生します。各セッションは Claude Code プロセスと、ビルド、テストスイート、パッケージインストール、[MCP サーバー](/docs/ja/mcp)など、それが開始するものです。

1 つのセッションについて、以下の値から開始してください。Kubernetes のリクエストと制限、またはプラットフォームの同等の値として記載されており、要件ではなく開始点として扱ってください。

* **メモリ**: 各 4 GiB のリクエストと制限。これは Claude Code の[システム要件](/docs/ja/setup#system-requirements)の 4 GB 最小値を満たします。2 つを等しく保つことで、スケジューラーはコンテナの全メモリを考慮に入れます。コンテナがメモリ制限に達すると、カーネルはその内部のプロセスを強制終了し、セッションをタスクの途中で終了させる可能性があります。
* **CPU**: 2 CPU のリクエストと 4 CPU の制限。セッションはビルド中にリクエストを超えてバースト可能です。カーネルはコンテナを CPU 制限でスロットルします。その制限でプロセスを強制終了するのではなく、セッションは制限で実行速度が低下しますが、実行を続けます。

Kubernetes コンテナスペックで、以下の `resources` ブロックを使用してこれらの開始値を設定します。

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

ビルドとテストは通常、セッションの負荷の最大かつ最も変動する部分です。リポジトリの代表的なビルドを実行し、ピーク CPU とメモリを測定し、そのピークの上に Claude Code プロセスの余地を残さない開始値を引き上げてください。

ランナーは `--capacity` を使用して、一度に実行するセッション数をキャップします。CPU またはメモリをセッション間で分割しないため、ランナー上のセッションはコンテナの CPU とメモリを共有します。1 つのセッションのシェアをキャップするには、[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts)から制限を適用してください。したがって、1 つのコンテナに与える内容は、一度に何個のセッションを提供するかによって異なります。

* **ランナーあたり 1 つのセッション**: 各コンテナに 1 つのセッションの値を与えます。[強化セクション](#harden-your-deployment)が推奨する `--capacity 1` でこのサイズ設定を使用し、[オンデマンドランナー](/docs/ja/self-hosted-environments-configuration#on-demand-runners)の場合、[`spawn-runner` フック](/docs/ja/self-hosted-environments-configuration#the-spawn-runner-hook)が送信するワークロード（Kubernetes Job のポッドテンプレートなど）に値を設定します。
* **ランナーあたり複数のセッション**: `--capacity` が 1 より上の場合、1 つのセッションの値に容量を掛けます。その容量まで多くのセッションがコンテナ内で同時に実行できるためです。[Kubernetes](#kubernetes) と [Docker Compose](#docker-compose) レシピは CPU またはメモリ制限なしで `--capacity 4` を実行するため、実行する容量のサイズ設定された制限を追加してください。

<h2 id="kubernetes">
  Kubernetes
</h2>

ランナーはデフォルトでポート 8080 で `GET /healthz` を提供し、`--health-port` で設定可能です。Kubernetes プローブは追加セットアップなしで動作します。エンドポイントはプロセスが生きている限り `200` を返すため、以下のプローブはデッドプロセスを検出し、スタックしたプロセスは検出しません。ランナーがポーリングを停止したことをキャッチするには、[`/metrics`](/docs/ja/self-hosted-environments-reference#prometheus-metrics)から `last_poll_age_seconds` シリーズでアラートを出してください。以下の Deployment は Kubernetes Secret から環境シークレットをマウントし、liveness および readiness プローブを `/healthz` に指します。90 秒の終了猶予期間を設定します。[シャットダウンタイミング](#shutdown-timing)を参照して、猶予期間が重要な理由を確認してください。

マニフェストはランナーコンテナに CPU またはメモリ `resources` を設定しません。実行する容量のサイズ設定されたブロックを追加してください。[セッションの CPU とメモリをサイズ設定する](#size-cpu-and-memory-for-sessions)で説明されています。

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

上記の Deployment は `claude-runners` 名前空間に存在します。最初に名前空間を作成してください：

```bash theme={null}
kubectl create namespace claude-runners
```

管理 UI の [**環境キーをコピー**ステップ](/docs/ja/self-hosted-environments-quickstart#set-up-an-environment-and-runner)でコピーした値を保持するローカルファイルからバッキング Secret を作成してください。シークレットはシェル履歴に表示されません。`(umask 077 && cat > ./environment-secret)` を実行し、シークレットを貼り付け、Enter キーを押してから Ctrl-D を押してください。次に Secret を作成してファイルを削除してください：

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

以下の Compose サービスは、ランナーが終了するたびに再起動します。これはクラッシュと通常のドレイン後の終了の両方をカバーします。Docker 再起動ポリシーは、書き込み可能なレイヤーを保持して同じコンテナを再起動するため、ランナーは[強化姿勢](#harden-your-deployment)が推奨する新しいファイルシステムではなく、再利用されたファイルシステムで戻ります。このレシピを評価に使用し、本番環境ではコンテナを実行ごとに再作成するか、そうするオーケストレーターを使用してください。

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  シャットダウンタイミング
</h2>

`SIGTERM` では、ランナーは新しい作業を受け取るのを停止し、[`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)を設定しない限り、最大 `--drain-wait-sec`（デフォルトはゼロ）待機して、進行中のターンが終了するのを待ちます。各セッションのプロセスツリーを終了し、[`post-session` ライフサイクルフック](/docs/ja/self-hosted-environments-configuration#post-session)を実行します。そのプロセスツリーには、Claude がセッションで実行していたコマンドが含まれます。

完全なドレインパスには、最大 `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec` が必要です。プロセスクリーンアップの固定オーバーヘッド 15 秒と、[`--push-outcome-on-release`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)が設定されている場合はさらに 30 秒が必要です。デフォルトでは 80 秒です。ランナーは起動時に合計をログに記録します。セッションはこの 1 つの予算の下で並列にドレインされるため、合計は `--capacity` で増加しません。

デフォルトの `--drain-wait-sec 0` では、ローリング再起動は進行中のターンを中断します。各セッションは別のランナーで再開され、[既知の問題](#additional-limitations)で説明されているように、プッシュされていない作業が失われます。`--drain-wait-sec` を設定し、猶予期間を一致させて、ターンが最初に終了するようにします。

その全体のパス全体を通じて、ランナーはゼロ容量でコントロールプレーンにハートビートを送信し続けるため、セッションリースは期限切れにならず、`post-session` フックがコミットされていない作業を書き出している間に別のランナーに再キューイングされません。ハートビートはランナーが登録解除される直前に停止します。

ランナーが起動時にログに記録する合計の少なくとも前に、ホストがそれを停止する前に与えてください。設定する場所は、ホストがどのように停止するかによって異なります：

* **`SIGTERM` 猶予期間付き**：Kubernetes で `terminationGracePeriodSeconds` を設定し、Docker Compose で `stop_grace_period` を設定するか、オーケストレーターの同等物をその合計の少なくとも設定してください。Kubernetes のデフォルト 30 秒はランナーのドレインパスより短いため、Kubernetes はランナーがドレインを終了する前にポッドを停止します。
* **[`--retire-at`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)付き**：リタイア時間とホストの停止時間の間のマージンを、典型的なターン、[ランナーのライフサイクル](/docs/ja/self-hosted-environments#runner-lifecycle)が説明するバックグラウンドタスク保持、およびその同じ合計をカバーするようにサイズ設定してください。各起動時にリタイア時間を計算します。例えば `date +%s` にランナーの意図された生涯を加えます。
* **[`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)付き**：ドレインパス合計に 2 つの部分を追加してください。最初は設定する分です。 2 番目は[最初のシグナルを超えてドレインを遅延させる](#defer-the-drain-past-the-first-signal)が説明するリリース後の猶予です。デフォルトでは 75 秒です。フラグが設定されている場合、ランナーは起動時に組み合わせた図も出力します。ドレインパス合計の後。

<h3 id="defer-the-drain-past-the-first-signal">
  最初のシグナルを超えてドレインを遅延させる
</h3>

再起動しているランナーが最初のシグナルでドレインするのではなく、最大 `n` 分間保持するセッションを提供し続けるようにしたい場合は、[`--defer-shutdown-max-min <n>`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)を設定してください。最初の `SIGTERM` または `SIGINT` では、ランナーは新しい作業を受け取るのを停止し、保持するセッションを提供し続けます。ポーリングを続けるため、コントロールプレーンはそれらのセッションを再キューイングしません。Claude Code v2.1.238 以降が必要です。

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  最初のシグナル後にランナーが保持するセッションに何が起こるか
</h4>

最初のシグナルに続く最初の 2 つのステージでは、ランナーはセッションをリリースし、リリースされたセッションはユーザーが次のメッセージを送信するときに新しいランナーで再開されます。最初のシグナルからカウントして、ランナーは 3 つのステージを通じて移動します：

* **最初の `n` 分間**：ランナーは通常セッションを提供し、`--startup-timeout-min` と `--kill-session-after-min` を適用し続けます。[`--release-idle-session-min`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)も設定した場合、ランナーはそのユーザーがアイドル状態だったセッションをリリースします。それなしでは、アイドルセッションはランナーに留まります。
* **`n` 分が経過したとき**：ランナーはまだ保持しているすべてのセッションをリリースします。アイドルかどうか。ランナーはターン途中のセッションのターンが終了するのを待ち、ターンのバックグラウンドタスクのために最大 60 秒待ってから、そのセッションをリリースします。
* **リリース後の猶予が経過したとき**：ランナーはまだ保持しているセッションをドレインし、コントロールプレーンは各ドレインされたセッションを別のランナーにすぐに再キューイングします。リリース後の猶予は `n` 分が経過したときに開始され、デフォルトでは 75 秒です。`--drain-wait-sec` を 60 秒以上に設定した場合、リリース後の猶予は `--drain-wait-sec` + 15 秒です。

任意のステージで、ランナーはセッションを保持しなくなるとすぐに 0 で終了します。 2 番目のシグナルはステージを短縮します：`--defer-shutdown-max-min` なしの最初のシグナルの場合と同様に、ランナーは直ちにドレインします。ドレインが進行中になると、次のシグナルはランナーを強制終了します。これは、ドレインを開始したのが 2 番目のシグナルであっても、リリース後の猶予の経過であっても同じです。

<h4 id="size-the-stop-timeout">
  停止タイムアウトをサイズ設定する
</h4>

ホストの停止タイムアウトに、少なくとも 3 つの部分の合計を与えてください：設定する `n` 分、リリース後の猶予、[シャットダウンタイミング](#shutdown-timing)が説明する完全なドレインパス。デフォルト設定ではリリース後の猶予は 75 秒、ドレインパスは 80 秒です。`n` 分 + 155 秒を許可してください。ランナーはこの合計を起動時に出力します。`--defer-shutdown-max-min` が設定されている場合。

ランナーが終了する前に停止タイムアウトが切れると、ホストはランナーを強制終了します。保持するセッションは `post-session` フックを取得しません。ランナーは登録解除されず、コントロールプレーンは約 1 分後にセッションを再キューイングします。停止タイムアウトをその合計に与えることができない場合は、`--defer-shutdown-max-min` を設定しないままにして、ランナーが最初のシグナルでドレインするようにしてください。

<h3 id="what-reaches-a-running-post-session-hook">
  実行中の post-session フックに到達するもの
</h3>

`post-session` フックと Claude セッション子は、それぞれ独自の POSIX プロセスグループで実行され、ランナーから分離されているため、停止メカニズムは異なる方法で到達します：

* **ランナーが既にドレイン中の `SIGTERM`**：ランナーを直ちに強制終了し、ドレインパスの残りをスキップします。[`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)なしでは、ランナーが受け取る 2 番目の `SIGTERM` です。何も実行中の `post-session` フックに信号を送信しません。そのため、孤児を採用する init プロセスを持つベアホストでは、それは独自に終了しますが、監視されていません：タイムアウト予算はもはや適用されず、閉じたログパイプへの書き込みは `SIGPIPE` で強制終了できるため、強制終了から生き残る必要があるフックはそこで独自の出力をファイルにリダイレクトする必要があります。このページのコンテナレシピでは、ランナーはコンテナの PID 1 であり、その終了はコンテナを終了し、systemd のデフォルト `KillMode=control-group` の下では、cgroup 全体のキルは **Cgroup 全体のキル**エントリが説明するようにフックにも到達します。どちらでも、強制終了をフックに対して致命的として扱い、猶予期間に依存してください。
* **プロセスグループ全体のシグナル**。`kill -- -<pid>` をラッパースクリプト、シェルジョブ制御、またはグループ全体のウォッチドッグで：ランナーと mid-`checkout`-hook サブプロセスに到達します。これは意図的にグループに接続されたままですが、実行中の `post-session` フックまたはセッション子には到達しません。
* **Cgroup 全体のキル**。systemd のデフォルト `KillMode=control-group` または `terminationGracePeriodSeconds` が期限切れになったときに Kubernetes が配信する `SIGKILL` など：すべてに到達します。フックを含む。プロセスグループ分離はこれらに対して保護しません。これが猶予期間が完全なドレインパスをカバーする必要がある理由です。
* **フック独自のタイムアウト**：フックが `--post-session-hook-timeout-sec` を超えると、ランナーはフックの全体プロセスグループに `SIGTERM` を送信し、2 秒後に `SIGKILL` を送信します。フックがフォークしたワーカー（tar、rsync、git など）はラッパーシェルと一緒に終了し、孤児として生き残りません。ランナーの監視は、フックの stdio が閉じたときに終了します：独自の出力をファイルにリダイレクトし、`SIGTERM` ステージを超えて生き残るワーカーはランナーの到達範囲を超えています。

ドレインが開始されたとき、および強制終了時に、ランナーはまだ実行中の `post-session` フックの数をログに記録するため、静かなドレインと mid-snapshot のドレインを区別できます。

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  ベースディレクトリと容量をランナー全体で同じに保つ
</h2>

ランナーがセッション途中で死亡した場合、サーバーはセッションを再キューイングし、環境内の別のランナーがそれを取得します。そのランナーは、独自の `--base-dir` と `--capacity` からチェックアウトパスを導出します：`--capacity 1` は `--base-dir` の直下にチェックアウトし、`--capacity` が 1 を超える場合は代わりにセッションごとの worktrees を使用します。同じ環境内のランナーがこれらのフラグのいずれかに異なる値を使用する場合、再開されたセッションの作業ディレクトリが変更され、エージェントが以前に記録した絶対パス（編集、ツール呼び出し、独自のメモ）は、もはや存在しない場所を指します。

環境内のすべてのランナーで同じ `--base-dir` と `--capacity` を使用し、インスタンス ID やホスト名などのホストごとの値を使用しないでください。

ベースディレクトリのデフォルトは `/workspace` です。[`--base-dir` リファレンス行](/docs/ja/self-hosted-environments-reference#runner-cli-flags)が記録する例外を除きます。ランナーは書き込みアクセスが必要です。起動時に登録する前に、ランナーはディレクトリを作成し、書き込みできることを確認し、できない場合は `cannot create or write to base directory` で終了します。ルートとして開始されたランナーはデフォルト `/workspace` を自分で作成します。非ルートランナーの場合、ランナーを開始する前にディレクトリを作成してランナーのユーザーに所有権を与えるか、`--base-dir` をそのユーザーが既に所有しているディレクトリを指してください。

<h2 id="reuse-a-pre-warmed-checkout">
  事前にウォームアップされたチェックアウトを再利用する
</h2>

大規模なリポジトリの場合、クローンがセッション起動を支配することがあります。`--capacity 1` で [`checkout` フック](/docs/ja/self-hosted-environments-configuration#checkout) がない場合、ランナーは `<base-dir>/<repo-owner>/<repo>` でリポジトリごとに 1 つの正規クローンを保持し、セッション全体で再利用します。要求された ref をフェッチし、`HEAD` をデタッチして、それにハードリセットします。これは変更がほとんどない場合、ほぼ瞬時に完了します。コールドクローンをスキップするには、次の 2 つの方法のいずれかでクローンを提供します。

* **イメージ内にクローンを配置する**: ランナーイメージをそのパスにビルドしてクローンを含めます。その後、新しいコンテナはすべてディスクを再利用せずにウォームクローンで起動します。
* **永続ボリューム上にクローンを配置する**: [`--lock-to-account`](/docs/ja/self-hosted-environments-reference#runner-cli-flags) で 1 人のユーザーアカウントにプリロックされたランナーで、`--base-dir` を永続ボリュームに指定すると、ディスクはそのアカウントのみを提供します。プリロックされたランナーは Claude Tag チャネルセッションを取得しないため、このオプションはそれらを提供するランナーには適用されません。

再利用パスが保証するもの、しないもの：

* **任意のクローン形状が機能する**: パスの完全、シャロー、または単一ブランチクローンはそのまま使用されます。ランナーは既存のクローンにフェッチするときに `--depth` を渡しません。そのため、完全なプリウォームは完全な履歴を保持し、シャロークローンはシャローのままです。`CLAUDE_RUNNER_FETCH_DEPTH`（`full`、`0`、または数値。デフォルト 50）は、クローンがまだ存在しない場合にランナーが作成するコールドクローンのみを制御します。
* **追跡された変更はリセットされ、追跡されていないファイルは保持される**: 各セッションはハードリセットから開始され、前のセッションの追跡された変更を削除しますが、ランナーは `git clean` を実行しないため、ロックされたオーナーの以前のセッションからの追跡されていないファイルはツリーに残ります。
* **セッションごとのディレクトリも保持される**: チェックアウトの横に、ランナーは実行するすべてのセッションに対して `<base-dir>/_sessions/` の下にセッションごとのエントリを作成します。セッションの Claude 設定ディレクトリは、会話トランスクリプトのローカルコピーを保持します。その横には、セッションがある場合、セッションのアップロードされたファイルが配置されます。セッションディレクトリもそこに配置されます。セッションの実行中、セッションごとの worktrees と `checkout` フックチェックアウトを保持し、Claude がそこに書き込んだ他のすべてのものを保持します。

  デフォルトでは、ランナーはセッションが終了したときにこれらをそのまま残すため、ランナープロセスより長く存続するディスク上に蓄積されます。すべてのセッションはランナー自身のユーザーとして実行されるため、そのディスクが提供する後続のセッションはそれらを読み取ることができます。永続的な `--base-dir` を保持する場合は、その成長に対応するようにボリュームのサイズを設定してください。同じことは、[Docker Compose レシピ](#docker-compose) を含む、同じファイルシステム上でランナーを再起動するすべてのセットアップに適用されます。
* **`--remove-session-state` を使用する場合、セッションごとのディレクトリは保持されない**: [`--remove-session-state`](/docs/ja/self-hosted-environments-reference#runner-cli-flags) でランナーを起動して、セッションが終了するときに各セッションのセッションごとのディレクトリを削除させます。削除はベストエフォートです。ランナーがクリーンアップ実行前に強制終了された場合、ディレクトリは残ります。正規クローンとセッションがホスト上の他の場所（一時ディレクトリなど）に書き込んだファイルは、いずれにせよ残ります。
* **git プロキシを使用する場合、リセットはチェックアウトになる**: [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) を使用すると、ランナーは各セッションの前にクローンの `.git/` をサニタイズし、オブジェクトストア、ref、およびシャロー状態を保持しますが、インデックスを削除するため、各セッションはほぼ瞬時のリセットではなく、完全なワーキングツリーチェックアウトを実行します。それでも再クローンは実行されません。プロキシの下ではサブモジュールプリウォームはサポートされていません。
* **長いクローンは回避策を必要としない**: ランナーは各 git 操作を 120 秒の無進捗ウォッチドッグと 30 分のハードキャップで制限し、フラットタイムアウトではないため、進捗を報告し続けるスローコールドクローンは完了します。

<h2 id="pin-the-version">
  バージョンをピンする
</h2>

各セッションの子 Claude Code プロセスはランナー独自のバイナリを実行し、ランナーはセッション内でオートアップデートをオフにするため、すべてのセッションはホストにインストールされたか、イメージに組み込まれたバージョンを実行します。ホストレベルのアップデートはランナーが次に開始するときに有効になります。

* **フリートを 1 つのバージョンに保持するには**：ピンされたバージョンでイメージをビルドするか、ベアホストで特定のバージョンをインストールし、[オートアップデートを無効にしてください](/docs/ja/setup#disable-auto-updates)
* **アップグレードするには**：新しいバージョンをインストールするか、イメージを再ビルドしてから、ランナーを再起動してください
* **プラグイン**：プラグインマーケットプレイスもオートアップデートしません。ランナーの環境で `FORCE_AUTOUPDATE_PLUGINS=1` を設定して、バイナリがピンされたままの間、プラグインをオートアップデートさせます

<h2 id="scale-the-fleet">
  フリートをスケーリングする
</h2>

オーケストレーターは、ランナーを追加または削除するタイミングを決定します。[ランナーのライフサイクル](/docs/ja/self-hosted-environments#runner-lifecycle)に関する 1 つのオーナーごとのロックのため、最小レプリカ数は、同時にアクティブになると予想されるユーザーと Claude Tag エージェントの数です。`--capacity` は、オーナー全体ではなく、1 つのオーナーのセッション内での並列処理を制御します。

2 つのスケーリングアプローチが利用可能です。

* **固定フリート**: 静的なランナーレプリカセットを実行し、各ランナーが提供する [Prometheus メトリクス](/docs/ja/self-hosted-environments-reference#prometheus-metrics)に基づいてスケーリングします
* **オンデマンドランナー**: `claude self-hosted-runner orchestrator` サブコマンドを実行します。このコマンドは、利用可能なランナーがないキューに入っているセッションについて Anthropic をポーリングし、`spawn-runner` フックを呼び出して、セッションごとに 1 つ起動します。[オンデマンドランナー](/docs/ja/self-hosted-environments-configuration#on-demand-runners)を参照してください。

<h2 id="known-issues-and-limitations">
  既知の問題と制限事項
</h2>

これらはこのリリースの制限事項です。回避策が存在する場合は記載されています。

<h3 id="connector-traffic-leaves-your-network">
  コネクタトラフィックはネットワークを離れます
</h3>

Anthropic はランナーからではなく、独自のインフラストラクチャからコネクタツールを呼び出します。コネクタツールは claude.ai コネクタです。GitHub、Slack、Linear など。Claude がセルフホストセッションでコネクタを使用する場合、そのトラフィックはネットワーク境界内から発信されるのではなく、`api.anthropic.com` を通じて移動します。

セルフホストセッションからコネクタを除外するには、[`allowedMcpServers` および `deniedMcpServers` ポリシー設定](/docs/ja/managed-mcp#policy-based-control-with-allowlists-and-denylists)でフィルタリングしてください。Claude Code はこれらの設定をランナーホストからシードするサーバーとユーザーが追加するサーバーと同様に、Anthropic が配信するコネクタに適用します。他のサーバーの URL ベースの許可リストをデプロイする場合、Claude Code は配信されたコネクタもブロックします。配信されたコネクタを他のサーバーと一緒に利用可能に保つには、Anthropic プロキシパスの配信されたコネクタに一致するエントリを追加してください：

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

ツールトラフィックがネットワーク内に留まる必要がある場合は、代わりにランナーイメージ上でローカル MCP サーバーとして同等のツールを実行してください。[MCP サーバー](/docs/ja/self-hosted-environments-configuration#mcp-servers)を参照してください。

<h3 id="some-sessions-don’t-count-as-idle">
  一部のセッションはアイドルとしてカウントされません
</h3>

終了しないバックグラウンドタスクを保持するセッションはアイドルとしてカウントされないため、`--release-idle-session-min` はそのセッションのスロットをリリースしません。実行中のツール呼び出し内から要求された承認を待機しているセッションもアイドルとしてカウントされません。常に `--kill-session-after-min` をそれと一緒に設定して、セッションがスロットを無期限に保持できないようにハードバックストップとしてください。

`--kill-session-after-min` は暴走セッションのバックストップです。v2.1.260 以降のランナーでは、制限に達したセッションは直ちに終了されません。ランナーは猶予ウィンドウを与えます。デフォルトでは 15 分です。[`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/ja/self-hosted-environments-reference#environment-variable-only-settings)で変更できます：

* セッションがユーザーを待機している場合、またはターンが終了してバックグラウンドタスクのみを保持している場合、ランナーはそれを直ちにリリースします。セッションはユーザーが次のメッセージを送信するときに再開されます。
* ターンがまだ実行中の場合、ランナーはターンが終了するのを待つか、セッションが次にユーザーを待機するのを待ってから、それをリリースします。
* セッションが猶予ウィンドウの終了時にランナーに留まっている場合、ランナーはそれを終了し、実行中のターンの作業は失われます。実行中のツール呼び出し内から要求された承認を待機しているターンは、セッションがウィンドウを超えて存続する 1 つの方法です。

リリースされたセッションは新しいクローンから再開されるため、プッシュしていない作業はどちらの方法でも失われます。[再開されたセッションはプッシュされていない作業を失う](#additional-limitations)を参照してください。v2.1.260 より前では、ランナーはすべてのセッションを制限で終了し、実行中のターンが終了するのを最大猶予ウィンドウ待機しました。

フラグを最長予想セッション（例えば 8 時間の場合は `--kill-session-after-min 480`）の上に設定してください。アイドル状態になった会話からスロットを解放するには、代わりに `--release-idle-session-min` を使用してください。

<h3 id="additional-limitations">
  追加の制限事項
</h3>

* **再開されたセッションはプッシュされていない作業を失う**：セッションがリリースされるか、ランナーが再起動され、ユーザーが別のメッセージを送信すると、セッションは新しいランナーで再開され、開始ブランチからリポジトリを再度クローンするため、セッションがプッシュしていない作業は失われます。[`--push-outcome-on-release`](/docs/ja/self-hosted-environments-reference#runner-cli-flags)を設定して、ランナーがリリースする前にセッションの結果ブランチをベストエフォートでプッシュするようにします。再開されたセッションはそれらのコミットから開始されます。これはコミットされた作業を保持し、ダーティな作業ツリーではありません。有効にする前に、ソースリモートの `claude/*` refs へのプッシュを制限してください。例えば、ブランチルールセットを使用します：再開時に、ランナーは以前にプッシュされたブランチをフェッチし、誰がプッシュしたかを検証しません。そのため、それらの refs へのプッシュアクセスを持つすべてのユーザーが再開されたワークスペースにコンテンツを配置できます。ランナーは再開時にセッションごとの設定も破棄します。つまり、セッションの Claude 設定ディレクトリとセッションが書き込んだシェル状態です。`--push-outcome-on-release` はそれらをカバーしません。
* **プライベートリポジトリは mid-session に追加できません**：セッション開始後に追加されたリポジトリは、セルフホストランナーで認証情報でクローンされないため、追加は失敗します。セッションを作成するときに、セッションが必要とするすべてのリポジトリを選択してください。
* **一部のコネクタはセルフホストセッションに表示されません**：claude.ai 設定でまだ接続していないコネクタはセルフホストセッションにリストされず、セッションはそれを接続するように促しません。最初に設定で接続してから、新しいセッションを開始してください。実行中のセッションにコネクタを追加しても、Claude がそのツールを利用できるようにはなりません。新しく追加されたコネクタを取得するには、新しいセッションを開始してください。

<h3 id="report-an-issue">
  問題を報告する
</h3>

セルフホスト環境の問題については、Anthropic アカウントチームに連絡してください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

ガイド付き診断については、ランナーホスト上で doctor サブコマンドを実行してください。doctor サブコマンドは、ランナーのログと状態が添付された対話型 Claude Code セッションを開始します。そのホスト上で `claude auth login` でサインインして、セッションが環境、ランナー、キューに入っているセッションをクエリできるようにしてください。そのサインインがない場合、例えばホストが API キーで認証する場合、ローカルヘルスエンドポイント、メトリクス、およびランナーのログに限定され、`--log-file` でランナーを起動した場合のみログを読み取ります。

```bash theme={null}
claude self-hosted-runner doctor
```

一般的な問題：

* **ランナーが環境に表示されない**：ホストが HTTPS 経由で `api.anthropic.com` に到達できること、環境シークレットが最新であること、ホストの時刻が実時間の 5 分以内であることを確認してください。より大きなずれは認証失敗を引き起こします。ランナーは認証失敗時に拒否理由を含む `[runner:fatal]` をログに記録します。
* **ランナーが `cannot create or write to base directory` で起動時に終了する**：ランナーが `--base-dir` を作成または書き込みできません。これはデフォルトで `/workspace` です。ディレクトリの所有権を修正するか、[ランナー全体でベースディレクトリと容量を同じに保つ](#keep-the-base-directory-and-capacity-identical-across-runners)で説明されているように `--base-dir` を書き込み可能なパスに指定してください。ランナーが代わりにベースディレクトリチェックがタイムアウトしたことを示す `[runner:fatal]` をログに記録する場合、ディレクトリはハングしている NFS または CSI マウント上にあります。権限ではなくマウントヘルスを確認してください。ランナーは `--log-file` を開く前にこれらの起動失敗を stderr に出力するため、ログファイルではなくターミナルまたはプラットフォームのコンテナログで探してください。v2.1.225 より前では、ランナーは起動時にベースディレクトリをチェックしておらず、この設定ミスはピックアップ後にセッションを失敗させました。
* **セッションがキューに留まる**：すべてのオンラインランナーは異なる所有者にロックされている可能性があります。各ランナーの `claude_code_self_hosted_runner_locked_account` [メトリクス](/docs/ja/self-hosted-environments-reference#prometheus-metrics)またはその `[runner:health]` ログ行の `locked_account` フィールドをチェックして、誰がそれを保持しているかを確認してください。どちらも、ランナーが `act.email` クレームを含むセッショントークンを発行された後にのみ所有者のメールアドレスを表示します。これは Claude Tag エージェントのセッションでは決して行われません。クレームがない場合、ランナーは `locked_account` シリーズを出力せず、`locked_account=yes` をログに記録します。これはランナーがロックされていることを示しますが、どの所有者にロックされているかは示しません。レプリカを追加するか、既存のランナーがドレインして再起動するのを待ってください。環境がオンデマンドランナーを使用する場合は、代わりにオーケストレーターをチェックしてください。[オンデマンドランナー](/docs/ja/self-hosted-environments-configuration#on-demand-runners)を参照してください。
* **セッションがピックアップ直後に失敗する**：claude.ai/code でセッションを開いてエラーを確認してください。最も一般的な原因は、ランナーイメージの [git 認証情報](#configure-git)の欠落とインストールされていないビルドツールです。書き込み不可能なベースディレクトリはセッションを失敗させるのではなく、起動時にランナーを停止させます。このリストの **ランナーが `cannot create or write to base directory` で起動時に終了する** エントリを参照してください。
* **セッションが認証エグレスプロキシ経由でネットワークに到達できない**：[`--proxy-authorization-command` または `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) で設定したソースが失敗する場合、30 秒後にタイムアウトする場合、または空の値を生成する場合、ランナーはその接続に `502 Bad Gateway` で応答し、理由をログに記録します。ランナーはそのログでコマンドの stderr を編集し、ヘッダー値をログに記録しません。`--proxy-authorization-command` を使用する場合、ホスト上でコマンド自体を実行して、stdout 全体のヘッダー値を出力することを確認してください。ランナーが代わりに `could not start the proxy-authorization listener` で起動時に終了する場合、ループバックリスナーを開くことができませんでした。
* **ランナーが `rejecting the malformed poll response` を含む `Poll failed` 行をログに記録する**：ランナーは、本体がキューの予期された JSON ではないワークポール応答を受け取りました。最も一般的には、インターセプティングプロキシやキャプティブポータルなど、ランナーと `api.anthropic.com` の間の何かが独自のページで応答したためです。ランナーは応答を拒否し、`claude_code_self_hosted_runner_poll_errors_total` [メトリクス](/docs/ja/self-hosted-environments-reference#prometheus-metrics)の `transport` 種別の下でカウントし、[セッションライフサイクル](/docs/ja/self-hosted-environments#session-lifecycle)で説明されている失敗したポールスケジュールで再試行します。ランナーはライブセッションを提供し続けます。`api.anthropic.com` からの応答を変更されずに通すようにプロキシを設定してください。v2.1.246 より前では、ランナーはそのような応答を空のワークキューとして読み取り、ライブセッションを終了するか、終了させる可能性がありました。
* **セッションのブランチがリモートに存在しなくなった**：セッションが読み取り専用の git ソースの場合、ランナーはそのソースをスキップして残りのソースで続行します。セッションが結果をプッシュするソースの場合、削除されたブランチ（通常はマージされて自動削除されたため）はセッションを失敗させ、リポジトリとブランチを名前付けするエラーを表示し、ブランチを復元して再試行するよう求めます。ランナーはスキップするとリポジトリがまったくなくなる場合、同じエラーでセッションを失敗させます。v2.1.228 より前では、そのようなセッションは空のディレクトリで開始されました。
* **セッションがそのリポジトリの 1 つなしで開始される**：[`checkout` hook](/docs/ja/self-hosted-environments-configuration#checkout) がないランナーでは、git ホストはセッションが読み取り専用のリポジトリのランナーのアクセスチェックを拒否できます。ランナーはそのリポジトリをスキップし、拒否を名前付けする `[runner:warn] could not access context source` 行をログに記録し、残りのリポジトリでセッションを開始します。

  ランナーはクリアな拒否のみをスキップします：ホストがリポジトリが見つからないことを答える、git がホストの認証情報を見つけない、または認証が失敗します。ネットワーク障害、タイムアウト、または HTTP `403` はセッション開始を失敗させます。セッションが結果をプッシュするリポジトリの拒否も同様です。ランナーはスキップするとリポジトリがまったくなくなるセッションを失敗させます。[`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) を使用する場合、ランナーは git プロキシ自体が拒否するリポジトリのみをスキップします。

  アクセスチェックはセッションがランナーで開始されるたびに再度実行されるため、ランナーの git アイデンティティが読み取りアクセスを持つと、次の開始でリポジトリをクローンします。v2.1.274 より前では、これらの拒否のそれぞれがセッション開始を失敗させました。
* **セッションの開始に数分かかる**：初期クローンが通常支配的です。`claude_code_self_hosted_runner_session_init_duration_seconds` [メトリクス](/docs/ja/self-hosted-environments-reference#prometheus-metrics)を監視して確認し、[事前にウォーミングされたチェックアウト](#reuse-a-pre-warmed-checkout)またはより小さい `CLAUDE_RUNNER_FETCH_DEPTH` でクローンを削減してください。
* **ターンが 401 で失敗する**：各セッションは、ランナーが Anthropic から取得し、セッションの stdin 経由でローテーションする短命の [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ja/self-hosted-environments-configuration#wrapper-scripts) を使用してモデル呼び出しを認証します。ターンがモデル API から 401 または 403 で終了する場合、ランナーは新しいトークンを取得し、セッションに渡します。失敗したターンは再試行されません。

  フェッチが失敗する場合、ランナーは `inference_token refresh failed` 行をログに記録し、いつ再試行するかを示し、セッションが実行されている限り再試行を続けます。

  セッションの約 30 分後にすべての呼び出しが失敗し始める場合、ラッパースクリプトがセッションの stdin を切断した可能性があります。そのため、トークンローテーションがそれに到達できません。[stdin とファイルディスクリプタ 3 を接続したままにする](/docs/ja/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached)を参照してください。

  v2.1.274 より前では、ランナーは数回の試行後に失敗したフェッチの再試行を停止し、次のスケジュール済みのものを待ちました。失敗したターンはフェッチをトリガーしなかったため、次のスケジュール済みフェッチまで、すべてのターンが 401 で失敗しました。
* **ポッドがドレイン中に強制終了される**：`terminationGracePeriodSeconds` をランナーが起動時にログに記録する値以上に引き上げてください。[シャットダウンタイミング](#shutdown-timing)を参照してください。

ログが初期化されると、ランナーはそのライフサイクルログ（`[runner:fatal]` 行を含む）を stdout に書き込み、デバッグ出力を stderr に書き込みます。すべて JSON ではなくプレーンテキスト行として。上記のトラブルシューティングエントリで説明されている起動失敗はその前に stderr に出力されます。`--log-file` で両方のストリームをキャプチャします。これにより `self-hosted-runner doctor` がそれらをテールできるようになり、またはプラットフォームのログ収集で。

各セッションの子プロセスは個別のデバッグログを書き込みます。失敗時、ランナーはログのテールを claude.ai/code のセッションと一緒に表示します。[`--remove-session-state`](/docs/ja/self-hosted-environments-reference#runner-cli-flags) でランナーを起動しない限り、失敗したセッションのログもディスク上に保持し、ランナーログにそのパスを出力します。

<h2 id="what’s-next">
  次のステップ
</h2>

* [セッションをカスタマイズする](/docs/ja/self-hosted-environments-configuration)：ラッパースクリプト、ライフサイクルフック、オンデマンドランナー、MCP サーバー、権限
* [エンドツーエンドをテストする](/docs/ja/self-hosted-environments-testing)：本番環境に昇格させる前に新しいランナーイメージを検証する
* [リファレンス](/docs/ja/self-hosted-environments-reference)：すべての CLI フラグ、環境変数、メトリクス
