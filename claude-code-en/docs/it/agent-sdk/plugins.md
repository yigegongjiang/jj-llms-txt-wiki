> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugin nell'SDK

> Carica plugin personalizzati per estendere Claude Code con skills, agenti, hooks e server MCP tramite l'Agent SDK

I plugin ti permettono di estendere Claude Code con funzionalità personalizzate che possono essere condivise tra i progetti. Attraverso l'Agent SDK, puoi caricare programmaticamente i plugin da directory locali per aggiungere skills, agenti, hooks e server MCP alle tue sessioni di agente.

<h2 id="what-are-plugins">
  Cosa sono i plugin?
</h2>

I plugin sono pacchetti di estensioni di Claude Code che possono includere:

* **Skills**: Capacità richiamate dal modello che Claude utilizza autonomamente (possono anche essere richiamate con `/skill-name`)
* **Agents**: Subagenti specializzati per compiti specifici
* **Hooks**: Gestori di eventi che rispondono all'uso degli strumenti e ad altri eventi
* **MCP servers**: Integrazioni di strumenti esterni tramite Model Context Protocol

<Note>
  La directory `commands/` è un formato legacy. Usa `skills/` per i nuovi plugin. Claude Code continua a supportare entrambi i formati per la compatibilità con le versioni precedenti.
</Note>

Per informazioni complete sulla struttura dei plugin e su come creare plugin, vedi [Plugins](/docs/it/plugins).

<h2 id="loading-plugins">
  Caricamento dei plugin
</h2>

Carica i plugin fornendo i loro percorsi del file system locale nella configurazione delle opzioni. Il campo `type` deve essere `"local"`, l'unico valore che l'SDK accetta. Per utilizzare un plugin distribuito tramite un [marketplace](/docs/it/plugin-marketplaces) o un repository remoto, scaricalo prima e fornisci il percorso della directory locale. L'SDK supporta il caricamento di più plugin da posizioni diverse.

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
  Specifiche dei percorsi
</h3>

I percorsi dei plugin possono essere:

* **Percorsi relativi**: Risolti rispetto alla tua directory di lavoro corrente (ad esempio, `"./plugins/my-plugin"`)
* **Percorsi assoluti**: Percorsi completi del file system (ad esempio, `"/home/user/plugins/my-plugin"`)

<Note>
  Il percorso deve puntare alla directory radice del plugin: la directory padre di `skills/`, `agents/`, `hooks/`, `commands/` (legacy), o `.claude-plugin/`, non una sottodirectory.
</Note>

<h2 id="verifying-plugin-installation">
  Verifica dell'installazione del plugin
</h2>

Quando i plugin si caricano correttamente, appaiono nel messaggio di inizializzazione del sistema. Puoi verificare che i tuoi plugin siano disponibili:

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
      // Example: [{ name: "my-plugin", path: "./my-plugin" }]

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
              # Example: [{"name": "my-plugin", "path": "./my-plugin"}]

              # Plugin skills appear with the plugin name as a prefix
              print("Skills:", message.data.get("skills"))
              # Example: ["my-plugin:greet"]

              # Plugin commands use the same prefix, and skills appear here too
              print("Commands:", message.data.get("slash_commands"))
              # Example: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
  Utilizzo delle skills dei plugin
</h2>

Le skills dai plugin vengono automaticamente associate allo spazio dei nomi del plugin per evitare conflitti. Per richiamare una direttamente, invia `/plugin-name:skill-name` come prompt.

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
  Se hai installato un plugin tramite la CLI (ad esempio, `/plugin install my-plugin@marketplace`), puoi comunque utilizzarlo nell'SDK fornendo il suo percorso di installazione. Controlla `~/.claude/plugins/` per i plugin installati tramite CLI.
</Note>

<h2 id="complete-example">
  Esempio completo
</h2>

Ecco un esempio completo che dimostra il caricamento e l'utilizzo dei plugin:

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
  Riferimento della struttura del plugin
</h2>

Una directory di plugin contiene tipicamente un file manifest `.claude-plugin/plugin.json`. Il manifest è facoltativo. Quando omesso, Claude Code scopre automaticamente i componenti dal layout della directory. La directory può includere:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest (facoltativo, componenti auto-scoperti senza di esso)
├── skills/                   # Agent Skills (invocati autonomamente o via /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Legacy: usa skills/ invece
│   └── custom-cmd.md
├── agents/                   # Agenti personalizzati
│   └── specialist.md
├── hooks/                    # Gestori di eventi
│   └── hooks.json
└── .mcp.json                # Definizioni del server MCP
```

Per informazioni dettagliate sulla creazione di plugin, vedi:

* [Plugins](/docs/it/plugins) - Guida completa allo sviluppo dei plugin
* [Plugins reference](/docs/it/plugins-reference) - Specifiche tecniche e schemi

<h2 id="common-use-cases">
  Casi d'uso comuni
</h2>

<h3 id="development-and-testing">
  Sviluppo e test
</h3>

Carica i plugin durante lo sviluppo senza installarli globalmente:

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Estensioni specifiche del progetto
</h3>

Includi i plugin nel tuo repository di progetto per la coerenza a livello di team:

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Più fonti di plugin
</h3>

Combina i plugin da posizioni diverse:

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
  Plugin non caricato
</h3>

Se il tuo plugin non appare nel messaggio di init:

1. **Controlla il percorso**: assicurati che il percorso punti alla directory radice del plugin, la directory padre di `skills/`, `agents/`, `hooks/`, `commands/` (legacy), o `.claude-plugin/`
2. **Valida plugin.json**: se il tuo plugin include un manifest, assicurati che abbia una sintassi JSON valida
3. **Controlla i permessi dei file**: assicurati che la directory del plugin sia leggibile

<h3 id="skills-not-appearing">
  Skills non appaiono
</h3>

Se le skills dei plugin non funzionano:

1. **Usa lo spazio dei nomi**: richiama le skills dei plugin come `/plugin-name:skill-name`
2. **Controlla il messaggio di init**: verifica che la skill appaia nell'elenco `skills` con lo spazio dei nomi corretto
3. **Valida i file delle skills**: assicurati che ogni skill abbia un file `SKILL.md` nella sua sottodirectory sotto `skills/`, ad esempio `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Problemi di risoluzione dei percorsi
</h3>

Se i percorsi relativi non funzionano:

1. **Controlla la directory di lavoro**: i percorsi relativi vengono risolti dalla tua directory di lavoro corrente
2. **Usa percorsi assoluti**: per l'affidabilità, considera l'utilizzo di percorsi assoluti
3. **Normalizza i percorsi**: usa le utility dei percorsi per costruire i percorsi correttamente

<h2 id="see-also">
  Vedi anche
</h2>

* [Plugins](/docs/it/plugins) - Guida completa allo sviluppo dei plugin
* [Plugins reference](/docs/it/plugins-reference) - Specifiche tecniche
* [Commands](/docs/it/agent-sdk/slash-commands) - Utilizzo dei comandi nell'SDK
* [Subagents](/docs/it/agent-sdk/subagents) - Lavoro con agenti specializzati
* [Skills](/docs/it/agent-sdk/skills) - Utilizzo delle Agent Skills
