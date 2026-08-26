> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mit MCP zu externen Tools verbinden

> Konfigurieren Sie MCP-Server, um Ihren Agenten mit externen Tools zu erweitern. Behandelt Transporttypen, Tool-Suche für große Tool-Sets, Authentifizierung und Fehlerbehandlung.

Das [Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) ist ein offener Standard für die Verbindung von KI-Agenten mit externen Tools und Datenquellen. Mit MCP kann Ihr Agent Datenbanken abfragen, sich mit APIs wie Slack und GitHub integrieren und sich mit anderen Diensten verbinden, ohne benutzerdefinierte Tool-Implementierungen zu schreiben.

MCP-Server können als lokale Prozesse ausgeführt werden, sich über HTTP verbinden oder direkt in Ihrer SDK-Anwendung ausgeführt werden.

<Note>
  Diese Seite behandelt die MCP-Konfiguration für das Agent SDK. Um MCP-Server zur Claude Code CLI hinzuzufügen, damit sie in jedem Projekt geladen werden, siehe [MCP-Installationsbereiche](/docs/de/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Schnellstart
</h2>

Dieses Beispiel verbindet sich mit dem [Claude Code-Dokumentations](https://code.claude.com/docs)-MCP-Server unter Verwendung von [HTTP-Transport](#http%2Fsse-servers) und verwendet [`allowedTools`](#allow-mcp-tools) mit einem Platzhalter, um alle Tools vom Server zuzulassen.

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

Der Agent verbindet sich mit dem Dokumentationsserver, sucht nach Informationen über hooks und gibt die Ergebnisse zurück.

<h2 id="add-an-mcp-server">
  Einen MCP-Server hinzufügen
</h2>

Sie können MCP-Server im Code beim Aufrufen von `query()` konfigurieren oder in einer `.mcp.json`-Datei, die über [`settingSources`](#from-a-config-file) geladen wird.

<h3 id="in-code">
  Im Code
</h3>

Übergeben Sie MCP-Server direkt in der `mcpServers`-Option:

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
  Aus einer Konfigurationsdatei
</h3>

Erstellen Sie eine `.mcp.json`-Datei im Stammverzeichnis Ihres Projekts. Die Datei wird aufgegriffen, wenn die `project`-Einstellungsquelle aktiviert ist, was sie für Standard-`query()`-Optionen ist. Wenn Sie `settingSources` explizit festlegen, fügen Sie `"project"` ein, damit diese Datei geladen wird:

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
  MCP-Tools zulassen
</h2>

MCP-Tools erfordern explizite Genehmigung, bevor Claude sie verwenden kann. Ohne Genehmigung sieht Claude, dass Tools verfügbar sind, kann sie aber nicht aufrufen.

<h3 id="tool-naming-convention">
  Tool-Benennungskonvention
</h3>

MCP-Tools folgen dem Benennungsmuster `mcp__<server-name>__<tool-name>`. Beispielsweise wird ein GitHub-Server mit dem Namen `"github"` mit einem `list_issues`-Tool zu `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Automatische Genehmigung mit allowedTools
</h3>

Verwenden Sie `allowedTools`, um bestimmte MCP-Tools automatisch zu genehmigen, damit Claude sie ohne Genehmigungsaufforderung verwenden kann:

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

Platzhalter (`*`) ermöglichen es Ihnen, alle Tools von einem Server zuzulassen, ohne jedes einzeln aufzulisten.

<Note>
  **Bevorzugen Sie `allowedTools` gegenüber Berechtigungsmodi für MCP-Zugriff.** `permissionMode: "acceptEdits"` genehmigt MCP-Tools nicht automatisch (nur Dateibearbeitungen und Filesystem-Bash-Befehle). `permissionMode: "bypassPermissions"` genehmigt MCP-Tools automatisch, deaktiviert aber auch die meisten anderen Sicherheitsaufforderungen, was breiter ist als nötig; siehe [Wie Berechtigungen ausgewertet werden](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) für die Aufforderungen, die bleiben. Ein Platzhalter in `allowedTools` gewährt genau den MCP-Server, den Sie möchten, und nichts mehr. Siehe [Berechtigungsmodi](/docs/de/agent-sdk/permissions#permission-modes) für einen vollständigen Vergleich.
</Note>

<h3 id="discover-available-tools">
  Verfügbare Tools entdecken
</h3>

Um zu sehen, welche Tools ein MCP-Server bereitstellt, überprüfen Sie die Dokumentation des Servers oder verbinden Sie sich mit dem Server und inspizieren Sie die `system`-Init-Nachricht:

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
  Transporttypen
</h2>

MCP-Server kommunizieren mit Ihrem Agenten über verschiedene Transportprotokolle. Überprüfen Sie die Dokumentation des Servers, um zu sehen, welchen Transport er unterstützt:

* Wenn die Dokumentation Ihnen einen **Befehl zum Ausführen** gibt (wie `npx @modelcontextprotocol/server-github`), verwenden Sie stdio
* Wenn die Dokumentation Ihnen eine **URL** gibt, verwenden Sie HTTP oder SSE
* Wenn Sie Ihre eigenen Tools im Code erstellen, verwenden Sie einen SDK MCP-Server

<h3 id="stdio-servers">
  stdio-Server
</h3>

Lokale Prozesse, die über stdin/stdout kommunizieren. Verwenden Sie dies für MCP-Server, die Sie auf demselben Computer ausführen:

<Tabs>
  <Tab title="Im Code">
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
  HTTP/SSE-Server
</h3>

Verwenden Sie HTTP oder SSE für Cloud-gehostete MCP-Server und Remote-APIs:

<Tabs>
  <Tab title="Im Code">
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

Verwenden Sie für den streamfähigen HTTP-Transport stattdessen `"type": "http"`. In `.mcp.json` und anderen JSON-Konfigurationsdateien wird `"streamable-http"` als Alias für `"http"` akzeptiert. Die programmgesteuerte `mcpServers`-Option akzeptiert nur `"http"`.

<h3 id="sdk-mcp-servers">
  SDK MCP-Server
</h3>

Definieren Sie benutzerdefinierte Tools direkt in Ihrem Anwendungscode, anstatt einen separaten Serverprozess auszuführen. Siehe das [Leitfaden für benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools) für Implementierungsdetails.

<h2 id="mcp-tool-search">
  MCP-Tool-Suche
</h2>

Wenn Sie viele MCP-Tools konfiguriert haben, können Tool-Definitionen einen erheblichen Teil Ihres Kontextfensters verbrauchen. Die Tool-Suche löst dies, indem Tool-Definitionen aus dem Kontext zurückgehalten und nur die Tools geladen werden, die Claude für jeden Durchgang benötigt.

Die Tool-Suche ist standardmäßig aktiviert. Siehe [Tool-Suche](/docs/de/agent-sdk/tool-search) für Konfigurationsoptionen und Details.

Für weitere Details, einschließlich Best Practices und Verwendung der Tool-Suche mit benutzerdefinierten SDK-Tools, siehe das [Tool-Suche-Leitfaden](/docs/de/agent-sdk/tool-search).

<h2 id="authentication">
  Authentifizierung
</h2>

Die meisten MCP-Server erfordern Authentifizierung, um auf externe Dienste zuzugreifen. Übergeben Sie Anmeldedaten über Umgebungsvariablen in der Serverkonfiguration.

<h3 id="pass-credentials-via-environment-variables">
  Anmeldedaten über Umgebungsvariablen übergeben
</h3>

Verwenden Sie das `env`-Feld, um API-Schlüssel, Token und andere Anmeldedaten an den MCP-Server zu übergeben:

<Tabs>
  <Tab title="Im Code">
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

    Die `${GITHUB_TOKEN}`-Syntax erweitert Umgebungsvariablen zur Laufzeit.
  </Tab>
</Tabs>

Siehe [Probleme aus einem Repository auflisten](#list-issues-from-a-repository) für ein vollständiges funktionierendes Beispiel mit Debug-Protokollierung.

<h3 id="http-headers-for-remote-servers">
  HTTP-Header für Remote-Server
</h3>

Für HTTP- und SSE-Server übergeben Sie Authentifizierungs-Header direkt in der Serverkonfiguration:

<Tabs>
  <Tab title="Im Code">
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

    Die `${API_TOKEN}`-Syntax erweitert Umgebungsvariablen zur Laufzeit.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  OAuth2-Authentifizierung
</h3>

Die [MCP-Spezifikation unterstützt OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) für Autorisierung. Das SDK öffnet keinen Browser und führt keinen interaktiven OAuth-Flow aus. Wenn ein konfigurierter Server eine Autorisierungsanforderung zurückgibt und kein gespeichertes Token verfügbar ist, wird die Agent-Ausführung ohne die Tools dieses Servers fortgesetzt, und der Server wird mit dem Status `needs-auth` im `mcp_servers`-Array der [System-Init-Nachricht](/docs/de/agent-sdk/typescript#sdksystemmessage) gemeldet. Überprüfen Sie dieses Array beim Start, wenn Ihr Agent von einem bestimmten Server abhängig ist.

Um Anmeldedaten bereitzustellen, führen Sie den OAuth-Flow in Ihrer eigenen Anwendung durch und übergeben Sie das resultierende Zugriffs-Token in den `headers` des Servers:

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
  Beispiele
</h2>

<h3 id="list-issues-from-a-repository">
  Probleme aus einem Repository auflisten
</h3>

Dieses Beispiel verbindet sich mit dem [GitHub MCP-Server](https://github.com/modelcontextprotocol/servers/tree/main/src/github), um aktuelle Probleme aufzulisten. Das Beispiel enthält Debug-Protokollierung, um die MCP-Verbindung und Tool-Aufrufe zu überprüfen.

Erstellen Sie vor dem Ausführen ein [GitHub-Persönliches Zugriffs-Token](https://github.com/settings/tokens) mit `repo`-Bereich und legen Sie es als Umgebungsvariable fest:

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
  Eine Datenbank abfragen
</h3>

Dieses Beispiel verwendet den [Postgres MCP-Server](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres), um eine Datenbank abzufragen. Die Verbindungszeichenfolge wird als Argument an den Server übergeben. Der Agent entdeckt automatisch das Datenbankschema, schreibt die SQL-Abfrage und gibt die Ergebnisse zurück:

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
  Fehlerbehandlung
</h2>

MCP-Server können aus verschiedenen Gründen keine Verbindung herstellen: Der Serverprozess ist möglicherweise nicht installiert, Anmeldedaten könnten ungültig sein, oder ein Remote-Server könnte unerreichbar sein.

Das SDK sendet eine `system`-Nachricht mit dem Subtyp `init` am Anfang jeder Abfrage. Diese Nachricht enthält den Verbindungsstatus für jeden MCP-Server. Überprüfen Sie das `status`-Feld, um Verbindungsfehler zu erkennen, bevor der Agent mit der Arbeit beginnt:

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
  Fehlerbehebung
</h2>

<h3 id="server-shows-failed-status">
  Server zeigt Status „fehlgeschlagen"
</h3>

Überprüfen Sie die `init`-Nachricht, um zu sehen, welche Server keine Verbindung herstellen konnten:

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Häufige Ursachen:

* **Fehlende Umgebungsvariablen**: Stellen Sie sicher, dass erforderliche Token und Anmeldedaten festgelegt sind. Überprüfen Sie für stdio-Server, dass das `env`-Feld dem entspricht, was der Server erwartet.
* **Server nicht installiert**: Überprüfen Sie für `npx`-Befehle, dass das Paket vorhanden ist und Node.js in Ihrem PATH ist.
* **Ungültige Verbindungszeichenfolge**: Überprüfen Sie für Datenbankserver das Format der Verbindungszeichenfolge und dass die Datenbank zugänglich ist.
* **Netzwerkprobleme**: Überprüfen Sie für Remote-HTTP/SSE-Server, dass die URL erreichbar ist und Firewalls die Verbindung zulassen.

<h3 id="tools-not-being-called">
  Tools werden nicht aufgerufen
</h3>

Wenn Claude Tools sieht, sie aber nicht verwendet, überprüfen Sie, dass Sie die Berechtigung mit `allowedTools` gewährt haben:

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
  Verbindungs-Timeouts
</h3>

MCP-Serververbindungen haben standardmäßig ein Timeout von 30 Sekunden. Wenn Ihr Server länger zum Starten benötigt, schlägt die Verbindung fehl. Erhöhen Sie das Limit mit der Umgebungsvariablen [`MCP_TIMEOUT`](/docs/de/env-vars) in Millisekunden. Für Server, die mehr Startzeit benötigen, erwägen Sie auch:

* Verwendung eines leichteren Servers, falls verfügbar
* Vorwärmung des Servers vor dem Starten Ihres Agenten
* Überprüfung von Serverprotokollen auf langsame Initialisierungsursachen

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Werkzeugausgabe überschreitet maximal zulässige Token
</h3>

Das SDK wendet das gleiche MCP-Ausgabelimit wie Claude Code an. Wenn ein Werkzeugergebnis größer als 25.000 Token ist, wird die vollständige Ausgabe in einer Datei gespeichert und das Werkzeugergebnis wird durch eine Fehlermeldung ersetzt, die den Dateipfad benennt, damit der Agent die Ausgabe in Teilen zurücklesen kann. Erhöhen Sie das Limit mit der Umgebungsvariablen [`MAX_MCP_OUTPUT_TOKENS`](/docs/de/env-vars). Siehe [MCP-Ausgabelimits und Warnungen](/docs/de/mcp#mcp-output-limits-and-warnings) für das vollständige Verhalten, einschließlich wie ein Server ein höheres Pro-Werkzeug-Limit deklarieren kann.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* **[Leitfaden für benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools)**: Erstellen Sie Ihren eigenen MCP-Server, der in-process mit Ihrer SDK-Anwendung ausgeführt wird
* **[Berechtigungen](/docs/de/agent-sdk/permissions)**: Kontrollieren Sie, welche MCP-Tools Ihr Agent mit `allowedTools` und `disallowedTools` verwenden kann
* **[MCP-Ausgabelimits und Warnungen](/docs/de/mcp#mcp-output-limits-and-warnings)**: Wie das SDK Tool-Ergebnisse handhabt, die `MAX_MCP_OUTPUT_TOKENS` überschreiten, einschließlich des Fallbacks zum Speichern auf der Festplatte und der `anthropic/maxResultSizeChars`-Annotation pro Tool
* **[TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript)**: Vollständige API-Referenz einschließlich MCP-Konfigurationsoptionen
* **[Python SDK-Referenz](/docs/de/agent-sdk/python)**: Vollständige API-Referenz einschließlich MCP-Konfigurationsoptionen
* **[MCP-Server-Verzeichnis](https://github.com/modelcontextprotocol/servers)**: Durchsuchen Sie verfügbare MCP-Server für Datenbanken, APIs und mehr
