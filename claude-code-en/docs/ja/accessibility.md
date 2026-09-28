> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# スクリーンリーダーで Claude Code を使用する

> VoiceOver や NVDA などのスクリーンリーダー、スクリーン拡大鏡、モーション削減、色覚異常対応テーマの設定で Claude Code をセットアップします。

Claude Code には、ビジュアルターミナルインターフェースをプレーンな線形テキストに置き換えるスクリーンリーダーモードがあります。ボックス、プログレスアニメーション、インプレース再描画の代わりに、Claude Code はラベル付きの行を出力し、VoiceOver や NVDA などのスクリーンリーダーが順番に読み上げます。完全な会話を保持し、ツール権限を承認し、出力を最後まで確認できます。

スクリーンリーダーモードはオプトインです。スクリーン拡大鏡、モーション削減、またはスクリーンリーダーの代わりにカラーブラインド対応テーマを使用する場合は、[アクセシビリティ設定](#accessibility-settings)テーブルから `CLAUDE_CODE_ACCESSIBILITY`、`prefersReducedMotion`、または `theme` を設定してください。スクリーンリーダーモードはターミナルインターフェースのみを適応させるため、VS Code 拡張機能のチャットパネルではこれを必要としません。Claude Code v2.1.236 以降では、拡張機能は設定なしで[スクリーンリーダーにコンバーセーション活動を通知](/docs/ja/vs-code#use-a-screen-reader)します。

<h2 id="turn-on-screen-reader-mode">
  スクリーンリーダーモードをオンにする
</h2>

スクリーンリーダーを使用する頻度に合わせて方法を選択してください。

* 1 つのセッション用：`claude --ax-screen-reader` を実行します。
* 1 つのシェルから開始されたセッション用：`CLAUDE_AX_SCREEN_READER` 環境変数を `1` に設定します。Bash または Zsh では `export CLAUDE_AX_SCREEN_READER=1` を実行し、PowerShell では `$env:CLAUDE_AX_SCREEN_READER = "1"` を実行します。シェルプロファイルにその行を追加して、今後のシェルでも保持します。
* マシン上のすべてのセッション用：ユーザー[設定ファイル](/docs/ja/settings)に `"axScreenReader": true` を追加します。この設定は VS Code 統合ターミナルを含むすべてのターミナルに適用されます。

メソッドを組み合わせた場合、Claude Code は [`--ax-screen-reader`](/docs/ja/cli-reference#cli-flags) フラグを [`CLAUDE_AX_SCREEN_READER`](/docs/ja/env-vars#variables) 環境変数より優先し、その環境変数を [`axScreenReader`](/docs/ja/settings-reference#axscreenreader) 設定より優先します。

SSH 経由で Claude Code を使用する場合は、Claude Code が実行されるリモートマシンで環境変数または設定を設定します。

Claude Code が最初に出力する行がモードを確認します。`[Screen Reader Mode: on via flag]`、`[Screen Reader Mode: on via env]`、または `[Screen Reader Mode: on via settings]` です。

<h2 id="turn-off-screen-reader-mode">
  スクリーンリーダーモードをオフにする
</h2>

モードをオンにした方法を逆にします。フラグなしで開始するか、環境変数を設定解除するか、`axScreenReader` を `false` に設定します。`CLAUDE_AX_SCREEN_READER` を `0` に設定すると、設定が `true` の場合でもモードはオフのままです。

<h2 id="accessibility-settings">
  アクセシビリティ設定
</h2>

次の表は、各アクセシビリティオプション、フラグ、環境変数、または設定として設定するかどうか、および何を変更するかを示しています。

| オプション                                                                   | タイプ  | 変更内容                                                                                                                                               |
| :---------------------------------------------------------------------- | :--- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/ja/cli-reference#cli-flags)                     | フラグ  | 1 つのセッションのスクリーンリーダーモード。                                                                                                                            |
| [`CLAUDE_AX_SCREEN_READER`](/docs/ja/env-vars#variables)                     | 環境変数 | それを設定したシェルから開始されたセッションのスクリーンリーダーモード。                                                                                                               |
| [`axScreenReader`](/docs/ja/settings-reference#axscreenreader)               | 設定   | `true` の場合、すべてのセッションのスクリーンリーダーモード。                                                                                                                 |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/ja/env-vars#variables)                  | 環境変数 | Claude Code が確認行の後、スクリーンリーダーモードで最初のプロンプトを描画する前に待機する時間。Claude Code v2.1.217 以降が必要です。                                                                |
| [`CLAUDE_AX_PREPARK_MS`](/docs/ja/env-vars#variables)                        | 環境変数 | Claude Code が行の開始時にカーソルを置いて、スクリーンリーダーモードで新しい行または変更された行を書き込む前に待機する時間。Claude Code v2.1.233 以降が必要です。                                                  |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/ja/env-vars#variables)                   | 環境変数 | `1` に設定した場合、macOS Zoom などのスクリーン拡大鏡に対して表示されたままのターミナルカーソル。カーソルは入力キャレットに従い、Claude Code v2.1.218 以降では、`/config` や `/plugin` などのメニューとパネルの強調表示された行に従います。 |
| [`prefersReducedMotion`](/docs/ja/settings-reference#prefersreducedmotion)   | 設定   | `true` の場合、スピナー、シマー、およびその他のアニメーションが削減または非表示になります。                                                                                                  |
| [`theme`](/docs/ja/settings-reference#theme)                                 | 設定   | 色覚異常対応の `dark-daltonized` および `light-daltonized` テーマを含むインターフェースカラー。[`/theme`](/docs/ja/commands#all-commands) で選択することもできます。                             |
| [`preferredNotifChannel`](/docs/ja/settings-reference#preferrednotifchannel) | 設定   | 値を `"terminal_bell"` にすると、Claude があなたを待機している場合、スクリーンリーダーモード外でターミナルベルが鳴ります。                                                                         |

<h2 id="what-your-screen-reader-hears">
  スクリーンリーダーが聞く内容
</h2>

スクリーンリーダーモードでは、Claude Code はフラットテキストを書き込みます：

* インターフェースクロームのボックス描画文字なし
* 色のみのキューなし
* 変更されていないコンテンツの再描画なし。プログレススピナーは静的テキストとしてレンダリングされます
* Claude の返信内のテーブルは、ボックス文字グリッドではなく `Header: value` 文として読み込まれます

Claude Code は、ターミナルのスクロールバックに印刷するすべてを残すため、スクリーンリーダーのレビューコマンドまたはターミナルの検索を使用して以前のターンを再度読むことができます。Claude Code は、スクリーンリーダーモードで [`tui` 設定](/docs/ja/settings-reference#tui) を無視します。[既知の制限事項](#known-limitations) に記載されている接続されたバックグラウンドセッションを除き、[フルスクリーンレンダリング](/docs/ja/fullscreen) の代わりにスクロールテキストを印刷します。

Claude Code は、スクリーンリーダーが追いつくことができるように 2 つのポイントで待機します：

* Claude Code が確認行を印刷した後、スクリーンリーダーが行を完了できるようにプロンプトを描画する前に 3 秒待機します。任意のキーを押して待機を終了します。待機の長さを変更するには、[`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/ja/env-vars#variables) を設定します。
* Claude Code が新しい行または変更された行（ヒントや Claude の返信の詳細など）を書き込む前に、カーソルを行の開始位置に移動して 50 ミリ秒待機します。その後、スクリーンリーダーは最初の文字から行を読み込みます。入力行の末尾に入力または削除した文字は直ちに表示されます。待機の長さを変更するには、[`CLAUDE_AX_PREPARK_MS`](/docs/ja/env-vars#variables) を設定します。

トランスクリプト内の各メッセージは、スクリーンリーダーが発表するラベルで始まり、それが何であるかを名前付けします：あなたのメッセージ、Claude の返信と思考、ツールアクティビティ、エラーと警告、およびプロンプト。ラベルは検索可能でもあるため、ターミナルのスクロールバックを検索してトランスクリプトのセクション間をジャンプできます：

| ラベル                    | 意味                                                                |
| :--------------------- | :---------------------------------------------------------------- |
| `you:`                 | あなたのメッセージ                                                         |
| `claude:`              | Claude の返信                                                        |
| `thinking:`            | Claude の思考                                                        |
| `tool:`                | ファイル編集やコマンド実行などのツールアクティビティ                                        |
| `tool error:`          | 失敗したツール                                                           |
| `error:`               | 失敗した API リクエストなどの会話内のエラー                                          |
| `warning:`             | モデルフォールバックへの切り替えなど、Claude Code からの警告                              |
| `Permission Required:` | あなたの回答を待つ権限プロンプト                                                  |
| `Cost:`                | Claude Code が終了するときのセッションコスト概要（アカウントが [コストを表示](/docs/ja/costs) している場合） |

Claude Code はターミナルカーソルを入力キャレットに保つため、スクリーンリーダーの現在の行を読むコマンドは、編集しているプロンプトを読み込みます。

入力行の末尾に入力するとき、または `Backspace` を押すと、Claude Code は変更された文字のみを書き込みます。スクリーンリーダーはそれらの文字のみをエコーします。

[テキスト編集ショートカット](/docs/ja/interactive-mode#text-editing) の 1 つで単語または行を削除すると、Claude Code は削除されたテキストを発表します：

* `Ctrl+W` または `Alt+D` で単語を削除、または macOS では `Option+Delete`、Windows では `Ctrl+Backspace`
* `Ctrl+U` または `Cmd+Backspace` で行の開始位置まで削除
* `Ctrl+K` で行の終了位置まで削除

[権限モード](/docs/ja/permission-modes) を `Shift+Tab` でサイクルすると、Claude Code は `[plan mode on]` または `[accept edits on]` などのランディングした権限モードを発表します。Claude Code は発表を 1 回印刷し、後の再描画では繰り返しません。

<h3 id="jump-between-turns">
  ターン間をジャンプする
</h3>

Claude Code はターン境界で OSC 133 シェル統合マーカーを発行するため、ターミナルの前のプロンプトへジャンプするキーはトランスクリプト全体を読まずにターン間を移動します：

* iTerm2：Cmd+Shift+Up
* VS Code ターミナル：Windows では Ctrl+Up、macOS では Cmd+Up
* Windows Terminal：デフォルトではキーなし。設定で `scrollToMark` アクションをバインドします
* Kitty と Ghostty：ターミナルのドキュメントでジャンプ・ツー・プロンプトキーを確認してください

macOS Terminal はマーカーに作用せず、Claude Code は WezTerm では発行しません。これらのターミナルでは、代わりにスクロールバックで `you:` ラベルを検索してください。

<h2 id="answer-menus-and-prompts">
  メニューとプロンプトに答える
</h2>

スクリーンリーダーモードでは、通常は矢印キーで操作するメニュー（権限プロンプトを含む）が番号付きリストになります。Claude Code は各オプションを番号付き行として発表し、その後に有効な範囲を示す `Enter selection` プロンプトを発表します。希望するオプションの番号を入力して Enter キーを押します。

* `or Escape to cancel` で終わるプロンプトのメニューをキャンセルするには Escape キーを押します。
* リストにない番号を入力した場合、Claude Code は有効な範囲を発表し、もう一度試すことができます。

スクリーンリーダーモード外ではスライダーである [`/effort`](/docs/ja/model-config#adjust-effort-level) セレクターは、同じ種類の番号付きリストになります。

はい/いいえプロンプトは、2 つのオプションメニューの代わりに、入力された回答を求めます。`y` または `n` と入力して Enter キーを押します。`yes` と `no` も機能します。

<h2 id="hear-when-claude-code-needs-you">
  Claude Code があなたの注意を必要とするときに聞く
</h2>

スクリーンリーダーモードでは、Claude Code があなたの注意を必要とするときにターミナルベルが鳴るため、トランスクリプトを常に確認する必要がありません。ベルは以下の場合に鳴ります：

* Claude が返信を完了した
* 権限プロンプトなど、プロンプトまたはダイアログがあなたの回答を必要としている
* 5 秒以上実行されたツールが完了した

ベルはターミナルの標準アラートです。これを消音にするには、ターミナルアプリケーションのベル設定を変更してください。スクリーンリーダーモード外では、[`preferredNotifChannel`](/docs/ja/settings-reference#preferrednotifchannel) を `"terminal_bell"` に設定して、Claude があなたの対応を待っているときに[同様のベル](/docs/ja/terminal-config#get-a-terminal-bell-or-notification)を取得します。

<h2 id="known-limitations">
  既知の制限事項
</h2>

一部の動作はスクリーンリーダーモード用に適応していません。

* スクリーンリーダーモードは、スクリーンリーダーが実行されているときに自動的にオンになりません。
* Claude Code は `Shift+Tab` でサイクリングする以外の方法で行われた権限モード変更を発表しません。例えば、コマンドから[プランモード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)に入るなどです。
* `claude attach` または agent view から[バックグラウンドセッション](/docs/ja/agent-view)にアタッチすると、ターミナルの代替画面に入ります。これはネイティブスクロールバックがありません。これは[他のアタッチされたセッションと同じ動作](/docs/ja/fullscreen)です。抜け出すには、空のプロンプトで左矢印を押すか、ダイアログにフォーカスがある場合は Ctrl+Z を押します。
* Claude Code はコストを終了時に出力するサマリーで発表し、ターンごとではありません。
* スクリーンリーダーモードは `-p` フラグで[非対話型モード](/docs/ja/headless)を変更しません。非対話型モードは既にプレーンテキストを書き込み、スクリプト作成の代替案のままです。

<h2 id="report-an-issue">
  問題を報告する
</h2>

スクリーンリーダー、拡大鏡、またはターミナルで何かが機能しない場合は、[Claude Code issue tracker](https://github.com/anthropics/claude-code/issues) で問題を開き、タイトルに支援技術を記載してください。レポートにオペレーティングシステム、ターミナルアプリケーション、支援技術の名前とバージョンを含めます。
