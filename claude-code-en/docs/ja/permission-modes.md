> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 権限モードを選択する

> Claude がアクションを実行する前に確認するかどうかを制御します。CLI で Shift+Tab でモードを切り替えるか、VS Code のモード指示器、Desktop のモードセレクター、または Web のモードドロップダウンを使用します。

権限モードは、Claude がセッション内でアクションを実行する前に確認を求めるかどうかを設定します。Manual モードでは、Claude Code はファイルを編集、シェルコマンドを実行、またはネットワークにアクセスするほとんどのアクションの前に一時停止して確認を求めます。[auto モード](#eliminate-prompts-with-auto-mode)では、分類器と呼ばれる 2 番目のモデルがあなたの代わりにアクションをレビューします。[分類器がアクションを評価する方法](#how-the-classifier-evaluates-actions)は、分類器がレビューするアクションとスキップするアクションをリストしています。

Pro、Max、Team プランでは、組み込みの開始権限モードは auto モードです。[セッションが開始するモード](#which-mode-a-session-starts-in)は、開始権限モードを変更するサーフェスと設定をカバーしています。実行中のセッションの権限モードはいつでも変更できます。

<h2 id="available-modes">
  利用可能なモード
</h2>

各モードは、利便性と監視のバランスを異なる方法で取ります。以下の表は、各モードで Claude がパーミッション プロンプトなしで実行できることを示しています。Manual モードはその設定値である `default` の下に表示されます。

| モード                                                                 | 確認なしで実行されるもの                                                              | 最適な用途                      |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------ | :------------------------- |
| `default`                                                           | 読み取りのみ                                                                    | すべてのアクションを自分で確認する、機密性の高い作業 |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | 読み取り、ファイル編集、一般的なファイルシステム コマンド（`mkdir`、`touch`、`mv`、`cp` など）               | 確認中のコードを反復処理する             |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | 読み取り、および [auto モード](#eliminate-prompts-with-auto-mode) が利用可能な場合の分類器承認コマンド | コードベースを変更する前に探索する          |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | すべて、バックグラウンド安全性チェック付き                                                     | 長いタスク、プロンプト疲労の軽減           |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | 読み取りと事前承認ツール。プロンプトが表示されるものはすべて拒否                                          | ロックダウン CI とスクリプト           |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | すべて                                                                       | 分離されたコンテナと VM のみ           |

すべてのアクションを確認するモードは、CLI では **Manual** という名前で、`claude --help` では、VS Code および JetBrains 拡張機能では、デスクトップ アプリでは Manual という名前です。その設定値は `default` で、これは hooks と SDK 統合が使用するものです。CLI は、値を入力する場所ならどこでも `manual` をエイリアスとして受け入れます。例えば `claude --permission-mode manual` または `"defaultMode": "manual"` です。Manual ラベルと `manual` エイリアスには Claude Code v2.1.200 以降が必要です。デスクトップ アプリのラベルはお使いの CLI バージョンに依存しません。

[保護されたパス](#protected-paths) への書き込みは、`bypassPermissions` モードおよび bypass 権限が利用可能な plan モード セッション（つまり、[`bypassPermissions` をモード サイクルに含める](#switch-permission-modes) 方法で開始されたインタラクティブ ターミナル セッション）を除いて、自動承認されることはありません。

モードはベースラインを設定します。特定のツールを事前承認またはブロックするために、[権限ルール](/docs/ja/permissions#manage-permissions) を上に重ねます。拒否ルールは `bypassPermissions` を含むすべてのモードでブロックします。拒否ルールと確認ルールは、Claude が呼び出せる他のツールが少なくとも 1 つある限り、[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior) には適用されません。許可ルールは `bypassPermissions` では効果がありません。

<h3 id="actions-no-mode-auto-approves">
  どのモードも自動承認しないアクション
</h3>

Claude Code は、`bypassPermissions` を含むどのモードでも、以下を自動承認しません。各項目は、各モードで代わりに何が起こるかを説明するセクションにリンクしています。

* 明示的な [確認ルール](/docs/ja/permissions#manage-permissions) に一致するツール
* 組織が [`ask`](/docs/ja/mcp#organization-controls-on-connector-tools) に設定したコネクタ ツール（その設定が Claude Code に到達するセッション内）
* ユーザー インタラクションが必要なツール：組み込みの `AskUserQuestion` ツールと [`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool) でマークされた MCP ツール
* [重要なパス](#critical-paths) をターゲットとする `rm` および `rmdir` 削除。許可ルールまたは `PreToolUse` hook `"allow"` では承認されません
* [クロス セッション メッセージング セーフガード](#skip-all-checks-with-bypasspermissions-mode)
* [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories) がオンの場合、作業ディレクトリ外の読み取り：認識されたファイル読み取り Bash コマンドおよび auto モードおよび `bypassPermissions` モードでもサンドボックス外で実行するために承認が必要な [unsandboxed retry](/docs/ja/sandboxing#the-unsandboxed-retry-escape-hatch)。Claude Code v2.1.257 以降が必要です

  シェル パーサーが追跡できないコマンド（例えば、複数回ディレクトリを変更したり、サブシェルを実行したりするコマンド）は、外部パスを指定しない場合でも同じ方法でプロンプトが表示されます。このプロンプトは、コマンドが [sandbox](/docs/ja/sandboxing) で実行され、sandbox がブロックを強制する場合には適用されません。

<h2 id="common-setups">
  一般的なセットアップ
</h2>

権限モードは Claude がアクションの前に確認するかどうかを決定し、[Bash サンドボックス](/docs/ja/sandboxing)と外側の[隔離境界](/docs/ja/sandbox-environments)は、アクションが実行されると何に到達できるかを決定します。以下の各行は、目標をフラグまたは設定と、必要な隔離とペアにします。これは開始点です。[利用可能なモード](#available-modes)は、各モードでプロンプトなしで実行されるものをリストしています。

| 実現したいこと                 | 開始する                                                                                                                                             | 必要な隔離                                                                                                                                                    | 注記                                                                                                                                                                                  |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| すべてのアクションを自分でレビュー       | Manual モード：`claude --permission-mode default`                                                                                                    | なし                                                                                                                                                       | 機密作業、不慣れなコード                                                                                                                                                                        |
| ローカルで反復、分類器なしでプロンプトを減らす | Manual モード + [auto-allow モード](/docs/ja/sandboxing#sandbox-modes)の Bash サンドボックス：`claude --permission-mode default`、その後 `/sandbox` を実行して auto-allow を選択 | 組み込み Bash サンドボックス、macOS、Linux、WSL2 上                                                                                                                     | 拒否ルールは依然として適用され、`Bash(git push *)` のようなコマンドに名前を付ける ask ルールは依然としてプロンプトを表示します。代わりに設定ファイルからサンドボックスをオンにするには、[`sandbox.enabled`](/docs/ja/settings-reference#sandbox-enabled) を `true` に設定します |
| 何かを変更する前に探索             | `claude --permission-mode plan`                                                                                                                  | なし                                                                                                                                                       | Claude Code は[計画を承認](#review-and-approve-a-plan)するまで編集をブロックします                                                                                                                      |
| auto モードでハンズオフで作業       | `claude --permission-mode auto`、Pro、Max、Team の[組み込み開始権限モード](#which-mode-a-session-starts-in)                                                     | なし。サンドボックスまたはコンテナは防御の深さを追加                                                                                                                               | [サポートされているモデル](#eliminate-prompts-with-auto-mode)が必要で、組織は [auto モードをオフ](#eliminate-prompts-with-auto-mode)にできます                                                                     |
| 正確な許可リストで CI で実行        | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                | CI ランナーが提供するもの以外はなし                                                                                                                                      | [Web 上の Claude Code](/docs/ja/claude-code-on-the-web)は設定ファイルから `dontAsk` を無視します                                                                                                          |
| コンテナ内で完全に無人で実行          | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                            | 必須：コンテナ、VM、または[サンドボックスランタイム](/docs/ja/sandbox-environments#sandbox-runtime)。Linux と macOS では、[非 root ユーザー](#skip-all-checks-with-bypasspermissions-mode)として実行 | Web 上の Claude Code は設定ファイルからこのモードを無視します。この `-p` 実行では、[依然としてプロンプトが表示される少数の呼び出し](#skip-all-checks-with-bypasspermissions-mode)は代わりに拒否されます                                             |

Bash サンドボックスと auto モードは独立して機能し、[Sandbox modes](/docs/ja/sandboxing#sandbox-modes)の下にリストされている例外を除いて組み合わさります。完全な相互作用については、[サンドボックスが権限と権限モードにどのように関連するか](/docs/ja/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes)および[隔離が権限モードにどのように関連するか](/docs/ja/sandbox-environments#how-isolation-relates-to-permission-modes)を参照してください。

<h2 id="which-mode-a-session-starts-in">
  セッションが開始するモード
</h2>

ターミナルで新しいセッションを開始すると、Claude Code は最初に適用されるものから権限モードを取得します。

1. `--permission-mode` フラグ、または `--dangerously-skip-permissions`

2. [設定ファイル](/docs/ja/settings#where-settings-live)の `permissions.defaultMode`

   `.claude/settings.json` または `.claude/settings.local.json` で `"auto"` を設定した場合、値は有効にならず、Claude Code は `~/.claude/settings.json` からの `defaultMode` ではなく組み込みデフォルトを使用します。これらの 2 つのファイルで `"bypassPermissions"` を設定した場合、それも有効にならず、セッションは Manual モードで開始します。他の値はすべての設定ファイルから適用されます。

3. 組み込みデフォルト

VS Code 拡張機能が開始する会話は、[権限モードを切り替える](#switch-permission-modes)の拡張機能独自のリストに従います。Claude Code が再開されたセッションを開始する権限モードについては、[再開時の権限モード](/docs/ja/sessions#permission-mode-on-resume)を参照してください。

組み込み `auto` デフォルトには、macOS、Linux、WSL では Claude Code v2.1.228 以降が必要で、ネイティブ Windows では v2.1.233 以降が必要です。以前のバージョンでは、組み込みデフォルトは Manual です。

組み込みデフォルトは、Claude Code の実行方法、プラン、および Claude Code がフィーチャーフラグを取得できるかどうかに依存します。セッションに一致する最初の行が適用されます。表は、ターミナルまたは VS Code 拡張機能を通じて開始するセッションをカバーしています。デスクトップアプリと claude.ai については、[権限モードを切り替える](#switch-permission-modes)の Desktop と Web タブを参照してください。

| Claude Code の実行方法                                                                                                                                                                   | 組み込み開始権限モード |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------- |
| 設定ファイルが `disableAutoMode` を `"disable"` に設定                                                                                                                                         | `default`   |
| [フィーチャーフラグ取得](/docs/ja/env-vars#features-that-need-feature-flag-fetching)がオフ                                                                                                             | `default`   |
| [Claude Code をインストールまたはアップグレード](/docs/ja/env-vars#first-session-after-an-install-or-upgrade)した後の最初のセッション。このデフォルトを追加するバージョンへ。ただし、新規インストール後、Claude Code がフラグを時間内に取得した場合を除く                 | `default`   |
| `claude -p` または [Agent SDK](/docs/ja/agent-sdk/permissions)                                                                                                                              | `default`   |
| Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、[Claude Platform on AWS](/docs/ja/claude-platform-on-aws)、またはサインイン済みの [Claude apps gateway](/docs/ja/claude-apps-gateway)セッション | `default`   |
| Pro、Max、または Team プラン。ターミナルまたは [VS Code 拡張機能](/docs/ja/vs-code)を通じて                                                                                                                       | `auto`      |
| Enterprise プランまたは Claude Console API キー                                                                                                                                             | `default`   |

フィーチャーフラグ取得がオフの場合、または [インストールまたはアップグレード後の最初のセッション](/docs/ja/env-vars#first-session-after-an-install-or-upgrade)でフラグがまだ到達していない場合、VS Code 拡張機能は開始権限モードを選択するときにすべての設定ファイルを無視します。

フラグ、設定ファイル、または組み込みデフォルトが `auto` を選択しても、auto モードがセッションで利用できない場合、Claude Code はセッションを Manual で開始します。Auto モードは、セッションが [利用可能性要件](#eliminate-prompts-with-auto-mode)を満たさない場合（設定ファイルがそれをオフにするか、サポートしていないモデルなど）、または Anthropic がサーバー側で一時的にそれをオフにした場合に利用できません。

組み込みデフォルトが初めてセッションを auto モードで開始するとき、Claude Code はこのページにリンクする通知を表示します。

* ターミナルでは、セッションの上部に 1 回
* VS Code 拡張機能では、新しい会話画面のカードとして、却下するまで表示されます

Pro、Max、Team プランでは、`~/.claude/settings.json` が `auto` 以外の `defaultMode` を設定し、他の設定ファイルが設定しない場合、セッションはそのモードで開始し続けます。Claude Code はターミナルまたは VS Code 拡張機能で 1 回、設定を auto モードに変更するかどうかを尋ねます。却下した場合、設定はそのままです。

<h3 id="start-in-a-different-mode">
  異なる権限モードで開始する
</h3>

1 つのセッション、マシン上のすべてのセッション、プロジェクト内、または組織内のすべてのセッションの開始権限モードを設定できます。複数の設定ファイルが `permissions.defaultMode` を設定する場合、[設定の優先順位](/docs/ja/settings#settings-precedence)が決定するため、プロジェクトまたは管理値は `~/.claude/settings.json` より優先されます。実行中のセッションの権限モードを変更するには、[権限モードを切り替える](#switch-permission-modes)を参照してください。

| 開始権限モードを設定する対象                | これを実行                                                                                                                                                                                                                                                                                  |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 開始しようとしている 1 つのセッション          | 権限モードをフラグとして渡します。例えば `claude --permission-mode default`                                                                                                                                                                                                                                |
| このマシンで開始するすべてのターミナルセッション      | `~/.claude/settings.json` で `permissions.defaultMode` を設定します。VS Code 拡張機能が読み取る内容については、[権限モードを切り替える](#switch-permission-modes)を参照してください                                                                                                                                                 |
| 1 つのプロジェクトで開始するすべてのターミナルセッション | プロジェクトの `.claude/settings.json` で `permissions.defaultMode` を設定します。ターミナルで開始するセッションは `auto` と `bypassPermissions` を除くすべての値を尊重します。VS Code 拡張機能が開始するセッションはプロジェクト設定を開始権限モードに読み込みません                                                                                                        |
| 組織内のすべてのターミナルセッション            | [管理設定](/docs/ja/managed-settings)で `permissions.defaultMode` を設定します。ターミナルセッションはそのモードで開始し、ユーザーは依然として auto モードに切り替えることができます。VS Code 拡張機能が読み取る内容については、[権限モードを切り替える](#switch-permission-modes)を参照してください。auto モードを削除してユーザーが選択できないようにするには、代わりに `permissions.disableAutoMode` を `"disable"` に設定します |

この例は、マシン上のすべてのターミナルセッションを Manual モード（その設定値は `default`）で開始するようにします。`~/.claude/settings.json` に保存します。

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

次に開始するセッションは、ステータスバーに `⏸ manual mode on` を表示します。

<h2 id="switch-permission-modes">
  権限モードを切り替える
</h2>

各インターフェースには、セッション中に権限モードを切り替えるための独自のコントロールと、新しいセッションが開始する権限モードを選択するための独自の方法があります。インターフェースを選択して、そのコントロールを確認してください。

<Tabs>
  <Tab title="CLI">
    **セッション中**：`Shift+Tab` を押して権限モードをサイクルします。`auto` から、最初のプレスは `default` に切り替わり、サイクルは `default` → `acceptEdits` → `plan` → `default` に戻ります。以下で説明されるオプションモードは `plan` の後にスロットインします。ステータスバーはアクティブなモードを、`default` の場合はグレーの `⏸ manual mode on`、または `⏵⏵ accept edits on`、`⏸ plan mode on`、`⏵⏵ auto mode on`、`⏵⏵ don't ask on`、または `⏵⏵ bypass permissions on` として表示します。

    すべてのモードがデフォルトサイクルに含まれるわけではありません。

    * `auto`：[auto モードが利用可能](#eliminate-prompts-with-auto-mode)な場合に表示されます。auto へのサイクルは確認プロンプトなしで権限モードを切り替えます
    * `bypassPermissions`：`--permission-mode bypassPermissions`、`--dangerously-skip-permissions`、`--allow-dangerously-skip-permissions`、または [ユーザー、`--settings`、または管理設定](/docs/ja/settings-reference#permissions-defaultmode)の `permissions.defaultMode: "bypassPermissions"` で開始した後に表示されます。`--allow-` バリアントはモードをサイクルに追加しますが、アクティブ化しません
    * `dontAsk`：サイクルに表示されることはありません。`--permission-mode dontAsk` で設定します

    有効なオプションモードは `plan` の後にスロットインし、`bypassPermissions` が最初で `auto` が最後です。両方が有効な場合、`bypassPermissions` から `auto` へのサイクルを通過します。

    **Bash 権限プロンプトから**：Manual と `acceptEdits` 権限モードで、[auto モード](#eliminate-prompts-with-auto-mode)が利用可能な場合、Claude Code は Bash コマンドの権限プロンプトに **Yes, and switch to auto mode** を追加します。それを選択してコマンドを承認し、セッションを auto モードに切り替えます。[PowerShell ツール](/docs/ja/tools-reference#powershell-tool)プロンプトはオプションを提供しません。Claude Code v2.1.247 以降が必要です。

    Claude Code は、[`ask` ルール](/docs/ja/permissions#manage-permissions)の 1 つによって強制されたプロンプト、または [フック](/docs/ja/hooks#pretooluse-decision-control)によるプロンプトにはオプションを追加しません。auto モードは依然としてそれらのプロンプトを表示するため、切り替えてもそれらは削除されません。

    **起動時**：権限モードをフラグとして渡します。

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **デフォルトとして**：[異なる権限モードで開始する](#start-in-a-different-mode)で説明されているように、必要なスコープで `permissions.defaultMode` を設定します。

    同じ `--permission-mode` フラグは [非対話的実行](/docs/ja/headless)用に `-p` で機能します。
  </Tab>

  <Tab title="VS Code">
    **セッション中**：プロンプトボックスの下部にあるモード指示器をクリックします。このページのモードに対して以下のラベルを使用します。

    | UI ラベル             | モード                 |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    **デフォルトとして**：権限モード会話が開始するようにピンするには、VS Code ユーザー設定で `claudeCode.initialPermissionMode` を `default`、`manual`、`acceptEdits`、`plan`、または `bypassPermissions` に設定します。設定は `auto` を受け入れません。Auto で開始するには、設定を解除したままにして、以下の項目 2 で説明されているようにモード指示器から **Auto** を 1 回選択します。拡張機能は以下の最初に適用されるものでそれぞれの新しい会話を開始します。

    1. `claudeCode.initialPermissionMode`
    2. モード指示器から最後に選択したモード（Manual、Edit automatically、または Auto の場合）。Plan または Bypass permissions を選択すると、その会話のみに適用されます
    3. [管理設定](/docs/ja/managed-settings)または `~/.claude/settings.json` からの `permissions.defaultMode`。Pro、Max、Team プランで [フィーチャーフラグ取得](#which-mode-a-session-starts-in)が利用可能な場合
    4. プラン、プロバイダー、組織設定の[組み込みデフォルト](#which-mode-a-session-starts-in)

    拡張機能は開始権限モードのためにプロジェクトの `.claude/settings.json` または `.claude/settings.local.json` を読み込むことはなく、項目 3 の条件を満たさない会話ではすべての設定ファイルを読み込みません。`claudeCode.claudeProcessWrapper` が設定されている場合、項目 3 と 4 も適用されません。それらの会話は、項目 1 または項目 2 が権限モードを設定しない限り Manual で開始します。

    Auto は [auto モードが利用可能](#eliminate-prompts-with-auto-mode)な場合、モード指示器に表示されます。

    Bypass permissions には拡張機能設定の **Allow dangerously skip permissions** トグルが必要です。それなしでは、権限モードは指示器に表示されず、項目 1 または項目 3 からの `bypassPermissions` 値は会話を Manual で開始します。同様に、auto モードが利用できない場合、項目からの Auto は会話を Manual で開始します。

    拡張機能固有の詳細については、[VS Code ガイド](/docs/ja/vs-code)を参照してください。
  </Tab>

  <Tab title="JetBrains">
    JetBrains プラグインは IDE ターミナルで Claude Code を実行するため、権限モードの切り替えは CLI と同じように機能します。`Shift+Tab` を押してサイクルするか、起動時に `--permission-mode` を渡します。
  </Tab>

  <Tab title="Desktop">
    **セッション中**：Code タブで、送信ボタンの横にあるモードセレクターを使用します。すべてのモードがセレクターに表示されるわけではありません。

    * **Auto**：[auto モードが利用可能](#eliminate-prompts-with-auto-mode)な場合に表示されます
    * **Bypass permissions**：Pro と Max プランの Desktop 設定で **Allow bypass permissions mode** トグルが必要です。Team と Enterprise プランでは、組織ポリシーがそれを制御します

    Cowork タブはこれらのモードを使用しません。Cowork には独自の権限モードがあり、別々に有効化され、Cowork タブはアカウントのデフォルトを超えるモードが有効化されるまでモードセレクターを表示しません。[Cowork ドキュメント](https://claude.com/docs/cowork/overview)を参照してください。

    Desktop 固有の詳細については、Desktop ガイドの [権限モードを選択する](/docs/ja/desktop#choose-a-permission-mode)を参照してください。

    **デフォルトとして**：[設定](/docs/ja/settings#where-settings-live)で `defaultMode` を設定します。デスクトップアプリは CLI と同じ設定ファイルを読み取り、新しいローカルセッションに権限モードを適用します。

    モードセレクターで選択したモードはフォルダごとに記憶され、そのフォルダの `defaultMode` より優先されます。Plan は例外です。それを選択すると現在のセッションのみに適用されます。

    `defaultMode` が設定ファイルのどこに行くかについては、[異なる権限モードで開始する](#start-in-a-different-mode)の例を参照してください。
  </Tab>

  <Tab title="Web and mobile">
    [claude.ai/code](https://claude.ai/code) のプロンプトボックスの横またはモバイルアプリのモードドロップダウンを使用します。権限プロンプトは承認のために claude.ai に表示されます。どのモードが表示されるかはセッションが実行される場所によります。

    * **[Claude Code on the web](/docs/ja/claude-code-on-the-web)のクラウドセッション**：Accept edits、Plan、Auto。Accept edits は `default` モードに対応します。クラウドセッションはモードに関係なくファイル編集を事前承認するため、ドロップダウンは Manual の代わりに Accept edits を表示します。設定からの `defaultMode: "acceptEdits"` は依然として尊重されます。Auto モードは組織がそれを許可し、選択されたモデルがそれをサポートする場合にのみ表示されます。Bypass permissions は利用できません。
    * **ローカルマシンの [Remote Control](/docs/ja/remote-control)セッション**：Manual、Accept edits、Plan。アプリから Auto または Bypass permissions を選択することはできません。
      * Bypass permissions を除き、ドロップダウンはローカルセッションが実行されているモードを表示します。これにはターミナルから設定されたモードが含まれ、アプリまたはターミナルでモードが変更されると更新されます。セッションは Bypass permissions を claude.ai に報告することはないため、ターミナルからそれに切り替えてもドロップダウンに表示される内容は変わりません。
      * [デスクトップアプリ](/docs/ja/desktop)または [VS Code 拡張機能](/docs/ja/vs-code)でホストされるセッションは、アプリで発生するのと同じように権限モード変更を claude.ai に報告します。
      * v2.1.202 より前では、`/remote-control` または `claude --remote-control` で接続されたセッションはモードをまったく報告しなかったため、claude.ai とモバイルアプリはセッションが実行されていないモードを表示する可能性がありました。不一致はラベルのみに影響しました。Claude Code は権限プロンプトをセッションの実際のモードから生成し、それらは依然としてアプリに表示されて承認されました。

    Remote Control の場合、ローカルマシンを実行するセッションは claude.ai アカウントでサインインする必要があります。API キーはサポートされていません。ローカルセッションを起動するときに開始権限モードを設定することもできます。

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  acceptEdits モードでファイル編集を自動承認する
</h2>

`acceptEdits` モードでは Claude はプロンプトなしに作業ディレクトリ内のファイルを作成および編集できます。このモードがアクティブな間、ステータスバーは `⏵⏵ accept edits on` を表示します。

ファイル編集に加えて、`acceptEdits` モードは一般的なファイルシステム Bash コマンドを自動承認します。`mkdir`、`touch`、`rm`、`rmdir`、`mv`、`cp`、`sed`。これらのコマンドは `LANG=C` または `NO_COLOR=1` のような安全な環境変数、または `timeout`、`nice`、`nohup` のようなプロセスラッパーでプレフィックスされた場合にも自動承認されます。ファイル編集と同様に、自動承認は作業ディレクトリまたは `additionalDirectories` 内のパスにのみ適用されます。そのスコープ外のパス、[保護されたパス](#protected-paths)への書き込み、`rm` と `rmdir` の削除が[重要なパス](#critical-paths)をターゲットにしている場合、および [読み取り専用コマンド](/docs/ja/permissions#read-only-commands)の組み込みセットを除くその他すべての Bash コマンドはまだプロンプトが表示されます。

[PowerShell ツール](/docs/ja/tools-reference#powershell-tool)が有効な場合、`acceptEdits` モードはスコープ内のパスに対して `Set-Content`、`Add-Content`、`Clear-Content`、`Remove-Item` も自動承認し、それらの一般的なエイリアスも承認します。同じスコープと保護されたパスのルールが適用され、`Remove-Item` は[独自のチェック](#remove-item-in-powershell)を取得します。`Set-Content .\notes.txt "It's done"` のようなアポストロフィを含む引用符を含む位置引数は、Claude Code が引用符付きと引用符なしの読み取りが異なるため、スコープ内のパスでもプロンプトが表示されます。`-Value` のような名前付きパラメーターを通じてコンテンツを渡してプロンプトを回避します。

事実後にエディターまたは `git diff` 経由で変更をレビューしたい場合、各編集をインラインで承認するのではなく `acceptEdits` を使用します。

Manual モードから `Shift+Tab` を 1 回押して入るか、直接開始します。

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  プランモードで編集前に分析する
</h2>

プランモードは Claude に変更を研究して提案するよう指示しますが、実際には変更を加えません。Claude はファイルを読み込み、シェルコマンドを実行して探索し、プランを作成しますが、ソースを編集しません。[bypass 権限が利用可能](#skip-all-checks-with-bypasspermissions-mode)なインタラクティブターミナルセッションを除き、編集はプランを承認するまでブロックされたままです。

[オートモード](/docs/ja/auto-mode-config)が利用可能で、デフォルトでオンになっている `useAutoModeDuringPlan` 設定がオンの場合、分類器はプランニング中にシェルコマンドをレビューしてプロンプトを表示する代わりに確認します。承認されたコマンドは実行され、拒否されたコマンドはブロックされます。それ以外の場合、[組み込みの読み取り専用セット](/docs/ja/permissions#read-only-commands)外のコマンドはサンドボックスの[オートアロウモード](/docs/ja/sandboxing#sandbox-modes)が有効な場合を含めて承認を求めるプロンプトが表示されます。bypass 権限が利用可能なインタラクティブターミナルセッションでは、分類器もプロンプトもプランニングコマンドには適用されません。[bypassPermissions モードですべてのチェックをスキップ](#skip-all-checks-with-bypasspermissions-mode)は、そこでもまだプロンプトが表示される少数のものをカバーしています。v2.1.212 から v2.1.217 では、bypass 権限のないセッションは、オートモードが利用可能かどうかに関わらず、読み取り専用セット外のすべてのコマンドについてプロンプトを表示していました。

プランモードに入るには、`Shift+Tab` を押すか、単一のプロンプトに `/plan` を付けます。CLI からプランモードで開始することもできます。

```bash theme={null}
claude --permission-mode plan
```

`Shift+Tab` をもう一度押してプランを承認せずにプランモードを終了します。

<h3 id="review-and-approve-a-plan">
  プランをレビューして承認する
</h3>

プランの準備ができたら、Claude はそれを提示し、どのように進めるかを尋ねます。そのプロンプトから以下を選択できます。

* **はい、オートモードを使用する**: 承認して[オートモード](#eliminate-prompts-with-auto-mode)で開始します。オートモードがセッションで[利用可能でない](#eliminate-prompts-with-auto-mode)場合（例えば、組織がそれをオフにした場合）、このオプションは**はい、編集を自動受け入れ**と表示されます。bypass 権限を有効にしてセッションを開始した場合、オプションは代わりに**はい、このセッションで BYPASS PERMISSIONS（以降プロンプトなし）に切り替える**と表示されます。
* **はい、編集を手動で承認する**: 承認して各編集を個別にレビューします。
* **いいえ、プランニングを続ける**: プランモードにとどまり、Claude に何を変更するかを伝えます。

プランを承認するとプランモードを終了し、セッションを各承認オプションが説明する権限モードに切り替えるため、Claude は編集を開始します。再度プランを立てるには、`Shift+Tab` でプランモードに戻すか、次のプロンプトに `/plan` を付けます。

`Ctrl+G` を押して、提案されたプランをデフォルトのテキストエディタで開き、Claude が進める前に直接編集します。[`showClearContextOnPlanAccept`](/docs/ja/settings-reference#showclearcontextonplanaccept)が有効な場合、リストはプランを承認してプランニングコンテキストをクリアする最初のオプションを取得します。

プランを受け入れると、セッションはプランに基づいて[生成されたタイトル](/docs/ja/sessions#name-your-sessions)も取得します。ただし、セッションに既に名前を付けている場合を除きます。

<h3 id="set-plan-mode-as-the-default">
  プランモードをデフォルトとして設定する
</h3>

プロジェクトのターミナルセッションのデフォルトをプランモードにするには、`.claude/settings.json` で `defaultMode` を `plan` に設定します。これは[別の権限モードで開始](#start-in-a-different-mode)の下の例として配置されます。[VS Code 拡張機能](/docs/ja/vs-code)が開始する会話は、開始権限モードのプロジェクト設定を読み込みません。そこで、VS Code ユーザー設定で `claudeCode.initialPermissionMode` を `plan` に設定してください。

<h2 id="eliminate-prompts-with-auto-mode">
  権限プロンプトを自動モードで排除する
</h2>

自動モードを使用すると、Claude は日常的な権限プロンプトなしで実行できます。別のクラシファイアモデルが実行前にアクションをレビューし、リクエストを超えるエスカレーション、認識されていないインフラストラクチャをターゲットにする、または Claude が読んだ敵対的なコンテンツによって駆動されているように見えるものをブロックします。明示的な[質問ルール](/docs/ja/permissions#manage-permissions)は依然としてプロンプトを強制します。

Pro、Max、Team プランでは、自動モードは[セッションが開始される組み込みの開始権限モード](#which-mode-a-session-starts-in)です。

クラシファイアは、Claude が [`SendMessage`](/docs/ja/tools-reference) で別のエージェントに送信する各メッセージもレビューします。プレーンテキストまたは構造化された[エージェントチーム](/docs/ja/agent-teams)メッセージのいずれかで、自動モードと[クラシファイアがコマンドをレビューしている間のプランモード](#analyze-before-you-edit-with-plan-mode)の両方で、Claude Code がそれを配信する前にレビューします。送信レビューには Claude Code v2.1.222 以降が必要です。

クラシファイアは、`rm` と `rmdir` の削除もレビューして承認またはブロックします。これは[重要なパス](#critical-paths)をターゲットにしています。例えば `rm -rf /` と `rm -rf ~` のように、削除がコマンドまたはプロセス置換内にある場合も含みます。

自動モードはまた、Claude が明確な質問のために停止することなく作業を続けるよう促します。ただし、Claude はプロンプトまたはスキルが明示的にそれに依存している場合は依然として質問します。モードでプロンプトを表示しながらより強力な自律動作を実現するには、代わりに[プロアクティブ出力スタイル](/docs/ja/output-styles)を設定します。

<Warning>
  自動モードは権限プロンプトを減らしますが、安全性を保証しません。一般的な方向を信頼するタスクに使用してください。機密操作のレビューの代わりではありません。
</Warning>

自動モードは、アカウントがこれらすべての要件を満たす場合にのみ利用可能です。

* **プラン**: すべてのプラン。
* **組織**: Team と Enterprise では、自動モードはデフォルトで利用可能です。管理者は、[管理設定](/docs/ja/managed-settings)で `permissions.disableAutoMode` を `"disable"` に設定することで、組織の自動モードをオフにできます。
* **モデル**: Anthropic API と[AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws)では、Claude Opus 4.6 以降、Sonnet 4.6 以降、または[Fable モデル](/docs/ja/model-config#work-with-fable)。Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、およびサインイン済みの[Claude apps gateway](/docs/ja/claude-apps-gateway)セッションでは、Claude Sonnet 5、Opus 4.7 以降、および Fable モデルのみです。Sonnet 4.5、Opus 4.5、Haiku、claude-3 モデルを含む古いモデルは、どのプロバイダーでもサポートされていません。
* **プロバイダー**: Anthropic API、AWS 上の Claude Platform、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、およびサインイン済みの Claude apps gateway セッションでデフォルトで利用可能です。

Claude Code が自動モードを利用不可と報告する場合は、まずこれらの要件を確認し、設定ファイルが [`disableAutoMode`](/docs/ja/settings-reference#disableautomode) を設定しているかどうかを確認してください。Anthropic がサーバー側で自動モードをオフにしたか、サーバーがアカウントの自動モードを拒否した可能性があります。どちらかの回答を受け取ったセッションは、セッションが終了するまで自動モードをオフのままにするため、後で新しいセッションを開始してください。

モデルに名前を付けて、自動モードがアクションの安全性を「判定できない」と言う別のメッセージは、クラシファイアリクエストが失敗したことを意味します。その失敗は通常は一時的ですが、Amazon Bedrock では、アカウントが指定されたモデルを呼び出せるようになるまで繰り返される可能性があります。原因と対処方法については、[エラーリファレンス](/docs/ja/errors#auto-mode-cannot-determine-the-safety-of-an-action)を参照してください。

[設定](/docs/ja/settings-reference#all-settings)で `defaultMode: "auto"` を設定し、ターミナルセッションがエラーなしで Manual モードで開始される場合、設定は `.claude/settings.json` または `.claude/settings.local.json` にある可能性があります。`auto` はこれらのファイルから有効になりません。`~/.claude/settings.json` に移動してください。VS Code 拡張機能が開始した会話の場合は、代わりに[権限モードの切り替え](#switch-permission-modes)で拡張機能自体のリストを確認してください。

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Bedrock、Agent Platform、または Foundry での自動モード
</h3>

[Amazon Bedrock](/docs/ja/amazon-bedrock)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、[Microsoft Foundry](/docs/ja/microsoft-foundry)、およびサインイン済みの[Claude apps gateway](/docs/ja/claude-apps-gateway)セッションでは、自動モードはデフォルトで `Shift+Tab` サイクルに表示されます。サイクルに表示されることは、セッションが開始される権限モードを変更しません。これらのプロバイダーでは、ターミナルセッションは [`defaultMode`](/docs/ja/settings-reference#permissions-defaultmode)で開始されます。これは変更しない限り Manual です。[VS Code 拡張機能](/docs/ja/vs-code)の会話は、`claudeCode.initialPermissionMode` または拡張機能で選択したモードが設定しない限り Manual で開始されます。これらのプロバイダーでは、Claude Sonnet 5、Opus 4.7 以降、および Fable モデルのみがサポートされています。

自動モードをデフォルトの開始権限モードにするには、ユーザーまたは管理設定で `"permissions": {"defaultMode": "auto"}` を設定します。VS Code 拡張機能が開始するセッションでは、モード指示器から **Auto** を選択します。[権限モードの切り替え](#switch-permission-modes)は、その選択を上回るものをカバーしています。

[`/doctor`](/docs/ja/commands#all-commands)チェックアップは、Anthropic API と同じ方法で、これらのプロバイダーのユーザー設定デフォルトを提案します。

開発者が自動モードを使用するのを防ぐには、[管理設定](/docs/ja/managed-settings)で `disableAutoMode` を `"disable"` に設定します。これにより `auto` が `Shift+Tab` サイクルから削除され、`--permission-mode auto` で開始されたセッションは Manual で開始されます。既に自動モードで実行されているセッションは、設定が[管理者がデプロイしたソース](/docs/ja/managed-settings#which-managed-source-claude-code-uses)からそのセッションに到達すると、自動モードを離れ、`auto mode disabled by settings` を表示します。v2.1.251 より前では、実行中のセッションは終了するまで自動モードを保持していました。

v2.1.158 から v2.1.206 では、これらのプロバイダーで自動モードはオフでした。`CLAUDE_CODE_ENABLE_AUTO_MODE=1` を設定するまで、Claude Code はこれらのプロバイダーで `defaultMode: "auto"` を無視していました。変数は互換性のために依然として受け入れられ、v2.1.207 以降は効果がありません。

<h3 id="server-side-classifier-review">
  サーバー側クラシファイアレビュー
</h3>

自動モードでは、Claude Code はサーバーに[決定順序](#how-the-classifier-evaluates-actions)が送信するアクションをレビューするよう要求できます。これはセッションのモデルリクエストの一部として行われ、Claude Code 独自のクラシファイアリクエストの代わりに行われます。これらのセッションは以下を要求します。

* **Anthropic API への直接接続**: インタラクティブターミナルセッションで、すべての claude.ai プランおよび Claude API を使用するアカウントで、Anthropic がロールアウトするにつれて。Pro、Max、Team プランでは Claude Code v2.1.271 以降が必要で、Enterprise プランおよび Claude API アカウントでは v2.1.278 以降が必要です。v2.1.282 から、[フィーチャーフラグをフェッチしない](/docs/ja/env-vars#features-that-need-feature-flag-fetching)セッション（例えば、テレメトリをオフにしたため）は、あらゆる種類のセッションでデフォルトでサーバーに要求します。
* **クラウドプロバイダー、または LLM ゲートウェイまたはプロキシ**: [AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws)、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、および `ANTHROPIC_BASE_URL` を[LLM ゲートウェイまたはプロキシ](/docs/ja/llm-gateway)に指す場合、プランに関係なく。デフォルトでサーバーに要求することには Claude Code v2.1.278 以降が必要です。
* **サインイン済みの[Claude apps gateway](/docs/ja/claude-apps-gateway)セッション**: Claude Code v2.1.280 以降が必要です

サーバーがアクションをレビューする場所では、その判定がこれらのアクションを決定します。他に 2 つの結果が考えられます。

* **サーバーがセッションをレビューしない**: レスポンスがレビュー結果なしで完了するか、サーバーがこのセッションをレビューしないと答えます。最も一般的な原因は、LLM ゲートウェイまたはプロキシがレビューのリクエストまたは結果をドロップすることと、プラットフォーム、リージョン、または認証情報がまだサーバー側チェックを持たないことです。Claude Code は独自のクラシファイアリクエストにフォールバックします。そのフォールバックがセッションの残りの間保持されると、これらのリクエストが請求されるアカウントで[クラシファイアリクエスト料金に関する通知](/docs/ja/auto-mode-classifier-billing)を表示します。
* **サーバーがアクションに対して判定を出さない**: Claude Code はアクションを拒否し、レビューなしで実行しません。あらゆる接続で、これはレスポンスがレビュー結果の到着前に終了するか、結果が Claude Code が読み取れない形式で到達する場合に発生します。レスポンスを短縮またはリライトする LLM ゲートウェイまたはプロキシはどちらかを引き起こす可能性があります。Anthropic API への直接接続では、サーバーのチェックがアクション（例えば、タイムアウト）に失敗する場合にも発生します。[サーバーが安全判定を返さなかった](/docs/ja/errors#the-server-returned-no-safety-verdict)は拒否メッセージ、拒否が繰り返される場合に何が起こるか、および対処方法をカバーしています。

サーバーに質問することをスキップして、常に Claude Code 独自のクラシファイアリクエストを使用するには、[`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/ja/env-vars)を設定します。Anthropic API への直接接続では、変数には Claude Code v2.1.281 以降が必要です。`1` に設定すると、そこでサーバーレビューをオンにします。セッションがまだそれを持たない場合（例えば、`-p` または Agent SDK セッション）。`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` も設定していない限り。`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` を設定し、`CLAUDE_CODE_AUTO_MODE_SERVER` を設定しないままにする場合、Claude Code もサーバーに質問することを停止します。

<h3 id="what-the-classifier-blocks-by-default">
  クラシファイアがデフォルトでブロックするもの
</h3>

クラシファイアは、作業ディレクトリとセッション開始時にそれに対して構成されたリモートを信頼します。セッション中に `git remote add` または `git remote set-url` で追加またはリポイントされたリモートは信頼されず、[信頼できるインフラストラクチャを構成](/docs/ja/auto-mode-config)するまで、他のすべてが外部として扱われます。v2.1.200 より前では、セッション中に追加されたリモートも信頼されていました。

**デフォルトでブロック**:

* `curl | bash` のようなコードのダウンロードと実行
* 外部エンドポイントへの機密データの送信
* 本番環境へのデプロイとマイグレーション
* クラウドストレージでの大量削除
* IAM またはリポジトリ権限の付与
* 共有インフラストラクチャの変更
* セッション前に存在していたファイルの不可逆的な破壊
* Force push
* 実行時にシークレットまたは機密データをリポジトリの外に送信するか、デプロイが公開するものを拡大する変更のコミットまたはプッシュ。これは、シークレットをまだ受け取っていない宛先にシークレットを渡す CI ワークフローまたはデプロイ構成、シークレットストアを読み取ってデータを送信するスクリプトまたはセットアップステップ、およびデプロイが公開するものを拡大する構成変更（レジストリ、可視性、アーティファクト、またはソースマップ設定など）をカバーします。チェックはすべてのブランチに適用され、リポジトリが公開されている場合でも適用され、コミットまたはプッシュがパイプラインをトリガーするかどうかに関係なく、コミットまたはプッシュされるときに発火します。クリアするには、コミットまたはプッシュだけでなく、実行効果に名前を付ける必要があります。v2.1.211 より前では、このチェックはデフォルトブランチにスコープされていました。そこへのプッシュは、機密コンテンツを含む場合、要求したものに対して隠蔽または誤表示されたコンテンツ、リポジトリの外からポーティングされたコンテンツ、またはリクエストしたレビューの周りをルーティングされたコンテンツを含む場合にブロックされました
* `git reset --hard`、`git checkout -- .`、`git restore .`、`git clean -fd`、`git stash drop`、または `git stash clear`。クラシファイアはこれらがコミットされていない変更を破棄すると推定します
* `git commit --amend`。HEAD のコミットがこのセッションで作成されていない場合
* v2.1.198 から、HEAD のコミットが既にプッシュされている場合の `git commit --amend`。メッセージのみの言い換えはブロックされません。`--amend -m`。新しくステージされたものはなく、Claude がこのセッション中に作成したコミット上
* `terraform destroy`、`pulumi destroy`、`cdk destroy`、または `terragrunt destroy`。リソースを破壊するプランの適用

Claude Code v2.1.195 以降は、デフォルトでより多くのカテゴリをブロックします。いくつかは、機密リモートターゲットや保護された IaC スコープなど、具体的な名前に絞ることができる[環境](/docs/ja/auto-mode-config#define-trusted-infrastructure)エントリに依存しています。

* シークレットマネージャーへの書き込み、または DNS レコードまたは TLS 証明書の変更
* 人間が承認していないプルリクエストのマージ、Claude 自身のプルリクエストの承認、または CI チェックの無効化
* `atlantis apply` や bot の `/deploy` や `/merge` などの自動化へのコマンド自体であるコメントの投稿
* 本番環境機能フラグの切り替え、ランプ、または削除
* 保護された IaC スコープへのインフラストラクチャ変更の適用、またはクラスタノードのドレインと削除
* ラベルセレクタや `--all` など、他のユーザーのジョブをキャッチする、指定したリソースを超えて到達する共有コンピュートクラスタへの書き込み
* すべてのノードで実行されるか、クラスタトラフィックをインターセプトする Kubernetes リソースの作成（DaemonSets や admission webhooks など）
* 機密リモートターゲットへのインタラクティブシェルまたはポートフォワード
* ローカルサービスをパブリックインターネットから到達可能にするトンネルまたはリバースシェルの開設
* ライブ認証情報またはトークンをトランスクリプトまたはファイルに出力
* [環境](/docs/ja/auto-mode-config#define-trusted-infrastructure)で機密データロケーションとしてリストされている場所へのアクセス、またはそこからのデータのコピー。v2.1.198 以降、これはエントリが除外する対象者にそこからデータを送信することもブロックします
* 内部パッケージレジストリの周りにパッケージインストールをルーティングしてパブリックレジストリに送信。v2.1.198 以降、これは会話で内部レジストリまたはミラーが存在することを Claude に伝えた場合にも適用されます。環境にリストされている場合だけではなく
* `--insecure` のような安全ガードを解除するフラグでコマンドを実行
* `--dangerously-skip-permissions` または `--no-sandbox` で開始されたものなど、人間の承認またはサンドボックスなしで実行される自律エージェントループの起動。v2.1.198 以降、これは `--yes-always` で開始されたランナーなど、分離とアクション単位の承認を無効にして、サードパーティエージェントまたは eval ハーネスを実行することもカバーします
* [Chrome の Claude](/docs/ja/chrome)ブラウザアクション。ページコンテンツ、クッキー、または認証情報をオリジン外に送信する可能性があります

Claude Code v2.1.198 以降もこれらをデフォルトでブロックします。

* ワイルドカード、glob、または年齢フィルタではなく、特定の名前付きパスによって `/tmp`、`$TMPDIR`、または別の共有スクラッチまたはキャッシュディレクトリ内のファイルを削除
* 独自のメッセージがそれらの詳細をその受信者に対して認可しなかった場合、送信、アップロード、公開、または他の人または共有システムに書き込まれるコンテンツに機密詳細を含める。PR および issue 本文、コミットメッセージ、およびコメントは、リポジトリが信頼境界の外にあるか公開されている場合、組織自体の公開リポジトリを含む、この種のアウトバウンドコンテンツとしてカウントされます。内部ファイルパス、コード名、ライブ API レスポンスデータ（メールやアカウント識別子など）、およびインフラストラクチャ識別子は機密詳細としてカウントされます。PR、issue、およびコミットメッセージのスコープには Claude Code v2.1.200 以降が必要です。PR または issue 本文内のライブ個人データ（メールアドレス、アカウントまたは組織識別子、使用メトリックなど）には、リポジトリの可視性または信頼境界に関係なく、それらの詳細と受信者に名前を付ける必要があります。そのチェックには Claude Code v2.1.203 以降が必要です
* Claude Code 自体の tmux ペインにキーストロークを送信して、独自のインターフェースを駆動。クラシファイアはこれを Claude が独自の権限または監視を変更することとして扱います

Claude Code v2.1.200 以降もこれらをデフォルトでブロックします。

* セキュリティ動作を保護するテストまたはアサーション（認証、アクセス制御、入力検証、またはサンドボックスなど）をコメントアウト、削除、または force-pass
* セッションで Claude が作成しなかった stateful リソースを削除またはティアダウン。より具体的な削除ルールが適用されず、そのリソースに名前を付けなかった場合
* API ベース URL、プロキシエンドポイント、webhook レシーバー、またはレジストリミラーを、タスクに適さないサードパーティホスト（`.env.example` のようなサンプルファイルを含む）にリポイント
* `git remote set-url` または `git remote add` で pushes の行き先を変更。新しいリモートに名前を付けない限り
* 公開されていることが知られているリポジトリにシークレットまたは個人または信頼されたデータをプッシュ、またはそのリポジトリ自体の作業の一部ではない機密資料をそこにプッシュ。dotfiles リポジトリ自体の主題は個人または信頼されたデータの唯一の例外であり、プライベートリポジトリからのコンテンツが公開サーフェスに到達することは同じ方法でブロックされます。両方の改善には Claude Code v2.1.203 以降が必要です。v2.1.203 より前では、個人データは機密資料とグループ化され、そのリポジトリ自体の作業の一部ではない場合にのみブロックされました。リポジトリの可視性が確立されていない場合、クラシファイアはそれだけではブロックしません。代わりに他のルールに対してコンテンツを判定します
* 別のリポジトリまたは組織に対するプルリクエストの開設、`gh repo fork` でのフォーク、またはサードパーティリポジトリへのプッシュ。その外部ターゲットに名前を付けない限り

Claude Code v2.1.203 以降もこれらをデフォルトでブロックします。

* 機密ローカルストアからのコンテンツ、またはファイル名、パス、またはタイプが機密としてマークしているファイルからのコンテンツ。コミット、プッシュ、PR または issue テキスト、gist または paste、またはパッケージ公開に入る。ソースと宛先の両方に名前を付けない限り。セッショントランスクリプトと会話ログ、SSH キー、クラウド認証情報、ブラウザプロファイル、シェル履歴などの認証情報と構成ドットフォルダ、およびユーザーデータエクスポートはすべてカウントされ、リポジトリがプライベートであることはそれをクリアしません

Claude Code v2.1.205 以降もこれらをデフォルトでブロックします。

* Claude Code セッショントランスクリプト、`~/.claude/projects/` または構成ディレクトリの下の `.jsonl` 履歴ファイルへの書き込み。シェルコマンドを通じて直接または間接的に。ルールはまた、Claude Code が独自のチェック用に各トランスクリプトエントリに追加するメタデータ行もカバーします。トランスクリプトの読み取りはブロックされません
* `rm -rf "$VAR"` または `Remove-Item -Recurse -Force $dir` のような再帰的な force delete。ターゲットはシェル変数であり、会話でクラシファイアが見るどこにも割り当てられていません。値は以前のコマンド出力からのみ来ました。クラシファイアは決してそれを受け取らないため、クラシファイアは削除ターゲットを他の削除ルールに対して検証できません。ブロックは、削除されている正確なパスに名前を付けるか、Claude が解決されたリテラルパスが書き込まれたコマンドで削除を再実行するときにクリアされます。クラシファイアが解決できるターゲットを持つ削除は影響を受けません。`Remove-Item` ターゲットが裸の `*` または `/*` または `\*` で終わる場合、クラシファイアに到達しません。Claude Code は[それらを直接拒否します](#remove-item-in-powershell)

Claude Code v2.1.257 以降もこれらをデフォルトでブロックします。

* `169.254.169.254` などのクラウドインスタンスメタデータエンドポイントから認証情報をリクエスト、またはマシン自体のサービスアカウントまたはノード ID でクラウド、クラスタ、またはレジストリコールを明示的に認証
* トンネル、リバースシェル、または外部を指すようにリライトされたリゾルバーまたはプロキシ構成など、直接リクエスト以外のルートでパブリックホストに到達
* ノード証明書やノードのコンテナレジストリ認証など、タスクではなくホストに属する認証情報を読み取る
* Claude が開始しなかった兄弟コンテナ、ポッド、または VM、またはコンテナの下のノードに接続またはスキャン

Claude Code がこれらの 1 つを許可することを意図した場所で実行される場合は、`autoMode.environment` の[ホスト containment エントリ](/docs/ja/auto-mode-config#define-trusted-infrastructure)でそのセットアップを説明してください。

Claude Code v2.1.261 以降もこれらをデフォルトでブロックします。

* メッセージ、PR または issue テキスト、ドキュメント、またはリンクが開かれるか取得される他の場所で、パブリック paste、図、またはデータ共有サービスへのリンクを投稿または書き込み。URL 自体が共有されているコンテンツを含む場合。そのサービスに名前を付けない限り

**デフォルトで許可**:

* 作業ディレクトリ内のローカルファイル操作
* ロックファイルまたはマニフェストで宣言された依存関係のインストール
* `.env` の読み取りと一致する API への認証情報の送信
* 読み取り専用 HTTP リクエスト
* 作業しているリポジトリのすべてのブランチへのプッシュ。デフォルトブランチを含む。デプロイまたは公開ターゲットとしてマークされた非デフォルトブランチ（`production` や `gh-pages` など）は対象外です。クラシファイアはそこへのプッシュを独自の条件で判定します。プッシュのコンテンツは依然として他のルールに対してチェックされ、[`permissions.deny` ルール](/docs/ja/permissions#manage-permissions)は依然として[書き込まれたとおり](/docs/ja/permissions#bash-rule-limits)すべてのモードでプッシュコマンドをブロックでき、リモート自体のブランチ保護は依然として適用されます。v2.1.211 より前では、開始したブランチ、Claude が作成したブランチ、およびデフォルトブランチへのルーチンプッシュへのプッシュのみがデフォルトで許可されていました。v2.1.203 より前では、デフォルトブランチへの直接プッシュはブロックされていました

Claude Code v2.1.195 以降もこれらをデフォルトで許可します。

* 同じセッションで前に Claude が作成した正確なジョブの削除
* セキュリティ関連のコード、構成、および脅威モデルの読み取り、レビュー、または書き込み。タスクの一部として
* 同じマルチエージェントセッションで一緒に作業しているエージェント間のメッセージ
* [`environment`](/docs/ja/auto-mode-config#define-trusted-infrastructure)にリストされている信頼できるドメイン、バケット、およびサービスへのデータ送信。これはデータフローのみをカバーし、同じインフラストラクチャ上の破壊的または認証情報操作ではありません
* [Chrome の Claude](/docs/ja/chrome)。信頼できる内部ドメイン、localhost、または指定した URL へのナビゲーション

サンドボックス化されたコマンドはデフォルトではネットワークアクセスを取得しません。Claude はコマンドが必要とするホストをコマンド自体に名前を付け、クラシファイアはそれらをコマンドでレビューし、承認されたリストはそのコマンドのみのためにそれらのホストを開きます。[コマンドごとの許可ドメイン](/docs/ja/sandboxing#per-command-allowed-domains-in-auto-mode)は、リストが何を開くことができるか、できないか、およびコマンドがリストされていないホストに到達するときに何が起こるかをカバーしています。

`claude auto-mode defaults` を実行して、完全なルールリストを JSON として出力します。ルーチンアクションがブロックされる場合、管理者は `autoMode.environment` 設定を通じて信頼できるリポジトリ、バケット、およびサービスを追加できます。[自動モードの構成](/docs/ja/auto-mode-config)を参照してください。

リポジトリで作業しているすべてのブランチへのプッシュとリクエストに一致するプルリクエストの作成は、プッシュまたはプルリクエストが[ブロックリスト](#what-the-classifier-blocks-by-default)に該当しない限り、プロンプトなしで実行されます。例えば、リポジトリを離れるシークレットまたは機密データ、または別のリポジトリまたは組織をターゲットにするプルリクエストなど。自動モードにとどまりながらこれらのコマンドの前に人間のチェックポイントを要求するには、`permissions.ask` ルールを追加します。これは[書き込まれたとおり](/docs/ja/permissions#bash-rule-limits)コマンドに一致します。[一般的な境界](/docs/ja/auto-mode-config#common-boundaries)を参照してください。

<h3 id="first-read-outside-the-working-directories">
  作業ディレクトリ外の最初の読み取り
</h3>

[`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories)がオフの間、ファイル読み取りは自動モードでプロンプトなしで実行されます。[作業ディレクトリ](/docs/ja/permissions#working-directories)外のパスを含む。Claude が Read、Grep、または Glob ツールを初めて使用するとき。外側のパスで、Claude Code はそれらの読み取りを許可し続けるかどうかを尋ねます。

プロンプトは非インタラクティブな `-p` 実行またはバックグラウンドセッションには表示されません。そこでの読み取りは以前と同じように実行されます。

答えに関係なく、Claude は作業を続けます。

* **許可し続ける**: 読み取りが実行され、作業ディレクトリ外の後の読み取りは以前と同じように実行され、Claude Code は答えを記録するため、プロンプトは再度表示されません
* **今からブロック**: 読み取りが拒否され、Claude Code は [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories)をユーザー設定で `true` に設定します。これにより、ファイルツールはすべての後のセッションとすべての権限モードでそのような読み取りを拒否します。後で Claude がそのようなパスを読み取ることを許可するには、`/add-dir` でそのディレクトリを追加するか、設定を削除します。
* **次回また質問**: 読み取りが拒否され、作業ディレクトリ外の次の読み取りが再度プロンプトします

<h3 id="boundaries-you-state-in-conversation">
  会話で述べた境界
</h3>

クラシファイアは、会話で述べた境界をブロック信号として扱います。Claude に「プッシュしないで」または「デプロイする前にレビューを待つ」と言う場合、クラシファイアはデフォルトルールが許可する場合でも一致するアクションをブロックします。境界は、後のメッセージでそれを解除するまで有効です。Claude 自身の条件が満たされたという判定はそれを解除しません。

境界はルールとして保存されません。クラシファイアはチェックのたびにトランスクリプトから再度読み取るため、[コンテキストコンパクション](/docs/ja/costs#reduce-token-usage)が境界を述べたメッセージを削除する場合、境界は失われる可能性があります。ハード保証の場合は、代わりに[拒否ルール](/docs/ja/permissions#permission-rule-syntax)を追加します。

<h3 id="approvals-you-state-in-conversation">
  会話で述べた承認
</h3>

ブロックされたアクションが許可されていることを Claude に伝える場合、クラシファイアはそれをあなたの承認として読み取り、ブロックをクリアできます。それをどのように表現したかは、アクションが実行されるかどうか、および承認がどこまで到達するかを決定します。

* **アクションとその詳細に名前を付ける**: メッセージはアクションと、それを危険にする特定のもの（force push のブランチなど）に名前を付ける必要があります。動詞だけに名前を付けるとブロックはクリアされません。「force-push できます」はブロックを有効なままにします。
* **1 つのアクションをカバーすることを期待する**: 承認は、あなたが名前を付けた破壊的なアクションをカバーするため、後のアクションは再度ブロックされます。ルーチンパターンを 1 つずつ承認することを停止するには、[`autoMode.allow`](/docs/ja/auto-mode-config#override-the-block-and-allow-rules)に追加します。
* **いくつかのブロックは有効なままです**: [クラシファイアの優先順位](/docs/ja/auto-mode-config#override-the-block-and-allow-rules)は、あなたの承認が到達できるブロックを設定します。それがクリアしないステップを実行するには、[自動モードを離れ](#switch-permission-modes)、権限プロンプトに答えます。

<h3 id="when-auto-mode-falls-back">
  自動モードがフォールバックするとき
</h3>

自動モードがセッションのアクションを承認できない場合、何が起こるかはケースによって異なります。

* **ブロックされたアクション**: Claude Code は通知を表示し、`/permissions` の下の **Recently denied** タブにアクションをリストします。そこで `r` を押して、手動承認で再試行できます。クラシファイアが[アクションに対して判定を出さない](/docs/ja/errors#auto-mode-cannot-determine-the-safety-of-an-action)場合。自動モードとは別の安全チェックがクラシファイアのリクエスト自体を拒否したか、その応答が解析されなかったため、Claude Code は通知または **Recently denied** エントリなしでアクションを拒否します。
* **繰り返されるブロック**: クラシファイアが連続して 3 回またはセッション全体で 20 回アクションをブロックする場合、自動モードは一時停止し、Claude Code はプロンプトを再開します。プロンプトされたアクションを承認すると、自動モードが再開されます。これらのしきい値は構成不可能です。許可されたアクションは連続カウンターをリセットしますが、合計カウンターはセッション用に保持され、独自のリミットがフォールバックをトリガーするときのみリセットされます。Claude Code は、[自動モードとは別の安全チェックがクラシファイアのリクエストを拒否する](/docs/ja/errors#auto-mode-cannot-determine-the-safety-of-an-action)場合、拒否をどちらのしきい値にもカウントしません。リンクされたエントリは Claude Code がそれらの拒否をどのように処理するかをカバーしています。
* **プロンプトできないセッション**: [`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags)のない[非インタラクティブ](/docs/ja/headless) `-p` 実行にはフォールバックするプロンプトがありません。繰り返されるブロックがしきい値に到達すると、アクションは実行されず、Claude は作業を続けます。[自動モードとは別の安全チェックがクラシファイアのリクエストを拒否する](/docs/ja/errors#auto-mode-cannot-determine-the-safety-of-an-action)場合も同じです。Claude Code はどちらの場合もランを停止しません。
* **サーバーからの判定なし**: [サーバー側クラシファイアレビュー](#server-side-classifier-review)では、Claude Code はサーバーが判定を出さないアクションを拒否し、連続して 10 回のレスポンスで判定がない場合、ターンを停止します。[サーバーが安全判定を返さなかった](/docs/ja/errors#the-server-returned-no-safety-verdict)を参照してください。
* **チェック中のモード切り替え**: クラシファイアチェックが保留中に権限モードを切り替える場合、Claude Code は新しいモードが要求しなかった判定を破棄します。代わりにプロンプトされるか、[`dontAsk` モード](#allow-only-pre-approved-tools-with-dontask-mode)でアクションが自動拒否されます。

繰り返されるブロックは通常、クラシファイアがインフラストラクチャについてのコンテキストを欠いていることを意味します。`/feedback` を使用して偽陽性を報告するか、管理者に[信頼できるインフラストラクチャを構成](/docs/ja/auto-mode-config)させてください。

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="クラシファイアがアクションを評価する方法">
    各アクションは固定の決定順序を通過します。最初に一致するステップが勝ちます。

    1. [許可、質問、または拒否ルール](/docs/ja/permissions#manage-permissions)に一致するアクションは、これらの例外を除いて直ちに解決されます。
       * [保護されたパス](#protected-paths)への書き込みは、許可ルールが一致する場合でもクラシファイアにルーティングされます。`rm` と `rmdir` の削除も Claude Code v2.1.218 以降で[重要なパス](#critical-paths)をターゲットにしている場合もそうです
       * [`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool)とマークされた MCP ツール。許可ルールが一致する場合でも直接プロンプトします。コネクタツール[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)したもの。その設定が Claude Code に到達するセッションでも
       * [コマンドごとの許可ドメイン](/docs/ja/sandboxing#per-command-allowed-domains-in-auto-mode)を含むシェルコマンド。許可ルールが一致する場合でもクラシファイアにルーティングされます。ルールはコマンドを承認するため、そのホストではなく
       * `Bash(git push *)` のようなコマンドのコンテンツで一致する質問ルール。権限プロンプトにフォールバック
    2. 読み取り専用アクションと作業ディレクトリ内のファイル編集は自動承認されます。[保護されたパス](#protected-paths)と[作業ディレクトリ外の最初の読み取り](#first-read-outside-the-working-directories)への書き込みを除く。プロンプトします
       * [サーバー側クラシファイアレビュー](#server-side-classifier-review)を持つセッションでは、読み取り専用および[サンドボックス化](/docs/ja/sandboxing#sandbox-modes)シェルコマンドはそのレビューを待ち、それがそれらにフラグを立てる場合はブロックされます
    3. その他すべてはクラシファイアに送信されます。ステップ 1 でプロンプトするコネクタツールと`requiresUserInteraction` MCP ツールはクラシファイアに到達しません。組織が必要な承認も同意ステップも自動承認されません
    4. クラシファイアがブロックする場合、Claude は理由を受け取り、代替を試みます。ほとんどのセッションでは、理由は `[Data Exfiltration]` のようにクラシファイアが一致したルールに名前を付けます。書かれた説明ではなく。[拒否をレビュー](/docs/ja/auto-mode-config#review-denials)を参照してください

    自動モードに入ると、広いルールが削除されます。これらは任意のコード実行を許可します。

    * ブランケット `Bash(*)` または `PowerShell(*)`
    * `Bash(python*)` のようなワイルドカードインタープリタ
    * パッケージマネージャー実行コマンド
    * `Agent` 許可ルール
    * [`Monitor`](/docs/ja/tools-reference#monitor-tool)許可ルール。Claude Code は Monitor コマンドをシェルを通じて実行するため

    `Bash(npm test)` のような狭いルールは有効なままです。Claude Code は自動モードを離れるときに削除されたルールを復元します。v2.1.236 より前では、Claude Code は自動モードで `Monitor` 許可ルールを有効なままにしていたため、ツール全体に一致するルールは分類器レビューなしで Monitor コマンドを承認しました。

    Claude Code はまた、`git reset --hard` や `rm -rf` のようなコミットされていない作業を破棄するコマンドの前に `git status` 自体を実行し、ステージされた、変更された、または追跡されていない作業が存在するかどうかをクラシファイアに表示します。Claude Code は、リポジトリの git 構成が `status.showUntrackedFiles=no` を設定する場合でも、そのチェックで追跡されていないファイルを報告します。

    Claude Code 自体が送信するクラシファイアリクエストでは、クラシファイアはユーザーメッセージ、ファイル読み取りや検索などの読み取り専用ルックアップ以外のツール呼び出し、および CLAUDE.md コンテンツを見ます。ツール結果はそれらのリクエストから削除されるため、ファイルまたは web ページ内の敵対的なコンテンツはクラシファイアを直接操作できません。

    [PostToolUse hook の `classifierContext` フィールド](/docs/ja/hooks#annotate-a-result-for-the-auto-mode-classifier)で呼び出しの結果に注釈を付けることができます。クラシファイアはアプリケーション提供のコンテキストとして読み取ります。フィールドには Claude Code v2.1.236 以降が必要です。

    別のサーバー側プローブは、Claude がそれを読む前に、受信ツール結果をスキャンして疑わしいコンテンツにフラグを立てます。これらのレイヤーがどのように連携するかについての詳細は、[自動モードアナウンスメント](https://claude.com/blog/auto-mode)と[エンジニアリング深掘り](https://www.anthropic.com/engineering/claude-code-auto-mode)を参照してください。
  </Accordion>

  <Accordion title="自動モードがサブエージェントを処理する方法">
    クラシファイアは[サブエージェント](/docs/ja/sub-agents)作業を 3 つのポイントでチェックします。

    1. サブエージェントが開始する前に、委任されたタスク説明が評価されるため、危険に見えるタスクはスポーン時にブロックされます。
    2. サブエージェントが実行されている間、その各アクションはクラシファイアを通じて親セッションと同じルールで実行され、サブエージェントの frontmatter の `permissionMode` は無視されます。
    3. サブエージェントが終了すると、クラシファイアはその作業と最終レポートをレビューしてから、親がレポートを読みます。クラシファイアがサブエージェントの作業またはレポートにフラグを立てるか、別の API 安全チェックがレビューを拒否する場合、レポートは依然として配信されます。セキュリティ警告が前に付きます。クラシファイアがレビューに利用できない場合、レポートはサブエージェントの作業を検証してから行動する前に確認するメモと共に到着します。
  </Accordion>

  <Accordion title="コストとレイテンシ">
    クラシファイアはデフォルトでは `/model` 選択ではなく Claude Sonnet 5 で実行されます。Anthropic がサーバー側で構成するクラシファイアモデルはそのデフォルトより優先されます。セッションのモデルが Claude Sonnet 4.6 の場合、または [`availableModels`](/docs/ja/model-config#restrict-model-selection)が Sonnet 5 を除外する場合、クラシファイアは代わりにセッションのモデルで実行されます。またはセッションが[Fable モデル](/docs/ja/model-config#work-with-fable)で実行される場合は Opus モデルで。Anthropic API 以外のプロバイダーでは、その Opus フォールバックはプロバイダーのデフォルト Opus モデルです。

    セッションの最初の自動モードリクエストは Sonnet 5 デフォルトを検証します。リクエストが成功する場合、Sonnet 5 はセッションのクラシファイアモデルのままです。リクエストがモデルが利用できないため失敗する場合、セッションは代わりにフォールバックを使用します。その検証が解決した後、クラシファイアのモデルはセッション用に変更されません。

    Enterprise プランおよび Claude API を使用するアカウント、[AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws)、Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry では、クラシファイア呼び出しはトークン使用量にカウントされます。各チェックはトランスクリプトの一部と保留中のアクションを送信し、実行前にラウンドトリップを追加します。読み取りと保護されたパス外の作業ディレクトリ編集はクラシファイアをスキップするため、オーバーヘッドは主にシェルコマンドとネットワーク操作から来ます。サーバーがアクションをレビューする場所では、セッションのモデルリクエストの一部として行われるため、カウントする別のクラシファイア呼び出しはありません。[サーバー側クラシファイアレビュー](#server-side-classifier-review)を参照してください。

    サンドボックス化されたネットワークアクセスは、コマンドごとのクラシファイアリクエストを追加しません。クラシファイアは[コマンドが名前を付けるホスト](/docs/ja/sandboxing#per-command-allowed-domains-in-auto-mode)をコマンドと一緒に判定し、Claude Code は承認されたリストに対して各接続をチェックします。クラシファイアを再度呼び出さずに。
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  dontAsk モードで事前承認済みツールのみを許可する
</h2>

`dontAsk` モードを設定すると、Claude Code は本来プロンプトを表示するすべてのツール呼び出しを自動的に拒否します。Claude は Manual モードで承認が不要なアクション（作業ディレクトリ内のファイル読み取りや[読み取り専用 Bash コマンド](/docs/ja/permissions#read-only-commands)など）、および `permissions.allow` ルールに一致するアクション、[PreToolUse フック](/docs/ja/permissions#extend-permissions-with-hooks)によって承認されたコール実行を継続します。このモードは CI パイプラインや制限された環境で使用します。Claude が実行できる内容を事前に定義でき、セッションは入力を待つことはありません。このモードがアクティブな間、ステータスバーに `⏵⏵ don't ask on` が表示されます。

Claude Code は、プロンプトを表示する代わりに、明示的な[`ask` ルール](/docs/ja/permissions#manage-permissions)に一致するコールを拒否します。また、allow ルールが一致する場合でも組み込みの `AskUserQuestion` ツールを拒否し、その設定が Claude Code に到達するセッションで[組織が `ask` に設定したコネクタツール](/docs/ja/mcp#organization-controls-on-connector-tools)についても同じことを行います。[`_meta["anthropic/requiresUserInteraction"]`](/docs/ja/mcp#require-approval-for-a-specific-tool)でマークされた MCP ツールも同じ方法で拒否します。これは、承認カードがこのモードが収集しない回答を必要とするためです。これには Claude Code v2.1.199 以降が必要です。

[重要なパス](#critical-paths)（`rm -rf /` や `rm -rf ~` など）を対象とした `rm` および `rmdir` の削除は、allow ルールが一致する場合や `PreToolUse` フックが許可する場合でも拒否されます。

[Claude Code on the web](/docs/ja/claude-code-on-the-web) のクラウドセッションは `defaultMode: "dontAsk"` を無視します。詳細は[bypassPermissions](#skip-all-checks-with-bypasspermissions-mode)を参照してください。

スタートアップ時にフラグで設定します：

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  bypassPermissions モードですべてのチェックをスキップする
</h2>

`bypassPermissions` モードは権限プロンプトとセーフティチェックを無効にするため、[保護されたパス](#protected-paths)への書き込みを含むツール呼び出しが即座に実行されます。

[アクション no モードが自動承認する](#actions-no-mode-auto-approves)アクションはこのモードでもプロンプトが表示されます。

このモードでは、および権限バイパスが利用可能なインタラクティブターミナルプランモードセッションでは、2 つの[クロスセッションメッセージング](/docs/ja/cross-session-messaging)セーフガードが引き続き適用されます。

* このマシンを超えたセッションへのメッセージに対する [`isolatePeerMachines`](/docs/ja/settings-reference#isolatepeermachines) 承認プロンプトが引き続き表示されます。
* [`crossSessionInbound`](/docs/ja/cross-session-messaging#control-inbound-messages) 値が適用されない場合、Claude Code は別のセッションからのインバウンドメッセージを承認待ちで保持し、送信セッションが権限プロンプトもバイパスしていることを識別した場合にのみ確認なしで配信します。権限モードを終了してメッセージが保持されている場合、Claude Code はインバウンドルールを再適用し、保持されているメッセージのうち現在受け入れるものを配信します。

権限バイパスが利用可能なインタラクティブターミナルセッションでは、Claude Code は[プランモードの](#analyze-before-you-edit-with-plan-mode)ブロックも強制しません。Claude はまだ編集なしでプランするよう指示されていますが、プランニング中に試みるファイル編集またはシェルコマンドはプロンプトなしで実行されます。明示的な[質問ルール](/docs/ja/permissions#manage-permissions)および `rm` と `rmdir` の削除で[クリティカルパス](#critical-paths)をターゲットにしたものはまだプロンプトが表示されます。

プランモードは Claude Code がインタラクティブターミナルなしで実行される場所ではブロックを保持します。これには `-p` を使用した[非インタラクティブ実行](/docs/ja/headless)、[Agent SDK](/docs/ja/agent-sdk/permissions#plan-mode-plan) セッション、および [VS Code 拡張機能](/docs/ja/vs-code)のチャットパネルでの会話が含まれます。そこでは、`--allow-dangerously-skip-permissions` により `bypassPermissions` が後で選択可能になります。

<Warning>
  このモードはコンテナ、VM、またはインターネットアクセスのない dev コンテナなどの隔離された環境でのみ使用してください。そのような環境では Claude Code がホストシステムに損害を与えることができません。
</Warning>

このモードを有効にせずに開始したセッションから `bypassPermissions` に入ることはできません。[`permissions.defaultMode: "bypassPermissions"`](/docs/ja/settings-reference#permissions-defaultmode) で起動時に有効にするか、有効化フラグを使用して有効にしてください。

```bash theme={null}
claude --permission-mode bypassPermissions
```

`--dangerously-skip-permissions` フラグは同等です。

Claude Code は [`--restricted`](/docs/ja/cli-reference#cli-flags) で開始したセッションで `bypassPermissions` を拒否します。`--restricted` には Claude Code v2.1.248 以降が必要です。

このモードを有効にしてインタラクティブセッションを初めて開始すると、Claude Code は権限チェックなしで実行されたアクションの責任を受け入れるよう求める警告ダイアログを表示します。Claude Code はユーザー設定に受け入れを保存するため、ダイアログは 1 回だけ表示されます。拒否した場合、Claude Code は終了します。[非インタラクティブモード](/docs/ja/headless)ではダイアログは表示されず、`--bg` で開始した[バックグラウンドセッション](/docs/ja/agent-view)はインタラクティブセッションでダイアログを受け入れるまで拒否されます。

Linux と macOS では、Claude Code はこのモードで root として、または `sudo` の下で実行されている場合、起動を拒否します。

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

チェックは認識されたサンドボックス内では自動的にスキップされます。コンテナで自律的に実行するには、[dev コンテナ](/docs/ja/devcontainer)設定を使用してください。これは Claude Code を非 root ユーザーとして実行します。

[Web 上の Claude Code](/docs/ja/claude-code-on-the-web) は設定ファイルから `defaultMode: "bypassPermissions"` または `"dontAsk"` を尊重しないため、リポジトリのチェックイン設定はクラウドセッションをバイパス権限モードで開始できません。設定は無視され、セッションはモードドロップダウンに表示される権限モードで開始されます。[権限モードを切り替える](#switch-permission-modes)を参照して、クラウドセッションが提供するモードを確認してください。

<Warning>
  `bypassPermissions` はプロンプトインジェクションまたは意図しないアクションに対する保護を提供しません。権限プロンプトがはるかに少ないバックグラウンドセーフティチェックの場合は、代わりに[オートモード](#eliminate-prompts-with-auto-mode)を使用してください。管理者は [管理設定](/docs/ja/managed-settings)で `permissions.disableBypassPermissionsMode` を `"disable"` に設定することでこのモードをブロックできます。
</Warning>

<h2 id="protected-paths">
  保護されたパス
</h2>

パスの小さなセットへの書き込みは、`bypassPermissions` モードおよび [bypass permissions が利用可能](#skip-all-checks-with-bypasspermissions-mode)なプラン モード セッションを除き、自動承認されることはありません。これはリポジトリ状態と Claude 独自の設定の偶発的な破損を防ぎます。

| モード                     | 保護されたパスへの書き込み                                                                                                                                                                  |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`、`acceptEdits` | プロンプト表示                                                                                                                                                                        |
| `plan`                  | [bypass permissions](#skip-all-checks-with-bypasspermissions-mode)が利用可能なセッションで許可。そうでない場合、[auto モード](#eliminate-prompts-with-auto-mode)が計画中に利用可能な場合は分類器にルーティング。利用できない場合はプロンプト表示 |
| `auto`                  | 分類器にルーティング                                                                                                                                                                     |
| `dontAsk`               | 拒否                                                                                                                                                                             |
| `bypassPermissions`     | 許可                                                                                                                                                                             |

[`--restricted`](/docs/ja/cli-reference#cli-flags)で開始されたセッションでは、Claude Code v2.1.248 以降が必要で、分類器は保護されたパスへの書き込みを承認できません。

[`permissions.allow`](/docs/ja/permissions#manage-permissions)設定ファイルのルールは、保護されたパスへの書き込みを事前承認しません。安全性チェックは Claude Code が設定から allow ルールを評価する前に実行されるため、`~/.claude/settings.json` または `.claude/settings.json` の `Edit(.claude/**)` などのエントリは、上記の表のモード別の結果を変更しません。プロンプトを表示するモードでは、`.claude/` への書き込みのプロンプトに **Yes, and allow Claude to edit its own settings for this session** というオプションが表示され、そのセッション内の後続の `.claude/` への書き込みを再度プロンプトなしで承認します。

保護されたディレクトリ：

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`。ただし `.claude/worktrees` は除く。Claude はここに独自の git worktrees を保存します

保護されたファイル：

* `.gitconfig`、`.gitmodules`
* `.bashrc`、`.bash_profile`、`.bash_login`、`.bash_aliases`、`.bash_logout`、`.zshrc`、`.zprofile`、`.zshenv`、`.zlogin`、`.zlogout`、`.profile`、`.envrc`
* `.npmrc`、`.yarnrc`、`.yarnrc.yml`、`.pnp.cjs`、`.pnp.loader.mjs`、`.pnpmfile.cjs`、`bunfig.toml`、`.bunfig.toml`
* `.bazelrc`、`.bazelversion`、`.bazeliskrc`
* `.pre-commit-config.yaml`、`lefthook.yml`、`lefthook.yaml`、`.lefthook.yml`、`.lefthook.yaml`
* `gradle-wrapper.properties`、`maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`、`pyrightconfig.json`
* `.mcp.json`、`.claude.json`

<h2 id="critical-paths">
  重要なパス
</h2>

Claude Code は、[`permissions.allow`](/docs/ja/permissions#manage-permissions) ルールまたは `"allow"` を返す [`PreToolUse` フック](/docs/ja/permissions#extend-permissions-with-hooks) が `rm` または `rmdir` コマンドを承認することはありません。そのコマンドが重要なパスをターゲットにしている場合、他のプロンプトをスキップするモードであっても承認されません。このサーキットブレーカーはモデルエラーから保護します。マッチする deny ルールはコマンドを完全にブロックします。

代わりに何が起こるかは、権限モードによって異なります。

| モード                     | Claude Code が重要なパスの削除に対して行うこと                                                                    |
| :---------------------- | :----------------------------------------------------------------------------------------------- |
| `default`、`acceptEdits` | 承認を求めます                                                                                          |
| `plan`                  | 承認を求めます。[計画中に自動モードが利用可能](#analyze-before-you-edit-with-plan-mode)で、バイパス権限が利用できない場合、代わりに分類器に送信します |
| `auto`                  | [分類器](#eliminate-prompts-with-auto-mode)に送信します                                                   |
| `dontAsk`               | 拒否します                                                                                            |
| `bypassPermissions`     | 承認を求めます                                                                                          |

明示的な [ask ルール](/docs/ja/permissions#manage-permissions) がコマンドにマッチする場合、Claude Code は `auto` モードでも承認を求めます。承認を求めるモードでは、[`PermissionRequest` フック](/docs/ja/hooks#permissionrequest) は他のプロンプトに答えるのと同じ方法でプロンプトに答えることができます。

Claude Code は、`rm` または `rmdir` ターゲットが以下のいずれかである場合、重要なパスとして扱います。

* ファイルシステムのルート
* トップレベルディレクトリ、つまりルートの直接の子である `/usr`、`/etc`、`/data` などのディレクトリ
* ホームディレクトリ
* Windows ドライブルートとそのトップレベルディレクトリ（`C:\` や `C:\Windows` など）
* 作業ディレクトリとその親
* 追加の作業ディレクトリとその親。ただし、削除が `rm -rf <dir>/*` のようにそれらの下のグロブである場合のみ。ディレクトリ自体に対する `rm -rf <dir>` はこのチェックをトリガーしません

Claude Code は、`rm -rf "$DIR"/*` のようなシェル変数の直下のグロブまたは末尾のスラッシュも重要なパスの削除として扱います。変数が空の場合、コマンドはファイルシステムルートからの削除になるためです。

このプロンプトは、フラグが付いた `rm` に名前を付け、チェックに合格するように書き直す方法を説明します。

* `$DIR` のような変数の場合、各展開をガードして、変数が設定されていないか空の場合にシェルがエラーで停止するようにします。例えば `rm -rf "${DIR:?}"/*` のように、またはリテラルパスを使用します
* `$HOME` のような通常設定されている変数の場合、リテラルパスを使用します

その方法ですべての展開がガードされている削除は重要なパスの削除ではないため、`bypassPermissions` モードではプロンプトなしで実行されます。

削除を `(...)` を使用したサブシェル、`{ ...; }` を使用したブレースグループ、`$(...)` またはバッククォートを使用したコマンド置換、あるいは `<(...)` を使用したプロセス置換内に隠すことは、チェックをスキップしません。Claude Code は、`(rm -rf ~)` や `echo "$(rm -rf ~)"` のように置換内にある重要なパスの削除、または同じコマンド内の他の場所にある削除を見つけます。

<h3 id="remove-item-in-powershell">
  PowerShell の Remove-Item
</h3>

[PowerShell ツール](/docs/ja/tools-reference#powershell-tool)を有効にすると、Claude Code は `Remove-Item` に独自のチェックを与えます。これは `rm` 重要なパスリストとは別です。結果はターゲットに依存し、最初にマッチするケースが適用されます。

* **システムパス**: ファイルシステムルートとそのトップレベルディレクトリ、ドライブルートとそのトップレベルディレクトリ、およびホームディレクトリ。Claude Code はすべてのモードでコマンドを拒否し、承認を求めません。
* **ワイルドカード**: 裸の `*`、または `/*` または `\*` で終わるターゲット（`$dir/*` のようなシェル変数の下のグロブを含む）。Claude Code は [分類器](#eliminate-prompts-with-auto-mode) がそれを見る前に、すべてのモードでコマンドを拒否し、承認を求めません。
* **作業ディレクトリまたはその親の 1 つ（`-Recurse` 付き）**: Claude Code はコマンドを承認が必要な他のコマンドと同じように扱うため、承認を求めるモードでは承認を求め、`auto` モードでは分類器に送信し、`dontAsk` モードでは拒否します。`bypassPermissions` モードはこのチェックをスキップします。

<h2 id="see-also">
  関連項目
</h2>

* [Permissions](/docs/ja/permissions)：allow、ask、deny ルール。管理ポリシー
* [Configure auto mode](/docs/ja/auto-mode-config)：分類器に組織が信頼するインフラストラクチャを伝える
* [Hooks](/docs/ja/hooks)：`PreToolUse` および `PermissionRequest` フック経由のカスタム権限ロジック
* [Security](/docs/ja/security)：セキュリティ保護とベストプラクティス
* [Sandboxing](/docs/ja/sandboxing)：Bash コマンドのファイルシステムとネットワーク隔離
* [Non-interactive mode](/docs/ja/headless)：`-p` フラグで Claude Code を実行
