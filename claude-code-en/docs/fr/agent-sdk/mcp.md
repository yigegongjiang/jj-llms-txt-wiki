> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connecter à des outils externes avec MCP

> Configurez les serveurs MCP pour étendre votre agent avec des outils externes. Couvre les types de transport, la recherche d'outils pour les grands ensembles d'outils, l'authentification et la gestion des erreurs.

Le [Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) est une norme ouverte pour connecter les agents IA aux outils externes et aux sources de données. Avec MCP, votre agent peut interroger des bases de données, s'intégrer à des API comme Slack et GitHub, et se connecter à d'autres services sans écrire d'implémentations d'outils personnalisés.

Les serveurs MCP peuvent s'exécuter en tant que processus locaux, se connecter via HTTP ou s'exécuter directement dans votre application SDK.

<Note>
  Cette page couvre la configuration de MCP pour l'Agent SDK. Pour ajouter des serveurs MCP à l'interface de ligne de commande Claude Code afin qu'ils se chargent dans chaque projet, consultez [Portées d'installation MCP](/docs/fr/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Démarrage rapide
</h2>

Cet exemple se connecte au serveur MCP de [documentation Claude Code](https://code.claude.com/docs) en utilisant le [transport HTTP](#http%2Fsse-servers) et utilise [`allowedTools`](#allow-mcp-tools) avec un caractère générique pour autoriser tous les outils du serveur.

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

L'agent se connecte au serveur de documentation, recherche des informations sur les hooks et retourne les résultats.

<h2 id="add-an-mcp-server">
  Ajouter un serveur MCP
</h2>

Vous pouvez configurer les serveurs MCP dans le code lors de l'appel de `query()`, ou dans un fichier `.mcp.json` chargé via [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  Dans le code
</h3>

Transmettez les serveurs MCP directement dans l'option `mcpServers` :

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
  À partir d'un fichier de configuration
</h3>

Créez un fichier `.mcp.json` à la racine de votre projet. Le fichier est récupéré lorsque la source de paramètre `project` est activée, ce qui est le cas pour les options `query()` par défaut. Si vous définissez `settingSources` explicitement, incluez `"project"` pour que ce fichier se charge :

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
  Autoriser les outils MCP
</h2>

Les outils MCP nécessitent une autorisation explicite avant que Claude puisse les utiliser. Sans autorisation, Claude verra que les outils sont disponibles mais ne pourra pas les appeler.

<h3 id="tool-naming-convention">
  Convention de nommage des outils
</h3>

Les outils MCP suivent le modèle de nommage `mcp__<server-name>__<tool-name>`. Par exemple, un serveur GitHub nommé `"github"` avec un outil `list_issues` devient `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Approbation automatique avec allowedTools
</h3>

Utilisez `allowedTools` pour approuver automatiquement des outils MCP spécifiques afin que Claude puisse les utiliser sans invite de permission :

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

Les caractères génériques (`*`) vous permettent d'autoriser tous les outils d'un serveur sans lister chacun individuellement.

<Note>
  **Préférez `allowedTools` aux modes de permission pour l'accès MCP.** `permissionMode: "acceptEdits"` n'approuve pas automatiquement les outils MCP (uniquement les modifications de fichiers et les commandes Bash du système de fichiers). `permissionMode: "bypassPermissions"` approuve automatiquement les outils MCP mais désactive également tous les autres messages de sécurité, ce qui est plus large que nécessaire ; consultez [Comment les permissions sont évaluées](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) pour les messages qui restent. Un caractère générique dans `allowedTools` accorde exactement le serveur MCP que vous souhaitez et rien de plus. Consultez [Modes de permission](/docs/fr/agent-sdk/permissions#permission-modes) pour une comparaison complète.
</Note>

<h3 id="discover-available-tools">
  Découvrir les outils disponibles
</h3>

Pour voir quels outils un serveur MCP fournit, consultez la documentation du serveur ou connectez-vous au serveur et inspectez le message d'initialisation `system` :

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
  Types de transport
</h2>

Les serveurs MCP communiquent avec votre agent en utilisant différents protocoles de transport. Consultez la documentation du serveur pour voir quel transport il supporte :

* Si la documentation vous donne une **commande à exécuter** (comme `npx @modelcontextprotocol/server-github`), utilisez stdio
* Si la documentation vous donne une **URL**, utilisez HTTP ou SSE
* Si vous construisez vos propres outils dans le code, utilisez un serveur MCP SDK

<h3 id="stdio-servers">
  Serveurs stdio
</h3>

Les processus locaux qui communiquent via stdin/stdout. Utilisez ceci pour les serveurs MCP que vous exécutez sur la même machine :

<Tabs>
  <Tab title="Dans le code">
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
  Serveurs HTTP/SSE
</h3>

Utilisez HTTP ou SSE pour les serveurs MCP hébergés dans le cloud et les API distantes :

<Tabs>
  <Tab title="Dans le code">
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

Pour le transport HTTP en continu, utilisez `"type": "http"` à la place. Dans `.mcp.json` et autres fichiers de configuration JSON, `"streamable-http"` est accepté comme alias pour `"http"`. L'option programmatique `mcpServers` accepte uniquement `"http"`.

<h3 id="sdk-mcp-servers">
  Serveurs MCP SDK
</h3>

Définissez des outils personnalisés directement dans le code de votre application au lieu d'exécuter un processus serveur séparé. Consultez le [guide des outils personnalisés](/docs/fr/agent-sdk/custom-tools) pour les détails d'implémentation.

<h2 id="mcp-tool-search">
  Recherche d'outils MCP
</h2>

Lorsque vous avez de nombreux outils MCP configurés, les définitions d'outils peuvent consommer une partie importante de votre fenêtre de contexte. La recherche d'outils résout ce problème en retenant les définitions d'outils du contexte et en chargeant uniquement ceux dont Claude a besoin pour chaque tour.

La recherche d'outils est activée par défaut. Consultez [Recherche d'outils](/docs/fr/agent-sdk/tool-search) pour les options de configuration et les détails.

Pour plus de détails, y compris les meilleures pratiques et l'utilisation de la recherche d'outils avec les outils SDK personnalisés, consultez le [guide de recherche d'outils](/docs/fr/agent-sdk/tool-search).

<h2 id="authentication">
  Authentification
</h2>

La plupart des serveurs MCP nécessitent une authentification pour accéder aux services externes. Transmettez les identifiants via des variables d'environnement dans la configuration du serveur.

<h3 id="pass-credentials-via-environment-variables">
  Transmettre les identifiants via des variables d'environnement
</h3>

Utilisez le champ `env` pour transmettre les clés API, les jetons et autres identifiants au serveur MCP :

<Tabs>
  <Tab title="Dans le code">
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

    La syntaxe `${GITHUB_TOKEN}` développe les variables d'environnement au moment de l'exécution.
  </Tab>
</Tabs>

Consultez [Lister les problèmes d'un référentiel](#list-issues-from-a-repository) pour un exemple complet fonctionnant avec la journalisation de débogage.

<h3 id="http-headers-for-remote-servers">
  En-têtes HTTP pour les serveurs distants
</h3>

Pour les serveurs HTTP et SSE, transmettez les en-têtes d'authentification directement dans la configuration du serveur :

<Tabs>
  <Tab title="Dans le code">
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

    La syntaxe `${API_TOKEN}` développe les variables d'environnement au moment de l'exécution.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  Authentification OAuth2
</h3>

La [spécification MCP supporte OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) pour l'autorisation. Le SDK n'ouvre pas de navigateur et n'exécute pas de flux OAuth interactif. Lorsqu'un serveur configuré retourne un défi d'autorisation et qu'aucun jeton stocké n'est disponible, l'exécution de l'agent continue sans les outils de ce serveur, et le serveur est signalé avec le statut `needs-auth` dans le tableau `mcp_servers` du [message d'initialisation système](/docs/fr/agent-sdk/typescript#sdksystemmessage). Vérifiez ce tableau au démarrage si votre agent dépend d'un serveur spécifique connecté.

Pour fournir les identifiants, complétez le flux OAuth dans votre propre application et transmettez le jeton d'accès résultant dans les `headers` du serveur :

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
  Exemples
</h2>

<h3 id="list-issues-from-a-repository">
  Lister les problèmes d'un référentiel
</h3>

Cet exemple se connecte au [serveur GitHub MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/github) pour lister les problèmes récents. L'exemple inclut la journalisation de débogage pour vérifier la connexion MCP et les appels d'outils.

Avant d'exécuter, créez un [jeton d'accès personnel GitHub](https://github.com/settings/tokens) avec la portée `repo` et définissez-le comme variable d'environnement :

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
  Interroger une base de données
</h3>

Cet exemple utilise le [serveur Postgres MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) pour interroger une base de données. La chaîne de connexion est transmise comme argument au serveur. L'agent découvre automatiquement le schéma de la base de données, écrit la requête SQL et retourne les résultats :

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
  Gestion des erreurs
</h2>

Les serveurs MCP peuvent échouer à se connecter pour diverses raisons : le processus serveur peut ne pas être installé, les identifiants peuvent être invalides, ou un serveur distant peut être inaccessible.

Le SDK émet un message `system` avec le sous-type `init` au début de chaque requête. Ce message inclut l'état de la connexion pour chaque serveur MCP. Vérifiez le champ `status` pour détecter les défaillances de connexion avant que l'agent ne commence à travailler :

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
  Dépannage
</h2>

<h3 id="server-shows-failed-status">
  Le serveur affiche le statut « failed »
</h3>

Vérifiez le message `init` pour voir quels serveurs n'ont pas pu se connecter :

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Causes courantes :

* **Variables d'environnement manquantes** : Assurez-vous que les jetons et identifiants requis sont définis. Pour les serveurs stdio, vérifiez que le champ `env` correspond à ce que le serveur attend.
* **Serveur non installé** : Pour les commandes `npx`, vérifiez que le package existe et que Node.js est dans votre PATH.
* **Chaîne de connexion invalide** : Pour les serveurs de base de données, vérifiez le format de la chaîne de connexion et que la base de données est accessible.
* **Problèmes réseau** : Pour les serveurs HTTP/SSE distants, vérifiez que l'URL est accessible et que les pare-feu autorisent la connexion.

<h3 id="tools-not-being-called">
  Les outils ne sont pas appelés
</h3>

Si Claude voit les outils mais ne les utilise pas, vérifiez que vous avez accordé la permission avec `allowedTools` :

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
  Délais d'expiration de la connexion
</h3>

Les connexions serveur MCP expirent après 30 secondes par défaut. Si votre serveur prend plus de temps pour démarrer, la connexion échoue. Augmentez la limite avec la variable d'environnement [`MCP_TIMEOUT`](/docs/fr/env-vars), en millisecondes. Pour les serveurs qui ont besoin de plus de temps de démarrage, envisagez également :

* Utiliser un serveur plus léger si disponible
* Préchauffer le serveur avant de démarrer votre agent
* Vérifier les journaux du serveur pour les causes de lenteur d'initialisation

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  La sortie de l'outil dépasse le nombre maximum de jetons autorisés
</h3>

Le SDK applique la même limite de sortie MCP que Claude Code. Lorsqu'un résultat d'outil est supérieur à 25 000 jetons, la sortie complète est enregistrée dans un fichier et le résultat de l'outil est remplacé par un message d'erreur qui indique le chemin du fichier, afin que l'agent puisse relire la sortie par portions. Augmentez la limite avec la variable d'environnement [`MAX_MCP_OUTPUT_TOKENS`](/docs/fr/env-vars). Consultez [Limites et avertissements de sortie MCP](/docs/fr/mcp#mcp-output-limits-and-warnings) pour le comportement complet, y compris la façon dont un serveur peut déclarer une limite supérieure par outil.

<h2 id="related-resources">
  Ressources connexes
</h2>

* **[Guide des outils personnalisés](/docs/fr/agent-sdk/custom-tools)** : Créez votre propre serveur MCP qui s'exécute en processus avec votre application SDK
* **[Permissions](/docs/fr/agent-sdk/permissions)** : Contrôlez quels outils MCP votre agent peut utiliser avec `allowedTools` et `disallowedTools`
* **[Limites et avertissements de sortie MCP](/docs/fr/mcp#mcp-output-limits-and-warnings)** : Comment le SDK gère les résultats d'outils qui dépassent `MAX_MCP_OUTPUT_TOKENS`, y compris le secours de persistance sur disque et l'annotation `anthropic/maxResultSizeChars` par outil
* **[Référence SDK TypeScript](/docs/fr/agent-sdk/typescript)** : Référence API complète incluant les options de configuration MCP
* **[Référence SDK Python](/docs/fr/agent-sdk/python)** : Référence API complète incluant les options de configuration MCP
* **[Répertoire des serveurs MCP](https://github.com/modelcontextprotocol/servers)** : Parcourez les serveurs MCP disponibles pour les bases de données, les API et bien d'autres
