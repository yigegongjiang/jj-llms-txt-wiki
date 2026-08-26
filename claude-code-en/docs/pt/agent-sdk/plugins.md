> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins no SDK

> Carregue plugins personalizados para estender Claude Code com skills, agentes, hooks e servidores MCP através do Agent SDK

Plugins permitem que você estenda Claude Code com funcionalidade personalizada que pode ser compartilhada entre projetos. Através do Agent SDK, você pode carregar programaticamente plugins de diretórios locais para adicionar skills, agentes, hooks e servidores MCP às suas sessões de agente.

<h2 id="what-are-plugins">
  O que são plugins?
</h2>

Plugins são pacotes de extensões Claude Code que podem incluir:

* **Skills**: Capacidades invocadas pelo modelo que Claude usa autonomamente (também podem ser invocadas com `/skill-name`)
* **Agents**: Subagentes especializados para tarefas específicas
* **Hooks**: Manipuladores de eventos que respondem ao uso de ferramentas e outros eventos
* **MCP servers**: Integrações de ferramentas externas via Model Context Protocol

<Note>
  O diretório `commands/` é um formato legado. Use `skills/` para novos plugins. Claude Code continua suportando ambos os formatos para compatibilidade com versões anteriores.
</Note>

Para informações completas sobre a estrutura de plugins e como criar plugins, consulte [Plugins](/docs/pt/plugins).

<h2 id="loading-plugins">
  Carregando plugins
</h2>

Carregue plugins fornecendo seus caminhos do sistema de arquivos local na configuração de opções. O campo `type` deve ser `"local"`, o único valor que o SDK aceita. Para usar um plugin distribuído através de um [marketplace](/docs/pt/plugin-marketplaces) ou repositório remoto, baixe-o primeiro e forneça o caminho do diretório local. O SDK suporta carregamento de múltiplos plugins de diferentes locais.

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

* **Caminhos relativos**: Resolvidos relativamente ao seu diretório de trabalho atual (por exemplo, `"./plugins/my-plugin"`)
* **Caminhos absolutos**: Caminhos completos do sistema de arquivos (por exemplo, `"/home/user/plugins/my-plugin"`)

<Note>
  O caminho deve apontar para o diretório raiz do plugin: o diretório pai de `skills/`, `agents/`, `hooks/`, `commands/` (legado), ou `.claude-plugin/`, não um subdiretório.
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
      // Verificar plugins carregados
      console.log("Plugins:", message.plugins);
      // Exemplo: [{ name: "my-plugin", path: "./my-plugin" }]

      // As skills do plugin aparecem com o nome do plugin como prefixo
      console.log("Skills:", message.skills);
      // Exemplo: ["my-plugin:greet"]

      // Os comandos do plugin usam o mesmo prefixo, e as skills aparecem aqui também
      console.log("Commands:", message.slash_commands);
      // Exemplo: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
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
              # Verificar plugins carregados
              print("Plugins:", message.data.get("plugins"))
              # Exemplo: [{"name": "my-plugin", "path": "./my-plugin"}]

              # As skills do plugin aparecem com o nome do plugin como prefixo
              print("Skills:", message.data.get("skills"))
              # Exemplo: ["my-plugin:greet"]

              # Os comandos do plugin usam o mesmo prefixo, e as skills aparecem aqui também
              print("Commands:", message.data.get("slash_commands"))
              # Exemplo: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="using-plugin-skills">
  Usando skills de plugins
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
  Se você instalou um plugin via CLI (por exemplo, `/plugin install my-plugin@marketplace`), você ainda pode usá-lo no SDK fornecendo seu caminho de instalação. Verifique `~/.claude/plugins/` para plugins instalados via CLI.
</Note>

<h2 id="complete-example">
  Exemplo completo
</h2>

Aqui está um exemplo completo demonstrando carregamento e uso de plugins:

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
  Referência de estrutura de plugin
</h2>

Um diretório de plugin normalmente contém um arquivo de manifesto `.claude-plugin/plugin.json`. O manifesto é opcional. Quando omitido, Claude Code descobre automaticamente componentes a partir do layout do diretório. O diretório pode incluir:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Manifesto de plugin (opcional, componentes descobertos automaticamente sem ele)
├── skills/                   # Agent Skills (invocadas autonomamente ou via /skill-name)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # Legado: use skills/ em vez disso
│   └── custom-cmd.md
├── agents/                   # Agentes personalizados
│   └── specialist.md
├── hooks/                    # Manipuladores de eventos
│   └── hooks.json
└── .mcp.json                # Definições de servidor MCP
```

Para informações detalhadas sobre como criar plugins, consulte:

* [Plugins](/docs/pt/plugins) - Guia completo de desenvolvimento de plugins
* [Plugins reference](/docs/pt/plugins-reference) - Especificações técnicas e esquemas

<h2 id="common-use-cases">
  Casos de uso comuns
</h2>

<h3 id="development-and-testing">
  Desenvolvimento e testes
</h3>

Carregue plugins durante o desenvolvimento sem instalá-los globalmente:

```typescript theme={null}
plugins: [{ type: "local", path: "./dev-plugins/my-plugin" }];
```

<h3 id="project-specific-extensions">
  Extensões específicas do projeto
</h3>

Inclua plugins no seu repositório de projeto para consistência em toda a equipe:

```typescript theme={null}
plugins: [{ type: "local", path: "./project-plugins/team-workflows" }];
```

<h3 id="multiple-plugin-sources">
  Múltiplas fontes de plugin
</h3>

Combine plugins de diferentes locais:

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
  Plugin não carregando
</h3>

Se seu plugin não aparecer na mensagem de inicialização:

1. **Verifique o caminho**: certifique-se de que o caminho aponta para o diretório raiz do plugin, o pai de `skills/`, `agents/`, `hooks/`, `commands/` (legado), ou `.claude-plugin/`
2. **Valide plugin.json**: se seu plugin inclui um manifesto, certifique-se de que ele tem sintaxe JSON válida
3. **Verifique permissões de arquivo**: certifique-se de que o diretório do plugin é legível

<h3 id="skills-not-appearing">
  Skills não aparecendo
</h3>

Se skills de plugins não funcionarem:

1. **Use o namespace**: invoque skills de plugins como `/plugin-name:skill-name`
2. **Verifique mensagem de inicialização**: verifique se a skill aparece na lista `skills` com o namespace correto
3. **Valide arquivos de skill**: certifique-se de que cada skill tem um arquivo `SKILL.md` em seu próprio subdiretório sob `skills/`, por exemplo `skills/my-skill/SKILL.md`

<h3 id="path-resolution-issues">
  Problemas de resolução de caminho
</h3>

Se caminhos relativos não funcionarem:

1. **Verifique diretório de trabalho**: caminhos relativos são resolvidos a partir do seu diretório de trabalho atual
2. **Use caminhos absolutos**: para confiabilidade, considere usar caminhos absolutos
3. **Normalize caminhos**: use utilitários de caminho para construir caminhos corretamente

<h2 id="see-also">
  Veja também
</h2>

* [Plugins](/docs/pt/plugins) - Guia completo de desenvolvimento de plugins
* [Plugins reference](/docs/pt/plugins-reference) - Especificações técnicas
* [Commands](/docs/pt/agent-sdk/slash-commands) - Usando comandos no SDK
* [Subagents](/docs/pt/agent-sdk/subagents) - Trabalhando com agentes especializados
* [Skills](/docs/pt/agent-sdk/skills) - Usando Agent Skills
