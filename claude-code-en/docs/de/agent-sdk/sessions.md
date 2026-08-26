> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mit Sitzungen arbeiten

> Wie Sitzungen die Gesprächsverlauf des Agenten speichern, und wann Sie continue, resume und fork verwenden, um zu einem früheren Durchlauf zurückzukehren.

Eine Sitzung ist der Gesprächsverlauf, den das SDK während der Arbeit Ihres Agenten sammelt. Sie enthält Ihren Prompt, jeden Werkzeugaufruf, den der Agent durchgeführt hat, jedes Werkzeugergebnis und jede Antwort. Das SDK schreibt sie automatisch auf die Festplatte, damit Sie später zu ihr zurückkehren können.

Zu einer Sitzung zurückzukehren bedeutet, dass der Agent den vollständigen Kontext von zuvor hat: Dateien, die er bereits gelesen hat, Analysen, die er bereits durchgeführt hat, Entscheidungen, die er bereits getroffen hat. Sie können eine Anschlussfrage stellen, sich von einer Unterbrechung erholen oder einen anderen Ansatz ausprobieren.

<Note>
  Sitzungen speichern das **Gespräch**, nicht das Dateisystem. Um Dateiänderungen, die der Agent vorgenommen hat, zu erfassen und rückgängig zu machen, verwenden Sie [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing).
</Note>

Dieser Leitfaden behandelt, wie Sie den richtigen Ansatz für Ihre App wählen, die SDK-Schnittstellen, die Sitzungen automatisch verfolgen, wie Sie Sitzungs-IDs erfassen und `resume` und `fork` manuell verwenden, und was Sie über das Fortsetzen von Sitzungen über Hosts hinweg wissen müssen.

<h2 id="choose-an-approach">
  Wählen Sie einen Ansatz
</h2>

Wie viel Sitzungsverwaltung Sie benötigen, hängt von der Form Ihrer Anwendung ab. Die Sitzungsverwaltung kommt ins Spiel, wenn Sie mehrere Prompts senden, die Kontext teilen sollten. Innerhalb eines einzelnen `query()`-Aufrufs nimmt der Agent bereits so viele Durchläufe vor, wie nötig, und Berechtigungsprompts und `AskUserQuestion` werden [in-loop verarbeitet](/docs/de/agent-sdk/user-input) (sie beenden den Aufruf nicht).

