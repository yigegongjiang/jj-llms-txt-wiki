> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 組織向けプラグインを推奨する

> マーケットプレイスプラグインエントリに関連性ブロックを追加して、ユーザーの作業が一致するときに Claude Code が推奨するようにし、マネージド設定でマーケットプレイスをホワイトリストに登録します。

Claude Code は、ユーザーのセッションが定義したシグナルと一致するときに、組織のマーケットプレイスからプラグインをインストールすることを提案できます。シグナルには、作業ディレクトリ、Claude が読み取ったファイル、および Claude が実行したコマンドが含まれます。これらは、プラグインの `marketplace.json` エントリに `relevance` ブロックを追加することで定義します。

マーケットプレイスオペレーターが `relevance` エントリを記述します。その後、管理者がマネージド設定でマーケットプレイスをホワイトリストに登録します。マーケットプレイスがホワイトリストに登録されるまで、ユーザーはそのマーケットプレイスからの提案を表示しません。

<Note>
  これらのケースは他のページで説明されています。

  * **プラグインをインストールしたい場合**: [プラグインのインストールと管理](/docs/ja/plugins/install)を参照してください
  * **提案をオフにしたい場合**: [プラグイン関連性の仕組みを理解する](#understand-how-plugin-relevance-works)を参照してください
</Note>

自分の役割に応じたセクションから始めてください。

* **マーケットプレイスオペレーター**: [提案の仕組み](#understand-how-plugin-relevance-works)を読んでから、[プラグインエントリに関連性を追加](#add-relevance-to-a-plugin-entry)し、[マーケットプレイスを検証](#validate-your-marketplace)してください
* **管理者**: [マネージド設定で提案を有効にする](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  プラグイン関連性の仕組みを理解する
</h2>

`marketplace.json` の各プラグインエントリには、`relevance` オブジェクトを含めることができます。このオブジェクトはトピックと 1 つ以上のシグナルを指定します。シグナルは、作業ディレクトリやClaudeが読み取ったファイルなど、Claude Code が現在のセッションに対してテストするパターンです。

シグナルマッチングはユーザーのマシン上でローカルに実行され、ネットワークトラフィックは追加されません。Claude Code は、どのシグナルが一致したか、またはそれらの値を Anthropic またはマーケットプレイス運営者に報告しません。

シグナルが一致し、プラグインがまだインストールされていない場合、Claude Code は以下の場所でプラグインを提案します。

* **スピナーチップ**: Claude が応答している間、スピナーの下に `/plugin install` コマンドを含むメッセージが表示されます。
* **セッション開始通知**: `cwd` シグナルが作業ディレクトリと一致する場合、ユーザーが最初のメッセージを送信する前に 1 行の通知が表示されます。
* **`/plugin` Discover タブ**: プラグインは Discover リストの上部にピン留めされます。

[ユーザーが見るものをプレビューする](#preview-what-the-user-sees)は、各テキストの正確なテキストと、それらがどのくらいの頻度で繰り返されるかを示しています。

Claude Code はプラグインを自動的にインストールすることはありません。ユーザーは常に確認します。

スピナーチップとセッション開始通知の両方は、ユーザーまたはプロジェクトが [`spinnerTipsEnabled`](/docs/ja/settings-reference#spinnertipsenabled) を `false` に設定するか、`excludeDefault` を含む [`spinnerTipsOverride`](/docs/ja/settings-reference#spinnertipsoverride) が組み込みチップを置き換える場合に表示されなくなります。Discover タブのピンはどちらの設定にも影響されません。

<h2 id="add-relevance-to-a-plugin-entry">
  プラグインエントリに関連性を追加する
</h2>

`marketplace.json` のプラグインエントリに `relevance` オブジェクトを追加します。次の例は、Claude が `.tf` ファイルを読み取るか `terraform` を実行するときに `terraform-helpers` プラグインが関連していることを宣言しています。

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

シグナルが一致しない場合、プラグインは Discover リストで通常の位置を保持し、スピナーチップとして表示されません。

公開する前にブロックを確認するには、[マーケットプレイスを検証](#validate-your-marketplace)してください。

<h2 id="field-reference">
  フィールドリファレンス
</h2>

`relevance` オブジェクトとそのネストされた `signals` オブジェクトは、次の表のフィールドを受け入れます。

古いクライアントは、認識しない `relevance` フィールドを使用するマーケットプレイスをまだロードします。これは、`relevance` と `relevance.signals` の下の未知のフィールドはロード時に無視されるためです。認識されたフィールドの値が[フィールドリファレンス](#field-reference)の制限を超える場合、プラグインエントリ全体が無効になり、ユーザーはそのプラグインをマーケットプレイスからインストールできなくなります。修正するまで、`claude plugin validate` は同じ制限を報告します。

<h3 id="relevance">
  `relevance`
</h3>

| フィールド     | 型      | 説明                                                                                                                              |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------ |
| `topic`   | 文字列    | オプション。スピナーチップの「Working with *topic*?」を埋める句。デフォルトはプラグイン名で、各ハイフンセグメントが大文字化されます。最大 64 文字。                                          |
| `signals` | オブジェクト | プラグインが関連するときを決定するマッチャー。Claude Code は、少なくとも 1 つのシグナルが設定されている場合にのみプラグインを提案します。[`relevance.signals`](#relevance-signals)を参照してください。 |

`topic` は多くの場合、製品名（例：`Terraform`）です。プラグイン名がトピックとして自然に聞こえない場合は、`design` などのドメインを使用してください。

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

`signals` オブジェクトは、次のフィールドを受け入れます。

| フィールド          | 型         | 説明                                                                                                                                                               | 制限                                                   |
| :------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `cwd`          | 文字列の配列    | セッションの作業ディレクトリに対してマッチするグロブパターン。[作業ディレクトリマッチング](#working-directory-matching)を参照してください。                                                                            | 256 文字以下の 10 パターン                                    |
| `cli`          | 文字列の配列    | Claude がこのセッションで実行したシェルコマンドからのコマンド名（例：`["terraform"]`）。完全一致。[コマンド名マッチング](#command-name-matching)を参照してください。                                                       | 64 文字以下の 10 エントリ                                     |
| `hosts`        | 文字列の配列    | このセッションの Bash コマンドの `http://` または `https://` URL に表示されるホスト名（例：`["registry.terraform.io"]`）。ベアの小文字ホスト名のみ：スキーム、ポート、またはパスなし。完全な大文字と小文字を区別しないマッチング。                  | 128 文字以下の 20 エントリ                                    |
| `filesRead`    | 文字列の配列    | Claude がこのセッションで読み取ったファイルのパスに対してマッチするグロブパターン（例：`["**/*.tf"]`）。フォワードスラッシュで正規化され、大文字と小文字を区別しません。                                                                   | 256 文字以下の 10 パターン                                    |
| `manifestDeps` | オブジェクトの配列 | Claude がこのセッションで読み取ったパッケージマニフェストで宣言された依存関係。各エントリは `{ "file": "...", "pattern": "..." }` で、両方の値は正規表現です。[マニフェスト依存関係マッチング](#manifest-dependency-matching)を参照してください。 | 10 エントリ、各値は最大 256 文字。512 KB より大きいマニフェストファイルはスキップされます |

`filesRead` と `manifestDeps` シグナルは、Claude がこのセッションで書き込みまたは編集したファイル、およびプロジェクトの自動ロードされた `CLAUDE.md` メモリファイルに対してもマッチします。

<h4 id="working-directory-matching">
  作業ディレクトリマッチング
</h4>

`cwd` は、セッション開始時、ユーザーが最初のメッセージを送信する前にマッチできる唯一のシグナルです。

Claude Code は各 `cwd` パターンを次のようにマッチします。

* パターンは、作業ディレクトリを絶対パスとしてマッチします。セッションが git リポジトリ内にある場合、リポジトリルートに対する作業ディレクトリのパスに対してもマッチします。
* マッチングはフォワードスラッシュで正規化され、大文字と小文字を区別しません。
* すべてのパターンはディレクトリ自体とその下のすべてにマッチするため、`infra`、`infra/`、および `infra/**` は同じように動作します。

<h4 id="command-name-matching">
  コマンド名マッチング
</h4>

Claude Code は、Claude が実行する各シェルコマンドに対して 1 つのコマンド名を記録します。これは、先頭の環境変数割り当てと `sudo` の後の最初のトークンです。複合コマンドは先頭のコマンドのみを提供するため、`cd infra && terraform plan` は `terraform` ではなく `cd` を記録します。

<h4 id="manifest-dependency-matching">
  マニフェスト依存関係マッチング
</h4>

各 `manifestDeps` エントリは、2 つの JavaScript `RegExp` ソース文字列をペアにします。

* `file`: マニフェストファイルのパスに対して大文字と小文字を区別しないでマッチします。パスは通常絶対パスであるため、開始ではなく終了にパターンをアンカーしてください。パスはこのシグナルに対して区切り文字で正規化されないため、Windows パスはバックスラッシュを使用します。
* `pattern`: そのファイルの内容に対して大文字と小文字を区別してマッチします。

次の例は、`manifestDeps` を使用して、Claude が SDK の npm パッケージ（ここでは `your-sdk` という名前）に依存する `package.json` を読み取った後、プラグインを提案しています。

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

この例では、`file` パターンは `[/\\\\]` を使用してフォワードスラッシュとバックスラッシュの両方のパス区切り文字にマッチし、`\\.` はドットがリテラルであることを示します。JSON では、正規表現の各バックスラッシュは 2 回書き込まれます。

<h2 id="validate-your-marketplace">
  マーケットプレイスを検証する
</h2>

シェルで、マーケットプレイスディレクトリに対して `claude plugin validate` を実行して、公開する前に `relevance` ブロックを確認します。

```bash theme={null}
claude plugin validate ./my-marketplace
```

バリデーターは `relevance` ブロックのエラーと警告を報告します。これには以下が含まれます。

* `relevance` と `relevance.signals` の下の未知のキーを警告として報告します
* オブジェクトではない `relevance` 値にフラグを立てます
* スキーム、ポート、またはパスを含む `signals.hosts` エントリを拒否します

各検出結果は、それが関係するフィールドのパスとともに出力され、出力は `Validation passed`、`Validation passed with warnings`、または `Validation failed` で終わります。

<h2 id="enable-suggestions-in-managed-settings">
  マネージド設定で提案を有効にする
</h2>

ユーザーは、マーケットプレイスの `marketplace.json` が `relevance` を宣言している場合でも、管理者が [マネージド設定](/docs/ja/plugins/org) でそれをホワイトリストに登録するまで、マーケットプレイスからの提案を表示されません。

マーケットプレイスをホワイトリストに登録するには、マネージド設定を次のように編集します。

* マーケットプレイス名を `pluginSuggestionMarketplaces` に追加します。
* 公式 Anthropic マーケットプレイス以外のマーケットプレイスについては、マーケットプレイスソースも宣言します。その名前のエントリとして [`extraKnownMarketplaces`](/docs/ja/plugins/org#require-a-marketplace-and-its-plugins) に、またはエントリとして [`strictKnownMarketplaces`](/docs/ja/plugins/org#allowlist-with-strictknownmarketplaces) に宣言します。

マーケットプレイスが登録されていないマシン、または異なるソースからホワイトリストに登録された名前で登録されているマシンでは、そこからの提案は表示されません。ソースチェックにより、関連のないソースがホワイトリストに登録された名前で登録されて、その プラグインが組織全体で提案されるのを防ぎます。

次の `managed-settings.json` は、GitHub リポジトリから組織マーケットプレイスを登録し、その提案を有効にします。

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

公式マーケットプレイスの名前は公式 Anthropic ソースからのみ登録できるため、ソース宣言は不要です。公式マーケットプレイスについては、名前だけをホワイトリストに登録します。

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  ユーザーが見るものをプレビューする
</h2>

プラグインの `relevance` シグナルがセッション中にマッチする場合、スピナーの下のチップは次のように読みます。

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

`cwd` シグナルがセッション開始時にマッチする場合、1 行の通知は次のように読みます。

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

`/plugin` Discover タブでは、プラグインは他の結果の上にピン留めされ、`suggested for this directory` または `suggested for terraform commands` などのマッチングシグナルを指定する注釈が付きます。

Claude Code は、特定のプラグインを提案する頻度を制限します。

* 提案は、スピナーチップとセッション開始通知を合わせて、最大 3 セッションごとに 1 回表示されます。
* セッション開始通知は、スピナーチップと通知がプラグインを合わせて 2 回表示されたら表示されなくなります。
* スピナーチップもセッション開始通知も、プラグインがインストールされたら繰り返されません。
* Discover タブは、プラグインのシグナルがマッチしている間にユーザーがタブを初めて開くときにプラグインをピン留めします。Claude Code はそれを `~/.claude.json` に記録するため、ユーザーがそのマシンで `/plugin` を開くたびに、プラグインは通常の順序で表示されます。

<h2 id="see-also">
  関連項目
</h2>

* [マーケットプレイスをホストする](/docs/ja/plugins/host-marketplace): プラグインをホストするマーケットプレイスを実行します
* [マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference#plugin-entries): プラグインエントリが受け入れるすべてのフィールド
* [CLI からプラグインを推奨する](/docs/ja/plugins/cli-hints): Claude Code のセッションシグナルではなく、独自の CLI からユーザーにプロンプトを表示します
* [組織向けプラグインを管理する](/docs/ja/plugins/org): `extraKnownMarketplaces`、`strictKnownMarketplaces`、およびその他のプラグインポリシーキー
