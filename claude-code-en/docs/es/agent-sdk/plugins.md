> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins en el SDK

> Cargue plugins personalizados para extender Claude Code con skills, agentes, hooks y servidores MCP a través del Agent SDK

Los plugins le permiten extender Claude Code con funcionalidad personalizada que se puede compartir entre proyectos. A través del Agent SDK, puede cargar programáticamente plugins desde directorios locales para agregar capacidades a sus sesiones de agente. Un plugin puede incluir:

* **Skills**: capacidades que Claude invoca de forma autónoma cuando es relevante. También puede invocar directamente un skill de plugin con `/plugin-name:skill-name`.
* **Agents**: subagentes especializados para tareas específicas
* **Hooks**: controladores de eventos que responden al uso de herramientas y otros eventos
* **MCP servers**: integraciones de herramientas externas a través del Model Context Protocol

Para obtener información completa sobre la estructura de plugins y cómo crear plugins, consulte [Plugins](/docs/es/plugins/overview).

<h2 id="loading-plugins">
  Cargando plugins
</h2>

Cargue plugins proporcionando sus rutas del sistema de archivos local en su configuración de opciones. El campo `type` debe ser `"local"`, el único valor que acepta el SDK. El SDK admite cargar múltiples plugins desde diferentes ubicaciones.

Para usar un plugin distribuido a través de un [marketplace](/docs/es/plugins/overview) o repositorio remoto, descárguelo primero y proporcione la ruta del directorio local. Para el diseño del directorio que necesita un plugin, consulte la [referencia de estructura de plugin](#plugin-structure-reference) a continuación.

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
  Especificaciones de ruta
</h3>

Las rutas de plugins pueden ser:

* **Rutas relativas**: se resuelven en relación con la opción `cwd` (por ejemplo, `"./plugins/my-plugin"`)
* **Rutas absolutas**: rutas completas del sistema de archivos (por ejemplo, `"/home/user/plugins/my-plugin"`)

<Note>
  La ruta debe apuntar al directorio raíz del plugin: el directorio padre de `skills/`, `agents/`, `hooks/`, `commands/`, o `.claude-plugin/`.
</Note>

<h2 id="verifying-plugin-installation">
  Verificando la instalación del plugin
</h2>

Cuando los plugins se cargan correctamente, aparecen en el mensaje de inicialización del sistema. Puede verificar que sus plugins estén disponibles:

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
  Usar skills de plugins
</h2>

Los skills de los plugins se espacian automáticamente con el nombre del plugin para evitar conflictos. Para invocar uno directamente, envíe `/plugin-name:skill-name` como el prompt.

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
  Si instaló un plugin a través de la CLI (por ejemplo, `/plugin install my-plugin@marketplace`), aún puede usarlo en el SDK proporcionando su ruta de instalación. Verifique `~/.claude/plugins/` para plugins instalados por CLI.
</Note>

<h2 id="complete-example">
  Ejemplo completo
</h2>

Aquí hay un ejemplo completo que demuestra la carga y el uso de plugins:

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
  Referencia de estructura de plugin
</h2>

Un directorio de plugin típicamente contiene un archivo de manifiesto `.claude-plugin/plugin.json`. El manifiesto es opcional. Cuando se omite, Claude Code descubre automáticamente los componentes desde el diseño del directorio. El directorio puede incluir:

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
  El directorio `commands/` contiene skills como archivos Markdown planos. Use `skills/` para nuevos plugins. Claude Code admite ambas ubicaciones.
</Note>

<h2 id="multiple-plugin-sources">
  Múltiples fuentes de plugins
</h2>

Combine plugins de diferentes ubicaciones:

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
  El SDK no expande rutas con tilde como `~/plugins`. Si una ruta de plugin no existe, el SDK omite ese plugin y la sesión continúa, así que verifique la lista `plugins` en el mensaje de inicialización para confirmar que cada plugin se cargó.
</Note>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-loading">
  Plugin no se carga
</h3>

Si su plugin no aparece en el mensaje de inicialización:

1. **Verifique la ruta**: asegúrese de que la ruta apunte al directorio raíz del plugin, el directorio padre de `skills/`, `agents/`, `hooks/`, `commands/`, o `.claude-plugin/`
2. **Valide plugin.json**: si su plugin incluye un manifiesto, asegúrese de que tenga una sintaxis JSON válida
3. **Verifique los permisos de archivo**: asegúrese de que el directorio del plugin sea legible
4. **Confirme que el directorio existe**: el SDK omite una ruta inexistente, y el plugin no aparece en la lista `plugins` del mensaje de inicialización

<h3 id="skills-not-appearing">
  Los skills no aparecen
</h3>

Si los skills del plugin no funcionan:

1. **Use el espacio de nombres**: invoque los skills del plugin como `/plugin-name:skill-name`
2. **Verifique el mensaje de inicialización**: verifique que el skill aparezca en la lista `skills` con el espacio de nombres correcto
3. **Valide los archivos de skill**: asegúrese de que cada skill tenga un archivo `SKILL.md` en su propio subdirectorio bajo `skills/`, por ejemplo `skills/my-skill/SKILL.md`

<h2 id="see-also">
  Ver también
</h2>

* [Plugins](/docs/es/plugins/overview) - Guía completa de desarrollo de plugins
* [Plugins reference](/docs/es/plugins/manifest-reference) - Especificaciones técnicas
* [Commands](/docs/es/agent-sdk/skills#dispatch-commands-by-name) - Despachando comandos en el SDK
* [Subagents](/docs/es/agent-sdk/subagents) - Trabajando con agentes especializados
* [Skills](/docs/es/agent-sdk/skills) - Usando Agent Skills
