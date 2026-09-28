> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 任意のデバイスからローカルセッションを続行する Remote Control

> Remote Control を使用して、電話、タブレット、または任意のブラウザから Claude Code のローカルセッションを続行します。claude.ai/code と Claude モバイルアプリで動作します。

<Note>
  Remote Control はすべてのプランで利用可能です。Team および Enterprise では、Owner が [Claude Code 管理設定](https://claude.ai/admin-settings/claude-code) で Remote Control トグルを有効にするまで、デフォルトではオフになっています。
</Note>

Remote Control は [claude.ai/code](https://claude.ai/code) または Claude アプリ（[iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) および [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)）をマシン上で実行されている Claude Code セッションに接続します。デスクでタスクを開始してから、ソファの電話またはコンピュータのブラウザで続行できます。

マシン上で Remote Control セッションを開始すると、Claude はローカルで実行され続けるため、コード実行とファイルシステムアクセスはマシン上に留まります。Remote Control を使用すると、以下のことができます。

* **ローカル環境全体をリモートで使用する**: ファイルシステム、[MCP サーバー](/docs/ja/mcp)、ツール、プロジェクト設定がすべて利用可能なままです。また、`@` を入力するとローカルプロジェクトのファイルパスが自動補完されます。
* **両方のサーフェスから同時に作業する**: 会話と [subagents](/docs/ja/sub-agents) および [dynamic workflows](/docs/ja/workflows) の進捗がすべての接続されたデバイス間で同期されるため、ターミナル、ブラウザ、電話から相互に交換可能にメッセージを送信できます。
* **電話またはブラウザから画像とファイルを送信する**: Claude アプリまたは claude.ai/code に写真またはファイルを添付できます。キャプション付きまたはキャプションなしで添付できます。Claude は添付された写真をメッセージの一部として直接見ることができます。Claude Code は他のファイルをマシンにダウンロードし、`@` ファイル参照として Claude に渡します。
* **中断に対応する**: ラップトップがスリープ状態になったり、ネットワークが切断されたりした場合、マシンがオンラインに戻ると、Claude Code は自動的に再接続されます。接続が再構築されている間、Claude Code はメッセージ、権限プロンプト、および subagents とワークフローからのステータス更新をキューに入れ、接続が復旧した後に配信します。

クラウドインフラストラクチャで実行される [Web 上の Claude Code](/docs/ja/claude-code-on-the-web) とは異なり、Remote Control セッションはマシン上で直接実行され、ローカルファイルシステムと相互作用します。Web およびモバイルインターフェースは、そのローカルセッションへのウィンドウにすぎません。

このページでは、セットアップ、セッションの開始と接続方法、および Remote Control と Web 上の Claude Code の比較について説明します。

<h2 id="requirements">
  要件
</h2>

Remote Control を使用する前に、環境が以下の条件を満たしていることを確認してください。

* **サブスクリプション**: Pro、Max、Team、および Enterprise プランで利用可能です。API キーはサポートされていません。Team および Enterprise では、Owner が [Claude Code 管理設定](https://claude.ai/admin-settings/claude-code) で Remote Control トグルを最初に有効にする必要があります。
* **認証**: `claude` を実行し、まだサインインしていない場合は `/login` を使用して claude.ai 経由でサインインします。適格なログインがない場合、`claude remote-control` はエラーで終了しますが、`claude --remote-control` は対話型セッションを開始し、起動直後に Remote Control 失敗通知を表示します。
* **API エンドポイント**: 以下のいずれかの構成では利用できません。
  * Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry を使用している。
  * [`ANTHROPIC_BASE_URL`](/docs/ja/env-vars) が `api.anthropic.com` 以外のホスト（[LLM gateway](/docs/ja/llm-gateway) やプロキシなど）を指している。Remote Control を使用するには、この変数を設定解除してください。v2.1.196 より前では、Claude Code はカスタム `ANTHROPIC_BASE_URL` で Remote Control を許可していました。
  * エンタープライズ [Claude apps gateway](/docs/ja/claude-apps-gateway) 経由でサインインしている。
* **機能フラグ評価**: [`DISABLE_TELEMETRY`、`DO_NOT_TRACK`、`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`、および `DISABLE_GROWTHBOOK`](/docs/ja/env-vars) はそれぞれ、Remote Control の可用性が依存する機能フラグ評価を無効にします。Remote Control を使用するには、シェル環境または [`settings.json` ファイル](/docs/ja/settings-reference#all-settings) の `env` ブロックのいずれかで変数が設定されている場所で変数を設定解除してください。
* **ワークスペース信頼**: プロジェクトディレクトリで少なくとも 1 回 `claude` を実行して、ワークスペース信頼ダイアログを受け入れます。スタートアップ信頼ダイアログはホームディレクトリの信頼を保存しないため、プロジェクトディレクトリから Remote Control を起動してください。

<h2 id="start-a-remote-control-session">
  Remote Control セッションを開始する
</h2>

CLI または VS Code 拡張機能から Remote Control セッションを開始できます。CLI は 3 つの呼び出しモードを提供します。VS Code は `/remote-control` コマンドを使用します。

<Tabs>
  <Tab title="サーバーモード">
    プロジェクトディレクトリで、以下を実行します。

    ```bash theme={null}
    claude remote-control
    ```

    Remote Control の一度限りの確認を受け入れるまで、`claude remote-control` は何をするかを説明し、サーバーを開始する前に `Enable Remote Control? (y/n)` と尋ねます。`y` と答えて受け入れ、サーバーを開始します。拒否した場合、Claude Code はサーバーを開始せずに終了し、次回コマンドを実行するときに再度尋ねます。

    プロセスはサーバーモードでターミナルで実行され続け、リモート接続を待機します。[別のデバイスから接続](#connect-from-another-device)するために使用できるセッション URL が表示され、スペースバーを押して電話からの高速アクセス用の QR コードを表示できます。リモートセッションがアクティブな間、ターミナルは接続ステータスとツールアクティビティを表示します。

    利用可能なフラグ:

    | フラグ                                             | 説明                                                                                                                                                                                                                                                                                                                                     |
    | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | claude.ai/code のセッションリストに表示されるカスタムセッションタイトルを設定します。                                                                                                                                                                                                                                                                                     |
    | `--remote-control-session-name-prefix <prefix>` | 明示的な名前が設定されていない場合の自動生成セッション名のプレフィックス。デフォルトはマシンのホスト名で、`myhost-graceful-unicorn` のような名前が生成されます。同じ効果のために `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` を設定します。                                                                                                                                                                              |
    | `-c`, `--continue`                              | このディレクトリから開始された最後のサーバーが開始したセッションを復帰させます。新しいセッションを作成する代わりに使用します。[サーバーを停止した後のセッションの再開](#resume-sessions-after-stopping-the-server)を参照してください。`--session-id`、`--spawn`、`--capacity`、または `--create-session-in-dir` と組み合わせることはできません。Claude Code v2.1.200 以降が必要です。それより前のバージョンはこのフラグを未知の引数として拒否します。                                            |
    | `--session-id <id>`                             | 特定のセッションをその ID で復帰させます。[サーバーを停止した後のセッションの再開](#resume-sessions-after-stopping-the-server)を参照してください。`--continue`、`--spawn`、`--capacity`、または `--create-session-in-dir` と組み合わせることはできません。Claude Code v2.1.200 以降が必要です。それより前のバージョンはこのフラグを未知の引数として拒否します。                                                                                      |
    | `--spawn <mode>`                                | サーバーがセッションを作成する方法。<br />• `same-dir`（デフォルト）: すべてのセッションが現在の作業ディレクトリを共有するため、同じファイルを編集している場合は競合する可能性があります。<br />• `worktree`: オンデマンドセッションごとに独自の [git worktree](/docs/ja/worktrees) を取得します。git リポジトリが必要です。<br />• `session`: シングルセッションモード。正確に 1 つのセッションを提供し、追加の接続を拒否します。スタートアップ時にのみ設定します。<br />実行時に `w` を押して `same-dir` と `worktree` の間でトグルします。 |
    | `--capacity <N>`                                | 同時セッションの最大数。デフォルトは 32 です。`--spawn=session` では使用できません。                                                                                                                                                                                                                                                                                  |
    | `--[no-]create-session-in-dir`                  | サーバーの起動時に現在のディレクトリに 1 つのセッションを事前作成し、すぐに入力できる場所を用意します。`worktree` モードでは、このセッションは現在のディレクトリに留まり、オンデマンドセッションは分離された worktree を取得します。デフォルトではオンです。`--no-create-session-in-dir` を渡して、何も作成しない状態で開始する場合、Claude Code はサーバーのセッションをアーカイブするため、[復帰](#resume-sessions-after-stopping-the-server)するものがありません。                                             |
    | `--permission-mode <mode>`                      | サーバーのセッションの開始時の[権限モード](/docs/ja/permission-modes)を設定します。例えば `acceptEdits` など。`manual` を `default` のエイリアスとして受け入れます。認識されないモードはサーバーをスタートアップで停止し、有効なモードをリストします。                                                                                                                                                                                 |
    | `--debug-file <path>`                           | デバッグログを指定されたファイルに書き込みます。                                                                                                                                                                                                                                                                                                               |
    | `--verbose`                                     | 詳細な接続とセッションログを表示します。                                                                                                                                                                                                                                                                                                                   |
    | `--sandbox` / `--no-sandbox`                    | ファイルシステムとネットワーク分離のための[サンドボックス](/docs/ja/sandboxing)を有効または無効にします。デフォルトではオフです。                                                                                                                                                                                                                                                                |

    これらのフラグは `remote-control` の後に指定します。

    `remote-control` の前にグローバル `claude` フラグを渡すか、ラッパースクリプトが 1 つを追加する場合、Claude Code はそのフラグをサーバーが作成するセッションに引き継ぎません。Claude Code は `--verbose` や `--model` など、それらのセッションが実行できることを変更しないことが既知のフラグのみを許可します。他のフラグ（例えば `--settings`）の場合、Claude Code は[開始を拒否](/docs/ja/errors#not-carried-over-to-the-sessions-remote-control-starts)し、削除するフラグを名前で指定します。v2.1.248 より前では、`remote-control` の前のオプションは Claude Code が後のフラグを `unknown option` エラーで拒否させていました。

    Claude Code はヘルプを出力する前に Remote Control の適格性をチェックするため、適格なアカウントでサインインしていない場合、`claude remote-control --help` はこのフラグリストの代わりにエラーを返します。
  </Tab>

  <Tab title="対話型セッション">
    Remote Control を有効にした通常の対話型 Claude Code セッションを開始するには、`--remote-control` フラグ（または `--rc`）を使用します。

    ```bash theme={null}
    claude --remote-control
    ```

    オプションでセッションの名前を渡します。

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    これにより、ターミナルで完全な対話型セッションが得られ、claude.ai または Claude アプリからも制御できます。`claude remote-control`（サーバーモード）とは異なり、セッションがリモートで利用可能な間、ローカルでメッセージを入力できます。
  </Tab>

  <Tab title="既存のセッションから">
    既に Claude Code セッションにいて、それをリモートで続行したい場合は、`/remote-control`（または `/rc`）コマンドを使用します。

    ```text theme={null}
    /remote-control
    ```

    カスタムセッションタイトルを設定するために、引数として名前を渡します。

    ```text theme={null}
    /remote-control My Project
    ```

    これにより、現在の会話履歴を引き継ぎ、Remote Control セッションが開始されます。

    Remote Control の一度限りの確認を受け入れるまで、`/remote-control` が接続する前にダイアログが表示されます。**Enable Remote Control** を選択して受け入れて接続します。**Never mind** を選択するか Esc を押した場合、Claude Code は接続せず、次回 `/remote-control` を実行するときに再度尋ねます。

    `--verbose`、`--sandbox`、および `--no-sandbox` フラグはこのコマンドでは利用できません。
  </Tab>

  <Tab title="VS Code">
    [Claude Code VS Code 拡張機能](/docs/ja/vs-code)で、プロンプトボックスに `/remote-control` または `/rc` を入力します。

    ```text theme={null}
    /remote-control
    ```

    Remote Control がオンの間、Claude Code はプロンプトボックスのフッターに **Remote Control** インジケーターを表示します。セッションが接続されたら、インジケーターをクリックしてセッションに直接移動するか、[claude.ai/code](https://claude.ai/code)のセッションリストで見つけます。Claude Code はセッション URL を会話にも投稿します。切断するには、`/remote-control` を再度実行します。

    CLI とは異なり、VS Code コマンドは名前引数を受け入れず、QR コードを表示しません。セッションタイトルは会話履歴または最初のプロンプトから派生します。
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  接続ステータスを確認する
</h3>

対話型セッションでは、Remote Control が接続されている間、ターミナルは `/rc active` インジケーターを表示し、claude.ai のセッションへのリンクになります。ターミナルが狭すぎる場合、インジケーターは非表示になります。セッション URL と [別のデバイスから接続](#connect-from-another-device)するために使用できる QR コードを表示するには、`/remote-control` を再度実行してステータスパネルを開きます。パネルは切断オプションも提供し、これにより Remote Control をオフにしながらローカルセッションはターミナルで実行され続けます。

<span id="session-ended-elsewhere" />対話型セッションで接続に失敗した場合、インジケーターは失敗を表示するように変わり、Claude Code は理由を通知で表示し、会話に追加します。`/remote-control` を実行して再接続します。ただし、理由がセッションが別の場所で引き継がれたか終了したか、またはサーバーがそれを見つけられないと言っている場合を除きます。

* **別の接続がこのセッションを引き継いだ**: 別のデバイスまたは Claude Code セッションがそれを持っています。セッションを取り戻したい場合のみ `/remote-control` を実行します。
* **このセッションは別のデバイスまたはアプリから終了またはアーカイブされました**: セッションを戻したい場合のみ `/remote-control` を実行します。Claude Code はアーカイブされたセッションを再度開きます。
* **サーバーはこのセッションを見つけられなくなりました**: 別のデバイスまたはアプリから削除されている可能性があります。

<h3 id="session-url-reminders">
  セッション URL リマインダー
</h3>

Remote Control が接続されている間、Claude Code は電話またはブラウザに切り替えるのが最も役立つときにセッション URL を思い出させるため、リンクを `/remote-control` で見つける必要がありません。リマインダーは次のいずれかの瞬間にプロンプトボックスの上に表示されます。

* **長いターン**: ターンがサーバーチューニングされたしきい値より長く実行される場合、Claude Code は **Still working** 通知と **Check in from your phone** リンクを表示し、ターミナルで待つ代わりに電話またはブラウザからターンをフォローできます。Claude Code はターンが終了するとそれを削除します。
* **繰り返される権限プロンプト**: セッションで複数の[権限プロンプト](/docs/ja/permissions)に答えた後、**Approve tool calls from your phone** 通知がセッション URL を表示します。Claude Code は次のターンが開始されるとそれを削除します。

リマインダーは、Remote Control が[自動的に接続](#enable-remote-control-for-all-sessions)する場合を含む、接続されたセッションに表示される可能性があります。これらの条件が発生するたびに表示されるわけではなく、各条件はセッション全体で数回だけ表示されます。それらを設定または無効にすることはできません。各条件は独自にクリアされます。

<h3 id="connect-from-another-device">
  別のデバイスから接続する
</h3>

Remote Control セッションがアクティブになったら、別のデバイスから接続するいくつかの方法があります。

* **セッション URL を開く**: 任意のブラウザで [claude.ai/code](https://claude.ai/code)のセッションに直接移動します。
* **QR コードをスキャンする**: セッション URL の横に表示される QR コードをスキャンして、Claude アプリで直接開きます。`claude remote-control` を使用する場合は、スペースバーを押して QR コード表示をトグルします。
* **[claude.ai/code](https://claude.ai/code)または Claude アプリを開く**: セッションリストで名前でセッションを見つけます。Claude モバイルアプリでは、ナビゲーションの **Code** をタップしてセッションリストに到達します。Remote Control セッションはオンラインの場合、コンピュータアイコンと緑色のステータスドットを表示します。

接続すると、デバイスはセッションが既にバックグラウンドで実行しているサブエージェントとワークフローを表示します。デバイスからそれらの 1 つを停止すると、Claude Code はマシンでそのタスクを停止します。

リモートセッションのタイトルは、この順序で選択されます。

1. `--name`、`--remote-control`、または `/remote-control` に渡した名前
2. `/rename` で設定したタイトル
3. 既存の会話履歴の最後の意味のあるメッセージ
4. `myhost-graceful-unicorn` のような自動生成名。ここで `myhost` はマシンのホスト名または `--remote-control-session-name-prefix` で設定したプレフィックスです

明示的な名前を設定しなかった場合、Claude Code はプロンプトを送信するとタイトルを更新して反映します。Claude Code は自動生成されたタイトルを会話の言語、または設定されている場合は [`language`](/docs/ja/settings-reference#language) 設定に一致させます。

claude.ai または Claude アプリからセッションの名前を変更すると、Claude Code は `claude --resume` に表示されるローカルタイトルも更新します。Claude Code は同じ名前変更をプロンプトバーに表示されるセッション名に適用し、セッションが[バックグラウンドで実行](/docs/ja/agent-view)される場合は `claude agents` リストに適用します。v2.1.221 より前では、claude.ai または Claude アプリのセッションリストから名前を変更するとタイトルのみが更新され、CLI は前のセッション名を保持していました。CLI 自体で実行される `/rename` は任意のバージョンで名前を設定します。

Claude アプリをまだ持っていない場合は、Claude Code 内で `/mobile` を実行して、[claude.ai/mobile](https://claude.ai/mobile)の QR コードを表示します。これにより、お使いの電話に適切なアプリストアが開きます。

<h3 id="what-connected-devices-see">
  接続されたデバイスが見るもの
</h3>

接続されたデバイスは、ターミナルの会話をリアルタイムで表示します。これらのケースは通常のメッセージを超えています。

* **圧縮と `/clear`**: Claude Code が[会話を圧縮](/docs/ja/context-window#what-survives-compaction)している間、接続されたデバイスは進捗を表示し、その後会話が圧縮された場所を表示します。`/clear` を実行すると、会話は接続されたデバイスでもリセットされます。
* **`/resume` で会話を切り替える**: 接続されたデバイスは切り替えられた会話のタイトルまたは以前の履歴を受け取りませんが、双方向の新しいメッセージはターミナルで開いている会話に対して行き来します。デバイスから元の会話で再度作業するには、ターミナルで `/resume` を実行して戻します。
* **`/teleport` でセッションをプルする**: [クラウドセッション](/docs/ja/claude-code-on-the-web#from-cloud-to-terminal)をターミナルに `/teleport` でプルする場合、接続されたデバイスはプルされた会話の以前の履歴を受け取りません。双方向の新しいメッセージはプルされた会話に対して行き来し、これはターミナルで開いている会話になります。
* **他のセッションからのメッセージ**: [クロスセッションメッセージング](/docs/ja/cross-session-messaging)では、同じ接続が異なるマシン上の独自のセッション間のメッセージと [Claude Code on the web](/docs/ja/claude-code-on-the-web) セッションからのメッセージを、Remote Control トラフィックの残りのように Anthropic サーバーを通じて運びます。[他のマシンのメッセージセッション](/docs/ja/cross-session-messaging#message-sessions-on-other-machines)は配信ルールをカバーし、[インバウンドメッセージを制御](/docs/ja/cross-session-messaging#control-inbound-messages)はインバウンドコントロールをカバーします。Claude Code v2.1.224 以降が必要です。
* **ターン中に送信されたプロンプト**: 現在のターンが終了する前に接続されたデバイスからプロンプトを送信する場合、Claude Code はそれをキューに入れ、そのターンが終了した後、デバイスのトランスクリプトに保持します。
* **変更の差分**: セッションのディレクトリが git リポジトリにある場合、接続されたデバイスの差分ペインはコミットされていない変更の差分を表示します。デバイスは接続を通じて差分をリクエストし、Claude Code はマシンで計算します。リポジトリのデフォルトブランチより先にコミットがあるブランチでは、ペインはブランチがそこから分岐してからの変更を表示します。これには、コミットされていない編集が含まれます。デフォルトブランチ自体、またはそれより先にないブランチでは、ペインはコミットされていない変更のみを表示します。v2.1.247 より前では、Claude Code は `claude remote-control` で提供されるセッションでのみ接続されたデバイスに差分を報告していました。
* **モデル**: 接続されたデバイスから[モデル](/docs/ja/model-config)を選択する場合、Claude Code はセッションをそのモデルで実行します。ターミナルの `/model` ピッカー、`/status`、および `/config` はそのモデルを表示します。Claude Code v2.1.238 以降が必要です。
  * デバイスのモデルコントロールから選択したモデルは現在のセッションのみに適用されます。デバイスから対話型セッションに `/model <name>` を送信する場合、Claude Code は新しいセッションのデフォルトも設定します。
  * Claude Code が認識しない名前（例えば、モデル ID が予期される場所の表示名）を送信する場合、Claude Code は[ピックを拒否](/docs/ja/errors#model-is-not-a-recognized-model-id)し、セッションは現在のモデルを保持します。v2.1.260 より前では、Claude Code はデバイスのモデルコントロールから認識されないピックを保存し、次のメッセージは失敗していました。
* **努力レベル**: 接続されたデバイスから[努力レベル](/docs/ja/model-config#adjust-effort-level)を設定する場合、`/effort` またはデバイスの努力コントロールで、Claude Code はマシンのセッションに適用し、claude.ai/code はセッションが使用しているレベルを表示します。`CLAUDE_CODE_EFFORT_LEVEL` でレベルをピンした場合、セッションはそのレベルを保持し、Claude Code は努力コントロールから異なるピックを拒否します。努力コントロールからレベルをピックするには、マシンで Claude Code v2.1.234 以降が必要です。
* **接続失敗後の再接続**: `/remote-control` を実行して再接続します。圧縮が会話を書き直したか、その間に `/resume` で会話を切り替えた場合、Claude Code は使用していたサーバーセッションをアーカイブします。セッションリストに残す代わりに。[アーカイブされたセッションをフィルタリング](/docs/ja/claude-code-on-the-web#archive-sessions)することで見つけることができます。デバイスがまだ接続されている間に会話を切り替えてもセッションはアーカイブされません。

<h3 id="enable-remote-control-for-all-sessions">
  すべてのセッションで Remote Control を有効にする
</h3>

Remote Control のみ、`claude remote-control`、`claude --remote-control`、または `/remote-control` を明示的に実行した場合、またはオートコネクトがオンになっている場合にアクティブになります。すべての対話型セッションで自動的に接続するには、Claude Code 内で `/config` を実行し、**Enable Remote Control for all sessions** を設定します。トグルは 3 つの値を取ります。

* **`true`**: 対話型セッションが開始されるときに自動的に接続します。
* **`false`**: オートコネクトをオフにします。ただし、[管理設定](/docs/ja/managed-settings)からの `true` はそれを上回ります。Claude Code はユーザー設定に選択を保存するためです。プロジェクトまたはローカル設定（`.claude/settings.json`、`.claude/settings.local.json`）の `false` は、管理 `true` の上でもオートコネクトをオフにします。
* **`default`**: 選択をクリアし、設定されている場合は組織の管理者デフォルトに従うか、そうでない場合は Claude Code の現在のデフォルトに従います。

同じトグルは CLI の外に表示されます。

* **デスクトップアプリ**: **Settings > Claude Code > Enable remote control by default**。
* **VS Code 拡張機能**: [コマンドメニューの](/docs/ja/vs-code#use-the-prompt-box)Settings セクションの **Enable Remote Control for all sessions**。Claude Code v2.1.203 以降が必要です。

代わりに設定ファイルからオートコネクトをオンにするには、ユーザー `~/.claude/settings.json` または[管理設定](/docs/ja/managed-settings)で [`remoteControlAtStartup`](/docs/ja/settings-reference#remotecontrolatstartup) を `true` に設定します。プロジェクトまたはローカル設定（`.claude/settings.json`、`.claude/settings.local.json`）では、Claude Code は `false` を尊重し、そのリポジトリのオートコネクトをオフにしますが、`true` を無視するため、チェックインされたファイルはリポジトリを開く誰もが Remote Control をオンにすることはできません。

オートコネクトは独自の claude.ai アカウントでサインインするため、開始するセッションは独自のアカウントの Claude アプリにのみ表示され、他の誰にもアクセスを許可しません。

この設定がオンの場合、各対話型 Claude Code プロセスは 1 つのリモートセッションを登録します。複数のインスタンスを実行する場合、各インスタンスは独自のリモートセッションを取得します。単一のプロセスから複数の同時セッションを実行するには、[サーバーモード](#start-a-remote-control-session)を使用します。

<h3 id="resume-sessions-after-stopping-the-server">
  サーバーを停止した後のセッションの再開
</h3>

`claude remote-control` を Ctrl+C で停止すると、提供していたセッションは電話またはブラウザからの応答を停止します。別の `claude remote-control` を同じディレクトリで実行していなかったか、これを `--no-create-session-in-dir` で開始していない限り、Claude Code はそれらをアーカイブしません。復帰させるには、同じディレクトリで次のいずれかのコマンドを実行します。

* **`claude remote-control`**: サーバーが提供していたすべてのセッションを復帰させます。
* **`claude remote-control --continue`**: サーバーが開始したセッションのみを復帰させ、そのセッションが終了するときに終了します。このディレクトリにレコードがない場合、Claude Code はこのリポジトリの他の git worktree から最新のものを使用します。
* **`claude remote-control --session-id <id>`**: 渡した ID のセッションのみを復帰させ、そのセッションが終了するときに終了します。ID はセッションの URL の claude.ai/code の `/code/` と任意の `?` の間の部分です。

これらのコマンドはサーバーが停止してから約 4 時間機能します。その後、`claude remote-control` を実行して新しいセッションを開始します。その間にセッションをアーカイブした場合、`--continue` と `--session-id` は Claude Code v2.1.228 以降でそれをアーカイブ解除します。

`claude --remote-control` または `/remote-control` で開始したセッションを復帰させるには、`claude --continue` または `claude --resume` で会話を再開します。Claude Code が再接続するかどうか、およびどのセッションに再接続するかは、会話の[再接続レコード](#resume-outcomes)に依存します。

最初のターミナルが Remote Control をオンにしたままで、2 番目のターミナルで会話を再開する場合、Claude Code は 2 番目のターミナルに通知を出力し、セッションを最初から取り去る代わりに Remote Control をオフのままにします。Remote Control がそこでオフのままの間、そのターミナルの Claude は[他のマシンのセッション](/docs/ja/cross-session-messaging#see-which-sessions-claude-can-reach)を見ず、それらはそれに到達できません。2 番目のターミナルで `/remote-control` を実行して Remote Control をそこに移動します。

Remote Control がオンだった Claude Desktop または IDE 拡張機能で会話を再開する場合、Claude Code は新しいセッションをセッションリストに追加する代わりに、既存の claude.ai セッションに再度接続します。

<h2 id="connection-and-security">
  接続とセキュリティ
</h2>

ローカル Claude Code セッションは、アウトバウンド HTTPS リクエストのみを行い、マシン上のインバウンドポートを開くことはありません。Remote Control を開始すると、Anthropic API に登録され、作業をポーリングします。別のデバイスから接続すると、サーバーは Web またはモバイルクライアントとローカルセッション間のメッセージをストリーミング接続経由でルーティングします。

すべてのトラフィックは TLS 経由で Anthropic API を通じて移動し、Claude Code セッションと同じトランスポートセキュリティです。接続は複数の短命の認証情報を使用し、各認証情報は単一の目的にスコープされ、独立して有効期限が切れます。`claude remote-control` サーバーの登録認証情報が有効期限切れになると、サーバーは Anthropic API に再度登録され、セッションの提供を継続します。

Remote Control が接続されている間、セッショントランスクリプト（メッセージ、Claude の応答、ツールアクティビティを含む）は Anthropic サーバーに保存されます。保存されたトランスクリプトは、デバイス間で会話を同期させ、ネットワーク障害後にセッションが再接続できるようにします。実行とファイルシステムアクセスはマシン上に留まり、保存されたトランスクリプトは [データ使用](/docs/ja/data-usage) ポリシーに基づいて保持されます。

Remote Control を完全にオフにするには、[`disableRemoteControl`](/docs/ja/settings-reference#disableremotecontrol) 設定を使用します。Zero Data Retention などのコンプライアンス要件を持つ組織は Remote Control を有効にすることはできません。

<h2 id="trusted-devices">
  信頼できるデバイス
</h2>

<Note>
  信頼できるデバイスは現在ベータ版です。エクスペリエンスが改善されるにつれて、機能と機能が進化する可能性があります。

  信頼できるデバイスは Pro、Max、Team、および Enterprise プランで利用可能であり、デフォルトではオフになっています。Team および Enterprise プランでは、Owner が組織に対してこれをオンにします。Pro および Max プランでは、設定の Cowork またはアカウントページで、自分で **信頼できるデバイスを要求** をオンにします。
</Note>

信頼できるデバイスは、組織のメンバー、または Pro もしくは Max プランではあなた自身が、claude.ai、Claude モバイルアプリ、または Claude Desktop から Remote Control セッションを表示または操作する前に、デバイスを確認する必要があります。これは、署名されたアカウントだけでなく、既知のデバイスと最近の認証に Remote Control アクセスを結び付けます。

設定がオンの場合、Remote Control セッションと相互作用するには、以下の両方が必要です。

* **登録されたデバイス**: メンバーが Remote Control に使用する各ブラウザ、電話、またはデスクトップアプリは、独自の認証情報を登録します。登録は完全なサインイン直後にのみ提供されるため、デバイスはバックグラウンドで静かに登録されるのではなく、実際の認証の一部として信頼できるリストに参加します。
* **最近のサインイン**: メンバーのサインインは 18 時間以内である必要があります。毎日サインインする代わりに、メンバーは Face ID、Touch ID、Windows Hello、またはパスキーで存在を確認します。このバイオメトリック段階的認証はセッションを即座にリフレッシュします。

バイオメトリック チェックはデバイス上でオペレーティングシステムまたはブラウザを通じて実行され、パスキーサインインと同じメカニズムです。Anthropic は指紋、顔データ、またはその他のバイオメトリック情報を受け取ったり保存したりすることはありません。デバイスの公開鍵と表示名、プラットフォーム、登録時刻などの基本的なメタデータのみが保存されます。

この設定は Remote Control にのみ適用されます。通常の Claude チャット、ターミナルの Claude Code、および API 使用は影響を受けません。

<h3 id="enable-trusted-devices-for-your-organization">
  Team または Enterprise 組織で信頼できるデバイスを有効にする
</h3>

Owner は claude.ai 組織設定から設定を有効にします。

<Steps>
  <Step title="Capabilities ページに移動する">
    [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities) に移動します。**Require trusted devices** トグルはそのセクションに表示されます。
  </Step>

  <Step title="信頼できるデバイスを要求をオンにする">
    この設定は組織のすべてのメンバーと、トグルを有効にした後に開始された Remote Control セッションに適用されます。トグルがオンになる前に既に実行されていたセッションは遡及的に保護されず、終了するまでデバイス要件なしで続行されます。チームごとまたはプロジェクトごとのスコープは利用できません。
  </Step>

  <Step title="メンバーに何を期待するかを伝える">
    設定が有効になった後、メンバーがブラウザ、電話、またはデスクトップアプリから新しい Remote Control セッションを初めて表示または操作するときに、そのデバイスを登録するよう求められます。事前に知らせることで混乱を避けられます。
  </Step>
</Steps>

<h3 id="what-members-see">
  メンバーが見るもの
</h3>

登録はデバイスごとに 1 回限りのステップです。その後、唯一の目に見える変化は時折のバイオメトリック プロンプトです。

* **各デバイスでの初回使用**: メンバーは登録するよう求められます。サインインが最近でない場合は、SSO が設定されている場合を含む通常のフローを通じてサインインしてから、登録を確認します。
* **日々**: 登録されたデバイスと最近のサインインを持つメンバーはプロンプトを見ません。サインインが 18 時間を超えて経過すると、次の Remote Control インタラクションは単一の Face ID、Touch ID、Windows Hello、またはパスキー プロンプトを表示します。
* **登録されていないデバイス**: デバイスが登録されるまで、Remote Control セッションを表示または操作することはできません。そのデバイスでの通常の Claude チャットは影響を受けません。
* **プラットフォーム認証器がない**: Face ID、Touch ID、または Windows Hello がないマシン上のメンバーは、ハードウェアセキュリティキーを使用するか、段階的認証の代わりにサインインできます。
* **ターミナルで**: Claude Code を実行しているマシンは、開発者が CLI にサインインするときに独自の認証情報を自動的に受け取ります。ターミナルに別の登録ステップはありません。

<h3 id="manage-enrolled-devices">
  登録されたデバイスを管理する
</h3>

メンバーはアカウント設定から独自のデバイスを確認および取り消すことができます。

[claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) を開き、**Trusted devices** セクションを見つけて、名前、プラットフォーム、登録日を含むすべての登録されたデバイスを確認します。デバイスを削除すると、その認証情報は即座に取り消され、デバイスは後で新しいサインイン後に再登録できます。認証情報は更新されない場合は自動的に有効期限が切れるため、未使用のデバイスは信頼できるリストから自動的に削除されます。

紛失または盗難されたデバイスの場合、メンバーはこのページから削除します。メンバーがサインインできない場合、管理者は管理コンソールで **Sign out everywhere** を使用してそのメンバーのすべてのセッションと登録されたデバイスを取り消すことができます。その後、メンバーは保持しているデバイスを再登録します。

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control とクラウドセッション
</h2>

Remote Control と [クラウドセッション](/docs/ja/claude-code-on-the-web) の両方が claude.ai/code インターフェースを使用します。主な違いはセッションが実行される場所です。Remote Control はマシン上で実行されるため、ローカル MCP サーバー、ツール、プロジェクト設定が利用可能なままです。クラウドセッションはクラウドインフラストラクチャ上で実行され、デフォルトでは Anthropic が管理します。

ローカル作業の途中で別のデバイスから続行したい場合は Remote Control を使用します。ローカルセットアップなしでタスクを開始したい場合、クローンしていないリポジトリで作業したい場合、または複数のタスクを並列で実行したい場合はクラウドセッションを使用します。

<h2 id="mobile-push-notifications">
  モバイルプッシュ通知
</h2>

Remote Control がアクティブな場合、Claude は電話にプッシュ通知を送信できます。

Claude がプッシュを送信するタイミングを決定します。通常は、長時間実行されるタスクが完了したときまたは続行するために決定が必要なときに送信されます。プロンプトでプッシュをリクエストすることもできます。たとえば、`notify me when the tests finish` のように指定します。以下のオン/オフトグル以外に、イベントごとの設定はありません。

モバイルプッシュ通知をセットアップするには:

<Steps>
  <Step title="Claude モバイルアプリをインストールする">
    Claude アプリを [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) または [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) にダウンロードします。
  </Step>

  <Step title="Claude Code アカウントでサインインする">
    ターミナルで Claude Code に使用するのと同じアカウントと組織を使用します。
  </Step>

  <Step title="通知を許可する">
    オペレーティングシステムからの通知許可プロンプトを受け入れます。
  </Step>

  <Step title="Claude Code でプッシュを有効にする">
    ターミナルで `/config` を実行し、プロアクティブな通知の場合は **Push when Claude decides** を有効にし、許可プロンプトと質問の場合は **Push when actions required** を有効にするか、その両方を有効にします。
  </Step>
</Steps>

通知が到着しない場合:

* `/config` が **No mobile registered** を表示する場合は、Claude アプリを電話で開いてプッシュトークンをリフレッシュできるようにします。警告は Remote Control が次に接続するときにクリアされます。
* iOS では、フォーカスモードと通知サマリーがプッシュを抑制または遅延させることができます。Settings → Notifications → Claude を確認してください。
* Android では、積極的なバッテリー最適化が配信を遅延させることができます。システム設定で Claude アプリをバッテリー最適化から除外します。

Claude Code は、ターミナルに入力中またはターミナルにフォーカスしている間、モバイルプッシュ通知をスキップします。v2.1.181 以降では、[`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/ja/env-vars) をマーカーファイルパスに設定して、別のウィンドウにいる場合でも、マシンにいるときはいつでもこれを拡張できます。ファイルが存在する間は通知がスキップされます。スクリーンロックリスナーまたは同様のツールを設定して、スクリーンがロック解除されたときにファイルを作成し、スクリーンがロックされたときにファイルを削除します。

<h2 id="limitations">
  制限事項
</h2>

* **対話型プロセスごとに 1 つのリモートセッション**: サーバーモード外では、各 Claude Code インスタンスは一度に 1 つのリモートセッションをサポートします。単一のプロセスから複数の同時セッションを実行するには、[サーバーモード](#start-a-remote-control-session)を使用します。
* **ローカルプロセスは実行し続ける必要があります**: Remote Control はローカルプロセスとして実行されます。ターミナルを閉じるか、VS Code を終了するか、または `claude` プロセスを停止すると、セッションはオフラインになります。[セッションを復帰させる](#resume-sessions-after-stopping-the-server)まで、セッションはオフラインのままです。Claude がタスクの途中でない限り、claude.ai と Claude アプリはプロセス終了後数秒以内にセッションをオフラインとして表示します。SSH から切断した後もリモートマシンでセッションを実行し続けるには、`tmux` または `screen` 内で起動します。
* **サーバーモードでのクラッシュしたセッション**: `claude remote-control` で提供されるセッションがクラッシュした場合、接続されたデバイスからメッセージを送信します。Claude Code はそれを再度提供します。サーバーを再起動する必要はありません。Claude Code v2.1.238 以降が必要です。
* **接続されたセッションでの HTTP 403 拒否**: 対話型セッションが接続されると、VPN またはネットワークの変更後に発生する可能性があるように、マシンと Anthropic のサーバー間の何かが HTTP 403 で応答する場合、Claude Code は最大 3 分間再試行を続けます。拒否が長く続く場合、Claude Code は切断され、理由は何が拒否したかを示します。ネットワークエッジ、またはユーザー自身のネットワーク上のプロキシ、VPN、またはファイアウォールです。
* **長時間のネットワーク障害**: マシンが起動しているがネットワークに到達できない場合、次に何をするかはモードによって異なります。
  * **サーバーモード**: Claude Code はおよそ 10 分後に諦め、`claude remote-control` プロセスは終了します。新しいセッションを開始するには、`claude remote-control` を再度実行します。
  * **対話型セッション**: ローカルで作業を続けます。Claude Code は障害が続く限り再試行し、ネットワークが復帰すると自動的に再接続します。
* **プレゼンスハートビートの失敗**: 対話型セッションが `could not reach the Remote Control server for about 30 minutes` で切断された場合、`/remote-control` を実行して再接続します。Claude Code はセッションのプレゼンスハートビートが失敗している場合にのみこのメッセージを表示し、接続の残りの部分は稼働したままです。セッションは約 30 分間再登録されてから切断されます。
* **転送されたダイアログの有効期限**: Claude Code は権限プロンプトと `AskUserQuestion` の質問を、ユーザーが回答するまで開いたままにします。Claude Code が別の種類のダイアログをリモートセッションに転送する場合（安全性拒否後に表示されるモデル選択プロンプトなど）、デフォルトでは 5 分待機してからダイアログを閉じ、ダイアログのアクション不要のデフォルトで続行します。[`dialogExpiry`](/docs/ja/settings-reference#dialogexpiry) を設定して期限を調整または無効化します。Claude Code v2.1.224 以降が必要です。
* **Fable 使用クレジット同意プロンプトは転送されません**: Claude Code はセッション中の [Fable 使用クレジット同意プロンプト](/docs/ja/model-config#fable-and-usage-credits)をセッションが実行される場所にのみ表示し、ユーザーのデバイスには表示しません。セッションがターミナルで実行され、Claude Code がプロンプトを閉じる前に誰もそこで回答しない場合、ターンはリクエストを送信せずに終了します。[確認するプロンプトが未回答でした](/docs/ja/errors#the-prompt-to-confirm-went-unanswered)を参照してください。
* **一部のコマンドはローカルのみです**: ターミナルインターフェースでのみ実行されるコマンド（`/plugin` や `/resume` など）は、引数を渡すかどうかに関わらず、ローカル CLI からのみ機能します。以下がモバイルと Web から機能します。
  * テキスト出力コマンド: `/compact`、`/clear`、`/context`、`/usage`、`/exit`、`/usage-credits`、`/recap`、`/reload-plugins`。`/usage-credits` はブラウザを開く代わりに請求 URL を出力します。`/reload-plugins` はセッションが対話型ターミナルで実行されている場合にのみ機能します。セッションがない場合は拒否されます。
  * `/model`、`/effort`、`/fast`、`/color`、`/rename`: 値を引数として渡します。例えば `/model sonnet` または `/effort high` のようにします。モバイルと Web からは、`/model` と `/effort` はターミナルピッカーまたはスライダーの代わりに引数を受け取ります。
  * `/mcp`: モバイルアプリからは、ピッカーを開く代わりにサーバーステータスのテキスト概要を返します。Web では、`/mcp` 単独で概要を返す代わりに [claude.ai コネクタ](/docs/ja/mcp#use-mcp-servers-from-claude-ai)のディレクトリを開きます。`reconnect`、`enable`、`disable` [サブコマンド](/docs/ja/commands#all-commands)は両方から機能します。ローカル CLI と異なり、サーバー名なしで `/mcp reconnect` を実行すると、失敗したか認証が必要なすべてのサーバーを再接続します。
  * `/config`: モバイルアプリからは、`key=value` を渡して設定を行うか、引数なしで実行して設定できるキーのリストを表示します。Web では、`/config` は代わりに設定の Claude Code セクションを開き、コマンドの後のテキストを無視します。
  * Team と Enterprise では、モバイルまたは Web から `/usage-credits` を実行しても、[管理者への使用クレジットリクエスト](/docs/ja/costs#add-usage-credits-to-your-subscription)は送信されません。送信には対話型 CLI にのみ表示される確認が必要なため、コマンドはそこで実行するよう指示します。v2.1.211 より前は、テキスト形式は確認なしでリクエストを送信していました。
  * `/autocompact`、v2.1.221 以降: ウィンドウサイズを引数として渡します。例えば `/autocompact 500k` のようにします。引数がない場合、ターミナルセッションで表示されるダイアログを開く代わりに、現在のウィンドウサイズをテキストとして出力します。
  * `/advisor`、v2.1.260 以降: モデルを引数として渡します。例えば `/advisor opus` のようにします。または `off` を渡してアドバイザーをオフにします。両方の形式は現在のセッションにのみ適用され、保存されたデフォルトは変わりません。引数がない場合、ピッカーを開く代わりに、現在のアドバイザーをテキストとして出力します。
  * `/output-style`、v2.1.269 以降: スタイル名を引数として渡します。例えば `/output-style concise` のようにします。または引数なしで実行してスタイルのリストを表示します。モバイルと Web からは、[組み込みスタイル](/docs/ja/output-styles#built-in-output-styles)のみをリストして選択できます。[カスタムスタイル](/docs/ja/output-styles#create-a-custom-output-style)を使用するには、セッション自体で選択します。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  「Remote Control には claude.ai サブスクリプションが必要です」
</h3>

claude.ai アカウントで認証されていないか、別の認証情報がログインより優先されています。メッセージは以下のいずれかの形式になります。

* サインアウト状態で `/remote-control` または `--remote-control` から：「Remote Control requires a claude.ai subscription.」または「/remote-control requires a claude.ai subscription.」
* サインアウト状態で `claude remote-control` から：「You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.」
* サインイン状態だが API キーまたはトークンが使用中：「Remote Control requires claude.ai subscription auth.」の後に、`ANTHROPIC_API_KEY is set, so this session is using API-key auth` などの使用中の認証情報が続きます。`apiKeyHelper` 設定と `ANTHROPIC_AUTH_TOKEN` は同じ方法で名前が付けられます。

`claude auth login` を実行して claude.ai オプションを選択してください。メッセージが `ANTHROPIC_API_KEY` または `ANTHROPIC_AUTH_TOKEN` を名前付けしている場合は、シェル環境または [設定ファイル](/docs/ja/settings-reference#env)の `env` ブロックのいずれかに設定されている場所から削除してください。`apiKeyHelper` を名前付けしている場合は、その設定を削除してください。

v2.1.206 より前では、サインアウト状態で `/remote-control` を実行すると、このメッセージの代わりに `Unknown command: /remote-control` が報告されていました。

<h3 id="remote-control-requires-a-full-scope-login-token">
  「Remote Control には完全スコープのログイントークンが必要です」
</h3>

`claude setup-token` または `CLAUDE_CODE_OAUTH_TOKEN` 環境変数からの長命トークンで認証されています。これらのトークンはモデルリクエストのみを実行できるため、Remote Control セッションを確立できません。代わりに `claude auth login` を実行して、完全スコープのセッショントークンで認証してください。

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  「Remote Control 適格性のための組織を決定できません」
</h3>

キャッシュされたアカウント情報が古いまたは不完全です。`claude auth login` を実行して更新してください。

<h3 id="remote-control-isn’t-enabled-for-this-account">
  「Remote Control はこのアカウントで有効になっていません」
</h3>

Claude Code はサインインしているアカウントの Remote Control 可用性をチェックし、チェックがオフで返されました。通常の原因は、プラン変更後に期限切れになったキャッシュされた権利です。`claude auth logout` を実行してから `claude auth login` を実行して更新し、古いバージョンを使用している場合は Claude Code を更新してください。

`claude doctor` を実行して、どの個別の適格性チェックが失敗したかを確認してください。環境変数の競合、到達不可能なチェック、および組織の Remote Control 設定はそれぞれ独自のメッセージを生成するため、このエラーはアカウントレベルのチェック自体を意味します。

v2.1.239 より前では、このメッセージは「Remote Control is not yet enabled for your account」と表示されていました。v2.1.154 より前では、`DISABLE_TELEMETRY` または `DO_NOT_TRACK` などのフィーチャーフラグ評価を無効にする変数もこのメッセージを生成していました。以下の「Remote Control はフィーチャーフラグ評価を必要とします」エントリがその設定をカバーしています。

<h3 id="couldn’t-verify-remote-control-eligibility">
  「Remote Control 適格性を確認できませんでした」
</h3>

Claude Code は Remote Control がアカウントで有効になっているかどうかをチェックするためにフィーチャーフラグサービスに到達できませんでした。通常、オフラインであるか、プロキシがリクエストをブロックしているためです。ネットワークアクセスが可能になったら再度実行するか、詳細については `claude doctor` を実行してください。関連メッセージ「組織の Remote Control ポリシーを確認できませんでした」は同じ原因を持ち、同じ修正があります。両方のメッセージは v2.1.178 で追加されました。

<h3 id="remote-control-requires-feature-flag-evaluation">
  「Remote Control はフィーチャーフラグ評価を必要とします」
</h3>

これらの変数のいずれかが設定されています：[`DISABLE_TELEMETRY`、`DO_NOT_TRACK`、`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`、または `DISABLE_GROWTHBOOK`](/docs/ja/env-vars)。これらのそれぞれは Remote Control 可用性が依存するフィーチャーフラグ評価を無効にし、完全なメッセージは Claude Code が見つけた変数を名前付けします。シェル環境または [`settings.json` ファイル](/docs/ja/settings-reference#all-settings)の `env` ブロックのいずれかに設定されている場所から、その変数を設定解除してください。v2.1.154 より前のバージョンでは、同じ設定により「Remote Control is not yet enabled for your account」が代わりに生成されます。

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  「Remote Control は Claude を api.anthropic.com 経由で使用している場合にのみ利用可能です」
</h3>

セッションが Anthropic API に直接通信していないため、ペアリングする claude.ai バックエンドがありません。これは Amazon Bedrock、Google Cloud の Agent Platform、および Microsoft Foundry で発生します。また、[`ANTHROPIC_BASE_URL`](/docs/ja/env-vars)が `api.anthropic.com` 以外のホスト（[LLM ゲートウェイ](/docs/ja/llm-gateway)やプロキシなど）を指している場合にも発生します。claude.ai でサインインしている場合でも同様です。v2.1.196 より前では、Claude Code はカスタム `ANTHROPIC_BASE_URL` に対してこのメッセージを表示していませんでした。完全な原因リストについては、[エラーリファレンス](/docs/ja/errors#remote-control-requires-the-anthropic-api)を参照してください。

メッセージは `CLAUDE_CODE_USE_BEDROCK` またはカスタム `ANTHROPIC_BASE_URL` など、セッションを Anthropic API から遠ざけたものを名前付けします。適格な claude.ai ログインがある場合は、名前付けされた変数を設定解除し、[設定](/docs/ja/settings)で `env` キーから削除した場合は削除し、セッションを再開してください。v2.1.219 より前では、メッセージはこのセクションのヘッダーの文のみであったため、古いバージョンでは `CLAUDE_CODE_USE_BEDROCK` や `CLAUDE_CODE_USE_VERTEX` などのプロバイダー変数と `ANTHROPIC_BASE_URL` について環境を自分で確認してください。

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  「Remote Control は組織のポリシーで無効になっています」
</h3>

ポリシーが Remote Control をブロックするか、Claude Code がこのマシンで組織のポリシーを読み込めず、その間 Remote Control をオフのままにしています。これらの原因を順番にチェックしてください。

* **エラーが `disableRemoteControl` を言及している**：IT 管理者が [管理設定](/docs/ja/managed-settings)を通じてこのデバイスで Remote Control を無効にしています。これは組織全体のトグルとは関係なく、サインイン方法とは関係なく行われています。
* **claude.ai プランが Pro または Max である**：Claude Code は以前のログインから Team または Enterprise 組織の下でまだサインインしているため、その組織の Remote Control ポリシーをチェックします。`/status` を実行して、サインインが使用するプランと組織を確認してください。`claude auth logout` を実行してから `claude auth login` を実行して、現在のプランの下で再度サインインしてください。
* **組織ポリシーがこのマシンで読み込まれなかった**：`claude doctor` を実行して `Organization policy` 行を読んでください。行がポリシーが読み込まれていないことを示している場合、それが Remote Control をオフのままにしているものです。v2.1.261 より前では、`claude doctor` はこの行を出力していませんでした。
* **メッセージが組織管理者に連絡するよう言っていない**：組織には Remote Control と互換性のない HIPAA 設定があり、`/status` は `Compliance` 行に `HIPAA` をリストしています。この状態では、管理パネルの Remote Control トグルはグレーアウトしているため、所有者はそこで変更できません。オプションについて説明するために Anthropic サポートに連絡してください。v2.1.267 より前では、このケースは「Remote Control isn't available for your organization due to its compliance policy」と表示されていました。
* **それ以外の場合、所有者が組織に対して有効にしていない**：Remote Control は Team および Enterprise プランではデフォルトでオフになっています。所有者は [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) で **Remote Control** トグルをオンにして有効にできます。このトグルはサーバー側の組織設定です。

<h3 id="remote-credentials-fetch-failed">
  「リモート認証情報の取得に失敗しました」
</h3>

Claude Code は Anthropic API から短命の認証情報を取得して接続を確立できませんでした。`--verbose` で再度実行して完全なエラーを確認してください。

```bash theme={null}
claude remote-control --verbose
```

一般的な原因：

* サインインしていない：`claude` を実行し、`/login` を使用して claude.ai アカウントで認証してください。API キー認証は Remote Control ではサポートされていません。
* ネットワークまたはプロキシの問題：ファイアウォールまたはプロキシがアウトバウンド HTTPS リクエストをブロックしている可能性があります。Remote Control はポート 443 で Anthropic API へのアクセスが必要です。
* セッション作成に失敗：`Session creation failed — see debug log` も表示される場合、失敗はセットアップの前の段階で発生しました。サブスクリプションがアクティブであることを確認してください。

古いログイントークンはこのエラーを引き起こしません。Anthropic API が保存されたトークンを拒否する場合（例えば、別の Claude Code プロセスがすでにそれを更新したため）、Claude Code はトークンを更新して自動的に再試行します。v2.1.224 より前では、古いトークンはこのメッセージで Remote Control スタートアップに失敗し、[自動的に接続するように設定](#enable-remote-control-for-all-sessions)されたセッションは起動時に断続的に失敗する可能性がありました。

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  「Remote Control セッションに再接続できませんでした」
</h3>

`claude --resume` または `claude --continue` で会話を再開すると、Claude Code はその会話に記録された Remote Control セッションに再接続します。このメッセージは、ネットワーク中断またはサーバーエラーなど、一時的な理由で再接続に失敗したことを意味します。そのため、Claude Code はリモートセッションがまだ存在するかどうかを確認できません。

`/remote-control` を実行して接続を再試行するか、`claude --remote-control` で新しいセッションを開始して新しい Remote Control セッションを作成してください。ローカルセッションは Remote Control なしで実行を続けます。

<span id="resume-outcomes" />再開すると、このメッセージの代わりにこれらの結果のいずれかを取得することもできます。

* **サーバーが記録されたセッションが消えたことを報告するか、再接続レコードが別のアカウントを名前付けする**：Claude Code は会話の再接続レコードが言うことに従います。
  * **レコードがサインインしているアカウントを名前付けする**：Claude Code は自動生成された名前で置換セッションを開始し、会話の以前のメッセージをそれから除外します。例えば、claude.ai または Claude アプリからセッションを削除した後、これを取得します。
  * **レコードが別のアカウントを名前付けする**：Claude Code は会話の以前のメッセージなしで新しいセッションを開始し、記録されたセッションがまだ存在するかどうかに関わらずメッセージを表示せずに開始します。
  * **レコードがセッションを所有していたアカウントを言わないか、Claude Code が保存されたサインインを読むことができない**：Claude Code はこのメッセージの代わりに [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable)を表示し、何も開始せず、会話からレコードを削除します。
* **再開する前に Remote Control をオフにした**：Claude Code をホストしているアプリが、アプリが claude.ai セッションを所有していることを通知していない限り、Claude Code は CLI の [ステータスパネル](#check-connection-status)、VS Code 拡張機能、または [Agent SDK](/docs/ja/agent-sdk/overview)に基づいて構築されたホストから Remote Control をオフにしたときに再接続レコードを削除したため、再接続しません。所有アプリがそれをオフにした場合、Claude Code はレコードを保持し、再接続します。
* **このマシン上の別の Claude Code がまだセッションを持っている**：`Remote Control not started here` で始まる通知が表示され、Claude Code は [再開されたセッションで Remote Control をオフのままにします](#resume-sessions-after-stopping-the-server)。そこで `/remote-control` を実行して移動してください。

<span id="reconnect-history" />v2.1.232 より前では、Claude Code はサーバーが記録されたセッションが消えたことを報告したときに異なる応答をしました。v2.1.227 から v2.1.231 まで、Claude Code はレコードがアカウントと一致した場合でも置換を開始することを拒否しました。v2.1.226 を通じて、Claude Code はレコードがアカウントと一致したかどうかに関わらず置換を開始し、v2.1.224 から v2.1.226 では、会話の以前のメッセージをアップロードせずに、そのマシンで署名されたアカウントの下で作成し、別のアカウントの下では決して作成しませんでした。v2.1.200 より前では、Claude Code は再接続の失敗後に新しいセッションを作成しました。

<h3 id="previous-session-is-unavailable">
  「Previous session is unavailable — run /remote-control to start a new one」
</h3>

Claude Code は前の Remote Control セッションを復元できず、自動的に新しいセッションを開始する代わりに停止しました。`claude --resume` または `claude --continue` で会話を再開した後、またはクロード Code が [切断後に自動的に再接続](/docs/ja/errors#remote-control-couldnt-refresh-your-login)した後、このメッセージが表示される場合があります。

`/remote-control` を実行して、現在のログインの下で新しい Remote Control セッションを開始してください。ローカルセッションは Remote Control なしで実行を続けます。関連メッセージ `Remote Control could not verify the signed-in account — run /remote-control to reconnect` は同じ修正を持っています。Claude Code はサインインしているアカウントが変更されたか、検証と再接続の間で読むことができなかった場合に表示します。`Previous session is unavailable` の後に Claude Code を再開する前に `/remote-control` を実行した場合、Claude Code は会話の以前のメッセージを新しいセッションから除外します。

再開時に、Claude Code は [その場所に新しいセッションを開始します](#resume-outcomes)。会話の再接続レコードがセッションを所有していたアカウントを名前付けしている場合のみです。サーバーは削除したセッションと別のアカウントが所有するセッションを同じ方法で報告するためです。v2.1.227 より前の Claude Code はそのアカウントを記録していなかったため、Claude Code は保存されたサインインを読むことができない場合はレコードをチェックできません。v2.1.232 より前の Claude Code は `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` を表示していました。[異なるケースセット](#reconnect-history)では。

<h3 id="remote-control-got-an-unexpected-server-response">
  「Remote Control は予期しないサーバー応答を受け取りました」
</h3>

Remote Control サーバーはリクエストを受け入れましたが、リモートセッションを作成するか、その認証情報を取得する際に、このバージョンの Claude Code が読むことができない形式で応答しました。同じバージョンで再試行すると、同じ方法で失敗します。`claude update` を実行してから、`/remote-control` を実行して再接続してください。このメッセージは v2.1.225 で追加されました。

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  「組織は Remote Control に信頼できるデバイスを要求していますが、このデバイスは登録されていません」
</h3>

組織は [信頼できるデバイス](#trusted-devices)を有効にしており、このマシンはまだ登録されていません。Claude Code で `/login` を実行してください。登録はサインインの一部として行われ、別の登録コマンドはありません。

<h3 id="session-expired-for-trusted-device-check">
  「信頼できるデバイスチェックのセッションが期限切れです」
</h3>

サインインが 18 時間以上前です。Claude Code で `/login` を実行するか、claude.ai またはモバイルアプリが Face ID、Touch ID、Windows Hello、またはパスキーで確認するよう求めたときに確認してください。[信頼できるデバイス](#trusted-devices)を参照してください。

<h2 id="choose-the-right-approach">
  適切なアプローチを選択する
</h2>

Claude Code は、ターミナルにいない時に作業するための複数の方法を提供しています。これらは、何が作業をトリガーするか、Claude がどこで実行されるか、そしてセットアップにどの程度の手間が必要かが異なります。

|                                                          | トリガー                                                                      | Claude が実行される場所                                                                           | セットアップ                                                                                                              | 最適な用途                       |
| :------------------------------------------------------- | :------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ | :-------------------------- |
| [Dispatch](/docs/ja/desktop#sessions-from-dispatch)           | Claude モバイルアプリからタスクをメッセージで送信                                              | あなたのマシン（Desktop）                                                                          | [モバイルアプリを Desktop とペアリング](https://support.claude.com/en/articles/13947068)                                          | 外出中の作業委譲、最小限のセットアップ         |
| [Remote Control](/docs/ja/remote-control)                     | [claude.ai/code](https://claude.ai/code) または Claude モバイルアプリから実行中のセッションを操作 | あなたのマシン（CLI または VS Code）                                                                  | `claude remote-control` を実行                                                                                         | 別のデバイスから進行中の作業を操舵           |
| [Channels](/docs/ja/channels)                                 | Telegram や Discord などのチャットアプリ、またはあなた自身のサーバーからイベントをプッシュ                    | あなたのマシン（CLI）                                                                              | [チャネルプラグインをインストール](/docs/ja/channels#quickstart)するか、[独自に構築](/docs/ja/channels-reference)                                      | CI 失敗やチャットメッセージなどの外部イベントに対応 |
| [Slack](/docs/ja/slack)                                       | チームチャネルで `@Claude` をメンション                                                 | Anthropic クラウド                                                                            | [Slack アプリをインストール](/docs/ja/slack#setting-up-claude-code-in-slack)し、[ウェブ上の Claude Code](/docs/ja/claude-code-on-the-web) を有効化 | チームチャットからの PR とレビュー         |
| [Self-hosted environments](/docs/ja/self-hosted-environments) | [クラウドセッション](/docs/ja/claude-code-on-the-web)を開始し、組織の環境を選択                      | あなたの組織のインフラストラクチャ                                                                         | [ランナーをデプロイ](/docs/ja/self-hosted-environments-quickstart)、Team および Enterprise プラン                                        | ネットワーク内で実行する必要があるクラウドセッション  |
| [Scheduled tasks](/docs/ja/scheduled-tasks)                   | スケジュールを設定                                                                 | [CLI](/docs/ja/scheduled-tasks)、[Desktop](/docs/ja/desktop-scheduled-tasks)、または[クラウド](/docs/ja/routines) | 頻度を選択                                                                                                               | 日次レビューなどの定期的な自動化            |

<h2 id="related-resources">
  関連リソース
</h2>

* [Web 上の Claude Code](/docs/ja/claude-code-on-the-web): マシン上ではなくクラウドでセッションを実行します。[クラウド環境](/docs/ja/cloud-environments)を通じて設定します
* [クロスセッションメッセージング](/docs/ja/cross-session-messaging): Claude が他のマシンまたは [クラウドセッション](/docs/ja/claude-code-on-the-web)上のセッションにメッセージを送信できるようにします
* [チャネル](/docs/ja/channels): Telegram、Discord、または iMessage をセッションに転送して、Claude が離席中にメッセージに反応するようにします
* [Dispatch](/docs/ja/desktop#sessions-from-dispatch): 電話からタスクをメッセージして、Desktop セッションを生成して処理できます
* [認証](/docs/ja/authentication): `/login` をセットアップし、claude.ai の認証情報を管理します
* [CLI リファレンス](/docs/ja/cli-reference): `claude remote-control` を含むフラグとコマンドの完全なリスト
* [セキュリティ](/docs/ja/security): Remote Control セッションが Claude Code セキュリティモデルにどのように適合するか
* [データ使用](/docs/ja/data-usage): ローカル、Remote Control、およびクラウドセッション中に Anthropic API を通じてどのようなデータが流れるか