| Was Sie erstellen                                                                          | Was Sie verwenden                                                                                                                                                                   |
| :----------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Einmalige Aufgabe: einzelner Prompt, keine Anschlussfrage                                  | Nichts Zusätzliches. Ein `query()`-Aufruf verarbeitet es.                                                                                                                           |
| Multi-Turn-Chat in einem Prozess                                                           | [`ClaudeSDKClient` (Python) oder `continue: true` (TypeScript)](#automatic-session-management). Das SDK verfolgt die Sitzung für Sie ohne ID-Verwaltung.                            |
| Dort weitermachen, wo Sie nach einem Prozessneustart aufgehört haben                       | `continue_conversation=True` (Python) / `continue: true` (TypeScript). Setzt die neueste Sitzung im Verzeichnis fort, keine ID erforderlich.                                        |
| Eine bestimmte frühere Sitzung fortsetzen (nicht die neueste)                              | Erfassen Sie die Sitzungs-ID und übergeben Sie sie an `resume`.                                                                                                                     |
| Einen alternativen Ansatz ausprobieren, ohne das Original zu verlieren                     | Forken Sie die Sitzung.                                                                                                                                                             |
| Zustandslose Aufgabe, möchten nichts auf die Festplatte geschrieben haben (nur TypeScript) | Setzen Sie [`persistSession: false`](/docs/de/agent-sdk/typescript#options). Die Sitzung existiert nur im Speicher für die Dauer des Aufrufs. Python speichert immer auf der Festplatte. |

<h3 id="continue-resume-and-fork">
  Continue, resume und fork
</h3>

Continue, resume und fork sind Optionsfelder, die Sie auf `query()` setzen ([`ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions) in Python, [`Options`](/docs/de/agent-sdk/typescript#options) in TypeScript).

**Continue** und **resume** holen beide eine vorhandene Sitzung ab und fügen sie hinzu. Der Unterschied liegt darin, wie sie diese Sitzung finden:

* **Continue** findet die neueste Sitzung im aktuellen Verzeichnis. Sie müssen nichts verfolgen. Funktioniert gut, wenn Ihre App jeweils ein Gespräch führt.
* **Resume** nimmt eine bestimmte Sitzungs-ID. Sie verfolgen die ID. Erforderlich, wenn Sie mehrere Sitzungen haben (z. B. eine pro Benutzer in einer Multi-User-App) oder zu einer zurückkehren möchten, die nicht die neueste ist.

**Fork** ist anders: Es erstellt eine neue Sitzung, die mit einer Kopie des Verlaufs des Originals beginnt. Das Original bleibt unverändert. Verwenden Sie fork, um eine andere Richtung auszuprobieren und gleichzeitig die Möglichkeit zu behalten, zurückzugehen.

<h2 id="automatic-session-management">
  Automatische Sitzungsverwaltung
</h2>

Beide SDKs bieten eine Schnittstelle, die den Sitzungszustand für Sie über Aufrufe hinweg verfolgt, sodass Sie IDs nicht manuell weitergeben müssen. Verwenden Sie diese für Multi-Turn-Gespräche innerhalb eines einzelnen Prozesses.

<h3 id="python-claudesdkclient">
  Python: `ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/de/agent-sdk/python#claudesdkclient) verwaltet Sitzungs-IDs intern. Jeder Aufruf von `client.query()` setzt automatisch die gleiche Sitzung fort. Rufen Sie [`client.receive_response()`](/docs/de/agent-sdk/python#claudesdkclient) auf, um über die Nachrichten für die aktuelle Abfrage zu iterieren. Verwenden Sie den Client als asynchronen Kontext-Manager, damit die Verbindungseinrichtung und das Herunterfahren für Sie verwaltet werden, oder rufen Sie `connect()` und `disconnect()` manuell auf.

Dieses Beispiel führt zwei Abfragen gegen denselben `client` aus. Die erste fordert den Agent auf, ein Modul zu analysieren; die zweite fordert ihn auf, dieses Modul zu refaktorieren. Da beide Aufrufe durch die gleiche Client-Instanz gehen, hat die zweite Abfrage den vollständigen Kontext aus der ersten, ohne explizites `resume` oder Sitzungs-ID:

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

Siehe die [Python SDK-Referenz](/docs/de/agent-sdk/python#choosing-between-query-and-claudesdkclient) für Details, wann Sie `ClaudeSDKClient` gegenüber der eigenständigen `query()`-Funktion verwenden sollten.

<h3 id="typescript-continue-true">
  TypeScript: `continue: true`
</h3>

Das TypeScript SDK hat kein Sitzungs-Halter-Client-Objekt wie Pythons `ClaudeSDKClient`. Übergeben Sie stattdessen `continue: true` bei jedem nachfolgenden `query()`-Aufruf und das SDK holt die neueste Sitzung im aktuellen Verzeichnis ab. Keine ID-Verfolgung erforderlich.

Dieses Beispiel macht zwei separate `query()`-Aufrufe. Der erste erstellt eine neue Sitzung; der zweite setzt `continue: true`, was dem SDK mitteilt, die neueste Sitzung auf der Festplatte zu finden und fortzusetzen. Der Agent hat den vollständigen Kontext aus dem ersten Aufruf:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
for await (const message of query({
  prompt: "Analyze the auth module",
  options: { allowedTools: ["Read", "Glob", "Grep"] }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  Die experimentelle [V2-Sitzungs-API](/docs/de/agent-sdk/typescript-v2-preview), die `createSession()` mit einem `send` / `stream`-Muster bereitstellte, wurde in TypeScript Agent SDK 0.3.142 entfernt. Verwenden Sie stattdessen die `query()`-Funktion und die auf dieser Seite beschriebenen Sitzungsoptionen.
</Note>

<h2 id="use-session-options-with-query">
  Verwenden Sie Sitzungsoptionen mit `query()`
</h2>

<h3 id="capture-the-session-id">
  Erfassen Sie die Sitzungs-ID
</h3>

Resume und fork erfordern eine Sitzungs-ID. Lesen Sie sie aus dem `session_id`-Feld in der Ergebnismeldung ([`ResultMessage`](/docs/de/agent-sdk/python#resultmessage) in Python, [`SDKResultMessage`](/docs/de/agent-sdk/typescript#sdkresultmessage) in TypeScript), die bei jedem Ergebnis unabhängig von Erfolg oder Fehler vorhanden ist. In TypeScript ist die ID auch früher als direktes Feld in der Init-`SystemMessage` verfügbar; in Python ist sie in `SystemMessage.data` verschachtelt.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      async for message in query(
          prompt="Analyze the auth module and suggest improvements",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id
              if message.subtype == "success":
                  print(message.result)

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  for await (const message of query({
    prompt: "Analyze the auth module and suggest improvements",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
      if (message.subtype === "success") {
        console.log(message.result);
      }
    }
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

<h3 id="resume-by-id">
  Fortsetzen nach ID
</h3>

Übergeben Sie eine Sitzungs-ID an `resume`, um zu dieser bestimmten Sitzung zurückzukehren. Der Agent setzt mit vollständigem Kontext von dort fort, wo die Sitzung endete. Häufige Gründe zum Fortsetzen:

* **Folgen Sie einer abgeschlossenen Aufgabe.** Der Agent hat bereits etwas analysiert; jetzt möchten Sie, dass er auf dieser Analyse handelt, ohne Dateien erneut zu lesen.
* **Erholen Sie sich von einem Limit.** Der erste Durchlauf endete mit `error_max_turns` oder `error_max_budget_usd` (siehe [Verarbeiten Sie das Ergebnis](/docs/de/agent-sdk/agent-loop#handle-the-result)); setzen Sie mit einem höheren Limit fort.
* **Starten Sie Ihren Prozess neu.** Sie haben die ID vor dem Herunterfahren erfasst und möchten das Gespräch wiederherstellen.

Dieses Beispiel setzt die Sitzung aus [Erfassen Sie die Sitzungs-ID](#capture-the-session-id) mit einer Anschlussfrage fort. Da Sie fortsetzen, hat der Agent die vorherige Analyse bereits im Kontext:

<CodeGroup>
  ```python Python theme={null}
  # Earlier session analyzed the code; now build on that analysis
  async for message in query(
      prompt="Now implement the refactoring you suggested",
      options=ClaudeAgentOptions(
          resume=session_id,
          allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Sie sollten eine Antwort sehen, die auf der früheren Analyse aufbaut, anstatt von vorne zu beginnen. Das bestätigt, dass der Agent die Sitzung mit seinem vorherigen Kontext intakt fortgesetzt hat.

<Tip>
  Wenn ein `resume`-Aufruf eine neue Sitzung statt des erwarteten Verlaufs zurückgibt, ist die häufigste Ursache ein nicht übereinstimmendes `cwd`. Sitzungen werden unter `~/.claude/projects/<encoded-cwd>/*.jsonl` gespeichert, oder unter `$CLAUDE_CONFIG_DIR/projects/<encoded-cwd>/*.jsonl`, wenn Sie die Umgebungsvariable `CLAUDE_CONFIG_DIR` setzen, wobei `<encoded-cwd>` das absolute Arbeitsverzeichnis mit jedem nicht-alphanumerischen Zeichen ist, das durch `-` ersetzt wird (also `/Users/me/proj` wird zu `-Users-me-proj`). Wenn Ihr Resume-Aufruf aus einem anderen Verzeichnis ausgeführt wird, sucht das SDK am falschen Ort. Die Sitzungsdatei muss auch auf dem aktuellen Computer vorhanden sein.
</Tip>

Um Sitzungen über Maschinen hinweg oder in serverlosen Umgebungen fortzusetzen, spiegeln Sie Transkripte mit einem [`SessionStore`-Adapter](/docs/de/agent-sdk/session-storage) zu gemeinsam genutztem Speicher.

<h3 id="fork-to-explore-alternatives">
  Forken Sie, um Alternativen zu erkunden
</h3>

Das Forken erstellt eine neue Sitzung, die mit einer Kopie des Verlaufs des Originals beginnt, aber von diesem Punkt an abweicht. Der Fork erhält seine eigene Sitzungs-ID; die ID und der Verlauf des Originals bleiben unverändert. Sie erhalten zwei unabhängige Sitzungen, die Sie separat fortsetzen können.

<Note>
  Das Forken verzweigt den Gesprächsverlauf, nicht das Dateisystem. Wenn ein gegabelter Agent Dateien bearbeitet, sind diese Änderungen real und für jede Sitzung sichtbar, die im gleichen Verzeichnis arbeitet. Um zu verzweigen und Dateiänderungen rückgängig zu machen, verwenden Sie [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing).
</Note>

Dieses Beispiel baut auf [Erfassen Sie die Sitzungs-ID](#capture-the-session-id) auf: Sie haben bereits ein Auth-Modul in `session_id` analysiert und möchten OAuth2 erkunden, ohne den JWT-fokussierten Thread zu verlieren. Der erste Block forkt die Sitzung und erfasst die ID des Forks (`forked_id`); der zweite Block setzt die ursprüngliche `session_id` fort, um den JWT-Pfad fortzusetzen. Sie haben jetzt zwei Sitzungs-IDs, die auf zwei separate Verläufe verweisen:

<CodeGroup>
  ```python Python theme={null}
  # Fork: branch from session_id into a new session
  forked_id = None
  async for message in query(
      prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
      options=ClaudeAgentOptions(
          resume=session_id,
          fork_session=True,
          max_turns=5,
      ),
  ):
      if isinstance(message, ResultMessage):
          forked_id = message.session_id  # The fork's ID, distinct from session_id
          if message.subtype == "success":
              print(message.result)

  print(f"Forked session: {forked_id}")

  # Original session is untouched; resuming it continues the JWT thread
  async for message in query(
      prompt="Continue with the JWT approach",
      options=ClaudeAgentOptions(resume=session_id),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  for await (const message of query({
    prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
    options: {
      resume: sessionId,
      forkSession: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      forkedId = message.session_id; // The fork's ID, distinct from sessionId
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  for await (const message of query({
    prompt: "Continue with the JWT approach",
    options: { resume: sessionId }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Sie sollten sehen, dass sich `forkedId` von der ursprünglichen Sitzungs-ID unterscheidet. Das Fortsetzen der ursprünglichen Sitzung setzt immer noch den JWT-Thread fort, was bestätigt, dass der Fork den ursprünglichen Verlauf nicht geändert hat.

<h2 id="resume-across-hosts">
  Fortsetzen über Hosts hinweg
</h2>

Sitzungsdateien sind lokal auf dem Computer, der sie erstellt hat. Um eine Sitzung auf einem anderen Host fortzusetzen (CI-Worker, kurzlebige Container, serverlos), haben Sie zwei Optionen:

* **Verschieben Sie die Sitzungsdatei.** Speichern Sie `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` aus dem ersten Durchlauf und stellen Sie es auf dem neuen Host auf dem gleichen Pfad wieder her, bevor Sie `resume` aufrufen. Das `cwd` muss übereinstimmen.
* **Verlassen Sie sich nicht auf Sitzungsfortsetzen.** Erfassen Sie die Ergebnisse, die Sie benötigen (Analyseergebnis, Entscheidungen, Datei-Diffs) als Anwendungszustand und übergeben Sie sie in den Prompt einer neuen Sitzung. Dies ist oft robuster als das Verschiffen von Transkriptdateien.

Beide SDKs stellen Funktionen zum Aufzählen von Sitzungen auf der Festplatte und zum Lesen ihrer Nachrichten bereit: [`listSessions()`](/docs/de/agent-sdk/typescript#listsessions) und [`getSessionMessages()`](/docs/de/agent-sdk/typescript#getsessionmessages) in TypeScript, [`list_sessions()`](/docs/de/agent-sdk/python#list_sessions) und [`get_session_messages()`](/docs/de/agent-sdk/python#get_session_messages) in Python. Verwenden Sie sie, um benutzerdefinierte Sitzungsauswähler, Bereinigungslogik oder Transkript-Viewer zu erstellen.

Beide SDKs stellen auch Funktionen zum Nachschlagen und Mutieren einzelner Sitzungen bereit: [`get_session_info()`](/docs/de/agent-sdk/python#get_session_info), [`rename_session()`](/docs/de/agent-sdk/python#rename_session) und [`tag_session()`](/docs/de/agent-sdk/python#tag_session) in Python, und [`getSessionInfo()`](/docs/de/agent-sdk/typescript#getsessioninfo), [`renameSession()`](/docs/de/agent-sdk/typescript#renamesession) und [`tagSession()`](/docs/de/agent-sdk/typescript#tagsession) in TypeScript. Verwenden Sie sie, um Sitzungen nach Tag zu organisieren oder ihnen menschenlesbare Titel zu geben.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Wie die Agent-Schleife funktioniert](/docs/de/agent-sdk/agent-loop): Verstehen Sie Durchläufe, Nachrichten und Kontextakkumulation innerhalb einer Sitzung
* [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing): Snapshot und Rückgängigmachen von Dateiänderungen, die der Agent innerhalb einer Sitzung vorgenommen hat
* [Python `ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions): Vollständige Sitzungsoptionsreferenz für Python
* [TypeScript `Options`](/docs/de/agent-sdk/typescript#options): Vollständige Sitzungsoptionsreferenz für TypeScript
