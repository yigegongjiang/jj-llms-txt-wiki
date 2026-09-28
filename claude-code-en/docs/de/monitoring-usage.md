> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Überwachung

> Erfahren Sie, wie Sie OpenTelemetry für Claude Code aktivieren und konfigurieren.

Verfolgen Sie die Nutzung, Kosten und Toolaktivität von Claude Code in Ihrer Organisation, indem Sie Telemetriedaten über OpenTelemetry (OTel) exportieren. Claude Code exportiert Metriken als Zeitreihendaten über das Standard-Metriken-Protokoll, Ereignisse über das Logs/Events-Protokoll und optional verteilte Traces über das [Traces-Protokoll](#traces-beta).

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

# 5. Zum Debuggen: Exportintervalle reduzieren und für die Produktionsnutzung zurücksetzen
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 Sekunden (Standard: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 Sekunden (Standard: 5000ms)

# 6. Claude Code ausführen
claude
```

Um eine Einrichtung zu überprüfen, die Metriken exportiert, suchen Sie in Ihrem Backend nach der Metrik `claude_code.session.count`, die Claude Code beim Start einer Sitzung ausgibt. Um eine reine Logs-Einrichtung zu überprüfen, senden Sie eine Eingabeaufforderung und suchen Sie nach dem Ereignis `claude_code.user_prompt`.

Wenn nichts ankommt, führen Sie `claude --debug` aus und überprüfen Sie das Debug-Protokoll. Claude Code meldet Fehler von den konfigurierten Exportern als `[3P telemetry]`-Fehler, wobei 3P für Drittanbieter steht. Zeilen mit dem Präfix `[Anthropic telemetry]` beschreiben [Anthropics separate operative Telemetrie](/docs/de/data-usage#telemetry-services) und deuten nicht auf ein Problem mit Ihrer Einrichtung hin.

Für vollständige Konfigurationsoptionen siehe die [OpenTelemetry-Spezifikation](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Administratorkonfiguration
</h2>

Administratoren können OpenTelemetry-Einstellungen für alle Benutzer über die [verwaltete Einstellungsdatei](/docs/de/managed-settings#delivery-mechanisms) konfigurieren. Weitere Informationen zur Anwendung von Einstellungen finden Sie unter [Einstellungspriorität](/docs/de/settings#settings-precedence).

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

Claude Code ignoriert die [OpenTelemetry-Exporter-Variablen](/docs/de/settings-reference#variables-claude-code-ignores-in-env) in der `.claude/settings.json` und `.claude/settings.local.json` eines Repositorys, daher kann ein Repository diese nicht verwenden, um Telemetrie einzuschalten, zu wählen, wohin sie geht, oder Inhalte zu erfassen. Setzen Sie sie in verwalteten Einstellungen, oder lassen Sie jeden Entwickler sie in seiner Shell oder `~/.claude/settings.json` setzen. Ein Repository kann ein Signal immer noch ausschalten, indem es seinen Exporter-Selektor, wie `OTEL_LOGS_EXPORTER`, auf `none` setzt, es sei denn, verwaltete Einstellungen, eine `--settings`-Datei oder die Umgebung, aus der Sie Claude Code starten, setzen diese Variable.

Claude Code übergibt `OTEL_*` Umgebungsvariablen nicht an die Subprozesse, die es erzeugt, einschließlich des Bash-Tools, Hooks, MCP-Server und Sprachserver. Eine OpenTelemetry-instrumentierte Anwendung, die Sie über das Bash-Tool ausführen, erbt nicht den Exporter-Endpunkt oder die Header von Claude Code, daher setzen Sie diese Variablen direkt im Befehl, wenn diese Anwendung ihre eigene Telemetrie exportieren muss.

<h3 id="how-managed-settings-lock-the-otlp-destination">
  Wie verwaltete Einstellungen das OTLP-Ziel sperren
</h3>

Wenn Sie eine `OTEL_EXPORTER_OTLP_*` Variable in verwalteten Einstellungen setzen, entfernt Claude Code bei der Initialisierung konfligierende von Entwicklern gesetzte Variablen und protokolliert eine Warnung, die Sie mit `claude --debug` sehen können. Was entfernt wird, hängt davon ab, welche Variable Sie setzen:

* **Endpunkte**: Wenn Sie `OTEL_EXPORTER_OTLP_ENDPOINT` setzen, entfernt Claude Code jeden von Entwicklern gesetzten signalspezifischen Endpunkt. Entwickler können ein Signal nicht auf einen anderen Collector verweisen, daher müssen Sie die signalspezifischen Endpunkt-Variablen nicht auch in verwalteten Einstellungen setzen.
* **Protokolle**: Wenn Sie `OTEL_EXPORTER_OTLP_PROTOCOL` setzen, entfernt Claude Code jeden von Entwicklern gesetzten signalspezifischen Protokoll.
* **Anmeldedaten**: Wenn Sie `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_CLIENT_KEY` oder `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE` setzen, entfernt Claude Code die von Entwicklern gesetzten signalspezifischen Versionen dieser Variable sowie jede von Entwicklern gesetzte Endpunkt-Variable, generisch oder signalspezifisch, da diese Anmeldedaten sonst einen Collector erreichen würden, den die verwalteten Einstellungen nicht ausgewählt haben.
* **Exporter-Selektoren**: `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` und der Beta-`OTEL_TRACES_EXPORTER` folgen der normalen Pro-Schlüssel-Priorität. Eine Einstellung eines Entwicklers kann ein Signal immer noch deaktivieren oder auf den Console-Exporter umschalten, daher setzen Sie die Selektoren auch in verwalteten Einstellungen, wenn Sie sie sperren müssen. Über [Admin-Quellen](/docs/de/managed-settings#precedence-within-the-managed-tier) folgt `OTEL_LOGS_EXPORTER` der [Telemetrie-Einheit](/docs/de/server-managed-settings#per-key-exceptions-across-managed-sources), während die anderen beiden Selektoren pro Schlüssel zusammengeführt werden. Erfordert Claude Code v2.1.223 oder später.
* **Beta-Tracing-Endpunkte**: Mit [detailliertem Beta-Tracing](#traces-beta) aktiv exportiert Claude Code Logs und Traces zu `BETA_TRACING_ENDPOINT` statt über die Logs- und Traces-Exporter. Claude Code entfernt daher einen von Entwicklern gesetzten `BETA_TRACING_ENDPOINT`, wenn eine dieser verwalteten Einstellungen das Ziel eines Signals entscheidet:

  * Ein generischer oder Logs/Traces-Endpunkt oder Anmeldedaten
  * Ein [`otelHeadersHelper`](/docs/de/settings-reference#otelheadershelper)
  * Ein Logs- oder Traces-Exporter-Selektor auf `none`, `console` oder leer gesetzt, Werte, die das Signal von einem Collector fernhalten
  * `CLAUDE_CODE_ENABLE_TELEMETRY` ausgeschaltet

  Ein nur-Metriken-Endpunkt oder Anmeldedaten entfernen ihn nicht. Vor v2.1.251 leitete ein von Entwicklern gesetzter `BETA_TRACING_ENDPOINT` die Logs und Traces um, die detailliertes Beta-Tracing exportiert, selbst wenn verwaltete Einstellungen den Collector festlegten.

Claude Code entfernt signalspezifische Variablen nicht, die Sie in verwalteten Einstellungen selbst setzen, daher können Sie ein Signal zu einem anderen Collector leiten, indem Sie seine Variable dort setzen, wie das [SIEM-Beispiel](#send-events-to-a-siem) zeigt. Wenn Sie dort eine signalspezifische Anmeldedaten setzen, entfernt Claude Code den von Entwicklern gesetzten Endpunkt für dieses Signal.

Dieses Entfernungsverhalten ändert, wohin Telemetrie geliefert wird, nicht was Claude Code erfasst.

Vor v2.1.217 folgte jede Variable unabhängig der Pro-Schlüssel-Einstellungspriorität, daher leitete ein signalspezifischer Endpunkt, der in Benutzereinstellungen oder der Shell gesetzt wurde, dieses Signal vom verwalteten Collector ab.

Wenn die Desktop-App oder ein [selbst gehosteter Umgebungs](/docs/de/self-hosted-environments)-Runner Claude Code startet und einen OTLP-Endpunkt in der bereitgestellten Umgebung benennt, heftet Claude Code das Ziel auf die gleiche Weise fest: Die Telemetrie-Variablen des Launchers entfernen von Entwicklern gesetzte Variablen genau wie verwaltete Einstellungen. Claude Code entfernt Variablen nicht, die der Launcher selbst gesetzt hat. Erfordert Claude Code v2.1.251 oder später.

<h2 id="configuration-details">
  Konfigurationsdetails
</h2>

<h3 id="common-configuration-variables">
  Allgemeine Konfigurationsvariablen
</h3>

Diese Variablen konfigurieren Exporter, Endpunkte und Exportverhalten für alle Bereitstellungen.

Wenn Sie eine signalspezifische Endpunkt- oder Protokollvariable wie `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` setzen, verwendet Claude Code diese statt der generischen Variablen für dieses Signal. Wenn Sie eine signalspezifische Header-Variable wie `OTEL_EXPORTER_OTLP_METRICS_HEADERS` setzen, führt Claude Code diese mit der generischen `OTEL_EXPORTER_OTLP_HEADERS` für dieses Signal zusammen.

Auf Maschinen mit verwalteten Einstellungen siehe [Wie verwaltete Einstellungen das OTLP-Ziel sperren](#how-managed-settings-lock-the-otlp-destination), um zu sehen, was Claude Code entfernt.

| Umgebungsvariable                                   | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Beispielwerte                                                                                                                                                               |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Aktiviert die Telemetrieerfassung (erforderlich)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `1`                                                                                                                                                                         |
| `OTEL_METRICS_EXPORTER`                             | Metriken-Exporter-Typ(en), kommagetrennt. Verwenden Sie `none` zum Deaktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `console`, `otlp`, `prometheus`, `none`                                                                                                                                     |
| `OTEL_LOGS_EXPORTER`                                | Logs/Events-Exporter-Typ(en), kommagetrennt. Verwenden Sie `none` zum Deaktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `console`, `otlp`, `none`                                                                                                                                                   |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protokoll für OTLP-Exporter, gilt für alle Signale. Claude Code hat kein Standardprotokoll, daher setzen Sie dies oder die signalspezifische Protokollvariable für jeden `otlp`-Exporter, den Sie aktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `grpc`, `http/json`, `http/protobuf`                                                                                                                                        |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | OTLP-Collector-Endpunkt für alle Signale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `http://localhost:4317`                                                                                                                                                     |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protokoll für Metriken, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `grpc`, `http/json`, `http/protobuf`                                                                                                                                        |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | OTLP-Metriken-Endpunkt, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `http://localhost:4318/v1/metrics`                                                                                                                                          |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protokoll für Logs, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `grpc`, `http/json`, `http/protobuf`                                                                                                                                        |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | OTLP-Logs-Endpunkt, überschreibt allgemeine Einstellung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `http://localhost:4318/v1/logs`                                                                                                                                             |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Authentifizierungsheader für OTLP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `Authorization=Bearer token`                                                                                                                                                |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | Authentifizierungsheader für Metriken, zusammengeführt mit den allgemeinen Headern                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `Authorization=Bearer token`                                                                                                                                                |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | Authentifizierungsheader für Logs, zusammengeführt mit den allgemeinen Headern                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `Authorization=Bearer token`                                                                                                                                                |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Exportintervall in Millisekunden (Standard: 60000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `5000`, `60000`                                                                                                                                                             |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Logs-Exportintervall in Millisekunden (Standard: 5000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `1000`, `10000`                                                                                                                                                             |
| `OTEL_LOG_USER_PROMPTS`                             | Aktiviert die Protokollierung von Benutzer-Prompt-Inhalten (Standard: deaktiviert)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `1` zum Aktivieren                                                                                                                                                          |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Aktiviert die Protokollierung von Assistent-Antworttext bei `assistant_response`-Ereignissen (Standard: deaktiviert). Wenn nicht gesetzt, wird auf den Wert von `OTEL_LOG_USER_PROMPTS` zurückgegriffen. Erfordert Claude Code v2.1.193 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `1` zum Aktivieren, `0` zum Beibehalten von Redaktionen                                                                                                                     |
| `OTEL_LOG_TOOL_DETAILS`                             | Aktiviert die Protokollierung von Tool-Parametern und Eingabeargumenten in Tool-Ereignissen und Trace-Span-Attributen: Bash-Befehle, MCP-Server- und Tool-Namen, Skill-Namen, benutzerdefinierte Workflow-Namen und Tool-Eingabe. Aktiviert auch benutzerdefinierte, Plugin- und MCP-Befehlsnamen bei `user_prompt`-Ereignissen (Standard: deaktiviert). Für Claude Desktop's integrierte Server, in Sitzungen, die Claude Desktop besitzt, werden `mcp_server_name`/`mcp_tool_name` bei `tool_decision`/`tool_result` auch mit ausgeschaltetem Flag ausgegeben. Die Ausnahme erfordert Claude Code v2.1.214 oder später                                                                                                                                                                      | `1` zum Aktivieren                                                                                                                                                          |
| `OTEL_LOG_TOOL_CONTENT`                             | Aktiviert die Protokollierung von Tool-Inhalten im [`tool.output` Span-Ereignis](#tool-output-span-event) (Standard: deaktiviert). Span-Attribute tragen Tool-Inhalte unter [ihren eigenen Gates](#new-context-gates). Erfordert [Tracing](#traces-beta). Der Inhalt wird bei der Inhaltsbegrenzung gekürzt (Standard: 60 KB)                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `1` zum Aktivieren                                                                                                                                                          |
| `OTEL_LOG_MANAGED_SETTINGS`                         | Fügt die redigierten verwalteten Einstellungen und einen SHA-256-Digest der Einstellungen vor der Redaktion zu [verwaltete Einstellungen aufgelöst](#managed-settings-resolved-event) Ereignissen hinzu (Standard: deaktiviert). Ein Wert in Projekt- oder lokalen Einstellungen schaltet ihn nicht ein. Erfordert Claude Code v2.1.274 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                           | `1` zum Aktivieren                                                                                                                                                          |
| `OTEL_LOG_RAW_API_BODIES`                           | Gibt die vollständige Anthropic Messages API-Anfrage und Antwort JSON als `api_request_body` / `api_response_body` Log-Ereignisse aus (Standard: deaktiviert). Texte enthalten die gesamte Konversationshistorie. Das Aktivieren impliziert Zustimmung zu allem, was `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` und `OTEL_LOG_TOOL_CONTENT` offenbaren würden                                                                                                                                                                                                                                                                                                                                                                                                                           | `1` für Inline-Texte gekürzt bei der Inhaltsbegrenzung (Standard: 60 KB), oder `file:<dir>` für ungekürzte Texte auf der Festplatte mit einem `body_ref`-Zeiger im Ereignis |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | Inhaltsbegrenzung: die maximale Länge von inhaltsführenden Attributen wie Modellreaktionen, Tool-Inhalten, Systemprompts und rohen API-Texten, Kürzungsmarkierung eingeschlossen, in UTF-16-Codeeinheiten (Standard: 61440, d. h. 60 KB). Der Standard ist für Backends ausgelegt, die Attributwerte auf 64 KB begrenzen; erhöhen Sie ihn nur, wenn Ihr Backend größere Werte akzeptiert, oder senken Sie ihn, um das Telemetrie-Volumen zu reduzieren. Wenn ein OpenTelemetry SDK-Attributlimit, `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` oder eine seiner Logrecord- und Span-Varianten, auf einen niedrigeren Wert gesetzt ist, kürzt Claude Code bei diesem kleineren Wert, damit die `[TRUNCATED ...]`-Markierung innerhalb des SDK-Limits bleibt. Erfordert Claude Code v2.1.214 oder später | `262144`                                                                                                                                                                    |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Metriken-Temporalitätspräferenz (Standard: `delta`). Setzen Sie auf `cumulative`, wenn Ihr Backend kumulative Temporalität erwartet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `delta`, `cumulative`                                                                                                                                                       |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervall zum Aktualisieren dynamischer Header (Standard: 1740000ms / 29 Minuten)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `900000`                                                                                                                                                                    |

Für die Protokolle `http/protobuf` und `http/json` sendet Claude Code jede Exportanfrage mit einem `Content-Length`-Header. Vor v2.1.212 sendeten Claude Code-Versionen ab v2.1.191 diese Anfragen mit Chunked-Transfer-Codierung; Azure Monitor und andere Endpunkte, die eine deklarierte Länge erfordern, lehnten sie mit `411 Length Required` oder `400` Fehlern ab.

<h3 id="mtls-authentication">
  mTLS-Authentifizierung
</h3>

Wie Sie Client-Zertifikate für den OTLP-Exporter konfigurieren, hängt vom OTLP-Protokoll ab, das für dieses Signal verwendet wird, das über `OTEL_EXPORTER_OTLP_PROTOCOL` oder die Pro-Signal-Überschreibung gesetzt wird. Die gleiche Konfiguration gilt für Metriken, Logs und Traces.

| Protokoll                    | Client-Zertifikat-Variablen                                                                                                                                                                               | Vertrauen Sie dem Collector-CA mit |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY` und optional `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Siehe [Netzwerkkonfiguration](/docs/de/network-config#mtls-authentication)                               | `NODE_EXTRA_CA_CERTS`              |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` und `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, oder die Pro-Signal-Varianten wie `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY`, um ein anderes Zertifikat pro Signal zu verwenden | `OTEL_EXPORTER_OTLP_CERTIFICATE`   |

Für `grpc` liest das OpenTelemetry SDK die Standard-OTLP-Variablen direkt, daher funktionieren bestehende Konfigurationen, die die signalspezifischen Metriken-Variablen setzen, weiterhin. Auf Maschinen mit verwalteten Einstellungen kann Claude Code [signalspezifische Anmeldedaten und Endpunkte, die von Entwicklern gesetzt wurden, beim Start entfernen](#how-managed-settings-lock-the-otlp-destination).

<h3 id="metrics-cardinality-control">
  Metriken-Kardinalitätskontrolle
</h3>

Die folgenden Umgebungsvariablen steuern, welche Attribute in Metriken enthalten sind, um die Kardinalität zu verwalten:

| Umgebungsvariable                          | Beschreibung                                                                                                                                               | Standardwert | Beispiel zum Deaktivieren |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ | ------------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Attribut session.id in Metriken einschließen                                                                                                               | `true`       | `false`                   |
| `OTEL_METRICS_INCLUDE_VERSION`             | Attribut app.version in Metriken einschließen                                                                                                              | `false`      | `true`                    |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Attribute user.account\_uuid und user.account\_id in Metriken einschließen                                                                                 | `true`       | `false`                   |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Attribut app.entrypoint in Metriken einschließen                                                                                                           | `false`      | `true`                    |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Schlüssel aus `OTEL_RESOURCE_ATTRIBUTES` als Attribute auf Metrik-Datenpunkten einschließen                                                                | `true`       | `false`                   |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | Schließen Sie `vcs.*` [Repository-Identitätsattribute](#repository-attributes) in Metriken und Ereignissen ein. Erfordert Claude Code v2.1.269 oder später | `false`      | `true`                    |

Eine niedrigere Kardinalität bedeutet in der Regel bessere Leistung und niedrigere Speicherkosten, aber weniger granulare Daten für die Analyse.

<h3 id="traces-beta">
  Traces (Beta)
</h3>

Verteiltes Tracing exportiert Spans, die jeden Benutzer-Prompt mit den API-Anfragen und Tool-Ausführungen verknüpfen, die er auslöst, sodass Sie eine vollständige Anfrage als einzelnen Trace in Ihrem Tracing-Backend anzeigen können.

Tracing ist standardmäßig deaktiviert. Um es zu aktivieren, setzen Sie sowohl `CLAUDE_CODE_ENABLE_TELEMETRY=1` als auch `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, und setzen Sie dann `OTEL_TRACES_EXPORTER`, um auszuwählen, wohin Spans gesendet werden. Traces verwenden die [allgemeine OTLP-Konfiguration](#common-configuration-variables) für Endpunkt, Protokoll, Header und [mTLS](#mtls-authentication) erneut. Auf Maschinen mit verwalteten Einstellungen kann Claude Code [signalspezifische Anmeldedaten und Endpunkte, die von Entwicklern gesetzt wurden, beim Start entfernen](#how-managed-settings-lock-the-otlp-destination).

| Umgebungsvariable                     | Beschreibung                                                                                 | Beispielwerte                        |
| ------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Aktiviert Span-Tracing (erforderlich). `ENABLE_ENHANCED_TELEMETRY_BETA` wird auch akzeptiert | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Traces-Exporter-Typ(en), kommagetrennt. Verwenden Sie `none` zum Deaktivieren                | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protokoll für Traces, überschreibt `OTEL_EXPORTER_OTLP_PROTOCOL`                             | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | OTLP-Traces-Endpunkt, überschreibt `OTEL_EXPORTER_OTLP_ENDPOINT`                             | `http://localhost:4318/v1/traces`    |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | Authentifizierungsheader für Traces, zusammengeführt mit `OTEL_EXPORTER_OTLP_HEADERS`        | `Authorization=Bearer token`         |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Span-Batch-Exportintervall in Millisekunden (Standard: 5000)                                 | `1000`, `10000`                      |

Spans schwärzen Benutzer-Prompt-Text, Tool-Eingabedetails und Tool-Inhalte standardmäßig. Setzen Sie `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` und `OTEL_LOG_TOOL_CONTENT=1`, um sie einzubeziehen.

Wenn Tracing aktiv ist, erben Bash- und PowerShell-Subprozesse automatisch eine `TRACEPARENT`-Umgebungsvariable, die den W3C-Trace-Kontext des aktiven Tool-Ausführungs-Spans enthält. Dies ermöglicht jedem Subprozess, der `TRACEPARENT` liest, seine eigenen Spans unter demselben Trace zu verschachteln, was eine End-to-End-verteilte Tracing durch Skripte und Befehle ermöglicht, die Claude ausführt.

Wenn Tracing aktiv ist und Claude Code direkt mit der Anthropic API verbunden ist, trägt jede Modellanfrage einen W3C `traceparent` Header, der auf den Kontext des `claude_code.llm_request` Spans gesetzt ist, und der `traceresponse` Header der API wird als Span-Link aufgezeichnet. Zusammen verbinden diese Claude Code's clientseitige Spans mit dem serverseitigen Trace durch jeden konformen Vermittler. Der Header wird nicht an Drittanbieter gesendet.

Standardmäßig wird der `traceparent` Header bei Model- und HTTP MCP-Anfragen nur gesendet, wenn `ANTHROPIC_BASE_URL` nicht gesetzt ist oder auf die Anthropic API verweist, da einige Proxys unbekannte Header ablehnen. Die Subprozess-`TRACEPARENT`-Variable wird durch denselben Schalter aus Konsistenzgründen gesteuert. Wenn Sie Claude Code durch einen benutzerdefinierten `ANTHROPIC_BASE_URL` Proxy ausführen und Trace-Kontext propagiert werden soll, setzen Sie `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

In Agent SDK und nicht-interaktiven Sitzungen, die mit `-p` gestartet werden, liest Claude Code auch `TRACEPARENT` und `TRACESTATE` aus seiner eigenen Umgebung, wenn jeder Interaktions-Span gestartet wird. Dies ermöglicht einem Embedding-Prozess, seinen aktiven W3C-Trace-Kontext in den Subprozess zu übergeben, sodass Claude Code's Spans als untergeordnete Elemente des Aufrufers verteilter Trace erscheinen. Interaktive Sitzungen ignorieren eingehende `TRACEPARENT`, um zu vermeiden, dass versehentlich Umgebungswerte aus CI oder Container-Umgebungen geerbt werden.

Der eingehende Trace-Kontext gilt auch für [Ereignisse](#events). In Agent SDK und `-p` Sitzungen mit `TRACEPARENT` gesetzt, trägt jeder OTLP-Ereignis-Log-Datensatz `trace_id` und `span_id` Werte, die ihn mit Ihrem Anwendungs-Trace verbinden, auch wenn der Traces-Exporter nicht konfiguriert ist, sodass Ihr Logging-Backend Ereignisse mit dem Rest des Trace korrelieren kann.

Ein Datensatz, der ausgegeben wird, während eine Interaktion aktiv ist, trägt die IDs des Interaktions-Spans, auch wenn Claude Code ihn außerhalb des asynchronen Kontexts des Spans ausgibt, wie in einem Berechtigungsprompt-Callback oder für einen Datensatz, der während des Starts gepuffert und später exportiert wird. Ein Datensatz, der ohne aktiven Interaktions-Span ausgegeben wird, trägt die eingehenden `TRACEPARENT` IDs direkt. Vor v2.1.214 trugen Datensätze, die außerhalb des asynchronen Kontexts des Spans ausgegeben wurden, die eingehenden `TRACEPARENT` IDs statt der Span-IDs. Vor v2.1.212 trugen Ereignis-Datensätze, die außerhalb eines aktiven Spans ausgegeben wurden, keine `trace_id` oder `span_id`.

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

Wenn ein `PreToolUse` Hook [einen Tool-Aufruf aufschiebt](/docs/de/hooks#defer-a-tool-call-for-later), speichert Claude Code den Trace-Kontext des Durchgangs, der ihn aufgeschoben hat. Wenn Sie die Sitzung fortsetzen und das Tool erneut ausgeführt wird, treten die Spans des Tools diesem früheren Durchgang's Trace als untergeordnete Elemente des Durchgangs's `claude_code.interaction` Spans bei.

<h4 id="span-attributes">
  Span-Attribute
</h4>

Jeder Span trägt die [Standardattribute](#standard-attributes) plus ein `span.type`-Attribut, das seinem Namen entspricht. Die folgenden Tabellen listen die zusätzlichen Attribute auf, die auf jedem Span gesetzt sind. Die Spans `llm_request`, `tool.execution` und `hook` setzen OpenTelemetry-Status `ERROR`, wenn sie einen Fehler aufzeichnen; die anderen Spans enden immer mit Status `UNSET`.

**`claude_code.interaction`**

| Attribut                  | Beschreibung                                                                                                                                                                                                     | Gated durch             |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Prompt-Text. Der Wert ist `<REDACTED>`, es sei denn, das Gate ist gesetzt                                                                                                                                        | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Prompt-Länge in Zeichen                                                                                                                                                                                          |                         |
| `interaction.sequence`    | 1-basierter Zähler von Interaktionen, gezählt pro Claude Code Prozess statt pro Sitzung, wie für [`event.sequence`](#event-correlation-attributes) beschrieben                                                   |                         |
| `parent.source`           | Wie der Span seinen Trace-Parent erhielt: `env`, wenn er unter einem eingehenden `TRACEPARENT` verschachtelt wurde, `none`, wenn er seine eigene Trace gestartet hat. Erfordert Claude Code v2.1.268 oder später |                         |
| `interaction.duration_ms` | Wanduhr-Dauer des Durchgangs                                                                                                                                                                                     |                         |

**`claude_code.llm_request`**

| Attribut                         | Beschreibung                                                                                                                                                                                                                                                                                            | Gated durch                    |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | Modellkennung                                                                                                                                                                                                                                                                                           |                                |
| `gen_ai.system`                  | Immer `anthropic`. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                                                                           |                                |
| `gen_ai.request.model`           | Gleicher Wert wie `model`. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                                                                   |                                |
| `query_source`                   | Subsystem, das die Anfrage gestellt hat, wie `repl_main_thread` oder ein Subagent-Name                                                                                                                                                                                                                  | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | Begrenzte Form von `query_source`, ausgegeben, ob detailliertes Beta-Tracing aktiv ist oder nicht, mit Werten wie `repl_main_thread` oder `agent.builtin.general-purpose`. `:` wird zu `.` und benutzerdefinierte Agenten erscheinen als `agent.custom`. Erfordert Claude Code v2.1.268 oder später     |                                |
| `agent_id`                       | Kennung des Subagenten oder Teamkollegen, der die Anfrage gestellt hat. Fehlt in der Hauptsitzung                                                                                                                                                                                                       |                                |
| `parent_agent_id`                | Kennung des Agenten, der diesen erzeugt hat. Fehlt für die Hauptsitzung und für Agenten, die direkt von ihr erzeugt wurden                                                                                                                                                                              |                                |
| `workflow.run_id`                | Run-Kennung des [Workflow](/docs/de/workflows) Tool-Durchlaufs, der diesen Agenten erzeugt hat, mit dem Präfix `wf_`. Fehlt für Agenten, die nicht durch einen Workflow erzeugt wurden                                                                                                                       |                                |
| `workflow.name`                  | Name des Workflows, der diesen Agenten erzeugt hat. Benutzerdefinierte Namen werden durch `custom` ersetzt, es sei denn, das Gate ist gesetzt                                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` oder `normal`                                                                                                                                                                                                                                                                                    |                                |
| `effort`                         | [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) auf die Anfrage angewendet: `low`, `medium`, `high`, `xhigh` oder `max`. Fehlt, wenn Claude Code keine Anstrengungsstufe sendet, zum Beispiel auf einem Modell, das Anstrengung nicht unterstützt. Erfordert Claude Code v2.1.274 oder später |                                |
| `llm_request.context`            | `interaction`, `tool` oder `standalone` je nach übergeordnetem Span                                                                                                                                                                                                                                     |                                |
| `duration_ms`                    | Wanduhr-Dauer einschließlich Wiederholungen                                                                                                                                                                                                                                                             |                                |
| `ttft_ms`                        | Zeit bis zum ersten Token in Millisekunden                                                                                                                                                                                                                                                              |                                |
| `first_content_ms`               | Zeit vom Anfrageanfang bis zum ersten Inhaltsblock des erfolgreichen Versuchs, in Millisekunden. Fehlt bei Anfragen, die auf den nicht-Streaming-Pfad zurückgegriffen haben. Erfordert Claude Code v2.1.268 oder später                                                                                 |                                |
| `input_tokens`                   | Eingabe-Token-Anzahl aus dem API-Nutzungsblock                                                                                                                                                                                                                                                          |                                |
| `output_tokens`                  | Ausgabe-Token-Anzahl                                                                                                                                                                                                                                                                                    |                                |
| `cache_read_tokens`              | Aus dem Prompt-Cache gelesene Token                                                                                                                                                                                                                                                                     |                                |
| `cache_creation_tokens`          | In den Prompt-Cache geschriebene Token                                                                                                                                                                                                                                                                  |                                |
| `request_id`                     | Anthropic API-Anfrage-ID aus dem `request-id` Response-Header                                                                                                                                                                                                                                           |                                |
| `gen_ai.response.id`             | Gleicher Wert wie `request_id`. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                                                              |                                |
| `client_request_id`              | Client-generierte `x-client-request-id` des letzten Versuchs                                                                                                                                                                                                                                            |                                |
| `attempt`                        | Gesamtzahl der Versuche für diese Anfrage                                                                                                                                                                                                                                                               |                                |
| `success`                        | `true` oder `false`                                                                                                                                                                                                                                                                                     |                                |
| `status_code`                    | HTTP-Statuscode, wenn die Anfrage fehlgeschlagen ist                                                                                                                                                                                                                                                    |                                |
| `error`                          | Fehlermeldung, wenn die Anfrage fehlgeschlagen ist                                                                                                                                                                                                                                                      |                                |
| `error_class`                    | Kurzes Fehlerklassen-Token, wenn die Anfrage fehlgeschlagen ist, wie `api_timeout` oder `server_overload`. Erfordert Claude Code v2.1.268 oder später                                                                                                                                                   |                                |
| `response.has_tool_call`         | `true`, wenn die Antwort Tool-Use-Blöcke enthielt                                                                                                                                                                                                                                                       |                                |
| `stop_reason`                    | API-Antwort `stop_reason`, wie `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn` oder `refusal`                                                                                                                                                                                       |                                |
| `gen_ai.response.finish_reasons` | Gleicher Wert wie `stop_reason`, in einem String-Array verpackt. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                             |                                |

Jeder Wiederholungsversuch wird auch als `gen_ai.request.attempt` Span-Ereignis mit `attempt` und `client_request_id` Attributen aufgezeichnet.

**`claude_code.tool`**

| Attribut              | Beschreibung                                                                                                                                                                                                                                                                                                                                               | Gated durch             |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Tool-Name                                                                                                                                                                                                                                                                                                                                                  |                         |
| `tool_name_safe`      | Form von `tool_name`, die keine benutzerdefinierte Namen trägt. Integrierte Tool-Namen werden wörtlich weitergegeben. MCP-Tool-Namen erscheinen als `mcp_other`, außer Tool-Namen, die bestimmte feste Formen entsprechen, wie `playwright` Tools mit dem Namen `browser_*`, die wörtlich weitergegeben werden. Erfordert Claude Code v2.1.268 oder später |                         |
| `bash_command_class`  | Für das Bash-Tool: Kategorie des ersten Programms des Befehls aus einer festen Liste, wie `vcs` oder `package_manager`. `other` für ein Programm außerhalb der Liste, `unparsed`, wenn die Zeile nicht geparst werden kann. Erfordert Claude Code v2.1.268 oder später                                                                                     |                         |
| `bash_argv0`          | Für das Bash-Tool: das erste Programm des Befehls, wenn es sich auf der gleichen festen Liste befindet, wie `git` oder `npm`. `other` für jedes Programm außerhalb der Liste. Erfordert Claude Code v2.1.268 oder später                                                                                                                                   |                         |
| `duration_ms`         | Wanduhr-Dauer einschließlich Berechtigungswartung und Ausführung                                                                                                                                                                                                                                                                                           |                         |
| `result_tokens`       | Ungefähre Token-Größe des Tool-Ergebnisses                                                                                                                                                                                                                                                                                                                 |                         |
| `agent_id`            | Kennung des Subagenten oder Teamkollegen, der das Tool ausgeführt hat. Fehlt in der Hauptsitzung                                                                                                                                                                                                                                                           |                         |
| `parent_agent_id`     | Kennung des Agenten, der diesen erzeugt hat. Fehlt für die Hauptsitzung und für Agenten, die direkt von ihr erzeugt wurden                                                                                                                                                                                                                                 |                         |
| `workflow.run_id`     | Run-Kennung des Workflow-Tool-Durchlaufs, der diesen Agenten erzeugt hat, mit dem Präfix `wf_`. Fehlt für Agenten, die nicht durch einen Workflow erzeugt wurden                                                                                                                                                                                           |                         |
| `workflow.name`       | Name des Workflows, der diesen Agenten erzeugt hat. Benutzerdefinierte Namen werden durch `custom` ersetzt, es sei denn, das Gate ist gesetzt                                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | Die Modell-`tool_use` Block-ID für diesen Aufruf. Entspricht der `tool_use_id` bei den [tool\_result](#tool-result-event) und [tool\_decision](#tool-decision-event) Ereignissen und in Hook-Payloads, sodass Sie den Span mit diesen Datensätzen verknüpfen können                                                                                        |                         |
| `gen_ai.tool.call.id` | Gleicher Wert wie `tool_use_id`. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                                                                                                                |                         |
| `file_path`           | Zieldateipfad für Read-, Edit- und Write-Tools                                                                                                                                                                                                                                                                                                             | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Befehlszeichenkette für das Bash-Tool                                                                                                                                                                                                                                                                                                                      | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Skill-Name für das Skill-Tool                                                                                                                                                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Subagent-Typ für das Agent-Tool oder veraltete Task-Tool                                                                                                                                                                                                                                                                                                   | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**`tool.output` Span-Ereignis auf `claude_code.tool`**

Wenn Sie `OTEL_LOG_TOOL_CONTENT=1` setzen, können Read- und Bash-Aufrufe ein `tool.output` Span-Ereignis auf dem `claude_code.tool` Span aufzeichnen. Edit- und Write-Aufrufe zeichnen eines nur auf, wenn Sie auch `OTEL_LOG_TOOL_DETAILS=1` setzen. Diese Variable ist nicht auf diese beiden Tools beschränkt, daher überprüfen Sie ihre [Zeile in der Konfigurationstabelle](#common-configuration-variables) für die Argumente, die sie an anderer Stelle hinzufügt.

Claude Code schreibt dieses Ereignis aus einer erfolgreichen Rückgabe eines Tool-Aufrufs, daher zeichnet ein Aufruf, der einen Fehler auslöst, nichts auf, unabhängig vom Tool. Unter den Aufrufen, die zurückgeben, zeichnet es kein `tool.output` Ereignis auf für:

* Ein Aufruf an ein anderes Tool als Read, Edit, Write und Bash, einschließlich MCP-Tools und WebFetch
* Ein Read, das etwas anderes als Dateitext zurückgibt, wie ein Bild, ein PDF oder ein erneutes Lesen einer Datei, deren Inhalte sich nicht geändert haben
* Ein Edit- oder Write-Aufruf, es sei denn, Sie setzen auch `OTEL_LOG_TOOL_DETAILS=1`

Das Ereignis trägt diese Attribute, jeweils gekürzt bei der Inhaltsbegrenzung (Standard: 60 KB). `Gated durch` nennt die Variable, die ein Attribut zusätzlich zu `OTEL_LOG_TOOL_CONTENT=1` benötigt, und für Edit und Write gated diese Variable das Ereignis selbst statt des Attributs.

| Attribut       | Beschreibung                                                                                               | Gated durch                                |
| -------------- | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| `content`      | Text, den das Read-Tool zurückgegeben hat, oder Text, den ein Write-Aufruf aufgefordert wurde zu schreiben | `OTEL_LOG_TOOL_DETAILS` für das Write-Tool |
| `output`       | Kombinierte Ausgabe eines Bash-Befehls, mit stderr in stdout verschachtelt                                 |                                            |
| `diff`         | Strukturierter Patch, den das Edit-Tool angewendet hat                                                     | `OTEL_LOG_TOOL_DETAILS`                    |
| `file_path`    | Zieldateipfad für die Read-, Edit- und Write-Tools, wiederholend das Span-Attribut desselben Namens        | `OTEL_LOG_TOOL_DETAILS`                    |
| `bash_command` | Befehlszeichenkette für das Bash-Tool                                                                      | `OTEL_LOG_TOOL_DETAILS`                    |

Das `tool_name` Attribut des übergeordneten Spans sagt Ihnen, von welchem Tool ein Ereignis stammt. Ein Attribut, das bei der Inhaltsbegrenzung gekürzt wird, wird von `<attribute>_truncated` und `<attribute>_original_length` begleitet.

**`claude_code.tool.blocked_on_user`**

| Attribut      | Beschreibung                                                                              | Gated durch |
| ------------- | ----------------------------------------------------------------------------------------- | ----------- |
| `duration_ms` | Zeit, die auf die Berechtigungsentscheidung gewartet wird                                 |             |
| `decision`    | `accept` oder `reject`                                                                    |             |
| `source`      | Entscheidungsquelle, entsprechend dem [Tool-Entscheidungs-Ereignis](#tool-decision-event) |             |

**`claude_code.tool.execution`**

| Attribut              | Beschreibung                                                                                                                                                                                                                                                                   | Gated durch             |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| `duration_ms`         | Zeit, die für die Ausführung des Tool-Body aufgewendet wird                                                                                                                                                                                                                    |                         |
| `tool_use_id`         | Gleicher Wert wie auf dem übergeordneten `claude_code.tool` Span                                                                                                                                                                                                               |                         |
| `gen_ai.tool.call.id` | Gleicher Wert wie `tool_use_id`. OpenTelemetry GenAI semantische Konvention                                                                                                                                                                                                    |                         |
| `success`             | `true` oder `false`                                                                                                                                                                                                                                                            |                         |
| `error`               | Fehler-Kategoriezeichenkette, wenn die Ausführung fehlgeschlagen ist, wie `Error:ENOENT` oder `ShellError`. Enthält die vollständige Fehlermeldung, wenn das Gate gesetzt ist                                                                                                  | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | Die Fehlerklasse in Identifierform, mit Zeichen außerhalb von Buchstaben, Ziffern und Unterstrichen ersetzt durch `_`, wie `Error_ENOENT` oder `ShellError`. Trägt die Kategorie auch, wenn `error` die vollständige Meldung trägt. Erfordert Claude Code v2.1.268 oder später |                         |

**`claude_code.hook`**

Dieser Span wird nur ausgegeben, wenn detailliertes Beta-Tracing aktiv ist, was `ENABLE_BETA_TRACING_DETAILED=1` und `BETA_TRACING_ENDPOINT` erfordert, ein Paar, das auch [ändert, wohin Ihre Logs und Traces gehen](/docs/de/env-vars#variables). Setzen Sie das Paar in Ihrer Shell, Benutzereinstellungen oder verwalteten Einstellungen; beide Variablen werden in [Projekt- und lokalen Einstellungen](/docs/de/settings-reference#variables-claude-code-ignores-in-env) ignoriert. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` allein erzeugt es nicht.

In interaktiven CLI-Sitzungen erfordert detailliertes Beta-Tracing auch, dass Ihre Organisation für die Funktion auf die Whitelist gesetzt ist. Agent SDK und nicht-interaktive `-p` Sitzungen erfordern keine Whitelist.

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

<span id="new-context-gates" />

<Note>
  Zusätzliche inhaltshaltige Attribute wie `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` und `response.model_output` werden nur ausgegeben, wenn detailliertes Beta-Tracing aktiv ist. Sie sind nicht Teil des stabilen Span-Schemas.

  Das Gate auf `new_context` hängt davon ab, welcher Span es trägt, und jede Kopie wird bei der Inhaltsbegrenzung gekürzt (Standard: 60 KB). Auf dem `claude_code.tool` Span trägt es das Ergebnis dieses Tool-Aufrufs, unabhängig vom Tool, und erfordert `OTEL_LOG_TOOL_CONTENT=1`. Auf dem `claude_code.interaction` Span trägt es den Benutzer-Prompt, und auf dem `claude_code.llm_request` Span die neuen Benutzer-Nachrichten und Tool-Ergebnisse dieser Anfrage. Beide erfordern `OTEL_LOG_USER_PROMPTS=1`.

  `user_system_prompt` erfordert zusätzlich `OTEL_LOG_USER_PROMPTS=1`. Es trägt nur den System-Prompt-Text, den Sie über die `systemPrompt` SDK-Option oder die Flags `--system-prompt` und `--append-system-prompt` bereitstellen, gekürzt bei der Inhaltsbegrenzung (Standard: 60 KB), und wird einmal pro Sitzung statt pro Anfrage ausgegeben.
</Note>

<h3 id="dynamic-headers">
  Dynamische Header
</h3>

Für Unternehmensumgebungen, die eine dynamische Authentifizierung erfordern, können Sie ein Skript konfigurieren, um Header dynamisch zu generieren. Dynamische Header gelten nur für die Protokolle `http/protobuf` und `http/json`. Mit dem `grpc` Protokoll verwendet Claude Code nur die statischen Header-Variablen, `OTEL_EXPORTER_OTLP_HEADERS` und seine signalspezifischen Varianten.

<h4 id="settings-configuration">
  Einstellungskonfiguration
</h4>

Fügen Sie zu Ihrer `.claude/settings.json` hinzu, ersetzen Sie den Pfad durch Ihren eigenen Skript:

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
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

Wenn das Helper-Skript fehlschlägt oder eine Ausgabe druckt, die diese Anforderungen nicht erfüllt, exportiert Claude Code nicht und Ihr Telemetrie-Backend erhält nichts aus der Sitzung, bis das Helper-Skript wieder funktioniert. Claude Code meldet den Fehler in:

* Eine Warnbenachrichtigung in interaktiven Sitzungen, [`otelHeadersHelper failed; telemetry is not being exported`](/docs/de/errors#otelheadershelper-failed), wird einmal pro Sitzung angezeigt, wenn das Helper-Skript zuerst fehlschlägt
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

Claude Code fügt diese Werte als Attribute auf jedem Metrik-Datenpunkt und Ereignisdatensatz an, zusätzlich zum Senden im OTLP-Ressourcenblock. Da die meisten Metriken-Backends Datenpunkt-Attribute als abfragbare Labels verfügbar machen, können Sie Metriken direkt nach Ihren benutzerdefinierten Schlüsseln gruppieren und filtern. Außer für die `vcs.*` [Repository-Attribute](#repository-attributes) überschreiben benutzerdefinierte Schlüssel niemals die [Standardattribute](#standard-attributes) wie `user.id` oder `session.id`: Wenn ein Schlüssel kollidiert, behält Claude Code den integrierten Wert.

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

Setzen Sie diese Umgebungsvariablen vor dem Ausführen von `claude`. Jedes Szenario unten zeigt eine vollständige Konfiguration, und jede Variable wird unter [Allgemeine Konfigurationsvariablen](#common-configuration-variables) beschrieben. Um zu bestätigen, dass eine Konfiguration wirksam wurde, überprüfen Sie Ihr Backend auf die `claude_code.session.count` Metrik nach dem Starten einer Sitzung; der [Schnellstart](#quick-start) behandelt Logs-only-Verifizierung und was zu überprüfen ist, wenn nichts ankommt.

Für Console-Debugging mit einem 1-Sekunden-Exportintervall:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

Für OTLP über gRPC:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Für Prometheus, gescraped von `http://localhost:9464/metrics`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

In einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments-reference#pass-through-session-child-metrics) bindet die Sitzung Port 9464 nur bei der Standard-Kapazität des Runners von eins. Bei höherer Kapazität stellt der Runner Sitzungszähler und Gauges stattdessen auf seinem eigenen `/metrics` Endpunkt erneut bereit.

Um Metriken an mehrere Exporter zu senden:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

Um Metriken und Logs an verschiedene Endpunkte oder Backends zu senden:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

Um nur Metriken zu exportieren, ohne Ereignisse oder Logs:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Um nur Ereignisse und Logs zu exportieren, ohne Metriken:

```bash theme={null}
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

| Attribut                                                                                | Beschreibung                                                                                                                                                                                                                                                                                      | Gesteuert durch                                                                                 |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `session.id`                                                                            | Eindeutige Sitzungskennung                                                                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_SESSION_ID` (Standard: true)                                              |
| `app.version`                                                                           | Aktuelle Claude Code-Version                                                                                                                                                                                                                                                                      | `OTEL_METRICS_INCLUDE_VERSION` (Standard: false)                                                |
| `app.entrypoint`                                                                        | Wie die Sitzung gestartet wurde, z. B. `cli`, `sdk-cli`, `sdk-ts`, `sdk-py` oder `claude-vscode`                                                                                                                                                                                                  | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (Standard: false)                                             |
| `organization.id`                                                                       | Organisations-UUID (wenn authentifiziert)                                                                                                                                                                                                                                                         | Immer enthalten, wenn verfügbar                                                                 |
| `user.account_uuid`                                                                     | Konto-UUID (wenn authentifiziert)                                                                                                                                                                                                                                                                 | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (Standard: true)                                            |
| `user.account_id`                                                                       | Konto-ID im getaggten Format, das Anthropic-Admin-APIs entspricht (wenn authentifiziert), z. B. `user_01BWBeN28...`                                                                                                                                                                               | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (Standard: true)                                            |
| `user.id`                                                                               | Zufällige anonyme Kennung, die beim ersten Ausführen generiert und in `~/.claude.json` gespeichert wird. Sie enthält keine persönlichen Informationen und wird nicht von Ihrem Claude-Konto abgeleitet. Das Löschen der Datei erzeugt beim nächsten Ausführen einen neuen, nicht verwandten Wert. | Immer enthalten                                                                                 |
| `user.email`                                                                            | E-Mail-Adresse des Benutzers, von Ihrer Anmeldung oder in einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web) von den Anmeldedaten der Sitzung selbst                                                                                                                                               | Immer enthalten, wenn verfügbar                                                                 |
| `terminal.type`                                                                         | Terminaltyp, z. B. `iTerm.app`, `vscode`, `cursor` oder `tmux`                                                                                                                                                                                                                                    | Immer enthalten, wenn erkannt                                                                   |
| Schlüssel aus `OTEL_RESOURCE_ATTRIBUTES`                                                | Benutzerdefinierte Attribute, die Sie festlegen, z. B. `department` oder `team.id`. Siehe [Unterstützung für Multi-Team-Organisationen](#multi-team-organization-support)                                                                                                                         | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (Standard: true)                                     |
| `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name`, `vcs.provider.name` | Die Identität des Sitzungs-Repositorys, abgeleitet von seinem `origin`-Remote. Siehe [Repository-Attribute](#repository-attributes)                                                                                                                                                               | `OTEL_METRICS_INCLUDE_REPOSITORY` (Standard: false). Erfordert Claude Code v2.1.269 oder später |

Wenn Claude Code bei einem [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) angemeldet ist, versieht die CLI Exporte mit der authentifizierten Identität aus der Gateway-Sitzung: `user.id` ist das IdP-Subjekt statt einer anonymen Installationskennung, `user.email` ist die angemeldete E-Mail, und `user.groups` enthält die IdP-Gruppenmitgliedschaft als kommagetrennte Zeichenkette. Jeder Export enthält auch `identity.source: gateway-oidc`. Die Gateway-Identität wird zuletzt angewendet, daher werden `user.*`- und `identity.*`-Schlüssel, die über `OTEL_RESOURCE_ATTRIBUTES` festgelegt werden, bei Gateway-Sitzungen ignoriert.

Ereignisse enthalten zusätzlich die folgenden Attribute. Diese werden niemals an Metriken angehängt, da sie zu unbegrenzter Kardinalität führen würden:

* `prompt.id`: UUID, die eine Benutzereingabeaufforderung mit allen nachfolgenden Ereignissen bis zur nächsten Eingabeaufforderung korreliert. Siehe [Ereigniskorrelationsattribute](#event-correlation-attributes).
* `workspace.host_paths`: Host-Arbeitsverzeichnisse, die in der Desktop-App ausgewählt wurden, als String-Array
* `workflow.run_id`: Laufkennung mit dem Präfix `wf_` auf den API- und Tool-Ereignissen, die von Agenten ausgegeben werden, die zu einem [Workflow](/docs/de/workflows)-Tool-Lauf gehören. Das Filtern von Ereignissen nach einer `workflow.run_id` rekonstruiert die API-Anfragen und Tool-Ergebnisse dieses Laufs. Die Kennung umfasst die Agenten, die das Workflow-Skript erzeugt, und alle Agenten, die diese wiederum erzeugen, z. B. Skill-Aufrufe. Sie entspricht der Laufkennung, die im Workflow-Tool-Ergebnis gemeldet wird. Fehlt bei allen anderen Ereignissen. Erfordert Claude Code v2.1.202 oder später
* `workflow.name`: Name des Workflows, das `meta.name` des Skripts, ausgegeben zusammen mit `workflow.run_id`. Integrierte Workflow-Namen werden wörtlich angezeigt, wenn der Lauf das unveränderte integrierte Skript ausführt. Benutzerdefinierte Namen, einschließlich bearbeiteter Kopien integrierter Skripte, werden durch `custom` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt. Erfordert Claude Code v2.1.202 oder später

<h4 id="repository-attributes">
  Repository-Attribute
</h4>

Setzen Sie `OTEL_METRICS_INCLUDE_REPOSITORY=true`, um Metriken und Ereignisse mit der Identität des Sitzungs-Repositorys zu taggen, damit ein gemeinsamer Collector die Nutzung pro Repository zuordnen kann. Erfordert Claude Code v2.1.269 oder später.

Claude Code leitet diese Attribute einmal pro Sitzung vom `origin`-Remote des Repositorys ab. Die HTTPS- und SSH-Remotes eines Repositorys erzeugen identische Werte:

| Attribut                  | Wert                                                                                                                                                            |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vcs.repository.url.full` | Die Browser-URL des Repositorys ohne `.git`, z. B. `https://github.com/example-org/example-repo`                                                                |
| `vcs.owner.name`          | Der Besitzer oder Gruppenpfad, z. B. `example-org`; weggelassen, wenn der Remote-Pfad ein einzelnes Segment hat                                                 |
| `vcs.repository.name`     | Der bloße Repository-Name, z. B. `example-repo`                                                                                                                 |
| `vcs.provider.name`       | `github`, `gitlab`, `bitbucket` oder `gitea`, wenn Claude Code den Host oder die URL-Form des Remote als einen dieser Anbieter erkennt; andernfalls weggelassen |

Werte werden in Kleinbuchstaben konvertiert, und Anmeldedaten, Abfragezeichenfolgen und Fragmente aus der Remote-URL erscheinen niemals darin. Die Attribute werden weggelassen, wenn die Sitzung keinen `origin`-Remote hat, wenn der Remote nicht URL-förmig ist, oder wenn das einzige umschließende Repository Ihr Home-Verzeichnis ist.

Ein `vcs.*`-Schlüssel, den Sie in [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) deklarieren, ersetzt den abgeleiteten Wert für diesen Schlüssel. Wenn Sie `vcs.repository.url.full` deklarieren, liest Claude Code den Remote nie und meldet nur die Schlüssel, die Sie deklarieren.

Die Attribute fließen nur zu Ihren eigenen Exportern; Anthropic's Telemetrie verwirft jeden `vcs.*`-Schlüssel.

<h3 id="metrics">
  Metriken
</h3>

Claude Code exportiert die folgenden Metriken. Die Spalte „Unit" zeigt die OpenTelemetry-Einheitenzeichenkette, die an jede Metrik angehängt wird; Zählmetriken haben keine.

| Metrikname                            | Beschreibung                                                         | Einheit |
| ------------------------------------- | -------------------------------------------------------------------- | ------- |
| `claude_code.session.count`           | Anzahl der gestarteten CLI-Sitzungen                                 | keine   |
| `claude_code.lines_of_code.count`     | Anzahl der geänderten Codezeilen                                     | keine   |
| `claude_code.pull_request.count`      | Anzahl der erstellten Pull Requests                                  | keine   |
| `claude_code.commit.count`            | Anzahl der erstellten Git-Commits                                    | keine   |
| `claude_code.cost.usage`              | Kosten der Claude Code-Sitzung                                       | USD     |
| `claude_code.token.usage`             | Anzahl der verwendeten Token                                         | tokens  |
| `claude_code.code_edit_tool.decision` | Anzahl der Genehmigungsentscheidungen für das Code-Bearbeitungs-Tool | keine   |
| `claude_code.active_time.total`       | Gesamtaktive Zeit                                                    | s       |

Wenn `prometheus` der einzige in `OTEL_METRICS_EXPORTER` aufgelistete Exporter ist, lässt Claude Code die Einheiten `USD`, `tokens` und `s` aus den exportierten Metriken weg, damit der Scrape im gültigen Prometheus-Textformat bleibt. Metriknamen ändern sich nicht, und Konfigurationen, die Exporter kombinieren, z. B. `otlp,prometheus`, behalten die Einheiten. Vor v2.1.216 enthielt der Prometheus-Scrape OpenMetrics-only `# UNIT`-Zeilen, die einige Scraper ablehnten.

<h3 id="metric-details">
  Metrikdetails
</h3>

Jede Metrik enthält die oben aufgelisteten Standardattribute. Metriken mit zusätzlichen kontextspezifischen Attributen werden unten vermerkt.

<h4 id="session-counter">
  Sitzungszähler
</h4>

Wird zu Beginn jeder Sitzung erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `start_type`: Wie die Sitzung gestartet wurde. Einer von `"fresh"`, `"resume"`, `"continue"` oder `"agents_view"`. Der Wert `"agents_view"` identifiziert den `claude agents`-Dashboard-Prozess, eine von Benutzern gestartete lokale UI statt einer Gesprächssitzung. Filtern Sie nach diesem Wert, um UI-Prozessstartvorgänge von Gesprächssitzungen in Ihren Dashboards zu trennen.

<h4 id="lines-of-code-counter">
  Codezeilen-Zähler
</h4>

Wird erhöht, wenn Code hinzugefügt oder entfernt wird.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Modellkennung für das Modell, das die Änderung vorgenommen hat (z. B. „claude-sonnet-5")

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
* `model`: Modellkennung (z. B. „claude-sonnet-5")
* `query_source`: Kategorie des Subsystems, das die Anfrage gestellt hat. Einer von `"main"`, `"subagent"` oder `"auxiliary"`
* `speed`: `"fast"`, wenn die Anfrage den schnellen Modus verwendet hat. Andernfalls nicht vorhanden
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird: `"low"`, `"medium"`, `"high"`, `"xhigh"` oder `"max"`. Nicht vorhanden, wenn Claude Code keine Anstrengungsstufe sendet, z. B. bei einem Modell, das Anstrengung nicht unterstützt.
* `agent.name`: Subagent-Typ, der die Anfrage gestellt hat. Integrierte Agent-Namen und Agenten aus offiziellen Marketplace-Plugins werden wörtlich angezeigt. Andere benutzerdefinierte Agent-Namen werden durch `"custom"` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt. Nicht vorhanden, wenn die Anfrage nicht von einem benannten Subagent-Typ gestellt wurde.
* `skill.name`: Skill, der für die Anfrage aktiv ist, gesetzt durch das Skill-Tool, einen `/`-Befehl oder geerbt von einem erzeugten Subagenten. Integrierte, gebündelte, benutzerdefinierte und offizielle Marketplace-Plugin-Skill-Namen werden wörtlich angezeigt. Skill-Namen von Drittanbieter-Plugins werden durch `"third-party"` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt. Nicht vorhanden, wenn kein Skill aktiv ist.
* `plugin.name`: Besitzendes Plugin, wenn der aktive Skill oder Subagent von einem Plugin bereitgestellt wird. Offizielle Marketplace-Plugin-Namen werden wörtlich angezeigt. Drittanbieter-Plugin-Namen werden durch `"third-party"` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt. Nicht vorhanden, wenn weder der Skill noch der Subagent ein besitzendes Plugin hat.
* `marketplace.name`: Marketplace, von dem das besitzende Plugin installiert wurde. Nur für offizielle Marketplace-Plugins ausgegeben. Andernfalls nicht vorhanden.
* `mcp_server.name`: MCP-Server, dessen Tool-Ergebnis diese Anfrage verbraucht hat. Integrierte, claude.ai-proxied und offizielle Registry-Server-Namen werden wörtlich angezeigt. Benutzerkonfigurierte Server-Namen werden durch `"custom"` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt. Nicht vorhanden, wenn die Anfrage kein MCP-Tool-Ergebnis verbraucht hat. Vor v2.1.222 setzte Claude Code dieses Attribut bei jeder Anfrage nach einem MCP-Tool-Aufruf, nicht nur bei Anfragen, die ein Tool-Ergebnis verbrauchten, daher zeigen Dashboards, die es aggregieren, einen Rückgang nach dem Upgrade.
* `mcp_tool.name`: MCP-Tool, dessen Ergebnis diese Anfrage verbraucht hat, mit demselben Redaktions- und Versionsverhalten wie `mcp_server.name`. Nicht vorhanden, wenn die Anfrage kein MCP-Tool-Ergebnis verbraucht hat.

<h4 id="token-counter">
  Token-Zähler
</h4>

Wird nach jeder API-Anfrage erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Modellkennung (z. B. „claude-sonnet-5")
* `query_source`: Kategorie des Subsystems, das die Anfrage gestellt hat. Einer von `"main"`, `"subagent"` oder `"auxiliary"`
* `speed`: `"fast"`, wenn die Anfrage den schnellen Modus verwendet hat. Andernfalls nicht vorhanden
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird. Siehe [Kostenzähler](#cost-counter) für Details.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Redaktionsverhalten.

<h4 id="code-edit-tool-decision-counter">
  Code-Bearbeitungs-Tool-Entscheidungszähler
</h4>

Wird erhöht, wenn der Benutzer die Verwendung von Edit-, Write- oder NotebookEdit-Tools akzeptiert oder ablehnt.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `tool_name`: Tool-Name (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Benutzerentscheidung (`"accept"`, `"reject"`)
* `source`: Woher die Entscheidung kam. Einer von `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"` oder `"user_reject"`. Siehe das [Tool-Entscheidungsereignis](#tool-decision-event) für die Bedeutung jedes Wertes.
* `language`: Programmiersprache der bearbeiteten Datei, z. B. `"TypeScript"`, `"Python"`, `"JavaScript"` oder `"Markdown"`. Gibt `"unknown"` für nicht erkannte Dateierweiterungen zurück.

<h4 id="active-time-counter">
  Aktive Zeit-Zähler
</h4>

Verfolgt die tatsächliche Zeit, die Claude Code aktiv verwendet wird, ausschließlich Leerlaufzeit. Diese Metrik wird während Benutzerinteraktionen wie Tippen und Lesen von Antworten sowie während CLI-Verarbeitung wie Tool-Ausführung und KI-Antworterzeugung erhöht.

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `type`: `"user"` für Tastaturinteraktionen, `"cli"` für Tool-Ausführung und KI-Antworten

<h3 id="events">
  Ereignisse
</h3>

Claude Code exportiert die folgenden Ereignisse über OpenTelemetry-Protokolle/Ereignisse (wenn `OTEL_LOGS_EXPORTER` konfiguriert ist):

<h4 id="event-correlation-attributes">
  Ereigniskorrelationsattribute
</h4>

Wenn ein Benutzer eine Eingabeaufforderung einreicht, kann Claude Code mehrere API-Aufrufe tätigen und mehrere Tools ausführen. Das Attribut `prompt.id` ermöglicht es Ihnen, alle diese Ereignisse an die einzelne Eingabeaufforderung zurückzuverfolgen, die sie ausgelöst hat.

| Attribut            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt.id`         | UUID v4-Kennung, die alle Ereignisse verknüpft, die während der Verarbeitung einer einzelnen Benutzereingabeaufforderung erzeugt werden                                                                                                                                                                                                                                                                                                                                                                                                                |
| `event.sequence`    | 0-basierter Zähler zum Ordnen von Ereignissen, gezählt pro Claude Code-Prozess statt pro Sitzung                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `message.uuid`      | UUID der Nachricht, wie sie im Sitzungstranskript gespeichert ist, die Dateien `~/.claude/projects/*/*.jsonl`. Vorhanden auf `assistant_response`, auf `api_response_body` und auf `user_prompt` außer für Befehlsverteilungen, die null oder viele Nachrichten erzeugen können. Auf `assistant_response` und `api_response_body` ist dies der endgültige Transkripteintrag der Antwort, von dem die `parentUuid` des nächsten Zuges verkettet wird. Erfordert Claude Code v2.1.214 oder später, oder v2.1.274 oder später auf `api_response_body`     |
| `client_request_id` | Von Client generierte UUID, die als `x-client-request-id`-Request-Header gesendet wird. Vorhanden auf `api_request` und `api_error` bei First-Party-API-Verbindungen; nicht vorhanden bei Drittanbieter-Provider-Backends und wenn die Anfrage über den Fallback ohne Streaming erneut versucht wurde. Paart eine Anfrage mit ihrer Antwort und bleibt für Fehler wie Timeouts verfügbar, die nie eine Server-`request_id` erzeugt haben. Entspricht demselben Attribut auf der `llm_request`-Trace-Spanne. Erfordert Claude Code v2.1.214 oder später |

Um alle Aktivitäten zu verfolgen, die durch eine einzelne Eingabeaufforderung ausgelöst werden, filtern Sie Ihre Ereignisse nach einem bestimmten `prompt.id`-Wert. Dies gibt das user\_prompt-Ereignis, alle api\_request-Ereignisse und alle tool\_result-Ereignisse zurück, die während der Verarbeitung dieser Eingabeaufforderung aufgetreten sind.

`event.sequence` beginnt bei 0 jedes Mal, wenn ein Claude Code-Prozess startet, und zählt für die Lebensdauer dieses Prozesses. Es zählt weiter über `/clear`, das eine neue `session.id` zuweist. Wenn Sie [eine Sitzung fortsetzen, ohne sie zu verzweigen](/docs/de/how-claude-code-works#resume-or-fork-sessions), behält die Sitzung ihre `session.id`, nimmt aber ihre `event.sequence`-Werte vom Prozess, der sie fortgesetzt hat, daher kann innerhalb einer Sitzung ein späteres Ereignis einen niedrigeren Wert als ein früheres tragen oder einen wiederholen. Um die Ereignisse einer Sitzung zu ordnen, sortieren Sie nach `event.timestamp` und verwenden Sie `event.sequence`, um Ereignisse zu ordnen, die einen Zeitstempel teilen.

Für die Rekonstruktion auf Nachrichtenebene trägt jede Ereignisklasse einen Schlüssel, der einem Feld im Sitzungstranskript entspricht. Das Transkript-Eintragsformat ist [intern für Claude Code](/docs/de/sessions#where-transcripts-are-stored) und ändert sich zwischen Versionen, daher kann eine Pipeline, die auf diesen Feldern verknüpft, bei jeder Veröffentlichung unterbrochen werden; behandeln Sie die Verknüpfungen als versionsspezifisch statt als stabilen Vertrag:

* `message.uuid` auf `user_prompt`, `assistant_response` und `api_response_body`
* `request_id` auf den API-Ereignissen, gespeichert als `requestId` auf den Assistent-Einträgen des Transkripts
* `tool_use_id` auf `tool_result`- und `tool_decision`-Ereignissen

<h4 id="user-prompt-event">
  Benutzereingabeaufforderungs-Ereignis
</h4>

Protokolliert, wenn ein Benutzer eine Eingabeaufforderung einreicht.

**Ereignisname**: `claude_code.user_prompt`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `prompt_length`: Länge der Eingabeaufforderung
* `prompt`: Inhalt der Eingabeaufforderung. Standardmäßig redaktioniert. Setzen Sie `OTEL_LOG_USER_PROMPTS=1`, um sie einzuschließen
* `message.uuid`: UUID der resultierenden Benutzernachricht, die dem gespeicherten Transkripteintrag entspricht. Nicht vorhanden bei Befehlsverteilungen, die null oder viele Nachrichten erzeugen können. Erfordert Claude Code v2.1.214 oder später
* `command_name`: Befehlsname, wenn die Eingabeaufforderung einen aufruft. Integrierte und gebündelte Befehlsnamen wie `compact` oder `debug` werden wie geschrieben ausgegeben; Aliase wie `reset` werden wie eingegeben statt des kanonischen Namens ausgegeben. Benutzerdefinierte, Plugin- und MCP-Befehlsnamen werden zu `custom` oder `mcp` zusammengefasst, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt
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
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `response_length`: Länge des Antworttexts in Zeichen
* `response`: Antworttext, gekürzt auf das Inhaltslimit (standardmäßig 60 KB). Standardmäßig auf `<REDACTED>` redaktioniert. Setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=1`, um sie einzuschließen. Wenn `OTEL_LOG_ASSISTANT_RESPONSES` nicht gesetzt ist, steuert `OTEL_LOG_USER_PROMPTS` es stattdessen, daher setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=0`, um Antworten redaktioniert zu halten, während die Eingabeaufforderungs-Protokollierung aktiv ist
* `model`: Modellkennung (z. B. „claude-sonnet-5")
* `request_id`: Anthropic API-Anfrage-ID aus dem `request-id`-Header der Antwort. Nur vorhanden, wenn die API eine zurückgibt
* `message.uuid`: UUID des endgültigen Transkripteintrags der Antwort. Eine API-Antwort wird als ein Transkripteintrag pro Inhaltsblock gespeichert; dies ist der letzte, von dem die `parentUuid` des nächsten Zuges verkettet wird. Erfordert Claude Code v2.1.214 oder später
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name

<h4 id="tool-result-event">
  Tool-Ergebnis-Ereignis
</h4>

Protokolliert, wenn ein Tool die Ausführung abgeschlossen hat. Nicht ausgegeben, wenn der Tool-Aufruf abgelehnt wurde; siehe das [Tool-Entscheidungsereignis](#tool-decision-event) für Ablehnungen.

**Ereignisname**: `claude_code.tool_result`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `tool_name`: Name des Tools
* `tool_use_id`: Eindeutige Kennung für diesen Tool-Aufruf. Entspricht der `tool_use_id`, die an Hooks übergeben wird, und ermöglicht die Korrelation zwischen OTel-Ereignissen und Hook-erfassten Daten.
* `success`: `"true"` oder `"false"`
* `duration_ms`: Ausführungszeit in Millisekunden
* `error_type`: Fehler-Kategoriezeichenkette, wenn das Tool fehlgeschlagen ist, z. B. `"Error:ENOENT"` oder `"ShellError"`
* `error` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Vollständige Fehlermeldung, wenn das Tool fehlgeschlagen ist
* `decision_type`: Immer `"accept"`, da dieses Ereignis nur ausgegeben wird, nachdem das Tool ausgeführt wurde. Abgelehnte Aufrufe erzeugen kein Tool-Ergebnis
* `decision_source`: Woher die Genehmigungsentscheidung kam. Einer von `"config"`, `"hook"`, `"user_permanent"` oder `"user_temporary"`. Siehe das [Tool-Entscheidungsereignis](#tool-decision-event) für die Bedeutung jedes Wertes. Die nur-Ablehnung-Quellen `"user_abort"` und `"user_reject"` erscheinen niemals auf diesem Ereignis.
* `tool_input_size_bytes`: Größe der JSON-serialisierten Tool-Eingabe in Bytes
* `tool_result_size_bytes`: Größe des Tool-Ergebnisses in Bytes
* `mcp_server_scope`: MCP-Server-Scope-Kennung (für MCP-Tools)
* `vcs.ref.head.revision`, `vcs.ref.head.name`, `vcs.ref.head.type` (wenn `OTEL_LOG_TOOL_DETAILS=1`): die Commit-Identität eines erfolgreichen `git commit`-Laufs durch das Bash- oder PowerShell-Tool. `vcs.ref.head.revision` ist die Commit-SHA, `vcs.ref.head.name` ist der Branch, auf dem es committed wurde, und `vcs.ref.head.type` ist `branch`. Der Name und Typ werden weggelassen, wenn der Commit auf einem detached HEAD gemacht wurde. Erfordert Claude Code v2.1.269 oder später
* `tool_parameters` (wenn `OTEL_LOG_TOOL_DETAILS=1`): JSON-Zeichenkette mit Tool-spezifischen Parametern. Für die integrierten Server von Claude Desktop in Sitzungen, die Claude Desktop besitzt, ist das Paar `mcp_server_name`/`mcp_tool_name` auch ohne das Flag enthalten, die gleiche Host-erstellte Ausnahme wie das [Tool-Entscheidungsereignis](#tool-decision-event), erfordert Claude Code v2.1.214 oder später. Die Parameter variieren je nach Tool:
  * Für Bash-Tool: enthält `bash_command`, `full_command`, `timeout`, `description` und `dangerouslyDisableSandbox`, plus `git_commit_id` und `git_branch`, wenn ein `git commit`-Befehl erfolgreich ist. `git_commit_id` ist die vollständige Commit-SHA, wenn der Commit der HEAD der Arbeitsverzeichnis der Sitzung ist, und Gits abgekürzte SHA andernfalls. `git_branch` ist der Branch, auf dem es committed wurde, weggelassen auf einem detached HEAD
  * Für das Workspace-Bash-Tool der Desktop-App, das auch `tool_name` als `Bash` meldet: enthält nur `bash_command`, `full_command` und `timeout`
  * Für MCP-Tools: enthält `mcp_server_name`, `mcp_tool_name`
  * Für Skill-Tool: enthält `skill_name`
  * Für Agent-Tool oder Legacy-Task-Tool: enthält `subagent_type`
* `tool_input` (wenn `OTEL_LOG_TOOL_DETAILS=1`): JSON-serialisierte Tool-Argumente. Einzelne Werte über 512 Zeichen werden gekürzt, und die vollständige Nutzlast ist auf \~4 K Zeichen begrenzt. Gilt für alle Tools einschließlich MCP-Tools.

<h4 id="api-request-event">
  API-Anfrage-Ereignis
</h4>

Protokolliert für jede API-Anfrage an Claude.

**Ereignisname**: `claude_code.api_request`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `model`: Verwendetes Modell (z. B. „claude-sonnet-5")
* `cost_usd`: Geschätzte Kosten in USD
* `cost_usd_micros`: Geschätzte Kosten in Millionsten eines US-Dollars, ausgegeben als Ganzzahl
* `duration_ms`: Anfrage-Dauer in Millisekunden
* `input_tokens`: Anzahl der Eingabe-Token
* `output_tokens`: Anzahl der Ausgabe-Token
* `cache_read_tokens`: Anzahl der aus dem Cache gelesenen Token
* `cache_creation_tokens`: Anzahl der für die Cache-Erstellung verwendeten Token
* `request_id`: Anthropic API-Anfrage-ID aus dem `request-id`-Header der Antwort, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `client_request_id`: Von Client generierte UUID, die als `x-client-request-id`-Request-Header gesendet wird; siehe die Tabelle [Ereigniskorrelationsattribute](#event-correlation-attributes) für wann sie vorhanden ist. Erfordert Claude Code v2.1.214 oder später
* `speed`: `"fast"` oder `"normal"`, was angibt, ob der schnelle Modus aktiv war
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird: `"low"`, `"medium"`, `"high"`, `"xhigh"` oder `"max"`. Nicht vorhanden, wenn Claude Code keine Anstrengungsstufe sendet, z. B. bei einem Modell, das Anstrengung nicht unterstützt.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Redaktionsverhalten.

<h4 id="api-error-event">
  API-Fehler-Ereignis
</h4>

Protokolliert, wenn eine API-Anfrage an Claude fehlschlägt.

**Ereignisname**: `claude_code.api_error`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `model`: Verwendetes Modell (z. B. „claude-sonnet-5")
* `error`: Fehlermeldung
* `status_code`: HTTP-Statuscode als Zahl. Nicht vorhanden für Nicht-HTTP-Fehler wie Verbindungsfehler.
* `duration_ms`: Anfrage-Dauer in Millisekunden
* `attempt`: Gesamtzahl der Versuche, einschließlich der ursprünglichen Anfrage (`1` bedeutet, dass keine Wiederholungen aufgetreten sind)
* `request_id`: Anthropic API-Anfrage-ID aus dem `request-id`-Header der Antwort, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `client_request_id`: Von Client generierte UUID, die als `x-client-request-id`-Request-Header gesendet wird. Verfügbar auch wenn ein Fehler wie ein Timeout oder Verbindungsfehler nie eine Server-`request_id` erzeugt hat; siehe die Tabelle [Ereigniskorrelationsattribute](#event-correlation-attributes) für wann sie vorhanden ist. Erfordert Claude Code v2.1.214 oder später
* `speed`: `"fast"` oder `"normal"`, was angibt, ob der schnelle Modus aktiv war
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird. Nicht vorhanden, wenn Claude Code keine Anstrengungsstufe sendet, z. B. bei einem Modell, das Anstrengung nicht unterstützt.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Redaktionsverhalten.

<h4 id="api-refusal-event">
  API-Ablehnung-Ereignis
</h4>

Protokolliert, wenn eine API-Anfrage `stop_reason: "refusal"` zurückgibt. Ablehnungen kommen auf einem erfolgreichen Response-Stream statt als HTTP-Fehler an, daher wird das `api_error`-Ereignis nicht für sie ausgelöst. Dieses Ereignis ermöglicht es Ihnen, die Ablehnungshäufigkeit zu verfolgen und Ablehnungen nach denselben Attributen wie `api_request` und `api_error` zu gruppieren.

**Ereignisname**: `claude_code.api_refusal`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `model`: Modellkennung aus der Anfrage
* `request_id`: Anthropic API-Anfrage-ID aus dem `request-id`-Header der Antwort, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `query_source`: Subsystem, das die Anfrage gestellt hat, z. B. `"repl_main_thread"`, `"compact"` oder ein Subagent-Name. Siehe [`api_request`](#api-request-event) für Definitionen.
* `speed`: Entweder `"fast"`, wenn [Schneller Modus](/docs/de/fast-mode) aktiv ist, oder `"normal"`
* `attempt`: Wiederholungsversuch-Nummer. Der erste Versuch ist `1`.
* `effort`: [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die auf die Anfrage angewendet wird. Nicht vorhanden, wenn Claude Code keine Anstrengungsstufe sendet, z. B. bei einem Modell, das Anstrengung nicht unterstützt.
* `server_fallback_hop`: `true`, wenn das Server-seitige Modell-Fallback diesen Fehler bereits auf einem anderen Modell erneut versucht hat, daher hat der Benutzer diese bestimmte Ablehnung nicht gesehen. `false`, wenn die Anfrage in einer Ablehnung endete. Ein einzelner Zug kann sowohl ein `true`-Hop-Ereignis als auch ein späteres `false`-Finales Ereignis ausgeben, wenn das Fallback-Modell auch ablehnt.
* `has_category`: `true`, wenn die API-Antwort eine `stop_details.category` von `"cyber"`, `"bio"`, `"frontier_llm"` oder `"reasoning_extraction"` trug. `false`, wenn die Antwort keine Kategorie oder einen Wert außerhalb dieses Satzes trug. Nicht vorhanden, wenn `server_fallback_hop` `true` ist, da Hop-Blöcke keine `stop_details` tragen.
* `has_explanation`: `true`, wenn die API-Antwort eine `stop_details.explanation` trug, andernfalls `false`. Nicht vorhanden, wenn `server_fallback_hop` `true` ist.
* `category`: Der `stop_details.category`-Wert aus der API-Antwort. Einer von `"cyber"`, `"bio"`, `"frontier_llm"` oder `"reasoning_extraction"`. Nur vorhanden, wenn `OTEL_LOG_TOOL_DETAILS=1` gesetzt ist und `has_category` `true` ist.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Skill-, Plugin-, Agent- und MCP-Zuordnung für die Anfrage. Siehe [Kostenzähler](#cost-counter) für Definitionen und Redaktionsverhalten.

<h4 id="api-request-body-event">
  API-Anfrage-Body-Ereignis
</h4>

Protokolliert für jeden API-Anfrage-Versuch, wenn `OTEL_LOG_RAW_API_BODIES` gesetzt ist. Ein Ereignis wird pro Versuch ausgegeben, daher erzeugen Wiederholungen mit angepassten Parametern jeweils ihr eigenes Ereignis.

**Ereignisname**: `claude_code.api_request_body`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `body`: JSON-serialisierte Messages API-Anfrage-Parameter, wie die System-Eingabeaufforderung, Nachrichten und Tools, gekürzt auf das Inhaltslimit (standardmäßig 60 KB). Extended-Thinking-Inhalte in vorherigen Assistent-Zügen sind redaktioniert. Nur im Inline-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Absoluter Pfad zu einer `<dir>/<uuid>.request.json`-Datei, die den ungekürzte Body enthält. Nur im Datei-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Ungekürzte Body-Länge. UTF-8-Bytes, wenn `OTEL_LOG_RAW_API_BODIES=file:<dir>`, oder UTF-16-Code-Einheiten, wenn `=1`
* `body_truncated`: `"true"`, wenn Inline-Kürzung aufgetreten ist. Nicht vorhanden im Datei-Modus und wenn keine Kürzung aufgetreten ist.
* `model`: Modellkennung aus den Anfrage-Parametern
* `query_source`: Subsystem, das die Anfrage gestellt hat (z. B. `"compact"`)
* `request_body_id`: UUID, die den Body dieses Versuchs identifiziert. Das [`api_response_body`-Ereignis](#api-response-body-event) für den Versuch, der erfolgreich ist, trägt denselben Wert, daher können Sie eine Antwort mit der genauen Anfrage, die sie erzeugt hat, paaren. Erfordert Claude Code v2.1.274 oder später

<h4 id="api-response-body-event">
  API-Antwort-Body-Ereignis
</h4>

Protokolliert für jede erfolgreiche API-Antwort, wenn `OTEL_LOG_RAW_API_BODIES` gesetzt ist.

Im Datei-Modus (`OTEL_LOG_RAW_API_BODIES=file:<dir>`) hängt Claude Code auch eine JSON-Zeile an `<dir>/index.jsonl` für jede erfolgreiche Antwort an, mit den Feldern `timestamp`, `session_id`, `query_source`, `model`, `request_id`, `message_id`, `message_uuid`, `request_file` und `response_file`. Lesen Sie es, um die Anfrage- und Antwort-Dateien hinter einer bestimmten Transkript-Nachricht zu finden, ohne Ihren Telemetrie-Backend abzufragen. Die Index-Datei erfordert Claude Code v2.1.274 oder später.

**Ereignisname**: `claude_code.api_response_body`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `body`: JSON-serialisierte Messages API-Antwort, einschließlich der ID, Inhaltsblöcke, Nutzung und Stop-Grund, gekürzt auf das Inhaltslimit (standardmäßig 60 KB). Extended-Thinking-Inhalte sind redaktioniert. Nur im Inline-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Absoluter Pfad zu einer `<dir>/<request_id>.response.json`-Datei, die den ungekürzte Body enthält. Nur im Datei-Modus ausgegeben (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Ungekürzte Body-Länge. UTF-8-Bytes, wenn `OTEL_LOG_RAW_API_BODIES=file:<dir>`, oder UTF-16-Code-Einheiten, wenn `=1`
* `body_truncated`: `"true"`, wenn Inline-Kürzung aufgetreten ist. Nicht vorhanden im Datei-Modus und wenn keine Kürzung aufgetreten ist.
* `model`: Modellkennung
* `query_source`: Subsystem, das die Anfrage gestellt hat
* `request_id`: Anthropic API-Anfrage-ID aus dem `request-id`-Header der Antwort, z. B. `"req_011..."`. Nur vorhanden, wenn die API eine zurückgibt.
* `request_body_id`: Die `request_body_id` des [`api_request_body`-Ereignisses](#api-request-body-event), das diese Antwort beantwortet. Erfordert Claude Code v2.1.274 oder später
* `message.id`: Nachrichten-ID, die die API der Antwort zugewiesen hat, das `id`-Feld des Response-Body. Erfordert Claude Code v2.1.274 oder später
* `message.uuid`: UUID des endgültigen Transkripteintrags der Antwort. Zusammen mit `request_body_id` verknüpft es eine Transkript-Nachricht mit den Anfrage- und Antwort-Bodies dahinter. Erfordert Claude Code v2.1.274 oder später

<h4 id="tool-decision-event">
  Tool-Entscheidungs-Ereignis
</h4>

Protokolliert, wenn eine Tool-Genehmigungsentscheidung getroffen wird (akzeptieren/ablehnen).

**Ereignisname**: `claude_code.tool_decision`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `tool_name`: Name des Tools (z. B. „Read", „Edit", „Write", „NotebookEdit")
* `tool_use_id`: Eindeutige Kennung für diesen Tool-Aufruf. Entspricht der `tool_use_id`, die an Hooks übergeben wird, und ermöglicht die Korrelation zwischen OTel-Ereignissen und Hook-erfassten Daten.
* `decision`: Entweder `"accept"` oder `"reject"`
* `tool_source`: Immer vorhanden. Die Herkunft des Tools als geschlossener Satz von CLI-erstellten Werten. Erfordert Claude Code v2.1.214 oder später
  * `"builtin"`: die eigenen Tools der CLI
  * `"mcp"`: MCP-Server allgemein
  * `"sdk_host_builtin_mcp"`: ein In-Process-Server, der in Claude Desktop selbst integriert ist, in einer Sitzung, die Claude Desktop besitzt. Claude Desktop besitzt eine Sitzung, die es von einem seiner eigenen Einstiegspunkte gestartet hat, `claude-desktop`, `claude-desktop-3p` oder `local-agent`, wenn diese Sitzung kein verschachteltes Kind ist; verschachtelte Sitzungen, einschließlich Sitzungen, die Claude Code selbst erzeugt, melden diese Server als `"mcp"`
* `source`: Woher die Entscheidung kam:
  * `"config"`: Automatisch entschieden ohne Eingabeaufforderung, basierend auf Projekteinstellungen, Zulassungs- oder Ablehnungsregeln in den persönlichen Einstellungen des Benutzers, Unternehmensrichtlinie, `--allowedTools`- oder `--disallowedTools`-Flags, dem aktiven Genehmigungsmodus, einer Sitzungs-Zuschuss aus einer früheren Eingabeaufforderung in derselben interaktiven CLI-Sitzung, oder weil das Tool inhärent sicher ist. Das Ereignis zeigt nicht an, welche dieser Quellen übereinstimmte. Claude Code meldet auch `"config"`, wenn die Genehmigungsaufforderungs-Anfrage selbst fehlschlägt, z. B. wenn der [`canUseTool`](/docs/de/agent-sdk/typescript#canusetool)-Callback des Agent SDK oder das [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags)-Tool ein ungültiges Ergebnis zurückgibt, oder wenn der Eingabestrom geschlossen wird, während die Anfrage ausstehend ist. Vor v2.1.216 meldete Claude Code diese Fehler als `"user_reject"`.
  * `"hook"`: Ein `PreToolUse`- oder `PermissionRequest`-Hook gab die Entscheidung zurück.
  * `"user_permanent"`: Ausgegeben, wenn der Benutzer „Ja, und nicht mehr fragen für ..." bei einer Genehmigungsaufforderung wählte, was eine Zulassungsregel in seinen persönlichen Einstellungen speichert. In der interaktiven CLI wird dies nur für diese Wahl selbst ausgegeben; spätere Aufrufe, die die gespeicherte Regel erfüllen, geben stattdessen `"config"` aus. In Agent SDK- oder nicht-interaktiven `-p`-Sitzungen geben sowohl die ursprüngliche Wahl als auch spätere Regelübereinstimmungen `"user_permanent"` aus. Wird als Akzeptanz behandelt.
  * `"user_temporary"`: Ausgegeben, wenn der Benutzer „Ja" bei einer Genehmigungsaufforderung für eine einmalige Genehmigung wählte, oder eine Option wählte, die Zugriff für den Rest der Sitzung bei einer Datei-Bearbeitungs- oder Lese-Aufforderung gewährt. In der interaktiven CLI wird dies nur für die Wahl selbst ausgegeben; spätere Aufrufe, die durch diesen Sitzungs-Zuschuss erlaubt sind, geben stattdessen `"config"` aus. In Agent SDK- oder nicht-interaktiven `-p`-Sitzungen geben sowohl die Wahl als auch spätere Übereinstimmungen `"user_temporary"` aus. Wird als Akzeptanz behandelt.
  * `"user_abort"`: Ausgegeben, wenn der Benutzer die Genehmigungsaufforderung ohne Antwort schloss. In Agent SDK- und nicht-interaktiven `-p`-Sitzungen schließt dies das Unterbrechen des Zuges ein, während eine `canUseTool`- oder `--permission-prompt-tool`-Genehmigungsanfrage ausstehend ist; vor v2.1.216 meldete Claude Code diese Unterbrechung als `"user_reject"`. Wird als Ablehnung behandelt.
  * `"user_reject"`: Ausgegeben, wenn der Benutzer „Nein" bei einer Aufforderung wählte. In der interaktiven CLI wird dies nur für diese Wahl selbst ausgegeben; Aufrufe, die eine Ablehnungsregel in den persönlichen Einstellungen des Benutzers erfüllen, geben stattdessen `"config"` aus. In Agent SDK- oder nicht-interaktiven `-p`-Sitzungen geben Aufrufe, die eine Ablehnungsregel in persönlichen Einstellungen erfüllen, `"user_reject"` aus. Wird als Ablehnung behandelt.
* `tool_parameters` (wenn `OTEL_LOG_TOOL_DETAILS=1`): JSON-Zeichenkette mit Tool-spezifischen Parametern. Gleiche Form wie das [Tool-Ergebnis-Ereignis](#tool-result-event), minus Post-Ausführungs-Felder wie `git_commit_id`. Werte können sich von `tool_result` für einen akzeptierten Aufruf unterscheiden, wenn die Genehmigungsentscheidung die Tool-Eingabe über `updatedInput` umschreibt. Verwenden Sie dieses Attribut, um zu sehen, welcher Befehl abgelehnt wurde, wenn `decision` `"reject"` ist.
  * Für `"sdk_host_builtin_mcp"`-Tools: `mcp_server_name` und `mcp_tool_name` sind auch enthalten, wenn `OTEL_LOG_TOOL_DETAILS` aus ist, da die Host-Anwendung diese Namen definiert; ohne sie wäre ein abgelehnter Aufruf an einen dieser integrierten Server auf dem Standard-Stream nicht zuordenbar. Für benutzerkonfigurierte MCP-Server ist der `tool_name` des Ereignisses immer das Literal `"mcp_tool"`, und die Server- und Tool-Namen erscheinen nur in `tool_parameters` mit dem Flag an; Argument-Inhalte erfordern das Flag überall. Erfordert Claude Code v2.1.214 oder später
  * Für Bash-Tool: enthält `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`. Das Workspace-Bash-Tool der Desktop-App meldet auch `tool_name` als `Bash`, enthält aber nur `bash_command`, `full_command` und `timeout`
  * Für MCP-Tools: enthält `mcp_server_name`, `mcp_tool_name`
  * Für Skill-Tool: enthält `skill_name`
  * Für Agent-Tool oder Legacy-Task-Tool: enthält `subagent_type`

<h4 id="permission-mode-changed-event">
  Genehmigungsmodus-Änderungs-Ereignis
</h4>

Protokolliert, wenn sich der Genehmigungsmodus ändert, z. B. durch Shift+Tab-Zyklus, Beendigung des Plan-Modus oder eine Auto-Modus-Gate-Prüfung.

**Ereignisname**: `claude_code.permission_mode_changed`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `from_mode`: Der vorherige Genehmigungsmodus, z. B. `"default"`, `"plan"`, `"acceptEdits"`, `"auto"` oder `"bypassPermissions"`
* `to_mode`: Der neue Genehmigungsmodus
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
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `action`: `"login"` oder `"logout"`
* `success`: `"true"` oder `"false"`
* `auth_method`: Authentifizierungsmethode, z. B. `"oauth"`
* `error_category`: Kategorische Fehlerart, wenn die Aktion fehlgeschlagen ist. Die rohe Fehlermeldung ist niemals enthalten
* `status_code`: HTTP-Statuscode als Zeichenkette, wenn die Aktion mit einem HTTP-Fehler fehlgeschlagen ist

<h4 id="mcp-server-connection-event">
  MCP-Server-Verbindungs-Ereignis
</h4>

Protokolliert, wenn ein MCP-Server verbunden wird, die Verbindung trennt oder nicht verbunden werden kann.

**Ereignisname**: `claude_code.mcp_server_connection`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `status`: `"connected"`, `"failed"` oder `"disconnected"`
* `transport_type`: Server-Transport, z. B. `"stdio"`, `"sse"` oder `"http"`
* `server_scope`: Scope, auf dem der Server konfiguriert ist, z. B. `"user"`, `"project"` oder `"local"`
* `duration_ms`: Verbindungsversuch-Dauer in Millisekunden
* `error_code`: Fehlercode, wenn die Verbindung fehlgeschlagen ist
* `is_plugin`: `true`, wenn der Server von einem Plugin bereitgestellt wird, `false` andernfalls
* `plugin_id_hash` (wenn `is_plugin` `true` ist): Stabiler Hash des Plugin-Namens und des Marketplace, zum Gruppieren von Ereignissen nach Plugin ohne Offenlegung des Namens. Claude Code berechnet ihn wie unter dem [Plugin-Laden-Ereignis](#plugin-loaded-event) beschrieben
* `plugin.name` (wenn `is_plugin` `true` ist): Name des Plugins, das den Server bereitstellt. Für Drittanbieter-Plugins ist dieser die Literal-Zeichenkette `"third-party"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`; dies schützt Drittanbieter-Plugin-Namen davor, standardmäßig in Protokollen zu erscheinen. Plugins aus offiziellen Anthropic-Quellen werden immer nach Name identifiziert. Die Attribute `plugin_id_hash` und `plugin.name` fließen zu Ihrem eigenen Monitoring-Backend und werden nicht an Anthropic gesendet
* `server_name` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Konfigurierter Server-Name
* `error` (wenn `OTEL_LOG_TOOL_DETAILS=1`): Vollständige Fehlermeldung, wenn die Verbindung fehlgeschlagen ist

<h4 id="internal-error-event">
  Interner Fehler-Ereignis
</h4>

Protokolliert, wenn Claude Code einen unerwarteten internen Fehler abfängt. Nur der Fehlerklassen-Name und ein errno-ähnlicher Code werden aufgezeichnet. Die Fehlermeldung und Stack-Trace sind niemals enthalten. Dieses Ereignis wird nicht ausgegeben, wenn gegen Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry ausgeführt wird, oder wenn `DISABLE_ERROR_REPORTING` gesetzt ist.

**Ereignisname**: `claude_code.internal_error`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `error_name`: Fehlerklassen-Name, z. B. `"TypeError"` oder `"SyntaxError"`
* `error_code`: Node.js errno-Code wie `"ENOENT"`, wenn auf dem Fehler vorhanden

<h4 id="plugin-installed-event">
  Plugin-Installiert-Ereignis
</h4>

Protokolliert, wenn ein Plugin die Installation abgeschlossen hat, sowohl vom `claude plugin install`-CLI-Befehl als auch von der interaktiven `/plugin`-UI.

**Ereignisname**: `claude_code.plugin_installed`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `marketplace.is_official`: `"true"`, wenn der Marketplace ein offizieller Anthropic-Marketplace ist, `"false"` andernfalls
* `install.trigger`: `"cli"` oder `"ui"`
* `plugin.name`: Name des installierten Plugins. Für Drittanbieter-Marketplaces ist dies nur enthalten, wenn `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Plugin-Version, wenn im Marketplace-Eintrag deklariert. Für Drittanbieter-Marketplaces ist dies nur enthalten, wenn `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace, von dem das Plugin installiert wurde. Für Drittanbieter-Marketplaces ist dies nur enthalten, wenn `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Plugin-Geladen-Ereignis
</h4>

Protokolliert einmal pro aktiviertem Plugin beim Sitzungsstart. Verwenden Sie dieses Ereignis, um zu inventarisieren, welche Plugins über Ihre Flotte aktiv sind, als Ergänzung zu `plugin_installed`, das die Installationsaktion selbst aufzeichnet.

**Ereignisname**: `claude_code.plugin_loaded`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `plugin.name`: Name des Plugins. Für Plugins außerhalb des offiziellen Marketplace und des integrierten Bundles ist der Wert `"third-party"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace, von dem das Plugin installiert wurde, wenn bekannt. Unter derselben Bedingung wie `plugin.name` auf `"third-party"` redaktioniert
* `plugin.version`: Version aus dem Plugin-Manifest. Nur enthalten, wenn der Name nicht redaktioniert ist und das Manifest eine Version deklariert
* `plugin.scope`: Herkunftskategorie für das Plugin: `"official"`, `"community"`, `"org"`, `"user-local"` oder `"default-bundle"`
* `enabled_via`: wie das Plugin aktiviert wurde: `"default-enable"`, `"org-policy"`, `"admin-install"`, `"seed-mount"` oder `"user-install"`. Der Wert `"admin-install"` bedeutet, dass das Plugin in [**Organisationseinstellungen > Plugins & Skills**](https://claude.ai/admin-settings/skills?tab=inventory) für Ihre Organisation erforderlich oder automatisch installiert ist. Vor v2.1.246 meldete Claude Code diese Plugins als `"user-install"` oder `"seed-mount"`
* `plugin_id_hash`: deterministischer Hash des Plugin-Namens und des Marketplace, nur an Ihren konfigurierten Exporter gesendet. Ermöglicht es Ihnen, die unterschiedlichen Drittanbieter-Plugins, die über Ihre Flotte geladen werden, zu zählen, ohne ihre Namen aufzuzeichnen. Für [Plugins, die von claude.ai synchronisiert werden](/docs/de/plugins/loading#synced-plugins), hasht Claude Code den Plugin-Namen mit dem Marketplace-Namen, den claude.ai für das Plugin meldet, oder mit `synced` andernfalls. Vor v2.1.246 verwendete Claude Code den Marketplace-Namen, den claude.ai meldet, nicht im Hash
* `has_hooks`: ob das Plugin Hooks beiträgt
* `has_mcp`: ob das Plugin MCP-Server beiträgt
* `host_owned_mcp`: `true`, wenn der SDK-Host die MCP-Verbindungen dieses Plugins verwaltet und Claude Code das Lesen der MCP-Server-Konfiguration des Plugins übersprungen hat, `false` andernfalls. Erfordert Claude Code v2.1.172 oder später
* `skill_path_count`: Anzahl der Skill-Verzeichnisse, die das Plugin deklariert
* `command_path_count`: Anzahl der Befehlsverzeichnisse, die das Plugin deklariert
* `agent_path_count`: Anzahl der Agent-Verzeichnisse, die das Plugin deklariert
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Im sicheren Modus meldet dieses Ereignis nur konfigurierte Inventare; die Befehle, Skills, Hooks und MCP-Server des Plugins werden nicht geladen. Erfordert Claude Code v2.1.169 oder später

<h4 id="skill-activated-event">
  Skill-Aktiviert-Ereignis
</h4>

Protokolliert, wenn ein Skill aufgerufen wird, ob Claude ihn durch das Skill-Tool aufruft oder Sie ihn als `/`-Befehl ausführen.

**Ereignisname**: `claude_code.skill_activated`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `skill.name`: Name des Skills. Für benutzerdefinierte und Drittanbieter-Plugin-Skills ist der Wert der Platzhalter `"custom_skill"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Wie der Skill ausgelöst wurde (`"user-slash"`, `"claude-proactive"` oder `"nested-skill"`)
* `skill.source`: Woher der Skill geladen wurde (z. B. `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"`, wenn der Skill ein Workflow-Skill ist. Andernfalls nicht vorhanden
* `plugin.name` (wenn `OTEL_LOG_TOOL_DETAILS=1` oder das Plugin aus einem offiziellen Marketplace ist): Name des besitzenden Plugins, wenn der Skill von einem Plugin bereitgestellt wird
* `marketplace.name` (wenn `OTEL_LOG_TOOL_DETAILS=1` oder das Plugin aus einem offiziellen Marketplace ist): Marketplace, von dem das besitzende Plugin installiert wurde, wenn der Skill von einem Plugin bereitgestellt wird

<h4 id="at-mention-event">
  At-Mention-Ereignis
</h4>

Protokolliert, wenn Claude Code eine `@`-Erwähnung in einer Eingabeaufforderung auflöst. Nicht jede Erwähnung gibt ein Ereignis aus: Early-Exit-Pfade wie Genehmigungsablehnungen, übergroße Dateien, PDF-Referenz-Anhänge und Fehler beim Auflisten von Verzeichnissen geben zurück, ohne zu protokollieren.

**Ereignisname**: `claude_code.at_mention`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `mention_type`: Typ der Erwähnung (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`, `"peer"`). Der Wert `"peer"` bedeutet, dass Sie [eine Ihrer anderen Claude Code-Sitzungen](/docs/de/cross-session-messaging) erwähnt haben. Erfordert Claude Code v2.1.232 oder später
* `success`: Ob die Erwähnung erfolgreich aufgelöst wurde (`"true"` oder `"false"`)

<h4 id="api-retries-exhausted-event">
  API-Wiederholungen-Erschöpft-Ereignis
</h4>

Protokolliert einmal, wenn eine API-Anfrage nach mehr als einem Versuch fehlschlägt. Ausgegeben zusammen mit dem endgültigen `api_error`-Ereignis.

**Ereignisname**: `claude_code.api_retries_exhausted`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `model`: Verwendetes Modell
* `error`: Endgültige Fehlermeldung
* `status_code`: HTTP-Statuscode als Zahl. Nicht vorhanden für Nicht-HTTP-Fehler.
* `total_attempts`: Gesamtzahl der Versuche
* `total_retry_duration_ms`: Gesamte Wanduhr-Zeit über alle Versuche
* `speed`: `"fast"` oder `"normal"`

<h4 id="hook-registered-event">
  Hook-Registriert-Ereignis
</h4>

Protokolliert einmal pro konfiguriertem Hook beim Sitzungsstart. Verwenden Sie dieses Ereignis, um zu inventarisieren, welche Hooks über Ihre Flotte aktiv sind, als Ergänzung zu den Pro-Ausführungs-Ereignissen `hook_execution_start` und `hook_execution_complete`.

**Ereignisname**: `claude_code.hook_registered`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `hook_event`: Hook-Ereignistyp, z. B. `"PreToolUse"` oder `"PostToolUse"`
* `hook_type`: Hook-Implementierungstyp: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"` oder `"agent"`
* `hook_source`: Woher der Hook definiert ist: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"` oder `"pluginHook"`
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Erfordert Claude Code v2.1.169 oder später
* `hook_matcher` (wenn `OTEL_LOG_TOOL_DETAILS=1`): die Matcher-Zeichenkette aus der Hook-Konfiguration, wenn eine gesetzt ist
* `plugin.name` (wenn `hook_source` `"pluginHook"` ist): Name des beitragenden Plugins. Für Plugins außerhalb des offiziellen Marketplace und des integrierten Bundles ist der Wert `"third-party"`, es sei denn, `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (wenn `hook_source` `"pluginHook"` ist): deterministischer Hash des Plugin-Namens und des Marketplace, nur an Ihren konfigurierten Exporter gesendet. Ermöglicht es Ihnen, unterschiedliche beitragende Plugins zu zählen, ohne ihre Namen aufzuzeichnen. Claude Code berechnet ihn wie unter dem [Plugin-Geladen-Ereignis](#plugin-loaded-event) beschrieben

<h4 id="hook-execution-start-event">
  Hook-Ausführungs-Start-Ereignis
</h4>

Protokolliert, wenn ein oder mehrere Hooks für ein Hook-Ereignis beginnen auszuführen.

**Ereignisname**: `claude_code.hook_execution_start`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `hook_event`: Hook-Ereignistyp, z. B. `"PreToolUse"` oder `"PostToolUse"`
* `hook_name`: Vollständiger Hook-Name einschließlich Matcher, z. B. `"PreToolUse:Write"`
* `num_hooks`: Anzahl der übereinstimmenden Hook-Befehle
* `managed_only`: `"true"`, wenn nur verwaltete Richtlinien-Hooks zulässig sind
* `hook_source`: `"policySettings"` oder `"merged"`
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Erfordert Claude Code v2.1.169 oder später
* `hook_definitions`: JSON-serialisierte Hook-Konfiguration. Nur enthalten, wenn sowohl detaillierte Beta-Verfolgung als auch `OTEL_LOG_TOOL_DETAILS=1` aktiviert sind

<h4 id="hook-execution-complete-event">
  Hook-Ausführungs-Abschluss-Ereignis
</h4>

Protokolliert, wenn alle Hooks für ein Hook-Ereignis abgeschlossen sind.

**Ereignisname**: `claude_code.hook_execution_complete`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `hook_event`: Hook-Ereignistyp
* `hook_name`: Vollständiger Hook-Name einschließlich Matcher
* `num_hooks`: Anzahl der übereinstimmenden Hook-Befehle
* `num_success`: Anzahl, die erfolgreich abgeschlossen wurde
* `num_blocking`: Anzahl, die eine blockierende Entscheidung zurückgab
* `num_non_blocking_error`: Anzahl, die fehlgeschlagen ist, ohne zu blockieren
* `num_cancelled`: Anzahl, die vor Abschluss abgebrochen wurde
* `total_duration_ms`: Wanduhr-Dauer aller übereinstimmenden Hooks
* `stdout_chars`: Gesamtzeichen von stdout über die übereinstimmenden Hooks, die erfolgreich waren. Erfordert Claude Code v2.1.280 oder später
* `additional_context_chars`: Gesamtzeichen von `additionalContext`, die von den übereinstimmenden Hooks zurückgegeben wurden. Erfordert Claude Code v2.1.280 oder später
* `system_message_chars`: Gesamtzeichen von `systemMessage`, die von den übereinstimmenden Hooks zurückgegeben wurden. Erfordert Claude Code v2.1.280 oder später
* `initial_user_message_chars`: Gesamtzeichen von `initialUserMessage`, die von den übereinstimmenden Hooks zurückgegeben wurden. Erfordert Claude Code v2.1.280 oder später
* `num_outputs_persisted`: Anzahl der Hook-Ausgaben über die [10.000-Zeichen-Obergrenze](/docs/de/hooks#json-output), die Claude Code in einer Datei gespeichert hat. Erfordert Claude Code v2.1.280 oder später
* `managed_only`: `"true"`, wenn nur verwaltete Richtlinien-Hooks zulässig sind
* `hook_source`: `"policySettings"` oder `"merged"`
* `safe_mode`: `"true"`, wenn die Sitzung mit [`--safe-mode`](/docs/de/cli-reference) gestartet wurde, `"false"` andernfalls. Erfordert Claude Code v2.1.169 oder später
* `hook_definitions`: JSON-serialisierte Hook-Konfiguration. Nur enthalten, wenn sowohl detaillierte Beta-Verfolgung als auch `OTEL_LOG_TOOL_DETAILS=1` aktiviert sind

<h4 id="hook-plugin-metrics-event">
  Hook-Plugin-Metriken-Ereignis
</h4>

Protokolliert, wenn ein Hook eines offiziellen Marketplace-Plugins Pro-Aufruf-Metriken ausgibt. Nur Plugins, die von einem offiziellen Anthropic-Marketplace installiert wurden, können diese ausgeben. Drittanbieter-Marketplace-Plugins und benutzerkonfigurierte Hooks geben nicht an dieses Ereignis aus. Verwenden Sie dieses Ereignis, um Plugin-Verhalten wie Findungsraten, Kosten und Dauern aus Ihrem eigenen Observability-Stack zu überwachen.

**Ereignisname**: `claude_code.hook_plugin_metrics`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `plugin_id`: Plugin-Kennung in `<name>@<marketplace>`-Form
* `hook_event`: Hook-Ereignistyp, der die Metriken ausgegeben hat
* Bis zu 20 Plugin-ausgegebene Metrik-Schlüssel. Namen entsprechen `^[a-z][a-z0-9_]{0,39}$`. Werte sind Boolean oder Zahl.

<h4 id="compaction-event">
  Kompaktierungs-Ereignis
</h4>

Protokolliert, wenn die Gesprächskompaktierung abgeschlossen ist.

**Ereignisname**: `claude_code.compaction`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `trigger`: `"auto"` oder `"manual"`
* `success`: `"true"` oder `"false"`
* `duration_ms`: Kompaktierungs-Dauer
* `pre_tokens`: Ungefähre Token-Anzahl vor Kompaktierung
* `post_tokens`: Ungefähre Token-Anzahl nach Kompaktierung
* `error`: Fehlermeldung, wenn Kompaktierung fehlgeschlagen ist
* `precompute_reuse`: Nur gesetzt, wenn `trigger` `"manual"` ist. Auto-Kompaktierung kann eine Zusammenfassung im Hintergrund vorbereiten, bevor das Kontextfenster voll wird, und dieses Attribut zeichnet auf, ob `/compact` diese vorbereitete Zusammenfassung wiederverwendet hat. `"hit"` bedeutet, dass sie wiederverwendet wurde; `"miss_custom_instructions"`, `"miss_hook"` und `"miss_not_ready"` geben den Grund an, warum stattdessen eine frische Zusammenfassung berechnet wurde. Erfordert Claude Code v2.1.153 oder später

<h4 id="subagent-completed-event">
  Subagent-Abgeschlossen-Ereignis
</h4>

Protokolliert, wenn ein [Subagent](/docs/de/sub-agents) fertig wird und sein Ergebnis an das Gespräch zurückgibt, das ihn gestartet hat. Verwenden Sie es, um Tool-Nutzung und Laufzeit nach Subagent-Typ zu aggregieren; für Token- oder Kosten-Aggregationen verwenden Sie den [Token-Zähler](#token-counter) und [Kostenzähler](#cost-counter), gefiltert nach `query_source` `"subagent"`, da das `total_tokens` dieses Ereignisses nur die endgültige Anfrage abdeckt. Die Kategorie `"subagent"` zählt auch Anfragen von Agent-basierten Hooks, die kein Subagent-Ereignis ausgeben.

**Ereignisname**: `claude_code.subagent_completed`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"subagent_completed"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `agent_type`: Der Subagent-Typ. Integrierte Agent-Namen und Agenten aus offiziellen Marketplace-Plugins werden wörtlich angezeigt; andere Agent-Namen werden durch `"custom"` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt
* `agent.source`: Woher die Agent-Definition kam: `built-in`, `plugin` oder die Einstellungsquelle, die einen benutzerdefinierten Agent definierte, z. B. `userSettings` oder `projectSettings`
* `is_built_in`: Ob der Subagent ein integrierter Agent-Typ ist
* `is_async`: Ob der Subagent im [Hintergrund](/docs/de/sub-agents#run-subagents-in-foreground-or-background) ausgeführt wurde
* `total_tokens`: Der Token-Fußabdruck der endgültigen API-Anfrage des Subagenten: dieser einen Anfrage's Eingabe-, Cache-Erstellungs-, Cache-Lese- und Ausgabe-Token, ungefähr die Kontextgröße des Subagenten beim Abschluss. Keine Summe über den Lauf
* `total_tool_uses`: Anzahl der Tool-Aufrufe, die der Subagent über den ganzen Lauf hinweg gemacht hat
* `duration_ms`: Laufzeit in Millisekunden
* `model`: Das Modell, das der Subagent ausgeführt wurde
* `final_model`: Das Modell, das die endgültige Antwort des Subagenten erzeugt hat, das sich von `model` nach einem Mid-Run-Wechsel wie einem Fallback unterscheidet. Erfordert Claude Code v2.1.212 oder später
* `model_swapped`: Ob mehr als ein Modell die Anfragen des Subagenten bedient hat. Erfordert Claude Code v2.1.212 oder später
* `plugin_id_hash`, `plugin.name`: Vorhanden für Plugin-bereitgestellte Agenten. Offizielle Marketplace-Plugin-Namen werden wörtlich angezeigt; andere Plugin-Namen werden durch `"third-party"` ersetzt, es sei denn, `OTEL_LOG_TOOL_DETAILS=1` ist gesetzt

<h4 id="feedback-survey-event">
  Feedback-Umfrage-Ereignis
</h4>

Protokolliert, wenn eine Sitzungsqualitäts-Umfrage angezeigt oder beantwortet wird. Siehe [Sitzungsqualitäts-Umfragen](/docs/de/data-usage#session-quality-surveys) für was die Umfragen erfassen und wie Sie sie steuern.

**Ereignisname**: `claude_code.feedback_survey`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `event_type`: Umfrage-Lebenszyklusereignis, z. B. `"appeared"`, `"responded"` oder `"transcript_prompt_appeared"`
* `appearance_id`: Eindeutige ID, die die Ereignisse verknüpft, die für eine Umfrage-Instanz ausgegeben werden
* `survey_type`: Welche Umfrage das Ereignis erzeugt hat. `"session"` ist die Aufforderung „Wie macht Claude es?" Bewertung
* `response`: Die Auswahl des Benutzers auf `responded`-Ereignissen
* `enabled_via_override`: `true`, wenn [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/de/env-vars) gesetzt ist. Ausgegeben als Boolean, nicht als Zeichenkette. Vorhanden auf `session`-Umfrage-Ereignissen. Filtern Sie nach diesem Attribut, um zu bestätigen, dass die Überschreibung über eine Flotte angewendet wird

<h4 id="retention-sweep-event">
  Aufbewahrungslösch-Ereignis
</h4>

Protokolliert einmal pro Lauf des Aufbewahrungslösch-Sweeps, der [Sitzungstranskripte und andere Anwendungsdaten](/docs/de/claude-directory#cleaned-up-automatically) löscht, die älter als die Einstellung [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays) sind. Claude Code führt den Sweep im Hintergrund höchstens einmal pro Sitzung aus, und ein Lauf, der nichts löscht, gibt trotzdem das Ereignis aus. Wenn Claude Code den Sweep in einer Sitzung auf derselben Maschine in den letzten 24 Stunden ausgeführt hat, verzögert er den Sweep dieser Sitzung um mindestens 10 Minuten, daher gibt eine Sitzung, die früher beendet wird, nichts aus. Wenn Sie `claude -p` mit `--bare` ausführen, führt Claude Code den Sweep nicht aus und gibt nichts aus.

Wie jedes OTel-Ereignis auf dieser Seite geht es nur an das Telemetrie-Backend, das Sie konfigurieren. Erfordert Claude Code v2.1.227 oder später.

Wenn Claude Code die Aufbewahrungsfrist nicht sicher bestimmen kann, pausiert es den Sweep und gibt das Ereignis mit `result` auf `"skipped"` und einem `skip_reason` aus. Wenn [verwaltete Einstellungen](/docs/de/server-managed-settings) `cleanupPeriodDays` setzen, pinnt der verwaltete Wert die Aufbewahrungsfrist und der Sweep läuft auch wenn eine Einstellungsdatei in einem niedrigeren Prioritätsbereich unterbrochen oder ungültig ist. Wenn `managed-settings.json` selbst nicht gelesen werden kann, pausiert Claude Code den Sweep trotzdem nicht, es sei denn, die [verwaltete Ebene](/docs/de/managed-settings#how-claude-code-combines-managed-sources) liefert `cleanupPeriodDays` von anderswo, z. B. Server-verwaltete Einstellungen oder ein `managed-settings.d/`-Drop-In neben der unterbrochenen Datei. Die Lösch-Zähler-Attribute sind nur vorhanden, wenn `result` `"complete"` ist.

**Ereignisname**: `claude_code.retention_sweep`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"retention_sweep"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `result`: `"complete"`, wenn der Sweep ausgeführt wurde, `"skipped"`, wenn Claude Code ihn pausierte
* `period_days`: Der `cleanupPeriodDays`-Wert aus zusammengeführten Einstellungen, in Tagen, oder `30`, wenn keine Quelle ihn setzt. Bei übersprungenen Ereignissen der Wert, den der Sweep verwendet hätte, berechnet aus den Einstellungsquellen, die Claude Code lesen konnte
* `used_default`: `"true"`, wenn keine lesbare Einstellungsquelle `cleanupPeriodDays` setzt, `"false"` andernfalls. Bei abgeschlossenen Ereignissen bedeutet `"true"`, dass der 30-Tage-Standard angewendet wurde
* `skip_reason`: Warum Claude Code den Sweep pausierte. Nur vorhanden, wenn `result` `"skipped"` ist:
  * `"user_source_disabled"`: Benutzereinstellungen sind ausgeschlossen, z. B. durch das Flag [`--setting-sources`](/docs/de/cli-reference#cli-flags) oder die Option [`settingSources`](/docs/de/agent-sdk/typescript#options) des SDK, und keine aktivierte Quelle liefert `cleanupPeriodDays`
  * `"settings_unknowable"`: Eine Einstellungsdatei konnte nicht gelesen oder geparst werden, daher kann `cleanupPeriodDays` oder `desktopSessionCleanupPeriodDays` auf einen Wert gesetzt sein, den Claude Code nicht sehen kann
  * `"settings_invalid_key_set"`: Einstellungen haben Validierungsfehler und `cleanupPeriodDays` oder `desktopSessionCleanupPeriodDays` ist explizit gesetzt, daher könnte ein Fallback auf den Standard Dateien gegen diese Einstellung löschen oder behalten
* `transcripts_deleted`: Anzahl der Sitzungstranskripte, die Top-Level-Dateien `~/.claude/projects/*/*.jsonl`, die der Sweep gelöscht hat
* `transcripts_exempted_desktop`: Anzahl der Transkripte, die über die Aufbewahrungsfrist hinaus sind und die der Sweep unter der [Claude Desktop- und Cowork-Regel](/docs/de/claude-directory#cleaned-up-automatically) behalten hat. Diese zählen nicht zu `files_past_cutoff`. Erfordert Claude Code v2.1.248 oder später
* `session_files_deleted`: Anzahl der Artefakte, die der Sitzungs-Dateien-Sweep gelöscht hat: Transkripte plus Pro-Sitzungs-Begleitdateien wie Sidecars, Aufzeichnungen und Tool-Ergebnisse
* `artifacts_deleted`: Gesamtzahl der Elemente, die der Sweep über die Datenverzeichnisse hinweg gelöscht hat, die er abdeckt, einschließlich der Sitzungsdateien. Einige Sweeps zählen einen ganzen entfernten Verzeichnisbaum als ein Element und ein paar Cleanup-Durchläufe tragen nicht zum Zähler bei, daher behandeln Sie den Wert als Untergrenze statt als genaue Dateianzahl
* `files_retained_fresh`: Dateien, die inspiziert und behalten wurden, weil sie noch innerhalb der Aufbewahrungsfrist sind. Nur Pro-Datei-Sweeps zählen diese, daher ist der Wert eine Untergrenze; ein Wert ungleich Null ist der normale stabile Zustand
* `files_past_cutoff`: Dateien, die älter als die Aufbewahrungsfrist sind und die der Sweep nicht löschen konnte, z. B. wegen eines Berechtigungsfehlers oder einer offenen Datei. Ein Wert über Null bedeutet, dass Dateien die konfigurierte Aufbewahrungsfrist überdauert haben; Null ist kein Beweis, dass keine, weil ein fehlgeschlagenes Entfernen eines ganzen Verzeichnisses zu `error_count` statt zählt
* `error_count`: Anzahl der Fehler, die der Sweep beim Auflisten oder Löschen von Dateien angetroffen hat

<h4 id="managed-settings-resolved-event">
  Verwaltete Einstellungen Aufgelöst-Ereignis
</h4>

Protokolliert mit den [verwalteten Einstellungen](/docs/de/managed-settings), die eine Sitzung aufgelöst hat: einmal beim Sitzungsstart, erneut wenn sich entweder die verwalteten Einstellungen oder der Zustand des [Policy-Helfers](/docs/de/managed-settings#compute-the-policy-with-a-helper-program) während der Sitzung ändert, und wenn Claude Code sich weigert zu starten oder die Sitzung aus einem der Gründe beendet, die das Attribut `error.type` auflistet.
Verwenden Sie dieses Ereignis, um Maschinen zu finden, die auf einer unerwarteten verwalteten Quelle laufen, Maschinen, deren Policy-Helfer fehlschlägt, und den Grund, warum eine Maschine sich weigert zu starten.
Erfordert Claude Code v2.1.274 oder später.

Standardmäßig trägt das Ereignis die verwalteten Quellen und den Zustand des Policy-Helfers, aber nicht die Einstellungen selbst. Um das redaktionierte Attribut `managed_settings.settings` und die Zusammenfassung `managed_settings.resolved_sha256` hinzuzufügen, setzen Sie `OTEL_LOG_MANAGED_SETTINGS=1`:

* Setzen Sie es im `env`-Block von verwalteten Einstellungen, Benutzereinstellungen oder `--settings`, oder in der Umgebung, mit der Sie Claude Code starten. Ein Wert in Projekt- oder lokalen Einstellungen schaltet es nicht ein, da ein geklontes Repository sie schreiben kann.
* Server-verwaltete Einstellungen können es setzen, ohne den [Sicherheitsgenehmigungsdialog](/docs/de/server-managed-settings#security-approval-dialogs) anzuzeigen, da die Variable nur Ihre Organisationseigene redaktionierte Richtlinie zu einem Ereignis hinzufügt, das Ihre Organisation bereits erhält.

In einer interaktiven Sitzung in einem Ordner, den Sie nicht [vertraut](/docs/de/permissions#what-runs-before-you-trust-a-folder) haben, exportiert Claude Code das Verweigerungs-Ereignis nicht.

**Ereignisname**: `claude_code.managed_settings_resolved`

**Attribute**:

* Alle [Standardattribute](#standard-attributes)
* `event.name`: `"managed_settings_resolved"`
* `event.timestamp`: ISO 8601-Zeitstempel
* `event.sequence`: Pro-Prozess-Zähler zum Ordnen von Ereignissen, beschrieben unter [Ereigniskorrelationsattribute](#event-correlation-attributes)
* `managed_settings.trigger`: `"startup"` für das Sitzungsstart-Ereignis, `"change"`, wenn sich die verwalteten Einstellungen oder der Zustand des Policy-Helfers später in der Sitzung ändert, oder `"refused"`, wenn eine verwaltete Einstellungsrichtlinie die Sitzung stoppte. Claude Code sendet ein `change`-Ereignis nur, wenn sich ein Attribut vom letzten Ereignis unterscheidet, das es gesendet hat, und ein geänderter Einstellungswert zählt auch wenn `OTEL_LOG_MANAGED_SETTINGS` aus ist
* `error.type`: warum Claude Code die Sitzung stoppte. Nur vorhanden auf `refused`-Ereignissen:
  * `"helper_failed"`: ein [Policy-Helfer-Lauf fehlgeschlagen](/docs/de/settings-reference#helper-failures)
  * `"policy_invalid"`: die verwalteten Einstellungen enthalten einen Fehler, der Claude Code vom Starten abhält, oder eine Admin-Quelle konnte nicht geladen werden, daher kann Claude Code die Organisations-Login-Erzwingung nicht prüfen
  * `"consent_rejected"`: der Benutzer lehnte den [Sicherheitsgenehmigungsdialog](/docs/de/server-managed-settings#security-approval-dialogs) für Server-verwaltete Einstellungen ab
  * `"force_refresh_failed"`: der Einstellungs-Abruf, den [`forceRemoteSettingsRefresh`](/docs/de/settings-reference#forceremotesettingsrefresh) erfordert, fehlgeschlagen
  * `"gateway_rejected"`: ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) antwortete auf den verwalteten Einstellungs-Abruf mit HTTP 403
  * `"version_below_minimum"`: diese Version von Claude Code ist unter [`requiredMinimumVersion`](/docs/de/settings-reference#requiredminimumversion) oder über [`requiredMaximumVersion`](/docs/de/settings-reference#requiredmaximumversion)
  * `"_OTHER"`: der Claude-Apps-Gateway verwaltete Einstellungs-Abruf fehlgeschlagen aus einem anderen Grund
* `managed_settings.sources`: jede verwaltete Quelle, die mindestens einen [Policy-Schlüssel](/docs/de/managed-settings#how-claude-code-combines-managed-sources) liefert, höchste Priorität zuerst, einschließlich Quellen, deren Schlüssel nicht unter `first-wins` wirksam werden. Werte sind `"remote"`, `"plist"` oder `"hklm"` für die MDM- oder OS-Level-Richtlinie, `"file"` für verwaltete Einstellungsdateien und Drop-Ins, `"parent"`, wenn ein [Embedding-Host](/docs/de/managed-settings#let-an-embedding-host-add-policy) Einstellungen liefert, und `"hkcu"` für den [Windows HKCU-Registry-Wert](/docs/de/managed-settings#where-each-mechanism-stores-the-policy), wenn Claude Code [ihn liest](/docs/de/managed-settings#how-claude-code-combines-managed-sources). Eine Quelle, die nur Kontrollschlüssel trägt, oder die Claude Code nicht lesen konnte, ist nicht aufgelistet. Ausgegeben als Array von Zeichenketten, leer wenn keine verwaltete Quelle einen Policy-Schlüssel liefert
* `managed_settings.source_behavior`: der [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior)-Wert, den Claude Code las, `"first-wins"` oder `"merge"`. `"first-wins"`, wenn keine Quelle den Schlüssel setzt
* `managed_settings.helper.state`: Zustand des Policy-Helfers, den die ausgewählte MDM- oder Datei-Quelle konfiguriert:
  * `"ok"`: die Ausgabe des Helfers dient als verwaltete Einstellungen
  * `"bad_path"`, `"not_a_file"`, `"exit_nonzero"`, `"timed_out"`, `"oversize"`, `"parse_failed"`, `"envelope_invalid"` oder `"schema_rejected"`: der letzte Lauf des Helfers fehlgeschlagen. [Helper-Fehler](/docs/de/settings-reference#helper-failures) beschreibt die Fälle
  * `"none"`: kein Helfer ist konfiguriert, oder die Quelle, die ihn konfiguriert, ist keine MDM-Richtlinie oder verwaltete Einstellungsdatei
* `managed_settings.helper.applied`: `"output"`, während die eigene Ausgabe des Helfers als verwaltete Einstellungen dient, `"none"`, wenn sie nicht
* `managed_settings.helper.entry`: `"policyHelper"`, wenn Claude Code einen [`policyHelper`](/docs/de/settings-reference#policyhelper) ausgewählt hat. Nicht vorhanden, wenn es keinen Helfer ausgewählt hat
* `managed_settings.helper.path`: der konfigurierte [`path`](/docs/de/settings-reference#policyhelper-path) des Helfers. Vorhanden, wenn Claude Code einen Helfer ausgewählt hat, ob `OTEL_LOG_MANAGED_SETTINGS` gesetzt ist oder nicht
* `managed_settings.resolved_sha256` (wenn `OTEL_LOG_MANAGED_SETTINGS=1`): SHA-256 der aufgelösten verwalteten Einstellungen vor Redaktion, serialisiert als JSON mit rekursiv sortierten Schlüsseln und ohne Leerzeichen. Maschinen mit derselben Zusammenfassung führen dieselbe Richtlinie aus. Claude Code sendet die Zusammenfassung nur mit dem Opt-In, da eine kurze Richtlinie durch Hashing von Vermutungen wiederhergestellt werden kann. Nicht vorhanden, wenn keine verwalteten Einstellungen aufgelöst wurden, und auf `refused`-Ereignissen
* `managed_settings.settings` (wenn `OTEL_LOG_MANAGED_SETTINGS=1`): die Namen und Form der aufgelösten verwalteten Einstellungen als JSON-Zeichenkette, mit den Werten redaktioniert. Nicht vorhanden auf `refused`-Ereignissen. Claude Code erstellt es aus seinem Einstellungsschema:

  * Ein Einstellungsname, den das Schema deklariert, wird exportiert, und ein Schlüssel, den es nicht deklariert, wird weggelassen
  * Booleans, Zahlen und String-Werte, die das Schema auf einen festen Satz von Optionen beschränkt, z. B. `permissions.defaultMode`, werden wie geschrieben exportiert. `sandbox.network.httpProxyPort` und `sandbox.network.socksProxyPort` werden als `"[REDACTED]"` exportiert
  * Jede andere Zeichenkette, z. B. `model`, `apiKeyHelper`, jeder `env`-Wert, jede URL und jeder Befehl, wird als `"[REDACTED]"` exportiert
  * Die Eintrags-Namen von Maps, z. B. `env`-Variablennamen und Plugin-IDs, werden wie geschrieben exportiert. Eine Einstellung, deren Einträge das Schema nicht typisiert, z. B. `vimInsertModeRemaps`, wird als ein einzelnes `"[REDACTED]"` exportiert, und `sandbox.ignoreViolations` wird als eine Liste seiner Pfad-Listen ohne die Befehlsmuster exportiert
  * Eine Liste behält ihre Länge, mit jedem Eintrag nach denselben Regeln redaktioniert
  * Eine `permissions.allow`-, `permissions.deny`- oder `permissions.ask`-Regel wird als ihr Tool-Name mit dem Inhalt redaktioniert exportiert, z. B. `Read([REDACTED])`, wenn das Tool in diese Version von Claude Code integriert ist oder ein `mcp__`-Verweis wie `mcp__jira__create_issue` ist. Jede andere Regel wird als `"[REDACTED]"` exportiert
  * Hooks folgen denselben Regeln, daher zeigen Fixed-Option- und numerische Felder wie `type` und `timeout`, während jeder Befehl, jede URL, `matcher` und `if`-Bedingung als `"[REDACTED]"` exportiert wird

  Zum Beispiel werden verwaltete Einstellungen mit `apiKeyHelper`, zwei `env`-Variablen und einer Ablehnungsregel als `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}` exportiert.

  Claude Code schneidet den Wert bei 8 KB UTF-8 ab, und der abgeschnittene Wert ist kein gültiges JSON
* `managed_settings.settings_truncated` (wenn `managed_settings.settings` vorhanden ist): `true`, wenn Claude Code `managed_settings.settings` bei 8 KB abschnitt, `false` andernfalls. Ausgegeben als Boolean, nicht als Zeichenkette

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

Claude Code zählt jede Streaming-Antwort genau einmal zu den Kosten- und Token-Metriken, auch wenn ein Gateway oder Proxy hinter `ANTHROPIC_BASE_URL` die Nutzung progressiv über mehrere Frames hinweg streamt. Vor v2.1.214 führten Streams, die Nutzung in mehr als einem Frame trugen, zu einer Aufblähung von `claude_code.cost.usage` und `claude_code.token.usage` um ungefähr eine zusätzliche vollständige Anfrage pro zusätzlichem Frame.

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

Das Attribut `attempt` auf dem Ereignis zeichnet auf, wie viele Versuche insgesamt unternommen wurden. `CLAUDE_CODE_MAX_RETRIES` hat einen Standardwert von 10 und ist auf 15 begrenzt. Ab v2.1.199 können Sie `CLAUDE_CODE_RETRY_WATCHDOG` setzen, um den Standardwert zu erhöhen und die Obergrenze zu entfernen.

Wenn die Anfrage alle Wiederholungen bei einem vorübergehenden Fehler erschöpft, ist `attempt` um eins höher als dieses effektive Limit: 11 standardmäßig und nie mehr als 16, es sei denn, der Watchdog ist gesetzt. Ein niedrigerer Wert zeigt einen nicht wiederholbaren Fehler wie eine `400` Antwort an, oder eine Ursache mit einem eigenen kleineren Wiederholungsbudget. Zum Beispiel wiederholt Claude Code einen Fehler beim Laden von AWS- oder Google Cloud-Anmeldedaten höchstens zweimal.

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

Die [Standardattribute](#standard-attributes) auf jedem Ereignis enthalten die Identität des authentifizierten Benutzers: `user.email`, `user.account_uuid`, `user.account_id` und `organization.id`, wenn mit einem Claude-Konto angemeldet oder in einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web), wenn die Anmeldedaten der Sitzung selbst diese tragen, plus `user.id` und die pro-Sitzung `session.id`. `user.id` ist ein installationsbegrenzter Bezeichner, außer bei [Claude apps gateway](/docs/de/claude-apps-gateway)-Sitzungen, wo es das IdP-Subjekt aus dem vom Gateway ausgegebenen Token ist.

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

* `tool_result`: behält `mcp_server_scope` und einen `tool_name`, der für benutzerkonfigurierte Server auf das Literal `"mcp_tool"` redigiert ist, lässt Argument-Inhalte weg. Für Claude Desktop's integrierte Server, in Sitzungen, die Claude Desktop besitzt, behält es auch das `mcp_server_name`/`mcp_tool_name`-Paar innerhalb von `tool_parameters`, die gleiche von Host verfasste Ausnahme wie `tool_decision`, erfordert Claude Code v2.1.214 oder später
* `tool_decision`: behält `tool_source` und einen `tool_name`, der für benutzerkonfigurierte Server auf das Literal `"mcp_tool"` redigiert ist, lässt Argument-Inhalte weg. Für Claude Desktop's integrierte Server, in Sitzungen, die Claude Desktop besitzt, behält es auch das `mcp_server_name`/`mcp_tool_name`-Paar innerhalb von `tool_parameters`; `tool_source` und das Name-Paar erfordern beide Claude Code v2.1.214 oder später
* `mcp_server_connection`: lässt `server_name` und die Fehlermeldung weg, behält aber `is_plugin`, `plugin_id_hash` und `plugin.name`, wobei Namen von Nicht-Anthropic-Plugins auf das Literal `"third-party"` redigiert werden, sodass von Plugins bereitgestellte Server ohne detaillierte Protokollierung unterscheidbar bleiben

<h3 id="map-security-questions-to-events">
  Sicherheitsfragen zu Ereignissen zuordnen
</h3>

Beim Erstellen von Erkennungsregeln schlagen Sie das Signal auf, das Sie überwachen möchten, und fragen Sie Ihr Backend nach dem entsprechenden Ereignis und den Attributen ab:

| Signal                                                                                                                                             | Ereignis                                                                                  | Schlüsselattribute                                                                                                                                                                                                                              |
| -------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tool-Aufruf zulässig oder verweigert, und von wem                                                                                                  | `tool_decision`                                                                           | `decision`, `source`, `tool_name`, `tool_parameters`                                                                                                                                                                                            |
| Berechtigungsmodus-Eskalation                                                                                                                      | `permission_mode_changed`                                                                 | `from_mode`, `to_mode`, `trigger`                                                                                                                                                                                                               |
| Policy-Hook blockierte eine Aktion                                                                                                                 | `hook_execution_complete`                                                                 | `hook_event`, `num_blocking`                                                                                                                                                                                                                    |
| Login, Logout und Authentifizierungsfehler                                                                                                         | `auth`                                                                                    | `action`, `success`, `error_category`                                                                                                                                                                                                           |
| MCP-Server-Verbindung oder Fehler                                                                                                                  | `mcp_server_connection`                                                                   | `status`, `server_name`, `is_plugin`, `error_code`                                                                                                                                                                                              |
| Plugin installiert und seine Quelle                                                                                                                | `plugin_installed`                                                                        | `plugin.name`, `marketplace.name`, `marketplace.is_official`                                                                                                                                                                                    |
| Befehle ausgeführt und Dateien berührt                                                                                                             | `tool_result` (ausgeführt) oder `tool_decision` (abgelehnt) mit `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` nur)                                                                                                                                                                                             |
| Welche verwalteten Einstellungsquellen ein Computer ausführt, ob sein Policy-Helper fehlerfrei ist und warum ein Computer sich weigerte zu starten | `managed_settings_resolved`                                                               | `managed_settings.trigger`, `managed_settings.sources`, `managed_settings.source_behavior`, `managed_settings.helper.state`, `error.type`; `managed_settings.settings` und `managed_settings.resolved_sha256` mit `OTEL_LOG_MANAGED_SETTINGS=1` |

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

Um zu bestätigen, dass Ereignisse ankommen, senden Sie eine Eingabeaufforderung in einer Sitzung, die unter dieser Konfiguration ausgeführt wird, und überprüfen Sie Ihr SIEM auf das `claude_code.user_prompt`-Ereignis. Wenn nichts ankommt, führen Sie `claude --debug` aus und überprüfen Sie das Debug-Protokoll auf `[3P telemetry]`-Exportfehler.

<h2 id="backend-considerations">
  Backend-Überlegungen
</h2>

Ihre Wahl des Metriken-, Logs- und Traces-Backends bestimmt die Arten von Analysen, die Sie durchführen können:

<h3 id="for-metrics">
  Für Metriken
</h3>

* **Zeitreihendatenbanken**: Ratenberechnungen, aggregierte Metriken
* **Spaltenorientierte Speicher**: Komplexe Abfragen, eindeutige Benutzeranalyse
* **Vollständige Observability-Plattformen**: Erweiterte Abfragen, Visualisierung, Warnungen

<h3 id="for-events/logs">
  Für Ereignisse/Logs
</h3>

* **Log-Aggregationssysteme**: Volltextsuche, Log-Analyse
* **Spaltenorientierte Speicher**: Strukturierte Ereignisanalyse
* **Vollständige Observability-Plattformen**: Korrelation zwischen Metriken und Ereignissen

<h3 id="for-traces">
  Für Traces
</h3>

Wählen Sie ein Backend, das verteilte Trace-Speicherung und Span-Korrelation unterstützt:

* **Verteilte Tracing-Systeme**: Span-Visualisierung, Request-Waterfalls, Latenzanalyse
* **Vollständige Observability-Plattformen**: Trace-Suche und Korrelation mit Metriken und Logs

Für Organisationen, die Daily/Weekly/Monthly Active User (DAU/WAU/MAU) Metriken benötigen, sollten Sie Backends in Betracht ziehen, die effiziente Abfragen eindeutiger Werte unterstützen.

<h2 id="service-information">
  Dienstinformationen
</h2>

Alle Metriken und Ereignisse werden mit den folgenden Ressourcenattributen exportiert:

* `service.name`: `claude-code` für Terminal-Sitzungen, `claude-code-desktop` für Sitzungen, die über die Registerkarte „Code" in der [Claude Desktop-App](/docs/de/desktop) gestartet werden
* `service.version`: Aktuelle Claude Code-Version oder die Desktop-App-Version für Code-Registerkarten-Sitzungen
* `os.type`: Betriebssystemtyp (zum Beispiel `linux`, `darwin`, `windows`)
* `os.version`: Betriebssystem-Versionsnummer
* `host.arch`: Host-Architektur (zum Beispiel `amd64`, `arm64`)
* `wsl.version`: WSL-Versionsnummer (nur vorhanden, wenn auf Windows Subsystem for Linux ausgeführt)
* Meter-Name: `com.anthropic.claude_code`

Wenn Ihre Collector-Pipelines oder Dashboards nach `service.name = claude-code` filtern, fügen Sie `claude-code-desktop` zum Filter hinzu, um auch Telemetrie von Code-Registerkarten-Sitzungen zu erfassen.

<h2 id="roi-measurement-resources">
  ROI-Messung-Ressourcen
</h2>

Für einen umfassenden Leitfaden zur Messung der Kapitalrendite für Claude Code, einschließlich Telemetrie-Setup, Kostenanalyse, Produktivitätsmetriken und automatisierter Berichterstattung, siehe den [Claude Code ROI Measurement Guide](https://github.com/anthropics/claude-code-monitoring-guide). Dieses Repository bietet einsatzbereite Docker Compose-Konfigurationen, Prometheus- und OpenTelemetry-Setups sowie Vorlagen zur Generierung von Produktivitätsberichten, die in Tools wie Linear integriert sind.

<h2 id="security-and-privacy">
  Sicherheit und Datenschutz
</h2>

* OpenTelemetry-Export zu Ihrem Backend ist opt-in und erfordert explizite Konfiguration. Informationen zu Anthropics separater operativer Telemetrie und wie Sie diese deaktivieren, finden Sie unter [Datennutzung](/docs/de/data-usage#telemetry-services)
* Rohe Dateiinhalte und Code-Snippets sind nicht in Metriken oder Ereignissen enthalten. Trace-Spans sind ein separater Datenpfad: siehe die Aufzählung `OTEL_LOG_TOOL_CONTENT` unten
* Wenn über OAuth authentifiziert, ist `user.email` in Telemetrie-Attributen enthalten und wird nur an den OTel-Endpunkt gesendet, den Sie konfigurieren, niemals an Anthropic. Wenn dies ein Problem für Ihre Organisation darstellt, arbeiten Sie mit Ihrem Telemetrie-Backend zusammen, um dieses Feld zu filtern oder zu schwärzen
* Benutzer-Prompt-Inhalte werden standardmäßig nicht erfasst. Nur die Prompt-Länge wird aufgezeichnet. Um Benutzer-Prompt-Inhalte einzubeziehen, setzen Sie `OTEL_LOG_USER_PROMPTS=1`. Unter detailliertem Beta-Tracing reicht diese Variable weiter als nur Prompt-Text: Sie steuert auch das [`new_context`-Span-Attribut](#new-context-gates), das Tool-Ergebnisse auf dem `claude_code.llm_request`-Span trägt
* Assistent-Antworttext wird standardmäßig nicht erfasst. Nur die Antwortlänge wird aufgezeichnet. Um Antworttext einzubeziehen, setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=1`. Wie alle OpenTelemetry-Daten von Claude Code wird der Antworttext nur an den OTel-Endpunkt gesendet, den Sie konfigurieren, niemals an Anthropic. Wenn diese Variable nicht gesetzt ist, wird `OTEL_LOG_USER_PROMPTS` als Fallback verwendet, daher setzen Sie `OTEL_LOG_ASSISTANT_RESPONSES=0`, wenn Sie Prompt-Inhalte ohne Antwortinhalte möchten
* Tool-Eingabeargumente und Parameter werden standardmäßig nicht protokolliert. Um sie einzubeziehen, setzen Sie `OTEL_LOG_TOOL_DETAILS=1`. Für die integrierten Server von Claude Desktop werden in Sitzungen, die Claude Desktop besitzt, `tool_decision` und `tool_result` mit dem Paar `mcp_server_name`/`mcp_tool_name` übertragen, von Hosts erstellte Namen statt Argumentinhalte, auch wenn das Flag aus ist. Die Ausnahme erfordert Claude Code v2.1.214 oder später. Diese Daten werden nur an den OTEL-Endpunkt gesendet, den Sie konfigurieren, niemals an Anthropic. Argumente können immer noch vertrauliche Werte enthalten, daher konfigurieren Sie Ihr Telemetrie-Backend, um diese Attribute nach Bedarf zu filtern oder zu schwärzen. Wenn aktiviert:
  * `tool_result`- und `tool_decision`-Ereignisse enthalten ein `tool_parameters`-Attribut mit Bash-Befehlen, MCP-Server- und Tool-Namen sowie Skill-Namen. Felder wie `full_command` werden ungekürzt ausgegeben
  * `tool_result`-Ereignisse enthalten zusätzlich ein `tool_input`-Attribut mit Dateipfaden, URLs, Suchmustern und anderen Argumenten. Einzelne Werte über 512 Zeichen werden gekürzt und die Gesamtmenge ist auf etwa 4 K Zeichen begrenzt
  * `user_prompt`-Ereignisse enthalten den wörtlichen `command_name` für benutzerdefinierte, Plugin- und MCP-Befehle
  * Trace-Spans enthalten das gleiche `tool_input`-Attribut und eingabebezogene Attribute wie `file_path`, mit der gleichen Kürzung wie `tool_input`
* Tool-Inhalte werden in Trace-Spans standardmäßig nicht protokolliert. Um sie einzubeziehen, setzen Sie `OTEL_LOG_TOOL_CONTENT=1`. Der `claude_code.tool`-Span trägt dann ein [`tool.output`-Span-Ereignis](#tool-output-span-event) mit rohen Dateiinhalten und Bash-Befehlsausgabe, gekürzt bei der Inhaltsbegrenzung (standardmäßig 60 KB) pro Attribut. Tool-Inhalte erreichen Spans auch durch [`new_context`, dessen Gate je nach Span unterschiedlich ist](#new-context-gates). Konfigurieren Sie Ihr Telemetrie-Backend, um diese Attribute nach Bedarf zu filtern oder zu schwärzen
* Rohe Anthropic Messages API-Anfrage- und Antwort-Texte werden standardmäßig nicht protokolliert. Um sie einzubeziehen, setzen Sie `OTEL_LOG_RAW_API_BODIES` in Ihrer Shell, Benutzereinstellungen oder verwalteten Einstellungen. Es wird in [Projekt- und lokalen Einstellungen](/docs/de/settings-reference#variables-claude-code-ignores-in-env) ignoriert. Die Texte enthalten die gesamte Konversationshistorie, einschließlich des Systemprompts, jedes vorherigen Benutzer- und Assistent-Durchgangs und Tool-Ergebnisse, daher impliziert das Aktivieren dies Zustimmung zu allem, was die anderen `OTEL_LOG_*`-Content-Flags offenbaren würden. Claude Code schwärzt immer Claudes Extended-Thinking-Inhalte aus diesen Texten, unabhängig von anderen Einstellungen. Der Wert, den Sie setzen, bestimmt, wie Claude Code die Texte bereitstellt:
  * Mit `=1` gibt Claude Code `api_request_body`- und `api_response_body`-Log-Ereignisse für jeden API-Aufruf aus. Das `body`-Attribut der Ereignisse trägt die JSON-serialisierte Nutzlast, gekürzt bei der Inhaltsbegrenzung (standardmäßig 60 KB)
  * Mit `=file:<dir>` schreibt Claude Code ungekürzte Texte unter diesem Verzeichnis in `.request.json`- und `.response.json`-Dateien, und die Ereignisse tragen einen `body_ref`-Pfad statt des Inline-Textes. Versenden Sie das Verzeichnis mit einem Log-Collector oder Sidecar statt über den Telemetrie-Stream.

    Für jede erfolgreiche Antwort hängt Claude Code auch eine Zeile an `index.jsonl` in diesem Verzeichnis an, die die Antwortdatei mit der Anfragedatei verknüpft, die sie erzeugt hat, und mit der Transkriptnachricht, zu der sie wurde. Jede Zeile enthält keinen Nachrichteninhalt, und der Abschnitt [API-Antwort-Body-Ereignis](#api-response-body-event) listet seine Felder auf. Die Index-Datei erfordert Claude Code v2.1.274 oder später

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Überwachung von Claude Code auf Amazon Bedrock
</h2>

Für detaillierte Anleitung zur Überwachung der Claude Code-Nutzung für Amazon Bedrock siehe [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
