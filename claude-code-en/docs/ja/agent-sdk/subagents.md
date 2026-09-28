> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK のサブエージェント

> コンテキストを分離し、タスクを並列実行し、Claude Agent SDK アプリケーションで特化した指示を適用するサブエージェントを定義および呼び出します。

サブエージェントは、メインエージェントが焦点を絞ったサブタスクを処理するために生成できる個別のエージェントインスタンスです。
これらを使用して、コンテキストを分離し、複数の分析を並列実行し、メインエージェントのプロンプトに追加することなく特化した指示を適用できます。

<h2 id="overview">
  概要
</h2>

サブエージェントは 3 つの方法で作成できます。

* **プログラム的に**: `query()` オプションの `agents` パラメータを使用します。[TypeScript](/docs/ja/agent-sdk/typescript#agentdefinition) および [Python](/docs/ja/agent-sdk/python#agentdefinition) リファレンスを参照してください
* **ファイルシステムベース**: `.claude/agents/` ディレクトリ内のマークダウンファイルとしてエージェントを定義します。[ファイルとしてのサブエージェント定義](/docs/ja/sub-agents)を参照してください
* **組み込み汎用**: Claude は、何も定義することなく、Agent ツール経由でいつでも組み込みの `general-purpose` サブエージェントを呼び出すことができます

このガイドはプログラム的なアプローチに焦点を当てており、SDK アプリケーションに推奨されます。

<h2 id="benefits-of-using-subagents">
  サブエージェントを使用する利点
</h2>

サブエージェントは個別のエージェントインスタンスであるため、サブエージェントに作業を委譲することで、4 つの利点が得られます。

* **コンテキスト分離**: 各サブエージェントは独自の会話内で実行され、サブエージェントが[現在の会話をフォーク](/docs/ja/sub-agents#fork-the-current-conversation)する場合を除き、新たに開始されます。いずれの場合でも、中間的なツール呼び出しと結果はサブエージェント内に留まり、最終メッセージのみが親に返されます。`research-assistant` サブエージェントは、そのコンテンツが主要な会話に蓄積されることなく、数十のファイルを探索できます。親は、サブエージェントが読んだすべてのファイルではなく、簡潔なサマリーを受け取ります。サブエージェントのコンテキストに含まれるものについては、[サブエージェントが継承するもの](#what-subagents-inherit)を参照してください。
* **並列化**: 複数のサブエージェントを同時に実行できるため、独立したサブタスクは、すべてのタスクの合計ではなく、最も遅いものの時間で完了します。コードレビュー中に、`style-checker`、`security-scanner`、`test-coverage` サブエージェントを順序立てて実行するのではなく、同時に実行できます。
* **特化した指示とナレッジ**: 各サブエージェントは、特定の専門知識、ベストプラクティス、制約を備えたカスタマイズされたシステムプロンプトを持つことができます。`database-migration` サブエージェントは、SQL ベストプラクティス、ロールバック戦略、データ整合性チェックに関する詳細なナレッジを持つことができます。これらは主要なエージェントの指示では不要なノイズになります。
* **ツール制限**: サブエージェントは特定のツールに限定でき、意図しないアクションのリスクを軽減します。`doc-reviewer` サブエージェントは Read と Grep ツールのみにアクセスでき、ドキュメンテーションファイルを分析できますが、誤って変更することはありません。

<h2 id="create-subagents">
  サブエージェントを作成する
</h2>

<h3 id="programmatic-definition-recommended">
  プログラマティック定義（推奨）
</h3>

`agents` パラメータを使用してコード内でサブエージェントを直接定義します。Claude は `Agent` ツールを通じてサブエージェントを呼び出します。

このページのほとんどの例は最終結果のみを出力します。Claude がサブエージェントに委譲したのか、それとも直接回答したのかを確認するには、[サブエージェント呼び出しの検出](#detect-subagent-invocation)を参照してください。

この例では、読み取り専用アクセス権を持つコードレビュアーと、コマンドを実行できるテストランナーの 2 つのサブエージェントを作成します。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  AgentDefinition 設定
</h3>

| フィールド             | 型                                                           | 必須  | 説明                                                                                                                                                                                                                                                                        |
| :---------------- | :---------------------------------------------------------- | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | `string`                                                    | はい  | このエージェントをいつ使用するかを説明する自然言語の説明                                                                                                                                                                                                                                              |
| `prompt`          | `string`                                                    | はい  | エージェントの役割と動作を定義するシステムプロンプト                                                                                                                                                                                                                                                |
| `tools`           | `string[]`                                                  | いいえ | 許可されたツール名の配列。省略した場合、[サブエージェントで利用可能なすべてのツール](/docs/ja/sub-agents#available-tools)を継承します                                                                                                                                                                                         |
| `disallowedTools` | `string[]`                                                  | いいえ | エージェントのツールセットから削除するツール名の配列。MCP サーバーレベルのパターンも受け入れられます。`mcp__server` または `mcp__server__*` はそのサーバーのすべてのツールを削除し、`mcp__*` はすべての MCP ツールをすべてのサーバーから削除します                                                                                                                        |
| `model`           | `string`                                                    | いいえ | このエージェントのモデルオーバーライド。`'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'` などのエイリアスまたは完全なモデル ID を受け入れます。`'inherit'` はメインモデルを使用します。省略した場合、Claude Code は[サブエージェントモデルの順序](/docs/ja/sub-agents#choose-a-model)でモデルを選択します                                                              |
| `skills`          | `string[]`                                                  | いいえ | 起動時にエージェントのコンテキストにプリロードするスキル名のリスト。リストされていないスキルは Skill ツールを通じて呼び出し可能なままです                                                                                                                                                                                                  |
| `memory`          | `'user' \| 'project' \| 'local'`                            | いいえ | このエージェントのメモリソース                                                                                                                                                                                                                                                           |
| `mcpServers`      | `(string \| object)[]`                                      | いいえ | このエージェントで利用可能な MCP サーバー（名前またはインライン設定）                                                                                                                                                                                                                                     |
| `initialPrompt`   | `string`                                                    | いいえ | このエージェントがメインスレッドエージェントとして実行される場合、最初のユーザーターンとして自動送信されます。エージェントがサブエージェントとして呼び出される場合は無視されます                                                                                                                                                                                  |
| `maxTurns`        | `number`                                                    | いいえ | エージェントが停止する前の最大 agentic ターン数。エージェントが制限に達すると、Claude Code は出力を部分的としてマークして返し、[エージェントを再開](#resume-subagents)して続行できます。部分的なマーキングには Claude Code v2.1.246 以降が必要です                                                                                                                 |
| `background`      | `boolean`                                                   | いいえ | 呼び出されたときにこのエージェントをノンブロッキングバックグラウンドタスクとして実行します                                                                                                                                                                                                                             |
| `omitClaudeMd`    | `boolean`                                                   | いいえ | このエージェントがサブエージェントとして実行される場合、ユーザー、プロジェクト、およびローカル CLAUDE.md ファイルなしでこのエージェントを実行します。管理ポリシーファイルは引き続き読み込まれます。エージェントがメインスレッドエージェントとして実行される場合は無視されます。TypeScript Agent SDK v0.3.271 以降が必要です。Python SDK の [`AgentDefinition`](/docs/ja/agent-sdk/python#agentdefinition) にはこのフィールドがありません |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | いいえ | このエージェントの推論努力レベル                                                                                                                                                                                                                                                          |
| `permissionMode`  | `PermissionMode`                                            | いいえ | このエージェント内のツール実行の権限モード。[サブエージェント継承ルール](/docs/ja/agent-sdk/permissions#available-modes)は、それが適用される場合を決定します                                                                                                                                                                        |

Python SDK では、`disallowedTools` や `mcpServers` などの複数単語のフィールド名は、Python の snake\_case 規約に従うのではなく、ワイヤ形式に一致させるために camelCase スペルを保持します。詳細については、[`AgentDefinition` リファレンス](/docs/ja/agent-sdk/python#agentdefinition)を参照してください。

サブエージェントはデフォルトでバックグラウンドで実行されます。[`run_in_background`](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) 入力を省略する Agent ツール呼び出しはバックグラウンドサブエージェントを起動し、Claude が続行する前に結果が必要な場合は `run_in_background: false` を設定します。`background` フィールドを `true` に設定して、Claude が要求する内容に関係なく、特定のエージェントに対してバックグラウンド実行を強制します。Claude Code v2.1.198 より前は、バックグラウンドデフォルトは段階的にロールアウトされており、`run_in_background` を省略する Agent ツール呼び出しはサブエージェントを同期的に実行できました。

サブエージェントは独自のサブエージェントを生成することもできます。そのネストがどの程度深くなるか、一度に何個のサブエージェントが実行されるか、クエリがいくら費やすかを制限するには、[サブエージェントの深さ、同時実行性、および支出を制限する](#cap-subagent-depth-concurrency-and-spend)を参照してください。

<h3 id="filesystem-based-definition-alternative">
  ファイルシステムベースの定義（代替案）
</h3>

`.claude/agents/` ディレクトリ内のマークダウンファイルとしてサブエージェントを定義することもできます。このアプローチの詳細については、[Claude Code サブエージェントドキュメント](/docs/ja/sub-agents)を参照してください。プログラマティックに定義されたエージェントは、同じ名前のファイルシステムベースのエージェントより優先されます。

<Note>
  Claude が `subagent_type` なしで Agent ツールを呼び出すと、組み込みの `general-purpose` サブエージェントが取得されます。これは、独自のエージェントを定義しない場合でも Claude が生成できます。[`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ja/env-vars)を設定するとそのデフォルトが削除され、そのような呼び出しは [`subagent_type is required`](/docs/ja/errors#subagent-type-is-required) で失敗します。
</Note>

<h2 id="what-subagents-inherit">
  サブエージェントが継承するもの
</h2>

サブエージェントが[フォーク](/docs/ja/sub-agents#fork-the-current-conversation)でない限り、そのコンテキストウィンドウは新しく開始され、親の会話がありませんが、空ではありません。親からサブエージェントに渡す唯一のコンテンツは Agent ツールのプロンプト文字列であるため、サブエージェントが必要とするファイルパス、エラーメッセージ、または決定をそのプロンプトに直接含めてください。

[`SendMessage`](/docs/ja/tools-reference) ツールを持つサブエージェントは、セッションで実行されている他の名前付きエージェントのリストで開始されるため、メッセージを送信できる名前を認識しています。Claude Code はこのリストをサブエージェントの最初のターンに自動的に追加します。[フォーク](/docs/ja/sub-agents#fork-the-current-conversation)は親の会話を継承するため、リストを取得しません。

サブエージェントはメインセッションの拡張思考設定も継承します。

以下の表は、フォークではないサブエージェントのコンテキストに含まれるもの、および含まれないものを示しています。

| サブエージェントが受け取るもの                                                                                                                                                                                    | サブエージェントが受け取らないもの                                         |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------- |
| 独自のシステムプロンプト（`AgentDefinition.prompt`）と Agent ツールのプロンプト                                                                                                                                            | 親の会話履歴またはツール結果                                            |
| Project CLAUDE.md（[`settingSources`](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) 経由でロード）（[`omitClaudeMd`](#agentdefinition-configuration) をエージェントが設定しない限り） | プリロードされたスキルコンテンツ（`AgentDefinition.skills` にリストされている場合を除く） |
| ツール定義（親から継承、または `tools` のサブセット、[バックグラウンド実行用にフィルタリング](/docs/ja/sub-agents#available-tools)）                                                                                                              | 親のシステムプロンプト                                               |

<Note>
  親はサブエージェントの最終メッセージを Agent ツール結果として受け取りますが、独自の応答でそれを要約する場合があります。サブエージェント出力をユーザー向けの応答に逐語的に保持するには、メインの `query()` 呼び出しに渡すプロンプトまたは `systemPrompt` オプションに実行するよう指示を含めてください。

  v2.1.210 以降では、Claude Code は親がそれを読む前に[最終メッセージを指示形パターンについてスキャン](/docs/ja/sub-agents#subagent-output-scanning)します。スキャンは 3 種類のパターンを異なる方法で処理します。

  * **制御タグの模倣**: Claude Code は、`<system-reminder>` ブロックなど、ハーネスのみが発行するタグを所定の位置で中立化します。開き角括弧の後にバックスラッシュを挿入し、何も削除しません。
  * **権限設定の言及**: Claude Code は、`.claude/settings.json`、`bypassPermissions`、または `--dangerously-skip-permissions` などの権限設定への参照を記述されたとおりに保持します。
  * **ターンマーカー**: `Human:` または `Assistant:` で始まる行は、メッセージが会話ターン境界を模倣できないように、コロンの前にバックスラッシュが付きます。

  制御タグまたは権限設定の一致の場合、Claude Code は一致したパターンに名前を付ける `[harness: ...]` マーカー行を先頭に付けます。ターンマーカーの一致は、マーカー行を追加しません。これらはスキャンが行う唯一の変更です。サブエージェントのテキストを削除または言い換えることはありません。
</Note>

レート制限などでサブエージェントを早期に終了させる API エラーは、その結果として配信されることはありません。[サブエージェントの API エラー](/docs/ja/sub-agents#api-errors-in-subagents)でフォアグラウンドおよびバックグラウンドの動作を参照してください。

<h2 id="invoke-subagents">
  サブエージェントを呼び出す
</h2>

<h3 id="automatic-invocation">
  自動呼び出し
</h3>

Claude はタスクと各サブエージェントの `description` に基づいて、サブエージェントを呼び出すタイミングを自動的に判断します。例えば、「クエリチューニング用のパフォーマンス最適化スペシャリスト」という説明を持つ `performance-optimizer` サブエージェントを定義した場合、プロンプトでクエリの最適化について言及すると、Claude はそれを呼び出します。

Claude が正しいサブエージェントにタスクをマッチングできるように、明確で具体的な説明を記述してください。

<h3 id="explicit-invocation">
  明示的な呼び出し
</h3>

Claude が特定のサブエージェントを使用することを保証するには、プロンプトで名前を指定してください。

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

これにより自動マッチングをバイパスし、指定されたサブエージェントを直接呼び出します。

<h3 id="dynamic-agent-configuration">
  動的エージェント設定
</h3>

実行時の条件に基づいて、エージェント定義を動的に作成できます。この例では、異なる厳密性レベルを持つセキュリティレビュアーを作成し、厳密なレビューにはより高性能なモデルを使用しています。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # AgentDefinition を返すファクトリ関数
  # このパターンにより、実行時の条件に基づいてエージェントをカスタマイズできます
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # 厳密性レベルに基づいてプロンプトをカスタマイズ
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # 重要な洞察：高リスクのレビューにはより高性能なモデルを使用
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # エージェントはクエリ時に作成されるため、各リクエストで異なる設定を使用できます
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # 目的の設定でファクトリを呼び出す
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // AgentDefinition を返すファクトリ関数
  // このパターンにより、実行時の条件に基づいてエージェントをカスタマイズできます
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // 厳密性レベルに基づいてプロンプトをカスタマイズ
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // 重要な洞察：高リスクのレビューにはより高性能なモデルを使用
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // エージェントはクエリ時に作成されるため、各リクエストで異なる設定を使用できます
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // 目的の設定でファクトリを呼び出す
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  サブエージェント呼び出しの検出
</h2>

Claude はエージェントツールを通じてサブエージェントを呼び出します。サブエージェントが呼び出されたときを検出するには、`name` が `"Agent"` である `tool_use` ブロックをチェックしてください。サブエージェントのコンテキスト内からのメッセージには、`parent_tool_use_id` フィールドが含まれます。

<Note>
  このツールは `tool_use` ブロックでは `"Agent"` として表示されますが、`system:init` ツールリストでは `"Task"` として表示されます。Claude Code v2.1.63 より前では、`tool_use` ブロックもこれを `"Task"` と名付けていました。SDK バージョン間で検出が機能し続けるようにするには、`block.name` で両方の値に一致させてください。
</Note>

メッセージ構造は SDK 間で異なります。Python では、`message.content` を通じてコンテンツブロックに直接アクセスします。TypeScript では、`SDKAssistantMessage` が Claude API メッセージをラップするため、`message.message.content` を通じてコンテンツにアクセスします。

この例は、ストリーミングされたメッセージを反復処理し、サブエージェントが呼び出されたときと、その後のメッセージがそのサブエージェントの実行コンテキスト内から発信されたときをログに記録します。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  サブエージェントの再開
</h2>

サブエージェントを再開して、最初からやり直すのではなく、中断したところから続行できます。再開されたサブエージェントは、以前のすべてのツール呼び出し、結果、推論を含む完全な会話履歴を保持します。

サブエージェントが [`maxTurns`](#agentdefinition-configuration) の制限に達して停止すると、Claude Code はエージェントツール結果の出力を部分的なものとしてマークするため、Claude は実行が未完了であることを認識します。

サブエージェントが完了すると、エージェントツール結果には `agentId: <id>` を含むテキストブロックが含まれます。組み込みの [`Explore` および `Plan` エージェント](/docs/ja/sub-agents#built-in-subagents) はワンショットであり、`agentId` を返さないため、再開が必要な場合はカスタムエージェントまたは `general-purpose` を使用してください。サブエージェントをプログラムで再開するには：

1. **セッション ID をキャプチャする**：最初のクエリ中にメッセージから `session_id` を抽出します
2. **エージェント ID を抽出する**：エージェントツール結果テキストから `agentId` を解析します
3. **セッションを再開する**：2 番目のクエリのオプションで `resume: sessionId` を渡し、プロンプトにエージェント ID を含めます。各 `query()` 呼び出しはデフォルトで新しいセッションを開始し、サブエージェントのトランスクリプトにアクセスするには同じセッションを再開する必要があります。

<Note>
  カスタムエージェントを使用する場合、両方のクエリの `agents` パラメータで同じエージェント定義を渡してください。
</Note>

以下の例は、カスタム `endpoint-finder` エージェントを定義しています。最初のクエリはそれを実行し、エージェントツール結果からセッション ID とエージェント ID をキャプチャします。その後、2 番目のクエリはセッションを再開して、最初の分析からのコンテキストが必要なフォローアップ質問をします。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

サブエージェントのトランスクリプトは別のファイルに保存され、メイン会話とは独立して永続化されます。圧縮動作と `cleanupPeriodDays` クリーンアップ期間については、[Claude Code でサブエージェントを再開する](/docs/ja/sub-agents#resume-subagents) を参照してください。

<h2 id="tool-restrictions">
  ツール制限
</h2>

`tools` フィールドを使用して、サブエージェントが実行できることを制限します。

* **`tools` を省略**: サブエージェントは[サブエージェントで利用可能なすべてのツール](/docs/ja/sub-agents#available-tools)を取得します
* **ツールをリスト化**: サブエージェントはそれらのツールのみを取得します。例えば、ファイルを編集してはいけないコードレビュアーは、`["Read", "Grep", "Glob"]` を取得します

省略したツールはサブエージェントのセッションに含まれません。Claude はそれなしで動作し、権限プロンプトやエラーは表示されません。

この例は、コードを検査できるが、ファイルを変更したりコマンドを実行したりできない読み取り専用分析エージェントを作成します。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  一般的なツール組み合わせ
</h3>

| ユースケース   | ツール                                 | 説明                                        |
| :------- | :---------------------------------- | :---------------------------------------- |
| 読み取り専用分析 | `Read`、`Grep`、`Glob`                | コードを検査できますが、変更または実行はできません                 |
| テスト実行    | `Bash`、`Read`、`Grep`                | コマンドを実行し、出力を分析できます                        |
| コード変更    | `Read`、`Edit`、`Write`、`Grep`、`Glob` | コマンド実行なしで完全な読み取り/書き込みアクセス                 |
| フルアクセス   | すべてのツール                             | サブエージェントで利用可能なツールを継承します（`tools` フィールドを省略） |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  サブエージェントの深さ、並行実行数、支出をキャップする
</h2>

<Note>
  このセクションでは TypeScript SDK v0.3.219 および Python SDK v0.2.127 以降について説明しており、これらは Claude Code v2.1.219 以降をバンドルしたリリースです。以前のリリースでは、これらの制限の一部が欠落しているか、デフォルト値が異なるため、実行をバウンドするために依存する前にアップグレードしてください。[環境変数リファレンス](/docs/ja/env-vars)および[ターンと予算](/docs/ja/agent-sdk/agent-loop#turns-and-budget)には、各変数を追加した Claude Code バージョンと支出キャップのサブエージェント実装が記録されています。
</Note>

Claude はサブエージェントをいつ生成するか、また何個生成するかを独自に決定します。各サブエージェントは独自の API リクエストを行い、これはクエリの `total_cost_usd` にカウントされ、サブエージェント自体がサブエージェントを生成できるため、1 つのプロンプトはエージェントのツリーに成長する可能性があります。

この成長は 3 つの方法でキャップできます。サブエージェントがどの程度深くネストするか、一度に何個実行するか、クエリ全体がいくら支出するかです。深さと並行実行数の制限は [`env`](/docs/ja/agent-sdk/typescript#options) オプションを通じて環境変数として設定し、支出制限はクエリオプションとして設定します。

| 制限    | 設定方法                                                    | デフォルト                                                               | Claude Code が制限に達したときの動作                                                                                                                                                                                                         |
| :---- | :------------------------------------------------------ | :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 深さ    | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/ja/env-vars)  | メインエージェントの下に `3` 層のサブエージェント。`1` はサブエージェントが独自のサブエージェントを生成することを停止します  | 下層のサブエージェントが生成できないようにして、委譲された作業を自分で実行します。[ネストされたサブエージェント](/docs/ja/sub-agents#let-subagents-spawn-their-own-subagents)を参照してください                                                                                                       |
| 並行実行数 | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/ja/env-vars)  | `20` 個のサブエージェントが同時に実行され、Agent ツールで Claude が生成するすべてのサブエージェントをカウントします | 別のサブエージェントの生成を拒否し、`Concurrent subagent limit reached` を返します。実行中のカウントが制限を下回るまで待機します。[ultracode](/docs/ja/model-config#adjust-effort-level)がアクティブなセッションは拒否されることはありません。[並行サブエージェント制限](/docs/ja/sub-agents#concurrent-subagent-limit)を参照してください |
| 支出    | TypeScript では `maxBudgetUsd`、Python では `max_budget_usd` | 制限なし。呼び出し自体の支出をカウントし、サブエージェントリクエストが含まれます                            | キャップを 3 つの方法で実装します。より多くのサブエージェントの生成を拒否し、`Budget limit reached` を返します。実行中のバックグラウンドサブエージェントを停止し、`error_max_budget_usd` 結果サブタイプでクエリを終了します。セッション全体でキャップがどのように動作するかについては、[ターンと予算](/docs/ja/agent-sdk/agent-loop#turns-and-budget)を参照してください |

2 つの SDK は `env` オプションを異なる方法で処理します。TypeScript SDK はサブプロセス環境をそれで置き換えるため、`PATH` のような変数を保持するために `process.env` をそれに展開し、Python SDK はそれを継承された環境にマージします。この例ではネストをオフにし、最大 5 個のサブエージェントを同時に許可し、推定支出が \$5 に達したらクエリを停止します。

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

表示される内容は、クエリが到達する制限（ある場合）によって異なります。

* **支出キャップ以下**: `success` と推定コストが表示されます。
* **支出キャップに達した**: `error_max_budget_usd` が \$5 以上のコストで表示され、その後エラーハンドラーが実行されます。
* **並行実行数制限に達した**: メッセージストリームで `Concurrent subagent limit reached` を含む `tool_result` ブロックが表示されます。Claude は Agent ツールの結果として同じブロックを受け取ります。

<h3 id="run-opus-5-with-subagents">
  Opus 5 をサブエージェントで実行する
</h3>

Claude Opus 5 は以前のモデルよりもサブエージェントへの委譲をより容易に行うため、[深さ、並行実行数、支出制限](#cap-subagent-depth-concurrency-and-spend)は Opus 5 を実行するクエリで最も重要です。[Opus 5 プロンプティングガイド](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning)には、任意のプロンプトに追加できる委譲命令があります。Claude Code が独自の命令を追加するかどうかは、使用する[システムプロンプト](/docs/ja/agent-sdk/modifying-system-prompts#how-system-prompts-work)によって異なります。

* **`claude_code` プリセット**: モデルが Opus 5 の場合、Claude Code はシステムプロンプトに 1 行追加して、尋ねられない限り Agent ツールを呼び出さないよう Claude に指示します。Agent ツールは利用可能なままです。
* **カスタムプロンプト、または `systemPrompt` なし**: Claude Code はシステムプロンプトを構築しないため、その行は存在しません。プロンプティングガイドの委譲命令を独自のプロンプトに追加してください。

どちらの命令も Claude を導くだけなので、制限も設定してください。Claude Code は Claude がどのように委譲するかに関わらず、それらを実装します。

<h2 id="scale-up-with-dynamic-workflows">
  動的ワークフローでスケールアップ
</h2>

サブエージェントは、ターンごとに数個の委譲されたタスクに適しています。数十から数百のエージェントを調整する実行の場合は、`Workflow` ツールを使用してください。これにより、オーケストレーションを会話コンテキストの外で実行時が実行するスクリプトに移動します。[動的ワークフロー](/docs/ja/workflows)を参照して、ワークフローがターンごとのサブエージェント委譲とどのように異なるかを確認してください。

`Workflow` ツールは TypeScript Agent SDK v0.3.149 以降で利用可能です。`allowedTools` に `Workflow` を含めてワークフロー実行を自動承認します。ツール入力および出力スキーマは [TypeScript リファレンス](/docs/ja/agent-sdk/typescript#workflow)に記載されています。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude がサブエージェントに委譲していない
</h3>

Claude がサブエージェントに委譲する代わりにタスクを直接完了する場合：

* **明示的なプロンプトを使用する**: プロンプトでサブエージェントを名前で言及します。例えば「code-reviewer エージェントを使用して認証モジュールをチェックする」のように
* **明確な説明を書く**: サブエージェントをいつ使用すべきかを正確に説明し、Claude がタスクを適切にマッチングできるようにします

<h3 id="filesystem-based-agents-not-loading">
  ファイルシステムベースのエージェントが読み込まれていない
</h3>

Claude Code は `~/.claude/agents/` と `.claude/agents/` を監視し、数秒以内に新しいまたは編集されたエージェントファイルを検出します。再起動は不要です。定義が表示されない場合は、以下の原因を確認してください：

* **新しい `agents` ディレクトリ**: ウォッチャーはセッション開始時に存在していたディレクトリのみをカバーするため、新しいディレクトリ内の最初のファイルはセッション再起動が必要です。これが最も一般的な原因です。
* **無効なフロントマターまたは重複した `name`**: ファイルの YAML を確認し、既存のエージェントが同じ `name` を使用していないか確認してください。
* **`--disable-slash-commands`**: このフラグで開始されたセッションはこれらのディレクトリを監視せず、新しいファイルを読み込むには常に再起動が必要です。
* **追加されたディレクトリ下のファイル**: Claude Code は `add_dirs`（Python）または `additionalDirectories`（TypeScript）オプション、または CLI の `--add-dir` または `/add-dir` で追加されたディレクトリから `.claude/agents/` を読み込みますが、監視しないため、そこにある新しいまたは編集されたファイルはセッション再起動が必要です。
* **同じ名前のプログラマティックエージェント**: `query()` に渡される `agents` は、同じ名前のファイルシステムエージェントをオーバーライドします。

ファイル形式については、[サブエージェントファイルの書き方](/docs/ja/sub-agents#write-subagent-files)を参照してください。

<h2 id="related-documentation">
  関連ドキュメント
</h2>

* [Claude Code サブエージェント](/docs/ja/sub-agents)：ファイルシステムベースの定義を含む包括的なサブエージェントドキュメント
* [動的ワークフロー](/docs/ja/workflows)：スクリプトから多くのサブエージェントをオーケストレートして、1 つの会話には大きすぎるジョブを実行します
* [SDK 概要](/docs/ja/agent-sdk/overview)：Claude Agent SDK の開始方法
