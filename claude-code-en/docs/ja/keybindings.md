> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# キーボードショートカットのカスタマイズ

> キーボードショートカットをカスタマイズして、Claude Code でキーバインディング設定ファイルを使用します。

Claude Code はカスタマイズ可能なキーボードショートカットをサポートしています。`/keybindings` を実行して、`~/.claude/keybindings.json` に設定ファイルを作成または開きます。

<h2 id="configuration-file">
  設定ファイル
</h2>

キーバインディング設定ファイルは、`bindings` 配列を持つオブジェクトです。各ブロックはコンテキストとキーストロークからアクションへのマップを指定します。

<Note>キーバインディングファイルへの変更は自動的に検出され、Claude Code を再起動することなく適用されます。</Note>

| フィールド      | 説明                                  |
| :--------- | :---------------------------------- |
| `$schema`  | エディタのオートコンプリート用のオプション JSON スキーマ URL |
| `$docs`    | オプションのドキュメント URL                    |
| `bindings` | コンテキスト別のバインディングブロックの配列              |

この例では、チャットコンテキストで `Ctrl+E` を外部エディタを開くにバインドし、`Ctrl+U` をアンバインドします。

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/ja/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  コンテキスト
</h2>

各バインディングブロックは、バインディングが適用される**コンテキスト**を指定します。

