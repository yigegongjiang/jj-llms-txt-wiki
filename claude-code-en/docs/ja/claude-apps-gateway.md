> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock、Claude Platform on AWS、Google Cloud、Microsoft Foundry 向け Claude アプリゲートウェイ

> SSO サインイン、グループごとのモデルアクセス、OTLP テレメトリを備えた自己ホスト型ゲートウェイを通じて、Amazon Bedrock、Claude Platform on AWS、Google Cloud、または Microsoft Foundry で Claude Code を実行します。

<Note>
  Claude アプリゲートウェイは、[データレジデンシー](/docs/ja/claude-apps-gateway-deploy#compliance-posture)要件を満たすなど、独自のクラウドプロバイダーを通じて推論をルーティングする必要がある、または希望する組織向けに設計されています。この要件がない場合、SCIM プロビジョニングや web・モバイル上の Claude Code などの他の機能へのアクセスを希望する場合は、Claude Enterprise の方がより適切である可能性があります。すべてのデプロイメント方法の完全な比較については、[機能可用性](/docs/ja/feature-availability)ページを参照してください。
</Note>

Claude アプリゲートウェイは、開発者の Claude Code クライアントとモデルプロバイダーの間に位置する自己ホスト型サービスです。開発者は API キーやクラウド認証情報を保持する代わりに、企業の ID プロバイダー（IdP）でサインインします。ゲートウェイはアップストリーム認証情報を保持し、IdP グループによるモデルアクセスと[管理設定](/docs/ja/managed-settings)を強制し、使用状況テレメトリを独自の可観測性スタックにリレーします。

これは `claude` バイナリに含まれているため、ラップトップで Claude Code を実行する同じ実行ファイルが `claude gateway --config gateway.yaml` でゲートウェイサーバーを実行します。

このページでは以下をカバーしています。

* [Claude アプリゲートウェイを使用する理由](#why-claude-apps-gateway)、独自に実行する場合に何が追加されるか、および他の何かがより適切な場合
* [前提条件](#prerequisites)を含む[クイックスタート](#quickstart)。ゲートウェイをゼロからサインイン済みの開発者まで進めます
* [開発者の接続](#connect-developers)。管理設定を通じてゲートウェイ URL を設定することを含みます
* [可用性と制限事項](#availability-and-limitations)。ゲートウェイを通じてどの Claude Code 機能が機能するか、およびサーバーが何をサポートするかをカバーしています

関連ページはさらに詳しく説明しています。[設定リファレンス](/docs/ja/claude-apps-gateway-config)はクイックスタートが書き込む YAML ファイルのすべてのオプションをカバーし、[デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy)は IdP ごとのセットアップ、Kubernetes と Cloud Run デプロイメント、および運用をカバーしています。

<h2 id="why-claude-apps-gateway">
  Claude apps gateway を使用する理由
</h2>

[ゲートウェイの概要](/docs/ja/gateways)はゲートウェイが何をするか、なぜ実行するかをカバーしています。Claude apps gateway は Anthropic 独自のゲートウェイで、`claude` バイナリに組み込まれており、各 Claude Code リリースと一緒にテストされているため、Claude Code が送信するヘッダーとリクエストフィールドを、オペレーターが個別の許可リストを維持することなく転送します。デプロイされると、以下が得られます。

* **認証情報**：アップストリーム API キーまたはクラウド認証情報は、インフラストラクチャ内にのみ存在します。開発者は企業 SSO で認証し、短期間有効なベアラートークンを受け取るため、オフボーディングは IdP で発生します。ユーザーをプロビジョニング解除すると、ゲートウェイアクセスはセッション有効期間内に期限切れになります。デフォルトは 1 時間です。
* **アクセス制御**：IdP グループはモデル許可リストと[管理設定](/docs/ja/managed-settings)ポリシーにマップされます。ゲートウェイはモデルアクセスをサーバー側で強制し、許可されていないモデルのリクエストを拒否し、各グループの管理設定ポリシーを選択します。CLI は[管理設定層](/docs/ja/settings#settings-precedence)でこれを適用します。異なるチームは異なるモデル、ツール、および権限を取得し、開発者はポリシーがロックしているものをオーバーライドできません。
* **設定配信**：ゲートウェイは管理設定をサインイン済みクライアント自体に配信し、claude.ai 管理コンソールからの[サーバー管理設定](/docs/ja/server-managed-settings)の場所を取ります。
* **テレメトリ**：各設定先は、デフォルトではトークン数、モデル、ユーザーアイデンティティ、レイテンシを含む[OpenTelemetry Protocol（OTLP）メトリクス](/docs/ja/monitoring-usage)を受け取り、ログとトレースは宛先ごとのオプトインです。
* **アップストリームルーティング**：クライアントは Anthropic Messages API をゲートウェイに話しかけ、ゲートウェイは各アップストリーム（Amazon Bedrock、[Claude Platform on AWS](/docs/ja/claude-platform-on-aws)、Google Cloud の Agent Platform、Microsoft Foundry、または Anthropic API）に対して変換し、それらの間でフェイルオーバーします。開発者が気付いたり再設定したりすることなく、リージョン、プロバイダー、またはフェイルオーバー順序を変更できます。

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Claude Code クライアントと Claude Desktop の Chat、Cowork、Code タブがベアラートークンを使用して HTTPS 経由でインフラストラクチャ内の自己ホスト型 Claude apps ゲートウェイに接続し、IdP に対してユーザーにサインインし、PostgreSQL に認証状態を保存し、テレメトリを OTLP コレクターにリレーし、Amazon Bedrock、Claude Platform on AWS、Google Cloud、Microsoft Foundry、または Anthropic API に推論を転送する図" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  ゲートウェイ独自のデータプレーンは、Anthropic API が設定されたアップストリームでない限り、Anthropic インフラストラクチャに何も送信しません。テレメトリ、監査ログ、管理設定、および開発者の IdP アイデンティティがどこに行くかを制御し、ゲートウェイはそれらのいずれも Anthropic に送信しません。残りのトラフィック CLI プロセスが送信できる方法と、それを閉じる方法については、[コンプライアンスポスチャ](/docs/ja/claude-apps-gateway-deploy#compliance-posture)を参照してください。
</Note>

ゲートウェイを通じてどの Claude Code 機能が機能するか、およびサーバー自体が何をサポートするかについては、以下の[可用性と制限](#availability-and-limitations)を参照してください。コスト、バイパス、複数ゲートウェイの実行、サーバーレスプラットフォームなどの決定については、[デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy#deployment)を参照してください。

<h3 id="other-gateway-implementations">
  その他のゲートウェイ実装
</h3>

既に要件を満たす LLM ゲートウェイまたは API ゲートウェイを実行している場合は、それを使い続けてください。[その他の LLM ゲートウェイ](/docs/ja/llm-gateway)は Claude Code をそれに対して設定することをカバーしています。

[ゲートウェイ互換性ガイド](/docs/ja/llm-gateway-protocol)は、Claude Code が任意のゲートウェイから期待するもの、つまり呼び出すエンドポイント、転送するヘッダーとボディフィールド、およびそれらが削除されたときに何が機能しなくなるかを文書化しています。実行中の Claude apps ゲートウェイは、`GET /protocol` で独自のプロトコルリファレンスを提供し、Claude Code クライアントに公開するエンドポイント（SSO サインイン、推論、管理設定配信、モデル検出、テレメトリ）を説明しています。デプロイされたゲートウェイ（例えば、以下の[クイックスタート](#quickstart)が生成するもの）から `curl https://claude-gateway.internal.example.com/protocol` で取得します。

プロトコルへの破壊的な変更は事前に発表されますが、無期限の後方互換性は保証されません。

<h2 id="quickstart">
  クイックスタート
</h2>

このクイックスタートは最小限のパスを説明しています。IdP で OAuth クライアントを登録し、`gateway.yaml` を書き、Docker Compose で Postgres と一緒にゲートウェイを実行し、エンドツーエンドでサインインを確認します。Amazon Bedrock アップストリームを使用します。Claude Platform on AWS、Google Cloud の Agent Platform、Microsoft Foundry、および Anthropic API は、[設定リファレンス](/docs/ja/claude-apps-gateway-config#upstreams)に示されているように `upstreams` ブロックをスワップすることで同様にサポートされます。最後に、開発者が `/login` できるゲートウェイがあります。

<Note>
  **プライベートネットワークにデプロイします。** Claude Code は、アドレスがプライベートであるゲートウェイにのみ接続します。これはセキュリティガードです。信頼されたゲートウェイは開発者マシンでコマンドを実行する設定をプッシュできるためです。ゲートウェイを内部ロードバランサーまたは VPN の背後に配置し、プライベート IP にのみ解決するホスト名を付けます。内部ネットワークが組織が所有するパブリック IPv4 スペースから番号付けされている場合は、[所有するパブリックアドレススペースでゲートウェイを許可する](#allow-a-gateway-on-public-address-space-you-own)を参照してください。
</Note>

<h3 id="prerequisites">
  前提条件
</h3>

開始する前に、以下を用意してください。

| 必要なもの                         | 詳細                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Claude Code v2.1.195 以降       | `claude gateway` サブコマンドとゲートウェイサインインフローは v2.1.195 で提供されます。以前のパブリックビルドには含まれていません。ゲートウェイサーバーを実行するマシンと各開発者のマシンの両方が v2.1.195 以降である必要があります。`claude update` を実行して最新リリースを取得します。[Claude Platform on AWS アップストリーム](/docs/ja/claude-apps-gateway-config#claude-platform-on-aws)はゲートウェイサーバーで Claude Code v2.1.198 以降が必要です。                                                                                                                                                                                                                                                    |
| OpenID Connect（OIDC）ID プロバイダー | Okta、Microsoft Entra ID、Google Workspace、Keycloak、Dex、または PingFederate などの OIDC 準拠の IdP。ゲートウェイは標準 OIDC ディスカバリーと認可コードフローを実行します。SAML と LDAP はサポートされていません。                                                                                                                                                                                                                                                                                                                                                                                                      |
| PostgreSQL 14 以降              | デバイスサインインフロー（ブラウザコールバックが書き込み、ポーリング CLI が読み取る）とレート制限カウンターをサポートします。最小層を含む任意の管理 Postgres が機能します。支出制限が設定されていない場合、ゲートウェイは数 KB の短期間有効な認証状態を保存します。[支出制限](/docs/ja/claude-apps-gateway-spend-limits)を使用すると、バックアップする必要がある耐久的な支出、監査、およびアイデンティティテーブルも保持します。`?sslmode=require` 経由の TLS が推奨されます。                                                                                                                                                                                                                                                                               |
| モデルアップストリーム                   | Amazon Bedrock 認証情報、Claude Platform on AWS 認証情報、Google Cloud 認証情報、Microsoft Foundry リソース、または Anthropic API キー。複数のアップストリームがサポートされ、フェイルオーバーがあります。                                                                                                                                                                                                                                                                                                                                                                                                              |
| HTTPS                         | ゲートウェイは開発者ラップトップとサインインに使用されるブラウザから `https://` 経由で到達可能である必要があります。ゲートウェイは同じリスナーでデバイス検証ページを提供します。`listen.tls` 経由で TLS 証明書を提供するか、TLS 終了イングレスの背後で実行し、`listen.public_url` を外部オリジンに設定します。プレーン `http://` オリジンはゲートウェイホストがループバック（`localhost`、`127.0.0.1`、または `::1`）の場合にのみ受け入れられます。                                                                                                                                                                                                                                                                                     |
| プライベートネットワークアドレス              | `/login` では、Claude Code はゲートウェイのホスト名または IP アドレスがプライベートアドレスのみに解決されることを要求します。RFC 1918、リンクローカル、CGNAT `100.64.0.0/10`、IPv6 ULA `fc00::/7`、またはループバック。ホストするゲートウェイの場合、宣言するブロック外のパブリックアドレスは拒否されます。デプロイメントガイドの[脅威モデル](/docs/ja/claude-apps-gateway-deploy#threat-model-summary)を参照してください。開発者マシンが HTTPS を企業プロキシ経由でルーティングする場合、サインインはプロキシホストもプライベートアドレスに解決されることを要求します。そうでない場合は、ゲートウェイホストを `NO_PROXY` に追加して、CLI が直接接続するようにします。内部ネットワークが組織が所有するパブリック IPv4 スペースから番号付けされている場合は、[これらのブロックを宣言](#allow-a-gateway-on-public-address-space-you-own)して、`/login` がそこでゲートウェイを受け入れるようにします。 |
| Linux ランタイム                   | ゲートウェイサーバーはネイティブ Linux バイナリでのみ実行されます。macOS はローカル開発用に機能します。Windows はサーバープラットフォームとしてサポートされていません。                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h3 id="steps">
  ステップ
</h3>

<Steps>
  <Step title="IdP で OAuth クライアントを登録する">
    リダイレクト URI がそれと一致する必要があるため、まずゲートウェイのホスト名を決定します。新しい OIDC ウェブアプリケーションを作成し、リダイレクト URI を `https://claude-gateway.<your-domain>/oauth/callback` に設定します。ホストはステップ 3 で [`listen.public_url`](/docs/ja/claude-apps-gateway-config#listen) として設定する値と同じです。`client_id` と `client_secret` をメモします。IdP ごとの手順は[ID プロバイダーセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)にあります。
  </Step>

  <Step title="PostgreSQL データベースをプロビジョニングする">
    最小管理層を含む任意の Postgres 14 以降が機能します。ゲートウェイは起動時に独自のスキーママイグレーションを実行するため、データベースロールはテーブルを作成および変更する権限が必要です。[`store`](/docs/ja/claude-apps-gateway-config#store)を参照してください。
  </Step>

  <Step title="gateway.yaml を書く">
    シークレットは `${ENV_VAR}` 展開経由で読み取られるため、ファイル自体はバージョン管理に存在できます。`/login` がパブリックアドレスを拒否するため、プライベート IP に解決する `public_url` ホスト名を使用します。最小設定には 5 つのセクションがあり、他のすべてのフィールドにはデフォルトがあります。

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # ホストがループバックアドレスでない限り必須。IdP
      # redirect_uri とディスカバリードキュメントに使用されます。
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # /.well-known/openid-configuration を提供する必要があります
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # 組織外の id_tokens を拒否します
      userinfo_fallback: true                  # id_token が email/groups を省略する IdP の場合。それ以外の場合は無害です

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # IdP プロビジョニング解除時の失効レイテンシもバウンドします

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # 管理 Postgres の場合は ?sslmode=require を追加します

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # 空：AWS デフォルト認証情報チェーン
    # （IRSA、EC2/ECS タスクロール、環境変数、~/.aws）

    # モデルはアップストリームごとに自動的に変換されます。組み込みカタログ
    # は claude-opus-4-8 を us.anthropic.claude-opus-4-8 にマップし、
    # Bedrock がサポートするすべての Claude モデルに対して同様にマップします。false に設定し、
    # `models:` リストを追加して、特定のモデルのみを公開します。
    auto_include_builtin_models: true
    ```

    この設定は、デフォルト Amazon Bedrock モデルカタログを使用した動作するサインインループに十分です。実行されたら、[`managed.policies`](/docs/ja/claude-apps-gateway-config#managed) 経由でグループごとの RBAC と管理設定を追加し、[`telemetry`](/docs/ja/claude-apps-gateway-config#telemetry) 経由でテレメトリファンアウトを追加し、[`models`](/docs/ja/claude-apps-gateway-config#models) 経由でマルチアップストリームフェイルオーバー、プロビジョニング済みスループット ARN、または非米国リージョンを追加します。

    <Note>
      Amazon Bedrock アップストリームは、`inference-profile/us.anthropic.*` ARN と基礎となる `foundation-model/anthropic.*` ARN の両方に対して `bedrock:InvokeModel` と `bedrock:InvokeModelWithResponseStream` を持つ AWS プリンシパルが必要です。また、そのアカウントについて、Bedrock コンソールのモデルカタログから Anthropic の一度限りのユースケースフォームが提出されている必要もあります。

      静的キーではなく、EKS の IRSA、ECS タスクロール、または EC2 インスタンスプロファイルを使用して認証情報を提供します。[`upstreams` リファレンス](/docs/ja/claude-apps-gateway-config#upstreams)には、完全な IAM 詳細、クロスクラウド認証情報マトリックス、および他のプロバイダーの `auth` ブロックがあります。
    </Note>
  </Step>

  <Step title="実行する">
    [イメージ要件](/docs/ja/claude-apps-gateway-deploy#container-image)を満たす `claude` バイナリの周りにコンテナイメージを構築し、Postgres と一緒に実行します。Compose ファイルはイメージを `registry.example.com/claude-gateway:2.1.198` として参照します。独自のレジストリとイメージタグに置き換えます。

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # AWS 認証情報：本番環境では、これらを省略し、インスタンス
          # ロールを使用します。ローカル Compose テストの場合、独自のものを渡します。
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    ゲートウェイは、設定を読み取り、Postgres に接続してスキーママイグレーションを適用し、IdP に対して OIDC ディスカバリーを実行し、アップストリームクライアントを構築し、リッスンを開始する単一の Linux バイナリです。起動は設定、Postgres 接続、OIDC ディスカバリー、およびアップストリームクライアント構築に対して失敗時に閉じられます。これらのいずれかが到達不可能または設定が誤っている場合、ゲートウェイは低下した状態でトラフィックを提供するのではなく、エラーで終了します。

    成功した起動は推論パスを検証しません。Amazon Bedrock と Google Cloud の Agent Platform インスタンス認証情報は起動時ではなく最初のリクエストで解決されるためです。

    起動シーケンスについて stderr を監視します。ログ行は `[gateway] <timestamp> <level> <message>` 形式を使用し、監査イベントは `evt` フィールド付きの単一行 JSON であり、起動バナーは以下で省略され、マイグレーションとリッスン行の間に出力されます。新しいデータベースはスキーママイグレーションごとに 1 行の `migration N applied` を出力します。既にマイグレーションされたデータベースは出力しません。順番に以下が表示されます。

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    ゲートウェイは、`access_control.allow_cidrs` が空であることを示す警告もログに記録します。これはここで予想されています。ゲートウェイがサーブするクライアントアドレスを制限するものがないためです。許可リストを設定するまで。[`access_control` リファレンス](/docs/ja/claude-apps-gateway-config#http-tuning)には推奨範囲があります。

    起動が `claude gateway listening on` 行の前に終了する場合、stderr の最後の行は問題を名前付けます。

    * 到達不可能な Postgres
    * DDL 権限のない Postgres ロール
    * 到達不可能または無効な OIDC ディスカバリードキュメント
    * 違反フィールドパスを含む設定スキーマ違反

    修正して再起動します。

    既に TLS 終了イングレスがある場合は、Compose をスキップし、`claude gateway --config gateway.yaml` でバイナリを直接実行します。`public_url` をイングレスオリジンに設定し、`listen` をループバックまたはクラスター内アドレスにバインドします。
  </Step>

  <Step title="認証サーフェスを確認する">
    3 つのチェックは、ゲートウェイが開発者に渡す前に実際のユーザーを認証できることを確認します。

    例はゲートウェイのパブリック URL を使用します。イングレスのないローカル Compose セットアップの場合、最初の 2 つのチェックで `http://localhost:8080` に置き換えます。3 番目のチェックは `verification_uri_complete` を開きます。これは `public_url` から構築されるため、ローカル Compose の場合は `gateway.yaml` で `public_url: http://localhost:8080` を設定し、ゲートウェイが `public_url` から IdP `redirect_uri` を構築するため、ステップ 1 の OAuth クライアントに 2 番目のリダイレクト URI として `http://localhost:8080/oauth/callback` を追加します。検証リンクはローカルブラウザで開きます。

    Windows PowerShell では、`curl.exe` を実行します。ベア `curl` は `Invoke-WebRequest` のエイリアスであり、これらのフラグを拒否します。

    まず、ディスカバリードキュメントを取得します。これはゲートウェイが起動し、設定が有効であり、すべての起動チェックが合格したことを確認します。

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    応答には `response_types_supported` や `scopes_supported` などの追加フィールドが含まれます。

    次に、デバイス認可をリクエストします。これはデバイスサインインフローが機能し、Postgres が到達可能で書き込み可能であることを確認します。

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    3 番目に、ブラウザで `verification_uri_complete` を開いてコードを確認することでブラウザレッグをテストします。IdP のサインインページにリダイレクトされ、サインイン後、ゲートウェイに戻ってサインイン確認に着地する必要があります。

    最初に失敗したチェックを使用して問題を特定します。

    * **最初のチェックが失敗**：起動が完了しませんでした。stderr を確認してください
    * **2 番目のチェックが失敗**：Postgres がゲートウェイから到達不可能であるか、ロールが書き込みできません。接続文字列と権限を確認してください
    * **3 番目のチェックが IdP に到達しない**：IdP のリダイレクト URI が `https://<gateway>/oauth/callback` と正確に一致することを確認してください
    * **3 番目のチェックが IdP に到達しますが、エラーで戻ります**：ゲートウェイの監査ログを読みます。これは `email domain not allowed` などの理由を含むすべての認証拒否を記録します
  </Step>

  <Step title="開発者をログインさせる">
    この最後のステップはサーバーではなく開発者マシンで発生します。そのマシンの[管理設定ファイル](/docs/ja/managed-settings#delivery-mechanisms)で `forceLoginMethod` を `"gateway"` に、`forceLoginGatewayUrl` をゲートウェイの `public_url` に設定し、`/login` を実行し、**Cloud gateway** 画面で Enter キーを押し、ブラウザサインインを完了します。以下の[ゲートウェイ URL を設定](#set-the-gateway-url)は、スケール時に両方のキーを配布することをカバーしています。
  </Step>
</Steps>

<h2 id="connect-developers">
  開発者を接続する
</h2>

開発者は独自のラップトップから 1 つのブラウザサインインで接続し、企業の仕事用アカウントを使用します。claude.ai アカウント、API キー、またはサブスクリプションは必要ありません。モデルへのリクエストは組織のアップストリーム認証情報を使用してゲートウェイを通じて行くためです。接続は、MDM 経由でプッシュする[クライアント側管理設定](/docs/ja/claude-apps-gateway-config#client-side-managed-settings)によって駆動されるため、開発者側に手動セットアップはありません。このセクションは管理者が設定するものをカバーしています。

CLI はゲートウェイの TLS リーフ証明書を最初の接続時にフィンガープリントし、ホスト名ごとにピン留めします。サインイン時、サイレントセッション更新時、および管理設定フェッチ時にそのピンを再度チェックしますが、推論リクエストはピンなしで標準 TLS 検証を使用します。HTTPS プロキシを通じてルーティングされたリクエストはピンチェックをスキップするため、ゲートウェイホストを `NO_PROXY` に追加して直接接続を保つようにします。

期待される SHA-256 フィンガープリントをゲートウェイ URL と一緒に公開して、開発者が比較するものを持つようにします。`/login` プロンプトはフィンガープリントの最初の 16 文字を小文字の 16 進数でコロンなしで表示します。証明書ファイルからこの形式で完全なフィンガープリントを出力するには、以下を実行します。

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

証明書がローテーションされると、すべての開発者は再度信頼プロンプトを見るため、ローテーションを計画されたイベントとして扱い、フィンガープリントを再公開します。ゲートウェイポリシーに[承認が必要な設定](/docs/ja/server-managed-settings#security-approval-dialogs)が含まれている場合、開発者は新しい証明書を受け入れた後、その承認ダイアログも再度見ます。Claude Code は[承認メモリ](/docs/ja/server-managed-settings#approval-memory)をピン留めされた証明書にキーイングするためです。

ゲートウェイはトークンレスポンスでオプションの `email` フィールドを返して、サインインが使用したアカウントに名前を付けることができます。そうする場合、開発者は Claude Code が認証情報を保存する前にアカウントを確認します。確認されたサインイン後、`/status` はアカウントを表示します。

確認には開発者マシンで Claude Code v2.1.275 以降が必要です。そのバージョンより下のクライアントはフィールドを無視します。`claude` バイナリのゲートウェイサーバーはフィールドを返さないため、そのサインインは確認なしで完了します。

開発者がサインインすると、[モデルピッカー](/docs/ja/model-config)は開発者の `availableModels` 許可リストのモデルを表示します。管理設定は起動時に適用され、1 時間ごとに更新され、テレメトリはコレクターにルーティングされます。

セッションは `ttl_hours` 有効期限の前にサイレントに更新されます。IdP プロビジョニング解除後の更新が失敗すると、Claude Code は開発者に再度ログインするよう促します。

<h3 id="set-the-gateway-url">
  ゲートウェイ URL を設定する
</h3>

MDM 経由またはディスク上で直接デプロイする OS ごとの[管理設定ファイル](/docs/ja/managed-settings#delivery-mechanisms)に 3 つのキーが入ります。`forceLoginMethod` と `forceLoginGatewayUrl` は `/login` を **Cloud gateway** 画面で URL が入力された状態で直接開き、`parentSettingsBehavior: "merge"` は Claude Desktop がゲートウェイの出力許可リストを起動する Claude Code セッションに配信できるようにします。これは[Claude Desktop セッションにポリシーを配信する](#deliver-policy-to-claude-desktop-sessions)で説明されています。

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

開発者は Enter キーを押して接続します。[最初の接続 TLS フィンガープリントプロンプト](#connect-developers)は引き続き表示されます。ファイルがマシンに配置されると、ゲートウェイサインインを完了していない開発者は、[Administrator policy requires a Cloud gateway sign-in](/docs/ja/errors#administrator-policy-requires-a-cloud-gateway-sign-in)で説明されているメッセージの 1 つを見ます。`CLAUDE_CODE_USE_BEDROCK` などの環境変数を通じてクラウドプロバイダーを選択する開発者はゲートウェイサインインを必要としません。

開発者はこれを手動で設定することはできません。ログインピッカーにはゲートウェイオプションがなく、`forceLoginGatewayUrl` は開発者独自の設定ファイルでは無視されます。URL なしの `forceLoginMethod` のみでは、開発者を「IT 管理者に連絡してください」メッセージのままにします。ログインキーは、マシンにプッシュするファイルに属し、ゲートウェイの `managed.policies[].cli` ブロックには属しません。このブロックは既に接続されているクライアントにのみ到達します。

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  公開アドレス空間でゲートウェイを許可する
</h3>

一部の組織は、キャリアの独自アドレス空間またはレガシー `/8` など、所有する公開 IPv4 ブロックから内部ネットワークに番号を付けるため、ゲートウェイはプライベートアドレスを持つことができません。これらのブロックを `gatewayInternalNetworks` 管理設定にリストします。`/login` は、開発者のマシンが同じブロック内のアドレスからそれに接続するときに、リストされたブロック内のゲートウェイを受け入れます。これには開発者マシンで Claude Code v2.1.268 以降が必要です。以前のバージョンはキーを無視し、プライベートアドレスルールを適用します。

<Warning>
  `gatewayInternalNetworks` は、公開アドレス空間から番号が付けられている内部ネットワーク用です。ゲートウェイをインターネットに公開することを安全にするわけではありません。信頼されたゲートウェイは開発者マシンでコマンドを実行する設定をプッシュできます。

  ファイアウォールまたはロードバランサーのルールでゲートウェイをネットワークの外から到達不可能に保ちます。ゲートウェイの [`access_control.allow_cidrs`](/docs/ja/claude-apps-gateway-config#http-tuning) をここで宣言するのと同じブロックに設定して、ゲートウェイ自体が他の場所からのクライアントを拒否するようにします。ロードバランサーまたはイングレスの背後にある場合、`listen.trusted_proxies` もそのフロントエンドに設定します。ゲートウェイはそうでなければ `allow_cidrs` をフロントエンド独自のアドレスではなく開発者のアドレスと照合するためです。
</Warning>

キーをログインキーと同じ管理設定ソースに追加します。管理設定ファイル、MDM プロファイル、またはレジストリポリシーです。Claude Code はユーザー、プロジェクト、およびサーバー管理設定でそれを無視します。

この例は 1 つのブロックを宣言します。`203.0.113.0/24` を独自のブロックに置き換えます。これはドキュメンテーション範囲であり、Claude Code はそれを拒否します。

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code はゲートウェイに接続する前に `/login` でリストを検証します。

* 各エントリは IPv4 ブロックで、最初のアドレスと `/8` から `/32` のプレフィックスとして記述されます。
* リストは最大 4 つのブロックを保持し、2 つは重複しません。
* ブロックはプライベートアドレス空間と重複しません。`10.0.0.0/8`、`172.16.0.0/12`、`192.168.0.0/16`、`127.0.0.0/8`、`169.254.0.0/16`、および `100.64.0.0/10`。`/login` は既にこのキーなしでそこのゲートウェイを受け入れます。
* ブロックは組織のネットワークになることのない空間と重複しません。`198.18.0.0/15` と `192.0.0.0/24`。VPN と NAT64 クライアントはローカルアドレスとして保持します。ドキュメンテーション範囲 `192.0.2.0/24`、`198.51.100.0/24`、および `203.0.113.0/24`。および予約範囲 `0.0.0.0/8`、`192.88.99.0/24`、およびマルチキャスト `224.0.0.0/4`。一部の大規模ネットワークが内部ユニキャスト空間として使用する `240.0.0.0/4` 内のブロックを宣言できます。

`managed-settings.json` とその `managed-settings.d/` ドロップインファイルのブロックは 1 つのリストに結合され、これらの制限は結合されたリストに適用されます。ブロックを絞るには、ドロップインで 2 番目の重複するものを追加するのではなく、そのエントリを置き換えます。`/login` は重複を拒否します。

エントリがルールを破るか、値が文字列のリストでない場合、Claude Code はそのマシンでのすべての新しいゲートウェイサインインを拒否し、メッセージで問題に名前を付けます。プライベートアドレスのゲートウェイへのサインインも失敗し、既存のサインインは機能し続けます。1 つのマシンでその値を試してからデプロイします。Claude Code はまた、[報告する無効な管理設定](/docs/ja/managed-settings#keys-that-fail-closed)の中に誤った型の値をリストします。

有効なリストでは、`/login` はアドレスがリストされたブロック内にあるゲートウェイに 3 つのチェックを適用します。

* ゲートウェイのホスト名が解決するすべてのアドレスはそのブロック内にあります。Claude Code は、プライベートおよび IPv6 アドレスを含む、それの外にもレコードを持つ名前を拒否します。
* 開発者のマシンは同じブロック内からの接続です。Claude Code は NAT の背後、コンテナまたは WSL2 内、またはアドレスプールがブロックの外にある VPN 上のマシンを拒否し、マシンが接続したアドレスに名前を付けます。
* 接続は直接です。`HTTPS_PROXY` がゲートウェイホストに適用される場合、`/login` は拒否し、追加する `NO_PROXY` エントリに名前を付けます。

3 つすべてが通過すると、[信頼プロンプト](#connect-developers)はマシンのアドレス、ゲートウェイのアドレス、および両方を含む宣言されたブロックに名前を付ける行を追加します。

キーは他のゲートウェイについては何も変わりません。プライベートアドレスのゲートウェイへのサインインは以前と同様に機能し、リストされたすべてのブロックの外の公開アドレスのゲートウェイへのサインインは以前と同様に拒否されます。

宣言されたブロックはサインインできるユーザーを絞りますが、マシンがどこにあるかを証明しません。そのため、組織が管理するアドレス空間のみを宣言します。クラウドプロバイダーの公開範囲など、他のテナントと共有されるブロックは、それに含まれるすべてのユーザーが同じチェックを通過できるようにします。

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Claude Desktop セッションにポリシーを配信する
</h3>

Claude Desktop は Cowork タブと Code タブ、および有効にした場合は Chat タブを、埋め込み Claude Code セッションで実行し、それらのモデルリクエストをゲートウェイを通じて送信します。ゲートウェイが `/user/bootstrap` で提供する設定から構築されたポリシーを各セッションに渡します。モデル許可リスト、無効化されたツール、および一致したポリシーの `cli` ブロックから派生した出力許可リスト、および[`desktop` オーバーレイ](/docs/ja/claude-apps-gateway-config#claude-desktop-overlay)です。

hooks、`env`、および `Bash(npm *)` のようなスコープ付き権限ルールなどの他の `cli` キーは、`/login` を通じてサインインするクライアントにのみ到達します。Claude Desktop はゲートウェイ URL を独自の管理設定から読み取り、[ゲートウェイ URL を設定する](#set-the-gateway-url)の `forceLoginMethod` と `forceLoginGatewayUrl` キーとは別の独自のフローでサインインします。

起動プロセスによって渡される設定は親設定です。Claude Code は、管理者がデプロイした管理ソースを持つマシンで親設定を無視します。ただし、[ポリシーを配信するソース](/docs/ja/managed-settings#which-managed-source-claude-code-uses)が `parentSettingsBehavior: "merge"` を設定する場合を除きます。

<h4 id="which-machines-need-the-opt-in">
  どのマシンがオプトインを必要とするか
</h4>

Claude Desktop のみを実行するマシンはそれを必要とします。Claude Desktop は埋め込みセッションにモデルリストと無効化されたツールリストを適用しますが、出力許可リストは親設定としてのみそれらに到達します。`WebFetch` ドメインルールとサンドボックスネットワークルールの形式です。オプトインなしでは、これらのセッションは出力制限なしで実行され、何も警告しません。ゲートウェイはポリシーが許可しないモデルの推論リクエストを引き続き拒否します。

`/login` を通じてサインインする開発者のマシンはそれを必要としません。各 Claude Code セッションはゲートウェイからポリシーをフェッチします。

[`policyHelper`](/docs/ja/settings-reference#policyhelper)が管理設定を提供するフリートはそれを使用できません。Claude Code はそれらのフリートで親設定をマージしません。ヘルパーの出力からのみ管理設定を読み取るためです。

<h4 id="set-the-opt-in">
  オプトインを設定する
</h4>

[ゲートウェイ URL を設定する](#set-the-gateway-url)から管理設定スニペットをデプロイし、ファイルを上回るクライアント側ソースにミラーリングしてから検証します。

<Steps>
  <Step title="管理設定ファイルにオプトインをデプロイする">
    [上記のスニペット](#set-the-gateway-url)には既に `parentSettingsBehavior: "merge"` が含まれているため、マシンにプッシュするファイルはそれを含みます。
  </Step>

  <Step title="ファイルを上回るソースにスニペットをミラーリングする">
    Claude Code は `parentSettingsBehavior` を[選択されたソース](/docs/ja/managed-settings#which-managed-source-claude-code-uses)からのみ読み取ります。クライアント側ソースにポリシーキーを追加すると、そのソースが選択されたものになる可能性があるため、クライアント側ソースでは `parentSettingsBehavior` のみではなくスニペット全体をミラーリングします。[クライアント側管理設定](/docs/ja/claude-apps-gateway-config#client-side-managed-settings)は Group Policy または設定プロファイルを通じてポリシーを配信するフリートをカバーしています。macOS の管理設定プリストまたは Windows の HKLM ポリシーは `managed-settings.json` ファイルを上回り、ゲートウェイ独自のリモート管理設定は両方を上回るため、ゲートウェイにサインインするマシンでは、ゲートウェイポリシーの [`cli` ブロック](/docs/ja/claude-apps-gateway-config#managed)にも `parentSettingsBehavior` を設定します。
  </Step>

  <Step title="どのソースが選択されているかを確認する">
    Claude Desktop のみを実行するマシンで、Agent SDK の [`resolveSettings()`](/docs/ja/agent-sdk/typescript#resolvesettings) を呼び出し、その `sources` リストの `managed` エントリで `policyOrigin` を読み取ります。値は選択されたクライアント側ソース `plist`、`hklm`、または `file` に名前を付けます。これはスニペットを含む必要があるソースです。Claude Desktop の埋め込みセッションはゲートウェイポリシーをフェッチしないため、ゲートウェイの `cli` ブロックは選択されたソースとしてカウントされません。
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  親設定を制限する
</h3>

`parentSettingsBehavior: "merge"` をデプロイすると、Claude Code を起動するホストプロセスは親設定を提供できます。Claude Desktop だけでなく、Agent SDK アプリケーションまたは IDE 拡張機能も同様です。

Claude Code は親設定を制限キーの許可リストに対してフィルタリングしますが、許可されたキーの中には、制限するのではなくアクセスを許可できるものもあります。`allowManaged*Only` ロックを設定しない限り、ホストが提供する権限許可ルールとサンドボックス許可リストが適用されます。ポリシーの deny ルールと ask ルールはどちらの場合でも有効です。[それらは任意の許可ルールの前に評価されます](/docs/ja/permissions#manage-permissions)。

Claude Code は親が提供した [`sandbox.credentials`](/docs/ja/settings-reference#sandbox-credentials) エントリを削除形式で転送します。

* **`deny` エントリ**：`path` または `name` とモードのみで転送されます。
* **[`mode: mask`](/docs/ja/sandboxing#mask-credential-files)を持つファイルエントリ**：センチネルのみとして転送され、`injectHosts` が空のリストである全ファイルマスクとして。プロキシは親が提供したエントリの実際の値を任意のプラットフォームで置き換えることはありません。すべての構造化マスキングフィールドも削除されるため、親が提供した抽出パターンは別のソースが同じパスに設定するより厳密なマスクを置き換えることはできません。
* **`mode: mask` を持つ `envVars` エントリ**：転送されません。`deny` は親チャネルが `envVars` エントリを通じて表現できる唯一の制限です。
* **[`awsPairs` と `sigv4`](/docs/ja/sandboxing#re-sign-aws-requests)**：制限のみ転送されます。`sigv4` からは `deny` 値のみが保持され、親が `sigv4` ブロックをまったく定義する場合、すべての 3 つのリクエスト形式 `streaming`、`presigned`、および `sigv4a` を `deny` にピン留めします。`awsPairs` ペアは再署名できる形式では転送されません。従来の AWS 変数の 1 つに名前を付けるペアは、`AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY`、および `AWS_SESSION_TOKEN` の自動ペアリングを抑制したままにする不活性エントリに置き換えられます。

<h4 id="deploy-the-locks">
  ロックをデプロイする
</h4>

親設定をフィルターがサポートする制限のみに可能な限り近く保つために、すべての 5 つの `allowManaged*Only` ロックと、それらが管理する許可リストを、マージオプトインと同じソースに追加します。

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

OS ポリシー（HKLM レジストリポリシーまたは管理設定プリストなど）はこのファイルを上回るため、ファイルではなくそれを通じてスニペット全体を配信します。ゲートウェイのリモート管理設定は OS ポリシーとファイルソースを上回りますが、接続されたクライアントにのみ到達します。ロック、許可リスト、およびマージオプトインをポリシーの [`cli` ブロック](/docs/ja/claude-apps-gateway-config#managed)にミラーリングし、このファイルをデプロイしたままにします。接続しないマシン（Claude Desktop のみを実行するものを含む）はファイルからのみポリシーを取得するためです。

<h4 id="lock-behavior-across-sources">
  ソース全体のロック動作
</h4>

1 つのロックを設定しても、他のロックは制限されません。各キーは[設定リファレンス](/docs/ja/settings-reference#all-settings)で文書化されています。勝者より下の管理ソースから、2 つのサンドボックスロックは引き続き適用され、`allowManagedPermissionRulesOnly` は引き続き親が提供した許可ルールと `additionalDirectories` をブロックします。Claude Code v2.1.273 以降では、MCP サーバーロックも勝者より下のソースから適用され、それがオンの間、管理 `allowedMcpServers` リストは最優先の管理ソースから来ます。

hooks ロックと `allowManagedPermissionRulesOnly` の開発者独自のルールへの影響は、デフォルトで勝者ソースが必要です。[Claude Code が管理ソースを組み合わせる方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources)の `managedSourcesBehavior` マージオプトインの下で、Claude Code はすべてのロックについてすべてのソースが設定する最も厳密な値を適用します。[`policyHelper`](/docs/ja/settings-reference#policyhelper) フリートでは、ロックはヘルパーの出力からのみ読み取られます。

各ロックは Claude Code が開発者独自のエントリをその設定について無視するようにするため、組織の許可リストをロックの隣に含めます。

* **ネットワークドメイン**：空の管理ドメインリストでロックするとサンドボックス化された全アウトバウンドトラフィックがブロックされます。
* **MCP サーバー**：管理またはホストが提供した `allowedMcpServers` なしでロックすると、`deniedMcpServers` がブロックしないすべてのサーバーが読み込まれます。
* **読み取りパス**：`allowRead` エントリは `denyRead` 領域内のパスのみを再許可するため、管理 `denyRead` とペアにします。

<h4 id="settings-the-locks-don’t-cover">
  ロックがカバーしない設定
</h4>

5 つのロックすべてが設定されていても、6 つの親が提供した設定がフィルターを通過します。デフォルトの最初の勝ちの設定の下で、親をブロックする管理値は最優先の管理ソースにあるものです。ただし、[MCP サーバーロック](#lock-behavior-across-sources)がオンの間は `allowedMcpServers` を除きます。`managedSourcesBehavior` マージオプトインの下で、[Claude Code が管理ソースを組み合わせる方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources)は代わりにどのソースの値が適用されるかを示します。

* **`forceLoginOrgUUID`**：最優先の管理ソースが組織 UUID を設定しない場合、Claude Code は親が提供した値を尊重します。ゲートウェイサインインはこのキーをチェックしないため、最初の当事者 Anthropic ログインも使用するフリートにのみ重要です。最優先の管理ソースの組織 UUID は親の値をブロックし、Claude Code が強制するものです。そこに `forceLoginOrgUUID` を設定します。
* **`allowedMcpServers`**：最優先の管理ソースが設定しない場合、Claude Code は親が提供した許可リストを尊重します。`allowManagedMcpServersOnly` はそれをブロックしません。ロックは勝者の許可リストを管理値として強制するため、最優先の管理ソースが設定しない場合は親が提供した許可リストを含みます。最優先の管理ソースのリストは親のリストをブロックし、Claude Code が強制するリストです。ロックの隣にそこに `allowedMcpServers` を設定します。v2.1.223 より前では、任意の管理ソースのいずれかのキーの値は親のリストをブロックしました。
* **`availableModels`**：勝者の管理ソースが設定しない場合、Claude Code は親が提供したモデルリストを尊重します。フリートがモデルを制限する場合、勝者ソースに `availableModels` を設定します。
* **`strictKnownMarketplaces`**：勝者の管理ソースが設定しない場合、Claude Code は親が提供したプラグインマーケットプレイス許可リストを尊重します。フリートがマーケットプレイスを制限する場合、勝者ソースに `strictKnownMarketplaces` を設定します。Claude Code v2.1.282 以降が必要です。
* **`blockedMarketplaces`**：親が提供したマーケットプレイスブロックリストは通過し、管理ソースが設定するブロックリストに追加されます。ブロックリストはさらに制限することのみができるためです。Claude Code v2.1.282 以降が必要です。
* **`strictPluginOnlyCustomization`**：このキーはロックに関係なくフィルターを通過し、Claude Code が開発者独自のカスタマイズ（保護フックを含む）を無視するようにします。ロックはそれをブロックしません。

<h3 id="connect-claude-desktop">
  Claude Desktop を接続する
</h3>

[Claude Desktop](/docs/ja/desktop)は異なる MDM キーを通じて同じゲートウェイに接続します。Claude Desktop の[管理設定](https://claude.com/docs/third-party/claude-desktop/configuration)で `bootstrapUrl` を `<listen.public_url>/user/bootstrap` に設定し、ユーザーのポリシーを `desktop` キーでオプトインします。[Claude Desktop オーバーレイ](/docs/ja/claude-apps-gateway-config#claude-desktop-overlay)は両方の部分をカバーしています。ゲートウェイサーバーで Claude Code v2.1.203 以降が必要です。

Claude Desktop は同じブラウザ SSO ステップでゲートウェイのアイデンティティプロバイダーを通じて開発者にサインインし、Anthropic ではなくゲートウェイから設定をフェッチします。モデルアクセスとポリシーは CLI と同じグループごとのルールに従います。CLI と Claude Desktop の両方を使用する開発者は各々に個別にサインインします。ゲートウェイセッションはそれらの間で共有されません。

接続すると、Claude Desktop は有効なすべてのタブからモデルリクエストをゲートウェイを通じて送信します。デフォルトで Cowork タブと Code タブを表示します。Chat タブもオンにするには、Claude Desktop の[管理設定](https://claude.com/docs/third-party/claude-desktop/configuration)で `chatTabEnabled` を `true` に設定するか、Claude Code v2.1.227 以降を実行しているゲートウェイのポリシーの [`desktop` ブロック](/docs/ja/claude-apps-gateway-config#claude-desktop-overlay)で設定します。

<h3 id="ci-pipelines-and-remote-machines">
  CI パイプラインとリモートマシン
</h3>

無人パイプラインのサービストークンフローはありません。ゲートウェイサインインは常にブラウザデバイスフローを実行するため、サインインを承認する開発者がない CI ジョブは認証できません。これらをプロバイダーに対して直接設定します。

開発者がサインインすると、そのマシンでの各 Claude Code セッションはゲートウェイセッションを使用します。非対話的な `claude -p` 実行と Agent SDK によって開始されたセッションを含みます。Claude Code は[ゲートウェイポリシー](/docs/ja/claude-apps-gateway-config#managed)をそれぞれに適用します。

デバイスフローはポーリング CLI を承認ブラウザから分離するため、ディスプレイのないリモート開発ボックスは引き続き機能します。開発者はリモートマシンで SSH 経由で `/login` を実行し、ラップトップのブラウザで検証リンクを開きます。

<h3 id="whats-enforced-on-developers">
  開発者に何が強制されるか
</h3>

これらの保証はすべての `/login` を通じてサインインしたセッションに適用されます。Claude Desktop が起動する埋め込みセッションは[Claude Desktop セッションにポリシーを配信する](#deliver-policy-to-claude-desktop-sessions)で説明されているようにポリシーを取得し、テレメトリの箇条書きはそれらのエクスポートがどこに行くかを示します。

* **モデルアクセス**：ポリシーが許可しないモデルのリクエストは 400 を返し、`/model` ピッカーはポリシーの `availableModels` 許可リストにフィルタリングされます。ポリシーで [`enforceAvailableModels: true`](/docs/ja/model-config#default-model-behavior) を設定して、Default オプションが Claude Code の組み込みデフォルトではなく `availableModels` 内のモデルに解決されるようにします。なしでは、Default は選択可能なままであり、そのモデルが許可されていない場合、リクエスト時に拒否されます。
* **テレメトリ宛先**：`/login` を通じてサインインしたセッションでは、CLI はローカルに設定された `OTEL_EXPORTER_OTLP_ENDPOINT` に関係なく、OTLP/HTTP エクスポートをゲートウェイに送信します。ただし、ポリシーが[コレクターをエンドポイントとして指定](/docs/ja/claude-apps-gateway-config#export-directly-to-your-collector)する場合を除きます。ゲートウェイは [`telemetry.forward_to`](/docs/ja/claude-apps-gateway-config#telemetry) の宛先にそれらをリレーします。
  * [Claude Desktop が起動する](#connect-claude-desktop)埋め込みセッションでは、CLI はエクスポートを設定された `OTEL_EXPORTER_OTLP_ENDPOINT` に送信します。CLI はそのエンドポイントがゲートウェイ自体を指す場合にのみ、ゲートウェイセッショントークンをそれらのエクスポートに添付します。
  * 信号に設定された宛先がない場合、ゲートウェイはそれを受け入れて破棄します。
  * 既に Claude Code テレメトリを直接収集する場合は、コレクターを `forward_to` 宛先として追加するか、ポリシーで指定してリレーをスキップします。
* **認証情報**：ゲートウェイトークンはセッションの唯一の認証情報です。[Anthropic プロファイル](/docs/ja/authentication#anthropic-profiles-and-federation-credentials)および以前の claude.ai ログインはサインイン中は無視されるため、開発者は最初に claude.ai からログアウトする必要はありません。設定された `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN`、または `apiKeyHelper` 認証情報については、[Administrator policy requires a Cloud gateway sign-in](/docs/ja/errors#administrator-policy-requires-a-cloud-gateway-sign-in)を参照してください。
* **管理設定**：ロックされたキーはローカルでオーバーライドできません。CLI はポリシーを起動時に適用し、[次の起動時にのみ適用される変更](/docs/ja/server-managed-settings#fetch-and-caching-behavior)を除いて、毎時間のポーリングで変更を適用します。
* **ゲートウェイが到達不可能な状態での起動**：サインイン済みセッションは、設定なしで起動するのではなく、約 10 秒後に起動時にエラーで終了します。
* **ゲートウェイがセッションを終了した後の起動**：[起動時の失敗クローズを強制する](/docs/ja/server-managed-settings#enforce-fail-closed-startup)を参照して、どの起動がゲートウェイからサインアウトした状態で開き、どの起動がゲートウェイが `401` で応答するときに終了するかを確認します。
* **プロビジョニング解除**：ユーザーが IdP で無効化されたセッションは、次の更新が失敗したときに `ttl_hours` 内に期限切れになります。
* **サインアウト**：`/logout` はゲートウェイ認証情報を開発者のマシンから削除します。
  * ゲートウェイの検出ドキュメントがゲートウェイ URL 独自のスキーム、ホスト、およびポートで `revocation_endpoint` をアドバタイズする場合、`/logout` は保存されたトークンをそのエンドポイントに送信して、ゲートウェイがその側でセッションを終了できるようにします。リクエストはベストエフォートであるため、エンドポイントが応答するかどうかに関係なく、サインアウトは開発者のマシンで完了します。失効には開発者マシンで Claude Code v2.1.275 以降が必要です。
  * `claude` バイナリのゲートウェイサーバーはアドバタイズしないため、そこからのサインアウトは開発者のマシンでのみセッションを終了します。サーバー側でセッションを強制的に終了するには、[JWT シークレットローテーション](/docs/ja/claude-apps-gateway-deploy#jwt-secret-rotation)を参照してください。

<h3 id="what-the-organization-can-see">
  組織が見ることができるもの
</h3>

使用状況テレメトリは開発者のアイデンティティ、トークン数、モデル、およびレイテンシを組織のコレクターに伝えます。ゲートウェイはプロンプトまたは完了コンテンツをログまたは保存しません。ログやトレースなどのより豊富なテレメトリが収集されるかどうか。コマンドやファイルパスを含む可能性があるのは、組織の[宛先ごとの選択](/docs/ja/claude-apps-gateway-config#telemetry)です。

<h2 id="availability-and-limitations">
  可用性と制限
</h2>

表は、開発者がゲートウェイを通じて接続するときに機能する Claude Code 機能と、ゲートウェイサーバー自体がサポートするものをカバーしています。何かがサポートされていない場合、Notes 列は代替案を提供します。

ゲートウェイは、CLI がすべてのアップストリームに送信する [`anthropic-beta`](https://platform.claude.com/docs/ja/api/beta-headers) 値を配信するため、オペレーターはベータ許可リストを維持しません。Amazon Bedrock の場合、ヘッダーを無視し、ゲートウェイは値をリクエストボディの `anthropic_beta` フィールドに移動します。他のアップストリームは送信されたままヘッダーを受け取ります。

| 機能                                                                                                    | ステータス      | 注記                                                                                                                                                                                                                                                                                                                                               |
| ----------------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 推論転送（Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、Microsoft Foundry、Anthropic） | 利用可能       | アップストリームごとのモデル変換とフェイルオーバー付き。Amazon Bedrock アップストリームは `bedrock-runtime` エンドポイントと AWS デフォルト認証情報チェーンを使用します。Amazon Bedrock [Mantle エンドポイント](/docs/ja/amazon-bedrock#use-the-mantle-endpoint)はサポートされたアップストリームではありません。[Claude Platform on AWS アップストリーム](/docs/ja/claude-apps-gateway-config#claude-platform-on-aws)には、ゲートウェイサーバー上の Claude Code v2.1.198 以降が必要です。 |
| IdP グループによるモデルアクセスと管理設定                                                                               | 利用可能       | モデルアクセスはサーバー側で強制されます。管理設定は IdP グループごとに配信され、CLI によって[管理設定層](/docs/ja/settings#settings-precedence)で適用されます                                                                                                                                                                                                                                              |
| Claude Desktop                                                                                        | 選択的に利用可能   | ゲートウェイは、ポリシーが [`desktop` キー](/docs/ja/claude-apps-gateway-config#claude-desktop-overlay)でオプトインした後、`/user/bootstrap` で Claude Desktop の設定を提供し、Claude Desktop は Cowork タブと Code タブからモデルリクエストを送信し、Chat タブから有効にした場合はゲートウェイを通じて送信します。Chat タブをオンにするには、[Claude Desktop を接続](#connect-claude-desktop)を参照してください。ゲートウェイサーバー上の Claude Code v2.1.203 以降が必要です。     |
| テレメトリファンアウト（OTLP/HTTP）                                                                                | 利用可能       | エクスポートごとにアイデンティティスタンプ付き。protobuf と JSON エンコーディングの両方                                                                                                                                                                                                                                                                                              |
| OIDC ID プロバイダー                                                                                        | 利用可能       | 任意の OIDC 準拠の IdP。ゲートウェイは標準 OIDC ディスカバリーと認可コードフローを実行します。[ID プロバイダーセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照して、IdP ごとの設定を確認してください                                                                                                                                                                                         |
| ユーザーごとおよびグループごとの支出制限                                                                                  | 利用可能       | [支出制限](/docs/ja/claude-apps-gateway-spend-limits)を参照してください                                                                                                                                                                                                                                                                                            |
| サーバー側ウェブ検索                                                                                            | 利用不可       | CLI はゲートウェイがルーティングするアップストリームプロバイダーを見ることができないため、ウェブ検索サポートを検証できず、ゲートウェイセッションで WebSearch を無効化します                                                                                                                                                                                                                                                    |
| [リモートコントロール](/docs/ja/remote-control)                                                                      | 利用不可       | CLI は[ゲートウェイを指定するエラー](/docs/ja/errors#remote-control-requires-the-anthropic-api)を表示します                                                                                                                                                                                                                                                                |
| [`/design-sync`](/docs/ja/commands#all-commands) と `/design-login`                                         | 利用不可       | どちらも claude.ai が必要ですが、CLI はゲートウェイセッションで claude.ai に接続しないため、どちらのコマンドもそこに表示されません                                                                                                                                                                                                                                                                   |
| `/import` と `claude import` などの機能フラグ取得が必要な機能                                                          | 利用不可       | CLI はゲートウェイセッションでフラグ取得をスキップします。[機能フラグ取得が必要な機能](/docs/ja/env-vars#features-that-need-feature-flag-fetching)は、それがオフにするものをリストします                                                                                                                                                                                                                         |
| 標準プロンプトキャッシング                                                                                         | 利用可能       | ゲートウェイは `cache_control` ブレークポイントをすべてのアップストリームに転送します。[キャッシュが存在する場所](/docs/ja/prompt-caching#where-the-cache-lives)は、CLI がマークするブロック（会話の途中で追加するシステムコンテキストを含む）をカバーしています                                                                                                                                                                                   |
| 1 時間キャッシュ TTL                                                                                         | 利用不可       | CLI はゲートウェイセッションで拡張キャッシュ TTL ベータを省略します。ゲートウェイがルーティングできるすべてのアップストリームが 1 時間 TTL をサポートしているわけではないため、ゲートウェイを通じたプロンプトキャッシングは 5 分 TTL を使用します。上記のベータヘッダーノートを参照してください                                                                                                                                                                                     |
| オートモード                                                                                                | 利用可能       | [サードパーティプロバイダールール](/docs/ja/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry)に従います。サードパーティプロバイダーで適格なモデルのみがそれを使用できます。v2.1.207 より前では、ゲートウェイセッションのオートモードは `CLAUDE_CODE_ENABLE_AUTO_MODE=1` を設定する必要があり、管理ポリシー `env` ブロック経由で配信可能でした                                                                                                  |
| グローバルキャッシュスコープとトークン効率的なツールなどのファーストパーティのみの最適化                                                          | 利用不可       | CLI はゲートウェイセッションでそれらを有効化しません。上記のベータヘッダーノートを参照してください                                                                                                                                                                                                                                                                                              |
| OTLP/gRPC                                                                                             | サポートされていない | HTTP 経由の OTLP のみ                                                                                                                                                                                                                                                                                                                                 |
| SAML、LDAP、およびその他の非 OIDC 認証                                                                            | サポートされていない | OIDC のみ。必要に応じて OIDC ブリッジで前面に配置します                                                                                                                                                                                                                                                                                                                |
| マルチテナント（複数の OIDC 発行者）                                                                                 | サポートされていない | ゲートウェイごとに 1 つの発行者。個別インスタンスを実行します                                                                                                                                                                                                                                                                                                                 |
| Windows サーバー                                                                                          | サポートされていない | Linux にデプロイします。ローカル開発用の macOS のみ                                                                                                                                                                                                                                                                                                                 |
| Helm チャート                                                                                             | 利用不可       | ゲートウェイは標準ステートレス Deployment として実行されます。[デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy#kubernetes)を参照してください                                                                                                                                                                                                                                            |
| 管理 UI                                                                                                 | 利用不可       | 設定は YAML ファイルです。変更するには再デプロイします                                                                                                                                                                                                                                                                                                                   |

<h2 id="next-steps">
  次のステップ
</h2>

クイックスタートは Docker Compose で実行されている最小設定を残します。さらに進めるには。

* グループごとの RBAC、マルチアップストリームフェイルオーバー、またはテレメトリ宛先を追加するなど、最小設定を超えて `gateway.yaml` を拡張します。[設定リファレンス](/docs/ja/claude-apps-gateway-config)はすべてのオプションをカバーしています。
* Compose から Kubernetes または Cloud Run での本番デプロイメントに移動し、IdP を適切に設定し、セキュリティモデルを確認します。[デプロイメントおよび運用ガイド](/docs/ja/claude-apps-gateway-deploy)は、IdP ごとのセットアップ、コンテナイメージ要件、ヘルスプローブ、およびトラブルシューティングをカバーしています。
* 個々の開発者またはグループに支出キャップを設定して、暴走ワークロードがコミットメント全体を消費できないようにします。[支出制限](/docs/ja/claude-apps-gateway-spend-limits)は管理 API と強制がどのように機能するかをカバーしています。
* AWS での完全な実装例については、ECS Fargate または EKS、Amazon RDS、および Secrets Manager を使用して、[AWS にデプロイ](/docs/ja/claude-apps-gateway-on-aws)を参照してください。
* Google Cloud での完全な実装例については、Cloud Run、Cloud SQL、Secret Manager を使用して、[Google Cloud にデプロイ](/docs/ja/claude-apps-gateway-on-gcp)を参照してください。
