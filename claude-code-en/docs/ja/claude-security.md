> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# コードベースの脆弱性をスキャンする

> Claude Security プラグインをインストールして、Claude Code セッション内でコードベースの脆弱性をスキャンし、検出結果をレビューして適用できるパッチに変換します。

Claude Security プラグインは、Claude Code セッション内でコードベースのマルチエージェント脆弱性スキャンを実行します。Claude エージェントのチームがアーキテクチャをマッピングし、脅威モデルを構築し、脆弱性を検出し、すべての検出結果を独立してレビューしてからレポートを作成します。プラグインを使用して、リポジトリ全体をスキャンするか、[変更のみをスキャン](#scan-only-your-changes)することができます。例えば、ブランチの diff、プルリクエストの diff、または単一のコミットなど、選択した検出結果をレビューして自分で適用できるパッチに変換します。

プラグインはセッション内でローカルに実行され、Claude Code で利用可能なモデルを使用し、各スキャンはプランの使用制限にカウントされます。リポジトリを監視するマネージドサービスが必要な場合、または [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) でスキャンを実行したい場合は、Enterprise プランで利用可能な [Claude Security](https://claude.com/product/claude-security) プロダクトを参照してください。プラグインは、GitLab や Bitbucket でホストされているリポジトリ、または受信接続を許可しないネットワーク上のリポジトリなど、マネージドプロダクトが到達できないコードに到達します。

プラグインは、Claude Code に既に存在するレビューツールとも異なります。[security guidance プラグイン](/docs/ja/security-guidance)は Claude が記述するコードをレビューし、[`/security-review`](/docs/ja/commands#all-commands)はブランチに対して単一パスを実行し、[Code Review](/docs/ja/code-review)はプルリクエストをレビューします。レイヤーがどのようにスタックするかについては、[プラグインが他のセキュリティツールとどのように適合するか](#how-the-plugin-fits-with-other-security-tools)を参照してください。

<h2 id="prerequisites">
  前提条件
</h2>

プラグインを実行するには、以下が必要です。

* 有料プラン。スキャンがエージェントをオーケストレーションするために使用する [動的ワークフロー](/docs/ja/workflows)用です。Pro では、`/config` の Dynamic workflows 行から有効にしてください。
* Python 3.9 以降が `PATH` で `python3` として利用可能です。`python3 --version` で確認してください。プラグインのツーリングは Python 標準ライブラリのみを使用するため、何もインストールされません。
* Linux、macOS、または Windows。
* Git（変更スキャンおよび検出結果をパッチに変換するため）。これらのジョブは他のバージョン管理システムをサポートしていません。完全スキャンは、バージョン管理の有無にかかわらず、任意のディレクトリで機能します。

<h2 id="install-the-plugin">
  プラグインをインストールする
</h2>

Claude Code セッションで、[公式 Anthropic マーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)からインストールします。

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

コマンドはプラグインの詳細を開き、[インストールスコープ](/docs/ja/plugins/install#install-a-plugin)を選択してインストールを開始します。

インストールが失敗した場合、修正は Claude Code が報告するメッセージによって異なります。

* `Marketplace "claude-plugins-official" not found` と報告された場合は、`/plugin marketplace add anthropics/claude-plugins-official` でマーケットプレイスを追加してから、インストールを再試行してください。
* マーケットプレイスで [プラグインが見つからないと報告された](/docs/ja/plugins/install#install-a-plugin)場合は、プラグイン名のタイプミスを確認してください。

インストール概要を確認してください。`Run /reload-plugins to activate.` と報告された場合は、[プラグインの変更を再起動なしで適用](/docs/ja/plugins/cli-reference#reload-plugins)を参照して、現在のセッションでプラグインをアクティブにしてください。

プラグインがアクティブになり、[コードベースをスキャンして修正](#scan-and-fix-your-codebase)する準備ができました。

<h3 id="uninstall-the-plugin">
  プラグインをアンインストールする
</h3>

プラグインを削除するには、`/plugin` メニューからアンインストールするか、ターミナルで `claude plugin uninstall claude-security` を実行します。

<h2 id="scan-and-fix-your-codebase">
  コードベースをスキャンして修正する
</h2>

プラグインは 1 つのコマンド `/claude-security` を追加します。これは 3 つのジョブのメニューを開きます。コードベースのスキャン、変更セットのスキャン、パッチの提案です。ハッピーパスは完全スキャンを実行してから、その検出結果をパッチに変換します。

<Steps>
  <Step title="Claude Security メニューを開く">
    `/claude-security` を実行して、**Scan codebase** を選択します。
  </Step>

  <Step title="スキャンする内容を選択する">
    プラグインはまずリポジトリを読み込み、その後、リポジトリ全体またはフォーカスされた領域を提供します。各オプションのファイル数と相対コストが記載されています。リポジトリ全体を選択するか、「I don't know」と答えると、プラグインはリポジトリのサイズに合わせて適切なデフォルトを選択します。
  </Step>

  <Step title="実行を確認する">
    スキャンには時間がかかる場合があり、かなりの数のトークンを使用する可能性があり、完了するまで Claude Code を開いたままにする必要があります。確認するまで何も実行されません。
  </Step>

  <Step title="レポートを読む">
    スキャンが実行されている間、各ステージが開始されるたびに報告され、詳細は [`/workflows`](/docs/ja/workflows) で利用可能です。結果は、リポジトリ内のタイムスタンプ付きディレクトリに格納され、[スキャン結果を読む](#read-the-scan-results)で説明されています。
  </Step>

  <Step title="検出結果をパッチに変換する">
    `/claude-security` を再度実行して、**Suggest patches** を選択してから、対処する検出結果を選択します。レビュー済みパッチはレポートの `patches/` フォルダに格納されます。[検出結果を修正](#fix-findings)では、各パッチがどのように構築およびレビューされるかについて説明しています。
  </Step>

  <Step title="受け入れたパッチを適用する">
    シェルから `git apply` で各パッチを適用します。独自のプルリクエストで実行してください。パッチは自動的に適用されることはありません。
  </Step>
</Steps>

メニューから開始する必要はありません。コマンドの引数として直接ジョブを要求することができます。例えば `/claude-security scan my branch` のように、またはプレーンテキストで「scan commit abc1234」のように要求できます。プラグインは [auto mode](/docs/ja/permission-modes) で最適に機能します。これにより、スキャンのエージェントは各ステップで権限プロンプトなしで進行できます。

<h3 id="scan-only-your-changes">
  変更のみをスキャンする
</h3>

ブランチにベースにないコミットがある場合、`/claude-security` メニューはその差分のみをスキャンするオプションを提供するため、マージ前にブランチを確認できます。開いているプルリクエストの 1 つをスキャンするか、「scan commit abc1234」のように要求して単一のコミットをスキャンすることもできます。コミットされた変更のみがスキャンされます。進行中の編集をコミットまたはスタッシュするか、作業ツリーを読み込む完全スキャンを実行してください。

変更スキャンには git リポジトリが必要です。バージョン管理されていないディレクトリの完全スキャンは引き続き機能します。開いているプルリクエストを見つけることは、ネットワークに到達する唯一のステップであり、セッションが既に GitHub CLI を実行する権限を持ち、`gh` がサインインしている場合にのみ提供されます。

<h3 id="scope-large-repositories">
  大規模なリポジトリのスコープを設定する
</h3>

大規模なリポジトリでは、ツリー全体ではなく、一度に 1 つの領域をスキャンします。プラグインが提供するフォーカスされたスコープの 1 つを選択します。例えば、API レイヤーまたは認証コードなど。実行は選択内容に合わせてサイズが調整されます。レポートのカバレッジセクションには、何が検査され、何が検査されなかったかが記載されています。別の領域で別のスキャンを実行してください。

<h3 id="read-the-scan-results">
  スキャン結果を読む
</h3>

すべてのスキャンは、リポジトリ内のタイムスタンプ付き `CLAUDE-SECURITY-<timestamp>/` ディレクトリに結果を書き込みます。

* **`CLAUDE-SECURITY-RESULTS.md`**: レポート。各検出結果の ID（`F1` など）、影響、悪用シナリオ、重大度、信頼度、推奨事項が含まれています。
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: 同じ検出結果を機械可読形式で、1 行に 1 つの JSON オブジェクト。
* **`CLAUDE-SECURITY-RESULTS.sarif`**: 同じ検出結果を [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) ログとして。GitHub コードスキャンおよび標準を読む他のツール用です。スキャンは検出結果を [CWE](https://cwe.mitre.org/) 弱点カテゴリの下に分類します。
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: リビジョンスタンプ。スキャンされたコミット、どの程度の労力で、コミットされていない変更がスキャンされたツリーの一部であったかどうか、実行がどの程度徹底的に検証されたかを記録します。これにより、レポートは常にそれが説明するコードに関連付けられます。バージョン管理外のスキャンは、コミットの代わりに `UNVERSIONED` をスタンプします。

そのディレクトリはスキャンがチェックアウトに加える唯一の変更であり、独自の `.gitignore` を含むため、迷った `git add` がレポートをコミットに掃き込むことはありません。監査証跡のためにレポートを履歴に保持するには、その 1 つの `.gitignore` ファイルを削除して、ディレクトリを他のものと同様にコミットします。

検出結果は、独立した検証エージェントが分析した後にのみレポートに表示されます。これにより、レポートは短く、読む価値があります。スキャンは非決定的です。同じコードの 2 つのスキャンは異なる検出結果を表示できます。スキャンを定期的に実行し、リビジョンスタンプを使用して、各レポートをそれがカバーする正確なコードと設定に属性付けします。

<h2 id="fix-findings">
  検出結果の修正
</h2>

`/claude-security` メニューから **Suggest patches** を選択するか、「finding F3 を修正」などのプレーンテキストで質問して、修正フローを開始し、レポートからどの検出結果に対処するかを選択します。パッチはコミットされたコードに対して構築され、レポートはまだ現在のコードを説明している必要があります。その後コードが変更された検出結果はスキップされ、プラグインは古いレポートからのパッチ適用ではなく新しいスキャンを提供します。各パッチはリポジトリのスクラッチコピーで作成されるため、パッチを自分で適用するまでソースファイルは変更されません。

配信前に、各パッチはそれを作成したエージェントとは独立したエージェントによってレビューされます。このエージェントはコードにテストがある場合、変更に対してプロジェクトのテストを実行し、導入される可能性のある新しい問題がないか diff を独自に読みます。パッチは、そのレビューが 3 つすべてを保証できる場合にのみ作成されます。つまり、変更が 1 つの検出結果に対処し、新しい脆弱性を導入せず、その他の動作は変わらないということです。3 つすべてを保証できない場合、パッチの代わりに、理由を説明する短いメモが表示されます。

<h3 id="patches-are-never-applied-automatically">
  パッチは自動的に適用されることはありません
</h3>

パッチの適用は常にあなたの決定です。パッチはレポートの `patches/` フォルダに配置され、検出結果ごとに 1 つの `F<n>.patch` ファイルが変更を説明するメモとともに配置されます。シェルから 1 つを適用するか、Claude にパッチを適用してプルリクエストを開くよう依頼します。

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

パッチされたコードにテストがない場合、パッチのメモはそのことを示しているため、レビューがテストパスなしで実行されたことがわかります。各パッチを独自のプルリクエストで適用して、独立してレビューおよびテストできるようにします。

<h2 id="how-the-plugin-fits-with-other-security-tools">
  プラグインが他のセキュリティツールとどのように適合するか
</h2>

Claude Security プラグインは、[セキュリティガイダンスプラグイン](/docs/ja/security-guidance)、[`/security-review`](/docs/ja/commands#all-commands)、[Code Review](/docs/ja/code-review)、マネージド [Claude Security](https://claude.com/product/claude-security) プロダクト、および既存のスキャナーと並んで、多層防御スタックのオンデマンド深スキャンレイヤーです。

| ステージ         | ツール                                                                          | カバー内容                                        |
| :----------- | :--------------------------------------------------------------------------- | :------------------------------------------- |
| セッション内       | [セキュリティガイダンスプラグイン](/docs/ja/security-guidance)                                    | Claude が書くコード内の一般的な脆弱性。同じセッションで修正            |
| オンデマンド、単一パス  | [`/security-review`](/docs/ja/commands#all-commands)                              | 現在のブランチに対する 1 回限りのセキュリティパス                   |
| オンデマンド、深スキャン | Claude Security プラグイン                                                        | リポジトリまたは差分のマルチエージェントスキャン。独立してレビューされた検出結果とパッチ |
| プルリクエスト時     | [Code Review](/docs/ja/code-review)、Team および Enterprise プラン                       | 完全なコードベースコンテキストを備えたマルチエージェント正確性およびセキュリティレビュー |
| マネージド        | [Claude Security](https://claude.com/product/claude-security)、Enterprise プラン | 接続されたリポジトリを監視するホストされたスキャン                    |
| CI 内         | 既存の静的分析および依存関係スキャナー                                                          | 言語固有のルール、サプライチェーンチェック、ポリシー実装                 |

プラグインは既存のソースコードセキュリティツールを置き換えません。静的分析、依存関係スキャン、コードレビューと並行して実行します。人間のセキュリティ研究者がするのと同じ方法でコードについて推論します。これは、これらのツールが提供する決定論的チェックを補完します。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

**`/claude-security` メニューが Python 警告で開きます。** プラグインは `PATH` に Python 3.9 以降の `python3` が必要です。`python3` がまったく見つからない場合、メニューは Claude Security がインストールされるまで機能しないことを警告します。`PATH` の最初の `python3` が古い場合、警告は見つかったバージョンを名前で指定します。Python 3 をインストールするか、新しい `python3` を `PATH` の最初に配置してから、新しいセッションを開始してください。

**Fable モデルでスキャンするときに「safeguards flagged this message」という通知が表示される場合があります。** メッセージはモデルを名前で指定します。例えば「Fable 5.1's safeguards flagged this message」。Fable のサイバーセキュリティ安全分類器により、特定のリクエストがフラグされ、Claude Code は [自動モデルフォールバック](/docs/ja/model-config#automatic-model-fallback)を通じてフラグされたリクエストを Opus モデルで再実行します。これは予想されており、スキャンは引き続き正常に完了するはずです。

<h2 id="related-resources">
  関連リソース
</h2>

このページが触れるピースについてさらに詳しく知るには：

* [セキュリティガイダンスプラグイン](/docs/ja/security-guidance)：Claude がコードを書くときにコード内の問題をキャッチします。同じセッション内で
* [Code Review](/docs/ja/code-review)：PR 時のマルチエージェントレビューをセットアップします
* [Claude Security](https://claude.com/product/claude-security)：接続されたリポジトリを監視するマネージドサービス
* [Claude Code セキュリティ](/docs/ja/security)：Claude Code がトラスト、権限、セーフガードにどのようにアプローチするか
* [プラグインをインストールして管理](/docs/ja/plugins/install)：公式マーケットプレイスから他のプラグインを検索してインストールします
