> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Cloud に Claude apps gateway をデプロイする

> Google Cloud で Claude apps gateway を実行する実装例：Cloud Run または GKE、Cloud SQL for PostgreSQL、Secret Manager、および Google Cloud の Agent Platform への service account 認証。

<Note>
  このページでは、Google Cloud で Claude apps gateway を実行する 1 つの方法を説明します。この設定は、サポートされている本番環境デプロイメントではなく、カスタマー管理インフラストラクチャの実装例です。各部分がどのように組み合わさるかを確認してから、自分の環境に適応させてください。プラットフォーム非依存の要件については、[デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy)を参照してください。
</Note>

この例では、Google Cloud の Agent Platform をモデルアップストリームとして使用し、Cloud Run または GKE をコンピュートに使用して、Google Cloud に Claude apps gateway をプロビジョニングします。Google Workspace は例の ID プロバイダー（IdP）ですが、OpenID Connect（OIDC）準拠の任意の IdP が機能します。`oidc` ブロックのみが変わります。IdP ごとの詳細については、[ID プロバイダーのセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください。

<h2 id="what-you’ll-build">
  構築内容
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Google Cloud 上の Claude apps gateway の図：Claude Code クライアントは HTTPS 経由でゲートウェイ（Cloud Run または GKE）に接続し、ゲートウェイは VPC 内でプライベート IP Cloud SQL データベースと並行して実行され、セッション状態を保存します。ゲートウェイは OIDC 経由で Google Workspace に対してユーザーをサインインさせ、Secret Manager から設定とシークレットを読み取り、モデルリクエストを Google Cloud の Agent Platform に転送し、デプロイ時に Artifact Registry からイメージをプルします。" width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

デプロイメントは以下で構成されます：

* ゲートウェイコンテナを実行する **Cloud Run** サービスまたは **GKE** Deployment
* ゲートウェイイメージ用の **Artifact Registry** リポジトリ
* ゲートウェイの[ストア](/docs/ja/claude-apps-gateway-config#store)用のプライベート IP のみの **Cloud SQL for PostgreSQL** インスタンス
* `gateway.yaml`、JWT 署名キー、OIDC クライアントシークレット、および Postgres URL 用の **Secret Manager** シークレット
* `roles/aiplatform.user` を持つ **Service account**、Cloud Run に直接アタッチされるか、GKE 上で Workload Identity 経由でバインドされます
* Cloud Run 前の内部 Application Load Balancer（このチュートリアルではゲートウェイ用に設定しますが、作成しません）、または GKE 上のクラス `gce-internal` の内部 **GKE Ingress** である **HTTPS フロントエンド**（お客様が提供）

<h2 id="prerequisites">
  前提条件
</h2>

* 課金が有効になっている GCP プロジェクトと、上記のリソースを作成する権限
* `gcloud auth login` で認証された `gcloud` CLI、およびローカルにインストールされた Docker
* GKE トラック用：`kubectl`、および以下のウォークスルーで作成された VPC 上の GKE クラスタ
* Model Garden で必要な Claude モデルへのアクセス、それらを公開している地域内
* リダイレクト URI が `https://<gateway-host>/oauth/callback` の Google Workspace OAuth 2.0 ウェブアプリケーションクライアント。[ID プロバイダーのセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください
* ゲートウェイ用の TLS ホスト名。通常はロードバランサーを指すプライベート DNS 名

プロジェクトと地域を一度設定します：

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  ゲートウェイをデプロイする
</h2>

以下の手順は、`gcloud` コマンドを使用して完全なデプロイメントをプロビジョニングします。

<Steps>
  <Step title="API を有効にする">
    このチュートリアルで使用するサービス API を有効にします。

    ```bash theme={null}
    gcloud services enable \
      aiplatform.googleapis.com \
      artifactregistry.googleapis.com \
      sqladmin.googleapis.com \
      secretmanager.googleapis.com \
      iamcredentials.googleapis.com \
      iam.googleapis.com \
      compute.googleapis.com \
      servicenetworking.googleapis.com \
      run.googleapis.com \
      container.googleapis.com
    ```

    必要な API はデプロイメント パスによって異なります。

    * `compute` と `servicenetworking`：プライベート IP Cloud SQL パスに必要
    * `run`：Cloud Run のみ
    * `container`：GKE のみ
  </Step>

  <Step title="サービス アカウントを作成して IAM を付与する">
    ゲートウェイは Google Cloud の Agent Platform を呼び出す権限を持つ専用サービス アカウントとして実行されます。VPC 経由で Cloud SQL に到達するパスワード ユーザーを使用するため、Cloud SQL IAM ロールは不要です。

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    次に、Model Garden でプロジェクトの Claude モデルを有効にします。モデルは特定のリージョンに公開されるため、各モデル カードを確認してください。
  </Step>

  <Step title="イメージをビルドして Artifact Registry にプッシュする">
    [コンテナ イメージ要件](/docs/ja/claude-apps-gateway-deploy#container-image)に従ってイメージをビルドし（`linux-x64` glibc バイナリを使用）、プッシュします。

    ```bash theme={null}
    gcloud artifacts repositories create claude-gateway \
      --repository-format=docker --location="$REGION"
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    # Cloud Run には linux/amd64 が必要です。--provenance=false は buildx OCI
    # イメージ インデックスを回避します。これは Cloud Run が拒否します。
    docker build --platform=linux/amd64 --provenance=false \
      -t "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" .
    docker push "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>"
    ```
  </Step>

  <Step title="Cloud SQL for PostgreSQL をプロビジョニングする">
    Private Services Access 経由で VPC 上にインスタンスを作成して、パブリック IP を持たないようにします。これは `constraints/sql.restrictPublicIp` が適用されているプロジェクトの要件も満たします。

    ```bash theme={null}
    VPC=cc-gateway-vpc
    gcloud compute networks create "$VPC" --subnet-mode=custom
    gcloud compute networks subnets create cc-gateway-subnet \
      --network="$VPC" --region="$REGION" --range=10.0.0.0/24

    # Private Services Access：VPC ごとに 1 回限り
    gcloud compute addresses create "google-managed-services-${VPC}" \
      --global --purpose=VPC_PEERING --prefix-length=16 --network="$VPC"
    gcloud services vpc-peerings connect \
      --service=servicenetworking.googleapis.com \
      --ranges="google-managed-services-${VPC}" --network="$VPC"

    gcloud sql instances create claude-gateway-db \
      --database-version=POSTGRES_16 --tier=db-g1-small --region="$REGION" \
      --network="projects/${PROJECT_ID}/global/networks/${VPC}" --no-assign-ip
    gcloud sql databases create claude_gateway --instance=claude-gateway-db
    PGPASS="$(openssl rand -hex 24)"
    gcloud sql users create gateway --instance=claude-gateway-db --password="$PGPASS"

    PRIVATE_IP="$(gcloud sql instances describe claude-gateway-db \
      --format='value(ipAddresses[0].ipAddress)')"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${PRIVATE_IP}:5432/claude_gateway?sslmode=require"
    ```

    Cloud Run または GKE ランタイムは、この VPC 上にあるか、この VPC にルーティングされている必要があります。
  </Step>

  <Step title="gateway.yaml を作成する">
    `upstreams` ブロックは Google Cloud の Agent Platform を `auth: {}` で指定するため、ゲートウェイはランタイム サービス アカウントからのアプリケーション デフォルト認証情報を使用して認証します。すべてのフィールドについては、[設定リファレンス](/docs/ja/claude-apps-gateway-config)を参照してください。

    2 つの `listen` フィールドはゲートウェイの前面を説明します。

    * `public_url`：外部 `https://` オリジン。ループバック以外のバインドに必須です。[`listen` リファレンス](/docs/ja/claude-apps-gateway-config#listen)を参照してください。ゲートウェイは IdP `redirect_uri` と検出ドキュメントをこの値からのみビルドし、`X-Forwarded-*` ヘッダーからはビルドしません。
    * `trusted_proxies`：フロント エンドのソース範囲。ゲートウェイは TCP ピアがこのリストにある場合にのみ `X-Forwarded-For` を受け入れ、信頼できるホップを超えてチェーンをウォークするため、IP ごとのサインイン レート制限と監査イベントはロード バランサーの IP ではなく開発者 IP を記録します。

    `trusted_proxies` をフロント エンドに合わせて設定します。クラス `gce` の外部 GKE Ingress はリストされていません。これはパブリック転送ルール アドレスをプロビジョニングし、`/login` [プライベート ネットワーク チェック](/docs/ja/claude-apps-gateway#prerequisites)がこれを拒否します。

    | フロント エンド                                  | `trusted_proxies`                   |
    | ----------------------------------------- | ----------------------------------- |
    | ロード バランサーなしで直接到達する Cloud Run              | `[169.254.0.0/16]`                  |
    | Cloud Run の前の内部 Application Load Balancer | `169.254.0.0/16` とプロキシ専用サブネットの CIDR |
    | GKE 内部 Ingress、クラス `gce-internal`         | プロキシ専用サブネットの CIDR                   |

    以下の例は、内部ロード バランサー イン フロント オブ Cloud Run の値を使用します。

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [169.254.0.0/16, <your-proxy-only-subnet-cidr>]

    oidc:
      issuer: https://accounts.google.com
      client_id: <your-oauth-client-id>
      client_secret: ${OIDC_CLIENT_SECRET}           # GKE: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Google は offline_access を無視します。これらはリフレッシュ トークンを生成します。
      scopes: [openid, profile, email]
      extra_auth_params: { access_type: offline, prompt: consent }

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # GKE: ${file:/secrets/jwt-secret}

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # GKE: ${file:/secrets/postgres-url}

    upstreams:
      - provider: vertex
        region: <your-region>                        # $REGION と一致する必要があります
        project_id: <your-project>
        auth: {} # ランタイム サービス アカウント経由の ADC
    ```

    <Note>
      Google id\_tokens は `groups` クレームを含みません。Google Workspace を IdP として [`managed.policies`](/docs/ja/claude-apps-gateway-config#managed)でグループベースのポリシーを使用するには、[`oidc.google_groups`](/docs/ja/claude-apps-gateway-config#oidc)を設定します。これは Admin SDK Directory API を使用してドメイン全体の委任を持つサービス アカウントを通じて各ユーザーのグループを検索します。これがない場合は、代わりに `email_domain` で一致させてください。
    </Note>
  </Step>

  <Step title="Secret Manager にシークレットを保存する">
    4 つのシークレットを作成し、`roles/secretmanager.secretAccessor` を `claude-gateway` サービス アカウントに付与します。

    | シークレット                       | ソース                                       |
    | ---------------------------- | ----------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                 |
    | `gateway-oidc-client-secret` | Google Cloud Console → OAuth クライアント       |
    | `gateway-postgres-url`       | Cloud SQL ステップからの `$GATEWAY_POSTGRES_URL` |
    | `gateway-config`             | 前のステップからの完全な `gateway.yaml`               |

    シークレットがコンテナに到達する方法はトラックによって異なります。

    * GKE では Secret Manager CSI ドライバー経由でファイルとしてマウントされ、`gateway.yaml` は `${file:/secrets/...}` を参照します。
    * Cloud Run では複数のシークレットを 1 つのディレクトリにマウントできないため、`gateway.yaml` はファイルとしてマウントされ、他の 3 つは環境変数として注入されるため、`gateway.yaml` は代わりに `${GATEWAY_JWT_SECRET}`、`${OIDC_CLIENT_SECRET}`、`${GATEWAY_POSTGRES_URL}` を参照します。
  </Step>

  <Step title="デプロイ">
    <Tabs>
      <Tab title="Cloud Run">
        以下のコマンドは内部ロード バランサーの背後で本番環境用にデプロイします。

        ```bash theme={null}
        gcloud run deploy claude-gateway \
          --image="${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" \
          --region="$REGION" \
          --service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --min-instances=1 \
          --max-instances=8 \
          --timeout=3600 \
          --ingress=internal \
          --network="$VPC" --subnet=cc-gateway-subnet --vpc-egress=private-ranges-only \
          --set-secrets=/etc/claude/gateway.yaml=gateway-config:latest,GATEWAY_JWT_SECRET=gateway-jwt-secret:latest,OIDC_CLIENT_SECRET=gateway-oidc-client-secret:latest,GATEWAY_POSTGRES_URL=gateway-postgres-url:latest \
          --no-invoker-iam-check
        ```

        `--network`、`--subnet`、`--vpc-egress=private-ranges-only` 経由の直接 VPC エグレスにより、サービスは Cloud SQL プライベート IP に直接到達できます。各インスタンスは最大 [`store.max_connections`](/docs/ja/claude-apps-gateway-config#store) Postgres 接続を保持し、デフォルトは 5 です。最大インスタンス数 × `store.max_connections` を Cloud SQL ティアの接続制限以下に保ちます。[リファレンス アセット](#terraform-reference)は、この理由から `db-g1-small` ティアのインスタンスを 8 に制限しています。Google Cloud の Agent Platform エンドポイントと `accounts.google.com` へのパブリック エグレスは VPC を通さずにインターネットに直接移動するため、Cloud NAT は不要です。

        インボーカー IAM チェックは開いているか無効にする必要があります。ゲートウェイは独自の OIDC を実行し、そのクライアントは GCP トークンを持たないため、Cloud Run のインボーカー チェックは認証されていないリクエストを許可する必要があります。ゲートウェイの OIDC サインインはリクエストがコンテナに到達すると認証し、`allowed_email_domains` がサインインできるドメインをゲートします。

        2 つのフラグが認証されていないリクエストを許可します。

        * `--no-invoker-iam-check`：管理する `allUsers` バインディングなしでチェックを無効にし、Domain Restricted Sharing の下で機能します
        * `--allow-unauthenticated`：`allUsers` に `run.invoker` ロールを付与します。組織が `--no-invoker-iam-check` を許可しない場合はこれを使用します

        `--ingress` 経由のイングレス制限はインボーカー チェックから独立した別のレイヤーです。サービスを企業ネットワークに制限するために設定したままにしておきます。

        デフォルトでは Cloud Run `*.run.app` URL はパブリック アドレスに解決され、`/login` [プライベート ネットワーク チェック](/docs/ja/claude-apps-gateway#prerequisites)がこれを拒否します。2 つのトポロジーが開発者にプライベートに解決可能なホスト名を提供し、Cloud Run はどちらもプロビジョニングしません。

        * **内部 Application Load Balancer**、このページの `gateway.yaml` が想定するトポロジー：内部 Application Load Balancer をサービスの前にプロビジョニングして、内部 DNS 名と証明書を使用し、`listen.public_url` をそのホスト名に設定します。`internal` イングレス設定は既に内部 Application Load Balancer からのトラフィックを許可します。`internal-and-cloud-load-balancing` はさらに外部 Application Load Balancer を許可しますが、そのパブリック アドレスは `/login` プライベート ネットワーク チェックが拒否するため、このページのトポロジーはそれを必要としません。
        * **ロード バランサーなしの内部専用イングレス**：デプロイ コマンドをそのままにして、`listen.public_url` を `*.run.app` URL（以下の[リファレンス アセット](#terraform-reference)のデフォルト）のままにします。`*.run.app` がプライベートに解決するには、ネットワーク チームが既に Google API 用の Private Service Connect エンドポイント、`*.run.app` をそれに解決する Cloud DNS プライベート ゾーン、およびそのエンドポイントへのオンプレミス ルーティングを運用している必要があります。

        Google の [Cloud Run のプライベート ネットワーク ガイド](https://cloud.google.com/run/docs/securing/private-networking)は両方のオプションが必要とするインフラストラクチャをカバーしています。ゲートウェイがプライベート ホスト名で提供されたら、サインインを確認します。それまでは、Cloud Run のログからコンテナがブートしたことを確認します。

        最初のサインイン前に OAuth クライアントの認可リダイレクト URI を `<public_url>/oauth/callback` に更新します。`public_url` を変更した後は再デプロイします。ゲートウェイはその設定からのみパブリック オリジンをビルドし、`X-Forwarded-Host` と `X-Forwarded-Proto` を無視するためです。`X-Forwarded-For` は `listen.trusted_proxies` が設定されている場合にのみクライアント IP に対して受け入れられます。
      </Tab>

      <Tab title="GKE">
        クラスターは Cloud SQL ステップで作成された `$VPC` 上にある必要があります。ポッドがデータベースのプライベート IP に到達できるようにするためです。VPC ピアリングだけでは機能しません。Cloud SQL プライベート IP 自体がピアリングされたネットワークであり、ピアリングは推移的ではないためです。そのVPC上に新しいクラスターを作成するには、`gcloud container clusters create` に `--network="$VPC" --subnetwork=cc-gateway-subnet` を渡します。

        クラスターとそのノード プールで Workload Identity を有効にし、Google サービス アカウントを Kubernetes サービス アカウントにバインドして、ポッドがその認証情報を継承するようにします。

        ```bash theme={null}
        gcloud container clusters update <cluster> --region="$REGION" \
          --workload-pool="${PROJECT_ID}.svc.id.goog"
        # Standard クラスターでは、既存のノード プールも GKE_METADATA が必要です。
        # Autopilot はデフォルトでこれを有効にします。
        gcloud container node-pools update <pool> --cluster=<cluster> \
          --region="$REGION" --workload-metadata=GKE_METADATA

        kubectl create namespace claude-gateway
        kubectl create serviceaccount gateway -n claude-gateway

        gcloud iam service-accounts add-iam-policy-binding \
          "claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --role roles/iam.workloadIdentityUser \
          --member "serviceAccount:${PROJECT_ID}.svc.id.goog[claude-gateway/gateway]"

        kubectl annotate serviceaccount gateway -n claude-gateway \
          iam.gke.io/gcp-service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"
        ```

        [Kubernetes デプロイメント](/docs/ja/claude-apps-gateway-deploy#kubernetes)で説明されているように、ゲートウェイを標準 Deployment、Service、および内部 Ingress（クラス `gce-internal`）としてデプロイします。以下を使用します。

        * `serviceAccountName: gateway`
        * Secret Manager CSI ドライバーが `/secrets` にシークレットをマウント
        * 準備完了プローブが `GET /readyz` を指す

        ゲートウェイ Service に `timeoutSec` を上げた BackendConfig をアタッチします。GKE Ingress の背後にあるロード バランサー バックエンド サービスはデフォルトで 30 秒のタイムアウトで、長いストリーミング レスポンスを切断します。

        Workload Identity クラスターで `169.254.169.254` をブロックするエグレス NetworkPolicy を適用しないでください。ポッドは認証情報のためにメタデータ サーバーに到達する必要があります。ゲートウェイの組み込み [SSRF ガード](/docs/ja/claude-apps-gateway-deploy#threat-model-summary)がそこでの防御です。

        ゲートウェイはメタデータ エンドポイントに到達可能であることを示すブート警告をログに記録し、エグレス NetworkPolicy を適用することを提案します。Workload Identity の下ではその警告は予想されます。ポッドがエンドポイントを必要とするためです。
      </Tab>
    </Tabs>
  </Step>

  <Step title="ゲートウェイ URL を開発者マシンにプッシュする">
    ゲートウェイは実行されていますが、ゲートウェイ URL がマシンに配置されるまで、開発者は `/login` からそれに到達できません。完全な[マネージド設定スニペット](/docs/ja/claude-apps-gateway#set-the-gateway-url)を `forceLoginMethod`、`forceLoginGatewayUrl`、および `parentSettingsBehavior: "merge"` オプトインと共に、MDM 経由で各デバイスにデプロイします。開発者が手動で選択できるログイン ピッカーのゲートウェイ オプションはありません。
  </Step>
</Steps>

<h2 id="terraform-reference">
  Terraform リファレンス
</h2>

[リファレンスデプロイメントアセット](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp)はこのページの Cloud Run トラックを自動化します。設定とイメージアセットは両方のトラックに適用されます：

* `setup.sh`：API の有効化から最初のデプロイまで、完全な Cloud Run パスを歩む冪等な `gcloud` プロビジョナー
* `terraform/`：インフラストラクチャアズコードとしての同じデプロイメント。greenfield デプロイ用：Artifact Registry リポジトリを作成するための対象適用、次にイメージをビルドしてプッシュ、次に完全適用
* `gateway.yaml.example` および distroless ランタイムイメージ用の `Dockerfile`

アーティファクトは Cloud Run イングレスをデフォルトで `internal` にするため、このページのデプロイコマンドに一致します。その設定は、サービスの前に内部 Application Load Balancer がある場合もない場合も機能し、アーティファクトはロードバランサーも作成しません。アーティファクトは invoker レイヤーもデフォルトで `allUsers` `run.invoker` 付与にするため、このページのウォークスルーの `--no-invoker-iam-check` ではなく逆です。どちらでも機能し、選択は組織のポリシー制約に依存します。

アセットは実装例として提供されており、サポートされている本番環境アーティファクトではありません。環境に合わせてレビューして適応させてください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

ゲートウェイブートとログインエラーについては、プラットフォーム非依存の[トラブルシューティングテーブル](/docs/ja/claude-apps-gateway-deploy#troubleshooting)を参照してください。以下のエントリは Google Cloud に固有です。

| 症状                                                                                  | 原因                                                                                     | 修正                                                                                                                                                                                          |
| ----------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cloud Run がコンテナに到達する前に `403 Forbidden` を返す                                          | invoker IAM チェックがまだ有効                                                                  | `--no-invoker-iam-check` でデプロイするか、`--allow-unauthenticated` で `allUsers` に `run.invoker` ロールを付与します                                                                                          |
| `--no-invoker-iam-check` が `invoker_iam_disabled is not currently available` で拒否される | `constraints/run.managed.requireInvokerIam` でブロック                                      | `--allow-unauthenticated` を使用します。`constraints/iam.allowedPolicyMemberDomains` 経由の Domain Restricted Sharing もそれをブロックする場合は、GKE トラックを使用します。これはネットワークレイヤーでゲートウェイを公開し、`allUsers` バインディングはありません。 |
| デプロイ時に `Container manifest type … must support amd64/linux`                         | イメージが非 amd64 ホストでビルドされたか、buildx が OCI イメージインデックスを発行した                                  | `--platform=linux/amd64 --provenance=false` でビルドします                                                                                                                                         |
| ゲートウェイブートが Cloud Run で Postgres 接続タイムアウトエラーで終了                                      | Service が VPC にアタッチされていないか、Cloud SQL がその VPC にプライベート IP がない                            | Direct VPC egress 用に `--network` および `--subnet` でデプロイし、Cloud SQL インスタンスを `--no-assign-ip` および `--network` で同じ VPC を指すように作成します                                                               |
| Agent Platform リクエストが `403 PERMISSION_DENIED` を返す                                   | ランタイムが `claude-gateway` service account を使用していないか、モデルが Model Garden でプロジェクト用に有効になっていない | Cloud Run で `--service-account` を設定するか、GKE で Workload Identity をバインドし、各 Claude モデルを Model Garden でターゲット地域用に有効にします                                                                           |
| ストリーミング応答が固定期間後に切断される                                                               | フロントエンドリクエストタイムアウト：GKE Ingress の背後のロードバランサーバックエンドサービスはデフォルトで 30 秒、Cloud Run は 300 秒    | GKE で `timeoutSec` を上げた BackendConfig をアタッチするか、Cloud Run で `--timeout=3600` でデプロイします                                                                                                        |

<h2 id="next-steps">
  次のステップ
</h2>

* [設定リファレンス](/docs/ja/claude-apps-gateway-config)：すべての `gateway.yaml` オプション（`managed.policies` および `telemetry` を含む）
* [デプロイメントと運用](/docs/ja/claude-apps-gateway-deploy)：IdP セットアップ、ヘルスチェック、JWT シークレットローテーション、アップグレード、およびセキュリティモデル
* [Claude apps gateway 概要](/docs/ja/claude-apps-gateway)：クイックスタートと開発者の接続
