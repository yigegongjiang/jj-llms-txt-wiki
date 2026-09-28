> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# コード インテリジェンス プラグイン

> 言語サーバー プラグインをインストールして、Claude が編集後の型エラーを検出し、シンボルでコードをナビゲートできるようにし、LSP プラグイン推奨ダイアログに応答します。

コード インテリジェンス プラグインは、エディターが持つライブ診断と定義へのジャンプを Claude に提供するため、Claude は自身の編集によって導入される型エラーと不足しているインポートをビルドを実行する前にキャッチでき、テキスト検索の代わりにシンボルで定義と参照を見つけることができます。

各プラグインは、Language Server Protocol（LSP）を通じて 1 つの言語の言語サーバーに Claude Code を接続します。プラグインは Anthropic の公式マーケットプレイスからインストールし、言語サーバー バイナリをマシンにインストールします。

<Note>
  コード インテリジェンス プラグインはターミナル セッションで動作します。[クラウド セッション](/docs/ja/claude-code-on-the-web)では、Claude Code はプラグイン言語サーバーを起動しないため、Claude はそこで診断またはコード ナビゲーションを取得しません。独自の言語サーバー プラグインを作成するか、プラグインがない言語サーバーを接続するには、[プラグイン コンポーネントの LSP サーバー](/docs/ja/plugins/components#lsp-servers)を参照してください。
</Note>

開始するには、[コード インテリジェンス プラグインをインストールする](#install-a-code-intelligence-plugin)の下の表で言語を見つけてください。その表のプラグインは Anthropic の[公式プラグイン マーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)から提供されています。

既に **LSP プラグイン推奨**ダイアログを見た場合は、[推奨ダイアログを受け入れるか却下する](#accept-or-dismiss-the-recommendation-dialog)を参照して、各選択肢の機能を確認してください。

<h2 id="install-a-code-intelligence-plugin">
  コード インテリジェンス プラグインをインストールする
</h2>

コード インテリジェンス プラグインは、言語サーバーを起動するコマンドと、それが処理するファイル拡張子を Claude Code に伝えます。言語サーバーは含まれていません。まず言語サーバー バイナリをインストールし、次にプラグインをインストールし、サーバーが起動することを確認します。

<Steps>
  <Step title="言語サーバー バイナリをインストールする">
    下の表で言語を見つけ、その行のバイナリをインストールします。言語がリストされていない場合は、[公式プラグインのない言語を追加する](#add-a-language-without-an-official-plugin)を参照してください。

    | 言語                      | プラグイン                                                                                                            | バイナリ                         |
    | :---------------------- | :--------------------------------------------------------------------------------------------------------------- | :--------------------------- |
    | C/C++                   | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                     |
    | C#                      | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                  |
    | Go                      | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                      |
    | Java                    | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                      |
    | Kotlin                  | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                 |
    | Liquid                  | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`（Shopify CLI から）    |
    | Lua                     | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`        |
    | PHP                     | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`               |
    | Python                  | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`         |
    | Ruby                    | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                   |
    | Rust                    | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`              |
    | Swift                   | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`              |
    | TypeScript と JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server` |

    Anthropic は `liquid-lsp` を除く表内のすべてのプラグインを保守しており、`liquid-lsp` は Shopify が保守し、公式マーケットプレイスにリストされています。

    バイナリをインストールするコマンドを見つけるには、表のプラグインのリンクをたどって README を参照してください。TypeScript の場合、そのコマンドは `npm install -g typescript-language-server typescript` です。

    バイナリをインストールした後、`claude` を起動するシェルの `PATH` 上にあることを確認します。例えば `which typescript-language-server` または PowerShell では `Get-Command typescript-language-server` を使用します。
  </Step>

  <Step title="プラグインをインストールする">
    ステップ 1 の表で言語用にリストされているプラグインをインストールするには、Claude Code セッション内で `/plugin install` を実行し、`typescript-lsp` をそのプラグインの名前に置き換えます。

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    確認メッセージは、プラグインが現在アクティブであるか、`/reload-plugins` が必要かを示します。インストールが `Marketplace "claude-plugins-official" not found` で失敗した場合は、[そのエラーのトラブルシューティング エントリ](/docs/ja/plugins/troubleshooting#marketplace-claude-plugins-official-not-found)を参照してください。プラグインがインストールされる場所を制御するか、Claude Code 内ではなくシェルからインストールを実行するには、[プラグインをインストールする](/docs/ja/plugins/install)を参照してください。
  </Step>

  <Step title="サーバーが起動することを確認する">
    言語サーバーは、Claude がそのプラグインの拡張子の 1 つを持つファイルを初めてエディットするときに起動します。動作を確認するには、Claude にそのプログラミング言語のファイルに型エラーを導入してから修正するよう依頼します。次に、会話で診断行を確認します。

    * **診断行が表示される**: エラーを導入したエディットの下に `Found N new diagnostic issues in M files (ctrl+o to expand)` が表示されることは、サーバーが起動したことを意味します。
    * **診断行が表示されない**: `/plugin` を実行して **Errors** タブを開きます。`Executable not found in $PATH: "<binary>"` と読む行は、インストールするバイナリを示します。タブにそのような行がない場合は、[コード インテリジェンスのトラブルシューティング](#troubleshoot-code-intelligence)を参照してください。

    不足しているバイナリをインストールした後、Claude Code は Claude が一致するファイルをエディットするたびに次回試行します。バイナリを `claude` を起動したシェルの `PATH` 上にないディレクトリにインストールした場合は、それが存在するシェルから新しいセッションを開始します。
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Claude が得るもの
</h2>

言語サーバーが実行されている場合、Claude は診断とコード ナビゲーションを取得します。

* **エディット後の診断**: Claude がサーバーが処理するファイルをエディットまたは作成するたびに、Claude はサーバーが報告するエラーと警告を取得します。コンパイラを実行せずに、導入した型エラー、不足しているインポート、または構文エラーを見ます。
* **コード ナビゲーション**: Claude はテキストを検索する代わりに、サーバーを通じてシンボルを検索する `LSP` ツールを取得します。ツールは読み取り専用です。Claude がツールで検索できるもの、および権限がツールにどのように適用されるかについては、[LSP ツール動作](/docs/ja/tools-reference#lsp-tool-behavior)を参照してください。

<h3 id="read-the-diagnostics-yourself">
  診断を自分で読む
</h3>

Claude がサーバーが処理するファイルをエディットした後、会話は `Found N new diagnostic issues` サマリーのみを表示します。問題自体を読むには、**Ctrl+O** を押します。

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  推奨ダイアログを受け入れるか却下する
</h2>

言語サーバー バイナリが既に `PATH` 上にあり、それを使用するプラグインがインストールされていない場合、Claude Code は **LSP プラグイン推奨**というタイトルのダイアログでプラグインをインストールするよう提供します。

<h3 id="when-the-recommendation-dialog-appears">
  推奨ダイアログが表示される場合
</h3>

**LSP プラグイン推奨**ダイアログは Claude がファイルをエディットした後に表示される場合があります。これらの条件は、表示されるかどうか、どのプラグインを提供するかを決定します。

* **プラグインがファイルと一致する**: 追加したマーケットプレイスの 1 つ、または Claude Code が登録した公式マーケットプレイスが、そのファイルの拡張子のコード インテリジェンス プラグインをリストし、プラグインのバイナリがインストールされています。
* **公式が最初**: 複数のマーケットプレイスが拡張子のプラグインを提供する場合、ダイアログは公式マーケットプレイスのプラグインを提供します。
* **セッションごとに 1 回**: ダイアログはセッションで最大 1 回、Claude がエディットする最初の一致するファイルに対して表示されます。
* **クラウド セッションではない**: ターミナルが [`claude --cloud`](/docs/ja/claude-code-on-the-web#from-terminal-to-cloud)で開始したセッションなどのクラウド セッションに接続されている場合、ダイアログは表示されません。

<h3 id="respond-to-the-recommendation-dialog">
  推奨ダイアログに応答する
</h3>

**LSP プラグイン推奨**ダイアログはプラグインに名前を付け、これらの選択肢を提供します。

* **はい、インストール**: Claude Code はプラグインをユーザー アカウント用にインストールし、`<plugin> installed · restart to apply` を出力します。新しいセッションを開始してサーバーを読み込みます。
* **いいえ、後で**: ダイアログが閉じ、後のセッションでプラグインを再度提供できます。**Esc** を押すと同じです。
* **このプラグインは表示しない**: ダイアログはそのプラグインに対して表示されなくなり、他のプラグインに対しては表示されます。
* **すべての LSP 推奨を無効にする**: ダイアログはすべての言語に対して表示されなくなります。

オプションを選択しない場合、Claude Code は 30 秒後にそれを閉じ、無視されたとカウントします。カウントはセッション全体で保持されます。5 つの無視されたダイアログの後、Claude Code はプラグインの推奨を停止します。これは **すべての LSP 推奨を無効にする**を選択した場合と同じです。

<h3 id="turn-recommendations-back-on">
  推奨をオンに戻す
</h3>

**LSP プラグイン推奨**ダイアログは、**すべての LSP 推奨を無効にする**を選択するか、5 回無視した後に表示されなくなります。

* **無効または 5 回無視**: どちらの場合でもオンに戻すには、Claude Code 独自の設定ファイルである `~/.claude.json` から `lspRecommendationDisabled` と `lspRecommendationIgnoredCount` キーを削除します。
* **このプラグインは表示しない**: **このプラグインは表示しない**を選択し、そのプラグインを再度提供したい場合は、同じファイルの `lspRecommendationNeverPlugins` リストからその `name@marketplace` ID を削除します。

<h2 id="troubleshoot-code-intelligence">
  コード インテリジェンスのトラブルシューティング
</h2>

プラグインのトラブルシューティング ページでは、[言語サーバーが起動しない、メモリ使用量が多い、または診断が正しくない](/docs/ja/plugins/troubleshooting#language-server-doesnt-start)の下にあるコード インテリジェンス プラグインに固有の症状について説明しています。

* **言語サーバーが起動しない**：`/plugin` の **Errors** タブに `Executable not found in $PATH` が表示されるか、Claude が言語の診断をレポートしません。
* **メモリ使用量が多い**：サーバーがプロジェクトをインデックスしている間、メモリ使用量が増加します。
* **モノレポでの誤検知診断**：診断が、実際には解決されているインポートを未解決としてレポートします。

<h2 id="add-a-language-without-an-official-plugin">
  公式プラグインのない言語を追加する
</h2>

言語が[公式プラグインの表](#install-a-code-intelligence-plugin)にない場合でも、言語サーバーを接続できます。

1. サーバー コマンドとそれが処理するファイル拡張子に名前を付ける `.lsp.json` ファイルを使用してプラグインを作成します。
2. 次に、[`--plugin-dir`](/docs/ja/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)を使用してプラグインを読み込むか、マーケットプレイスに公開します。

ファイルのフィールドと実装例については、[プラグイン コンポーネント内の LSP サーバー](/docs/ja/plugins/components#lsp-servers)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグイン コンポーネント内の LSP サーバー](/docs/ja/plugins/components#lsp-servers): 公式プラグインがない言語サーバーの `.lsp.json` を作成します。
* [プラグインをインストールして管理する](/docs/ja/plugins/install): スコープ、更新、アンインストール
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting): このページの言語サーバー以外のロード エラー
* [公式マーケットプレイスでプラグインを見つける](/docs/ja/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): 公式マーケットプレイスの残りを参照する場所
