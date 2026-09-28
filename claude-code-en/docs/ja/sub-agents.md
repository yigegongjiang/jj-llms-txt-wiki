> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# カスタムサブエージェントの作成

> Claude Code でタスク固有のワークフローと改善されたコンテキスト管理のための特化した AI サブエージェントを作成して使用します。

サブエージェントは、特定の種類のタスクを処理する特化した AI アシスタントです。サイドタスクがメイン会話に検索結果、ログ、または再度参照しないファイルコンテンツで溢れかえる場合に使用します。サブエージェントはそのタスクを独自のコンテキストで実行し、概要のみを返します。同じ種類のワーカーを同じ指示で繰り返し生成する場合は、カスタムサブエージェントを定義します。

各サブエージェントは、カスタムシステムプロンプト、特定のツールアクセス、および独立した権限を備えた独自のコンテキストウィンドウで実行されます。Claude がサブエージェントの説明に一致するタスクに遭遇すると、そのサブエージェントに委譲し、サブエージェントは独立して動作して結果を返します。実際にコンテキスト節約を確認するには、[コンテキストウィンドウの可視化](/docs/ja/context-window)で、サブエージェントが独自の別のウィンドウで研究を処理するセッションを説明しています。

<Note>
  サブエージェントは単一のセッション内で動作します。多くの独立したセッションを並行して実行し、1 つの場所から監視するには、[バックグラウンドエージェント](/docs/ja/agent-view)を参照してください。互いにメッセージを渡すセッションについては、[クロスセッションメッセージング](/docs/ja/cross-session-messaging)を参照してください。Claude が生成および監督する調整されたセッションのチームについては、[エージェントチーム](/docs/ja/agent-teams)を参照してください。
</Note>

サブエージェントは以下に役立ちます：

* **コンテキストを保持する** ことで、探索と実装をメインの会話から分離します
* **制約を強制する** ことで、サブエージェントが使用できるツールを制限します
* **設定を再利用する** ことで、ユーザーレベルのサブエージェントをプロジェクト全体で再利用します
* **動作を特化させる** ことで、特定のドメイン向けの焦点を絞ったシステムプロンプトを使用します
* **コストを制御する** ことで、Haiku のような高速で安価なモデルにタスクをルーティングします

Claude は各サブエージェントの説明を使用して、タスクを委譲するかどうかを決定します。サブエージェントを作成するときは、Claude がいつそれを使用するかを知るように、明確な説明を書いてください。

