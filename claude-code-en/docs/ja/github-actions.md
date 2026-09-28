> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> @claude メンションに応答し、タスクを自動化し、イシューをプルリクエストに変換するために GitHub Actions ワークフロー内で Claude Code を実行します

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) は、リポジトリのワークフロー内で Claude Code を実行する GitHub Action です。プルリクエストまたはイシューコメントで `@claude` とメンションすると、Claude はコードを分析し、変更を実装し、コミットをプッシュします。また、Claude Code GitHub Action にプロンプトを与えて、任意の GitHub イベントで自動的に実行することもできます。イシューをプルリクエストに変換したり、コメントからバグを修正したり、繰り返されるタスクを自動化したりするために使用します。

複数の製品が Claude Code という名前を共有しています。このページは、リポジトリ内のワークフローファイルで設定する `claude-code-action` ワークフロー統合について説明しています。関連製品については、以下を参照してください。

* [Code Review](/docs/ja/code-review)：ワークフローを記述せずに、すべてのプルリクエストで自動レビュー
* [Claude Code in the cloud](/docs/ja/claude-code-on-the-web)：クラウドインフラストラクチャで実行される Claude Code セッション（マシンではなく）
* [Claude Agent SDK](/docs/ja/agent-sdk/overview)：GitHub Actions 外のカスタム自動化。Claude Code GitHub Action は SDK の上に構築されています
* [GitHub Enterprise Server](/docs/ja/github-enterprise-server)：自己ホスト型 GitHub での Claude Code

<h2 id="setup">
  セットアップ
</h2>

Claude Code GitHub Action は 2 つの方法のいずれかでセットアップできます。

* **クイックセットアップ**：Claude Code から `/install-github-app` を実行します。Claude Code は GitHub App をインストールし、認証シークレットを追加し、ワークフロープルリクエストを準備します
* **手動セットアップ**：アプリをインストールし、シークレットを追加し、ワークフローファイルをリポジトリにコピーします。Claude Code をローカルで実行しない場合、コマンドが失敗した場合、またはワークフローファイルを完全に制御したい場合は、このパスを使用します

どちらのパスでも、リポジトリへの管理者アクセスが必要です。

<h3 id="quick-setup">
  クイックセットアップ
</h3>

`/install-github-app` は github.com リポジトリでのみ機能します。リポジトリの git リモートが gitlab.com または bitbucket.org にある場合、コマンドは通知を出力して終了し、セットアップを開始しません。GitLab パイプラインから Claude Code を実行するには、[Claude Code GitLab CI/CD](/docs/ja/gitlab-ci-cd) を参照してください。

