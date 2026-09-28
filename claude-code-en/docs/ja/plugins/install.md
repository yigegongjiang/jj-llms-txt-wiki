> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインのインストールと管理

> 任意のサーフェスから Claude Code プラグインをマーケットプレイスからインストールし、インストール範囲を選択して、後で更新または削除します。

プラグインをインストールすると、そのスキル、エージェント、フック、および MCP サーバーが、マシン上の Claude Code に追加されます。

このページは、ターミナル、デスクトップアプリ、IDE、またはクラウドセッションで、自分のマシンまたはアカウントでプラグインを使用している人向けです。プラグインのインストール、範囲の選択、マーケットプレイスの追加、およびプラグインの更新を保つ方法について説明しています。

<Note>
  以下のケースは他のページで説明されています。

  * **claude.ai チャットまたは Cowork を使用していて、Claude Code ではない場合**: [claude.ai と Cowork のプラグイン](https://claude.com/docs/plugins/overview)を参照してください
  * **Claude Code がエラーを出力した場合**: [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)で見つけてください
</Note>

[プラグインのインストール](#install-a-plugin)から始めてください。誰かが送信したインストールコマンドの `@` 名が `claude-plugins-official` でない場合は、まず[マーケットプレイスを追加](#add-a-marketplace)してください。

<h2 id="install-a-plugin">
  プラグインのインストール
</h2>

例として、このセクションでは [Anthropic の公式マーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)から [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) をインストールします。このプラグインは、コミット、プッシュ、およびプルリクエストを開くためのコマンドを追加します。

同じ手順で他のプラグインもインストールできます。`commit-commands` と `claude-plugins-official` が表示される場所で、プラグインの名前とそのマーケットプレイスの名前に置き換えてください。そのプラグインが別のマーケットプレイスから来ている場合は、まず[マーケットプレイスを追加](#add-a-marketplace)してください。

Claude Code を実行する場所のタブを選択してください。

<Tabs>
  <Tab title="Terminal">
    プロジェクトで `claude` を使用して Claude Code を開始してから、以下を実行します。

    <Steps>
      <Step title="インストールコマンドでプラグインの詳細を開く">
        プラグインの名前とマーケットプレイスを指定して `/plugin install` を実行します。セッション内では、このコマンドはすぐにはインストールされません。代わりに、そのプラグインの詳細を `/plugin` パネルで開き、レビューして最初に範囲を選択できるようにします。

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        代わりに参照するには、プラグイン名なしで `/plugin` を実行します。パネルは **Discover** タブで開き、追加したすべてのマーケットプレイスからプラグインをリストアップし、入力して検索してから、プラグインで **Enter** を押してその詳細を開くことができます。
      </Step>

      <Step title="プラグインが追加する内容を確認する">
        詳細ペインにはプラグインの説明が表示されます。また、以下も表示できます。

        * **Will install**: プラグインが追加するコマンド、エージェント、スキル、フック、および MCP と LSP サーバー。
        * **Last updated**: Anthropic の公式マーケットプレイスのプラグインに対して表示されます。
        * **Context cost**: Anthropic の公式マーケットプレイスのプラグインの場合、2 つのトークン推定値。**Every turn** はプラグインが送信する各メッセージに追加するもので、**When invoked** はスキルとエージェントが Claude がそれらを読み込んだ後に追加するものです。推定値は、ステップ 1 のコマンドのようにマーケットプレイスを指定してプラグインを開くか、**Marketplaces** タブから開くときに表示されます。**Discover** リストから到達する詳細ペインには表示されません。

        ローカルまたはカスタムマーケットプレイスのプラグインは、代わりに `Components will be discovered at installation` を表示できます。

        プラグインはフックと MCP サーバーを実行できるため、インストール前にペインを読んでください。[プラグインのセキュリティと信頼](/docs/ja/plugins/security)を参照してください。
      </Step>

      <Step title="範囲を選択する">
        3 つのインストールオプションのいずれかを選択します。

        * **Install for you (user scope)**: このマシン上のすべてのプロジェクトでプラグインを取得します
        * **Install for all collaborators on this repository (project scope)**: このリポジトリで作業するすべての人に対して有効になります
        * **Install for you, in this repo only (local scope)**: このリポジトリのみでプラグインを取得します

        [インストール範囲を選択](#choose-an-install-scope)では、各範囲が書き込む設定ファイルと、同じプラグインが複数の場所で設定されている場合に適用されるものについて説明しています。

        範囲を選択すると、Claude Code はプラグインと宣言された依存関係をインストールし、インストール概要を出力します。
      </Step>

      <Step title="インストール概要を読む">
        概要の最後の文は、このセッションでプラグインが使用可能かどうかを示しています。

        * **Active now**: `Plugin is now active.` リロードは不要です。
        * **Reload needed**: `Run /reload-plugins to activate.` パネルが閉じ、Claude Code がそのリロードを実行します。リロードが[プロンプトキャッシュを無効化](/docs/ja/prompt-caching#enabling-or-disabling-a-plugin)する場合、警告が表示され、代わりにプラグインは保留中のままになります。`/reload-plugins --force` を実行してアクティブ化すると、キャッシュされていない 1 つのリクエストがかかります。
        * **Load failed**: `The plugin couldn't be loaded`。`/plugin` の **Errors** タブを開いて理由を確認し、[インストール後: プラグインが機能しない](/docs/ja/plugins/troubleshooting#plugin-installed-but-not-working)を参照してください。
      </Step>

      <Step title="プラグインが機能することを確認する">
        `/` を入力し、`/<plugin>:<skill>` の形式でプラグイン名の下にプラグインのスキルを探します。`commit-commands` の場合、`/commit-commands:commit` が表示されます。プラグインをリストアップする他の 2 つの場所があります。

        * `/plugin` の **Installed** タブを開き、プラグインをそのスコープと共にリストアップします。
        * シェルで `claude plugin list` を実行し、`Version`、`Scope`、および `Status` 行と同じリストを出力します。

        `/commit-commands:commit` が表示されない場合は、[インストール後: プラグインが機能しない](/docs/ja/plugins/troubleshooting#plugin-installed-but-not-working)を参照してください。
      </Step>
    </Steps>

    他のマーケットプレイスからのインストールには、最初に 1 つの追加ステップが必要です。[マーケットプレイスを追加](#add-a-marketplace)してください。Claude Code は、対話的なターミナルセッションを初めて開始するときに、Anthropic の公式マーケットプレイスを自動的に追加します。これが例がそのステップをスキップする理由です。[claude.com/marketplace](https://claude.com/marketplace) でプラグインを見つけた場合、その **Claude Code** ボタンは、[シェル形式](#install-from-your-shell)のインストールコマンド `claude plugin install <name>@claude-plugins-official` をコピーします。
  </Tab>

  <Tab title="Desktop app">
    デスクトップアプリの **Code** タブのローカルまたは SSH セッションで。

    <Steps>
      <Step title="プラグインブラウザを開く">
        プロンプトボックスの横にある **+** ボタンをクリックし、**Plugins** を選択してから **Add plugin** を選択します。プラグインブラウザがマーケットプレイスのプラグインと共に開きます。
      </Step>

      <Step title="プラグインを選択する">
        `commit-commands` を見つけて選択します。
      </Step>

      <Step title="範囲を選択する">
        [範囲](#choose-an-install-scope)を選択します。ユーザーアカウント、このプロジェクト、またはローカルのみ。
      </Step>
    </Steps>

    後で有効化、無効化、またはアンインストールするには、**+ > Plugins > Manage plugins** を使用します。プラグインブラウザはデスクトップアプリのクラウドセッションでは利用できません。[デスクトップアプリでプラグインをインストール](/docs/ja/desktop#install-plugins)を参照してください。
  </Tab>

  <Tab title="VS Code">
    VS Code の Claude Code パネルで。

    <Steps>
      <Step title="プラグインの管理を開く">
        プロンプトボックスに `/plugins` を入力して **Manage plugins** を開きます。
      </Step>

      <Step title="プラグインをインストールする">
        **Plugins** タブで `commit-commands` を検索し、**Install** をクリックします。タブにプラグインがリストアップされていない場合は、まず **Marketplaces** タブで `anthropics/claude-plugins-official` を追加してください。
      </Step>

      <Step title="範囲を選択する">
        [範囲](#choose-an-install-scope)を選択します。**Install for you**、**Install for this project**、または **Install locally**。
      </Step>
    </Steps>

    変更は再起動なしで開いているセッションに適用されます。[VS Code でプラグインを管理](/docs/ja/vs-code#manage-plugins)を参照してください。
  </Tab>

  <Tab title="Cloud session">
    [クラウドセッション](/docs/ja/cloud-environments)（[claude.ai/code のブラウザ](/docs/ja/claude-code-on-the-web)を含む）には、プラグインブラウザがなく、自分のマシンにインストールしたプラグインや、リポジトリの `.claude/settings.json` がオンにするプラグインは読み込まれません。組織が管理設定を通じて配布するプラグインについては、[組織のプラグインを管理](/docs/ja/plugins/org)を参照してください。

    [セットアップのどの部分がクラウドセッションでも利用可能か](/docs/ja/cloud-environments#what-carries-over-from-your-setup)については、セットアップの残りの部分を参照してください。
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  インストール範囲を選択する
</h3>

プラグインのインストール範囲は、誰がプラグインを取得し、どの設定ファイルがそれを有効として記録するかを決定します。

* **User scope**: プラグインはこのマシン上のすべてのプロジェクトで有効になります。エントリは `~/.claude/settings.json` の `enabledPlugins` に入ります。
* **Project scope**: プラグインはこのリポジトリで作業するすべての人に対して有効になります。エントリは `.claude/settings.json` に入り、コミットします。
* **Local scope**: プラグインはこのリポジトリのみで有効になります。エントリは `.claude/settings.local.json` に入ります。

一部のプラグインは、[`defaultEnabled`](/docs/ja/plugins/manifest-reference#defaultenabled) フィールドを通じて、作成者によってオフで開始するように設定されています。そのようなプラグインはインストールされていますが、シェルで `claude plugin enable <name>` を実行するか、セッション内の `/plugin` の **Installed** タブからオンにするまでオフのままです。

同じプラグインが複数のスコープで設定されている場合、ローカル設定がプロジェクト設定をオーバーライドし、プロジェクト設定がユーザー設定をオーバーライドします。完全なルールについては、[プラグインが有効な場所を見つける](/docs/ja/plugins/loading#find-where-a-plugin-is-enabled)を参照してください。

ターミナル、デスクトップアプリのローカルセッション、および 1 台のコンピュータ上の VS Code 拡張機能は、同じ設定ファイルを読み込むため、それらのいずれかでユーザースコープでインストールしたプラグインは、他の 2 つで利用可能です。

<h3 id="other-places-you-run-claude-code">
  JetBrains、非対話的実行、および Agent SDK
</h3>

Claude Code を実行する一部の場所には、独自のプラグインブラウザがありません。

* **JetBrains IDEs**: JetBrains プラグインは IDE のターミナルで Claude Code を実行するため、**Terminal** タブのステップをそこで使用してください。
* **`claude -p` およびその他の非対話的実行**: `/plugin` は実行されず、Claude は `/plugin isn't available in this environment.` と返信します。既にインストールしたプラグインは読み込まれます。シェルから [`claude plugin` コマンド](#install-from-your-shell)を使用してインストールおよび管理してください。
* **Agent SDK**: SDK のプラグインオプションを通じてプラグインを読み込みます。[Agent SDK でプラグインを読み込む](/docs/ja/agent-sdk/plugins)を参照してください。

Claude Code がリポジトリの `.claude/settings.json` で有効になっているプラグインがインストールされていないと報告する場合は、[プロジェクト設定で有効だがインストールされていない](/docs/ja/plugins/loading#enabled-in-project-settings-but-not-installed)を参照してください。

<Tip>
  プラグイン作成者がディスク上のプラグインのコピーをテストしている場合は、シェルから `--plugin-dir` を使用して Claude Code を開始し、インストールする代わりに 1 つのセッションでそれを読み込みます。[プラグインを 1 つのセッションで読み込むフラグ](/docs/ja/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)を参照してください。
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  claude.ai アカウントからのプラグイン
</h3>

claude.ai アカウントは、インストール元のマーケットプレイスと並んで、プラグインの別のソースです。

* **到着するもの**: claude.ai アカウントでオンにするすべてのプラグイン、および組織がそのメンバーに対してオンにするすべてのプラグイン。ターミナルセッションでは、そのアカウントでサインインしながら Claude Code を開始するたびにバックグラウンドで同期されます。Cowork セッションでは、セッションが開始するときにダウンロードされます。
* **表示される場所**: `/plugin` と `claude plugin list` で、ID `<name>@synced` の下。組織がそれを必須にしない限り、独自のスコープでオフにできます。
* **反対方向に進まないもの**: `/plugin` または `claude plugin install` でインストールしたプラグインはこのマシンに留まり、claude.ai アカウントに追加されません。

同期タイミング、サインイン要件、および同期をオフにすることについては、[claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins)を参照してください。

<h3 id="install-from-your-shell">
  シェルからインストール
</h3>

シェルで `claude plugin install` を実行して、Claude Code セッションを開始せずにプラグインをインストールします。例えば、セットアップスクリプトから。

* **Scope**: デフォルトではユーザースコープ。`--scope project` または `--scope local` を渡して変更します。
* **プラグインが読み込まれるとき**: インストールするプラグインは、Claude Code を次に開始するときか、既に開いているセッションで `/reload-plugins` を実行するときに読み込まれます。
* **マーケットプレイスは最初に追加する必要があります**: 誰も対話的な Claude Code セッションを開いていないマシンでは、公式マーケットプレイスが登録されていないため、そこからインストールするスクリプトは、インストール前に `claude plugin marketplace add anthropics/claude-plugins-official` を実行します。

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

コマンドは完了時に `Successfully installed plugin: formatter@your-org (scope: project)` を出力します。

一部のプラグインは、マーケットプレイスが名前を付けるコマンドを実行することでインストールされます。これは [`command` ソース](/docs/ja/plugins/marketplace-reference#command-plugin-source)と呼ばれます。Claude Code はそのコマンドを表示し、実行前にそれを受け入れるよう求めます。スクリプトにはそのプロンプトに答える人がいないため、そこで `--yes` を渡してそれを受け入れます。

すべての `claude plugin install` フラグについては、[plugin install](/docs/ja/plugins/cli-reference#plugin-install)を参照してください。

<h2 id="add-a-marketplace">
  マーケットプレイスを追加する
</h2>

このセクションは、必要なプラグインが Anthropic の公式マーケットプレイスにない場合にのみ必要です。例えば、同僚が公開したものや、Anthropic のコミュニティマーケットプレイスからのものなど。

マーケットプレイスはプラグインのカタログであり、Claude Code がそこからインストールする前に、マーケットプレイスについて知る必要があります。マーケットプレイスは 1 回追加します。その後、そのプラグインは **Discover** タブに表示され、セッション内で `/plugin install <plugin>@<marketplace>` またはシェルで `claude plugin install <plugin>@<marketplace>` でインストールされます。ここで `<marketplace>` はマーケットプレイスが登録した名前です。両方を 1 つのステップで実行するには、[マーケットプレイスを追加してインストール](#add-a-marketplace-and-install-in-one-command)を参照してください。

Claude Code セッション内で、`/plugin marketplace add` の後にマーケットプレイスのソースを実行します。GitHub リポジトリ、任意のホスト上の git リポジトリ、ローカルディレクトリまたはファイル、またはホストされた `marketplace.json`。

| ソース                       | 入力するもの                                                                                                                                                          | 例                                                                                                                          |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| GitHub リポジトリ              | `owner/repo`。ブランチまたはタグをピンするには `#ref` を追加します。                                                                                                                    | `/plugin marketplace add anthropics/claude-code`、または `/plugin marketplace add your-org/plugins#v1.2.0` で `v1.2.0` タグをピンします |
| 任意のホスト上の Git リポジトリ        | 完全なクローン URL。ブランチまたはタグをピンするには `#ref` を追加します。                                                                                                                     | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                |
| ローカルディレクトリまたはファイル         | `.claude-plugin/marketplace.json` を保持するディレクトリへの相対パスまたは絶対パス、または JSON ファイル自体へのパス。相対パスを `./` または `../` で開始します。Claude Code は裸の `name/name` を GitHub リポジトリとして読み込むため。 | `/plugin marketplace add ./my-marketplace`                                                                                 |
| ホストされた `marketplace.json` | その `https://` URL                                                                                                                                               | `/plugin marketplace add https://example.com/marketplace.json`                                                             |

シェルから、`claude plugin marketplace add` は同じソースを取ります。

<Tip>
  `/plugin market` は `/plugin marketplace` の短い形式としても機能します。
</Tip>

すべての URL に `https://` プレフィックスを含めるか、SSH に `git@host:path` 形式を使用します。裸の `gitlab.example.com/your-group/your-marketplace.git` を入力すると、Claude Code はそれを GitHub `owner/repo` 短縮形として読み込み、拒否します。

コマンドが成功すると、`Successfully added marketplace: <name>` が出力され、マーケットプレイスのプラグインは次に `/plugin` を開くときに **Discover** タブに表示されます。リロードは不要です。失敗した場合は、[プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting#add-a-marketplace)のエラーメッセージと一致させてください。

<h3 id="add-a-marketplace-and-install-in-one-command">
  マーケットプレイスを追加してインストール（1 つのコマンド）
</h3>

まだ追加していないマーケットプレイスからプラグインをインストールするには、Claude Code セッション内で `/plugin install` を実行し、`--marketplace` でマーケットプレイスソースを指定します。Claude Code v2.1.275 以降が必要です。

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

ソースは [/plugin marketplace add](#add-a-marketplace) と同じ形式を取ります。例えば、GitHub `owner/repo`、git URL、またはローカルパス。ただし、スペースを含むことはできません。プラグイン名を `@marketplace` サフィックスなしで指定します。

そのマーケットプレイスをまだ追加していない場合、Claude Code は解決したソースを表示し、追加する前に確認するよう求めます。マーケットプレイスが追加されると、プラグインの詳細が開き、[インストール範囲](#install-a-plugin)を選択します。ソースが既に追加したマーケットプレイスと一致する場合、Claude Code は確認をスキップし、そのマーケットプレイスでプラグインの詳細を開きます。

<h3 id="add-a-private-marketplace">
  プライベートマーケットプレイスを追加する
</h3>

プライベートマーケットプレイスは、GitHub または他の git ホスト上の、クローンするために認証情報が必要なリポジトリ内のマーケットプレイスです。公開マーケットプレイスと同じ `/plugin marketplace add` または `claude plugin marketplace add` コマンドで追加します。Claude Code はマシンに既にある git 認証情報でそれをクローンし、プロンプトを表示しません。そのため、各接続方法には要件があります。

* **HTTPS**: git 認証情報ヘルパーが適用されるため、`gh auth login`、macOS Keychain、または `git-credential-store` で設定したアクセスが機能します。対話的なプロンプトは抑制されるため、認証したことのないホストはパスワードを求める代わりに失敗します。
* **SSH**: ホストは既に `known_hosts` ファイルに含まれている必要があり、キーはパスフレーズプロンプトなしで機能する必要があります。ホストフィンガープリントとパスフレーズプロンプトも抑制されるため。
* **GitHub `owner/repo` 短縮形**: Claude Code は SSH キーが `github.com` に認証するかどうかを確認し、認証する場合は SSH でクローンし、認証しない場合は HTTPS でクローンします。[`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/ja/env-vars#variables)を設定して、そのチェックをスキップし、常に HTTPS でクローンします。

同じ認証情報は、`/plugin install`、`/plugin marketplace update`、および `claude plugin update` を実行するときに適用されます。

GitHub Enterprise Server ホストについては、[GHES 上のプラグインマーケットプレイス](/docs/ja/github-enterprise-server#plugin-marketplaces-on-ghes)を参照して、各操作に必要な認証情報を確認してください。

組織が管理設定を通じてマーケットプレイスを登録する場合、自分で追加する必要はありません。[プラグインの事前インストールと要求](/docs/ja/plugins/org#pre-install-and-require-plugins)を参照してください。

<h3 id="add-from-claude-ai">
  claude.ai からマーケットプレイスを追加する
</h3>

[claude.ai アカウントからプラグインが同期される](/docs/ja/plugins/loading#synced-plugins)ターミナルセッションでは、claude.ai はプラグインマーケットプレイスもリストアップできます。例えば、組織のプラグインライブラリと独自の claude.ai アップロード。ソースではなく名前でこれらの 1 つを追加します。claude.ai からマーケットプレイスを追加するには、Claude Code v2.1.273 以降が必要です。

`/plugin` パネルまたはシェルから claude.ai マーケットプレイスを追加します。

* **セッション内**: `/plugin` を実行し、**Marketplaces** タブに移動します。これは claude.ai からのマーケットプレイスをリストアップします。そこで 1 つを選択して追加します。
* **シェルから**: `claude plugin marketplace list` を実行します。これは `From claude.ai:` セクションでそれらを出力します。次に、`claude plugin marketplace add` を `--claudeai` フラグと、リストに表示されている名前で実行します。

例えば、このコマンドは `claudeai-organization-library` という名前のマーケットプレイスを追加します。

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code はマーケットプレイスを、claude.ai がリストアップした名前から派生したローカル名で登録します。これは `claudeai-` で始まります。例えば、「Organization library」としてリストアップされたマーケットプレイスは `claudeai-organization-library` になります。例えば `claude plugin install <plugin>@claudeai-organization-library` でその名前でプラグインをインストールします。

サインアウトするか、別の claude.ai 組織にサインインすると、マーケットプレイスは設定されたままですがプラグインを表示せず、既にそこからインストールしたプラグインは読み込まれ続けます。

`From claude.ai:` セクションは、claude.ai を通じて共有される git ベースのマーケットプレイスもリストアップでき、それぞれのソースを出力します。[マーケットプレイスを追加](#add-a-marketplace)のようにそのソースで追加します。`--claudeai` ではなく。

<h2 id="manage-installed-plugins">
  インストール済みプラグインを管理する
</h2>

`/plugin` の **Installed** タブは、プラグインをリストアップし、各プラグインを有効化、無効化、更新、またはアンインストールするアクションを表示します。Claude Code セッション内で、`/plugin` を実行して **Tab** を押してそこに到達するか、`/plugin enable`、`/plugin disable`、または `/plugin uninstall` を実行してパネルを開き、そこで変更を加えます。無効化されたプラグインは、リストの下部の折りたたまれたヘッダーの下にグループ化されます。リストでこれらのキーを使用します。

* 入力して名前または説明でフィルタリングします。
* **Space** を押して選択したプラグインを有効化または無効化し、**f** でお気に入りにします。
* **Enter** を押してプラグインの詳細を開きます。そこのメニューは **Disable plugin** または **Enable plugin**、**Update now**、および **Uninstall** を提供します。設定を取得するプラグインは **Configure options** も提供します。

タブは **Managed** スコープのプラグインも表示できます。組織は [管理設定](/docs/ja/settings#settings-files)を通じてそれらをインストールし、ここでそれらを有効化、無効化、またはアンインストールすることはできません。

組織が claude.ai で必須にしている同期プラグインについては、[claude.ai から同期されたプラグインを管理](#manage-plugins-synced-from-claude-ai)を参照してください。

`/plugin` パネルを、それで加えた保留中の変更で閉じると、Claude Code は `/reload-plugins` を実行してそれらを適用します。リロードが[プロンプトキャッシュを無効化](/docs/ja/prompt-caching#enabling-or-disabling-a-plugin)する場合、警告が表示され、代わりに変更は保留中のままになります。`/reload-plugins --force` を実行してそれらを適用します。

<h3 id="manage-plugins-synced-from-claude-ai">
  claude.ai から同期されたプラグインを管理する
</h3>

`/plugin` の **Installed** タブは、[claude.ai アカウントから同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins)もリストアップし、ソースとして `synced` を使用します。同期されたプラグインは Claude Code v2.1.273 以降のターミナルセッションに表示されます。

* **有効化または無効化**: **Installed** タブを使用します。組織がプラグインを必須としてマークしない限り。
* **削除**: claude.ai でプラグインをオフにします。

Claude Code が追加、更新、または削除されたプラグインを対話的なセッションに同期するとき、`Plugins changed. Run /reload-plugins to activate.` が表示されます。`/reload-plugins` を実行してそのセッションで変更を読み込むか、次に Claude Code を開始するときのために残します。

<h3 id="uninstall-a-plugin-the-project-enables">
  プロジェクトが有効にするプラグインをアンインストール
</h3>

このリポジトリの `.claude/settings.json` が有効にするプラグインに対して **Uninstall** を選択するとき、**Installed** タブから、または `/plugin uninstall` で、Claude Code は、それを自分に対して無効化するか、すべての人に対してアンインストールするかを尋ねます。

* **Disable for me**: **y** を押します。Claude Code は `.claude/settings.local.json` でプラグインに対して `false` を書き込み、プロジェクトに対してインストールされたままにします。
* **Uninstall for everyone**: **u** を押します。Claude Code は共有 `.claude/settings.json` からプラグインを削除します。

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  インストール済みプラグインがセッションに追加するものを確認する
</h3>

シェルで、インストール済みプラグインに対して `claude plugin details <name>` を実行します。**Always-on** 行は、プラグインが有効になっているすべてのセッションに追加するトークン数であり、コンポーネント行は、どのスキルまたはエージェントが最も貢献するかを示します。完全な出力と各数値の意味については、[プラグインのコストを測定](/docs/ja/plugins/measure#measure-what-a-plugin-costs)を参照してください。

<h3 id="find-plugins-you-no-longer-use">
  使用していないプラグインを見つける
</h3>

`/plugin` の **Installed** タブで、自分でインストールし、最近使用していないプラグインは **Not used recently** ヘッダーの下に表示され、各プラグインの詳細は **Last used** 行を表示します。そのヘッダーとその行を使用して、スタートアップとコンテキストコストを追加し続けるプラグインを見つけ、無効化またはアンインストールします。

<h3 id="plugins-with-dependencies">
  依存関係を持つプラグイン
</h3>

プラグインは、それが依存する他のプラグインを宣言できます。マーケットプレイスからそのようなプラグインをインストール、無効化、またはアンインストールするとき、Claude Code はそれらの依存関係にも作用します。

* **Install**: Claude Code はプラグインの宣言された依存関係も同じスコープでインストールおよび有効化します。成功メッセージはそれらをリストアップします。
* **Enable**: Claude Code はプラグインの依存関係もインストールされているが無効化されているものを有効化します。宣言された依存関係がインストールされていない場合、有効化は失敗し、メッセージは最初にそれをインストールするよう指示します。
* **Disable**: 別の有効なプラグインがまだ名前を付けたものを必要とするとき、Claude Code は拒否し、両方を正しい順序で無効化するチェーンコマンドを出力します。
* **Uninstall**: 自動インストールされた依存関係は、シェルで `claude plugin prune` を実行するまで残ります。[plugin prune](/docs/ja/plugins/cli-reference#plugin-prune)を参照してください。

代わりに `--plugin-dir` でプラグインを読み込んだ場合は、[プラグインとその依存関係をローカルでテスト](/docs/ja/plugins/dependencies#test-a-plugin-and-its-dependency-locally)を参照してください。

<h3 id="manage-plugins-from-your-shell">
  シェルからプラグインを管理する
</h3>

Claude Code セッションを開始せずにプラグインを管理することもできます。シェルで、`claude plugin install`、`enable`、`disable`、または `uninstall` を通常のターミナルコマンドとして実行します。それらは `/plugin` パネルが行うのと同じ設定を変更します。各々は `--scope` を取ってスコープをターゲットにし、省略するときはデフォルトスコープを使用します。

* `enable` と `disable` は、プラグインを既にリストアップしている最も具体的なスコープに作用します。
* `install` と `uninstall` はユーザースコープに作用します。

例えば、これらのコマンドはプラグインを無効化して再度有効化し、プロジェクトスコープでアンインストールします。

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  プラグインを更新し続ける
</h2>

プラグインは、それらが来たマーケットプレイスが自動更新をオンにしているときに自動的に更新されます。セッションが開始した後、Claude Code はそれらのマーケットプレイスをリフレッシュし、インストールしたプラグインのディスク上のコピーを更新します。

実行中のセッションは、既に読み込んだバージョンを保持します。更新後、`Plugin updated: <name> · Run /reload-plugins to apply` が表示され、次のセッションは新しいバージョンを自動的に読み込みます。

これらは各マーケットプレイスの種類の自動更新デフォルトです。

* **デフォルトでオン**: `claude-plugins-official` および `knowledge-work-plugins` と `first-party-plugins` を除く他の[公式マーケットプレイス名](/docs/ja/plugins/security#official-marketplace-names)、および [claude.ai から追加されたマーケットプレイス](#add-from-claude-ai)。
* **デフォルトでオフ**: 他のすべてのマーケットプレイス。コミュニティマーケットプレイス、サードパーティマーケットプレイス、およびローカル開発マーケットプレイスを含む。

自動更新が実行されるとき、スキップするプラグイン、および自動更新をオフにする環境変数については、[自動更新が実行されるとき](/docs/ja/plugins/loading#when-auto-update-runs)を参照してください。

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  マーケットプレイスの自動更新をオンまたはオフにする
</h3>

Claude Code セッション内で、`/plugin` を実行し、**Marketplaces** タブに移動します。マーケットプレイスを選択し、**Enable auto-update** または **Disable auto-update** を選択します。

<h3 id="update-one-plugin-now">
  1 つのプラグインを今すぐ更新
</h3>

セッション内で、`/plugin` の **Installed** タブでプラグインを開き、**Update now** を選択するか、シェルで `claude plugin update <plugin>@<marketplace>` を実行します。

<h3 id="auto-update-from-a-private-marketplace">
  プライベートマーケットプレイスから自動更新
</h3>

プライベートマーケットプレイスについては、[バックグラウンド自動更新が認証情報で行うこと](/docs/ja/plugins/host-marketplace#what-background-auto-update-does-with-credentials)を参照して、バックグラウンド自動更新が SSH と HTTPS でどのように認証するか、および [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting#add-a-marketplace)を参照して、失敗時に表示されるメッセージを確認してください。

<h2 id="manage-marketplaces">
  マーケットプレイスを管理する
</h2>

`/plugin` の **Marketplaces** タブは、登録したすべてのマーケットプレイスをそのソースと共にリストアップします。1 つを選択して、そのプラグインを参照し、そのリストを更新し、自動更新をオンまたはオフにするか、削除します。

シェルまたはセッション内から、コマンドを使用してマーケットプレイスをリストアップ、更新、および削除することもできます。

| アクション            | シェルで                                      | セッション内で                             |
| :--------------- | :---------------------------------------- | :---------------------------------- |
| マーケットプレイスをリストアップ | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| マーケットプレイスのリストを更新 | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| マーケットプレイスを削除     | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

マーケットプレイスを削除すると、Claude Code はそこからインストールしたすべてのプラグインをアンインストールし、設定ファイルから `enabledPlugins` エントリを削除します。**Marketplaces** タブは、確認を求める前にそれらのプラグインに名前を付けます。

<h2 id="next-steps">
  次のステップ
</h2>

* [Anthropic のマーケットプレイス](/docs/ja/plugins/anthropic-marketplaces): 公式、コミュニティ、およびデモマーケットプレイスがどのように異なり、各マーケットプレイスを参照する場所
* [プラグイン読み込みリファレンス](/docs/ja/plugins/loading): プラグインが読み込まれた理由、読み込まれなかった理由、または更新後に変更されなかった理由
* [プラグインのセキュリティと信頼](/docs/ja/plugins/security): 知らないマーケットプレイスからプラグインをインストールする前に確認すること
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting): インストールおよびマーケットプレイスエラーメッセージとその修正
* [プラグインを作成](/docs/ja/plugins/create): 独自のプラグインを構築
