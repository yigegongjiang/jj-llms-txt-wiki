> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# システムプロンプトの変更

> `claude_code` プリセットとカスタムシステムプロンプトの間で選択し、CLAUDE.md、出力スタイル、append、または完全にカスタムなプロンプトで動作をカスタマイズします。

システムプロンプトは Claude の動作、機能、応答スタイルを定義します。人間が作業を監視して操舵する CLI または IDE のようなコーディングツール向けに `claude_code` プリセットから始めます。異なるサーフェス、アイデンティティ、またはパーミッションモデルを持つエージェント向けに独自のプロンプトを作成します。

<h2 id="how-system-prompts-work">
  システムプロンプトの仕組み
</h2>

システムプロンプトは、会話全体を通じて Claude の動作方法を形作る初期命令セットです。Agent SDK には、これに対する 3 つの開始点があります：

* **最小限のデフォルト**：TypeScript で `systemPrompt` を設定しない、または Python で `system_prompt` を設定しない場合、SDK はツール呼び出しをカバーする最小限のプロンプトを使用しますが、`claude_code` プリセットの残りのコンテンツ（セキュリティと安全性の命令、および作業ディレクトリと環境に関するコンテキストを含む）は省略されています。これは、デフォルトで Claude Code システムプロンプトを使用する `claude -p` とは異なります。CLI から移行していて、一致する動作を望む場合は、`claude_code` プリセットを設定してください。
* **`claude_code` プリセット**：Claude Code CLI が使用するシステムプロンプト。ツール使用命令、セキュリティと安全性の命令、および作業ディレクトリと環境に関するコンテキストが含まれています。TypeScript で `systemPrompt: { type: "preset", preset: "claude_code" }` を設定するか、Python で `system_prompt={"type": "preset", "preset": "claude_code"}` を設定してください。オプションで `append` を使用して、最後に独自の命令を追加できます。
* **カスタム文字列**：自分で作成したプロンプト。SDK は提供したものだけを送信します。

<h3 id="decide-on-a-starting-point">
  開始点を決定する
</h3>

決定要因は、エージェントが Claude Code にどの程度似ているかです：リポジトリで動作するコーディングエージェント。人間がストリーミング出力を監視して作業を指導します。製品がそれから遠いほど、独自のプロンプトを作成する必要があります。

| 構築しているもの                                                       | 使用するもの                           | 得られるもの                                                  |
| :------------------------------------------------------------- | :------------------------------- | :------------------------------------------------------ |
| 人間が監視して指導する CLI または IDE のようなコーディングツール。Claude Code のデフォルトが必要なもの | `claude_code` プリセット              | Claude Code プロンプト（ツールガイダンス、安全ルール、環境コンテキストを含む）           |
| 同じ種類のツール、プラス、コーディング標準、出力形式、またはドメインコンテキストなどの製品固有のルール            | `claude_code` プリセット（`append` 付き） | 上記のすべて。プリセットの後に命令が追加されます。何も削除されないため、これは最もリスクが低いカスタマイズです |
| 異なるサーフェス、アイデンティティ、または権限モデルを持つエージェント、またはコーディング以外のエージェント         | カスタムプロンプト文字列                     | 作成したもののみ。エージェントが必要とするツールガイダンスと安全命令を置き換える責任があります         |
| ツール呼び出しループが薄く、エージェントペルソナがなく、ユーザープロンプトですべての動作を提供する              | `systemPrompt` オプションなし           | 最小限のデフォルト：ツール呼び出しサポートのみ                                 |

「Claude Code と異なる」は通常、以下のいずれかを意味します：

* **異なるサーフェス**：出力は、それをトリガーした人によってターミナルで読まれません。チャット UI、構造化出力コンシューマー、およびコーディング以外の自動化は、それぞれ、出力がどのようにレンダリングおよびレビューされるかに一致するプロンプトが必要です。CI ジョブがリントエラーを修正したり、diff をレビューしたりするような無人コーディング自動化は、作業自体がプリセットが書かれているものであるため、プリセットに適合します。
* **異なるアイデンティティ**：エージェントは Claude Code として自分自身を提示すべきではありません。サポートボット、データ分析アシスタント、またはドメイン固有のエージェントは、独自の名前、スコープ、およびペルソナが必要です。
* **異なる権限モデル**：エージェントは人間が各ステップを承認することなく自律的に実行されるか、リソースの限定されたセットで動作します。Claude Code のプロンプトは、人間がループ内にいて、完全なツールセットにアクセスできることを前提としています。
* **コーディング以外のタスク**：Claude Code のプロンプトのほとんどはコーディングガイダンスです。研究、コンテンツ、または運用エージェントの場合、そのガイダンスは実際に必要な命令と競合します。

