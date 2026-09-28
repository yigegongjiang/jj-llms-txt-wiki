> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins no SDK

> Carregue plugins personalizados para estender Claude Code com skills, agentes, hooks e servidores MCP através do Agent SDK

Plugins permitem que você estenda Claude Code com funcionalidade personalizada que pode ser compartilhada entre projetos. Através do Agent SDK, você pode carregar programaticamente plugins de diretórios locais para adicionar capacidades às suas sessões de agente. Um plugin pode incluir:

* **Skills**: capacidades que Claude invoca autonomamente quando relevante. Você também pode invocar uma skill de plugin diretamente com `/plugin-name:skill-name`.
* **Agents**: subagentes especializados para tarefas específicas
* **Hooks**: manipuladores de eventos que respondem ao uso de ferramentas e outros eventos
* **MCP servers**: integrações de ferramentas externas via Model Context Protocol

Para informações completas sobre a estrutura de plugins e como criar plugins, consulte [Plugins](/docs/pt/plugins/overview).

<h2 id="loading-plugins">
  Carregando plugins
</h2>

Carregue plugins fornecendo seus caminhos do sistema de arquivos local na configuração de opções. O campo `type` deve ser `"local"`, o único valor que o SDK aceita. O SDK suporta carregamento de múltiplos plugins de diferentes locais.

Para usar um plugin distribuído através de um [marketplace](/docs/pt/plugins/overview) ou repositório remoto, baixe-o primeiro e forneça o caminho do diretório local. Para o layout de diretório que um plugin precisa, consulte a [referência de estrutura de plugin](#plugin-structure-reference) abaixo.

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
  Especificações de caminho
</h3>

Os caminhos de plugin podem ser:

* **Caminhos relativos**: resolvidos relativamente à opção `cwd` (por exemplo, `"./plugins/my-plugin"`)
* **Caminhos absolutos**: caminhos completos do sistema de arquivos (por exemplo, `"/home/user/plugins/my-plugin"`)

<Note>
  O caminho deve apontar para o diretório raiz do plugin: o diretório pai de `skills/`, `agents/`, `hooks/`, `commands/`, ou `.claude-plugin/`.
</Note>

<h2 id="verifying-plugin-installation">
  Verificando a instalação do plugin
</h2>

Quando os plugins carregam com sucesso, eles aparecem na mensagem de inicialização do sistema. Você pode verificar que seus plugins estão disponíveis:

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

Skills de plugins são automaticamente nomeados com o nome do plugin para evitar conflitos. Para invocar um diretamente, envie `/plugin-name:skill-name` como o prompt.

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
  Se você instalou um plugin via CLI (por exemplo, `/plugin install my-plugin@marketplace`), você ainda pode usá-lo no SDK fornecendo seu caminho de instalação. Verifique `~/.claude/plugins/` para plugins instalados via CLI.
</Note>

<h2 id="complete-example">
  Exemplo completo
</h2>

Aqui está um exemplo completo demonstrando carregamento e uso de plugins:

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
  Referência de estrutura de plugin
</h2>

Um diretório de plugin normalmente contém um arquivo de manifesto `.claude-plugin/plugin.json`. O manifesto é opcional. Quando omitido, Claude Code descobre automaticamente componentes a partir do layout do diretório. O diretório pode incluir:

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
  O diretório `commands/` contém skills como arquivos Markdown simples. Use `skills/` para novos plugins. Claude Code suporta ambos os locais.
</Note>

<h2 id="multiple-plugin-sources">
  Múltiplas fontes de plugin
</h2>

Combine plugins de diferentes locais:

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
  O SDK não expande caminhos com til como `~/plugins`. Se um caminho de plugin não existir, o SDK pula esse plugin e a sessão continua, então verifique a lista `plugins` na mensagem de inicialização para confirmar que cada plugin foi carregado.
</Note>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-loading">
  Plugin não carregando
</h3>

Se seu plugin não aparecer na mensagem de inicialização:

1. **Verifique o caminho**: certifique-se de que o caminho aponta para o diretório raiz do plugin, o diretório pai de `skills/`, `agents/`, `hooks/`, `commands/`, ou `.claude-plugin/`
2. **Valide plugin.json**: se seu plugin inclui um manifesto, certifique-se de que ele tem sintaxe JSON válida
3. **Verifique permissões de arquivo**: certifique-se de que o diretório do plugin é legível
4. **Confirme que o diretório existe**: o SDK pula um caminho inexistente, e o plugin não aparece na lista `plugins` da mensagem de inicialização

<h3 id="skills-not-appearing">
  Skills não aparecendo
</h3>

Se skills de plugins não funcionarem:

1. **Use o namespace**: invoque skills de plugins como `/plugin-name:skill-name`
2. **Verifique mensagem de inicialização**: verifique se a skill aparece na lista `skills` com o namespace correto
3. **Valide arquivos de skill**: certifique-se de que cada skill tem um arquivo `SKILL.md` em seu próprio subdiretório sob `skills/`, por exemplo `skills/my-skill/SKILL.md`

<h2 id="see-also">
  Veja também
</h2>

* [Plugins](/docs/pt/plugins/overview) - Guia completo de desenvolvimento de plugins
* [Plugins reference](/docs/pt/plugins/manifest-reference) - Especificações técnicas
* [Commands](/docs/pt/agent-sdk/skills#dispatch-commands-by-name) - Despachando comandos no SDK
* [Subagents](/docs/pt/agent-sdk/subagents) - Trabalhando com agentes especializados
* [Skills](/docs/pt/agent-sdk/skills) - Usando Agent Skills
