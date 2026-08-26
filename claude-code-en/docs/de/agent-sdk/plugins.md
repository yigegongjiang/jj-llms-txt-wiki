> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins im SDK

> Laden Sie benutzerdefinierte Plugins, um Claude Code mit Skills, Agenten, Hooks und MCP-Servern über das Agent SDK zu erweitern

Plugins ermöglichen es Ihnen, Claude Code mit benutzerdefinierten Funktionen zu erweitern, die projektübergreifend gemeinsam genutzt werden können. Über das Agent SDK können Sie Plugins programmgesteuert aus lokalen Verzeichnissen laden, um Skills, Agenten, Hooks und MCP-Server zu Ihren Agent-Sitzungen hinzuzufügen.

<h2 id="what-are-plugins">
  Was sind Plugins?
</h2>

Plugins sind Pakete von Claude Code-Erweiterungen, die Folgendes enthalten können:

* **Skills**: Von Modellen aufgerufene Funktionen, die Claude autonom nutzt (können auch mit `/skill-name` aufgerufen werden)
* **Agenten**: Spezialisierte Subagenten für spezifische Aufgaben
* **Hooks**: Event-Handler, die auf Tool-Nutzung und andere Ereignisse reagieren
* **MCP-Server**: Externe Tool-Integrationen über das Model Context Protocol

<Note>
  Das Verzeichnis `commands/` ist ein veraltetes Format. Verwenden Sie `skills/` für neue Plugins. Claude Code unterstützt weiterhin beide Formate für Rückwärtskompatibilität.
</Note>

Vollständige Informationen zur Plugin-Struktur und zum Erstellen von Plugins finden Sie unter [Plugins](/docs/de/plugins).

<h2 id="loading-plugins">
  Plugins laden
</h2>

Laden Sie Plugins, indem Sie ihre lokalen Dateisystempfade in Ihrer Optionskonfiguration angeben. Das Feld `type` muss `"local"` sein, der einzige Wert, den das SDK akzeptiert. Um ein Plugin zu verwenden, das über einen [Marketplace](/docs/de/plugin-marketplaces) oder ein Remote-Repository verteilt wird, laden Sie es zunächst herunter und geben Sie den lokalen Verzeichnispath an. Das SDK unterstützt das Laden mehrerer Plugins aus verschiedenen Speicherorten.

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

* **Relative Pfade**: Aufgelöst relativ zu Ihrem aktuellen Arbeitsverzeichnis (zum Beispiel `"./plugins/my-plugin"`)
* **Absolute Pfade**: Vollständige Dateisystempfade (zum Beispiel `"/home/user/plugins/my-plugin"`)

<Note>
  Der Pfad sollte auf das Root-Verzeichnis des Plugins verweisen: das übergeordnete Verzeichnis von `skills/`, `agents/`, `hooks/`, `commands/` (Legacy) oder `.claude-plugin/`, nicht auf ein Unterverzeichnis.
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
      // Geladene Plugins überprüfen
      console.log("Plugins:", message.plugins);
      // Beispiel: [{ name: "my-plugin", path: "./my-plugin" }]

      // Plugin-Skills erscheinen mit dem Plugin-Namen als Präfix
      console.log("Skills:", message.skills);
      // Beispiel: ["my-plugin:greet"]

      // Plugin-Befehle verwenden denselben Präfix, und Skills erscheinen auch hier
      console.log("Commands:", message.slash_commands);
      // Beispiel: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
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
              # Geladene Plugins überprüfen
              print("Plugins:", message.data.get("plugins"))
              # Beispiel: [{"name": "my-plugin", "path": "./my-plugin"}]

              # Plugin-Skills erscheinen mit dem Plugin-Namen als Präfix
              print("Skills:", message.data.get("skills"))
              # Beispiel: ["my-plugin:greet"]

              # Plugin-Befehle verwenden denselben Präfix, und Skills erscheinen auch hier
              print("Commands:", message.data.get("slash_commands"))
              # Beispiel: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
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
  Wenn Sie ein Plugin über die CLI installiert haben (zum Beispiel `/plugin install my-plugin@marketplace`), können Sie es im SDK weiterhin verwenden, indem Sie seinen Installationspfad angeben. Überprüfen Sie `~/.claude/plugins/` auf über die CLI installierte Plugins.
</Note>

<h2 id="complete-example">
  Vollständiges Beispiel
