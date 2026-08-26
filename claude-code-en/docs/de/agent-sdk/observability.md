> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Observabilität mit OpenTelemetry

> Exportieren Sie Traces, Metriken und Events aus dem Agent SDK in Ihr Observability-Backend mit OpenTelemetry.

Wenn Sie Agenten in der Produktion ausführen, benötigen Sie Einblick in ihre Aktivitäten:

* welche Tools sie aufgerufen haben
* wie lange jede Modellanfrage gedauert hat
* wie viele Token ausgegeben wurden
* wo Fehler aufgetreten sind

Das Agent SDK kann diese Daten als OpenTelemetry-Traces, Metriken und Log-Events in jedes Backend exportieren, das das OpenTelemetry Protocol (OTLP) akzeptiert, wie Honeycomb, Datadog, Grafana, Langfuse oder einen selbst gehosteten Collector.

Dieser Leitfaden erklärt, wie das SDK Telemetrie ausgibt, wie Sie den Export konfigurieren und wie Sie die Daten nach ihrer Ankunft in Ihrem Backend taggen und filtern. Um Token-Nutzung und Kosten direkt aus dem SDK-Antwortstrom zu lesen, anstatt in ein Backend zu exportieren, siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking).

<h2 id="how-telemetry-flows-from-the-sdk">
  Wie Telemetrie vom SDK fließt
</h2>

Das Agent SDK führt die Claude Code CLI als untergeordneten Prozess aus und kommuniziert mit ihr über eine lokale Pipe. Die CLI hat OpenTelemetry-Instrumentierung eingebaut: Sie zeichnet Spans um jede Modellanfrage und Tool-Ausführung auf, gibt Metriken für Token- und Kostenleistungsindikatoren aus und gibt strukturierte Log-Events für Prompts und Tool-Ergebnisse aus. Das SDK produziert keine Telemetrie selbst. Stattdessen übergibt es die Konfiguration an den CLI-Prozess, und die CLI exportiert direkt zu Ihrem Collector.

Die Konfiguration wird als Umgebungsvariablen übergeben. Standardmäßig erbt der untergeordnete Prozess die Umgebung Ihrer Anwendung, sodass Sie die Telemetrie an einem von zwei Orten konfigurieren können:

* **Prozessumgebung:** Setzen Sie die Variablen in Ihrer Shell, Ihrem Container oder Orchestrator, bevor Ihre Anwendung startet. Jeder `query()`-Aufruf nimmt sie automatisch ohne Codeänderung auf. Dies ist der empfohlene Ansatz für Produktionsbereitstellungen.
* **Pro-Aufruf-Optionen:** Setzen Sie die Variablen in `ClaudeAgentOptions.env` (Python) oder `options.env` (TypeScript). Verwenden Sie dies, wenn verschiedene Agenten im selben Prozess unterschiedliche Telemetrie-Einstellungen benötigen. In Python wird `env` auf die geerbte Umgebung zusammengeführt. In TypeScript ersetzt `env` die geerbte Umgebung vollständig, daher schließen Sie `...process.env` in das Objekt ein, das Sie übergeben.

Die CLI exportiert drei unabhängige OpenTelemetry-Signale. Jedes hat seinen eigenen Enable-Schalter und seinen eigenen Exporter, sodass Sie nur die benötigten aktivieren können.

