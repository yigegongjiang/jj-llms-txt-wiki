> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Überwachung

> Erfahren Sie, wie Sie OpenTelemetry für Claude Code aktivieren und konfigurieren.

Verfolgen Sie die Nutzung, Kosten und Toolaktivität von Claude Code in Ihrer Organisation, indem Sie Telemetriedaten über OpenTelemetry (OTel) exportieren. Claude Code exportiert Metriken als Zeitreihendaten über das Standard-Metriken-Protokoll, Ereignisse über das Logs/Events-Protokoll und optional verteilte Traces über das [Traces-Protokoll](#traces-beta). Konfigurieren Sie Ihre Metriken-, Logs- und Traces-Backends, um Ihre Überwachungsanforderungen zu erfüllen.

<h2 id="quick-start">
  Schnellstart
</h2>

Konfigurieren Sie OpenTelemetry mit Umgebungsvariablen:

```bash theme={null}
# 1. Telemetrie aktivieren
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. Exporter auswählen (beide sind optional - konfigurieren Sie nur das, was Sie benötigen)
export OTEL_METRICS_EXPORTER=otlp       # Optionen: otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # Optionen: otlp, console, none

# 3. OTLP-Endpunkt konfigurieren (für OTLP-Exporter)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. Authentifizierung festlegen (falls erforderlich)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. Zum Debuggen: Exportintervalle reduzieren
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 Sekunden (Standard: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 Sekunden (Standard: 5000ms)

# 6. Claude Code ausführen
claude
```

<Note>
  Die Standard-Exportintervalle betragen 60 Sekunden für Metriken und 5 Sekunden für Logs. Während der Einrichtung möchten Sie möglicherweise kürzere Intervalle für Debugging-Zwecke verwenden. Denken Sie daran, diese für die Produktionsnutzung zurückzusetzen.
</Note>

Für vollständige Konfigurationsoptionen siehe die [OpenTelemetry-Spezifikation](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Administratorkonfiguration
</h2>

Administratoren können OpenTelemetry-Einstellungen für alle Benutzer über die [verwaltete Einstellungsdatei](/docs/de/settings#settings-files) konfigurieren. Dies ermöglicht eine zentrale Kontrolle der Telemetrie-Einstellungen in einer Organisation. Weitere Informationen zur Anwendung von Einstellungen finden Sie unter [Einstellungspriorität](/docs/de/settings#settings-precedence).

Beispiel für verwaltete Einstellungskonfiguration:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

<Note>
  Verwaltete Einstellungen können über MDM (Mobile Device Management) oder andere Geräteverwaltungslösungen verteilt werden. Umgebungsvariablen, die in der verwalteten Einstellungsdatei definiert sind, haben hohe Priorität und können von Benutzern nicht überschrieben werden.
</Note>

Claude Code übergibt `OTEL_*` Umgebungsvariablen nicht an die Subprozesse, die es erzeugt, einschließlich des Bash-Tools, Hooks, MCP-Server und Sprachserver. Eine OpenTelemetry-instrumentierte Anwendung, die Sie über das Bash-Tool ausführen, erbt nicht den Exporter-Endpunkt oder die Header von Claude Code, daher setzen Sie diese Variablen direkt im Befehl, wenn diese Anwendung ihre eigene Telemetrie exportieren muss.

<h2 id="configuration-details">
  Konfigurationsdetails
</h2>

<h3 id="common-configuration-variables">
  Allgemeine Konfigurationsvariablen
</h3>

| Umgebungsvariable                                   | Beschreibung                                                                                                                                                                                                                                                                                                                                                        | Beispielwerte                                                                                                                             |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Aktiviert die Telemetrieerfassung (erforderlich)                                                                                                                                                                                                                                                                                                                    | `1`                                                                                                                                       |
| `OTEL_METRICS_EXPORTER`                             | Metriken-Exporter-Typ(en), kommagetrennt. Verwenden Sie `none` zum Deaktivieren                                                                                                                                                                                                                                                                                     | `console`, `otlp`, `prometheus`, `none`                                                                                                   |
| `OTEL_LOGS_EXPORTER`                                | Logs/Events-Exporter-Typ(en), kommagetrennt. Verwenden Sie `none` zum Deaktivieren                                                                                                                                                                                                                                                                                  | `console`, `otlp`, `none`                                                                                                                 |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protokoll für OTLP-Exporter, gilt für alle Signale                                                                                                                                                                                                                                                                                                                  | `grpc`, `http/json`, `http/protobuf`                                                                                                      |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | OTLP-Collector-Endpunkt für alle Signale                                                                                                                                                                                                                                                                                                                            | `http://localhost:4317`                                                                                                                   |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protokoll für Metriken, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                         | `grpc`, `http/json`, `http/protobuf`                                                                                                      |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | OTLP-Metriken-Endpunkt, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                         | `http://localhost:4318/v1/metrics`                                                                                                        |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protokoll für Logs, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                             | `grpc`, `http/json`, `http/protobuf`                                                                                                      |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | OTLP-Logs-Endpunkt, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                             | `http://localhost:4318/v1/logs`                                                                                                           |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Authentifizierungsheader für OTLP                                                                                                                                                                                                                                                                                                                                   | `Authorization=Bearer token`                                                                                                              |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Exportintervall in Millisekunden (Standard: 60000)                                                                                                                                                                                                                                                                                                                  | `5000`, `60000`                                                                                                                           |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Logs-Exportintervall in Millisekunden (Standard: 5000)                                                                                                                                                                                                                                                                                                              | `1000`, `10000`                                                                                                                           |
| `OTEL_LOG_USER_PROMPTS`                             | Aktiviert die Protokollierung von Benutzer-Prompt-Inhalten (Standard: deaktiviert)                                                                                                                                                                                                                                                                                  | `1` zum Aktivieren                                                                                                                        |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Aktiviert die Protokollierung von Assistent-Antworttext bei `assistant_response`-Ereignissen (Standard: deaktiviert). Wenn nicht gesetzt, wird auf den Wert von `OTEL_LOG_USER_PROMPTS` zurückgegriffen. Erfordert Claude Code v2.1.193 oder später                                                                                                                 | `1` zum Aktivieren, `0` zum Beibehalten von Redaktionen                                                                                   |
| `OTEL_LOG_TOOL_DETAILS`                             | Aktiviert die Protokollierung von Tool-Parametern und Eingabeargumenten in Tool-Ereignissen und Trace-Span-Attributen: Bash-Befehle, MCP-Server- und Tool-Namen, Skill-Namen, benutzerdefinierte Workflow-Namen und Tool-Eingabe. Aktiviert auch benutzerdefinierte, Plugin- und MCP-Befehlsnamen bei `user_prompt`-Ereignissen (Standard: deaktiviert)             | `1` zum Aktivieren                                                                                                                        |
| `OTEL_LOG_TOOL_CONTENT`                             | Aktiviert die Protokollierung von Tool-Eingabe- und Ausgabeinhalten in Span-Ereignissen (Standard: deaktiviert). Erfordert [Tracing](#traces-beta). Der Inhalt wird bei 60 KB gekürzt                                                                                                                                                                               | `1` zum Aktivieren                                                                                                                        |
| `OTEL_LOG_RAW_API_BODIES`                           | Gibt die vollständige Anthropic Messages API-Anfrage und Antwort JSON als `api_request_body` / `api_response_body` Log-Ereignisse aus (Standard: deaktiviert). Texte enthalten die gesamte Konversationshistorie. Das Aktivieren impliziert Zustimmung zu allem, was `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` und `OTEL_LOG_TOOL_CONTENT` offenbaren würden | `1` für Inline-Texte gekürzt bei 60 KB, oder `file:<dir>` für ungekürzte Texte auf der Festplatte mit einem `body_ref`-Zeiger im Ereignis |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Metriken-Temporalitätspräferenz (Standard: `delta`). Setzen Sie auf `cumulative`, wenn Ihr Backend kumulative Temporalität erwartet                                                                                                                                                                                                                                 | `delta`, `cumulative`                                                                                                                     |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervall zum Aktualisieren dynamischer Header (Standard: 1740000ms / 29 Minuten)                                                                                                                                                                                                                                                                                   | `900000`                                                                                                                                  |

<h3 id="mtls-authentication">
  mTLS-Authentifizierung
</h3>

Wie Sie Client-Zertifikate für den OTLP-Exporter konfigurieren, hängt vom OTLP-Protokoll ab, das für dieses Signal verwendet wird, das über `OTEL_EXPORTER_OTLP_PROTOCOL` oder die Pro-Signal-Überschreibung gesetzt wird. Die gleiche Konfiguration gilt für Metriken, Logs und Traces.

| Protokoll                    | Client-Zertifikat-Variablen                                                                                                                                                                               | Vertrauen Sie dem Collector-CA mit |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY` und optional `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Siehe [Netzwerkkonfiguration](/docs/de/network-config#mtls-authentication)                               | `NODE_EXTRA_CA_CERTS`              |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` und `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, oder die Pro-Signal-Varianten wie `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY`, um ein anderes Zertifikat pro Signal zu verwenden | `OTEL_EXPORTER_OTLP_CERTIFICATE`   |

Für `grpc` liest das OpenTelemetry SDK die Standard-OTLP-Variablen direkt, daher funktionieren bestehende Konfigurationen, die die Pro-Signal-Metriken-Variablen setzen, weiterhin.

<h3 id="metrics-cardinality-control">
  Metriken-Kardinalitätskontrolle
</h3>

Die folgenden Umgebungsvariablen steuern, welche Attribute in Metriken enthalten sind, um die Kardinalität zu verwalten:

| Umgebungsvariable                          | Beschreibung                                                                                | Standardwert | Beispiel zum Deaktivieren |
| ------------------------------------------ | ------------------------------------------------------------------------------------------- | ------------ | ------------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Attribut session.id in Metriken einschließen                                                | `true`       | `false`                   |
| `OTEL_METRICS_INCLUDE_VERSION`             | Attribut app.version in Metriken einschließen                                               | `false`      | `true`                    |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Attribute user.account\_uuid und user.account\_id in Metriken einschließen                  | `true`       | `false`                   |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Attribut app.entrypoint in Metriken einschließen                                            | `false`      | `true`                    |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Schlüssel aus `OTEL_RESOURCE_ATTRIBUTES` als Attribute auf Metrik-Datenpunkten einschließen | `true`       | `false`                   |

Diese Variablen helfen, die Kardinalität von Metriken zu kontrollieren, was sich auf die Speicheranforderungen und die Abfrageleistung in Ihrem Metriken-Backend auswirkt. Eine niedrigere Kardinalität bedeutet in der Regel bessere Leistung und niedrigere Speicherkosten, aber weniger granulare Daten für die Analyse.

<h3 id="traces-beta">
  Traces (Beta)
</h3>

Verteiltes Tracing exportiert Spans, die jeden Benutzer-Prompt mit den API-Anfragen und Tool-Ausführungen verknüpfen, die er auslöst, sodass Sie eine vollständige Anfrage als einzelnen Trace in Ihrem Tracing-Backend anzeigen können.

Tracing ist standardmäßig deaktiviert. Um es zu aktivieren, setzen Sie sowohl `CLAUDE_CODE_ENABLE_TELEMETRY=1` als auch `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, und setzen Sie dann `OTEL_TRACES_EXPORTER`, um auszuwählen, wohin Spans gesendet werden. Traces verwenden die [allgemeine OTLP-Konfiguration](#common-configuration-variables) für Endpunkt, Protokoll, Header und [mTLS](#mtls-authentication) erneut.

| Umgebungsvariable                     | Beschreibung                                                                                 | Beispielwerte                        |
| ------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Aktiviert Span-Tracing (erforderlich). `ENABLE_ENHANCED_TELEMETRY_BETA` wird auch akzeptiert | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Traces-Exporter-Typ(en), kommagetrennt. Verwenden Sie `none` zum Deaktivieren                | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protokoll für Traces, überschreibt `OTEL_EXPORTER_OTLP_PROTOCOL`                             | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | OTLP-Traces-Endpunkt, überschreibt `OTEL_EXPORTER_OTLP_ENDPOINT`                             | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Span-Batch-Exportintervall in Millisekunden (Standard: 5000)                                 | `1000`, `10000`                      |

Spans schwärzen Benutzer-Prompt-Text, Tool-Eingabedetails und Tool-Inhalte standardmäßig. Setzen Sie `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` und `OTEL_LOG_TOOL_CONTENT=1`, um sie einzubeziehen.

Wenn Tracing aktiv ist, erben Bash- und PowerShell-Subprozesse automatisch eine `TRACEPARENT`-Umgebungsvariable, die den W3C-Trace-Kontext des aktiven Tool-Ausführungs-Spans enthält. Dies ermöglicht jedem Subprozess, der `TRACEPARENT` liest, seine eigenen Spans unter demselben Trace zu verschachteln, was eine End-to-End-verteilte Tracing durch Skripte und Befehle ermöglicht, die Claude ausführt.

Wenn Tracing aktiv ist und Claude Code direkt mit der Anthropic API verbunden ist, trägt jede Modellanfrage einen W3C `traceparent` Header, der auf den Kontext des `claude_code.llm_request` Spans gesetzt ist, und der `traceresponse` Header der API wird als Span-Link aufgezeichnet. Zusammen verbinden diese Claude Code's clientseitige Spans mit dem serverseitigen Trace durch jeden konformen Vermittler. Der Header wird nicht an Drittanbieter gesendet.

Standardmäßig wird der `traceparent` Header bei Model- und HTTP MCP-Anfragen nur gesendet, wenn `ANTHROPIC_BASE_URL` nicht gesetzt ist oder auf die Anthropic API verweist, da einige Proxys unbekannte Header ablehnen. Die Subprozess-`TRACEPARENT`-Variable wird durch denselben Schalter aus Konsistenzgründen gesteuert. Wenn Sie Claude Code durch einen benutzerdefinierten `ANTHROPIC_BASE_URL` Proxy ausführen und Trace-Kontext propagiert werden soll, setzen Sie `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

In Agent SDK und nicht-interaktiven Sitzungen, die mit `-p` gestartet werden, liest Claude Code auch `TRACEPARENT` und `TRACESTATE` aus seiner eigenen Umgebung, wenn jeder Interaktions-Span gestartet wird. Dies ermöglicht einem Embedding-Prozess, seinen aktiven W3C-Trace-Kontext in den Subprozess zu übergeben, sodass Claude Code's Spans als untergeordnete Elemente des Aufrufers verteilter Trace erscheinen. Interaktive Sitzungen ignorieren eingehende `TRACEPARENT`, um zu vermeiden, dass versehentlich Umgebungswerte aus CI oder Container-Umgebungen geerbt werden.

<h4 id="span-hierarchy">
  Span-Hierarchie
</h4>

Jeder Benutzer-Prompt startet einen `claude_code.interaction` Root-Span. API-Aufrufe, Tool-Aufrufe und Hook-Ausführungen werden als untergeordnete Elemente aufgezeichnet. Tool-Spans haben zwei untergeordnete Spans: einen für die Zeit, die auf eine Berechtigungsentscheidung gewartet wird, und einen für die Ausführung selbst. Wenn das Agent-Tool oder das veraltete Task-Tool einen Subagenten erzeugt, werden die API- und Tool-Spans des Subagenten unter dem `claude_code.tool`-Span des übergeordneten Elements verschachtelt.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (erfordert detailliertes Beta-Tracing)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (Agent-Tool) Subagent claude_code.llm_request / claude_code.tool Spans
```

In Agent SDK und `claude -p` Sitzungen wird `claude_code.interaction` selbst ein untergeordnetes Element des Aufrufers-Spans, wenn `TRACEPARENT` in der Umgebung gesetzt ist.

<h4 id="span-attributes">
  Span-Attribute
</h4>

Jeder Span trägt die [Standardattribute](#standard-attributes) plus ein `span.type`-Attribut, das seinem Namen entspricht. Die folgenden Tabellen listen die zusätzlichen Attribute auf, die auf jedem Span gesetzt sind. Die Spans `llm_request`, `tool.execution` und `hook` setzen OpenTelemetry-Status `ERROR`, wenn sie einen Fehler aufzeichnen; die anderen Spans enden immer mit Status `UNSET`.

**`claude_code.interaction`**

| Attribut                  | Beschreibung                                                              | Gated durch             |
| ------------------------- | ------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Prompt-Text. Der Wert ist `<REDACTED>`, es sei denn, das Gate ist gesetzt | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Prompt-Länge in Zeichen                                                   |                         |
| `interaction.sequence`    | 1-basierter Zähler von Interaktionen in dieser Sitzung                    |                         |
| `interaction.duration_ms` | Wanduhr-Dauer des Durchgangs                                              |                         |

**`claude_code.llm_request`**

| Attribut                         | Beschreibung                                                                                                                                                                      | Gated durch             |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | Modellkennung                                                                                                                                                                     |                         |
| `gen_ai.system`                  | Immer `anthropic`. OpenTelemetry GenAI semantische Konvention                                                                                                                     |                         |
| `gen_ai.request.model`           | Gleicher Wert wie `model`. OpenTelemetry GenAI semantische Konvention                                                                                                             |                         |
| `query_source`                   | Subsystem, das die Anfrage gestellt hat, wie `repl_main_thread` oder ein Subagent-Name                                                                                            |                         |
| `agent_id`                       | Kennung des Subagenten oder Teamkollegen, der die Anfrage gestellt hat. Fehlt in der Hauptsitzung                                                                                 |                         |
| `parent_agent_id`                | Kennung des Agenten, der diesen erzeugt hat. Fehlt für die Hauptsitzung und für Agenten, die direkt von ihr erzeugt wurden                                                        |                         |
| `workflow.run_id`                | Run-Kennung des [Workflow](/docs/de/workflows) Tool-Durchlaufs, der diesen Agenten erzeugt hat, mit dem Präfix `wf_`. Fehlt für Agenten, die nicht durch einen Workflow erzeugt wurden |                         |
| `workflow.name`                  | Name des Workflows, der diesen Agenten erzeugt hat. Benutzerdefinierte Namen werden durch `custom` ersetzt, es sei denn, das Gate ist gesetzt                                     | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` oder `normal`                                                                                                                                                              |                         |
| `llm_request.context`            | `interaction`, `tool` oder `standalone` je nach übergeordnetem Span                                                                                                               |                         |
| `duration_ms`                    | Wanduhr-Dauer einschließlich Wiederholungen                                                                                                                                       |                         |
| `ttft_ms`                        | Zeit bis zum ersten Token in Millisekunden                                                                                                                                        |                         |
| `input_tokens`                   | Eingabe-Token-Anzahl aus dem API-Nutzungsblock                                                                                                                                    |                         |
| `output_tokens`                  | Ausgabe-Token-Anzahl                                                                                                                                                              |                         |
| `cache_read_tokens`              | Aus dem Prompt-Cache gelesene Token                                                                                                                                               |                         |
| `cache_creation_tokens`          | In den Prompt-Cache geschriebene Token                                                                                                                                            |                         |
| `request_id`                     | Anthropic API-Anfrage-ID aus dem `request-id` Response-Header                                                                                                                     |                         |
| `gen_ai.response.id`             | Gleicher Wert wie `request_id`. OpenTelemetry GenAI semantische Konvention                                                                                                        |                         |
| `client_request_id`              | Client-generierte `x-client-request-id` des letzten Versuchs                                                                                                                      |                         |
| `attempt`                        | Gesamtzahl der Versuche für diese Anfrage                                                                                                                                         |                         |
| `success`                        | `true` oder `false`                                                                                                                                                               |                         |
| `status_code`                    | HTTP-Statuscode, wenn die Anfrage fehlgeschlagen ist                                                                                                                              |                         |
| `error`                          | Fehlermeldung, wenn die Anfrage fehlgeschlagen ist                                                                                                                                |                         |
| `response.has_tool_call`         | `true`, wenn die Antwort Tool-Use-Blöcke enthielt                                                                                                                                 |                         |
| `stop_reason`                    | API-Antwort `stop_reason`, wie `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn` oder `refusal`                                                                 |                         |
| `gen_ai.response.finish_reasons` | Gleicher Wert wie `stop_reason`, in einem String-Array verpackt. OpenTelemetry GenAI semantische Konvention                                                                       |                         |

Jeder Wiederholungsversuch wird auch als `gen_ai.request.attempt` Span-Ereignis mit `attempt` und `client_request_id` Attributen aufgezeichnet.

**`claude_code.tool`**

| Attribut              | Beschreibung                                                                                                                                                                                                                                                        | Gated durch             |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Tool-Name                                                                                                                                                                                                                                                           |                         |
| `duration_ms`         | Wanduhr-Dauer einschließlich Berechtigungswartung und Ausführung                                                                                                                                                                                                    |                         |
| `result_tokens`       | Ungefähre Token-Größe des Tool-Ergebnisses                                                                                                                                                                                                                          |                         |
| `agent_id`            | Kennung des Subagenten oder Teamkollegen, der das Tool ausgeführt hat. Fehlt in der Hauptsitzung                                                                                                                                                                    |                         |
| `parent_agent_id`     | Kennung des Agenten, der diesen erzeugt hat. Fehlt für die Hauptsitzung und für Agenten, die direkt von ihr erzeugt wurden                                                                                                                                          |                         |
| `workflow.run_id`     | Run-Kennung des Workflow-Tool-Durchlaufs, der diesen Agenten erzeugt hat, mit dem Präfix `wf_`. Fehlt für Agenten, die nicht durch einen Workflow erzeugt wurden                                                                                                    |                         |
| `workflow.name`       | Name des Workflows, der diesen Agenten erzeugt hat. Benutzerdefinierte Namen werden durch `custom` ersetzt, es sei denn, das Gate ist gesetzt                                                                                                                       | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | Die Modell-`tool_use` Block-ID für diesen Aufruf. Entspricht der `tool_use_id` bei den [tool\_result](#tool-result-event) und [tool\_decision](#tool-decision-event) Ereignissen und in Hook-Payloads, sodass Sie den Span mit diesen Datensätzen verknüpfen können |                         |
| `gen_ai.tool.call.id` | Gleicher Wert wie `tool_use_id`. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                         |                         |
| `file_path`           | Zieldateipfad für Read-, Edit- und Write-Tools                                                                                                                                                                                                                      | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Befehlszeichenkette für das Bash-Tool                                                                                                                                                                                                                               | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Skill-Name für das Skill-Tool                                                                                                                                                                                                                                       | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Subagent-Typ für das Agent-Tool oder veraltete Task-Tool                                                                                                                                                                                                            | `OTEL_LOG_TOOL_DETAILS` |

Wenn `OTEL_LOG_TOOL_CONTENT=1`, zeichnet dieser Span auch ein `tool.output` Span-Ereignis auf, dessen Attribute die Tool-Eingabe- und Ausgabetexte enthalten, gekürzt bei 60 KB pro Attribut.

**`claude_code.tool.blocked_on_user`**

| Attribut      | Beschreibung                                                                              | Gated durch |
| ------------- | ----------------------------------------------------------------------------------------- | ----------- |
| `duration_ms` | Zeit, die auf die Berechtigungsentscheidung gewartet wird                                 |             |
| `decision`    | `accept` oder `reject`                                                                    |             |
| `source`      | Entscheidungsquelle, entsprechend dem [Tool-Entscheidungs-Ereignis](#tool-decision-event) |             |

**`claude_code.tool.execution`**

| Attribut              | Beschreibung                                                                                                                                                                  | Gated durch             |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Zeit, die für die Ausführung des Tool-Body aufgewendet wird                                                                                                                   |                         |
| `tool_use_id`         | Gleicher Wert wie auf dem übergeordneten `claude_code.tool` Span                                                                                                              |                         |
| `gen_ai.tool.call.id` | Gleicher Wert wie `tool_use_id`. OpenTelemetry GenAI semantische Konvention                                                                                                   |                         |
| `success`             | `true` oder `false`                                                                                                                                                           |                         |
| `error`               | Fehler-Kategoriezeichenkette, wenn die Ausführung fehlgeschlagen ist, wie `Error:ENOENT` oder `ShellError`. Enthält die vollständige Fehlermeldung, wenn das Gate gesetzt ist | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

Dieser Span wird nur ausgegeben, wenn detailliertes Beta-Tracing aktiv ist, was `ENABLE_BETA_TRACING_DETAILED=1` und `BETA_TRACING_ENDPOINT` zusätzlich zur obigen Trace-Exporter-Konfiguration erfordert. In interaktiven CLI-Sitzungen erfordert dies auch, dass Ihre Organisation für die Funktion auf die Whitelist gesetzt ist. Agent SDK und nicht-interaktive `-p` Sitzungen sind nicht gated. Es wird nicht ausgegeben, wenn nur `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` gesetzt ist.

| Attribut                 | Beschreibung                                                            | Gated durch             |
| ------------------------ | ----------------------------------------------------------------------- | ----------------------- |
| `hook_event`             | Hook-Ereignistyp, wie `PreToolUse`                                      |                         |
| `hook_name`              | Vollständiger Hook-Name, wie `PreToolUse:Write`                         |                         |
| `num_hooks`              | Anzahl der ausgeführten übereinstimmenden Hook-Befehle                  |                         |
| `hook_definitions`       | JSON-serialisierte Hook-Konfiguration                                   | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | Wanduhr-Dauer aller übereinstimmenden Hooks                             |                         |
| `num_success`            | Anzahl der Hooks, die erfolgreich abgeschlossen wurden                  |                         |
| `num_blocking`           | Anzahl der Hooks, die eine Blockierungsentscheidung zurückgegeben haben |                         |
| `num_non_blocking_error` | Anzahl der Hooks, die ohne Blockierung fehlgeschlagen sind              |                         |
| `num_cancelled`          | Anzahl der Hooks, die vor Abschluss abgebrochen wurden                  |                         |

<Note>
  Zusätzliche inhaltshaltige Attribute wie `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` und `response.model_output` werden nur ausgegeben, wenn detailliertes Beta-Tracing aktiv ist. Sie sind nicht Teil des stabilen Span-Schemas. `user_system_prompt` erfordert zusätzlich `OTEL_LOG_USER_PROMPTS=1`. Es trägt nur den System-Prompt-Text, den Sie über die `systemPrompt` SDK-Option oder die Flags `--system-prompt` und `--append-system-prompt` bereitstellen, gekürzt bei 60 KB, und wird einmal pro Sitzung statt pro Anfrage ausgegeben.
</Note>

<h3 id="dynamic-headers">
  Dynamische Header
</h3>

Für Unternehmensumgebungen, die eine dynamische Authentifizierung erfordern, können Sie ein Skript konfigurieren, um Header dynamisch zu generieren. Dynamische Header gelten nur für die Protokolle `http/protobuf` und `http/json`. Der `grpc` Exporter verwendet nur den statischen `OTEL_EXPORTER_OTLP_HEADERS` Wert.

<h4 id="settings-configuration">
  Einstellungskonfiguration
</h4>

Fügen Sie zu Ihrer `.claude/settings.json` hinzu:

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

Der Wert kann der Pfad zu einer ausführbaren Datei sein, einschließlich eines Pfads, der Leerzeichen enthält, oder eine Shell-Befehlszeile mit Argumenten. Unter Windows wird der Wert immer durch die Shell ausgeführt, daher setzen Sie einen Pfad, der Leerzeichen enthält, in Anführungszeichen innerhalb des JSON-Werts.

<h4 id="script-requirements">
  Skriptanforderungen
</h4>

Das Skript muss gültiges JSON mit Zeichenketten-Schlüssel-Wert-Paaren ausgeben, die HTTP-Header darstellen:

```bash theme={null}
#!/bin/bash
# Beispiel: Mehrere Header
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

Wenn das Helper-Skript fehlschlägt oder eine Ausgabe druckt, die diese Anforderungen nicht erfüllt, meldet Claude Code den Fehler in:

* `/status` Ausgabe
* Das Debug-Log, wenn Sie mit [`--debug`](/docs/de/cli-reference#cli-flags) ausführen oder nach dem Ausführen von `/debug` in der Sitzung
* stderr, in nicht-interaktiven Sitzungen, die mit `-p` gestartet werden

<h4 id="refresh-behavior">
  Aktualisierungsverhalten
</h4>

Das Headers-Helper-Skript wird beim Start und danach regelmäßig ausgeführt, um Token-Aktualisierung zu unterstützen. Standardmäßig wird das Skript alle 29 Minuten ausgeführt. Passen Sie das Intervall mit der Umgebungsvariable `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS` an.

<h3 id="multi-team-organization-support">
  Unterstützung für Multi-Team-Organisationen
</h3>

Organisationen mit mehreren Teams oder Abteilungen können benutzerdefinierte Attribute hinzufügen, um zwischen verschiedenen Gruppen zu unterscheiden, indem sie die Umgebungsvariable `OTEL_RESOURCE_ATTRIBUTES` verwenden:

```bash theme={null}
# Benutzerdefinierte Attribute für Team-Identifikation hinzufügen
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

Diese benutzerdefinierten Attribute werden in alle Metriken und Ereignisse einbezogen, sodass Sie:

* Metriken nach Team oder Abteilung filtern können
* Kosten pro Kostenstelle verfolgen können
* Team-spezifische Dashboards erstellen können
* Warnungen für bestimmte Teams einrichten können

Claude Code fügt diese Werte als Attribute auf jedem Metrik-Datenpunkt und Ereignisdatensatz an, zusätzlich zum Senden im OTLP-Ressourcenblock. Da die meisten Metriken-Backends Datenpunkt-Attribute als abfragbare Labels verfügbar machen, können Sie Metriken direkt nach Ihren benutzerdefinierten Schlüsseln gruppieren und filtern. Benutzerdefinierte Schlüssel überschreiben niemals die [Standardattribute](#standard-attributes) wie `user.id` oder `session.id`: Wenn ein Schlüssel kollidiert, behält Claude Code den integrierten Wert.

Jeder benutzerdefinierte Schlüssel wird zu einem Label auf jeder Metrik-Serie, daher erhöhen hochkardinalige Werte die Speicherkosten in Ihrem Metriken-Backend. Um benutzerdefinierte Attribute nur im Ressourcenblock zu senden und sie von Datenpunkt-Labels auszulassen, setzen Sie `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`. Siehe [Metriken-Kardinalitätskontrolle](#metrics-cardinality-control).

<Warning>
  Die Umgebungsvariable `OTEL_RESOURCE_ATTRIBUTES` verwendet kommagetrennte Schlüssel=Wert-Paare mit strikten Formatierungsanforderungen:

  * **Keine Leerzeichen erlaubt**: Werte dürfen keine Leerzeichen enthalten. Zum Beispiel ist `user.organizationName=My Company` ungültig
  * **Format**: Muss kommagetrennte Schlüssel=Wert-Paare sein: `key1=value1,key2=value2`
  * **Zulässige Zeichen**: Nur US-ASCII-Zeichen ohne Steuerzeichen, Leerzeichen, doppelte Anführungszeichen, Kommas, Semikola und Backslashes
  * **Sonderzeichen**: Zeichen außerhalb des zulässigen Bereichs müssen prozentcodiert sein

  Für einen Wert, der ein Leerzeichen benötigen würde, verwenden Sie stattdessen Unterstriche oder camelCase. Die folgenden Beispiele setzen `org.name` mit jeder Form:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  Sie können jedes Zeichen prozentcodieren, nicht nur die ausgeschlossenen. Dieses Beispiel codiert sowohl das Leerzeichen als auch den Apostroph:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  Das Einschließen von Werten in Anführungszeichen entkommt keine Leerzeichen. Zum Beispiel führt `org.name="My Company"` zum Literalwert `"My Company"` mit den Anführungszeichen enthalten, nicht zu `My Company`.
</Warning>

<h3 id="example-configurations">
  Beispielkonfigurationen
</h3>

Setzen Sie diese Umgebungsvariablen vor dem Ausführen von `claude`. Jeder Block zeigt eine vollständige Konfiguration für einen anderen Exporter oder ein anderes Bereitstellungsszenario:

```bash theme={null}
# Console-Debugging (1-Sekunden-Intervalle)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000

# OTLP/gRPC
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Prometheus
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus

# Mehrere Exporter
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# Unterschiedliche Endpunkte/Backends für Metriken und Logs
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# Nur Metriken (keine Ereignisse/Logs)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Nur Ereignisse/Logs (keine Metriken)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Verfügbare Metriken und Ereignisse
</h2>

<h3 id="standard-attributes">
  Standardattribute
</h3>

Alle Metriken und Ereignisse teilen diese Standardattribute:

| Attribut                                 | Beschreibung                                                                                                                                                                                                                                                                                      | Gesteuert durch                                             |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| `session.id`                             | Eindeutige Sitzungskennung                                                                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_SESSION_ID` (Standard: true)          |
| `app.version`                            | Aktuelle Claude Code-Version                                                                                                                                                                                                                                                                      | `OTEL_METRICS_INCLUDE_VERSION` (Standard: false)            |
| `app.entrypoint`                         | Wie die Sitzung gestartet wurde, z. B. `cli`, `sdk-cli`, `sdk-ts`, `sdk-py` oder `claude-vscode`                                                                                                                                                                                                  | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (Standard: false)         |
| `organization.id`                        | Organisations-UUID (wenn authentifiziert)                                                                                                                                                                                                                                                         | Immer enthalten, wenn verfügbar                             |
| `user.account_uuid`                      | Konto-UUID (wenn authentifiziert)                                                                                                                                                                                                                                                                 | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (Standard: true)        |
| `user.account_id`                        | Konto-ID im getaggten Format, das Anthropic-Admin-APIs entspricht (wenn authentifiziert), z. B. `user_01BWBeN28...`                                                                                                                                                                               | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (Standard: true)        |
| `user.id`                                | Zufällige anonyme Kennung, die beim ersten Ausführen generiert und in `~/.claude.json` gespeichert wird. Sie enthält keine persönlichen Informationen und wird nicht von Ihrem Claude-Konto abgeleitet. Das Löschen der Datei erzeugt beim nächsten Ausführen einen neuen, nicht verwandten Wert. | Immer enthalten                                             |
| `user.email`                             | E-Mail-Adresse des Benutzers (wenn über OAuth authentifiziert)                                                                                                                                                                                                                                    | Immer enthalten, wenn verfügbar                             |
| `terminal.type`                          | Terminal-Typ, z. B. `iTerm.app`, `vscode`, `cursor` oder `tmux`                                                                                                                                                                                                                                   | Immer enthalten, wenn erkannt                               |
| Schlüssel aus `OTEL_RESOURCE_ATTRIBUTES` | Benutzerdefinierte Attribute, die Sie festlegen, z. B. `department` oder `team.id`. Siehe [Multi-Team-Organisationsunterstützung](#multi-team-organization-support)                                                                                                                               | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (Standard: true) |

Wenn Claude Code bei einem [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) angemeldet ist, versieht die CLI Exporte mit der authentifizierten Identität aus der Gateway-Sitzung: `user.id` ist das IdP-Subjekt statt einer anonymen Installationskennung, `user.email` ist die angemeldete E-Mail, und `user.groups` enthält die IdP-Gruppenmitgliedschaft als kommagetrennte Zeichenkette. Jeder Export trägt auch `identity.source: gateway-oidc`. Die Gateway-Identität wird zuletzt angewendet, daher werden `user.*` und `identity.*` Schlüssel, die über `OTEL_RESOURCE_ATTRIBUTES` gesetzt werden, bei Gateway-Sitzungen ignoriert.

Ereignisse enthalten zusätzlich die folgenden Attribute. Diese werden niemals an Metriken angehängt, da sie zu unbegrenzter Kardinalität führen würden:

* `prompt.id`: UUID, die einen Benutzer-Prompt mit allen nachfolgenden Ereignissen bis zum nächsten Prompt korreliert. Siehe [Ereigniskorrelationsattribute](#event-correlation-attributes).
* `workspace.host_paths`: Host-Workspace-Verzeichnisse, die in der Desktop-App ausgewählt wurden, als String-Array
* `workflow.run_id`: Lauf-Kennung mit dem Präfix `wf_` auf der API und Tool-Ereignisse, die von Agents ausgegeben werden, die zu einem [Workflow](/docs/de/workflows) Tool-Lauf gehören. Das Filtern von Ereignissen nach einer `workflow.run_id` rekonstruiert die API-Anfragen und Tool-Ergebnisse dieses Laufs. Die Kennung umfasst die Agents, die das Workflow-Skript erzeugt, und alle Agents, die diese wiederum erzeugen, z. B. Skill-Aufrufe. Sie entspricht der Lauf-Kennung, die im Workflow-Tool-Ergebnis gemeldet wird. Nicht vorhanden bei allen anderen Ereignissen. Erfordert Claude Code v2.1.202 oder später
* `workflow.name`: Name des Workflows, das `meta.name` des Skripts, ausgegeben zusammen mit `workflow.run_id`. Integrierte Workflow-Namen erscheinen wörtlich, wenn der Lauf das unmodifizierte integrierte Skript ausführt. Benutzerdefinierte Namen, einschließlich bearbeiteter Kopien integrierter Skripte, werden durch `custom` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt. Erfordert Claude Code v2.1.202 oder später

<h3 id="metrics">
  Metriken
</h3>

Claude Code exportiert die folgenden Metriken:

| Metrikname                            | Beschreibung                                                          | Einheit |
| ------------------------------------- | --------------------------------------------------------------------- | ------- |
| `claude_code.session.count`           | Anzahl der gestarteten CLI-Sitzungen                                  | count   |
| `claude_code.lines_of_code.count`     | Anzahl der geänderten Codezeilen                                      | count   |
| `claude_code.pull_request.count`      | Anzahl der erstellten Pull Requests                                   | count   |
| `claude_code.commit.count`            | Anzahl der erstellten Git-Commits                                     | count   |
| `claude_code.cost.usage`              | Kosten der Claude Code-Sitzung                                        | USD     |
| `claude_code.token.usage`             | Anzahl der verwendeten Token                                          | tokens  |
| `claude_code.code_edit_tool.decision` | Anzahl der Entscheidungen zur Berechtigung des Code-Bearbeitungstools | count   |
| `claude_code.active_time.total`       | Gesamte aktive Zeit in Sekunden                                       | s       |

<h3 id="metric-details">
  Metrik-Details
</h3>

Jede Metrik enthält die oben aufgeführten Standardattribute. Metriken mit zusätzlichen kontextspezifischen Attributen werden nachfolgend vermerkt.

<h4 id="session-counter">
  Sitzungszähler
</h4>

Wird zu Beginn jeder Sitzung erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `start_type`: Wie die Sitzung gestartet wurde. Einer von `"fresh"`, `"resume"`, `"continue"` oder `"agents_view"`. Der Wert `"agents_view"` identifiziert den `claude agents` Dashboard-Prozess, eine von Benutzern gestartete lokale Benutzeroberfläche statt einer Konversationssitzung. Filtern Sie nach diesem Wert, um UI-Prozessstart von Konversationssitzungen in Ihren Dashboards zu trennen.

<h4 id="lines-of-code-counter">
  Codezeilen-Zähler
</h4>

Wird erhöht, wenn Code hinzugefügt oder entfernt wird.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Modellkennung für das Modell, das die Änderung vorgenommen hat (z. B. "claude-sonnet-5")

<h4 id="pull-request-counter">
  Pull-Request-Zähler
</h4>

Wird erhöht, wenn Claude Code einen Pull Request oder Merge Request über einen Shell-Befehl oder ein MCP-Tool erstellt.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)

<h4 id="commit-counter">
  Commit-Zähler
</h4>

Wird erhöht, wenn Git-Commits über Claude Code erstellt werden.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)

<h4 id="cost-counter">
  Kostenzähler
</h4>

Wird nach jeder API-Anfrage erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `model`: Modellkennung (z. B. "claude-sonnet-5")
* `query_source`: Kategorie des Subsystems, das die Anfrage gestellt hat. Einer von `"main"`, `"subagent"` oder `"auxiliary"`
* `speed`: `"fast"`, wenn die Anfrage den schnellen Modus verwendet hat. Andernfalls nicht vorhanden
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird: `"low"`, `"medium"`, `"high"`, `"xhigh"` oder `"max"`. Nicht vorhanden, wenn das Modell Anstrengung nicht unterstützt.
* `agent.name`: Subagent-Typ, der die Anfrage gestellt hat. Integrierte Agent-Namen und Agents aus offiziellen Marketplace-Plugins werden wörtlich angezeigt. Andere benutzerdefinierte Agent-Namen werden durch `"custom"` ersetzt. Nicht vorhanden, wenn die Anfrage nicht von einem benannten Subagent-Typ gestellt wurde.
* `skill.name`: Skill, der für die Anfrage aktiv ist, gesetzt durch das Skill-Tool, einen `/` Befehl oder geerbt von einem erzeugten Subagent. Integrierte, gebündelte, benutzerdefinierte und offizielle Marketplace-Plugin-Skill-Namen werden wörtlich angezeigt. Drittanbieter-Plugin-Skill-Namen werden durch `"third-party"` ersetzt. Nicht vorhanden, wenn kein Skill aktiv ist.
* `plugin.name`: Besitzendes Plugin, wenn der aktive Skill oder Subagent von einem Plugin bereitgestellt wird. Offizielle Marketplace-Plugin-Namen werden wörtlich angezeigt. Drittanbieter-Plugin-Namen werden durch `"third-party"` ersetzt. Nicht vorhanden, wenn weder der Skill noch der Subagent ein besitzendes Plugin hat.
* `marketplace.name`: Marketplace, von dem das besitzende Plugin installiert wurde. Nur für offizielle Marketplace-Plugins ausgegeben. Andernfalls nicht vorhanden.
* `mcp_server.name`: MCP-Server, dessen Tool in der Runde ausgeführt wurde, die diese Anfrage erzeugt hat. Integrierte, claude.ai-proxied und offizielle Registry-Server-Namen werden wörtlich angezeigt. Benutzerkonfigurierte Server-Namen werden durch `"custom"` ersetzt. Nicht vorhanden, wenn kein MCP-Tool ausgeführt wurde.
* `mcp_tool.name`: MCP-Tool, das in der Runde ausgeführt wurde, die diese Anfrage erzeugt hat, mit der gleichen Schwärzung wie `mcp_server.name`. Nicht vorhanden, wenn kein MCP-Tool ausgeführt wurde.

<h4 id="token-counter">
  Token-Zähler
</h4>

Wird nach jeder API-Anfrage erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Modellkennung (z. B. "claude-sonnet-5")
* `query_source`: Kategorie des Subsystems, das die Anfrage gestellt hat. Einer von `"main"`, `"subagent"` oder `"auxiliary"`
* `speed`: `"fast"`, wenn die Anfrage den schnellen Modus verwendet hat. Andernfalls nicht vorhanden
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird. Siehe [Kostenzähler](#cost-counter) für Details.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Schwärzungsverhalten.

<h4 id="code-edit-tool-decision-counter">
  Code-Edit-Tool-Entscheidungszähler
</h4>

Wird erhöht, wenn der Benutzer die Verwendung des Edit-, Write- oder NotebookEdit-Tools akzeptiert oder ablehnt.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `tool_name`: Tool-Name (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Benutzerentscheidung (`"accept"`, `"reject"`)
* `source`: Entscheidungsquelle. Einer von `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"` oder `"user_reject"`. Siehe das [Tool-Entscheidungs-Ereignis](#tool-decision-event) für die Bedeutung jedes Wertes.
* `language`: Programmiersprache der bearbeiteten Datei, z. B. `"TypeScript"`, `"Python"`, `"JavaScript"` oder `"Markdown"`. Gibt `"unknown"` für nicht erkannte Dateierweiterungen zurück.

<h4 id="active-time-counter">
  Aktive-Zeit-Zähler
</h4>

Verfolgt die tatsächliche Zeit, die aktiv Claude Code verwendet wird, ohne Leerlaufzeit. Diese Metrik wird während Benutzerinteraktionen (Eingabe, Lesen von Antworten) und während CLI-Verarbeitung (Tool-Ausführung, KI-Antwortgenerierung) erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `type`: `"user"` für Tastaturinteraktionen, `"cli"` für Tool-Ausführung und KI-Antworten

<h3 id="events">
  Ereignisse
</h3>

Claude Code exportiert die folgenden Ereignisse über OpenTelemetry Logs/Events (wenn `OTEL_LOGS_EXPORTER` konfiguriert ist):

<h4 id="event-correlation-attributes">
  Ereigniskorrelationsattribute
</h4>

Wenn ein Benutzer einen Prompt einreicht, kann Claude Code mehrere API-Aufrufe tätigen und mehrere Tools ausführen. Das Attribut `prompt.id` ermöglicht es Ihnen, alle diese Ereignisse an den einzelnen Prompt zu binden, der sie ausgelöst hat.

| Attribut    | Beschreibung                                                                                                                 |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `prompt.id` | UUID v4-Kennung, die alle Ereignisse verknüpft, die während der Verarbeitung eines einzelnen Benutzer-Prompts erzeugt werden |

Um alle Aktivitäten zu verfolgen, die durch einen einzelnen Prompt ausgelöst werden, filtern Sie Ihre Ereignisse nach einem bestimmten `prompt.id`-Wert. Dies gibt das user\_prompt-Ereignis, alle api\_request-Ereignisse und alle tool\_result-Ereignisse zurück, die während der Verarbeitung dieses Prompts aufgetreten sind.

<Note>
  `prompt.id` ist absichtlich aus Metriken ausgeschlossen, da jeder Prompt eine eindeutige ID generiert, was zu einer ständig wachsenden Anzahl von Zeitreihen führen würde. Verwenden Sie es nur für Ereignisanalyse und Audit-Trails.
</Note>

<h4 id="user-prompt-event">
  Benutzer-Prompt-Ereignis
</h4>

Protokolliert, wenn ein Benutzer einen Prompt einreicht.

**Ereignisname**: `claude_code.user_prompt`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `prompt_length`: Länge des Prompts
* `prompt`: Prompt-Inhalt. Standardmäßig geschwärzt. Setzen Sie `OTEL_LOG_USER_PROMPTS=1`, um ihn einzubeziehen
* `command_name`: Befehlsname, wenn der Prompt einen aufruft. Integrierte und gebündelte Befehlsnamen wie `compact` oder `debug` werden wie geschrieben ausgegeben; Aliase wie `reset` werden wie eingegeben ausgegeben, nicht der kanonische Name. Benutzerdefinierte, Plugin- und MCP-Befehlsnamen werden zu `custom` oder `mcp` zusammengefasst, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt
* `command_source`: Ursprung des Befehls, wenn vorhanden: `builtin`, `custom` oder `mcp`. Von Plugins bereitgestellte Befehle werden als `custom` gemeldet

<h4 id="assistant-response-event">
  Assistent-Antwort-Ereignis
</h4>

Protokolliert nach jeder API-Anfrage, die Textinhalte vom Modell zurückgibt. Nur die Textblöcke der Antwort sind enthalten; Thinking-Blöcke und Tool-Use-Blöcke sind ausgeschlossen. Erfordert Claude Code v2.1.193 oder später.

**Ereignisname**: `claude_code.assistant_response`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `response_length`: Länge des Antworttexts in Zeichen
* `response`: Antworttext, gekürzt bei 60 KB. Standardmäßig auf `<REDACTED>` geschwärzt. Setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=1`, um ihn einzubeziehen. Wenn `OTEL_LOG_ASSISTANT_RESPONSES` nicht gesetzt ist, steuert `OTEL_LOG_USER_PROMPTS` es stattdessen, also setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=0`, um Antworten geschwärzt zu halten, während Prompt-Protokollierung aktiv ist
* `model`: Modellkennung (z. B. "claude-sonnet-5")
* `request_id`: Anthropic API-Anfrage-ID aus dem Response-Header `request-id`. Nur vorhanden, wenn die API eine zurückgibt
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name

<h4 id="tool-result-event">
  Tool-Ergebnis-Ereignis
</h4>

Protokolliert, wenn ein Tool die Ausführung abgeschlossen hat. Nicht ausgegeben, wenn der Tool-Aufruf abgelehnt wurde; siehe das [Tool-Entscheidungs-Ereignis](#tool-decision-event) für Ablehnungen.

**Ereignisname**: `claude_code.tool_result`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `tool_name`: Name des Tools
* `tool_use_id`: Eindeutige Kennung für diese Tool-Invokation. Entspricht der `tool_use_id`, die an Hooks übergeben wird, und ermöglicht die Korrelation zwischen OTel-Ereignissen und Hook-erfassten Daten.
* `success`: `"true"` oder `"false"`
* `duration_ms`: Ausführungszeit in Millisekunden
* `error_type`: Fehler-Kategoriezeichenkette, wenn das Tool fehlgeschlagen ist, z. B. `"Error:ENOENT"` oder `"ShellError"`
* `error` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Vollständige Fehlermeldung, wenn das Tool fehlgeschlagen ist
* `decision_type`: Immer `"accept"`, da dieses Ereignis nur ausgegeben wird, nachdem das Tool ausgeführt wurde. Abgelehnte Aufrufe erzeugen kein Tool-Ergebnis
* `decision_source`: Entscheidungsquelle. Einer von `"config"`, `"hook"`, `"user_permanent"` oder `"user_temporary"`. Siehe das [Tool-Entscheidungs-Ereignis](#tool-decision-event) für die Bedeutung jedes Wertes. Die nur-Ablehnung-Quellen `"user_abort"` und `"user_reject"` erscheinen niemals auf diesem Ereignis.
* `tool_input_size_bytes`: Größe der JSON-serialisierten Tool-Eingabe in Bytes
* `tool_result_size_bytes`: Größe des Tool-Ergebnisses in Bytes
* `mcp_server_scope`: MCP-Server-Scope-Kennung (für MCP-Tools)
* `tool_parameters` (wenn `OTEL_LOG_TOOL_DETAILS=1`): JSON-Zeichenkette mit Tool-spezifischen Parametern:
  * Für Bash-Tool: enthält `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox` und `git_commit_id` (der Commit-SHA, wenn ein `git commit`-Befehl erfolgreich ist)
  * Für WorkspaceBash-Tool: enthält `bash_command`, `full_command`, `timeout`
  * Für MCP-Tools: enthält `mcp_server_name`, `mcp_tool_name`
  * Für Skill-Tool: enthält `skill_name`
  * Für Agent-Tool oder Legacy-Task-Tool: enthält `subagent_type`
* `tool_input` (wenn `OTEL_LOG_TOOL_DETAILS=1`): JSON-serialisierte Tool-Argumente. Einzelne Werte über 512 Zeichen werden gekürzt, und die gesamte Nutzlast ist auf etwa 4 K Zeichen begrenzt. Gilt für alle Tools einschließlich MCP-Tools.

<h4 id="api-request-event">
  API-Anfrage-Ereignis
</h4>

Protokolliert für jede API-Anfrage an Claude.

**Ereignisname**: `claude_code.api_request`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `model`: Verwendetes Modell (z. B. "claude-sonnet-5")
* `cost_usd`: Geschätzte Kosten in USD
* `duration_ms`: Anfragedauer in Millisekunden
* `input_tokens`: Anzahl der Eingabe-Token
* `output_tokens`: Anzahl der Ausgabe-Token
* `cache_read_tokens`: Anzahl der aus dem Cache gelesenen Token
* `cache_creation_tokens`: Anzahl der Token, die für die Cache-Erstellung verwendet werden
* `request_id`: Anthropic API-Anfrage-ID aus dem Response-Header `request-id`, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `speed`: `"fast"` oder `"normal"`, was angibt, ob der schnelle Modus aktiv war
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird: `"low"`, `"medium"`, `"high"`, `"xhigh"` oder `"max"`. Nicht vorhanden, wenn das Modell Anstrengung nicht unterstützt.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Schwärzungsverhalten.

<h4 id="api-error-event">
  API-Fehler-Ereignis
</h4>

Protokolliert, wenn eine API-Anfrage an Claude fehlschlägt.

**Ereignisname**: `claude_code.api_error`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `model`: Verwendetes Modell (z. B. "claude-sonnet-5")
* `error`: Fehlermeldung
* `status_code`: HTTP-Statuscode als Zahl. Nicht vorhanden für Nicht-HTTP-Fehler wie Verbindungsfehler.
* `duration_ms`: Anfragedauer in Millisekunden
* `attempt`: Gesamtzahl der Versuche, einschließlich der ursprünglichen Anfrage (`1` bedeutet, dass keine Wiederholungen aufgetreten sind)
* `request_id`: Anthropic API-Anfrage-ID aus dem Response-Header `request-id`, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `speed`: `"fast"` oder `"normal"`, was angibt, ob der schnelle Modus aktiv war
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird. Nicht vorhanden, wenn das Modell Anstrengung nicht unterstützt.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Schwärzungsverhalten.

<h4 id="api-refusal-event">
  API-Verweigerung-Ereignis
</h4>

Protokolliert, wenn eine API-Anfrage `stop_reason: "refusal"` zurückgibt. Verweigerungen kommen in einem erfolgreichen Response-Stream an, nicht als HTTP-Fehler, daher wird das `api_error`-Ereignis nicht für sie ausgelöst. Dieses Ereignis ermöglicht es Ihnen, die Verweigerungshäufigkeit zu verfolgen und Verweigerungen nach den gleichen Attributen wie `api_request` und `api_error` zu gruppieren.

**Ereignisname**: `claude_code.api_refusal`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `model`: Modellkennung aus der Anfrage
* `request_id`: Anthropic API-Anfrage-ID aus dem Response-Header `request-id`, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name. Siehe [`api_request`](#api-request-event) für Definitionen.
* `speed`: Entweder `"fast"`, wenn [Schneller Modus](/docs/de/fast-mode) aktiv ist, oder `"normal"`
* `attempt`: Wiederholungsversuch-Nummer. Der erste Versuch ist `1`.
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird. Nicht vorhanden, wenn das Modell Anstrengung nicht unterstützt.
* `server_fallback_hop`: `true`, wenn das Server-seitige Modell-Fallback der API diesen Verweigerung bereits auf einem anderen Modell erneut versucht hat, sodass der Benutzer diese bestimmte Verweigerung nicht sah. `false`, wenn die Anfrage in einer Verweigerung endete. Eine einzelne Runde kann sowohl ein `true` Hop-Ereignis als auch ein späteres `false` Finales Ereignis ausgeben, wenn das Fallback-Modell auch verweigert.
* `has_category`: `true`, wenn die API-Antwort eine `stop_details.category` von `"cyber"`, `"bio"`, `"frontier_llm"` oder `"reasoning_extraction"` trug. `false`, wenn die Antwort keine Kategorie oder einen Wert außerhalb dieses Satzes trug. Nicht vorhanden, wenn `server_fallback_hop` `true` ist, da Hop-Blöcke keine `stop_details` tragen.
* `has_explanation`: `true`, wenn die API-Antwort eine `stop_details.explanation` trug, andernfalls `false`. Nicht vorhanden, wenn `server_fallback_hop` `true` ist.
* `category`: Der `stop_details.category`-Wert aus der API-Antwort. Einer von `"cyber"`, `"bio"`, `"frontier_llm"` oder `"reasoning_extraction"`. Nur vorhanden, wenn `OTEL_LOG_TOOL_DETAILS=1` gesetzt ist und `has_category` `true` ist.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Schwärzungsverhalten.

<h4 id="api-request-body-event">
  API-Anfrage-Text-Ereignis
</h4>

Protokolliert für jeden API-Anfrage-Versuch, wenn `OTEL_LOG_RAW_API_BODIES` gesetzt ist. Ein Ereignis wird pro Versuch ausgegeben, daher erzeugen Wiederholungen mit angepassten Parametern jeweils ihr eigenes Ereignis.

**Ereignisname**: `claude_code.api_request_body`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `body`: JSON-serialisierte Messages API-Anfrageparameter (Systemprompt, Nachrichten, Tools usw.), gekürzt bei 60 KB. Extended-Thinking-Inhalte in vorherigen Assistent-Durchgängen werden geschwärzt. Nur im Inline-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Absoluter Pfad zu einer `<dir>/<uuid>.request.json` Datei, die den ungekürzte Text enthält. Nur im Datei-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Ungekürzte Text-Länge. UTF-8-Bytes, wenn `OTEL_LOG_RAW_API_BODIES=file:<dir>`, oder UTF-16-Code-Einheiten, wenn `=1`
* `body_truncated`: `"true"`, wenn Inline-Kürzung aufgetreten ist. Nicht vorhanden im Datei-Modus und wenn keine Kürzung aufgetreten ist.
* `model`: Modellkennung aus den Anfrageparametern
* `query_source`: Subsystem, das die Anfrage gestellt hat (z. B. `"compact"`)

<h4 id="api-response-body-event">
  API-Antwort-Text-Ereignis
</h4>

Protokolliert für jede erfolgreiche API-Antwort, wenn `OTEL_LOG_RAW_API_BODIES` gesetzt ist.

**Ereignisname**: `claude_code.api_response_body`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `body`: JSON-serialisierte Messages API-Antwort (id, Inhaltsblöcke, Nutzung, Stoppgrund), gekürzt bei 60 KB. Extended-Thinking-Inhalte werden geschwärzt. Nur im Inline-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Absoluter Pfad zu einer `<dir>/<request_id>.response.json` Datei, die den ungekürzte Text enthält. Nur im Datei-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Ungekürzte Text-Länge. UTF-8-Bytes, wenn `OTEL_LOG_RAW_API_BODIES=file:<dir>`, oder UTF-16-Code-Einheiten, wenn `=1`
* `body_truncated`: `"true"`, wenn Inline-Kürzung aufgetreten ist. Nicht vorhanden im Datei-Modus und wenn keine Kürzung aufgetreten ist.
* `model`: Modellkennung
* `query_source`: Subsystem, das die Anfrage gestellt hat
* `request_id`: Anthropic API-Anfrage-ID aus dem Response-Header `request-id`, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.

<h4 id="tool-decision-event">
  Tool-Entscheidungs-Ereignis
</h4>

Protokolliert, wenn eine Tool-Berechtigungsentscheidung getroffen wird (akzeptieren/ablehnen).

**Ereignisname**: `claude_code.tool_decision`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `tool_name`: Name des Tools (z. B. "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Eindeutige Kennung für diese Tool-Invokation. Entspricht der `tool_use_id`, die an Hooks übergeben wird, und ermöglicht die Korrelation zwischen OTel-Ereignissen und Hook-erfassten Daten.
* `decision`: Entweder `"accept"` oder `"reject"`
* `source`: Entscheidungsquelle:
  * `"config"`: Automatisch entschieden, ohne Aufforderung, basierend auf Projekteinstellungen, Zulassungsregeln in den persönlichen Einstellungen des Benutzers, verwalteter Unternehmensrichtlinie, `--allowedTools` oder `--disallowedTools` Flags, dem aktiven Berechtigungsmodus, einer sitzungsbegrenzten Zulassung aus einem früheren Prompt in der gleichen interaktiven CLI-Sitzung oder weil das Tool inhärent sicher ist. Das Ereignis gibt nicht an, welche dieser Quellen übereinstimmte.
  * `"hook"`: Ein `PreToolUse` oder `PermissionRequest` Hook hat die Entscheidung zurückgegeben.
  * `"user_permanent"`: Wird ausgegeben, wenn der Benutzer "Ja, und nicht mehr fragen für ..." bei einer Berechtigungsaufforderung wählte, was eine Zulassungsregel in seinen persönlichen Einstellungen speichert. In der interaktiven CLI wird dies nur für diese Wahl selbst ausgegeben; spätere Aufrufe, die der gespeicherten Regel entsprechen, geben stattdessen `"config"` aus. In Agent SDK oder nicht-interaktiven `-p` Sitzungen geben sowohl die ursprüngliche Wahl als auch spätere Regelübereinstimmungen `"user_permanent"` aus. Wird als Akzeptanz behandelt.
  * `"user_temporary"`: Wird ausgegeben, wenn der Benutzer "Ja" bei einer Berechtigungsaufforderung wählte, oder eine der Optionen "... während dieser Sitzung" bei einer Dateibearbeitungs- oder Leseanforderung wählte. In der interaktiven CLI wird dies nur für die Wahl selbst ausgegeben; spätere Aufrufe, die durch diese sitzungsbegrenzte Zulassung zulässig sind, geben stattdessen `"config"` aus. In Agent SDK oder nicht-interaktiven `-p` Sitzungen geben sowohl die Wahl als auch spätere Übereinstimmungen `"user_temporary"` aus. Wird als Akzeptanz behandelt.
  * `"user_abort"`: Wird ausgegeben, wenn der Benutzer die Berechtigungsaufforderung geschlossen hat, ohne zu antworten. Wird als Ablehnung behandelt.
  * `"user_reject"`: Wird ausgegeben, wenn der Benutzer "Nein" wählte, wenn aufgefordert. In der interaktiven CLI wird dies nur für diese Wahl selbst ausgegeben; Aufrufe, die einer Ablehnungsregel in den persönlichen Einstellungen des Benutzers entsprechen, geben stattdessen `"config"` aus. In Agent SDK oder nicht-interaktiven `-p` Sitzungen geben Aufrufe, die einer Ablehnungsregel in persönlichen Einstellungen entsprechen, `"user_reject"` aus. Wird als Ablehnung behandelt.
* `tool_parameters` (wenn `OTEL_LOG_TOOL_DETAILS=1`): JSON-Zeichenkette mit Tool-spezifischen Parametern. Gleiche Form wie das [Tool-Ergebnis-Ereignis](#tool-result-event), minus Post-Ausführungs-Felder wie `git_commit_id`. Werte können sich von `tool_result` für einen akzeptierten Aufruf unterscheiden, wenn die Berechtigungsentscheidung die Tool-Eingabe über `updatedInput` umschreibt. Verwenden Sie dieses Attribut, um zu sehen, welcher Befehl abgelehnt wurde, wenn `decision` `"reject"` ist.
  * Für Bash-Tool: enthält `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`
  * Für WorkspaceBash-Tool: enthält `bash_command`, `full_command`, `timeout`
  * Für MCP-Tools: enthält `mcp_server_name`, `mcp_tool_name`
  * Für Skill-Tool: enthält `skill_name`
  * Für Agent-Tool oder Legacy-Task-Tool: enthält `subagent_type`

<h4 id="permission-mode-changed-event">
  Berechtigungsmodus-Änderungs-Ereignis
</h4>

Protokolliert, wenn sich der Berechtigungsmodus ändert, z. B. durch Shift+Tab-Zyklus, Beendigung des Plan-Modus oder eine Auto-Modus-Gate-Prüfung.

**Ereignisname**: `claude_code.permission_mode_changed`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `from_mode`: Der vorherige Berechtigungsmodus, z. B. `"default"`, `"plan"`, `"acceptEdits"`, `"auto"` oder `"bypassPermissions"`
* `to_mode`: Der neue Berechtigungsmodus
* `trigger`: Was die Änderung verursacht hat. Einer von `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"` oder `"auto_opt_in"`. Nicht vorhanden, wenn der Übergang vom SDK oder Bridge stammt

<h4 id="auth-event">
  Auth-Ereignis
</h4>

Protokolliert, wenn `/login` oder `/logout` abgeschlossen ist.

**Ereignisname**: `claude_code.auth`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `action`: `"login"` oder `"logout"`
* `success`: `"true"` oder `"false"`
* `auth_method`: Authentifizierungsmethode, z. B. `"oauth"`
* `error_category`: Kategorische Fehlerart, wenn die Aktion fehlgeschlagen ist. Die rohe Fehlermeldung ist nie enthalten
* `status_code`: HTTP-Statuscode als Zeichenkette, wenn die Aktion mit einem HTTP-Fehler fehlgeschlagen ist

<h4 id="mcp-server-connection-event">
  MCP-Server-Verbindungs-Ereignis
</h4>

Protokolliert, wenn ein MCP-Server verbunden wird, getrennt wird oder keine Verbindung herstellen kann.

**Ereignisname**: `claude_code.mcp_server_connection`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `status`: `"connected"`, `"failed"` oder `"disconnected"`
* `transport_type`: Server-Transport, z. B. `"stdio"`, `"sse"` oder `"http"`
* `server_scope`: Bereich, in dem der Server konfiguriert ist, z. B. `"user"`, `"project"` oder `"local"`
* `duration_ms`: Verbindungsversuch-Dauer in Millisekunden
* `error_code`: Fehlercode, wenn die Verbindung fehlgeschlagen ist
* `is_plugin`: `true`, wenn der Server von einem Plugin bereitgestellt wird, `false` andernfalls
* `plugin_id_hash` (wenn `is_plugin` `true` ist): Stabiler Hash des Plugin-Namens und des Marketplace, zum Gruppieren von Ereignissen nach Plugin, ohne den Namen offenzulegen
* `plugin.name` (wenn `is_plugin` `true` ist): Name des Plugins, das den Server bereitstellt. Für Drittanbieter-Plugins ist dies die Zeichenkette `"third-party"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`; dies schützt Drittanbieter-Plugin-Namen davor, standardmäßig in Protokollen zu erscheinen. Plugins aus offiziellen Anthropic-Quellen werden immer anhand des Namens identifiziert. Die Attribute `plugin_id_hash` und `plugin.name` fließen zu Ihrem eigenen Monitoring-Backend und werden nicht an Anthropic gesendet
* `server_name` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Konfigurierter Server-Name
* `error` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Vollständige Fehlermeldung, wenn die Verbindung fehlgeschlagen ist

<h4 id="internal-error-event">
  Interner Fehler-Ereignis
</h4>

Protokolliert, wenn Claude Code einen unerwarteten internen Fehler abfängt. Nur der Fehlerklassenname und ein errno-ähnlicher Code werden aufgezeichnet. Die Fehlermeldung und Stack-Trace sind nie enthalten. Dieses Ereignis wird nicht ausgegeben, wenn gegen Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry ausgeführt wird, oder wenn `DISABLE_ERROR_REPORTING` gesetzt ist.

**Ereignisname**: `claude_code.internal_error`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `error_name`: Fehlerklassenname, z. B. `"TypeError"` oder `"SyntaxError"`
* `error_code`: Node.js errno-Code wie `"ENOENT"`, wenn auf dem Fehler vorhanden

<h4 id="plugin-installed-event">
  Plugin-Installiert-Ereignis
</h4>

Protokolliert, wenn ein Plugin die Installation abgeschlossen hat, sowohl vom `claude plugin install` CLI-Befehl als auch von der interaktiven `/plugin` UI.

**Ereignisname**: `claude_code.plugin_installed`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `marketplace.is_official`: `"true"`, wenn der Marketplace ein offizieller Anthropic-Marketplace ist, `"false"` andernfalls
* `install.trigger`: `"cli"` oder `"ui"`
* `plugin.name`: Name des installierten Plugins. Für Drittanbieter-Marketplaces ist dies nur enthalten, wenn `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Plugin-Version, wenn in der Marketplace-Eintrag deklariert. Für Drittanbieter-Marketplaces ist dies nur enthalten, wenn `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace, von dem das Plugin installiert wurde. Für Drittanbieter-Marketplaces ist dies nur enthalten, wenn `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Plugin-Geladen-Ereignis
</h4>

Protokolliert einmal pro aktiviertem Plugin beim Sitzungsstart. Verwenden Sie dieses Ereignis, um zu inventarisieren, welche Plugins über Ihre gesamte Flotte hinweg aktiv sind, als Ergänzung zu `plugin_installed`, das die Installationsaktion selbst aufzeichnet.

**Ereignisname**: `claude_code.plugin_loaded`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `plugin.name`: Name des Plugins. Für Plugins außerhalb des offiziellen Marketplace und des integrierten Bundles ist der Wert `"third-party"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace, von dem das Plugin installiert wurde, wenn bekannt. Auf `"third-party"` unter der gleichen Bedingung wie `plugin.name` geschwärzt
* `plugin.version`: Version aus dem Plugin-Manifest. Nur enthalten, wenn der Name nicht geschwärzt ist und das Manifest eine Version deklariert
* `plugin.scope`: Herkunftskategorie für das Plugin: `"official"`, `"org"`, `"user-local"` oder `"default-bundle"`
* `enabled_via`: Wie das Plugin aktiviert wurde: `"default-enable"`, `"org-policy"`, `"seed-mount"` oder `"user-install"`
* `plugin_id_hash`: Deterministische Hash des Plugin-Namens und des Marketplace, nur an Ihren konfigurierten Exporter gesendet. Ermöglicht es Ihnen, zu zählen, wie viele unterschiedliche Drittanbieter-Plugins über Ihre gesamte Flotte hinweg geladen sind, ohne ihre Namen aufzuzeichnen
* `has_hooks`: Ob das Plugin Hooks beiträgt
* `has_mcp`: Ob das Plugin MCP-Server beiträgt
* `host_owned_mcp`: `true`, wenn der SDK-Host die MCP-Verbindungen dieses Plugins verwaltet und Claude Code das Lesen der MCP-Server-Konfiguration des Plugins übersprungen hat, `false` andernfalls. Erfordert Claude Code v2.1.172 oder später
* `skill_path_count`: Anzahl der Skill-Verzeichnisse, die das Plugin deklariert
* `command_path_count`: Anzahl der Befehlsverzeichnisse, die das Plugin deklariert
* `agent_path_count`: Anzahl der Agent-Verzeichnisse, die das Plugin deklariert
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Im sicheren Modus meldet dieses Ereignis nur die konfigurierte Inventur; die Befehle, Skills, Hooks und MCP-Server des Plugins werden nicht geladen. Erfordert Claude Code v2.1.169 oder später

<h4 id="skill-activated-event">
  Skill-Aktiviert-Ereignis
</h4>

Protokolliert, wenn ein Skill aufgerufen wird, ob Claude ihn über das Skill-Tool aufruft oder Sie ihn als `/` Befehl ausführen.

**Ereignisname**: `claude_code.skill_activated`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `skill.name`: Name des Skills. Für benutzerdefinierte und Drittanbieter-Plugin-Skills ist der Wert der Platzhalter `"custom_skill"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Wie der Skill ausgelöst wurde (`"user-slash"`, `"claude-proactive"` oder `"nested-skill"`)
* `skill.source`: Wo der Skill geladen wurde (z. B. `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"`, wenn der Skill ein Workflow-Skill ist. Andernfalls nicht vorhanden
* `plugin.name` (wenn `OTEL_LOG_TOOL_DETAILS=1` oder das Plugin ist von einem offiziellen Marketplace): Name des besitzenden Plugins, wenn der Skill von einem Plugin bereitgestellt wird
* `marketplace.name` (wenn `OTEL_LOG_TOOL_DETAILS=1` oder das Plugin ist von einem offiziellen Marketplace): Marketplace des besitzenden Plugins, wenn der Skill von einem Plugin bereitgestellt wird

<h4 id="at-mention-event">
  At-Mention-Ereignis
</h4>

Protokolliert, wenn Claude Code ein `@`-Mention in einem Prompt auflöst. Nicht jedes Mention gibt ein Ereignis aus: Early-Exit-Pfade wie Berechtigungsverweigerungen, übergroße Dateien, PDF-Referenz-Anhänge und Fehler beim Auflisten von Verzeichnissen werden zurückgegeben, ohne zu protokollieren.

**Ereignisname**: `claude_code.at_mention`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `mention_type`: Typ des Mentions (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success`: Ob das Mention erfolgreich aufgelöst wurde (`"true"` oder `"false"`)

<h4 id="api-retries-exhausted-event">
  API-Wiederholungen-Erschöpft-Ereignis
</h4>

Protokolliert einmal, wenn eine API-Anfrage nach mehr als einem Versuch fehlschlägt. Wird zusammen mit dem letzten `api_error` Ereignis ausgegeben.

**Ereignisname**: `claude_code.api_retries_exhausted`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `model`: Verwendetes Modell
* `error`: Letzte Fehlermeldung
* `status_code`: HTTP-Statuscode als Zahl. Nicht vorhanden für Nicht-HTTP-Fehler.
* `total_attempts`: Gesamtzahl der Versuche
* `total_retry_duration_ms`: Gesamte Wanduhr-Zeit über alle Versuche
* `speed`: `"fast"` oder `"normal"`

<h4 id="hook-registered-event">
  Hook-Registriert-Ereignis
</h4>

Protokolliert einmal pro konfiguriertem Hook beim Sitzungsstart. Verwenden Sie dieses Ereignis, um zu inventarisieren, welche Hooks über Ihre gesamte Flotte hinweg aktiv sind, als Ergänzung zu den Pro-Ausführungs-Ereignissen `hook_execution_start` und `hook_execution_complete`.

**Ereignisname**: `claude_code.hook_registered`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `hook_event`: Hook-Ereignistyp, z. B. `"PreToolUse"` oder `"PostToolUse"`
* `hook_type`: Hook-Implementierungstyp: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"` oder `"agent"`
* `hook_source`: Wo der Hook definiert ist: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"` oder `"pluginHook"`
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Erfordert Claude Code v2.1.169 oder später
* `hook_matcher` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Die Matcher-Zeichenkette aus der Hook-Konfiguration, wenn eine gesetzt ist
* `plugin.name` (wenn `hook_source` `"pluginHook"` ist): Name des beitragenden Plugins. Für Plugins außerhalb des offiziellen Marketplace und des integrierten Bundles ist der Wert `"third-party"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (wenn `hook_source` `"pluginHook"` ist): Deterministische Hash des Plugin-Namens und des Marketplace, nur an Ihren konfigurierten Exporter gesendet. Ermöglicht es Ihnen, unterschiedliche beitragende Plugins zu zählen, ohne ihre Namen aufzuzeichnen

<h4 id="hook-execution-start-event">
  Hook-Ausführungs-Start-Ereignis
</h4>

Protokolliert, wenn ein oder mehrere Hooks für ein Hook-Ereignis beginnen auszuführen.

**Ereignisname**: `claude_code.hook_execution_start`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `hook_event`: Hook-Ereignistyp, z. B. `"PreToolUse"` oder `"PostToolUse"`
* `hook_name`: Vollständiger Hook-Name einschließlich Matcher, z. B. `"PreToolUse:Write"`
* `num_hooks`: Anzahl der übereinstimmenden Hook-Befehle
* `managed_only`: `"true"`, wenn nur verwaltete Richtlinien-Hooks zulässig sind
* `hook_source`: `"policySettings"` oder `"merged"`
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Erfordert Claude Code v2.1.169 oder später
* `hook_definitions`: JSON-serialisierte Hook-Konfiguration. Nur enthalten, wenn sowohl detailliertes Beta-Tracing als auch `OTEL_LOG_TOOL_DETAILS=1` aktiviert sind

<h4 id="hook-execution-complete-event">
  Hook-Ausführungs-Abschluss-Ereignis
</h4>

Protokolliert, wenn alle Hooks für ein Hook-Ereignis abgeschlossen sind.

**Ereignisname**: `claude_code.hook_execution_complete`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `hook_event`: Hook-Ereignistyp
* `hook_name`: Vollständiger Hook-Name einschließlich Matcher
* `num_hooks`: Anzahl der übereinstimmenden Hook-Befehle
* `num_success`: Anzahl, die erfolgreich abgeschlossen wurde
* `num_blocking`: Anzahl, die eine Blockierungsentscheidung zurückgegeben hat
* `num_non_blocking_error`: Anzahl, die ohne Blockierung fehlgeschlagen ist
* `num_cancelled`: Anzahl, die vor Abschluss abgebrochen wurde
* `total_duration_ms`: Wanduhr-Dauer aller übereinstimmenden Hooks
* `managed_only`: `"true"`, wenn nur verwaltete Richtlinien-Hooks zulässig sind
* `hook_source`: `"policySettings"` oder `"merged"`
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Erfordert Claude Code v2.1.169 oder später
* `hook_definitions`: JSON-serialisierte Hook-Konfiguration. Nur enthalten, wenn sowohl detailliertes Beta-Tracing als auch `OTEL_LOG_TOOL_DETAILS=1` aktiviert sind

<h4 id="hook-plugin-metrics-event">
  Hook-Plugin-Metriken-Ereignis
</h4>

Protokolliert, wenn ein offizieller Marketplace-Plugin-Hook Pro-Invokations-Metriken ausgibt. Nur Plugins, die von einem offiziellen Anthropic-Marketplace installiert wurden, können diese ausgeben. Drittanbieter-Marketplace-Plugins und benutzerdefinierte Hooks geben nicht zu diesem Ereignis aus. Verwenden Sie dieses Ereignis, um Plugin-Verhalten wie Findungsraten, Kosten und Dauern aus Ihrem eigenen Observability-Stack zu überwachen.

**Ereignisname**: `claude_code.hook_plugin_metrics`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `plugin_id`: Plugin-Kennung in `<name>@<marketplace>` Form
* `hook_event`: Hook-Ereignistyp, der die Metriken ausgegeben hat
* Bis zu 20 Plugin-ausgegebene Metrik-Schlüssel. Namen entsprechen `^[a-z][a-z0-9_]{0,39}$`. Werte sind boolescher Wert oder Zahl.

<h4 id="compaction-event">
  Kompaktierungs-Ereignis
</h4>

Protokolliert, wenn die Konversationskompaktierung abgeschlossen ist.

**Ereignisname**: `claude_code.compaction`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `trigger`: `"auto"` oder `"manual"`
* `success`: `"true"` oder `"false"`
* `duration_ms`: Kompaktierungs-Dauer
* `pre_tokens`: Ungefähre Token-Anzahl vor Kompaktierung
* `post_tokens`: Ungefähre Token-Anzahl nach Kompaktierung
* `error`: Fehlermeldung, wenn Kompaktierung fehlgeschlagen ist
* `precompute_reuse`: Nur gesetzt, wenn `trigger` `"manual"` ist. Auto-Kompaktierung kann eine Zusammenfassung im Hintergrund vorbereiten, bevor das Kontextfenster voll wird, und dieses Attribut zeichnet auf, ob `/compact` diese vorbereitete Zusammenfassung wiederverwendet hat. `"hit"` bedeutet, dass sie wiederverwendet wurde; `"miss_custom_instructions"`, `"miss_hook"` und `"miss_not_ready"` geben den Grund an, warum stattdessen eine neue Zusammenfassung berechnet wurde. Erfordert Claude Code v2.1.153 oder später

<h4 id="feedback-survey-event">
  Feedback-Umfrage-Ereignis
</h4>

Protokolliert, wenn eine Sitzungsqualitäts-Umfrage angezeigt oder beantwortet wird. Siehe [Sitzungsqualitäts-Umfragen](/docs/de/data-usage#session-quality-surveys) für das, was die Umfragen erfassen und wie Sie sie steuern.

**Ereignisname**: `claude_code.feedback_survey`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: monoton steigende Zähler zur Sortierung von Ereignissen innerhalb einer Sitzung
* `event_type`: Umfrage-Lebenszyklusereignis, z. B. `"appeared"`, `"responded"` oder `"transcript_prompt_appeared"`
* `appearance_id`: Eindeutige ID, die die Ereignisse verknüpft, die für eine Umfrage-Instanz ausgegeben werden
* `survey_type`: Welche Umfrage das Ereignis erzeugt hat. `"session"` ist die Aufforderung "Wie macht sich Claude?" Bewertung
* `response`: Die Auswahl des Benutzers bei `responded` Ereignissen
* `enabled_via_override`: `true`, wenn [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/de/env-vars) gesetzt ist. Wird als boolescher Wert ausgegeben, nicht als Zeichenkette. Vorhanden bei `session` Umfrage-Ereignissen. Filtern Sie nach diesem Attribut, um zu bestätigen, dass die Überschreibung über eine Flotte angewendet wird

<h2 id="interpret-metrics-and-events-data">
  Interpretation von Metriken- und Ereignisdaten
</h2>

Die exportierten Metriken und Ereignisse unterstützen eine Reihe von Analysen:

<h3 id="usage-monitoring">
  Nutzungsüberwachung
</h3>

| Metrik                                                        | Analysemöglichkeit                                                                                                |
| ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | Aufschlüsselung nach `type` (input/output), Benutzer, Team, Modell, `skill.name`, `plugin.name` oder `agent.name` |
| `claude_code.session.count`                                   | Verfolgung der Akzeptanz und des Engagements im Laufe der Zeit                                                    |
| `claude_code.lines_of_code.count`                             | Messung der Produktivität durch Verfolgung von Code-Hinzufügungen und -Entfernungen, aufgeschlüsselt nach Modell  |
| `claude_code.commit.count` & `claude_code.pull_request.count` | Verständnis der Auswirkungen auf Entwicklungs-Workflows                                                           |

<h3 id="cost-monitoring">
  Kostenüberwachung
</h3>

Die Metrik `claude_code.cost.usage` hilft bei:

* Verfolgung von Nutzungstrends über Teams oder Einzelpersonen hinweg
* Identifikation von Sitzungen mit hoher Nutzung zur Optimierung
* Zuordnung von Ausgaben zu spezifischen Skills, Plugins oder Subagent-Typen über die Attribute `skill.name`, `plugin.name` und `agent.name`

<Note>
  Kostenmetriken sind Näherungswerte. Für offizielle Abrechnungsdaten konsultieren Sie Ihren API-Anbieter (Claude Console, Amazon Bedrock oder Google Cloud's Agent Platform).
</Note>

<h3 id="alerting-and-segmentation">
  Warnungen und Segmentierung
</h3>

Häufige Warnungen, die Sie in Betracht ziehen sollten:

* Kostensteigerungen
* Ungewöhnlicher Token-Verbrauch
* Hohes Sitzungsvolumen von bestimmten Benutzern

Alle Metriken können nach den [Standard-Attributen](#standard-attributes) segmentiert werden. Das Attribut `model` ist auf `claude_code.token.usage`, `claude_code.cost.usage` und ab v2.1.172 auf `claude_code.lines_of_code.count` verfügbar.

Aufschlüsselungen pro Modell von Commits können nur durch Verknüpfung mit den Token- oder Kostenmetriken auf `session.id` angenähert werden, da eine Sitzung mehrere Modelle umfassen kann. Filtern Sie die Token- oder Kostenseite auf Zeilen, bei denen `query_source` `"main"` ist, damit Hilfs- und Subagent-Anfragen die Commits der Sitzung nicht einem Modell zuordnen, das sie nicht erstellt hat.

<h3 id="detect-retry-exhaustion">
  Wiederholungserschöpfung erkennen
</h3>

Claude Code wiederholt fehlgeschlagene API-Anfragen intern und gibt nur nach dem Aufgeben ein einzelnes `claude_code.api_error` Ereignis aus, daher ist das Ereignis selbst das Endsignal für diese Anfrage. Zwischenzeitliche Wiederholungsversuche werden nicht als separate Ereignisse protokolliert.

Das Attribut `attempt` auf dem Ereignis zeichnet auf, wie viele Versuche insgesamt unternommen wurden. `CLAUDE_CODE_MAX_RETRIES` hat einen Standardwert von 10 und ist auf 15 begrenzt; ab v2.1.199 erhöht `CLAUDE_CODE_RETRY_WATCHDOG` den Standardwert und entfernt die Obergrenze. Wenn die Anfrage alle Wiederholungen bei einem vorübergehenden Fehler erschöpft, ist `attempt` um eins höher als dieses effektive Limit: 11 standardmäßig und nie mehr als 16, es sei denn, der Watchdog ist gesetzt. Ein niedrigerer Wert zeigt einen nicht wiederholbaren Fehler wie eine `400` Antwort an.

Um eine Sitzung zu unterscheiden, die sich von einer, die steckengeblieben ist, erholt hat, gruppieren Sie Ereignisse nach `session.id` und prüfen Sie, ob ein späteres `api_request` Ereignis nach dem Fehler vorhanden ist.

<h3 id="event-analysis">
  Ereignisanalyse
</h3>

Die Ereignisdaten bieten detaillierte Einblicke in Claude Code-Interaktionen:

**Tool-Nutzungsmuster**: Analysieren Sie Tool-Ergebnis-Ereignisse, um zu identifizieren:

* Am häufigsten verwendete Tools
* Tool-Erfolgsquoten
* Durchschnittliche Tool-Ausführungszeiten
* Fehlermuster nach Tool-Typ

**Leistungsüberwachung**: Verfolgen Sie API-Anfrage-Dauern und Tool-Ausführungszeiten, um Leistungsengpässe zu identifizieren.

<h2 id="audit-security-events">
  Audit-Sicherheitsereignisse
</h2>

OpenTelemetry-Ereignisse sind die Audit-Datenquelle für Claude Code-Aktivität. Jedes Ereignis trägt Identitätsattribute, die Tool-Aufrufe, MCP-Aktivität und Berechtigungsentscheidungen an den Benutzer zurückbinden, der sie ausgelöst hat. Der OTLP-Logs-Exporter kann diese Ereignisse an jede Security Information and Event Management (SIEM)-Plattform mit einem OTLP-Receiver oder an einen OpenTelemetry Collector liefern, der an Ihr SIEM weiterleitet.

<h3 id="attribute-actions-to-users">
  Attribut-Aktionen an Benutzer
</h3>

Die [Standardattribute](#standard-attributes) auf jedem Ereignis enthalten die Identität des authentifizierten Benutzers: `user.email`, `user.account_uuid`, `user.account_id` und `organization.id`, wenn mit einem Claude-Konto angemeldet, plus `user.id` und die pro-Sitzung `session.id`. `user.id` ist ein installationsbegrenzter Bezeichner, außer bei [Claude apps gateway](/docs/de/claude-apps-gateway)-Sitzungen, wo es das IdP-Subjekt aus dem vom Gateway ausgegebenen Token ist.

MCP-Tool-Aufrufe, Bash-Befehle und Dateibearbeitungen werden daher dem Entwickler zugeordnet, der die Sitzung gestartet hat. Claude Code handelt nicht unter einem separaten Service-Konto; die Identität, die auf jedem Ereignis aufgezeichnet wird, ist das Claude-Konto des Entwicklers selbst, oder die IdP-Identität des Entwicklers bei einer [Claude apps gateway](/docs/de/claude-apps-gateway)-Sitzung.

Wenn Claude Code sich mit einem direkten API-Schlüssel authentifiziert oder gegen Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry, gibt es kein Claude-Konto in der Sitzung und nur `user.id` und `session.id` werden gefüllt. In diesen Bereitstellungen fügen Sie die Benutzeridentität selbst mit `OTEL_RESOURCE_ATTRIBUTES` hinzu, die pro Benutzer über die [verwaltete Einstellungsdatei](#administrator-configuration) oder einen Launch-Wrapper gesetzt wird. Claude apps gateway-Sitzungen benötigen nichts davon: Die CLI stempelt die IdP-Identität automatisch ab, wie in [Standardattribute](#standard-attributes) beschrieben.

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  Audit MCP-Aktivität
</h3>

Um MCP-Server-Aktivität mit vollständiger Call-Detail zu erfassen, aktivieren Sie den Logs-Exporter und setzen Sie `OTEL_LOG_TOOL_DETAILS=1`. Jede MCP-Operation erzeugt dann strukturierte Ereignisse, die den Server-Namen, Tool-Namen und Call-Argumente zusammen mit den Standard-Identitätsattributen tragen:

| Ereignis                | Was es für MCP aufzeichnet                                                                                                                                                                      |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | Server-Verbindung, Trennung und Verbindungsfehler mit `server_name`, `transport_type`, `server_scope` und Fehlerdetail                                                                          |
| `tool_result`           | Jeder MCP-Tool-Aufruf mit `tool_name` und `mcp_server_scope`, eine `tool_parameters` Nutzlast mit `mcp_server_name` und `mcp_tool_name`, und eine `tool_input` Nutzlast mit den Call-Argumenten |
| `tool_decision`         | Ob der Aufruf zulässig oder verweigert wurde, ob die Entscheidung von Config, einem Hook oder dem Benutzer kam, und eine `tool_parameters` Nutzlast mit `mcp_server_name` und `mcp_tool_name`   |

Ohne `OTEL_LOG_TOOL_DETAILS` lassen diese Ereignisse die identifizierende Detail fallen:

* `tool_result`: behält `tool_name` und `mcp_server_scope`, lässt `mcp_server_name`, `mcp_tool_name` und Argumente weg
* `tool_decision`: behält `tool_name`, lässt `tool_parameters` weg
* `mcp_server_connection`: lässt `server_name` und die Fehlermeldung weg, behält aber `is_plugin`, `plugin_id_hash` und `plugin.name`, wobei Namen von Nicht-Anthropic-Plugins auf das Literal `"third-party"` redigiert werden, sodass von Plugins bereitgestellte Server ohne detaillierte Protokollierung unterscheidbar bleiben

<h3 id="map-security-questions-to-events">
  Sicherheitsfragen zu Ereignissen zuordnen
</h3>

Beim Erstellen von Erkennungsregeln schlagen Sie das Signal auf, das Sie überwachen möchten, und fragen Sie Ihr Backend nach dem entsprechenden Ereignis und den Attributen ab:

| Signal                                            | Ereignis                                                                                  | Schlüsselattribute                                           |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Tool-Aufruf zulässig oder verweigert, und von wem | `tool_decision`                                                                           | `decision`, `source`, `tool_name`, `tool_parameters`         |
| Berechtigungsmodus-Eskalation                     | `permission_mode_changed`                                                                 | `from_mode`, `to_mode`, `trigger`                            |
| Policy-Hook blockierte eine Aktion                | `hook_execution_complete`                                                                 | `hook_event`, `num_blocking`                                 |
| Login, Logout und Authentifizierungsfehler        | `auth`                                                                                    | `action`, `success`, `error_category`                        |
| MCP-Server-Verbindung oder Fehler                 | `mcp_server_connection`                                                                   | `status`, `server_name`, `is_plugin`, `error_code`           |
| Plugin installiert und seine Quelle               | `plugin_installed`                                                                        | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| Befehle ausgeführt und Dateien berührt            | `tool_result` (ausgeführt) oder `tool_decision` (abgelehnt) mit `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` nur)          |

Claude Code gibt nur den rohen Ereignisstrom aus. Anomalieerkennung, Baselining, Korrelation über Sitzungen hinweg und Warnungen sind die Verantwortung Ihres SIEM oder Observability-Backends.

<h3 id="send-events-to-a-siem">
  Ereignisse an ein SIEM senden
</h3>

Zeigen Sie `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` auf den OTLP-Receiver Ihres SIEM oder auf einen OpenTelemetry Collector, der an die native Ingest-API Ihres SIEM weiterleitet. Das folgende verwaltete Einstellungsbeispiel exportiert nur Ereignisse, mit vollständiger Tool-Detail-Aktivierung für MCP- und Bash-Auditing:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

<h2 id="backend-considerations">
  Backend-Überlegungen
</h2>

Ihre Wahl des Metriken-, Logs- und Traces-Backends bestimmt die Arten von Analysen, die Sie durchführen können:

<h3 id="for-metrics">
  Für Metriken
</h3>

* **Zeitreihendatenbanken (zum Beispiel Prometheus)**: Ratenberechnungen, aggregierte Metriken
* **Spaltenorientierte Speicher (zum Beispiel ClickHouse)**: Komplexe Abfragen, eindeutige Benutzeranalyse
* **Vollständige Observability-Plattformen (zum Beispiel Honeycomb, Datadog, Grafana Cloud)**: Erweiterte Abfragen, Visualisierung, Warnungen

<h3 id="for-events/logs">
  Für Ereignisse/Logs
</h3>

* **Log-Aggregationssysteme (zum Beispiel Elasticsearch, Loki)**: Volltextsuche, Log-Analyse
* **Spaltenorientierte Speicher (zum Beispiel ClickHouse)**: Strukturierte Ereignisanalyse
* **Vollständige Observability-Plattformen (zum Beispiel Honeycomb, Datadog, Grafana Cloud)**: Korrelation zwischen Metriken und Ereignissen

<h3 id="for-traces">
  Für Traces
</h3>

Wählen Sie ein Backend, das verteilte Trace-Speicherung und Span-Korrelation unterstützt:

* **Verteilte Tracing-Systeme (zum Beispiel Jaeger, Zipkin, Grafana Tempo)**: Span-Visualisierung, Request-Waterfalls, Latenzanalyse
* **Vollständige Observability-Plattformen (zum Beispiel Honeycomb, Datadog, Grafana Cloud)**: Trace-Suche und Korrelation mit Metriken und Logs

Für Organisationen, die Daily/Weekly/Monthly Active User (DAU/WAU/MAU) Metriken benötigen, sollten Sie Backends in Betracht ziehen, die effiziente Abfragen eindeutiger Werte unterstützen.

<h2 id="service-information">
  Dienstinformationen
</h2>

Alle Metriken und Ereignisse werden mit den folgenden Ressourcenattributen exportiert:

* `service.name`: `claude-code`
* `service.version`: Aktuelle Claude Code-Version
* `os.type`: Betriebssystemtyp (zum Beispiel `linux`, `darwin`, `windows`)
* `os.version`: Betriebssystem-Versionsnummer
* `host.arch`: Host-Architektur (zum Beispiel `amd64`, `arm64`)
* `wsl.version`: WSL-Versionsnummer (nur vorhanden, wenn auf Windows Subsystem for Linux ausgeführt)
* Meter-Name: `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  ROI-Messung-Ressourcen
</h2>

Für einen umfassenden Leitfaden zur Messung der Kapitalrendite für Claude Code, einschließlich Telemetrie-Setup, Kostenanalyse, Produktivitätsmetriken und automatisierter Berichterstattung, siehe den [Claude Code ROI Measurement Guide](https://github.com/anthropics/claude-code-monitoring-guide). Dieses Repository bietet einsatzbereite Docker Compose-Konfigurationen, Prometheus- und OpenTelemetry-Setups sowie Vorlagen zur Generierung von Produktivitätsberichten, die in Tools wie Linear integriert sind.

<h2 id="security-and-privacy">
  Sicherheit und Datenschutz
</h2>

* OpenTelemetry-Export zu Ihrem Backend ist opt-in und erfordert explizite Konfiguration. Informationen zu Anthropics separater operativer Telemetrie und wie Sie diese deaktivieren, finden Sie unter [Datennutzung](/docs/de/data-usage#telemetry-services)
* Rohe Dateiinhalte und Code-Snippets sind nicht in Metriken oder Ereignissen enthalten. Trace-Spans sind ein separater Datenpfad: siehe die Aufzählung `OTEL_LOG_TOOL_CONTENT` unten
* Wenn über OAuth authentifiziert, ist `user.email` in Telemetrie-Attributen enthalten. Wenn dies ein Problem für Ihre Organisation darstellt, arbeiten Sie mit Ihrem Telemetrie-Backend zusammen, um dieses Feld zu filtern oder zu schwärzen
* Benutzer-Prompt-Inhalte werden standardmäßig nicht erfasst. Nur die Prompt-Länge wird aufgezeichnet. Um Benutzer-Prompt-Inhalte einzubeziehen, setzen Sie `OTEL_LOG_USER_PROMPTS=1`
* Assistent-Antworttext wird standardmäßig nicht erfasst. Nur die Antwortlänge wird aufgezeichnet. Um Antworttext einzubeziehen, setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=1`. Wie alle OpenTelemetry-Daten von Claude Code wird der Antworttext nur an den OTel-Endpunkt gesendet, den Sie konfigurieren, niemals an Anthropic. Wenn diese Variable nicht gesetzt ist, wird `OTEL_LOG_USER_PROMPTS` als Fallback verwendet, daher setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=0`, wenn Sie Prompt-Inhalte ohne Antwortinhalte möchten
* Tool-Eingabeargumente und Parameter werden standardmäßig nicht protokolliert. Um sie einzubeziehen, setzen Sie `OTEL_LOG_TOOL_DETAILS=1`. Diese Daten werden nur an den OTEL-Endpunkt gesendet, den Sie konfigurieren, niemals an Anthropic. Argumente können immer noch vertrauliche Werte enthalten, daher konfigurieren Sie Ihr Telemetrie-Backend, um diese Attribute nach Bedarf zu filtern oder zu schwärzen. Wenn aktiviert:
  * `tool_result`- und `tool_decision`-Ereignisse enthalten ein `tool_parameters`-Attribut mit Bash-Befehlen, MCP-Server- und Tool-Namen sowie Skill-Namen. Felder wie `full_command` werden ungekürzt ausgegeben
  * `tool_result`-Ereignisse enthalten zusätzlich ein `tool_input`-Attribut mit Dateipfaden, URLs, Suchmustern und anderen Argumenten. Einzelne Werte über 512 Zeichen werden gekürzt und die Gesamtmenge ist auf etwa 4 K Zeichen begrenzt
  * `user_prompt`-Ereignisse enthalten den wörtlichen `command_name` für benutzerdefinierte, Plugin- und MCP-Befehle
  * Trace-Spans enthalten das gleiche `tool_input`-Attribut und eingabebezogene Attribute wie `file_path`, mit der gleichen Kürzung wie `tool_input`
* Tool-Eingabe- und Ausgabeinhalte werden in Trace-Spans standardmäßig nicht protokolliert. Um sie einzubeziehen, setzen Sie `OTEL_LOG_TOOL_CONTENT=1`. Wenn aktiviert, enthalten Span-Ereignisse vollständige Tool-Eingabe- und Ausgabeinhalte, gekürzt bei 60 KB pro Span. Dies kann rohe Dateiinhalte aus Read-Tool-Ergebnissen und Bash-Befehlsausgabe enthalten. Konfigurieren Sie Ihr Telemetrie-Backend, um diese Attribute nach Bedarf zu filtern oder zu schwärzen
* Rohe Anthropic Messages API-Anfrage- und Antwort-Texte werden standardmäßig nicht protokolliert. Um sie einzubeziehen, setzen Sie `OTEL_LOG_RAW_API_BODIES`. Mit `=1` gibt jeder API-Aufruf `api_request_body`- und `api_response_body`-Log-Ereignisse aus, deren `body`-Attribut die JSON-serialisierte Nutzlast ist, gekürzt bei 60 KB. Mit `=file:<dir>` werden ungekürzte Texte unter diesem Verzeichnis in `.request.json`- und `.response.json`-Dateien geschrieben und die Ereignisse tragen einen `body_ref`-Pfad statt des Inline-Textes. Versenden Sie das Verzeichnis mit einem Log-Collector oder Sidecar statt über den Telemetrie-Stream. In beiden Modi enthalten Texte die gesamte Konversationshistorie (Systemprompt, jeder vorherige Benutzer- und Assistent-Durchgang, Tool-Ergebnisse), daher impliziert das Aktivieren dies Zustimmung zu allem, was die anderen `OTEL_LOG_*`-Content-Flags offenbaren würden. Claudes Extended-Thinking-Inhalte werden unabhängig von anderen Einstellungen immer aus diesen Texten geschwärzt

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Überwachung von Claude Code auf Amazon Bedrock
</h2>

Für detaillierte Anleitung zur Überwachung der Claude Code-Nutzung für Amazon Bedrock siehe [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
