> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK – Übersicht

> Erstellen Sie produktive KI-Agenten mit Claude Code als Bibliothek

Erstellen Sie KI-Agenten, die autonom Dateien lesen, Befehle ausführen, das Web durchsuchen, Code bearbeiten und vieles mehr. Das Agent SDK bietet Ihnen die gleichen Tools, die Agent-Schleife und das Kontextmanagement, die Claude Code antreiben, programmierbar in Python und TypeScript. Informationen zur Überlegung hinter dem Agent-Harness-Design finden Sie unter [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) im Blog.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

Das Agent SDK enthält integrierte Tools zum Lesen von Dateien, Ausführen von Befehlen und Bearbeiten von Code, sodass Ihr Agent sofort arbeiten kann, ohne dass Sie die Tool-Ausführung implementieren müssen. Tauchen Sie in den Schnellstart ein oder erkunden Sie echte Agenten, die mit dem SDK erstellt wurden:

<CardGroup cols={2}>
  <Card title="Schnellstart" icon="play" href="/docs/de/agent-sdk/quickstart">
    Erstellen Sie einen Fehlerbereinigungsagenten in wenigen Minuten
  </Card>

  <Card title="Beispielagenten" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    E-Mail-Assistent, Forschungsagent und mehr
  </Card>
</CardGroup>

<h2 id="get-started">
  Erste Schritte
</h2>

