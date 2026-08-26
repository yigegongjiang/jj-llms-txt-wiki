> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins в SDK

> Загружайте пользовательские plugins для расширения Claude Code с помощью skills, agents, hooks и MCP серверов через Agent SDK

Plugins позволяют расширить Claude Code пользовательской функциональностью, которая может быть общей для нескольких проектов. Через Agent SDK вы можете программно загружать plugins из локальных директорий, чтобы добавить skills, agents, hooks и MCP серверы к сеансам вашего agent.

<h2 id="what-are-plugins">
  Что такое plugins?
</h2>

Plugins — это пакеты расширений Claude Code, которые могут включать:

* **Skills**: Возможности, вызываемые моделью, которые Claude использует автономно (также могут быть вызваны с помощью `/skill-name`)
* **Agents**: Специализированные подагенты для конкретных задач
* **Hooks**: Обработчики событий, которые реагируют на использование инструментов и другие события
* **MCP серверы**: Интеграции внешних инструментов через Model Context Protocol

<Note>
  Директория `commands/` — это устаревший формат. Используйте `skills/` для новых plugins. Claude Code продолжает поддерживать оба формата для обратной совместимости.
</Note>

Для полной информации о структуре plugin и способах создания plugins см. [Plugins](/docs/ru/plugins).

<h2 id="loading-plugins">
  Загрузка plugins
</h2>

Загружайте plugins, предоставляя пути их локальной файловой системы в конфигурации параметров. Поле `type` должно быть `"local"`, это единственное значение, которое принимает SDK. Чтобы использовать plugin, распространяемый через [marketplace](/docs/ru/plugin-marketplaces) или удаленный репозиторий, сначала загрузите его и предоставьте путь локальной директории. SDK поддерживает загрузку нескольких plugins из разных мест.

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

* **Относительные пути**: Разрешаются относительно вашей текущей рабочей директории (например, `"./plugins/my-plugin"`)
* **Абсолютные пути**: Полные пути файловой системы (например, `"/home/user/plugins/my-plugin"`)

<Note>
  Путь должен указывать на корневую директорию plugin: родительскую директорию `skills/`, `agents/`, `hooks/`, `commands/` (устаревший), или `.claude-plugin/`, а не на поддиректорию.
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
      // Проверка загруженных plugins
      console.log("Plugins:", message.plugins);
      // Пример: [{ name: "my-plugin", path: "./my-plugin" }]

      // Plugin skills появляются с именем plugin в качестве префикса
      console.log("Skills:", message.skills);
      // Пример: ["my-plugin:greet"]

      // Plugin команды используют тот же префикс, и skills также появляются здесь
      console.log("Commands:", message.slash_commands);
      // Пример: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
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
              # Проверка загруженных plugins
              print("Plugins:", message.data.get("plugins"))
              # Пример: [{"name": "my-plugin", "path": "./my-plugin"}]

              # Plugin skills появляются с именем plugin в качестве префикса
              print("Skills:", message.data.get("skills"))
              # Пример: ["my-plugin:greet"]

              # Plugin команды используют тот же префикс, и skills также появляются здесь
              print("Commands:", message.data.get("slash_commands"))
              # Пример: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
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
  Если вы установили plugin через CLI (например, `/plugin install my-plugin@marketplace`), вы все еще можете использовать его в SDK, предоставив путь его установки. Проверьте `~/.claude/plugins/` для plugins, установленных через CLI.
</Note>

<h2 id="complete-example">
  Полный пример
</h2>

Вот полный пример, демонстрирующий загрузку и использование plugin:

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
  Справочник структуры plugin
</h2>

Директория plugin обычно содержит файл манифеста `.claude-plugin/plugin.json`. Манифест является опциональным. Когда он опущен, Claude Code автоматически обнаруживает компоненты из структуры директории. Директория может включать:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Манифест plugin (опциональный, компоненты автоматически обнаруживаются без него)
├── skills/                   # Agent Skills (вызываются автономно или через /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Legacy: используйте skills/ вместо этого
│   └── custom-cmd.md
├── agents/                   # Пользовательские агенты
│   └── specialist.md
├── hooks/                    # Обработчики событий
│   └── hooks.json
└── .mcp.json                # Определения MCP server
```

Для подробной информации о создании plugins см.:

* [Plugins](/docs/ru/plugins) — Полное руководство по разработке plugin
* [Plugins reference](/docs/ru/plugins-reference) — Технические спецификации и схемы

<h2 id="common-use-cases">
  Распространенные варианты использования
</h2>

<h3 id="development-and-testing">
  Разработка и тестирование
</h3>

Загружайте plugins во время разработки без их глобальной установки:

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Расширения, специфичные для проекта
</h3>

Включайте plugins в репозиторий вашего проекта для согласованности в команде:

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Несколько источников plugins
</h3>

Объединяйте plugins из разных мест:

```typescript theme={null}
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
];
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-loading">
  Plugin не загружается
</h3>

Если ваш plugin не появляется в сообщении инициализации:

1. **Проверьте путь**: убедитесь, что путь указывает на корневую директорию plugin, родительскую директорию для `skills/`, `agents/`, `hooks/`, `commands/` (устаревшее), или `.claude-plugin/`
2. **Проверьте plugin.json**: если ваш plugin включает манифест, убедитесь, что он имеет корректный синтаксис JSON
3. **Проверьте разрешения файлов**: убедитесь, что директория plugin доступна для чтения

<h3 id="skills-not-appearing">
  Skills не появляются
</h3>

Если skills plugin не работают:

1. **Используйте пространство имен**: вызывайте skills plugin как `/plugin-name:skill-name`
2. **Проверьте сообщение инициализации**: убедитесь, что skill появляется в списке `skills` с правильным пространством имен
3. **Проверьте файлы skill**: убедитесь, что каждый skill имеет файл `SKILL.md` в собственной поддиректории под `skills/`, например `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Проблемы с разрешением пути
</h3>

Если относительные пути не работают:

1. **Проверьте рабочую директорию**: относительные пути разрешаются из вашей текущей рабочей директории
2. **Используйте абсолютные пути**: для надежности рассмотрите использование абсолютных путей
3. **Нормализуйте пути**: используйте утилиты пути для правильного построения путей

<h2 id="see-also">
  См. также
</h2>

* [Plugins](/docs/ru/plugins) — Полное руководство по разработке plugin
* [Plugins reference](/docs/ru/plugins-reference) — Технические спецификации
* [Commands](/docs/ru/agent-sdk/slash-commands) — Использование команд в SDK
* [Subagents](/docs/ru/agent-sdk/subagents) — Работа со специализированными agents
* [Skills](/docs/ru/agent-sdk/skills) — Использование Agent Skills
