> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude apps gateway のデプロイと運用

> IdP にゲートウェイを登録し、コンテナをビルドして Kubernetes または Cloud Run にデプロイし、ヘルスチェック、シークレットローテーション、アップグレード、セキュリティを運用します。

このページでは、[Claude apps gateway](/docs/ja/claude-apps-gateway) の運用側について説明します。ID プロバイダー（IdP）で OAuth クライアントを登録し、ゲートウェイをコンテナとしてデプロイし、日々運用します。ゲートウェイが起動時に読み込む `gateway.yaml` ファイルのすべてのオプションについては、[設定リファレンス](/docs/ja/claude-apps-gateway-config) を参照してください。

本番環境のデプロイメントは順序立てた 4 つのステップに従い、以下のセクションがそれに対応しています。最初の 2 つは選択を行う場所です。後の 2 つは、実行中に参照するリファレンス資料です。

1. [ID プロバイダーをセットアップする](#identity-provider-setup)：OAuth クライアントを登録し、Okta、Entra、Google の IdP 固有の注記を確認します
2. [ゲートウェイをデプロイする](#deployment)：ピン留めされたコンテナイメージをビルドし、Kubernetes、Cloud Run、または独自のプラットフォームで実行します。このセクションではコスト、バイパス、複数ゲートウェイ、サーバーレスの決定についても説明します
3. [運用をセットアップする](#operations)：ログ、ヘルスプローブ、障害時の動作、シークレットローテーション、アップグレード。監視とランブックを配線するときに参照するリファレンス
4. [セキュリティ体制を確認する](#security)：データがどこを流れるか、脅威モデル、コンプライアンスの回答。セキュリティレビュー用のリファレンス

サインインまたはブート中に失敗が発生した場合は、[トラブルシューティング](#troubleshooting) に直接進んでください。これは表示されるエラーに基づいてキー付けされています。

<Note>
  **プライベートネットワークにデプロイします。** Claude Code は、アドレスがプライベートであるゲートウェイにのみ接続します。これはセキュリティガードです。信頼されたゲートウェイは、開発者マシンでコマンドを実行する設定をプッシュできるためです。ゲートウェイを内部ロードバランサーまたは VPN の背後に配置し、プライベート IP にのみ解決するホスト名を付与します。内部ネットワークが組織が所有するパブリック IPv4 スペースから番号付けされている場合は、[所有するパブリックアドレススペースでゲートウェイを許可する](/docs/ja/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) を参照してください。
</Note>

<h2 id="identity-provider-setup">
  ID プロバイダーのセットアップ
</h2>

単一のリダイレクト URI `https://<gateway>/oauth/callback` を持つ機密 OAuth/OpenID Connect（OIDC）ウェブアプリケーションを ID プロバイダーに登録し、ゲートウェイアクセスを持つべきユーザーまたはグループに割り当てます。

任意の OIDC 準拠 IdP が機能します：Okta、Microsoft Entra ID、Google Workspace、Keycloak、Dex、PingFederate など。IdP は 3 つの要件を満たす必要があります：

* `/.well-known/openid-configuration` を提供します。本番環境では HTTPS 経由です。ゲートウェイは [`http://` issuer](/docs/ja/claude-apps-gateway-config#oidc) を受け入れ、ループバック issuer は追加で `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` が必要です
* 認可コードフローをサポートします。PKCE（Proof Key for Code Exchange）はデフォルトで有効です。サポートしない IdP の場合は `oidc.use_pkce: false` で無効にします
* id\_token で `email` と必要に応じて `groups` を返すか、`oidc.userinfo_fallback: true` で userinfo エンドポイントから提供します

プライベート PKI の場合は、`oidc.ca_cert_pem` を設定します。

いくつかのプロバイダーはメールとグループクレームを異なる方法で処理します：

* **Okta**：`https://example.okta.com` の org 認可サーバーは、`email` と `groups` を省略した薄い id\_token を返すため、issuer として使用する場合は常に `oidc.userinfo_fallback: true` を設定します。`https://example.okta.com/oauth2/default` などのカスタム認可サーバーは、id\_token に `email` と必要に応じて `groups` を含め、直接発行し、フォールバックは不要です。Okta は `oidc.scopes` で `groups` スコープがリクエストされ、アプリのグループクレームフィルターが許可する場合にのみ `groups` を発行します。`userinfo_fallback` は IdP がリクエストされなかったクレームを埋めることはできません。
* **Microsoft Entra ID**：`issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`。Entra はグループ名ではなくグループオブジェクト ID を発行するため、`managed.policies.match.groups` で GUID を使用するか、人間が読める名前のためにアプリロールを使用します。テナントが `groups` の代わりに `roles` の下でロールを発行する場合は、`oidc.groups_claim: roles` を設定します。
* **Google Workspace**：`issuer` = `https://accounts.google.com`。Google の id\_token はグループを含みません。Google を IdP として、グループベースの `allowed_groups` または `managed.policies` を使用するには、[`oidc.google_groups`](/docs/ja/claude-apps-gateway-config#oidc) を設定します。これは、ドメイン全体の委任を持つサービスアカウントを使用して Admin SDK Directory API を通じて各ユーザーのグループを検索します。これなしで、メンバーシップゲーティングに `oidc.allowed_email_domains` を使用し、ポリシー割り当てに `managed.policies.match.email_domain` を使用します。Google は標準の `offline_access` スコープも無視します。リフレッシュトークンの場合は、`oidc.scopes: [openid, profile, email]` と `oidc.extra_auth_params: { access_type: offline, prompt: consent }` を設定します。

<Warning>
  リフレッシュトークンにより、ゲートウェイは開発者のセッションをサイレントに更新でき、開発者をブラウザに戻す必要がありません。また、IdP がユーザーを無効にすると、次のリフレッシュが失敗し、セッションは `ttl_hours` 内に終了するため、プロビジョニング解除を駆動します。ゲートウェイはデフォルトでリフレッシュトークンを取得するために `offline_access` をリクエストします。IdP がオフラインアクセスに明示的な同意を必要とする場合は、OAuth クライアントを設定してそれを許可します。

  IdP がリフレッシュトークンをまったく発行できない場合、ゲートウェイは引き続き機能しますが、サイレント更新がないため、開発者はセッションの有効期限が切れるとブラウザログインを再実行します。これが 1 時間ごとに発生するのを防ぐには、[`session.ttl_hours`](/docs/ja/claude-apps-gateway-config#session) を `8` または `12` に上げます。トレードオフはプロビジョニング解除の遅延です。リフレッシュトークンなしでは、無効にされたユーザーはより長い TTL が経過するまでアクセスを保持します。
</Warning>

<h2 id="deployment">
  デプロイ
</h2>

ゲートウェイは単一のステートレス Linux バイナリで、Postgres を通じて調整されるため、環境内でステートレスサービスをデプロイする方法でデプロイします。ネットワーク内に保持し、開発者と IdP が HTTPS 経由で到達でき、本番認証情報を保持する他のサービスと同様に扱います。

デプロイメントを実行する場所を超えて形作るいくつかの決定があります：

* **コスト**：ゲートウェイの個別ライセンスまたはシートごとの料金はありません。ゲートウェイは `claude` バイナリの一部であるため、既存のコミットメントを通じて推論に対して支払い、実行するコンピュートに対して支払います。
* **バイパス**：ゲートウェイは、モデルへの唯一のルートがそれを通過することを強制しません。独自の認証情報を持つ開発者は引き続きプロバイダーを直接呼び出すことができるため、そのパスを閉じることはネットワークポリシーの決定です。例えば、`api.anthropic.com` へのエグレスをゲートウェイ以外からブロックします。そのエグレスをブロックすると、各開発者のマシンから `api.anthropic.com` を呼び出す [WebFetch ドメインセーフティチェック](/docs/ja/data-usage#webfetch-domain-safety-check) も破壊されます。管理ポリシーで `skipWebFetchPreflight: true` を設定して無効にします。
* **複数ゲートウェイ**：各ゲートウェイは独自の設定を持つ個別のデプロイメントで、CLI はゲートウェイホスト名ごとに信頼と認証情報を保存するため、チームは競合なしに異なるゲートウェイを使用できます。複数の OIDC issuer を提供するには、個別のインスタンスを実行します。
* **サーバーレス**：Cloud Run は機能します。`min-instances: 1` を設定して、コールド OIDC ディスカバリーを回避します。Lambda と Cloud Functions は機能しません。ゲートウェイは長時間実行 HTTP サーバーであるためです。

ここのすべての本番トポロジーは、L7 プロキシ（Ingress、Cloud Run のフロントエンド、ALB など）をプレーン HTTP レプリカの前に配置します。[`listen.trusted_proxies`](/docs/ja/claude-apps-gateway-config#listen) をプロキシのソース範囲に設定して、ゲートウェイが `X-Forwarded-For` からクライアント IP を読み込みます。ゲートウェイは TCP ピアが信頼されている場合にのみヘッダーを尊重します。[Google Cloud](/docs/ja/claude-apps-gateway-on-gcp) と [AWS](/docs/ja/claude-apps-gateway-on-aws) の実装例には、トポロジーごとに具体的な値があります。信頼されたプロキシなしでは、すべてのリクエストはプロキシの IP から来ているように見え、IP ごとのレート制限を 1 つの共有バケットに折りたたみ、監査イベントにプロキシの IP を記録します。

ゲートウェイのデバイス認可とトークンエンドポイントへのリクエストをリダイレクトしないでください。例えば、HTTP から HTTPS へのリダイレクトやホスト正規化の書き換えなどです。Claude Code はこれらのリクエストでリダイレクトに従わないため、ingress ルールがそれらをリダイレクトするとサインインとトークン更新が破壊されます。

プロキシに、ゲートウェイのキープアライブ間隔より長いアイドルタイムアウトを与えます。これは上流に依存します：

* `provider: anthropic` 以外のすべての上流では、ゲートウェイはストリームが約 15 秒間サイレント状態になった後、SSE `ping` を 1 回書き込みます。
* `provider: anthropic` では、ゲートウェイは Anthropic API 独自の ping を含む応答をそのまま渡します。

ALB の 60 秒などのデフォルトは、静かなストリームを開いたままにするのに十分です。[AWS の実装例](/docs/ja/claude-apps-gateway-on-aws#troubleshooting) はとにかくそれを 1 時間に引き上げ、トラブルシューティング行は v2.1.229 より古いゲートウェイをカバーしており、現在 ping を取得する上流でサイレント期間中に何も送信しませんでした。

<h3 id="container-image">
  コンテナイメージ
</h3>

標準 Claude Code リリースのネイティブ `claude` バイナリの周りに独自のイメージをビルドします：

1. ピン留めされたリリースからイメージアーキテクチャの Linux ビルドをダウンロードします。ダウンロード URL については、[特定のバージョンをインストールする](/docs/ja/setup#install-a-specific-version) を参照してください。
2. [バイナリの整合性とコード署名](/docs/ja/setup#binary-integrity-and-code-signing) で説明されているように、リリースの GPG 署名付き `manifest.json` に対して検証します。
3. ビルドコンテキストにコピーします。

ビルドがリリースホストに到達できない場合は、リリースを内部レジストリにミラーリングし、フロートが実行するバージョンをピン留めします。

バイナリを超えて、イメージは以下が必要です：

* **glibc ベースのイメージ**：glibc ビルドの唯一の動的依存関係は glibc ライブラリです。Musl ベースのイメージは `linux-x64-musl` または `linux-arm64-musl` ビルドと追加パッケージが必要です。[Alpine Linux セットアップ](/docs/ja/setup#alpine-linux-and-musl-based-distributions) を参照してください。
* **書き込み可能な状態ディレクトリ**：ゲートウェイは任意のユーザーとして実行されますが、最小限のイメージには書き込み可能なホームがありません。`CLAUDE_CONFIG_DIR` を `/tmp/.claude` などの書き込み可能なパスに設定します。
* **コンテナコマンド**：`claude gateway --config /etc/claude/gateway.yaml`。設定ファイルは読み取り専用でマウントされ、シークレットは環境変数として提供されます。ゲートウェイは `listen.port` でリッスンします。デフォルトは `8080` です。

<h3 id="kubernetes">
  Kubernetes
</h3>

ゲートウェイをデプロイメントとして実行します。他のステートレスサービスと同様です：

* ConfigMap から設定をマウントし、Secret からシークレットをマウントします。YAML で `${file:/path/to/secret}` または環境変数を通じてシークレットを参照します
* Ingress で TLS を終了し、`listen.public_url` を Ingress ホスト名に設定します
* readiness プローブを `GET /readyz` に、liveness プローブを `GET /healthz` に指定します

AWS での完全な実装例（ECS Fargate または EKS、Amazon RDS、AWS Secrets Manager をカバー）については、[AWS にデプロイする](/docs/ja/claude-apps-gateway-on-aws) を参照してください。

静的キーよりもプラットフォームのワークロードアイデンティティを優先します。[`upstreams` リファレンス](/docs/ja/claude-apps-gateway-config#upstreams) にはプラットフォームごとのセットアップ詳細があります。クロスクラウドペアリング（例えば GKE 上の Amazon Bedrock 上流）の場合は、上流の `auth` ブロックで明示的な認証情報を設定します。

<h3 id="cloud-run">
  Cloud Run
</h3>

サービスを以下のように設定します：

* `listen.port` をデフォルトの `8080` のままにします。これは Cloud Run のデフォルト `PORT` と一致するか、`port: ${PORT}` を設定します
* `public_url` を外部到達可能なオリジンに設定します。本番環境では、これは通常、内部ロードバランサーのホスト名です。`/login` は [パブリックアドレスを拒否](/docs/ja/claude-apps-gateway#prerequisites) し、`*.run.app` URL はそれに解決するため、Cloud Run URL だけは `curl` またはブラウザスモークテストにのみ機能します。例外は、`*.run.app` が Private Service Connect と Cloud DNS プライベートゾーンを通じてプライベートに解決するネットワークです。そのトポロジーでは、Cloud Run URL は有効な `public_url` です。[Google Cloud の実装例](/docs/ja/claude-apps-gateway-on-gcp#deploy-the-gateway) は両方をカバーしています。
* シークレットボリュームとして設定をマウントします
* `min-instances: 1` を設定して、最初のリクエストでコールド OIDC ディスカバリーを回避します

Google Cloud での完全な実装例（Cloud Run または GKE、Cloud SQL、Secret Manager をカバー）については、[Google Cloud にデプロイする](/docs/ja/claude-apps-gateway-on-gcp) を参照してください。

<h3 id="push-the-gateway-url-to-developer-machines">
  ゲートウェイ URL を開発者マシンにプッシュする
</h3>

ゲートウェイがサービスを提供したら、MDM を通じて、または OS ごとの `managed-settings.json` を直接書き込むことで、管理設定を通じて各開発者のマシンに `forceLoginMethod`、`forceLoginGatewayUrl`、および `parentSettingsBehavior: "merge"` をプッシュします。これなしでは、`/login` はゲートウェイオプションなしで標準アカウントピッカーを表示します。

キーをデプロイしたら、Claude Code はマシン上の残存する API キーまたは claude.ai ログインの使用を停止するため、サインイン指示と一緒にプッシュを計画します。[Administrator policy requires a Cloud gateway sign-in](/docs/ja/errors#administrator-policy-requires-a-cloud-gateway-sign-in) は開発者が見るメッセージについて説明しています。

各メカニズムがポリシーを保存する場所については [where each mechanism stores the policy](/docs/ja/managed-settings#where-each-mechanism-stores-the-policy) を参照し、Claude Desktop `bootstrapUrl` 相当については [Client-side managed settings](/docs/ja/claude-apps-gateway-config#client-side-managed-settings) を参照してください。

<h3 id="large-rollouts">
  大規模なロールアウト
</h3>

サインインはクライアント IP アドレスごとにレート制限されており、デフォルトは小規模なチームに適しています。各アドレスは 10 分ごとに 30 回のサインイン開始と 10 回のコード送信を取得します。数千人の開発者へのロールアウトは、次の 2 つの理由のいずれかで、最初の朝にこれらの制限に達する可能性があります：

* **ゲートウェイはロードバランサーを超えて見ることができません。** [`listen.trusted_proxies`](/docs/ja/claude-apps-gateway-config#listen) がない場合、すべての開発者はロードバランサーのアドレスから来ているように見え、1 つの制限を共有します。他の何よりも先にそれを設定します。ゲートウェイは、`X-Forwarded-For` ヘッダーを無視する最初の時間に警告をログに記録します。
* **多くの開発者が少数の NAT または VPN エグレスアドレスを共有しています。** `trusted_proxies` が正しい場合でも、それらのアドレスの制限を共有します。[`rate_limits`](/docs/ja/claude-apps-gateway-config#http-tuning) を引き上げて適合させます。

`max` のサイズを決定するには、開発者をそれらが共有するエグレスアドレスで割ります。1 つの `window_seconds` 期間内にそれらのうち何人がサインインするかを推定します。デフォルトは 10 分です。その後、リトライと Claude Code と Claude Desktop の両方にサインインする開発者をカバーするために 2 倍にします。

例えば、10,000 人の開発者が 4 つのエグレスアドレスの背後にあり、1 時間にわたって均等にサインインします。これは、アドレスごとに 2,500 人の開発者で、各 10 分ごとに約 420 人です。これを 2 倍にして 1,000 に切り上げます。以下の例は両方の制限を 1,000 に設定します：

```yaml theme={null}
rate_limits:
  device_authorization: { max: 1000, window_seconds: 600 }
  device_verify: { max: 1000, window_seconds: 600 }
```

`device_verify` は、別の開発者のサインインコードを推測するのを防ぐものであるため、推定が必要な限りだけそれを引き上げます。これらの制限でも、コードは 20 文字のアルファベットから 8 文字で、10 分後に期限切れになるため、推測は実用的なままです。[User-code brute-force resistance](#user-code-brute-force-resistance) を参照してください。

IdP がリフレッシュトークンを発行する場合、Claude Code はセッションをサイレントに更新するため、ロールアウト後に制限を戻すことができます。リフレッシュトークンがない場合、開発者は [`session.ttl_hours`](/docs/ja/claude-apps-gateway-config#session) ごとに再度サインインします。その定常状態レートの両方の制限のサイズを決定し、それらを引き上げたままにします。

制限に達すると、Claude Code v2.1.274 以降は `The gateway is limiting sign-in attempts right now` を表示します。v2.1.274 以降のゲートウェイは、検証ページに `Too many attempts came from your network address` を表示し、確認する設定を表示します。また、変更する設定に名前を付ける `sign-in refused` ログ行も書き込みます。

<h2 id="operations">
  運用
</h2>

ゲートウェイがトラフィックを提供したら、日々の運用はそのログを読み込み、その健全性をプローブし、スケジュールに従ってシークレットをローテーションすることです。サブセクションは各々をカバーし、Postgres が保持するもの、アップグレードとロールバックがどのように動作するかをカバーします。

<h3 id="logs">
  ログ
</h3>

ゲートウェイは stderr に 2 つのストリームを書き込みます。両方とも JSON フレンドリーです：

* **監査イベント**：セキュリティ関連イベントごとの単一行 JSON。stderr をログアグリゲーターにパイプします。

  発行されるイベントには `config.load`、`session.mint`、`session.refresh`、`device.authorize`、`device.verify`、`device.callback`、`auth.denied`、`access.denied`、`access.public_client`、`inference`、`managed.serve`、`desktop_bootstrap.serve`、`desktop_bootstrap.denied`、`spend.blocked`、`admin.denied`、`admin.limit.upsert`、`admin.limit.delete` が含まれます。フィールドはイベントによって異なります：

  * 成功した mint と refresh イベントは `sub`、`email`、`client_ip`、結果を含みます
  * `auth.denied` と `access.denied` は理由とクライアント IP を含み、`auth.denied` ではリクエストパスも含みます。拒否時にはユーザーアイデンティティが存在しないため。2 つの `access.denied` 理由はイベントが何を含むかを変更します：
    * `xff_unparseable`：イベントは読み込めなかった `X-Forwarded-For` エントリも含みます
    * `client_ip_unknown`：イベントはクライアント IP を含みません。接続にピアアドレスがなく、`access_control` リストが設定されていたため
  * `access.public_client` は、`access_control.allow_cidrs` が空の場合、プロセスごとに最初に公開アドレスから到着したリクエストのクライアント IP を含みます。ゲートウェイはリクエストを通常通り提供します。イベントは、ゲートウェイが公開インターネットから到達可能である可能性があることを示します。公開と見なされるもの、および推奨される許可リストについては、[`access_control` リファレンス](/docs/ja/claude-apps-gateway-config#http-tuning) を参照してください。
  * `inference` は、どの上流がリクエストを提供したか、応答ステータスを記録します
  * `desktop_bootstrap.denied` は、拒否された Claude Desktop ブートストラップフェッチを理由（`not_configured`、`policy_not_opted_in`、`no_policy_matched`）とユーザーのアイデンティティで記録します
  * `admin.denied` は、拒否された admin-API 認証試行をクライアント IP、メソッド、パス、理由で記録します。提示されたキーマテリアルなし：`x-api-key` が提示されたが設定されたキーと一致しなかった場合は `invalid_key`、`Authorization` ヘッダーのみが提示され、ゲートウェイセッションとして `admin.admin_groups` で検証されなかった場合は `bearer_rejected`、どちらのヘッダーも提示されなかった場合は `no_credentials`
* **運用ログ**：ブート、警告、上流エラーの人間が読める `[gateway]` プレフィックス付きの行。`CLAUDE_GATEWAY_LOG_LEVEL` 環境変数は詳細度を制御し、`debug`、`info`、`warn`、`error` を受け入れます。デフォルトは `info` です。`debug` では、各サインインと更新は id\_token のクレーム名（値ではなく）もログに記録され、`userinfo_fallback` が提供した場合は userinfo クレーム名もログに記録されるため、PII をログに記録することなく `email_claim` と `groups_claim` 設定を診断できます。監査イベントには影響しません。常に発行されます。

<h3 id="health">
  ヘルス
</h3>

ゲートウェイは `GET /healthz` を liveness プローブとして、`GET /readyz` を readiness プローブとして提供します。`/readyz` はストアが到達可能であることを検証します。両方とも `access_control.allow_cidrs` から除外されるため、プローブはロックダウンされたリスナーで機能し続けます。

`/.well-known/oauth-authorization-server` の OAuth ディスカバリードキュメントは、設定ロード、OIDC ディスカバリー、上流クライアント構築、Postgres マイグレーションがすべて成功した後にのみ `200` を返すため、エンドツーエンドのブートチェックとしても機能します。

<h3 id="concurrent-upstream-requests">
  同時上流リクエスト
</h3>

デフォルトでは、各ゲートウェイレプリカは最大 256 個のリクエストを同時に上流に送信します。ストリーミング応答はストリームが終了するまで制限に対してカウントされます。

レプリカが制限に達している間にリクエストが到着すると、ゲートウェイ内で空きスロットを待ちます。開発者は開始が遅い、またはハングしているように見える応答を見ます。`provider: anthropic` 上流では、[`timeouts.upstream_ttfb_ms`](/docs/ja/claude-apps-gateway-config#http-tuning) より長く待つリクエストはその上流をあきらめ、後の上流がそれを提供しない場合は 502 で失敗します。

`upstream requests:` を含むスタートアップログ行は、有効な制限を示します。レプリカが制限より多くのリクエストを開いている間、最大 1 分に 1 回、`client requests are open` を含む警告もログに記録されます。

一度に複数のリクエストを提供するには、2 つのオプションがあります：

* レプリカを追加します。
* 各レプリカの制限を上げます。ゲートウェイコンテナで `BUN_CONFIG_MAX_HTTP_REQUESTS` 環境変数を 1 から 65535 の整数に設定し、コンテナを再起動します。

レプリカは、制限を約リクエストが開いている平均秒数で割った値のリクエストレートで制限を満たします。たとえば、リクエストが平均 10 秒間開いている場合、デフォルト制限 256 のレプリカは約 26 リクエスト/秒で制限を満たします。

CPU でオートスケールする場合、制限でのレプリカはスケールアウトをトリガーせずにリクエストをキューに入れるため、レプリカが `client requests are open` 警告をログに記録するときに表示される CPU レベルより下のターゲットを設定します。

<Warning>
  開いているすべてのリクエストは、ストリーミング中およびスロットを待つ間、ゲートウェイプロセスでメモリを保持します。制限を 256 に保つ場合、オーバーロードされたレプリカのメモリは引き続き増加します。待機中のリクエストはリクエストボディを保持するため。ピーク時に開いているリクエスト数のコンテナメモリをサイズし、制限を変更するときにメモリを監視します。メモリが不足したレプリカは強制終了され、保持するすべてのストリームがドロップされます。
</Warning>

<h3 id="outage-behavior">
  障害時の動作
</h3>

Postgres がダウンした場合、ゲートウェイ自体はサインイン済みの開発者にサービスを提供し続け、新しいサインインは失敗します。開発者が実際に機能し続けるかどうかは、オーケストレーターが readiness をどのように処理するかに依存します：

* **既存セッション**：ベアラートークンは JWT シークレットでローカルに検証され、セッション更新はストアに触れず、ゲートウェイプロセスは引き続き推論を提供できます
* **新しいサインイン**：Postgres が回復するまで失敗します。デバイスフローとそのレート制限カウンターは Postgres に存在するため
* **[支出制限の実装](/docs/ja/claude-apps-gateway-spend-limits#postgres-availability)**：デフォルトでは障害中に失敗してオープンになるため、推論は引き続き流れます。ブロックするのを好む場合は、失敗を閉じるようにフリップします
* **Readiness**：`/readyz` は障害中に not-ready を報告するため、readiness でトラフィックをゲートするオーケストレーターはすべてのレプリカを一度にローテーションから削除します。そのトポロジーでは、ゲートウェイが引き続き提供できる推論を含むすべてのトラフィックは、Postgres が回復するまでロードバランサーで失敗します。`/healthz` の liveness プローブは引き続き合格するため、レプリカは再起動されません。ストア障害を通じてサインイン済みの開発者が機能し続けるようにしたい場合は、readiness プローブを `/healthz` に指定します。コストは新しいサインインが引き続き ready を報告するレプリカに対して失敗することです。

IdP がダウンした場合、既存セッションは `ttl_hours` まで機能し、新しいログインは失敗し、セッション更新は再試行の回答を取得して IdP が戻ったら機能します。IdP が頻繁なメンテナンスウィンドウを持つ場合は、より長い `ttl_hours` を設定します。

<h3 id="jwt-secret-rotation">
  JWT シークレットローテーション
</h3>

既存セッションが有効なままになるように、段階的にシークレットをローテーションします：

1. 新しいシークレットを生成します。`session.jwt_secret` 配列の前に付加します。
2. デプロイメントをロールします。新しいトークンは新しいシークレットで署名します。古いトークンは引き続き検証されます。
3. `ttl_hours` とマージンの後、古いシークレットを削除してもう一度ロールします。

ローテーションは、有効期限が切れる前にセッションを強制的に削除する唯一の方法でもあります。ベアラートークンは JWT シークレットに対してローカルに検証されるため、セッションごとの取り消しはありません。シークレットを完全に置き換え、配列に古いものを保持しないと、すべての未処理セッションが一度に無効になります。個別のオフボーディングの場合は、IdP でユーザーをプロビジョニング解除します。セッションは `ttl_hours` 内に終了します。

<h3 id="postgres">
  Postgres
</h3>

ゲートウェイは 5 つのデータテーブルと `_migrations` テーブルを保持し、すべてはブート時マイグレーションで作成されます：

| テーブル               | 内容                                       | 保持期間                                                  |
| ------------------ | ---------------------------------------- | ----------------------------------------------------- |
| `kv`               | デバイスグラント（10 分 TTL）とレート制限カウンター            | 行ごとの TTL                                              |
| `spend`            | プリンシパルごとの期間から現在までの支出カウンター（セント単位）         | `admin.spend_retention_months`、デフォルト 13               |
| `spend_limits`     | 設定された支出キャップ                              | API 経由で削除されるまで                                        |
| `admin_audit`      | Admin API ミューテーショントレイル                   | `admin.audit_retention_days`、デフォルト 365                |
| `principal_emails` | 各プリンシパルの最後に見たメール、表示名、IdP グループ。PII を含みます。 | `admin.identity_retention_days` 最後のアクティビティ以来、デフォルト 90 |

30 秒ループは TTL を超えた `kv` 行を期限切れにし、1 時間のスイープは支出テーブルの保持ウィンドウを実装するため、何も無制限に成長しません。[支出制限](/docs/ja/claude-apps-gateway-spend-limits) が設定されていない場合、`kv` のみが書き込まれます。ゲートウェイはブート時と各アップグレード時に独自のスキーママイグレーションを適用するため、そのデータベースロールはテーブルを作成および変更する権限が必要です。ゲートウェイ専用のデータベースまたはスキーマを指定して、その権限を狭く保ちます。

支出制限が使用されている場合、失われたデータベースは失われた支出追跡とキャップを意味し、開発者の再ログインだけではないため、定期的なバックアップを実行します。保持を待つのではなく、出発した開発者を直ちに削除するには、`DELETE FROM principal_emails WHERE principal = '<sub>'` を直接実行します。これはメール、名前、グループを保持する唯一のテーブルを削除します。`spend` と `admin_audit` 行は疑似匿名 OIDC `sub` のみを参照します。

<h3 id="upgrades">
  アップグレード
</h3>

レプリカはステートレスであるため、ローリング再起動はゲートウェイの状態を失いません。ゲートウェイはブート時にスキーママイグレーションを実行します。つまり、新しいバイナリをデプロイするとデータベースが自動的にマイグレーションされます。同時実行レプリカは Postgres アドバイザリロックでシリアライズされるため、各マイグレーションを適用するのは 1 つだけです。

オーケストレーターがローリング再起動またはスケールインのように `SIGTERM` でレプリカを停止する場合、ゲートウェイは新しい接続の受け入れを停止し、既に進行中のリクエストとストリームが終了してから終了するのを待ちます。ドレインウィンドウと呼ばれる最大 25 秒間待機し、その後、まだ開いているものを閉じます。`SIGINT`（ターミナルの Ctrl+C など）は同じドレインを開始し、ドレイン中の 2 番目のシグナルは開いているリクエストを閉じて直ちに終了します。ドレインには gateway v2.1.274 以降が必要です。

長い生成はストリーミングを数分間続けることができます。Kubernetes と Amazon ECS では、これらの両方を一緒に上げて、それらのストリームにより多くの時間を与えます：

* **ドレインウィンドウ**：ゲートウェイコンテナで `CLAUDE_GATEWAY_DRAIN_TIMEOUT_MS` 環境変数を `120000` などのミリ秒の正の整数に設定します。ゲートウェイは `120s` などの他の形式の値を無視し、25 秒のデフォルトを保持します
* **オーケストレーターのグレースピリオド**：Kubernetes の `terminationGracePeriodSeconds`、または Amazon ECS の `stopTimeout`

グレースピリオドは両方のプラットフォームでデフォルト 30 秒です。ドレインウィンドウより少なくとも 5 秒長く保つか、オーケストレーターはドレインが終了する前にゲートウェイを強制終了します。Kubernetes では、`preStop` フックの期間も追加します。グレースピリオドはフックが実行されるのではなく、ゲートウェイが `SIGTERM` を受け取る前にカウント開始するため。

プラットフォームはドレインが実行できる期間をキャップすることもあります：

* **Amazon ECS on Fargate**：`stopTimeout` は最大 120 秒を許可します
* **Cloud Run**：`SIGTERM` の 10 秒後にインスタンスを停止するため、開いているストリームはドレインウィンドウが何であれ最大 10 秒を取得します

ドレインウィンドウが開いているリクエストで終了する場合、ゲートウェイは `drain window over after` を含む警告をログに記録し、カットしたリクエストをカウントし、上げるべき両方の設定に名前を付けます。

マイグレーションは追加のみであるため、より少ないマイグレーションを知っている以前のバイナリにロールバックするのは安全です。余分な行を無視します。ロールバックは YAML を古いバイナリのスキーマに対して再検証するため、新しいリリースで導入されたキーを採用した設定は古いバイナリでのブートに失敗します。ロールバックする前に新しいキーを削除します。

独自のイメージでゲートウェイのバージョンをピン留めするため、新しい Claude Code リリースのセキュリティ修正を含む修正は、ピンを更新して再デプロイするときにのみデプロイメントに到達します。ゲートウェイを、本番認証情報を保持する他のサービスに使用するのと同じパッチングケーデンスに含めます。

<h2 id="security">
  セキュリティ
</h2>

このセクションは、セキュリティレビューが尋ねる質問に答えます：ゲートウェイを通じてどのデータが流れ、どこに行くか、設計が防御する攻撃、コンプライアンスアンケートに属する回答。

<h3 id="data-flow">
  データフロー
</h3>

| データ                                                                             | パス                                                                                                                                                                                                                 | ゲートウェイによって Anthropic に送信    |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------- |
| 推論（プロンプト、完了）                                                                    | CLI → ゲートウェイ → 上流                                                                                                                                                                                                  | Anthropic API が設定された上流の場合のみ |
| テレメトリ（OTLP メトリクス、プラス [オプトイン ログとトレース](/docs/ja/claude-apps-gateway-config#telemetry)） | CLI → ゲートウェイ → コレクター                                                                                                                                                                                               | なし                          |
| アイデンティティ（メール、グループ、sub）                                                          | IdP → ゲートウェイ → JWT → CLI。CLI はそれを OTLP エクスポートにスタンプします。[`forward_user_identity`](/docs/ja/claude-apps-gateway-config#per-user-identity-headers-for-a-proxy-you-run) をオンにすると、ゲートウェイは開発者のメールと IdP サブジェクトをヘッダーとしてプロキシに送信します | なし                          |
| 管理設定                                                                            | ゲートウェイ YAML → CLI                                                                                                                                                                                                  | なし                          |
| 監査ログ                                                                            | ゲートウェイ stderr → アグリゲーター                                                                                                                                                                                            | なし                          |

<h3 id="threat-model-summary">
  脅威モデルの概要
</h3>

ゲートウェイはネットワーク境界内に位置しますが、個々の開発者ラップトップは信頼されていないと見なされます。設計はこれを 3 つの方法で説明します：

* 開発者は生の上流キーの代わりに短命の JWT を保持します。CLI からゲートウェイへのレッグは RFC 8628 デバイスグラントを使用し、ゲートウェイの IdP との認可コード交換はデフォルト設定で PKCE を実行するため、インターセプトされた IdP 認可コードは無用です。
* デバイス検証ページは同一オリジン POST と RFC 8628 §5.1 ごとの IP ごとのレート制限を実装します。[ユーザーコードブルートフォース耐性](#user-code-brute-force-resistance) を参照してください。
* ゲートウェイの IdP、OTLP コレクター、および `provider: anthropic` 上流へのリクエストは、サーバー側リクエストフォージェリ（SSRF）ガードを通じて行われます。DNS を解決し、リンクローカルとクラウドメタデータアドレスをブロックし、デフォルトではループバックをブロックし、接続を解決された IP にピン留めするため、オペレーター影響 URL はクラウドメタデータエンドポイントにリダイレクトできません。RFC 1918 プライベート範囲は意図的に許可されます。IdP と OTLP コレクターは一般的にプライベート IP に存在するため。その他のプロバイダーの場合、ゲートウェイは設定をロードするときにそれらのアドレスまたはメタデータホスト名を指定する `base_url` を拒否し、プロバイダーの SDK は DNS チェックなしで接続します。

  [プロキシのみのエグレス](/docs/ja/claude-apps-gateway-config#proxy-only-egress) をオンにすると、そのアドレスチェックはフォワードプロキシに移動します：ゲートウェイはホスト名を渡し、プロキシのアロウリストはそれらの宛先を拒否する必要があります。

  ゲートウェイが正当に到達する必要があるもの（ローカル開発 IdP やサイドカー OTLP コレクター（`localhost` など））がループバック上に存在する場合にのみ、ゲートウェイの環境で `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` を設定します。変数はすべてのオペレーター設定 URL のループバックブロックを緩和し、ポッドがクラウドメタデータエンドポイントに到達できるかどうかをチェックするブート時警告もスキップするため、コレクターに独自の内部アドレスを与えることをお勧めします。

独自のエグレス制御を追加する場合、ゲートウェイはワークロードアイデンティティなどのインスタンスメタデータ認証情報を使用するときはいつでもメタデータサーバーに到達する必要があります。

2 つの脅威は、インフラストラクチャを保護するためのものであるため、スコープ外です：

* **侵害されたゲートウェイホスト**：ホストは上流認証情報を保持し、[管理設定](/docs/ja/claude-apps-gateway-config#managed) をすべての接続された開発者に配布するため、ゲートウェイの設定の制御は MDM の制御に匹敵します。CLI の [承認ダイアログ](/docs/ja/server-managed-settings#approval-memory) はシェル対応設定のサイレント変更を制限しますが、ホストセキュリティに置き換わりません。
* **悪意のある OIDC プロバイダー**：プロバイダーはゲートウェイが信頼する id\_token に署名するため、任意のアイデンティティを主張できます。IdP の検証と保護はあなたの責任です。

<h3 id="user-code-brute-force-resistance">
  ユーザーコードブルートフォース耐性
</h3>

開発者が `/device` 検証ページに入力する `user_code` は、20 文字のアルファベットから引き出された 8 文字です。これは 20⁸ または約 2.56×10¹⁰ の組み合わせを生成し、10 分後に期限切れになります。

ゲートウェイは [`rate_limits`](/docs/ja/claude-apps-gateway-config#http-tuning) を通じて設定可能なデバイスグラントエンドポイントに IP ごとのレート制限を適用します。多くの開発者が単一の共有企業 NAT アドレスからサインインする場合は、制限を上げます。[大規模ロールアウト](#large-rollouts) は、それらのサイズを決定する方法を示しています。制限はサインインフローにのみ適用され、推論には適用されません。

<h3 id="compliance-posture">
  コンプライアンス体制
</h3>

* **データレジデンシー**：ゲートウェイ自体のデータプレーンは、Anthropic API が設定された上流の場合を除き、Anthropic に何も送信しません。その場合、既存のデータ処理契約が推論パスに適用されます。テレメトリ、監査、アイデンティティ、設定は設定した宛先にのみ行きます。
* **ホストプロセストラフィック**：ホストプロセスは Claude Code CLI です。`claude gateway` は Amazon Bedrock および Google Cloud の Agent Platform デプロイメントと同じサードパーティルールの下で実行され、Anthropic に何も送信しません。v2.1.227 より前では、ホストプロセスは製品バージョンとプラットフォームなどのスタートアップテレメトリを送信していました。これはコンテナ環境で `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` を設定することでオフにできました。これらのリリースはまた、ボート時に本文または認証情報なしで 1 つの `HEAD` リクエストを `https://api.anthropic.com` の `/api/hello` に送信していました。または環境が `ANTHROPIC_BASE_URL` を設定した場合はそれに、環境が `HTTPS_PROXY` などのプロキシ変数または mTLS クライアント証明書も設定していない限り。彼らは応答を無視したため、エグレスファイアウォールでそのリクエストをブロックしてもゲートウェイに影響しませんでした。
* **クライアント分析**：CLI はゲートウェイにサインインしている間、独自の使用分析とエラー報告を無効にします。最初のサインイン前に、CLI はまだ Anthropic にスタートアップイベントを送信します。これには、管理設定がゲートウェイサインインを強制するマシンも含まれます。これらもオフにするには、ゲートウェイサインインを強制する同じ [クライアント側管理設定](/docs/ja/claude-apps-gateway-config#client-side-managed-settings) で [`DISABLE_TELEMETRY`](/docs/ja/managed-settings#turn-telemetry-off-for-your-organization) を配信します。
* **エラー報告**：CLI はモデルリクエストが Anthropic のファーストパーティ API 以外のエンドポイント（Amazon Bedrock やカスタム `ANTHROPIC_BASE_URL` など）に行くときはいつでもエラー報告をオフにします。
* **クライアントマシン**：開発者の CLI は、`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` と `skipWebFetchPreflight: true` が設定されていない限り、WebFetch ホスト名チェックとバージョンチェックを Anthropic に送信し続けます。[データ使用](/docs/ja/data-usage) を参照してください。
* **サーベイ評価**：ゲートウェイにサインインしている間、CLI は Anthropic バウンド評価アップロードを分析ストリームと一緒に無効にするため、評価を Anthropic に送信しません。
* **トランスクリプト共有**：サーベイのトランスクリプト共有プロンプトで「はい」を選択すると、Anthropic にアップロードする代わりに `~/.claude/feedback-bundles/` の下にローカルファイルを書き込みます。
* **クライアント更新**：更新チェックはゲートウェイトラフィックとは別です。独自の配布を通じてバージョンをピン留めし、ラップトップがリリースをフェッチしてはいけない場合は `DISABLE_UPDATES` を設定します。`DISABLE_AUTOUPDATER` はバックグラウンド更新のみを停止し、`claude update` は引き続き機能します。
* **TLS**：本番環境で `public_url` を HTTPS 経由で提供します。ゲートウェイ自体のリスナー経由で `listen.tls` を使用するか、プレーン HTTP レプリカの前の TLS 終了 ingress から `listen.public_url` を設定します。どちらの場合でも。ゲートウェイはプレーン HTTP を拒否しません。IdP は本番環境で HTTPS を提供する必要があり、Postgres は `?sslmode=require` をサポートします。ingress で `Strict-Transport-Security` を設定します。
* **脆弱性開示**：[セキュリティ問題の報告](/docs/ja/security#reporting-security-issues) に従います

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

ご質問やフィードバックについては、[Claude Code サポート](https://support.claude.com/en/collections/14445694-claude-code)をご利用いただくか、[Claude Code GitHub リポジトリ](https://github.com/anthropics/claude-code/issues)で issue を開いてください。問題を報告する際は、以下の情報を含めてください。

* **Gateway の問題**: gateway の stderr（該当するウィンドウの）、`gateway.yaml`（シークレットは削除）、gateway のバージョン（ランディングページの `/` と `/managed/settings` の `x-cc-gateway-version` レスポンスヘッダーに表示）、および最近の変更内容
* **ログイン問題**: 開発者が `claude --debug-file ./claude-debug.txt` を実行して再現し、そのファイルと同じウィンドウの gateway の監査ログを送信
* **推論の問題**: リクエストされたモデル、設定されたアップストリーム、およびリクエストの gateway 監査ログ（どのアップストリームがそれを処理したか、およびレスポンスステータスを記録）

gateway の stderr には監査イベントストリームが含まれ、監査ログには開発者の ID が記録され、デバッグファイルには開発者のマシンからの hook と MCP サーバーの出力が記録されます。公開 issue に投稿する前に、これらを確認して削除してください。

| 症状                                                                                                                                                                                  | 原因                                                                                                                                                                                                                                                                                  | 修正方法                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 開発者の `/login` が **Cloud gateway** 画面ではなく標準のアカウントピッカーを表示する                                                                                                                           | そのマシンのマネージド設定で `forceLoginMethod` または `forceLoginGatewayUrl` が設定されていない                                                                                                                                                                                                              | [マネージド設定ファイル](/docs/ja/claude-apps-gateway#set-the-gateway-url)をデバイスにデプロイしてください。`/login` はそこから gateway URL を読み込みます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| 開発者のリクエストが `Not signed in to the Cloud gateway — run /login.` で失敗する                                                                                                                 | マシンのマネージド設定で `forceLoginMethod: "gateway"` または `forceLoginGatewayUrl` が設定されており、セッションに gateway サインインがない。残っている claude.ai ログインは要件を満たしていません。                                                                                                                                            | 開発者に `/login` を実行して gateway サインインを完了させてください。[Administrator policy requires a Cloud gateway sign-in](/docs/ja/errors#administrator-policy-requires-a-cloud-gateway-sign-in) も参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Claude Desktop がブートストラップ設定を取得できないと報告する                                                                                                                                              | `/user/bootstrap` が 404 を返した: ユーザーに一致するポリシーが `desktop` キーを持たないか、ポリシーが一致しなかった。gateway の監査ログは各拒否を `desktop_bootstrap.denied` として理由とともに記録します。                                                                                                                                          | ユーザーに一致するポリシー、または `match: {}` ベースレイヤーに `desktop` ブロックを追加してください。空の `desktop: {}` で十分です。[Claude Desktop overlay](/docs/ja/claude-apps-gateway-config#claude-desktop-overlay) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| スタートアップが `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.` を表示する                                                | インストールされている Claude Code ビルドが gateway サポート前のバージョン                                                                                                                                                                                                                                    | 開発者に Claude Code を Cloud gateway サポートを含むリリースに更新させてください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| スタートアップが `Administrator policy requires a Cloud gateway sign-in on this machine` で終了する                                                                                              | 開発者の環境が `ANTHROPIC_API_KEY` または `ANTHROPIC_AUTH_TOKEN` を設定しているか、設定が [`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper) を設定しているか、または以前の Claude Console ログインからの API キーがまだ保存されている                                                                                                   | 適用される各項目をクリアするよう開発者に指示してください: 変数を設定解除するか、`apiKeyHelper` エントリを削除するか、`claude auth logout` を実行して保存されたキーを削除してください。その後、`claude` を起動して `/login` でサインインさせてください。[Administrator policy requires a Cloud gateway sign-in](/docs/ja/errors#administrator-policy-requires-a-cloud-gateway-sign-in) も参照してください。                                                                                                                                                                                                                                                                                                                       |
| スタートアップまたは `/login` がマネージド設定ロード時の 403 の後に `Claude Code may not be enabled for your organization` を報告する                                                                              | gateway、またはその前にあるもの、が `/managed/settings` リクエストに 403 で応答した。gateway 自体の設定ルートは 403 で応答することはありません。ステータスは [`access_control`](/docs/ja/claude-apps-gateway-config#http-tuning) IP チェック、または gateway の前にあるプロキシまたは WAF から来ています。監査ログは IP チェック拒否を `access.denied` として理由とともに記録します。開発者はサインイン状態を保ちます。 | 失敗時の監査ログで `access.denied` を確認し、`access_control` リストまたはフロントエンドを修正してから、開発者に `claude` を再度起動させてください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| CLI `/login`: `The gateway is limiting sign-in attempts right now`、または古いバージョンで `Request failed with status code 429`。`/device` ページは以前に試したことのない開発者に `Too many attempts` を表示する場合があります | IP ごとのサインインレート制限に達した。`listen.trusted_proxies` がロードバランサーをカバーしていないため、すべての開発者がそのアドレスを共有するか、多くの開発者が NAT または VPN 出口アドレスを共有しています。`result: rate_limited` の監査イベントは同じ 1 つまたは少数の `client_ip` 値を表示します。                                                                                         | まず `listen.trusted_proxies` をロードバランサーのソース範囲に設定し、開発者がアドレスを共有し続ける場合は `rate_limits` を上げてください。[大規模なロールアウト](#large-rollouts)を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`                                          | gateway ホスト名が少なくとも 1 つのパブリック IP アドレスに解決される。Claude Code は各解決されたアドレスをチェックし、すべてがプライベートであることを要求します。一般的な原因は、1 つのファミリーがパブリックアドレスに解決されるデュアルスタック名です。AWS 内部デュアルスタックロードバランサーを含み、パブリック範囲の AAAA アドレスを返します。                                                                                      | gateway 名が開発者マシン上でのみプライベートアドレスに解決されるようにしてください。デュアルスタック名の場合、パブリック範囲のレコードを削除するか、別の内部専用 DNS 名を提供してください。[プライベートネットワークの前提条件](/docs/ja/claude-apps-gateway#prerequisites)を参照してください。アドレスがお客様の組織が所有して内部的に使用するパブリックスペースである場合、[そのブロックを宣言](/docs/ja/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)してください。                                                                                                                                                                                                                                                                                                         |
| CLI `/login`: `Gateway login would go through proxy <proxy>, which is not on a private network`                                                                                     | `HTTPS_PROXY` または `HTTP_PROXY` が gateway ホストに適用され、プロキシのホスト名がパブリックアドレスに解決される。ホストがプライベートアドレスのみに解決されるプロキシは許可され、このエラーをトリガーしません                                                                                                                                                          | 開発者のマシンの `NO_PROXY` に gateway ホストを追加して接続を直接にするか、ホスト名がプライベートアドレスに解決されるプロキシを使用してください。メッセージは追加する正確な `NO_PROXY` エントリを名前付けします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| CLI `/login`: `Claude Code only signs in to <host> from inside its declared network <block> (managed settings), and this machine is connecting from <ip>, outside it`               | gateway は [`gatewayInternalNetworks`](/docs/ja/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) で宣言されたブロック上にあり、開発者のマシンがそのブロック外のアドレスからそれに到達した: VPN アドレスプール、コンテナまたは WSL2 NAT セグメント、またはお客様の組織のものではないネットワーク                                                              | 開発者にお客様のネットワーク上のホスト OS から `/login` を実行させてください。表示されたアドレスがお客様の組織のパブリックスペースでもある場合、gateway のエントリを両方をカバーするブロックに置き換えてください。最大 `/8`。2 番目の重複するエントリは拒否されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| CLI `/login`: `Every address for gateway host <host> must be inside its declared network <block>, and it also resolves to <ip>`                                                     | gateway の名前が [`gatewayInternalNetworks`](/docs/ja/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) で宣言されたブロック外のアドレスに解決される: 2 番目のサイト、またはデュアルスタック名上の IPv6 レコード。宣言されたブロックの下では、すべてのレコードがその 1 つの IPv4 ブロック内にある必要があります。プライベートおよび IPv6 アドレスを含む                              | 開発者マシン上の gateway 名に対してブロック内のレコードのみを公開するか、別の内部専用名を提供してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| CLI `/login`: `<host> is on the declared network <block>, which Claude Code checks over a direct connection, not through an HTTP proxy`                                             | `HTTPS_PROXY` または `HTTP_PROXY` が宣言されたブロック上の gateway に適用される                                                                                                                                                                                                                          | 開発者のマシンで、メッセージが名前付けする `NO_PROXY` エントリを追加してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| CLI `/login`: `gatewayInternalNetworks in managed settings` で始まるメッセージ                                                                                                               | 値が [検証ルール](/docs/ja/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)の 1 つを破り、メッセージがどれかを名前付けします。修正するまで、Claude Code はマシン上のすべての新しい gateway `/login` を拒否します。プライベートアドレス上の gateway を含む。既存のサインインは機能し続けます                                                                    | デプロイするマネージド設定ソースで、メッセージが名前付けするエントリを修正してから、`/login` を再実行してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| CLI `/login`: `Could not resolve the configured HTTP proxy`                                                                                                                         | `HTTPS_PROXY` または `HTTP_PROXY` のホスト名が開発者のマシンから解決されない。通常、企業ネットワークに接続されていないため                                                                                                                                                                                                         | 開発者にネットワークまたは VPN に接続させて再試行するか、プロキシ URL を修正してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                                                               | マシンが gateway の内部 DNS 名を解決できない。通常、企業ネットワーク上にないため                                                                                                                                                                                                                                     | 開発者にネットワークまたは VPN に接続させてから、`/login` を再試行してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ブート時に `store.postgres_url` という名前の設定検証エラーで終了する                                                                                                                                       | Postgres が設定されていない。gateway は Postgres を必要とします                                                                                                                                                                                                                                       | `store.postgres_url` を設定してください。ローカル開発の場合、使い捨てコンテナを使用してください: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`。                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ブート時に終了: `requires the native binary`                                                                                                                                               | Node の代わりにネイティブバイナリで実行されていない                                                                                                                                                                                                                                                        | Claude Code を [スタンドアロンインストール方法](/docs/ja/setup)のいずれかでインストールしてください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ブート時に `config.load` の後に OIDC ディスカバリーエラーで終了する                                                                                                                                        | `oidc.issuer` に到達できない、または TLS チェーンが信頼されていない                                                                                                                                                                                                                                         | 発行者がポッドから到達可能で、`/.well-known/openid-configuration` を提供していることを確認してください。プライベート PKI の場合は `ca_cert_pem` を設定してください。ポッドが IdP にフォワードプロキシ経由でのみ到達する場合、[`oidc.use_proxy: true`](/docs/ja/claude-apps-gateway-config#idp-requests-through-a-forward-proxy)を設定してください。v2.1.227 より前のバージョンでは、代わりに IdP の各エンドポイントへの直接ルートをポッドに提供してください。ポッドが IdP のホスト名を解決できない場合、またはプロキシが IP アドレスへの `CONNECT` を拒否する場合、[プロキシのみの出口](/docs/ja/claude-apps-gateway-config#proxy-only-egress)を参照してください。これには v2.1.277 以降が必要です。                                                                                                                                         |
| ブート時に Postgres パーミッションエラーで終了する                                                                                                                                                      | データベースロールがそのスキーマに対する DDL 権限を持たない                                                                                                                                                                                                                                                    | ロールに gateway のスキーマに対する `CREATE` を付与して、ブート時にテーブルを作成・変更できるようにしてください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ログ: `could not connect to Postgres at boot, attempt 1 of 3`                                                                                                                         | gateway が起動したときにデータベースがまだ到達可能ではなかった。例えば、ネットワークがまだ起動中のコールドインスタンス                                                                                                                                                                                                                     | gateway がブートを完了する場合、アクションは不要です。データベースに到達できない場合、gateway は接続を 3 回試行し、2 秒間隔で、終了する前に試行します。`could not connect to Postgres` で終了する場合、`store.postgres_url` とデータベースへのネットワークパスを確認してください。試行がタイムアウトするのではなく拒否される場合、[`store.connect_timeout_seconds`](/docs/ja/claude-apps-gateway-config#store)を上げて各試行に長い時間を与えてください。                                                                                                                                                                                                                                                                                                                 |
| `/oauth/callback` が「Sign-in could not be completed」を表示する                                                                                                                            | メールドメインが拒否された、id\_token 検証が失敗した、または `email_verified` が明示的に `false` である。gateway は常にオーバーライドなしでこれを拒否します                                                                                                                                                                                | `allowed_email_domains` を確認し、IdP が検証済みの `email` クレームを返していることを確認してください。`email_verified: false` の場合、IdP 側の検証を修正してください。IdP がメールを別のクレーム名で発行する場合、`oidc.email_claim` を設定してください。                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ログ: `token exchange failed request_id=<id>: id_token missing email claim`                                                                                                           | IdP がデフォルトで id\_token に `email` を含めていない。この拒否は `allowed_email_domains` が設定されている場合にのみ発火します。設定されていない場合、メールがないとメールなしのセッションが作成されます                                                                                                                                                       | IdP を設定して id\_token に `email` を発行させてください。Okta: カスタム認可サーバーの ID トークンクレームに `email` を追加してください。Entra: アプリ登録でオプションクレームとして `email` を追加してください。PingFederate: `email` を発行する OpenID Connect ポリシーを有効にしてください。IdP が userinfo エンドポイントから `email` を提供するが id\_token に含めない場合（Okta org 認可サーバーなど）、`oidc.userinfo_fallback: true` を設定してください。                                                                                                                                                                                                                                                                                             |
| ログ: `refresh failed request_id=<id>: invalid_token (…) (at userinfo_no_id_token, …)`、および開発者が `Cloud gateway session expired` を `session.ttl_hours` ごとに見る                            | IdP がリフレッシュトークンを受け入れたが、それで id\_token を返さなかったため、gateway は IdP の userinfo エンドポイントにユーザーのクレームを求めました。IdP はそこでリフレッシュされたアクセストークンを拒否しました。gateway は `temporarily_unavailable` で応答するため、Claude Code はリフレッシュトークンを保持しますがセッションを更新できません。v2.1.260 より前の gateway バージョンは `(at …)` の詳細なしで同じ行をログします。     | [`oidc.scope_on_refresh: true`](/docs/ja/claude-apps-gateway-config#oidc)を設定してください。gateway v2.1.260 以降で利用可能です。リフレッシュリクエストが再び `openid` を要求するようにします。Okta などの一部の IdP は、要求された場合にのみリフレッシュ時に id\_token を返します。PingFederate では、代わりに **Applications > OAuth > OpenID Connect Policy Management** の下で **Return ID Token On Refresh Grant** を有効にしてください。キーは PingFederate の動作を変更しません。それでも省略する他の IdP の場合、userinfo エンドポイントがリフレッシュによって発行されたアクセストークンを受け入れるかどうかを確認してください。一時的な対応として、[`session.ttl_hours`](/docs/ja/claude-apps-gateway-config#session)を上げてください。[Identity provider setup](#identity-provider-setup) でプロビジョニング解除のトレードオフを参照してください。 |
| すべての Amazon Bedrock リクエストが 502 を返す。ログに `Could not load credentials from any providers` が表示される                                                                                       | EC2 では、IMDSv2 のデフォルトホップリミット 1 がコンテナ内からのインスタンスメタデータリクエストをブロックします。ブートと `/readyz` は AWS SDK がクライアント構築時ではなく最初のリクエストでインスタンス認証情報を解決するため、とにかく成功します                                                                                                                                         | `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2` でホップリミットを上げるか、起動テンプレートで設定してください。変更はインスタンス上のすべてのコンテナに適用されます。利用可能な場合は ECS タスクロールを優先してください。これは ECS コンテナ認証情報エンドポイントから認証情報を読み込み、変更を完全に回避します。または、変更を専用 gateway インスタンスに適用して露出を制限してください。                                                                                                                                                                                                                                                                                                                                     |
| ピークロード時に、レスポンスの開始が遅い、またはハングしているように見える、または upstream が健全な場合に 502 `all upstreams failed` で失敗する                                                                                         | レプリカは upstream に一度に送信するよりも多くのリクエストを開いているため、余分なリクエストは gateway 内で待機します。`provider: anthropic` upstream では、`timeouts.upstream_ttfb_ms` より長く待つリクエストはその upstream をあきらめ、後の upstream がそれを処理しない場合は 502 を生成します。ログは `client requests are open` を含む警告を表示します。                                    | レプリカを追加するか、各レプリカの制限を上げてください。[同時 upstream リクエスト](#concurrent-upstream-requests)を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| IdP エラー: unknown or unsupported scope                                                                                                                                               | IdP が認識しないスコープを拒否する                                                                                                                                                                                                                                                                 | `oidc.scopes` を IdP が受け入れるリストに正確に設定してください。`openid` を含める必要があります。デフォルトは `openid profile email offline_access` です。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `oidc.scopes` を設定した後、セッションが自動的に更新されない                                                                                                                                               | `offline_access` がオーバーライドから削除された                                                                                                                                                                                                                                                    | IdP がサポートしている場合は `offline_access` を戻してください。リフレッシュトークンがない場合、開発者は `session.ttl_hours` ごとにブラウザログインを再実行します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ブラウザが「This request came from another site and was blocked」を表示する                                                                                                                     | クロスサイトフォーム POST。CSRF 保護としてブロックされました。埋め込みまたはプロキシされたページでは予想されます                                                                                                                                                                                                                       | 検証リンクを直接開いてください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Chrome が「Refused to send form data … violates … Content Security Policy directive: form-action」で Approve ボタンをブロックするが、同じページが Safari または Firefox で機能する                                | Chrome はリダイレクトチェーン全体に対して `form-action` を適用します。IdP が許可リストに登録されていない 2 番目のホストにさらにリダイレクトします。                                                                                                                                                                                            | リダイレクトチェーン内の各追加オリジンを `oidc.form_action_origins` に追加してください。Approve ページで Chrome DevTools → Console を開いて、どのオリジンがブロックされたかを確認してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| サインインが IdP で完了するがコールバックが失敗する。Chrome で CSP エラーまたは Safari で「this sign-in link has expired」が表示される                                                                                      | IdP が `response_mode=form_post` 経由でコードを返しました。これは POST 経由で `/oauth/callback` にクロスオリジンで自動送信します。Chrome はこれを厳密な CSP の下でブロックします。Safari は送信を許可しますがコールバックはクエリ文字列のみを読み込みます。                                                                                                                 | IdP が `response_mode=query` を尊重していることを確認してください。gateway はコールバックがプレーンリダイレクトになるように明示的にこれをリクエストします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ログインはローカルで機能するが ALB の背後では失敗する                                                                                                                                                       | `public_url` がまだローカルまたは内部の `http://` オリジンを名前付けしているため、IdP は間違った `redirect_uri` を取得します                                                                                                                                                                                                | `listen.public_url` を外部の `https://` オリジンに設定し、`<public_url>/oauth/callback` を IdP に登録してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| 開発者が信頼プロンプトを繰り返し見る                                                                                                                                                                  | TLS 証明書がレプリカごと、またはリクエストごとにローテーションしている                                                                                                                                                                                                                                               | イングレスで安定した証明書を使用するか、TLS を 1 回終了して、レプリカをプレーン HTTP で内部的に実行してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| CLI `/login`: 「Could not verify the gateway's TLS certificate」または `SELF_SIGNED_CERT_IN_CHAIN`                                                                                       | Gateway の TLS チェーンが CLI ホストの信頼ストアにないプライベート CA によって署名されている                                                                                                                                                                                                                           | Claude Code はネイティブバイナリ上でデフォルトで OS 信頼ストアを読み込み、Node 22.15 以降で読み込みます。[`CLAUDE_CODE_CERT_STORE`](/docs/ja/network-config#ca-certificate-store) がこの動作を制御します。CA が OS 信頼ストアにインストールされている場合、開発者が現在のランタイムを使用していることを確認してください。そうでない場合、起動前に `NODE_EXTRA_CA_CERTS` を CA 証明書 PEM に設定してください。最初の接続フィンガープリントプロンプトは引き続き適用されます。                                                                                                                                                                                                                                                                                                             |
| CLI `/login` がブラウザサインインを完了してから、セッションが `Cloud gateway sign-in was not completed` と TLS 証明書の不一致で終了する                                                                                  | サインイン後の最初のリクエストで、gateway は Claude Code がピンした フィンガープリントと一致しない証明書を提示したため、Claude Code は gateway 認証情報を保持しませんでした。通常の原因は、1 つのアドレスの背後にあるレプリカが異なる証明書を提供するか、ネットワークパス上の何かが TLS をインターセプトしています。                                                                                                 | ホスト名に対して 1 つの証明書を提供してください。例えば、イングレスで TLS を 1 回終了してから、開発者に `/login` を再度実行させてください。その証明書がピンされたものと異なる場合、Claude Code は [信頼プロンプト](/docs/ja/claude-apps-gateway#connect-developers) を警告とともに表示します。証明書が変更されました。                                                                                                                                                                                                                                                                                                                                                                                                                  |
| CLI `/login` が `The gateway's TLS certificate changed during sign-in: it no longer matches the one you trusted` で停止する                                                               | サインインリクエストが、開発者が `/login` を開始したときに受け入れた証明書と一致しない証明書を提供するサーバーに到達しました: 1 つのアドレスの背後にあるレプリカが異なる証明書を提供する、パス上の TLS インターセプション、またはサインイン中の証明書ローテーション。                                                                                                                                        | ホスト名に対して 1 つの証明書を提供してから、開発者にサインインを再度開始させ、[信頼プロンプト](/docs/ja/claude-apps-gateway#connect-developers)で新しい証明書を確認させてください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

`Cloud gateway sign-in was not completed` メッセージは gateway ホスト名を名前付けします。Claude Code がピンされたフィンガープリントと提示されたフィンガープリントの両方を持つ場合、メッセージは各の最初の 16 文字も表示します。

Claude Code がゲートウェイサインイン後に `couldn't load your organization's managed settings` を報告する場合、Claude Code は理由を名前付けし、その場で再起動して、会話を再開します。Claude Code が再起動できない場合（例えば、バックグラウンドセッション）、Claude Code はセッションを終了し、サインインを保持します。

<h2 id="related">
  関連
</h2>

* [Claude apps gateway の概要](/docs/ja/claude-apps-gateway)：クイックスタートと開発者接続
* [設定リファレンス](/docs/ja/claude-apps-gateway-config)：すべての `gateway.yaml` オプション
