> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Schnellstart

> Erste Schritte mit dem Python- oder TypeScript-Agent-SDK zum Erstellen von KI-Agenten, die autonom funktionieren

Verwenden Sie das Agent SDK, um einen KI-Agenten zu erstellen, der Ihren Code liest, Fehler findet und behebt – alles ohne manuelle Eingriffe.

**Das werden Sie tun:**

1. Ein Projekt mit dem Agent SDK einrichten
2. Eine Datei mit fehlerhaftem Code erstellen
3. Einen Agenten ausführen, der Fehler automatisch findet und behebt

<h2 id="prerequisites">
  Voraussetzungen
</h2>

* **Node.js 18+** oder **Python 3.10+**
* Ein **Anthropic-Konto**. Falls Sie noch kein Konto haben, [registrieren Sie sich hier](https://platform.claude.com/).

<h2 id="setup">
  Einrichtung
</h2>

<Steps>
  <Step title="Erstellen Sie einen Projektordner">
    Erstellen Sie ein neues Verzeichnis für diesen Schnellstart:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Für Ihre eigenen Projekte können Sie das SDK aus jedem Ordner ausführen; es hat standardmäßig Zugriff auf Dateien in diesem Verzeichnis und seinen Unterverzeichnissen.
  </Step>

  <Step title="Installieren Sie das SDK">
    Installieren Sie das Agent SDK-Paket für Ihre Sprache:

    <Tabs>
      <Tab title="TypeScript (neues Projekt)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Das Setzen von `"type": "module"` in `package.json` ermöglicht Ihrem Agent-Skript die Verwendung von Top-Level-`await`, und [tsx](https://tsx.hirok.io) führt TypeScript-Dateien direkt aus. npm gibt `added N packages` aus, wenn die Installation erfolgreich ist.
      </Tab>

      <Tab title="TypeScript (bestehendes Projekt)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.hirok.io) führt TypeScript-Dateien direkt aus. Wenn Ihr Projekt CommonJS verwendet, benennen Sie Ihr Agent-Skript `agent.mts` statt `agent.ts`. Die `.mts`-Erweiterung veranlasst tsx, die Datei als ES-Modul zu behandeln, sodass Top-Level-`await` funktioniert, ohne Ihr gesamtes Projekt in ES-Module zu konvertieren. Verwenden Sie `agent.mts` anstelle von `agent.ts` in den Erstellungs- und Ausführungsschritten später in diesem Schnellstart.
      </Tab>

      <Tab title="Python (uv)">
        [Installieren Sie uv](https://docs.astral.sh/uv/), einen schnellen Python-Paketmanager, der virtuelle Umgebungen automatisch verwaltet. Initialisieren Sie dann ein Projekt und fügen Sie das SDK hinzu:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Erstellen und aktivieren Sie eine virtuelle Umgebung und installieren Sie dann das Paket.

        Auf macOS oder Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Unter Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Wenn PowerShell `Activate.ps1` mit einem Ausführungsrichtlinienfehler blockiert, führen Sie zuerst `Set-ExecutionPolicy -Scope Process RemoteSigned` aus.
      </Tab>
    </Tabs>

    <Note>
      Sowohl das TypeScript als auch das Python SDK bündeln eine native Claude Code-Binärdatei, daher benötigen die meisten Installationen keine separate Claude Code-Installation. Einige Installationen haben keine gebündelte Binärdatei:

      * Wenn pip die Python SDK-Quelldistribution anstelle eines Plattform-Wheels installiert, beispielsweise auf ARM64 Windows, wird keine Binärdatei gebündelt. [Installieren Sie Claude Code nativ](/docs/de/setup#install-claude-code). Das Python SDK findet es auf Ihrem `PATH`.
      * Das TypeScript SDK installiert seine Binärdatei über npm optionale Abhängigkeiten, daher erhält eine Installation, die diese überspringt, beispielsweise `npm ci --omit=optional`, auch auf einer unterstützten Plattform keine Binärdatei. Installieren Sie erneut, ohne optionale Abhängigkeiten zu überspringen, oder [installieren Sie Claude Code nativ](/docs/de/setup#install-claude-code) und setzen Sie `pathToClaudeCodeExecutable` auf seinen Pfad.
    </Note>
  </Step>

  <Step title="Legen Sie Ihren API-Schlüssel fest">
    Rufen Sie einen API-Schlüssel von der [Claude-Konsole](https://platform.claude.com/) ab und legen Sie ihn dann als Umgebungsvariable in der Shell fest, in der Sie Ihren Agenten ausführen:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    Das SDK liest den Schlüssel aus der Umgebung des Prozesses, der Ihren Agenten ausführt; es lädt `.env`-Dateien nicht automatisch. Wenn Sie den Schlüssel in einer `.env`-Datei speichern, laden Sie ihn selbst, beispielsweise mit dem `dotenv`-Paket, bevor Sie das SDK aufrufen.

    Das SDK unterstützt auch Authentifizierung über Drittanbieter-API-Anbieter:

    * **Amazon Bedrock**: Setzen Sie die Umgebungsvariable `CLAUDE_CODE_USE_BEDROCK=1` und konfigurieren Sie AWS-Anmeldedaten
    * **Claude Platform on AWS**: Setzen Sie `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` und `ANTHROPIC_AWS_WORKSPACE_ID` und konfigurieren Sie AWS-Anmeldedaten
    * **Google Cloud's Agent Platform**: Setzen Sie die Umgebungsvariable `CLAUDE_CODE_USE_VERTEX=1` und konfigurieren Sie Google Cloud-Anmeldedaten
    * **Microsoft Foundry**: Setzen Sie die Umgebungsvariable `CLAUDE_CODE_USE_FOUNDRY=1` und konfigurieren Sie Azure-Anmeldedaten

    Weitere Informationen finden Sie in den Einrichtungsleitfäden für [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry).

    <Note>
      Sofern nicht zuvor genehmigt, erlaubt Anthropic Drittentwicklern nicht, claude.ai-Anmeldungen oder Ratenlimits für ihre Produkte anzubieten, einschließlich Agenten, die auf dem Claude Agent SDK basieren. Verwenden Sie stattdessen die in diesem Dokument beschriebenen API-Schlüssel-Authentifizierungsmethoden.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Erstellen Sie eine fehlerhafte Datei
</h2>

Dieser Schnellstart führt Sie durch die Erstellung eines Agenten, der Fehler im Code finden und beheben kann. Zunächst benötigen Sie eine Datei mit einigen absichtlichen Fehlern, die der Agent beheben kann. Erstellen Sie `utils.py` im Verzeichnis `my-agent` und fügen Sie den folgenden Code ein:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Dieser Code hat zwei Fehler:

1. `calculate_average([])` stürzt mit Division durch Null ab
2. `get_user_name(None)` stürzt mit einem TypeError ab

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Erstellen Sie einen Agenten, der Fehler findet und behebt
</h2>

Erstellen Sie `agent.py`, wenn Sie das Python SDK verwenden, oder `agent.ts` für TypeScript. Verwenden Sie `agent.mts` stattdessen, wenn Ihr bestehendes Projekt CommonJS verwendet:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Dieser Code hat drei Hauptteile:

1. **`query`**: der Haupteinstiegspunkt, der die agentic loop erstellt. Er gibt einen asynchronen Iterator zurück, daher verwenden Sie `async for`, um Nachrichten zu streamen, während Claude arbeitet. Siehe die vollständige API in der [Python](/docs/de/agent-sdk/python#query) oder [TypeScript](/docs/de/agent-sdk/typescript#query) SDK-Referenz.

2. **`prompt`**: was Sie Claude tun möchten. Claude ermittelt basierend auf der Aufgabe, welche Tools verwendet werden sollen.

3. **`options`**: Konfiguration für den Agenten. Dieses Beispiel verwendet `allowedTools`, um `Read`, `Edit` und `Glob` vorab zu genehmigen, und `permissionMode: "acceptEdits"`, um Dateiänderungen automatisch zu genehmigen. Weitere Optionen sind `systemPrompt`, `mcpServers` und mehr. Siehe alle Optionen für [Python](/docs/de/agent-sdk/python#claudeagentoptions) oder [TypeScript](/docs/de/agent-sdk/typescript#options).

Die `async for`-Schleife läuft weiter, während Claude denkt, Tools aufruft, Ergebnisse beobachtet und entscheidet, was als nächstes zu tun ist. Jede Iteration ergibt eine Nachricht: Claudes Überlegung, ein Tool-Aufruf, ein Tool-Ergebnis oder das endgültige Ergebnis. Das SDK verwaltet die Orchestrierung, Tool-Ausführung, Kontextverwaltung und Wiederholungen, sodass Sie einfach den Stream verbrauchen. Die Schleife endet, wenn Claude die Aufgabe abschließt oder auf einen Fehler stößt.

Die Nachrichtenbehandlung in der Schleife filtert nach benutzerfreundlicher Ausgabe. Ohne Filterung würden Sie rohe Nachrichtenobjekte sehen, einschließlich Systeminitialisierung und internem Status, was zum Debuggen nützlich ist, aber sonst störend wirkt.

<Note>
  Dieses Beispiel verwendet Streaming, um den Fortschritt in Echtzeit anzuzeigen. Wenn Sie keine Live-Ausgabe benötigen (z. B. für Hintergrundaufträge oder CI-Pipelines), können Sie alle Nachrichten auf einmal sammeln. Weitere Informationen finden Sie unter [Streaming vs. Single-Turn-Modus](/docs/de/agent-sdk/streaming-vs-single-mode).
</Note>

<h3 id="run-your-agent">
  Führen Sie Ihren Agenten aus
</h3>

Ihr Agent ist bereit. Führen Sie ihn mit dem folgenden Befehl aus:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Wenn Sie Ihr Skript `agent.mts` genannt haben, führen Sie stattdessen `npx tsx agent.mts` aus.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    Mit Ihrer noch aktivierten virtuellen Umgebung:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

Während es arbeitet, druckt der Agent seine Überlegungen und jeden Tool-Aufruf aus und endet mit `Done: success`. Nach der Ausführung überprüfen Sie `utils.py`. Sie sehen defensiven Code, der leere Listen und Null-Benutzer verarbeitet. Ihr Agent hat autonom:

1. **Gelesen** `utils.py`, um den Code zu verstehen
2. **Analysiert** die Logik und identifiziert Grenzfälle, die zum Absturz führen würden
3. **Bearbeitet** die Datei, um ordnungsgemäße Fehlerbehandlung hinzuzufügen

Das macht das Agent SDK anders: Claude führt Tools direkt aus, anstatt Sie zu bitten, sie zu implementieren.

<Note>
  Wenn Sie einen Authentifizierungsfehler wie `Not logged in` oder `Invalid API key` sehen, stellen Sie sicher, dass Sie die Umgebungsvariable `ANTHROPIC_API_KEY` in der Shell gesetzt haben, in der Sie Ihren Agenten ausführen. Das SDK lädt `.env`-Dateien nicht automatisch.

  Informationen zu den Ursachen und Lösungen dieser und anderer Authentifizierungsfehler finden Sie unter [Authentifizierungsfehler](/docs/de/errors#authentication-errors) in der Fehlerreferenz.
</Note>

<h3 id="try-other-prompts">
  Versuchen Sie andere Prompts
</h3>

Jetzt, da Ihr Agent eingerichtet ist, versuchen Sie einige verschiedene Prompts:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Passen Sie Ihren Agenten an
</h3>

Sie können das Verhalten Ihres Agenten ändern, indem Sie die Optionen ändern. Hier sind einige Beispiele:

**Fügen Sie Web-Suchfunktion hinzu:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Geben Sie Claude einen benutzerdefinierten System-Prompt:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Führen Sie Befehle im Terminal aus:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

Mit aktiviertem `Bash` versuchen Sie: `"Write unit tests for utils.py, run them, and fix any failures"`

Jedes dieser Snippets setzt Felder auf demselben Optionsobjekt. Weitere Informationen finden Sie unter [Konfigurieren Sie Ihren Agenten](/docs/de/agent-sdk/configuration).

<h2 id="key-concepts">
  Wichtige Konzepte
</h2>

**Tools** steuern, was Ihr Agent tun kann:

| Tools                                  | Was der Agent tun kann       |
| -------------------------------------- | ---------------------------- |
| `Read`, `Glob`, `Grep`                 | Schreibgeschützte Analyse    |
| `Read`, `Edit`, `Glob`                 | Code analysieren und ändern  |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Vollständige Automatisierung |

**Genehmigungsmodi** steuern, wie viel menschliche Aufsicht Sie möchten. Das SDK wertet den aktiven Modus zusammen mit Ihren Allow- und Deny-Regeln in einer festen Reihenfolge aus, die unter [Wie Berechtigungen ausgewertet werden](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) beschrieben ist. Die vollständige Liste der Modi, ihr Verhalten und wann Sie jeden verwenden sollten, finden Sie unter [Genehmigungsmodus in Wie die Agent-Schleife funktioniert](/docs/de/agent-sdk/agent-loop#permission-mode).

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Ihren ersten Agenten erstellt haben, erfahren Sie, wie Sie seine Funktionen erweitern und ihn an Ihren Anwendungsfall anpassen:

* **[Konfigurieren Sie Ihren Agenten](/docs/de/agent-sdk/configuration)**: Stellen Sie das Optionsobjekt zusammen und finden Sie die Seite, die jede Einstellung abdeckt
* **[Berechtigungen](/docs/de/agent-sdk/permissions)**: Steuern Sie, was Ihr Agent tun kann und wann er Genehmigung benötigt
* **[Hooks](/docs/de/agent-sdk/hooks)**: Führen Sie benutzerdefinierten Code vor oder nach Tool-Aufrufen aus
* **[Sitzungen](/docs/de/agent-sdk/sessions)**: Erstellen Sie Multi-Turn-Agenten, die den Kontext beibehalten
* **[MCP-Server](/docs/de/agent-sdk/mcp)**: Verbinden Sie sich mit Datenbanken, Browsern, APIs und anderen externen Systemen
* **[Hosting](/docs/de/agent-sdk/hosting)**: Stellen Sie Agenten in Docker, Cloud und CI/CD bereit
* **[Beispiel-Agenten](https://github.com/anthropics/claude-agent-sdk-demos)**: Siehe vollständige Beispiele: E-Mail-Assistent, Forschungsagent und mehr
* **[Fehlerbehebung](/docs/de/agent-sdk/troubleshooting)**: Beheben Sie Fehler, wenn die CLI nicht startet oder beendet wird, oder wenn ein Ergebnis ohne strukturierte Ausgabe ankommt
