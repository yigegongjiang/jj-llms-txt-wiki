> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# モノレポまたは大規模コードベースで Claude Code をセットアップする

> ネストされた CLAUDE.md ファイル、スパースワークツリー、コード インテリジェンス、パッケージごとのスキルを使用して、モノレポと大規模シングルツリーコードベース向けに Claude Code を設定し、Claude が作業中のコードに焦点を当てるようにします。

大規模コードベースは、数百万行のコードを持つ 1 つのリポジトリ、または多くのパッケージを持つモノレポです。Claude Code はどのサイズでも動作しますが、コードベースが成長するにつれて、小規模プロジェクト向けにチューニングされたデフォルト設定は、タスクに関連しない命令とファイル読み取りでコンテキストウィンドウを満たし、トークンを消費して Claude のパフォーマンスを低下させます。

このガイドでは、個々の開発者とエンジニアリングチームが Claude をタスクが触れるコードベースの部分にスコープする方法を示します。各セクションでは、設定が個人用マシンに固有か、リポジトリにコミットされるかを記載しています。

<h2 id="what-this-guide-covers">
  このガイドで説明する内容
</h2>

[下の表](#settings-on-this-page)は各設定とその目的を一覧表示しています。[その後のファイルツリー](#the-example-monorepo)は、このページのすべてのコード例が参照するサンプルモノレポです。

<h3 id="settings-on-this-page">
  このページの設定
</h3>

以下の各設定は独立しています。相互に置き換わるのではなく層状に重なるため、リポジトリに適したものを適用してください。[Claude を開始する場所を選択](#choose-where-to-start-claude)は設定ファイルが存在する場所を決定するため、最初にそれを読んでください。[すべてをまとめる](#put-it-together)はそれらすべてを組み合わせたものを示しています。

| 実行したいこと                                                   | 使用するもの                                                                                   |
| :-------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| すべてのサブシステムをカバーする 1 つのルートファイルの代わりに、タッチするコードの規約のみを読み込む      | ディレクトリごとの [CLAUDE.md ファイル](#layer-claude-md-files-by-directory)                          |
| 作業しないパッケージの CLAUDE.md ファイルを除外する                           | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                |
| Claude がビルド出力、生成されたコード、ベンダー依存関係を開くのをブロックする                | `permissions.deny` の [`Read` 拒否ルール](#block-reads-of-generated-and-vendored-code)         |
| ファイルをスキャンする代わりに言語サーバーを通じてシンボルの定義または呼び出し元を見つける             | [コード インテリジェンス プラグイン](#reduce-file-reads-with-code-intelligence)                          |
| Claude がワークツリーを作成するときにタスクが必要とするディレクトリのみをチェックアウトする         | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                       |
| 同じセッションから兄弟パッケージまたは別のリポジトリを読み取り、編集する                      | [`--add-dir`](#grant-access-across-packages-or-repositories) または `additionalDirectories` |
| 関連する場合にのみ読み込まれる 1 つの領域に固有の手順を Claude に提供する                | ディレクトリごとの [スキル](#add-per-directory-skills)                                               |
| 多くのディレクトリごとの CLAUDE.md ファイルを、すべてがインストールする 1 つの規約セットに置き換える | 内部マーケットプレイスの [プラグイン](#centralize-conventions-when-layering-stops-scaling)                |

<Tip>
  [サブエージェントで調査を実行する](/docs/ja/best-practices#use-subagents-for-investigation)など、任意のリポジトリでコンテキストを小さく保つワークフロー技術については、[Claude Code のベストプラクティス](/docs/ja/best-practices)を参照してください。組織内のすべての開発者にベースライン設定をロールアウトするには、[組織向けに Claude Code をセットアップする](/docs/ja/admin-setup)を参照してください。
</Tip>

<h3 id="the-example-monorepo">
  サンプルモノレポ
</h3>

このページ全体の例は、3 つのパッケージを持つモノレポを参照しています。同じパターンは大規模シングルツリーコードベースで機能します。例が `packages/api/` を使用する場合は、`src/backend/` や `lib/core/` などの独自のサブシステムディレクトリに置き換えてください。

```text theme={null}
monorepo/
  CLAUDE.md                     # ルート命令
  packages/
    api/
      CLAUDE.md                 # API 固有の命令
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # フロントエンド固有の命令
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # 共有ライブラリ命令
      src/
```

<h2 id="choose-where-to-start-claude">
  Claude を開始する場所を選択
</h2>

`claude` を起動する場所は、Claude が追加の許可なしで読み取り、編集できるファイル、起動時に読み込まれる CLAUDE.md ファイル、および適用されるプロジェクト設定を決定します。

| 開始場所     | ファイルアクセス               | 起動時に読み込まれる CLAUDE.md                               | 使用する場合                           |
| :------- | :--------------------- | :------------------------------------------------- | :------------------------------- |
| リポジトリルート | すべてのファイル               | ルートのみ。サブディレクトリファイルは Claude がそこを読むときにオンデマンドで読み込まれます | タスクが複数のパッケージまたはサブシステムにまたがる       |
| サブディレクトリ | そのサブツリーのみ、さらに許可を付与するまで | そのディレクトリのプラス、すべての祖先                                | 作業が 1 つのパッケージまたはサブシステムにスコープされている |

`.claude/settings.json` のプロジェクト設定は開始ディレクトリからのみ読み込まれ、CLAUDE.md ファイルのように親ディレクトリから継承されません。セッションが読み込む `.claude/settings.json` がどのディレクトリのものであるかについては、[Claude Code が各ファイルを探す場所](/docs/ja/settings#where-claude-code-looks-for-each-file)を参照してください。

以下の各セクションでは、その設定ファイルがリポジトリルートまたは開始するサブディレクトリに属するか、およびコミットされるか、ローカルに保持されるかを記載しています。

<h2 id="layer-claude-md-files-by-directory">
  ディレクトリごとに CLAUDE.md ファイルを層状にする
</h2>

大規模コードベースでは、リポジトリルートの単一の CLAUDE.md は、すべてのサブシステムの規約をカバーするために成長する傾向があり、現在のタスクに関連しない命令でコンテキストを消費するか、有用であるには一般的すぎます。命令をディレクトリごとのファイルに分割すると、Claude はリポジトリ全体のルールと、作業中のコードの規約のみを読み込みます。

Claude Code は起動時に作業ディレクトリとすべての親ディレクトリから [CLAUDE.md](/docs/ja/memory) ファイルを読み込み、その後、Claude がそこでファイルを読むときにオンデマンドで各サブディレクトリのファイルを読み込みます。ルートファイルはリポジトリ全体のルールを設定し、各サブディレクトリは独自のルールを追加します。

一般的な分割は 2 つのレベルです。

* **ルート `CLAUDE.md`**: コーディング標準やコミット規約など、どこにでも適用される命令
* **サブディレクトリごとの `CLAUDE.md`**: その領域のスタックに固有の規約。モノレポではパッケージごとに 1 つ。大規模シングルツリーでは `src/db/` や `src/api/` などのサブシステムごとに 1 つ

これらのファイルをリポジトリにコミットして、チームメイトが継承するようにします。各ディレクトリの所有者は通常、そのファイルを保守します。

既にチェックインされているファイルをトリミングするには、[`/doctor` チェックアップ](/docs/ja/memory#my-claude-md-is-too-large)を実行します。ルート `CLAUDE.md` はすべてのパッケージに適用されるルールを保持します。

```markdown CLAUDE.md theme={null}
パッケージスクリプトはモノレポルートではなく、パッケージディレクトリから実行します。
コミットサブジェクトにパッケージ名をプレフィックスします。例えば `api: add rate limiting`。
packages/*/generated/ の下のファイルを編集しないでください。代わりにパッケージで `npm run codegen` を実行します。
```

各サブディレクトリの `CLAUDE.md`（ここでは `packages/api/CLAUDE.md`）は、その領域に固有の規約を追加します。

```markdown packages/api/CLAUDE.md theme={null}
何かを実行する前に `.env.example` を `.env` にコピーします。テストと開発サーバーはそれなしでは失敗します。
Knex クエリビルダーでデータベースクエリを記述します。ルートハンドラーに生の SQL 文字列を入れないでください。
マイグレーションがマージされた後は編集しないでください。代わりに新しいマイグレーションを追加します。
```

`packages/api/` から Claude を開始すると、`packages/api/CLAUDE.md` とルート `CLAUDE.md` の両方が読み込まれます。Claude はローカル命令をリポジトリ全体のルールと一緒に見て、`packages/web/` からの命令はコンテキストに含まれません。同じことは非モノレポツリーのすべてのサブディレクトリに当てはまります。どのファイルが読み込まれたかを確認するには、`/context` を実行して、**Memory files** の下のリストを確認します。

コードベースとモデルが変わるにつれて、ファイルを最新に保つためのいくつかの方法があります。

* **プルリクエストで確認**: CLAUDE.md の編集を他のドキュメント変更と同じように扱い、規約がコードを追跡するようにします
* **主要なモデルリリース後に再検討**: 古いモデルの制限を回避した命令は、新しいモデルがケースを独自に処理すると、オーバーヘッドになる可能性があります。例えば、単一ファイルのリファクタリングを強制するルールは、制限がなくなると削除できます
* **Stop フックを追加して更新を提案**: [`Stop` フック](/docs/ja/hooks#stop)は Claude が応答を終了したときにセッショントランスクリプトへのパスを受け取るため、スクリプトはセッションを確認し、ギャップが新鮮なうちに CLAUDE.md の更新を提案できます

CLAUDE.md ファイルがどのように読み込まれ、相互作用するかについての詳細は、[メモリとプロジェクト命令](/docs/ja/memory)を参照してください。

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  ディレクトリごとの CLAUDE.md とパススコープルールの選択
</h3>

ディレクトリごとの `CLAUDE.md` ファイルと [パススコープルール](/docs/ja/memory#path-specific-rules)の下の `.claude/rules/` の両方により、ツリーの一部に命令をターゲットできます。ファイルが存在する場所と読み込まれるタイミングが異なります。

| アプローチ                       | ファイルの場所                | 読み込まれるタイミング                                          | 使用する場合                                       |
| :-------------------------- | :--------------------- | :--------------------------------------------------- | :------------------------------------------- |
| ディレクトリごとの `CLAUDE.md`       | ディレクトリ内、そのコードと一緒に      | そのディレクトリから開始したときの起動時、または Claude がそこでファイルを読むときのオンデマンド | ディレクトリ所有者が独自の規約を保守します。命令はコードでバージョン管理されます     |
| `.claude/rules/` のパススコープルール | リポジトリルートの中央 `.claude/` | Claude がルールの `paths:` グロブに一致するファイルで動作するとき            | すべての規約を 1 つの場所に置きたい、または同じルールが多くの散在したパスに適用される |

スキルもカバーする比較については、[同様の機能を比較](/docs/ja/features-overview#compare-similar-features)を参照してください。

<h3 id="exclude-irrelevant-claude-md-files">
  関連のない CLAUDE.md ファイルを除外する
</h3>

リポジトリルートから Claude を開始すると、各サブディレクトリの CLAUDE.md は Claude がそのディレクトリ内のファイルを読むとすぐに読み込まれます。`claudeMdExcludes` 設定は特定のファイルをパスまたはグロブパターンでスキップして、読み込まれないようにします。

他のチームのパッケージ、レガシーコード、ベンダーサブツリーなど、作業しないディレクトリに使用します。除外リストは静的で、タスクごとのスイッチではありません。今日は 1 つのパッケージに焦点を当て、明日は別のパッケージに焦点を当てるには、除外を編集する代わりに [そのパッケージのディレクトリから Claude を開始](#choose-where-to-start-claude)してください。

これらの除外をご自身のみに使用する場合は、`.claude/settings.local.json` に設定を配置します。Claude Code はそれを作成するときにそのファイルを gitignore に追加します。ここで手動で作成しているため、gitignore に自分で追加してください。パターンはグロブ構文を使用して絶対ファイルパスに対してマッチするため、相対スタイルのパターンを `**/` で開始して、ツリーのどこにでもマッチするようにします。以下の例は他のチームが所有するパッケージを除外します。

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/web/**"
  ]
}
```

これはそのパッケージの下のすべての CLAUDE.md とルールファイルをスキップします。ルート CLAUDE.md と作業するパッケージは通常どおり読み込まれます。

これらのパターンは他の一般的なケースをカバーしています。

* `"**/packages/*/CLAUDE.md"`: ルートを保持しながら、すべてのパッケージの CLAUDE.md を除外します
* `"**/packages/legacy-*/**"`: グロブに一致する名前のすべてのパッケージを除外します。ルールを含みます
* `"/home/user/monorepo/legacy/CLAUDE.md"`: 絶対パスで 1 つの特定のファイルを除外します

管理ポリシー CLAUDE.md ファイルは除外できないため、組織全体の命令は常に適用されます。`claudeMdExcludes` は任意の [設定スコープ](/docs/ja/settings#where-settings-live)で設定できます。ユーザー、プロジェクト、ローカル、または管理。配列はスコープ全体でマージされるため、チームはプロジェクトレベルのデフォルトを設定でき、個人はローカルオーバーライドを追加できます。

完全な除外ドキュメントについては、[特定の CLAUDE.md ファイルを除外](/docs/ja/memory#exclude-specific-claude-md-files)を参照してください。

<h2 id="reduce-what-claude-reads">
  Claude が読む内容を削減する
</h2>

命令は Claude のコンテキストに入るもののほんの一部です。ファイル読み取りは、コードベースとともに成長するもう 1 つのコストです。以下の設定は関連のないパスの読み取りをブロックし、徹底的なファイルスキャンを言語サーバーのルックアップに置き換えます。

<h3 id="block-reads-of-generated-and-vendored-code">
  生成されたコードとベンダーコードの読み取りをブロックする
</h3>

Claude のコンテンツ検索はデフォルトで `.gitignore` を尊重するため、`node_modules/`、`dist/`、`build/` などのそこにリストされているパスは、追加の設定なしで検索結果から除外されます。

ベンダー SDK やコミットされた生成コードなど、チェックインされているパスについては、`permissions.deny` に `Read` 拒否ルールを追加して、Claude がそれらのファイルを開くのをブロックします。

拒否ルールは、リポジトリで作業するすべての人、あなただけ、またはマシン上のすべてのセッションに適用できます。これは、どの設定ファイルに配置するかによって異なります。

* **リポジトリで作業するすべての人**: ルールを `.claude/settings.json` にコミットします。Claude をそこから開始する場合はリポジトリルートに配置するか、サブディレクトリから開始する場合は各パッケージの `.claude/` に配置します。このページの他のプロジェクト設定と同様に、そのファイルは親ディレクトリから継承されません。
* **あなただけ**: リポジトリルートで `.claude/settings.local.json` を使用します。これは開始ディレクトリに関係なく、リポジトリ内のすべての CLI セッションで読み込まれます。ただし、Windows など Claude Code が [リポジトリルートを使用しない](/docs/ja/settings#where-claude-code-looks-for-each-file)場合は除きます。例の `Read(./**/vendor/**/*)` のような相対パターンは、[セッションの現在の作業ディレクトリ](/docs/ja/permissions#read-and-edit)ではなく、リポジトリルートではなく、セッションの現在の作業ディレクトリにアンカーされます。サブディレクトリからセッションを開始する場合は、このファイルのルールを `//` 絶対パスとして記述します。例えば `Read(//absolute/path/to/repo/**/vendor/**/*)` のようにします。v2.1.211 より前では、`.claude/settings.local.json` は開始ディレクトリからのみ読み込まれていました。
* **すべての人、すべてのセッションで強制**: [管理設定](/docs/ja/managed-settings)でルールを設定します。ユーザーとプロジェクト設定はこれをオーバーライドできません。

以下の例はビルドアーティファクトとベンダー SDK をブロックします。そのディレクトリパターンは `/**` ではなく `/**/*` で終わるため、各ルールはディレクトリ内のすべてをカバーしますが、ディレクトリ自体はカバーしません。Claude はその後、`ls dist` や `cd build` などでこれらのディレクトリをリストしたり、変更したりできます。

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)",
      "Read(./**/*.generated.*)",
      "Read(./**/vendor/**/*)"
    ]
  }
}
```

拒否ルールは Claude の組み込みファイルツールをカバーします。Bash では、拒否されたパスが引数として表示される場合、Claude Code が認識するファイルコマンド（`cat`、`head`、`grep`、`find` など）をカバーします。また、`< file` などの[リダイレクション](/docs/ja/permissions#redirections)のターゲットもカバーします。Claude Code は、組み込みの Grep および Glob ツールの結果から拒否されたパスを除外するために最善の努力を行います。`grep -r` または `find` などの Bash 検索を拒否されたファイルを含むディレクトリに対して実行すると、その出力にそれらが含まれます。

拒否ルールは、ファイルを自分で開くサブプロセスをカバーしません。完全なパターン構文については、[Read および Edit 権限ルール](/docs/ja/permissions#read-and-edit)を参照してください。

<h3 id="reduce-file-reads-with-code-intelligence">
  コード インテリジェンスでファイル読み取りを削減する
</h3>

大規模コードベースでは、シンボルが定義または使用されている場所を見つけることは、多くのファイル読み取りと grep 呼び出しを消費する可能性があります。[コード インテリジェンス プラグイン](/docs/ja/plugins/code-intelligence)は Claude を言語サーバーに接続して、ツリーをスキャンする代わりに、定義にジャンプしたり、参照を見つけたり、型エラーを直接表示したりできます。

公式マーケットプレイスには TypeScript、Python、Go、Rust、その他の一般的な言語用のプラグインがあります。Claude Code セッション内で以下のコマンドを実行して TypeScript プラグインをインストールします。

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

インストールが失敗した場合は、Claude Code が報告するメッセージに一致させます。

* `Marketplace "claude-plugins-official" not found`: `/plugin marketplace add anthropics/claude-plugins-official` でマーケットプレイスを追加してから、インストールを再試行します。
* プラグインが[マーケットプレイスで見つかりません](/docs/ja/plugins/install#install-a-plugin): プラグイン名を確認します。

プラグインを自分でインストールするのではなく、リポジトリ内のすべての人に対して有効にするには、[`enabledPlugins` プロジェクト設定](/docs/ja/settings-reference#plugin-settings)に追加します。

コード インテリジェンス プラグインには、各開発者のマシンに言語の言語サーバーバイナリが必要です。[各言語が必要とするバイナリ](/docs/ja/plugins/code-intelligence)を参照してください。公式マーケットプレイスからのインストールには、マーケットプレイスがホストされている GitHub へのネットワークアクセスが必要です。制限されたネットワークでは、代わりに[内部 Git ホストまたはローカルパスからマーケットプレイスを追加](/docs/ja/plugins/install#add-a-marketplace)してください。

これは上記の `claudeMdExcludes` と `Read` 拒否ルールとよく組み合わされます。これらは関連のないコンテンツをコンテキストから除外し、コード インテリジェンスは Claude が定義を見つけるために残りを読むのを防ぎます。

<h2 id="scope-worktrees-and-file-access">
  ワークツリーとファイルアクセスをスコープする
</h2>

これらの設定は、ワークツリーでディスク上にあるもの、および開始ポイントを超えて Claude が読み取り、書き込みできるディレクトリを制御します。

<h3 id="check-out-only-the-directories-you-need">
  必要なディレクトリのみをチェックアウトする
</h3>

`--worktree` フラグは新しい git ワークツリーでセッションを開始して、変更がメインチェックアウトから分離されたままになるようにします。デフォルトではリポジトリ全体をチェックアウトします。大規模リポジトリでは、`worktree.sparsePaths` 設定は git sparse-checkout を使用して、リストされたディレクトリとルートレベルのファイルのみをディスクに書き込むため、ワークツリーはより速く開始し、より少ないスペースを使用します。

このディレクトリで作業するすべての人が同じパスを必要とする場合は、設定を `.claude/settings.json` にコミットします。自分用にパスを追加するには、`.claude/settings.local.json` を使用します。リストはスコープ全体でマージされるため、ローカルファイルはコミットされたリストにパスを追加できますが、削除することはできません。

このページの JSON の例は、一度に 1 つの設定を示しています。`.claude/settings.json` に既に `permissions.deny` ルールなどの他のキーが含まれている場合は、ファイルを置き換えるのではなく、`worktree` キーをそれらと一緒に追加します。[まとめる](#put-it-together)は結合された結果を示しています。

以下の例はコミットされたファイルを示しています。

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Claude がワークツリーを作成するとき、フルツリーの代わりに `.claude/`、`packages/api/`、`packages/shared/` のみをチェックアウトします。`sparsePaths` のパスはリポジトリルートに相対的で、Claude を開始するサブディレクトリに関係なく。任意のディレクトリパスがここで機能し、パッケージルートのみではありません。

これは特に[サブエージェント ワークツリー分離](/docs/ja/worktrees#isolate-subagents-with-worktrees)に役立ちます。サブエージェントはサブタスク用に生成される並列 Claude インスタンスで、ワークツリーで実行されるそれぞれは、フルツリーの代わりに軽量チェックアウトを取得します。セッション内のすべてのワークツリーは同じ `sparsePaths` を共有するため、1 つのサブエージェントが `packages/api/` を必要とし、別のサブエージェントが `packages/web/` を必要とする場合は、両方をリストします。

`sparsePaths` にはディレクトリをリストし、個別のファイルはリストしません。`package.json`、`tsconfig.base.json`、ロックファイルなどのルートレベルのファイルは、リストするディレクトリと一緒に常にチェックアウトされます。ルートレベルのディレクトリはそうではないため、ワークツリー内でリポジトリルートの `.claude/settings.json` または `.claude/rules/` を利用可能にしたい場合は、リストに `.claude` を含めます。プロジェクトスキル、エージェント、コマンドについては、[ワークツリーがメインチェックアウトと共有するもの](/docs/ja/worktrees#what-worktrees-share-with-the-main-checkout)を参照してください。

Sparse checkout では、sparse ワークツリーが存在する間、git がリポジトリの共有 `.git/config` で `extensions.worktreeConfig` を有効にする必要があります。Claude Code は最後のワークツリーが削除された後、そのエントリを削除しますが、Claude Code が追加した場合のみです。自分で設定した値は削除されません。v2.1.207 より前では、エントリは最後のワークツリーが削除された後も残り、`tea` などの go-git ベースのツールは `git config --unset extensions.worktreeConfig` を実行するまでリポジトリを開くことができませんでした。

`node_modules` などの大規模ディレクトリをワークツリー全体で複製するのを避けるには、同じ `.claude/settings.json` で `sparsePaths` を `symlinkDirectories` とペアにします。

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

これは各ワークツリーの `node_modules/` からメインリポジトリのコピーへのシンボリックリンクを作成し、ディスク上で複製するのではなく。

<Note>
  `sparsePaths` と `symlinkDirectories` 設定は、ワークツリーが作成される前に開始ディレクトリから読み込まれます。作成後、セッションの作業ディレクトリはワークツリールートで、起動したサブディレクトリではありません。したがって、ワークツリー内のプロジェクト設定はワークツリールートの `.claude/settings.json`（リポジトリルートのファイルのチェックアウトされたコピー）から読み込まれます。許可ルールやフックなど、ワークツリー内で必要な他の設定は、リポジトリルートの `.claude/settings.json` に配置します。
</Note>

完全なワークツリー設定リファレンスについては、[ワークツリー設定](/docs/ja/settings-reference#worktree)を参照してください。

<h3 id="grant-access-across-packages-or-repositories">
  パッケージまたはリポジトリ全体でアクセスを付与する
</h3>

このセクションは、サブディレクトリから Claude を開始するとき、またはタスクが複数のチェックアウトにまたがるときに適用されます。単一の大規模ツリーのリポジトリルートから開始する場合、Claude はすでにすべてのファイルにアクセスでき、これをスキップできます。

`packages/api/` から Claude を開始すると、そのディレクトリ内のファイルを読み取り、書き込みできます。タスクが `api` と `web` の両方がインポートする共有型を更新するなど、パッケージ全体の変更を必要とする場合は、兄弟ディレクトリへのアクセスを付与する必要があります。同じメカニズムは、別にチェックアウトされたリポジトリへのアクセスを付与します。

`.claude/settings.json` の `additionalDirectories` 設定は、作業ディレクトリの外のディレクトリへのアクセスを Claude に与えます。以下の例は 2 つの兄弟パッケージへのアクセスを付与します。

```json packages/api/.claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

相対パスは Claude を開始するディレクトリに対して解決されます。この設定では、Claude は `packages/api/` から作業しながら `packages/shared/` と `packages/web/` のファイルを読み取り、編集できます。

設定を編集せずに実行時にディレクトリを付与することもできます。Claude を開始するときに `--add-dir` を渡すことで。

```bash theme={null}
claude --add-dir ../shared
```

ディレクトリを追加する方法に関係なく、Claude はそのファイルを読み取り、編集できます。ディレクトリの CLAUDE.md、`.claude/rules/` ファイル、スキルも読み込まれるかどうかは、追加方法によって異なります。

| 追加方法                               | CLAUDE.md とルールを読み込む | スキルを読み込む |
| :--------------------------------- | :------------------ | :------- |
| `additionalDirectories` 設定         | しない                 | しない      |
| `--add-dir` フラグまたは `/add-dir` コマンド | 以下の環境変数のみ           | はい       |

`--add-dir` または `/add-dir` で追加されたディレクトリから CLAUDE.md とルールファイルを読み込むには、`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` 環境変数を設定します。

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

環境変数は `additionalDirectories` 設定にリストされているディレクトリには影響しません。詳細については、[追加ディレクトリから読み込む](/docs/ja/memory#load-from-additional-directories)を参照してください。

この領域のすべての人が必要とする兄弟ディレクトリについては、`additionalDirectories` を `.claude/settings.json` にコミットします。個人的な選択または 1 回限りのアクセスについては、`.claude/settings.local.json` を使用するか、起動時に `--add-dir` を渡します。

<h2 id="add-per-directory-skills">
  ディレクトリごとのスキルを追加する
</h2>

任意のサブディレクトリは、独自のスタックにスコープされた[スキル](/docs/ja/skills)を定義できます。スキルは Claude が関連性があると判断したときにオンデマンドで読み込まれるため、API 固有のツーリングはフロントエンド作業中にコンテキストを消費しません。

スキルはディレクトリ内の `.claude/skills/` の下に存在します。その領域のコードと一緒にコミットして、リポジトリをクローンした誰もがそれらを取得できるようにします。モノレポでは、パッケージごとに 1 つのスキルセットを持つことができます。大規模な単一ツリーコードベースでは、`src/db/.claude/skills/` などのサブシステムごとに 1 つのセットを持つことができます。

サブディレクトリ内にスキルディレクトリを作成します。

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

次に、そのディレクトリ内に `SKILL.md` を書きます。ここでは `packages/api/.claude/skills/api-testing/SKILL.md` です。この例は Claude に API パッケージのテストパターンを教えます。

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Testing patterns for the API package. Use when writing or modifying tests in packages/api/.
---

## Test structure

Tests are in `src/__tests__/` mirroring the `src/` directory structure.
Each route file has a corresponding `.test.ts` file.

## Running tests

- All tests: `npm test`
- Single file: `npm test -- src/__tests__/routes/users.test.ts`
- Watch mode: `npm test -- --watch`

## Test utilities

- `src/__tests__/helpers/db.ts`: provides `setupTestDb()` and `teardownTestDb()` for database tests
- `src/__tests__/helpers/auth.ts`: provides `createTestUser()` and `getAuthToken()` for authenticated endpoints

## Patterns

- Use `supertest` for HTTP assertions, not raw fetch
- Always wrap database tests in a transaction that rolls back
- Mock external services in `src/__tests__/mocks/`
```

別のサブディレクトリは同じ方法で異なるスキルを保持します。`packages/web/.claude/skills/component-patterns/` はテストの代わりにフロントエンドのコンポーネント規約を説明します。Claude が `packages/api/` のファイルで作業するとき、api-testing スキルを読み込みます。`packages/web/` で作業するとき、component-patterns を読み込みます。どちらのディレクトリのスキルも他方のタスク中には読み込まれません。

配置の代わりにファイルパターンでスキルをスコープすることもできます。[`paths` frontmatter フィールド](/docs/ja/skills#frontmatter-reference)はグロブパターンを受け取り、Claude は一致するファイルで作業するときのみ自動的にスキルを読み込みます。これをリポジトリルートの `.claude/skills/` に存在するスキルに使用して、データベースマイグレーションスキルなど `**/migrations/**` にスコープされた特定のファイルにのみ適用します。

スキルの作成と整理の詳細については、[スキル](/docs/ja/skills)を参照してください。

<h3 id="keep-skills-discoverable">
  スキルを発見可能に保つ
</h3>

スキルが多くのディレクトリに分散すると、Claude が選択できるリストが大きくなる可能性があります。Claude は発見されたすべてのスキルの名前と説明を読むことでスキルを選択し、選択されたスキルの完全なコンテンツのみがコンテキストに読み込まれます。このセクションでは、そのリストを小さく保つ方法について説明します。

スコープ内のスキルは、Claude をどこから開始するかによって異なります。

* **`packages/api/` などのサブディレクトリから**: そのディレクトリのスキル、リポジトリルートまでのすべての親、およびユーザーとエンタープライズレベル
* **リポジトリルートから**: ルートスキル、およびセッション中に Claude が接触するすべてのサブディレクトリのスキル。これは数百に蓄積する可能性があります
* **[`--add-dir`](#grant-access-across-packages-or-repositories)でシブリングを追加した後**: そのシブリングのスキルも読み込まれます。`additionalDirectories` 設定はファイルアクセスのみを付与し、スキルは読み込みません

名前は常に読み込まれますが、[多くの場合、一部のスキルはその説明全体を失う可能性があります](/docs/ja/skills#skill-descriptions-are-cut-short)。これにより、Claude がスキルを適用するかどうかを決定するために使用するキーワードが削除される可能性があります。説明を短く保ち、「`packages/api/` でテストを書いたり変更したりするとき」のようなリクエストに含まれる単語で始めます。

PR 規約やデプロイチェックリストなど、多くのディレクトリが共有するスキルの場合、リポジトリルートの `.claude/skills/` に配置して、任意の開始ディレクトリから読み込まれるようにします。共有スキルが独自のバージョン履歴を必要とするか、リポジトリ間で機能する必要がある場合は、代わりに[プラグイン](/docs/ja/plugins/overview)としてパッケージ化します。プラグインスキルは `plugin-name:skill-name` 名前空間を使用するため、ディレクトリごとのスキルと衝突することはありません。プラットフォームチームは 1 つの場所でそれらをバージョン管理および更新できます。

使用されていないスキルを見つけるには、OpenTelemetry [ログエクスポーター](/docs/ja/monitoring-usage)を有効にして `OTEL_LOG_TOOL_DETAILS=1` を設定し、スキル名が編集されずに逐語的に記録されるようにします。[`skill_activated` イベント](/docs/ja/monitoring-usage#skill-activated-event)はその `skill.name` 属性のすべての呼び出しを記録し、`invocation_trigger` はコマンド、Claude、またはネストされたスキルが呼び出したかどうかを記録します。これにより、統合または廃止するものを判断できます。

<h2 id="centralize-conventions-when-layering-stops-scaling">
  レイアリングが拡張を停止したときに規約を一元化する
</h2>

ディレクトリごとの CLAUDE.md ファイルは、コードベースが成長するにつれて管理が難しくなる可能性があります。規約は漂流し、ファイルは古くなり、誰もルートを所有していません。これを解決することは通常、各開発者が独自の領域で作業するのではなく、リポジトリの Claude Code セットアップを保守するチームに該当します。

常に読み込まれる CLAUDE.md から規約と参照コンテンツを、タスクに関連する場合にのみ読み込まれるメカニズムに移動します。

* [Skills](/docs/ja/skills): Claude がタスクに関連する場合にのみ読み込む参照資料
* [Plugins](/docs/ja/plugins/overview): プラットフォームチームが一元的に所有するスキル、フック、コマンドのバージョン管理されたバンドル
* [MCP servers](/docs/ja/mcp): 組織がすでにリポジトリ上でコード検索または RAG インデックスを実行している場合は、MCP ツールとして公開して、Claude がファイルを直接読む代わりにクエリを実行するようにします

プラットフォームチームがこれらを一元的に適用する方法については、[server-managed or endpoint-managed settings](/docs/ja/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings)を参照してください。

<h3 id="recommend-the-right-plugin-at-session-start">
  セッション開始時に正しいプラグインを推奨する
</h3>

規約がプラグインに存在すると、チームメイトがツリーの不慣れな部分で Claude を開始しても、その領域の所有者が保守するプラグインについてのシグナルがありません。[`SessionStart` hook](/docs/ja/hooks#sessionstart)はこのギャップを埋めることができます。フックが stdout に出力するプレーンテキストはすべて、最初のプロンプトの前に Claude のコンテキストに追加されるためです。

たとえば、[hook input](/docs/ja/hooks#common-input-fields)から起動ディレクトリを読み取り、リポジトリにコミットされたパスからプラグインへのマップで検索し、Claude が最初の応答で中継する推奨事項を出力するスクリプトを書くことができます。フックを書き込み、登録する方法については、[Automate actions with hooks](/docs/ja/hooks-guide)を参照してください。

<h2 id="put-it-together">
  すべてをまとめる
</h2>

以下の組み合わせ設定はモノレポレイアウトを使用します。同じファイルは大規模シングルツリーのすべてのサブディレクトリで機能します。各サブディレクトリの `.claude/settings.json` はルートファイルに層状にされるのではなく、自己完結型である必要があります。

例は `worktree`、`additionalDirectories`、`.claude/settings.json` の `Read` 拒否ルールをコミットして、`packages/api/` のすべての開発者が同じ兄弟アクセス、スパースパス、除外を取得するようにします。以下のファイルは `packages/api/` のコミットされたエリアごとの設定です。

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

このセッションは `packages/api/` から開始するため、兄弟パッケージの CLAUDE.md ファイルはすでにスコープ外であるため、`claudeMdExcludes` はここで必要ありません。ルートからセッションも開始する場合は、代わりにリポジトリルートの `.claude/settings.local.json` に追加します。

`additionalDirectories` エントリは `packages/api/` から直接 Claude を開始するときに適用されます。このセッションから作成されたワークツリー内では、作業ディレクトリはワークツリールートであるため、この設定ファイルは読み込まれません。兄弟パッケージはワークツリー内で既にアクセス可能ですが、拒否ルールはリポジトリルートの `.claude/settings.json` に 2 番目のコピーが必要です。ワークツリーセッションがそれらを取得するため、[ディレクトリをチェックアウトするだけで十分です](#check-out-only-the-directories-you-need)で説明するように。

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

セットアップ後、リポジトリはこのレイアウトを持ちます。

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # ワークツリーセッション用の拒否ルール
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # ワークツリー、additionalDirectories、拒否ルール
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

このセットアップで `packages/api/` から Claude を開始すると。

* ルート CLAUDE.md と `packages/api/CLAUDE.md` を読み込み、`packages/web/CLAUDE.md` をスキップ
* `packages/api/` と `packages/shared/` のファイルを読み取り、編集できます
* `packages/api/` の `dist/` と `build/` の下のビルド出力の読み取りをスキップ
* オンデマンドで利用可能な api-testing スキルを持ちます
* `.claude/`、`packages/api/`、`packages/shared/`、ルートレベルのファイルを含むワークツリーを作成し、ルート設定ファイルからワークツリー全体に適用される拒否ルール

<h2 id="scope-and-plan-changes-that-span-packages">
  パッケージにまたがる変更をスコープし、計画する
</h2>

上記の設定は Claude が見るものを制御します。共有型を更新し、それを使用するすべての呼び出しサイトを更新するなど、単一の変更が複数のパッケージに触れる場合、タスクをスコープし、シーケンスする方法も結果に影響します。

クロスパッケージの変更を一貫性のあるものに保つのに役立つ 2 つの技術があります。

* **1 つのセッションで Claude に全体の変更を与える**: 共有編集とその呼び出しサイトを一緒に引き渡すことで、各編集の背後にある決定を一貫性のあるものに保ちます。パッケージごとに再導出するのではなく
* **編集する前に計画する**: [最初に計画](/docs/ja/best-practices#explore-first-then-plan-then-code)し、[Plan Mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) で Claude が計画をファイルに書き込みます。長いクロスパッケージセッションは [コンテキストをコンパクト化](/docs/ja/context-window#what-survives-compaction)します。Claude Code は各コンパクト化の後に計画ファイルを再度注入するため、計画は会話履歴が存在しない場合でも生き残ります。

<h2 id="next-steps">
  次のステップ
</h2>

このセットアップが完了したら、それを改善できます。

* [フック](/docs/ja/hooks-guide)を使用して、Claude がファイルを編集した後、ディレクトリごとのリンターまたは型チェッカーを実行
* [コストを効果的に管理](/docs/ja/costs)を確認して、コードベースサイズがトークン使用量にどのように影響するか、および広範なロールアウト前に支出制限を設定する方法を理解
* Claude ブログの [Claude Code が大規模コードベースでどのように機能するか](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start)を読んで、組織的なロールアウトパターンと所有権モデルについて学びます。このページのリポジトリごとの設定の上に位置します。
