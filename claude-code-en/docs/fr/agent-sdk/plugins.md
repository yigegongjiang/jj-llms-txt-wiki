> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins dans le SDK

> Chargez des plugins personnalisés pour étendre Claude Code avec des skills, des agents, des hooks et des serveurs MCP via le SDK Agent

Les plugins vous permettent d'étendre Claude Code avec des fonctionnalités personnalisées qui peuvent être partagées entre les projets. Via le SDK Agent, vous pouvez charger programmatiquement des plugins à partir de répertoires locaux pour ajouter des skills, des agents, des hooks et des serveurs MCP à vos sessions d'agent.

<h2 id="what-are-plugins">
  Que sont les plugins ?
</h2>

Les plugins sont des packages d'extensions Claude Code qui peuvent inclure :

* **Skills** : Capacités invoquées par le modèle que Claude utilise de manière autonome (peuvent également être invoquées avec `/skill-name`)
* **Agents** : Sous-agents spécialisés pour des tâches spécifiques
* **Hooks** : Gestionnaires d'événements qui répondent à l'utilisation d'outils et à d'autres événements
* **Serveurs MCP** : Intégrations d'outils externes via Model Context Protocol

<Note>
  Le répertoire `commands/` est un format hérité. Utilisez `skills/` pour les nouveaux plugins. Claude Code continue de supporter les deux formats pour la compatibilité rétroactive.
</Note>

Pour des informations complètes sur la structure des plugins et comment créer des plugins, consultez [Plugins](/docs/fr/plugins).

<h2 id="loading-plugins">
  Chargement des plugins
</h2>

Chargez les plugins en fournissant leurs chemins du système de fichiers local dans votre configuration d'options. Le champ `type` doit être `"local"`, la seule valeur que le SDK accepte. Pour utiliser un plugin distribué via une [marketplace](/docs/fr/plugin-marketplaces) ou un référentiel distant, téléchargez-le d'abord et fournissez le chemin du répertoire local. Le SDK supporte le chargement de plusieurs plugins à partir de différents emplacements.

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

* **Chemins relatifs** : Résolus par rapport à votre répertoire de travail actuel (par exemple, `"./plugins/my-plugin"`)
* **Chemins absolus** : Chemins complets du système de fichiers (par exemple, `"/home/user/plugins/my-plugin"`)

<Note>
  Le chemin doit pointer vers le répertoire racine du plugin : le parent de `skills/`, `agents/`, `hooks/`, `commands/` (hérité), ou `.claude-plugin/`, et non un sous-répertoire.
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
      // Vérifier les plugins chargés
      console.log("Plugins:", message.plugins);
      // Exemple : [{ name: "my-plugin", path: "./my-plugin" }]

      // Les compétences du plugin apparaissent avec le nom du plugin comme préfixe
      console.log("Skills:", message.skills);
      // Exemple : ["my-plugin:greet"]

      // Les commandes du plugin utilisent le même préfixe, et les compétences apparaissent également ici
      console.log("Commands:", message.slash_commands);
      // Exemple : ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
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
              # Vérifier les plugins chargés
              print("Plugins:", message.data.get("plugins"))
              # Exemple : [{"name": "my-plugin", "path": "./my-plugin"}]

              # Les compétences du plugin apparaissent avec le nom du plugin comme préfixe
              print("Skills:", message.data.get("skills"))
              # Exemple : ["my-plugin:greet"]

              # Les commandes du plugin utilisent le même préfixe, et les compétences apparaissent également ici
              print("Commands:", message.data.get("slash_commands"))
              # Exemple : ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
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
  Si vous avez installé un plugin via la CLI (par exemple, `/plugin install my-plugin@marketplace`), vous pouvez toujours l'utiliser dans le SDK en fournissant son chemin d'installation. Vérifiez `~/.claude/plugins/` pour les plugins installés via la CLI.
</Note>

<h2 id="complete-example">
  Exemple complet
</h2>

Voici un exemple complet démontrant le chargement et l'utilisation des plugins :

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
  Référence de la structure des plugins
</h2>

Un répertoire de plugin contient généralement un fichier manifeste `.claude-plugin/plugin.json`. Le manifeste est optionnel. Lorsqu'il est omis, Claude Code découvre automatiquement les composants à partir de la disposition du répertoire. Le répertoire peut inclure :

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Manifeste du plugin (optionnel, composants découverts automatiquement sans lui)
├── skills/                   # Agent Skills (invoquées de manière autonome ou via /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Héritage : utilisez skills/ à la place
│   └── custom-cmd.md
├── agents/                   # Agents personnalisés
│   └── specialist.md
├── hooks/                    # Gestionnaires d'événements
│   └── hooks.json
└── .mcp.json                # Définitions du serveur MCP
```

Pour des informations détaillées sur la création de plugins, consultez :

* [Plugins](/docs/fr/plugins) - Guide complet de développement de plugins
* [Référence des plugins](/docs/fr/plugins-reference) - Spécifications techniques et schémas

<h2 id="common-use-cases">
  Cas d'usage courants
</h2>

<h3 id="development-and-testing">
  Développement et test
</h3>

Chargez les plugins pendant le développement sans les installer globalement :

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Extensions spécifiques au projet
</h3>

Incluez les plugins dans votre référentiel de projet pour la cohérence à l'échelle de l'équipe :

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Plusieurs sources de plugins
</h3>

Combinez les plugins de différents emplacements :

```typescript theme={null}
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
];
```

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="plugin-not-loading">
  Plugin ne se charge pas
</h3>

Si votre plugin n'apparaît pas dans le message d'initialisation :

1. **Vérifiez le chemin** : Assurez-vous que le chemin pointe vers le répertoire racine du plugin, le parent de `skills/`, `agents/`, `hooks/`, `commands/` (hérité), ou `.claude-plugin/`
2. **Validez plugin.json** : Si votre plugin inclut un manifeste, assurez-vous qu'il a une syntaxe JSON valide
3. **Vérifiez les permissions de fichier** : Assurez-vous que le répertoire du plugin est lisible

<h3 id="skills-not-appearing">
  Les skills n'apparaissent pas
</h3>

Si les skills des plugins ne fonctionnent pas :

1. **Utilisez l'espace de noms** : Invoquez les skills des plugins en tant que `/plugin-name:skill-name`
2. **Vérifiez le message d'initialisation** : Vérifiez que le skill apparaît dans la liste `skills` avec l'espace de noms correct
3. **Validez les fichiers de skill** : Assurez-vous que chaque skill a un fichier `SKILL.md` dans son propre sous-répertoire sous `skills/`, par exemple `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Problèmes de résolution de chemin
</h3>

Si les chemins relatifs ne fonctionnent pas :

1. **Vérifiez le répertoire de travail** : Les chemins relatifs sont résolus à partir de votre répertoire de travail actuel
2. **Utilisez des chemins absolus** : Pour la fiabilité, envisagez d'utiliser des chemins absolus
3. **Normalisez les chemins** : Utilisez les utilitaires de chemin pour construire les chemins correctement

<h2 id="see-also">
  Voir aussi
</h2>

* [Plugins](/docs/fr/plugins) - Guide complet de développement de plugins
* [Référence des plugins](/docs/fr/plugins-reference) - Spécifications techniques
* [Commands](/docs/fr/agent-sdk/slash-commands) - Utilisation des commandes dans le SDK
* [Subagents](/docs/fr/agent-sdk/subagents) - Travail avec des agents spécialisés
* [Skills](/docs/fr/agent-sdk/skills) - Utilisation des Agent Skills
