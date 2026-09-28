> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# ステータスラインをカスタマイズする

> Claude Code でコンテキストウィンドウの使用状況、コスト、git ステータスを監視するカスタムステータスバーを設定します

ステータスラインは Claude Code の下部にあるカスタマイズ可能なバーで、設定したシェルスクリプトを実行します。stdin 経由で JSON セッションデータを受け取り、スクリプトが出力したものを表示し、コンテキスト使用状況、コスト、git ステータス、またはその他の追跡したい情報を一目で確認できる永続的なビューを提供します。

ステータスラインは以下の場合に便利です：

* 作業中にコンテキストウィンドウの使用状況を監視したい
* セッションコストを追跡する必要がある
* 複数のセッション間で作業し、それらを区別する必要がある
* git ブランチとステータスを常に表示したい

ステータスラインは組み込みのフッターバッジの上にある独自の行にレンダリングされ、それらを置き換えません。カスタムステータスラインが設定されている場合、Claude Code はフッターのキーボードヒントのほとんどを表示しなくなります。これには `esc to interrupt`、`? for shortcuts` フォールバック、および `hold space to speak` [音声入力](/docs/ja/voice-dictation) ヒントが含まれます。会話内に ID が表示されたときにフッターにクリック可能なリンクバッジを追加する場合は、スクリプトを記述せずに [`footerLinksRegexes`](/docs/ja/settings-reference#footerlinksregexes) を設定してください。

以下は、最初の行に git 情報を表示し、2 番目の行にカラーコード化されたコンテキストバーを表示する [複数行ステータスライン](#display-multiple-lines) の例です。

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="最初の行にモデル名、ディレクトリ、git ブランチを表示し、2 番目の行にコンテキスト使用状況プログレスバー、コスト、期間を表示する複数行ステータスライン" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

このページでは、[基本的なステータスラインの設定](#set-up-a-status-line) について説明し、Claude Code からスクリプトへの [データフロー](#how-status-lines-work) について説明し、[表示できるすべてのフィールド](#available-data) をリストアップし、git ステータス、コスト追跡、プログレスバーなどの一般的なパターンの [すぐに使える例](#examples) を提供します。

<h2 id="set-up-a-status-line">
  ステータスラインを設定する
</h2>

[`/statusline` コマンド](#use-the-%2Fstatusline-command) を使用して Claude Code にスクリプトを生成させるか、[手動でスクリプトを作成](#manually-configure-a-status-line) して設定に追加します。

<h3 id="use-the-/statusline-command">
  /statusline コマンドを使用する
</h3>

`/statusline` コマンドは、表示したい内容を説明する自然言語の指示を受け入れます。Claude Code は `~/.claude/` にスクリプトファイルを生成し、設定を自動的に更新します：

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

セットアップ中に Claude Code が権限を求める場合は、ファイル編集プロンプトを承認してください。

<h3 id="manually-configure-a-status-line">
  ステータスラインを手動で設定する
</h3>

ユーザー設定（`~/.claude/settings.json`、`~` はホームディレクトリ）または [プロジェクト設定](/docs/ja/settings#where-settings-live) に `statusLine` フィールドを追加します。`type` を `"command"` に設定し、`command` をスクリプトパスまたはインラインシェルコマンドに指定します。スクリプト作成の完全なチュートリアルについては、[ステータスラインをステップバイステップで構築する](#build-a-status-line-step-by-step) を参照してください。

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

`command` フィールドはシェルで実行されるため、スクリプトファイルの代わりにインラインコマンドを使用することもできます。この例では `jq` を使用して JSON 入力を解析し、モデル名とコンテキスト割合を表示します：

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

オプションの `padding` フィールドは、ステータスラインコンテンツに追加の水平スペース（文字単位）を追加します。デフォルトは `0` です。このパディングはインターフェイスの組み込みスペースに加えて追加されるため、ターミナルエッジからの絶対距離ではなく相対的なインデントを制御します。

オプションの `refreshInterval` フィールドは、[イベント駆動更新](#how-status-lines-work) に加えて、N 秒ごとにコマンドを再実行します。最小値は `1` です。ステータスラインが時計などの時間ベースのデータを表示する場合、またはメインセッションがアイドル状態の間にバックグラウンドサブエージェントが git 状態を変更する場合に設定します。イベントのみで実行する場合は設定しないままにします。

オプションの `hideVimModeIndicator` フィールドは、プロンプトの下にある組み込みの `-- INSERT --` テキストを非表示にします。スクリプトが [`vim.mode`](#available-data) 自体をレンダリングする場合は、これを `true` に設定して、モードが 2 回表示されないようにします。

<h3 id="disable-the-status-line">
  ステータスラインを無効にする
</h3>

`/statusline` を実行し、ステータスラインを削除またはクリアするよう指示します（例：`/statusline delete`、`/statusline clear`、`/statusline remove it`）。settings.json から `statusLine` フィールドを手動で削除することもできます。

<h2 id="build-a-status-line-step-by-step">
  ステータスラインをステップバイステップで構築する
</h2>

このチュートリアルでは、現在のモデル、作業ディレクトリ、コンテキストウィンドウ使用状況の割合を表示するステータスラインを手動で作成することで、`/statusline` が内部で何をセットアップするかを示します。

<Note>[`/statusline`](#use-the-%2Fstatusline-command) を実行して、表示したい内容を説明すると、これらすべてが自動的に設定されます。</Note>

これらの例では Bash スクリプトを使用しており、macOS と Linux で動作します。Windows では、[Windows 設定](#windows-configuration) で PowerShell と Git Bash の例を参照してください。

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="モデル名、ディレクトリ、コンテキスト割合を表示するステータスライン" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="JSON を読み取り、出力を出力するスクリプトを作成する">
    Claude Code は stdin 経由でスクリプトに JSON データを送信します。このスクリプトは [`jq`](https://jqlang.org/)（コマンドラインの JSON パーサーで、インストールが必要な場合があります）を使用して、モデル名、ディレクトリ、コンテキスト割合を抽出し、フォーマットされた行を出力します。

    これを `~/.claude/statusline.sh` に保存します（`~` はホームディレクトリ、macOS では `/Users/username`、Linux では `/home/username` など）：

    ```bash theme={null}
    #!/bin/bash
    # Claude Code が stdin に送信する JSON データを読み取る
    input=$(cat)

    # jq を使用してフィールドを抽出する
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # "// 0" はフィールドが null の場合のフォールバックを提供します
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # ステータスラインを出力します - ${DIR##*/} はフォルダ名のみを抽出します
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="実行可能にする">
    スクリプトを実行可能にマークして、シェルが実行できるようにします：

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="設定に追加する">
    Claude Code にスクリプトをステータスラインとして実行するよう指示します。この設定を `~/.claude/settings.json` に追加します。これは `type` を `"command"`（「このシェルコマンドを実行する」という意味）に設定し、`command` をスクリプトに指定します：

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    ステータスラインはインターフェイスの下部に表示されます。Claude Code は設定を自動的に再読み込みし、ファイルを保存するとすぐにスクリプトを実行します。
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  ステータスラインの仕組み
</h2>

Claude Code はスクリプトを実行し、stdin 経由で [JSON セッションデータ](#available-data) をパイプします。スクリプトが stdout に出力したものを Claude Code が表示します。

**更新のタイミング**

スクリプトはセッション開始時（再開時を含む）に 1 回実行されます。その後、以下の場合に再度実行されます：

* 新しいアシスタントメッセージが到着したとき
* `/compact` が完了したとき
* パーミッション権限モードが変更されたとき
* Vim モードが切り替わったとき
* `statusLine` 設定で `command` を変更したとき
* [`refreshInterval`](#manually-configure-a-status-line) タイマーが経過したとき（設定した場合）
* スクリプトが最後に受け取ったデータ内の [レート制限ウィンドウ](#rate-limit-usage) が `resets_at` 時刻に到達したとき
* スクリプトが最後に受け取ったデータ内のウォーム [プロンプトキャッシュ](#prompt-cache-fields) が `expires_at` 時刻に到達したとき

Claude Code は更新を 300ms でデバウンスするため、急速な変更がバッチ処理され、スクリプトは変更が停止した後に 1 回実行されます。`command` 自体への変更はデバウンスをスキップします。Claude Code は新しいコマンドをすぐに実行します。スクリプトがまだ実行中に新しい更新がトリガーされた場合、Claude Code は実行中のスクリプトをキャンセルします。スクリプトを編集した場合、変更は更新トリガーが次に実行されるときに表示されます。

イベント駆動型トリガーは、メインセッションがアイドル状態の場合（例えば、コーディネーターがバックグラウンドサブエージェントを待機している場合）、静かになる可能性があります。アイドル期間中に時間ベースまたは外部ソースのセグメントを最新に保つには、[`refreshInterval`](#manually-configure-a-status-line) を設定して、固定タイマーでもコマンドを再実行します。

**スクリプトが出力できるもの**

* **複数行**：各 `echo` または `print` ステートメントは別の行として表示されます。[複数行の例](#display-multiple-lines) を参照してください。
* **色**：`\033[32m` のような [ANSI エスケープコード](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) を使用して緑色を表示します（ターミナルがサポートしている必要があります）。[git ステータスの例](#git-status-with-colors) を参照してください。
* **リンク**：[OSC 8 エスケープシーケンス](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC) を使用してテキストをクリック可能にします（macOS では Cmd+クリック、Windows/Linux では Ctrl+クリック）。iTerm2、Kitty、WezTerm などのハイパーリンクをサポートするターミナルが必要です。[クリック可能なリンクの例](#clickable-links) を参照してください。

**ターミナルに出力をサイズ調整する**

Claude Code はスクリプトの出力をキャプチャするため、ターミナルに直接接続しません。そのため、`tput cols` と言語レベルの幅検出はスクリプト内からターミナルサイズを読み取ることができません。代わりに `COLUMNS` および `LINES` 環境変数を読み取ってください。Claude Code はスクリプトを実行する前に、これらを現在のターミナルサイズに設定します。

<Note>ステータスラインはローカルで実行され、API トークンを消費しません。オートコンプリート提案、ヘルプメニュー、パーミッションプロンプトなど、特定の UI 相互作用中は一時的に非表示になります。</Note>

<h2 id="available-data">
  利用可能なデータ
</h2>

Claude Code は以下の JSON フィールドを stdin 経由でスクリプトに送信します：

| フィールド                                                                           | 説明                                                                                                                                                                                                                                                                                                            |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model.id`、`model.display_name`                                                 | 現在のモデル識別子と表示名                                                                                                                                                                                                                                                                                                 |
| `cwd`、`workspace.current_dir`                                                   | 現在の作業ディレクトリ。両方のフィールドに同じ値が含まれます。`workspace.current_dir` は `workspace.project_dir` との一貫性のために推奨されます。                                                                                                                                                                                                             |
| `workspace.project_dir`                                                         | Claude Code が起動されたディレクトリ。セッション中に作業ディレクトリが変更された場合、`cwd` と異なる場合があります                                                                                                                                                                                                                                            |
| `workspace.added_dirs`                                                          | `/add-dir` または `--add-dir` 経由で追加された追加ディレクトリ。追加されていない場合は空配列                                                                                                                                                                                                                                                    |
| `workspace.git_worktree`                                                        | 現在のディレクトリが `git worktree add` で作成されたリンク worktree 内にある場合の git worktree 名。メイン作業ツリーでは不在。`worktree.*` が [worktree セッション](/docs/ja/worktrees) 中のみに存在するのとは異なり、任意の git worktree に対して入力されます                                                                                                                                |
| `workspace.repo.host`、`workspace.repo.owner`、`workspace.repo.name`              | `origin` リモートから解析されたリポジトリ ID。例えば `"github.com"`、`"anthropics"`、`"claude-code"`。git リポジトリの外部または `origin` リモートが設定されていない場合は不在。gitlab.com プロジェクトがサブグループにネストされている場合、`owner` は `"group/subgroup"` のようなスラッシュ付きの完全な名前空間パスです。v2.1.260 より前は、これらのプロジェクトでは `workspace.repo` が不在でした                                        |
| `cost.total_cost_usd`                                                           | USD でのセッションの推定コスト。クライアント側で計算されます。[`modelPricing`](/docs/ja/settings-reference#modelpricing) テーブルが有効な場合を除き、定価で計算されます。実際の請求額と異なる場合があります。`/clear` で新しいセッションが開始されると \$0 にリセットされます。v2.1.211 より前は、`/clear` の後も合計が引き継がれていました                                                                                              |
| `cost.total_duration_ms`                                                        | セッション開始からの総経過時間（ミリ秒）                                                                                                                                                                                                                                                                                          |
| `cost.total_api_duration_ms`                                                    | API レスポンスを待つのに費やされた総時間（ミリ秒）                                                                                                                                                                                                                                                                                   |
| `cost.total_lines_added`、`cost.total_lines_removed`                             | 変更されたコード行                                                                                                                                                                                                                                                                                                     |
| `context_window.total_input_tokens`、`context_window.total_output_tokens`        | コンテキストウィンドウに現在あるトークン数。最新の API レスポンスから取得。入力にはキャッシュ読み取りと書き込みが含まれます                                                                                                                                                                                                                                              |
| `context_window.context_window_size`                                            | トークン単位の最大コンテキストウィンドウサイズ。デフォルトは 200000、拡張コンテキストを持つモデルの場合は 1000000                                                                                                                                                                                                                                              |
| `context_window.used_percentage`                                                | 事前計算されたコンテキストウィンドウ使用割合                                                                                                                                                                                                                                                                                        |
| `context_window.remaining_percentage`                                           | 事前計算されたコンテキストウィンドウ残り割合                                                                                                                                                                                                                                                                                        |
| `context_window.current_usage`                                                  | 最後の API 呼び出しからのトークン数。[コンテキストウィンドウフィールド](#context-window-fields) で説明されています                                                                                                                                                                                                                                     |
| `exceeds_200k_tokens`                                                           | 最新の API レスポンスからの総トークン数（入力、キャッシュ、出力トークンの組み合わせ）が 200k を超えるかどうか。これは実際のコンテキストウィンドウサイズに関係なく固定閾値です。                                                                                                                                                                                                                 |
| `fast_mode`                                                                     | セッションで [fast mode](/docs/ja/fast-mode) が有効になっているかどうか                                                                                                                                                                                                                                                               |
| `effort.level`                                                                  | 現在の推論努力レベル（`low`、`medium`、`high`、`xhigh`、または `max`）。ライブセッション値を反映しており、セッション中の `/effort` 変更を含みます。Ultracode は個別のレベルではなく、`xhigh` として報告されます。現在のモデルが effort パラメータをサポートしていない場合は不在                                                                                                                                    |
| `thinking.enabled`                                                              | セッションで拡張思考が有効になっているかどうか                                                                                                                                                                                                                                                                                       |
| `rate_limits.five_hour.used_percentage`、`rate_limits.seven_day.used_percentage` | 5 時間または 7 日のレート制限の消費割合（0～100）                                                                                                                                                                                                                                                                                 |
| `rate_limits.five_hour.resets_at`、`rate_limits.seven_day.resets_at`             | 5 時間または 7 日のレート制限ウィンドウがリセットされる Unix エポック秒                                                                                                                                                                                                                                                                     |
| `rate_limits.spend_limit.used_percentage`、`rate_limits.spend_limit.resets_at`   | [Claude apps gateway](/docs/ja/claude-apps-gateway-spend-limits#usage-warnings-in-claude-code) の背後にある場合、あなたに適用される支出制限の使用割合、およびその期間がリセットされる Unix エポック秒。割合は 0～100 の範囲、または制限を超えると 100 以上になります。Claude Code v2.1.251 以降が必要です                                                                                            |
| `prompt_cache`                                                                  | メイン会話の [prompt cache](/docs/ja/prompt-caching) 統計情報：ヒット率、ミス数、キャッシュがウォーム状態かどうか。すべてのフィールドについては [prompt cache フィールド](#prompt-cache-fields) を参照してください。メイン会話の最初の API レスポンスまで不在。Claude Code v2.1.251 以降が必要です                                                                                                             |
| `session_id`                                                                    | 一意のセッション識別子                                                                                                                                                                                                                                                                                                   |
| `session_name`                                                                  | セッション名。`--name` フラグまたは `/rename` で設定されたカスタム名が存在する場合はそれを使用し、そうでない場合は AI が生成したセッションタイトルを使用します。[デフォルト表示名](/docs/ja/sessions#name-your-sessions)（`my-app-3f` など）はこのフィールドに入力されません。セッションにカスタム名も AI が生成したタイトルもない場合は不在                                                                                                     |
| `prompt_id`                                                                     | 現在処理中のユーザープロンプトを識別する UUID。OpenTelemetry イベントの [`prompt.id` 属性](/docs/ja/monitoring-usage#event-correlation-attributes) と一致します。最初のユーザー入力まで不在。Claude Code v2.1.196 以降が必要です                                                                                                                                           |
| `transcript_path`                                                               | 会話トランスクリプトファイルへのパス                                                                                                                                                                                                                                                                                            |
| `version`                                                                       | Claude Code バージョン                                                                                                                                                                                                                                                                                             |
| `output_style.name`                                                             | 現在の出力スタイルの名前                                                                                                                                                                                                                                                                                                  |
| `vim.mode`                                                                      | [vim モード](/docs/ja/interactive-mode#vim-editor-mode) が有効な場合の現在の vim モード（`NORMAL`、`INSERT`、`VISUAL`、または `VISUAL LINE`）                                                                                                                                                                                              |
| `agent.name`                                                                    | `--agent` フラグまたはエージェント設定が設定されている場合のエージェント名                                                                                                                                                                                                                                                                    |
| `pr.number`、`pr.url`                                                            | 現在のブランチのオープンプルリクエスト。フッターの PR バッジをミラーします。リポジトリに GitLab リモートがある場合、Claude Code はこれらのフィールドをブランチのオープン [merge request](/docs/ja/interactive-mode#gitlab-merge-requests) から代わりに入力するため、`pr.number` はマージリクエスト番号です。マージリクエストデータには Claude Code v2.1.234 以降が必要です。git リポジトリにない場合、プルリクエストまたはマージリクエストが見つかるまで、またはマージもしくはクローズされた後は不在 |
| `pr.review_state`                                                               | オープン PR のレビューステータス：`approved`、`pending`、`changes_requested`、または `draft`。`pr` が存在する場合でも独立して不在の可能性があります                                                                                                                                                                                                         |
| `pr.kind`                                                                       | [GitLab merge request](/docs/ja/interactive-mode#gitlab-merge-requests) を説明する場合は `mr`。GitHub プルリクエストでは不在なため、このフィールドの前に書かれたスクリプトは引き続き機能します。マージリクエストの場合、Claude Code は GitLab がマージ可能と報告した場合は `review_state` を `approved` に、その他のオープン状態の場合は `pending` に、ドラフトの場合は `draft` に設定します。Claude Code v2.1.234 以降が必要です            |
| `worktree.name`                                                                 | アクティブな worktree の名前。[worktree セッション](/docs/ja/worktrees) 中のみ存在                                                                                                                                                                                                                                                     |
| `worktree.path`                                                                 | worktree ディレクトリへの絶対パス                                                                                                                                                                                                                                                                                         |
| `worktree.branch`                                                               | worktree の git ブランチ名（例：`"worktree-my-feature"`）。フックベースの worktree では不在                                                                                                                                                                                                                                         |
| `worktree.original_cwd`                                                         | worktree に入る前に Claude がいたディレクトリ                                                                                                                                                                                                                                                                               |
| `worktree.original_branch`                                                      | worktree に入る前にチェックアウトされた git ブランチ。フックベースの worktree では不在                                                                                                                                                                                                                                                       |

<Accordion title="完全な JSON スキーマ">
  ステータスラインコマンドは stdin 経由でこの JSON 構造を受け取ります：

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-5-5",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "prompt_cache": {
      "warm": true,
      "caching_observed": true,
      "ttl": "1h",
      "expires_at": 1738429200,
      "requests": 14,
      "misses": 2,
      "expected_rebuilds": 1,
      "hit_ratio": 0.91,
      "cache_write_tokens": 352000,
      "miss_recache_tokens": 310200,
      "last_miss_at": 1738425230,
      "last_miss_cause": {
        "causes": ["tools_changed"],
        "tools_added": 2,
        "tools_removed": 0
      },
      "miss_causes": {
        "tools_changed": 2
      },
      "recache_tokens_if_cold": 45000
    },
    "fast_mode": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      },
      "spend_limit": {
        "used_percentage": 62.8,
        "resets_at": 1740787200
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **不在の可能性があるフィールド**（JSON に存在しない）：

  * `session_name`：`--name` または `/rename` でカスタム名が設定されている場合、または AI が生成したセッションタイトルが存在する場合に表示されます。`my-app-3f` などのデフォルト表示名はこのフィールドに入力されません
  * `prompt_id`：最初のユーザー入力の後のみ表示
  * `workspace.git_worktree`：現在のディレクトリがリンク git worktree 内にある場合のみ表示
  * `workspace.repo`：git リポジトリ内で `origin` リモートが設定されている場合のみ表示
  * `effort`：現在のモデルが推論努力パラメータをサポートしている場合のみ表示
  * `vim`：vim モードが有効な場合のみ表示
  * `agent`：`--agent` フラグまたはエージェント設定が設定されている場合のみ表示
  * `pr`：現在のブランチのオープン PR または GitLab マージリクエストが見つかった場合のみ表示。PR またはマージリクエストがマージまたはクローズされると削除されます。`pr.review_state` と `pr.kind` は独立して不在の可能性があります
  * `worktree`：[worktree セッション](/docs/ja/worktrees) 中のみ表示。存在する場合、`branch` と `original_branch` もフックベースの worktree では不在の可能性があります
  * `rate_limits`：Claude.ai Pro および Max サブスクライバー、または支出制限を設定する Claude apps gateway の背後にある場合のみ表示。セッションの最初の API レスポンスの後のみ表示。各ウィンドウ（`five_hour`、`seven_day`、`spend_limit`）は独立して不在の可能性があり、Claude Code は `resets_at` 時刻が経過するとウィンドウを削除します。`jq -r '.rate_limits.five_hour.used_percentage // empty'` を使用して、不在を適切に処理します。
  * `prompt_cache`：メイン会話の最初の API レスポンスの後に表示。[prompt cache フィールド](#prompt-cache-fields) を参照してください

  **`null` の可能性があるフィールド**：

  * `context_window.current_usage`：セッションの最初の API 呼び出しの前は `null`。また `/compact` の直後は次の API 呼び出しが再度入力されるまで `null`
  * `context_window.used_percentage`、`context_window.remaining_percentage`：セッションの早期段階では `null` の可能性があります

  スクリプトで条件付きアクセスと null 値のフォールバックデフォルトを使用して、不在のフィールドを処理します。
</Accordion>

<h3 id="context-window-fields">
  コンテキストウィンドウフィールド
</h3>

`context_window` オブジェクトは、最新の API レスポンスからのライブコンテキストウィンドウを説明します。

* **結合合計**（`total_input_tokens`、`total_output_tokens`）：コンテキストウィンドウに現在あるトークン。`total_input_tokens` は `input_tokens`、`cache_creation_input_tokens`、および `cache_read_input_tokens` の合計です。`total_output_tokens` は最新レスポンスからの出力トークンです。両方とも最初の API レスポンスの前は `0` です。
* **コンポーネント別使用状況**（`current_usage`）：カテゴリ別に分類された同じトークン数。キャッシュヒットを新規入力から分離する必要がある場合に使用します。

`current_usage` オブジェクトには以下が含まれます：

* `input_tokens`：現在のコンテキストの入力トークン
* `output_tokens`：生成された出力トークン
* `cache_creation_input_tokens`：キャッシュに書き込まれたトークン
* `cache_read_input_tokens`：キャッシュから読み取られたトークン

キャッシュフィールドの意味とそれらがどのように請求されるかについては、[キャッシュパフォーマンスの確認](/docs/ja/prompt-caching#check-cache-performance) を参照してください。

`used_percentage` フィールドは入力トークンのみから計算されます：`input_tokens + cache_creation_input_tokens + cache_read_input_tokens`。`output_tokens` は含まれません。

`current_usage` から手動でコンテキスト割合を計算する場合、`used_percentage` と一致させるために同じ入力のみの式を使用します。

`current_usage` オブジェクトはセッションの最初の API 呼び出しの前は `null` です。また `/compact` の直後は `null` であり、次の API 呼び出しが再度入力されるまで `null` のままです。

<h3 id="prompt-cache-fields">
  Prompt cache フィールド
</h3>

`prompt_cache` オブジェクトは、セッションのメイン会話が [prompt cache](/docs/ja/prompt-caching) をどのように使用しているかを要約します。Claude Code は API のレスポンスのキャッシュトークン数から計算するため、すべてのプロバイダーで機能します。

オブジェクトはメイン会話の最初の API レスポンスの後に表示されます。Claude Code はこれらの統計情報でサブエージェントリクエストをカウントしません。Claude Code v2.1.251 以降が必要です。

テーブルは各フィールドとその意味を示しています。タイムスタンプは Unix エポック秒で、`rate_limits.*.resets_at` と同じ単位です。短いステータスラインは通常、これらの 1 つまたは 2 つを表示します。`warm` と `hit_ratio` はキャッシュ状態を最も直接的に要約します。

| フィールド                    | 説明                                                                                                                            |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| `warm`                   | キャッシュされたプレフィックスがまだ TTL 内にあるかどうか。最後のレスポンスがキャッシュトークンを報告しなかった場合は `false`。`caching_observed` が `true` の場合でも                       |
| `caching_observed`       | このセッションのレスポンスがキャッシュトークンを報告したかどうか。`false` は prompt caching がオフ、またはプロバイダーまたはゲートウェイがそれを報告しないことを意味します                             |
| `ttl`                    | 現在のキャッシュされたプレフィックスの [キャッシュライフタイム](/docs/ja/prompt-caching#cache-lifetime)：`"5m"` または `"1h"`                                        |
| `expires_at`             | キャッシュされたプレフィックスが TTL を離れてコールドになる時刻（エポック秒）。最後のレスポンスがキャッシュトークンを報告しなかった場合は `null`                                                |
| `requests`               | このセッションのメイン会話で記録された API リクエスト                                                                                                 |
| `misses`                 | キャッシュが既に保持していたコンテンツを再処理したリクエスト：キャッシュが読み取ることができた 5% 以上かつ少なくとも 2,000 トークン。コンパクション またはツール結果のクリアで不足を説明できない場合                      |
| `expected_rebuilds`      | コンパクションまたは古いツール結果のクリアに続いたキャッシュリビルド                                                                                            |
| `hit_ratio`              | キャッシュ読み取りトークンをこのセッションのすべての入力トークンの分数として表したもの。0～1 の範囲。分母はキャッシュ読み取り、キャッシュ書き込み、およびキャッシュされていない入力をカウントします。これらのカウントがすべてゼロの場合は `null` |
| `cache_write_tokens`     | このセッション中にキャッシュに書き込まれたすべてのトークン。最初のリクエストの初期書き込みを含む                                                                              |
| `miss_recache_tokens`    | ミスとしてカウントされたリクエストによってキャッシュに書き込まれたトークン                                                                                         |
| `last_miss_at`           | 最後のミスが発生した時刻（エポック秒）。セッションにミスがない場合は `null`                                                                                     |
| `last_miss_cause`        | Claude Code が最後のミスの可能な原因として特定したもの。[最後のミスの原因](#last-miss-cause) で説明されています。Claude Code v2.1.260 以降が必要です                         |
| `miss_causes`            | このセッションの診断されたミスのうち、各原因を持つ数。`last_miss_cause` と同じ原因名でキー付けされています。Claude Code v2.1.260 以降が必要です                                   |
| `recache_tokens_if_cold` | キャッシュがそれまでにコールドになった場合、次のリクエストが再キャッシュするトークン。コンパクションまたは古いツール結果のクリアの直後は `null`。次のリクエストが書き直された会話のサイズを記録するまで `null` のままです          |

Claude Code はターミナルで同じ統計情報を表示します。[`/usage` コマンドの `Prompt cache (main)` 行](/docs/ja/costs#prompt-cache-statistics) を参照してください。

<h4 id="last-miss-cause">
  最後のミスの原因
</h4>

`last_miss_cause` オブジェクトは、Claude Code が最新のミスの可能な原因として特定したものを報告します。その `causes` 配列は `tools_changed`、`system_prompt_changed`、`ttl_expired_5m`、または `likely_server_side` などの 1 つ以上の原因名を保持します。オブジェクトはセッションの最初のミスまで `null` であり、Claude Code が最新のミスの原因を特定できなかった場合は再び `null` になります。Claude Code v2.1.260 以降が必要です。

2 つの原因はオブジェクトにカウントを追加します：

* `tools_added` と `tools_removed`：`tools_changed` を使用して、リクエストに追加または削除されたツールの数
* `system_char_delta`：`system_prompt_changed` を使用して、システムプロンプトの長さの変化（文字数）

<h2 id="examples">
  例
</h2>

これらの例は一般的なステータスラインパターンを示しています。任意の例を使用するには：

1. スクリプトを `~/.claude/statusline.sh`（または `.py`/`.js`）などのファイルに保存します
2. 実行可能にします：`chmod +x ~/.claude/statusline.sh`
3. [設定](#manually-configure-a-status-line) にパスを追加します

Bash の例は [`jq`](https://jqlang.org/) を使用して JSON を解析します。Python と Node.js には組み込みの JSON 解析があります。

<h3 id="context-window-usage">
  コンテキストウィンドウの使用状況
</h3>

現在のモデルとコンテキストウィンドウの使用状況を視覚的なプログレスバーで表示します。各スクリプトは stdin から JSON を読み取り、`used_percentage` フィールドを抽出し、塗りつぶされたブロック（▓）が使用状況を表す 10 文字のバーを構築します：

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="モデル名とパーセンテージ付きプログレスバーを表示するステータスライン" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # stdin 全体を変数に読み込む
  input=$(cat)

  # jq でフィールドを抽出します。"// 0" は null のフォールバックを提供します
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # プログレスバーを構築します：printf -v はスペースを作成し、
  # ${var// /▓} は各スペースをブロック文字に置き換えます
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load は stdin を 1 ステップで読み取り、解析します
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" は null 値を処理します
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # 文字列乗算がバーを構築します
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js はイベントで stdin を非同期に読み取ります
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // オプショナルチェーン（?.）は null フィールドを安全に処理します
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() がバーを構築します
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  git ステータスと色
</h3>

ステージングされたファイルと変更されたファイルのカラーコード化されたインジケーターを使用して git ブランチを表示します。このスクリプトはターミナルの色に [ANSI エスケープコード](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) を使用します：`\033[32m` は緑、`\033[33m` は黄、`\033[0m` はデフォルトにリセットします。

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="モデル、ディレクトリ、git ブランチ、ステージングされたファイルと変更されたファイルのカラーコード化されたインジケーターを表示するステータスライン" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

各スクリプトは現在のディレクトリが git リポジトリであるかどうかを確認し、ステージングされたファイルと変更されたファイルをカウントし、カラーコード化されたインジケーターを表示します：

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  コストと期間の追跡
</h3>

セッションの API コストと経過時間を追跡します。`cost.total_cost_usd` フィールドは現在のセッションのすべての API 呼び出しの推定コストを累積します。`cost.total_duration_ms` フィールドはセッション開始からの総経過時間を測定し、`cost.total_api_duration_ms` は API レスポンスを待つのに費やされた時間のみを追跡します。

各スクリプトはコストを通貨としてフォーマットし、ミリ秒を分と秒に変換します：

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="モデル名、セッションコスト、期間を表示するステータスライン" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  複数行を表示する
</h3>

スクリプトは複数の行を出力して、より豊かなディスプレイを作成できます。

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="最初の行にモデル名、ディレクトリ、git ブランチを表示し、2 番目の行にコンテキスト使用状況プログレスバー、コスト、期間を表示する複数行ステータスライン" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

この例は複数のテクニックを組み合わせています：閾値ベースの色（70% 未満は緑、70～89% は黄、90% 以上は赤）、プログレスバー、git ブランチ情報。各 `print` または `echo` ステートメントは別の行を作成します：

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # コンテキスト使用状況に基づいてバーの色を選択します
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  クリック可能なリンク
</h3>

この例は GitHub リポジトリへのクリック可能なリンクを作成します。Cmd（macOS）または Ctrl（Windows/Linux）を押しながらクリックして、ブラウザでリンクを開きます。

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="GitHub リポジトリへのクリック可能なリンクを表示するステータスライン" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

各スクリプトは git リモート URL を取得し、SSH 形式を HTTPS に変換し、リポジトリ名を OSC 8 エスケープコードでラップします。Bash バージョンは `printf '%b'` を使用します。これはバックスラッシュエスケープを異なるシェル間でより確実に解釈します：

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # git SSH URL を HTTPS に変換します
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 形式：\e]8;;URL\a その後 TEXT その後 \e]8;;\a
      # printf %b はシェル間でエスケープシーケンスを確実に解釈します
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # git リモート URL を取得します
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # SSH を HTTPS 形式に変換します
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 エスケープシーケンス
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // SSH を HTTPS 形式に変換します
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 エスケープシーケンス
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  レート制限の使用状況
</h3>

claude.ai サブスクリプションのレート制限使用状況をステータスラインに表示します。`rate_limits` オブジェクトには、ローリング `five_hour` ウィンドウと週間 `seven_day` ウィンドウが含まれます。各ウィンドウは `used_percentage`（0～100）とウィンドウがリセットされる Unix エポック秒の `resets_at` を提供します。

Claude アプリゲートウェイの背後にある支出制限を使用する場合、`rate_limits` は支出制限に対して同じ 2 つのフィールドを持つ `spend_limit` を含みます。ただし、その `used_percentage` は制限を超えると 100 を超える可能性があります。Claude Code v2.1.251 以降が必要です。

`rate_limits` オブジェクトは claude.ai Pro および Max サブスクライバー、または支出制限を持つ Claude アプリゲートウェイの背後にある場合のみ存在し、最初の API レスポンスの後のみです。各スクリプトは不在のフィールドを適切に処理します：

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" は rate_limits が不在の場合、出力を生成しません
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  高コストな操作をキャッシュする
</h3>

ステータスラインスクリプトはアクティブなセッション中に頻繁に実行されます。`git status` や `git diff` などのコマンドは、特に大規模なリポジトリでは遅い場合があります。この例は git 情報を一時ファイルにキャッシュし、5 秒ごとにのみ更新します。

キャッシュファイル名は、セッション内のステータスラインの呼び出し間で安定している必要がありますが、異なるリポジトリの同時セッションが互いのキャッシュされた git 状態を読み取らないように、セッション間で一意である必要があります。`$$`、`os.getpid()`、`process.pid` のようなプロセスベースの識別子は、呼び出しのたびに変わり、キャッシュを無効にします。代わりに JSON 入力から `session_id` を使用します：これはセッションの有効期間中は安定しており、セッションごとに一意です。

各スクリプトは git コマンドを実行する前に、キャッシュファイルが不在であるか 5 秒より古いかを確認します：

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # 秒

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -c %Y（Linux）または stat -f %m（macOS）はファイルの最終更新時刻を出力します。
      # Linux フォームは最初に実行する必要があります：Linux では、macOS フォームは stdout にファイルシステムレポートを出力してから失敗し、その出力はコマンド置換によってキャプチャされ、算術を破壊します。
      [ $(($(date +%s) - $(stat -c %Y "$CACHE_FILE" 2>/dev/null || stat -f %m "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # 秒

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // 秒

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Windows 設定
</h3>

Windows では、Claude Code はステータスラインコマンドを Git Bash 経由で実行します。Git Bash がインストールされている場合、または Git Bash がない場合は PowerShell を通じて実行します。

Git Bash は引用符なしのバックスラッシュをエスケープ文字として扱うため、`C:\Users\username\script.mjs` のような Windows スタイルのパスはセパレーターが削除された状態でスクリプトランナーに到達し、目に見えるエラーなしでコマンドが失敗します。`command` 文字列のファイルパスを以下の例に示すようにフォワードスラッシュで記述します。`~` 短縮形も機能し、Windows ホームディレクトリに展開されます。

PowerShell スクリプトをステータスラインとして実行するには、`powershell` 経由で呼び出します。これは Claude Code がコマンドを Git Bash または PowerShell を通じてルーティングするかどうかに関わらず機能します：

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

または、Git Bash がインストールされている場合は、Bash スクリプトを直接実行します：

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  サブエージェントステータスライン
</h2>

`subagentStatusLine` 設定は、エージェントパネルに表示される各 [サブエージェント](/docs/ja/sub-agents) のカスタム行本体をレンダリングします。デフォルトの `name · description · token count` 行を独自のフォーマットに置き換えるために使用します。

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

コマンドは、すべての表示されているサブエージェント行が stdin で単一の JSON オブジェクトとして渡される各リフレッシュティックで実行されます。入力には [基本フックフィールド](/docs/ja/hooks#common-input-fields)、使用可能な行幅を示す `columns` フィールド、および `tasks` 配列が含まれます。各タスクには `id`、`name`、`type`、`status`、`description`、`label`、`startTime`、`model`、`effort`、`contextWindowSize`、`tokenCount`、`tokenSamples`、`cwd` があります。

タスクごとの `model` フィールドは、タスクが実行される解決済みモデル ID です。`contextWindowSize` はそのモデルのコンテキストウィンドウ（トークン単位）で、メインステータスラインの `context_window.context_window_size` と同じ方法で計算されるため、`tokenCount` から行ごとのパーセンテージをレンダリングできます。両方のフィールドには Claude Code v2.1.205 以降が必要で、モデルがまだ解決されていないタスクでは省略されます。

タスクごとの `effort` フィールドは、そのサブエージェントに設定された推論努力で、その [定義フロントマター](/docs/ja/sub-agents#supported-frontmatter-fields) または個別の呼び出しで設定されます。値は、努力レベル文字列 `low`、`medium`、`high`、`xhigh`、`max` のいずれか、またはトークン予算の数値です。フィールドは、記述されたとおりに設定された値を報告します。モデルがそのレベルをサポートしていない場合、Claude Code が実際に適用する努力は異なる可能性があります。フィールドには Claude Code v2.1.214 以降が必要で、サブエージェントがセッションの努力レベルを継承する場合は存在しません。

オーバーライドしたい各行に対して stdout に 1 つの JSON 行を書き込みます。形式は `{"id": "<task id>", "content": "<row body>"}` です。`content` 文字列はそのままレンダリングされます。ANSI 色と OSC 8 ハイパーリンクを含みます。タスクの `id` を省略して、その行のデフォルトレンダリングを保持します。空の `content` 文字列を出力して、その行を非表示にします。

`statusLine` に適用される同じトラストと `disableAllHooks` および [`allowManagedHooksOnly`](/docs/ja/settings-reference#allowmanagedhooksonly) ゲートが `subagentStatusLine` に適用されます。プラグインは、[`settings.json`](/docs/ja/plugins/manifest-reference#standard-layout) でデフォルトの `subagentStatusLine` を配布できます。ただし、フックとは異なり、プラグインが管理設定で強制的に有効化されている場合でも、プラグイン値は `allowManagedHooksOnly` の下で実行されません。

<h2 id="tips">
  ヒント
</h2>

* **モック入力でテストする**：`echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **出力を短く保つ**：ステータスバーの幅は限られているため、長い出力は切り詰められたり、不適切にラップされたりする可能性があります
* **遅い操作をキャッシュする**：スクリプトはアクティブなセッション中に頻繁に実行されるため、`git status` などのコマンドは遅延を引き起こす可能性があります。これを処理する方法については、[キャッシング例](#cache-expensive-operations) を参照してください。

[ccstatusline](https://github.com/sirmalloc/ccstatusline) や [starship-claude](https://github.com/martinemde/starship-claude) などのコミュニティプロジェクトは、テーマと追加機能を備えた事前構築設定を提供します。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

**ステータスラインが表示されない**

* スクリプトが実行可能であることを確認します：`chmod +x ~/.claude/statusline.sh`
* スクリプトが stdout に出力し、stderr に出力していないことを確認します
* スクリプトを手動で実行して、出力を生成することを確認します
* Windows で Git Bash がインストールされている場合、`command` パスのバックスラッシュはスクリプトが実行される前にエスケープ文字として消費される可能性があります。パスでは前方スラッシュを使用してください。[Windows 設定](#windows-configuration)を参照してください。
* [設定の優先順位](/docs/ja/hooks#disable-or-remove-hooks)が適用された後、`disableAllHooks` が管理設定外で `true` の場合、Claude Code は管理設定からの `statusLine` のみを実行し、管理 `statusLine` がない場合はステータスラインが無効になります。この設定を削除するか、それを設定するファイルで `false` に設定して、再度有効にします。[`disableAllHooks`](/docs/ja/settings-reference#disableallhooks)を参照してください。
* 組織が管理設定で `allowManagedHooksOnly` を設定している場合、カスタムステータスラインは警告なく消えます：これらの管理設定の `statusLine` 値からのみステータスラインを取得できます。[`allowManagedHooksOnly` で実行される内容](/docs/ja/settings-reference#what-runs-under-allowmanagedhooksonly)を参照して完全な動作を確認し、この設定があなたに適用されるかどうかを管理者に確認してください。
* `claude --debug` を実行して、セッションの最初のステータスラインの呼び出しからの終了コードと stderr をログに記録します
* Claude にスクリプトファイルを読み取り、`statusLine` コマンドを直接実行するよう依頼して、エラーを表示します

**ステータスラインが `--` または空の値を表示する**

* フィールドは最初の API レスポンスが完了する前は `null` の可能性があります
* スクリプトで `// 0` のようなフォールバックを使用して null 値を処理します
* 複数のメッセージの後も値が空のままの場合は、Claude Code を再起動します

**コンテキスト割合が予期しない値を表示する**

* 最も単純で正確なコンテキスト状態には `used_percentage` を使用します
* コンテキスト割合は `/context` 出力と異なる場合があります。これは各が計算されるタイミングが異なるためです

**OSC 8 リンクがクリック可能でない**

* ターミナルが OSC 8 ハイパーリンクをサポートしていることを確認します（iTerm2、Kitty、WezTerm）

* Terminal.app はクリック可能なリンクをサポートしていません

* リンクテキストが表示されているがクリック可能でない場合、Claude Code がターミナルのハイパーリンクサポートを検出できていない可能性があります。Claude Code を起動する前に `FORCE_HYPERLINK` 環境変数を設定して、検出をオーバーライドします：

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  PowerShell では、最初に現在のセッションで変数を設定します：

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* SSH と tmux セッションは設定に応じて OSC シーケンスをストリップする可能性があります

* エスケープシーケンスが `\e]8;;` のようなリテラルテキストとして表示される場合は、`echo -e` の代わりに `printf '%b'` を使用して、より確実なエスケープ処理を行います

**エスケープシーケンスでの表示の不具合**

* 複雑なエスケープシーケンス（ANSI 色、OSC 8 リンク）は、他の UI 更新と重複する場合、時々破損した出力を引き起こす可能性があります
* 破損したテキストが表示される場合は、スクリプトをプレーンテキスト出力に簡略化してみてください
* エスケープコード付きの複数行ステータスラインは、プレーンテキストの単一行よりもレンダリングの問題が発生しやすくなります

**ワークスペーストラストが必要**

* `statusLine` はシェルコマンドを実行するため、Claude Code は[設定ファイルのフックと同じワークスペーストラストルール](/docs/ja/permissions#what-runs-before-you-trust-a-folder)の下で実行します。フォルダのダイアログを受け入れるか、その信頼がそれに拡張される親ディレクトリのダイアログを受け入れるだけで十分です。
* それまでの間、ステータスラインは空白のままで、`claude --debug` は `Status line command skipped: workspace trust not accepted` をログに記録します。Claude Code を再起動し、トラストダイアログを受け入れて有効にします。

**スクリプトエラーまたはハング**

* ゼロ以外のコードで終了するか、出力を生成しないスクリプトは、ステータスラインを空白にします
* 遅いスクリプトは、完了するまでステータスラインの更新をブロックします。古い出力を避けるために、スクリプトを高速に保ちます。
* 遅いスクリプトの実行中に新しい更新がトリガーされた場合、実行中のスクリプトはキャンセルされます
* 設定する前に、モック入力を使用してスクリプトを独立してテストします

**通知がステータスラインの行を共有する**

[フルスクリーンレンダリング](/docs/ja/fullscreen)の外では、Claude Code は通知をステータスラインと同じ行に表示します。フルスクリーンレンダリングでは、Claude Code は通知に独自の行を提供します。

* MCP サーバーエラーおよび自動更新などのシステム通知は、ステータスラインと同じ行の右側に表示されます。コンテキスト低警告などの一時的な通知もこの領域を循環します。
* 詳細モードを有効にすると、この領域にトークンカウンターが追加されます
* 狭いターミナルでは、これらの通知がステータスラインの出力を切り詰める可能性があります
