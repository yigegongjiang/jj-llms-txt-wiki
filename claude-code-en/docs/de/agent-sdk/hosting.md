> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hosting des Agent SDK

> Bereitstellung des Agent SDK in der Produktion: Subprocess-Architektur, Sitzungspersistenz, Skalierung, Observability und Multi-Tenant-Isolation für Docker, Kubernetes und Sandbox-Provider.

Das Agent SDK spawnt und überwacht einen `claude` CLI-Subprocess, der eine Shell, ein Arbeitsverzeichnis und Sitzungsdateien auf der Festplatte besitzt. Das Hosting unterscheidet sich vom Hosting eines zustandslosen API-Wrappers. Jeder laufende Agent ist ein langlebiger Prozess, der an lokalen Zustand gebunden ist, was beeinflusst, wie Sie Ressourcen zuordnen, Sitzungen persistieren und über Mandanten skalieren.

Diese Seite behandelt das Self-Hosting auf Ihrer eigenen Infrastruktur: verstehen Sie [das Subprocess-Modell](#the-subprocess-model), [wählen Sie ein Sitzungsmuster](#choose-a-session-pattern), [stellen Sie den Container bereit](#provision-the-container) und [behandeln Sie Produktionsbedenken](#handle-production-concerns) wie Persistenz, Observability, Authentifizierung und Multi-Tenant-Isolation. Für bereitstellbare Dockerfiles und Kubernetes-Manifeste siehe das [Hosting-Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Wenn Sie keine Infrastrukturkontrolle, benutzerdefinierte Isolation oder Ihre eigene Datenebene benötigen, erwägen Sie stattdessen [Managed Agents](https://platform.claude.com/docs/de/managed-agents/overview): eine gehostete REST-API, bei der Anthropic den Agent und die Sandbox ausführt, sodass Ihre Anwendung Ereignisse sendet und Ergebnisse zurückstreamt, ohne dass Sie eine Hosting-Infrastruktur betreiben müssen.

<Info>
  Für Sicherheitshärtung über grundlegendes Sandboxing hinaus, einschließlich Netzwerkkontrollen, Verwaltung von Anmeldedaten und Isolationsoptionen, siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment).
</Info>

<h2 id="the-subprocess-model">
  Das Subprocess-Modell
</h2>

Jede Hosting-Entscheidung auf dieser Seite folgt aus der Art und Weise, wie das SDK den Agent ausführt. Wenn Ihr Code `query()` aufruft, spawnt das SDK einen separaten `claude` CLI-Prozess und kommuniziert mit ihm über stdio. Dieser Subprocess besitzt die Shell, das Arbeitsverzeichnis und die JSONL-Sitzungstranskripte auf der lokalen Festplatte.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Request-Fluss: Client zu Ihrer App, die einen claude CLI-Subprocess über stdio im Container spawnt; der Subprocess schreibt auf die lokale Festplatte und ruft api.anthropic.com über HTTPS auf" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

Eine Agent-Sitzung wird einer Subprocess zugeordnet. Das Ausführen von N gleichzeitigen Sitzungen bedeutet N Subprozesse, jeder mit seinem eigenen Prozessbaum und einer Transkriptdatei. Standardmäßig erben sie alle das Arbeitsverzeichnis Ihrer Anwendung. Übergeben Sie daher `cwd` bei jedem `query()`-Aufruf, wenn Sitzungen separate Dateisysteme benötigen:

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  Status, der auf der lokalen Festplatte gespeichert ist
</h3>

Drei Arten von Agent-Status befinden sich standardmäßig im Dateisystem des Containers. Keine von ihnen überlebt einen Container-Neustart, ein Scale-Down oder einen Wechsel zu einem anderen Knoten.

| Status                       | Standardort                                                                                             |
| ---------------------------- | ------------------------------------------------------------------------------------------------------- |
| Sitzungstranskripte          | `~/.claude/projects/`, oder das Verzeichnis `projects/` unter `CLAUDE_CONFIG_DIR`, falls gesetzt        |
| `CLAUDE.md` Speicherdateien  | `~/.claude/CLAUDE.md` für die Benutzerebene und das Arbeitsverzeichnis der Sitzung für die Projektebene |
| Arbeitsverzeichnis-Artefakte | Das Arbeitsverzeichnis der Sitzung                                                                      |