[比較表](#compare-the-four-approaches)は、各カスタマイズ方法が何を保持するかを示しています。

<h2 id="customize-agent-behavior">
  エージェントの動作をカスタマイズする
</h2>

`append` とカスタムプロンプト文字列はそれぞれシステムプロンプトを直接変更し、出力スタイルは Claude Code がすべてのレスポンスに対して Claude に与える指示を変更します。CLAUDE.md は異なるパスを取ります。SDK がそれを読み込み、その内容をプロジェクトコンテキストとして会話に注入するため、選択したシステムプロンプトと一緒に動作を形作ります。[Skills](/docs/ja/agent-sdk/skills)、[hooks](/docs/ja/agent-sdk/hooks)、および [permissions](/docs/ja/agent-sdk/permissions) もシステムプロンプト外で動作を形作り、独自のページで説明されています。

<h3 id="claude-md-files-for-project-level-instructions">
  プロジェクトレベルの指示のための CLAUDE.md ファイル
</h3>

CLAUDE.md ファイルは Claude に永続的なプロジェクトコンテキストと指示を提供します。SDK はその内容を会話に注入し、システムプロンプトはそのままにするため、どのシステムプロンプト設定でも機能します。CLAUDE.md に何を入れるか、どこに配置するか、効果的な指示を書く方法については、[When to add to CLAUDE.md](/docs/ja/memory#when-to-add-to-claude-md) および [How Claude remembers your project](/docs/ja/memory) の残りの部分を参照してください。このセクションでは SDK に固有のもの、つまり CLAUDE.md がどのように読み込まれるかについて説明します。

SDK は、一致する設定ソースが有効な場合に CLAUDE.md を読み込みます。`'project'` はワーキングディレクトリから `CLAUDE.md` または `.claude/CLAUDE.md` を読み込み、`'user'` は `~/.claude/CLAUDE.md` を読み込みます。デフォルトの `query()` オプションは両方のソースを有効にするため、CLAUDE.md は自動的に読み込まれます。TypeScript で `settingSources` または Python で `setting_sources` を明示的に設定する場合は、必要なソースを含めてください。CLAUDE.md の読み込みは設定ソースによって制御され、`claude_code` プリセットによっては制御されません。

<h4 id="load-claude-md-with-the-sdk">
  SDK で CLAUDE.md を読み込む
</h4>

CLAUDE.md を読み込むには、`settingSources` を CLAUDE.md を保持するレベルを含むように設定します。以下の例は、`claude_code` プリセットと一緒にプロジェクトレベルの CLAUDE.md を読み込むため、Claude はコーディングエージェントプロンプトとプロジェクトの規約の両方を持ちます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
      async for message in query(
          prompt="Add a new React component for user profiles",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",  # Use Claude Code's system prompt
              },
              setting_sources=["project"],  # Loads CLAUDE.md from project
          ),
      ):
          messages.append(message)


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

いずれかの例を実行すると、SDK は Claude が作業する際にメッセージをストリーミングします。システム初期化メッセージ、アシスタントメッセージ、ツール結果を含むユーザーメッセージ、およびセッション結果を含む最終結果メッセージです。

CLAUDE.md はプロジェクト内のすべてのセッションで永続的であり、git を通じてチームと共有され、コード変更なしで自動的に検出されます。空の `settingSources` 配列を渡す場合は読み込まれません。

<h3 id="output-styles-for-persistent-configurations">
  永続的な設定のための出力スタイル
</h3>

出力スタイルは、Claude のロール、トーン、および出力形式を変更する保存された指示セットです。マークダウンファイルとして保存され、セッションとプロジェクト全体で再利用できます。

