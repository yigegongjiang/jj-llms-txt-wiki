> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# hooks でアクションを自動化する

> Claude Code がファイルを編集したり、タスクを完了したり、入力が必要になったりしたときに、シェルコマンドを自動的に実行します。コードをフォーマットし、通知を送信し、コマンドを検証し、プロジェクトルールを適用します。

Hooks はユーザー定義のシェルコマンドです。Claude Code はそのライフサイクルの特定のポイントで実行され、決定論的な制御を提供します。LLM が実行を選択するのに依存するのではなく、特定のアクションが常に発生します。Hooks を使用して、プロジェクトルールを適用し、反復的なタスクを自動化し、Claude Code を既存のツールと統合します。

判断が必要な決定については、決定論的なルールではなく、Claude モデルを使用して条件を評価する [プロンプトベースの hooks](#prompt-based-hooks) または [エージェントベースの hooks](#agent-based-hooks) を使用することもできます。

Claude Code を拡張する他の方法については、Claude に追加の指示と実行可能なコマンドを与えるための [skills](/docs/ja/skills)、分離されたコンテキストでタスクを実行するための [subagents](/docs/ja/sub-agents)、プロジェクト全体で共有する拡張機能をパッケージ化するための [plugins](/docs/ja/plugins/overview) を参照してください。

<Tip>
  このガイドでは一般的なユースケースと始め方をカバーしています。完全なイベントスキーマ、JSON 入力/出力形式、非同期 hooks や MCP ツール hooks などの高度な機能については、[Hooks リファレンス](/docs/ja/hooks) を参照してください。
</Tip>

<h2 id="set-up-your-first-hook">
  最初の hook をセットアップする
</h2>

Hook を作成するには、[設定ファイル](#configure-hook-location) に `hooks` ブロックを追加します。このチュートリアルではデスクトップ通知 hook を作成するため、Claude があなたの入力を待っているときにアラートを受け取ることができます。ターミナルを監視する代わりに。

<Steps>
  <Step title="hook を設定に追加する">
    `~/.claude/settings.json` を開き、`Notification` hook を追加します。ファイルが存在しない場合は、作成してください。以下の例は macOS 用に `osascript` を使用しています。Linux と Windows のコマンドについては、[Claude が入力を必要とするときに通知を受け取る](#get-notified-when-claude-needs-input) を参照してください。

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    設定ファイルに既に `hooks` キーがある場合は、オブジェクト全体を置き換えるのではなく、`Notification` を既存のイベントキーの兄弟として追加します。各イベント名は単一の `hooks` オブジェクト内のキーです：

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    CLI で説明することで、Claude に hook を書いてもらうこともできます。
  </Step>

  <Step title="設定を確認する">
    `/hooks` と入力して hooks ブラウザを開きます。利用可能なすべての hook イベントのリストが表示され、hooks が設定されているイベントの横に数が表示されます。`Notification` を選択して、新しい hook がリストに表示されることを確認します。Hook を選択すると、その詳細が表示されます：イベント、マッチャー、タイプ、ソースファイル、およびコマンド。
  </Step>

  <Step title="hook をテストする">
    `Esc` を押して CLI に戻ります。`Shift+Tab` を押してステータスバーに `⏸ manual mode on` が表示されるまで続け、Claude に許可が必要な何かをするよう依頼し、ターミナルから切り替えます。デスクトップ通知を受け取るはずです。
  </Step>
</Steps>

<Tip>
  `/hooks` メニューは読み取り専用です。Hooks を追加、変更、または削除するには、設定 JSON を直接編集するか、Claude に変更を依頼します。
</Tip>

<h2 id="what-you-can-automate">
  自動化できるもの
</h2>

Hooks を使用すると、Claude Code のライフサイクルの主要なポイントでコードを実行できます：編集後にファイルをフォーマットし、実行前にコマンドをブロックし、Claude が入力を必要とするときに通知を送信し、セッション開始時にコンテキストを注入するなど。Hook イベントの完全なリストについては、[Hooks リファレンス](/docs/ja/hooks#hook-lifecycle) を参照してください。

各例には、[設定ファイル](#configure-hook-location) に追加する準備ができた設定ブロックが含まれています。

本番環境での hooks の例として、別のモデルレビューを実行し、その結果をセッションにフィードバックする場合は、[`security-guidance` プラグインが Claude Code と統合する方法](/docs/ja/security-guidance#how-the-plugin-integrates-with-claude-code) を参照してください。

<h3 id="get-notified-when-claude-needs-input">
  Claude が入力を必要とするときに通知を受け取る
</h3>

Claude が作業を完了して入力を必要とするときはいつでもデスクトップ通知を取得し、ターミナルをチェックせずに他のタスクに切り替えることができます。

この hook は `Notification` イベントを使用します。これは Claude が入力または許可を待っているときに発火します。[各通知タイプが発火するタイミング](/docs/ja/hooks#notification) を参照して、正確なタイミングを確認してください。以下の各タブはプラットフォームのネイティブ通知コマンドを使用します。これを `~/.claude/settings.json` に追加します：

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="通知が表示されない場合">
      `osascript` は組み込みの Script Editor アプリを通じて通知をルーティングします。Script Editor に通知権限がない場合、コマンドは静かに失敗し、macOS はそれを付与するよう求めません。Terminal でこれを 1 回実行して、Script Editor を通知設定に表示させます：

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      まだ何も表示されません。**System Settings > Notifications** を開き、リストで **Script Editor** を見つけて、**Allow Notifications** をオンにします。コマンドを再度実行して、テスト通知が表示されることを確認します。
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="通知が表示されない場合">
      `notify-send` はデスクトップ通知デーモンが必要です。ヘッドレスサーバー、SSH セッション、ほとんどのコンテナにはこれがありません。まずコマンドを直接テストしてください：

      ```bash theme={null}
      notify-send 'Claude Code' 'test'
      ```

      コマンドが見つからない場合は、Debian と Ubuntu で `libnotify-bin` パッケージをインストールするか、ディストリビューションの同等のものをインストールしてください。
    </Accordion>
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="ダイアログが表示されない場合">
      このコマンドは画面の隅の通知ではなくダイアログボックスを開くため、ダイアログはターミナルウィンドウの背後で開く可能性があります。まず PowerShell でコマンドを直接テストしてください。Claude Code を WSL 内で実行する場合、`powershell.exe` は Windows interop を通じて `PATH` で利用可能である必要があります。
    </Accordion>
  </Tab>
</Tabs>

空の `matcher` はすべての通知タイプで発火します。特定のイベントでのみ発火させるには、次のいずれかの値に設定します：

| Matcher                      | 発火するタイミング                                                                                                                                                                                                                                                                                               |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permission_prompt`          | Claude がツール使用を承認する必要があるか、サンドボックス化されたコマンドの[ネットワークリクエスト](/docs/ja/sandboxing#network-isolation)を承認する必要があり、プロンプトが約 6 秒待機している                                                                                                                                                                                    |
| `idle_prompt`                | Claude が約 60 秒前に応答を完了し、その後入力していない                                                                                                                                                                                                                                                                       |
| `auth_success`               | 認証が完了したとき                                                                                                                                                                                                                                                                                               |
| `elicitation_dialog`         | MCP サーバーが引き出しフォームを開き、約 6 秒入力していない                                                                                                                                                                                                                                                                       |
| `elicitation_url_dialog`     | MCP サーバーがブラウザURL を開くよう求め、約 6 秒入力していない                                                                                                                                                                                                                                                                   |
| `elicitation_complete`       | MCP サーバーが[URL モード引き出し](/docs/ja/hooks#elicitation-input)が完了したことを報告する                                                                                                                                                                                                                                         |
| `elicitation_response`       | MCP 引き出し応答がサーバーに送り返されたとき                                                                                                                                                                                                                                                                                |
| `agent_needs_input`          | バックグラウンドセッションがあなたの入力を待つのを開始するか、現在のセッションが[agent team チームメイトのターミナルセットアップ質問](/docs/ja/agent-teams#choose-a-display-mode)を尋ね、約 6 秒入力していない。[agent view](/docs/ja/agent-view) が開いている間のみ発火します                                                                                                                            |
| `agent_completed`            | バックグラウンドセッションが完了または失敗します。[agent view](/docs/ja/agent-view) が開いている間のみ発火します                                                                                                                                                                                                                                    |
| `quota_auto_resume_fired`    | Claude Code は claude.ai 使用制限が一時停止した後、タスクを続行します：リセット時、または Claude Code 中に何かを行うことで使用可能になったとき（使用クレジットの追加、プランのアップグレード、モデルの切り替えなど）。[モデル設定の例外](/docs/ja/interactive-mode#wait-for-a-usage-limit-to-reset)を参照してください                                                                                                  |
| `quota_auto_resume_stale`    | claude.ai 使用制限がコンピュータが約 30 分以上スリープしている間にリセットされました。Claude Code はタスクを続行する代わりに `Enter` キーを押すのを待ちます。より短いスリープの後、それは続行し、代わりに `quota_auto_resume_fired` を発火します                                                                                                                                                 |
| `quota_auto_resume_disabled` | Claude Code は claude.ai 使用制限の待機をタスクを続行せずに終了します：[`autoContinueAtUsageLimit`](/docs/ja/settings-reference#autocontinueatusagelimit) がオフになったか、リセットが Claude Code が開始した待機中に 24 時間以上先に移動した、続行されたタスクが制限に引き続きヒットした、または継続がモデルに到達する前にブロックされました。`Esc` または `Ctrl+C` を押すか、**Don't continue automatically** を選択したときは発火しません |

Claude Code は `permission_prompt` をターミナルと Claude Desktop、VS Code 拡張機能、および Agent SDK を通じて許可リクエストに答えるその他のホストで異なる方法でタイミングします。[各通知タイプが発火するタイミング](/docs/ja/hooks#notification) を参照して、両方のタイミングを確認してください。

`agent_needs_input` および `agent_completed` マッチャーには Claude Code v2.1.198 以降が必要です。

`quota_auto_resume_fired`、`quota_auto_resume_stale`、および `quota_auto_resume_disabled` マッチャーには Claude Code v2.1.234 以降が必要です。

ターミナルセッションでは、サンドボックス化されたコマンドのネットワークリクエストに対する `permission_prompt` には Claude Code v2.1.246 以降が必要です。

チームメイトのターミナルセットアップ質問に対する `agent_needs_input` には Claude Code v2.1.248 以降が必要です。

`/hooks` と入力して `Notification` を選択し、hook が登録されていることを確認します。完全なイベントスキーマについては、[Notification リファレンス](/docs/ja/hooks#notification) を参照してください。

<h3 id="auto-format-code-after-edits">
  編集後にコードを自動フォーマットする
</h3>

Claude が編集するすべてのファイルで [Prettier](https://prettier.io/) を自動的に実行し、手動操作なしでフォーマットの一貫性を保ちます。

この hook は `PostToolUse` イベントを `Edit|Write` マッチャーで使用するため、ファイル編集ツールの後にのみ実行されます。コマンドは [`jq`](https://jqlang.org/) で編集されたファイルパスを抽出し、Prettier に渡します。これをプロジェクトルートの `.claude/settings.json` に追加します：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

hook をテストするには、Claude に JavaScript ファイルにシングルクォート文字列を含む行を追加するよう求めてください。その後ファイルを開きます：Prettier のデフォルト設定では、hook はそれらをダブルクォートに書き直します。

hook が成功すると、Claude Code は会話に何も表示しません。hook が実行されたことを確認するには、編集されたファイルが再フォーマットされていることを確認するか、[デバッグテクニック](#debug-techniques) を参照してください。

ファイルが `Bash` コマンドで書き直されるときを含め、特定のファイルがどのように変更されても再フォーマットするには、代わりに [FileChanged](/docs/ja/hooks#filechanged) hook を使用してください。

<Note>
  このページの Bash の例は JSON 解析に `jq` を使用します。macOS で `brew install jq`、Debian と Ubuntu で `apt-get install jq` でインストールするか、[`jq` ダウンロード](https://jqlang.org/download/) を参照してください。
</Note>

<h3 id="block-edits-to-protected-files">
  保護されたファイルへの編集をブロックする
</h3>

Claude が `.env`、`package-lock.json`、`.git/` 内のものなどの機密ファイルを変更するのを防ぎます。Claude は編集がブロックされた理由を説明するフィードバックを受け取るため、アプローチを調整できます。

この例は hook が呼び出す別のスクリプトファイルを使用します。スクリプトはターゲットファイルパスを保護されたパターンのリストに対してチェックし、終了コード 2 で編集をブロックします。

<Steps>
  <Step title="hook スクリプトを作成する">
    これを `.claude/hooks/protect-files.sh` に保存します：

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    # Windows バックスラッシュセパレータを正規化して、以下のパターンがマッチするようにします
    FILE_PATH="${FILE_PATH//\\//}"

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="macOS と Linux でスクリプトを実行可能にする">
    Hook スクリプトが Claude Code で実行されるには、実行可能である必要があります：

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="hook を登録する">
    `.claude/settings.json` に `PreToolUse` hook を追加して、`Edit` または `Write` ツール呼び出しの前にスクリプトを実行します：

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="hook をテストする">
    Claude に `.env` ファイルにコメントを追加するよう求めてください。Claude Code は実行前に編集をブロックし、スクリプトの `Blocked:` メッセージを Claude にフィードバックとして渡します。
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  圧縮後にコンテキストを再注入する
</h3>

Claude のコンテキストウィンドウがいっぱいになると、圧縮は会話を要約してスペースを解放します。これは重要な詳細を失う可能性があります。`compact` マッチャーで `SessionStart` hook を使用して、すべての圧縮後に重要なコンテキストを再注入します。

Claude Code はコマンドが stdout に書き込むプレーンテキストを Claude のコンテキストに追加します。この例はプロジェクト規約と最近の作業を Claude に思い出させます。これをプロジェクトルートの `.claude/settings.json` に追加します：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

`echo` を `git log --oneline -5` などの動的出力を生成するコマンドに置き換えて、最近のコミットを表示できます。すべてのセッション開始時にコンテキストを注入する場合は、代わりに [CLAUDE.md](/docs/ja/memory) を使用することを検討してください。環境変数については、リファレンスの [`CLAUDE_ENV_FILE`](/docs/ja/hooks#persist-environment-variables) を参照してください。

<h3 id="audit-configuration-changes">
  設定変更を監査する
</h3>

セッション中に設定またはスキルファイルが変更されたときを追跡します。`ConfigChange` イベントは外部プロセスまたはエディタが設定ファイルを変更したときに発火するため、コンプライアンスのために変更をログに記録したり、不正な変更をブロックしたりできます。

この例は各変更を監査ログに追加します。これを `~/.claude/settings.json` に追加します：

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

マッチャーは設定タイプでフィルタリングします：`user_settings`、`project_settings`、`local_settings`、`policy_settings`、または `skills`。変更が有効になるのをブロックするには、終了コード 2 で終了するか、`{"decision": "block"}` を返します。完全な入力スキーマについては、[ConfigChange リファレンス](/docs/ja/hooks#configchange) を参照してください。

hook が変更を記録することを確認するには、セッションが実行中に別のエディタで設定ファイルを編集してから、`~/claude-config-audit.log` を開きます：hook は変更ごとに 1 つの JSON 行をタイムスタンプ、ソース、ファイルパスとともに追加します。

<h3 id="reload-environment-when-directory-or-files-change">
  ディレクトリまたはファイルが変更されたときに環境をリロードする
</h3>

一部のプロジェクトは、どのディレクトリにいるかに応じて異なる環境変数を設定します。[direnv](https://direnv.net/) などのツールはシェルで自動的にこれを行いますが、Claude の Bash ツールはそれらの変更を自動的に取得しません。

`SessionStart` hook を `CwdChanged` hook とペアリングすることでこれを修正します。`SessionStart` は起動したディレクトリの変数をロードし、`CwdChanged` は Claude がディレクトリを変更するたびにそれらをリロードします。どちらも `CLAUDE_ENV_FILE` に書き込み、Claude Code は各 Bash コマンドの前にスクリプトプリアンブルとして実行します。これを `~/.claude/settings.json` に追加します：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

`direnv allow` をすべてのディレクトリで 1 回実行して、direnv が `.envrc` をロードすることが許可されるようにします。direnv の代わりに devbox または nix を使用する場合、同じパターンは `direnv export bash` の代わりに `devbox shellenv` または `devbox global shellenv` で機能します。

すべてのディレクトリ変更ではなく、特定のファイルに反応するには、`FileChanged` を `matcher` で使用して、監視するファイル名をリストします（パイプで区切られています）。ウォッチリストを構築するために、この値は正規表現として評価されるのではなく、リテラルファイル名に分割されます。[FileChanged](/docs/ja/hooks#filechanged) を参照して、同じ値がファイルが変更されたときにどの hook グループが実行されるかをフィルタリングする方法を確認してください。この例は現在のディレクトリの `.envrc` と `.env` を監視します：

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

入力スキーマ、`watchPaths` 出力、および `CLAUDE_ENV_FILE` の詳細については、[CwdChanged](/docs/ja/hooks#cwdchanged) および [FileChanged](/docs/ja/hooks#filechanged) リファレンスエントリを参照してください。

<h3 id="auto-approve-specific-permission-prompts">
  特定の許可プロンプトを自動承認する
</h3>

常に許可するツール呼び出しの承認ダイアログをスキップします。この例は `ExitPlanMode` を自動承認します。これは Claude がプランの提示を終了して続行するよう求めるときに呼び出すツールです。プランが準備できるたびにプロンプトが表示されることはありません。

上記の終了コード例とは異なり、自動承認には hook が JSON 決定を stdout に書き込む必要があります。Claude Code は `PermissionRequest` hook を実行します。これは Claude Code が許可ダイアログを表示しようとするときに実行され、hook が `"behavior": "allow"` を返すと、Claude Code はあなたの代わりにリクエストに答えます。

マッチャーは hook を `ExitPlanMode` のみにスコープするため、他のプロンプトは影響を受けません。これを `~/.claude/settings.json` に追加します：

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Hook が承認すると、Claude Code は Plan Mode を終了し、Plan Mode に入る前にアクティブだった許可モードを復元します。トランスクリプトは、ダイアログが表示されたはずの場所に「Allowed by PermissionRequest hook」と表示されます。Hook パスは常に現在の会話を保持します：ダイアログができるように、コンテキストをクリアして新しい実装セッションを開始することはできません。

特定の許可モードを設定する代わりに、hook の出力に `setMode` エントリを含む `updatedPermissions` 配列を含めることができます。`mode` 値は `default`、`acceptEdits`、または `bypassPermissions` などの任意の許可モードであり、`destination: "session"` は現在のセッションのみに適用します。

<Note>
  `bypassPermissions` は、セッションが既にバイパスモードで起動された場合にのみ適用されます：`--dangerously-skip-permissions`、`--permission-mode bypassPermissions`、`--allow-dangerously-skip-permissions`、または [ユーザー、`--settings`、または管理設定](/docs/ja/settings-reference#permissions-defaultmode) の `permissions.defaultMode: "bypassPermissions"`。[`permissions.disableBypassPermissionsMode`](/docs/ja/permissions#managed-settings) で無効化されている場合、または [制限モード](/docs/ja/cli-reference#cli-flags) でセッションを開始した場合は適用されません。

  Claude Code は `defaultMode` として永続化することはありません。
</Note>

セッションを `acceptEdits` に切り替えるには、hook は stdout に次の JSON を書き込みます：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

マッチャーをできるだけ狭く保ちます。`.*` でマッチングするか、マッチャーを空のままにすると、ファイル書き込みやシェルコマンドを含むすべての許可プロンプトが自動承認されます。決定フィールドの完全なセットについては、[PermissionRequest リファレンス](/docs/ja/hooks#permissionrequest-decision-control) を参照してください。

<h2 id="how-hooks-work">
  hooks の仕組み
</h2>

Claude Code は、ライフサイクルの特定のポイントで hook イベントを発火させます。イベントが発火すると、Claude Code はすべてのマッチングする hooks を並列で実行します。重複するハンドラーの扱い方については、[Hook ハンドラーフィールド](/docs/ja/hooks#hook-handler-fields) を参照してください。以下の表は各イベントとそれがトリガーされるときを示しています：

| イベント                  | 発火するタイミング                                                                                                                                                     |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `SessionStart`        | セッションが開始または再開されたとき                                                                                                                                            |
| `Setup`               | `--init-only` で Claude Code を起動するとき、または `-p` モードで `--init` または `--maintenance` を使用するとき。CI またはスクリプトでの 1 回限りの準備用                                                |
| `UserPromptSubmit`    | プロンプトを送信するとき、Claude が処理する前                                                                                                                                    |
| `UserPromptExpansion` | ユーザーが入力したコマンドがプロンプトに展開されるとき、Claude に到達する前。展開をブロックできます                                                                                                         |
| `PreToolUse`          | ツール呼び出しが実行される前。ブロックできます                                                                                                                                       |
| `PermissionRequest`   | ツール呼び出しが権限決定を必要とするとき                                                                                                                                          |
| `PermissionDenied`    | オートモードがツール呼び出しを拒否するとき、分類器の判定がない拒否を含みます。JSON `hookSpecificOutput.retry: true` を使用して、モデルが拒否されたツール呼び出しを再試行できることを伝えます。Claude Code は分類器が判定を出さなかった場合、`retry` を無視します |
| `PostToolUse`         | ツール呼び出しが成功した後                                                                                                                                                 |
| `PostToolUseFailure`  | ツール呼び出しが失敗した後                                                                                                                                                 |
| `PostToolBatch`       | 並列ツール呼び出しの完全なバッチが解決した後、次のモデル呼び出しの前                                                                                                                            |
| `Notification`        | Claude Code が通知を送信するとき                                                                                                                                        |
| `MessageDisplay`      | アシスタントメッセージテキストが表示されている間                                                                                                                                      |
| `SubagentStart`       | サブエージェントがスポーンされるとき                                                                                                                                            |
| `SubagentStop`        | サブエージェントが終了するとき                                                                                                                                               |
| `TaskCreated`         | `TaskCreate` 経由でタスクが作成されるとき                                                                                                                                   |
| `TaskCompleted`       | タスクが完了としてマークされるとき                                                                                                                                             |
| `Stop`                | Claude が応答を終了するとき                                                                                                                                             |
| `StopFailure`         | API エラーが原因でターンが終了するとき                                                                                                                                         |
| `TeammateIdle`        | [エージェントチーム](/docs/ja/agent-teams) のチームメイトがアイドル状態になろうとするとき                                                                                                          |
| `InstructionsLoaded`  | CLAUDE.md または `.claude/rules/*.md` ファイルがコンテキストに読み込まれるとき。セッション開始時およびセッション中にファイルが遅延読み込みされるときに発火します                                                              |
| `ConfigChange`        | セッション中に設定ファイルが変更されるとき                                                                                                                                         |
| `CwdChanged`          | 作業ディレクトリが変更されるとき、例えば Claude が `cd` コマンドを実行するとき。direnv などのツールを使用したリアクティブな環境管理に便利です                                                                             |
| `DirectoryAdded`      | `/add-dir` または SDK `register_repo_root` コントロールリクエスト経由でセッション中盤に作業ディレクトリが追加されるとき                                                                                |
| `FileChanged`         | 監視対象ファイルがディスク上で変更されるとき。`matcher` フィールドは監視するファイル名を指定します                                                                                                        |
| `WorktreeCreate`      | `--worktree`、`isolation: "worktree"`、またはバックグラウンドセッション経由で worktree が作成されるとき。デフォルトの git 動作を置き換えます                                                               |
| `WorktreeRemove`      | セッション終了時、サブエージェント終了時、またはバックグラウンドセッションを削除するときに worktree が削除されるとき                                                                                               |
| `PreCompact`          | コンテキスト圧縮の前                                                                                                                                                    |
| `PostCompact`         | コンテキスト圧縮が完了した後                                                                                                                                                |
| `PreModelSwitch`      | Claude Code があなたまたはクライアントがリクエストしたモデルスイッチを適用する前。スイッチをブロックできます                                                                                                  |
| `PostModelSwitch`     | セッションのモデルが変更された後、Claude Code が独自に行う変更（セッションを再開するときのモデル復元など）を含みます                                                                                              |
| `Elicitation`         | MCP サーバーがツール呼び出し中にユーザー入力をリクエストするとき                                                                                                                            |
| `ElicitationResult`   | ユーザーが MCP エリシテーションに応答した後、レスポンスがサーバーに送り返される前                                                                                                                   |
| `SessionEnd`          | セッションが終了するとき                                                                                                                                                  |

各 hook には、それがどのように実行されるかを決定する `type` があります。ほとんどの hooks は `"type": "command"` を使用し、シェルコマンドを実行します。他の 4 つのタイプが利用可能です：

* `"type": "http"`：イベントデータを URL に POST します。[HTTP hooks](#http-hooks) を参照してください。
* `"type": "mcp_tool"`：既に接続されている MCP サーバー上のツールを呼び出します。[MCP tool hooks](/docs/ja/hooks#mcp-tool-hook-fields) を参照してください。
* `"type": "prompt"`：シングルターン LLM 評価。[プロンプトベースの hooks](#prompt-based-hooks) を参照してください。
* `"type": "agent"`：ツールアクセス付きマルチターン検証。エージェント hooks は実験的であり、変更される可能性があります。[エージェントベースの hooks](#agent-based-hooks) を参照してください。

<h3 id="combine-results-from-multiple-hooks">
  複数の hooks からの結果を組み合わせる
</h3>

複数の hooks が同じイベントにマッチする場合、すべての hook のコマンドが完了してから Claude Code は結果をマージします。1 つの hook が `deny` を返しても、兄弟 hooks の実行は停止されません。1 つの hook の `deny` が別の hook の副作用を抑制することに依存しないでください。

すべてのマッチングする hooks が完了した後、Claude Code はそれらの出力を組み合わせます。`PreToolUse` 許可決定については、最も制限的な答えが適用されます。順序は `deny`、`defer`、`ask`、`allow` です。`additionalContext` からのテキストはすべての hook から保持され、Claude に一緒に渡されます。

以下の例は `Bash` に 2 つの `PreToolUse` hooks を登録しています。最初のものはすべてのコマンドをログファイルに追加して 0 で終了します。2 番目のものはスクリプトを実行し、コマンドに `rm -rf` が含まれている場合は 2 で終了して拒否します：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Claude が `rm -rf /tmp/build` を実行しようとするとき、両方の hooks が並列で実行されます。ログ hook はコマンドを `~/.claude/bash.log` に書き込み、0 で終了します。これは決定がないことを報告します。ガードレール hook は 2 で終了し、ツール呼び出しを拒否します。deny が優先されるため、Claude Code はコマンドをブロックし、Claude にガードレールの stderr を表示します。ログエントリはまだ書き込まれます。なぜなら、ログ hook は既に実行されているからです。

<h3 id="read-input-and-return-output">
  入力を読み取り、出力を返す
</h3>

Hooks は stdin、stdout、stderr、および終了コードを通じて Claude Code と通信します。イベントが発火すると、Claude Code はイベント固有のデータを JSON としてスクリプトの stdin に渡します。スクリプトはそのデータを読み取り、作業を行い、終了コードを通じて Claude Code に次に何をするかを伝えます。

<h4 id="hook-input">
  Hook 入力
</h4>

すべてのイベントには `session_id`（セッションの一意の ID）や `cwd`（イベントが発火したときの作業ディレクトリ）などの共通フィールドが含まれていますが、各イベントタイプは異なるデータを追加します。Claude が Bash コマンドを実行するとき、`PreToolUse` hook は stdin で次のフィールドを受け取ります：

* `hook_event_name`：hook をトリガーしたイベント
* `tool_name`：Claude が使用しようとしているツール
* `tool_input`：Claude がツールに渡した引数。Bash の場合、その `command` フィールドはシェルコマンドを保持します。

たとえば、`npm test` コマンドの hook 入力は次のようになります：

```json theme={null}
{
  "session_id": "abc123",
  "cwd": "/Users/sarah/myproject",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

スクリプトはその JSON を解析し、これらのフィールドのいずれかに基づいて動作できます。`UserPromptSubmit` hooks は代わりに `prompt` テキストを取得し、`SessionStart` hooks は `source`（`startup`、`resume`、`clear`、`compact`、または `fork`）を取得するなど。リファレンスの [共通入力フィールド](/docs/ja/hooks#common-input-fields) で共有フィールドを参照し、各イベントのセクションでイベント固有のスキーマを参照してください。

<h4 id="hook-output">
  Hook 出力
</h4>

スクリプトは stdout または stderr に書き込み、特定のコードで終了することで、Claude Code に次に何をするかを伝えます。以下は、コマンドをブロックする `PreToolUse` hook の例です：

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr は Claude のフィードバックになります
  exit 2 # exit 2 = アクションをブロック
fi

exit 0  # exit 0 = 決定なし。通常の許可フローが適用されます
```

終了コードは次に何が起こるかを決定します：

* **終了 0**：hook は異議を報告しません。
  * `PreToolUse` hook の場合、これはツール呼び出しを承認しません：通常の [許可フロー](/docs/ja/permissions) が引き続き適用されます。
  * `UserPromptSubmit`、`UserPromptExpansion`、`SessionStart`、および `PostModelSwitch` hooks の場合、Claude Code は stdout を [プレーンテキストとして扱い](/docs/ja/hooks#exit-code-0) Claude のコンテキストに追加します。
* **終了 2**：Claude Code はアクションをブロックします。stderr に理由を書き込みます。それがどこに着地するかはイベントによって異なります：一部のイベントはそれを Claude にフィードバックとして供給するため、調整できます。他のイベントはそれをユーザーに表示し、`ConfigChange` や `Elicitation` などのいくつかはメッセージを表示しません。一部のイベントはブロックできません：`SessionStart` などの場合、終了 2 は stderr をユーザーに表示し、実行は続行されます。[イベントごとの終了コード 2 の動作](/docs/ja/hooks#exit-code-2-behavior-per-event) で完全なリストを参照してください。
* **その他の終了コード**：ほとんどのイベントでは、結果は hook が stdout に出力したものによって異なります：
  * スキーマ検証に合格した解析済みオブジェクト：Claude Code は終了コードを無視し、JSON だけが結果を決定し、hook はエラーとして報告されません。`WorktreeCreate` が任意の非ゼロ終了で失敗するなどの例外は、リファレンスの [終了コード出力](/docs/ja/hooks#exit-code-output) セクションにリストされています。
  * スキーマ検証に失敗した解析済みオブジェクト、または Claude Code が [JSON として解析しようとする](/docs/ja/hooks#exit-code-0) が有効な JSON ではない stdout：ブロックされないエラー。通知は検証またはパースメッセージを含みます。
  * Claude Code が [プレーンテキストとして扱う](/docs/ja/hooks#exit-code-0) stdout、または空の stdout：アクションはブロックされないエラーとして進行します。トランスクリプトは `<hook name> hook error` 通知を表示し、その後 stderr の最初の行が `Failed with non-blocking status code:` で始まります。完全な stderr をキャプチャするには、`claude --debug` で [デバッグログ](/docs/ja/hooks#debug-hooks) を有効にするか、セッション中に `/debug` を実行してください。

<h4 id="structured-json-output">
  構造化 JSON 出力
</h4>

終了コードはブロックするか沈黙するかのみを許可します。より多くの制御のために、終了 0 して stdout に JSON オブジェクトを出力します。

<Note>
  終了 2 で stderr メッセージでブロックするか、終了 0 で JSON で構造化制御を使用します。hook ごとに 1 つのアプローチを選択してください。混在させた場合の動作については、[終了コード出力](/docs/ja/hooks#exit-code-output) を参照してください。
</Note>

たとえば、`PreToolUse` hook はツール呼び出しを拒否して理由を Claude に伝えたり、ユーザーの承認のためにエスカレートしたりできます：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

`"deny"` を使用すると、Claude Code はツール呼び出しをキャンセルし、`permissionDecisionReason` を Claude にフィードバックとして返します。

`PreToolUse` では、Claude Code は各 `permissionDecision` 値を次のように処理します：

* `"allow"`：インタラクティブな許可プロンプトをスキップします。Deny および ask ルール（エンタープライズ管理 deny リストを含む）は引き続き適用されます。また、[`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool) とマークされた MCP ツールのプロンプトと、[組織が `ask` に設定した](/docs/ja/mcp#organization-controls-on-connector-tools) コネクタツールのプロンプトも適用されます。その設定が Claude Code に到達するセッションでも適用されます。
* `"deny"`：ツール呼び出しをキャンセルし、理由を Claude に送信します
* `"ask"`：通常どおりユーザーに許可プロンプトを表示します

4 番目の値 `"defer"` は、`-p` フラグ付きの [非インタラクティブモード](/docs/ja/headless) で利用可能です。プロセスを終了し、ツール呼び出しを保持して、Agent SDK ラッパーが入力を収集して再開できるようにします。リファレンスの [ツール呼び出しを後で延期する](/docs/ja/hooks#defer-a-tool-call-for-later) を参照してください。

`PreModelSwitch` hook は同じ `permissionDecision` フィールドを返します：`"allow"` はモデルスイッチを進行させ、`"deny"` はそれをキャンセルします。`"ask"` はインタラクティブセッションで `/model` を実行するときに確認を求めます。他の場所では、Claude Code は `"ask"` を拒否として扱います。[PreModelSwitch 決定制御](/docs/ja/hooks#premodelswitch-decision-control) を参照してください。

他のイベントは異なる決定パターンを使用します。たとえば、`PostToolUse` および `Stop` hooks はトップレベルの `decision: "block"` フィールドを使用し、`PermissionRequest` は `hookSpecificOutput.decision.behavior` を使用します。リファレンスの [サマリーテーブル](/docs/ja/hooks#decision-control) でイベント別の完全な内訳を参照してください。

`UserPromptSubmit` hooks の場合、代わりに `hookSpecificOutput.additionalContext` を使用して Claude のコンテキストにテキストを注入します。`additionalContext` を `hookSpecificOutput` の内側にネストします。JSON の最上位レベルに配置すると、Claude Code はそれを無視します。たとえば、この出力はすべてのプロンプトに現在のブランチ状態を追加します：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

ブロックプロンプトとセッションタイトルの設定を含む完全な出力形状については、[UserPromptSubmit 決定制御](/docs/ja/hooks#userpromptsubmit-decision-control) を参照してください。

`type: "prompt"` の Hooks は出力を異なる方法で処理します：[プロンプトベースの hooks](#prompt-based-hooks) を参照してください。

<h3 id="filter-hooks-with-matchers">
  マッチャーで hooks をフィルタリングする
</h3>

マッチャーなしでは、hook はそのイベントのすべての発生で発火します。マッチャーを使用すると、それを絞り込むことができます。たとえば、ファイル編集後にのみフォーマッターを実行したい場合（すべてのツール呼び出しの後ではなく）、`PostToolUse` hook にマッチャーを追加します：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

`"Edit|Write"` マッチャーは `Edit` または `Write` ツール呼び出しでのみ発火し、`Bash`、`Read`、または他のツールでは発火しません。カンマもまた同じ方法で代替を区切るため、`"Edit, Write"` は同等です。[マッチャーパターン](/docs/ja/hooks#matcher-patterns) を参照して、プレーン名と正規表現がどのように評価されるかを確認してください。

<Note>
  Claude はまた、シェルコマンドを実行することでファイルを作成または変更できます。コンプライアンススキャンまたは監査ログなど、hook がすべてのファイル変更を確認する必要がある場合は、ターンごとに 1 回作業ツリーをスキャンする [`Stop`](/docs/ja/hooks#stop) hook を追加してください。呼び出しごとのカバレッジの場合は、`Bash|PowerShell` もマッチさせ、スクリプトで `git status --porcelain` を使用して変更されたファイルと追跡されていないファイルをリストアップしてください。[PowerShell hook 入力セクション](/docs/ja/hooks#powershell) は、`Bash` だけをマッチさせるのが十分でない理由を説明しています。ディスク上の特定のファイルが変更されたときに hook を実行するには、それを書き込んだものが何であれ、[FileChanged](/docs/ja/hooks#filechanged) hook を使用してください。
</Note>

各イベントタイプは特定のフィールドでマッチします：

| イベント                                                                                                                                                   | マッチャーがフィルタリングするもの                                                  | マッチャー値の例                                                                                                                                                                                                                                                            |
| :----------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`、`PermissionDenied`                                                                 | ツール名                                                               | `Bash`、`Edit\|Write`、`mcp__.*`                                                                                                                                                                                                                                      |
| `SessionStart`                                                                                                                                         | セッションがどのように開始されたか                                                  | `startup`、`resume`、`clear`、`compact`、`fork`                                                                                                                                                                                                                         |
| `Setup`                                                                                                                                                | どの CLI フラグがセットアップをトリガーしたか                                          | `init`、`maintenance`                                                                                                                                                                                                                                                |
| `SessionEnd`                                                                                                                                           | セッションが終了した理由                                                       | `clear`、`resume`、`logout`、`prompt_input_exit`、`other`                                                                                                                                                                                                               |
| `Notification`                                                                                                                                         | 通知タイプ                                                              | `permission_prompt`、`idle_prompt`、`auth_success`、`elicitation_dialog`、`elicitation_url_dialog`、`elicitation_complete`、`elicitation_response`、`agent_needs_input`、`agent_completed`、`quota_auto_resume_fired`、`quota_auto_resume_stale`、`quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                        | エージェントタイプ                                                          | `general-purpose`、`Explore`、`Plan`、またはカスタムエージェント名                                                                                                                                                                                                                   |
| `PreCompact`、`PostCompact`                                                                                                                             | 圧縮をトリガーしたもの                                                        | `manual`、`auto`                                                                                                                                                                                                                                                     |
| `PreModelSwitch`、`PostModelSwitch`                                                                                                                     | セッションが切り替わるモデルの正規名（[PreModelSwitch](/docs/ja/hooks#premodelswitch) で説明） | `claude-opus-5`、`claude-opus-4-6\|claude-opus-5`、`.*opus.*`                                                                                                                                                                                                         |
| `SubagentStop`                                                                                                                                         | エージェントタイプ                                                          | `SubagentStart` と同じ値                                                                                                                                                                                                                                                |
| `ConfigChange`                                                                                                                                         | 設定ソース                                                              | `user_settings`、`project_settings`、`local_settings`、`policy_settings`、`skills`                                                                                                                                                                                      |
| `DirectoryAdded`                                                                                                                                       | ディレクトリがどのように追加されたか                                                 | `slash_command`、`register_repo_root`                                                                                                                                                                                                                                |
| `StopFailure`                                                                                                                                          | エラータイプ                                                             | `rate_limit`、`overloaded`、`authentication_failed`、`oauth_org_not_allowed`、`account_on_hold`、`billing_error`、`invalid_request`、`model_not_found`、`server_error`、`max_output_tokens`、`cloud_credential_error`、`unknown`                                               |
| `InstructionsLoaded`                                                                                                                                   | ロード理由                                                              | `session_start`、`nested_traversal`、`path_glob_match`、`include`、`compact`                                                                                                                                                                                            |
| `Elicitation`                                                                                                                                          | MCP サーバー名                                                          | 設定した MCP サーバー名                                                                                                                                                                                                                                                      |
| `ElicitationResult`                                                                                                                                    | MCP サーバー名                                                          | `Elicitation` と同じ値                                                                                                                                                                                                                                                  |
| `FileChanged`                                                                                                                                          | リテラルファイル名を監視（[FileChanged](/docs/ja/hooks#filechanged) を参照）             | `.envrc\|.env`                                                                                                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                  | コマンド名                                                              | スキルまたはコマンド名                                                                                                                                                                                                                                                         |
| `UserPromptSubmit`、`PostToolBatch`、`Stop`、`TeammateIdle`、`TaskCreated`、`TaskCompleted`、`WorktreeCreate`、`WorktreeRemove`、`CwdChanged`、`MessageDisplay` | マッチャーサポートなし                                                        | すべての発生で常に発火                                                                                                                                                                                                                                                         |

以下のタブは、異なるイベントタイプのいくつかのマッチャーを示しています。

<Tabs>
  <Tab title="すべての Bash コマンドをログに記録する">
    `Bash` ツール呼び出しのみをマッチし、各コマンドをファイルにログに記録します。`PostToolUse` イベントはコマンドが完了した後に発火するため、`tool_input.command` は実行されたものを含みます。Hook は stdin で JSON としてイベントデータを受け取り、`jq -r '.tool_input.command'` はコマンド文字列のみを抽出し、`>>` はログファイルに追加します：

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="MCP ツールをマッチさせる">
    MCP ツールは組み込みツールとは異なる命名規則を使用します：`mcp__<server>__<tool>`。ここで `<server>` は MCP サーバー名で、`<tool>` はそれが提供するツールです。たとえば、`mcp__github__search_repositories` または `mcp__filesystem__read_file`。[プラグインバンドルサーバー](/docs/ja/mcp#plugin-provided-mcp-servers) からのツールは、`mcp__plugin_my-plugin_db__query` などのスコープ付きサーバーセグメントを使用します。特定のサーバーからすべてのツールをターゲットするために正規表現マッチャーを使用するか、`mcp__.*__write.*` のようなパターンでサーバー全体でマッチします。リファレンスの [MCP ツールをマッチさせる](/docs/ja/hooks#match-mcp-tools) を参照して、完全な例のリストを確認してください。

    以下のコマンドは hook の JSON 入力からツール名を `jq` で抽出し、stderr に書き込みます。stderr に書き込むことで stdout をクリーンに保ち、メッセージを [デバッグログ](/docs/ja/hooks#debug-hooks) に送信します：

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="セッション終了時にクリーンアップする">
    `SessionEnd` イベントはセッションが終了した理由のマッチャーをサポートします。この hook は `clear`（`/clear` を実行するとき）でのみ発火し、通常の終了では発火しません：

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  `if` フィールドでツール名と引数でフィルタリングする
</h4>

`if` フィールドは [許可ルール構文](/docs/ja/permissions) を使用して、ツール名と引数の両方で hooks をフィルタリングするため、hook プロセスはツール呼び出しがマッチするときにのみ生成されます。これは `matcher` を超えており、ツール名のみでグループレベルでフィルタリングします。

たとえば、すべての Bash コマンドではなく、Claude が `git` コマンドを使用するときにのみ hook を実行するには：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Hook コマンドが実行されるかどうかは、`if` パターンの形状と Claude が呼び出している Bash コマンドによって異なります：

| `if` パターン          | Bash コマンド              | Hook は実行されるか？ | 理由                                                       |
| :----------------- | :--------------------- | :------------ | :------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | はい            | コマンド名がマッチします                                             |
| `Bash(git *)`      | `npm test && git push` | はい            | 各サブコマンドがチェックされます。`git push` がマッチします                      |
| `Bash(git *)`      | `echo $(git log)`      | はい            | `$()` とバッククォート内のコマンドがチェックされます。`git log` がマッチします          |
| `Bash(git *)`      | `echo $(date)`         | いいえ           | サブコマンドが `git *` にマッチしません                                 |
| `Bash(git push *)` | `echo $(date)`         | はい            | コマンド名以上を指定するパターンは、`$()`、バッククォート、または `$VAR` で hook を実行します |

Claude Code がどのコマンドが Bash 入力を実行するかを判断できない場合、パターンに関係なく hook を実行します。[Bash マッチングテーブル](/docs/ja/hooks#bash-if-matching) は、Claude Code がサブコマンドで絞り込むことができる、またはできないコマンド形状をカバーしています。フィルターはベストエフォートであるため、ハード allow または deny を強制するには、hook ではなく [許可システム](/docs/ja/permissions) を使用してください。

`if` フィールドは許可ルールと同じパターンを受け入れます：`"Bash(git *)"`、`"Edit(*.ts)"` など。複数のツール名をマッチさせるには、それぞれ独自の `if` 値を持つ別のハンドラーを使用するか、パイプ交替がサポートされている `matcher` レベルでマッチします。

`if` はツールイベントでのみ機能します：`PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`、および `PermissionDenied`。他のイベントに追加すると、hook が実行されるのを防ぎます。

<h3 id="configure-hook-location">
  hook の場所を設定する
</h3>

Hook を追加する場所がそのスコープを決定します：

| 場所                                                | スコープ                                                                                      | 共有可能               |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------- | :----------------- |
| `~/.claude/settings.json`                         | すべてのプロジェクト                                                                                | いいえ、マシンにローカル       |
| `.claude/settings.json`                           | 単一プロジェクト                                                                                  | はい、リポジトリにコミット可能    |
| `.claude/settings.local.json`                     | 単一プロジェクト                                                                                  | いいえ、gitignored     |
| 管理ポリシー設定                                          | 組織全体                                                                                      | はい、管理者制御           |
| [Plugin](/docs/ja/plugins/overview) `hooks/hooks.json` | プラグインが有効なとき                                                                               | はい、プラグインにバンドル      |
| [Skill](/docs/ja/skills) frontmatter                   | スキルが呼び出されたら、セッションの残り。[スキルとエージェントの Hooks](/docs/ja/hooks#hooks-in-skills-and-agents) を参照してください。 | はい、スキルファイルで定義      |
| [Subagent](/docs/ja/sub-agents) frontmatter            | そのサブエージェントが実行されている間                                                                       | はい、サブエージェントファイルで定義 |

Claude Code で [`/hooks`](/docs/ja/hooks#the-%2Fhooks-menu) を実行して、イベント別にグループ化されたすべての設定済み hooks を参照します。

hooks を無効にするには、設定ファイルで `"disableAllHooks": true` を設定します。Claude Code は [設定の優先順位](/docs/ja/hooks#disable-or-remove-hooks) の後に残された値を読み取るため、プロジェクトの設定ファイルはあなたのものをオーバーライドできます。管理設定で設定された Hooks は、`disableAllHooks` がそこにも設定されていない限り、実行されます。各レベルの完全な範囲については、[`disableAllHooks`](/docs/ja/settings-reference#disableallhooks) を参照してください。

Claude Code が実行中に設定ファイルを直接編集する場合、ファイルウォッチャーは通常、hook の変更を自動的に取得します。

<h2 id="prompt-based-hooks">
  プロンプトベースの hooks
</h2>

決定論的なルールではなく判断が必要な決定については、`type: "prompt"` hooks を使用します。シェルコマンドを実行する代わりに、Claude Code はプロンプトと hook の入力データを Claude モデル（デフォルトでは Haiku）に送信して決定を下します。より多くの機能が必要な場合は、`model` フィールドで異なるモデルを指定できます。

モデルの唯一の仕事は、その決定を JSON として返すことです：

* `"ok": true`：アクションが続行されます
* `"ok": false`：何が起こるかはイベントによって異なります：
  * `Stop` および `SubagentStop`：`reason` は Claude にフィードバックとして返されるため、作業を続けます。ただし、レスポンスが `"impossible": true` も設定している場合は、その条件が決して満たされることのない条件として印付けられ、その場合 Claude Code は停止を許可してターンが終了します
  * `PreToolUse`：ツール呼び出しが拒否されます。デフォルトではターンが終了し、拒否の `reason` は警告行としてチャットに表示されます。hook に `continueOnBlock: true` を設定すると、代わりに `reason` を Claude にツールエラーとして返すため、調整して続行できます。v2.1.210 より前では、拒否の `reason` は Claude にツールエラーとして返され、ターンが続行されていました
  * `PostToolUse`：デフォルトではターンが終了し、`reason` は警告行としてチャットに表示されます。`continueOnBlock: true` を設定すると、`reason` を Claude にフィードバックとして返し、代わりにターンを続行します
  * `PostToolBatch`、`UserPromptSubmit`、および `UserPromptExpansion`：ターンが終了し、`reason` は警告行としてチャットに表示されます

この例は `Stop` hook を使用して、要求されたすべてのタスクが完了しているかどうかをモデルに尋ねます。モデルが条件がまだ満たされていないため `"ok": false` を返す場合、Claude は作業を続け、`reason` を次の指示として使用します：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

完全な設定オプションについては、リファレンスの [プロンプトベースの hooks](/docs/ja/hooks#prompt-based-hooks) を参照してください。

<h2 id="agent-based-hooks">
  エージェントベースの hooks
</h2>

<Warning>
  エージェント hooks は実験的です。動作と設定は将来のリリースで変更される可能性があります。本番ワークフローについては、[コマンド hooks](/docs/ja/hooks#command-hook-fields) を優先してください。
</Warning>

検証がファイルの検査またはコマンドの実行を必要とする場合、`type: "agent"` hooks を使用します。プロンプト hooks は単一の LLM 呼び出しを行いますが、エージェント hooks は条件を返す前にファイルを読み取り、コードを検索し、他のツールを使用できる subagent を生成します。

エージェント hooks は `"ok"` / `"reason"` 応答形式を使用し、デフォルトのタイムアウトが 60 秒で、最大 50 ツール使用ターンです。プロンプト hooks の `impossible` フィールドはサポートされていません。`ok: false` の場合、Claude Code はエージェント hooks を同じイベント上で `continueOnBlock: true` を持つプロンプト hooks と同じ方法で処理するため、`PreToolUse` と `PostToolUse` ではターンが続行されます。エージェント hooks には `continueOnBlock` フィールドがありません。フィールドを含む [エージェント hooks の設定](/docs/ja/hooks#agent-hook-configuration) を参照してください。これには Claude Code が hooks の JSON 入力に置き換える `$ARGUMENTS` プレースホルダーが含まれます。

この例は Claude が停止することを許可する前にテストが合格することを検証します：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

Hook 入力データだけで決定を下すのに十分な場合はプロンプト hooks を使用します。コードベースの実際の状態に対して何かを検証する必要がある場合はエージェント hooks を使用します。

完全な設定オプションについては、リファレンスの [エージェントベースの hooks](/docs/ja/hooks#agent-based-hooks) を参照してください。

<h2 id="http-hooks">
  HTTP hooks
</h2>

`type: "http"` hooks を使用して、シェルコマンドを実行する代わりに、イベントデータを HTTP エンドポイントに POST します。エンドポイントはコマンド hook が stdin で受け取るのと同じ JSON を受け取り、HTTP レスポンスボディを使用して同じ JSON 形式で結果を返します。

HTTP hooks は、Web サーバー、クラウド関数、または外部サービスに hook ロジックを処理させたい場合に便利です。たとえば、チーム全体のツール使用イベントをログに記録する共有監査サービス。

この例はすべてのツール使用をローカルログサービスに POST します：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

エンドポイントは、コマンド hooks と同じ [出力形式](/docs/ja/hooks#json-output) を使用して JSON レスポンスボディを返す必要があります。ツール呼び出しをブロックするには、適切な `hookSpecificOutput` フィールドで 2xx レスポンスを返します。HTTP ステータスコードだけではアクションをブロックできません。

ヘッダー値は `$VAR_NAME` または `${VAR_NAME}` 構文を使用した環境変数補間をサポートします。`allowedEnvVars` 配列にリストされている変数のみが解決されます。他のすべての `$VAR` 参照は空のままです。

完全な設定オプションとレスポンス処理については、リファレンスの [HTTP hooks](/docs/ja/hooks#http-hook-fields) を参照してください。

<h2 id="limitations-and-troubleshooting">
  制限とトラブルシューティング
</h2>

<h3 id="limitations">
  制限
</h3>

hooks を設計する際は、以下の制約を念頭に置いてください：

* コマンド hooks は stdout、stderr、および終了コードを通じてのみ通信します。これらは `/` コマンドまたはツール呼び出しをトリガーできません。`additionalContext` を通じて返されたテキストは、Claude が平文として読むシステムリマインダーとして注入されます。HTTP hooks はレスポンスボディを通じて通信します。
* Hook タイムアウトはタイプによって異なります。`timeout` フィールド（秒単位）で hook ごとにオーバーライドできます。
  * `command`、`http`、`mcp_tool`：10 分。Claude Code は `UserPromptSubmit`、`PreModelSwitch`、および `PostModelSwitch` hooks のこのデフォルトを 30 秒に短縮し、`MessageDisplay` を 10 秒に短縮します。
  * `prompt`：30 秒。
  * `agent`：60 秒。
  * [`SessionEnd`](/docs/ja/hooks#sessionend) hooks はすべてのタイプで 1.5 秒の予算を共有します。設定で hook ごとの `timeout` がより長い場合、Claude Code は予算を引き上げて一致させ、最大 60 秒までです。
* `PostToolUse` hooks はツールが既に実行されているため、アクションを元に戻すことはできません。
* `PermissionRequest` hooks は Claude Code があなたに許可を求めようとしているときに発火します。
  * [非インタラクティブモード](/docs/ja/headless)（`-p` フラグ）では、そのプロンプトは Agent SDK の [`canUseTool` コールバック](/docs/ja/agent-sdk/permissions)がそれを提供する場合にのみ存在します。プレーンな `-p` 実行または `--permission-prompt-tool` では、自動化された許可決定に代わりに `PreToolUse` hooks を使用します。
  * バックグラウンド subagents は非インタラクティブモードでプロンプトを表示できません。Claude Code は依然としてそれらのツール呼び出しの hooks を実行し、hook が決定を返さない場合は呼び出しを拒否します。インタラクティブセッションでは、バックグラウンド subagent プロンプトはメインセッションに表示され、hooks は通常通り発火します。
* `Stop` hooks はタスク完了時だけでなく、Claude が応答を終了するたびに発火します。ユーザーの割り込みでは発火しません。API エラーは代わりに [StopFailure](/docs/ja/hooks#stopfailure) を発火させます。
* 複数の `PreToolUse` hooks が [`updatedInput`](/docs/ja/hooks#pretooluse) を返してツールの引数を書き直す場合、最後に完了したものが勝ちます。Hooks は並列で実行されるため、順序は非決定的です。同じツールの入力を変更する複数の hooks を持つことを避けてください。

<h3 id="hooks-and-permission-modes">
  Hooks と許可モード
</h3>

`PreToolUse` hooks は任意の権限モードチェックの前に発火します。すべての [権限モード](/docs/ja/permission-modes)（`dontAsk` を含む）で発火します。`permissionDecision: "deny"` を返す hook は、`bypassPermissions` モードまたは `--dangerously-skip-permissions` でもツールをブロックします。これにより、ユーザーが権限モードを変更してバイパスできないポリシーを適用できます。

逆は真ではありません：`"allow"` を返す hook は、設定からの deny ルールをバイパスしません。また、[`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool) とマークされた MCP ツールのプロンプトを抑制することもできず、組織が [セッションでそのセッティングに到達する Claude Code で `ask` に設定した](/docs/ja/mcp#organization-controls-on-connector-tools)コネクタツールも抑制できません。Hooks は制限を厳しくできますが、許可ルールが許可する範囲を超えて緩和することはできません。

<h3 id="hook-not-firing">
  Hook が発火しない
</h3>

Hook は設定されていますが、実行されません。

* `/hooks` を実行し、hook が正しいイベントの下に表示されることを確認します
* マッチャーパターンがツール名と正確にマッチすることを確認します。マッチャーは大文字小文字を区別します
* 正しいイベントタイプをトリガーしていることを確認します：`PreToolUse` はツール実行前に発火し、`PostToolUse` は後に発火します。`PermissionRequest` hook は Claude Code があなたに許可を求めようとしているときに発火します。非インタラクティブケースについては [制限](#limitations)を参照してください

<h3 id="hook-error-in-output">
  Hook エラーが出力に表示される
</h3>

トランスクリプトに「PreToolUse hook error: ...」というメッセージが表示されます。

* スクリプトが予期せずゼロ以外のコードで終了しました。サンプル JSON をパイプして手動でテストします：
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # 終了コードを確認
  ```
* 「command not found」が表示される場合は、絶対パスを使用するか、スクリプトを参照するために `${CLAUDE_PROJECT_DIR}` を使用します。シェルクォーティングを完全に回避するには、`"args": []` を追加して [exec form](/docs/ja/hooks#exec-form-and-shell-form) に切り替えます。これはシェルなしでスクリプトを直接生成します
* 「jq: command not found」が表示される場合は、`jq` をインストールするか、JSON 解析に Python/Node.js を使用します
* 通知が JSON 検証メッセージを表示する場合、hook の stdout は JSON として解析されましたがスキーマ検証に失敗しました。JSON 解析メッセージを表示する場合、stdout は JSON オブジェクトのように見えましたが有効な JSON ではありませんでした。どちらも終了コード 0 でも発生します。

  解析失敗を修正するには、文字列連結の代わりに `jq` などの JSON エンコーダーでペイロードを構築して、値内の引用符とバックスラッシュがエスケープされるようにします。リファレンスの [終了コード出力](/docs/ja/hooks#exit-code-output)セクションは終了コードと JSON の組み合わせをカバーしています
* スクリプトがまったく実行されていない場合は、実行可能にします：`chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` に設定された hooks が表示されない
</h3>

設定ファイルを編集しましたが、hooks がメニューに表示されません。

* ファイル編集は通常自動的に取得されます。数秒後に表示されていない場合、ファイルウォッチャーが変更を見逃した可能性があります：セッションを再開して強制的にリロードします。
* JSON が有効であることを確認します：末尾のコンマとコメントは許可されていません
* 設定ファイルが正しい場所にあることを確認します：プロジェクト hooks の場合は `.claude/settings.json`、グローバル hooks の場合は `~/.claude/settings.json`

<h3 id="stop-hook-hits-the-block-cap">
  Stop hook がブロック上限に達する
</h3>

Claude は無限ループで作業を続け、停止する代わりに、Stop hook が連続して 8 回ブロックしたという警告でターンを終了します。

Claude Code は Stop hook が進捗なしで 8 回連続でブロックした後、それをオーバーライドします。Hook スクリプトは、それが既にトリガーされたかどうかをチェックする必要があります。JSON 入力から `stop_hook_active` フィールドを解析し、`true` の場合は早期に終了します：

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Claude が停止することを許可
fi
# ... hook ロジックの残り
```

Hook が収束するために 8 回以上の反復が正当に必要な場合は、[`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/ja/env-vars) で上限を引き上げます。

<h3 id="hook-json-has-no-effect">
  Hook JSON に効果がない
</h3>

Hook は有効な JSON を出力していますが、決定が有効にならず、トランスクリプトにエラーが表示されません。どの原因が当てはまるかを確認します：

* **JSON の前の追加出力**：通常、シェルプロファイルの無条件の `echo` により、何か他のものが最初に stdout に書き込まれるため、出力はもはや `{` で始まらず、Claude Code はそれを JSON として解析しません。原因と修正は以下のリストに従います。
* **フィールドが間違ったレベルにある**：各フィールドの配置を [JSON 出力](/docs/ja/hooks#json-output)形式と比較します。例えば、`permissionDecision` はトップレベルではなく `hookSpecificOutput` の内部に属します。

Claude Code が shell form コマンド hook（`args` なし）を実行する場合、macOS と Linux では `sh -c` を、Windows では Git Bash を、Git Bash がデフォルトでインストールされていない場合は PowerShell を生成します。このシェルは非インタラクティブですが、Git Bash と一部の設定（`BASH_ENV` が `~/.bashrc` を指すなど）は依然としてプロファイルをソースします。そのプロファイルに無条件の `echo` ステートメントが含まれている場合、その出力は hook の JSON に前置されます：

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

結合された出力はもはや `{` で始まらないため、Claude Code は stdout 全体をプレーンテキストとして扱い、JSON を無視します。終了コード 0 ではトランスクリプトに何も報告されません。解析試行は [デバッグログ](/docs/ja/hooks#debug-hooks)にのみ記録されます。これを修正するには、シェルプロファイルの echo ステートメントをラップして、インタラクティブシェルでのみ実行するようにします：

```bash theme={null}
# ~/.zshrc または ~/.bashrc 内
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

`$-` 変数はシェルフラグを含み、`i` はインタラクティブを意味します。Hooks は非インタラクティブシェルで実行されるため、echo はスキップされます。

Hook が `permissionDecision` または `additionalContext` を `hookSpecificOutput` の内部ではなくトップレベルに返す場合、JSON は依然として解析され、Claude Code は誤配置されたフィールドを報告なしで無視します。どのフィールドが無視されたかを確認するには、`claude --debug` で Claude Code を開始し、[デバッグログ](/docs/ja/hooks#debug-hooks)で `Hook JSON output had unrecognized keys` を検索します。

<h3 id="debug-techniques">
  デバッグ技術
</h3>

`Ctrl+O` を押してトランスクリプトビューを開き、hook 実行の結果を確認します：

* **成功した実行**：hook の JSON が `systemMessage` や Stop hook フィードバックなどのサーフェスを表示しない限り、何も表示されません。
  * Hook が実行されたことを確認するには、再フォーマットされたファイルなどの効果をチェックするか、以下で説明されているようにデバッグログを有効にして hook を再度トリガーします
* **ブロッキングエラー**：ほとんどのイベントでは hook のフィードバックが表示されます。Hook の JSON がブロッキング決定を下した場合、フィードバックはその決定からの理由です。そうでない場合は hook の stderr です。`ConfigChange` や `Elicitation` などのいくつかのイベントでは、ブロックはメッセージを表示しません。
* **非ブロッキングエラー**：アクションが進行し、`<hook name> hook error` 通知が短い説明とともに表示されます。例えば stderr の最初の行に「Failed with non-blocking status code:」というプレフィックスが付いているか、JSON 検証またはパースメッセージです。

どの終了コードと JSON の組み合わせが各結果を生成するか、イベントごとの例外を含めて、リファレンスの [終了コード出力](/docs/ja/hooks#exit-code-output)セクションで定義されています。

完全な実行詳細（どの hooks がマッチしたか、それらの終了コード、stdout、stderr など）については、デバッグログを読みます。`claude --debug-file /tmp/claude.log` で Claude Code を開始して既知のパスに書き込み、別のターミナルで `tail -f /tmp/claude.log` を実行します。そのフラグなしで開始した場合は、セッション中に `/debug` を実行してログを有効にし、ログパスを見つけます。

<h2 id="learn-more">
  詳細を学ぶ
</h2>

* [Hooks リファレンス](/docs/ja/hooks)：完全なイベントスキーマ、JSON 出力形式、非同期 hooks、および MCP ツール hooks
* [セキュリティに関する考慮事項](/docs/ja/hooks#security-considerations)：共有または本番環境に hooks をデプロイする前に確認してください
* [Bash コマンドバリデーター例](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py)：完全なリファレンス実装
