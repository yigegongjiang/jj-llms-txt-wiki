> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグイン依存関係

> プラグインが依存するプラグインを宣言し、^1.2 などのバージョン範囲を指定して、Claude Code がどのようにインストール、解決、削除するかを確認します。

プラグイン依存関係は、プラグインが依存する別のプラグインです。例えば、その MCP サーバーまたはスキルを呼び出すプラグインなどです。各依存関係は、バージョン制約を宣言しない限り、マーケットプレイスが提供する最新バージョンを追跡します。バージョン制約は、`^2.0` や `~2.1.0` などのセマンティックバージョン範囲で、テスト済みのものです。

このページは、`plugin.json` で依存関係を宣言するプラグイン作成者と、リリースにタグを付けるマーケットプレイス管理者向けです。

<Note>
  以下のケースは他のページで説明されています。

  * **依存関係を持つプラグインのインストール**: [インストール済みプラグインの管理](/docs/ja/plugins/install#manage-installed-plugins)を参照してください
  * **依存関係エラーの読み取り**: [依存関係エラー](/docs/ja/plugins/troubleshooting#dependency-errors)を参照してください
  * **プラグイン自体のコードが必要とする npm および Bun パッケージの宣言**: [Node.js パッケージ依存関係](/docs/ja/plugins/loading#node-js-package-dependencies)を参照してください
</Note>

制約を追加するには、[バージョン制約を使用して依存関係を宣言する](#declare-a-dependency-with-a-version-constraint)から始めてください。他のプラグインが依存するプラグインを管理している場合は、[リリースにタグを付ける](#tag-plugin-releases-for-version-resolution)ことで、制約を解決できるようにしてください。

<h2 id="declare-dependencies">
  依存関係を宣言する
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />バージョン制約がない場合、依存関係はマーケットプレイスが公開する新しいリリースのたびに移動します。これは、ユーザーが更新するときに発生します。そのリリースがプラグインが呼び出す MCP ツールの名前を変更した場合、プラグインは更新するすべてのユーザーに対して破損します。

`~2.1.0` などの制約を git バックアップソースの依存関係に設定すると、プラグインがインストールされているユーザーは、依存関係の `2.1.x` パッチを受け取り続け、`2.2` に移動することはありません。独自のスケジュールでアップグレードするには、新しいリリースに対してテストを実行し、より広い制約を持つプラグインの新しいバージョンを公開します。

<h3 id="declare-a-dependency-with-a-version-constraint">
  バージョン制約を使用して依存関係を宣言する
</h3>

プラグインの `.claude-plugin/plugin.json` の `dependencies` 配列に依存関係をリストします。次のマニフェストは、バージョン制約なしの依存関係と制約付きの依存関係を宣言しています。

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

エントリは文字列にすることができます。プラグイン名だけ（このマニフェストの `"audit-logger"` など）、または別のマーケットプレイスで解決するために `"name@marketplace"` です。ベアな文字列の場合、プラグインはそのプラグインのマーケットプレイスが提供するバージョンに依存します。

バージョン制約を設定するには、これらのフィールドを持つオブジェクトを使用します。各フィールドは文字列です。

| フィールド         | 説明                                                                                                                                                                                                                     |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | 依存関係のプラグイン名。マーケットプレイスエントリに表示されるとおりです。Claude Code は、`marketplace` を設定しない限り、宣言プラグインと同じマーケットプレイスで検索します。必須です。                                                                                                              |
| `version`     | `~2.1.0`、`^2.0`、`>=1.4`、または `=2.1.0` などの[セマンティックバージョン範囲](https://github.com/npm/node-semver#ranges)です。依存関係は、この範囲を満たす最高の git タグでインストールされるため、依存関係の管理者は[リリースにタグを付ける](#tag-plugin-releases-for-version-resolution)必要があります。 |
| `marketplace` | `name` を解決する別のマーケットプレイスです。許可リストがクロスマーケットプレイス依存関係を制御します。詳細は[別のマーケットプレイスからプラグインに依存する](#depend-on-a-plugin-from-another-marketplace)を参照してください。                                                                            |

範囲は、`^2.0.0-0` などのプレリリースサフィックスでオプトインしない限り、`2.0.0-beta.1` などのプレリリースバージョンと一致しません。

<h3 id="bundle-plugins-for-a-team">
  チーム向けにプラグインをバンドルする
</h3>

エンジニアが 1 つのコマンドでキュレーションされたプラグインセットをインストールできるようにするには、マニフェストに `name` と `dependencies` 配列を含むプラグインを公開します。プラグインマニフェストは `name` のみが必要なため、これは有効なプラグインであり、インストールするとすべての依存関係がインストールされます。

例えば、プラットフォームチームは内部マーケットプレイスでロール固有のバンドルを公開できるため、エンジニアは各プラグインを個別にインストールする代わりに、1 つの `claude plugin install` を実行します。

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

後で標準セットにプラグインを追加するには、追加の依存関係を持つ新しい `backend-standard` バージョンを公開します。マーケットプレイスが[デフォルトで自動更新しない](/docs/ja/plugins/loading#which-marketplaces-and-plugins-auto-update)場合、エンジニアはマーケットプレイスの自動更新をオンにするか、手動で更新します。

* **マーケットプレイスの自動更新をオンにする**: 次の自動更新はバンドルを新しいバージョンに移動し、追加される依存関係をインストールします。
* **手動で更新する**: シェルで `claude plugin update backend-standard` を実行し、開いているセッションで `/reload-plugins` を実行して、新しく追加された依存関係をインストールします。

エンジニア側の手順については、[プラグインを最新に保つ](/docs/ja/plugins/install#keep-plugins-updated)を参照してください。

バンドルを組織内のすべてのユーザーにデプロイするには、管理者が管理設定の `enabledPlugins` に追加します。[プラグインの事前インストールと要求](/docs/ja/plugins/org#pre-install-and-require-plugins)を参照してください。

<h3 id="depend-on-a-plugin-from-another-marketplace">
  別のマーケットプレイスからプラグインに依存する
</h3>

デフォルトでは、Claude Code は、ユーザーがその依存関係を同じスコープでインストールして有効にしていない限り、宣言プラグイン自体とは異なるマーケットプレイスから依存関係をインストールしません。このデフォルトは、1 つのマーケットプレイスがユーザーが確認していないソースからプラグインをサイレントにインストールするのを防ぎます。

インストールを許可するには、ルートマーケットプレイスの `marketplace.json` の `allowCrossMarketplaceDependenciesOn` にターゲットマーケットプレイスの名前を追加します。ルートマーケットプレイスは、ユーザーがインストールしているプラグインをホストするマーケットプレイスです。ルートマーケットプレイスの許可リストのみが適用されます。

次の `marketplace.json` は、`deploy-kit` が `your-shared-marketplace` からプラグインに依存することを許可します。

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

`allowCrossMarketplaceDependenciesOn` が欠落しているか、ターゲットマーケットプレイスを含まない場合、Claude Code は依存関係をインストールしません。依存関係がマーケットプレイスエントリで宣言されている場合、インストール自体は `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` で始まるメッセージで拒否され、設定するフィールドに名前が付けられます。`plugin.json` で宣言されている場合、インストールは依存関係なしで完了し、プラグインはロードに失敗します。

許可リストチェックは、既に有効になっている依存関係には適用されません。ユーザーが最初に `your-shared-marketplace` から `audit-logger` を同じスコープでインストールした場合、`deploy-kit` はその後、許可リストに変更を加えずにインストールされます。

<h3 id="test-a-plugin-and-its-dependency-locally">
  プラグインとその依存関係をローカルでテストする
</h3>

プラグインとそれが依存するプラグインを同時に開発している場合は、シェルから Claude Code を起動し、[`--plugin-dir`](/docs/ja/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) で両方をロードします。

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

依存関係のローカルコピーはプラグインの依存関係エントリを満たすため、マーケットプレイスから依存関係をインストールする必要はありません。

* **`version` は不要**: ローカル `plugin.json` は、[バージョン制約](#declare-a-dependency-with-a-version-constraint)がローカルコピーに対してチェックされないため、`version` も必要ありません。
* **マーケットプレイスに名前を付けるエントリ**: マーケットプレイスに名前を付けるエントリは、Claude Code v2.1.242 以降のローカルコピーにも一致します。

マーケットプレイスから依存関係をインストールするまで、ローカルコピーが無効または存在しないときはいつでも、プラグインはロードを停止します。

* **ローカルコピーを無効にした**: プラグインは次のプラグインロードで無効になり、`is disabled — enable it or remove the dependency` で終わるエラーが表示されます。エラーが依存関係を `<name>@inline` として名前付けする場合、その識別子は `--plugin-dir` コピーを参照します。
* **依存関係の `--plugin-dir` フラグなしでセッションを開始した**: エラーは依存関係がインストールされていないと報告します。フラグを再度渡すか、マーケットプレイスから依存関係をインストールします。

両方のプラグインが 1 つの親フォルダにある場合、そのフォルダを `--plugin-dir` に 1 回渡すことができます。フォルダ自体がプラグインでない場合、Claude Code は `.claude-plugin/plugin.json` を持つ各子フォルダをロードします。Claude Code v2.1.265 以降が必要です。

<h2 id="tag-plugin-releases-for-version-resolution">
  他のプラグインが依存するプラグインをリリースする
</h2>

他のプラグインがバージョン制約で依存するプラグインを管理している場合は、リリースにタグを付けて、制約を解決できるようにします。制約は、プラグインをホストするリポジトリの git タグに対して解決されます。プラグインの `marketplace.json` の[プラグインソース](/docs/ja/plugins/marketplace-reference#plugin-sources)が指すリポジトリにタグを付けます。

* **`github`、`url`、または `git-subdir` ソース**: プラグイン自体のリポジトリ。プラグインの作成者がタグを作成します
* **`./plugins/secrets-vault` などの相対パス**: マーケットプレイスリポジトリ。マーケットプレイス管理者がタグを作成します

<h3 id="create-a-release-tag">
  リリースタグを作成する
</h3>

各リリースに `<plugin-name>--v<version>` というタグを付けます。`<version>` はそのコミットの `plugin.json` の `version` フィールドと一致します。プラグイン名プレフィックスにより、1 つのマーケットプレイスリポジトリが複数のプラグインをホストでき、独立したバージョン履歴を持つことができます。

プラグインディレクトリから、`origin` リモートが設定されているタグをプッシュするように設定して、[`claude plugin tag`](/docs/ja/plugins/cli-reference#plugin-tag) を使用してタグを作成します。

```bash theme={null}
claude plugin tag --push
```

このコマンドはプラグインのマニフェストからタグ名を構築します。タグを作成する前に、次のチェックを実行します。

* プラグインを検証します
* プラグインディレクトリがマーケットプレイスチェックアウト内にある場合、`plugin.json` とマーケットプレイスエントリがバージョンに同意していることを確認します
* プラグインディレクトリの下でクリーンな作業ツリーが必要です
* タグが既に存在する場合は拒否します

成功した実行は `Created tag secrets-vault--v2.1.0` を出力します。`--push` を使用すると、`Pushed to origin` も出力されます。`--push` なしで、自分で実行する `git push` コマンドを出力します。

`--dry-run` を渡して、何も作成せずにプランを確認します。

[`claude plugin tag` リファレンス](/docs/ja/plugins/cli-reference#plugin-tag)に残りのフラグがリストされています。

`git tag secrets-vault--v2.1.0` を直接実行することもできます。`plugin.json` とマーケットプレイスエントリのバージョンを自分で同期させておく限り。

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  git 以外のソースを持つ依存関係を制約する
</h3>

タグベースの解決は、git バックアップソースにのみ適用されます。`npm`、`archive`、または `command` [プラグインソース](/docs/ja/plugins/marketplace-reference#plugin-sources)を持つ依存関係の場合、制約はどのバージョンがフェッチされるかを制御しません。プラグインがロードされるときにチェックされ、インストールされたバージョンが制約を満たさない場合、依存プラグインは無効になります。

`npm`、`archive`、および `command` ソースの場合、チェックされるバージョンは依存関係の `plugin.json` の `version` です。その依存関係を制約する前に、そこに 1 つを設定します。バージョンを設定しない `plugin.json` は制約を満たしません。

Claude Code は `command` ソースを持つ依存関係を自分でインストールしないため、ユーザーは[最初にインストール](/docs/ja/plugins/marketplace-reference#command-plugin-source)します。また、依存関係の [`headersHelper`](/docs/ja/plugins/host-marketplace#authenticate-archive-downloads) を実行しないため、ユーザーはマーケットプレイスエントリが 1 つを設定する依存関係をプラグインをインストールする前にインストールします。

`claude plugin install` に加えて、これらの操作も宣言された欠落依存関係をインストールし、`command` と `headersHelper` の制限が適用されます。

* `/reload-plugins`
* 依存プラグインのマーケットプレイスの自動更新
* 依存プラグインで `claude plugin install` を再実行する
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  依存関係がユーザーにどのように動作するか
</h2>

これらのセクションでは、プラグインが他のプラグインと一緒にインストールされた後、Claude Code が宣言した制約をどのように解決、チェック、および組み合わせるかについて説明します。

<h3 id="how-a-constraint-resolves-against-tags">
  制約がタグに対してどのように解決されるか
</h3>

ユーザーが `{ "name": "secrets-vault", "version": "~2.1.0" }` を宣言するプラグインをインストールすると、依存関係は `secrets-vault` をホストするリポジトリの `~2.1.0` を満たす最高の `secrets-vault--v` タグからインストールされます。タグが範囲を満たさない場合、インストールは失敗するか、マーケットプレイスの現在のコピーを使用します。

* **独自のリポジトリを持つプラグイン**: インストールは `Dependency "secrets-vault@your-marketplace" has no git tag satisfying` を含むメッセージで失敗します。
* **相対パスで参照されるプラグイン**: インストールは代わりにマーケットプレイスの現在のコピーを使用し、プラグインがロードされるときに制約がチェックされます。そのコピーが範囲外の場合、依存プラグインは無効のままで、`claude plugin list` は `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0` を表示します。

マーケットプレイスが相対パスで参照するプラグインの場合、ローカルフォルダパスとして追加したマーケットプレイスは、フォルダが git リポジトリの場合、そのフォルダの git タグに対して制約を解決します。これには Claude Code v2.1.196 以降が必要です。git リポジトリではないローカルフォルダにはタグがないため、Claude Code はフォルダの現在の内容から依存関係をインストールします。

<h3 id="confirm-the-resolved-version">
  解決されたバージョンを確認する
</h3>

制約が解決されたバージョンを確認するには、シェルで `claude plugin list` を実行します。タグ解決された依存関係は、`2.1.0-8713c5b11005` などの 12 文字のコミットサフィックス付きでバージョンを表示します。

制約チェックは、`plugin.json` の `version` が遅れていても、タグのバージョンを使用します。

タグを別のコミットに強制移動する場合、次のインストールは古いキャッシュコピーを再利用する代わりに、そのコミットのコンテンツをフェッチします。プラグインのバージョンがキャッシュキーになる方法については、[バージョンと更新](/docs/ja/plugins/loading#versions-and-updates)を参照してください。

<h3 id="combine-constraints-from-several-plugins">
  複数のプラグインからの制約を組み合わせる
</h3>

複数のインストール済みプラグインが同じ依存関係を制約する場合、依存関係はすべての範囲を満たす最高バージョンに解決されます。一般的な組み合わせは次のように解決されます。

| プラグイン A が要求 | プラグイン B が要求 | 結果                                                                                         |
| :---------- | :---------- | :----------------------------------------------------------------------------------------- |
| `^2.0`      | `>=2.1`     | `2.1.0` 以上の最高 `2.x` タグで 1 つのインストール。両方のプラグインがロードされます。                                       |
| `~2.1`      | `~3.0`      | プラグイン B のインストールは `has conflicting version requirements` メッセージで失敗します。プラグイン A と依存関係は以前のままです。 |
| `=2.1.0`    | なし          | 依存関係は `2.1.0` のままです。プラグイン A がインストールされている間、自動更新は新しいバージョンをスキップします。                           |

自動更新は、マーケットプレイスの最新バージョンではなく、インストール済みプラグインのすべての範囲を満たす最高 git タグで制約された依存関係をフェッチします。インストール済みプラグインの範囲が重複しない場合、自動更新はその依存関係を現在のバージョンのままにし、`/plugin` **Errors** タブは制約プラグインに名前を付けるエントリを表示します。範囲が重複しているがタグが範囲内に収まらない場合、自動更新はマーケットプレイスの現在のコピーをフェッチし、そのコピーの `version` がインストール済みプラグインの範囲外にある場合は更新をスキップします。

ユーザーが依存関係を制約する最後のプラグインをアンインストールすると、依存関係はバージョン範囲に制約されなくなり、次の更新でマーケットプレイスエントリの追跡を再開します。

<h2 id="see-also">
  関連項目
</h2>

* [`claude plugin prune`](/docs/ja/plugins/cli-reference#plugin-prune): プラグインが不要になった自動インストール依存関係を削除する
* [マーケットプレイスをホストする](/docs/ja/plugins/host-marketplace): リリースチャネルと他のプラグインの推奨
