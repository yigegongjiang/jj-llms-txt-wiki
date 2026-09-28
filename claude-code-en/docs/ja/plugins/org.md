> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 組織向けの Claude Code プラグインを管理する

> マネージド設定を通じて、組織内のすべてのマシンに Claude Code がインストールして許可するプラグインを制御します。

マネージド設定を使用すると、組織内のすべてのマシンに Claude Code がインストールして許可するプラグインを決定できます。ユーザーはこれらをオーバーライドできません。これらは [サーバーマネージド設定](/docs/ja/server-managed-settings) として claude.ai 管理コンソールから、または MDM または `managed-settings.json` ファイルを通じてエンドポイントマネージド設定として配信できます。このページのほとんどのコントロールはマネージド設定からのみ有効になります。

このページは管理者向けであり、ここの設定は Claude Code を管理します。

<Note>
  これらのケースは他のページで説明されています：

  * **自分自身のためにプラグインをインストールする**: [プラグインをインストール](/docs/ja/plugins/install) から開始してください
  * **claude.ai と Cowork でメンバーが使用できるプラグインを制御する**: ヘルプセンターの [組織向けプラグインを管理する](https://support.claude.com/en/articles/13837433) を参照してください
  * **claude.ai の管理設定のプラグインページ**: [**組織設定 > プラグインとスキル**](https://claude.ai/admin-settings/skills?tab=inventory) はメンバーの claude.ai アカウント向けにプラグインをオンにし、これらは Claude Code に [同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins) として到達します。このページのキーは設定しません
</Note>

セクションはほとんどのロールアウトが取るオーダーに従います：[すべてのユーザーまたはリポジトリごとにプラグインを要求](#pre-install-and-require-plugins)し、[コンテナと CI をシード](#seed-containers-and-ci)し、[ユーザーが追加できるものを制限](#restrict-what-users-can-install)し、[更新ポリシーを設定](#set-update-policy)してから、[インストールされたものを監査](#audit-and-review)します。すべてのポリシーキーを 1 か所で確認するには、[コントロールマトリックス](#control-matrix) を参照してください。

<h2 id="pre-install-and-require-plugins">
  プラグインの事前インストールと要求
</h2>

マーケットプレイスは、Claude Code が git リポジトリ、URL、またはローカルパスから取得するプラグインのカタログです。マシンにマーケットプレイスを登録すると、Claude Code はそこからプラグインをインストールできます。

フリート向けにプラグインをインストールするには、[マネージド設定](/docs/ja/managed-settings) のポリシーファイルまたはサーバー配信ポリシーで 2 つのキーを一緒に設定します。このポリシーは組織内のすべてのマシンが読み取ります：`extraKnownMarketplaces` は各マシンにマーケットプレイスを登録し、`enabledPlugins` はそこからインストールして有効にするプラグインを指定します。[配信メカニズムを選択](#choose-a-delivery-mechanism) はマネージド設定が各マシンに到達する方法をカバーしています。

<h3 id="choose-a-delivery-mechanism">
  配信メカニズムを選択する
</h3>

マネージド設定は、3 つの配信メカニズムのいずれかを通じてマシンに到達します：

* **サーバーマネージド設定**: [**組織設定 > Claude Code > マネージド設定**](https://claude.ai/admin-settings/claude-code) でプラグインキーを JSON として設定します。Claude 組織の [オーナーロール](/docs/ja/server-managed-settings#access-control) が必要です。クラウドセッションはプラグインをインストールする前にこれらの設定を取得します。
* **MDM ポリシー**: macOS では、トップレベルキーが設定キーである plist を配信します。Windows では、JSON ドキュメント全体をレジストリ値の文字列として保存します。plist ドメインとレジストリキーは [各メカニズムがポリシーを保存する場所](/docs/ja/managed-settings#where-each-mechanism-stores-the-policy) にあります。
* **マネージド設定ファイル**: プラットフォームのシステムパスに `managed-settings.json` を配置します。その横の `managed-settings.d/` ドロップイン ディレクトリにファイルを追加することもできます。プラットフォームごとのファイルパスは [各メカニズムがポリシーを保存する場所](/docs/ja/managed-settings#where-each-mechanism-stores-the-policy) にあり、ドロップイン マージルールは [ファイルベースのポリシーをチーム間で分割する](/docs/ja/managed-settings#split-a-file-based-policy-across-teams) にあります。

Claude for Teams または Enterprise 組織が claude.ai にあり、デバイスがすべて MDM の下にない場合は、サーバーマネージド設定を使用してください。それ以外の場合は、MDM ポリシーまたはマネージド設定ファイルを使用してください。トレードオフについては、[サーバーマネージド設定とエンドポイントマネージド設定の選択](/docs/ja/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) を参照してください。

<h4 id="which-managed-source-applies-on-a-machine">
  マシンに適用されるマネージドソース
</h4>

デフォルトでは、これら 3 つのソースのうち 1 つだけがマシンに適用されます。Claude Code は最初にポリシーキーを配信するものを使用し、サーバーマネージド設定をチェックしてから MDM ポリシー、次にマネージド設定ファイルをチェックします。サーバーマネージド設定が関連のないポリシーキーを 1 つでも配信する場合、Claude Code はそのマシンの MDM ポリシーまたはマネージド設定ファイルのプラグインキーを無視します。ただし、[すべてのソースから読み取るキー](/docs/ja/managed-settings#keys-read-from-every-admin-source) は除きます。

すべてのソースを適用するには、[`managedSourcesBehavior`](/docs/ja/managed-settings#compose-every-managed-source) を `"merge"` に設定します。

[Claude Code がマネージドソースを組み合わせる方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources) は、両方のモードで Claude Code がすべてのソースから読み取るキーもリストしています。

<h3 id="require-a-marketplace-and-its-plugins">
  マーケットプレイスとそのプラグインを要求する
</h3>

`extraKnownMarketplaces` の下にマーケットプレイスを追加し、マーケットプレイスの `marketplace.json` からの `name` でキーを付けます。次に、各プラグインを `enabledPlugins` の下に `plugin-name@marketplace-name` として追加します。各マーケットプレイスエントリは、`source` フィールドを持つ `source` オブジェクトを含み、`github` などのタイプを指定します。このマネージド設定の例は、組織マーケットプレイスを登録し、そこから 2 つのプラグインを強制的に有効にします：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

設定がマシンに到達した後、Claude Code はマーケットプレイスを登録し、ユーザーの次のセッションの開始時に 2 つのプラグインをインストールします。ユーザーは `/plugin` でそれらを見ることができ、独自のスコープで 1 つを無効にしても、マネージド設定がすべての他のスコープより優先されるため、読み込みが停止しません。

プラグインをすべてのスコープでブロックしてマーケットプレイスリストから非表示にするには、代わりにマネージド `enabledPlugins` で `false` に設定します。

マーケットプレイスの `autoUpdate` と `source` フィールドを調整します：

* **`autoUpdate`**: `true` はマーケットプレイスとそのプラグインをバックグラウンドで更新し続け、`false` はそれをオフにします。[更新ポリシーを設定](#set-update-policy) を参照してください。
* **`source`**: `github` は複数のソースタイプの 1 つです。`git` ソースは GitLab または内部ホスト用の `url` を取り、`url` ソースはホストされた `marketplace.json` のアドレスを取ります。すべてのソース形状は [マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference) にあります。

マーケットプレイスがプライベート git リポジトリの場合、各ユーザーはそれへの読み取りアクセスが必要です。git ベースのマーケットプレイスのクローンはユーザーのマシンで git を使用して実行され、保存された認証情報を使用してプロンプトなしで実行されます。git ホストアカウントを持たないユーザーの場合は、[シード](#seed-containers-and-ci) を代わりに使用してください。

マネージドエントリは、別のソースからの同じ名前のマーケットプレイスエントリまたは `--plugin-dir` コピーもオーバーライドします：

* **マーケットプレイス**: マネージドマーケットプレイスエントリは、同じ名前の低優先度エントリを置き換え、2 つのエントリのフィールドはマージされません。
* **`--plugin-dir` コピー**: `--plugin-dir` は 1 つのセッション用にローカルディレクトリからプラグインを読み込みます。そのコピーの名前がマネージド `enabledPlugins` が指定するプラグインと一致する場合の動作については、[名前の競合](/docs/ja/plugins/loading#name-conflicts) を参照してください。

Anthropic の公式マーケットプレイス `claude-plugins-official` は、`enabledPlugins` がそのプラグインの 1 つを `true` に設定する場合、`extraKnownMarketplaces` エントリを必要としません。その `name@claude-plugins-official` エントリは、これらのキーが適用される場所ならどこでも、マーケットプレイスを宣言します。そのプラグインのいずれも有効にしないが、それでもすべてのマシンに登録したい場合は、[公式マーケットプレイスと独自のマーケットプレイスを許可する](#allow-the-official-marketplace-and-your-own) のように明示的なエントリを付与します。

<h3 id="require-plugins-per-repository">
  リポジトリごとにプラグインを要求する
</h3>

フリート全体ではなく 1 つのリポジトリの貢献者をカバーするには、そのリポジトリの `.claude/settings.json` で `extraKnownMarketplaces` と `enabledPlugins` を設定します。`extraKnownMarketplaces` エントリは、貢献者が信頼したフォルダ内でのみ適用され、信頼されていないフォルダでは Claude Code はメッセージなしでそれらを無視します：

* **インタラクティブセッション**: Claude Code は、貢献者がそのフォルダの [ワークスペース信頼ダイアログ](/docs/ja/permissions#what-runs-before-you-trust-a-folder) を受け入れた後にのみマーケットプレイスを登録します。
* **[非インタラクティブ `-p` 実行](/docs/ja/headless)**: エントリは、ユーザーが既にインタラクティブに信頼を受け入れたフォルダ、または `~/.claude.json` で `hasTrustDialogAccepted` フラグを設定したフォルダでのみ適用されます。

マーケットプレイスが相対パスでリストするプラグインは、リポジトリの `extraKnownMarketplaces` エントリが適用されると、マーケットプレイスコピーから読み込まれます。マーケットプレイスエントリが代わりにプラグイン独自の GitHub リポジトリなどの外部ソースを指すプラグインは、リポジトリの設定だけからはインストールされません。各貢献者は、[プラグインをインストール](/docs/ja/plugins/install) で説明されているように、`claude plugin install <name>@<marketplace> --scope project` を実行するまで `Plugin "<name>" is enabled in project settings but isn't installed` を見ます。

相対パスで `directory` または `file` ソースを使用する場合、パスはリポジトリのメインチェックアウトに対して解決されます。git worktree から Claude Code を実行する場合、パスはまだメインチェックアウトを指すため、すべての worktree は同じマーケットプレイスの場所を共有します。

依存関係を持つプラグインのバンドルをロールアウトするには、[プラグインの依存関係](/docs/ja/plugins/dependencies) で説明されているように、バンドルプラグインを `enabledPlugins` に入れます。

<h3 id="when-each-surface-applies-the-plugin-keys">
  各サーフェスがプラグインキーを適用する場合
</h3>

テーブルは、マネージド設定とリポジトリの `.claude/settings.json` から、各種 Claude Code セッションが `extraKnownMarketplaces` と `enabledPlugins` を適用する場合を示しています。Desktop アプリと IDE 拡張機能については、[プラグインをインストール](/docs/ja/plugins/install#install-a-plugin) を参照してください。

| サーフェス          | マネージド `extraKnownMarketplaces` と `enabledPlugins`                                                                                                                                                    | リポジトリ `.claude/settings.json`                                                    |
| :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| ターミナル、インタラクティブ | 設定を受け取るすべてのマシンでセッション開始時に適用                                                                                                                                                                           | `extraKnownMarketplaces` は信頼後に適用；`enabledPlugins` はセッション開始時に適用                   |
| `-p` と CI      | セッション開始時に適用、インストールはバックグラウンドで実行                                                                                                                                                                       | `extraKnownMarketplaces` は信頼されたフォルダのみ；`enabledPlugins` は適用                       |
| クラウドセッション      | Anthropic ホスト環境では、サーバーマネージド設定のみがセッションに到達し、プラグインをインストールする前にそれを待ちます。MDM ポリシーとマネージド設定ファイルはユーザーのマシンに留まります。自己ホスト環境については、[ポリシーが適用される場所と時期](/docs/ja/managed-settings#where-and-when-a-policy-applies) を参照してください | [プラグインをインストール](/docs/ja/plugins/install#install-a-plugin) の **クラウドセッション** タブを参照してください |

`-p` または CI 実行では、マーケットプレイスとプラグインはバックグラウンドでインストールされるため、プラグインは最初のターンから欠落する可能性があります。`CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` を設定して、最初のクエリの前にインストールを待つようにします。

<h3 id="confirm-the-rollout">
  ロールアウトを確認する
</h3>

マーケットプレイスとプラグインがマシンまたは CI 実行に到達したことを確認します：

* **1 つのマシン上**: Claude Code を開始して `/plugin` を実行します。マーケットプレイスとプラグインがリストされます。
* **CI 内**: `claude -p` を `--output-format stream-json --verbose` で実行します。`init` イベントは `plugins` の下に読み込まれたプラグインをリストします。

<h2 id="seed-containers-and-ci">
  コンテナと CI をシードする
</h2>

実行時にクローンできないコンテナイメージと CI ランナーの場合、ビルド時にプラグインディレクトリを事前に入力し、`CLAUDE_CODE_PLUGIN_SEED_DIR` でそれを指します。Claude Code はスタートアップでシードのマーケットプレイスを登録し、クローンなしでシードからプラグインキャッシュを読み込みます。

シードは、git ホストアカウントを持たないユーザーにも役立ちます。

<Note>
  CI/CD 環境では、プライベートリポジトリからプラグインをインストールする前に git 認証情報ヘルパーを設定してください。GitHub Actions では、マーケットプレイスリポジトリへの読み取りアクセス権を持つトークンを `GH_TOKEN` として エクスポートしてから、`gh auth setup-git` を実行します。デフォルトワークフロートークンはワークフロー独自のリポジトリにのみアクセスできるため、別のリポジトリのプライベートマーケットプレイスには個人用アクセストークンまたはアプリトークンが必要です。
</Note>

<Steps>
  <Step title="ビルド時にシードにインストールする">
    `CLAUDE_CODE_PLUGIN_CACHE_DIR` をシードパスに設定して、マーケットプレイスとプラグインが `~/.claude/plugins` の代わりにそこにインストールされるようにします：

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    シードは `~/.claude/plugins` と同じレイアウトを持ちます：`known_marketplaces.json`、`marketplaces/<name>/`、および `cache/<marketplace>/<plugin>/<version>/`。シードをビルドしたパスとは異なるパスにマウントできます。
  </Step>

  <Step title="ランタイムをシードに指す">
    コンテナの環境で `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` を設定します。複数のシードを使用するには、Unix では `:` で、Windows では `;` でパスを分離します。Claude Code は、指定されたマーケットプレイスまたはプラグインキャッシュを含む最初のシードを使用します。
  </Step>

  <Step title="プラグインを有効にする">
    シード内のプラグインは独自に有効になりません。読み込みたい各シードプラグインに対して、マネージド設定またはリポジトリの `.claude/settings.json` で `enabledPlugins` を設定します。
  </Step>
</Steps>

シードを検証するには、イメージで `claude -p` を `--output-format stream-json --verbose` で実行します。`init` イベントの `plugins` リストで、各読み込まれたプラグインの `path` はシードの下にあります。例えば `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`。

シードマーケットプレイスは以下のルールに従います：

* **読み取り専用**: Claude Code はシードに書き込まず、シードマーケットプレイスに対して `autoUpdate` をオフに強制します。
* **シードエントリが優先**: 各スタートアップで、シードで宣言されたマーケットプレイスは同じ名前のユーザーエントリを上書きします。ユーザーはマーケットプレイスを削除することではなく、`claude plugin disable` でシードプラグインをオプトアウトします。
* **更新と削除が失敗**: `claude plugin marketplace update <name>` と `remove` は、シードマーケットプレイスで `--scope` なしで失敗し、シードディレクトリを指定するメッセージが表示されます。
* **ポリシーはまだ適用**: [許可リストとブロックリスト](#restrict-what-users-can-install) はシードマーケットプレイスの記録されたソースもチェックします。ビルドしたソースを許可します。

アウトバウンド git アクセスがないフリートの場合、シードを共有マウント上の `directory` または `file` マーケットプレイスソースと組み合わせます。`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` も設定します。これは [プラグイン自動更新](/docs/ja/plugins/loading#when-auto-update-runs) もオフにします。プロキシが利用可能な場合は、[プロキシ設定](/docs/ja/network-config#proxy-configuration) を参照して設定する変数を確認してください。

<h2 id="restrict-what-users-can-install">
  ユーザーがインストールできるものを制限する
</h2>

マネージド `strictKnownMarketplaces` 許可リストと `blockedMarketplaces` ブロックリストは、プラグインがどのマーケットプレイスソースから来るかを決定します。マーケットプレイスのソースは、Claude Code がそれを取得する git リポジトリ、URL、またはローカルパスです。両方のリストは、プラグイン内のプラグイン独自のエントリではなく、プラグインが来るマーケットプレイスのソースと一致します。

公式マーケットプレイスと独自のマーケットプレイスを許可する一般的なロックダウンについては、[公式マーケットプレイスと独自のマーケットプレイスを許可する](#allow-the-official-marketplace-and-your-own) を参照してください。ユーザーがローカルディレクトリまたは URL からプラグインを読み込めないようにするために [`disableSideloadFlags`](#control-matrix) と組み合わせます。

両方のリストは、ダウンロード前とセッション開始時に適用されます：

* **ダウンロード前**: リストは、ユーザーがマーケットプレイスを追加し、すべてのインストール、更新、更新、および自動更新時に適用されます。
* **セッション開始時**: リストは既にインストールされているプラグインに再度適用されるため、マーケットプレイスソースがもはや一致しないインストール済みプラグインは読み込まれません。`/plugin` は `Marketplace "<name>" is not in the allowed marketplace list` または `Marketplace "<name>" is blocked by enterprise policy` でリストします。

2 つのリストが適用される場所は、それらを設定する場所によって異なります：

* **claude.ai 管理コンソール**: Claude Code は [サーバーマネージド設定を読み取る](/docs/ja/managed-settings#where-and-when-a-policy-applies) セッションで両方のリストを適用します。claude.ai は、組織内の誰かが claude.ai から新しいマーケットプレイスを git リポジトリから追加するか、Claude Desktop アプリの Code タブ外から **カスタマイズ** から追加する場合もチェックします。これは、メンバーが独自のアカウント用に追加するマーケットプレイスと、[**組織設定 > プラグイン**](https://claude.ai/admin-settings/plugins) の下で組織全体に追加されるマーケットプレイスをカバーします。claude.ai は、許可リストが認めないリポジトリまたはブロックリストが指定するリポジトリを拒否します。リストを設定する前にどちらかの場所で追加されたマーケットプレイスを再チェックしません。また、アップロードされたプラグインもチェックしません。
* **マネージド設定ファイル、OS レベルのポリシー、または他のマネージドソース**: Claude Code は、そのソースを読み取る場所で両方のリストを適用します。claude.ai はそれを読み取りません。

許可リストが設定されている場合、またはブロックリストが [`skills-dir`](#blocklist-with-blockedmarketplaces) 以外のソースを指定する場合、Claude Code が見つけられないマーケットプレイスを持つプラグインは読み込まれません。`/plugin` は見つからないエラーではなくポリシーエラーを表示します。一般的なケースは、誰も登録しなかったマーケットプレイスの古い `enabledPlugins` エントリです。

<h3 id="control-matrix">
  コントロールマトリックス
</h3>

テーブルは各プラグインポリシーキー、それが適用するもの、および実行できないことをリストしています。

| キー                                                                       | 適用するもの                                                                                                                                                                                          | 実行できないこと                                                                                                                  |
| :----------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `strictKnownMarketplaces`                                                | マーケットプレイスソースの許可リスト。`[]` は公式マーケットプレイスを含むすべてのソースをブロックします。エイリアス：`allowedMarketplaces`                                                                                                              | マーケットプレイスを登録、許可されたマーケットプレイス内のエントリを制限、または `--plugin-dir` をブロックしません                                                         |
| `blockedMarketplaces`                                                    | マーケットプレイスソースのブロックリスト、許可リストの前にチェック                                                                                                                                                               | 既に登録されているマーケットプレイスをそれが一致しないソースからブロックしません                                                                                  |
| `syncClaudeAiPlugins`                                                    | `false` に設定して、Claude Code が各ユーザーのアカウント用に [claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins) をダウンロードして読み込むのを停止します。Claude Code v2.1.273 以降が必要です                                         | 1 つの同期されたプラグインをオフにしません。そのためには、[`enabledPlugins`](/docs/ja/settings-reference#enabledplugins) で `"<name>@synced": false` を設定します |
| `enabledPlugins`                                                         | `true` は強制的に有効にし、`false` はすべてのスコープでブロックしてプラグインを非表示にします                                                                                                                                          | マーケットプレイスが登録または許可されていないプラグインをインストールしません                                                                                   |
| `disableSideloadFlags`                                                   | `--plugin-dir`、`--plugin-url`、`--agents`、Agent SDK `plugins` オプション、および非 SDK `--mcp-config` をスタートアップで拒否し、[`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ja/env-vars#variables) 変数で指定されたフォルダを同じ方法で拒否します             | `.mcp.json`、`claude mcp add`、または SDK 提供サーバーを制限しません。[`allowedMcpServers`](/docs/ja/managed-mcp) と組み合わせます                        |
| `disableCommandPluginSources`                                            | `command` ソースを持つプラグインがインストール、更新、または読み込みされるのをブロックします。`command` ソースは、プラグインディレクトリがマシンでコマンドを実行することで生成されるものです。設定されていない場合、`allowManagedHooksOnly` の値を取ります                                             | 他のソースタイプに影響しません                                                                                                           |
| `allowManagedHooksOnly`                                                  | どのフックが実行されるかを制限します。[`allowManagedHooksOnly`](/docs/ja/settings-reference#allowmanagedhooksonly) を参照してください                                                                                            | ユーザーが自分で有効にするプラグインからのフックを信頼しません                                                                                           |
| `strictPluginOnlyCustomization`                                          | プラグイン、マネージド設定、または Claude Code の組み込みから来ないスキル、エージェント、フック、および MCP サーバーをブロックします。すべての 4 つのタイプをカバーするには `true` に設定するか、`["skills", "hooks"]` などの `skills`、`agents`、`hooks`、および `mcp` 値の配列に設定して一部をカバーします | ユーザーがインストールするプラグインを制限しません。`strictKnownMarketplaces` と組み合わせます                                                              |
| `pluginSuggestionMarketplaces`                                           | プラグインがインストール提案として表示される可能性があるマーケットプレイス。[プラグインを推奨する](#recommend-plugins) を参照してください                                                                                                                | 組み込みのヒントに影響しません                                                                                                           |
| `pluginTrustMessage`                                                     | プラグインがインストールされる前に `/plugin` が表示する信頼警告にテキストを追加します                                                                                                                                                | 警告独自のテキストを変更しません                                                                                                          |
| `allowedChannelPlugins`                                                  | チャネルメッセージをプッシュできるプラグインのデフォルトリストを置き換えます。`channelsEnabled: true` が必要です                                                                                                                            | [チャネルプラグインが実行できるものを制限する](/docs/ja/channels#restrict-which-channel-plugins-can-run) を参照してください                                   |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/ja/env-vars) | インタラクティブターミナルセッションが公式マーケットプレイスを自動登録するのを停止します                                                                                                                                                    | 既に登録されているマーケットプレイスを削除しません。許可リストとブロックリストはそれなしで同じ自動登録をゲートします。それを設定して開始したマシンは、設定を解除した後に自動登録を再開しません                           |

テーブルのすべてのキーはマネージド設定です。ただし、`enabledPlugins`、`syncClaudeAiPlugins`、および `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` は除きます：

* **`enabledPlugins`**: 任意のスコープで設定でき、マネージド設定がそれをロックします。
* **`syncClaudeAiPlugins`**: 各ユーザーは独自のユーザーまたはローカル設定でも設定できます。[設定リファレンス](/docs/ja/settings-reference#syncclaudeaiplugins) でそのスコープを参照してください。
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: これは、[フリート全体の更新をオフにする](#turn-updates-off-for-the-whole-fleet) の下に示されているマネージド `env` ブロックを通じて配信する環境変数です。

ここの各設定キーは [設定リファレンス](/docs/ja/settings-reference) にエントリを持ちます。

<h4 id="aliases-for-the-marketplace-keys">
  マーケットプレイスキーのエイリアス
</h4>

`strictKnownMarketplaces` は `allowedMarketplaces` とも綴ることができ、`extraKnownMarketplaces` は `additionalMarketplaces` とも綴ることができます。

* **バージョン**: エイリアスは Claude Code v2.1.232 以降が必要であり、古いクライアントはそれらを無視します。混合フリートが読み取るファイルでは、正規名を保持してください。
* **両方の綴りが設定**: ファイルが両方の綴りを設定する場合、正規キーの値が適用されます。

<h3 id="allowlist-with-strictknownmarketplaces">
  `strictKnownMarketplaces` を使用した許可リスト
</h3>

許可リストをこれらのソースオブジェクトのリストに設定します。ほとんどのエントリは正確に一致し、`hostPattern` と `pathPattern` エントリは正規表現として一致し、`github` オーナーワイルドカードはオーナーで一致します：

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`、オプションの `ref` と `path` を使用。
* **`github` オーナーワイルドカード**: `{ "source": "github", "repo": "your-org/*" }` はそのオーナーの下のすべてのリポジトリと一致します。`*` はリポジトリ名全体を表す必要があります。Claude Code は `*/plugins` や `your-org/tools-*` などのエントリを無効として無視するため、何も一致しません。Claude Code v2.1.223 以降が必要です。
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`、オプションの `ref` と `path` を使用。
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`、オプションの `headers` を使用。
* **`file` と `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` または `{ "source": "directory", "path": "/opt/marketplace/plugins" }`、絶対パスを使用。
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`、`github`、`git`、および `url` ソースのホストと照合。パターンはホスト名のどこかで一致するため、示されているように `^` と `$` でアンカーしてホスト全体と一致させます。`github` ソースは常に `github.com` としてカウントされます。開発者が独自のマーケットプレイスを作成する GitHub Enterprise Server または GitLab ホストに `hostPattern` エントリを使用します。[GHES ページ](/docs/ja/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) に実装例があります。
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`、`file` と `directory` ソースの `path` と照合。パターンはパスのどこかで一致するため、ディレクトリプレフィックスをピンするために `^` で開始します。`".*"` はすべてのローカルパスを許可します。
* **`skills-dir`**: `{ "source": "skills-dir" }` は、許可リストが設定されている間 [スキルディレクトリプラグイン](#keep-skills-directory-plugins-loading) を読み込み続け、マーケットプレイスと一致しません。

<h4 id="how-entries-match">
  エントリがどのように一致するか
</h4>

`url` エントリは `url` 値で一致します；`headers` は比較されません。`github` と `git` エントリの場合、`repo` または `url`、`ref`、および `path` はすべて一致するか、両側で存在しない必要があります：

* `ref` のないエントリは `ref: "main"` を持つソースをカバーしません。
* `your-org/your-marketplace` のエントリは、同じリポジトリをクローンする `git` URL をカバーしません。
* 末尾のスラッシュ、`.git` サフィックス、または `https://` の代わりの `ssh://` は異なる値です。マーケットプレイスを複数の URL でクローンできる場合は、`hostPattern` エントリを優先します。

オーナーワイルドカードエントリは `ref` の正確なルールに従い、エントリが 1 つをピンしない限り、リポジトリ内のすべての `path` と一致します。ワイルドカード一致は許可リストで大文字と小文字を区別します。

<h4 id="keep-skills-directory-plugins-loading">
  スキルディレクトリプラグインを読み込み続ける
</h4>

スキルディレクトリプラグインは、ユーザーが `~/.claude/skills/` または `.mcp.json` を持つプロジェクトの `.claude/skills/` の下に `.claude-plugin/plugin.json` マニフェストを持つフォルダに保持するプラグインです。`{ "source": "skills-dir" }` エントリなしで許可リストを設定する場合、それらは読み込みを停止します。プレーン [スキル](/docs/ja/skills)、つまり `SKILL.md` はそのマニフェストなしで読み込み続けます。

<h4 id="marketplaces-hosted-on-claude-ai">
  claude.ai でホストされているマーケットプレイス
</h4>

許可リストとブロックリストは、[claude.ai でホストされているマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai) をそのホストで一致させます。1 つを許可またはブロックするには、`claude.ai` と一致する `hostPattern` エントリを `strictKnownMarketplaces` または `blockedMarketplaces` に追加します。許可リストでは、そのようなエントリは組織の claude.ai マーケットプレイスと claude.ai デフォルトマーケットプレイスを認めますが、メンバー独自の claude.ai アップロードで構成されるマーケットプレイスや claude.ai が範囲を述べなかったマーケットプレイスは認めません。Claude Code v2.1.273 以降が必要です。

<h4 id="lock-every-source-out">
  すべてのソースをロックアウトする
</h4>

空の許可リスト `[]` は、公式マーケットプレイスを含むすべてのマーケットプレイスソースをロックアウトします。

このロックダウンは [claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins) をカバーしません。これは Claude Code が各ユーザーのアカウントからマーケットプレイスではなくダウンロードします。それらも停止するには、マネージド設定で [`syncClaudeAiPlugins`](/docs/ja/settings-reference#syncclaudeaiplugins) を `false` に設定するか、claude.ai で組織のスキルをオフにします。

<h3 id="blocklist-with-blockedmarketplaces">
  `blockedMarketplaces` を使用したブロックリスト
</h3>

`blockedMarketplaces` は [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) と同じソースオブジェクトを取り、最初にチェックされるため、両方のリストのソースはブロックされます。ブロックリスト一致は許可リスト一致より広いです：

* Git URL は正規化されるため、1 つの `github.com` リポジトリの `git@` と `https://` フォーム、`.git` サフィックス、および末尾のスラッシュはすべて同じエントリと一致します。
* `github` エントリは同等の `git` URL もブロックし、その逆も同様です。
* `owner/*` エントリの場合、オーナー比較は大文字と小文字を区別しません。
* `ref` または `path` のないエントリは、一致するリポジトリのすべての ref とパスをブロックします。

このエントリは 1 つの GitHub オーナーの下のすべてのリポジトリをブロックします：

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

`blockedMarketplaces` の `url` エントリは、ユーザーが Claude Code が [フェッチではなくクローンする](/docs/ja/plugins/cli-reference#plugin-marketplace-add) `https://` リポジトリ URL を追加する場合にも適用されます。例えば、ベアな `github.com` または `gitlab.com` リポジトリ URL。ユーザーはエントリが指定する URL を追加できません。一致は `.git` サフィックスとユーザーが `#` の後に追加する ref を無視します。Claude Code v2.1.232 以降が必要です。

`{ "source": "skills-dir" }` エントリはここで [スキルディレクトリプラグイン](#keep-skills-directory-plugins-loading) が `~/.claude/skills/` とプロジェクトの `.claude/skills/` の両方から読み込みを停止します。

そのエントリのみを指定するブロックリストは、アクティブな制限としてカウントされないため、[Claude Code が見つけられないマーケットプレイスを持つプラグイン](#restrict-what-users-can-install) が読み込みを停止しません。

<h3 id="allow-the-official-marketplace-and-your-own">
  公式マーケットプレイスと独自のマーケットプレイスを許可する
</h3>

ほとんどの組織は公式マーケットプレイスと独自のマーケットプレイスを許可し、両方を登録してすべてのマシンがそれらを持つようにします。このマネージド設定ポリシーは両方のマーケットプレイスを許可し、両方を登録し、 2 つのプラグインを強制的に有効にし、`--plugin-dir` を拒否します：

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

このポリシーを持つマシンでは、リストの外のソースを追加しようとすると、例えば `/plugin marketplace add https://example.com/other-marketplace.git` は `is blocked by enterprise policy` を含むメッセージで失敗し、許可されたソースが続きます。`claude --plugin-dir ./x` は `disableSideloadFlags` を指定するメッセージで終了します。

`{ "source": "skills-dir" }` エントリは、このポリシーが行うように、このポリシーの下で [スキルディレクトリプラグイン](#keep-skills-directory-plugins-loading) を読み込み続けます。そのエントリを削除すると、それらは読み込みを停止します。

許可リストまたは公式マーケットプレイスが自分自身を登録することに依存するのではなく、このポリシーが行うように明示的な `extraKnownMarketplaces` エントリで両方のマーケットプレイスを登録します：

* **許可リストは何も登録しません**: `extraKnownMarketplaces` エントリは登録し、それ自体が許可リストを通す必要があります。Claude Code は、ソースが許可リストと一致しないマネージドマーケットプレイスを登録することを拒否します。
* **公式マーケットプレイスはインタラクティブターミナルセッションでのみ自分自身を登録します**: そこでも、許可リストが許可する場合にのみ登録します。`-p` 実行またはクラウドセッションに接続されたターミナルは決して登録しません。
* **ブロックされた試みは記憶されます**: マシンが公式マーケットプレイスをブロックしたポリシーの下で実行された場合、Claude Code はブロックされた試みを記録し、ポリシーが変更された後に再試行しません。`[]` ロックダウンはそのようなポリシーの 1 つです。そのマシンは、このポリシーのような `extraKnownMarketplaces` エントリ、そのプラグインの 1 つの `enabledPlugins` エントリ、または手動の `/plugin marketplace add` を通じてのみ再度登録します。

<h2 id="set-update-policy">
  更新ポリシーを設定する
</h2>

マーケットプレイスごと、フリート全体、またはリリースチャネルを通じてユーザーグループごとに更新ポリシーを設定できます。

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  マーケットプレイスごとに自動更新をオンまたはオフにする
</h3>

プラグイン自動更新は、それをオンにしたマーケットプレイスのスタートアップ後にバックグラウンドで実行されます。デフォルトでどのマーケットプレイスがオンになっているかについては、[自動更新が実行される場合](/docs/ja/plugins/loading#when-auto-update-runs) を参照してください。フリートに対して決定するには、マネージド `extraKnownMarketplaces` エントリで `"autoUpdate": true` または `false` を設定します：

* マネージドエントリがフィールドを設定する場合、Claude Code はユーザーの `/plugin` トグルを `Auto-update for '<name>' is set by` で始まるエラーで拒否します。
* マネージドエントリがフィールドを設定しない場合、ユーザーのトグルは保持されます。

<h3 id="turn-updates-off-for-the-whole-fleet">
  フリート全体の更新をオフにする
</h3>

すべてのマーケットプレイスのプラグイン自動更新をオフにするには、この例が行うようにマネージド `env` ブロックで `DISABLE_AUTOUPDATER` を設定します。同じ変数は Claude Code 独自の更新も停止します：

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Claude Code 独自の更新を停止しながらプラグイン自動更新を保持するには、同じブロックに `"FORCE_AUTOUPDATE_PLUGINS": "1"` を追加します。[プラグイン自動更新を停止する他の環境変数](/docs/ja/plugins/loading#when-auto-update-runs) は同じ方法で機能します。

`DISABLE_AUTOUPDATER` は [`command` ソース](/docs/ja/plugins/marketplace-reference#command-plugin-source) を持つプラグインをカバーしません。Claude Code は有効なものの各コマンドを毎セッション再実行し、変更されたときに出力をインストールします。それらの実行を停止するものについては、[コマンドソースが再実行される場合](/docs/ja/plugins/loading#when-a-command-source-re-runs) を参照してください。

<h3 id="assign-release-channels-to-user-groups">
  ユーザーグループにリリースチャネルを割り当てる
</h3>

安定版と早期アクセスチャネルを実行するには、同じプラグインの異なる ref を指す 2 つのマーケットプレイスをホストします。次に、各ユーザーグループに独立したエンドポイントマネージド設定またはゲートウェイポリシーを通じて独自のマーケットプレイスを付与します。管理コンソールからのサーバーマネージド設定は [組織内のすべてのユーザーに適用](/docs/ja/server-managed-settings#current-limitations) されるため、異なるグループに異なる設定を割り当てることはできません。

* マネージド設定ファイルまたは MDM プロファイルなどの独立した [エンドポイントマネージド設定](/docs/ja/managed-settings#delivery-mechanisms) を各グループのデバイスに展開します。組織全体のソースも持つデバイスにグループごとのファイルまたはプロファイルが適用されるかどうかを確認するには、[Claude Code がマネージドソースを組み合わせる方法](/docs/ja/managed-settings#precedence-within-the-managed-tier) を参照してください。
* グループごとに 1 つの [Claude アプリゲートウェイポリシー](/docs/ja/claude-apps-gateway-config#managed) を定義します。ゲートウェイは一致ルールがユーザーに適合する最初のポリシーを適用するため、各ユーザーがグループのポリシーに到達するようにポリシーを順序付けます。そのポリシーの `extraKnownMarketplaces` マップは他のポリシーのマップとマージされないため、グループが必要とするすべてのマーケットプレイスをリストします。チャネルマーケットプレイスのみではなく。

どちらのメカニズムでも、安定版グループはこの設定を受け取ります：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

早期アクセスグループは代わりに `latest-tools` を受け取ります。2 つのマーケットプレイスを設定するには、[リリースチャネルを実行する](/docs/ja/plugins/host-marketplace#run-release-channels) を参照してください。

<h2 id="recommend-plugins">
  プラグインを推奨する
</h2>

マーケットプレイスオーナーは、プロジェクトが一致するときに Claude Code がプラグインを提案するようにエントリに `relevance` シグナルを添付できます。

マーケットプレイスからの提案は、ユーザーのマシンに登録されている場合、マネージド設定で `pluginSuggestionMarketplaces` にその名前をリストしている場合、および同じポリシーでそのソースを宣言している場合にのみ表示されます。ソースをマーケットプレイスの `extraKnownMarketplaces` エントリまたは許可リストエントリとして宣言します。公式マーケットプレイスは名前のみが必要です。[マネージド設定で提案を有効にする](/docs/ja/plugins/relevance#enable-suggestions-in-managed-settings) を参照してください。

<h2 id="audit-and-review">
  監査とレビュー
</h2>

OpenTelemetry イベントと Analytics API により、フロートがインストールして実行するものを確認できます。

プラグインがマシンで実行できるもの、および各信頼レベルが許可するものについては、マーケットプレイスを承認する前に [プラグインセキュリティ](/docs/ja/plugins/security) をお読みください。

<h3 id="opentelemetry-events">
  OpenTelemetry イベント
</h3>

`claude_code.plugin_installed` は各インストールを記録し、`claude_code.plugin_loaded` はセッション開始時に有効になっているプラグインを記録します。両方のイベントは、`OTEL_LOG_TOOL_DETAILS=1` を設定しない限り、サードパーティプラグインおよびマーケットプレイス名を編集するか省略します。これは [バックエンドでのマスクされたプラグイン名](/docs/ja/plugins/measure#redacted-plugin-names-in-your-backend) に示されています。フィールドリストは [プラグインインストールイベント](/docs/ja/monitoring-usage#plugin-installed-event) および [プラグイン読み込みイベント](/docs/ja/monitoring-usage#plugin-loaded-event) にあります。

<h3 id="analytics-api">
  Analytics API
</h3>

Enterprise プランでは、`GET /v1/organizations/analytics/plugins` は Claude Code と Cowork 全体でプラグインごと、日ごとのインストール数と呼び出し数を返します。ユーザーまたは RBAC グループごとに数をグループ化できます。プラグイン名なしで Anthropic に到達するプラグインアクティビティは、1 つの集約 `third-party` 行に表示されます。[エンドポイントリファレンス](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) および [プログラムでデータにアクセス](/docs/ja/analytics#access-data-programmatically) を参照して、必要な API キーを確認してください。

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  マネージド設定が適用できないものを計画する
</h2>

セキュリティレビューからのこれらのリクエストは、現在の設定スキーマに専用キーがありません。最も近い既存のコントロールは：

* **ユーザーごとまたはグループごとのターゲティング**: すべてのプラグインキーは、設定を受け取るすべてのユーザーに適用されます。サーバーマネージド設定は組織ごとに 1 つの設定を配信します。グループごとのポリシーについては、[ユーザーグループにリリースチャネルを割り当てる](#assign-release-channels-to-user-groups) の下のように独立したエンドポイントマネージド設定またはゲートウェイポリシーを使用してください。
* **許可されたマーケットプレイス内のエントリを制限**: 許可リストはマーケットプレイスソースと一致します。許可されたマーケットプレイスから 1 つのプラグインをブロックするには、マネージド `enabledPlugins` で `false` に設定します。
* **`/plugin` を非表示にする**: キーはコマンドを無効にしません。最も近い同等物は、マーケットプレイスのみを指定する許可リスト、提供するプラグイン用のマネージド `enabledPlugins` エントリ、および `disableSideloadFlags` を組み合わせます。
* **許可リストを通じて `--plugin-dir` をゲートする**: 許可リストは `--plugin-dir` をカバーしません。`disableSideloadFlags` はカバーします。
* **これらのキーを通じて claude.ai プラグイントグルを適用する**: [**組織設定 > プラグインとスキル**](https://claude.ai/admin-settings/skills?tab=inventory) はこのページのキーを設定しません。メンバーと組織がそこでオンにするものは CLI に [同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins) として到達し、独自のコントロールを持ちます。

<h2 id="troubleshoot-policy">
  ポリシーをトラブルシューティングする
</h2>

プラグインポリシーがマシンで期待どおりに動作しない場合は、最初にこれらの症状をチェックしてください：

* **マネージドファイルが解析されませんでした**: `managed-settings.json` が有効な JSON でない場合、Claude Code は起動を拒否し、[ファイルを指定するエラー](/docs/ja/errors#managed-settings-document-could-not-be-parsed) を出力します。解析されるが無効なエントリを 1 つ持つファイルは、ポリシーの残りを保持します。[マネージド設定の無効なエントリ](/docs/ja/managed-settings#invalid-entries-in-managed-settings) を参照してください。
* **マネージドソースが読み込まれませんでした**: `/status` を実行し、`Setting sources` 行で `Enterprise managed settings` を探します。欠落している場合、ソースは読み込まれませんでした。
* **ユーザーが `blocked by enterprise policy` を報告**: メッセージはマーケットプレイスまたはそのソースを指定します。許可リストの場合、許可されたソースもリストします。ユーザー向けエントリは [プラグインをトラブルシューティング](/docs/ja/plugins/troubleshooting) にあります。
* **ユーザーが `~/.claude/settings.json` で無効にしたプラグインがまだ読み込まれます**: 別の設定ソースが再度有効にしました。例えば、それを強制的に有効にするマネージド `enabledPlugins` エントリ。`/plugin` と `claude plugin list` は `Disabled in ~/.claude/settings.json but still loads` をその設定ソースで表示します。

<h2 id="next-steps">
  次のステップ
</h2>

* [マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference#marketplace-sources): `extraKnownMarketplaces`、`strictKnownMarketplaces`、および `blockedMarketplaces` が受け入れる `source` 値
* [マーケットプレイスをホストして維持する](/docs/ja/plugins/host-marketplace): ポリシーが指す マーケットプレイスを実行する
* [プラグインセキュリティと信頼](/docs/ja/plugins/security): プラグインがマシンで実行できるもの、およびインストール前に 1 つをレビューする方法
* [サーバーマネージド設定](/docs/ja/server-managed-settings): claude.ai 管理コンソールからこれらのキーを配信する
* [プラグインをトラブルシューティング](/docs/ja/plugins/troubleshooting#blocked-by-your-organization): ポリシーがユーザーをブロックするときにユーザーが見るメッセージ
