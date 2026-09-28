> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Anthropic のマーケットプレイス

> Claude Code 向けの Anthropic 公式、コミュニティ、デモプラグインマーケットプレイス：それぞれの名前、リポジトリ、追加方法、プラグインの閲覧場所。

Anthropic は Claude Code 向けに 3 つの汎用プラグインマーケットプレイスを公開しています：[公式](https://github.com/anthropics/claude-plugins-official)、[コミュニティ](https://github.com/anthropics/claude-plugins-community)、[デモ](https://github.com/anthropics/claude-code)。各マーケットプレイスは独自の GitHub リポジトリ内のプラグインカタログです。Claude Code セッションでこれらのいずれかからプラグインをインストールする場合、`@` の後にマーケットプレイス名を入力します。例えば `/plugin install commit-commands@claude-plugins-official` のようにします。

このページを使用して、3 つのマーケットプレイスを区別し、公式マーケットプレイスに特定のプラグインが含まれているかどうかを確認する場所を見つけてください。

<Note>
  これらのケースは他のページで説明されています：

  * **プラグインのインストール方法**：[プラグインのインストール](/docs/ja/plugins/install)を参照してください
  * **インストール失敗**：[プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)を参照してください
</Note>

必要なページの部分に移動してください：

* 3 つのマーケットプレイスをリポジトリ、マーケットプレイス名、取得方法で区別するには、[Anthropic のマーケットプレイス](#anthropic%E2%80%99s-marketplaces)を参照してください。
* 公式マーケットプレイスでプラグインを見つけるには、[公式マーケットプレイスでプラグインを見つける](#find-plugins-in-the-official-marketplace)を参照してください。

<h2 id="anthropic’s-marketplaces">
  Anthropic のマーケットプレイス
</h2>

マーケットプレイスは、リポジトリが `.claude-plugin/marketplace.json` ファイルで定義するプラグインのカタログです。公式、コミュニティ、デモマーケットプレイスはそれぞれ独自の GitHub リポジトリから提供されます。Anthropic は `anthropics/skills` や `anthropics/knowledge-work-plugins` などのトピック固有のマーケットプレイスも公開しており、Claude Code セッションで `/plugin marketplace add <owner>/<repo>` を使用して追加できます。

この表は各マーケットプレイスのリポジトリとマーケットプレイス名を示しており、マーケットプレイス名はそのマーケットプレイスからプラグインをインストールする際に `@` の後に入力するものです。コミュニティマーケットプレイスの名前はリポジトリ名ではなく `claude-community` です。

|            | 公式                                                                                                                                                                                                                                                                                                                                 | コミュニティ                                                                                          | デモ                                                                                      |
| :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| リポジトリ      | [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official)                                                                                                                                                                                                                                      | [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) | [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/plugins) |
| マーケットプレイス名 | `claude-plugins-official`                                                                                                                                                                                                                                                                                                          | `claude-community`                                                                              | `claude-code-plugins`                                                                   |
| 含まれるもの     | Anthropic が保守するプラグイン、およびパートナーと他の作成者からのプラグイン                                                                                                                                                                                                                                                                                        | 作成者が Anthropic に提出したサードパーティプラグイン                                                                | プラグインに含まれる内容を示す小規模な例プラグインセット                                                            |
| 取得方法       | Claude Code は、[管理ポリシー](/docs/ja/plugins/org#allow-the-official-marketplace-and-your-own)または `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` がブロックしない限り、対話型ターミナルセッションを初めて開始するときに追加します。見つからない場合は[マーケットプレイス `claude-plugins-official` が見つかりません](/docs/ja/plugins/troubleshooting#marketplace-claude-plugins-official-not-found)を参照してください | Claude Code セッションで `/plugin marketplace add anthropics/claude-plugins-community` を実行して追加します     | Claude Code セッションで `/plugin marketplace add anthropics/claude-code` を実行して追加します          |

プラグインを作成して他の人にインストールしてもらいたい場合は、[プラグインを公開する](/docs/ja/plugins/publish)を参照してください。これは独自のマーケットプレイスとコミュニティマーケットプレイスへの提出について説明しています。

<h3 id="the-demo-marketplace-in-anthropics/claude-code">
  `anthropics/claude-code` のデモマーケットプレイス
</h3>

チュートリアルまたは古い指示セットで `/plugin marketplace add anthropics/claude-code` を実行するよう指示されている場合、これはデモマーケットプレイス（`claude-code-plugins` という名前）を追加します。これは Claude Code が既に追加した公式マーケットプレイスではありません。

デモマーケットプレイスのプラグインのほとんどは、同じ名前で公式マーケットプレイスにも含まれています。例えば、`code-review`、`feature-dev`、`commit-commands`、`security-guidance` は両方に含まれています。`claude-plugins-official` からこれらをインストールして、2 つのコピーがインストールされないようにしてください。

<h2 id="find-plugins-in-the-official-marketplace">
  公式マーケットプレイスでプラグインを見つける
</h2>

公式マーケットプレイス `claude-plugins-official` は Claude Code があなたのために追加するものです。リストの大部分は Anthropic ではなくパートナーと他の作成者から提供されています：ツールベンダーは Claude Code をそのサービスに接続するプラグインを公開し、Anthropic は `commit-commands`、`code-review`、`feature-dev`、[言語サーバープラグイン](/docs/ja/plugins/code-intelligence)などの小規模なセットを保守しています。カタログは頻繁に変更されるため、このページではリストしていません。

その中身を確認するには、Claude Code セッションで `/plugin` の **Discover** タブを使用します。これは検索可能です。または、ウェブで [Claude Marketplace](https://claude.com/marketplace/plugins) を閲覧してください。

<h2 id="browse-and-install-from-anthropic’s-marketplaces">
  Anthropic のマーケットプレイスを閲覧してインストールする
</h2>

Claude Code、ウェブ、または GitHub で Anthropic のマーケットプレイスからプラグインを検索できます：

* **Claude Code で閲覧する**：対話型セッションで `/plugin` を実行します。**Discover** タブには、追加したマーケットプレイスからのプラグインがリストされます。
* **Claude Code で名前で検索**：セッションで `/plugin install <name>` を実行します。これは追加したマーケットプレイスで名前を検索します。プラグインがそのいずれかに含まれている場合、その詳細が `/plugin` パネルで開き、[インストールスコープ](/docs/ja/plugins/install#install-a-plugin)を選択して確認するまで何もインストールされません。含まれていない場合は、`Plugin "<name>" not found in any marketplace` が表示されます。
* **ウェブで**：[Claude Marketplace](https://claude.com/marketplace/plugins) で完全なカタログを検索します。これはインストール数を表示し、一部のプラグインを **Anthropic verified** としてマークしています。
* **GitHub で**：マーケットプレイスのリポジトリ（[`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official) など）で `.claude-plugin/marketplace.json` を開きます。このファイルはカタログ自体です。

デスクトップアプリまたはスクリプトからインストールするか、クラウドセッションが読み込むものを確認するには、[プラグインのインストール](/docs/ja/plugins/install)を参照してください。

<h3 id="add-the-community-or-demo-marketplace">
  コミュニティマーケットプレイスまたはデモマーケットプレイスを追加する
</h3>

コミュニティマーケットプレイスとデモマーケットプレイスは、Claude Code セッションで追加するまで登録されません：

* **コミュニティ**：`/plugin marketplace add anthropics/claude-plugins-community` を実行してから、`@claude-community` サフィックスでインストールします。
* **デモ**：`/plugin marketplace add anthropics/claude-code` を実行してから、`@claude-code-plugins` サフィックスでインストールします。

`claude-plugins-official` が `/plugin` の **Marketplaces** タブにない場合は、`/plugin marketplace add anthropics/claude-plugins-official` で同じ方法で追加してください。

`not found` エラーと追加されないマーケットプレイスについては、[プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting#install-a-plugin)を参照してください。

<h2 id="third-party-marketplaces">
  サードパーティマーケットプレイス
</h2>

多くの人気プラグインは Anthropic マーケットプレイスのいずれにも含まれていません。それらは作成者独自のマーケットプレイスにあり、通常はルートに `.claude-plugin/marketplace.json` を持つ GitHub リポジトリです。

Anthropic はサードパーティマーケットプレイスをレビューしないため、追加する前に[プラグインのセキュリティと信頼](/docs/ja/plugins/security)を読んでください。

サードパーティマーケットプレイスを使用するには、Claude Code セッションで `/plugin marketplace add <owner>/<repo>` を使用してそのリポジトリを追加してから、`/plugin install <plugin>@<marketplace-name>` でインストールします。マーケットプレイス名はその `marketplace.json` の `name` フィールドであり、Claude Code はマーケットプレイスを追加した後にそれを出力します。

マーケットプレイスを追加する他の方法については、[マーケットプレイスを追加する](/docs/ja/plugins/install#add-a-marketplace)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインのインストールと管理](/docs/ja/plugins/install)：これらのマーケットプレイスの 1 つからプラグインをインストールしてスコープを選択します
* [プラグインのセキュリティと信頼](/docs/ja/plugins/security)：プラグインがマシンで何ができるか、およびインストール前にプラグインをレビューする方法
* [コード インテリジェンスプラグイン](/docs/ja/plugins/code-intelligence)：公式マーケットプレイスの言語サーバープラグインの 1 つをインストールします
* [マーケットプレイスを作成する](/docs/ja/plugins/create-marketplace)：Anthropic のマーケットプレイスと並行して独自のマーケットプレイスを実行します
