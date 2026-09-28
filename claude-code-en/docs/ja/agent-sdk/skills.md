> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills でエージェントを拡張する

> Claude Agent SDK セッションで Claude が呼び出せる Skills を制御し、名前でコマンドをディスパッチし、セッションが検出する Skills を作成します

Agent Skills は、Claude が関連する場合に呼び出す特殊な機能で Claude を拡張します。Skills は、指示、説明、およびオプションのサポートリソースを含む `SKILL.md` ファイルとしてパッケージ化されます。このページでは、[Agent SDK セッションのコマンド](#commands-in-agent-sdk-sessions)についても説明しています。

Skills に関する包括的な情報（利点、アーキテクチャ、作成ガイドラインを含む）については、[Agent Skills の概要](https://platform.claude.com/docs/ja/agents-and-tools/agent-skills/overview)を参照してください。

<h2 id="how-skills-work-with-the-agent-sdk">
  Agent SDK での Skills の動作方法
</h2>

Claude Agent SDK を使用する場合、Skills は以下のように機能します。

* **ファイルシステムアーティファクトとして定義される**：各 Skill を `SKILL.md` ファイルとして独自のディレクトリ（`.claude/skills/<name>/SKILL.md` など）に作成します
* **ファイルシステムから読み込まれる**：SDK は `settingSources`（TypeScript）または `setting_sources`（Python）によって管理されるファイルシステムの場所から Skills を読み込みます
* **自動的に検出される**：ファイルシステム設定が読み込まれると、SDK はスタートアップ時にユーザーおよびプロジェクトディレクトリから Skill メタデータを検出し、Claude が Skill を呼び出すときに完全なコンテンツを読み込みます
* **モデルによって呼び出される**：Claude はコンテキストに基づいて自律的に使用するタイミングを選択します
* **ユーザーによって呼び出される**：プロンプトで `/<name>` を送信して Skill を直接ディスパッチします。[Agent SDK セッションのコマンド](#commands-in-agent-sdk-sessions)を参照してください
* **`skills` オプションでスコープされる**：検出された Skills はデフォルトで有効になります。Skill 名のリスト、`"all"`、または `[]` を渡して、Claude が呼び出せる Skills を制御します

サブエージェント（[`agents` オプション](/docs/ja/agent-sdk/subagents#programmatic-definition-recommended)で定義できる）とは異なり、Skills はディスク上のファイルとして作成します。SDK は Skills を登録するためのプログラマティック API を提供しません。

<Note>
  Skills はファイルシステム設定ソースを通じて検出されます。デフォルトの `query()` オプションでは、SDK はユーザーおよびプロジェクトソースを読み込むため、`~/.claude/skills/`、`<cwd>/.claude/skills/`、および `<cwd>` の親ディレクトリからリポジトリルートまでの `.claude/skills/` の Skills が利用可能です。プロジェクトソースは、`additionalDirectories`（TypeScript）または `add_dirs`（Python）を通じて渡す各ディレクトリの `<dir>/.claude/skills/` もカバーしています。これは、SDK がそれらのディレクトリを Claude Code に [`--add-dir`](/docs/ja/skills#skills-from-additional-directories) として渡すためです。`settingSources` を明示的に設定する場合は、プロジェクトおよび追加ディレクトリの Skills を保持するために `'project'` を含め、個人用 Skills を保持するために `'user'` を含めるか、[`plugins` オプション](/docs/ja/agent-sdk/plugins)を使用して特定のパスから Skills を読み込みます。
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Agent SDK で Skills を使用する
</h2>

`query()` の `skills` オプションを設定して、セッションで Claude が呼び出せる Skills を制御します。省略した場合、検出された Skills が有効になり、Skill ツールが利用可能になり、CLI の動作と一致します。`"all"` を渡してすべての検出された Skill を呼び出せるようにするか、Skill 名のリストを渡してそれらのみを許可するか、`[]` を渡して Claude が Skill を呼び出せないようにします。

たとえば、Claude が 2 つの名前付き Skills のみを呼び出せるようにするには：

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  セッションで Skills をセットアップする
</h3>

`skills` を設定すると、SDK は Skill ツールを `allowedTools` に自動的に追加します。明示的な `tools` リストも渡す場合は、Claude が Skills を呼び出せるようにそのリストに `"Skill"` を含めてください。

設定されると、Claude はファイルシステムから Skills を自動的に検出し、ユーザーのリクエストに関連する場合に呼び出します。

次の例は、検出されたすべての Skill をセッションで有効にし、Skills が一般的に必要とするツールを事前承認します。この例は `cwd` をプロセスの現在の作業ディレクトリに設定するため、現在のディレクトリまたはリポジトリルートまでの親ディレクトリに `.claude/skills/` ディレクトリを持つプロジェクト内から実行してください：

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Skills が読み込まれたことを確認する
</h3>

ストリームの開始付近で、SDK はサブタイプ `init` のシステムメッセージを生成します。その `skills` 配列をチェックして、Claude が作業を開始する前に Skills が読み込まれたことを確認します。配列には、`description` または `when_to_use` frontmatter フィールドで定義したユーザー呼び出し可能な Skills と、[Claude Code に含まれるバンドルされた Skills](/docs/ja/skills#bundled-skills) が含まれます。

配列はユーザー呼び出し可能な Skills のみをリストします。フロントマターで [`user-invocable: false`](/docs/ja/skills#control-who-invokes-a-skill) を持つ Skill は読み込まれ、Claude で利用可能なままですが、配列には表示されません。配列は、`skills` リストに含まれているかどうかに関わらず、同じ Skills をリストします。

<h3 id="allow-only-specific-skills">
  特定の Skills のみを許可する
</h3>

Claude が特定の Skills のみを呼び出せるようにするには、それらの名前を `skills` リストで渡します。名前は `SKILL.md` の `name` フィールドまたは Skill のディレクトリ名と一致します。プラグイン提供の Skills には `plugin:skill` を使用します。

リストは正確な Skill 名のみを受け入れます。エントリが正確な名前として機能できない場合、`query()` はセッション開始前にリストを拒否します。[無効な Skill 名エラー](#invalid-skill-name-error)で名前ルールと各 SDK が発生させるエラーを参照してください。

モデルはリストされていない Skills を見ず、Skill ツールはそれらを拒否しますが、それらのファイルはディスク上に残り、Read および Bash を通じてアクセス可能です。リストを制限しても、[名前でのディスパッチ](#dispatch-commands-by-name)は制限されません。

検出されたすべての Skill を呼び出せるようにするには、ワイルドカードではなく `skills: "all"` を渡します。

<h2 id="commands-in-agent-sdk-sessions">
  Agent SDK セッションのコマンド
</h2>

このセクションは SDK のコマンドドキュメントです。コマンドは、プロンプトで `/<name>` を送信して実行するものです。コマンドサーフェスのエントリは、それらをサポートするものが異なります：

* **組み込みコマンド**：SDK が実行する Claude Code プロセスにコード化されたロジックを実行します。たとえば `/compact`
* **バンドルされた Skills**：Claude Code に含まれるプロンプトアーティファクト。たとえば `/code-review`
* **あなたの Skills**：あなたが作成するプロンプトアーティファクト。各ディレクトリは `SKILL.md` ファイルを保持します。ユーザー呼び出し可能な Skill の名前はサーフェスに自動的に参加するため、独自の `/security-check` をディスパッチして組み込みを実行することは同じ方法で機能します
* **カスタムコマンドファイル**：同じ動作を持つ古いアーティファクト形式。`.claude/commands/` 内のフラットな Markdown ファイル。ファイル名がコマンド名になります。Skills はそれらの推奨される後継です

デフォルトでは、あなたと Claude の両方が任意の skill を呼び出せます。skill の[フロントマター](/docs/ja/skills#control-who-invokes-a-skill)を通じてどちらかのパスを制限できます。2 つの用語の定義については、用語集の[コマンド](/docs/ja/glossary#command)および[Skill](/docs/ja/glossary#skill)エントリを参照してください。すべての組み込みについては[Claude Code のコマンド](/docs/ja/commands)を、両方のアーティファクト形式の完全なガイドについては[Claude を skills で拡張する](/docs/ja/skills)を参照してください。

<h3 id="discover-available-commands">
  利用可能なコマンドを検出する
</h3>

SDK を通じてインタラクティブターミナルなしで機能するコマンドをディスパッチできます。`system/init` メッセージは、セッションで利用可能なコマンドを `slash_commands` フィールドにリストします。`/theme` や `/terminal-setup` などのインタラクティブターミナルが必要なコマンドはリストに表示されません。セッション開始時にフィールドにアクセスします：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

印刷されたリストは、組み込みコマンド、バンドルされた Skills、ユーザー呼び出し可能な Skills、および `.claude/commands/` ファイルを混在させます：

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

フロントマターで [`user-invocable: false`](/docs/ja/skills#control-who-invokes-a-skill) を持つ skill はこのリストや[Confirm skills loaded](#confirm-skills-loaded)からの `skills` 配列に表示されません。[MCP サーバー](/docs/ja/agent-sdk/mcp)を設定するセッションは、[MCP プロンプトをコマンドとして使用](/docs/ja/mcp#use-mcp-prompts-as-commands)することもできます。

<h3 id="dispatch-commands-by-name">
  名前でコマンドをディスパッチする
</h3>

プロンプト文字列にコマンドを含めて、通常のテキストを送信するのと同じ方法でコマンドを送信します。ディスパッチは `skills` オプションに依存しません。`/<name>` を送信すると、`skills` リストがそれを省略している場合でも、ユーザー呼び出し可能な Skill が実行されます。会話履歴に作用するコマンド（`/compact` など）は、作業する前のメッセージが必要です。

`/<name>` がセッション内のコマンドにも組み込み Claude Code コマンドにも一致しない場合、クエリは失敗しません。Claude Code はプロンプトを Claude に通常のメッセージとして送信し、コマンドが実行されなかったことを示すメモを付けるため、クエリはモデルターンを費やし、Claude の返信を返します。v2.1.274 より前では、何にも一致しない `/<name>` はモデルターンなしで `Unknown command: /<name>` を結果として返しました。

セッションで利用できない `/theme` などの組み込み Claude Code コマンドに一致する `/<name>` は、モデルターンなしで `/theme isn't available in this environment.` を結果として返します。

<Note>
  コマンドは、他のプロンプトと同様に `maxTurns` / `max_turns` 制限に達する可能性があり、`success` ではなくエラー結果でクエリを終了します。エラー結果契約については、[結果を処理する](/docs/ja/agent-sdk/agent-loop#handle-the-result)を参照してください。コマンドが制限に達する可能性がある場合は、[単一メッセージ入力](/docs/ja/agent-sdk/streaming-vs-single-mode#single-message-input)に示されているように、TypeScript で `try`/`catch` またはPython で `try`/`except` でループをラップするか、作業が完了するのに十分な高さに `maxTurns` を設定します。
</Note>

<h3 id="compact-history-with-/compact">
  `/compact` で履歴をコンパクト化する
</h3>

`/compact` コマンドは、古いメッセージを要約しながら重要なコンテキストを保持することで、会話履歴のサイズを削減します。コンパクト化には、要約する十分な前のメッセージを持つ既存の会話が必要です。この例は、最初に会話を行い、次にそれをコンパクト化し、結果を報告する `compact_boundary` システムメッセージを読みます：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  `compact_boundary` メッセージは、コンパクト化が実行された場合にのみ到着します。要約するものがない場合、`/compact` は代わりに理由を報告します。実行は依然として `success` 結果で終了し、`compact_boundary` メッセージはなく、結果テキストは理由を含みます。たとえば、短い交換の後に `Not enough messages to compact.` です。新しいワンショット `query()` 呼び出しは空のコンテキストで開始するため、このパターンを前のターンを持つセッション（たとえば[ストリーミング入力モード](/docs/ja/agent-sdk/streaming-vs-single-mode)またはセッションを再開するとき）で使用します。
</Note>

<h3 id="reset-context-with-/clear">
  `/clear` でコンテキストをリセットする
</h3>

`/clear` コマンドは、会話を空のコンテキストにリセットするため、後続のプロンプトは前の会話履歴なしで開始します。前の会話はディスク上に残ります。[`resume` オプション](/docs/ja/agent-sdk/sessions#resume-by-id)にセッション ID を渡すことで、その会話に戻ることができます。

`/clear` は[ストリーミング入力モード](/docs/ja/agent-sdk/streaming-vs-single-mode)で有用です。単一の接続を通じて複数のプロンプトを送信します。ワンショット `query()` 呼び出しの場合、各呼び出しは既に空のコンテキストで開始するため、`/clear` を送信しても実際の効果はありません。代わりに新しい `query()` を開始します。

<h2 id="create-skills">
  Skills を作成する
</h2>

各 Skill を、YAML フロントマターと Markdown コンテンツを含む `SKILL.md` ファイルを含むディレクトリとして作成します。`description` フィールドは、Claude が Skill を呼び出すタイミングを決定します。

**ディレクトリ構造の例**：

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  検出レベルを選択する
</h3>

2 つの最も一般的な[検出レベル](/docs/ja/skills#where-skills-live)のいずれかで Skills を保存します：

* **プロジェクト Skills**：`.claude/skills/`。現在のプロジェクトでのみ利用可能
* **個人用 Skills**：`~/.claude/skills/`。すべてのプロジェクト全体で利用可能

`.claude/commands/` に既存のカスタムコマンドファイルがある場合、それらは機能し続けます。`.claude/commands/deploy.md` のコマンドファイルは `/deploy` を作成し、`.claude/skills/deploy/SKILL.md` の Skill と同じ方法で機能します。コマンドファイルと Skill が名前を共有する場合、[名前を共有する Skills を解決する](/docs/ja/skills#resolve-skills-that-share-a-name)を参照して、どちらが実行されるかを確認します。SDK は `.claude/commands/` および `~/.claude/commands/` ファイルを Skills と同じ 2 つのスコープから読み込みます。両方のアーティファクト形式の完全なガイドについては、[Claude を Skills で拡張する](/docs/ja/skills)を参照してください。

<h3 id="create-and-dispatch-your-first-skill">
  最初の Skill を作成してディスパッチする
</h3>

完全なフローを確認するには、`.claude/skills/security-check/SKILL.md` を作成します：

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

ファイルが存在すると、Skill は SDK を通じて利用可能になります。Claude はリクエストが説明と一致するときに呼び出し、直接ディスパッチできます：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

成功した実行は、テキストがスキャン結果を含む `success` 結果で終了します。シードされた問題を持つ小さな Express アプリに対して、結果テキストは以下で始まります：

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Skill の名前は、init メッセージの `slash_commands` 配列にも表示されます。

<Note>
  Claude Code には、バンドルされた `code-review` および `verify` Skills が含まれています。`.claude/commands/` ファイルにそれらの 1 つの後に名前を付ける場合（たとえば `.claude/commands/code-review.md`）、ファイルのコマンドはバンドルされた Skill をシャドウし、`slash_commands` は名前を 1 回リストします。
</Note>

<h2 id="pre-approve-tools-for-skills">
  Skills のツールを事前承認する
</h2>

<Note>
  プロジェクトおよび個人用 Skills の場合、Claude Code は SDK セッションで [`allowed-tools`](/docs/ja/skills#pre-approve-tools-for-a-skill) フロントマターフィールドを適用します。クエリ設定の `allowedTools` オプション（Python では `allowed_tools`）を通じて、これらの Skills のツールを事前承認することもできます。[claude.ai から同期された](/docs/ja/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) Skills は、独自のフロントマタールールに従います。
</Note>

Skills はセッションのツールで実行されます。以下の例は、`allowedTools`（Python では `allowed_tools`）で `Read`、`Grep`、および `Glob` を事前承認するため、Claude は[security-check Skill](#create-and-dispatch-your-first-skill)を実行しながらファイルを検査でき、承認を停止する必要がありません：

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

ストリームでは、Skill 呼び出しは Skill ツール使用として表示され、その後にプロジェクトファイルの Read 呼び出しが続きます。実行は、テキストが結果を含む `success` 結果で終了します。

リストは、他を制限するのではなく、名前付きツールを事前承認します。権限モードおよび `canUseTool` コールバックを含む完全な権限フローについては、[権限](/docs/ja/agent-sdk/permissions)を参照してください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="skills-not-found">
  Skills が見つからない
</h3>

**settingSources 設定を確認する**：SDK は `user` および `project` 設定ソースを通じて Skills を検出します。`settingSources`/`setting_sources` を明示的に設定し、それらのソースを省略した場合、SDK は Skills を読み込みません：

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

各ソースが読み込む Skill ディレクトリについては、[ファイルシステムソーステーブル](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)を参照してください。`settingSources`/`setting_sources` の詳細については、[TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript#settingsource)または [Python SDK リファレンス](/docs/ja/agent-sdk/python#settingsource)を参照してください。

**作業ディレクトリを確認する**：SDK は `cwd` オプションの `.claude/skills/` およびリポジトリルートまでのすべての親ディレクトリから Skills を読み込みます。`cwd` が `.claude/skills/` を含むディレクトリ以下を指していることを確認してください。同じリポジトリ内である必要があります：

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

完全なパターンについては、[Agent SDK で Skills を使用する](#use-skills-with-the-agent-sdk)を参照してください。

**ファイルシステムの場所を確認する**：

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill が使用されていない
</h3>

**`skills` オプションを確認する**：`skills` リストを渡した場合、Skill の名前が含まれていることを確認します。Claude がリストされていない Skill を呼び出そうとすると、Skill ツールは `Skill <name> is not in this session's skills allowlist` を返します。名前をリストに追加するか、プロンプトで `/<name>` を送信して Skill を直接ディスパッチします。これはリストなしで機能します。

**説明を確認する**：具体的で関連するキーワードが含まれていることを確認します。効果的な説明の書き方に関するガイダンスについては、[Agent Skills のベストプラクティス](https://platform.claude.com/docs/ja/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions)を参照してください。

<h3 id="invalid-skill-name-error">
  無効な Skill 名エラー
</h3>

`skills` リストの名前が正確な Skill 名として機能できない場合、`query()` は Claude Code プロセスを開始する前にリストを拒否します。拒否をトリガーする名前には以下が含まれます：

* 空の名前
* 括弧、コンマ、または制御文字を含む名前
* 空白でパディングされた名前
* `*` の裸の形式または`:*` サフィックスなどのワイルドカード形式

各 SDK は拒否を異なる方法でサーフェスします：

<Tabs>
  <Tab title="TypeScript">
    TypeScript SDK は、エントリが破ったルールを述べる `Error` をスローします。たとえば、`skills: ["docs:*"]` はスローします：

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    空の名前は `Skill names must be non-empty strings.` を報告します

    TypeScript Agent SDK 0.3.221 より前では、SDK はこのチェックを実行しませんでした。
  </Tab>

  <Tab title="Python">
    Python SDK は、エントリが破ったルールを述べる `ValueError` を発生させます。たとえば、`skills=["docs:*"]` は発生させます：

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    空の名前は `Skill names must be non-empty strings` を報告します。

    Python Agent SDK 0.2.129 より前では、SDK はこのチェックを実行しませんでした。
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  追加のトラブルシューティング
</h3>

YAML 構文エラーおよびデバッグなどの一般的な Skills トラブルシューティングについては、[Claude Code Skills トラブルシューティングセクション](/docs/ja/skills#troubleshooting)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

[Claude Code Skills ガイド](/docs/ja/skills)は、作成を詳細にカバーしています。そのガイダンスは SDK セッションに適用されます。これらのセクションから始めてください：

* [フロントマターリファレンス](/docs/ja/skills#frontmatter-reference)：サポートされているすべてのフィールド
* [Skills に引数を渡す](/docs/ja/skills#pass-arguments-to-skills)：`$ARGUMENTS`、`$0`、`$1`、および Skill スタッキング。[完全な置換テーブル](/docs/ja/skills#available-string-substitutions)は、名前付き引数および `${CLAUDE_*}` 変数を追加します
* [動的コンテキストを注入する](/docs/ja/skills#inject-dynamic-context)：Claude が Skill コンテンツを見る前に実行される `` !`command` `` 行
* [Skills が読み込まれる場所を選択する](/docs/ja/skills#where-skills-live)：すべての Skill の場所、プラグイン名前空間、および 2 つが名前を共有するときにどの Skill が実行されるか

<h2 id="related-resources">
  関連リソース
</h2>

* [Claude Code のコマンド](/docs/ja/commands)：完全なコマンドサーフェス（すべての組み込みを含む）
* [Agent Skills の概要](https://platform.claude.com/docs/ja/agents-and-tools/agent-skills/overview)：概念的な概要、利点、およびアーキテクチャ
* [Agent Skills のベストプラクティス](https://platform.claude.com/docs/ja/agents-and-tools/agent-skills/best-practices)：効果的な Skills のための作成ガイドライン
* [Agent Skills クックブック](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction)：例の Skills およびテンプレート
* [SDK のサブエージェント](/docs/ja/agent-sdk/subagents)：プログラマティックオプションを備えた同様のファイルシステムベースのエージェント
* [SDK の概要](/docs/ja/agent-sdk/overview)：一般的な SDK の概念
* [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)：完全な API ドキュメント
* [Python SDK リファレンス](/docs/ja/agent-sdk/python)：完全な API ドキュメント
