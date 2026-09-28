> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# エージェントを設定する

> Agent SDK セッションを設定する：options オブジェクトを構成し、モデル、環境、制限を設定し、各機能オプションのページを見つけます。

Agent SDK セッションは、設定ファイル、環境変数、およびセッション開始時に渡す `options` オブジェクトから設定を読み込みます。このページでは、`options` オブジェクトを構成する方法と、どの設定ファイルと環境変数が制御するかを示します。

すべてのオプションの型とデフォルトについては、[`Options`](/docs/ja/agent-sdk/typescript#options)（TypeScript）および [`ClaudeAgentOptions`](/docs/ja/agent-sdk/python#claudeagentoptions)（Python）リファレンスを参照してください。

<h2 id="pass-options-to-a-session">
  セッションにオプションを渡す
</h2>

すべての `query()` 呼び出しは options オブジェクトを受け入れます：TypeScript では `Options`、Python では `ClaudeAgentOptions` です。各フィールドはオプションであり、オプションなしで開始されたセッションは SDK のデフォルトで実行されます。以下の例は、プロジェクトのオープン TODO を要約する読み取り専用セッションを設定します。ペアは、スペルが異なる TypeScript / Python として読み取られます：

* **`model`**：モデルを選択します
* **`allowedTools` / `allowed_tools`**：読み取り専用ツールリストを事前承認します
* **`maxTurns` / `max_turns`**：ターン数をキャップします
* **`cwd`**：作業ディレクトリを設定します

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

`cwd` を自分のプロジェクトの 1 つに指定して、例を実行します。そのプロジェクトのオープン TODO の要約は、結果メッセージが到着したときに出力されます。

`allowedTools`（TypeScript）または `allowed_tools`（Python）は、リストされたツールを事前承認するため、それらへの呼び出しは承認を待たずに実行されます。リストの外側のツールは利用可能なままです。Claude が未リストのツールを呼び出すと、権限モードが呼び出しを実行するかどうかを決定します。詳細については、[許可と拒否ルール](/docs/ja/agent-sdk/permissions#allow-and-deny-rules)を参照してください。

<h2 id="load-settings-files">
  設定ファイルを読み込む
</h2>

設定ファイルは options オブジェクトを超えた設定を提供します。2 つのオプションが読み込み方法を制御します：

* **`settingSources` / `setting_sources`**：どのファイルシステムソースを読み込むかを制御します：ユーザー、プロジェクト、ローカル。設定ファイルと CLAUDE.md ファイルはこれらのソースを通じて到着します。
* **`settings`**：設定ファイルパスまたはいずれかの言語のインライン JSON 文字列を読み込み、TypeScript は設定オブジェクトも受け入れます。渡すフォームに関係なく、ユーザー、プロジェクト、ローカルファイルシステム設定をオーバーライドします。管理ポリシー設定のみがより高いランクです。リファレンスは TypeScript の [設定の優先順位](/docs/ja/agent-sdk/typescript#settings-precedence) および Python の [設定の優先順位](/docs/ja/agent-sdk/python#settings-precedence) の下で完全な優先順位を文書化しています。

ユーザー、プロジェクト、ローカル設定を無効にするには `[]` を渡します。詳細については、[SDK で Claude Code 機能を使用する](/docs/ja/agent-sdk/claude-code-features)を参照してください。

<h2 id="choose-a-model">
  モデルを選択する
</h2>

`model` オプション、設定、または環境がモデルを選択しない限り、新しいセッションは [Claude Code のデフォルトモデル](/docs/ja/model-config#default-model-setting)で開始されます。これらのソースの順序については、[モデルを設定する](/docs/ja/model-config#setting-your-model)を参照してください。特定のモデルをピン留めするか、より小さいモデルを選択して、より高速で安価なエージェントを実現するには、`model` を設定します。値はモデルエイリアスまたは完全なモデル名を取ります。エイリアスとそれらが解決するバージョンは [モデルエイリアス](/docs/ja/model-config#model-aliases)の下にリストされています。

バックアップモデルに名前を付けるには、`fallbackModel`（TypeScript）または `fallback_model`（Python）を設定します。プライマリがオーバーロードされているか利用できない場合、セッションはバックアップに切り替わります。プライマリは各ユーザーターンの開始時に再試行されるため、停止が解決されるとセッションはそれに戻ります。

どちらの言語でも、オプションは単一のモデルまたはコンマ区切りのバックアップリストを受け入れます。順序とチェーンキャップについては、[フォールバックモデルチェーン](/docs/ja/model-config#fallback-model-chains)を参照してください。TypeScript では、`model` に等しいフォールバックはスタートアップでエラーをスローします。

以下の例は、TypeScript のフォールバックリストと Python の単一フォールバックを示しています：

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  [Messages API](https://platform.claude.com/docs/en/api/messages) リクエストパラメータ `temperature`、`top_p`、および `max_tokens` には、どちらの言語でも options オブジェクトにフィールドがありません。[努力レベル](/docs/ja/agent-sdk/agent-loop#effort-level)または [支出キャップ](#limit-turns-and-spend)を設定するか、これらのパラメータが直接必要な場合は Messages API を呼び出します。
</Note>

<h2 id="set-environment-variables">
  環境変数を設定する
</h2>

`env` オプションは、セッションを実行する Claude Code プロセスの環境変数を設定します。値が継承された環境を置き換えるか、それにマージするかは言語によって異なります：

* **TypeScript**：`env` はサブプロセス環境を置き換えます
* **Python**：SDK は値を継承された環境にマージし、値は継承されたものをオーバーライドします

TypeScript では、`process.env` を `env` に展開して、`PATH`、`HOME`、`ANTHROPIC_API_KEY` などの継承された変数を保持します。`env` を設定しないままにすると、サブプロセスは両方の言語で環境を継承します。

例は、`ANTHROPIC_BASE_URL` を設定してゲートウェイを通じて API トラフィックをルーティングします。

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

渡す変数は Claude Code 自体を設定することもできます。Claude Code プロセスが読み込む変数については、[環境変数](/docs/ja/env-vars)を参照してください。API タイムアウトとスタール検出をこの方法で調整するには、[TypeScript リファレンス](/docs/ja/agent-sdk/typescript#handle-slow-or-stalled-api-responses)または [Python リファレンス](/docs/ja/agent-sdk/python#handle-slow-or-stalled-api-responses)の「遅いまたは停止した API レスポンスを処理する」セクションに従ってください。

<h2 id="set-the-working-directory">
  作業ディレクトリを設定する
</h2>

特定のディレクトリでセッションを実行するには、`cwd` を設定します。`cwd` を設定しないままにすると、セッションはプロセスの作業ディレクトリで実行されます。どちらの SDK にも `cwd` のセッターはありません。別のディレクトリで実行するには、その `cwd` で別のセッションを開始します。

Claude Code は作業ディレクトリを読み込んで、以下を決定します：

* **プロジェクト設定とフック**：どのプロジェクトの [設定とフックが読み込まれるか](/docs/ja/agent-sdk/claude-code-features)
* **スキル**：[セッションスキルが発見される](/docs/ja/agent-sdk/skills)場所
* **セッションストレージ**：[保存されたセッションが属するプロジェクト](/docs/ja/agent-sdk/session-storage)

ツールが作業ディレクトリの外側のファイルに到達できるようにするには、`additionalDirectories`（TypeScript）または `add_dirs`（Python）でパスを追加します。その付与の範囲については、[追加ディレクトリはファイルアクセスを付与し、設定ではない](/docs/ja/permissions#additional-directories-grant-file-access-not-configuration)を参照してください。

<h2 id="limit-turns-and-spend">
  ターンと支出を制限する
</h2>

`maxTurns` / `max_turns` および `maxBudgetUsd` / `max_budget_usd` でターンと支出をキャップします。両方のキャップは設定しないままにすると無効です。セッションがキャップに達すると、実行は、サブタイプがキャップに名前を付ける結果メッセージで終了します。`error_max_turns` または `error_max_budget_usd`。次に何が起こるかは入力モードによって異なります：

* **シングルショット `query()`**：SDK はキャップ結果を生成してから発生するため、ループを try ブロックでラップしてエラーを超えて続行します
* **ストリーミング入力**：セッションはキャップ結果を超えて生きたままであり、max-turns カウントは各キューに入ったメッセージに対して開始されます。予算合計はメッセージ全体で蓄積され、支出がキャップに達すると、同じ会話の後のメッセージは同じ予算結果で終了します。[`/clear`](/docs/ja/agent-sdk/cost-tracking) は予算を開始します

2 つのキャップは `0` を異なる方法で処理します：

* **`maxTurns` / `max_turns`**：`0` はセッションをターン制限なしで実行します。オプションを設定しないままにするのと同じです
* **`maxBudgetUsd` / `max_budget_usd`**：CLI はスタートアップで `0` を無効な金額として拒否し、セッションは実行されません

サブエージェント支出を含む両方のキャップの詳細については、[ターンと予算](/docs/ja/agent-sdk/agent-loop#turns-and-budget)を参照してください。

<h2 id="change-configuration-mid-session">
  セッション中に設定を変更する
</h2>

[ストリーミング入力](/docs/ja/agent-sdk/streaming-vs-single-mode)でセッションを開始する場合、実行中にモデルと権限モードを切り替えることができます。セッターを呼び出す場所は言語によって異なります：

* **TypeScript**：`query()` が返すオブジェクトのメソッド
* **Python**：[`ClaudeSDKClient`](/docs/ja/agent-sdk/python#claudesdkclient)のメソッド。`query()` は制御メソッドのないプレーンイテレータを返すため

両方の言語には同じセッターがあります：

* **`setModel()` / `set_model()`**：モデルを切り替えます。モデルなしで呼び出して、渡した `model` ではなく [Claude Code のデフォルトモデル](/docs/ja/model-config#default-model-setting)に切り替えます。
* **`setPermissionMode()` / `set_permission_mode()`**：権限モードを切り替えます

TypeScript には `applyFlagSettings()` と `updateSettings()` もあります：

* **`applyFlagSettings()`**：`await session.applyFlagSettings({ effortLevel: "high" })` のように実行時に設定を適用します。メソッドは options フィールドではなく設定ファイルキーを取るため、スキーマについては [`applyFlagSettings()` リファレンス](/docs/ja/agent-sdk/typescript#applyflagsettings)を確認し、どのキーがセッション中に有効になるかを確認してください。
* **`updateSettings()`**：許可リストに登録されたキーを設定ファイルに書き込みます。[`updateSettings()` リファレンス](/docs/ja/agent-sdk/typescript#updatesettings)は各ソースが受け入れるキーとバージョンフロアに名前を付けます。
  * `"localSettings"` を渡して、プロジェクトのローカル設定ファイルに書き込みます。`await session.updateSettings("localSettings", { outputStyle: "Explanatory" })` のように使用します。書き込まれたキーはセッションの次のリクエストで有効になり、`local` 設定を読み込む後のセッションに対して永続化されます。
  * `"userSettings"` を渡して、このソースが受け入れる唯一のキーである `effortLevel` を書き込みます。Claude Code はそれをセッションの現在のモデルのデフォルト努力レベルとして保存し、実行中のセッションの努力は変わりません。

以下の例は 2 ターンセッションを実行し、ターン間で設定を変更し、各ターンに答えたモデルを出力します。TypeScript では、プロンプトストリームは 2 番目のメッセージをセッターが実行されるまで保持し、2 番目のターンは新しいモデルで実行されます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Claude API では、プログラムは `First turn model: claude-sonnet-5` を出力してから、切り替え後に `Second turn model: claude-opus-5` を出力します。

<Note>
  各モデルは独自のプロンプトキャッシュを持つため、セッション中の切り替え後、次のリクエストは新しいモデルのレートでキャッシュされていない完全な会話を再計算します。詳細については、[モデルの切り替え](/docs/ja/prompt-caching#switching-models)を参照してください。
</Note>

<h2 id="configure-specific-features">
  特定の機能を設定する
</h2>

以下の表は、各オプションをそれが設定する機能にマップします。このページでカバーされていないオプションについては、[TypeScript](/docs/ja/agent-sdk/typescript#options) および [Python](/docs/ja/agent-sdk/python#claudeagentoptions) リファレンスを参照してください。目標は知っているが、どのオプションがそれを提供するかわからない場合は、[正しい機能を選択する](/docs/ja/agent-sdk/claude-code-features#choose-the-right-feature)から始めてください。

| TypeScript                | Python                      | 制御                         | カバー対象                                                                                                                                                                                   |
| ------------------------- | --------------------------- | -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | エージェントが承認なしでできることは何か       | [権限を設定する](/docs/ja/agent-sdk/permissions)                                                                                                                                                    |
| `allowedTools`            | `allowed_tools`             | どのツール呼び出しが事前承認されるか         | [権限を設定する](/docs/ja/agent-sdk/permissions)                                                                                                                                                    |
| `canUseTool`              | `can_use_tool`              | ツール呼び出しの承認コールバック           | [ツール承認リクエストを処理する](/docs/ja/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                               |
| `systemPrompt`            | `system_prompt`             | エージェントの指示                  | [システムプロンプトを変更する](/docs/ja/agent-sdk/modifying-system-prompts)                                                                                                                                |
| `settingSources`          | `setting_sources`           | どのファイルシステム設定が読み込まれるか       | [SDK で Claude Code 機能を使用する](/docs/ja/agent-sdk/claude-code-features)                                                                                                                         |
| `mcpServers`              | `mcp_servers`               | 外部ツールサーバー                  | [MCP で外部ツールに接続する](/docs/ja/agent-sdk/mcp)                                                                                                                                                    |
| `agents`                  | `agents`                    | サブエージェント定義                 | [サブエージェント](/docs/ja/agent-sdk/subagents)                                                                                                                                                     |
| `hooks`                   | `hooks`                     | ライフサイクルポイントでのコールバック        | [フック](/docs/ja/agent-sdk/hooks)                                                                                                                                                              |
| `skills`                  | `skills`                    | どのスキルが読み込まれるか              | [スキルでエージェントを拡張する](/docs/ja/agent-sdk/skills)                                                                                                                                                 |
| `plugins`                 | `plugins`                   | どのプラグインが読み込まれるか            | [プラグイン](/docs/ja/agent-sdk/plugins)                                                                                                                                                          |
| `outputFormat`            | `output_format`             | 構造化出力スキーマ                  | [構造化出力](/docs/ja/agent-sdk/structured-outputs)                                                                                                                                               |
| `resume`                  | `resume`                    | 保存されたセッションを続行する            | [セッション](/docs/ja/agent-sdk/sessions)                                                                                                                                                         |
| `forkSession`             | `fork_session`              | セッションをブランチする               | [セッション](/docs/ja/agent-sdk/sessions)                                                                                                                                                         |
| `sessionStore`            | `session_store`             | 外部セッション永続化                 | [セッションストレージ](/docs/ja/agent-sdk/session-storage)                                                                                                                                             |
| `enableFileCheckpointing` | `enable_file_checkpointing` | 巻き戻し可能なファイル編集              | [ファイルチェックポイント](/docs/ja/agent-sdk/file-checkpointing)                                                                                                                                        |
| `effort`                  | `effort`                    | Claude がレスポンスにどれだけの作業を入れるか | [努力レベル](/docs/ja/agent-sdk/agent-loop#effort-level)                                                                                                                                          |
| `sandbox`                 | `sandbox`                   | ツール実行のサンドボックス動作            | [TypeScript](/docs/ja/agent-sdk/typescript#sandbox-configuration) および [Python](/docs/ja/agent-sdk/python#sandbox-configuration) リファレンス。[安全なデプロイ](/docs/ja/agent-sdk/secure-deployment)のデプロイメントコンテキスト付き |

<h2 id="next-steps">
  次のステップ
</h2>

設定を構成した作業中のエージェントを確認するには：

* **[クイックスタート](/docs/ja/agent-sdk/quickstart)**：最初のエージェントをエンドツーエンドで構築して実行します
* **[例](/docs/ja/agent-sdk/examples)**：構築したいものと一致する完全で実行可能なプロジェクトまたはガイド付き Claude Cookbook レシピを見つけます
* **[マルチテナント分離](/docs/ja/agent-sdk/hosting#multi-tenant-isolation)**：`settingSources` / `setting_sources`、`env`、および `cwd` で各テナントの設定とメモリを分離します
