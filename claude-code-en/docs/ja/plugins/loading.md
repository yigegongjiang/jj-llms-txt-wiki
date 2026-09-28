> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグイン読み込みリファレンス

> Claude Code がどのプラグインをどこから読み込むか、どの設定ファイルが読み込みを決定するか、なぜアップデートが反映されなかったのかを追跡します。

プラグインが読み込まれなかった場合、予期していたのと異なるコピーが読み込まれた場合、またはアップデートが反映されなかった場合に、どのソース、設定スコープ、またはディスク上のファイルがそれを決定したのかを確認したいときに、このページを使用してください。セッションが開始されるたびに、および `/reload-plugins` を実行するたびに Claude Code が適用するルールを示します。Claude にこのページを読んでセットアップを診断するよう依頼することもできます。

<Note>
  これらのケースは他のページで説明されています：

  * **インストール、有効化、無効化、およびアップデートの手順**: [プラグインのインストールと管理](/docs/ja/plugins/install)を参照してください
  * **特定のエラーメッセージがある場合**: [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)を参照してください
</Note>

インストール済みプラグインが通過する 3 つのステージについては [プラグインが到達したステージを確認](#check-which-stage-a-plugin-reached)から始めるか、表示されている内容に一致するセクションに移動してください：

* オフにしたプラグインがまだ読み込まれている場合: [プラグインが有効になっている場所を見つける](#find-where-a-plugin-is-enabled)
* アップデートが反映されなかった場合: [バージョンとアップデート](#versions-and-updates)
* `~/.claude/plugins/` の下のファイルを確認している場合: [ディスク上のプラグインを見つける](#find-plugins-on-disk)
* `--plugin-dir` プラグインが読み込まれなかった場合、または同じ名前のプラグインが代わりに読み込まれた場合: [名前の競合](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  プラグインが到達したステージを確認
</h2>

`enabledPlugins` エントリは、複数のステージを通じてプラグインになります：設定がそれを宣言し、Claude Code がそれをディスクにフェッチし、実行中のセッションがそれを読み込みます。プラグインが設定ファイルが示唆する動作をしない場合、どのステージに到達したかを確認してください：

* **宣言済み、設定内**: `enabledPlugins` はどのプラグインがオンになるべきかを示し、`extraKnownMarketplaces` はどのマーケットプレイスが存在すべきかを示します。`claude plugin marketplace add` を実行すると、Claude Code はマーケットプレイスをユーザー設定の `extraKnownMarketplaces` とディスクの両方に書き込みます
* **フェッチ済み、`~/.claude/plugins/` の下のディスク上**: Claude Code がフェッチしたものの記録、およびフェッチされたファイル自体：
  * `known_marketplaces.json` は Claude Code がフェッチした各マーケットプレイスを、その `source`、`installLocation`、`lastUpdated`、および `autoUpdate` とともに記録します。ユーザーごとに 1 つの `known_marketplaces.json` があるため、1 つのプロジェクトで追加したマーケットプレイスはすべてのプロジェクトで利用可能です
  * `installed_plugins.json` は各インストールをその `scope`、`installPath`、および `version` とともに記録します
  * `cache/` はプラグインファイルを保持します
* **読み込み済み、実行中のセッション内**: Claude Code がスタートアップまたは最後の `/reload-plugins` で読み込んだプラグインセット。設定またはディスクへの変更は、`/reload-plugins` を実行するか新しいセッションを開始するまで、このレイヤーに到達しません。これが `claude plugin update` が `Restart to apply changes.` で終わり、バックグラウンドアップデートが `Run /reload-plugins to apply` でプロンプトを表示する理由です

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  セッション開始時にディスク上にないプラグインとマーケットプレイス
</h3>

プラグインはセッション開始時に `installed_plugins.json` とキャッシュから、ネットワークを使用せずに読み込まれます。セッション開始後、Claude Code はバックグラウンドで宣言されたマーケットプレイスをチェックします：

* **設定が宣言しているが `known_marketplaces.json` に欠けているマーケットプレイス**: Claude Code はそれをクローンし、プラグインを再度読み込み、キャッシュされていない有効なプラグインをダウンロードします
* **宣言されたマーケットプレイスのソースが設定で変更された場合**: Claude Code は新しいソースから再度フェッチし、`Plugins changed. Run /reload-plugins to activate.` を表示します

どちらのパスもフェッチしておらず、使用可能なキャッシュディレクトリがない有効なプラグインは、`/plugin` **Errors** タブに `Plugin "<name>" not cached at <path>` を表示し、`claude plugin list` は同じ行に `— run /plugin to refresh` を追加します。修正については、[`Plugin "<name>" not cached at <path>`](/docs/ja/plugins/troubleshooting#plugin-not-cached-at)を参照してください。

<h2 id="find-where-a-plugin-came-from">
  プラグインがどこから来たかを見つける
</h2>

すべてのプラグインは `<name>@<origin>` の形式の ID を持ち、これは設定ファイルと `claude plugin list --json` で表示されるものです。`@` の後の部分は、Claude Code がプラグインを見つけた場所を示します：

| ID の末尾           | プラグインがそこに到達した方法                                                                                                                                                         | オンまたはオフにする方法                                                                                                         |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| `@<marketplace>` | 追加したマーケットプレイスからインストールしました                                                                                                                                               | 設定ファイルの `enabledPlugins` の下で `"<name>@<marketplace>": true` または `false`                                              |
| `@inline`        | `--plugin-dir` または `--plugin-url` で Claude Code を開始したか、[`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ja/env-vars#variables) を設定したか、Agent SDK アプリが `plugins` オプションを渡しました。そのセッションのみ読み込まれます | マニフェストが `defaultEnabled: false` を設定するか、設定ファイルが `"<name>@inline": false` を設定しない限り、オンです                                |
| `@skills-dir`    | `~/.claude/skills/` またはプロジェクトの `.claude/skills/` の下に `.claude-plugin/plugin.json` を持つプラグインディレクトリを保存しました                                                                 | マニフェストの `defaultEnabled`、設定ファイルが `"<name>@skills-dir"` を `true` または `false` に設定しない限り                                 |
| `@synced`        | あなたまたはあなたの組織が claude.ai アカウントでそれをオンにし、Claude Code が[それをダウンロード](#synced-plugins)しました                                                                                     | マニフェストが `defaultEnabled: false` を設定するか、設定ファイルが `"<name>@synced": false` を設定しない限り、オンです。組織が必須としてマークしたプラグインは関係なく読み込まれます |

マーケットプレイスプラグインの場合、`<name>` は `marketplace.json` のエントリ名です。`@inline` と `@skills-dir` の場合、プラグインのマニフェストの `name` です。

このテーブルのオリジン名は予約されているため、マーケットプレイスは `inline`、`skills-dir`、または `synced` という名前にすることはできません。

<h3 id="entry-name-and-manifest-name">
  エントリ名とマニフェスト名
</h3>

マーケットプレイスプラグインには 2 つの名前があり、異なる場合があります：

* **`marketplace.json` のエントリ名**: インストールおよび有効化キー。`enabledPlugins` に書き込むもの、キャッシュディレクトリの名前、および `claude plugin list` が表示するものです
* **マニフェストの `name`**: プラグインのコンポーネントが名前空間化される対象、および [名前の競合](#name-conflicts)が比較するもの

<h3 id="plugins-shared-through-a-repository">
  リポジトリを通じて共有されるプラグイン
</h3>

リポジトリを通じてプラグインを共有するには、`.claude/settings.json` の `enabledPlugins` の下にリストするか、`.claude/skills/` の下に配置します。Claude Code はプロジェクトの `.claude/plugins/` ディレクトリをスキャンしません。

クラウドセッションは、リポジトリが [`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) の下にリストするマーケットプレイスを追加しません。これはワークスペーストラストダイアログが必要であり、クラウドセッションはそれを表示しないためです。

プロジェクトスコープのスキルディレクトリプラグインは、セッションの[プライマリワーキングディレクトリ](/docs/ja/permissions#working-directories)の `.claude/skills/` からのみ読み込まれ、そのフォルダの[ワークスペーストラストダイアログ](/docs/ja/permissions#what-runs-before-you-trust-a-folder)を受け入れた後のみです。プレーンスキルとコマンドが行うように、[リポジトリルートまでの親ディレクトリを検索](/docs/ja/skills#discovery-from-parent-and-nested-directories)しません。サブディレクトリから起動した場合、リポジトリルートのプラグインは読み込まれません。代わりにリポジトリルートから起動するか、[v2.1.246 以降で `/cd` でセッションをそこに移動](/docs/ja/permissions#move-the-session-to-another-directory)してください。

プロジェクトスコープのプラグインはリポジトリにチェックインされ、それをクローンするすべての協力者に到達します。そのコンテンツはあなたではなくリポジトリから来るため、`.claude/settings.json` のプロジェクト許可ルールに適用されるのと同じトラストチェックの後にのみ読み込まれます。親フォルダを信頼するか `-p` で実行することは十分ではありません。コードを実行するコンポーネントはさらに制限されます：

* 宣言する MCP サーバーは、プロジェクト `.mcp.json` と同じ[サーバーごとの承認](/docs/ja/mcp)を通過します
* [MCP バンドル](/docs/ja/plugins/manifest-reference#mcpservers)として、`.mcpb` または `.dxt` ファイル、またはプラグインディレクトリ外のファイルから宣言する MCP サーバーはスキップされます。インラインで宣言するか、プラグインディレクトリ内の `.mcp.json` で宣言してください
* [バックグラウンドモニター](/docs/ja/plugins/components#monitors)は読み込まれません

個人スコープのプラグインにはこれらの制限はありません。

`--plugin-dir` とスキルディレクトリプラグインの書き方については、[プラグインの作成](/docs/ja/plugins/create)を参照してください。

<h3 id="synced-plugins">
  claude.ai から同期されたプラグイン
</h3>

claude.ai アカウントでオンにしたプラグインは Claude Code でも読み込まれ、マーケットプレイスからインストールしたプラグインと並んで読み込まれます。これには組織がメンバーのためにオンにしたプラグインが含まれます。これらの各プラグインは `<name>@synced` として読み込まれ、マーケットプレイスも[インストール記録](#check-which-stage-a-plugin-reached)もありません。

ターミナルセッションでは、同期されたプラグインのスキル、エージェント、hooks、MCP サーバー、および LSP サーバーはすべて読み込まれ、インストールしたマーケットプレイスプラグインと同じトラストを持ちます。

Cowork が読み込むコンポーネントについては、claude.com の [claude.ai と Cowork のプラグイン](https://claude.com/docs/plugins/overview)を参照してください。

同期されたプラグインは Cowork セッションと、claude.ai アカウントでサインインするターミナルセッションで読み込まれます：

* **[Cowork](https://claude.com/product/cowork)**: Claude Code はセッション開始時にセッション独自の環境にダウンロードします
* **ターミナルセッション**: Claude Code を開始するたびに、バックグラウンドで 1 回同期され、新しいプラグインと更新されたプラグインをダウンロードし、あなたまたは組織がオフにしたプラグインを削除します。ターミナルセッションでの同期には Claude Code v2.1.273 以降が必要です

<h4 id="sync-timing-in-terminal-sessions">
  ターミナルセッションでの同期タイミング
</h4>

ターミナル同期はバックグラウンドで実行されるため、セッション開始後に完了する可能性があります。インタラクティブセッションで同期されたプラグインを追加、更新、または削除すると、`Plugins changed. Run /reload-plugins to activate.` が表示されます。`/reload-plugins` を実行してそのセッションで変更を読み込むか、次に Claude Code を開始するまで待ってください。

claude.ai でセッション実行中にプラグインを有効にした場合、プラグインは次に Claude Code を開始するときにダウンロードされます。

<h4 id="sign-in-requirements-for-terminal-sync">
  ターミナル同期のサインイン要件
</h4>

ターミナルでは、claude.ai アカウントでサインインするセッションでのみプラグインが同期されます。

Claude Code の以前のバージョンでサインインした場合、そのサインインはバックグラウンドで Claude Code がそれを更新するまでプラグインをカバーしません。より早くアクセスするには、`/login` を再度実行してください。プラグイン同期は次に Claude Code を開始するときに開始されます。

<h4 id="control-which-synced-plugins-load">
  同期されたプラグインの読み込みを制御
</h4>

同期されたプラグインを 1 つずつオフにすることができます。ただし、組織が必須とするプラグインは除きます。または、マシン上のすべての同期されたプラグインをオフにします：

* **1 つのプラグイン**: シェルで `claude plugin disable <name>@synced` を実行し、セッションの `/plugin` **Installed** タブの両方が、ユーザーレベルの [`enabledPlugins`](/docs/ja/settings-reference#enabledplugins) に `"<name>@synced": false` を保存します。すべての環境でプロジェクトからプラグインを除外するには、プロジェクトのコミットされた `.claude/settings.json` で同じキーを設定します
* **マシン上のすべての同期されたプラグイン**: ユーザー設定で [`syncClaudeAiPlugins`](/docs/ja/settings-reference#syncclaudeaiplugins) を `false` に設定するか、組織が[マネージド設定](/docs/ja/managed-settings)で設定します。Claude Code はダウンロードを停止し、次に起動するときに、既に同期したプラグインを `~/.claude/plugins/.trash/` に移動し、それ以上読み込みません。組織が claude.ai でスキルをオフにした場合、プラグインも同期を停止します
* **組織が必須とするプラグイン**: 組織が claude.ai で必須としてマークしたプラグインは、以前に無効にした場合でも読み込まれます。`claude plugin disable` は `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.` で拒否し、`claude plugin list` は `required by your org` としてマークします

claude.ai でプラグインを削除する方法については、[インストール済みプラグインの管理](/docs/ja/plugins/install#manage-installed-plugins)を参照してください。

<h2 id="find-where-a-plugin-is-enabled">
  プラグインが有効になっている場所を見つける
</h2>

6 つのソースのいずれかで `enabledPlugins` エントリを設定できます。テーブルは最も低い優先度から最も高い優先度にリストし、各テーブルが誰に適用されるかを示します。設定ファイル自体については、[設定ファイルとそれらが影響する人](/docs/ja/settings#where-settings-live)を参照してください。

| ソース         | 設定する場所                                                                           | 到達範囲                                                                    |
| :---------- | :------------------------------------------------------------------------------- | :---------------------------------------------------------------------- |
| `--add-dir` | `--add-dir` で渡すディレクトリの `.claude/settings.json` または `.claude/settings.local.json` | このセッションのみ。`true` 値のみが効果を持ち、他のすべてのソースがそれをオーバーライドします                      |
| `user`      | `~/.claude/settings.json`                                                        | あなた、すべてのプロジェクトで                                                         |
| `project`   | `.claude/settings.json`                                                          | リポジトリをクローンするすべての人                                                       |
| `local`     | `.claude/settings.local.json`                                                    | あなた、このリポジトリのみ                                                           |
| `flag`      | 起動時に渡す `--settings` 値                                                            | このセッションのみ                                                               |
| `managed`   | [マネージド設定](/docs/ja/managed-settings)                                                  | ポリシーがカバーするすべてのユーザー。`true` は強制的に有効にし、`false` はブロックし、他のソースはそれをオーバーライドしません |

これらのソースはキーごとにマージされます。各プラグイン ID について、適用される値は ID を言及する最も高い優先度のソースからの値です。ID を言及しないソースは、低い優先度のソースからの値を有効なままにします。

<h3 id="disabled-in-user-settings-but-still-loads">
  ユーザー設定で無効化されているが、まだ読み込まれている
</h3>

`~/.claude/settings.json` でプラグインを `false` に設定し、それでも読み込まれる場合、より高い優先度のソースの `true` がそれをオーバーライドしています。`claude plugin list` と `/plugin` のプラグインの行は `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting` を表示します。メッセージはあなたをオーバーライドしたソースを名前付けします：`project`、`project, gitignored` は `.claude/settings.local.json`、`cli flag`、または `managed` です。

プロジェクトで有効なプラグインをマシンでオプトアウトするには、ID を `.claude/settings.local.json` で `false` に設定します。これはプロジェクトファイルより高い優先度を持ちます。

<h3 id="enabled-in-project-settings-but-not-installed">
  プロジェクト設定で有効化されているが、インストールされていない
</h3>

プラグインの唯一の `true` がプロジェクトの `.claude/settings.json` にある場合、Claude Code はそのマーケットプレイスエントリが[相対パスソース](/docs/ja/plugins/marketplace-reference#plugin-sources)を持つか、[シードディレクトリ](/docs/ja/plugins/org#seed-containers-and-ci)がそれを既に保持していない限り、インストールされていないマシンにそれをフェッチしません。代わりに、`/plugin` **Errors** タブは `Plugin "<name>" is enabled in project settings but isn't installed here` を表示します。

相対パスプラグインはインストール記録を必要としません。マーケットプレイス自体から読み込まれるためです。

Claude Code は、これらのソースのいずれかがそれを `true` に設定した場合にのみ、外部ソースを持つプラグインをフェッチします：

* ユーザー設定
* git が追跡しない `.claude/settings.local.json`
* `--settings` フラグ
* マネージド設定

<h2 id="find-plugins-on-disk">
  ディスク上のプラグインを見つける
</h2>

Claude Code はプラグインファイルと状態記録を 1 つのプラグインルートの下に保持します。これは `~/.claude/plugins` です。ただし、[`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/ja/env-vars)を設定した場合を除きます。テーブルのすべてのパスはそのルートに相対的です。

| パス                                                   | 保持するもの                                                                                                                                                                                                                                                                                         |
| :--------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache/<marketplace>/<plugin>/<version>/`            | マーケットプレイスプラグインのインストール済みバージョンごとに 1 つのディレクトリ。`<plugin>` はマーケットプレイスエントリ名で、`<version>` は[解決されたバージョン](#versions-and-updates)です。`${CLAUDE_PLUGIN_ROOT}` はこのディレクトリを指します                                                                                                                               |
| `data/<plugin-id>/`                                  | プラグインの永続ディレクトリ。`${CLAUDE_PLUGIN_DATA}` として公開されます。`<plugin-id>` がどのように形成されるかについては、[パス変数と永続データ](/docs/ja/plugins/components#path-variables-and-persistent-data)を参照してください。Claude Code はプラグインコンポーネントが最初に使用するときに作成し、アップデート全体で保持します。Claude Code は `--keep-data` を渡さない限り、最後のスコープからプラグインをアンインストールするときに削除します |
| `marketplaces/<name>/`                               | GitHub、別の Git ホスト、または URL から追加されたマーケットプレイスのクローンまたはダウンロード。ローカル `file` または `directory` ソースから追加されたマーケットプレイスはここにコピーがなく、その `installLocation` は `known_marketplaces.json` で提供したパスです                                                                                                                  |
| `synced/`                                            | Claude Code が[claude.ai アカウントから同期](#synced-plugins)したプラグイン                                                                                                                                                                                                                                     |
| `.trash/`                                            | claude.ai 同期が削除したプラグイン。例えば、claude.ai でプラグインをオフにした後、または同期を停止した後                                                                                                                                                                                                                                 |
| `installed_plugins.json` と `known_marketplaces.json` | Claude Code がインストールしたものと、フェッチしたマーケットプレイスの記録。[Check which stage a plugin reached](#check-which-stage-a-plugin-reached) の下で説明されています。[claude.ai でホストされているマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)は代わりに `known_marketplaces_claudeai.json` に記録されます                                       |
| `flagged-plugins.json`                               | Claude Code がマーケットプレイスがそれらをリストから削除したため、アンインストールしたプラグイン。`/plugin` の **Flagged** セクションに表示されます。[マーケットプレイスのホスト](/docs/ja/plugins/host-marketplace)を参照してください                                                                                                                                             |

`${CLAUDE_PLUGIN_ROOT}` はバージョンディレクトリを指すため、プラグインのルートパスはすべてのバージョンで変更されます。プラグインの耐久ファイルを `${CLAUDE_PLUGIN_DATA}` に保持してください。

<h3 id="in-place-and-copied-plugins">
  インプレイスおよびコピーされたプラグイン
</h3>

Claude Code は、オリジンに従って、いくつかのプラグインをそれらを保持する場所からインプレイスで読み込み、残りをキャッシュにコピーします：

* **`--plugin-dir` とスキルディレクトリプラグイン**: ディレクトリはインプレイスで読み込まれ、決してコピーされません。`--plugin-url` アーカイブまたは `--plugin-dir` `.zip` は最初にセッション一時ディレクトリに抽出されます
* **ローカルディレクトリから追加したマーケットプレイスの相対パスプラグイン**: プラグインはマーケットプレイスフォルダ内のパスからインプレイスで読み込まれます。ソースディレクトリへの編集は次のセッション開始または `/reload-plugins` で有効になり、バージョンを増やす必要はありません。プラグインの hook プロセスと MCP および LSP サーバーは、ソースディレクトリを指す `CLAUDE_PLUGIN_ROOT` を受け取ります。Node.js パッケージ依存関係については、[依存関係インストールが実行される場合](#when-the-dependency-install-runs)を参照してください
* **[リンクモード](/docs/ja/plugins/marketplace-reference#command-plugin-source)の `command` ソースプラグイン**: コマンドが出力したディレクトリはキャッシュエントリ内のリンクを通じてインプレイスで読み込まれます
* **他のすべてのマーケットプレイスプラグイン**: Claude Code はプラグインを `cache/<marketplace>/<plugin>/<version>/` にコピーし、そのコピーから読み込みます。プラグインディレクトリ外のファイルはコピーされないため、コピーされたプラグイン内のスクリプトが `../shared` などのプラグインルート上のパスを読む場合、それらは見つかりません

<h3 id="paths-that-escape-the-plugin-directory">
  プラグインディレクトリを超えるパス
</h3>

プラグインがインプレイスで読み込まれるか、キャッシュコピーから読み込まれるかに関わらず、Claude Code はそれが独自のディレクトリ外のコンポーネントを宣言することを許可しません。プラグインルート外に解決されるコンポーネントパスを拒否します。パスが `plugin.json` またはマーケットプレイスエントリで宣言されているかどうかに関わらず：

* **書き込まれたときにプラグインの外を指すパス**。例えば `../shared-utils`
* \*\*プラグイン内のリンク間を除く、プラグインの外につながるシンボリックリンク]\(/ja/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)
* **macOS と Linux では、パスのどこかにバックスラッシュを含むパス**。プラグイン内にとどまる場合でも。バックスラッシュパスで宣言されたコンポーネントは Windows でのみ読み込まれるため、`./commands/deploy.md` などの前方スラッシュを使用してコンポーネントパスを書き込んでください

拒否されたパスは [`path escapes plugin directory`](/docs/ja/errors#path-escapes-plugin-directory) エラーとして表示され、プラグインはそのコンポーネントなしで読み込まれます。

<h3 id="cleanup-of-previous-versions">
  以前のバージョンのクリーンアップ
</h3>

プラグインを更新またはアンインストールすると、Claude Code は前のバージョンディレクトリに `.orphaned_at` マーカーを書き込みます。14 日後のバックグラウンドクリーンアップでそのディレクトリを削除するため、既に古いバージョンを読み込んだセッションは実行を続けます。

スイープは `installed_plugins.json` が少なくとも 1 つのインストールを記録している間のみ実行されます。最後のプラグインをアンインストールした後、孤立したディレクトリは別のプラグインをインストールするまで残ります。

<h3 id="node-js-package-dependencies">
  Node.js パッケージ依存関係
</h3>

Claude Code がプラグインをキャッシュにコピーするとき、プラグインの Node.js パッケージ依存関係もそこにインストールするため、プラグインの hooks と MCP サーバーはそれらを読み込むことができます。

このセクションは、プラグインが独自の `package.json` で宣言する npm および Bun パッケージをカバーしています。他のプラグインに依存するプラグインについては、[プラグイン依存関係バージョン](/docs/ja/plugins/dependencies)を参照してください。

<h4 id="when-the-dependency-install-runs">
  依存関係インストールが実行される場合
</h4>

Claude Code は、それが作成するたびにコピーされたバージョンディレクトリ内でインストールを実行します：

* プラグインをインストールするとき
* Claude Code がプラグインを新しいバージョンに更新するとき
* セッション開始時に有効なプラグインがキャッシュされていない場合。例えば、新しいマシン上

ローカルディレクトリマーケットプレイスから[インプレイスで読み込まれた](#in-place-and-copied-plugins)相対パスプラグインの場合、Claude Code はソースディレクトリに依存関係をインストールしません。そこに自分でインストールするか、hook から [`${CLAUDE_PLUGIN_DATA}`](/docs/ja/plugins/components#path-variables-and-persistent-data) にインストールしてください。

インストールは、プラグインのルートディレクトリに `package.json` とサポートされているロックファイルの両方が含まれている場合にのみ実行されます。ロックファイルは Claude Code が実行するコマンドを決定します：

| ロックファイル                                       | コマンド                                             |
| :-------------------------------------------- | :----------------------------------------------- |
| `bun.lock` または `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` または `package-lock.json` | `npm ci --ignore-scripts`                        |

プラグインにこれらのロックファイルの複数が含まれている場合、Claude Code は最初のマッチを使用し、順序をチェックします：`bun.lock`、`bun.lockb`、`npm-shrinkwrap.json`、`package-lock.json`。

Claude Code は Yarn および pnpm ロックファイルと Bun ロックファイルの横にある `bunfig.toml` のインストールをスキップします：

* プラグインに `yarn.lock` または `pnpm-lock.yaml` のみがある場合、npm ロックファイルに置き換えてください
* `bunfig.toml` が Bun ロックファイルと同じディレクトリにある場合、`bunfig.toml` を削除するか、Bun ロックファイルを npm ロックファイルに置き換えてください

npm ロックファイルを含めて、最も多くのユーザーに到達してください。Claude Code はマッチされたロックファイルのパッケージマネージャーをユーザーの PATH から実行し、そのパッケージマネージャーが見つからない場合は他のロックファイルを試しません。

npm ソースを通じて配布されるプラグインの場合、`npm-shrinkwrap.json` を使用してください。npm は公開されたパッケージから `package-lock.json` を除外するためです。

<h4 id="limits-on-the-dependency-install">
  依存関係インストールの制限
</h4>

Claude Code はこの依存関係インストールを制約するため、プラグインまたはそのパッケージからのコードはそれ中に実行されず、実行時間が制限されます：

* **フローズン解決**: Bun と npm はロックファイルがピンしたものを正確にインストールし、`package.json` とロックファイルが不一致の場合、バージョンを再解決するのではなく失敗します
* **ライフサイクルスクリプトなし**: `--ignore-scripts` は `preinstall`、`install`、および `postinstall` スクリプトが実行されるのを防ぎ、ネイティブモジュールをビルドする依存関係はこのインストール中にダウンロードされますがコンパイルされません
* **60 秒のタイムアウト**: Claude Code は実行時間が長いインストールを停止し、失敗として扱います

Claude Code は npm ソースプラグインをこの依存関係インストールの前にフェッチし、パッケージ独自のインストールスクリプトはフェッチ中に実行されません。[npm プラグインソース](/docs/ja/plugins/marketplace-reference#npm-plugin-source)を参照してください。

自動インストールをオフにすることはできません。設定または環境変数はそれを無効にしません。

制限されたネットワークでは、[ネットワークアクセス要件](/docs/ja/network-config#network-access-requirements)を許可するホストを参照してください。

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  依存関係インストールが失敗またはスキップされた場合
</h4>

失敗またはスキップされたインストールはプラグインをブロックしません。各ケースは異なる兆候を残します：

* 失敗したインストール、または Yarn もしくは pnpm ロックファイルまたは `bunfig.toml` のためにスキップされたインストールは、`claude --debug` 出力に警告として表示されます
* `package.json` を持つが、ロックファイルがないプラグインはログエントリなしでスキップされます
* タイムアウトしたインストールは、キャッシュされたコピーに部分的な `node_modules` ツリーを残す可能性があります

自動インストールが依存関係を提供できない場合、hook から[永続データディレクトリ](/docs/ja/plugins/components#path-variables-and-persistent-data)にインストールしてください。これには、ライフサイクルスクリプトをビルドする必要があるパッケージ、Python 依存関係、および Yarn または pnpm でロックされたプラグインが含まれます。

<h2 id="versions-and-updates">
  バージョンとアップデート
</h2>

プラグインの作成者が新しいコミットをプッシュし、`claude plugin update` が `<name> is already at the latest version (<version>).` を出力する場合、Claude Code が計算するプラグインのバージョンは変更されないため、ディスク上で何も変更されません。

Claude Code はインストールするすべてのプラグインのバージョンを計算し、そのバージョンはアップデートを検出する方法です。`claude plugin update` とバックグラウンド自動アップデートはバージョンを再度計算し、`installed_plugins.json` が記録するものと一致する場合、プラグインをスキップします。

バージョンはプラグインのキャッシュディレクトリにも名前を付けます。

`"version"` をピンするマニフェストは、計算されたバージョンがコミット全体で同じままである 1 つの方法です。[Claude Code がバージョンを計算する方法](#how-claude-code-computes-the-version)を参照してください。

ローカルディレクトリマーケットプレイスから[インプレイスで読み込まれた](#in-place-and-copied-plugins)プラグインは、バージョン文字列が何を言おうとも、すべてのセッション開始で現在のソースファイルを読み込みます。[claude.ai でホストされているマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)のプラグインの場合、claude.ai がプラグインのために記録するバージョンはそのバージョンであり、マニフェストの `version` は読み込まれません。

<h3 id="how-claude-code-computes-the-version">
  Claude Code がバージョンを計算する方法
</h3>

追加したソースのマーケットプレイスの場合、Claude Code はプラグインのマーケットプレイスエントリの `source` タイプによってルールを選択します。[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference#plugin-sources)はソースタイプをリストします。そのリストのすべてのソースタイプについて、`command` を除く：

1. プラグインのマニフェストの `version` フィールドが最初に来ます
2. その後、プラグインのマーケットプレイスエントリの `version` フィールド
3. どちらも設定されていない場合、バージョンはソースタイプから来ます：

| ソースタイプ                                              | `version` フィールドが設定されていない場合のバージョン                                                       |
| :-------------------------------------------------- | :------------------------------------------------------------------------------------- |
| `github`、`url`、または `git-subdir`                     | ソースのコミット SHA。12 文字に短縮されます。`git-subdir` バージョンはサブディレクトリパスのハッシュも含みます                      |
| `archive`                                           | SHA-256 ダイジェスト。12 文字に短縮されます：マーケットプレイスエントリの `sha256` ピン、またはピンがない場合はダウンロードされたファイルのダイジェスト |
| Git ホストマーケットプレイス内の相対パス                              | インストール済みディレクトリのコミット SHA                                                                |
| ローカルディレクトリ。プラグインディレクトリもそのマーケットプレイスも git リポジトリではない場合 | `unknown`                                                                              |
| `npm`                                               | `unknown`                                                                              |

Claude Code は、`~/.claude` を管理する git などの、インストールパスを囲むリポジトリからバージョンを取得しません。

`command` ソースの場合、Claude Code は常にコマンドが生成したものからバージョンを導出します：単独で 12 文字のハッシュ、またはマニフェストが 1 つを設定する場合は `<manifest version>-<hash>`。マーケットプレイスエントリの `version` はコマンドソースでは無視されます。ハッシュがカバーするものについては、[コピーモードとリンクモード](/docs/ja/plugins/marketplace-reference#copy-mode-and-link-mode)を参照してください。

マニフェストが最初に来るため、`"version": "1.0.0"` をピンするマニフェストは、作成者がプッシュするコミット数に関わらず、文字列を変更するまで、すべてのユーザーをキャッシュされたコピーに保持します。ユーザーがコミットを追跡できるようにするには、マニフェストとエントリの両方から `version` を除外してください。[マーケットプレイスのホスト](/docs/ja/plugins/host-marketplace)は、どの選択がどのリリースセットアップに適合するかをカバーしています。

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Claude Code がインストール前にマーケットプレイスをリフレッシュする場合
</h3>

プラグインをインストールするとき、Claude Code はローカルコピーのマーケットプレイスカタログでそれを検索します。セッションで `/plugin install` を実行するか、シェルで `claude plugin install` を実行し、プラグインをマーケットプレイスの有無で名前付けることができます。テーブルはこれらの組み合わせのどれがローカルコピーをリフレッシュするかを示します。

| プラグイン名             | コマンド                                          | Claude Code がリフレッシュするもの                |
| :----------------- | :-------------------------------------------- | :------------------------------------- |
| `name@marketplace` | `/plugin install` または `claude plugin install` | ルックアップの前に、名前付きマーケットプレイス                |
| `name` のみ          | `/plugin install`                             | 自動アップデートがオンのマーケットプレイスのみ、ルックアップが失敗した後のみ |
| `name` のみ          | `claude plugin install`                       | なし。キャッシュされたカタログをリフレッシュなしで読み込みます        |

`name@marketplace` インストール前のリフレッシュはマーケットプレイスの自動アップデート設定または `DISABLE_AUTOUPDATER` に依存しません。

リフレッシュが失敗した場合、インストールはキャッシュされたカタログから進行し、`claude plugin install` は `marketplace not refreshed` を報告します。

Claude Code は以下の場合、`name@marketplace` インストール前のリフレッシュをスキップします：

* マーケットプレイスがローカル `file` または `directory` ソースから追加されたか、[`settings` ソース](/docs/ja/settings-reference#extraknownmarketplaces)を持つ設定でインラインで定義されている
* [シードディレクトリ](/docs/ja/env-vars)がマーケットプレイスを供給している
* Claude Code が過去 30 秒以内にマーケットプレイスをリフレッシュした
* `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` を設定した
* [マネージド設定](/docs/ja/plugins/org#restrict-what-users-can-install)がマーケットプレイスをブロックしている。この場合、Claude Code もインストールを拒否します

<h3 id="when-auto-update-runs">
  自動アップデートが実行される場合
</h3>

インタラクティブセッションでは、最初のメッセージを送信した後、Claude Code は最大 10 分のランダムな遅延を待ちます。その後、自動アップデートがオンのすべてのマーケットプレイスをリフレッシュし、ディスク上のそれらからインストールされたプラグインを更新します。

実行中のセッションは読み込んだバージョンを保持し、`Plugin updated: <name> · Run /reload-plugins to apply` が表示されます。リロードするかどうかに関わらず、新しいバージョンは次の起動時に読み込まれます。

<h4 id="which-marketplaces-and-plugins-auto-update">
  どのマーケットプレイスとプラグインが自動アップデートするか
</h4>

マーケットプレイスが自動アップデートするかどうかは、設定されている最初のものに従います：

1. **設定ファイルの `extraKnownMarketplaces` エントリの `autoUpdate`**
2. **`known_marketplaces.json` エントリの `autoUpdate`**。これは `/plugin` **Marketplaces** の下の **Enable auto-update** トグルが書き込みます。設定ファイルが `extraKnownMarketplaces` の下でマーケットプレイスも宣言する場合、トグルはその設定エントリにも `autoUpdate` を書き込みます
3. **デフォルト**: `claude-plugins-official` などの Anthropic の公式マーケットプレイスではオン。`knowledge-work-plugins` と `first-party-plugins` ではオフ。[claude.ai から追加されたマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)ではオン。他のすべてのマーケットプレイスではオフ

`DISABLE_UPDATES=1`、`DISABLE_AUTOUPDATER=1`、または `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` を設定した場合、全体のパスはオフであり、**Enable auto-update** トグルは非表示です。ただし、`FORCE_AUTOUPDATE_PLUGINS=1` も設定した場合を除きます。[環境変数リファレンス](/docs/ja/env-vars)は各変数のより広い効果をカバーしています。

自動アップデートはまた、マーケットプレイスエントリが `headersHelper` を宣言するプラグインもスキップします。[コマンドを要求する代わりに拒否するインストールとアップデート](/docs/ja/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking)は、そのようなプラグインが `/plugin` **Errors** タブに表示される場合と、そこからそれを更新する方法を説明しています。

コピーされたプラグインがセッション中盤で更新される場合、hook コマンド、モニター、MCP サーバー、および LSP サーバーは前のバージョンのパスを使用し続けます。`/reload-plugins` を実行して、hooks、MCP サーバー、および LSP サーバーを新しいパスに切り替えてください。モニターはセッション再起動が必要です。

<h3 id="when-a-command-source-re-runs">
  コマンドソースが再実行される場合
</h3>

`command` ソースを持つプラグインは、[自動アップデートパス](#when-auto-update-runs)を待ちません。出力されたディレクトリはコマンドが実行された時点でのツールの状態を反映するため、Claude Code は[受け入れたコマンド](/docs/ja/plugins/host-marketplace#change-the-command-of-a-command-source)を次の時点で再度実行します：

* プラグインをインストールまたは更新するたびに
* セッションごとに 1 回、有効なコマンドソースプラグインごとに、セッション開始直後のバックグラウンドで。この実行はマーケットプレイスの自動アップデート設定または `DISABLE_AUTOUPDATER` に依存しません
* スタートアップまたは `/reload-plugins` で、有効なプラグインのインストール済みバージョンがプラグインキャッシュから欠けている場合

[`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ja/env-vars)を設定した場合、Claude Code は 2 つのバックグラウンド実行をスキップします。明示的なインストールとアップデートはその変数セットでコマンドを実行します。

コマンドのハッシュされた出力が変更された場合、Claude Code は結果を新しいバージョンとしてインストールし、実行中のインタラクティブセッションで再度読み込み、[`/reload-plugins` が切り替える同じコンポーネント](/docs/ja/plugins/cli-reference#reload-plugins)を切り替えます。プラグインが再度読み込まれたという通知が表示されます。

インプレイスで再度読み込むことがセッションのプロンプトキャッシュを無効にする場合、Claude Code は代わりに `/reload-plugins` を実行するようプロンプトを表示します。これは[キャッシュコストについて警告し、`--force` で再実行するときに適用](/docs/ja/prompt-caching#enabling-or-disabling-a-plugin)されます。

<h2 id="name-conflicts">
  名前の競合
</h2>

異なるオリジンから有効なプラグインがマニフェスト名を共有する場合、このオーダーは最も高い優先度から最も低い優先度へ、どれが読み込まれるかを決定します：

1. ID がマネージド設定 `enabledPlugins` に表示されるプラグイン。`true` または `false` として。マニフェスト名が ID の名前部分と一致する `--plugin-dir` コピーは読み込まれず、`--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings` が表示されます
2. 有効な `--plugin-dir`、`--plugin-url`、または `CLAUDE_CODE_PLUGIN_DIRS` プラグイン。同じ名前のインストール済みマーケットプレイスプラグインまたはスキルディレクトリプラグインを置き換えます：
   * **インストール済みマーケットプレイスプラグイン**: サイレントに置き換えられます。`claude plugin list` はマーケットプレイス行を有効として表示し続けます。その行は設定を反映するためです。`--debug` で開始するときに Claude Code が `~/.claude/debug/` の下に書き込むログのみが `Plugin "<name>" from --plugin-dir overrides installed version` を記録します
   * **スキルディレクトリプラグイン**: `/plugin` **Errors** タブ行で置き換えられます。`Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence` と読みます
3. インストール済みマーケットプレイスプラグイン。同じ名前のスキルディレクトリプラグインは同じ `Not loaded` 行を取得し、インストール済みプラグインを名前付けます
4. スキルディレクトリプラグイン。これら 2 つの間で、`~/.claude/skills/` の下のコピーが読み込まれ、プロジェクトの `.claude/skills/` コピーは削除されます。どのパスがそれをシャドウしたかを示す行があります
5. [claude.ai から同期](#synced-plugins)されたプラグイン。他のオリジンから有効なプラグインが名前と一致する場合、Claude Code はそのプラグインを読み込み、同期されたコピーを読み込まれていないと報告します。claude.ai コピーを代わりに使用するには、独自のコピーを無効にしてください

オーダーはマニフェスト名を比較するため、`hello-plugin` という名前の `--plugin-dir` プラグインは、そのプラグインのマニフェストが `"name": "hello-plugin"` も言う場合、`hello@example-marketplace` を置き換えます。

<h3 id="keep-a-session-only-plugin-from-loading">
  セッションのみのプラグインが読み込まれるのを防ぐ
</h3>

`--plugin-dir` プラグインが何かをシャドウするのを防ぐか、親プロセスがフラグを渡す場合にオフにするには、その ID を任意の設定ファイルで `false` に設定します。マニフェスト名が `hello-plugin` のプラグインの場合、エントリは `"enabledPlugins": {"hello-plugin@inline": false}` です。無効なセッションのみのプラグインはシャドウしないため、マーケットプレイスまたはスキルディレクトリコピーが代わりに読み込まれます。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインのインストールと管理](/docs/ja/plugins/install): インストール、有効化、無効化、およびアップデートの手順自体
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting): それらを生成するステージ別のエラーメッセージ
* [プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference): このページで名前付けされたフラグとコマンド
* [組織のプラグインを管理](/docs/ja/plugins/org): プラグインを強制的に有効にするか、ブロックするマネージド設定
