> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Agent SDK への移行

> Claude Code TypeScript および Python SDK を Claude Agent SDK に移行するためのガイド

<h2 id="overview">
  概要
</h2>

Claude Code SDK は **Claude Agent SDK** に名前が変更され、ドキュメントが再編成されました。この変更は、コーディングタスクだけでなく、AI エージェント構築のための SDK のより広い機能を反映しています。

OpenAI Agents SDK から移行していますか？[OpenAI Agents SDK 移行レシピ](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk)では、単一の実装例を通じて各プリミティブを Claude Agent SDK にマッピングしています。

<h2 id="what’s-changed">
  変更内容
</h2>

| 項目                | 旧版                          | 新版                                                                 |
| :---------------- | :-------------------------- | :----------------------------------------------------------------- |
| **パッケージ名（TS/JS）** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                   |
| **Python パッケージ**  | `claude-code-sdk`           | `claude-agent-sdk`                                                 |
| **ドキュメント場所**      | Claude Code ドキュメント          | Claude Code ドキュメント → 専用の [Agent SDK](/docs/ja/agent-sdk/overview) セクション |

<h2 id="migration-steps">
  マイグレーションステップ
</h2>

<h3 id="for-typescript/javascript-projects">
  TypeScript/JavaScript プロジェクト向け
</h3>

**1. 古いパッケージをアンインストールします：**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. 新しいパッケージをインストールします：**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. インポートを更新します：**

`@anthropic-ai/claude-code` からのすべてのインポートを `@anthropic-ai/claude-agent-sdk` に変更します：

```typescript theme={null}
// Before
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// After
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. package.json を更新します：**

`@anthropic-ai/claude-code` が `package.json` にまだ記載されている場合は、`@anthropic-ai/claude-agent-sdk` に置き換え、バージョン範囲も更新します。例えば、`"^0.0.42"` から `"^0.3.0"` に更新します。

**5. [破壊的変更](#breaking-changes)を確認します**

マイグレーションを完了するために必要なコード変更を行います。

<h3 id="for-python-projects">
  Python プロジェクト向け
</h3>

**1. 古いパッケージをアンインストールします：**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

古いパッケージがインストールされていない場合、pip は `WARNING: Skipping claude-code-sdk as it is not installed.` と出力します。これは予期された動作であり、次のステップに進むことができます。

**2. 新しいパッケージをインストールします：**

```bash theme={null}
pip install claude-agent-sdk
```

`claude-code-sdk` が `requirements.txt` または `pyproject.toml` に記載されている場合は、`claude-agent-sdk` に置き換えます。

**3. インポートを更新します：**

`claude_code_sdk` からのすべてのインポートを `claude_agent_sdk` に変更します：

```python theme={null}
# Before
from claude_code_sdk import query, ClaudeCodeOptions

# After
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. [破壊的変更](#breaking-changes)を確認します**

マイグレーションを完了するために必要なコード変更を行います。

<h2 id="breaking-changes">
  破壊的変更
</h2>

<Warning>
  分離と明示的な設定を改善するため、Claude Agent SDK v0.1.0 では Claude Code SDK から移行するユーザー向けの破壊的変更が導入されています。
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions が ClaudeAgentOptions に名前変更
</h3>

**変更内容：** Python SDK の型 `ClaudeCodeOptions` が `ClaudeAgentOptions` に名前変更されました。

**移行方法：**

```python theme={null}
# BEFORE (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# AFTER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  システムプロンプトがデフォルトではなくなった
</h3>

**変更内容：** SDK は Claude Code のシステムプロンプトをデフォルトで使用しなくなりました。

**移行方法：**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // BEFORE (v0.0.x) - デフォルトで Claude Code のシステムプロンプトを使用していました
  const before = query({ prompt: "Hello" });

  // AFTER (v0.1.0) - デフォルトで最小限のシステムプロンプトを使用します
  // 以前の動作を取得するには、Claude Code のプリセットを明示的にリクエストしてください：
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // またはカスタムシステムプロンプトを使用します：
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio


  async def main():
      # BEFORE (v0.0.x) - デフォルトで Claude Code のシステムプロンプトを使用していました
      async for message in query(prompt="Hello"):
          print(message)

      # AFTER (v0.1.0) - デフォルトで最小限のシステムプロンプトを使用します
      # 以前の動作を取得するには、Claude Code のプリセットを明示的にリクエストしてください：
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # プリセットを使用
          ),
      ):
          print(message)

      # またはカスタムシステムプロンプトを使用します：
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  設定ソースのデフォルト
</h3>

このデフォルトは v0.1.0 で一時的にファイルシステム設定を読み込まないように変更され、その後元に戻されたため、移行アクションは必要ありません。

**現在の動作：** `query()` で `settingSources` を省略すると、ユーザー、プロジェクト、ローカルファイルシステムの設定が読み込まれ、CLI と一致します。これには `~/.claude/settings.json`、`.claude/settings.json`、`.claude/settings.local.json`、CLAUDE.md ファイル、およびカスタムコマンドが含まれます。

ファイルシステム設定から分離して実行するには、`settingSources: []` を渡すか、Python では `setting_sources=[]` を渡してください。各ソースが読み込む内容については、[settingSources でファイルシステム設定を制御する](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)を参照してください。

分離は、ローカルカスタマイズが漏洩してはいけない CI/CD パイプライン、デプロイされたアプリケーション、テスト環境、マルチテナントシステムで特に重要です。

<Note>
  Python SDK 0.1.59 以前は、空のリストを省略した場合と同じように扱っていたため、`setting_sources=[]` に依存する前にアップグレードしてください。`settingSources` が `[]` の場合でも読み込まれる入力については、[settingSources が制御しないもの](/docs/ja/agent-sdk/claude-code-features#what-settingsources-does-not-control)を参照してください。
</Note>

<h2 id="next-steps">
  次のステップ
</h2>

* [Agent SDK Overview](/docs/ja/agent-sdk/overview) を探索して、利用可能な機能について学びます
* [TypeScript SDK Reference](/docs/ja/agent-sdk/typescript) をチェックして、詳細な API ドキュメントを確認します
* [Python SDK Reference](/docs/ja/agent-sdk/python) を確認して、Python 固有のドキュメントを確認します
* [Custom Tools](/docs/ja/agent-sdk/custom-tools) と [MCP Integration](/docs/ja/agent-sdk/mcp) について学びます
