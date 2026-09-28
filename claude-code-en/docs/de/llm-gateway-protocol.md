> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Gateway-Kompatibilitätsleitfaden

> Halten Sie ein LLM-Gateway mit Claude Code kompatibel: die Endpunkte, die es aufruft, die Header und Body-Felder, die weitergeleitet werden müssen, und was bricht, wenn sie entfernt werden.

Diese Seite dokumentiert die Anfragen, die Claude Code an ein Gateway sendet, einschließlich der Endpunkte, die es aufruft, der Header und Body-Felder, die das Gateway weiterleiten muss, und welche Funktionen nicht mehr funktionieren, wenn diese nicht weitergeleitet werden. Sie ist für Betreiber geschrieben, die ein Gateway-Produkt für die Zusammenarbeit mit Claude Code konfigurieren.

Das [Claude Apps Gateway](/docs/de/claude-apps-gateway), Anthropics selbstgehostetes Gateway, stellt seine eigene Endpunkt-Referenz unter `GET /protocol` bereit, die die Sign-in-, Inference-, verwalteten Einstellungen-, Modellerkennungs- und Telemetrie-Endpunkte dieses Gateways abdeckt. Es ist ein separates Dokument von diesem Leitfaden.

<Note>
  * Um ein vorhandenes oder Gateway eines Drittanbieters für Ihre Organisation bereitzustellen, siehe [Rollout eines LLM-Gateways](/docs/de/llm-gateway-rollout)
  * Wenn Sie ein einzelner Entwickler sind, der Claude Code mit einem Ihnen gegebenen Anmeldedaten bei einem Gateway authentifiziert, siehe [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect)
</Note>

Diese Seite behandelt:

