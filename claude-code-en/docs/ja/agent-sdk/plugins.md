> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK のプラグイン

> Agent SDK を通じてカスタムプラグインを読み込み、スキル、エージェント、フック、MCP サーバーで Claude Code を拡張します

プラグインを使用すると、Claude Code をカスタム機能で拡張でき、プロジェクト全体で共有できます。Agent SDK を通じて、ローカルディレクトリからプログラムでプラグインを読み込み、エージェントセッションに機能を追加できます。プラグインには以下を含めることができます：

* **Skills**: Claude が関連する場合に自律的に呼び出す機能。`/plugin-name:skill-name` でプラグインスキルを直接呼び出すこともできます。
* **Agents**: 特定のタスク用の専門的なサブエージェント
* **Hooks**: ツール使用およびその他のイベントに応答するイベントハンドラー
* **MCP servers**: Model Context Protocol 経由の外部ツール統合

プラグイン構造とプラグインの作成方法に関する完全な情報については、[Plugins](/docs/ja/plugins/overview) を参照してください。

<h2 id="loading-plugins">
  プラグインの読み込み
</h2>

オプション設定でローカルファイルシステムパスを指定してプラグインを読み込みます。`type` フィールドは `"local"` である必要があります。これは SDK が受け入れる唯一の値です。SDK は複数の場所から複数のプラグインを読み込むことをサポートしています。

[マーケットプレイス](/docs/ja/plugins/overview)またはリモートリポジトリを通じて配布されているプラグインを使用するには、まずダウンロードしてローカルディレクトリパスを指定してください。プラグインが必要とするディレクトリレイアウトについては、以下の[プラグイン構造リファレンス](#plugin-structure-reference)を参照してください。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello",
    options: {
      plugins: [
        { type: "local", path: "./my-plugin" },
        { type: "local", path: "/absolute/path/to/another-plugin" }
      ]
    }
  })) {
    // Plugin commands, agents, and other features are now available
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              plugins=[
                  {"type": "local", "path": "./my-plugin"},
                  {"type": "local", "path": "/absolute/path/to/another-plugin"},
              ]
          ),
      ):
          # Plugin commands, agents, and other features are now available
          pass


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="path-specifications">
  パス指定
</h3>

プラグインパスは以下のいずれかです：

* **相対パス**: `cwd` オプションを基準に解決されます（例：`"./plugins/my-plugin"`）
* **絶対パス**: 完全なファイルシステムパス（例：`"/home/user/plugins/my-plugin"`）

<Note>
  パスはプラグインのルートディレクトリ（`skills/`、`agents/`、`hooks/`、`commands/`、または `.claude-plugin/` の親ディレクトリ）を指す必要があります。
</Note>

<h2 id="verifying-plugin-installation">
  プラグインインストールの確認
</h2>

プラグインが正常に読み込まれると、システム初期化メッセージに表示されます。プラグインが利用可能であることを確認できます：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello",
    options: {
      plugins: [{ type: "local", path: "./my-plugin" }]
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      // Check loaded plugins
      console.log("Plugins:", message.plugins);
      // Example: [{ name: "my-plugin", path: "/absolute/path/to/my-plugin" }]

      // Plugin skills appear with the plugin name as a prefix
      console.log("Skills:", message.skills);
      // Example: ["my-plugin:greet"]

      // Plugin commands use the same prefix, and skills appear here too
      console.log("Commands:", message.slash_commands);
      // Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./my-plugin"}]
          ),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              # Check loaded plugins
              print("Plugins:", message.data.get("plugins"))
              # Example: [{"name": "my-plugin", "path": "/absolute/path/to/my-plugin"}]

              # Plugin skills appear with the plugin name as a prefix
              print("Skills:", message.data.get("skills"))
              # Example: ["my-plugin:greet"]

              # Plugin commands use the same prefix, and skills appear here too
              print("Commands:", message.data.get("slash_commands"))
              # Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="use-plugin-skills">
  プラグインスキルの使用
</h2>

プラグインのスキルは競合を避けるためにプラグイン名で自動的に名前空間化されます。直接呼び出すには、プロンプトとして `/plugin-name:skill-name` を送信してください。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Load a plugin with a custom /greet skill
  for await (const message of query({
    prompt: "/my-plugin:greet", // Use plugin skill with namespace
    options: {
      plugins: [{ type: "local", path: "./my-plugin" }]
    }
  })) {
    // Claude executes the custom greeting skill from the plugin
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, TextBlock


  async def main():
      # Load a plugin with a custom /greet skill
      async for message in query(
          prompt="/my-plugin:greet",  # Use plugin skill with namespace
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./my-plugin"}]
          ),
      ):
          # Claude executes the custom greeting skill from the plugin
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if isinstance(block, TextBlock):
                      print(f"Claude: {block.text}")


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  CLI 経由でプラグインをインストールした場合（例：`/plugin install my-plugin@marketplace`）、SDK でそのインストールパスを指定することで引き続き使用できます。CLI でインストールされたプラグインについては `~/.claude/plugins/` を確認してください。
</Note>

<h2 id="complete-example">
  完全な例
</h2>