| コンテキスト            | 説明                                                |
| :---------------- | :------------------------------------------------ |
| `Global`          | アプリ全体に適用                                          |
| `Chat`            | メインチャット入力エリア                                      |
| `Autocomplete`    | オートコンプリートメニューが開いている                               |
| `Settings`        | 設定メニュー                                            |
| `Confirmation`    | 権限と確認ダイアログ                                        |
| `Tabs`            | タブナビゲーションコンポーネント                                  |
| `Help`            | ヘルプメニューが表示されている                                   |
| `Transcript`      | トランスクリプトビューア                                      |
| `HistorySearch`   | 履歴検索モード（Ctrl+R）                                   |
| `Task`            | バックグラウンドタスクが実行中                                   |
| `ThemePicker`     | テーマピッカーダイアログ                                      |
| `Attachments`     | 選択ダイアログ内の画像添付ファイルナビゲーション                          |
| `Footer`          | フッターインジケータナビゲーション（タスク、チーム、diff、Artifacts）         |
| `MessageSelector` | 巻き戻しと要約ダイアログのメッセージ選択                              |
| `DiffDialog`      | Diff ビューアナビゲーション                                  |
| `DiffPanel`       | [diff パネル](/docs/ja/interactive-mode#diff-panel)が開いている |
| `ModelPicker`     | モデルピッカー努力レベル                                      |
| `EffortSlider`    | `/effort` で開かれた努力スライダー                            |
| `Select`          | 汎用選択/リストコンポーネント                                   |
| `Plugin`          | プラグインダイアログ（参照、発見、管理）                              |
| `Agents`          | [エージェントビュー](/docs/ja/agent-view)（`claude agents`）      |
| `Scroll`          | 会話スクロールとフルスクリーンモードでのテキスト選択                        |

v2.1.205 より前では、`Doctor` コンテキストと `/doctor` 診断スクリーン用の `doctor:fix` アクションが存在していました。

<h2 id="available-actions">
  利用可能なアクション
</h2>

アクションは `namespace:action` 形式に従います。例えば、メッセージを送信する `chat:submit` や、タスクリストを表示する `app:toggleTodos` などです。各コンテキストには特定のアクションが利用可能です。

<h3 id="app-actions">
  アプリケーションアクション
</h3>

`Global` コンテキストで利用可能なアクション：

| アクション                  | デフォルト    | 説明                                                                           |
| :--------------------- | :------- | :--------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C   | 現在の操作をキャンセル                                                                  |
| `app:exit`             | Ctrl+D   | Claude Code を終了します。800ms 以内に 2 回押して確認                                        |
| `app:redraw`           | （バインドなし） | ターミナルの再描画を強制                                                                 |
| `app:toggleTodos`      | Ctrl+T   | Claude のやることリストの表示を切り替えます。これは [`/tasks`](/docs/ja/commands) バックグラウンドタスクビューではありません |
| `app:toggleTranscript` | Ctrl+O   | 詳細トランスクリプトの表示を切り替え                                                           |

<h3 id="history-actions">
  履歴アクション
</h3>

コマンド履歴をナビゲートするためのアクション：

| アクション              | デフォルト  | 説明      |
| :----------------- | :----- | :------ |
| `history:search`   | Ctrl+R | 履歴検索を開く |
| `history:previous` | Up     | 前の履歴項目  |
| `history:next`     | Down   | 次の履歴項目  |

<h3 id="chat-actions">
  チャットアクション
</h3>

`Chat` コンテキストで利用可能なアクション：

| アクション                 | デフォルト                          | 説明                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------------- | :----------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                         | 現在の入力をキャンセル                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:clearInput`     | Ctrl+L                         | 入力と会話を保持したまま、フルスクリーン再描画を強制                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:clearScreen`    | Cmd+K                          | `chat:clearInput` と同じです。iTerm2 と Terminal.app で Cmd+K がどのように動作するかについては、[会話をクリア](/docs/ja/fullscreen#clear-the-conversation) を参照してください                                                                                                                                                                                                                                                                                               |
| `chat:killAgents`     | Ctrl+X Ctrl+K                  | このセッション内で実行中のすべての [バックグラウンドサブエージェント](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) を停止し、残りのセッションの [アーティファクト自動返信](/docs/ja/artifacts#let-claude-reply-to-comments-on-its-own) をオフにします                                                                                                                                                                                                                                    |
| `chat:cycleMode`      | Shift+Tab\*                    | 権限モードをサイクル                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:modelPicker`    | Meta+P                         | モデルピッカーを開く                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:fastMode`       | Meta+O                         | 高速モードを切り替え                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:thinkingToggle` | Meta+T                         | 拡張思考を切り替え                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `chat:submit`         | Enter                          | メッセージを送信                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `chat:queueSubmit`    | Ctrl+X Enter                   | メッセージを送信し、順番を待つようにマーク：Claude が作業中の場合、Claude Code は [それをキューに入れ](/docs/ja/interactive-mode#queue-messages-while-claude-works)、ターンを中断しません。`chat:submit` とは異なり、オートコンプリート候補がハイライトされている場合でも下書きを送信します。v2.1.247 以降が必要                                                                                                                                                                                                                       |
| `chat:sendNow`        | Ctrl+Enter、Ctrl+X Ctrl+S       | [キューに入れたメッセージ](/docs/ja/interactive-mode#queue-messages-while-claude-works) と下書きをすぐに送信します。Claude Code が [キューに入れたものを送信する](/docs/ja/interactive-mode#when-claude-code-sends-what-you-queued) 場合、Claude が作業中のターンに何が起こるかについて説明します。何も実行されていない場合は下書きを送信し、[シェルモード](/docs/ja/interactive-mode#shell-mode-with-prefix) ではコマンドをキューに入れるだけです。拡張キーを報告しないターミナルは `Ctrl+Enter` をプレーン `Enter` として配信するため、`Ctrl+X Ctrl+S` はどのターミナルでも機能するバインディングです。v2.1.275 以降が必要 |
| `chat:newline`        | Ctrl+J                         | 送信せずに改行を挿入                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:undo`           | Ctrl+\_、Ctrl+Shift+-           | 最後のアクションを元に戻す                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:externalEditor` | Ctrl+G、Ctrl+X Ctrl+E           | 外部エディタで開きます。[エージェントビューディスパッチ入力](/docs/ja/agent-view#keyboard-shortcuts) もこのアクションの単一キーストロークバインディングに従います                                                                                                                                                                                                                                                                                                                              |
| `chat:stash`          | Ctrl+S                         | 現在のプロンプトを保存                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:imagePaste`     | Ctrl+V（Windows と WSL では Alt+V） | クリップボードから画像を貼り付けます。WSL では、両方のショートカットがデフォルトでバインドされています                                                                                                                                                                                                                                                                                                                                                                          |

\*VT モードなし Windows（Node \<24.2.0/\<22.17.0、Bun \<1.2.23）では、デフォルトは Meta+M です。

<h3 id="autocomplete-actions">
  オートコンプリートアクション
</h3>

`Autocomplete` コンテキストで利用可能なアクション：

| アクション                   | デフォルト  | 説明       |
| :---------------------- | :----- | :------- |
| `autocomplete:accept`   | Tab    | 候補を受け入れ  |
| `autocomplete:dismiss`  | Escape | メニューを閉じる |
| `autocomplete:previous` | Up     | 前の候補     |
| `autocomplete:next`     | Down   | 次の候補     |

<h3 id="confirmation-actions">
  確認アクション
</h3>

`Confirmation` コンテキストで利用可能なアクション：

| アクション                   | デフォルト       | 説明                                                                                                                                                                                           |
| :---------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Enter       | アクションを確認                                                                                                                                                                                     |
| `confirm:no`            | Escape      | アクションを拒否                                                                                                                                                                                     |
| `confirm:previous`      | Up          | 前のオプション                                                                                                                                                                                      |
| `confirm:next`          | Down        | 次のオプション                                                                                                                                                                                      |
| `confirm:nextField`     | Tab         | 次のフィールド                                                                                                                                                                                      |
| `confirm:previousField` | （バインドなし）    | 前のフィールド                                                                                                                                                                                      |
| `confirm:toggle`        | Space       | 選択を切り替え                                                                                                                                                                                      |
| `confirm:cycleMode`     | Shift+Tab\* | 権限モードをサイクルします。ファイル権限プロンプトでは、開いている [コメントフィールド](/docs/ja/permissions#add-a-comment-when-you-answer-a-permission-prompt) を閉じます。フィールドが開いていない場合は、プロンプトがそのオプションを提供するときに、セッションの残りの間アクションを許可するオプションを選択します |

\*VT モードなし Windows（Node \<24.2.0/\<22.17.0、Bun \<1.2.23）では、デフォルトは Meta+M です。

v2.1.257 より前では、`confirm:toggleExplanation` アクション（デフォルトでは Ctrl+E にバインド）が Bash と PowerShell 権限プロンプトでモデル生成の説明を表示していました。

ダイアログは、はい/いいえの質問をしていない場合でも、`confirm:yes` と `confirm:no` を使用して受け入れとキャンセルを行います。このコンテキストで `y` や `n` などのベアレターをバインドすると、そのレターをキーとして表示しないダイアログにも作用します。`y` と `n` をキーとして表示するダイアログはそれらのレターを自分で読み取り、バインディングは不要です。

この例は `y` を `confirm:yes` に、`n` を `confirm:no` にバインドします：

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

これらのバインディングでは、[テキストフィールド](#text-fields) がフォーカスされている間、`y` と `n` は引き続き文字として入力されます。

v2.1.280 より前では、`y` もデフォルトで `confirm:yes` にバインドされ、`n` は `confirm:no` にバインドされていました。v2.1.280 より前に `/keybindings` で `keybindings.json` を作成した場合、ファイルは両方のバインディングをリストし、それらの 2 行を削除するまで有効なままです。

<h3 id="permission-actions">
  権限アクション
</h3>

権限ダイアログの `Confirmation` コンテキストで利用可能なアクション：

| アクション                    | デフォルト    | 説明                                                                         |
| :----------------------- | :------- | :------------------------------------------------------------------------- |
| `permission:toggleDebug` | （バインドなし） | 権限デバッグ情報を切り替えます。v2.1.146 で `app:exit` をシャドウしていたため、Ctrl+D の以前のデフォルトが削除されました |

<h3 id="transcript-actions">
  トランスクリプトアクション
</h3>

`Transcript` コンテキストで利用可能なアクション：

| アクション                      | デフォルト           | 説明                |
| :------------------------- | :-------------- | :---------------- |
| `transcript:toggleShowAll` | Ctrl+E          | すべてのコンテンツの表示を切り替え |
| `transcript:exit`          | q、Ctrl+C、Escape | トランスクリプトビューを終了    |

`transcript:toggleShowAll` はクラシックレンダラーでのみ適用されます。[フルスクリーンレンダリング](/docs/ja/fullscreen) では、トランスクリプトビューアは show-all トグルを提供しません。

<h3 id="history-search-actions">
  履歴検索アクション
</h3>

`HistorySearch` コンテキストで利用可能なアクション：

| アクション                      | デフォルト      | 説明                         |
| :------------------------- | :--------- | :------------------------- |
| `historySearch:next`       | Ctrl+R     | 次のマッチ                      |
| `historySearch:accept`     | Escape、Tab | 選択を受け入れ                    |
| `historySearch:cancel`     | Ctrl+C     | 検索をキャンセル                   |
| `historySearch:execute`    | Enter      | 選択したコマンドを実行                |
| `historySearch:cycleScope` | Ctrl+S     | スコープをサイクル：セッション、プロジェクト、すべて |

`historySearch:next`、`historySearch:accept`、`historySearch:cancel`、`historySearch:execute` のデフォルトはクラシックレンダラーのインライン履歴検索に適用され、常にすべてのプロジェクトからプロンプトを検索します。`historySearch:cycleScope` は [フルスクリーンレンダリング](/docs/ja/fullscreen) でのみ有効です。ここで Ctrl+R は検索ダイアログを開き、Ctrl+S はそのスコープをサイクルします。ダイアログの他のキーは固定されており、リバインドできません：`Enter` または `Tab` はハイライトされたマッチをプロンプト入力に配置し、`Esc` はキャンセルします。

<h3 id="task-actions">
  タスクアクション
</h3>

`Task` コンテキストで利用可能なアクション：

| アクション             | デフォルト                | 説明                                                             |
| :---------------- | :------------------- | :------------------------------------------------------------- |
| `task:background` | Ctrl+B、Ctrl+X Ctrl+B | 現在のタスクをバックグラウンドに移動します。Ctrl+X Ctrl+B コードは tmux プレフィックスの競合を回避します |

<h3 id="theme-actions">
  テーマアクション
</h3>

`ThemePicker` コンテキストで利用可能なアクション：

| アクション                            | デフォルト  | 説明           |
| :------------------------------- | :----- | :----------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T | 構文ハイライトを切り替え |

<h3 id="help-actions">
  ヘルプアクション
</h3>

`Help` コンテキストで利用可能なアクション：

| アクション          | デフォルト  | 説明          |
| :------------- | :----- | :---------- |
| `help:dismiss` | Escape | ヘルプメニューを閉じる |

<h3 id="tabs-actions">
  タブアクション
</h3>

`Tabs` コンテキストで利用可能なアクション：

| アクション           | デフォルト          | 説明   |
| :-------------- | :------------- | :--- |
| `tabs:next`     | Tab、Right      | 次のタブ |
| `tabs:previous` | Shift+Tab、Left | 前のタブ |

<h3 id="attachments-actions">
  添付ファイルアクション
</h3>

`Attachments` コンテキストで利用可能なアクション：

| アクション                  | デフォルト            | 説明               |
| :--------------------- | :--------------- | :--------------- |
| `attachments:next`     | Right            | 次の添付ファイル         |
| `attachments:previous` | Left             | 前の添付ファイル         |
| `attachments:remove`   | Backspace、Delete | 選択した添付ファイルを削除    |
| `attachments:exit`     | Down、Escape      | 添付ファイルナビゲーションを終了 |

<h3 id="footer-actions">
  フッターアクション
</h3>

`Footer` コンテキストで利用可能なアクション：

| アクション                   | デフォルト            | 説明                                                                                                               |
| :---------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------- |
| `footer:next`           | Right            | 次のフッター項目                                                                                                         |
| `footer:previous`       | Left             | 前のフッター項目                                                                                                         |
| `footer:up`             | Up               | フッター内を上にナビゲート（上部で選択解除）                                                                                           |
| `footer:down`           | Down             | フッター内を下にナビゲート                                                                                                    |
| `footer:openSelected`   | Enter            | 選択したフッター項目を開く                                                                                                    |
| `footer:clearSelection` | Escape           | フッター選択をクリア                                                                                                       |
| `footer:dismiss`        | Backspace、Delete | フッターから選択した [アーティファクト](/docs/ja/artifacts) リンクを削除します。公開されたアーティファクト自体は影響を受けません。他のフッター行では、これらのキーは効果がありません。v2.1.217 以降が必要 |

フッター項目が選択されている場合（プロンプトの下のエージェントパネルの行など）、`Chat` コンテキストで `Enter` を `chat:queueSubmit` または `chat:newline` に再バインドしても、`Enter` はそれを開きます。

`Chat` コンテキストの `Shift+Tab` の `chat:cycleMode` など、`Footer` コンテキストがバインドしないキーのバインディングは、項目が選択されている間も機能します。

<h3 id="message-selector-actions">
  メッセージセレクターアクション
</h3>

`MessageSelector` コンテキストで利用可能なアクション：

| アクション                    | デフォルト                                  | 説明        |
| :----------------------- | :------------------------------------- | :-------- |
| `messageSelector:up`     | Up、K、Ctrl+P                            | リスト内を上に移動 |
| `messageSelector:down`   | Down、J、Ctrl+N                          | リスト内を下に移動 |
| `messageSelector:top`    | Ctrl+Up、Shift+Up、Meta+Up、Shift+K       | 上部にジャンプ   |
| `messageSelector:bottom` | Ctrl+Down、Shift+Down、Meta+Down、Shift+J | 下部にジャンプ   |
| `messageSelector:select` | Enter                                  | メッセージを選択  |

<h3 id="diff-actions">
  Diff アクション
</h3>

`DiffDialog` コンテキストで利用可能なアクション：

| アクション                 | デフォルト    | 説明                                                                                                |
| :-------------------- | :------- | :------------------------------------------------------------------------------------------------ |
| `diff:dismiss`        | Escape   | Diff ビューアを閉じます。詳細ビューからは、ファイルリストに戻ります                                                              |
| `diff:previousSource` | Left     | 前の Diff ソース                                                                                       |
| `diff:nextSource`     | Right    | 次の Diff ソース                                                                                       |
| `diff:previousFile`   | Up、K     | ファイルリスト内の前のファイル。詳細ビューで 1 行上にスクロール                                                                 |
| `diff:nextFile`       | Down、J   | ファイルリスト内の次のファイル。詳細ビューで 1 行下にスクロール                                                                 |
| `diff:viewDetails`    | Enter    | Diff の詳細を表示                                                                                       |
| `diff:back`           | （バインドなし） | Diff ビューアで戻ります。Escape は `diff:dismiss` を介して戻るアクションを実行します。詳細ビューの Left の以前のデフォルトは v2.1.203 で削除されました |

Diff 詳細ビューは、ページャースタイルのキーを標準 [スクロールアクション](#scroll-actions) にバインドします。これらのバインディングは `DiffDialog` コンテキストの一部であり、詳細ビューでのみ適用されます。[スクロールアクション](#scroll-actions) の下にリストされている `Scroll` コンテキストのデフォルトは変更されません。

| アクション                 | デフォルト         | 説明               |
| :-------------------- | :------------ | :--------------- |
| `scroll:pageUp`       | PageUp        | ビューポートの半分上にスクロール |
| `scroll:pageDown`     | PageDown      | ビューポートの半分下にスクロール |
| `scroll:fullPageUp`   | Shift+Space、B | ビューポート全体を上にスクロール |
| `scroll:fullPageDown` | Space         | ビューポート全体を下にスクロール |
| `scroll:top`          | G、Home        | 上部にジャンプ          |
| `scroll:bottom`       | Shift+G、End   | 下部にジャンプ          |

<h3 id="diff-panel-actions">
  Diff パネルアクション
</h3>

`/diff` がフルスクリーンレンダリングで開く [Diff パネル](/docs/ja/interactive-mode#diff-panel) のアクション。`app:cycleDiffBase` は `DiffPanel` コンテキストにあり、パネルが開いている間はアクティブです。その他は `Global` です。パネルには Claude Code v2.1.260 以降が必要です。

| アクション                       | デフォルト               | 説明                                     |
| :-------------------------- | :------------------ | :------------------------------------- |
| `app:toggleReplTab`         | （バインドなし）            | Diff パネルを開くか閉じます。`/diff` を実行するのと同じです   |
| `app:cycleDiffBase`         | Ctrl+X B            | パネルの比較ベースをサイクル：このセッション、コミットされていない、ブランチ |
| `app:diffFileListUp`        | Ctrl+Up、Meta+Up     | パネルのファイルリストがオーバーフローするときに上にスクロール        |
| `app:diffFileListDown`      | Ctrl+Down、Meta+Down | パネルのファイルリストがオーバーフローするときに下にスクロール        |
| `app:toggleDiffNoiseFilter` | （バインドなし）            | パネルのテストファイルと生成ファイルを表示または非表示            |
| `app:toggleDiffPreSession`  | （バインドなし）            | このセッション前の変更を展開または折りたたむ                 |

<h3 id="model-picker-actions">
  モデルピッカーアクション
</h3>

`ModelPicker` コンテキストで利用可能なアクション：

| アクション                         | デフォルト | 説明                       |
| :---------------------------- | :---- | :----------------------- |
| `modelPicker:decreaseEffort`  | Left  | 努力レベルを低下                 |
| `modelPicker:increaseEffort`  | Right | 努力レベルを増加                 |
| `modelPicker:thisSessionOnly` | s     | ハイライトされたモデルをこのセッションのみに適用 |

<h3 id="effort-slider-actions">
  努力スライダーアクション
</h3>

`EffortSlider` コンテキストで利用可能なアクション。引数なしで `/effort` を実行するときに開くスライダーです。スライダーの Left、Right、Enter、Escape キーは再バインドできません。

| アクション                          | デフォルト | 説明                                                                                     |
| :----------------------------- | :---- | :------------------------------------------------------------------------------------- |
| `effortSlider:thisSessionOnly` | s     | フォーカスされた [努力レベル](/docs/ja/model-config#adjust-effort-level) をこのセッションのみに適用します。v2.1.257 以降が必要 |

<h3 id="select-actions">
  選択アクション
</h3>

`Select` コンテキストで利用可能なアクション：

| アクション             | デフォルト         | 説明               |
| :---------------- | :------------ | :--------------- |
| `select:next`     | Down、J、Ctrl+N | 次のオプション          |
| `select:previous` | Up、K、Ctrl+P   | 前のオプション          |
| `select:pageUp`   | PageUp        | オプションの 1 ページ上に移動 |
| `select:pageDown` | PageDown      | オプションの 1 ページ下に移動 |
| `select:first`    | Home          | 最初のオプション         |
| `select:last`     | End           | 最後のオプション         |
| `select:accept`   | Enter         | 選択を受け入れ          |
| `select:cancel`   | Escape        | 選択をキャンセル         |

Claude Code は `/skills` メニューで `select:pageUp`、`select:pageDown`、`select:first`、`select:last` バインディングを適用します。`/model` ピッカーなどのほとんどの他のリストでは、`select:first` と `select:last` バインディングが適用されます。PageUp と PageDown は、バインディングに関係なく、これらのリストのオプションをページングします。

v2.1.280 より前では、これらの他のリストは Home、End、および `select:first` と `select:last` バインディングを無視していました。

<h3 id="plugin-actions">
  プラグインアクション
</h3>

`Plugin` コンテキストで利用可能なアクション：

| アクション             | デフォルト | 説明                                              |
| :---------------- | :---- | :---------------------------------------------- |
| `plugin:toggle`   | Space | プラグイン選択を切り替え                                    |
| `plugin:install`  | I     | 選択したプラグインをインストール                                |
| `plugin:favorite` | F     | 選択したプラグインをお気に入りにして、インストール済みタブの上部付近でソートされるようにします |

<h3 id="settings-actions">
  設定アクション
</h3>

`Settings` コンテキストで利用可能なアクション。`select:accept` と `confirm:no` アクションは [選択](#select-actions) と [確認](#confirmation-actions) コンテキストから再利用され、設定固有の動作があります：変更は各設定を変更するとすぐに適用されるため、Escape はパネルを閉じて変更を保存します。拒否するのではなく。

| アクション             | デフォルト       | 説明                       |
| :---------------- | :---------- | :----------------------- |
| `settings:search` | /           | 検索モードに入る                 |
| `settings:retry`  | R           | エラー時に使用データの読み込みを再試行      |
| `select:accept`   | Enter、Space | 選択した設定を変更するか、そのサブメニューを開く |
| `confirm:no`      | Escape      | パネルを閉じます。変更は既に保存されています   |

<h3 id="agents-actions">
  エージェントアクション
</h3>

`claude agents` で開く [エージェントビュー](/docs/ja/agent-view) で適用される `Agents` コンテキストで利用可能なアクション。v2.1.257 以降が必要です。

| アクション               | デフォルト  | 説明                                                               |
| :------------------ | :----- | :--------------------------------------------------------------- |
| `agents:switchView` | Ctrl+S | [セッショングループ化](/docs/ja/agent-view#organize-the-list) を状態とディレクトリの間で切り替え |
| `agents:togglePin`  | Ctrl+T | 選択したセッションを [ピン留めまたはピン留め解除](/docs/ja/agent-view#organize-the-list)     |

エージェントビューが開いている間、Claude Code は `Agents` コンテキストがバインドするキーに対して `Agents` バインディングを使用し、同じキーの `Chat` または `Global` バインディングを無視します。例えば、エージェントビューで Ctrl+S を押すと、デフォルトの `chat:stash` をトリガーするのではなく、セッショングループ化を切り替えます。

ディスパッチ入力の外部エディタショートカットは `Agents` アクションではありません。エージェントビューは `Chat` コンテキストの `chat:externalEditor` バインディング（デフォルトでは Ctrl+G）に従います。

バインディングはエージェントビューで単一キーストロークで発火するため、`chat:externalEditor` にバインドされた Ctrl+X Ctrl+E コードはエディタをそこで開きません。

<h3 id="voice-actions">
  音声アクション
</h3>

[音声ディクテーション](/docs/ja/voice-dictation) が有効な場合の `Chat` コンテキストで利用可能なアクション：

| アクション              | デフォルト | 説明                                    |
| :----------------- | :---- | :------------------------------------ |
| `voice:pushToTalk` | Space | プロンプトをディクテート。`/voice` モードに応じて保持またはタップ |

<h3 id="scroll-actions">
  スクロールアクション
</h3>

[フルスクリーンレンダリング](/docs/ja/fullscreen) が有効な場合の `Scroll` コンテキストで利用可能なアクション：

| アクション                       | デフォルト                | 説明                                                            |
| :-------------------------- | :------------------- | :------------------------------------------------------------ |
| `scroll:lineUp`             | `wheelup`            | 1 行上にスクロール。マウスホイールスクロールはこのアクションをトリガー                          |
| `scroll:lineDown`           | `wheeldown`          | 1 行下にスクロール。マウスホイールスクロールはこのアクションをトリガー                          |
| `scroll:pageUp`             | PageUp               | ビューポート高さの半分上にスクロール                                            |
| `scroll:pageDown`           | PageDown             | ビューポート高さの半分下にスクロール                                            |
| `scroll:top`                | Ctrl+Home            | 会話の開始にジャンプ                                                    |
| `scroll:bottom`             | Ctrl+End             | 最新のメッセージにジャンプして自動フォローを再度有効化                                   |
| `scroll:halfPageUp`         | （バインドなし）             | ビューポート高さの半分上にスクロール。`scroll:pageUp` と同じ動作で、vi スタイルの再バインド用に提供   |
| `scroll:halfPageDown`       | （バインドなし）             | ビューポート高さの半分下にスクロール。`scroll:pageDown` と同じ動作で、vi スタイルの再バインド用に提供 |
| `scroll:fullPageUp`         | （バインドなし）             | ビューポート高さ全体を上にスクロール                                            |
| `scroll:fullPageDown`       | （バインドなし）             | ビューポート高さ全体を下にスクロール                                            |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | 選択したテキストをクリップボードにコピー                                          |
| `selection:clear`           | （バインドなし）             | アクティブなテキスト選択をクリア。v2.1.234 以降が必要                               |
| `selection:extendLeft`      | Shift+Left           | アクティブな選択を 1 列左に拡張                                             |
| `selection:extendRight`     | Shift+Right          | アクティブな選択を 1 列右に拡張                                             |
| `selection:extendUp`        | Shift+Up             | アクティブな選択を 1 行上に拡張。選択が上端に達するとビューポートをスクロール                      |
| `selection:extendDown`      | Shift+Down           | アクティブな選択を 1 行下に拡張。選択が下端に達するとビューポートをスクロール                      |
| `selection:extendLineStart` | Shift+Home           | アクティブな選択を行の開始まで拡張                                             |
| `selection:extendLineEnd`   | Shift+End            | アクティブな選択を行の終了まで拡張                                             |

<h2 id="keystroke-syntax">
  キーストロークシンタックス
</h2>

<h3 id="modifiers">
  モディファイア
</h3>

`+` セパレータでモディファイアキーを使用します。

* `ctrl` または `control` - Control キー
* `shift` - Shift キー
* `alt`、`opt`、`option`、または `meta` - Windows と Linux の Alt キー、macOS の Option キー
* `cmd`、`command`、`super`、または `win` - macOS の Command キー、Windows の Windows キー、Linux の Super キー

`cmd` グループは Super モディファイアを報告するターミナル（Kitty キーボードプロトコルまたは xterm の `modifyOtherKeys` モードをサポートするターミナルなど）でのみ検出されます。ほとんどのターミナルはこれを送信しないため、すべての場所で機能するバインディングには `ctrl` または `meta` を使用してください。

例えば：

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          macOS の Option + P、その他の場所では Alt + P
ctrl+shift+c    複数のモディファイア
```

<h3 id="uppercase-letters">
  大文字
</h3>

Claude Code はキー名を大文字と小文字を区別せずに解析するため、`K` は `k` と同じバインディングであり、`ctrl+K` は `ctrl+k` と同じです。Shift とレターをバインドするには、`shift+k` と記述します。

<h3 id="non-us-keyboard-layouts">
  非 US キーボードレイアウト
</h3>

Ctrl ショートカットのキー名は、アクティブなキーボードレイアウトが他の文字を入力する場合でも、ラテン文字として記述してください。

Claude Code がキーボードで押したキーをバインディングにマッチさせる方法は、レイアウトの種類によって異なります。

* キリル文字などの非ラテンレイアウトの場合、Claude Code はターミナルが Kitty キーボードプロトコルを使用し、その位置を報告する場合、キーの US レイアウト位置で Ctrl ショートカットをマッチさせます。そのようなターミナルでロシア語レイアウトがアクティブな場合、Ctrl キーと物理的な W キーを押すと `ctrl+w` がトリガーされます。位置を報告しないターミナルでは、Claude Code はターミナルがキープレスに対して送信するものをマッチさせます。ASCII 制御コードは Latin ショートカットをトリガーし、キリル文字として到着するキープレスはバインディングにマッチしません。
* AZERTY などのラテン文字を並べ替えるレイアウトの場合、Claude Code はキーが入力する文字をマッチさせるため、Ctrl キーと A というラベルのキーを押すと `ctrl+a` がトリガーされます。

v2.1.247 より前では、Ghostty、Kitty、WezTerm、iTerm2 などの Kitty キーボードプロトコルを使用するターミナルで、非ラテンレイアウトの下で Ctrl ショートカットを押してもそのバインディングはトリガーされませんでした。

<h3 id="chords">
  コード
</h3>

コードはスペースで区切られたキーストロークのシーケンスです。

```text theme={null}
ctrl+k ctrl+s   Ctrl+K を押して、リリースしてから Ctrl+S
```

各キーストロークは、その前のキーストロークから 3 秒以内に押してください。それより長く待つと、Claude Code はコードをキャンセルし、そのことを示す簡潔な通知を表示します。

<h3 id="special-keys">
  特殊キー
</h3>

* `escape` または `esc` - Escape キー
* `enter` または `return` - Enter キー
* `tab` - Tab キー
* `space` - スペースバー
* `up`、`down`、`left`、`right` - 矢印キー
* `pageup`、`pagedown` - Page Up キーと Page Down キー
* `home`、`end` - Home キーと End キー
* `backspace`、`delete` - Delete キー
* `wheelup`、`wheeldown` - マウスホイールスクロールイベント

<h2 id="unbind-default-shortcuts">
  デフォルトショートカットをアンバインド
</h2>

アクションを `null` に設定して、デフォルトショートカットをアンバインドします。

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

これはコード バインディングでも機能します。プレフィックスを共有するすべてのコードをアンバインドすると、そのプレフィックスを単一キー バインディングとして使用できるようになります。コード バインディングは任意のアクティブなコンテキストに存在し、そのプレフィックスを予約したままにするため、それを定義するコンテキストで各コードをアンバインドする必要があります。

Claude Code は `ctrl+x` プレフィックスに以下のデフォルトコードをバインドします。`Chat` では `ctrl+x ctrl+k`、`ctrl+x ctrl+e`、`ctrl+x enter`、`ctrl+x ctrl+a`、`ctrl+x ctrl+s`、`ctrl+x tab`、`Task` では `ctrl+x ctrl+b`、`DiffPanel` では `ctrl+x b` です。`ctrl+x enter` コードは v2.1.247 以降が必要で、`ctrl+x b`、`ctrl+x ctrl+a`、`ctrl+x tab` は v2.1.260 以降が必要で、`ctrl+x ctrl+s` は v2.1.275 以降が必要です。

`ctrl+x` 自体を単一キー バインディングとして再利用するには、すべてをアンバインドします。

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

プレフィックス上の一部のコードをアンバインドしても、すべてをアンバインドしない場合、プレフィックスを押すと残りのバインディングのコード待機モードに入ります。

<h2 id="reserved-shortcuts">
  予約済みショートカット
</h2>

これらのショートカットは再バインドできません。

| ショートカット   | 理由                                                                                                                                                                                                  |
| :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | ハードコードされた割り込み/キャンセル                                                                                                                                                                                 |
| Ctrl+D    | ハードコードされた終了                                                                                                                                                                                         |
| Ctrl+M    | Claude Code は常に Enter として受け取ります                                                                                                                                                                     |
| Ctrl+\[   | Claude Code は常に Escape として受け取ります。Kitty キーボードプロトコルを使用するターミナルでは、v2.1.242 以降が必要です                                                                                                                      |
| Ctrl+I    | Claude Code は常に Tab として受け取ります                                                                                                                                                                       |
| Ctrl+H    | ASCII バックスペースバイトを送信します。[Claude Code が Windows で読み込む方法](/docs/ja/terminal-config#fix-backspace-deleting-a-whole-word-on-windows)は、ターミナルと [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/ja/env-vars) 環境変数によって異なります |
| Caps Lock | ターミナルアプリケーションに配信されない                                                                                                                                                                                |

<h2 id="terminal-conflicts">
  ターミナルの競合
</h2>

一部のショートカットはターミナルマルチプレクサと競合する可能性があります。

| ショートカット | 競合                     |
| :------ | :--------------------- |
| Ctrl+B  | tmux プレフィックス（2 回押して送信） |
| Ctrl+A  | GNU screen プレフィックス     |
| Ctrl+Z  | Unix プロセス一時停止（SIGTSTP） |

<h2 id="text-fields">
  テキストフィールド
</h2>

ベアレター、数字、またはスペースをバインドした場合でも、ダイアログまたはパネル内のテキストフィールドでそのキャラクターを入力できます。このようなフィールドの 1 つは、Claude が質問する「Other」の回答です。フィールドがフォーカスを持っている間、Ctrl、Alt、または Cmd なしで押したプリント可能なキーはフィールドに送られ、Claude Code はそれをバインディングと照合しません。

これらのキーはフィールドがフォーカスを持っている間でもバインディングを実行します。

* Enter、Escape、Tab、矢印キーなど、キャラクターを入力しないキー
* Ctrl、Alt、または Cmd で押されたキー
* 既に進行中の[コード](#chords)の 2 番目のキーストローク

メインプロンプトでは、Claude Code はすべてのキーを `Chat` などのアクティブなコンテキストと照合し、バインディングがそれを取らない場合にのみキーを入力します。

<h2 id="vim-mode-interaction">
  Vim モードの相互作用
</h2>

Vim モードが `/config` → エディタモードで有効な場合、キーバインディングと Vim モードは独立して動作します。

* **Vim モード** はテキスト入力レベルで入力を処理します（カーソル移動、モード、モーション）
* **キーバインディング** はコンポーネントレベルでアクションを処理します（todos を切り替え、送信など）
* Vim モードの Escape キーは INSERT から NORMAL モードに切り替わります。`chat:cancel` をトリガーしません
* ほとんどの Ctrl+key ショートカットは Vim モードを通過してキーバインディングシステムに渡されます
* Vim キーはキーバインディングファイルを通じて再マップできません。`jj` を Escape にマップするような 2 キーの INSERT モードシーケンスをマップするには、[`vimInsertModeRemaps`](/docs/ja/interactive-mode#remap-insert-mode-key-sequences) 設定を使用してください
* Vim NORMAL モードでは、`?` はヘルプメニューを表示します（Vim の動作）
* Vim NORMAL モードでは、`/` は履歴検索を開きます。標準モードの Ctrl+R と同じです

<h2 id="validation">
  検証
</h2>

Claude Code はキーバインディングを検証し、以下の警告を表示します。

* 解析エラー（無効な JSON または構造）
* 無効なコンテキスト名
* 無効なアクション値（アクションが文字列または `null` ではない場合など）
* 不明なアクション名（登録されたアクションのタイプミスなど）。Claude Code はバインディングをスキップし、そのキーのデフォルトバインディングを有効に保ちます。v2.1.246 より前では、不明なアクション名を持つバインディングはそのキーを静かに無効化していました
* 予約済みショートカットの競合
* 同じコンテキスト内の重複バインディング

Claude Code はファイルが読み込まれるときに警告を報告し、各警告をデバッグログに書き込みます。Claude Code を [`--debug`](/docs/ja/cli-reference#cli-flags) で起動して、詳細を確認してください。
