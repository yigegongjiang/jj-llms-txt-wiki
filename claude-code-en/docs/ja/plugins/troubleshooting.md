> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインのトラブルシューティング

> Claude Code でプラグインエラーを修正します。/plugin の実行からインストール、組織ポリシーまで、段階ごとにグループ化された正確なメッセージを見つけます。

このページでは、Claude Code プラグインおよびマーケットプレイス（Claude Code がプラグインをインストールするカタログ）のエラーメッセージと症状を一覧表示しています。各エントリは、原因、1 つの修正方法、および修正が機能した後に表示される内容を示しています。

メッセージがプラグインまたはマーケットプレイスに名前を付ける場合、エントリは `<name>` などのプレースホルダーを表示します。

プラグインをインストールする場合、構築する場合、マーケットプレイスをホストする場合、または組織のプラグインを管理する場合は、このページを使用してください。

<Note>
  これらのケースは他のページで説明されています。

  * **スコープ、キャッシュ、および優先度の動作方法**: [プラグイン読み込みリファレンス](/docs/ja/plugins/loading)を参照してください
  * **フラグ、フィールド、またはコマンドを検索する**: [プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference)、[マニフェストリファレンス](/docs/ja/plugins/manifest-reference)、または[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)を使用してください
</Note>

表示されたメッセージを検索してください。各メッセージは、実行したコマンドではなく、それを生成する段階の下に一覧表示されています。たとえば、マーケットプレイスが見つからないためにインストールが失敗する場合があるため、そのメッセージは[マーケットプレイスを追加する](#add-a-marketplace)の下に表示されます。

<h2 id="find-where-/plugin-runs">
  `/plugin` が実行される場所を見つける
</h2>

`/plugin` は、実行中の Claude Code ターミナルセッション内で入力するコマンドであり、インタラクティブパネルを開きます。このセクションのエントリは、入力できるが実行できない場所と、存在しないコマンドスペルをカバーしています。

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

`/plugin` を Claude Code ターミナルセッション以外の場所で入力し、Claude は何も開く代わりにこの行で返信しました。

`/plugin` パネルを描画するターミナルがないセッションでこの返信を取得します：`claude -p` を使用した[非インタラクティブモード](/docs/ja/headless)、Agent SDK、Claude デスクトップアプリの Code タブ、VS Code 拡張パネル、および claude.ai/code のブラウザ。

VS Code 拡張パネルでは、`/plugin install <plugin>@<marketplace>` など、その後に何かがある `/plugin` 行のみがこの返信を取得します。単独で入力された `/plugin` または `/plugins` は、**プラグインを管理**ダイアログを開きます。

代わりに、使用しているサーフェスからプラグインをインストールしてください：

* **Claude デスクトップアプリ、ローカルまたは SSH セッション**：プロンプトの横にある **+** ボタンをクリックし、**プラグイン**、**プラグインを追加**をクリックして[プラグインブラウザ](/docs/ja/desktop#install-plugins)を開きます
* **VS Code 拡張**：[プラグインをインストール](/docs/ja/plugins/install#install-a-plugin)の下の **VS Code** タブを使用してください
* **Web 上の Claude Code、またはデスクトップクラウドセッション**：クラウドセッションにはプラグインブラウザがありません。[プラグインをインストール](/docs/ja/plugins/install#install-a-plugin)の下の **Cloud session** タブを参照して、クラウドセッションが読み込むものを確認してください
* **アクセス権のあるターミナル**：`claude` を実行してそこで `/plugin` を入力するか、セッションを開始せずにシェルで `claude plugin install <plugin>@<marketplace>` を実行してください

ターミナルインストールが機能する場合、`/plugin` は `✓ Installed <plugin>.` で始まるインストール概要を出力し、`claude plugin install` は `Successfully installed plugin: <plugin>@<marketplace>` を出力します。

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

シェルプロンプトで `/plugin ...` を入力し、シェルが `/plugin` という名前のファイルが存在しないと報告しました。Bash は `bash: /plugin: No such file or directory` と報告します。

`/plugin` は Claude Code セッション内で入力するコマンドであり、シェルプロンプトではありません。セッションを開始して、そこで同じコマンドを入力してください：

```shell theme={null}
claude
```

次に、Claude Code プロンプトで：

```text theme={null}
/plugin install <plugin>@<marketplace>
```

成功したインストールは `✓ Installed <plugin>.` で始まる概要を出力します。インストール自体が失敗する場合、そのメッセージは[マーケットプレイスを追加](#add-a-marketplace)または[プラグインをインストール](#install-a-plugin)の下にあります。

セッションを開始せずにシェルからインストールするには、代わりに `claude plugin install <plugin>@<marketplace>` を実行してください。

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

PowerShell プロンプトで `/plugin ...` を入力し、`/plugin` は Claude Code コマンドであり、プログラムではありません。Bash と Zsh は[このエラーの独自の形式](#zsh-no-such-file-or-directory-plugin)を報告します。

代わりに、これらのいずれかを使用してください：

* `claude` を実行し、Claude Code プロンプトで `/plugin` を入力してください
* PowerShell でセッションを開始せずに `claude plugin install <plugin>@<marketplace>` を実行してください

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` after `claude plugin ...`
</h3>

シェルで `claude plugin install ...` を実行し、シェルが `claude` をまったく見つけられませんでした。Windows では、メッセージは `'claude' is not recognized as the name of a cmdlet` または `'claude' is not recognized as an internal or external command` です。

原因はプラグインコマンドではありません。Claude Code がインストールされていないか、このシェルの `PATH` にそのインストールディレクトリがありません。[インストール後の `command not found: claude`](/docs/ja/troubleshoot-install#command-not-found-claude-after-installation)に従い、プラグインコマンドを再試行してください。

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` と存在しないコマンドスペル
</h3>

どこかで見たプラグインコマンドを入力し、セッションで `Unknown command: /<name>` を取得したか、シェルの `claude` バイナリから `error: unknown command '<name>'` または `error: unknown option '<flag>'` を取得しました。

Claude Code が持たないいくつかのコマンドスペルが使用されています。下の表は、各スペルを実際のコマンドにマップします。[プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference)はすべてのサブコマンドとフラグを一覧表示しています。

| 入力したもの                                     | Claude Code が言うこと                                                            | 代わりに使用してください                                                                                                                 |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | マーケットプレイスを追加するには `claude plugin marketplace add <source>`、またはプラグインをインストールするには `claude plugin install <plugin>@<marketplace>` |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                               |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                     |
| `/plugin add <source>`                     | `/plugin` パネルが **Discover** タブで開きます                                          | `/plugin marketplace add <source>`                                                                                           |
| `marketplace.anthropic.com` をソースとして        | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | 公式マーケットプレイスの場合は `anthropics/claude-plugins-official`                                                                         |

これらのスペルは間違っているように見えますが、機能します：

* `claude plugins` は `claude plugin` のエイリアスです
* `claude plugin remove` は `claude plugin uninstall` のエイリアスです
* セッション内の `/plugins` と `/marketplace` は `/plugin` と同じパネルを開きます

<h2 id="add-a-marketplace">
  マーケットプレイスを追加
</h2>

マーケットプレイスは、git リポジトリ、URL、またはローカルパスから Claude Code に追加するカタログです。これらのエントリは、追加が失敗するか、後で更新が失敗するときに取得するメッセージをカバーしています。

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

セッションで `/plugin install <plugin>@claude-plugins-official` を実行し、Claude Code はこの名前のマーケットプレイスがないと報告しました。

公式マーケットプレイスはまだこのマシンに登録されていません。Claude Code は通常、インタラクティブターミナルセッションを初めて開始するときに自動的に登録します。VS Code 拡張を通じてのみ Claude Code を使用した場合、またはそのステップをスキップまたは延期する場合は実行されていません：

* ポリシーがソースをブロックする場合
* `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` が設定されている場合
* 再試行を待機している失敗した試行の後

`claude plugin` シェルコマンドは決してそれを登録しません。

追加してから、インストールを再試行してください：

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code は `Successfully added marketplace: claude-plugins-official` を出力し、`/plugin marketplace list` はマーケットプレイスをそのソースと共に表示します。

このメッセージの他のマーケットプレイス名については、[`Marketplace "<name>" not found`](#marketplace-not-found)を参照してください。

同じ文字列は、パネルの読み込み失敗のリストである `/plugin` **Errors** タブにも表示されます。設定で名前が付けられたプラグインが追加していないマーケットプレイスを名前付けする場合。

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

セッションで `/plugin install <plugin>@<name>` を実行し、多くの場合は誰かが送信したインストール行から、Claude Code はこの名前のマーケットプレイスがないと報告しました。

名前が `claudeai-` で始まる場合、マーケットプレイスは claude.ai でホストされており、シェルから `claude plugin marketplace add --claudeai <name>` で名前で追加します。[claude.ai からマーケットプレイスを追加](/docs/ja/plugins/install#add-from-claude-ai)を参照してください。

他の名前については、インストール行はマーケットプレイスの名前を付けますが、マーケットプレイスがホストされている場所は言いません。Claude Code にはマーケットプレイス名を検索するインデックスがありません。行を送信した人にマーケットプレイスのソースを尋ねてください。これは GitHub `owner/repo`、git URL、またはパスです。次に[マーケットプレイスを追加](/docs/ja/plugins/install#add-a-marketplace)し、インストール行を再度実行してください。

誰かが送信したマーケットプレイスはサードパーティであるため、[インストール前にプラグインを確認](/docs/ja/plugins/security#review-a-plugin-before-you-install)してください。

既にマーケットプレイスを追加した場合は、`/plugin marketplace list` に対してスペルを確認してください。

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

`/plugin marketplace add <source>` または `claude plugin marketplace add <source>` を実行し、Claude Code は `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` で返信しました。

Claude Code は、次のいずれかの形式でソースを受け入れます：

* GitHub `owner/repo` ショートハンド
* `https://` または `http://` URL
* `user@host:path` SSH URL
* `./`、`../`、`/`、または `~` で始まるローカルパス

`claude-plugins-official` などの裸の名前はそれらのいずれにも一致しません。`marketplace.anthropic.com` などの裸のホスト名も同様です。

ソースを受け入れられた形式の 1 つで再入力してください：

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

追加が機能する場合、Claude Code は `Successfully added marketplace: <name>` を出力します。

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

`github.com/owner/repo` または `gitlab.example.com/group/project` パスなど、`owner/repo` ではないスラッシュを含むソースを渡しました。Claude Code はこのメッセージと受け入れられた形式のリストでそれを拒否しました。

`owner/repo` ショートハンドは GitHub のみであり、GitHub の命名規則に従う必要があるため、ホスト名または追加のパスセグメントが失敗します。マーケットプレイスがホストされている場所に一致する形式でソースを渡してください：

* **任意のホスト上のリポジトリ**：完全なクローン URL
* **ホストされた `marketplace.json`**：その `https://` URL
* **ローカルチェックアウト**：`./path` または絶対パス

たとえば、公式マーケットプレイスをそのクローン URL で追加するには、セッションで：

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

成功した追加は `Successfully added marketplace: <name>` を出力します。

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

`marketplace add` にローカルパスを渡し、そのパスに何も存在しません。相対パスは現在のディレクトリに対して解決されます。

メッセージで解決されたパスを確認してください。次に、相対パスが開始するディレクトリからコマンドを実行するか、マーケットプレイスディレクトリへの絶対パスを渡してください。成功した追加は `Successfully added marketplace: <name>` を出力します。

Claude Code は `.claude-plugin/marketplace.json` を含むディレクトリ、または `.json` ファイルへのパスを受け入れます。他のファイルへのパスは `File path must point to a .json file (marketplace.json)` で失敗します。

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code はマーケットプレイスをクローンまたはダウンロードしましたが、その内部の予想されるパスに `marketplace.json` が見つかりませんでした。追加コマンドは `Failed to add marketplace: Marketplace file not found at ...` として報告します。

デフォルトの場所はリポジトリルートの `.claude-plugin/marketplace.json` であり、[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)は受け入れられた場所を一覧表示しています。

修正は所有者と他の人で異なります：

* **マーケットプレイスを所有している場合**：ファイルをその場所に配置し、マーケットプレイスを再度追加してください
* **他の誰かがホストしている場合**：所有者に正確なソースを尋ねてください

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` または `HTTPS authentication failed`
</h3>

git リポジトリからマーケットプレイスを追加または更新し、クローンが `Failed to clone marketplace repository:` で失敗し、その後にこれらの行のいずれかが続きました。

まずリポジトリ自体を確認してください：スペルが間違った `owner/repo`、存在しないリポジトリ、またはアクセスできないプライベートリポジトリもこのメッセージで終わります。ブラウザでリポジトリ URL を開くか、ターミナルで `git ls-remote <url>` を実行して、それが存在し、アクセス権があることを確認してください。

リポジトリが正しい場合、原因は認証情報です。Claude Code は git をインタラクティブプロンプト無効で実行するため、パスワード、キーパスフレーズ、またはターミナルが行うような認証情報を要求できません。git がプロンプトを必要とする場合、`fatal: Cannot prompt because user interactivity has been disabled` または `terminal prompts disabled` が元のエラーに表示されます。既に非対話的に機能する認証情報のみが成功します：

* **SSH**：`ssh -T git@<host>` はパスフレーズを要求せずに成功する必要があり、ホストは既に `known_hosts` にある必要があります
* **HTTPS**：認証情報ヘルパーはホストのトークンを保持する必要があります。GitHub の場合は、`gh auth login` と `gh auth setup-git` を実行してください。別のホストの場合は、個人用アクセストークンを git 認証情報ヘルパーに保存してください。`git ls-remote <url>` でテストしてください

ターミナルで `git ls-remote` がプロンプトなしで成功したら、追加または更新を再度実行してください。成功した追加は `Successfully added marketplace: <name>` を出力します。成功した更新はシェルから `Successfully updated marketplace: <name>` を出力するか、セッションで `✔ Updated 1 marketplace` を出力します。

GitHub `owner/repo` ソースの SSH をスキップするように Claude Code を設定するには、`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` を設定してください。これがないと、Claude Code は `github.com` の SSH キーが設定されているように見える場合、これらのソースを SSH 経由でクローンし、SSH クローンが失敗するときに HTTPS にフォールバックします。

バックグラウンド自動更新が認証情報で何ができるか、できないかについては、[バックグラウンド自動更新が認証情報で行うこと](/docs/ja/plugins/host-marketplace#what-background-auto-update-does-with-credentials)を参照してください。

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

接続したことのないホストから SSH 経由でマーケットプレイスを追加し、クローンがこの行と `ssh -T git@<host>` ヒントで失敗しました。キーが変更されたホストの場合、メッセージは `SSH host key has changed` で、代わりに `ssh-keygen -R <host>` ヒントが表示されます。

Claude Code は `StrictHostKeyChecking=yes` でクローンするため、キーを自動的に受け入れるのではなく、まだ受け入れていないホストを拒否します。ターミナルから 1 回接続してフィンガープリントを受け入れ、再試行してください：

```shell theme={null}
ssh -T git@github.com
```

パブリックリポジトリの場合は、SSH を完全に回避するために、代わりにマーケットプレイスを `https://` URL で追加してください。

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

Windows では、マーケットプレイスを追加し、Claude Code は `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)` と報告しました。

Claude Code は `PATH` で `git` を探し、現在のディレクトリでのみ見つかったものを実行することを拒否します。修正するには、Git をインストールして再試行してください：

<Steps>
  <Step title="Git for Windows をインストール">
    Git for Windows をインストールして、`git` が `PATH` 上にあるようにしてください。
  </Step>

  <Step title="新しいターミナルを開く">
    新しいターミナルを開いて、更新された `PATH` が適用されるようにしてください。
  </Step>

  <Step title="git が実行されることを確認">
    `git --version` がバージョンを出力することを確認してください。
  </Step>

  <Step title="追加を再試行">
    `marketplace add` コマンドを再度実行してください。
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

マーケットプレイスを追加または更新し、`Git clone timed out after 120s` で失敗し、その後に `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS` を設定するヒントが続きました。

マーケットプレイスのクローンと、更新するために再クローンすることは、デフォルトで 120 秒を取得します。大規模なリポジトリまたは遅い接続の場合は、制限を上げてください。値はミリ秒単位です：

<Tabs>
  <Tab title="Bash または Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

次に、同じシェルで再試行してください。

リポジトリがモノレポの場合は、`claude plugin marketplace add <source> --sparse <paths>` で名前を付けたディレクトリにチェックアウトを制限してください。

<h3 id="marketplace-updates-keep-failing-offline">
  マーケットプレイスの更新がオフラインで失敗し続ける
</h3>

マーケットプレイスの git ホストに到達できない環境で作業しており、すべてのセッションがバックグラウンドで失敗した更新を繰り返します。マーケットプレイスの既存のチェックアウトは所定の位置に留まり、スタートアップは遅延しません。

各セッション、[自動更新がオン](/docs/ja/plugins/loading#which-marketplaces-and-plugins-auto-update)のマーケットプレイスの場合、Claude Code はバックグラウンドでマーケットプレイスの git ホストをチェックして新しいコミットを確認します。そのチェックがホストに到達できない場合、マーケットプレイスを再度クローンしようとし、オフラインではそのクローンも失敗します。

この変数を設定して、チェックがホストに到達できない場合の再クローン試行をスキップし、既存のチェックアウトを使用し続けてください：

<Tabs>
  <Tab title="Bash または Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

変数が設定されている場合、Claude Code は `.claude-plugin/marketplace.json` を既に含むチェックアウトの再クローンのみをスキップします。クローンされたことのないマーケットプレイスまたはクローンが途中で停止したマーケットプレイスは、クローン試行を取得するため、オンライン中に 1 回追加してください。

完全にオフラインの展開の場合は、代わりに `CLAUDE_CODE_PLUGIN_SEED_DIR` を使用してイメージビルド時にプラグインディレクトリを事前に入力してください。[コンテナと CI をシード](/docs/ja/plugins/org#seed-containers-and-ci)に従ってください。

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  GitHub Enterprise Server ホストでマーケットプレイス追加が失敗
</h3>

GitHub Enterprise Server（GHES）URL からマーケットプレイスを追加し、ポリシーエラーを取得したか、claude.ai から追加し、GitHub アクセスエラーを取得しました。

両方のケースは GHES ページにあります：

* [ポリシーエラー](/docs/ja/github-enterprise-server#marketplace-add-fails-with-a-policy-error)は、組織がマーケットプレイスソースを制限し、管理者がホストの `hostPattern` を追加する必要があることを意味します
* [claude.ai での GitHub アクセスエラー](/docs/ja/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error)は、独自の GitHub Enterprise アカウントがまだ接続されていないことを意味します

<h2 id="install-a-plugin">
  プラグインをインストール
</h2>

マーケットプレイスを追加してインストールを実行し、インストールが何かをインストールする代わりにメッセージで停止しました。これらのエントリはそれらのメッセージをカバーしています。また、プラグインまたはそのマーケットプレイスが見つからない、読み込めない、または信頼できない場合に、後で `/plugin` **Errors** タブに表示される関連メッセージ、または空の **Discover** タブもカバーしています。

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

`/plugin install <name>@<marketplace>` または `claude plugin install <name>@<marketplace>` を実行し、プラグイン名がマシン上のそのマーケットプレイスのカタログのコピーにありません。

シェルで `claude plugin install` を実行し、マーケットプレイスをまったく追加していない場合、同じメッセージを出力します。`claude plugin marketplace update <marketplace>` が `Marketplace '<marketplace>' not found` で答える場合、[マーケットプレイスを追加](#add-a-marketplace)してください。

<h4 id="the-message-ends-with-a-refresh-hint">
  更新ヒント付きの `not found in marketplace`
</h4>

ヒントは `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` または `The marketplace couldn't be refreshed (...)` を読みます。Claude Code はマーケットプレイスをオフラインの場合など、ルックアップの前に更新しなかったため、カタログのコピーが古い可能性があります。マーケットプレイスの名前で更新し、再度インストールしてください：

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` は `Successfully updated marketplace: <name>` を出力し、`/plugin marketplace update` は `✔ Updated 1 marketplace` を表示します。再試行されたインストールが同じメッセージを出力する場合、[ヒントなしの `not found in marketplace`](#the-message-has-no-hint)が説明するように名前を確認してください。[Claude Code がインストール前にマーケットプレイスを更新する場合](/docs/ja/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install)は、更新が実行されない他のケースを一覧表示しています。

<h4 id="the-message-has-no-hint">
  ヒントなしの `not found in marketplace`
</h4>

名前が最も可能性の高い問題です。`/plugin` を開き、**Discover** に移動し、リストから名前をコピーしてください。

v2.1.232 より前では、Claude Code はルックアップが失敗した後にのみ名前付きマーケットプレイスを更新し、自動更新がオンの場合のみでした。

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

`@marketplace` なしで `/plugin install <name>` を実行し、登録されたマーケットプレイスにそのプラグインがありません。`claude plugin install <name>` は `Plugin "<name>" not found in any configured marketplace` を報告します。

マーケットプレイス名がない場合、`claude plugin install` は既に持っているカタログを検索し、最初に更新しません。`/plugin install` は自動更新がオンのマーケットプレイスのみを更新します。マーケットプレイス名を付けると、Claude Code はルックアップの前に更新します：

```text theme={null}
/plugin install <name>@<marketplace>
```

インストールが機能する場合、セッションで `✓ Installed <plugin>.` を表示するか、`claude plugin install` から `Successfully installed plugin: <plugin>@<marketplace>` を表示します。

どのマーケットプレイスがプラグインをリストしているかわからない場合は、`/plugin marketplace list` を実行して、持っているマーケットプレイスを確認し、`/plugin` の **Discover** でプラグイン名を参照してください。

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

既にユーザースコープまたは管理設定でインストールされているプラグインの `/plugin install` を実行し、Claude Code は `Use '/plugin' to manage existing plugins.` で拒否しました。プラグイン名を `@<marketplace>` なしで入力した場合、メッセージは `globally` を省略します。

プラグインはすべてのプロジェクトで既に利用可能であるため、追加するものはありません。その[スコープ](/docs/ja/plugins/install)を変更したり、有効または無効にしたり、設定したりするには、`/plugin` を開いて **Installed** に移動してください。

プロジェクトまたはローカルスコープでのみインストールされたプラグインはこのメッセージをトリガーしません。Claude Code はユーザースコープでもインストールできるため、他のプロジェクトで利用可能です。

シェルで `claude plugin install` は別のメッセージを出力します。ターゲットスコープで既にインストールされているプラグインの場合、`Plugin "<name>@<marketplace>" is already installed (scope: user)` を出力して終了 0 で終了します。キャッシュディレクトリが見つからない場合、同じコマンドは再度ダウンロードします。

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

マーケットプレイスエントリがこのバージョンの Claude Code がフェッチできないソースタイプを使用するプラグインをインストールし、Claude Code はこのメッセージと `Update Claude Code and try again.` で停止しました。

Claude Code を更新し、インストールを再試行してください。ソースタイプは[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)にあります。

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

zip アーカイブとして配布されるプラグインをインストールし、Claude Code はこの行と `The archive was not installed.` で拒否しました。プラグインのマーケットプレイスエントリは `sha256` ピン付きの [`archive` ソース](/docs/ja/plugins/marketplace-reference)を使用し、ダウンロードされたファイルのダイジェストはピンと一致しません。

完全なメッセージは次のようになります：

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

修正は発行者とインストーラーで異なります：

* **プラグインを発行する場合**：URL が提供する正確なファイルのダイジェストを再計算し、マーケットプレイスエントリの `sha256` を更新してください。`shasum -a 256 my-plugin.zip` を使用するか、PowerShell で `Get-FileHash -Algorithm SHA256 my-plugin.zip` を使用してください
* **プラグインをインストールする場合**：セッションで `/plugin marketplace update <name>` を実行してカタログを更新し、エントリが修正された場合に備えて、インストールを再試行してください。更新後もダイジェストが一致しない場合は、インストール前にマーケットプレイス所有者にピン留めされたファイルを尋ねてください

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

以前に追加したマーケットプレイスが読み込みを停止し、そのプラグインも同様です。この行は `/plugin` **Errors** タブまたは次の更新に表示されます。

マーケットプレイスは[公式 Anthropic マーケットプレイス用に予約されている](/docs/ja/plugins/marketplace-reference)名前で登録されていますが、登録されたソースは `anthropics` GitHub リポジトリではありません。予約された名前はマーケットプレイスが読み込まれるか更新されるたびに再チェックされるため、マーケットプレイスとそれからインストールされたプラグインは読み込みを停止します。

完全なメッセージは予約された名前と修正を名前付けします：

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

修正はユーザーと発行者で異なります：

* **マーケットプレイスを使用する場合**：シェルで `claude plugin marketplace remove <name>` を実行し、公式 `github.com/anthropics` リポジトリからマーケットプレイスを再度追加してください
* **名前が予約される前に名前を使用したサードパーティマーケットプレイスを発行する場合**：名前を変更し、ユーザーにソースから再度追加するよう依頼してください

v2.1.205 より前では、Claude Code はマーケットプレイスを追加するときのみ名前をチェックしたため、名前が予約される前に登録されたエントリは読み込みを続けました。

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` または `has an invalid manifest file`
</h3>

Claude Code はプラグインをフェッチしましたが、その `.claude-plugin/plugin.json` を読み込めませんでした。シェルでは、この行の `<name>` は一時ディレクトリ名である可能性があります。`Failed to install plugin "<name>@<marketplace>"` プレフィックスはプラグインの実際の名前を運びます。表現は失敗したチェックを示します：

* **`corrupt manifest file`、その後に `JSON parse error:`**：ファイルは有効な JSON ではありません
* **`invalid manifest file`、その後に `Validation errors:`**：ファイルは解析されますが、スキーマに失敗します。例えば、必須フィールドが見つからない場合の `name: Invalid input`

`claude plugin install` は `Failed to install plugin "<name>@<marketplace>":` として報告し、コード 1 で終了します。

プラグインの作成者がファイルを修正する必要があり、その後までプラグインをインストールできません：

* **それがあなたの場合**：シェルで `claude plugin validate <plugin-directory>` を実行して、問題のあるパスで同じエラーを表示し、ファイルを修正してください
* **そうでない場合**：メッセージをマーケットプレイス所有者に報告してください

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

`/plugin` の **Errors** タブは、マーケットプレイスが `./plugins/my-plugin` などの相対パスでリストする有効なプラグインに対してこれを表示し、マーケットプレイス内のそのパスにディレクトリが存在しません。マーケットプレイスを維持する場合は、エントリの `source` パスを修正するか、フォルダを復元してください。それ以外の場合は、メッセージをマーケットプレイス所有者に報告してください。

`Marketplace directory not found at path: <path>` は、代わりにマーケットプレイス自体のディレクトリが見つからないことを意味します。ローカルパスから追加したマーケットプレイスの場合、そのディレクトリが移動または削除されました。復元するか、マーケットプレイスを削除して新しい場所から再度追加してください。

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` または `No marketplaces configured`
</h3>

`/plugin` を開き、**Discover** タブが空であるか、`claude plugin marketplace list` が `No marketplaces configured` を出力しました。

マーケットプレイスが登録されていないため、表示するカタログがありません。セッションで、公式マーケットプレイス `anthropics/claude-plugins-official` を追加してください：

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code は `Successfully added marketplace: claude-plugins-official` を出力し、**Discover** はそのプラグインをリストします。[Anthropic マーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)ページは、追加できる他のマーケットプレイスをリストしています。

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

[`/plugin install <plugin> --marketplace <source>`](/docs/ja/plugins/install#add-a-marketplace-and-install-in-one-command)を通じてマーケットプレイスの追加を確認し、Claude Code がそのソースからフェッチしたカタログは、別のソースから既に追加したマーケットプレイスと同じ名前を持っています。Claude Code は既存のマーケットプレイスを保持し、プラグインをインストールしません。

完全なメッセージは次のようになります：

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

どのソースを使用するかを選択してください：

* **既に追加したマーケットプレイス**：`/plugin install <plugin>@<name>` で名前でインストールしてください
* **新しいソース**：`/plugin marketplace remove <name>` を実行し、インストールを再試行してください

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

`marketplace add` を実行し、そのソースのカタログは、設定ファイルが [`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) で異なるソースで既に宣言しているマーケットプレイスと同じ名前を持っています。Claude Code は追加を拒否し、何も登録しません。

メッセージは修正で終わります：ソースは設定で宣言されたものと一致する必要があります。または、宣言を変更します。渡したソースを `extraKnownMarketplaces` エントリと比較し、その `ref`、`path`、`headers` を含めて、次のいずれかを実行してください：

* **宣言されたソースを使用**：設定エントリが名前を付けるソースからマーケットプレイスを追加してください
* **新しいソースを使用**：`extraKnownMarketplaces` エントリを編集または削除し、マーケットプレイスを再度追加してください。管理設定がそれを宣言する場合は、管理者に尋ねてください

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

`/plugin` メニューでインストールするプラグインを選択し、それらのいずれもインストールされず、メニューは失敗した内容の概要で閉じました。

git の出力など、いくつかの理由は最初の行のみを表示します。そのような理由が短縮された場合、概要は `Installing a plugin from its details (Enter) in /plugin shows its full error.` で終わります。

何をするかは、概要が理由を短縮したかどうかによって異なります：

* 括弧内の理由が名前を付けるものを修正してください
* 理由が短縮された場合、`/plugin` を実行し、**Discover** タブでプラグインを選択し、**Enter** を押してその詳細からインストールしてください。インストールがそこで失敗する場合、詳細ビューは完全なエラーを表示します

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

プラグインをインストールする場合、Claude Code はそのファイルの新しいコピーをダウンロードし、[プラグインキャッシュ](/docs/ja/plugins/loading#find-plugins-on-disk)のそのバージョンのフォルダに移動します。このメッセージは移動が失敗したことを意味し、通常は別のプログラムがインストール中にフォルダを使用していたためです。ファイルシステムコードは括弧内に表示されます：

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

メッセージは、インストール前にインストールされたコピーに何が起こったかを示し、プラグインが引き続き機能するかどうかを示します：

* `The previously installed copy was moved back`：持っていたバージョンはまだインストールされています
* `had to be removed first`、`was not moved back`、または `could not be moved back`：そのプラグインバージョンはインストールが成功するまでインストールされません
* そのような文がない：以前のコピーがなかったため、バージョンはまだインストールされていません

Windows では、別のプログラムがインストールされたコピー自体を保持している場合、メッセージは代わりにそのコピーが `could not be replaced` であり、`It was not replaced and the new copy was discarded` であると言うため、持っていたバージョンはまだインストールされています。

`Left on disk` リストはキャッシュ内に設定されたフォルダに名前を付けます。そのバージョンの後のインストールまたはプラグインキャッシュクリーンアップはそれらを削除するため、削除する必要はありません。

インストールを修正するには：

* `~/.claude/plugins/cache` の下のプラグインのフォルダを使用している他の Claude Code セッション、エディタ、ターミナルを閉じ、インストールを再度実行してください
* メッセージがプラグインキャッシュフォルダのアクセス許可を確認するよう指示する場合は、名前を付けたフォルダの書き込みアクセス許可を復元し、ディスク領域を解放し、インストールを再度実行してください

<h3 id="dependency-errors">
  依存関係エラー
</h3>

依存関係を宣言するプラグインは、依存関係を満たすことができない場合、インストールに失敗するか、インストールして無効のままになる可能性があります。メッセージはインストール時または読み込み時に到達します：

* **インストール中**：拒否はインストールのエラーメッセージとして返されます
* **プラグインが読み込まれるとき**：問題は `claude plugin list` と `/plugin` **Errors** タブに表示され、Claude Code は解決するまで影響を受けたプラグインを無効のままにします

表は各メッセージとその修正をリストしています。作成者として依存関係を宣言するには、[プラグイン依存関係](/docs/ja/plugins/dependencies)を参照してください。

| メッセージ                                                                                            | 意味                                             | 解決方法                                                                                                                                                                                      |
| :----------------------------------------------------------------------------------------------- | :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Dependency "<dep>" is not installed`                                                            | 宣言された依存関係がインストールされていません。                       | シェルで `claude plugin install <dep>@<marketplace>` でインストールするか、プラグインをアンインストールしてください。依存関係のマーケットプレイスがまだ登録されていない場合は、それを追加し、セッションで `/reload-plugins` を実行してください。これにより、解決できる不足している依存関係がインストールされます。 |
| `Dependency "<dep>" is disabled`                                                                 | 依存関係はインストールされていますが、オフになっています。                  | 依存関係を有効にするか、それを必要とするプラグインをアンインストールしてください。                                                                                                                                                 |
| `Requires "<dep>" <range>, installed <version>`                                                  | インストールされた依存関係のバージョンはプラグインの宣言された範囲外です。          | 依存関係を範囲内のバージョンに更新するか、プラグインをアンインストールしてください。                                                                                                                                                |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                           | バージョンはすべての範囲を満たしていません。メッセージは範囲をリストします。         | 競合するプラグインの 1 つをアンインストールまたは更新するか、上流の作成者に制約を広げるよう依頼してください。                                                                                                                                  |
| `... has version requirements too complex to intersect` または `has an invalid version requirement` | 範囲は有効な semver ではないか、結合された範囲を交差させることができません。     | 無効な範囲を修正するか、長い `\|\|` チェーンを簡素化してください。                                                                                                                                                     |
| `... has no git tag satisfying <range>`                                                          | 依存関係のリポジトリには範囲内に `<name>--v*` タグがありません。        | 上流がその規約でリリースをタグ付けしていることを確認するか、範囲を緩和してください。                                                                                                                                                |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist`   | 依存関係は別のマーケットプレイスにあり、クロスマーケットプレイス解決はデフォルトでオフです。 | 依存関係を自分でインストールしてください。シェルで `claude plugin install <dep>@<marketplace>` に加えて、プラグインをインストールしている `--scope` を使用してから、再試行してください。                                                                  |

これらをプログラムで表示するには、シェルで `claude plugin list --json` を実行してください。問題のあるプラグインは、メッセージを含む `errors` フィールドと、各フィールドの `type` を含む `errorDetails` フィールドを運びます：最初の 2 行は `dependency-unsatisfied` で、3 番目は `dependency-version-unsatisfied` です。

<h2 id="plugin-installed-but-not-working">
  プラグインがインストールされているが機能していない
</h2>

インストールは成功しましたが、プラグインのスキル、フック、またはサーバーは何もしていません。[プラグインが表示されないか、そのスキルが表示されない](#plugin-doesnt-appear-or-its-skills-dont-show-up)から始めてください。これは Claude Code が読み込んだものを報告する場所を示し、メッセージを一致させてください。

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  プラグインが表示されないか、そのスキルが表示されない
</h3>

プラグインをインストールし、`/` を入力してそのスキルを期待したか、Claude にそれを使用するよう依頼し、何も起こりませんでした。

何かを変更する前に、プラグインの状態を確認してください：

<Steps>
  <Step title="プラグインがインストールされ、有効になっていることを確認">
    `/plugin` を実行して **Installed** を開きます。プラグインがリストされ、有効になっていることを確認してください。シェルで `claude plugin list` は各プラグインのバージョン、スコープ、`Status: ✔ enabled` を含む同じリストを出力します。
  </Step>

  <Step title="Errors タブを読む">
    同じパネルで **Errors** タブを開きます。各エントリはメッセージをガイダンス行と組み合わせます。このセクションの残りのほとんどのメッセージはそのタブから来ています。
  </Step>

  <Step title="このセッション中にインストールした場合は再度読み込む">
    プラグインがインストールされ、エラーがなく、このセッション中にインストールした場合は、`/reload-plugins` を実行してください。プラグイン、スキル、エージェント、フック、サーバーの数で `Reloaded:` を出力します。何かが失敗した場合、`N errors during load. Run /plugin for details.` を追加します。
  </Step>
</Steps>

プラグインがエラーなく読み込まれ、そのスキルがまだ表示されない場合、次のステップは独自のプラグインと他の人のプラグインで異なります：

* **構築しているプラグイン**：[プラグインが読み込まれているがそのスキルが見つからない](#plugin-loads-but-its-skills-are-missing)を参照してください
* **他の誰かが発行したプラグイン**：`/plugin` で **Installed** を開き、プラグインの詳細ペインを開きます。これはプラグインに含まれるものをリストします。そこにスキルをリストしないプラグインは、`/` を入力するときに提供するスキルがありません

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

`/plugin` のインストール概要は `Plugin is now active.` の代わりに `Run /reload-plugins to activate.` で終わりました。

Claude Code はインストール中にプラグインを有効にしませんでした。これは、有効化が[プロンプトキャッシュを無効にする](/docs/ja/prompt-caching#enabling-or-disabling-a-plugin)か、有効化の試みが失敗したためです。

コマンドを入力する必要はありません。パネルが閉じ、Claude Code が `/reload-plugins` を実行するか、ストリーミングが終了する応答までキューに入れます。

その再度読み込みが出力するものを読んでください：

* **プラグイン、スキル、エージェント、フック、サーバーの数で `Reloaded:`**：プラグインはアクティブです。何かが読み込みに失敗した場合、行は `N errors during load. Run /plugin for details.` を追加します。
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**：再度読み込みはプラグイン MCP サーバーを追加または削除するか、`LSP` ツールを追加または削除し、プロンプトキャッシュを無効にします。LSP ケースの場合、行は `This reload adds the LSP tool` または `This reload removes the LSP tool` で始まります。`--force` で実行してプラグインを有効にするか、新しいセッションを開始してください

v2.1.268 より前では、インストール中に有効にされなかったインストールは、自分で `/reload-plugins` を実行するまで保留中のままでした。

v2.1.246 より前では、その概要のスキル数にはプラグインの `commands/` エントリのみが含まれていたため、再度読み込みはプラグインの `SKILL.md` スキルを読み込むことができ、それでも `0 skills` を報告できました。

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

**Errors** タブはこの行をガイダンス `Run /plugin to refresh the plugin cache` で表示します。Claude Code はプラグインのインストール記録を持っていますが、記録が指すディレクトリが見つかりません。例えば、キャッシュをクリアした後。

シェルからプラグインを再度インストールしてください。`claude plugin install <name>@<marketplace>` は、インストールディレクトリが見つからないプラグインを再度ダウンロードします。記録は存在します：

```shell theme={null}
claude plugin install <name>@<marketplace>
```

次に、セッションで `/reload-plugins` を実行してください。**Errors** タブエントリが消え、プラグインは **Installed** の下に戻ります。

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

`~/.claude/settings.json` でプラグインを `false` に設定し、`claude plugin list` または `/plugin` の行がこのメッセージを表示し、その後に `— project settings enable it, which overrides your user setting` などのソースが続きます。その高い優先度のソースの `true` はユーザー設定をオーバーライドしています。

マシンでプロジェクト対応プラグインをオプトアウトするには、`.claude/settings.local.json` で id を `false` に設定してください。これはプロジェクトファイルより優先度が高いです。メッセージが名前を付けることができる他のソースについては、[ユーザー設定で無効になっているが引き続き読み込まれる](/docs/ja/plugins/loading#disabled-in-user-settings-but-still-loads)を参照してください。

`claude plugin list` が代わりにプラグインを `required by your org` とマークする場合、設定ファイルは関係ありません：組織は claude.ai で同期されたプラグインを必須としてマークし、以前に無効にした場合でも読み込みます。[claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins)を参照してください。

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

**Errors** タブはプロジェクトの `.claude/settings.json` が有効にするプラグインに対してこの行を表示し、ガイダンス `Run claude plugin install <name>@<marketplace> --scope project to install it for this project` が続きます。

リポジトリの設定はすべての人がそれを開くためにプラグインを有効にすることができますが、インストールしません。プラグインが GitHub リポジトリや npm パッケージなどの外部ソースから来る場合、Claude Code はインストールするまでダウンロードしません。ガイダンス行からコマンドをシェルで実行し、再度読み込んでください：

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

セッションで `/reload-plugins` を実行した後、**Errors** タブエントリは消え、プラグインは **Installed** の下にリストされます。

組織がプラグインを事前にインストールする場合、代わりに管理設定を通じて行います。[プラグインを事前にインストールして必須にする](/docs/ja/plugins/org#pre-install-and-require-plugins)を参照してください。

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` とフックが発火しない
</h3>

プラグインのフックが実行されません。**Errors** タブが読み込み失敗を表示するか、フックが読み込まれ、トランスクリプトで `<Event> hook error` 通知が表示されるか、フックがエラーなく読み込まれ、決して発火しません。

<h4 id="hooks-fail-to-load">
  フックが読み込みに失敗
</h4>

**Errors** タブは次のいずれかのメッセージを表示します：

* **`Failed to load hooks from <path>: <reason>`**：`hooks/hooks.json` は有効な JSON ではないか、フックスキーマに失敗します。理由は解析または検証エラーに名前を付けます。ファイルを修正してください。プラグインを発行する前に `hooks/hooks.json` の JSON 構文の問題をキャッチするには、シェルで `claude plugin validate <plugin-directory>` を実行してください
* **`hooks path not found: <path>`**：マニフェストの `hooks` フィールドは、プラグインルートに対してそのパスに存在しないファイルに名前を付けます。パスを修正するか、ファイルを追加してください

<h4 id="hook-error-notices-in-the-transcript">
  トランスクリプトの `hook error` 通知
</h4>

`... hook error: Failed with non-blocking status code: <stderr>` の形式の通知は、フックが実行され、そのコマンドが失敗したことを意味します。例えば、`Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` は、Claude Code が生成したシェルが `node` を見つけられなかったことを意味します。インストールするか、`claude` を開始するターミナルの `PATH` にあることを確認してください。

他のエラーについては、プラグインディレクトリからフックのコマンドを自分で実行して完全な出力を表示するか、[デバッグログ](/docs/ja/hooks#debug-hooks)で完全な stderr をキャプチャしてください。

<h4 id="hook-loads-but-never-fires">
  フックが読み込まれるが決して発火しない
</h4>

フックがエラーなく読み込まれるが決して発火しない場合は、その定義を確認してから、それが実行されるのを見てください：

<Steps>
  <Step title="イベント名を確認">
    イベント名は大文字と小文字を区別するため、例えば `PostToolUse` など、正確に一致することを確認してください。
  </Step>

  <Step title="マッチャーを確認">
    フックの `matcher` がツール名と一致することを確認してください。
  </Step>

  <Step title="イベントを意図的にトリガー">
    `PostToolUse` フックの場合は、Claude にファイルを編集するよう依頼してください。
  </Step>

  <Step title="デバッグログを読む">
    [デバッグログ](/docs/ja/hooks#debug-hooks)を開きます。これはどのフックが一致したかを記録します。実行されたフックは終了コードで表示されます。
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` と MCP サーバーが開始しない
</h3>

プラグインは MCP サーバーをバンドルし、**Errors** タブは `Invalid MCP server config for "<server>": <error>` を表示するか、サーバーはリストされていますが `/mcp` は決して接続を表示しません。

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

サーバーの設定はスキーマチェックに合格しますが、Claude Code はこのセッションに対してそれを解決できません。コロンの後のテキストは原因に名前を付け、修正を決定します：

* **`Missing environment variables: <names>`**：Claude Code を開始するシェルでそれらの変数を設定し、新しいセッションを開始してください
* **`URL is unset or invalid`**：URL が使用する `${user_config.*}` オプションが設定されていません。`/plugin configure <plugin>` を実行して設定してください
* **`has an invalid MCP url`** または **`headersHelper for MCP server '<server>' references ${user_config.*}`**：プラグイン自体の設定が問題です。プラグインの MCP 設定の `url` または `headersHelper` を修正するか、プラグインがあなたのものでない場合は発行者に報告してください。`headersHelper` ケースは[プラグインコマンドリファレンス user\_config](/docs/ja/errors#plugin-command-references-user-config)の下に独自のエントリを持っています

<h4 id="server-is-configured-but-never-connects">
  サーバーが設定されているが決して接続しない
</h4>

`/mcp` を実行してサーバーのステータスを確認してください。サーバーが健全な場合、`/mcp` はそれを接続として一覧表示します。

サーバーが開始中に出力したエラーを読むには、`claude --debug` を実行し、`~/.claude/debug/<session-id>.txt` でログを開いてください。`--debug` フラグはターミナルに出力しません。

`.mcp.json` のサーバーエントリがスキーマに失敗しても、**Errors** タブに表示されません。Claude Code はそのサーバーをドロップし、`Invalid MCP server config for <server> in <path>` をそのデバッグログにのみ記録します。プラグインを読み込まずにエントリを見つけるには、シェルでプラグインディレクトリで `claude plugin validate` を実行してください。これはエラーとして報告します。

v2.1.281 より前では、`claude plugin validate` は `.mcp.json` をチェックしませんでした。

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  サーバーは `--plugin-dir` で機能しますが、インストール後に失敗します
</h4>

プラグインの作成者であり、サーバーは `--plugin-dir` でソースディレクトリからプラグインを読み込むときに開始しますが、プラグインがインストールされた後に失敗します。

Claude Code はインストールされたプラグインをキャッシュにコピーするため、ソースディレクトリからのみ機能するパスが壊れます。`${CLAUDE_PLUGIN_ROOT}` を使用してプラグイン内のパスを書き込んでください。

プラグインディレクトリの外側に到達するパスについては、[プラグインが参照するファイルがそのディレクトリの外側にある](#files-the-plugin-references-outside-its-directory-arent-found)を参照してください。

<h3 id="language-server-doesnt-start">
  言語サーバーが開始しない、メモリを使いすぎる、または間違った診断を報告
</h3>

[コード知能プラグイン](/docs/ja/plugins/code-intelligence)をインストールし、Claude が診断を表示していないか、言語サーバーがメモリを使いすぎているか、実際ではないエラーを報告しています。

<h4 id="language-server-doesn’t-start">
  言語サーバーが開始しない
</h4>

プラグインは言語サーバーバイナリに接続し、Claude Code は `PATH` からコマンド名でそれを生成します。

`/plugin` **Errors** タブは失敗をその理由で表示します。例えば、`Executable not found in $PATH: "<binary>"`、および `claude --debug` はそれを `LSP server <name> failed to start: <reason>` としてログします。

バイナリをインストールし、`claude` を開始するターミナルの `PATH` にあることを確認してください。例えば、`which typescript-language-server` で確認してください。次に、新しいセッションを開始してください。

<h4 id="language-server-uses-too-much-memory">
  言語サーバーがメモリを使いすぎる
</h4>

`rust-analyzer` や `pyright` などの言語サーバーはプロジェクト全体をインデックスします。`/plugin disable <plugin>` でセッションのプラグインを無効にし、代わりに Claude の組み込み検索ツールに依存してください。

<h4 id="false-positive-diagnostics-in-a-monorepo">
  モノレポの偽陽性診断
</h4>

ワークスペース用に設定されていない言語サーバーは、内部パッケージの未解決のインポートを報告できます。Claude Code 側で修正するものはなく、診断は Claude がコードを編集するのを止めません。

<h2 id="build-a-plugin">
  プラグインを構築
</h2>

プラグインを開発し、`--plugin-dir` で読み込むか、ローカルマーケットプレイスからインストールしています。これらのエントリはプラグインを開発している間に発生する失敗をカバーしています。各変更後にチェックを実行するには、[テストとデバッグ](/docs/ja/plugins/create#test-and-debug)を参照してください。

プラグインのユーザーにも到達する 2 つの失敗は[プラグインがインストールされているが機能していない](#plugin-installed-but-not-working)の下にエントリを持っています：

* **発火しないフック**：[発火しないフック](#failed-to-load-hooks-from-and-hooks-that-dont-fire)を参照してください
* **開始しない MCP サーバー**：[開始しない MCP サーバー](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)を参照してください

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

**Errors** タブは `commands path not found: <absolute path>` をガイダンス `Check that the path in your manifest or marketplace config is correct` で表示します。同じメッセージは `skills`、`agents`、`hooks` に対して表示されます。

Claude Code はマニフェストまたはマーケットプレイスエントリからパスをプラグインルートに対して解決し、そこに何も見つかりませんでした。メッセージのパスは確認した絶対パスであるため、ディスク上のものと比較してください。パスを修正するか、ディレクトリを作成し、`/reload-plugins` を実行してください。

マニフェストのパスはプラグインルートに対して相対的で、`./` で始まります。プラグインルートの外側に解決されるパスは、代わりに `<component> path escapes plugin directory` として報告され、ドロップされます。

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` をマーケットプレイスルートで実行しても、`plugins/` の下のプラグインを読み込まない
</h3>

`claude --plugin-dir <path>` を開始し、エラーは表示されませんが、プラグインのスキル、エージェント、フックはありません。

`--plugin-dir` はプラグインのルートディレクトリを取得します。`.claude-plugin/plugin.json` と `skills/` などのコンポーネントディレクトリを含むディレクトリです。代わりにマーケットプレイスルートを指すと、Claude Code は `marketplace.json` を読み込まないため、`plugins/` の下のプラグインは読み込まれず、エラーは表示されません。v2.1.281 より前では、Claude Code はマーケットプレイスルートを、そのディレクトリにちなんで名前が付けられた 1 つの空のプラグインとして読み込みました。フラグをプラグインディレクトリ自体に指してください：

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

次に、`/plugin` で **Installed** を開き、プラグインの詳細ペインを開きます。これはそのコンポーネントをリストします。

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  プラグインがそのディレクトリの外側で参照するファイルが見つからない
</h3>

プラグインはソースディレクトリで `--plugin-dir` で機能しますが、インストール後に失敗し、`../shared-utils` などのパスについてのエラーが表示されます。

Claude Code はインストールされたプラグインをキャッシュにコピーし、そこから読み込むため、プラグイン自体のディレクトリの外側に到達するパスはキャッシュで何も指しません。共有ファイルをプラグインディレクトリ内に移動するか、それを通じて参照してください。キャッシュがどこにあるか、パスがどのように解決されるかについては、[ディスク上のプラグインを見つける](/docs/ja/plugins/loading#find-plugins-on-disk)を参照してください。

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` は Windows でスラッシュを前方に表示
</h3>

Windows では、プラグインフックは `${CLAUDE_PLUGIN_ROOT}` を `C:/Users/you/...` として受け取り、バックスラッシュを期待していたスクリプトが壊れます。

Claude Code は Windows で Git Bash を通じてシェル形式のフックを実行し、目的上、プラグインルートを前方スラッシュ Win32 形式で置き換えます。Bash ビルトイン、MSYS ツール、ネイティブ Windows バイナリはすべてその形式を受け入れます。

スクリプトがバックスラッシュを必要とする場合は、[exec 形式とシェル形式](/docs/ja/hooks#exec-form-and-shell-form)で説明されているネイティブパスを保持する形式の 1 つに切り替えてください：

* exec 形式フック。`args` 配列でプロセスを直接生成します
* `"shell": "powershell"` を持つフック

<h3 id="plugin-loads-but-its-skills-are-missing">
  プラグインが読み込まれるがそのスキルが見つからない
</h3>

プラグインは **Installed** の下にエラーなくリストされていますが、`/` を入力するときにそのスキルは提供されません。

スキルはプラグインルートの `skills/` から読み込まれ、コマンドはプラグインルートの `commands/` から読み込まれます。`.claude-plugin/plugin.json` のみが `.claude-plugin/` 内に属し、`.claude-plugin/` 内の `skills/` ディレクトリはスキャンされません。ディレクトリをプラグインルートに移動し、`/reload-plugins` を実行してください。その後、`/plugin` のプラグインの詳細ペインはスキルをリストし、`/` を入力するとそれらが提供されます。

各スキルは `SKILL.md` を含むディレクトリです。`SKILL.md` ファイルではなくそのディレクトリを指すマニフェストの `skills` エントリは、`path is a file; skills entries must be directories containing SKILL.md` として報告されます。

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  スキルが読み込まれるが Claude は決してスキルを呼び出さない
</h3>

プラグインのスキルは `/<plugin>:<skill>` コマンドを入力するときに実行されますが、Claude は平文のリクエストに応じてそれを呼び出しません。

これらの原因を順番に確認してください：

* **スキルが `disable-model-invocation: true` を設定**：そのフィールドが設定されている場合、あなただけがスキルを呼び出すことができます。[最初のプラグインを作成](/docs/ja/plugins/create#create-your-first-plugin)のテンプレートスキルはそれを設定します。Claude に独自に呼び出させたいスキルから行を削除してください。[スキルを呼び出す人を制御](/docs/ja/skills#control-who-invokes-a-skill)はフィールドをカバーしています
* **説明は人々がどのように尋ねるかと一致しない**：[スキルがトリガーされない](/docs/ja/skills#skill-not-triggering)のチェックを実行してください
* **説明が切り詰められている**：多くのスキルがインストールされている場合、Claude Code は説明を短縮してリストの文字予算に合わせます。これは Claude が要求と一致するために必要なキーワードを削除できます。[スキルの説明が短くカットされている](/docs/ja/skills#skill-descriptions-are-cut-short)を参照してください

1 つずつチェックするのではなく、現実的なプロンプト全体でスキルがどのくらい頻繁にトリガーされるかを測定するには、[`tool_used: Skill` グレーダー](/docs/ja/plugin-evals#create-your-first-eval-suite)を使用して eval ケースを書き、各説明変更後に `claude plugin eval` で実行してください。

<h3 id="is-not-a-plugin-or-skill-folder">
  `claude plugin eval init` から `<directory> is not a plugin or skill folder`
</h3>

プラグインのルートではないディレクトリから `claude plugin eval init` を実行しました。例えば、ホームディレクトリまたはプラグインをサブディレクトリに保つリポジトリのルート。`init` は作業ディレクトリの下にスイートを書き込むため、プラグインが決して見ないであろう `evals/` ディレクトリを作成する代わりに停止します。

プラグインのルート、`.claude-plugin/plugin.json` またはスキルの `SKILL.md` を保持するディレクトリに変更し、コマンドを再度実行してください。目的上、スイートを別の場所にスキャフォールドするには、`--eval-dir` を渡してください。[eval でプラグインをテスト](/docs/ja/plugin-evals)を参照してください。

<h3 id="the-userconfig-dialog-never-appears">
  `userConfig` ダイアログが決して表示されない
</h3>

プラグインは `userConfig` オプションを宣言しますが、インストール時に設定ダイアログが表示されません。

インタラクティブインストールはダイアログを表示し、シェルコマンドは代わりに値をフラグとして取得します：

* **セッションで `/plugin install`、または `/plugin` の Discover タブ**：ダイアログはこのインタラクティブインストールの一部です
* **シェルで `claude plugin install`**：`userConfig` 値を要求しません。渡す `--config KEY=VALUE` 値を保存し、オプションが設定されたままの場合、`N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` を出力します。設定されていないオプションが必須の場合、`(M required)` は `not yet set` に従います。

シェルからインストールした場合は、`--config` で値を渡してください。オプションごとに 1 つのフラグ：

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

すべてのオプションが設定されている場合、インストール出力は `not yet set` 行を運びません。代わりに後でダイアログを開くには、セッションで `/plugin configure my-plugin@my-marketplace` を実行してください。

マニフェストが宣言しない `--config` キーを渡す場合、プラグインはまだインストールされ、コマンドは `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` を出力し、その後にプラグインが宣言するキーが続きます。

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` がエラーを報告
</h3>

`claude plugin validate <path>` を実行するか、セッションで `/plugin validate <path>` を実行し、`Found N errors` と `Validation failed` を出力し、コード 1 で終了しました。

バリデーターは渡したパスのマニフェストを読み込みます：プラグインディレクトリの `.claude-plugin/plugin.json`、またはマーケットプレイスディレクトリの `.claude-plugin/marketplace.json`。マーケットプレイスの場合、エントリ自体のマニフェストの問題にエントリインデックスをプレフィックスします。例えば、`plugins[1] plugin.json → json: ...`。

表は検証を停止するメッセージと 2 つの警告 `No frontmatter block found` と `Unknown field '<key>'` をカバーしています。これらは `--strict` を渡すときのみ停止します。説明の欠落など、他の警告はリストされていません。

| メッセージ                                                                                                    | 原因                                                        | 修正                                                                         |
| :------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------- | :------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | パスにマニフェストがないか、存在しません。                                     | プラグインまたはマーケットプレイスルートに対してコマンドを実行してください。`.claude-plugin/` を含むディレクトリです。       |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | ディレクトリに `.claude-plugin/` マニフェストがありません。                   | マニフェストを作成するか、正しいディレクトリを指してください。                                            |
| `Invalid JSON syntax: <parse error>`                                                                     | マニフェストまたは `hooks/hooks.json` は有効な JSON ではありません。           | JSON を修正してください。`hooks/hooks.json` を修正するまで、セッションはそのファイルのフックなしでプラグインを読み込みます。 |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | マニフェストのコンポーネントパスが存在しません。                                  | パスを修正するか、ディレクトリを作成してください。                                                  |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | コンポーネントパスはプラグインディレクトリをエスケープします。                           | プラグインルート内のパスを使用してください。                                                     |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | `skills` エントリは `SKILL.md` ではなくそのディレクトリを指しています。            | 親ディレクトリを指してください。またはルートレベルの `SKILL.md` の場合は `.`。                            |
| `No frontmatter block found` または `YAML frontmatter failed to parse: <error>`                             | スキル、エージェント、またはコマンドファイルに不足しているか無効な YAML frontmatter があります。 | `---` デリミタ間に frontmatter を追加または修正してください。プラグインディレクトリを検証するときに報告されます。         |
| `Unknown field '<key>'`                                                                                  | マニフェストにスキーマが定義しないフィールドがあります。                              | それを削除するか、メッセージが提案する名前を使用してください。Claude Code は読み込み時に不明なフィールドを無視します。          |

各修正後にコマンドを再度実行して、エラーが出力されなくなるまで実行してください。

`plugin.json` フィールドは[マニフェストリファレンス](/docs/ja/plugins/manifest-reference)にあり、マーケットプレイスレベルのメッセージは[マーケットプレイス検証エラー](#marketplace-validation-errors)の下にあります。

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

プラグインは `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.` で読み込みに失敗します。

プラグインは独自の `plugin.json` を持ち、そのマーケットプレイスエントリは `strict: false` を設定しながら、`commands`、`agents`、`skills`、`hooks`、`outputStyles`、または `themes` のいずれかを宣言しています。エントリからそれらのフィールドを削除するか、エントリで `strict: true` を設定して、Claude Code がそれらを `plugin.json` に追加するようにしてください。[厳密モード](/docs/ja/plugins/marketplace-reference#strict-mode)を参照してください。

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

プラグインが読み込まれるとき、`claude --debug` ログは `~/.claude/debug/<session-id>.txt` で `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` を記録します。セッションまたは **Errors** タブに何も表示されません。

マニフェストの `commands` パスは存在しますが、`.md` ファイルを保持せず、サブディレクトリに `SKILL.md` を保持しません。コマンドファイルを追加するか、マニフェストからパスを削除してください。

<h2 id="host-a-marketplace">
  マーケットプレイスをホスト
</h2>

マーケットプレイスを発行し、ユーザーがエラーを報告するか、独自の検証が失敗します。これらのエントリはマーケットプレイス所有者向けです。

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  相対パスを持つプラグインが URL ベースのマーケットプレイスで失敗
</h3>

ユーザーは `https://example.com/marketplace.json` URL でマーケットプレイスを追加しました。`./plugins/my-plugin` などの相対パスである `source` を持つプラグインのインストールは `its marketplace entry path does not stay inside the marketplace directory` で失敗します。既にインストールされているプラグインは `Plugin source path refused` で読み込みに失敗します。両方のメッセージは[エラーリファレンスエントリ](/docs/ja/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)を持っています。

ユーザーが URL ベースのマーケットプレイスを追加する場合、Claude Code は `marketplace.json` ファイル自体のみをダウンロードします。相対パスからプラグインファイルをそのサーバーからフェッチしないため、相対パスはダウンロードされたことのないディレクトリを指しています。Claude Code が独自にフェッチできるソース（GitHub リポジトリなど）を各エントリに与えてください：

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

または、マーケットプレイスを git リポジトリでホストし、ユーザーにリポジトリ URL で追加するよう指示してください。git ソースの場合、Claude Code はリポジトリ全体をクローンするため、相対パスは解決されます。ソースタイプは[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)にあります。

<h3 id="marketplace-validation-errors">
  マーケットプレイス検証エラー
</h3>

マーケットプレイスディレクトリから `claude plugin validate .` を実行し、マーケットプレイスファイル自体のエラーまたは警告を報告しました。

`claude plugin validate` はまた、`source` がローカルパスである各エントリを検証し、エントリの `version` がプラグイン自体のマニフェストと一致しないときに警告します。

表はマーケットプレイスレベルのメッセージをリストしています。エントリレベルのメッセージは[`claude plugin validate` がエラーを報告](#claude-plugin-validate-reports-errors)の下のプラグインメッセージで、`plugins[N] plugin.json →` でプレフィックスされています。

| メッセージ                                                                                                                      | 種類  | 修正                                                                                                     |
| :------------------------------------------------------------------------------------------------------------------------- | :-- | :----------------------------------------------------------------------------------------------------- |
| `Duplicate plugin name "<name>" found in marketplace`                                                                      | エラー | 各プラグインに一意の `name` を与えてください。                                                                            |
| `Path contains "..": <path>` under `plugins[N].source`                                                                     | エラー | `..` セグメントなしでマーケットプレイスルートに対して相対パスを使用してください。                                                            |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                           | エラー | 名前からエスケープまたは改行などの文字を削除してください。                                                                          |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                | エラー | プラグイン `name` から文字を削除してください。                                                                            |
| `Marketplace has no plugins defined`                                                                                       | 警告  | `plugins` に少なくとも 1 つのエントリを追加してください。                                                                    |
| `No marketplace description provided`                                                                                      | 警告  | トップレベルの `description` を追加してください。                                                                       |
| `Plugin name "<name>" is not kebab-case` under `plugins[N] plugin.json → name`                                             | 警告  | 小文字、数字、ハイフンに名前を変更してください。Claude Code は他の形式を受け入れますが、claude.ai マーケットプレイス同期はそれらを拒否します。                     |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                           | 警告  | エントリを `plugin.json` と一致するように更新してください。これはインストール時に権威です。                                                  |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                  | 警告  | マーケットプレイスの名前を変更してください。Claude Desktop の管理マーケットプレイス同期は、任意のケースで `org`、`org-provisioned`、`unknown` を拒否します。 |
| `Marketplace name "<name>" is not accepted by Claude Desktop` または `Plugin name "<name>" is not accepted by Claude Desktop` | 警告  | 最大 128 文字の文字、数字、`.`、`_`、`-` に名前を変更し、文字または数字で始まります。                                                     |

v2.1.247 より前では、制御またはビジョナル形式の文字を含むマーケットプレイス名は、`Marketplace name impersonates an official Anthropic/Claude marketplace` としてのみ報告されました。

<h2 id="blocked-by-your-organization">
  組織によってブロック
</h2>

組織は管理設定をデプロイしてプラグインを制限し、コマンドはポリシーメッセージで拒否されました。これらのエントリは各拒否の背後にある設定に名前を付けるため、管理者に何を依頼するかを知っています。管理者側については、[組織のプラグインを管理](/docs/ja/plugins/org)を参照してください。

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

`/plugin marketplace add`、`update`、またはインストールを実行し、Claude Code はこの行で拒否しました。GitHub または git ソースの場合、ホストはソースの後に括弧内に続きます。例えば、`'github:owner/repo' (github.com)`。

管理者は管理設定で `blockedMarketplaces` または `strictKnownMarketplaces` を設定し、このソースは許可されていません。管理者にソースを許可するよう依頼するか、メッセージがリストする許可されたソースの 1 つを追加してください。

メッセージの残りを一致させて、どのポリシーがソースをブロックしたかを確認してください：

* **`Allowed sources: <list>`**：ブロックは `blockedMarketplaces` ブロックリストではなく `strictKnownMarketplaces` 許可リストから来ています
* **`No external marketplaces are allowed.`**：`strictKnownMarketplaces` 許可リストは空です
* **ショートハンドが github.com を想定するという `Tip:`**：許可リストは git ホストをホスト名で許可し、渡した `owner/repo` ショートハンドは github.com を指しています。リポジトリが内部ホストに存在する場合は、`git@your-git-host.com:owner/repo.git` などの完全な URL で再度追加してください

ポリシーがより制限的になる前に追加したマーケットプレイスは、ポリシーがすべての更新に適用されるため、更新を停止します。

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

**Errors** タブはこの行を表示するか、既に登録しているマーケットプレイスの場合は `Marketplace "<name>" is blocked by enterprise policy` を表示します。

[マーケットプレイスソース](#marketplace-source-is-blocked-by-enterprise-policy)をブロックする同じ管理設定は読み込み時に適用されます。`strictKnownMarketplaces` はこのマーケットプレイスを含まないか、`blockedMarketplaces` がそれに名前を付けるため、Claude Code は読み込みを停止し、そのプラグインも同様です。許可リストバリアントの場合、ガイダンス行は許可されたソースを表示するか、`Contact your administrator to configure allowed marketplace sources` を読みます。ブロックリストバリアントの場合、`This marketplace source is explicitly blocked by your administrator` を読みます。

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

インストールはこの行で拒否されたか、有効化は同じ行で終わったか、`Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy` または `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy` という理由で名前を付けたインストールまたは更新。

管理設定はこのプラグイン、そのマーケットプレイス、またはそれが必要とする依存関係をブロックします。どのエントリが適用されるかを管理者に尋ねてください。ブロックされた依存関係は、依存関係のマーケットプレイスが許可されるまでプラグインをインストールできないことを意味します。

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

`--plugin-dir`、`--plugin-url`、`--agents`、または `--mcp-config` で `claude` を開始しました。Claude Code はこのメッセージで終了し、`Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

管理者は管理設定で `disableSideloadFlags` を設定し、任意のパスからプラグイン、エージェント、サーバーを読み込むフラグをオフにします。承認されたマーケットプレイスからプラグインを読み込むか、管理者に設定を削除するよう依頼してください。

`/plugin` **Errors** タブの関連メッセージは `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings` です。管理設定はそのプラグインを名前で有効または無効にし、Claude Code はポリシーをオーバーライドできないようにそのコピーを無視します。

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

`claude plugin init` または `claude plugin enable` を実行し、この行で停止しました。メッセージは `strictKnownMarketplaces or blockedMarketplaces` に名前を付け、管理者に `{"source":"skills-dir"}` を `strictKnownMarketplaces` に追加するか、`blockedMarketplaces` から削除するよう依頼します。

`skills-dir` ソースは Claude Code が `~/.claude/skills/` ディレクトリから読み込むプラグインを表します。メッセージが名前を付ける変更を行うよう管理者に依頼してください。

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

`command` ソースを持つプラグインをインストールまたは更新し、この行で停止し、`The plugin was not installed or updated and its command was not run.`

管理者は `disableCommandPluginSources` を設定したため、Claude Code はマーケットプレイスが宣言したコマンドを実行することを拒否してプラグインを生成します。`disableCommandPluginSources` が設定されていない場合、`allowManagedHooksOnly` のみを設定すると同じ効果があります。ポリシーが許可するソースタイプからプラグインを発行できるかどうかを管理者に尋ねてください。

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

`claude plugin marketplace update <name>` を実行し、`Marketplace '<name>' is seed-managed (<dir>)` で失敗し、管理者に尋ねるヒントが続きました。

オペレーターは `CLAUDE_CODE_PLUGIN_SEED_DIR` を通じてこのマーケットプレイスを事前に入力し、Claude Code はシード管理マーケットプレイスを読み取り専用として扱います。バルク `marketplace update` はそれをスキップし、他を更新します。

マーケットプレイスのコンテンツを変更するには、シードイメージを維持する人に更新するよう依頼してください。手順については、[コンテナと CI をシード](/docs/ja/plugins/org#seed-containers-and-ci)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグイン読み込みリファレンス](/docs/ja/plugins/loading)：スコープ、キャッシュ、優先度の動作方法
* [プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference)：`claude plugin` コマンドのフラグ、デフォルト、出力、終了コード
* [プラグインをインストールして管理](/docs/ja/plugins/install)：開始からのインストール手順
* [組織のプラグインを管理](/docs/ja/plugins/org#troubleshoot-policy)：管理者向けのポリシー側トラブルシューティング