Um Transkripte über Hosts hinweg zu persistieren, konfigurieren Sie einen [`SessionStore`-Adapter](/docs/de/agent-sdk/session-storage). Speicherdateien und andere Arbeitsverzeichnis-Artefakte benötigen ihre eigene Speicherstrategie, wie z. B. ein bereitgestelltes Volume oder eine Objektspeicher-Synchronisierung.

Informationen dazu, wie Sitzungen, Wiederaufnahme und Forking auf API-Ebene funktionieren, finden Sie unter [Sitzungen](/docs/de/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Wählen Sie ein Sitzungsmuster
</h2>

Diese vier Muster decken den Sitzungslebenszyklus ab: wie lange ein Container im Verhältnis zu den Sitzungen, die er bedient, existiert. Für den Ort, an dem der Container ausgeführt wird, hat das [Hosting-Kochbuch](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) [bereitstellbaren Code](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) für lokales Docker, Modal und Kubernetes. Wählen Sie hier ein Sitzungsmuster und ein Bereitstellungsziel aus dem Kochbuch.

<h3 id="ephemeral-sessions">
  Ephemere Sitzungen
</h3>

Erstellen Sie einen Container für jede Benutzeraufgabe und zerstören Sie ihn, wenn die Aufgabe abgeschlossen ist. Am besten für einmalige Aufgaben. Der Benutzer kann möglicherweise noch mit der KI interagieren, während die Aufgabe abgeschlossen wird, aber nach Abschluss wird der Container zerstört.

Beispielworkloads umfassen Fehleruntersuchung und -behebung, Rechnungs- und Belegextraktion, Dokumentenübersetzung und Medientransformation.

Der Container führt einen einmaligen Einstiegspunkt aus, der das SDK aufruft und beendet wird. Das folgende Beispiel zeigt eine minimale TypeScript-Version. Speichern Sie es als `entrypoint.mts` oder setzen Sie `"type": "module"` in `package.json`, damit `await` auf oberster Ebene verfügbar ist.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  Langfristige Sitzungen
</h3>

Führen Sie persistente Container-Instanzen aus, die häufig mehrere SDK-Prozesse pro Container hosten, um laufende Arbeiten zu bedienen. Am besten für Agenten, die autonome Maßnahmen ergreifen, Inhalte bereitstellen oder hochvolumige Nachrichtenströme verarbeiten.

Beispielworkloads umfassen einen E-Mail-Agenten, der eingehende Post sortiert und beantwortet, einen Website-Builder, der eine pro Benutzer bearbeitbare Website über Container-Ports hostet, und einen Chatbot, der kontinuierlichen Datenverkehr von einer Plattform wie Slack verarbeitet.

Der Container stellt einen HTTP- oder WebSocket-Endpunkt bereit und ordnet jede aktive Sitzung einer langlebigen Abfrage und dem dahinter liegenden Unterprozess zu. In TypeScript verwenden Sie [`streamInput()`](/docs/de/agent-sdk/typescript#query-object), um Züge zu einer aktiven Sitzung hinzuzufügen, und [`startup()`](/docs/de/agent-sdk/typescript#startup), um Unterprozesse vor eingehendem Datenverkehr vorzuwärmen. In Python verwenden Sie [`ClaudeSDKClient`](/docs/de/agent-sdk/python#claudesdkclient), um eine Sitzung über Züge hinweg offen zu halten. Dimensionieren Sie den Container so, dass er die maximale Anzahl gleichzeitiger Sitzungen im Speicher halten kann.

<h3 id="hybrid-sessions">
  Hybrid-Sitzungen
</h3>

Ephemere Container, die beim Start aus einem [`SessionStore`](/docs/de/agent-sdk/session-storage) rehydriert werden und Updates zurück persistieren. Am besten für Sitzungen, die viele Interaktionen umfassen, aber zwischen ihnen untätig sind. Der Container wird während Leerlaufperioden heruntergefahren und wieder hochgefahren, wenn der Benutzer zurückkehrt.

Beispielworkloads umfassen einen persönlichen Projektmanager mit gelegentlichen Check-ins, tiefe Recherche, die über Stunden pausiert und fortgesetzt wird, und einen Kundenservice-Agenten, der die Tickethistorie über Interaktionen hinweg lädt.

Stimmen Sie das Leerlauf-Timeout Ihres Anbieters darauf ab, wie häufig Sie erwarten, dass Benutzer zurückkehren. Das Herunterfahren eines Containers ohne konfiguriertes `SessionStore` verliert das Transkript damit, daher ist der Store für dieses Muster erforderlich, nicht optional.

Das Muster basiert auf der Wiederaufnahme einer Sitzung nach ID mit einem angehängten gemeinsamen Store:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // S3, Redis, Postgres, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=user_input,
      options=ClaudeAgentOptions(
          resume=session_id,            # looked up from your database by user
          session_store=session_store,  # S3, Redis, Postgres, or your own adapter
      ),
  ):
      ...
  ```
</CodeGroup>

Siehe [Sitzungsspeicher](/docs/de/agent-sdk/session-storage) für die vollständige `SessionStore`-Schnittstelle und Referenzadapter.

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

Führen Sie das SDK in einem sandboxierten Container aus, um Prozessisolation, Ressourcenlimits, Netzwerkkontrolle und ein kurzlebiges Dateisystem zu erreichen. Mehrere Anbieter spezialisieren sich auf sandboxierte Container-Umgebungen, die zum Modell des Agent SDK passen.

Fragen, die Sie bei der Wahl eines Anbieters beantworten sollten:

* **Wer betreibt die Sandbox**: Ein Sandbox-as-a-Service-Anbieter betreibt die Infrastruktur für Sie, während selbst gehostete Optionen Ihnen Software zur Verfügung stellen, die Sie auf Ihren eigenen Systemen ausführen können.
* **Cold-Start-Latenz**: Wie lange dauert es von „Sandbox erstellen" bis „bereit, die erste Anfrage zu akzeptieren". Kurzlebige Muster benötigen Sub-Sekunden-Starts. Langfristige Muster tolerieren mehr.
* **Persistenter Speicher**: Ob der Anbieter dauerhafte Volumes oder nur kurzlebige Festplatte anbietet. Das Hybrid-Muster benötigt dauerhaften Speicher irgendwo, ob in der Sandbox oder daneben.
* **Preismodell**: Pro-Sekunde, Pro-Anfrage oder pauschale stündliche Abrechnung. Pro-Sekunde-Preisgestaltung eignet sich für bursty kurzlebige Workloads. Stündlich eignet sich für langfristige Sitzungen.
* **Netzwerk**: Unterstützung für benutzerdefinierte Egress-Regeln, ausgehende Proxys und privates VPC-Peering für regulierte Umgebungen.

Anbieter zur Evaluierung:

* [Modal Sandbox](https://modal.com/docs/guide/sandbox), mit einer [Demo-Implementierung](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

Für selbst gehostete Optionen wie Docker, gVisor und Firecracker sowie detaillierte Isolationskonfiguration siehe [Isolationstechnologien](/docs/de/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Laufzeit-Abhängigkeiten
</h3>

Der Container benötigt nur die Laufzeit Ihres SDK:

* Python 3.10+ für das Python SDK oder Node.js 18+ für das TypeScript SDK
* Beide SDK-Pakete enthalten eine native Claude Code-Binärdatei für die Host-Plattform, daher ist keine separate Claude Code- oder Node.js-Installation für die erzeugte CLI erforderlich

Die gebündelte Binärdatei ist an die SDK-Paketversion gebunden, daher ist das Aktualisieren des SDK die Methode zum Aktualisieren der CLI. Das SDK folgt semver: Nehmen Sie Patch-Releases kontinuierlich an und überprüfen Sie das [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)- oder [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)-Changelog, bevor Sie ein Minor-Release annehmen.

<h3 id="resources">
  Ressourcen
</h3>

1 GiB RAM, 5 GiB Festplatte und 1 CPU pro Agent ist ein angemessener Ausgangspunkt für eine neu gestartete Instanz. Der Speicherverbrauch wächst mit der Sitzungsdauer und der Tool-Aktivität, daher sollten Sie für die Sitzungslängen und Parallelität dimensionieren, die Sie tatsächlich benötigen, anstatt für die untätige Baseline. Siehe [Skalierung und Parallelität](#scaling-and-concurrency), um zu erfahren, wie Sie Agents pro Host berechnen.

<h3 id="network">
  Netzwerk
</h3>

Das SDK benötigt ausgehende HTTPS zu `api.anthropic.com` oder zu Ihrem regionalen Endpunkt des Anbieters, wenn Sie auf Amazon Bedrock oder Google Cloud's Agent Platform ausführen. Wenn Ihre Agents [MCP-Server](/docs/de/agent-sdk/mcp) oder externe Tools verwenden, benötigen sie auch ausgehenden Zugriff auf diese Endpunkte. Für die Produktion leiten Sie ausgehenden Datenverkehr durch einen Egress-Proxy, der Domain-Allowlists durchsetzt, Anmeldeinformationen injiziert und Anfragen protokolliert. Siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment) für das vollständige Muster.

Für eingehenden Datenverkehr stellen Sie einen HTTP- oder WebSocket-Port auf dem Container bereit. Ihre Anwendung verarbeitet Client-Anfragen auf diesem Port und ruft das SDK intern auf; der Unterprozess selbst lauscht nicht im Netzwerk.

<h2 id="handle-production-concerns">
  Produktionsbedenken behandeln
</h2>

Arbeiten Sie diese Entscheidungen durch, bevor Sie einen selbstgehosteten Agenten bereitstellen.

<h3 id="session-and-state-persistence">
  Sitzungs- und Zustandspersistenz
</h3>

Der standardmäßige lokale Datenträger geht bei Neustart, Herunterskalierung oder Verschiebung auf einen anderen Knoten verloren. Für jede Sitzung, die ein Benutzer fortsetzen möchte, spiegeln Sie das Transkript mit einem [`SessionStore`-Adapter](/docs/de/agent-sdk/session-storage) auf dauerhaften Speicher. Siehe [Referenzimplementierungen](/docs/de/agent-sdk/session-storage#reference-implementations) für S3-, Redis- und Postgres-Adapter sowie eine Konformitätssuite für Ihre eigenen.

Drei Dinge, die Sie über das Verhalten von `SessionStore` wissen sollten:

* **Nur Transkripte**: `SessionStore` spiegelt Transkripte, nicht `CLAUDE.md`-Speicherdateien oder andere Arbeitsverzeichnis-Artefakte. Mounten Sie ein gemeinsames Volume oder synchronisieren Sie diese separat.
* **Spiegelung, keine Ersetzung**: Der Unterprozess schreibt zuerst auf die lokale Festplatte, und der Store erhält eine Kopie jedes Batches. Lokale Schreibvorgänge bleiben maßgeblich.
* **`mirror_error`-Meldungen**: Ein Batch, den der Store ablehnt, wird insgesamt bis zu dreimal gesendet, mit einer kurzen Backoff-Zeit vor jedem Wiederholungsversuch; ein Aufruf mit Zeitüberschreitung wird nicht wiederholt. Wenn der Batch immer noch fehlschlägt, verwirft das SDK ihn, gibt eine `{ type: "system", subtype: "mirror_error" }`-Meldung aus und setzt die Abfrage fort. Warnen Sie vor diesen, wenn die Store-Dauerhaftigkeit wichtig ist.

<h3 id="observability">
  Observability
</h3>

Agent SDK-Agenten sind langlebige Prozesse, die Werkzeugaufrufe über viele API-Roundtrips hinweg spawnen. Ohne Telemetrie können Sie nicht sehen, welche Werkzeuge ausgeführt wurden, wie lange sie dauerten oder wo eine Sitzung stecken blieb.

Das SDK erbt die OpenTelemetry-Konfiguration aus der Umgebung. Legen Sie die OTEL-Umgebungsvariablen auf Container- oder Orchestrator-Ebene fest, damit jeder `query()`-Aufruf Spans, Metriken und Log-Ereignisse an Ihren Collector exportiert. Das folgende Beispiel aktiviert OTLP-Export für alle drei Signale. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` ist nur für Traces erforderlich; lassen Sie es weg, wenn Sie nur Metriken und Logs exportieren.

```bash title=".env' theme={null}
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

* **Anthropic API**: Der Unterprozess liest `ANTHROPIC_API_KEY` aus seiner Umgebung. Stellen Sie es von Ihrem Secret Manager bereit, oder setzen Sie `ANTHROPIC_BASE_URL`, um Modellaufrufe durch einen Proxy zu leiten, der den Schlüssel außerhalb des Containers injiziert. Siehe [Credential Management](/docs/de/agent-sdk/secure-deployment#credential-management) für das Proxy-Muster und [SDK-Übersicht](/docs/de/agent-sdk/overview#get-started) für unterstützte Authentifizierungsmethoden.
* **Eingehend**: Setzen Sie Authentifizierung an einem Gateway vor dem Agent-Container. Der Agent sollte vorauthentifizierte Anfragen erhalten und sollte nicht die Komponente sein, die Benutzer-Token validiert.
* **Ausgehende Werkzeuge**: Halten Sie Werkzeug-Anmeldedaten aus der Agent-Umgebung. Leiten Sie ausgehende Aufrufe durch einen Proxy, der API-Schlüssel injiziert, nachdem die Anfrage den Container verlässt. Der Agent tätigt den Aufruf; der Proxy fügt die Anmeldedaten hinzu.

<h3 id="scaling-and-concurrency">
  Skalierung und Parallelität
</h3>

Jede Sitzung läuft in ihrem eigenen Unterprozess, daher ist die Parallelität auf einem Host durch die Anzahl der Unterprozesse begrenzt, die sein RAM halten kann.

Dimensionieren Sie jeden Host mit dieser Formel:

```text theme={null}
Agenten pro Host = (Host-RAM - Overhead) / (RAM-Obergrenze pro Sitzung)
```

Messen Sie die RAM-Obergrenze pro Sitzung, indem Sie eine repräsentative Sitzung bis zu Ihrer Zieldauer unter Ihrer erwarteten Werkzeuglast ausführen und den Peak-RSS aufzeichnen. Der 1-GiB-Startpunkt in [Ressourcen](#resources) ist ein Minimum, nicht die Obergrenze.

Horizontal-Skalierungs-Routing hängt von Ihrem Muster ab. Für lang laufende Sitzungen, bei denen Container viele Sitzungen halten, führen Sie einen Pool von Containern hinter einem Load Balancer aus und heften Sie jede Sitzung mit konsistentem Hashing auf `sessionId` an einen Container. Eine angeheftete Sitzung trifft immer wieder auf denselben Container und daher auf denselben laufenden Unterprozess, bis er entfernt oder der Container neu gestartet wird.

Große Fanouts von gleichzeitigen [Subagenten](/docs/de/agent-sdk/subagents) aus einer einzelnen Sitzung können API-Ratenlimits treffen. Teilen Sie die Arbeit in kleinere Batches auf, anstatt eine breite Verteilung auszugeben.

<h3 id="cost">
  Kosten
</h3>

Die Anthropic-Token-Kosten dominieren typischerweise die Container-Infrastrukturkosten um eine Größenordnung oder mehr. Ein minimal bereitgestellter Container läuft ungefähr \$0,05 pro Stunde, während eine einzelne lange Agent-Sitzung Dollar in Token ausgeben kann. Siehe [Kostenverfolgung](/docs/de/agent-sdk/cost-tracking) für Token-Buchhaltung pro Sitzung.

<h3 id="multi-tenant-isolation">
  Multi-Tenant-Isolation
</h3>

Das standardmäßige SDK-Verhalten liest Einstellungen und `CLAUDE.md`-Speicherdateien aus dem Dateisystem. In einem gemeinsamen Container, der mehrere Mandanten bedient, können diese Dateien den Kontext eines Mandanten in die Sitzung eines anderen Mandanten durchsickern lassen.

Um Mandanten in einem gemeinsamen Container zu isolieren:

* Übergeben Sie `settingSources: []` in TypeScript oder `setting_sources=[]` in Python, damit keine Dateisystem-Einstellungen geladen werden.
* Setzen Sie `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`. [Auto Memory](/docs/de/memory#auto-memory) bei `~/.claude/projects/<project>/memory/` wird unabhängig von `settingSources` in den System-Prompt geladen. Siehe [Was settingSources nicht steuert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) für die anderen Eingaben, die bedingungslos geladen werden.
* Zeigen Sie `CLAUDE_CONFIG_DIR` auf ein mandantenspezifisches Verzeichnis, damit Mandanten die globale Konfiguration `~/.claude.json` nicht teilen.
* Verwenden Sie ein mandantenspezifisches Arbeitsverzeichnis. Übergeben Sie `cwd` explizit bei jedem `query()`-Aufruf.
* Wenden Sie mandantenspezifische Egress-Regeln bei Ihrem Proxy an, wie z. B. unterschiedliche ausgehende IPs, Anmeldedaten oder Domain-Allowlists, damit ein kompromittierter Mandant keine Daten über die ausgehende Richtlinie eines anderen Mandanten exfiltrieren kann.

Das folgende Beispiel wendet die vier SDK-Ebenen-Optionen zusammen an. Konstruieren Sie `tenantDir` und `configDir` so, dass jeder Mandant einen Pfad erhält, den kein anderer Mandant lesen kann. In TypeScript ersetzt `env` die Unterprozess-Umgebung, daher verteilen Sie `...process.env`, um geerbte Variablen wie `PATH` und `ANTHROPIC_API_KEY` zu behalten. In Python wird `env` auf die geerbte Umgebung zusammengeführt.

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
  ```
</CodeGroup>

Für mandantenspezifische Netzwerkkontrollen siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment).

