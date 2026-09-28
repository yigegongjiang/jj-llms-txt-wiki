> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# セルフホスト環境リファレンス

> セルフホストランナーとオーケストレーターの完全なリファレンス：CLI フラグ、環境変数、Prometheus メトリクス。

<Note>
  セルフホスト環境は Team および Enterprise プランでパブリックベータ版です。[Owner](/docs/ja/cloud-environments#organization-shared-environments) が [**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments) で **Allow self-hosted environments** をオンにすることで有効になります。このページはフラグとメトリクスのリファレンスです。セットアップについては [クイックスタート](/docs/ja/self-hosted-environments-quickstart) を、フリート構成については [本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy) をご覧ください。
</Note>

このページは、[セルフホスト環境](/docs/ja/self-hosted-environments) で実行する 2 つのプロセスのリファレンスです。ランナーはホスト上で Claude Code [クラウドセッション](/docs/ja/claude-code-on-the-web) を実行し、オプションのオートスケーリングオーケストレーターはセッションがキューに入ると同時にランナーを起動します。それぞれ独自のフラグテーブルを持っています。どちらも Linux または macOS ホスト上で実行され、`/workspace` や `~/.claude` などのデフォルトを想定しています。インストール済みバージョンの権限あるリストについては、`claude self-hosted-runner --help` を実行してください。

メトリクスシリーズと一部の API フィールドは、これらのページが環境と呼ぶものに対して `pool` を使用しています。どちらの用語も同じものを指しています。環境 ID は `pool_id` フィールドで、形式は `ccpool_...` です。これらのページが `pool` 識別子を示す場所では、環境を指しています。CLI フラグと環境変数では `environment` と表記されます。例えば `--environment-secret-file` のように。非推奨の `pool` 表記はまだ機能します。[`--environment-secret-file` 行](#runner-cli-flags) で説明されているとおりです。

<h2 id="runner-cli-flags">
  Runner CLI フラグ
</h2>

ほとんどのフラグには対応する環境変数があります。両方が設定されている場合、フラグが優先されます。期間フラグは CLI では分または秒を取りますが、対応する環境変数は常にミリ秒単位で、`_MS` サフィックスで示され、デフォルト列はフラグの単位を示します。`--exit-if-unused-min 10` は `SELF_HOSTED_RUNNER_IDLE_SHUTDOWN_MS=600000` と同等であり、`SELF_HOSTED_RUNNER_STARTUP_TIMEOUT_MS: "15"` のような Helm 値は 15 分のデフォルトではなく 15 ミリ秒を意味します。

| フラグ                                       | 環境変数                                              | デフォルト                       | 説明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------------------------------- | :------------------------------------------------ | :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--api-url <url>`                         | なし                                                | `https://api.anthropic.com` | API ベース URL。テスト目的でのみオーバーライドしてください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `--base-dir <path>`                       | `SELF_HOSTED_RUNNER_BASE_DIR`                     | `/workspace`; Windows では なし | リポジトリチェックアウトとセッションごとの作業ディレクトリ用のディレクトリ。ランナーはこのパスまたはその親への書き込みアクセスが必要です。ランナーはスタートアップ時にディレクトリを作成し、作成または書き込みができない場合は `cannot create or write to base directory` で終了します。v2.1.225 より前は、ランナーは最初のセッションが開始されたときにディレクトリを作成していたため、使用不可能なパスはスタートアップではなくセッションを失敗させていました。Windows はサポートされていないランナーホストであり、デフォルトはありません。フラグを渡すか変数を設定しない限り、ランナーはスタートアップ時に終了します。環境内のすべてのランナーで同じ値を使用してください。[ランナー全体でベースディレクトリと容量を同じに保つ](/docs/ja/self-hosted-environments-deploy#keep-the-base-directory-and-capacity-identical-across-runners) を参照してください。                                                                                                                                                             |
| `--capacity <n>`                          | なし                                                | `1`                         | このランナーが処理する最大同時セッション数。すべてのセッションは同じロック済み [オーナー](/docs/ja/self-hosted-environments#key-concepts) に属します。環境内のすべてのランナーで同じ値を使用してください。[ランナー全体でベースディレクトリと容量を同じに保つ](/docs/ja/self-hosted-environments-deploy#keep-the-base-directory-and-capacity-identical-across-runners) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                       |
| `--client-label <label>`                  | `SELF_HOSTED_RUNNER_CLIENT_LABEL`                 | ホストのホスト名                    | ランナーが登録時に送信するラベル。ランナーはこれを [`claude_code_self_hosted_runner_info`](#prometheus-metrics) の `client_label` ラベルとしても報告します。Claude Code v2.1.248 以降が必要です。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `--configure-git`                         | `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`              | オフ                          | スタートアップ時にグローバル git ID を書き込み、Anthropic コミット署名を有効にし、git プッシュネゴシエーションをオンにし、`Co-authored-by:` トレーラーを追加するコミットフックをインストールします。プッシュネゴシエーションには Claude Code v2.1.257 以降が必要です。[git を設定する](/docs/ja/self-hosted-environments-deploy#configure-git) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `--confine-repo-settings <mode>`          | `SELF_HOSTED_RUNNER_CONFINE_REPO_SETTINGS`        | `warn`                      | リポジトリのコミットされた設定がそのセッション独自のワークスペース外への書き込みまたは読み取りアクセスを許可しようとする場合、環境変数を設定する場合、またはオペレーターのサンドボックスまたはフック姿勢をオーバーライドしようとする場合（`sandbox.enabled: false` や `disableAllHooks` など）にセッションにフラグを立てるガードのモードを設定します。デフォルトの `warn` は違反をログに記録してもセッションを開始し、`enforce` はセッションを拒否し、`off` はスキャンを無効にします。[デプロイメントを強化する](/docs/ja/self-hosted-environments-deploy#harden-your-deployment) を参照してください。                                                                                                                                                                                                                                                                                                        |
| `--debug-token-dir <path>`                | `SELF_HOSTED_RUNNER_DEBUG_TOKEN_DIR`              | 未設定                         | ライブトークンをディスクに書き込んで検査します。デバッグのみ。本番環境では使用しないでください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `--defer-shutdown-max-min <n>`            | `SELF_HOSTED_RUNNER_DEFER_SHUTDOWN_MAX_MS`        | `0`                         | 最初の `SIGTERM` または `SIGINT` で、ドレインする代わりに既にアタッチされているセッションの提供を続け、その後 N 分後に残っているものをリリースして終了します。これを設定する前に、ホストの停止タイムアウトを上げてください。[最初のシグナルの後のドレインを遅延させる](/docs/ja/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal) を参照してください。`0` は無効にします。Claude Code v2.1.238 以降が必要です。                                                                                                                                                                                                                                                                                                                                                                                           |
| `--drain-grace-sec <n>`                   | `SELF_HOSTED_RUNNER_DRAIN_GRACE_MS`               | `0`                         | ランナーがシャットダウンシグナルを受け取るか、リタイア時間に達するまで、アクティブなセッションが終了した後にランナーが終了するタイミングを制御します。`0` はポーリングなしで即座に終了し、正の値はランナーをアライブに保ち、ロック済みオーナーのキューを最初にその秒数だけ再ポーリングします。これは [強化セクション](/docs/ja/self-hosted-environments-deploy#harden-your-deployment) で説明されているセッションごとのコンテナ分離のコストがかかります。[`--defer-shutdown-max-min`](/docs/ja/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal) で遅延させた最初のシグナルの後、ランナーはセッションを保持していない限り即座に終了し、ここで設定したものは関係ありません。                                                                                                                                                                                                                                         |
| `--drain-marker-file <path>`              | `SELF_HOSTED_RUNNER_DRAIN_MARKER_FILE`            | 未設定                         | ホストが `SIGTERM` を送信する前にグレースフルドレインを通知するために書き込むマーカーファイル。ドレインが開始されるときにファイルが存在する場合、ランナーはその終了を Anthropic に通常のシャットダウンシグナルではなくホストドレインとして報告します。`--drain-wait-sec` ホールドを含むドレイン自体は、フラグなしの場合と同じように実行されます。セッションが書き込みできないローカルファイルシステム上のパスを指定してください。Claude Code v2.1.271 以降が必要です。                                                                                                                                                                                                                                                                                                                                                                                                        |
| `--drain-wait-sec <n>`                    | `SELF_HOSTED_RUNNER_DRAIN_WAIT_MS`                | `0`                         | ドレインが開始されたら（[`--defer-shutdown-max-min`](/docs/ja/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal) を設定していない限り `SIGTERM` で）、各セッションの進行中のターンとバックグラウンドタスクが終了するまで最大 N 秒待機してから子を終了します。この待機中、ランナーは終了したばかりのバックグラウンドタスクを、その結果を読む後続のターンが開始されるまで実行中としてカウントします。最大で [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](#environment-variable-only-settings) ウィンドウです。                                                                                                                                                                                                                                                                                                           |
| `--environment-secret-file <path>`        | `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET`           | 必須                          | 環境シークレットを含むファイルへのパス、または [オーケストレーター](/docs/ja/self-hosted-environments-configuration#on-demand-runners) によって生成されたランナーの場合は単一使用のワークオーダー JWT。`SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` はファイルパスではなくシークレット値を直接保持します。古い `--pool-secret-file` フラグと `SELF_HOSTED_RUNNER_POOL_SECRET` 変数はまだ機能し、stderr に非推奨通知を出力します。2.1.216 より前のプレビュープログラムランナービルドはこれらの古い名前のみを認識します。                                                                                                                                                                                                                                                                                                                         |
| `--exec-path <path>`                      | `SELF_HOSTED_RUNNER_EXEC_PATH`                    | 独自のバイナリ                     | 各セッション用に生成するバイナリまたはラッパースクリプト。[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `--exit-if-unused-min <n>`                | `SELF_HOSTED_RUNNER_IDLE_SHUTDOWN_MS`             | `0`                         | 作業が割り当てられたことのない N 分間のポーリング後に終了します。オートスケーラーのスケールダウン用です。`0` は無効にします。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `--git-host-rewrite <from>=<to>`          | なし                                                | 未設定                         | クローン前に `https://<from>/...` ソース URL を `https://<to>/...` に書き換えます。スプリットホライズン DNS 用です。繰り返し可能。フラグのみ。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `--git-ssh-rewrite <host>`                | なし                                                | 未設定                         | クローン前に `https://<host>/...` ソース URL を `git@<host>:...` に書き換えます。SSH のみの git ホスト用です。繰り返し可能。フラグのみ。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `--health-port <port>`                    | `SELF_HOSTED_RUNNER_HEALTH_PORT`                  | `8080`                      | `/healthz` と `/metrics` リスナーのポート。`0` に設定して無効にします。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `--hooks-dir <path>`                      | `SELF_HOSTED_RUNNER_HOOKS_DIR`                    | 未設定                         | ライフサイクルフックスクリプトのディレクトリ。[ライフサイクルフック](/docs/ja/self-hosted-environments-configuration#lifecycle-hooks) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `--host-config-snapshot <mode>`           | `SELF_HOSTED_RUNNER_HOST_CONFIG_SNAPSHOT`         | `disk`                      | ランナーが [ホスト設定ディレクトリ](#environment-variable-only-settings) のスタートアップスナップショットを保持する場所。各セッションをシードします。`disk` はスナップショットを `--base-dir` の下のランナー所有ディレクトリにコピーし、各セッション開始時に、すべてのファイルをメモリ内ダイジェストに対して検証します。コピー内のファイルが変更されている場合、セッションは失敗し、ランナーは再起動するまでセッションを拒否します。`memory` はスナップショット全体をヒープに保持し、64 MiB でキャップされます。キャップを超える場合、セッションはホスト設定なしで開始され、そのことを示す通知が表示されます。ランナーがディスクスナップショットを書き込めない場合、失敗をログに記録し、その実行に `memory` を使用します。Claude Code v2.1.271 以降が必要です。                                                                                                                                                                                                                                    |
| `--kill-session-after-min <n>`            | `SELF_HOSTED_RUNNER_MAX_LIFETIME_MS`              | `0`                         | セッションを N 分のウォールクロック時間に制限します。スタックセッションの安全制限として機能します。v2.1.260 以降では、ランナーは制限に達したセッションをリリースして、ユーザーの次のメッセージで再開できるようにし、[`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](#environment-variable-only-settings) グレースウィンドウが終了した時点でまだランナー上にある場合にのみ、そのセッションを終了します。v2.1.260 より前は、ランナーは制限でセッションを終了していました。詳細と値の選択方法については、[一部のセッションはアイドルとしてカウントされない](/docs/ja/self-hosted-environments-deploy#some-sessions-don%E2%80%99t-count-as-idle) を参照してください。`0` は無効にします。                                                                                                                                                                                                                                     |
| `--lock-to-account <id>`                  | `SELF_HOSTED_RUNNER_LOCK_TO_ACCOUNT`              | 未設定                         | スタートアップ時に最初のセッションでロックする代わりに、ランナーを特定のアカウントに事前ロックします。環境の組織内のメールアドレスまたは `user_...` ID を受け入れます。事前ロックされたランナーは、アカウントを持たない Claude Tag チャネルセッションを決してピックアップしません。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `--log-file <path>`                       | `SELF_HOSTED_RUNNER_LOG_FILE`                     | 未設定                         | ランナーログを stdout と stderr に加えてファイルにミラーリングします。`0600` 権限で作成されます。`self-hosted-runner doctor` がローカルでログをテールするために必要です。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `--log-level <level>`                     | なし                                                | `info`                      | `info` または `debug`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `--post-session-hook-timeout-sec <n>`     | `SELF_HOSTED_RUNNER_POST_SESSION_HOOK_TIMEOUT_MS` | `60`                        | すべてのセッション終了時（ランナーシャットダウンを含む）の [`post-session` フック](/docs/ja/self-hosted-environments-configuration#post-session) の予算                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `--proxy-authorization-command <command>` | `SELF_HOSTED_RUNNER_PROXY_AUTHORIZATION_COMMAND`  | 未設定                         | ランナーがエグレスプロキシへのすべての接続に対して実行するシェルコマンド。トリミングされた stdout を `Proxy-Authorization` ヘッダー値として使用します。`HTTPS_PROXY` または `HTTP_PROXY` が必要で、`--proxy-authorization-file` と組み合わせることはできません。[エグレスプロキシに認証する](/docs/ja/self-hosted-environments-deploy#authenticate-to-an-egress-proxy) を参照してください。Claude Code v2.1.238 以降が必要です。                                                                                                                                                                                                                                                                                                                                                                  |
| `--proxy-authorization-file <path>`       | `SELF_HOSTED_RUNNER_PROXY_AUTHORIZATION_FILE`     | 未設定                         | ランナーがエグレスプロキシへのすべての接続に対して読み取るファイル。トリミングされた内容を `Proxy-Authorization` ヘッダー値として使用します。別のプロセスが所定の位置でローテーションするトークンの場合、このフラグを使用してください。`--proxy-authorization-command` と同じ要件を持ち、それと組み合わせることはできません。[エグレスプロキシに認証する](/docs/ja/self-hosted-environments-deploy#authenticate-to-an-egress-proxy) を参照してください。Claude Code v2.1.238 以降が必要です。                                                                                                                                                                                                                                                                                                                                                    |
| `--push-outcome-on-release`               | `SELF_HOSTED_RUNNER_PUSH_OUTCOME_ON_RELEASE`      | オフ                          | ドレインやアイドルリリースなどのランナー開始セッション終了時に、ワークスペースを削除する前に追跡された結果ブランチを `origin` にプッシュします。進行中のコミットは再起動後も生き残ります。ベストエフォート。シャットダウン予算に 30 秒を追加し、プッシュされたブランチから再開するには git 2.29 以降が必要です。有効にする前に、`claude/*` refs へのプッシュアクセスを制限してください。[再開されたセッションはプッシュされていない作業を失う](/docs/ja/self-hosted-environments-deploy#additional-limitations) を参照してください。`checkout` ライフサイクルフックを介してチェックアウトされたリポジトリはプッシュされません。代わりに [`post-session` フック](/docs/ja/self-hosted-environments-configuration#post-session) からスナップショットしてください。                                                                                                                                                                                                |
| `--release-idle-session-min <n>`          | `SELF_HOSTED_RUNNER_SESSION_IDLE_MS`              | `0`                         | ターンが終了するか、セッションがユーザーのアクションを待つ後、N 分間の非アクティビティ後にセッションスロットをリリースします。ターン中のセッション（決して終了しないバックグラウンドタスクを保持しているセッション、または実行中のツール呼び出し内から要求された承認を含む）はアイドルとしてカウントされません。`--kill-session-after-min` とペアにして、ハードバックストップとして機能させてください。セッションのバックグラウンドタスクが終了した後、ランナーはセッションをビジーと見なします。その結果を読む後続のターンが開始されるまで、最大で [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](#environment-variable-only-settings) ウィンドウです。ランナーがシャットダウンシグナルを受け取るか、リタイア時間に達するまで、ランナーにアクティブなセッションがなくなるリリースは、通常のドレインと同じ終了パスを開始します。`--drain-grace-sec` によって管理されます。[`--defer-shutdown-max-min`](/docs/ja/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal) で遅延させた最初のシグナルの後、ランナーはリリースがセッションを保持していない限り即座に終了します。`0` は無効にします。 |
| `--remove-session-state [bool]`           | `SELF_HOSTED_RUNNER_REMOVE_SESSION_STATE`         | オフ                          | セッションがこのランナーで終了したときに、`<base-dir>/_sessions/` の下のセッションごとのディレクトリを削除します。結果に関係なく。[事前ウォーミングされたチェックアウトを再利用する](/docs/ja/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) は、それらが保持するものと、それらが残っているときに誰がそれらを読むことができるかを説明しています。削除はベストエフォート。ランナーが強制終了されるか、クリーンアップが実行される前にドレイン期限に達した場合、セッションごとのディレクトリは所定の位置に留まります。フラグがオンの場合、失敗または中断されたセッションのデバッグログはディスクに保持されません。Claude Code v2.1.268 以降が必要です。                                                                                                                                                                                                                                                                                   |
| `--retire-at <epoch-seconds>`             | `SELF_HOSTED_RUNNER_RETIRE_AT`                    | 未設定                         | ランナーを秒単位の絶対 Unix タイムスタンプでリタイアします。ランナーが既知の時間に強制終了されるインフラストラクチャ用です。[ランナーライフサイクル](/docs/ja/self-hosted-environments#runner-lifecycle) はリリースシーケンスとマージンのサイズ方法を説明しています。2001 より前または 5138 年より後の値はフラグによって拒否され、環境変数によって無視されます。                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `--session-stop-grace-sec <n>`            | `SELF_HOSTED_RUNNER_SESSION_STOP_GRACE_MS`        | `5`                         | セッション終了後、Claude プロセスがクリーンに終了するまで待機する時間。強制終了する前に。子独自の `SessionEnd` フックがより多くの時間を必要とする場合は、値を上げてください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `--startup-timeout-min <n>`               | `SELF_HOSTED_RUNNER_STARTUP_TIMEOUT_MS`           | `15`                        | 子が生成後 N 分以内に [アクティビティチャネル](/docs/ja/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached) で初期化されたことを通知していない場合、セッションスロットをリリースします。通常の出力ではなく、子の初期化シグナルによってクリアされます。その後、`--release-idle-session-min` が引き継ぎます。`0` は無効にします。                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `--trust-workspace [bool]`                | `SELF_HOSTED_RUNNER_TRUST_WORKSPACE`              | オン                          | 各セッションのリポジトリパスの永続化された信頼をシードします。リポジトリコミットされた `permissions.allow` と `additionalDirectories` が尊重されるようにします。`false` に設定して、リポジトリコミットされた権限付与をドロップし、代わりにホスト設定の `settings.json` で許可ルールを設定します。リポジトリコミットされた `sandbox.*` 設定はどちらの方法でも適用されます。これが [リポジトリ設定ガード](/docs/ja/self-hosted-environments-deploy#harden-your-deployment) がこのフラグに関係なくそれらをスキャンする理由です。                                                                                                                                                                                                                                                                                                                                    |
| `--use-anthropic-git-proxy`               | `CLAUDE_RUNNER_USE_GIT_PROXY=1`                   | オフ                          | 顧客管理の git 認証の代わりに [Anthropic git プロキシ](/docs/ja/self-hosted-environments-deploy#use-the-anthropic-git-proxy) 経由でクローンします。`--capacity 1` と git 2.32 以降が必要です。ランナーはそれ以外の場合は起動を拒否します。書き換えフラグに優先します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

ほとんどの期間フラグには最大値があります。各タイムアウトをランタイムの 32 ビットタイマー上限（約 24.85 日）内に保つために選択されています。`--*-min` フラグは 10080 分（7 日）でキャップされます。`--drain-grace-sec` は 604800 秒（7 日）でもキャップされます。`--drain-wait-sec` は 86400 秒（24 時間）でキャップされます。`--session-stop-grace-sec` と `--post-session-hook-timeout-sec` はキャップされていません。キャップを超過する動作は表面ごとに異なります。

* **フラグ**: スタートアップはエラーで失敗します。
* **環境変数**: ランナーはそれを拒否するのではなく、値をタイマー上限にクランプします。

<h2 id="orchestrator-cli-flags">
  オーケストレーター CLI フラグ
</h2>

`self-hosted-runner orchestrator` サブコマンド（[オンデマンドランナー](/docs/ja/self-hosted-environments-configuration#on-demand-runners) を生成）は、`--api-url`、`--environment-secret-file`、`--hooks-dir`、`--health-port`、`--log-level` をランナーと同じデフォルトで受け入れます。ランナーのフラグに 1 つある場合は同じ環境変数を使用します。ただし、`--hooks-dir` は必須で、`spawn-runner` フックを含む必要があります。また、独自のフラグも取ります。

| フラグ                              | デフォルト | 説明                                                                                                                                                |
| :------------------------------- | :---- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--hook-concurrency <n>`         | `4`   | 並列で実行される最大 `spawn-runner` フック数。また、ポーリングごとにクレームされるスポーン要求の数もキャップします。                                                                                |
| `--hook-timeout <sec>`           | `60`  | この多くの秒後にフックのプロセスツリーを終了します。タイムアウトとその 5 秒のキルグレースは `--expected-spawn-seconds` より下にある必要があります。オーケストレーターはスタートアップでこれを強制します。                              |
| `--expected-spawn-seconds <sec>` | `120` | スポーン済みランナーの予想 p99 ブート時間（秒単位）。サーバー強制範囲 10 ～ 3600。すべてのポーリングでサーバー側リースとして送信されます。ランナーが経過前に登録されない場合、セッションは新しいオーダー ID で再提供されます。すべてのレプリカはこの値を共有する必要があります。 |
| `--min-idle <n>`                 | `0`   | スタンバイランナーを積極的に生成することで、少なくとも N 個のアイドルセッションスロットを無料で保ちます。`0` はプレウォーミングを無効にします。ランナーの `--exit-if-unused-min` とペアにして、余分なスタンバイランナーが自分自身を再利用するようにします。     |
| `--debug-dir <path>`             | 未設定   | 各スポーン要求のワークオーダーとフック stderr をディスクに書き込みます。デバッグのみ。本番環境では設定しないでください。                                                                                  |

<h3 id="scm-connector-flags">
  SCM コネクタフラグ
</h3>

オーケストレーターは Anthropic のコントロールプレーンへのスタンディング WebSocket 接続を保持できます。リポジトリピッカーやブランチまたは ref リゾルバーなどのホスト済みプリセッションフローが、ネットワーク内からのみルーティング可能な GitHub Enterprise Server ホストに到達できるようにします。`--scm-connector-host` を設定しない限り、コネクタはオフのままです。

| フラグ                                                     | デフォルト                      | 説明                                                                                    |
| :------------------------------------------------------ | :------------------------- | :------------------------------------------------------------------------------------ |
| `--scm-connector-host <host[:port]>`                    | 未設定                        | リクエストを転送する GitHub Enterprise Server ホスト名。ポートはデフォルトで `443` です。このフラグを設定するとコネクタが有効になります。 |
| `--scm-connector-id <n>`                                | `--scm-connector-host` で必須 | 組織の GitHub Enterprise Server 接続の数値 ID。コネクタを有効にするときは、Anthropic アカウントチームに値を問い合わせてください。  |
| `--scm-connector-provider <slug>`                       | `ghe`                      | プロバイダーを識別するパスセグメント。`^[a-z0-9-]{1,32}$` と一致します。                                        |
| `--scm-connector-ca-file <path>`                        | 未設定                        | GitHub Enterprise Server ホストへの TLS 接続用の追加 CA バンドル（PEM 形式）。                            |
| `--scm-connector-host-rewrite <from>=<to_host:to_port>` | 未設定                        | エンドツーエンドテスト専用：ホストヘッダーと TLS SNI を `--scm-connector-host` として保ちながら TCP 接続をリダイレクトします。    |

コネクタはオーケストレーターの既存の環境シークレットで認証し、自動的に再接続します。ドロップされた接続で指数バックオフするか、別のオーケストレーターレプリカが既に保持しているため、コントロールプレーンが接続を閉じるときに固定 30 秒の遅延があります。

<h2 id="environment-variable-only-settings">
  環境変数のみの設定
</h2>

これらのランナー設定は環境からのみ読み取られ、ほとんどのデプロイメントがデフォルトのままにしておく動作をカバーしています。

| 環境変数                                       | デフォルト       | 説明                                                                                                                                                                                                                                                                                                                                                                    |
| :----------------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`    | `30000`     | バックグラウンドタスクが終了した後、その結果を読み取る後続ターンが開始されていない間、ランナーがセッションをビジーと見なす時間。[`--drain-wait-sec` および `--release-idle-session-min` 行](#runner-cli-flags) はドレインおよびアイドルリリース時にホールドが適用される場所を説明し、[ランナーライフサイクル](/docs/ja/self-hosted-environments#runner-lifecycle) は `--retire-at` リタイアメント時に適用される場所を説明しています。`0` または使用不可能な値はデフォルトにフォールバックするため、ホールドをオフにすることはできません。Claude Code v2.1.228 以降が必要です。 |
| `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR`       | `~/.claude` | ランナーのスタートアップスナップショットにキャプチャされ、各セッションの `CLAUDE_CONFIG_DIR` にシードされるディレクトリ。ディスク上の変更はランナーの再起動後に適用されます。変数を設定すると、ランナーが [MCP シーディング](/docs/ja/self-hosted-environments-configuration#mcp-servers) 用に `.claude.json` を読み取る場所も移動します。設定すると、独自のデフォルトを含めて、その検索を再配置します。空のディレクトリを指してシーディングを完全に無効にします。                                                                                     |
| `SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS` | `900000`    | セッションが `--kill-session-after-min` 制限に達した後、実行中のターンが終了するか、リリースが完了するのを待つ時間。その後、ランナーはセッションを終了します。                                                                                                                                                                                                                                                                         |
| `SELF_HOSTED_RUNNER_POST_TURN_SETTLE_MS`   | `7000`      | ターンが終了した後、セッションのプロセスがターンの終了を Anthropic に報告している間、ランナーが `--drain-wait-sec` ドレイン用にセッションをビジーとしてカウントする時間の上限。`0` または使用不可能な値はデフォルトにフォールバックするため、ホールドをオフにすることはできません。Claude Code v2.1.275 以降が必要です。                                                                                                                                                                              |
| `SELF_HOSTED_RUNNER_SIGKILL_GRACE_MS`      | `30000`     | ランナーが割り込み不可能な I/O でスタックしている子に `SIGKILL` を配信するのを待つ時間。その後、ランナー自体が終了します。`--post-session-hook-timeout-sec` プラス 15 秒でフロアされ、`--push-outcome-on-release` が設定されている場合は 30 秒追加されます。有効な最小値はデフォルトで 75 秒です。                                                                                                                                                                        |
| `CLAUDE_RUNNER_FETCH_DEPTH`                | `50`        | 新規クローン用の Git フェッチ深度。正の整数、または完全なフェッチ用に `full` または `0` を設定します。ワークスペースに既に存在するリポジトリは既存の深度を保持します。                                                                                                                                                                                                                                                                          |
| `CLAUDE_RUNNER_SKIP_GIT_VERIFY`            | 未設定         | `1` の場合、`checkout` フック実行後の `.git` 存在チェックをスキップします。フックが非 git ソースを具体化する場合は、これを設定します。                                                                                                                                                                                                                                                                                     |
| `FORCE_AUTOUPDATE_PLUGINS`                 | 未設定         | `1` の場合、バイナリがピン留めされていても、プラグインマーケットプレイスの自動更新を許可します。                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_CODE_DISABLE_ARTIFACT`             | 未設定         | `1` の場合、組織の管理者設定に関係なくセッション内の Artifact ツールを無効にし、`*.frame.claudeusercontent.com` エグレス要件をドロップします。                                                                                                                                                                                                                                                                        |

<h2 id="telemetry">
  テレメトリ
</h2>

セッション子は、オフにしない限り、運用テレメトリを Anthropic に送信します。コードまたはリポジトリコンテンツは送信されません。ランナープロセスでテレメトリ変数を設定します。ランナーはサーバー提供の環境変数を適用した後、それらを再アサートするため、オペレーターの設定は常に優先されます。

1 つのコントロールはセルフホスト環境に固有です。`CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` は Datadog 運用メトリクスにオプトインします。これはセルフホスト環境ではデフォルトでオフです。一般的な Claude Code テレメトリコントロール `DISABLE_TELEMETRY`、`DO_NOT_TRACK`、`DISABLE_ERROR_REPORTING`、`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` は [環境変数リファレンス](/docs/ja/env-vars) に記載されているようにセッション子に適用されます。`DISABLE_GROWTHBOOK` は関連していますが異なります。`DISABLE_GROWTHBOOK=1` を設定するとフィーチャーフラグフェッチが無効になり、`DISABLE_TELEMETRY` も設定されない限りテレメトリはオンのままです。

`CLAUDE_CODE_ENABLE_TELEMETRY` は無関係です。これは [監視](/docs/ja/monitoring-usage) で説明されているように、独自のコレクターへの OpenTelemetry エクスポートを有効にし、Anthropic のアナリティクスを制御しません。

<h2 id="health-endpoint">
  ヘルスエンドポイント
</h2>

ランナーは設定されたヘルスポートで `GET /healthz` を提供します。レスポンスは、プロセスが生存している限り、ポーリングループがどの状態にあるかに関係なく `200 OK` です。したがって、このエンドポイントの HTTP プローブはデッドプロセスのみを検出します。JSON ボディは現在の状態を説明します。

```json theme={null}
{
  "status": "ok",
  "runner_id": "ccrunner_...",
  "active_sessions": 2,
  "last_poll_at": "2026-03-31T18:04:11.220Z",
  "last_poll_age_ms": 842
}
```

カスタムプローブでライブネスシグナルとして `last_poll_age_ms` を使用します。無限に増加する値は、ポーリングループがスタックしていることを示します。`last_poll_at` と `last_poll_age_ms` の両方は、最初のポーリングが完了するまで `null` です。

オーケストレーターはそのヘルスポートで独自の `/healthz` を提供します。そのエンドポイントは常に `200` を返し、ボディは最新のポーリングが成功したかどうかを報告する `connected` フィールドと、`queue_counts` のスポーン キュー数ごとの状態を持ちます。ステータスコードではなく `connected` でレディネスとアラートをゲートします。

[SCM コネクタ](#scm-connector-flags) が設定されている場合、オーケストレーターの `/healthz` ボディは `scm_connector_connected` と、`connected`、`last_connected_at`、`last_error`、`reconnects`、`requests_forwarded` を持つ `scm_connector` オブジェクトも持ちます。`--scm-connector-host` が設定されていない場合、両方のフィールドは `null` です。

<h2 id="prometheus-metrics">
  Prometheus メトリクス
</h2>

各ランナーは `/healthz` と同じポートで `GET /metrics` で Prometheus メトリクスを提供します。主要なシリーズ：

| シリーズ                                                                              | 注記                                                                                                                                                                                                                                                                                                                                       |
| :-------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude_code_self_hosted_runner_info{runner_id,version,client_label}`             | 常に `1`。フリートインベントリとバージョンドリフト検出に有用です。                                                                                                                                                                                                                                                                                                      |
| `claude_code_self_hosted_runner_capacity`                                         | 設定済み `--capacity`                                                                                                                                                                                                                                                                                                                        |
| `claude_code_self_hosted_runner_active_sessions`                                  | 現在実行中のセッション                                                                                                                                                                                                                                                                                                                              |
| `claude_code_self_hosted_runner_locked_account{email}`                            | ランナーがユーザーにロックされ、`act.email` クレームを持つセッショントークンが発行された後に存在します。シリーズは Claude Tag エージェントにロックされたランナーには存在しません。そのセッショントークンは `act.email` を持ちません。ラベル値はアカウントメールです。メトリクスストアが広く読み取り可能な場合は、スクレイプ時にラベルをドロップまたはハッシュします。例えば Prometheus `metric_relabel_configs` を使用します。                                                                                     |
| `claude_code_self_hosted_runner_last_poll_age_seconds`                            | 最後の成功したポーリング以降の秒数。60 を超える場合はアラートします。                                                                                                                                                                                                                                                                                                     |
| `claude_code_self_hosted_runner_poll_errors_total{error_kind}`                    | 種類別の累積 PollWork 失敗：`transport`、`timeout`、`5xx`、`429`、または `4xx`。すべての 5 つのシリーズはプロセス開始から存在します。`rate(...[5m]) > 0` でアラートします。                                                                                                                                                                                                                 |
| `claude_code_self_hosted_runner_sessions_started_total{client_platform}`          | ランナーの生存期間にわたってスポーンされたセッション子プロセス。セッションオリジン（`web_claude_ai`、`ios`、`android`、`desktop_app`、`claude_code_cli` など）ごとに 1 つのシリーズ、またはサーバーが 1 つを送信しなかった場合は `unknown`。Slack セッションは、どの Slack 統合が作成したかに応じて `claude_in_slack` または `claude-in-slack` を持ちます。`{client_platform=~"claude[-_]in[-_]slack"}` などの正規表現セレクターで両方と一致します。フリート合計には `sum()` を使用します。 |
| `claude_code_self_hosted_runner_sessions_completed_total{client_platform}`        | クリーンに終了したセッション。同じ方法でラベル付けされます。より広い範囲：[セッションライフサイクルカウンターセマンティクス](#session-lifecycle-counter-semantics) を参照して、何がカウントされるかを確認してください。                                                                                                                                                                                                         |
| `claude_code_self_hosted_runner_sessions_failed_total{client_platform}`           | 失敗で終了したセッション。同じ注意事項：[セッションライフサイクルカウンターセマンティクス](#session-lifecycle-counter-semantics) を参照してください。                                                                                                                                                                                                                                          |
| `claude_code_self_hosted_runner_sessions_interrupted_total{client_platform}`      | ランナーがセッション結果ではなく運用上の理由で終了したセッション。同じ方法でラベル付けされます。[セッションライフサイクルカウンターセマンティクス](#session-lifecycle-counter-semantics) を参照してください。                                                                                                                                                                                                              |
| `claude_code_self_hosted_runner_initializing_sessions`                            | 初期化フェーズ内のセッション。割り当てから子の初期化イベントまで。                                                                                                                                                                                                                                                                                                        |
| `claude_code_self_hosted_runner_session_init_duration_seconds`                    | セッション初期化期間のヒストグラム                                                                                                                                                                                                                                                                                                                        |
| `claude_code_self_hosted_runner_session_init_errors_total`                        | 初期化に到達する前に失敗したセッション：チェックアウトフック失敗、git 準備、トークン問題、または初期化前の子クラッシュ                                                                                                                                                                                                                                                                            |
| `claude_code_self_hosted_runner_session_start_hook_errors_total`                  | エラー結果を報告した `SessionStart` フック。失敗したフック実行ごとに 1 つ。                                                                                                                                                                                                                                                                                          |
| `claude_code_self_hosted_runner_session_idle_seconds{session_id,client_platform}` | セッションがアイドル状態になってからの秒数のセッションごとのゲージ。未回答の権限プロンプトでスタックしているセッションを終了するのに有用です。                                                                                                                                                                                                                                                                  |

オーケストレーターは `/healthz` と同じポートで `GET /metrics` で独自のシリーズを提供します。

| シリーズ                                                                                    | 注記                                                                                                                                                                                                    |
| :-------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude_code_self_hosted_orchestrator_info{version,pool_id,orchestrator_uuid,hostname}` | 常に `1`                                                                                                                                                                                                |
| `claude_code_self_hosted_orchestrator_connected`                                        | 最新のポーリングが成功した場合は `1`。失敗の種類に関係なく、失敗したポーリング後は `0` にドロップします。                                                                                                                                             |
| `claude_code_self_hosted_orchestrator_last_poll_age_seconds`                            | 最後のポーリング試行（成功または失敗）以降の秒数。最後の成功以降を測定するランナーの同じ名前のメトリクスとは異なります。`connected` とペアにして、失敗したポーリングをキャッチします。オーケストレーターのポーリングループはフック実行で待機するため、デフォルトで約 90 秒の `--hook-timeout` プラス余裕の上でアラートします。フラット 60 ではなく。         |
| `claude_code_self_hosted_orchestrator_poll_errors_total{error_kind}`                    | 種類別の累積 PollSpawnHints 失敗：`transport`、`timeout`、`5xx`、`429`、または `4xx`。すべての 5 つのシリーズはプロセス開始から存在します。`rate(...[5m]) > 0` でアラートします。                                                                        |
| `claude_code_self_hosted_orchestrator_queue_pending_sessions`                           | 今すぐクレーム可能なスポーン要求                                                                                                                                                                                      |
| `claude_code_self_hosted_orchestrator_queue_backing_off_sessions`                       | 再試行可能なフック失敗後の再試行バックオフ内のスポーン要求                                                                                                                                                                         |
| `claude_code_self_hosted_orchestrator_queue_circuit_broken_sessions`                    | Owner が環境の **Activity** タブから再試行するまでブロックされたスポーン要求。ゼロを超える場合はアラートします。                                                                                                                                    |
| `claude_code_self_hosted_orchestrator_pool_pending_sessions`                            | この環境でランナーを待機している総セッション数。環境全体の集計。すべてのオーケストレーターインスタンスで同一です。インスタンス全体で `SUM` ではなく `MAX` を使用します。                                                                                                           |
| `claude_code_self_hosted_orchestrator_pool_active_sessions`                             | この環境内のアライブランナーに現在割り当てられているセッション。環境全体の集計。すべてのオーケストレーターインスタンスで同一です。インスタンス全体で `SUM` ではなく `MAX` を使用します。                                                                                                   |
| `claude_code_self_hosted_orchestrator_spawn_hooks_total{result}`                        | 累積 `spawn-runner` フック結果：`ok`、`retryable`、`non_retryable`。オーケストレーターフック呼び出しをカウントします。ランナーがスポーンするセッション子ではありません。容量が 1 を超える場合、ウォームプール、同じセッション用に再度スポーンされたランナーは `sessions_started_total` と比較できないため、2 つが異なります。 |
| `claude_code_self_hosted_orchestrator_spawn_hook_duration_seconds`                      | フック期間のヒストグラム                                                                                                                                                                                          |
| `claude_code_self_hosted_orchestrator_warm_hints_dispatched_total`                      | プロセス開始以降にディスパッチされたスタンバイスポーン要求                                                                                                                                                                         |
| `claude_code_self_hosted_orchestrator_session_queue_wait_seconds`                       | 各セッションがオーケストレーターがスポーン用にクレームするまでキューで待機した秒数のヒストグラム。コントロールプレーンが各セッションのスポーン要求で送信するキュー待機タイムスタンプから記録されます。p50/p99 キュー時間アラートに使用します。プレウォーミングスポーンはサンプリングされません。                                                  |
| `claude_code_self_hosted_orchestrator_clock_skew_seconds`                               | ローカルマイナスサーバークロックスキュー。診断。測定後に存在します。                                                                                                                                                                    |
| `claude_code_self_hosted_orchestrator_scm_connector_connected`                          | [SCM コネクタ](#scm-connector-flags) の WebSocket がオープンしている場合は `1`。ダイアルまたはバックオフ中は `0`。`--scm-connector-host` が設定されていない場合は存在しません。                                                                           |
| `claude_code_self_hosted_orchestrator_scm_connector_requests_forwarded_total`           | プロセス開始以降に設定された SCM ホストにプロキシされた累積 HTTP リクエスト。`--scm-connector-host` が設定されていない場合は存在しません。                                                                                                                |

オートスケーリングの場合、スケーリングスタイルに一致するシリーズを選択し、スケーラーに供給する前にゲートします。

* **キュー深度スケーリング**：`queue_pending_sessions` ではなく `claude_code_self_hosted_orchestrator_pool_pending_sessions` を HPA または KEDA スケーラーに供給します。
* **容量スケーリング**：ランナーの `active_sessions` と `capacity` の比率でスケーリングします。
* **`connected` でゲート**：インスタンスごとに `claude_code_self_hosted_orchestrator_connected == 1` でクエリをフィルタリングします。切断されたレプリカの古い値がスケーラーに供給されないようにします。

完全なポーリング停止中、すべてのレプリカが切断されると、ゲートされたクエリはデータを返しません。HPA は欠落メトリクスで現在のレプリカ数を保持しますが、KEDA の Prometheus スケーラーはデフォルト `ignoreNullValues: "true"` で空の結果をゼロとして読み取り、スケールインします。ScaledObject で `ignoreNullValues: "false"` を設定し、オプションで `fallback` レプリカフロアを設定します。

次の Prometheus Operator `PodMonitor` は両方のプロセスをカバーしています。`app.kubernetes.io/part-of: claude-code-self-hosted-runner` ラベルと、[Kubernetes レシピ](/docs/ja/self-hosted-environments-deploy#kubernetes) が設定する名前付き `health` ポートでポッドを選択します。デプロイメントに合わせて名前空間を調整します。

```yaml theme={null}
# Claude Code セルフホストランナー + オーケストレーター用の Prometheus Operator PodMonitor の例。
# 名前空間とラベルセレクターをデプロイメントに合わせて調整します。
# ランナーとオーケストレーターの両方は、その --health-port（デフォルト 8080）で /metrics を提供します。
apiVersion: monitoring.coreos.com/v1
kind: PodMonitor
metadata:
  name: claude-code-self-hosted-runner
  namespace: monitoring
spec:
  namespaceSelector:
    matchNames:
      - claude-runners
  selector:
    matchExpressions:
      # Kubernetes レシピからのランナー Deployment、および同じ方法でラベル付けされた
      # オンデマンドランナー Job とオーケストレーターポッド、および名前付き 'health' containerPort を持つものと一致します。
      - key: app.kubernetes.io/part-of
        operator: In
        values: [claude-code-self-hosted-runner]
  podMetricsEndpoints:
    - port: health
      path: /metrics
      interval: 30s
```

これらのサンプルアラートルールは出発点です。フリートサイズのしきい値を調整します。

```yaml theme={null}
# Claude Code セルフホストランナー + オーケストレーター用の Prometheus アラートルールの例。
# フリートサイズと SLO のしきい値を調整します。
groups:
  - name: claude-code-self-hosted-runner
    rules:
      - alert: ClaudeRunnerPollStale
        expr: claude_code_self_hosted_runner_last_poll_age_seconds > 60
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "ランナー {{ $labels.pod }} は 60 秒以上ポーリングしていません"
      - alert: ClaudeRunnerVersionDrift
        expr: count(count by (version) (claude_code_self_hosted_runner_info)) > 1
        for: 30m
        labels: {severity: info}
        annotations:
          summary: "ランナーは混合バージョンを実行しています"
      - alert: ClaudeRunnerInitErrorsHigh
        expr: increase(claude_code_self_hosted_runner_session_init_errors_total[10m]) > 3
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "ランナー {{ $labels.pod }}：10 分間に 3 回以上のセッション初期化失敗（チェックアウトフック / git / トークン / 初期化前クラッシュ）"
      - alert: ClaudeRunnerPollErrors
        expr: sum by (pod) (rate(claude_code_self_hosted_runner_poll_errors_total[5m])) > 0
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "ランナー {{ $labels.pod }}：PollWork が失敗しています（5 分間に {{ $value | humanize }}/s）"
      - alert: ClaudeRunnerSessionStartHookErrors
        expr: increase(claude_code_self_hosted_runner_session_start_hook_errors_total[10m]) > 3
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "ランナー {{ $labels.pod }}：10 分間に 3 回以上の SessionStart フック失敗"

  - name: claude-code-self-hosted-orchestrator
    rules:
      - alert: ClaudeOrchestratorDisconnected
        expr: claude_code_self_hosted_orchestrator_connected == 0
        for: 2m
        labels: {severity: critical}
        annotations:
          summary: "オーケストレーター {{ $labels.pod }} は Anthropic コントロールプレーンに到達できません"
      - alert: ClaudeOrchestratorPollStale
        expr: claude_code_self_hosted_orchestrator_last_poll_age_seconds > 90
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "オーケストレーター {{ $labels.pod }} は 90 秒以上ポーリングしていません（ポーリングループはフック実行で待機）"
      - alert: ClaudeOrchestratorCircuitBroken
        expr: claude_code_self_hosted_orchestrator_queue_circuit_broken_sessions > 0
        for: 1m
        labels: {severity: critical}
        annotations:
          summary: "{{ $value }} セッションがサーキットブレーク — spawn-runner フックが繰り返し非再試行可能。インフラを修正してから Activity タブから再試行してください"
      - alert: ClaudeOrchestratorPollErrors
        expr: sum by (pod) (rate(claude_code_self_hosted_orchestrator_poll_errors_total[5m])) > 0
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "オーケストレーター {{ $labels.pod }}：PollSpawnHints が失敗しています（5 分間に {{ $value | humanize }}/s）"
      - alert: ClaudeOrchestratorSpawnHookFailing
        expr: sum by (pod) (increase(claude_code_self_hosted_orchestrator_spawn_hooks_total{result!="ok"}[5m])) > 3
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "オーケストレーター {{ $labels.pod }}：5 分間に 3 回以上の spawn-runner フック失敗"
```

<h3 id="pass-through-session-child-metrics">
  セッション子メトリクスをパススルーする
</h3>

各セッションは独自の子プロセスで実行され、独自の OpenTelemetry メトリクスを持ちます。`--capacity` が 1 を超える場合、ランナーはそれらの子メトリクスの公開方法を書き換えます。ランナーホストで `OTEL_METRICS_EXPORTER=prometheus` を設定し、セッションの環境で `CLAUDE_CODE_ENABLE_TELEMETRY=1` を設定します。例えば、[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts) またはセッションが継承するランナー独自の環境から、各子のカウンターとゲージ計器をランナー独自の `/metrics` エンドポイントで再公開します。ランナーのシリーズと並んで。ランナーは子のエクスポーターを書き換えて、ヘルスポートのループバックのみのレシーバーに OTLP 経由でプッシュし、各シリーズに `session_id` および `client_platform` ラベルでタグ付けし、セッションが終了するとセッションのシリーズを削除します。ヒストグラムはパススルーしません。ランナー独自のプレフィックスと衝突する子メトリクスの名前は削除されます。

デフォルトの `--capacity 1` では、書き換えは適用されません。セッションの子は通常どおりポート 9464 で独自の Prometheus エンドポイントをバインドします。

<h3 id="session-lifecycle-counter-semantics">
  セッションライフサイクルカウンターセマンティクス
</h3>

`sessions_started_total`、`sessions_completed_total`、`sessions_failed_total`、`sessions_interrupted_total` カウンターは、各セッションがどのように終了したかで分類します。スポーンされたすべてのセッション子はスポーン時に `sessions_started_total` をインクリメントし、終了時に他の 3 つのうち正確に 1 つをインクリメントします。したがって、`sessions_started_total` から他の 3 つの合計を引いたものは、現在実行中のセッション子の数に等しくなります。

* `completed`：セッションはクリーンに終了しました。これは子がコード `0` で独自に終了する場合、セッションが子がまだ接続されている間にアーカイブまたは削除される場合、およびランナーがスロットをクリーンハンドオフとしてリリースする場合をカバーします。アイドルタイムアウト、リタイア時間、または `--kill-session-after-min` 制限でのリリース、スタートアップタイムアウト、またはポーリングループが子が終了する前に気付いたサーバー側の割り当て解除。`sessions_completed_total` をインクリメントします。
* `failed`：子がゼロ以外のコードで独自に終了しました。クラッシュまたはスポーン後のセットアップ失敗のいずれか。`sessions_failed_total` をインクリメントします。
* `interrupted`：ランナーがセッション成功またはランナー障害のいずれでもない運用上の理由で子を終了しました。たとえばドレインや、`--kill-session-after-min` 制限後の [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](#environment-variable-only-settings) グレースウィンドウが終了した時点でまだランナー上にあったセッションの終了などです。Kubernetes ローリング再起動が `SIGTERM` を送信することは、ドレインの一例です。`sessions_interrupted_total` をインクリメントします。

v2.1.260 より前では、ランナーは `--kill-session-after-min` 制限に達したすべてのセッションを終了し、`sessions_interrupted_total` でカウントしました。

[`post-session` フック](/docs/ja/self-hosted-environments-configuration#post-session) の `CLAUDE_RUNNER_EXIT_REASON` はクリーンハンドオフを異なる方法で分類します。フックはリリース、スタートアップタイムアウト、サーバー割り当て解除をランナーが子を停止したため `interrupted` として報告します。これらのカウンターは、スロットがクリーンに返されたため、`completed` として同じイベントを記録します。

フック受信を `sessions_completed_total` に対して直接調整する場合、完了をアンダーカウントします。セッションごとの保証にはフックを使用し、集計レートにはカウンターを使用します。

ワンショット環境では、`--capacity 1` とデフォルト `--drain-grace-sec 0` で、各ランナープロセスは 1 つのセッションが終了した直後に終了します。`sessions_completed_total`、`sessions_failed_total`、`sessions_interrupted_total` はセッション終了時にのみインクリメントされます。その終了の直前に、Prometheus スクレイプが 15 ～ 60 秒ごとの場合、ランナーのシリーズが消える前にインクリメントをキャッチすることはめったにありません。これら 3 つのセッション終了カウンターは、このセクションの残りが参照するターミナルカウンターです。`sessions_started_total` はスポーン時にインクリメントされ、セッションの生存期間中は表示されたままなので、確実に表示されます。ただし、ワンショット環境では、累積カウントよりも「現在実行中のセッション」に近く読み取られます。

対応する目標の代わりにこのテーブルのシリーズを使用します。

| 目標     | 使用                                                                                                                                                                                                                                                                                                                  |
| :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| スループット | `claude_code_self_hosted_orchestrator_spawn_hooks_total{result="ok"}`。成功した `spawn-runner` フック呼び出しごとに 1 回インクリメントされ、`rate()` の下で意味のある長寿命オーケストレーター上のカウンター。フック呼び出しをカウントします。ランナーがスポーンするセッションではなく、プレウォーミングと同じセッション用の繰り返しスポーンはセッションカウントから異なります。                                                                            |
| 利用率    | `sum(claude_code_self_hosted_runner_active_sessions)` 対 `sum(claude_code_self_hosted_runner_capacity)`。ランナーの生存期間に関係なく、すべてのスクレイプで有効なゲージ。                                                                                                                                                                             |
| バックログ  | キュー深度の `claude_code_self_hosted_orchestrator_pool_pending_sessions`、および `claude_code_self_hosted_orchestrator_queue_circuit_broken_sessions`。ゼロを超える場合はアラートします。                                                                                                                                                      |
| 失敗     | `claude_code_self_hosted_runner_sessions_failed_total`。ベストエフォート。スポーン後の実際のクラッシュはインクリメントされ、`rate()` はセッションを超えて生存するランナーで意味があります。`--drain-grace-sec` が `0` を超える場合。ワンショット環境は他のターミナルカウンターと同じスクレイプウィンドウ問題を持つため、表示される非ゼロ値を調査する価値があるものとして扱います。スポーン前の失敗（チェックアウトフック失敗、git 準備、トークン問題など）は `session_init_errors_total` にのみ表示されます。 |

`orchestrator_*` 行は [オンデマンドオーケストレーター](/docs/ja/self-hosted-environments-configuration#on-demand-runners) を実行している環境にのみ存在します。セッションを超えて生存するランナーを持つ固定フリートで、`--drain-grace-sec` が `0` を超える場合、スループットに `sum(rate(claude_code_self_hosted_runner_sessions_started_total[5m]))` を使用します。ワンショットフリートではそのシリーズは他のターミナルカウンターと同じスクレイプウィンドウ問題を持つため、キューに入ったセッション数に依存します。バックログを環境の **Activity** タブで確認します。[**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments)：ランナーはキュー深度シリーズをエクスポートしません。

セッションごとの結果報告については、代わりに [`post-session` フック](/docs/ja/self-hosted-environments-configuration#post-session) を使用してください。VM プリエンプションなどの突然のランナー終了を除き、子プロセスがスポーンされたすべてのセッション終了で発火します。[フック独自の契約](/docs/ja/self-hosted-environments-configuration#post-session) に従って。

<h2 id="what’s-next">
  次のステップ
</h2>

* [セルフホスト環境](/docs/ja/self-hosted-environments)：環境、ランナー、セッションモデル。[クイックスタート](/docs/ja/self-hosted-environments-quickstart) と [本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy) はセットアップと運用を保持しています。
* [セッションをカスタマイズする](/docs/ja/self-hosted-environments-configuration)：ラッパースクリプト、ライフサイクルフック、オンデマンドランナー。
* [セッション ID を検証する](/docs/ja/self-hosted-environments-identity)：セッショントークン、そのクレーム、検証方法。
