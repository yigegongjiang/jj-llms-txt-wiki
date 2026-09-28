> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# マーケットプレイスリファレンス

> marketplace.json フィールド、プラグインエントリ、プラグインおよびマーケットプレイスソースオブジェクトの完全なリファレンス。各フィールドの有効な場所を含みます。

`marketplace.json` はプラグインマーケットプレイスを定義するファイルです。マーケットプレイスの名前、所有者、およびプラグインごとに 1 つのエントリが含まれます。各エントリのプラグインソースは、Claude Code がそのプラグインをどこから取得するかを指定します。

マーケットプレイスソースは、Claude Code が `marketplace.json` ファイル自体をどこから取得するかを指定する別のオブジェクトです。設定で記述するか、`claude plugin marketplace add` を実行するときに Claude Code が構築します。

このリファレンスは、正確なフィールド名または値が必要なマーケットプレイス管理者、および [`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces)、[`strictKnownMarketplaces`](/docs/ja/settings-reference#strictknownmarketplaces)、および [`blockedMarketplaces`](/docs/ja/plugins/org#restrict-what-users-can-install) で有効な `source` 値を知る必要がある管理者向けです。

<Note>
  これらのケースは他のページで説明されています：

  * **マーケットプレイスの構築またはホスト**: [マーケットプレイスの作成](/docs/ja/plugins/create-marketplace) および [マーケットプレイスのホストと保守](/docs/ja/plugins/host-marketplace) を参照してください
  * **許可リストと拒否リストのレシピ**: [組織のプラグインを管理する](/docs/ja/plugins/org) を参照してください
</Note>

記述または読み取る内容のセクションを見つけてください：

* **マーケットプレイスファイル**: [トップレベルフィールド](#top-level-fields) および [プラグインエントリ](#plugin-entries)
* **エントリの `source`**: [プラグインソース](#plugin-sources)
* **設定の `source` オブジェクト**: [マーケットプレイスソース](#marketplace-sources)
* **[`claude plugin validate <path>`](/docs/ja/plugins/cli-reference) からの出力**: [検証メッセージ](#validation-messages)。各メッセージをそれが名前を付けるフィールドにマップします

<h2 id="marketplace-file">
  マーケットプレイスファイル
</h2>

マーケットプレイスファイルをマーケットプレイスのディレクトリの `.claude-plugin/marketplace.json` に保存します。ファイルをリポジトリ内の別の場所に保持する場合、ユーザーは [`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) でマーケットプレイスを宣言する必要があり、その source に `path` を設定する必要があります。`claude plugin marketplace add` にはそのオプションがないためです。

`.claude-plugin/` を含むディレクトリはマーケットプレイスルートと呼ばれ、すべての相対プラグインソースは `.claude-plugin/` からではなく、そこから解決されます。

各ユーザーは `name` ごとに 1 つのマーケットプレイスを登録するため、ユーザーは同じ名前の 2 つのマーケットプレイスを同時に登録することはできません。

Claude Code は不明なトップレベルキーまたはプラグインエントリキーを無視し、拒否しないため、タイプミスは静かに読み込まれます。`claude plugin validate` は各不明なキーを警告として報告します。

<h3 id="reserved-names">
  予約名
</h3>

マーケットプレイスに次の名前を付けることはできません：