* [API-Formate](#api-formats) und die Endpunkte, die für jedes bereitgestellt werden müssen
* [Client-Verhalten nach Verbindungsmethode](#how-the-connection-method-changes-client-behavior): wie sich Modell-IDs, `anthropic-beta`-Werte, Anforderungsfelder und Standardwerte zwischen den Formaten und einer Claude Apps Gateway-Anmeldung unterscheiden
* [Anforderungs-Header](#request-headers): welche den Upstream erreichen müssen und welche Ihr Gateway verbrauchen kann
* [Antwort-Header](#response-headers): was zurückgegeben werden muss, damit Stall-Erkennung, Wiederholungen und die Anzeige von Nutzungslimits funktionieren
* Der [System-Prompt-Attributionsblock](#system-prompt-attribution-block) und wie er mit Prompt-Caching interagiert
* [Feature-Weitergabe](#feature-pass-through): was bricht, wenn Header oder Body-Felder entfernt werden
* [Modellermittlung](#model-discovery)

Diese Seite verwendet zwei Begriffe für das, was Ihr Gateway mit jedem Header und Body-Feld tut:

* **Unverändert weiterleiten**: es byte-für-byte an den Upstream weitergeben
* **Verbrauchen**: das Gateway kann es zum Routing, zur Zuordnung oder zum Tracing lesen und muss es nicht weiterleiten

Alles, das nicht als unverändert weitergeleitet markiert ist, können Sie verbrauchen oder ignorieren.

<h2 id="api-formats">
  API-Formate
</h2>

Ein Gateway muss mindestens eines der folgenden API-Formate für Claude Code-Clients bereitstellen. Ein Client wählt ein Format aus und verweist Claude Code auf Ihr Gateway mit den Variablen in der Spalte „Ausgewählt von" der folgenden Tabelle.

Google Cloud's Agent Platform ist Google Clouds Claude-Endpunkt, ehemals Vertex AI; seine Variablennamen behalten die Schreibweise `VERTEX`.

| Format                                   | Ausgewählt von                                               | Endpunkte                                                                                                       | Unverändert weitergeleitet                                                                                 |
| :--------------------------------------- | :----------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (optional)                                                          | `anthropic-beta` und `anthropic-version` Request-Header                                                    |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` mit `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (optional) | `anthropic_beta` und `anthropic_version` Request-Body-Felder                                               |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` mit `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (optional)                                        | `anthropic-beta` und `anthropic-version` Request-Header sowie das Feld `anthropic_version` im Request-Body |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry und Claude Platform on AWS
</h3>

Microsoft Foundry und die [Claude Platform on AWS](/docs/de/claude-platform-on-aws) implementieren das Anthropic Messages-Format. Claude Code leitet zu ihnen über ihre eigenen Variablen weiter, `ANTHROPIC_FOUNDRY_BASE_URL` und `ANTHROPIC_AWS_BASE_URL`, aber ein Gateway, das eines von beiden frontet, implementiert die Anthropic Messages-Zeile oben. Ein Gateway, das die Claude Platform on AWS frontet, muss auch den Header `anthropic-workspace-id` weitergeleitet, den [diese Plattform bei jeder Anfrage erfordert](/docs/de/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Optionale Endpunkte und Startup-Traffic
</h3>

Token-Counting-Endpunkte sind die einzigen optionalen: Wenn sie fehlen, greift Claude Code auf eine zeichenbasierte Schätzung der Kontextnutzung zurück.

Gleichen Sie den Pfad ab, nicht die vollständige URL:

* Inferenzanfragen werden an `/v1/messages?beta=true` gesendet
* Die Google Cloud's Agent Platform-Methode hängt Suffixe an den Publisher-Modellpfad an, wie in `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

Ein Gateway sieht auch Best-Effort-Startup-Traffic, den es ablehnen kann, ohne etwas zu unterbrechen. Ein Anthropic Messages-Format-Gateway empfängt eine `HEAD /api/hello` Verbindungs-Aufwärm-Sonde, die Claude Code überspringt, wenn ein HTTP-Proxy oder ein Client-Zertifikat konfiguriert ist. Ein Amazon Bedrock-Format-Gateway empfängt eine `GET /inference-profiles?type=SYSTEM_DEFINED` Anfrage und, wenn das konfigurierte Modell ein Inferenzprofil ist, `GET /inference-profiles/{profile}` Lookups.

Die [Fast Mode](/docs/de/fast-mode) Verfügbarkeitsprüfung erscheint niemals in Gateway-Protokollen: Sie ruft `api.anthropic.com` direkt auf, anstatt `ANTHROPIC_BASE_URL` zu folgen, sodass in einem Netzwerk, das direkten Ausgang zu `api.anthropic.com` blockiert, Fast Mode einen Konnektivitätsfehler melden kann, während die Inferenz durch das Gateway weiterhin funktioniert. Die [WebFetch-Domänensicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check) ruft auch `api.anthropic.com` direkt auf. [Verwenden Sie Fast Mode hinter Proxys und LLM-Gateways](/docs/de/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) behandelt die Variablen, die es wiederherstellen.

<h3 id="streaming">
  Streaming
</h3>

Streamen Sie Inferenzantworten. Claude Code liest den Stream, während er ankommt, also wenn Ihr Gateway vollständige Antworten puffert, bevor es sie weiterleitet, stellt Claude Code fest.

Wenn der Client das Amazon Bedrock-Format spricht, leiten Sie den `InvokeModelWithResponseStream` Response-Body und seinen Header `Content-Type: application/vnd.amazon.eventstream` unverändert weiter, und konvertieren Sie den Stream nicht in Server-Sent Events. Siehe [Streaming-Fehler hinter einem Gateway oder Proxy](/docs/de/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

Leiten Sie auch Keep-Alive-Pings weiter. Bei Verbindungen über `ANTHROPIC_BASE_URL` oder `ANTHROPIC_AWS_BASE_URL` zählt Claude Code jedes Byte, das Ihr Gateway weiterleitet, einschließlich SSE `ping` Events und Kommentarzeilen, und bricht einen Stream ab, der standardmäßig 300 Sekunden lang stumm ist. Die Pings des Upstream sind der einzige Traffic während langer Denkpausen, also wenn Ihr Gateway sie entfernt oder puffert, bricht Claude Code den Stream während dieser Pausen ab; [Automatische Wiederholungen](/docs/de/errors#automatic-retries) behandelt, was ein abgebrochener Stream basierend darauf meldet, wie weit die Antwort fortgeschritten war. Ein Upstream, der überhaupt keine Pings sendet, wie Amazon Bedrocks binärer Event-Stream, lässt diese Pausen ohne etwas zum Weiterleiten. Beim Übersetzen von einem solchen Upstream geben Sie Ihre eigenen `ping` Events während stiller Lücken aus. Gateways, die über `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL` oder `ANTHROPIC_FOUNDRY_BASE_URL` erreicht werden, sind nicht von dieser Byte-Level-Überwachung umgeben, auch wenn sie das Anthropic Messages-Format weitergeleitet; dort bricht ein [5-Minuten-Idle-Timeout](/docs/de/env-vars) einen stummen Stream statt ab, und bei `ANTHROPIC_BEDROCK_BASE_URL` Verbindungen können Sie die Byte-Überwachung mit [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/de/env-vars) hinzufügen.

<h3 id="format-mismatch-with-the-upstream">
  Format-Nichtübereinstimmung mit dem Upstream
</h3>

Welches Format der Client spricht, bestimmt, was Ihr Gateway empfängt. Der häufige Fehlermodus ist eine Nichtübereinstimmung zwischen dem Format, das der Client an Ihr Gateway sendet, und dem Format, das der Upstream-Provider dahinter akzeptiert.

* Wenn der Client das Amazon Bedrock- oder Google Cloud's Agent Platform-Format spricht, sendet Claude Code nur die Teilmenge seines vollständigen Funktionssatzes, die diese Provider akzeptieren
* Wenn der Client das Anthropic Messages-Format spricht, sendet Claude Code den vollständigen Satz, auch wenn Ihr Gateway an einen Amazon Bedrock- oder Google Cloud's Agent Platform-Upstream weiterleitet

Diese Differenz zu überbrücken ist die Aufgabe Ihres Gateways. [Feature-Durchleitung](#feature-pass-through) beschreibt, was bricht, wenn es nicht funktioniert.

Wenn Ihr Upstream Amazon Bedrock oder Google Cloud's Agent Platform ist, können Sie die Überbrückung vermeiden, indem Sie stattdessen das Format dieses Providers bereitstellen. [Route zu einem Cloud-Provider über ein Gateway](/docs/de/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) zeigt die Client-Konfiguration für dieses Format.

<h2 id="how-the-connection-method-changes-client-behavior">
  Wie die Verbindungsmethode das Verhalten des Clients ändert
</h2>

Die Art und Weise, wie ein Entwickler sich mit Ihrem Gateway verbindet, bestimmt, welche Modell-IDs, `anthropic-beta`-Werte und Anforderungsfelder Claude Code sendet, und welche Standardwerte er anwendet. Ihr Gateway sieht eines von drei Client-Verhaltensweisen:

* **Amazon Bedrock oder Agent Platform-Format**: Der Entwickler setzt `CLAUDE_CODE_USE_BEDROCK=1` mit `ANTHROPIC_BEDROCK_BASE_URL` oder `CLAUDE_CODE_USE_VERTEX=1` mit `ANTHROPIC_VERTEX_BASE_URL`, die auf Ihr Gateway verweisen. Claude Code verwendet die Modell-IDs, Anforderungsfelder und Standardwerte dieses Anbieters.
* **Anthropic Messages-Format**: Der Entwickler setzt `ANTHROPIC_BASE_URL` auf Ihr Gateway. Claude Code behandelt das Gateway als die Claude API und kann nicht erkennen, an welchen Upstream Sie weiterleiten.
* **Claude Apps Gateway-Anmeldung**: Der Entwickler meldet sich bei einem [Claude Apps Gateway](/docs/de/claude-apps-gateway) an. Dieses Gateway spricht das Anthropic Messages-Format, kann aber an jeden Upstream weiterleiten, daher sendet Claude Code nur die `anthropic-beta`-Werte und Modell-Funktionsannahmen, die Amazon Bedrock und Agent Platform ebenfalls akzeptieren.

<h3 id="requests-and-defaults-by-connection-method">
  Anforderungen und Standardwerte nach Verbindungsmethode
</h3>

Die folgende Tabelle vergleicht die drei Verbindungsmethoden, ein Verhalten pro Zeile. Sie lässt Microsoft Foundry und Claude Platform on AWS aus, die ebenfalls das Anthropic Messages-Format verwenden, aber die Claude Code über ihre eigenen Variablen erreicht. Für diese siehe die Seiten [Microsoft Foundry](/docs/de/microsoft-foundry) und [Claude Platform on AWS](/docs/de/claude-platform-on-aws).

| Verhalten                                                                                                                  | Amazon Bedrock oder Agent Platform-Format                                                                                                                                                                                          | Anthropic Messages-Format                                                                                                                                                                                       | Claude Apps Gateway-Anmeldung                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| Modell-IDs in Anforderungen standardmäßig                                                                                  | Das Format des Anbieters, z. B. `us.anthropic.claude-opus-4-8` auf Amazon Bedrock                                                                                                                                                  | Anthropic-IDs, z. B. `claude-opus-4-8`                                                                                                                                                                          | Anthropic-IDs                                                                                                                       |
| Gesendete `anthropic-beta`-Werte                                                                                           | Die Teilmenge, die Amazon Bedrock und Agent Platform akzeptieren                                                                                                                                                                   | Der vollständige Satz, der unter [Feature Pass-Through](#feature-pass-through) beschrieben ist, sofern der Entwickler nicht [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities) setzt | Die Teilmenge, die Amazon Bedrock und Agent Platform akzeptieren                                                                    |
| Anforderungsfelder für eine Modell-ID, die Claude Code nicht erkennt, z. B. ein Gateway-Alias                              | Thinking mit einem festen Budget statt adaptiver Argumentation und keine Aufwands- oder Kontextverwaltungsfelder                                                                                                                   | Alles, was aktuelle Claude-Modelle auf der Claude API akzeptieren, einschließlich adaptiver Argumentation, Aufwand und Kontextverwaltung, die ein Amazon Bedrock oder Agent Platform Upstream ablehnen kann     | Gleich wie das Amazon Bedrock oder Agent Platform-Format                                                                            |
| Eine Stunde [Prompt Cache TTL](/docs/de/prompt-caching#choose-the-ttl-yourself), wenn sich ein Entwickler anmeldet              | Angefordert über das Feld `ttl` in `cache_control`, ohne Beta-Wert                                                                                                                                                                 | Angefordert über das Feld `ttl` plus einen `extended-cache-ttl`-Wert in `anthropic-beta`, den Sie weiterleiten müssen                                                                                           | Siehe die Tabelle [Verfügbarkeit und Einschränkungen](/docs/de/claude-apps-gateway#availability-and-limitations) des Claude Apps Gateway |
| Modell für [Hintergrundaufgaben](/docs/de/costs#background-token-usage), sofern `ANTHROPIC_DEFAULT_HAIKU_MODEL` keinen festlegt | Das Standard-Sonnet-Modell oder das Hauptmodell, sobald eines ausgewählt ist, wie die Seiten [Amazon Bedrock](/docs/de/amazon-bedrock#4-pin-model-versions) und [Agent Platform](/docs/de/google-vertex-ai#5-pin-model-versions) beschreiben | Das Hauptmodell oder das Standard-Haiku-Modell, wenn `ANTHROPIC_API_KEY` oder `apiKeyHelper` einen Anthropic Console-Schlüssel bereitstellt und `ANTHROPIC_AUTH_TOKEN` nicht gesetzt ist                        | Das Hauptmodell                                                                                                                     |

Für die Funktionen, die jede Verbindung unterstützt, und die Telemetrie, die sie standardmäßig an Anthropic sendet, siehe [Feature-Verfügbarkeit](/docs/de/feature-availability#availability-by-model-provider) und [Standardverhalten nach API-Anbieter](/docs/de/data-usage#default-behaviors-by-api-provider).

<h3 id="settings-for-unrecognized-model-ids">
  Einstellungen für nicht erkannte Modell-IDs
</h3>

Zwei clientseitige Einstellungen ändern, was Claude Code für eine Modell-ID annimmt, die es nicht erkennt, unabhängig davon, welche Verbindungsmethode der Entwickler verwendet:

* **Kontextfenster**: Claude Code nimmt 200K an, oder 1M, wenn die ID `[1m]` trägt. Um das echte Fenster zu deklarieren, siehe [Korrigieren Sie das Fenster für eine Gateway- oder benutzerdefinierte Modell-ID](/docs/de/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **Funktionen**: Um einem Gateway-Alias die Funktionen des dahinter liegenden Modells zu geben, ordnen Sie die Anthropic-ID dieses Modells Ihrem Alias mit einem [`modelOverrides`](/docs/de/errors#unrecognized-model-id-on-a-request)-Eintrag in den Einstellungen zu, die Sie verteilen. Für den Ort, an dem die Variablen `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` gelten, siehe [Feature Pass-Through](#feature-pass-through)

<h2 id="request-headers">
  Request-Header
</h2>

Claude Code enthält diese Header bei API-Anfragen. Header-Namen sind auf dem Draht case-insensitiv. Leiten Sie `anthropic-version` und `anthropic-beta` unverändert weiter, plus `anthropic-workspace-id`, wenn das Upstream die [Claude Platform on AWS](/docs/de/claude-platform-on-aws) ist; der Rest kann vom Gateway zum Routing, zur Zuordnung und zum Tracing verbraucht werden und muss nicht weitergeleitet werden.

| Header                          | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization`, `x-api-key`    | Die Gateway-Anmeldedaten des Entwicklers, in einem oder beiden Headern, je nachdem, welche [Anmeldedaten-Variable](/docs/de/llm-gateway-connect#set-the-credential-variable) sie setzen                                                                                                                                                                                                                                                                                                                    |
| `anthropic-version`             | API-Version, derzeit `2023-06-01`. Amazon Bedrock- und Google Cloud Agent Platform-Format-Anfragen enthalten auch das `anthropic_version` Body-Feld, dessen Wert die Provider-Dialekt-Zeichenkette ist, nicht der Wert dieses Headers                                                                                                                                                                                                                                                                 |
| `anthropic-beta`                | Komma-getrennte Funktionswerte für die Anfrage. Leiten Sie den Header wörtlich weiter; erstellen Sie keine Allowlist einzelner Werte, da sich die Menge mit Claude Code-Versionen ändert. Wenn sich der Entwickler mit einer claude.ai-Anmeldung authentifiziert, was möglich ist, wenn `ANTHROPIC_BASE_URL` ohne eine Gateway-Anmeldedaten-Variable gesetzt ist, trägt dieser Header auch eine OAuth-Funktion, die das Upstream benötigt, und das Löschen führt zu `401` Fehlern bei diesen Anfragen |
| `x-claude-code-session-id`      | Ein eindeutiger Bezeichner für die aktuelle Claude Code-Sitzung. Verwenden Sie ihn, um alle Anfragen aus einer Sitzung zu aggregieren, ohne Request-Bodies zu analysieren                                                                                                                                                                                                                                                                                                                             |
| `x-claude-code-agent-id`        | Bezeichner des [Subagenten](/docs/de/sub-agents), der die Anfrage gestellt hat, vorhanden nur bei Anfragen von einem Agenten, den Claude Code in der Sitzung spawnt. Verwenden Sie ihn mit der Sitzungs-ID, um Kosten parallelen Agenten zuzuordnen                                                                                                                                                                                                                                                        |
| `x-claude-code-parent-agent-id` | Bezeichner des Agenten, der den anfragenden Agenten spawnt, vorhanden nur für verschachtelte Agenten                                                                                                                                                                                                                                                                                                                                                                                                  |

Subagenten-IDs werden bei jedem Spawn neu generiert. Teamkollegen-Agenten, die benannten Mitglieder eines [Agenten-Teams](/docs/de/agent-teams), verwenden eine stabile namensbasierte ID über Wiederverbindungen hinweg. In beiden Fällen identifiziert die ID einen Agenten, keine Person oder ein Gerät, daher behandeln Sie den Agenten-ID-Header nicht als Benutzerkennung.

Wenn Ihre Entwickler `ANTHROPIC_CUSTOM_HEADERS` setzen, erscheinen diese Header auch bei Anfragen.

<h3 id="gateway-hint-headers">
  Gateway-Hinweis-Header
</h3>

Claude Code kann auch Routing-Hinweise senden: anfragespezifische Fakten, die ein Gateway oder Router zum Planen, Zwischenspeichern oder Zuordnen einer Anfrage verwenden kann. Erfordert Claude Code v2.1.273 oder später.

Ob eine Anfrage diese trägt, hängt davon ab, wohin Claude Code sie sendet:

* Direkte Verbindung zur Anthropic API: standardmäßig gesendet
* Benutzerdefinierte Basis-URL: standardmäßig deaktiviert, da ein Proxy, der unbekannte Header ablehnt, die Anfrage fehlschlagen würde. Um sie zu erhalten, setzen Sie [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/de/env-vars) für Ihre Entwickler, zum Beispiel im `env` Block von [verwalteten Einstellungen](/docs/de/managed-settings)
* Jedes andere Backend, einschließlich Amazon Bedrock, Google Cloud Agent Platform, Microsoft Foundry und Claude Platform on AWS: nur gesendet, wenn `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` gesetzt ist

Das Setzen von `CLAUDE_CODE_GATEWAY_HINT_HEADERS` auf `0` stoppt die Header bei jeder Verbindung.

Die Header enthalten nur das, was die folgenden Zeilen auflisten: feste Vokabulare, Tool-Namen und Dauern, niemals Prompt-Text oder Dateiinhalte. Jeder Wert ist druckbares ASCII.

| Header                              | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `x-claude-code-request-class`       | Welche Art von Anfrage dies ist: `main` für einen Zug des Hauptgesprächs, `subagent` für einen Zug eines [Subagenten](/docs/de/sub-agents), `workflow` für einen Agenten, der in einem Workflow läuft, `compaction` für die Zusammenfassungsanfrage, die ein Gespräch komprimiert, oder `auxiliary` für Seitenanfragen wie Sitzungstitel, Klassifizierer und Zusammenfassungen. Wird bei jeder Anfrage gesendet                                                                                                                                                                                             |
| `x-claude-code-agent-type`          | Die Art des Subagenten, der die Anfrage gestellt hat: ein Name eines integrierten Agententyps wie `Explore`, `Plan` oder `general-purpose`, oder `custom` für einen benutzerdefinierten Agenten, `teammate` für ein [Agenten-Team](/docs/de/agent-teams) Mitglied, das im Prozess des Leiters läuft, oder `fork` für einen [Fork](/docs/de/sub-agents#fork-the-current-conversation). Vorhanden nur bei den eigenen Zügen eines Subagenten; die Komprimierung oder Seitenanfragen eines Subagenten behalten die Agenten-ID, tragen aber keinen Typ. Ein vom Benutzer gewählter Agentenname wird niemals gesendet |
| `x-claude-code-compaction`          | Vorhanden bei der Anfrage, die das Gespräch während einer [Komprimierung](/docs/de/prompt-caching#compacting-the-conversation) zusammenfasst. Der Wert sagt, was sie ausgelöst hat: `auto`, wenn sich das Kontextfenster der Kapazität näherte, `manual` für `/compact`, oder `reactive`, wenn die API eine Anfrage als zu lang ablehnte. Abwesend bei jeder anderen Anfrage                                                                                                                                                                                                                                |
| `x-claude-code-context-compacted`   | Vorhanden einmal, bei der ersten Hauptgesprächs-Anfrage nach einer Komprimierung, mit den gleichen Werten wie `x-claude-code-compaction`. Das Gesprächspräfix vor dieser Anfrage wird nicht mehr verwendet, daher kann ein Cache, der darauf basiert, gelöscht werden                                                                                                                                                                                                                                                                                                                                  |
| `x-claude-code-prev-tool-durations` | Gemessene Laufzeit der Tool-Aufrufe, deren Ergebnisse diese Anfrage trägt, als `<name>=<ms>;<name>=<ms>`, zum Beispiel `Bash=742;Read=9`. Wird bei der nächsten Anfrage desselben Gesprächs nach einem Batch von Tool-Aufrufen gesendet, von der Hauptsitzung oder einem Subagenten                                                                                                                                                                                                                                                                                                                    |

Bevor Sie `x-claude-code-prev-tool-durations` analysieren, überprüfen Sie, wie Claude Code den Wert erstellt und was es auslässt:

* Einträge: einer pro Tool-Aufruf, der lief, in der Reihenfolge, in der sein Ergebnis erfasst wurde, in ganzen Millisekunden
* Obergrenze: Claude Code sendet höchstens 32 Einträge und 4 KB, wobei die ersten Einträge beibehalten werden
* Kodierung: Tool-Namen sind prozentual kodiert, abdeckend `%`, `;`, `=`, Komma, Leerzeichen und jedes Zeichen außerhalb druckbarem ASCII
* Analyse: auf `;` teilen, dann auf `=`, und jeden Namen dekodieren
* Abwesenheit: Komprimierungsaufrufe, Seitenanfragen und die erste Anfrage eines neuen Prompts tragen ihn niemals. Lesen Sie einen fehlenden Header nicht als einen Zug, der keine Tools lief
* Zeiten: jede schließt Berechtigungsprompts und Hooks aus, und parallele Tool-Aufrufe berichten jeweils ihre eigene Zeit, daher addieren sich die Einträge nicht zur Lücke zwischen Anfragen

<h3 id="forward-as-open-lists">
  Weiterleitung als offene Listen
</h3>

Behandeln Sie die Header und Body-Felder als offene Listen, nicht als geschlossene. Claude Code gewinnt Funktionen über Versionen hinweg, und sie kommen als neue `anthropic-beta` Werte, neue Request-Body-Felder und gelegentlich neue `anthropic-*` oder `x-claude-code-*` Header an.

Beim Weiterleiten an ein Anthropic-Format-Upstream leiten Sie `anthropic-*` Request-Header und Request-Body-Felder unverändert durch, anstatt die heute beobachteten zu allowlisten. Ein Gateway, das an eine beobachtete Liste gepinnt ist, löscht den Header oder das Feld der nächsten Funktion und bricht es bei der Veröffentlichung, die es einführt.

Die Ausnahme ist ein Nicht-Anthropic-Upstream wie Amazon Bedrock oder Google Cloud Agent Platform, wo die Überbrückung der Schemadifferenz die Aufgabe des Gateways ist; siehe [Funktionsdurchleitung](#feature-pass-through).

<h2 id="response-headers">
  Antwortheader
</h2>

Claude Code liest diese Antwortheader, um stagnierende Streams zu erkennen, um zu entscheiden, ob und wann erneut versucht werden soll, und um Nutzungslimits anzuzeigen. Die Tabelle listet auf, was für jeden Header zurückgegeben werden soll. Leiten Sie auch Fehlerantworttexte unverändert weiter, damit Claude Code's [Fehlertoleranz bei Funktionsablehnung](#automatic-retry-and-error-forwarding) die Fehlerformulierung des Upstream-Systems abgleichen kann.

| Header                          | Was zurückgegeben werden soll und warum                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content-type`                  | Geben Sie `text/event-stream` bei gestreamten Anthropic Messages-Format-Antworten zurück und `application/vnd.amazon.eventstream`, unverändert, bei Amazon Bedrock-Format-Antworten, wobei [ein anderer Typ die Anfrage fehlschlagen lässt](/docs/de/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [Streaming](#streaming) listet auf, welche Verbindungen Stagnationserkennung auf diesen Streams ausführen                                       |
| `retry-after`                   | Geben Sie Ganzzahlsekunden statt eines HTTP-Datums zurück. Claude Code wartet mindestens so lange, bevor der nächste [automatische Wiederholungsversuch](/docs/de/errors#automatic-retries) erfolgt, und außerhalb von [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/de/env-vars) Sessions stoppt ein Wert über 60 die Wiederholungsversuche und zeigt den Fehler sofort an                                                                                                    |
| `x-should-retry`                | Leiten Sie den Wert des Upstream-Systems unverändert durch. Claude Code liest diesen Header als eine Eingabe, wenn entschieden wird, ob ein fehlgeschlagener Request erneut versucht werden soll: `true` markiert die Antwort als wiederholbar und `false` markiert sie als nicht wiederholbar. Für Wiederholungszählungen, Backoff und welche Fehler Claude Code erneut versucht, siehe [automatische Wiederholungsversuche](/docs/de/errors#automatic-retries) |
| `anthropic-ratelimit-unified-*` | Leiten Sie die Werte des Upstream-Systems unverändert bei jeder Antwort weiter. Claude Code liest sie bei erfolgreichen Antworten, um die Nutzung gegen Planlimits für bei claude.ai angemeldete Entwickler anzuzeigen, und bei einem `429`, um ein Planlimit oder Ausgabenlimit von einer temporären Drosselung zu unterscheiden; siehe [Nutzungslimits](/docs/de/errors#usage-limits)                                                                          |

<h2 id="system-prompt-attribution-block">
  System-Prompt-Attributionsblock
</h2>

Claude Code stellt einen kurzen Attributionsblock dem System-Prompt voran, der die Client-Version und einen Fingerabdruck aus dem Gespräch enthält. Der `api.anthropic.com` Endpunkt löscht den Block vor der Verarbeitung, wenn er unverändert als erster System-Block ankommt, daher beeinflusst er nicht das First-Party-Prompt-Caching. Jedes andere Upstream empfängt ihn als Teil des Prompts.

Das Löschen ist positionsbezogen, daher funktioniert es nur, wenn das Gateway das `system` Array unverändert weiterleitet. Um den Block aus dem Prompt zu halten, ohne andere System-Inhalte zu verlieren:

* Leiten Sie das `system` Array genau wie empfangen weiter, wobei Sie den Block an erster Stelle halten: Das Voranstellen eines weiteren System-Blocks, das Neuordnen des Arrays oder das Konvertieren in einen einzelnen String besiegt das Löschen, und der Block erreicht dann das Modell und den Prompt-Cache-Schlüssel.
* Halten Sie den Block in seinem eigenen Array-Eintrag: Der Endpunkt behandelt einen zusammengeführten Block, der mit dem Attributions-Header beginnt, als Attribution in ihrer Gesamtheit und löscht alles, das darin zusammengeführt wurde, einschließlich des restlichen System-Prompts.
* Wenn Ihr Gateway System-Inhalte umgestalten muss, setzen Sie [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/de/env-vars), damit Claude Code den Block auslässt. Anthropic und die Claude-Endpunkte der Cloud-Provider lesen den Block zur Zuordnung, daher lassen Sie ihn auf der Client-Seite aus, anstatt ihn im Gateway zu löschen oder zu verschieben.

Die Variable existiert für Gateway- und Third-Party-Caching-Kompatibilität, nicht als Datenschutzkontrolle: Bei einer direkten Verbindung geht die vollständige Anfrage ohnehin an die Anthropic API. Wenn beide dieser Bedingungen erfüllt sind, behält Claude Code den Block bei [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) Klassifizierungsanfragen bei, auch wenn Sie die Variable auf `0` setzen:

* Die Anfragen gehen an `api.anthropic.com`, wobei `ANTHROPIC_BASE_URL` nicht gesetzt ist oder diesen Host benennt und kein Third-Party-Provider ausgewählt ist.
* Die aktive Anmeldedaten sind keine [Anthropic-Profile oder Verbundsanmeldedaten](/docs/de/authentication#anthropic-profiles-and-federation-credentials).

Klassifizierungsanfragen überspringen den Rest des System-Prompts von Claude Code, daher ist der Block bei diesen Anfragen der einzige Marker im Request-Body, der sie als Claude Code Traffic identifiziert. Wenn eine der Bedingungen fehlschlägt, über ein LLM-Gateway, bei einem Third-Party-Provider oder mit aktiven Profile- oder Verbundsanmeldedaten, entfernt das Setzen von `0` den Block auch aus Klassifizierungsanfragen. Vor v2.1.229 existierte diese Ausnahme nicht: Das Setzen von `0` entfernte den Block aus diesen Klassifizierungsanfragen, und wenn die API die nicht identifizierten Anfragen ablehnte, schlug der Auto-Modus bei jeder Aktion fehl, die er an den Klassifizierer sendete.

Ab Claude Code v2.1.181 ist der Block für die Lebensdauer eines Gesprächs stabil, wenn Anfragen durch eine benutzerdefinierte Basis-URL geleitet werden, daher funktioniert ein Gateway-seitiger Prompt-Cache, der auf dem vollständigen Request-Body basiert, ohne ihn zu deaktivieren, und jeder Provider, an den Ihr Gateway Anfragen weiterleitet, empfängt ein stabiles Prompt-Präfix. Vor v2.1.181 enthielt der Block ein Pro-Request-Token, das den Anfang des System-Prompts bei jeder Anfrage änderte. Bei diesen Versionen setzen Sie `CLAUDE_CODE_ATTRIBUTION_HEADER=0`, wenn Ihr Gateway eines der folgenden Dinge tut:

* Implementiert einen Prompt-Cache, der auf dem Request-Body basiert.
* Leitet Anfragen an einen Third-Party-Provider wie Amazon Bedrock, Microsoft Foundry oder Google Cloud's Agent Platform weiter, im Anthropic Messages Format oder im eigenen Format des Providers, wobei das sich ändernde Präfix die Prompt-Cache-Wiederverwendung bei diesem Provider reduziert.

<h2 id="feature-pass-through">
  Funktionsdurchleitung
</h2>

Claude Code behandelt ein `ANTHROPIC_BASE_URL` Gateway als einen Anthropic-Format-Endpunkt und sendet ihm die Beta-Header und Request-Body-Felder, die es an `api.anthropic.com` sendet, außer einer kleinen Menge von Diagnosen und Standardwerten, die für direkte Verbindungen reserviert sind, wie z. B. der unten behandelte Fine-Grained-Tool-Streaming-Standard. Diese Menge variiert je nach Version, daher verlassen Sie sich nicht auf ihren Inhalt.

Funktionen, die Body-Felder hinzufügen, paaren sie mit einem Beta-Header, und das Paar reist zusammen. Ein Gateway, das den Header löscht, während es den Body durchleitet, oder ein Anthropic-Format-Body an ein Upstream mit einem anderen Schema weiterleitet, erzeugt harte `400` Fehler; nur wenn beide Hälften zusammen fehlen, schaltet sich die Funktion stillschweigend aus. Ein Gateway, das Request-Bodies zur Inhaltsüberprüfung umschreibt oder redigiert, bricht die Paarung auf die gleiche Weise wie das Löschen, daher überprüfen Sie ohne Änderung. Die Tabelle vermerkt, wo eine Funktion von der Paarung abweicht.

Fine-Grained Tool Streaming ist einer der Direct-Connection-Standardwerte: Es ist standardmäßig aus, wenn Anfragen durch eine benutzerdefinierte Basis-URL geleitet werden, und ein Gateway empfängt es, wenn Entwickler [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/de/env-vars) setzen.

| Funktion                                                                                                                                                                                                                                              | Header und Body-Paar                                                                                                                                                                                             | Symptom bei Fehler                                                                                                                                                 | Abhilfe                                                                                                                                                       |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Adaptive Reasoning](/docs/de/model-config#adjust-effort-level)                                                                                                                                                                                            | Kein Beta-Header. Claude Code sendet `thinking: {"type": "adaptive"}` für Claude 4.6 und später und behandelt Modellnamen, die es nicht erkennt, wie Gateway-Aliase, als aktuelle Modelle, die das Feld erhalten | `400` mit Nennung des `thinking` Feldes oder des `adaptive` Tags, wenn der Upstream-Modell-Build es nicht akzeptiert                                               | Aktualisieren Sie das Upstream. Auf Opus 4.6 und Sonnet 4.6 können Entwickler stattdessen `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` setzen                    |
| [Kontextverwaltung](https://platform.claude.com/docs/de/build-with-claude/context-editing)                                                                                                                                                            | Kontextverwaltungs-Beta-Header paart sich mit dem `context_management` Body-Feld                                                                                                                                 | `400` mit `Extra inputs are not permitted`. Häufig, wenn ein Gateway Anthropic-Format-Anfragen akzeptiert, aber an Amazon Bedrock weiterleitet                     | Leiten Sie beide weiter, oder [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/de/env-vars)                                                                      |
| [Erweiterter Kontext](https://platform.claude.com/docs/de/build-with-claude/context-windows#context-window-sizes-by-model) und [verschachteltes Denken](https://platform.claude.com/docs/de/build-with-claude/extended-thinking#interleaved-thinking) | Nur Beta-Header, kein Body-Feld                                                                                                                                                                                  | Stillschweigend nicht verfügbar, wenn der Header gelöscht wird; das Upstream sieht die Funktionsanfrage nie                                                        | Leiten Sie `anthropic-beta` wörtlich weiter                                                                                                                   |
| Beta [Tool-Felder](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                                            | Tool-bezogene Beta-Header paaren sich mit Tool-Schema-Feldern wie `strict` und `defer_loading`                                                                                                                   | `400` mit Nennung des nicht erkannten Tool-Schema-Feldes, wenn der Body ohne seinen Header durchgeht                                                               | Leiten Sie beide weiter, oder [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)                                                 |
| [Aufwand](https://platform.claude.com/docs/de/build-with-claude/effort) und [strukturierte Ausgaben](https://platform.claude.com/docs/de/build-with-claude/structured-outputs)                                                                        | Das `output_config` Body-Feld trägt Aufwand, strukturierte Ausgabeformat und Task-Budget-Einstellungen; jedes paart sich mit seinem eigenen Beta-Header                                                          | `400` mit Nennung von `output_config`, oft `Extra inputs are not permitted`, auf Amazon Bedrock und Google Cloud Agent Platform Upstreams                          | Leiten Sie das Feld und seine Header zusammen weiter                                                                                                          |
| [Prompt Caching](/docs/de/prompt-caching)                                                                                                                                                                                                                  | Keine Beta-Paarung. Claude Code fügt `cache_control` Marker an `system` Blöcke und an `messages` Einträge an, einschließlich `role: "system"` Einträge, die mid-conversation angehängt werden                    | Kein Fehler: das Gespräch wird bei jedem Turn als unkacherter Input abgerechnet, sichtbar als hohe `input_tokens` mit wenig oder keiner Cache-Aktivität in `usage` | Leiten Sie `cache_control` unverändert weiter, wo immer es erscheint, und konvertieren Sie nicht Block-Form `system` oder Message-Inhalte in einfache Strings |
| [Token Counting](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                                | Keine Beta-Paarung; verwendet den `count_tokens` Endpunkt                                                                                                                                                        | Kein Fehler: Claude Code fällt auf eine zeichenbasierte Schätzung zurück, daher zeigt `/context` ungefähre Zählungen                                               | Stellen Sie den Endpunkt für genaue Token-Zählungen bereit                                                                                                    |

Die `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [Variablen](/docs/de/model-config) deklarieren Modellkapazitäten nur in den Provider-Konfigurationen: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY` und [`CLAUDE_CODE_USE_MANTLE`](/docs/de/amazon-bedrock#use-the-mantle-endpoint). Sie haben keine Auswirkung hinter einem `ANTHROPIC_BASE_URL` Gateway.

<h3 id="automatic-retry-and-error-forwarding">
  Automatische Wiederholung und Fehlerweiterleitung
</h3>

Was Claude Code nach einer Upstream-Ablehnung tut, hängt davon ab, was abgelehnt wurde:

* Wenn das Upstream das `thinking` Feld, eine Mid-Conversation-Systemnachricht oder den `cache_control` Marker auf einer solchen Nachricht ablehnt, versucht Claude Code die Anfrage erneut und deaktiviert die abgelehnte Funktion für den Rest des Gesprächs
* Wenn das Upstream eine [Thinking-Signatur](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) ablehnt, einschließlich mit einem `400` dessen Nachricht besagt, dass der Block `bound to a different conversation` ist, entfernt Claude Code frühere Thinking-Blöcke aus der Anfrage, versucht erneut und hält sie aus jeder späteren Anfrage heraus. Neue Antworten enthalten immer noch Thinking
* Wenn das Gateway oder sein Upstream den [Advisor-Tool](/docs/de/advisor) Eintrag in `tools` als einen nicht erkannten Tool-Typ ablehnt, versucht Claude Code die Anfrage einmal ohne diesen Eintrag und seinen `anthropic-beta` Wert erneut. Spätere Anfragen an diese Basis-URL lassen den Advisor aus, bis Claude Code beendet wird, und `/advisor` ist für den Entwickler für diese Zeit nicht verfügbar. Claude Code erkennt diese Ablehnung durch eine `400` oder `422` Antwort, deren Nachricht den Tool-Typ nach `Input tag` benennt, wie z. B. `Input tag 'advisor_20260301'`. Vor v2.1.280 wiederholte Claude Code diese Ablehnung nicht
* Claude Code versucht Ablehnungen von Kontextverwaltungs- oder Tool-Schema-Feldern nicht erneut, daher erreichen diese `400` Fehler den Entwickler

Die `bound to a different conversation` Ablehnung kommt von der API-Überprüfung für [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), die fehlschlägt, wenn `system`, `tools` oder frühere `messages` Inhalte sich von der Anfrage unterscheiden, die das Thinking produziert hat. Ein Gateway, das einen dieser Inhalte umschreibt, kann die Ablehnung selbst verursachen; [Libraries, proxies, and gateways](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways) behandelt, was unverändert durchgeleitet werden muss.

Die Wiederholungslogik gleicht die Fehlerformulierung des Upstreams ab, daher leiten Sie Fehler-Response-Bodies unverändert weiter. Ein Gateway, das Upstream-Fehler in seine eigene Hülle einwickelt, bricht den Wiederherstellungspfad, auch wenn es den Statuscode beibehält, es sei denn, die Nachricht der Hülle trägt ein stabiles `capability_rejected:` Token. [Claude Apps Gateway ersetzt diese Tokens für Cloud-Provider-Fehlerformulierungen](/docs/de/claude-apps-gateway-config#upstream-error-messages), zum Beispiel `capability_rejected: prompt_too_long`.

<h3 id="disable-pre-release-capabilities">
  Deaktivieren Sie Pre-Release-Funktionen
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` stoppt Claude Code vom Senden von Pre-Release-Funktionen und ihren Body-Feldern auf jedem Provider, einschließlich Kontextverwaltung und der Beta-Tool-Felder. Die Variable beeinflusst nicht adaptive Reasoning, das nach Modell ausgewählt wird, nicht nach Beta. Sie unterdrückt nie die OAuth-Funktion, die die Abonnement-Authentifizierung benötigt.

Auf Claude Code v2.1.227 oder später kann Ihre Organisation [MCP Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) unter dieser Variable durch [verwaltete Einstellungen](/docs/de/managed-settings) aktiviert halten. Was Claude Code mit dieser Außerkraftsetzung sendet, hängt davon ab, wie Sie sich verbinden:

* Bei einer direkten Verbindung oder durch ein Gateway mit `ANTHROPIC_BASE_URL` sendet Claude Code weiterhin den Tool-Search-Beta-Header, `defer_loading` Tool-Felder und `tool_reference` Blöcke und entfernt den Rest
* Bei einem Cloud-Provider oder angemeldet durch ein [Claude Apps Gateway](/docs/de/claude-apps-gateway), hat die Außerkraftsetzung keine Auswirkung

Die Menge der Funktionen, die Claude Code sendet, wächst über Versionen. Für aktuelle Beta-Header-Zeichenketten siehe die [Beta-Headers-Referenz](https://platform.claude.com/docs/de/api/beta-headers); testen Sie Ihr Gateway gegen neue Claude Code-Versionen, anstatt an eine beobachtete Liste zu pinnen.

<h2 id="model-discovery">
  Modellermittlung
</h2>

Wenn `ANTHROPIC_BASE_URL` auf ein Gateway verweist, das das Anthropic Messages-Format bereitstellt, kann Claude Code beim Startup den `/v1/models` Endpunkt des Gateways abfragen und die zurückgegebenen Modelle zur `/model` Auswahl hinzufügen. Wenn Sie oder Ihr Administrator `replaceBuiltInOptions` in einer [`modelPicker`](/docs/de/settings-reference#modelpicker) Konfiguration setzen, blendet Claude Code die ermittelten Modelle aus der Auswahl aus.

Entwickler aktivieren dies durch Setzen von [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/de/env-vars), in ihrer eigenen Umgebung oder durch verwaltete Einstellungen. Die Ermittlung ist standardmäßig aus, damit Gateways, die von einem gemeinsamen API-Schlüssel unterstützt werden, nicht jedes Modell, auf das der Schlüssel zugreifen kann, jedem Benutzer anzeigen.

<h3 id="when-discovery-runs">
  Wenn die Ermittlung läuft
</h3>

Die Ermittlung gilt nur für das Anthropic Messages-Format. Sie läuft nicht, wenn:

* Eine beliebige `CLAUDE_CODE_USE_*` Provider-Variable gesetzt ist, auch wenn `ANTHROPIC_BASE_URL` auch gesetzt ist
* `ANTHROPIC_BASE_URL` nicht gesetzt ist oder auf `api.anthropic.com` verweist

Die Ermittlung läuft weiterhin, wenn [nicht wesentlicher Traffic deaktiviert ist](/docs/de/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path), da die Anfrage nur an Ihr Gateway geht. Vor v2.1.257 lief die Ermittlung nicht, während nicht wesentlicher Traffic deaktiviert war.

<h3 id="request-and-response">
  Request und Response
</h3>

Die Anfrage ist `GET /v1/models?limit=1000` mit einem 3-Sekunden-Timeout, und jede Umleitung wird als Fehler behandelt, daher können die Anmeldedaten nicht an ein Umleitungsziel durchsickern. Ein Gateway, das langsamer antwortet als das Timeout, oder eines, das `/v1/models` umleitet, auch `http` zu `https`, schlägt die Ermittlung stillschweigend fehl; stellen Sie den Endpunkt direkt unter der konfigurierten Basis-URL bereit.

Um einem langsamen Gateway mehr Zeit zu geben, setzen Sie [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/de/env-vars#variables). Die Variable erfordert Claude Code v2.1.269 oder später.

Claude Code sendet die Ermittlungsanfrage mit beiden Anmeldedaten-Headern unten und lässt einen Header weg, dessen Wert sich nicht auflöst. Das Senden beider Header erfordert Claude Code v2.1.248 oder später. Frühere Versionen senden nur `Authorization`, wenn `ANTHROPIC_AUTH_TOKEN` gesetzt ist, und nur `x-api-key` andernfalls.

* `Authorization`: `ANTHROPIC_AUTH_TOKEN` als Bearer-Token, andernfalls der [`apiKeyHelper`](/docs/de/llm-gateway-connect#rotate-credentials-with-apikeyhelper) Wert als Bearer-Token. In diesem Fall wartet Claude Code, bis der Helper zurückkommt, bevor die Anfrage gesendet wird.
* `x-api-key`: der API-Schlüssel, den Claude Code aufgelöst hat, wie `ANTHROPIC_API_KEY`. Wenn ein Helper-Wert die einzige Anmeldedaten ist, trägt dieser Header ihn auch, daher kommt der Wert in beiden Headern an.

Claude Code sendet auch alle Header von `ANTHROPIC_CUSTOM_HEADERS`. Wenn ein benutzerdefinierter Header einen nicht leeren Wert hat, sendet Claude Code ihn anstelle eines eingebauten Headers mit demselben Namen, wobei die Namen Groß- und Kleinschreibung ignoriert werden.

Wenn sich der Wert keines Anmeldedaten-Headers auflöst, überspringt Claude Code die Ermittlung und schreibt eine `[gatewayDiscovery] skipped` Zeile in das Debug-Protokoll einer `claude --debug` Sitzung. Wenn Sie eine Anmeldedaten nur über `ANTHROPIC_CUSTOM_HEADERS` bereitstellen, überspringt Claude Code weiterhin die Ermittlung.

Claude Code liest `id`, den optionalen `display_name` und die optionale `description` aus jedem Eintrag im `data` Array der Response:

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code behält einen Eintrag, wenn sein `id` `claude` oder `anthropic` irgendwo in der Zeichenkette enthält, wobei Groß- und Kleinschreibung ignoriert wird, und ignoriert den Rest. Provider-Präfix-IDs wie `vertex_ai/claude-sonnet-4-6` oder `bedrock/anthropic.claude-sonnet-4-5` bestehen den Filter; eine ID, die keine der beiden Teilzeichenketten enthält, nicht. Vor v2.1.223 behielt Claude Code einen Eintrag nur, wenn sein `id` mit `claude` oder `anthropic` begann, was Provider-Präfix-IDs verbarg.

<h3 id="picker-entries-and-caching">
  Auswahl-Einträge und Caching
</h3>

Die Auswahl ist die interaktive Modelliste, die sich öffnet, wenn ein Entwickler `/model` in Claude Code ausführt. Jeder ermittelte Eintrag verwendet `display_name` als seinen Namen, wenn das Gateway einen sendet, der sich vom `id` unterscheidet. Andernfalls zeigt der Eintrag den Namen des Modells, wenn Claude Code die `id` [erkennt](/docs/de/model-config#customize-pinned-model-display-and-capabilities), und die `id`, wenn nicht. Zum Beispiel erscheint ein Eintrag mit der `id` `my-gateway-claude-sonnet-4-6` und ohne `display_name` als `Sonnet 4.6`.

Die Ermittlung fügt nur Modelle hinzu, die die [`availableModels` verwaltete Einstellung](/docs/de/settings-reference#availablemodels) erlaubt.

Jeder Eintrag zeigt auch die `description` des Modells, auf eine Zeile zusammengefasst. Ein Eintrag ohne `description` liest stattdessen „From gateway". Vor v2.1.257 las jeder ermittelte Eintrag „From gateway".

Eine ermittelte ID erhält keine eigene Zeile, wenn sie einer Zeile in der Auswahl bereits entspricht:

* Gleiche ID: die ermittelte ID entspricht genau der ID einer vorhandenen Zeile, oder die beiden IDs sind Schreibweisen derselben [Fable](/docs/de/model-config#work-with-fable) Version.
* Gleiches Modell wie ein eingebauter Alias: wenn eine ermittelte explizite ID das Modell benennt, zu dem ein eingebauter Alias derzeit aufgelöst wird, zeigt die Auswahl nur die Alias-Zeile. Zum Beispiel, während `sonnet` zu `claude-sonnet-5` aufgelöst wird, wird eine ermittelte `claude-sonnet-5` in die `sonnet` Zeile zusammengefasst, und eine ermittelte `claude-sonnet-4-6` erhält immer noch ihre eigene Zeile. Vor v2.1.197 faltete Claude Code diese IDs nicht in eingebaute Zeilen, daher erhielt `claude-sonnet-5` auch ihre eigene „From gateway" Zeile.

Ergebnisse werden in `~/.claude/cache/gateway-models.json` oder `%USERPROFILE%\.claude\cache\gateway-models.json` unter Windows zwischengespeichert und bei jedem Startup aktualisiert. Wenn Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) setzen, lebt der Cache stattdessen unter diesem Verzeichnis. Wenn die Anfrage fehlschlägt oder das Gateway `/v1/models` nicht implementiert, fällt die Auswahl auf die zwischengespeicherte Liste aus dem vorherigen Startup oder auf die eingebaute Modelliste zurück. Wenn Ihr Gateway Claude-Modelle unter Aliasen bereitstellt, die nicht dem Ermittlungsfilter entsprechen, können Entwickler diese Aliase manuell mit den [Modellkonfigurationsvariablen](/docs/de/model-config) hinzufügen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Für den Rest der Gateway-Dokumentationsserie und die zugrunde liegenden API-Referenzen:

* [Gateway-Übersicht](/docs/de/gateways): was ein Gateway ist und wie Sie zwischen Claude Apps Gateway und einem anderen Produkt wählen
* [Andere LLM-Gateways](/docs/de/llm-gateway): wie Sie ein Gateway bereitstellen, das Ihre Organisation betreibt, und wie es mit claude.ai-Abonnements interagiert
* [LLM-Gateway für Ihre Organisation bereitstellen](/docs/de/llm-gateway-rollout): die Admin-Checkliste, die diesen Leitfaden verwendet
* [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect): Pro-Entwickler-Konfiguration und die Fehlerbehebungstabelle
* [Beta-Headers-Referenz](https://platform.claude.com/docs/de/api/beta-headers): der aktuelle Satz von `anthropic-beta` Werten
* [Messages API](https://platform.claude.com/docs/de/api/messages): das API-Format, das ein Anthropic-Format-Gateway implementiert
