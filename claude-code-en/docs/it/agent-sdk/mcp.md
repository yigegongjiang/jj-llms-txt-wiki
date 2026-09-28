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
  Aggiungere un server MCP
</h2>

È possibile configurare i server MCP nel codice quando si chiama `query()`, oppure in un file `.mcp.json` caricato tramite [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  Nel codice
</h3>

Passare i server MCP direttamente nell'opzione `mcpServers`. Questo esempio avvia un server MCP del filesystem locale per `/Users/me/projects`. Sostituire quel percorso con una directory sulla propria macchina:

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

Creare un file `.mcp.json` nella radice del progetto. Il file viene rilevato quando la sorgente di impostazione `project` è abilitata, il che avviene per le opzioni predefinite di `query()`. Se si imposta `settingSources` esplicitamente, includere `"project"` affinché questo file venga caricato. Sostituire `/Users/me/projects` con una directory sulla propria macchina:

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
  Tempistica della connessione
</h2>

Claude Code registra i server che passate in `options.mcpServers` all'avvio e invia il [messaggio init](#error-handling) una volta che l'attesa del primo turno, se presente, si risolve. Se ogni server `options.mcpServers` ritarda il primo turno, e quando si connette, dipende dal suo tipo:

| Tipo di server                                                                                                         | Ritarda il primo turno?                                                    | Timeout di attesa del primo turno                                                                                   |
| :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| server stdio, o server HTTP/SSE senza un elenco di strumenti memorizzato nella cache                                   | Sì, fino a quando non si connette                                          | [`MCP_TIMEOUT`](/docs/it/env-vars), 30 secondi per impostazione predefinita; la connessione non riesce a quella scadenza |
| Server remoto con un elenco di strumenti memorizzato nella cache, salvato da Claude Code da una connessione precedente | No; gli strumenti memorizzati nella cache sono disponibili dal primo turno | Nessuno; si connette alla sua prima chiamata di strumento, e quella connessione differita ha il suo proprio timeout |
| Server [SDK](#sdk-mcp-servers) in-process                                                                              | Sì, fino a quando non si connette e non elenca i suoi strumenti            | Nessuno; le richieste di connessione e di elenco degli strumenti hanno ciascuna il loro proprio timeout             |

I server caricati da [file di configurazione](#from-a-config-file) come `.mcp.json` o da plugin comunemente mostrano `pending` nel messaggio init. Quando `options.mcpServers` contiene un server stdio, HTTP o SSE, il primo turno attende anche questi server in sospeso, fino a `MCP_TIMEOUT`. Quando `options.mcpServers` è vuoto o contiene solo server SDK, il primo turno attende invece fino a 2 secondi:

* **Con [ricerca degli strumenti](/docs/it/agent-sdk/tool-search), l'impostazione predefinita**: l'attesa copre i server ancora in sospeso configurati con [`alwaysLoad: true`](/docs/it/mcp#exempt-a-server-from-deferral) e non il resto. Il resto continua a connettersi in background. [Disponibilità degli strumenti](/docs/it/mcp#tool-availability) descrive come Claude raggiunge i loro strumenti una volta che si connettono.
* **Senza ricerca degli strumenti**: l'attesa copre ogni server in sospeso. [Configurare la ricerca degli strumenti](/docs/it/agent-sdk/tool-search#configure-tool-search) copre cosa disattiva la ricerca degli strumenti. Se escludete lo strumento `ToolSearch` dalla sessione, ad esempio tramite `disallowedTools`, la sessione viene eseguita anche senza ricerca degli strumenti.

Se impostate `permissionPromptToolName`, il primo turno attende anche il server di quello strumento in ogni caso, fino a `MCP_TIMEOUT`.

Per impostare voi stessi l'attesa del primo turno, aggiungete `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` all'[opzione `env`](/docs/it/agent-sdk/configuration#set-environment-variables), ad esempio `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. Il primo turno attende quindi fino a quel numero di millisecondi per ogni server in sospeso, indipendentemente dal fatto che la ricerca degli strumenti sia disponibile o meno. Questa scadenza sostituisce anche l'attesa del primo turno `MCP_TIMEOUT` per i server stdio, HTTP e SSE in `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` richiede Claude Code v2.1.274 o successivo.

I server ancora in sospeso quando l'attesa termina continuano a connettersi in background. Impostate la variabile a `0` per saltare l'attesa. Un server `permissionPromptToolName` mantiene la sua propria attesa `MCP_TIMEOUT` indipendentemente dal valore.

Per bloccare l'avvio stesso in una fase separata e precedente rispetto all'attesa del primo turno, prima che il messaggio init sia inviato:

* Impostare [`MCP_CONNECTION_NONBLOCKING`](/docs/it/env-vars) a `0` per bloccare l'intero batch di connessione. Claude Code limita tale attesa a 5 secondi per impostazione predefinita. Regolare il limite con la variabile di ambiente [`MCP_CONNECT_TIMEOUT_MS`](/docs/it/env-vars), in millisecondi. I server ancora in sospeso a quella scadenza continuano a connettersi in background.
* Impostare `alwaysLoad: true` sulla configurazione di un server per rendere i suoi strumenti disponibili ai loro schemi completi al primo turno, [esenti dal differimento della ricerca degli strumenti](/docs/it/mcp#exempt-a-server-from-deferral). Claude Code attende all'avvio gli strumenti di quel server, limitati alla stessa scadenza, mentre gli altri server continuano a connettersi in background; un server remoto con un elenco di strumenti memorizzato nella cache li fornisce senza connettersi, secondo la tabella sopra.

Il messaggio `system` con sottotipo `init` segnala lo stato di ogni server nel momento in cui viene emesso; vedere [Gestione degli errori](#error-handling) per leggere questi stati.

<h2 id="allow-mcp-tools">
  Consenti strumenti MCP
</h2>

Gli strumenti MCP richiedono un'autorizzazione esplicita prima che Claude possa utilizzarli. Senza autorizzazione, Claude vedrà che gli strumenti sono disponibili ma non sarà in grado di chiamarli.

<h3 id="tool-naming-convention">
  Convenzione di denominazione degli strumenti
</h3>

Gli strumenti MCP seguono il modello di denominazione `mcp__<server-name>__<tool-name>`. Ad esempio, un server GitHub denominato `"github"` con uno strumento `list_issues` diventa `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Auto-approvazione con allowedTools
</h3>

Utilizzare `allowedTools` per approvare automaticamente strumenti MCP specifici in modo che Claude possa utilizzarli senza un prompt di autorizzazione:

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

I caratteri jolly (`*`) consentono di approvare tutti gli strumenti da un server senza elencare singolarmente ciascuno.

<Note>
  **Preferire `allowedTools` rispetto alle modalità di autorizzazione per l'accesso MCP.** `permissionMode: "acceptEdits"` non approva automaticamente gli strumenti MCP (solo le modifiche ai file e i comandi Bash del filesystem). `permissionMode: "bypassPermissions"` approva automaticamente gli strumenti MCP ma disabilita anche la maggior parte degli altri prompt di sicurezza, il che è più ampio del necessario; vedere [Come vengono valutate le autorizzazioni](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) per i prompt che rimangono. Un carattere jolly in `allowedTools` concede esattamente il server MCP che desiderate e nient'altro. Vedere [Modalità di autorizzazione](/docs/it/agent-sdk/permissions#permission-modes) per un confronto completo.
</Note>

<h3 id="discover-available-tools">
  Scopri gli strumenti disponibili
</h3>

Per vedere quali strumenti fornisce un server MCP, controllare la documentazione del server o ispezionare l'array `tools` nel messaggio di inizializzazione `system`. I nomi degli strumenti MCP iniziano con `mcp__`.

Claude Code emette il messaggio di inizializzazione dopo l'[attesa di connessione al primo turno](#connection-timing) per i server passati in `options.mcpServers`, quindi l'array `tools` elenca gli strumenti `mcp__` di ciascun server che si è connesso entro quel momento, più quelli dei server con un [elenco di strumenti memorizzato nella cache](#connection-timing), che si connettono al primo utilizzo. Gli strumenti di qualsiasi altro server che non si è ancora connesso sono assenti; vedere [Gestione degli errori](#error-handling) per leggere lo stato di ciascun server.

Questo filtro stampa i nomi degli strumenti MCP:

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

Potete anche chiedere a Claude di elencare gli strumenti disponibili da un server.

<h2 id="transport-types">
  Tipi di trasporto
</h2>

I server MCP comunicano con il vostro agente utilizzando diversi protocolli di trasporto. Controllate la documentazione del server per vedere quale trasporto supporta:

* Se la documentazione vi fornisce un **comando da eseguire** (come `npx @modelcontextprotocol/server-filesystem`), utilizzate stdio
* Se la documentazione vi fornisce un **URL**, utilizzate HTTP o SSE
* Se state costruendo i vostri strumenti personalizzati nel codice, utilizzate un server MCP SDK

<h3 id="stdio-servers">
  Server stdio
</h3>

Processi locali che comunicano tramite stdin/stdout. Utilizzate questo per i server MCP che eseguite sulla stessa macchina. Per il modulo `.mcp.json`, utilizzate gli stessi campi mostrati in [Da un file di configurazione](#from-a-config-file). Nel codice, passate il comando e i suoi argomenti. Sostituite `/Users/me/projects` con una directory sulla vostra macchina:

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
  Server HTTP/SSE
</h3>

Utilizzate HTTP o SSE per i server MCP ospitati nel cloud e le API remote. Per il modulo `.mcp.json`, utilizzate gli stessi campi dell'esempio in [Intestazioni HTTP per server remoti](#http-headers-for-remote-servers), con `"type": "sse"` per un server SSE. Nel codice, passate l'URL del server:

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

Per il trasporto HTTP trasmissibile, utilizzate `"type": "http"` invece. Nei file di configurazione `.mcp.json` e altri file JSON, `"streamable-http"` è accettato come alias per `"http"`. Il tipo `McpHttpServerConfig` degli SDK dichiara solo `"http"`, quindi utilizzate `"http"` per i server che passate nel codice.

<h3 id="sdk-mcp-servers">
  Server MCP SDK
</h3>

Definite strumenti personalizzati direttamente nel codice della vostra applicazione invece di eseguire un processo server separato. Consultate la [guida agli strumenti personalizzati](/docs/it/agent-sdk/custom-tools) per i dettagli di implementazione.

Un server MCP SDK registrato da una [richiesta di controllo `initialize`](/docs/it/agent-sdk/typescript#sdkcontrolinitializeresponse) inizia a connettersi non appena Claude Code elabora la richiesta.

<h2 id="mcp-tool-search">
  Ricerca MCP tool
</h2>

Quando hai molti MCP tool configurati, le definizioni dei tool possono consumare una parte significativa della tua finestra di contesto. La ricerca dei tool risolve questo problema trattenendo le definizioni dei tool dal contesto e caricando solo quelli di cui Claude ha bisogno per ogni turno.

La ricerca dei tool è abilitata per impostazione predefinita. Vedi [Tool search](/docs/it/agent-sdk/tool-search) per le opzioni di configurazione, le best practice e l'utilizzo della ricerca dei tool con i tool SDK personalizzati.

<h2 id="authentication">
  Autenticazione
</h2>

La maggior parte dei server MCP richiede l'autenticazione per accedere ai servizi esterni. Passare le credenziali tramite variabili di ambiente nella configurazione del server.

<h3 id="pass-credentials-via-environment-variables">
  Passare le credenziali tramite variabili di ambiente
</h3>

Utilizzare il campo `env` per passare chiavi API, token e altre credenziali al server MCP:

<Tabs>
  <Tab title="Nel codice">
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

    La sintassi `${API_KEY}` espande le variabili di ambiente in fase di esecuzione.
  </Tab>
</Tabs>

<h3 id="http-headers-for-remote-servers">
  Intestazioni HTTP per server remoti
</h3>

Per i server HTTP e SSE, passare le intestazioni di autenticazione direttamente nella configurazione del server:

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

Per un esempio completo e funzionante di un server remoto autenticato con intestazioni, vedere [Elencare i problemi da un repository](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  Autenticazione OAuth2
</h3>

La [specifica MCP supporta OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) per l'autorizzazione. L'SDK non apre un browser né esegue un flusso OAuth interattivo. Quando un server configurato restituisce una sfida di autorizzazione e nessun token memorizzato è disponibile, l'esecuzione dell'agente continua senza gli strumenti di quel server, e il server segnala lo stato `needs-auth`. L'array `mcp_servers` del [messaggio di inizializzazione del sistema](/docs/it/agent-sdk/typescript#sdksystemmessage) potrebbe comunque mostrare `pending` per quel server quando viene emesso. Per confermare se un server necessita di credenziali, eseguire il polling di `mcpServerStatus()` nell'SDK TypeScript o [`get_mcp_status()`](/docs/it/agent-sdk/python#methods) in Python.

Per fornire le credenziali, completare il flusso OAuth nella propria applicazione e passare il token di accesso risultante nelle `headers` del server:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app.
  // Implement getAccessTokenFromOAuthFlow for your OAuth provider.
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
  # After completing OAuth flow in your app.
  # Implement get_access_token_from_oauth_flow for your OAuth provider.
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
  Elencare i problemi da un repository
</h3>

Questo esempio si connette al [server MCP GitHub](https://github.com/github/github-mcp-server) remoto per elencare i problemi recenti. L'esempio include la registrazione del debug per verificare la connessione MCP e le chiamate agli strumenti.

Prima di eseguire, crea un [token di accesso personale GitHub](https://github.com/settings/personal-access-tokens) con accesso in lettura ai repository che desideri interrogare e impostalo come variabile di ambiente:

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

Nella riga `MCP servers:`, uno `status` di `connected` per `github` conferma che il token funziona. Se Claude Code ha un [elenco di strumenti memorizzato nella cache](#connection-timing) per il server, lo stato può leggere `pending` invece e il server si connette alla sua prima chiamata di strumento. Se lo stato è `failed` o `needs-auth`, vedi [Gestione degli errori](#error-handling) prima di fidarti del risultato, poiché Claude può ricorrere agli strumenti integrati quando il server non è disponibile.

<h3 id="query-a-database">
  Interrogare un database
</h3>

Questo esempio utilizza [DBHub](https://github.com/bytebase/dbhub) per interrogare un database Postgres. L'agente scopre automaticamente lo schema del database, scrive la query SQL e restituisce i risultati.

Lo strumento `execute_sql` di DBHub esegue qualsiasi SQL che l'agente emette, incluse le scritture, a meno che non lo limiti. Impostando `readonly = true` nel [file di configurazione di DBHub](https://dbhub.ai/config/toml), DBHub rifiuta le istruzioni `INSERT`, `UPDATE`, `DELETE` e DDL, quindi l'esempio non può modificare i tuoi dati anche se l'agente emette una scrittura. DBHub risolve `${DATABASE_URL}` dall'ambiente del processo quando carica la configurazione, quindi la stringa di connessione rimane fuori dal file. Crea questo `dbhub.toml` accanto al tuo script:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

Lo script quindi punta DBHub al file di configurazione invece di passare una stringa di connessione direttamente. Prima di eseguire, imposta la variabile di ambiente `DATABASE_URL` sulla tua stringa di connessione. Sostituisci i valori segnaposto con i dettagli del tuo database:

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
  Gestione degli errori
</h2>

I server MCP possono non riuscire a connettersi per vari motivi: il processo del server potrebbe non essere installato, le credenziali potrebbero non essere valide, oppure un server remoto potrebbe essere irraggiungibile.

Claude Code emette un messaggio `system` con sottotipo `init` all'inizio di ogni query. Questo messaggio include lo stato della connessione per ogni server MCP. Il campo `status` può essere `"pending"`, `"connected"`, `"failed"`, `"needs-auth"` o `"disabled"`. Claude Code emette il messaggio init dopo l'[attesa di connessione al primo turno](#connection-timing) per i server passati in `options.mcpServers`, quindi un server di questo tipo che si è connesso entro l'attesa mostra `"connected"`.

Nel messaggio init, non trattare `"pending"` come un errore di per sé. Può significare uno di questi:

* Il server non si è ancora connesso. Vedere [quanto tempo Claude Code aspetta prima del primo turno](#connection-timing)
* L'elenco degli strumenti del server è stato [servito dalla cache](#connection-timing), con una connessione effettuata al primo utilizzo
* La scadenza della connessione è scaduta. Un server di questo tipo segnala `"pending"` o `"failed"` a seconda dei tempi

Verificare la presenza di `"failed"` o `"needs-auth"` per rilevare i server che non saranno utilizzabili:

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

Lo stato di un server remoto può anche cambiare dopo che segnala `"connected"`. Quando la connessione ad esso si interrompe durante la sessione, Claude Code sposta il server di nuovo a `"pending"` mentre [si riconnette](/docs/it/mcp#automatic-reconnection). Una successiva chiamata a `mcpServerStatus()` in TypeScript, o [`ClaudeSDKClient.get_mcp_status()`](/docs/it/agent-sdk/python#methods) in Python, può quindi segnalare `"pending"` per un server che hai visto connesso in precedenza, senza alcun cambiamento di configurazione da parte tua.

Dopo che cinque tentativi di riconnessione falliscono, il server segnala `"failed"`, o `"needs-auth"` quando ha bisogno di essere autorizzato di nuovo. Per riprovare manualmente, chiama [`reconnectMcpServer()`](/docs/it/agent-sdk/typescript#methods) in TypeScript o [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/it/agent-sdk/python#methods) in Python.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Server shows "failed" status
</h3>

Controllare il messaggio `init` per vedere quali server non hanno potuto connettersi:

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

Uno stato `"pending"` non significa che il server non abbia potuto connettersi. Vedere [Error handling](#error-handling) per i casi che copre all'init. Per ottenere stati aggiornati più avanti nella sessione, chiamare il metodo `mcpServerStatus()` della query in TypeScript SDK, oppure [`ClaudeSDKClient.get_mcp_status()`](/docs/it/agent-sdk/python#methods) in Python.

Cause comuni:

* **Missing environment variables**: Assicurarsi che i token e le credenziali richiesti siano impostati. Per i server stdio, verificare che il campo `env` corrisponda a quello che il server si aspetta.
* **Server not installed**: Per i comandi `npx`, verificare che il pacchetto esista e che Node.js sia nel vostro PATH.
* **Invalid connection string**: Per i server di database, verificare il formato della stringa di connessione e che il database sia accessibile.
* **Network issues**: Per i server HTTP/SSE remoti, controllare che l'URL sia raggiungibile e che eventuali firewall consentano la connessione.

<h3 id="tools-not-being-called">
  Tools not being called
</h3>

Se Claude vede gli strumenti ma non li utilizza, verificare di aver concesso il permesso con `allowedTools`:

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

Le connessioni del server MCP scadono dopo 30 secondi per impostazione predefinita. Per modificare il tempo massimo che una chiamata di strumento in esecuzione può richiedere, impostare [`MCP_TOOL_TIMEOUT`](/docs/it/env-vars). Se il vostro server impiega più tempo per avviarsi, la connessione non riesce. Aumentare il limite di connessione con la variabile di ambiente [`MCP_TIMEOUT`](/docs/it/env-vars), in millisecondi. Per i server che necessitano di più tempo di avvio, considerare anche:

* Utilizzare un server più leggero se disponibile
* Pre-riscaldare il server prima di avviare l'agente
* Controllare i log del server per le cause di inizializzazione lenta

In TypeScript, è possibile impostare il limite di chiamata dello strumento per un singolo [SDK MCP server](#sdk-mcp-servers) passando [`timeout` a `createSdkMcpServer()`](/docs/it/agent-sdk/typescript#createsdkmcpserver).

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Tool output exceeds maximum allowed tokens
</h3>

L'SDK applica lo stesso limite di output MCP di Claude Code. Quando il risultato di uno strumento senza contenuto di immagine è più grande di 25.000 token, Claude Code salva l'output in un file e sostituisce il risultato dello strumento con un messaggio di errore che nomina il percorso del file, in modo che l'agente possa leggere l'output in porzioni.

Aumentare il limite con la variabile di ambiente [`MAX_MCP_OUTPUT_TOKENS`](/docs/it/env-vars). Vedere [MCP output limits and warnings](/docs/it/mcp#mcp-output-limits-and-warnings) per il comportamento completo, incluso il modo in cui un server può dichiarare un limite per strumento più elevato con l'annotazione `anthropic/maxResultSizeChars`.

<h2 id="related-resources">
  Risorse correlate
</h2>

* **[Guida agli strumenti personalizzati](/docs/it/agent-sdk/custom-tools)**: Crea il tuo server MCP che viene eseguito in-process con la tua applicazione SDK
* **[Autorizzazioni](/docs/it/agent-sdk/permissions)**: Controlla quali strumenti MCP il tuo agente può utilizzare con `allowedTools` e `disallowedTools`
* **[Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript)**: Riferimento API completo incluse le opzioni di configurazione MCP
* **[Riferimento Python SDK](/docs/it/agent-sdk/python)**: Riferimento API completo incluse le opzioni di configurazione MCP
* **[Directory server MCP](https://github.com/modelcontextprotocol/servers)**: Sfoglia i server MCP disponibili per database, API e altro
