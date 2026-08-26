> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins en el SDK

> Cargue plugins personalizados para extender Claude Code con skills, agentes, hooks y servidores MCP a través del Agent SDK

Los plugins le permiten extender Claude Code con funcionalidad personalizada que se puede compartir entre proyectos. A través del Agent SDK, puede cargar programáticamente plugins desde directorios locales para agregar skills, agentes, hooks y servidores MCP a sus sesiones de agente.

<h2 id="what-are-plugins">
  ¿Qué son los plugins?
</h2>

Los plugins son paquetes de extensiones de Claude Code que pueden incluir:

* **Skills**: Capacidades invocadas por el modelo que Claude utiliza de forma autónoma (también se pueden invocar con `/skill-name`)
* **Agents**: Subagentes especializados para tareas específicas
* **Hooks**: Controladores de eventos que responden al uso de herramientas y otros eventos
* **MCP servers**: Integraciones de herramientas externas a través del Model Context Protocol

<Note>
  El directorio `commands/` es un formato heredado. Use `skills/` para nuevos plugins. Claude Code continúa admitiendo ambos formatos para compatibilidad hacia atrás.
</Note>

Para obtener información completa sobre la estructura de plugins y cómo crear plugins, consulte [Plugins](/docs/es/plugins).

<h2 id="loading-plugins">
  Cargando plugins
</h2>

Cargue plugins proporcionando sus rutas del sistema de archivos local en su configuración de opciones. El campo `type` debe ser `"local"`, el único valor que acepta el SDK. Para usar un plugin distribuido a través de un [marketplace](/docs/es/plugin-marketplaces) o repositorio remoto, descárguelo primero y proporcione la ruta del directorio local. El SDK admite cargar múltiples plugins desde diferentes ubicaciones.

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

* **Rutas relativas**: Se resuelven en relación con su directorio de trabajo actual (por ejemplo, `"./plugins/my-plugin"`)
* **Rutas absolutas**: Rutas completas del sistema de archivos (por ejemplo, `"/home/user/plugins/my-plugin"`)

<Note>
  La ruta debe apuntar al directorio raíz del plugin: el directorio padre de `skills/`, `agents/`, `hooks/`, `commands/` (heredado), o `.claude-plugin/`, no a un subdirectorio.
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
      // Verificar plugins cargados
      console.log("Plugins:", message.plugins);
      // Ejemplo: [{ name: "my-plugin", path: "./my-plugin" }]

      // Las skills del plugin aparecen con el nombre del plugin como prefijo
      console.log("Skills:", message.skills);
      // Ejemplo: ["my-plugin:greet"]

      // Los comandos del plugin usan el mismo prefijo, y las skills también aparecen aquí
      console.log("Commands:", message.slash_commands);
      // Ejemplo: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
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
              # Verificar plugins cargados
              print("Plugins:", message.data.get("plugins"))
              # Ejemplo: [{"name": "my-plugin", "path": "./my-plugin"}]

              # Las skills del plugin aparecen con el nombre del plugin como prefijo
              print("Skills:", message.data.get("skills"))
              # Ejemplo: ["my-plugin:greet"]

              # Los comandos del plugin usan el mismo prefijo, y las skills también aparecen aquí
              print("Commands:", message.data.get("slash_commands"))
              # Ejemplo: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
  Usando skills de plugins
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
  Si instaló un plugin a través de la CLI (por ejemplo, `/plugin install my-plugin@marketplace`), aún puede usarlo en el SDK proporcionando su ruta de instalación. Verifique `~/.claude/plugins/` para plugins instalados por CLI.
</Note>

<h2 id="complete-example">
  Ejemplo completo
</h2>

Aquí hay un ejemplo completo que demuestra la carga y el uso de plugins:

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
  Referencia de estructura de plugin
</h2>

Un directorio de plugin típicamente contiene un archivo de manifiesto `.claude-plugin/plugin.json`. El manifiesto es opcional. Cuando se omite, Claude Code descubre automáticamente los componentes desde el diseño del directorio. El directorio puede incluir:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Manifiesto de plugin (opcional, componentes descubiertos automáticamente sin él)
├── skills/                   # Agent Skills (invocadas autónomamente o vía /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Heredado: usar skills/ en su lugar
│   └── custom-cmd.md
├── agents/                   # Agentes personalizados
│   └── specialist.md
├── hooks/                    # Manejadores de eventos
│   └── hooks.json
└── .mcp.json                # Definiciones de servidor MCP
```

Para obtener información detallada sobre cómo crear plugins, consulte:

* [Plugins](/docs/es/plugins) - Guía completa de desarrollo de plugins
* [Plugins reference](/docs/es/plugins-reference) - Especificaciones técnicas y esquemas

<h2 id="common-use-cases">
  Casos de uso comunes
</h2>

<h3 id="development-and-testing">
  Desarrollo y pruebas
</h3>

Cargue plugins durante el desarrollo sin instalarlos globalmente:

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Extensiones específicas del proyecto
</h3>

Incluya plugins en su repositorio de proyecto para consistencia en todo el equipo:

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Múltiples fuentes de plugins
</h3>

Combine plugins de diferentes ubicaciones:

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
  Plugin no se carga
</h3>

Si su plugin no aparece en el mensaje de inicialización:

1. **Verifique la ruta**: Asegúrese de que la ruta apunte al directorio raíz del plugin, el directorio padre de `skills/`, `agents/`, `hooks/`, `commands/` (heredado), o `.claude-plugin/`
2. **Valide plugin.json**: Si su plugin incluye un manifiesto, asegúrese de que tenga una sintaxis JSON válida
3. **Verifique los permisos de archivo**: Asegúrese de que el directorio del plugin sea legible

<h3 id="skills-not-appearing">
  Los skills no aparecen
</h3>

Si los skills del plugin no funcionan:

1. **Use el espacio de nombres**: Invoque los skills del plugin como `/plugin-name:skill-name`
2. **Verifique el mensaje de inicialización**: Verifique que el skill aparezca en la lista `skills` con el espacio de nombres correcto
3. **Valide los archivos de skill**: Asegúrese de que cada skill tenga un archivo `SKILL.md` en su propio subdirectorio bajo `skills/`, por ejemplo `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Problemas de resolución de ruta
</h3>

Si las rutas relativas no funcionan:

1. **Verifique el directorio de trabajo**: Las rutas relativas se resuelven desde su directorio de trabajo actual
2. **Use rutas absolutas**: Para mayor confiabilidad, considere usar rutas absolutas
3. **Normalice las rutas**: Use utilidades de ruta para construir rutas correctamente

<h2 id="see-also">
  Ver también
</h2>

* [Plugins](/docs/es/plugins) - Guía completa de desarrollo de plugins
* [Plugins reference](/docs/es/plugins-reference) - Especificaciones técnicas
* [Commands](/docs/es/agent-sdk/slash-commands) - Usando comandos en el SDK
* [Subagents](/docs/es/agent-sdk/subagents) - Trabajando con agentes especializados
* [Skills](/docs/es/agent-sdk/skills) - Usando Agent Skills
