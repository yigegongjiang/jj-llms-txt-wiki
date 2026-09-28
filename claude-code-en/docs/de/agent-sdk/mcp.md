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
  MCP-Server hinzufügen
</h2>

Sie können MCP-Server im Code beim Aufrufen von `query()` konfigurieren oder in einer `.mcp.json`-Datei, die über [`settingSources`](#from-a-config-file) geladen wird.

<h3 id="in-code">
  Im Code
</h3>

Übergeben Sie MCP-Server direkt in der Option `mcpServers`. Dieses Beispiel startet einen lokalen Dateisystem-MCP-Server für `/Users/me/projects`. Ersetzen Sie diesen Pfad durch ein Verzeichnis auf Ihrem Computer:

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

Erstellen Sie eine `.mcp.json`-Datei im Stammverzeichnis Ihres Projekts. Die Datei wird geladen, wenn die `project`-Einstellungsquelle aktiviert ist, was sie standardmäßig für `query()`-Optionen ist. Wenn Sie `settingSources` explizit festlegen, fügen Sie `"project"` ein, damit diese Datei geladen wird. Ersetzen Sie `/Users/me/projects` durch ein Verzeichnis auf Ihrem Computer:

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
  Verbindungszeitpunkt
</h2>

Claude Code registriert die Server, die Sie in `options.mcpServers` übergeben, beim Start und sendet die [Init-Nachricht](#error-handling), sobald die Wartezeit für die erste Runde, falls vorhanden, abgelaufen ist. Ob jeder `options.mcpServers`-Server die erste Runde verzögert und wann er sich verbindet, hängt von seinem Typ ab:

| Servertyp                                                                                                          | Verzögert die erste Runde?                                                   | Wartezeit-Timeout für die erste Runde                                                                        |
| :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| Stdio-Server oder HTTP/SSE-Server ohne zwischengespeicherte Werkzeugliste                                          | Ja, bis zur Verbindung                                                       | [`MCP_TIMEOUT`](/docs/de/env-vars), standardmäßig 30 Sekunden; die Verbindung schlägt bei dieser Frist fehl       |
| Remote-Server mit zwischengespeicherter Werkzeugliste, gespeichert von Claude Code aus einer vorherigen Verbindung | Nein; die zwischengespeicherten Werkzeuge sind ab der ersten Runde verfügbar | Keine; verbindet sich beim ersten Werkzeugaufruf, und diese aufgeschobene Verbindung hat ihr eigenes Timeout |
| In-Process-[SDK-Server](#sdk-mcp-servers)                                                                          | Ja, bis zur Verbindung und Auflistung der Werkzeuge                          | Keine; die Verbindungs- und Werkzeugauflistungsanfragen haben jeweils ihr eigenes Timeout                    |

Server, die aus [Konfigurationsdateien](#from-a-config-file) wie `.mcp.json` oder aus Plugins geladen werden, zeigen häufig `pending` in der Init-Nachricht an. Wenn `options.mcpServers` einen Stdio-, HTTP- oder SSE-Server enthält, wartet die erste Runde auch auf diese ausstehenden Server, bis zu `MCP_TIMEOUT`. Wenn `options.mcpServers` leer ist oder nur SDK-Server enthält, wartet die erste Runde stattdessen bis zu 2 Sekunden:

* **Mit [Werkzeugsuche](/docs/de/agent-sdk/tool-search), der Standardeinstellung**: Die Wartezeit umfasst noch ausstehende Server, die mit [`alwaysLoad: true`](/docs/de/mcp#exempt-a-server-from-deferral) konfiguriert sind, und nicht die übrigen. Die übrigen bleiben im Hintergrund verbunden. [Werkzeugverfügbarkeit](/docs/de/mcp#tool-availability) beschreibt, wie Claude auf ihre Werkzeuge zugreift, sobald sie verbunden sind.
* **Ohne Werkzeugsuche**: Die Wartezeit umfasst jeden ausstehenden Server. [Werkzeugsuche konfigurieren](/docs/de/agent-sdk/tool-search#configure-tool-search) behandelt, wie die Werkzeugsuche deaktiviert wird. Wenn Sie das `ToolSearch`-Werkzeug aus der Sitzung ausschließen, beispielsweise durch `disallowedTools`, wird die Sitzung auch ohne Werkzeugsuche ausgeführt.

Wenn Sie `permissionPromptToolName` setzen, wartet die erste Runde auch in jedem Fall auf den Server dieses Werkzeugs, bis zu `MCP_TIMEOUT`.

Um die Wartezeit für die erste Runde selbst zu setzen, fügen Sie `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` zur [`env`-Option](/docs/de/agent-sdk/configuration#set-environment-variables) hinzu, beispielsweise `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. Die erste Runde wartet dann bis zu dieser Anzahl von Millisekunden auf jeden ausstehenden Server, unabhängig davon, ob die Werkzeugsuche verfügbar ist. Diese Frist ersetzt auch die `MCP_TIMEOUT`-Wartezeit für die erste Runde für Stdio-, HTTP- und SSE-Server in `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` erfordert Claude Code v2.1.274 oder später.

Server, die noch ausstehend sind, wenn die Wartezeit endet, bleiben im Hintergrund verbunden. Setzen Sie die Variable auf `0`, um die Wartezeit zu überspringen. Ein `permissionPromptToolName`-Server behält seine eigene `MCP_TIMEOUT`-Wartezeit unabhängig vom Wert bei.

Um den Start selbst in einer separaten, früheren Phase als die Wartezeit für die erste Runde zu blockieren, bevor die Init-Nachricht gesendet wird:

* Setzen Sie [`MCP_CONNECTION_NONBLOCKING`](/docs/de/env-vars) auf `0`, um den gesamten Verbindungsstapel zu blockieren. Claude Code begrenzt diese Wartezeit standardmäßig auf 5 Sekunden. Passen Sie die Obergrenze mit der Umgebungsvariablen [`MCP_CONNECT_TIMEOUT_MS`](/docs/de/env-vars) in Millisekunden an. Server, die bei dieser Frist noch ausstehend sind, bleiben im Hintergrund verbunden.
* Setzen Sie `alwaysLoad: true` in der Konfiguration eines Servers, um seine Werkzeuge mit ihren vollständigen Schemas ab der ersten Runde verfügbar zu machen, [ausgenommen von der Werkzeugsuche-Aufschubung](/docs/de/mcp#exempt-a-server-from-deferral). Claude Code wartet beim Start auf die Werkzeuge dieses Servers, begrenzt auf die gleiche Frist, während andere Server im Hintergrund verbunden bleiben; ein Remote-Server mit einer zwischengespeicherten Werkzeugliste stellt sie ohne Verbindung bereit, wie in der obigen Tabelle angegeben.

Die `system`-Nachricht mit dem Subtyp `init` meldet den Status jedes Servers zum Zeitpunkt ihrer Emission; siehe [Fehlerbehandlung](#error-handling) zum Lesen dieser Status.

<h2 id="allow-mcp-tools">
  MCP-Tools zulassen
</h2>

MCP-Tools erfordern eine explizite Genehmigung, bevor Claude sie verwenden kann. Ohne Genehmigung sieht Claude, dass Tools verfügbar sind, kann sie aber nicht aufrufen.

<h3 id="tool-naming-convention">
  Benennungskonvention für Tools
</h3>

MCP-Tools folgen dem Benennungsmuster `mcp__<server-name>__<tool-name>`. Beispielsweise wird ein GitHub-Server mit dem Namen `"github"` mit einem `list_issues`-Tool zu `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Automatische Genehmigung mit allowedTools
</h3>

Verwenden Sie `allowedTools`, um bestimmte MCP-Tools automatisch zu genehmigen, damit Claude sie ohne Genehmigungsaufforderung verwenden kann:

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

Platzhalter (`*`) ermöglichen es Ihnen, alle Tools von einem Server zuzulassen, ohne jedes einzeln aufzulisten.

<Note>
  **Bevorzugen Sie `allowedTools` gegenüber Berechtigungsmodi für MCP-Zugriff.** `permissionMode: "acceptEdits"` genehmigt MCP-Tools nicht automatisch (nur Dateibearbeitungen und Dateisystem-Bash-Befehle). `permissionMode: "bypassPermissions"` genehmigt MCP-Tools automatisch, deaktiviert aber auch die meisten anderen Sicherheitsaufforderungen, was breiter ist als nötig; siehe [Wie Berechtigungen ausgewertet werden](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) für die verbleibenden Aufforderungen. Ein Platzhalter in `allowedTools` gewährt genau den MCP-Server, den Sie möchten, und nichts mehr. Siehe [Berechtigungsmodi](/docs/de/agent-sdk/permissions#permission-modes) für einen vollständigen Vergleich.
</Note>

<h3 id="discover-available-tools">
  Verfügbare Tools entdecken
</h3>

Um zu sehen, welche Tools ein MCP-Server bereitstellt, überprüfen Sie die Dokumentation des Servers oder inspizieren Sie das `tools`-Array in der `system`-Init-Nachricht. MCP-Tool-Namen beginnen mit `mcp__`.

Claude Code gibt die Init-Nachricht nach der [Verbindungswartzeit beim ersten Durchgang](#connection-timing) für Server aus, die in `options.mcpServers` übergeben werden, sodass das `tools`-Array die `mcp__`-Tools jedes Servers auflistet, der bis dahin verbunden ist, plus die von Servern mit einer [zwischengespeicherten Tool-Liste](#connection-timing), die beim ersten Gebrauch verbunden werden. Tools von anderen Servern, die noch nicht verbunden sind, fehlen; siehe [Fehlerbehandlung](#error-handling) zum Lesen des Status jedes Servers.

Dieser Filter gibt die MCP-Tool-Namen aus:

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

Sie können Claude auch bitten, die verfügbaren Tools von einem Server aufzulisten.

<h2 id="transport-types">
  Transporttypen
</h2>

MCP-Server kommunizieren mit Ihrem Agenten über verschiedene Transportprotokolle. Überprüfen Sie die Dokumentation des Servers, um zu sehen, welcher Transport unterstützt wird:

* Wenn die Dokumentation Ihnen einen **Befehl zum Ausführen** gibt (wie `npx @modelcontextprotocol/server-filesystem`), verwenden Sie stdio
* Wenn die Dokumentation Ihnen eine **URL** gibt, verwenden Sie HTTP oder SSE
* Wenn Sie Ihre eigenen Tools im Code erstellen, verwenden Sie einen SDK MCP-Server

<h3 id="stdio-servers">
  stdio-Server
</h3>

Lokale Prozesse, die über stdin/stdout kommunizieren. Verwenden Sie dies für MCP-Server, die Sie auf demselben Computer ausführen. Für das `.mcp.json`-Format verwenden Sie die gleichen Felder wie unter [From a config file](#from-a-config-file). Im Code übergeben Sie den Befehl und seine Argumente. Ersetzen Sie `/Users/me/projects` durch ein Verzeichnis auf Ihrem Computer:

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
  HTTP/SSE-Server
</h3>

Verwenden Sie HTTP oder SSE für Cloud-gehostete MCP-Server und Remote-APIs. Für das `.mcp.json`-Format verwenden Sie die gleichen Felder wie im Beispiel unter [HTTP headers for remote servers](#http-headers-for-remote-servers), mit `"type": "sse"` für einen SSE-Server. Im Code übergeben Sie die URL des Servers:

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

Für den streamfähigen HTTP-Transport verwenden Sie stattdessen `"type": "http"`. In `.mcp.json` und anderen JSON-Konfigurationsdateien wird `"streamable-http"` als Alias für `"http"` akzeptiert. Der `McpHttpServerConfig`-Typ der SDKs deklariert nur `"http"`, verwenden Sie also `"http"` für Server, die Sie im Code übergeben.

<h3 id="sdk-mcp-servers">
  SDK MCP-Server
</h3>

Definieren Sie benutzerdefinierte Tools direkt in Ihrem Anwendungscode, anstatt einen separaten Serverprozess auszuführen. Weitere Implementierungsdetails finden Sie im [custom tools guide](/docs/de/agent-sdk/custom-tools).

Ein SDK MCP-Server, der von einer [`initialize`-Steueranforderung](/docs/de/agent-sdk/typescript#sdkcontrolinitializeresponse) registriert wird, beginnt die Verbindung, sobald Claude Code die Anforderung verarbeitet.

<h2 id="mcp-tool-search">
  MCP-Werkzeugsuche
</h2>

Wenn Sie viele MCP-Werkzeuge konfiguriert haben, können Werkzeugdefinitionen einen erheblichen Teil Ihres Kontextfensters verbrauchen. Die Werkzeugsuche löst dieses Problem, indem sie Werkzeugdefinitionen aus dem Kontext vorenthält und nur die Werkzeuge lädt, die Claude für jeden Durchgang benötigt.

Die Werkzeugsuche ist standardmäßig aktiviert. Weitere Informationen zu Konfigurationsoptionen, Best Practices und der Verwendung der Werkzeugsuche mit benutzerdefinierten SDK-Werkzeugen finden Sie unter [Werkzeugsuche](/docs/de/agent-sdk/tool-search).

<h2 id="authentication">
  Authentifizierung
</h2>

Die meisten MCP-Server erfordern eine Authentifizierung für den Zugriff auf externe Dienste. Übergeben Sie Anmeldedaten über Umgebungsvariablen in der Serverkonfiguration.

<h3 id="pass-credentials-via-environment-variables">
  Anmeldedaten über Umgebungsvariablen übergeben
</h3>

Verwenden Sie das Feld `env`, um API-Schlüssel, Token und andere Anmeldedaten an den MCP-Server zu übergeben:

<Tabs>
  <Tab title="Im Code">
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

    Die Syntax `${API_KEY}` erweitert Umgebungsvariablen zur Laufzeit.
  </Tab>
</Tabs>

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

    Die Syntax `${API_TOKEN}` erweitert Umgebungsvariablen zur Laufzeit.
  </Tab>
</Tabs>

Ein vollständiges funktionierendes Beispiel eines Remote-Servers mit Header-Authentifizierung finden Sie unter [Probleme aus einem Repository auflisten](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  OAuth2-Authentifizierung
</h3>

Die [MCP-Spezifikation unterstützt OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) für die Autorisierung. Das SDK öffnet keinen Browser und führt keinen interaktiven OAuth-Flow aus. Wenn ein konfigurierter Server eine Autorisierungsanforderung zurückgibt und kein gespeichertes Token verfügbar ist, wird die Agent-Ausführung ohne die Tools dieses Servers fortgesetzt, und der Server meldet den Status `needs-auth`. Das Array `mcp_servers` der [System-Init-Nachricht](/docs/de/agent-sdk/typescript#sdksystemmessage) kann für diesen Server immer noch `pending` anzeigen, wenn es ausgegeben wird. Um zu bestätigen, ob ein Server Anmeldedaten benötigt, fragen Sie `mcpServerStatus()` im TypeScript SDK oder [`get_mcp_status()`](/docs/de/agent-sdk/python#methods) in Python ab.

Um Anmeldedaten bereitzustellen, führen Sie den OAuth-Flow in Ihrer eigenen Anwendung durch und übergeben Sie das resultierende Zugriffs-Token in den `headers` des Servers:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Nach Abschluss des OAuth-Flows in Ihrer App.
  // Implementieren Sie getAccessTokenFromOAuthFlow für Ihren OAuth-Provider.
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
  # Nach Abschluss des OAuth-Flows in Ihrer App.
  # Implementieren Sie get_access_token_from_oauth_flow für Ihren OAuth-Provider.
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

Dieses Beispiel verbindet sich mit dem Remote-[GitHub MCP Server](https://github.com/github/github-mcp-server), um aktuelle Probleme aufzulisten. Das Beispiel enthält Debug-Protokollierung, um die MCP-Verbindung und Toolaufrufe zu überprüfen.

Erstellen Sie vor der Ausführung ein [GitHub Personal Access Token](https://github.com/settings/personal-access-tokens) mit Lesezugriff auf die Repositories, die Sie abfragen möchten, und legen Sie es als Umgebungsvariable fest:

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

In der Zeile `MCP servers:` bestätigt ein `status` von `connected` für `github`, dass das Token funktioniert. Wenn Claude Code eine [zwischengespeicherte Toolliste](#connection-timing) für den Server hat, kann der Status stattdessen `pending` anzeigen und der Server verbindet sich beim ersten Toolaufruf. Wenn der Status `failed` oder `needs-auth` ist, siehe [Fehlerbehandlung](#error-handling), bevor Sie dem Ergebnis vertrauen, da Claude auf integrierte Tools zurückgreifen kann, wenn der Server nicht verfügbar ist.

<h3 id="query-a-database">
  Eine Datenbank abfragen
</h3>

Dieses Beispiel verwendet [DBHub](https://github.com/bytebase/dbhub), um eine Postgres-Datenbank abzufragen. Der Agent erkennt das Datenbankschema automatisch, schreibt die SQL-Abfrage und gibt die Ergebnisse zurück.

Das `execute_sql`-Tool von DBHub führt jede SQL aus, die der Agent ausgibt, einschließlich Schreibvorgänge, es sei denn, Sie beschränken es. Das Setzen von `readonly = true` in der [DBHub-Konfigurationsdatei](https://dbhub.ai/config/toml) bewirkt, dass DBHub `INSERT`-, `UPDATE`-, `DELETE`- und DDL-Anweisungen ablehnt, sodass das Beispiel Ihre Daten nicht ändern kann, selbst wenn der Agent einen Schreibvorgang ausgibt. DBHub löst `${DATABASE_URL}` aus der Prozessumgebung auf, wenn es die Konfiguration lädt, sodass die Verbindungszeichenfolge aus der Datei bleibt. Erstellen Sie diese `dbhub.toml` neben Ihrem Skript:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

Das Skript verweist dann DBHub auf die Konfigurationsdatei, anstatt eine Verbindungszeichenfolge direkt zu übergeben. Legen Sie vor der Ausführung die Umgebungsvariable `DATABASE_URL` auf Ihre Verbindungszeichenfolge fest. Ersetzen Sie die Platzhalterwerte durch Ihre eigenen Datenbankdetails:

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
  Fehlerbehandlung
</h2>

MCP-Server können aus verschiedenen Gründen keine Verbindung herstellen: Der Server-Prozess ist möglicherweise nicht installiert, die Anmeldedaten könnten ungültig sein, oder ein Remote-Server könnte unerreichbar sein.

Claude Code sendet eine `system`-Nachricht mit dem Subtyp `init` am Anfang jeder Abfrage. Diese Nachricht enthält den Verbindungsstatus für jeden MCP-Server. Das Feld `status` kann `"pending"`, `"connected"`, `"failed"`, `"needs-auth"` oder `"disabled"` sein. Claude Code sendet die Init-Nachricht nach dem [Wartezeit für die Verbindung beim ersten Durchgang](#connection-timing) für Server, die in `options.mcpServers` übergeben werden, sodass ein solcher Server, der sich innerhalb der Wartezeit verbunden hat, `"connected"` anzeigt.

In der Init-Nachricht sollten Sie `"pending"` nicht als Fehler an sich behandeln. Es kann eines der folgenden bedeuten:

* Der Server hat sich noch nicht verbunden. Siehe [wie lange Claude Code darauf wartet, bevor der erste Durchgang beginnt](#connection-timing)
* Die Werkzeugliste des Servers wurde [aus dem Cache bereitgestellt](#connection-timing), mit einer Verbindung bei der ersten Verwendung
* Die Verbindungsfrist ist abgelaufen. Ein solcher Server meldet `"pending"` oder `"failed"` je nach Zeitpunkt

Prüfen Sie auf `"failed"` oder `"needs-auth"`, um Server zu erkennen, die nicht verwendbar sind:

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

Der Status eines Remote-Servers kann sich auch ändern, nachdem er `"connected"` meldet. Wenn die Verbindung zu ihm während einer Sitzung unterbrochen wird, verschiebt Claude Code den Server zurück zu `"pending"`, während [Wiederverbindung](/docs/de/mcp#automatic-reconnection) stattfindet. Ein späterer `mcpServerStatus()`-Aufruf in TypeScript oder [`ClaudeSDKClient.get_mcp_status()`](/docs/de/agent-sdk/python#methods) in Python kann dann `"pending"` für einen Server melden, den Sie zuvor verbunden gesehen haben, ohne dass sich die Konfiguration auf Ihrer Seite ändert.

Nach fünf fehlgeschlagenen Wiederverbindungsversuchen meldet der Server `"failed"` oder `"needs-auth"`, wenn er erneut autorisiert werden muss. Um manuell erneut zu versuchen, rufen Sie [`reconnectMcpServer()`](/docs/de/agent-sdk/typescript#methods) in TypeScript oder [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/de/agent-sdk/python#methods) in Python auf.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="server-shows-failed-status">
  Server zeigt Status „failed" an
</h3>

Überprüfen Sie die `init`-Nachricht, um zu sehen, welche Server keine Verbindung herstellen konnten:

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

Ein Status `"pending"` bedeutet nicht, dass der Server fehlgeschlagen ist. Siehe [Fehlerbehandlung](#error-handling) für die Fälle, die er bei der Initialisierung abdeckt. Um später in der Sitzung aktualisierte Status zu erhalten, rufen Sie die Methode `mcpServerStatus()` der Abfrage im TypeScript SDK auf, oder [`ClaudeSDKClient.get_mcp_status()`](/docs/de/agent-sdk/python#methods) in Python.

Häufige Ursachen:

* **Fehlende Umgebungsvariablen**: Stellen Sie sicher, dass erforderliche Token und Anmeldedaten gesetzt sind. Überprüfen Sie bei stdio-Servern, dass das Feld `env` dem entspricht, was der Server erwartet.
* **Server nicht installiert**: Überprüfen Sie bei `npx`-Befehlen, dass das Paket vorhanden ist und Node.js in Ihrem PATH liegt.
* **Ungültige Verbindungszeichenfolge**: Überprüfen Sie bei Datenbankservern das Format der Verbindungszeichenfolge und dass die Datenbank erreichbar ist.
* **Netzwerkprobleme**: Überprüfen Sie bei Remote-HTTP/SSE-Servern, dass die URL erreichbar ist und dass Firewalls die Verbindung zulassen.

<h3 id="tools-not-being-called">
  Tools werden nicht aufgerufen
</h3>

Wenn Claude Tools sieht, sie aber nicht verwendet, überprüfen Sie, dass Sie die Berechtigung mit `allowedTools` erteilt haben:

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
  Verbindungs-Timeouts
</h3>

MCP-Serververbindungen werden standardmäßig nach 30 Sekunden unterbrochen. Um zu ändern, wie lange ein laufender Tool-Aufruf dauern darf, setzen Sie [`MCP_TOOL_TIMEOUT`](/docs/de/env-vars). Wenn Ihr Server länger zum Starten braucht, schlägt die Verbindung fehl. Erhöhen Sie das Verbindungslimit mit der Umgebungsvariablen [`MCP_TIMEOUT`](/docs/de/env-vars) in Millisekunden. Für Server, die mehr Startzeit benötigen, sollten Sie auch Folgendes in Betracht ziehen:

* Verwendung eines leichteren Servers, falls verfügbar
* Vorwärmung des Servers vor dem Starten Ihres Agenten
* Überprüfung der Serverprotokolle auf langsame Initialisierungsursachen

In TypeScript können Sie das Tool-Aufruf-Limit für einen einzelnen [SDK MCP Server](#sdk-mcp-servers) setzen, indem Sie [`timeout` an `createSdkMcpServer()`](/docs/de/agent-sdk/typescript#createsdkmcpserver) übergeben.

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Tool-Ausgabe überschreitet maximale zulässige Token
</h3>

Das SDK wendet das gleiche MCP-Ausgabelimit wie Claude Code an. Wenn ein Tool-Ergebnis ohne Bildinhalt größer als 25.000 Token ist, speichert Claude Code die Ausgabe in einer Datei und ersetzt das Tool-Ergebnis durch eine Fehlermeldung, die den Dateipfad benennt, damit der Agent die Ausgabe in Teilen zurücklesen kann.

Erhöhen Sie das Limit mit der Umgebungsvariablen [`MAX_MCP_OUTPUT_TOKENS`](/docs/de/env-vars). Siehe [MCP-Ausgabelimits und Warnungen](/docs/de/mcp#mcp-output-limits-and-warnings) für das vollständige Verhalten, einschließlich wie ein Server ein höheres Pro-Tool-Limit mit der Anmerkung `anthropic/maxResultSizeChars` deklarieren kann.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* **[Anleitung für benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools)**: Erstellen Sie Ihren eigenen MCP-Server, der prozessintern mit Ihrer SDK-Anwendung ausgeführt wird
* **[Berechtigungen](/docs/de/agent-sdk/permissions)**: Kontrollieren Sie, welche MCP-Tools Ihr Agent mit `allowedTools` und `disallowedTools` verwenden kann
* **[TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript)**: Vollständige API-Referenz einschließlich MCP-Konfigurationsoptionen
* **[Python SDK-Referenz](/docs/de/agent-sdk/python)**: Vollständige API-Referenz einschließlich MCP-Konfigurationsoptionen
* **[MCP-Serververzeichnis](https://github.com/modelcontextprotocol/servers)**: Durchsuchen Sie verfügbare MCP-Server für Datenbanken, APIs und mehr
