> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# トラブルシューティング

> Claude Code の高い CPU またはメモリ使用量、ハング、auto-compact スラッシング、検索の問題を修正し、その他の問題に対応する適切なページを見つけます。

このページでは、Claude Code が実行中のパフォーマンス、安定性、検索の問題について説明します。その他の問題については、問題が発生している場所に一致するページから始めてください：

| 症状                                                                                                                                | 移動先                                                                          |
| :-------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------- |
| `command not found`、インストール失敗、PATH の問題、`EACCES`、TLS エラー                                                                            | [インストールとログインのトラブルシューティング](/docs/ja/troubleshoot-install)                          |
| 更新またはインストールダウンロードが `The connection dropped while downloading the update` または `aborted` で失敗する                                      | [エラーリファレンス](/docs/ja/errors#the-connection-dropped-while-downloading-the-update)  |
| ログインループ、OAuth エラー、`403 Forbidden`、「organization disabled」、Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry 認証情報 | [インストールとログインのトラブルシューティング](/docs/ja/troubleshoot-install#login-and-authentication) |
| 設定が適用されない、hooks が実行されない、MCP サーバーがロードされない                                                                                          | [設定をデバッグする](/docs/ja/debug-your-config)                                           |
| セッションが auto モードで開始された、または Claude がファイルを編集してコマンドを実行する（確認なし）                                                                        | [セッションが開始するモード](/docs/ja/permission-modes#which-mode-a-session-starts-in)         |
| `API Error: 5xx`、`529 Overloaded`、`429`、リクエスト検証エラー                                                                                | [エラーリファレンス](/docs/ja/errors)                                                      |
| `model not found` または `you may not have access to it`                                                                             | [エラーリファレンス](/docs/ja/errors#theres-an-issue-with-the-selected-model)              |
| VS Code 拡張機能が接続されていない、または Claude を検出していない                                                                                         | [VS Code 統合](/docs/ja/vs-code#fix-common-issues)                                  |
| VS Code または SDK アプリで `Claude Code process exited with code 1`                                                                     | [エラーリファレンス](/docs/ja/errors#claude-code-process-exited-with-code-n)               |
| JetBrains プラグインまたは IDE が検出されない                                                                                                    | [JetBrains 統合](/docs/ja/jetbrains#troubleshooting)                                |
| CPU またはメモリ使用量が多い、応答が遅い、ハング、検索がファイルを見つけられない                                                                                        | [パフォーマンスと安定性](#performance-and-stability)（下記）                                |

どれが当てはまるかわからない場合は、Claude Code 内で `/doctor` を実行して、インストール、設定、拡張機能、コンテキスト使用量の自動チェックを実行してください。確認後に適用できる修正を提案します。`claude` がまったく起動しない場合は、代わりにシェルから `claude doctor` を実行してください。MCP サーバーのステータスを確認するには `/mcp` を実行してください。

<h2 id="performance-and-stability">
  パフォーマンスと安定性
</h2>

これらのセクションでは、リソース使用量、応答性、検索動作に関連する問題について説明します。

<h3 id="high-cpu-or-memory-usage">
  CPU またはメモリ使用量が多い
</h3>

Claude Code はほとんどの開発環境で動作するように設計されていますが、大規模なコードベースを処理する場合、かなりのリソースを消費する可能性があります。パフォーマンスの問題が発生している場合：

1. `/compact` を定期的に使用してコンテキストサイズを削減します。`Not enough messages to compact.` が返される場合、カンバセーションのターン数が少なすぎて要約できません。これは、単一の大規模なペーストがコンテキストを満杯にした場合でも、コンテキストが満杯でも発生する可能性があります
2. 主要なタスク間で Claude Code を閉じて再起動します
3. 大規模なビルドディレクトリを `.gitignore` ファイルに追加することを検討してください
4. [`claude --safe-mode`](/docs/ja/cli-reference#cli-flags) で再起動して、プラグイン、MCP サーバー、またはフックが原因かどうかを確認します。セッション中のすべてのカスタマイズが無効になります。使用量が低下した場合は、[設定をデバッグする](/docs/ja/debug-your-config#test-against-a-clean-configuration)を参照して、どれが原因かを特定します

セッションのヒープメモリが 2.5GB を超える場合、重大なメモリ使用量警告が表示されます。メモリを解放するには、Claude Code を再起動して [`claude --continue`](/docs/ja/cli-reference#cli-flags) を実行し、新しいプロセスでカンバセーションを再開します。

[フルスクリーンレンダリング](/docs/ja/fullscreen)の外では、`/compact` を実行するとメモリも解放されます。メモリ使用量が 2.5GB を下回ると、警告は消えます。

これらのステップ後もメモリ使用量が高いままの場合は、`/heapdump` を実行して 2 つのファイルを `~/Desktop` に書き込みます。`<session-id>.heapsnapshot` という名前の JavaScript ヒープスナップショットと、`<session-id>-diagnostics.json` という名前のメモリ分析です。Claude Code は[コマンドメニューからコマンドを非表示にします](/docs/ja/commands#how-the-command-menu-matches-what-you-type)。完全に入力してください。Linux でデスクトップフォルダがない場合、ファイルはホームディレクトリに書き込まれます。

<Warning>
  `.heapsnapshot` ファイルには、プロセス内のすべての文字列が含まれています。これには、完全なカンバセーションと認証情報が含まれます。公開の issue に添付したり、共有したりしないでください。
</Warning>

コマンドはカンバセーションに要約も出力します。resident set size、JS ヒープ、array buffers、および説明されていないネイティブメモリを表示します。また、高いメモリ増加率や異常に多くのオープンハンドルなど、検出されたリーク指標も表示します。要約は、ほとんどのメモリが JS ヒープ（スナップショットがキャプチャする）にあるか、ネイティブメモリ（キャプチャしない）にあるかを示します。

出力に対して次の 2 つのいずれかを実行します：

* **報告する**：[GitHub issue](https://github.com/anthropics/claude-code/issues) を開き、`-diagnostics.json` ファイルのみを添付します。このファイルには、出力された要約の背後にある統計が含まれており、カンバセーション内容や認証情報は含まれていません
* **自分で調査する**：要約がほとんどのメモリが JS ヒープにあると示している場合、Chrome DevTools の Memory → Load で `.heapsnapshot` ファイルを開き、保持されたサイズでソートして、メモリを保持しているものを確認します

要約がほとんどのメモリがネイティブにあると示している場合、スナップショットはそれを表示できません。代わりに、要約のリーク指標をレポートに含めてください。

<h3 id="large-tables-are-cut-off-in-the-terminal">
  ターミナルで大きなテーブルが切り取られる
</h3>

200 行以上の Markdown テーブルは、最初の 200 行とそれに続く `… N more rows not shown` 行をレンダリングします。表示のみが制限されます。完全なテーブルはカンバセーションに残り、[`/copy`](/docs/ja/commands) はすべての行をコピーします。ターミナルで読むには大きすぎるテーブルの場合は、Claude にファイルに書き込むよう依頼してください。v2.1.208 より前では、Claude Code はすべての行をレンダリングしていたため、非常に大きなテーブルを含むセッションを再開すると、再レンダリング中にスタールする可能性がありました。

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  自動コンパクションがスラッシングエラーで停止する
</h3>

`Autocompact is thrashing: the context refilled to the limit...` が表示される場合、自動コンパクションは成功しましたが、ファイルまたはツール出力がコンテキストウィンドウを数回連続で満杯に戻しました。Claude Code は進捗を遂行していないループで API 呼び出しを無駄にするのを避けるために再試行を停止します。

回復するには：

1. Claude に、ファイル全体ではなく、特定の行範囲または関数など、より小さなチャンクで大きなファイルを読むよう依頼します
2. `/compact` を実行して、大きな出力を削除するフォーカスを使用します（例：`/compact keep only the plan and the diff`）
3. 大規模ファイルの作業を [subagent](/docs/ja/sub-agents) に移動して、別のコンテキストウィンドウで実行されるようにします
4. 以前のカンバセーションがもう必要ない場合は `/clear` を実行します

<h3 id="command-hangs-or-freezes">
  コマンドがハングまたはフリーズする
</h3>

Claude Code が応答しないように見える場合：

1. Ctrl+C を押して現在の操作をキャンセルしてみます
2. 応答しない場合は、ターミナルを閉じて再起動する必要があります

再起動してもカンバセーションは失われません。同じディレクトリで `claude --resume` を実行してセッションを再開してください。

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  エディタの統合ターミナルでのテキストの文字化けまたは破損
</h3>

VS Code、Cursor、または Devin Desktop の統合ターミナルで Claude Code を実行する場合、文字がボックス、スミア、または間違ったグリフとしてレンダリングされる場合、ターミナルの GPU レンダラーが原因である可能性があります。Claude Code 内で `/terminal-setup` を実行して、`terminal.integrated.gpuAcceleration` を `"off"` に設定するか、エディタの設定で手動で設定してウィンドウをリロードします。[ターミナル設定](/docs/ja/terminal-config)で、`/terminal-setup` が書き込む他の設定を参照してください。

<h3 id="mouse-wheel-scrolls-one-line-at-a-time-in-fullscreen-rendering">
  フルスクリーンレンダリングでマウスホイールが一度に 1 行スクロールする
</h3>

[フルスクリーンレンダリング](/docs/ja/fullscreen)では、Claude Code はカンバセーション自体をスクロールし、ターミナルに任せません。各ホイールノッチが希望より少ない行を移動する場合、`/scroll-speed` を実行してノッチあたりの行数を増やして保存するか、`CLAUDE_CODE_SCROLL_SPEED` 環境変数を設定します。ただし、JetBrains IDE ターミナルでは、Claude Code は独自のスクロール処理を適用し、どちらも有効になりません。[マウスホイールスクロール](/docs/ja/fullscreen#mouse-wheel-scrolling)で、各値が受け入れるものを参照してください。

速度を変更せずにより速く移動するには、`PgUp` と `PgDn` を押して一度に半画面スクロールします。スクロールをターミナルのネイティブスクロールバックに戻すには、`/tui default` を実行してクラシックレンダラーに切り替えます。

<h3 id="clipboard-commands-such-as-pbcopy-fail-inside-the-sandbox">
  サンドボックス内で `pbcopy` などのクリップボードコマンドが失敗する
</h3>

[サンドボックス](/docs/ja/sandboxing)がオンの場合、`pbcopy`、`xclip`、`wl-copy` などのクリップボードユーティリティは、サンドボックス化された Bash コマンド内からシステムクリップボードに到達できず、Claude がテキストをパイプした後、クリップボードが変更されないままになる可能性があります。

Claude の出力をクリップボードに配置するには、Claude に応答でコンテンツを出力するよう依頼してから、[`/copy`](/docs/ja/commands) を実行します。`/copy` はサンドボックス化されたコマンドではなく Claude Code プロセス自体からクリップボードに書き込むため、サンドボックスはそれをブロックしません。応答全体ではなく単一のコードブロックをコピーでき、コピーしたものをファイルに書き込んで、パスを出力します。これにより、クリップボード書き込みがターミナルに到達しない場合（例えば SSH 経由）のフォールバックが提供されます。

Claude がテキストをこれらのツールにパイプする場合、`pbcopy *`、`wl-copy *`、または `xclip *` を [`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) に追加しても、そのコマンドをサンドボックスの外で実行することはできません。

<h3 id="copied-text-doesn’t-reach-your-local-clipboard-over-ssh">
  SSH 経由でコピーされたテキストがローカルクリップボードに到達しない
</h3>

Claude Code がリモートマシンで SSH 経由で実行されている場合、ローカルマシンでクリップボードツールを実行できません。tmux の外では、[フルスクリーンレンダリング](/docs/ja/fullscreen)でテキストを選択するか `/copy` を実行すると、Claude Code はテキストを OSC 52 エスケープシーケンスとしてターミナルに送信します。ターミナルがそれをクリップボードに配置するかどうかを決定します。`/copy` は、テキストが到達したかどうかに関わらず `Copied to clipboard` を報告し、tmux の外では選択通知は `sent N chars via OSC 52` と表示されます。

一部のターミナルは OSC 52 に対応していません。iTerm2 は **Settings > General > Selection > Applications in terminal may access clipboard** をオンにするまで無視し、macOS Terminal.app はそれをサポートしていません。

OSC 52 なしでテキストを取得するには：

* ターミナルのネイティブ選択キーを押しながらドラッグしてから、ターミナルの通常のショートカット（`Cmd+C` など）でコピーします。キーは Terminal.app では `Fn`、iTerm2 では `Option` です。[ネイティブテキスト選択を保持](/docs/ja/fullscreen#keep-native-text-selection)で他のターミナルのキーを一覧表示します。
* リモートマシンで [`CLAUDE_CODE_DISABLE_MOUSE=1`](/docs/ja/env-vars) を設定して、ターミナルがセッション全体の選択を処理するようにします。

<h3 id="search-and-discovery-issues">
  検索と発見の問題
</h3>

Search ツール、`@file` メンション、カスタムエージェント、またはカスタムスキルがファイルを見つけられない場合、バンドルされた `ripgrep` バイナリがシステムで実行されない可能性があります。プラットフォームの `ripgrep` パッケージをインストールして、Claude Code にそれを使用するよう指示します：

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```

    `ripgrep` は Alpine のコミュニティリポジトリにあります。`apk` がパッケージが見つからないと報告する場合は、[Alpine Linux セットアップ](/docs/ja/setup#alpine-linux-and-musl-based-distributions)を参照してください。
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

その後、`USE_BUILTIN_RIPGREP` を `0` に設定します。シェル[環境](/docs/ja/env-vars)または [`settings.json`](/docs/ja/settings-reference#all-settings) の `env` ブロックで設定します：

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

切り替えが有効になったことを確認するには、ターミナルで `claude doctor` を実行して、Search 行がシステム ripgrep のパスを表示していることを確認します。`OK (bundled)` ではなく。

<h3 id="slow-or-incomplete-search-results-on-wsl">
  WSL での遅い、または不完全な検索結果
</h3>

[WSL でファイルシステム間で作業する場合](https://learn.microsoft.com/en-us/windows/wsl/filesystems)のディスク読み取りパフォーマンスペナルティにより、WSL で Claude Code を使用する場合、Search ツール使用時に予想より少ないマッチが返される可能性があります。検索は機能しますが、ネイティブファイルシステムより少ない結果を返します。

<Note>
  `claude doctor` はこの場合、Search を OK として表示します。
</Note>

**解決策：**

1. **より具体的な検索を送信する**：検索するファイル数を減らすために、ディレクトリまたはファイルタイプを指定します：「auth-service パッケージで JWT 検証ロジックを検索」または「JS ファイルで md5 ハッシュの使用を見つける」。

2. **プロジェクトを Linux ファイルシステムに移動する**：可能であれば、プロジェクトが Windows ファイルシステム（`/mnt/c/`）ではなく Linux ファイルシステム（`/home/`）に配置されていることを確認します。

3. **ネイティブ Windows を使用する**：WSL ではなく Windows でネイティブに Claude Code を実行することを検討して、ファイルシステムのパフォーマンスを向上させます。

<h2 id="get-more-help">
  さらにヘルプを得る
</h2>

ここで説明されていない問題が発生している場合：

1. `/doctor` を実行してセットアップをチェックし、`/mcp` を実行して MCP サーバーのステータスを確認します
2. Claude Code 内で `/feedback` コマンドを使用して、Anthropic に問題を直接報告します
3. [GitHub リポジトリ](https://github.com/anthropics/claude-code)で既知の問題を確認します
4. Claude に直接その機能と機能について質問します。Claude はドキュメントへの組み込みアクセスを持っています。

アカウント、請求、またはサブスクリプションの問題については、代わりに Anthropic サポートにお問い合わせください。[claude.ai](https://claude.ai) にサインインし（Console ユーザーの場合：[platform.claude.com](https://platform.claude.com)）、左下のイニシャルをクリックして、**Get help** を選択します。各プランで人間のエージェントに連絡できるユーザーを含む完全なフローについては、[How to get support](https://support.claude.com/en/articles/9015913-how-to-get-support) を参照してください。
