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

Transmettez les serveurs MCP directement dans l'option `mcpServers`. Cet exemple démarre un serveur MCP de système de fichiers local pour `/Users/me/projects`. Remplacez ce chemin par un répertoire sur votre machine :

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

Créez un fichier `.mcp.json` à la racine de votre projet. Le fichier est détecté lorsque la source de paramètre `project` est activée, ce qui est le cas pour les options `query()` par défaut. Si vous définissez `settingSources` explicitement, incluez `"project"` pour que ce fichier soit chargé. Remplacez `/Users/me/projects` par un répertoire sur votre machine :

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
  Délai de connexion
</h2>

Claude Code enregistre les serveurs que vous transmettez dans `options.mcpServers` au démarrage et émet le [message init](#error-handling) une fois que l'attente du premier tour, le cas échéant, est résolue. Que chaque serveur `options.mcpServers` retarde le premier tour, et quand il se connecte, dépend de son type :

| Type de serveur                                                                                                   | Retarde le premier tour ?                                      | Délai d'attente du premier tour                                                                                 |
| :---------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| Serveur stdio, ou serveur HTTP/SSE sans liste d'outils en cache                                                   | Oui, jusqu'à ce qu'il se connecte                              | [`MCP_TIMEOUT`](/docs/fr/env-vars), 30 secondes par défaut ; la connexion échoue à cette limite                      |
| Serveur distant avec une liste d'outils en cache, enregistrée par Claude Code à partir d'une connexion précédente | Non ; les outils en cache sont disponibles dès le premier tour | Aucun ; se connecte lors de son premier appel d'outil, et cette connexion différée a son propre délai d'attente |
| Serveur [SDK](#sdk-mcp-servers) en processus                                                                      | Oui, jusqu'à ce qu'il se connecte et énumère ses outils        | Aucun ; les demandes de connexion et d'énumération d'outils ont chacune leur propre délai d'attente             |

Les serveurs chargés à partir de [fichiers de configuration](#from-a-config-file) tels que `.mcp.json` ou à partir de plugins affichent généralement `pending` dans le message init. Quand `options.mcpServers` contient un serveur stdio, HTTP ou SSE, le premier tour attend également ces serveurs en attente, jusqu'à `MCP_TIMEOUT`. Quand `options.mcpServers` est vide ou contient uniquement des serveurs SDK, le premier tour attend jusqu'à 2 secondes à la place :

* **Avec [recherche d'outils](/docs/fr/agent-sdk/tool-search), la valeur par défaut** : l'attente couvre les serveurs toujours en attente configurés avec [`alwaysLoad: true`](/docs/fr/mcp#exempt-a-server-from-deferral) et non les autres. Les autres continuent de se connecter en arrière-plan. [Disponibilité des outils](/docs/fr/mcp#tool-availability) décrit comment Claude accède à leurs outils une fois qu'ils se connectent.
* **Sans recherche d'outils** : l'attente couvre tous les serveurs en attente. [Configurer la recherche d'outils](/docs/fr/agent-sdk/tool-search#configure-tool-search) couvre ce qui désactive la recherche d'outils. Si vous excluez l'outil `ToolSearch` de la session, par exemple via `disallowedTools`, la session s'exécute également sans recherche d'outils.

Si vous définissez `permissionPromptToolName`, le premier tour attend également le serveur de cet outil dans tous les cas, jusqu'à `MCP_TIMEOUT`.

Pour définir vous-même l'attente du premier tour, ajoutez `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` à l'[option `env`](/docs/fr/agent-sdk/configuration#set-environment-variables), par exemple `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. Le premier tour attend alors jusqu'à ce nombre de millisecondes pour tous les serveurs en attente, que la recherche d'outils soit disponible ou non. Cette limite remplace également l'attente du premier tour `MCP_TIMEOUT` pour les serveurs stdio, HTTP et SSE dans `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` nécessite Claude Code v2.1.274 ou ultérieur.

Les serveurs toujours en attente quand l'attente se termine continuent de se connecter en arrière-plan. Définissez la variable à `0` pour ignorer l'attente. Un serveur `permissionPromptToolName` conserve sa propre attente `MCP_TIMEOUT` indépendamment de la valeur.

Pour bloquer le démarrage lui-même à une phase distincte et antérieure à l'attente du premier tour, avant que le message init soit envoyé :

* Définissez [`MCP_CONNECTION_NONBLOCKING`](/docs/fr/env-vars) à `0` pour bloquer sur tout le lot de connexions. Claude Code limite cette attente à 5 secondes par défaut. Ajustez la limite avec la variable d'environnement [`MCP_CONNECT_TIMEOUT_MS`](/docs/fr/env-vars), en millisecondes. Les serveurs toujours en attente à cette limite continuent de se connecter en arrière-plan.
* Définissez `alwaysLoad: true` sur la configuration d'un serveur pour rendre ses outils disponibles à leurs schémas complets au premier tour, [exempts du report de recherche d'outils](/docs/fr/mcp#exempt-a-server-from-deferral). Claude Code attend au démarrage les outils de ce serveur, limités à la même limite, tandis que les autres serveurs continuent de se connecter en arrière-plan ; un serveur distant avec une liste d'outils en cache les fournit sans se connecter, selon le tableau ci-dessus.

Le message `system` avec le sous-type `init` rapporte l'état de chaque serveur au moment où il est émis ; voir [Gestion des erreurs](#error-handling) pour lire ces états.

<h2 id="allow-mcp-tools">
  Autoriser les outils MCP
</h2>

Les outils MCP nécessitent une permission explicite avant que Claude puisse les utiliser. Sans permission, Claude verra que les outils sont disponibles mais ne pourra pas les appeler.

<h3 id="tool-naming-convention">
  Convention de nommage des outils
</h3>

Les outils MCP suivent le modèle de nommage `mcp__<server-name>__<tool-name>`. Par exemple, un serveur GitHub nommé `"github"` avec un outil `list_issues` devient `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Auto-approbation avec allowedTools
</h3>

Utilisez `allowedTools` pour auto-approuver des outils MCP spécifiques afin que Claude puisse les utiliser sans invite de permission :

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

Les caractères génériques (`*`) vous permettent d'autoriser tous les outils d'un serveur sans lister chacun individuellement.

<Note>
  **Préférez `allowedTools` aux modes de permission pour l'accès MCP.** `permissionMode: "acceptEdits"` n'auto-approuve pas les outils MCP (uniquement les modifications de fichiers et les commandes Bash du système de fichiers). `permissionMode: "bypassPermissions"` auto-approuve les outils MCP mais désactive également la plupart des autres invites de sécurité, ce qui est plus large que nécessaire ; consultez [Comment les permissions sont évaluées](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) pour les invites qui restent. Un caractère générique dans `allowedTools` accorde exactement le serveur MCP que vous souhaitez et rien de plus. Consultez [Modes de permission](/docs/fr/agent-sdk/permissions#permission-modes) pour une comparaison complète.
</Note>

<h3 id="discover-available-tools">
  Découvrir les outils disponibles
</h3>

Pour voir quels outils un serveur MCP fournit, consultez la documentation du serveur ou inspectez le tableau `tools` dans le message init `system`. Les noms des outils MCP commencent par `mcp__`.

Claude Code émet le message init après l'[attente de connexion au premier tour](#connection-timing) pour les serveurs passés dans `options.mcpServers`, donc le tableau `tools` liste les outils `mcp__` de chaque serveur qui s'est connecté à ce moment-là, plus ceux des serveurs avec une [liste d'outils en cache](#connection-timing), qui se connectent à la première utilisation. Les outils de tout autre serveur qui ne s'est pas connecté sont absents ; consultez [Gestion des erreurs](#error-handling) pour lire l'état de chaque serveur.

Ce filtre imprime les noms des outils MCP :

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

Vous pouvez également demander à Claude de lister les outils disponibles à partir d'un serveur.

<h2 id="transport-types">
  Types de transport
</h2>

Les serveurs MCP communiquent avec votre agent en utilisant différents protocoles de transport. Consultez la documentation du serveur pour voir quel transport il prend en charge :

* Si la documentation vous donne une **commande à exécuter** (comme `npx @modelcontextprotocol/server-filesystem`), utilisez stdio
* Si la documentation vous donne une **URL**, utilisez HTTP ou SSE
* Si vous créez vos propres outils dans le code, utilisez un serveur MCP SDK

<h3 id="stdio-servers">
  Serveurs stdio
</h3>

Des processus locaux qui communiquent via stdin/stdout. Utilisez ceci pour les serveurs MCP que vous exécutez sur la même machine. Pour le formulaire `.mcp.json`, utilisez les mêmes champs affichés à [À partir d'un fichier de configuration](#from-a-config-file). Dans le code, transmettez la commande et ses arguments. Remplacez `/Users/me/projects` par un répertoire sur votre machine :

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
  Serveurs HTTP/SSE
</h3>

Utilisez HTTP ou SSE pour les serveurs MCP hébergés dans le cloud et les API distantes. Pour le formulaire `.mcp.json`, utilisez les mêmes champs que l'exemple à [En-têtes HTTP pour les serveurs distants](#http-headers-for-remote-servers), avec `"type": "sse"` pour un serveur SSE. Dans le code, transmettez l'URL du serveur :

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

Pour le transport HTTP en continu, utilisez `"type": "http"` à la place. Dans `.mcp.json` et autres fichiers de configuration JSON, `"streamable-http"` est accepté comme alias pour `"http"`. Le type `McpHttpServerConfig` des SDK déclare uniquement `"http"`, donc utilisez `"http"` pour les serveurs que vous transmettez dans le code.

<h3 id="sdk-mcp-servers">
  Serveurs MCP SDK
</h3>

Définissez des outils personnalisés directement dans le code de votre application au lieu d'exécuter un processus serveur séparé. Consultez le [guide des outils personnalisés](/docs/fr/agent-sdk/custom-tools) pour les détails de mise en œuvre.

Un serveur MCP SDK enregistré par une [demande de contrôle `initialize`](/docs/fr/agent-sdk/typescript#sdkcontrolinitializeresponse) commence à se connecter dès que Claude Code traite la demande.

<h2 id="mcp-tool-search">
  Recherche d'outils MCP
</h2>

Lorsque vous avez de nombreux outils MCP configurés, les définitions d'outils peuvent consommer une part importante de votre fenêtre de contexte. La recherche d'outils résout ce problème en retenant les définitions d'outils du contexte et en chargeant uniquement ceux dont Claude a besoin à chaque tour.

La recherche d'outils est activée par défaut. Consultez [Recherche d'outils](/docs/fr/agent-sdk/tool-search) pour les options de configuration, les meilleures pratiques et l'utilisation de la recherche d'outils avec les outils SDK personnalisés.

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

    La syntaxe `${API_KEY}` développe les variables d'environnement au moment de l'exécution.
  </Tab>
</Tabs>

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

Pour un exemple complet et fonctionnel d'un serveur distant authentifié avec des en-têtes, consultez [Lister les problèmes d'un référentiel](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  Authentification OAuth2
</h3>

La [spécification MCP prend en charge OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) pour l'autorisation. Le SDK n'ouvre pas de navigateur ni n'exécute de flux OAuth interactif. Lorsqu'un serveur configuré retourne un défi d'autorisation et qu'aucun jeton stocké n'est disponible, l'exécution de l'agent continue sans les outils de ce serveur, et le serveur signale le statut `needs-auth`. Le tableau `mcp_servers` du [message d'initialisation du système](/docs/fr/agent-sdk/typescript#sdksystemmessage) peut toujours afficher `pending` pour ce serveur lors de son émission. Pour confirmer si un serveur a besoin d'identifiants, interrogez `mcpServerStatus()` dans le SDK TypeScript ou [`get_mcp_status()`](/docs/fr/agent-sdk/python#methods) en Python.

Pour fournir les identifiants, complétez le flux OAuth dans votre propre application et transmettez le jeton d'accès résultant dans les `headers` du serveur :

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
  Exemples
</h2>

<h3 id="list-issues-from-a-repository">
  Lister les problèmes d'un référentiel
</h3>

Cet exemple se connecte au [serveur MCP GitHub](https://github.com/github/github-mcp-server) distant pour lister les problèmes récents. L'exemple inclut la journalisation de débogage pour vérifier la connexion MCP et les appels d'outils.

Avant d'exécuter, créez un [jeton d'accès personnel GitHub](https://github.com/settings/personal-access-tokens) avec accès en lecture aux référentiels que vous souhaitez interroger et définissez-le comme variable d'environnement :

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

Dans la ligne `MCP servers :`, un `status` de `connected` pour `github` confirme que le jeton fonctionne. Si Claude Code dispose d'une [liste d'outils mise en cache](#connection-timing) pour le serveur, le statut peut afficher `pending` à la place et le serveur se connecte lors de son premier appel d'outil. Si le statut est `failed` ou `needs-auth`, consultez [Gestion des erreurs](#error-handling) avant de faire confiance au résultat, car Claude peut revenir aux outils intégrés lorsque le serveur n'est pas disponible.

<h3 id="query-a-database">
  Interroger une base de données
</h3>

Cet exemple utilise [DBHub](https://github.com/bytebase/dbhub) pour interroger une base de données Postgres. L'agent découvre automatiquement le schéma de la base de données, écrit la requête SQL et retourne les résultats.

L'outil `execute_sql` de DBHub exécute toute requête SQL que l'agent émet, y compris les écritures, sauf si vous la limitez. Définir `readonly = true` dans le [fichier de configuration DBHub](https://dbhub.ai/config/toml) fait que DBHub rejette les instructions `INSERT`, `UPDATE`, `DELETE` et DDL, de sorte que l'exemple ne peut pas modifier vos données même si l'agent émet une écriture. DBHub résout `${DATABASE_URL}` à partir de l'environnement du processus lorsqu'il charge la configuration, de sorte que la chaîne de connexion reste en dehors du fichier. Créez ce `dbhub.toml` à côté de votre script :

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

Le script pointe ensuite DBHub vers le fichier de configuration au lieu de passer une chaîne de connexion directement. Avant d'exécuter, définissez la variable d'environnement `DATABASE_URL` sur votre chaîne de connexion. Remplacez les valeurs d'espace réservé par les détails de votre propre base de données :

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
  Gestion des erreurs
</h2>

Les serveurs MCP peuvent échouer à se connecter pour diverses raisons : le processus du serveur pourrait ne pas être installé, les identifiants pourraient être invalides, ou un serveur distant pourrait être inaccessible.

Claude Code émet un message `system` avec le sous-type `init` au début de chaque requête. Ce message inclut l'état de la connexion pour chaque serveur MCP. Le champ `status` peut être `"pending"`, `"connected"`, `"failed"`, `"needs-auth"`, ou `"disabled"`. Claude Code émet le message init après le [délai d'attente de connexion à la première requête](#connection-timing) pour les serveurs passés dans `options.mcpServers`, donc un tel serveur qui s'est connecté dans le délai d'attente affiche `"connected"`.

Dans le message init, ne traitez pas `"pending"` comme un échec en soi. Cela peut signifier l'une de ces situations :

* Le serveur ne s'est pas encore connecté. Voir [combien de temps Claude Code attend avant la première requête](#connection-timing)
* La liste des outils du serveur a été [servie à partir du cache](#connection-timing), avec une connexion établie à la première utilisation
* Le délai de connexion a expiré. Un tel serveur rapporte `"pending"` ou `"failed"` selon le timing

Vérifiez `"failed"` ou `"needs-auth"` pour détecter les serveurs qui ne seront pas utilisables :

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

L'état d'un serveur distant peut également changer après qu'il rapporte `"connected"`. Lorsque la connexion à celui-ci s'interrompt en cours de session, Claude Code ramène le serveur à `"pending"` pendant la [reconnexion](/docs/fr/mcp#automatic-reconnection). Un appel ultérieur à `mcpServerStatus()` en TypeScript, ou [`ClaudeSDKClient.get_mcp_status()`](/docs/fr/agent-sdk/python#methods) en Python, peut alors rapporter `"pending"` pour un serveur que vous aviez vu connecté plus tôt, sans aucun changement de configuration de votre côté.

Après cinq tentatives de reconnexion échouées, le serveur rapporte `"failed"`, ou `"needs-auth"` lorsqu'il a besoin d'être autorisé à nouveau. Pour réessayer manuellement, appelez [`reconnectMcpServer()`](/docs/fr/agent-sdk/typescript#methods) en TypeScript ou [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/fr/agent-sdk/python#methods) en Python.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="server-shows-failed-status">
  Le serveur affiche un statut « failed »
</h3>

Vérifiez le message `init` pour voir quels serveurs n'ont pas pu se connecter :

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

Un statut `"pending"` ne signifie pas que le serveur a échoué. Consultez [Gestion des erreurs](#error-handling) pour connaître les cas qu'il couvre à l'initialisation. Pour obtenir les statuts mis à jour plus tard dans la session, appelez la méthode `mcpServerStatus()` de la requête dans le SDK TypeScript, ou [`ClaudeSDKClient.get_mcp_status()`](/docs/fr/agent-sdk/python#methods) en Python.

Causes courantes :

* **Variables d'environnement manquantes** : Assurez-vous que les jetons et identifiants requis sont définis. Pour les serveurs stdio, vérifiez que le champ `env` correspond à ce que le serveur attend.
* **Serveur non installé** : Pour les commandes `npx`, vérifiez que le package existe et que Node.js se trouve dans votre PATH.
* **Chaîne de connexion invalide** : Pour les serveurs de base de données, vérifiez le format de la chaîne de connexion et que la base de données est accessible.
* **Problèmes réseau** : Pour les serveurs HTTP/SSE distants, vérifiez que l'URL est accessible et que les pare-feu autorisent la connexion.

<h3 id="tools-not-being-called">
  Les outils ne sont pas appelés
</h3>

Si Claude voit les outils mais ne les utilise pas, vérifiez que vous avez accordé la permission avec `allowedTools` :

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
  Délais d'expiration de la connexion
</h3>

Les connexions au serveur MCP expirent après 30 secondes par défaut. Pour modifier la durée maximale d'un appel d'outil en cours, définissez [`MCP_TOOL_TIMEOUT`](/docs/fr/env-vars). Si votre serveur met plus de temps à démarrer, la connexion échoue. Augmentez la limite de connexion avec la variable d'environnement [`MCP_TIMEOUT`](/docs/fr/env-vars), en millisecondes. Pour les serveurs qui ont besoin de plus de temps de démarrage, envisagez également :

* Utiliser un serveur plus léger si disponible
* Préchauffer le serveur avant de démarrer votre agent
* Vérifier les journaux du serveur pour les causes d'initialisation lente

En TypeScript, vous pouvez définir la limite d'appel d'outil pour un seul [serveur MCP du SDK](#sdk-mcp-servers) en passant [`timeout` à `createSdkMcpServer()`](/docs/fr/agent-sdk/typescript#createsdkmcpserver).

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  La sortie de l'outil dépasse le nombre maximum de jetons autorisés
</h3>

Le SDK applique la même limite de sortie MCP que Claude Code. Lorsqu'un résultat d'outil sans contenu image est supérieur à 25 000 jetons, Claude Code enregistre la sortie dans un fichier et remplace le résultat de l'outil par un message d'erreur qui nomme le chemin du fichier, afin que l'agent puisse relire la sortie par portions.

Augmentez la limite avec la variable d'environnement [`MAX_MCP_OUTPUT_TOKENS`](/docs/fr/env-vars). Consultez [Limites et avertissements de sortie MCP](/docs/fr/mcp#mcp-output-limits-and-warnings) pour le comportement complet, y compris la façon dont un serveur peut déclarer une limite supérieure par outil avec l'annotation `anthropic/maxResultSizeChars`.

<h2 id="related-resources">
  Ressources connexes
</h2>

* **[Guide des outils personnalisés](/docs/fr/agent-sdk/custom-tools)** : Créez votre propre serveur MCP qui s'exécute en processus avec votre application SDK
* **[Permissions](/docs/fr/agent-sdk/permissions)** : Contrôlez les outils MCP que votre agent peut utiliser avec `allowedTools` et `disallowedTools`
* **[Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript)** : Référence API complète incluant les options de configuration MCP
* **[Référence du SDK Python](/docs/fr/agent-sdk/python)** : Référence API complète incluant les options de configuration MCP
* **[Répertoire des serveurs MCP](https://github.com/modelcontextprotocol/servers)** : Parcourez les serveurs MCP disponibles pour les bases de données, les API, et bien d'autres
