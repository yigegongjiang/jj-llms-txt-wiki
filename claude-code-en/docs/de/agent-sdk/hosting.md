> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hosting des Agent SDK

> Stellen Sie das Agent SDK in der Produktion bereit: Subprocess-Architektur, Sitzungspersistenz, Skalierung, Observability und Multi-Tenant-Isolation für Docker, Kubernetes und Sandbox-Provider.

Das Agent SDK spawnt und überwacht einen `claude` CLI-Subprocess, der eine Shell, ein Arbeitsverzeichnis und Sitzungsdateien auf der Festplatte besitzt. Das Hosting unterscheidet sich vom Hosting eines zustandslosen API-Wrappers. Jeder laufende Agent ist ein langlebiger Prozess, der an lokale Zustände gebunden ist, was beeinflusst, wie Sie Ressourcen zuordnen, Sitzungen persistieren und über Mandanten hinweg skalieren.

Diese Seite behandelt das Self-Hosting auf Ihrer eigenen Infrastruktur. Für bereitstellbare Dockerfiles und Kubernetes-Manifeste siehe das [Hosting-Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Wenn Sie die Agent-Schleife nicht auf Ihrer eigenen Infrastruktur ausführen müssen, erwägen Sie stattdessen [Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview). Anthropic hostet die Agent-Schleife, und Ihre Anwendung sendet Ereignisse und empfängt gestreamte Ergebnisse über die Client-SDKs oder die REST-API. Die Tool-Ausführung läuft in einer von Anthropic verwalteten Cloud-Sandbox oder einer [selbstgehosteten Sandbox](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) auf Ihrer eigenen Infrastruktur.

<h2 id="the-subprocess-model">
  Das Subprocess-Modell
</h2>

Jede Hosting-Entscheidung auf dieser Seite folgt aus der Art und Weise, wie das SDK den Agent ausführt. Wenn Ihr Code `query()` aufruft, spawnt das SDK einen separaten `claude` CLI-Prozess und kommuniziert mit ihm über stdio. Dieser Subprocess besitzt die Shell, das Arbeitsverzeichnis und die JSONL-Sitzungstranskripte auf der lokalen Festplatte.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" className="dark:hidden" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/hosting-subprocess-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=3fdeff3d7f44b2b67762668acfbb25f5" className="hidden dark:block" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess-dark.svg" />

Eine Agent-Sitzung wird einem Subprocess zugeordnet. Das Ausführen von N gleichzeitigen Sitzungen bedeutet N Subprozesse, jeder mit seinem eigenen Prozessbaum und seiner eigenen Transkriptdatei. Standardmäßig erben sie alle das Arbeitsverzeichnis Ihrer Anwendung. Wenn Sitzungen separate Dateisysteme benötigen, übergeben Sie ein unterschiedliches `cwd` in den Optionen des `query()`-Aufrufs jeder Sitzung:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the files in this directory",
    options: { cwd: "/work/session-a" },
  })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt="Summarize the files in this directory",
          options=ClaudeAgentOptions(cwd="/work/session-a"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Die TypeScript-Beispiele auf dieser Seite verwenden Top-Level-`await`, daher speichern Sie sie als `.mts`-Dateien oder setzen Sie `"type": "module"` in `package.json`.

<h3 id="state-that-lives-on-local-disk">
  Zustand, der auf der lokalen Festplatte lebt
</h3>

Drei Arten von Agent-Zustand leben standardmäßig im Dateisystem des Containers. Keiner von ihnen überlebt einen Container-Neustart, ein Scale-Down oder einen Wechsel zu einem anderen Knoten.

| Zustand                         | Standardort                                                                                             |
| ------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Sitzungstranskripte             | `~/.claude/projects/`, oder das Verzeichnis `projects/` unter `CLAUDE_CONFIG_DIR`, falls gesetzt        |
| `CLAUDE.md` Speicherdateien     | `~/.claude/CLAUDE.md` für die Benutzerebene und das Arbeitsverzeichnis der Sitzung für die Projektebene |
| Artefakte im Arbeitsverzeichnis | Das Arbeitsverzeichnis der Sitzung                                                                      |

Um Transkripte über Hosts hinweg zu persistieren, konfigurieren Sie einen [`SessionStore`-Adapter](/docs/de/agent-sdk/session-storage). Speicherdateien und andere Artefakte im Arbeitsverzeichnis benötigen ihre eigene Speicherstrategie, wie z. B. ein bereitgestelltes Volume oder eine Objektspeicher-Synchronisierung.

Informationen dazu, wie Sitzungen, Wiederaufnahme und Forking auf API-Ebene funktionieren, finden Sie unter [Sitzungen](/docs/de/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Wählen Sie ein Sitzungsmuster
</h2>

Diese vier Muster decken den Sitzungslebenszyklus ab: wie lange ein Container im Verhältnis zu den Sitzungen, die er bedient, existiert. Für den Ort, an dem der Container ausgeführt wird, hat das [Hosting-Kochbuch](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) [bereitstellbaren Code](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) für lokales Docker, Modal und Kubernetes. Wählen Sie hier ein Sitzungsmuster und ein Bereitstellungsziel aus dem Kochbuch.

<h3 id="ephemeral-sessions">
  Kurzlebige Sitzungen
</h3>

Erstellen Sie einen Container für jede Benutzeraufgabe und zerstören Sie ihn, wenn die Aufgabe abgeschlossen ist. Am besten für einmalige Aufgaben. Der Benutzer kann weiterhin mit der KI interagieren, während die Aufgabe abgeschlossen wird, aber nach Abschluss wird der Container zerstört.

Beispielworkloads umfassen Fehleruntersuchung und -behebung, Rechnungs- und Belegextraktion, Dokumentübersetzung und Medientransformation.

Der Container führt einen einmaligen Einstiegspunkt aus, der die Aufgabe aus der Umgebungsvariablen `TASK_PROMPT` liest, das SDK aufruft und beendet wird.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompt = process.env.TASK_PROMPT!;
  for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt=os.environ["TASK_PROMPT"],
          options=ClaudeAgentOptions(max_turns=20),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Das Skript gibt jede Nachricht aus, wenn sie ankommt, einschließlich einer Ergebnismeldung, deren `subtype` `success` ist, wenn die Aufgabe innerhalb des Turnus abgeschlossen wird. Wenn die Aufgabe stattdessen das 20-Turnus-Limit erreicht, ist der `subtype` der Ergebnismeldung `error_max_turns` und der `query()`-Aufruf löst einen Fehler aus, nachdem er ihn ausgegeben hat. Wickeln Sie daher die Schleife in einen Try-Block ein, wenn der Container sauber beendet werden muss. Siehe [Behandeln Sie das Ergebnis](/docs/de/agent-sdk/agent-loop#handle-the-result) für die Fehlersubtypen.

<h3 id="long-running-sessions">
  Langfristige Sitzungen
</h3>

Führen Sie persistente Container-Instanzen aus, die häufig mehrere SDK-Prozesse pro Container hosten, um laufende Arbeiten zu bedienen. Am besten für Agenten, die autonome Maßnahmen ergreifen, Inhalte bereitstellen oder hochvolumige Nachrichtenströme verarbeiten.

Beispielworkloads umfassen einen E-Mail-Agenten, der eingehende Post sortiert und beantwortet, einen Website-Builder, der eine pro Benutzer bearbeitbare Website über Container-Ports hostet, und einen Chatbot, der kontinuierlichen Datenverkehr von einer Plattform wie Slack verarbeitet.

Der Container stellt einen HTTP- oder WebSocket-Endpunkt bereit und ordnet jede aktive Sitzung einer langlebigen Abfrage und dem dahinter stehenden Unterprozess zu. In TypeScript verwenden Sie [`streamInput()`](/docs/de/agent-sdk/typescript#query-object), um Züge zu einer aktiven Sitzung hinzuzufügen, und [`startup()`](/docs/de/agent-sdk/typescript#startup), um Unterprozesse vor eingehendem Datenverkehr vorzuwärmen. In Python verwenden Sie [`ClaudeSDKClient`](/docs/de/agent-sdk/python#claudesdkclient), um eine Sitzung über Züge hinweg offen zu halten. Dimensionieren Sie den Container so, dass er die maximale Anzahl gleichzeitiger Sitzungen im Speicher halten kann.

<h3 id="hybrid-sessions">
  Hybrid-Sitzungen
</h3>

Kurzlebige Container, die beim Start aus einem [`SessionStore`](/docs/de/agent-sdk/session-storage) rehydriert werden und Updates zurück persistieren. Am besten für Sitzungen, die viele Interaktionen umfassen, aber zwischen ihnen untätig sind. Der Container wird während Leerlaufperioden heruntergefahren und wieder hochgefahren, wenn der Benutzer zurückkehrt.

Beispielworkloads umfassen einen persönlichen Projektmanager mit gelegentlichen Check-ins, tiefe Forschung, die über Stunden pausiert und fortgesetzt wird, und einen Kundenservice-Agenten, der die Tickethistorie über Interaktionen hinweg lädt.

Stimmen Sie das Leerlauf-Timeout Ihres Anbieters darauf ab, wie häufig Sie erwarten, dass Benutzer zurückkehren. Das Herunterfahren eines Containers ohne konfiguriertes `SessionStore` verliert das Transkript damit, daher ist der Store für dieses Muster erforderlich, nicht optional.

Das Muster hängt davon ab, eine Sitzung nach ID mit einem angehängten gemeinsamen Store fortzusetzen:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // an object store, key-value store, database, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, SessionStore
  import asyncio

  user_input: str = ...
  session_id: str = ...              # looked up from your database by user
  session_store: SessionStore = ...  # an object store, key-value store, database, or your own adapter


  async def main():
      async for message in query(
          prompt=user_input,
          options=ClaudeAgentOptions(
              resume=session_id,
              session_store=session_store,
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="multi-agent-container">
  Multi-Agent-Container
</h3>

Führen Sie mehrere SDK-Unterprozesse in einem Container aus. Am besten für Agenten, die eng zusammenarbeiten müssen, beispielsweise Multi-Agent-Simulationen, bei denen die Agenten in einer gemeinsamen Umgebung miteinander interagieren.

Geben Sie jedem Agenten sein eigenes Arbeitsverzeichnis, damit sie die Dateien des anderen nicht überschreiben, und isolieren Sie das Einstellungsladen, damit pro-Agent `CLAUDE.md`-Dateien nicht über Agenten hinweg durchsickern. Siehe [Multi-Tenant-Isolation](#multi-tenant-isolation) für die spezifischen Optionen.

<h2 id="provision-the-container">
  Container bereitstellen
</h2>

<h3 id="container-based-sandboxing">
  Container-basiertes Sandboxing
</h3>

Führen Sie das SDK in einem Sandbox-Container aus, um Prozessisolation, Ressourcenlimits, Netzwerkkontrolle und ein kurzlebiges Dateisystem zu erreichen.

Fragen, die Sie bei der Wahl eines Anbieters beantworten sollten:

* **Wer betreibt die Sandbox**: Ein Sandbox-as-a-Service-Anbieter betreibt die Infrastruktur für Sie, während Self-Hosted-Optionen Ihnen Software zum Ausführen auf Ihren eigenen Systemen bieten.
* **Cold-Start-Latenz**: wie lange es dauert, von „Sandbox erstellen" bis „bereit, die erste Anfrage zu akzeptieren". Kurzlebige Muster benötigen Sub-Sekunden-Starts. Langfristige Muster tolerieren mehr.
* **Persistenter Speicher**: ob der Anbieter dauerhafte Volumes oder nur kurzlebige Festplatte anbietet. Das Hybrid-Muster benötigt irgendwo dauerhaften Speicher, entweder in der Sandbox oder daneben.
* **Preismodell**: Pro-Sekunde, Pro-Anfrage oder pauschale stündliche Abrechnung. Pro-Sekunde-Preisgestaltung eignet sich für bursty kurzlebige Workloads. Stündlich eignet sich für langfristige Sitzungen.
* **Netzwerk**: Unterstützung für benutzerdefinierte Egress-Regeln, ausgehende Proxys und privates VPC-Peering für regulierte Umgebungen.

Für Self-Hosted-Optionen wie Docker, gVisor und Firecracker sowie detaillierte Isolationskonfiguration siehe [Isolationstechnologien](/docs/de/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Laufzeit-Abhängigkeiten
</h3>

Der Container benötigt die Sprachlaufzeit Ihres SDK:

* Python 3.10+ für das Python SDK oder Node.js 18+ für das TypeScript SDK
* Sowohl das TypeScript als auch das Python SDK bündeln eine native Claude Code-Binärdatei für die meisten Installationen, und die erzeugte CLI benötigt keine separate Node.js-Installation. Siehe die [Installationsnotiz des Schnellstarts](/docs/de/agent-sdk/quickstart) für die Installationen, die eine separate native Claude Code-Installation benötigen.

Die gebündelte Binärdatei ist an die SDK-Paketversion gebunden, daher ist das Aktualisieren des SDK die Möglichkeit, die CLI zu aktualisieren. Das SDK folgt semver: Nehmen Sie Patch-Releases kontinuierlich an und überprüfen Sie das [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)- oder [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)-Changelog, bevor Sie ein Minor-Release annehmen.

<h3 id="resources">
  Ressourcen
</h3>

1 GiB RAM, 5 GiB Festplatte und 1 CPU pro Agent ist ein angemessener Ausgangspunkt für eine neu gestartete Instanz. Die Speichernutzung wächst mit der Sitzungslänge und der Tool-Aktivität, daher sollten Sie für die Sitzungslängen und Parallelität dimensionieren, die Sie tatsächlich benötigen, anstatt für die untätige Baseline. Siehe [Skalierung und Parallelität](#scaling-and-concurrency), um zu erfahren, wie Sie Agents pro Host berechnen.

<h3 id="network">
  Netzwerk
</h3>

Das SDK benötigt ausgehende HTTPS zu `api.anthropic.com` oder zu Ihrem regionalen Endpunkt des Anbieters, wenn es auf Amazon Bedrock oder Google Cloud's Agent Platform ausgeführt wird. Wenn Ihre Agents [MCP-Server](/docs/de/agent-sdk/mcp) oder externe Tools verwenden, benötigen sie auch ausgehenden Zugriff auf diese Endpunkte. Für die Produktion leiten Sie ausgehenden Datenverkehr durch einen Egress-Proxy weiter, der Domain-Allowlists erzwingt, Anmeldeinformationen injiziert und Anfragen protokolliert. Siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment) für das vollständige Muster.

Für eingehenden Datenverkehr stellen Sie einen HTTP- oder WebSocket-Port auf dem Container bereit. Ihre Anwendung verarbeitet Client-Anfragen auf diesem Port und ruft das SDK intern auf; der Unterprozess selbst lauscht nicht im Netzwerk.

<h2 id="handle-production-concerns">
  Produktionsbedenken behandeln
</h2>

Arbeiten Sie diese Entscheidungen durch, bevor Sie einen selbstgehosteten Agent bereitstellen.

<h3 id="session-and-state-persistence">
  Sitzungs- und Zustandspersistenz
</h3>

Der Standard-lokale Datenträger geht bei Neustart, Herunterskalierung oder Verschiebung auf einen anderen Knoten verloren. Für jede Sitzung, die ein Benutzer fortsetzen möchte, spiegeln Sie das Transkript mit einem [`SessionStore`-Adapter](/docs/de/agent-sdk/session-storage) zu dauerhaftem Speicher. Siehe [Referenzimplementierungen](/docs/de/agent-sdk/session-storage#reference-implementations) für Beispieladapter für einen Objektspeicher, einen Schlüssel-Wert-Speicher und eine Datenbank sowie eine Konformitätssuite für Ihre eigenen.

Drei Dinge, die Sie über das Verhalten von `SessionStore` wissen sollten:

* **Nur Transkripte**: `SessionStore` spiegelt Transkripte, nicht `CLAUDE.md`-Speicherdateien oder andere Artefakte im Arbeitsverzeichnis. Mounten Sie ein gemeinsames Volume oder synchronisieren Sie diese separat.
* **Spiegelung, keine Ersetzung**: Der Unterprozess schreibt zuerst auf die lokale Festplatte, und das SDK leitet eine Kopie jedes Batches an den Speicher weiter. Das lokale Transkript einer neuen Sitzung überlebt den Lauf; ein aus dem Speicher fortgesetzter Lauf löscht seine lokale Kopie am Ende, sodass der Speicher die einzige dauerhafte Kopie enthält. Siehe [Dual-Write-Architektur](/docs/de/agent-sdk/session-storage#dual-write-architecture).
* **`mirror_error`-Meldungen**: Wenn das SDK einen Batch nicht an den Speicher liefern kann, verwirft es den Batch, gibt eine `{ type: "system", subtype: "mirror_error" }`-Meldung aus und setzt die Abfrage fort. Warnen Sie vor diesen, wenn die Speicherdauerhaftigkeit wichtig ist. Siehe [Spiegelschreibvorgänge sind Best-Effort](/docs/de/agent-sdk/session-storage#mirror-writes-are-best-effort) für das Wiederholungs- und Timeout-Verhalten.

<h3 id="observability">
  Observability
</h3>

Agent SDK-Agenten sind langlebige Prozesse, die Werkzeugaufrufe über viele API-Roundtrips hinweg erzeugen. Ohne Telemetrie können Sie nicht sehen, welche Werkzeuge ausgeführt wurden, wie lange sie dauerten oder wo eine Sitzung stecken blieb.

Das SDK erbt die OpenTelemetry-Konfiguration aus der Umgebung. Legen Sie die OTEL-Umgebungsvariablen auf Container- oder Orchestrator-Ebene fest, damit jeder `query()`-Aufruf Spans, Metriken und Log-Ereignisse an Ihren Collector exportiert. Das folgende Beispiel aktiviert OTLP-Export für alle drei Signale. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` ist nur für Traces erforderlich; lassen Sie es weg, wenn Sie nur Metriken und Logs exportieren.

```bash title=".env" theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

Eingabetext und Werkzeugeingaben sind standardmäßig nicht in Exporten enthalten. Siehe [Sensible Daten in Exporten steuern](/docs/de/agent-sdk/observability#control-sensitive-data-in-exports) für die Opt-in-Flags und [Observability](/docs/de/agent-sdk/observability) für den vollständigen Signalkatalog.

<h3 id="auth-and-secrets">
  Authentifizierung und Geheimnisse
</h3>

Drei Authentifizierungsbedenken sind zum Zeitpunkt des Hostings wichtig:

* **Anthropic API**: Der Unterprozess liest `ANTHROPIC_API_KEY` aus seiner Umgebung. Stellen Sie es von Ihrem Secret Manager bereit, oder setzen Sie `ANTHROPIC_BASE_URL`, um Modellaufrufe durch einen Proxy zu leiten, der den Schlüssel außerhalb des Containers injiziert. Siehe [Credential Management](/docs/de/agent-sdk/secure-deployment#credential-management) für das Proxy-Muster und [Setup im SDK-Schnellstart](/docs/de/agent-sdk/quickstart#setup) für unterstützte Authentifizierungsmethoden.
* **Eingehend**: Platzieren Sie die Authentifizierung an einem Gateway vor dem Agent-Container. Der Agent sollte vorauthentifizierte Anfragen erhalten und sollte nicht die Komponente sein, die Benutzer-Token validiert.
* **Ausgehende Werkzeuge**: Halten Sie Werkzeugzugangsanmeldedaten aus der Agent-Umgebung. Leiten Sie ausgehende Aufrufe durch einen Proxy, der API-Schlüssel injiziert, nachdem die Anfrage den Container verlässt. Der Agent führt den Aufruf durch; der Proxy fügt die Anmeldedaten hinzu.

<h3 id="scaling-and-concurrency">
  Skalierung und Parallelität
</h3>

Jede Sitzung läuft in ihrem eigenen Unterprozess, daher ist die Parallelität auf einem Host durch die Anzahl der Unterprozesse begrenzt, die sein RAM halten kann.

Dimensionieren Sie jeden Host mit dieser Formel:

```text theme={null}
agents per host = (host RAM - overhead) / (per-session RAM ceiling)
```

Messen Sie die Pro-Sitzungs-Obergrenze, indem Sie eine repräsentative Sitzung bis zu Ihrer Zieldauer unter Ihrer erwarteten Werkzeuglast ausführen und den Peak-RSS aufzeichnen. Der 1-GiB-Startpunkt in [Ressourcen](#resources) ist ein Minimum, nicht die Obergrenze.

Das horizontale Skalierungs-Routing hängt von Ihrem Muster ab. Bei langlebigen Sitzungen, bei denen Container viele Sitzungen halten, führen Sie einen Pool von Containern hinter einem Load Balancer aus und heften Sie jede Sitzung mit konsistentem Hashing auf `sessionId` an einen Container. Eine angeheftete Sitzung trifft immer wieder auf denselben Container und daher auf denselben laufenden Unterprozess, bis er entfernt oder der Container neu gestartet wird.

<h3 id="cost">
  Kosten
</h3>

Die Anthropic-Token-Kosten dominieren typischerweise die Container-Infrastrukturkosten um eine Größenordnung oder mehr. Ein minimal bereitgestellter Container läuft ungefähr \$0,05 pro Stunde, während eine einzelne lange Agent-Sitzung Dollar in Token ausgeben kann. Siehe [Cost Tracking](/docs/de/agent-sdk/cost-tracking) für die Token-Abrechnung pro Sitzung.

<h3 id="multi-tenant-isolation">
  Multi-Tenant-Isolation
</h3>

Das Standard-SDK-Verhalten liest Einstellungen und `CLAUDE.md`-Speicherdateien aus dem Dateisystem. In einem gemeinsamen Container, der mehrere Mandanten bedient, können diese Dateien den Kontext eines Mandanten in die Sitzung eines anderen Mandanten durchsickern lassen.

Um Mandanten in einem gemeinsamen Container zu isolieren:

* Übergeben Sie `settingSources: []` in TypeScript oder `setting_sources=[]` in Python, um Benutzer-, Projekt- und lokale Einstellungen zu überspringen.
* Setzen Sie `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`. [Auto Memory](/docs/de/memory#auto-memory) bei `~/.claude/projects/<project>/memory/` wird unabhängig von `settingSources` in den System-Prompt geladen. Siehe [Was settingSources nicht steuert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) für die anderen Eingaben, die bedingungslos geladen werden.
* Zeigen Sie `CLAUDE_CONFIG_DIR` auf ein mandantenspezifisches Verzeichnis, damit Mandanten die globale Konfiguration `~/.claude.json` nicht teilen. Wenn jedes Konfigurationsverzeichnis ein Arbeitsverzeichnis bedient und Sie keinen [`SessionStore`](/docs/de/agent-sdk/session-storage) über Mandanten hinweg teilen, können Sie auch [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/de/sessions#name-the-project-directory-yourself) in `env` setzen, um die Transkriptpfade darunter kurz zu halten. Erfordert TypeScript Agent SDK v0.3.234 oder später oder Python Agent SDK v0.2.140 oder später.
* Verwenden Sie ein mandantenspezifisches Arbeitsverzeichnis. Übergeben Sie `cwd` explizit bei jedem `query()`-Aufruf.
* Wenden Sie mandantenspezifische Egress-Regeln bei Ihrem Proxy an, wie z. B. unterschiedliche ausgehende IPs, Anmeldedaten oder Domain-Allowlists, damit ein kompromittierter Mandant keine Daten über die ausgehende Richtlinie eines anderen Mandanten exfiltrieren kann.

Das folgende Beispiel wendet die Einstellungen-, Auto-Memory-, Konfigurationsverzeichnis- und Arbeitsverzeichnisoptionen zusammen an. Konstruieren Sie `tenantDir` und `configDir` so, dass jeder Mandant einen Pfad erhält, den kein anderer Mandant lesen kann. In TypeScript ersetzt `env` die Unterprozessumgebung, daher verteilen Sie `...process.env`, um geerbte Variablen wie `PATH` und `ANTHROPIC_API_KEY` zu behalten. In Python wird `env` auf die geerbte Umgebung zusammengeführt.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio

  prompt: str = ...
  tenant_dir: str = ...
  config_dir: str = ...


  async def main():
      async for message in query(
          prompt=prompt,
          options=ClaudeAgentOptions(
              cwd=tenant_dir,
              setting_sources=[],
              env={
                  "CLAUDE_CONFIG_DIR": config_dir,
                  "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="known-limitations">
  Bekannte Einschränkungen
</h2>

Berücksichtigen Sie diese in Ihrem Bereitstellungsdesign.

| Einschränkung                                               | Maßnahme                                                                                                                                                                                                                                                                      |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Kein Sitzungs-Timeout auf oberster Ebene                    | Eine Sitzung läuft nicht automatisch ab. Setzen Sie `maxTurns` in TypeScript oder `max_turns` in Python, um zu begrenzen, wie viele Tool-Use-Rundläufe der Agent durchführt, bevor er stoppt.                                                                                 |
| Speicherwachstum über lange Sitzungen                       | Begrenzen Sie die Sitzungslänge oder recyceln Sie Subprozesse regelmäßig. Siehe [Skalierung und Parallelität](#scaling-and-concurrency).                                                                                                                                      |
| Große parallele Subagent-Fanouts können Ratenlimits treffen | Teilen Sie die Arbeit in kleinere Batches auf, anstatt eine breite Verteilung auszugeben.                                                                                                                                                                                     |
| Keine Wanduhr-Frist pro Subagent                            | Begrenzen Sie jeden [Subagent](/docs/de/agent-sdk/subagents) mit `maxTurns` in seiner `AgentDefinition`. `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` setzt einen Stall-Watchdog, der ausgelöst wird, wenn ein Subagent keine Ausgabe mehr produziert; es ist keine Gesamtlaufzeit-Frist. |

<h2 id="troubleshoot-deployment-failures">
  Bereitstellungsfehler beheben
</h2>

Verwenden Sie diesen Abschnitt, wenn ein Agent, der auf Ihrem Computer funktioniert, in einem bereitgestellten Dienst fehlschlägt. Jedes Element unten benennt einen Fehler und verlinkt den Eintrag, der ihn behandelt:

* **CLI nicht gefunden beim Dienstart**: In Python führt ein Container oder Service-Manager Ihre Anwendung mit einem anderen `PATH` aus als Ihre Shell, daher ist eine lokal funktionierende Installation für den Prozess nicht sichtbar. In TypeScript hat der Image-Build die optionalen Abhängigkeiten des SDK übersprungen, oder `pathToClaudeCodeExecutable` verweist auf eine Datei, die im Image nicht vorhanden ist. Siehe [Claude Code nicht gefunden](/docs/de/agent-sdk/troubleshooting#clinotfounderror-claude-code-not-found).
* **CLI im Image vorhanden, wird aber nicht gestartet**: Claude Code kann nicht von einer Binärdatei gestartet werden, die nicht der Architektur oder libc des Containers entspricht, oder von einer Datei, die während des Image-Builds ihre Ausführungsberechtigung verloren hat. Siehe [Claude Code konnte nicht gestartet werden](/docs/de/agent-sdk/troubleshooting#cliconnectionerror-failed-to-start-claude-code).
* **Claude Code-Prozess beendet sich während der Ausführung**: Der Fehler, den Ihre Anwendung erhält, hängt von der SDK-Sprache und davon ab, ob die CLI zuerst ein Fehlerergebnis gemeldet hat. Die Einträge unter [CLI-Prozessbeendigung](/docs/de/agent-sdk/troubleshooting#cli-process-exit) behandeln jede Nachricht.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Hosting-Cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): Notebook-Anleitung mit [bereitstellbarem Code](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) für Docker, Modal und Kubernetes.
* [Sitzungsspeicher](/docs/de/agent-sdk/session-storage): Persistieren Sie Transkripte über Hosts hinweg mit einem `SessionStore`-Adapter.
* [Observability](/docs/de/agent-sdk/observability): Exportieren Sie OTEL-Traces, Metriken und Protokolle zu Ihrem Collector.
* [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment): Netzwerkkontrollen, Verwaltung von Anmeldedaten und Isolationshärtung.
* [Kostenverfolgung](/docs/de/agent-sdk/cost-tracking): Token- und Kostenabrechnung pro Sitzung.
