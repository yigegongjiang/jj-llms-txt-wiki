> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connettiti a strumenti esterni con MCP

> Configura i server MCP per estendere il tuo agente con strumenti esterni. Copre i tipi di trasporto, la ricerca di strumenti per set di strumenti di grandi dimensioni, l'autenticazione e la gestione degli errori.

Il [Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) è uno standard aperto per connettere agenti AI a strumenti e fonti di dati esterni. Con MCP, il tuo agente può interrogare database, integrarsi con API come Slack e GitHub e connettersi ad altri servizi senza scrivere implementazioni di strumenti personalizzate.

I server MCP possono essere eseguiti come processi locali, connettersi tramite HTTP o essere eseguiti direttamente all'interno della tua applicazione SDK.

<Note>
  Questa pagina copre la configurazione di MCP per l'Agent SDK. Per aggiungere server MCP al Claude Code CLI in modo che si carichino in ogni progetto, consulta [Ambiti di installazione di MCP](/docs/it/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Quickstart
</h2>

Questo esempio si connette al server MCP della [documentazione di Claude Code](https://code.claude.com/docs) utilizzando il [trasporto HTTP](#http%2Fsse-servers) e utilizza [`allowedTools`](#allow-mcp-tools) con un carattere jolly per consentire tutti gli strumenti dal server.

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

L'agente si connette al server di documentazione, cerca informazioni su hooks e restituisce i risultati.

<h2 id="add-an-mcp-server">
  Aggiungi un server MCP
</h2>

Puoi configurare i server MCP nel codice quando chiami `query()`, oppure in un file `.mcp.json` caricato tramite [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  Nel codice
</h3>

Passa i server MCP direttamente nell'opzione `mcpServers`:

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
  Da un file di configurazione
</h3>

Crea un file `.mcp.json` nella radice del tuo progetto. Il file viene rilevato quando la fonte di impostazione `project` è abilitata, il che è il caso per le opzioni predefinite di `query()`. Se imposti `settingSources` esplicitamente, includi `"project"` affinché questo file venga caricato:

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
  Consenti strumenti MCP
</h2>

Gli strumenti MCP richiedono un'autorizzazione esplicita prima che Claude possa utilizzarli. Senza autorizzazione, Claude vedrà che gli strumenti sono disponibili ma non sarà in grado di chiamarli.

<h3 id="tool-naming-convention">
  Convenzione di denominazione degli strumenti
</h3>

Gli strumenti MCP seguono il modello di denominazione `mcp__<server-name>__<tool-name>`. Ad esempio, un server GitHub denominato `"github"` con uno strumento `list_issues` diventa `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Approvazione automatica con allowedTools
</h3>

Utilizza `allowedTools` per approvare automaticamente strumenti MCP specifici in modo che Claude possa utilizzarli senza un prompt di autorizzazione:

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

I caratteri jolly (`*`) ti permettono di consentire tutti gli strumenti da un server senza elencare ciascuno individualmente.

<Note>
  **Preferisci `allowedTools` rispetto alle modalità di autorizzazione per l'accesso a MCP.** `permissionMode: "acceptEdits"` non approva automaticamente gli strumenti MCP (solo le modifiche ai file e i comandi Bash del filesystem). `permissionMode: "bypassPermissions"` approva automaticamente gli strumenti MCP ma disabilita anche tutti gli altri prompt di sicurezza, il che è più ampio del necessario; consulta [Come vengono valutate le autorizzazioni](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) per i prompt che rimangono. Un carattere jolly in `allowedTools` concede esattamente il server MCP che desideri e niente di più. Consulta [Modalità di autorizzazione](/docs/it/agent-sdk/permissions#permission-modes) per un confronto completo.
</Note>

<h3 id="discover-available-tools">
  Scopri gli strumenti disponibili
</h3>

Per vedere quali strumenti fornisce un server MCP, controlla la documentazione del server o connettiti al server e ispeziona il messaggio di inizializzazione `system`:

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
  Tipi di trasporto
</h2>

I server MCP comunicano con il tuo agente utilizzando diversi protocolli di trasporto. Controlla la documentazione del server per vedere quale trasporto supporta:

* Se la documentazione ti fornisce un **comando da eseguire** (come `npx @modelcontextprotocol/server-github`), utilizza stdio
* Se la documentazione ti fornisce un **URL**, utilizza HTTP o SSE
* Se stai costruendo i tuoi strumenti personalizzati nel codice, utilizza un server MCP SDK

<h3 id="stdio-servers">
  Server stdio
</h3>

Processi locali che comunicano tramite stdin/stdout. Utilizza questo per i server MCP che esegui sulla stessa macchina:

<Tabs>
  <Tab title="Nel codice">
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
  Server HTTP/SSE
</h3>

Utilizza HTTP o SSE per i server MCP ospitati nel cloud e le API remote:

<Tabs>
  <Tab title="Nel codice">
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

Per il trasporto HTTP in streaming, utilizza `"type": "http"` invece. Nei file di configurazione `.mcp.json` e altri file JSON, `"streamable-http"` è accettato come alias per `"http"`. L'opzione programmatica `mcpServers` accetta solo `"http"`.

<h3 id="sdk-mcp-servers">
  Server MCP SDK
</h3>

Definisci strumenti personalizzati direttamente nel codice della tua applicazione invece di eseguire un processo server separato. Consulta la [guida agli strumenti personalizzati](/docs/it/agent-sdk/custom-tools) per i dettagli di implementazione.

<h2 id="mcp-tool-search">
  Ricerca di strumenti MCP
</h2>

Quando hai molti strumenti MCP configurati, le definizioni degli strumenti possono consumare una parte significativa della tua finestra di contesto. La ricerca di strumenti risolve questo problema trattenendo le definizioni degli strumenti dal contesto e caricando solo quelli di cui Claude ha bisogno per ogni turno.

La ricerca di strumenti è abilitata per impostazione predefinita. Consulta [Ricerca di strumenti](/docs/it/agent-sdk/tool-search) per le opzioni di configurazione e i dettagli.

Per ulteriori dettagli, incluse le best practice e l'utilizzo della ricerca di strumenti con strumenti SDK personalizzati, consulta la [guida alla ricerca di strumenti](/docs/it/agent-sdk/tool-search).

<h2 id="authentication">
  Autenticazione
</h2>

La maggior parte dei server MCP richiede l'autenticazione per accedere ai servizi esterni. Passa le credenziali tramite variabili di ambiente nella configurazione del server.

<h3 id="pass-credentials-via-environment-variables">
  Passa le credenziali tramite variabili di ambiente
</h3>

Utilizza il campo `env` per passare chiavi API, token e altre credenziali al server MCP:

<Tabs>
  <Tab title="Nel codice">
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

    La sintassi `${GITHUB_TOKEN}` espande le variabili di ambiente in fase di esecuzione.
  </Tab>
</Tabs>

Consulta [Elenca i problemi da un repository](#list-issues-from-a-repository) per un esempio completo e funzionante con registrazione di debug.

<h3 id="http-headers-for-remote-servers">
  Intestazioni HTTP per server remoti
</h3>

Per i server HTTP e SSE, passa le intestazioni di autenticazione direttamente nella configurazione del server:

<Tabs>
  <Tab title="Nel codice">
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

    La sintassi `${API_TOKEN}` espande le variabili di ambiente in fase di esecuzione.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  Autenticazione OAuth2
</h3>

La [specifica MCP supporta OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) per l'autorizzazione. L'SDK non apre un browser né esegue un flusso OAuth interattivo. Quando un server configurato restituisce una sfida di autorizzazione e nessun token memorizzato è disponibile, l'esecuzione dell'agente continua senza gli strumenti di quel server, e il server viene segnalato con lo stato `needs-auth` nell'array `mcp_servers` del [messaggio di inizializzazione del sistema](/docs/it/agent-sdk/typescript#sdksystemmessage). Controlla quell'array all'avvio se il tuo agente dipende da uno specifico server connesso.

Per fornire le credenziali, completa il flusso OAuth nella tua applicazione e passa il token di accesso risultante nelle `headers` del server:

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
  Esempi
</h2>

<h3 id="list-issues-from-a-repository">
  Elenca i problemi da un repository
</h3>

Questo esempio si connette al [server GitHub MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/github) per elencare i problemi recenti. L'esempio include la registrazione di debug per verificare la connessione MCP e le chiamate agli strumenti.

Prima di eseguire, crea un [token di accesso personale GitHub](https://github.com/settings/tokens) con ambito `repo` e impostalo come variabile di ambiente:

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
  Interroga un database
</h3>

Questo esempio utilizza il [server Postgres MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) per interrogare un database. La stringa di connessione viene passata come argomento al server. L'agente scopre automaticamente lo schema del database, scrive la query SQL e restituisce i risultati:

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
  Gestione degli errori
</h2>

I server MCP possono non riuscire a connettersi per vari motivi: il processo del server potrebbe non essere installato, le credenziali potrebbero non essere valide o un server remoto potrebbe essere irraggiungibile.

L'SDK emette un messaggio `system` con sottotipo `init` all'inizio di ogni query. Questo messaggio include lo stato della connessione per ogni server MCP. Controlla il campo `status` per rilevare gli errori di connessione prima che l'agente inizi a lavorare:

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
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Il server mostra lo stato "failed"
</h3>

Controlla il messaggio `init` per vedere quali server non hanno potuto connettersi:

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Cause comuni:

* **Variabili di ambiente mancanti**: Assicurati che i token e le credenziali richiesti siano impostati. Per i server stdio, controlla che il campo `env` corrisponda a quello che il server si aspetta.
* **Server non installato**: Per i comandi `npx`, verifica che il pacchetto esista e che Node.js sia nel tuo PATH.
* **Stringa di connessione non valida**: Per i server di database, verifica il formato della stringa di connessione e che il database sia accessibile.
* **Problemi di rete**: Per i server HTTP/SSE remoti, controlla che l'URL sia raggiungibile e che eventuali firewall consentano la connessione.

<h3 id="tools-not-being-called">
  Gli strumenti non vengono chiamati
</h3>

Se Claude vede gli strumenti ma non li utilizza, controlla di aver concesso l'autorizzazione con `allowedTools`:

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
  Timeout della connessione
</h3>

Le connessioni del server MCP hanno un timeout predefinito di 30 secondi. Se il tuo server impiega più tempo per avviarsi, la connessione avrà esito negativo. Aumenta il limite con la variabile di ambiente [`MCP_TIMEOUT`](/docs/it/env-vars), in millisecondi. Per i server che hanno bisogno di più tempo di avvio, considera anche:

* Utilizzare un server più leggero se disponibile
* Pre-riscaldare il server prima di avviare il tuo agente
* Controllare i log del server per le cause di inizializzazione lenta

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  L'output dello strumento supera il numero massimo di token consentiti
</h3>

L'SDK applica lo stesso limite di output MCP di Claude Code. Quando il risultato di uno strumento è più grande di 25.000 token, l'output completo viene salvato in un file e il risultato dello strumento viene sostituito con un messaggio di errore che indica il percorso del file, in modo che l'agente possa leggere l'output in porzioni. Aumenta il limite con la variabile di ambiente [`MAX_MCP_OUTPUT_TOKENS`](/docs/it/env-vars). Vedi [Limiti di output MCP e avvisi](/docs/it/mcp#mcp-output-limits-and-warnings) per il comportamento completo, incluso come un server può dichiarare un limite per strumento più elevato.

<h2 id="related-resources">
  Risorse correlate
</h2>

* **[Guida agli strumenti personalizzati](/docs/it/agent-sdk/custom-tools)**: Costruisci il tuo server MCP che viene eseguito in-process con la tua applicazione SDK
* **[Autorizzazioni](/docs/it/agent-sdk/permissions)**: Controlla quali strumenti MCP il tuo agente può utilizzare con `allowedTools` e `disallowedTools`
* **[Limiti di output MCP e avvisi](/docs/it/mcp#mcp-output-limits-and-warnings)**: Come l'SDK gestisce i risultati degli strumenti che superano `MAX_MCP_OUTPUT_TOKENS`, incluso il fallback persist-to-disk e l'annotazione per-tool `anthropic/maxResultSizeChars`
* **[Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript)**: Riferimento API completo incluse le opzioni di configurazione di MCP
* **[Riferimento SDK Python](/docs/it/agent-sdk/python)**: Riferimento API completo incluse le opzioni di configurazione di MCP
* **[Directory dei server MCP](https://github.com/modelcontextprotocol/servers)**: Sfoglia i server MCP disponibili per database, API e altro
