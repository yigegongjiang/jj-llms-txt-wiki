> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK 中的 Plugins

> 通過 Agent SDK 加載自訂 plugins，以使用 skills、agents、hooks 和 MCP servers 擴展 Claude Code

Plugins 允許您使用可在專案間共享的自訂功能來擴展 Claude Code。通過 Agent SDK，您可以以程式方式從本地目錄加載 plugins，以將 skills、agents、hooks 和 MCP servers 添加到您的 agent sessions。

<h2 id="what-are-plugins">
  什麼是 plugins？
</h2>

Plugins 是 Claude Code 擴展的套件，可以包括：

* **Skills**：Claude 自主使用的模型調用功能（也可以使用 `/skill-name` 調用）
* **Agents**：用於特定任務的專門子 agents
* **Hooks**：響應工具使用和其他事件的事件處理程序
* **MCP servers**：通過 Model Context Protocol 的外部工具集成

<Note>
  `commands/` 目錄是舊版格式。對於新 plugins，請使用 `skills/`。Claude Code 繼續支持兩種格式以實現向後相容性。
</Note>

有關 plugin 結構和如何創建 plugins 的完整信息，請參閱 [Plugins](/docs/zh-TW/plugins)。

<h2 id="loading-plugins">
  加載 plugins
</h2>

通過在選項配置中提供本地文件系統路徑來加載 plugins。`type` 字段必須是 `"local"`，這是 SDK 接受的唯一值。要使用通過 [marketplace](/docs/zh-TW/plugin-marketplaces) 或遠程存儲庫分發的 plugin，請先下載它並提供本地目錄路徑。SDK 支持從不同位置加載多個 plugins。

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
  路徑規範
</h3>

Plugin 路徑可以是：

* **相對路徑**：相對於您的當前工作目錄解析（例如，`"./plugins/my-plugin"`）
* **絕對路徑**：完整文件系統路徑（例如，`"/home/user/plugins/my-plugin"`）

<Note>
  路徑應指向 plugin 的根目錄：`skills/`、`agents/`、`hooks/`、`commands/`（舊版）或 `.claude-plugin/` 的父目錄，而不是子目錄。
</Note>

<h2 id="verifying-plugin-installation">
  驗證 plugin 安裝
</h2>

當 plugins 成功加載時，它們會出現在系統初始化消息中。您可以驗證您的 plugins 是否可用：

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
      // 檢查已加載的 plugins
      console.log("Plugins:", message.plugins);
      // 範例：[{ name: "my-plugin", path: "./my-plugin" }]

      // Plugin skills 會以 plugin 名稱作為前綴出現
      console.log("Skills:", message.skills);
      // 範例：["my-plugin:greet"]

      // Plugin 命令使用相同的前綴，skills 也會出現在這裡
      console.log("Commands:", message.slash_commands);
      // 範例：["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
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
              # 檢查已加載的 plugins
              print("Plugins:", message.data.get("plugins"))
              # 範例：[{"name": "my-plugin", "path": "./my-plugin"}]

              # Plugin skills 會以 plugin 名稱作為前綴出現
              print("Skills:", message.data.get("skills"))
              # 範例：["my-plugin:greet"]

              # Plugin 命令使用相同的前綴，skills 也會出現在這裡
              print("Commands:", message.data.get("slash_commands"))
              # 範例：["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
  使用 plugin skills
</h2>

來自 plugins 的 skills 會自動使用 plugin 名稱進行命名空間化，以避免衝突。若要直接調用，請在提示中發送 `/plugin-name:skill-name`。

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
          prompt="/demo-plugin:greet",  # Use plugin skill with namespace
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./plugins/demo-plugin"}]
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
  如果您通過 CLI 安裝了 plugin（例如，`/plugin install my-plugin@marketplace`），您仍然可以通過提供其安裝路徑在 SDK 中使用它。檢查 `~/.claude/plugins/` 以查找 CLI 安裝的 plugins。
</Note>

<h2 id="complete-example">
  完整示例
</h2>

以下是演示 plugin 載入和使用的完整示例：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as path from "path";

  async function runWithPlugin() {
    const pluginPath = path.join(__dirname, "plugins", "my-plugin");

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

  from pathlib import Path
  import anyio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeAgentOptions,
      SystemMessage,
      TextBlock,
      query,
  )


  async def run_with_plugin():
      """Example using a custom plugin."""
      plugin_path = Path(__file__).parent / "plugins" / "demo-plugin"

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
      anyio.run(run_with_plugin)
  ```
</CodeGroup>

<h2 id="plugin-structure-reference">
  Plugin 結構參考
</h2>

Plugin 目錄通常包含 `.claude-plugin/plugin.json` 清單文件。清單是可選的。省略時，Claude Code 會從目錄佈局自動發現組件。目錄可以包括：

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin 清單（可選，沒有它也會自動發現組件）
├── skills/                   # Agent Skills（自主調用或通過 /skill-name）
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # 舊版：改用 skills/ 代替
│   └── custom-cmd.md
├── agents/                   # 自訂代理
│   └── specialist.md
├── hooks/                    # 事件處理程序
│   └── hooks.json
└── .mcp.json                # MCP 伺服器定義
```

有關創建 plugins 的詳細信息，請參閱：

* [Plugins](/docs/zh-TW/plugins) - 完整的 plugin 開發指南
* [Plugins reference](/docs/zh-TW/plugins-reference) - 技術規範和架構

<h2 id="common-use-cases">
  常見用例
</h2>

<h3 id="development-and-testing">
  開發和測試
</h3>

在開發期間加載 plugins，無需全局安裝它們：

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  專案特定的擴展
</h3>

在您的專案存儲庫中包含 plugins，以實現團隊範圍的一致性：

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  多個 plugin 來源
</h3>

結合來自不同位置的 plugins：

```typescript theme={null}
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
];
```

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="plugin-not-loading">
  Plugin 未加載
</h3>

如果您的 plugin 未出現在初始化消息中：

1. **檢查路徑**：確保路徑指向 plugin 根目錄，即 `skills/`、`agents/`、`hooks/`、`commands/`（舊版）或 `.claude-plugin/` 的父目錄
2. **驗證 plugin.json**：如果您的 plugin 包含清單，請確保它具有有效的 JSON 語法
3. **檢查文件權限**：確保 plugin 目錄可讀

<h3 id="skills-not-appearing">
  Skills 未出現
</h3>

如果 plugin skills 不起作用：

1. **使用命名空間**：以 `/plugin-name:skill-name` 的方式調用 plugin skills
2. **檢查初始化消息**：驗證 skill 是否以正確的命名空間出現在 `skills` 列表中
3. **驗證 skill 文件**：確保每個 skill 在 `skills/` 下的自己的子目錄中都有 `SKILL.md` 文件，例如 `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  路徑解析問題
</h3>

如果相對路徑不起作用：

1. **檢查工作目錄**：相對路徑從您的當前工作目錄解析
2. **使用絕對路徑**：為了可靠性，請考慮使用絕對路徑
3. **規範化路徑**：使用路徑實用程序正確構造路徑

<h2 id="see-also">
  另請參閱
</h2>

* [Plugins](/docs/zh-TW/plugins) - 完整的 plugin 開發指南
* [Plugins reference](/docs/zh-TW/plugins-reference) - 技術規範
* [Commands](/docs/zh-TW/agent-sdk/slash-commands) - 在 SDK 中使用 commands
* [Subagents](/docs/zh-TW/agent-sdk/subagents) - 使用專門的 agents
* [Skills](/docs/zh-TW/agent-sdk/skills) - 使用 Agent Skills