| Signal     | Was es enthält                                                                        | Aktivieren mit                                                      |
| ---------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Metriken   | Leistungsindikatoren für Token, Kosten, Sitzungen, Codezeilen und Tool-Entscheidungen | `OTEL_METRICS_EXPORTER`                                             |
| Log-Events | Strukturierte Datensätze für jeden Prompt, API-Anfrage, API-Fehler und Tool-Ergebnis  | `OTEL_LOGS_EXPORTER`                                                |
| Traces     | Spans für jede Interaktion, Modellanfrage, Tool-Aufruf und Hook (Beta)                | `OTEL_TRACES_EXPORTER` plus `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

Die vollständige Liste der Metriknamen, Event-Namen und Attribute finden Sie in der Claude Code [Überwachungs](/docs/de/monitoring-usage)-Referenz. Das Agent SDK gibt die gleichen Daten aus, da es die gleiche CLI ausführt. Span-Namen sind in [Agent-Traces lesen](#read-agent-traces) unten aufgelistet.

<h2 id="enable-telemetry-export">
  Telemetrie-Export aktivieren
</h2>

Telemetrie ist deaktiviert, bis Sie `CLAUDE_CODE_ENABLE_TELEMETRY=1` setzen und mindestens einen Exporter wählen. Die häufigste Konfiguration sendet alle drei Signale über OTLP HTTP an einen Collector.

Das folgende Beispiel setzt die Variablen in einem Dictionary und übergibt sie durch `options.env`. Der Agent führt eine einzelne Aufgabe aus, und die CLI exportiert Spans, Metriken und Events an den Collector unter `collector.example.com`, während die Schleife den Antwortstrom verbraucht:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # Required for traces, which are in beta. Metrics and log events do not need this.
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # Choose an exporter per signal. Use otlp for the SDK; see the Note below.
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # Standard OTLP transport configuration.
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // Required for traces, which are in beta. Metrics and log events do not need this.
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // Choose an exporter per signal. Use otlp for the SDK; see the Note below.
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // Standard OTLP transport configuration.
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // env replaces the inherited environment in TypeScript, so spread
    // process.env first to keep PATH, ANTHROPIC_API_KEY, and other variables.
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Da der untergeordnete Prozess standardmäßig die Umgebung Ihrer Anwendung erbt, können Sie das gleiche Ergebnis erreichen, indem Sie diese Variablen in einer Dockerfile, einem Kubernetes-Manifest oder einem Shell-Profil exportieren und `options.env` ganz weglassen.

<Note>
  Der `console`-Exporter schreibt Telemetrie in die Standardausgabe, die das SDK als seinen Nachrichtenkanal verwendet. Setzen Sie `console` nicht als Exporterwert, wenn Sie durch das SDK ausführen. Um Telemetrie lokal zu inspizieren, verweisen Sie `OTEL_EXPORTER_OTLP_ENDPOINT` stattdessen auf einen lokalen Collector oder einen All-in-One-Jaeger-Container.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  Telemetrie aus kurzlebigen Aufrufen leeren
</h3>

Die CLI batched Telemetrie und exportiert in einem Intervall. Bei einem sauberen Prozessabschluss versucht sie, ausstehende Daten zu leeren, aber das Leeren ist durch ein kurzes Timeout begrenzt, sodass Spans immer noch gelöscht werden können, wenn der Collector langsam reagiert. Wenn Ihr Prozess beendet wird, bevor die CLI heruntergefahren wird, geht alles, was sich noch im Batch-Puffer befindet, verloren. Das Senken der Export-Intervalle reduziert beide Fenster.

Standardmäßig exportieren Metriken alle 60 Sekunden und Traces und Logs exportieren alle 5 Sekunden. Das folgende Beispiel verkürzt alle drei Intervalle, sodass Daten den Collector erreichen, während eine kurze Aufgabe noch ausgeführt wird:

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... exporter configuration from the previous example ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... exporter configuration from the previous example ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  Agent-Traces lesen
</h2>

Traces geben Ihnen die detaillierteste Ansicht eines Agent-Laufs. Mit `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` gesetzt, wird jeder Schritt der Agent-Schleife zu einem Span, den Sie in Ihrem Tracing-Backend inspizieren können:

* **`claude_code.interaction`:** umhüllt eine einzelne Runde der Agent-Schleife, vom Empfangen eines Prompts bis zur Erzeugung einer Antwort.
* **`claude_code.llm_request`:** umhüllt jeden Aufruf der Claude API mit Modellname, Latenz und Token-Zählungen als Attribute.
* **`claude_code.tool`:** umhüllt jede Tool-Invokation mit untergeordneten Spans für das Berechtigungswarten (`claude_code.tool.blocked_on_user`) und die Ausführung selbst (`claude_code.tool.execution`).
* **`claude_code.hook`:** umhüllt jede [Hook](/docs/de/agent-sdk/hooks)-Ausführung. Erfordert detailliertes Beta-Tracing (`ENABLE_BETA_TRACING_DETAILED=1` und `BETA_TRACING_ENDPOINT`) zusätzlich zu den obigen Variablen.

Die `llm_request`-, `tool`- und `hook`-Spans sind untergeordnete Elemente des umschließenden `claude_code.interaction`-Spans. Wenn der Agent einen Subagenten durch das Task-Tool spawnt, verschachteln sich die `llm_request`- und `tool`-Spans des Subagenten unter dem `claude_code.tool`-Span des übergeordneten Agenten, sodass die vollständige Delegationskette als ein Trace erscheint.

Spans tragen standardmäßig ein `session.id`-Attribut. Wenn Sie mehrere `query()`-Aufrufe gegen die gleiche [Sitzung](/docs/de/agent-sdk/sessions) machen, filtern Sie auf `session.id` in Ihrem Backend, um sie als eine Zeitleiste zu sehen. Das Attribut wird weggelassen, wenn `OTEL_METRICS_INCLUDE_SESSION_ID` auf einen falschen Wert gesetzt ist.

<Note>
  Tracing ist in Beta. Span-Namen und Attribute können sich zwischen Releases ändern. Siehe [Traces (Beta)](/docs/de/monitoring-usage#traces-beta) in der Überwachungsreferenz für die Trace-Exporter-Konfigurationsvariablen.
</Note>

<h2 id="link-traces-to-your-application">
  Traces mit Ihrer Anwendung verknüpfen
</h2>

Das SDK propagiert automatisch W3C-Trace-Kontext in den CLI-Subprozess. Wenn Sie `query()` aufrufen, während ein OpenTelemetry-Span in Ihrer Anwendung aktiv ist, injiziert das SDK `TRACEPARENT` und `TRACESTATE` in die Umgebung des untergeordneten Prozesses, und die CLI liest sie, sodass sein `claude_code.interaction`-Span ein untergeordnetes Element Ihres Spans wird. Der Agent-Lauf erscheint dann in Ihrem Anwendungs-Trace, anstatt als getrennter Root.

Wenn die Trace-Kontext-Propagation aktiviert ist, leitet die CLI auch `TRACEPARENT` an jeden Bash- und PowerShell-Befehl weiter, den sie ausführt. Wenn ein Befehl, der durch das Bash-Tool gestartet wird, seine eigenen OpenTelemetry-Spans ausgibt, verschachteln sich diese Spans unter dem `claude_code.tool.execution`-Span, der den Befehl umhüllt.

Die Auto-Injektion wird übersprungen, wenn Sie `TRACEPARENT` explizit in `options.env` setzen, sodass Sie einen bestimmten übergeordneten Kontext anheften können, falls erforderlich. Interaktive CLI-Sitzungen ignorieren eingehende `TRACEPARENT` vollständig; nur Agent SDK und `claude -p`-Läufe beachten sie. Siehe [Traces (Beta)](/docs/de/monitoring-usage#traces-beta) in der Überwachungsreferenz für die vollständige Span- und Attributreferenz.

<h2 id="tag-telemetry-from-your-agent">
  Telemetrie von Ihrem Agenten taggen
</h2>

Standardmäßig meldet die CLI `service.name` als `claude-code`. Wenn Sie mehrere Agenten ausführen oder das SDK neben anderen Services ausführen, die in den gleichen Collector exportieren, überschreiben Sie den Service-Namen und fügen Sie Ressourcenattribute hinzu, damit Sie nach Agent in Ihrem Backend filtern können.

Das folgende Beispiel benennt den Service um und fügt Bereitstellungsmetadaten an. Diese Werte werden als OpenTelemetry-Ressourcenattribute auf jedem Span, jeder Metrik und jedem Event angewendet, das der Agent ausgibt:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  Attribut-Aktionen Ihren Endbenutzern zuordnen
</h2>

Die CLI fügt [Identitätsattribute](/docs/de/monitoring-usage#standard-attributes) an jeden Event basierend auf der Anmeldeinformation an, die sie zum Aufrufen von Anthropic verwendet. Wenn Sie eine Anwendung erstellen, die viele Endbenutzer aus einer Bereitstellung bedient, identifizieren diese Attribute die Anmeldeinformation Ihres Service, nicht den Endbenutzer, in dessen Namen der Agent handelte.

Um Tool-Aufrufe und MCP-Aktivität Ihren Anwendungs-Endbenutzern zuzuordnen, injizieren Sie die Endbenutzer-Identität als Ressourcenattribute bei jedem `query()`-Aufruf. Prozentcodieren Sie Werte, bevor Sie sie interpolieren, da `OTEL_RESOURCE_ATTRIBUTES` [Kommas, Leerzeichen und Gleichheitszeichen reserviert](/docs/de/monitoring-usage#multi-team-organization-support). Das folgende Beispiel fügt den anfordernden Benutzer und Mandanten an jeden Span und Event aus einer Anfrage an. Es setzt ein `request`-Objekt aus Ihrem Web-Framework voraus, das die Benutzer- und Mandanten-IDs trägt:

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

Mit Endbenutzer-Identität angeheftet, werden die `tool_decision`-, `tool_result`-, `mcp_server_connection`- und `permission_mode_changed`-Events, die als Log-Datensätze mit einem `claude_code.`-Präfix exportiert werden, zu einem Pro-Benutzer-Audit-Trail, den Sie an eine Security Information and Event Management (SIEM)-Plattform weiterleiten können. Siehe [Sicherheitsereignisse prüfen](/docs/de/monitoring-usage#audit-security-events) in der Überwachungsreferenz für die vollständige Liste sicherheitsrelevanter Events und der Attribute, die jedes trägt.

<h2 id="control-sensitive-data-in-exports">
  Kontrollieren Sie sensible Daten in Exporten
</h2>

Telemetrie ist standardmäßig strukturell. Dauern, Modellnamen und Tool-Namen werden auf jedem Span aufgezeichnet; Token-Zählungen werden aufgezeichnet, wenn die zugrunde liegende API-Anfrage Nutzungsdaten zurückgibt, sodass Spans für fehlgeschlagene oder abgebrochene Anfragen diese möglicherweise weglassen. Der Inhalt, den Ihr Agent liest und schreibt, wird standardmäßig nicht aufgezeichnet. Diese Opt-in-Variablen fügen Inhalte zu den exportierten Daten hinzu:

| Variable                  | Fügt hinzu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | Prompt-Text auf `claude_code.user_prompt`-Events und auf dem `claude_code.interaction`-Span                                                                                                                                                                                                                                                                                                                                                                                                        |
| `OTEL_LOG_TOOL_DETAILS=1` | Tool-Eingabeargumente (Dateipfade, Shell-Befehle, Suchmuster) auf `claude_code.tool_result`-Events                                                                                                                                                                                                                                                                                                                                                                                                 |
| `OTEL_LOG_TOOL_CONTENT=1` | Vollständige Tool-Eingabe- und Ausgabetexte als Span-Events auf `claude_code.tool`, gekürzt auf 60 KB. Erfordert, dass [Tracing](#read-agent-traces) aktiviert ist                                                                                                                                                                                                                                                                                                                                 |
| `OTEL_LOG_RAW_API_BODIES` | Vollständige Anthropic Messages API-Anfrage und Antwort JSON als `claude_code.api_request_body` und `claude_code.api_response_body` Log-Events. Setzen Sie auf `1` für Inline-Texte gekürzt auf 60 KB oder `file:<dir>` für ungekürzte Texte auf der Festplatte mit einem `body_ref`-Pfad im Event. Texte enthalten die gesamte Gesprächshistorie und haben erweiterte Denkinhalte redigiert. Das Aktivieren davon impliziert Zustimmung zu allem, was die drei obigen Variablen offenbaren würden |

Lassen Sie diese ungesetzt, es sei denn, Ihre Observability-Pipeline ist genehmigt, um die Daten zu speichern, die Ihr Agent verarbeitet. Siehe [Sicherheit und Datenschutz](/docs/de/monitoring-usage#security-and-privacy) in der Überwachungsreferenz für die vollständige Liste der Attribute und des Redaktionsverhaltens.

<h2 id="related-documentation">
  Zugehörige Dokumentation
</h2>

Diese Leitfäden behandeln angrenzende Themen für Überwachung und Bereitstellung von Agenten:

* [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking): Lesen Sie Token- und Kostendaten aus dem Nachrichtenstrom ohne ein externes Backend.
* [Hosting des Agent SDK](/docs/de/agent-sdk/hosting): Stellen Sie Agenten in Containern bereit, in denen Sie OpenTelemetry-Variablen auf Umgebungsebene setzen können.
* [Überwachung](/docs/de/monitoring-usage): Die vollständige Referenz für jede Umgebungsvariable, Metrik und jeden Event, den die CLI ausgibt.
