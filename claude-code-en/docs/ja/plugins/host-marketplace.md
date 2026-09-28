> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# マーケットプレイスをホストして維持する

> ユーザーが到達できる場所にプラグインマーケットプレイスを公開し、プライベートマーケットプレイスへのアクセスを許可し、インストールを破損させずに更新と名前変更をリリースします。

マーケットプレイスをホストするということは、`marketplace.json` カタログを他のユーザーが `/plugin marketplace add` で追加でき、そのプラグインをインストールでき、プッシュ後も変更を受け取り続けられる場所に配置することを意味します。

このページはマーケットプレイスを運営する人向けです。

<Note>
  これらのケースは他のページで説明されています。

  * **カタログファイルをまだ作成していない場合**: [マーケットプレイスを作成する](/docs/ja/plugins/create-marketplace)から始めてください
  * **組織のマシン全体でマーケットプレイスを要求、制限、または事前インストールする必要がある管理者の場合**: [組織のプラグインを管理する](/docs/ja/plugins/org)をお読みください
</Note>

[マーケットプレイスをホストする](#host-your-marketplace)から始めて、ホストとユーザーが実行するコマンドを選択してください。最初のリリースの前に[ユーザーを最新の状態に保つ](#keep-users-up-to-date)をお読みください。プラグインの `name` を変更する前に[プラグインの名前変更または削除](#rename-or-remove-a-plugin)をお読みください。

<h2 id="host-your-marketplace">
  マーケットプレイスをホストする
</h2>

マーケットプレイスは GitHub、別の git ホスト、ホストされた `marketplace.json` URL、または共有ファイルシステム上のディレクトリでホストできます。ユーザーにホストの追加コマンドを送信し、マシンで必要なものを伝えてください：

| ホスト                                                     | ユーザーが Claude Code セッションで実行                                             | ユーザーが必要なもの                                                                                               |
| :------------------------------------------------------ | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| GitHub                                                  | `/plugin marketplace add your-org/your-marketplace`                    | `git`、およびプライベートリポジトリの場合は[プライベートマーケットプレイスへのアクセスを許可する](#grant-access-to-a-private-marketplace)で説明されているアクセス |
| GitLab、Bitbucket、GitHub Enterprise Server、または別の git ホスト | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git`、およびマシンからホストへのアクセス。`owner/repo` 短縮形は常に github.com を意味するため、完全な URL を送信してください                         |
| ホストされた `marketplace.json` URL                           | `/plugin marketplace add https://plugins.example.com/marketplace.json` | URL への HTTPS アクセス。ユーザーはカタログ自体に `git` は不要です                                                               |
| 共有ファイルシステム上のディレクトリ                                      | `/plugin marketplace add /Volumes/shared/claude-plugins`               | パスへの読み取りアクセス                                                                                             |

GitHub または git URL マーケットプレイスのブランチまたはタグをピンするには、ユーザーに `#<ref>` を追加するよう指示してください。例えば `your-org/your-marketplace#stable` のように。[プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference#plugin-marketplace-add)は、コマンドが受け入れるすべての形式をリストしています。

追加に成功すると `Successfully added marketplace: your-marketplace` が出力されます。Claude Code はリポジトリ名ではなく、`marketplace.json` の `name` フィールドからその名前を取得します。

その後、ユーザーはプラグインをマーケットプレイスの `name` とエントリの `name` でインストールします。例えば `/plugin install code-formatter@your-marketplace` のように。

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  リポジトリ内のすべてのユーザーのためにマーケットプレイスを登録する
</h3>

1 つのリポジトリで作業するすべてのユーザーとマーケットプレイスを共有するには、シェルからそこで `claude plugin marketplace add your-org/your-marketplace --scope project` を 1 回実行し、書き込まれた `.claude/settings.json` をコミットしてください。Claude Code は、[フォルダを信頼する](/docs/ja/plugins/org#require-plugins-per-repository)各チームメイトのマーケットプレイスを登録します。

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  URL ホストマーケットプレイスで相対パスエントリを避ける
</h3>

ユーザーがマーケットプレイスをベアな `marketplace.json` URL として追加する場合、Claude Code はそのファイルのみをダウンロードします。`plugins` 配列内のエントリで、`source` が `./plugins/formatter` のような相対パスの場合、インストール時に[`its marketplace entry path does not stay inside the marketplace directory`](/docs/ja/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)で失敗します。すべてのエントリに、`github` リポジトリや `archive` URL のような単独でフェッチできるソースを指定するか、マーケットプレイスを git リポジトリでホストして Claude Code がツリー全体をクローンするようにしてください。

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  共有ディレクトリでプラグインをその場で編集する
</h3>

ユーザーが共有ディレクトリからマーケットプレイスを追加する場合、Claude Code は相対パスソースを持つプラグインをそのディレクトリから直接読み取り、コピーしません。ユーザーは次のセッション開始時または `/reload-plugins` 実行時に編集内容を確認します。更新ステップやバージョンバンプは不要です。

<h3 id="keep-plugin-files-out-of-git-lfs">
  プラグインファイルを Git LFS から除外する
</h3>

プラグインが必要とするファイルを [Git LFS](https://git-lfs.com) から除外してください。ユーザーが git リポジトリでホストされたマーケットプレイスを追加するか、それがリストする git ベースのプラグインをインストールする場合、Claude Code はそのマーケットプレイスまたはプラグインリポジトリをマシンにクローンします。クローンは LFS コンテンツをダウンロードしないため、LFS 追跡ファイルはポインタファイルとして到着します。

<h3 id="share-files-within-a-marketplace-with-symlinks">
  シンボリックリンクでマーケットプレイス内のファイルを共有する
</h3>

プラグインと同じマーケットプレイスの他の部分の間でファイルを共有するには、プラグインディレクトリ内にシンボリックリンクを作成してください。Claude Code がプラグインをキャッシュにコピーする場合、各シンボリックリンクをターゲットが解決される場所で処理します：

* **プラグイン自体のディレクトリ内**：シンボリックリンクはキャッシュ内で相対シンボリックリンクとして保持されるため、実行時にコピーされたターゲットへの解決を続けます。
* **同じマーケットプレイス内の他の場所**：シンボリックリンクは逆参照されます。ターゲットのコンテンツはキャッシュにコピーされます。これにより、メタプラグインの `skills/` ディレクトリがマーケットプレイス内の他のプラグインで定義されたスキルにリンクできます。
* **マーケットプレイス外**：セキュリティ上の理由からシンボリックリンクはスキップされます。

ローカルパスからインストールされたプラグイン、またはデフォルトの `mode` が `copy` である [`command` ソース](/docs/ja/plugins/marketplace-reference#command-plugin-source)からインストールされたプラグインの場合、Claude Code はプラグイン自体のディレクトリ内で解決するシンボリックリンクのみを保持し、他はすべてスキップします。

次のコマンドは、マーケットプレイスプラグイン内から兄弟プラグインで定義された共有スキルへのリンクを作成します。Windows では、昇格されたコマンドプロンプトから `mklink /D` を使用するか、開発者モードを有効にしてください：

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  組織設定を通じて配布する
</h2>

Team または Enterprise プランでは、ユーザーが自分で追加するホストの代わりに、claude.ai の[**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory)を通じてマーケットプレイスを配布することもできます。Organization sync は、claude.ai 上の組織の GitHub または GitLab 接続を通じてリポジトリを読み取るため、ユーザーの git 認証情報は関係ありません。

Organization sync は、`/plugin marketplace add` よりもリポジトリについてより厳密です：

* **マーケットプレイスリポジトリ**：github.com と gitlab.com では、プライベートまたは内部である必要があります
* **プラグインソース**：各プラグインソースは `github`、`url`、または `git-subdir` タイプ、または `./` で始まる[相対パス](/docs/ja/plugins/marketplace-reference#relative-path-plugin-source)である必要があります
* **トップレベルの `bin/` ディレクトリ**：claude.ai はこれを持つプラグインを拒否し、マーケットプレイスの残りを同期します。エラーメッセージは `Plugin contains a top-level bin/ directory` で始まります。実行可能ファイルを `scripts/` などの別のディレクトリに保持し、フック または MCP サーバー設定から `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` として参照してください

[組織のプラグインを管理する](https://support.claude.com/en/articles/13837433)を参照して、管理者ワークフローを確認してください。

<h2 id="grant-access-to-a-private-marketplace">
  プライベートマーケットプレイスへのアクセスを許可する
</h2>

ユーザーがマーケットプレイスを追加、インストール、または更新する場合、Claude Code はマシンで `git` を実行し、対話的なプロンプトをオフにして、そのマシンが既に保持している認証情報に依存します。Claude Code は独自の git トークンを持たず、`marketplace.json` にはそのためのフィールドがありません。

追加コマンドの形式によって、クローンが SSH または HTTPS で実行されるかを選択します：

* **GitHub `owner/repo`**：Claude Code は `ssh -T git@github.com` をプローブし、プローブが成功する場合は SSH でクローンします。プローブが失敗するか、SSH クローン自体が失敗する場合は、HTTPS でクローンします。GitHub SSH キーを持たないマシンのユーザーは、`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` を設定してプローブをスキップし、HTTPS でクローンできます。
* **`git@host:path.git`**：SSH。
* **`https://example.com/repo.git`**：HTTPS。

各プロトコルがマシンで必要とするものをユーザーに伝えてください：

* **SSH**：キーはパスフレーズプロンプトなしで機能する必要があります。例えば `ssh-agent` に読み込まれているため。ホストは既に `known_hosts` にある必要があります。
* **HTTPS**：Claude Code はユーザーの git 認証情報ヘルパーを有効にしたままにしますが、プロンプトを禁止します。ヘルパーが既に保存している認証情報は機能します。要求する必要があるものは失敗します。GitHub では、`gh auth login` の後に `gh auth setup-git` を実行すると、認証情報が保存されます。

GitHub Enterprise Server ホストの場合、ユーザーはマシンからそのホストへの git アクセスが必要です。[GHES 上のプラグインマーケットプレイス](/docs/ja/github-enterprise-server#plugin-marketplaces-on-ghes)を参照して、各 Claude Code サーフェスが GHES ホストマーケットプレイスに到達するために必要なものを確認してください。

代わりに claude.ai の**Organization settings > Plugins & skills**を通じて配布する場合、ユーザーの git 認証情報は関係ありません。[組織設定を通じて配布する](#distribute-through-organization-settings)を参照して、どのプラグインソースがプライベートになる可能性があるかを確認してください。

<h3 id="serve-users-who-have-no-git-host-account">
  git ホストアカウントを持たないユーザーにサービスを提供する
</h3>

git ホストアカウントを持たないユーザーは、`marketplace.json` URL として、または共有ディレクトリからマーケットプレイスを追加できますが、エントリソースにもアクセスできるプラグインのみをインストールできます。プライベート `github` リポジトリを指すエントリは、Claude Code がマーケットプレイスをホストする git に使用するのと同じ非対話的な `git` でフェッチするため、インストール時に失敗します。

これらのエントリソースは git アカウントを必要としません：

* **`archive`**：HTTPS でダウンロードされた zip。ユーザーは `git` またはアカウントを必要とせず、URL へのネットワークアクセスのみが必要です。Claude Code v2.1.224 以降が必要です。各アーカイブを `sha256` でピンして、Claude Code が変更されたダウンロードを拒否するようにしてください。ダウンロードで認証情報を送信するには、[アーカイブダウンロードを認証する](#authenticate-archive-downloads)を参照してください。
* **パブリック git リポジトリ**：Claude Code は、エントリが `https://` URL を指定する場合、認証情報なしで HTTPS 経由でパブリック `url` または `git-subdir` ソースをクローンします。`github` ソース、または `owner/repo` として記述された `git-subdir` ソースの場合、GitHub SSH キーを持たないユーザーは `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` を設定します。

1 つのネットワーク上のチームの場合、共有ファイルシステム上の `directory` マーケットプレイスも git アカウントなしで機能します。ユーザーはパスへの読み取りアクセスのみが必要です。

<h3 id="what-background-auto-update-does-with-credentials">
  バックグラウンド自動更新が認証情報で行うこと
</h3>

バックグラウンド自動更新は、セッション開始後のマーケットプレイスとインストール済みプラグインの Claude Code の無人更新です。[ユーザーを最新の状態に保つ](#keep-users-up-to-date)で説明されているように、ユーザーまたは管理者がオンにするまで、マーケットプレイスではオフです。

プライベートマーケットプレイスでオンの場合、新しいコミットのバックグラウンドチェックはユーザーの設定された git 認証情報ヘルパーを使用し、プロンプトを表示しません。各種類のリモートとヘルパーは異なる結果を与えます：

* **SSH リモート**：`ssh-agent` に読み込まれたキーがチェックを認証します。
* **保存された認証情報を持つ HTTPS リモート**：プロンプトなしで保存された認証情報を提供できるヘルパーがチェックを認証します。Git Credential Manager、macOS Keychain ヘルパー、および `git-credential-store` は、ホストの認証情報を保持すると、このように機能します。
* **プロンプトが必要なヘルパーを持つ HTTPS リモート**：ヘルパーはバックグラウンドで応答できません。更新は静かに失敗し、既存のチェックアウトが所定の位置に留まるため、ユーザーのプラグインは最後に同期された状態から機能し続けます。

チェック後、Claude Code は次のいずれかを実行します：

* **チェックアウトは最新です**：Claude Code はそのままにします。
* **チェックが新しいコミットを見つけるか、リモートに到達または認証できないため失敗する**：Claude Code はマーケットプレイスを再度クローンし、既存のチェックアウトを新しいクローンに置き換えます。そのクローンが失敗する場合、既存のチェックアウトが所定の位置に留まります。再クローンは[大規模なリポジトリでタイムアウト](/docs/ja/plugins/troubleshooting#git-clone-timed-out-after-120s)する可能性があります。

プライベートマーケットプレイスを最新に保つために、ユーザーは次のいずれかを実行できます：

* **認証情報を保存する**：最初に認証情報ヘルパーにサインインして、ホストの認証情報を保持するようにします。GitHub の場合、`gh auth login` を実行してから `gh auth setup-git` を実行します。
* **失敗時にチェックアウトを保持する**：ユーザーが `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` を設定する場合、バックグラウンドチェックがリモートに到達または認証できない場合、Claude Code は再クローンを試みずに既存のチェックアウトを保持します。プラグインは最後に同期された状態から機能し続けます。

ユーザーが環境で `GITHUB_TOKEN` または別のプロバイダートークンを設定する場合、それだけではバックグラウンドチェックを認証しません。トークンは `gh` CLI のヘルパーなどの認証情報ヘルパーを通じて有効になり、`GH_TOKEN` と `GITHUB_TOKEN` を読み取ります。

<h2 id="roll-out-to-a-whole-company">
  会社全体にロールアウトする
</h2>

プラグインを会社全体にロールアウトするには、マーケットプレイスの所有者、管理設定を制御する管理者、および Claude Code を使用する各ユーザーが関係します。管理者なしでロールアウトを実行できます。その場合、各ユーザーはマーケットプレイスを追加してプラグインをインストールします。

| 誰                 | 何をするか                                                                                                           | どこで説明されているか                                                                                                                      |
| :---------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| あなた、マーケットプレイスの所有者 | カタログを会社のみが読み取れるリポジトリに保持し、ホストの追加コマンドを送信し、各ユーザーがマシンで必要なものを伝える                                                     | [マーケットプレイスをホストする](#host-your-marketplace)と[プライベートマーケットプレイスへのアクセスを許可する](#grant-access-to-a-private-marketplace)                   |
| 管理者               | 管理設定で `extraKnownMarketplaces` と `enabledPlugins` を使用してマーケットプレイスを登録し、すべてのユーザーのプラグインをオンにし、そこで `autoUpdate` を設定する | [マーケットプレイスとそのプラグインを要求する](/docs/ja/plugins/org#require-a-marketplace-and-its-plugins)と[更新ポリシーを設定する](/docs/ja/plugins/org#set-update-policy) |
| 各ユーザー             | プライベート git リポジトリへの読み取りアクセスが必要で、認証情報がマシンに既に保存されている。管理者がいない場合、追加とインストールコマンドも実行する                                  | [プライベートマーケットプレイスを追加する](/docs/ja/plugins/install#add-a-private-marketplace)                                                            |

git ホストアカウントを持たないユーザーの場合、これらのセクションはそれぞれ 1 つの方法をカバーしています：

* **git アカウントを必要としないエントリソース**：[git ホストアカウントを持たないユーザーにサービスを提供する](#serve-users-who-have-no-git-host-account)
* **事前入力されたプラグインディレクトリ**：[コンテナと CI をシードする](/docs/ja/plugins/org#seed-containers-and-ci)。これは git ホストアカウントを持たないユーザーにもサービスを提供します
* **claude.ai 組織設定**：[組織設定を通じて配布する](#distribute-through-organization-settings)。ユーザーの git 認証情報は関係ありません

<h2 id="keep-users-up-to-date">
  ユーザーを最新の状態に保つ
</h2>

ユーザーへの変更は、マーケットプレイスでバックグラウンド自動更新がオンになると、またはユーザーがプラグイン自体を更新するときに到達します。どちらの場合も、ユーザーは計算されたバージョンが変更されたときにのみプラグインの新しいコピーを取得します。[バージョンをリリースする](#release-a-new-version)で説明されています。

<h3 id="turn-on-auto-update">
  自動更新をオンにする
</h3>

バックグラウンド自動更新はデフォルトではマーケットプレイスではオフで、`marketplace.json` にはそれをオンにするフィールドがありません。ユーザーまたは管理者がオンにします：

* **ユーザーにオンにするよう指示する**：各ユーザーは `/plugin` の**Marketplaces**に移動し、マーケットプレイスを選択して、**Enable auto-update**を選択します。
* **管理者に設定するよう依頼する**：管理者が管理設定でマーケットプレイスの `extraKnownMarketplaces` エントリで `"autoUpdate": true` を設定する場合、それらの設定を受け取るすべてのユーザーに対してオンです。[更新ポリシーを設定する](/docs/ja/plugins/org#set-update-policy)を参照してください。

自動更新がない場合、ユーザーはセッションで `/plugin marketplace update <name>` を実行するか、シェルで `claude plugin update <plugin>@<name>` を実行して変更を受け取ります。

更新がユーザーに到達するときに何が表示されるかについては、[自動更新が実行されるとき](/docs/ja/plugins/loading#when-auto-update-runs)を参照してください。

<h3 id="release-a-new-version">
  新しいバージョンをリリースする
</h3>

ユーザーに新しいバージョンをリリースするには、プラグインの `version` を変更してください。ユーザーは、プラグインの計算されたバージョンが持っているものと異なる場合にのみ新しいコピーを取得します。そのバージョンは `plugin.json` から最初に来て、次にマーケットプレイスエントリから来ます。[バージョンと更新](/docs/ja/plugins/loading#versions-and-updates)を参照してください。

ユーザーが追加したマーケットプレイスからローカルディレクトリとして[その場で読み込む](/docs/ja/plugins/loading#find-plugins-on-disk)プラグインは `version` で制御されません。セッション開始時に現在のファイルを読み込みます。バージョン文字列が何を言おうとも。

その場での読み込みまたは `command` ソースからのインストール以外のすべてのインストールについて、各リリースで `version` を増やすか、省略してください：

* **各リリースで `version` をバンプする**：ユーザーはキャッシュされたコピーに留まります。文字列が変更されるまで。`"version": "1.0.0"` を設定してコミットをプッシュしても変更しない場合、ユーザーはそれらを受け取りません。
* **`version` を省略する**：ユーザーはコミットを追跡します。`plugin.json` とマーケットプレイスエントリの両方から `version` を除外してください。

`plugin.json` とマーケットプレイスエントリの両方で `version` を設定しないでください。そうする場合、Claude Code は警告なしに `plugin.json` 値を使用し、`claude plugin validate` は不一致を `Entry declares version "<a>" but <path>/plugin.json says "<b>"` として報告します。

<h3 id="hold-users-on-one-version">
  ユーザーを 1 つのバージョンに保持する
</h3>

1 つのマーケットプレイスは一度に各プラグインの 1 つのバージョンを提供するため、各エントリが指すものを選択することでユーザーをバージョンに保持します：

* **プラグインエントリの `ref` と `sha`**：`ref` はブランチまたはタグに名前を付け、`sha` は `github`、`url`、または `git-subdir` ソースのコミットに名前を付けます。[プラグインソース](/docs/ja/plugins/marketplace-reference#plugin-sources)を参照してください。
* **追加コマンドの `#<ref>`**：`your-org/your-marketplace#stable` を追加するユーザーはカタログのそのブランチまたはタグを取得します。一度に 2 つのリリースラインの場合は、[リリースチャネルを実行する](#run-release-channels)を参照してください。
* **`<plugin>--v<version>` タグ**：依存関係のバージョン範囲はこれらのタグに対して解決されます。[他のプラグインが依存するプラグインをリリースする](/docs/ja/plugins/dependencies#tag-plugin-releases-for-version-resolution)を参照してください。

[新しいバージョンをリリースする](#release-a-new-version)は、変更されたエントリがユーザーに到達するときを示します。

<h3 id="change-the-command-of-a-command-source">
  コマンドソースのコマンドを変更する
</h3>

[`command` ソース](/docs/ja/plugins/marketplace-reference#command-plugin-source)の `command` を変更するか、その `mode` を切り替える場合、各ユーザーは Claude Code がそれを実行する前に新しいコマンドを受け入れる必要があります。Claude Code は、ユーザーがプラグインをインストールまたは最後に更新したときに受け入れた正確なコマンドのみを実行します。

ユーザーのマーケットプレイスのコピーが変更を取得した後、そのユーザーは以下を見ます：

* **バックグラウンド実行なし**：コマンドの[セッションごとの実行](/docs/ja/plugins/loading#when-a-command-source-re-runs)はそのユーザーに対して停止するため、ツールの新しい出力はそれらに到達しません。
* **`/plugin` エラータブのエントリ**：エントリは新しいコマンドと実行する `claude plugin update` コマンドを表示します。

ユーザーにそのエントリが表示する `claude plugin update` コマンドをターミナルで実行するよう指示してください。Claude Code は新しいコマンドを表示し、それを受け入れるよう求めます。

<h2 id="run-release-channels">
  リリースチャネルを実行する
</h2>

安定版と早期アクセストラックを提供するには、エントリが同じプラグインの異なる ref を指す 2 つのマーケットプレイスをホストし、各ユーザーが必要なものを追加できるようにしてください。Claude Code にはリリースチャネルの概念がなく、1 つのマーケットプレイスは一度に各プラグインの 1 つのバージョンを提供します。

2 つの `marketplace.json` ファイルに異なる `name` 値を指定してください。Claude Code はマーケットプレイスを `name` で識別するため、ユーザーは一度に同じ名前の 2 つのマーケットプレイスを登録できません。

これら 2 つのカタログでは、`stable-tools` を追加するユーザーは `stable` ブランチから `code-formatter` をインストールし、`latest-tools` を追加するユーザーは `latest` からインストールします：

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

2 つの ref に異なる `plugin.json` バージョンを指定するか、`version` を省略してコミット SHA がそれらを区別するようにしてください。更新はバージョンを比較することで検出されるため、バージョン変更なしで移動する ref はユーザーをキャッシュされたコピーに残します。

チャネルをユーザーが選択させるのではなくユーザーグループに割り当てるには、管理者が各グループに一致する `extraKnownMarketplaces` エントリを指定します。[更新ポリシーを設定する](/docs/ja/plugins/org#set-update-policy)で説明されています。

<h2 id="rename-or-remove-a-plugin">
  プラグインの名前変更または削除
</h2>

プラグインの `name` はその識別子です。ユーザーは `enabledPlugins` と `pluginConfigs` 設定キーおよび `/plugin install` でそれを参照するため、変更するとすべての既存インストールが破損します。

ユーザーが `/plugin` で見るラベルを何も破損させずに変更するには、`plugin.json` で `displayName` を設定し、`name` を変更しないままにしてください。

<h3 id="migrate-users-with-a-renames-map">
  名前変更マップでユーザーを移行する
</h3>

`name` を変更する必要がある場合、`marketplace.json` にトップレベルの `renames` マップを追加して、Claude Code が[`Plugin "<name>" not found in marketplace`](/docs/ja/plugins/troubleshooting#plugin-not-found-in-marketplace)を報告する代わりに既存ユーザーを移行するようにしてください。`plugins` からエントリを削除する場合も同じことをしてください。自動移行には Claude Code v2.1.193 以降が必要です。

各前の名前を現在の名前にマップするか、プラグインが削除されたときは `null` にマップしてください。このマーケットプレイスは `formatter` を `code-formatter` に名前変更し、`legacy-linter` が削除されたことを記録します：

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

プッシュ後、古い名前がまだ有効になっているユーザーは、これらの結果のいずれかを見ます：

* **名前変更されたエントリ**：プラグインは新しい名前で読み込まれます。`claude plugin list` とプラグインの詳細は `/plugin` の下に `Renamed to "code-formatter" in the "your-marketplace" marketplace` を 1 回表示し、Claude Code はユーザー、プロジェクト、ローカル設定スコープの `enabledPlugins` と `pluginConfigs` の古いキーを新しいキーに書き直します。
* **`null` エントリ**：古いキーはそれらのスコープから削除され、ユーザーは `Removed from the "your-marketplace" marketplace` を見ます。
* **管理設定で有効**：プラグインは新しい名前で読み込まれ続けますが、Claude Code は管理設定を書き直すことができないため、管理者がそこで `enabledPlugins` を更新するまで通知が繰り返されます。

git リポジトリまたは URL から追加されたマーケットプレイスの場合、名前変更されたプラグインは、ユーザーがセッションで `/plugin install code-formatter@your-marketplace` を 1 回実行するまで[`Plugin "<name>" not cached at <path>`](/docs/ja/plugins/troubleshooting#plugin-not-cached-at)を報告します。

`renames` を追加のみの履歴として扱ってください。すべてのユーザーが移行した後も古いエントリを保持してください。再度名前変更する場合、最初のエントリを編集するのではなく、2 番目のエントリを追加してください。Claude Code は最も古い名前からチェーンをたどるため。

シェルで、マップを編集した後に `claude plugin validate .` を実行してください。サイクルするか、`null` またはプラグイン内の名前以外の場所で終わるチェーンを拒否します。`renames.<name>: chain does not resolve` で。

<h3 id="uninstall-removed-plugins-from-users’-machines">
  削除されたプラグインをユーザーのマシンからアンインストールする
</h3>

削除されたプラグインをユーザーのマシンから残すのではなくアンインストールするには、`marketplace.json` のトップレベルで `"forceRemoveDeletedPlugins": true` を設定してください。フィールドがない場合、削除されたプラグインはインストールされたままで、セッションが読み込むときに `Plugin "<name>" not found in marketplace` を報告します。それがある場合、Claude Code は各セッション開始時に以下を実行します：

1. ユーザーがマーケットプレイスからインストールしたものをエントリと `renames` マップと比較し、リストされていないか名前変更されていないプラグインを削除されたものとして扱います。
2. ユーザー、プロジェクト、ローカルスコープから各削除されたプラグインをアンインストールします。管理設定のみがインストールしたプラグインは所定の位置に留まります。
3. `/plugin` の**Flagged**見出しの下に各削除されたプラグインをリストします。ステータスは `Removed from marketplace` です。

<h2 id="authenticate-archive-downloads">
  アーカイブダウンロードを認証する
</h2>

[`archive`](/docs/ja/plugins/marketplace-reference#archive-plugin-source) ダウンロード（プライベートレジストリからのダウンロードなど）を認証するには、Claude Code がそれで送信する HTTP ヘッダーを設定してください。これらの場所のいずれかで `headers` を設定できます：

* **マーケットプレイスの `url` ソース**：[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) エントリなど、マーケットプレイスを登録した `url` ソース。
* **プラグインのエントリ**：Claude Code v2.1.238 以降では、代わりに `source` の横にプラグインの `marketplace.json` エントリで設定できます。

どちらの場所でも、値が短命の場合（レジストリが要求時に生成するトークンなど）は、`headers` の代わりに `headersHelper` コマンドを設定してください。Claude Code はコマンドを実行し、その場所のヘッダーとして出力する JSON オブジェクトを送信します。Claude Code v2.1.238 以降が必要です。

[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference#plugin-entries)は `headers` と `headersHelper` エントリフィールドをリストしています。

選択する場所は、どのダウンロードがヘッダーを取得し、Claude Code がコマンドをいつ実行するかを決定します：

| 場所                  | ヘッダーを取得するダウンロード                                 | Claude Code が `headersHelper` をそこで実行するとき                                                              |
| :------------------ | :---------------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| マーケットプレイス `url` ソース | マーケットプレイス URL のオリジン上のアーカイブダウンロード。同じスキーム、ホスト、ポート | マーケットプレイスの `marketplace.json` の各フェッチの前と、そのオリジン上の各アーカイブダウンロードの前。Claude Code は 1 回の実行の出力を最大 60 秒間再利用します |
| プラグインエントリ           | そのエントリのダウンロードのみ                                 | ユーザーがそのプラグインを単独でインストールまたは更新し、[コマンドを受け入れる](#how-users-accept-a-headershelper-command)場合のみ              |

両方の場所が同じ名前のヘッダーを設定する場合、Claude Code はエントリの値を送信します。1 つの場所内で、コマンドが出力するヘッダーは同じ名前のリストされたヘッダーをオーバーライドします。

<h3 id="add-a-headershelper-to-a-plugin-entry">
  プラグインエントリに headersHelper を追加する
</h3>

このエントリは `source` の横に `headersHelper` を設定します。また、[`"strict": false`](/docs/ja/plugins/marketplace-reference#strict-mode)を設定します。これは Claude Code が `headersHelper` を設定する `marketplace.json` エントリに必要とします：

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

エントリを確認するには、シェルで `claude plugin install my-plugin@your-marketplace` を実行してください。Claude Code はコマンドとアーカイブ URL を表示し、受け入れた後に zip をダウンロードします。

<h3 id="write-the-headershelper-command">
  headersHelper コマンドを書く
</h3>

マーケットプレイスの `url` ソースまたはプラグインエントリで `headersHelper` を設定するかどうかにかかわらず、コマンドを書いてこれらの要件を満たしてください：

* **コマンドテキスト**：最大 500 文字の印字可能 ASCII。4 つ以上のスペースの実行なし。
* **出力**：stdout に 1 つの JSON オブジェクトのヘッダー名と文字列値を出力してから、10 秒以内に終了 0 で終了します。
* **シェルと作業ディレクトリ**：Claude Code はコマンドを `sh` を通じて実行するか、Windows では `cmd.exe` を通じて実行します。作業ディレクトリは設定ディレクトリです。`~/.claude` または [`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars#variables)。相対パスはそのディレクトリに対して解決されるため、ユーザーのプロジェクトではなく、絶対パスまたは `PATH` 上のコマンドを指定してください。
* **Claude Code が削除する変数**：コマンドが `marketplace.json` エントリ、またはプロジェクトの `.claude/settings.json` または `.claude/settings.local.json` で設定されている場合、Claude Code は環境から認証情報のように見える名前を持つすべての変数を削除します。[MCP `headersHelper` に適用するのと同じルール](/docs/ja/mcp#which-variables-a-helper-can-read)。`ANTHROPIC_API_KEY` と `MY_REGISTRY_TOKEN` は両方とも削除されるため、コマンドはファイルまたは認証情報ストアから認証情報を読み取ります。この削除はユーザー設定、`--settings` ファイル、または管理設定で設定されたコマンドには適用されません。
* **Claude Code が設定する変数**：`url` ソースのコマンドの場合は `CLAUDE_CODE_MARKETPLACE_URL` と `CLAUDE_CODE_MARKETPLACE_NAME`。エントリのコマンドの場合は `CLAUDE_CODE_PLUGIN_NAME` と `CLAUDE_CODE_PLUGIN_ARCHIVE_URL`。`CLAUDE_CODE_MARKETPLACE_NAME` は、ユーザーが URL でマーケットプレイスを追加した後の最初のフェッチでは設定されていません。そのフェッチが名前を提供するため。

ベアラートークンをミントするコマンドは、このようなオブジェクトを出力します：

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  Claude Code が headersHelper コマンドをスキップするか出力をドロップするとき
</h3>

`headersHelper` コマンドは実行されないか、`headers` からのヘッダーまたはコマンドの出力は、以下のいずれかが適用される場合にドロップされます：

* **コマンドが失敗する**：コマンドが非ゼロで終了するか、10 秒を超えて実行するか、JSON オブジェクト以外の文字列値を出力する場合、コマンドが実行されたフェッチまたはダウンロードは発生しません。
* **マーケットプレイス URL が `https://` で始まらない**：その `url` ソースのコマンドは実行されず、リクエストは `headers` フィールドにリストされたヘッダーのみを実行します。
* **リダイレクトがオリジンを離れる**：ダウンロードがアーカイブ URL のオリジンからリダイレクトされる場合、リダイレクトされたリクエストはマーケットプレイス `url` ソースまたはプラグインエントリからの `headers` 値またはコマンド出力を実行しません。
* **エントリがルーティングまたはアイデンティティヘッダーを設定する**：Claude Code はエントリの `headers` とコマンド出力から `Host`、`Cookie`、`X-Forwarded-*` などのリクエストルーティングおよびクライアントアイデンティティ名をドロップし、`Authorization` などの認証名を保持します。すべての `marketplace.json` エントリはこのようにフィルタリングされます。設定内のインラインプラグインエントリの場合は、[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces)を参照してください。
* **`--add-dir` ディレクトリの設定で設定されたコマンド**：コマンドは無視され、`url` ソースと[インラインプラグインエントリ](/docs/ja/settings-reference#extraknownmarketplaces)の両方で、そのファイルの `headers` のみが送信されます。
* **管理設定がコマンドをブロックする**：[`disableCommandPluginSources`](/docs/ja/settings-reference#disablecommandpluginsources)を `true` に設定するとブロック `headersHelper` コマンド、および [`allowManagedHooksOnly`](/docs/ja/settings-reference#allowmanagedhooksonly) も `disableCommandPluginSources` が明示的に `false` でない限りブロックします。どちらのブロックでも、Claude Code は管理設定自体が宣言するマーケットプレイスのコマンドを実行します。

<h3 id="how-users-accept-a-headershelper-command">
  ユーザーが headersHelper コマンドを受け入れる方法
</h3>

ユーザーは、そのプラグインを単独でインストールまたは更新するたびに、プラグインエントリのコマンドを受け入れます。彼らは `/plugin` のプラグイン自体のビューから、または `claude plugin install` または `claude plugin update` でそれを行います。Claude Code はコマンドとアーカイブ URL を表示し、ユーザーが受け入れた後にのみコマンドを実行します。

非対話的なシェルでは、[`--yes`](/docs/ja/plugins/cli-reference#plugin-install)を渡してコマンドを受け入れます。前の `--json` 実行が表示した、そのコマンドのみを受け入れるには、[`--accept-command`](/docs/ja/plugins/cli-reference#plugin-install)を実行が報告した `sha256` で渡します。

Claude Code は表示したコマンドのみを実行し、表示したアーカイブ URL に対してのみ実行します。エントリのコマンドまたはアーカイブ URL が間に変更された場合、Claude Code はインストールまたは更新を拒否します。クエリ文字列のみの変更はカウントされません。

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  コマンドを要求する代わりに拒否するインストールと更新
</h3>

単一プラグインのインストールまたは更新以外の操作では、Claude Code はエントリのコマンドを実行せず、アーカイブをダウンロードしません。プラグインはインストール済みバージョンに留まるか、インストールされたままで、ユーザーは以下のいずれかを見ます：

* **複数のプラグインを一度にインストールする、プラグイン提案から、または別のプラグインの依存関係として**：Claude Code はコマンドを持つプラグインを拒否し、ユーザーを `/plugin` のそのプラグイン自体のビューに指示します。バルクインストール内の他のプラグインはまだインストールします。プラグインが拒否されたプラグインに依存する場合、ユーザーがそのプラグインを単独でインストールするまで失敗します。
* **バックグラウンド自動更新、またはアーカイブがダウンロードされたことのないプラグインのセッション開始**：Claude Code はプラグインを `/plugin` エラータブにリストして、ユーザーが単独でインストールまたは更新することを知っています。

<h3 id="when-a-marketplace-url-sources-command-runs">
  マーケットプレイス `url` ソースのコマンドが実行されるとき
</h3>

マーケットプレイス `url` ソースの `headersHelper` を、マーケットプレイスが公開するカタログではなく、[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) エントリなどの設定ファイルで宣言します。Claude Code はそれぞれのインストールまたは更新でユーザーに受け入れるよう求めません。代わりに、それを宣言する設定ファイルが Claude Code がそれを実行するときを決定します：

| 設定ファイル                                                            | Claude Code がコマンドを実行するとき                                                                                                                                     |
| :---------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ユーザー設定、`--settings` ファイル、またはマシン上の管理設定ファイル                         | 尋ねずに、バックグラウンドマーケットプレイス更新を含む                                                                                                                                  |
| プロジェクトの `.claude/settings.json` または `.claude/settings.local.json` | ユーザーがそのフォルダ自体の[ワークスペーストラストダイアログ](/docs/ja/permissions#what-runs-before-you-trust-a-folder)を受け入れた後のみ。`-p` または SDK セッションはそれを受け入れるとしてカウントされず、親フォルダに付与された信頼もカウントされません |
| サーバー管理設定                                                          | 対話的なセッションでは、ユーザーが[セキュリティ承認ダイアログ](/docs/ja/server-managed-settings#security-approval-dialogs)で配信された設定を承認した後のみ                                                      |

これらのファイルの 1 つの[インラインプラグインエントリ](/docs/ja/settings-reference#extraknownmarketplaces)の場合、Claude Code はそのファイルのマーケットプレイスレベルのコマンドと同じフォルダ信頼または設定承認を要求し、ユーザーはまたそれぞれのインストールまたは更新でエントリのコマンドを受け入れます。

<h2 id="depend-on-and-recommend-other-plugins">
  他のプラグインに依存し、推奨する
</h2>

エントリは他のプラグインへの依存関係を宣言できます。

* **バージョン範囲**: 依存関係は semver 範囲を含むことができます。
* **クロスマーケットプレイス依存関係**: 別のマーケットプレイスからの依存関係は、お客様のマーケットプレイスが `allowCrossMarketplaceDependenciesOn` にそのマーケットプレイスをリストしている場合にのみインストールされます。

バージョン範囲、`<plugin>--v<version>` git タグ規約がそれらに対して解決される方法、およびクロスマーケットプレイス信頼については、[Plugin dependencies](/docs/ja/plugins/dependencies) を参照してください。

Claude Code がプロジェクトに一致するときにプラグインを提案するようにするには、プロジェクトを識別するシグナルを含む `relevance` ブロックをエントリに追加します。ユーザーは、管理者がそれを `pluginSuggestionMarketplaces` にリストしている場合にのみ、お客様のマーケットプレイスからの提案を表示します。シグナルと有効化ステップについては、[Plugin relevance](/docs/ja/plugins/relevance) を参照してください。

<h2 id="work-around-what-a-marketplace-can’t-do">
  マーケットプレイスができないことへの対応
</h2>

マーケットプレイスの所有者が要求する一部の機能には、`marketplace.json` にフィールドがありません。以下は各機能に対する最も近いオプションです。

* **ユーザーがインストールできるその他のプラグインを制限する**: マーケットプレイスの許可リストは管理設定 `strictKnownMarketplaces` です。[ユーザーがインストールできるものを制限する](/docs/ja/plugins/org#restrict-what-users-can-install) を参照してください。
* **ユーザーが要求せずにプラグインをインストールまたは有効にする**: エントリフィールドはプラグインをインストールしません。管理設定の `enabledPlugins` はフリート全体に対してそれを実行します。[プラグインを事前インストールして必須にする](/docs/ja/plugins/org#pre-install-and-require-plugins) を参照してください。
* **異なるユーザーに異なるエントリを表示する**: エントリには対象ユーザーフィールドがなく、マーケットプレイスを追加するすべてのユーザーがカタログ全体を表示します。異なる対象ユーザー向けに別々のマーケットプレイスをホストしてください。
* **プラグインを非推奨としてマークする**: 非推奨状態はありません。オプションはエントリを削除し、その名前を `renames` で `null` にマップし、オプションで `forceRemoveDeletedPlugins` を設定することです。
* **ユーザーの自動更新をオンにする**: 各ユーザーは `/plugin` の **Marketplaces** でオンにするか、管理者が管理設定で `autoUpdate` を設定します。[自動更新をオンにする](#turn-on-auto-update) を参照してください。
* **Git 認証情報を保持する**: マーケットプレイスフィールドは Git トークンを保持しません。Git でホストされるマーケットプレイスまたはプラグインへのアクセスは、[プライベートマーケットプレイスへのアクセスを許可する](#grant-access-to-a-private-marketplace) に従い、ユーザーの Git セットアップに従います。`archive` ソースの場合、エントリは代わりに [`headers` または `headersHelper`](#authenticate-archive-downloads) を設定できます。

<h2 id="next-steps">
  次のステップ
</h2>

* [マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)：`marketplace.json` フィールド、ソースタイプ、および検証メッセージ
* [組織のプラグインを管理する](/docs/ja/plugins/org)：組織のマシン全体でマーケットプレイスを要求、制限、またはシードする
* [プラグイン依存関係](/docs/ja/plugins/dependencies)：プラグインが依存するプラグインがバージョンを解決できるようにリリースをタグ付けする
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)：ユーザーがマーケットプレイスから追加または更新するときに見るエラー
