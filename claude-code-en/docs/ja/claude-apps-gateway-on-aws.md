> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# AWS に Claude apps gateway をデプロイする

> AWS で Claude apps gateway を実行する実装例：ECS Fargate または EKS、Amazon RDS for PostgreSQL、AWS Secrets Manager、および Amazon Bedrock への IAM ロール認証。

<Note>
  このページは、AWS で Claude apps gateway を実行する 1 つの方法を説明しています。この設定は、サポートされている本番環境デプロイメントではなく、カスタマー管理インフラストラクチャの実装例です。各部分がどのように組み合わさるかを確認してから、自分の環境に適応させてください。プラットフォーム非依存の要件については、[デプロイメントガイド](/docs/ja/claude-apps-gateway-deploy)を参照してください。
</Note>

この例では、Amazon Bedrock をモデルアップストリームとして使用し、[Amazon ECS](https://aws.amazon.com/ecs/) を [AWS Fargate](https://aws.amazon.com/fargate/) で実行するか、[Amazon EKS](https://aws.amazon.com/eks/) をコンピュートに使用して、AWS に Claude apps gateway をプロビジョニングします。[Okta](https://www.okta.com/) は例の ID プロバイダー（IdP）ですが、OpenID Connect（OIDC）準拠の任意の IdP が機能します。IdP ごとの詳細については、[ID プロバイダーのセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください。

<Note>
  Bedrock は AWS 上の唯一の Claude アップストリームではありません。ゲートウェイは、Bedrock の代わりに、または Bedrock と並行して、AWS 認証と AWS Marketplace 課金を備えた Anthropic 運営の Claude API である Claude Platform on AWS もサポートしています。そのアップストリームエントリ、認証情報、および IAM 権限は、このページの Bedrock スコープのものとは異なります。[Claude Platform on AWS アップストリームリファレンス](/docs/ja/claude-apps-gateway-config#claude-platform-on-aws)は何が変わるかをカバーしており、このページの残りは変わらずに適用されます。
</Note>

<h2 id="architecture">
  アーキテクチャ
</h2>

<Frame caption="例のアーキテクチャ。Amazon Bedrock をモデルアップストリームとして使用しています。Claude Platform on AWS アップストリームは同じ位置を占めます。">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="AWS 上の Claude apps gateway の図：Claude Code クライアントは HTTPS 経由でゲートウェイ（ECS Fargate または EKS）の前にある内部アプリケーションロードバランサーに接続し、プライベートサブネット内で Amazon RDS for PostgreSQL インスタンスと並行して実行されます。ゲートウェイは OIDC 経由でユーザーを企業 IdP に対してサインインさせ、AWS Secrets Manager からシークレットを読み取り、IAM ロールを使用してモデルリクエストを Amazon Bedrock に転送し、デプロイ時に Amazon ECR からイメージをプルします。" width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

ゲートウェイは、開発者が IdP を通じてサインインするネットワーク上のプライベート HTTPS エンドポイントとして実行されます。Claude Code セッションは、ゲートウェイの IAM ロールを通じて Amazon Bedrock 上の Claude モデルに到達するため、モデル認証情報は開発者マシンに到達しません。参照設定は以下をプロビジョニングします：

* **Amazon ECS on AWS Fargate** サービスまたは **Amazon EKS** デプロイメント（ゲートウェイコンテナを実行）
* **Amazon ECR** リポジトリ（ゲートウェイイメージ用）
* **Amazon RDS for PostgreSQL** インスタンス（プライベートサブネット内、公開アクセス不可、ゲートウェイの[ストア](/docs/ja/claude-apps-gateway-config#store)用）
* **AWS Secrets Manager** シークレット（JWT 署名キー、OIDC クライアントシークレット、Postgres URL 用）
* **IAM ロール**（`bedrock:InvokeModel`、`bedrock:InvokeModelWithResponseStream`、`bedrock:CountTokens` 権限付き、ECS タスクロールとしてアタッチされるか、EKS 上の IAM Roles for Service Accounts（IRSA）経由でバインドされる）
* **内部アプリケーションロードバランサー**（HTTPS 用）

<h2 id="prerequisites">
  前提条件
</h2>

このウォークスルーではゲートウェイ独自のリソースを作成しますが、既に存在するネットワークおよびアイデンティティインフラストラクチャの上に構築されます。開始する前に、以下が必要です。

* [上記のリソース](#architecture)を作成する権限を持つ AWS アカウント
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) がインストールされ[認証済み](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html)であること、および [Docker](https://docs.docker.com/get-started/get-docker/) がローカルにインストールされていること
* 異なるアベイラビリティゾーンに少なくとも 2 つの[プライベートサブネット](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html)を持つ [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html)。[NAT ゲートウェイ](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html)を通じたアウトバウンドインターネットアクセスがあること。内部ロードバランサーは 2 つの AZ のサブネットが必要であり、ゲートウェイは Bedrock および IdP へのエグレスが必要です
* リダイレクト URI が `https://<gateway-host>/oauth/callback` の Okta OIDC ウェブアプリケーション。[ID プロバイダーのセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください
* ゲートウェイ用の TLS ホスト名。通常は [Route 53 プライベートホストゾーン](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html)内の内部 DNS 名でロードバランサーを指し、そのホスト名用の [ACM 証明書](https://docs.aws.amazon.com/acm/latest/userguide/gs.html)があり、[AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)によってインポートまたは発行されていること

<h3 id="set-your-environment-variables">
  環境変数を設定する
</h3>

このページのすべてのコマンドはシェルから 4 つの値を読み込みます。`AWS_REGION`、`ACCOUNT_ID`、`VPC_ID`、および `PRIVATE_SUBNETS` です。

必要な Claude モデルを Bedrock が提供する US リージョンを選択してください。このウォークスルーはゲートウェイの組み込みモデルカタログに依存しており、これは `us.anthropic.*` 推論プロファイルに解決され、IAM ポリシーはそれらの ARN を許可します。US 以外のリージョンでは、そのジオの推論プロファイル ID を含む [`models:` ブロック](/docs/ja/claude-apps-gateway-config#models)を追加し、IAM ポリシーの ARN プレフィックスを変更して一致させてください。

VPC ID が手元にない場合は、`aws ec2 describe-vpcs` で VPC をリストアップし、その VPC のサブネットをリストアップして、異なるアベイラビリティゾーンにある 2 つのプライベートサブネットを見つけてください。

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

続行する前に、4 つすべてをエクスポートしてください。

```bash theme={null}
export AWS_REGION=us-east-1   # a US region where Bedrock serves the Claude models you need
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  ゲートウェイをデプロイする
</h2>

以下の手順は、`aws` コマンドを使用して完全なデプロイをプロビジョニングします。

<Steps>
  <Step title="セキュリティグループを作成する">
    3 つのセキュリティグループがトラフィックパスをチェーンします。企業ネットワークはロードバランサーに 443 で到達し、ロードバランサーはゲートウェイに 8080 で到達し、ゲートウェイは Postgres に 5432 で到達します。それ以外は到達不可能です。それらをアタッチする方法は、コンピュートトラックによって異なります。

    * ECS Fargate では、デプロイステップが `$ALB_SG` をロードバランサーにアタッチし、`$GW_SG` をサービスにアタッチします。
    * EKS では、AWS Load Balancer Controller が ALB 用に独自のフロントエンドセキュリティグループを作成するため、`$ALB_SG` と `$GW_SG` は使用されません。デプロイステップの `inbound-cidrs` アノテーションがリスナーを企業ネットワークに制限し、データベースセキュリティグループはクラスタのセキュリティグループを `$GW_SG` の代わりに許可します。

    ```bash theme={null}
    ALB_SG="$(aws ec2 create-security-group --group-name claude-gateway-alb \
      --description "Claude gateway ALB" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    GW_SG="$(aws ec2 create-security-group --group-name claude-gateway-svc \
      --description "Claude gateway service" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    DB_SG="$(aws ec2 create-security-group --group-name claude-gateway-db \
      --description "Claude gateway Postgres" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"

    aws ec2 authorize-security-group-ingress --group-id "$ALB_SG" \
      --protocol tcp --port 443 --cidr <your-corporate-cidr>
    aws ec2 authorize-security-group-ingress --group-id "$GW_SG" \
      --protocol tcp --port 8080 --source-group "$ALB_SG"
    aws ec2 authorize-security-group-ingress --group-id "$DB_SG" \
      --protocol tcp --port 5432 --source-group "$GW_SG"
    ```
  </Step>

  <Step title="IAM ロールを作成して使用例フォームを送信する">
    ゲートウェイは、Bedrock で Claude モデルを呼び出す唯一の権限を持つ専用タスクロールで実行されます。[Bedrock アップストリームリファレンス](/docs/ja/claude-apps-gateway-config#amazon-bedrock)に従い、ポリシーはクロスリージョン推論プロファイル ARN と基盤となるファウンデーションモデル ARN の両方をカバーする必要があります。

    ```bash theme={null}
    cat > bedrock-invoke.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["bedrock:InvokeModel", "bedrock:InvokeModelWithResponseStream", "bedrock:CountTokens"],
        "Resource": [
          "arn:aws:bedrock:${AWS_REGION}:${ACCOUNT_ID}:inference-profile/us.anthropic.*",
          "arn:aws:bedrock:*::foundation-model/anthropic.*"
        ]
      }]
    }
    EOF
    cat > ecs-trust.json <<'EOF'
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Principal": { "Service": "ecs-tasks.amazonaws.com" },
        "Action": "sts:AssumeRole"
      }]
    }
    EOF

    aws iam create-role --role-name claude-gateway-task \
      --assume-role-policy-document file://ecs-trust.json
    aws iam put-role-policy --role-name claude-gateway-task \
      --policy-name bedrock-invoke --policy-document file://bedrock-invoke.json
    ```

    ECS には実行ロールも必要です。これは ECS エージェント自体が ECR からイメージをプルし、後で作成される Secrets Manager 値を注入するために使用します。これはゲートウェイの AWS SDK が実行時に使用するタスクロールとは別です。

    ```bash theme={null}
    aws iam create-role --role-name claude-gateway-execution \
      --assume-role-policy-document file://ecs-trust.json
    aws iam attach-role-policy --role-name claude-gateway-execution \
      --policy-arn arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy
    cat > secrets-read.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["secretsmanager:GetSecretValue", "secretsmanager:DescribeSecret"],
        "Resource": [
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-jwt-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-oidc-client-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-postgres-url-??????"
        ]
      }]
    }
    EOF
    aws iam put-role-policy --role-name claude-gateway-execution \
      --policy-name read-gateway-secrets --policy-document file://secrets-read.json
    ```

    ポリシーは、ベアの `gateway-*` ワイルドカードではなく、シークレットごとに 1 つの ARN を指定します。共有アカウントでは、ベアのワイルドカードは無関係なシークレットにも一致します。末尾の `-??????` は、Secrets Manager がすべてのシークレットの ARN に追加する 6 文字のランダムサフィックスと正確に一致します。末尾の `-*` はプレーンプレフィックスグロブであり、`gateway-postgres-url-prod` などのより長い名前にも一致します。

    IAM ポリシーはゲートウェイに Bedrock を呼び出す権限を付与し、Bedrock は商用リージョンでデフォルトでモデルアクセスを有効にします。残りのアカウントレベルのゲートは Anthropic の 1 回限りの使用例フォームです。アカウント内の誰もそれを送信していない場合は、[Amazon Bedrock コンソール](https://console.aws.amazon.com/bedrock/)を開き、モデルカタログから Anthropic モデルを選択して、フォームを完成させます。アクセスは送信直後に付与されます。[Claude Code on Amazon Bedrock](/docs/ja/amazon-bedrock#1-submit-use-case-details)で AWS Organizations フォームと送信者が必要な IAM 権限を参照してください。

    EKS トラックは、2 つの ECS ロールの代わりに IRSA ロール上で両方のポリシードキュメントを再利用します。デプロイステップを参照してください。
  </Step>

  <Step title="Amazon RDS for PostgreSQL をプロビジョニングする">
    インスタンスはプライベートサブネットで実行され、パブリックアドレスがなく、ストレージ暗号化がオンです。エンジンバージョンは Postgres 16 に固定されており、ゲートウェイがサポートする PostgreSQL 14 の下限を満たし、以下のパラメータグループファミリーがインスタンスが実行するエンジンと一致することを保証します。

    まず、プライベートサブネットにデータベースを配置するサブネットグループと、`rds.force_ssl=1` を使用してサーバーがプレーンテキスト接続を拒否するパラメータグループを作成します。エンジンバージョンは 1 回固定されます。パラメータグループのファミリーはインスタンスが実行するエンジンのメジャーバージョンと一致する必要があるためです。

    ```bash theme={null}
    aws rds create-db-subnet-group --db-subnet-group-name claude-gateway-db \
      --db-subnet-group-description "Claude gateway" --subnet-ids $PRIVATE_SUBNETS

    PG_VERSION=16
    PG_FAMILY="postgres${PG_VERSION}"
    aws rds create-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --db-parameter-group-family "$PG_FAMILY" \
      --description "Claude gateway - require TLS on every connection"
    aws rds modify-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --parameters "ParameterName=rds.force_ssl,ParameterValue=1,ApplyMethod=immediate"
    ```

    次に、生成されたマスターパスワードでインスタンスを作成します。

    ```bash theme={null}
    PGPASS="$(openssl rand -hex 24)"
    aws rds create-db-instance --db-instance-identifier claude-gateway-db \
      --engine postgres --engine-version "$PG_VERSION" \
      --db-instance-class db.t4g.micro \
      --allocated-storage 20 --db-name claude_gateway \
      --master-username gateway --master-user-password "$PGPASS" \
      --db-subnet-group-name claude-gateway-db \
      --db-parameter-group-name claude-gateway-db \
      --vpc-security-group-ids "$DB_SG" \
      --no-publicly-accessible --storage-encrypted
    ```

    リテラル `--master-user-password` 引数は、コマンド実行中のプロセステーブルおよび監査/EDR ログに表示されます。これは、シークレットステップのメモがカバーする同じ露出です。共有またはモニタリングされたホストでは、代わりに `0600` ファイルを介して `--cli-input-json` でパスワードを渡してください。バンドルの `setup.sh` は、`0600` 一時ファイルを `--cli-input-json` に渡すことで、同じ方法でシークレット値をプロセス argv から保ちます。

    インスタンスが起動するのを待ちます。これには数分かかる場合があります。その後、プライベートエンドポイントを読み取り、ゲートウェイが使用する接続文字列を組み立てます。

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` は、ゲートウェイが RDS サーバー証明書のチェーンとホスト名を検証し、暗号化するだけでなく検証することを確認します。トラストアンカーは [AWS RDS 証明書バンドル](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem)です。これは、以下のイメージビルドステップで `/etc/claude/rds-global-bundle.pem` にコピーされ、`NODE_EXTRA_CA_CERTS` を介して信頼されます。libpq スタイルの `sslrootcert=` パラメータを URL に追加しないでください。ゲートウェイのドライバーはクエリ文字列から `sslmode` のみを読み取り、`sslrootcert` を Postgres スタートアップパラメータとして転送します。サーバーはこれを拒否します。

    ECS サービスまたは EKS ポッドはこの VPC で実行され、インスタンスのプライベートエンドポイントに到達でき、`claude-gateway-db` セキュリティグループはゲートウェイのセキュリティグループのみを許可します。
  </Step>

  <Step title="gateway.yaml を書き込む">
    `upstreams` ブロックは `auth: {}` で Bedrock を指します。ゲートウェイは ECS のタスクロールまたは EKS の IRSA ロールから AWS デフォルト認証情報チェーンを介して認証します。すべてのフィールドについては、[設定リファレンス](/docs/ja/claude-apps-gateway-config)を参照してください。

    2 つの `listen` フィールドは、ゲートウェイの前にあるものに依存します。

    * `public_url`：外部 `https://` オリジン。ロードバランサーの背後で必須です。[`listen` リファレンス](/docs/ja/claude-apps-gateway-config#listen)を参照してください。ゲートウェイは IdP `redirect_uri` と検出ドキュメントをこの値からのみ構築し、`X-Forwarded-*` ヘッダーからは構築しません。
    * `trusted_proxies`：フロントエンドのソース範囲。ゲートウェイは TCP ピアがこのリストにある場合にのみ `X-Forwarded-For` を尊重し、信頼できるホップを過ぎてチェーンをウォークします。IP ごとのサインイン率制限と監査イベントは、ロードバランサーの代わりに開発者 IP を記録します。

    両方のトラックでフロントエンドは内部 ALB です。直接作成されるか、AWS Load Balancer Controller によって作成されるかは関係ありません。ALB のノードはアタッチされたサブネットからアドレスを取得するため、`trusted_proxies` をそれらのサブネットの CIDR に設定します。これはそれらのサブネット内のすべてのホストをプロキシとして信頼します。ALB のイングレスソース（企業 CIDR）がそれらと重複しないようにし、`X-Forwarded-For` を介してクライアント IP をスプーフできる信頼できないワークロードとサブネットを共有しないでください。

    ALB のクライアントポート保存属性 `routing.http.xff_client_port.enabled` は、どちらの設定でも保つことができます。オンの場合、ALB はクライアントを `203.0.113.7:54321` または `[2001:db8::1]:54321` として書き込み、ゲートウェイはポートをドロップして両方を読み取ります。

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [<your-alb-subnet-cidrs>]

    oidc:
      issuer: https://example.okta.com
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}           # EKS: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Okta org 認可サーバーは、メールとグループを省略した薄い id_token を返します。
      # ゲートウェイは /userinfo からそれらを入力します。
      userinfo_fallback: true
      # Okta は、`groups` スコープがリクエストされ、アプリのグループクレーム
      # フィルターがそれらを許可する場合にのみグループを発行します。
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # デプロビジョニングレイテンシーを制限します。より厳密な
    # 取り消しのために 1 に向かって下げます

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # IAM ポリシーの ARN がそれをカバーするように $AWS_REGION と一致させます
        auth: {} # AWS デフォルト認証情報チェーン：
    # ECS タスクロール、または EKS の IRSA
    ```

    <Note>
      `oidc` ブロックのみが Okta 固有です。Microsoft Entra ID を代わりに使用するには、`issuer` を `https://login.microsoftonline.com/<tenant-id>/v2.0` に設定し、`userinfo_fallback` と `groups` スコープをドロップし、Entra がグループ名ではなくグループ Object ID を発行することに注意してください。[`managed.policies`](/docs/ja/claude-apps-gateway-config#managed)は GUID で一致するか、`oidc.groups_claim: roles` を使用した App Roles で一致する必要があります。[ID プロバイダーセットアップ](/docs/ja/claude-apps-gateway-deploy#identity-provider-setup)を参照してください。
    </Note>
  </Step>

  <Step title="AWS Secrets Manager にシークレットを保存する">
    3 つのシークレットを作成します。IAM ステップからの実行ロールはすでにそれらを読み取ることができます。

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    各呼び出しが出力する ARN に注意してください。ECS タスク定義は ARN でシークレットを参照します。

    <Note>
      リテラル `--secret-string` 引数は、各コマンド実行中のプロセステーブルおよび監査/EDR ログに表示されます。共有またはモニタリングされたホストでは、値を `0600` ファイルに入れ、代わりに `--secret-string file://<path>` を渡してください。バンドルの `setup.sh` は、`0600` 一時ファイルを `--cli-input-json` に渡すことで、同じ方法でシークレット値をプロセス argv から保ちます。
    </Note>

    シークレットとは異なり、`gateway.yaml` 自体にはシークレット値が含まれていません。すべての認証情報は [`${VAR}` または `${file:...}` 展開](/docs/ja/claude-apps-gateway-config#secret-expansion)を通じてブート時に解決されるためです。すべてがコンテナに到達する方法はトラックによって異なります。

    * ECS では、次のステップのビルドが `gateway.yaml` をイメージにコピーして `/etc/claude/gateway.yaml` に配置し、タスク定義は 3 つのシークレットを環境変数として `secrets` フィールドを介して注入するため、YAML は `${GATEWAY_JWT_SECRET}`、`${OIDC_CLIENT_SECRET}`、および `${GATEWAY_POSTGRES_URL}` を参照します。
    * EKS では、`gateway.yaml` を ConfigMap からマウントし、シークレットを `/secrets` のファイルとしてマウントし、`${file:/secrets/...}` として参照します。Kubernetes Secrets を External Secrets Operator または Secrets Store CSI ドライバーの AWS プロバイダーで Secrets Manager からソースするか、`kubectl` で直接作成します。
  </Step>

  <Step title="イメージを構築して Amazon ECR にプッシュする">
    [コンテナイメージ要件](/docs/ja/claude-apps-gateway-deploy#container-image)に従ってイメージを構築し、`linux-x64` glibc バイナリをビルドコンテキストの `./claude` に配置します。これらの要件に従って独自の Dockerfile を作成するか、バンドルの [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile)から始めます。これは、前のステップから入力された `gateway.yaml` をイメージにコピーして `/etc/claude/gateway.yaml` に配置します。ECS では、その埋め込みコピーは設定がコンテナに到達する方法です。これが、ファイルが書き込まれた後にビルドが行われる理由です。EKS トラックは代わりにデプロイ時に ConfigMap から `gateway.yaml` をマウントするため、埋め込みコピーはそこで使用されません。

    イメージは、接続文字列の `sslmode=verify-full` のトラストアンカーとして AWS RDS 証明書バンドルも搭載しているため、最初にビルドコンテキストにダウンロードします。AWS はバンドルをローテーションします（新しい地域の CA が追加されます）。ため、チェックサムをピンするか、コミットするのではなく、ビルドごとにダウンロードします。

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    コンテナイメージ要件はバンドルをカバーしていないため、独自の Dockerfile を作成する場合は、それをコピーして信頼する 2 行を追加してください。バンドルの `Dockerfile` にはすでに両方が含まれています。

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    ECR リポジトリを作成し、Docker をそれにサインインします。イミュータブルタグは、デプロイステップがピンする `<version>` タグが後で別のイメージに静かに再ポイントされることはできないことを意味します。

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    イメージを構築してプッシュします。以下のタスク定義は `linux/amd64` を実行するため、プラットフォームはここで一致する必要があります。Fargate on ARM64（Graviton）の場合は、`linux-arm64` バイナリで `linux/arm64` を構築し、代わりに `cpuArchitecture` を `ARM64` に設定します。

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="デプロイ">
    <Tabs>
      <Tab title="ECS Fargate">
        クラスターと、ゲートウェイの stderr 用のロググループを作成します。stderr は監査イベントと運用ログの両方を搭載しています。保持は別の呼び出しであり、保持がない場合、CloudWatch はログを永遠に保ちます。90 日を監査保持ポリシーと調整します。

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        タスク定義を書き込みます。タスクロールは Bedrock 権限を搭載し、実行ロールはシークレットを注入します。Secrets Manager ステップからシークレット ARN を使用します。

        ```json claude-gateway-task.json theme={null}
        {
          "family": "claude-gateway",
          "networkMode": "awsvpc",
          "requiresCompatibilities": ["FARGATE"],
          "cpu": "1024",
          "memory": "2048",
          "runtimePlatform": { "cpuArchitecture": "X86_64", "operatingSystemFamily": "LINUX" },
          "executionRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-execution",
          "taskRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-task",
          "containerDefinitions": [
            {
              "name": "gateway",
              "image": "<account-id>.dkr.ecr.<region>.amazonaws.com/claude-gateway:<version>",
              "portMappings": [{ "containerPort": 8080 }],
              "secrets": [
                { "name": "GATEWAY_JWT_SECRET",   "valueFrom": "<gateway-jwt-secret ARN>" },
                { "name": "OIDC_CLIENT_SECRET",   "valueFrom": "<gateway-oidc-client-secret ARN>" },
                { "name": "GATEWAY_POSTGRES_URL", "valueFrom": "<gateway-postgres-url ARN>" }
              ],
              "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                  "awslogs-group": "/ecs/claude-gateway",
                  "awslogs-region": "<region>",
                  "awslogs-stream-prefix": "gateway"
                }
              }
            }
          ]
        }
        ```

        それを登録します。

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        ゲートウェイをヘルスチェックするターゲットグループを持つ内部 ALB を前に配置します。`--ip-address-type ipv4` は重要です。内部デュアルスタック ALB はパブリック範囲の AAAA レコードを公開し、`/login` プライベートネットワークチェックはそれらを拒否します。

        ```bash theme={null}
        ALB_ARN="$(aws elbv2 create-load-balancer --name claude-gateway \
          --scheme internal --type application --ip-address-type ipv4 \
          --subnets $PRIVATE_SUBNETS --security-groups "$ALB_SG" \
          --query 'LoadBalancers[0].LoadBalancerArn' --output text)"

        TG_ARN="$(aws elbv2 create-target-group --name claude-gateway \
          --protocol HTTP --port 8080 --vpc-id "$VPC_ID" --target-type ip \
          --health-check-path /readyz \
          --query 'TargetGroups[0].TargetGroupArn' --output text)"
        ```

        HTTPS リスナーを追加します。`--ssl-policy` は最新の TLS フロアをピンします。これを省略すると、レガシー `ELBSecurityPolicy-2016-08` デフォルトにフォールバックします。これは TLS 1.0/1.1 をまだ受け入れます。

        ALB はデフォルトで 60 秒間データがない接続を閉じます。ゲートウェイのキープアライブピングはストリームをそのデフォルト内に保つため、タイムアウトを上げるとピングケイデンスの上にマージンを追加します。[トラブルシューティング](#troubleshooting)行はドロップされたストリームのメカニズムと古いゲートウェイをカバーしています。以下のコマンドはリスナーを追加し、タイムアウトを上げます。

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        サービスを作成します。デプロイメント回路ブレーカーは、タスクが失敗し続けるデプロイメント（不正なイメージまたはブート不可能な設定から）を、失敗するタスクを永遠に再起動する代わりに、最後の安定した状態にロールバックします。

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        60 秒のグレースピリオドは、コールドタスクがイメージをプルし、ストアに接続し、ECS が失敗をデプロイメントに対してカウントし始める前に最初のヘルスチェックに答える時間を与えます。ターゲットグループの `GET /readyz` のヘルスチェックはストアが到達可能であることを検証するため、Postgres に到達できないタスクはローテーションに入りません。[停止動作](/docs/ja/claude-apps-gateway-deploy#outage-behavior)でトレードオフと `/healthz` 代替案を参照してください。

        タスクはパブリック IP なしのプライベートサブネットで実行されるため、すべてのエグレス（Bedrock、IdP、Secrets Manager、ECR、CloudWatch Logs へ）は NAT ゲートウェイを通過します。Bedrock トラフィックをパブリックパスから保つには、`bedrock-runtime` インターフェース VPC エンドポイントを作成し、アップストリームの `base_url` をそれを指すように設定します。[Bedrock アップストリームリファレンス](/docs/ja/claude-apps-gateway-config#amazon-bedrock)に示されているように。IdP はまだインターネットエグレスが必要です。

        開発者にプライベートに解決可能なホスト名を与えることで完了します。Route 53 プライベートホストゾーンで、ゲートウェイの内部 DNS 名を ALB にエイリアスし、`listen.public_url` をそのホスト名に設定します。ALB 自体の `*.elb.amazonaws.com` 名は内部 ALB のプライベートアドレスに解決されますが、ACM 証明書を搭載できないため、独自の名前を使用します。

        最初のサインイン前に OAuth クライアントの認可リダイレクト URI を `<public_url>/oauth/callback` に更新します。`public_url` を変更した後、新しいタグの下でイメージを再構築してプッシュし、新しいタスク定義リビジョンを登録し、再デプロイします。ECS では、設定はイメージの埋め込み `gateway.yaml` に存在し、ゲートウェイはその設定からのみパブリックオリジンを構築し、`X-Forwarded-Host` と `X-Forwarded-Proto` を無視します。`X-Forwarded-For` は、`listen.trusted_proxies` が設定されている場合にのみクライアント IP に対して尊重されます。
      </Tab>

      <Tab title="EKS">
        このトラックには、ローカルにインストールされた `kubectl` と `eksctl` が必要です。また、IAM OIDC プロバイダーと AWS Load Balancer Controller がインストールされた既存の EKS クラスターが必要です。クラスターは `$VPC_ID` 上にある必要があります。ポッドが RDS プライベートエンドポイントに到達でき、`claude-gateway-db` セキュリティグループは `$GW_SG` の代わりにクラスタのポッドまたはノードセキュリティグループを許可する必要があります。

        EKS では、ゲートウェイは ECS ロールではなく IRSA を通じて Bedrock 認証情報を取得します。IAM ステップからの `ecs-tasks.amazonaws.com` トラストポリシーはここに適用されません。IRSA には、クラスタの OIDC プロバイダーにフェデレートするトラストポリシーを持つロールが必要です。`system:serviceaccount:claude-gateway:gateway` にスコープされます。`eksctl create iamserviceaccount` は、そのロールを作成し、ポリシーをアタッチし、Kubernetes サービスアカウントに 1 つのステップでロール ARN に注釈を付けます。IAM ステップからの 2 つのポリシードキュメントをマネージドポリシーに変換します。それはアタッチできます。

        ```bash theme={null}
        BEDROCK_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-bedrock-invoke \
          --policy-document file://bedrock-invoke.json --query Policy.Arn --output text)"
        SECRETS_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-secrets-read \
          --policy-document file://secrets-read.json --query Policy.Arn --output text)"

        kubectl create namespace claude-gateway
        eksctl create iamserviceaccount --cluster <your-cluster> --region "$AWS_REGION" \
          --namespace claude-gateway --name gateway --role-name claude-gateway \
          --attach-policy-arn "$BEDROCK_POLICY_ARN" \
          --attach-policy-arn "$SECRETS_POLICY_ARN" \
          --approve
        ```

        シークレットポリシーは、Secrets Store CSI ドライバーの AWS プロバイダーがマウントするポッドのサービスアカウントを使用して行うように、ポッドが Secrets Manager 自体を読み取る場合にのみ必要です。別の方法で Kubernetes Secrets を作成する場合はドロップします。プロバイダーはポリシーの両方のアクションが必要です。ローテーションされたシークレットを調整するときに `DescribeSecret` を呼び出すため、`GetSecretValue` のみの付与はマウントされますが、最初のデプロイでローテーションの取得を停止します。

        [Kubernetes デプロイメント](/docs/ja/claude-apps-gateway-deploy#kubernetes)で説明されているように、ゲートウェイを標準 Deployment、Service、および Ingress としてデプロイします。

        * `serviceAccountName: gateway`
        * ConfigMap からマウントされた `gateway.yaml` と `/secrets` にマウントされたシークレット
        * `GET /readyz` を指すレディネスプローブ

        フロントエンドの場合、AWS Load Balancer Controller によって管理される Ingress は内部 ALB をプロビジョニングします。以下でアノテーションを付けます。

        * `alb.ingress.kubernetes.io/scheme: internal` と `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`。パブリック範囲の AAAA レコードが `/login` [プライベートネットワークチェック](/docs/ja/claude-apps-gateway#prerequisites)に公開されないようにするため。拒否します
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`。コントローラー管理のフロントエンドセキュリティグループが `0.0.0.0/0` デフォルトの代わりに企業ネットワークのみを許可するようにします
        * `alb.ingress.kubernetes.io/certificate-arn` と ACM 証明書
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`。リスナーが TLS 1.0 と 1.1 を受け入れるレガシーデフォルトポリシーにフォールバックしないようにするため
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`。ゲートウェイのストリーミングキープアライブの上のマージン。[トラブルシューティング](#troubleshooting)を参照してください

        IRSA では、AWS SDK はプロジェクトされたサービスアカウントトークンを読み取り、AWS STS と交換するため、ポッドは EC2 インスタンスメタデータサービスを必要としません。エグレス NetworkPolicy は `169.254.169.254` をゲートウェイポッドに対してブロックする場合があります。以下の[トラブルシューティング](#troubleshooting)のノードホップリミット問題は、IRSA をスキップし、ノードインスタンスロールに依存するクラスターにのみ適用されます。
      </Tab>
    </Tabs>
  </Step>

  <Step title="ゲートウェイ URL を開発者マシンにプッシュする">
    ゲートウェイは実行されていますが、開発者は `/login` からそれに到達できません。ゲートウェイ URL がマシンに存在するまで。MDM を介して各デバイスにデプロイする[マネージド設定ファイル](/docs/ja/claude-apps-gateway#set-the-gateway-url)で `forceLoginMethod` と `forceLoginGatewayUrl` を設定します。ログインピッカーにはゲートウェイオプションがなく、開発者が手動で選択することはできません。
  </Step>
</Steps>

<h2 id="terraform-reference">
  Terraform リファレンス
</h2>

[`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) の付属バンドルは、このページをコードとしてパッケージ化しています。

* **`setup.sh`** は、上記のプロビジョニング手順を ECS Fargate トラックで同じ `aws` コマンドでスクリプト化しています。べき等性があります。既存のリソースは検出されてスキップされるため、再実行しても安全です。また、デフォルト値は環境変数で上書きできます。Okta OIDC クライアントシークレットと ACM 証明書は自分で作成する必要があります。それらがない場合、実行は ECS/ALB デプロイをスキップし、不足している入力を名前付けし、`create-secret` コマンドを出力します。両方を作成して再実行してください。Bedrock ユースケースフォームと Route 53 エイリアスは、自動的に実行されるのではなく、次のステップとして出力されます。クライアント MDM プッシュはこのページからの手動ステップのままです。
* **`gateway.yaml.example`** は gateway.yaml ステップの設定テンプレートで、オプションキーはコメントアウトされた状態で含まれています。これを `gateway.yaml` にコピーし、ビルド前にすべての `REPLACE_ME` を置き換えてください。
* **`Dockerfile`** は、プリビルドされた `linux-x64` バイナリからランタイムイメージをビルドし、入力済みの `gateway.yaml` を `/etc/claude/gateway.yaml` にコピーします。また、ストアの `sslmode=verify-full` をアンカーする AWS RDS 証明書バンドルもコピーします。`setup.sh` は、ビルドコンテキストにまだ存在しない場合にのみバンドルをダウンロードします。ファイルを削除して新しいタグで再ビルドすると、AWS CA ローテーションを取得できます。設定ファイルはシークレット値を保持しません。すべての認証情報はブート時に `${VAR}` 展開を通じて解決されるためです。したがって、設定ファイルの編集は新しいタグでの再ビルドを意味します。`setup.sh` はファイルのハッシュでイメージにタグを付けることでこれを自動化します。
* **`terraform/`** は、同じ ECS Fargate スコープを宣言的にプロビジョニングします。セキュリティグループ、IAM ロール、ECR リポジトリ、RDS インスタンス、Secrets Manager シークレット、および内部 ALB の背後にある ECS サービスです。VPC とプライベートサブネットは前提条件のままで、変数として渡されます。Terraform は ECR リポジトリを作成しますがイメージはビルドしません。サービス定義はイメージを参照するため、apply は 2 パスです。リポジトリの対象 apply、その後ビルドとプッシュ、その後フル apply です。バンドルの `terraform/README.md` は変数、リモート状態、およびティアダウンについて説明しています。

このページと同様に、バンドルはサポートされている本番環境デプロイメントではなく、カスタマー管理インフラストラクチャの動作例です。それに依存する前に、自分の環境に合わせてレビューして適応させてください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

ゲートウェイのブートおよびログインエラーについては、プラットフォーム非依存の[トラブルシューティングテーブル](/docs/ja/claude-apps-gateway-deploy#troubleshooting)を参照してください。以下のエントリは AWS に固有です。

| 症状                                                                                                                                         | 原因                                                                                                                                                                                                                                                                                                                    | 修正                                                                                                                                                                                                                                           |
| ------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | ゲートウェイ名が少なくとも 1 つのパブリックアドレスに解決されます。デュアルスタック内部 ALB はパブリック範囲の AAAA レコードを公開し、[プライベートネットワークチェック](/docs/ja/claude-apps-gateway#prerequisites)は解決されたすべてのアドレスがプライベートであることを要求します                                                                                                                                                   | ALB を `--ip-address-type ipv4` で作成するか、パブリック AAAA レコードのない別の内部専用 DNS 名を提供してください                                                                                                                                                                |
| すべての Bedrock リクエストが 502 を返す。ログに `Could not load credentials from any providers` が表示される                                                     | タスクは ECS EC2 起動タイプでタスクロールなしで実行されるか、ポッドは IRSA なしで EKS ノードで実行されるため、認証情報はインスタンスメタデータから取得されます。IMDSv2 のデフォルトホップリミット 1 はコンテナ内で停止します。このページの両方のトラック（Fargate タスクロールと IRSA）はインスタンスメタデータを使用しません                                                                                                                                  | タスクロールと IRSA を優先してください。インスタンス認証情報が避けられない場合は、`aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2` でホップリミットを上げてください。[プラットフォーム非依存テーブル](/docs/ja/claude-apps-gateway-deploy#troubleshooting)はトレードオフをカバーしています |
| Bedrock リクエストが `403 AccessDeniedException` を返す                                                                                             | アカウントが Anthropic のワンタイム使用ケースフォームを送信していない、アカウントの最初の呼び出しで開始される自動 AWS Marketplace サブスクリプションがまだ完了していない、またはタスクロールのポリシーに推論プロファイルまたは基盤モデル ARN が不足している                                                                                                                                                                        | Bedrock コンソールのモデルカタログから使用ケースフォームを送信してください。フォームが送信されたばかりの場合、またはこれがアカウントの最初の呼び出しの場合は、数分後に再試行してください。`bedrock:InvokeModel` と `bedrock:InvokeModelWithResponseStream` を両方の ARN ファミリーに付与してください。                                                    |
| Bedrock がオンデマンドスループットがサポートされていないと言う `ValidationException` を返す                                                                              | カスタム `models:` エントリが、リージョンが推論プロファイルを通じてのみ提供する基盤モデル ID にマップされている                                                                                                                                                                                                                                                       | モデルをクロスリージョン推論プロファイル ID（`us.anthropic.*`）にマップしてください。組み込みカタログはすでにこれを行っています                                                                                                                                                                    |
| ECS タスクがゲートウェイがログに何も出力する前に `ResourceInitializationError` で停止する                                                                             | 実行ロールが Secrets Manager シークレットを読み取ることができない、またはプライベートサブネットが Secrets Manager または ECR へのパスを持たない                                                                                                                                                                                                                           | 実行ロールに 3 つの `gateway-` シークレット ARN に対する `secretsmanager:GetSecretValue` を付与し、NAT ゲートウェイ経由でエグレスを提供するか、NAT ゲートウェイなしで Secrets Manager、ECR、CloudWatch Logs のインターフェースエンドポイント（`awslogs` ドライバーが同じステージで必要とする）と S3 ゲートウェイエンドポイントを提供してください              |
| ゲートウェイブートが Postgres 接続タイムアウトエラーで終了する                                                                                                       | データベースセキュリティグループがゲートウェイのセキュリティグループを 5432 で許可していない、またはサービスがデータベースの VPC 外で実行されている                                                                                                                                                                                                                                       | データベースのセキュリティグループでゲートウェイのセキュリティグループから 5432 を許可し、サービスを DB サブネットグループと同じ VPC で実行してください                                                                                                                                                          |
| ゲートウェイブートが Postgres TLS 証明書検証エラーで終了する                                                                                                      | 接続文字列が `sslmode=verify-full` を設定しているが、イメージが RDS CA バンドルを信頼していない。バンドルがイメージにコピーされていない、または `NODE_EXTRA_CA_CERTS` がそれを指していない                                                                                                                                                                                              | ビルドステップの 2 つの Dockerfile 行を追加してバンドルをコピーし、`NODE_EXTRA_CA_CERTS` を設定してから、リビルドして新しいタグで プッシュし、再デプロイしてください                                                                                                                                        |
| ストリーミング応答が静止期間後にストリーム途中でドロップする                                                                                                             | v2.1.229 より前のゲートウェイが Bedrock または AWS 上の Claude Platform 上流で、上流が静止している間（例えば、ストリーム出力のない拡張思考中）は何も送信しません。ALB はデフォルトで 60 秒間データがない場合に接続を閉じるため、そのギャップでストリームを切断します。v2.1.229 以降のゲートウェイはそのタイムアウト下で静止したストリームを保持します。これらの上流では、ゲートウェイはストリームデータがない状態で約 15 秒経過すると SSE `ping` イベントを 1 回発行し、Anthropic API 上流ではゲートウェイは API 自体のピングをリレーします | ゲートウェイを v2.1.229 以降に更新するか、`idle_timeout.timeout_seconds` 属性を `3600` に設定してください。`modify-load-balancer-attributes` または EKS の `load-balancer-attributes` Ingress アノテーション経由で設定します                                                                 |

<h2 id="telemetry">
  テレメトリ
</h2>

ゲートウェイは、マシンごとの OTEL 設定なしで開発者ごとの使用メトリクスを提供します。Claude Code は OpenTelemetry（OTLP）メトリクス、ログ、およびオプトインのトレースを発行します。[使用状況の監視](/docs/ja/monitoring-usage)は CLI が報告するすべてをカバーしています。ゲートウェイセッションでは、CLI は各エクスポートに認証された IdP ID 属性 `user.id`、`user.email`、および `user.groups` でスタンプを付けるため、使用状況は `OTEL_RESOURCE_ATTRIBUTES` 配管なしで開発者ごとにロールアップされます。

ゲートウェイ自体は認証された OTLP リレーです。[`telemetry.forward_to`](/docs/ja/claude-apps-gateway-config#telemetry) を `listen.public_url` と一緒に設定し、OTEL エクスポーター設定をすべての接続されたクライアントにプッシュし、OTLP トラフィックを逐語的にリストするすべての宛先に転送します。各宛先はメトリクス、ログ、およびトレースを独立して選択し、デフォルトはメトリクスのみです。[`telemetry` リファレンス](/docs/ja/claude-apps-gateway-config#telemetry)を参照してください。シグナルごとのフィールドとそれらの感度トレードオフについて。ゲートウェイはバッファ、集約、またはテレメトリを保存しないため、データが到達する場所は完全にコレクターのエクスポーター設定です。

クライアントテレメトリはデフォルトでオフです。`telemetry.forward_to` を設定することは、接続された開発者のためにそれをオンにするものです。各インタラクティブクライアントは、[設定リファレンス](/docs/ja/claude-apps-gateway-config#telemetry)で説明されているように、プッシュされた設定の 1 回限りのセキュリティ承認ダイアログを表示します。AWS では、各シグナルは次のように宛先にマップされます。

<h3 id="client-metrics-logs-and-traces">
  クライアントメトリクス、ログ、およびトレース
</h3>

`telemetry.forward_to` を OpenTelemetry コレクター（[AWS Distro for OpenTelemetry（ADOT）コレクター](https://aws-otel.github.io/)など）に指し、Amazon CloudWatch、Amazon Managed Service for Prometheus、または任意の OTLP バックエンドにエクスポートします。

`https://` 経由で到達可能な独自の内部サービスとしてコレクターを実行します。ゲートウェイはループバック URL に対してのみプレーンテキスト `http://` を受け入れ、その場合でも [SSRF ガード](/docs/ja/claude-apps-gateway-deploy#threat-model-summary)はデフォルトで送信時にループバック接続をブロックします。`http://localhost:4318` のサイドカーコレクターは設定検証を渡しますが、トラフィックを受け取りません。エクスポートは `ECONNREFUSED_SSRF` として失敗します。ゲートウェイログで、`CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` がゲートウェイの環境に設定されていない限り。その変数はすべてのオペレーター設定 URL のループバックブロックを緩和し、テレメトリのみではなく、ネットワークが他の方法でロックダウンされているタスクのサイドカープラスフラグセットアップを予約してください。内部サービスパターンを優先します。

<h3 id="gateway-logs">
  ゲートウェイログ
</h3>

ECS Fargate では、追加のセットアップはありません。`awslogs` ドライバーはゲートウェイの stderr を配信します。これは監査イベントと運用ログを運びます。`/ecs/claude-gateway` ロググループに上記で作成されました。EKS では、ポッドログはデフォルトで CloudWatch に到達しないため、監査証跡は失われます。ログ収集をインストールするまで：コンテナログキャプチャが有効な Amazon CloudWatch Observability アドオン、または Fluent Bit DaemonSet。どちらのトラックでも、CloudWatch Logs Insights でログをクエリし、メトリクスフィルターからアラームを駆動します。

<h3 id="container-metrics">
  コンテナメトリクス
</h3>

`aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` でクラスタで Container Insights を有効にして、タスクごとの CPU、メモリ、およびネットワーク。EKS では、Amazon CloudWatch Observability アドオンをインストールします。

<h3 id="spend">
  支出
</h3>

テレメトリは事後に使用状況を表示します。[支出制限](/docs/ja/claude-apps-gateway-spend-limits)は、共有アップストリーム認証情報の上にゲートウェイのライブ開発者ごとのビューと実装です。

<h2 id="next-steps">
  次のステップ
</h2>

* [設定リファレンス](/docs/ja/claude-apps-gateway-config)：すべての `gateway.yaml` オプション。`managed.policies` と `telemetry` を含む
* [デプロイメントと運用](/docs/ja/claude-apps-gateway-deploy)：IdP セットアップ、ヘルスチェック、JWT シークレットローテーション、アップグレード、およびセキュリティモデル
* [Claude apps gateway 概要](/docs/ja/claude-apps-gateway)：クイックスタートと開発者の接続
* [Claude apps gateway の AWS サンプル](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway)：顧客環境の範囲をカバーする AWS 保守デプロイメントサンプル