* **公式マーケットプレイス名**: `claude-code-marketplace`、`claude-code-plugins`、`claude-plugins-official`、`anthropic-marketplace`、`anthropic-plugins`、`agent-skills`、`anthropic-agent-skills`、`life-sciences`、`knowledge-work-plugins`、`claude-for-legal`、`claude-for-financial-services`、`financial-services-plugins`、`first-party-plugins`、および `claude-tag-plugins`。マーケットプレイスが `github.com/anthropics/` の下の `github` または `git` [マーケットプレイスソース](#marketplace-sources) から来ない限り予約されています。
* **コミュニティマーケットプレイス名**: `claude-community`、`claude-plugins-community`、および `healthcare`。公式名と同じルールの下で予約されています。
* **プラグインディレクトリ名**: `anthropic-plugin-directory` および `claude-plugin-directory`。公式名と同じルールの下で予約されています。
* **公式マーケットプレイスになりすまし名**: `official-claude-plugins` または `claude-plugins-v2` などの名前、および非 ASCII 文字を含む任意の名前。エラーは `Marketplace name impersonates an official Anthropic/Claude marketplace` です。名前内の制御文字または双方向フォーマット文字も `Marketplace name cannot contain control or bidirectional-formatting characters` を報告します。
* <span id="reserved-name-spellings" />**予約名の別のスペル**: 予約名と末尾のドット、またはハイフンの代わりにハイフン以外の記号によってのみ異なる名前。`claude.code.plugins` は `claude-code-plugins` としてカウントされます。`claude plugin validate` はそのような名前を受け入れます。マーケットプレイスの追加は [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/ja/errors#marketplace-name-is-another-spelling-of-a-reserved-name) で失敗し、1 つの下で登録されたマーケットプレイスは読み込みを停止します。このチェックには Claude Code v2.1.280 以降が必要です。
* **Claude Code がマーケットプレイスから来ないプラグインに使用する名前**: [`--plugin-dir`](/docs/ja/cli-reference) で読み込まれたプラグインの `inline`、組み込みプラグインの `builtin`、[`.claude/skills/`](/docs/ja/skills) から自動読み込みされたプラグインの `skills-dir`、および claude.ai アカウントから同期されたプラグインの `synced`。`claude-plugin-test` も予約されています。`skills-dir` は `strictKnownMarketplaces` および `blockedMarketplaces` で `{"source": "skills-dir"}` としても表示されます。[ポリシーリストでのみ有効なソース値](#source-values-valid-only-in-policy-lists) で説明されています。
* **`npm`、`pip`、`uv`、`cargo`、`github`、および `gh`**: 任意の大文字小文字で予約されています。このチェックには Claude Code v2.1.275 以降が必要です。
* **`claudeai-` で始まる名前**: claude.ai でホストされているマーケットプレイス用に予約されています。`claude plugin marketplace add` は `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai` で他のマーケットプレイスを拒否します。

<h2 id="top-level-fields">
  トップレベルフィールド
</h2>

テーブルは Claude Code が `marketplace.json` から読み取るすべてのキーをリストします。`name`、`owner`、および `plugins` は必須です。

| フィールド                                     | 型                | 説明                                                                                                                                                                      |
| :---------------------------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                    | string           | マーケットプレイス識別子。スペース、制御文字、または双方向フォーマット文字なし。`/` または `\` なし、`..` なし、`.` ではない。[予約名](#reserved-names) を参照してください。ユーザーはプラグインをインストールするときに `@` の後に入力します                            |
| `owner`                                   | object           | 管理者情報。`name` は必須。`email` および `url` はオプション                                                                                                                               |
| `plugins`                                 | array            | [プラグインエントリ](#plugin-entries)。各エントリは独立して検証されるため、1 つの無効なエントリがマーケットプレイスを失敗させません                                                                                            |
| `$schema`                                 | string           | エディタオートコンプリート用の JSON Schema URL。読み込み時に無視されます                                                                                                                            |
| `description`                             | string           | ユーザーに表示されるマーケットプレイスの説明。`claude plugin validate` は欠落時に警告します                                                                                                              |
| `version`                                 | string           | マーケットプレイスマニフェストバージョン                                                                                                                                                    |
| `metadata.description`、`metadata.version` | string           | `description` および `version` の代替位置                                                                                                                                       |
| `metadata.pluginRoot`                     | string           | ベアプラグインソース名が解決される下のディレクトリ。[相対パスプラグインソース](#relative-path-plugin-source) を参照してください。Claude Code v2.1.239 以降が必要です                                                           |
| `forceRemoveDeletedPlugins`               | boolean          | `true` の場合、`plugins` から削除したプラグインはユーザーのマシンでアンインストールされます。[マーケットプレイスのホストと保守](/docs/ja/plugins/host-marketplace) を参照してください                                                       |
| `allowCrossMarketplaceDependenciesOn`     | array of strings | このマーケットプレイスのプラグインの依存関係としてインストールされる可能性があるマーケットプレイス名。プラグインをインストールするときは、そのプラグイン自身のマーケットプレイスのリストのみが適用され、その依存関係チェーン全体に適用されます。[プラグイン依存関係](/docs/ja/plugins/dependencies) を参照してください |
| `renames`                                 | object           | 前のプラグイン `name` を現在の名前にマップするか、削除したプラグインの場合は `null` にマップします。Claude Code v2.1.193 以降が必要です。[マーケットプレイスのホストと保守](/docs/ja/plugins/host-marketplace) を参照してください                       |

<h2 id="plugin-entries">
  プラグインエントリ
</h2>

`marketplace.json` のトップレベル `plugins` 配列内の各オブジェクトはプラグインに名前を付け、そこからフェッチする場所を指定します。`name` および `source` は必須です。

エントリは、`description`、`version`、`author`、`commands`、および `hooks` などのすべての [`plugin.json` フィールド](/docs/ja/plugins/manifest-reference) も受け入れます。これらのフィールドが適用される場合については、[エントリが plugin.json とどのように組み合わされるか](#entry-and-plugin-json) を参照してください。

テーブルはエントリ自身のフィールドと、エントリ内での意味が変わるマニフェストフィールドをリストします。

| フィールド            | 型                | 説明                                                                                                                                                                                                                                            |
| :--------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | プラグイン識別子。スペース、制御文字、または双方向フォーマット文字なし。ユーザーはインストール時に `@` の前に入力します。プラグイン自身の `plugin.json` が異なる `name` を設定している場合でも                                                                                                                                 |
| `source`         | string or object | プラグインをフェッチする場所。[プラグインソース](#plugin-sources) を参照してください                                                                                                                                                                                          |
| `description`    | string           | [`/plugin`](/docs/ja/plugins/install) リストと詳細に表示されます                                                                                                                                                                                                |
| `version`        | string           | プラグインのバージョン文字列。`plugin.json` も `version` を設定する場合、`plugin.json` が優先され、`claude plugin validate` が警告します。[プラグイン読み込みリファレンス](/docs/ja/plugins/loading) を参照してください                                                                                         |
| `category`       | string           | カタログを整理するための自由形式のカテゴリ                                                                                                                                                                                                                         |
| `tags`           | array of strings | 検索用の自由形式のタグ                                                                                                                                                                                                                                   |
| `strict`         | boolean          | デフォルト `true`。`plugin.json` がプラグインのコンポーネントの決定的なソースであるかどうか。[厳密モード](#strict-mode) を参照してください                                                                                                                                                      |
| `relevance`      | object           | Claude Code にプラグインをいつ提案するかを伝えるシグナル。[組織のプラグインを推奨する](/docs/ja/plugins/relevance) を参照してください                                                                                                                                                           |
| `dependencies`   | array            | このプラグインが機能するために有効にする必要があるプラグイン。各項目は `"name"`、`"name@marketplace"`、またはオブジェクトです。[プラグイン依存関係](/docs/ja/plugins/dependencies) を参照してください                                                                                                                 |
| `defaultEnabled` | boolean          | デフォルト `true`。ユーザーが [`enabledPlugins`](/docs/ja/settings-reference#enabledplugins) で設定していない場合、プラグインが有効で開始するかどうか。エントリ値は `plugin.json` より優先されます                                                                                                       |
| `displayName`    | string           | UI に表示される人間が読める名前。エントリもプラグインの `plugin.json` も設定しない場合、ユーザーはプラグインの `name` を見ます                                                                                                                                                                  |
| `metadata`       | object           | 独自のフィールド用の自由形式のオブジェクト。Claude Code はそれを読み取りません。Claude Code v2.1.222 以降が必要です                                                                                                                                                                    |
| `headers`        | object           | Claude Code がこのエントリの [アーカイブ](#archive-plugin-source) をダウンロードするときに送信する HTTP ヘッダー。ここで設定されたヘッダーは、マーケットプレイスソースの [`headers`](#fields-by-type) から同じ名前のヘッダーを置き換えます。Claude Code v2.1.238 以降が必要です                                                      |
| `headersHelper`  | string           | このエントリのアーカイブダウンロードヘッダーを 1 つの JSON オブジェクトとして出力するコマンド。有効期限が切れる認証情報用です。エントリは [`"strict": false`](#strict-mode) も設定する必要があります。Claude Code v2.1.238 以降が必要です。[アーカイブダウンロードの認証](/docs/ja/plugins/host-marketplace#authenticate-archive-downloads) を参照してください |

<h3 id="entry-and-plugin-json">
  エントリが plugin.json とどのように組み合わされるか
</h3>

エントリのフィールドは、フェッチされたプラグインが独自の `.claude-plugin/plugin.json` を持つ場合と持たない場合で異なる方法で適用されます：

* **`plugin.json` なし**: エントリは `strict` に関係なくマニフェストです。[`mcpServers`、`lspServers`、`userConfig`、および `channels`](/docs/ja/plugins/manifest-reference) を含むすべてのマニフェストフィールドがエントリに適用されます。
* **`plugin.json` 存在**: `plugin.json` はマニフェストです。[厳密モード](#strict-mode) は、エントリの 6 つのコンポーネントフィールド `commands`、`agents`、`skills`、`hooks`、`outputStyles`、および `themes` が組み合わされるか、競合として拒否されるかを決定します。エントリ `mcpServers`、`lspServers`、`userConfig`、および `channels` は適用されません。`plugin.json` で宣言してください。

<h4 id="hooks-in-an-entry">
  エントリ内のフック
</h4>

エントリ `hooks` をフックイベント名をマッチャー配列にマップするインラインオブジェクトとして記述します。ファイルパスまたは配列を記述する場合、`claude plugin validate` はそれを渡します。これらのフックは実行されず、Claude Code はプラグインの `not yet supported in a marketplace entry` エラーを報告します。ファイルベースのフックをプラグイン自身の [`hooks/hooks.json`](/docs/ja/plugins/components) または `plugin.json` に入れてください。

<h4 id="display-fields">
  表示フィールド
</h4>

エントリとプラグイン自身の `plugin.json` の両方が、表示フィールド `displayName`、`description`、`author`、`homepage`、`repository`、`license`、および `keywords` を設定できます。ユーザーはインストール前後のプラグインリストと詳細でこれらの値を見ます：

* エントリで設定したフィールドの場合、ユーザーはエントリの値を見ます。`plugin.json` が異なる値を設定している場合でも。
* エントリが設定しないフィールドの場合、ユーザーは `plugin.json` 値を見ます。

インストール前に、Claude Code は [相対パスソース](#relative-path-plugin-source) を持つエントリの `plugin.json` のみを読み取ることができます。そのプラグインファイルはマーケットプレイス内にあります。他のソースタイプを持つエントリの場合、ユーザーはプラグインをインストールするまでエントリ自身のフィールドのみを見ます。

<h3 id="strict-mode">
  厳密モード
</h3>

`strict` は、フェッチされたプラグインが独自の `plugin.json` を持ち、エントリが [コンポーネントフィールド](#entry-and-plugin-json) のいずれかも宣言する場合に何が起こるかを決定します：`commands`、`agents`、`skills`、`hooks`、`outputStyles`、または `themes`。デフォルトの `strict: true` では、Claude Code はエントリのコンポーネントフィールドを `plugin.json` に追加します。ただし `hooks` は例外で、そのマッチャーはマニフェストのイベントごとのマッチャーを置き換えます。`strict: false` では、コンポーネントフィールドを宣言するエントリは競合であり、プラグインは読み込みに失敗します。テーブルは `strict`、`plugin.json`、およびエントリのコンポーネントフィールドの各組み合わせを示します。

| `strict`     | `plugin.json` | エントリコンポーネントフィールド | 結果                                                                                                                                                                                        |
| :----------- | :------------ | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any          | absent        | any              | エントリはマニフェストです                                                                                                                                                                             |
| `true`、デフォルト | present       | any              | `plugin.json` が権限です。Claude Code はエントリのコンポーネントフィールドを追加します。ただし `hooks` は例外で、そのマッチャーは [マニフェストのイベントごとのマッチャーを置き換えます](/docs/ja/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`      | present       | none             | `true` と同様に `plugin.json` はマニフェストです                                                                                                                                                       |
| `false`      | present       | one or more      | 競合。プラグインは `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components` で読み込みに失敗します                                                                |

<h2 id="plugin-sources">
  プラグインソース
</h2>

プラグインエントリの `source` は、Claude Code がそのプラグインをどこから取得するかを指定します。相対パス文字列か、独自の `source` キーでタイプを指定するオブジェクトのいずれかです。エントリは `"source": { "source": "github", "repo": "your-org/formatter" }` のようになります。

以下の表は、各プラグインソースタイプとそのフィールドを示しています。

| タイプ          | フィールド                          | 注記                                                                                                                                                          |
| :----------- | :----------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 相対パス         | 文字列そのもの                        | マーケットプレイス内のディレクトリで、マーケットプレイスルートから解決されます。`./` で始まる必要があります。ただし、[`metadata.pluginRoot` の下に裸の名前を記述する](#relative-path-plugin-source)場合は除きます。`"."` 単独はルート自体を意味します |
| `github`     | `repo`、`ref`、`sha`             | `owner/repo` 形式の GitHub リポジトリ                                                                                                                               |
| `url`        | `url`、`ref`、`sha`              | URL による任意の git リポジトリ                                                                                                                                        |
| `git-subdir` | `url`、`path`、`ref`、`sha`       | git リポジトリの 1 つのサブディレクトリで、スパース部分クローンで取得されます                                                                                                                  |
| `npm`        | `package`、`version`、`registry` | npm パッケージで、npm クライアントで取得され、インストールスクリプトを実行せずに展開されます                                                                                                          |
| `archive`    | `url`、`sha256`                 | HTTPS 経由の Zip アーカイブ。Claude Code v2.1.224 以降が必要です                                                                                                            |
| `command`    | `command`、`timeout`、`mode`     | Claude Code がユーザーのマシン上で実行するコマンドによって出力されるディレクトリ。Claude Code v2.1.229 以降が必要です                                                                                 |

`url` と `github` という名前は[マーケットプレイスソース](#marketplace-sources)タイプでもあります。ここで `url` は git リポジトリではなく `marketplace.json` ファイルへの直接リンクを意味します。`git` はマーケットプレイスソースとしてのみ存在し、`npm` は両方として存在します。`git-subdir`、`archive`、`command` はプラグインソースとしてのみ存在します。

マーケットプレイスリポジトリ自体のサブディレクトリにあるプラグインには相対パスを使用します。他のリポジトリのサブディレクトリには `git-subdir` を使用します。

`github`、`url`、`git-subdir` ソースは `ref` と `sha` フィールドを共有します。

* **`ref`**: ブランチまたはタグ。リポジトリのデフォルトブランチにデフォルト設定されます。
* **`sha`**: 40 文字の小文字のコミット SHA。`ref` と `sha` の両方を設定すると、Claude Code は `sha` をチェックアウトします。GitHub、GitLab、Bitbucket を含むほとんどの git ホストでは、`ref` で指定されたブランチまたはタグが上流で削除されていても、コミットがリポジトリから到達可能である限り、インストールは成功します。AWS CodeCommit などの一部のサーバーは SHA によるコミット取得をサポートしていません。これらのサーバーでは、`ref` が存在し、ピン留めされたコミットがそこから到達可能である必要があります。

各タイプがどのように取得、キャッシュ、バージョン管理されるかについては、[プラグイン読み込みリファレンス](/docs/ja/plugins/loading)を参照してください。

<h3 id="relative-path-plugin-source">
  相対パスプラグインソース
</h3>

パスはマーケットプレイスルートから解決されます。`./plugins/formatter` は `<root>/plugins/formatter` です。マーケットプレイスファイルが `<root>/.claude-plugin/` にあっても同じです。

`..` を含むパスは検証に失敗します。macOS と Linux では、Claude Code は先頭の `./` の後に任意の場所にバックスラッシュを含むエントリパスを拒否するため、パスはフォワードスラッシュで記述してください。

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

相対パスはマーケットプレイスのファイルを Claude Code が持つ場合にのみ解決されるため、[マーケットプレイスソース](#marketplace-sources)タイプを確認してください。

* **`github`、`git`、`file`、`directory`**: Claude Code はマーケットプレイスのファイルを持っています。
* **`url`**: Claude Code は `marketplace.json` のみを取得するため、相対パスは解決できません。各プラグインに `github` や `git-subdir` などのオブジェクトソースを指定してください。
* **`settings`**: 相対パスは完全に拒否されます。

<h4 id="bare-names-under-pluginroot">
  pluginRoot の下の裸の名前
</h4>

裸の名前は `/` を含まない単一のディレクトリ名です。例えば `"formatter"` です。`./` パスの代わりに裸の名前を記述するには、[`metadata.pluginRoot`](#top-level-fields)をそれらが解決するディレクトリに設定します。`"pluginRoot": "./plugins"` の場合、`"source": "formatter"` は `./plugins/formatter` に解決されます。Claude Code v2.1.239 以降が必要です。

`metadata.pluginRoot` には以下の制限があります。

* それ自体がマーケットプレイス内の相対パスである必要があります。
* 既に `./` で始まるソースには影響を与えません。
* `team-a/formatter` のように `/` を含むソースは裸の名前ではなく、`metadata.pluginRoot` が設定されていても `./` プレフィックスが必要です。

<h3 id="github-plugin-source">
  github プラグインソース
</h3>

`repo` は `owner/repo` を取ります。`ref` と `sha` はオプションです。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  url プラグインソース
</h3>

`url` は完全な git URL です。`https://`、`http://`、`file://`、または `git@` です。`.git` サフィックスは必須ではないため、Azure DevOps と AWS CodeCommit の URL はそのまま機能します。このタイプは `owner/repo` ショートハンドを取りません。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  git-subdir プラグインソース
</h3>

`url` は完全な git URL または GitHub `owner/repo` ショートハンドを受け入れます。`path` はプラグインを保持するサブディレクトリで、Claude Code はそのサブディレクトリのみをダウンロードします。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  npm プラグインソース
</h3>

`npm` ソースは以下のフィールドを取ります。

* `package`: パッケージ名、または `@your-org/formatter` のようなスコープ付き名前
* `version`: バージョンまたは範囲
* `registry`: デフォルトレジストリにないパッケージのレジストリ URL

Claude Code はあなたの npm クライアントでパッケージを取得します。パッケージのインストールスクリプト（`preinstall` や `postinstall` など）は実行されず、その依存関係は取得中にインストールされません。パッケージが `package.json` の隣にサポートされているロックファイルを持っている場合、Claude Code はそれらの[Node.js パッケージ依存関係](/docs/ja/plugins/loading#node-js-package-dependencies)を別のステップでインストールします。この場合もスクリプトは無効です。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  archive プラグインソース
</h3>

`url` は `https://` を使用する必要があり、ループバック、リンクローカル、またはクラウドメタデータホストを指すことはできません。

プラグインルートは zip の最上部または 1 つ下のディレクトリにある場合があります。

`sha256` はアーカイブのダイジェストで、64 文字の 16 進数です。大文字でも小文字でも構いません。これを設定すると、Claude Code は一致しないダウンロードを拒否します。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  command プラグインソース
</h3>

ユーザーのマシンにインストールされたツールがプラグインディレクトリを生成する場合（例えば、ユーザーが選択したツールチェーンのプラグインをレンダリングする IDE など）に `command` ソースを使用します。Claude Code はユーザーがプラグインをインストールまたは更新するときにコマンドを実行し、[セッションごとに 1 回再度実行](/docs/ja/plugins/loading#when-a-command-source-re-runs)するため、ユーザーは再インストールなしでツールの変更された出力を取得します。

`command` ソースは以下のフィールドを取ります。

* `command`: プラグインディレクトリの絶対パスを 1 行として出力し、終了コード 0 で終了するシェルコマンド。Claude Code はユーザーに実行前にレビュー用の文字列全体を表示します。印字可能な ASCII で記述し、最大 500 文字で、4 文字以上の連続スペースはありません。
* `timeout`: 1 から 600 までの秒数。デフォルトは 60 です。
* `mode`: `copy`（デフォルト）または `link`。[コピーモードとリンクモード](#copy-mode-and-link-mode)を参照してください。

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

ユーザーがコマンドを受け入れる方法については、[シェルからインストール](/docs/ja/plugins/install#install-from-your-shell)を参照してください。コマンドを変更した後にユーザーが何を見るかについては、[command ソースのコマンドを変更](/docs/ja/plugins/host-marketplace#change-the-command-of-a-command-source)を参照してください。管理者は [`disableCommandPluginSources`](/docs/ja/settings-reference#disablecommandpluginsources) でコマンドソースをオフにします。

<h4 id="what-the-command-must-do">
  コマンドが実行する必要があること
</h4>

以下の要件を満たすようにコマンドを記述してください。

* **シェルと作業ディレクトリ**: Claude Code はコマンドを `sh` を通じて実行するか、Windows では `cmd.exe` を通じて実行し、ユーザーのホームディレクトリから実行します。絶対パスまたは `PATH` 上のコマンドを指定してください。
* **出力**: stdout に正確に 1 行、プラグインディレクトリの絶対パスを出力し、`timeout` 秒以内に終了コード 0 で終了します。
* **ディレクトリの内容**: コマンドが終了するまでに、ディレクトリはプラグイン全体を保持します。パスは実行ごとに異なる場合があります。

<h4 id="output-that-fails-the-install-or-update">
  インストールまたは更新に失敗する出力
</h4>

コマンドが 0 以外で終了する、`timeout` より長く実行される、または 1 つの絶対パス以外を出力する場合、インストールまたは更新は失敗します。また、出力されたディレクトリが以下のいずれかの場合も失敗します。

* **プラグインコンテンツなし**: 出力されたディレクトリの最上部にプラグインコンテンツがありません。例えば `.claude-plugin/` ディレクトリや `skills/`、`commands/`、`agents/`、`hooks/` ディレクトリなどです。
* **セッション自体のディレクトリ**: 出力されたディレクトリは Claude Code が開始されたディレクトリ、またはその親の 1 つです。
* **ネットワークパス**: Windows では、出力されたパスは UNC パスです。
* **コピーするには大きすぎます**: コピーモードでは、ディレクトリが 256 MiB より大きいか、20,000 を超えるエントリを持っています。

<h4 id="copy-mode-and-link-mode">
  コピーモードとリンクモード
</h4>

`mode` は Claude Code が出力されたディレクトリをコピーするか、それを所定の位置で使用するかを決定します。

* **`copy`**: Claude Code はディレクトリをプラグインキャッシュにコピーし、コピーされたファイルのハッシュから[プラグインバージョン](/docs/ja/plugins/loading#how-claude-code-computes-the-version)を導出します。ツールはコマンド終了後にディレクトリを削除または上書きできます。同じファイルを生成する再実行は最新と見なされます。
* **`link`**: Claude Code はプラグインのキャッシュエントリを出力されたディレクトリの各最上位エントリへのリンクで満たし、ファイルを所定の位置で読み込みます。何もコピーされず、ファイルの内容はハッシュされず、サイズ制限は適用されません。コピーするには大きすぎるディレクトリ（レンダリングされた SDK エクスポートなど）に使用します。

リンクモードプラグインには以下の要件があります。

* **ディレクトリを所定の位置に保つ**: Claude Code はすべての起動時にリンクを通じてプラグインを読み込むため、出力されたディレクトリはプラグインがインストールされている限り、その場所に留まる必要があります。
* **新しいコンテンツを通知するために異なるパスを出力**: バージョンはファイル内の内容ではなく、出力されたディレクトリの実際のパスとその最上位エントリから取得されます。
* **ディレクトリ内に最上位シンボリックリンクを保つ**: 最上位エントリが出力されたディレクトリの外を指すシンボリックリンクの場合、インストールは失敗します。
* **`node_modules` を含める**: Claude Code はリンクモードプラグインの[Node.js パッケージ依存関係インストール](/docs/ja/plugins/loading#node-js-package-dependencies)をスキップするため、プラグインが必要とするパッケージを既に含むディレクトリを出力してください。
* **ディレクトリ内で開始されたセッション**: 出力されたディレクトリまたはその下の任意の場所で開始されたセッションはプラグインを読み込みません。
* **Windows ではない**: Claude Code は Windows でリンクモードプラグインのインストールを拒否します。そこで `"mode": "copy"` を宣言してください。

<h2 id="marketplace-sources">
  マーケットプレイスソース
</h2>

マーケットプレイスソースは Claude Code が `marketplace.json` をどこからフェッチするかを指定します。CLI はマーケットプレイスを追加するときにあなたのために 1 つを構築し、設定で自分で 1 つを記述します：

* **[`claude plugin marketplace add`](/docs/ja/plugins/cli-reference)**: Claude Code は渡した文字列からソースを構築します。
* **[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces)**: `source` オブジェクトとして自分でソースを記述します。
* **[`strictKnownMarketplaces`](/docs/ja/settings-reference#strictknownmarketplaces) および [`blockedMarketplaces`](/docs/ja/plugins/org#restrict-what-users-can-install)**: 管理者はこれら 2 つのポリシーリストでソースを記述します。`strictKnownMarketplaces` は許可リストで、`blockedMarketplaces` は拒否リストです。

タイプ名 `url`、`git`、および `github` は、[プラグインソース](#plugin-sources) とは異なるマーケットプレイスソースで異なる意味を持ちます：

| タイプ名     | マーケットプレイスソースとして                                                          | プラグインソースとして                                         |
| :------- | :----------------------------------------------------------------------- | :-------------------------------------------------- |
| `url`    | `marketplace.json` ファイルへの直接リンク。フィールド `url`、`headers`、および `headersHelper` | クローンする git リポジトリ。フィールド `url`、`ref`、および `sha`        |
| `git`    | クローンする git リポジトリ。フィールド `url`、`ref`、`path`、および `sparsePaths`              | 存在しません                                              |
| `github` | GitHub リポジトリ。フィールド `repo`、`ref`、`path`、および `sparsePaths`                 | GitHub リポジトリ。フィールド `repo`、`ref`、および `sha`。`path` なし |

テーブルはすべてのマーケットプレイスソースタイプをそのフィールド、それを生成する `claude plugin marketplace add` 入力、および 3 つの設定キーのそれぞれでの動作とともにリストします。

| タイプ           | フィールド                             | `marketplace add` 入力                                                                                                              | `extraKnownMarketplaces`                                 | `strictKnownMarketplaces`                                                                                                                                                 | `blockedMarketplaces`                        |
| :------------ | :-------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------- |
| `url`         | `url`、`headers`、`headersHelper`   | git フォームに一致しない `http://` または `https://` URL                                                                                       | 読み込みます                                                   | 同じ URL を許可します                                                                                                                                                             | 同じ URL をブロックします                              |
| `github`      | `repo`、`ref`、`path`、`sparsePaths` | `owner/repo`、`owner/repo@ref`、または `owner/repo#ref`                                                                                | 読み込みます                                                   | 同じ `repo`、`ref`、および `path` を許可します。`repo` は `owner/*` である可能性があります                                                                                                          | 同じものをブロックし、同じリポジトリへの `git` URL をブロックします      |
| `git`         | `url`、`ref`、`path`、`sparsePaths`  | `user@host:path` URL、または `.git` で終わる `https://` URL、`/_git/` を含む、または github.com または gitlab.com リポジトリに名前を付ける。`#ref` は ref をピン留めします | 読み込みます                                                   | 同じ URL、`ref`、および `path` を許可します                                                                                                                                            | 同じものをブロックし、同じ github.com リポジトリの他のスペルをブロックします |
| `npm`         | `package`                         | 生成されません                                                                                                                           | 読み込みに失敗します：`NPM marketplace sources not yet implemented` | 解析されますが、何も登録されていないため何も一致しません。`npm` マーケットプレイス                                                                                                                              | 解析されますが何も一致しません                              |
| `file`        | `path`                            | `.json` ファイルへのパス                                                                                                                  | 読み込みます                                                   | 同じパスを許可します                                                                                                                                                                | 同じパスをブロックします                                 |
| `directory`   | `path`                            | ディレクトリへのパス                                                                                                                        | 読み込みます                                                   | 同じパスを許可します                                                                                                                                                                | 同じパスをブロックします                                 |
| `settings`    | `name`、`plugins`、`owner`          | 生成されません                                                                                                                           | 読み込みます                                                   | 同じ `name` と同じ `plugins` を持つエントリを許可します                                                                                                                                     | 同じ `name` をブロックします                           |
| `skills-dir`  | なし                                | 生成されません                                                                                                                           | 読み込みに失敗します：`Unsupported marketplace source type`         | 許可リストが設定されている間、[スキルディレクトリプラグイン](/docs/ja/plugins/org#keep-skills-directory-plugins-loading) を読み込み続けます。[ポリシーリストでのみ有効なソース値](#source-values-valid-only-in-policy-lists) を参照してください | スキルディレクトリプラグインの読み込みを停止します                    |
| `hostPattern` | `hostPattern`                     | 生成されません                                                                                                                           | 読み込みに失敗します：`Unsupported marketplace source type`         | ホストが一致する `github`、`git`、および `url` ソースを許可します                                                                                                                               | これらのソースをブロックします                              |
| `pathPattern` | `pathPattern`                     | 生成されません                                                                                                                           | 読み込みに失敗します：`Unsupported marketplace source type`         | `path` が一致する `file` および `directory` ソースを許可します                                                                                                                             | これらのソースをブロックします                              |

<h3 id="fields-by-type">
  タイプ別フィールド
</h3>

テーブルはデフォルト、制約、またはそのタイプに固有の意味を持つ各マーケットプレイスソースフィールドをリストします。

| フィールド           | タイプ            | 説明                                                                                                                                                                                                            |
| :-------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `url`           | `url`          | `marketplace.json` ファイルへのリンク。Claude Code はそのファイルのみをダウンロードするため、マーケットプレイスのプラグインは [相対パスソース](#relative-path-plugin-source) を使用できません                                                                               |
| `url`           | `git`          | クローンする git リポジトリ                                                                                                                                                                                              |
| `headers`       | `url`          | Claude Code が認証されたホストのフェッチで送信する HTTP ヘッダーのマップ                                                                                                                                                                 |
| `headersHelper` | `url`          | `headers` にリストするには短すぎる値を持つヘッダーを出力するコマンド。Claude Code v2.1.238 以降が必要です。[アーカイブダウンロードの認証](/docs/ja/plugins/host-marketplace#authenticate-archive-downloads) を参照してください                                                  |
| `repo`          | `github`       | `marketplace add` および `extraKnownMarketplaces` では、`repo` は 1 つのリポジトリに名前を付ける必要があります。`marketplace add` は `owner/*` を有効な `owner/repo` 短縮形として拒否します。`extraKnownMarketplaces` では Claude Code はそれを文字通りに取り、クローンは失敗します |
| `ref`           | `github`、`git` | ブランチまたはタグ。リポジトリのデフォルトブランチにデフォルト設定されます                                                                                                                                                                         |
| `path`          | `github`、`git` | リポジトリ内のマーケットプレイスファイルのパス。デフォルトは `.claude-plugin/marketplace.json`                                                                                                                                              |
| `path`          | `file`         | マーケットプレイスファイル自体。Claude Code はそれを使用中に読み取り、2 レベル上のディレクトリをマーケットプレイスルートとして取ります。ファイルを `<root>/.claude-plugin/marketplace.json` に保つ                                                                                 |
| `path`          | `directory`    | マーケットプレイスルート。`.claude-plugin/marketplace.json` を含むディレクトリ                                                                                                                                                      |
| `sparsePaths`   | `github`、`git` | スパースチェックアウト用のディレクトリの配列。`[".claude-plugin", "plugins"]` など。`claude plugin marketplace add --sparse` が設定します                                                                                                     |
| `skipLfs`       | `github`、`git` | 受け入れられ、効果がありません。[プラグインファイルを Git LFS から除外](/docs/ja/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs) を参照してください                                                                                            |
| `name`          | `settings`     | `extraKnownMarketplaces` キーと等しい必要があり、[予約名](#reserved-names) にすることはできません                                                                                                                                       |
| `plugins`       | `settings`     | インラインカタログ。ホストされたファイルなし。各項目は `name`、`source`、`description`、`version`、`strict`、`headers`、および `headersHelper` を取ります。相対パスには解決するリポジトリがないため、各項目の `source` をオブジェクトタイプとして記述してください                                     |

<h3 id="source-values-valid-only-in-policy-lists">
  ポリシーリストでのみ有効なソース値
</h3>

`hostPattern`、`pathPattern`、`skills-dir`、および `repo` の `owner/*` フォームは、2 つのポリシーリスト `strictKnownMarketplaces` および `blockedMarketplaces` でのみ有効です：

* **`hostPattern` および `pathPattern`**: Claude Code がソースをフェッチする前にテストする正規表現。
* **`skills-dir`**: ソースではありません。`strictKnownMarketplaces` をすべて設定する場合、[スキルディレクトリプラグイン](/docs/ja/plugins/org#keep-skills-directory-plugins-loading) は `{"source": "skills-dir"}` をそのリストに追加するまで読み込みを停止します。
* **`owner/*`**: `github` `repo` 値として、正確にその GitHub 所有者の下のすべてのリポジトリに一致します。Claude Code v2.1.223 以降が必要です。

マッチ順序、正確な `ref` セマンティクス、およびレシピについては、[組織のプラグインを管理する](/docs/ja/plugins/org) を参照してください。

<h3 id="source-objects-in-settings">
  設定のソースオブジェクト
</h3>

`extraKnownMarketplaces` 値はマーケットプレイス名から `source` を持つオブジェクトへのマップです。このエントリは `main` ブランチの git リポジトリからマーケットプレイスを登録します：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` および `blockedMarketplaces` はソースオブジェクトの配列です。この許可リストは 1 つの GitHub 所有者と 1 つの内部ホストを許可します：

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  検証メッセージ
</h2>

`claude plugin validate <path>` はマーケットプレイスのルートまたはマーケットプレイスファイル自体を受け取ります。エラーと警告を出力します。終了コードと `--strict` については、[plugin validate](/docs/ja/plugins/cli-reference#plugin-validate) を参照してください。

メッセージはプラグインエントリをそのインデックスで名前付けします。`plugins.1.source` または `plugins[1].source` として記述されます。

`plugins[2] plugin.json →` などのエントリインデックスと `plugin.json →` で始まるメッセージは、そのプラグイン自体のファイルに関するものです。[`claude plugin validate` がエラーを報告する](/docs/ja/plugins/troubleshooting#claude-plugin-validate-reports-errors) にはそれらのメッセージと修正方法が記載されています。

Claude Desktop フラグ名に言及する警告は、Claude Code が受け入れるが Claude Desktop が拒否するものです。Claude Desktop の名前ルールがより厳密であるためです。

表はマーケットプレイスレベルのメッセージを各メッセージが関連するフィールドにマップしています。

| メッセージ                                                                                                                                                                                               | レベル | フィールド                                                                               |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-- | :---------------------------------------------------------------------------------- |
| `Marketplace must have a name`                                                                                                                                                                      | エラー | `name` が空です                                                                         |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                                   | エラー | `name`                                                                              |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                               | エラー | `name`                                                                              |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                            | エラー | `name`。[予約名](#reserved-names) を参照してください                                             |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                                    | エラー | `name` に制御文字（エスケープや改行など）または Unicode 双方向フォーマット文字が含まれています                             |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, and the `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github`, and `gh` variants | エラー | `name`                                                                              |
| `Author name cannot be empty`                                                                                                                                                                       | エラー | `owner.name`                                                                        |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                             | エラー | `plugins[i].name`                                                                   |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                         | エラー | `plugins[i].name`                                                                   |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                                    | エラー | 2 つのエントリが同じ `name` を共有しています                                                         |
| `plugins.i.source: Invalid input`                                                                                                                                                                   | エラー | エントリの `source` がどのタイプにも一致しません。[source の無効な入力](#invalid-input-on-a-source) を参照してください |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                                     | エラー | マーケットプレイスルートをエスケープする相対 `source`                                                     |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                                   | エラー | `plugins[i].source`                                                                 |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                          | エラー | `plugins[i].headersHelper`、`archive` エントリ上                                          |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                                 | エラー | `renames.<old>`                                                                     |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                            | エラー | `renames.<old>`                                                                     |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                           | 警告  | トップレベル、`metadata` の下、エントリ内、またはエントリの `relevance` の下の名前付きキー                           |
| `Marketplace has no plugins defined`                                                                                                                                                                | 警告  | `plugins` が空です                                                                      |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                                  | 警告  | `plugins[i].headers` または `plugins[i].headersHelper`、`source` が `archive` ではないエントリ上  |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                        | 警告  | `plugins[i].source.sha256`                                                          |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                          | 警告  | `plugins[i].headers.<name>`                                                         |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                                | 警告  | `plugins[i].source`                                                                 |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                                     | 警告  | `description`                                                                       |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                                     | 警告  | `plugins[i].version`、相対パスエントリ上                                                      |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                          | 警告  | `plugins[i].relevance`                                                              |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                               | 警告  | `plugins[i].metadata`                                                               |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                                  | 警告  | `plugins[i].experimental`                                                           |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                                | 警告  | `name` が `org`、`org-provisioned`、または `unknown` です。Claude Desktop はマーケットプレイスを拒否します   |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                   | 警告  | `name`。Claude Desktop はマーケットプレイスを拒否します                                              |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                        | 警告  | `plugins[i].name`。Claude Desktop はエントリを削除します                                        |

<h3 id="invalid-input-on-a-source">
  source の無効な入力
</h3>

`source` の `Invalid input` は、オブジェクトがどのソースタイプにも一致しなかったことを意味します。これらの原因を確認してください：

* `./` で始まらない相対パス。ただし `"."` または [metadata.pluginRoot の下のベアネーム](#relative-path-plugin-source) を除く
* `..` を含む `npm` `package`
* [プラグインソース](#plugin-sources) の 1 つではない `source` タイプ
* `github` なしの `repo` など、必須フィールドが欠落しているか間違った型の既知のタイプ

<h3 id="failures-that-validation-doesn’t-catch">
  検証が検出しない失敗
</h3>

`claude plugin validate` はすべての失敗を報告するわけではありません。ファイルパスまたは配列として記述されたエントリ `hooks` は検証に合格し、エラーはプラグインが読み込まれるときにのみ表示されます。[エントリ内の Hooks](#hooks-in-an-entry) で説明されているとおりです。`source` をフェッチするエラーも、検証時ではなくインストール後にのみ表示されます。

[`claude plugin list`](/docs/ja/plugins/cli-reference) は読み込みに失敗したプラグインをそのエラーとともに表示し、[プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting) は読み込み時の文字列をカバーしています。

<h2 id="next-steps">
  次のステップ
</h2>

* [マーケットプレイスの作成](/docs/ja/plugins/create-marketplace): これらのフィールドからマーケットプレイスを構築し、ローカルからインストール
* [マーケットプレイスのホストと保守](/docs/ja/plugins/host-marketplace): ファイルを配置する場所とユーザーが変更を受け取る方法
* [プラグインマニフェストリファレンス](/docs/ja/plugins/manifest-reference): エントリがオーバーライドできる `plugin.json` フィールド
* [組織のプラグインを管理する](/docs/ja/plugins/org): これらのソース値を使用する許可リストと拒否リストのレシピ
