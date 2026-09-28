> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code がプロンプトキャッシングを使用する方法

> Claude Code はプロンプトキャッシングを自動的に管理します。モデル切り替えがキャッシュなしの遅いターンをトリガーする理由、`/compact` のコスト、CLAUDE.md の編集がセッション中に適用されない理由、キャッシュヒット率を確認する方法を確認してください。

プロンプトキャッシングにより、Claude Code はより高速で費用効率的になります。キャッシングがなければ、API はターンごとに完全な履歴を再処理します。キャッシングがあれば、既に処理したものを再利用し、再読み込みを[キャッシュされたトークンレート](https://platform.claude.com/docs/en/about-claude/pricing)で請求し、変更されたものに対してのみ完全に処理します。

Claude Code はプロンプトキャッシングを自動的に処理します。ただし、[無効にする](#disable-prompt-caching)ことはできます。プロンプトキャッシングの仕組みを理解することは依然として有用です。キャッシュを無効にするアクションがあり、次の応答が遅くなり、再構築中により高くなるためです。このページでは、どのアクションがそうであるか、一部の設定が再起動を待つ理由、使用量が高く見える場合にキャッシュパフォーマンスを確認する方法について説明します。

<h2 id="how-the-cache-is-organized">
  キャッシュの構成方法
</h2>

Claude Code でメッセージを送信するたびに、新しい API リクエストが作成されます。モデルはリクエスト間で何も記憶しないため、Claude Code は完全なコンテキストを再送信します。システムプロンプト、プロジェクトコンテキスト、すべての以前のメッセージとツール結果、および新しいメッセージです。新しいコンテンツは最後に追加されるため、各リクエストのほとんどは前のリクエストと同じです。プロンプトキャッシングは、API が変更されなかった部分の再処理を回避する方法です。

API は各リクエストの開始部分（プリフィックスと呼ばれます）を最近処理したコンテンツと照合することでキャッシュします。通常のターンでは、プリフィックスは前のリクエスト全体であり、最新の交換のみが新しいものです。一致は正確であるため、プリフィックスのどこかで変更があると、その後のすべてが再計算されます。ファイルごとまたはセグメントごとのキャッシングはありません。API リファレンスの [プロンプトキャッシングの仕組み](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) を参照して、基礎となるメカニズムを確認してください。

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="4 つのターンが成長する水平バーとして表示されています。各ターンのリクエストには、前のターンのすべてと、最後に追加された最新の交換が含まれています。ターン 2 と 3 では、変更されていないプリフィックスがキャッシュから読み込まれ、新しい交換のみが処理されます。ターン 4 では、システムプロンプトが変更されたため、プリフィックスが一致しなくなり、リクエスト全体が再処理されてキャッシュに書き込まれます。" width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="4 つのターンが成長する水平バーとして表示されています。各ターンのリクエストには、前のターンのすべてと、最後に追加された最新の交換が含まれています。ターン 2 と 3 では、変更されていないプリフィックスがキャッシュから読み込まれ、新しい交換のみが処理されます。ターン 4 では、システムプロンプトが変更されたため、プリフィックスが一致しなくなり、リクエスト全体が再処理されてキャッシュに書き込まれます。" width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

プリフィックスマッチングを最大限に活用するために、Claude Code は各リクエストを整理して、ターン間で変更されることが少ないコンテンツを最初に配置します。

| レイヤー         | コンテンツ                     | 変更される場合                                 |
| ------------ | ------------------------- | --------------------------------------- |
| システムプロンプト    | コア命令、ツール定義                | 読み込まれたツール定義のセットが変更される                   |
| プロジェクトコンテキスト | CLAUDE.md、自動メモリ、スコープなしルール | セッション開始時、または `/clear` または `/compact` の後 |
| 会話           | メッセージ、Claude の応答、ツール結果    | 毎ターン                                    |

会話レイヤーへの変更は、システムプロンプトとプロジェクトコンテキストをキャッシュされたままにします。システムプロンプトへの変更は、すべての後続コンテンツが異なるプリフィックスの後ろに配置されるようになるため、すべてを無効にします。3 番目の列は、完全なリストではなく一般的なトリガーを示しており、以下のセクションで完全なセットについて説明します。

プリフィックスマッチルールは、このページのほとんどの動作を説明しています。たとえば、[Plan Mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) と [スキル読み込み](/docs/ja/skills) は、その命令を会話メッセージとして追加するため、キャッシュされたプリフィックスはそのままです。

レイヤーテーブルに表示されないが、キャッシュされたままになるものに影響する 2 つの設定があります。

* **モデル**：各モデルには独自のキャッシュがあります。モデルを切り替えると、コンテンツが同じであってもリクエスト全体が再計算されます。以下の [モデルの切り替え](#switching-models) を参照してください。
* **エフォートレベル**：ほとんどのモデルでは、各エフォートレベルには独自のキャッシュがあるため、セッション中にエフォートを変更するとリクエスト全体が再計算されます。API キーまたは Claude サブスクリプションを使用した Opus 5.5 および Fable 5.1 では、デフォルトではキャッシュはそのままです。以下の [エフォートレベルの変更](#changing-effort-level) を参照してください。

<Tip>
  セッションの最初にモデルとエフォートレベルを選択し、タスク間の自然な区切りのために `/compact` を保存してください。タスク中に行う変更が少ないほど、キャッシュヒット率が高くなります。
</Tip>

<h3 id="where-the-cache-lives">
  キャッシュの場所
</h3>

キャッシングはサーバー側で行われ、モデルを提供するインフラストラクチャで行われます。その場所は、認証方法によって異なります。

* **API キー、Claude サブスクリプション、または [Claude Platform on AWS](/docs/ja/claude-platform-on-aws)**：キャッシュは Anthropic のインフラストラクチャに存在し、[Claude API](https://platform.claude.com/docs) を通じてアクセスされます。
* **Amazon Bedrock または Google Cloud の Agent Platform**：キャッシュはクラウドプロバイダーのサービングインフラストラクチャに存在します。
* **Microsoft Foundry**：デプロイメントの [ホスティングオプション](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) によって異なります。Azure デプロイメント上でホストされている場合は Azure インフラストラクチャで提供されます。Anthropic デプロイメント上でホストされている場合は Anthropic のインフラストラクチャで提供されます。
* **カスタム `ANTHROPIC_BASE_URL` または [LLM ゲートウェイ](/docs/ja/llm-gateway)**：キャッシュはリクエストが転送される場所に存在し、キャッシングが機能するかどうかはゲートウェイに依存します。

Claude Code は会話中にシステムコンテキスト（ファイル変更通知など）も追加し、[`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ja/llm-gateway-protocol#disable-pre-release-capabilities) を設定しない限り、すべてのプロバイダーと接続でそのブロックをキャッシング用にマークします。その場合、そのブロックはキャッシュなしで送信されます。

プロバイダー独自のエンドポイント、Amazon Bedrock とその [Mantle エンドポイント](/docs/ja/amazon-bedrock#use-the-mantle-endpoint)、Google Cloud の Agent Platform、および Microsoft Foundry では、Claude API と同じ方法でブロックをキャッシュします。

リクエストが [LLM ゲートウェイ](/docs/ja/llm-gateway)、カスタム `ANTHROPIC_BASE_URL`、または [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/ja/env-vars) などのクラウドプロバイダーベース URL オーバーライドを通過する場合、キャッシュされたままになるものは、ゲートウェイが Claude Code が送信する [`cache_control` マーカー](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) をどのように処理するかに依存します。

* **変更されずに転送する**：ブロックと会話は、プロバイダー独自のエンドポイントと同じようにキャッシュされます。
* **`cache_control` という名前の `400` エラーでマークされたリクエストを拒否する**：Claude Code はマーカーをブロックから最後の会話メッセージに移動させてリクエストを再送信し、会話の残りの部分でそこに保持します。ブロックはキャッシュなし入力として請求されます。会話はキャッシュされたままです。
* **マーカーを削除して成功を返す**：会話履歴全体は、毎ターン、キャッシュなし入力として請求されます。ブロック形式のシステムコンテンツをプレーン文字列に変換するゲートウェイは、同じ方法でマーカーをドロップします。

各プロバイダーが保存および処理するものについては、[データ使用](/docs/ja/data-usage) を参照してください。キャッシュがどこに存在するかに関係なく、エントリは非アクティブ期間後に期限切れになり、以下の [キャッシュの有効期間](#cache-lifetime) では TTL とそれを延長する方法について説明しています。

<h2 id="actions-that-invalidate-the-cache">
  キャッシュを無効化するアクション
</h2>

これらのアクションは、次のリクエストでキャッシュの一部または全部がミスになる原因となります。その後、新しいプレフィックスがキャッシュされるまで、1 回限りの遅く、より高額なターンが表示されます。これらのほとんどは、コストがあることを知ったら、タスク中に回避できます。モデルスイッチは、その後の遅いターンに気付くまで無料に感じられるかもしれません。

* [モデルの切り替え](#switching-models)
* [努力レベルの変更](#changing-effort-level)
* [高速モードの有効化](#turning-on-fast-mode)
* [MCP サーバーの接続または切断](#connecting-or-disconnecting-an-mcp-server)
* [プラグインの有効化または無効化](#enabling-or-disabling-a-plugin)
* [ツール全体の拒否](#denying-an-entire-tool)
* [会話のコンパクト化](#compacting-the-conversation)
* [多くの画像の蓄積](#accumulating-many-images)
* [Claude Code のアップグレード](#upgrading-claude-code)

<h3 id="switching-models">
  モデルの切り替え
</h3>

各モデルには独自のキャッシュがあります。[`/model`](/docs/ja/model-config#setting-your-model) で切り替えると、コンテンツが同じであっても、次のリクエストはキャッシュヒットなしで会話履歴全体を読み込みます。

ターミナルで `/model` を実行すると、キャッシュがまだ温かい間は、Claude Code はスイッチの確認を求めます。ただし、新しいモデルが最後のレスポンスを生成したモデルではない場合に限ります。キャッシュは、Claude Code がこの会話で最後にリクエストを送信した後、または Claude が最後に応答した後、1 つの[キャッシュ TTL](#cache-lifetime) の間、温かいままです。その時間が経過すると、キャッシュは期限切れになるため、Claude Code は確認を求めずに切り替えます。

v2.1.238 より前では、Claude Code はキャッシュ TTL をチェックせず、キャッシュが期限切れになった後でも確認を求めていました。

[PreModelSwitch フック](/docs/ja/hooks#premodelswitch-decision-control)を使用して、この確認を必須にするか、スキップすることもできます。

[`opusplan` モデル設定](/docs/ja/model-config#opusplan-model-setting)は、プランモード中は Opus に、実行中は Sonnet に解決されるため、各プランモードの切り替えはモデルスイッチであり、新しいキャッシュを開始します。

Fable モデルと Opus 5.5 および Opus 5 の[自動モデルフォールバック](/docs/ja/model-config#automatic-model-fallback)もモデルスイッチです。安全分類器がフォールバックモデルを持つカテゴリーでリクエストにフラグを立てると、Claude Code はそのモデルでリクエストを再実行し、セッションはそこで続行されます。

スキルまたはコマンドのフロントマターがセッションの現在のモデル以外の [`model`](/docs/ja/skills#frontmatter-reference)を指定する場合、そのターンもモデルスイッチです。次のリクエストはキャッシュヒットなしで会話履歴全体を読み込みます。セッションモデルは次のプロンプトで再開されます。`context: fork` スキルは、代わりに[フォークされたサブエージェントのモデル](/docs/ja/skills#run-skills-in-a-subagent)を設定します。

<h3 id="changing-effort-level">
  努力レベルの変更
</h3>

ほとんどのモデルでは、セッション中に[努力レベル](/docs/ja/model-config#adjust-effort-level)を変更すると、次のリクエストはキャッシュヒットなしで会話履歴全体を読み込みます。キャッシュがまだ温かい間は、Claude Code は最初に変更を確認するよう求めます。

API キーまたは Claude サブスクリプションを使用した Opus 5.5 および Fable 5.1 では、努力レベルの変更によってキャッシュが保持され、Claude Code は確認を求めずに新しいレベルを適用します。これは Amazon Bedrock、Google Cloud の Agent Platform、[Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway)、または [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ja/llm-gateway-protocol#disable-pre-release-capabilities)を設定した場合、あるいは組織が HIPAA 構成を持つ場合には適用されません。

v2.1.260 より前では、API キーまたは Claude サブスクリプションを使用した Fable 5.1 での努力レベルの変更もキャッシュを無効化していました。

<h3 id="turning-on-fast-mode">
  高速モードの有効化
</h3>

[高速モード](/docs/ja/fast-mode)を有効にすると、キャッシュキーの一部であるリクエストヘッダーが追加されるため、Claude Code が高速モードをオンにして送信する最初のリクエストはキャッシュヒットなしで会話履歴全体を読み込みます。Claude Code はターンの開始時にそのヘッダーを 1 回設定し、ターン全体でそれを保持するため、Claude が作業中に高速モードをオンにすると、ヘッダーからのキャッシュミスは次のターンの最初のリクエストで発生します。これらのキャッシュされていない入力トークンは[高速モードレート](/docs/ja/fast-mode#understand-the-cost-tradeoff)で請求されます。これが、セッションの開始時にオンにするのが、長いセッションの深くでオンにするよりもコストが低い理由です。現在のモデルが高速モードをサポートしていない場合、高速モードを有効にすると[モデルも切り替わり](#switching-models)、そのスイッチ自体が実行中のターンの次のリクエストから新しいキャッシュを開始します。

コストは会話ごとに 1 回適用されます。最初の高速モードターンの後、Claude Code はヘッダーを送信し続け、キャッシュキーの一部ではないリクエストの速度設定のみを変更します。高速モードをオフにする、[レート制限後の標準速度への自動フォールバック](/docs/ja/fast-mode#handle-rate-limits)、およびその後にオンに戻すことはすべてキャッシュを保持します。[使用クレジットが不足した](/docs/ja/fast-mode#handle-rate-limits)場合、Claude Code は各拒否された高速モードリクエストを同じ方法で標準速度で再試行するため、このフォールバックもキャッシュを保持します。`/clear` と `/compact` はこれをリセットします。これらはとにかくそれらのポイントでキャッシュを再構築するためです。

<h3 id="connecting-or-disconnecting-an-mcp-server">
  MCP サーバーの接続または切断
</h3>

ツール定義はシステムプロンプトレイヤーに存在するため、ターン間でリクエスト内のツール定義のセットが変わるとキャッシュが無効化されます。[アドバイザーツール](/docs/ja/advisor)の切り替えは例外です。その定義はキャッシュブレークポイントの後に存在するため、`/advisor` を有効または無効にするとキャッシュされたプレフィックスはそのまま保持されます。[MCP サーバー](/docs/ja/mcp)の変更がこれを行うかどうかは、そのツールが[ツール検索](/docs/ja/mcp#scale-with-mcp-tool-search)によって遅延されるか、プレフィックスに読み込まれるかによって異なります。

* **遅延ツール**、サポートされているモデルのデフォルト：サーバーの接続、切断、またはツールリストの変更は、新しいコンテンツを追加するだけで、既にキャッシュされているものを乱しません。
* **プレフィックスに読み込まれるツール**：それらへの変更はキャッシュを無効化します。これは、[ツール検索が利用できないか無効になっている](/docs/ja/mcp#configure-tool-search)場合に発生します。例えば、Claude 4.5 世代より前の Google Cloud の Agent Platform モデル、カスタム `ANTHROPIC_BASE_URL` ゲートウェイ、または Claude Code がデプロイメントがツール検索を拒否することを検出した Microsoft Foundry [Azure でホストされているデプロイメント](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)の場合です。また、[`alwaysLoad`](/docs/ja/mcp#exempt-a-server-from-deferral)とマークされたサーバーまたはツール、および[閾値ベースの読み込み](/docs/ja/mcp#configure-tool-search)によって前もって保持される定義の場合にも発生します。

ツールがプレフィックスに読み込まれる場合、無効化の最も一般的な原因は、セッション中にサーバーが接続または切断されることです。これは、あなたのアクションなしに発生する可能性があります。stdio サーバーのプロセスが終了する、HTTP セッションが期限切れになる、またはサーバーが[一時的な障害後に自動的に再接続](/docs/ja/mcp#automatic-reconnection)する場合です。接続されたサーバーは、そのツールリストを変更する[動的ツール更新](/docs/ja/mcp#dynamic-tool-updates)をプッシュすることもできます。

MCP 設定を編集しても、それ自体ではキャッシュは変わりません。新しい設定は、サーバーが接続または切断される再起動後にのみ有効になります。

<h3 id="enabling-or-disabling-a-plugin">
  プラグインの有効化または無効化
</h3>

[プラグイン](/docs/ja/plugins/overview)を有効または無効にする場合、変更のコストはプラグインが提供するコンポーネントタイプによって異なります。以下のケースは、各コンポーネントタイプ、Claude Code が変更を適用するタイミング、および同じセッション内でプラグインを再度無効にする場合の動作をカバーしています。

<h4 id="plugin-components-that-keep-the-cache">
  キャッシュを保持するプラグインコンポーネント
</h4>

Claude Code は、プラグインのスキル、コマンド、エージェント、フック、モニター、またはテーマのキャッシュを無効化することはありません。既存の会話の後にそのコンテンツを追加するため、次のリクエストはそのコンテンツに対して支払い、その前のすべてをキャッシュから読み込みます。

<h4 id="plugins-that-provide-mcp-servers">
  MCP サーバーを提供するプラグイン
</h4>

[MCP サーバー](/docs/ja/plugins/components#mcp-servers)を提供するプラグインを有効または無効にする場合、Claude Code は[MCP サーバーを接続または切断する](#connecting-or-disconnecting-an-mcp-server)場合と同じルールに従います。

* Claude Code がサーバーのツールを遅延させる場合、キャッシュを保持します。
* Claude Code がそれらをプレフィックスに読み込む場合、次のリクエストは会話全体を再読み込みします。

<h4 id="code-intelligence-plugins">
  コード インテリジェンス プラグイン
</h4>

[コード インテリジェンス プラグイン](/docs/ja/plugins/code-intelligence)を有効にすると、Claude は[LSP ツール](/docs/ja/tools-reference#lsp-tool-behavior)を取得します。

<h4 id="when-plugin-changes-apply">
  プラグイン変更が適用される場合
</h4>

`/plugin` メニューで行った変更は、[`/reload-plugins`](/docs/ja/plugins/cli-reference#reload-plugins)を通じて行われます。Claude Code はメニューを閉じるときにこれを実行します。追加されたアナウンスメントまたは完全な再読み込みのいずれかのコストを、変更が適用された後の最初のターンで支払います。Claude Code は変更を独自に適用することもできます。

* `command` ソースを持つプラグインの場合、Claude Code は[プラグイン自体を再読み込みできます](/docs/ja/plugins/loading#when-a-command-source-re-runs)。
* [`/plugin` インターフェースからプラグインをインストール](/docs/ja/plugins/install#install-a-plugin)する場合、Claude Code はインストール中にそれを有効化できます。インストール概要は、それが行われたかどうかを示します。
* v2.1.246 以降で [`/cd`](/docs/ja/permissions#move-the-session-to-another-directory)でセッションを移動する場合、Claude Code は新しいディレクトリの設定が有効にするプラグインを移動の一部として適用します。`/reload-plugins` が保持する完全な再読み込み警告なしで。
* インタラクティブセッションでは、`--plugin-dir` で渡した[プラグインのフォルダ](/docs/ja/plugins/create#load-a-directory-or-archive-for-one-session)でプラグインを追加または削除する場合、変更は直ちに適用されます。それを適用すると完全な再読み込みがトリガーされる場合、Claude Code は変更を保持し、`/reload-plugins` を実行するための通知を表示します。Claude Code v2.1.265 以降が必要です。

`/reload-plugins` が実行され、再読み込みが完全な再読み込みをトリガーする場合、Claude Code は警告を表示し、再読み込みを適用しません。`/reload-plugins --force` を実行して、とにかくそれを適用します。

`/reload-plugins` は、デスクトップアプリ、Agent SDK、および [`-p` を使用した非インタラクティブモード](/docs/ja/headless)など、インタラクティブターミナルのないセッションでも実行されます。セッションに直接入力する場合です。Claude Code v2.1.260 以降が必要です。

これらのセッションでは、再読み込みはプラグイン MCP サーバーの変更を除くすべてを適用します。これらは[次のセッションで有効になり](/docs/ja/plugins/cli-reference#reload-plugins)、セッション中に完全な再読み込みのコストは決してかかりません。

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  セッション内で有効化してから無効化するプラグイン
</h4>

セッションの前半で有効にしたプラグインを無効にする場合、Claude Code は以前のリクエスト形状を復元します。そのプレフィックスがまだ[キャッシュ有効期間](#cache-lifetime)内にある場合、次のリクエストは再構築する代わりに、より古いキャッシュエントリを読み込みます。

<h3 id="denying-an-entire-tool">
  ツール全体の拒否
</h3>

`Bash` または `WebFetch` のような裸のツール名を[拒否ルール](/docs/ja/permissions#manage-permissions)として追加すると、Claude はそのツールを次のリクエストから呼び出すことができなくなります。`/permissions` を通じて追加するか、[設定ファイルを直接編集](/docs/ja/settings#when-edits-take-effect)するかのいずれかです。これには、ターンの途中で `/permissions` を通じて追加するルールが含まれます。

[ツール検索](/docs/ja/mcp#scale-with-mcp-tool-search)がアクティブな場合（サポートされているモデルではデフォルト）、リクエストのツール定義は変更されず、キャッシュされたプレフィックスは保持されます。ツール検索が利用できないか無効になっている場合、Claude Code は次のリクエストから定義を削除し、キャッシュを無効化します。ルールを後で削除する場合も同じです。

ツール名の位置で一致する拒否ルールのみがこの効果を持ちます。裸のツール名、同等の `Bash(*)` 形式、または [`"*"` のようなツール名グロブ](/docs/ja/permissions#tool-name-wildcards)です。`"mcp__*"` のような MCP ツールのみに一致するグロブは、同じ方法でそれらのツールをブロックします。`Bash(rm *)` のようなスコープ付き拒否ルール、およびすべての許可とアスクルールは、Claude が見るツールを変更しません。Claude Code は Claude が呼び出しを試みるときにそれらをチェックし、プレフィックスはそのまま保持されます。

<h3 id="compacting-the-conversation">
  会話のコンパクト化
</h3>

[コンパクト化](/docs/ja/context-window#what-survives-compaction)は、メッセージ履歴を概要に置き換えます。設計上、これは会話レイヤーを無効化します。次のリクエストには、古いものと共通のプレフィックスを共有しない、新しく短い履歴があるためです。Claude Code はシステムプロンプトレイヤーを再利用します。ただし、会話が[そうでなければ変更されるシステムプロンプトを保持しながら再開された](#resuming-a-session)場合を除きます。その場合、最初のコンパクト化は現在のプロンプトに切り替わり、そのレイヤーは 1 回再構築されます。プロジェクトコンテキストをディスクから再読み込みします。これはセッション開始以降 CLAUDE.md とメモリが変更されていない場合にのみキャッシュヒットします。

概要を生成するために、Claude Code は会話と同じシステムプロンプト、ツール、および履歴を持つ別のリクエストを送信します。さらに、最終ユーザーメッセージとして追加された要約化指示があります。キャッシュが温かい間、そのリクエストはキャッシュからプレフィックスを読み込むため、セッション中の `/compact` はコンテキストサイズが示唆するコストの一部であり、ほとんどの時間を概要の生成に費やします。

[キャッシュ有効期間](#cache-lifetime)より長い休止の後、読み込むキャッシュは残っていないため、要約化リクエストはキャッシュされていない入力として完全な履歴を再処理します。これが `/compact` が[古いセッションを再開](/docs/ja/sessions#resume-from-a-summary)するときに最もコストがかかる理由です。温かいケースと冷たいケースの両方で、コンパクト化後のターンは、はるかに短い概要のためだけに会話キャッシュを再構築するため、そのターンは遅い部分ではありません。

<Tip>
  コンパクト化は、破棄するコンテキストがもう必要ないコンテンツである場合、あなたに有利に機能します。オーバーヘッドが発生するタイミングを選択するには、タスク間などの作業の自然な区切りで `/compact` を実行します。自動コンパクト化がタスク中にトリガーされるのを待つ代わりに。完全に放棄したいパスを下った場合は、代わりに[`/rewind`](#rewinding-the-conversation)を使用して以前のターンに戻ります。巻き戻しは、コンパクト化が行うように新しいものを構築するのではなく、既にキャッシュされているプレフィックスに切り詰めます。
</Tip>

<h3 id="accumulating-many-images">
  多くの画像の蓄積
</h3>

API は、各リクエストが実行できる画像と PDF の数を制限しています。現在の数については、API ドキュメントの[リクエスト制限](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits)を参照してください。Claude Code はリクエスト内の画像と PDF の合計サイズもキャップするため、大きなスクリーンショットは小さいものより少ない画像で制限に達します。

次のリクエストがいずれかの制限を超える場合、Claude Code は送信する内容から最も古い画像と PDF のバッチを削除します。これにより、再度削除する必要があるまでさらに多くの余地が生まれます。Claude はもう削除された画像を見ることができません。Claude がそれらの 1 つを再度必要とする場合は、再度共有してください。

画像を削除すると、それらを保持していたメッセージが変更されるため、次のリクエストはそれらのメッセージの最も早いものから会話を再処理します。Claude Code はバッチごとに削除するため、新しいスクリーンショットごとに 1 つではなく、バッチごとに 1 つの遅いターンが表示されます。

<h3 id="upgrading-claude-code">
  Claude Code のアップグレード
</h3>

新しい Claude Code バージョンは通常、システムプロンプトまたはツール定義を更新するため、アップグレード後に開始する最初の会話はトップからキャッシュを構築します。[自動更新](/docs/ja/setup#auto-updates)は新しいバージョンをバックグラウンドでダウンロードしますが、次の起動時に適用され、セッション中には決して適用されません。そのため、セッション中の驚きではなく、再起動後のキャッシュされていない最初のターンとしてこれが表示されます。`DISABLE_AUTOUPDATER=1` を設定して、アップグレードが適用されるタイミングを制御します。

<Note>
  アップグレード前に開始した会話を再開するコストについては、[セッションの再開](#resuming-a-session)を参照してください。
</Note>

<h2 id="actions-that-keep-the-cache">
  キャッシュを保持するアクション
</h2>

これらのアクションは、会話の末尾に追加されるか、リクエストにまったく触れません。CLAUDE.md の編集など、その中には `/clear`、`/compact`、または再起動までキャッシュを保持する理由が、変更が実行中のセッションに到達しない理由と同じであるものもあります。

* [リポジトリ内のファイルを編集する](#editing-files-in-your-repository)
* [セッション中に CLAUDE.md を編集する](#editing-claude-md-mid-session)
* [権限モードを変更する](#changing-permission-mode)
* [出力スタイルを変更する](#changing-output-style)
* [スキルとコマンドを呼び出す](#invoking-skills-and-commands)
* [`/recap` を実行する](#running-%2Frecap)
* [会話を巻き戻す](#rewinding-the-conversation)
* [サブエージェントを生成する](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  リポジトリ内のファイルを編集する
</h3>

ファイルの内容は Claude がそれらを読むときにのみコンテキストに入り、読み取りは会話に追加されます。Claude が以前読んだファイルを編集しても、履歴内の以前の読み取りは遡及的に変更されません。代わりに、Claude Code はファイルが変更されたことを示す `<system-reminder>` を追加し、必要に応じて Claude がそれを再度読み取ります。

<h3 id="editing-claude-md-mid-session">
  セッション中に CLAUDE.md を編集する
</h3>

プロジェクトルートとユーザーレベルの CLAUDE.md ファイルはセッション開始時に 1 回読み取られ、メモリに保持されます。セッション中にそれらを編集してもキャッシュは無効化されませんが、編集も適用されません。Claude はセッション開始時に読み込まれたバージョンで動作し続けます。新しいコンテンツは次の `/clear`、`/compact`、または再起動時に読み込まれます。

[サブディレクトリ内のネストされた CLAUDE.md ファイル](/docs/ja/memory)と[`paths:` frontmatter を持つルール](/docs/ja/memory#path-specific-rules)は後で、Claude が最初に一致するファイルを読むときに読み込まれます。読み込まれる前にそれを編集すると、実際に効果があります。読み込まれた後、コンテンツは会話履歴の一部であるため、セッション中の編集は遡及的にそれを変更しません。

<h3 id="changing-permission-mode">
  権限モードを変更する
</h3>

[権限モード](/docs/ja/permission-modes)（手動から編集を受け入れるなど）を切り替えても、システムプロンプトやツール定義は変更されないため、モード変更はキャッシュセーフです。例外は [`opusplan`](/docs/ja/model-config#opusplan-model-setting) モデル設定を使用した plan mode です。これはプラン モードに入るか終了するときに、モデルを Opus と Sonnet の間で切り替えます。これにより、モード切り替えは[モデル切り替え](#switching-models)になります。

<h3 id="changing-output-style">
  出力スタイルを変更する
</h3>

セッション中に [`/output-style`](/docs/ja/output-styles#change-your-output-style)、`/config`、または `outputStyle` 設定で[出力スタイル](/docs/ja/output-styles)を切り替えると、Claude は次のメッセージから新しいスタイルを使用します。Claude Code は新しいスタイルの指示をメッセージとして会話に配信するため、そのリクエストはシステムプロンプトと以前の会話をキャッシュから読み取ります。

v2.1.251 より前では、セッション中のスタイル切り替えはキャッシュを保持していましたが、`/clear` を実行するか新しいセッションを開始するまで適用されませんでした。

<h3 id="invoking-skills-and-commands">
  スキルとコマンドを呼び出す
</h3>

[スキル](/docs/ja/skills)と[コマンド](/docs/ja/commands)は、呼び出しポイントでユーザーメッセージとして指示を挿入します。会話内の以前のものは何も変わりません。frontmatter で `model` という名前を付けたスキルまたはコマンドは、そのターンの[モデル切り替え](#switching-models)になる可能性があります。

<h3 id="running-/recap">
  `/recap` を実行する
</h3>

[`/recap`](/docs/ja/interactive-mode#session-recap)はターミナルに表示するための概要を生成します。`/compact` とは異なり、メッセージ履歴を置き換えるのではなく、コマンド出力として概要を追加するため、キャッシュされたプレフィックスはそのまま残ります。

<h3 id="rewinding-the-conversation">
  会話を巻き戻す
</h3>

[`/rewind`](/docs/ja/checkpointing)は会話を以前のターンに切り詰めます。残りの履歴は、その時点でキャッシュが構築されたのと同じコンテンツであり、システムプロンプトとプロジェクトコンテキストレイヤーは変更されないため、次のリクエストは以前のキャッシュエントリにヒットします。それ以降のすべてのターンはそのプレフィックスを読み取っており、元のターンが TTL より長い前であっても、エントリをウォーム状態に保ちました。

会話と一緒にファイルチェックポイントを復元しても、キャッシュに対する個別の効果はありません。ファイルの内容は、[リポジトリ内のファイルを編集する](#editing-files-in-your-repository)と同じように、Claude がそれらを読むときにのみコンテキストに入ります。

<h2 id="resuming-a-session">
  セッションの再開
</h2>

[セッションを再開](/docs/ja/sessions#resume-a-session)する場合、Claude Code は会話全体を再度送信し、リクエストはキャッシュから、そのプレフィックスの変更されていない部分で、かつ [キャッシュの有効期限](#cache-lifetime)内にある部分を読み込みます。このページの上部にあるレイヤーテーブルは、各レイヤーで何が変わるかを示しています。

システムプロンプトは [Claude Code のアップグレード](#upgrading-claude-code)後、または再開時に異なる [`--append-system-prompt`](/docs/ja/cli-reference#system-prompt-flags) テキストがある場合に変更されます。デフォルトでは、再開された会話は開始時のシステムプロンプトを保持するため、その履歴は同じプロンプトの背後にあり、会話がコンパクト化されるか新しい会話で変更が有効になります。[再開されたセッションのシステムプロンプトフラグ](/docs/ja/cli-reference#system-prompt-flags-in-resumed-conversations)は Claude Code がすべてのリクエストでプロンプトを再構築する場合をカバーしています。

<h2 id="cache-lifetime">
  キャッシュライフタイム
</h2>

キャッシュされたプリフィックスは、非アクティブ期間後に期限切れになります。キャッシュにヒットするすべてのリクエストはタイマーをリセットするため、作業を続ける限りキャッシュは温かく保たれます。十分に長いギャップの後、次のリクエストは完全な入力を再計算し、キャッシュを再確立します。これが、立ち去った後の最初のターンが顕著に遅い理由です。

Pro または Max プランでは、長い休止後に大規模なセッションを再開する場合、Claude Code は[サマリーから再開する](/docs/ja/sessions#resume-from-a-summary)ことを提案するため、後続のリクエストは完全な履歴を保持する必要がありません。

Time to Live（TTL）は、キャッシュが生き残るギャップの長さを制御します。API は 2 つを提供します。5 分の TTL と、より長い休止を通じてキャッシュを温かく保つ[1 時間の TTL](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration)ですが、[キャッシュ書き込みをより高いレートで請求](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing)します。より長い TTL は、セッションをアイドル状態のままにして戻ってくる場合に役立ちます。期限切れのプリフィックスが発生する再処理をスキップできるためです。5 分を超えてアイドル状態にならない短いバースト作業では、より高い書き込みレートが適用され、より長いキャッシュライフタイムが未使用のままになるため、コストが高くなります。

<h3 id="which-ttl-each-request-gets">
  各リクエストが取得する TTL
</h3>

Claude Code はリクエストごとに TTL を決定し、すべてのリクエストは 2 つの固定バケットのいずれかに該当します。

* **メイン会話**: インタラクティブなターン、非インタラクティブな `-p` 実行、Agent SDK ターン、およびそれらと共にインラインで実行される Claude Code ヘルパー
* **その他すべて**: [サブエージェント](/docs/ja/sub-agents)、[ワークフロー](/docs/ja/workflows)、プロセス内[チームメイト](/docs/ja/agent-teams)、フォーク、圧縮、セッションタイトルなど、その会話の外で Claude Code が行うリクエスト

TTL を自分で選択しない限り、Claude Code は Claude サブスクリプション内でプランに含まれる使用量内でのみ 1 時間の TTL をリクエストします。そこでメイン会話に対して 1 時間をリクエストし、Anthropic がサーバー側で制御する小さなヘルパーリクエストセットをリクエストします。このテーブルは、両方の種類の請求下での各バケットのデフォルト TTL を示しています。

| リクエストバケット | Claude サブスクリプション、プラン使用量内        | 使用クレジット、API キー、またはクラウドプロバイダー |
| --------- | ------------------------------- | ---------------------------- |
| メイン会話     | 1 時間                            | 5 分                          |
| その他すべて    | 5 分（ただし、サーバー制御のヘルパーリクエストは 1 時間） | 5 分                          |

プランの使用量制限を超えて、Claude Code が[使用クレジット](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)を引き出すと、その使用量に対して請求されるため、Claude Code はメイン会話をより安い 5 分の TTL に低下させます。そこで 1 時間の TTL を保つには、[TTL を自分で選択](#choose-the-ttl-yourself)してください。

<h3 id="choose-the-ttl-yourself">
  TTL を自分で選択する
</h3>

どちらのバケットに対しても TTL を設定できます。各コントロールは `5m` または `1h` を取り、Claude Code は他の値を無視します。

* **メイン会話**: [`promptCacheTtl`](/docs/ja/settings-reference#promptcachettl) 設定、または `CLAUDE_CODE_PROMPT_CACHE_TTL` [環境変数](/docs/ja/env-vars)
* **その他すべて**: [`subagentPromptCacheTtl`](/docs/ja/settings-reference#subagentpromptcachettl) 設定、または `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` 環境変数

両方の設定と両方の環境変数には Claude Code v2.1.242 以降が必要です。API キーで署名するか、クラウドプロバイダーを使用する場合は、`promptCacheTtl` を `1h` に設定して、メイン会話に 1 時間のキャッシュを与えます。その外のリクエストは、そのバケットに対しても TTL を選択するまで 5 分のデフォルトを保ちます。

複数のコントロールが適用される場合、Claude Code はこの順序で最初にマッチするものを取ります。

1. `FORCE_PROMPT_CACHING_5M=1`。両方のバケットに対して 5 分を強制します
2. バケットの環境変数
3. バケットの設定
4. サブエージェントのリクエストの場合、サブエージェントの [`experimental` frontmatter フィールド](/docs/ja/sub-agents#supported-frontmatter-fields)の `cacheTtl` 値。Claude Code v2.1.248 以降が必要です。Claude サブスクリプションが使用クレジットを使用している間、Claude Code はそこの `1h` を無視します
5. `ENABLE_PROMPT_CACHING_1H=1`。両方のバケットに対して 1 時間をリクエストします
6. [リクエストのバケットのデフォルト](#which-ttl-each-request-gets)

キャッシング動作をデバッグする場合、2 つの TTL を比較する場合、または[管理設定](/docs/ja/managed-settings)で設定された長い TTL をオーバーライドする場合は、`FORCE_PROMPT_CACHING_5M=1` を設定します。

メイン会話のキャッシュ書き込みが使用した TTL を確認するには、`claude -p "hello" --output-format json` を実行し、結果の `usage.cache_creation` を読みます。Claude Code は 1 時間のキャッシュ書き込みを `ephemeral_1h_input_tokens` の下で報告し、5 分のキャッシュ書き込みを `ephemeral_5m_input_tokens` の下で報告します。

`ANTHROPIC_BASE_URL` で設定した LLM ゲートウェイを通じて、1 時間のリクエストの一部は `anthropic-beta` ヘッダーで移動するため、ゲートウェイを[そのヘッダーを変更されずに転送](/docs/ja/llm-gateway-protocol#request-headers)するように設定します。1 時間の TTL は[Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway#availability-and-limitations)を通じて利用できません。Amazon Bedrock では、プロンプトキャッシングサポート、最小キャッシュ可能プリフィックス長、および 1 時間の TTL 可用性はすべてモデルによって異なります。キャッシュトークン数がゼロのままの場合は、Amazon Bedrock ドキュメントの[サポートされているモデル、リージョン、制限](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)を確認してください。

<h2 id="cache-scope">
  キャッシュスコープ
</h2>

Claude Code では、キャッシュは事実上 1 つのマシンとディレクトリにスコープされます。各会話は作業ディレクトリ、プラットフォーム、シェル、OS バージョンを保持し、システムプロンプトは自動メモリパスに名前を付けるため、異なるディレクトリの 2 つのセッションは異なるプリフィックスを構築し、互いのキャッシュをミスします。これには同じリポジトリの worktrees が含まれます。各 worktree は独自の作業ディレクトリを持つためです。

同じディレクトリで並行して実行するセッションは、一致するプリフィックスを構築し、互いのキャッシュを読み取ります。順序付きセッションは、起動時に取得された git ステータススナップショットが一致する場合にのみプリフィックスを共有します。各会話はそのスナップショットからブランチと最近のコミットも保持するためです。

基礎となる API キャッシュはより広いです。キャッシュは組織間で分離され、一部のプロバイダーでは、[組織内のワークスペース間](https://platform.claude.com/docs/ja/build-with-claude/prompt-caching#cache-storage-and-sharing)で分離されます。これらの境界内で、同じモデルとプリフィックスを持つ 2 つのリクエストは同じキャッシュを読み取ります。自動化されたプロセスのフリートを実行する Agent SDK 呼び出し元については、[ユーザーとマシン間でプロンプトキャッシングを改善](/docs/ja/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)を参照して、システムプロンプトのマシンごとのセクションを抑制し、マシン間でキャッシュを共有します。

<h2 id="check-cache-performance">
  キャッシュパフォーマンスを確認する
</h2>

キャッシュパフォーマンスは、API がすべての応答で報告する 2 つのトークン数として表示されます。最も直接的な方法は、`current_usage` オブジェクトを読み取る[statusline スクリプト](/docs/ja/statusline)を監視することです。

| フィールド                         | 意味                                                                                                                            |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | このターンでキャッシュに書き込まれたトークン。キャッシュ書き込みレートで請求されます                                                                                    |
| `cache_read_input_tokens`     | このターンでキャッシュから提供されたトークン。モデルの[キャッシュされたトークンレート](https://platform.claude.com/docs/en/about-claude/pricing)で請求されます。標準入力レートより低くなります |

読み取りから作成への比率が高いほど、キャッシングが機能しています。作成がターンごとに高いままの場合、プリフィックスで何かが変更されています。[キャッシュを無効にするアクション](#actions-that-invalidate-the-cache)セクションは、通常の原因をリストします。

セッションごとのサマリーについては、`/usage` を実行してください。メインの会話の最初の応答の後、Claude Code はセッションブロックに[`Prompt cache (main)` 行](/docs/ja/costs#prompt-cache-statistics)を追加し、セッションのヒット率、ミス数、およびキャッシュが現在ウォームであるかどうかを表示します。statusline スクリプトは、[`prompt_cache` オブジェクト](/docs/ja/statusline#prompt-cache-fields)から同じ数値を読み取ることができます。どちらも Claude Code v2.1.251 以降が必要です。

`Prompt cache (main)` 行は、Claude Code が識別できる場合、最後のミスの可能性のある原因も名前を付けます。例えば `likely cause: tool definitions changed` のようにです。可能性のある原因テキストには Claude Code v2.1.260 以降が必要です。

組織全体の可視性については、OpenTelemetry エクスポーターはユーザーとセッションごとにキャッシュ読み取りと作成トークンを報告します。メトリックとイベント属性リファレンスについては、[使用状況の監視](/docs/ja/monitoring-usage)を参照してください。

<h2 id="subagents-and-the-cache">
  サブエージェントとキャッシュ
</h2>

[サブエージェント](/docs/ja/sub-agents)は、親とは別に、独自のシステムプロンプトとツールセットを持つ独自の会話を開始します。最初のリクエストは親のキャッシュを読み取りません。2 つのプリフィックスが異なるためです。また、独自のターン全体で独自のキャッシュを温めます。サブエージェントはメイン会話の [TTL バケット](#which-ttl-each-request-gets)の外にあるため、サブスクリプション上でも 5 分間の TTL を取得します。[より長い TTL を選択](#choose-the-ttl-yourself)するまでです。

親のキャッシュは影響を受けません。親の側から、サブエージェントの呼び出しと結果は会話に追加され、親のプリフィックスはそのままです。

一方、[フォーク](/docs/ja/sub-agents#fork-the-current-conversation)は、親のシステムプロンプト、ツール、会話履歴を正確に継承するため、最初のリクエストは親のキャッシュを読み取ります。

他のリクエストも、以前のリクエストがキャッシュしたプリフィックスを読み取ることができます。

* **セッションコピー**: [`/fork`](/docs/ja/agent-view#copy-the-session-with-%2Ffork)でコピーしたセッションは、コピーされた会話の最後にメッセージとして分離命令を受け取るため、元の会話が構築したキャッシュはそのままです。
* **コンパクト化**: [会話のコンパクト化](#compacting-the-conversation)で説明されている要約呼び出しは、同じプリフィックス共有アプローチを使用します。
* **再開されたサブエージェント**: Claude が[サブエージェントを再開](/docs/ja/sub-agents#resume-subagents)する場合、再開実行の最初のリクエストは、元の実行が温めたキャッシュを読み取ることができます。
* **ワークフローファンアウト**: [ワークフローファンアウト](/docs/ja/workflows#prompt-caching-in-a-fan-out)の同じプリフィックスエージェントでは、Claude Code はデフォルトで最初のエージェント以外をすべて最大 5 秒間保持するため、最初のエージェントがキャッシュしたプリフィックスを読み取ることができます。

<h2 id="disable-prompt-caching">
  プロンプトキャッシングを無効にする
</h2>

キャッシング動作を特定のモデルまたはプロバイダーでデバッグするときは、キャッシングを無効にすることが時々役立ちます。オフにするには、これらの環境変数のいずれかを `1` に設定します。

| 変数                              | 効果                 |
| ------------------------------- | ------------------ |
| `DISABLE_PROMPT_CACHING`        | すべてのモデルに対して無効にする   |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Haiku のみに対して無効にする  |
| `DISABLE_PROMPT_CACHING_SONNET` | Sonnet のみに対して無効にする |
| `DISABLE_PROMPT_CACHING_OPUS`   | Opus のみに対して無効にする   |
| `DISABLE_PROMPT_CACHING_FABLE`  | Fable のみに対して無効にする  |

組織全体でキャッシングポリシーを設定するには、これらのいずれかまたは [TTL 変数](#cache-lifetime)を [管理設定](/docs/ja/managed-settings)の `env` ブロックに入れます。通常の使用では、キャッシングを有効のままにしてください。

<h2 id="related-resources">
  関連リソース
</h2>

* [Claude Code の構築から学んだ教訓: プロンプトキャッシングがすべて](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): Plan mode、遅延ツール読み込み、コンパクト化の設計根拠
* [コンテキストウィンドウを探索](/docs/ja/context-window): コンテキストに読み込まれるもの、いつ読み込まれるか
* [トークン使用量を削減](/docs/ja/costs#reduce-token-usage): コンテキストサイズを管理するためのキャッシング以外の戦略
* [コストを追跡および削減](/docs/ja/agent-sdk/cost-tracking): Agent SDK 呼び出し元のキャッシュトークン追跡と TTL 設定
* [プロンプトキャッシング](https://platform.claude.com/docs/ja/build-with-claude/prompt-caching): 基礎となる API メカニズム、ブレークポイント、価格設定
