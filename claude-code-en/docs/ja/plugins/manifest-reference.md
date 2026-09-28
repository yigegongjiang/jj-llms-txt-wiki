> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインマニフェストリファレンス

> plugin.json の完全なリファレンス：すべてのフィールドとその型、デフォルト値、受け入れられるパス形式、userConfig と環境変数スキーマ。

プラグインマニフェストは、プラグインの `.claude-plugin/` ディレクトリにある `plugin.json` ファイルです。プラグインのメタデータと、Claude Code がユーザーに入力を促す [`userConfig`](#user-configuration) 値を含みます。また、インラインで定義するか、[デフォルトの場所](#standard-layout)の外に保持するコンポーネントを宣言します。

このリファレンスはプラグイン作成者向けであり、プラグインのコンポーネントフィールドをマーケットプレイスエントリに配置するマーケットプレイスオーナー向けです。

<Note>
  これらのケースは他のページで説明されています：

  * **プラグイン構築の学習**: [プラグインを作成する](/docs/ja/plugins/create)から始めてください
  * **各コンポーネントが実行時に何をするか**: [プラグインコンポーネント](/docs/ja/plugins/components)を参照してください
</Note>

検索内容に一致するセクションから始めてください：

* フィールド：[フィールドテーブル](#fields)は各フィールドの型、必須かどうか、デフォルト値、受け入れられるものを示します。[パスルール](#path-rules)はすべてのコンポーネントパスの `./` プレフィックスと包含をカバーします
* `userConfig` オプションまたは `channels` エントリ：[ユーザー設定](#user-configuration)と[チャネル](#channels)スキーマ
* `${CLAUDE_PLUGIN_ROOT}` またはプラグインが参照できる別の変数：[環境変数](#environment-variables)
* 各コンポーネントのファイルの場所：[標準レイアウト](#standard-layout)
* `claude plugin validate` からのメッセージ：[トラブルシューティングページ](/docs/ja/plugins/troubleshooting)は各メッセージとその修正、およびこのページの関連セクションへのリンクを一覧表示します

<h2 id="manifest-file">
  マニフェストファイル
</h2>

マニフェストはオプションです。マニフェストがない場合、Claude Code は[標準レイアウト](#standard-layout)で見つかるコンポーネントを読み込みます。その場合、プラグイン名はマーケットプレイスエントリから、または `--plugin-dir` でプラグインを読み込むときはディレクトリ名から取得されます。

メタデータ、デフォルトディレクトリの外のコンポーネント、`userConfig`、またはインラインコンポーネント定義が必要な場合は、マニフェストを作成してください。

マニフェストをプラグインルートの `.claude-plugin/plugin.json` に保存してください。他のすべてのプラグインファイルをプラグインルートに配置し、`.claude-plugin/` の内部には配置しないでください。これには `skills/`、`commands/`、`hooks/` が含まれます。

次の例は[フィールドテーブル](#fields)のほとんどのキーを設定します。参照されるすべてのパスを含むプラグインディレクトリで検証に合格します。

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  認識されないフィールド
</h3>

認識されないトップレベルキーは削除され、`userConfig` オプション、`channels` エントリ、`lspServers` 設定、または `monitors` エントリ内の認識されないキーは拒否されます：

* **トップレベルフィールド**：フィールドは削除され、プラグインは読み込まれます。`claude plugin validate` は認識されないトップレベルフィールドを警告として報告します
* **厳密なオブジェクト**：`userConfig` オプション、`channels` エントリ、`lspServers` 設定、`monitors` エントリは厳密です。その中の未知のキーはエラーであり、プラグインは読み込まれません

<h3 id="validate-the-manifest">
  マニフェストを検証する
</h3>

`claude plugin validate` はマニフェストの権威的なチェックです。シェルからプラグインディレクトリに対して実行してください：

```bash theme={null}
claude plugin validate ./my-plugin
```

コマンドは次のいずれかの結果を報告します：

* **`Validation passed`**：マニフェストが読み込まれます
* **`Validation passed with warnings`**：マニフェストは読み込まれますが、バリデータが修正すべき点を見つけました。例えば、Claude Code が削除する未知のトップレベルフィールド、kebab-case でない `name`、または欠落している `version`、`description`、`author` などです。CI で警告をエラーに変えるには `--strict` を渡してください
* **`Validation failed`**：マニフェストに型の不一致、欠落しているか、プラグインルートを超えるパス、または `userConfig` オプション、`channels` エントリ、`lspServers` 設定、`monitors` エントリ内の未知のキーがあります。Claude Code はプラグインを読み込むときに同じ問題を報告します

<h2 id="fields">
  フィールド
</h2>

テーブルは `plugin.json` のトップレベルキーを一覧表示します。`name` は唯一の必須キーです。フィールド名がリンクの場合、リンク先のセクションに完全なルールがあります。

`commands` や `hooks` などのコンポーネントキーについては、[コンポーネントパス形式](#component-path-forms)は受け入れられる各形式を例とともに示し、すべてのパスは `./` プレフィックス、拡張子、包含の[パスルール](#path-rules)に従います。

| フィールド                                | 型                                | 説明                                                                                                                                                                                                                                        |
| :----------------------------------- | :------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                            | String                           | エディタのオートコンプリート用 JSON Schema URL。Claude Code は読み込み時に無視します                                                                                                                                                                                  |
| [`name`](#name)                      | String                           | プラグイン識別子、必須。kebab-case を使用してください。すべてのコンポーネントはその下に名前空間化されます                                                                                                                                                                                |
| [`displayName`](#displayname)        | String                           | `name` の代わりに UI に表示される名前                                                                                                                                                                                                                  |
| [`version`](#version)                | String                           | バージョン文字列。設定すると、変更するまでユーザーはそのバージョンに留まります                                                                                                                                                                                                   |
| `description`                        | String                           | プラグインが提供するものの簡潔な説明                                                                                                                                                                                                                        |
| `author`                             | Object                           | 必須の `name`、およびオプションの `email` と `url`                                                                                                                                                                                                      |
| `homepage`                           | String                           | ドキュメント URL。URL として解析できない場合、プラグインは読み込みに失敗します                                                                                                                                                                                               |
| `repository`                         | String                           | ソースリポジトリ URL。検証されません                                                                                                                                                                                                                      |
| `license`                            | String                           | `MIT` や `Apache-2.0` などの SPDX 識別子                                                                                                                                                                                                         |
| `keywords`                           | Array of strings                 | 検出タグ                                                                                                                                                                                                                                      |
| [`metadata`](#metadata)              | Object                           | 独自のデータ用の自由形式オブジェクト。Claude Code は読み込みません                                                                                                                                                                                                   |
| [`defaultEnabled`](#defaultenabled)  | Boolean                          | ユーザーが設定していない場合、プラグインが有効な状態で開始するかどうか。デフォルトは `true`                                                                                                                                                                                         |
| [`dependencies`](#dependencies)      | Array of strings or objects      | このプラグインが機能するために有効にする必要があるプラグイン                                                                                                                                                                                                            |
| [`settings`](#settings)              | Object                           | プラグインが有効な間に Claude Code が適用する設定。`agent` と `subagentStatusLine` のみが有効です                                                                                                                                                                    |
| [`userConfig`](#user-configuration)  | Object                           | プラグインが有効な場合に Claude Code がユーザーに入力を促す値                                                                                                                                                                                                     |
| [`channels`](#channels)              | Array of objects                 | プラグインが提供するメッセージチャネル。各チャネルは MCP サーバーの 1 つにバインドされます                                                                                                                                                                                         |
| `skills`                             | Path, or array of paths          | スキルをスキャンするディレクトリ。各ディレクトリは `<name>/SKILL.md` フォルダまたは `SKILL.md` を直接保持する 1 つのフォルダです。`"."` はプラグインルートを指定します。デフォルトの `skills/` スキャンに追加されます                                                                                                      |
| [`commands`](#commands)              | Path, array of paths, or object  | フラットな `.md` コマンドファイル、それらのディレクトリ、またはコマンド名を `source` または `content` にマップするオブジェクト。デフォルトの `commands/` スキャンを置き換えます                                                                                                                              |
| `agents`                             | Path, or array of paths          | エージェント `.md` ファイル。ディレクトリは受け入れられません。デフォルトの `agents/` スキャンを置き換えます                                                                                                                                                                           |
| [`hooks`](#hooks)                    | Path, object, or array of either | `.json` フックファイルまたはインラインフック設定。`hooks/hooks.json` と一緒に読み込まれます                                                                                                                                                                               |
| [`mcpServers`](#mcpservers)          | Path, object, or array of either | `.json` MCP 設定ファイル、`.mcpb` または `.dxt` バンドル、またはインラインサーバー設定（名前でキー化）。`.mcp.json` と一緒に読み込まれます。後で宣言されたサーバー名は前のものを置き換えます                                                                                                                        |
| [`lspServers`](#lspservers)          | Path, object, or array of either | `.json` LSP 設定ファイルまたはインラインサーバー設定（名前でキー化）。`.lsp.json` と一緒に読み込まれます                                                                                                                                                                          |
| `outputStyles`                       | Path, or array of paths          | 出力スタイルファイルまたはディレクトリ。デフォルトの `output-styles/` スキャンを置き換えます                                                                                                                                                                                   |
| `workflows`                          | Path, or array of paths          | [ワークフロー](/docs/ja/workflows#distribute-a-workflow-in-a-plugin) `.js` ファイルまたはディレクトリ。デフォルトの `workflows/` スキャンを置き換えます                                                                                                                             |
| `experimental`                       | Object                           | `themes`、`monitors`、`evals` のコンテナ。マニフェスト形状はまだ変わる可能性があります                                                                                                                                                                                  |
| `experimental.themes`                | Path, or array of paths          | テーマファイルまたはディレクトリ。デフォルトの `themes/` スキャンを置き換えます。トップレベルの `themes` キーはまだ読み込まれ、`claude plugin validate` 警告が表示されます                                                                                                                              |
| [`experimental.monitors`](#monitors) | Path, or inline array            | monitors 配列を保持する `.json` ファイル、または配列自体。デフォルトは `monitors/monitors.json`。トップレベルの `monitors` キーはまだ読み込まれ、`claude plugin validate` 警告が表示されます。モニターはインタラクティブセッションでのみ実行され、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry では実行されません |
| `experimental.evals`                 | Path, or array of paths          | デフォルトが `evals/` でない場合、プラグインの[eval ケース](/docs/ja/plugin-evals#use-a-different-eval-directory)を保持するディレクトリ。`claude plugin eval --eval-dir` はそれをオーバーライドします                                                                                         |

型列では、パスはプラグインルートに相対する文字列です（例：`"./custom/commands"`）。

<h3 id="name">
  `name`
</h3>

プラグイン識別子。空でなく、スペース、`@`、`:`、パス区切り文字、制御文字、双方向フォーマット文字を含まない必要があります。kebab-case を使用してください。

Claude Code はすべてのコンポーネントをその下に名前空間化するため、プラグイン `deploy-tools` のエージェント `reviewer` は `deploy-tools:reviewer` として表示されます。

<h3 id="displayname">
  `displayName`
</h3>

`name` の代わりに UI に表示される名前。スペースと任意の大文字小文字を含むことができ、名前空間化またはルックアップには使用されません。

マーケットプレイスにインストールされたプラグインの場合、[マーケットプレイスエントリ](/docs/ja/plugins/marketplace-reference#plugin-entries)の `displayName` がこの値より優先されます。

<h3 id="version">
  `version`
</h3>

semver に対してチェックされないバージョン文字列。設定すると、変更するまでプラグインはそのバージョンに固定されます。[バージョンと更新](/docs/ja/plugins/loading#versions-and-updates)を参照してください。[`command` ソース](/docs/ja/plugins/marketplace-reference)を持つプラグイン、[claude.ai でホストされているマーケットプレイス](/docs/ja/plugins/install#add-from-claude-ai)からのプラグイン、およびローカルディレクトリとして追加されたマーケットプレイスから[その場で読み込まれた](/docs/ja/plugins/loading#find-plugins-on-disk)プラグインはこのフィールドで固定されません。

<h3 id="metadata">
  `metadata`
</h3>

カタログまたは権利フィールドなど、独自のデータ用の自由形式オブジェクト。Claude Code は読み込みません。Claude Code v2.1.222 以降が必要です。

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

ユーザーが [`enabledPlugins`](/docs/ja/settings-reference#enabledplugins) で設定していない場合、プラグインが有効な状態で開始するかどうか。デフォルトは `true`。有効なプラグインが依存するプラグインは、関係なく有効な状態で開始されます。マーケットプレイスエントリの同じフィールドがこれをオーバーライドします。

ユーザーの `enabledPlugins` エントリが書き込まれると、プラグイン更新全体で保持されるため、後のリリースで `defaultEnabled` を変更しても、既存ユーザーの設定は変わりません。

<h3 id="dependencies">
  `dependencies`
</h3>

このプラグインが機能するために有効にする必要があるプラグイン。各エントリは `"name"`、`"name@marketplace"`、または `{ "name": "...", "marketplace": "...", "version": "..." }` です。ベア名はこのプラグイン独自のマーケットプレイスに対して解決されます。[依存関係の制約](/docs/ja/plugins/dependencies)を参照してください。

<h3 id="settings">
  `settings`
</h3>

プラグインが有効な間に Claude Code が適用する設定。`agent` と `subagentStatusLine` のみが有効です。他のキーは読み込み時に削除されます。プラグインルートの `settings.json` がこのキーより優先されます。[デフォルト設定](/docs/ja/plugins/components#default-settings)を参照してください。

<h2 id="component-path-forms">
  コンポーネントパス形式
</h2>

すべてのコンポーネントキーはプラグインルートに相対するパスを受け入れます。`hooks`、`mcpServers`、`lspServers`、`experimental.monitors` はインライン設定も受け入れ、`commands` はオブジェクトマップも受け入れ、`mcpServers` は MCP バンドルパスと URL も受け入れます。以下の例は受け入れられる各形式を 1 回示します。各コンポーネントが実行時に何をするかについては、[プラグインコンポーネント](/docs/ja/plugins/components)を参照してください。

<h3 id="path-only-fields">
  パスのみのフィールド
</h3>

`agents`、`skills`、`outputStyles`、`workflows`、`experimental.themes` は 1 つのパスまたはパスの配列を受け入れます。`agents` エントリは `.md` ファイルである必要があり、`skills` エントリはディレクトリである必要があります。他の 3 つはディレクトリまたはファイルを受け入れます。

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` はパス、パスの配列、またはオブジェクトマップを受け入れます。パスはフラットな `.md` コマンドファイルまたはディレクトリを指定します。オブジェクトマップでは、各キーはプラグインプレフィックスの後のコマンド名になります。例えば、プラグイン `deploy-tools` の `"about"` は `/deploy-tools:about` として実行されます。

各値は `source` または `content` のいずれか 1 つを設定し、両方を設定するか、どちらも設定しないエントリは検証に失敗します。このテーブルの他のフィールドはオプションです：

| フィールド          | 型                | 説明                                   |
| :------------- | :--------------- | :----------------------------------- |
| `source`       | string           | コマンドの Markdown ファイルへのパス（プラグインルートに相対） |
| `content`      | string           | `source` の代わりにコマンド本体のインライン Markdown  |
| `description`  | string           | コマンドに表示される説明                         |
| `argumentHint` | string           | コマンド名の後に表示される引数ヒント（例：`[file]`）       |
| `model`        | string           | コマンドのデフォルトモデル                        |
| `allowedTools` | array of strings | コマンドがプロンプトなしで使用できるツール                |

このマップはファイルからの 1 つのコマンドとインラインコンテンツからの 1 つを宣言します：

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` は `.json` ファイルパス、[`settings.json` の `hooks`](/docs/ja/hooks#configuration)と同じ形状のインラインフックオブジェクト、またはその両方を混ぜた配列を受け入れます。フックイベントとハンドラーフィールドについては、[フックリファレンス](/docs/ja/hooks#hook-events)を参照してください。

Claude Code は、そのファイルが存在する場合、`hooks/hooks.json` で宣言したものをマージします。

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` は `.json` ファイルパス、MCP バンドルパスまたは URL、インラインマップ、またはそれらを混ぜた配列を受け入れます。サーバー設定フィールドについては、[プラグイン提供の MCP サーバー](/docs/ja/mcp#plugin-provided-mcp-servers)を参照してください。

Claude Code はプラグインルートの `.mcp.json` を最初に読み込み、次に宣言された各形状を順に読み込みます。後で宣言されたサーバー名は前のものを置き換えます。

`mcpServers` 値は次のいずれかの形状を取ります：

| 形状             | 値の例                                                                                    | Claude Code が行うこと                                                   |
| :------------- | :------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| `.json` ファイルパス | `"./mcp/servers.json"`                                                                 | ファイルを `mcpServers` マップとして読み込みます                                     |
| MCP バンドルパス     | `"./bundle.mcpb"`                                                                      | `.mcpb` または `.dxt` バンドルをプラグインルートの `.mcpb-cache/` に抽出し、サーバー設定を読み込みます |
| MCP バンドル URL   | `"https://example.com/server.mcpb"`                                                    | バンドルを `.mcpb-cache/` にダウンロードし、読み込みます                                |
| インラインマップ       | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | マップをサーバー設定（名前でキー化）として使用します                                          |

バンドルパスまたは URL は `.mcpb` または `.dxt` で終わる必要があります。他の拡張子は検証に失敗します。

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` は `.json` ファイルパス、サーバー名から設定へのインラインマップ、またはその両方の配列を受け入れます。

Claude Code はプラグインルートの `.lsp.json` を最初に読み込み、次に宣言された各設定を順に読み込みます。後で宣言されたサーバー名は前のものを置き換えます。

各サーバー設定は、これらのフィールドを持つ厳密なオブジェクトです。未知のキーは検証に失敗します。

| フィールド                   | 必須  | 説明                                                                                                                            |
| :---------------------- | :-- | :---------------------------------------------------------------------------------------------------------------------------- |
| `command`               | Yes | 言語サーバーバイナリ。値が `/` で始まる場合を除き、スペースなし。引数を `args` に入れてください                                                                        |
| `extensionToLanguage`   | Yes | ファイル拡張子から LSP 言語 ID へのマップ。少なくとも 1 つのエントリ。キーは `.go` などのドットで始まります                                                               |
| `args`                  | No  | サーバーに渡される引数                                                                                                                   |
| `transport`             | No  | 通信トランスポート：`stdio`（デフォルト）または `socket`。Claude Code は `socket` を受け入れますが、すべてのサーバーを stdio 上で実行するため、stdout プロトコルルールがすべてのサーバーに適用されます |
| `env`                   | No  | サーバープロセスの環境変数                                                                                                                 |
| `initializationOptions` | No  | initialize リクエストで送信されるオプション                                                                                                   |
| `settings`              | No  | `workspace/didChangeConfiguration` で送信される設定                                                                                   |
| `workspaceFolder`       | No  | サーバーのワークスペースフォルダパス                                                                                                            |
| `startupTimeout`        | No  | スタートアップを待つミリ秒。正の整数                                                                                                            |
| `shutdownTimeout`       | No  | グレースフルシャットダウンを待つミリ秒。正の整数。タイムアウトが経過すると、Claude Code はサーバープロセスを終了します。設定されていない場合、タイムアウトは適用されません                                   |
| `restartOnCrash`        | No  | クラッシュ後にサーバーを再起動するかどうか。デフォルトは `true`。クラッシュしたサーバーを再起動する代わりに停止したままにするには `false` に設定してください                                        |
| `maxRestarts`           | No  | 諦める前の再起動試行。ゼロ以上                                                                                                               |
| `diagnostics`           | No  | 編集後に診断をコンテキストにプッシュするかどうか。デフォルトは `true`                                                                                        |

このインライン設定は `.go` ファイルに対して `gopls` を実行します：

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Anthropic が公開する言語サーバープラグインと、サーバーが実行時にどのように動作するかについては、[コード インテリジェンス](/docs/ja/plugins/code-intelligence)を参照してください。

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` は `.json` ファイルパスまたはインライン配列を受け入れます。キーを省略すると、Claude Code は存在する場合 `monitors/monitors.json` を読み込みます。

各エントリは、これらのフィールドを持つ厳密なオブジェクトです。

| フィールド         | 必須  | 説明                                                                                                            |
| :------------ | :-- | :------------------------------------------------------------------------------------------------------------ |
| `name`        | Yes | プラグイン内で一意の識別子                                                                                                 |
| `command`     | Yes | Claude Code がセッション作業ディレクトリで永続的なバックグラウンドプロセスとして実行するシェルコマンド                                                     |
| `description` | Yes | タスクパネルと通知サマリーに表示される簡潔なサマリー                                                                                    |
| `when`        | No  | `"always"`（デフォルト）の場合、モニターはセッション開始時とプラグイン再読み込み時に開始されます。`"on-skill-invoke:<skill>"` の場合、そのスキルが初めて実行されるときに開始されます |

このインライン配列は、`deploy` スキルが初めて実行されるときに開始される 1 つのモニターを宣言します：

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

モニター `command` は `${user_config.*}` を参照できません。[シェルを通じて実行されるフィールド](#fields-that-run-through-a-shell)を参照してください。

<h2 id="path-rules">
  パスルール
</h2>

マニフェスト内のすべてのコンポーネントパスはプラグインルートに相対し、`./` で始まる必要があります。`commands/foo.md` などのパスは検証に失敗します。`skills` と `mcpServers` は各々、そのルール外の 1 つの形式を受け入れます：

* **`skills`**：`"."` も受け入れます。`"."` と `"./"` の両方がプラグインルートを示します。v2.1.221 より前では、`"."` はマニフェスト検証に失敗したため、プラグインが以前のバージョンで読み込まれる必要がある場合は `"./"` を使用してください
* **`mcpServers`**：`https://` バンドル URL も受け入れます

<h3 id="containment-and-existence">
  包含と存在
</h3>

すべてのコンポーネントパスはプラグインルート内で解決され、存在する必要があります。`claude plugin validate` は `outputStyles`、`lspServers`、`monitors`、`themes` パスをチェックしないため、これらのフィールドの不正なパスはプラグインが読み込まれるときにのみ失敗します：

* **包含**：プラグインルート外で解決されるパスは読み込まれず、`/plugin` **Errors** タブに `<component> path escapes plugin directory: <path>` が表示されます。`..` を含むパスが一般的なケースであり、`claude plugin validate` は `Path contains ".." which could be a path traversal attempt` として報告します
* **存在**：存在しないパスは読み込まれず、`/plugin` **Errors** タブに `<component> path not found: <path>` が表示されます。`claude plugin validate` は `Path not found` として報告します

<h3 id="how-each-key-combines-with-its-default-location">
  各キーがデフォルトの場所とどのように組み合わされるか
</h3>

各コンポーネントキーは、デフォルトの場所を置き換えるか、追加するか、またはマージします：

* **デフォルトを置き換える**：`commands`、`agents`、`outputStyles`、`workflows`、`experimental.themes`、`experimental.monitors`。`commands` を設定すると、デフォルトの `commands/` ディレクトリはスキャンされません。デフォルトを保持して追加するには、明示的にリストします：`"commands": ["./commands/", "./extras/"]`
* **デフォルトに追加**：`skills`。`skills/` ディレクトリはまだスキャンされ、リストされたディレクトリはそれと一緒に読み込まれます
* **マージ**：`hooks`、`mcpServers`、`lspServers`。デフォルトファイルが最初に読み込まれ、マニフェストが宣言したものはそれにマージされます。[コンポーネントパス形式](#component-path-forms)で説明されているとおりです

プラグインが `commands/` などのデフォルトフォルダを持ち、それを置き換えるマニフェストキーも設定している場合、Claude Code はマニフェストパスを読み込み、フォルダは読み込みません。`claude plugin list` と `/plugin` インターフェイスは警告 `Default <folder>/ folder is ignored because the manifest sets "<key>"` を表示します。

警告を避けるには、キーをそのフォルダ内のパスに設定してください：`"commands": ["./commands/deploy.md"]` はデフォルトフォルダ内のファイルを指定し、警告は生成されません。

<h2 id="user-configuration">
  ユーザー設定
</h2>

`userConfig` は、プラグインが有効な場合に Claude Code がユーザーに入力を促す値を宣言するため、ユーザーは `settings.json` を自分で編集する必要がありません。

キーは文字、数字、アンダースコアで構成される識別子であり、数字で始まることはできません。

各値は、これらのフィールドを持つ厳密なオブジェクトです。未知のキーは検証に失敗します。

| フィールド         | 必須  | 説明                                                                                                                                             |
| :------------ | :-- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Yes | `string`、`number`、`boolean`、`directory`、`file` のいずれか                                                                                           |
| `title`       | Yes | 設定ダイアログに表示されるラベル                                                                                                                               |
| `description` | Yes | フィールドの下に表示されるヘルプテキスト                                                                                                                           |
| `required`    | No  | `true` の場合、設定ダイアログは空の値を受け入れません                                                                                                                 |
| `default`     | No  | ユーザーが何も提供しない場合に使用される値：文字列、数字、ブール値、または文字列の配列                                                                                                    |
| `options`     | No  | `string` の場合、フィールドが受け入れる値。`/config` でピッカーとして表示されます。[フィールドを固定オプションに制限する](#limit-a-field-to-fixed-options)を参照してください。Claude Code v2.1.271 以降が必要です |
| `multiple`    | No  | `string` の場合、文字列の配列を許可します                                                                                                                      |
| `sensitive`   | No  | `true` の場合、入力をマスクし、`settings.json` の代わりにセキュアストレージに値を保存します                                                                                      |
| `min` / `max` | No  | `number` の境界                                                                                                                                   |

各有効なプラグインの各オプションは `/config` パネルの行としても表示されます。ただし、`sensitive` オプションと `multiple` リストは除きます。`/config` 行には Claude Code v2.1.269 以降が必要です。

この `userConfig` はエンドポイントとマスクされたトークンを宣言します：

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  フィールドを固定オプションに制限する
</h3>

`userConfig` フィールドに `options` を設定して、ユーザーが固定リストからその値を選択するようにします。

`tone` フィールドを 3 つのオプションに制限するには、`options` にリストし、`default` をそのいずれかに設定します：

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

任意のフィールドで `options` を宣言する場合、Claude Code v2.1.271 より前のバージョンのユーザーはプラグインを読み込むことができません。

`options` は `multiple` または `sensitive` でない `string` フィールドに適用されます。`default` をリストされた値の 1 つに設定するか、ユーザーが 1 つを選択する必要があるように `required: true` を設定してください。各オプションは 1 ～ 64 文字のプレーンラベルであり、シェルで実行する `claude plugin validate` は他に拒否するものを報告します。`options` がこれらのルールを破るプラグインは読み込みに失敗します。

<h3 id="where-values-are-stored">
  値が保存される場所
</h3>

機密でない値はユーザーの `settings.json` の [`pluginConfigs`](/docs/ja/settings-reference#pluginconfigs) に保存されます。機密値はプラットフォームのセキュアな認証情報ストアに代わりに保存されます。[設定ページ](/docs/ja/settings-reference#pluginconfigs)は `pluginConfigs` が読み込まれる設定ファイルを一覧表示します。

<h3 id="reference-a-saved-value">
  保存された値を参照する
</h3>

プラグインが必要とする場所で保存された値を参照します。次の 2 つの形式のいずれかで：

* **`${user_config.KEY}`**：MCP サーバー設定、LSP サーバー設定、[exec 形式](/docs/ja/hooks#exec-form-and-shell-form)フック `args`、スキルおよびエージェントコンテンツで置き換えられます。スキルおよびエージェントコンテンツでは、機密でない値のみが置き換えられ、機密値はプレースホルダーになります
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**：すべてのオプションに対してフックプロセスにエクスポートされます。`<KEY>` は大文字です。シェル形式フックは `api_token` に対して `$CLAUDE_PLUGIN_OPTION_API_TOKEN` を読み込みます

<h3 id="fields-that-run-through-a-shell">
  シェルを通じて実行されるフィールド
</h3>

シェル形式フックコマンド、モニターコマンド、MCP [`headersHelper`](/docs/ja/mcp#use-dynamic-headers-for-custom-authentication) は `${user_config.*}` を拒否します。これらのフィールドの 1 つで参照するコンポーネントは、フィールドの値がシェルに渡され、置き換えられた値を再解析するため、実行する代わりに[エラー](/docs/ja/errors#plugin-command-references-user-config)で失敗します。

テーブルは、値がこれらの各フィールドに到達する方法を示します。

| フィールド               | 値がそこに到達する方法                                                                                                                                                      |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| シェル形式フックコマンド        | [exec 形式](/docs/ja/hooks#exec-form-and-shell-form)を `args` で使用するか、フックの環境から `CLAUDE_PLUGIN_OPTION_<KEY>` を読み込みます                                                       |
| モニターコマンド            | Claude Code を通じてではありません。モニタープロセスは `CLAUDE_PLUGIN_OPTION_<KEY>` を受け取らないため、モニタースクリプトは値を独自に取得する必要があります                                                              |
| MCP `headersHelper` | Claude Code を通じてではありません。ヘルパーの環境は `CLAUDE_PLUGIN_ROOT`、`CLAUDE_CODE_MCP_SERVER_NAME`、`CLAUDE_CODE_MCP_SERVER_URL` を含みますが、オプション値は含まないため、ヘルパースクリプトは値を独自に取得する必要があります |

<h2 id="channels">
  チャネル
</h2>

`channels` はプラグインが提供するメッセージチャネル（チャットアプリへのブリッジなど）を宣言します。1 つを宣言すると、Claude Code はプラグインが有効な場合にチャネルの設定を入力するよう促すことができます。サーバーがメッセージを注入する方法については、[チャネルリファレンス](/docs/ja/channels-reference#package-as-a-plugin)を参照してください。

各エントリは、プラグインの MCP サーバーの 1 つにバインドされた厳密なオブジェクトであり、これらのフィールドを持ちます：

| フィールド         | 必須  | 説明                                                        |
| :------------ | :-- | :-------------------------------------------------------- |
| `server`      | Yes | チャネルがバインドされるこのプラグインの `mcpServers` の MCP サーバーのキー           |
| `displayName` | No  | 設定ダイアログのタイトルに表示される名前。デフォルトはサーバー名                          |
| `userConfig`  | No  | 入力するオプション。[トップレベル `userConfig`](#user-configuration)と同じ形状 |

このマニフェストはチャネルをプラグインの `telegram` MCP サーバーにバインドし、サーバーの `env` に置き換わるボットトークンを入力するよう促します：

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  環境変数
</h2>

Claude Code は 3 つのパス変数をプラグインコンポーネントに提供します。[各変数が解決される場所](#where-each-variable-resolves)にリストされたフィールドで `${NAME}` として参照し、それらを受け取るプロセスで環境変数として読み込みます。

| 変数                      | 解決先                                                                                                                  | 用途                                            |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | プラグインのインストール済みバージョンの絶対パス                                                                                             | プラグインにバンドルされたスクリプト、バイナリ、設定ファイル                |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`。最初の参照時に作成され、プラグイン更新全体で保持されます。`<id>` はプラグイン識別子で、文字、数字、`_`、`-` 以外のすべての文字が `-` に置き換えられます | `node_modules` などのインストール済み依存関係、生成されたコード、キャッシュ |
| `${CLAUDE_PROJECT_DIR}` | プロジェクトルート                                                                                                            | プロジェクトローカルスクリプトと設定ファイル                        |

`${CLAUDE_PLUGIN_ROOT}` はプラグインが更新されるときに変わるため、そこに状態を書き込まないでください。ルートが移動する場合と古いディレクトリがクリーンアップされる場合については、[読み込みページ](/docs/ja/plugins/loading)を参照してください。

最後にインストールされた場所からプラグインをアンインストールすると、[`--keep-data`](/docs/ja/plugins/cli-reference) を渡さない限り、`${CLAUDE_PLUGIN_DATA}` ディレクトリは削除されます。

<h3 id="where-each-variable-resolves">
  各変数が解決される場所
</h3>

各プラグインコンポーネントでは、`${...}` 参照は特定のフィールドでインラインで解決され、一部のコンポーネントはプロセス環境でも変数を受け取ります：

| プラグインコンポーネント               | `${...}` が解決されるフィールド                     | プロセスにエクスポートされます                                                                             |
| :------------------------- | :--------------------------------------- | :------------------------------------------------------------------------------------------ |
| フックコマンド                    | `command` と `args` の任意の場所                | `CLAUDE_PLUGIN_ROOT`、`CLAUDE_PLUGIN_DATA`、`CLAUDE_PROJECT_DIR`、`CLAUDE_PLUGIN_OPTION_<KEY>` |
| モニターコマンド                   | `command` の任意の場所                         | エクスポートされません                                                                                 |
| MCP `stdio` サーバー           | `command`、`args`、`env`                   | `CLAUDE_PLUGIN_ROOT`、`CLAUDE_PLUGIN_DATA`                                                   |
| MCP `http`、`sse`、`ws` サーバー | `url`、`headers`、`headersHelper`          | 適用されません                                                                                     |
| LSP サーバー                   | `command`、`args`、`env`、`workspaceFolder` | `CLAUDE_PLUGIN_ROOT`、`CLAUDE_PLUGIN_DATA`、`CLAUDE_PROJECT_DIR`                              |
| スキル、コマンド、エージェントコンテンツ       | Markdown 本体の任意の場所                        | 適用されません                                                                                     |

変数は、Bash ツールを通じて Claude が実行するコマンドの環境、メインセッション、またはサブエージェントに存在しません。スキル、コマンド、エージェントコンテンツでは、Markdown 本体に `${...}` 参照を書き込み、Claude Code はコンテンツを読み込むときにパスをインラインで置き換えます。

<h3 id="quoting-and-path-separators">
  クォートとパス区切り文字
</h3>

置き換えられた各パスを 1 つの引数に保ちます：

* **フックコマンド**：[exec 形式](/docs/ja/hooks#exec-form-and-shell-form)を `args` で使用して、各パスが 1 つの引数でクォートなしになるようにします
* **シェル形式フックとモニターコマンド**：変数をダブルクォートで囲んで、スペースを含むパスが 1 つの単語のままになるようにします

このシェル形式フックはプラグインにバンドルされたスクリプトを実行します：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

Windows では、置き換えられたパスはフォワードスラッシュを使用するため、シェルはバックスラッシュをエスケープとして読み込みません。

<h2 id="standard-layout">
  標準レイアウト
</h2>

各コンポーネントタイプには、マニフェストが別の場所を指さない場合に使用されるプラグインルート下のデフォルトの場所があります。

| コンポーネント  | デフォルトの場所                     | コンテンツ                                                                                                                                                                                                                        |
| :------- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| マニフェスト   | `.claude-plugin/plugin.json` | プラグインメタデータと設定。オプション                                                                                                                                                                                                          |
| スキル      | `skills/`                    | スキルごとに 1 つの `<name>/SKILL.md`。`SKILL.md` がルートにあり、`skills/` がなく、`skills` キーがないプラグインは単一スキルとして読み込まれます                                                                                                                           |
| コマンド     | `commands/`                  | フラットな Markdown コマンドファイル。新しいプラグインでは `skills/` を推奨します                                                                                                                                                                          |
| エージェント   | `agents/`                    | エージェント Markdown ファイル。サブフォルダは[エージェント名](/docs/ja/plugins/components#agents)の一部です                                                                                                                                                    |
| フック      | `hooks/hooks.json`           | フック設定                                                                                                                                                                                                                        |
| MCP サーバー | `.mcp.json`                  | MCP サーバー定義                                                                                                                                                                                                                   |
| LSP サーバー | `.lsp.json`                  | LSP サーバー設定                                                                                                                                                                                                                   |
| 出力スタイル   | `output-styles/`             | 出力スタイル Markdown ファイル                                                                                                                                                                                                         |
| ワークフロー   | `workflows/`                 | ワークフロー `.js` ファイル                                                                                                                                                                                                            |
| テーマ      | `themes/`                    | テーマ JSON ファイル                                                                                                                                                                                                                |
| モニター     | `monitors/monitors.json`     | monitors 配列                                                                                                                                                                                                                  |
| 実行可能ファイル | `bin/`                       | ここのファイルはプラグインが有効な間、Bash ツールの `PATH` 上にあるため、Claude はそれらをベアコマンドとして実行します。claude.ai と Cowork は、このディレクトリを持つプラグイン（[claude.ai 組織設定を通じて配布する](/docs/ja/plugins/host-marketplace#distribute-through-organization-settings)ものを含む）をインストールしません |
| 設定       | `settings.json`              | プラグインが有効な間に適用される `agent` と `subagentStatusLine` のデフォルト                                                                                                                                                                       |

すべてのデフォルトの場所を使用し、フックが呼び出す `scripts/` フォルダを持つプラグインは、次のようにレイアウトされます：

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

このレイアウトをクリックして、各ファイルが何をするかを読むには、[プラグインエクスプローラー](/docs/ja/plugins/components#explore-the-plugin-directory)を開いてください。

プラグインルートの `CLAUDE.md` はコンテキストとして読み込まれず、`claude plugin validate` はそれを見つけると警告します。Claude のコンテキストに読み込まれる指示を含めるには、スキルに入れてください。

<h2 id="marketplace-entries-and-the-manifest">
  マーケットプレイスエントリとマニフェスト
</h2>

[マーケットプレイスエントリ](/docs/ja/plugins/marketplace-reference)は、このページのすべてのフィールドを[独自のフィールド](/docs/ja/plugins/marketplace-reference#plugin-entries)と一緒に受け入れます。`strict` を含みます。

`strict` フィールドは、エントリが独自の `plugin.json` を持つプラグインにコンポーネントを追加できるかどうかを決定します。デフォルトは `true` です。

<h3 id="how-entry-fields-combine-with-plugin-json">
  エントリフィールドが `plugin.json` とどのように組み合わされるか
</h3>

エントリはマニフェストとして機能するか、コンポーネントを追加するか、またはそれと競合します：

* **`plugin.json` なし**：エントリはマニフェストです。`strict` に関係なく。エントリ `hooks` はインラインオブジェクト形式でのみ読み込まれます。ファイルパスまたは配列の場合、`/plugin` **Errors** タブに `not yet supported in a marketplace entry` エラーが表示されます
* **`plugin.json` 存在、`strict` 未設定または `true`**：Claude Code はマニフェストを読み込み、エントリの `commands`、`agents`、`skills`、`outputStyles`、`themes` をそれに追加します。`hooks` の場合、エントリのイベントのマッチャーはマニフェストの同じイベントのマッチャーを置き換え、マニフェストのみが宣言するイベントはそのマッチャーを保持します
* **`plugin.json` 存在、`strict: false`**：`commands`、`agents`、`skills`、`hooks`、`outputStyles`、`themes` のいずれかを宣言するエントリは競合であり、プラグインは `Plugin <name> has conflicting manifests` で読み込みに失敗します

[ソースがマーケットプレイスルートであるマーケットプレイスエントリ](/docs/ja/plugins/marketplace-reference)が特定の `skills` サブディレクトリをリストする場合、それらのサブディレクトリのみが読み込まれ、プラグインのデフォルト `skills/` ディレクトリはスキャンされません。マニフェストの `skills` キーは代わりに[デフォルトに追加](#how-each-key-combines-with-its-default-location)されます。

<h3 id="metadata-precedence">
  メタデータの優先順位
</h3>

一部のメタデータフィールドは `strict` に関係なく固定の優先順位を持ちます：

* **`defaultEnabled` と表示フィールド**：エントリの `defaultEnabled` とその[表示フィールド](/docs/ja/plugins/marketplace-reference#entry-and-plugin-json)（`displayName` など）はマニフェストのものをオーバーライドします
* **`version`**：マニフェストの `version` はエントリのものをオーバーライドします
* **`name`**：エントリがプラグインをマニフェストとは異なる `name` でリストする場合、`enabledPlugins` はエントリ名を使用し、コンポーネントはマニフェスト名の下に名前空間化されます

完全な優先順位テーブルについては、[厳密モード](/docs/ja/plugins/marketplace-reference)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインにコンポーネントを追加する](/docs/ja/plugins/components)：各コンポーネントが実行時に何をするか。検証する例付き
* [マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)：マーケットプレイスがプラグインに設定できるエントリフィールド
* [プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference#plugin-validate)：`claude plugin validate` フラグと出力
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting#claude-plugin-validate-reports-errors)：各検証メッセージとその修正