</h2>

Hier ist ein vollständiges Beispiel, das das Laden und die Verwendung von Plugins demonstriert:

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
  Plugin-Struktur-Referenz
</h2>

Ein Plugin-Verzeichnis enthält typischerweise eine `.claude-plugin/plugin.json`-Manifestdatei. Das Manifest ist optional. Wenn es weggelassen wird, erkennt Claude Code Komponenten automatisch aus dem Verzeichnislayout. Das Verzeichnis kann Folgendes enthalten:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin-Manifest (optional, Komponenten werden ohne es automatisch erkannt)
├── skills/                   # Agent Skills (werden autonom aufgerufen oder über /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Legacy: verwenden Sie stattdessen skills/
│   └── custom-cmd.md
├── agents/                   # Benutzerdefinierte Agenten
│   └── specialist.md
├── hooks/                    # Event-Handler
│   └── hooks.json
└── .mcp.json                # MCP-Server-Definitionen
```

Detaillierte Informationen zum Erstellen von Plugins finden Sie unter:

* [Plugins](/docs/de/plugins) - Vollständiger Plugin-Entwicklungsleitfaden
* [Plugins-Referenz](/docs/de/plugins-reference) - Technische Spezifikationen und Schemas

<h2 id="common-use-cases">
  Häufige Anwendungsfälle
</h2>

<h3 id="development-and-testing">
  Entwicklung und Tests
</h3>

Laden Sie Plugins während der Entwicklung, ohne sie global zu installieren:

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Projektspezifische Erweiterungen
</h3>

Beziehen Sie Plugins in Ihr Projekt-Repository ein, um teamweite Konsistenz zu gewährleisten:

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Mehrere Plugin-Quellen
</h3>

Kombinieren Sie Plugins aus verschiedenen Speicherorten:

```typescript theme={null}
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
];
```

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="plugin-not-loading">
  Plugin wird nicht geladen
</h3>

Wenn Ihr Plugin nicht in der Init-Meldung angezeigt wird:

1. **Überprüfen Sie den Pfad**: Stellen Sie sicher, dass der Pfad auf das Plugin-Root-Verzeichnis verweist, das übergeordnete Verzeichnis von `skills/`, `agents/`, `hooks/`, `commands/` (veraltet) oder `.claude-plugin/`
2. **Validieren Sie plugin.json**: Wenn Ihr Plugin ein Manifest enthält, stellen Sie sicher, dass es eine gültige JSON-Syntax hat
3. **Überprüfen Sie Dateiberechtigungen**: Stellen Sie sicher, dass das Plugin-Verzeichnis lesbar ist

<h3 id="skills-not-appearing">
  Skills werden nicht angezeigt
</h3>

Wenn Plugin-Skills nicht funktionieren:

1. **Verwenden Sie den Namespace**: Rufen Sie Plugin-Skills als `/plugin-name:skill-name` auf
2. **Überprüfen Sie die Init-Meldung**: Überprüfen Sie, dass der Skill in der `skills`-Liste mit dem korrekten Namespace angezeigt wird
3. **Validieren Sie Skill-Dateien**: Stellen Sie sicher, dass jeder Skill eine `SKILL.md`-Datei in seinem eigenen Unterverzeichnis unter `skills/` hat, zum Beispiel `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Pfadauflösungsprobleme
</h3>

Wenn relative Pfade nicht funktionieren:

1. **Überprüfen Sie das Arbeitsverzeichnis**: Relative Pfade werden von Ihrem aktuellen Arbeitsverzeichnis aus aufgelöst
2. **Verwenden Sie absolute Pfade**: Verwenden Sie für Zuverlässigkeit absolute Pfade
3. **Normalisieren Sie Pfade**: Verwenden Sie Pfad-Dienstprogramme, um Pfade korrekt zu konstruieren

<h2 id="see-also">
  Siehe auch
</h2>

* [Plugins](/docs/de/plugins) - Vollständiger Plugin-Entwicklungsleitfaden
* [Plugins-Referenz](/docs/de/plugins-reference) - Technische Spezifikationen
* [Befehle](/docs/de/agent-sdk/slash-commands) - Verwendung von Befehlen im SDK
* [Subagenten](/docs/de/agent-sdk/subagents) - Arbeiten mit spezialisierten Agenten
* [Skills](/docs/de/agent-sdk/skills) - Verwendung von Agent Skills
