> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins в SDK

> Загружайте пользовательские plugins для расширения Claude Code с помощью skills, agents, hooks и MCP серверов через Agent SDK

Plugins позволяют расширить Claude Code пользовательской функциональностью, которая может быть общей для нескольких проектов. Через Agent SDK вы можете программно загружать plugins из локальных директорий, чтобы добавить возможности к сеансам вашего agent. Plugin может включать:

* **Skills**: возможности, которые Claude вызывает автономно, когда это уместно. Вы также можете вызвать skill plugin напрямую с помощью `/plugin-name:skill-name`.
* **Agents**: специализированные подагенты для конкретных задач
* **Hooks**: обработчики событий, которые реагируют на использование инструментов и другие события
* **MCP серверы**: интеграции внешних инструментов через Model Context Protocol

Для полной информации о структуре plugin и способах создания plugins см. [Plugins](/docs/ru/plugins/overview).

<h2 id="loading-plugins">
  Загрузка plugins
</h2>

Загружайте plugins, предоставляя пути их локальной файловой системы в конфигурации параметров. Поле `type` должно быть `"local"`, это единственное значение, которое принимает SDK. SDK поддерживает загрузку нескольких plugins из разных мест.

Чтобы использовать plugin, распространяемый через [marketplace](/docs/ru/plugins/overview) или удаленный репозиторий, сначала загрузите его и предоставьте путь локальной директории. Для структуры директории, которая требуется plugin, см. [справочник структуры plugin](#plugin-structure-reference) ниже.

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
  Спецификации путей
</h3>

Пути plugins могут быть:

* **Относительные пути**: разрешаются относительно опции `cwd` (например, `"./plugins/my-plugin"`)
* **Абсолютные пути**: полные пути файловой системы (например, `"/home/user/plugins/my-plugin"`)

<Note>
  Путь должен указывать на корневую директорию plugin: родительскую директорию `skills/`, `agents/`, `hooks/`, `commands/` или `.claude-plugin/`.
</Note>

<h2 id="verifying-plugin-installation">
  Проверка установки plugin
</h2>

Когда plugins загружаются успешно, они появляются в системном сообщении инициализации. Вы можете проверить, что ваши plugins доступны:

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
  Использование plugin skills
</h2>

Skills из plugins автоматически получают пространство имен с именем plugin, чтобы избежать конфликтов. Для прямого вызова отправьте `/plugin-name:skill-name` как подсказку.

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
  Если вы установили plugin через CLI (например, `/plugin install my-plugin@marketplace`), вы все еще можете использовать его в SDK, предоставив путь его установки. Проверьте `~/.claude/plugins/` для plugins, установленных через CLI.
</Note>

<h2 id="complete-example">
  Полный пример
</h2>

Вот полный пример, демонстрирующий загрузку и использование plugin:

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
  Справочник структуры plugin
</h2>

Директория plugin обычно содержит файл манифеста `.claude-plugin/plugin.json`. Манифест является опциональным. Когда он опущен, Claude Code автоматически обнаруживает компоненты из структуры директории. Директория может включать:

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
  Директория `commands/` содержит skills как плоские файлы Markdown. Используйте `skills/` для новых plugins. Claude Code поддерживает оба расположения.
</Note>

<h2 id="multiple-plugin-sources">
  Несколько источников plugin
</h2>

Объединяйте plugins из разных мест:

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
  SDK не расширяет пути с тильдой, такие как `~/plugins`. Если путь plugin не существует, SDK пропускает этот plugin и сеанс продолжается, поэтому проверьте список `plugins` в сообщении инициализации, чтобы подтвердить, что каждый plugin загрузился.
</Note>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-loading">
  Plugin не загружается
</h3>

Если ваш plugin не появляется в сообщении инициализации:

1. **Проверьте путь**: убедитесь, что путь указывает на корневую директорию plugin, родительскую директорию `skills/`, `agents/`, `hooks/`, `commands/` или `.claude-plugin/`
2. **Проверьте plugin.json**: если ваш plugin включает манифест, убедитесь, что он имеет корректный синтаксис JSON
3. **Проверьте разрешения файлов**: убедитесь, что директория plugin доступна для чтения
4. **Подтвердите существование директории**: SDK пропускает несуществующий путь, и plugin не появляется в списке `plugins` сообщения инициализации

<h3 id="skills-not-appearing">
  Skills не появляются
</h3>

Если skills plugin не работают:

1. **Используйте пространство имен**: вызывайте skills plugin как `/plugin-name:skill-name`
2. **Проверьте сообщение инициализации**: убедитесь, что skill появляется в списке `skills` с правильным пространством имен
3. **Проверьте файлы skill**: убедитесь, что каждый skill имеет файл `SKILL.md` в собственной поддиректории под `skills/`, например `skills/my-skill/SKILL.md`

<h2 id="see-also">
  См. также
</h2>

* [Plugins](/docs/ru/plugins/overview) - Полное руководство по разработке plugin
* [Plugins reference](/docs/ru/plugins/manifest-reference) - Технические спецификации
* [Commands](/docs/ru/agent-sdk/skills#dispatch-commands-by-name) - Отправка команд в SDK
* [Subagents](/docs/ru/agent-sdk/subagents) - Работа со специализированными agents
* [Skills](/docs/ru/agent-sdk/skills) - Использование Agent Skills
