> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectar a ferramentas externas com MCP

> Configure servidores MCP para estender seu agente com ferramentas externas. Abrange tipos de transporte, busca de ferramentas para grandes conjuntos de ferramentas, autenticação e tratamento de erros.

O [Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) é um padrão aberto para conectar agentes de IA a ferramentas e fontes de dados externas. Com MCP, seu agente pode consultar bancos de dados, integrar com APIs como Slack e GitHub, e conectar a outros serviços sem escrever implementações de ferramentas personalizadas.

Os servidores MCP podem ser executados como processos locais, conectar via HTTP ou executar diretamente dentro de sua aplicação SDK.

<Note>
  Esta página abrange a configuração de MCP para o Agent SDK. Para adicionar servidores MCP ao Claude Code CLI para que sejam carregados em cada projeto, consulte [escopos de instalação de MCP](/docs/pt/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Início rápido
</h2>

Este exemplo conecta ao servidor MCP de [documentação do Claude Code](https://code.claude.com/docs) usando [transporte HTTP](#http%2Fsse-servers) e usa [`allowedTools`](#allow-mcp-tools) com um curinga para permitir todas as ferramentas do servidor.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

O agente conecta ao servidor de documentação, busca informações sobre hooks e retorna os resultados.

<h2 id="add-an-mcp-server">
  Adicionar um servidor MCP
</h2>

Você pode configurar servidores MCP em código ao chamar `query()`, ou em um arquivo `.mcp.json` carregado via [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  Em código
</h3>

Passe servidores MCP diretamente na opção `mcpServers`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  De um arquivo de configuração
</h3>

Crie um arquivo `.mcp.json` na raiz do seu projeto. O arquivo é detectado quando a fonte de configuração `project` está habilitada, o que é padrão para as opções `query()`. Se você definir `settingSources` explicitamente, inclua `"project"` para que este arquivo seja carregado:

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="allow-mcp-tools">
  Permitir ferramentas MCP
</h2>

As ferramentas MCP requerem permissão explícita antes que Claude possa usá-las. Sem permissão, Claude verá que as ferramentas estão disponíveis, mas não poderá chamá-las.

<h3 id="tool-naming-convention">
  Convenção de nomenclatura de ferramentas
</h3>

As ferramentas MCP seguem o padrão de nomenclatura `mcp__<server-name>__<tool-name>`. Por exemplo, um servidor GitHub nomeado `"github"` com uma ferramenta `list_issues` se torna `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Aprovação automática com allowedTools
</h3>

Use `allowedTools` para aprovar automaticamente ferramentas MCP específicas para que Claude possa usá-las sem um prompt de permissão:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: [
      "mcp__github__*", // All tools from the github server
      "mcp__db__query", // Only the query tool from db server
      "mcp__slack__send_message" // Only send_message from slack server
    ]
  }
};
```

Curingas (`*`) permitem que você autorize todas as ferramentas de um servidor sem listar cada uma individualmente.

<Note>
  **Prefira `allowedTools` em vez de modos de permissão para acesso MCP.** `permissionMode: "acceptEdits"` não aprova automaticamente ferramentas MCP (apenas edições de arquivo e comandos Bash do sistema de arquivos). `permissionMode: "bypassPermissions"` aprova automaticamente ferramentas MCP, mas também desabilita todos os outros prompts de segurança, o que é mais amplo do que necessário; consulte [Como as permissões são avaliadas](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) para os prompts que permanecem. Um curinga em `allowedTools` concede exatamente o servidor MCP que você deseja e nada mais. Consulte [Modos de permissão](/docs/pt/agent-sdk/permissions#permission-modes) para uma comparação completa.
</Note>

<h3 id="discover-available-tools">
  Descobrir ferramentas disponíveis
</h3>

Para ver quais ferramentas um servidor MCP fornece, verifique a documentação do servidor ou conecte ao servidor e inspecione a mensagem de inicialização `system`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available MCP tools:", message.mcp_servers);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, SystemMessage


  async def main():
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available MCP tools:", message.data["mcp_servers"])


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="transport-types">
  Tipos de transporte
</h2>

Os servidores MCP se comunicam com seu agente usando diferentes protocolos de transporte. Verifique a documentação do servidor para ver qual transporte ele suporta:

* Se a documentação fornecer um **comando para executar** (como `npx @modelcontextprotocol/server-github`), use stdio
* Se a documentação fornecer uma **URL**, use HTTP ou SSE
* Se você estiver construindo suas próprias ferramentas em código, use um servidor MCP SDK

<h3 id="stdio-servers">
  Servidores stdio
</h3>

Processos locais que se comunicam via stdin/stdout. Use isso para servidores MCP que você executa na mesma máquina:

<Tabs>
  <Tab title="Em código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues", "mcp__github__search_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues", "mcp__github__search_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

<h3 id="http/sse-servers">
  Servidores HTTP/SSE
</h3>

Use HTTP ou SSE para servidores MCP hospedados em nuvem e APIs remotas:

<Tabs>
  <Tab title="Em código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "remote-api": {
              type: "sse",
              url: "https://api.example.com/mcp/sse",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__remote-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "remote-api": {
                  "type": "sse",
                  "url": "https://api.example.com/mcp/sse",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__remote-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "remote-api": {
          "type": "sse",
          "url": "https://api.example.com/mcp/sse",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Para o transporte HTTP em fluxo contínuo, use `"type": "http"` em vez disso. Em `.mcp.json` e outros arquivos de configuração JSON, `"streamable-http"` é aceito como um alias para `"http"`. A opção programática `mcpServers` aceita apenas `"http"`.

<h3 id="sdk-mcp-servers">
  Servidores MCP SDK
</h3>

Defina ferramentas personalizadas diretamente no código da sua aplicação em vez de executar um processo de servidor separado. Consulte o [guia de ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools) para detalhes de implementação.

<h2 id="mcp-tool-search">
  Busca de ferramentas MCP
</h2>

Quando você tem muitas ferramentas MCP configuradas, as definições de ferramentas podem consumir uma porção significativa de sua janela de contexto. A busca de ferramentas resolve isso retendo as definições de ferramentas do contexto e carregando apenas as que Claude precisa para cada turno.

A busca de ferramentas está habilitada por padrão. Consulte [Busca de ferramentas](/docs/pt/agent-sdk/tool-search) para opções de configuração e detalhes.

Para mais detalhes, incluindo melhores práticas e uso de busca de ferramentas com ferramentas SDK personalizadas, consulte o [guia de busca de ferramentas](/docs/pt/agent-sdk/tool-search).

<h2 id="authentication">
  Autenticação
</h2>

A maioria dos servidores MCP requer autenticação para acessar serviços externos. Passe credenciais através de variáveis de ambiente na configuração do servidor.

<h3 id="pass-credentials-via-environment-variables">
  Passar credenciais via variáveis de ambiente
</h3>

Use o campo `env` para passar chaves de API, tokens e outras credenciais para o servidor MCP:

<Tabs>
  <Tab title="Em código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```

    A sintaxe `${GITHUB_TOKEN}` expande variáveis de ambiente em tempo de execução.
  </Tab>
</Tabs>

Consulte [Listar problemas de um repositório](#list-issues-from-a-repository) para um exemplo completo e funcional com registro de depuração.

<h3 id="http-headers-for-remote-servers">
  Cabeçalhos HTTP para servidores remotos
</h3>

Para servidores HTTP e SSE, passe cabeçalhos de autenticação diretamente na configuração do servidor:

<Tabs>
  <Tab title="Em código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    A sintaxe `${API_TOKEN}` expande variáveis de ambiente em tempo de execução.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  Autenticação OAuth2
</h3>

A [especificação MCP suporta OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) para autorização. O SDK não abre um navegador ou executa um fluxo OAuth interativo. Quando um servidor configurado retorna um desafio de autorização e nenhum token armazenado está disponível, a execução do agente continua sem as ferramentas desse servidor, e o servidor é relatado com status `needs-auth` no array `mcp_servers` da [mensagem de inicialização do sistema](/docs/pt/agent-sdk/typescript#sdksystemmessage). Verifique esse array na inicialização se seu agente depende de um servidor específico estar conectado.

Para fornecer credenciais, complete o fluxo OAuth em sua própria aplicação e passe o token de acesso resultante nos `headers` do servidor:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # After completing OAuth flow in your app
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  Exemplos
</h2>

<h3 id="list-issues-from-a-repository">
  Listar problemas de um repositório
</h3>

Este exemplo conecta ao [servidor GitHub MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/github) para listar problemas recentes. O exemplo inclui registro de depuração para verificar a conexão MCP e chamadas de ferramentas.

Antes de executar, crie um [token de acesso pessoal do GitHub](https://github.com/settings/tokens) com escopo `repo` e defina-o como uma variável de ambiente:

```bash theme={null}
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-github"],
          env: {
            GITHUB_TOKEN: process.env.GITHUB_TOKEN
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="query-a-database">
  Consultar um banco de dados
</h3>

Este exemplo usa o [servidor Postgres MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) para consultar um banco de dados. A string de conexão é passada como um argumento para o servidor. O agente descobre automaticamente o esquema do banco de dados, escreve a consulta SQL e retorna os resultados:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Connection string from environment variable
  const connectionString = process.env.DATABASE_URL;

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // Pass connection string as argument to the server
          args: ["-y", "@modelcontextprotocol/server-postgres", connectionString]
        }
      },
      // Allow only read queries, not writes
      allowedTools: ["mcp__postgres__query"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      # Connection string from environment variable
      connection_string = os.environ["DATABASE_URL"]

      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # Pass connection string as argument to the server
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-postgres",
                      connection_string,
                  ],
              }
          },
          # Allow only read queries, not writes
          allowed_tools=["mcp__postgres__query"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Tratamento de erros
</h2>

Os servidores MCP podem falhar ao conectar por vários motivos: o processo do servidor pode não estar instalado, as credenciais podem ser inválidas ou um servidor remoto pode estar inacessível.

O SDK emite uma mensagem `system` com subtipo `init` no início de cada consulta. Esta mensagem inclui o status de conexão para cada servidor MCP. Verifique o campo `status` para detectar falhas de conexão antes que o agente comece a trabalhar:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Process data",
    options: {
      mcpServers: {
        "data-processor": dataServer
      }
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      const failedServers = message.mcp_servers.filter((s) => s.status !== "connected");

      if (failedServers.length > 0) {
        console.warn("Failed to connect:", failedServers);
      }
    }

    if (message.type === "result" && message.subtype === "error_during_execution") {
      console.error("Execution failed");
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      async for message in query(prompt="Process data", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              failed_servers = [
                  s
                  for s in message.data.get("mcp_servers", [])
                  if s.get("status") != "connected"
              ]

              if failed_servers:
                  print(f"Failed to connect: {failed_servers}")

          if (
              isinstance(message, ResultMessage)
              and message.subtype == "error_during_execution"
          ):
              print("Execution failed")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="server-shows-failed-status">
  Servidor mostra status "failed"
</h3>

Verifique a mensagem `init` para ver quais servidores falharam ao conectar:

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Causas comuns:

* **Variáveis de ambiente ausentes**: Certifique-se de que tokens e credenciais necessários estão definidos. Para servidores stdio, verifique se o campo `env` corresponde ao que o servidor espera.
* **Servidor não instalado**: Para comandos `npx`, verifique se o pacote existe e se Node.js está em seu PATH.
* **String de conexão inválida**: Para servidores de banco de dados, verifique o formato da string de conexão e se o banco de dados está acessível.
* **Problemas de rede**: Para servidores HTTP/SSE remotos, verifique se a URL está acessível e se algum firewall permite a conexão.

<h3 id="tools-not-being-called">
  Ferramentas não sendo chamadas
</h3>

Se Claude vê ferramentas mas não as usa, verifique se você concedeu permissão com `allowedTools`:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
  }
};
```

<h3 id="connection-timeouts">
  Tempos limite de conexão
</h3>

As conexões do servidor MCP expiram após 30 segundos por padrão. Se seu servidor levar mais tempo para iniciar, a conexão falhará. Aumente o limite com a variável de ambiente [`MCP_TIMEOUT`](/docs/pt/env-vars), em milissegundos. Para servidores que precisam de mais tempo de inicialização, considere também:

* Usar um servidor mais leve se disponível
* Pré-aquecer o servidor antes de iniciar seu agente
* Verificar logs do servidor para causas de inicialização lenta

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Saída de ferramenta excede o máximo de tokens permitidos
</h3>

O SDK aplica o mesmo limite de saída MCP que Claude Code. Quando um resultado de ferramenta é maior que 25.000 tokens, a saída completa é salva em um arquivo e o resultado da ferramenta é substituído por uma mensagem de erro que nomeia o caminho do arquivo, para que o agente possa ler a saída novamente em porções. Aumente o limite com a variável de ambiente [`MAX_MCP_OUTPUT_TOKENS`](/docs/pt/env-vars). Veja [Limites de saída MCP e avisos](/docs/pt/mcp#mcp-output-limits-and-warnings) para o comportamento completo, incluindo como um servidor pode declarar um limite por ferramenta mais alto.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Guia de ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools)**: Construa seu próprio servidor MCP que é executado em processo com sua aplicação SDK
* **[Permissões](/docs/pt/agent-sdk/permissions)**: Controle quais ferramentas MCP seu agente pode usar com `allowedTools` e `disallowedTools`
* **[Limites de saída do MCP e avisos](/docs/pt/mcp#mcp-output-limits-and-warnings)**: Como o SDK lida com resultados de ferramentas que excedem `MAX_MCP_OUTPUT_TOKENS`, incluindo o fallback de persistência em disco e a anotação `anthropic/maxResultSizeChars` por ferramenta
* **[Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript)**: Referência completa da API incluindo opções de configuração de MCP
* **[Referência do SDK Python](/docs/pt/agent-sdk/python)**: Referência completa da API incluindo opções de configuração de MCP
* **[Diretório de servidores MCP](https://github.com/modelcontextprotocol/servers)**: Procure servidores MCP disponíveis para bancos de dados, APIs e muito mais
