> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock 上の Claude Code

> Amazon Bedrock を通じた Claude Code の設定方法（セットアップ、IAM 設定、トラブルシューティングを含む）について学習します。

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  前提条件
</h2>

Claude Code を Amazon Bedrock で設定する前に、以下を確認してください。

* Amazon Bedrock アクセスが有効になっている AWS アカウント
* Amazon Bedrock で目的の Claude モデル（例：Claude Sonnet 4.6）へのアクセス
* AWS CLI がインストールされ、設定されていること（オプション - 認証情報を取得する別のメカニズムがない場合のみ必要）
* 適切な IAM 権限

Amazon Bedrock 認証情報を使用してサインインするには、以下の [Amazon Bedrock でサインイン](#sign-in-with-bedrock)に従ってください。チーム全体に Claude Code をデプロイするには、[手動でセットアップ](#set-up-manually)の手順を使用し、ロールアウト前に[モデルバージョンをピン留め](#4-pin-model-versions)してください。

<h2 id="sign-in-with-bedrock">
  Bedrock でサインイン
</h2>

AWS 認証情報を持っていて、Amazon Bedrock を通じて Claude Code の使用を開始したい場合、ログインウィザードがその手順を案内します。AWS 側の前提条件はアカウントごとに 1 回完了します。ウィザードが Claude Code 側を処理します。

<Steps>
  <Step title="AWS アカウントで Anthropic モデルを有効にする">
    [Amazon Bedrock コンソール](https://console.aws.amazon.com/bedrock/)で、モデルカタログを開き、Anthropic モデルを選択して、ユースケースフォームを送信します。送信直後にアクセスが許可されます。AWS Organizations については[ユースケースの詳細を送信](#1-submit-use-case-details)を、権限については[IAM 設定](#iam-configuration)を参照してください。
  </Step>

  <Step title="Claude Code を起動して Amazon Bedrock を選択する">
    `claude` を実行します。ログインプロンプトで、**3rd-party platform** を選択してから、**Amazon Bedrock** を選択します。既にサインインしていてチャットプロンプトが表示されている場合は、`/setup-bedrock` を実行してウィザードを開きます。`CLAUDE_CODE_USE_BEDROCK=1` が設定されるまで、Claude Code は[コマンドメニューからコマンドを非表示にします](/docs/ja/commands#how-the-command-menu-matches-what-you-type)。完全に入力してください。
  </Step>

  <Step title="ウィザードプロンプトに従う">
    AWS への認証方法を選択します。`~/.aws` ディレクトリから検出された AWS プロファイル、Amazon Bedrock API キー、アクセスキーとシークレット、または環境に既に存在する認証情報です。ウィザードはリージョンを要求し、アカウントが呼び出せる Claude モデルを確認し、それらをピン留めできます。結果は[ユーザー設定ファイル](/docs/ja/settings)の `env` ブロックに保存されるため、環境変数を自分でエクスポートする必要はありません。
  </Step>
</Steps>

サインイン後、いつでも `/setup-bedrock` を実行してウィザードを再度開き、認証情報、リージョン、またはモデルピンを変更できます。モデルピンステップは、現在ピン留めされているモデルから開始します。ウィザードは `~/.claude/settings.json` に書き込むか、[`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars#variables) が設定されている場合は `$CLAUDE_CONFIG_DIR/settings.json` に書き込みます。

<h2 id="set-up-manually">
  手動でセットアップする
</h2>

ウィザードの代わりに環境変数を使用して Amazon Bedrock を設定する場合（例えば CI やスクリプト化されたエンタープライズロールアウト）、以下の手順に従ってください。

<h3 id="1-submit-use-case-details">
  1. ユースケースの詳細を送信する
</h3>

Anthropic モデルを初めて呼び出す前に、ユースケースの詳細を送信してください。これは AWS アカウントごとに 1 回行います。

1. 以下に説明する適切な IAM 権限があることを確認してください
2. [Amazon Bedrock コンソール](https://console.aws.amazon.com/bedrock/)に移動します
3. **Model catalog** から Anthropic モデルを選択します
4. ユースケースフォームを完成させます。アクセスは送信直後に付与されます。

AWS Organizations を使用している場合、[`PutUseCaseForModelAccess` API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html) を使用して管理アカウントからフォームを 1 回送信できます。この呼び出しには `bedrock:PutUseCaseForModelAccess` IAM 権限が必要です。承認は自動的に子アカウントに拡張されます。

<h3 id="2-configure-aws-credentials">
  2. AWS 認証情報を設定する
</h3>

Claude Code は AWS SDK のデフォルト認証情報チェーンを使用します。以下のいずれかの方法を使用して認証情報を設定してください。

**オプション A: AWS CLI 設定**

```bash theme={null}
aws configure
```

**オプション B: 環境変数（アクセスキー）**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**オプション C: 環境変数（SSO プロファイル）**

これらのコマンドを実行する前に、`your-profile-name` を AWS プロファイルの名前に置き換えてください。

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code は、プロファイルの `sso_region` で指定された IAM Identity Center リージョンからロール認証情報をリクエストします。これは Amazon Bedrock を実行するリージョンと一致する必要はありません。v2.1.207 では、Amazon Bedrock リージョンが `sso_region` をオーバーライドしていたため、IAM Identity Center インスタンスが別のリージョンにあるプロファイルは `Session token not found or invalid` エラーで認証に失敗しました。

**オプション D: AWS Management Console 認証情報**

```bash theme={null}
aws login
```

`aws login` について[詳しく学ぶ](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html)。

**オプション E: Amazon Bedrock API キー**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Amazon Bedrock API キーは、完全な AWS 認証情報を必要としない、より簡単な認証方法を提供します。[Amazon Bedrock API キーについて詳しく学ぶ](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/)。

<h4 id="credential-caching-and-resolution-timeout">
  認証情報キャッシングと解決タイムアウト
</h4>

Claude Code は AWS デフォルト認証情報プロバイダーチェーンを 1 回解決し、解決された認証情報をメモリに保持します。認証情報の有効期限の 5 分前まで、または有効期限がない場合は 1 時間の間、それらを再利用するため、SSO でサポートされたプロファイルは認証情報の有効期間ごとに約 1 回 IAM Identity Center から認証情報をリクエストします。API からの認証情報エラーはキャッシュをクリアし、再試行は新しい認証情報を解決します。Claude Code v2.1.207 以降が必要です。

キャッシュは上記のすべての認証情報オプションをカバーしていますが、Amazon Bedrock API キーはプロバイダーチェーンを使用しないため除外されます。代わりにすべてのリクエストでチェーンを解決するには、[`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/ja/env-vars) を設定してください。

チェーンの各解決は 60 秒後にタイムアウトします。チェーン内のステップが停止した場合（例えば、受け取ることができない入力を待つ `credential_process` ヘルパー）、リクエストは [`AWS default-chain credential resolve timed out`](/docs/ja/errors#aws-default-chain-credential-resolve-timed-out) で失敗します。チェーンが正当に長い時間が必要なインタラクティブサインイン（`aws-vault` のようなラッパーを使用した MFA 付きブラウザベースの SSO など）を実行する場合、[`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ja/env-vars) でミリ秒単位で制限を引き上げてください。v2.1.207 より前では、停止した認証情報解決はリクエストを無期限に待機させていました。

Amazon Bedrock API キーで認証する場合を除き、[セットアップウィザード](#sign-in-with-bedrock)は認証情報を検証する際に行う各 AWS 呼び出しに同じ制限を適用し、各モデルチェック前の認証情報ルックアップにも適用します。認証情報検証中に、制限を超えるチェックは [`Timed out after 60s waiting for AWS`](/docs/ja/errors#bedrock-setup-verification-timed-out-waiting-for-aws) で失敗します。

<h4 id="advanced-credential-configuration">
  高度な認証情報設定
</h4>

Claude Code は AWS SSO および企業 ID プロバイダーの自動認証情報更新をサポートしています。これらの設定を Claude Code 設定ファイルに追加してください（ファイルの場所については [Settings](/docs/ja/settings) を参照）。

これら 2 つの設定には異なるトリガー条件があります。

* **`awsAuthRefresh`**: Claude Code がタイムスタンプに基づいてローカルで AWS 認証情報が期限切れであることを検出した場合、または API が認証情報エラーを返した場合にのみ実行され、更新された認証情報でリクエストを再試行します。
* **`awsCredentialExport`**: セッション開始時および各認証情報リロード時に実行されます。AWS デフォルト認証情報プロバイダーチェーン内の認証情報がまだ有効な場合でも実行されます。Amazon Bedrock アカウントがデフォルトプロバイダーチェーンが解決するものと異なるクロスアカウント認証情報を必要とする場合に使用してください。

`awsAuthRefresh` コマンドを実行する前に、Claude Code は STS `GetCallerIdentity` 呼び出しを行って、認証情報が実際に期限切れであることを確認し、認証情報がまだ機能している場合はコマンドをスキップします。Claude Code はこのチェックを [プロキシ設定](/docs/ja/network-config#proxy-configuration) を通じて送信し、`HTTPS_PROXY` と `NO_PROXY` を尊重します。v2.1.239 より前では、Claude Code はこのチェックを直接送信し、プロキシを通じてのみエグレスを許可するネットワークでスタートアップでハングしていました。

<h5 id="example-configuration">
  設定例
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  設定の説明
</h5>

**`awsAuthRefresh`**: `.aws` ディレクトリを変更するコマンド（認証情報、SSO キャッシュ、または設定ファイルの更新など）に使用してください。コマンドの出力はユーザーに表示されますが、インタラクティブ入力はサポートされていません。これは CLI が URL またはコードを表示し、ブラウザで認証を完了するブラウザベースの SSO フローに適しています。

**`awsCredentialExport`**: `.aws` を変更できず、認証情報を直接返す必要がある場合にのみ使用してください。出力はサイレントにキャプチャされ、ユーザーに表示されません。コマンドは次の形式で JSON を出力する必要があります。

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

`aws configure export-credentials --format process` からのフラット出力も受け入れられます。同じキーが `Credentials` の下にネストされるのではなく、トップレベルにあります。

`Expiration` はオプションです。コマンドが有効な ISO 8601 `Expiration` を返すと、Claude Code はその時刻の 5 分前まで認証情報をキャッシュします。それがない場合、認証情報は 1 時間キャッシュされます。

`awsCredentialExport` を `awsAuthRefresh` なしで設定する場合、Claude Code はエクスポートされた認証情報を直接使用し、スタートアップで AWS デフォルト認証情報プロバイダーチェーンを再解決しません。Claude Code v2.1.206 以降が必要です。

<h3 id="3-configure-claude-code">
  3. Claude Code を設定する
</h3>

Amazon Bedrock を有効にするには、以下の環境変数を設定してください。

```bash theme={null}
# Bedrock 統合を有効にする
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # AWS プロファイルがすでにリージョンを設定している場合はオプション

# オプション: 小型/高速モデル（Bedrock と Mantle）の AWS リージョンをオーバーライドします。
# Bedrock では ANTHROPIC_DEFAULT_HAIKU_MODEL
# または非推奨の ANTHROPIC_SMALL_FAST_MODEL が設定されていない場合、効果がありません。
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# オプション: カスタムエンドポイントまたはゲートウェイ用に Bedrock エンドポイント URL をオーバーライドします
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

Claude Code で Amazon Bedrock を有効にする場合、以下の点に注意してください。

* `AWS_REGION` を設定する必要があるのは、AWS プロファイルのリージョンをオーバーライドする場合、またはプロファイルにリージョンがない場合のみです。Claude Code はこの順序でリージョンを解決します。

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * AWS 共有認証情報ファイルから最初に読み込まれ、次に共有設定ファイルから読み込まれるアクティブな AWS プロファイルに設定された `region`（AWS SDK の優先順位と一致）
  * `us-east-1`

  これらのソースからの値がリージョン名のような形をしていない場合、Claude Code はそれを未設定として扱い、順序を続けます。例えば、Claude Code はスラッシュ、ドット、またはスペースを含む値を未設定として扱います。

  アクティブなプロファイルは、設定されている場合は `AWS_PROFILE`、そうでない場合は `default` です。`AWS_SHARED_CREDENTIALS_FILE` または `AWS_CONFIG_FILE` を設定して、デフォルト以外のファイルパスを指定してください。

  `/status` を実行して、解決されたリージョンを確認してください。リージョンが AWS 設定ファイルまたはデフォルトフォールバックから来た場合、Claude Code は `/status` 出力でソースも記載します。
* Amazon Bedrock を使用する場合、認証は AWS 認証情報を通じて処理されるため、`/logout` コマンドは利用できません。
* WebSearch ツールは Amazon Bedrock では利用できません。[WebSearch ツールの動作](/docs/ja/tools-reference#websearch-tool-behavior)を参照してください。
* `AWS_PROFILE` のような他のプロセスにリークしたくない環境変数に設定ファイルを使用できます。詳細については [Settings](/docs/ja/settings) を参照してください。

<h3 id="4-pin-model-versions">
  4. モデルバージョンをピンする
</h3>

<Warning>
  複数のユーザーにデプロイする場合、特定のモデルバージョンをピンしてください。ピンしない場合、`sonnet` や `opus` などのモデルエイリアスは Claude Code の Amazon Bedrock 用の組み込みデフォルトに解決されます。これは最新リリースより遅れる可能性があり、アカウントでまだ利用できない場合があります。Claude Code はスタートアップ時にデフォルトが利用できない場合、[以前のモデルまたは下位層のモデルにフォールバック](#startup-model-checks)しますが、ピンすることでユーザーが新しいモデルに移行するタイミングを制御できます。
</Warning>

これらの環境変数を特定の Amazon Bedrock モデル ID に設定してください。

`ANTHROPIC_DEFAULT_OPUS_MODEL` がない場合、Amazon Bedrock の `opus` エイリアスは Opus 5.5 に解決され、`ANTHROPIC_DEFAULT_SONNET_MODEL` がない場合、`sonnet` エイリアスは Sonnet 4.5 に解決されます。この例では各エイリアスを特定のバージョンにピンしています。

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

これらの ID は `us.` クロスリージョン推論プロファイルプレフィックスを使用しています。別のリージョンプレフィックスまたはアプリケーション推論プロファイルを使用する場合は、それに応じて調整してください。AWS GovCloud リージョンでは、`us-gov.` プレフィックスを使用してください。

組み込みデフォルトモデルを保持し、優先プレフィックスのみを変更するには、ピンの代わりに [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes) を設定してください。`opus` エイリアスが解決する内容の違いを示します。

| 設定内容                                                          | `opus` エイリアスが解決する内容                                  |
| :------------------------------------------------------------ | :--------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`、ピンした正確な ID            |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu`                          | `eu.anthropic.claude-opus-5-5`、優先プレフィックス付きの組み込みデフォルト |

現在および従来のモデル ID については、[Models overview](https://platform.claude.com/docs/en/about-claude/models/overview) を参照してください。ピン環境変数の完全なリストについては、[Model configuration](/docs/ja/model-config#pin-models-for-third-party-deployments) を参照してください。

ピン変数が設定されていない場合、Claude Code はこれらのデフォルトモデルを使用します。

| モデルタイプ   | デフォルトモデル                                                                    |
| :------- | :-------------------------------------------------------------------------- |
| プライマリモデル | Opus 5.5、例えば `us-*` リージョンの `us.anthropic.claude-opus-5-5`                   |
| 小型/高速モデル | Sonnet 4.5、例えば `us-*` リージョンの `us.anthropic.claude-sonnet-4-5-20250929-v1:0` |

セッションタイトル生成などのバックグラウンドタスクは、小型/高速モデル（通常は Haiku クラスモデル）を使用します。Amazon Bedrock では、すべてのアカウントまたはリージョンで Haiku が有効になっていない可能性があるため、Claude Code はバックグラウンドタスクにデフォルト Sonnet モデルを使用します。2 つの選択がどのモデルがそれらを実行するかを変更します。

* `--model`、`ANTHROPIC_MODEL`、または `model` 設定でプライマリモデルを選択する場合、バックグラウンドタスクはそのモデルを使用します。Claude Code が [`ANTHROPIC_DEFAULT_MODEL`](/docs/ja/model-config#set-a-default-model-for-new-sessions) で設定したモデルでセッションを開始する場合、バックグラウンドタスクもそのモデルを使用します。`ANTHROPIC_DEFAULT_SONNET_MODEL` なしで `ANTHROPIC_DEFAULT_OPUS_MODEL` を設定することも、組み込み Sonnet モデルが独自の Opus を操舵するアカウントで有効になっていない可能性があるため、選択としてカウントされます。
* バックグラウンドタスクに Haiku を使用するには、`ANTHROPIC_DEFAULT_HAIKU_MODEL` をアカウントで利用可能なモデル ID に設定してください。

<Warning>
  Opus モデルは Sonnet モデルより高いトークンあたりの価格を持つため、プライマリモデルをピンしないデプロイメントは v2.1.207 以降に更新されると Opus レートで請求されます。Sonnet 4.5 をプライマリモデルとして保持するには、`ANTHROPIC_MODEL` をその完全なモデル ID に設定してください。`ANTHROPIC_DEFAULT_SONNET_MODEL` で操舵され、`ANTHROPIC_DEFAULT_OPUS_MODEL` を設定しないデプロイメントは、操舵された Sonnet モデルをデフォルトとして保持します。
</Warning>

v2.1.280 より前では、Amazon Bedrock のプライマリモデルは Opus 5 にデフォルト設定され、`opus` エイリアスは v2.1.219 から Opus 5 に解決されました。v2.1.207 から v2.1.218 では、Amazon Bedrock のプライマリモデルは Opus 4.8 にデフォルト設定され、`opus` エイリアスは Opus 4.8 に解決されました。v2.1.207 より前では、プライマリモデルは Sonnet 4.5 にデフォルト設定され、`opus` エイリアスは Opus 4.6 に解決され、バックグラウンドタスクは常にプライマリモデルを使用していました。

モデルをさらにカスタマイズするには、以下のいずれかの方法を使用してください。

```bash theme={null}
# 推論プロファイル ID を使用する
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# アプリケーション推論プロファイル ARN を使用する
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# オプション: 必要に応じてプロンプトキャッシングを無効にする
# export DISABLE_PROMPT_CACHING=1

# オプション: デフォルトの 5 分の代わりに 1 時間のプロンプトキャッシュ TTL をリクエストする
# export ENABLE_PROMPT_CACHING_1H=1
```

1 時間のキャッシュ TTL はデフォルトの 5 分より高いレートで請求されます。[キャッシュの有効期間](/docs/ja/prompt-caching#cache-lifetime)を参照してください。メイン会話と Claude Code が外部で行うリクエストに異なる TTL を設定するには、[TTL を自分で選択してください](/docs/ja/prompt-caching#choose-the-ttl-yourself)。

<Note>プロンプトキャッシングはすべての Amazon Bedrock リージョンで利用できない場合があります。キャッシュトークンカウントがゼロのままの場合、Amazon Bedrock ドキュメントの [サポートされているモデル、リージョン、および制限](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)を確認してください。</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  各モデルバージョンを推論プロファイルにマップする
</h4>

`ANTHROPIC_DEFAULT_*_MODEL` 環境変数は、モデルファミリーごとに 1 つの推論プロファイルを設定します。組織が同じファミリーの複数のバージョンを `/model` ピッカーで公開し、それぞれを独自のアプリケーション推論プロファイル ARN にルーティングする必要がある場合は、[設定ファイル](/docs/ja/settings#where-settings-live)の `modelOverrides` 設定を使用してください。

この例は 4 つの Opus バージョンを異なる ARN にマップするため、ユーザーは組織の推論プロファイルをバイパスすることなくそれらを切り替えることができます。

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

ユーザーが `/model` でこれらのバージョンのいずれかを選択すると、Claude Code はマップされた ARN で Amazon Bedrock を呼び出します。同じマッピングは `--model` または `ANTHROPIC_MODEL` を通じて Anthropic モデル ID を直接渡す場合に適用されます。オーバーライドのないバージョンは、組み込み Amazon Bedrock モデル ID またはスタートアップで検出された一致する推論プロファイルにフォールバックします。v2.1.200 より前では、`--model` と `ANTHROPIC_MODEL` の値はオーバーライドマップを通さずに Amazon Bedrock に到達していました。オーバーライドが `availableModels` および他のモデル設定とどのように相互作用するかについては、[Override model IDs per version](/docs/ja/model-config#override-model-ids-per-version) を参照してください。

<h2 id="startup-model-checks">
  スタートアップモデルチェック
</h2>

Claude Code が Amazon Bedrock で設定されて起動する場合、使用予定のモデルがアカウントでアクセス可能であることを確認します。

現在の Claude Code デフォルトより古いモデルバージョンをピン留めしており、アカウントが新しいバージョンを呼び出せる場合、Claude Code はピンを更新するよう促します。受け入れると、新しいモデル ID が[ユーザー設定ファイル](/docs/ja/settings)に書き込まれ、Claude Code が再起動します。拒否すると、次のデフォルトバージョン変更まで記憶されます。[アプリケーション推論プロファイル ARN](#map-each-model-version-to-an-inference-profile) を指す pin は、管理者によって管理されるため、スキップされます。

モデルをピン留めしておらず、現在のデフォルトがアカウントで利用できない場合、Claude Code は現在のセッションにフォールバックし、通知を表示します。デフォルトモデルの以前のバージョンを最初に試し、デフォルトが Opus モデルで Opus バージョンが利用できない場合、デフォルト Sonnet モデルにフォールバックします。フォールバックは保持されません。Amazon Bedrock アカウントで新しいモデルを有効にするか、[バージョンをピン留めして](#4-pin-model-versions)選択を永続的にしてください。

特定の Sonnet または Opus バージョンでセッションを開始する場合（例えば `--model`、`ANTHROPIC_MODEL`、または[`model` 設定](/docs/ja/settings-reference#model)を使用）、そのバージョンは一致する `sonnet` または `opus` エイリアスのセッションのピン留めされたデフォルトとして機能します。Claude Code は、設定したモデルが置き換える組み込みデフォルトの可用性チェックをスキップし、設定したモデルで起動し、フォールバック通知はありません。

`opus` などのモデルエイリアスはピンとして機能せず、Claude Code が認識しないモデル ID（アプリケーション推論プロファイル ARN など）も同様です。

<h2 id="cross-region-inference-profile-prefixes">
  クロスリージョン推論プロファイルプレフィックス
</h2>

Amazon Bedrock の [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) では、Claude Code は組み込みのデフォルトモデルを [クロスリージョン推論プロファイル](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html) ID に解決します。独自の推論プロファイルを通じてモデルバージョンをルーティングするには、[各モデルバージョンを推論プロファイルにマップする](#map-each-model-version-to-an-inference-profile) を参照してください。このテーブルは、解決された各 AWS リージョンに対して Claude Code が優先するプレフィックスを示しています。

| AWS リージョン                 | プレフィックス   |
| :------------------------ | :-------- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*`                    | `us.`     |
| `eu-*`                    | `eu.`     |
| `ap-*`                    | `apac.`   |
| その他すべてのリージョン              | `global.` |

`ANTHROPIC_BEDROCK_REGION_PREFIX` を設定して、Claude Code が最初に試すプレフィックスを選択します。Claude Code がプロファイルの可用性を確認でき、モデルに一致するプロファイルが見つからない場合、以下の解決順序に従ってフォールバックします。有効な値は `us`、`eu`、`apac`、`jp`、`au`、および `global` です。たとえば、アカウントに `global.` プロファイルが有効になっているが、Claude Code が AWS リージョンから地理的に固有のプロファイルを導出する場合は、`global` に設定します。Claude Code v2.1.224 以降が必要です。

この例は、デフォルトモデルを `global.` プロファイルを通じてルーティングします。

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# us-* リージョンでは、プライマリモデルは
# us.anthropic.claude-opus-5-5 の代わりに
# global.anthropic.claude-opus-5-5 に解決されます
```

優先プレフィックスは保証ではなく、リージョンから来ているか変数から来ているかに関わらず、単なる優先度です。Claude Code がそれを適用する方法は、アカウント内のプロファイル可用性を確認できるかどうかによって異なります。

* Claude Code がアカウント内の [推論プロファイルをリストできる](#iam-configuration) 場合、各モデルを次の順序で解決します。
  1. 優先プレフィックスを持つプロファイル。
  2. そのプレフィックスを持つプロファイルがないモデルの場合、一致するプロファイル。
  3. 一致するプロファイルがないモデルの場合、優先プレフィックスを持つ組み込みモデル ID。Claude Code はこのステップで可用性を確認せずにこの ID を適用します。[スタートアップモデルチェック](#startup-model-checks) はセッションのデフォルトモデルをカバーします。
* プロファイル検出が利用できない場合、Claude Code は可用性を確認せずにプレフィックスを適用します。アカウントにそのプレフィックスを持つ推論プロファイルが有効になっていない場合、リクエストは 400 エラーで失敗します。

Claude Code は、自分で設定した Amazon Bedrock 推論プロファイル ID または ARN、または [`modelOverrides`](#map-each-model-version-to-an-inference-profile) 値を書き直しません。Anthropic 形式のモデル ID は [/model ピッカーと同じマッピング](#map-each-model-version-to-an-inference-profile) を通じて解決されます。Claude Code は 2 つのケースで変数を無視します。

* AWS GovCloud リージョンでは、Claude Code は常に `us-gov.` を使用します。これは GovCloud パーティション内でルーティングする唯一のプレフィックスです。
* 有効な値の 1 つではない値を設定した場合、Claude Code はリージョン導出の優先プレフィックスにフォールバックします。

<h2 id="iam-configuration">
  IAM 設定
</h2>

Claude Code に必要な権限を持つ IAM ポリシーを作成します。

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

より制限的な権限の場合は、リソースを特定の推論プロファイル ARN に制限できます。

`bedrock:GetInferenceProfile` により、Claude Code は[アプリケーション推論プロファイル ARN](#map-each-model-version-to-an-inference-profile)をそのバッキング基盤モデルに解決でき、そのモデルに対して正しいリクエスト形状を選択するために使用されます。

トークンにこの権限がない場合、Claude Code は代替形状で 1 回再試行することで自動的に復旧するため、リクエストは成功しますが、新しいモデルが追加されるたびに追加のラウンドトリップが発生します。権限を付与することで再試行を回避できます。これは `AWS_BEARER_TOKEN_BEDROCK` デプロイメントに最も頻繁に適用され、トークンのポリシーは通常、完全な IAM ロールよりも狭くなります。

詳細については、[Amazon Bedrock IAM ドキュメント](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html)を参照してください。

<Note>
  コスト追跡とアクセス制御を簡素化するために、Claude Code 用の専用 AWS アカウントを作成してください。
</Note>

<h2 id="1m-token-context-window">
  1M トークンコンテキストウィンドウ
</h2>

Claude Sonnet 5、Opus 4.6 以降、および Sonnet 4.6 は、Amazon Bedrock で [1M トークンコンテキストウィンドウ](https://platform.claude.com/docs/ja/build-with-claude/context-windows#context-window-sizes-by-model)をサポートしています。Sonnet 5 は Invoke API と [Mantle エンドポイント](#use-the-mantle-endpoint)の両方で常に 1M ウィンドウで実行され、選択する `[1m]` バリアントはありません。Invoke API 上の他のモデルについては、Claude Code は 1M モデルバリアントを選択すると、拡張コンテキストウィンドウを自動的に有効にします。

[セットアップウィザード](#sign-in-with-bedrock)は、モデルをピン留めするときに 1M コンテキストオプションを提供します。手動でピン留めされたモデルの代わりに有効にするには、モデル ID に `[1m]` を追加します。詳細については、[サードパーティデプロイメント用のモデルをピン留めする](/docs/ja/model-config#pin-models-for-third-party-deployments)を参照してください。

<h2 id="service-tiers">
  サービスティア
</h2>

[Amazon Bedrock サービスティア](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html)を使用すると、コストとレイテンシーのトレードオフを行うことができます。`ANTHROPIC_BEDROCK_SERVICE_TIER` を `default`、`flex`、または `priority` に設定します。

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code は、各リクエストで `X-Amzn-Bedrock-Service-Tier` ヘッダーとしてこれを送信します。ティアの可用性はモデルとリージョンによって異なります。予約容量は、この設定の代わりに[プロビジョニングされたスループット](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) ARN をモデル ID として使用します。

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html)を使用すると、Claude Code のコンテンツフィルタリングを実装できます。[Amazon Bedrock コンソール](https://console.aws.amazon.com/bedrock/)で Guardrail を作成し、バージョンを公開してから、Guardrail ヘッダーを [settings file](/docs/ja/settings) に追加します。クロスリージョン推論プロファイルを使用している場合は、Guardrail でクロスリージョン推論を有効にしてください。

設定例：

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

組織が [Claude apps gateway](/docs/ja/claude-apps-gateway) ポリシーを通じて guardrail ヘッダーを配信する場合、それらは [承認が必要な設定](/docs/ja/server-managed-settings#environment-variables-and-the-approval-dialog)としてカウントされます。

<h2 id="use-the-mantle-endpoint">
  Mantle エンドポイントを使用する
</h2>

Mantle は、Bedrock Invoke API ではなく、ネイティブ Anthropic API シェイプを通じて Claude モデルを提供する Amazon Bedrock エンドポイントです。同じ [AWS 認証情報](#2-configure-aws-credentials) と [`awsAuthRefresh` 設定](#advanced-credential-configuration) を使用します。

Mantle は `bedrock-mantle:` プレフィックスの下に独自の IAM アクションを持つため、[IAM 設定](#iam-configuration) の `bedrock:` アクションはこれをカバーしていません。推論用に `bedrock-mantle:CreateInference` と、トークンカウント用に `bedrock-mantle:CountTokens` を IAM アイデンティティに付与します。AWS ドキュメントの [推論リクエストの実行](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html) と [トークンのカウント](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html)、および [サービス認可リファレンス](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html) を参照して、すべての Mantle アクションを確認してください。

<h3 id="enable-mantle">
  Mantle を有効にする
</h3>

AWS 認証情報が既に設定されている場合、`CLAUDE_CODE_USE_MANTLE` を設定して、リクエストを Mantle エンドポイントにルーティングします。

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code は AWS リージョンからエンドポイント URL を構築します。リージョンは [上記の Amazon Bedrock](#3-configure-claude-code) と同じ優先順位で解決されます。カスタムエンドポイントまたはゲートウェイの URL をオーバーライドするには、`ANTHROPIC_BEDROCK_MANTLE_BASE_URL` を設定します。

Claude Code 内で `/status` を実行して確認します。Mantle がアクティブな場合、プロバイダー行は `Amazon Bedrock (Mantle)` を表示します。

<h3 id="select-a-mantle-model">
  Mantle モデルを選択する
</h3>

Mantle は `anthropic.` で始まり、バージョンサフィックスのないモデル ID を使用します。例えば `anthropic.claude-sonnet-5` または `anthropic.claude-haiku-4-5` です。アカウントで利用可能なモデルは、組織に付与されたものに依存します。追加のモデル ID は AWS からのオンボーディング資料に記載されています。AWS アカウントチームに連絡して、許可リストされたモデルへのアクセスをリクエストしてください。

`--model` フラグまたは Claude Code 内の `/model` でモデルを設定します。

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Mantle を Invoke API と並行して実行する
</h3>

Mantle で利用可能なモデルは、今日使用するすべてのモデルを含まない場合があります。`CLAUDE_CODE_USE_BEDROCK` と `CLAUDE_CODE_USE_MANTLE` の両方を設定すると、Claude Code は同じセッションから両方のエンドポイントを呼び出せます。Mantle 形式に一致するモデル ID は Mantle にルーティングされ、他のすべてのモデル ID は Amazon Bedrock Invoke API に移動します。

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Mantle モデルを `/model` ピッカーに表示するには、[settings file](/docs/ja/settings) の `availableModels` にその ID をリストします。この設定はピッカーをリストされたエントリに制限するため、保持したいバージョンのバージョンプレフィックスまたは完全な ID もリストします。Mantle ID と `haiku` エイリアスは同じモデルファミリーに解決されるため、マージはより具体的なエントリのみを保持します。[Merge behavior](/docs/ja/model-config#merge-behavior) を参照してください。

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

`anthropic.` プレフィックス付きのエントリはカスタムピッカーオプションとして追加され、Mantle にルーティングされます。`anthropic.claude-haiku-4-5` をアカウントに付与されたモデル ID に置き換えます。`availableModels` が他のモデル設定とどのように相互作用するかについては、[Restrict model selection](/docs/ja/model-config#restrict-model-selection) を参照してください。

両方のプロバイダーがアクティブな場合、`/status` は `Amazon Bedrock + Amazon Bedrock (Mantle)` を表示します。

<h3 id="route-mantle-through-a-gateway">
  Mantle をゲートウェイ経由でルーティングする
</h3>

組織がモデルトラフィックを集中化された [LLM gateway](/docs/ja/llm-gateway) を通じてルーティングし、AWS 認証情報をサーバー側に注入する場合、クライアント側認証を無効にして、Claude Code が SigV4 署名または `x-api-key` ヘッダーなしでリクエストを送信するようにします。

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Mantle 環境変数
</h3>

これらの変数は Mantle エンドポイントに固有です。完全なリストについては、[Environment variables](/docs/ja/env-vars) を参照してください。

| 変数                                      | 目的                                           |
| :-------------------------------------- | :------------------------------------------- |
| `CLAUDE_CODE_USE_MANTLE`                | Mantle エンドポイントを有効にします。`1` または `true` に設定します。 |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | デフォルト Mantle エンドポイント URL をオーバーライド            |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | プロキシセットアップのクライアント側認証をスキップ                    |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Haiku クラスモデルの AWS リージョンをオーバーライド（Bedrock と共有） |

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  SSO と企業プロキシでの認証ループ
</h3>

AWS SSO を使用する場合にブラウザタブが繰り返し生成される場合は、[settings file](/docs/ja/settings) から `awsAuthRefresh` 設定を削除してください。これは、企業 VPN または TLS 検査プロキシが SSO ブラウザフローを中断した場合に発生する可能性があります。Claude Code は中断された接続を認証失敗として扱い、`awsAuthRefresh` を再実行し、無限ループします。

ネットワーク環境が自動ブラウザベースの SSO フローに干渉する場合は、`awsAuthRefresh` に依存する代わりに、Claude Code を開始する前に手動で `aws sso login` を使用してください。

<h3 id="certificate-errors-behind-a-tls-inspecting-proxy">
  TLS 検査プロキシの背後での証明書エラー
</h3>

Claude Code は、[CA certificate store](/docs/ja/network-config#ca-certificate-store) 設定を AWS へのリクエストに適用します。これには以下が含まれます：

* モデル検出
* トークンカウント
* AWS 認証情報を解決する STS および SSO ロール認証情報呼び出し
* [setup wizard](#sign-in-with-bedrock) の認証情報検証とモデルチェック

これらのリクエストについては、OS トラストストアまたは `NODE_EXTRA_CA_CERTS` バンドル内の企業ルート証明書には、Amazon Bedrock 固有のセットアップは必要ありません。

v2.1.260 より前では、Claude Code は設定されたプロキシを通過するリクエストにのみ CA 設定を適用し、直接接続では実行時のデフォルト証明書ストアのみを信頼していました。

v2.1.261 より前では、**Use credentials already in my environment** オプションを使用した setup wizard のモデルチェックの背後での認証情報ルックアップは、実行時のデフォルト証明書ストアのみを信頼していました。ルート証明書が OS ストアにのみある TLS 検査プロキシの背後では、影響を受けるリクエストは `unable to get local issuer certificate` で失敗するか、ウィザードはモデルを `unreachable` として表示していましたが、推論リクエストは成功していました。v2.1.261 以降に更新してください。

<h3 id="region-issues">
  リージョンの問題
</h3>

リージョンの問題が発生した場合：

* モデルの可用性を確認：`aws bedrock list-inference-profiles --region your-region`
* サポートされているリージョンに切り替え：`export AWS_REGION=us-east-1`
* クロスリージョンアクセスに推論プロファイルの使用を検討

「on-demand throughput isn't supported」エラーが表示される場合：

* モデルを [inference profile](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html) ID として指定します

Claude Code は Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) を使用し、Converse API はサポートしていません。

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  ゲートウェイまたはプロキシの背後でのストリーミングエラー
</h3>

Amazon Bedrock は `InvokeModelWithResponseStream` レスポンスをバイナリイベントストリーム形式でストリーミングし、ヘッダー `Content-Type: application/vnd.amazon.eventstream` を含みます。Claude Code と Amazon Bedrock の間のゲートウェイまたはプロキシは、Amazon Bedrock が送信したレスポンスボディとそのヘッダー（`Content-Type` を含む）を変更されずに転送する必要があります。

ゲートウェイが `Content-Type` を別の値に書き換える場合、Claude Code は `Bedrock streaming response has content-type` で始まるエラーでレスポンスを拒否し、受け取った値を名前付けます。一般的な書き換えは `text/event-stream` で、ストリームをサーバー送信イベントとして再発行する統合からのものです。

ゲートウェイがヘッダーをドロップまたは空白にする代わりに、Claude Code は本体が Amazon Bedrock のイベントストリームであると仮定してデコードするため、ゲートウェイが変更されずに通した本体はストリーミングを続けます。

ヘッダーをドロップするゲートウェイがストリームをサーバー送信イベントとして再発行する場合、Claude Code は本体をデコードできず、すべてのターンで遅いノンストリーミングパスにフォールバックします。各レスポンスはストリーミングされるのではなく、完全になった後にのみ表示されます。その場合、[`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](/docs/ja/env-vars) を設定して、Claude Code が本体をサーバー送信イベントとして読み取るようにしてください。

エラーまたはフォールバックを修正するには、ゲートウェイを設定して `InvokeModelWithResponseStream` レスポンスボディとその `Content-Type` ヘッダーを変更されずに転送するようにしてください。

ストリームをサーバー送信イベントに変換するゲートウェイは、Amazon Bedrock API を提供していません。Anthropic Messages API リクエストも受け入れる場合は、`CLAUDE_CODE_USE_BEDROCK` の代わりに `ANTHROPIC_BASE_URL` を使用して [LLM gateway](/docs/ja/llm-gateway-connect) として接続してください。

<h3 id="zero-token-counts-in-/context">
  /context でのゼロトークンカウント
</h3>

`/context` コマンドは、ツールスキーマを Amazon Bedrock count-tokens API に送信することで、各ツールグループのトークンをカウントします。Claude Code v2.1.196 より前のバージョンでは、スキーマが count-tokens API が受け入れないフィールドを含んでいたため、Amazon Bedrock がそのリクエストを拒否し、すべてのツールグループが 0 トークンを表示していました。メッセージやメモリファイルなど、内訳の他の行は影響を受けません。

v2.1.196 以降に更新してください。

<h3 id="mantle-endpoint-errors">
  Mantle エンドポイントエラー
</h3>

`CLAUDE_CODE_USE_MANTLE` を設定した後、`/status` が `Amazon Bedrock (Mantle)` を表示しない場合、変数がプロセスに到達していません。Claude Code を起動したシェルでエクスポートされているか、[settings file](/docs/ja/settings) の `env` ブロックで設定されていることを確認してください。

Mantle エンドポイントからの `403` が何を意味するかは、エラーが IAM アクションを名前付けるかどうかによって異なります：

* エラーが `bedrock-mantle:` アクションを名前付ける場合は、IAM アイデンティティにそのアクションを付与してください。
* エラーがアクションを名前付けず、認証情報が有効な場合は、AWS アカウントがリクエストしたモデルへのアクセスを許可されていません。AWS アカウントチームに連絡してアクセスをリクエストしてください。

モデル ID を名前付ける `400` は、そのモデルが Mantle で提供されていないことを意味します。Mantle は標準 Amazon Bedrock カタログとは別の独自のモデルラインアップを持っているため、`us.anthropic.claude-sonnet-4-6` などの推論プロファイル ID は機能しません。Mantle 形式の ID を使用するか、[両方のエンドポイントを有効にして](#run-mantle-alongside-the-invoke-api)、Claude Code が各リクエストをモデルが利用可能なエンドポイントにルーティングするようにしてください。

<h2 id="additional-resources">
  追加リソース
</h2>

* [Amazon Bedrock ドキュメント](https://docs.aws.amazon.com/bedrock/)
* [Amazon Bedrock 料金](https://aws.amazon.com/bedrock/pricing/)
* [Amazon Bedrock 推論プロファイル](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Amazon Bedrock トークンバーンダウンとクォータ](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code on Amazon Bedrock: Quick Setup Guide](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Claude Code Monitoring Implementation（Amazon Bedrock）](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
