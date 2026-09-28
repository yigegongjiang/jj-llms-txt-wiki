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

Passe servidores MCP diretamente na opção `mcpServers`. Este exemplo inicia um servidor MCP de sistema de arquivos local para `/Users/me/projects`. Substitua esse caminho por um diretório em sua máquina:

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

Crie um arquivo `.mcp.json` na raiz do seu projeto. O arquivo é detectado quando a fonte de configuração `project` está habilitada, o que é padrão para as opções `query()`. Se você definir `settingSources` explicitamente, inclua `"project"` para que este arquivo seja carregado. Substitua `/Users/me/projects` por um diretório em sua máquina:

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

<h2 id="connection-timing">
  Tempo de conexão
</h2>

Claude Code registra os servidores que você passa em `options.mcpServers` na inicialização e emite a [mensagem init](#error-handling) uma vez que a espera da primeira volta, se houver, seja resolvida. Se cada servidor `options.mcpServers` atrasa a primeira volta, e quando se conecta, depende do seu tipo:

| Tipo de servidor                                                                                     | Atrasa a primeira volta?                                              | Tempo limite de espera da primeira volta                                                                  |
| :--------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| Servidor stdio, ou servidor HTTP/SSE sem uma lista de ferramentas em cache                           | Sim, até que se conecte                                               | [`MCP_TIMEOUT`](/docs/pt/env-vars), 30 segundos por padrão; a conexão falha nesse prazo                        |
| Servidor remoto com uma lista de ferramentas em cache, salva por Claude Code de uma conexão anterior | Não; as ferramentas em cache estão disponíveis desde a primeira volta | Nenhum; conecta na sua primeira chamada de ferramenta, e essa conexão adiada tem seu próprio tempo limite |
| Servidor [SDK](#sdk-mcp-servers) em processo                                                         | Sim, até que se conecte e liste suas ferramentas                      | Nenhum; as solicitações de conexão e listagem de ferramentas têm seus próprios tempos limite              |

Servidores carregados de [arquivos de configuração](#from-a-config-file) como `.mcp.json` ou de plugins geralmente mostram `pending` na mensagem init. Quando `options.mcpServers` contém um servidor stdio, HTTP ou SSE, a primeira volta aguarda esses servidores pendentes também, até `MCP_TIMEOUT`. Quando `options.mcpServers` está vazio ou contém apenas servidores SDK, a primeira volta aguarda até 2 segundos em vez disso:

* **Com [busca de ferramentas](/docs/pt/agent-sdk/tool-search), o padrão**: a espera cobre servidores ainda pendentes configurados com [`alwaysLoad: true`](/docs/pt/mcp#exempt-a-server-from-deferral) e não o resto. O resto continua se conectando em segundo plano. [Disponibilidade de ferramentas](/docs/pt/mcp#tool-availability) descreve como Claude alcança suas ferramentas uma vez que se conectam.
* **Sem busca de ferramentas**: a espera cobre cada servidor pendente. [Configure a busca de ferramentas](/docs/pt/agent-sdk/tool-search#configure-tool-search) cobre o que desativa a busca de ferramentas. Se você excluir a ferramenta `ToolSearch` da sessão, por exemplo através de `disallowedTools`, a sessão também é executada sem busca de ferramentas.

Se você definir `permissionPromptToolName`, a primeira volta também aguarda o servidor dessa ferramenta em todos os casos, até `MCP_TIMEOUT`.

Para definir a espera da primeira volta você mesmo, adicione `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` à [opção `env`](/docs/pt/agent-sdk/configuration#set-environment-variables), por exemplo `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. A primeira volta então aguarda até esse número de milissegundos para cada servidor pendente, independentemente de a busca de ferramentas estar disponível ou não. Este prazo também substitui a espera da primeira volta `MCP_TIMEOUT` para servidores stdio, HTTP e SSE em `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` requer Claude Code v2.1.274 ou posterior.

Servidores ainda pendentes quando a espera termina continuam se conectando em segundo plano. Defina a variável como `0` para pular a espera. Um servidor `permissionPromptToolName` mantém sua própria espera `MCP_TIMEOUT` independentemente do valor.

Para bloquear a própria inicialização em uma fase separada e anterior à espera da primeira volta, antes da mensagem init ser enviada:

* Defina [`MCP_CONNECTION_NONBLOCKING`](/docs/pt/env-vars) como `0` para bloquear em todo o lote de conexão. Claude Code limita essa espera a 5 segundos por padrão. Ajuste o limite com a variável de ambiente [`MCP_CONNECT_TIMEOUT_MS`](/docs/pt/env-vars), em milissegundos. Servidores ainda pendentes nesse prazo continuam se conectando em segundo plano.
* Defina `alwaysLoad: true` na configuração de um servidor para disponibilizar suas ferramentas em seus esquemas completos na primeira volta, [isentos do adiamento de busca de ferramentas](/docs/pt/mcp#exempt-a-server-from-deferral). Claude Code aguarda na inicialização pelas ferramentas desse servidor, limitado ao mesmo prazo, enquanto outros servidores continuam se conectando em segundo plano; um servidor remoto com uma lista de ferramentas em cache as fornece sem se conectar, conforme a tabela acima.

A mensagem `system` com subtipo `init` relata o status de cada servidor no momento em que é emitida; consulte [Tratamento de erros](#error-handling) para ler esses status.

<h2 id="allow-mcp-tools">
  Permitir ferramentas MCP
</h2>

As ferramentas MCP requerem permissão explícita antes que Claude possa usá-las. Sem permissão, Claude verá que as ferramentas estão disponíveis, mas não poderá chamá-las.

<h3 id="tool-naming-convention">
  Convenção de nomenclatura de ferramentas
</h3>

As ferramentas MCP seguem o padrão de nomenclatura `mcp__<server-name>__<tool-name>`. Por exemplo, um servidor GitHub nomeado `"github"` com uma ferramenta `list_issues` se torna `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Auto-aprovação com allowedTools
</h3>

Use `allowedTools` para auto-aprovar ferramentas MCP específicas para que Claude possa usá-las sem um prompt de permissão:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
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

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=[
          "mcp__github__*",  # All tools from the github server
          "mcp__db__query",  # Only the query tool from db server
          "mcp__slack__send_message",  # Only send_message from slack server
      ],
  )
  ```
</CodeGroup>

Curingas (`*`) permitem que você aprove todas as ferramentas de um servidor sem listar cada uma individualmente.

<Note>
  **Prefira `allowedTools` em relação aos modos de permissão para acesso MCP.** `permissionMode: "acceptEdits"` não auto-aprova ferramentas MCP (apenas edições de arquivo e comandos Bash do sistema de arquivos). `permissionMode: "bypassPermissions"` auto-aprova ferramentas MCP, mas também desabilita a maioria dos outros prompts de segurança, o que é mais amplo do que o necessário; veja [Como as permissões são avaliadas](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) para os prompts que permanecem. Um curinga em `allowedTools` concede exatamente o servidor MCP que você deseja e nada mais. Veja [Modos de permissão](/docs/pt/agent-sdk/permissions#permission-modes) para uma comparação completa.
</Note>

<h3 id="discover-available-tools">
  Descobrir ferramentas disponíveis
</h3>

Para ver quais ferramentas um servidor MCP fornece, verifique a documentação do servidor ou inspecione o array `tools` na mensagem init `system`. Os nomes das ferramentas MCP começam com `mcp__`.

Claude Code emite a mensagem init após a [espera de conexão de primeira volta](#connection-timing) para servidores passados em `options.mcpServers`, então o array `tools` lista as ferramentas `mcp__` de cada servidor que se conectou até então, mais aquelas de servidores com uma [lista de ferramentas em cache](#connection-timing), que se conectam no primeiro uso. As ferramentas de qualquer outro servidor que não se conectou estão ausentes; veja [Tratamento de erros](#error-handling) para ler o status de cada servidor.

Este filtro imprime os nomes das ferramentas MCP:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    mcpServers: {
      // your servers
    },
  };

  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
      console.log("Available MCP tools:", mcpTools);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              # your servers
          },
      )
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
              print("Available MCP tools:", mcp_tools)


  asyncio.run(main())
  ```
</CodeGroup>

Você também pode pedir a Claude para listar as ferramentas disponíveis de um servidor.

<h2 id="transport-types">
  Tipos de transporte
</h2>

Os servidores MCP se comunicam com seu agente usando diferentes protocolos de transporte. Verifique a documentação do servidor para ver qual transporte ele suporta:

* Se a documentação fornece um **comando para executar** (como `npx @modelcontextprotocol/server-filesystem`), use stdio
* Se a documentação fornece uma **URL**, use HTTP ou SSE
* Se você está construindo suas próprias ferramentas em código, use um servidor MCP SDK

<h3 id="stdio-servers">
  Servidores stdio
</h3>

Processos locais que se comunicam via stdin/stdout. Use isso para servidores MCP que você executa na mesma máquina. Para o formulário `.mcp.json`, use os mesmos campos mostrados em [De um arquivo de configuração](#from-a-config-file). Em código, passe o comando e seus argumentos. Substitua `/Users/me/projects` por um diretório em sua máquina:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__read_file", "mcp__filesystem__list_directory"]
    }
  };
  ```

  ```python Python theme={null}
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
      allowed_tools=["mcp__filesystem__read_file", "mcp__filesystem__list_directory"],
  )
  ```
</CodeGroup>

<h3 id="http/sse-servers">
  Servidores HTTP/SSE
</h3>

Use HTTP ou SSE para servidores MCP hospedados em nuvem e APIs remotas. Para o formulário `.mcp.json`, use os mesmos campos do exemplo em [Cabeçalhos HTTP para servidores remotos](#http-headers-for-remote-servers), com `"type": "sse"` para um servidor SSE. Em código, passe a URL do servidor:

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

Para o transporte HTTP transmissível, use `"type": "http"` em vez disso. Em arquivos de configuração `.mcp.json` e outros JSON, `"streamable-http"` é aceito como um alias para `"http"`. O tipo `McpHttpServerConfig` dos SDKs declara apenas `"http"`, então use `"http"` para servidores que você passa em código.

<h3 id="sdk-mcp-servers">
  Servidores SDK MCP
</h3>

Defina ferramentas personalizadas diretamente no código da sua aplicação em vez de executar um processo de servidor separado. Consulte o [guia de ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools) para detalhes de implementação.

Um servidor MCP SDK registrado por uma [solicitação de controle `initialize`](/docs/pt/agent-sdk/typescript#sdkcontrolinitializeresponse) começa a se conectar assim que Claude Code processa a solicitação.

<h2 id="mcp-tool-search">
  Busca de ferramentas MCP
</h2>

Quando você tem muitas ferramentas MCP configuradas, as definições de ferramentas podem consumir uma porção significativa da sua janela de contexto. A busca de ferramentas resolve isso ao reter as definições de ferramentas do contexto e carregar apenas as que Claude precisa para cada turno.

A busca de ferramentas está ativada por padrão. Consulte [Busca de ferramentas](/docs/pt/agent-sdk/tool-search) para opções de configuração, melhores práticas e uso da busca de ferramentas com ferramentas SDK personalizadas.

<h2 id="authentication">
  Autenticação
</h2>

A maioria dos servidores MCP requer autenticação para acessar serviços externos. Passe credenciais através de variáveis de ambiente na configuração do servidor.

<h3 id="pass-credentials-via-environment-variables">
  Passar credenciais via variáveis de ambiente
</h3>

Use o campo `env` para passar chaves de API, tokens e outras credenciais para o servidor MCP:

<Tabs>
  <Tab title="No código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "api-server": {
              command: "npx",
              args: ["-y", "@your-org/api-mcp-server"],
              env: {
                API_KEY: process.env.API_KEY
              }
            }
          },
          allowedTools: ["mcp__api-server__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "api-server": {
                  "command": "npx",
                  "args": ["-y", "@your-org/api-mcp-server"],
                  "env": {"API_KEY": os.environ["API_KEY"]},
              }
          },
          allowed_tools=["mcp__api-server__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "api-server": {
          "command": "npx",
          "args": ["-y", "@your-org/api-mcp-server"],
          "env": {
            "API_KEY": "${API_KEY}"
          }
        }
      }
    }
    ```

    A sintaxe `${API_KEY}` expande variáveis de ambiente em tempo de execução.
  </Tab>
</Tabs>

<h3 id="http-headers-for-remote-servers">
  Cabeçalhos HTTP para servidores remotos
</h3>

Para servidores HTTP e SSE, passe cabeçalhos de autenticação diretamente na configuração do servidor:

<Tabs>
  <Tab title="No código">
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

Para um exemplo completo de funcionamento de um servidor remoto autenticado com cabeçalhos, consulte [Listar problemas de um repositório](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  Autenticação OAuth2
</h3>

A [especificação MCP suporta OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) para autorização. O SDK não abre um navegador nem executa um fluxo OAuth interativo. Quando um servidor configurado retorna um desafio de autorização e nenhum token armazenado está disponível, a execução do agente continua sem as ferramentas desse servidor, e o servidor relata o status `needs-auth`. O array `mcp_servers` da [mensagem de inicialização do sistema](/docs/pt/agent-sdk/typescript#sdksystemmessage) ainda pode mostrar `pending` para esse servidor quando for emitido. Para confirmar se um servidor precisa de credenciais, consulte `mcpServerStatus()` no SDK TypeScript ou [`get_mcp_status()`](/docs/pt/agent-sdk/python#methods) em Python.

Para fornecer credenciais, complete o fluxo OAuth em sua própria aplicação e passe o token de acesso resultante nos `headers` do servidor:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Após completar o fluxo OAuth em sua aplicação.
  // Implemente getAccessTokenFromOAuthFlow para seu provedor OAuth.
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
  # Após completar o fluxo OAuth em sua aplicação.
  # Implemente get_access_token_from_oauth_flow para seu provedor OAuth.
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

Este exemplo se conecta ao [servidor MCP do GitHub](https://github.com/github/github-mcp-server) remoto para listar problemas recentes. O exemplo inclui registro de depuração para verificar a conexão MCP e as chamadas de ferramentas.

Antes de executar, crie um [token de acesso pessoal do GitHub](https://github.com/settings/personal-access-tokens) com acesso de leitura aos repositórios que você deseja consultar e defina-o como uma variável de ambiente:

```bash theme={null}
export GITHUB_TOKEN=YOUR_GITHUB_PAT
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          type: "http",
          url: "https://api.githubcopilot.com/mcp/",
          headers: {
            Authorization: `Bearer ${process.env.GITHUB_TOKEN}`
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
                  "type": "http",
                  "url": "https://api.githubcopilot.com/mcp/",
                  "headers": {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"},
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

Na linha `MCP servers:`, um `status` de `connected` para `github` confirma que o token funciona. Se Claude Code tiver uma [lista de ferramentas em cache](#connection-timing) para o servidor, o status pode ler `pending` em vez disso e o servidor se conecta na sua primeira chamada de ferramenta. Se o status for `failed` ou `needs-auth`, consulte [Tratamento de erros](#error-handling) antes de confiar no resultado, pois Claude pode voltar para ferramentas integradas quando o servidor não está disponível.

<h3 id="query-a-database">
  Consultar um banco de dados
</h3>

Este exemplo usa [DBHub](https://github.com/bytebase/dbhub) para consultar um banco de dados Postgres. O agente descobre automaticamente o esquema do banco de dados, escreve a consulta SQL e retorna os resultados.

A ferramenta `execute_sql` do DBHub executa qualquer SQL que o agente emita, incluindo gravações, a menos que você o restrinja. Definir `readonly = true` no [arquivo de configuração do DBHub](https://dbhub.ai/config/toml) faz com que o DBHub rejeite instruções `INSERT`, `UPDATE`, `DELETE` e DDL, para que o exemplo não possa modificar seus dados mesmo se o agente emitir uma gravação. O DBHub resolve `${DATABASE_URL}` do ambiente do processo quando carrega a configuração, portanto a string de conexão fica fora do arquivo. Crie este `dbhub.toml` ao lado do seu script:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

O script então aponta o DBHub para o arquivo de configuração em vez de passar uma string de conexão diretamente. Antes de executar, defina a variável de ambiente `DATABASE_URL` para sua string de conexão. Substitua os valores de espaço reservado pelos detalhes do seu banco de dados:

```bash theme={null}
export DATABASE_URL=postgresql://user:password@localhost:5432/mydb
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // dbhub.toml sets readonly = true, so execute_sql rejects writes
          args: ["-y", "@bytebase/dbhub", "--config", "dbhub.toml"]
        }
      },
      allowedTools: ["mcp__postgres__execute_sql"]
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
              "postgres": {
                  "command": "npx",
                  # dbhub.toml sets readonly = true, so execute_sql rejects writes
                  "args": [
                      "-y",
                      "@bytebase/dbhub",
                      "--config",
                      "dbhub.toml",
                  ],
              }
          },
          allowed_tools=["mcp__postgres__execute_sql"],
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

Os servidores MCP podem falhar ao conectar por várias razões: o processo do servidor pode não estar instalado, as credenciais podem ser inválidas ou um servidor remoto pode estar inacessível.

Claude Code emite uma mensagem `system` com subtipo `init` no início de cada consulta. Esta mensagem inclui o status de conexão para cada servidor MCP. O campo `status` pode ser `"pending"`, `"connected"`, `"failed"`, `"needs-auth"` ou `"disabled"`. Claude Code emite a mensagem init após o [tempo de espera de conexão da primeira volta](#connection-timing) para servidores passados em `options.mcpServers`, portanto, tal servidor que se conectou dentro do tempo de espera mostra `"connected"`.

Na mensagem init, não trate `"pending"` como uma falha por si só. Pode significar qualquer um destes:

* O servidor ainda não se conectou. Veja [quanto tempo Claude Code espera por ele antes da primeira volta](#connection-timing)
* A lista de ferramentas do servidor foi [servida do cache](#connection-timing), com uma conexão feita no primeiro uso
* O prazo de conexão expirou. Tal servidor relata `"pending"` ou `"failed"` dependendo do tempo

Verifique `"failed"` ou `"needs-auth"` para detectar servidores que não serão utilizáveis:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Process data",
      options: {
        mcpServers: {
          // Replace dataServer with your server configuration
          "data-processor": dataServer
        }
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        const unavailableServers = message.mcp_servers.filter(
          (s) => s.status === "failed" || s.status === "needs-auth"
        );

        if (unavailableServers.length > 0) {
          console.warn("Unavailable MCP servers:", unavailableServers);
        }
      }

      if (message.type === "result" && message.subtype === "error_during_execution") {
        console.error("Execution failed");
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branch above has
    // already run; a failure to start or reach the Claude Code process
    // yields no result message. MCP servers that fail to connect don't
    // throw: use the status check above, and note that servers still
    // "pending" at init need a later status check.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      # Replace data_server with your server configuration
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      try:
          async for message in query(prompt="Process data", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  unavailable_servers = [
                      s
                      for s in message.data.get("mcp_servers", [])
                      if s.get("status") in ("failed", "needs-auth")
                  ]

                  if unavailable_servers:
                      print(f"Unavailable MCP servers: {unavailable_servers}")

              if (
                  isinstance(message, ResultMessage)
                  and message.subtype == "error_during_execution"
              ):
                  print("Execution failed")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branch above has
          # already run; a failure to start or reach the Claude Code process
          # yields no result message. MCP servers that fail to connect don't
          # raise: use the status check above, and note that servers still
          # "pending" at init need a later status check.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

O status de um servidor remoto também pode mudar após relatar `"connected"`. Quando a conexão com ele cai no meio da sessão, Claude Code move o servidor de volta para `"pending"` enquanto [reconecta](/docs/pt/mcp#automatic-reconnection). Uma chamada posterior a `mcpServerStatus()` em TypeScript, ou [`ClaudeSDKClient.get_mcp_status()`](/docs/pt/agent-sdk/python#methods) em Python, pode então relatar `"pending"` para um servidor que você viu conectado anteriormente, sem nenhuma mudança de configuração do seu lado.

Após cinco tentativas de reconexão falharem, o servidor relata `"failed"` ou `"needs-auth"` quando precisa ser autorizado novamente. Para tentar novamente manualmente, chame [`reconnectMcpServer()`](/docs/pt/agent-sdk/typescript#methods) em TypeScript ou [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/pt/agent-sdk/python#methods) em Python.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Server shows "failed" status
</h3>

Verifique a mensagem `init` para ver quais servidores falharam ao conectar:

<CodeGroup>
  ```typescript TypeScript theme={null}
  if (message.type === "system" && message.subtype === "init") {
    for (const server of message.mcp_servers) {
      if (server.status === "failed") {
        console.error(`Server ${server.name} failed to connect`);
      }
    }
  }
  ```

  ```python Python theme={null}
  if isinstance(message, SystemMessage) and message.subtype == "init":
      for server in message.data.get("mcp_servers", []):
          if server.get("status") == "failed":
              print(f"Server {server['name']} failed to connect")
  ```
</CodeGroup>

Um status `"pending"` não significa que o servidor falhou. Consulte [Tratamento de erros](#error-handling) para os casos que ele cobre na inicialização. Para obter status atualizados posteriormente na sessão, chame o método `mcpServerStatus()` da consulta no SDK TypeScript, ou [`ClaudeSDKClient.get_mcp_status()`](/docs/pt/agent-sdk/python#methods) em Python.

Causas comuns:

* **Variáveis de ambiente ausentes**: Certifique-se de que tokens e credenciais necessários estejam definidos. Para servidores stdio, verifique se o campo `env` corresponde ao que o servidor espera.
* **Servidor não instalado**: Para comandos `npx`, verifique se o pacote existe e se Node.js está no seu PATH.
* **String de conexão inválida**: Para servidores de banco de dados, verifique o formato da string de conexão e se o banco de dados está acessível.
* **Problemas de rede**: Para servidores HTTP/SSE remotos, verifique se a URL está acessível e se qualquer firewall permite a conexão.

<h3 id="tools-not-being-called">
  Tools not being called
</h3>

Se Claude vir ferramentas mas não as usar, verifique se você concedeu permissão com `allowedTools`:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=["mcp__servername__*"],  # Auto-approve calls from this server
  )
  ```
</CodeGroup>

<h3 id="connection-timeouts">
  Connection timeouts
</h3>

As conexões do servidor MCP expiram após 30 segundos por padrão. Para alterar quanto tempo uma chamada de ferramenta em execução pode levar, defina [`MCP_TOOL_TIMEOUT`](/docs/pt/env-vars). Se seu servidor levar mais tempo para iniciar, a conexão falhará. Aumente o limite de conexão com a variável de ambiente [`MCP_TIMEOUT`](/docs/pt/env-vars), em milissegundos. Para servidores que precisam de mais tempo de inicialização, também considere:

* Usar um servidor mais leve, se disponível
* Pré-aquecer o servidor antes de iniciar seu agente
* Verificar os logs do servidor para causas de inicialização lenta

Em TypeScript, você pode definir o limite de chamada de ferramenta para um único [servidor MCP do SDK](#sdk-mcp-servers) passando [`timeout` para `createSdkMcpServer()`](/docs/pt/agent-sdk/typescript#createsdkmcpserver).

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Tool output exceeds maximum allowed tokens
</h3>

O SDK aplica o mesmo limite de saída MCP que Claude Code. Quando um resultado de ferramenta sem conteúdo de imagem é maior que 25.000 tokens, Claude Code salva a saída em um arquivo e substitui o resultado da ferramenta por uma mensagem de erro que nomeia o caminho do arquivo, para que o agente possa ler a saída novamente em porções.

Aumente o limite com a variável de ambiente [`MAX_MCP_OUTPUT_TOKENS`](/docs/pt/env-vars). Consulte [Limites de saída MCP e avisos](/docs/pt/mcp#mcp-output-limits-and-warnings) para o comportamento completo, incluindo como um servidor pode declarar um limite por ferramenta mais alto com a anotação `anthropic/maxResultSizeChars`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Guia de ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools)**: Crie seu próprio servidor MCP que é executado em processo com sua aplicação SDK
* **[Permissões](/docs/pt/agent-sdk/permissions)**: Controle quais ferramentas MCP seu agente pode usar com `allowedTools` e `disallowedTools`
* **[Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript)**: Referência completa da API incluindo opções de configuração do MCP
* **[Referência do SDK Python](/docs/pt/agent-sdk/python)**: Referência completa da API incluindo opções de configuração do MCP
* **[Diretório de servidores MCP](https://github.com/modelcontextprotocol/servers)**: Navegue pelos servidores MCP disponíveis para bancos de dados, APIs e muito mais
