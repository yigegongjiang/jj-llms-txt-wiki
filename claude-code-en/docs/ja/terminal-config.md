> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 用にターミナルを設定する

> Shift+Enter で改行を挿入する、Claude の処理完了時にターミナルベルを鳴らす、tmux を設定する、カラーテーマを合わせる、Claude Code CLI で Vim モードを有効にする方法を説明します。

Claude Code はターミナル設定なしで任意のターミナルで動作します。このページは、特定の動作が期待通りに機能していない場合のためのものです。以下から症状を見つけてください。すべてが既に正常に動作している場合は、このページは不要です。

* [Shift+Enter が改行を挿入する代わりに送信する](#enter-multiline-prompts)
* [macOS でオプションキーのショートカットが機能しない](#enable-option-key-shortcuts-on-macos)
* [Claude の処理完了時に音またはアラートが出ない](#get-a-terminal-bell-or-notification)
* [Claude Code を tmux 内で実行している](#configure-tmux)
* [Windows でバックスペースが単語全体を削除する](#fix-backspace-deleting-a-whole-word-on-windows)
* [表示がちらつくまたはスクロールバックがジャンプする](#switch-to-fullscreen-rendering)
* [プロンプトで Vim キーを使いたい](#edit-prompts-with-vim-keybindings)

このページは、ターミナルが Claude Code に正しい信号を送信するようにするためのものです。Claude Code 自体が応答するキーを変更するには、代わりに [キーバインディング](/docs/ja/keybindings) を参照してください。

<h2 id="enter-multiline-prompts">
  複数行のプロンプトを入力する
</h2>

Enter キーを押すとメッセージが送信されます。送信せずに改行を追加するには、Ctrl+J を押すか、`\` を入力してから Enter を押してください。どちらの方法もセットアップなしですべてのターミナルで機能します。

ほとんどのターミナルでは Shift+Enter を押すこともできますが、サポートはターミナルエミュレータによって異なります。

| ターミナル                                                             | 改行用の Shift+Enter                         |
| :---------------------------------------------------------------- | :--------------------------------------- |
| Ghostty、Kitty、iTerm2、WezTerm、Warp、Apple Terminal、Windows Terminal | セットアップなしで機能                              |
| kitty キーボードプロトコルをサポートする他のターミナル（foot や Alacritty 0.16 以降など）        | セットアップなしで機能。Claude Code v2.1.269 以降が必要です |
| VS Code、Cursor、Devin Desktop、Alacritty 0.16 より前のバージョン、Zed         | 1 回 `/terminal-setup` を実行                |
| gnome-terminal、PyCharm や Android Studio などの JetBrains IDE         | 利用不可。Ctrl+J または `\` の後に Enter を使用        |

VS Code、Cursor、Devin Desktop、Alacritty 0.16 より前のバージョン、Zed の場合、`/terminal-setup` はターミナルの設定ファイルに Shift+Enter キーバインディングを書き込みます。初回実行時には `Installed VSCode terminal Shift+Enter key binding` などの確認メッセージが表示されます。既存のバインディングはそのまま保持されます。`VSCode terminal Shift+Enter key binding already configured` などのメッセージが表示される場合は、変更は加えられていません。`/terminal-setup` は tmux または screen の内部ではなく、ホストターミナル内で直接実行してください。ホストターミナルの設定に書き込む必要があるためです。

VS Code、Cursor、Devin Desktop では、`/terminal-setup` は 2 つのエディタ設定も更新します。統合ターミナルでのテキストの乱れを防ぐために `terminal.integrated.gpuAcceleration` を `"off"` に設定し、[フルスクリーンモード](/docs/ja/fullscreen)でのスムーズなスクロールのために `terminal.integrated.mouseWheelScrollSensitivity` を設定します。GPU アクセラレーション変更を元に戻すには、`"auto"` に設定し直してエディタウィンドウをリロードしてください。

Zed では、`/terminal-setup` は `keymap.json` をその場で更新します。

* キーマップに既にバインディングがあり、そのいずれもターミナルの `shift-enter` ではない場合、Claude Code はまずそれを同じディレクトリ内のコピー（`keymap.json.1a2b3c4d.bak` など）にバックアップしてから、Shift+Enter バインディングをキーマップにマージし、他のキーバインディングとコメントを保持します
* Claude Code がキーマップを読み込めない、解析できない、バックアップできない、またはマージされた結果を検証できない場合、[ファイルを変更せず、追加するキーバインディングブロックを出力します](/docs/ja/errors#terminal-setup-left-your-zed-keymap-unchanged)

tmux 内で実行している場合、外側のターミナルがサポートしている場合でも、Shift+Enter には以下の [tmux 設定](#configure-tmux)が必要です。

改行を別のキーにバインドするか、Enter が改行を挿入し Shift+Enter が送信するように動作を入れ替えるには、[キーバインディングファイル](/docs/ja/keybindings)で `chat:newline` および `chat:submit` アクションをマップしてください。

<h2 id="enable-option-key-shortcuts-on-macos">
  macOS で Option キーショートカットを有効にする
</h2>

Claude Code の一部のショートカットは Option キーを使用します。例えば、Option+Enter で改行したり、Option+P でモデルを切り替えたりします。macOS では、ほとんどのターミナルはデフォルトで Option を修飾キーとして送信しないため、これを有効にするまでこれらのショートカットは機能しません。ターミナル設定は通常「Use Option as Meta Key」というラベルが付いています。Meta は、現在 Option または Alt というラベルが付いているキーの歴史的な Unix 名です。

<Tabs>
  <Tab title="Apple Terminal">
    設定 → プロファイル → キーボードを開き、「Use Option as Meta Key」をチェックします。

    Claude Code の初回実行時のターミナルセットアッププロンプトを受け入れた場合、これはすでに完了しています。そのプロンプトは `/terminal-setup` を実行し、Option を Meta として有効にし、Apple Terminal プロファイルで可聴ベルをオフにします。

    [スクリーンリーダーモード](/docs/ja/accessibility)では、`/terminal-setup` はベル設定を変更しないままにするため、ターミナルベルは可聴のままです。v2.1.211 より前では、`/terminal-setup` はスクリーンリーダーモードでもベルをオフにしていました。以前の実行でベルがオフになった場合は、設定 → プロファイル → 詳細 → 「可聴ベル」でオンに戻してください。
  </Tab>

  <Tab title="iTerm2">
    設定 → プロファイル → キー → 一般を開き、左 Option キーと右 Option キーを「Esc+」に設定します。

    iTerm2 で `/terminal-setup` を実行すると、設定 → 一般 → 選択の下で「Applications in terminal may access clipboard」が有効になり、`/copy` コマンドがシステムクリップボードに書き込むことができます。このコマンドは tmux 内から実行された場合でも iTerm2 を検出します。変更を有効にするために iTerm2 を再起動してください。
  </Tab>

  <Tab title="VS Code">
    VS Code 設定に `"terminal.integrated.macOptionIsMeta": true` を追加します。
  </Tab>
</Tabs>

Ghostty、Kitty、およびその他のターミナルについては、ターミナルの設定ファイルで Option-as-Alt または Option-as-Meta 設定を探してください。

<h2 id="get-a-terminal-bell-or-notification">
  ターミナルベルまたは通知を取得する
</h2>

Claude がタスクを完了するか権限プロンプトで一時停止し、ターミナルから離れているように見える場合、通知イベントが発火します。[各通知タイプが発火するタイミング](/docs/ja/hooks#notification)を参照して、正確なタイミングを確認してください。これをターミナルベルまたはデスクトップ通知として表示することで、長いタスクが実行されている間に他の作業に切り替えることができます。

デフォルトでは Claude Code は Ghostty、Kitty、および iTerm2 でのみデスクトップ通知を送信します。他のターミナルでは、[`preferredNotifChannel`](/docs/ja/settings-reference#preferrednotifchannel)を`"terminal_bell"`に設定してターミナルベルを鳴らすか、カスタム音またはコマンド用に[通知フック](#play-a-sound-with-a-notification-hook)を設定してください。次の設定エントリはターミナルベルをオンにします：

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

デスクトップ通知は SSH 経由でローカルマシンに到達するため、リモートセッションでもアラートを受け取ることができます。Ghostty と Kitty はさらなるセットアップなしで OS 通知センターに転送します。iTerm2 では転送を有効にする必要があります：

<Steps>
  <Step title="Open iTerm2 notification settings">
    Go to Settings → Profiles → Terminal.
  </Step>

  <Step title="Enable alerts">
    Check "Notification Center Alerts", then click "Filter Alerts" and enable "Send escape sequence-generated alerts".
  </Step>
</Steps>

通知がまだ表示されない場合は、ターミナルアプリケーションが OS 設定で通知権限を持っていることを確認し、tmux 内で実行している場合は[パススルーを有効にしてください](#configure-tmux)。

<h3 id="play-a-sound-with-a-notification-hook">
  通知フックで音を再生する
</h3>

任意のターミナルで[通知フック](/docs/ja/hooks-guide#get-notified-when-claude-needs-input)を設定して、Claude があなたの注意が必要な場合に音を再生するか、カスタムコマンドを実行できます。フックは組み込み通知と並行して実行され、それを置き換えるのではなく、Warp や VS Code 統合ターミナルなどのデスクトップ通知を受け取らないターミナルは、フックを使用するか、代わりに`preferredNotifChannel`を`"terminal_bell"`に設定できます。

以下の例は macOS でシステム音を再生します。リンクされたガイドには、macOS、Linux、および Windows のデスクトップ通知コマンドがあります。

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  tmux を設定する
</h2>

Claude Code が tmux 内で実行される場合、デフォルトでは Shift+Enter が改行を挿入する代わりに送信してしまい、デスクトップ通知と[プログレスバー](/docs/ja/settings-reference#terminalprogressbarenabled)が外側のターミナルに到達しません。これらの行を `~/.tmux.conf` に追加してから、`tmux source-file ~/.tmux.conf` を実行して、実行中のサーバーに適用してください。

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

`allow-passthrough` 行により、通知とプログレス更新が tmux に飲み込まれるのではなく、外側のターミナルに到達できるようになります。`extended-keys` 行により、tmux が Shift+Enter と通常の Enter を区別できるようになり、改行ショートカットが機能するようになります。

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Windows で Backspace がワード全体を削除する問題を修正する
</h2>

Windows では、Claude Code は Backspace として到着した `^H` を Ctrl+Backspace として読み込み、[前のワードを削除します](/docs/ja/interactive-mode#text-editing)。ただし、`TERM_PROGRAM` が `mintty` または `TERM` が `cygwin` の場合は除きます。macOS と Linux では、Claude Code はこれをプレーン Backspace として読み込みます。

Backspace を押すたびにワード全体が削除される場合、ターミナルはプレーン Backspace に対して `^H` を送信しています。[`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/ja/env-vars) を設定してください。その後、Backspace と Ctrl+H は各々 1 文字を削除します。macOS または Linux で Ctrl+Backspace が 1 文字だけを削除する場合、ターミナルが `^H` を送信しているため、代わりに変数を `1` に設定してください。

<h2 id="match-the-color-theme">
  カラーテーマを合わせる
</h2>

`/theme` コマンドを使用するか、`/config` のテーマピッカーを使用して、ターミナルに合わせた Claude Code テーマを選択します。auto オプションを選択すると、ターミナルの明るい背景または暗い背景が検出されるため、ターミナルが OS の外観の変更に従うたびにテーマが従います。Claude Code はターミナルアプリケーションによって設定されるターミナル自体のカラースキームを制御しません。

インターフェースの下部に表示される内容をカスタマイズするには、現在のモデル、作業ディレクトリ、git ブランチ、またはその他のコンテキストを表示する[カスタムステータスライン](/docs/ja/statusline)を設定します。

<h3 id="create-a-custom-theme">
  カスタムテーマを作成する
</h3>

組み込みプリセットに加えて、`/theme` には定義したカスタムテーマと、インストール済みの[プラグイン](/docs/ja/plugins/components#themes-and-output-styles)によって提供されたテーマが一覧表示されます。リストの最後にある\*\*新しいカスタムテーマ…\*\*を選択して、対話的に作成します。テーマに名前を付けてから、個別のカラートークンを選択してオーバーライドします。カスタムテーマがハイライトされている間に `Ctrl+E` を押して編集します。

各カスタムテーマは `~/.claude/themes/` 内の JSON ファイルです。`.json` 拡張子を除いたファイル名がテーマのスラッグであり、テーマを選択すると `custom:<slug>` がテーマの設定として保存されます。ファイルには 3 つのオプションフィールドがあります。

| フィールド       | タイプ    | 説明                                                                                                                 |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- |
| `name`      | string | `/theme` に表示されるラベル。デフォルトはファイル名スラッグ                                                                                 |
| `base`      | string | テーマの開始元となる組み込みプリセット：`dark`、`light`、`dark-daltonized`、`light-daltonized`、`dark-ansi`、または `light-ansi`。デフォルトは `dark` |
| `overrides` | object | カラートークン名をカラー値にマップします。ここにリストされていないトークンはベースプリセットにフォールスルーします                                                          |

カラー値は `#rrggbb`、`#rgb`、`rgb(r,g,b)`、`ansi256(n)`、または `ansi:<name>` を受け入れます。ここで `<name>` は `red` や `cyanBright` などの 16 個の標準 ANSI カラー名の 1 つです。不明なトークンと無効なカラー値は無視されるため、タイプミスはレンダリングを破壊することはできません。

次の例は、暗いプリセットを保持しながら、プロンプトアクセント、エラーテキスト、成功テキストを再色付けするテーマを定義しています。

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code は `~/.claude/themes/` を監視し、ファイルが追加または変更されるとリロードするため、エディターで行われた編集は再起動なしで実行中のセッションに適用されます。Claude Code の起動時に `~/.claude/themes/` フォルダ自体が存在しなかった場合は、最初のテーマファイルを作成した後に 1 回再起動します。その後、変更は再起動なしで適用されます。

以下のリファレンスは、`overrides` で設定できるトークンをカバーしています。`/theme` の対話的エディターは、同じトークンをライブプレビューと共に表示し、さらにオンボーディング画面のカラーなどの単一目的のアクセントをいくつか表示します（ここでは省略されています）。

<Accordion title="カラートークンリファレンス">
  次の例は、以下のいくつかのグループからトークンを組み合わせています。ブランドアクセント、プランモードボーダー、diff 背景、およびメッセージ背景です。

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    テキストとアクセントカラー
  </h4>

  プライマリブランドアクセントと、インターフェース全体で使用されるフォアグラウンドテキストの色合いを制御します。

  | トークン          | 制御対象                             |
  | :------------ | :------------------------------- |
  | `claude`      | プライマリブランドアクセント、スピナーとアシスタントラベルに使用 |
  | `text`        | デフォルトのフォアグラウンドテキスト               |
  | `inverseText` | ステータスバッジなどの色付き背景の上に描画されるテキスト     |
  | `inactive`    | ヒント、タイムスタンプ、無効な項目などのセカンダリテキスト    |
  | `subtle`      | 薄いボーダーと強調されていないセカンダリテキスト         |
  | `suggestion`  | オートコンプリート候補とピッカーの選択ハイライト         |
  | `permission`  | パーミッションプロンプトとピッカーを含むダイアログボーダー    |
  | `remember`    | メモリと `CLAUDE.md` インジケーター         |

  <h4 id="status-colors">
    ステータスカラー
  </h4>

  メッセージとインジケーター全体で成功、失敗、警告状態を通知します。

  | トークン      | 制御対象                           |
  | :-------- | :----------------------------- |
  | `success` | 成功メッセージと合格チェック                 |
  | `error`   | エラーメッセージと失敗                    |
  | `warning` | 警告、注意メッセージ、および auto モードインジケーター |
  | `merged`  | マージされたプルリクエストステータス             |

  <h4 id="input-box-and-mode-indicators">
    入力ボックスとモードインジケーター
  </h4>

  入力ボックスのボーダーカラーと、パーミッションモードまたはインジケーターがアクティブな間に表示されるアクセントを設定します。

  | トークン           | 制御対象                                                                                                                              |
  | :------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
  | `promptBorder` | 入力ボックスボーダー                                                                                                                        |
  | `planMode`     | Plan モードアクセント、プランメッセージ、および Plan モードダイアログ                                                                                          |
  | `autoAccept`   | Accept-edits モードアクセント                                                                                                             |
  | `bashBorder`   | `!` シェルコマンドを入力するときの入力ボックスボーダー                                                                                                     |
  | `ide`          | IDE 接続インジケーター                                                                                                                     |
  | `fastMode`     | Fast モードインジケーター                                                                                                                   |
  | `effortUltra`  | [ultracode](/docs/ja/model-config#adjust-effort-level) がオンの間、入力ボックスボーダーの `ultracode` タグ。このカラーのオーバーライドは Claude Code v2.1.239 以降で有効になります |

  <h4 id="diff-rendering">
    Diff レンダリング
  </h4>

  ファイル編集とレビューで追加および削除されたコードを色付けします。

  | トークン                | 制御対象                          |
  | :------------------ | :---------------------------- |
  | `diffAdded`         | 追加された行の背景                     |
  | `diffRemoved`       | 削除された行の背景                     |
  | `diffAddedDimmed`   | 編集を拒否した後に表示される薄い diff の追加行の背景 |
  | `diffRemovedDimmed` | 編集を拒否した後に表示される薄い diff の削除行の背景 |
  | `diffAddedWord`     | 追加された行内のワードレベルハイライト           |
  | `diffRemovedWord`   | 削除された行内のワードレベルハイライト           |

  <h4 id="fullscreen-mode">
    フルスクリーンモード
  </h4>

  Claude Code は、デフォルトとフルスクリーンレンダラーの両方で `userMessageBackground`、`bashMessageBackgroundColor`、および `memoryBackgroundColor` を描画します。[フルスクリーンレンダリングモード](/docs/ja/fullscreen)でのみ `userMessageBackgroundHover` と `selectionBg` を使用します。

  | トークン                         | 制御対象                             |
  | :--------------------------- | :------------------------------- |
  | `userMessageBackground`      | トランスクリプト内のメッセージの背後の背景            |
  | `userMessageBackgroundHover` | ホバーまたは展開されている間のメッセージの背後の背景       |
  | `bashMessageBackgroundColor` | トランスクリプト内の `!` シェルコマンドエントリの背後の背景 |
  | `memoryBackgroundColor`      | トランスクリプト内の `#` メモリエントリの背後の背景     |
  | `selectionBg`                | マウスで選択されたテキストの背景                 |

  <h4 id="usage-meter-and-speaker-labels">
    使用量メーターとスピーカーラベル
  </h4>

  `/usage` ビューに表示されるバーと、メッセージを Claude のメッセージと区別するラベルを調整します。

  | トークン               | 制御対象                          |
  | :----------------- | :---------------------------- |
  | `rate_limit_fill`  | 使用量メーターの塗りつぶされた部分             |
  | `rate_limit_empty` | 使用量メーターの塗りつぶされていない部分          |
  | `briefLabelYou`    | メッセージの `You` ラベルのカラー          |
  | `briefLabelClaude` | アシスタントメッセージの `Claude` ラベルのカラー |

  <h4 id="shimmer-variants-and-subagent-colors">
    シマーバリアントとサブエージェントカラー
  </h4>

  いくつかのトークンには、スピナーのアニメーション化されたグラデーションで使用される明るいカラーを提供するペアのシマーバリアントがあります。アニメーションが一致しないように見える場合は、ベーストークンと一緒にシマーをオーバーライドします。

  * `claude` と `claudeShimmer`
  * `warning` と `warningShimmer`
  * `permission` と `permissionShimmer`
  * `promptBorder` と `promptBorderShimmer`
  * `inactive` と `inactiveShimmer`
  * `fastMode` と `fastModeShimmer`

  各[サブエージェント](/docs/ja/sub-agents)と並列タスクは、トランスクリプト内で区別できるように 8 つの名前付きカラーの 1 つで表示されます。トークン名は `<color>_FOR_SUBAGENTS_ONLY` パターンに従います。ここで `<color>` は `red`、`blue`、`green`、`yellow`、`purple`、`orange`、`pink`、または `cyan` です。これらをオーバーライドして、各名前付きカラーの外観を変更します。たとえば、定義に `color: blue` を持つサブエージェントは、`blue_FOR_SUBAGENTS_ONLY` 値を使用して描画されます。

  Claude Code は、プロンプト入力で [`ultrathink`](/docs/ja/model-config#use-ultrathink-for-one-off-deep-reasoning) キーワードを 7 色の虹グラデーションでレンダリングします。トークン名は `rainbow_<color>` と `rainbow_<color>_shimmer` パターンに従います。ここで `<color>` は `red`、`orange`、`yellow`、`green`、`blue`、`indigo`、または `violet` です。
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  フルスクリーンレンダリングに切り替える
</h2>

[スクリーンリーダーモード](/docs/ja/accessibility)では、このセクションは適用されません。Claude Code は、添付された[バックグラウンドセッション](/docs/ja/agent-view)を除き、常にプレーンなスクロールテキストとしてレンダリングされます。また、他のセッションで `/tui fullscreen` を実行した場合、Claude Code はレンダラーを切り替える代わりに説明を出力します。

ディスプレイがちらつくか、Claude が作業中にスクロール位置がジャンプする場合は、[フルスクリーンレンダリングモード](/docs/ja/fullscreen)に切り替えてください。このモードでは、ターミナルのネイティブスクロールバックではなく、マウスまたは PageUp を使用して Claude Code 内でスクロールします。検索とコピーの方法については、[フルスクリーンページ](/docs/ja/fullscreen#search-and-review-the-conversation)を参照してください。

ちらつきが唯一の問題で、ターミナルが同期出力をサポートしているが自動検出されていない場合（Emacs `eat` など）、[`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/ja/env-vars)を設定して、レンダラーを変更せずにちらつきを停止します。

`/tui fullscreen` を実行して切り替え、設定を保存します。会話はそのまま再起動され、[フルスクリーン開始に失敗](/docs/ja/fullscreen#fullscreen-renderer-didnt-finish-starting)しない限り、今後のセッションはフルスクリーンで開始されます。Claude Code を起動する前に、`CLAUDE_CODE_NO_FLICKER` 環境変数を設定することもできます。

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  大量のコンテンツを貼り付ける
</h2>

800 文字以上または 3 行以上をプロンプトに貼り付けると、Claude Code は入力を `[Pasted text #1 +120 lines]` などのプレースホルダーに折りたたんで、入力ボックスが使用可能な状態を保ちます。送信時に完全なコンテンツを送信します。ファイル全体やログが長いなど、非常に大きな入力の場合は、コンテンツをファイルに書き込んで、貼り付けの代わりに Claude にそれを読むよう依頼してください。会話トランスクリプトが読みやすくなり、Claude が後のターンでパスによってファイルを参照できます。VS Code 統合ターミナルは、非常に大きな貼り付けから Claude Code に到達する前に文字をドロップできるため、そこではファイルを使用してください。

貼り付けに [目に見えない Unicode 文字](/docs/ja/interactive-mode#invisible-characters-in-prompts) が含まれている場合、Claude Code は Enter キーを押すときにそれらを削除し、クリーンアップされたプロンプトを入力ボックスに戻して、別の Enter で送信できるようにします。

<h3 id="how-claude-treats-pasted-text">
  Claude が貼り付けたテキストをどのように処理するか
</h3>

送信すると、Claude は各 `[Pasted text #N]` プレースホルダーの背後にあるコンテンツを、他の場所から貼り付けたテキストとして表示され、入力したテキストではなく表示されます。Claude には、貼り付けには入力していない指示が含まれる可能性があり、入力したメッセージがそれを要求する場所でのみ指示に従うよう指示されます。[機能フラグを取得](/docs/ja/env-vars#features-that-need-feature-flag-fetching) しないセッションでは、貼り付けはマークされません。

<h3 id="delete-and-restore-a-collapsed-paste">
  折りたたまれた貼り付けを削除して復元する
</h3>

`Ctrl+W` や `Ctrl+K` などの単語または行ショートカットで削除するか、`df]` などの `f`/`t` モーションを使用した vim 削除で削除し、削除範囲が `[Pasted text #N]` プレースホルダー内に達する場合、Claude Code はプレースホルダー全体を削除します。復元するには、単語または行ショートカットの後に [`Ctrl+Y`](/docs/ja/interactive-mode#text-editing) で削除を貼り付け直すか、vim 削除の後に [`p` NORMAL モード](/docs/ja/interactive-mode#editing-normal-mode) で復元してください。

<h3 id="recall-a-prompt-that-had-pasted-text">
  貼り付けたテキストを含むプロンプトを呼び出す
</h3>

Claude Code は折りたたまれたコンテンツを各 `[Pasted text #N]` プレースホルダーの背後に `~/.claude/paste-cache/` に保持するため、[コマンド履歴](/docs/ja/interactive-mode#command-history) からプロンプトを呼び出して再送信すると、完全な貼り付けコンテンツが再度送信され、後のセッションでも含まれます。

[保持期間スイープルール](/docs/ja/claude-directory#cleaned-up-automatically) に従い、[`cleanupPeriodDays`](/docs/ja/settings-reference#cleanupperioddays) より古いキャッシュファイルは削除されるため、呼び出されたプロンプトは存在しなくなった貼り付けテキストを参照できます。そのようなプロンプトを送信すると、Claude Code はリテラル `[Pasted text #N]` 文字列を送信することはなく、不足している貼り付けの名前を示す通知を表示します。

* テキストが残っているプレーンプロンプトでは、Claude Code はプレースホルダーを削除して残りのテキストを送信します。
* [シェルモード](/docs/ja/interactive-mode#shell-mode-with-prefix) コマンドまたは `/` コマンドでは、削除が実行内容を変更する可能性があり、プロンプトの削除が空のままになる場合、Claude Code は送信をキャンセルし、元のテキストを入力に保持します。プレースホルダーはまだ含まれています。プレースホルダーを削除するか、コマンドを編集してから再送信してください。

<h2 id="edit-prompts-with-vim-keybindings">
  Vim キーバインディングでプロンプトを編集する
</h2>

Claude Code には、プロンプト入力用の Vim スタイルの編集モードが含まれています。`/config` → Editor mode から有効にするか、`~/.claude/settings.json` で [`editorMode`](/docs/ja/settings-reference#editormode) を `"vim"` に設定することで有効にできます。Editor mode を `normal` に戻すと、オフになります。

Vim モードは、NORMAL モードと VISUAL モードのモーション演算子のサブセットをサポートしており、`hjkl` ナビゲーション、`v`/`V` 選択、テキストオブジェクトを使用した `d`/`c`/`y` などが含まれます。完全なキーテーブルについては、[Vim エディタモードリファレンス](/docs/ja/interactive-mode#vim-editor-mode)を参照してください。

Vim モーションはキーバインディングファイルを通じて再マップできません。`jj` を Escape にマップするなどの 2 キー INSERT モードシーケンスをマップするには、ユーザー設定で [`vimInsertModeRemaps`](/docs/ja/interactive-mode#remap-insert-mode-key-sequences) を設定してください。

INSERT モードで Enter キーを押すと、標準的な Vim とは異なり、プロンプトが送信されます。代わりに NORMAL モードで `o` または `O` を使用するか、Ctrl+J を使用して改行を挿入してください。

<h2 id="related-resources">
  関連リソース
</h2>

* [インタラクティブモード](/docs/ja/interactive-mode)：完全なキーボードショートカットリファレンスと Vim キーテーブル
* [キーバインディング](/docs/ja/keybindings)：Enter と Shift+Enter を含む任意の Claude Code ショートカットを再マップ
* [フルスクリーンレンダリング](/docs/ja/fullscreen)：フルスクリーンモードでのスクロール、検索、コピーの詳細
* [フック ガイド](/docs/ja/hooks-guide)：Linux と Windows の詳細な通知フック例
* [トラブルシューティング](/docs/ja/troubleshooting)：ターミナル設定外の問題の修正