プラグインの読み込みと使用を示す完全な例を以下に示します：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import { fileURLToPath } from "node:url";

  async function runWithPlugin() {
    const pluginPath = fileURLToPath(new URL("./plugins/my-plugin", import.meta.url));

    console.log("Loading plugin from:", pluginPath);

    for await (const message of query({
      prompt: "What custom commands do you have available?",
      options: {
        plugins: [{ type: "local", path: pluginPath }],
        maxTurns: 3
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        console.log("Loaded plugins:", message.plugins);
        console.log("Available skills:", message.skills);
        console.log("Available commands:", message.slash_commands);
      }

      if (message.type === "assistant") {
        console.log("Assistant:", message.message.content);
      }
    }
  }

  runWithPlugin().catch(console.error);
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  """Example demonstrating how to use plugins with the Agent SDK."""

  import asyncio
  from pathlib import Path

  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeAgentOptions,
      SystemMessage,
      TextBlock,
      query,
  )


  async def run_with_plugin():
      """Example using a custom plugin."""
      plugin_path = Path(__file__).parent / "plugins" / "my-plugin"

      print(f"Loading plugin from: {plugin_path}")

      options = ClaudeAgentOptions(
          plugins=[{"type": "local", "path": str(plugin_path)}],
          max_turns=3,
      )

      async for message in query(
          prompt="What custom commands do you have available?", options=options
      ):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print(f"Loaded plugins: {message.data.get('plugins')}")
              print(f"Available skills: {message.data.get('skills')}")
              print(f"Available commands: {message.data.get('slash_commands')}")

          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if isinstance(block, TextBlock):
                      print(f"Assistant: {block.text}")


  if __name__ == "__main__":
      asyncio.run(run_with_plugin())
  ```
</CodeGroup>

<h2 id="plugin-structure-reference">
  プラグイン構造リファレンス
</h2>

プラグインディレクトリには通常、`.claude-plugin/plugin.json` マニフェストファイルが含まれています。マニフェストはオプションです。省略した場合、Claude Code はディレクトリレイアウトからコンポーネントを自動検出します。ディレクトリには以下を含めることができます：

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest (optional, components auto-discovered without it)
├── skills/                   # Agent Skills (invoked autonomously or via /plugin-name:skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Skills as flat .md files
│   └── custom-cmd.md
├── agents/                   # Custom agents
│   └── specialist.md
├── hooks/                    # Event handlers
│   └── hooks.json
└── .mcp.json                # MCP server definitions
```

<Note>
  `commands/` ディレクトリはスキルをフラットな Markdown ファイルとして保持しています。新しいプラグインには `skills/` を使用してください。Claude Code は両方の場所をサポートしています。
</Note>

<h2 id="multiple-plugin-sources">
  複数のプラグインソース
</h2>

異なる場所からプラグインを組み合わせます：

```typescript theme={null}
import * as os from "node:os";
import * as path from "node:path";

plugins: [
  { type: "local", path: "./local-plugin" },
  {
    type: "local",
    path: path.join(os.homedir(), ".claude", "custom-plugins", "shared-plugin")
  }
];
```

<Note>
  SDK はチルダパス（`~/plugins` など）を展開しません。プラグインパスが存在しない場合、SDK はそのプラグインをスキップしてセッションは続行されるため、初期化メッセージの `plugins` リストを確認して各プラグインが読み込まれたことを確認してください。
</Note>

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="plugin-not-loading">
  プラグインが読み込まれない
</h3>

プラグインが初期化メッセージに表示されない場合：

1. **パスを確認する**: パスがプラグインルートディレクトリ（`skills/`、`agents/`、`hooks/`、`commands/`、または `.claude-plugin/` の親）を指していることを確認してください
2. **plugin.json を検証する**: プラグインにマニフェストが含まれている場合、有効な JSON 構文を持っていることを確認してください
3. **ファイルパーミッションを確認する**: プラグインディレクトリが読み取り可能であることを確認してください
4. **ディレクトリが存在することを確認する**: SDK は存在しないパスをスキップし、プラグインは初期化メッセージの `plugins` リストに表示されません

<h3 id="skills-not-appearing">
  スキルが表示されない
</h3>

プラグインスキルが機能しない場合：

1. **名前空間を使用する**: `/plugin-name:skill-name` としてプラグインスキルを呼び出してください
2. **初期化メッセージを確認する**: スキルが正しい名前空間で `skills` リストに表示されることを確認してください
3. **スキルファイルを検証する**: 各スキルが `skills/` の下の独自のサブディレクトリに `SKILL.md` ファイルを持っていることを確認してください（例：`skills/my-skill/SKILL.md`）

<h2 id="see-also">
  関連項目
</h2>

* [Plugins](/docs/ja/plugins/overview) - プラグイン開発の完全ガイド
* [Plugins reference](/docs/ja/plugins/manifest-reference) - 技術仕様
* [Commands](/docs/ja/agent-sdk/skills#dispatch-commands-by-name) - SDK でのコマンドのディスパッチ
* [Subagents](/docs/ja/agent-sdk/subagents) - 専門的なエージェントの操作
* [Skills](/docs/ja/agent-sdk/skills) - Agent Skills の使用
