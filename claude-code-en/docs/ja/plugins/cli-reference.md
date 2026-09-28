> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインコマンドリファレンス

> claude プラグインシェルコマンド、セッション内の /plugin と /reload-plugins、および 1 つのセッションのためにプラグインをロードするフラグの完全なリファレンス。

プラグインコマンドは、シェルまたはスクリプトから `claude plugin` として実行するか、Claude Code セッション内で `/plugin` と `/reload-plugins` として実行します。このリファレンスでは、各コマンドのフラグ、デフォルト、出力、終了コード、および 1 つのセッションのためにプラグインをロードする 2 つのフラグについて説明します。

ビルドで `claude plugin --help` を実行して、バージョンにどのサブコマンドがあるかを確認してください。

<Note>
  これらのケースは他のページで説明されています：

  * **ステップのインストールと管理、および `/plugin` が実行される場所**: [プラグインのインストールと管理](/docs/ja/plugins/install)を参照してください
  * **コマンドがディスク上で何を変更し、どのスコープが優先されるか**: [プラグイン読み込みリファレンス](/docs/ja/plugins/loading)を参照してください
  * **エラーメッセージの意味**: [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)を参照してください
</Note>

<h2 id="claude-plugin-commands">
  claude plugin コマンド
</h2>

Claude Code セッション外のシェルまたはスクリプトから `claude plugin <subcommand>` を実行します。これらのサブコマンドは、[`/plugin`](#plugin-in-a-session) パネルを開かずにプラグインをインストールおよび管理します。

`claude plugins` は `claude plugin` のエイリアスです。

すべてのサブコマンドは、これらの終了コード、プラグイン引数、およびスコープ値を共有します：

* **終了コード**: 成功時は `0`、失敗時は `1`。`validate` は予期しないエラーに対して終了 `2` を追加し、`eval` は[そのセクション](#plugin-eval)にリストされたコードを追加します。
* **プラグイン引数**: `<plugin>` 引数はプラグイン `name` または `name@marketplace` です。2 つのマーケットプレイスが同じ名前を提供する場合は、修飾形式を使用してください。
* **スコープ**: `--scope` は `user`、`project`、または `local` を取り、コマンドが書き込む設定ファイルに名前を付けます。`update` は `managed` も取ります。

<h3 id="plugin-init">
  plugin init
</h3>

`~/.claude/skills/<name>/` に新しいプラグインをスキャフォールドします。次のセッションで `<name>@skills-dir` としてロードされ、インストール手順は不要です。

`new` は `init` のエイリアスです。

このコマンドで始まる作成、テスト、編集ワークフローについては、[プラグインの作成](/docs/ja/plugins/create)を参照してください。

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` は `~/.claude/skills/` の下のディレクトリ名になり、プラグインのマニフェストの `name` になります。

このコマンドには別の場所のフラグはありません。代わりにプロジェクト内にスキャフォールドするには、[プラグインの作成](/docs/ja/plugins/create)を参照してください。

| フラグ                      | 説明                                                                                        |
| :----------------------- | :---------------------------------------------------------------------------------------- |
| `--description <text>`   | マニフェストの説明                                                                                 |
| `--author <name>`        | 著者名。デフォルトは `git config user.name`                                                         |
| `--author-email <email>` | 著者メール。デフォルトは `git config user.email`                                                      |
| `--with <components...>` | `skills`、`agents`、`hooks`、`mcp`、`lsp`、`output-style`、または `channel` のスターターファイルもスキャフォールドします |
| `-f, --force`            | ターゲットの既存の `.claude-plugin/` を上書きします                                                       |

スキル と hook ファイルのスターターを含むプラグインをスキャフォールドします：

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code は書き込んだ内容を検証し、`Created plugin "my-helper" at ~/.claude/skills/my-helper` を出力し、その後にロードされる id と、それをオフにする `claude plugin disable` コマンドを出力します。

Claude Code は安全にスキャフォールドできない場合は `1` で終了し、メッセージは理由を名前付けします。これらは一般的な理由です：

* 不明な `--with` 値
* `--force` なしのターゲットの既存スキャフォールド
* スキルディレクトリプラグインをブロックする管理設定

<h3 id="plugin-install">
  plugin install
</h3>

追加したマーケットプレイスからプラグインをインストールします。`i` は `install` のエイリアスです。

```bash theme={null}
claude plugin install <plugin> [options]
```

ほとんどのプラグインはプロンプトなしでインストールされます。マーケットプレイスエントリが[インストールするコマンドを実行する](/docs/ja/plugins/host-marketplace)か、[ダウンロード用に `headersHelper` を設定する](/docs/ja/plugins/host-marketplace#how-users-accept-a-headershelper-command)プラグインの場合、Claude Code は最初にコマンドを出力し、`Run this command now? [y/N]` と尋ねます。

| フラグ                         | 説明                                                                                                                                                                                                                                     |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | インストールスコープ: `user`、`project`、または `local`。デフォルトは `user`                                                                                                                                                                                 |
| `--config <key=value>`      | プラグインのマニフェストが宣言する [`userConfig`](/docs/ja/plugins/manifest-reference) オプションを設定します。各オプションについてフラグを繰り返します。Claude Code v2.1.147 以降が必要です                                                                                                         |
| `-y, --yes`                 | `Run this command now?` プロンプトなしで表示されたインストールコマンドを受け入れます。Bash ツールまたはフックからなど、Claude Code セッション内で実行されるコマンドでは無視されます。Claude Code v2.1.229 以降が必要です                                                                                            |
| `--accept-command <sha256>` | 前の [`--json` 実行](#plugin-json-result)が `shownCommand` で報告した `sha256` を持つ表示されたインストールコマンドを受け入れます。`-y` の代わりに使用します。`-y` と組み合わせることはできません。[表示されたインストールコマンドを受け入れる](#accept-a-displayed-install-command)を参照してください。Claude Code v2.1.271 以降が必要です |
| `--json`                    | スクリプトで使用するために、人間が読める形式のメッセージの代わりに、stdout の最後の行に 1 つの JSON オブジェクトとして結果を出力します。[JSON 結果形式](#plugin-json-result)を参照してください。Claude Code v2.1.268 以降が必要です                                                                                     |

自分のターミナルから `-y` を渡して、プロンプトなしで表示されたコマンドを受け入れます。TTY がない場合と Claude がコマンドを実行する場合の動作は次のとおりです：

* **stdin または stdout が TTY ではなく、`-y` も `--accept-command` も渡さない**: インストールが拒否されます。出力はコマンドが表示されたのみであることを示し、終了コードは `1` です
* **Claude が Bash ツール経由でコマンドを実行**: `-y` は無視されます。代わりに自分のターミナルからコマンドを実行してください

プロジェクトをクローンするすべての人のためにプラグインをインストールします：

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code は `Successfully installed plugin: formatter@my-marketplace (scope: project)` を出力します。新しいものがインストールされない場合、出力は理由を説明します：

* **そのスコープで既にインストール**: 出力は `Plugin "formatter@my-marketplace" is already installed (scope: project)` で、終了コードは `0` です
* **コマンドソースプロンプトを拒否**: 出力は `Aborted.` で、終了コードは `1` です
* **`headersHelper` プロンプトを拒否するか、TTY なしで確認できない**: 出力は `Aborted — the command was not run.` で、終了コードは `1` です

<h4 id="plugin-json-result">
  JSON 結果形式
</h4>

`plugin install` に `--json` を渡すと、stdout の最後の行は 1 つの JSON オブジェクトです。Claude Code がそれより前に宣言したコマンドを出力する可能性があるため、その行のみを解析してください。

3 つのフィールドは常に存在します：

* `command`: 実行されたサブコマンド（例：`install`）
* `outcome`: `ok` または `failed`
* `message`: 結果の人間が読める説明

`pluginId`、`scope`、`failureCode` などの他のフィールドは、適用される場合にのみ表示されます。

無効な `--scope` などの使用エラーは、結果行を出力せず、stderr に理由を付けて `1` で終了します。

<h4 id="accept-a-displayed-install-command">
  表示されたインストールコマンドを受け入れる
</h4>

`--json` 実行がマーケットプレイスで宣言されたコマンドを表示し、それを実行しない場合、`failed` 結果は `shownCommand` オブジェクトも含みます。そのフィールドには、表示されたコマンド、それが属するプラグイン、およびコマンドの `sha256` が含まれます。

正確にそのコマンドを受け入れるには、フラグが Claude Code セッション内で効果がないため、自分のターミナルからその `sha256` を `--accept-command` として再実行してください。Claude Code v2.1.271 以降が必要です。

`sha256` は、正確にそのコマンド、プラグイン、およびマーケットプレイスカタログの受け入れとしてカウントされます。実行自体のマーケットプレイス更新が取得する変更を含め、それらのいずれかが変更された場合、Claude Code は `sha256` を受け入れず、コマンドを再度表示します。`shownCommand.acceptCommandMatched` が `false` の場合、渡した `sha256` は現在表示されているコマンドと一致しません。そのコマンドを確認してから、その `sha256` で再実行してください。

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

インストール済みプラグインを 1 つのスコープから削除します。`remove` と `rm` は `uninstall` のエイリアスです。

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| フラグ                   | 説明                                                                                                                                                 |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | スコープからアンインストール: `user`、`project`、または `local`。デフォルトは `user`                                                                                         |
| `--keep-data`         | プラグインの永続データディレクトリ `~/.claude/plugins/data/<id>/` を保持します                                                                                            |
| `--prune`             | 残りのプラグインが必要としない自動インストール[依存関係](/docs/ja/plugins/dependencies)も削除します                                                                                      |
| `-y, --yes`           | `--prune` 確認プロンプトをスキップします。stdin または stdout が TTY でない場合は `--prune` で必須です                                                                            |
| `--json`              | stdout の最後の行に 1 つの JSON オブジェクトを出力します。[`plugin install --json`](#plugin-json-result) と同じ形式です。`--prune` と組み合わせることはできません。Claude Code v2.1.268 以降が必要です |

プロジェクトスコープからプラグインをアンインストールします：

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code は `Successfully uninstalled plugin: formatter (scope: project)` を出力します。プラグインがそのスコープにインストールされていない場合、コマンドは `Failed to uninstall plugin "formatter@my-marketplace":` で始まる行を出力し、`1` で終了します。

<h3 id="plugin-enable">
  plugin enable
</h3>

無効なプラグインを有効にします。[claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins)の場合、プラグインとして `<name>@synced` を渡します。

```bash theme={null}
claude plugin enable <plugin> [options]
```

| フラグ                   | 説明                                                                                                                       |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | 有効にするスコープ: `user`、`project`、または `local`。省略時は自動検出                                                                         |
| `--json`              | stdout の最後の行に 1 つの JSON オブジェクトを出力します。[`plugin install --json`](#plugin-json-result) と同じ形式です。Claude Code v2.1.268 以降が必要です |

`--scope` なしで、コマンドは設定ファイルをローカル、プロジェクト、ユーザーの順序でチェックし、プラグインを言及する最初のスコープを使用します。

プラグインが宣言されていない `--scope` を渡す場合、コマンドはオーバーライドを書き込むか失敗します：

* **宣言するスコープより[優先される](/docs/ja/plugins/loading)スコープ**: Claude Code は渡したスコープでオーバーライドを書き込みます。たとえば、`claude plugin disable formatter --scope local` はプロジェクトで有効なプラグインをあなただけのためにオフにします
* **その他のスコープ**: コマンドは `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.` で失敗します

プラグインが解決されたスコープで既に有効な場合、コマンドは `Plugin "formatter" is already enabled` を出力し、`1` で終了します。`--json` を使用すると、結果は `"failureCode": "already_in_goal_state"` と `"alreadyInGoalState": true` を持つため、スクリプトはそのケースを成功として扱うことができます。

プラグインが[依存関係](/docs/ja/plugins/dependencies)を宣言する場合、Claude Code はそれらも有効にします。コマンドはこれらのケースで失敗します：

* **依存関係がインストールされていない**: 有効化が失敗し、各欠落依存関係に対して `claude plugin install` コマンドを出力します
* **依存関係が組織のプラグインポリシーによってブロックされている**: 有効化が失敗し、ブロックされた依存関係に名前を付けます
* **依存関係がターゲットスコープより優先度の高いスコープで `false` に設定されている**: 有効化が失敗します。そのスコープで依存関係を有効にするか、`--scope` を渡してそこに書き込みます

宣言されている場所でプラグインを再度有効にします：

```bash theme={null}
claude plugin enable formatter
```

Claude Code は `Successfully enabled plugin: formatter (scope: project)` を出力し、検出されたスコープに名前を付けます。

<h3 id="plugin-disable">
  plugin disable
</h3>

プラグインをアンインストールせずに無効にします。[claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins)の場合、プラグインとして `<name>@synced` を渡します。

```bash theme={null}
claude plugin disable [plugin] [options]
```

| フラグ                   | 説明                                                                                                                       |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | すべての有効なプラグインを無効にします。プラグイン名または `--scope` と組み合わせることはできません                                                                  |
| `-s, --scope <scope>` | 無効にするスコープ: `user`、`project`、または `local`。省略時は自動検出                                                                         |
| `--json`              | stdout の最後の行に 1 つの JSON オブジェクトを出力します。[`plugin install --json`](#plugin-json-result) と同じ形式です。Claude Code v2.1.268 以降が必要です |

`--scope` なしで、スコープは [`plugin enable`](#plugin-enable) と同じローカル、プロジェクト、ユーザーの順序で自動検出されます。

プラグイン名も `--all` も渡さない場合、Claude Code は `Please specify a plugin name or use --all to disable all plugins` を出力し、`1` で終了します。既に無効なプラグインを無効にすると、`Plugin "formatter" is already disabled` を出力し、[`plugin enable`](#plugin-enable) が既に有効なプラグインに対して行うのと同様に `1` で終了します。

コマンドは依然として必要なプラグインに対して失敗します：

* **別の有効なプラグインが[それに依存している](/docs/ja/plugins/dependencies)**: コマンドが失敗し、最初に無効にする依存関係に名前を付けます
* **組織がそれを同期プラグインとして要求している**: コマンドが失敗し、何も保存されません

1 つのプラグインを無効にします：

```bash theme={null}
claude plugin disable formatter
```

Claude Code は `Successfully disabled plugin: formatter (scope: project)` を出力します。

<h3 id="plugin-update">
  plugin update
</h3>

プラグインをマーケットプレイスが提供する最新バージョンに更新します。新しいバージョンは次のセッションでロードされるか、実行中のセッションで `/reload-plugins` を実行した後にロードされます。

```bash theme={null}
claude plugin update <plugin> [options]
```

| フラグ                         | 説明                                                                                                                                                                     |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | 更新するスコープ: `user`、`project`、`local`、または `managed`。デフォルトはプラグインがインストールされているスコープ                                                                                           |
| `-y, --yes`                 | [コマンドソース](/docs/ja/plugins/host-marketplace)プラグインから変更されたインストールコマンドをプロンプトなしで受け入れます。stdin または stdout が TTY でない場合は必須です。`--accept-command` を渡さない限り。Claude Code v2.1.229 以降が必要です |
| `--accept-command <sha256>` | 前の [`--json` 実行](#plugin-json-result)が `shownCommand` で報告した `sha256` を持つマーケットプレイスで宣言されたコマンドを受け入れます。`-y` の代わりに使用します。`-y` と組み合わせることはできません。Claude Code v2.1.271 以降が必要です   |
| `--json`                    | stdout の最後の行に 1 つの JSON オブジェクトを出力します。[`plugin install --json`](#plugin-json-result) と同じ形式です。Claude Code v2.1.268 以降が必要です                                               |

`managed` は更新できるが、インストールできない唯一のスコープです。管理者がインストールしたプラグインについては、[組織のプラグインを管理する](/docs/ja/plugins/org)を参照してください。

プラグインを更新します：

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code は `Checking for updates for plugin "formatter@my-marketplace"…` を出力し、その後に結果を出力します。新しいものがない場合、`formatter is already at the latest version (1.0.0).` を出力し、`0` で終了します。

ベアプラグイン名を渡すことができます。コマンドはインストール済みプラグインと照合します。異なるマーケットプレイスからインストール済みプラグインが名前を共有する場合、コマンドは更新を拒否し、実行する修飾 `plugin-name@marketplace-name` コマンドをリストします。ベア名による更新には Claude Code v2.1.246 以降が必要です。

<h3 id="plugin-list">
  plugin list
</h3>

インストール済みプラグインをバージョン、スコープ、ステータスと共にリストします。

```bash theme={null}
claude plugin list [options]
```

| フラグ           | 説明                                                           |
| :------------ | :----------------------------------------------------------- |
| `--json`      | リストを JSON として出力                                              |
| `--available` | マーケットプレイスが提供するがインストールしていないプラグインもリストします。`--json` なしでは効果がありません |

Claude Code は、各プラグインがどのようにロードされるかでグループ化された人間が読める出力を出力します：

* **`Installed plugins:`**: マーケットプレイスからインストールしたプラグイン
* **`Session-only plugins (--plugin-dir / --plugin-url):`**: 同じコマンドでこれらのフラグによってロードされたプラグイン（例：`claude --plugin-dir ./my-plugin plugin list`）
* **`Skills-directory plugins (.claude/skills/*):`**: Claude Code がスキルディレクトリで見つけたプラグイン
* **`Synced from claude.ai`**: [claude.ai アカウントから同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins)

どのグループにも何もない場合、Claude Code は ``No plugins installed. Use `claude plugin install` to install a plugin.`` を出力します。

<h4 id="json-output">
  JSON 出力
</h4>

`--json` を使用すると、Claude Code はインストールごとに 1 つのオブジェクトを持つ配列を出力します。各オブジェクトは以下のフィールドを含みます。`id`、`version`、`scope`、`enabled`、および `installPath` は常に存在し、その他は適用される場合にのみ表示されます。

| フィールド          | 型         | 説明                                                                                                                                                                       |
| :------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | 文字列       | インストールの場合は `name@marketplace`、セッションのみのプラグインの場合は `name@inline`、スキルディレクトリプラグインの場合は `name@skills-dir`、claude.ai から同期されたプラグインの場合は `name@synced`                              |
| `version`      | 文字列       | マーケットプレイスインストールの場合、[Claude Code が計算した](/docs/ja/plugins/loading#versions-and-updates)インストール時のバージョン。セッションのみ、スキルディレクトリ、または同期プラグインの場合、マニフェストの `version`、または宣言されていない場合は `unknown` |
| `scope`        | 文字列       | インストールの場合は `user`、`project`、`local`、または `managed`。スキルディレクトリプラグインの場合は `user` または `project`。セッションのみのプラグインの場合は `session`。claude.ai から同期されたプラグインの場合は `synced`                |
| `enabled`      | ブール値      | マージされた設定でプラグインが有効かどうか                                                                                                                                                    |
| `installPath`  | 文字列       | プラグインがロードされるディレクトリ                                                                                                                                                       |
| `installedAt`  | 文字列       | インストールの ISO タイムスタンプ。マーケットプレイスインストールのみ                                                                                                                                    |
| `lastUpdated`  | 文字列       | 最後の更新の ISO タイムスタンプ。マーケットプレイスインストールのみ                                                                                                                                     |
| `projectPath`  | 文字列       | インストールが属するプロジェクト。`project` および `local` スコープのみ                                                                                                                            |
| `mcpServers`   | オブジェクト    | マーケットプレイスがインストールしたプラグインが持つ場合、プラグインの MCP サーバー定義                                                                                                                           |
| `errors`       | 文字列の配列    | プラグインがロードに失敗した場合のロードエラー                                                                                                                                                  |
| `notes`        | 文字列の配列    | ロードして機能するプラグインのオーサリング警告                                                                                                                                                  |
| `errorDetails` | オブジェクトの配列 | 各 `errors` エントリごとに 1 つのオブジェクト。診断 `type` と、プラグイン、マーケットプレイス、サーバー、またはファイルなど、それが参照する名前を提供します。Claude Code v2.1.268 以降が必要です                                                    |
| `noteDetails`  | オブジェクトの配列 | 各 `notes` エントリの同じ詳細オブジェクト。Claude Code v2.1.268 以降が必要です                                                                                                                   |

`--json --available` を使用すると、Claude Code は配列の代わりに 1 つのオブジェクトを出力します。その `installed` フィールドはインストール済みプラグインオブジェクトの配列を保持し、その `available` フィールドはインストールされていない各マーケットプレイスプラグインを以下のフィールドを持つオブジェクトとして保持します。

| フィールド             | 型            | 説明                                                                                |
| :---------------- | :----------- | :-------------------------------------------------------------------------------- |
| `pluginId`        | 文字列          | `name@marketplace`                                                                |
| `name`            | 文字列          | マーケットプレイスのプラグイン名                                                                  |
| `marketplaceName` | 文字列          | それを提供するマーケットプレイス                                                                  |
| `source`          | 文字列またはオブジェクト | マーケットプレイスエントリの[ソース](/docs/ja/plugins/marketplace-reference)：相対パスの場合は文字列、それ以外の場合はオブジェクト |
| `description`     | 文字列          | エントリの説明（ある場合）                                                                     |
| `version`         | 文字列          | エントリのバージョン（宣言されている場合）                                                             |
| `installCount`    | 数値           | インストール数（Claude Code がプラグインに対して持っている場合）                                            |

<h3 id="plugin-details">
  plugin details
</h3>

プラグインのコンポーネントインベントリと予想トークンコストを表示します。

プラグインはロードされている必要があります：インストール済み、スキルディレクトリで見つかった、または同じコマンドで `--plugin-dir` または `--plugin-url` で渡されました。`<name>` はプラグイン `name` または `name@marketplace` です。

```bash theme={null}
claude plugin details <name>
```

コマンドは `--help` を超えるフラグを取りません。

インストール済みプラグインが何を提供するかを表示します：

```bash theme={null}
claude plugin details formatter
```

Claude Code はプラグインの名前、バージョン、説明、ソースを出力し、その後これらのセクションを出力します：

* **`Component inventory`**: プラグインのスキル、エージェント、フック、MCP サーバー、および LSP サーバー
* **`Projected token cost`**: プラグインがすべてのセッションに追加する常時オンのトークン
* **`Per-component (rounded)`**: 各スキル、エージェント、およびコマンドの常時オンおよびオンインボーク推定。プラグインが何も持たない場合は省略

2 つのコスト数値が何を意味するかについては、[プラグインコストと使用量を測定する](/docs/ja/plugins/measure)を参照してください。

ロードされていないプラグインの場合、Claude Code は ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` を出力し、`1` で終了します。

<h3 id="plugin-prune">
  plugin prune
</h3>

インストール済みプラグインが必要としなくなった自動インストール[依存関係](/docs/ja/plugins/dependencies)を削除します。コマンドは自分でインストールしたプラグインを削除することはありません。`autoremove` は `prune` のエイリアスです。

```bash theme={null}
claude plugin prune [options]
```

| フラグ                   | 説明                                                  |
| :-------------------- | :-------------------------------------------------- |
| `-s, --scope <scope>` | スコープで削除: `user`、`project`、または `local`。デフォルトは `user` |
| `--dry-run`           | 削除せずに削除されるものをリストします                                 |
| `-y, --yes`           | 確認プロンプトをスキップします。stdin または stdout が TTY でない場合は必須です   |

削除が削除するものをプレビューします：

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code は孤立した依存関係をリストし、`(dry run — nothing removed)` で終了します。削除するものがない場合、`Nothing to prune` で始まる行を出力します。

`--dry-run` なしで、コマンドは確認プロンプトで確認するか `-y` を渡した後にのみ孤立した依存関係を削除します。

プロンプトでどのように答えても、終了コードは `0` です。

`prune` が何をするかは、ターミナルが接続されているかどうか、および `-y` を渡すかどうかによって異なります：

| ターミナルとフラグ                      | 何が起こるか                                                                        |
| :----------------------------- | :---------------------------------------------------------------------------- |
| インタラクティブターミナル、`-y` なし          | 孤立した依存関係をリストし、`Remove? [y/N]` と尋ねます                                           |
| 任意のターミナル、`-y`                  | それらを削除し、`Removed N auto-installed plugins: <names>` を出力します                    |
| 非 TTY stdin または stdout、`-y` なし | リストを出力し、``Not a TTY — run `claude plugin prune -y` to remove.`` を出力し、何も削除しません |

<h3 id="plugin-eval">
  plugin eval
</h3>

プラグインの[eval ケース](/docs/ja/plugin-evals)を実行し、スコア付き結果を報告します。Claude Code v2.1.269 以降が必要です。

各ケースはプロンプトとグレーダーです。Claude Code はターゲットプラグインのみがロードされた分離セッションで複数回実行し、デフォルトではプラグインなしでも実行するため、レポートは違いを示します。

ケース形式、グレーダー、結果、および CI 使用については、[eval でプラグインをテストする](/docs/ja/plugin-evals)を参照してください。

```bash theme={null}
claude plugin eval [target] [options]
```

オプションの `target` はデフォルトで現在のディレクトリになり、これらの形式のいずれかを取ります：

* プラグインディレクトリ
* 単一の `prompt.md` または `case.yaml` ファイル
* `name` または `name@marketplace` としてインストール済みプラグイン
* `name@skills-dir`

ターゲットを `--tag`、`--allow-tools`、および `--json` の前に配置します。これらの各オプションは、それに続く単語をその値として取るため、これらのいずれかの後に書かれたターゲットはタグ、ツール名、または JSON 出力パスの代わりにターゲットとして読み取られます。

この表は、ほとんどの実行が使用するオプションをリストします。`--case`、`--tag`、`--output-dir`、`--report`、`--allow-real-servers`、`--keep-temp`、および `--verbose` を含む完全なセットについては、`claude plugin eval --help` を実行してください。

| オプション                      | 説明                                                                                                                                        | デフォルト                                                                     |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| `--runs <n>`               | 各[アーム](/docs/ja/plugin-evals#compare-against-a-no-plugin-baseline)のケースごとの実行                                                                    | 各ケースの `runs`、それ以外は 3                                                      |
| `-j, --concurrency <n>`    | 一度に実行するエージェントセッション、1 から 8。レート制限を共有します                                                                                                     | `1`                                                                       |
| `--model <model>`          | テスト中のエージェントのモデル                                                                                                                           | 各ケースの `model`、それ以外は `ANTHROPIC_MODEL` が設定されている場合、それ以外は Claude Code のデフォルト |
| `--judge-model <model>`    | `llm` および `baseline` グレーダーのモデル                                                                                                            | 小さく高速なモデル                                                                 |
| `--ablation <mode>`        | `none` または `with-without`。[プラグインなしベースラインと比較する](/docs/ja/plugin-evals#compare-against-a-no-plugin-baseline)を参照してください                            | プラグインが解決される場合は `with-without`、それ以外は `none`                                |
| `--threshold <0..1>`       | ケースがこれ以下でスコアされた場合は終了 1                                                                                                                    | `1.0`                                                                     |
| `--max-cost-usd <usd>`     | 支出がこれに達すると次の実行の前に停止し、終了 2、および部分的な結果を報告します                                                                                                 | 制限なし                                                                      |
| `--allow-tools <tools...>` | `Bash`、`Write`、`Edit`、または `"mcp__plugin_<plugin>_<server>__*"` などの読み取り専用セット以外のツールを付与します。[ツールを付与する](/docs/ja/plugin-evals#grant-tools)を参照してください |                                                                           |
| `--scaffold`               | 各ケースの [`scaffold_script`](/docs/ja/plugin-evals#add-setup-or-history-with-case-yaml) を実行                                                       | オフ                                                                        |
| `--trust-plugin`           | 最初の実行信頼プロンプトをスキップします。CI の場合。[実行がアクセスできるもの](/docs/ja/plugin-evals#security)を参照してください                                                            | オフ                                                                        |
| `--mocks <mode>`           | `record` または `off`。[MCP サーバーをモック](/docs/ja/plugin-evals#mock-mcp-servers)を参照してください                                                             | `record`                                                                  |
| `--eval-dir <dir>`         | ケースを保持するプラグイン下のディレクトリ                                                                                                                     | マニフェストの `experimental.evals`、それ以外は `evals`                                |
| `--json [path]`            | [結果ドキュメント](/docs/ja/plugin-evals#json-result)を stdout に出力するか、`.json` パスに書き込みます                                                                 |                                                                           |
| `--no-publish`             | HTML レポートをローカルに保つ                                                                                                                         |                                                                           |

終了コードは実行がどのように終了したかを報告します。パイプラインで機能させるには、[CI で eval を実行する](/docs/ja/plugin-evals#run-evals-in-ci)を参照してください。

| 終了コード | 意味                                    |
| :---- | :------------------------------------ |
| `0`   | すべてのケースがしきい値を満たしている                   |
| `1`   | 失敗するケース、ロードエラー、または信頼されていないプラグインディレクトリ |
| `2`   | 部分的な実行                                |
| `130` | 中断                                    |
| `143` | 終了                                    |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

現在のディレクトリのプラグイン用の eval スイートを作成します。Claude Code v2.1.269 以降が必要です。[最初の eval スイートを作成する](/docs/ja/plugin-evals#create-your-first-eval-suite)を参照してください。

```bash theme={null}
claude plugin eval init [name] [options]
```

ターミナルでは、コマンドはオーサリングインタビューのためのインタラクティブな Claude Code セッションを開きます。インタビューでは、Claude は以下を実行します：

1. プラグインを読む
2. それが何をうまくすべきかを尋ねる
3. ケースとグレーダーを提案する
4. ケースファイルを書く
5. ケースを実行し、グレーダーがあなたがするのと同じ方法でスコアするかどうかを確認するために、あなたと一緒に成績を確認します

`--bare` を使用するか、ターミナルなしで、コマンドは代わりに空白の単一ケーステンプレートを書き込みます。Claude がコマンドを Claude Code セッション内から実行する場合、コマンドはそのセッションが従うべきインタビュー指示を出力します。

オプションの `name` はケース名です。`--bare` を使用するか、ターミナルなしで必須です。コマンドはそのケースの空白テンプレートを書き込むためです。インタビューは 1 つを必要としません。

コマンドはこれらのオプションを受け入れます：

| オプション               | 説明                                                                       | デフォルト                                      |
| :------------------ | :----------------------------------------------------------------------- | :----------------------------------------- |
| `--bare`            | インタビューを実行する代わりに、`<name>` の空白 `prompt.md` と `graders/criteria.md` を書き込みます |                                            |
| `-i, --interactive` | インタビューを要求します。ターミナルなしではテンプレートを書き込む代わりに失敗します                               |                                            |
| `--eval-dir <dir>`  | ケースを書き込む現在のディレクトリ下のディレクトリ                                                | マニフェストの `experimental.evals`、それ以外は `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

プラグインリリース用に `<name>--v<version>` という名前の注釈付き git タグを作成します。タグ付けする前に、コマンドはプラグインの `plugin.json` とそれをリストするマーケットプレイスエントリがバージョンに同意することを確認します。

リリースをタグ付けするタイミングについては、[プラグインを公開する](/docs/ja/plugins/publish)を参照してください。

```bash theme={null}
claude plugin tag [path] [options]
```

`[path]` はプラグインディレクトリで、デフォルトは現在のディレクトリです。コマンドは、そのディレクトリから上へ歩いて、プラグインをリストする `.claude-plugin/marketplace.json` を見つけることで、マーケットプレイスエントリを見つけます。

| フラグ                   | 説明                                                   |
| :-------------------- | :--------------------------------------------------- |
| `--push`              | タグを作成した後、`--remote` にプッシュします                         |
| `--dry-run`           | タグを作成せずに計画を出力します                                     |
| `-f, --force`         | ダーティワーキングツリーとタグ既存チェックをスキップします                        |
| `-m, --message <msg>` | タグ注釈メッセージ。`%s` はバージョンを表します。デフォルトは `<name> <version>` |
| `--remote <name>`     | `--push` でプッシュするリモート。デフォルトは `origin`                 |

マーケットプレイスチェックアウトのプラグインのタグをプレビューします：

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code は計画を出力します：

* プラグイン名
* バージョンとそれがどのファイルから来たか
* マーケットプレイスエントリがある場合、一致するマーケットプレイスエントリ
* タグ名
* 実行する `git tag` および `git push` コマンド

`--dry-run` なしで、Claude Code は `Created tag formatter--v1.0.0` を出力し、`Pushed to origin` または自分で実行するプッシュコマンドを出力します。プッシュが失敗した場合、タグはまだローカルで作成され、コマンドはエラーで終了します。

コマンドは `1` で終了し、安全にタグ付けできない場合は理由を出力します。一般的な理由は：

* `plugin.json` またはマーケットプレイスエントリに `version` がない
* タグが既に存在する
* ワーキングツリーがダーティ

<h3 id="plugin-validate">
  plugin validate
</h3>

プラグインマニフェスト、マーケットプレイスマニフェスト、またはディレクトリ内のスキル、エージェント、およびコマンドを検証し、CI ジョブが機能できるコードで終了します。作成、テスト、編集ワークフローについては、[プラグインの作成](/docs/ja/plugins/create)を参照してください。バリデーターが各マニフェストで何をチェックするかについては、[プラグインマニフェストリファレンス](/docs/ja/plugins/manifest-reference)および[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)を参照してください。

```bash theme={null}
claude plugin validate <path> [options]
```

| フラグ        | 説明                                                                                |
| :--------- | :-------------------------------------------------------------------------------- |
| `--strict` | 警告をエラーとして扱うため、ランタイムが許容する認識されないフィールドと欠落メタデータが実行に失敗します。Claude Code v2.1.145 以降が必要です |
| `--json`   | 検証レポートを同じ終了コードを持つ 1 つの JSON オブジェクトとして出力します。Claude Code v2.1.259 以降が必要です           |

コミット前にプラグインを検証します：

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  ディレクトリを検証する
</h4>

`<path>` はマニフェストファイルまたはディレクトリです。ディレクトリが与えられた場合、Claude Code はそこで見つけたものによって何を検証するかを選択します：

* `.claude-plugin/marketplace.json`（存在する場合）
* それ以外の場合は `.claude-plugin/plugin.json`
* それ以外の場合はコンポーネントファイル。ディレクトリの名前で選択されます。マニフェストなしでコンポーネントファイルを検証するには Claude Code v2.1.233 以降が必要です：
  * `skills`、`agents`、または `commands` という名前のディレクトリ：その中のファイル
  * `.claude` という名前のディレクトリ：その中の `skills`、`agents`、および `commands` ディレクトリ
  * その他のディレクトリ：その `.claude` の下のこれら 3 つのディレクトリ

Claude Code はディレクトリ内のシンボリックリンクをたどりません。リンクがどこにあるかによって異なります：

* **プラグインまたは `.claude` ルートの下にリンクされた `skills`、`agents`、または `commands` ディレクトリ**: Claude Code は何も読み込まれなかったことを警告します。
* **`skills`、`agents`、または `commands` ディレクトリ内のリンクされたエントリ**: Claude Code はそれをスキップし、ディレクトリごとにスキップしたエントリの数を警告します。セッションはロードします。
* **名前を付けた `skills`、`agents`、または `commands` ディレクトリ自体がシンボリックリンク、またはその親 `.claude` ディレクトリ**: Claude Code はエラーを報告し、その中の何もチェックしません。代わりに実際のディレクトリに名前を付けてください。

いくつかのファイルは検証実行では読み込まれません：

* **プラグインルートの `SKILL.md`**: プラグインディレクトリに対して `claude plugin validate` を実行する場合、Claude Code はプラグインルートの `SKILL.md` をチェックしません
* **プラグインルートの `CLAUDE.md`**: プラグイン実行では、Claude Code はプラグインルートの `CLAUDE.md` についても警告します
* **マーケットプレイス実行のプラグインファイル**: マーケットプレイスディレクトリから、Claude Code はプラグインのスキル、エージェント、コマンド、またはフックファイルを開きません。これらのファイルのエラーを見つけるには、各プラグインディレクトリを検証してください

<h4 id="output-and-exit-codes">
  出力と終了コード
</h4>

Claude Code は検証したファイル、エラーと警告とそのパス、および判定行を出力します。終了コードは判定に従います：

| 終了コード | 判定行                                                                              | 意味                                        |
| :---- | :------------------------------------------------------------------------------- | :---------------------------------------- |
| `0`   | `Validation passed` または `Validation passed with warnings`                        | マニフェストがロードされます。`--strict` を使用すると、警告もありません |
| `1`   | `Validation failed` または `Validation failed (--strict treats warnings as errors)` | エラー、または `--strict` の下での警告                 |
| `2`   | `Unexpected error during validation: <reason>`                                   | バリデーター自体が失敗しました。読み取り不可能なパスなど              |

`--json` を使用すると、Claude Code はレポートを stdout に 1 つの JSON オブジェクトとして書き込みます。これらのトップレベルフィールドを持ちます：

* `success`: 終了コードが与える同じ判定
* `strict`: 実行が警告をエラーとして扱ったかどうか
* `target`: Claude Code が検証した解決されたパス
* `manifest`: マニフェスト自体の結果、またはマニフェストなしの実行の場合は `null`
* `contents`: ファイルごとの結果。各結果は `file` に名前を付け、`errors`、`warnings`、および `notes` 配列を含みます

終了 `2` では、コマンドは stdout に何も書き込みません。エラーメッセージは stderr に移動します。

<h2 id="claude-plugin-marketplace-commands">
  claude plugin marketplace コマンド
</h2>

シェルから `claude plugin marketplace <subcommand>` を実行して、プラグインをインストールするマーケットプレイスを追加、リスト、更新、および削除します。

* **終了コード**: これらのサブコマンドはプラグインコマンドの[終了コード規約](#claude-plugin-commands)に従います
* **スコープ**: それらの `--scope` フラグには `-s` 短形式がありません

マーケットプレイスが何であり、Claude Code がそれをキャッシュする方法については、[プラグイン読み込みリファレンス](/docs/ja/plugins/loading)を参照してください。

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

GitHub リポジトリ、git URL、ホストされた `marketplace.json`、またはローカルパスからマーケットプレイスを追加し、設定ファイルで宣言します。

追加した後、Claude Code はインストール済みプラグインが欠落していた[依存関係](/docs/ja/plugins/dependencies)をインストールします。

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| フラグ                   | 説明                                                                                                                |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>`     | マーケットプレイスを宣言する設定ファイル: `user`、`project`、または `local`。デフォルトは `user`                                                  |
| `--sparse <paths...>` | git チェックアウトをこれらのディレクトリに制限します。モノレポの場合。`github` および `git` ソースのみ                                                     |
| `--claudeai`          | 引数を [claude.ai でホストされたマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)の名前として読み取ります。Claude Code v2.1.273 以降が必要です |

`<source>` は以下の表の形式のいずれかを取り、その形式はソースタイプを決定し、Claude Code がマーケットプレイスをどのようにフェッチするかを決定します。結果のソースオブジェクトについては、[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)を参照してください。

| 入力                                                                         | ソースタイプ      | Claude Code がそれをフェッチする方法                                                 |
| :------------------------------------------------------------------------- | :---------- | :----------------------------------------------------------------------- |
| `owner/repo`、`owner/repo#ref`、または `owner/repo@ref`                         | `github`    | GitHub リポジトリをクローンし、与えられた場合は `ref` にピン留めします。所有者とリポは GitHub 命名規則に従う必要があります |
| `user@host:path[.git][#ref]`                                               | `git`       | SSH 経由でクローン                                                              |
| `https://example.com/repo.git[#ref]`、または `/_git/` を含む URL                  | `git`       | Azure DevOps URL を含む HTTPS 経由でクローン                                       |
| `https://github.com/owner/repo` または `https://gitlab.com/namespace/project` | `git`       | `.git` を追加した後、HTTPS 経由でクローン                                              |
| その他の `http://` または `https://` URL（`.git` なしの自己ホストされた git ホストを含む）           | `url`       | URL を `marketplace.json` としてフェッチします。代わりにリポジトリをクローンするには、`.git` を追加します     |
| `./path`、`../path`、`/path`、または `~/path` をディレクトリに                           | `directory` | ディレクトリを所定の位置で読み込みます。Windows では、`.\`、`..\`、および `C:\` 形式も機能します             |
| 同じパス形式を `.json` ファイルに                                                      | `file`      | ファイルを所定の位置で読み込みます                                                        |

`.git` サフィックスを含まないクローン URL を持つホスト（AWS CodeCommit など）の場合、代わりに [`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) にマーケットプレイスを git エントリとして追加してください。Claude Code は URL が `.git` で終わるかどうかに関係なく git エントリをクローンします。

Claude Code はネストされたサブグループを持つ `gitlab.com` URL もクローンします（例：`https://gitlab.com/group/subgroup/project`）。

マーケットプレイスを追加してプロジェクトと共有します：

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code は `Successfully added marketplace: your-marketplace (declared in project settings)` を出力し、マーケットプレイス自体のマニフェストから `name` を使用します。繰り返し追加または無効なソースは、代わりにこれらの結果のいずれかを出力します：

* **マーケットプレイスが既にディスク上にある**: 出力は `Marketplace 'your-marketplace' already on disk — declared in project settings` で、終了コードは `0` です
* **認識されないソース**: 出力は `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` で、終了コードは `1` です
* **`gitlab.example.com/team/plugins` などのベアホスト**: 追加は無効な `owner/repo` 短縮形として失敗し、メッセージは `https://` を追加するか、ローカルパスを使用するよう指示します

[claude.ai でホストされたマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)を `claude plugin marketplace list` の `From claude.ai:` セクションで出力された名前で追加します：

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

`--claudeai` を使用すると、コマンドは `--scope` と `--sparse` を拒否します。マーケットプレイスはアカウント用にホストされ、設定ファイルで宣言されていないため、プロジェクトの `.claude/settings.json` を通じて共有することはできません。

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

追加したすべてのマーケットプレイスをそのソースと共にリストします。

```bash theme={null}
claude plugin marketplace list [options]
```

| フラグ      | 説明              |
| :------- | :-------------- |
| `--json` | リストを JSON として出力 |

Claude Code は `Configured marketplaces:` を出力し、マーケットプレイスごとに 1 つの `Source:` 行を出力するか、`No marketplaces configured` を出力します。

`--json` を使用すると、Claude Code はマーケットプレイスごとに 1 つのオブジェクトを持つ配列を出力し、以下のフィールドを含みます。すべてのフィールドは文字列です。

| フィールド             | 説明                                                     |
| :---------------- | :----------------------------------------------------- |
| `name`            | マーケットプレイスの名前                                           |
| `source`          | `github`、`git`、`url`、`directory`、`file`、または `claudeai` |
| `repo`            | `owner/repo`。`github` ソースのみ                            |
| `url`             | クローンまたはフェッチ URL。`git` および `url` ソースのみ                  |
| `path`            | ローカルパス。`directory` および `file` ソースのみ                    |
| `ref`             | ピン留めされたブランチまたはタグ。`github` および `git` ソース、ピン留めされた場合のみ    |
| `installLocation` | Claude Code がマーケットプレイスをキャッシュした場所                       |

追加された [claude.ai マーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)にはローカルクローンがないため、そのエントリは `installLocation` の代わりに claude.ai 識別子 `marketplaceId` と `organizationUuid` を含みます。また、記録されている場合は `scope` と `status` も含みます。

ターミナルセッションが [claude.ai アカウントからプラグインを同期](/docs/ja/plugins/loading#synced-plugins)する場合、テキストリストは `From claude.ai:` セクションで終了します。そのセクションは、claude.ai が追加していないアカウント用にリストするマーケットプレイスに名前を付けます。git ベースとホストされたの両方です。Claude Code v2.1.273 以降が必要です。

そのセクションからマーケットプレイスを追加するには、[claude.ai からマーケットプレイスを追加する](/docs/ja/plugins/install#add-from-claude-ai)を参照してください。

`--json` 出力は設定されたマーケットプレイスのみをカバーし、セクションを除外します。

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

マーケットプレイスの宣言を設定から削除します。`rm` は `remove` のエイリアスです。

<Warning>
  マーケットプレイスを最後のスコープから削除すると、Claude Code はそのキャッシュも削除し、そこからインストールしたすべてのプラグインをアンインストールします。`--scope` なしで、コマンドはすべてのスコープから宣言を削除します。マーケットプレイスをプラグインを失わずに更新するには、代わりに `plugin marketplace update` を実行してください。
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

`<name>` は `plugin marketplace list` が表示するマーケットプレイス名で、`add` に渡したソースではありません。

| フラグ               | 説明                                                                                  |
| :---------------- | :---------------------------------------------------------------------------------- |
| `--scope <scope>` | 1 つの設定スコープから宣言を削除します: `user`、`project`、または `local`。なしで、Claude Code はすべてのスコープから削除します |

すべてのスコープからマーケットプレイスを削除します：

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code は `Successfully removed marketplace: your-marketplace` を出力し、スコープを設定した場合は `(from project settings)` を追加します。マーケットプレイスを宣言しない設定ファイルにスコープを設定した場合、コマンドは `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.` で失敗します

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

1 つのマーケットプレイス、またはすべてのマーケットプレイスをそのソースから更新して、新しいプラグインとバージョンをフェッチします。ブランチまたはタグ `ref` で追加されたマーケットプレイスは、リポジトリのデフォルトブランチではなく、その ref の最新コミットに更新されます。

```bash theme={null}
claude plugin marketplace update [name]
```

コマンドは `--help` を超えるフラグを取りません。

1 つのマーケットプレイスを更新します：

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code は `Successfully updated marketplace: your-marketplace` を出力します。名前を省略すると、`Successfully updated 2 marketplaces` などのカウントを出力します。マーケットプレイスが追加されていない場合、`No marketplaces configured` を出力し、`0` で終了します。

<h2 id="plugin-in-a-session">
  セッション内の /plugin
</h2>

インタラクティブセッション内では、`/plugin` はプラグインパネルを開きます。各サブコマンドはパネルをタブで開いたり、そこでアクションを実行したり、結果をインラインで出力したりします。`/plugins` と `/marketplace` は `/plugin` のエイリアスです。

これらのコマンドはインタラクティブターミナルセッションでのみ実行できます。`claude -p` などの非インタラクティブ実行では、Claude Code は `/plugin` がこの環境では利用できないと返答します。

どのサーフェスが `/plugin` を持つか、それなしでインストールする方法、および各パネルタブが何を表示するかについては、[プラグインのインストールと管理](/docs/ja/plugins/install)を参照してください。

`<plugin>` はプラグイン `name` または `name@marketplace` です。

以下の表は、すべてのセッション形式を一覧表示しています。シェルサブコマンド `init`、`update`、`details`、`prune`、`eval`、および `eval init` にはセッション形式がありません。

| コマンド                                                | エイリアス                                        | 機能                                                                                                                                                                                                       |
| :-------------------------------------------------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/plugin`                                           |                                              | **Discover** タブでパネルを開きます。`/plugin` の後の認識されない最初の単語も同じことを行います                                                                                                                                              |
| `/plugin help`                                      | `/plugin --help`、`/plugin -h`                | `/plugin` サブコマンドの使用方法リストを表示します                                                                                                                                                                           |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                         | マーケットプレイスにインストールされたプラグインをインラインで出力します。バージョン、スコープ、ステータスが表示されます。フィルターフラグはその状態のみを表示します。有効化状態がまだ適用されていないプラグインは `— /reload-plugins を実行して適用してください` とマークされます。Claude Code v2.1.163 以降が必要です                        |
| `/plugin install`                                   | `i`                                          | **Discover** タブを開きます                                                                                                                                                                                     |
| `/plugin install <plugin>`                          | `i`                                          | **Discover** タブでプラグインの詳細を開きます。`name@marketplace` の場合、そのマーケットプレイスのリストで開きます                                                                                                                                |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                          | `<source>` のマーケットプレイスをまだ追加していない場合は追加し、最初に確認を求めてから、プラグインの詳細を開きます。[マーケットプレイスを追加して 1 つのコマンドでインストール](/docs/ja/plugins/install#add-a-marketplace-and-install-in-one-command)を参照してください。Claude Code v2.1.275 以降が必要です |
| `/plugin manage`                                    |                                              | **Installed** タブを開きます                                                                                                                                                                                    |
| `/plugin stats`                                     |                                              | [`/skill-doctor`](/docs/ja/skills#find-unused-skills) が利用可能なセッションで **Stats** タブを開きます。その他の場所では **Discover** タブでパネルを開きます                                                                                        |
| `/plugin enable <plugin>`                           |                                              | **Installed** タブでプラグインを開いて有効化します                                                                                                                                                                         |
| `/plugin disable <plugin>`                          |                                              | **Installed** タブでプラグインを開いて無効化します                                                                                                                                                                         |
| `/plugin uninstall <plugin>`                        |                                              | **Installed** タブでプラグインを開いてアンインストールします                                                                                                                                                                    |
| `/plugin configure <plugin>`                        | `config`                                     | プラグインの [`userConfig`](/docs/ja/plugins/manifest-reference) ダイアログを開くか、プラグインが宣言していないことを報告します。Claude Code v2.1.147 以降が必要です                                                                                       |
| `/plugin validate <path>`                           |                                              | `claude plugin validate` と同じレポートをインラインで出力します                                                                                                                                                             |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                              | `claude plugin tag` が行うようにリリースタグを作成します。`--push`、`--dry-run`、`--force` または `-f` を受け入れます。その他のフラグまたは追加の引数がある場合、Claude Code は代わりに使用方法を出力します                                                                  |
| `/plugin marketplace`                               | `market`                                     | 目に見える動作はしません。`add`、`list`、`update`、または `remove` を渡してください                                                                                                                                                 |
| `/plugin marketplace add [source]`                  | `market add`                                 | ソースを指定すると、それを追加して結果を報告します。指定しない場合は、**マーケットプレイスを追加** 入力を開きます                                                                                                                                              |
| `/plugin marketplace list`                          | `market list`                                | マーケットプレイス名をインラインで出力します                                                                                                                                                                                   |
| `/plugin marketplace update [name]`                 | `market update`                              | **Marketplaces** タブを開きます。名前を指定すると、そこでそのマーケットプレイスを更新します                                                                                                                                                   |
| `/plugin marketplace remove [name]`                 | `market remove`、`market rm`、`marketplace rm` | **Marketplaces** タブを開きます。名前を指定すると、そこでそのマーケットプレイスを削除します                                                                                                                                                   |

`/plugin enable`、`disable`、`uninstall`、または `configure` で現在のプロジェクトにインストールされていないプラグインを指定した場合、Claude Code はアクションを実行する代わりに `Plugin "<plugin>" is not installed in this project` を出力します。

<h2 id="reload-plugins">
  /reload-plugins
</h2>

実行中のセッションを再起動せずに、保留中のプラグイン変更をセッションに適用します。保留中の変更は、セッション開始以降にディスク上でインストール、更新、有効化、無効化、または編集したプラグインです。

保留中の変更を加えて `/plugin` パネルを閉じると、Claude Code は自動的に `/reload-plugins` を実行します。パネル外で発生するプラグイン変更（別のターミナルで実行した `claude plugin` コマンドなど）の後に自分で実行してください。

```text theme={null}
/reload-plugins [--force]
```

| フラグ       | 説明                                                     |
| :-------- | :----------------------------------------------------- |
| `--force` | プロンプトキャッシュを無効にする場合でも、リロードを適用します。ダッシュなしの `force` も機能します |

<h3 id="reload-summary">
  リロード概要
</h3>

Claude Code はすべてのアクティブなプラグインをリロードし、1 つの概要行 `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers` を出力します。インタラクティブターミナルのないセッションではプラグイン MCP サーバーカウントを省略します。プラグインが失敗した場合、概要は `N errors during load. Run /plugin for details.` を追加します

スキルカウントは、プラグインが提供するすべてのスキル（`commands/` エントリとその `SKILL.md` スキルの両方）をカバーします。エージェントカウントはセッションにロードされたエージェントの数で、プラグインから来ていないものを含みます。

リロードされたプラグインの[依存関係](/docs/ja/plugins/dependencies)が欠落している場合、Claude Code はそれらをインストールし、再度リロードし、概要に `(+ N dependencies: <names>) resolved` を追加します。

<h3 id="reloads-that-change-mcp-tools">
  MCP ツールを変更するリロード
</h3>

リロードがプラグイン MCP サーバーまたは `LSP` ツールを追加または削除し、その変更が[プロンプトキャッシュ](/docs/ja/prompt-caching#enabling-or-disabling-a-plugin)を無効にする場合、Claude Code はリロードを適用しません。`This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` などの行を出力します。`--force` を渡して、とにかく適用してください。

<h3 id="sessions-without-an-interactive-terminal">
  インタラクティブターミナルのないセッション
</h3>

`/reload-plugins` はデスクトップアプリ、Agent SDK、および [非インタラクティブモード](/docs/ja/headless)（`-p` 付き）などのインタラクティブターミナルのないセッションでも実行されます。Claude Code v2.1.260 以降が必要です。

これらのセッションでは、コマンドは `-p` プロンプトまたはデスクトップアプリのプロンプトボックスなど、セッション自体に入力する場合にのみ実行されます。[Remote Control](/docs/ja/remote-control) またはSlack から中継されたメッセージなど、別の方法で到着した場合、コマンドは `/reload-plugins isn't available over a remote connection in this session.` と返信し、何もリロードしません。

これらのセッションのリロードはプラグイン MCP サーバーを接続または切断しません。これらの変更は次のセッションで有効になります。

<h2 id="flags-that-load-a-plugin-for-one-session">
  1 つのセッションのためにプラグインをロードするフラグ
</h2>

2 つの `claude` フラグは、インストールせずに 1 つのセッションのためだけにプラグインをロードします。両方とも繰り返し可能です。

プラグイン作成者はそれらを使用して、公開する前にプラグインをテストします。ロード編集リロードワークフローについては、[マーケットプレイスなしで開発する](/docs/ja/plugins/create#develop-without-a-marketplace)を参照してください。

| フラグ                   | 説明                                                                                                                          | 例                                                                           |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | ディレクトリまたはそのディレクトリの `.zip` アーカイブからプラグインをロードします。プラグインのフォルダは、`.claude-plugin/plugin.json` を保持する各子フォルダをロードします。各フラグは 1 つのパスを取ります | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | URL からプラグイン `.zip` アーカイブをフェッチします。フラグを繰り返すか、1 つの引用符で囲まれた値で複数の URL をスペース区切りで渡します                                              | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

どちらかのフラグがロードするプラグインはセッションのみのプラグインです。`claude plugin list` はそれを `<name>@inline` としてスコープ `session` で表示しますが、同じフラグがサブコマンドの前にある場合のみです。たとえば、`claude --plugin-dir ./my-plugin plugin list` を実行してください。

セッションのみのプラグインがインストール済みプラグインと名前を共有する場合、Claude Code はそのセッションのセッションのみのコピーをロードし、インストール済みのコピーをスキップします。インストール済みのコピーは、`claude plugin disable <name>@inline` でセッションのみのコピーを無効にした場合、または管理設定がそのプラグイン名をロックする場合にロードされます。優先度については、[プラグイン読み込みリファレンス](/docs/ja/plugins/loading)を参照してください。

管理者は、[`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ja/env-vars#variables) 変数で名前が付けられたフォルダを含む両方のフラグを拒否できます。管理設定 [`disableSideloadFlags`](/docs/ja/settings-reference#disablesideloadflags)。Claude Code はその後、フラグが組織の管理設定によって無効化されていることを出力し、起動せずに `1` で終了します。

Agent SDK から、[`plugins`](/docs/ja/agent-sdk/plugins) オプションは `--plugin-dir` と同等です。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインのインストールと管理](/docs/ja/plugins/install): ステップと同じ操作。各ステップで表示される内容
* [プラグイン読み込みリファレンス](/docs/ja/plugins/loading): 各コマンドがディスク上で何を変更し、どのスコープが有効になるか
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting): インストール、マーケットプレイス、ロード、および検証エラーメッセージとその修正
* [プラグインマニフェストリファレンス](/docs/ja/plugins/manifest-reference): `claude plugin validate` がチェックするフィールド