<h2 id="known-limitations">
  Bekannte Einschränkungen
</h2>

Berücksichtigen Sie diese in Ihrem Bereitstellungsdesign.

| Einschränkung                                                      | Was zu tun ist                                                                                                                                                                                                                                                                                                                     |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Kein Sitzungs-Timeout auf oberster Ebene                           | Eine Sitzung läuft nicht automatisch ab. Legen Sie `maxTurns` in `Options` fest, um zu begrenzen, wie viele Tool-Use-Rundläufe der Agent durchführt, bevor er stoppt.                                                                                                                                                              |
| Speicherwachstum über lange Sitzungen                              | Begrenzen Sie die Sitzungslänge oder recyceln Sie Subprozesse regelmäßig. Siehe [Skalierung und Parallelität](#scaling-and-concurrency).                                                                                                                                                                                           |
| Große parallele Subagent-Ausfächerungen können Ratenlimits treffen | Teilen Sie die Arbeit in kleinere Batches auf, anstatt eine breite Verteilung auszugeben.                                                                                                                                                                                                                                          |
| Keine Wanduhr-Frist pro Subagent                                   | Begrenzen Sie jeden [Subagent](/docs/de/agent-sdk/subagents) mit `maxTurns` in seiner `AgentDefinition`. Nur für Hintergrund-Subagenten setzt `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` einen Stall-Watchdog, der aktiviert wird, wenn ein `run_in_background`-Subagent keine Ausgabe mehr produziert; dies ist keine Gesamtlaufzeit-Frist. |

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Hosting-Cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): Notebook-Anleitung mit [bereitstellbarem Code](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) für Docker, Modal und Kubernetes.
* [Sitzungsspeicher](/docs/de/agent-sdk/session-storage): Persistieren Sie Transkripte über Hosts hinweg mit einem `SessionStore`-Adapter.
* [Observability](/docs/de/agent-sdk/observability): Exportieren Sie OTEL-Traces, Metriken und Protokolle zu Ihrem Collector.
* [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment): Netzwerkkontrollen, Verwaltung von Anmeldedaten und Isolationshärtung.
* [Kostenverfolgung](/docs/de/agent-sdk/cost-tracking): Token- und Kostenabrechnung pro Sitzung.
