> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins im SDK

> Laden Sie benutzerdefinierte Plugins, um Claude Code mit Skills, Agenten, Hooks und MCP-Servern über das Agent SDK zu erweitern

Plugins ermöglichen es Ihnen, Claude Code mit benutzerdefinierten Funktionen zu erweitern, die projektübergreifend gemeinsam genutzt werden können. Über das Agent SDK können Sie Plugins programmgesteuert aus lokalen Verzeichnissen laden, um Funktionen zu Ihren Agent-Sitzungen hinzuzufügen. Ein Plugin kann Folgendes enthalten:

* **Skills**: Funktionen, die Claude autonom aufruft, wenn relevant. Sie können einen Plugin-Skill auch direkt mit `/plugin-name:skill-name` aufrufen.
* **Agenten**: Spezialisierte Subagenten für spezifische Aufgaben
* **Hooks**: Event-Handler, die auf Tool-Nutzung und andere Ereignisse reagieren
* **MCP-Server**: Externe Tool-Integrationen über das Model Context Protocol

Vollständige Informationen zur Plugin-Struktur und zum Erstellen von Plugins finden Sie unter [Plugins](/docs/de/plugins/overview).

<h2 id="loading-plugins">
  Plugins laden
</h2>

Laden Sie Plugins, indem Sie ihre lokalen Dateisystempfade in Ihrer Optionskonfiguration angeben. Das Feld `type` muss `"local"` sein, der einzige Wert, den das SDK akzeptiert. Das SDK unterstützt das Laden mehrerer Plugins aus verschiedenen Speicherorten.

Um ein Plugin zu verwenden, das über einen [Marketplace](/docs/de/plugins/overview) oder ein Remote-Repository verteilt wird, laden Sie es zunächst herunter und geben Sie den lokalen Verzeichnispath an. Informationen zum erforderlichen Verzeichnislayout eines Plugins finden Sie in der [Plugin-Struktur-Referenz](#plugin-structure-reference) unten.

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
  Pfadangaben
</h3>

Plugin-Pfade können sein:

* **Relative Pfade**: Aufgelöst relativ zu der `cwd`-Option (zum Beispiel `"./plugins/my-plugin"`)
* **Absolute Pfade**: Vollständige Dateisystempfade (zum Beispiel `"/home/user/plugins/my-plugin"`)

<Note>
  Der Pfad sollte auf das Root-Verzeichnis des Plugins verweisen: das übergeordnete Verzeichnis von `skills/`, `agents/`, `hooks/`, `commands/` oder `.claude-plugin/`.
</Note>

<h2 id="verifying-plugin-installation">
  Plugin-Installation überprüfen
</h2>

Wenn Plugins erfolgreich geladen werden, erscheinen sie in der Systeminitalisierungsmeldung. Sie können überprüfen, dass Ihre Plugins verfügbar sind:

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
  Plugin-Skills verwenden
</h2>

Skills aus Plugins werden automatisch mit dem Plugin-Namen versehen, um Konflikte zu vermeiden. Um einen direkt aufzurufen, senden Sie `/plugin-name:skill-name` als Eingabeaufforderung.

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
  Wenn Sie ein Plugin über die CLI installiert haben (zum Beispiel `/plugin install my-plugin@marketplace`), können Sie es im SDK weiterhin verwenden, indem Sie seinen Installationspfad angeben. Überprüfen Sie `~/.claude/plugins/` auf über die CLI installierte Plugins.
</Note>

<h2 id="complete-example">
  Vollständiges Beispiel
</h2>

Hier ist ein vollständiges Beispiel, das das Laden und die Verwendung von Plugins demonstriert:

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
  Plugin-Struktur-Referenz
</h2>

Ein Plugin-Verzeichnis enthält typischerweise eine `.claude-plugin/plugin.json`-Manifestdatei. Das Manifest ist optional. Wenn es weggelassen wird, erkennt Claude Code Komponenten automatisch aus dem Verzeichnislayout. Das Verzeichnis kann Folgendes enthalten:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin-Manifest (optional, Komponenten werden ohne es automatisch erkannt)
├── skills/                   # Agent Skills (werden autonom aufgerufen oder über /plugin-name:skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Skills als flache .md-Dateien
│   └── custom-cmd.md
├── agents/                   # Benutzerdefinierte Agenten
│   └── specialist.md
├── hooks/                    # Event-Handler
│   └── hooks.json
└── .mcp.json                # MCP-Server-Definitionen
```

<Note>
  Das Verzeichnis `commands/` enthält Skills als flache Markdown-Dateien. Verwenden Sie `skills/` für neue Plugins. Claude Code unterstützt beide Speicherorte.
</Note>

<h2 id="multiple-plugin-sources">
  Mehrere Plugin-Quellen
</h2>

Kombinieren Sie Plugins aus verschiedenen Speicherorten:

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
  Das SDK erweitert keine Tilde-Pfade wie `~/plugins`. Wenn ein Plugin-Pfad nicht vorhanden ist, überspringt das SDK dieses Plugin und die Sitzung wird fortgesetzt. Überprüfen Sie daher die `plugins`-Liste in der Init-Meldung, um zu bestätigen, dass jedes Plugin geladen wurde.
</Note>

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="plugin-not-loading">
  Plugin wird nicht geladen
</h3>

Wenn Ihr Plugin nicht in der Init-Meldung angezeigt wird:

1. **Überprüfen Sie den Pfad**: Stellen Sie sicher, dass der Pfad auf das Plugin-Root-Verzeichnis verweist, das übergeordnete Verzeichnis von `skills/`, `agents/`, `hooks/`, `commands/` oder `.claude-plugin/`
2. **Validieren Sie plugin.json**: Wenn Ihr Plugin ein Manifest enthält, stellen Sie sicher, dass es eine gültige JSON-Syntax hat
3. **Überprüfen Sie Dateiberechtigungen**: Stellen Sie sicher, dass das Plugin-Verzeichnis lesbar ist
4. **Bestätigen Sie, dass das Verzeichnis vorhanden ist**: Das SDK überspringt einen nicht vorhandenen Pfad, und das Plugin wird nicht in der `plugins`-Liste der Init-Meldung angezeigt

<h3 id="skills-not-appearing">
  Skills werden nicht angezeigt
</h3>

Wenn Plugin-Skills nicht funktionieren:

1. **Verwenden Sie den Namespace**: Rufen Sie Plugin-Skills als `/plugin-name:skill-name` auf
2. **Überprüfen Sie die Init-Meldung**: Überprüfen Sie, dass der Skill in der `skills`-Liste mit dem korrekten Namespace angezeigt wird
3. **Validieren Sie Skill-Dateien**: Stellen Sie sicher, dass jeder Skill eine `SKILL.md`-Datei in seinem eigenen Unterverzeichnis unter `skills/` hat, zum Beispiel `skills/my-skill/SKILL.md`

<h2 id="see-also">
  Siehe auch
</h2>

* [Plugins](/docs/de/plugins/overview) - Vollständiger Plugin-Entwicklungsleitfaden
* [Plugins-Referenz](/docs/de/plugins/manifest-reference) - Technische Spezifikationen
* [Befehle](/docs/de/agent-sdk/skills#dispatch-commands-by-name) - Versand von Befehlen im SDK
* [Subagenten](/docs/de/agent-sdk/subagents) - Arbeiten mit spezialisierten Agenten
* [Skills](/docs/de/agent-sdk/skills) - Verwendung von Agent Skills
