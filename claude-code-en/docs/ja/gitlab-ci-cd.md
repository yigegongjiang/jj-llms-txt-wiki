> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Claude Code を GitLab CI/CD で開発ワークフローに統合する方法を学びます

<Info>
  Claude Code for GitLab CI/CD は現在ベータ版です。機能と機能性は、エクスペリエンスを改善する際に進化する可能性があります。

  この統合は GitLab によって保守されています。サポートについては、以下の [GitLab issue](https://gitlab.com/gitlab-org/gitlab/-/issues/573776) を参照してください。
</Info>

<Note>
  この統合は [Claude Code CLI and Agent SDK](/docs/ja/agent-sdk/overview) の上に構築されており、CI/CD ジョブとカスタム自動化ワークフローで Claude をプログラム的に使用できます。
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  GitLab で Claude Code を使用する理由
</h2>

* **インスタント MR 作成**: 必要なことを説明すると、Claude は変更と説明を含む完全な MR を提案します
* **自動実装**: 単一のコマンドまたはメンションで issue を実行可能なコードに変換します
* **プロジェクト対応**: Claude は `CLAUDE.md` ガイドラインと既存のコードパターンに従います
* **シンプルなセットアップ**: `.gitlab-ci.yml` に 1 つのジョブとマスクされた CI/CD 変数を追加します
* **エンタープライズ対応**: Claude API、Amazon Bedrock、または Google Cloud の Agent Platform を選択して、データレジデンシーと調達のニーズを満たします
* **デフォルトでセキュア**: GitLab ランナーで実行され、ブランチ保護と承認が適用されます

<h2 id="how-it-works">
  仕組み
</h2>

Claude Code は GitLab CI/CD を使用して AI タスクを分離されたジョブで実行し、MR 経由で結果をコミットバックします。

1. **イベント駆動型オーケストレーション**: GitLab は選択したトリガー（例えば、issue、MR、またはレビュースレッドで `@claude` をメンションするコメント）をリッスンします。ジョブはスレッドとリポジトリからコンテキストを収集し、その入力からプロンプトを構築し、Claude Code を実行します。

2. **プロバイダー抽象化**: 環境に適したプロバイダーを使用します。
   * Claude API（SaaS）
   * Amazon Bedrock（IAM ベースのアクセス、クロスリージョンオプション）
   * Google Cloud の Agent Platform（GCP ネイティブ、Workload Identity Federation）

3. **サンドボックス実行**: 各インタラクションは厳密なネットワークとファイルシステムルールを持つコンテナで実行されます。Claude Code はワークスペーススコープの権限を適用して書き込みを制限します。すべての変更は MR を通じてフローするため、レビュアーは diff を確認でき、承認が引き続き適用されます。

地域エンドポイントを選択して、既存のクラウド契約を使用しながらレイテンシーを削減し、データソブリンティ要件を満たします。

<h2 id="what-can-claude-do">
  Claude にはどのようなことができますか？
</h2>

GitLab パイプラインでは、Claude Code は以下のことができます：

* issue の説明またはコメントから MR を作成および更新する
* パフォーマンス低下を分析し、最適化を提案する
* ブランチに機能を直接実装してから MR を開く
* テストまたはコメントで特定されたバグおよび低下を修正する
* フォローアップコメントに応答して、リクエストされた変更を反復する

<h2 id="setup">
  セットアップ
</h2>

<h3 id="quick-setup">
  クイックセットアップ
</h3>

最速で始める方法は、`.gitlab-ci.yml` に最小限のジョブを追加し、API キーをマスク変数として設定することです。

1. **マスク CI/CD 変数を追加する**
   * **Settings** → **CI/CD** → **Variables** に移動します
   * `ANTHROPIC_API_KEY` を追加します（マスク、必要に応じて保護）

2. **`.gitlab-ci.yml` に Claude ジョブを追加する**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # ジョブをトリガーする方法に合わせてルールを調整します：
  # - 手動実行
  # - マージリクエストイベント
  # - '@claude' を含むコメント時の web/API トリガー
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # インストーラーは claude を ~/.local/bin に配置しますが、このイメージの PATH には含まれていません
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # オプション：セットアップが提供する場合は GitLab MCP サーバーを起動します
    - /bin/gitlab-mcp-server || true
    # コンテキストペイロード付きの web/API トリガー経由で呼び出す場合は AI_FLOW_* 変数を使用します
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

ジョブと `ANTHROPIC_API_KEY` 変数を追加した後、**CI/CD** → **Pipelines** からジョブを手動で実行してテストするか、MR からトリガーして Claude が提案した更新をブランチで実装し、必要に応じて MR を開くことができます。

<Note>
  Claude API の代わりに Amazon Bedrock または Google Cloud の Agent Platform で実行する場合は、認証と環境セットアップについて下記の [Amazon Bedrock と Google Cloud を使用する](#using-with-amazon-bedrock-and-google-cloud) セクションを参照してください。
</Note>

<h3 id="manual-setup-recommended-for-production">
  手動セットアップ（本番環境に推奨）
</h3>

より制御されたセットアップが必要な場合またはエンタープライズプロバイダーが必要な場合：

1. **プロバイダーアクセスを設定する**：
   * **Claude API**：`ANTHROPIC_API_KEY` を作成してマスク CI/CD 変数として保存します
   * **Amazon Bedrock**：**Configure GitLab** → **AWS OIDC** を設定し、Amazon Bedrock 用の IAM ロールを作成します
   * **Google Cloud の Agent Platform**：**Configure Workload Identity Federation for GitLab** → **GCP** を設定します

2. **GitLab API 操作用のプロジェクト認証情報を追加する**：
   * デフォルトで `CI_JOB_TOKEN` を使用するか、`api` スコープを持つ Project Access Token を作成します
   * PAT を使用する場合は `GITLAB_ACCESS_TOKEN`（マスク）として保存します

3. **`.gitlab-ci.yml` に Claude ジョブを追加する**：Claude API の場合は [クイックセットアップ](#quick-setup) ジョブを使用するか、[設定例](#configuration-examples) からプロバイダージョブを使用します

4. **（オプション）メンション駆動トリガーを有効にする**：
   * 「Comments (notes)」のプロジェクト webhook をイベントリスナー（使用している場合）に追加します
   * コメントに `@claude` が含まれている場合、リスナーが `AI_FLOW_INPUT` や `AI_FLOW_CONTEXT` などの変数を使用してパイプライントリガー API を呼び出すようにします

<h2 id="example-use-cases">
  使用例
</h2>

<h3 id="turn-issues-into-mrs">
  Issue を MR に変換する
</h3>

Issue コメント内：

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude は Issue とコードベースを分析し、ブランチで変更を記述して、レビュー用の MR を開きます。

<h3 id="get-implementation-help">
  実装サポートを取得する
</h3>

MR ディスカッション内：

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude は変更を提案し、適切なキャッシング機能を備えたコードを追加し、MR を更新します。

<h3 id="fix-bugs-quickly">
  バグを素早く修正する
</h3>

Issue または MR コメント内：

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude はバグを特定し、修正を実装し、ブランチを更新するか新しい MR を開きます。

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Amazon Bedrock と Google Cloud での使用
</h2>

エンタープライズ環境では、Claude Code をクラウドインフラストラクチャ全体で実行でき、同じ開発者体験を得られます。

<Tabs>
  <Tab title="Amazon Bedrock">
    ### 前提条件

    Amazon Bedrock で Claude Code をセットアップする前に、以下が必要です。

    1. 目的の Claude モデルへの Amazon Bedrock アクセス権を持つ AWS アカウント
    2. AWS IAM で OIDC アイデンティティプロバイダーとして設定された GitLab
    3. Amazon Bedrock 権限を持つ IAM ロールと、GitLab プロジェクト/refs に制限されたトラストポリシー
    4. ロール引き受けのための GitLab CI/CD 変数：
       * `AWS_ROLE_TO_ASSUME`（ロール ARN）
       * `AWS_REGION`（Amazon Bedrock リージョン）

    ### セットアップ手順

    GitLab CI ジョブが OIDC 経由で IAM ロールを引き受けることを許可するよう AWS を設定します（静的キーは不要）。

    **必須セットアップ：**

    1. Amazon Bedrock を有効にし、目的の Claude モデルへのアクセスをリクエストします
    2. まだ存在しない場合は、GitLab 用の IAM OIDC プロバイダーを作成します
    3. GitLab OIDC プロバイダーによって信頼され、プロジェクトと保護された refs に制限された IAM ロールを作成します
    4. Amazon Bedrock invoke API に対する最小権限パーミッションをアタッチします

    [Amazon Bedrock ジョブの例](#configuration-examples)を使用して、実行時にジョブの OIDC トークンを一時的な AWS 認証情報と交換します。
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### 前提条件

    Google Cloud's Agent Platform で Claude Code をセットアップする前に、以下が必要です。

    1. 以下を備えた Google Cloud プロジェクト：
       * Google Cloud's Agent Platform API が有効化されている
       * GitLab OIDC を信頼するよう設定された Workload Identity Federation
    2. 必要な Google Cloud's Agent Platform ロールのみを持つ専用サービスアカウント
    3. GitLab CI/CD 変数：
       * `GCP_WORKLOAD_IDENTITY_PROVIDER`（`//iam.googleapis.com/` プレフィックスなしのプロバイダーリソース名。例：`projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`）
       * `GCP_SERVICE_ACCOUNT`（サービスアカウントメール）
       * `GCP_PROJECT_ID`（Google Cloud プロジェクト ID）

    ### セットアップ手順

    Workload Identity Federation 経由で GitLab CI ジョブがサービスアカウントを偽装することを許可するよう Google Cloud を設定します。

    **必須セットアップ：**

    1. IAM Credentials API、STS API、および Google Cloud's Agent Platform API を有効にします
    2. GitLab OIDC 用の Workload Identity Pool とプロバイダーを作成します
    3. Google Cloud's Agent Platform ロールを持つ専用サービスアカウントを作成します
    4. WIF プリンシパルにサービスアカウントを偽装するパーミッションを付与します

    [Agent Platform ジョブの例](#configuration-examples)を使用して、キーを保存することなく認証します。
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  設定例
</h2>

以下は、パイプラインに適応させることができる、すぐに使用できるスニペットです。

<h3 id="amazon-bedrock-job-example-oidc">
  Amazon Bedrock ジョブの例（OIDC）
</h3>

**前提条件：**

* Amazon Bedrock が有効化され、選択した Claude モデルへのアクセスが可能
* GitLab OIDC が AWS で設定され、GitLab プロジェクトと refs を信頼するロールが存在
* Amazon Bedrock 権限を持つ IAM ロール（最小権限を推奨）

**必須 CI/CD 変数：**

* `AWS_ROLE_TO_ASSUME`：Amazon Bedrock アクセス用の IAM ロールの ARN
* `AWS_REGION`：Amazon Bedrock リージョン（例：`us-west-2`）

GitLab は `id_tokens:` ブロックからジョブの OIDC トークンを生成し、`GITLAB_OIDC_TOKEN` として公開します。`aud` を AWS の IAM OIDC アイデンティティプロバイダーで設定したオーディエンス値（例：GitLab インスタンス URL）に設定します。

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
    CLAUDE_CODE_USE_BEDROCK: "1"
```

<Note>
  Amazon Bedrock のモデル ID にはリージョン固有のプレフィックスが含まれます（例：`us.anthropic.claude-sonnet-4-6`）。ワークフローがサポートしている場合は、ジョブ設定またはプロンプトを通じて目的のモデルを渡します。
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Agent Platform ジョブの例（Workload Identity Federation）
</h3>

**前提条件：**

* Google Cloud の Agent Platform API が GCP プロジェクトで有効化
* Workload Identity Federation が GitLab OIDC を信頼するように設定
* Google Cloud の Agent Platform 権限を持つサービスアカウント

**必須 CI/CD 変数：**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`：`//iam.googleapis.com/` プレフィックスなしのプロバイダーリソース名（例：`projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`）
* `GCP_SERVICE_ACCOUNT`：サービスアカウントメール
* `GCP_PROJECT_ID`：Google Cloud プロジェクト ID
* `CLOUD_ML_REGION`：Google Cloud の Agent Platform リージョン（例：`us-east5`）

GitLab は `id_tokens:` ブロックからジョブの OIDC トークンを生成し、`GITLAB_OIDC_TOKEN` として公開します。`aud` を Workload Identity Pool プロバイダーで設定したオーディエンス値（例：GitLab インスタンス URL）に設定します。ジョブはトークンをファイルに書き込み、認証情報設定の `credential_source` エントリは Google の認証ライブラリにそこから読み込むよう指示します。`GOOGLE_APPLICATION_CREDENTIALS` を認証情報設定ファイルに設定すると、[Application Default Credentials](/docs/ja/google-vertex-ai#3-configure-gcp-credentials) を通じて Claude Code で利用可能になります。

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

<Note>
  Workload Identity Federation では、サービスアカウントキーを保存する必要がありません。リポジトリ固有の信頼条件と最小権限のサービスアカウントを使用します。
</Note>

<h2 id="best-practices">
  ベストプラクティス
</h2>

<h3 id="claude-md-configuration">
  CLAUDE.md 設定
</h3>

リポジトリのルートに `CLAUDE.md` ファイルを作成して、コーディング標準、レビュー基準、プロジェクト固有のルールを定義します。Claude はランの実行中にこのファイルを読み込み、変更を提案する際にあなたの規約に従います。

<h3 id="security-considerations">
  セキュリティに関する考慮事項
</h3>

**API キーやクラウド認証情報をリポジトリにコミットしないでください**。常に GitLab CI/CD 変数を使用してください：

* `ANTHROPIC_API_KEY` をマスク変数として追加します（必要に応じて保護してください）
* 可能な限りプロバイダー固有の OIDC を使用します（長期的なキーは不要）
* ジョブの権限とネットワーク出力を制限します
* Claude の MR を他のコントリビューターと同じようにレビューします

<h3 id="optimizing-performance">
  パフォーマンスの最適化
</h3>

* `CLAUDE.md` を焦点を絞った簡潔なものに保ちます
* 明確なイシュー/MR の説明を提供して、反復を減らします
* ランナーで npm とパッケージのインストールをキャッシュします（可能な場合）

<h3 id="ci-costs">
  CI コスト
</h3>

Claude Code を GitLab CI/CD で使用する場合、関連するコストに注意してください：

* **GitLab ランナー時間**：
  * Claude はあなたの GitLab ランナーで実行され、コンピュート分を消費します
  * 詳細については、GitLab プランのランナー課金を参照してください

* **API コスト**：
  * Claude との各インタラクションは、プロンプトと応答のサイズに基づいてトークンを消費します
  * トークン使用量はタスクの複雑さとコードベースのサイズによって異なります
  * 詳細については、[Anthropic 価格](https://platform.claude.com/docs/en/about-claude/pricing)を参照してください

* **コスト最適化のヒント**：
  * 特定の `@claude` コマンドを使用して、不要なターンを減らします
  * 適切な `--max-turns` とジョブ `timeout` 値を設定します
  * 並列実行を制御するために同時実行を制限します

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude が @claude コマンドに応答しない
</h3>

* パイプラインがトリガーされていることを確認してください（手動、MR イベント、またはノート イベント リスナー/webhook 経由）
* `ANTHROPIC_API_KEY` またはクラウドプロバイダー変数が存在することを確認してください
* コメントに `@claude` が含まれていること（`/claude` ではなく）、メンション トリガーが設定されていることを確認してください

<h3 id="job-can’t-write-comments-or-open-mrs">
  ジョブがコメントを書き込めない、または MR を開けない
</h3>

* `CI_JOB_TOKEN` がプロジェクトに対して十分な権限を持っていることを確認するか、`api` スコープを持つプロジェクト アクセス トークンを使用してください
* `mcp__gitlab` ツールが `--allowedTools` で有効になっていることを確認してください
* ジョブが MR のコンテキストで実行されているか、`AI_FLOW_*` 変数を経由して十分なコンテキストを持っていることを確認してください

<h3 id="authentication-errors">
  認証エラー
</h3>

* **Claude API の場合**: `ANTHROPIC_API_KEY` が有効で期限切れでないことを確認してください
* **Amazon Bedrock または Google Cloud の Agent Platform の場合**: OIDC/WIF 設定、ロール偽装、シークレット名を確認してください。リージョンとモデルの可用性を確認してください

<h2 id="advanced-configuration">
  高度な設定
</h2>

<h3 id="common-parameters-and-variables">
  一般的なパラメータと変数
</h3>

これらの CLI フラグ、GitLab キーワード、および変数を使用して、ジョブ内の Claude Code 実行を制御します。

* `-p`: インラインで指示を提供します。例えば `claude -p "Review this MR"`
* `--max-turns`: バックアンドフォース反復の回数を制限します
* `timeout`: GitLab のジョブレベルの `timeout` キーワードで総ジョブ実行時間を制限します。例えば `timeout: 30m`
* `ANTHROPIC_API_KEY`: Claude API に必要です（Amazon Bedrock または Google Cloud の Agent Platform では使用されません）
* プロバイダー固有の環境: `AWS_REGION`、Google Cloud の Agent Platform のプロジェクト/リージョン変数

<Note>
  正確なフラグとパラメータは `@anthropic-ai/claude-code` のバージョンによって異なる場合があります。ジョブ内で `claude --help` を実行して、サポートされているオプションを確認してください。
</Note>

<h3 id="customizing-claude’s-behavior">
  Claude の動作をカスタマイズする
</h3>

Claude をガイドする方法は 2 つあります。

1. **CLAUDE.md**: コーディング標準、セキュリティ要件、およびプロジェクト規約を定義します。Claude は実行中にこれを読み、ルールに従います。
2. **カスタムプロンプト**: ジョブ内の `-p` を使用してタスク固有の指示を渡します。異なるジョブに異なるプロンプトを使用します（例えば、レビュー、実装、リファクタリング）。