これらの説明はコンテキストを占有するため、短く保ってください。組み込みのサブエージェントを除いたサブエージェントの説明の合計が 15,000 トークンを超える場合、Claude Code は[スタートアップ時に総トークン数を含む警告を表示](/docs/ja/errors#agent-descriptions-are-over-the-15000-token-limit)します。サブエージェントの `description` フィールドをトリミングし、詳細を各サブエージェントのシステムプロンプトに移動します。システムプロンプトは、そのサブエージェントが実行される場合にのみロードされます。

<h2 id="built-in-subagents">
  組み込みサブエージェント
</h2>

Claude Code には、Claude が必要に応じて自動的に使用する組み込みサブエージェントが含まれています。各サブエージェントは親の会話の権限を継承します。ほとんどは制限されたツールセットで実行されます。

Explore と Plan は、研究を高速かつ低コストに保つために、CLAUDE.md ファイルと git ステータススナップショットをスキップします。その他のすべての組み込みサブエージェントと[カスタムサブエージェント](#configure-subagents)は、その定義が[`omitClaudeMd`](#supported-frontmatter-fields)フィールドを設定してユーザー、プロジェクト、およびローカル CLAUDE.md ファイルをスキップしない限り、両方を読み込みます。サブエージェントに到達するものの完全な内訳については、[スタートアップ時に読み込まれるもの](#what-loads-at-startup)を参照してください。

<Tabs>
  <Tab title="Explore">
    コードベースの検索と分析に最適化された高速な読み取り専用エージェント。

    * **モデル**: メイン会話から継承され、Claude API では Opus でキャップされます。つまり、Explore は、`CLAUDE_CODE_SUBAGENT_MODEL`を設定して[すべてのサブエージェントを 1 つのモデルで実行](#run-every-subagent-on-one-model)しない限り、セッション用に既に選択したモデルより高価なモデルで実行されることはありません。
    * **ツール**: 読み取り専用ツール。Write と Edit は拒否されます。
    * **目的**: ファイル検出、コード検索、コードベース探索

    v2.1.198 以降、Explore はメイン会話のモデルを継承し、常に Haiku で実行されるわけではありません。Claude API では、継承されたモデルは Opus でキャップされます。より高いティアのメイン会話は Explore を Opus で実行し、Sonnet または Haiku のメイン会話は Explore を同じモデルで実行します。Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、または AWS 上の Claude Platform などの[その他のプロバイダー](/docs/ja/third-party-integrations)では、Explore はメイン会話のモデルを直接継承します。

    [ユーザーまたはプロジェクトサブエージェント](#choose-the-subagent-scope)として名前付けられた`Explore`は、組み込みをオーバーライドし、独自の`model`フィールドを保持します。探索をより低コストのモデルに保つために、`model: haiku`で定義します。

    Claude は、変更を加えずにコードベースを検索または理解する必要がある場合、Explore に委譲します。これにより、探索結果がメイン会話コンテキストから除外されます。

    Explore を呼び出すとき、Claude は徹底度レベルを指定します。ターゲット検索の場合は**quick**、バランスの取れた探索の場合は**medium**、包括的な分析の場合は**very thorough**です。
  </Tab>

  <Tab title="Plan">
    [プランモード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)中にプランを提示する前にコンテキストを収集するために使用される研究エージェント。

    * **モデル**: メイン会話から継承されます。ただし、`CLAUDE_CODE_SUBAGENT_MODEL`を設定して[すべてのサブエージェントを 1 つのモデルで実行](#run-every-subagent-on-one-model)する場合を除きます。
    * **ツール**: 読み取り専用ツール。Write と Edit は拒否されます。
    * **目的**: 計画のためのコードベース研究

    プランモード中で Claude がコードベースを理解する必要がある場合、探索出力が別のコンテキストウィンドウに留まるように、Plan サブエージェントに研究を委譲し、メイン会話は読み取り専用のままです。
  </Tab>

  <Tab title="General-purpose">
    探索とアクションの両方を必要とする複雑なマルチステップタスク用の有能なエージェント。

    * **モデル**: [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model)モデル（設定した場合で、他の何もモデルを別の方法で割り当てない場合）、それ以外の場合はメイン会話のモデル。[モデルを選択](#choose-a-model)は完全な順序を示し、[すべてのサブエージェントを 1 つのモデルで実行](#run-every-subagent-on-one-model)は変数がこれらのソースをオーバーライドする方法を示します。
    * **ツール**: [サブエージェントで利用可能](#available-tools)なすべてのツール
    * **目的**: 複雑な研究、マルチステップ操作、コード変更

    Claude は、タスクが探索と変更の両方を必要とする場合、結果を解釈するための複雑な推論、または複数の依存ステップが必要な場合、general-purpose に委譲します。
  </Tab>

  <Tab title="Other">
    Claude Code には、特定のタスク用の追加のヘルパーエージェントが含まれています。これらは通常自動的に呼び出されるため、直接使用する必要はありません。

    | エージェント            | モデル                                                                  | Claude が使用する場合                                                                                                                                                                                                         |
    | :---------------- | :------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | 独自のモデルはありません。Claude がサブエージェントとして生成する場合は[モデル順序](#choose-a-model)に従います | タスクがより特化したエージェントに適合しない場合。[サブエージェントで利用可能](#available-tools)なすべてのツールを備えたキャッチオール。また、ディスパッチされた[バックグラウンドセッション](/docs/ja/agent-view)のデフォルトエージェント。[セッションが開始される方法に応じて、開始される権限モード](/docs/ja/agent-view#permission-mode-model-and-effort)。 |
    | statusline-setup  | Sonnet                                                               | `/statusline`を実行してステータスラインを設定する場合                                                                                                                                                                                      |
    | claude-code-guide | Haiku                                                                | Claude Code 機能について質問する場合                                                                                                                                                                                               |
  </Tab>
</Tabs>

組み込みサブエージェントはインタラクティブセッションでデフォルトで登録されます。これらを制限するには：

* 特定の組み込みタイプをブロックするには、[特定のサブエージェントを無効にする](#disable-specific-subagents)に示されているように、`permissions.deny`に追加します。
* Claude がサブエージェントに委譲するのを防ぐには、[`permissions.deny`](/docs/ja/permissions#tool-specific-permission-rules)で`Agent`ツール自体を拒否します。
* 組み込みの`Explore`および`Plan`サブエージェントのみを削除するには、[`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/ja/env-vars)を設定します。Claude はファイルを直接読み取り、探索し、サブエージェントに委譲しません。Claude Code v2.1.198 以降が必要です。
* [非インタラクティブモード](/docs/ja/headless)および[Agent SDK](/docs/ja/agent-sdk/overview)では、[`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ja/env-vars)を設定して、すべての組み込みタイプを削除し、独自のタイプのみを提供します。

サブエージェント型を省略する Agent ツール呼び出しは、セッションにフォールバックする general-purpose サブエージェントがない場合、[`subagent_type is required`](/docs/ja/errors#subagent-type-is-required)で失敗します。

これらの組み込みサブエージェント以外に、カスタムプロンプト、ツール制限、権限モード、フック、およびスキルを使用して独自のサブエージェントを作成できます。次のセクションでは、サブエージェントの使用を開始し、カスタマイズする方法を示します。

<h2 id="quickstart-create-your-first-subagent">
  クイックスタート：最初のサブエージェントを作成する
</h2>

サブエージェントは YAML フロントマターを含む Markdown ファイルです。Claude に作成してもらうか、[手動で作成](#write-subagent-files)することができます。

v2.1.198 以降、`/agents` コマンドはインタラクティブな作成ウィザードを開かなくなりました。実行すると、Claude に依頼するか `.claude/agents/` を直接編集するよう促すメッセージが表示されます。サブエージェントファイル、フロントマターフィールド、`.claude/agents/` および `~/.claude/agents/` の場所は変わりません。ターミナルウィザードのみが削除されました。

このチュートリアルでは、コードをレビューして改善を提案するユーザーレベルのサブエージェントを作成します。

<Steps>
  <Step title="Claude にサブエージェントの作成を依頼する">
    Claude Code で、作成したいサブエージェントと保存場所を説明します：

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude は `name`、`description`、`tools` リスト、`model`、およびシステムプロンプトを含むファイルを作成します。
  </Step>

  <Step title="ファイルを確認する">
    `~/.claude/agents/code-improver.md` を開き、フロントマターが要求内容と一致することを確認します。結果は次のようになります：

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    ファイルが `~/.claude/agents/` に存在するため、サブエージェントはマシン上のすべてのプロジェクトで利用可能です。代わりに 1 つのプロジェクトにスコープを設定するには、そのプロジェクトの `.claude/agents/` ディレクトリに移動します。[サブエージェントスコープを選択する](#choose-the-subagent-scope)で 2 つを比較しています。
  </Step>

  <Step title="試してみる">
    Claude に新しいサブエージェントに委譲するよう依頼します：

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude は新しいサブエージェントに委譲し、コードベースをスキャンして改善提案を返します。トランスクリプトでは、委譲はツール呼び出し行として表示され、サブエージェント名の後に短いタスク説明が続きます（例：`code-improver(Suggest code improvements)`）。

    Claude が新しいサブエージェントを見つけられない場合は、Claude Code を再起動してもう一度試してください。これは `~/.claude/agents/` がセッション開始前に存在しなかった場合にのみ発生します。実行中のセッションは新しく作成された `agents` ディレクトリを検出しないためです。
  </Step>
</Steps>

これで、マシン上のプロジェクトでコードベースを分析し、改善を提案するために使用できるサブエージェントができました。

サブエージェントファイルを手動で作成したり、CLI フラグを使用して定義したり、プラグインを通じて配布したりすることもできます。以下のセクションでは、すべての設定オプションについて説明します。

<Note>
  Claude Code v2.1.197 以前では、`/agents` はライブサブエージェントを一覧表示する **Running** タブと、サブエージェントを作成、編集、削除するための **Library** タブを備えたインタラクティブウィザードを開きます。
</Note>

<h2 id="configure-subagents">
  サブエージェントの設定
</h2>

サブエージェントのファイルの場所によって、誰がそれを利用できるかが決まり、frontmatter によってそれが何をできるかが決まります。このセクションでは、サブエージェントファイルがどこに存在するか、およびサポートされるすべてのフィールドについて説明します。

<h3 id="choose-the-subagent-scope">
  サブエージェントのスコープを選択する
</h3>

スコープに応じて、サブエージェントファイルを異なる場所に保存します。複数のサブエージェントが同じ名前を共有する場合、Claude Code はより優先度の高い場所のものを使用します。

| 場所                      | スコープ        | 優先度   | 作成方法                                 |
| :---------------------- | :---------- | :---- | :----------------------------------- |
| 管理設定                    | 組織全体        | 1（最高） | [管理設定](/docs/ja/settings)経由でデプロイ          |
| `--agents` CLI フラグ      | 現在のセッション    | 2     | Claude Code 起動時に JSON を渡す            |
| `.claude/agents/`       | 現在のプロジェクト   | 3     | Claude に依頼するか、ファイルを手動で作成             |
| `~/.claude/agents/`     | すべてのプロジェクト  | 4     | Claude に依頼するか、ファイルを手動で作成             |
| プラグインの `agents/` ディレクトリ | プラグインが有効な場所 | 5（最低） | [プラグイン](/docs/ja/plugins/overview)でインストール |

**プロジェクトサブエージェント**（`.claude/agents/`）は、コードベースに固有のサブエージェントに最適です。バージョン管理にチェックインして、チームが協力して使用および改善できるようにします。

プロジェクトサブエージェントは現在の作業ディレクトリから上へ向かって検出されるため、そこからリポジトリルートまでのすべての `.claude/agents/` がスキャンされます。これらのネストされたディレクトリの複数が同じ `name` を定義する場合、Claude Code は作業ディレクトリに最も近い定義を使用します。

`--add-dir` または `/add-dir` でディレクトリを追加すると、Claude Code はプロジェクトサブエージェントと一緒にその `.claude/agents/` フォルダも読み込みます。[追加ディレクトリ](/docs/ja/permissions#additional-directories-grant-file-access-not-configuration)を参照して、`--add-dir` から読み込む他の設定タイプを確認してください。`--add-dir` を使用せずにプロジェクト間でサブエージェントを共有するには、`~/.claude/agents/` または[プラグイン](/docs/ja/plugins/overview)を使用します。

**ユーザーサブエージェント**（`~/.claude/agents/`）は、すべてのプロジェクトで利用可能な個人用サブエージェントです。

Claude Code は `.claude/agents/` と `~/.claude/agents/` を再帰的にスキャンするため、`agents/review/` や `agents/research/` などのサブフォルダに定義を整理できます。サブディレクトリパスはサブエージェントの識別方法や呼び出し方法に影響しません。これは、ID が `name` frontmatter フィールドのみから来るためです。

`name` 値をツリー全体で一意に保ちます。同じ `.claude/agents/` ディレクトリ内（サブフォルダを含む）の 2 つのファイルが同じ名前を宣言する場合、Claude Code はそのうちの 1 つだけを読み込み、ドキュメント化された優先順位ではなくファイルシステムの読み取り順序で選択されます。ネストされたプロジェクトディレクトリ全体では、作業ディレクトリに最も近い定義が優先されます（上記で説明）。[`/doctor`](/docs/ja/commands#all-commands)セットアップチェックアップは、同じディレクトリ内で名前を共有するファイルをレポートし、1 つを除くすべての名前変更または削除を提案します。v2.1.205 より前では、`/doctor` は重複をリストし、どの定義がアクティブであるかを示す診断画面を開きました。

プラグイン `agents/` ディレクトリも再帰的にスキャンされます。プロジェクトおよびユーザースコープとは異なり、プラグインの `agents/` ディレクトリ内のサブフォルダは[スコープ付き識別子](#invoke-subagents-explicitly)の一部になります。プラグイン `my-plugin` の `agents/review/security.md` にあるファイルは `my-plugin:review:security` として登録されます。

**CLI 定義サブエージェント**は Claude Code 起動時に JSON として渡されます。これらはそのセッションのみに存在し、ディスクに保存されないため、クイックテストまたは自動化スクリプトに便利です。1 つの `--agents` 呼び出しで複数のサブエージェントを定義できます。

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

`--agents` フラグは `prompt` フィールドと以下の [frontmatter](#supported-frontmatter-fields) フィールドを含む JSON を受け入れます。`description`、`tools`、`disallowedTools`、`model`、`permissionMode`、`mcpServers`、`hooks`、`maxTurns`、`skills`、`initialPrompt`、`memory`、`effort`、`background`、`omitClaudeMd`、および `isolation`。システムプロンプトには `prompt` を使用します。これはファイルベースのサブエージェントのマークダウン本体と同等です。`color` と `experimental` はここでは受け入れられず、拒否されるのではなく無視されます。

JSON の各トップレベルキーはエージェントの名前です。名前を `-` で始めないでください。

Claude Code が読み込めない値で何をするか、およびそのチェックをスキップするフラグと環境変数については、[`Invalid --agents configuration`](/docs/ja/errors#invalid-agents-configuration)を参照してください。

**管理サブエージェント**は組織管理者によってデプロイされます。[管理設定ディレクトリ](/docs/ja/managed-settings#delivery-mechanisms)内の `.claude/agents/` にマークダウンファイルを配置し、プロジェクトおよびユーザーサブエージェントと同じ frontmatter 形式を使用します。管理定義は同じ名前のプロジェクトおよびユーザーサブエージェントより優先されます。

**プラグインサブエージェント**は、インストールした[プラグイン](/docs/ja/plugins/overview)から来ます。これらはカスタムサブエージェントと一緒に自動的に読み込まれ、スコープ付き名の下の @-mention タイプアヘッドに表示されます。プラグインサブエージェント作成の詳細については、[プラグインコンポーネントリファレンス](/docs/ja/plugins/components#agents)を参照してください。

<Note>
  セキュリティ上の理由から、プラグインサブエージェントは `hooks`、`mcpServers`、または `permissionMode` frontmatter フィールドをサポートしていません。これらのフィールドはプラグインからエージェントを読み込むときに無視されます。これらが必要な場合は、エージェントファイルを `.claude/agents/` または `~/.claude/agents/` にコピーしてください。また、`settings.json` または `settings.local.json` の [`permissions.allow`](/docs/ja/settings-reference#permissions-allow) にルールを追加することもできますが、これらのルールはセッション全体に適用され、プラグインサブエージェントのみには適用されません。
</Note>

これらのスコープからのサブエージェント定義は、[エージェントチーム](/docs/ja/agent-teams#use-subagent-definitions-for-teammates)でも利用可能です。チームメイトをスポーンするときに、サブエージェントタイプを参照でき、Claude Code はその定義の一部をチームメイトに適用します。各表示モードで適用される部分については、[エージェントチーム](/docs/ja/agent-teams#use-subagent-definitions-for-teammates)を参照してください。

<h3 id="write-subagent-files">
  サブエージェントファイルを作成する
</h3>

サブエージェントファイルは設定用の YAML frontmatter を使用し、その後にマークダウンのシステムプロンプトが続きます。

<Note>
  Claude Code は `~/.claude/agents/` と `.claude/agents/` を監視します。ディスク上のサブエージェントファイルを追加または編集するか、Claude にそれを作成するよう依頼すると、Claude Code は数秒以内に変更を検出し、次の委任は再起動なしで更新された定義を使用します。

  3 つのケースではまだ再起動が必要です。

  * ウォッチャーはセッション開始時に存在していたディレクトリのみをカバーするため、新しい `agents` ディレクトリでスコープの最初のエージェントファイルを作成した後、再起動して読み込みます。
  * Claude Code は `--add-dir` または `/add-dir` で追加されたディレクトリ内の `.claude/agents/` を監視しないため、そこでサブエージェントを追加または編集した後、再起動して変更を読み込みます。
  * `--disable-slash-commands` で開始されたセッションはこれらのディレクトリをまったく監視しません。
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

frontmatter はサブエージェントのメタデータと設定を定義します。本体はサブエージェントの動作をガイドするシステムプロンプトになります。サブエージェントは、このシステムプロンプトと作業ディレクトリなどの基本的な環境詳細のみを受け取り、Claude Code システムプロンプトは受け取りません。

[非対話モード](/docs/ja/headless)では、[`--append-subagent-system-prompt`](/docs/ja/cli-reference#cli-flags)を渡して、テキストをすべてのサブエージェント（ネストされたサブエージェントを含む）のシステムプロンプトの末尾に追加します。ただし、[フォークされたサブエージェント](#fork-the-current-conversation)は除きます。これは会話独自のプロンプトを再利用します。Claude Code v2.1.205 以降が必要です。テキストがコマンドラインで渡すには長すぎる場合は、ファイルに保存して `--append-subagent-system-prompt-file` でパスを渡してください。ファイルフラグには Claude Code v2.1.261 以降が必要です。

サブエージェントはメイン会話の現在の作業ディレクトリで開始します。サブエージェント内では、`cd` コマンドは Bash または PowerShell ツール呼び出し間で永続化されず、メイン会話の作業ディレクトリに影響しません。代わりにサブエージェントにリポジトリの分離されたコピーを与えるには、[`isolation: worktree`](#supported-frontmatter-fields)を設定します。

`isolation: worktree` を持つサブエージェントは、その worktree 内で Bash および PowerShell コマンドを実行します。作業ディレクトリがメインチェックアウトに解決されるコマンド（例えば、サブエージェント実行中に worktree ディレクトリが削除された場合）はエラーで失敗します。v2.1.203 より前では、そのようなコマンドはメインチェックアウトで実行できました。

この作業ディレクトリチェックは、Claude Code を起動したディレクトリを含むリポジトリ全体をカバーします。セッションがリンクされた[worktree](/docs/ja/worktrees)で独自に実行される場合、チェックはその worktree がリンクされているメインチェックアウトもカバーします。v2.1.210 より前では、チェックは起動ディレクトリのみをカバーしていました。作業ディレクトリがリポジトリ内の他の場所（例えば、monorepo サブディレクトリから Claude Code を起動した場合のリポジトリルート）に解決されるコマンドは、失敗する代わりにそこで実行されました。

Bash コマンドの場合、Claude Code はコマンド自体を 2 つの方法でチェックします。

* git をメインチェックアウトにリダイレクトするコマンドをブロックします。
* コマンドテキストから、コマンドが実行する git がすべて worktree 内に留まることを確認できないコマンドを拒否します。例えば、コマンド名が実行時に計算される場合。

リダイレクトベクトルと形状ルールは[Claude Code が分離を強制する方法](/docs/ja/worktrees#how-claude-code-enforces-isolation)の下にリストされています。PowerShell コマンドは作業ディレクトリチェックのみを取得します。

[Monitor](/docs/ja/tools-reference#monitor-tool)コマンドは Bash コマンドと同じ作業ディレクトリおよびコマンドコンテンツチェックを通過します。

メイン会話自体が worktree で分離されて実行される場合、Claude Code はセッションとそれがスポーンするすべてのサブエージェント（`isolation: worktree` なしのサブエージェントを含む）に同じチェックを適用します。[Claude Code が分離を強制する方法](/docs/ja/worktrees#how-claude-code-enforces-isolation)を参照してください。

<h3 id="supported-frontmatter-fields">
  Frontmatter リファレンス
</h3>

以下のフィールドは YAML [frontmatter](/docs/ja/glossary#frontmatter) で使用できます。`name` と `description` のみが必須です。

複数単語のフィールド名は `maxTurns` や `disallowedTools` などの camelCase を使用し、テーブルと正確に一致する必要があります。Claude Code は認識しないフィールドを無視し、エラーを報告しません。サブエージェントファイルが読み込まれなかった理由を確認するには、[Claude Code がスキップするサブエージェントファイル](#subagent-files-claude-code-skips)を参照してください。

| フィールド             | 必須  | 説明                                                                                                                                                                                                                                                                                                                                                |
| :---------------- | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`            | はい  | 一意の識別子（`code-reviewer` や `reviewer-v2` など）。[フック](/docs/ja/hooks#subagentstart)はこの値を `agent_type` として受け取ります。ファイル名は一致する必要はありません。名前に `:` を含めることはできません。これは [plugin-scoped identifiers](/docs/ja/plugins/overview)（`my-plugin:reviewer` など）用に予約されています。Claude Code は `:` を含む名前のファイルを読み込まず、デバッグログにエラーをログします。v2.1.218 より前では、そのような名前は受け入れられていました                     |
| `description`     | はい  | Claude がこのサブエージェントに委任すべき場合                                                                                                                                                                                                                                                                                                                        |
| `tools`           | いいえ | サブエージェントが使用できる[ツール](#available-tools)。`Read, Grep, Glob` や YAML リストなどのカンマ区切り文字列として。省略した場合、サブエージェントで利用可能なすべてのツールを継承します。リスト内のエントリがツールに解決されない場合、サブエージェントは通常、エントリに名前を付けるエラーで[起動に失敗](/docs/ja/errors#agent-would-be-spawned-with-zero-tools)します。スキルをコンテキストにプリロードするには、ここで `Skill` をリストするのではなく、`skills` フィールドを使用します                                           |
| `disallowedTools` | いいえ | 継承または指定されたリストから削除するツール。`tools` と同じ形式。`Bash(git push *)` などの指定子を持つエントリは、[ツール全体](#available-tools)を削除します                                                                                                                                                                                                                                            |
| `model`           | いいえ | 使用する[モデル](#choose-a-model)。`sonnet`、`opus`、`haiku`、`fable`、`claude-opus-5-5` などの完全なモデル ID、または `inherit`。省略した場合、Claude Code は[サブエージェントモデル順序](#choose-a-model)でモデルを選択します                                                                                                                                                                            |
| `permissionMode`  | いいえ | [権限モード](#permission-modes)。`default`、`acceptEdits`、`auto`、`dontAsk`、`bypassPermissions`、`plan`、または `default` のエイリアスとしての `manual`。`manual` エイリアスには Claude Code v2.1.200 以降が必要です。[プラグインサブエージェント](#choose-the-subagent-scope)では無視されます                                                                                                                |
| `maxTurns`        | いいえ | サブエージェントが停止する前の最大 agentic ターン数。サブエージェントが制限に達すると、Claude Code は出力を部分的としてマークして返し、Claude は[再開](#resume-subagents)して続行できます。部分的なマーキングには Claude Code v2.1.246 以降が必要です                                                                                                                                                                                    |
| `skills`          | いいえ | 起動時にサブエージェントのコンテキストにプリロードする[スキル](/docs/ja/skills)。説明のみではなく、完全なスキルコンテンツが注入されます。サブエージェントは、Skill ツールを通じてリストされていないプロジェクト、ユーザー、およびプラグインスキルを呼び出すことができます                                                                                                                                                                                                      |
| `mcpServers`      | いいえ | このサブエージェントで利用可能な[MCP サーバー](/docs/ja/mcp)。各エントリは、既に設定されているサーバーを参照するサーバー名（例えば、`"slack"`）、または完全な[MCP サーバー設定](/docs/ja/mcp#installing-mcp-servers)を値として持つサーバー名をキーとするインライン定義のいずれかです。[プラグインサブエージェント](#choose-the-subagent-scope)では無視されます                                                                                                                          |
| `hooks`           | いいえ | このサブエージェントにスコープされた[ライフサイクルフック](#define-hooks-for-subagents)。[プラグインサブエージェント](#choose-the-subagent-scope)では無視されます                                                                                                                                                                                                                                   |
| `memory`          | いいえ | [永続メモリスコープ](#enable-persistent-memory)。`user`、`project`、または `local`。クロスセッション学習を有効にします                                                                                                                                                                                                                                                             |
| `background`      | いいえ | Claude がフォアグラウンドで実行するよう要求した場合でも、このサブエージェントをバックグラウンドに保つには `true` に設定します。[フォークモード](#turn-fork-mode-on-or-off)がオンの場合、Claude Code は Claude がスポーンするサブエージェントを既に[バックグラウンド](#run-subagents-in-foreground-or-background)で実行します                                                                                                                             |
| `omitClaudeMd`    | いいえ | このサブエージェントをユーザー、プロジェクト、およびローカル CLAUDE.md ファイルなしで起動するには `true` に設定します。[管理ポリシーファイル](/docs/ja/memory#how-claude-md-files-load)は引き続き読み込まれます。ただし、[管理サブエージェント](#choose-the-subagent-scope)は除きます。[委任プロンプト](#what-loads-at-startup)から必要なすべてを取得するサブエージェントに使用します。エージェントが `--agent` または `agent` 設定経由でメインセッションエージェントとして実行される場合は無視されます。Claude Code v2.1.271 以降が必要です |
| `effort`          | いいえ | このサブエージェントがアクティブな場合の努力レベル。セッション努力レベルをオーバーライドします。デフォルト。セッションから継承します。オプション。`low`、`medium`、`high`、`xhigh`、`max`。利用可能なレベルはモデルに依存します                                                                                                                                                                                                                   |
| `isolation`       | いいえ | サブエージェントを一時的な[git worktree](/docs/ja/worktrees)で実行するには `worktree` に設定します。これにより、親セッションの `HEAD` ではなく、デフォルトで[デフォルトブランチ](/docs/ja/worktrees#choose-the-base-branch)からブランチされたリポジトリの分離されたコピーが提供されます。サブエージェントが変更を加えない場合、worktree は自動的にクリーンアップされます                                                                                                                  |
| `color`           | いいえ | タスクリストとトランスクリプトでのサブエージェントの表示カラー。`red`、`blue`、`green`、`yellow`、`purple`、`orange`、`pink`、または `cyan` を受け入れます                                                                                                                                                                                                                                         |
| `initialPrompt`   | いいえ | このエージェントがメインセッションエージェント（`--agent` または `agent` 設定経由）として実行される場合、最初のユーザーターンとして自動送信されます。[コマンド](/docs/ja/commands)と[スキル](/docs/ja/skills)が処理されます。ユーザー提供のプロンプトの前に付加されます。[プラグインサブエージェント](#choose-the-subagent-scope)では無視されます                                                                                                                                       |
| `experimental`    | いいえ | 実験的オプションのマップ。その `cacheTtl` キーを `5m` または `1h` に設定して、このサブエージェントのリクエストの[プロンプトキャッシュライフタイム](/docs/ja/prompt-caching#choose-the-ttl-yourself)を選択します。frontmatter の[キャッシュライフタイム優先順位](/docs/ja/prompt-caching#choose-the-ttl-yourself)の場所。Claude Code は他の値を無視し、Claude サブスクリプションが使用クレジットを使用している間は `1h` を無視し、サブエージェントファイルからのみフィールドを読み取ります。Claude Code v2.1.248 以降が必要です |

`cacheTtl` を frontmatter のトップレベルではなく、`experimental` マップ内に書き込みます。

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Claude Code がスキップするサブエージェントファイル
</h4>

Claude Code は、frontmatter に以下の問題がある場合、プロジェクト、ユーザー、または管理 `agents` ディレクトリ、または `--add-dir` で追加したディレクトリの下のファイルをセッションで報告せずにスキップします。

* **`name` がない**。Claude Code はファイルをエージェントの横に保持されたドキュメントとして扱います。
* **ファイルの最初の行ではない開き `---`**。Claude Code はファイルに frontmatter がないと読み取り、ドキュメントとして扱います。
* **`-` で始まるか `:` を含む `name`**。Claude Code はファイルをスキップし、デバッグログにエラーを書き込みます。上記の表の `name` 行を参照してください。
* **`name` があるが `description` がない**。Claude Code はファイルをスキップし、理由をデバッグログに書き込みます。
* **解析されない YAML**。Claude Code はファイルからフィールドを読み取らず、スキップして、解析エラーをデバッグログに書き込みます。

デバッグログを表示するには、`--debug` で Claude Code を実行します。

frontmatter に `name` がない、または解析されない[プラグインサブエージェント](/docs/ja/plugins/components#agents)は、ファイル名の下で引き続き読み込まれます。

<h5 id="check-an-agents-directory-before-a-session">
  セッション前に `agents` ディレクトリをチェックする
</h5>

frontmatter が解析されない `agents` ディレクトリ内のファイルを見つけるには、例えば `.claude/agents` または `~/.claude/agents` に対して `claude plugin validate` を実行します。Claude Code は[名前を付けたディレクトリのみをチェック](/docs/ja/plugins/cli-reference#validate-a-directory)し、frontmatter が解析されるが `name` がないファイルにはフラグを立てません。Claude Code v2.1.233 以降が必要です。

<h3 id="choose-a-model">
  モデルを選択する
</h3>

`model` フィールドはサブエージェントが使用するモデルを制御します。

* **モデルエイリアス**。利用可能なエイリアスの 1 つを使用します。`sonnet`、`opus`、`haiku`、または `fable`
* **完全なモデル ID**。`claude-opus-5-5` または `claude-sonnet-5` などの完全なモデル ID を使用します。`--model` フラグと同じ値を受け入れます
* **inherit**。メイン会話と同じモデルを使用します

Claude がサブエージェントを呼び出すとき、その特定の呼び出しに対して `model` パラメータを渡すこともできます。Claude Code はサブエージェントのモデルをこの順序で解決します。

1. 呼び出しごとの `model` パラメータ
2. サブエージェント定義の `model` frontmatter。`inherit` はメイン会話のモデルを選択します
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/ja/model-config#environment-variables)環境変数。モデルエイリアスまたはモデル ID に設定した場合
4. メイン会話のモデル

2 つのケースでは、`opus` などのファミリエイリアスが、呼び出しごとのパラメータまたは frontmatter で、メイン会話のモデルの代わりに[エイリアスが指す](/docs/ja/model-config#model-aliases)バージョンに解決されます。

* **メイン会話のモデルがそのファミリに属する**。サブエージェントはメイン会話の正確なモデル（`[1m]` サフィックスを含む）で実行されるため、メイン会話と同じ[拡張コンテキスト](/docs/ja/model-config#extended-context)ウィンドウを取得します。
* **Claude Code がメイン会話のモデルファミリを判断できない。[Anthropic API 以外のプロバイダー](/docs/ja/third-party-integrations)上で**。これは Claude Code がバッキングモデルに解決していない[アプリケーション推論プロファイル ARN](/docs/ja/amazon-bedrock#iam-configuration)を持つ Amazon Bedrock で発生する可能性があります。このケースは `opus` エイリアスのみをカバーし、[`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/ja/model-config#environment-variables)を設定した場合は適用されません。`opus` はその後、設定したモデルに解決されるため。

`CLAUDE_CODE_SUBAGENT_MODEL` のエイリアスは、メイン会話のファミリに名前を付けた場合でも、常にエイリアスが指すバージョンに解決されます。

`CLAUDE_CODE_SUBAGENT_MODEL` を単独で設定しても、組み込みの Explore および Plan サブエージェントが実行されるモデルは変わりません。変更するには、[すべてのサブエージェントを 1 つのモデルで実行](#run-every-subagent-on-one-model)を参照してください。

v2.1.251 より前では、`CLAUDE_CODE_SUBAGENT_MODEL` はこの順序で最初に来て、呼び出しごとのパラメータと frontmatter（`model: inherit` を含む）の両方をオーバーライドしました。

変数を `inherit` に設定することは、設定を解除するのと同じです。v2.1.196 より前では、その値はサブエージェントをメイン会話のモデルに強制し、他のソースを無視しました。

Claude Code は、呼び出しごとのパラメータ、frontmatter、および環境変数の値を組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection)許可リストに対してチェックします。ブロックされた値の場合、別のモデルに置き換えます。

* `opus` などのファミリエイリアスがブロックされた場合、Claude Code はサブエージェントを許可リストが許可するそのファミリの最新バージョンで実行します。`/model` と同じ[置換ルールとプロバイダースコープ](/docs/ja/model-config#restrict-model-selection)に従います。v2.1.222 より前では、Claude Code はブロックされたファミリエイリアスについても継承されたモデルでサブエージェントを実行していました。
* その他のブロックされた値の場合、その置換が動作しないプロバイダー、または許可リストがファミリのバージョンを許可しない場合、Claude Code は代わりに継承されたモデルでサブエージェントを実行します。`CLAUDE_CODE_SUBAGENT_MODEL` を設定した場合、Claude Code はそのモデルを最初に試します。これらの同じルールの下で。

対話型セッションでは、Claude Code は要求されたモデルとサブエージェントが実行されるモデルに名前を付ける警告を表示します。どちらの置換についても。

サブエージェントが実行されているモデルを確認するには、[`/tasks`](/docs/ja/commands)を実行します。Claude Code はサブエージェントの行でモデルに名前を付け、サブエージェントの定義またはそれがフォークされたスキルが [`effort`](#supported-frontmatter-fields)を設定する場合、[努力レベル](/docs/ja/model-config#adjust-effort-level)を追加します。Claude Code v2.1.242 以降が必要です。

呼び出しごとの `model` パラメータは、サブエージェントが[再開または後続メッセージが送信](#resume-subagents)される場合にも適用されるため、サブエージェントはそのモデルに留まります。v2.1.211 より前では、再開は呼び出しごとの値をドロップし、サブエージェントは定義の `model` フィールドまたはメイン会話のモデルに戻りました。

v2.1.198 以降、サブエージェントはメイン会話の[拡張思考](/docs/ja/model-config#extended-thinking)設定も継承します。セッションで思考がオンの場合、サブエージェントではオンになり、オフの場合、オフのままです。サブエージェントごとの思考設定はありません。v2.1.198 より前では、サブエージェントはメイン会話の設定に関係なく、拡張思考が無効で実行されていました。

<h4 id="run-every-subagent-on-one-model">
  すべてのサブエージェントを 1 つのモデルで実行する
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` はデフォルトであるため、サブエージェントの定義または Claude が渡すモデルは引き続き優先されます。すべてのサブエージェント、[チームメイト](/docs/ja/agent-teams#specify-teammates-and-models)、および[ワークフローエージェント](/docs/ja/workflows)に 1 つのモデルを適用するには、`CLAUDE_CODE_SUBAGENT_MODEL_FORCE` を `1` に設定します。Claude Code v2.1.257 以降が必要です。

* 両方の変数を設定した場合、サブエージェントは `CLAUDE_CODE_SUBAGENT_MODEL` のモデルで実行されます。
* `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` のみを設定した場合、サブエージェントはメイン会話のモデルで実行されます。

例えば、すべてのサブエージェントを Haiku で実行するには、[設定ファイル](/docs/ja/settings)の `env` ブロックで両方の変数を設定します。

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

設定が有効になったことを確認するには、サブエージェントが実行されている間に [`/tasks`](/docs/ja/commands)を実行します。サブエージェントの行は、実行されるモデルを示します。

`CLAUDE_CODE_SUBAGENT_MODEL_FORCE` が[オン](/docs/ja/env-vars)の場合、Claude Code は組み込みの Explore および Plan サブエージェントを含むすべてのサブエージェント定義の `model` フィールドを無視し、Claude はサブエージェント開始時にモデルを渡すことができません。2 種類のサブエージェントはメイン会話のモデルで引き続き実行されます。

* [フォーク](#fork-the-current-conversation)
* `model: inherit` を持つ[サブエージェントで実行されるスキル](/docs/ja/skills#run-skills-in-a-subagent)

`CLAUDE_CODE_SUBAGENT_MODEL_FORCE` のみを設定した場合、組み込みの Explore サブエージェントは[モデルキャップ](#built-in-subagents)を保持します。

<h3 id="control-subagent-capabilities">
  サブエージェント機能を制御する
</h3>

ツールアクセス、権限モード、および条件付きルールを通じて、サブエージェントが何をできるかを制御できます。

<h4 id="available-tools">
  利用可能なツール
</h4>

サブエージェントは、メイン会話で利用可能な[組み込みツール](/docs/ja/tools-reference)と MCP ツールを継承しますが、2 つのフィルタで絞られます。最初のフィルタはすべてのサブエージェントから短いツールリストを削除し、2 番目のフィルタは[バックグラウンド](#run-subagents-in-foreground-or-background)で実行されるサブエージェント（デフォルト）の組み込みツールセットを削減します。macOS、Linux、および WSL では、メイン会話にない場合、サブエージェントは Glob および Grep ツールを受け取ることもできます。[Glob ツール動作](/docs/ja/tools-reference#glob-tool-behavior)で説明されています。[フォーク](#fork-the-current-conversation)は両方のフィルタをスキップし、メイン会話の正確なツールプールを受け取ります。最初のフィルタは、`tools` フィールドにリストされている場合でも、これらのツールを削除します。

* `Agent`。サブエージェントが[深さ制限](#let-subagents-spawn-their-own-subagents)にある場合。[フォーク](#fork-the-current-conversation)ではツールはリストされたままですが、代わりにエラーを返します
* `AskUserQuestion`
* `EndConversation`。メイン会話のみを終了できます。[EndConversation ツール動作](/docs/ja/tools-reference#endconversation-tool-behavior)を参照してください
* `EnterPlanMode`
* `ExitPlanMode`。サブエージェントの [`permissionMode`](#permission-modes)が `plan` でない限り
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

2 番目のフィルタはバックグラウンドで実行されるサブエージェントに適用されます。`Agent` と `ExitPlanMode` を除き、これらはサブエージェントが実行される場所に関係なく最初のフィルタの条件に従います。バックグラウンドサブエージェントはすべての MCP ツールを保持しますが、これらの組み込みツールのみです。`Read`、`Grep`、`Glob`、`LSP`、`Bash`、`PowerShell`、`Edit`、`Write`、`NotebookEdit`、`WebFetch`、`WebSearch`、`TodoWrite`、`Skill`、`ToolSearch`、`EnterWorktree`、`ExitWorktree`、`Monitor`、`TaskStop`、`SendMessage`、および `Artifact`。それを報告するサブエージェント用の [`SubagentHandback`](/docs/ja/tools-reference)。Claude Code はバックグラウンドサブエージェントから他のすべての組み込みツールを削除します。継承またはリストされているかどうかに関わらず、`tools` フィールドで。同じ定義はフォアグラウンドとバックグラウンドで異なるツールに解決できます。削除は、`tools` リストが[何も解決しない](/docs/ja/errors#agent-would-be-spawned-with-zero-tools)場合を除き、エラーを報告しません。

v2.1.280 より前では、バックグラウンドサブエージェントは `LSP` を使用できませんでした。

[`ListAgents`](/docs/ja/cross-session-messaging)は、他の組み込みツールと同様にこれらのフィルタに従います。フォアグラウンドサブエージェントは、クロスセッションメッセージングが有効なセッションで継承し、バックグラウンドサブエージェントは保持しません。

[エージェントチーム](/docs/ja/agent-teams)のチームメイトは、さらにタスクツールと cron ツールを保持します。`TaskCreate`、`TaskGet`、`TaskList`、`TaskUpdate`、`CronCreate`、`CronDelete`、および `CronList`。

[Task ツールのないセッション](/docs/ja/tools-reference#task-tool-availability)では、Claude Code はサブエージェントにもタスクツールを提供しません。サブエージェントが異なるモデルを実行する場合でも。インプロセスチームメイトはセッションと同じ方法に従いますが、チームメイトが独自の[分割ペイン](/docs/ja/agent-teams#choose-a-display-mode)で実行される場合、別の Claude Code プロセスとして実行されるため、独自のモデルが決定します。

ツールを制限するには、`tools` フィールドを許可リストとして、または `disallowedTools` フィールドを拒否リストとして使用します。この例は `tools` を使用して、Read、Grep、Glob、および Bash のみを許可します。サブエージェントはファイルを編集、書き込み、または MCP ツールを使用することはできません。

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

この例は `disallowedTools` を使用して、Write と Edit を除く利用可能なツールを継承します。サブエージェントは Bash、MCP ツール、およびプールの残りを保持します。

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

両方が設定されている場合、`disallowedTools` が最初に適用され、次に `tools` が残りのプールに対して解決されます。両方にリストされているツールは削除されます。

`tools` リスト内の何もツールに解決されない場合（例えば、すべてのエントリが誤字であるか、サブエージェントで利用できないツールに名前を付けている場合）、Claude Code は通常、サブエージェントの起動を拒否し、Agent ツールは解決されないエントリに名前を付けるエラーを返します。[Agent would be spawned with zero tools](/docs/ja/errors#agent-would-be-spawned-with-zero-tools)を参照してください。メッセージと各エントリを修正する方法。v2.1.208 より前では、そのサブエージェントはツールなしで起動し、空または混乱した結果を返す可能性があります。

両方のフィールドは、正確なツール名に加えて MCP サーバーレベルのパターンを受け入れます。`mcp__<server>` または `mcp__<server>__*` は、名前付きサーバーからすべてのツールを付与または削除します。`disallowedTools` では、`mcp__*` はすべての MCP ツールをすべてのサーバーから削除します。この例は、`github` MCP サーバーからすべてのツールを削除しながら、他のサーバーのツールとプール内の組み込みツールを保持します。

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

`disallowedTools` エントリに `Bash(git push *)` などの指定子がある場合、マッチングコマンドのみではなく、ツール全体をサブエージェントから削除します。Bash を保持し、特定のコマンドをブロックするには、設定に [Bash 拒否ルール](/docs/ja/permissions#bash)（`Bash(git push *)` など）を `permissions.deny` に追加します。ルールはメイン会話とサブエージェントに適用されます。

<h4 id="restrict-which-subagents-can-be-spawned">
  スポーンできるサブエージェントを制限する
</h4>

エージェントが `claude --agent` でメインスレッドとして実行される場合、Agent ツールを使用してサブエージェントをスポーンできます。スポーンできるサブエージェントタイプを制限するには、`tools` フィールドで `Agent(agent_type)` 構文を使用します。

<Note>バージョン 2.1.63 では、Task ツールが Agent に名前変更されました。設定とエージェント定義の既存の `Task(...)` 参照は引き続きエイリアスとして機能します。</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

これは許可リストです。`worker` と `researcher` サブエージェントのみをスポーンできます。エージェントが他のタイプをスポーンしようとすると、リクエストは失敗し、エージェントはプロンプトで許可されたタイプのみを見ます。特定のエージェントをブロックしながら他のすべてを許可するには、代わりに [`permissions.deny`](#disable-specific-subagents)を使用します。

制限なしでサブエージェントをスポーンできるようにするには、括弧なしで `Agent` を使用します。

```yaml theme={null}
tools: Agent, Read, Bash
```

`tools` リストから `Agent` を完全に省略した場合、エージェントは Agent ツールでサブエージェントをスポーンできません。

`Agent(agent_type)` 許可リスト構文は、`claude --agent` でメインスレッドとして実行されるエージェントにのみ適用されます。サブエージェント定義では、`tools` に `Agent` をリストするとそのサブエージェントが[深さ制限](#let-subagents-spawn-their-own-subagents)を許可する限り独自のサブエージェントをスポーンできますが、括弧内のタイプリストは無視されます。

<h4 id="scope-mcp-servers-to-a-subagent">
  MCP サーバーをサブエージェントにスコープする
</h4>

`mcpServers` フィールドを使用して、メイン会話で利用できない[MCP](/docs/ja/mcp)サーバーへのアクセスをサブエージェントに与えます。ここで定義されたインラインサーバーは、サブエージェント開始時に接続され、[エージェントファイルのフォルダの信頼ルール](#inline-server-trust)の対象となり、終了時に切断されます。文字列参照は親セッションの接続を共有します。

<Note>
  `mcpServers` フィールドは、エージェントファイルが実行できる両方のコンテキストに適用されます。

  * Agent ツールまたは @-mention を通じてスポーンされたサブエージェント
  * [`--agent`](#invoke-subagents-explicitly)または `agent` 設定で起動されたメインセッション

  エージェントがメインセッションの場合、インラインサーバー定義は [`.mcp.json`](/docs/ja/mcp)および設定ファイルのサーバーと一緒に起動時に接続され、[エージェントファイルのフォルダの信頼ルール](#inline-server-trust)の下にあります。`/mcp` では、以前に使用したリモート（HTTP または SSE）サーバーは [`cached` ステータス](/docs/ja/mcp#managing-your-servers)を代わりに表示できます。Claude Code は Claude が最初にそのツールの 1 つを呼び出すときに接続します。
</Note>

リスト内の各エントリは、インラインサーバー定義またはセッションで既に設定されている MCP サーバーを参照する文字列のいずれかです。

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

インライン定義は `.mcp.json` サーバーエントリと同じスキーマを使用し、サーバー名でキー付けされ、`stdio`、`http`、`sse`、および `ws` タイプをサポートします。

MCP サーバーをメイン会話から完全に除外し、そのツール説明がコンテキストを消費するのを避けるには、`.mcp.json` ではなくここでインラインで定義します。サブエージェントはツールを取得します。親会話は取得しません。

<span id="inline-server-trust" />Claude Code は、プロジェクトの `.claude/agents/` ディレクトリ、または `--add-dir` ディレクトリの `.claude/agents/` にあるエージェントファイルからインラインサーバーを読み込みます。エージェントファイルが来たフォルダを[信頼](/docs/ja/permissions#what-runs-before-you-trust-a-folder)した後のみです。v2.1.238 より前では、Claude Code はこれらのサーバーを信頼チェックなしで読み込みました。

* **カウントされない信頼**。親フォルダの信頼、および `-p` または SDK セッションが[設定ファイルのフック](/docs/ja/permissions#what-runs-before-you-trust-a-folder)に対して取得する自動信頼
* **それまで**。Claude Code はそのエージェントファイル内のすべてのインラインサーバーをスキップし、`~/.claude.json` の正確な `projects["<path>"].hasTrustDialogAccepted` キーをデバッグログに書き込みます
* **`--add-dir` ディレクトリ**。信頼されたワークスペースのリポジトリの外側のディレクトリは、その `.claude/agents/` ファイルがワークスペースの信頼を継承しないため、独自の信頼エントリが必要です

Claude Code は、エージェントファイルが来たフォルダの信頼をチェックせずに 2 種類のサーバーを読み込みます。

* 既に設定されているサーバーを参照する名前
* `~/.claude/agents/` のエージェントファイル、`--agents` または SDK `agents` オプションで渡すもの、または管理設定が提供するもの内のインラインサーバー

メインセッションに適用される MCP 制限はサブエージェント frontmatter で宣言されたサーバーもカバーします。

* [`--strict-mcp-config`](/docs/ja/cli-reference)および [`--bare`](/docs/ja/cli-reference)
* [エンタープライズ管理 MCP 設定](/docs/ja/managed-mcp)
* [`allowedMcpServers` および `deniedMcpServers` ポリシー](/docs/ja/managed-mcp#policy-based-control-with-allowlists-and-denylists)

これらの 1 つがサーバーをブロックする場合、Claude Code はスキップし、ブロックされたサーバーに名前を付ける警告を表示します。

管理設定の制限は、定義方法に関係なくすべてのサブエージェントに適用されます。`--strict-mcp-config` は `--agents` または SDK `agents` オプション経由でインラインで渡すサーバーをフィルタリングしません。これらは明示的な呼び出し元入力であるため。

<h4 id="permission-modes">
  権限モード
</h4>

`permissionMode` を設定して、サブエージェントが実行される権限モードを選択します。モードの設定値を使用するため、Manual モードは `default` です。設定を解除した場合、サブエージェントはメイン会話のモードを継承します。これは Pro、Max、および Team プランで[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)として開始されます。設定または組織が変更しない限り。

メイン会話の権限モードは、Claude Code が設定した値を使用するかどうかを決定します。

* メイン会話が `bypassPermissions`、`acceptEdits`、または[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)にある場合、サブエージェントはそのモードで実行され、Claude Code は設定した `permissionMode` を無視します。自動モードでは、分類器はメイン会話のブロックおよび許可ルールでサブエージェントのツール呼び出しを評価します。サブエージェントが終了すると、分類器はその作業と最終レポートもレビューしてから、レポートが配信されます。[自動モードがサブエージェントを処理する方法](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)を参照してください。
* メイン会話が `default`、`dontAsk`、または `plan` モードにある場合、サブエージェントは設定した権限モードで実行されます。ただし `bypassPermissions` を除きます。`bypassPermissions` を宣言するサブエージェントはメイン会話のモードを保持します。`bypassPermissions` 例外には Claude Code v2.1.267 以降が必要です。

`permissionMode` はこれらの値を受け入れ、`default` のエイリアスとして `manual` を受け入れます。

| モード                 | 動作                                                                                                                                                                                                                                                                              |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Manual モード。権限を求めるプロンプト                                                                                                                                                                                                                                                          |
| `acceptEdits`       | ファイル編集と作業ディレクトリまたは `additionalDirectories` 内のパスの一般的なファイルシステムコマンドを自動受け入れ                                                                                                                                                                                                         |
| `auto`              | [自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)。バックグラウンド分類器がコマンドと保護されたディレクトリ書き込みをレビュー                                                                                                                                                                            |
| `dontAsk`           | 権限プロンプトを自動拒否。明示的に許可されたツールは引き続き機能します。`AskUserQuestion`、[`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool)とマークされた MCP ツール、およびコネクタツール[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)したセッション（その設定が Claude Code に到達する場合）は、許可されている場合でも拒否されます |
| `bypassPermissions` | [権限プロンプトをスキップ](/docs/ja/permission-modes#skip-all-checks-with-bypasspermissions-mode)。サブエージェントはメイン会話がそうである場合のみこのモードで実行されます                                                                                                                                                           |
| `plan`              | Plan モード（読み取り専用探索）                                                                                                                                                                                                                                                              |

<h4 id="preload-skills-into-subagents">
  スキルをサブエージェントにプリロードする
</h4>

`skills` フィールドを使用して、スキルコンテンツをサブエージェントのコンテキストに起動時に注入します。これにより、実行中にスキルを発見して読み込む必要なく、サブエージェントにドメイン知識を提供します。

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

リストされた各スキルの完全なコンテンツは起動時にサブエージェントのコンテキストに注入されます。このフィールドはプリロードされるスキルを制御し、サブエージェントがアクセスできるスキルではありません。なしで、サブエージェントは実行中に Skill ツールを通じてプロジェクト、ユーザー、およびプラグインスキルを発見して呼び出すことができます。スキルを完全に呼び出すことを防ぐには、[`tools`](#available-tools)リストから `Skill` を省略するか、`disallowedTools` に追加します。

[`disable-model-invocation: true`](/docs/ja/skills#control-who-invokes-a-skill)を設定するスキルはプリロードできません。プリロードは Claude が呼び出すことができるスキルの同じセットから描画するため。これには、バンドルされた `/verify` スキルが含まれます。実行できるのはあなただけなので、プリロードすることもできません。

リストされたスキルが見つからないか無効な場合（例えば、組織のポリシーによって）、Claude Code はスキップし、デバッグログに警告をログします。

<Note>
  これは[スキルをサブエージェントで実行する](/docs/ja/skills#run-skills-in-a-subagent)の逆です。サブエージェント内の `skills` を使用すると、サブエージェントはシステムプロンプトを制御し、スキルコンテンツを読み込みます。スキル内の `context: fork` を使用すると、スキルコンテンツが指定したエージェントに注入されます。どちらの場合も、サブエージェントは会話履歴なしで開始されます。
</Note>

<h4 id="enable-persistent-memory">
  永続メモリを有効にする
</h4>

`memory` フィールドはサブエージェントに、会話全体で存続する永続ディレクトリを提供します。サブエージェントはこのディレクトリを使用して、コードベースパターン、デバッグインサイト、アーキテクチャ決定などの知識を時間をかけて構築します。

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

メモリがどの程度広く適用されるべきかに基づいてスコープを選択します。

| スコープ      | 場所                                            | 使用時期                                         |
| :-------- | :-------------------------------------------- | :------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | サブエージェントはすべてのプロジェクト全体で学習を記憶すべき               |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | サブエージェントの知識はプロジェクト固有で、バージョン管理経由で共有可能         |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | サブエージェントの知識はプロジェクト固有だが、バージョン管理にチェックインすべきではない |

サブエージェントメモリは[自動メモリ](/docs/ja/memory#auto-memory)の一部です。自動メモリをオフにした場合、`autoMemoryEnabled` 設定または `CLAUDE_CODE_DISABLE_AUTO_MEMORY` を使用すると、`memory` フィールドは効果がなく、サブエージェントはメモリ指示またはメモリツールアクセスなしで起動されます。以下で説明します。

メモリが有効な場合。

* サブエージェントのシステムプロンプトには、メモリディレクトリへの読み取りと書き込みの指示が含まれます。
* サブエージェントのシステムプロンプトには、メモリディレクトリの `MEMORY.md` の最初の 200 行または 25KB（どちらか先）も含まれます。その制限を超える場合は `MEMORY.md` をキュレートする指示付き。
* Read、Write、および Edit ツールは自動的に有効になり、サブエージェントはメモリファイルを管理できます。

<h5 id="persistent-memory-tips">
  永続メモリのヒント
</h5>

* `project` は推奨されるデフォルトスコープです。サブエージェント知識をバージョン管理経由で共有可能にします。
* サブエージェントに作業開始前にメモリを確認するよう依頼します。「このプルリクエストをレビューし、以前に見たパターンについてメモリを確認してください。」
* タスク完了後にメモリを更新するようサブエージェントに依頼します。「完了したので、学習したことをメモリに保存してください。」時間をかけて、これはサブエージェントをより効果的にする知識ベースを構築します。
* メモリ指示をサブエージェントのマークダウンファイルに直接含めて、積極的にメモリを維持します。

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  フックを使用した条件付きルール
</h4>

ツール使用をより動的に制御するには、`PreToolUse` フックを使用して、実行前に操作を検証します。これは、ツールの一部の操作を許可しながら他をブロックする必要がある場合に便利です。

この例は、読み取り専用データベースクエリのみを許可するサブエージェントを作成します。`PreToolUse` フックは、各 Bash コマンド実行前に `command` で指定されたスクリプトを実行します。

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code は[フック入力を JSON として](/docs/ja/hooks#pretooluse-input)stdin 経由でフックコマンドに渡します。検証スクリプトはこの JSON を読み取り、Bash コマンドを抽出し、[終了コード 2](/docs/ja/hooks#exit-code-2-behavior-per-event)で書き込み操作をブロックします。

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

macOS および Linux では、スクリプトを実行可能にするか、フックが失敗します。何もブロックしません。

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

ルールをテストするには、サブエージェントに `UPDATE` ステートメントを実行するよう依頼します。スクリプトは終了コード 2 で終了し、Claude Code はコマンドをブロックし、サブエージェントは `Blocked: Only SELECT queries are allowed` メッセージを見ます。

完全な入力スキーマについては[フック入力](/docs/ja/hooks#pretooluse-input)を、終了コードが動作に影響する方法については[終了コード](/docs/ja/hooks#exit-code-output)を参照してください。Windows では、PowerShell でフックスクリプトを作成し、[PowerShell でフックを実行](/docs/ja/hooks#windows-powershell-tool)に示すように、フックエントリに `shell: powershell` を追加します。

<h4 id="disable-specific-subagents">
  特定のサブエージェントを無効にする
</h4>

[設定](/docs/ja/settings-reference#permission-settings)の `deny` 配列にサブエージェントを追加して、Claude が特定のサブエージェントを使用するのを防ぐことができます。`Agent(subagent-name)` 形式を使用します。ここで `subagent-name` はサブエージェントの name フィールドと一致します。

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

これは組み込みとカスタムサブエージェントの両方で機能します。`--disallowedTools` CLI フラグを使用することもできます。

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

権限ルールの詳細については、[権限ドキュメント](/docs/ja/permissions#tool-specific-permission-rules)を参照してください。

<h3 id="define-hooks-for-subagents">
  サブエージェント用のフックを定義する
</h3>

サブエージェントはサブエージェントのライフサイクル中に実行される[フック](/docs/ja/hooks)を定義できます。フックを設定する 2 つの方法があります。

* **サブエージェントの frontmatter で**。そのサブエージェントがアクティブな間のみ実行されるフックを定義
* **`settings.json` で**。セッション全体のフック。サブエージェント内でも発火します。`PreToolUse` および `PostToolUse` などのツールイベントはメイン会話と同じ方法でサブエージェントのツール呼び出しに対して発火し、`SubagentStart` および `SubagentStop` はサブエージェント開始時および終了時に発火します

[設定ファイル、管理ポリシー設定、およびプラグイン](/docs/ja/hooks#hook-locations)からのフックはすべてサブエージェント内に適用されるため、`settings.json` の `PreToolUse` フックはサブエージェントが使用するすべてのツールの前にも実行されます。

<h4 id="hooks-in-subagent-frontmatter">
  サブエージェント frontmatter のフック
</h4>

サブエージェントのマークダウンファイルでフックを直接定義します。これらのフックはその特定のサブエージェントがアクティブな間のみ実行され、終了時にクリーンアップされます。

<Note>
  Frontmatter フックは、Agent ツールまたは @-mention を通じてサブエージェントとしてスポーンされた場合、および [`--agent`](#invoke-subagents-explicitly)または `agent` 設定経由でメインセッションとして実行される場合に発火します。メインセッションの場合、[`settings.json`](/docs/ja/hooks)で定義されたフックと一緒に実行されます。
</Note>

プロジェクトレベルのサブエージェントの frontmatter フックを実行させるには、エージェントファイルを含むフォルダの[ワークスペース信頼ダイアログ](/docs/ja/permissions#project-allow-rules-and-workspace-trust)を受け入れます。`~/.claude/agents/` のユーザーレベルサブエージェントからのフックと `--agents` で渡す定義は、このステップなしで実行されます。`--add-dir` で信頼されたワークスペースのリポジトリの外側からフォルダを追加した場合、そのフォルダを個別に信頼します。その `.claude/agents/` フックはワークスペースの付与を継承しません。

フォルダを信頼するまで、サブエージェントは引き続き実行されますが、Claude Code は frontmatter フックをスキップし、フォルダを信頼する方法を説明するエラーをデバッグログにログします。これは設定ファイルのフックのルールより厳しいです。親フォルダの信頼は十分ではなく、`-p` セッションは信頼されたとしてカウントされません。[フォルダを信頼する前に実行されるもの](/docs/ja/permissions#what-runs-before-you-trust-a-folder)は 2 つを比較します。v2.1.218 より前では、frontmatter フックは信頼していないフォルダから実行でき、非対話型セッションを含めて実行できました。

すべての[フックイベント](/docs/ja/hooks#hook-events)がサポートされています。サブエージェントの最も一般的なイベントは。

| イベント          | マッチャー入力 | 発火時期                                     |
| :------------ | :------ | :--------------------------------------- |
| `PreToolUse`  | ツール名    | サブエージェントがツールを使用する前                       |
| `PostToolUse` | ツール名    | サブエージェントがツールを使用した後                       |
| `Stop`        | （なし）    | サブエージェントが終了する場合（実行時に `SubagentStop` に変換） |

この例は `PreToolUse` フックで Bash コマンドを検証し、`PostToolUse` でファイル編集後にリンターを実行します。

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

エージェントがサブエージェントとして呼び出される場合、frontmatter の `Stop` フックは自動的に `SubagentStop` イベントに変換されます。

<h4 id="project-level-hooks-for-subagent-events">
  サブエージェントイベント用のプロジェクトレベルフック
</h4>

メインセッションでサブエージェントライフサイクルイベントに応答するフックを `settings.json` で設定します。

| イベント            | マッチャー入力    | 発火時期          |
| :-------------- | :--------- | :------------ |
| `SubagentStart` | エージェントタイプ名 | サブエージェント実行開始時 |
| `SubagentStop`  | エージェントタイプ名 | サブエージェント完了時   |

両方のイベントは、名前でエージェントタイプをターゲットするマッチャーをサポートします。マッチャー値は、プロジェクトレベルおよびユーザーレベルサブエージェントの frontmatter `name`、または[プラグインサブエージェント](/docs/ja/plugins/components#agents)の `my-plugin:db-agent` などのプラグインスコープ識別子です。スコープ付き名にはコロンが含まれるため、[アンカーなし正規表現](/docs/ja/hooks#matcher-patterns)として評価されます。`^my-plugin:db-agent$` のように `^` と `$` でアンカーして、そのエージェントのみをマッチします。

この例は、`db-agent` サブエージェント開始時のみセットアップスクリプトを実行し、サブエージェント停止時にクリーンアップスクリプトを実行します。

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

ハイフン付きマッチャー（`db-agent`）は Claude Code v2.1.195 以降で正確にマッチします。以前のバージョンではアンカーなし正規表現として評価され、`prod-db-agent` などそれを含むエージェントタイプについても発火します。これらのバージョンでは `^db-agent$` としてアンカーします。

完全なフック設定形式については[フック](/docs/ja/hooks)を参照してください。

<h2 id="work-with-subagents">
  サブエージェントを使用する
</h2>

<h3 id="understand-automatic-delegation">
  自動委譲を理解する
</h3>

Claude は、リクエスト内のタスク説明、サブエージェント設定の `description` フィールド、および現在のコンテキストに基づいて、タスクを自動的に委譲します。プロアクティブな委譲を促進するには、サブエージェントの説明フィールドに「use proactively」などのフレーズを含めてください。

説明は簡潔に保ってください。サブエージェントの説明の合計が [15,000 トークンの制限](/docs/ja/errors#agent-descriptions-are-over-the-15000-token-limit) を超えると、Claude Code は起動時に警告を表示しますが、すべてのサブエージェントは読み込まれます。

サブエージェントが [プラグイン](/docs/ja/plugins/overview) に含まれている場合、現実的なプロンプトで Claude がそれに確実に委譲するかどうかを測定できます。1 つずつチェックする代わりに、[`claude plugin eval`](/docs/ja/plugin-evals) は各プロンプトをプラグインの有無で実行し、結果をスコアリングします。

<h3 id="invoke-subagents-explicitly">
  サブエージェントを明示的に呼び出す
</h3>

自動委譲では不十分な場合、サブエージェントを自分で要求できます。3 つのパターンは、1 回限りの提案からセッション全体のデフォルトまでエスカレートします。

* **自然言語**: プロンプトでサブエージェントに名前を付けます。Claude が委譲するかどうかを決定します
* **@-mention**: サブエージェントが 1 つのタスクで実行されることを保証します
* **セッション全体**: セッション全体が `--agent` フラグまたは `agent` 設定を介して、そのサブエージェントのシステムプロンプト、ツール制限、およびモデルを使用します

自然言語の場合、特別な構文はありません。サブエージェントに名前を付けると、Claude は通常委譲します。

```text wrap theme={null}
test-runner サブエージェントを使用して失敗したテストを修正してください
code-reviewer サブエージェントに最近の変更を確認させてください
```

**サブエージェントを @-mention してください。** `@` を入力し、ファイルを @-mention するのと同じ方法で、タイプアヘッドからサブエージェントを選択します。これにより、Claude に選択を任せるのではなく、特定のサブエージェントが実行されることが保証されます。

```text wrap theme={null}
@"code-reviewer (agent)" 認証の変更を確認してください
```

フルメッセージは引き続き Claude に送信され、Claude はあなたが要求したことに基づいてサブエージェントのタスクプロンプトを作成します。@-mention は Claude が呼び出すサブエージェントを制御し、受け取るプロンプトではありません。

有効な [プラグイン](/docs/ja/plugins/overview) によって提供されるサブエージェントは、`my-plugin:code-reviewer` や `my-plugin:review:security` などのスコープ付き名でタイプアヘッドに表示されます。プラグインが [エージェントをサブフォルダに整理](#choose-the-subagent-scope) する場合です。セッションで現在実行されている名前付きバックグラウンドサブエージェントもタイプアヘッドに表示され、名前の横にステータスが表示されます。

ピッカーを使用せずに手動でメンションを入力することもできます。ローカルサブエージェントの場合は `@agent-<name>`、プラグインサブエージェントの場合は `@agent-` の後にスコープ付き名を入力します。例えば `@agent-my-plugin:code-reviewer` です。このフォームを入力している間、タイプアヘッドはエージェントではなくファイルマッチを表示します。エージェントメンションは送信時に解決されます。

**セッション全体をサブエージェントとして実行します。** [`--agent <name>`](/docs/ja/cli-reference) を渡して、メインスレッド自体がそのサブエージェントのシステムプロンプト、ツール制限、およびモデルを引き継ぐセッションを開始します。

```bash theme={null}
claude --agent code-reviewer
```

サブエージェントのシステムプロンプトは、[`--system-prompt`](/docs/ja/cli-reference) と同じ方法で、デフォルトの Claude Code システムプロンプトを完全に置き換えます。`CLAUDE.md` ファイルとプロジェクトメモリは、エージェントの定義が [`omitClaudeMd`](#supported-frontmatter-fields) を設定している場合でも、通常のメッセージフローを通じて読み込まれます。

エージェント名は起動ヘッダーに `@<name>` として表示されるため、アクティブであることを確認できます。

これは組み込みおよびカスタムサブエージェントで機能し、セッションを再開するときに選択が保持されます。Claude Code はエージェントのツール制限とモデルを会話とともに復元します。セッションを再開するときにエージェントが存在しなくなった場合、セッションはデフォルトツールで続行され、[エージェントに名前を付ける警告](/docs/ja/errors#session-agent-no-longer-available) が表示されます。どちらの場合のシステムプロンプトについては、[再開された会話でのシステムプロンプトフラグ](/docs/ja/cli-reference#system-prompt-flags-in-resumed-conversations) を参照してください。

プラグインが提供するサブエージェントの場合、エージェント名のみを渡すことができ、Claude Code がそれを見つけます。

```bash theme={null}
claude --agent security-reviewer
```

複数のプラグインが同じ名前のエージェントを提供する場合、スコープ付き名を渡して曖昧さを解消します。

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

プラグインがエージェントを `agents/` ディレクトリのサブフォルダに配置する場合、スコープ付き名にサブフォルダを含めます。例えば `claude --agent my-plugin:review:security` です。

プロジェクト内のすべてのセッションのデフォルトにするには、`.claude/settings.json` で `agent` を設定します。

```json theme={null}
{
  "agent": "code-reviewer"
}
```

両方が存在する場合、CLI フラグが設定をオーバーライドします。

<h3 id="run-subagents-in-foreground-or-background">
  サブエージェントをフォアグラウンドまたはバックグラウンドで実行する
</h3>

サブエージェントはフォアグラウンドまたはバックグラウンドで実行できます。

* **フォアグラウンドサブエージェント** は、完了するまでメイン会話をブロックします。権限プロンプトは発生時にあなたに渡されます。
* **バックグラウンドサブエージェント** は、作業を続行しながら同時に実行されます。バックグラウンドサブエージェントが権限が必要なツール呼び出しに達すると、Claude Code はメインセッションでプロンプトを表示し、要求しているサブエージェントに名前を付けます。承認してサブエージェントを続行させるか、Esc を押してそのツール呼び出しのみを拒否し、サブエージェントを停止しません。

Claude が Agent ツールで生成するサブエージェントごとに、Claude Code は、適用される最初のケースからフォアグラウンドまたはバックグラウンドを選択します。

* インプロセス [エージェントチーム](/docs/ja/agent-teams#limitations) チームメイトがサブエージェントを生成した場合、Claude Code はそれをフォアグラウンドで実行します。Claude Code は、定義が [`background: true`](#supported-frontmatter-fields) を設定するチームメイトのサブエージェントを生成することを拒否し、エラーを返します。[フォークモード](#turn-fork-mode-on-or-off) がオフで、[バックグラウンドタスクをオフ](/docs/ja/env-vars) にしていない場合、チームメイトが `run_in_background: true` を設定すると、Claude Code もエラーで拒否します。
* [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/ja/env-vars) を `1` に設定した場合、Claude Code はすべての種類のセッションでサブエージェントをフォアグラウンドで実行し、フォークモードがオンかどうかに関わらず実行します。
* [フォークモード](#turn-fork-mode-on-or-off) がオンの場合（インタラクティブセッションではデフォルト）、Claude Code はサブエージェントをバックグラウンドで実行し、フォークおよび非フォークサブエージェントの両方を実行し、Claude はフォアグラウンドを要求できません。
* フォークモードがオフの場合、Claude はデフォルトでサブエージェントをバックグラウンドで実行し、結果が必要な場合はフォアグラウンドで実行します。フォークモードは [非インタラクティブモード](/docs/ja/headless) で `-p` を使用する場合と、Agent SDK でオンにしない限りオフです。特定のサブエージェントを Claude が結果を必要とする場合でもバックグラウンドに保つには、そのフロントマター [`background`](#supported-frontmatter-fields) フィールドを `true` に設定します。

`context: fork` を持つスキルの場合、フォークモードがオンかどうかに関わらず、Claude Code は [サブエージェントでスキルを実行](/docs/ja/skills#run-skills-in-a-subagent) のルールに従います。

バックグラウンドサブエージェントは、会話フォークと [再開](#resume-subagents) されたフォアグラウンドサブエージェントを除き、フォアグラウンドサブエージェントより小さい [組み込みツールセット](#available-tools) で実行されます。

バックグラウンドサブエージェントはメインセッションのすべての権限プロンプトを表示します。セッションの残り期間の付与など、1 つのツール呼び出しを超えて続く選択でこれらのプロンプトの 1 つに答える場合、Claude Code はメイン会話を含むセッション全体にあなたの答えを適用します。

バックグラウンドサブエージェントは、バックグラウンド [Bash または PowerShell コマンド](/docs/ja/tools-reference#background-commands) を [ターンの終了後も実行し続ける](/docs/ja/interactive-mode#how-backgrounding-works) ことができます。そのコマンドが終了すると、Claude Code はサブエージェントに通知を送信します。

バックグラウンドサブエージェントの結果は、後のターンで完了通知として Claude に到達します。Claude は、その通知を受け取る前にサブエージェントの結果を報告し、最初に進捗について尋ねた場合、サブエージェントがまだ実行中であることを報告します。v2.1.211 より前では、Claude は完了していないバックグラウンドサブエージェントの結果を報告することがありました。

これを自分で操作することもできます。

* フォークモードがオフの場合、Claude にタスクをバックグラウンドまたはフォアグラウンドで実行するよう依頼します
* **Ctrl+B** を押して、実行中のタスクをバックグラウンドにします

Claude Code は、サブエージェントの終了方法に応じて、2 つの方法のいずれかで、プロンプト入力の下のサブエージェントパネルからバックグラウンドサブエージェントの行をクリアします。

* サブエージェントが正常に完了すると、Claude Code はその行をすぐに削除し、[スクリーンリーダーモード](/docs/ja/accessibility) を除き、フッターに「`/tasks` でサブエージェントを表示」を 30 秒間表示します。その 30 秒間に [`/tasks`](/docs/ja/commands) を実行し、サブエージェントで `Enter` を押してそのトランスクリプトを開きます。v2.1.232 より前では、Claude Code はサブエージェントが完了した後、失敗したものと同じ 30 秒間行を保持し、フッターヒントを表示しませんでした。
* サブエージェントが失敗するか、停止すると、Claude Code は 30 秒間その行を保持します。行をすぐにクリアするには、それを選択して `x` を押します。

完了したバックグラウンドサブエージェントは [`/tasks`](/docs/ja/commands) にリストされたままで、完了とマークされ、実行中の作業の下にソートされ、フッターヒントと同じ 30 秒間です。その詳細ビューはサブエージェントが完了したときに開いたままです。失敗するか停止したサブエージェントはリストを離れます。v2.1.208 より前では、完了したサブエージェントは完了した瞬間にリストを離れ、その詳細ビューが閉じられました。

<h3 id="subagent-names">
  サブエージェント名
</h3>

Claude は Agent ツール呼び出しで `name` パラメータを渡すことでサブエージェントに名前を付けることができ、最初にあなたに尋ねることなく自分で行うことができます。名前はサブエージェントをアドレス可能にします。Claude は [メッセージを送信するか、完了後に名前で再開](#resume-subagents) できます。

[エージェントチーム](/docs/ja/agent-teams) が有効なインタラクティブセッションでは、メイン会話から Claude が生成する `name` を持つサブエージェントは、呼び出しが [フォーク](#fork-the-current-conversation) であるか、呼び出しで `isolation` を渡さない限り、チームメイトとして起動します。サブエージェントのフロントマターの `isolation` 値はそれを防ぎません。チームメイトはメインセッションの作業ディレクトリで実行されます。[Claude がエージェントチームを開始する方法](/docs/ja/agent-teams#how-claude-starts-agent-teams) を参照してください。

<h3 id="api-errors-in-subagents">
  サブエージェント内の API エラー
</h3>

何かが [サブエージェントの応答をストリーム中に切断](/docs/ja/errors#the-response-above-may-be-incomplete) し、部分的な応答にテキストが含まれているがツール呼び出しがない場合、Claude Code はサブエージェントに実行を続行するよう促し、実行を終了しません。これはインタラクティブセッションでも発生します。実行は、これらの継続が使い果たされた場合にのみエラーで終了します。

v2.1.199 以降、実行が API エラー（使用制限や繰り返されるサーバーエラーなど）で終了するサブエージェントは、エラーテキストをサブエージェントの検出結果のように返すのではなく、その失敗を Claude に報告します。Claude が受け取るものは、サブエージェントが実行された場所によって異なります。

* **フォアグラウンド**: レート制限、オーバーロード、またはサーバーエラーがテキスト出力を既に生成したサブエージェントを切断した場合、Agent ツールはその部分出力を、サブエージェントが切断され、タスクを完了しなかったというメモとともに返します。何も生成しなかった、またはその唯一の出力がツール呼び出しだったサブエージェントは、[`Agent terminated early due to an API error`](/docs/ja/errors#agent-terminated-early-due-to-an-api-error) で失敗し、その後にエラーの詳細が続きます。v2.1.199 では、ツール呼び出しのみの形状を切断したレート制限、オーバーロード、またはサーバーエラーは、切断メモのみを含む空の部分結果を返しました。
* **バックグラウンド**: サブエージェントは失敗とマークされ、Claude が受け取るメッセージは終了時に API エラーに名前を付け、サブエージェントの最後の出力を含むため、部分的な作業は失われません。

[フォールバックモデルチェーン](/docs/ja/model-config#fallback-model-chains) を設定し、サブエージェントがチェーンがカバーする失敗（モデルが利用できないなど）に遭遇した場合、Claude Code はサブエージェントをリクエストを受け入れるチェーンの最初のモデルに切り替えます。サブエージェントはエラーで終了するのではなく、作業を続行します。

基盤となる API エラーがクリアされたら、Claude にタスクを再試行するか、[サブエージェントを再開](#resume-subagents) するよう依頼してください。

<h3 id="subagent-output-scanning">
  サブエージェント出力スキャン
</h3>

Claude Code は、Claude がそれを読む前に、各サブエージェントの最終レポートをスキャンします。サブエージェントはファイル、ウェブページ、またはコマンド出力を読んだ可能性があり、これらのソースからのテキストはメイン会話を対象とした指示を含むことができます。スキャンは何も削除または言い換えません。レポートで気付く可能性のある 2 種類の変更を行います。

* **バックスラッシュ挿入**: スキャンは、Claude Code 独自の出力を模倣するテキスト（`<system-reminder>` タグや `Human:` または `Assistant:` で始まる行など）にバックスラッシュを挿入し、模倣が会話の一部として誤解されるのではなく、通常のテキストとして読まれるようにします。
* **マーカー行**: スキャンは、`<system-reminder>` のようなタグを模倣するか、`bypassPermissions` や `--dangerously-skip-permissions` などの権限設定に言及するレポートの場合、`[harness: subagent output matched instruction-shaped pattern(s):` で始まる行を先頭に追加します。権限設定の言及はマーカー行を取得しますが、テキスト自体は書かれたままです。

スキャンはコンテンツが悪意があるかどうかを判断せず、レポート内の指示が何ができるかは変わりません。レポートが Claude に行わせるツール呼び出しは、セッションの [権限チェック](/docs/ja/permissions) と [サンドボックス](/docs/ja/sandboxing) を通じて実行されます。これは [サブエージェントが到達できるものを制限](#control-subagent-capabilities) する代わりではありません。

サブエージェントの結果として Claude に返されるレポートは、サブエージェント出力としてマークするヘッダーの下に到達します。ヘッダーは、レポート内の指示または承認請求がサブエージェントの言葉であり、あなたからの権限を持たないことを述べています。

[バックグラウンドサブエージェントのレポート](#run-subagents-in-foreground-or-background) は完了通知内に到達し、これはあなたからのメッセージではなく自動化されたイベントとしてマークされます。

<Note>
  サブエージェント出力スキャンには Claude Code v2.1.210 以降が必要です。
</Note>

<h3 id="common-patterns">
  一般的なパターン
</h3>

<h4 id="isolate-high-volume-operations">
  大量の操作を分離する
</h4>

サブエージェントの最も効果的な用途の 1 つは、大量の出力を生成する操作を分離することです。テストの実行、ドキュメントの取得、またはログファイルの処理は、かなりのコンテキストを消費できます。これらをサブエージェントに委譲することで、詳細な出力はサブエージェントのコンテキストに留まり、関連する概要のみがメイン会話に返されます。

```text wrap theme={null}
サブエージェントを使用してテストスイートを実行し、失敗したテストとそのエラーメッセージのみを報告してください
```

<h4 id="run-parallel-research">
  並列研究を実行する
</h4>

独立した調査の場合、複数のサブエージェントを生成して同時に作業させます。

```text wrap theme={null}
認証、データベース、および API モジュールを、別々のサブエージェントを使用して並列で調査してください
```

各サブエージェントは独立して領域を探索し、Claude は検出結果を合成します。これは研究パスが互いに依存しない場合に最適です。

<Warning>
  サブエージェントが完了すると、その結果はメイン会話に返されます。詳細な結果を返す多くのサブエージェントを実行すると、かなりのコンテキストを消費できます。
</Warning>

並列で実行し続ける必要があるか、1 つのコンテキストウィンドウに収まらない作業の場合、[別々のセッション](/docs/ja/agents) で実行し、Claude に [セッション間で検出結果を渡す](/docs/ja/cross-session-messaging) ようにします。

<h4 id="chain-subagents">
  サブエージェントをチェーンする
</h4>

マルチステップワークフローの場合、Claude にサブエージェントを順序で使用するよう依頼します。各サブエージェントはタスクを完了し、結果を Claude に返します。Claude は関連するコンテキストを次のサブエージェントに渡します。

```text wrap theme={null}
code-reviewer サブエージェントを使用してパフォーマンスの問題を見つけ、次に optimizer サブエージェントを使用してそれらを修正してください
```

<h3 id="choose-between-subagents-and-main-conversation">
  サブエージェントとメイン会話の選択
</h3>

**メイン会話** を使用する場合：

* タスクは頻繁なやり取りまたは反復的な改善が必要です
* 計画、実装、テストなど、複数のフェーズが重要なコンテキストを共有します
* 迅速でターゲットを絞った変更を行っています
* レイテンシが重要です。[フォーク](#fork-the-current-conversation) ではないサブエージェントは新規に開始し、コンテキストを収集するのに時間がかかる場合があります

**サブエージェント** を使用する場合：

* タスクはメインコンテキストで必要のない詳細な出力を生成します
* 特定のツール制限または権限を適用したいです
* 作業は自己完結型で、概要を返すことができます

メイン会話コンテキストではなく、分離されたサブエージェントコンテキストで実行される再利用可能なプロンプトまたはワークフローが必要な場合は、代わりに [スキル](/docs/ja/skills) を検討してください。

会話に既にあるものについての質問の場合、サブエージェントの代わりに [`/btw`](/docs/ja/interactive-mode#side-questions-with-%2Fbtw) を使用してください。フルコンテキストが表示されますが、ツールアクセスはなく、答えは履歴に追加されません。

<h3 id="let-subagents-spawn-their-own-subagents">
  サブエージェントが独自のサブエージェントを生成できるようにする
</h3>

デフォルトでは、サブエージェントは独自のサブエージェントを生成でき、メイン会話の下に最大 3 層です。深さの制限では、Claude Code は [フォーク](#fork-the-current-conversation) を除くすべてのサブエージェントから `Agent` ツールを保留するため、制限時のサブエージェントは委譲された作業を自分で行い、1 つの概要を返します。制限時のフォークは継承されたツールリストに `Agent` を保持しますが、ツールはエラーを返す代わりに生成します。

ネストされたサブエージェントは、委譲されたタスクが並列サブタスクに分割される場合に適しています。例えば、検出結果ごとに検証者を派遣するレビュアーサブエージェント。インタラクティブセッションでは、中間出力はメイン会話に到達しません。トップレベルのサブエージェントの概要のみがあなたに返されます。サブエージェントがバックグラウンドサブエージェントを起動する場合、それは完了する前に結果を待ちます。[非インタラクティブモード](/docs/ja/headless) と Agent SDK では、起動するサブエージェントは待たないため、ネストされたバックグラウンドサブエージェントがランチャーの終了後に完了すると、メイン会話に報告されます。

制限を変更するには、[`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/ja/env-vars) をメイン会話の下に必要なサブエージェント層の数に設定します。例えば、[`settings.json`](/docs/ja/settings) のこのエントリは、ネストを 2 層に制限します。

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

この値では、サブエージェントは 2 番目の層に委譲でき、その 2 番目の層はさらに委譲できません。ネストをオフにするには `1` を設定します。

ネストされたサブエージェントはトップレベルのサブエージェントと同じ方法で設定され、同じ [スコープ](#choose-the-subagent-scope) から解決されます。読み取り専用のままにするレビュアーなど、1 つのサブエージェントが生成されないようにするには、その [`tools`](#available-tools) リストから `Agent` を省略するか、`disallowedTools` に追加します。

Claude Code は、プロンプト入力の下のサブエージェントパネルにネストされたサブエージェントをツリーとして表示し、パネル内に子孫がまだいる各行を `(+N)` 個数でマークします。行を開いて、そのサブエージェントの兄弟と直接の子を `main` へのパスとともに表示します。

<Note>
  以前のバージョンは異なるデフォルトを使用していました。

  * **v2.1.172 から v2.1.216**: サブエージェントはデフォルトでネストでき、最大 5 層深く、制限は変更できませんでした。
  * **v2.1.217 から v2.1.218**: 制限はデフォルトで 1 でしたので、サブエージェントは上げない限り独自に生成できませんでした。v2.1.219 はデフォルトを 3 に上げました。
</Note>

<h3 id="concurrent-subagent-limit">
  同時実行サブエージェント制限
</h3>

2 つの制限がサブエージェント使用を制御し、それぞれ独自の変数があります。これは Claude が多くのサブエージェントが実行されている間に、より多くのサブエージェントを生成するのを停止し、[深さ制限](#let-subagents-spawn-their-own-subagents) はサブエージェントがどの程度深くネストするかを制限します。セッション全体で Claude が生成できるサブエージェントの総数に制限はありません。

デフォルトでは、セッションで 20 個のサブエージェントが実行されている場合、Agent ツールで別のサブエージェントを生成しようとすると `Concurrent subagent limit reached` で失敗し、エラーは Claude に再試行しないよう指示します。実行中の数が制限を下回ると、生成が再び成功します。制限を変更するには、[`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/ja/env-vars) を任意の正の整数に設定します。[ultracode](/docs/ja/model-config#adjust-effort-level) がアクティブなセッションは除外されます。制限はそこで適用されません。Claude Code v2.1.217 以降が必要です。

制限は Claude が Agent ツールで生成するサブエージェントのみをブロックしますが、他の実行は同じスロットを占有します。

* [`/subtask`](#fork-the-current-conversation) で開始するインセッションフォークは、実行中にスロットを取得し、制限によってブロックされることはありません。
* [再開](#resume-subagents) 済みのサブエージェントは、既に完了したスロットを取得し、制限をチェックせずに新しいスロットを取得するため、再開は実行中の数を制限を超えて押すことができます。

[ワークフロー](/docs/ja/workflows) エージェントや [エージェントチーム](/docs/ja/agent-teams) チームメイトなど、他の機能が実行するエージェントは、代わりに独自の制限に従います。

<h3 id="manage-subagent-context">
  サブエージェントコンテキストを管理する
</h3>

<h4 id="what-loads-at-startup">
  起動時に読み込まれるもの
</h4>

各サブエージェントは新しい、分離されたコンテキストウィンドウで開始されます。会話履歴、既に呼び出したスキル、または Claude が既に読んだファイルは表示されません。Claude はタスクを要約する委譲メッセージを作成し、サブエージェントはそこから作業します。例外は [フォーク](#fork-the-current-conversation) で、親会話を継承し、新規に開始しません。

非フォークサブエージェントの初期コンテキストには以下が含まれます。

* **システムプロンプト**: エージェント独自のプロンプトと Claude Code が追加する環境詳細。Claude Code システムプロンプトではありません。カスタムサブエージェントは [マークダウン本体](#write-subagent-files) または `prompt` フィールドで定義します。組み込みエージェントは事前定義されたプロンプトを持ちます。
* **タスクメッセージ**: Claude が作業を引き継ぐときに作成する委譲プロンプト。
* **CLAUDE.md ファイル**: メイン会話が読み込む [CLAUDE.md 階層](/docs/ja/memory#how-claude-md-files-load) のすべてのレベル。`~/.claude/CLAUDE.md`、プロジェクトルール、`CLAUDE.local.md`、管理ポリシーファイル、および [AGENTS.md ファイル](/docs/ja/memory#agents-md) を含みます。組み込みの Explore および Plan エージェントはこれをスキップします。定義が [`omitClaudeMd`](#supported-frontmatter-fields) を設定するサブエージェントは、管理ポリシーファイルのみを読み込むか、定義が [管理設定](#choose-the-subagent-scope) から来る場合は何も読み込みません。
* **Git ステータス**: 親セッションの開始時に取得されたスナップショット。作業ディレクトリが Git リポジトリでない場合、または [`includeGitInstructions`](/docs/ja/settings-reference#includegitinstructions) が `false` の場合は不在です。Explore および Plan はそれをスキップします。
* **事前読み込みスキル**: エージェントの [`skills` フィールド](#preload-skills-into-subagents) に名前が付いているスキルの完全なコンテンツ。組み込みエージェントはスキルを事前読み込みしません。
* **兄弟名簿**: `main` とセッション内のすべての他の名前付きエージェントをリストするシステムリマインダー。各エージェントは [`SendMessage`](#resume-subagents) の有効な `to` 値です。Claude Code v2.1.206 以降が必要です。名簿は、サブエージェントのツールに `SendMessage` が含まれ、少なくとも 1 つの他のエージェントに名前がある場合にのみ表示されます。Claude が生成時に名前を付けたか、[エージェントチーム](/docs/ja/agent-teams) チームメイトとして実行されるかに関わらず。これはサブエージェントが開始するときに取得されたスナップショットであるため、後で名前が付けられたエージェントは表示されません。

ユーザー、プロジェクト、およびローカル CLAUDE.md ファイルなしで独自のサブエージェントの 1 つを起動するには、フロントマターで [`omitClaudeMd: true`](#supported-frontmatter-fields) を設定するか、`--agents` JSON を設定します。

メイン会話は、これらのサブエージェントの結果を読むときに完全な CLAUDE.md を持っているため、ほとんどのルールはサブエージェント自体に到達する必要はありません。ルールが必要な場合（「`vendor/` ディレクトリを無視する」など）、委譲時に Claude に与えるプロンプトで再度述べてください。

サブエージェントが git ステータスを受け取るかどうかは変更できません。Explore および Plan のみがそれをスキップします。

いくつかのメイン会話状態は非フォークサブエージェントに到達しません。

* **出力スタイル**: サブエージェントは独自のシステムプロンプトを実行するため、[出力スタイル](/docs/ja/output-styles) はその応答を形成しません。[フォーク](#fork-the-current-conversation) を除きます。
* **自動メモリ**: メイン会話の [自動メモリ](/docs/ja/memory#auto-memory) は読み込まれません。サブエージェントに独自の永続的なメモリを与えるには、[`memory` フィールド](#enable-persistent-memory) を使用します。
* **コンテキストウィンドウサイズ**: サブエージェントのコンテキストウィンドウは、親のモデルではなく、独自のモデルによってサイズ設定されます。より小さいウィンドウを持つモデルに委譲すると、そのサブエージェントはより小さいウィンドウを取得します。

<h4 id="resume-subagents">
  サブエージェントを再開する
</h4>

各サブエージェント呼び出しは、以前のものを続行するのではなく、新しいインスタンスを作成します。既存のサブエージェントの作業を続行するのではなく、新規に開始するには、Claude にそれを再開するよう依頼します。

再開されたサブエージェントは、すべての前のツール呼び出し、結果、および推論を含む、完全な会話履歴を保持します。サブエージェントが [独自のバックグラウンドサブエージェント](#let-subagents-spawn-their-own-subagents) を生成した場合、その履歴には、実行中に配信された結果が含まれます。サブエージェントは新規に開始するのではなく、停止した場所から正確に再開されます。

* サブエージェントが完了すると、Claude はそのエージェント ID を受け取ります。
* 組み込みの Explore および Plan エージェントは 1 回限りで、エージェント ID を返さないため、Claude はそれらを再開できません。作業を続行する必要がある場合は、`general-purpose` またはカスタムサブエージェントを使用してください。
* サブエージェントが [`maxTurns`](#supported-frontmatter-fields) 制限で停止すると、Claude Code は返された出力を部分的としてマークします。エージェント ID を返すサブエージェントの場合、Claude Code は、Claude がサブエージェントにメッセージを送信して停止した場所から続行できることを結果に記載します。

Claude は `SendMessage` ツールを使用し、エージェントの ID または名前を `to` フィールドとして使用して、それを再開します。`SendMessage` は [エージェントチーム](/docs/ja/agent-teams) が有効になっている必要はありません。`shutdown_request` や `plan_approval_response` などの構造化されたチームプロトコルメッセージのみが必要です。サブエージェントとチームメイトを超えて、クロスセッションメッセージングが有効なセッションでは、Claude は同じツールを使用して [他の Claude Code セッション](/docs/ja/cross-session-messaging) にメッセージを送信でき、このマシンまたは [それを超えて](/docs/ja/cross-session-messaging#message-sessions-on-other-machines) です。

サブエージェントを再開するには、Claude に前の作業を続行するよう依頼してください。

```text wrap theme={null}
code-reviewer サブエージェントを使用して認証モジュールをレビューしてください
[エージェントが完了します]

そのコードレビューを続行し、今度は認可ロジックを分析してください
[Claude は前の会話からのフルコンテキストでサブエージェントを再開します]
```

Claude が完了したサブエージェントに `SendMessage` ツールでメッセージを送信すると、サブエージェントは新しい `Agent` 呼び出しなしでバックグラウンドで再開されます。同じことが、Claude が `TaskStop` ツールで停止したサブエージェントに適用され、停止した実行が終了した後です。再開された実行は、サブエージェントが最初に実行された場所から [ツールセット](#run-subagents-in-foreground-or-background) を保持し、元の実行が温めた [プロンプトキャッシュ](/docs/ja/prompt-caching#subagents-and-the-cache) を読み続けることができます。

`SendMessage` ツールを持つサブエージェントはそのメッセージも送信できます。インタラクティブセッションでは、再開されたエージェントはメインセッションではなく、それを再開したサブエージェントに報告します。そのサブエージェントは、独自の作業を完了する前に結果を待ちます。サブエージェントが、独自のランチャーなど、報告するエージェントにメッセージを送信する場合、Claude Code はそのエージェントを結果をリダイレクトせずに再開します。

自分で停止したサブエージェント（`/tasks` の `x` または SDK `stop_task` リクエスト）は自動的に再開されません。Claude がメッセージを送信する場合、メッセージは拒否され、Claude はエージェントがキャンセルされたことが通知されます。

[そのサブエージェントの行がサブエージェントパネルにまだある](#run-subagents-in-foreground-or-background) 間、そのトランスクリプトに入力して、自分で再開します。その後、Claude からのメッセージは再び自動的に再開できます。

再開は同じ ID の下でエージェントの新しい実行を開始するため、既に失敗または完了したサブエージェントはタスクリストと Agent SDK のタスクイベントで再び実行中として表示されます。v2.1.205 より前では、再開された実行が機能している間、以前の失敗または完了ステータスを表示し続けました。

v2.1.199 以降、`SendMessage` は名前が会話の前半で到達したのと同じエージェントを参照していることを確認します。新しいエージェントが名前を取得した場合（例えば、名前を再利用した再生成されたバックグラウンドエージェント）、Claude Code は送信を拒否し、エラーは名前が現在到達するエージェントを報告するため、Claude は再ターゲットできます。以前のエージェントにアクセスするには、まだ実行中の場合、Claude がそのエージェントを生成したときに受け取ったエージェント ID でアドレスします。チェックは現在の会話にスコープされ、`/clear` でリセットされます。

v2.1.198 以降、サブエージェントは、それを起動したエージェントからのメッセージを通常のタスク方向として扱い、タスク中のコース修正を含め、独自の権限設定内で機能します。2 つの制限は、メッセージを送信したエージェントに関わらず保持されます。エージェントメッセージは、保留中の権限プロンプトの承認としてカウントされず、エージェントメッセージはサブエージェントの権限設定、`CLAUDE.md`、または設定を変更できません。権限システムまたは独自のメッセージのみが承認を付与できます。

エージェント ID を明示的に参照したい場合は Claude に要求することもできます。または、`~/.claude/projects/{project}/{sessionId}/subagents/` のトランスクリプトファイルで ID を見つけます。各トランスクリプトは `agent-{agentId}.jsonl` として保存されます。

サブエージェントトランスクリプトはメイン会話とは独立に永続化されます。

* **メイン会話圧縮**: メイン会話が圧縮されると、サブエージェントトランスクリプトは影響を受けません。別々のファイルに保存されます。
* **セッション永続化**: サブエージェントトランスクリプトはセッション内で永続化されます。Claude Code を再起動して同じセッションを再開することで、[サブエージェントを再開](#resume-subagents) できます。
* **自動クリーンアップ**: Claude Code は、`cleanupPeriodDays` 保持期間（デフォルトは 30 日）の後、サブエージェントトランスクリプトを削除し、[保持スイープルール](/docs/ja/claude-directory#cleaned-up-automatically) に従います。

<h4 id="auto-compaction">
  自動圧縮
</h4>

サブエージェントは、メイン会話と同じロジックを使用して自動圧縮をサポートします。圧縮は同じ条件下でトリガーされ、`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` はサブエージェントにも適用されます。環境変数がいつ有効になるかについては、[環境変数](/docs/ja/env-vars) を参照してください。

圧縮イベントはサブエージェントトランスクリプトファイルに記録されます。

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

`preTokens` 値は、圧縮が発生する前に使用されたトークン数を示します。

<h2 id="fork-the-current-conversation">
  現在の会話をフォークする
</h2>

<Note>
  フォークされたサブエージェントは `/subtask` で実行され、Claude Code v2.1.212 以降が必要です。[エージェントビューがオフになっている](/docs/ja/agent-view#turn-off-agent-view)場合、`/subtask` は利用できず、`/fork` がフォークされたサブエージェントを開始します。それ以外の場合、`/fork` はセッション全体を新しい[バックグラウンドセッション](/docs/ja/agent-view#from-inside-a-session)にコピーします。
</Note>

フォークは、これまでの会話全体を継承するサブエージェントです。これにより、サブエージェントが通常提供する入力分離が削除されます。フォークはメインセッションと同じシステムプロンプト、ツール、モデル、およびメッセージ履歴を表示するため、状況を再度説明することなく、サイドタスクを渡すことができます。フォークのツール呼び出しはまだ会話から除外され、最終結果のみが返されるため、メインコンテキストウィンドウはクリーンなままです。フォークを使用する場合は、他のサブエージェントが有用であるには背景が多すぎる場合、または同じ開始点から複数のアプローチを並行して試したい場合です。

Claude は Agent ツールを通じて `fork` サブエージェントタイプをリクエストすることでフォークを開始します。これを制御するのは[フォークモード](#turn-fork-mode-on-or-off)で、インタラクティブセッションではデフォルトでオンになっています。

`/subtask` の後にタスクを続けることで、フォークモードがオンかどうかに関わらず、自分でフォークを開始できます。v2.1.161 から v2.1.211 ではコマンドは `/fork` です。Claude Code はフォークにタスクの最初の単語から名前を付けます。次の例は、メインセッションで実装を続ける間に、会話をドラフトテストケースにフォークします：

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

フォークはプロンプト入力の下のパネルに表示され、作業を続ける間にバックグラウンドで実行されます。完了すると、その結果がメイン会話にメッセージとして到着します。次のセクションでは、実行中のフォークを監視して操作するためのパネルコントロールについて説明します。

<h3 id="observe-and-steer-running-forks">
  実行中のフォークを観察して操作する
</h3>

実行中のフォークはプロンプト入力の下のパネルに表示され、メインセッション用に 1 行、各フォーク用に 1 行があります。

フォークが正常に完了すると、Claude Code はその行を削除します。Claude Code は失敗したフォークまたは停止したフォークの行を 30 秒間保持します。これは[他のバックグラウンドサブエージェントと同じ](#run-subagents-in-foreground-or-background)です。v2.1.232 より前では、Claude Code は完了したフォークの行も 30 秒間保持していました。

これらのキーを使用してパネルと対話します：

| キー        | アクション                                                                                                  |
| :-------- | :----------------------------------------------------------------------------------------------------- |
| `↑` / `↓` | 行間を移動                                                                                                  |
| `Enter`   | 選択したフォークのトランスクリプトを開き、フォローアップメッセージを送信                                                                   |
| `x`       | 実行中の場合は選択したフォークを停止するか、実行中でなくなった場合はその行を閉じます。メインセッション行またはトランスクリプトを `Enter` で開いたフォークの行では、`x` はプロンプトに入力します |
| `Esc`     | フォーカスをプロンプト入力に戻す                                                                                       |

フォークまたはサブエージェントのトランスクリプトが開いている場合、フォローアップメッセージと[スキル](/docs/ja/skills)はそのエージェントに送信されますが、組み込みコマンドはメイン会話で実行されたままです。v2.1.199 以降では、そのビューで `/model` または `/fast` を入力すると、表示されているエージェントのモデルまたはファストモードではなく、メイン会話のモデルまたはファストモードを変更することを示す通知が表示されます。サイレントに実行される代わりに。

<h3 id="how-forks-differ-from-other-subagents">
  フォークと他のサブエージェントの違い
</h3>

フォークはメインセッションがその時点で持っているすべてを継承します。他のサブエージェントはその定義から開始します。

|               | フォーク           | 非フォークサブエージェント                                                                      |
| :------------ | :------------- | :--------------------------------------------------------------------------------- |
| コンテキスト        | 完全な会話履歴        | 渡すプロンプトを使用した新しいコンテキスト                                                              |
| システムプロンプトとツール | メインセッションと同じ    | サブエージェントの[定義ファイル](#write-subagent-files)から、[バックグラウンド実行用にフィルタリング](#available-tools) |
| モデル           | メインセッションと同じ    | サブエージェントの `model` フィールドから                                                          |
| 権限            | プロンプトがターミナルに表示 | [バックグラウンド実行時にメインセッションに表示](#run-subagents-in-foreground-or-background)              |
| プロンプトキャッシュ    | メインセッションと共有    | 別のキャッシュ                                                                            |

フォークのシステムプロンプトとツール定義は親と同じであるため、最初のリクエストは親の[プロンプトキャッシュ](/docs/ja/prompt-caching#subagents-and-the-cache)を再利用します。これにより、同じコンテキストが必要なタスクの場合、フォークは新しいサブエージェントをスポーンするよりも安価です。

Claude が Agent ツール経由でフォークをスポーンするときに、`isolation: "worktree"` を渡すことができるため、フォークのファイル編集は、チェックアウトではなく、別の git worktree に書き込まれます。フォークはさらにフォークをスポーンできません。

<h3 id="turn-fork-mode-on-or-off">
  フォークモードをオンまたはオフにする
</h3>

Claude Code はインタラクティブセッションではフォークモードをデフォルトでオンにし、[非インタラクティブモード](/docs/ja/headless)（`-p` 付き）および Agent SDK ではデフォルトでオフにします。インタラクティブデフォルトには Claude Code v2.1.232 以降が必要です。それより前のバージョンでは、`CLAUDE_CODE_FORK_SUBAGENT` を `1` に設定してフォークモードをオンにします。

フォークモードがオンであることは、Claude Code が Agent ツールを処理する方法からわかります：

* Claude は `fork` サブエージェントタイプをリクエストすることでフォークをスポーンできます。Claude がタイプをリクエストしない場合、セッションがまだそのタイプを持っていれば[汎用](#built-in-subagents)サブエージェントを取得します。Explore などの定義から生成されたサブエージェントは通常通り機能します。
* Claude Code は Claude がスポーンするサブエージェント（フォークと非フォークサブエージェント両方）をバックグラウンドで実行します。ただし[フォアグラウンドに留まるケース](#run-subagents-in-foreground-or-background)は除きます。Claude Code は Agent ツールの `run_in_background` パラメータも削除するため、Claude はフォアグラウンドをリクエストできません。

[`CLAUDE_CODE_FORK_SUBAGENT`](/docs/ja/env-vars)環境変数を設定してデフォルトをオーバーライドします：

* `1` は非インタラクティブモードおよび Agent SDK でもフォークモードをオンにします
* `0` はすべての種類のセッションでフォークモードをオフにします

フォークモードをオンに保ちながら Claude がフォークをスポーンするのを停止するには、`Agent(fork)` ルールで[`fork` サブエージェントタイプを拒否](#disable-specific-subagents)します。Claude Code は Claude がスポーンするサブエージェントをバックグラウンドで実行し続けます。ただし同じ[フォアグラウンドに留まるケース](#run-subagents-in-foreground-or-background)は除きます。

<h2 id="example-subagents">
  サブエージェントの例
</h2>

これらの例は、サブエージェントを構築するための効果的なパターンを示しています。出発点として使用するか、Claude を使用してカスタマイズされたバージョンを生成します。

<Tip>
  **ベストプラクティス：**

  * **焦点を絞ったサブエージェントを設計する：** 各サブエージェントは 1 つの特定のタスクに優れている必要があります
  * **各サブエージェントを単独で指定する説明を書く：** Claude は説明を使用して委譲するかどうかを決定します。各説明は正しいサブエージェントにルーティングするのに十分な具体性を持たせ、組み合わせたセット全体を[15,000 トークン説明予算](#understand-automatic-delegation)内に保ちます
  * **ツールアクセスを制限する：** セキュリティと焦点のために必要な権限のみを付与します
  * **バージョン管理にチェックインする：** プロジェクトサブエージェントをチームと共有します
</Tip>

<h3 id="code-reviewer">
  コードレビュアー
</h3>

コードを変更せずにレビューする読み取り専用サブエージェント。この例は、制限されたツールアクセス（Edit と Write を除外）と、何を探すべきか、出力をどのようにフォーマットするかを正確に指定する詳細なプロンプトを使用して、焦点を絞ったサブエージェントを設計する方法を示しています。

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  デバッガー
</h3>

問題を分析して修正できるサブエージェント。コードレビュアーとは異なり、このサブエージェントはバグの修正にはコード変更が必要なため、Edit を含みます。プロンプトは診断から検証までの明確なワークフローを提供します。

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  データサイエンティスト
</h3>

データ分析作業向けのドメイン固有のサブエージェント。この例は、典型的なコーディングタスク以外の特化したワークフロー向けのサブエージェントを作成する方法を示しています。より有能な分析のために `model: sonnet` を明示的に設定します。

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  データベースクエリバリデーター
</h3>

Bash アクセスを許可しますが、読み取り専用 SQL クエリのみを許可するようにコマンドを検証するサブエージェント。この例は、`tools` フィールドが提供するよりも細かい制御が必要な場合に、`PreToolUse` hooks を使用して条件付き検証を行う方法を示しています。

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code は[hook 入力を JSON として](/docs/ja/hooks#pretooluse-input)stdin を通じて hook コマンドに渡します。検証スクリプトはこの JSON を読み取り、実行されるコマンドを抽出し、SQL 書き込み操作のリストに対してチェックします。書き込み操作が検出された場合、スクリプトは[終了コード 2](/docs/ja/hooks#exit-code-2-behavior-per-event)で終了して、stderr を通じて Claude にエラーメッセージを返します。

プロジェクト内の任意の場所に検証スクリプトを作成します。パスは hook 設定の `command` フィールドと一致する必要があります：

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

macOS と Linux では、スクリプトを実行可能にします：

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Windows では、検証スクリプトを PowerShell で記述し、hook エントリに `shell: powershell` を追加します。[PowerShell で hooks を実行する](/docs/ja/hooks#windows-powershell-tool)を参照してください。

hook は stdin を通じて JSON を受け取り、Bash コマンドは `tool_input.command` にあります。終了コード 2 は操作をブロックし、エラーメッセージを Claude にフィードバックします。終了コードと出力の詳細については[Hooks](/docs/ja/hooks#exit-code-output)を参照し、完全な入力スキーマについては[Hook 入力](/docs/ja/hooks#pretooluse-input)を参照してください。

システムプロンプトはサブエージェントに書き込みリクエストを拒否するよう指示するため、hook はバックストップです。サブエージェントが書き込みを試みた場合、Claude Code はコマンドをブロックし、サブエージェントは `Blocked: Write operations not allowed. Use SELECT queries only.` メッセージを表示します。

<h2 id="next-steps">
  次のステップ
</h2>

サブエージェントを理解したので、これらの関連機能を探索してください：

* [プラグインでサブエージェントを配布する](/docs/ja/plugins/components#agents)ことで、チームまたはプロジェクト全体でサブエージェントを共有します
* [Claude Code をプログラムで実行する](/docs/ja/headless)ことで、Agent SDK を使用して CI/CD と自動化を行います
* [MCP サーバーを使用する](/docs/ja/mcp)ことで、サブエージェントに外部ツールとデータへのアクセスを提供します
