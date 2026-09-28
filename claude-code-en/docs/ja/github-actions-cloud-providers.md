> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions をクラウドプロバイダーで使用する

> Claude Code GitHub Actions を Claude API の代わりに Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry を通じて実行する

[Claude Code GitHub Actions](/docs/ja/github-actions) は、デフォルトで Claude API を呼び出します。代わりに独自のクラウドアカウントを通じて推論をルーティングするには、Claude Code GitHub Action のプロバイダー入力を設定し、ワークフローの OpenID Connect（OIDC）トークンを信頼するようにクラウドを構成します。ワークフローはそのトークンで認証するため、リポジトリに長期的なクラウド認証情報を保存する必要がありません。

<Info>
  このページは [GitHub Actions セットアップ](/docs/ja/github-actions#setup) に基づいています。ワークフローファイルと `anthropics/claude-code-action` ステップについてすでに理解していることを前提としており、クラウドプロバイダーが変更する内容のみをカバーしています。
</Info>

<h2 id="choose-your-provider">
  プロバイダーを選択する
</h2>

Claude Code GitHub Action は 3 つのプロバイダーをサポートしており、以下のセットアップステップはクラウド側の構成のみが異なります。組織が既に Claude モデルアクセスを持っているプロバイダーを使用してください。`anthropics/claude-code-action` ステップの `with:` ブロック内の 1 つの入力で、Claude Code GitHub Action にどのプロバイダーを使用するかを指定します。

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud の Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

[統合をセットアップする](#set-up-the-integration) の完全なワークフロー例には、各プロバイダーの入力が既に含まれています。

<h2 id="prerequisites">
  前提条件
</h2>

開始する前に、以下が必要です。

* Claude Code GitHub Action が実行されるリポジトリへの管理者アクセス（GitHub App をインストールしてシークレットを追加するため）
* クラウドアカウントでアイデンティティリソースを作成する権限：AWS の IAM ロールと OIDC アイデンティティプロバイダー、Google Cloud のワークロードアイデンティティフェデレーションリソースとサービスアカウント、または Azure の Microsoft Entra アプリケーション
* プロバイダーでの Claude モデルアクセス：
  * **Amazon Bedrock**: Claude モデルへのアクセスが許可されている。このページの例の `us.` モデル ID などのクロスリージョン推論プロファイルは、リージョングループのすべてのリージョンでアクセスが許可されている必要があります。[Amazon Bedrock の Claude Code](/docs/ja/amazon-bedrock) を参照してください
  * **Google Cloud の Agent Platform**: Agent Platform API が有効になっており Claude モデルへのアクセスがあるプロジェクト。[Google Cloud の Agent Platform の Claude Code](/docs/ja/google-vertex-ai) を参照してください
  * **Microsoft Foundry**: Claude モデルデプロイメントを備えた Foundry リソース。[Microsoft Foundry の Claude Code](/docs/ja/microsoft-foundry) を参照してください

<h2 id="set-up-the-integration">
  統合をセットアップする
</h2>

前提条件を超えて、Claude Code GitHub Action 用の GitHub アイデンティティ、クラウド側の信頼構成、リポジトリシークレット、およびワークフローファイルを作成します。以下のステップでそれぞれを説明します。

<Steps>
  <Step title="GitHub アイデンティティを選択する">
    Claude Code GitHub Action はコミットをプッシュし、GitHub アイデンティティを通じてコメントを投稿します。[クイックセットアップ](/docs/ja/github-actions#quick-setup) はこのために公式 Claude GitHub App をインストールします。クラウドプロバイダーを使用する場合、アイデンティティを自分で選択します。

    * **公式 [Claude GitHub App](https://github.com/apps/claude)**: リポジトリにインストールするか、既にインストールされている場合は次のステップにスキップします
    * **カスタム GitHub App**: Claude Code GitHub Action が使用する 3 つの権限のみが必要な場合に独自のアプリを作成します（[公式アプリの完全なセット](/docs/ja/github-actions#github-app-permissions) ではなく）
    * **GitHub の自動 `GITHUB_TOKEN`**: 作成またはインストールするアプリはありませんが、GitHub はそれで作成されたコミットで CI ワークフローをトリガーしません

    4 番目のステップのワークフロー例はカスタムアプリで認証します。そのステップでは、他の 2 つのオプションに対して何を変更するかも説明しています。

    カスタムアプリを作成するには、[新しい GitHub App を登録](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) し、この統合では使用しないため Webhook を無効にします。3 つのリポジトリ権限を付与します。

    * **Contents**: 読み取りと書き込み
    * **Issues**: 読み取りと書き込み
    * **Pull requests**: 読み取りと書き込み

    アプリを登録した後、秘密鍵を生成してダウンロードした `.pem` ファイルを保持し、アプリの設定ページから App ID をメモし、Claude Code GitHub Action が実行されるリポジトリに [アプリをインストール](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) します。キーと ID を 3 番目のステップでシークレットとして追加します。
  </Step>

  <Step title="クラウド認証を構成する">
    GitHub がワークフローに発行する OIDC トークンを信頼するようにクラウドを構成し、各ワークフロー実行が短期的なクラウド認証情報を取得できるようにします。各タブの箇条書きは作成する内容をまとめており、各タブはコンソールレベルのステップについてクラウドベンダー独自のガイドにリンクしています。

    <Tabs>
      <Tab title="Amazon Bedrock">
        AWS アカウントで信頼構成を作成し、[OIDC アイデンティティプロバイダーを作成するための AWS ガイド](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) に従います。

        * プロバイダー URL `https://token.actions.githubusercontent.com` とオーディエンス `sts.amazonaws.com` を使用して GitHub OIDC アイデンティティプロバイダーを追加します
        * そのプロバイダーによって Web アイデンティティとして信頼される IAM ロールを作成し、[IAM 構成](/docs/ja/amazon-bedrock#iam-configuration) からスコープ付き呼び出しポリシーをアタッチします。これは `bedrock:InvokeModel`、`bedrock:InvokeModelWithResponseStream`、`bedrock:ListInferenceProfiles`、`bedrock:GetInferenceProfile` を付与し、2 つの `aws-marketplace` サブスクリプションアクションも付与します
        * ロールの信頼ポリシーをリポジトリに制限します。`repo:your-org/your-repo:*` などのサブジェクト条件を使用します。クレーム形式については [GitHub の OIDC 強化ガイド](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) を参照してください

        ロールの ARN をメモします。次のステップでシークレットとして追加します。
      </Tab>

      <Tab title="Google Cloud の Agent Platform">
        Google Cloud プロジェクトでフェデレーションリソースを作成し、[ワークロードアイデンティティフェデレーションドキュメント](https://cloud.google.com/iam/docs/workload-identity-federation) に従います。

        * 3 つの API を有効にします：IAM Credentials、Security Token Service（STS）、および Agent Platform API（サービス名は `aiplatform.googleapis.com`）
        * 発行者が `https://token.actions.githubusercontent.com` である GitHub OIDC プロバイダーを持つワークロードアイデンティティプールを作成し、プールをリポジトリに制限する属性条件を追加します
        * `Vertex AI User` ロール（`roles/aiplatform.user`）のみを持つ専用サービスアカウントを作成し、プールがそれを偽装することを許可します

        プロバイダーの完全なリソース名とサービスアカウントのメールアドレスをメモします。次のステップでシークレットとして追加します。
      </Tab>

      <Tab title="Microsoft Foundry">
        リポジトリ用のフェデレーション認証情報を持つ Microsoft Entra アプリケーションを作成し、[GitHub Actions から認証するための Microsoft ガイド](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect) に従います。

        * Microsoft Entra アプリケーションを登録し、GitHub がリポジトリに発行するトークンを信頼するフェデレーション ID 認証情報を追加します。ユーザー割り当てマネージドアイデンティティはアプリケーションの代わりに機能します。どちらも以下でメモするクライアント ID を持ちます
        * Foundry リソースでアプリケーションに `Azure AI User` ロールを割り当てます。より狭いカスタムロールについては [Azure RBAC 構成](/docs/ja/microsoft-foundry#azure-rbac-configuration) を参照してください

        アプリケーションのクライアント ID、テナント ID、サブスクリプション ID をメモします。次のステップでシークレットとして追加します。
      </Tab>
    </Tabs>
  </Step>

  <Step title="リポジトリシークレットを追加する">
    Claude Code GitHub Action が実行されるリポジトリで、プロバイダーのシークレットを追加します。また、最初のステップでカスタム GitHub App を作成した場合は、2 つのアプリシークレットも追加します。GitHub の [GitHub Actions でシークレットを使用する](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) ガイドを参照してください。

    | シークレット                           | 必要な対象                         | 値                        |
    | -------------------------------- | ----------------------------- | ------------------------ |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                | IAM ロールの ARN             |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud の Agent Platform | プロバイダーの完全なリソース名          |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud の Agent Platform | サービスアカウントのメールアドレス        |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry             | Entra アプリケーションのクライアント ID |
    | `AZURE_TENANT_ID`                | Microsoft Foundry             | Microsoft Entra テナント ID  |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry             | Azure サブスクリプション ID       |
    | `APP_ID`                         | カスタム GitHub App               | GitHub App の ID          |
    | `APP_PRIVATE_KEY`                | カスタム GitHub App               | `.pem` 秘密鍵ファイルの内容        |
  </Step>

  <Step title="ワークフローファイルを作成する">
    プロバイダー用のワークフローファイル（`.github/workflows/claude.yml` など）を作成します。各例は `@claude` メンションに応答し、カスタムアプリで GitHub に認証し、`id-token: write` 権限を含みます。これは GitHub がクラウドプロバイダーが認証情報と交換する OIDC トークンを発行するために必要です。

    最初のステップで別の GitHub アイデンティティを選択した場合、例を調整します。

    * **公式 Claude GitHub App**: GitHub App トークン生成ステップと `github_token` 行を削除します
    * **GitHub の自動トークン**: トークン生成ステップを削除し、`github_token` 行を `github_token: ${{ secrets.GITHUB_TOKEN }}` に変更します

    <Warning>
      公開リポジトリでは、任意のユーザーからのトリガーフレーズを含むコメントがこのワークフローを開始します。認証情報ステップは Claude Code GitHub Action がコメント作成者の書き込みアクセスをチェックする前に実行されるため、アクションは App トークンを生成してクラウドプロバイダーにサインインした後にのみ未認可ユーザーを拒否します。これはログエントリを残し、Actions 分を消費します。これらの実行を避けるには、認証情報ステップの前にコメント作成者の書き込みアクセスを確認するステップを追加します。
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        `aws-region` 値を自分のものに置き換えます。認証情報ステップはそれをジョブの残りの部分のために `AWS_REGION` としてエクスポートします。

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6'
        ```

        <Tip>
          Bedrock モデル ID には `us.` などのクロスリージョン推論プロファイルプレフィックスが含まれます。モデルアクセスを許可したリージョングループのプレフィックスを使用します。
        </Tip>
      </Tab>

      <Tab title="Google Cloud の Agent Platform">
        `CLOUD_ML_REGION` 値を自分のものに置き換えます。ワークフローが `auth` ステップの出力からプロジェクト ID を読み取るため、プロジェクト ID をハードコードする必要はありません。

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
        ```
      </Tab>

      <Tab title="Microsoft Foundry">
        `your-resource-name` を Foundry リソース名に置き換えます。Claude Code はそれからエンドポイント URL を構築します。`azure/login` ステップはワークフローの OIDC トークンでサインインし、Claude Code は Azure [デフォルト認証情報チェーン](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) を通じて認証情報を取得します。

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Azure
                uses: azure/login@v2
                with:
                  client-id: ${{ secrets.AZURE_CLIENT_ID }}
                  tenant-id: ${{ secrets.AZURE_TENANT_ID }}
                  subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_foundry: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
        ```

        <Tip>
          Foundry リソース内の Claude デプロイメントと一致するモデル ID を使用します。モデル構成とバージョンピニングについては [Microsoft Foundry の Claude Code](/docs/ja/microsoft-foundry) を参照してください。
        </Tip>
      </Tab>
    </Tabs>

    任意のプロバイダーで、実行時間とコストを制限するために `claude_args` に `--max-turns` を追加できます。[コストを管理する](/docs/ja/github-actions#manage-costs) を参照してください。
  </Step>

  <Step title="セットアップをテストする">
    イシューまたは PR コメントで `@claude` をメンションし、リポジトリの Actions タブで実行を監視します。Claude は同じイシューまたは PR のコメントで返信します。
  </Step>
</Steps>

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

失敗した実行は通常、2 つの場所のいずれかで中断します。

* **認証エラー**: 通常は OIDC の設定ミス。ワークフローに `id-token: write` 権限が含まれていること、信頼構成のリポジトリ条件がリポジトリと正確に一致していること、ワークフロー内のシークレット名が追加したものと一致していることを確認します
* **トリガーと CI の問題**: Claude Code GitHub Action が Claude API を呼び出す場合と同じように動作します。メインページの [トラブルシューティングセクション](/docs/ja/github-actions#troubleshooting) と Claude Code GitHub Action の [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) を参照してください

<h2 id="what’s-next">
  次のステップ
</h2>

* [Claude Code GitHub Actions](/docs/ja/github-actions) 例、パラメーター、ベストプラクティス用
* [Amazon Bedrock の Claude Code](/docs/ja/amazon-bedrock) Bedrock モデル ID とリージョン用
* [Google Cloud の Agent Platform の Claude Code](/docs/ja/google-vertex-ai) Agent Platform モデル ID とリージョン用
* [Microsoft Foundry の Claude Code](/docs/ja/microsoft-foundry) Foundry モデルとエンドポイント構成用
