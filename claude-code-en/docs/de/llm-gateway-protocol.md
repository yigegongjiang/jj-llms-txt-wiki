> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway-Protokollreferenz

> Der API-Vertrag zwischen Claude Code und einem LLM-Gateway: Endpunkte, Header und Body-Felder zum Weiterleiten, Funktionsverschlechterung bei gelöschten Feldern, Attributions-Header für Kostenverfolgung und Modellermittlung.

Diese Seite dokumentiert die Anfragen, die Claude Code an ein Gateway sendet, einschließlich der Endpunkte, die es aufruft, der Header und Body-Felder, die das Gateway weiterleiten muss, und welche Funktionen nicht mehr funktionieren, wenn dies nicht der Fall ist. Sie ist für Operatoren geschrieben, die ein Gateway-Produkt für die Zusammenarbeit mit Claude Code konfigurieren.

Ein laufendes [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) stellt eine maschinenlesbare Version dieses Vertrags unter `GET /protocol` bereit, die dieselben Weiterleitungsanforderungen sowie die Claude-Apps-Gateway-spezifischen Endpunkte für SSO-Anmeldung, verwaltete Einstellungsbereitstellung und Telemetrie abdeckt. Claude-Apps-Gateway wird aus derselben `claude`-Binärdatei wie die CLI ausgeführt, daher ist der [Claude-Apps-Gateway-Schnellstart](/docs/de/claude-apps-gateway#quickstart) der kürzeste Weg zu einer laufenden Instanz, von der Sie die Spezifikation abrufen können.

<Note>
  * Um ein vorhandenes oder Gateway eines Drittanbieters für Ihre Organisation bereitzustellen, siehe [LLM-Gateway bereitstellen](/docs/de/llm-gateway-rollout)
  * Wenn Sie ein einzelner Entwickler sind, der Claude Code mit einer Anmeldedaten, die Sie erhalten haben, bei einem Gateway authentifiziert, siehe [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect)
</Note>

Diese Seite behandelt:

* [API-Formate](#api-formats) und die Endpunkte, die für jedes bereitgestellt werden sollen
* [Request-Header](#request-headers): welche das Upstream erreichen müssen und welche Ihr Gateway verbrauchen kann
* Der [System-Prompt-Attributionsblock](#system-prompt-attribution-block) und wie er mit Prompt-Caching interagiert
* [Funktionsdurchleitung](#feature-pass-through): was bricht, wenn Header oder Body-Felder gelöscht werden
* [Modellermittlung](#model-discovery)

Diese Seite verwendet zwei Begriffe für das, was Ihr Gateway mit jedem Header und Body-Feld tut:

* **Unverändert weiterleiten**: es Byte-für-Byte an das Upstream weitergeben
* **Verbrauchen**: das Gateway kann es zum Routing, zur Zuordnung oder zum Tracing lesen und muss es nicht weiterleiten

Alles, das nicht als unverändert weiterleiten gekennzeichnet ist, können Sie verbrauchen oder ignorieren.

<h2 id="api-formats">
  API-Formate
</h2>

Ein Gateway muss mindestens eines der folgenden API-Formate für Claude Code-Clients bereitstellen. Welches Format Claude Code spricht, wird durch die Konfiguration des Clients bestimmt: die Variable in der Spalte „Ausgewählt von" der folgenden Tabelle verweist Claude Code auf Ihr Gateway in diesem Format. Google Cloud's Agent Platform ist Googles Claude-Endpunkt in Google Cloud, ehemals Vertex AI; seine Variablennamen behalten die Schreibweise `VERTEX`.

| Format                                   | Ausgewählt von                                               | Endpunkte                                                                | Unverändert weiterleiten                                                                                |
| :--------------------------------------- | :----------------------------------------------------------- | :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (optional)                   | `anthropic-beta` und `anthropic-version` Request-Header                                                 |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` mit `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`    | `anthropic_beta` und `anthropic_version` Request-Body-Felder                                            |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` mit `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (optional) | `anthropic-beta` und `anthropic-version` Request-Header sowie das `anthropic_version` Request-Body-Feld |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry und Claude Platform on AWS
</h3>

Microsoft Foundry und die [Claude Platform on AWS](/docs/de/claude-platform-on-aws) implementieren das Anthropic Messages-Format. Claude Code leitet sie über ihre eigenen Variablen weiter, `ANTHROPIC_FOUNDRY_BASE_URL` und `ANTHROPIC_AWS_BASE_URL`, aber ein Gateway, das eines von beiden frontet, implementiert die Anthropic Messages-Zeile oben. Ein Gateway, das die Claude Platform on AWS frontet, muss auch den `anthropic-workspace-id`-Header weiterleiten, den [diese Plattform bei jeder Anfrage benötigt](/docs/de/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Optionale Endpunkte und Startup-Traffic
</h3>

Token-Counting-Endpunkte sind die einzigen optionalen: Wenn sie fehlen, schätzt Claude Code die Kontextnutzung lokal. Inferenzanfragen werden an `/v1/messages?beta=true` gesendet, daher sollten Sie auf dem Pfad abgleichen, nicht auf der vollständigen URL. Die Google Cloud's Agent Platform-Methode hängt Suffixe an den Publisher-Modellpfad an, wie in `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

Ein Gateway sieht auch Best-Effort-Startup-Traffic, den es ablehnen kann, ohne etwas zu unterbrechen: eine `HEAD /` Konnektivitätsprobe und auf Amazon Bedrock-Format-Gateways eine `GET /inference-profiles?type=SYSTEM_DEFINED` Anfrage.

<h3 id="streaming">
  Streaming
</h3>

Inferenzantworten müssen streamen. Claude Code verbraucht Server-Sent Events, wenn sie ankommen, daher stellt ein Gateway, das vollständige Antworten puffert, bevor es sie weiterleitet, den Client still.

<h3 id="format-mismatch-with-the-upstream">
  Format-Mismatch mit dem Upstream
</h3>

Welches Format der Client spricht, bestimmt, was Ihr Gateway empfängt. Der häufige Fehlermodus ist ein Mismatch zwischen dem Format, das der Client an Ihr Gateway sendet, und dem Format, das der Upstream-Provider dahinter akzeptiert.

* Wenn der Client das Amazon Bedrock- oder Google Cloud's Agent Platform-Format spricht, sendet Claude Code nur die Teilmenge seiner vollständigen Funktionsmenge, die diese Provider akzeptieren
* Wenn der Client das Anthropic Messages-Format spricht, sendet Claude Code die vollständige Menge, auch wenn Ihr Gateway an ein Amazon Bedrock- oder Google Cloud's Agent Platform-Upstream weiterleitet

Diese Differenz zu überbrücken ist die Aufgabe Ihres Gateways. [Funktionsdurchleitung](#feature-pass-through) beschreibt, was bricht, wenn dies nicht der Fall ist.

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

<h3 id="forward-as-open-lists">
  Weiterleitung als offene Listen
</h3>

Behandeln Sie die Header und Body-Felder als offene Listen, nicht als geschlossene. Claude Code gewinnt Funktionen über Versionen hinweg, und sie kommen als neue `anthropic-beta` Werte, neue Request-Body-Felder und gelegentlich neue `anthropic-*` oder `x-claude-code-*` Header an.

Beim Weiterleiten an ein Anthropic-Format-Upstream leiten Sie `anthropic-*` Request-Header und Request-Body-Felder unverändert durch, anstatt die heute beobachteten zu allowlisten. Ein Gateway, das an eine beobachtete Liste gepinnt ist, löscht den Header oder das Feld der nächsten Funktion und bricht es bei der Veröffentlichung, die es einführt.

Die Ausnahme ist ein Nicht-Anthropic-Upstream wie Amazon Bedrock oder Google Cloud Agent Platform, wo die Überbrückung der Schemadifferenz die Aufgabe des Gateways ist; siehe [Funktionsdurchleitung](#feature-pass-through).

<h2 id="system-prompt-attribution-block">
  System-Prompt-Attributionsblock
</h2>

Claude Code stellt einen kurzen Attributionsblock dem System-Prompt voran, der die Client-Version und einen Fingerabdruck aus dem Gespräch enthält. Der `api.anthropic.com` Endpunkt löscht den Block vor der Verarbeitung, wenn er unverändert als erster System-Block ankommt, daher beeinflusst er nicht das First-Party-Prompt-Caching. Jedes andere Upstream empfängt ihn als Teil des Prompts.

Das Löschen ist positionsbezogen, daher funktioniert es nur, wenn das Gateway das `system` Array unverändert weiterleitet. Um den Block aus dem Prompt zu halten, ohne andere System-Inhalte zu verlieren:

* Leiten Sie das `system` Array genau wie empfangen weiter, wobei Sie den Block an erster Stelle halten: Das Voranstellen eines weiteren System-Blocks, das Neuordnen des Arrays oder das Konvertieren in einen einzelnen String besiegt das Löschen, und der Block erreicht dann das Modell und den Prompt-Cache-Schlüssel.
* Halten Sie den Block in seinem eigenen Array-Eintrag: Der Endpunkt behandelt einen zusammengeführten Block, der mit dem Attributions-Header beginnt, als Attribution in ihrer Gesamtheit und löscht alles, das darin zusammengeführt wurde, einschließlich des restlichen System-Prompts.
* Wenn Ihr Gateway System-Inhalte umgestalten muss, setzen Sie [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/de/env-vars), damit Claude Code den Block auslässt. Anthropic und die Claude-Endpunkte der Cloud-Provider lesen den Block zur Zuordnung, daher lassen Sie ihn auf der Client-Seite aus, anstatt ihn im Gateway zu löschen oder zu verschieben.

Anfragen, die unverändert den Endpunkt erreichen, sind nicht betroffen.

Ab Claude Code v2.1.181 ist der Block für die Lebensdauer eines Gesprächs stabil, wenn Anfragen durch eine benutzerdefinierte Basis-URL geleitet werden, daher funktioniert ein Gateway-seitiger Prompt-Cache, der auf dem vollständigen Request-Body basiert, ohne ihn zu deaktivieren. Vor v2.1.181 enthielt der Block ein Pro-Request-Token; setzen Sie auf diesen Versionen `CLAUDE_CODE_ATTRIBUTION_HEADER=0`, wenn Ihr Gateway einen solchen Cache implementiert.

<h2 id="feature-pass-through">
  Funktionsdurchleitung
</h2>

Claude Code behandelt ein `ANTHROPIC_BASE_URL` Gateway als einen Anthropic-Format-Endpunkt und sendet ihm die Beta-Header und Request-Body-Felder, die es an `api.anthropic.com` sendet, außer einer kleinen Menge von Diagnosen und Standardwerten, die für direkte Verbindungen reserviert sind, wie z. B. der unten behandelte Fine-Grained-Tool-Streaming-Standard. Diese Menge variiert je nach Version, daher verlassen Sie sich nicht auf ihren Inhalt.

Funktionen, die Body-Felder hinzufügen, paaren sie mit einem Beta-Header, und das Paar reist zusammen. Ein Gateway, das den Header löscht, während es den Body durchleitet, oder ein Anthropic-Format-Body an ein Upstream mit einem anderen Schema weiterleitet, erzeugt harte `400` Fehler; nur wenn beide Hälften zusammen fehlen, schaltet sich die Funktion stillschweigend aus. Ein Gateway, das Request-Bodies zur Inhaltsüberprüfung umschreibt oder redigiert, bricht die Paarung auf die gleiche Weise wie das Löschen, daher überprüfen Sie ohne Änderung. Die Tabelle vermerkt, wo eine Funktion von der Paarung abweicht.

Fine-Grained Tool Streaming ist einer der Direct-Connection-Standardwerte: Es ist standardmäßig aus, wenn Anfragen durch eine benutzerdefinierte Basis-URL geleitet werden, und ein Gateway empfängt es, wenn Entwickler [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/de/env-vars) setzen.

| Funktion                                                                                                                                                                                                                                              | Header und Body-Paar                                                                                                                                                                                             | Symptom bei Fehler                                                                                                                             | Abhilfe                                                                                                                                    |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [Adaptive Reasoning](/docs/de/model-config#adjust-effort-level)                                                                                                                                                                                            | Kein Beta-Header. Claude Code sendet `thinking: {"type": "adaptive"}` für Claude 4.6 und später und behandelt Modellnamen, die es nicht erkennt, wie Gateway-Aliase, als aktuelle Modelle, die das Feld erhalten | `400` mit Nennung des `thinking` Feldes oder des `adaptive` Tags, wenn der Upstream-Modell-Build es nicht akzeptiert                           | Aktualisieren Sie das Upstream. Auf Opus 4.6 und Sonnet 4.6 können Entwickler stattdessen `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` setzen |
| [Kontextverwaltung](https://platform.claude.com/docs/de/build-with-claude/context-editing)                                                                                                                                                            | Kontextverwaltungs-Beta-Header paart sich mit dem `context_management` Body-Feld                                                                                                                                 | `400` mit `Extra inputs are not permitted`. Häufig, wenn ein Gateway Anthropic-Format-Anfragen akzeptiert, aber an Amazon Bedrock weiterleitet | Leiten Sie beide weiter, oder [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/de/env-vars)                                                   |
| [Erweiterter Kontext](https://platform.claude.com/docs/de/build-with-claude/context-windows#context-window-sizes-by-model) und [verschachteltes Denken](https://platform.claude.com/docs/de/build-with-claude/extended-thinking#interleaved-thinking) | Nur Beta-Header, kein Body-Feld                                                                                                                                                                                  | Stillschweigend nicht verfügbar, wenn der Header gelöscht wird; das Upstream sieht die Funktionsanfrage nie                                    | Leiten Sie `anthropic-beta` wörtlich weiter                                                                                                |
| Beta [Tool-Felder](https://platform.claude.com/docs/de/agents-and-tools/tool-use/overview)                                                                                                                                                            | Tool-bezogene Beta-Header paaren sich mit Tool-Schema-Feldern wie `strict` und `defer_loading`                                                                                                                   | `400` mit Nennung des nicht erkannten Tool-Schema-Feldes, wenn der Body ohne seinen Header durchgeht                                           | Leiten Sie beide weiter, oder `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                                   |
| [Aufwand](https://platform.claude.com/docs/de/build-with-claude/effort) und [strukturierte Ausgaben](https://platform.claude.com/docs/de/build-with-claude/structured-outputs)                                                                        | Das `output_config` Body-Feld trägt Aufwand, strukturierte Ausgabeformat und Task-Budget-Einstellungen; jedes paart sich mit seinem eigenen Beta-Header                                                          | `400` mit Nennung von `output_config`, oft `Extra inputs are not permitted`, auf Bedrock- und Agent-Platform-Upstreams                         | Leiten Sie das Feld und seine Header zusammen weiter                                                                                       |
| [Token-Counting](https://platform.claude.com/docs/de/build-with-claude/token-counting)                                                                                                                                                                | Keine Beta-Paarung; verwendet den `count_tokens` Endpunkt                                                                                                                                                        | Claude Code fällt auf lokale Kontextnutzungsschätzung zurück                                                                                   | Stellen Sie den Endpunkt bereit, wenn Sie genaue Zählungen möchten                                                                         |

Die `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [Variablen](/docs/de/model-config) deklarieren Modellkapazitäten nur in den Provider-Konfigurationen: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY` und [`CLAUDE_CODE_USE_MANTLE`](/docs/de/amazon-bedrock#use-the-mantle-endpoint). Sie haben keine Auswirkung hinter einem `ANTHROPIC_BASE_URL` Gateway.

<h3 id="automatic-retry-and-error-forwarding">
  Automatische Wiederholung und Fehlerweiterleitung
</h3>

Claude Code versucht automatisch nach einigen Upstream-Ablehnungen und deaktiviert die abgelehnte Funktion für den Rest des Gesprächs. Ablehnungen des `thinking` Feldes, von [Thinking-Signaturen](https://platform.claude.com/docs/de/build-with-claude/extended-thinking) und von Mid-Conversation-Systemnachrichten erholen sich auf diese Weise. Kontextverwaltungs- und Tool-Schema-Feld-Ablehnungen versuchen nicht erneut; diese `400` Fehler erreichen den Entwickler.

Die Wiederholungslogik gleicht die Fehlerformulierung des Upstreams ab, daher leiten Sie Fehler-Response-Bodies unverändert weiter. Ein Gateway, das Upstream-Fehler in seine eigene Hülle einwickelt, bricht den Wiederherstellungspfad, auch wenn es den Statuscode beibehält.

<h3 id="disable-pre-release-capabilities">
  Deaktivieren Sie Pre-Release-Funktionen
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` stoppt Claude Code vom Senden von Pre-Release-Funktionen und ihren Body-Feldern auf jedem Provider, einschließlich Kontextverwaltung und der Beta-Tool-Felder. Es beeinflusst nicht adaptive Reasoning, das nach Modell ausgewählt wird, nicht nach Beta, und es unterdrückt nie die OAuth-Funktion, die die Abonnement-Authentifizierung benötigt.

Die Menge der Funktionen, die Claude Code sendet, wächst über Versionen. Für aktuelle Beta-Header-Zeichenketten siehe die [Beta-Headers-Referenz](https://platform.claude.com/docs/de/api/beta-headers); testen Sie Ihr Gateway gegen neue Claude Code-Versionen, anstatt an eine beobachtete Liste zu pinnen.

<h2 id="model-discovery">
  Modellermittlung
</h2>

Wenn `ANTHROPIC_BASE_URL` auf ein Gateway verweist, das das Anthropic Messages-Format bereitstellt, kann Claude Code beim Startup den `/v1/models` Endpunkt des Gateways abfragen und die zurückgegebenen Modelle zur `/model` Auswahl hinzufügen.

Entwickler aktivieren dies durch Setzen von [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/de/env-vars), in ihrer eigenen Umgebung oder durch verwaltete Einstellungen. Die Ermittlung ist standardmäßig aus, damit Gateways, die von einem gemeinsamen API-Schlüssel unterstützt werden, nicht jedes Modell, auf das der Schlüssel zugreifen kann, jedem Benutzer anzeigen. Dies erfordert Claude Code v2.1.129 oder später.

<h3 id="when-discovery-runs">
  Wenn die Ermittlung läuft
</h3>

Die Ermittlung gilt nur für das Anthropic Messages-Format. Sie läuft nicht, wenn:

* Eine beliebige `CLAUDE_CODE_USE_*` Provider-Variable gesetzt ist, auch wenn `ANTHROPIC_BASE_URL` auch gesetzt ist
* `ANTHROPIC_BASE_URL` nicht gesetzt ist oder auf `api.anthropic.com` verweist
* Nicht wesentlicher Traffic ist deaktiviert, durch [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) oder Organisationsrichtlinie

<h3 id="request-and-response">
  Request und Response
</h3>

Die Anfrage ist `GET /v1/models?limit=1000` mit einem 3-Sekunden-Timeout, und jede Umleitung wird als Fehler behandelt, daher können die Anmeldedaten nicht an ein Umleitungsziel durchsickern. Ein Gateway, das langsam antwortet oder `/v1/models` umleitet, auch `http` zu `https`, schlägt die Ermittlung stillschweigend fehl; stellen Sie den Endpunkt direkt unter der konfigurierten Basis-URL bereit.

Die Ermittlungsanfrage sendet genau einen Anmeldedaten-Header:

* `ANTHROPIC_AUTH_TOKEN` als Bearer-Token, wenn gesetzt
* Andernfalls der aufgelöste API-Schlüssel, einschließlich eines [`apiKeyHelper`](/docs/de/llm-gateway-connect#rotate-credentials-with-apikeyhelper) Wertes, im `x-api-key` Header

Dies unterscheidet sich von Inferenzanfragen, die einen Helper-Wert in beiden Headern senden. Ein Gateway, das `/v1/models` authentifiziert, muss `x-api-key` für Helper-Bereitstellungen akzeptieren. Alle Header von `ANTHROPIC_CUSTOM_HEADERS` sind ebenfalls enthalten.

Claude Code liest `id` und den optionalen `display_name` aus jedem Eintrag im `data` Array der Response und ignoriert Einträge, deren `id` nicht mit `claude` oder `anthropic` beginnt:

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  Auswahl-Einträge und Caching
</h3>

Die Auswahl ist die interaktive Modelliste, die sich öffnet, wenn ein Entwickler `/model` in Claude Code ausführt. Jeder ermittelte Eintrag ist mit „Aus Gateway" gekennzeichnet und verwendet `display_name`, wenn bereitgestellt. Die [`availableModels` verwaltete Einstellung](/docs/de/settings#available-settings) begrenzt, was die Ermittlung hinzufügen kann.

Eine ermittelte ID wird übersprungen, wenn sie genau einer Zeile in der Auswahl entspricht, oder wenn sowohl die ermittelte als auch die vorhandene ID zu [Fable](/docs/de/model-config#work-with-fable-5) aufgelöst werden. Ab Claude Code v2.1.197 wird eine ermittelte explizite ID auch in einen eingebauten Eintrag zusammengefasst, wenn beide zum gleichen Modell aufgelöst werden. Eingebaute Zeilen werden nach Aliasen wie `sonnet` verschlüsselt, daher wird eine ermittelte explizite ID des Modells, zu dem der Alias derzeit aufgelöst wird, wie `claude-sonnet-5`, in die `sonnet` Zeile zusammengefasst, während eine ID, zu der der Alias nicht aufgelöst wird, wie `claude-sonnet-4-6`, immer noch ihre eigene „Aus Gateway" Zeile neben dem eingebauten Eintrag hinzufügt.

Ergebnisse werden in `~/.claude/cache/gateway-models.json` oder `%USERPROFILE%\.claude\cache\gateway-models.json` unter Windows zwischengespeichert und bei jedem Startup aktualisiert. Wenn die Anfrage fehlschlägt oder das Gateway `/v1/models` nicht implementiert, fällt die Auswahl auf die zwischengespeicherte Liste aus dem vorherigen Startup oder auf die eingebaute Modelliste zurück. Wenn Ihr Gateway Claude-Modelle unter Aliasen bereitstellt, die nicht dem Ermittlungsfilter entsprechen, können Entwickler diese Aliase manuell mit den [Modellkonfigurationsvariablen](/docs/de/model-config) hinzufügen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Für den Rest der Gateway-Dokumentationsserie und die zugrunde liegenden API-Referenzen:

* [Gateway-Übersicht](/docs/de/gateways): was ein Gateway ist und wie Sie zwischen Claude Apps Gateway und einem anderen Produkt wählen
* [Andere LLM-Gateways](/docs/de/llm-gateway): wie Sie ein Gateway bereitstellen, das Ihre Organisation betreibt, und wie es mit claude.ai-Abonnements interagiert
* [LLM-Gateway für Ihre Organisation bereitstellen](/docs/de/llm-gateway-rollout): die Admin-Checkliste, die diesen Vertrag verwendet
* [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect): Pro-Entwickler-Konfiguration und die Fehlerbehebungstabelle
* [Beta-Headers-Referenz](https://platform.claude.com/docs/de/api/beta-headers): der aktuelle Satz von `anthropic-beta` Werten
* [Messages API](https://platform.claude.com/docs/de/api/messages): das API-Format, das ein Anthropic-Format-Gateway implementiert
