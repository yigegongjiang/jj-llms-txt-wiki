> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins dans le SDK

> Chargez des plugins personnalisés pour étendre Claude Code avec des skills, des agents, des hooks et des serveurs MCP via le SDK Agent

Les plugins vous permettent d'étendre Claude Code avec des fonctionnalités personnalisées qui peuvent être partagées entre les projets. Via le SDK Agent, vous pouvez charger programmatiquement des plugins à partir de répertoires locaux pour ajouter des capacités à vos sessions d'agent. Un plugin peut inclure :

* **Skills** : capacités que Claude invoque de manière autonome lorsqu'elles sont pertinentes. Vous pouvez également invoquer directement un skill de plugin avec `/plugin-name:skill-name`.
* **Agents** : sous-agents spécialisés pour des tâches spécifiques
* **Hooks** : gestionnaires d'événements qui répondent à l'utilisation d'outils et à d'autres événements
* **Serveurs MCP** : intégrations d'outils externes via Model Context Protocol

Pour des informations complètes sur la structure des plugins et comment créer des plugins, consultez [Plugins](/docs/fr/plugins/overview).

<h2 id="loading-plugins">
  Chargement des plugins
</h2>

Chargez les plugins en fournissant leurs chemins du système de fichiers local dans votre configuration d'options. Le champ `type` doit être `"local"`, la seule valeur que le SDK accepte. Le SDK supporte le chargement de plusieurs plugins à partir de différents emplacements.

Pour utiliser un plugin distribué via une [marketplace](/docs/fr/plugins/overview) ou un référentiel distant, téléchargez-le d'abord et fournissez le chemin du répertoire local. Pour la disposition du répertoire dont un plugin a besoin, consultez la [référence de structure des plugins](#plugin-structure-reference) ci-dessous.

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
  Spécifications des chemins
</h3>

Les chemins des plugins peuvent être :

* **Chemins relatifs** : résolus par rapport à l'option `cwd` (par exemple, `"./plugins/my-plugin"`)
* **Chemins absolus** : chemins complets du système de fichiers (par exemple, `"/home/user/plugins/my-plugin"`)

<Note>
  Le chemin doit pointer vers le répertoire racine du plugin : le parent de `skills/`, `agents/`, `hooks/`, `commands/`, ou `.claude-plugin/`.
</Note>

<h2 id="verifying-plugin-installation">
  Vérification de l'installation du plugin
</h2>

Lorsque les plugins se chargent avec succès, ils apparaissent dans le message d'initialisation du système. Vous pouvez vérifier que vos plugins sont disponibles :

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
  Utilisation des skills des plugins
</h2>

Les skills des plugins sont automatiquement espacés de noms avec le nom du plugin pour éviter les conflits. Pour invoquer l'un d'eux directement, envoyez `/plugin-name:skill-name` comme prompt.

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
  Si vous avez installé un plugin via la CLI (par exemple, `/plugin install my-plugin@marketplace`), vous pouvez toujours l'utiliser dans le SDK en fournissant son chemin d'installation. Vérifiez `~/.claude/plugins/` pour les plugins installés via la CLI.
</Note>

<h2 id="complete-example">
  Exemple complet
</h2>

Voici un exemple complet démontrant le chargement et l'utilisation des plugins :

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
  Référence de la structure des plugins
</h2>

Un répertoire de plugin contient généralement un fichier manifeste `.claude-plugin/plugin.json`. Le manifeste est optionnel. Lorsqu'il est omis, Claude Code découvre automatiquement les composants à partir de la disposition du répertoire. Le répertoire peut inclure :

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
  Le répertoire `commands/` contient les skills sous forme de fichiers Markdown plats. Utilisez `skills/` pour les nouveaux plugins. Claude Code supporte les deux emplacements.
</Note>

<h2 id="multiple-plugin-sources">
  Plusieurs sources de plugins
</h2>

Combinez les plugins de différents emplacements :

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
  Le SDK n'étend pas les chemins avec tilde comme `~/plugins`. Si un chemin de plugin n'existe pas, le SDK ignore ce plugin et la session continue, donc vérifiez la liste `plugins` dans le message d'initialisation pour confirmer que chaque plugin s'est chargé.
</Note>

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="plugin-not-loading">
  Plugin ne se charge pas
</h3>

Si votre plugin n'apparaît pas dans le message d'initialisation :

1. **Vérifiez le chemin** : assurez-vous que le chemin pointe vers le répertoire racine du plugin, le parent de `skills/`, `agents/`, `hooks/`, `commands/`, ou `.claude-plugin/`
2. **Validez plugin.json** : si votre plugin inclut un manifeste, assurez-vous qu'il a une syntaxe JSON valide
3. **Vérifiez les permissions de fichier** : assurez-vous que le répertoire du plugin est lisible
4. **Confirmez que le répertoire existe** : le SDK ignore un chemin inexistant, et le plugin n'apparaît pas dans la liste `plugins` du message d'initialisation

<h3 id="skills-not-appearing">
  Les skills n'apparaissent pas
</h3>

Si les skills des plugins ne fonctionnent pas :

1. **Utilisez l'espace de noms** : invoquez les skills des plugins en tant que `/plugin-name:skill-name`
2. **Vérifiez le message d'initialisation** : vérifiez que le skill apparaît dans la liste `skills` avec l'espace de noms correct
3. **Validez les fichiers de skill** : assurez-vous que chaque skill a un fichier `SKILL.md` dans son propre sous-répertoire sous `skills/`, par exemple `skills/my-skill/SKILL.md`

<h2 id="see-also">
  Voir aussi
</h2>

* [Plugins](/docs/fr/plugins/overview) - Guide complet de développement de plugins
* [Référence des plugins](/docs/fr/plugins/manifest-reference) - Spécifications techniques
* [Commands](/docs/fr/agent-sdk/skills#dispatch-commands-by-name) - Dispatching commands in the SDK
* [Subagents](/docs/fr/agent-sdk/subagents) - Travail avec des agents spécialisés
* [Skills](/docs/fr/agent-sdk/skills) - Utilisation des Agent Skills
