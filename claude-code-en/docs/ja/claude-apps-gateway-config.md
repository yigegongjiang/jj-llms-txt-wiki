> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude apps gateway 設定

> gateway.yaml のすべてのオプションのリファレンス：リスナーと TLS、OIDC、セッション、Postgres ストア、Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、Microsoft Foundry アップストリーム、モデルルーティング、マネージドポリシー、テレメトリー。

Claude apps gateway デプロイメントは、慣例的に `gateway.yaml` という 1 つの YAML ファイルで設定されます。このファイルは、ゲートウェイが行うすべてのことを定義します：どこでリッスンするか、開発者がどのようにサインインするか、推論がどこに行くか、どのポリシーとテレメトリーが適用されるかです。このページは、そのファイル内のすべてのオプションのリファレンスです。最初のファイルを作成するには、[クイックスタート](/docs/ja/claude-apps-gateway#quickstart)から始めてください。これは最小限の動作設定を構築して実行します。設定に満足したら、[デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy)で、Kubernetes、Cloud Run、または独自のプラットフォームでのコンテナ化とホスティングについて説明しています。

ゲートウェイは、`claude gateway --config /path/to/gateway.yaml` でスタートアップ時にファイルを 1 回読み込みます。すべてのオプションはブート時にスキーマに対して検証されるため、形式が正しくない設定は、最初の使用時ではなく、フィールドレベルのエラーで開始時に失敗します。

このページの最後にある[完全な例](#complete-example)は、すべてのセクションを実行します。

<h2 id="file-structure">
  ファイル構造
</h2>

5 つのセクションが[必須](#required-sections)です。その他のセクションはすべて[オプション](#optional-sections)であり、省略されたセクションはデフォルト値を使用します。不明なキーはブート時に失敗するため、タイプミスは設定が無視されるのではなく、名前付きエラーとして表示されます。

**必須セクション：**

* [`listen`](#listen)：バインドアドレス、パブリック URL、TLS ターミネーション
* [`oidc`](#oidc)：ID プロバイダー（IdP）、発行者、クライアント、クレームマッピング、サインイン可能なユーザーを含む
* [`session`](#session)：ゲートウェイが発行するベアラートークン、シークレット、ライフタイム
* [`store`](#store)：デバイスグラント、レート制限カウンター用の PostgreSQL
* [`upstreams`](#upstreams)：推論の送信先、Anthropic、Amazon Bedrock、AWS 上の Claude Platform、Google Cloud の Agent Platform、Microsoft Foundry のいずれか

**オプションセクション：**

* [`admin`](#admin)：Admin API 認証、支出制限の保持
* [`enforcement`](#enforcement)：支出制限のフェイルオープンまたはフェイルクローズ動作
* [`pricing`](#pricing)：契約レート、支出メーター用の乗数、開発者が表示するコスト数値用の乗数
* [`models`](#models) と `auto_include_builtin_models`：管理者がキュレーションしたモデルリスト、アップストリームごとの ID
* [`managed`](#managed)：IdP グループ別の管理設定ポリシー
* [`telemetry`](#telemetry)：オブザーバビリティスタックへの OTLP フォワーディング
* [`access_control`、`limits`、`timeouts`、`rate_limits`](#http-tuning)：IP 許可/拒否、リクエストサイズ上限、アップストリーム初バイト到達時間、IP ごとのサインイン制限

<h2 id="secret-expansion">
  シークレット展開
</h2>

`client_secret`、`jwt_secret`、`postgres_url` などのシークレットを `gateway.yaml` に直接書き込まないでください。以下のいずれかの形式で参照すると、ゲートウェイはブート時に環境変数またはファイルから値を解決します：

| 形式              | 解決先                                                                                                                                                  | 用途                                           |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| `${VAR}`        | 環境変数 `VAR`。未定義の場合はブート失敗。                                                                                                                             | コンテナ環境変数、env インジェクション経由の AWS Secrets Manager |
| `${file:/path}` | そのパスにあるファイルの内容、トリミング済み。参照はフィールド全体の値である必要があります：`${VAR}` とは異なり、より長い文字列内で展開されないため、データベースパスワードの場合は `postgres_url` に埋め込むのではなく `store.password` を設定してください。 | Kubernetes Secret ボリュームマウント、Vault Agent、SOPS |

<h2 id="required-sections">
  必須セクション
</h2>

<h3 id="listen">
  `listen`
</h3>

`listen` ブロックはゲートウェイがサービスを提供する場所を制御します。バインドアドレスとポート、外部から見えるオリジン、およびオプションの TLS 終了です。

| フィールド                  | 必須                       | 説明                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------------------- | ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | いいえ                      | バインドアドレス。デフォルト `0.0.0.0`。                                                                                                                                                                                                                                                                                                                                                                                   |
| `port`                 | いいえ                      | バインドポート。デフォルト `8080`。                                                                                                                                                                                                                                                                                                                                                                                       |
| `public_url`           | `host` がループバックでない場合を除き必須 | 外部から見える `https://` オリジン。IdP の `redirect_uri` と検出メタデータを構築するために使用されます。`host` がループバックアドレスでない場合は常に必須です。TLS が ALB、Ingress、Cloud Run などのプロキシで終了するか、`tls` を通じてゲートウェイ自体で終了するかに関わらず必須です。ゲートウェイは `X-Forwarded-*` ヘッダーから独自のオリジンを導出することはありません。これらはクライアントがなりすまし可能です。これなしではブート失敗します。以下の `trusted_proxies` はクライアント IP 解決のみを制御します。また、[テレメトリ](#telemetry)を有効にするためにも必須です。ゲートウェイはこの URL からクライアントにプッシュする OTLP エンドポイントを構築するためです。 |
| `tls.cert` / `tls.key` | いいえ                      | ゲートウェイが TLS を自身で終了する場合の PEM パス                                                                                                                                                                                                                                                                                                                                                                              |
| `trusted_proxies`      | いいえ                      | ゲートウェイの前にあるロードバランサーの CIDR または IP。設定されている場合、ゲートウェイはこれらのピアからのみ `X-Forwarded-For` を信頼し、IP ごとのレート制限と監査のために実際のクライアント IP を記録します。nginx の `set_real_ip_from` と同等です。`X-Forwarded-For` エントリが `ipv4:port` または `[ipv6]:port` として書かれている場合（一部のロードバランサーがそうするように）、ポートを削除して読み取られます。ポートが追加された括弧なしの IPv6 アドレスは、異なるアドレスとして読み取られるか、まったく読み取られない可能性があるため、そのフォームを書き込むプロキシのポートオプションをオフにしてください。                                       |

<h3 id="oidc">
  `oidc`
</h3>

`oidc` ブロックはゲートウェイをアイデンティティプロバイダーに接続し、誰がサインインできるかを決定します。発行者と OAuth クライアントに名前を付け、メールとグループを含むクレームをマップし、メールドメインまたはグループによるサインインを制限します。

OpenID Connect（OIDC）はゲートウェイがアイデンティティプロバイダーで使用する SSO プロトコルです。IdP 側で登録する内容については、[アイデンティティプロバイダーのセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください。

| フィールド                           | 必須  | 説明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------- | --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | はい  | OIDC 検出ベース。`/.well-known/openid-configuration` で検出を提供する必要があります。本番環境では HTTPS を使用してください。ゲートウェイは `http://` 発行者を受け入れます。`http://localhost:8081` などのループバック発行者は、`CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` がゲートウェイの環境に設定されていない限り、[SSRF ガード](/docs/ja/claude-apps-gateway-deploy#threat-model-summary)によって拒否されます。                                                                                                                                                                                                                                                                                                                                                                                        |
| `client_id` / `client_secret`   | はい  | OAuth クライアント登録から                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `allowed_email_domains`         | いいえ | `email` クレームがこれらのドメインのいずれかにない id\_token を拒否します。大文字と小文字を区別しません。マルチテナント IdP の設定ミスに対する多層防御です。この設定とは無関係に、`email_verified` クレームが明示的に `false` である id\_token は常に拒否されます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `allowed_groups`                | いいえ | サインインを `groups_claim` に対してマッチされるこれらの IdP グループのメンバーに制限します。許可されたメールドメイン内のユーザーがこれらのグループのいずれにも属していない場合は拒否されます。IdP がグループクレームを発行する必要があります。マッチングは、そのクレーム内の値に対する正確で大文字と小文字を区別する文字列比較であり、ゲートウェイはネストされたグループを展開しません。サブグループのメンバーを許可するには、ここにサブグループをリストするか、IdP を設定してフラット化されたメンバーシップを発行してください。                                                                                                                                                                                                                                                                                                                                                                                                    |
| `groups_claim`                  | いいえ | グループメンバーシップを含む id\_token クレーム。デフォルト `groups`。Microsoft Entra は `roles` の下にアプリロールを発行します。フラットキーまたは `/resource_access/gateway/roles` などのネストされたクレーム用の RFC 6901 JSON ポインターを受け入れます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `google_groups`                 | いいえ | Google Workspace Admin SDK Directory API を通じてサインインしたユーザーのグループを検索します。Google の id\_token はグループクレームを含まないためです。`service_account_json_path` を `https://www.googleapis.com/auth/admin.directory.group.readonly` スコープでドメイン全体の委任を持つサービスアカウントキーファイルに設定し、`admin_email` をサービスアカウントが偽装する Workspace 管理者に設定します。Directory API は実際の管理者サブジェクトを必要とします。各ユーザーのグループメールアドレスがそのグループクレームになるため、`allowed_groups` と `managed.policies.match.groups` はグループメールでマッチします。                                                                                                                                                                                                                                           |
| `email_claim`                   | いいえ | ユーザーのメールを含む id\_token クレーム。デフォルト `email`。ADFS や Entra B2C などの一部の IdP は、代わりに `upn` または `preferred_username` を発行します。フラットキー、JSON ポインター、または最初に存在するキーが使用されるフォールバックキーのリストを受け入れます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `scopes`                        | いいえ | ゲートウェイが要求する OIDC スコープの完全なオーバーライド。デフォルト `[openid, profile, email, offline_access]`。IdP が認識しないスコープを拒否する場合、またはグループやメールを発行するためにカスタムスコープが必要な場合に設定します。`openid` を含める必要があります。`offline_access` を削除するとリフレッシュトークンが無効になるため、開発者は `session.ttl_hours` ごとにブラウザログインを再実行します。Google のリフレッシュトークンフローなどの IdP ごとのスコープレシピについては、[アイデンティティプロバイダーのセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください。                                                                                                                                                                                                                                                                                  |
| `scope_on_refresh`              | いいえ | リフレッシュトークンを交換するときに、サインインリクエストと同じリストで `scope` も送信します。デフォルト `false`。リフレッシュリクエストは `scope` を省略します。ほとんどの IdP はすべてのリフレッシュで id\_token を返し、これを必要としません。IdP が再度 `openid` を要求された場合にのみリフレッシュ時に id\_token を返す場合は `true` に設定します。Okta はそのリフレッシュグラントについてこれを文書化しています。id\_token がない場合、すべてのリフレッシュはリフレッシュされたアクセストークンを受け入れる IdP の userinfo エンドポイントに依存します。グループでサインインまたはポリシーマッチをゲートし、IdP のリフレッシュ時 id\_token がそれらを省略する場合は、`userinfo_fallback: true` も設定して、ゲートウェイが userinfo エンドポイントからそれらを入力するようにしてください。許可されたスコープが要求されたスコープより少ない IdP は、これがオンの場合、既存のセッションを含めて `invalid_scope` でリフレッシュを拒否できます。`scopes` にエントリを追加した後にリフレッシュが `token_endpoint` で失敗し始めた場合は、キーを設定解除してください。ゲートウェイサーバーで Claude Code v2.1.260 以降が必要です。 |
| `extra_auth_params`             | いいえ | IdP 認可リクエストに逐語的に追加される追加クエリパラメーター。これは、Google リフレッシュトークンの `access_type: offline`、一部の Entra テナントの `domain_hint`、またはステップアップフローの `acr_values` など、IdP 固有の動作のオーバーライドメカニズムです。ゲートウェイが管理するプロトコルパラメーターはオーバーライドできません。`state`、`nonce`、`redirect_uri`、PKCE、`scope`、`response_type`、`response_mode`、`client_id`。                                                                                                                                                                                                                                                                                                                                                                                  |
| `userinfo_fallback`             | いいえ | id\_token がメールまたはグループを省略する場合、`/userinfo` からそれらを取得します。Keycloak 軽量アクセストークン、Okta org サーバー、ADFS 最小トークンに必要です。id\_token は権威的なままです。userinfo はギャップのみを埋めます。デフォルト `false`。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `use_pkce`                      | いいえ | 認可リクエストで PKCE（S256）チャレンジを送信します。デフォルト `true`。IdP がこの機密クライアントの PKCE を拒否する場合のみ `false` に設定します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `clock_skew_seconds`            | いいえ | id\_token 時間クレームを検証するときにクロックドリフトを許容します。デフォルト `0`。厳密です。サインイン直後にホスト/IdP クロックスキューのため「トークン期限切れ/まだ有効でない」エラーが表示される場合は、これを上げてください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `token_endpoint_auth_method`    | いいえ | トークンエンドポイント認証方法をオーバーライドします。`client_secret_basic` または `client_secret_post` を受け入れます。デフォルトで自動ネゴシエーションされます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `id_token_signed_response_alg`  | いいえ | 予想される id\_token 署名アルゴリズム。デフォルト `RS256`。ES256、PS256、または EdDSA で署名する IdP に設定します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `additional_authorized_parties` | いいえ | `client_id` を超えて受け入れる追加の `azp` 値。Keycloak ブローカーとトークン交換フロー用                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `discovery_url`                 | いいえ | `issuer` から導出する代わりに、この URL から検出ドキュメントを取得します。発行者ホストを書き換えるプロキシの背後にある IdP 用です。パスは `/.well-known/` を含む必要があります。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `use_proxy`                     | いいえ | ゲートウェイ独自の IdP リクエストを `HTTPS_PROXY` または `HTTP_PROXY` のフォワードプロキシを通じて送信し、`NO_PROXY` を尊重します。設定解除または `false` の場合、これらのリクエストは直接実行されます。v2.1.227 以降が必要です。以下の[フォワードプロキシを通じた IdP リクエスト](#idp-requests-through-a-forward-proxy)を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `form_action_origins`           | いいえ | `/device` ページの `Content-Security-Policy: form-action` ディレクティブの追加オリジン。ゲートウェイはすでに `'self'` と検出された `authorization_endpoint` オリジンを許可していますが、Chrome は `form-action` をリダイレクトチェーン全体に対して強制します。IdP が Azure AD が ADFS にフェデレーションされている場合、ハブスポーク Okta、または企業 SSO インターセプターなど、2 番目のホストを通じてリダイレクトする場合は、認可リクエストがリダイレクトされる可能性があるすべてのオリジンをリストします。                                                                                                                                                                                                                                                                                                                                                       |
| `ca_cert_pem`                   | いいえ | ファイルへのパスではなく、PEM エンコードされた CA 証明書自体。IdP リクエストのみのシステムトラストストアを置き換えます。マウントされたファイルを読み込むには、`${file:/etc/gateway/idp-ca.pem}` と書きます。企業 PKI の背後にある Keycloak または Dex に使用します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="idp-requests-through-a-forward-proxy">
  フォワードプロキシを通じた IdP リクエスト
</h4>

推論アップストリームはすべてのバージョンで `HTTPS_PROXY` と `HTTP_PROXY` を尊重します。IdP、検出、JWKS、トークン、userinfo へのゲートウェイ独自のリクエストは、`oidc.use_proxy: true` を設定しない限り直接実行されます。これには v2.1.227 以降が必要です。プロキシ変数が設定され、`use_proxy` が設定解除され、発行者が `NO_PROXY` でカバーされていない場合、ゲートウェイはこれらのリクエストを直接保つし、ブート時に選択するよう求める通知をログに記録します。`use_proxy: false` はそれらを直接保つし、通知をサイレンスします。

`use_proxy: true` の場合、ポッドは各 IdP エンドポイントのホスト名を自身で解決し、プロキシに解決された IP アドレスへの `CONNECT` を要求するため、プロキシは発行者だけでなく、検出ドキュメントが名前を付けるすべてのホストの IP アドレスへの `CONNECT` を受け入れる必要があります。`http://` プロキシ URL を使用してください。`ca_cert_pem` と[SSRF ガード](/docs/ja/claude-apps-gateway-deploy#threat-model-summary)はプロキシされたパスにも適用されます。

<h3 id="session">
  `session`
</h3>

`session` ブロックはサインイン後にゲートウェイが鋳造するベアラートークンを形成します。それらに署名するシークレットと、どのくらい長く生きるかです。

| フィールド        | 必須  | 説明                                                                                                                                                                                                                                                              |
| ------------ | --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | はい  | 少なくとも 32 バイトのエントロピー。例えば `openssl rand -base64 32` から。ゲートウェイの HS256 ベアラートークンに署名します。単一の文字列または回転用の配列を受け入れます。インデックス 0 が署名し、すべてのエントリが検証します。回転するには、新しいシークレットを先頭に追加し、`ttl_hours` を待ってから古いものを削除します。                                                                     |
| `ttl_hours`  | いいえ | ゲートウェイベアラートークンの有効期間。デフォルト `1`。IdP がリフレッシュトークンを発行する場合、CLI は有効期限前に静かにリフレッシュします。より短い有効期間はより速く廃止されます。より長い有効期間は IdP ラウンドトリップが少なくなります。IdP が `offline_access` が利用できないためリフレッシュトークンを発行できない場合、静かなリフレッシュはないため、開発者が 1 時間ごとにブラウザログインに戻されるのを避けるために、これを `8` または `12` に上げてください。 |

<h3 id="store">
  `store`
</h3>

`store` ブロックはゲートウェイを PostgreSQL データベースに指します。これはデバイスグラントとレート制限カウンターを保持します。

| フィールド             | 必須  | 説明                                                                                                                                                                                                                                                                                                       |
| ----------------- | --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | はい  | `postgres://` または `postgresql://` URL。必須。デバイスグラント集合場所。ブラウザコールバックが書き込み、ポーリング CLI が読み取る場所。クロスレプリカ状態が必要です。ゲートウェイはブート時とアップグレード時に独自のスキーママイグレーションを実行するため、ロールはターゲットスキーマでテーブルを作成および変更する権限が必要です。[アップグレード](/docs/ja/claude-apps-gateway-deploy#upgrades)と [Postgres](/docs/ja/claude-apps-gateway-deploy#postgres)を参照してください。 |
| `username`        | いいえ | `postgres_url` のユーザーをオーバーライドします                                                                                                                                                                                                                                                                          |
| `password`        | いいえ | データベース認証情報。認証情報が URL から外れるように、ここに設定します。任意の文字を受け入れ、URL 認証情報よりも優先されます。                                                                                                                                                                                                                                     |
| `max_connections` | いいえ | レプリカあたりの Postgres 接続プール サイズ。デフォルト `5`。保守的で共有データベースに優しいです。[支出制限](#admin)が有効な場合、ホットパスは推論リクエストごとにいくつかの操作を実行するため、負荷の下で専用データベースの場合は上げ、レプリカ × これをデータベースの `max_connections` 以下に保ちます。                                                                                                                            |

ローカル開発の場合、`postgres_url` を使い捨て Postgres コンテナに指します。例えば `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`。

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` は順序付きリストです。ゲートウェイは要求されたモデルを解決する最初のアップストリームに推論を転送します。

`5xx`、`429`、`401`、`403`、`404`、またはタイムアウトで、ゲートウェイは次のアップストリームにフェイルオーバーします。他の `4xx` はそうしません。これらのエラーはリクエストではなくアップストリームに起因するためです。`401` または `403` はゲートウェイ独自の認証情報がそのアップストリームに対して失敗したことを意味します。`404` はそのアップストリームが要求されたモデルを提供しないことを意味するため、リスト内の後のアップストリームはまだそれを提供できます。

アップストリームで `forward_user_identity: true` を設定する場合、開発者のメールを含むリクエストに返す `429` はフェイルオーバーしません。[per-user limit denial が開発者に到達する方法](#per-user-identity-headers-for-a-proxy-you-run)を参照してください。

`404` でのフェイルオーバーにはゲートウェイ v2.1.198 以降が必要です。以前のリリースは、リスト内の後のアップストリームがモデルを提供している場合でも、最初の `404` をクライアントに返しました。

同じプロバイダーの複数のアップストリームは、異なる `name:` を設定する必要があります。

Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、Microsoft Foundry クライアントはスタートアップ時に一度構築され、それらの SDK は認証情報を内部的にリフレッシュするため、クラウド認証情報のローテーションは再起動を必要としません。静的 Anthropic API キーとベアラーはスタートアップ時に読み取られます。[Anthropic API](#anthropic-api) を参照してください。

<h4 id="upstream-error-messages">
  アップストリームエラーメッセージ
</h4>

ゲートウェイはアップストリームの 1 つのエラー応答、またはアップストリームがどのように応答したかに応じて独自の `502` を返します。

* **ゲートウェイが[フェイルオーバー](#multiple-upstreams)しないステータスをアップストリームが返した**。そのアップストリームの応答。ゲートウェイはさらなるアップストリームを試みません。
* **ゲートウェイが試みたすべてのアップストリームが[フェイルオーバー](#multiple-upstreams)する方法で失敗した**。最後の `429`。どれも `429` を返さなかった場合、ゲートウェイは順に、最後の `401` または `403`、最後の `404`、最後の `501` を優先します。どれもそれらのいずれも返さなかった場合、ゲートウェイ独自の `502`。`all upstreams failed (N attempted)`。N は [`upstreams`](#upstreams) のすべてのエントリをカウントします。要求されたモデルを提供しないためゲートウェイがスキップしたエントリを含みます。

ゲートウェイがアップストリームの応答を返す場合、アップストリームのステータスコードを保ちます。アップストリームのメッセージを保つかどうかはプロバイダーに依存します。Anthropic API アップストリームのエラー本体は開発者に変更されずに到達します。

Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、Microsoft Foundry アップストリームはそれらのエラーテキストでアカウント ID、ロール ARN、プロジェクト ID に名前を付けることができます。ゲートウェイはその完全なテキストを[操作ログ](/docs/ja/claude-apps-gateway-deploy#logs)に記録します。開発者がこれらのアップストリームから見るものは拒否に依存します。

* Anthropic の標準エラーエンベロープの `400` または `413`。`prompt is too long` などのアップストリーム独自のメッセージ。Claude Platform on AWS、Agent Platform、Microsoft Foundry はモデル API 拒否のためこのエンベロープを返します。
* プロバイダー独自の形状の `400` または `413`。`capability_rejected:` トークン。ゲートウェイが拒否を分類できない場合、`400` で `upstream rejected the request` または `413` で `request too large for this upstream`。
* その他のステータス。`429` で `upstream rate limit exceeded` などのステータスごとの汎用コピー。

例えば、ゲートウェイは Amazon Bedrock の `Input is too long for requested model.` を `capability_rejected: prompt_too_long` に置き換えます。Claude Code はそのトークンで[自動的にコンパクト](/docs/ja/errors#prompt-is-too-long)にします。`prompt is too long` と同じようにです。

クラウドアップストリームの `400` または `413` メッセージを保つか、`capability_rejected:` トークンで置き換えるには、ゲートウェイ v2.1.233 以降が必要です。

<h4 id="anthropic-api">
  Anthropic API
</h4>

最小限の Anthropic アップストリームは [Claude Console](https://platform.claude.com) からの API キーです。

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OR an OAuth bearer (e.g. a Workload-Identity-Federation-exchanged token):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # default; override for a forward proxy
```

2 つの認証情報フォームは送信するヘッダーが異なります。

* **`api_key`**。`x-api-key` を送信します。Claude Console でローテーションし、環境変数を更新します。
* **`oauth_token`**。`Authorization: Bearer` を送信します。組織が長期 API キーではなく短期トークンを発行する場合、ベアラーフォームを使用します。ベアラーはスタートアップ時に一度読み取られるため、シークレットを再マウントして再起動することでリフレッシュします。

静的キーまたはベアラーの代わりに、Workload Identity Federation を使用できます。[Workload Identity Federation ガイド](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)に従って federation ルールを作成し、ワークロードの OIDC JWT をファイルとしてマウントします。例えば、Kubernetes プロジェクトサービスアカウントトークンまたは CI プラットフォームの id-token。ゲートウェイは JWT を短期ベアラーと交換し、自動的にリフレッシュします。トークンファイルはすべての交換で再読み取りされるため、ローテーションされたプロジェクトトークンは再起動なしで取得されます。

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # required if the rule covers >1 workspace
      # service_account_id: svac_...   # optional expected-target check
```

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  あなたが実行するプロキシの per-user identity ヘッダー
</h5>

`provider: anthropic` アップストリームの `base_url` を Anthropic API ではなく実行するプロキシに指すことができます。各リクエストを送信した開発者をそのプロキシに伝えるには、そのアップストリームで `forward_user_identity: true` を設定します。プロキシはその後、開発者ごとに支出を属性化できます。ゲートウェイで Claude Code v2.1.233 以降が実行されている必要があります。

例えば、`upstream-gateway.internal.example.com` のプロキシの場合。

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # default false
```

ゲートウェイはそのアップストリームに転送するすべてのリクエストにこれらのヘッダーを追加します。

| ヘッダー                          | 値                                   |
| ----------------------------- | ----------------------------------- |
| `x-litellm-end-user-id`       | IdP が提供した場合、開発者のメール。                |
| `x-claude-gateway-user-id`    | トークンの `sub` クレームからの開発者の IdP サブジェクト。 |
| `x-claude-gateway-user-email` | IdP が提供した場合、開発者のメール。                |

IdP トークンがメールを含まない場合、ゲートウェイは `x-claude-gateway-user-id` のみを送信し、2 つのメールヘッダーを省略します。IdP がメールを別のクレームに入れる場合は、[`oidc.email_claim`](#oidc) をそのクレームに設定します。

プロキシが開発者のメールを含むリクエストに `429` で応答する場合、ゲートウェイはその応答を開発者にそのまま返し、次のアップストリームにフェイルオーバーしません。プロキシの per-user バジェットまたはレート制限が保持されます。プロキシの他の応答は通常の[フェイルオーバールール](#upstreams)に従います。開発者の IdP トークンがメールを含まない場合、ゲートウェイはメールヘッダーなしでリクエストを転送するため、そのようなリクエストへの `429` はアップストリーム容量としてカウントされ、フェイルオーバーします。ゲートウェイサーバーで v2.1.267 より前では、すべての `429` がフェイルオーバーしました。

`forward_user_identity` を、`base_url` が操作するプロキシであるアップストリームにのみ設定します。ゲートウェイは開発者メールを、その `base_url` が名前を付けるサーバーに送信します。`base_url` が Anthropic API（デフォルト）の場合、ゲートウェイは起動を拒否します。

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

ゲートウェイが置き換えるか前に置く、クライアント側の Amazon Bedrock デプロイメントについては、[Amazon Bedrock の Claude Code](/docs/ja/amazon-bedrock) を参照してください。ゲートウェイ側のアップストリーム。

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferred: AWS default credential chain
    # OR explicit credentials:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OR a Bedrock API bearer token:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Override the bedrock-runtime endpoint for FIPS or VPC-endpoint deployments:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

空の `auth` ブロックは AWS SDK のデフォルト認証情報チェーンを使用します。環境変数、`~/.aws/credentials`、ECS タスクロール、EC2 インスタンスメタデータ、または EKS の IRSA。本番環境では、コンテナイメージに静的キーを埋め込む代わりに、ゲートウェイポッドに IAM ロールを付与します。

明示的な認証情報は完全である必要があります。`aws_access_key_id` と `aws_secret_access_key` が一緒に設定されていない場合、または `aws_session_token` が設定されていない場合、ゲートウェイはブート時に失敗します。v2.1.207 より前では、部分的な `auth:` ブロックが検証に合格しました。

| セットアップ    | 方法                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 権限    | ゲートウェイのプリンシパルに推論プロファイル ARN と基盤モデル ARN の両方に `bedrock:InvokeModel` と `bedrock:InvokeModelWithResponseStream` を付与します。US リージョンの組み込みカタログの場合。`arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` と `arn:aws:bedrock:*::foundation-model/anthropic.*`。また、基盤モデル ARN に `bedrock:CountTokens` を付与します。ゲートウェイはそれを使用して、クライアントが放棄したリクエストの入力トークンをカウントするため、[支出制限](#admin)は正確なままです。これなしでは、ゲートウェイはそのカウントのために 1 トークン Bedrock リクエストにフォールバックします。 |
| モデルアクセス   | Amazon Bedrock は商用リージョンでデフォルトでモデルアクセスを有効にします。残りのアカウントレベルゲートは Anthropic のワンタイムユースケースフォームです。AWS アカウント内の誰もそれを送信していない場合、Amazon Bedrock コンソールを開き、モデルカタログから Anthropic モデルを選択し、フォームを完成させます。AWS Organizations フォームと送信者が必要とする権限については、[ユースケース詳細を送信](/docs/ja/amazon-bedrock#1-submit-use-case-details)を参照してください。                                                                                                                                             |
| EKS（IRSA） | 上記のポリシーと、クラスターの OIDC プロバイダーのトラストポリシーを持つ IAM ロールを作成します。ゲートウェイのサービスアカウントにスコープされます。サービスアカウントに `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway` でアノテーションを付けます。`auth: {}` はそれを取得します。                                                                                                                                                                                                                                          |
| ECS / EC2 | IAM ロールをタスク定義またはインスタンスプロファイルにアタッチします。`auth: {}` はそれを取得します。                                                                                                                                                                                                                                                                                                                                                                                    |
| その他の場所    | `AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY`、`AWS_SESSION_TOKEN` 環境変数を通じて認証情報を渡すか、`${VAR}` 展開で `auth:` に明示的に設定します。                                                                                                                                                                                                                                                                                                                             |
| リージョン     | `region:` は API エンドポイントリージョンです。クロスリージョン推論プロファイルは、どれを選択するかに関わらず、地理（US、EU、APAC）全体でルーティングします。非 US リージョンまたはプロビジョニングされたスループット ARN の場合、正しいアップストリームごとの ID を持つ [`models:`](#models) ブロックを追加します。                                                                                                                                                                                                                                                       |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS は `aws-external-anthropic.<region>.api.aws` で AWS インフラストラクチャ上の第一者 Anthropic API を提供します。第一者モデル ID を使用し、送信されたとおりに `anthropic-beta` ヘッダーを尊重し、`count_tokens` を提供するため、Bedrock 固有の翻訳は適用されません。`anthropicAws` プロバイダーには Claude Code v2.1.198 以降が必要です。以前のゲートウェイリリースはブート時にそれを拒否します。

同じプラットフォームのクライアント側デプロイメントについては、[Claude Platform on AWS の Claude Code](/docs/ja/claude-platform-on-aws) を参照してください。ゲートウェイ側のアップストリーム。

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # sent as x-api-key
    # OR SigV4 via the AWS default credential chain:
    # auth: {}
    # OR explicit SigV4 credentials:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Override the derived endpoint:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

プラットフォームは Amazon Bedrock とは別の AWS アカウントで実行され、独自のサービス名 `aws-external-anthropic` の SigV4 リクエストに署名するため、Bedrock スコープの IAM ロールはそれを認可しません。`auth.api_key` の API キーは SigV4 認証情報も設定されている場合に優先されます。空の `auth` ブロックは AWS SDK のデフォルト認証情報チェーンを使用します。[Amazon Bedrock](#amazon-bedrock) アップストリームが使用するのと同じチェーンです。

| フィールド                                                   | 必須  | 説明                                                                                                |
| ------------------------------------------------------- | --- | ------------------------------------------------------------------------------------------------- |
| `region`                                                | はい  | AWS リージョン。小文字、数字、ハイフン。ゲートウェイは `https://aws-external-anthropic.<region>.api.aws` としてエンドポイントを導出します。 |
| `workspace_id`                                          | はい  | すべてのリクエストでヘッダーとして送信されます。プラットフォームはそれを必要とします。                                                       |
| `auth.api_key`                                          | いいえ | プラットフォームの API キー。`x-api-key` として送信されます。ベアラートークンではありません。2 つの認証モードは API キーまたは SigV4 です。             |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | いいえ | 明示的な SigV4 認証情報。一方を他方なしで設定するとブート時に失敗します。`auth.aws_session_token` はそれらと一緒に受け入れられます。                |
| `base_url`                                              | いいえ | 導出されたエンドポイントをオーバーライドします。                                                                          |

プラットフォームは第一者モデル ID を解決するため、組み込みカタログは [`models:`](#models) ブロックなしでそれにルーティングします。`models:` リストをキュレートする場合、エントリを `anthropicAws:` で第一者 ID でキーします。

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

同等のクライアント側セットアップについては、[Google Cloud の Claude Code](/docs/ja/google-vertex-ai) を参照してください。ゲートウェイ側のアップストリーム。

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # preferred: Application Default Credentials
    # OR a service account key file:
    # auth: { service_account_json: /secrets/sa.json }
    # Override the aiplatform endpoint for Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

空の `auth` ブロックは Application Default Credentials を使用します。`GOOGLE_APPLICATION_CREDENTIALS`、GCE メタデータ、または GKE Workload Identity。サービスアカウント JSON キーファイルはサポートされていますが、推奨されません。Workload Identity を使用するか、GCE または Cloud Run インスタンスにサービスアカウントをアタッチします。

`region: global` を設定して、リージョナルエンドポイントの代わりに [Google Cloud の Agent Platform のグローバルエンドポイント](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)を使用します。Google はその後、各リクエストを利用可能なリージョンにルーティングするため、リージョンごとのモデル可用性を追跡しません。特定のリージョンを設定するとすべてのリクエストをそれにピンします。

| セットアップ                 | 方法                                                                                                                                                                         |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 権限                 | ゲートウェイのサービスアカウントにプロジェクトで `roles/aiplatform.user` を付与するか、`aiplatform.endpoints.predict` を持つカスタムロール。Google Cloud の Agent Platform API（`aiplatform.googleapis.com`）を有効にします。   |
| モデルアクセス                | Model Garden で、プロジェクトの Claude モデルを有効にします。それらは特定のリージョンに公開されます。サポートされているリージョンについてはモデルカードを確認してください。                                                                           |
| GKE（Workload Identity） | GCP サービスアカウントをゲートウェイの Kubernetes サービスアカウントにバインドし、KSA に `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com` でアノテーションを付けます。`auth: {}` はそれを取得します。 |
| Cloud Run / GCE        | サービスのサービスアカウントを `roles/aiplatform.user` を持つものに設定します。`auth: {}` はそれを取得します。                                                                                                  |
| その他の場所                 | `auth: { service_account_json: /secrets/sa.json }`。JSON キーファイルへのパス。マウントされたシークレットとして。フィールドはキーコンテンツではなくファイルパスを取得するため、`${file:…}` 展開は関係ありません。                                 |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

クライアント側の Microsoft Foundry デプロイメントについては、[Microsoft Foundry の Claude Code](/docs/ja/microsoft-foundry) を参照してください。ゲートウェイ側のアップストリーム。

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferred: DefaultAzureCredential / Managed Identity
    # OR an API key:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` は `DefaultAzureCredential` を通じて解決します。AKS、ACI、または App Service の Managed Identity。Azure CLI。または環境認証情報。API キーは機能しますが、プロジェクト全体であり、自動的にローテーションしません。Microsoft Foundry のエンドポイントは `resource:` から導出されます。Azure Government などのソブリンクラウドのオプション `base_url` を設定してオーバーライドします。

| セットアップ              | 方法                                                                                                                                                   |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                | ゲートウェイのアイデンティティに Microsoft Foundry リソースで `Azure AI User` または `Cognitive Services User` を付与します。                                                       |
| デプロイメント             | Microsoft Foundry は正規モデル ID ではなく、管理者が選択したデプロイメント名を使用します。各正規 ID をデプロイメント名にマップする [`models:`](#models) ブロックを追加します。                                      |
| AKS（ワークロードアイデンティティ） | User-Assigned Managed Identity をクラスターの OIDC 発行者とフェデレーションし、ゲートウェイのサービスアカウントにバインドします。`use_azure_ad: true` は `WorkloadIdentityCredential` を通じてそれを取得します。 |
| ACI / App Service   | リソースでシステム割り当てまたはユーザー割り当てのマネージドアイデンティティを有効にします。`use_azure_ad: true` はそれを取得します。                                                                        |
| その他の場所              | `auth: { api_key: "${FOUNDRY_API_KEY}" }`。`{ }` 内の `${…}` を引用します。                                                                                    |

<h4 id="multiple-upstreams">
  複数のアップストリーム
</h4>

同じプロバイダーは異なる `name:` で複数回表示できます。これは異なるリージョン、異なるアカウント（異なる認証情報チェーン経由）、プロビジョニングされたスループット対オンデマンド、およびクロスプロバイダーフォールバックをカバーします。

ゲートウェイはアップストリームを順に試みます。`5xx`、`429`、`401`、`403`、`404`、タイムアウト、および欠落エンドポイント（`501`）がフェイルオーバーします。他の `4xx` はそうしません。

`429` はアップストリーム容量ごとであるため、プロビジョニングされたスループット（PT）枯渇はオンデマンドにフェイルオーバーします。アップストリームで [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) を設定する場合、開発者のメールを含むリクエストへの `429` は per-user 拒否であり、フェイルオーバーしません。

`404` はアップストリームモデル可用性ごとであるため、モデルを有効にしていないアップストリームは、それを提供する後のアップストリームをブロックしません。要求されたモデルを解決できないアップストリームはネットワークラウンドトリップなしでスキップされます。

この例は、プロビジョニングされたスループット Amazon Bedrock 割り当てを最初にルーティングし、オンデマンドと 2 番目のアカウントにオーバーフローし、最後に Anthropic API にフォールバックします。

```yaml theme={null}
upstreams:
  # Primary: provisioned throughput in your home region.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Overflow: on-demand cross-region.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Different account: a separate Bedrock allotment via assumed-role creds.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Last resort: direct Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Per-upstream model IDs are keyed on the upstream's `name:`.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| レバー                | 方法                                                                                                                                                                                                                                                                                     |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 異なるリージョン           | リージョンごとに 1 つの Amazon Bedrock アップストリーム。それぞれ独自の `region:`。[`auto_include_builtin_models: true`](#models) を使用すると、クロスリージョン推論プロファイルは自動的にルーティングします。リージョンピン留めデプロイメントの場合は `models:` ブロックを使用します。                                                                                                |
| 異なるアカウント           | アカウントごとに 1 つの Amazon Bedrock アップストリーム。それぞれ `auth:` に独自の認証情報。デフォルトチェーン（`auth: {}`）はポッドのアイデンティティを使用します。2 番目のアカウントの場合は、明示的な認証情報またはベアラートークンを設定します。                                                                                                                                         |
| プロビジョニングされたスループット  | そのアップストリームの名前の `models:` でモデルをプロビジョニングされたスループット ARN にマップします。他のアップストリームはオンデマンド ID を保つため、PT 容量はフェイルオーバーする前に枯渇します。                                                                                                                                                                        |
| VPC / FIPS エンドポイント | アップストリームで `base_url:` を VPC エンドポイントまたは FIPS エンドポイント URL に設定します。                                                                                                                                                                                                                        |
| モデルスコープルーティング      | 組み込み Claude モデルではないカスタムモデル `id` のみが、`upstream_model:` マップから欠落しているアップストリームをスキップします。ゲートウェイはすべてのアップストリームで組み込みモデルを試み、マップにエントリがない場合はプロバイダーのデフォルト ID を使用するため、組み込みモデルの場合、マップはアップストリームが試みられるかどうかではなく、アップストリームが受け取る ID を変更します。ID を拒否するアップストリームは、他のアップストリームエラーと同じ[フェイルオーバールール](#upstreams)に従います。 |

クラウドプロバイダー間、または直接 Anthropic API へのフェイルオーバーは、リクエストを管理する契約、地理、およびその他の条件を変更します。

CLI はゲートウェイに同じ機能ゲーティングを適用します。特定のリクエストがどのアップストリームを提供するかに関わらず、フェイルオーバーはアップストリームが拒否する本体フィールドを送信しません。

<h2 id="optional-sections">
  オプションセクション
</h2>

<h3 id="admin">
  `admin`
</h3>

オプション。`/v1/organizations/spend_limits` を有効にします。これは Anthropic のパブリック Admin API をミラーリングし、`/v1/messages` で開発者ごとの支出強制を行います。[支出制限](/docs/ja/claude-apps-gateway-spend-limits)で、キャップがどのように設定および強制されるかを参照してください。このセクションは、機能をオンにしてチューニングする `gateway.yaml` キーをカバーします。

```yaml theme={null}
admin:
  # 管理エンドポイント用の名前付き静的 API キー。x-api-key として送信されます。
  # ID は監査ログに admin-key:<id> として表示されるため、各キーは
  # 属性可能です。回転用の配列：新しいキーを追加し、クライアントをロール、
  # 古いものを削除します。
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # 通常のゲートウェイ JWT（API キーなし）経由で完全な管理者を付与された IdP グループ。
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| フィールド                     | 必須  | 説明                                                                                                                                                                                                                                             |
| ------------------------- | --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | いいえ | `{id, key}` の配列。これらのいずれかと一致する `x-api-key` は、支出制限をリスト、設定、削除できます。キー値は少なくとも 32 文字である必要があります。`id` は `read_keys` と `write_keys` 全体で一意である必要があります。                                                                                                    |
| `read_keys`               | いいえ | `{id, key}` の配列。読み取り専用：すべての `GET` エンドポイント。キャップのリスト、ID による 1 つの取得、[`/effective`](/docs/ja/claude-apps-gateway-spend-limits#%2Feffective) と [`/audit`](/docs/ja/claude-apps-gateway-spend-limits#%2Faudit) の読み取りを含みます。                                     |
| `admin_groups`            | いいえ | IdP グループ名。`groups` クレームがこれらのいずれかを含むゲートウェイ JWT は、完全な管理者アクセス、読み取りと書き込みを持ち、`oidc:<sub>` として監査されます。人間の管理者に使用します。マシンに API キーを使用します。このリストの空のエントリはゲートウェイをブート時に停止します。[ゲートウェイをブート時に停止するマッチャー値](#matcher-values-that-stop-the-gateway-at-boot)を参照してください。 |
| `blocked_message`         | いいえ | ブロックされた開発者が見る `429 billing_error` に逐語的に追加されます。URL または Slack チャネルなど、完全な指示を書きます。未設定の場合、ゲートウェイはデフォルトメッセージのみを送信します。[強制がどのように機能するか](/docs/ja/claude-apps-gateway-spend-limits#how-enforcement-works)を参照してください。                                           |
| `audit_retention_days`    | いいえ | デフォルト `365`。古い `admin_audit` 行はスイープされます。                                                                                                                                                                                                       |
| `spend_retention_months`  | いいえ | デフォルト `13`。この期間より古い `spend` カウンター行はスイープされます。デフォルトは、年間比較レポート用に完全な年と現在の部分月を保持します。                                                                                                                                                                |
| `identity_retention_days` | いいえ | デフォルト `90`。`principal_emails` 行の最後に見た TTL。各開発者のメール、表示名、グループ（PII）を保持します。意図的に支出保持より短いため、プロビジョニング解除されたアイデンティティは、その匿名支出カウンターが残っている間に期限切れになります。                                                                                                     |
| `group_limit_mode`        | いいえ | `min`（デフォルト）または `max`。開発者が複数のグループにキャップがある場合、`min` は最も制限的なものを強制し、`max` は最も制限的でないものを強制します。強制と `/effective` の両方で使用されます。                                                                                                                           |

<h3 id="enforcement">
  `enforcement`
</h3>

`enforcement` ブロックは、ストアが利用できない場合の支出制限チェックの動作を制御します。

| フィールド                  | 必須  | 説明                                                                                                                                                                                                                                         |
| ---------------------- | --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `fail_closed_on_error` | いいえ | デフォルト `false`。支出強制は Postgres 停止時にオープンで失敗するため、推論は稼働したままです。`true` に設定してクローズで失敗：上限を超えた開発者はブロックされますが、ストアに到達できない場合は他のすべてもブロックされます。[`admin:`](#admin) ブロックが必要です：支出強制は `admin` が設定されている場合にのみ実行され、これを `true` に設定して `admin` ブロックなしでゲートウェイは起動を拒否します。 |

<h3 id="pricing">
  `pricing`
</h3>

`pricing` ブロックは、支出メーターに USD リスト価格の代わりに請求する内容を指示するため、キャップと [`/effective`](/docs/ja/claude-apps-gateway-spend-limits#%2Feffective) は契約レートを反映します。金額は USD のままで、請求書ではなく見積もりのままです。2 つの前提条件：

* ゲートウェイサーバー上の Claude Code v2.1.227 以降。以前のバージョンはブート時に不明なキーを拒否します。
* [`admin:`](#admin) ブロック、または v2.1.268 以降では、少なくとも 1 つのポリシーを持つ [`managed:`](#managed) ブロック。支出メーターのみが `pricing` を読み込むためです。ゲートウェイは `pricing` が設定されていて両方のブロックがない場合、起動を拒否します。

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| フィールド        | 必須  | 説明                                                                                                                                                   |
| ------------ | --- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | いいえ | デフォルト `1`。メーターはリスト価格またはオーバーライドされたかどうかに関わらず、すべてのメーター量にこれを乗算するため、`0.85` は価格の 85% を請求します。0 より大きく、最大 10 である必要があります。1 より上の値は [マークアップ](#mark-prices-up)です。 |
| `overrides`  | いいえ | `{upstream, model, input, output, cache_read, cache_write}` の行。USD/百万トークン。4 つのレートすべてが必須です。各レートは 0 より大きく、最大 10000 である必要があります。                         |

メーターがオーバーライド行をマッチする方法：

* 行は、`upstream`（[`upstreams[].name`](#upstreams)）が `model` に対して提供するリクエストのリスト価格を置き換えます。これには、より高い [高速モード](/docs/ja/fast-mode#understand-the-cost-tradeoff) レートが含まれるため、高速と標準リクエストは同じ 4 つのレートでメーターされます。
* `claude-sonnet-4-6` などの組み込み ID（[`models[].id`](#models) のようにマッチ）は、メーターがそのモデルとして価格設定するすべての日付形式、地域 Amazon Bedrock 形式、または Google Cloud の Agent Platform 形式をカバーします。エイリアスまたは推論プロファイル ARN などの他の文字列は、クライアントが送信した ID またはアップストリームに送信された文字列と大文字小文字を区別せずにマッチします。
* 行が重複する場合、メーターは最初の行ではなく最も具体的な行を選択します：アップストリームに送信された正確なモデル文字列である `model` を持つ行、次にクライアントが送信した正確な ID にマッチする行、次に組み込みモデルに名前を付ける行。
* 不明なアップストリーム名はブートに失敗し、1 つのアップストリームに対して同じモデルに名前を付ける 2 つの行も失敗します。これには、組み込みモデルの 2 つのスペルが含まれます。ゲートウェイはブート時に、リクエスト可能なモデルが使用できない行について警告します。
* Web 検索リクエストは \$0.01 リスト価格のままです。乗算器はそれらにも適用されます。

地域ごとのレートについては、各地域に独自の名前付きアップストリームを指定し、アップストリームごとに 1 つの行を指定します。

<h4 id="mark-prices-up">
  価格をマークアップする
</h4>

ゲートウェイサーバー上で v2.1.271 以降を使用すると、`multiplier` を 1 より上に設定でき、最大 10 まで、プロバイダーが請求するより多くをメーターするため、例えば内部チャージバックレート。この例は、すべてのリクエストを価格の 120% でメーターします：

```yaml theme={null}
pricing:
  multiplier: 1.2
```

[`admin:`](#admin) ブロックを使用すると、マークアップは支出制限にも適用されます。メーターは価格の 120% をカウントするため、開発者はキャップに早く到達します。ゲートウェイはブート時に、そのことを示す警告をログします。

乗算器は、アップストリームプロバイダーがリクエストに請求する内容を変更しません。

ゲートウェイが [署名されたクライアントにレートを送信](#send-the-rates-to-signed-in-clients)する場合、開発者はマークアップを見るために Claude Code v2.1.271 以降が必要です。以前のクライアントは 1 より上の `multiplier` を無視し、それなしでコストを表示します。

v2.1.271 より前のゲートウェイサーバーは、1 より上の `multiplier` を設定した場合、起動を拒否します。

<h4 id="send-the-rates-to-signed-in-clients">
  署名されたクライアントにレートを送信する
</h4>

ゲートウェイサーバー上で v2.1.268 以降を使用すると、ゲートウェイは `pricing` からのレートを提供する [`managed`](#managed) ポリシーに入れます。[`modelPricing`](/docs/ja/settings-reference#modelpricing) マネージド設定として。ポリシーにマッチした開発者は、`/usage`、ステータス行、OpenTelemetry で各モデル ID を提供する最初のアップストリームの `pricing` レートを見ます。ポリシーにマッチしない開発者はマネージド設定を受け取らないため、彼らの数字はリスト価格のままです。クライアントは Claude Code v2.1.242 以降で設定を適用します。

* ゲートウェイが追加するもの：ポリシーの `cli` ブロックが既に `modelPricing` を設定していない限り、ゲートウェイは `multiplier` を追加し、クライアントがリクエストできるすべてのモデル ID について、そのモデル ID を提供する最初のアップストリームのオーバーライド行を追加します。フェイルオーバーアップストリームのみが請求するレートはゲートウェイに留まります。
* 1 つのポリシーをオプトアウト：ポリシーの `cli` ブロックで `modelPricing` を `{}` に設定し、その開発者はリスト価格のままです。
* ポリシー独自のレートを保持：`cli` ブロックが独自の `multiplier` または `overrides` で `modelPricing` を設定するポリシーは、その `modelPricing` 全体を保持し、ゲートウェイはそれに独自のレートを追加しません。

<h3 id="models">
  `models`
</h3>

`models` ブロックはオプションの管理者がキュレーションしたモデルリストで、`/v1/models` で提供され、アップストリームごとのモデル ID を変換するために使用されます。US 以外の Amazon Bedrock リージョン、Amazon Bedrock プロビジョニングスループット ARN、Microsoft Foundry デプロイメント名に必須です。

```yaml theme={null}
auto_include_builtin_models: true   # false：以下のリストのみを公開
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description：オプションのテキスト。クライアントに表示される場合がある
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # または推論プロファイル ARN
      foundry: your-opus-deployment-name
```

`upstream_model` の下の各キーは、設定されたアップストリームの `name` と一致する必要があります。デフォルトはプロバイダー名です。アップストリームと一致しないキーはブートに失敗するため、使用しないプロバイダーの行は省略します。

<h3 id="managed">
  `managed`
</h3>

`managed` ブロックは、IdP グループまたはメールドメインでキーイングされた、ロールベースのアクセスポリシーを定義します。ポリシーは順番に評価されます。最初のマッチが選択され、`match: {}` キャッチオール基盤にマージされます。ユーザーごとに `GET /managed/settings` で ETag/304 キャッシング付きで提供されます。

```yaml theme={null}
managed:
  policies:
    # 特定のグループを最初に。
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # デフォルトキャッチオール最後：認証されたすべてのユーザーにマッチします。
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

`match: {}` キャッチオール。慣例的に最後にリストされます。基盤層として扱われます。他のすべてのポリシーは、設定しないキーについてキャッチオールから継承するため、ロール別エントリは組織デフォルトから異なるものだけをリストする必要があります。マージルールはキータイプに依存します：

* **許可リスト**：`availableModels` と `permissions.allow`。特定のポリシーのリストは基盤のリストを完全に置き換えます。
* **拒否リストとフックアレイ**：`permissions.deny`、`permissions.ask`、`disabledMcpjsonServers`、`deniedMcpServers`、`blockedMarketplaces`、およびすべての `hooks` イベントタイプアレイ。これらは基盤とポリシーの和集合を取得するため、組織全体の拒否または監査フックは、ロール別オーバーライドによって誤ってドロップされることはできません。
* **レコードタイプキー**：`env`、`modelOverrides`、`skillOverrides`。これらは浅くマージするため、ロール別 `env` ブロックは設定するキーをオーバーライドし、基盤から残りを継承します。

`availableModels` は `/v1/messages` でサーバー側でも強制されるため、拒否されたモデルはクライアントが送信するものに関わらず `400` を返します。

ゲートウェイはリクエストをリレーする前に `model` 値自体を検証するため、形式が正しくない値がアップストリームに到達することはありません。2 つのケースで `400` でリクエストを拒否します：

* 値が欠落しているか空の場合、ゲートウェイはメッセージ `model is required` でリクエストを拒否します。このチェックには Claude Code v2.1.228 以降を実行しているゲートウェイが必要です。
* 値が存在しているが文字列ではない場合、ゲートウェイはメッセージ `model must be a string` でリクエストを拒否します。Claude Code v2.1.221 以降を実行しているゲートウェイが必要です。

| マッチャー                                               | 動作                                                                                             |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `match: {}`                                         | 認証されたすべてのユーザーにマッチします。これで開始し、後で上にグループスコープポリシーを追加します。                                            |
| `match: { groups: [a, b] }`                         | JWT の `groups` クレームがリストされたグループのいずれかを含む場合にマッチします。大文字小文字を区別します：グループは IdP の正確な大文字小文字と一致する必要があります。 |
| `match: { email_domain: example.com }`              | JWT の `email` クレームの最後の `@` の後の部分にマッチします。大文字小文字を区別しません。ポリシーごとに 1 つのドメインを受け入れます。                 |
| `match: { groups: [a], email_domain: example.com }` | 両方の条件がマッチする必要があります。                                                                            |

ポリシーにマッチしない認証されたユーザーは、ゲートウェイのデフォルトを取得します。これは、カタログ内のすべてのモデルと管理設定なしを意味します。最後に `match: {}` キャッチオールを追加して、保証されたデフォルトポリシーが必要な場合。

<Note>
  ゲートウェイは独自のユーザーディレクトリを保持しません。ユーザーの IdP トークンから各リクエストを認可し、トークンの `groups` クレームからグループメンバーシップを読み込み、それに対してポリシーを評価します。列挙するロスターはなく、事前作成するアカウントもありません。したがって、SCIM エンドポイントはありません。SCIM が同期するものがないためです。

  ユーザーとグループのライフサイクル管理を、真実の源である IdP のネイティブ SCIM プロビジョニングまたは専用アイデンティティガバナンスプラットフォームで実行します。メンバーシップとプロビジョニング解除はそこで管理され、トークンを通じてゲートウェイに自動的に流れます。Claude アカウント自体の SCIM プロビジョニングが必要な場合、それは [Claude for Enterprise](/docs/ja/admin-setup) 機能です。

  2 つの伝播クロックが適用されます：

  * **ポリシーコンテンツ**：ポリシーを編集して再デプロイすると、接続されたクライアントの次のマネージド設定ポーリング時に到達します。1 時間以内。[次の起動時にのみ適用される変更](/docs/ja/server-managed-settings#fetch-and-caching-behavior)を除きます。
  * **グループメンバーシップ**：ユーザーのグループメンバーシップを変更すると、どのポリシーが彼らにマッチするかが変わります。これは次のセッション再発行時に有効になります。つまり、次の無言リフレッシュ。`session.ttl_hours` で制限されます。
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  ゲートウェイをブート時に停止するマッチャー値
</h4>

ブート時に、ゲートウェイはすべてのポリシーの `match` ブロックと [`admin_groups`](#admin) リストをチェックします。これらの値のいずれかがゲートウェイをフィールドに名前を付けるエラーで停止します：

* 空の `groups` リスト
* `groups` または `admin_groups` の空のエントリ
* 空の `email_domain`
* `@`、空白、またはコンマを含む `email_domain`。ゲートウェイはこのチェック前に値をトリムし、1 つの先頭 `@` を削除します。`example.com` などの 1 つの裸のドメインを書きます。

v2.1.232 より前では、ゲートウェイはこれらの値で起動しました。各値はこの効果を持っていました：

* 空の `email_domain`：ゲートウェイはドメインチェックをスキップしたため、空の `email_domain` と `groups` リストなしのポリシーはすべての認証されたユーザーにマッチしました。
* 空の `groups` リスト：ポリシーは誰にもマッチしませんでした。
* `@`、空白、またはコンマを含む `email_domain`：ポリシーは誰にもマッチしませんでした。
* `groups` または `admin_groups` の空のエントリ：エントリはそのユーザーの IdP `groups` クレームも空のエントリを含む場合にのみユーザーにマッチしました。`admin_groups` では、そのマッチは管理者アクセスを付与しました。`admin_groups` リストに空のエントリが含まれていない場合、誰もこの方法で管理者アクセスを取得しませんでした。

<h4 id="what-goes-in-cli">
  `cli` に何が入るか
</h4>

各 `cli` 値は、完全な Claude Code `managed-settings.json` ドキュメント。MDM または `/etc/claude-code/managed-settings.json` を通じてデプロイするのと同じスキーマ。ここでは YAML として表現されます。CLI は、マネージド層で配信されたドキュメントを適用します。ユーザーとプロジェクト設定の上。サーバー管理設定の代わりに。したがって、[OS レベルのポリシーソースに制限されている設定](/docs/ja/server-managed-settings#current-limitations)（`policyHelper` と `wslInheritsWindowsSettings` など）を無視します。

ゲートウェイは、ブート時に CLI の設定スキーマに対して各ドキュメントを検証するため、認識されないトップレベルキーはすべての違反キーに名前を付けるエラーでブートに失敗します。スキーマの意図的にオープンな部分は、新しいクライアントがゲートウェイのスキーマが認識しないエントリを認識する可能性があるため、任意の値を受け入れます。これらのオープンキーは `env`、`pluginConfigs`、`permissions` の下にネストされたキーです。

検証はゲートウェイのインストール済みバージョンにバンドルされたスキーマを使用するため、新しい Claude Code リリースで導入されたトップレベル設定キーをマネージド設定に入れるには、最初にゲートウェイをアップグレードする必要があります。新しいポリシーを 1 つのクライアントでスモークテストしてから、ロールアウトします。

完全なキーリファレンスは [Claude Code 設定](/docs/ja/settings-reference#all-settings) にあります。オペレーターが最初に到達するキー：

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # モデルアクセス（/v1/messages でサーバー側でも強制）
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # 権限ポリシー
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # --dangerously-skip-permissions をブロック
        allowManagedPermissionRulesOnly: true     # ユーザー/プロジェクト権限ルールを無視

        # CLI プロセスにプッシュされた環境。DISABLE_UPDATES はバックグラウンドと手動更新をブロック；DISABLE_AUTOUPDATER はバックグラウンド更新のみを停止。
        env:
          DISABLE_UPDATES: "1"                    # 独自の配布経由でバージョンをピン

        # 組織全体のフック。フックコマンドはゲートウェイではなく開発者マシンで実行されるため、パスはポリシー内のすべてのクライアント OS に存在する必要があります。
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| キー                                         | 強制者          | 効果                                                                                                                                                                                                                                                    |
| ------------------------------------------ | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | ゲートウェイ + CLI | モデル許可リスト。`/v1/messages` でもチェックされるため、パッチされたクライアントはバイパスできません。                                                                                                                                                                                           |
| `permissions.allow` / `.deny`              | CLI          | ツールとコマンドルール。[権限](/docs/ja/permissions)を参照してください。                                                                                                                                                                                                           |
| `permissions.disableBypassPermissionsMode` | CLI          | `disable` に設定して [`bypassPermissions`](/docs/ja/permission-modes#skip-all-checks-with-bypasspermissions-mode) をブロック。すべてのツール呼び出しを自動承認するモード、および `--dangerously-skip-permissions` フラグ。                                                                         |
| `allowManagedPermissionRulesOnly`          | CLI          | `true` の場合、マネージド設定は権限ルールの唯一の設定ソースになります。[`allowManagedPermissionRulesOnly`](/docs/ja/settings-reference#allowmanagedpermissionrulesonly) エントリは Claude Code がその後無視するすべてのソースをリストします。                                                                          |
| `env`                                      | CLI          | CLI プロセスにマージされた環境変数。テレメトリ、自動更新、モデル名オーバーライドに使用します。                                                                                                                                                                                                     |
| `hooks`                                    | CLI          | 組織全体の [フック](/docs/ja/hooks)。                                                                                                                                                                                                                               |
| `managedMcpServers`                        | CLI          | リモート MCP サーバー [マッチする開発者ごとに提供](/docs/ja/managed-mcp#provide-servers-through-managed-settings)。彼らが自分で追加するサーバーの横に、`http` と `sse` のみ。[ポリシー内の MCP サーバー](#mcp-servers-in-a-policy)を参照してください。ゲートウェイサーバーとクライアント上で Claude Code v2.1.259 以降が必要です。以前のクライアントはキーを無視します。 |

これらの設定はネットワーク経由で到着するため、CLI は以下にリストされた設定を適用する前に、各開発者にセキュリティ承認ダイアログを表示します：

* `hooks`
* プロキシとベース URL 変数など、開発者の承認が必要な `env` 変数
* `apiKeyHelper` と `statusLine` などのシェル実行設定
* サンドボックスバイナリ設定 `sandbox.bwrapPath`、`sandbox.socatPath`、`sandbox.ripgrep`
* `sandbox.network.tlsTerminate` とプロキシポート設定など、トラフィックをインターセプト、認証情報を注入、または分離を弱める Sandbox 設定。[セキュリティ承認ダイアログ](/docs/ja/server-managed-settings#security-approval-dialogs)はすべてをリストします。

[承認メモリ](/docs/ja/server-managed-settings#approval-memory)は、承認がどのくらい続くか、およびダイアログが再度表示されるときをカバーします。

Claude Code は、モデル選択設定や数値制限など、開発者の承認ダイアログを表示せずに配信された `env` 変数の一部を適用します。他の配信変数は、開発者の承認が必要な場合があります。空でないプロキシ、ベース URL、または `OTEL_EXPORTER_OTLP_ENDPOINT` 値は常にそうです。配信変数が承認を必要とする場合、ダイアログはそれに名前を付けます。

[環境変数と承認ダイアログ](/docs/ja/server-managed-settings#environment-variables-and-the-approval-dialog)には詳細があります。配信値が承認を必要とするかどうかを決定する 4 つのプライバシートグルを含みます。v2.1.218 より前では、Claude Code はより少ない変数を開発者に尋ねずに適用したため、より多くの配信変数がダイアログをトリガーしました。

ゲートウェイの [テレメトリ](#telemetry) 設定は `OTEL_EXPORTER_OTLP_ENDPOINT` をプッシュするため、`telemetry.forward_to` を設定すると、各インタラクティブクライアントで承認ダイアログがトリガーされます。ダイアログは、組織から開発者を保護するのではなく、開発者のマシンを侵害または敵対的なゲートウェイから保護します。

`-p` フラグを使用した非インタラクティブ実行はダイアログを表示できません。その実行のみのためにプッシュされた設定を適用し、それらを承認済みとして記録しないため、開発者の次のインタラクティブセッションはまだダイアログを表示します。v2.1.207 より前では、非インタラクティブ実行は設定を承認済みとして保存し、後のインタラクティブセッションはそれらのダイアログを表示しませんでした。

開発者が拒否した場合、Claude Code はポリシーを適用せずにそのセッションを終了します。新しいフックまたはダイアログをトリガーする env var を広いポリシーにプッシュすることは、Claude Code がマッチする開発者に次の起動時にダイアログを表示することを意味します。ダイアログは実行中のセッションで次の時間ごとのポーリング時に表示され、そうでなければ開発者の次の起動時に表示されます。

`cli` キーは以前のリリースで `settings` という名前でした。その綴りはまだエイリアスとして受け入れられていますが、新しいデプロイメントは `cli` を使用する必要があります。

<h4 id="mcp-servers-in-a-policy">
  ポリシー内の MCP サーバー
</h4>

ポリシーが一致する Claude Code クライアントに MCP サーバーを提供するには、そのポリシーの `cli` ブロックで [`managedMcpServers`](/docs/ja/managed-mcp#provide-servers-through-managed-settings) を設定します。ゲートウェイサーバーとクライアント上で Claude Code v2.1.259 以降が必要です。

ゲートウェイは [Claude Code がクライアントで適用するのと同じルール](/docs/ja/managed-mcp#what-an-entry-can-contain)で各エントリをブート時にチェックし、エントリがチェックに失敗した場合、ゲートウェイは起動を拒否してエントリに名前を付けます。

`gateway.yaml` に `${VAR}` 参照を書く場合、ゲートウェイはブート時に [シークレット展開](#secret-expansion) を通じてその環境から解決するため、マッチする各クライアントはリテラル値を受け取り、それを読み込むことができます。[提供されたサーバーのヘッダーガイダンス](/docs/ja/managed-mcp#provide-servers-through-managed-settings)は展開された値に適用されます。

ゲートウェイは `cli` ブロック内の `.mcp.json` スペル `mcpServers` を拒否し、ブートエラーは使用するキーとして `managedMcpServers` に名前を付けます。v2.1.259 より前では、ゲートウェイは `cli` ブロック内の MCP サーバー定義を拒否しました。

<h4 id="claude-desktop-overlay">
  Claude Desktop オーバーレイ
</h4>

組織が [Claude Desktop](/docs/ja/desktop) もデプロイする場合、同じゲートウェイが両方のクライアントに提供します。Claude Desktop の [マネージド設定](https://claude.com/docs/third-party/claude-desktop/configuration) で `bootstrapUrl` を `<listen.public_url>/user/bootstrap` にポイントします。Claude Desktop はその URL から OAuth 発行者を導出し、このゲートウェイに対して同じデバイスコード サインインを実行し、レスポンスから設定を取得します。

<Note>
  ゲートウェイサーバー上で Claude Code v2.1.203 以降が必要で、明示的なオプトイン：`/user/bootstrap` はポリシーがマッチするユーザーが `desktop` キーを持たない限り 404 を返します。空の `desktop: {}` はポリシーをオプトインし、`match: {}` 基盤層の `desktop` キーはすべてのポリシーをオプトインします。監査ログは各リクエストを `desktop_bootstrap.serve` または `desktop_bootstrap.denied` として記録します。
</Note>

ゲートウェイはレスポンスの多くをマッチしたポリシーの `cli` ブロックとトップレベルゲートウェイ設定から導出します：

* モデルリスト。`availableModels` から
* 無効なツール。裸のツール名 `permissions.deny` エントリから。ポリシーの `desktop` ブロックで `disabledBuiltinTools` を設定する場合、ゲートウェイはあなたの値と導出されたリストの和集合を提供するため、この方法でより多くのツールを無効にできますが、`permissions.deny` を通じて無効にしたものを再度有効にすることはできません。
* エグレス許可リスト。`sandbox.network.allowedDomains` から。ポリシーの `desktop` ブロックで `coworkEgressAllowedHosts` を設定する場合、ゲートウェイは導出されたリストの代わりにその値を使用します。
* ゲートウェイ自体をポイントする OTLP エンドポイント。これは宛先にファンアウトします。[`telemetry`](#telemetry) フォワーディングが設定されている場合に含まれます。

  Claude Desktop はすべてのシグナルを 1 つのエンコーディングでエクスポートします：`http/protobuf`、またはポリシーの `env` で `OTEL_EXPORTER_OTLP_PROTOCOL` またはそのシグナルごとのバリアントを `http/json` に設定する場合は `http/json`。ゲートウェイサーバー上の Claude Code v2.1.261 より前では、レスポンスは関係なく `http/json` を設定したため、protobuf のみを受け入れるコレクターは Claude Desktop のエクスポートを拒否しました。

ポリシーの `desktop` ブロックで `disabledBuiltinTools`、`coworkEgressAllowedHosts`、または Claude Desktop 独自の `managedMcpServers` 設定を設定するには、ゲートウェイサーバー上で Claude Code v2.1.232 以降が必要です。Claude Desktop の `managedMcpServers` はオブジェクトではなく配列値を取ります。

ゲートウェイは Claude Desktop 相当がないキー（`hooks` やスコープ権限ルール（`Bash(npm *)` など））をブートストラップレスポンスから省略します。

`cli` の横にオプションの `desktop` ブロックを追加して、Claude Desktop 設定を直接設定します。Claude Desktop の [マネージド設定リファレンス](https://claude.com/docs/third-party/claude-desktop/configuration) からの設定を平坦なキー名として書きます。ゲートウェイが読み込むのみのキー（`bootstrapUrl` など）を省略します。MDM またはローカルファイルから。ゲートウェイはブート時にそれらを拒否します。v2.1.232 より前では、ゲートウェイは `chatTabEnabled` と `disableAutoUpdates` などの固定リストの 11 個の機能ゲートキーを受け入れ、他のすべてのキーをブート時に拒否しました。v2.1.227 より前では、ゲートウェイは `chatTabEnabled` と `chatAdvancedFileAnalysisEnabled` もブート時に拒否しました。

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

すべてのキーはオプションです。Claude Desktop は省略したキーに対して独自のデフォルトを適用します。ゲートウェイは各 `desktop` ブロックをブート時に Claude Desktop 自体が使用する設定スキーマに対して検証するため、間違いはゲートウェイ起動時にキーに名前を付けるエラーとして表示され、接続されたすべてのデスクトップに到達しません。ゲートウェイはブロックに以下が含まれる場合に失敗します：

* 不明なキー
* Claude Desktop が拒否または無言でドロップするであろう認識されたキー。空の値やネストされたエントリ内のスペルミスされたサブキーなど。v2.1.260 より前では、ゲートウェイは `managedMcpServers` または `orgPluginSettings` エントリのネストされたオブジェクト内のスペルミスされたフィールドを無言でドロップしました。ブート時に失敗する代わりに。
* ゲートウェイが自身で計算するキー：推論接続、モデルリスト、OTLP リレー。[`upstreams`](#upstreams)、[`models`](#models)、[`telemetry`](#telemetry) セクションの `forward_to` を通じてそれらを設定します。
* 現在のキーのレガシーエイリアス。ブートエラーで、ゲートウェイは書くべき正規キーに名前を付けます。

非推奨の値またはエントリ形状（`transport` なしの `managedMcpServers` エントリなど）を使用する場合、ゲートウェイは起動し、置き換えに名前を付ける警告をログします。

ゲートウェイは `desktop` ブロックを `cli` ブロックと同様にインストール済みバージョンにバンドルされたスキーマに対して検証します。新しい Claude Desktop リリースで導入された設定を配信するには、最初にゲートウェイをアップグレードします。例えば、`userPluginMarketplacesEnabled` と `userPluginUploadsEnabled` はゲートウェイサーバー上で Claude Code v2.1.260 以降と Claude Desktop 1.37937.0 以降が必要です。メンバーのマシン上で。

ポリシーの `desktop` ブロックで `orgPluginSettings` を設定する場合、ゲートウェイは Claude Desktop 1.15200.0 以降が読み込む配列形式で提供します。古いデスクトップは配列を無視し、プラグインツールポリシーを強制しないため、それに依存する前にメンバーを 1.15200.0 以降に更新します。

ゲートウェイはポリシーの `desktop` ブロックが設定しないキーを `match: {}` キャッチオールの `desktop` ブロックから埋めます。ポリシーの `cli` ブロックを基盤から埋めるのと同じ方法で。ベースとロールポリシーの両方で `disabledBuiltinTools` または `builtinToolPolicy` を設定する場合、ゲートウェイはベースの制限を保持します：

* `disabledBuiltinTools`：ゲートウェイはベースのリストとポリシーのリストの和集合を使用します。
* `builtinToolPolicy`：ベースでツールを `allow` 以外の値に設定する場合、ゲートウェイはロールポリシーで同じツールに対して `allow` を設定しても、その値を保持します。

他のすべてのキーについて、ロールポリシーで設定する場合、ゲートウェイはロールポリシーの値を使用します。ゲートウェイは配列またはネストされたオブジェクト（`banner` など）を全体で置き換えるため、ロールポリシーで `banner.text` を設定する場合、ゲートウェイはベースの `banner.backgroundColor` をドロップします。

Claude Desktop をデプロイしない場合、ポリシーから `desktop` を完全に省略します。ゲートウェイはその後、すべてのユーザーに対して `/user/bootstrap` から 404 を返します。

<h4 id="precedence-with-other-managed-sources">
  他のマネージドソースとの優先順位
</h4>

デバイスに MDM 配信ポリシーまたはローカル `managed-settings.json` もある場合、ゲートウェイ配信設定がランク付けされます。最初。[マネージド層内の優先順位](/docs/ja/managed-settings#precedence-within-the-managed-tier)はマネージド設定ページで、ローカルソースが適用される場合を説明し、[すべての管理ソースから読み込まれる Claude Code キー](/docs/ja/managed-settings#keys-read-from-every-admin-source)を持っています。サンドボックスロックキー、`forceRemoteSettingsRefresh`、変数ごとの `env` マージなど、どのソースを選択したかに関わらず。[`policyHelper`](/docs/ja/settings-reference#policyhelper) は MDM プロファイルまたはマネージド設定ファイルで設定され、ゲートウェイが設定を配信しない場合にのみ実行されます。エントリは出力が置き換えるものを説明します。

[Claude Desktop](/docs/ja/desktop) などの埋め込みホストは SDK `managedSettings` オプションを通じてポリシーを提供できます。[埋め込みホストからの親設定](/docs/ja/managed-settings#parent-settings-from-embedding-hosts)は Claude Code がそれを適用する場合を説明し、[親設定を制限](/docs/ja/claude-apps-gateway#restrict-parent-settings)は `allowManaged*Only` ロックなしでもまだ適用される許可方向設定をリストします。

ゲートウェイポリシーはマシン上のすべての Claude Code 呼び出しに適用されます。非インタラクティブ `claude -p` 実行と Agent SDK によって生成されたセッションを含みます。ゲートウェイがスタートアップ時に到達不可能な場合、署名されたセッションはポリシーなしで実行するのではなく、エラーで終了します。

<h3 id="telemetry">
  `telemetry`
</h3>

CLI は OpenTelemetry Protocol（OTLP）を HTTP メトリクス、ログ、有効な場合はトレースでゲートウェイに送信します。ゲートウェイはそれらを逐語的に各設定先にリレーします。エクスポートは OpenTelemetry Protocol（OTLP）を HTTP 経由で使用します。リレーをスキップして、セッションが直接コレクターにエクスポートするには、[ポリシーでコレクターに名前を付けます](#export-directly-to-your-collector)。[使用状況の監視](/docs/ja/monitoring-usage)で、CLI が発行するメトリクスとイベントを参照してください。

CLI は、ゲートウェイ発行 JWT から読み込まれた認証されたユーザーのアイデンティティで各エクスポートにスタンプを付けます：`user.id`、`user.email`、`user.groups` 属性。開発者ごとのコストと使用状況の属性は、開発者側の設定なしで機能します。

[Claude Desktop](#claude-desktop-overlay) と Cowork セッションがゲートウェイ経由でサインインすると、`user.email` と `user.groups` を `enduser.id` と一緒にテレメトリにスタンプを付けるため、1 つのクエリで `user.email` または `user.groups` でターミナル、Desktop、Cowork 使用状況をカバーできます。`user.groups` はコンマ区切りの IdP グループリストです。

Claude Code からのすべての OpenTelemetry データと同様に、これらの属性は組織が設定する宛先にのみ移動し、Anthropic には移動しません。

ユーザーのグループリストがパーセントエンコード後に 255 文字より長い場合、またはグループ名にコンマまたは等号が含まれている場合、ゲートウェイはそのユーザーの Desktop と Cowork テレメトリから `user.groups` を省略します。そのユーザーのターミナルセッションは完全なリストを引き続き実行します。

ゲートウェイサーバー上で Claude Code v2.1.265 以降が必要で、Desktop と Cowork テレメトリで `user.email` と `user.groups` が必要です。各開発者のマシン上で Claude Desktop 1.24012 以降が `user.groups` に必要です。

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # シグナルごとのオプトイン。デフォルト：メトリクスのみ。
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  各宛先は `metrics`、`logs`、`traces` に独立してオプトインし、デフォルトはメトリクスのみです。シグナルは感度が異なります：

  * **メトリクス**：トークンカウント、リクエストカウント、レイテンシなどの集計カウンター
  * **ログとトレース**：完全な bash コマンド、ツール入力、ファイルパスを含むことができます。Claude Code が開発者のマシンで行うすべてをカバーします。

  ログとトレースは、アクセス制御と保持ポリシーがデータを保証する宛先でのみ有効にします。
</Warning>

各 `forward_to` URL は `https://` を使用する必要があります。ゲートウェイ独自のループバックインターフェース上のコレクターの場合は 1 つの例外：

* `http://localhost:<port>` は設定検証を通過しますが、[SSRF ガード](/docs/ja/claude-apps-gateway-deploy#threat-model-summary)は `ECONNREFUSED_SSRF` ですべてのエクスポートをブロックします。`CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` をゲートウェイの環境に設定しない限り。
* `http://127.0.0.1:<port>` または `http://[::1]:<port>` はその変数が設定されていない限りブートに失敗します。

クラスター内コレクターの場合、HTTPS で独自の内部アドレスで公開するか、変数が設定されたサイドカーとして実行します。

テレメトリは CLI でデフォルトでオフです。`telemetry.forward_to` と `listen.public_url` の両方を設定すると、ゲートウェイはそれをオンにします。接続されたクライアント用に `/managed/settings` を通じて 6 つの環境変数をプッシュします：

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`、`OTEL_LOGS_EXPORTER`、`OTEL_TRACES_EXPORTER`。少なくとも 1 つの `forward_to` 宛先がそのシグナルを有効にする場合は `otlp` に設定され、そうでない場合は `none` に設定されます。
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

ゲートウェイサーバー上の Claude Code v2.1.265 より前では、ゲートウェイはすべての 3 つのエクスポーターセレクターを `otlp` としてプッシュしました。宛先がオプトインしなかったシグナルを含みます。

プッシュされたエンドポイントはパブリック URL から構築されるため、メトリクスとログは開発者またはポリシーからの OTEL 設定を必要としません。

`/login` を通じてサインインした開発者は、独自の OTEL 設定でエクスポートをリダイレクトできません：

* **ローカルに設定された変数**：Claude Code はプッシュされた変数をマネージド層で適用するため、各変数はローカルで設定する値をオーバーライドします。
* **ローカルに設定されたエンドポイント**：OTLP/HTTP エクスポート有効にすると、CLI はローカルに設定されたエンドポイントを無視します。ゲートウェイがプッシュしたテレメトリ変数があるかどうかに関わらず。エクスポートはゲートウェイに移動します。ポリシーが [コレクターをエンドポイントとして名前を付けない](#export-directly-to-your-collector)限り。

`forward_to` 宛先がシグナルにない場合、ゲートウェイはそれを受け入れて破棄します。開発者が既に Claude Code テレメトリを 1 つのコレクターにエクスポートしている場合、それを `forward_to` 宛先として追加します。ログまたはトレースをエクスポートする場合は、それらを有効にして、サインイン後もデータを受け取り続けるようにします。リレーをスキップするには、代わりに [ポリシーでコレクターに名前を付けます](#export-directly-to-your-collector)。

[トレース](/docs/ja/monitoring-usage#traces-beta)はさらに各クライアントで `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` を必要とします。ゲートウェイはその変数をプッシュしないため、マネージドポリシーの `env` ブロックを通じて設定します。開発者はそれを [セキュリティ承認ダイアログ](#managed)で承認します。プッシュされたエンドポイントがすでてトリガーするのと同じダイアログです。

それを `1` に設定するのは、トレースしたいグループのポリシーのみです。ポリシーが設定しない場合、`match: {}` キャッチオールポリシーが設定する場合、その値を継承します。[マージルール](#managed)に従って。グループのクライアントが開発者がローカルで変数を設定しても、トレースを送信しないようにするには、そのグループのポリシーで `0` に設定します。

protobuf と JSON OTLP エンコーディングの両方がリレーされ、OpenTelemetry 互換バックエンドは宛先として機能します。

<h4 id="export-directly-to-your-collector">
  コレクターに直接エクスポートする
</h4>

`/login` を通じてサインインしたセッションがリレーを通じてではなく、コレクターに直接テレメトリを送信するには、[マネージドポリシー](#managed)の `env` ブロックでコレクターの `https://` ベース URL に `OTEL_EXPORTER_OTLP_ENDPOINT` を設定します。Claude Code は `/v1/metrics`、`/v1/logs`、`/v1/traces` を URL に追加します。例えば `https://otel-collector.example.com:4318`。各シグナルをそこに OTLP/HTTP 経由でエクスポートします。各開発者のマシン上で Claude Code v2.1.265 以降が必要です。以前のクライアントはリレーを通じてエクスポートします。

コレクターに認証するには、同じ `env` ブロックで `OTEL_EXPORTER_OTLP_HEADERS` を設定します。セッションはこの方法で名前を付けられたコレクターに開発者のゲートウェイセッショントークンを送信しません。

ポリシーでこのエンドポイントを追加または変更すると、Claude Code は [セキュリティ承認ダイアログ](#managed)でそれを適用する前に各開発者に承認を求めます。

Claude Code はシグナルを直接エクスポートする前にエンドポイントをチェックし、チェックが失敗するとそのシグナルをリレーに保持します。チェックには以下が含まれます：

* エンドポイントはゲートウェイ自体から来ます。MDM プロファイルまたはローカル `managed-settings.json` で同じ変数を設定する場合、エクスポートはリレーに留まります。
* URL は `https://` を使用するか、ループバックアドレスに `http://` を使用します。
* URL は `/v1/<signal>` で終わるパスに解決され、クエリまたはフラグメントはありません。Claude Code はジェネリック変数からそのパスを自身で構築します。`OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` などのシグナルごとの変数を使用する場合は、完全なパスをそこに含めます。
* URL はゲートウェイ独自のホストではありません。ゲートウェイに対処されたエンドポイントはリレーパスとセッショントークンを保持します。
* あなたも開発者も [`otelHeadersHelper`](/docs/ja/settings-reference#otelheadershelper) を設定していません。任意の設定ソースで。ヘルパーが設定されている場合、すべてのシグナルはリレーに留まります。

名前を付けるエンドポイントはエクスポートがどこに移動するかのみを変更します。どのシグナルがエクスポートするかは、`OTEL_*_EXPORTER` セレクターで選択します。

エンドポイント単独ではエクスポートをオンにしないため、ゲートウェイが既にプッシュしていない限り、それをオンにする変数も設定します：

* ゲートウェイが既に [テレメトリ変数をプッシュ](#telemetry)する場合、それらは有効化、セレクター、プロトコルをカバーし、プッシュされた `<public_url>` 値をオーバーライドします。`forward_to` 宛先が有効にしないシグナルについてのみ、`OTEL_*_EXPORTER` セレクターを `otlp` に自身で設定します。
* そうでない場合、`CLAUDE_CODE_ENABLE_TELEMETRY=1`、`OTEL_*_EXPORTER` セレクター、`OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf` も設定します。

開発者がサインアウトするか、別のゲートウェイにサインインすると、コレクターへのエクスポートは停止し、Claude Code は各残りのバッチをドロップします。

<h4 id="when-a-destination-fails">
  宛先が失敗する場合
</h4>

ゲートウェイはバッファリング、再試行、またはテレメトリを保存しないため、宛先に到達しないエクスポートは遅く配信するのではなくドロップされます。各宛先は独立して成功または失敗し、エクスポートクライアントはどちらの方法でも成功レスポンスを受け取るため、失敗した配信はゲートウェイのログにのみ表示されます。

5 つの連続した失敗した配信の後、ゲートウェイは 30 秒のストレッチで宛先へのフォワーディングを一時停止し、各一時停止をログします。配信が成功するまで。エラーレスポンス、タイムアウト、接続エラーはすべて失敗した配信としてカウントされます。`400`、`413`、`415`、`422`、`431` を除き、コレクターがそのエクスポートのペイロードを形式が正しくないか大きすぎるとして拒否したことを意味します。

拒否されたペイロードは失敗カウントを進めたり、リセットしたりしません：ゲートウェイは宛先へのフォワーディングを続け、最初の拒否と 100 番目ごとに、宛先に名前を付けるステータスを警告します。

<h3 id="http-tuning">
  HTTP チューニング
</h3>

4 つのオプションのトップレベルブロック、`access_control`、`limits`、`timeouts`、`rate_limits`。HTTP サーフェスをチューニングします。デフォルトはほとんどのデプロイメントに適しています。

| ブロック             | キー                                             | デフォルト    | 説明                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------- | ---------------------------------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | 空        | `trusted_proxies` 解決後のクライアントアドレスによるインバウンド IP 許可/拒否。`deny_cidrs` が最初にチェックされます。クライアントがマッチする場合、`allow_cidrs` もマッチしても拒否されます。`allow_cidrs` が空でない場合、ゲートウェイはデフォルト拒否です。`/healthz` と `/readyz` は `allow_cidrs` から除外されます。信頼できるプロキシが `X-Forwarded-For` エントリを送信し、それが IP アドレスではない場合、実際のクライアントは不明で、ゲートウェイは何をチェックするかに名前を付ける警告を 1 回ログします。どちらかのリストがリクエストに適用される場合、それはリクエストを拒否し、`403` と監査理由 `xff_unparseable` を返します。どちらでもない場合、リクエストを提供し、プロキシ独自のアドレスを IP ごとのレート制限と監査のクライアント IP として使用します。 |
| `limits`         | `max_request_bytes`                            | 32 MiB   | 最大インバウンドリクエストボディ。サイズを超えるリクエストはボディがバッファリングされる前に `413` を取得します。大きなファイルまたは画像リクエストの場合は増やします。                                                                                                                                                                                                                                                                                                                                                                              |
| `limits`         | `max_request_header_bytes`                     | 未設定      | 設定すると、サイズを超えるヘッダーは `431` を返します。                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `limits`         | `max_url_length`                               | 未設定      | 設定すると、長すぎる URL は `414` を返します。                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000   | アップストリームのレスポンスヘッダー（初バイト時間）を待つ最大時間。レスポンスボディはその後、ウォールクロックキャップなしでストリーミングされます。直接 Anthropic アップストリームパスに適用されます。他のすべてのプロバイダーはプロバイダー SDK 独自のタイムアウトで制限されます。                                                                                                                                                                                                                                                                                                                   |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600 | 認証されていないデバイス認可エンドポイントの IP ごとのレート制限。共有エグレス IP または NAT の背後にある大規模な組織の場合は増やします。これらの制限は、デバイスグラント サインインフローにのみ適用され、`/v1/messages` 推論には適用されません。[ユーザーコードブルートフォース耐性](/docs/ja/claude-apps-gateway-deploy#user-code-brute-force-resistance)を参照してください。                                                                                                                                                                                                                                |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600 | `/device` での `user_code` 送信の IP ごとのレート制限。                                                                                                                                                                                                                                                                                                                                                                                                                            |

`access_control` リストを両方とも空のままにする場合、これはデフォルトで、ゲートウェイはすべてのクライアントアドレスに提供するため、ネットワークのみがそれに到達できるユーザーを制限します。これは重要です。ゲートウェイは [マネージド設定](#managed)をプッシュできるため、開発者マシンでコマンドを実行します。

`allow_cidrs` が空の間、ゲートウェイは 2 つの場所で警告します。リクエストへの回答方法を変更することなく：

* **ブート時**：運用ログの警告は、プライベート範囲 `10.0.0.0/8`、`172.16.0.0/12`、`192.168.0.0/16`、`100.64.0.0/10`、`127.0.0.0/8`、`::1/128`、`fc00::/7` のみを許可することを推奨します。開発者が接続する他の内部範囲を加えます。ゲートウェイをループバックアドレスにバインドし、`trusted_proxies` も `public_url` も設定しない場合、ローカル開発のように、警告は表示されません。
* **実行時**：リクエストが最初にこれらのプライベート範囲外のアドレスから到着すると、ゲートウェイは警告をログし、[`access.public_client` 監査イベント](/docs/ja/claude-apps-gateway-deploy#logs)をクライアント IP で発行します。両方とも 1 回プロセスごとに発火します。リンクローカルアドレス、`169.254.0.0/16` と `fe80::/10` は、パブリックとしてカウントされません。ゲートウェイは `/healthz` と `/readyz` をこのチェック実行前に回答するため、パブリック範囲からのヘルスプローブはそれをトリガーしません。

両方のシグナルはゲートウェイが解決するクライアントアドレスを使用します。ロードバランサー、ポートフォワード、またはトンネルがトラフィックをリレーし、`listen.trusted_proxies` にリストされていない場合、ゲートウェイはリレーのアドレスを見ます。通常はプライベートです。したがって、実行時警告もプライベート許可リストもそれをキャッチしません。

そのようなフロントエンドの背後で、[`listen.trusted_proxies`](#listen)を最初に設定して、ゲートウェイが実際のクライアントアドレスを見るようにし、ゲートウェイとその前のすべてをパブリックインターネットから到達不可能に保ちます。

<h2 id="complete-example">
  完全な例
</h2>

この完全なリファレンス設定は、すべてのコアセクションを実行します。[HTTP チューニングブロック](#http-tuning)はデフォルト値を保持します。これをコピーして、不要な部分を削除し、値を入力してください。[クイックスタート](/docs/ja/claude-apps-gateway#quickstart)の設定は、これの最小限のバージョンです。

```yaml gateway.yaml theme={null}
# Run with:
#   claude gateway --config gateway.yaml
#
# Operational log verbosity is controlled by the CLAUDE_GATEWAY_LOG_LEVEL
# environment variable (debug | info | warn | error; default info). debug
# also logs the claim names in each id_token, for groups_claim diagnosis.
# It does not affect audit events, which are always emitted.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Omit the tls block when running behind a TLS-terminating ingress.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Required when the issuer is the Okta org server, whose id_tokens
  # can omit email and groups; the gateway fills them from /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta emits groups only when the `groups` scope is requested and the
  # app's groups claim filter allows them. The contractors policy below
  # matches on groups, so the scope is requested here.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra app roles: use `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# Enables /v1/organizations/spend_limits (mirrors the Anthropic Admin API)
# and per-developer spend enforcement on /v1/messages. Omit to disable.
# Caps themselves are set via the admin API, not here.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

# Meter at contracted rates instead of USD list price. Requires admin: or a
# managed: policy. With managed:, the same rates also go to signed-in clients.
# Rates below are placeholders, not real contract prices.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Constrain the Default picker option to availableModels instead of
        # the tier default, so contractors don't get a 400 on the default.
        enforceAvailableModels: true
        # allow auto-approves these tools; it does not block the rest.
        # Add deny rules to restrict tools.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  クライアント側で管理される設定
</h2>

上記のすべてはゲートウェイサーバーを設定します。開発者マシンはゲートウェイに別途ポイントし、各デバイスで Claude Code の[管理設定](/docs/ja/managed-settings)を通じて設定します。ゲートウェイはログインキー自体をプッシュできません。これらのキーがクライアントにゲートウェイの場所を伝えるためです。

CLI の場合、OS ごとの `managed-settings.json` でこれらのキーを設定します。2 つのログインキーは各開発者の `/login` をゲートウェイにルーティングします。

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` は Claude Desktop の出力許可リストの配信を埋め込み Claude Code セッションで機能させ続けます。[Claude Desktop セッションにポリシーを配信する](/docs/ja/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions)はメカニズムと opt-in が配置される場所を説明しています。

`managed-settings.json` ファイルを各デバイスにデプロイします。通常は MDM プラットフォーム経由です。ファイルパスはプラットフォームによって異なります。[各メカニズムがポリシーを保存する場所](/docs/ja/managed-settings#where-each-mechanism-stores-the-policy)を参照してください。

デフォルトでは、Windows のレジストリポリシーまたは macOS のマネージドプリファレンス plist は、[上記の例外キーとクロスソースチェック](#precedence-with-other-managed-sources)を除き、`managed-settings.json` ファイルとマージするのではなく置き換えます。このスニペットの 3 つのキーはすべて最優先ソースルールに従うため、Group Policy または設定プロファイルを通じてポリシーを配信するフリートは、代わりにそのメカニズムにすべての 3 つを配置する必要があります。

Claude Desktop の場合、Claude Desktop 独自の[マネージド設定](https://claude.com/docs/third-party/claude-desktop/configuration)で `bootstrapUrl` キーを `<listen.public_url>/user/bootstrap` に設定します。サインインフロー及びグループごとのポリシーは、ポリシーが `desktop` キーでサーバー側で opt-in した後、CLI のものと一致します。opt-in がない場合、`/user/bootstrap` は 404 を返します。[Claude Desktop オーバーレイ](#claude-desktop-overlay)でサーバー側の半分を参照してください。

Claude Code は [`forceLoginGatewayUrl`](/docs/ja/settings-reference#forcelogingatewayurl)、[`gatewayInternalNetworks`](/docs/ja/settings-reference#gatewayinternalnetworks)、および [`forceLoginMethod`](/docs/ja/settings-reference#forceloginmethod) の `"gateway"` 値をマシン上のマネージドソースからのみ認識します。`managed-settings.json`、macOS plist または Windows HKLM レジストリ、またはポリシーヘルパーです。開発者が独自の `~/.claude/settings.json` でこれらを設定しても効果がなく、ゲートウェイペイロードで設定しても同様です。

<h2 id="related">
  関連
</h2>

* [Claude apps gateway 概要](/docs/ja/claude-apps-gateway)：クイックスタートと開発者接続
* [デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy)：IdP セットアップ、コンテナイメージ、Kubernetes と Cloud Run、運用
* [支出制限](/docs/ja/claude-apps-gateway-spend-limits)：開発者ごとのキャップと Admin API