開始する前に、[GitHub CLI](https://cli.github.com) をインストールし、`gh auth login` で認証します。Claude Code はそれをチェックし、不足している場合は警告します。

接続したいリポジトリで `claude` を開き、`/install-github-app` を実行して、プロンプトに従います。Claude Code は Claude GitHub App をインストールし、ワークフロー用の認証シークレットをセットアップします。

* Claude Code に既に API キーがある場合、そのキーを再利用し、リポジトリに既に設定されている既存の `ANTHROPIC_API_KEY` シークレットを保持することを提案します
* それ以外の場合は、Claude サブスクリプションで長期トークンを作成するか、API キーを貼り付けるかを選択します

Claude Code は認証情報をリポジトリシークレットとして保存します。API キーの場合は `ANTHROPIC_API_KEY`、サブスクリプショントークンの場合は `CLAUDE_CODE_OAUTH_TOKEN` という名前です。

Claude Code はその後、選択したワークフローファイルを含むブランチをプッシュし、既にそのシークレットを使用するように設定され、ブラウザで GitHub を開いてプルリクエストを作成する準備ができています。そのプルリクエストを作成してマージすると、リポジトリで `@claude` が機能します。

レビューワークフローを選択した場合、Claude は各レビューをプルリクエスト自体に投稿します。見つかった各イシューのインラインコメントとして、または見つからない場合は 1 つの概要コメントとして投稿します。Claude は下書きなどの一部のプルリクエストをスキップします。[レビューワークフロー例](#run-a-skill)は同じスキルを使用し、それらをリストします。v2.1.229 より前では、Claude はレビューをワークフロー実行ログにのみ書き込みました。

以前のバージョンが生成したレビューワークフローを更新するには、以下のいずれかを実行します。

* `/install-github-app` を再度実行します。リポジトリに既に `claude.yml` がある場合は、**Update workflow file with latest version** を選択します。Claude Code は新しいブランチにワークフローファイルの新しいコピーをプッシュし、最初のインストールと同じようにプルリクエストを開きます。
* [レビューワークフロー例](#run-a-skill)から `--comment` 引数と `claude_args` 行をチェックインファイルに自分で追加します。これにより、他の編集は保持されます。

GitHub App をインストールした後、Claude Code は GitHub Actions セットアップを続行するかどうかを尋ねます。**Skip for now** を選択して、GitHub App のインストールのみで停止します。後で `/install-github-app` を再度実行してワークフローとシークレットのステップを完了します。

<Note>
  * GitHub App をインストールすると、複数の権限を付与します。完全なセットについては [GitHub App 権限](#github-app-permissions) を参照してください
  * クイックセットアップは Claude API と Claude サブスクリプションで機能します。Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry を使用する場合は、[クラウドプロバイダーで Claude Code GitHub Actions を使用する](/docs/ja/github-actions-cloud-providers) を参照してください
</Note>

<h3 id="manual-setup">
  手動セットアップ
</h3>

`/install-github-app` を実行せずに Claude Code GitHub Action を設定するには、アプリをインストールし、シークレットを追加し、ワークフローファイルを自分でコピーします。

<Steps>
  <Step title="Claude GitHub App をインストール">
    [Claude GitHub App](https://github.com/apps/claude) をリポジトリにインストールします。Claude Code GitHub Action はアプリの 3 つの権限に依存しています。

    * **Contents**：読み取りと書き込み。Claude がリポジトリファイルを変更できるようにするため
    * **Issues**：読み取りと書き込み。Claude がイシューに応答できるようにするため
    * **Pull requests**：読み取りと書き込み。Claude が PR を作成し、変更をプッシュできるようにするため

    インストール中に、他の Claude 機能が使用する権限も付与します。完全なセットについては [GitHub App 権限](#github-app-permissions) を参照してください。
  </Step>

  <Step title="認証シークレットを追加">
    認証方法に応じて、リポジトリに以下のシークレットのいずれかを追加します。GitHub の [GitHub Actions でシークレットを使用する](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) ガイドを参照してください。

    * `ANTHROPIC_API_KEY`：[Claude Console](https://platform.claude.com) からの Claude API キー
    * `CLAUDE_CODE_OAUTH_TOKEN`：Claude サブスクリプションで認証する OAuth トークン。Pro、Max、Team、Enterprise プランで利用可能。`claude setup-token` をローカルで実行して生成します。[長期トークンを生成する](/docs/ja/authentication#generate-a-long-lived-token) を参照してください

    ワークフローファイルで、シークレットを一致する入力に渡します。API キーの場合は `anthropic_api_key`、OAuth トークンの場合は `claude_code_oauth_token`。
  </Step>

  <Step title="ワークフローファイルをコピー">
    [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) をリポジトリの `.github/workflows/` ディレクトリにコピーします。ファイルは単なる例ではなく、動作するワークフローです。コミットされたとおり、Claude はイシューまたはプルリクエストで誰かが `@claude` とメンションするたびに応答し、`ANTHROPIC_API_KEY` シークレットで認証します。代わりに `CLAUDE_CODE_OAUTH_TOKEN` を追加した場合は、ワークフローの `anthropic_api_key` 行を `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}` に変更します。
  </Step>
</Steps>

<Tip>
  セットアップ後、イシューまたは PR コメントで `@claude` をタグ付けして Claude Code GitHub Action をテストします。
</Tip>

<h3 id="set-up-for-an-organization">
  組織向けセットアップ
</h3>

クイックセットアップまたは手動セットアップでは、一度に 1 つのリポジトリを設定します。Claude Code GitHub Action を組織全体にロールアウトするには、以下を実行します。

* [Claude GitHub App](https://github.com/apps/claude) を組織レベルで 1 回インストールし、すべてのリポジトリまたは選択したリストを選択します
* 認証シークレットを組織レベルの Actions シークレットとして保存し、各リポジトリが独自のコピーを必要としないようにします
* Claude Code GitHub Action を実行する各リポジトリにワークフローファイルを追加するか、ジョブを [再利用可能なワークフロー](https://docs.github.com/en/actions/using-workflows/reusing-workflows) として 1 回定義し、各リポジトリが呼び出すようにします

リポジトリ間で共有されるシークレットの場合、OAuth トークンは `claude setup-token` を実行した人のサブスクリプションに関連付けられているため、[Claude Console](https://platform.claude.com) から API キーで認証します。

長期シークレットを保存することを完全に回避するには、ワークロード ID フェデレーション経由で認証します。Claude Code GitHub Action はワークフローの GitHub OpenID Connect（OIDC）トークンを Claude Console サービスアカウント経由で Claude API アクセスと交換します。これらの入力を設定します。

* `anthropic_federation_rule_id`：フェデレーションルール ID、`fdrl_...`
* `anthropic_organization_id`：Anthropic 組織 ID
* `anthropic_service_account_id`：サービスアカウント ID、`svac_...`。オプション。Console で作成するフェデレーションルールが既にサービスアカウントをターゲットしているため
* `anthropic_workspace_id`：ワークスペース ID、`wrkspc_...`。フェデレーションルールが単一のワークスペースをターゲットしている場合はオプション

ワークフローに `id-token: write` 権限を付与します。これは Claude Code GitHub Action が独自の `github_token` を渡す場合でもフェデレーション交換に必要です。Console 側の設定については、[Claude Code GitHub Action のセットアップガイド](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) を参照してください。

セキュリティレビューでのデータ処理と保持に関する質問については、[データ使用](/docs/ja/data-usage) と [セキュリティ](/docs/ja/security) を参照してください。

<h3 id="uninstall">
  アンインストール
</h3>

Claude Code GitHub Action を削除するには、インストールに適用される各セットアップを元に戻します。

* **ワークフローファイル**：`.github/workflows/` から `anthropics/claude-code-action` を使用するワークフローを削除します。クイックセットアップを使用した場合は、`claude.yml` を探し、レビューワークフローを選択した場合は `claude-code-review.yml` を探します。ワークフローが削除されると、Claude Code GitHub Action は実行されなくなります
* **シークレット**：リポジトリから `ANTHROPIC_API_KEY` または `CLAUDE_CODE_OAUTH_TOKEN` シークレットを削除し、[リポジトリ間で共有](#set-up-for-an-organization) した場合は組織レベルの Actions シークレットから削除します。シークレットを削除しても、それが保持していた認証情報は有効なままです。API キーを完全に廃止するには、[Claude Console](https://platform.claude.com) でキーも削除します
* **GitHub App**：リポジトリまたは組織設定の GitHub Apps で Claude GitHub App をアンインストールします。ただし、Code Review や web auto-fix などの別の Claude 機能に使用しない場合のみです

[クラウドプロバイダー](/docs/ja/github-actions-cloud-providers) を設定した場合は、`AWS_ROLE_TO_ASSUME`、`GCP_*` シークレット、または `AZURE_*` シークレットなどのプロバイダーシークレットも削除し、`APP_ID` と `APP_PRIVATE_KEY` シークレットとともにカスタム GitHub App をアンインストールします。

<h3 id="github-app-permissions">
  GitHub App 権限
</h3>

[Claude GitHub App](https://github.com/apps/claude) は、Claude Code GitHub Action、[Code Review](/docs/ja/code-review)、cloud sessions の Claude Code on the web の [プルリクエストの auto-fix](/docs/ja/claude-code-on-the-web#auto-fix-pull-requests) など、GitHub と統合するすべての Claude 機能で共有されます。GitHub App は、すべての機能をカバーする単一の権限セットを持つため、セットには Claude Code GitHub Action が使用しない権限が含まれます。

アプリをインストールすると、以下の権限を付与します。

| 権限               | アクセス      |
| ---------------- | --------- |
| Actions          | 読み取りと書き込み |
| Checks           | 読み取りと書き込み |
| Contents         | 読み取りと書き込み |
| Discussions      | 読み取りと書き込み |
| Issues           | 読み取りと書き込み |
| Members          | 読み取り      |
| Metadata         | 読み取り      |
| Pull requests    | 読み取りと書き込み |
| Repository hooks | 読み取りと書き込み |
| Statuses         | 読み取り      |
| Workflows        | 読み取りと書き込み |

権限セットは、それを使用する機能より前に変更される可能性があります。アプリが以前に持っていなかった権限をリクエストすると、GitHub はアカウント所有者に承認を促します。組織インストールの場合は組織所有者に促します。インストールは、承認されるまで古い権限を保持します。たとえば、Actions アクセスが読み取りから書き込みに変更されると、アプリはワークフローを再実行できるようになり、実行とログのみを表示できるようになるため、GitHub は所有者に変更を承認するよう求めます。

アプリをインストールすると、その完全な権限セットを受け入れます。GitHub では、サブセットを受け入れることはできません。組織が Claude Code GitHub Action が使用する権限のみを必要とする場合は、[Claude Code GitHub Action のセットアップガイド](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) に従って、Contents、Issues、Pull requests を持つカスタム GitHub App を作成します。カスタムアプリは Claude Code GitHub Action のみをカバーします。Code Review と web auto-fix には公式アプリが必要です。

Claude Code GitHub Action がこれらの権限で何ができるかを制限する方法の詳細については、[セキュリティドキュメント](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md) を参照してください。

<h2 id="interactive-and-automation-modes">
  インタラクティブモードと自動化モード
</h2>

Claude Code GitHub Action はワークフロー設定から実行方法を検出します。

* **インタラクティブモード**：ワークフローが `prompt` 入力を提供しない場合、Claude はトリガーフレーズ（デフォルトは `@claude`）をイシューまたはプルリクエストコメント、プルリクエストレビュー、または新しく開かれたイシューの本文またはタイトルで待機し、そのリクエストに応答します。進捗と結果は、トリガーするイシューまたは PR のコメントとして表示されます。
* **自動化モード**：ワークフローが `prompt` 入力を提供する場合、Claude は [実行をトリガーできるユーザーのチェック](#who-can-trigger-runs) のみに従って、待機せずに実行します。デフォルトでは、結果はコメントではなくワークフロー実行ログに表示されます。Claude は、プロンプトが指示し、[コード レビュー例](#run-a-skill) のようにツールがある場合、イシューまたはプルリクエストに投稿できます。

<h3 id="who-can-trigger-runs">
  実行をトリガーできるユーザー
</h3>

両方のモードで、Claude Code GitHub Action はトリガーするアクターに対して 2 つのチェックを実行してから Claude を開始します。どちらかのチェックが拒否すると、実行は失敗します。

* **書き込みアクセス**：イシューおよびプルリクエストイベントで、トリガーするユーザーはリポジトリへの書き込みアクセスを持つ必要があります。書き込みアクセスのない特定のユーザーを許可するには、`allowed_non_write_users` を設定し、独自の `github_token` 入力を渡します。`schedule` トリガーなど、ユーザーが作成しないイベントはこのチェックをスキップします。
* **人間のアクター**：すべてのイベントで、Claude Code GitHub Action は `allowed_bots` にリストされていない限り、ボットアクターを拒否します。これにより、ボットが Claude をループでトリガーするのを防ぎます。このチェックはスケジュール実行にも適用されます。GitHub はこれらを、通常はワークフローの `cron` スケジュールを最後に変更したリポジトリユーザーに属性付けします。そのユーザーがボットの場合は、`allowed_bots` にリストします。

<h2 id="example-use-cases">
  使用例
</h2>

[examples ディレクトリ](https://github.com/anthropics/claude-code-action/tree/main/examples) には、さまざまなシナリオ用の使用可能なワークフローが含まれています。

このページの例は API キー認証を示しています。Claude サブスクリプションで認証する場合は、任意の例の `anthropic_api_key` 行を `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}` に置き換えます。

<h3 id="respond-to-claude-mentions">
  @claude メンションに応答
</h3>

このワークフローは Claude Code GitHub Action をインタラクティブモードで実行するため、イシューまたは PR コメントで誰かが `@claude` とメンションするたびに Claude が応答します。

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

このワークフローのボイラープレートではない部分：

* `id-token: write`：Claude Code GitHub Action のデフォルト GitHub App 認証に必要
* `actions: read`：Claude が PR の CI 結果を読み取ることができるようにします
* `actions/checkout`：Claude がリポジトリのローカルコピーで作業できるようにします
* `if`：`@claude` とメンションしないコメントでランナーが開始されるのを防ぎます。Claude Code GitHub Action はトリガーフレーズ自体をチェックしてから応答します

ワークフローが配置されたら、リクエストを含むイシューまたは PR コメントで `@claude` とメンションします。

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude は同じイシューまたは PR のコメントで応答し、作業中に更新します。

<h3 id="run-a-skill">
  スキルを実行
</h3>

`prompt` 入力は、プレーンテキストだけでなく [スキル](/docs/ja/skills) 呼び出しも受け入れます。

* リポジトリの `.claude/skills/` ディレクトリ内のスキルの場合、`anthropics/claude-code-action` ステップの前に `actions/checkout` を実行してスキルファイルをランナーで利用可能にし、`/skill-name` を `prompt` として渡します。
* [plugin](/docs/ja/plugins/overview) にパッケージされたスキルの場合、`plugin_marketplaces` と `plugins` 入力でプラグインをインストールし、名前空間付きの `/plugin-name:skill-name` を `prompt` として渡します。`plugins` 入力は `plugin-name@marketplace-name` を取ります。マーケットプレイス名はマーケットプレイスのリポジトリ URL ではなく、マーケットプレイス自体のマニフェストから取得されます。

次のワークフローは `code-review` プラグインをインストールし、プルリクエストが開かれた、更新された、レビュー準備完了、または再度開かれたときにそのスキルを実行します。クイックセットアップからのレビューワークフローと同じプラグインを実行します。プロンプト、モデル、トリガーを自分で制御したい場合は、このようなワークフローを使用します。ワークフローファイルを維持せずに自動レビューするには、[Code Review](/docs/ja/code-review) を参照してください。パブリックリポジトリでは、GitHub はフォークプルリクエストでトリガーされた実行からシークレットを保留するため、レビューは同じリポジトリ内のブランチからのプルリクエストでのみ実行されます。

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

このワークフローの 2 つの行がレビューの場所を制御します。

* **`--comment`**：Claude はプルリクエストにレビューを投稿します。見つかった各イシューのインラインコメントとして、または見つからない場合は 1 つの概要コメントとして投稿します。これがないと、Claude は何も投稿せず、ワークフロー実行ログで結果を読みます。
* **`claude_args`**：スキル自体の `allowed-tools` frontmatter が同じツールを名前付けしていても、この行を保持します。Claude Code GitHub Action はインラインコメントを投稿する MCP サーバーを開始するのは、`claude_args` の `--allowedTools` がそれを名前付けしている場合のみです。

Claude は下書きおよび閉じられたプルリクエスト、レビューが不要と判断するプルリクエスト（自動化されたものや些細なものなど）、および Claude からのコメントが既にあるプルリクエストをスキップします。

<h3 id="run-on-a-schedule">
  スケジュールで実行
</h3>

`prompt` 入力を使用すると、Claude Code GitHub Action は cron スケジュールを含む任意の GitHub イベントで自動化モードで実行されます。プレーンテキストプロンプトの場合、Claude は `claude_args` の `--allowedTools` または `settings` 入力の [`permissions.allow` ルール](/docs/ja/permissions#permission-rule-syntax) でプロンプトが必要とするツールを付与するまで、シェルまたは GitHub API アクセスを持ちません。代わりにスキルを呼び出す場合、Claude は [`allowed-tools` frontmatter](/docs/ja/skills#pre-approve-tools-for-a-skill) が付与するツールを使用できます。GitHub はスケジュール済みワークフローをデフォルトブランチからのみ実行し、パブリックリポジトリではリポジトリアクティビティがない 60 日後にスケジュールを無効にします。

このワークフローは毎日 09:00 UTC にワークフロー実行ログでレポートを生成します。その `claude_args` 行は [CLI 引数を渡す](#pass-cli-arguments) ことで、モデルを選択し、2 つの GitHub MCP ツールを許可します。Claude はこれらのツールで GitHub API を通じてコミットとイシューを読み取るため、チェックアウトステップを省略できます。

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  ベストプラクティス
</h2>

<h3 id="define-project-standards-in-claude-md">
  CLAUDE.md でプロジェクト標準を定義
</h3>

リポジトリルートに `CLAUDE.md` ファイルを作成して、コードスタイルガイドライン、レビュー基準、プロジェクト固有のルール、および推奨パターンを定義します。Claude は PR を作成し、リクエストに応答するときにこれらのガイドラインに従います。詳細については、[メモリドキュメント](/docs/ja/memory) を参照してください。

<h3 id="protect-your-credentials">
  認証情報を保護
</h3>

<Warning>
  API キーまたは OAuth トークンをリポジトリに直接コミットしないでください。常に GitHub Secrets として保存し、ワークフローで参照します。例えば `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`。
</Warning>

ワークフローに必要な権限のみを付与し、マージする前に Claude の変更を確認します。

権限と認証を含む包括的なセキュリティガイダンスについては、[Claude Code Action セキュリティドキュメント](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md) を参照してください。

<h3 id="manage-costs">
  コストを管理
</h3>

各実行は 2 種類のリソースを消費します。

* **GitHub Actions 分**：Claude Code GitHub Action は GitHub ホストランナーで実行され、GitHub Actions 分を消費します。価格と分の制限については、[GitHub の請求ドキュメント](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) を参照してください。
* **API トークン**：各インタラクションはプロンプトと応答の長さ、タスクの複雑さ、コードベースのサイズに基づいてトークンを消費します。現在のトークンレートについては、[Claude の価格ページ](https://claude.com/platform/api) を参照してください。OAuth トークンで認証する場合、実行は API 請求ではなく Claude サブスクリプションを使用します。

両方の種類のコストを低下させるには、Claude により明確なコンテキストを提供し、各実行が実行できる作業量を制限します。

* 特定の `@claude` リクエストを記述して、Claude が完了するのに必要なターン数を減らします
* イシューテンプレートを使用して事前にコンテキストを提供します
* `CLAUDE.md` を簡潔に保ちます。Claude は実行ごとにそれを読み取るため
* `claude_args` で `--max-turns` を設定して反復を制限します
* ワークフローレベルのタイムアウトを設定して暴走ジョブを回避します
* GitHub の並行制御を使用して並列実行を制限します

組織全体での使用追跡については、[分析ダッシュボード](/docs/ja/analytics) と [監視](/docs/ja/monitoring-usage) を参照してください。使用量がどのように測定され、請求されるかについては、[コスト](/docs/ja/costs) を参照してください。

<h2 id="use-a-cloud-provider">
  クラウドプロバイダーを使用
</h2>

デフォルトでは、Claude Code GitHub Action は API キーまたは OAuth トークンで Claude API を直接呼び出します。代わりに独自のクラウドアカウント経由で推論をルーティングするには、プロバイダーの入力を設定し、[クラウドプロバイダーで Claude Code GitHub Actions を使用する](/docs/ja/github-actions-cloud-providers) に従います。

* **Amazon Bedrock**：`use_bedrock: "true"`
* **Google Cloud の Agent Platform**：`use_vertex: "true"`
* **Microsoft Foundry**：`use_foundry: "true"`

3 つのプロバイダーすべてで、Claude API キーではなく OIDC ID フェデレーション経由で認証するため、リポジトリに静的クラウド認証情報を保存しません。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude が @claude コマンドに応答しない
</h3>

* GitHub App がリポジトリにインストールされていることを確認します
* リポジトリでワークフローが有効になっていることを確認します
* API キーまたは OAuth トークンがリポジトリシークレットに設定されていることを確認します
* コメントに `@claude` が完全な単語として含まれていることを確認します。`/claude` または `@claude-bot` ではなく
* コメントするユーザーがリポジトリへの書き込みアクセスを持つことを確認します。[実行をトリガーできるユーザー](#who-can-trigger-runs) の例外を参照してください

<h3 id="ci-not-running-on-claude’s-commits">
  CI が Claude のコミットで実行されない
</h3>

* GitHub はデフォルトの `GITHUB_TOKEN` で作成されたコミットでワークフローをトリガーしません。Claude Code GitHub Action に `github_token: ${{ secrets.GITHUB_TOKEN }}` を渡す場合は、Claude GitHub App として認証するようにそれを削除するか、代わりにカスタムアプリトークンを渡します
* CI ワークフローのトリガーに Claude のプッシュが生成するイベント（`push` または `pull_request` など）が含まれていることを確認します

<h3 id="authentication-errors">
  認証エラー
</h3>

* API キーまたは OAuth トークンが有効であることを確認します。ワークフローをデバッグする前に `claude` でローカルでテストします
* Bedrock、Agent Platform、Foundry の場合は、クラウドプロバイダーページの [トラブルシューティングセクション](/docs/ja/github-actions-cloud-providers#troubleshooting) を参照してください

その他のソリューションについては、Claude Code GitHub Action の [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) を参照してください。

<h2 id="advanced-configuration">
  高度な設定
</h2>

<h3 id="action-parameters">
  アクション パラメータ
</h3>

これらは最も一般的に使用される入力です。各パラメータは `anthropics/claude-code-action` ステップの `with:` キーにマップされます。

| パラメータ                     | 説明                                                                                                                        | 必須                                                                                                                                                              |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Claude への指示。プレーン テキストまたは[スキル](/docs/ja/skills)呼び出しとして指定します。省略した場合、Claude は[トリガー フレーズ](#interactive-and-automation-modes)に応答します | いいえ                                                                                                                                                             |
| `claude_args`             | Claude Code に渡される CLI 引数                                                                                                  | いいえ                                                                                                                                                             |
| `anthropic_api_key`       | Claude API キー                                                                                                             | Claude API の場合は必須です。ただし、`claude_code_oauth_token` または[ワークロード ID フェデレーション](#set-up-for-an-organization)を使用する場合は不要です。Bedrock、Agent Platform、または Foundry では使用されません |
| `claude_code_oauth_token` | Claude サブスクリプションで認証するための OAuth トークン。`claude setup-token` で生成されます                                                          | いいえ                                                                                                                                                             |
| `github_token`            | GitHub 操作用のトークン。省略した場合、Claude Code GitHub Action は Claude GitHub App として認証されます                                            | いいえ                                                                                                                                                             |
| `plugin_marketplaces`     | プラグイン マーケットプレイス Git URL の改行区切りリスト                                                                                         | いいえ                                                                                                                                                             |
| `plugins`                 | 実行前にインストールするプラグイン名の改行区切りリスト                                                                                               | いいえ                                                                                                                                                             |
| `settings`                | Claude Code 設定。JSON 文字列またはセッティング JSON ファイルへのパス                                                                            | いいえ                                                                                                                                                             |
| `trigger_phrase`          | Claude が応答するトリガー フレーズ。デフォルト: `@claude`                                                                                    | いいえ                                                                                                                                                             |
| `use_bedrock`             | Claude API の代わりに Amazon Bedrock を使用します                                                                                    | いいえ                                                                                                                                                             |
| `use_vertex`              | Claude API の代わりに Google Cloud の Agent Platform を使用します                                                                     | いいえ                                                                                                                                                             |
| `use_foundry`             | Claude API の代わりに Microsoft Foundry を使用します                                                                                 | いいえ                                                                                                                                                             |

完全な入力リストについては、Claude Code GitHub Action の[設定リファレンス](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs)を参照してください。

<h3 id="pass-cli-arguments">
  CLI 引数を渡す
</h3>

`claude_args` パラメータは、任意の [Claude Code CLI 引数](/docs/ja/cli-reference)を受け入れます。

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

一般的な引数:

* `--max-turns`: 会話ターン数を制限します
* `--model`: 使用するモデル。例えば `claude-sonnet-5`。この引数がない場合、Claude Code GitHub Action は Claude Code の[デフォルト モデル](/docs/ja/model-config)を使用します
* `--mcp-config`: [MCP 設定](/docs/ja/mcp)へのパス
* `--allowedTools`: 許可されたツールのカンマ区切りリスト。`--allowed-tools` エイリアスも機能します
* `--debug`: デバッグ出力を有効にします

<h2 id="upgrade-from-beta">
  ベータ版からアップグレード
</h2>

ワークフローがまだ `anthropics/claude-code-action@beta` を参照している場合は、v1 に更新します。

1. `uses` 行の `@beta` を `@v1` に変更します
2. `mode` 入力を削除します。Claude Code GitHub Action は [モードを自動的に検出](#interactive-and-automation-modes) するようになったため
3. `direct_prompt` を `prompt` に置き換えます
4. `max_turns` や `model` などの CLI オプションを `claude_args` に移動します。`custom_instructions` は同じ名前のフラグを持たず、`--append-system-prompt` になります

完全な入力マッピングと前後の例については、[マイグレーションガイド](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md) を参照してください。

<h2 id="what’s-next">
  次のステップ
</h2>

* [クラウドプロバイダーで Claude Code GitHub Actions を使用する](/docs/ja/github-actions-cloud-providers)：Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry 経由で推論をルーティング
* [設定リファレンス](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs)：アクション入力の完全なリスト
* [Examples ディレクトリ](https://github.com/anthropics/claude-code-action/tree/main/examples)：より多くのシナリオ用の使用可能なワークフロー
* [Code Review](/docs/ja/code-review)：ワークフローファイルを維持せずに自動プルリクエストレビュー