<h4 id="create-an-output-style">
  出力スタイルを作成する
</h4>

出力スタイルは、メタデータの [frontmatter](/docs/ja/output-styles#frontmatter) の後にプロンプトコンテンツが続くマークダウンファイルです。すべてのプロジェクトで利用可能なユーザーレベルのスタイルの場合は `~/.claude/output-styles/` に保存し、リポジトリ内のプロジェクトレベルのスタイルの場合は `.claude/output-styles/` に保存してチームと共有できます。

カスタム出力スタイルは `claude_code` プリセットのソフトウェアエンジニアリング指示を除外し、独自のものを使用します。それらを保持し、指示を上に重ねるには、frontmatter で `keep-coding-instructions: true` を設定します。これらの指示は Claude Code の完全なシステムプロンプトにのみあるため、この設定は [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/ja/env-vars#variables) でオンまたはオフにピン留めする短いシステムプロンプトのセッションには影響を与えません。エージェントがまだソフトウェアエンジニアリング作業を行っている場合は保持します。ロール全体を置き換える場合は除外します。

以下の例は、コーディング指示を保持するコードレビュー担当者のペルソナを定義します。コードレビューは Claude Code のセキュリティとコード品質ガイダンスから引き続き利益を得るためです。`~/.claude/output-styles/code-reviewer.md` として保存して、プロジェクト全体で利用可能にします。

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  出力スタイルを有効化する
</h4>

作成後、出力スタイルは以下を通じて有効化します。

* **CLI**: `/output-style <style>` を実行します。例えば `/output-style concise` を実行するか、`/config` を実行してスタイルを選択します。`/output-style` コマンドには Claude Code v2.1.269 以降が必要です。
* **Settings**: `.claude/settings.local.json` で `outputStyle` を設定します。
* **TypeScript SDK**: `query()` に渡されるインライン `settings` オブジェクト内で `outputStyle` を設定するか、`settings` を `outputStyle` を設定する設定ファイルにポイントします。`outputStyle` はトップレベルの `Options` フィールドではありません。

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Python SDK では、JSON 文字列（例：`'{"outputStyle": "Explanatory"}'`）または `outputStyle` を設定する設定ファイルへのパスを取る `settings` オプションを通じて `outputStyle` を設定します。

**SDK ユーザーへの注意:** 出力スタイルは、オプションに `settingSources: ['user']` または `settingSources: ['project']`（TypeScript）/ `setting_sources=["user"]` または `setting_sources=["project"]`（Python）を含める場合に読み込まれます。

<h3 id="append-to-the-claude_code-preset">
  `claude_code` プリセットに追加する
</h3>

Claude Code プリセットを `append` プロパティと共に使用して、すべての組み込み機能を保持しながらカスタム指示を追加できます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
      async for message in query(
          prompt="Help me write a Python function to calculate fibonacci numbers",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "Always include detailed docstrings and type hints in Python code.",
              }
          ),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  ユーザーとマシン全体でプロンプトキャッシングを改善する
</h4>

デフォルトでは、同じ `claude_code` プリセットと `append` テキストを使用する 2 つのセッションは、異なるワーキングディレクトリから実行される場合でも、プロンプトキャッシュエントリを共有できません。これは、プリセットが `append` テキストの前にシステムプロンプトにセッションごとのコンテキストを埋め込むためです。ワーキングディレクトリ、git リポジトリであるかどうか、プラットフォーム、アクティブなシェル、OS バージョン、および自動メモリパスです。そのコンテキストの違いはシステムプロンプトの違いを生じ、キャッシュミスになります。CLAUDE.md コンテンツはシステムプロンプトに影響を与えません。SDK がそれをシステムプロンプトではなく会話に注入するためです。

セッション全体でシステムプロンプトを同一にするには、TypeScript で `excludeDynamicSections: true` を設定するか、Python で `"exclude_dynamic_sections": True` を設定します。セッションごとのコンテキストは最初のユーザーメッセージに移動し、静的プリセットと `append` テキストのみがシステムプロンプトに残るため、同一の設定はユーザーとマシン全体でキャッシュエントリを共有できます。

<Note>
  `excludeDynamicSections` には `@anthropic-ai/claude-agent-sdk` v0.2.98 以降、または Python の場合は `claude-agent-sdk` v0.1.58 以降が必要です。プリセットオブジェクト形式でのみ設定します。SDK はカスタムプロンプトの代わりにプリセットを渡す場合、それを無視します。カスタムプロンプトの指示を TypeScript SDK でキャッシュに保つには、[Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt) を参照してください。
</Note>

次の例は、共有 `append` ブロックを `excludeDynamicSections` と組み合わせるため、異なるディレクトリから実行されるエージェントのフリートが同じキャッシュされたシステムプロンプトを再利用できます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Triage the open issues in this repo",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
                  "exclude_dynamic_sections": True,
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

**トレードオフ:** ワーキングディレクトリ、git リポジトリフラグ、プラットフォーム、アクティブなシェル、OS バージョン、および自動メモリパスは引き続き Claude に到達しますが、システムプロンプトではなく最初のユーザーメッセージの一部として到達します。ユーザーメッセージの指示はシステムプロンプトの同じテキストよりもわずかに低い重みを持つため、Claude は現在のディレクトリまたは自動メモリパスについて推論する際にそれらに依存する可能性が低くなります。セッション間のキャッシュ再利用が最大限に権威あるコンテキストよりも重要な場合、このオプションを有効にします。

非対話型 CLI モードの同等のフラグについては、[`--exclude-dynamic-system-prompt-sections`](/docs/ja/cli-reference) を参照してください。

<h3 id="custom-system-prompts">
  カスタムシステムプロンプト
</h3>

カスタム文字列を `systemPrompt` として提供して、デフォルトを独自の指示で完全に置き換えることができます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

Python では、文字列として渡す代わりに `system_prompt={"type": "file", "path": "..."}` を使用してファイルから大きなカスタムプロンプトを読み込みます。Python SDK は文字列プロンプトを CLI サブプロセスへの 1 つのコマンドライン引数として渡すため、OS 引数長制限を超えるプロンプトはプロセス生成時に失敗し、API リクエストが送信される前に失敗します。Linux ではエラーは `Argument list too long` です。プラットフォームのしきい値と Windows の動作については、[`SystemPromptFile`](/docs/ja/agent-sdk/python#systempromptfile) を参照してください。

<h4 id="cache-the-static-part-of-a-custom-prompt">
  カスタムプロンプトの静的部分をキャッシュする
</h4>

TypeScript SDK では、カスタムプロンプトを 1 つの文字列ではなく文字列の配列として渡すことができます。静的部分と残りの部分の間に `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` マーカーを付けます。プロンプトが、すべてのリクエストで同じ指示とリクエストごとに変わるコンテキスト（エージェントが処理しているカスタマーまたはチケットなど）を組み合わせる場合に使用します。両方の部分を 1 つの文字列として渡す場合、リクエストごとの部分への変更は全体のシステムプロンプトを変更するため、静的指示もキャッシュを逃します。配列形式は Python SDK では利用できません。[`ClaudeAgentOptions`](/docs/ja/agent-sdk/python#claudeagentoptions) は `system_prompt` が受け入れる形式をリストします。

<Note>
  SDK は Claude API を直接呼び出すか、[Claude Platform on AWS](/docs/ja/claude-platform-on-aws) で実行する場合にのみプロンプトを分割します。Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、または [LLM gateway](/docs/ja/llm-gateway-connect) など、その他のすべての設定では、[`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ja/llm-gateway-protocol#disable-pre-release-capabilities) を設定するたびに、SDK は全体のプロンプトを 1 つのブロックとして送信します。これは 1 つの文字列を渡すのと同じです。
</Note>

プロンプトを分割するには、`@anthropic-ai/claude-agent-sdk` から `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` をインポートし、2 つの部分の間の配列要素として渡します。SDK はマーカーの前の文字列を 1 つのテキストブロックとして送信し、その後の文字列を 2 番目のブロックとして送信します。各ブロックは独自のキャッシュブレークポイントを持ちます。以下の例では、サポートエージェントがトリアージ指示をファイルから読み込み、各リクエストで 1 つのチケットの詳細を受け取るため、指示はキャッシュされたままでチケットの詳細が変わります。

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/ja/agent-sdk/cost-tracking#track-cache-tokens) は各結果メッセージの `cache_creation_input_tokens` および `cache_read_input_tokens` フィールドについて説明します。

SDK は配列から次のようにブロックを組み立てます。

* SDK はマーカーの各側の文字列をそれらの間に空行を入れて結合し、マーカー自体を削除するため、マーカーテキストは Claude に到達しません。
* マーカーを複数回含める場合、最初のものが分割であり、SDK は他のものを削除します。
* マーカーを除外する場合、SDK はすべての文字列を 1 つのブロックに結合します。これは 1 つの文字列を渡すのと同じです。

CLI の [`--system-prompt` または `--system-prompt-file` フラグ](/docs/ja/cli-reference#system-prompt-flags) では、プロンプトは 1 つの文字列であるため、マーカーを含む配列はありません。静的部分とリクエストごとの部分の間に `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` のみを含む行を含めます。Claude Code はプロンプトを最初のそのような行で同じ 2 つのブロックに分割し、その行を削除します。Claude Code v2.1.275 以降が必要です。

SDK では、配列形式を推奨します。これはマーカー行なしで境界を含みます。

<h3 id="change-the-prompt-of-an-existing-session">
  既存のセッションのプロンプトを変更する
</h3>

デフォルトでは、`resume` または `continue` でセッションに戻るときに異なる `append` またはカスタムプロンプトを渡す場合、Claude は次のターンでそれを見ません。Claude Code はセッションの最初のリクエストでシステムプロンプトを記録し、セッションがコンパクト化されるまでそのレコードを再利用します。新しいテキストはその後のコンパクト化後、または新しいセッションで有効になります。

<h4 id="update-claude’s-instructions-mid-session">
  セッション中に Claude の指示を更新する
</h4>

システムプロンプトに入れた指示がセッション実行中に変わる必要がある場合（例えば、ユーザーがエージェントを読み取り専用モードに切り替えたり、アプリで設定を編集したりした場合）、`systemPrompt` を変更する代わりに会話で新しい指示を送信します。

* **次のメッセージで**: 送信する次のユーザーメッセージに新しい指示を含めます。
* **フックから**: `UserPromptSubmit` または `PostToolUse` [hook callback](/docs/ja/agent-sdk/hooks#outputs) から [`additionalContext`](/docs/ja/hooks#add-context-for-claude) を返します。「ワークスペースは読み取り専用になりました」などの事実上の声明として書かれています。SDK はフックが発火した時点で会話にテキストを挿入するため、記録されたプロンプトは変わりません。

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  単語遣いを反復処理する際に記録をオフにする
</h4>

プロンプト単語遣いを反復処理し、各編集が再開するセッションに到達するようにしたい場合、システムプロンプトのオブジェクト形式で `snapshot` を false に設定します。Claude Code はすべてのリクエストでプロンプトを再構築します。このフィールドは TypeScript の [`systemPrompt`](/docs/ja/agent-sdk/typescript#options) のプリセットおよびカスタム形式、および Python の [`system_prompt`](/docs/ja/agent-sdk/python#systempromptpreset) で利用可能であり、`@anthropic-ai/claude-agent-sdk` v0.3.257 以降、または `claude-agent-sdk` v0.2.153 以降が必要です。

本番環境では記録をオンのままにしてください。記録がオフの場合、再開されたセッションの異なる `append` またはカスタムプロンプトは次のターンで Claude に到達し、そのリクエストはセッションの [prompt cache](/docs/ja/prompt-caching#how-the-cache-is-organized) を再利用できません。API が [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) を強制する場合、Claude は以前のターンからの思考も失います。

[cloud sessions](/docs/ja/cloud-environments) の外で、`extraArgs` を通じて `--bare` を渡すか、`CLAUDE_CODE_SIMPLE=1` を設定することで Claude Code を [bare mode](/docs/ja/headless#start-faster-with-bare-mode) で開始する場合、`snapshot: true` を設定しない限り、記録はオフのままです。

`append` またはカスタムプロンプトを記録するには、デフォルトで Claude Code v2.1.265 以降が必要です。TypeScript Agent SDK は v0.3.265 から、Python Agent SDK は v0.2.153 からバンドルされています。Claude Code v2.1.268 より前では、[feature flags](/docs/ja/env-vars#features-that-need-feature-flag-fetching) をフェッチしないセッション（Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry のセッションを含む）はすべてのリクエストでプロンプトを再構築し、`snapshot` は効果がありませんでした。

<h2 id="compare-the-four-approaches">
  4 つのアプローチすべての比較
</h2>

4 つのカスタマイズ方法は、どこに存在するか、どのように共有されるか、および `claude_code` プリセットから何を保持するかが異なります。

| 機能             | CLAUDE.md     | 出力スタイル        | `systemPrompt` を追加 | カスタム `systemPrompt` |
| -------------- | ------------- | ------------- | ------------------ | ------------------- |
| **永続性**        | プロジェクトごとのファイル | ファイルとして保存     | セッションのみ            | セッションのみ             |
| **再利用性**       | プロジェクトごと      | プロジェクト全体      | コード重複              | コード重複               |
| **管理**         | ファイルシステム上     | CLI + ファイル    | コード内               | コード内                |
| **デフォルトツール**   | 保持            | 保持            | 保持                 | 失われる（含まれない限り）       |
| **組み込みセキュリティ** | 維持            | 維持            | 維持                 | 追加する必要がある           |
| **環境コンテキスト**   | 自動            | 自動            | 自動                 | 提供する必要がある           |
| **カスタマイズレベル**  | 追加のみ          | デフォルトを置き換え    | 追加のみ               | 完全な制御               |
| **バージョン管理**    | プロジェクトと共に     | はい            | コードと共に             | コードと共に              |
| **スコープ**       | プロジェクト固有      | ユーザーまたはプロジェクト | コードセッション           | コードセッション            |

「追加を使用」は TypeScript で `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` を使用するか、Python で `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` を使用することを意味します。CLAUDE.md はシステムプロンプト自体を変更しません。SDK はそのコンテンツをプロジェクトコンテキストとして会話に注入します。

<h2 id="combine-approaches">
  アプローチを組み合わせる
</h2>

これらのアプローチは組み合わせることができます。永続的な出力スタイルまたは CLAUDE.md は長期的な動作を設定し、`append` はセッション固有の指示を保存された設定に触れることなく上に重ねます。

<h3 id="combine-an-output-style-with-session-specific-additions">
  出力スタイルとセッション固有の追加を組み合わせる
</h3>

以下の例は、Code Reviewer 出力スタイルが既にアクティブであることを想定しています。`append` ブロックはセッション固有のフォーカス領域をペルソナの上に重ねるため、単一のレビュー セッションで保存された出力スタイルを変更することなく OAuth とトークン ストレージを優先できます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // "Code Reviewer" 出力スタイルがアクティブであると仮定（/config または settings 経由）
  // セッション固有のフォーカス領域を追加
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # "Code Reviewer" 出力スタイルがアクティブであると仮定（/config または settings 経由）
  # セッション固有のフォーカス領域を追加
  messages = []


  async def main():
      async for message in query(
          prompt="Review this authentication module",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": """
                  For this review, prioritize:
                  - OAuth 2.0 compliance
                  - Token storage security
                  - Session management
                  """,
              }
          ),
      ):
          messages.append(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="see-also">
  関連項目
</h2>

* [出力スタイル](/docs/ja/output-styles)：CLI の出力スタイルを作成、管理、共有します。ファイル形式とストレージの場所を含みます
* [Claude がプロジェクトを記憶する方法](/docs/ja/memory)：CLAUDE.md に何を入れるか、どこに配置するか、効果的なプロジェクト指示の書き方
* [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)：`systemPrompt`、`settingSources`、`settings` を含む完全な `Options` 型
* [Python SDK リファレンス](/docs/ja/agent-sdk/python)：`system_prompt` と `setting_sources` を含む完全な `ClaudeAgentOptions` 型
* [設定](/docs/ja/settings)：出力スタイルおよび他の設定が保存される場所を含む `settings.json` リファレンス