<Steps>
  <Step title="Installieren Sie das SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) ist ein schneller Python-Paketmanager, der virtuelle Umgebungen automatisch verwaltet:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Erstellen und aktivieren Sie eine virtuelle Umgebung und installieren Sie dann das Paket. Die Installation in einer virtuellen Umgebung vermeidet den Fehler `error: externally-managed-environment`, den System-Python bei neueren Debian-, Ubuntu- und Homebrew-Installationen für `pip install` außerhalb einer venv zurückgibt.

        Auf macOS oder Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Auf Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Wenn PowerShell `Activate.ps1` mit einem Ausführungsrichtlinienfehler blockiert, führen Sie zuerst `Set-ExecutionPolicy -Scope Process RemoteSigned` aus.

        Das Python-Paket erfordert Python 3.10 oder später. Wenn pip `No matching distribution found for claude-agent-sdk` meldet, ist Ihr Interpreter älter als 3.10. Führen Sie `python3 --version` auf macOS oder Linux oder `py --version` auf Windows aus, um dies zu überprüfen.
      </Tab>
    </Tabs>

    <Note>
      Das TypeScript SDK bündelt eine native Claude Code-Binärdatei für Ihre Plattform als optionale Abhängigkeit, sodass Sie Claude Code nicht separat installieren müssen.
    </Note>
  </Step>

  <Step title="Legen Sie Ihren API-Schlüssel fest">
    Rufen Sie einen API-Schlüssel aus der [Konsole](https://platform.claude.com/) ab und legen Sie ihn als Umgebungsvariable fest.

    Auf macOS oder Linux:

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    Auf Windows PowerShell:

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    Das SDK unterstützt auch Authentifizierung über Drittanbieter-API-Anbieter:

    * **Amazon Bedrock**: Setzen Sie die Umgebungsvariable `CLAUDE_CODE_USE_BEDROCK=1` und konfigurieren Sie AWS-Anmeldedaten
    * **Claude Platform on AWS**: Setzen Sie `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` und `ANTHROPIC_AWS_WORKSPACE_ID` und konfigurieren Sie AWS-Anmeldedaten
    * **Google Cloud's Agent Platform**: Setzen Sie die Umgebungsvariable `CLAUDE_CODE_USE_VERTEX=1` und konfigurieren Sie Google Cloud-Anmeldedaten
    * **Microsoft Azure**: Setzen Sie die Umgebungsvariable `CLAUDE_CODE_USE_FOUNDRY=1` und konfigurieren Sie Azure-Anmeldedaten

    Weitere Informationen finden Sie in den Einrichtungsleitfäden für [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry).

    <Note>
      Sofern nicht zuvor genehmigt, erlaubt Anthropic Drittentwicklern nicht, claude.ai-Anmeldungen oder Ratenlimits für ihre Produkte anzubieten, einschließlich Agenten, die auf dem Claude Agent SDK basieren. Verwenden Sie stattdessen die in diesem Dokument beschriebenen API-Schlüssel-Authentifizierungsmethoden.
    </Note>
  </Step>

  <Step title="Führen Sie Ihren ersten Agenten aus">
    Dieses Beispiel erstellt einen Agenten, der Dateien in Ihrem aktuellen Verzeichnis mit integrierten Tools auflistet.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**Bereit zum Erstellen?** Folgen Sie dem [Schnellstart](/docs/de/agent-sdk/quickstart), um einen Agenten zu erstellen, der Fehler in wenigen Minuten findet und behebt.

<h2 id="capabilities">
  Funktionen
</h2>

Alles, was Claude Code leistungsstark macht, ist im SDK verfügbar:

<Tabs>
  <Tab title="Integrierte Tools">
    Ihr Agent kann Dateien lesen, Befehle ausführen und Codebases sofort durchsuchen. Wichtige Tools sind:

    | Tool                                                                        | Was es tut                                                                                |
    | --------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
    | **Read**                                                                    | Lesen Sie jede Datei im Arbeitsverzeichnis                                                |
    | **Write**                                                                   | Erstellen Sie neue Dateien                                                                |
    | **Edit**                                                                    | Nehmen Sie präzise Änderungen an vorhandenen Dateien vor                                  |
    | **Bash**                                                                    | Führen Sie Terminalbefehle, Skripte und Git-Operationen aus                               |
    | **Monitor**                                                                 | Überwachen Sie ein Hintergrundskript und reagieren Sie auf jede Ausgabezeile als Ereignis |
    | **Glob**                                                                    | Suchen Sie Dateien nach Muster (`**/*.ts`, `src/**/*.py`)                                 |
    | **Grep**                                                                    | Durchsuchen Sie Dateiinhalte mit Regex                                                    |
    | **WebSearch**                                                               | Durchsuchen Sie das Web nach aktuellen Informationen                                      |
    | **WebFetch**                                                                | Rufen Sie Webseiteninhalte ab und analysieren Sie sie                                     |
    | **[AskUserQuestion](/docs/de/agent-sdk/user-input#handle-clarifying-questions)** | Stellen Sie dem Benutzer Klärungsfragen mit Mehrfachauswahloptionen                       |

    Dieses Beispiel erstellt einen Agenten, der Ihre Codebasis nach TODO-Kommentaren durchsucht:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Hooks">
    Führen Sie benutzerdefinierten Code an wichtigen Punkten im Agent-Lebenszyklus aus. SDK-Hooks verwenden Callback-Funktionen, um Agent-Verhalten zu validieren, zu protokollieren, zu blockieren oder zu transformieren.

    **Verfügbare Hooks:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit` und mehr.

    Dieses Beispiel protokolliert alle Dateiänderungen in einer Audit-Datei:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Weitere Informationen zu Hooks →](/docs/de/agent-sdk/hooks)
  </Tab>

  <Tab title="Subagenten">
    Spawnen Sie spezialisierte Agenten, um fokussierte Teilaufgaben zu bewältigen. Ihr Hauptagent delegiert Arbeit, und Subagenten berichten mit Ergebnissen zurück.

    Definieren Sie benutzerdefinierte Agenten mit spezialisierten Anweisungen. Subagenten werden über das Agent-Tool aufgerufen, daher fügen Sie `Agent` in `allowedTools` ein, um diese Aufrufe automatisch zu genehmigen:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
                      )
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    Nachrichten aus dem Kontext eines Subagenten enthalten ein `parent_tool_use_id`-Feld, mit dem Sie verfolgen können, welche Nachrichten zu welcher Subagenten-Ausführung gehören.

    [Weitere Informationen zu Subagenten →](/docs/de/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Verbinden Sie sich mit externen Systemen über das Model Context Protocol: Datenbanken, Browser, APIs und [hunderte mehr](https://github.com/modelcontextprotocol/servers).

    Dieses Beispiel verbindet den [Playwright MCP-Server](https://github.com/microsoft/playwright-mcp), um Ihrem Agenten Browser-Automatisierungsfunktionen zu geben:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Weitere Informationen zu MCP →](/docs/de/agent-sdk/mcp)
  </Tab>

  <Tab title="Berechtigungen">
    Kontrollieren Sie genau, welche Tools Ihr Agent verwenden kann. Erlauben Sie sichere Operationen, blockieren Sie gefährliche oder erfordern Sie Genehmigung für sensible Aktionen.

    <Note>
      Für interaktive Genehmigungseingabeaufforderungen und das `AskUserQuestion`-Tool siehe [Genehmigungen und Benutzereingaben verarbeiten](/docs/de/agent-sdk/user-input).
    </Note>

    Dieses Beispiel erstellt einen schreibgeschützten Agenten, der Code analysieren, aber nicht ändern kann. `allowed_tools` genehmigt `Read`, `Glob` und `Grep` vorab.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Weitere Informationen zu Berechtigungen →](/docs/de/agent-sdk/permissions)
  </Tab>

  <Tab title="Sitzungen">
    Behalten Sie den Kontext über mehrere Austausche hinweg bei. Claude merkt sich gelesene Dateien, durchgeführte Analysen und Gesprächsverlauf. Setzen Sie Sitzungen später fort oder verzweigen Sie sie, um verschiedene Ansätze zu erkunden.

    Dieses Beispiel erfasst die Sitzungs-ID aus der ersten Abfrage und setzt sie dann fort, um mit vollständigem Kontext fortzufahren:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Weitere Informationen zu Sitzungen →](/docs/de/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Claude Code-Funktionen
</h3>

Das SDK unterstützt auch die dateisystembasierte Konfiguration von Claude Code. Mit Standardoptionen lädt das SDK diese aus `.claude/` in Ihrem Arbeitsverzeichnis und `~/.claude/`. Um einzuschränken, welche Quellen geladen werden, setzen Sie `setting_sources` (Python) oder `settingSources` (TypeScript) in Ihren Optionen.

| Funktion                                         | Beschreibung                                                                                          | Speicherort                             |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------- | --------------------------------------- |
| [Skills](/docs/de/agent-sdk/skills)                   | Spezialisierte Funktionen, die Claude automatisch verwendet oder die Sie mit `/name` aufrufen         | `.claude/skills/*/SKILL.md`             |
| [Befehle](/docs/de/agent-sdk/slash-commands)          | Benutzerdefinierte Befehle im Legacy-Format. Verwenden Sie Skills für neue benutzerdefinierte Befehle | `.claude/commands/*.md`                 |
| [Memory](/docs/de/agent-sdk/modifying-system-prompts) | Projektkontext und Anweisungen                                                                        | `CLAUDE.md` oder `.claude/CLAUDE.md`    |
| [Plugins](/docs/de/agent-sdk/plugins)                 | Erweitern Sie mit Skills, Agenten, Hooks und MCP-Servern                                              | Programmgesteuert über `plugins`-Option |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Vergleichen Sie das Agent SDK mit anderen Claude-Tools
</h2>

Die Claude-Plattform bietet mehrere Möglichkeiten, mit Claude zu erstellen. So passt das Agent SDK:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    Das [Anthropic Client SDK](https://platform.claude.com/docs/de/api/client-sdks) bietet Ihnen direkten API-Zugriff: Sie senden Eingabeaufforderungen und implementieren die Tool-Ausführung selbst. Das **Agent SDK** bietet Ihnen Claude mit integrierter Tool-Ausführung.

    Mit dem Client SDK implementieren Sie eine Tool-Schleife. Mit dem Agent SDK handhabt Claude es:

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    Gleiche Funktionen, andere Schnittstelle:

    | Anwendungsfall                 | Beste Wahl |
    | ------------------------------ | ---------- |
    | Interaktive Entwicklung        | CLI        |
    | CI/CD-Pipelines                | SDK        |
    | Benutzerdefinierte Anwendungen | SDK        |
    | Einmalige Aufgaben             | CLI        |
    | Produktionsautomatisierung     | SDK        |

    Viele Teams verwenden beide: CLI für die tägliche Entwicklung, SDK für die Produktion. Workflows lassen sich direkt zwischen ihnen übersetzen.
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/de/managed-agents/overview) ist eine gehostete REST-API: Anthropic führt den Agent und die Sandbox aus, und Ihre Anwendung sendet Ereignisse und streamt Ergebnisse zurück. Das **Agent SDK** ist eine Bibliothek, die die Agent-Schleife in Ihrem eigenen Prozess ausführt.

    |                              | Agent SDK                                                                                     | Managed Agents                                                                                                 |
    | ---------------------------- | --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
    | **Läuft in**                 | Ihr Prozess, Ihre Infrastruktur                                                               | Von Anthropic verwaltete Infrastruktur                                                                         |
    | **Schnittstelle**            | Python- oder TypeScript-Bibliothek                                                            | REST-API                                                                                                       |
    | **Agent arbeitet an**        | Dateien in Ihrer Infrastruktur                                                                | Eine verwaltete Sandbox pro Sitzung                                                                            |
    | **Sitzungsstatus**           | JSONL auf Ihrem Dateisystem                                                                   | Von Anthropic gehostetes Ereignisprotokoll                                                                     |
    | **Benutzerdefinierte Tools** | In-Process-Python- oder TypeScript-Funktionen                                                 | Claude löst das Tool aus; Sie führen es aus und geben Ergebnisse zurück                                        |
    | **Am besten für**            | Lokale Prototypisierung, Agents, die direkt auf Ihrem Dateisystem und Ihren Diensten arbeiten | Produktions-Agents ohne Betrieb von Sandbox- oder Sitzungsinfrastruktur, langfristige und asynchrone Sitzungen |

    Ein häufiger Weg ist die Prototypisierung mit dem Agent SDK lokal und dann der Wechsel zu Managed Agents für die Produktion.
  </Tab>
</Tabs>

<h2 id="changelog">
  Änderungsprotokoll
</h2>

Sehen Sie sich das vollständige Änderungsprotokoll für SDK-Updates, Fehlerbehebungen und neue Funktionen an:

* **TypeScript SDK**: [CHANGELOG.md anzeigen](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [CHANGELOG.md anzeigen](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Fehler melden
</h2>

Wenn Sie auf Fehler oder Probleme mit dem Agent SDK stoßen:

* **TypeScript SDK**: [Probleme auf GitHub melden](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [Probleme auf GitHub melden](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Richtlinien für die Markennutzung
</h2>

Für Partner, die das Claude Agent SDK integrieren, ist die Verwendung von Claude-Branding optional. Wenn Sie Claude in Ihrem Produkt referenzieren:

**Erlaubt:**

* „Claude Agent" (bevorzugt für Dropdown-Menüs)
* „Claude" (wenn bereits in einem Menü mit der Bezeichnung „Agents")
* „{YourAgentName} Powered by Claude" (wenn Sie einen vorhandenen Agentennamen haben)

**Nicht erlaubt:**

* „Claude Code" oder „Claude Code Agent"
* Claude Code-Branding ASCII-Art oder visuelle Elemente, die Claude Code nachahmen

Ihr Produkt sollte sein eigenes Branding beibehalten und nicht wie Claude Code oder ein anderes Anthropic-Produkt aussehen. Wenden Sie sich bei Fragen zur Markenkonformität an das Anthropic-[Vertriebsteam](https://www.anthropic.com/contact-sales).

<h2 id="license-and-terms">
  Lizenz und Bedingungen
</h2>

Die Verwendung des Claude Agent SDK unterliegt den [Anthropic Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms), auch wenn Sie es verwenden, um Produkte und Dienste bereitzustellen, die Sie Ihren eigenen Kunden und Endbenutzern zur Verfügung stellen, außer soweit eine bestimmte Komponente oder Abhängigkeit unter einer anderen Lizenz abgedeckt ist, wie in der LICENSE-Datei dieser Komponente angegeben.

<h2 id="next-steps">
  Nächste Schritte
</h2>

<CardGroup cols={2}>
  <Card title="Schnellstart" icon="play" href="/docs/de/agent-sdk/quickstart">
    Erstellen Sie einen Agenten, der Fehler in wenigen Minuten findet und behebt
  </Card>

  <Card title="Beispielagenten" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    E-Mail-Assistent, Forschungsagent und mehr
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/de/agent-sdk/typescript">
    Vollständige TypeScript-API-Referenz und Beispiele
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/de/agent-sdk/python">
    Vollständige Python-API-Referenz und Beispiele
  </Card>
</CardGroup>
