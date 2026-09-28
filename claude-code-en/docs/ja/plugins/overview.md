> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインの概要

> Claude Code プラグインとは何か、スタンドアロンスキルまたは MCP サーバーの代わりにプラグインが必要な場合、およびプラグインをインストールまたは作成するために読むべきページについて理解します。

Claude Code プラグインは、Claude Code がインストールして 1 つのユニットとして読み込むスキル、エージェント、hooks、MCP サーバー、またはその他のコンポーネントのディレクトリです。ほとんどのプラグインはマーケットプレイスから提供されます。マーケットプレイスはプラグインをリストアップし、各プラグインをどこから取得するかを示すカタログです。誰かが提供したフォルダからプラグインを読み込むこともできますし、[独自に構築する](/docs/ja/plugins/create)こともできます。

<Note>
  claude.ai チャットまたは Cowork を使用していて Claude Code を使用していない場合は、[claude.ai および Cowork のプラグイン](https://claude.com/docs/plugins/overview)を参照してください。
</Note>

プラグインを今すぐ試すには、Claude Code ターミナルセッションで `/plugin` を実行し、**Discover** タブからプラグインをインストールします。このタブには、Anthropic の公式マーケットプレイスと追加したマーケットプレイスのプラグインがリストアップされています。そこから：

* [プラグインのインストールと管理](/docs/ja/plugins/install)：完全なインストール手順、スコープ、およびその他のサーフェス
* [プラグインの作成](/docs/ja/plugins/create)：独自のプラグインを構築する
* [プラグインが必要かどうかを判断する](#decide-whether-you-need-a-plugin)：プラグインが必要なツールかどうかを判断する

<h2 id="understand-what-a-plugin-is">
  プラグインとは何かを理解する
</h2>

プラグインは通常、マニフェストを含むコンポーネントのディレクトリです。マニフェストは `.claude-plugin/plugin.json` にある JSON ファイルで、プラグインに名前を付け、バージョン、説明、およびその他の[メタデータ](/docs/ja/plugins/manifest-reference)を追加できます。コンポーネントはプラグインが Claude Code に追加するもので、以下のようなものです：

* [**Skills**](/docs/ja/plugins/components#skills)：Claude が関連する場合に読み込む `SKILL.md` 命令。コマンドとして実行することもできます
* [**Agents**](/docs/ja/plugins/components#agents)：Claude が委譲できるサブエージェント定義
* [**Hooks**](/docs/ja/plugins/components#hooks)：編集後など、ライフサイクルのポイントで Claude Code が実行するコマンド
* [**MCP servers**](/docs/ja/plugins/components#mcp-servers)：プラグインが有効な場合に Claude Code が接続するツールサーバー

このダイアグラムは、`my-plugin` という名前のプラグインを示しており、これらの各コンポーネントを 1 つずつ保持しており、プラグインが読み込まれた後に各ファイルから何を取得するかを示しています。

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

プラグインが保持できるすべてのコンポーネントタイプについて、各コンポーネントの例を含めて、[プラグインコンポーネント](/docs/ja/plugins/components)を参照してください。プラグインのディレクトリ内の各部分がどこに配置されているかを確認するには、そのページの[プラグインエクスプローラー](/docs/ja/plugins/components#explore-the-plugin-directory)を使用してください。

<h3 id="decide-whether-you-need-a-plugin">
  プラグインが必要かどうかを判断する
</h3>

スキル、サブエージェント、hooks、および MCP サーバーはすべて、プラグインなしで単独で機能します。たとえば、`~/.claude/skills/` に保存したスキルは、マシン上のすべてのプロジェクトで利用可能です。単独で設定するには、[Skills](/docs/ja/skills)、[Subagents](/docs/ja/sub-agents)、[Hooks](/docs/ja/hooks-guide)、または [MCP](/docs/ja/mcp) を参照してください。

複数のスキル、サブエージェント、hooks、または MCP サーバーを 1 つのユニットとしてパッケージ化したい場合は、プラグインを使用します。インストールして、他の誰かが構築したセットアップを取得し、1 つのコマンドとマーケットプレイスからの更新を使用します。独自のセットアップをチームメイトに提供したり、多くのプロジェクトにインストールしたり、バージョン付きリリースを公開したりするために作成します。

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  有効なプラグインがセッションに追加するもの
</h3>

有効なプラグインは、それを使用するセッションだけでなく、すべてのセッションの一部です。インストール前に知っておく価値のある結果がいくつかあります：

* **コンテキストと使用状況**：[Claude が独自に呼び出すことができる](/docs/ja/skills#control-who-invokes-a-skill)各スキル、エージェント、およびコマンドについて、名前と説明は Claude のコンテキストにあり、Claude がそれが存在することを知っています。これらのトークンは使用状況にカウントされ、プラグインから何も実行されないセッションでも[コンテキストウィンドウ](/docs/ja/context-window)の空き容量が減ります。スキルまたはエージェントの完全なテキストは、使用される場合にのみ読み込まれます。プラグインの MCP サーバーがターンごとに追加するものは、[MCP ツール検索](/docs/ja/mcp#scale-with-mcp-tool-search)に従います。
* **プロセス**：プラグインが定義する MCP サーバーは、有効な各セッションと並行して実行され、その hooks はイベントで発火します。
* **権限**：プラグインが実行するもの、それはあなたとして実行されます。最初に確認すべきことについては、[プラグインのセキュリティと信頼](/docs/ja/plugins/security)を参照してください。

各段階でプラグインのフットプリントを確認できます：

* **インストール前**：`/plugin` の **Marketplaces** タブからプラグインを開きます。Anthropic の公式マーケットプレイスのプラグインは、そこに **Context cost** の推定値を表示します。
* **インストール後**：[プラグインのコストを測定する](/docs/ja/plugins/measure#measure-what-a-plugin-costs)は、プラグインのフットプリントを読む方法を示し、**Installed** タブの **Not used recently** グループは、オフにできるプラグインをリストアップします。
* **アンインストールせずに停止する**：`/plugin` でプラグインを無効にするか、シェルで `claude plugin disable` を実行します。[インストール済みプラグインの管理](/docs/ja/plugins/install#manage-installed-plugins)を参照してください。

<h2 id="get-plugins-from-a-marketplace">
  マーケットプレイスからプラグインを取得する
</h2>

マーケットプレイスは、プラグインをリストアップし、各プラグインをどこから取得するかを示す `.claude-plugin/marketplace.json` ファイルを持つリポジトリまたはディレクトリです。これはホストされたストアではなく、カタログです。マーケットプレイスを 1 回追加してから、`commit-commands@claude-plugins-official` などの名前でプラグインをインストールします。

<Note>
  プラグインマーケットプレイスは [Claude Marketplace](https://claude.com/marketplace) ではありません。Claude Marketplace は claude.com/marketplace の Web サイトで、プラグイン、コネクタ、パートナー製品、およびサービスパートナーを参照できます。これは `/plugin marketplace add` で追加するマーケットプレイスではありません。
</Note>

Claude Code は、[管理ポリシー](/docs/ja/plugins/org#allow-the-official-marketplace-and-your-own)がブロックしない限り、対話型ターミナルセッションを初めて開始するときに Anthropic の公式マーケットプレイスを追加します。Claude Code は、Anthropic のコミュニティおよびデモマーケットプレイスを含む、他のマーケットプレイスを独自に追加しません。3 つの Anthropic マーケットプレイスを区別するには、[Anthropic のマーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)を読んでください。公式マーケットプレイスがリストアップしているものを確認するには、セッションで `/plugin` の **Discover** タブを開くか、[Claude Marketplace](https://claude.com/marketplace/plugins)を参照してください。

このダイアグラムは、マーケットプレイスからセッションへのパスを示しています。マーケットプレイスはプラグインをリストアップし、そのプラグインをインストールし、Claude Code がそのコンポーネントを読み込みます。

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagram of the marketplace path in three boxes, left to right. A marketplace, a catalog of plugins, lists a plugin. The plugin is one directory installed as a unit, holding skills, agents, hooks, MCP servers, and other components. You install the plugin into Claude Code, which loads its components." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagram of the marketplace path in three boxes, left to right. A marketplace, a catalog of plugins, lists a plugin. The plugin is one directory installed as a unit, holding skills, agents, hooks, MCP servers, and other components. You install the plugin into Claude Code, which loads its components." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[プラグインのインストールと管理](/docs/ja/plugins/install#install-a-plugin)には、Claude Code を実行する各場所のインストール手順があります。プラグインを開発している間は、マーケットプレイスは必要ありません。[マーケットプレイスなしで開発する](/docs/ja/plugins/create#develop-without-a-marketplace)に示すように、`--plugin-dir` でフォルダから直接読み込みます。

<h3 id="make-an-installed-plugin-available-in-your-session">
  インストール済みプラグインをセッションで利用可能にする
</h3>

インストール済みプラグインが実行できるスキルを提供する前に、これらの各レイヤーに存在する必要があります：

* **Settings**：設定は、追加したマーケットプレイスと有効なプラグインをリストアップします。
* **Disk**：`~/.claude/plugins/` は、Claude Code がフェッチしてインストールしたものを保持します。
* **Session**：プラグインはスタートアップで読み込まれるか、[プラグインを再度読み込む](/docs/ja/plugins/loading#check-which-stage-a-plugin-reached)ときに読み込まれます。

[プラグイン読み込みリファレンス](/docs/ja/plugins/loading)を読んで、各レイヤーのルール、どの設定ファイルが優先されるか、およびディスク上のファイルの場所を確認してください。

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Anthropic のマーケットプレイスをサードパーティのマーケットプレイスから区別する
</h2>

マーケットプレイスの名前は、それを 3 つのティアのいずれかに配置します。Claude Code は、`github.com/anthropics/` リポジトリから取得したマーケットプレイスの公式およびコミュニティ名のみを受け入れます：

* **Official**：Anthropic の[公式マーケットプレイス名](/docs/ja/plugins/security#official-marketplace-names)の 1 つを持つマーケットプレイス。`claude-plugins-official` とデモマーケットプレイス `claude-code-plugins` を含みます。
* **Community**：`claude-community` などの Anthropic のコミュニティ名を持つマーケットプレイス。[Anthropic のマーケットプレイスを名前で識別する](/docs/ja/plugins/security#marketplace-tiers)がそれらをリストアップします。
* **Third-party**：他のすべてのマーケットプレイス。同僚または組織が公開するマーケットプレイスはサードパーティです。

ティアに関係なく、インストールするプラグインはユーザー権限でコードを実行できます。プラグインをインストール前に確認する方法については、[プラグインのセキュリティと信頼](/docs/ja/plugins/security)を読んでください。

[管理設定](/docs/ja/settings#settings-files)を通じて、組織はマーケットプレイスをホワイトリストまたはブロックし、プラグインを強制インストールし、セッションのみの読み込みをオフにできます。これらのコントロールについては、[組織のプラグインを管理する](/docs/ja/plugins/org)を読んでください。

<h2 id="understand-install-scopes">
  インストールスコープを理解する
</h2>

プラグインをインストールするときは、スコープを選択し、スコープはプラグインが有効な対象を決定します：

* **User scope**：このコンピューター上のすべてのプロジェクトで有効
* **Project scope**：コミットされた `.claude/settings.json` を通じて、このリポジトリで作業するすべての人に対して有効。各協力者は依然として[自分のマシンにインストール](/docs/ja/plugins/loading#enabled-in-project-settings-but-not-installed)する必要があります
* **Local scope**：このリポジトリでのみ有効

ターミナル、デスクトップアプリのローカルセッション、または VS Code 拡張機能でユーザースコープでインストールしたプラグインは、3 つすべてが同じ設定ファイルを読むため、そのコンピューター上の他の 2 つで利用可能です。スコープを選択する方法については、[インストールスコープを選択する](/docs/ja/plugins/install#choose-an-install-scope)を参照してください。

claude.ai/code のブラウザーを含むクラウドセッションは、ローカル設定のプラグインを読み込みません。ターミナル、VS Code、デスクトップアプリでのインストール手順、およびクラウドセッションが読み込むものについては、[プラグインのインストール](/docs/ja/plugins/install#install-a-plugin)を参照してください。

<Note>
  同じプラグイン形式は claude.ai および Cowork にもインストールされます。異なるコンポーネントセットが読み込まれます。これらのサーフェスについては、claude.com の[claude.ai および Cowork のプラグイン](https://claude.com/docs/plugins/overview)を参照してください。
</Note>

<h2 id="next-steps">
  次のステップ
</h2>

ほとんどの人は、Claude Code が対話型ターミナルセッションを初めて開始するときに追加する Anthropic の公式マーケットプレイスからプラグインをインストールすることから始めます。ターミナルセッションで `/plugin` を実行して参照するか、[プラグインのインストールと管理](/docs/ja/plugins/install)に従います。これはデスクトップアプリと VS Code もカバーしています。Claude Code を開く前にそのマーケットプレイスに何があるかを確認するには、Web で [Claude Marketplace](https://claude.com/marketplace/plugins)を参照してください。

独自に構築するには、[プラグインの作成](/docs/ja/plugins/create)は空のディレクトリから始まり、動作するプラグインで終わります。

プラグインをインストールまたは構築したら、これらのページは次に来るものをカバーしています：

* **構築したものを共有する**：[プラグインの公開と配布](/docs/ja/plugins/publish)
* **それが機能しているかどうか、および使用されているかどうかを確認する**：[evals でプラグインをテストする](/docs/ja/plugin-evals)および[プラグインのコストと使用状況を測定する](/docs/ja/plugins/measure)
* **チーム向けのマーケットプレイスを実行する**：[マーケットプレイスを作成する](/docs/ja/plugins/create-marketplace)、次に[マーケットプレイスをホストして維持する](/docs/ja/plugins/host-marketplace)
* **組織のプラグインポリシーを設定する**：[組織のプラグインを管理する](/docs/ja/plugins/org)
* **問題を修正する**：[プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)
